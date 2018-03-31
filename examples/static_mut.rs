#![no_std]

extern crate riscv_rt;

static mut GLOB: bool = false;

fn main() {
    unsafe { GLOB = true; }
}
