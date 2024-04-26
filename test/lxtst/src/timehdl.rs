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

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
//use super::*;
//use std::io::Read;
//use std::io::Seek;
//use std::io::Write;
use chrono::prelude::*;
use crate::strop::{parse_u64};

extargs_error_class!{TimeHdlError}

fn time_trans_value(tmval :i64) -> String {
	let rets :String;
	let onative = NaiveDateTime::from_timestamp_opt(tmval,0);
	let dt :DateTime<Utc>;

	if onative.is_some() {
		let native = onative.unwrap();
		dt = DateTime::<Utc>::from_naive_utc_and_offset(native,Utc);
	} else {
		dt = chrono::offset::Utc::now();
	}
	

	let newdate = dt.format("%Y-%m-%d %H:%M:%S");
	rets = format!("{}",newdate);
	return rets;
}


fn timeval_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");

	init_log(ns.clone())?;
	if sarr.len() < 1 {
		extargs_new_error!{TimeHdlError,"need timeval"}
	}

	let mut timeval :i64 = parse_u64(&sarr[0])? as i64;
	timeval += 8* 3600;

	println!("0x{:x} {} [{}]", timeval,timeval,time_trans_value(timeval));
	Ok(())
}


#[extargs_map_function(timeval_handler)]
pub fn load_time_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"timeval<timeval_handler>##val to transform epoch seconds to string of year value ##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}