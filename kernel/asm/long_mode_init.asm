global long_mode_start

section .text
extern kernel_main
extern os_main
; extern p4_table

bits 64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    mov qword [0xb8008], 0
    
    ; lea rsi, [p4_table]
    call kernel_main
    call os_main

.hlt_loop:
    hlt
    jmp .hlt_loop
