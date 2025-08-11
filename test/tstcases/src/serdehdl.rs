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


//#[asn1_sequence()]
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

struct StringVisitor(String);

impl<'de> serde::de::Visitor<'de> for StringVisitor {
	type Value = String;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "an string")
	}



	fn visit_string<E>(self, v :String) -> Result<Self::Value,E>
	where E :Error
	{
		Ok(v)
	}
}

#[derive(Debug,Clone)]
pub struct SerdeError {
	msg :String,		
}

#[allow(dead_code)]
impl SerdeError {
	fn create(c :&str) -> SerdeError {
		SerdeError {msg : format!("{}",c)}
	}
}

impl std::fmt::Display for SerdeError {
	fn fmt(&self,f :&mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,"{}",self.msg)
	}
}

impl std::error::Error for SerdeError {}



fn obj_deserialize<'de, D>(deserializer :D) -> Result<Asn1Object,D::Error> 
where D: serde::de::Deserializer<'de> {
	let vs :StringVisitor = StringVisitor("".to_string());
	let s = format!("{}",deserializer.deserialize_string(vs)?);
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

fn value_deserialize<'de, D>(deserializer :D) -> Result<Asn1Any, D::Error> 
where D: serde::de::Deserializer<'de> {
	let retv :Asn1Any = Asn1Any::init_asn1();
	Ok(retv)
}



fn implserde_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let s = read_file(f)?;
		let p :NVersion = serde_json::from_str(&s)?;

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