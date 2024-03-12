<&T as core::fmt::Debug>::fmt:
        push    rax
        mov     rdi, qword ptr [rdi]
        call    core::fmt::num::<impl core::fmt::Debug for i32>::fmt
        and     al, 1
        movzx   eax, al
        pop     rcx
        ret

<[T] as core::fmt::Debug>::fmt:
        sub     rsp, 72
        mov     qword ptr [rsp], rdx
        mov     rax, rsi
        mov     rsi, qword ptr [rsp]
        mov     qword ptr [rsp + 8], rdi
        mov     qword ptr [rsp + 16], rax
        lea     rdi, [rsp + 24]
        call    qword ptr [rip + core::fmt::Formatter::debug_list@GOTPCREL]
        xor     eax, eax
        test    al, 1
        jne     .LBB1_2
        mov     rax, qword ptr [rsp + 8]
        mov     rcx, qword ptr [rsp + 16]
        shl     rcx, 2
        add     rax, rcx
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB1_3
.LBB1_2:
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 56], rax
.LBB1_3:
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 64], rax
        mov     rax, qword ptr [rsp + 56]
        mov     rcx, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 40], rcx
        mov     qword ptr [rsp + 48], rax
        mov     rsi, qword ptr [rsp + 40]
        mov     rdx, qword ptr [rsp + 48]
        lea     rdi, [rsp + 24]
        call    qword ptr [rip + core::fmt::builders::DebugList::entries@GOTPCREL]
        mov     rdi, rax
        call    qword ptr [rip + core::fmt::builders::DebugList::finish@GOTPCREL]
        and     al, 1
        movzx   eax, al
        add     rsp, 72
        ret

core::fmt::num::<impl core::fmt::Debug for i32>::fmt:
        sub     rsp, 24
        mov     qword ptr [rsp], rdi
        mov     qword ptr [rsp + 8], rsi
        mov     eax, dword ptr [rsi + 52]
        and     eax, 16
        cmp     eax, 0
        jne     .LBB2_2
        mov     rax, qword ptr [rsp + 8]
        mov     eax, dword ptr [rax + 52]
        and     eax, 32
        cmp     eax, 0
        je      .LBB2_3
        jmp     .LBB2_4
.LBB2_2:
        mov     rsi, qword ptr [rsp + 8]
        mov     rdi, qword ptr [rsp]
        call    qword ptr [rip + core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 23], al
        jmp     .LBB2_5
.LBB2_3:
        mov     rsi, qword ptr [rsp + 8]
        mov     rdi, qword ptr [rsp]
        call    qword ptr [rip + core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 23], al
        jmp     .LBB2_5
.LBB2_4:
        mov     rsi, qword ptr [rsp + 8]
        mov     rdi, qword ptr [rsp]
        call    qword ptr [rip + core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 23], al
.LBB2_5:
        mov     al, byte ptr [rsp + 23]
        and     al, 1
        movzx   eax, al
        add     rsp, 24
        ret

core::fmt::builders::DebugList::entries:
        sub     rsp, 88
        mov     qword ptr [rsp + 16], rdx
        mov     rax, rsi
        mov     rsi, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 24], rax
        mov     rax, rdi
        mov     rdi, qword ptr [rsp + 24]
        mov     qword ptr [rsp + 32], rax
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 40], rax
        mov     qword ptr [rsp + 48], rdx
.LBB3_1:
        mov     rax, qword ptr [rip + <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::next@GOTPCREL]
        lea     rdi, [rsp + 40]
        call    rax
        mov     qword ptr [rsp + 8], rax
        jmp     .LBB3_4
.LBB3_2:
        mov     rdi, qword ptr [rsp + 72]
        call    _Unwind_Resume@PLT
        mov     rcx, rax
        mov     eax, edx
        mov     qword ptr [rsp + 72], rcx
        mov     dword ptr [rsp + 80], eax
        jmp     .LBB3_2
.LBB3_4:
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 56], rax
        mov     rdx, qword ptr [rsp + 56]
        mov     eax, 1
        xor     ecx, ecx
        cmp     rdx, 0
        cmove   rax, rcx
        cmp     rax, 0
        jne     .LBB3_6
        mov     rax, qword ptr [rsp + 32]
        add     rsp, 88
        ret
.LBB3_6:
        mov     rdi, qword ptr [rsp + 32]
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 64], rax
        lea     rdx, [rip + .L__unnamed_1]
        mov     rax, qword ptr [rip + core::fmt::builders::DebugList::entry@GOTPCREL]
        lea     rsi, [rsp + 64]
        call    rax
        jmp     .LBB3_9
.LBB3_7:
        jmp     .LBB3_2
        mov     rcx, rax
        mov     eax, edx
        mov     qword ptr [rsp + 72], rcx
        mov     dword ptr [rsp + 80], eax
        jmp     .LBB3_7
.LBB3_9:
        jmp     .LBB3_10
.LBB3_10:
        jmp     .LBB3_1

core::fmt::Arguments::new_v1:
        sub     rsp, 136
        mov     qword ptr [rsp + 8], r8
        mov     qword ptr [rsp + 16], rcx
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 32], rsi
        mov     qword ptr [rsp + 40], rdi
        mov     qword ptr [rsp + 48], rdi
        cmp     rdx, r8
        jb      .LBB4_2
        mov     rax, qword ptr [rsp + 24]
        mov     rcx, qword ptr [rsp + 8]
        add     rcx, 1
        cmp     rax, rcx
        ja      .LBB4_4
        jmp     .LBB4_3
.LBB4_2:
        jmp     .LBB4_4
.LBB4_3:
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        mov     rdx, qword ptr [rsp + 8]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdi, qword ptr [rsp + 24]
        mov     r8, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 104], 0
        mov     qword ptr [rcx], r8
        mov     qword ptr [rcx + 8], rdi
        mov     r8, qword ptr [rsp + 104]
        mov     rdi, qword ptr [rsp + 112]
        mov     qword ptr [rcx + 32], r8
        mov     qword ptr [rcx + 40], rdi
        mov     qword ptr [rcx + 16], rsi
        mov     qword ptr [rcx + 24], rdx
        add     rsp, 136
        ret
.LBB4_4:
        mov     qword ptr [rsp + 120], 0
        lea     rax, [rip + .L__unnamed_2]
        mov     qword ptr [rsp + 56], rax
        mov     qword ptr [rsp + 64], 1
        mov     rcx, qword ptr [rsp + 120]
        mov     rax, qword ptr [rsp + 128]
        mov     qword ptr [rsp + 88], rcx
        mov     qword ptr [rsp + 96], rax
        lea     rax, [rip + .L__unnamed_3]
        mov     qword ptr [rsp + 72], rax
        mov     qword ptr [rsp + 80], 0
        lea     rsi, [rip + .L__unnamed_4]
        mov     rax, qword ptr [rip + core::panicking::panic_fmt@GOTPCREL]
        lea     rdi, [rsp + 56]
        call    rax

core::ptr::drop_in_place<&i32>:
        ret

core::array::<impl core::fmt::Debug for [T; N]>::fmt:
        sub     rsp, 24
        mov     rdx, rsi
        mov     qword ptr [rsp + 8], rdi
        mov     qword ptr [rsp + 16], 4
        mov     rdi, qword ptr [rsp + 8]
        mov     rsi, qword ptr [rsp + 16]
        call    qword ptr [rip + <[T] as core::fmt::Debug>::fmt@GOTPCREL]
        and     al, 1
        movzx   eax, al
        add     rsp, 24
        ret

<I as core::iter::traits::collect::IntoIterator>::into_iter:
        mov     rdx, rsi
        mov     rax, rdi
        ret

<core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::next:
        mov     qword ptr [rsp - 48], rdi
        xor     eax, eax
        test    al, 1
        jne     .LBB8_2
        mov     rax, qword ptr [rsp - 48]
        mov     rcx, qword ptr [rax + 8]
        mov     qword ptr [rsp - 24], rcx
        mov     rax, qword ptr [rax]
        cmp     rax, qword ptr [rsp - 24]
        sete    al
        and     al, 1
        mov     byte ptr [rsp - 25], al
        jmp     .LBB8_3
.LBB8_2:
        mov     rax, qword ptr [rsp - 48]
        mov     rax, qword ptr [rax + 8]
        cmp     rax, 0
        sete    al
        and     al, 1
        mov     byte ptr [rsp - 25], al
.LBB8_3:
        test    byte ptr [rsp - 25], 1
        jne     .LBB8_5
        mov     rax, qword ptr [rsp - 48]
        mov     rax, qword ptr [rax]
        mov     qword ptr [rsp - 16], rax
        xor     eax, eax
        test    al, 1
        jne     .LBB8_7
        jmp     .LBB8_6
.LBB8_5:
        mov     qword ptr [rsp - 40], 0
        jmp     .LBB8_9
.LBB8_6:
        mov     rax, qword ptr [rsp - 48]
        mov     rcx, qword ptr [rax]
        add     rcx, 4
        mov     qword ptr [rsp - 8], rcx
        mov     rcx, qword ptr [rsp - 8]
        mov     qword ptr [rax], rcx
        jmp     .LBB8_8
.LBB8_7:
        mov     rax, qword ptr [rsp - 48]
        mov     rcx, qword ptr [rax + 8]
        sub     rcx, 1
        mov     qword ptr [rax + 8], rcx
.LBB8_8:
        mov     rax, qword ptr [rsp - 16]
        mov     qword ptr [rsp - 40], rax
.LBB8_9:
        mov     rax, qword ptr [rsp - 40]
        ret

example::question_2:
        mov     qword ptr [rsp - 32], rdi
        mov     qword ptr [rsp - 24], rdi
        mov     dword ptr [rsp - 16], 10
        mov     dword ptr [rsp - 12], 0
        mov     dword ptr [rsp - 8], 12
        mov     dword ptr [rsp - 4], 32
        cmp     dword ptr [rsp - 12], 32
        jg      .LBB9_2
        mov     dword ptr [rsp - 12], 10
        jmp     .LBB9_3
.LBB9_2:
        mov     dword ptr [rsp - 4], 10
.LBB9_3:
        mov     rax, qword ptr [rsp - 24]
        mov     rcx, qword ptr [rsp - 32]
        movdqu  xmm0, xmmword ptr [rsp - 16]
        movdqu  xmmword ptr [rcx], xmm0
        ret

example::main:
        sub     rsp, 104
        lea     rdi, [rsp + 72]
        mov     qword ptr [rsp], rdi
        call    example::question_2
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 88], rax
        mov     rax, qword ptr [rip + core::array::<impl core::fmt::Debug for [T; N]>::fmt@GOTPCREL]
        mov     qword ptr [rsp + 96], rax
        mov     rcx, qword ptr [rsp + 88]
        mov     rax, qword ptr [rsp + 96]
        mov     qword ptr [rsp + 56], rcx
        mov     qword ptr [rsp + 64], rax
        lea     rdi, [rsp + 8]
        lea     rsi, [rip + .L__unnamed_5]
        mov     edx, 2
        lea     rcx, [rsp + 56]
        mov     r8d, 1
        call    core::fmt::Arguments::new_v1
        lea     rdi, [rsp + 8]
        call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
        add     rsp, 104
        ret

.L__unnamed_1:
        .quad   core::ptr::drop_in_place<&i32>
        .asciz  "\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
        .quad   <&T as core::fmt::Debug>::fmt

.L__unnamed_6:
        .ascii  "invalid args"

.L__unnamed_2:
        .quad   .L__unnamed_6
        .asciz  "\f\000\000\000\000\000\000"

.L__unnamed_3:

.L__unnamed_7:
        .ascii  "/rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/fmt/mod.rs"

.L__unnamed_4:
        .quad   .L__unnamed_7
        .asciz  "K\000\000\000\000\000\000\000M\001\000\000\r\000\000"

.L__unnamed_8:
        .byte   10

.L__unnamed_5:
        .quad   .L__unnamed_3
        .zero   8
        .quad   .L__unnamed_8
        .asciz  "\001\000\000\000\000\000\000"

DW.ref.rust_eh_personality:
        .quad   rust_eh_personality