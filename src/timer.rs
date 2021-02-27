/* 
This submodule interfaces with the on-board 32,768-Hz timer to
produce 1ms interrupts which will be utilized for a proper
sleep mechanism. 
*/

const BASE_REG: u32 = 0x40000100; // Base offset
const TCLR_REG: u32 = 0x38; // Timer control register
const TCRR_REG: u32 = 0x3C; // Timer counter register
const TLDR_REG: u32 = 0x40; // Timer load register


pub fn init() {
    
}

pub fn start_timer() {
    // Enable GPIO1 clock
    let ptr = (BASE_REG + TCLR_REG) as *mut u32;
    unsafe { *ptr = 0x01; }
}

pub fn stop_timer() {
    // Enable GPIO1 clock
    let ptr = (BASE_REG + TCLR_REG) as *mut u32;
    unsafe { *ptr = 0x00; }
}

pub fn read_timer() -> u32  {
    let ptr = (BASE_REG + TCRR_REG) as *mut u32;
    unsafe { 
        return *ptr; 
    };
}
