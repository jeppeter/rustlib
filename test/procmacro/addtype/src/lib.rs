extern crate proc_macro;

use proc_macro::TokenStream;
use syn;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
//use std::cell::RefCell;
//use std::rc::Rc;


lazy_static! {
	static ref LINK_NAMES :Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

//fn get_static_names() -> Rc<RefCell<Vec<String>>> {

#[proc_macro_attribute]
pub fn print_func_name(_args :TokenStream, input :TokenStream) -> TokenStream {
	match syn::parse(input.clone()) {
		Ok(v1) => {
			let v :syn::ItemFn = v1;
			{
				let c = LINK_NAMES.clone();
				let mut cb = c.lock().unwrap();
				cb.push(v.sig.ident.to_string());
			}
			
		},
		Err(e) => {
			eprintln!("error {}", e);
		}
	}
	println!("call print_func_name");

	input
}


