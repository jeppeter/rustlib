use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};

#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
#[allow(unused_imports)]
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1PrintableString,Asn1BitString,Asn1Null,Asn1OctData,Asn1BitData,Asn1IA5String,Asn1BitDataFlag};
#[allow(unused_imports)]
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};
use asn1obj::asn1impl::Asn1Op;
#[allow(unused_imports)]
use asn1obj::strop::{asn1_format_line};

#[allow(unused_imports)]
use std::io::{Write};



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

//use super::{debug_trace,debug_warn,debug_error,debug_info,debug_debug,format_str_log};
//use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::loglib::{init_log};
use serde::{Deserialize, Serialize};
use super::fileop::{read_file};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
	name: String,
	age: u8,
}

fn serdeperson_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;


	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let s = read_file(f)?;
		let p :Person = serde_json::from_str(&s)?;

		//let p :Person = ores.unwrap();
		println!("{}\n{:?}",f, p);
		let j :String = serde_json::to_string(&p)?;
		println!("to_string\n{}", j);
	}
	Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct FlatternInfo {
	addr :String,
	
	#[serde(flatten)]
	pers :Person,
}


fn serdeflattern_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;


	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let s = read_file(f)?;
		let p :FlatternInfo = serde_json::from_str(&s)?;

		//let p :Person = ores.unwrap();
		println!("{}\n{:?}",f, p);
		let j :String = serde_json::to_string(&p)?;
		println!("to_string\n{}", j);
	}
	Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
struct NVersion {
	oid :String,
	#[serde(serialize_with="data_serialize",deserialize_with="data_deserialize")]
	data :Vec<u8>,
}


use serde::ser::{SerializeSeq,SerializeStruct};

fn data_serialize<S>(data :&Vec<u8>,serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	let mut seq = serializer.serialize_seq(Some(data.len()))?;
	for v in data.iter() {
		seq.serialize_element(v)?;
	}
	seq.end()
}


#[allow(dead_code)]
struct FlatternVecVisitor(Vec<u8>);

impl<'de> serde::de::Visitor<'de> for FlatternVecVisitor {
	type Value = Vec<u8>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "an array of arrays")
	}



	fn visit_seq<A>(self, mut seq: A) -> Result<Vec<u8>, A::Error>
	where A: serde::de::SeqAccess<'de>,
	{
		let mut vec :Vec<u8>= Vec::new();

		while let Some(v) = seq.next_element()? {
			vec.push(v);
		}

		Ok(vec)
	}
}



fn data_deserialize<'de, D>(deserializer :D) -> Result<Vec<u8>, D::Error> 
where D: serde::de::Deserializer<'de> {

	let visitor :FlatternVecVisitor = FlatternVecVisitor(vec![]);
	let val = deserializer.deserialize_seq(visitor)?;
	Ok(val)
}


#[asn1_sequence()]
#[derive(Clone)]
#[derive(Serialize,Deserialize)]
pub struct Asn1X509NameAnyElement {
	#[serde(serialize_with = "obj_serialize", deserialize_with = "obj_deserialize")]
	pub obj :Asn1Object,
	#[serde(serialize_with = "value_serialize" , deserialize_with = "value_deserialize")]
	pub value :Asn1Any,
}

fn obj_serialize<S>(obj :&Asn1Object,serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	serializer.serialize_str(&obj.get_value())
}

#[allow(dead_code)]
struct StringVisitor(String);

impl<'de> serde::de::Visitor<'de> for StringVisitor {
	type Value = String;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "an string")
	}



	fn visit_str<E>(self, v :&str) -> Result<Self::Value,E>
	where E :Error
	{
		Ok(format!("{}",v))
	}
}




fn obj_deserialize<'de, D>(deserializer :D) -> Result<Asn1Object,D::Error> 
where D: serde::de::Deserializer<'de> {
	let vs :StringVisitor = StringVisitor("".to_string());
	let s = format!("{}",deserializer.deserialize_str(vs)?);
	let mut obj :Asn1Object = Asn1Object::init_asn1();
	let ores = obj.set_value(&s);
	if ores.is_err() {
		let e  : D::Error =  serde::de::Error::custom( ores.err().unwrap().to_string());
		return Err(e);
	}
	Ok(obj)
}

fn value_serialize<S>(oany :&Asn1Any,serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	let mut map = serializer.serialize_struct("Asn1Any",2)?;
	map.serialize_field("tag",&oany.tag)?;
	map.serialize_field("data",&oany.content)?;
	map.end()
}

#[allow(dead_code)]
struct Asn1AnyVisitor(Asn1Any);

impl<'de> serde::de::Visitor<'de> for Asn1AnyVisitor {
	type Value = Asn1Any;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "a map need")
	}



	fn visit_map<A>(self, mut mapv: A) -> Result<Asn1Any, A::Error>
	where A: serde::de::MapAccess<'de>,
	{
		let mut oany :Asn1Any = Asn1Any::init_asn1();
		let mut tagv :Option<u64> = None;
		let mut contentv :Option<Vec<u8>> = None;

		while let Some(key) = mapv.next_key::<String>()? {
			match key.as_str() {
				"tag" => {

					if tagv.is_some() {
						return Err(serde::de::Error::duplicate_field("tag"));
					}
					tagv = Some(mapv.next_value::<u64>()?);
				},
				"content" => {
					if contentv.is_some() {
						return Err(serde::de::Error::duplicate_field("content"));
					}
					contentv = Some(mapv.next_value::<Vec<u8>>()?);
				},
				_ => {

				},
			}
		}

		if tagv.is_some() {
			oany.tag = tagv.as_ref().unwrap().clone();
		}

		if contentv.is_some() {
			oany.content = contentv.as_ref().unwrap().clone();
		}

		Ok(oany)
	}
}



fn value_deserialize<'de, D>(deserializer :D) -> Result<Asn1Any, D::Error> 
where D: serde::de::Deserializer<'de> {
	let visitor :Asn1AnyVisitor = Asn1AnyVisitor(Asn1Any::init_asn1());
	deserializer.deserialize_map(visitor)
}


#[derive(Clone,Serialize,Deserialize)]
struct PkixNameFake {
	#[serde(default = "array_default")]
	pub country :Vec<String>,
	#[serde(serialize_with="extra_serialize", deserialize_with="extra_deserialize" ,default="extra_default")]
	pub extra_names :Vec<Asn1X509NameAnyElement>,
}

fn array_default() -> Vec<String> {
	vec![]
}

fn extra_default() -> Vec<Asn1X509NameAnyElement> {
	vec![]
}

macro_rules! expand_pkix_fmt {
	($name :expr, $elem :expr, $f :expr) => {
		let mut _idx :usize = 0;
		$f.write_fmt(format_args!("{} : [",$name))?;
		while _idx < $elem.len() {
			if _idx > 0 {
				$f.write_fmt(format_args!(","))?;
			}
			$f.write_fmt(format_args!("\"{}\"",$elem[_idx]))?;
			_idx += 1;
		}
		$f.write_fmt(format_args!("]"))?;
	};
}


macro_rules! expand_pkix_fmt_extra {
	($name :expr, $elem :expr, $f :expr) => {
		let mut _idx :usize = 0;
		let mut _jdx :usize;
		$f.write_fmt(format_args!("{} :[",$name))?;
		while _idx < $elem.len() {
			if _idx > 0 {
				$f.write_fmt(format_args!(","))?;
			}
			$f.write_fmt(format_args!("{{ obj :\"{}\" ,tag: {}, content[",$elem[_idx].obj.get_value(),$elem[_idx].value.tag))?;
			_jdx = 0;
			while _jdx < $elem[_idx].value.content.len() {
				if _jdx > 0 {
					$f.write_fmt(format_args!(","))?;
				}
				$f.write_fmt(format_args!("{}",$elem[_idx].value.content[_jdx]))?;
				_jdx += 1;
			}
			$f.write_fmt(format_args!("]}}"))?;
			_idx += 1;
		}
		$f.write_fmt(format_args!("]"))?;		
	};
}


impl std::fmt::Debug for PkixNameFake {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("PkixName{{"))?;
		expand_pkix_fmt!("country",self.country,f);
		f.write_fmt(format_args!(","))?;
		expand_pkix_fmt_extra!("extra_names",self.extra_names,f);
		f.write_fmt(format_args!("}}"))
	}
}

fn extra_serialize<S>(oany :&Vec<Asn1X509NameAnyElement>,serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	let mut seq = serializer.serialize_seq(Some(oany.len()))?;
	for v in oany.iter() {
		seq.serialize_element(v)?;
	}
	seq.end()
}

#[allow(dead_code)]
struct Asn1X509NameAnyElementSeq(Vec<Asn1X509NameAnyElement>);

impl<'de> serde::de::Visitor<'de> for Asn1X509NameAnyElementSeq {
	type Value = Vec<Asn1X509NameAnyElement>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "an array need")
	}



	fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Asn1X509NameAnyElement>, A::Error>
	where A: serde::de::SeqAccess<'de>,
	{
		let mut vec :Vec<Asn1X509NameAnyElement>= Vec::new();

		while let Some(v) = seq.next_element::<Asn1X509NameAnyElement>()? {
			vec.push(v);
		}
		Ok(vec)

	}
}



fn extra_deserialize<'de, D>(deserializer :D) -> Result<Vec<Asn1X509NameAnyElement>, D::Error> 
where D: serde::de::Deserializer<'de> {
	let visitor :Asn1X509NameAnyElementSeq = Asn1X509NameAnyElementSeq(vec![]);
	deserializer.deserialize_seq(visitor)
}

#[derive(Debug)]
#[derive(Clone,Serialize,Deserialize)]
pub enum SignatureAlgorithm {
	UnknownSignatureAlgorithm,
	MD2WithRSA,
	MD5WithRSA,
	SHA1WithRSA,
	SHA256WithRSA,
	SHA384WithRSA,
	SHA512WithRSA,
	DSAWithSHA1,
	DSAWithSHA256,
	ECDSAWithSHA1,
	ECDSAWithSHA256,
	ECDSAWithSHA384,
	ECDSAWithSHA512,
	SHA256WithRSAPSS,
	SHA384WithRSAPSS,
	SHA512WithRSAPSS,
	PureEd25519,
}

#[derive(Clone)]
#[derive(Debug,Serialize,Deserialize)]
pub enum ExtKeyUsage {
    ExtKeyUsageAny,
    ExtKeyUsageServerAuth,
    ExtKeyUsageClientAuth,
    ExtKeyUsageCodeSigning,
    ExtKeyUsageEmailProtection,
    ExtKeyUsageIPSECEndSystem,
    ExtKeyUsageIPSECTunnel,
    ExtKeyUsageIPSECUser,
    ExtKeyUsageTimeStamping,
    ExtKeyUsageOCSPSigning,
    ExtKeyUsageMicrosoftServerGatedCrypto,
    ExtKeyUsageNetscapeServerGatedCrypto,
    ExtKeyUsageMicrosoftCommercialCodeSigning,
    ExtKeyUsageMicrosoftKernelCodeSigning,
}

#[derive(Debug)]
#[derive(Clone,Serialize,Deserialize)]
pub enum KeyUsage {
	KeyUsageDigitalSignature,
	KeyUsageContentCommitment,
	KeyUsageKeyEncipherment,
	KeyUsageDataEncipherment,
	KeyUsageKeyAgreement,
	KeyUsageCertSign,
	KeyUsageCRLSign,
	KeyUsageEncipherOnly,
	KeyUsageDecipherOnly,
}




#[derive(Debug)]
#[derive(Clone,Serialize,Deserialize)]
pub struct X509BuildConfig {
	#[serde(default = "serial_number_default")]
	pub serial_number  :BigInt,
	#[serde(default = "signature_algorithm_default")]
	pub signature_algorithm :SignatureAlgorithm,
	#[serde(default = "pkix_name_default")]
	pub issuer :PkixNameFake,
	#[serde(default = "data_time_default")]
	pub not_before :DateTime<Utc>,
	#[serde(default = "vec_key_usage_default")]
	pub key_usage :Vec<KeyUsage>,
	#[serde(default = "vec_ext_key_usage_default")]
	pub ext_key_usage :Vec<ExtKeyUsage>,
}

fn serial_number_default() -> BigInt {
	BigInt::zero()
}

fn signature_algorithm_default() -> SignatureAlgorithm {
	SignatureAlgorithm::UnknownSignatureAlgorithm
}

fn pkix_name_default() -> PkixNameFake {
	PkixNameFake {
		country : vec![],
		extra_names : vec![],
	}
}

fn data_time_default() -> DateTime<Utc> {
	Utc::now()
}


fn vec_key_usage_default() -> Vec<KeyUsage> {
	vec![]
}

fn vec_ext_key_usage_default() -> Vec<ExtKeyUsage> {
	vec![]
}




fn implserde_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	let mut sout = std::io::stdout();
	for f in sarr.iter() {
		let s = read_file(f)?;
		let p :X509BuildConfig = serde_json::from_str(&s)?;

		//let p :Person = ores.unwrap();
		println!("{}\n{:?}",f, p);
		let j :String = serde_json::to_string(&p)?;
		println!("to_string\n{}", j);
	}
	Ok(())
}



#[extargs_map_function(serdeperson_handler,serdeflattern_handler,implserde_handler)]
pub fn load_serde_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"serdeperson<serdeperson_handler>##inputname ...##" : {
			"$" : "*"
		},
		"serdeflattern<serdeflattern_handler>##inputname ...##" : {
			"$" : "+"
		},
		"implserde<implserde_handler>##file ... to serialize and deserialize##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}