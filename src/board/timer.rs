/* 
    This submodule interfaces with the on-board 32,768-Hz timer to
    produce 1ms interrupts which will be utilized for a proper
    sleep mechanism. 
*/
#![allow(dead_code)]
use crate::board::memorymap::DMTIMER0;
const TCLR_REG: u32 = 0x38;
const TCRR_REG: u32 = 0x3C;
const TIOCP_CFG: u32 = 0x10;

pub fn init() {
    let tiocp_ptr = (DMTIMER0 + TIOCP_CFG) as *mut u32;
    unsafe { *tiocp_ptr = 0x00; }
}

pub fn start_timer() {
    let ptr = (DMTIMER0 + TCLR_REG) as *mut u32;
    unsafe { *ptr = 0x01; }
}

pub fn stop_timer() {
    let ptr = (DMTIMER0 + TCLR_REG) as *mut u32;
    unsafe { *ptr = 0x00; }
}

pub fn reset_timer() {
    let ptr = (DMTIMER0 + TCRR_REG) as *mut u32;
    unsafe { *ptr = 0x00; }
    // Wait a few cycle
    for _ in 0 ..= 5000 {
        unsafe { asm!("nop"); }
    }
}

pub fn read_timer() -> u32  {
    let ptr = (DMTIMER0 + TCRR_REG) as *mut u32;
    unsafe { 
        return *ptr; 
    };
}

pub fn wait(nano: u32) {
    start_timer();
    // TODO: Determine mathematically, why it needs 4 cycles extra
    // to be temporally accurate.
    let target = read_timer() + nano * 4;
    loop {
        if read_timer() >= target {
            break;
        } else {
            unsafe { asm!("nop"); }
        }
    }
}

pub fn wait_ms(ms: u32) {
    wait(ms * 10);
}