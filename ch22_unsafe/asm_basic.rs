#![allow(unused)]
fn main() {
#[cfg(target_arch = "aarch64")] {
use std::arch::asm;

unsafe {
    asm!("nop");
}
}
}