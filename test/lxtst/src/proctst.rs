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
//use chrono::prelude::*;
//use crate::strop::{parse_u64};
use evtcall::defer::*;
use crate::*;

extargs_error_class!{ProcHdlError}

struct ErrorResultInner {
	res :i32,
}

impl ErrorResultInner {
	fn new(val :i32) -> Result<Self,Box<dyn Error>> {
		Ok(Self {
			res : val,
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
	fn new(val :i32) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			inner : Arc::new(UnsafeCell::new(ErrorResultInner::new(val)?)),
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

const DEV_NULL :&str = "/dev/null";

#[allow(unused_assignments)]
fn daemon_proc(infile:&str ,outfile :&str,errfile :&str, note :&str) -> Result<(),Box<dyn Error>> {
	debug_trace!("{} daemon",note);
	let mut infd :i32 = -1;
	let mut outfd :i32 = -1;
	let mut errfd :i32 = -1;
	let mut defercall :DeferCall = DeferCall::new();
	let mut resval :ErrorResult = ErrorResult::new(1)?;
	let nval = resval.clone();
	let mut rawinbytes :Vec<u8> = DEV_NULL.as_bytes().to_vec().clone();
	let mut rawoutbytes :Vec<u8> = DEV_NULL.as_bytes().to_vec().clone();
	let mut rawerrbytes :Vec<u8> = DEV_NULL.as_bytes().to_vec().clone();
	let mut reti :libc::c_int;

	if infile.len() > 0 {
		rawinbytes = infile.as_bytes().to_vec().clone();
	}

	if outfile.len() > 0 {
		rawoutbytes = outfile.as_bytes().to_vec().clone();
	}

	if errfile.len() > 0 {
		rawerrbytes = errfile.as_bytes().to_vec().clone();
	}

	/*to make string '\0'*/
	rawinbytes.push(0);
	rawoutbytes.push(0);
	rawerrbytes.push(0);



	defercall.push_call(move || {
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

	unsafe {
		let _ptr = rawinbytes.as_ptr() as *const libc::c_char;
		infd = libc::open(_ptr,libc::O_RDONLY,0);
	}

	if infd < 0 {
		reti = get_errno!();
		extargs_new_error!{ProcHdlError,"open [{}] error [{}]",infile,reti}
	}

	unsafe {
		let _ptr = rawoutbytes.as_ptr() as *const libc::c_char;
		outfd = libc::open(_ptr,libc::O_RDWR,0);
	}

	if outfd < 0 {
		reti = get_errno!();
		extargs_new_error!{ProcHdlError,"open [{}] error [{}]",outfile,reti}
	}


	unsafe {
		let _ptr = rawerrbytes.as_ptr() as *const libc::c_char;
		errfd = libc::open(_ptr,libc::O_RDWR,0);
	}

	if errfd < 0 {
		reti = get_errno!();
		extargs_new_error!{ProcHdlError,"open [{}] error [{}]",errfile,reti}
	}

	unsafe {
		reti = libc::dup2(infd,0);
	}
	if reti < 0 {
		extargs_new_error!{ProcHdlError,"can not dup2 [{}] to stdin",infile}
	}


	unsafe {
		reti = libc::dup2(outfd,1);
	}
	if reti < 0 {
		extargs_new_error!{ProcHdlError,"can not dup2 [{}] to stdout",outfile}
	}

	unsafe {
		reti = libc::dup2(errfd,2);
	}
	if reti < 0 {
		extargs_new_error!{ProcHdlError,"can not dup2 [{}] to stderr",errfile}
	}

	unsafe {
		reti = libc::fork();
	}

	if reti < 0 {
		extargs_new_error!{ProcHdlError,"can not fork"}
	} else if reti > 0 {
		/*we exit*/
		unsafe {
			libc::exit(0);
		}
	}

	unsafe {
		libc::close(infd);
		libc::close(outfd);
		libc::close(errfd);		
	}

	infd = -1;
	outfd = -1;
	errfd = -1;

	/*ok all is ok*/


	resval.set_value(0);

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
		if cnt > 100 {
			break;
		}
		std::thread::sleep(std::time::Duration::from_millis(500));
		debug_trace!("cnt [{}]",cnt);
		cnt += 1;
	}


	Ok(())
}


#[extargs_map_function(daemonize_handler)]
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