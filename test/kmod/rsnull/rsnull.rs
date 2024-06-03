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

unsafe impl Send for RsNullMod{}
unsafe impl Sync for RsNullMod{}

unsafe extern "C" fn null_open() -> core::ffi::c_int {
    
}

fn new_null_fop() -> Option<bindings::file_operations> {
    let mut retv : bindings::file_operations = unsafe {core::mem::zeroed()};
    retv.owner = &mut bindings::__this_module;
    retv.llseek = Some();
    return None;
}


impl kernel::Module for RsNullMod {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Null (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        let mut retv :RsNullMod = RsNullMod {
            fop : None,
            register : false,
        };

        retv.fop = new_null_fop();
        if retv.fop.is_none() {
            return Err(error::code::ENOMEM);
        }

        Ok(retv)
    }
}

impl Drop for RsNullMod {
    fn drop(&mut self) {
        pr_info!("Rust Null (exit)\n");
        if self.register {

        }
    }
}
