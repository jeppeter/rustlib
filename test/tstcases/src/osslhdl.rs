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
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

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
#[allow(unused_imports)]
use super::fileop::{read_file_bytes,write_file_bytes};

use super::ossllib::{SpcString,SpcSerializedObject,SpcLink,SpcSpOpusInfo,SpcAttributeTypeAndOptionalValue,AlgorithmIdentifier,DigestInfo,SpcIndirectDataContent,CatalogAuthAttr,CatalogInfo};
use asn1obj::asn1impl::Asn1Op;

asn1obj_error_class!{OsslHdlError}


fn spcstringdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcString::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("spcstring",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcString");
	}

	Ok(())
}

fn spcserobjdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcSerializedObject::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcSerializedObject",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcSerializedObject");
	}

	Ok(())
}


fn spclinkdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcLink::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcLink",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcLink");
	}

	Ok(())
}

fn spcopusinfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcSpOpusInfo::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcSpOpusInfo",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcSpOpusInfo");
	}

	Ok(())
}

fn spcattrvaldec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcAttributeTypeAndOptionalValue::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcAttributeTypeAndOptionalValue",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcAttributeTypeAndOptionalValue");
	}

	Ok(())
}

fn algoridentdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = AlgorithmIdentifier::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("AlgorithmIdentifier",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode AlgorithmIdentifier");
	}

	Ok(())
}

fn diginfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = DigestInfo::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("DigestInfo",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode DigestInfo");
	}

	Ok(())
}

fn spcinddatacondec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcIndirectDataContent::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcIndirectDataContent",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcIndirectDataContent");
	}

	Ok(())
}

fn cataattrdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = CatalogAuthAttr::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("CatalogAuthAttr",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode CatalogAuthAttr");
	}

	Ok(())
}

fn catainfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = CatalogInfo::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("CatalogInfo",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode CatalogInfo");
	}

	Ok(())
}
#[extargs_map_function(spcstringdec_handler,spcserobjdec_handler,spclinkdec_handler,spcopusinfodec_handler,spcattrvaldec_handler,algoridentdec_handler,diginfodec_handler,spcinddatacondec_handler,cataattrdec_handler,catainfodec_handler)]
pub fn load_ossl_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"spcstringdec<spcstringdec_handler>##files ... to decode spcstring##" : {
			"$" : "+"
		},
		"spcserobjdec<spcserobjdec_handler>##binfile ... to decode SpcSerializedObject##" : {
			"$" : "+"
		},
		"spclinkdec<spclinkdec_handler>##binfile ... to decode SpcLink##" : {
			"$" : "+"
		},
		"spcopusinfodec<spcopusinfodec_handler>##binfile ... to decode SpcSpOpusInfo##" : {
			"$" : "+"
		},
		"spcattrvaldec<spcattrvaldec_handler>##binfile ... to decode SpcAttributeTypeAndOptionalValue##" : {
			"$" : "+"
		},
		"algoridentdec<algoridentdec_handler>##binfile ... to decode AlgorithmIdentifier##" : {
			"$" : "+"
		},
		"diginfodec<diginfodec_handler>##binfile ... to decode DigestInfo##" : {
			"$" : "+"
		},
		"spcinddatacondec<spcinddatacondec_handler>##binfile ... to decode SpcIndirectDataContent##" : {
			"$" : "+"
		},
		"cataattrdec<cataattrdec_handler>##binfile ... to decode CatalogAuthAttr##" : {
			"$" : "+"
		},
		"catainfodec<catainfodec_handler>##binfile ... to decode CatalogInfo##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
