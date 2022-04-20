
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cmp::Ordering;

use rand::Rng;
use bytes::{BytesMut,BufMut};

use std::env;


use log::{error, info, trace};
use log::{LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root,RootBuilder,ConfigBuilder};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

//use std::cell::RefCell;
//use std::rc::Rc;


fn get_environ_var(envname :&str) -> String {
	match env::var(envname) {
		Ok(v) => {
			format!("{}",v)
		},
		Err(_e) => {
			String::from("")
		}
	}
}

const RAND_NAME_STRING :[u8; 62]= *b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DEFAULT_MSG_FMT :&str = "{d(%Y-%m-%d %H:%M:%S)}[{l}]{m}\n";

fn proc_log_init(prefix :&str) -> i32 {
		let mut msgfmt :String = String::from(DEFAULT_MSG_FMT);
		let mut getv :String;
		let mut retv :i32 = 0;
		let mut level :LevelFilter  = log::LevelFilter::Error;
		let mut rbuiler :RootBuilder;
		let mut cbuild :ConfigBuilder;
		let mut key :String;
		let wfile :String ;
		key = format!("{}_MSGFMT", prefix);
		getv = get_environ_var(&key);
		if getv.len() > 0 {
			msgfmt = format!("{}",getv);
		}
		let stderr =ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&msgfmt)))
        .target(Target::Stderr).build();

        key = format!("{}_LEVEL", prefix);
        getv = get_environ_var(&key);
        if getv.len() > 0 {
        	match getv.parse::<i32>() {
        		Ok(v) => {
        			retv = v;
        			println!("retv [{}]",retv);
        		},
        		Err(e) => {
        			retv = 0;
        			eprintln!("can not parse [{}] error[{}]", getv,e);
        		}
        	}
        }        

        if retv >= 40 {
        	level = log::LevelFilter::Trace;
        } else if retv >= 30 {
        	level = log::LevelFilter::Debug;
        } else if retv >= 20 {
        	level = log::LevelFilter::Info;
        } else if retv >= 10 {
        	level = log::LevelFilter::Warn;
        }

	    cbuild = Config::builder()
	        .appender(
	            Appender::builder()
	                .filter(Box::new(ThresholdFilter::new(level)))
	                .build("stderr", Box::new(stderr)),
	        );
	    rbuiler =  Root::builder().appender("stderr");

	    key = format!("{}_LOGFILE",prefix);

	    wfile = get_environ_var(&key);
	    if wfile.len() > 0 {
	    	let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(&msgfmt))).build(&wfile).unwrap();

	        cbuild = cbuild.appender(Appender::builder().build("logfile", Box::new(logfile)));
	        rbuiler = rbuiler.appender("logfile");
	    }

	    let config = cbuild.build(rbuiler.build(level)).unwrap();

	    let _handle = log4rs::init_config(config).unwrap();

		retv	
}

lazy_static! {
	static ref LINK_NAMES :Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
	static ref SET_NAME : Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("FUNC_CALL")));
	static ref CALL_LEVEL : i32 = {
		proc_log_init("CALL")
	};
}

pub (crate)  fn type_call_debug_out(level :i32, outs :String) {
	if *CALL_LEVEL >= level {
		if level <= 0 {
			error!("{}",outs);
		} else if level < 40 {
			info!("{}",outs);
		}  else {
			trace!("{}",outs);
		}
	}
	return;
}

macro_rules! call_error {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}] ",file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		type_call_debug_out(0, c);
	}
}

macro_rules! call_info {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}] ",file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		type_call_debug_out(20, c);
	}
}



macro_rules! call_trace {
	($($arg:tt)+) => {
		let mut c :String= format!("[{}:{}] ",file!(),line!());
		c.push_str(&(format!($($arg)+)[..]));
		type_call_debug_out(40, c);
	}
}



fn get_random_bytes(num :u32, basevec :&[u8]) -> String {
	let mut retm = BytesMut::with_capacity(num as usize);
	let mut rng = rand::thread_rng();
	let mut curi :usize;

	for _i in 0..num {
		curi = rng.gen_range(0..basevec.len());
		retm.put_u8(basevec[curi]);
	}
	let a = retm.freeze();
	String::from_utf8_lossy(&a).to_string()
}


//fn get_static_names() -> Rc<RefCell<Vec<String>>> {

#[proc_macro_attribute]
pub fn print_func_name(_args :TokenStream, input :TokenStream) -> TokenStream {
	let sp = Span::call_site();
	let src = sp.source_file();
	let fname = format!("{}",src.path().to_str().unwrap());
	match syn::parse(input.clone()) {
		Ok(v1) => {
			let v :syn::ItemFn = v1;
			{
				let mut cb = LINK_NAMES.lock().unwrap();
				let cs = format!("{}",fname);
				call_trace!("insert [{}]=[{}]",v.sig.ident.to_string(),cs);
				cb.insert(v.sig.ident.to_string(),cs);
			}			
		},
		Err(e) => {
			call_error!("error {}", e);
		}
	}
	call_info!("call print_func_name [{}]",fname);
	input
}

#[proc_macro_attribute]
pub fn print_all_links(_args :TokenStream, input :TokenStream) -> TokenStream {
	{
		let sp = Span::call_site();
		let src = sp.source_file();
		let fname = format!("{}",src.path().to_str().unwrap());
		let c = LINK_NAMES.clone();
		let cb = c.lock().unwrap();
		let mut codes :String = String::from("");
		let mut outs :String;

		codes += "lazy_static ! {\n";
		{
			let mut scb = SET_NAME.lock().unwrap();
			let mut funcname :String;
			funcname = "FUNC_CALL_".to_string();
			funcname += &(format!("{}",get_random_bytes(15,&RAND_NAME_STRING))[..]);
			*scb = funcname;
			codes += &(format!(" static ref {} :Vec<FuncName> = {{\n", *scb)[..]);
		}
		
		codes += "        let mut vret :Vec<FuncName> = Vec::new();\n";

		for (_k,v )in cb.iter() {
			if fname.cmp(v) == Ordering::Equal {
				codes += &(format!("        vret.push(FuncName::new(String::from(\"{}\"),{}));\n",_k,_k)[..]);
			}			
		}
		codes += "        vret\n";
		codes += "   };\n";
		codes += "}";
		outs = codes;
		outs += "\n";
		outs += &(input.to_string()[..]);
		call_info!("outs\n{}",outs);
		return outs.parse().unwrap();
	}
}

#[proc_macro]
pub fn call_list_all(input1 :TokenStream) -> TokenStream {
	let mut codes :String = "".to_string();
	let mut i :i32 = 0;
	let input = proc_macro2::TokenStream::from(input1.clone());
	let mut lastc :String = "".to_string();
	call_info!("{:?}",input1.clone());
	{
		let sc = SET_NAME.clone();
		let scb = sc.lock().unwrap();
		for v in input {		
			call_trace!("[{}]=[{:?}]",i,v);
			match v {
				proc_macro2::TokenTree::Literal(t) => {
					call_trace!("[{}]Literal [{}]",i,t.to_string());
					codes += &(format!("call_functions({},&{});\n", t.to_string(),*scb)[..]);
				},
				proc_macro2::TokenTree::Ident(t) => {
					call_trace!("[{}]Ident [{}]",i,t.to_string());
					if lastc == "&" {
						codes += &(format!("call_functions(&{},&{});\n",t.to_string(),*scb)[..]);
					} else {
						codes += &(format!("call_functions({},&{});\n",t.to_string(),*scb)[..]);	
					}
					
				},
				proc_macro2::TokenTree::Punct(t) => {
					call_trace!("[{}]Punct [{}]",i,t.to_string());
					codes = codes;
					lastc = t.to_string();
				},
				proc_macro2::TokenTree::Group(t) => {
					call_trace!("[{}]Group [{}]",i,t.to_string());
					if lastc == "&" {
						codes += &(format!("call_functions(&{},&{});\n",t.to_string(),scb)[..]);
					} else {
						codes += &(format!("call_functions({},&{});\n",t.to_string(),scb)[..]);	
					}
					
				}
			}
			i += 1;
		}

	}
	call_info!("codes\n{}",codes );
	codes.parse().unwrap()
}



