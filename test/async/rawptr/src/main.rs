
use lazy_static::lazy_static;
use chrono::{Local,Timelike,Datelike};
use std::io::{Write};


#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}


#[allow(dead_code)]
struct LogVar {
	level :i32,
	nostderr : bool,
	wfile : Option<std::fs::File>,
	wfilename :String,
	baklevel :i32,
	baknostderr :bool,
}


fn _get_environ_var(envname :&str) -> String {
	match std::env::var(envname) {
		Ok(v) => {
			format!("{}",v)
		},
		Err(_e) => {
			String::from("")
		}
	}
}


fn proc_log_init(prefix :&str) -> LogVar {
	let mut getv :String;
	let mut retv :i32 = 0;
	let mut nostderr :bool = false;
	let mut coptfile :Option<std::fs::File> = None;
	let mut key :String;
	let mut fname :String = "".to_string();

	key = format!("{}_LEVEL", prefix);
	getv = _get_environ_var(&key);
	if getv.len() > 0 {
		match getv.parse::<i32>() {
			Ok(v) => {
				retv = v;
			},
			Err(e) => {
				retv = 0;
				eprintln!("can not parse [{}] error[{}]", getv,e);
			}
		}
	}

	key = format!("{}_NOSTDERR",prefix);
	getv = _get_environ_var(&key);
	if getv.len() > 0 {
		nostderr = true;
	}



	key = format!("{}_LOGFILE",prefix);
	getv = _get_environ_var(&key);
	if getv.len() > 0 {
		fname = format!("{}",getv);
		let fo = std::fs::File::create(&getv);
		if fo.is_err() {
			eprintln!("can not open [{}]", getv);
		} else {
			coptfile = Some(fo.unwrap());
		}
	}

	return LogVar {
		level : retv,
		nostderr : nostderr,
		wfile : coptfile,
		wfilename : fname,
		baklevel : 0,
		baknostderr : true,
	};
}


lazy_static! {
	static ref LOG_LEVEL : std::sync::RwLock<LogVar> = {
	 	std::sync::RwLock::new(proc_log_init("RAWPTR"))
	};
}


#[allow(dead_code)]
pub (crate)  fn debug_out(level :i32, outs :&str) {
	let refecsimple = LOG_LEVEL.write().unwrap();
	if refecsimple.level >= level {
		let c = format!("{}\n",outs);
		if !refecsimple.nostderr {
			let _ = std::io::stderr().write_all(c.as_bytes());
		}

		if refecsimple.wfile.is_some() {
			let mut wf = refecsimple.wfile.as_ref().unwrap();
			let _ = wf.write(c.as_bytes());
		}
	}
	return;
}


pub (crate) fn log_get_timestamp() -> String {
	let now = Local::now();
	return format!("{}/{}/{} {}:{}:{}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
}


#[macro_export]
macro_rules! format_buffer_log {
	($buf:expr,$len:expr,$info:tt,$iv:expr,$($arg:tt)+) => {
		let mut c :String = format!("[RAWPTR][{}:{}]",file!(),line!());
		c.push_str(&format!("{} ",$info));
		c.push_str(&log_get_timestamp());
		c.push_str(": ");
		c.push_str(&(format!($($arg)+)[..]));
		let _ptr :*const u8 = $buf as *const u8;
		let  mut _ci :usize;
		let _totallen: usize = $len as usize;
		let mut _lasti :usize = 0;
		let mut _nb :u8;
		c.push_str(&format!(" buffer [{:?}][{}]",_ptr,_totallen));
		_ci = 0;
		while _ci < _totallen {
			if (_ci % 16) == 0 {
				if _ci > 0 {
					c.push_str("    ");
					while _lasti < _ci {
						unsafe{
							_nb = *_ptr.offset(_lasti as isize);	
						}
						
						if _nb >= 0x20 && _nb <= 0x7e {
							c.push(_nb as char);
						} else {
							c.push_str(".");
						}
						_lasti += 1;
					}
				}
				c.push_str(&format!("\n0x{:08x}:", _ci));
			}
			unsafe {_nb = *_ptr.offset(_ci as isize);}			
			c.push_str(&format!(" 0x{:02x}",_nb));
			_ci += 1;
		}

		if _lasti < _ci {
			while (_ci % 16) != 0 {
				c.push_str("     ");
				_ci += 1;
			}

			c.push_str("    ");

			while _lasti < _totallen {
				unsafe {_nb = *_ptr.offset(_lasti as isize);}				
				if _nb >= 0x20 && _nb <= 0x7e {
					c.push(_nb as char);
				} else {
					c.push_str(".");
				}
				_lasti += 1;
			}
			//c.push_str("\n");
		}
		debug_out($iv,&c);
	}
}

#[macro_export]
macro_rules! debug_buffer_error {
	($buf:expr,$len:expr,$($arg:tt)+) => {
		format_buffer_log!($buf,$len,"<ERROR>",0,$($arg)+);
	}
}

fn main() {
    let mut test1 = Test::new("test111");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {} {:p}", test1.a(), test1.b(),test1.b());
    println!("a: {}, b: {} {:p}", test2.a(), test2.b(),test2.b());
    {
	    let ptra :*const Test = &test1;
	    debug_buffer_error!(ptra,std::mem::size_of::<Test>(),"test1");
	    let ptrb :*const Test = &test2;
	    debug_buffer_error!(ptrb,std::mem::size_of::<Test>(),"test2");
	    let stra :*const String = &test1.a;
	    debug_buffer_error!(stra,std::mem::size_of::<String>(),"test1.a");
	    let strb :*const String = &test2.a;
	    debug_buffer_error!(strb,std::mem::size_of::<String>(),"test2.a");
    }

    std::mem::swap(&mut test1, &mut test2);

    {
	    let ptra :*const Test = &test1;
	    debug_buffer_error!(ptra,std::mem::size_of::<Test>(),"test1");
	    let ptrb :*const Test = &test2;
	    debug_buffer_error!(ptrb,std::mem::size_of::<Test>(),"test2");
	    let stra :*const String = &test1.a;
	    debug_buffer_error!(stra,std::mem::size_of::<String>(),"test1.a");
	    let strb :*const String = &test2.a;
	    debug_buffer_error!(strb,std::mem::size_of::<String>(),"test2.a");
    }

    println!("a: {}, b: {} {:p}", test1.a(), test1.b(),test1.b());
    println!("a: {}, b: {} {:p}", test2.a(), test2.b(),test2.b());
}
