


const DEFAULT_MSG_FMT :&str = "{m}";

pub fn init_log(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let mut level :LevelFilter  = log::LevelFilter::Error;
	let mut rbuiler :RootBuilder;
	let mut cbuild :ConfigBuilder;
	let mut key :String;
	let mut sarr :Vec<String>;
	let nostderr :bool;
	let stderr =ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(DEFAULT_MSG_FMT))).target(Target::Stderr).build();

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

	cbuild = Config::builder();
	rbuiler = Root::builder();
	nostderr = ns.get_bool("log_nostderr");

	if !nostderr {
		cbuild = cbuild.appender(
			Appender::builder()
			.filter(Box::new(ThresholdFilter::new(level)))
			.build("stderr", Box::new(stderr)),
			);
		rbuiler = rbuiler.appender("stderr");		
	}

	sarr = ns.get_array("log_files");
	for wf in sarr.iter() {
		let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(DEFAULT_MSG_FMT))).build(wf)?;
		cbuild = cbuild.appender(Appender::builder().build(wf, Box::new(logfile)));
		rbuiler = rbuiler.appender(wf);
	}

	sarr = ns.get_array("log_appends");
	for wf in sarr.iter() {

	}

	let config = cbuild.build(rbuiler.build(level))?;
	let _handle = log4rs::init_config(config)?;
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