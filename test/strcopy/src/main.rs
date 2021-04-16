use std::collections::HashMap;


const KEYWORD_VALUE :&str = "value";
const KEYWORD_PREFIX :&str = "prefix";
const KEYWORD_FLAGNAME :&str = "flagname";
const KEYWORD_HELPINFO :&str = "helpinfo";
const KEYWORD_SHORTFLAG :&str = "shortflag";
const KEYWORD_NARGS :&str = "nargs";
const KEYWORD_VARNAME :&str = "varname";
const KEYWORD_CMDNAME :&str = "cmdname";
const KEYWORD_FUNCTION :&str = "function";
const KEYWORD_ORIGKEY :&str = "origkey";
const KEYWORD_ISCMD :&str = "iscmd";
const KEYWORD_ISFLAG :&str = "isflag";
const KEYWORD_TYPE :&str = "type";
const KEYWORD_ATTR :&str = "attr";
const KEYWORD_LONGPREFIX :&str = "longprefix";
const KEYWORD_SHORTPREFIX :&str = "shortprefix";
const KEYWORD_LONGOPT :&str = "longopt";
const KEYWORD_SHORTOPT :&str = "shortopt";
const KEYWORD_OPTDEST :&str = "optdest";
const KEYWORD_NEEDARG :&str = "needarg";
const KEYWORD_BLANK :&str  = "";

const KEYWORD_NOCHANGE :&str  = "nochange";

#[allow(dead_code)]
const FLAGSPECIAL : &'static [&'static str] = &[KEYWORD_VALUE,KEYWORD_PREFIX];
#[allow(dead_code)]
const FLAGWORDS :&'static [&'static str] = &[KEYWORD_FLAGNAME,KEYWORD_HELPINFO,KEYWORD_SHORTFLAG,KEYWORD_NARGS,KEYWORD_VARNAME];
#[allow(dead_code)]
const CMDWORDS :&'static [&'static str] = &[KEYWORD_CMDNAME,KEYWORD_FUNCTION,KEYWORD_HELPINFO];
#[allow(dead_code)]
const OTHERWORDS :&'static [&'static str] = &[KEYWORD_ORIGKEY,KEYWORD_ISCMD,KEYWORD_ISFLAG,KEYWORD_TYPE,KEYWORD_ATTR,KEYWORD_LONGPREFIX,KEYWORD_SHORTPREFIX];
#[allow(dead_code)]
const FORMWORDS :&'static [&'static str] = &[KEYWORD_LONGOPT,KEYWORD_SHORTOPT,KEYWORD_OPTDEST,KEYWORD_NEEDARG];

pub enum Nargs {	
	Argtype(String),
	Argnum(i32),
}

impl PartialEq for Nargs {
	fn eq (&self, other :&Self) -> bool {
		let mut retval :bool = false;
		match self {
			Nargs::Argtype(s) => {
				match other {
					Nargs::Argtype(os) => {
						if s == os {
							retval = true;
						}
					},
					_ => {},
				}
			},
			Nargs::Argnum(i) => {
				match other {
					Nargs::Argnum(oi) => {
						if i == oi {
							retval = true;
						}
					},
					_ => {},
				}
			}
		}
		return retval;
	}
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


pub enum KeyVal {
	StrVal(Option<String>),
	NArgVal(Option<Nargs>),
}

impl Clone for KeyVal {
	fn clone(&self) -> KeyVal {
		match self {
			KeyVal::StrVal(Some(v)) => {
				return KeyVal::StrVal(Some(v.clone()));
			},
			KeyVal::StrVal(None) => {
				return KeyVal::StrVal(None);
			},
			KeyVal::NArgVal(Some(ni)) => {
				match ni {
					Nargs::Argtype(s) => {
						return KeyVal::NArgVal(Some(Nargs::Argtype(s.clone())));
					},
					Nargs::Argnum(i) => {
						return KeyVal::NArgVal(Some(Nargs::Argnum(*i)));
					}
				}
			},
			KeyVal::NArgVal(None) => {
				return KeyVal::NArgVal(None);
			}
		}
	}
}


pub struct KeyData {
	data :HashMap<String,KeyVal>,
}

impl KeyData {

	pub fn reset(&mut self) {
		self.data.clear();
		self.data.insert(String::from(KEYWORD_PREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_FLAGNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_HELPINFO),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_SHORTFLAG),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_NARGS),KeyVal::NArgVal(None));
		self.data.insert(String::from(KEYWORD_VARNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_CMDNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_FUNCTION),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_ORIGKEY),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_TYPE),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_LONGPREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_SHORTPREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		return;
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

	pub fn get_string(&self,key :&str) -> Option<String> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::StrVal(kv2) => {
						match kv2 {
							Some(sv) => {
								return Some(sv.clone());
							},
							_ => {return None;},
						}						
						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}
	}


	pub fn set_nargs(&mut self, key :&str, val :&Nargs) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vb :Nargs ;

		match val {
			Nargs::Argtype(s) => {vb = Nargs::Argtype(s.clone());},
			Nargs::Argnum(v) => {vb = Nargs::Argnum(*v);},
		}

		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::NArgVal(Some(vb)));
		
		return retval;		
	}

	pub fn get_nargs(&self, key :&str) -> Option<Nargs> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::NArgVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								return Some(kv3.clone());
							},
							_ => {
								return None;
							}
						}						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}		
	}


	pub fn new() -> KeyData {
		let mut retval = KeyData{ data : HashMap::new() };
		retval.reset();
		return retval;
	}
}




fn main() {
    let c :KeyData = KeyData::new();
    println!("get [{}]=[{:?}]",KEYWORD_SHORTFLAG,c.get_string(KEYWORD_SHORTFLAG));
}
