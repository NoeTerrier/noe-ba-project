// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;
use ext_crate::add;

module! {
    type: HelloWorld,
    name: "hello_world",
    author: "Noe Terrier",
    description: "Rust hello_world module",
    license: "GPL",
}

struct HelloWorld;

impl kernel::Module for HelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        
        pr_info!("Hello world! (init)\n");

        let answer = add(5, 10);
        pr_info!("Answer is : {}", answer);


        Ok(HelloWorld {})
    }
}

impl Drop for HelloWorld {
    fn drop(&mut self) {
        pr_info!("Goodbye! (exit)\n");
    }
}
