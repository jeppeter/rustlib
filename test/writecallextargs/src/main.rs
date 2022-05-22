use tempfile::TempDir;
use std::env;
use std::thread;
use std::time;
use std::fs;

use extaparse_worker::{ExtArgsParser};



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

	fn uc_first(&self,n :&str) -> String {
	let cv :Vec<char> = n.chars().collect();
	let mut cidx :i32 =0;
	let mut rets :String = "".to_string();
	let bv :Vec<char> = n.to_uppercase().chars().collect();
	for c in cv.iter() {
		if cidx == 0 {
			rets.push(bv[0]);
		} else {
			rets.push(*c);
		}
		cidx += 1;
	}
	return rets;
}


	fn get_cmd_struct_name(&self, cmdname :&str) -> String {
		let mut rets :String = "".to_string();
		if cmdname.len() == 0 {
			rets.push_str("MainDataStruct");
		} else {
			let cv :Vec<&str> = cmdname.split(".").collect();
			for cs in cv.iter() {
				rets.push_str(&self.uc_first(cs));
			}
			rets.push_str("DataStruct");
		}
		return rets;
	}

	fn get_parser_struct(&self,tabs :i32 ,parser :ExtArgsParser,options :ExtArgsOptions, cmdname :&str) -> Result<String,Box<dyn Error>> {
		let mut rets :String = "".to_string();
		let subcmds :Vec<String>;
		let opts :Vec<ExtKeyParse>;
		let mut idx : i32 = 0;
		let  mut strprefix :String;

		if cmdname.len() > 0 {
			let v :Vec<&str> = cmdname.split(".").collect();
			strprefix = "".to_string();
			for c in v.iter() {
				let cv :Vec<char> = c.chars().collect();
				let mut cidx :i32 = 0;
				for cc in cv.iter() {
					if cidx  == 0 {
						strprefix.push(cc.to_uppercase());
					} else {
						strprefix.push(cc);
					}

					cidx += 1;
				}
			}
			strprefix.push_str("DataStruct");
		} else {
			strprefix = format!("MainDataStruct");
		}

		subcmds = parser.get_sub_commands_ex(cmdname)?;
		for c in subcmds.iter() {			
			let mut curcmd :String = format!("{}",cmdname);

			if curcmd.len() > 0 {
				curcmd.push_str(".");
			}
			curcmd.push_str(&(format!("{}", c)));
			let curs = self.get_parser_struct(tabs , parser.clone(),options.clone(),&curcmd)?;
			rets.push_str("\n");
			rets.push_str(&curs);
		}

		opts = parser.get_cmd_opts_ex(cmdname)?;
		for o in opts.iter() {
			if o.is_flag() && o.type_name() != KEYWORD_HELP && o.type_name() != KEYWORD_JSONFILE && o.type_name() != KEYWORD_PREFIX {
				let tname :String;
				let kname :String;
				if idx == 0 {
					rets.push_str("#[derive(ArgSet)]\n");
					rets.push_str(&(format!("struct {} {{\n",strprefix)));
				}
				if o.type_name() == KEYWORD_LIST {
					tname = format!("Vec<String>");
				} else if o.type_name() == KEYWORD_STRING {
					tname = format!("String");
				} else if o.type_name() == KEYWORD_INT {
					tname = format!("i64");
				} else if o.type_name() == KEYWORD_FLOAT {
					tname = format!("f64");
				} else if o.type_name() == KEYWORD_BOOL {
					tname = format!("bool");
				} else if o.type_name() == KEYWORD_ARGS {
					tname = format!("Vec<String>");
				} else if o.type_name() == KEYWORD_COUNT {
					tname = format!("i64");
				} else {
					extargs_new_error!{ExtArgsDirError,"not supported type [{}]", o.type_name()}
				}

				if o.type_name() != KEYWORD_ARGS {
					kname = format!("{}",o.flag_name());
				} else {
					if cmdname.len() > 0 {
						kname = format!("{}",KEYWORD_SUBNARGS);
					} else {
						kname = format!("{}",KEYWORD_ARGS);
					}
				}

				rets.push_str(&format!("    {} : {},",))
				idx += 1;
			}
		}

		for c in subcmds.iter() {			
			if idx == 0 {
				rets.push_str("#[derive(ArgSet)]\n");
				rets.push_str(&(format!("struct {} {{\n",strprefix)));
			}



			idx += 1;
		}

		if idx > 0 {
			rets.push_str("}}\n");
		}




		Ok(rets)
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
