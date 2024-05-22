
#[allow(unused_imports)]
use std::error::Error;
use syn;
use std::collections::HashMap;
use proc_macro::TokenStream;

#[macro_use]
mod errors;
#[macro_use]
mod logger;

use crate::logger::*;

implfn_error_class!{ImplFnError}

#[derive(Debug)]
#[allow(dead_code)]
struct ImplAttrs {
	fnames :Vec<String>,
	args :HashMap<String,Vec<Vec<String>>>,
}

impl syn::parse::Parse for ImplAttrs {
	#[allow(unused_assignments)]
	fn parse(_input : syn::parse::ParseStream) -> syn::parse::Result<Self> {
		let retv :ImplAttrs = ImplAttrs {
			fnames :vec![],
			args :HashMap::new(),
		};
		return Ok(retv);
	}
}

impl ImplAttrs {
	fn format_code(&self) -> String {
		"".to_string()
	}
}

#[proc_macro_attribute]
pub fn impl_fn(_args :TokenStream , input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	let nargs = _args.clone();
	let attrs  = syn::parse_macro_input!(nargs as ImplAttrs);
	implfn_log_trace!("attrs [{:?}]",attrs);


	/**/
	code.push_str(&(input.to_string()));
	code.push_str(&attrs.format_code());
	implfn_log_trace!("code \n{}",code);	
	code.parse().unwrap()
}
