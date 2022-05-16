extern crate addtype;
extern crate funccall;

use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
use std::fmt;
#[allow(unused_imports)]
use std::collections::HashMap;
use funccall::{ExtKeyParse,NameSpaceEx,ArgSetImpl,ExtArgsParseFunc,ExtArgsParser};
use std::cell::RefCell;
#[allow(unused_imports)]
use serde_json::Value;
use std::any::Any;
use std::sync::{Arc};


use addtype::{ArgSet,extargs_map_function,extargs_load_commandline};

mod ali;
mod bob;

#[allow(unused_imports)]
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

#[allow(dead_code)]
fn name_help(_k :&ExtKeyParse) -> String {
	return "name help".to_string();
}

#[allow(dead_code)]
fn name_json_set(_ns :NameSpaceEx,_k :ExtKeyParse,_v :Value) -> Result<(),Box<dyn Error>> {
	println!("name_json_set");
	Ok(())
}

#[allow(dead_code)]
fn name_value_set(_ns :NameSpaceEx,_i :i32,_k :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
	println!("name value set");
	return Ok(1);
}

#[allow(dead_code)]
fn parser_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("parser handler");
	Ok(())
}

#[allow(dead_code)]
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
	let mut cv = BBFunc::new();
	let ns = NameSpaceEx::new();
	let mut parser :ExtArgsParser = ExtArgsParser::new();
	let cmdline :String = "main cmdline".to_string();
	let c = extargs_load_commandline!(parser,&(cmdline));
	if c.is_err() {
		std::process::exit(5);
	}
	ali::ali_func();
	bob::bob_func();
	call_arg_set(&mut cv,ns).unwrap();
	println!("cv [{:?}]",cv);
	return;
}
