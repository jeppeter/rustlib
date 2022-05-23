use tempfile::TempDir;
use std::env;
use std::thread;
use std::time;
use std::fs;
use std::io::prelude::*;
use std::error::Error;

use extargsparse_worker::{extargs_new_error,extargs_error_class};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::options::{ExtArgsOptions};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_PREFIX,KEYWORD_LIST,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_BOOL,KEYWORD_ARGS,KEYWORD_COUNT,KEYWORD_SUBNARGS};
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use extargsparse_codegen::{extargs_load_commandline};




#[derive(Debug,Clone)]
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


#[allow(dead_code)]
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

extargs_error_class!{ExtArgsDirError}

#[derive(Debug)]
struct ExtArgsDir {
	srcdir : String,
	workdir :String,
	gendir :String,	
	tdir : TempDir,
}

impl ExtArgsDir {
	pub fn new(workdir :&str,gendir :&str) -> ExtArgsDir {
		let mut retv :ExtArgsDir = ExtArgsDir{
			srcdir : "".to_string(),
			workdir : format!("{}",workdir),
			gendir : format!("{}",gendir ),
			tdir : TempDir::new().unwrap(),
		};
		let srcd = retv.tdir.path().join("src");
		retv.srcdir = format!("{}",srcd.display());
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

	fn get_parser_struct(&self,tabs :i32 ,parser :ExtArgsParser, cmdname :&str) -> Result<String,Box<dyn Error>> {
		let mut rets :String = "".to_string();
		let subcmds :Vec<String>;
		let opts :Vec<ExtKeyParse>;
		let mut idx : i32 = 0;
		let strprefix :String;

		strprefix = self.get_cmd_struct_name(cmdname);

		subcmds = parser.get_sub_commands_ex(cmdname)?;
		for c in subcmds.iter() {			
			let mut curcmd :String = format!("{}",cmdname);

			if curcmd.len() > 0 {
				curcmd.push_str(".");
			}
			curcmd.push_str(&(format!("{}", c)));
			let curs = self.get_parser_struct(tabs , parser.clone(),&curcmd)?;
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

				rets.push_str(&format!("    {} : {},\n", kname, tname));
				idx += 1;
			}
		}

		for c in subcmds.iter() {			
			if idx == 0 {
				rets.push_str("#[derive(ArgSet)]\n");
				rets.push_str(&(format!("struct {} {{\n",strprefix)));
			}

			let mut cname :String = "".to_string();
			let kname :String;
			if cmdname.len() > 0 {
				cname.push_str(cmdname);
				cname.push_str(".");
			}
			cname.push_str(c);

			kname = self.get_cmd_struct_name(&cname);

			rets.push_str(&format!("    {} : {},\n",c,kname));
			idx += 1;
		}

		if idx > 0 {
			rets.push_str("}\n");
		}
		Ok(rets)
	}

	fn format_priority(&self, priority :Option<Vec<i32>>) -> String {
		let mut rets :String = "None".to_string();

		if priority.is_some() {
			rets = "Some".to_string();
			rets.push_str("(vec![");
			let pv = priority.unwrap().clone();
			let mut idx :i32 = 0;
			for v in pv.iter() {
				if idx > 0 {
					rets.push_str(",");
				}
				if *v == COMMAND_SET {
					rets.push_str("COMMAND_SET");
				} else if *v == SUB_COMMAND_JSON_SET {
					rets.push_str("SUB_COMMAND_JSON_SET");
				} else if *v == COMMAND_JSON_SET {
					rets.push_str("COMMAND_JSON_SET");
				} else if *v == ENVIRONMENT_SET {
					rets.push_str("ENVIRONMENT_SET");
				} else if *v == ENV_SUB_COMMAND_JSON_SET {
					rets.push_str("ENV_SUB_COMMAND_JSON_SET");
				} else if *v == ENV_COMMAND_JSON_SET {
					rets.push_str("ENV_COMMAND_JSON_SET");
				} else if *v == DEFAULT_SET {
					rets.push_str("DEFAULT_SET");
				} else {
					panic!("not valid priority");
				}
				idx += 1;
			}


			rets.push_str("])");
		}

		rets
	}

	fn format_imports(&self) -> String {
		let mut rets :String = "".to_string();
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use extargsparse_worker::{extargs_error_class,extargs_new_error};\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use extargsparse_worker::namespace::{NameSpaceEx};\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use extargsparse_worker::argset::{ArgSetImpl};\n");
		rets.push_str("use extargsparse_worker::parser::{ExtArgsParser};\n");
		rets.push_str("use extargsparse_worker::funccall::{ExtArgsParseFunc};\n");
		rets.push_str("use extargsparse_worker::options::{ExtArgsOptions};\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};\n");
		rets.push_str("\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use std::cell::RefCell;\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use std::sync::Arc;\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use std::error::Error;\n");
		rets.push_str("use std::boxed::Box;\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use regex::Regex;\n");
		rets.push_str("#[allow(unused_imports)]\n");
		rets.push_str("use std::any::Any;\n");
		rets.push_str("use lazy_static::lazy_static;\n");
		rets.push_str("use std::collections::HashMap;\n");
		rets.push_str("\n");
		rets.push_str("\n");

		rets
	}
	fn format_extargs_map_functions(&self,fcomposer :FuncComposer) -> String {
		let mut rets =  fcomposer.get_extargs_map_func();
		rets.push_str("\n");
		rets
	}

	fn format_cargo_toml(&self) -> String {
		let mut rets :String = "".to_string();
		rets.push_str("[package]\n");
		rets.push_str("name = \"callextargs\"\n");
		rets.push_str("version = \"0.1.0\"\n");
		rets.push_str("edition = \"2018\"\n");
		rets.push_str("\n");
		rets.push_str("[dependencies]\n");
		rets.push_str(&format!("extargsparse_codegen = {{ path = \"{}\"}}\n",self.gendir.replace("\\","\\\\")));
		rets.push_str(&format!("extargsparse_worker = {{ path = \"{}\" }}\n", self.workdir.replace("\\","\\\\")));
		rets.push_str("regex = \"1\"\n");
		rets.push_str("lazy_static = \"^1.4.0\"\n");
		rets.push_str("\n");

		rets
	}

	fn write_cargo_toml(&self) -> Result<(),Box<dyn Error>> {
		let cargopath = self.tdir.path().join("Cargo.toml").display().to_string();
		let mut fp: fs::File = fs::File::create(&cargopath).unwrap();
		fp.write_all(self.format_cargo_toml().as_bytes())?;
		Ok(())
	}

	fn format_print_out(&self,parser :ExtArgsParser,cmdname :&str, piname :&str) -> String {
		let subcmds :Vec<String>;
		let mut rets :String = "".to_string();

		subcmds = parser.get_sub_commands_ex(cmdname).unwrap();
		for c in subcmds.iter() {
			let mut curcmd :String = "".to_string();
			curcmd.push_str(cmdname);
			if curcmd.len() > 0 {
				curcmd.push_str(".");
			}
			curcmd.push_str(c);
			rets.push_str(&self.format_print_out(parser.clone(),&curcmd,piname));
		}

		let opts = parser.get_cmd_opts_ex(cmdname).unwrap();
		let mut rename = format!("{}",cmdname);
		if rename.len() == 0 {
			rename = format!("main");
		}
		rets.push_str(&format!("    /* print out {} */\n",rename));
		for o in opts.iter() {
			if o.is_flag() && o.type_name() != KEYWORD_HELP && o.type_name() != KEYWORD_JSONFILE && 
				o.type_name() != KEYWORD_PREFIX {
				if o.type_name() != KEYWORD_ARGS {
					if cmdname.len() > 0 {
						if o.type_name() == KEYWORD_LIST {
							rets.push_str(&format!("    println!(\"{}.{}.{} = {{:?}}\", {}.borrow().{}.{});\n", piname,cmdname,o.flag_name(), piname,cmdname,o.flag_name()));	
						} else {
							rets.push_str(&format!("    println!(\"{}.{}.{} = {{}}\", {}.borrow().{}.{});\n", piname,cmdname,o.flag_name(), piname,cmdname,o.flag_name()));		
						}
						
					} else {
						if o.type_name() == KEYWORD_LIST {
							rets.push_str(&format!("    println!(\"{}.{} = {{:?}}\", {}.borrow().{});\n", piname,o.flag_name(), piname,o.flag_name()));	
						} else {
							rets.push_str(&format!("    println!(\"{}.{} = {{}}\", {}.borrow().{});\n", piname,o.flag_name(), piname,o.flag_name()));		
						}
						
					}
					
				} else {
					let  argname :String;
					if cmdname.len() > 0 {
						argname = format!("{}",KEYWORD_SUBNARGS);
						rets.push_str(&format!("    println!(\"{}.{}.{} = {{:?}}\", {}.borrow().{}.{});\n", piname, cmdname,argname, piname,cmdname,argname));
					} else {
						argname = format!("{}",KEYWORD_ARGS);
						rets.push_str(&format!("    println!(\"{}.{} = {{:?}}\", {}.borrow().{});\n", piname,argname, piname,argname));
					}
					
				}
			}
		}
		rets.push_str("\n");
		rets
	}

	fn format_main(&self,optstr :&str,cmdstr :&str,printout : bool, parser :ExtArgsParser,priority :Option<Vec<i32>>,nsname :&str, piname :&str) -> String {
		let mut rets :String = "".to_string();

		rets.push_str("fn main() -> Result<(),Box<dyn Error>> {\n");
		rets.push_str("    let loads = r#\"");
		if cmdstr.len() > 0 {
			rets.push_str(cmdstr);	
		} else {
			rets.push_str("{}");
		}		
		rets.push_str("\"#;\n");
		rets.push_str("    let optstr = r#\"");
		if optstr.len() > 0 {
			rets.push_str(optstr);	
		} else {
			rets.push_str("{}");
		}		
		rets.push_str("\"#;\n");
		rets.push_str("    let optref = ExtArgsOptions::new(optstr)?;\n");
		rets.push_str(&format!("    let parser = ExtArgsParser::new(Some(optref.clone()),{})?;\n",self.format_priority(priority)));
		rets.push_str("    extargs_load_commandline!(parser,loads)?;\n");

		if printout {
			rets.push_str(&format!("    let {} = Arc::new(RefCell::new(MainDataStruct::new()));\n",piname));			
			rets.push_str(&format!("    let {} = parser.parse_commandline_ex(None,None,Some({}.clone()),None)?;\n",nsname,piname));
		} else {
			rets.push_str(&format!("    let {} = parser.parse_commandline_ex(None,None,None,None)?\n",nsname));
		}

		if printout {
			rets.push_str(&self.format_print_out(parser.clone(),"",piname));
		}

		rets.push_str("    Ok(())");

		rets.push_str("}\n");
		rets
	}

	fn write_main_rs(&self,mainrs :&str) -> Result<(),Box<dyn Error>> {
		let mainrspath = self.tdir.path().join("src").join("main.rs").display().to_string();
		let maindirpath = self.tdir.path().join("src").display().to_string();
		fs::create_dir_all(&maindirpath)?;
		let mut fp: fs::File = fs::File::create(&mainrspath).unwrap();
		fp.write_all(mainrs.as_bytes())?;
		Ok(())
	}

	pub fn write_rust_code(&self,optstr :&str,cmdstr :&str, _addmode :Vec<String>,fcomposer :FuncComposer, priority :Option<Vec<i32>>,printout :bool, nsname :&str, piname :&str) -> Result<(),Box<dyn Error>> {
		self.write_cargo_toml()?;
		/*to get write main file*/
		let mut rets :String = "".to_string();
		rets.push_str(&(self.format_imports()));
		rets.push_str(&fcomposer.funcstr);

		let mut opt :Option<ExtArgsOptions> = None;
		if optstr.len() > 0 {
			let ov = ExtArgsOptions::new(optstr)?;
			opt = Some(ov.clone());
		}
		let mut inprior :Option<Vec<i32>> = None;
		if priority.is_some() {
			inprior =Some(priority.as_ref().unwrap().clone());
		}
		let parser :ExtArgsParser = ExtArgsParser::new(opt, inprior)?;
		/*now for the */
		extargs_load_commandline!(parser,cmdstr)?;
		/*now to get the string*/
		rets.push_str(&(self.get_parser_struct(0,parser.clone(),"").unwrap()));

		rets.push_str(&self.format_extargs_map_functions(fcomposer.clone()));
		inprior = None;
		if priority.is_some() {
			inprior =Some(priority.as_ref().unwrap().clone());
		}
		rets.push_str(&self.format_main(optstr,cmdstr,printout, parser.clone(),inprior,nsname,piname));

		self.write_main_rs(&rets)?;
		Ok(())
	}
}


fn read_file(fname :&str) -> String {
	let ferr = fs::File::open(fname);
	let mut rets = String::new();
	if ferr.is_err() {
		return "".to_string();
	}
	let mut f = ferr.unwrap();

	let nerr = f.read_to_string(&mut rets);
	if nerr.is_err() {
		return "".to_string();
	}

	return rets;
}

fn main() -> Result<(),Box<dyn Error>> {
	let args :Vec<String> = env::args().collect();
	if args.len() >= 4 {
		let gendir :String = format!("{}",args[2]);
		let workdir :String = format!("{}",args[1]);
		let d :ExtArgsDir = ExtArgsDir::new(&workdir,&gendir);
		let cmdstr :String = read_file(&args[3]);
		let fcomposer :FuncComposer = FuncComposer::new();
		let mut optstr :String = "".to_string();
		let addmode :Vec<String> = Vec::new();
		if args.len() >= 5{
			optstr = read_file(&args[4]);
		}

		d.write_rust_code(&optstr,&cmdstr,addmode.clone(),fcomposer.clone(),None,true,"ns","piargs")?;
		println!("write code in [{}]", d.srcdir);
		while 1 == 1 {
			thread::sleep(time::Duration::from_millis(1000));
		}
	}
	Ok(())
}
