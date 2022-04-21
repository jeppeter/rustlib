use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone)]
pub struct TypeClass {
	typeval : String,
}


pub enum Nargs {	
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

#[derive(Clone)]
pub enum KeyVal {
	StrVal(Option<String>),
	BoolVal(Option<bool>),
	JsonVal(Option<Value>),
	KeyAttrVal(Option<KeyAttr>),
	NArgVal(Option<Nargs>),
	TypeVal(Option<TypeClass>),
}


#[derive(Clone)]
pub struct KeyData {
	data :HashMap<String,KeyVal>,
}

#[derive(Clone)]
pub struct ExtKeyParse {
	keydata : KeyData,
	__helpexpr :Regex,
	__cmdexpr : Regex,
	__prefixexpr : Regex,
	__funcexpr : Regex,
	__flagexpr : Regex,
	__mustflagexpr : Regex,
	__attrexpr : Regex,
}

impl KeyData {
	pub fn new() -> KeyData {
		let retval = KeyData{ data : HashMap::new() };
		return retval;
	}	
}

fn compile_regex(expr :&str) -> Regex {
	let mut retv :Regex;
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

#[derive(Clone)]
pub struct ExtArgsOptions {
	values :HashMap<String,Value>,
}


fn newkey() -> ExtKeyParse {
	let  key :ExtKeyParse;
	key = ExtKeyParse {
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

fn newoptions() -> ExtArgsOptions {
	ExtArgsOptions {
		values : HashMap::new(),
	}
}

fn main() {
	let kev = newkey();
	let _c :ExtKeyParse = kev.clone();
	let nopt = newoptions();
	let _d = nopt.clone();

}
