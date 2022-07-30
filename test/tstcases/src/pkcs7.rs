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
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence};
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1OctString,Asn1PrintableString,Asn1BitString,Asn1Null};
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1ImpVec,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
use asn1obj::strop::{asn1_format_line};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
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
use std::io::{Write};

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};


use super::fileop::{read_file_bytes};

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509NameElement {
	pub obj : Asn1Object,
	pub name :Asn1PrintableString,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509NameEntry {
	pub names : Asn1Set<Asn1Seq<Asn1X509NameElement>>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Name {
	pub entries : Asn1Seq<Asn1X509NameEntry>,
}


#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Attribute {
	pub object :Asn1Object,
	pub set :Asn1Any,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509ValElem {
	pub notBefore : Asn1Time,
	pub notAfter : Asn1Time,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Val {
	pub elem : Asn1Seq<Asn1X509ValElem>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509AlgorElem {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Opt<Asn1Any>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Algor {
	pub elem : Asn1Seq<Asn1X509AlgorElem>,
}


#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7ContentElem {
	pub objval : Asn1Object,
	pub data :Asn1Any,	
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7Content {
	pub elem :Asn1Seq<Asn1Pkcs7ContentElem>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1RsaPubkey {
	pub n :Asn1BigNum,
	pub e :Asn1BigNum,
}

#[asn1_obj_selector(selector=val,any=default,rsa="1.2.840.113549.1.1.1",debug=enable)]
#[derive(Clone)]
struct Asn1X509PubkeySelector {
	pub val : Asn1Object,
	pub padded : Asn1Null,
}

#[asn1_choice(selector=valid,debug=enable)]
#[derive(Clone)]
struct Asn1X509PubkeyElem {
	pub valid : Asn1SeqSelector<Asn1X509PubkeySelector>,
	pub rsa : Asn1BitSeq<Asn1Seq<Asn1RsaPubkey>>,
	pub any : Asn1Any,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Pubkey {
	pub elem :Asn1Seq<Asn1X509PubkeyElem>,
}


#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509ExtensionElem {
	pub object :Asn1Object,
	pub critical : Asn1Opt<Asn1Boolean>,
	pub value : Asn1OctString,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Extension {
	pub elem :Asn1Seq<Asn1X509ExtensionElem>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509CinfElem {
	pub version : Asn1Opt<Asn1ImpVec<Asn1Integer,0>>,
	pub serial_number :Asn1BigNum,
	pub signature : Asn1X509Algor,
	pub issuer : Asn1X509Name,
	pub validity : Asn1X509Val,
	pub subject :Asn1X509Name,
	pub key : Asn1X509Pubkey,
	pub issuerUID : Asn1Opt<Asn1Imp<Asn1BitString,1>>,
	pub subjectUID : Asn1Opt<Asn1Imp<Asn1BitString,2>>,
	pub extensions : Asn1Opt<Asn1ImpSet<Asn1Seq<Asn1X509Extension>,3>>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Cinf {
	pub elem : Asn1Seq<Asn1X509CinfElem>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Revoked {
	pub serialNumber : Asn1Integer,
	pub revocationDate : Asn1Time,
	pub extensions : Asn1Opt<Asn1Seq<Asn1X509Extension>>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509CrlInfo {
	pub version : Asn1Opt<Asn1Integer>,
	pub sig_alg : Asn1X509Algor,
	pub issuer : Asn1X509Name,
	pub lastUpdate : Asn1Time,
	pub nextUpdate :Asn1Time,
	pub revoked : Asn1Opt<Asn1Seq<Asn1X509Revoked>>,
	pub extensions : Asn1Opt<Asn1Seq<Asn1X509Extension>>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Crl {
	pub crl : Asn1X509CrlInfo,
	pub sig_alg :Asn1X509Algor,
	pub signature : Asn1BitString,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509Elem {
	pub certinfo : Asn1X509Cinf,
	pub sig_alg : Asn1X509Algor,
	pub signature : Asn1BitString,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1X509 {
	pub elem : Asn1Seq<Asn1X509Elem>,
}


#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7IssuerAndSerial {
	pub issuer : Asn1X509Name,
	pub serial : Asn1Integer,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7SignerInfo {
	pub version : Asn1Integer,
	pub issuer_and_serial : Asn1Pkcs7IssuerAndSerial,
	pub digest_algo : Asn1X509Algor,
	pub auth_attr : Asn1Opt<Asn1ImpVec<Asn1X509Attribute,0>>,
	pub digest_enc_algo : Asn1X509Algor,
	pub enc_digest : Asn1OctString,
	pub unauth_attr : Asn1Opt<Asn1ImpVec<Asn1X509Attribute,1>>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7SignedElem {
	pub version :Asn1Integer,
	pub md_algs : Asn1Set<Asn1X509Algor>,
	pub contents : Asn1Pkcs7Content,
	pub cert :Asn1Opt<Asn1ImpSet<Asn1X509,0>>,
	pub signer_info : Asn1Set<Asn1Pkcs7SignerInfo>,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7Signed {
	pub elem : Asn1Seq<Asn1Pkcs7SignedElem>,
}


#[asn1_obj_selector(anyobj=default,signed="1.2.840.113549.1.7.2")]
#[derive(Clone)]
struct Asn1Pkcs7Selector {
	pub val :Asn1Object,
}

#[asn1_choice(selector=selector,debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7Elem {
	pub selector :Asn1Pkcs7Selector,
	pub signed : Asn1Ndef<Asn1Pkcs7Signed,0>,
	pub anyobj :Asn1Any,
}

#[asn1_sequence(debug=enable)]
#[derive(Clone)]
struct Asn1Pkcs7 {
	pub elem :Asn1Seq<Asn1Pkcs7Elem>,
}





fn pkcs7dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let code = read_file_bytes(f)?;
		let mut xname = Asn1Pkcs7::init_asn1();
		let _ = xname.decode_asn1(&code)?;
		let mut f = std::io::stderr();
		xname.print_asn1("xname",0,&mut f)?;
	}

	Ok(())
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


fn objenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut objd :Asn1Object = Asn1Object::init_asn1();

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let _ = objd.set_value(f)?;
		let vcode = objd.encode_asn1()?;
		debug_buffer_trace!(vcode.as_ptr(), vcode.len(), "encode {} object", f);
	}

	Ok(())
}


#[extargs_map_function(pkcs7dec_handler,x509namedec_handler,objenc_handler)]
pub fn load_pkcs7_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"pkcs7dec<pkcs7dec_handler>##derfile ... to decode file##" : {
			"$" : "+"
		},
		"x509namedec<x509namedec_handler>##derfile ... to decode file##" : {
			"$" : "+"
		},
		"objenc<objenc_handler>##objid ... to encode object##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}