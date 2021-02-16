
pub fn sleep(cycles: u32) {
    for _ in 0..cycles {
        unsafe { asm!("nop"); }
    }
}