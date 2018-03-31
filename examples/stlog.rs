#![no_std]
#![feature(never_type)]
#![feature(used)]

extern crate hifive;
//#[macro_use]
extern crate nb;
#[macro_use]
extern crate riscv_semihosting;
#[macro_use]
extern crate stlog;

use hifive::hal::prelude::*;
use hifive::hal::e310x::{self, UART0};

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
pub struct UartLogger<T> {
    _tx: Tx<T>
}

impl<T> UartLogger<T> {
    pub fn new(tx: Tx<T>) -> Self {
        UartLogger { _tx: tx }
    }
}

impl stlog::Logger for UartLogger<UART0> {
    // required: the error type must be `!`
    type Error = !;

    fn log(&self, _addr: u8) -> Result<(), !> {
        //block!(self.tx.write(addr))
        Ok(())
    }
}

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
    let serial = Serial::uart0(p.UART0, (tx, rx), 115_200.bps(), clocks);
    let (tx, _) = serial.split();

    let logger = UartLogger::new(tx);
    info!(logger, "Hello, world!").unwrap();
    warn!(logger, "The quick brown fox jumps over the lazy dog.").unwrap();
}
