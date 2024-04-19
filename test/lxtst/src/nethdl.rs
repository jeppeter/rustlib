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
use super::netlib::{format_sinaddr_in,NetDevFd};
use super::*;

extargs_error_class!{NetHdlError}

fn sockaddrinfmt_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");
	let port :u32;
	let ipaddr :String;
	let retv :libc::sockaddr_in;

	init_log(ns.clone())?;
	if sarr.len() < 2 {
		extargs_new_error!{NetHdlError,"need ipaddr port"}
	}

	ipaddr = format!("{}",sarr[0]);
	port = parse_u64(&sarr[1])? as u32;
	
	retv = format_sinaddr_in(&ipaddr,port)?;
	debug_buffer_trace!((&retv as *const libc::sockaddr_in ),std::mem::size_of::<libc::sockaddr_in>(),"sinaddr_in");


	Ok(())
}

fn nicstate_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>  = ns.get_array("subnargs");

	init_log(ns.clone())?;
	if sarr.len() < 1 {
		extargs_new_error!{NetHdlError,"need ethname ..."}
	}

	for s in sarr.iter() {
		let dev :NetDevFd = NetDevFd::new(s)?;
		println!("[{}]ipaddr [{}] netmask [{}] defgate [{}] dns {:?}",s,dev.get_ipaddr()?,dev.get_netmask()?,dev.get_default_gateway()?,dev.get_dns()?);
	}

	Ok(())
}


#[extargs_map_function(sockaddrinfmt_handler,nicstate_handler)]
pub fn load_net_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"sockaddrinfmt<sockaddrinfmt_handler>##ip port to format##" : {
			"$" : 2
		},
		"nicstate<nicstate_handler>##ethname ... to get state##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}