#![allow(unused)]
fn main() {
#[cfg(target_arch = "x86_64")] {
use std::arch::asm;

let mut x: u16 = 0xab;

unsafe {
    asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
}

assert_eq!(x, 0xabab);
}
}