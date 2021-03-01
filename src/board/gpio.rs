#![allow(dead_code)]
use crate::sys;
use crate::board::memorymap::{CM_PER, GPIO1};

const CM_PER_GPIO1_CLKCTRL: u32 = 0xAC;
const GPIO_OE_OFFSET: u32 = 0x134;
const GPIO_DATAOUT: u32 = 0x13C;

static mut GPIO_MODE_BITS: u32 = 0xFF;
static mut GPIO_VAL: u32 = 0x00;

pub enum GpioMode {
    Input,
    Output,
}

pub fn init() {
    // Enable GPIO1 clock
    let clock_ptr = (CM_PER + CM_PER_GPIO1_CLKCTRL) as *mut u32;
    unsafe { *clock_ptr = 0x2; }
}

pub fn configure(pin: u8, mode: GpioMode) {
    match mode {
        GpioMode::Input => {
            unsafe { GPIO_MODE_BITS = sys::set_bit(GPIO_MODE_BITS, pin); }
        },
        GpioMode::Output => {
            unsafe { GPIO_MODE_BITS = sys::clear_bit(GPIO_MODE_BITS, pin); }
        },
    }

    let gpio_oe_ptr = (GPIO1 + GPIO_OE_OFFSET) as *mut u32;
    unsafe { *gpio_oe_ptr = GPIO_MODE_BITS; }
}

pub fn set(pin: u8, val: bool) {
    match val {
        true => {
            unsafe { GPIO_VAL = sys::set_bit(GPIO_VAL, pin); }
        },
        false => {
            unsafe { GPIO_VAL = sys::clear_bit(GPIO_VAL, pin); }
        },
    }
    
    let gpio_ptr = (GPIO1 + GPIO_DATAOUT) as *mut u32;
    unsafe { *gpio_ptr = GPIO_VAL; }
}