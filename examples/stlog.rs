#![no_std]
#![feature(never_type)]
#![feature(used)]

extern crate hifive;
#[macro_use]
extern crate nb;
#[macro_use]
extern crate stlog;

use hifive::prelude::*;
use hifive::Serial;

struct Logger;

impl stlog::Logger for Logger {
    // required: the error type must be `!`
    type Error = !;

    fn log(&self, addr: u8) -> Result<(), !> {
        let peripherals = hifive::init(115_200);
        let serial = Serial(peripherals.UART0);
        block!(serial.write(addr))
    }
}

set_global_logger!(Logger);

fn main() {
    info!("Hello, world!");
    warn!("The quick brown fox jumps over the lazy dog.");
}
