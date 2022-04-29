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

#[derive(Clone)]
pub enum FuncEnum {
	StringFunc(Rc<dyn Fn(String) -> i32>),
	LoadFunc(Rc<dyn Fn(String,ExtKeyparse,ParserCompat) -> Result<(),Box<dyn Error>>>),
	ActionFunc(Rc<dyn Fn(NameSpaceEx,i32,ExtKeyparse,Vec<String>) -> Result<i32,Box<dyn Error>>>),
	LoadJsonFunc(Rc<dyn Fn(NameSpaceEx) -> Result<(),Box<dyn Error>>>),
	JsonFunc(Rc<dyn Fn(NameSpaceEx,ExtKeyparse,Value) -> Result<(),Box<dyn Error>>>),
}

#[derive(Clone)]
pub struct VFn {
	//innermap : HashMap<String,Rc<FuncEnum>>,
	innermap : Rc<RefCell<HashMap<String,Rc<RefCell<FuncEnum>>>>>,
}


impl VFn {
	pub fn new() -> VFn {
		VFn{
			//innermap : HashMap::new(),
			innermap : Rc::new(RefCell::new(HashMap::new())),
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

	pub fn string_action(&mut self,_ns :NameSpaceEx, _k :i32,_keycls :ExtKeyparse, _args :Vec<String>) -> Result<i32,Box<dyn Error>> {
		println!("string_action {}",_k);
		//for (k,_) in self.innermap.iter() {
		for (k,_) in self.innermap.borrow().iter() {
			println!("string_action {}", k);
		}
		Ok(0)
	}

	pub fn insertmaps(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let c = b.clone();
		//self.innermap.insert(format!("hello"), Rc::new(FuncEnum::StringFunc(Rc::new(move |x| {  c.borrow_mut().hello(x) } ))));
		self.innermap.borrow_mut().insert(format!("hello"), Rc::new(RefCell::new(FuncEnum::StringFunc(Rc::new(move |x| {  c.borrow_mut().hello(x) } )))));
		let e = b.clone();
		//self.innermap.insert(format!("world"),Rc::new(FuncEnum::StringFunc(Rc::new(move |x| {  e.borrow_mut().world(x) }))));
		self.innermap.borrow_mut().insert(format!("world"),Rc::new(RefCell::new(FuncEnum::StringFunc(Rc::new(move |x| {  e.borrow_mut().world(x) })))));
		//let s1 = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		//self.innermap.insert(format!("stract"),Rc::new(FuncEnum::ActionFunc(Rc::new(move |n,i,k,s| { s1.borrow_mut().string_action(n,i,k,s) }))));
		self.innermap.borrow_mut().insert(format!("stract"),Rc::new(RefCell::new(FuncEnum::ActionFunc(Rc::new(move |n,i,k,s| { s1.borrow_mut().string_action(n,i,k,s) })))));
		let s2 =  Arc::new(RefCell::new(self.clone()));
		let s3 = s2.clone();
		//self.innermap.insert(format!("stract2"),Rc::new(FuncEnum::ActionFunc(Rc::new(move |n,i,k,s| { s3.borrow_mut().string_action(n,i,k,s) }))));
		self.innermap.borrow_mut().insert(format!("stract2"),Rc::new(RefCell::new(FuncEnum::ActionFunc(Rc::new(move |n,i,k,s| { s3.borrow_mut().string_action(n,i,k,s) })))));
	}

	fn c_fn(&self,c :&FuncEnum) -> Option<FuncEnum> {
		return Some(c.clone());
	}

	fn get_fn(&mut self, k :&str) -> Option<FuncEnum> {
		let mut retv :Option<FuncEnum> = None;
		match self.innermap.borrow_mut().get(k)  {
			Some(f1) => {
				let  f2 :&FuncEnum = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {
				println!("no {} function", k);
			}
		}
		retv
	}

	fn call_str_fn(&mut self, k :&str,v :&str) -> i32 {
		let mut retv : i32 = 0;
		let f1 = self.get_fn(k);
		if f1.is_some() {
			let f2 = f1.unwrap();
			match f2 {
				FuncEnum::StringFunc(f) => {
					let c = f(format!("{}",v));
					println!("retval [{}]",c);
					retv = c;
				},
				_ => {
					println!("no function {}",k);
				}
			}
		}
		retv
	}

	fn call_act_fn(&mut self, k :&str,ns :NameSpaceEx,kv :i32,ks :ExtKeyparse, args :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let mut retv :Result<i32,Box<dyn Error>> = Ok(0);
		let f1 = self.get_fn(k);
		if f1.is_some() {
			let f2 = f1.unwrap();
			match f2 {
				FuncEnum::ActionFunc(f) => {
					retv = f(ns.clone(),kv,ks.clone(),args.clone());
				},
				_ => {
					println!("no result {}", k);
					retv = Ok(32);
				}
			}
		}

		retv
	}

	pub fn call_fn(&mut self) {
		self.call_str_fn("hello","cxx");
		self.call_str_fn("world","vs2w");
		let args :Vec<String> = Vec::new();
		let ns = NameSpaceEx{};
		let ek = ExtKeyparse{};
		_ =self.call_act_fn("stract",ns.clone(),20,ek.clone(),args.clone());
		_ =self.call_act_fn("stract2",ns.clone(),22,ek.clone(),args.clone());
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
