#![allow(unused)]
fn main() {
#[cfg(target_arch = "x86_64")] {
use std::arch::asm;

fn load_fpu_control_word(control: u16) {
    unsafe {
        asm!("fldcw [{}]", in(reg) &control, options(nostack));
    }
}
}
}