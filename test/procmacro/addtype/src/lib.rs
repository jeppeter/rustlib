extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
fn print_func_name(args :TokenStream, input :TokenStream) -> TokenStream {
	println!("call print_func_name");
	input
}

#[print_func_name]
fn hello_world() {
	println!("hello world");
}

