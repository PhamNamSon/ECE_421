<usize as core::iter::range::Step>::forward_unchecked:
        mov     rax, rdi
        add     rax, rsi
        ret

core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next:
        push    rax
        mov     rax, qword ptr [rip + <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next@GOTPCREL]
        call    rax
        pop     rcx
        ret

core::slice::<impl [T]>::swap:
        sub     rsp, 56
        mov     qword ptr [rsp + 8], rdi
        mov     qword ptr [rsp + 16], rsi
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 32], rcx
        mov     qword ptr [rsp + 40], r8
        cmp     rdx, rsi
        setb    al
        test    al, 1
        jne     .LBB2_1
        jmp     .LBB2_2
.LBB2_1:
        mov     rax, qword ptr [rsp + 32]
        mov     rcx, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        mov     rsi, qword ptr [rsp + 24]
        shl     rsi, 3
        add     rdx, rsi
        mov     qword ptr [rsp], rdx
        cmp     rax, rcx
        setb    al
        test    al, 1
        jne     .LBB2_3
        jmp     .LBB2_4
.LBB2_2:
        mov     rdx, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdi, qword ptr [rsp + 24]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        call    rax
.LBB2_3:
        mov     rax, qword ptr [rsp + 8]
        mov     rcx, qword ptr [rsp + 32]
        mov     rdx, qword ptr [rsp]
        mov     rdi, rcx
        shl     rdi, 3
        mov     rsi, rax
        add     rsi, rdi
        mov     rdi, qword ptr [rdx]
        mov     qword ptr [rsp + 48], rdi
        mov     rsi, qword ptr [rsi]
        mov     qword ptr [rdx], rsi
        mov     rdx, qword ptr [rsp + 48]
        mov     qword ptr [rax + 8*rcx], rdx
        add     rsp, 56
        ret
.LBB2_4:
        mov     rdx, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdi, qword ptr [rsp + 32]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        call    rax

<I as core::iter::traits::collect::IntoIterator>::into_iter:
        mov     rdx, rsi
        mov     rax, rdi
        ret

<core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next:
        sub     rsp, 40
        mov     qword ptr [rsp + 16], rdi
        mov     rax, qword ptr [rdi]
        cmp     rax, qword ptr [rdi + 8]
        jb      .LBB4_2
        mov     qword ptr [rsp + 24], 0
        jmp     .LBB4_3
.LBB4_2:
        mov     rax, qword ptr [rsp + 16]
        mov     rdi, qword ptr [rax]
        mov     qword ptr [rsp + 8], rdi
        mov     esi, 1
        call    <usize as core::iter::range::Step>::forward_unchecked
        mov     rcx, qword ptr [rsp + 16]
        mov     rdx, rax
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rcx], rdx
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], 1
.LBB4_3:
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 40
        ret

example::question_2:
        sub     rsp, 168
        mov     qword ptr [rsp + 48], rdi
        mov     qword ptr [rsp + 56], rsi
        mov     qword ptr [rsp + 64], 0
        mov     qword ptr [rsp + 72], rsi
        mov     rdi, qword ptr [rsp + 64]
        mov     rsi, qword ptr [rsp + 72]
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 80], rax
        mov     qword ptr [rsp + 88], rdx
.LBB5_1:
        mov     rax, qword ptr [rip + core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next@GOTPCREL]
        lea     rdi, [rsp + 80]
        call    rax
        mov     qword ptr [rsp + 104], rdx
        mov     qword ptr [rsp + 96], rax
        cmp     qword ptr [rsp + 96], 0
        jne     .LBB5_3
        mov     rdx, qword ptr [rsp + 56]
        mov     rax, qword ptr [rsp + 48]
        add     rsp, 168
        ret
.LBB5_3:
        mov     rax, qword ptr [rsp + 104]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 112], rax
        add     rax, 1
        mov     qword ptr [rsp + 40], rax
        setb    al
        test    al, 1
        jne     .LBB5_5
        mov     rax, qword ptr [rsp + 56]
        mov     rcx, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 120], rcx
        mov     qword ptr [rsp + 128], rax
        mov     rdi, qword ptr [rsp + 120]
        mov     rsi, qword ptr [rsp + 128]
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 136], rax
        mov     qword ptr [rsp + 144], rdx
        jmp     .LBB5_6
.LBB5_5:
        lea     rdi, [rip + str.0]
        lea     rdx, [rip + .L__unnamed_1]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 28
        call    rax
.LBB5_6:
        mov     rax, qword ptr [rip + core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next@GOTPCREL]
        lea     rdi, [rsp + 136]
        call    rax
        mov     qword ptr [rsp + 160], rdx
        mov     qword ptr [rsp + 152], rax
        cmp     qword ptr [rsp + 152], 0
        jne     .LBB5_8
        mov     rax, qword ptr [rsp + 32]
        cmp     qword ptr [rsp + 112], rax
        jne     .LBB5_9
        jmp     .LBB5_1
.LBB5_8:
        mov     rcx, qword ptr [rsp + 56]
        mov     rax, qword ptr [rsp + 160]
        mov     qword ptr [rsp + 24], rax
        cmp     rax, rcx
        setb    al
        test    al, 1
        jne     .LBB5_10
        jmp     .LBB5_11
.LBB5_9:
        mov     rdx, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 56]
        mov     rdi, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 112]
        lea     r8, [rip + .L__unnamed_2]
        call    qword ptr [rip + core::slice::<impl [T]>::swap@GOTPCREL]
        jmp     .LBB5_1
.LBB5_10:
        mov     rcx, qword ptr [rsp + 56]
        mov     rax, qword ptr [rsp + 48]
        mov     rdx, qword ptr [rsp + 24]
        mov     rax, qword ptr [rax + 8*rdx]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 112]
        mov     qword ptr [rsp + 16], rax
        cmp     rax, rcx
        setb    al
        test    al, 1
        jne     .LBB5_12
        jmp     .LBB5_13
.LBB5_11:
        mov     rsi, qword ptr [rsp + 56]
        mov     rdi, qword ptr [rsp + 24]
        lea     rdx, [rip + .L__unnamed_3]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        call    rax
.LBB5_12:
        mov     rax, qword ptr [rsp + 8]
        mov     rcx, qword ptr [rsp + 48]
        mov     rdx, qword ptr [rsp + 16]
        cmp     rax, qword ptr [rcx + 8*rdx]
        jl      .LBB5_14
        jmp     .LBB5_6
.LBB5_13:
        mov     rsi, qword ptr [rsp + 56]
        mov     rdi, qword ptr [rsp + 16]
        lea     rdx, [rip + .L__unnamed_4]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        call    rax
.LBB5_14:
        mov     rax, qword ptr [rsp + 24]
        mov     qword ptr [rsp + 112], rax
        jmp     .LBB5_6

.L__unnamed_5:
        .ascii  "/app/example.rs"

.L__unnamed_1:
        .quad   .L__unnamed_5
        .asciz  "\017\000\000\000\000\000\000\000\005\000\000\000\022\000\000"

str.0:
        .ascii  "attempt to add with overflow"

.L__unnamed_2:
        .quad   .L__unnamed_5
        .asciz  "\017\000\000\000\000\000\000\000\013\000\000\000\022\000\000"

.L__unnamed_3:
        .quad   .L__unnamed_5
        .asciz  "\017\000\000\000\000\000\000\000\006\000\000\000\020\000\000"

.L__unnamed_4:
        .quad   .L__unnamed_5
        .asciz  "\017\000\000\000\000\000\000\000\006\000\000\000\032\000\000"