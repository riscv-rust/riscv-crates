#![no_std]

#[macro_use]
extern crate nb;
extern crate hifive;

use hifive::prelude::*;
use hifive::{led, Peripherals, Red, Green, Blue, Clint, UExt};

fn delay(clint: &Clint) {
    block!(clint.wait()).unwrap();
    clint.restart();
}

fn main() {
    let p = Peripherals::take().unwrap();
    led::init(&p.GPIO0);

    let clint = Clint(&p.CLINT);
    clint.set_timeout(500.ms());

    loop {
        Red::on(&p.GPIO0);
        delay(&clint);
        Red::off(&p.GPIO0);
        Green::on(&p.GPIO0);
        delay(&clint);
        Green::off(&p.GPIO0);
        Blue::on(&p.GPIO0);
        delay(&clint);
        Blue::off(&p.GPIO0);
    }
}
