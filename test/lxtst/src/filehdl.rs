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
use super::*;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

extargs_error_class!{FileHdlError}

#[derive(Debug)]
struct FdStruct {
	fp : Option<std::fs::File>,
	fname :String,
	msgs :Vec<String>,
}

impl Drop for FdStruct {
	fn drop(&mut self) {
		self.close();
	}
}

impl FdStruct {

	fn close(&mut self) {
		if self.fp.is_some() {
			debug_trace!("close [{}]",self.fname);
		}
		self.fp = None;
		self.fname = format!("");
	}

	fn add_line(&mut self,lines :&[String]) -> Result<(),Box<dyn Error>>  {
		for l in lines.iter() {
			let c = format!("{}\n",l);
			if self.fp.is_some() {
				let p = self.fp.as_mut().unwrap();
				let _ = p.write_all(c.as_bytes())?;
			}
			self.msgs.push(c);
		}
		Ok(())
	}
	fn new(fname :&str) -> Result<Self,Box<dyn Error>> {
		let mut retv :Self = Self {
			fp : None,
			fname :format!("{}",fname),
			msgs :Vec::new(),
		};

		let mut fp :std::fs::File;

		let mut fres :std::io::Result<std::fs::File>; 
		fres = std::fs::OpenOptions::new().read(true).write(true).open(fname);
		if fres.is_ok(){
			{
				let fip = fres.unwrap();
				let mut fin = std::io::BufReader::new(fip);
				let mut rets :String = String::new();
				let res = fin.read_to_string(&mut rets);
				if res.is_err() {
					extargs_new_error!{FileHdlError,"read [{}] error {}",fname,res.err().unwrap()}
				}
				let sarr : Vec<&str> = rets.split("\n").collect();
				for s in sarr.iter() {
					let cs :String = format!("{}",s);
					retv.msgs.push(format!("{}",cs.trim_end_matches('\r')));
				}

			}

			fres = std::fs::OpenOptions::new().read(true).write(true).open(fname);
			fp = fres.unwrap();
			let bres = fp.seek(std::io::SeekFrom::End(0));
			if bres.is_err() {
				extargs_new_error!{FileHdlError,"seek [{}] error {}",fname,bres.err().unwrap()}
			}
		} else {
			fres = std::fs::File::create(fname);
			if fres.is_err() {
				extargs_new_error!{FileHdlError,"create [{}] error {}",fname,fres.err().unwrap()}
			}
			fp = fres.unwrap();
		}
		retv.fp = Some(fp);
		Ok(retv)
	}
}



fn reopen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");

	init_log(ns.clone())?;
	if sarr.len() < 1 {
		extargs_new_error!{FileHdlError,"need file ..."}
	}

	let fname = format!("{}",sarr[0]);
	let mut fd = FdStruct::new(&fname)?;

	let _ = fd.add_line(&sarr[1..])?;

	println!("fd\n{:?}",fd);

	Ok(())
}


#[extargs_map_function(reopen_handler)]
pub fn load_file_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"reopen<reopen_handler>##file lines... to reopen file##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}