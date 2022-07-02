#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use std::error::Error;

use std::str::FromStr;
use std::ops::Shr;
use num_bigint::{BigUint};
use num_traits::{Zero};

extargs_error_class!{SslLibError}

pub fn vec_str_to_u8(vs :Vec<String>) -> Result<Vec<u8>,Box<dyn Error>> {
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
						extargs_new_error!{SslLibError,"{} > 255", s}
					}
					retv.push(v as u8);
				},
				Err(e) => {
					extargs_new_error!{SslLibError, "parse [{}] error [{:?}]", s, e}
				}
			}			
		}

	}
	Ok(retv)
}

const ULONG_MAX :u64 = 0xffffffffffffffff;

#[allow(unused_assignments)]
pub fn asn1_to_object(v8 :&[u8]) -> Result<String,Box<dyn Error>> {
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
				extargs_new_error!{SslLibError,"c [0x{:02x}] at the end",c}
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

pub fn str_to_asn1obj(s :&str) -> Result<Vec<u8>,Box<dyn Error>> {
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
							extargs_new_error!{SslLibError,"can not parse [{}] at [{}] with bigint", s,v}
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
							extargs_new_error!{SslLibError	,"bignum is {} to small", bn2}
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
						extargs_new_error!{SslLibError,"can not parse [{}] at [{}] {:?}", s,v,e}
					}
				}
			}
		}
	}

	Ok(retv)
}