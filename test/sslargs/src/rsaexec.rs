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

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;


#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};


fn genrsa_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	debug_trace!("F4 {}",ns.get_bool("F4"));
	debug_trace!("rand {}",ns.get_string("rand"));
	debug_trace!("writerand {}", ns.get_string("writerand"));
	debug_trace!("sarr {:?}",sarr);
	Ok(())
}


#[extargs_map_function(genrsa_handler)]
pub fn load_rsa_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"genrsa<genrsa_handler>##to gen rsa functions##" : {
			"$" : "*",
			"F4##Use F4 (0x10001) for the E value##" : false,
			"f4##Use F4 (0x10001) for the E value##" : false,
			"out" : null,
			"rand##Load the file(s) into the random number generator##" : null,
			"writerand##Write random data to the specified file##" : null,
			"passout##Output file pass phrase source##" : null,
			"engine##Use engine, possibly a hardware device##" : null,
			"primes##Specify number of primes##" : []
		}
	}
	"#;
	/**/
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
