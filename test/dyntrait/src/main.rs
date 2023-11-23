
use std::sync::Arc;
//use std::thread;
//use std::thread::JoinHandle;
//use std::time;
//use std::cell::RefCell;
//use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;

pub const READ_EVENT :u32 = 0x1;
pub const WRITE_EVENT :u32 = 0x2;
pub const ERROR_EVENT :u32 = 0x4;
pub const ET_TRIGGER  :u32 = 0x80;

pub trait EvtCall {
	fn get_evt(&self) -> u64;
	fn get_evttype(&self) -> u32;
	fn read(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn write(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn error(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}

pub trait EvtTimer {
	fn timer(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}


pub struct EvtMain {	
	evtmaps :HashMap<u64,*mut dyn EvtCall>,
	evttimers :HashMap<u64,*mut dyn EvtTimer>,	
	guid :u64,
	exited : i32,
}

impl EvtMain {
	pub fn new() -> Result<Self,Box<dyn Error>> {
		Ok(Self {
			evtmaps : HashMap::new(),
			evttimers : HashMap::new(),
			guid : 1,
			exited : 0,
		})
	}
}

impl Drop for EvtMain {
	fn drop(&mut self) {
		self.reset_all();
		println!("Call EvtMain Free");
	}
}


impl EvtMain {
	pub fn add_timer(&mut self,bv :Arc<*mut dyn EvtTimer>,_interval:i32,_conti:bool) -> Result<u64,Box<dyn Error>> {
		self.guid += 1;
		unsafe {
			self.evttimers.insert(self.guid,*(Arc::as_ptr(&bv)));	
		}		
		Ok(self.guid)
	}

	pub fn add_event(&mut self,bv :Arc<*mut dyn EvtCall>) -> Result<(),Box<dyn Error>> {
		self.guid += 1;
		unsafe {
			self.evtmaps.insert(self.guid, *(Arc::as_ptr(&bv)));	
		}		
		Ok(())
	}

	pub fn remove_timer(&mut self,guid:u64) -> Result<(),Box<dyn Error>> {
		self.evttimers.remove(&guid);
		Ok(())
	}

	pub fn remove_event(&mut self,bv :Arc<*mut dyn EvtCall>) -> Result<(),Box<dyn Error>> {
		let mut findguid :u64 = 0;
		let b = Arc::as_ptr(&bv);
		for (k,v) in self.evtmaps.iter() {
			unsafe {
				if (&(*(*b))).get_evt() == (*(*v)).get_evt()  {
					findguid = *k;
					break;
				}					
			}
		}
		if findguid > 0 {
			self.evtmaps.remove(&findguid);
		}
		Ok(())
	}

	pub fn main_loop(&mut self) -> Result<(),Box<dyn Error>> {
		while self.exited == 0 {
			let mut evtguids :Vec<u64> = Vec::new();
			let mut tmguids :Vec<u64> = Vec::new();
			for (v,_k) in self.evtmaps.iter() {
				evtguids.push(*v);
			}

			for (v,_k) in self.evttimers.iter() {
				tmguids.push(*v);
			}

			for v in evtguids.iter() {
				let mut findvk :Option<Arc<* mut dyn EvtCall>> = None;
				match self.evtmaps.get(v) {
					Some(vk) => {
						findvk = Some(Arc::new(*vk));
					},
					None => {						
					}
				}
				if findvk.is_some() {
					let c :Arc<* mut dyn EvtCall> = findvk.unwrap();
					let b = Arc::as_ptr(&c);
					let evttype :u32;

					unsafe {
						evttype = (&(*(*b))).get_evttype();
					}
					if (evttype & READ_EVENT) != 0 {
						unsafe {

							(&mut (*(*b))).read(self)?;
						}						
					} 
					if (evttype & WRITE_EVENT) != 0 { 
						unsafe {

							(&mut (*(*b))).write(self)?;
						}
					} 

					if (evttype & ERROR_EVENT) != 0 { 
						unsafe {

							(&mut (*(*b))).error(self)?;
						}
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

	pub fn reset_all(&mut self) {
		self.evtmaps = HashMap::new();
		self.evttimers = HashMap::new();
		self.guid = 1;
		self.exited = 0;
	}
}



pub struct SockCall {
	maxcnt : i32,
	rdcnt : i32,
	wrcnt : i32,
	errcnt : i32,
	evttype : u32,
}


impl SockCall {
	fn new(max :i32) -> Self {
		Self{
			maxcnt : max,
			rdcnt : 0,
			wrcnt : 0,
			errcnt : 0,
			evttype : READ_EVENT,
		}
	}
}


impl Drop for SockCall {
	fn drop(&mut self) {
		println!("call SockCall Free");
	}
}

impl EvtCall for SockCall {
	fn get_evt(&self) -> u64 {
		return 0;
	}

	fn get_evttype(&self) -> u32 {
		return self.evttype;
	}

	fn read(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.rdcnt += 1;
		println!("rdcnt {}", self.rdcnt);
		if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
			evtmain.break_up()?;
		} else {
			let c :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			evtmain.remove_event(c)?;
			let c2 :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			self.evttype = WRITE_EVENT;
			evtmain.add_event(c2)?;
		}
		Ok(())
	}

	fn write(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.wrcnt += 1;
		println!("wrcnt {}", self.wrcnt);
		if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
			evtmain.break_up()?;
		} else {
			let c :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			evtmain.remove_event(c)?;
			let c2 :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			self.evttype = ERROR_EVENT;
			evtmain.add_event(c2)?;
		}
		Ok(())
	}

	fn error(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.errcnt += 1;
		println!("errcnt {}", self.errcnt);
		if self.rdcnt >= self.maxcnt && self.wrcnt >= self.maxcnt && self.errcnt >= self.maxcnt {
			evtmain.break_up()?;
		} else {
			let c :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			evtmain.remove_event(c)?;
			let c2 :Arc<*mut dyn EvtCall> = Arc::new(self as *mut dyn EvtCall);
			self.evttype = READ_EVENT;
			evtmain.add_event(c2)?;			
		}
		Ok(())
	}
}



fn  main() -> Result<(),Box<dyn Error>> {
	let mut ac :SockCall = SockCall::new(5);
	let av :Arc<* mut dyn EvtCall> = Arc::new(&mut ac as * mut dyn EvtCall);
	let mut evmain :EvtMain = EvtMain::new()?;
	evmain.add_event(av)?;
	evmain.main_loop()?;
	println!("call CC over");
	drop(&evmain);
	drop(&ac);
	Ok(())
}