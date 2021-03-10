#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod sys;
mod board;
mod debug;

use board::{clocks, platform, timer, interrupts};

pub struct SystemContext {
    pub sysclock: timer::Timer,
}

static mut CONTEXT: SystemContext = SystemContext {
    sysclock: timer::Timer::new(platform::DMTIMER2),
};

pub fn get_context() -> &'static mut SystemContext {
    unsafe { return &mut CONTEXT };
}

#[no_mangle]
pub fn kmain() {

    initialize_platform();
    initialize_interrupts();

    loop {
        debug::emit(b"hello world");
        unsafe {
            asm!("nop");
        }
    }    
}

fn initialize_platform() {
    clocks::enable_clock_devices(&[
        clocks::CM_PER_L4LS_CLKSTCTRL,
        clocks::CM_PER_L4LS_CLKCTRL,
        clocks::CM_PER_GPIO1_CLKCTRL,
        clocks::CM_PER_EMIF_CLKCTRL,
        clocks::CM_PER_TIMER2_CLKCTRL,
        clocks::CM_PER_TIMER3_CLKCTRL,
    ]);

    // Set GPIO pins for USR LED's to output
    for i in 21 ..= 24 {
        board::gpio::configure(i, board::gpio::GpioMode::Output);
    }

    // Set the on gpio pin
    board::gpio::set(24, true);
}

fn initialize_interrupts() {
    let context = get_context();

    // Enable DMTimer2
    context.sysclock.stop();
    context.sysclock.set_load_value(0xFFFF_FFDF);
    context.sysclock.set_value(0xFFFF_FFDF);
    context.sysclock.configure(timer::ENABLE_AUTO_RELOAD | timer::IRQ_OVERFLOW_MODE);
    context.sysclock.irq_enable();

    // Wire up register
    interrupts::register_handler(interrupts::INT_DMTIMER2, handle_timer_irq);
    interrupts::unmask_interrupt(interrupts::INT_DMTIMER2);

    // Start the clock
    context.sysclock.start();
}

fn handle_timer_irq() {
    let context = get_context();

    context.sysclock.irq_disable();
    context.sysclock.stop();
    context.sysclock.irq_acknowledge();
    context.sysclock.irq_clear();
    context.sysclock.set_value(0xFFFF_FFDF);
    context.sysclock.incr();
    context.sysclock.irq_enable();
    context.sysclock.start();
}

#[no_mangle]
fn handle_irq_rust() {
    let int_number = interrupts::get_active_irq_number();
    interrupts::service_handler(int_number);
    interrupts::clear_interrupts();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() { }

#[panic_handler]
#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }

#[no_mangle]
pub extern "C" fn _ZN4core9panicking18panic_bounds_check17h2af2af33179e0cf0E() { }