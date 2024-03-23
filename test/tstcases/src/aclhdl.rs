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
use super::{debug_trace};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::reglib::{open_reg_key,format_reg_value,get_reg_keys,get_reg_values,REG_HKCR,reg_del_val,reg_del_key,reg_create_key};


extargs_error_class!{AclHdlError}


#[allow(unused_assignments)]
fn acllist_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let fperm :FilePerm = FilePerm::new(f)?;
		println!("{} perm {:?}", f, fperm);
	}

	return Ok(());
}



#[extargs_map_function(regread_handler,regwrite_handler,regenum_handler,abandonedcomkeys_handler,regdelval_handler,regdelkey_handler,regcreatekey_handler,comhunter_handler)]
pub fn load_acl_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"acllist<acllist_handler>##file... to list type ##" : {
			"$" : "*"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}