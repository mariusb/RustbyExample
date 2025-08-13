#![allow(unused)]
fn main() {
#[cfg(target_arch = "aarch64")] {
use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
println!("x: {}", x);
assert_eq!(x, 5);
}
}