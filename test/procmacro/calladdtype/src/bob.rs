extern crate addtype;
extern crate funccall;

use addtype::{print_func_name,print_all_links,call_list_all};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;


#[print_func_name]
fn hello_world_2() {
	println!("hello world_2");
}

#[print_func_name]
fn get_a_reply_2() {
	println!("reply ok_2");
}


pub fn bob_func() {
	let bcc = "hello_world_2";
	call_list_all!("hello_world_2",&(cc[..]),&(String::from("get_a_reply_2")[..]));
	call_list_all!("hello_world");
	call_list_all!(bcc);
	return;
}