use lib;

const GPIO_BASE: u32 = 0x4804C000;
const GPIO_OE_OFFSET: u32 = 0x134;
const GPIO_DATAOUT: u32 = 0x13C;

static mut GPIO_MODE_BITS: u32 = 0xFF;
static mut GPIO_VAL: u32 = 0x00;

pub enum GpioMode {
    Input,
    Output,
}

pub fn configure(pin: u8, mode: GpioMode) {
    match mode {
        GpioMode::Input => {
            unsafe { GPIO_MODE_BITS = lib::set_bit(GPIO_MODE_BITS, pin); }
        },
        GpioMode::Output => {
            unsafe { GPIO_MODE_BITS = lib::clear_bit(GPIO_MODE_BITS, pin); }
        },
    }

    let gpio_oe_ptr = (GPIO_BASE + GPIO_OE_OFFSET) as *mut u32;
    unsafe { *gpio_oe_ptr = GPIO_MODE_BITS; }
}

pub fn set(pin: u8, val: bool) {
    match val {
        true => {
            unsafe { GPIO_VAL = lib::set_bit(GPIO_VAL, pin); }
        },
        false => {
            unsafe { GPIO_VAL = lib::clear_bit(GPIO_VAL, pin); }
        },
    }
    
    let gpio_ptr = (GPIO_BASE + GPIO_DATAOUT) as *mut u32;
    unsafe { *gpio_ptr = GPIO_VAL; }
}