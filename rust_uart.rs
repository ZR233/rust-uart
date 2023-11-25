// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
use kernel::prelude::*;

use crate::uart::UART;
pub mod uart;



module! {
    type: RustUart,
    name: "Rust_UART",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustUart {
    uart: UART
}

impl kernel::Module for RustUart {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust UART (init)\n");

        let uart = UART::new();
        

        Ok(RustUart { uart })
    }
}

impl Drop for RustUart {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
