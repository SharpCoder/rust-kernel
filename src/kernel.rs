#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
pub mod lib;

const GPIO_BASE: u32 = 0x4804C000;
const GPIO_OE_OFFSET: u32 = 0x134;
const GPIO_DATAOUT: u32 = 0x13C;
const CM_BASE: u32 = 0x44E00000;
const CM_CLKCTRL_OFFSET: u32 = 0xAC;

#[no_mangle]
pub fn main() {
    loop {
        init();
        set_led(true);
        lib::sleep(5000000);
        set_led(false);
        lib::sleep(5000000);
    }
}

fn set_led(on: bool) {
    let gpio_ptr = (GPIO_BASE + GPIO_DATAOUT) as *mut u32;
    if on {
        unsafe { *gpio_ptr = 0xFFFFFFFF; }
    } else {
        unsafe { *gpio_ptr = 0x0; }
    }
}

fn init() {
    let clock_ptr = (CM_BASE + CM_CLKCTRL_OFFSET) as *mut u32;
    let gpio_oe_ptr = (GPIO_BASE + GPIO_OE_OFFSET) as *mut u32;
    unsafe {
        *clock_ptr = 0x2;
        *gpio_oe_ptr = 0x00;
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }