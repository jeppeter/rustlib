use std::ops::{Deref,DerefMut};
use std::error::Error;
use std::sync::Arc;
//use std::cell::RefCell;
use std::cell::UnsafeCell;

mod errors;

defertest_error_class!{DeferError}

struct InnerCall {
	x :i32,
	y :i32,
}

impl InnerCall {
	fn new(x :i32,y :i32) -> Result<Self,Box<dyn Error>> {
		Ok(Self{
			x:x,
			y:y,
		})
	}

	fn set_x(&mut self, x:i32) -> i32 {
		let retv = self.x;
		self.x = x;
		return retv;
	}

	fn set_y(&mut self, y :i32) -> i32 {
		let retv = self.y;
		self.y = y;
		return retv;		
	}
	fn get_x(&self) -> i32 {
		return self.x;
	}

	fn get_y(&self) -> i32 {
		return self.y;
	}
}

#[derive(Clone)]
struct CallMain {
	inner :Arc<UnsafeCell<InnerCall>>,
}

impl CallMain {
	fn new(x :i32,y :i32) -> Result<Self,Box<dyn Error>> {
		Ok(Self{
			inner :Arc::new(UnsafeCell::new(InnerCall::new(x,y)?)),
		})
	}
}

impl Deref for CallMain {
	type Target = InnerCall;
	fn deref(&self) -> &InnerCall {
		let s1 = unsafe {&*self.inner.get()};
		return s1;
	}
}

impl DerefMut for CallMain {
	fn deref_mut(&mut self) -> &mut InnerCall {
		let s1 = unsafe {&mut *self.inner.get()};
		return s1;
	}
}


fn main() -> Result<(),Box<dyn Error>> {
    let c = CallMain::new(3,5)?;
    let mut d = c.clone();
    println!("x : {} y : {}", c.get_x(),c.get_y());
    d.set_x(5);
    d.set_y(10);
    println!("x : {} y : {}", c.get_x(),c.get_y());
    return Ok(());
}
