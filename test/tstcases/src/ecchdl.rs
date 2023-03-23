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
use hex::FromHex;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
use super::fileop::{read_file_bytes,write_file_bytes};

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
    let v8 :Vec<u8> = Vec::from_hex(&sarr[1])?;
    let multval :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
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
    let mut v8 :Vec<u8> = Vec::from_hex(&sarr[1])?;
    let mut multval :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let mut cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    let mut retcv :PointJacobi = cv.generator.mul_int(&multval);
    let mut idx :usize = 2;
    while idx < sarr.len() {
	    cv = get_ecc_curve_by_name(&sarr[0])?;
        v8 = Vec::from_hex(&sarr[idx])?;
        multval = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
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
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 4 {
        extargs_new_error!{EcchdlError,"need eccname secnum hashnumber randkey"}
    }
    let mut v8 :Vec<u8> = Vec::from_hex(&sarr[1])?;
    let secnum :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    v8 = Vec::from_hex(&sarr[2])?;
    let hashnumber :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let (_,hashcode) = hashnumber.to_bytes_be();
    v8 = Vec::from_hex(&sarr[3])?;
    let randkey :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
    let sig  =  privkey.sign(&hashcode,&randkey)?;
    let outv8 = sig.to_der()?;
    let pubkey :PublicKey = privkey.get_public_key();


    let outf :String = ns.get_string("output");
    if outf.len() == 0 {
        debug_buffer_trace!(outv8.as_ptr(),outv8.len(),"output ");
    } else {
        let _ = write_file_bytes(&outf,&outv8)?;
    }
    let outv8 = pubkey.to_der(EC_UNCOMPRESSED,EC_PARAMS_EXLICIT)?;
    debug_buffer_trace!(outv8.as_ptr(),outv8.len(),"pubkey ");
    Ok(())
}


fn verifybaseecc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{EcchdlError,"need eccname secnum hashnumber"}
    }
    let inf :String = ns.get_string("input");
    let signcode :Vec<u8> = read_file_bytes(&inf)?;
    let mut v8 :Vec<u8> = Vec::from_hex(&sarr[1])?;
    let secnum :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;
    v8 = Vec::from_hex(&sarr[2])?;
    let hashnumber :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let (_,hashcode) = hashnumber.to_bytes_be();
    let privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
    let pubkey :PublicKey = privkey.get_public_key();
    let sigv :ECCSignature = ECCSignature::from_der(&signcode)?;
    let valid :bool = pubkey.verify(&hashcode,&sigv);
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
    let mut v8 :Vec<u8> = Vec::from_hex(&sarr[0])?;
    let anum :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    v8 = Vec::from_hex(&sarr[1])?;
    let pnum :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let bnum :BigInt = square_root_mod_prime(&anum,&pnum)?;
    println!("0x{:x} = ( 0x{:x}) ** 2 % 0x{:x}",anum, bnum,pnum);
    Ok(())
}

fn expecpubkey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut types :String = "uncompressed".to_string();
    let mut paramstype :String = "explicit".to_string();
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
        extargs_new_error!{EcchdlError,"need eccname secnum"}
    }
    let v8 :Vec<u8> = Vec::from_hex(&sarr[1])?;
    let secnum :BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus,&v8);
    let cv : ECCCurve = get_ecc_curve_by_name(&sarr[0])?;

    if sarr.len() > 2 {
        types = format!("{}",sarr[2]);
    }

    if sarr.len() > 3 {
        paramstype = format!("{}",sarr[3]);
    }

    let privkey :PrivateKey = PrivateKey::new(&cv,&secnum)?;
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

#[extargs_map_function(multecc_handler,addecc_handler,signbaseecc_handler,verifybaseecc_handler,modsquareroot_handler,expecpubkey_handler,impecpubkey_handler)]
pub fn load_ecc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
    	"multecc<multecc_handler>##eccname multval to multiple##" : {
    		"$" : 2
    	},
    	"addecc<addecc_handler>##eccname multval ... to add ecc with multivalue##" : {
    		"$" : "+"
    	},
        "signbaseecc<signbaseecc_handler>##eccname secnum hashnumber randkey to sign to output##" : {
            "$" : 4
        },
        "verifybaseecc<verifybaseecc_handler>##eccname secnum hashnumber to verify input##" : {
            "$" : 3
        },
        "modsquareroot<modsquareroot_handler>##anum pnum to modsquareroot##" : {
            "$" : 2
        },
        "expecpubkey<expecpubkey_handler>##ecname secnum [compresstype] [explicit]##" : {
            "$" : "+"
        },
        "impecpubkey<impecpubkey_handler>##pubkeybin##" : {
            "$" : 1
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}


