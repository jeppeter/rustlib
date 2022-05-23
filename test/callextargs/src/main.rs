#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::options::{ExtArgsOptions};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};
#[allow(unused_imports)]
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};


#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(ArgSet)]
struct MainDataStruct {
	verbose : i32,
}

extargs_error_class!{MainFuncError}


#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
	let loads = r#"{
		"verbose|v" : "+"
	}"#;
	let optstr = r#"{}"#;
	let optref = ExtArgsOptions::new(optstr)?;
	let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None)?;
	extargs_load_commandline!(parser,loads)?;	
	let argsp = Arc::new(RefCell::new(MainDataStruct::new()));
	let ns = parser.parse_commandline_ex(None,None,Some(argsp.clone()),None)?;
	println!("subcommand={}", ns.get_string("subcommand"));
	println!("verbose={}", ns.get_int("verbose"));
	println!("args={:?}",ns.get_array("args"));

	return Ok(());
}

