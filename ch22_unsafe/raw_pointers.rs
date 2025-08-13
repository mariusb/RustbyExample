fn main() {
    let raw_p: *const u32 = &10;

    println!("raw_p: {:?}", unsafe { *raw_p });
    unsafe {
        assert!(*raw_p == 10);
    }
}