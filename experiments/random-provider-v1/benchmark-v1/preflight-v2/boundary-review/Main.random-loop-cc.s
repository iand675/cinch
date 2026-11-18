"Main.random-loop#cc":                  # @"Main.random-loop#cc"
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
	subq	$56, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r9, %r12
	movq	%r8, %r14
                                        # kill: def $ecx killed $ecx def $rcx
	movq	16(%rbp), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movq	%rcx, -72(%rbp)                 # 8-byte Spill
	movl	%edx, -60(%rbp)                 # 4-byte Spill
	cmpl	%ecx, %edx
	jbe	.LBB100_107
# %bb.1:                                # %bb1.lr.ph
	movq	%rsi, %rbx
	movq	%rsi, %rax
	shrq	%rax
	movq	%rax, -80(%rbp)                 # 8-byte Spill
	movq	%rsi, -88(%rbp)                 # 8-byte Spill
	jmp	.LBB100_4
	.p2align	4
.LBB100_2:                              # %bb2.i774
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
.LBB100_3:                              # %cinch.rc.drop.obj.exit776
                                        #   in Loop: Header=BB100_4 Depth=1
	movzbl	%bl, %eax
	addq	%rax, %r14
	movq	%r12, -56(%rbp)                 # 8-byte Spill
	movq	%r12, %rdi
	callq	Main.fingerprint@PLT
	movq	%rax, %r12
	xorq	%r15, %r12
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r12
	movq	-72(%rbp), %rax                 # 8-byte Reload
	addl	$2, %eax
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	cmpl	%eax, -60(%rbp)                 # 4-byte Folded Reload
	movq	-88(%rbp), %rbx                 # 8-byte Reload
	jbe	.LBB100_107
.LBB100_4:                              # %bb1
                                        # =>This Inner Loop Header: Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %r13
	testq	%rax, %rax
	je	.LBB100_8
# %bb.5:                                # %bb0.i294
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_8
# %bb.6:                                # %bb1.i308
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	(%r13), %rax
	testb	$1, %bl
	je	.LBB100_9
.LBB100_7:                              # %bb0.i187
                                        #   in Loop: Header=BB100_4 Depth=1
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	-80(%rbp), %rax                 # 8-byte Folded Reload
	jmp	.LBB100_10
	.p2align	4
.LBB100_8:                              # %bb2.i302
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %bl
	jne	.LBB100_7
.LBB100_9:                              # %bb1.i192
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
.LBB100_10:                             # %cinch.runtime.natCmp.exit194
                                        #   in Loop: Header=BB100_4 Depth=1
	sete	-41(%rbp)                       # 1-byte Folded Spill
	testq	%r13, %r13
	je	.LBB100_13
# %bb.11:                               # %bb0.i276
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_15
# %bb.12:                               # %cinch.runtime.bytesLen.exit292.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	(%r13), %rax
	jmp	.LBB100_16
	.p2align	4
.LBB100_13:                             # %cinch.runtime.natCmp.exit179
                                        #   in Loop: Header=BB100_4 Depth=1
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	leaq	1(,%rax,2), %r15
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	jne	.LBB100_23
	jmp	.LBB100_46
	.p2align	4
.LBB100_15:                             # %cinch.runtime.bytesLen.exit292
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
.LBB100_16:                             # %bb0.i258
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %r15
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_19
# %bb.17:                               # %cinch.runtime.natCmp.exit179.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	movabsq	$9223372036854775807, %rax      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rax, (%r13)
	je	.LBB100_46
.LBB100_20:                             # %bb3.i364
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_23
# %bb.21:                               # %bb4.i377
                                        #   in Loop: Header=BB100_4 Depth=1
	cmpq	$0, (%r13)
	je	.LBB100_23
# %bb.22:                               # %bb7.i380
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	8(%r13), %rax
	movzbl	(%rax), %eax
	jmp	.LBB100_24
	.p2align	4
.LBB100_23:                             # %bb5.i372
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_get@PLT
.LBB100_24:                             # %cinch.runtime.bytesGet.exit387
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rsi
	jmp	.LBB100_25
	.p2align	4
.LBB100_19:                             # %cinch.runtime.natCmp.exit179.thread852
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	jne	.LBB100_20
.LBB100_46:                             #   in Loop: Header=BB100_4 Depth=1
	movl	$1, %esi
.LBB100_25:                             # %bb0.i79
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r15, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB100_27
# %bb.26:                               # %bb3.i84
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rbx
	testq	%r13, %r13
	jne	.LBB100_28
	jmp	.LBB100_30
	.p2align	4
.LBB100_27:                             # %bb1.i88
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r13, %r13
	je	.LBB100_30
.LBB100_28:                             # %bb0.i240
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_30
# %bb.29:                               # %bb0.i162.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	(%r13), %rax
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r15, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB100_37
	jmp	.LBB100_32
	.p2align	4
.LBB100_30:                             # %bb0.i162
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r15, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB100_37
# %bb.31:                               # %bb0.i329
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%r13, %r13
	je	.LBB100_35
.LBB100_32:                             # %bb3.i334
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_35
# %bb.33:                               # %bb4.i347
                                        #   in Loop: Header=BB100_4 Depth=1
	cmpq	(%r13), %rsi
	jae	.LBB100_35
# %bb.34:                               # %bb7.i350
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	8(%r13), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB100_36
	.p2align	4
.LBB100_35:                             # %bb5.i342
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_get@PLT
.LBB100_36:                             # %cinch.runtime.bytesGet.exit359
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rcx
.LBB100_37:                             # %bb13
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r14, -96(%rbp)                 # 8-byte Spill
	testb	$1, %bl
	je	.LBB100_40
# %bb.38:                               # %bb0.i64
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdx
	shrq	%rdx
	movq	%rcx, %rax
	shrq	%rax
	addq	%rdx, %rax
	js	.LBB100_40
# %bb.39:                               # %cinch.runtime.natAdd.exit75.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %r14
	jmp	.LBB100_48
	.p2align	4
.LBB100_40:                             # %cinch.runtime.natAdd.exit75
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	movq	%rcx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB100_48
# %bb.41:                               # %bb0.i534
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB100_43
# %bb.42:                               # %bb2.i544
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB100_48
.LBB100_43:                             # %bb3.i540
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rax, %rax
	je	.LBB100_48
# %bb.44:                               # %bb3.i540
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB100_47
# %bb.45:                               # %bb5.i541
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_48
.LBB100_47:                             # %bb4.i543
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB100_48:                             # %cinch.runtime.natCmp.exit158
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	-2(%r15), %rbx
	testq	%r13, %r13
	je	.LBB100_51
# %bb.49:                               # %bb0.i222
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_51
# %bb.50:                               # %cinch.runtime.bytesLen.exit238.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	shrq	%rbx
	cmpq	$2, %r15
	movl	$0, %eax
	cmovbq	%rax, %rbx
	movq	(%r13), %rax
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movl	$1, %esi
	cmpq	%rax, %rbx
	jb	.LBB100_53
	jmp	.LBB100_58
	.p2align	4
.LBB100_51:                             # %cinch.runtime.bytesLen.exit238
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
	shrq	%rbx
	cmpq	$2, %r15
	movl	$0, %ecx
	cmovbq	%rcx, %rbx
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movl	$1, %esi
	cmpq	%rax, %rbx
	jae	.LBB100_58
# %bb.52:                               # %bb20
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%r13, %r13
	je	.LBB100_56
.LBB100_53:                             # %bb3.i318
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_56
# %bb.54:                               # %bb4.i
                                        #   in Loop: Header=BB100_4 Depth=1
	cmpq	(%r13), %rbx
	jae	.LBB100_56
# %bb.55:                               # %bb7.i
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	8(%r13), %rax
	movzbl	(%rax,%rbx), %eax
	jmp	.LBB100_57
	.p2align	4
.LBB100_56:                             # %bb5.i
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	movq	%rbx, %rsi
	callq	cinch_bytes_get@PLT
.LBB100_57:                             # %cinch.runtime.bytesGet.exit
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rsi
.LBB100_58:                             # %bb19
                                        #   in Loop: Header=BB100_4 Depth=1
	testb	$1, %r14b
	je	.LBB100_61
# %bb.59:                               # %bb0.i
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB100_61
# %bb.60:                               # %cinch.runtime.natAdd.exit.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rbx
	jmp	.LBB100_68
	.p2align	4
.LBB100_61:                             # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r14, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB100_68
# %bb.62:                               # %bb0.i653
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB100_64
# %bb.63:                               # %bb2.i663
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB100_68
.LBB100_64:                             # %bb3.i659
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rax, %rax
	je	.LBB100_68
# %bb.65:                               # %bb3.i659
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB100_67
# %bb.66:                               # %bb5.i660
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_68
.LBB100_67:                             # %bb4.i662
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB100_68:                             # %cinch.rc.drop.tobj.exit665
                                        #   in Loop: Header=BB100_4 Depth=1
	testb	$1, %bl
	movq	-96(%rbp), %r14                 # 8-byte Reload
	jne	.LBB100_73
# %bb.69:                               # %cinch.runtime.natCmp.exit137
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB100_74
# %bb.70:                               # %bb24
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rbx, %rbx
	je	.LBB100_84
# %bb.71:                               # %bb0.i706
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %r15
	shlq	$29, %r15
	cmpq	$536870913, %r15                # imm = 0x20000001
	jl	.LBB100_77
# %bb.72:                               # %bb2.i716
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB100_84
	.p2align	4
.LBB100_73:                             # %cinch.runtime.natCmp.exit137.thread
                                        #   in Loop: Header=BB100_4 Depth=1
	shrq	%rbx
	movq	%rbx, %r15
	jmp	.LBB100_85
.LBB100_74:                             # %cinch.runtime.natToI64.exit
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r15
	testq	%rbx, %rbx
	je	.LBB100_85
# %bb.75:                               # %bb0.i689
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB100_80
# %bb.76:                               # %bb2.i699
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB100_85
.LBB100_77:                             # %bb3.i712
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%r15, %r15
	je	.LBB100_85
# %bb.78:                               # %bb3.i712
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %r15                # imm = 0x20000000
	jne	.LBB100_83
# %bb.79:                               # %bb5.i713
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_84
.LBB100_80:                             # %bb3.i695
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rax, %rax
	je	.LBB100_85
# %bb.81:                               # %bb3.i695
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB100_105
# %bb.82:                               # %bb5.i696
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_85
.LBB100_83:                             # %bb4.i715
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB100_84:                             # %bb22
                                        #   in Loop: Header=BB100_4 Depth=1
	xorl	%r15d, %r15d
.LBB100_85:                             # %bb22
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	testq	%rdx, %rdx
	movq	-88(%rbp), %rbx                 # 8-byte Reload
	je	.LBB100_92
# %bb.86:                               # %bb0.i721
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%rdx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB100_88
# %bb.87:                               # %bb2.i731
                                        #   in Loop: Header=BB100_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdx)
	jmp	.LBB100_92
	.p2align	4
.LBB100_88:                             # %bb3.i727
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rax, %rax
	je	.LBB100_92
# %bb.89:                               # %bb3.i727
                                        #   in Loop: Header=BB100_4 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB100_91
# %bb.90:                               # %bb5.i728
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_92
.LBB100_91:                             # %bb4.i730
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB100_92:                             # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB100_4 Depth=1
	movzbl	-41(%rbp), %eax                 # 1-byte Folded Reload
	addq	%rax, %r14
	xorq	%r12, %r15
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r15
	movq	-72(%rbp), %rax                 # 8-byte Reload
	incl	%eax
	cmpl	%eax, -60(%rbp)                 # 4-byte Folded Reload
	jbe	.LBB100_106
# %bb.93:                               # %bb25
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %r12
	testq	%rax, %rax
	je	.LBB100_97
# %bb.94:                               # %bb0.i210
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r12), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB100_97
# %bb.95:                               # %bb1.i219
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	(%r12), %rax
	testb	$1, %bl
	je	.LBB100_98
.LBB100_96:                             # %bb0.i121
                                        #   in Loop: Header=BB100_4 Depth=1
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	-80(%rbp), %rax                 # 8-byte Folded Reload
	jmp	.LBB100_99
	.p2align	4
.LBB100_97:                             # %bb2.i
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r12, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %bl
	jne	.LBB100_96
.LBB100_98:                             # %bb1.i126
                                        #   in Loop: Header=BB100_4 Depth=1
	leaq	1(,%rax,2), %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
.LBB100_99:                             # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB100_4 Depth=1
	sete	%bl
	testq	%r13, %r13
	je	.LBB100_3
# %bb.100:                              # %bb0.i764
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB100_2
# %bb.101:                              # %bb3.i770
                                        #   in Loop: Header=BB100_4 Depth=1
	testq	%rax, %rax
	je	.LBB100_3
# %bb.102:                              # %bb3.i770
                                        #   in Loop: Header=BB100_4 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB100_104
# %bb.103:                              # %bb5.i771
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB100_3
.LBB100_104:                            # %bb4.i773
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB100_3
.LBB100_105:                            # %bb4.i698
                                        #   in Loop: Header=BB100_4 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB100_85
.LBB100_106:
	movq	%r15, %r12
	movq	%r13, -56(%rbp)                 # 8-byte Spill
.LBB100_107:                            # %common.ret
	movq	%r14, %rax
	movq	%r12, %rdx
	movq	-56(%rbp), %rcx                 # 8-byte Reload
	addq	$56, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end100:
