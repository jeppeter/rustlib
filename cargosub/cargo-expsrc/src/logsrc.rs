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

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};


extargs_error_class!{ExpLogError}

fn explog_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let logname :String;
	let caplogname :String;
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{ExpLogError,"need logname [CAPITAL_LOGNAME]"}
	}
	logname = format!("{}",sarr[0]);
	if sarr.len() > 1 {
		caplogname = format!("{}",sarr[1]);
	} else {
		caplogname = logname.to_uppercase();
	}

	debug_trace!("logname [{}] caplogname [{}]",logname,caplogname);


	Ok(())
}



#[extargs_map_function(explog_handler)]
pub fn load_log_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"explog<explog_handler>##logname [CAPITAL_LOGNAME] ##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
