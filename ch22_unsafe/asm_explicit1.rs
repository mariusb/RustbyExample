#![allow(unused)]
fn main() {
#[cfg(target_arch = "x86_64")] {
use std::arch::asm;

let cmd = 0xd1;
unsafe {
    asm!("out 0x64, eax", in("eax") cmd);
}
}
}