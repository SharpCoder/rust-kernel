#![allow(dead_code)]

type Ptr = fn();

pub const INT_USB0: usize = 18;
pub const INT_USB1: usize = 19;
pub const INT_DMTIMER0: usize = 66;
pub const INT_DMTIMER2: usize = 68;
pub const INT_UART0: usize = 72;

const INTC_SIR_IRQ_REG: u32 = 0x40;

static mut VECTOR_HANDLERS: [Ptr; 128] = [noop; 128];
static MIRN_BANK_ADDRESSES: [u32;4] = [0x88,0xA8,0xC8,0xE8];

fn get_mirn_address(int_number: usize) -> u32 {
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

    return crate::board::platform::INTCPS + MIRN_BANK_ADDRESSES[bank];
}

pub fn get_active_irq_number() -> usize {
    return crate::sys::read_word(crate::board::platform::INTCPS + INTC_SIR_IRQ_REG) as usize;
}

pub fn unmask_interrupt(int_number: usize) {
    let address = get_mirn_address(int_number);
    let current_value = crate::sys::read_word(address);
    let next_value = crate::sys::set_bit(current_value, int_number as u8);
    crate::sys::assign(address, next_value);

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
    crate::sys::assign(crate::board::platform::INTCPS + 0x48, 0x1);
}

pub fn noop() {

}