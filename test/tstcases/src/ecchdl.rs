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
use num_traits::{zero,one};


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
use super::{debug_trace};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use ecsimple::curves::{get_ecc_by_name};
use ecsimple::jacobi::{PointJacobi};


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
    let mut cv : PointJacobi = get_ecc_by_name(&sarr[0])?;
    let retcv :PointJacobi = cv.mul_int(&multval);
    println!("PointJacobi\n{:?}",cv);
    println!("multval\n0x{:x}",multval);
    println!("retcv\n{:?}",retcv);
    Ok(())
}


#[extargs_map_function(multecc_handler)]
pub fn load_ecc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
    	"multecc<multecc_handler>##eccname multval to multiple##" : {
    		"$" : 2
    	}
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}


