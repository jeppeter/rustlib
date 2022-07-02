#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;
use pkcs7::{ContentInfo};
use der::{Decode};

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use super::pelib::{get_securtiy_buffer,SecData};
use super::fileop::{write_file,read_file_bytes};

use std::str::FromStr;
use std::ops::Shr;
use num_bigint::{BigUint};
use num_traits::{Zero};


extargs_error_class!{PeHdlError}

fn pesecdata_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let mut secdata :SecData;
	let sarr :Vec<String>;
	//let mut lastidx :usize;
	let mut idx :usize = 0;
	let fname :String;


	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	fname = ns.get_string("output");
	for f in sarr.iter() {
		secdata = get_securtiy_buffer(f)?;
		let dlen :usize;
		let curname :String;

		if (secdata.buf.len() as u32) < secdata.size {
			dlen = secdata.buf.len() ;
		} else {
			dlen = secdata.size as usize;
		}

		if fname.len() == 0 {
			curname = "".to_string();
		} else {
			curname = format!("{}.{}",fname,idx);
		}

		debug_buffer_trace!(secdata.buf.as_ptr(), dlen, "{} security data [0x{:08x}].[0x{:08x}][0x{:08x}]:",f,secdata.virtaddr,secdata.size,secdata.buf.len());
		debug_trace!("write curname [{}]",curname);
		write_file(&curname,&(secdata.buf[..dlen]))?;
		/*
		print!("{} security data [0x{:08x}].[0x{:08x}][0x{:08x}]:",f,secdata.virtaddr,secdata.size,secdata.buf.len());
		lastidx = 0;
		idx = 0;
		for b in secdata.buf.iter() {
			if idx >= secdata.size as usize {
				break;
			}
			if (idx % 16) == 0 {
				if idx > 0 {
					print!("    ");
					while lastidx != idx {
						if secdata.buf[lastidx] >= 0x20 && secdata.buf[lastidx] <= 0x7e {
							print!("{}", secdata.buf[lastidx] as char );
						} else {
							print!(".");
						}
						lastidx += 1;
					}
				}
				print!("\n0x{:08x}",idx);
			}
			print!(" 0x{:02x}",  b);
			idx += 1;
		}

		if idx != lastidx {
			while (idx % 16) != 0 {
				print!("     ");
				idx += 1;
			}

			print!("    ");
			while lastidx < secdata.buf.len() && lastidx < secdata.size as usize {
				if secdata.buf[lastidx] >= 0x20 && secdata.buf[lastidx] <= 0x7e {
					print!("{}", secdata.buf[lastidx] as char );
				} else {
					print!(".");
				}
				lastidx += 1;				
			}
			print!("\n");
		}*/
		idx += 1;
	}


	Ok(())
}


fn signdump_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let bs = read_file_bytes(f)?;
		let ocon = ContentInfo::from_der(&bs);
		if ocon.is_err() {
			let err = ocon.err().unwrap();
			extargs_new_error!{PeHdlError,"can not parse [{}] error[{:?}]", f,err}
		}
		let _content = ocon.unwrap();
		//println!("content [{:?}]",content);
		println!("get content");
	}

	Ok(())
}

fn vec_str_to_u8(vs :Vec<String>) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	for s in vs.iter() {
		let bb = format!("{}",s);

		let vv :Vec<&str> = bb.split(",").collect();
		for v in vv.iter() {
			let mut cparse = format!("{}",v);
			let mut base :u32 = 10;
			if cparse.starts_with("0x") || cparse.starts_with("0X") {
				cparse = cparse[2..].to_string();
				base = 16;
			} else if cparse.starts_with("x") || cparse.starts_with("X") {
				cparse = cparse[1..].to_string();
				base = 16;
			}

			match i64::from_str_radix(&cparse,base) {
				Ok(v) => {
					if v > 255 {
						extargs_new_error!{PeHdlError,"{} > 255", s}
					}
					retv.push(v as u8);
				},
				Err(e) => {
					extargs_new_error!{PeHdlError, "parse [{}] error [{:?}]", s, e}
				}
			}			
		}

	}
	Ok(retv)
}

const ULONG_MAX :u64 = 0xffffffffffffffff;

#[allow(unused_assignments)]
fn asn1_to_object(v8 :Vec<u8>) -> Result<String,Box<dyn Error>> {
	let mut rets :String = "".to_string();
	let mut bn :BigUint = Zero::zero();
	let mut l :u64;
	let mut lenv :usize = v8.len();
	let mut usebn :bool;
	let mut idx :usize = 0;
	let mut bfirst :bool = true;
	let mut i :u32 = 0;

	while lenv > 0 {
		l = 0;
		usebn = false;
		loop {
			let c = v8[idx];
			idx += 1;
			lenv -= 1;
			if lenv == 0 && (c & 0x80) != 0 {
				extargs_new_error!{PeHdlError,"c [0x{:02x}] at the end",c}
			}
			if usebn {
				bn += c & 0x7f;
				debug_trace!("bn [{}]",bn);
			} else {
				l += (c & 0x7f) as u64;
				debug_trace!("l [{}]", l);
			}

			if (c & 0x80) == 0 {
				break;
			}

			if !usebn && l >( ULONG_MAX >> 7) {
				bn = Zero::zero();
				bn += l;
				usebn = true;
			}

			if usebn {
				bn <<= 7;
			} else {
				l <<= 7;
			}
		}

		if bfirst {
			bfirst = false;
			if l >= 80 {
				i = 2;
				if usebn {
					bn -= 80 as u64;
				} else {
					l -= 80;
				}
			} else {
				i = (l / 40) as u32;
				l -= (i * 40) as u64;
			}

			debug_trace!("i {}",i);
			rets.push_str(&format!("{}",i));

		} 
		if usebn {
			rets.push_str(".");
			rets.push_str(&format!("{}",bn));
		} else {
			rets.push_str(".");
			rets.push_str(&format!("{}", l));
		}
	}

	Ok(rets)
}

fn str_to_asn1obj(s :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	let mut idx :usize = 0;
	let sarr :Vec<&str> = s.split(".").collect();
	let  mut curn :u64 = 0;
	for v in sarr.iter() {
		match u64::from_str_radix(v,10) {
			Ok(cn) => {
				if idx < 2 {
					if idx == 0 {
						curn = cn;
					} else {
						curn *= 40;
						curn += cn;

						retv.push(curn as u8);
						curn = 0;
					}

				} else {
					let mut maxidx :usize = 0;

					curn = cn;
					loop {
						if (curn >> (maxidx * 7))  == 0 {
							break;
						}
						maxidx += 1;
					}

					if maxidx == 0 {
						retv.push(0);
					} else {
						while maxidx > 1 {
							let bb :u8 = ((cn >> ((maxidx - 1) * 7)) & 0x7f) as u8;
							retv.push(bb | 0x80 );
							maxidx -= 1;
						}
						if maxidx == 1 {
							let bb :u8 = (cn & 0x7f) as u8;
							retv.push(bb);
						}
					}

				}
				idx += 1;
			},
			Err(e) => {
				match BigUint::from_str(v) {
					Ok(bn2) => {
						if idx < 2 {
							extargs_new_error!{PeHdlError,"can not parse [{}] at [{}] with bigint", s,v}
						}

						let mut maxidx :usize = 0;
						loop {
							let bn :BigUint = bn2.clone();
							let cb :BigUint = bn.shr(maxidx * 7);
							let zb :BigUint = Zero::zero();
							if cb.eq(&zb) {
								break;
							}
							maxidx += 1;
						}

						if maxidx < 1 {
							extargs_new_error!{PeHdlError,"bignum is {} to small", bn2}
						} else {
							while maxidx > 1 {
								let bn :BigUint = bn2.clone();
								let cb :BigUint = bn.shr((maxidx - 1) * 7);
								let bv :Vec<u8> = cb.to_bytes_le();
								let bb :u8 = bv[0] & 0x7f;
								retv.push(bb | 0x80);
								maxidx -= 1;
							}

							let bv :Vec<u8> = bn2.to_bytes_le();
							let bb :u8 = bv[0] & 0x7f;
							retv.push(bb);
						}

						idx += 1;
					},
					Err(_e2) => {
						extargs_new_error!{PeHdlError,"can not parse [{}] at [{}] {:?}", s,v,e}
					}
				}
			}
		}
	}

	Ok(retv)
}

fn asn1objdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	let v8 = vec_str_to_u8(sarr.clone())?;
	let vs = asn1_to_object(v8.clone())?;
	let vb = str_to_asn1obj(&vs)?;
	println!("{:?} => {} => {:?}", v8, vs, vb);
	Ok(())
}

#[extargs_map_function(pesecdata_handler,signdump_handler,asn1objdec_handler)]
pub fn load_pe_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"pesecdata<pesecdata_handler>##file ... to display pe security data##" : {
			"$" : "+"
		},
		"signdump<signdump_handler>##file ... to dump handler##" : {
			"$" : "+"
		},
		"asn1objdec<asn1objdec_handler>##byte ... to byte to decode object##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}

