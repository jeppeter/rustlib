
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

pub struct EvtMain {	
	evtmaps :HashMap<u64,Arc<RefCell<dyn EvtCall>>>,
	evttimers :HashMap<u64,Arc<RefCell<dyn EvtTimer>>>,	
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
		self.evttimers.insert(self.guid,bv.clone());
		Ok(self.guid)
	}

	pub fn add_event(&mut self,bv :Arc<RefCell<dyn EvtCall>>,evthd :u64,evttype :u32) -> Result<(),Box<dyn Error>> {
		{
			bv.borrow_mut().debug_mode(file!(),line!());	
		}
		
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
		debug_trace!("remove evthd 0x{:x}",evthd);
		match self.guidevts.get(&evthd) {
			Some(_v) => {
				guid = *_v;
			},
			None => {
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

	pub fn main_loop(&mut self) -> Result<(),Box<dyn Error>> {
		while self.exited == 0 {
			let mut evtguids :Vec<u64> = Vec::new();
			let mut tmguids :Vec<u64> = Vec::new();
			for (v,_) in self.evtmaps.iter() {
				debug_trace!("v 0x{:x}",*v);
				evtguids.push(*v);
			}
			debug_trace!(" ");

			for (v,_) in self.evttimers.iter() {
				tmguids.push(*v);
			}

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
					debug_trace!("evthd 0x{:x} guid 0x{:x}",evthd,*pguid);

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



pub struct SockCall {
	maxcnt : i32,
	rdhd : u64,
	wrhd : u64,
	errhd : u64,
	rdcnt : i32,
	wrcnt : i32,
	errcnt : i32,
}


impl SockCall {
	fn new(max :i32,evtmain :&mut EvtMain) -> Result<Arc<RefCell<Self>>,Box<dyn Error>> {
		let iretv :Self = Self{
			maxcnt : max,
			rdhd : 1,
			wrhd : 2,
			errhd : 3,
			rdcnt : 0,
			wrcnt : 0,
			errcnt : 0,
		};
		debug_trace!("iretv {:p}",&iretv);
		let retv  = Arc::new(RefCell::new(iretv));
		debug_trace!(" ");
		let mut hd:u64 = retv.borrow().rdhd;
		evtmain.add_event(retv.clone(),hd,READ_EVENT)?;
		hd = retv.borrow().wrhd;
		evtmain.add_event(retv.clone(),hd,READ_EVENT)?;
		hd = retv.borrow().errhd;
		evtmain.add_event(retv.clone(),hd,READ_EVENT)?;
		Ok(retv)
	}

}


impl Drop for SockCall {
	fn drop(&mut self) {
		println!("call SockCall Free");
	}
}

impl EvtCall for SockCall {
	fn handle(&mut self,evthd :u64,_evttype :u32,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		debug_trace!("evthd 0x{:x} rdhd 0x{:x} wrhd 0x{:x} errhd 0x{:x} self {:p}",evthd,self.rdhd,self.wrhd,self.errhd,self);
		debug_trace!("rdcnt {} wrcnt {} errcnt {} maxcnt {}",self.rdcnt,self.wrcnt,self.errcnt,self.maxcnt);
		if evthd == self.rdhd {
			self.rdcnt += 1;
			println!("rdcnt {}", self.rdcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				evtmain.break_up()?;
			}			
		} else if evthd == self.wrhd {
			self.wrcnt += 1;
			println!("wrcnt {}", self.wrcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				evtmain.break_up()?;
			}
		} else if evthd == self.errhd {
			self.errcnt += 1;
			println!("errcnt {}", self.errcnt);
			if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
				evtmain.break_up()?;
			}			
		}
		Ok(())
	}

	fn close_event(&mut self,_evthd :u64, _evttype :u32,evtmain :&mut EvtMain) {

		evtmain.remove_event(self.rdhd);
		evtmain.remove_event(self.wrhd);
		evtmain.remove_event(self.errhd);

		self.errcnt = 0;
		self.rdcnt = 0;
		self.wrcnt = 0;
		return;
	}

	fn debug_mode(&mut self,_fname :&str,_lineno:u32) {
		debug_trace!("{}:{} self {:p}",_fname,_lineno,self);
		return;
	}
}



fn  main() -> Result<(),Box<dyn Error>> {
	let mut evmain :EvtMain = EvtMain::new()?;
	let  _ac :Arc<RefCell<SockCall>> = SockCall::new(5,&mut evmain)?;
	evmain.main_loop()?;
	println!("call CC over");
	//drop(&evmain);
	//drop(&ac);
	Ok(())
}