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
use std::io::{Write};

use super::fileop::{read_file_bytes};

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use asn1obj_codegen::{asn1_sequence};
use asn1obj::base::{Asn1PrintableString,Asn1Object,asn1obj_extract_header,asn1obj_format_header};
use asn1obj::consts::{ASN1_SEQ_MASK};
use asn1obj::complex::{Asn1Seq,Asn1Set};
use asn1obj::strop::{asn1_format_line};
use asn1obj::asn1impl::{Asn1Op};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};



#[asn1_sequence(asn1seq=disable)]
struct Asn1X509NameElement {
	pub obj :Asn1Object,
	pub name :Asn1PrintableString,
}

#[asn1_sequence(asn1seq=disable)]
struct Asn1X509NameEntry {
	pub names : Asn1Set<Asn1Seq<Asn1X509NameElement>>,
}

#[asn1_sequence()]
struct Asn1X509Name {
	pub entries : Asn1X509NameEntry,
}


fn x509namedec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = Asn1X509Name::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("xname",0,&mut f)?;
	}

	Ok(())
}


#[extargs_map_function(x509namedec_handler)]
pub fn load_asn1_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"x509namedec<x509namedec_handler>##derfile ... to decode file##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}