#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::options::{ExtArgsOptions};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};
#[allow(unused_imports)]
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};


#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[cfg(windows)]
mod wchar_windows;
#[cfg(windows)]
mod loglib_windows;
mod loglib;
mod fileop;
mod logsrc;



#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
	let mut optargs :Option<Vec<String>> = None;
	let args :Vec<String> = std::env::args().collect();
	if args.len() >= 2 && args[1] == "expsrc" {
		let mut vargs :Vec<String> = Vec::new();

		for iv in 0..args.len() {
			if iv != 1  && iv != 0 {
				vargs.push(format!("{}",args[iv]));
			}
		}
		optargs = Some(vargs.clone());
	}
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	let commandline = r#"
	{
		"output|o" : null,
		"input|i" : null
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	logsrc::load_log_handler(parser.clone())?;

	let ores = parser.parse_commandline_ex(optargs,None,None,None);
	if ores.is_err() {
		let e = ores.err().unwrap();
		eprintln!("{:?}", e);
		return Err(e);
	}


	Ok(())
}
