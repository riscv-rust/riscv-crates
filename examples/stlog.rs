#![no_std]
#![feature(never_type)]
#![feature(used)]

extern crate hifive;
#[macro_use]
extern crate nb;
#[macro_use]
extern crate riscv_semihosting;
#[macro_use]
extern crate stlog;

use hifive::prelude::*;
use hifive::{Peripherals, Serial, UExt};
use hifive::e310x::UART0;

/// JTAG Logger implementation
pub struct JtagLogger;

impl stlog::Logger for JtagLogger {
    type Error = !;

    fn log(&self, addr: u8) -> Result<(), !> {
        const STDOUT: usize = 1;
        unsafe { syscall!(WRITE, STDOUT, *&addr, 1) };
        Ok(())
    }
}

/// UART Logger implementation
pub struct UartLogger {
    uart: UART0
}

impl UartLogger {
    pub fn new(uart: UART0) -> Self {
        UartLogger { uart }
    }
}

impl stlog::Logger for UartLogger {
    // required: the error type must be `!`
    type Error = !;

    fn log(&self, addr: u8) -> Result<(), !> {
        let serial = Serial(&self.uart);
        block!(serial.write(addr))
    }
}

fn main() {
    let p = Peripherals::take().unwrap();
    {
        let serial = Serial(&p.UART0);
        serial.init(115_200.hz().invert(), &p.GPIO0);
    }
    let logger = UartLogger::new(p.UART0);
    info!(logger, "Hello, world!");
    warn!(logger, "The quick brown fox jumps over the lazy dog.");
}
