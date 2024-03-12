example::question_2:
        push    rax
        test    rsi, rsi
        je      .LBB0_11
        xor     edx, edx
        jmp     .LBB0_2
.LBB0_10:
        cmp     rdx, rsi
        je      .LBB0_11
.LBB0_2:
        mov     r8, rdx
        inc     rdx
        mov     r9, rdx
        mov     rcx, r8
        cmp     rdx, rsi
        jb      .LBB0_3
        jmp     .LBB0_10
.LBB0_6:
        inc     r9
        mov     rcx, rax
        cmp     r9, rsi
        jae     .LBB0_7
.LBB0_3:
        cmp     rcx, rsi
        jae     .LBB0_12
        mov     r10, qword ptr [rdi + 8*r9]
        mov     rax, r9
        cmp     r10, qword ptr [rdi + 8*rcx]
        jl      .LBB0_6
        mov     rax, rcx
        jmp     .LBB0_6
.LBB0_7:
        cmp     rax, r8
        je      .LBB0_10
        cmp     rax, rsi
        jae     .LBB0_13
        mov     rcx, qword ptr [rdi + 8*r8]
        mov     r9, qword ptr [rdi + 8*rax]
        mov     qword ptr [rdi + 8*r8], r9
        mov     qword ptr [rdi + 8*rax], rcx
        jmp     .LBB0_10
.LBB0_11:
        mov     rax, rdi
        mov     rdx, rsi
        pop     rcx
        ret
.LBB0_12:
        lea     rdx, [rip + .L__unnamed_1]
        mov     rdi, rcx
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
.LBB0_13:
        lea     rdx, [rip + .L__unnamed_2]
        mov     rdi, rax
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]

.L__unnamed_3:
        .ascii  "/app/example.rs"

.L__unnamed_2:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\013\000\000\000\022\000\000"

.L__unnamed_1:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\006\000\000\000\032\000\000"