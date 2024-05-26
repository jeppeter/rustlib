
use std::error::Error;
use implshare::{impl_share_fn};
use std::sync::Arc;
use std::cell::UnsafeCell;


#[allow(dead_code)]
struct ImplObjInner {
	fd :i32,	
}

#[impl_share_fn]
impl ImplObjInner {
	fn new(val :i32) -> Result<Self,Box<dyn Error>> {
		let retv : Self = Self {
			fd : val,
		};
		Ok(retv)
	}

	fn get_val(&self) -> i32 {
		return self.fd;
	}

	fn set_val(&mut self, val :i32) -> Result<i32,Box<dyn Error>> {
		let retv = self.fd;
		self.fd = val;
		Ok(retv)
	}
}

pub struct ImplObj {
	inner :Arc<UnsafeCell<ImplObjInner>>,
}

impl ImplObj {
	pub fn new(val :i32) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner :Arc::new(UnsafeCell::new(ImplObjInner::new(val)?)),
		};
		Ok(retv)
	}

	pub fn get_val(&self) -> i32 {
		let sq : &ImplObjInner = unsafe {&*self.inner.get()};
		return sq.get_val();
	}

	pub fn set_val(&mut self,val :i32) -> Result<i32,Box<dyn Error>> {
		let sq : &mut ImplObjInner = unsafe {&mut *self.inner.get()};
		return sq.set_val(val);		
	}
}


fn main() {
    let mut s :ImplObj = ImplObj::new(4).unwrap();
    let _ = s.set_val(11);
    println!("new val {}",s.get_val());
}
