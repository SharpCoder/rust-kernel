pub mod lists;
pub mod mem;

pub fn sleep(cycles: u32) {
    for _ in 0..cycles {
        unsafe { asm!("nop"); }
    }
}

pub fn clear_bit(number: u32, bit: u8) -> u32 {
    return number & !(0x01 << bit);
}

pub fn set_bit(number: u32, bit: u8) -> u32 {
    return number | (0x01 << bit);
}