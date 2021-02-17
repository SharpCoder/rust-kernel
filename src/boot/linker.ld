/* 
  sram is defined as position 402F0400 per 2.1 in the documentation. 
  It contains a length of 64kb
*/
MEMORY {
    rom (rw) : ORIGIN = 0x40000000, LENGTH = 0x1FFFF
    sram (rwx) : ORIGIN = 0x402F0400, LENGTH = 0x10000
}

/* Section variables */
STACK_SIZE = 0x2000;

ENTRY(main)
SECTIONS {
  .text : {
    *(.text*)
  } > sram

  .bss : {
    *(.bss*)
  } > sram

  .data : {
    *(.data*);
  } > sram

  .stack : {
    . = ALIGN(8);
    . = . + STACK_SIZE;
    . = ALIGN(8);
  } > sram

  _end = . ;
}