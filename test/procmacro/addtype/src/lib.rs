extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cmp::Ordering;
//use std::cell::RefCell;
//use std::rc::Rc;


lazy_static! {
	static ref LINK_NAMES :Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
	static ref SET_NAME : String = String::from("");
}

//fn get_static_names() -> Rc<RefCell<Vec<String>>> {

#[proc_macro_attribute]
pub fn print_func_name(_args :TokenStream, input :TokenStream) -> TokenStream {
	let sp = Span::call_site();
	let src = sp.source_file();
	let fname = format!("{}",src.path().to_str().unwrap());
	match syn::parse(input.clone()) {
		Ok(v1) => {
			let v :syn::ItemFn = v1;
			{
				let mut cb = LINK_NAMES.lock().unwrap();
				let cs = format!("{}",fname);
				cb.insert(v.sig.ident.to_string(),cs);
			}			
		},
		Err(e) => {
			eprintln!("error {}", e);
		}
	}
	println!("call print_func_name [{}]",fname);

	input
}

#[proc_macro_attribute]
pub fn print_all_links(_args :TokenStream, input :TokenStream) -> TokenStream {
	{
		let sp = Span::call_site();
		let src = sp.source_file();
		let fname = format!("{}",src.path().to_str().unwrap());
		let c = LINK_NAMES.clone();
		let cb = c.lock().unwrap();
		let mut codes :String = String::from("");
		let mut outs :String;
		codes += "lazy_static ! {\n";
		codes += " static ref FUNC_CALL :Vec<FuncName> = {\n";
		codes += "        let mut vret :Vec<FuncName> = Vec::new();\n";

		for (_k,v )in cb.iter() {
			if fname.cmp(v) == Ordering::Equal {
				codes += &(format!("        vret.push(FuncName::new(String::from(\"{}\"),{}));\n",_k,_k)[..]);
			}			
		}
		codes += "        vret\n";
		codes += "   };\n";
		codes += "}";
		outs = codes;
		outs += "\n";
		outs += &(input.to_string()[..]);
		println!("outs\n{}",outs);
		return outs.parse().unwrap();
	}
}

#[proc_macro]
pub fn call_list_all(input :TokenStream) -> TokenStream {
	let mut codes :String = "".to_string();
	println!("{:?}",input);

	codes.parse().unwrap()
}



