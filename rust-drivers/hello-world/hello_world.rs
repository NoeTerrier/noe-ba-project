// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustOutOfTree,
    name: "hello_world",
    author: "Noe Terrier",
    description: "Rust hello_world module",
    license: "GPL",
}

struct RustOutOfTree {
   
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world! (init)\n");

        Ok(RustOutOfTree {})
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("Goodbye! (exit)\n");
    }
}
