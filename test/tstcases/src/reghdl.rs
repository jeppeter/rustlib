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
use std::path::Path;
use std::fs;
use winreg::enums::*;
use winreg::{RegValue,RegKey};


use super::{debug_trace};
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::reglib::{open_reg_key,format_reg_value,get_reg_keys,get_reg_values,REG_HKCR,reg_del_val,reg_del_key,reg_create_key};
//use super::loglib::{init_log};


extargs_error_class!{NParseError}


#[allow(unused_assignments)]
fn regread_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut idx :usize = 0;
	let kpath :&str;
	let mut cv :&str = "";
	let ckey ;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{NParseError,"need 1 args"}
	}

	if sarr.len() > 1 {
		ckey = open_reg_key(&sarr[0],&sarr[1],KEY_READ)?;
		kpath = &sarr[1];
		idx = 2;		
	} else {
		ckey = open_reg_key(&sarr[0],"", KEY_READ)?;
		kpath = "";
		idx = 1;
	}
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
	let mut idx :usize;
	let mut cv :&str = "";
	let mut typesarr :Vec<String> = Vec::new();
	let ckey ;

	init_log(ns.clone())?;


	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need 1 args"}
	}


	ckey = open_reg_key(&sarr[0],&sarr[1],KEY_WRITE)?;
	idx = 2;

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

	val = format_reg_value(typesarr.clone());
	ckey.set_raw_value(cv,&val)?;

	println!("{:?} succ", sarr);

	return Ok(());
}

#[allow(unused_assignments)]
fn regenum_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let kpath :&str;
	let ckey ;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{NParseError,"need at least path"}
	}

	if sarr.len() > 1 {
		ckey = open_reg_key(&sarr[0],&sarr[1],KEY_READ)?;
		kpath = &(sarr[1]);
	} else {
		ckey = open_reg_key(&sarr[0],"", KEY_READ)?;
		kpath = "";
	}

	let ks = get_reg_keys(&ckey);
	let vs = get_reg_values(&ckey);
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

fn abandonedcomkeys_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	init_log(ns.clone())?;
	let ckey :RegKey = open_reg_key(REG_HKCR,"CLSID",KEY_READ)?;
	let subkeys :Vec<String> = get_reg_keys(&ckey);
	let mut abondans :HashMap<String,String> = HashMap::new();
	let mut envpaths :Vec<String> = Vec::new();

	init_log(ns.clone())?;

	for k in subkeys.iter() {
		//debug_trace!("k [{}]",k);
		let cpath = format!("CLSID\\{}\\InprocServer32",k);
		let ores = open_reg_key(REG_HKCR,&cpath,KEY_READ);
		if ores.is_ok() {
			let bkey = ores.unwrap();
			let vs = get_reg_values(&bkey);
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

fn regdelval_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr= ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{NParseError,"need ktype kpath valpath"}
	}
	return reg_del_val(&sarr[0],&sarr[1],&sarr[2]);
}


fn regdelkey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr= ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{NParseError,"need ktype kpath"}
	}
	return reg_del_key(&sarr[0],&sarr[1]);
}

fn regcreatekey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr= ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{NParseError,"need ktype kpath key"}
	}
	return reg_create_key(&sarr[0],&sarr[1],&sarr[2]);
}


#[extargs_map_function(regread_handler,regwrite_handler,regenum_handler,abandonedcomkeys_handler,regdelval_handler,regdelkey_handler,regcreatekey_handler)]
pub fn load_reg_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"regread<regread_handler>## HKLM|HCU|HKCR|HKCC|HKU path [key] ##" : {
			"$" : "+"
		},
		"regwrite<regwrite_handler>## HKLM|HCU|HKCR|HKCC|HKU path key [type] [value] ##" : {
			"$" : "+"
		},
		"regenum<regenum_handler>## HKLM|HCU|HKCR|HKCC|HKU path to enum keyname ##" : {
			"$" : "+"
		},
		"abandonedcomkeys<abandonedcomkeys_handler>## to list abondan class id##" : {
			"$" : 0
		},
		"regdelval<regdelval_handler>## HKLM|HCU|HKCR|HKCC|HKU path [key] ##" : {
			"$" : "+"
		},
		"regdelkey<regdelkey_handler>## HKLM|HCU|HKCR|HKCC|HKU path ##" : {
			"$" : "+"
		},
		"regcreatekey<regcreatekey_handler>## HKLM|HCU|HKCR|HKCC|HKU path key ##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}