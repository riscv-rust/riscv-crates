#![no_std]

extern crate hifive;

use hifive::prelude::*;
use hifive::{interrupt, led, Blue, Clint, Peripherals, UExt};

fn main() {
    let p = Peripherals::take().unwrap();
    led::init(&p.GPIO0);

    let timer = Clint(&p.CLINT);
    timer.set_timeout(1.s());

    unsafe {
        interrupt::enable();
    }
}

#[no_mangle]
pub fn mtimer_trap_handler(p: &Peripherals) {
    Clint(&p.CLINT).restart();
    Blue::toggle(&p.GPIO0);
}
