extern crate addtype;
extern crate funccall;

use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
use std::fmt;
#[allow(unused_imports)]
use std::collections::HashMap;


mod bob;

use addtype::{print_func_name,print_all_links,call_list_all,ArgSet};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;



macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	#[allow(dead_code)]
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

macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
}


#[derive(Clone)]
pub struct NameSpaceEx {

}

impl NameSpaceEx {
	pub fn new() -> NameSpaceEx {
		NameSpaceEx{}
	}
	pub fn get_bool(&self, _k :&str) -> bool {
		return false;
	}
	pub fn get_int(&self,_k :&str) -> i64 {
		return 0;
	}
	pub fn get_float(&self,_k :&str) -> f64 {
		return 0.0;
	}
	pub fn get_array(&self,_k :&str) -> Vec<String> {
		return Vec::new();
	}
	pub fn get_string(&self,_k :&str) -> String {
		return "".to_string();
	}
}


pub trait ArgSetImpl {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
	fn new() -> Self;
}


#[print_func_name]
fn hello_world() -> String {
	println!("hello world");
	String::from("hello_world")
}

#[print_func_name]
fn get_a_reply() -> String {
	println!("reply ok");
	String::from("reply ok")
}

/*
lazy_static !{
	static ref FUNC_CALL :Vec<FuncName> = {
		let mut vret:Vec<FuncName> = Vec::new();
		vret.push(FuncName::new(
			String::from("hello_world"),
			hello_world,
		));
		vret
	};
}*/


#[derive(ArgSet,Debug)]
pub struct CCFunc {
	aval :f64,
	bval :f64,
	cval :Vec<String>,
}

// impl ArgSet for CCFunc {
// 	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
// 		Ok(())
// 	}
// 	fn new() -> Self {
// 		CCFunc {
// 			aval : 0.0,
// 			bval : 0.0,
// 			cval : Vec::new(),
// 		}
// 	}
// }

#[derive(ArgSet,Debug)]
pub struct BBFunc {
	csub :CCFunc,
	bbx : HashMap<String,String>,
	xstr :String,
	bval : bool,
	ii : i32,
	ui : u32,
	ii6 : i64,
	ui6 : u64,
	fi : f32,
	fi6 :f64,
}

// impl ArgSet for BBFunc {
// 	fn new() -> Self {
// 		BBFunc{
// 			csub : CCFunc::new(),
// 			xstr : "".to_string(),
// 			bval : false,
// 			ii : 0,
// 			ui : 0,
// 			ii6 : 0,
// 			ui6 : 0,
// 			fi : 0.0,
// 			fi6 : 0.0,
// 		}
// 	}
// 	fn set_value(&mut self,k :&str, _ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
// 		println!("{} set", k);
// 		Ok(())
// 	}
// }


// const _ :fn() = || {
// 	fn  assert_impl_all<T : ?Sized + ArgSet>() {}
// 	assert_impl_all::<CCFunc>();
// };

// const _ :fn() = || {
// 	fn  assert_impl_all<T : ?Sized + ArgSet>() {}
// 	assert_impl_all::<CCFunc>();
// };


// const _ :fn() = || {
// 	fn  assert_impl_all<T : ?Sized + ArgSet>() {}
// 	assert_impl_all::<BBFunc>();
// };




fn call_arg_set<T : ArgSetImpl>(cv :&mut T,ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	cv.set_value("csub.cval",ns.clone())?;
	Ok(())
}


#[print_all_links]
fn main() {
	let cc = String::from("hello_world");
	let scc = &(String::from("get_a_reply")[..]);
	let bcc = "hello_world";
	let mut cv = BBFunc::new();
	let ns = NameSpaceEx::new();
	call_list_all!("hello_world",&(cc[..]),&(String::from("get_a_repl")[..]));
	call_list_all!("hello_world");
	call_list_all!(bcc);
	call_list_all!();
	bob::bob_func();
	call_arg_set(&mut cv,ns).unwrap();
	println!("cv [{:?}]",cv);
	return;
}



#[allow(dead_code)]
#[print_func_name]
fn c_f() {
	println!("c_f");
}
