
use std::error::Error;
use syn;

mod errors;
mod logger;

use crate::*;
use crate::logger::*;

struct ImplAttrs {
	fnames :Vec<String>,
	args :HashMap<String,Vec<Vec<String>>>,
}

impl syn::parse::Parse for ImplAttrs {
	#[allow(unused_assignments)]
	fn parse(input : syn::parse::ParseStream) -> syn::parse::Result<Self> {
		let retv :ImplAttrs = ImplAttrs {
			fnames :vec![],
			args :vec![],
		};
		return Ok(retv);
	}
}

#[proc_macro_attribute]
pub fn impl_fn(_args :TokenStream , input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	let nargs = _args.clone();
	let attrs  = syn::parse_macro_input!(nargs as FuncAttrs);
	em_log_trace!("attrs [{:?}]",attrs);

	/**/
	code.push_str(&attrs.format_code());
	code.push_str(&(input.to_string()));
	em_log_trace!("code \n{}",code);	
	code.parse().unwrap()
}
