#![no_std]

extern crate hifive;

use core::fmt::Write;
use hifive::{Peripherals, Port, Serial, UExt};

fn main() {
    let p = Peripherals::take().unwrap();

    let serial = Serial(&p.UART0);
    serial.init(115_200.hz().invert(), &p.GPIO0);

    writeln!(Port(&serial), "hello world!").unwrap();
}
