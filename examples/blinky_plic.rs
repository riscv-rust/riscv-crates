#![no_std]

extern crate riscv;
extern crate hifive;

use hifive::hal::prelude::*;
use hifive::hal::e310x;
use hifive::hal::stdout::*;
use riscv::interrupt;

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
        &mut gpio.iof_en,
    );
    let (_red, _green, _blue) = hifive::rgb(
        gpio.pin22,
        gpio.pin19,
        gpio.pin21,
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en,
    );

    let serial = Serial::uart0(p.UART0, (tx, rx), 115_200.bps(), clocks);
    let (mut tx, _) = serial.split();
    let _stdout = Stdout(&mut tx);

    //let plic = Plic(&p.PLIC);
    //plic.init();

    //RtcConf::new().end(&p.RTC);
    //Rtc(&p.RTC).set_timeout(500.ms());
    /*
    plic.set_priority(Interrupt::RTC, Priority::P7);
    plic.enable(Interrupt::RTC);

    writeln!(stdout, "External interrupts enabled: {}",
             csr::mie.read().mext()).unwrap();
    let threshold: u32 = plic.get_threshold().into();
    writeln!(stdout, "PLIC threshold priority: {}",
             threshold).unwrap();
    writeln!(stdout, "RTC interrupt number: {}",
             Interrupt::RTC.nr()).unwrap();
    writeln!(stdout, "RTC interrupt enabled: {}",
             plic.is_enabled(Interrupt::RTC)).unwrap();
    let priority: u32 = plic.get_priority(Interrupt::RTC).into();
    writeln!(stdout, "RTC interrupt priority: {}",
             priority).unwrap();*/

    unsafe {
        interrupt::enable();
    }
}

/*#[no_mangle]
pub fn plic_trap_handler(p: &Peripherals, intr: &Interrupt) {
    match *intr {
        Interrupt::RTC => {
            Rtc(&p.RTC).restart();
            Blue::toggle(&p.GPIO0);
        },
        _ => {},
    }
}*/
