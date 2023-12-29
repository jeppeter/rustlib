use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

//use super::{debug_trace,debug_warn,debug_error,debug_info,debug_debug,format_str_log};
//use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::loglib::{init_log};
use serde::{Deserialize, Serialize};
use super::fileop::{read_file};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}

fn serdeperson_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;


	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let s = read_file(f)?;
		let p :Person = serde_json::from_str(&s)?;

		//let p :Person = ores.unwrap();
		println!("{}\n{:?}",f, p);
		let j :String = serde_json::to_string(&p)?;
		println!("to_string\n{}", j);
	}
	Ok(())
}

#[extargs_map_function(serdeperson_handler)]
pub fn load_serde_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"serdeperson<serdeperson_handler>##inputname ...##" : {
			"$" : "*"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}