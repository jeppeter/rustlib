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

use winapi::shared::guiddef::{GUID};


extargs_error_class!{WinSetupError}


struct HwProp {
	guid :String,
	propidx :u32,
	buf :Vec<u8>,
}

struct HwInfo {
	pros :Vec<HwProp>,
}

macro_rules! GET_HEX_VAL {
	( $tv:expr, $ch :expr) => {
		$tv <<= 4;
		if $ch >= '0' as u8 && $ch <= '9' as u8 {
			$tv += ($ch - ('0' as u8)) as u32;
		} else if $ch >= 'a' as u8 && $ch <= 'f' as u8 {
			$tv += ($ch - ('a' as u8) + 10) as u32;
		} else if $ch >= 'A' as u8 && $ch <= 'F' as u8 {
			$tv += ($ch - ('A' as u8) + 10) as u32;
		} else {
			extargs_new_error!{WinSetupError,"not valid char [{}]", $ch}
		}
	}
}

fn parse_guid(ins :&str) -> Result<GUID,Box<dyn Error>> {
	let mut retv :GUID = GUID::default();
	let bs = ins.as_bytes();
	let mut cv :u32;
	if bs[0] != ('{' as u8) {
		extargs_new_error!{WinSetupError,"bs[0] [{}] != {{", bs[0] as char}
	}

	cv = 0;
	for i in 1..9 {
		GET_HEX_VAL!(cv,bs[i]);
	}

	retv.Data1 = cv;
	if bs[9] != '-' as u8 {
		extargs_new_error!{WinSetupError,"bs[9] [{}] != -",bs[9] as char}
	}

	cv = 0;
	for i in 10..14 {
		GET_HEX_VAL!(cv,bs[i]);
	}

	retv.Data2 = cv as u16;

	if bs[14] != '-' as u8 {
		extargs_new_error!{WinSetupError,"bs[14] [{}] != -",bs[14] as char}
	}

	cv = 0;
	for i in 15..19 {
		GET_HEX_VAL!(cv,bs[i]);
	}

	retv.Data3 = cv as u16;

	if bs[19] != '-' as u8 {
		extargs_new_error!{WinSetupError,"bs[19] [{}] != -",bs[19] as char}
	}

	cv = 0;
	for i in 20..24 {
		GET_HEX_VAL!(cv,bs[i]);
		if (i % 2) != 0 {
			retv.Data4[((i- 20) / 2)] = cv as u8;
			cv = 0;			
		}
	}

	if bs[24] != '-' as u8 {
		extargs_new_error!{WinSetupError,"bs[24] [{}] != -",bs[24] as char}
	}

	cv = 0;
	for i in 25..37 {
		GET_HEX_VAL!(cv,bs[i]);
		if (i % 2) == 0 {
			retv.Data4[(i- 21)/2] = cv as u8;
			cv = 0;
		}
	}

	if bs[37] != '}' as u8 {
		extargs_new_error!{WinSetupError,"bs[37] [{}] != }}",bs[37] as char}
	}

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
    debug_trace!("guid {:?}",guidget);
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