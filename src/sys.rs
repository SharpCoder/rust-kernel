pub mod lists;
pub mod mem;
pub mod context;

pub fn assign(address: u32, value: u32) {
    unsafe {
        *(address as *mut u32) = value;
    }
}

pub fn read_word(address: u32) -> u32 {
    unsafe {
        return *(address as *mut u32);
    }
}

pub fn clear_bit(number: u32, bit: u8) -> u32 {
    return number & !(0x01 << bit);
}

pub fn set_bit(number: u32, bit: u8) -> u32 {
    return number | (0x01 << bit);
}

pub fn millis() -> u32 {
    let context = context::get_context();
    return context.sysclock.elapsed();
}

pub fn wait_ms(ms: u32) {
    let target = millis() + ms;
    loop {
        if millis() > target {
            break;
        } else {
            unsafe {
                asm!("nop");
            }
        }
    }
}