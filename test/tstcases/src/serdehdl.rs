use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
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


use serde::ser::{SerializeSeq};

fn data_serialize<S>(data :&Vec<u8>,serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	let mut seq = serializer.serialize_seq(Some(data.len()))?;
	for v in data.iter() {
		seq.serialize_element(v)?;
	}
	seq.end()
}

struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

impl<'de, 'a, T> serde::de::DeserializeSeed<'de> for ExtendVec<'a, T>
where
T: serde::de::Deserialize<'de>,
{
     // The return type of the `deserialize` method. This implementation
     // appends onto an existing vector but does not create any new data
     // structure, so the return type is ().
     type Value = ();

     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
     where
     D: serde::de::Deserializer<'de>,
     {
         // Visitor implementation that will walk an inner array of the JSON
         // input.
         struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

         impl<'de, 'a, T> serde::de::Visitor<'de> for ExtendVecVisitor<'a, T>
         where
         T: serde::de::Deserialize<'de>,
         {
         	type Value = ();

         	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
         		write!(formatter, "an array of integers")
         	}

         	fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
         	where
         	A: serde::de::SeqAccess<'de>,
         	{
                 // Decrease the number of reallocations if there are many elements
                 if let Some(size_hint) = seq.size_hint() {
                 	self.0.reserve(size_hint);
                 }

                 // Visit each element in the inner array and push it onto
                 // the existing vector.
                 while let Some(elem) = seq.next_element()? {
                 	self.0.push(elem);
                 }
                 Ok(())
             }
         }

         deserializer.deserialize_seq(ExtendVecVisitor(self.0))
     }
 }



 struct FlattenedVecVisitor(Vec<u8>);

 impl<'de> serde::de::Visitor<'de> for FlattenedVecVisitor {
     // This Visitor constructs a single Vec<T> to hold the flattened
     // contents of the inner arrays.
     type Value = Vec<u8>;

     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
     	write!(formatter, "an array of arrays")
     }

     fn visit_seq<A>(self, mut seq: A) -> Result<Vec<u8>, A::Error>
     where
     A: serde::de::SeqAccess<'de>,
     {
         // Create a single Vec to hold the flattened contents.
         let mut vec :Vec<u8>= Vec::new();

         // Each iteration through this loop is one inner array.
         while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
         }

         // Return the finished vec.
         Ok(vec)
     }
 }


 fn data_deserialize<'de, D>(deserializer :D) -> Result<Vec<u8>, D::Error> 
 where D: serde::de::Deserializer<'de> {
 	let visitor = FlattenedVecVisitor(vec![]);
 	let val = deserializer.deserialize_any(visitor)?;
 	Ok(val)
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