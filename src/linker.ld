/* 
  sram is defined as position 402F0400 per 2.1 in the documentation. 
  It contains a length of 64kb
*/
MEMORY {
    sram (rwx) : ORIGIN = 0x402F0400, LENGTH = 0xFFFF
}

/* Section variables */
ENTRY(main)
SECTIONS {
  .text : { *(.text*) } > sram
  .bss : { *(.bss*) } > sram
  .data : { *(.data*) } > sram
  _end = . ;
}