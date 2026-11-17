"Main.clock-loop#cc":                   # @"Main.clock-loop#cc"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$24, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r9, -56(%rbp)                  # 8-byte Spill
	movq	%r8, %r12
	movq	%rcx, %r14
	movl	%esi, -44(%rbp)                 # 4-byte Spill
	cmpl	%edx, %esi
	jbe	.LBB106_67
# %bb.1:                                # %bb1.preheader
	movl	%edx, %r15d
	jmp	.LBB106_2
	.p2align	4
.LBB106_70:                             # %cinch.runtime.natCmp.exit.thread
                                        #   in Loop: Header=BB106_2 Depth=1
	shrq	%r13
	movq	%r13, %r12
.LBB106_64:                             # %bb13
                                        #   in Loop: Header=BB106_2 Depth=1
	xorq	%rbx, %r12
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r12
	addl	$3, %r15d
	addq	$3, %r14
	cmpl	%r15d, -44(%rbp)                # 4-byte Folded Reload
	jbe	.LBB106_67
.LBB106_2:                              # %bb1
                                        # =>This Inner Loop Header: Depth=1
	callq	cinch_clock_wall_ns@PLT
	testq	%rax, %rax
	js	.LBB106_4
# %bb.3:                                # %bb0.i62
                                        #   in Loop: Header=BB106_2 Depth=1
	leaq	1(,%rax,2), %rbx
	testb	$1, %bl
	jne	.LBB106_68
.LBB106_6:                              # %cinch.runtime.natCmp.exit45
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rbx, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB106_7
# %bb.14:                               # %bb5
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%rbx, %rbx
	je	.LBB106_21
# %bb.15:                               # %bb0.i110
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %r13
	shlq	$29, %r13
	cmpq	$536870913, %r13                # imm = 0x20000001
	jl	.LBB106_17
# %bb.16:                               # %bb2.i120
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB106_21
	.p2align	4
.LBB106_4:                              # %bb1.i59
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %rbx
	testb	$1, %bl
	je	.LBB106_6
.LBB106_68:                             # %cinch.runtime.natCmp.exit45.thread
                                        #   in Loop: Header=BB106_2 Depth=1
	shrq	%rbx
	movq	%rbx, %r13
	jmp	.LBB106_22
.LBB106_7:                              # %cinch.runtime.natToI64.exit87
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r13
	testq	%rbx, %rbx
	je	.LBB106_22
# %bb.8:                                # %bb0.i101
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB106_10
# %bb.9:                                # %bb2.i
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB106_22
.LBB106_17:                             # %bb3.i116
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%r13, %r13
	je	.LBB106_22
# %bb.18:                               # %bb3.i116
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %r13                # imm = 0x20000000
	jne	.LBB106_20
# %bb.19:                               # %bb5.i117
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_21
.LBB106_10:                             # %bb3.i
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%rax, %rax
	je	.LBB106_22
# %bb.11:                               # %bb3.i
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB106_13
# %bb.12:                               # %bb5.i
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_22
.LBB106_20:                             # %bb4.i119
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB106_21:                             # %bb3
                                        #   in Loop: Header=BB106_2 Depth=1
	xorl	%r13d, %r13d
.LBB106_22:                             # %bb3
                                        #   in Loop: Header=BB106_2 Depth=1
	xorq	%r12, %r13
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r13
	leal	1(%r15), %eax
	cmpl	%eax, -44(%rbp)                 # 4-byte Folded Reload
	jbe	.LBB106_65
# %bb.23:                               # %bb6
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_clock_wall_ns@PLT
	testq	%rax, %rax
	js	.LBB106_25
# %bb.24:                               # %bb0.i54
                                        #   in Loop: Header=BB106_2 Depth=1
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	jne	.LBB106_69
.LBB106_27:                             # %cinch.runtime.natCmp.exit36
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r12, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB106_28
# %bb.35:                               # %bb10
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%r12, %r12
	je	.LBB106_42
# %bb.36:                               # %bb0.i157
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rbx
	shlq	$29, %rbx
	cmpq	$536870913, %rbx                # imm = 0x20000001
	jl	.LBB106_38
# %bb.37:                               # %bb2.i167
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB106_42
	.p2align	4
.LBB106_25:                             # %bb1.i51
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	je	.LBB106_27
.LBB106_69:                             # %cinch.runtime.natCmp.exit36.thread
                                        #   in Loop: Header=BB106_2 Depth=1
	shrq	%r12
	movq	%r12, %rbx
	jmp	.LBB106_43
.LBB106_28:                             # %cinch.runtime.natToI64.exit79
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rbx
	testq	%r12, %r12
	je	.LBB106_43
# %bb.29:                               # %bb0.i140
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB106_31
# %bb.30:                               # %bb2.i150
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB106_43
.LBB106_38:                             # %bb3.i163
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%rbx, %rbx
	je	.LBB106_43
# %bb.39:                               # %bb3.i163
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rbx                # imm = 0x20000000
	jne	.LBB106_41
# %bb.40:                               # %bb5.i164
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_42
.LBB106_31:                             # %bb3.i146
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%rax, %rax
	je	.LBB106_43
# %bb.32:                               # %bb3.i146
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB106_34
# %bb.33:                               # %bb5.i147
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_43
.LBB106_41:                             # %bb4.i166
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB106_42:                             # %bb8
                                        #   in Loop: Header=BB106_2 Depth=1
	xorl	%ebx, %ebx
.LBB106_43:                             # %bb8
                                        #   in Loop: Header=BB106_2 Depth=1
	xorq	%r13, %rbx
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %rbx
	leal	2(%r15), %eax
	cmpl	%eax, -44(%rbp)                 # 4-byte Folded Reload
	jbe	.LBB106_66
# %bb.44:                               # %bb11
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_clock_wall_ns@PLT
	testq	%rax, %rax
	js	.LBB106_46
# %bb.45:                               # %bb0.i48
                                        #   in Loop: Header=BB106_2 Depth=1
	leaq	1(,%rax,2), %r13
	testb	$1, %r13b
	jne	.LBB106_70
	jmp	.LBB106_48
	.p2align	4
.LBB106_46:                             # %bb1.i46
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r13
	testb	$1, %r13b
	jne	.LBB106_70
.LBB106_48:                             # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r13, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB106_49
# %bb.56:                               # %bb15
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%r13, %r13
	je	.LBB106_63
# %bb.57:                               # %bb0.i205
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%r13), %rax
	movq	%rax, %r12
	shlq	$29, %r12
	cmpq	$536870913, %r12                # imm = 0x20000001
	jl	.LBB106_59
# %bb.58:                               # %bb2.i215
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rax
	movq	%rax, -8(%r13)
	jmp	.LBB106_63
.LBB106_49:                             # %cinch.runtime.natToI64.exit
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r13, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r12
	testq	%r13, %r13
	je	.LBB106_64
# %bb.50:                               # %bb0.i188
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB106_52
# %bb.51:                               # %bb2.i198
                                        #   in Loop: Header=BB106_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB106_64
.LBB106_59:                             # %bb3.i211
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%r12, %r12
	je	.LBB106_64
# %bb.60:                               # %bb3.i211
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %r12                # imm = 0x20000000
	jne	.LBB106_62
# %bb.61:                               # %bb5.i212
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_63
.LBB106_52:                             # %bb3.i194
                                        #   in Loop: Header=BB106_2 Depth=1
	testq	%rax, %rax
	je	.LBB106_64
# %bb.53:                               # %bb3.i194
                                        #   in Loop: Header=BB106_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB106_55
# %bb.54:                               # %bb5.i195
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB106_64
.LBB106_62:                             # %bb4.i214
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB106_63:                             # %bb13
                                        #   in Loop: Header=BB106_2 Depth=1
	xorl	%r12d, %r12d
	jmp	.LBB106_64
.LBB106_13:                             # %bb4.i
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB106_22
.LBB106_34:                             # %bb4.i149
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB106_43
.LBB106_55:                             # %bb4.i197
                                        #   in Loop: Header=BB106_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB106_64
.LBB106_65:                             # %common.ret.split.loop.exit258
	incq	%r14
	movq	%r13, %r12
	jmp	.LBB106_67
.LBB106_66:                             # %common.ret.split.loop.exit261
	addq	$2, %r14
	movq	%rbx, %r12
.LBB106_67:                             # %common.ret
	movq	%r14, %rax
	movq	%r12, %rdx
	movq	-56(%rbp), %rcx                 # 8-byte Reload
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end106:
