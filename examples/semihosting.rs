#![no_std]

extern crate hifive;
#[macro_use]
extern crate riscv_semihosting;

fn main() {
    // File descriptor (on the host)
    const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
    static MSG: &'static [u8] = b"Hello, world!\n";

    // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
    let _r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
}
