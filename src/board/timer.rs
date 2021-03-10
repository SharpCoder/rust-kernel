/* 
    This submodule interfaces with the on-board 32,768-Hz timer to
    produce 1ms interrupts which will be utilized for a proper
    sleep mechanism. 
*/
#![allow(dead_code)]
const TIOCP_CFG: u32 = 0x10;
const IRQ_EOI_REG: u32 = 0x20;
const IRQENABLE_SET_REG: u32 = 0x2C;
const IRQSTATUS_REG: u32 = 0x28;
const IRQENABLE_CLR_REG: u32 = 0x30;
const TCLR_REG: u32 = 0x38;
const TCRR_REG: u32 = 0x3C;
const TLDR_REG: u32 = 0x40;
const TTRG_REG: u32 = 0x44;

pub const ENABLE_AUTO_RELOAD: u32 = 0x1;
pub const IRQ_MATCH_MODE: u32 = 0x2;
pub const IRQ_OVERFLOW_MODE: u32 = 0x4;
pub const IRQ_CAPTURE_MODE: u32 = 0x8;


pub struct Timer {
    base_addr: u32,
    pub config_mask: u32,
    ticks: u32,
}

impl Timer {
    pub const fn new(base_addr: u32) -> Self {
        return Timer {
            base_addr: base_addr,
            config_mask: 0x0,
            ticks: 0,
        };
    }

    pub fn incr(&mut self) {
        self.ticks += 1;
    }

    pub fn elapsed(&self) -> u32 {
        return self.ticks;
    }

    pub fn configure(&mut self, bit_mask: u32) {
        self.config_mask = bit_mask;
    }    

    pub fn start(&self) {
        // Initialize
        crate::sys::assign(self.base_addr + TIOCP_CFG, 0x0);
        crate::sys::assign(self.base_addr + TCLR_REG, 0x1 | ((0x1 & self.config_mask) << 1));
    }

    pub fn stop(&self) {
        crate::sys::assign(self.base_addr + TCLR_REG, 0x0);
    }

    pub fn set_load_value(&self, value: u32) {
        crate::sys::assign(self.base_addr + TLDR_REG, value);
    }

    pub fn set_value(&self, value: u32) {
        crate::sys::assign(self.base_addr + TTRG_REG, value);
        crate::sys::assign(self.base_addr + TCRR_REG, value);
    }

    pub fn irq_enable(&self) {
        crate::sys::assign(self.base_addr + IRQENABLE_SET_REG, (0xE & self.config_mask) >> 1);
    }

    pub fn irq_disable(&self) {
        crate::sys::assign(self.base_addr + IRQENABLE_CLR_REG, 0x2);
    }

    pub fn irq_clear(&self) {
        crate::sys::assign(self.base_addr + IRQSTATUS_REG, 0x7);
    }

    pub fn irq_acknowledge(&self) {
        crate::sys::assign(self.base_addr + IRQ_EOI_REG, 0x0);
    }
}

#[cfg(test)]
mod test_timer {

    use super::*;

    #[test]
    fn test_bit_shift() {
        assert_eq!((0xE & IRQ_MATCH_MODE) >> 1, 0x1);
        assert_eq!((0xE & IRQ_OVERFLOW_MODE) >> 1, 0x2);
        assert_eq!((0xE & (IRQ_MATCH_MODE | IRQ_OVERFLOW_MODE)) >> 1, 0x3);
        assert_eq!( 0x1 | ((0x1 & 0x0) << 1), 0x1);
        assert_eq!( 0x1 | ((0x1 & (ENABLE_AUTO_RELOAD | IRQ_OVERFLOW_MODE | IRQ_MATCH_MODE)) << 1), 0x3);
    }
}