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
//#[allow(unused_imports)]
//use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use extlog::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use extlog::loglib::{log_get_timestamp,log_output_function};
use extutils::logtrans::{init_log};
use extutils::strop::{parse_u64};
use libloading;


extargs_error_class!{DlError}

fn dlcallint_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval :i32;
	let mut idx :usize;
	let mut params :Vec<i32> = vec![];
	let funcname :String;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;

	funcname = format!("{}\0",sarr[1]);


    if numparam == 0 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn() -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func();
        }
    } else if numparam == 1 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0]);
        }
    } else if numparam == 2 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1]);
        }
    } else if numparam == 3 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2]);
        }
    } else if numparam == 4 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3]);
        }
    } else if numparam == 5 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4]);
        }
    } else if numparam == 6 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5]);
        }
    } else if numparam == 7 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6]);
        }
    } else if numparam == 8 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7]);
        }
    } else if numparam == 9 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8]);
        }
    } else if numparam == 10 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9]);
        }
    } else if numparam == 11 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10]);
        }
    } else if numparam == 12 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11]);
        }
    } else if numparam == 13 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12]);
        }
    } else if numparam == 14 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13]);
        }
    } else if numparam == 15 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14]);
        }
    } else if numparam == 16 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15]);
        }
    } else if numparam == 17 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15],params[16]);
        }
    } else if numparam == 18 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> i32 > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15],params[16],params[17]);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }


	println!("call [{}].[{}] retval {}", sarr[0],sarr[1],retval);
	Ok(())
}

fn dlcallptr_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval :*const std::ffi::c_void;
	let mut idx :usize;
	let mut params :Vec<i32> = vec![];
	let funcname :String;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;
	funcname = format!("{}\0",sarr[1]);


    if numparam == 0 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn() -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func();
        }
    } else if numparam == 1 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0]);
        }
    } else if numparam == 2 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1]);
        }
    } else if numparam == 3 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2]);
        }
    } else if numparam == 4 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3]);
        }
    } else if numparam == 5 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4]);
        }
    } else if numparam == 6 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5]);
        }
    } else if numparam == 7 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6]);
        }
    } else if numparam == 8 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7]);
        }
    } else if numparam == 9 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8]);
        }
    } else if numparam == 10 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9]);
        }
    } else if numparam == 11 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10]);
        }
    } else if numparam == 12 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11]);
        }
    } else if numparam == 13 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12]);
        }
    } else if numparam == 14 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13]);
        }
    } else if numparam == 15 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14]);
        }
    } else if numparam == 16 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15]);
        }
    } else if numparam == 17 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15],params[16]);
        }
    } else if numparam == 18 {
        idx = 3;
        while idx < sarr.len() {
            params.push(parse_u64(&sarr[idx])? as i32);
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(0);
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0],params[1],params[2],params[3],params[4],params[5],params[6],params[7],params[8],params[9],params[10],params[11],params[12],params[13],params[14],params[15],params[16],params[17]);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }


	println!("call [{}].[{}] retval {:p}", sarr[0],sarr[1],retval);
	Ok(())
}


fn dlcallstr_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval :*const std::ffi::c_void;
	let mut idx :usize;
	let mut params :Vec<String> = vec![];
	let funcname :String;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;
	funcname = format!("{}\0",sarr[1]);


    if numparam == 0 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn() -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func();
        }
    } else if numparam == 1 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 2 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 3 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 4 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 5 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 6 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 7 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 8 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 9 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 10 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 11 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 12 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 13 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 14 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 15 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 16 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 17 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 18 {
        idx = 3;
        while idx < sarr.len() {
            params.push(format!("{}\0",sarr[idx]));
            idx += 1;
        }
        while (params.len() as u64) < numparam {
            params.push(format!("\0"));
        }
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;
            retval = func(params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char,params[17].as_ptr() as *const std::ffi::c_char);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }




	println!("call [{}].[{}] retval {:p}", sarr[0],sarr[1],retval);
	Ok(())
}


// callback function with 0 params
unsafe extern "C" fn callback_0() -> std::ffi::c_int {
    return 0;
}

// callback function with 1 params
unsafe extern "C" fn callback_1(a0 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
    return 1;
}

// callback function with 2 params
unsafe extern "C" fn callback_2(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
    return 2;
}

// callback function with 3 params
unsafe extern "C" fn callback_3(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
    return 3;
}

// callback function with 4 params
unsafe extern "C" fn callback_4(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
    return 4;
}

// callback function with 5 params
unsafe extern "C" fn callback_5(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
    return 5;
}

// callback function with 6 params
unsafe extern "C" fn callback_6(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
    return 6;
}

// callback function with 7 params
unsafe extern "C" fn callback_7(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
    return 7;
}

// callback function with 8 params
unsafe extern "C" fn callback_8(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
    return 8;
}

// callback function with 9 params
unsafe extern "C" fn callback_9(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
    return 9;
}

// callback function with 10 params
unsafe extern "C" fn callback_10(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
    return 10;
}

// callback function with 11 params
unsafe extern "C" fn callback_11(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
    return 11;
}

// callback function with 12 params
unsafe extern "C" fn callback_12(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
    return 12;
}

// callback function with 13 params
unsafe extern "C" fn callback_13(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
    return 13;
}

// callback function with 14 params
unsafe extern "C" fn callback_14(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char,a13 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let a13s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a13).to_str()};
    if ores.is_err() {
        println!("RUST: error on 13");
        return -1;
    }
    a13s = ores.unwrap().to_string();
    println!("RUST:a13={}",a13s);
    return 14;
}

// callback function with 15 params
unsafe extern "C" fn callback_15(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char,a13 :*const std::ffi::c_char,a14 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let a13s : String;
    let a14s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a13).to_str()};
    if ores.is_err() {
        println!("RUST: error on 13");
        return -1;
    }
    a13s = ores.unwrap().to_string();
    println!("RUST:a13={}",a13s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a14).to_str()};
    if ores.is_err() {
        println!("RUST: error on 14");
        return -1;
    }
    a14s = ores.unwrap().to_string();
    println!("RUST:a14={}",a14s);
    return 15;
}

// callback function with 16 params
unsafe extern "C" fn callback_16(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char,a13 :*const std::ffi::c_char,a14 :*const std::ffi::c_char,a15 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let a13s : String;
    let a14s : String;
    let a15s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a13).to_str()};
    if ores.is_err() {
        println!("RUST: error on 13");
        return -1;
    }
    a13s = ores.unwrap().to_string();
    println!("RUST:a13={}",a13s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a14).to_str()};
    if ores.is_err() {
        println!("RUST: error on 14");
        return -1;
    }
    a14s = ores.unwrap().to_string();
    println!("RUST:a14={}",a14s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a15).to_str()};
    if ores.is_err() {
        println!("RUST: error on 15");
        return -1;
    }
    a15s = ores.unwrap().to_string();
    println!("RUST:a15={}",a15s);
    return 16;
}

// callback function with 17 params
unsafe extern "C" fn callback_17(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char,a13 :*const std::ffi::c_char,a14 :*const std::ffi::c_char,a15 :*const std::ffi::c_char,a16 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let a13s : String;
    let a14s : String;
    let a15s : String;
    let a16s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a13).to_str()};
    if ores.is_err() {
        println!("RUST: error on 13");
        return -1;
    }
    a13s = ores.unwrap().to_string();
    println!("RUST:a13={}",a13s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a14).to_str()};
    if ores.is_err() {
        println!("RUST: error on 14");
        return -1;
    }
    a14s = ores.unwrap().to_string();
    println!("RUST:a14={}",a14s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a15).to_str()};
    if ores.is_err() {
        println!("RUST: error on 15");
        return -1;
    }
    a15s = ores.unwrap().to_string();
    println!("RUST:a15={}",a15s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a16).to_str()};
    if ores.is_err() {
        println!("RUST: error on 16");
        return -1;
    }
    a16s = ores.unwrap().to_string();
    println!("RUST:a16={}",a16s);
    return 17;
}

// callback function with 18 params
unsafe extern "C" fn callback_18(a0 :*const std::ffi::c_char,a1 :*const std::ffi::c_char,a2 :*const std::ffi::c_char,a3 :*const std::ffi::c_char,a4 :*const std::ffi::c_char,a5 :*const std::ffi::c_char,a6 :*const std::ffi::c_char,a7 :*const std::ffi::c_char,a8 :*const std::ffi::c_char,a9 :*const std::ffi::c_char,a10 :*const std::ffi::c_char,a11 :*const std::ffi::c_char,a12 :*const std::ffi::c_char,a13 :*const std::ffi::c_char,a14 :*const std::ffi::c_char,a15 :*const std::ffi::c_char,a16 :*const std::ffi::c_char,a17 :*const std::ffi::c_char) -> std::ffi::c_int {
    let a0s : String;
    let a1s : String;
    let a2s : String;
    let a3s : String;
    let a4s : String;
    let a5s : String;
    let a6s : String;
    let a7s : String;
    let a8s : String;
    let a9s : String;
    let a10s : String;
    let a11s : String;
    let a12s : String;
    let a13s : String;
    let a14s : String;
    let a15s : String;
    let a16s : String;
    let a17s : String;
    let mut ores:Result<&str,std::str::Utf8Error>;
     
    ores = unsafe { std::ffi::CStr::from_ptr(a0).to_str()};
    if ores.is_err() {
        println!("RUST: error on 0");
        return -1;
    }
    a0s = ores.unwrap().to_string();
    println!("RUST:a0={}",a0s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a1).to_str()};
    if ores.is_err() {
        println!("RUST: error on 1");
        return -1;
    }
    a1s = ores.unwrap().to_string();
    println!("RUST:a1={}",a1s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a2).to_str()};
    if ores.is_err() {
        println!("RUST: error on 2");
        return -1;
    }
    a2s = ores.unwrap().to_string();
    println!("RUST:a2={}",a2s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a3).to_str()};
    if ores.is_err() {
        println!("RUST: error on 3");
        return -1;
    }
    a3s = ores.unwrap().to_string();
    println!("RUST:a3={}",a3s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a4).to_str()};
    if ores.is_err() {
        println!("RUST: error on 4");
        return -1;
    }
    a4s = ores.unwrap().to_string();
    println!("RUST:a4={}",a4s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a5).to_str()};
    if ores.is_err() {
        println!("RUST: error on 5");
        return -1;
    }
    a5s = ores.unwrap().to_string();
    println!("RUST:a5={}",a5s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a6).to_str()};
    if ores.is_err() {
        println!("RUST: error on 6");
        return -1;
    }
    a6s = ores.unwrap().to_string();
    println!("RUST:a6={}",a6s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a7).to_str()};
    if ores.is_err() {
        println!("RUST: error on 7");
        return -1;
    }
    a7s = ores.unwrap().to_string();
    println!("RUST:a7={}",a7s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a8).to_str()};
    if ores.is_err() {
        println!("RUST: error on 8");
        return -1;
    }
    a8s = ores.unwrap().to_string();
    println!("RUST:a8={}",a8s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a9).to_str()};
    if ores.is_err() {
        println!("RUST: error on 9");
        return -1;
    }
    a9s = ores.unwrap().to_string();
    println!("RUST:a9={}",a9s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a10).to_str()};
    if ores.is_err() {
        println!("RUST: error on 10");
        return -1;
    }
    a10s = ores.unwrap().to_string();
    println!("RUST:a10={}",a10s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a11).to_str()};
    if ores.is_err() {
        println!("RUST: error on 11");
        return -1;
    }
    a11s = ores.unwrap().to_string();
    println!("RUST:a11={}",a11s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a12).to_str()};
    if ores.is_err() {
        println!("RUST: error on 12");
        return -1;
    }
    a12s = ores.unwrap().to_string();
    println!("RUST:a12={}",a12s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a13).to_str()};
    if ores.is_err() {
        println!("RUST: error on 13");
        return -1;
    }
    a13s = ores.unwrap().to_string();
    println!("RUST:a13={}",a13s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a14).to_str()};
    if ores.is_err() {
        println!("RUST: error on 14");
        return -1;
    }
    a14s = ores.unwrap().to_string();
    println!("RUST:a14={}",a14s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a15).to_str()};
    if ores.is_err() {
        println!("RUST: error on 15");
        return -1;
    }
    a15s = ores.unwrap().to_string();
    println!("RUST:a15={}",a15s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a16).to_str()};
    if ores.is_err() {
        println!("RUST: error on 16");
        return -1;
    }
    a16s = ores.unwrap().to_string();
    println!("RUST:a16={}",a16s);
     
    ores = unsafe { std::ffi::CStr::from_ptr(a17).to_str()};
    if ores.is_err() {
        println!("RUST: error on 17");
        return -1;
    }
    a17s = ores.unwrap().to_string();
    println!("RUST:a17={}",a17s);
    return 18;
}


fn dlcallback_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval : std::ffi::c_int;
	let mut idx :usize;
	let mut params :Vec<String> = vec![];
	let funcname :String;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;
	funcname = format!("{}\0",sarr[1]);

	idx = 3;
	while idx < sarr.len() {
		params.push(format!("{}\0",sarr[idx]));
		idx += 1;
	}

	while params.len() < (numparam as usize) {
		params.push(format!("\0"));
	}


    if numparam == 0 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn() -> std::ffi::c_int,) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_0);
        }
    } else if numparam == 1 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_1,params[0].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 2 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_2,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 3 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_3,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 4 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_4,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 5 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_5,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 6 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_6,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 7 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_7,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 8 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_8,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 9 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_9,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 10 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_10,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 11 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_11,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 12 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_12,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 13 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_13,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 14 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_14,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 15 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_15,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 16 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_16,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 17 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_17,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 18 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(callback_18,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char,params[17].as_ptr() as *const std::ffi::c_char);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }




	println!("call [{}].[{}] retval {}", sarr[0],sarr[1],retval);
	Ok(())
}

unsafe extern "C" fn stk_call_back(num :std::ffi::c_int,argv :*const *const std::ffi::c_char) -> std::ffi::c_int {
	let mut vecs:Vec<String> = vec![];
	let mut i:std::ffi::c_int;
    let mut ores:Result<&str,std::str::Utf8Error>;
    let mut curptr :*const std::ffi::c_char;

	i= 0;
	while i < num {
		curptr = unsafe{*(argv.wrapping_add(i as usize))};
     
	    ores = unsafe { std::ffi::CStr::from_ptr(curptr).to_str()};
	    if ores.is_err() {
	        println!("RUST: error on {}",i);
	        return -1;
	    }
	    vecs.push(ores.unwrap().to_string());
		i += 1;
	}

	i = 0;
	while (i as usize) < vecs.len() {
		println!("RUST:a{}={}",i,vecs[i as usize]);
		i += 1;
	}
	return num;
}


fn dlcallstkback_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval : std::ffi::c_int;
	let mut idx :usize;
	let mut params :Vec<String> = vec![];
	let funcname :String;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;
	funcname = format!("{}\0",sarr[1]);

	idx = 3;
	while idx < sarr.len() {
		params.push(format!("{}\0",sarr[idx]));
		idx += 1;
	}

	while params.len() < (numparam as usize) {
		params.push(format!("\0"));
	}

    if numparam == 0 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back);
        }
    } else if numparam == 1 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 2 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 3 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 4 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 5 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 6 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 7 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 8 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 9 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 10 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 11 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 12 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 13 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 14 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 15 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 16 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 17 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 18 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char,params[17].as_ptr() as *const std::ffi::c_char);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }




	println!("call [{}].[{}] retval {}", sarr[0],sarr[1],retval);
	Ok(())
}

struct CCValue {
	iv :i32,
	strval :String,
}

impl CCValue {
	fn new(iv :i32,nv :&str) -> CCValue {
		CCValue {
			iv : iv,
			strval :format!("{}",nv),
		}
	}
}


unsafe extern "C" fn stk_call_back_with_arg(args :*const std::ffi::c_void,num :std::ffi::c_int,argv :*const *const std::ffi::c_char) -> std::ffi::c_int {
	let mut vecs:Vec<String> = vec![];
	let mut i:std::ffi::c_int;
    let mut ores:Result<&str,std::str::Utf8Error>;
    let mut curptr :*const std::ffi::c_char;
    let ptrcc :*const CCValue = args as *const CCValue;

	i= 0;
	while i < num {
		curptr = unsafe{*(argv.wrapping_add(i as usize))};
     
	    ores = unsafe { std::ffi::CStr::from_ptr(curptr).to_str()};
	    if ores.is_err() {
	        println!("RUST: error on {}",i);
	        return -1;
	    }
	    vecs.push(ores.unwrap().to_string());
		i += 1;
	}

	println!("RUST:iv={}",unsafe{(*ptrcc).iv});
	println!("RUST:strval={}",unsafe{(*ptrcc).strval.clone()});
	i = 0;
	while (i as usize) < vecs.len() {
		println!("RUST:a{}={}",i,vecs[i as usize]);
		i += 1;
	}
	return num;
}

fn dlcallstkbackarg_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let numparam :u64;
	let retval : std::ffi::c_int;
	let mut idx :usize;
	let mut params :Vec<String> = vec![];
	let funcname :String;
	let ptrcc :CCValue;
	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	if sarr.len() < 3 {
		extargs_new_error!{DlError,"need dll/so funcname numparam [params]..."}
	}

	let lib :libloading::Library;
	lib = unsafe { libloading::Library::new(&sarr[0])?};


	/*should get the name*/
	numparam = parse_u64(&sarr[2])?;
	funcname = format!("{}\0",sarr[1]);

	idx = 3;
	while idx < sarr.len() {
		params.push(format!("{}\0",sarr[idx]));
		idx += 1;
	}

	while params.len() < (numparam as usize) {
		params.push(format!("\0"));
	}

	ptrcc = CCValue::new(numparam as i32,&format!("with arg {}",numparam));

    if numparam == 0 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void);
        }
    } else if numparam == 1 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 2 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 3 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 4 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 5 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 6 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 7 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 8 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 9 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 10 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 11 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 12 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 13 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 14 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 15 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 16 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 17 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char);
        }
    } else if numparam == 18 {
        unsafe {
            let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char,*const std::ffi::c_char) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;
            retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,params[0].as_ptr() as *const std::ffi::c_char,params[1].as_ptr() as *const std::ffi::c_char,params[2].as_ptr() as *const std::ffi::c_char,params[3].as_ptr() as *const std::ffi::c_char,params[4].as_ptr() as *const std::ffi::c_char,params[5].as_ptr() as *const std::ffi::c_char,params[6].as_ptr() as *const std::ffi::c_char,params[7].as_ptr() as *const std::ffi::c_char,params[8].as_ptr() as *const std::ffi::c_char,params[9].as_ptr() as *const std::ffi::c_char,params[10].as_ptr() as *const std::ffi::c_char,params[11].as_ptr() as *const std::ffi::c_char,params[12].as_ptr() as *const std::ffi::c_char,params[13].as_ptr() as *const std::ffi::c_char,params[14].as_ptr() as *const std::ffi::c_char,params[15].as_ptr() as *const std::ffi::c_char,params[16].as_ptr() as *const std::ffi::c_char,params[17].as_ptr() as *const std::ffi::c_char);
        }
    } else {
        extargs_new_error!{DlError,"not supported {}",numparam}
    }


	println!("call [{}].[{}] retval {}", sarr[0],sarr[1],retval);
	Ok(())
}

#[extargs_map_function(dlcallint_handler,dlcallptr_handler,dlcallstr_handler,dlcallback_handler,dlcallstkback_handler,dlcallstkbackarg_handler)]
pub fn load_dlopen_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"dlcallint<dlcallint_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		},
		"dlcallptr<dlcallptr_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		},
		"dlcallstr<dlcallstr_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		},
		"dlcallback<dlcallback_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		},
		"dlcallstkback<dlcallstkback_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		},
		"dlcallstkbackarg<dlcallstkbackarg_handler>##dlfile funcname numcall args ... to call dlcall##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}