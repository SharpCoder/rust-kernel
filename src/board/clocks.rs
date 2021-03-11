#![allow(dead_code)]

use crate::board::platform::CM_PER;

pub const CM_PER_L4LS_CLKSTCTRL: u32 = 0x0;
pub const CM_PER_L3S_CLKSTCTRL: u32 = 0x4;
pub const CM_PER_L3_CLKSTCTRL: u32 = 0xC;
pub const CM_PER_EMIF_CLKCTRL: u32 = 0x28;
pub const CM_PER_L4LS_CLKCTRL: u32 = 0x60;
pub const CM_PER_TIMER7_CLKCTRL: u32 = 0x7C;
pub const CM_PER_TIMER2_CLKCTRL: u32 = 0x80;
pub const CM_PER_TIMER3_CLKCTRL: u32 = 0x84;
pub const CM_PER_TIMER4_CLKCTRL: u32 = 0x88;
pub const CM_PER_GPIO1_CLKCTRL: u32 = 0xAC;
pub const CM_PER_GPIO2_CLKCTRL: u32 = 0xB0;
pub const CM_PER_GPIO3_CLKCTRL: u32 = 0xB4;
pub const CM_PER_L3_INSTR_CLKCTRL: u32 = 0xDC;
pub const CM_PER_L3_CLKCTRL: u32 = 0xE0;
pub const CM_PER_TIMER5_CLKCTRL: u32 = 0xEC;
pub const CM_PER_TIMER6_CLKCTRL: u32 = 0xF0;
pub const CM_PER_LCDC_CLKSTCTRL: u32 = 0x148;
pub const CM_PER_L4HS_CLKSTCTRL: u32 = 0x11C;
pub const CM_PER_L4HS_CLKCTRL: u32 = 0x120;


pub fn enable_clock_device(address: u32) {
    crate::sys::assign(CM_PER + address, 0x2);
}

pub fn enable_clock_devices(addresses: &[u32]) {
    for address in addresses {
        enable_clock_device(*address);
    }
}