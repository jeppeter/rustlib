#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};

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
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	let perr = extargs_load_commandline!(parser,loads);	
	if perr.is_err() {
		let errv = perr.err().unwrap();
		extargs_new_error!{MainFuncError,"load commandline [{}] erorr [{:?}]", loads,errv}
	}
	let argsp = Arc::new(RefCell::new(MainDataStruct::new()));
	let nserr = parser.parse_commandline_ex(None,None,Some(argsp.clone()),None);
	if nserr.is_err() {
		let errv = nserr.err().unwrap();
		extargs_new_error!{MainFuncError,"[{:?}]", errv}
	}

	let ns = nserr.unwrap();
	println!("subcommand={}", ns.get_string("subcommand"));
	println!("verbose={}", ns.get_int("verbose"));

	return Ok(());
}

