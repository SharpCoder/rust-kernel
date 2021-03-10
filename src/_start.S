.global main

main:
    @ Assign the IRQ interrupt method
    LDR r0, base_irq_addr
    LDR r1, basic_handler
    STR r1,[r0]

    @ Setup default stack pointer, wherever the hell we are at
    ldr r0, stack_base
    mov sp, r0

    @ Setup sp in IRQ mode
    mov r0, #0xD2
    msr cpsr_c, r0
    ldr r0, stack_base
    ADD r0, #1024
    mov sp, r0

    @ Enter supervisor mode, irq disabled
    mov r0, #0xD3
    msr cpsr_c, r0
    ldr r0, stack_base
    ADD r0, #1024
    ADD r0, #1024
    mov sp, r0

    @ Enter supervisor mode, irq enabled
    mov r0, #0x53
    msr cpsr_c, r0

    bl kmain
    b hang


handle_irq:
    SUB lr, lr, #4
    STMFD sp!, {R0-R12, lr}

    @ Save SPSR in R11
    MRS R11, SPSR
    PUSH {r11}

    @ Invoke handler
    bl handle_irq_rust

    @ Restore pending program state
    POP {r11}
    MSR SPSR, R11 

    @ Return
    LDMFD sp!, {R0-R12, pc}^
    b hang
    

hang:
    b hang

stack_base: .word 0x4030BE30
base_irq_addr: .word 0x4030CE38
basic_handler: .word handle_irq
