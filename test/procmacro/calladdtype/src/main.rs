extern crate addtype;

use addtype::{print_func_name,print_all_links};

#[print_func_name]
fn hello_world() {
	println!("hello world");
}

#[print_func_name]
fn get_a_reply() {
	println!("reply ok");
}


#[print_all_links]
fn main() {
	hello_world();
	get_a_reply();
	c_f();
}



#[print_func_name]
fn c_f() {
	println!("c_f");
}
