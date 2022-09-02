#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::fileop::{write_file_bytes};


extargs_error_class!{ExpLogError}

fn explog_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let logname :String;
	let caplogname :String;
	let sarr :Vec<String>;
	let cf :String;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{ExpLogError,"need logname [CAPITAL_LOGNAME]"}
	}
	logname = format!("{}",sarr[0]);
	if sarr.len() > 1 {
		caplogname = format!("{}",sarr[1]);
	} else {
		caplogname = logname.to_uppercase();
	}

	debug_trace!("logname [{}] caplogname [{}]",logname,caplogname);

	cf = format!(r#"

use std::env;
use std::io::{{Write}};
use std::fs;

use lazy_static::lazy_static;
use chrono::{{Local,Timelike,Datelike}};



fn _{bname}_get_environ_var(envname :&str) -> String {{
	match env::var(envname) {{
		Ok(v) => {{
			format!("{{}}",v)
		}},
		Err(_e) => {{
			String::from("")
		}}
	}}
}}

struct LogVar {{
	level :i32,
	nostderr : bool,
	wfile : Option<fs::File>,
}}



fn {bname}_macro_log_init(prefix :&str) -> LogVar {{
	let mut getv :String;
	let mut retv :i32 = 0;
	let mut nostderr :bool = false;
	let mut coptfile :Option<fs::File> = None;
	let mut key :String;

	key = format!("{{}}_LEVEL", prefix);
	getv = _{bname}_get_environ_var(&key);
	if getv.len() > 0 {{
		match getv.parse::<i32>() {{
			Ok(v) => {{
				retv = v;
			}},
			Err(e) => {{
				retv = 0;
				eprintln!("can not parse [{{}}] error[{{}}]", getv,e);
			}}
		}}
	}}

	key = format!("{{}}_NOSTDERR",prefix);
	getv = _{bname}_get_environ_var(&key);
	if getv.len() > 0 {{
		nostderr = true;
	}}



	key = format!("{{}}_LOGFILE",prefix);
	getv = _{bname}_get_environ_var(&key);
	if getv.len() > 0 {{
		let fo = fs::File::create(&getv);
		if fo.is_err() {{
			eprintln!("can not open [{{}}]", getv);		
		}} else {{
			coptfile = Some(fo.unwrap());
		}}
	}}

	return LogVar {{
		level : retv,
		nostderr : nostderr,
		wfile : coptfile,		
	}};
}}

lazy_static! {{
	static ref {cname}_LOG_LEVEL : LogVar = {{
		{bname}_macro_log_init("{cname}")
	}};
}}


pub (crate)  fn {bname}_debug_out(level :i32, outs :&str) {{
	if {cname}_LOG_LEVEL.level >= level {{
		let c = format!("{{}}\n",outs);
		if !{cname}_LOG_LEVEL.nostderr {{
			let _ = std::io::stderr().write_all(c.as_bytes());
		}}

		if {cname}_LOG_LEVEL.wfile.is_some() {{
			let mut wf = {cname}_LOG_LEVEL.wfile.as_ref().unwrap();
			let _ = wf.write(c.as_bytes());
		}}
	}}
	return;
}}


pub (crate) fn {bname}_log_get_timestamp() -> String {{
	let now = Local::now();
	return format!("{{}}/{{}}/{{}} {{}}:{{}}:{{}}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
}}


macro_rules! {bname}_log_error {{
	($($arg:tt)+) => {{
		let mut c :String= format!("<ERROR>{{}}[{{}}:{{}}]  ",{bname}_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		{bname}_debug_out(0,&c);
	}}
}}

#[allow(unused_macros)]
macro_rules! {bname}_log_warn {{
	($($arg:tt)+) => {{
		let mut c :String= format!("<WARN>{{}}[{{}}:{{}}]  ",{bname}_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		{bname}_debug_out(10,&c);
	}}
}}


#[allow(unused_macros)]
macro_rules! {bname}_log_info {{
	($($arg:tt)+) => {{
		let mut c :String= format!("<INFO>{{}}[{{}}:{{}}]  ",{bname}_log_get_timestamp(),file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		{bname}_debug_out(20,&c);
	}}
}}

macro_rules! {bname}_log_trace {{
	($($arg:tt)+) => {{
		let mut _c :String= format!("<TRACE>{{}}[{{}}:{{}}]  ",{bname}_log_get_timestamp(),file!(),line!());
		_c.push_str(&(format!($($arg)+)[..]));
		{bname}_debug_out(40, &_c);
	}}
}}
"#,bname=logname,cname=caplogname);
	let fname = ns.get_string("output");
	_ = write_file_bytes(&fname,cf.as_bytes())?;
	Ok(())
}



#[extargs_map_function(explog_handler)]
pub fn load_log_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"explog<explog_handler>##logname [CAPITAL_LOGNAME] ##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
