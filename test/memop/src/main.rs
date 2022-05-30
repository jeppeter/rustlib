use extargsparse_codegen::{ArgSet,extargs_map_function,extargs_load_commandline};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};



use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
use std::any::Any;



fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call dep_handler");
	Ok(())
}

#[derive(ArgSet)]
struct DepStruct {
	subnargs :Vec<String>
}

#[derive(ArgSet)]
struct ArgsValue {
	verbose : i32,
	port :i32,
	args :Vec<String>,
	dep :DepStruct,
}

#[extargs_map_function(dep_handler)]
fn main() -> Result<(),Box<dyn Error>> {
	let cmdline = r#"{
		"verbose|v" : "+",
		"port|p" : 3002,
		"dep<dep_handler>## dep set ##" : {
			"$" : "+"
		}
	}"#;

	let parser : ExtArgsParser = ExtArgsParser::new(None,None)?;
	extargs_load_commandline!(parser,cmdline)?;
	let b :ArgsValue = ArgsValue::new();
	let pi :Arc<RefCell<ArgsValue>> = Arc::new(RefCell::new(b));
	let ns = parser.parse_commandline_ex(None,None,Some(pi.clone()),None)?;
	println!("verbose [{}] port [{}]",ns.get_int("verbose"),ns.get_int("port"));
	println!("subnargs {:?}", ns.get_array("subnargs"));
	println!("args {:?}", ns.get_array("args"));
	Ok(())
}