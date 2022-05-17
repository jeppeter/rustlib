
use std::cell::RefCell;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::any::Any;
//use std::rc::Rc;
use std::collections::HashMap;



#[derive(Clone)]
pub struct ExtKeyParse {
}


impl ExtKeyParse {
	fn new() -> ExtKeyParse {
		ExtKeyParse {}
	}
}

pub trait ArgSetImpl {
	fn set_value(&mut self,prefix :&str,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
	fn new() -> Self where Self :Sized;
}

#[derive(Clone)]
pub struct NameSpaceEx {
}

impl NameSpaceEx {
	pub fn new() -> NameSpaceEx {
		NameSpaceEx{}
	}
	pub fn get_bool(&self, _k :&str) -> bool {
		println!("namespace k [{}]",_k);
		return false;
	}
	pub fn get_int(&self,_k :&str) -> i64 {
		println!("namespace k [{}]",_k);
		return 0;
	}
	pub fn get_float(&self,_k :&str) -> f64 {
		println!("namespace k [{}]",_k);
		return 0.0;
	}
	pub fn get_array(&self,_k :&str) -> Vec<String> {
		println!("namespace k [{}]",_k);
		return Vec::new();
	}
	pub fn get_string(&self,_k :&str) -> String {
		println!("namespace k [{}]",_k);
		return "".to_string();
	}
}



pub type ExtArgsParseHelpFunc = fn(&ExtKeyParse) -> String;
pub type ExtArgsJsonFunc = fn(NameSpaceEx,ExtKeyParse,Value)  -> Result<(),Box<dyn Error>> ;
pub type ExtArgsActionFunc = fn(NameSpaceEx,i32,ExtKeyParse,Vec<String>) -> Result<i32,Box<dyn Error>>;
pub type ExtArgsCallbackFunc = fn(NameSpaceEx,Option<Arc<RefCell<dyn ArgSetImpl>>>,Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>>;

#[derive(Clone)]
pub enum ExtArgsParseFunc {
	HelpFunc(ExtArgsParseHelpFunc),
	JsonFunc(ExtArgsJsonFunc),
	ActionFunc(ExtArgsActionFunc),
	CallbackFunc(ExtArgsCallbackFunc),
}

#[derive(Clone)]
pub struct ExtArgsParser{}

impl ExtArgsParser {
	pub fn new() -> ExtArgsParser {
		ExtArgsParser {}
	}

	pub fn load_commandline_string(&self,s :&str, fnptrs1 :Option<HashMap<String,ExtArgsParseFunc>>) -> Result<(),Box<dyn Error>> {
		println!("input s\n{}", s);
		if fnptrs1.is_some() {
			let fnptrs = fnptrs1.unwrap();
			for (k,v) in fnptrs.clone().iter() {
				println!("call [{}] function", k);
				match v {
					ExtArgsParseFunc::JsonFunc(v1) => {
						let f = v1.clone();
						let n = NameSpaceEx::new();
						let k = ExtKeyParse::new();
						let v = Value::Null;
						f(n,k,v)?;
					},
					ExtArgsParseFunc::HelpFunc(v1) => {
						let f = v1.clone();
						let k = ExtKeyParse::new();
						println!("get [{}]",f(&k));
					},
					ExtArgsParseFunc::ActionFunc(v1) => {
						let f = v1.clone();
						let n = NameSpaceEx::new();
						let k = ExtKeyParse::new();
						let params = Vec::<String>::new();
						let c = f(n,0,k,params)?;
						println!("ret [{}]",c);
					},
					ExtArgsParseFunc::CallbackFunc(v1) => {
						let f = v1.clone();
						let n = NameSpaceEx::new();
						f(n,None,None)?;
					}
				}
			}
		}
		Ok(())
	}
}
