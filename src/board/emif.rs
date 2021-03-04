use crate::board::memorymap::{
    CM_PER,
};

pub fn init() {
    let ptr = (CM_PER + 0x28) as *mut u32;
    unsafe { *ptr = 0x2; }
}
