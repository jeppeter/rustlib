use std::error::Error;
use std::boxed::Box;
use regex::Regex;
use std::fmt;
use serde_json::Value;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{UnsafeCell,RefCell};
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
	loadfuncs :Rc<RefCell<HashMap<String,Rc<RefCell<ExtArgsFunc>>>>>,
}


impl ExtArgsParserInner {

	pub fn new() -> ExtArgsParserInner {
		let mut retv :ExtArgsParserInner = ExtArgsParserInner {
			loadfuncs : Rc::new(RefCell::new(HashMap::new())),
		};
		retv.insert_load_command_funcs();
		retv
	}

	fn insert_load_command_funcs(&mut self)  {
		let b = Arc::new(UnsafeCell::new(self.clone()));
		let mut bmut =  self.loadfuncs.borrow_mut();
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_STRING),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  
			extargs_log_trace!("call [{}]", KEYWORD_STRING) ;
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v)
			} )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_INT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_INT) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_FLOAT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_FLOAT) ;  
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_LIST),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_LIST) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_BOOL),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_BOOL) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_ARGS),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_ARGS) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_args(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_COMMAND),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_COMMAND) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_command_subparser(n,k,v) 
		} )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_PREFIX),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_PREFIX) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_command_prefix(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_COUNT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_COUNT) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_HELP),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_HELP) ; 
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_JSONFILE),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_JSONFILE) ;  
			let  c :&mut ExtArgsParserInner = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		return;
	}	

	fn load_commandline_base(&mut self, _prefix :String, _keycls :ExtKeyParse, _parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("load_commandline_base");
		Ok(())
	}	
	fn load_commandline_args(&mut self, _prefix :String, _keycls :ExtKeyParse, _parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("load_commandline_args");
		Ok(())
	}

	fn load_command_prefix(&mut self,_prefix :String, _keycls :ExtKeyParse, _parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("load_command_prefix");
		Ok(())
	}


	fn load_command_subparser(&mut self,prefix :String, _keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		extargs_log_trace!("load_command_subparser");
		let nk :ExtKeyParse = ExtKeyParse::new(KEYWORD_STRING);
		return self.call_load_command_map_func(prefix,nk,parsers);
	}

	fn get_load_func(&self, k :&str) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.loadfuncs.borrow().get(k) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}

	fn call_load_command_map_func(&mut self,prefix :String,keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let fnptr :Option<ExtArgsFunc>;
		extargs_log_trace!("typename [{}]",keycls.type_name());
		fnptr = self.get_load_func(&(keycls.type_name()));
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::LoadFunc(f) => {
					return f(prefix,keycls.clone(),parsers.clone());
				},
				_ => {
					new_error!{ParserError,"return [{}] not load function", prefix}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load command map function", prefix}
		}
	}


	pub fn call_command(&mut self) -> Result<(),Box<dyn Error>> {
		let k :ExtKeyParse = ExtKeyParse::new(KEYWORD_COMMAND);
		let prefix :String = "".to_string();
		let parsers :Vec<ParserCompat> = Vec::new();
		return self.call_load_command_map_func(prefix,k,parsers);
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
