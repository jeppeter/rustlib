// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;

module! {
    type: RsNull,
    name: "rsnull",
    author: "jeppeter@gmail.com",
    description: "rust /dev/null in kernel",
    license: "GPL",
}

struct RsNull {    
}

impl kernel::Module for RsNull {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Null (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        Ok(RsNull{})
    }
}

impl Drop for RsNull {
    fn drop(&mut self) {
        pr_info!("Rust Null (exit)\n");
    }
}
