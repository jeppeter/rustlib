use std::collections::HashMap;
use std::sync::Arc;
//use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use serde_json::Value;




fn add(i :i32, j :i32) -> i32 {
	println!("add function {}+{}={}", i,j,i+j);
	return i+j;
}

fn add_2(i :i32, j :i32) -> i32 {
	println!("add2 function {}+{}+2={}",i,j,i+j+2 );
	return i+j+2;
}

#[derive(Clone)]
pub struct ExtKeyparse {}

#[derive(Clone)]
pub struct ParserCompat {}

#[derive(Clone)]
pub struct NameSpaceEx {}

pub enum FuncEnum {
	StringFunc(Box<dyn Fn(String) -> i32>),
	LoadFunc(Box<dyn Fn(String,ExtKeyparse,ParserCompat) -> Result<(),Box<dyn Error>>>),
	ActionFunc(Box<dyn Fn(NameSpaceEx,i32,ExtKeyparse,Vec<String>) -> Result<i32,Box<dyn Error>>>),
	LoadJsonFunc(Box<dyn Fn(NameSpaceEx) -> Result<(),Box<dyn Error>>>),
	JsonFunc(Box<dyn Fn(NameSpaceEx,ExtKeyparse,Value) -> Result<(),Box<dyn Error>>>),
}

#[derive(Clone)]
pub struct VFn {
	innermap : HashMap<String,Arc<FuncEnum>>,
}


impl VFn {
	pub fn new() -> VFn {
		VFn{
			innermap : HashMap::new(),
		}
	}

	pub fn hello(&mut self,c :String) -> i32 {
		println!("hello from self");
		c.len() as i32
	}

	pub fn world(&mut self,c :String) -> i32 {
		println!("world from self");
		c.len() as i32
	}

	pub fn insertmaps(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let c = b.clone();
		self.innermap.insert(format!("hello"), Arc::new(FuncEnum::StringFunc(Box::new(move |x| {  c.borrow_mut().hello(x) } ))));
		let e = b.clone();
		self.innermap.insert(format!("world"),Arc::new(FuncEnum::StringFunc(Box::new(move |x| {  e.borrow_mut().world(x) }))));
	}

	pub fn call_fn(&mut self) {
		match self.innermap.get_mut("hello") {
			Some(f1) => {
				let f2 = Arc::get_mut(f1).unwrap();
				match f2 {
					FuncEnum::StringFunc(f) => {
						let c = f(format!("call"));
						println!("retval [{}]",c);
					},
					_ => {
						println!("hello no func");
					},
				}
			},
			None => {println!("None");}
		}


		match self.innermap.get_mut("world") {
			Some(f1) => {
				let f2 = Arc::get_mut(f1).unwrap();
				match f2 {
					FuncEnum::StringFunc(f) => {
						let c = f(format!("call"));
						println!("retval [{}]",c);
					},
					_ => {
						println!("world no func");
					},
				}
			},
			None => {println!("None");}
		}

	}
}

pub struct CommonVFn {
	innerrc :Rc<RefCell<VFn>>,
}

impl CommonVFn {
	pub fn new() -> CommonVFn {
		CommonVFn {
			innerrc : Rc::new(RefCell::new( VFn::new())),
		}
	}
	pub fn insertmaps(&self) {
		self.innerrc.borrow_mut().insertmaps();
	}

	pub fn call_fn(&self) {
		self.innerrc.borrow_mut().call_fn();
	}
}


fn main() {
	let fnptr : fn(i32,i32) -> i32 = add;
	let fn2ptr = add_2;
	let text = "call text";
	let mut closures :Vec<Box<dyn Fn()>> = Vec::new();
	fnptr(3,2);
	fn2ptr(3,2);
	(fnptr)(3,10);
	(fn2ptr)(20,10);
	closures.push(Box::new(|| println!("first")));
	closures.push(Box::new(|| println!("second {}",text)));
	closures.push(Box::new(|| println!("third")));
	for f in closures.iter() {
		f();
	}

	{
		let c = CommonVFn::new();
		c.insertmaps();
		c.call_fn();
	}
}
