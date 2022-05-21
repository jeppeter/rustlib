use tempfile::TempDir;
use std::env;
use std::thread;
use std::time;
use std::fs;

#[#[derive(Debug)]]
struct FuncComposer {
	funcstr :String,
	helpfuncs :Vec<String>,
	jsonfuncs :Vec<String>,
	actfuncs : Vec<String>,
	callbackfuncs : Vec<String>,	
}

const FUNC_OPTHELP :&str = "opthelp";
const FUNC_JSONFUNC :&str = "jsonfunc";
const FUNC_ACTFUNC :&str = "actfunc";
const FUNC_CALLBACK :&str = "callbackfunc";


impl FuncComposer {
	pub fn new() -> FuncComposer {
		FuncComposer {
			funcstr : "".to_string(),
			helpfuncs : Vec::new(),
			jsonfuncs : Vec::new(),
			actfuncs : Vec::new(),
			callbackfuncs : Vec::new(),
		}
	}

	fn add_code(&mut self, code :&str) {
		self.funcstr.push_str(code);
		self.funcstr.push_str("\n");
		return;
	}

	pub fn add_json_func(&mut self,name :&str, code :&str) {
		self.add_code(code);
		self.jsonfuncs.push(format!("{}",name));
	}

	pub fn add_help_func(&mut self,name :&str, code :&str) {
		self.add_code(code);
		self.helpfuncs.push(format!("{}",name));
	}

	pub fn add_act_func(&mut self,name :&str, code :&str) {
		self.add_code(code);
		self.actfuncs.push(format!("{}",name));
	}
	pub fn add_call_back(&mut self,name :&str, code :&str) {
		self.add_code(code);
		self.callbackfuncs.push(format!("{}",name));
	}

	pub fn get_func(&self) -> String {
		return format!("{}",self.funcstr);
	}

	pub fn get_extargs_map_func(&self) -> String {
		let mut rets :String = "".to_string();
		let mut ival :i32 = 0;
		rets.push_str("#[extargs_map_function(");
		if self.helpfuncs.len() > 0 {
			for f in self.helpfuncs.iter() {
				if ival > 0 {
					rets.push_str(",");
				}
				rets.push_str(&(format!("{}={}",FUNC_OPTHELP,f)));
				ival += 1;
			}
		}

		if self.jsonfuncs.len() > 0 {
			for f in self.jsonfuncs.iter() {
				if ival > 0 {
					rets.push_str(",");
				}
				rets.push_str(&(format!("{}={}",FUNC_JSONFUNC,f)));
				ival += 1;
			}
		}

		if self.actfuncs.len() > 0 {
			for f in self.actfuncs.iter() {
				if ival > 0 {
					rets.push_str(",");
				}
				rets.push_str(&(format!("{}={}",FUNC_ACTFUNC,f)));
				ival += 1;
			}
		}

		if self.callbackfuncs.len() > 0 {
			for f in self.callbackfuncs.iter() {
				if ival > 0 {
					rets.push_str(",");
				}
				rets.push_str(&(format!("{}",f)));
				ival += 1;
			}
		}

		rets.push_str(")]");
		return rets;
	}
}

#[derive(Debug)]
struct ExtArgsDir {
	srcdir : String,
	workdir :String,
	gendir :String,	
	reserved : bool,
	tdir : TempDir,
}

impl ExtArgsDir {
	pub fn new(workdir :&str,gendir :&str) -> ExtArgsDir {
		let retv :ExtArgsDir = ExtArgsDir{
			srcdir : "".to_string(),
			workdir : format!("{}",workdir),
			gendir : format!("{}",gendir ),
			reserved : false,
			tdir : TempDir::new().unwrap(),
		};
		retv
	}

	pub fn write_rust_code(optstr :&str,cmdstr :&str, addmode :Vec<String>,funcstr :&str, priority :Vec<i32>) -> Result<(),Box<dyn Error>> {

	}
}



fn main() {
    let args :Vec<String> = env::args().collect();
    if args.len() >= 3 {
    	let d :ExtArgsDir = ExtArgsDir::new(&(args[1]),&(args[2]));
    	println!("d  [{:?}]", d);
    }
}
