pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    for i in 0..list.len() {
        let mut min = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[min] {
                min = j
            }
        }
        list.swap(min, i);
    }
}

#[cfg(test)]
#[test]
pub fn ssort_test() {
    let mut list = vec![6, 3, 7, 1, 4, 2, 0, 5, 9, 8];
    selection_sort(&mut list);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], list,)
}

/* machine code (without -O):
<core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from:
        sub     rsp, 24
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rdi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::new_unchecked@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 8]
        add     rsp, 24
        ret

<core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from:
        sub     rsp, 40
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::new_unchecked@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

<&T as core::fmt::Debug>::fmt:
        push    rax
        mov     rdi, qword ptr [rdi]
        call    qword ptr [rip + <alloc::vec::Vec<T> as core::fmt::Debug>::fmt@GOTPCREL]
        mov     byte ptr [rsp + 7], al
        mov     al, byte ptr [rsp + 7]
        and     al, 1
        movzx   eax, al
        pop     rcx
        ret

<&T as core::fmt::Debug>::fmt:
        push    rax
        mov     rdi, qword ptr [rdi]
        call    core::fmt::num::<impl core::fmt::Debug for i32>::fmt
        mov     byte ptr [rsp + 7], al
        mov     al, byte ptr [rsp + 7]
        and     al, 1
        movzx   eax, al
        pop     rcx
        ret

<[T] as core::fmt::Debug>::fmt:
        sub     rsp, 72
        lea     rax, [rsp + 56]
        mov     qword ptr [rsp + 48], rdi
        mov     rdi, rax
        mov     qword ptr [rsp + 40], rsi
        mov     rsi, rdx
        call    qword ptr [rip + core::fmt::Formatter::debug_list@GOTPCREL]
        mov     rdi, qword ptr [rsp + 48]
        mov     rsi, qword ptr [rsp + 40]
        call    qword ptr [rip + core::slice::<impl [T]>::iter@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        lea     rdi, [rsp + 56]
        mov     rsi, qword ptr [rsp + 32]
        mov     rdx, qword ptr [rsp + 24]
        call    qword ptr [rip + core::fmt::builders::DebugList::entries@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rdi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::fmt::builders::DebugList::finish@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        and     al, 1
        movzx   eax, al
        add     rsp, 72
        ret

<usize as core::iter::range::Step>::add_usize:
        sub     rsp, 40
        mov     qword ptr [rsp + 24], rdi
        mov     rdi, rsi
        call    qword ptr [rip + <T as core::convert::TryFrom<U>>::try_from@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     rsi, qword ptr [rsp + 32]
        mov     rax, qword ptr [rsp + 24]
        mov     rdi, qword ptr [rax]
        call    core::num::<impl usize>::checked_add
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

core::intrinsics::copy_nonoverlapping:
        sub     rsp, 40
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 32], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 16], rdx
        jne     .LBB6_1
        jmp     .LBB6_5
.LBB6_1:
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB6_4
        jmp     .LBB6_5
.LBB6_4:
        lea     rdi, [rip + .L__unnamed_2]
        lea     rdx, [rip + .L__unnamed_3]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 46
        call    rax
        ud2
.LBB6_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB6_6
        jmp     .LBB6_10
.LBB6_6:
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 14], al
        mov     al, byte ptr [rsp + 14]
        xor     al, -1
        test    al, 1
        jne     .LBB6_9
        jmp     .LBB6_10
.LBB6_9:
        lea     rdi, [rip + .L__unnamed_4]
        lea     rdx, [rip + .L__unnamed_5]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 44
        call    rax
        ud2
.LBB6_10:
        xor     eax, eax
        test    al, 1
        jne     .LBB6_11
        jmp     .LBB6_15
.LBB6_11:
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 16]
        call    qword ptr [rip + core::intrinsics::overlaps@GOTPCREL]
        mov     byte ptr [rsp + 13], al
        mov     al, byte ptr [rsp + 13]
        xor     al, -1
        xor     al, -1
        test    al, 1
        jne     .LBB6_14
        jmp     .LBB6_15
.LBB6_14:
        lea     rdi, [rip + .L__unnamed_6]
        lea     rdx, [rip + .L__unnamed_7]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 37
        call    rax
        ud2
.LBB6_15:
        mov     rax, qword ptr [rsp + 16]
        shl     rax, 2
        mov     rcx, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        mov     rdi, rcx
        mov     rsi, rdx
        mov     rdx, rax
        call    memcpy@PLT
        add     rsp, 40
        ret

core::intrinsics::copy_nonoverlapping:
        sub     rsp, 40
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 32], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 16], rdx
        jne     .LBB7_1
        jmp     .LBB7_5
.LBB7_1:
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB7_4
        jmp     .LBB7_5
.LBB7_4:
        lea     rdi, [rip + .L__unnamed_2]
        lea     rdx, [rip + .L__unnamed_3]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 46
        call    rax
        ud2
.LBB7_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB7_6
        jmp     .LBB7_10
.LBB7_6:
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 14], al
        mov     al, byte ptr [rsp + 14]
        xor     al, -1
        test    al, 1
        jne     .LBB7_9
        jmp     .LBB7_10
.LBB7_9:
        lea     rdi, [rip + .L__unnamed_4]
        lea     rdx, [rip + .L__unnamed_5]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 44
        call    rax
        ud2
.LBB7_10:
        xor     eax, eax
        test    al, 1
        jne     .LBB7_11
        jmp     .LBB7_15
.LBB7_11:
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 16]
        call    qword ptr [rip + core::intrinsics::overlaps@GOTPCREL]
        mov     byte ptr [rsp + 13], al
        mov     al, byte ptr [rsp + 13]
        xor     al, -1
        xor     al, -1
        test    al, 1
        jne     .LBB7_14
        jmp     .LBB7_15
.LBB7_14:
        lea     rdi, [rip + .L__unnamed_6]
        lea     rdx, [rip + .L__unnamed_7]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 37
        call    rax
        ud2
.LBB7_15:
        mov     rax, qword ptr [rsp + 16]
        shl     rax, 0
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        mov     rdx, rax
        call    memcpy@PLT
        add     rsp, 40
        ret

core::intrinsics::copy_nonoverlapping:
        sub     rsp, 40
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 32], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 16], rdx
        jne     .LBB8_1
        jmp     .LBB8_5
.LBB8_1:
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB8_4
        jmp     .LBB8_5
.LBB8_4:
        lea     rdi, [rip + .L__unnamed_2]
        lea     rdx, [rip + .L__unnamed_3]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 46
        call    rax
        ud2
.LBB8_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB8_6
        jmp     .LBB8_10
.LBB8_6:
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 14], al
        mov     al, byte ptr [rsp + 14]
        xor     al, -1
        test    al, 1
        jne     .LBB8_9
        jmp     .LBB8_10
.LBB8_9:
        lea     rdi, [rip + .L__unnamed_4]
        lea     rdx, [rip + .L__unnamed_5]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 44
        call    rax
        ud2
.LBB8_10:
        xor     eax, eax
        test    al, 1
        jne     .LBB8_11
        jmp     .LBB8_15
.LBB8_11:
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 16]
        call    qword ptr [rip + core::intrinsics::overlaps@GOTPCREL]
        mov     byte ptr [rsp + 13], al
        mov     al, byte ptr [rsp + 13]
        xor     al, -1
        xor     al, -1
        test    al, 1
        jne     .LBB8_14
        jmp     .LBB8_15
.LBB8_14:
        lea     rdi, [rip + .L__unnamed_6]
        lea     rdx, [rip + .L__unnamed_7]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 37
        call    rax
        ud2
.LBB8_15:
        mov     rax, qword ptr [rsp + 16]
        shl     rax, 3
        mov     rcx, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        mov     rdi, rcx
        mov     rsi, rdx
        mov     rdx, rax
        call    memcpy@PLT
        add     rsp, 40
        ret

core::intrinsics::is_aligned_and_not_null:
        sub     rsp, 40
        mov     qword ptr [rsp + 16], rdi
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        jmp     .LBB9_5
.LBB9_1:
        mov     byte ptr [rsp + 31], 1
        jmp     .LBB9_4
.LBB9_2:
        mov     byte ptr [rsp + 31], 0
        jmp     .LBB9_4
.LBB9_3:
        mov     qword ptr [rsp + 32], 4
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp], rax
        jmp     .LBB9_6
.LBB9_4:
        mov     al, byte ptr [rsp + 31]
        and     al, 1
        movzx   eax, al
        add     rsp, 40
        ret
.LBB9_5:
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB9_3
        jmp     .LBB9_2
.LBB9_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        test    cl, 1
        jne     .LBB9_8
        mov     rax, qword ptr [rsp + 16]
        xor     ecx, ecx
        mov     edx, ecx
        mov     rsi, qword ptr [rsp]
        div     rsi
        cmp     rdx, 0
        je      .LBB9_1
        jmp     .LBB9_2
.LBB9_8:
        lea     rdi, [rip + str.0]
        lea     rdx, [rip + .L__unnamed_8]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 57
        call    rax
        ud2

core::intrinsics::is_aligned_and_not_null:
        sub     rsp, 40
        mov     qword ptr [rsp + 16], rdi
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        jmp     .LBB10_5
.LBB10_1:
        mov     byte ptr [rsp + 31], 1
        jmp     .LBB10_4
.LBB10_2:
        mov     byte ptr [rsp + 31], 0
        jmp     .LBB10_4
.LBB10_3:
        mov     qword ptr [rsp + 32], 1
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp], rax
        jmp     .LBB10_6
.LBB10_4:
        mov     al, byte ptr [rsp + 31]
        and     al, 1
        movzx   eax, al
        add     rsp, 40
        ret
.LBB10_5:
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB10_3
        jmp     .LBB10_2
.LBB10_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        test    cl, 1
        jne     .LBB10_8
        mov     rax, qword ptr [rsp + 16]
        xor     ecx, ecx
        mov     edx, ecx
        mov     rsi, qword ptr [rsp]
        div     rsi
        cmp     rdx, 0
        je      .LBB10_1
        jmp     .LBB10_2
.LBB10_8:
        lea     rdi, [rip + str.0]
        lea     rdx, [rip + .L__unnamed_8]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 57
        call    rax
        ud2

core::intrinsics::is_aligned_and_not_null:
        sub     rsp, 40
        mov     qword ptr [rsp + 16], rdi
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        jmp     .LBB11_5
.LBB11_1:
        mov     byte ptr [rsp + 31], 1
        jmp     .LBB11_4
.LBB11_2:
        mov     byte ptr [rsp + 31], 0
        jmp     .LBB11_4
.LBB11_3:
        mov     qword ptr [rsp + 32], 8
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp], rax
        jmp     .LBB11_6
.LBB11_4:
        mov     al, byte ptr [rsp + 31]
        and     al, 1
        movzx   eax, al
        add     rsp, 40
        ret
.LBB11_5:
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB11_3
        jmp     .LBB11_2
.LBB11_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        test    cl, 1
        jne     .LBB11_8
        mov     rax, qword ptr [rsp + 16]
        xor     ecx, ecx
        mov     edx, ecx
        mov     rsi, qword ptr [rsp]
        div     rsi
        cmp     rdx, 0
        je      .LBB11_1
        jmp     .LBB11_2
.LBB11_8:
        lea     rdi, [rip + str.0]
        lea     rdx, [rip + .L__unnamed_8]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 57
        call    rax
        ud2

core::intrinsics::copy:
        sub     rsp, 40
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 32], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 16], rdx
        jne     .LBB12_1
        jmp     .LBB12_5
.LBB12_1:
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        xor     al, -1
        test    al, 1
        jne     .LBB12_4
        jmp     .LBB12_5
.LBB12_4:
        lea     rdi, [rip + .L__unnamed_2]
        lea     rdx, [rip + .L__unnamed_9]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 46
        call    rax
        ud2
.LBB12_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB12_6
        jmp     .LBB12_10
.LBB12_6:
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 14], al
        mov     al, byte ptr [rsp + 14]
        xor     al, -1
        test    al, 1
        jne     .LBB12_9
        jmp     .LBB12_10
.LBB12_9:
        lea     rdi, [rip + .L__unnamed_4]
        lea     rdx, [rip + .L__unnamed_10]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 44
        call    rax
        ud2
.LBB12_10:
        mov     rax, qword ptr [rsp + 16]
        shl     rax, 2
        mov     rcx, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rip + memmove@GOTPCREL]
        mov     rdi, rcx
        mov     qword ptr [rsp], rsi
        mov     rsi, rdx
        mov     rdx, rax
        mov     rax, qword ptr [rsp]
        call    rax
        add     rsp, 40
        ret

core::intrinsics::overlaps:
        sub     rsp, 72
        mov     qword ptr [rsp + 64], 1
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 48], rdi
        mov     qword ptr [rsp + 40], rsi
        mov     qword ptr [rsp + 32], rdx
        mov     qword ptr [rsp + 24], rax
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        call    core::num::<impl usize>::checked_mul
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::option::Option<T>::unwrap@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        cmp     rax, rcx
        ja      .LBB13_5
        mov     rax, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 48]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB13_6
.LBB13_5:
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
.LBB13_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, qword ptr [rsp + 56]
        seta    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        add     rsp, 72
        ret

core::intrinsics::overlaps:
        sub     rsp, 72
        mov     qword ptr [rsp + 64], 8
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 48], rdi
        mov     qword ptr [rsp + 40], rsi
        mov     qword ptr [rsp + 32], rdx
        mov     qword ptr [rsp + 24], rax
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        call    core::num::<impl usize>::checked_mul
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::option::Option<T>::unwrap@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        cmp     rax, rcx
        ja      .LBB14_5
        mov     rax, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 48]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB14_6
.LBB14_5:
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
.LBB14_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, qword ptr [rsp + 56]
        seta    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        add     rsp, 72
        ret

core::intrinsics::overlaps:
        sub     rsp, 72
        mov     qword ptr [rsp + 64], 4
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 48], rdi
        mov     qword ptr [rsp + 40], rsi
        mov     qword ptr [rsp + 32], rdx
        mov     qword ptr [rsp + 24], rax
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        call    core::num::<impl usize>::checked_mul
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::option::Option<T>::unwrap@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        cmp     rax, rcx
        ja      .LBB15_5
        mov     rax, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 48]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB15_6
.LBB15_5:
        mov     rax, qword ptr [rsp + 48]
        mov     rcx, qword ptr [rsp + 40]
        sub     rax, rcx
        mov     qword ptr [rsp + 56], rax
.LBB15_6:
        mov     rax, qword ptr [rsp]
        cmp     rax, qword ptr [rsp + 56]
        seta    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        add     rsp, 72
        ret

core::cmp::impls::<impl core::cmp::PartialOrd for i32>::lt:
        mov     eax, dword ptr [rdi]
        cmp     eax, dword ptr [rsi]
        setl    cl
        and     cl, 1
        movzx   eax, cl
        ret

core::cmp::impls::<impl core::cmp::PartialOrd for usize>::lt:
        mov     rax, qword ptr [rdi]
        cmp     rax, qword ptr [rsi]
        setb    cl
        and     cl, 1
        movzx   eax, cl
        ret

core::fmt::ArgumentV1::new:
        sub     rsp, 56
        mov     qword ptr [rsp + 40], rsi
        mov     rax, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 16], rdi
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 48], rax
        mov     rcx, qword ptr [rsp + 48]
        mov     qword ptr [rsp], rcx
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 24], rax
        mov     rcx, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 32], rcx
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 56
        ret

core::fmt::num::<impl core::fmt::Debug for i32>::fmt:
        sub     rsp, 40
        mov     qword ptr [rsp + 24], rdi
        mov     rdi, rsi
        mov     qword ptr [rsp + 16], rsi
        call    qword ptr [rip + core::fmt::Formatter::debug_lower_hex@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        test    al, 1
        jne     .LBB19_3
        jmp     .LBB19_2
.LBB19_2:
        mov     rdi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::fmt::Formatter::debug_upper_hex@GOTPCREL]
        mov     byte ptr [rsp + 14], al
        jmp     .LBB19_5
.LBB19_3:
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 39], al
        jmp     .LBB19_11
.LBB19_5:
        mov     al, byte ptr [rsp + 14]
        test    al, 1
        jne     .LBB19_7
        jmp     .LBB19_6
.LBB19_6:
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 39], al
        jmp     .LBB19_9
.LBB19_7:
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt@GOTPCREL]
        and     al, 1
        mov     byte ptr [rsp + 39], al
        jmp     .LBB19_10
.LBB19_9:
        jmp     .LBB19_10
.LBB19_10:
        jmp     .LBB19_11
.LBB19_11:
        mov     al, byte ptr [rsp + 39]
        and     al, 1
        movzx   eax, al
        add     rsp, 40
        ret

core::fmt::builders::DebugList::entries:
        sub     rsp, 104
        mov     byte ptr [rsp + 87], 0
        mov     qword ptr [rsp + 40], rdi
        mov     rdi, rsi
        mov     rsi, rdx
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        jmp     .LBB20_2
.LBB20_1:
        mov     rdi, qword ptr [rsp + 88]
        call    _Unwind_Resume@PLT
        ud2
.LBB20_2:
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 48], rax
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rsp + 56], rcx
.LBB20_3:
        mov     rax, qword ptr [rip + <core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::next@GOTPCREL]
        lea     rdi, [rsp + 48]
        call    rax
        mov     qword ptr [rsp + 16], rax
        jmp     .LBB20_5
.LBB20_4:
        jmp     .LBB20_1
.LBB20_5:
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 64], rax
        mov     rcx, qword ptr [rsp + 64]
        test    rcx, rcx
        setne   dl
        movzx   esi, dl
        mov     ecx, esi
        je      .LBB20_6
        jmp     .LBB20_20
.LBB20_20:
        jmp     .LBB20_8
.LBB20_6:
        jmp     .LBB20_9
        ud2
.LBB20_8:
        xor     eax, eax
        mov     ecx, eax
        mov     rdx, qword ptr [rsp + 64]
        mov     byte ptr [rsp + 87], 1
        cmp     qword ptr [rsp + 64], 0
        mov     esi, 1
        cmovbe  rsi, rcx
        cmp     rsi, 1
        mov     qword ptr [rsp + 8], rdx
        je      .LBB20_16
        jmp     .LBB20_17
.LBB20_9:
        mov     byte ptr [rsp + 87], 0
        jmp     .LBB20_13
.LBB20_10:
        jmp     .LBB20_12
.LBB20_11:
        jmp     .LBB20_15
.LBB20_12:
        mov     byte ptr [rsp + 87], 0
        jmp     .LBB20_3
.LBB20_13:
        mov     rax, qword ptr [rsp + 40]
        add     rsp, 104
        ret
.LBB20_14:
        mov     byte ptr [rsp + 87], 0
        jmp     .LBB20_4
.LBB20_15:
        test    byte ptr [rsp + 87], 1
        jne     .LBB20_14
        jmp     .LBB20_4
.LBB20_16:
        mov     byte ptr [rsp + 87], 0
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 72], rax
        lea     rdx, [rip + .L__unnamed_1]
        mov     rcx, qword ptr [rip + core::fmt::builders::DebugList::entry@GOTPCREL]
        lea     rsi, [rsp + 72]
        mov     rdi, qword ptr [rsp + 40]
        call    rcx
        jmp     .LBB20_10
.LBB20_17:
        jmp     .LBB20_16
        mov     qword ptr [rsp + 88], rax
        mov     dword ptr [rsp + 96], edx
        jmp     .LBB20_15
        mov     qword ptr [rsp + 88], rax
        mov     dword ptr [rsp + 96], edx
        jmp     .LBB20_11

core::fmt::Arguments::new_v1:
        sub     rsp, 16
        mov     rax, rdi
        mov     qword ptr [rsp], 0
        mov     qword ptr [rdi], rsi
        mov     qword ptr [rdi + 8], rdx
        mov     rdx, qword ptr [rsp]
        mov     rsi, qword ptr [rsp + 8]
        mov     qword ptr [rdi + 16], rdx
        mov     qword ptr [rdi + 24], rsi
        mov     qword ptr [rdi + 32], rcx
        mov     qword ptr [rdi + 40], r8
        add     rsp, 16
        ret

core::mem::size_of_val:
        sub     rsp, 16
        shl     rsi, 2
        mov     qword ptr [rsp + 8], rsi
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 16
        ret

core::mem::swap:
        push    rax
        call    qword ptr [rip + core::ptr::swap_nonoverlapping_one@GOTPCREL]
        pop     rax
        ret

core::mem::forget:
        sub     rsp, 16
        mov     qword ptr [rsp], rdi
        mov     qword ptr [rsp + 8], rsi
        add     rsp, 16
        ret

core::num::NonZeroUsize::new_unchecked:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::num::NonZeroUsize::get:
        mov     rax, rdi
        ret

core::num::<impl usize>::checked_add:
        sub     rsp, 40
        call    core::num::<impl usize>::overflowing_add
        mov     qword ptr [rsp + 16], rax
        mov     byte ptr [rsp + 15], dl
        mov     al, byte ptr [rsp + 15]
        test    al, 1
        jne     .LBB27_3
        jmp     .LBB27_2
.LBB27_2:
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], 1
        jmp     .LBB27_4
.LBB27_3:
        mov     qword ptr [rsp + 24], 0
.LBB27_4:
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 40
        ret

core::num::<impl usize>::checked_mul:
        sub     rsp, 40
        call    core::num::<impl usize>::overflowing_mul
        mov     qword ptr [rsp + 16], rax
        mov     byte ptr [rsp + 15], dl
        mov     al, byte ptr [rsp + 15]
        test    al, 1
        jne     .LBB28_3
        jmp     .LBB28_2
.LBB28_2:
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], 1
        jmp     .LBB28_4
.LBB28_3:
        mov     qword ptr [rsp + 24], 0
.LBB28_4:
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 40
        ret

core::num::<impl usize>::saturating_mul:
        sub     rsp, 24
        call    core::num::<impl usize>::checked_mul
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        jmp     .LBB29_2
.LBB29_2:
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        mov     rdx, -1
        call    qword ptr [rip + core::option::Option<T>::unwrap_or@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 24
        ret

core::num::<impl usize>::overflowing_add:
        sub     rsp, 48
        add     rdi, rsi
        setb    al
        and     al, 1
        mov     qword ptr [rsp + 32], rdi
        mov     byte ptr [rsp + 40], al
        mov     rcx, qword ptr [rsp + 32]
        mov     al, byte ptr [rsp + 40]
        mov     qword ptr [rsp + 8], rcx
        mov     byte ptr [rsp + 7], al
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 16], rax
        mov     cl, byte ptr [rsp + 7]
        and     cl, 1
        mov     byte ptr [rsp + 24], cl
        mov     rax, qword ptr [rsp + 16]
        mov     dl, byte ptr [rsp + 24]
        add     rsp, 48
        ret

core::num::<impl usize>::overflowing_mul:
        sub     rsp, 48
        mov     rax, rdi
        mul     rsi
        seto    cl
        and     cl, 1
        mov     qword ptr [rsp + 32], rax
        mov     byte ptr [rsp + 40], cl
        mov     rax, qword ptr [rsp + 32]
        mov     cl, byte ptr [rsp + 40]
        mov     qword ptr [rsp + 8], rax
        mov     byte ptr [rsp + 7], cl
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 16], rax
        mov     cl, byte ptr [rsp + 7]
        and     cl, 1
        mov     byte ptr [rsp + 24], cl
        mov     rax, qword ptr [rsp + 16]
        mov     dl, byte ptr [rsp + 24]
        add     rsp, 48
        ret

core::ptr::real_drop_in_place:
        push    rax
        call    qword ptr [rip + <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop@GOTPCREL]
        pop     rax
        ret

core::ptr::real_drop_in_place:
        sub     rsp, 24
        mov     qword ptr [rsp], rdi
        mov     rdi, qword ptr [rsp]
        mov     rax, qword ptr [rip + <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop@GOTPCREL]
        call    rax
        jmp     .LBB33_4
.LBB33_1:
        mov     rdi, qword ptr [rsp + 8]
        call    _Unwind_Resume@PLT
        ud2
.LBB33_2:
        add     rsp, 24
        ret
.LBB33_3:
        mov     rax, qword ptr [rsp]
        mov     rdi, rax
        call    core::ptr::real_drop_in_place
        jmp     .LBB33_1
.LBB33_4:
        mov     rax, qword ptr [rsp]
        mov     rdi, rax
        call    core::ptr::real_drop_in_place
        jmp     .LBB33_2
        mov     qword ptr [rsp + 8], rax
        mov     dword ptr [rsp + 16], edx
        jmp     .LBB33_3

core::ptr::real_drop_in_place:
        sub     rsp, 24
        mov     qword ptr [rsp], rdi
        jmp     .LBB34_2
.LBB34_1:
        add     rsp, 24
        ret
.LBB34_2:
        mov     rax, qword ptr [rsp]
        mov     rdi, qword ptr [rax]
        mov     rsi, qword ptr [rax + 8]
        call    qword ptr [rip + alloc::alloc::box_free@GOTPCREL]
        jmp     .LBB34_1

core::ptr::real_drop_in_place:
        ret

core::ptr::swap_nonoverlapping:
        sub     rsp, 40
        mov     qword ptr [rsp + 32], 8
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 16], rdi
        mov     qword ptr [rsp + 8], rsi
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        mov     rcx, qword ptr [rsp + 24]
        imul    rax, rcx
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        mov     rdx, rax
        call    core::ptr::swap_nonoverlapping_bytes
        add     rsp, 40
        ret

core::ptr::slice_from_raw_parts:
        sub     rsp, 32
        mov     qword ptr [rsp + 16], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     rax, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rsp], rax
        mov     qword ptr [rsp + 8], rcx
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 32
        ret

core::ptr::swap_nonoverlapping_one:
        sub     rsp, 72
        mov     byte ptr [rsp + 47], 0
        mov     qword ptr [rsp + 64], 8
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 32], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 16], rax
        jmp     .LBB38_2
.LBB38_1:
        mov     rdi, qword ptr [rsp + 48]
        call    _Unwind_Resume@PLT
        ud2
.LBB38_2:
        mov     rax, qword ptr [rsp + 16]
        cmp     rax, 32
        jb      .LBB38_4
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        mov     edx, 1
        call    qword ptr [rip + core::ptr::swap_nonoverlapping@GOTPCREL]
        jmp     .LBB38_8
.LBB38_4:
        mov     byte ptr [rsp + 47], 1
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + core::ptr::read@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     edx, 1
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        call    rax
        jmp     .LBB38_6
.LBB38_6:
        mov     byte ptr [rsp + 47], 0
        mov     rax, qword ptr [rip + core::ptr::write@GOTPCREL]
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 8]
        call    rax
        jmp     .LBB38_7
.LBB38_7:
        mov     byte ptr [rsp + 47], 0
        jmp     .LBB38_9
.LBB38_8:
        jmp     .LBB38_9
.LBB38_9:
        add     rsp, 72
        ret
.LBB38_10:
        mov     byte ptr [rsp + 47], 0
        jmp     .LBB38_1
.LBB38_11:
        test    byte ptr [rsp + 47], 1
        jne     .LBB38_10
        jmp     .LBB38_1
        mov     qword ptr [rsp + 48], rax
        mov     dword ptr [rsp + 56], edx
        jmp     .LBB38_11

core::ptr::slice_from_raw_parts_mut:
        sub     rsp, 32
        mov     qword ptr [rsp + 16], rdi
        mov     qword ptr [rsp + 24], rsi
        mov     rax, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rsp], rax
        mov     qword ptr [rsp + 8], rcx
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 32
        ret

core::ptr::swap_nonoverlapping_bytes:
        push    rbp
        mov     rbp, rsp
        and     rsp, -32
        sub     rsp, 256
        mov     qword ptr [rsp + 200], 32
        mov     rax, qword ptr [rsp + 200]
        mov     qword ptr [rsp + 112], rdi
        mov     qword ptr [rsp + 104], rsi
        mov     qword ptr [rsp + 96], rdx
        mov     qword ptr [rsp + 88], rax
        mov     qword ptr [rsp + 120], 0
.LBB40_2:
        mov     rax, qword ptr [rsp + 88]
        add     rax, qword ptr [rsp + 120]
        mov     rcx, qword ptr [rsp + 96]
        cmp     rax, rcx
        jbe     .LBB40_4
        mov     rax, qword ptr [rsp + 96]
        cmp     qword ptr [rsp + 120], rax
        jb      .LBB40_12
        jmp     .LBB40_20
.LBB40_4:
        lea     rax, [rsp + 128]
        mov     qword ptr [rsp + 208], rax
        mov     rax, qword ptr [rsp + 208]
        mov     qword ptr [rsp + 216], rax
        mov     rax, qword ptr [rsp + 216]
        mov     qword ptr [rsp + 80], rax
        mov     rax, qword ptr [rsp + 80]
        mov     rsi, qword ptr [rsp + 120]
        mov     rdi, qword ptr [rsp + 112]
        mov     qword ptr [rsp + 72], rax
        call    qword ptr [rip + core::ptr::<impl *mut T>::add@GOTPCREL]
        mov     qword ptr [rsp + 64], rax
        mov     rsi, qword ptr [rsp + 120]
        mov     rdi, qword ptr [rsp + 104]
        call    qword ptr [rip + core::ptr::<impl *mut T>::add@GOTPCREL]
        mov     qword ptr [rsp + 56], rax
        mov     rdi, qword ptr [rsp + 64]
        mov     rsi, qword ptr [rsp + 72]
        mov     rdx, qword ptr [rsp + 88]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rdi, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 64]
        mov     rdx, qword ptr [rsp + 88]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rdi, qword ptr [rsp + 72]
        mov     rsi, qword ptr [rsp + 56]
        mov     rdx, qword ptr [rsp + 88]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rax, qword ptr [rsp + 88]
        add     rax, qword ptr [rsp + 120]
        mov     qword ptr [rsp + 120], rax
        jmp     .LBB40_2
.LBB40_12:
        mov     rax, qword ptr [rsp + 96]
        sub     rax, qword ptr [rsp + 120]
        lea     rcx, [rsp + 168]
        mov     qword ptr [rsp + 224], rcx
        mov     rcx, qword ptr [rsp + 224]
        mov     qword ptr [rsp + 232], rcx
        mov     rcx, qword ptr [rsp + 232]
        mov     qword ptr [rsp + 48], rax
        mov     qword ptr [rsp + 40], rcx
        mov     rax, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 120]
        mov     rdi, qword ptr [rsp + 112]
        mov     qword ptr [rsp + 32], rax
        call    qword ptr [rip + core::ptr::<impl *mut T>::add@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        mov     rsi, qword ptr [rsp + 120]
        mov     rdi, qword ptr [rsp + 104]
        call    qword ptr [rip + core::ptr::<impl *mut T>::add@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 32]
        mov     rdx, qword ptr [rsp + 48]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 48]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 48]
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        jmp     .LBB40_20
.LBB40_20:
        mov     rsp, rbp
        pop     rbp
        ret

core::ptr::<impl *mut T>::add:
        push    rax
        call    qword ptr [rip + core::ptr::<impl *mut T>::offset@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::ptr::<impl *mut T>::offset:
        sub     rsp, 16
        add     rdi, rsi
        mov     qword ptr [rsp + 8], rdi
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 16
        ret

core::ptr::<impl *mut T>::is_null:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        pop     rcx
        ret

core::ptr::<impl *mut T>::is_null:
        push    rax
        mov     qword ptr [rsp], rdi
        jmp     .LBB44_1
.LBB44_1:
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        pop     rcx
        ret

core::ptr::<impl *const T>::wrapping_add:
        push    rax
        call    qword ptr [rip + core::ptr::<impl *const T>::wrapping_offset@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::ptr::<impl *const T>::wrapping_offset:
        sub     rsp, 16
        add     rdi, rsi
        mov     qword ptr [rsp + 8], rdi
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 16
        ret

core::ptr::<impl *const T>::add:
        push    rax
        call    qword ptr [rip + core::ptr::<impl *const T>::offset@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::ptr::<impl *const T>::offset:
        sub     rsp, 16
        shl     rsi, 2
        add     rdi, rsi
        mov     qword ptr [rsp + 8], rdi
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 16
        ret

core::ptr::<impl *const T>::is_null:
        push    rax
        mov     qword ptr [rsp], rdi
        jmp     .LBB49_1
.LBB49_1:
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        pop     rcx
        ret

core::ptr::<impl *const T>::is_null:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        pop     rcx
        ret

core::ptr::<impl *const T>::is_null:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        cmp     rax, 0
        sete    cl
        and     cl, 1
        movzx   edx, cl
        mov     eax, edx
        pop     rcx
        ret

core::ptr::read:
        sub     rsp, 56
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 16], rdi
        lea     rax, [rsp + 24]
        mov     qword ptr [rsp + 40], rax
        mov     rax, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 48], rax
        mov     rsi, qword ptr [rsp + 48]
        mov     qword ptr [rsp + 8], rsi
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        mov     edx, 1
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rax, qword ptr [rsp + 24]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 56
        ret

core::ptr::swap:
        sub     rsp, 72
        mov     eax, dword ptr [rsp + 36]
        mov     dword ptr [rsp + 32], eax
        mov     qword ptr [rsp + 24], rdi
        mov     qword ptr [rsp + 16], rsi
        lea     rax, [rsp + 32]
        mov     qword ptr [rsp + 40], rax
        mov     rax, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 48], rax
        mov     rsi, qword ptr [rsp + 48]
        mov     qword ptr [rsp + 8], rsi
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 8]
        mov     edx, 1
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 24]
        mov     edx, 1
        call    qword ptr [rip + core::intrinsics::copy@GOTPCREL]
        lea     rax, [rsp + 32]
        mov     qword ptr [rsp + 56], rax
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 64], rax
        mov     rdi, qword ptr [rsp + 64]
        mov     qword ptr [rsp], rdi
        mov     rdi, qword ptr [rsp]
        mov     rsi, qword ptr [rsp + 16]
        mov     edx, 1
        call    qword ptr [rip + core::intrinsics::copy_nonoverlapping@GOTPCREL]
        add     rsp, 72
        ret

core::ptr::write:
        mov     qword ptr [rdi], rsi
        ret

core::ptr::unique::Unique<T>::new_unchecked:
        sub     rsp, 16
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        add     rsp, 16
        ret

core::ptr::unique::Unique<T>::new_unchecked:
        sub     rsp, 24
        mov     qword ptr [rsp], rdi
        mov     qword ptr [rsp + 8], rsi
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 24
        ret

core::ptr::unique::Unique<T>::as_mut:
        sub     rsp, 24
        mov     rax, qword ptr [rdi]
        mov     rsi, qword ptr [rdi + 8]
        mov     rdi, rax
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 24
        ret

core::ptr::unique::Unique<T>::as_ptr:
        mov     rax, rdi
        mov     rdx, rsi
        ret

core::ptr::unique::Unique<T>::as_ptr:
        mov     rax, rdi
        ret

core::ptr::non_null::NonNull<T>::new_unchecked:
        sub     rsp, 16
        mov     qword ptr [rsp], rdi
        mov     qword ptr [rsp + 8], rsi
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 16
        ret

core::ptr::non_null::NonNull<T>::new_unchecked:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::ptr::non_null::NonNull<T>::new_unchecked:
        push    rax
        mov     qword ptr [rsp], rdi
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::ptr::non_null::NonNull<T>::cast:
        sub     rsp, 24
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 16]
        mov     rdi, rax
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::new_unchecked@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 8]
        add     rsp, 24
        ret

core::ptr::non_null::NonNull<T>::as_ptr:
        mov     rax, rdi
        ret

core::ptr::non_null::NonNull<T>::as_ptr:
        mov     rax, rdi
        ret

core::ptr::non_null::NonNull<T>::as_ptr:
        mov     rax, rdi
        mov     rdx, rsi
        ret

core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next:
        sub     rsp, 88
        mov     qword ptr [rsp + 16], rdi
        mov     byte ptr [rsp + 71], 0
        mov     rax, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rsp + 16]
        add     rcx, 8
        mov     rdi, rax
        mov     rsi, rcx
        call    core::cmp::impls::<impl core::cmp::PartialOrd for usize>::lt
        mov     byte ptr [rsp + 15], al
        jmp     .LBB67_2
.LBB67_1:
        mov     rdi, qword ptr [rsp + 72]
        call    _Unwind_Resume@PLT
        ud2
.LBB67_2:
        mov     al, byte ptr [rsp + 15]
        test    al, 1
        jne     .LBB67_4
        jmp     .LBB67_3
.LBB67_3:
        mov     qword ptr [rsp + 24], 0
        jmp     .LBB67_11
.LBB67_4:
        mov     rdi, qword ptr [rsp + 16]
        mov     byte ptr [rsp + 71], 1
        mov     esi, 1
        call    <usize as core::iter::range::Step>::add_usize
        mov     qword ptr [rsp + 48], rdx
        mov     qword ptr [rsp + 40], rax
        cmp     qword ptr [rsp + 40], 1
        je      .LBB67_7
        mov     qword ptr [rsp + 24], 0
        jmp     .LBB67_17
.LBB67_7:
        mov     byte ptr [rsp + 71], 0
        mov     rax, qword ptr [rsp + 48]
        mov     qword ptr [rsp + 56], rax
        mov     rsi, qword ptr [rsp + 16]
        mov     rax, qword ptr [rip + core::mem::swap@GOTPCREL]
        lea     rdi, [rsp + 56]
        call    rax
        jmp     .LBB67_8
.LBB67_8:
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], 1
        jmp     .LBB67_17
.LBB67_9:
        cmp     qword ptr [rsp + 40], 1
        je      .LBB67_1
        jmp     .LBB67_12
.LBB67_10:
        jmp     .LBB67_9
.LBB67_11:
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 88
        ret
.LBB67_12:
        jmp     .LBB67_1
.LBB67_13:
        mov     byte ptr [rsp + 71], 0
        jmp     .LBB67_11
.LBB67_14:
        test    byte ptr [rsp + 71], 1
        je      .LBB67_13
        mov     byte ptr [rsp + 71], 0
        jmp     .LBB67_13
.LBB67_16:
        jmp     .LBB67_13
.LBB67_17:
        cmp     qword ptr [rsp + 40], 1
        je      .LBB67_14
        jmp     .LBB67_16
        mov     qword ptr [rsp + 72], rax
        mov     dword ptr [rsp + 80], edx
        jmp     .LBB67_10

core::alloc::Layout::from_size_align_unchecked:
        sub     rsp, 40
        mov     qword ptr [rsp + 16], rdi
        mov     rdi, rsi
        call    core::num::NonZeroUsize::new_unchecked
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 24], rax
        mov     rcx, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 32], rcx
        mov     rax, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 32]
        add     rsp, 40
        ret

core::alloc::Layout::size:
        mov     rax, qword ptr [rdi]
        ret

core::alloc::Layout::align:
        push    rax
        mov     rdi, qword ptr [rdi + 8]
        call    core::num::NonZeroUsize::get
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

core::slice::from_raw_parts:
        sub     rsp, 72
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 56], rdi
        mov     qword ptr [rsp + 48], rsi
        jne     .LBB71_1
        jmp     .LBB71_5
.LBB71_1:
        mov     rdi, qword ptr [rsp + 56]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 47], al
        mov     al, byte ptr [rsp + 47]
        xor     al, -1
        test    al, 1
        jne     .LBB71_4
        jmp     .LBB71_5
.LBB71_4:
        lea     rdi, [rip + .L__unnamed_11]
        lea     rdx, [rip + .L__unnamed_12]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 41
        call    rax
        ud2
.LBB71_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB71_6
        jmp     .LBB71_11
.LBB71_6:
        mov     qword ptr [rsp + 64], 4
        mov     rdi, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 48]
        call    core::num::<impl usize>::saturating_mul
        mov     qword ptr [rsp + 24], rax
        movabs  rax, 9223372036854775807
        mov     rcx, qword ptr [rsp + 24]
        cmp     rcx, rax
        setbe   dl
        xor     dl, -1
        test    dl, 1
        jne     .LBB71_10
        jmp     .LBB71_11
.LBB71_10:
        lea     rdi, [rip + .L__unnamed_13]
        lea     rdx, [rip + .L__unnamed_14]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 55
        call    rax
        ud2
.LBB71_11:
        mov     rdi, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 48]
        call    qword ptr [rip + core::ptr::slice_from_raw_parts@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 72
        ret

core::slice::from_raw_parts_mut:
        sub     rsp, 72
        xor     eax, eax
        test    al, 1
        mov     qword ptr [rsp + 56], rdi
        mov     qword ptr [rsp + 48], rsi
        jne     .LBB72_1
        jmp     .LBB72_5
.LBB72_1:
        mov     rdi, qword ptr [rsp + 56]
        call    qword ptr [rip + core::intrinsics::is_aligned_and_not_null@GOTPCREL]
        mov     byte ptr [rsp + 47], al
        mov     al, byte ptr [rsp + 47]
        xor     al, -1
        test    al, 1
        jne     .LBB72_4
        jmp     .LBB72_5
.LBB72_4:
        lea     rdi, [rip + .L__unnamed_11]
        lea     rdx, [rip + .L__unnamed_15]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 41
        call    rax
        ud2
.LBB72_5:
        xor     eax, eax
        test    al, 1
        jne     .LBB72_6
        jmp     .LBB72_11
.LBB72_6:
        mov     qword ptr [rsp + 64], 4
        mov     rdi, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 48]
        call    core::num::<impl usize>::saturating_mul
        mov     qword ptr [rsp + 24], rax
        movabs  rax, 9223372036854775807
        mov     rcx, qword ptr [rsp + 24]
        cmp     rcx, rax
        setbe   dl
        xor     dl, -1
        test    dl, 1
        jne     .LBB72_10
        jmp     .LBB72_11
.LBB72_10:
        lea     rdi, [rip + .L__unnamed_13]
        lea     rdx, [rip + .L__unnamed_16]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 55
        call    rax
        ud2
.LBB72_11:
        mov     rdi, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 48]
        call    qword ptr [rip + core::ptr::slice_from_raw_parts_mut@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 72
        ret

core::slice::<impl [T]>::len:
        sub     rsp, 16
        mov     qword ptr [rsp], rdi
        mov     qword ptr [rsp + 8], rsi
        mov     rax, qword ptr [rsp + 8]
        add     rsp, 16
        ret

core::slice::<impl [T]>::iter:
        sub     rsp, 104
        mov     qword ptr [rsp + 56], rdi
        mov     qword ptr [rsp + 48], rsi
        call    qword ptr [rip + core::slice::<impl [T]>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 40], rax
        mov     rdi, qword ptr [rsp + 40]
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        mov     qword ptr [rsp + 96], 4
        mov     rax, qword ptr [rsp + 96]
        mov     qword ptr [rsp + 32], rax
        mov     rax, qword ptr [rsp + 32]
        cmp     rax, 0
        je      .LBB74_6
        mov     rdi, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 48]
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        jmp     .LBB74_9
.LBB74_6:
        mov     rax, qword ptr [rsp + 40]
        mov     rdi, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 48]
        mov     qword ptr [rsp + 16], rax
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::ptr::<impl *const T>::wrapping_add@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 80], rax
        jmp     .LBB74_11
.LBB74_9:
        mov     rdi, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::ptr::<impl *const T>::add@GOTPCREL]
        mov     qword ptr [rsp + 80], rax
        jmp     .LBB74_11
.LBB74_11:
        mov     rax, qword ptr [rsp + 80]
        mov     rcx, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 64], rcx
        mov     qword ptr [rsp + 72], rax
        mov     rax, qword ptr [rsp + 64]
        mov     rdx, qword ptr [rsp + 72]
        add     rsp, 104
        ret

core::slice::<impl [T]>::swap:
        sub     rsp, 56
        mov     qword ptr [rsp + 40], rdi
        mov     qword ptr [rsp + 48], rsi
        mov     rax, qword ptr [rsp + 48]
        cmp     rdx, rax
        setb    r8b
        test    r8b, 1
        mov     qword ptr [rsp + 32], rdx
        mov     qword ptr [rsp + 24], rcx
        mov     qword ptr [rsp + 16], rax
        jne     .LBB75_1
        jmp     .LBB75_4
.LBB75_1:
        mov     rax, qword ptr [rsp + 32]
        shl     rax, 2
        add     rax, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 48]
        mov     rdx, qword ptr [rsp + 24]
        cmp     rdx, rcx
        setb    sil
        test    sil, 1
        mov     qword ptr [rsp + 8], rax
        mov     qword ptr [rsp], rcx
        jne     .LBB75_2
        jmp     .LBB75_5
.LBB75_2:
        mov     rax, qword ptr [rsp + 24]
        shl     rax, 2
        add     rax, qword ptr [rsp + 40]
        mov     rdi, qword ptr [rsp + 8]
        mov     rsi, rax
        call    qword ptr [rip + core::ptr::swap@GOTPCREL]
        add     rsp, 56
        ret
.LBB75_4:
        lea     rdi, [rip + .L__unnamed_17]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        mov     rsi, qword ptr [rsp + 32]
        mov     rdx, qword ptr [rsp + 16]
        call    rax
        ud2
.LBB75_5:
        lea     rdi, [rip + .L__unnamed_18]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp]
        call    rax
        ud2

core::slice::<impl [T]>::as_ptr:
        mov     rax, rdi
        ret

core::slice::<impl core::ops::index::Index<I> for [T]>::index:
        sub     rsp, 24
        call    qword ptr [rip + <core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 24
        ret

core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut:
        sub     rsp, 24
        call    qword ptr [rip + <core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index_mut@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 24
        ret

core::slice::<impl core::cmp::PartialEq<[B]> for [A]>::eq:
        push    rax
        call    qword ptr [rip + <[A] as core::slice::SlicePartialEq<A>>::equal@GOTPCREL]
        mov     byte ptr [rsp + 7], al
        mov     al, byte ptr [rsp + 7]
        and     al, 1
        movzx   eax, al
        pop     rcx
        ret

core::option::Option<T>::unwrap:
        sub     rsp, 40
        mov     qword ptr [rsp + 8], rdi
        mov     qword ptr [rsp + 16], rsi
        mov     rax, qword ptr [rsp + 8]
        test    rax, rax
        je      .LBB80_2
        jmp     .LBB80_10
.LBB80_10:
        jmp     .LBB80_4
.LBB80_1:
        mov     rdi, qword ptr [rsp + 24]
        call    _Unwind_Resume@PLT
        ud2
.LBB80_2:
        lea     rdi, [rip + .L__unnamed_19]
        lea     rdx, [rip + .L__unnamed_20]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 43
        call    rax
        jmp     .LBB80_8
        ud2
.LBB80_4:
        mov     rax, qword ptr [rsp + 16]
        cmp     qword ptr [rsp + 8], 1
        mov     qword ptr [rsp], rax
        je      .LBB80_6
        jmp     .LBB80_7
.LBB80_5:
        jmp     .LBB80_1
.LBB80_6:
        mov     rax, qword ptr [rsp]
        add     rsp, 40
        ret
.LBB80_7:
        jmp     .LBB80_6
.LBB80_8:
        ud2
        mov     qword ptr [rsp + 24], rax
        mov     dword ptr [rsp + 32], edx
        jmp     .LBB80_5

core::option::Option<T>::unwrap_or:
        sub     rsp, 56
        mov     qword ptr [rsp + 8], rdi
        mov     qword ptr [rsp + 16], rsi
        mov     byte ptr [rsp + 38], 0
        mov     byte ptr [rsp + 39], 0
        mov     byte ptr [rsp + 38], 1
        mov     byte ptr [rsp + 39], 1
        mov     rax, qword ptr [rsp + 8]
        test    rax, rax
        mov     qword ptr [rsp], rdx
        je      .LBB81_1
        jmp     .LBB81_11
.LBB81_11:
        jmp     .LBB81_3
.LBB81_1:
        mov     byte ptr [rsp + 39], 0
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 24], rax
        jmp     .LBB81_6
        ud2
.LBB81_3:
        mov     byte ptr [rsp + 38], 0
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 24], rax
        jmp     .LBB81_6
.LBB81_4:
        cmp     qword ptr [rsp + 8], 1
        je      .LBB81_8
        jmp     .LBB81_10
.LBB81_5:
        mov     byte ptr [rsp + 39], 0
        jmp     .LBB81_4
.LBB81_6:
        test    byte ptr [rsp + 39], 1
        jne     .LBB81_5
        jmp     .LBB81_4
.LBB81_7:
        mov     rax, qword ptr [rsp + 24]
        add     rsp, 56
        ret
.LBB81_8:
        test    byte ptr [rsp + 38], 1
        je      .LBB81_7
        mov     byte ptr [rsp + 38], 0
        jmp     .LBB81_7
.LBB81_10:
        jmp     .LBB81_7

<T as core::convert::From<T>>::from:
        mov     rax, rdi
        ret

<T as core::convert::Into<U>>::into:
        sub     rsp, 24
        call    qword ptr [rip + <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 24
        ret

<T as core::convert::Into<U>>::into:
        push    rax
        call    qword ptr [rip + <T as core::convert::From<T>>::from@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

<T as core::convert::TryFrom<U>>::try_from:
        sub     rsp, 24
        call    qword ptr [rip + <T as core::convert::Into<U>>::into@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 16]
        add     rsp, 24
        ret

<alloc::alloc::Global as core::alloc::Alloc>::dealloc:
        sub     rsp, 40
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, rsi
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 16], rcx
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 8]
        mov     rsi, qword ptr [rsp + 24]
        mov     rdx, qword ptr [rsp + 16]
        call    alloc::alloc::dealloc
        add     rsp, 40
        ret

alloc::vec::Vec<T>::as_mut_ptr:
        sub     rsp, 24
        mov     qword ptr [rsp + 16], rdi
        mov     rax, qword ptr [rsp + 16]
        mov     rdi, rax
        call    qword ptr [rip + alloc::raw_vec::RawVec<T,A>::ptr@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::ptr::<impl *mut T>::is_null@GOTPCREL]
        mov     rax, qword ptr [rsp + 8]
        add     rsp, 24
        ret

alloc::vec::Vec<T>::from_raw_parts:
        sub     rsp, 40
        mov     rax, rdi
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, rsi
        mov     rsi, rcx
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 16], rax
        call    qword ptr [rip + alloc::raw_vec::RawVec<T>::from_raw_parts@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     qword ptr [rsp], rdx
        mov     rax, qword ptr [rsp + 32]
        mov     rcx, qword ptr [rsp + 8]
        mov     qword ptr [rax], rcx
        mov     rdx, qword ptr [rsp]
        mov     qword ptr [rax + 8], rdx
        mov     rsi, qword ptr [rsp + 24]
        mov     qword ptr [rax + 16], rsi
        mov     rax, qword ptr [rsp + 16]
        add     rsp, 40
        ret

alloc::vec::Vec<T>::as_ptr:
        sub     rsp, 24
        mov     qword ptr [rsp + 16], rdi
        mov     rax, qword ptr [rsp + 16]
        mov     rdi, rax
        call    qword ptr [rip + alloc::raw_vec::RawVec<T,A>::ptr@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::ptr::<impl *mut T>::is_null@GOTPCREL]
        mov     rax, qword ptr [rsp + 8]
        add     rsp, 24
        ret

alloc::alloc::exchange_malloc:
        sub     rsp, 56
        cmp     rdi, 0
        mov     qword ptr [rsp + 40], rdi
        mov     qword ptr [rsp + 32], rsi
        je      .LBB90_2
        mov     rdi, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 32]
        call    core::alloc::Layout::from_size_align_unchecked
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 16], rdx
        jmp     .LBB90_3
.LBB90_2:
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 48], rax
        jmp     .LBB90_8
.LBB90_3:
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        call    alloc::alloc::alloc
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 8]
        call    qword ptr [rip + core::ptr::<impl *mut T>::is_null@GOTPCREL]
        mov     byte ptr [rsp + 7], al
        mov     al, byte ptr [rsp + 7]
        xor     al, -1
        test    al, 1
        jne     .LBB90_7
        mov     rax, qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        call    rax
        ud2
.LBB90_7:
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 48], rax
.LBB90_8:
        mov     rax, qword ptr [rsp + 48]
        add     rsp, 56
        ret

alloc::alloc::alloc:
        sub     rsp, 40
        mov     qword ptr [rsp + 24], rdi
        mov     qword ptr [rsp + 32], rsi
        lea     rdi, [rsp + 24]
        call    core::alloc::Layout::size
        mov     qword ptr [rsp + 16], rax
        lea     rdi, [rsp + 24]
        call    core::alloc::Layout::align
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        call    qword ptr [rip + __rust_alloc@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        add     rsp, 40
        ret

alloc::alloc::dealloc:
        sub     rsp, 40
        mov     qword ptr [rsp + 24], rsi
        mov     qword ptr [rsp + 32], rdx
        lea     rax, [rsp + 24]
        mov     qword ptr [rsp + 16], rdi
        mov     rdi, rax
        call    core::alloc::Layout::size
        mov     qword ptr [rsp + 8], rax
        lea     rdi, [rsp + 24]
        call    core::alloc::Layout::align
        mov     qword ptr [rsp], rax
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 8]
        mov     rdx, qword ptr [rsp]
        call    qword ptr [rip + __rust_dealloc@GOTPCREL]
        add     rsp, 40
        ret

alloc::alloc::box_free:
        sub     rsp, 72
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 48], rax
        mov     qword ptr [rsp + 40], rdx
        mov     rax, qword ptr [rsp + 40]
        shl     rax, 2
        mov     qword ptr [rsp + 56], rax
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 64], 4
        mov     rsi, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 24], rsi
        mov     rax, qword ptr [rsp + 32]
        cmp     rax, 0
        je      .LBB93_7
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        call    core::alloc::Layout::from_size_align_unchecked
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 48]
        mov     rdi, rax
        mov     rsi, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        call    alloc::alloc::dealloc
        jmp     .LBB93_7
.LBB93_7:
        add     rsp, 72
        ret

alloc::boxed::Box<T>::into_unique:
        sub     rsp, 56
        mov     rax, rdi
        mov     qword ptr [rsp + 40], rax
        mov     qword ptr [rsp + 48], rsi
        call    qword ptr [rip + core::mem::forget@GOTPCREL]
        lea     rdi, [rsp + 40]
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_mut@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::ptr::unique::Unique<T>::new_unchecked@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 56
        ret

alloc::boxed::Box<T>::into_raw_non_null:
        sub     rsp, 40
        call    qword ptr [rip + alloc::boxed::Box<T>::into_unique@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        call    qword ptr [rip + <T as core::convert::Into<U>>::into@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

alloc::boxed::Box<T>::into_raw:
        sub     rsp, 40
        call    qword ptr [rip + alloc::boxed::Box<T>::into_raw_non_null@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rdx
        mov     rdi, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

alloc::slice::<impl [T]>::into_vec:
        push    rax
        mov     rax, rdi
        mov     qword ptr [rsp], rax
        call    qword ptr [rip + alloc::slice::hack::into_vec@GOTPCREL]
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

alloc::slice::hack::into_vec:
        sub     rsp, 104
        mov     rax, rdi
        mov     qword ptr [rsp + 40], rsi
        mov     qword ptr [rsp + 48], rdx
        mov     byte ptr [rsp + 87], 0
        mov     byte ptr [rsp + 87], 1
        mov     rcx, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 48]
        mov     rdx, qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, rcx
        mov     qword ptr [rsp + 24], rax
        call    rdx
        mov     qword ptr [rsp + 16], rax
        jmp     .LBB98_2
.LBB98_1:
        mov     rdi, qword ptr [rsp + 88]
        call    _Unwind_Resume@PLT
        ud2
.LBB98_2:
        mov     byte ptr [rsp + 87], 0
        mov     rdi, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 48]
        mov     rax, qword ptr [rip + alloc::boxed::Box<T>::into_raw@GOTPCREL]
        call    rax
        mov     qword ptr [rsp + 8], rax
        jmp     .LBB98_3
.LBB98_3:
        mov     rax, qword ptr [rip + alloc::vec::Vec<T>::from_raw_parts@GOTPCREL]
        lea     rdi, [rsp + 56]
        mov     rsi, qword ptr [rsp + 8]
        mov     rdx, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rsp + 16]
        call    rax
        jmp     .LBB98_4
.LBB98_4:
        mov     rax, qword ptr [rsp + 56]
        mov     rcx, qword ptr [rsp + 32]
        mov     qword ptr [rcx], rax
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rcx + 8], rax
        mov     rax, qword ptr [rsp + 72]
        mov     qword ptr [rcx + 16], rax
        mov     rax, qword ptr [rsp + 24]
        add     rsp, 104
        ret
.LBB98_5:
        mov     byte ptr [rsp + 87], 0
        lea     rdi, [rsp + 40]
        call    core::ptr::real_drop_in_place
        jmp     .LBB98_1
.LBB98_6:
        test    byte ptr [rsp + 87], 1
        jne     .LBB98_5
        jmp     .LBB98_1
        mov     qword ptr [rsp + 88], rax
        mov     dword ptr [rsp + 96], edx
        jmp     .LBB98_6

alloc::raw_vec::RawVec<T>::from_raw_parts:
        sub     rsp, 40
        mov     qword ptr [rsp + 8], rsi
        call    qword ptr [rip + core::ptr::unique::Unique<T>::new_unchecked@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 16], rax
        mov     rcx, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 24], rcx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 24]
        add     rsp, 40
        ret

alloc::raw_vec::RawVec<T,A>::current_layout:
        sub     rsp, 72
        cmp     qword ptr [rdi + 8], 0
        mov     qword ptr [rsp + 32], rdi
        je      .LBB100_2
        mov     qword ptr [rsp + 56], 4
        mov     rsi, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 24], rsi
        jmp     .LBB100_3
.LBB100_2:
        mov     qword ptr [rsp + 48], 0
        jmp     .LBB100_6
.LBB100_3:
        mov     qword ptr [rsp + 64], 4
        mov     rax, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rsp + 32]
        imul    rax, qword ptr [rcx + 8]
        mov     rdi, rax
        mov     rsi, qword ptr [rsp + 24]
        call    core::alloc::Layout::from_size_align_unchecked
        mov     qword ptr [rsp + 8], rax
        mov     qword ptr [rsp], rdx
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 40], rax
        mov     rcx, qword ptr [rsp]
        mov     qword ptr [rsp + 48], rcx
.LBB100_6:
        mov     rax, qword ptr [rsp + 40]
        mov     rdx, qword ptr [rsp + 48]
        add     rsp, 72
        ret

alloc::raw_vec::RawVec<T,A>::dealloc_buffer:
        sub     rsp, 88
        mov     qword ptr [rsp + 56], rdi
        mov     qword ptr [rsp + 80], 4
        mov     rax, qword ptr [rsp + 80]
        mov     qword ptr [rsp + 48], rax
        mov     rax, qword ptr [rsp + 48]
        cmp     rax, 0
        je      .LBB101_9
        mov     rdi, qword ptr [rsp + 56]
        mov     rax, qword ptr [rip + alloc::raw_vec::RawVec<T,A>::current_layout@GOTPCREL]
        call    rax
        mov     qword ptr [rsp + 72], rdx
        mov     qword ptr [rsp + 64], rax
        xor     eax, eax
        mov     ecx, eax
        cmp     qword ptr [rsp + 72], 0
        mov     edx, 1
        cmovbe  rdx, rcx
        cmp     rdx, 1
        jne     .LBB101_8
        mov     rdx, qword ptr [rsp + 64]
        mov     rcx, qword ptr [rsp + 72]
        mov     rax, qword ptr [rsp + 56]
        mov     rsi, qword ptr [rsp + 56]
        mov     rdi, qword ptr [rsi]
        mov     qword ptr [rsp + 40], rdx
        mov     qword ptr [rsp + 32], rcx
        mov     qword ptr [rsp + 24], rax
        call    qword ptr [rip + <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rdi, qword ptr [rsp + 16]
        call    qword ptr [rip + core::ptr::non_null::NonNull<T>::cast@GOTPCREL]
        mov     qword ptr [rsp + 8], rax
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 8]
        mov     rdx, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 32]
        call    <alloc::alloc::Global as core::alloc::Alloc>::dealloc
        jmp     .LBB101_8
.LBB101_8:
        jmp     .LBB101_9
.LBB101_9:
        add     rsp, 88
        ret

alloc::raw_vec::RawVec<T,A>::ptr:
        push    rax
        mov     rdi, qword ptr [rdi]
        call    qword ptr [rip + core::ptr::unique::Unique<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp], rax
        mov     rax, qword ptr [rsp]
        pop     rcx
        ret

<alloc::vec::Vec<T> as core::fmt::Debug>::fmt:
        sub     rsp, 40
        mov     qword ptr [rsp + 32], rsi
        call    qword ptr [rip + <alloc::vec::Vec<T> as core::ops::deref::Deref>::deref@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 16], rdx
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 32]
        call    qword ptr [rip + <[T] as core::fmt::Debug>::fmt@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        and     al, 1
        movzx   eax, al
        add     rsp, 40
        ret

<I as core::iter::traits::collect::IntoIterator>::into_iter:
        mov     rax, rdi
        mov     rdx, rsi
        ret

<I as core::iter::traits::collect::IntoIterator>::into_iter:
        mov     rax, rdi
        mov     rdx, rsi
        ret

<alloc::vec::Vec<T> as core::ops::drop::Drop>::drop:
        push    rax
        call    qword ptr [rip + <alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut@GOTPCREL]
        jmp     .LBB106_2
.LBB106_2:
        pop     rax
        ret

<[A] as core::slice::SlicePartialEq<A>>::equal:
        sub     rsp, 120
        mov     qword ptr [rsp + 104], rdi
        mov     qword ptr [rsp + 96], rsi
        mov     qword ptr [rsp + 88], rdx
        mov     qword ptr [rsp + 80], rcx
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 72], rax
        mov     rdi, qword ptr [rsp + 88]
        mov     rsi, qword ptr [rsp + 80]
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 64], rax
        mov     rax, qword ptr [rsp + 72]
        mov     rcx, qword ptr [rsp + 64]
        cmp     rax, rcx
        jne     .LBB107_4
        mov     rdi, qword ptr [rsp + 104]
        mov     rsi, qword ptr [rsp + 96]
        call    qword ptr [rip + core::slice::<impl [T]>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB107_6
.LBB107_4:
        mov     byte ptr [rsp + 119], 0
.LBB107_5:
        mov     al, byte ptr [rsp + 119]
        and     al, 1
        movzx   eax, al
        add     rsp, 120
        ret
.LBB107_6:
        mov     rdi, qword ptr [rsp + 88]
        mov     rsi, qword ptr [rsp + 80]
        call    qword ptr [rip + core::slice::<impl [T]>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 48], rax
        mov     rax, qword ptr [rsp + 56]
        mov     rcx, qword ptr [rsp + 48]
        cmp     rax, rcx
        je      .LBB107_9
        mov     rdi, qword ptr [rsp + 104]
        mov     rsi, qword ptr [rsp + 96]
        call    qword ptr [rip + core::mem::size_of_val@GOTPCREL]
        mov     qword ptr [rsp + 40], rax
        jmp     .LBB107_10
.LBB107_9:
        mov     byte ptr [rsp + 119], 1
        jmp     .LBB107_5
.LBB107_10:
        mov     rdi, qword ptr [rsp + 104]
        mov     rsi, qword ptr [rsp + 96]
        call    qword ptr [rip + core::slice::<impl [T]>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 32], rax
        mov     rax, qword ptr [rsp + 32]
        mov     rdi, qword ptr [rsp + 88]
        mov     rsi, qword ptr [rsp + 80]
        mov     qword ptr [rsp + 24], rax
        call    qword ptr [rip + core::slice::<impl [T]>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 16]
        mov     rcx, qword ptr [rip + memcmp@GOTPCREL]
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, rax
        mov     rdx, qword ptr [rsp + 40]
        call    rcx
        mov     dword ptr [rsp + 12], eax
        mov     eax, dword ptr [rsp + 12]
        cmp     eax, 0
        sete    cl
        and     cl, 1
        mov     byte ptr [rsp + 119], cl
        jmp     .LBB107_5

<alloc::vec::Vec<T> as core::ops::deref::Deref>::deref:
        sub     rsp, 40
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + alloc::vec::Vec<T>::as_ptr@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        mov     rax, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rax + 16]
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::slice::from_raw_parts@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

<alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut:
        sub     rsp, 40
        mov     qword ptr [rsp + 32], rdi
        mov     rdi, qword ptr [rsp + 32]
        call    qword ptr [rip + alloc::vec::Vec<T>::as_mut_ptr@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        mov     rax, qword ptr [rsp + 32]
        mov     rsi, qword ptr [rax + 16]
        mov     rdi, qword ptr [rsp + 24]
        call    qword ptr [rip + core::slice::from_raw_parts_mut@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        mov     qword ptr [rsp + 8], rdx
        mov     rax, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 40
        ret

<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop:
        push    rax
        call    qword ptr [rip + alloc::raw_vec::RawVec<T,A>::dealloc_buffer@GOTPCREL]
        pop     rax
        ret

<alloc::vec::Vec<T> as core::ops::index::Index<I>>::index:
        sub     rsp, 56
        mov     byte ptr [rsp + 39], 0
        mov     byte ptr [rsp + 39], 1
        mov     rax, qword ptr [rip + <alloc::vec::Vec<T> as core::ops::deref::Deref>::deref@GOTPCREL]
        call    rax
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 16], rax
        jmp     .LBB111_2
.LBB111_1:
        mov     rdi, qword ptr [rsp + 40]
        call    _Unwind_Resume@PLT
        ud2
.LBB111_2:
        mov     byte ptr [rsp + 39], 0
        mov     rax, qword ptr [rip + core::slice::<impl core::ops::index::Index<I> for [T]>::index@GOTPCREL]
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 24]
        call    rax
        mov     qword ptr [rsp + 8], rdx
        mov     qword ptr [rsp], rax
        jmp     .LBB111_3
.LBB111_3:
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 56
        ret
.LBB111_4:
        mov     byte ptr [rsp + 39], 0
        jmp     .LBB111_1
.LBB111_5:
        test    byte ptr [rsp + 39], 1
        jne     .LBB111_4
        jmp     .LBB111_1
        mov     qword ptr [rsp + 40], rax
        mov     dword ptr [rsp + 48], edx
        jmp     .LBB111_5

<alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut:
        sub     rsp, 56
        mov     byte ptr [rsp + 39], 0
        mov     byte ptr [rsp + 39], 1
        mov     rax, qword ptr [rip + <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut@GOTPCREL]
        call    rax
        mov     qword ptr [rsp + 24], rdx
        mov     qword ptr [rsp + 16], rax
        jmp     .LBB112_2
.LBB112_1:
        mov     rdi, qword ptr [rsp + 40]
        call    _Unwind_Resume@PLT
        ud2
.LBB112_2:
        mov     byte ptr [rsp + 39], 0
        mov     rax, qword ptr [rip + core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut@GOTPCREL]
        mov     rdi, qword ptr [rsp + 16]
        mov     rsi, qword ptr [rsp + 24]
        call    rax
        mov     qword ptr [rsp + 8], rdx
        mov     qword ptr [rsp], rax
        jmp     .LBB112_3
.LBB112_3:
        mov     rax, qword ptr [rsp]
        mov     rdx, qword ptr [rsp + 8]
        add     rsp, 56
        ret
.LBB112_4:
        mov     byte ptr [rsp + 39], 0
        jmp     .LBB112_1
.LBB112_5:
        test    byte ptr [rsp + 39], 1
        jne     .LBB112_4
        jmp     .LBB112_1
        mov     qword ptr [rsp + 40], rax
        mov     dword ptr [rsp + 48], edx
        jmp     .LBB112_5

<core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::next:
        sub     rsp, 72
        mov     rax, qword ptr [rdi]
        mov     qword ptr [rsp + 24], rdi
        mov     rdi, rax
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        mov     qword ptr [rsp + 40], 4
        mov     rax, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 16]
        cmp     rax, 0
        je      .LBB113_7
        mov     rax, qword ptr [rsp + 24]
        mov     rdi, qword ptr [rax + 8]
        call    qword ptr [rip + core::ptr::<impl *const T>::is_null@GOTPCREL]
        jmp     .LBB113_7
.LBB113_7:
        mov     rax, qword ptr [rsp + 24]
        mov     rcx, qword ptr [rax]
        cmp     rcx, qword ptr [rax + 8]
        je      .LBB113_12
        mov     rax, qword ptr [rsp + 24]
        mov     qword ptr [rsp + 48], rax
        mov     qword ptr [rsp + 64], 4
        cmp     qword ptr [rsp + 64], 0
        je      .LBB113_10
        mov     rax, qword ptr [rsp + 48]
        mov     rax, qword ptr [rax]
        mov     rcx, qword ptr [rsp + 48]
        mov     rdi, qword ptr [rcx]
        mov     esi, 1
        mov     qword ptr [rsp + 8], rax
        call    qword ptr [rip + core::ptr::<impl *const T>::offset@GOTPCREL]
        mov     rcx, qword ptr [rsp + 48]
        mov     qword ptr [rcx], rax
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 56], rax
        jmp     .LBB113_11
.LBB113_10:
        mov     rax, qword ptr [rsp + 48]
        mov     rax, qword ptr [rax + 8]
        mov     rdi, rax
        mov     rsi, -1
        call    qword ptr [rip + core::ptr::<impl *const T>::wrapping_offset@GOTPCREL]
        mov     rcx, qword ptr [rsp + 48]
        mov     qword ptr [rcx + 8], rax
        mov     rax, qword ptr [rsp + 48]
        mov     rax, qword ptr [rax]
        mov     qword ptr [rsp + 56], rax
.LBB113_11:
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp], rax
        jmp     .LBB113_13
.LBB113_12:
        mov     qword ptr [rsp + 32], 0
        jmp     .LBB113_14
.LBB113_13:
        mov     rax, qword ptr [rsp]
        mov     qword ptr [rsp + 32], rax
.LBB113_14:
        mov     rax, qword ptr [rsp + 32]
        add     rsp, 72
        ret

<core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index:
        mov     rax, rdi
        mov     rdx, rsi
        ret

<core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index_mut:
        mov     rax, rdi
        mov     rdx, rsi
        ret

<alloc::vec::Vec<A> as core::cmp::PartialEq<alloc::vec::Vec<B>>>::eq:
        sub     rsp, 72
        mov     qword ptr [rsp + 48], rsi
        call    qword ptr [rip + <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index@GOTPCREL]
        mov     qword ptr [rsp + 40], rax
        mov     qword ptr [rsp + 32], rdx
        mov     rdi, qword ptr [rsp + 48]
        call    qword ptr [rip + <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index@GOTPCREL]
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 16], rdx
        mov     rdi, qword ptr [rsp + 40]
        mov     rsi, qword ptr [rsp + 32]
        mov     rdx, qword ptr [rsp + 24]
        mov     rcx, qword ptr [rsp + 16]
        call    qword ptr [rip + core::slice::<impl core::cmp::PartialEq<[B]> for [A]>::eq@GOTPCREL]
        mov     byte ptr [rsp + 15], al
        mov     al, byte ptr [rsp + 15]
        and     al, 1
        movzx   eax, al
        add     rsp, 72
        ret

example::selection_sort:
        sub     rsp, 232
        mov     qword ptr [rsp + 112], rdi
        mov     qword ptr [rsp + 120], rsi
        mov     rdi, qword ptr [rsp + 112]
        mov     rsi, qword ptr [rsp + 120]
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 104], rax
        mov     qword ptr [rsp + 128], 0
        mov     rax, qword ptr [rsp + 104]
        mov     qword ptr [rsp + 136], rax
        mov     rdi, qword ptr [rsp + 128]
        mov     rsi, qword ptr [rsp + 136]
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 96], rax
        mov     qword ptr [rsp + 88], rdx
        mov     rax, qword ptr [rsp + 96]
        mov     qword ptr [rsp + 144], rax
        mov     rcx, qword ptr [rsp + 88]
        mov     qword ptr [rsp + 152], rcx
.LBB117_3:
        mov     rax, qword ptr [rip + core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next@GOTPCREL]
        lea     rdi, [rsp + 144]
        call    rax
        mov     qword ptr [rsp + 168], rdx
        mov     qword ptr [rsp + 160], rax
        mov     rax, qword ptr [rsp + 160]
        test    rax, rax
        je      .LBB117_5
        jmp     .LBB117_25
.LBB117_25:
        jmp     .LBB117_7
.LBB117_5:
        add     rsp, 232
        ret
        ud2
.LBB117_7:
        mov     rax, qword ptr [rsp + 168]
        mov     qword ptr [rsp + 176], rax
        mov     rcx, rax
        add     rcx, 1
        setb    dl
        test    dl, 1
        mov     qword ptr [rsp + 80], rax
        mov     qword ptr [rsp + 72], rcx
        jne     .LBB117_22
        mov     rdi, qword ptr [rsp + 112]
        mov     rsi, qword ptr [rsp + 120]
        call    qword ptr [rip + core::slice::<impl [T]>::len@GOTPCREL]
        mov     qword ptr [rsp + 64], rax
        mov     rax, qword ptr [rsp + 72]
        mov     qword ptr [rsp + 184], rax
        mov     rcx, qword ptr [rsp + 64]
        mov     qword ptr [rsp + 192], rcx
        mov     rdi, qword ptr [rsp + 184]
        mov     rsi, qword ptr [rsp + 192]
        call    qword ptr [rip + <I as core::iter::traits::collect::IntoIterator>::into_iter@GOTPCREL]
        mov     qword ptr [rsp + 56], rax
        mov     qword ptr [rsp + 48], rdx
        mov     rax, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 200], rax
        mov     rcx, qword ptr [rsp + 48]
        mov     qword ptr [rsp + 208], rcx
.LBB117_11:
        mov     rax, qword ptr [rip + core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next@GOTPCREL]
        lea     rdi, [rsp + 200]
        call    rax
        mov     qword ptr [rsp + 224], rdx
        mov     qword ptr [rsp + 216], rax
        mov     rax, qword ptr [rsp + 216]
        test    rax, rax
        je      .LBB117_13
        jmp     .LBB117_26
.LBB117_26:
        jmp     .LBB117_15
.LBB117_13:
        mov     rdi, qword ptr [rsp + 112]
        mov     rsi, qword ptr [rsp + 120]
        mov     rdx, qword ptr [rsp + 176]
        mov     rcx, qword ptr [rsp + 80]
        call    qword ptr [rip + core::slice::<impl [T]>::swap@GOTPCREL]
        jmp     .LBB117_21
        ud2
.LBB117_15:
        mov     rax, qword ptr [rsp + 224]
        mov     rcx, qword ptr [rsp + 120]
        cmp     rax, rcx
        setb    dl
        test    dl, 1
        mov     qword ptr [rsp + 40], rax
        mov     qword ptr [rsp + 32], rcx
        jne     .LBB117_16
        jmp     .LBB117_23
.LBB117_16:
        mov     rax, qword ptr [rsp + 40]
        shl     rax, 2
        add     rax, qword ptr [rsp + 112]
        mov     rcx, qword ptr [rsp + 176]
        mov     rdx, qword ptr [rsp + 120]
        cmp     rcx, rdx
        setb    sil
        test    sil, 1
        mov     qword ptr [rsp + 24], rax
        mov     qword ptr [rsp + 16], rcx
        mov     qword ptr [rsp + 8], rdx
        jne     .LBB117_17
        jmp     .LBB117_24
.LBB117_17:
        mov     rax, qword ptr [rsp + 16]
        shl     rax, 2
        add     rax, qword ptr [rsp + 112]
        mov     rdi, qword ptr [rsp + 24]
        mov     rsi, rax
        call    core::cmp::impls::<impl core::cmp::PartialOrd for i32>::lt
        mov     byte ptr [rsp + 7], al
        mov     al, byte ptr [rsp + 7]
        test    al, 1
        jne     .LBB117_19
        jmp     .LBB117_20
.LBB117_19:
        mov     rax, qword ptr [rsp + 40]
        mov     qword ptr [rsp + 176], rax
.LBB117_20:
        jmp     .LBB117_11
.LBB117_21:
        jmp     .LBB117_3
.LBB117_22:
        lea     rdi, [rip + str.1]
        lea     rdx, [rip + .L__unnamed_21]
        mov     rax, qword ptr [rip + core::panicking::panic@GOTPCREL]
        mov     esi, 28
        call    rax
        ud2
.LBB117_23:
        lea     rdi, [rip + .L__unnamed_22]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        mov     rsi, qword ptr [rsp + 40]
        mov     rdx, qword ptr [rsp + 32]
        call    rax
        ud2
.LBB117_24:
        lea     rdi, [rip + .L__unnamed_23]
        mov     rax, qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        mov     rsi, qword ptr [rsp + 16]
        mov     rdx, qword ptr [rsp + 8]
        call    rax
        ud2

example::main:
        sub     rsp, 296
        mov     edi, 40
        mov     esi, 4
        call    alloc::alloc::exchange_malloc
        mov     rcx, rax
        mov     dword ptr [rax], 6
        mov     dword ptr [rax + 4], 3
        mov     dword ptr [rax + 8], 7
        mov     dword ptr [rax + 12], 1
        mov     dword ptr [rax + 16], 4
        mov     dword ptr [rax + 20], 2
        mov     dword ptr [rax + 24], 0
        mov     dword ptr [rax + 28], 5
        mov     dword ptr [rax + 32], 9
        mov     dword ptr [rax + 36], 8
        lea     rdi, [rsp + 104]
        mov     rsi, rcx
        mov     edx, 10
        call    qword ptr [rip + alloc::slice::<impl [T]>::into_vec@GOTPCREL]
        jmp     .LBB118_2
.LBB118_1:
        mov     rdi, qword ptr [rsp + 280]
        call    _Unwind_Resume@PLT
        ud2
.LBB118_2:
        mov     rax, qword ptr [rip + <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut@GOTPCREL]
        lea     rdi, [rsp + 104]
        call    rax
        mov     qword ptr [rsp + 96], rdx
        mov     qword ptr [rsp + 88], rax
        jmp     .LBB118_3
.LBB118_3:
        mov     rax, qword ptr [rip + example::selection_sort@GOTPCREL]
        mov     rdi, qword ptr [rsp + 88]
        mov     rsi, qword ptr [rsp + 96]
        call    rax
        jmp     .LBB118_5
.LBB118_4:
        lea     rdi, [rsp + 104]
        call    core::ptr::real_drop_in_place
        jmp     .LBB118_1
.LBB118_5:
        mov     edi, 40
        mov     esi, 4
        call    alloc::alloc::exchange_malloc
        mov     dword ptr [rax], 0
        mov     dword ptr [rax + 4], 1
        mov     dword ptr [rax + 8], 2
        mov     dword ptr [rax + 12], 3
        mov     dword ptr [rax + 16], 4
        mov     dword ptr [rax + 20], 5
        mov     dword ptr [rax + 24], 6
        mov     dword ptr [rax + 28], 7
        mov     dword ptr [rax + 32], 8
        mov     dword ptr [rax + 36], 9
        mov     rcx, qword ptr [rip + alloc::slice::<impl [T]>::into_vec@GOTPCREL]
        lea     rdi, [rsp + 144]
        mov     edx, 10
        mov     rsi, rax
        call    rcx
        jmp     .LBB118_6
.LBB118_6:
        lea     rax, [rsp + 144]
        mov     qword ptr [rsp + 128], rax
        lea     rax, [rsp + 104]
        mov     qword ptr [rsp + 136], rax
        mov     rax, qword ptr [rsp + 128]
        mov     rcx, qword ptr [rsp + 136]
        mov     rdx, qword ptr [rip + <alloc::vec::Vec<A> as core::cmp::PartialEq<alloc::vec::Vec<B>>>::eq@GOTPCREL]
        mov     rdi, rax
        mov     rsi, rcx
        mov     qword ptr [rsp + 80], rax
        mov     qword ptr [rsp + 72], rcx
        call    rdx
        mov     byte ptr [rsp + 71], al
        jmp     .LBB118_9
.LBB118_7:
        lea     rdi, [rsp + 144]
        call    core::ptr::real_drop_in_place
        jmp     .LBB118_1
.LBB118_8:
        lea     rdi, [rsp + 104]
        call    core::ptr::real_drop_in_place
        jmp     .LBB118_7
.LBB118_9:
        mov     al, byte ptr [rsp + 71]
        xor     al, -1
        test    al, 1
        jne     .LBB118_11
        lea     rdi, [rsp + 104]
        call    core::ptr::real_drop_in_place
        jmp     .LBB118_15
.LBB118_11:
        mov     rax, qword ptr [rsp + 80]
        mov     qword ptr [rsp + 264], rax
        mov     rcx, qword ptr [rsp + 72]
        mov     qword ptr [rsp + 272], rcx
        lea     rdx, [rsp + 264]
        mov     qword ptr [rsp + 248], rdx
        lea     rdx, [rsp + 272]
        mov     qword ptr [rsp + 256], rdx
        mov     rdi, qword ptr [rsp + 248]
        mov     rdx, qword ptr [rsp + 256]
        mov     rsi, qword ptr [rip + <&T as core::fmt::Debug>::fmt@GOTPCREL]
        mov     r8, qword ptr [rip + core::fmt::ArgumentV1::new@GOTPCREL]
        mov     qword ptr [rsp + 56], rdx
        call    r8
        mov     qword ptr [rsp + 48], rdx
        mov     qword ptr [rsp + 40], rax
        jmp     .LBB118_12
.LBB118_12:
        mov     rax, qword ptr [rsp + 40]
        mov     rcx, qword ptr [rsp + 48]
        mov     rsi, qword ptr [rip + <&T as core::fmt::Debug>::fmt@GOTPCREL]
        mov     rdx, qword ptr [rip + core::fmt::ArgumentV1::new@GOTPCREL]
        mov     rdi, qword ptr [rsp + 56]
        mov     qword ptr [rsp + 32], rax
        mov     qword ptr [rsp + 24], rcx
        call    rdx
        mov     qword ptr [rsp + 16], rdx
        mov     qword ptr [rsp + 8], rax
        jmp     .LBB118_13
.LBB118_13:
        mov     rax, qword ptr [rsp + 32]
        mov     qword ptr [rsp + 216], rax
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rsp + 224], rcx
        mov     rdx, qword ptr [rsp + 8]
        mov     qword ptr [rsp + 232], rdx
        mov     rsi, qword ptr [rsp + 16]
        mov     qword ptr [rsp + 240], rsi
        lea     rsi, [rip + .L__unnamed_24]
        lea     rdi, [rsp + 168]
        mov     edx, 3
        lea     rcx, [rsp + 216]
        mov     r8d, 2
        call    core::fmt::Arguments::new_v1
        jmp     .LBB118_14
.LBB118_14:
        lea     rsi, [rip + .L__unnamed_25]
        mov     rax, qword ptr [rip + std::panicking::begin_panic_fmt@GOTPCREL]
        lea     rdi, [rsp + 168]
        call    rax
        jmp     .LBB118_19
.LBB118_15:
        lea     rdi, [rsp + 144]
        call    core::ptr::real_drop_in_place
        add     rsp, 296
        ret
        mov     qword ptr [rsp + 280], rax
        mov     dword ptr [rsp + 288], edx
        jmp     .LBB118_4
        mov     qword ptr [rsp + 280], rax
        mov     dword ptr [rsp + 288], edx
        jmp     .LBB118_8
.LBB118_19:
        ud2
        mov     qword ptr [rsp + 280], rax
        mov     dword ptr [rsp + 288], edx
        jmp     .LBB118_7

.L__unnamed_2:
        .ascii  "attempt to copy from unaligned or null pointer"

.L__unnamed_26:
        .ascii  "src/libcore/intrinsics.rs"

.L__unnamed_3:
        .quad   .L__unnamed_26
        .asciz  "\031\000\000\000\000\000\000\000\274\005\000\000\005\000\000"

.L__unnamed_4:
        .ascii  "attempt to copy to unaligned or null pointer"

.L__unnamed_5:
        .quad   .L__unnamed_26
        .asciz  "\031\000\000\000\000\000\000\000\275\005\000\000\005\000\000"

.L__unnamed_6:
        .ascii  "attempt to copy to overlapping memory"

.L__unnamed_7:
        .quad   .L__unnamed_26
        .asciz  "\031\000\000\000\000\000\000\000\276\005\000\000\005\000\000"

.L__unnamed_27:
        .ascii  "/rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/intrinsics.rs"

.L__unnamed_8:
        .quad   .L__unnamed_27
        .asciz  "I\000\000\000\000\000\000\000Q\005\000\000\027\000\000"

str.0:
        .ascii  "attempt to calculate the remainder with a divisor of zero"

.L__unnamed_9:
        .quad   .L__unnamed_26
        .asciz  "\031\000\000\000\000\000\000\000\374\005\000\000\005\000\000"

.L__unnamed_10:
        .quad   .L__unnamed_26
        .asciz  "\031\000\000\000\000\000\000\000\375\005\000\000\005\000\000"

.L__unnamed_1:
        .quad   core::ptr::real_drop_in_place
        .quad   8
        .quad   8
        .quad   <&T as core::fmt::Debug>::fmt

.L__unnamed_11:
        .ascii  "attempt to create unaligned or null slice"

.L__unnamed_28:
        .ascii  "src/libcore/slice/mod.rs"

.L__unnamed_12:
        .quad   .L__unnamed_28
        .asciz  "\030\000\000\000\000\000\000\000\301\024\000\000\005\000\000"

.L__unnamed_13:
        .ascii  "attempt to create slice covering half the address space"

.L__unnamed_14:
        .quad   .L__unnamed_28
        .asciz  "\030\000\000\000\000\000\000\000\302\024\000\000\005\000\000"

.L__unnamed_15:
        .quad   .L__unnamed_28
        .asciz  "\030\000\000\000\000\000\000\000\326\024\000\000\005\000\000"

.L__unnamed_16:
        .quad   .L__unnamed_28
        .asciz  "\030\000\000\000\000\000\000\000\327\024\000\000\005\000\000"

.L__unnamed_29:
        .ascii  "/rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs"

.L__unnamed_17:
        .quad   .L__unnamed_29
        .asciz  "H\000\000\000\000\000\000\000\002\002\000\000#\000\000"

.L__unnamed_18:
        .quad   .L__unnamed_29
        .asciz  "H\000\000\000\000\000\000\000\003\002\000\000#\000\000"

.L__unnamed_19:
        .ascii  "called `Option::unwrap()` on a `None` value"

.L__unnamed_30:
        .ascii  "src/libcore/option.rs"

.L__unnamed_20:
        .quad   .L__unnamed_30
        .asciz  "\025\000\000\000\000\000\000\000z\001\000\000\025\000\000"

.L__unnamed_31:
        .ascii  "./example.rs"

.L__unnamed_21:
        .quad   .L__unnamed_31
        .asciz  "\f\000\000\000\000\000\000\000\004\000\000\000\022\000\000"

str.1:
        .ascii  "attempt to add with overflow"

.L__unnamed_22:
        .quad   .L__unnamed_31
        .asciz  "\f\000\000\000\000\000\000\000\005\000\000\000\020\000\000"

.L__unnamed_23:
        .quad   .L__unnamed_31
        .asciz  "\f\000\000\000\000\000\000\000\005\000\000\000\032\000\000"

.L__unnamed_32:
        .ascii  "assertion failed: `(left == right)`\n  left: `"

.L__unnamed_33:
        .ascii  "`,\n right: `"

.L__unnamed_34:
        .byte   96

.L__unnamed_24:
        .quad   .L__unnamed_32
        .asciz  "-\000\000\000\000\000\000"
        .quad   .L__unnamed_33
        .asciz  "\f\000\000\000\000\000\000"
        .quad   .L__unnamed_34
        .asciz  "\001\000\000\000\000\000\000"

.L__unnamed_25:
        .quad   .L__unnamed_31
        .asciz  "\f\000\000\000\000\000\000\000\020\000\000\000\005\000\000"
*/

// NOTE question 4:
// The optimization flag -O makes a huge difference, the amount of assembly generated is much less,
// however, it mostly just gets rid of things like bounds checking.
/* machine code (with -O)
<&T as core::fmt::Debug>::fmt:
        push    rbp
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        sub     rsp, 24
        mov     rax, qword ptr [rdi]
        mov     rbx, qword ptr [rax]
        mov     r13, qword ptr [rax + 16]
        lea     rdi, [rsp + 8]
        call    qword ptr [rip + core::fmt::Formatter::debug_list@GOTPCREL]
        test    r13, r13
        je      .LBB0_3
        shl     r13, 2
        lea     r14, [rip + .L__unnamed_1]
        lea     r15, [rsp + 8]
        mov     r12, rsp
        mov     rbp, qword ptr [rip + core::fmt::builders::DebugList::entry@GOTPCREL]
.LBB0_2:
        mov     qword ptr [rsp], rbx
        mov     rdi, r15
        mov     rsi, r12
        mov     rdx, r14
        call    rbp
        add     rbx, 4
        add     r13, -4
        jne     .LBB0_2
.LBB0_3:
        lea     rdi, [rsp + 8]
        call    qword ptr [rip + core::fmt::builders::DebugList::finish@GOTPCREL]
        add     rsp, 24
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        pop     rbp
        ret

<&T as core::fmt::Debug>::fmt:
        push    r14
        push    rbx
        push    rax
        mov     rbx, rsi
        mov     r14, qword ptr [rdi]
        mov     rdi, rsi
        call    qword ptr [rip + core::fmt::Formatter::debug_lower_hex@GOTPCREL]
        test    al, al
        je      .LBB1_1
        mov     rdi, r14
        mov     rsi, rbx
        add     rsp, 8
        pop     rbx
        pop     r14
        jmp     qword ptr [rip + _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h6ba9be131e4407d6E@GOTPCREL]
.LBB1_1:
        mov     rdi, rbx
        call    qword ptr [rip + core::fmt::Formatter::debug_upper_hex@GOTPCREL]
        mov     rdi, r14
        mov     rsi, rbx
        add     rsp, 8
        test    al, al
        je      .LBB1_4
        pop     rbx
        pop     r14
        jmp     qword ptr [rip + _ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc8aba6a768cf831cE@GOTPCREL]
.LBB1_4:
        pop     rbx
        pop     r14
        jmp     qword ptr [rip + _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h36aabbbcc5a22eb9E@GOTPCREL]

core::ptr::real_drop_in_place:
        mov     rsi, qword ptr [rdi + 8]
        test    rsi, rsi
        je      .LBB2_1
        shl     rsi, 2
        mov     rdi, qword ptr [rdi]
        mov     edx, 4
        jmp     qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB2_1:
        ret

core::ptr::real_drop_in_place:
        ret

.LCPI4_0:
        .long   6
        .long   3
        .long   7
        .long   1
.LCPI4_1:
        .long   4
        .long   2
        .long   0
        .long   5
.LCPI4_2:
        .long   0
        .long   1
        .long   2
        .long   3
.LCPI4_3:
        .long   4
        .long   5
        .long   6
        .long   7
example::main:
        push    r15
        push    r14
        push    rbx
        sub     rsp, 144
        mov     edi, 40
        mov     esi, 4
        call    qword ptr [rip + __rust_alloc@GOTPCREL]
        test    rax, rax
        je      .LBB4_21
        mov     rbx, rax
        movaps  xmm0, xmmword ptr [rip + .LCPI4_0]
        movups  xmmword ptr [rax], xmm0
        movaps  xmm0, xmmword ptr [rip + .LCPI4_1]
        movups  xmmword ptr [rax + 16], xmm0
        movabs  rax, 34359738377
        mov     qword ptr [rbx + 32], rax
        mov     qword ptr [rsp], rbx
        mov     qword ptr [rsp + 8], 10
        mov     qword ptr [rsp + 16], 10
        xor     ecx, ecx
        jmp     .LBB4_2
.LBB4_8:
        cmp     rsi, 10
        jae     .LBB4_9
.LBB4_12:
        mov     eax, dword ptr [rbx + 4*rsi]
        mov     edx, dword ptr [rbx + 4*rcx]
        mov     dword ptr [rbx + 4*rsi], edx
        mov     dword ptr [rbx + 4*rcx], eax
        mov     rcx, r8
        cmp     r8, 10
        je      .LBB4_13
.LBB4_2:
        lea     r8, [rcx + 1]
        mov     rsi, rcx
        cmp     r8, 9
        ja      .LBB4_12
        mov     dl, 1
        mov     rdi, r8
        mov     rax, rcx
        test    dl, 1
        jne     .LBB4_5
        jmp     .LBB4_11
.LBB4_7:
        add     rdi, 1
        cmp     rsi, 10
        setb    dl
        mov     rax, rsi
        cmp     rdi, 10
        je      .LBB4_8
        test    dl, 1
        je      .LBB4_11
.LBB4_5:
        mov     edx, dword ptr [rbx + 4*rdi]
        mov     rsi, rdi
        cmp     edx, dword ptr [rbx + 4*rax]
        jl      .LBB4_7
        mov     rsi, rax
        jmp     .LBB4_7
.LBB4_13:
        mov     edi, 40
        mov     esi, 4
        call    qword ptr [rip + __rust_alloc@GOTPCREL]
        test    rax, rax
        je      .LBB4_21
        mov     r14, rax
        movaps  xmm0, xmmword ptr [rip + .LCPI4_2]
        movups  xmmword ptr [rax], xmm0
        movaps  xmm0, xmmword ptr [rip + .LCPI4_3]
        movups  xmmword ptr [rax + 16], xmm0
        movabs  rax, 38654705672
        mov     qword ptr [r14 + 32], rax
        mov     qword ptr [rsp + 40], r14
        mov     qword ptr [rsp + 48], 10
        mov     qword ptr [rsp + 56], 10
        cmp     rbx, r14
        je      .LBB4_17
        mov     edx, 40
        mov     rdi, r14
        mov     rsi, rbx
        call    qword ptr [rip + bcmp@GOTPCREL]
        test    eax, eax
        jne     .LBB4_16
.LBB4_17:
        mov     r15, qword ptr [rip + __rust_dealloc@GOTPCREL]
        mov     esi, 40
        mov     edx, 4
        mov     rdi, rbx
        call    r15
        mov     esi, 40
        mov     edx, 4
        mov     rdi, r14
        mov     rax, r15
        add     rsp, 144
        pop     rbx
        pop     r14
        pop     r15
        jmp     rax
.LBB4_11:
        lea     rdi, [rip + .L__unnamed_2]
        mov     edx, 10
        mov     rsi, rax
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
.LBB4_10:
        ud2
.LBB4_9:
        lea     rdi, [rip + .L__unnamed_3]
        mov     edx, 10
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        jmp     .LBB4_10
.LBB4_21:
        mov     edi, 40
        mov     esi, 4
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
        ud2
.LBB4_16:
        lea     rbx, [rsp + 40]
        mov     qword ptr [rsp + 24], rbx
        mov     rax, rsp
        mov     qword ptr [rsp + 32], rax
        lea     rax, [rsp + 24]
        mov     qword ptr [rsp + 64], rax
        lea     rax, [rip + <&T as core::fmt::Debug>::fmt]
        mov     qword ptr [rsp + 72], rax
        lea     rcx, [rsp + 32]
        mov     qword ptr [rsp + 80], rcx
        mov     qword ptr [rsp + 88], rax
        lea     rax, [rip + .L__unnamed_4]
        mov     qword ptr [rsp + 96], rax
        mov     qword ptr [rsp + 104], 3
        mov     qword ptr [rsp + 112], 0
        lea     rax, [rsp + 64]
        mov     qword ptr [rsp + 128], rax
        mov     qword ptr [rsp + 136], 2
        lea     rsi, [rip + .L__unnamed_5]
        lea     rdi, [rsp + 96]
        call    qword ptr [rip + std::panicking::begin_panic_fmt@GOTPCREL]
        jmp     .LBB4_10
        mov     r14, rax
        mov     rdi, rsp
        call    core::ptr::real_drop_in_place
        jmp     .LBB4_19
        mov     r14, rax
        mov     rbx, rsp
.LBB4_19:
        mov     rdi, rbx
        call    core::ptr::real_drop_in_place
        mov     rdi, r14
        call    _Unwind_Resume@PLT
        ud2

.L__unnamed_1:
        .quad   core::ptr::real_drop_in_place
        .quad   8
        .quad   8
        .quad   <&T as core::fmt::Debug>::fmt

.L__unnamed_6:
        .ascii  "/rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs"

.L__unnamed_3:
        .quad   .L__unnamed_6
        .asciz  "H\000\000\000\000\000\000\000\002\002\000\000#\000\000"

.L__unnamed_7:
        .ascii  "./example.rs"

.L__unnamed_2:
        .quad   .L__unnamed_7
        .asciz  "\f\000\000\000\000\000\000\000\005\000\000\000\032\000\000"

.L__unnamed_8:
        .ascii  "assertion failed: `(left == right)`\n  left: `"

.L__unnamed_9:
        .ascii  "`,\n right: `"

.L__unnamed_10:
        .byte   96

.L__unnamed_4:
        .quad   .L__unnamed_8
        .asciz  "-\000\000\000\000\000\000"
        .quad   .L__unnamed_9
        .asciz  "\f\000\000\000\000\000\000"
        .quad   .L__unnamed_10
        .asciz  "\001\000\000\000\000\000\000"

.L__unnamed_5:
        .quad   .L__unnamed_7
        .asciz  "\f\000\000\000\000\000\000\000\020\000\000\000\005\000\000"
*/
