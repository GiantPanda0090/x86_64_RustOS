global long_mode_start
extern rust_main
section .text
bits 64
long_mode_start:
				; Reload DATA SAGMENT with NULL=0
				; load 0 into all data segment registers
	mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	extern	rust_main;
	call rust_main;'

    ; print `OKAY` to screen
    mov rax, 0x2f492f512f492f4c
    mov qword [0xb8000], rax
    hlt
