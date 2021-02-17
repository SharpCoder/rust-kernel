#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
pub mod lib;
pub mod gpio;

const CM_BASE: u32 = 0x44E00000;
const CM_CLKCTRL_OFFSET: u32 = 0xAC;

#[no_mangle]
pub fn main() {
    init();
    for i in 21 .. 24 {
        gpio::configure(i, gpio::GpioMode::Output);
    }
    
    loop {
        for i in 21 ..= 24 {
            gpio::set(i, true);
            lib::sleep(5000000);
            gpio::set(i, false);
        }
        lib::sleep(5000000);
    }
}

fn init() {
    let clock_ptr = (CM_BASE + CM_CLKCTRL_OFFSET) as *mut u32;
    unsafe {
        *clock_ptr = 0x2;
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