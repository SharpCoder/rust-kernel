/*
    This outlines the ARM CORTEX-A8 memory mapping
    as defined in the Technical Reference Manual. Check out
    page 177 for the start of the memory mappings.
*/
pub const CM_PER: u32 = 0x44E0_0000; // Clock Module Peripheral Registers
pub const GPIO1: u32 = 0x4804_C000; // GPIO1 Registers
pub const DMTIMER0: u32 = 0x44E0_5000; // DMTimer0 Registers