extern crate addtype;
extern crate funccall;

use addtype::{extargs_map_function,extargs_load_commandline};
#[allow(unused_imports)]
use funccall::{ExtArgsParseFunc,ExtKeyParse,NameSpaceEx,ArgSetImpl,ExtArgsParser};
#[allow(unused_imports)]
use lazy_static::lazy_static;
use std::error::Error;
use serde_json::Value;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;



#[allow(dead_code)]
fn bob_help(_k :&ExtKeyParse) -> String {
	return "bob help".to_string();
}

#[allow(dead_code)]
fn bob_json_set(_ns :NameSpaceEx,_k :ExtKeyParse,_v :Value) -> Result<(),Box<dyn Error>> {
	println!("bob_json_set");
	Ok(())
}

#[allow(dead_code)]
fn bob_value_set(_ns :NameSpaceEx,_i :i32,_k :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
	println!("bob value set");
	return Ok(1);
}

#[allow(dead_code)]
fn bobparser_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("bob parser handler");
	Ok(())
}

#[allow(dead_code)]
fn bobcall_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("bob call handler");
	Ok(())
}


#[extargs_map_function(opthelp=bob_help,jsonfunc=bob_json_set,optparse=bob_value_set,callbackfunc=bobparser_handler,bobcall_handler)]
pub fn bob_func() {
	let mut parser = ExtArgsParser::new();
	let cmdline :String = "bob cmdline".to_string();
	let c = extargs_load_commandline!(parser,&cmdline);
	if c.is_err() {
		std::process::exit(5);
	}
	return;
}