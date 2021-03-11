#![allow(dead_code)]

use crate::board::platform;
use crate::sys::{set_bit, read_word, assign};

type Ptr = fn();

pub const INT_USB0: usize = 18;
pub const INT_USB1: usize = 19;
pub const INT_DMTIMER0: usize = 66;
pub const INT_DMTIMER2: usize = 68;
pub const INT_DMTIMER3: usize = 69;
pub const INT_UART0: usize = 72;
pub const INT_DMTIMER4: usize = 92;
pub const INT_DMTIMER7: usize = 95;

const INTC_SIR_IRQ_REG: u32 = 0x40;

static mut VECTOR_HANDLERS: [Ptr; 128] = [noop; 128];
static MIRN_CLEAR_BANK_ADDRESSES: [u32;4] = [0x88,0xA8,0xC8,0xE8];

fn get_address_bank(int_number: usize) -> usize {
    let mut int_number = int_number;
    let mut bank = 0;
    loop {
        if int_number > 32 {
            int_number -= 32;
            bank += 1;
        } else {
            break;
        }
    }

    return bank;
}

pub fn get_active_irq_number() -> usize {
    return read_word(platform::INTCPS + INTC_SIR_IRQ_REG) as usize;
}

pub fn unmask_interrupt(int_number: usize) {
    let bank = get_address_bank(int_number);
    let mirn_clear_address = platform::INTCPS + MIRN_CLEAR_BANK_ADDRESSES[bank];
    let next_value = set_bit(0x0, int_number as u8);
    assign(mirn_clear_address, next_value);
}

pub fn register_handler(int_number: usize, method: Ptr) {
    unsafe {
        VECTOR_HANDLERS[int_number] = method;
    }
}

pub fn service_handler(int_number: usize) {
    unsafe {
        VECTOR_HANDLERS[int_number]();
    }
}

pub fn clear_interrupts() {
    crate::sys::assign(platform::INTCPS + 0x48, 0x1);
}

pub fn noop() {

}