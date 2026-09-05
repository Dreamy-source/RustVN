; vn code-generation
; example

global _start
default rel

section .data
    x: dq 0

section .text
_start:
    mov rax, (5 + (3 * (2 / 5)))
    mov [x], rax

    mov rax, 60
    mov rdi, 0
    syscall
