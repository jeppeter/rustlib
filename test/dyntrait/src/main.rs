
use std::sync::Arc;
//use std::thread;
//use std::thread::JoinHandle;
//use std::time;
use std::cell::RefCell;
//use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;

pub const READ_EVENT :u32 = 0x1;
pub const WRITE_EVENT :u32 = 0x2;
pub const ERROR_EVENT :u32 = 0x4;
pub const ET_TRIGGER  :u32 = 0x80;



#[cfg(target_os = "linux")]
use libc::{clock_gettime,CLOCK_MONOTONIC_COARSE,timespec};

#[cfg(target_os = "windows")]
use winapi::um::sysinfoapi::{GetTickCount64};

const MAX_U64_VAL :u64 = 0xffffffffffffffff;

#[cfg(target_os = "linux")]
pub (crate) fn get_cur_ticks() -> u64 {
	let mut  curtime = timespec {
		tv_sec : 0,
		tv_nsec : 0,
	};
	unsafe {clock_gettime(CLOCK_MONOTONIC_COARSE,&mut curtime);};
	let mut retmills : u64 = 0;
	retmills += (curtime.tv_sec as u64 )  * 1000;
	retmills += ((curtime.tv_nsec as u64) % 1000000000) / 1000000;
	return retmills;
}

#[cfg(target_os = "windows")]
pub (crate) fn get_cur_ticks() -> u64 {
	let retv :u64;
	unsafe {
		retv = GetTickCount64() as u64;
	}
	return retv;
}


pub (crate) fn time_left(sticks : u64,cticks :u64, leftmills :i32) -> i32 {
	let eticks = sticks + leftmills as u64;
	if cticks < eticks && cticks >= sticks {
		return (eticks - cticks) as i32;
	}

	if (MAX_U64_VAL - sticks) < (leftmills as u64) {
		if cticks > 0 && cticks < (leftmills as u64 - (MAX_U64_VAL - sticks)) {
			return ((leftmills as u64) - (MAX_U64_VAL - sticks) - cticks) as i32;
		}

		if cticks >= sticks && cticks < MAX_U64_VAL {
			return ((leftmills as u64) - (cticks - sticks)) as i32;
		}
	}
	return -1;
}


fn get_logger_level() -> i64 {
	return 60;
}

fn log_output_function_inner(level :i64, outs :&str) {
	if level <= get_logger_level() {
		print!("{}",outs);
	}
	return;	
}


pub fn log_output_function(level :i64, outs :&str) {
	return log_output_function_inner(level,outs);
}


macro_rules! format_str_log {
	($info:tt,$iv:expr,$($arg:tt)+) => {
		let mut c :String= format!("[{}:{}]",file!(),line!());
		c.push_str(&format!("{} ",$info));
		c.push_str(": ");
		c.push_str(&(format!($($arg)+)[..]));
		c.push_str("\n");
		log_output_function($iv, &c);		
	}
}


macro_rules! debug_trace {
	($($arg:tt)+) => {
		format_str_log!("<TRACE>",4,$($arg)+);
	}
}

pub trait EvtCall {
	fn debug_mode(&mut self,fname :&str, lineno :u32);
	fn handle(&mut self,evthd :u64, evttype :u32,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn close_event(&mut self,evthd :u64, evttype :u32,evtmain :&mut EvtMain);
}

pub trait EvtTimer {
	fn timer(&mut self,timerguid :u64,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn close_timer(&mut self,timerguid :u64, evtmain :&mut EvtMain);
}

struct EvtTimerElem {
	timer :Arc<RefCell<dyn EvtTimer>>,
	startticks :u64,
	_conti :bool,
	mills :i32,
}

impl EvtTimerElem  {
	fn new(bv :Arc<RefCell<dyn EvtTimer>>, conti :bool, interval :i32) -> Self {
		Self {
			timer : bv.clone(),
			startticks : get_cur_ticks(),
			_conti : conti,
			mills : interval,
		}
	}
}

pub struct EvtMain {	
	evtmaps :HashMap<u64,Arc<RefCell<dyn EvtCall>>>,
	evttimers :HashMap<u64,EvtTimerElem>,
	guidevts :HashMap<u64,u64>,
	evttypes :HashMap<u64,u32>,
	guid :u64,
	exited : i32,
}

impl EvtMain {
	pub fn new() -> Result<Self,Box<dyn Error>> {
		Ok(Self {
			evtmaps : HashMap::new(),
			evttimers : HashMap::new(),
			guidevts : HashMap::new(),
			evttypes : HashMap::new(),
			guid : 1,
			exited : 0,
		})
	}
}

impl Drop for EvtMain {
	fn drop(&mut self) {
		self.close();
		println!("Call EvtMain Free");
	}
}


impl EvtMain {
	pub fn add_timer(&mut self,bv :Arc<RefCell<dyn EvtTimer>>,_interval:i32,_conti:bool) -> Result<u64,Box<dyn Error>> {
		self.guid += 1;
		let gc = EvtTimerElem::new(bv.clone(),_conti,_interval);
		self.evttimers.insert(self.guid,gc);
		Ok(self.guid)
	}

	pub fn add_event(&mut self,bv :Arc<RefCell<dyn EvtCall>>,evthd :u64,evttype :u32) -> Result<(),Box<dyn Error>> {
		self.guid += 1;
		self.evtmaps.insert(self.guid, bv.clone());	
		self.guidevts.insert(evthd,self.guid);
		self.evttypes.insert(evthd,evttype);
		debug_trace!("add 0x{:x} guid 0x{:x}",evthd,self.guid);
		Ok(())
	}

	pub fn remove_timer(&mut self,guid:u64) -> i32 {
		self.evttimers.remove(&guid);
		return 1;
	}

	pub fn remove_event(&mut self,evthd :u64) -> i32 {
		let guid :u64 ;
		//debug_trace!("remove evthd 0x{:x}",evthd);
		match self.guidevts.get(&evthd) {
			Some(_v) => {
				guid = *_v;
			},
			None => {
				debug_trace!("maybe deleted 0x{:x}",evthd);
				return 0;
			}
		}

		self.evtmaps.remove(&guid);
		self.guidevts.remove(&evthd);
		self.evttypes.remove(&evthd);
		return 1;
	}

	fn _find_evthd(&self, guid :u64) -> Option<u64> {
		let mut retv :Option<u64> = None;
		for (v,k) in self.guidevts.iter() {
			if *k == guid {
				retv = Some(*v);
				break;
			}
		}
		return retv;
	}

	pub fn get_timeout(&self, maxtime :u32) -> u32 {
		let mut retv :u32 = maxtime;
		let cticks :u64 = get_cur_ticks();
		for (_,v) in self.evttimers.iter() {
			let reti = time_left(v.startticks,cticks,v.mills);
			if reti < 0 {
				return 1;
			}

			if (reti as u32) < retv {
				retv = reti as u32;
			}
		}

		return retv;
	}


	fn get_time_guids(&self) -> Vec<u64> {
		let cticks = get_cur_ticks();
		let mut retv :Vec<u64> = Vec::new();
		for (g,v) in self.evttimers.iter() {
			let reti = time_left(v.startticks,cticks,v.mills);
			if reti < 0 {
				retv.push(*g);
			}
		}
		return retv;
	}

	pub fn main_loop(&mut self) -> Result<(),Box<dyn Error>> {
		while self.exited == 0 {
			let mut evtguids :Vec<u64> = Vec::new();
			for (v,_) in self.evtmaps.iter() {
				//debug_trace!("v 0x{:x}",*v);
				evtguids.push(*v);
			}


			if evtguids.len() > 0 {
				for pguid in evtguids.iter() {
					let mut findvk :Option<Arc<RefCell<dyn EvtCall>>> = None;
					match self.evtmaps.get(pguid) {
						Some(vk) => {
							findvk = Some(vk.clone());
						},
						None => {
							debug_trace!("cannot find 0x{:x}",*pguid);
						}
					}
					if findvk.is_some() {
						let c :Arc<RefCell<dyn EvtCall>> = findvk.unwrap();
						let evttype :u32;
						let oevthd :Option<u64> = self._find_evthd(*pguid);
						if oevthd.is_none() {
							debug_trace!("cannot find evthd 0x{:x}",*pguid);
							continue;
						}

						let evthd = oevthd.unwrap();
						//debug_trace!("evthd 0x{:x} guid 0x{:x}",evthd,*pguid);

						match self.evttypes.get(&evthd) {
							Some(_v) => {
								evttype = *_v;
							},
							None => {
								debug_trace!("cannot find evttype 0x{:x}",evthd);
								continue;
							}
						}
						c.borrow_mut().debug_mode(file!(),line!());
						c.borrow_mut().handle(evthd,evttype,self)?;					
					}
				}
			} else {				
				std::thread::sleep(std::time::Duration::from_millis(100));
				let tmguids = self.get_time_guids();
				for pguid in tmguids.iter() {
					let mut findvk :Option<Arc<RefCell<dyn EvtTimer>>> = None;
					match self.evttimers.get(pguid) {
						Some(vk) => {
							findvk = Some(vk.timer.clone());
						},
						None => {
							debug_trace!("cannot find 0x{:x}",*pguid);
						}
					}
					if findvk.is_some() {
						let c :Arc<RefCell<dyn EvtTimer>> = findvk.unwrap();
						c.borrow_mut().timer(*pguid,self)?;
					}
				}
			}



		}
		Ok(())
	}

	pub fn break_up(&mut self) -> Result<(),Box<dyn Error>> {
		self.exited = 1;
		Ok(())
	}

	pub fn close(&mut self) {

		let mut guids :Vec<u64> = Vec::new();
		let mut evthds :Vec<u64> = Vec::new();
		//let mut tmguids :Vec<u64> = Vec::new();
		for (k,v) in self.guidevts.iter() {
			guids.push(*v);
			evthds.push(*k);
			debug_trace!("evthds 0x{:x} guid 0x{:x}",*k,*v);
		}

		let mut idx :usize;
		idx = 0;
		while idx < guids.len() {
			let mut fk :Option<Arc<RefCell<dyn EvtCall>>> = None;
			let evttype :u32;
			let evthd :u64;
			match self.evtmaps.get(&guids[idx]) {
				None => {

				},
				Some(_v) => {
					fk = Some(_v.clone());
				}
			}

			if fk.is_some() {
				evthd = evthds[idx];
				match self.evttypes.get(&evthd) {
					Some(_v) => {
						evttype = *_v;
						let c = fk.unwrap();
						c.borrow_mut().close_event(evthd,evttype,self);
					},
					None => {

					}
				}
			}
			idx += 1;
		}

		self.evtmaps = HashMap::new();
		self.evttimers = HashMap::new();
		self.guidevts = HashMap::new();
		self.evttypes = HashMap::new();
		self.guid = 1;
		self.exited = 0;
	}
}



pub struct SockCallInner {
	maxcnt : i32,
	rdhd : u64,
	wrhd : u64,
	errhd : u64,
	rdcnt : i32,
	wrcnt : i32,
	errcnt : i32,
}

#[derive(Clone)]
pub struct SockCall {
	inner :Arc<RefCell<SockCallInner>>,
}

impl Drop for SockCallInner {
	fn drop(&mut self) {
		self.close();
		self.maxcnt = 0;
		self.rdcnt = 0;
	}
}


impl SockCallInner {
	fn new(max :i32,startcode :u64,_evtmain :&mut EvtMain) -> Result<Arc<RefCell<Self>>,Box<dyn Error>> {
		let iretv :Self = Self{
			maxcnt : max,
			rdhd : startcode,
			wrhd : startcode + 1,
			errhd : startcode + 2,
			rdcnt : 0,
			wrcnt : 0,
			errcnt : 0,
		};
		debug_trace!("iretv {:p}",&iretv);
		let retv  = Arc::new(RefCell::new(iretv));
		Ok(retv)
	}

	fn new_after(&mut self, evtmain :&mut EvtMain,parent :SockCall) -> Result<(),Box<dyn Error>> {
		debug_trace!(" ");
		evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.rdhd,READ_EVENT)?;
		//evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.wrhd,READ_EVENT)?;
		//evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.errhd,READ_EVENT)?;
		Ok(())
	}

	fn close(&mut self) {
		debug_trace!("SockCallInner close {:p}",self );
	}
}

impl SockCallInner {
	fn handle(&mut self,evthd :u64,_evttype :u32,evtmain :&mut EvtMain,parent :SockCall) -> Result<(),Box<dyn Error>> {
		debug_trace!("evthd 0x{:x} rdhd 0x{:x} wrhd 0x{:x} errhd 0x{:x} self {:p}",evthd,self.rdhd,self.wrhd,self.errhd,self);
		debug_trace!("rdcnt {} wrcnt {} errcnt {} maxcnt {}",self.rdcnt,self.wrcnt,self.errcnt,self.maxcnt);
		if evthd == self.rdhd {
			self.rdcnt += 1;
			//println!("rdcnt {}", self.rdcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				//evtmain.break_up()?;
				evtmain.remove_event(self.rdhd);
			} else {
				evtmain.remove_event(self.rdhd);
				evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.wrhd,READ_EVENT)?;
			}			
		} else if evthd == self.wrhd {
			self.wrcnt += 1;
			//println!("wrcnt {}", self.wrcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				//evtmain.break_up()?;
				evtmain.remove_event(self.wrhd);
			} else {
				evtmain.remove_event(self.wrhd);
				evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.errhd,READ_EVENT)?;
			}
		} else if evthd == self.errhd {
			self.errcnt += 1;
			//println!("errcnt {}", self.errcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				//evtmain.break_up()?;
				evtmain.remove_event(self.errhd);
			} else {
				evtmain.remove_event(self.errhd);
				evtmain.add_event(Arc::new(RefCell::new(parent.clone())),self.rdhd,READ_EVENT)?;
			}
		}
		Ok(())
	}

	fn close_event(&mut self,_evthd :u64, _evttype :u32,evtmain :&mut EvtMain, _parent :SockCall) {
		debug_trace!("close_event");
		evtmain.remove_event(self.rdhd);
		evtmain.remove_event(self.wrhd);
		evtmain.remove_event(self.errhd);
		self.errcnt = 0;
		self.rdcnt = 0;
		self.wrcnt = 0;
		return;
	}

	fn debug_mode(&mut self,_fname :&str,_lineno:u32,_parent :SockCall) {
		debug_trace!("{}:{} self {:p}",_fname,_lineno,self);
		return;
	}	
}


impl Drop for SockCall {
	fn drop(&mut self) {
		self.close();
	}
}

impl SockCall {
	fn new(maxcnt :i32,startcode :u64, evtmain :&mut EvtMain) -> Result<Self,Box<dyn  Error>> {
		let iretv :Arc<RefCell<SockCallInner>> = SockCallInner::new(maxcnt,startcode,evtmain)?;
		let retv :SockCall = SockCall {
			inner : iretv,
		};
		retv.inner.borrow_mut().new_after(evtmain,retv.clone())?;
		Ok(retv)
	}

	fn close(&mut self) {
		debug_trace!("close SockCall {:p}",self);
	}
}


impl EvtCall for SockCall {
	fn handle(&mut self,evthd :u64,_evttype :u32,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		let p = self.clone();
		return self.inner.borrow_mut().handle(evthd,_evttype,evtmain,p);
	}

	fn close_event(&mut self,_evthd :u64, _evttype :u32,evtmain :&mut EvtMain) {
		let p = self.clone();
		return self.inner.borrow_mut().close_event(_evthd,_evttype,evtmain,p);
	}

	fn debug_mode(&mut self,_fname :&str,_lineno:u32) {
		let p = self.clone();
		return self.inner.borrow_mut().debug_mode(_fname,_lineno,p);
	}	
}



struct TimerOutEventInner {
	guid :u64,
	insertguid : bool,
}

#[derive(Clone)]
struct TimerOutEvent {
	inner :Arc<RefCell<TimerOutEventInner>>
}

impl Drop for TimerOutEventInner {
	fn drop(&mut self) {
		self.close();
	}
}

impl TimerOutEventInner {
	fn timer(&mut self,guid :u64,evtmain :&mut EvtMain,_parent :TimerOutEvent) -> Result<(),Box<dyn Error>> {
		if self.insertguid {
			evtmain.remove_timer(guid);
			self.insertguid = false;	
		}
		evtmain.break_up()?;
		Ok(())
	}

	fn close_timer(&mut self,guid:u64,evtmain :&mut EvtMain, _parent :TimerOutEvent) {
		debug_trace!("close_timer");
		if guid == self.guid && self.insertguid {
			evtmain.remove_timer(self.guid);
			self.insertguid = false;	
		}
		
		return;
	}

	fn new_after(&mut self,maxmills : i32,evtmain :&mut EvtMain,parent :TimerOutEvent) -> Result<(),Box<dyn Error>> {
		if !self.insertguid {
			self.guid = evtmain.add_timer(Arc::new(RefCell::new(parent.clone())),maxmills,false)?;
			self.insertguid = true;
		}		
		Ok(())
	}

	fn new() -> Result<Arc<RefCell<TimerOutEventInner>>,Box<dyn Error>> {
		Ok(Arc::new(RefCell::new(Self {
			guid : 0,
			insertguid : false,
		})))
	}

	fn close(&mut self) {
		debug_trace!("close TimerOutEventInner {:p}",self);
	}
}


impl Drop for TimerOutEvent {
	fn drop(&mut self) {
		self.close();
	}
}

impl TimerOutEvent {
	fn new(maxmills :i32, evtmain :&mut EvtMain) -> Result<Self,Box<dyn Error>> {
		let ninner = TimerOutEventInner::new()?;
		let retv :Self = Self {
			inner : ninner,
		};
		let _ = retv.inner.borrow_mut().new_after(maxmills,evtmain,retv.clone())?;
		Ok(retv)
	}

	fn close(&mut self) {
		debug_trace!("close TimerOutEvent {:p}",self);
	}
}

impl EvtTimer for TimerOutEvent {
	fn timer(&mut self,timerguid :u64,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		let p = self.clone();
		return self.inner.borrow_mut().timer(timerguid,evtmain,p);
	}
	fn close_timer(&mut self,timerguid :u64, evtmain :&mut EvtMain) {
		let p = self.clone();
		return self.inner.borrow_mut().close_timer(timerguid,evtmain,p);
	}

}



fn  main() -> Result<(),Box<dyn Error>> {
	let mut evmain :EvtMain = EvtMain::new()?;
	{
		let  av = SockCall::new(5,0x10,&mut evmain)?;
		let  bv = SockCall::new(10,0x20,&mut evmain)?;
		let  cv = SockCall::new(20,0x30,&mut evmain)?;
		let dv = TimerOutEvent::new(500,&mut evmain)?;
		println!("av {:p}", &av);
		println!("bv {:p}", &bv);
		println!("cv {:p}", &cv);
		println!("dv {:p}", &dv);
	}
	
	evmain.main_loop()?;
	println!("call CC over");
	drop(&evmain);
	println!("after exit evmain");
	//drop(&ac);
	Ok(())
}