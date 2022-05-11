
use std::cell::RefCell;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::any::Any;



#[derive(Clone)]
pub struct ExtKeyParse {
}


pub trait ArgSetImpl {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
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
		return false;
	}
	pub fn get_int(&self,_k :&str) -> i64 {
		return 0;
	}
	pub fn get_float(&self,_k :&str) -> f64 {
		return 0.0;
	}
	pub fn get_array(&self,_k :&str) -> Vec<String> {
		return Vec::new();
	}
	pub fn get_string(&self,_k :&str) -> String {
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
