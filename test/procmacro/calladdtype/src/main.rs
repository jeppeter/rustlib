extern crate addtype;
extern crate funccall;

use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
use std::fmt;
#[allow(unused_imports)]
use std::collections::HashMap;


mod bob;

use addtype::{print_func_name,print_all_links,call_list_all,ArgSet};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;



macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	#[allow(dead_code)]
	impl $type {
		fn create(c :&str) -> $type {
			$type {msg : format!("{}",c)}
		}
	}

	impl fmt::Display for $type {
		fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
			write!(f,"{}",self.msg)
		}
	}

	impl Error for $type {}
	};
}

macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
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

#[derive(Clone)]
pub struct ExtKeyParse {
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



pub trait ArgSetImpl {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
	fn new() -> Self;
}

fn name_help(_k :&ExtKeyParse) -> String {
	return "name help".to_string();
}

fn name_json_set(_ns :NameSpaceEx,_k :ExtKeyParse,_v :Value) -> Result<(),Box<dyn Error>> {
	println!("name_json_set");
	Ok(())
}

fn name_value_set(_ns :NameSpaceEx,_i :i32,_k :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
	println!("name value set");
	return Ok(1);
}

fn parser_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("parser handler");
	Ok(())
}

fn call_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call handler");
	Ok(())
}




#[derive(ArgSet,Debug)]
pub struct CCFunc {
	aval :f64,
	bval :f64,
	cval :Vec<String>,
}


#[derive(ArgSet,Debug)]
pub struct BBFunc {
	csub :CCFunc,
	//bbx : HashMap<String,String>,
	xstr :String,
	bval : bool,
	ii : i32,
	ui : u32,
	ii6 : i64,
	ui6 : u64,
	fi : f32,
	fi6 :f64,
}



fn call_arg_set<T : ArgSetImpl>(cv :&mut T,ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	cv.set_value("csub.cval",ns.clone())?;
	Ok(())
}


#[extargs_map_function(opthelp=name_help,jsonfunc=name_json_set,actfunc=name_value_set,callbackfunc=parser_handler,call_handler)]
fn main() {
	let cc = String::from("hello_world");
	let scc = &(String::from("get_a_reply")[..]);
	let bcc = "hello_world";
	let mut cv = BBFunc::new();
	let ns = NameSpaceEx::new();
	call_list_all!("hello_world",&(cc[..]),&(String::from("get_a_repl")[..]));
	call_list_all!("hello_world");
	call_list_all!(bcc);
	call_list_all!();
	bob::bob_func();
	call_arg_set(&mut cv,ns).unwrap();
	println!("cv [{:?}]",cv);
	return;
}
