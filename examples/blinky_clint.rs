#![no_std]

extern crate riscv;
extern crate hifive;

use core::sync::atomic::{AtomicBool, Ordering};
use hifive::hal::prelude::*;
use hifive::hal::e310x;
use hifive::hal::clint::{MTIME, MTIMECMP};
use riscv::interrupt;
use riscv::register::mcause::{Trap, Interrupt};

static CLINT_TIMEOUT: AtomicBool = AtomicBool::new(false);

fn set_mtimecmp(mtime: &MTIME, mtimecmp: &mut MTIMECMP) {
    let next = mtime.mtime() + 32768;
    mtimecmp.set_mtimecmp(next);
}

fn main() {
    let p = e310x::Peripherals::take().unwrap();
    let mut gpio = p.GPIO0.split();
    let (_red, _green, mut blue) = hifive::rgb(
        gpio.pin22,
        gpio.pin19,
        gpio.pin21,
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en,
    );

    let mut clint = p.CLINT.split();
    set_mtimecmp(&clint.mtime, &mut clint.mtimecmp);

    clint.mtimer.enable();

    unsafe {
        interrupt::enable();
    }

    loop {
        if CLINT_TIMEOUT.load(Ordering::Relaxed) {
            CLINT_TIMEOUT.store(false, Ordering::Relaxed);
            set_mtimecmp(&clint.mtime, &mut clint.mtimecmp);
            blue.toggle();
        }
    }
}

#[no_mangle]
pub fn trap_handler(trap: Trap) {
    match trap {
        Trap::Interrupt(Interrupt::MachineTimer) => {
            CLINT_TIMEOUT.store(true, Ordering::Relaxed);
        },
        _ => {}
    }
}
