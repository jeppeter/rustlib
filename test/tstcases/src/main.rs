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
use std::slice;
use std::ffi::OsStr;
use winreg::enums::*;
use winreg::{RegValue,RegKey};
use std::os::windows::ffi::OsStrExt;

extargs_error_class!{NParseError}


const KEYWORD_HKLM :&str = "HKLM";
const KEYWORD_HCU :&str = "HCU";

const TYPE_REG_SZ :&str = "REG_SZ";
const TYPE_REG_EXPAND_SZ :&str = "REG_EXPAND_SZ";
const TYPE_REG_BINARY :&str = "REG_BINARY";
const TYPE_REG_DWORD :&str = "REG_DWORD";
const TYPE_REG_QWORD :&str = "REG_QWORD";
const TYPE_REG_DWORD_BIG_ENDIAN :&str = "REG_DWORD_BIG_ENDIAN";
const TYPE_REG_MULTI_SZ :&str = "REG_MULTI_SZ";

fn get_regk(ktype :&str) -> (RegKey,usize) {
	let mut step :usize = 0;
	let retk :RegKey;
	if ktype == KEYWORD_HKLM {
		retk = RegKey::predef(HKEY_CURRENT_USER);
		step += 1;
	} else if ktype == KEYWORD_HCU {
		retk = RegKey::predef(HKEY_CURRENT_USER);
		step += 1;
	} else {
		retk = RegKey::predef(HKEY_LOCAL_MACHINE);
	}
	return (retk,step);
}

fn main_to_utf16<P: AsRef<OsStr>>(s: P) -> Vec<u16> {
	s.as_ref()
	.encode_wide()
	.chain(Some(0).into_iter())
	.collect()
}

fn main_v16_to_v8(v: &[u16]) -> Vec<u8> {
	unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 2).to_vec() }
}



fn get_reg_value(v :Vec<String>) -> RegValue {
	let mut idx :usize;

	if v.len() > 0 {
		if v[0] == TYPE_REG_SZ {
			if v.len() == 1 {
				return RegValue { 
					bytes : main_v16_to_v8(&main_to_utf16("\0")),
					vtype : REG_SZ,
				};
			}
			let cstr = format!("{}",v[1]);

			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(cstr)),
				vtype : REG_SZ,
			};
		} else if v[0] == TYPE_REG_EXPAND_SZ {
			if v.len() == 1 {
				return RegValue { 
					bytes : main_v16_to_v8(&main_to_utf16("\0")),
					vtype : REG_EXPAND_SZ,
				};
			}
			let cstr = format!("{}",v[1]);

			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(cstr)),
				vtype : REG_EXPAND_SZ,
			};
		} else if v[0] == TYPE_REG_BINARY {
			let mut retb :Vec<u8> = Vec::new();
			if v.len() == 1 {
				retb.push(0 as u8);
				return RegValue {
					bytes : retb,
					vtype : REG_BINARY,
				};
			}

			idx = 1;
			while idx < v.len() {
				retb.push(v[idx].parse::<u8>().unwrap());
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_BINARY,
			};
		} else if v[0] == TYPE_REG_DWORD {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u32 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u32>().unwrap();
			}
			idx = 0;
			while idx < 4 {
				let curb :u8 = ((dval >> (idx * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_DWORD,
			};
		} else if v[0] == TYPE_REG_QWORD {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u64 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u64>().unwrap();
			}
			idx = 0;
			while idx < 8 {
				let curb :u8 = ((dval >> (idx * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_QWORD,
			};
		} else if v[0] == TYPE_REG_DWORD_BIG_ENDIAN {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u32 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u32>().unwrap();
			}
			idx = 0;
			while idx < 4 {
				let curb :u8 = ((dval >> ((3-idx) * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_DWORD_BIG_ENDIAN,
			};			
		} else if v[0] == TYPE_REG_MULTI_SZ {
			let mut s :String = "".to_string();

			if v.len() > 1 {
				idx = 1;
				while idx < v.len() {
					s.push_str(&v[idx]);
					idx += 1;
				}
			}
			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(&s)),
				vtype : REG_MULTI_SZ,
			};
		}

	}
	return RegValue {
		bytes : main_v16_to_v8(&main_to_utf16("\0")),
		vtype : REG_SZ,
	};

}

fn regread_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let regk :RegKey;
	let step :usize;
	let mut idx :usize = 0;
	let kpath :&str;
	let mut cv :&str = "";
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}

	(regk, step) = get_regk(&sarr[idx]);
	idx = idx + step;

	if sarr.len() <= idx {
		extargs_new_error!{NParseError,"need path value"}
	}

	kpath = &sarr[idx];
	idx = idx + 1;

	let ckey = regk.open_subkey(kpath)?;
	let val :RegValue ;

	if sarr.len() > idx {
		cv = &sarr[idx];
		idx = idx + 1;
	}
	val = ckey.get_raw_value(cv)?;

	println!("open [{}].[{}] value {:?}", kpath, cv,val);

	return Ok(());
}

fn regwrite_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let regk :RegKey;
	let step :usize;
	let mut idx :usize = 0;
	let kpath :&str;
	let mut cv :&str = "";
	let mut typesarr :Vec<String> = Vec::new();
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}

	(regk, step) = get_regk(&sarr[idx]);
	idx = idx + step;

	if sarr.len() <= idx {
		extargs_new_error!{NParseError,"need path value"}
	}

	kpath = &sarr[idx];
	idx = idx + 1;

	let ckey = regk.open_subkey(kpath)?;
	let val :RegValue ;

	if idx < sarr.len() {
		cv = &sarr[idx];
		idx += 1;
	}


	while idx < sarr.len() {
		typesarr.push(format!("{}",sarr[idx]));
		idx += 1;
	}

	val = get_reg_value(typesarr.clone());
	ckey.set_raw_value(cv,&val)?;

	println!("{:?} succ", sarr);

	return Ok(());
}


#[extargs_map_function(regread_handler,regwrite_handler)]
fn main() -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"verbose|v" : "+",
		"regread<regread_handler>## [HKLM|HCU] path [key] ##" : {
			"$" : "+"
		},
		"regwrite<regwrite_handler>## [HKLM|HCU]  path type [key] [value] ##" : {
			"$" : "+"
		}
	}
	"#;
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	extargs_load_commandline!(parser,cmdline)?;
	let _ = parser.parse_commandline_ex(None,None,None,None)?;
	return Ok(());
}
