// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;
use kernel::error;
use kernel::types::{Opaque};
use kernel::bindings;

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
    fop : Option<bindings::file_operations>,
    register : bool,
}

unsafe impl Sync for RsNullMod{}
unsafe impl Send for RsNullMod{}

unsafe extern "C" fn null_open(_arg1 :*mut bindings::inode, _arg2 :*mut bindings::file) -> core::ffi::c_int {
    pr_info!("null_open");
    return 0;
}

unsafe extern "C" fn null_llseek(_arg1 :*mut bindings::file,_arg2 :bindings::loff_t,_arg3 : core::ffi::c_int) -> bindings::loff_t {
    pr_info!("null_llseek");
    return 0;
}

unsafe extern "C" fn null_write(_arg1 :*mut bindings::file, _arg2 :* const core::ffi::c_char, _arg3 : usize,_arg4 : *mut bindings::loff_t) -> isize {
    pr_info!("null_write");
    if _arg4 != core::ptr::null_mut() {
        unsafe {
            *_arg4 = 0;    
        }
    }
    return _arg3 as isize;
}

unsafe extern "C" fn null_release(_arg1 :*mut bindings::inode, _arg2 :*mut bindings::file)  -> core::ffi::c_int {
    pr_info!("null_release");
    return 0;
}



fn new_null_fop() -> Option<bindings::file_operations> {
    let mut retv : bindings::file_operations = unsafe {core::mem::zeroed()};
    unsafe {
        retv.owner = &mut bindings::__this_module;
    }
    
    retv.open = Some(null_open);
    retv.llseek = Some(null_llseek);
    retv.write = Some(null_write);
    retv.release = Some(null_release);
    return Some(retv);
}

const RS_NULL_MAJOR :core::ffi::c_uint = 30;
const RS_NULL_MINOR :core::ffi::c_uint = 12;

impl kernel::Module for RsNullMod {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Null (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        let mut retv :RsNullMod = RsNullMod {
            fop : None,
            register : false,
        };
        let rsnullname = kernel::c_str!("rsnull");

        retv.fop = new_null_fop();
        if retv.fop.is_none() {
            return Err(error::code::ENOMEM);
        }

        let reti :core::ffi::c_int;
        unsafe {
            let retop = retv.fop.as_ref().unwrap();
            reti = bindings::__register_chrdev(RS_NULL_MAJOR,RS_NULL_MINOR,1,rsnullname.as_char_ptr(),retop);
        }

        if reti < 0{
            pr_err!("register errno {}",reti);
            return Err(kernel::error::to_result(reti).err().unwrap());
        }
        retv.register = true;

        Ok(retv)
    }
}

impl Drop for RsNullMod {
    fn drop(&mut self) {
        let rsnullname = kernel::c_str!("rsnull");
        pr_info!("Rust Null (exit)\n");
        if self.register {
            unsafe {
                bindings::__unregister_chrdev(RS_NULL_MAJOR,RS_NULL_MINOR,1,rsnullname.as_char_ptr());    
            }            
        }
        self.register = false;
        self.fop = None;
        return;
    }
}
