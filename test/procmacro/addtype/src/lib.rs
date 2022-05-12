
use proc_macro::TokenStream;
//use proc_macro2::Span;
use syn;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cmp::Ordering;

use rand::Rng;
use bytes::{BytesMut,BufMut};


use std::fmt::{Debug};
use std::fmt;
use std::error::Error;
use std::boxed::Box;

mod consts;
#[macro_use]
mod errors;
#[macro_use]
mod logger;
mod util;

use consts::{KEYWORD_U64,KEYWORD_I64,KEYWORD_F64,KEYWORD_U32,KEYWORD_I32,KEYWORD_F32,KEYWORD_TYPE_STRING,KEYWORD_TYPE_BOOL,KEYWORD_VEC_STRING,KEYWORD_LEFT_ARROW};
use logger::{em_debug_out};
use util::{check_in_array};


//use std::cell::RefCell;
//use std::rc::Rc;



const RAND_NAME_STRING :[u8; 62]= *b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";



lazy_static! {
	static ref LINK_NAMES :Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
	static ref SET_NAME : Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("FUNC_CALL")));

	static ref ARGSET_KEYWORDS :Vec<String> = {
		let mut retv :Vec<String> = Vec::new();
		retv.push(format!("{}",KEYWORD_U64));
		retv.push(format!("{}",KEYWORD_I64));
		retv.push(format!("{}",KEYWORD_F64));
		retv.push(format!("{}",KEYWORD_U32));
		retv.push(format!("{}",KEYWORD_I32));
		retv.push(format!("{}",KEYWORD_F32));
		retv.push(format!("{}",KEYWORD_TYPE_STRING));
		retv.push(format!("{}",KEYWORD_TYPE_BOOL));
		retv.push(format!("{}",KEYWORD_VEC_STRING));
		retv
	};
}


fn get_random_bytes(num :u32, basevec :&[u8]) -> String {
	let mut retm = BytesMut::with_capacity(num as usize);
	let mut rng = rand::thread_rng();
	let mut curi :usize;

	for _i in 0..num {
		curi = rng.gen_range(0..basevec.len());
		retm.put_u8(basevec[curi]);
	}
	let a = retm.freeze();
	String::from_utf8_lossy(&a).to_string()
}




#[proc_macro_attribute]
pub fn print_all_links(_args :TokenStream, input :TokenStream) -> TokenStream {
	{
		//let sp = Span::call_site();
		//let src = sp.source_file();
		let fname = format!("xxx");
		//let fname = format!("{}",src.path().to_str().unwrap());
		let c = LINK_NAMES.clone();
		let cb = c.lock().unwrap();
		let mut codes :String = String::from("");
		let mut outs :String;

		codes += "lazy_static ! {\n";
		{
			let mut scb = SET_NAME.lock().unwrap();
			let mut funcname :String;
			funcname = "FUNC_CALL_".to_string();
			funcname += &(format!("{}",get_random_bytes(15,&RAND_NAME_STRING))[..]);
			*scb = funcname;
			codes += &(format!(" static ref {} :Vec<FuncName> = {{\n", *scb)[..]);
		}

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
		em_log_info!("outs\n{}",outs);
		return outs.parse().unwrap();
	}
}

macro_rules! syn_error_fmt {
	($($a:expr),*) => {
		let cerr = format!($($a),*);
		eprintln!("{}",cerr);
		em_log_error!("{}",cerr);
		return cerr.parse().unwrap();
		//return syn::Error::new(
        //            Span::call_site(),
        //            $cerr,
        //        ).to_compile_error().to_string().parse().unwrap();
	}
}


#[proc_macro_attribute]
pub fn extargs_map_function(_args :TokenStream , input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	em_log_trace!("args [{:?}]",_args);	
	for v in _args.clone() {
		em_log_trace!("v [{:?}]",v);
	}

	code.push_str(&(input.to_string()));
	code.parse().unwrap()
}


error_class!{TypeError}

fn get_name_type(n : syn::Field) -> Result<(String,String), Box<dyn Error>> {
	let name :String ;
	let mut typename :String = "".to_string();
	match n.ident {
		Some(ref _i) => {
			name = format!("{}",_i);
		},
		None => {
			new_error!{TypeError,"can not get"}
		}
	}

	match n.ty {
		syn::Type::Path(ref _p) => {
			let mut pidx :i32 = 0;
			if _p.path.leading_colon.is_some() {
				typename.push_str("::");
			}
			for _s in _p.path.segments.iter() {
				if pidx > 0 {
					typename.push_str("::");
				}
				typename.push_str(&(format!("{}",_s.ident)));
				//em_log_trace!("f [{}]",typename);
				match _s.arguments {
					syn::PathArguments::None => {},
					syn::PathArguments::AngleBracketed(ref _an) => {
						typename.push_str("<");
						let mut idx :i32 = 0;
						for _ii in _an.args.iter() {
							match _ii {
								syn::GenericArgument::Type(ref _pi) => {
									match _pi {
										syn::Type::Path(ref _pt) => {
											let mut jdx : i32 = 0;
											if idx > 0 {
												typename.push_str(",");
											}
											for _tt in _pt.path.segments.iter() {
												if jdx > 0 {
													typename.push_str("::");
												}
												typename.push_str(&(format!("{}", _tt.ident)));
												jdx += 1;
											}
										},
										_ => { new_error!{TypeError, "not "}}
									}
								},
								_ => {
									new_error!{TypeError,"no args type"}
								}
							}
							idx += 1;
						}
						typename.push_str(">");
					},
					syn::PathArguments::Parenthesized(ref _pn) => {
						new_error!{TypeError,"Parenthesized"}
					}
				}
				pidx += 1;
			}
		},
		_ => {
			new_error!{TypeError,"ty not support for"}
		}
	}
	em_log_trace!("name [{}] typename [{}]",name,typename);
	Ok((name,typename))
}

fn format_code(ident :&str,names :HashMap<String,String>, structnames :Vec<String>) -> String {
	let mut rets :String = "".to_string();
	let mut typeerrname :String = format!("{}_typeerror",ident);
	if structnames.len() > 0 {
		for i in structnames.clone() {
			/*to make the type check for ArgSetImpl*/
			rets.push_str(&format!("const _ :fn() = || {{\n"));
			rets.push_str(&format!("    fn assert_impl_all<T : ?Sized + ArgSetImpl>() {{}}\n"));
			rets.push_str(&format!("    assert_impl_all::<{}>();\n", i));
			rets.push_str(&format!("}};\n"));
		}
	}

	typeerrname.push_str("_");
	typeerrname.push_str(&(get_random_bytes(15,&RAND_NAME_STRING)));



	rets.push_str(&format!("error_class!{{{}}}\n",typeerrname));

	rets.push_str(&format!("impl ArgSetImpl for {} {{\n",ident));
	rets.push_str(&format!("    fn new() -> Self {{\n"));
	rets.push_str(&format!("        {} {{\n",ident));
	for (k,v) in names.clone().iter() {
		rets.push_str(&format!("            "));
		if v == KEYWORD_TYPE_STRING {
			rets.push_str(&format!("{} : \"\".to_string(),\n", k));
		} else if v == KEYWORD_U32 || v == KEYWORD_I32 || v == KEYWORD_U64 || v == KEYWORD_I64 {
			rets.push_str(&format!("{} : 0,\n",k));
		} else if v == KEYWORD_F32  || v == KEYWORD_F64 {
			rets.push_str(&format!("{} : 0.0,\n",k));
		} else if v == KEYWORD_VEC_STRING {
			rets.push_str(&format!("{} : Vec::new(),\n",k));
		} else if v == KEYWORD_TYPE_BOOL {
			rets.push_str(&format!("{} : false,\n",k));
		}else  {
			/*to make new type*/
			rets.push_str(&format!("{} : {}::new(),\n",k,v));
		}
	}
	rets.push_str(&format!("        }}\n"));
	rets.push_str(&format!("    }}\n"));


	rets.push_str(&format!("    \n"));

	rets.push_str(&format!("    fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {{\n"));
	let mut i :i32 = 0;
	for (k,v) in names.clone().iter() {
		if !check_in_array(ARGSET_KEYWORDS.clone(), v) {
			continue;
		}

		rets.push_str(&format!("        "));
		if i > 0 {
			rets.push_str(&format!("}} else if "));
		} else {
			rets.push_str(&format!("if "));
		}
		rets.push_str(&format!("k == \"{}\" {{\n", k));
		rets.push_str(&format!("            "));
		if v == KEYWORD_TYPE_STRING {
			rets.push_str(&format!("self.{} = ns.get_string(k);\n", k));
		} else if v == KEYWORD_I32 {
			rets.push_str(&format!("self.{} = ns.get_int(k) as i32;\n",k));
		} else if v == KEYWORD_U32 {
			rets.push_str(&format!("self.{} = ns.get_int(k) as u32;\n",k));
		} else if v == KEYWORD_F32 {
			rets.push_str(&format!("self.{} = ns.get_float(k) as f32;\n",k));
		} else if v == KEYWORD_I64 {
			rets.push_str(&format!("self.{} = ns.get_int(k);\n",k));
		} else if v == KEYWORD_U64 {
			rets.push_str(&format!("self.{} = ns.get_int(k) as u64;\n",k));
		} else if v == KEYWORD_F64 {
			rets.push_str(&format!("self.{} = ns.get_float(k);\n",k));
		} else if v == KEYWORD_TYPE_BOOL {
			rets.push_str(&format!("self.{} = ns.get_bool(k);\n",k));
		} else if v == KEYWORD_VEC_STRING {
			rets.push_str(&format!("self.{} = ns.get_array(k);\n",k));
		} 
		i += 1;
	}

	if structnames.len() > 0 {
		for s in structnames.clone() {
			for (k,v) in names.clone().iter() {
				if s.eq(v) {
					rets.push_str(&format!("        "));
					if i > 0 {
						rets.push_str(&format!("}} else if "));
					} else {
						rets.push_str(&format!("if "));
					}
					rets.push_str(&format!("k.starts_with(&format!(\"{}.\")) {{\n",k));
					rets.push_str(&format!("            let nk = format!(\"{{}}\",k);\n"));
					rets.push_str(&format!("            let re = Regex::new(r\"^{}\\.\").unwrap();\n",k));
					rets.push_str(&format!("            let kn = re.replace_all(&nk,\"\").to_string();\n"));
					rets.push_str(&format!("            self.{}.set_value(&kn,ns.clone())?;\n",k));
					break;
				}
			}
			i += 1;
		}
	}


	if i > 0 {
		rets.push_str(&format!("        }} else {{\n"));
		rets.push_str(&format!("            new_error!{{ {},\"{{}} not valid\" , k}}\n", typeerrname));
		rets.push_str(&format!("        }}\n"));
	}
	rets.push_str(&format!("        Ok(())\n"));

	rets.push_str(&format!("    }}\n"));

	rets.push_str(&format!("}}\n"));

	rets
}


#[proc_macro_derive(ArgSet)]
pub fn argset_impl(item :TokenStream) -> TokenStream {
	em_log_trace!("item\n{}",item.to_string());
	let co :syn::DeriveInput;
	let sname :String;
	let mut names :HashMap<String,String> = HashMap::new();
	let mut structnames :Vec<String> = Vec::new();

	match syn::parse::<syn::DeriveInput>(item.clone()) {
		Ok(v) => {
			co = v.clone();
		},
		Err(_e) => {
			syn_error_fmt!("not parse \n{}",item.to_string());
          	//return syn::Error::new(
            //        Span::call_site(),
            //        &format!(
            //            "not parse \n{}",
            //            item.to_string()
            //        ),
            //    ).to_compile_error().to_string().parse().unwrap();

		}
	}

	sname = format!("{}",co.ident);
	em_log_trace!("sname [{}]",sname);


	match co.data {
		syn::Data::Struct(ref _vv) => {
			match _vv.fields {
				syn::Fields::Named(ref _n) => {
					for _v in _n.named.iter() {
						let res = get_name_type(_v.clone());
						if res.is_err() {
							syn_error_fmt!("{:?}",res.err().unwrap());
						}
						let (n,tn) = res.unwrap();
						if tn.contains(KEYWORD_LEFT_ARROW) && tn != KEYWORD_VEC_STRING {
							syn_error_fmt!("tn [{}] not valid",tn);
						}
						if names.get(&n).is_some() {
							syn_error_fmt!("n [{}] has already in",n);
						}

						if !check_in_array(ARGSET_KEYWORDS.clone(),&tn) {
							em_log_trace!("input typename [{}]",tn);
							structnames.push(format!("{}",tn));
						}

						names.insert(format!("{}",n),format!("{}",tn));
					}
				},
				_ => {
					syn_error_fmt!("not Named structure\n{}",item.to_string());
				}
			}
		},
		_ => {
			syn_error_fmt!("not struct format\n{}",item.to_string());
		}
	}

	/*now to compile ok*/
	let cc = format_code(&sname,names.clone(),structnames.clone());
	em_log_trace!("cc\n{}",cc);

	cc.parse().unwrap()
}


