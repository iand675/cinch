Main.finish:                            # @Main.finish
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
	subq	$40, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rcx, %rbx
	movq	%rdx, %r12
	movq	%rsi, %r13
	callq	cinch_clock_monotonic_ns@PLT
	testq	%rax, %rax
	js	.LBB113_2
# %bb.1:                                # %bb0.i281
	leaq	1(,%rax,2), %r14
	jmp	.LBB113_3
.LBB113_2:                              # %bb1.i278
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r14
.LBB113_3:                              # %cinch.runtime.natFromI64.exit284
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB113_5
# %bb.4:                                # %bb2
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB113_5:                              # %bb4
	testq	%rax, %rax
	je	.LBB113_7
# %bb.6:
	xorl	%eax, %eax
	jmp	.LBB113_282
.LBB113_7:                              # %bb5
	movq	(%rbx), %rax
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movq	8(%rbx), %rax
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movq	16(%rbx), %r15
	testq	%rbx, %rbx
	je	.LBB113_10
# %bb.8:                                # %cinch.rc.unique.exit
	movq	-8(%rbx), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB113_10
# %bb.9:                                # %bb6
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	jmp	.LBB113_22
.LBB113_10:                             # %bb7
	testq	%r15, %r15
	je	.LBB113_15
# %bb.11:                               # %bb0.i439
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB113_13
# %bb.12:                               # %bb2.i448
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB113_15
.LBB113_13:                             # %bb3.i445
	je	.LBB113_15
# %bb.14:                               # %bb4.i446
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB113_15:                             # %cinch.rc.dup.obj.exit
	testq	%rbx, %rbx
	je	.LBB113_22
# %bb.16:                               # %bb0.i451
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_18
# %bb.17:                               # %bb2.i461
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB113_22
.LBB113_18:                             # %bb3.i457
	testq	%rax, %rax
	je	.LBB113_22
# %bb.19:                               # %bb3.i457
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_21
# %bb.20:                               # %bb5.i458
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_22
.LBB113_21:                             # %bb4.i460
	callq	cinch_drop_mt@PLT
.LBB113_22:                             # %bb8
	movq	%r13, -48(%rbp)                 # 8-byte Spill
	movq	%r14, -56(%rbp)                 # 8-byte Spill
	movabsq	$9223372036854775807, %r13      # imm = 0x7FFFFFFFFFFFFFFF
	movabsq	$1030792151040, %rbx            # imm = 0xF000000000
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	testq	%r15, %r15
	je	.LBB113_25
# %bb.23:                               # %bb0.i340
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	cmpq	%rcx, %rax
	jne	.LBB113_27
# %bb.24:                               # %cinch.runtime.bytesLen.exit356.thread
	movq	(%r15), %rax
	jmp	.LBB113_28
.LBB113_25:                             # %cinch.runtime.natCmp.exit245
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	leaq	1(,%rax,2), %r14
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	testq	%r13, %rax
	jne	.LBB113_35
	jmp	.LBB113_59
.LBB113_27:                             # %cinch.runtime.bytesLen.exit356
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
.LBB113_28:                             # %bb0.i322
	leaq	1(,%rax,2), %r14
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	cmpq	%rcx, %rax
	jne	.LBB113_31
# %bb.29:                               # %cinch.runtime.natCmp.exit245.thread
	testq	%r13, (%r15)
	je	.LBB113_59
.LBB113_32:                             # %bb3.i410
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	cmpq	%rcx, %rax
	jne	.LBB113_35
# %bb.33:                               # %bb4.i423
	cmpq	$0, (%r15)
	je	.LBB113_35
# %bb.34:                               # %bb7.i426
	movq	8(%r15), %rax
	movzbl	(%rax), %eax
	jmp	.LBB113_36
.LBB113_35:                             # %bb5.i418
	movq	%r15, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_get@PLT
.LBB113_36:                             # %cinch.runtime.bytesGet.exit433
	leaq	1(,%rax,2), %rsi
	jmp	.LBB113_37
.LBB113_31:                             # %cinch.runtime.natCmp.exit245.thread1329
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	testq	%r13, %rax
	jne	.LBB113_32
.LBB113_59:
	movl	$1, %esi
.LBB113_37:                             # %bb0.i123
	movq	%r12, -80(%rbp)                 # 8-byte Spill
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB113_39
# %bb.38:                               # %bb3.i128
	leaq	1(,%rax,2), %r12
	jmp	.LBB113_40
.LBB113_39:                             # %bb1.i132
	movq	%r14, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
.LBB113_40:                             # %cinch.runtime.natDiv.exit
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	testq	%r15, %r15
	je	.LBB113_43
# %bb.41:                               # %bb0.i304
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	cmpq	%rcx, %rax
	jne	.LBB113_43
# %bb.42:                               # %bb0.i228.thread
	movq	(%r15), %rax
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jb	.LBB113_45
	jmp	.LBB113_50
.LBB113_43:                             # %bb0.i228
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB113_50
# %bb.44:                               # %bb0.i375
	testq	%r15, %r15
	je	.LBB113_48
.LBB113_45:                             # %bb3.i380
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB113_48
# %bb.46:                               # %bb4.i393
	cmpq	(%r15), %rsi
	jae	.LBB113_48
# %bb.47:                               # %bb7.i396
	movq	8(%r15), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB113_49
.LBB113_48:                             # %bb5.i388
	movq	%r15, %rdi
	callq	cinch_bytes_get@PLT
.LBB113_49:                             # %cinch.runtime.bytesGet.exit405
	leaq	1(,%rax,2), %rcx
.LBB113_50:                             # %bb12
	testb	$1, %r12b
	je	.LBB113_53
# %bb.51:                               # %bb0.i108
	movq	%r12, %rdx
	shrq	%rdx
	movq	%rcx, %rax
	shrq	%rax
	addq	%rdx, %rax
	js	.LBB113_53
# %bb.52:                               # %cinch.runtime.natAdd.exit119.thread
	leaq	1(,%rax,2), %rbx
	jmp	.LBB113_61
.LBB113_53:                             # %cinch.runtime.natAdd.exit119
	movq	%r12, %rdi
	movq	%rcx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_61
# %bb.54:                               # %bb0.i583
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_56
# %bb.55:                               # %bb2.i593
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB113_61
.LBB113_56:                             # %bb3.i589
	testq	%rax, %rax
	je	.LBB113_61
# %bb.57:                               # %bb3.i589
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_60
# %bb.58:                               # %bb5.i590
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_61
.LBB113_60:                             # %bb4.i592
	callq	cinch_drop_mt@PLT
.LBB113_61:                             # %cinch.runtime.natCmp.exit224
	leaq	-2(%r14), %rax
	cmpq	$2, %r14
	movl	$1, %r14d
	cmovaeq	%rax, %r14
	testq	%r15, %r15
	je	.LBB113_64
# %bb.62:                               # %bb0.i292
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB113_64
# %bb.63:                               # %cinch.runtime.natCmp.exit214.thread
	shrq	%r14
	andq	(%r15), %r13
	cmpq	%r13, %r14
	movq	-56(%rbp), %r12                 # 8-byte Reload
	movq	-48(%rbp), %r13                 # 8-byte Reload
	jb	.LBB113_66
	jmp	.LBB113_70
.LBB113_64:                             # %cinch.runtime.natCmp.exit214
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	shrq	%r14
	andq	%r13, %rax
	cmpq	%rax, %r14
	jae	.LBB113_69
# %bb.65:                               # %bb2.i361
	testq	%r15, %r15
	movq	-56(%rbp), %r12                 # 8-byte Reload
	movq	-48(%rbp), %r13                 # 8-byte Reload
	je	.LBB113_81
.LBB113_66:                             # %bb3.i364
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB113_72
# %bb.67:                               # %bb4.i
	cmpq	(%r15), %r14
	jae	.LBB113_72
# %bb.68:                               # %cinch.runtime.bytesGet.exit.thread
	movq	8(%r15), %rax
	movzbl	(%rax,%r14), %eax
	jmp	.LBB113_73
.LBB113_69:                             # %bb20
	testq	%r15, %r15
	movq	-56(%rbp), %r12                 # 8-byte Reload
	movq	-48(%rbp), %r13                 # 8-byte Reload
	je	.LBB113_83
.LBB113_70:                             # %bb0.i696
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_75
# %bb.71:                               # %bb2.i706
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB113_83
.LBB113_72:                             # %cinch.runtime.bytesGet.exit.thread1259
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	cinch_bytes_get@PLT
.LBB113_73:                             # %bb0.i665
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB113_78
# %bb.74:                               # %bb2.i675
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB113_89
.LBB113_75:                             # %bb3.i702
	movl	$1, %esi
	testq	%rax, %rax
	je	.LBB113_84
# %bb.76:                               # %bb3.i702
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_82
# %bb.77:                               # %bb5.i703
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_83
.LBB113_78:                             # %bb3.i671
	testq	%rcx, %rcx
	je	.LBB113_89
# %bb.79:                               # %bb3.i671
	movq	%rax, %r14
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB113_87
# %bb.80:                               # %bb5.i672
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_88
.LBB113_81:                             # %cinch.runtime.bytesGet.exit
	xorl	%edi, %edi
	movq	%r14, %rsi
	callq	cinch_bytes_get@PLT
	jmp	.LBB113_89
.LBB113_82:                             # %bb4.i705
	callq	cinch_drop_mt@PLT
.LBB113_83:                             # %bb18
	movl	$1, %esi
.LBB113_84:                             # %bb18
	testb	$1, %bl
	je	.LBB113_90
	jmp	.LBB113_85
.LBB113_87:                             # %bb4.i674
	callq	cinch_drop_mt@PLT
.LBB113_88:                             # %cinch.rc.drop.obj.exit677
	movq	%r14, %rax
.LBB113_89:                             # %cinch.rc.drop.obj.exit677
	leaq	1(,%rax,2), %rsi
	testb	$1, %bl
	je	.LBB113_90
.LBB113_85:                             # %bb0.i93
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB113_90
# %bb.86:                               # %cinch.runtime.natAdd.exit104.thread
	leaq	1(,%rax,2), %r14
	testb	$1, %r14b
	je	.LBB113_97
.LBB113_111:                            # %cinch.runtime.natCmp.exit203.thread
	shrq	%r14
	movq	%r14, %rbx
	jmp	.LBB113_114
.LBB113_90:                             # %cinch.runtime.natAdd.exit104
	movq	%rbx, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB113_91
.LBB113_96:                             # %cinch.rc.drop.tobj.exit742
	testb	$1, %r14b
	jne	.LBB113_111
.LBB113_97:                             # %cinch.runtime.natCmp.exit203
	movq	%r14, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB113_101
# %bb.98:                               # %bb23
	testq	%r14, %r14
	je	.LBB113_113
# %bb.99:                               # %bb0.i783
	movq	-8(%r14), %rax
	movq	%rax, %rbx
	shlq	$29, %rbx
	cmpq	$536870913, %rbx                # imm = 0x20000001
	jl	.LBB113_104
# %bb.100:                              # %bb2.i793
	decq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB113_113
.LBB113_91:                             # %bb0.i730
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_93
# %bb.92:                               # %bb2.i740
	decq	%rcx
	movq	%rcx, -8(%rbx)
	testb	$1, %r14b
	je	.LBB113_97
	jmp	.LBB113_111
.LBB113_101:                            # %cinch.runtime.natToI64.exit
	movq	%r14, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB113_114
# %bb.102:                              # %bb0.i766
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_107
# %bb.103:                              # %bb2.i776
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_114
.LBB113_93:                             # %bb3.i736
	testq	%rax, %rax
	je	.LBB113_96
# %bb.94:                               # %bb3.i736
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_110
# %bb.95:                               # %bb5.i737
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_96
.LBB113_104:                            # %bb3.i789
	testq	%rbx, %rbx
	je	.LBB113_114
# %bb.105:                              # %bb3.i789
	movq	%r14, %rdi
	cmpq	$536870912, %rbx                # imm = 0x20000000
	jne	.LBB113_112
# %bb.106:                              # %bb5.i790
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_113
.LBB113_107:                            # %bb3.i772
	testq	%rax, %rax
	je	.LBB113_114
# %bb.108:                              # %bb3.i772
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_283
# %bb.109:                              # %bb5.i773
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_114
.LBB113_110:                            # %bb4.i739
	callq	cinch_drop_mt@PLT
	testb	$1, %r14b
	je	.LBB113_97
	jmp	.LBB113_111
.LBB113_112:                            # %bb4.i792
	callq	cinch_drop_mt@PLT
.LBB113_113:                            # %bb21
	xorl	%ebx, %ebx
.LBB113_114:                            # %bb21
	movl	%r13d, %r15d
	andl	$1, %r15d
	movl	%r12d, %eax
	andl	%r13d, %eax
	testb	$1, %al
	je	.LBB113_117
# %bb.115:                              # %cinch.runtime.natCmp.exit193.thread
	movq	%r13, %rcx
	shrq	%rcx
	movq	%r12, %rax
	shrq	%rax
	subq	%rcx, %rax
	jb	.LBB113_118
# %bb.116:                              # %bb3.i142
	leaq	1(,%rax,2), %r14
	jmp	.LBB113_122
.LBB113_117:                            # %cinch.runtime.natCmp.exit193
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB113_121
.LBB113_118:                            # %bb26
	testq	%r13, %r13
	sete	%al
	orb	%al, %r15b
	jne	.LBB113_132
# %bb.119:                              # %bb0.i836
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_125
# %bb.120:                              # %bb2.i846
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB113_132
.LBB113_121:                            # %bb1.i147
	movq	%r12, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r14
.LBB113_122:                            # %cinch.runtime.natSub.exit
	testq	%r13, %r13
	sete	%al
	orb	%al, %r15b
	jne	.LBB113_140
# %bb.123:                              # %bb0.i802
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_128
# %bb.124:                              # %bb2.i812
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB113_140
.LBB113_125:                            # %bb3.i842
	testq	%rax, %rax
	je	.LBB113_132
# %bb.126:                              # %bb3.i842
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_131
# %bb.127:                              # %bb5.i843
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_132
.LBB113_128:                            # %bb3.i808
	testq	%rax, %rax
	je	.LBB113_140
# %bb.129:                              # %bb3.i808
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_139
# %bb.130:                              # %bb5.i809
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_140
.LBB113_131:                            # %bb4.i845
	callq	cinch_drop_mt@PLT
.LBB113_132:                            # %cinch.rc.drop.tobj.exit848
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	movl	$1, %r14d
	testb	$1, %cl
	jne	.LBB113_152
# %bb.133:                              # %bb0.i853
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_135
# %bb.134:                              # %bb2.i863
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB113_152
.LBB113_135:                            # %bb3.i859
	testq	%rax, %rax
	je	.LBB113_152
# %bb.136:                              # %bb3.i859
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_138
# %bb.137:                              # %bb5.i860
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_152
.LBB113_138:                            # %bb4.i862
	callq	cinch_drop_mt@PLT
	jmp	.LBB113_152
.LBB113_139:                            # %bb4.i811
	callq	cinch_drop_mt@PLT
.LBB113_140:                            # %cinch.rc.drop.tobj.exit814
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_147
# %bb.141:                              # %bb0.i819
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_143
# %bb.142:                              # %bb2.i829
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB113_147
.LBB113_143:                            # %bb3.i825
	testq	%rax, %rax
	je	.LBB113_147
# %bb.144:                              # %bb3.i825
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_146
# %bb.145:                              # %bb5.i826
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_147
.LBB113_146:                            # %bb4.i828
	callq	cinch_drop_mt@PLT
.LBB113_147:                            # %bb24
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_152
# %bb.148:                              # %bb0.i870
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB113_150
# %bb.149:                              # %bb2.i879
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB113_152
.LBB113_150:                            # %bb3.i876
	je	.LBB113_152
# %bb.151:                              # %bb4.i877
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB113_152:                            # %cinch.rc.dup.tobj.exit
	movq	%r14, %rdi
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r15
	testq	%rax, %rax
	sete	%al
	movl	%r15d, %r13d
	orb	%al, %r13b
	testb	$1, %r13b
	jne	.LBB113_157
# %bb.153:                              # %bb0.i885
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB113_155
# %bb.154:                              # %bb2.i895
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB113_157
.LBB113_155:                            # %bb3.i891
	je	.LBB113_157
# %bb.156:                              # %bb4.i893
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB113_157:                            # %cinch.rc.dup.tobj.exit897
	movq	%r15, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r12
	testb	$1, %r13b
	jne	.LBB113_164
# %bb.158:                              # %bb0.i.i
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_160
# %bb.159:                              # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB113_164
.LBB113_160:                            # %bb3.i.i
	testq	%rax, %rax
	je	.LBB113_164
# %bb.161:                              # %bb3.i.i
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_163
# %bb.162:                              # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_164
.LBB113_163:                            # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB113_164:                            # %cinch.runtime.natFromI64.exit268
	movabsq	$8589934590, %r13               # imm = 0x1FFFFFFFE
	movl	$1, %esi
	movq	%r12, %rdi
	movq	%r14, %rdx
	movq	%r15, %rcx
	callq	"Text.Show.nat-digits-into"@PLT
	movq	%r12, %rdi
	callq	"Text.Show.wrap-bytes"@PLT
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movq	-64(%rbp), %rdi                 # 8-byte Reload
	callq	Main.finish.shared0@PLT
	movq	%rax, -48(%rbp)                 # 8-byte Spill
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	callq	Main.finish.shared0@PLT
	movq	%rax, %r14
	movl	%ebx, %r15d
	addq	%r15, %r15
	shrq	$31, %rbx
	orq	$1, %rbx
	addq	$3, %r13
	movq	%rbx, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_mul@PLT
	movq	%rax, %rbx
	testb	$1, %bl
	je	.LBB113_167
# %bb.165:                              # %bb0.i78
	movq	%r15, %rcx
	shrq	%rcx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB113_167
# %bb.166:                              # %cinch.rc.drop.tobj.exit938.thread
	leaq	1(,%rax,2), %rdi
	jmp	.LBB113_175
.LBB113_167:                            # %cinch.rc.drop.tobj.exit938
	incq	%r15
	movq	%r15, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rdi
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_175
# %bb.168:                              # %bb0.i943
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_170
# %bb.169:                              # %bb2.i953
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB113_175
.LBB113_170:                            # %bb3.i949
	testq	%rax, %rax
	je	.LBB113_175
# %bb.171:                              # %bb3.i949
	movq	%rdi, %r15
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_173
# %bb.172:                              # %bb5.i950
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_174
.LBB113_173:                            # %bb4.i952
	callq	cinch_drop_mt@PLT
.LBB113_174:                            # %cinch.rc.drop.tobj.exit955
	movq	%r15, %rdi
.LBB113_175:                            # %cinch.rc.drop.tobj.exit955
	callq	"Text.Show.nat-to-string"@PLT
	movq	%rax, %r15
	leaq	Main.finish.dec3+8(%rip), %rsi
	movq	%rax, %rdi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r12
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	testq	%r15, %r15
	je	.LBB113_182
# %bb.176:                              # %bb0.i959
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_178
# %bb.177:                              # %bb2.i969
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB113_182
.LBB113_178:                            # %bb3.i965
	testq	%rax, %rax
	je	.LBB113_182
# %bb.179:                              # %bb3.i965
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_181
# %bb.180:                              # %bb5.i966
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_182
.LBB113_181:                            # %bb4.i968
	callq	cinch_drop_mt@PLT
.LBB113_182:                            # %bb0.i981
	leaq	Main.finish.dec2+8(%rip), %rdi
	movq	%rbx, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r12
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%r12, (%rax)
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_184
# %bb.183:                              # %bb2.i991
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB113_188
.LBB113_184:                            # %bb3.i987
	testq	%rax, %rax
	je	.LBB113_188
# %bb.185:                              # %bb3.i987
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_187
# %bb.186:                              # %bb5.i988
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_188
.LBB113_187:                            # %bb4.i990
	callq	cinch_drop_mt@PLT
.LBB113_188:                            # %cinch.rc.drop.obj.exit993
	movq	%r14, %rdi
	movq	%r15, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r12
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	testq	%r14, %r14
	movq	-80(%rbp), %r12                 # 8-byte Reload
	je	.LBB113_195
# %bb.189:                              # %bb0.i997
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_191
# %bb.190:                              # %bb2.i1007
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_195
.LBB113_191:                            # %bb3.i1003
	testq	%rax, %rax
	je	.LBB113_195
# %bb.192:                              # %bb3.i1003
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_194
# %bb.193:                              # %bb5.i1004
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_195
.LBB113_194:                            # %bb4.i1006
	callq	cinch_drop_mt@PLT
.LBB113_195:                            # %bb0.i1011
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_197
# %bb.196:                              # %bb2.i1021
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB113_201
.LBB113_197:                            # %bb3.i1017
	testq	%rax, %rax
	je	.LBB113_201
# %bb.198:                              # %bb3.i1017
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_200
# %bb.199:                              # %bb5.i1018
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_201
.LBB113_200:                            # %bb4.i1020
	callq	cinch_drop_mt@PLT
.LBB113_201:                            # %bb0.i1030
	leaq	Main.finish.dec1+8(%rip), %rdi
	movq	%rbx, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r15
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r15, (%rax)
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_203
# %bb.202:                              # %bb2.i1040
	decq	%rcx
	movq	%rcx, -8(%rbx)
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	jmp	.LBB113_208
.LBB113_203:                            # %bb3.i1036
	testq	%rax, %rax
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	je	.LBB113_208
# %bb.204:                              # %bb3.i1036
	movq	%rdi, %r15
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_206
# %bb.205:                              # %bb5.i1037
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_207
.LBB113_206:                            # %bb4.i1039
	callq	cinch_drop_mt@PLT
.LBB113_207:                            # %cinch.rc.drop.obj.exit1042
	movq	%r15, %rdi
.LBB113_208:                            # %cinch.rc.drop.obj.exit1042
	movq	%r14, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r15
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB113_215
# %bb.209:                              # %bb0.i1046
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_211
# %bb.210:                              # %bb2.i1056
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB113_215
.LBB113_211:                            # %bb3.i1052
	testq	%rax, %rax
	je	.LBB113_215
# %bb.212:                              # %bb3.i1052
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_214
# %bb.213:                              # %bb5.i1053
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_215
.LBB113_214:                            # %bb4.i1055
	callq	cinch_drop_mt@PLT
.LBB113_215:                            # %bb0.i1060
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_217
# %bb.216:                              # %bb2.i1070
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_221
.LBB113_217:                            # %bb3.i1066
	testq	%rax, %rax
	je	.LBB113_221
# %bb.218:                              # %bb3.i1066
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_220
# %bb.219:                              # %bb5.i1067
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_221
.LBB113_220:                            # %bb4.i1069
	callq	cinch_drop_mt@PLT
.LBB113_221:                            # %bb0.i1079
	leaq	Main.finish.dec0+8(%rip), %rdi
	movq	%rbx, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r15
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r15, (%rax)
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_223
# %bb.222:                              # %bb2.i1089
	decq	%rcx
	movq	%rcx, -8(%rbx)
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	jmp	.LBB113_228
.LBB113_223:                            # %bb3.i1085
	testq	%rax, %rax
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	je	.LBB113_228
# %bb.224:                              # %bb3.i1085
	movq	%rdi, %r15
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_226
# %bb.225:                              # %bb5.i1086
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_227
.LBB113_226:                            # %bb4.i1088
	callq	cinch_drop_mt@PLT
.LBB113_227:                            # %cinch.rc.drop.obj.exit1091
	movq	%r15, %rdi
.LBB113_228:                            # %cinch.rc.drop.obj.exit1091
	movq	%r14, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %r15
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB113_235
# %bb.229:                              # %bb0.i1095
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_231
# %bb.230:                              # %bb2.i1105
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB113_235
.LBB113_231:                            # %bb3.i1101
	testq	%rax, %rax
	je	.LBB113_235
# %bb.232:                              # %bb3.i1101
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_234
# %bb.233:                              # %bb5.i1102
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_235
.LBB113_234:                            # %bb4.i1104
	callq	cinch_drop_mt@PLT
.LBB113_235:                            # %bb0.i1109
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_237
# %bb.236:                              # %bb2.i1119
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_241
.LBB113_237:                            # %bb3.i1115
	testq	%rax, %rax
	je	.LBB113_241
# %bb.238:                              # %bb3.i1115
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_240
# %bb.239:                              # %bb5.i1116
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_241
.LBB113_240:                            # %bb4.i1118
	callq	cinch_drop_mt@PLT
.LBB113_241:                            # %bb0.i1123
	movq	%rbx, %rdi
	callq	"Data.String.string-to-bytes"@PLT
	movq	%rax, %r14
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_243
# %bb.242:                              # %bb2.i1133
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB113_247
.LBB113_243:                            # %bb3.i1129
	testq	%rax, %rax
	je	.LBB113_247
# %bb.244:                              # %bb3.i1129
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_246
# %bb.245:                              # %bb5.i1130
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_247
.LBB113_246:                            # %bb4.i1132
	callq	cinch_drop_mt@PLT
.LBB113_247:                            # %cinch.rc.drop.obj.exit1135
	movq	%r14, %rdi
	callq	cinch_io_write@PLT
	testq	%r14, %r14
	je	.LBB113_254
# %bb.248:                              # %bb0.i1138
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_250
# %bb.249:                              # %bb2.i1148
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_254
.LBB113_250:                            # %bb3.i1144
	testq	%rax, %rax
	je	.LBB113_254
# %bb.251:                              # %bb3.i1144
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_253
# %bb.252:                              # %bb5.i1145
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_254
.LBB113_253:                            # %bb4.i1147
	callq	cinch_drop_mt@PLT
.LBB113_254:                            # %cinch.runtime.natFromI64.exit
	movq	-64(%rbp), %rdi                 # 8-byte Reload
	leaq	(%rdi,%rdi), %rbx
	movabsq	$8589934590, %rax               # imm = 0x1FFFFFFFE
	andq	%rax, %rbx
	shrq	$31, %rdi
	orq	$1, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	je	.LBB113_257
# %bb.255:                              # %bb0.i
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%r14, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB113_257
# %bb.256:                              # %cinch.rc.drop.tobj.exit1189.thread
	leaq	1(,%rax,2), %rbx
	jmp	.LBB113_264
.LBB113_257:                            # %cinch.rc.drop.tobj.exit1189
	incq	%rbx
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_264
# %bb.258:                              # %bb0.i1194
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_260
# %bb.259:                              # %bb2.i1204
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB113_264
.LBB113_260:                            # %bb3.i1200
	testq	%rax, %rax
	je	.LBB113_264
# %bb.261:                              # %bb3.i1200
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_263
# %bb.262:                              # %bb5.i1201
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_264
.LBB113_263:                            # %bb4.i1203
	callq	cinch_drop_mt@PLT
.LBB113_264:                            # %cinch.rc.drop.tobj.exit1206
	movl	%r12d, %r15d
	andl	$1, %r15d
	testl	%ebx, %r15d
	je	.LBB113_266
# %bb.265:                              # %bb0.i176
	movq	%rbx, %rax
	xorq	%r12, %rax
	cmpq	$2, %rax
	setb	%r14b
	jmp	.LBB113_267
.LBB113_266:                            # %bb1.i181
	movq	%rbx, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	sete	%r14b
.LBB113_267:                            # %cinch.runtime.natCmp.exit
	testq	%r12, %r12
	sete	%al
	orb	%al, %r15b
	jne	.LBB113_274
# %bb.268:                              # %bb0.i1211
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_270
# %bb.269:                              # %bb2.i1221
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB113_274
.LBB113_270:                            # %bb3.i1217
	testq	%rax, %rax
	je	.LBB113_274
# %bb.271:                              # %bb3.i1217
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_273
# %bb.272:                              # %bb5.i1218
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_274
.LBB113_273:                            # %bb4.i1220
	callq	cinch_drop_mt@PLT
.LBB113_274:                            # %cinch.rc.drop.tobj.exit1223
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB113_281
# %bb.275:                              # %bb0.i1228
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB113_277
# %bb.276:                              # %bb2.i1238
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB113_281
.LBB113_277:                            # %bb3.i1234
	testq	%rax, %rax
	je	.LBB113_281
# %bb.278:                              # %bb3.i1234
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB113_280
# %bb.279:                              # %bb5.i1235
	callq	cinch_drop_slow@PLT
	jmp	.LBB113_281
.LBB113_280:                            # %bb4.i1237
	callq	cinch_drop_mt@PLT
.LBB113_281:                            # %cinch.rc.drop.tobj.exit1240
	xorb	$1, %r14b
	movzbl	%r14b, %eax
	leaq	1(,%rax,2), %rax
.LBB113_282:                            # %common.ret
	addq	$40, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB113_283:                            # %bb4.i775
	.cfi_def_cfa %rbp, 16
	callq	cinch_drop_mt@PLT
	jmp	.LBB113_114
.Lfunc_end113:
