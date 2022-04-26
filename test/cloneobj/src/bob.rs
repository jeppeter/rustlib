use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;


#[allow(dead_code)]
#[derive(Clone,Debug)]
struct TypeClass {
	typeval : String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Nargs {	
	Argtype(String),
	Argnum(i32),
}

impl Clone for Nargs {
	fn clone(&self) -> Nargs {
		match self {
			Nargs::Argtype(s) => {
				return Nargs::Argtype(s.clone());
			},
			Nargs::Argnum(iv) => {
				return Nargs::Argnum(*iv);
			}
		}
	}
}


#[derive(Debug)]
pub struct KeyAttr {
	__splitchar :char,
	__obj :HashMap<String,String>,
}

impl Clone for KeyAttr {
	fn clone(&self) -> KeyAttr {
		let mut retattr :KeyAttr = KeyAttr{__splitchar : self.__splitchar, __obj : HashMap::new(),};
		for (k,v) in &(self.__obj) {
			retattr.__obj.insert(String::from(k),String::from(v));
		}
		return retattr;
	}
}

#[allow(dead_code)]
#[derive(Clone,Debug)]
enum KeyVal {
	StrVal(Option<String>),
	BoolVal(Option<bool>),
	JsonVal(Option<Value>),
	KeyAttrVal(Option<KeyAttr>),
	NArgVal(Option<Nargs>),
	TypeVal(Option<TypeClass>),
}


#[allow(dead_code)]
#[derive(Clone)]
struct KeyData {
	data :HashMap<String,KeyVal>,
}

struct InnerExtKeyParse {
	keydata : KeyData,
	__helpexpr :Regex,
	__cmdexpr : Regex,
	__prefixexpr : Regex,
	__funcexpr : Regex,
	__flagexpr : Regex,
	__mustflagexpr : Regex,
	__attrexpr : Regex,
}

type RcExtKeyParse = Rc<RefCell<InnerExtKeyParse>>;

#[derive(Clone)]
pub struct ExtKeyParse {
	c : RcExtKeyParse,
}

impl KeyData {
	pub fn new() -> KeyData {
		let retval = KeyData{ data : HashMap::new() };
		return retval;
	}	
	pub fn set_string(&mut self,key :&str, val :&str) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vs :String = String::from(val);
		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::StrVal(Some(vs)));
		
		return retval;
	}

	pub fn string(&self) -> String {
		let mut rets :String = "".to_string();
		let mut i :i32 = 0;
		for (k,v) in self.data.iter() {
			if i > 0 {
				rets.push_str(",");
			}
			rets.push_str(&format!("[{}]=[{:?}]",k,v));
			i += 1;
		}
		rets
	}
}

fn compile_regex(expr :&str) -> Regex {
	let retv :Regex;
	match Regex::new(expr) {
		Err(e) => {
			panic!("compile [{}] error[{:?}]",expr,e);
		},
		Ok(v) => {
			retv = v
		},
	}
	retv
}

struct InnerExtArgsOptions {
	values :HashMap<String,Value>,
}

type RcExtArgsOptions = Rc<RefCell<InnerExtArgsOptions>>;

#[derive(Clone)]
pub struct ExtArgsOptions {
	c :RcExtArgsOptions,
}





impl InnerExtKeyParse {
	pub fn new() -> InnerExtKeyParse {
		let key = InnerExtKeyParse {
			keydata : KeyData::new(),
			__helpexpr : compile_regex("##([^#]+)##$"),
			__cmdexpr : compile_regex("^([^#<>\\+\\$!]+)"),
			__prefixexpr : compile_regex("\\+([a-zA-Z]+[a-zA-Z0-9]*)"),
			__funcexpr : compile_regex("<([^<>\\$| \t!\\+]+)>"),
			__flagexpr : compile_regex("^([a-zA-Z]+[a-zA-Z0-9|\\?\\-]*)"),
			__mustflagexpr : compile_regex("^\\$([a-zA-Z]+[a-zA-Z0-9|\\?\\-]*)"),
			__attrexpr : compile_regex("!([^<>\\$!#|]+)!"),
		};
		key
	}


	pub fn set_string(&mut self,k :&str, v :&str) -> bool {
		return self.keydata.set_string(k,v);
	}

	pub fn string(&self) -> String {
		return self.keydata.string();
	}
}


impl InnerExtArgsOptions {
	// add code here
	pub fn string(&self) -> String {
		let mut rets :String = "".to_string();
		let mut idx :i32 = 0;

		for (k,v) in self.values.clone() {
			if idx > 0 {
				rets.push_str(",");
			}
			rets.push_str(&(format!("[{}]=[{:?}]", k,v)));
			idx += 1;
		}
		rets
	}

	pub fn insert(&mut self, k :&str, v :Value) {
		self.values.insert(format!("{}",k), v.clone());
	}
}



impl ExtKeyParse {
	pub fn new() -> ExtKeyParse {
		let k = InnerExtKeyParse::new();
		ExtKeyParse	{
			c :Rc::new(RefCell::new(k))
		}		
	}

	pub fn set_string(&self, k:&str,v :&str) {
		self.c.borrow_mut().set_string(k,v);
	}

	pub fn string(&self) -> String {
		return self.c.borrow().string();
	}
}

impl ExtArgsOptions {
	pub fn new() -> ExtArgsOptions {
		let k :InnerExtArgsOptions = InnerExtArgsOptions {
			values : HashMap::new(),
		};
		ExtArgsOptions {
			c :	Rc::new(RefCell::new(k)),
		}		
	}

	pub fn string(&self) -> String {
		return self.c.borrow().string();
	}

	pub fn insert(&self, k :&str, v :Value) {
		self.c.borrow_mut().insert(k,v);
	}
}

