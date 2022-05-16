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




#[extargs_map_function()]
pub fn ali_func() {
	let mut parser = ExtArgsParser::new();
	let cmdline :String = "ali cmdline".to_string();
	let c = extargs_load_commandline!(parser,&cmdline);
	if c.is_err() {
		std::process::exit(5);
	}
	return;
}