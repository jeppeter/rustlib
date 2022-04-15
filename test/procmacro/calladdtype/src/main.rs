extern crate addtype;
extern crate funccall;

use addtype::{print_func_name,print_all_links,call_list_all};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;

#[print_func_name]
fn hello_world() {
	println!("hello world");
}

#[print_func_name]
fn get_a_reply() {
	println!("reply ok");
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

#[print_all_links]
fn main() {
	call_list_all!("hello_world","get_a_reply");
	call_list_all!("hello_world");
	call_list_all!(cc);
	call_list_all!();
	call_functions("hello_world", &FUNC_CALL);
}



#[allow(dead_code)]
#[print_func_name]
fn c_f() {
	println!("c_f");
}
