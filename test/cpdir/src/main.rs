#[allow(unused_imports)]
use std::fs::{metadata,read_dir,create_dir_all,copy,Metadata};
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

error_class!{CpdirError}

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

fn new_cpdir_string(csrcf :&str, cdstf :&str,totalnum :usize,lastlen :usize,notenum :usize,force :bool) -> usize {
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
		c = format!("[{}] [{}] => [{}]",totalnum,csrcf,cdstf);
		retlast = c.len();
		print!("{}",c);
		stdout().flush().unwrap();
	}

	return retlast;
}

fn cpdir_succ(_sname :&str,_dname :&str, totalnum :usize, lastlen :usize, notenum :usize,_verbose :i32) -> Result<(usize,usize),Box<dyn Error>> {
	let mut smd = metadata(_sname)?;
	let  mut rettotal:usize = totalnum;
	let mut retlast :usize = lastlen;
	if smd.is_dir() {
		let spath = Path::new(_sname);
		if !Path::new(_dname).exists() {
			create_dir_all(_dname)?;
			rettotal += 1;
			retlast = new_cpdir_string(_sname,_dname,rettotal,retlast,notenum,false);
		}

		for f in read_dir(spath)? {
			match f {
				Ok(fd) => {
					match fd.path().to_str() {
						Some(d) => {
							let curd :String;
							let curs :String;
							if d != "." && d != ".." && d != _sname {
								curs = format!("{}",d);
								curd = curs.replacen(_sname,_dname,1);
								//debug_output!("curs [{}] curd[{}]", curs,curd);
								smd = metadata(&(curs[..]))?;
								if  smd.is_dir() && !Path::new(&(curd[..])).exists() {
									create_dir_all(&(curd[..]))?;
									rettotal += 1;
									retlast = new_cpdir_string(&(curs[..]),&(curd[..]),rettotal,retlast,notenum,false);
								}
								let c = cpdir_succ(&(curs[..]),&(curd[..]), rettotal,retlast,notenum,_verbose)?;
								rettotal = c.0;
								retlast = c.1;
							}
						},
						None => {
							new_error!{CpdirError,"can not get [{:?}]",fd.path()}
						}
					}

				},
				Err(e) => {
					new_error!{CpdirError,"[{}] error[{:?}]", _sname,e}
				}
			}

		}
	} else {
		copy(_sname,_dname)?;
		rettotal += 1;
		retlast = new_cpdir_string(_sname,_dname,rettotal,retlast,notenum,false);
	}
	Ok((rettotal,retlast))
}


fn main() {
	let mut i :usize = 0;
	let totalnum :usize = 0;
	let lastlen :usize = 0;
	let mut srcd :String = String::from("");
	let mut dstd :String = String::from("");
    for a in std::env::args() {
    	println!("[{}]=[{}]",i,a);
    	if i == 1 {
    		srcd = format!("{}",a);
    	} else if i == 2 {
    		dstd = format!("{}",a);
    	}
    	i += 1;
    }

    if srcd.len() == 0 || dstd.len() == 0 {
    	error_output!("need srcd dstd");
    	return;
    }

    match cpdir_succ(&(srcd[..]),&(dstd[..]),totalnum,lastlen,100,0) {
    	Ok(cpv) => {
    		new_cpdir_string(&(srcd[..]),&(dstd[..]),cpv.0,cpv.1,100,true);
    	},
    	Err(e) => {
    		error_output!("cp [{}]=>[{}] error[{:?}]", srcd,dstd,e);
    	}
    }
    return;
}
