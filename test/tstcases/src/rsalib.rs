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
use std::io::{Write,Read};


#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use rsa::BigUint as rsaBigUint;
use num_bigint::BigUint;
#[allow(unused_imports)]
use rsa::{RsaPublicKey,RsaPrivateKey,PublicKeyParts};
use hex::FromHex;
use num_traits::{Zero};

#[allow(unused_imports)]
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};


asn1obj_error_class!{RsalibError}

pub struct RandFile {
	f : std::fs::File,
	fname :String,
}

impl RandFile {
	pub fn new(name :&str) -> Result<RandFile,Box<dyn Error>> {
		let ores = std::fs::File::open(name);
		if ores.is_err() {
			let e = ores.err().unwrap();
			asn1obj_new_error!{RsalibError,"open {} error {:?}", name,e}
		}
		let f = ores.unwrap();
		Ok(RandFile {
			f : f,
			fname : format!("{}",name),
		})
	}
}

impl rand_core::CryptoRng  for RandFile {
}

impl rand_core::RngCore for RandFile {
	fn next_u32(&mut self) -> u32 {
		let mut buf = [0u8; 4];
		let ores = self.f.read(&mut buf);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != 4 {
			panic!("can not read [{}]", self.fname);
		}
		let mut retv :u32 = 0;
		for i in 0..buf.len() {
			retv |= (buf[i] as u32) << (i * 8);
		}
		retv
	}

	fn next_u64(&mut self) -> u64 {
		let mut buf = [0u8; 8];
		let ores = self.f.read(&mut buf);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != 8 {
			panic!("can not read [{}]", self.fname);
		}
		let mut retv :u64 = 0;
		for i in 0..buf.len() {
			retv |= (buf[i] as u64) << (i * 8);
		}
		retv
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		let ores = self.f.read(dest);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != dest.len() {
			panic!("can not read [{}]", self.fname);	
		}
		return;
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(),rand_core::Error> {
		let ores = self.f.read(dest);
		if ores.is_err() {
			let e = ores.err().unwrap();
			let e2 = RsalibError::create(&format!("read {} error {:?}",self.fname,e));
			return Err(rand_core::Error::new(e2));
		}
		let cnt = ores.unwrap();
		if cnt != dest.len() {
			let e2 = RsalibError::create(&format!("read {} cnt {} != {}",self.fname,cnt,dest.len()));
			return Err(rand_core::Error::new(e2));
		}
		Ok(())
	}
}

fn format_vecs(buf :&[u8], tab :i32) -> String {
	let mut outs :String = "".to_string();
	let mut lasti : usize = 0;
	let mut ki :usize;
	for i in 0..buf.len() {
		if (i%16) == 0 {
			if i > 0 {
				outs.push_str("    ");
				while lasti != i {
					if buf[lasti] >= 0x20 && buf[lasti] <= 0x7e {
						outs.push(buf[lasti] as char);
					} else {
						outs.push_str(".");
					}
					lasti += 1;
				}
				outs.push_str("\n");
			}

			for _j in 0..tab {
				outs.push_str("    ");
			}
		}
		if (i % 16) == 0 {
			outs.push_str(&format!("{:02x}", buf[i]));	
		} else {
			outs.push_str(&format!(":{:02x}", buf[i]));	
		}
		
	}

	if lasti != buf.len() {
		ki = buf.len();
		while (ki % 16) != 0 {
			outs.push_str("   ");
			ki += 1;
		}
		outs.push_str("    ");
		while lasti != buf.len() {
			if buf[lasti] >= 0x20 && buf[lasti] <= 0x7e {
				outs.push(buf[lasti] as char);
			} else {
				outs.push_str(".");
			}
			lasti += 1;
		}
	}
	outs.push_str("\n");
	return outs;
}

fn get_bigints(bn :&rsaBigUint,tab : i32) -> String {
	let buf :Vec<u8> = bn.to_bytes_be();
	return format_vecs(&buf,tab);
}

fn get_bigints_bn(bn :&BigUint,tab : i32) -> String {
	let buf :Vec<u8> = bn.to_bytes_be();
	return format_vecs(&buf,tab);
}

fn rsagen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let mut bits : usize = 2048;
	let mut gencore  = rand::thread_rng();
	let key :RsaPrivateKey;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");

	if sarr.len() > 0 {
		match i64::from_str_radix(&sarr[0],10) {
			Ok(v) => {
				bits = v as usize;
			},
			Err(e) => {
				extargs_new_error!{RsalibError, "parse [{}] error [{:?}]", sarr[0], e}
			}
		}
	}

	if sarr.len() > 1 {
		let mut cf = RandFile::new(&sarr[1])?;
		key = RsaPrivateKey::new(&mut cf,bits)?;
	} else {
		key = RsaPrivateKey::new(&mut gencore,bits)?;
	}

	
	let mut f = std::io::stdout();

	let mut outs :String;

	outs = format!("n {}\n{}",bits,get_bigints(key.n(),1));
	f.write(outs.as_bytes())?;
	outs = format!("e {}\n{}",bits,get_bigints(key.e(),1));
	f.write(outs.as_bytes())?;
	outs = format!("d {}\n{}",bits,get_bigints(key.d(),1));
	f.write(outs.as_bytes())?;
	let primes = key.primes();
	for i in 0..primes.len() {
		outs = format!("primes[{}] {}\n{}",i,bits,get_bigints(&(primes[i].clone()),1));
		f.write(outs.as_bytes())?;
	}

	Ok(())
}


fn rsaform_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	let p :BigUint ;
	let q :BigUint;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 2 {
		asn1obj_new_error!{RsalibError,"need p q primes"}
	}

	let mut v8 :Vec<u8>;
	v8 = Vec::from_hex(&sarr[0])?;
	p = BigUint::from_bytes_be(&v8);
	v8 = Vec::from_hex(&sarr[1])?;
	q = BigUint::from_bytes_be(&v8);
	let n = p.clone() * q.clone();
	let e1 :BigUint = Zero::zero();
	let e :BigUint = e1 + 0x10001 as u32;
	let r1 :BigUint = p.clone() - 1 as u32;
	let r2 :BigUint = q.clone() - 1 as u32;
	let d = r1.clone() * r2.clone();
	let mut outs :String;
	let mut f = std::io::stdout();
	outs = format!("p \n{}",get_bigints_bn(&p,1));
	f.write(outs.as_bytes())?;
	outs = format!("q \n{}",get_bigints_bn(&q,1));
	f.write(outs.as_bytes())?;
	outs = format!("n \n{}",get_bigints_bn(&n,1));
	f.write(outs.as_bytes())?;
	outs = format!("e \n{}",get_bigints_bn(&e,1));
	f.write(outs.as_bytes())?;
	outs = format!("d \n{}",get_bigints_bn(&d,1));
	f.write(outs.as_bytes())?;




	Ok(())
}

#[extargs_map_function(rsagen_handler,rsaform_handler)]
pub fn load_rsa_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"genrsa<rsagen_handler>##[bits] [rndfile] to generate rsa bits default 2048##" : {
			"$" : "*"
		},
		"rsaform<rsaform_handler>##p q prime to generate rsa value##" : {
			"$" : 2
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}
