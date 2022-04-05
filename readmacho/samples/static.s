section .text
global start
start:
  mov rax, 0x2000000 + 4    ;sys_write
  mov rdi, 1

  ;PC relative な命令を試す.
  ;lea reg, [exp] 命令はexpの結果をそのままregに代入する.
  ;mov reg, [exp] 命令はexpの結果のアドレスの値をregに代入する.
  ;mov reg, imm 命令はimm値をregに代入する.
  ;つまり、`lea rsi, [rel msg]` と `mov rsi, msg` は等価.
  ;ただし、Mach-O（nasmで？）`mov rsi, msg` の形式が使える
  ;ようになったのは割と最近（2020年くらいから？）ぽい
  ; lea rsi, [rel msg]
  mov rsi, msg
  ;こっちは mov reg, imm の形式
  mov rdx, len
  syscall

  mov rax, 0x2000000 + 1    ;sys_exit
  mov rdi, 0
  syscall

section .data
  msg db  'hello, world',0x0A
  len equ $ - msg

;bss section を試す
section .bss
  hoge resb 42
