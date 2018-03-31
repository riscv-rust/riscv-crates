#![no_std]

extern crate hifive;

use hifive::hal::prelude::*;
use hifive::hal::e310x;
use hifive::hal::stdout::*;

fn main() {
    let p = e310x::Peripherals::take().unwrap();

    let clint = p.CLINT.split();
    let clocks = Clocks::freeze(p.PRCI.constrain(),
                                p.AONCLK.constrain(),
                                &clint.mtime);
    let mut gpio = p.GPIO0.split();
    let (tx, rx) = hifive::tx_rx(
        gpio.pin17,
        gpio.pin16,
        &mut gpio.out_xor,
        &mut gpio.iof_sel,
        &mut gpio.iof_en
    );
    let serial = Serial::uart0(p.UART0, (tx, rx), 115_200.bps(), clocks);
    let (mut tx, _) = serial.split();

    writeln!(Stdout(&mut tx), "hello world!").unwrap();
}
