#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::io;
use std::fs;
use std::io::prelude::*;


use std::error::Error;

extargs_error_class!{FileOpError}


pub fn write_file(fname :&str, byts :&[u8]) -> Result<(),Box<dyn Error>> {
	if fname.len() == 0 {
		let res = io::stdout().write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}	
		}
	} else {
		let fo  = fs::File::create(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"create [{}] error[{:?}]", fname,err}
		}
		let mut fp :fs::File = fo.unwrap();
		let res = fp.write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [{}] len[{}] error[{:?}]", fname, byts.len(),err}	
		}
	}
	Ok(())
}