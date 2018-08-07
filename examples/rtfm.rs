#![no_std]
#![feature(proc_macro_gen)]
#![feature(use_extern_macros)]
#![feature(extern_prelude)]

extern crate riscv;
extern crate hifive;
extern crate riscv_rtfm as rtfm;
extern crate riscv_rtfm_macros as rtfm_macros;
#[macro_use]
extern crate riscv_semihosting;
extern crate numtoa;

use hifive::hal::prelude::*;
use rtfm::{Threshold, Resource};
use riscv::register::{mcause, mepc};
use hifive::hal::plic::Priority;
use numtoa::NumToA;

macro_rules! debug_print {
    ($str:expr) => {
        {
            // File descriptor (on the host)
            let stdout: usize = 1;
            static MSG: &'static [u8] = $str;
            unsafe {
                syscall!(WRITE, stdout, MSG.as_ptr(), MSG.len());
            }
        }
    }
}

macro_rules! debug_print_numtoa {
    ($num:expr, $base:expr) => {
        {
            // File descriptor (on the host)
            let stdout: usize = 1;
            let mut num_buf = [0u8; 32];
            ($num as u32).numtoa($base, &mut num_buf);
            unsafe {
                syscall!(WRITE, stdout, num_buf.as_ptr(), num_buf.len());
            }
        }
    }
}

rtfm_macros::app! {
    device: hifive::hal::e310x,

    resources: {
        static TX: hifive::TX;
        static RX: hifive::RX;
        static RED: hifive::RED;
        static GREEN: hifive::GREEN;
        static BLUE: hifive::BLUE;
        static RTC: hifive::hal::rtc::Rtc;
        static COLOR_CNT: u8;
    },

    idle: {
        resources: [RTC, TX],
    },

    tasks: {
        rtc: {
            path: rtc,
            resources: [RTC, RED, GREEN, BLUE, COLOR_CNT],
        },
        uart0: {
            path: uart0,
            resources: [TX, RX],
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut clint = p.CLINT.split();
    let mut plic = p.PLIC.split();

    // Configure clocks
    let clocks = Clocks::freeze(p.PRCI.constrain(),
                                p.AONCLK.constrain(),
                                &clint.mtime);
    // Setup gpio pins
    let mut gpio = p.GPIO0.split();
    let (tx, rx) = hifive::tx_rx(
        gpio.pin17,
        gpio.pin16,
        &mut gpio.out_xor,
        &mut gpio.iof_sel,
        &mut gpio.iof_en
    );
    let (red, green, blue) = hifive::rgb(
        gpio.pin22,
        gpio.pin19,
        gpio.pin21,
        &mut gpio.output_en,
        &mut gpio.drive,
        &mut gpio.out_xor,
        &mut gpio.iof_en,
    );

    // Configure uart0
    let serial = Serial::uart0(p.UART0, (tx, rx), 115_200.bps(), clocks).listen();
    let (tx, rx) = serial.split();

    // Disable watchdog
    p.WDOG.configure().freeze().feed();

    // Configure rtc
    let mut rtc = p.RTC.constrain();
    let next = rtc.rtccmp() + 1;
    rtc.set_rtccmp(next);
    rtc.set_scale(15);
    rtc.enable();

    // Configure interrupts
    plic.threshold.set(Priority::P0);
    clint.mtimer.disable();
    plic.mext.enable();

    init::LateResources {
        TX: tx,
        RX: rx,
        RED: red,
        GREEN: green,
        BLUE: blue,
        RTC: rtc,
        COLOR_CNT: 0,
    }
}

fn idle(t: &mut rtfm::Threshold, r: idle::Resources) -> ! {
    loop {
        let cmp = r.RTC.claim(t, |rtc, _t| {
            rtc.rtccmp()
        });
        debug_print_numtoa!(cmp, 10);
        //rtfm::wfi();
    }
}

fn rtc(_t: &mut Threshold, mut r: rtc::Resources) {
    let rtc: &mut hifive::hal::rtc::Rtc = &mut r.RTC;
    let red: &mut hifive::RED = &mut r.RED;
    let green: &mut hifive::GREEN = &mut r.GREEN;
    let blue: &mut hifive::BLUE = &mut r.BLUE;

    match *r.COLOR_CNT {
        0 => red.toggle(),
        1 => green.toggle(),
        _ => blue.toggle(),
    }

    *r.COLOR_CNT = (*r.COLOR_CNT + 1) % 3;

    let cmp = rtc.rtccmp() + 1;
    rtc.set_rtccmp(cmp);
}

fn uart0(_t: &mut Threshold, mut _r: uart0::Resources) {
    //(*r.TX).write((*r.RX).read().unwrap()).unwrap();
}

#[no_mangle]
pub fn trap_handler(trap: mcause::Trap) {
    match trap {
        mcause::Trap::Interrupt(mcause::Interrupt::MachineExternal) => {
            let _plic: hifive::hal::e310x::PLIC = unsafe { core::mem::transmute(()) };
            let mut plic = _plic.split();
            let intr = plic.claim.claim();

            match intr {
                Some(hifive::hal::e310x::Interrupt::RTC) => unsafe { _rtc(); },
                _ => {},
            }

            if intr.is_some() {
                debug_print!(b"p ");
                debug_print_numtoa!(intr.unwrap(), 10);
                debug_print!(b"\n");
                plic.claim.complete(intr.unwrap());
            } else {
                debug_print!(b"p none\n");
            }
        },
        mcause::Trap::Interrupt(intr) => {
            debug_print!(b"i ");
            debug_print_numtoa!(intr, 10);
            debug_print!(b"\n");
        },
        mcause::Trap::Exception(ex) => {
            debug_print!(b"e ");
            debug_print_numtoa!(ex, 10);
            debug_print!(b" at 0x");
            debug_print_numtoa!(mepc::read(), 16);
            debug_print!(b"\n");
        },
    }
}
