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

extargs_error_class!{ProcHdlError}

struct ErrorResultInner {
	res :i32,
}

impl ErrorResultInner {
	fn new() -> Result<Self,Box<dyn Error>> {
		Ok(Self {
			res : 1,
		})
	}

	fn set_value(&mut self,nval :i32) -> i32 {
		let retv :i32 = self.res;
		self.res = nval;
		return retv;
	}

	fn get_value(&self) -> i32 {
		return self.res;
	}
}

#[derive(Clone)]
struct ErrorResult {
	inner :Arc<UnsafeCell<ErrorResultInner>>,
}

impl ErrorResult {
	fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			inner : Arc::new(UnsafeCell::new(ErrorResultInner::new()?)),
		})
	}

	fn set_value(&mut self, nval :i32) -> i32{
		let s1 :&mut ErrorResultInner = unsafe {&mut *self.inner.get()};
		return s1.set_value(nval);
	}

	fn get_value(&self) -> i32 {
		let s1 :&mut ErrorResultInner = unsafe {&mut *self.inner.get()};
		return s1.get_value();
	}
}

fn daemon_proc(infile:&str ,outfile :&str,errfile :&str, note :&str) -> Result<(),Box<dyn Error>> {
	debug_trace!("{} daemon",note);
	let mut infd :i32 = -1;
	let mut outfd :i32 = -1;
	let mut errfd :i32 = -1;
	let mut defercall :DeferCall = DeferCall::new();
	let mut resval :ErrorResult = ErrorResult::new()?;
	let nval = resval.clone();

	defercall.push(move || {
		let getval = nval.get_value();
		if getval != 0 {
			if infd >= 0 {
				unsafe {
					libc::close(infd);
				}
				infd = -1;
			}

			if outfd >= 0 {
				unsafe {
					libc::close(outfd);
				}
				outfd = -1;
			}

			if errfd >= 0 {
				unsafe {
					libc::close(errfd);
				}
				errfd = -1;
			}
		}
	});


	Ok(())
}


fn daemonize_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");
	let mut infile :String = "".to_string();
	let mut outfile :String = "".to_string();
	let mut errfile :String = "".to_string();
	let mut cnt :i32 = 0;

	init_log(ns.clone())?;
	if sarr.len() > 0 {
		outfile = format!("{}",sarr[0]);
	}

	if sarr.len() > 1 {
		errfile = format!("{}",sarr[1]);
	}

	if sarr.len() > 2 {
		infile = format!("{}",sarr[2]);
	}

	let _ = daemon_proc(&infile,&outfile,&errfile,"daemonize")?;

	loop {
		std::thread::sleep(std::time::Duration::from_millis(500));
		debug_trace!("cnt [{}]",cnt);
		cnt += 1;
	}


	Ok(())
}


#[extargs_map_function(timeval_handler)]
pub fn load_proc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"daemonize<daemonize_handler>##[outfile] [errfile] [infile] to handle file default /dev/null##" : {
			"$" : "*"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}