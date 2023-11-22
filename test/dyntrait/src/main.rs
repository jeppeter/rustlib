
use std::sync::Arc;
//use std::thread;
//use std::thread::JoinHandle;
//use std::time;
use std::cell::RefCell;
//use std::sync::Arc;
use std::error::Error;

pub struct EvtMain {	
}

impl EvtMain {
	fn new() -> Self {
		EvtMain{}
	}
}

pub trait EvtCall {
	fn get_evt(&self) -> u64;
	fn read(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn write(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
	fn error(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}

pub trait EvtTimer {
	fn timer(&mut self,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>>;
}

#[derive(Clone)]
pub struct SockCall {	
}


impl SockCall {
	fn new() -> Self {
		Self{}
	}
}

impl Drop for SockCall {
	fn drop(&mut self) {
		println!("call SockCall Free");
	}
}

#[allow(non_snake_case)]
fn CallAA(bv :Arc<RefCell<dyn EvtCall>>,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
	//let u = bv.get().get_evt();
	{
		let mut b = bv.borrow_mut();
		println!("AA cnt {}", Arc::strong_count(&bv));
		b.error(evtmain)?;
	}
	{
		//bv.get_mut().error(&mut evtmain)?;	
	}
	
	Ok(())
}


#[allow(non_snake_case)]
fn CallBB(bv :Arc<RefCell<dyn EvtCall>>,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
	//let u = bv.get().get_evt();
	{
		let mut b = bv.borrow_mut();
		println!("BB cnt {}", Arc::strong_count(&bv));
		b.write(evtmain)?;
	}
	{
		//bv.get_mut().error(&mut evtmain)?;	
	}
	
	Ok(())
}


#[allow(non_snake_case)]
fn CallCC(bv :Arc<RefCell<dyn EvtCall>>,evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
	//let u = bv.get().get_evt();
	{
		let mut b = bv.borrow_mut();
		println!("CC cnt {}", Arc::strong_count(&bv));
		b.read(evtmain)?;
	}
	{
		//bv.get_mut().write(&mut evtmain)?;	
	}
	{
		//bv.get_mut().error(&mut evtmain)?;	
	}
	
	Ok(())
}

impl EvtCall for SockCall {
	fn get_evt(&self) -> u64 {
		return 0;
	}

	fn read(&mut self,_evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		println!("call SockCall read");
		let n = Arc::new(RefCell::new(self.clone()));
		println!("cnt {}",Arc::strong_count(&n));
		CallBB(n.clone(),_evtmain)?;
		println!("cnt {}",Arc::strong_count(&n));
		Ok(())
	}

	fn write(&mut self,_evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		println!("call SockCall write");
		let n = Arc::new(RefCell::new(self.clone()));
		println!("cnt {}",Arc::strong_count(&n));
		CallAA(n.clone(),_evtmain)?;
		println!("cnt {}",Arc::strong_count(&n));
		Ok(())
	}

	fn error(&mut self,_evtmain :&mut EvtMain) -> Result<(),Box<dyn Error>> {
		println!("call SockCall error");
		Ok(())
	}
}



fn  main() -> Result<(),Box<dyn Error>> {
	let av :Arc<RefCell<SockCall>> = Arc::new(RefCell::new(SockCall::new()));
	let mut evmain :EvtMain = EvtMain::new();
	CallCC(av,&mut evmain)?;
	println!("call CC over");
	Ok(())
}