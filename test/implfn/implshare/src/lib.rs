
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

macro_rules! syn_error_fmt {
	($($a:expr),*) => {
		let cerr = format!($($a),*);
		eprintln!("{}",cerr);
		implfn_log_error!("{}",cerr);
		return cerr.parse().unwrap();
		//return syn::Error::new(
        //            Span::call_site(),
        //            $cerr,
        //        ).to_compile_error().to_string().parse().unwrap();
    }
}


#[proc_macro_attribute]
pub fn impl_share_fn(_args :TokenStream , input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	let nargs = _args.clone();
	let attrs  = syn::parse_macro_input!(nargs as ImplAttrs);
	let co :syn::ItemImpl;
	implfn_log_trace!("attrs [{:?}]",attrs);

	match syn::parse::<syn::ItemImpl>(input.clone()) {
		Ok(v) => {
			co = v.clone();
		},
		Err(e) => {
			syn_error_fmt!("can not parse {:?}",e);
		}
	}

	for v in co.items {
		match v {
			syn::ImplItem::Fn(fnitem) => {
				match fnitem.vis {
					syn::Visibility::Public(v) =>  {
						implfn_log_trace!("");
					},
					syn::Visibility::Restricted(rv) => {

					},
					syn::Visibility::Inherited => {

					},
				}


			},
			_ => {},
		}
	}




	/**/
	code.push_str(&(input.to_string()));
	code.push_str(&attrs.format_code());
	implfn_log_trace!("code \n{}",code);	
	code.parse().unwrap()
}
