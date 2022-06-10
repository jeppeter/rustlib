
use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use super::loglib_windows::{win_output_debug};
use lazy_static::lazy_static;
use log::{LevelFilter};
use log::{error, info, trace,warn};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root,RootBuilder,ConfigBuilder};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use std::error::Error;
use std::boxed::Box;
use chrono::{Local,Datelike,Timelike};
use std::collections::HashMap;

use std::sync::{Mutex,Arc};



const DEFAULT_MSG_FMT :&str = "{m}";

lazy_static! {
	static ref LOGGER_LEVEL :Arc<Mutex<i64>> = Arc::new(Mutex::new(0 as i64));
}

fn get_logger_level() -> i64 {
	let scb = LOGGER_LEVEL.lock().unwrap();
	let retv :i64;
	retv = *scb;
	return retv;
}

fn set_logger_level(nv :i64) -> i64 {
	let mut scb = LOGGER_LEVEL.lock().unwrap();
	let retv :i64;

	retv = *scb;
	*scb = nv;
	return retv;
}

pub fn init_log(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let mut level :LevelFilter  = log::LevelFilter::Error;
	let mut rbuiler :RootBuilder;
	let mut cbuild :ConfigBuilder;
	let mut sarr :Vec<String>;
	let retv :i64;
	let nostderr :bool;
	let stderr =ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(DEFAULT_MSG_FMT))).target(Target::Stderr).build();

	println!("1");
	retv = ns.get_int("verbose");

	if retv >= 4 {
		level = log::LevelFilter::Trace;
	} else if retv >= 3 {
		level = log::LevelFilter::Debug;
	} else if retv >= 2 {
		level = log::LevelFilter::Info;
	} else if retv >= 1 {
		level = log::LevelFilter::Warn;
	}

	println!("2");

	set_logger_level(retv);

	cbuild = Config::builder();
	rbuiler = Root::builder();
	nostderr = ns.get_bool("log_nostderr");

	println!("3");


	if !nostderr {
		cbuild = cbuild.appender(
			Appender::builder()
			.filter(Box::new(ThresholdFilter::new(level)))
			.build("stderr", Box::new(stderr)),
			);
		rbuiler = rbuiler.appender("stderr");		
	}

	println!("4");


	sarr = ns.get_array("log_files");
	for wf in sarr.iter() {
		let logfile = FileAppender::builder().append(false).encoder(Box::new(PatternEncoder::new(DEFAULT_MSG_FMT))).build(wf)?;
		cbuild = cbuild.appender(Appender::builder().build(wf, Box::new(logfile)));
		rbuiler = rbuiler.appender(wf);
	}


	println!("5");

	sarr = ns.get_array("log_appends");
	for wf in sarr.iter() {
		let logfile = FileAppender::builder().append(true).encoder(Box::new(PatternEncoder::new(DEFAULT_MSG_FMT))).build(wf)?;
		cbuild = cbuild.appender(Appender::builder().build(wf, Box::new(logfile)));
		rbuiler = rbuiler.appender(wf);
	}

	println!("6");


	let config = cbuild.build(rbuiler.build(level))?;
	println!("7");

	let _handle = log4rs::init_config(config)?;
	println!("8");

	Ok(())
}


#[extargs_map_function()]
pub fn prepare_log(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"{
			"verbose|v" : "+",
			"log-files##set write rotate files##" : [],
			"log-appends##set append files##" : [],
			"log-nostderr##specified no stderr output##" : false
	}"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())	
}

pub fn log_get_timestamp() -> String {
	let now = Local::now();
	return format!("{}/{}/{} {}:{}:{}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
}

fn log_output_function_inner(level :i64, outs :&str) {
	if level <= get_logger_level() {
		if level == 1 {
			error!("{}",outs);
		} else if level == 2 {
			warn!("{}",outs);
		} else if level == 3 {
			info!("{}",outs);
		} else if level >= 4 {
			trace!("{}",outs);
		}
		win_output_debug(outs);
	}
	return;	
}

pub fn log_output_function(level :i64, outs :&str) {
	return log_output_function_inner(level,outs);
}

#[macro_export]
macro_rules! debug_error {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}]",file!(),line!());
		c.push_str("<ERROR> ");
		c.push_str(&log_get_timestamp());
		c.push_str(":");
		c.push_str(&(format!($($arg)+)[..]));
		c.push_str("\n");
		log_output_function(1, &c);
	}
}

#[macro_export]
macro_rules! debug_warn {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}]",file!(),line!());
		c.push_str("<WARN> ");
		c.push_str(&log_get_timestamp());
		c.push_str(":");
		c.push_str(&(format!($($arg)+)[..]));
		c.push_str("\n");
		log_output_function(2, &c);
	}
}


#[macro_export]
macro_rules! debug_info {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}]",file!(),line!());
		c.push_str("<INFO> ");
		c.push_str(&log_get_timestamp());
		c.push_str(":");
		c.push_str(&(format!($($arg)+)[..]));
		c.push_str("\n");
		log_output_function(3, &c);
	}
}


#[macro_export]
macro_rules! debug_trace {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}]",file!(),line!());
		c.push_str("<TRACE> ");
		c.push_str(&log_get_timestamp());
		c.push_str(":");
		c.push_str(&(format!($($arg)+)[..]));
		c.push_str("\n");
		log_output_function(4, &c);
	}
}
