<&T as core::fmt::Debug>::fmt:
        mov     rdi, qword ptr [rdi]
        mov     eax, dword ptr [rsi + 52]
        test    al, 16
        jne     .LBB0_3
        test    al, 32
        jne     .LBB0_2
        jmp     qword ptr [rip + _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hc9733ab76fb3c52cE@GOTPCREL]
.LBB0_3:
        jmp     qword ptr [rip + _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8e89ccd3b870a36eE@GOTPCREL]
.LBB0_2:
        jmp     qword ptr [rip + _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h891839fecf0a4aefE@GOTPCREL]

core::ptr::drop_in_place<&i32>:
        ret

core::array::<impl core::fmt::Debug for [T; N]>::fmt:
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        sub     rsp, 32
        mov     r14, rdi
        lea     rbx, [rsp + 16]
        mov     rdi, rbx
        call    qword ptr [rip + core::fmt::Formatter::debug_list@GOTPCREL]
        mov     qword ptr [rsp + 8], r14
        lea     r15, [rip + .L__unnamed_1]
        mov     r13, qword ptr [rip + core::fmt::builders::DebugList::entry@GOTPCREL]
        lea     r12, [rsp + 8]
        mov     rdi, rbx
        mov     rsi, r12
        mov     rdx, r15
        call    r13
        lea     rax, [r14 + 4]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, rbx
        mov     rsi, r12
        mov     rdx, r15
        call    r13
        lea     rax, [r14 + 8]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, rbx
        mov     rsi, r12
        mov     rdx, r15
        call    r13
        add     r14, 12
        mov     qword ptr [rsp + 8], r14
        mov     rdi, rbx
        mov     rsi, r12
        mov     rdx, r15
        call    r13
        mov     rdi, rbx
        call    qword ptr [rip + core::fmt::builders::DebugList::finish@GOTPCREL]
        add     rsp, 32
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        ret

.LCPI3_0:
        .long   10
        .long   10
        .long   12
        .long   32
example::main:
        sub     rsp, 88
        movaps  xmm0, xmmword ptr [rip + .LCPI3_0]
        movaps  xmmword ptr [rsp + 16], xmm0
        lea     rax, [rsp + 16]
        mov     qword ptr [rsp], rax
        lea     rax, [rip + core::array::<impl core::fmt::Debug for [T; N]>::fmt]
        mov     qword ptr [rsp + 8], rax
        lea     rax, [rip + .L__unnamed_2]
        mov     qword ptr [rsp + 40], rax
        mov     qword ptr [rsp + 48], 2
        mov     qword ptr [rsp + 72], 0
        mov     rax, rsp
        mov     qword ptr [rsp + 56], rax
        mov     qword ptr [rsp + 64], 1
        lea     rdi, [rsp + 40]
        call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
        add     rsp, 88
        ret

.L__unnamed_1:
        .quad   core::ptr::drop_in_place<&i32>
        .asciz  "\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
        .quad   <&T as core::fmt::Debug>::fmt

.L__unnamed_3:

.L__unnamed_4:
        .byte   10

.L__unnamed_2:
        .quad   .L__unnamed_3
        .zero   8
        .quad   .L__unnamed_4
        .asciz  "\001\000\000\000\000\000\000"