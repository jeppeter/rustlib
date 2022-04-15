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
	static ref SET_NAME : String = String::from("FUNC_CALL");
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
		let mut funcname :String;
		funcname = "FUNC_CALL_";

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
pub fn call_list_all(input1 :TokenStream) -> TokenStream {
	let mut codes :String = "".to_string();
	let mut i :i32 = 0;
	let input = proc_macro2::TokenStream::from(input1.clone());
	let mut lastc :String = "".to_string();
	//println!("{:?}",input1.clone());
	for v in input {		
		//println!("[{}]=[{:?}]",i,v);
		match v {
			proc_macro2::TokenTree::Literal(t) => {
				//println!("[{}]Literal [{}]",i,t.to_string());
				codes += &(format!("call_functions({},&{});\n", t.to_string(),*SET_NAME)[..]);
			},
			proc_macro2::TokenTree::Ident(t) => {
				println!("[{}]Ident [{}]",i,t.to_string());
				if lastc == "&" {
					codes += &(format!("call_functions(&{},&{});\n",t.to_string(),*SET_NAME)[..]);
				} else {
					codes += &(format!("call_functions({},&{});\n",t.to_string(),*SET_NAME)[..]);	
				}
				
			},
			proc_macro2::TokenTree::Punct(t) => {
				println!("[{}]Punct [{}]",i,t.to_string());
				codes = codes;
				lastc = t.to_string();
			},
			proc_macro2::TokenTree::Group(t) => {
				println!("[{}]Group [{}]",i,t.to_string());
				if lastc == "&" {
					codes += &(format!("call_functions(&{},&{});\n",t.to_string(),*SET_NAME)[..]);
				} else {
					codes += &(format!("call_functions({},&{});\n",t.to_string(),*SET_NAME)[..]);	
				}
				
			},
			_ => {
				println!("[{}]v [{:?}]",i,v);
			}
		}
		i += 1;
	}
	println!("codes\n{}",codes );
	codes.parse().unwrap()
}



