
use std::sync::Arc;
//use std::thread;
//use std::thread::JoinHandle;
//use std::time;
use std::cell::RefCell;
//use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;

pub trait EvtCall {
	fn get_evt(&self) -> u64;
	fn read(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn write(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn error(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}

pub trait EvtTimer {
	fn timer(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}


pub struct EvtMain {	
	evtmaps :HashMap<u64,Arc<RefCell<dyn EvtCall>>>,
	evttimers :HashMap<u64,Arc<RefCell<dyn EvtTimer>>>,	
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


impl EvtMain {
	pub fn add_timer(&mut self,bv :Arc<RefCell<dyn EvtTimer>>,_interval:i32,_conti:bool) -> Result<u64,Box<dyn Error>> {
		self.guid += 1;
		self.evttimers.insert(self.guid,bv.clone());
		Ok(self.guid)
	}

	pub fn add_event<T : EvtCall>(&mut self,bv :&T>, _eventtype :u32) -> Result<(),Box<dyn Error>> {
		self.guid += 1;
		self.evtmaps.insert(self.guid, bv.clone());
		Ok(())
	}

	pub fn remove_timer(&mut self,guid:u64) -> Result<(),Box<dyn Error>> {
		self.evttimers.remove(&guid);
		Ok(())
	}

	pub fn remove_event<T : EvtCall>(&mut self,bv :&T) -> Result<(),Box<dyn Error>> {
		let mut findguid :u64 = 0;
		for (k,v) in self.evtmaps.iter() {
			if  {
				findguid = *k;
				break;
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
				let mut findvk :Option<Arc<RefCell<dyn EvtCall>>> = None;
				match self.evtmaps.get(v) {
					Some(vk) => {
						findvk = Some(vk.clone());
					},
					None => {						
					}
				}
				if findvk.is_some() {
					let c :Arc<RefCell<dyn EvtCall>> = findvk.unwrap();
					let mut b = c.borrow_mut();
					b.read(self)?;
				}
			}
		}
		Ok(())
	}

	pub fn break_up(&mut self) -> Result<(),Box<dyn Error>> {
		self.exited = 1;
		Ok(())
	}
}



pub struct SockCall {
	maxcnt : i32,
	rdcnt : i32,
	wrcnt : i32,
	errcnt : i32,
}


impl SockCall {
	fn new(max :i32) -> Self {
		Self{
			maxcnt : max,
			rdcnt : 0,
			wrcnt : 0,
			errcnt : 0,
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

	fn read(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.rdcnt += 1;
		println!("rdcnt {}", self.rdcnt);
		if self.rdcnt >= self.maxcnt {
			evtmain.break_up()?;
		} else {
			let c :Arc<RefCell<dyn EvtCall>> = Arc::new(RefCell::new(self.clone()));
			evtmain.remove_event(c)?;
			evtmain.add_event(c,0x2);
		}
		Ok(())
	}

	fn write(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.wrcnt += 1;
		println!("wrcnt {}", self.wrcnt);
		if self.wrcnt >= self.maxcnt {
			evtmain.break_up()?;
		}
		Ok(())
	}

	fn error(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		self.errcnt += 1;
		println!("errcnt {}", self.errcnt);
		if self.errcnt >= self.maxcnt {
			evtmain.break_up()?;
		}
		Ok(())
	}
}



fn  main() -> Result<(),Box<dyn Error>> {
	let av :Arc<RefCell<SockCall>> = Arc::new(RefCell::new(SockCall::new(5)));
	let mut evmain :EvtMain = EvtMain::new()?;
	evmain.add_event(av,1)?;
	evmain.main_loop()?;
	println!("call CC over");
	Ok(())
}