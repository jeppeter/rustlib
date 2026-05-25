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


#[extargs_map_function(dlcallint_handler,dlcallptr_handler,dlcallstr_handler)]
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
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}