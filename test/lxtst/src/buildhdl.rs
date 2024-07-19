use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::{RefCell,UnsafeCell};
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use crate::fileop::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};


//const VERSION_INFO :&str = env!("VERSION");
//const COMPILE_TIME :&str = env!("COMPILE_TIME");
//const GIT_HASH :&str = env!("GIT_HASH");
const VERSION_INFO :&str = "2.3.0";
include!("version.rs");



fn versioninfo_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	println!("version {}",VERSION_INFO);
	println!("compile at {} gitversion {}", COMPILE_TIME,GIT_HASH);

	Ok(())

}


#[extargs_map_function(versioninfo_handler)]
pub fn load_build_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"versioninfo<versioninfo_handler>##to output version##" : {
			"$" : 0
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}