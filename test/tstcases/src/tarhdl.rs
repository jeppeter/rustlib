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
#[allow(unused_imports)]
use super::*;
use tar::Builder;
use std::io::Write;

extargs_error_class!{TarHdlError}

struct FileOutput {
	out :Vec<std::io::Stdout>,
	file :Vec<std::fs::File>,
}

impl FileOutput {
	fn new(n :&str) -> Result<Self,Box<dyn Error>> {
		let  mut retv :Self = Self {
			out : vec![],
			file :vec![],
		};

		if n.len() == 0 {
			retv.out.push(std::io::stdout());
		} else {
			retv.file.push(std::fs::File::create(n)?);
		}
		Ok(retv)
	}
}

impl Write for FileOutput {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		if self.out.len() > 0 {
			return self.out[0].write(buf);
		} else if self.file.len() > 0 {
			return self.file[0].write(buf);
		}
		return Err(std::io::Error::new(std::io::ErrorKind::Other,"no specified"));
	}
	fn flush(&mut self) -> std::io::Result<()> {
		if self.out.len() > 0 {
			return self.out[0].flush();
		} else if self.file.len() > 0 {
			return self.file[0].flush();
		}
		return Err(std::io::Error::new(std::io::ErrorKind::Other,"no specified"));
	}
}


fn tarcreate_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");
	let output :String = ns.get_string("output");
	let mut tarout :tar::Builder<FileOutput>;

	init_log(ns.clone())?;
	if sarr.len() < 1 {
		extargs_new_error!{TarHdlError,"need file ..."}
	}

	tarout = Builder::new(FileOutput::new(&output)?);


	for f in sarr.iter() {
		let narr :Vec<&str> = f.split(":").collect();
		if narr.len() <= 1 {
			tarout.append_path(narr[0])?;
		} else {
			let mut infile :std::fs::File = std::fs::File::open(narr[0])?;
			tarout.append_file(narr[1],&mut infile)?;
		}
	}

	tarout.finish()?;
	Ok(())
}

fn tardelete_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");
	let output :String = ns.get_string("output");
	let mut tarout :tar::Builder<FileOutput>;

	init_log(ns.clone())?;
	if sarr.len() < 1 {
		extargs_new_error!{TarHdlError,"need file ..."}
	}

	tarout = Builder::new(FileOutput::new(&output)?);


	for f in sarr.iter() {
		let narr :Vec<&str> = f.split(":").collect();
		if narr.len() <= 1 {
			tarout.append_path(narr[0])?;
		} else {
			let mut infile :std::fs::File = std::fs::File::open(narr[0])?;
			tarout.append_file(narr[1],&mut infile)?;
		}
	}

	tarout.finish()?;
	Ok(())
}



#[extargs_map_function(tarcreate_handler,tardelete_handler)]
pub fn load_tar_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"tarcreate<tarcreate_handler>##files... to create tar to output##" : {
			"$" : "+"
		},
		"tardel<tardelete_handler>##fname ... to delete file##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}