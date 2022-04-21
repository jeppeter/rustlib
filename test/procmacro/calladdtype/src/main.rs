extern crate addtype;
extern crate funccall;

mod bob;

use addtype::{print_func_name,print_all_links,call_list_all};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;

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

#[print_all_links]
fn main() {
	let cc = String::from("hello_world");
	let scc = &(String::from("get_a_reply")[..]);
	let bcc = "hello_world";
	call_list_all!("hello_world",&(cc[..]),&(String::from("get_a_repl")[..]));
	call_list_all!("hello_world");
	call_list_all!(bcc);
	call_list_all!();
	bob::bob_func();
	return;
}



#[allow(dead_code)]
#[print_func_name]
fn c_f() {
	println!("c_f");
}
