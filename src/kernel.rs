#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
pub mod lib;
pub mod gpio;
pub mod debug;

const CM_BASE: u32 = 0x44E00000;
const CM_CLKCTRL_OFFSET: u32 = 0xAC;

#[no_mangle]
pub fn main() {
    init();
    loop {
        debug::emit(b"jurassic park");
        debug::emit(b"hello world");
    }
}

fn init() {
    // Enable clock
    let clock_ptr = (CM_BASE + CM_CLKCTRL_OFFSET) as *mut u32;
    unsafe { *clock_ptr = 0x2; }

    // Set GPIO pins to output
    gpio::configure(21, gpio::GpioMode::Output);
    gpio::configure(22, gpio::GpioMode::Output);
    gpio::configure(23, gpio::GpioMode::Output);
    gpio::configure(24, gpio::GpioMode::Output);
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    for i in 21..=24 {
        gpio::set(i, true);
    }

    loop { }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }