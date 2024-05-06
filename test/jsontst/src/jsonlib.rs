use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use serde_json::{json,Value};
use super::strop::{format_tabs};
use super::fileop::{read_file};
use super::logtrans::{init_log};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::sync::{Arc};
use std::cell::{RefCell};
use std::any::Any;


fn array_json(_key :&str, _instr :&str, tabs :i32) -> String {
	let mut retstr :String = String::from("");
	let vv :&Vec<Value>;
	let d :Value;
	//let mut cnt:usize;
	match serde_json::from_str(_instr) {
		Ok(v)  => {
			d = v;
			match d.as_array() {
				Some(dv) => { vv = dv;},
				None => {eprintln!("---------------\n{}+++++++++++++++++\nnot array",_instr); return retstr;}
			}
		}, 
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	retstr.push_str(&(format_tabs(tabs)[..]));
	if _key.len() > 0 {
		retstr.push_str(&(format!("\"{}\" : [",_key)[..]));
	} else {
		retstr.push_str("[");
	}

	for (i,v) in vv.iter().enumerate() {
		if i > 0 {
			retstr.push_str(",");
		}
		retstr.push_str("\n");
		if v.is_object() {
			retstr.push_str(&(enumerate_json("",&(v.to_string()[..]), tabs + 1)[..]));
			continue;
		}
		if v.is_array() {
			retstr.push_str(&(array_json("",&(v.to_string()[..]),tabs + 1)[..]));
			continue;
		}

		retstr.push_str(&(format_tabs(tabs + 1)[..]));
		retstr.push_str(&(format!("{}",v.to_string())[..]));
	}

	retstr.push_str("\n");
	retstr.push_str(&(format_tabs(tabs)[..]));
	retstr.push_str(&(format!("]")[..]));
	return retstr;
}


fn enumerate_json(_key :&str,_instr :&str,tabs :i32) -> String {
	let d :HashMap<String,Value>;
	let mut retstr :String = String::from("");
	let mut i:i32;
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	retstr.push_str(&(format_tabs(tabs)[..]));
	if _key.len() > 0 {
		retstr.push_str(&(format!("\"{}\" : {{",_key)[..]));		
	} else {
		retstr.push_str(&(format!("{{")[..]));
	}


	i = 0;
	for (s,v) in d {
		if i > 0 {
			retstr.push_str(",");
		}
		retstr.push_str("\n");
		if v.is_object() {
			retstr.push_str(&(enumerate_json(&(s[..]),&(v.to_string()[..]),tabs + 1)[..]));
			i += 1;
			continue;
		}

		if v.is_array() {
			retstr.push_str(&(array_json(&(s[..]),&(v.to_string()[..]),tabs+1)[..]));
			i += 1;
			continue;
		}

		retstr.push_str(&(format_tabs(tabs+1)[..]));
		retstr.push_str(&(format!("\"{}\" : {}",s,v)[..]));
		i += 1;
	}
	if i > 0 {
		retstr.push_str("\n");
	}
	retstr.push_str(&(format_tabs(tabs)[..]));
	retstr.push_str(&(format!("}}")[..]));
	return retstr;
}


fn add_json_value(_instr :&str, _key :&str, _val:&str) -> String {
	let mut d :HashMap<String,Value>;
	let c :Value;
	let mut retstr :String = String::from("");
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	match serde_json::from_str(_val) {
		Ok(v) => {c = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	d.insert(String::from(_key),c);
	retstr = enumerate_json("",&(format!("{}",json!(d).to_string())[..]),0);
	return retstr;
}

fn del_json_value(_instr :&str, _key :&str) -> String {
	let mut d :HashMap<String,Value>;
	let mut retstr :String = String::from("");
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}
	if d.contains_key(_key) {
		d.remove(_key);
	}
	retstr = enumerate_json("",&(format!("{}",json!(d).to_string())[..]),0);
	return retstr;
}

fn parse_json_value(_instr :&str) {
	let mut retstr :String;
	retstr = format!("[{}]",_instr);
	match serde_json::from_str(_instr) {
		Err(e) => {
			eprintln!("{} not valid string [{:?}]", _instr,e);
			return;
		},
		Ok(v) => {
			match v {
				Value::Null => {
					retstr.push_str(&(format!("null")[..]));
				},
				Value::Bool(bval) => {
					retstr.push_str(&(format!("bool({:?})",bval)[..]));
				},
				Value::Number(nval) => {
					retstr.push_str(&(format!("number({})",nval)[..]));
				},
				Value::String(sval) => {
					retstr.push_str(&(format!("string({})",sval)[..]));
				},
				Value::Array(aval) => {
					retstr.push_str(&(format!("array({:?})",aval)[..]));
				},
				Value::Object(oval) => {
					retstr.push_str(&(format!("object({:?})",oval)[..]));
				},
			}
		}
	}
	println!("{}", retstr);
	return;
}


extargs_error_class!{JsonLibError}

fn enumerate_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let s = read_file(f)?;
		println!("{}", enumerate_json("",&s,0));
	}

	Ok(())
}


fn add_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{JsonLibError,"need file key value"}
	}

	let s = read_file(&sarr[0])?;
	println!("add [{}] value [{}]\n{}",sarr[1],sarr[2],add_json_value(&s,&sarr[1],&sarr[2]));
	Ok(())
}

fn del_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		extargs_new_error!{JsonLibError,"need file keys..."}
	}

	let mut ss = read_file(&sarr[0])?;
	let mut idx :usize = 1;
	let mut retstr :String;
	while idx < sarr.len() {
		retstr = del_json_value(&ss, &sarr[idx]);
		println!("delete {}\n{}\n----------------\n{}\n++++++++++++++++++", sarr[idx], ss, retstr);
		ss = retstr;
		idx +=1;
	}
	Ok(())
}

fn parse_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{JsonLibError,"need json value"}
	}

	let mut idx :usize = 0;
	while idx < sarr.len() {
		parse_json_value(&sarr[idx]);
		idx += 1;
	}

	Ok(())
}

#[derive(Clone,Serialize,Deserialize)]
#[serde(untagged)]
pub enum DnsValue {
	Str(String),
	Arr(Vec<String>),
}

pub const NICSTATE_DHCP :&str = "dhcp";


#[derive(Clone,Deserialize,Serialize)]
pub struct ProtoNicStat {
	#[serde(default = "nic_stat_state_default")]
	pub state :String,
	#[serde(default = "nic_stat_ip_default")]
	pub ip :String,
	#[serde(default = "nic_stat_netmask_default")]
	pub netmask :String,
	#[serde(default = "nic_stat_gateway_default")]
	pub gateway :String,
	#[serde(default = "nic_stat_dns_default")]
	pub dns :DnsValue,
}

impl Default for ProtoNicStat {
	fn default() -> Self {
		Self {
			state : format!("{}",NICSTATE_DHCP),
			ip :format!(""),
			netmask :format!(""),
			gateway : format!(""),
			dns : DnsValue::Arr(Vec::new()),
		}
	}
}

fn nic_stat_state_default() -> String {
	format!("{}",NICSTATE_DHCP)
}

fn nic_stat_ip_default() -> String {
	format!("")
}

fn nic_stat_netmask_default() -> String {
	format!("")
}

fn nic_stat_gateway_default() -> String {
	format!("")
}

fn nic_stat_dns_default() -> DnsValue {
	DnsValue::Arr(Vec::new())
}


fn enumdns_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{JsonLibError,"need json value"}
	}
	let s = read_file(&sarr[0])?;
	let mut nval :ProtoNicStat = serde_json::from_str(&s)?;
	if sarr.len() == 2 {
		nval.dns = DnsValue::Str(format!("{}",sarr[1]));
	} else if sarr.len() > 2 {
		let mut cval :Vec<String> = vec![];
		let mut idx :usize = 1;
		while idx < sarr.len() {
			cval.push(format!("{}", sarr[idx]));
			idx += 1;
		}
		nval.dns = DnsValue::Arr(cval.clone());
	}

	let outs = serde_json::to_string(&nval)?;
	println!("{}", outs);
	Ok(())
}

pub trait AckProto {
	fn new_from_cmd(cmd :&ProtoCmd, res :i32) -> Self;
}

#[derive(Clone,Deserialize,Serialize)]
pub struct ProtoCmd {
	pub command :String,
	pub uid :u32,
}

#[derive(Clone,Deserialize,Serialize)]
pub struct ProtoAck {
	pub ack :String,
	pub uid :u32,
	pub res :i32,
}

impl Default for ProtoAck {
	fn default() -> Self {
		Self {
			ack : format!(""),
			uid : 1,
			res : 1,
		}
	}
}


#[derive(Clone,Deserialize,Serialize)]
pub struct ProtoCmdJsonFruSet {
	#[serde(flatten)]
	pub cmd :ProtoCmd,
	#[serde(flatten,default = "json_fru_set_map_default")]
	pub map :HashMap<String,serde_json::Value>,
}

fn json_fru_set_map_default() -> HashMap<String,serde_json::Value> {
	HashMap::new()
}


#[derive(Clone,Deserialize,Serialize)]
pub struct ProtoAckJsonFruSet {
	#[serde(flatten)]
	pub ack :ProtoAck,
}

impl AckProto for ProtoAckJsonFruSet {
	fn new_from_cmd(cmd :&ProtoCmd, res :i32) -> Self {
		Self {
			ack : ProtoAck {
				ack : format!("{}",cmd.command),
				uid : cmd.uid,
				res : res,
			},
		}
	}
}

fn hashenum_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{JsonLibError,"need json value"}
	}
	let s = read_file(&sarr[0])?;
	let nval :ProtoCmdJsonFruSet = serde_json::from_str(&s)?;
	for (k,v) in nval.map.iter() {
		println!("[{}]=[{:?}]",k,v);
	}
	Ok(())
}


#[extargs_map_function(enumerate_handler,add_handler,del_handler,parse_handler,enumdns_handler,hashenum_handler)]
pub fn load_json_cmd(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"{
		"enumerate<enumerate_handler>##file ... to enumerate json values##" : {
			"$" : "+"
		},
		"add<add_handler>##file key value to add key value ##" : {
			"$" : 3
		},
		"del<del_handler>##file keys ... to delete key##" : {
			"$" : "+"
		},
		"parse<parse_handler>##str ... to parse values##" : {
			"$" : "+"
		},
		"enumdns<enumdns_handler>##jsonfile dns ... ... to make dns value##" : {
			"$" : "+"
		},
		"hashenum<hashenum_handler>##jsonfile to list keys values##" : {
			"$" : 1
		}
	}"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())	
}


