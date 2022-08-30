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
use super::fileop::{read_file_bytes,write_file_bytes};

use super::cryptlib::{aes256_cbc_encrypt,aes256_cbc_decrypt,aes128_encrypt,aes128_decrypt,aes192_encrypt,aes192_decrypt,aes256_encrypt,aes256_decrypt,aes256_cbc_pure_encrypt,aes256_cbc_pure_decrypt,aes256_cfb_decrypt,aes256_cfb_encrypt,opengpg_s2k_sha512};
use super::strop::{parse_u64};

use hex::{FromHex};
use hex;

asn1obj_error_class!{CryptHdlError}

fn aescbcenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let datav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let encdata :Vec<u8> = aes256_cbc_encrypt(&datav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(datav8.as_ptr(),datav8.len(),"read [{}]", f);
	debug_buffer_trace!(encdata.as_ptr(),encdata.len(), "enc data with key [{}] iv[{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&encdata)?;
	Ok(())
}

fn aescbcdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let encdatav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let data :Vec<u8> = aes256_cbc_decrypt(&encdatav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(encdatav8.as_ptr(),encdatav8.len(),"read [{}]",f);
	debug_buffer_trace!(data.as_ptr(),data.len(), "decrypt with [{}] [{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&data)?;
	Ok(())
}


fn aescbcpureenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let datav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let encdata :Vec<u8> = aes256_cbc_pure_encrypt(&datav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(datav8.as_ptr(),datav8.len(),"read [{}]", f);
	debug_buffer_trace!(encdata.as_ptr(),encdata.len(), "enc data with key [{}] iv[{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&encdata)?;
	Ok(())
}

fn aescbcpuredec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let encdatav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let data :Vec<u8> = aes256_cbc_pure_decrypt(&encdatav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(encdatav8.as_ptr(),encdatav8.len(),"read [{}]",f);
	debug_buffer_trace!(data.as_ptr(),data.len(), "decrypt with [{}] [{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&data)?;
	Ok(())
}


fn aesencbase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key data"}
	}
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let datav8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let encdatav8 :Vec<u8>;
	if keyv8.len() == 16 {
		encdatav8 = aes128_encrypt(&datav8,&keyv8)?;
	} else if keyv8.len() == 24 {
		encdatav8 = aes192_encrypt(&datav8,&keyv8)?;
	} else if keyv8.len() == 32 {
		encdatav8 = aes256_encrypt(&datav8,&keyv8)?;
	} else {
		extargs_new_error!{CryptHdlError,"key len [{}] not valid", keyv8.len()}
	}
	debug_buffer_trace!(keyv8.as_ptr(), keyv8.len(),"key");
	debug_buffer_trace!(datav8.as_ptr(),datav8.len(),"data");
	debug_buffer_trace!(encdatav8.as_ptr(),encdatav8.len(), "encrypt data");
	Ok(())
}

fn aesdecbase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key data"}
	}
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let encdatav8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let datav8 :Vec<u8>;
	if keyv8.len() == 16 {
		datav8 = aes128_decrypt(&encdatav8,&keyv8)?;
	} else if keyv8.len() == 24 {
		datav8 = aes192_decrypt(&encdatav8,&keyv8)?;
	} else if keyv8.len() == 32 {
		datav8 = aes256_decrypt(&encdatav8,&keyv8)?;
	} else {
		extargs_new_error!{CryptHdlError,"key len [{}] not valid", keyv8.len()}
	}
	debug_buffer_trace!(keyv8.as_ptr(), keyv8.len(),"key");
	debug_buffer_trace!(encdatav8.as_ptr(),encdatav8.len(),"encdata");
	debug_buffer_trace!(datav8.as_ptr(),datav8.len(), "decrypt data");
	Ok(())
}

fn aescfbenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let datav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let encdata :Vec<u8> = aes256_cfb_encrypt(&datav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(datav8.as_ptr(),datav8.len(),"read [{}]", f);
	debug_buffer_trace!(encdata.as_ptr(),encdata.len(), "enc data with key [{}] iv[{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&encdata)?;
	Ok(())
}

fn aescfbdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let encdatav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let data :Vec<u8> = aes256_cfb_decrypt(&encdatav8,&keyv8,&ivv8)?;
	debug_buffer_trace!(encdatav8.as_ptr(),encdatav8.len(),"read [{}]",f);
	debug_buffer_trace!(data.as_ptr(),data.len(), "decrypt with [{}] [{}]",sarr[0],sarr[1]);
	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&data)?;
	Ok(())
}


fn aescfbmutlenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let datav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let mut pdata :Vec<u8> = Vec::new();
	for i in 0..datav8.len() {
		pdata.push(datav8[i]);
	}
	for i in 2..sarr.len() {
		let f = format!("{}",sarr[i]);
		let datav8 = read_file_bytes(&f)?;
		for i in 0..datav8.len() {
			pdata.push(datav8[i]);
		}
	}

	let encdata = aes256_cfb_encrypt(&pdata,&keyv8,&ivv8)?;

	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&encdata)?;
	Ok(())
}

fn aescfbmutldec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{CryptHdlError,"need key iv"}
	}
	let f = ns.get_string("input");
	let datav8 :Vec<u8> = read_file_bytes(&f)?;
	let keyv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
	let ivv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let mut encdata :Vec<u8> = Vec::new();
	for i in 0..datav8.len() {
		encdata.push(datav8[i]);
	}
	for i in 2..sarr.len() {
		let f = format!("{}",sarr[i]);
		let datav8 = read_file_bytes(&f)?;
		for i in 0..datav8.len() {
			encdata.push(datav8[i]);
		}
	}
	let decdata = aes256_cfb_decrypt(&encdata,&keyv8,&ivv8)?;

	let outf = ns.get_string("output");
	let _ = write_file_bytes(&outf,&decdata)?;
	Ok(())
}

fn gpgkdfs2k512_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() != 3 {
		asn1obj_new_error!{CryptHdlError,"need pass salt iterations"}
	}
	let passin = format!("{}",sarr[0]);
	let saltv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let iterations :u64 = parse_u64(&sarr[2])?;
	let outv = opengpg_s2k_sha512(passin.as_bytes(),&saltv8,iterations as usize,32)?;
	debug_buffer_trace!(outv.as_ptr(),outv.len(),"output kdf");
	Ok(())
}


#[extargs_map_function(aescbcenc_handler,aescbcdec_handler,aesencbase_handler,aesdecbase_handler,aescbcpureenc_handler,aescbcpuredec_handler,aescfbenc_handler,aescfbdec_handler,aescfbmutlenc_handler,aescfbmutldec_handler,gpgkdfs2k512_handler)]
pub fn load_crypto_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"aescbcpureenc<aescbcpureenc_handler>##key iv to encrypt data from input encdata to output##" : {
			"$" : 2
		},
		"aescbcpuredec<aescbcpuredec_handler>##key iv to encrypt data from input encdata to output##" : {
			"$" : 2
		},
		"aescbcenc<aescbcenc_handler>##key iv to encrypt data from input encdata to output##" : {
			"$" : 2
		},
		"aescbcdec<aescbcdec_handler>##key iv to decrypt encdata from input data to output##" : {
			"$" : 2
		},
		"aesencbase<aesencbase_handler>##key data [size] to encrypt aes##" : {
			"$" : "+"
		},
		"aesdecbase<aesdecbase_handler>##key encdata [size] to decrypt aes##" : {
			"$" : "+"
		},
		"aescfbenc<aescfbenc_handler>##key encdata [size] to decrypt aes##" : {
			"$" : "+"
		},
		"aescfbdec<aescfbdec_handler>##key encdata [size] to decrypt aes##" : {
			"$" : "+"
		},
		"aescfbmultenc<aescfbmutlenc_handler>##key iv [file] to encrypt aes cfb##" : {
			"$" : "+"
		},
		"aescfbmultdec<aescfbmutldec_handler>##key iv [file] to decrypt aes cfb##" : {
			"$" : "+"
		},
		"gpgkdfs2k512<gpgkdfs2k512_handler>##pass salt iterations to encrypt##" : {
			"$" : 3
		}

	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}