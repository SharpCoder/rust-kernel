#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod sys;
mod gpio;
mod debug;
mod timer;

const CM_BASE: u32 = 0x44E00000;
const CM_CLKCTRL_OFFSET: u32 = 0xAC;

#[no_mangle]
pub fn main() {
    init();
    // timer::init();
    // timer::start_timer();
    // sys::sleep(1000000);
    // timer::stop_timer();
    // let timer_val = timer::read_timer();

    loop {
        // Output the timer
        debug::emit_num(123);
        debug::emit(b" ");
    }
}

fn init() {
    // Enable GPIO1 clock
    let clock_ptr = (CM_BASE + CM_CLKCTRL_OFFSET) as *mut u32;
    unsafe { *clock_ptr = 0x2; }

    // Set GPIO pins for USR LED's to output
    for i in 21 ..= 24 {
        gpio::configure(i, gpio::GpioMode::Output);
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
        gpio::set(i, true);
    }

    loop { }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }