global _start


section .data


section .text

_start:
  ; sysexit(1) ;
  mov rax, 60
  mov rdi, 1
  syscall
