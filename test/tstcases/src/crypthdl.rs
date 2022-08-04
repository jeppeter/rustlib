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

use super::aeslib::{aes256_cbc_encrypt,aes256_cbc_decrypt};

use hex::{FromHex};
use hex;

asn1obj_error_class!{CryptHdlError}

fn aescbcenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		asn1obj_new_error!{CryptHdlError,"need data key iv"}
	}
	let datav8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[2]).unwrap();
	let encdata :Vec<u8> = aes256_cbc_encrypt(&datav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(encdata.as_ptr(),encdata.len(), "enc data");
	println!("encrypt [{}] from key [{}] iv[{}]", sarr[0],sarr[1],sarr[2]);
	let s = format!("{}",hex::encode(&encdata));
	println!("{}",s);
	Ok(())
}

fn aescbcdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		asn1obj_new_error!{CryptHdlError,"need encdata key iv"}
	}
	let encdatav8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[2]).unwrap();
	let data :Vec<u8> = aes256_cbc_decrypt(&encdatav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(data.as_ptr(),data.len(), "enc data");
	println!("decrypt [{}] from key [{}] iv[{}]", sarr[0],sarr[1],sarr[2]);
	let s = format!("{}",hex::encode(&data));
	println!("{}",s);
	Ok(())
}


#[extargs_map_function(aescbcenc_handler,aescbcdec_handler)]
pub fn load_crypto_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"aescbcenc<aescbcenc_handler>##data key iv to encrypt##" : {
			"$" : 3
		},
		"aescbcdec<aescbcdec_handler>##encdata key iv to decrypt##" : {
			"$" : 3
		}

	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}