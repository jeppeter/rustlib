use std::error::Error;
use std::boxed::Box;
use regex::Regex;
use std::fmt;
use serde_json::Value;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{UnsafeCell,RefCell,RefMut};
use std::collections::HashMap;


macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	#[allow(dead_code)]
	impl $type {
		fn create(c :&str) -> $type {
			$type {msg : format!("{}",c)}
		}
	}

	impl fmt::Display for $type {
		fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
			write!(f,"{}",self.msg)
		}
	}

	impl Error for $type {}
	};
}

macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
}

error_class!{ParserError}

macro_rules! extargs_log_trace {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}] ",file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		println!("{}",c);
	}
}


#[derive(Clone)]
pub struct NameSpaceEx {

}

impl NameSpaceEx {
	pub fn new() -> NameSpaceEx {
		NameSpaceEx{}
	}
	pub fn get_bool(&self, _k :&str) -> bool {
		return false;
	}
	pub fn get_int(&self,_k :&str) -> i64 {
		return 0;
	}
	pub fn get_float(&self,_k :&str) -> f64 {
		return 0.0;
	}
	pub fn get_array(&self,_k :&str) -> Vec<String> {
		return Vec::new();
	}
}

pub trait ArgSet {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
}


#[derive(Debug)]
struct Dimension {
	bb :f64,
	cc :f64,
}

error_class!{DimensionError}

impl ArgSet for Dimension {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		if k == "bb" {
			self.bb = ns.get_float(k);
		} else if k == "cc" {
			self.cc = ns.get_float(k);
		} else {
			new_error!{DimensionError,"{} not support", k}
		}
		Ok(())
	}

}



#[derive(Debug)]
struct PoinX {
	x :f64,
	y :f64,
	next :Option<Box<PoinX>>,
	bs : Dimension,
}

error_class!{PoinXError}

impl PoinX {
	fn new(x1:f64,y1:f64) -> PoinX {
		PoinX{x:x1,y:y1,next:None, bs :Dimension {
			bb :x1,
			cc :y1,
		}}		
	}
	fn add_next(&mut self,v :Option<Box<PoinX>>) -> &PoinX{
		self.next = v;
		self
	}
	fn get_x(&self) -> f64 {
		self.x
	}
	fn get_y(&self) -> f64 {
		self.y
	}
}



impl ArgSet for PoinX {
	fn set_value(&mut self, k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		if k == "x" {
			self.x = ns.get_float("x");
		} else if k == "y" {
			self.y = ns.get_float("y");
		} else if k.starts_with("bs.") {
			let nk = format!("{}",k);
			let re = Regex::new(r"^bs\.").unwrap();
			let kn = re.replace_all(&nk,"").to_string();
			println!("kn {}", kn);
			self.bs.set_value(&kn,ns.clone())?;
		} else {
			new_error!{PoinXError,"[{}] not support", k}
		}
		Ok(())
	}
}

pub const KEYWORD_STRING :&str = "string";
pub const KEYWORD_DICT :&str = "dict";
pub const KEYWORD_LIST :&str = "list";
pub const KEYWORD_BOOL :&str = "bool";
pub const KEYWORD_INT :&str = "int";
pub const KEYWORD_FLOAT :&str = "float";
pub const KEYWORD_ARGS :&str = "args";
pub const KEYWORD_HELP :&str = "help";
pub const KEYWORD_JSONFILE :&str = "jsonfile";
pub const KEYWORD_COUNT :&str = "count";
pub const KEYWORD_COMMAND :&str = "command";
pub const KEYWORD_PREFIX :&str = "prefix";

pub const COMMAND_SET                  :i32 = 10;
pub const SUB_COMMAND_JSON_SET         :i32 = 20;
pub const COMMAND_JSON_SET             :i32 = 30;
pub const ENVIRONMENT_SET              :i32 = 40;
pub const ENV_SUB_COMMAND_JSON_SET     :i32 = 50;
pub const ENV_COMMAND_JSON_SET         :i32 = 60;
pub const DEFAULT_SET                  :i32 = 70;


#[derive(Clone)]
struct ParserCompat {
}

#[derive(Clone)]
struct ExtKeyParse {
	typname :String,
}

impl ExtKeyParse {
	pub fn new(t :&str) -> ExtKeyParse {
		ExtKeyParse {
			typname : String::from(t),
		}
	}
	pub fn type_name(&self) -> String {
		return format!("{}",self.typname);
	}
}

#[allow(dead_code)]
#[derive(Clone)]
enum ExtArgsFunc {
	LoadFunc(Rc<dyn Fn(String,ExtKeyParse,Vec<ParserCompat>) -> Result<(),Box<dyn Error>>>),
	ActionFunc(Rc<dyn Fn(NameSpaceEx,i32,ExtKeyParse,Vec<String>) -> Result<i32,Box<dyn Error>>>),
	LoadJsonFunc(Rc<dyn Fn(NameSpaceEx) -> Result<(),Box<dyn Error>>>),
	JsonFunc(Rc<dyn Fn(NameSpaceEx,ExtKeyParse,Value) -> Result<(),Box<dyn Error>>>),	
}

#[derive(Clone)]
struct ExtArgsParserInner {
	setmapfuncs :Rc<RefCell<HashMap<i32,Rc<RefCell<ExtArgsFunc>>>>>,
	val : Rc<RefCell<i32>>,
	load_priority :Vec<i32>,
}


impl ExtArgsParserInner {

	pub fn new() -> ExtArgsParserInner {
		let mut retv :ExtArgsParserInner = ExtArgsParserInner {
			setmapfuncs : Rc::new(RefCell::new(HashMap::new())),
			val : Rc::new(RefCell::new(0)),
			load_priority : vec![SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET],
		};

		retv.insert_setmap_funcs();
		retv
	}


	fn parse_subcommand_json_set(&self, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("val [{}]", self.val.borrow());
		Ok(())
	}

	fn parse_command_json_set(&self, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("val [{}]", self.val.borrow());
		Ok(())
	}

	fn parse_environment_set(&self, ns :NameSpaceEx) ->  Result<(),Box<dyn Error>> {
		extargs_log_trace!("val [{}]", self.val.borrow());
		Ok(())
	}

	fn parse_env_subcommand_json_set(&self,ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("val [{}]", self.val.borrow());
		Ok(())		
	}

	fn parse_env_command_json_set(&self, ns :NameSpaceEx)  -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("val [{}]", self.val.borrow());
		Ok(())		
	}


	fn get_setmap_func(&self, val :i32) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.setmapfuncs.borrow().get(&val) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}

	fn insert_setmap_funcs(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",SUB_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(SUB_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n| { s1.borrow().parse_subcommand_json_set(n) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n| { s1.borrow().parse_command_json_set(n) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENVIRONMENT_SET);
		self.setmapfuncs.borrow_mut().insert(ENVIRONMENT_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n| { s1.borrow().parse_environment_set(n) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENV_SUB_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(ENV_SUB_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n| { s1.borrow().parse_env_subcommand_json_set(n) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENV_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(ENV_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n| { s1.borrow().parse_env_command_json_set(n) })))));
		return;
	}


	fn call_parse_setmap_func(&self,idx :i32,ns:NameSpaceEx) -> Result<(),Box<dyn Error>> {
		let fnptr = self.get_setmap_func(idx);
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::LoadJsonFunc(f) => {
					return f(ns);
				},
				_ => {
					new_error!{ParserError,"return [{}] not LoadJsonFunc", idx}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load json  function", idx}
		}
	}


	pub fn call_command(&mut self) -> Result<(),Box<dyn Error>> {
		let ns = NameSpaceEx::new();
		for p in self.load_priority.clone() {
			{
		      let mut age_mut_ref: RefMut<i32> = self.val.borrow_mut();
		       *age_mut_ref += 1;
			}
			self.call_parse_setmap_func(p,ns.clone())?;
		}
		Ok(())
	}
}

fn main() {
    let mut xc :Box<PoinX> = Box::new(PoinX::new(1.1,1.1));
    let mut c :Box<PoinX> = Box::new(PoinX::new(2.2,2.2));
    let bc :Box<PoinX> = Box::new(PoinX::new(3.3,3.3));
    let ns :NameSpaceEx = NameSpaceEx::new();
    let mut cc :ExtArgsParserInner = ExtArgsParserInner::new();
    c.add_next(Some(bc));
    xc.add_next(Some(c));
    xc.bs.bb = 2.2;
    xc.bs.cc = 3.2;
    xc.set_value("bs.bb",ns.clone()).unwrap();
    xc.set_value("bs.cc",ns.clone()).unwrap();
    println!("xc {:?} x {} y {}", xc,xc.get_x(),xc.get_y());
    cc.call_command().unwrap();
}
