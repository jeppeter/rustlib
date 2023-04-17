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
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use winapi::shared::guiddef::*;

struct HwProp {
	guid :String,
	propidx :u32,
	buf :Vec<u8>,
}

struct HwInfo {
	pros :Vec<HwProp>,
}

fn parse_guid(ins :&str) -> Result<GUID,Box<dyn Error>> {
	let mut retv :GUID = GUID::default();
	Ok(retv)
}

fn get_hw_infos(guid :* const GUID) -> Result<Vec<HwInfo>,Box<dyn Error>> {
	let mut retv :Vec<HwInfo> = Vec::new();

	Ok(retv)
}

fn lshwinfo_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut ptrguid :* const GUID = std::ptr::null_mut();
    let mut guidget :GUID = GUID::default();

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");

    if sarr.len() > 0 {
    	guidget = parse_guid(&sarr[0])?;
    	ptrguid = &guidget;
    }
    let hwinfos = get_hw_infos(ptrguid)?;

    Ok(())
}


#[extargs_map_function(lshwinfo_handler)]
pub fn load_ecc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
    	"lshwinfo<lshwinfo_handler>##[guids]... to list handle of guids##" : {
    		"$" : "*"
    	}
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}