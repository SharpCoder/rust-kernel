/*
    Author: Josh Cole
    This library is dedicated to onboard OCMC RAM. 64kb
*/
#![allow(dead_code)]
#[cfg(test)]
use std::alloc::{alloc, Layout};
use core::mem::{size_of};

// Per 2.1 in the AM335x Technical Reference Manual
const OCMC0_BASE_PTR: u32 = 0x4030_0000;
const OCMC0_MAX: u32 = 0x10000;
static mut OCMC0_OFFSET: u32 = 0;

/// Overwrite entire memoryspace with zeroes.
/// Is this useful? Idk.
pub fn zero_all() {
    let ptr = (OCMC0_BASE_PTR) as *mut u32;
    unsafe {
        for idx in 0 .. OCMC0_MAX / 4 {
            *ptr.offset(idx as isize) = 0;
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
        if OCMC0_OFFSET + bytes as u32 > OCMC0_MAX {
            OCMC0_OFFSET = 0;
        }
    
        let ptr = (OCMC0_BASE_PTR + OCMC0_OFFSET) as *mut T;
        OCMC0_OFFSET += bytes as u32;
        return ptr;
    }
}

/// Free a pointer by updating the pagefile
pub fn free<T>(ptr: *mut T) {
    let bytes = size_of::<T>();
    for i in 0 .. bytes / 4 {
        unsafe { 
            let zero_ptr = ptr as *mut u32;
            *(zero_ptr.offset(i as isize)) = 0;
        }
    }
}