global _vn
section .text
_vn:
    mov rax, (5 + (3 * (2 / 5)))
    mov [x], rax

    mov rax, 60
    mov rdi, 0
    syscall
