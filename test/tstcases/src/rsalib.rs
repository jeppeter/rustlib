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
#[allow(unused_imports)]
use rsa::{RsaPublicKey,RsaPrivateKey,PublicKeyParts};
use hex::FromHex;

#[allow(unused_imports)]
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};


use num_bigint::traits::ModInverse;
use num_bigint::{BigUint, RandPrime};
#[allow(unused_imports)]
use num_traits::Float;
use num_traits::{FromPrimitive, One, Zero};

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

#[allow(dead_code)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorId {
    InvalidPaddingScheme,
    Decryption,
    Verification,
    MessageTooLong,
    InputNotHashed,
    NprimesTooSmall,
    TooFewPrimes,
    InvalidPrime,
    InvalidModulus,
    InvalidExponent,
    InvalidCoefficient,
    PublicExponentTooSmall,
    PublicExponentTooLarge,
    Internal,
    LabelTooLong,
}


/// Generates a multi-prime RSA keypair of the given bit size, public exponent,
/// and the given random source, as suggested in [1]. Although the public
/// keys are compatible (actually, indistinguishable) from the 2-prime case,
/// the private keys are not. Thus it may not be possible to export multi-prime
/// private keys in certain formats or to subsequently import them into other
/// code.
///
/// Table 1 in [2] suggests maximum numbers of primes for a given size.
///
/// [1]: https://patents.google.com/patent/US4405829A/en
/// [2]: http://www.cacr.math.uwaterloo.ca/techreports/2006/cacr2006-16.pdf
#[allow(unused_variables,unused_assignments)]
pub fn generate_multi_prime_key_with_exp<R: rand_core::RngCore + rand_core::CryptoRng>(
    rng: &mut R,
    nprimes: usize,
    bit_size: usize,
    exp: &rsaBigUint,
) -> Result<(),ErrorId> {
    if nprimes < 2 {
        return Err(ErrorId::NprimesTooSmall);
    }

    if bit_size < 64 {
        let prime_limit = (1u64 << (bit_size / nprimes) as u64) as f64;

        // pi aproximates the number of primes less than prime_limit
        let mut pi = prime_limit / (prime_limit.ln() - 1f64);
        // Generated primes start with 0b11, so we can only use a quarter of them.
        pi /= 4f64;
        // Use a factor of two to ensure taht key generation terminates in a
        // reasonable amount of time.
        pi /= 2f64;

        if pi < nprimes as f64 {
            return Err(ErrorId::TooFewPrimes);
        }
    }

    let mut primes = vec![BigUint::zero(); nprimes];
    let n_final: BigUint;
    let d_final: BigUint;

    'next: loop {
        let mut todo = bit_size;
        // `gen_prime` should set the top two bits in each prime.
        // Thus each prime has the form
        //   p_i = 2^bitlen(p_i) × 0.11... (in base 2).
        // And the product is:
        //   P = 2^todo × α
        // where α is the product of nprimes numbers of the form 0.11...
        //
        // If α < 1/2 (which can happen for nprimes > 2), we need to
        // shift todo to compensate for lost bits: the mean value of 0.11...
        // is 7/8, so todo + shift - nprimes * log2(7/8) ~= bits - 1/2
        // will give good results.
        if nprimes >= 7 {
            todo += (nprimes - 2) / 5;
        }

        for (i, prime) in primes.iter_mut().enumerate() {
            *prime = rng.gen_prime(todo / (nprimes - i));
            todo -= prime.bits();
        }

        // Makes sure that primes is pairwise unequal.
        for (i, prime1) in primes.iter().enumerate() {
            for prime2 in primes.iter().take(i) {
                if prime1 == prime2 {
                    continue 'next;
                }
            }
        }

        let mut n = BigUint::one();
        let mut totient = BigUint::one();

        for prime in &primes {
            n *= prime;
            totient *= prime - BigUint::one();
            debug_trace!("prime \n{}",get_bigints_bn(&prime,1));
            debug_trace!("cur totient\n{}",get_bigints_bn(&totient,1));
        }

        if n.bits() != bit_size {
            // This should never happen for nprimes == 2 because
            // gen_prime should set the top two bits in each prime.
            // For nprimes > 2 we hope it does not happen often.
            continue 'next;
        }

        if let Some(d) = exp.mod_inverse(totient) {
            n_final = n;
            d_final = d.to_biguint().unwrap();
            debug_trace!("d_final \n{}",get_bigints_bn(&d_final,1));
            break;
        }
    }

    Ok(())
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

	let exp = rsaBigUint::from_u64(0x10001).unwrap();
	let _ = generate_multi_prime_key_with_exp(&mut gencore,2 as usize,bits,&exp);

	
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
	let dbase =  r1.clone() * r2.clone();
	let d2 = e.clone().mod_inverse(&dbase).unwrap();
	let d = d2.to_biguint().unwrap();
	let co2 = q.clone().mod_inverse(&p).unwrap();
	let co = co2.to_biguint().unwrap();
	let exp1 = d.clone() % r1.clone();
	let exp2 = d.clone() % r2.clone();
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
	outs = format!("exp1 \n{}",get_bigints_bn(&exp1,1));
	f.write(outs.as_bytes())?;
	outs = format!("exp2 \n{}",get_bigints_bn(&exp2,1));
	f.write(outs.as_bytes())?;
	outs = format!("coeff \n{}",get_bigints_bn(&co,1));
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
