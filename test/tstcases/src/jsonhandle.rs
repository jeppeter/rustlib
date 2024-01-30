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

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::fileop::{read_file_bytes,read_file,write_file_bytes};
#[allow(unused_imports)]
use super::strop::{parse_u64,decode_base64};
use std::any::Any;
use super::jsondata::{JSonPack,JSonUnpack};


asn1obj_error_class!{JsonHdlError}

fn jpmergejup_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{JsonHdlError,"need merge jsonpack jsonunpack json file"}
    }
    let s = read_file(&sarr[0])?;
    let us = read_file(&sarr[1])?;
    let mut jp :JSonPack = JSonPack::new(&s)?;
    let jup :JSonUnpack = JSonUnpack::new(&us)?;
    {
        let mut refv :Vec<&str> = Vec::new();
        for s in sarr[2..].iter() {
            refv.push(s);
        }
        let _ = jp.merge_unpack_ref(&jup,&refv)?;
        let data = jp.pack()?;
        debug_buffer_trace!(data.as_ptr(),data.len(),"json pack ref");
    }
    let mut jp :JSonPack = JSonPack::new(&s)?;
    let jup :JSonUnpack = JSonUnpack::new(&us)?;
    
    let _ = jp.merge_unpack(&jup,&sarr[2..])?;
    let data = jp.pack()?;
    let output = ns.get_string("output");
    debug_buffer_trace!(data.as_ptr(),data.len(),"json pack");
    let _ = write_file_bytes(&output,&data)?;
    Ok(())
}

fn jupmergejp_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{JsonHdlError,"need merge jsonpack jsonunpack json file"}
    }
    let s = read_file(&sarr[0])?;
    let us = read_file(&sarr[1])?;
    let mut jup :JSonUnpack = JSonUnpack::new(&s)?;
    let jp :JSonPack = JSonPack::new(&us)?;
    let _ = jup.merge_pack(&jp,&sarr[2..])?;
    let data = jup.pack()?;
    let output = ns.get_string("output");
    debug_buffer_trace!(data.as_ptr(),data.len(),"json unpack");
    let _ = write_file_bytes(&output,&data)?;
    Ok(())
}



#[extargs_map_function(jpmergejup_handler,jupmergejp_handler)]
pub fn load_json_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
        "jpmergejup<jpmergejup_handler>##from files to input and to output##" : {
            "$" : "+"
        },
        "jupmergejp<jupmergejp_handler>##from files to input and to output##" : {
            "$" : "+"
        }
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}