#![allow(unused)]
fn main() {
#[cfg(target_arch = "x86_64")] {
use std::arch::asm;

let i: u64 = 3;
let o: u64;
unsafe {
    asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
    );
}
assert_eq!(o, 8);
println!("o: {}", o);
println!("i: {}", i);
}
}