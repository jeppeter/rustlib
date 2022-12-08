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
use asn1obj::base::*;
use asn1obj::consts::*;

use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use std::io::{Write};

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::fileop::{read_file_bytes,write_file_bytes,read_file,get_sha256_data};

use super::ossllib::*;
use super::asn1def::*;
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

fn msctlcondec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = MsCtlContent::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("MsCtlContent",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode MsCtlContent");
	}

	Ok(())
}

fn spcpeimagedatadec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcPeImageData::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcPeImageData",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcPeImageData");
	}

	Ok(())
}

fn spcsipinfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcSipInfo::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("SpcSipInfo",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcSipInfo");
	}

	Ok(())
}

fn msgimpprintdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = MessageImprint::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("MessageImprint",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode MessageImprint");
	}

	Ok(())
}

fn timestamprqstblobdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = TimeStampRequestBlob::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampRequestBlob",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampRequestBlob");
	}

	Ok(())
}

fn timestamprqstdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = TimeStampRequest::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampRequest",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampRequest");
	}

	Ok(())
}

fn pkistatusinfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = PKIStatusInfo::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("PKIStatusInfo",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode PKIStatusInfo");
	}

	Ok(())
}

fn timestamprespdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut cv :serde_json::value::Value;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = TimeStampResp::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampResp",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		cv = serde_json::json!({});
		let _ = xname.encode_json("",&mut cv)?;
		let s = serde_json::to_string_pretty(&cv)?;
		let _ = f.write(s.as_bytes())?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampResp");
	}

	Ok(())
}

fn timestamprespenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut cv :serde_json::value::Value;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let jcode = read_file(f)?;
		cv = serde_json::from_str(&jcode)?;
		let mut xname = TimeStampResp::init_asn1();
		let _ = xname.decode_json("",&cv)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampResp",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampResp");
	}

	Ok(())
}


fn timestampreqdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = TimeStampReq::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampReq",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampReq");
	}

	Ok(())
}

fn timestampaccdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = TimeStampAccuracy::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("TimeStampAccuracy",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode TimeStampAccuracy");
	}

	Ok(())
}

fn spcasn1codedec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut cv :serde_json::value::Value;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = SpcAsn1Code::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		cv = serde_json::json!({});
		xname.print_asn1("SpcAsn1Code",0,&mut f)?;
		let vcode = xname.encode_asn1()?;
		let _ = xname.encode_json("",&mut cv)?;
		let s = serde_json::to_string_pretty(&cv)?;
		let _ = f.write(s.as_bytes())?;
		debug_buffer_trace!(vcode.as_ptr(),vcode.len(),"encode SpcAsn1Code");
	}

	Ok(())
}

fn removeselfcert_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut p7 :Asn1Pkcs7 = Asn1Pkcs7::init_asn1();
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		asn1obj_new_error!{OsslHdlError,"need at least one argument"}
	}
	let data = read_file_bytes(&sarr[0])?;
	let _ = p7.decode_asn1(&data)?;
	let p7signed :&mut Asn1Pkcs7Signed = p7.get_signed_data_mut()?;
	let ocerts :Vec<Asn1X509> = p7signed.get_certs()?;
	let mut ncerts :Vec<Asn1X509> = Vec::new();
	let mut scerts :Vec<Asn1X509> = Vec::new();
	let mut f = std::io::stderr();
	let mut i :i32=0;
	for k in ocerts.iter() {
		if k.is_self_signed() {
			scerts.push(k.clone());
			continue;
		}
		ncerts.push(k.clone());
		i += 1;
	}
	let _ = p7signed.set_certs(&ncerts)?;
	p7.print_asn1("Pkcs7",0,&mut f)?;
	let code = p7.encode_asn1()?;
	if sarr.len() > 1 {
		let _ = write_file_bytes(&sarr[1],&code)?;
	} else {
		debug_buffer_trace!(code.as_ptr(),code.len(),"out buffer");
	}

	if sarr.len() > 2 {
		i = 0;
		for k in scerts.iter() {
			let code = k.encode_asn1()?;
			let fname = format!("{}.{}",sarr[2],i);
			let _ = write_file_bytes(&fname,&code)?;
			i += 1;
		}
	}

	Ok(())
}


fn digestset_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut p7 :Asn1Pkcs7 = Asn1Pkcs7::init_asn1();
	let passin :String = ns.get_string("passin");
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		asn1obj_new_error!{OsslHdlError,"need at least pkcs7.bin infile privpem"}
	}
	let data = read_file_bytes(&sarr[0])?;
	let _ = p7.decode_asn1(&data)?;
	let p7signed :&mut Asn1Pkcs7Signed = p7.get_signed_data_mut()?;
	let data = read_file_bytes(&sarr[1])?;
	let shadigest = get_sha256_data(&data);
	let mut idx :usize;
	let mut setobj :Asn1Object = Asn1Object::init_asn1();
	let _ = setobj.set_value(OID_SHA256_DIGEST_SET)?;
	let mut setany :Asn1Any = Asn1Any::init_asn1();
	let mut setdata :Vec<u8> = Vec::new();
	let privkey = get_private_key_file(&sarr[2],passin.as_bytes())?;

	setdata.push(ASN1_OCT_STRING_FLAG);
	setdata.push(shadigest.len() as u8);
	for v in shadigest.iter() {
		setdata.push(*v);
	}

	setany.tag = 0x31;
	setany.content = setdata;

	assert!(p7signed.elem.val.len() == 1);

	idx = 0;
	loop {
		let ov :Option<&mut Asn1Pkcs7SignerInfo> = p7signed.get_signer_info_mut(idx);
		if ov.is_none() {
			break;
		}

		let si :&mut Asn1Pkcs7SignerInfo = ov.unwrap();

		let mut cattrs :Vec<Asn1X509Attribute> = si.get_auth_attrs()?;
		for i in 0..cattrs.len() {
			let _ = cattrs[i].check_set_object_val(&setobj,&setany)?;
		}

		let bval = si.set_auth_attrs(&cattrs)?;
		let _ = si.sign_auth_attr_enc(&privkey)?;

		idx += 1;
	}


	let data = p7.encode_asn1()?;


	if sarr.len() > 3 {
		let _ = write_file_bytes(&sarr[3],&data)?;
	} else {
		debug_buffer_trace!(data.as_ptr(),data.len(),"dump new data");
	}


	Ok(())
}

#[extargs_map_function(spcstringdec_handler,spcserobjdec_handler,spclinkdec_handler,spcopusinfodec_handler,spcattrvaldec_handler,algoridentdec_handler,diginfodec_handler,spcinddatacondec_handler,cataattrdec_handler,catainfodec_handler,msctlcondec_handler,spcpeimagedatadec_handler,spcsipinfodec_handler,msgimpprintdec_handler,timestamprqstblobdec_handler,timestamprqstdec_handler,pkistatusinfodec_handler,timestamprespdec_handler,timestampreqdec_handler,timestampaccdec_handler,spcasn1codedec_handler,timestamprespenc_handler,removeselfcert_handler,digestset_handler)]
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
		},
		"msctlcondec<msctlcondec_handler>##binfile ... to decode MsCtlContent##" : {
			"$" : "+"
		},
		"spcpeimagedatadec<spcpeimagedatadec_handler>##binfile ... to decode SpcPeImageData##" : {
			"$" : "+"
		},
		"spcsipinfodec<spcsipinfodec_handler>##binfile ... to decode SpcSipInfo##" : {
			"$" : "+"
		},
		"msgimpprintdec<msgimpprintdec_handler>##binfile ... to decode MessageImprint##" : {
			"$" : "+"
		},
		"timestamprqstblobdec<timestamprqstblobdec_handler>##binfile ... to decode TimeStampRequestBlob##" : {
			"$" : "+"
		},
		"timestamprqstdec<timestamprqstdec_handler>##binfile ... to decode TimeStampRequest##" : {
			"$" : "+"
		},
		"pkistatusinfodec<pkistatusinfodec_handler>##binfile ... to decode PKIStatusInfo##" : {
			"$" : "+"
		},
		"timestamprespdec<timestamprespdec_handler>##binfile ... to decode TimeStampResp##" : {
			"$" : "+"
		},
		"timestampreqdec<timestampreqdec_handler>##binfile ... to decode TimeStampReq##" : {
			"$" : "+"
		},
		"timestampaccdec<timestampaccdec_handler>##binfile ... to decode TimeStampAccuracy##" : {
			"$" : "+"
		},
		"spcasn1codedec<spcasn1codedec_handler>##binfile ... to decode SpcAsn1Code##" : {
			"$" : "+"
		},
		"timestamprespenc<timestamprespenc_handler>##jsonfile ... to encode TimeStampResp##" : {
			"$" : "+"
		},
		"removeselfcert<removeselfcert_handler>##pkcs7.bin [out.bin] [selfsigncert.bin] to remove self cert##" : {
			"$" : "+"
		},
		"pk7digestset<digestset_handler>##pkcs7.bin infile privpem [out.bin] to change digest for infile ##" : {
			"$" : "+"
		}
	}
	"#;
	/**/
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
