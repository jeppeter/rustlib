#[allow(unused_imports)]
use std::fs::{metadata,read_dir,remove_dir,remove_file,remove_dir_all};
use std::io::{stdout,Write};
use std::path::{Path};
use std::error::Error;
use std::fmt;

#[allow(unused_macros)]
macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	impl $type {
		fn create(c :&str) -> $type {
			$type {msg : format!("{}",c)}
		}
	}

	impl fmt::Display for $type {
		fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
			write!(f,"{}",self.msg)
		}
	}

	impl Error for $type {}
	};
}

error_class!{DirError}

#[allow(unused_macros)]
macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
}

#[allow(unused_macros)]
macro_rules! debug_output {
	($($a:expr),*) => {
		let mut c :String = format!("[{}:{}]",file!(),line!());
		c.push_str(&(format!($($a),*)[..]));
		eprintln!("{}", c);
	}
}

#[allow(unused_macros)]
macro_rules! error_output {
	($($a:expr),*) => {
		let mut c :String = format!("[{}:{}]",file!(),line!());
		c.push_str(&(format!($($a),*)[..]));
		eprintln!("{}", c);
	}
}

fn new_line_string(curf :&str,totalnum :usize,lastlen :usize,notenum :usize,force :bool) -> usize {
	let mut retlast :usize = lastlen;
	let c :String;
	if (totalnum % notenum) == 0 || force {
		let mut i :usize =0;
		print!("{}",(13u8 as char));
		while i < lastlen {
			print!(" ");
			i += 1;
		}
		print!("{}",(13u8 as char));
		c = format!("[{}] {}",totalnum,curf);
		retlast = c.len();
		print!("{}",c);
		stdout().flush().unwrap();
	}

	return retlast;
}

fn rmdir_succ(_dname :&str, totalnum :usize, lastlen :usize, notenum :usize,_verbose :i32) -> Result<(usize,usize),Box<dyn Error>> {
	let mut md = metadata(_dname)?;
	let  mut rettotal:usize = totalnum;
	let mut retlast :usize = lastlen;
	if md.is_dir() {
		let path = Path::new(_dname);
		for f in read_dir(path)? {			
			match f {
				Ok(fd) => {
					match fd.path().to_str() {
						Some(d) => {
							if d != "." && d != ".." && d != _dname {
								//debug_output!("name [{}]", d);
								md = metadata(d)?;
								let c = rmdir_succ(d,rettotal,retlast,notenum,_verbose)?;
								rettotal = c.0;
								retlast = c.1;
								if md.is_dir() {
									remove_dir_all(d)?;
									rettotal += 1;
									retlast = new_line_string(d,rettotal,retlast,notenum,false);
								}						
							}
						},
						None => {

						}
					}

				},
				Err(e) => {
					new_error!{DirError,"[{}] error[{:?}]", _dname,e}
				}
			}

		}
	} else {
		remove_file(_dname)?;
		rettotal += 1;
		retlast = new_line_string(_dname,rettotal,retlast,notenum,false);
	}
	Ok((rettotal,retlast))
}

fn main() {
	let mut i :usize = 0;
	let mut totalnum :usize = 0;
	let mut lastlen :usize = 0;
    for a in std::env::args() {
    	println!("[{}]=[{}]",i,a);
    	if i > 0 {
    		match rmdir_succ(&(a[..]),totalnum,lastlen,100,0) {
    			Ok(c) => {
		    		totalnum = c.0;
		    		lastlen = c.1;
		    		new_line_string(&(a[..]),totalnum,lastlen,100,true);
    			},
    			Err(e) => {
    				error_output!("[{}]=>[{:?}]",a,e);
    			}
    		}
    	}
    	i += 1;
    }
}
