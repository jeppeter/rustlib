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
use std::slice;
use std::ffi::OsStr;
use winreg::enums::*;
use winreg::{RegValue,RegKey};
use winapi::um::winreg as winapi_reg;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::fs;

use super::{debug_trace};
use super::loglib::{log_get_timestamp,log_output_function,init_log};
//use super::loglib::{init_log};




extargs_error_class!{NParseError}


const KEYWORD_HKLM :&str = "HKLM";
const KEYWORD_HCU :&str = "HCU";
const KEYWORD_HKCR :&str = "HKCR";
const KEYWORD_HKU :&str = "HKU";
const KEYWORD_HKCC :&str = "HKCC";

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
		retk = RegKey::predef(HKEY_LOCAL_MACHINE);
		step += 1;
	} else if ktype == KEYWORD_HCU {
		retk = RegKey::predef(HKEY_CURRENT_USER);
		step += 1;
	} else if ktype == KEYWORD_HKCR {
		retk = RegKey::predef(HKEY_CLASSES_ROOT);
		step += 1;
	} else if ktype == KEYWORD_HKCC {
		retk = RegKey::predef(HKEY_CURRENT_CONFIG);
		step += 1;
	} else if ktype == KEYWORD_HKU {
		retk = RegKey::predef(HKEY_USERS);
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

fn open_reg_key_inner(ktype :&str,kpath :&str, perms :winapi_reg::REGSAM) -> Result<RegKey,Box<dyn Error>> {
	let (regk,_) =  get_regk(ktype);
	let ckey :RegKey = regk.open_subkey_with_flags(kpath,perms)?;
	return Ok(ckey);
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


fn open_reg_key(sarr :Vec<String>,perms :winapi_reg::REGSAM) -> Result<(RegKey,usize),Box<dyn Error>> {
	let mut step :usize;
	let regk :RegKey;
	let mut idx :usize = 0;

	(regk,step) = get_regk(&sarr[idx]);
	idx += step;
	let ckey :RegKey = regk.open_subkey_with_flags(&sarr[idx],perms)?;
	step += 1;
	return Ok((ckey,step));
}

#[allow(unused_assignments)]
fn regread_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut idx :usize = 0;
	let kpath :&str;
	let mut cv :&str = "";

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}

	let (ckey,step) = open_reg_key(sarr.clone(),KEY_READ)?;
	kpath = &sarr[(step-1)];
	idx = idx + step;
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
	let mut idx :usize = 0;
	let mut cv :&str = "";
	let mut typesarr :Vec<String> = Vec::new();

	init_log(ns.clone())?;


	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}


	let (ckey,step) = open_reg_key(sarr.clone(),KEY_WRITE)?;
	idx = idx + step;

	if sarr.len() <= idx {
		extargs_new_error!{NParseError,"need path value"}
	}


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

fn get_keys(k :&RegKey) -> Vec<String> {
	let mut retv :Vec<String> = Vec::new();
	for kv in k.enum_keys() {
		retv.push(format!("{}",kv.unwrap()));
	}
	retv
}

fn get_values(k :&RegKey) -> HashMap<String,RegValue> {
	let mut retv :HashMap<String,RegValue> = HashMap::new();
	for kp in k.enum_values() {
		let (kn,kv) = kp.unwrap();
		let nk :String = format!("{}",kn);
		let nv :RegValue = RegValue {
			bytes : kv.bytes.clone(),
			vtype : kv.vtype,
		};
		retv.insert(nk,nv);
	}	
	retv
}

#[allow(unused_assignments)]
fn regenum_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let kpath :&str;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{NParseError,"need at least path"}
	}

	let (ckey,step) = open_reg_key(sarr.clone(),KEY_READ)?;
	kpath = &(sarr[(step-1)]);

	let ks = get_keys(&ckey);
	let vs = get_values(&ckey);
	let mut i :usize = 0;
	println!("subkeys[{}]", kpath);
	for k in ks.iter() {
		println!("[{}]=[{}]",i,k);
		i += 1;
	}

	i = 0;
	println!("values[{}]", kpath);
	for (kk,kv) in vs.iter() {
		println!("[{}].[{}]=[{:?}]",i,kk,kv);
		i += 1;
	}

	Ok(())
}

fn expand_environ_val(kp :&str) -> String {
	let mut retv :String = format!("{}",kp).to_lowercase();

	for (k,v) in std::env::vars() {
		let kl = k.to_lowercase();
		let re :Regex = Regex::new(&format!("%{}%",kl)).unwrap();
		let vv :String = format!("{}",v);
		retv = re.replace_all(&retv,&vv).to_string().to_lowercase();
	}
	return retv;
}

fn get_environ_paths() -> Vec<String> {
	let mut retv :Vec<String>= Vec::new();
	for (k,v) in std::env::vars() {
		let kl = k.to_lowercase();
		if kl == "path" {
			let re = Regex::new(";").unwrap();
			for ks in re.split(&v).into_iter() {
				retv.push(format!("{}",ks));
			}
			break;
		}
	}
	return retv;
}

fn listabandoncom_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	init_log(ns.clone())?;
	let ckey :RegKey = open_reg_key_inner(KEYWORD_HKCR,"CLSID",KEY_READ)?;
	let subkeys :Vec<String> = get_keys(&ckey);
	let mut abondans :HashMap<String,String> = HashMap::new();
	let mut envpaths :Vec<String> = Vec::new();

	init_log(ns.clone())?;

	for k in subkeys.iter() {
		//debug_trace!("k [{}]",k);
		let cpath = format!("CLSID\\{}\\InprocServer32",k);
		let ores = open_reg_key_inner(KEYWORD_HKCR,&cpath,KEY_READ);
		if ores.is_ok() {
			let bkey = ores.unwrap();
			let vs = get_values(&bkey);
			let mut bmatch : bool = false;
			for (kk,kv) in vs.iter() {
				if kk == "" {
					bmatch = true;
					let kpath = format!("{}",kv);
					let kpath = format!("{}",kpath.trim_start_matches("\""));
					let kpath = format!("{}",kpath.trim_start_matches("\\"));
					let kpath = format!("{}",kpath.trim_start_matches("\""));
					let kpath = format!("{}",kpath.trim_end_matches("\""));
					let kpath = format!("{}",kpath.trim_end_matches("\\"));
					let kpath = format!("{}",kpath.trim_end_matches("\""));
					let kpath = format!("{}",kpath.trim_start());
					let kpath = format!("{}",kpath.trim_end());
					let kpath = kpath.replace("\\\\","\\").to_lowercase();
					let kpath = expand_environ_val(&kpath);
					let npath  = Path::new(&kpath);
					if npath.is_absolute() {
						let ometadata = fs::metadata(npath);
						if ometadata.is_err() {
							abondans.insert(format!("{}",k),format!("{}",kv));
						} else {
							let metadata = ometadata.unwrap();
							if !metadata.is_file() {
								abondans.insert(format!("{}",k),format!("{}",kv));	
							}							
						}
					} else {
						if envpaths.len() == 0 {
							envpaths = get_environ_paths();
						}
						let mut bfind :bool = false;
						for k in envpaths.iter() {
							let cpath = Path::new(k).join(&kpath);
							let ometadata = fs::metadata(cpath);
							if ometadata.is_ok() {
								let metadata = ometadata.unwrap();
								if metadata.is_file() {
									bfind = true;
									break;
								}
							}
						}

						if !bfind {
							abondans.insert(format!("{}",k),format!("not found [{}]",kpath));
						}
					}
					break;
				}
			}

			if !bmatch {
				abondans.insert(format!("{}",k),format!("no InprocServer32\\"));
			}
		}
	}

	for (k,v) in abondans.iter() {
		println!("[{}]=[{}]",k,v);
	}

	Ok(())
}

#[extargs_map_function(regread_handler,regwrite_handler,regenum_handler,listabandoncom_handler)]
pub fn load_reg_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"regread<regread_handler>## [HKLM|HCU|HKCR|HKCC|HKU] path [key] ##" : {
			"$" : "+"
		},
		"regwrite<regwrite_handler>## [HKLM|HCU|HKCR|HKCC|HKU] path key [type] [value] ##" : {
			"$" : "+"
		},
		"regenum<regenum_handler>## [HKLM|HCU|HKCR|HKCC|HKU] path to enum keyname ##" : {
			"$" : "+"
		},
		"listabandoncom<listabandoncom_handler>## to list abondan class id##" : {
			"$" : 0
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}