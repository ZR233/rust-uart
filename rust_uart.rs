// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
use crate::uart::*;
use core::default::Default;
use core::ptr::null_mut;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::str::CString;
use kernel::sync::*;
use kernel::uart::Registration;
use port::Port;
mod linux;
pub mod port;
mod sbi_print;
pub mod uart;

const NR: i32 = 1;
const TTY_MAJOR: i32 = 240;
const TTY_MINOR: i32 = 5;
const DEV_NAME: &CStr = c_str!("ttyS");
use core::arch::asm;
pub use sbi_print::*;

module! {
    type: RustUart,
    name: "Rust_UART",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustUart {
    reg: Registration,
    uart: Pin<UniqueArc<UART8250>>,
}

#[pin_data]
struct UART8250 {
    #[pin]
    inner: SpinLock<UART8250Inner>,
}
struct UART8250Inner {
    platform_driver: platform_driver,
    ports: Vec<Port>,
}

unsafe impl Send for UART8250Inner {}

extern "C" fn prob(dev: *mut platform_device) -> i32 {
    pr_println!("prob");
    0
}
extern "C" fn remove(dev: *mut platform_device) -> i32 {
    pr_println!("remove");

    0
}

impl UART8250 {
    fn new() -> Result<impl PinInit<Self>> {
        let mut platform_driver = platform_driver::default();
        platform_driver.probe = Some(prob);
        platform_driver.remove = Some(remove);
        let mut ports = Vec::try_with_capacity(NR as _).unwrap();
        unsafe {
            let devs =
                platform_device_alloc(c_str!("serial8250").as_char_ptr(), PLAT8250_DEV_LEGACY);

            to_result(platform_device_add(devs))?;

            for _ in 0..NR {
                let port: Port = Port::new()?;
                port.register();
                ports.try_push(port)?;
            }

            platform_device_del(devs);
        }

        let inner = UART8250Inner {
            platform_driver,
            ports,
        };
        Ok(pin_init!( Self{
            inner <- new_spinlock!(inner)
        }))
    }

}

impl kernel::Module for RustUart {
    fn init(module: &'static ThisModule) -> Result<Self> {
        pr_println!("Rust UART (init)");

        let mut reg = Registration::default();
        reg.reg.nr = NR;
        reg.reg.major = TTY_MAJOR;
        reg.reg.minor = TTY_MINOR;
        reg.reg.dev_name = DEV_NAME.as_char_ptr();

        reg.register(module)?;

        let uart8250 = UART8250::new().unwrap();

        pr_println!("Rust UART init finish");
        pr_info!("init finish");

        let uart = UniqueArc::pin_init(uart8250).unwrap();
        Ok(RustUart { reg, uart })
    }

}

impl Drop for RustUart {
    fn drop(&mut self) {
        pr_info!("Rust UART (exit)\n");
    }
}
