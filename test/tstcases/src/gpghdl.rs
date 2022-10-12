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

#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::cell::RefCell;
use std::sync::Arc;
//use std::io::Write;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::fileop::{read_file_bytes,read_file};

use super::cryptlib::{opengpg_s2k_sha512,aes256_cfb_decrypt};
use super::strop::{parse_u64,decode_base64};
use gpgobj::crc::{GpgCrc24};
use gpgobj::complex::{GpgPubKeyFile,GpggpgFile};
use gpgobj::gpgimpl::{GpgOp};

use hex::{FromHex};
use hex;

asn1obj_error_class!{GpgHdlError}


fn gpgkdfs2k512_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	if sarr.len() != 3 {
		asn1obj_new_error!{GpgHdlError,"need pass salt iterations"}
	}
	let passin = format!("{}",sarr[0]);
	let saltv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
	let iterations :u64 = parse_u64(&sarr[2])?;
	let outv = opengpg_s2k_sha512(passin.as_bytes(),&saltv8,iterations as usize,32)?;
	debug_buffer_trace!(outv.as_ptr(),outv.len(),"output kdf");
	Ok(())
}


fn gpgcrc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	let mut gpgcrc  = GpgCrc24::new();
	sarr = ns.get_array("subnargs");
	for f in sarr.iter() {
		let datav8 = read_file_bytes(f)?;
		gpgcrc.update(&datav8);
		println!("after [{}] crc [0x{:x}]", f,gpgcrc.get());
	}
	Ok(())
}


#[allow(unused_assignments)]
fn decode_gpg_asc(s :&str) -> Result<(Vec<u8>,Vec<u8>), Box<dyn Error>> {
    let sarr :Vec<&str> = s.split("\n").collect();
    let mut maind :Vec<u8> = Vec::new();
    let mut bd :Vec<u8> = Vec::new();
    let mut c :String = "".to_string();
    for l in sarr.iter() {
        let mut kl = format!("{}",l);
        kl = kl.trim_end_matches("\r").to_string();
        if !kl.starts_with("---") {
            c.push_str(&kl);
            if kl.len() < 64 {
                if maind.len() == 0 {
                    maind = decode_base64(&c)?;
                    c = "".to_string();
                } else if bd.len() == 0 {
                    c = c[1..].to_string();                    
                    bd = decode_base64(&c)?;
                    c = "".to_string();
                }
            }
        }
    }

    if c.len() > 0 && bd.len() == 0 {
        c = c[1..].to_string();
        bd = decode_base64(&c)?;
        c = "".to_string();
    }
    return Ok((maind,bd));
}

fn gpgascdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let s = read_file(f)?;
        let (maind,bd) = decode_gpg_asc(&s)?;
        let mut getcrc :u32 = 0;
        let mut gpgcrc24 :GpgCrc24 = GpgCrc24::new();
        if bd.len() != 3 {
        	asn1obj_new_error!{GpgHdlError,"crc [{:?}] not valid", bd}
        }
        for i in 0..bd.len() {
        	getcrc |= (bd[i] as u32 ) << ((2 - i) * 8);
        }

        gpgcrc24.update(&maind);
        if gpgcrc24.get() != getcrc {
        	asn1obj_new_error!{GpgHdlError,"check crc [0x{:x}] != get crc [0x{:x}]", gpgcrc24.get(), getcrc}
        }

        debug_buffer_trace!(maind.as_ptr(),maind.len(),"maind");
        debug_buffer_trace!(bd.as_ptr(),bd.len(),"bd");
    }
    Ok(())
}

fn gpgpubdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut serr = std::io::stderr();
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut pubk  = GpgPubKeyFile::init_gpg();
        let _ = pubk.decode_gpg(&code)?;
        pubk.print_gpg("pubk",0,&mut serr)?;
    }
    Ok(())
}

fn s2k_decode_cnt( cnt :u8) -> u64 {
    let retv :u64;
    retv = ((16 + cnt & 0xf) as u64) << (( cnt >> 4) + 6);
    return retv;
}

fn gpggpgdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut serr = std::io::stderr();
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut pubk  = GpggpgFile::init_gpg();
        let _ = pubk.decode_gpg(&code)?;
        pubk.print_gpg("pubk",0,&mut serr)?;

        /*now we should decode the number*/
        let salt = pubk.seskey.salt.data.clone();
        let cnt :u64 = s2k_decode_cnt(pubk.seskey.cnt.data);
        let passinstr = ns.get_string("passin");
        let passin = passinstr.as_bytes();
        let key = opengpg_s2k_sha512(passin,&salt,cnt as usize,32)?;
        let iv :Vec<u8> = Vec::from_hex("00000000000000000000000000000000")?;
        let decdata = aes256_cfb_decrypt(&(pubk.encdata.data.data),&key,&iv)?;
        debug_buffer_trace!(decdata.as_ptr(),decdata.len(),"decdata ");
    }
    Ok(())
}


#[extargs_map_function(gpgkdfs2k512_handler,gpgcrc_handler,gpgascdec_handler,gpgpubdec_handler,gpggpgdec_handler)]
pub fn load_gpg_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"gpgkdfs2k512<gpgkdfs2k512_handler>##pass salt iterations to encrypt##" : {
			"$" : 3
		},
		"gpgcrc<gpgcrc_handler>##file ... to crc in gpg##" : {
			"$" : "+"
		},
        "gpgascdec<gpgascdec_handler>##gpgascfile ... to decode gpg file##" : {
            "$" : "+"
        },
        "gpggpgdec<gpggpgdec_handler>##gpggpgfile ... to decode gpg file##" : {
            "$" : "+"
        },
        "gpgpubdec<gpgpubdec_handler>##gpgpubfile ... to decode gpg public key##" : {
            "$" : "+"
        }
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}