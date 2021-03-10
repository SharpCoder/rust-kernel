#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod sys;
mod board;
mod interrupts;

use board::{clocks, platform, timer};

static mut CLOCK: timer::Timer = timer::Timer::new(platform::DMTIMER2);

#[no_mangle]
pub fn kmain() {

    initialize_platform();
    initialize_interrupts();


    loop {
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
    ]);

    // Set GPIO pins for USR LED's to output
    for i in 21 ..= 24 {
        board::gpio::configure(i, board::gpio::GpioMode::Output);
    }

    // Set the on gpio pin
    board::gpio::set(24, true);
}

fn initialize_interrupts() {
    unsafe {
        // Enable DMTimer2
        CLOCK.stop();
        CLOCK.set_load_value(0xFFFF_0000);
        CLOCK.set_value(0xFFFF_0000);
        CLOCK.configure(timer::ENABLE_AUTO_RELOAD | timer::IRQ_OVERFLOW_MODE);
        CLOCK.irq_enable();

        // Wire up register
        interrupts::register_handler(interrupts::INT_DMTIMER2, handle_timer_irq);
        interrupts::unmask_interrupt(interrupts::INT_DMTIMER2);

        // Start the clock
        CLOCK.start();
    }
}

fn wait() {
    for _ in 0 ..= 5000000 {
        unsafe { asm!("nop"); }
    }
}

fn handle_timer_irq() {
    unsafe {
        CLOCK.irq_disable();
        CLOCK.stop();
        CLOCK.irq_acknowledge();
        CLOCK.irq_clear();
        CLOCK.set_value(0xFFFF_0000);
        CLOCK.incr();

        board::gpio::set(21, true);
        wait();
        board::gpio::set(21, false);
        wait();
        
        CLOCK.irq_enable();
        CLOCK.start();
    }
}

#[no_mangle]
fn handle_irq_rust() {
    board::gpio::set(24, true);
    let int_number = interrupts::get_active_irq_number();
    interrupts::service_handler(int_number);
    interrupts::clear_interrupts();
    board::gpio::set(24, false);
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