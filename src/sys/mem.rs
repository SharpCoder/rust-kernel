/*
    Author: Josh Cole
    This library is dedicated to onboard OCMC RAM. 64kb
*/
#![allow(dead_code, unused_imports)]
#[cfg(test)]
use std::alloc::{alloc, Layout};
use core::mem::{size_of};
use crate::board::memorymap::{
    EMIF0_SDRAM,
    KB64,
    GB1,
};

// Per 2.1 in the AM335x Technical Reference Manual
const MEMORY_MAXIMUM: u32 = GB1;
static mut MEMORY_OFFSET: u32 = 0;

pub fn memtest() {
    // Write one to every byte of memory and check for overflow.
    let ptr = EMIF0_SDRAM as *mut u32;
    for idx in 0 .. GB1 / 4 {
        unsafe { 
            *ptr.offset(idx as isize) = 0xFFFF_FFFE;
        }
    }
}

#[cfg(test)]
pub fn kalloc<T>() -> *mut T {
    return unsafe { alloc(Layout::new::<T>()) as *mut T };
}

#[cfg(not(test))]
/// This is kernal alloc and it implements what I call a "cyclical mempage strategy".
/// Memory is constantly allocated in RAM and eventually will loop back around
/// if all memory is used up. Clearly, this is a bad idea. Will be improved over time.
pub fn kalloc<T>() -> *mut T {
    let bytes = size_of::<T>();
    // Check for boundaries and reset if applicable.
    // TODO: this is definitely USR LED worthy. Maybe even panic worthy.
    unsafe {
        if MEMORY_OFFSET + bytes as u32 > MEMORY_MAXIMUM {
            MEMORY_OFFSET = 0;
        }
    
        let ptr = (EMIF0_SDRAM + MEMORY_OFFSET) as *mut T;
        MEMORY_OFFSET += bytes as u32;
        return ptr;
    }
}

/// Free a pointer by updating the pagefile
pub fn free<T>(ptr: *mut T) {
    let bytes = size_of::<T>();
    let zero_ptr = ptr as *mut u32;
    for i in 0 .. bytes / 4 {
        unsafe { 
            *(zero_ptr.offset(i as isize)) = 0;
        }
    }
}