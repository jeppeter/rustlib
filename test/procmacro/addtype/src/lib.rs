extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::Span;
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
	let sp = Span::call_site();
	let src = sp.source_file();
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
	println!("call print_func_name [{}]",src.path().to_str().unwrap());

	input
}

#[proc_macro_attribute]
pub fn print_all_links(_args :TokenStream, input :TokenStream) -> TokenStream {
	{
		let c = LINK_NAMES.clone();
		let cb = c.lock().unwrap();
		let mut i:i32;
		i = 0;
		for s in cb.iter() {
			println!("[{}]{}",i, s);
			i += 1;
		}
	}
	input	
}


