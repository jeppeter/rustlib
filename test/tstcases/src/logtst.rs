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

use super::{debug_trace,debug_warn,debug_error,debug_info,debug_debug,format_str_log};
use super::loglib::{log_get_timestamp,log_output_function,init_log};



fn logtst_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut times :i32 = 100;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() > 0 {
		match sarr[0].parse::<i32>() {
			Ok(n) => {times = n;},
			Err(_e) => {},
		}
	}

	for i in 0..times {
		debug_trace!("{} trace",i);
		debug_debug!("{} debug",i);
		debug_info!("{} info",i);
		debug_warn!("{} warn",i);
		debug_error!("{} error",i);
	}
	Ok(())
}

#[extargs_map_function(logtst_handler)]
pub fn load_log_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"logtst<logtst_handler>##[times] to debug times default 100##" : {
			"$" : "*"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}