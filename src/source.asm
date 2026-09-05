; vn code-generation
; code-generated example

global _start
default rel

section .data
    x: dq 0

section .text
_start:
    mov rax, (5 + (3 * (2 / 5)))
    mov [x], rax

    mov eax, 60
    xor edi, edi
    syscall
