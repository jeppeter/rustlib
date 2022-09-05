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
use super::{debug_trace};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use super::credlib::{NetworkCredentials,cred_phisher};
use super::strop::{timesec_to_tm,parse_u64};

extargs_error_class!{NCredError}

//#[allow(unused_assignments)]
fn credphisher_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{NCredError,"need message"}
	}

	let cred :NetworkCredentials = cred_phisher(&sarr[0])?;

	println!("domain [{}] name [{}] password [{}]", cred.Domain,cred.Username,cred.Password);

	return Ok(());
}


fn timesectotm_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for v in sarr.iter() {
		let tsec :u64 = parse_u64(v)?;
		let ts = timesec_to_tm(tsec)?;
		println!("[{}]=[{}]", v,ts);
	}

	return Ok(());
}


#[extargs_map_function(credphisher_handler,timesectotm_handler)]
pub fn load_cred_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"credphisher<credphisher_handler>##message to display credphisher message ##" : {
			"$" : "+"
		},
		"timesectotm<timesectotm_handler>##timesec ... to make timestamp##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}

