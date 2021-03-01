#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod sys;
mod debug;
mod board;

#[no_mangle]
pub fn main() {
    init();
    loop {
        blink();
    }
}

fn blink() {
    debug::emit_num(1237);
    debug::emit(b" ");
}

fn init() {
    // Enable timer
    board::timer::init();
    board::gpio::init();

    // Set GPIO pins for USR LED's to output
    for i in 21 ..= 24 {
        board::gpio::configure(i, board::gpio::GpioMode::Output);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    // Turn on all USR LED's for obvious panic condition
    for i in 21..=24 {
        board::gpio::set(i, true);
    }

    loop { }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }