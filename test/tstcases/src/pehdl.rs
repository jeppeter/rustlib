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

use super::pelib::{get_securtiy_buffer,SecData};
use super::fileop::{write_file_bytes};



extargs_error_class!{PeHdlError}

fn pesecdata_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let mut secdata :SecData;
	let sarr :Vec<String>;
	//let mut lastidx :usize;
	let mut idx :usize = 0;
	let fname :String;


	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	fname = ns.get_string("output");
	for f in sarr.iter() {
		secdata = get_securtiy_buffer(f)?;
		let dlen :usize;
		let curname :String;

		if (secdata.buf.len() as u32) < secdata.size {
			dlen = secdata.buf.len() ;
		} else {
			dlen = secdata.size as usize;
		}

		if fname.len() == 0 {
			curname = "".to_string();
		} else {
			curname = format!("{}.{}",fname,idx);
		}

		debug_buffer_trace!(secdata.buf.as_ptr(), dlen, "{} security data [0x{:08x}].[0x{:08x}][0x{:08x}]:",f,secdata.virtaddr,secdata.size,secdata.buf.len());
		debug_trace!("write curname [{}]",curname);
		write_file_bytes(&curname,&(secdata.buf[..dlen]))?;
		/*
		print!("{} security data [0x{:08x}].[0x{:08x}][0x{:08x}]:",f,secdata.virtaddr,secdata.size,secdata.buf.len());
		lastidx = 0;
		idx = 0;
		for b in secdata.buf.iter() {
			if idx >= secdata.size as usize {
				break;
			}
			if (idx % 16) == 0 {
				if idx > 0 {
					print!("    ");
					while lastidx != idx {
						if secdata.buf[lastidx] >= 0x20 && secdata.buf[lastidx] <= 0x7e {
							print!("{}", secdata.buf[lastidx] as char );
						} else {
							print!(".");
						}
						lastidx += 1;
					}
				}
				print!("\n0x{:08x}",idx);
			}
			print!(" 0x{:02x}",  b);
			idx += 1;
		}

		if idx != lastidx {
			while (idx % 16) != 0 {
				print!("     ");
				idx += 1;
			}

			print!("    ");
			while lastidx < secdata.buf.len() && lastidx < secdata.size as usize {
				if secdata.buf[lastidx] >= 0x20 && secdata.buf[lastidx] <= 0x7e {
					print!("{}", secdata.buf[lastidx] as char );
				} else {
					print!(".");
				}
				lastidx += 1;				
			}
			print!("\n");
		}*/
		idx += 1;
	}


	Ok(())
}



#[extargs_map_function(pesecdata_handler)]
pub fn load_pe_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"pesecdata<pesecdata_handler>##file ... to display pe security data##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}

