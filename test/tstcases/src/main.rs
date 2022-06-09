#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::options::{ExtArgsOptions};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};
#[allow(unused_imports)]
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};


#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use winreg::enums::*;
use winreg::RegKey;
use winreg::RegValue;

extargs_error_class!{NParseError}


const KEYWORD_HKLM :&str = "HKLM";
const KEYWORD_HCU :&str = "HCU";

fn regread_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let regk :RegKey;
	let mut idx :usize = 0;
	let kpath :&str;
	let mut cv :&str = "";
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}

	if sarr[idx] == KEYWORD_HKLM {
		regk = RegKey::predef(HKEY_LOCAL_MACHINE);
		idx += 1;
	} else if sarr[idx] == KEYWORD_HCU {
		regk = RegKey::predef(HKEY_CURRENT_USER);
		idx += 1;
	} else {
		regk = RegKey::predef(HKEY_LOCAL_MACHINE);
	}

	if sarr.len() <= idx {
		extargs_new_error!{NParseError,"need path value"}
	}

	kpath = &sarr[idx];
	idx += 1;

	let ckey = regk.open_subkey(kpath)?;
	let val :RegValue ;

	if sarr.len() > idx {
		cv = &sarr[idx];
		idx += 1;
	}
	val = ckey.get_raw_value(cv)?;

	println!("open [{}].[{}] value {:?}", kpath, cv,val);

	return Ok(());
}

#[extargs_map_function(regread_handler)]
fn main() -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"verbose|v" : "+",
		"regread<regread_handler>## HKLM|HKCU path key ##" : {
			"$" : "+"
		}
	}
	"#;
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	extargs_load_commandline!(parser,cmdline)?;
	let _ = parser.parse_commandline_ex(None,None,None,None)?;
	return Ok(());
}
