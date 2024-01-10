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


use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::strop::{parse_u64};
use super::*;
use std::sync::atomic::{AtomicU16,Ordering};

extargs_error_class!{SigHdlError}

static ST_SIGINITED : AtomicU16 = AtomicU16::new(0);
static ST_EXITVAL : AtomicU16 = AtomicU16::new(0);

fn rust_signal(_iv :libc::c_int) {
	ST_EXITVAL.store(1,Ordering::SeqCst);
	return;
}

unsafe extern "system" fn notice_signal(iv : libc::c_int)  {
	rust_signal(iv);
	return;
}

fn get_notice_signal() -> libc::sighandler_t {
	notice_signal as *mut libc::c_void as libc::sighandler_t
}


fn init_sig_value(sigv :&[i32]) -> Result<(),Box<dyn Error>> {
	let mut reti :libc::sighandler_t;
	if ST_SIGINITED.load(Ordering::SeqCst) == 0 {
		for v in sigv.iter() {
			unsafe {
				reti = libc::signal(*v,get_notice_signal());
			}
			if reti == libc::SIG_ERR {
				extargs_new_error!{SigHdlError,"cannot set {} signal",*v}
			}
		}

		ST_SIGINITED.store(1,Ordering::SeqCst);
	}
	Ok(())
}

fn fini_sig_value() {
	ST_SIGINITED.store(0,Ordering::SeqCst);
}

fn sigtst_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut sigv :Vec<libc::c_int> = Vec::new();
	let mut count :i32 = 0;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{SigHdlError,"need 1 arg"}
	}

	for v in sarr.iter() {
		sigv.push(parse_u64(v)? as libc::c_int);
	}

	init_sig_value(&sigv)?;
	while ST_EXITVAL.load(Ordering::SeqCst)  == 0 {
		debug_trace!("at {} count",count);
		count += 1;
		std::thread::sleep(std::time::Duration::from_millis(1000));
	}
	debug_trace!("return count {}",count);

	fini_sig_value();

	Ok(())
}


#[extargs_map_function(sigtst_handler)]
pub fn load_sig_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"sigtst<sigtst_handler>##ip port to format##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}