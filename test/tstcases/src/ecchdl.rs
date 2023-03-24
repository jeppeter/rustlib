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
use num_bigint::{BigInt};
//use num_traits::{zero,one};


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

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::fileop::{read_file_bytes,write_file_bytes};
use super::strop::{parse_to_bigint};
use super::asn1def::*;

use ecsimple::arithmetics::*;
use ecsimple::consts::*;
use ecsimple::curves::*;
use ecsimple::jacobi::*;
use ecsimple::keys::*;
use ecsimple::signature::*;


extargs_error_class!{EcchdlError}


fn multecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
    	extargs_new_error!{EcchdlError,"need eccname and multval"}
    }
    let multval :BigInt = parse_to_bigint(&sarr[1])?;
    let mut cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    let retcv :PointJacobi = cv.generator.mul_int(&multval);
    println!("PointJacobi\n{:?}",cv.generator);
    println!("multval\n0x{:x}",multval);
    println!("retcv\n{:?}",retcv);
    Ok(())
}

fn addecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
    	extargs_new_error!{EcchdlError,"need eccname and multval"}
    }
    let mut multval :BigInt = parse_to_bigint(&sarr[1])?;
    let mut cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    let mut retcv :PointJacobi = cv.generator.mul_int(&multval);
    let mut idx :usize = 2;
    while idx < sarr.len() {
	    cv = get_ecc_curve_by_name(&sarr[0])?;
        multval = parse_to_bigint(&sarr[idx])?;
	    let curv :PointJacobi = cv.generator.mul_int(&multval);
	    retcv = retcv.add_jacobi(&curv);
        idx += 1;
    }

    println!("PointJacobi\n{:?}",cv.generator);
    println!("result\n{:?}",retcv);
    Ok(())
}


fn signbaseecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut rname :Option<String> = None;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 4 {
        extargs_new_error!{EcchdlError,"need eccname secnum hashnumber randkey"}
    }
    if ns.get_string("ecrandfile").len() > 0 {
        rname = Some(format!("{}",ns.get_string("ecrandfile")));
    }
    let secnum :BigInt = parse_to_bigint(&sarr[1])?;
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    let hashnumber :BigInt = parse_to_bigint(&sarr[2])?;
    let (_,hashcode) = hashnumber.to_bytes_be();
    let randkey :BigInt = parse_to_bigint(&sarr[3])?;
    let mut privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
    privkey.set_rand_file(rname);
    let sig  =  privkey.sign_base(&hashcode,&randkey)?;
    let outv8 = sig.to_der()?;
    let pubkey :PublicKey = privkey.get_public_key();


    let outf :String = ns.get_string("output");
    if outf.len() == 0 {
        debug_buffer_trace!(outv8.as_ptr(),outv8.len(),"output ");
    } else {
        let _ = write_file_bytes(&outf,&outv8)?;
    }
    let outv8 = pubkey.to_der(EC_UNCOMPRESSED,EC_PARAMS_EXLICIT)?;
    let pubout = ns.get_string("ecpubout");
    if pubout.len() > 0 {
        let _ = write_file_bytes(&pubout,&outv8)?;
    } else {
        debug_buffer_trace!(outv8.as_ptr(),outv8.len(),"pubkey ");    
    }
    
    Ok(())
}


fn verifybaseecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 1 {
        extargs_new_error!{EcchdlError,"need hashnumber pubkey.bin"}
    }
    let inf :String = ns.get_string("input");
    let signcode :Vec<u8> = read_file_bytes(&inf)?;
    let hashnumber :BigInt = parse_to_bigint(&sarr[0])?;
    let (_,hashcode) = hashnumber.to_bytes_be();
    let pubcode :Vec<u8> = read_file_bytes(&sarr[1])?;
    let pubkey :PublicKey = PublicKey::from_der(&pubcode)?;
    let sigv :ECCSignature = ECCSignature::from_der(&signcode)?;
    let valid :bool = pubkey.verify_base(&hashcode,&sigv);
    if valid {
        println!("verify {} ok", inf);
    } else {
        extargs_new_error!{EcchdlError,"verify {} failed ",inf}
    }
    Ok(())
}


fn modsquareroot_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
        extargs_new_error!{EcchdlError,"need anum pnum"}
    }
    let anum :BigInt = parse_to_bigint(&sarr[0])?;
    let pnum :BigInt = parse_to_bigint(&sarr[1])?;
    let bnum :BigInt = square_root_mod_prime(&anum,&pnum)?;
    println!("0x{:x} = ( 0x{:x}) ** 2 % 0x{:x}",anum, bnum,pnum);
    Ok(())
}

fn expecpubkey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut types :String = "uncompressed".to_string();
    let mut paramstype :String = "".to_string();
    let mut rname :Option<String> = None;
    init_log(ns.clone())?;

    if ns.get_string("ecrandfile").len() > 0 {
        rname = Some(format!("{}",ns.get_string("ecrandfile")));
    }

    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
        extargs_new_error!{EcchdlError,"need eccname secnum"}
    }
    let secnum :BigInt = parse_to_bigint(&sarr[1])?;
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;

    if sarr.len() > 2 {
        types = format!("{}",sarr[2]);
    }

    if sarr.len() > 3 {
        paramstype = format!("{}",sarr[3]);
    }

    let mut privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
    privkey.set_rand_file(rname);
    let pubkey :PublicKey = privkey.get_public_key();
    let outv8 = pubkey.to_der(&types,&paramstype)?;


    let outf :String = ns.get_string("output");
    if outf.len() == 0 {
        debug_buffer_trace!(outv8.as_ptr(),outv8.len(),"output ");
    } else {
        let _ = write_file_bytes(&outf,&outv8)?;
    }
    Ok(())
}

fn impecpubkey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 1 {
        extargs_new_error!{EcchdlError,"need pubkeybin"}
    }
    let vecs = read_file_bytes(&sarr[0])?;
    let pubk = PublicKey::from_der(&vecs)?;
    println!("{:?}",pubk);
    Ok(())
}

fn signdigestecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut rname :Option<String> = None;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{EcchdlError,"eccname secnum binfile"}
    }
    let secnum :BigInt = parse_to_bigint(&sarr[1])?;
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;


    let mut privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
    if ns.get_string("ecrandfile").len() > 0 {
        rname = Some(format!("{}",ns.get_string("ecrandfile")));
    }
    privkey.set_rand_file(rname);
    let rdata :Vec<u8> = read_file_bytes(&sarr[2])?;
    let digdata :Vec<u8> = Sha1Digest::calc(&rdata);
    let sigv = privkey.sign_digest(&digdata)?;
    let sigcode = sigv.to_der()?;
    let outf = ns.get_string("output");
    if outf.len() > 0 {
        let _ = write_file_bytes(&outf,&sigcode)?;
    } else {
        debug_buffer_trace!(sigcode.as_ptr(),sigcode.len(),"sig code");
    }
    Ok(())
}

fn verifydigestecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{EcchdlError,"need pubkeybin contentbin signbin"}
    }
    let pubcode :Vec<u8> = read_file_bytes(&sarr[0])?;
    let hashcode = read_file_bytes(&sarr[1])?;
    let signcode :Vec<u8> = read_file_bytes(&sarr[2])?;
    let pubkey :PublicKey = PublicKey::from_der(&pubcode)?;
    let sigv :ECCSignature = ECCSignature::from_der(&signcode)?;
    let digcode = Sha1Digest::calc(&hashcode);
    let valid :bool = pubkey.verify_digest(&digcode,&sigv);
    if valid {
        println!("verify {} ok", sarr[1]);
    } else {
        extargs_new_error!{EcchdlError,"verify {} failed ",sarr[1]}
    }
    Ok(())}


#[extargs_map_function(multecc_handler,addecc_handler,signbaseecc_handler,verifybaseecc_handler,modsquareroot_handler,expecpubkey_handler,impecpubkey_handler,signdigestecc_handler,verifydigestecc_handler)]
pub fn load_ecc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
        "ecpubout" : null,
        "ecrandfile" : null,
    	"multecc<multecc_handler>##eccname multval to multiple##" : {
    		"$" : 2
    	},
    	"addecc<addecc_handler>##eccname multval ... to add ecc with multivalue##" : {
    		"$" : "+"
    	},
        "signbaseecc<signbaseecc_handler>##eccname secnum hashnumber randkey to sign to output##" : {
            "$" : 4
        },
        "verifybaseecc<verifybaseecc_handler>##hashnumber pubkey.bin to verify input##" : {
            "$" : 2
        },
        "modsquareroot<modsquareroot_handler>##anum pnum to modsquareroot##" : {
            "$" : 2
        },
        "expecpubkey<expecpubkey_handler>##ecname secnum [compresstype] [explicit]##" : {
            "$" : "+"
        },
        "impecpubkey<impecpubkey_handler>##pubkeybin##" : {
            "$" : 1
        },
        "signdigestecc<signdigestecc_handler>##eccname secnum binfile to sign digest##" : {
            "$" : 3
        },
        "verifydigestecc<verifydigestecc_handler>##pubkeybin binfile signbin to verify digest##" : {
            "$" : 2
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}


