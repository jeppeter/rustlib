// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;
#[allow(unused_imports)]
use kernel::error;
use kernel::types::{Opaque};
use kernel::bindings;

#[macro_use]
mod rsnull_macro;

//use crate::{debug_trace};

module! {
    type: RsNullMod,
    name: "rsnull",
    author: "jeppeter@gmail.com",
    description: "rust /dev/null in kernel",
    license: "GPL",
}

#[allow(dead_code)]
#[repr(transparent)]
struct RsNullFile(Opaque<bindings::file>);


struct RsNullMod{
    register : bool,
}

unsafe impl Sync for RsNullMod{}
unsafe impl Send for RsNullMod{}

unsafe extern "C" fn null_open(_arg1 :*mut bindings::inode, _arg2 :*mut bindings::file) -> core::ffi::c_int {
    debug_trace!("null_open");
    return 0;
}

unsafe extern "C" fn null_llseek(_arg1 :*mut bindings::file,_arg2 :bindings::loff_t,_arg3 : core::ffi::c_int) -> bindings::loff_t {
    debug_trace!("null_llseek");
    return 0;
}

unsafe extern "C" fn null_write(_arg1 :*mut bindings::file, _arg2 :* const core::ffi::c_char, _arg3 : usize,_arg4 : *mut bindings::loff_t) -> isize {
    debug_trace!("null_write");
    unsafe {
        *_arg4 += _arg3 as bindings::loff_t;
    }
    return _arg3 as isize;
}

unsafe extern "C" fn null_release(_arg1 :*mut bindings::inode, _arg2 :*mut bindings::file)  -> core::ffi::c_int {
    debug_trace!("null_release");
    return 0;
}



#[allow(dead_code)]
static mut RSNULL_FOP :Option<bindings::file_operations> = None;

fn new_null_fop() -> Option<bindings::file_operations> {
    let mut retv : bindings::file_operations = unsafe {core::mem::zeroed()};
    unsafe {
        retv.owner = &mut bindings::__this_module;
    }
    
    retv.open = Some(null_open);
    retv.llseek = Some(null_llseek);
    retv.write = Some(null_write);
    retv.release = Some(null_release);
    debug_trace!("retv {:p} open fn {:p}",&retv,(*retv.open.as_ref().unwrap()));
    let cptr :*const u8 = &retv as *const bindings::file_operations as *const u8;
    let clen :usize = core::mem::size_of::<bindings::file_operations>();
    debug_buffer_trace!(cptr,clen,"file_operations");
    return Some(retv);
}

const RS_NULL_MAJOR :core::ffi::c_uint = 30;
const RS_NULL_MINOR :core::ffi::c_uint = 12;

impl kernel::Module for RsNullMod {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        debug_trace!("Rust Null (init)\n");
        debug_trace!("Am I built-in? {}\n", !cfg!(MODULE));
        let mut retv :RsNullMod = RsNullMod {
            register : false,
        };
        let rsnullname = kernel::c_str!("rsnull");

        unsafe {
            RSNULL_FOP = new_null_fop();    
        }
        
        if unsafe {RSNULL_FOP.is_none()} {
            return Err(error::code::ENOMEM);
        }
        //retv.setfop = true;
        let ptr = unsafe {RSNULL_FOP.as_ref().unwrap()} as *const bindings::file_operations as *const u8;
        let clen = core::mem::size_of::<bindings::file_operations>();
        debug_buffer_trace!(ptr,clen,"RSNULL_FOP dump");

        let reti :core::ffi::c_int;
        unsafe {
            let retop = RSNULL_FOP.as_ref().unwrap() as *const bindings::file_operations;
            //debug_trace!("retop {:p} open {:p}",retop,*((*retop).open.as_ref().unwrap()));
            reti = bindings::__register_chrdev(RS_NULL_MAJOR,RS_NULL_MINOR,1,rsnullname.as_char_ptr(),retop);
        }

        if reti < 0{
            error_trace!("register errno {}",reti);
            return Err(kernel::error::to_result(reti).err().unwrap());
        }
        retv.register = true;
        debug_trace!("insert MAJOR {} MINOR {} retv {:p}",RS_NULL_MAJOR,RS_NULL_MINOR,&retv as *const RsNullMod);
        Ok(retv)
    }
}

impl Drop for RsNullMod {
    fn drop(&mut self) {
        let rsnullname = kernel::c_str!("rsnull");
        debug_trace!("Rust Null (exit)\n");
        if self.register {
            //debug_trace!("bindings::__unregister_chrdev before");
            unsafe {
                bindings::__unregister_chrdev(RS_NULL_MAJOR,RS_NULL_MINOR,1,rsnullname.as_char_ptr());    
            }
            //debug_trace!("bindings::__unregister_chrdev");
        }

        /*
        if unsafe {RSNULL_FOP.is_some()} {
            //debug_trace!("is_some before self {:p}", self as *const RsNullMod);
            //let retop = unsafe {RSNULL_FOP.as_ref().unwrap()} as *const bindings::file_operations;
            //debug_trace!("retop {:p}",retop);
            //let p = unsafe {*((*retop).open.as_ref().unwrap())};
            //debug_trace!("retop {:p} open {:p}",retop,p);
        } else {
            //debug_trace!("fop none self {:p}",self as *const RsNullMod);
        }*/
        self.register = false;
        unsafe {
            RSNULL_FOP = None;
        }
        debug_trace!("all over");
        return;
    }
}
