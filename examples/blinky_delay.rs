#![no_std]

//#[macro_use]
//extern crate nb;
extern crate hifive;

use hifive::hal::prelude::*;
use hifive::hal::e310x;
use hifive::led::{Led, RED, GREEN, BLUE};

fn delay() {
    //block!(clint.wait()).unwrap();
    //clint.restart();
    for _i in 0..10000 {

    }
}

fn main() {
    let p = e310x::Peripherals::take().unwrap();
    let mut gpio = p.GPIO0.split();
    let mut red: RED = gpio.pin22.into_inverted_output(
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en
    );
    let mut green: GREEN = gpio.pin19.into_inverted_output(
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en
    );
    let mut blue: BLUE = gpio.pin21.into_inverted_output(
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en
    );

    //let clint = Clint(&p.CLINT);
    //clint.set_timeout(500.ms());

    loop {
        red.on();
        delay();
        red.off();
        green.on();
        delay();
        green.off();
        blue.on();
        delay();
        blue.off();
    }
}
