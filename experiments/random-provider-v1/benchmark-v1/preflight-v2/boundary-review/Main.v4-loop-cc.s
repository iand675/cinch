"Main.v4-loop#cc":                      # @"Main.v4-loop#cc"
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
	movq	%r8, %rbx
	cmpl	%edx, %esi
	jbe	.LBB108_194
# %bb.1:                                # %bb1.lr.ph.preheader
	movl	%esi, %r14d
	movq	%r9, -64(%rbp)                  # 8-byte Spill
	movl	%edx, %r13d
	movl	%esi, -44(%rbp)                 # 4-byte Spill
.LBB108_2:                              # %bb1.lr.ph
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB108_5 Depth 2
	movq	%rcx, -72(%rbp)                 # 8-byte Spill
	movl	%r13d, %r12d
	addq	$2, %r13
	movq	%rbx, %r15
	jmp	.LBB108_5
	.p2align	4
.LBB108_3:                              # %bb2.i165
                                        #   in Loop: Header=BB108_5 Depth=2
	decq	%rdx
	movq	%rdx, -8(%rax)
.LBB108_4:                              # %cinch.rc.drop.obj.exit167
                                        #   in Loop: Header=BB108_5 Depth=2
	movl	$4294967295, %eax               # imm = 0xFFFFFFFF
	xorq	%rax, %r15
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r15
	addq	$2, %r13
	addl	$2, %r12d
	cmpl	%r12d, %r14d
	jbe	.LBB108_197
.LBB108_5:                              # %bb1
                                        #   Parent Loop BB108_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %rbx
	movq	%rax, %rdi
	callq	"Main.v4-loop.shared0"@PLT
	testq	%rbx, %rbx
	je	.LBB108_13
# %bb.6:                                # %bb0.i111
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_8
# %bb.7:                                # %bb2.i118
                                        #   in Loop: Header=BB108_5 Depth=2
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB108_13
	.p2align	4
.LBB108_8:                              # %bb3.i
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	je	.LBB108_13
# %bb.9:                                # %bb3.i
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_11
# %bb.10:                               # %bb5.i
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_12
.LBB108_11:                             # %bb4.i
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_mt@PLT
.LBB108_12:                             # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_13:                             # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB108_15
# %bb.14:                               # %bb4
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB108_15:                             # %bb6
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	jne	.LBB108_41
# %bb.16:                               # %bb7
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rax, %rax
	je	.LBB108_23
# %bb.17:                               # %bb0.i121
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_19
# %bb.18:                               # %bb2.i131
                                        #   in Loop: Header=BB108_5 Depth=2
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB108_23
	.p2align	4
.LBB108_19:                             # %bb3.i127
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	je	.LBB108_23
# %bb.20:                               # %bb3.i127
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_22
# %bb.21:                               # %bb5.i128
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_23
.LBB108_22:                             # %bb4.i130
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB108_23:                             # %cinch.rc.drop.obj.exit133
                                        #   in Loop: Header=BB108_5 Depth=2
	movl	$4294967295, %eax               # imm = 0xFFFFFFFF
	xorq	%rax, %r15
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r15
	leal	-1(%r13), %eax
	cmpl	%eax, %r14d
	jbe	.LBB108_196
# %bb.24:                               # %bb9
                                        #   in Loop: Header=BB108_5 Depth=2
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %rbx
	movq	%rax, %rdi
	callq	"Main.v4-loop.shared0"@PLT
	testq	%rbx, %rbx
	je	.LBB108_32
# %bb.25:                               # %bb0.i141
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_27
# %bb.26:                               # %bb2.i151
                                        #   in Loop: Header=BB108_5 Depth=2
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB108_32
	.p2align	4
.LBB108_27:                             # %bb3.i147
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	je	.LBB108_32
# %bb.28:                               # %bb3.i147
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_30
# %bb.29:                               # %bb5.i148
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_31
.LBB108_30:                             # %bb4.i150
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_mt@PLT
.LBB108_31:                             # %cinch.rc.drop.obj.exit153
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_32:                             # %cinch.rc.drop.obj.exit153
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB108_34
# %bb.33:                               # %bb12
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB108_34:                             # %bb14
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	jne	.LBB108_45
# %bb.35:                               # %bb15
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rax, %rax
	je	.LBB108_4
# %bb.36:                               # %bb0.i155
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jge	.LBB108_3
# %bb.37:                               # %bb3.i161
                                        #   in Loop: Header=BB108_5 Depth=2
	testq	%rcx, %rcx
	je	.LBB108_4
# %bb.38:                               # %bb3.i161
                                        #   in Loop: Header=BB108_5 Depth=2
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_40
# %bb.39:                               # %bb5.i162
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_4
.LBB108_40:                             # %bb4.i164
                                        #   in Loop: Header=BB108_5 Depth=2
	callq	cinch_drop_mt@PLT
	jmp	.LBB108_4
	.p2align	4
.LBB108_41:                             # %bb6
                                        #   in Loop: Header=BB108_2 Depth=1
	cmpq	$1, %rcx
	jne	.LBB108_195
# %bb.42:                               # %bb8
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rax), %rbx
	testq	%rax, %rax
	je	.LBB108_49
# %bb.43:                               # %cinch.rc.unique.exit257
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	cmpq	$1, %rcx
	jne	.LBB108_49
# %bb.44:                               # %bb20
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB108_72
	.p2align	4
.LBB108_45:                             # %bb14
                                        #   in Loop: Header=BB108_2 Depth=1
	cmpq	$1, %rcx
	jne	.LBB108_195
# %bb.46:                               # %bb16
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rax), %rbx
	testq	%rax, %rax
	je	.LBB108_52
# %bb.47:                               # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	cmpq	$1, %rcx
	jne	.LBB108_52
# %bb.48:                               # %bb17
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_free@PLT
	movq	-64(%rbp), %r12                 # 8-byte Reload
	jmp	.LBB108_84
.LBB108_49:                             # %bb21
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rbx, %rbx
	je	.LBB108_57
# %bb.50:                               # %bb0.i259
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB108_55
# %bb.51:                               # %bb2.i269
                                        #   in Loop: Header=BB108_2 Depth=1
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB108_57
.LBB108_52:                             # %bb18
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rbx, %rbx
	movq	-64(%rbp), %r12                 # 8-byte Reload
	je	.LBB108_65
# %bb.53:                               # %bb0.i175
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB108_63
# %bb.54:                               # %bb2.i184
                                        #   in Loop: Header=BB108_2 Depth=1
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB108_65
.LBB108_55:                             # %bb3.i265
                                        #   in Loop: Header=BB108_2 Depth=1
	je	.LBB108_57
# %bb.56:                               # %bb4.i267
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_57:                             # %cinch.rc.dup.obj.exit271
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_72
# %bb.58:                               # %bb0.i273
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_60
# %bb.59:                               # %bb2.i283
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB108_72
.LBB108_60:                             # %bb3.i279
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_72
# %bb.61:                               # %bb3.i279
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_71
# %bb.62:                               # %bb5.i280
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_72
.LBB108_63:                             # %bb3.i181
                                        #   in Loop: Header=BB108_2 Depth=1
	je	.LBB108_65
# %bb.64:                               # %bb4.i182
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_65:                             # %cinch.rc.dup.obj.exit
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_84
# %bb.66:                               # %bb0.i187
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_68
# %bb.67:                               # %bb2.i197
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB108_84
.LBB108_68:                             # %bb3.i193
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_84
# %bb.69:                               # %bb3.i193
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_83
# %bb.70:                               # %bb5.i194
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_84
.LBB108_71:                             # %bb4.i282
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_72:                             # %bb22
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB108_74
# %bb.73:                               # %bb25
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB108_74:                             # %bb27
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	jne	.LBB108_195
# %bb.75:                               # %bb28
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rbx), %r12
	movq	8(%rbx), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	testq	%rbx, %rbx
	je	.LBB108_105
# %bb.76:                               # %cinch.rc.unique.exit294
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB108_78
# %bb.77:                               # %bb29
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	jmp	.LBB108_105
.LBB108_78:                             # %bb0.i296
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB108_80
# %bb.79:                               # %bb2.i306
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB108_105
.LBB108_80:                             # %bb3.i302
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_105
# %bb.81:                               # %bb3.i302
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB108_104
# %bb.82:                               # %bb5.i303
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_105
.LBB108_83:                             # %bb4.i196
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_84:                             # %bb19
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB108_86
# %bb.85:                               # %bb2.i209
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB108_86:                             # %bb4.i205
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_88
# %bb.87:                               #   in Loop: Header=BB108_2 Depth=1
	xorl	%esi, %esi
	testq	%rbx, %rbx
	jne	.LBB108_89
	jmp	.LBB108_96
.LBB108_88:                             # %bb5.i208
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rbx), %rax
	movq	%rax, -80(%rbp)                 # 8-byte Spill
	movq	8(%rbx), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r12
	movq	-80(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$17, %esi
	movq	%r12, %rdi
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	%r12, %rsi
	movq	-64(%rbp), %r12                 # 8-byte Reload
	testq	%rbx, %rbx
	je	.LBB108_96
.LBB108_89:                             # %bb0.i215
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB108_91
# %bb.90:                               # %bb2.i225
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB108_96
.LBB108_91:                             # %bb3.i221
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_96
# %bb.92:                               # %bb3.i221
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r12, -64(%rbp)                 # 8-byte Spill
	movq	%rsi, %r12
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB108_94
# %bb.93:                               # %bb5.i222
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_95
.LBB108_94:                             # %bb4.i224
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_95:                             # %cinch.rc.drop.obj.exit227
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r12, %rsi
	movq	-64(%rbp), %r12                 # 8-byte Reload
.LBB108_96:                             # %cinch.rc.drop.obj.exit227
                                        #   in Loop: Header=BB108_2 Depth=1
	movl	$4, %edi
	movq	%rsi, -56(%rbp)                 # 8-byte Spill
	callq	"Main.observe-uuid"@PLT
	movq	%rax, %rbx
	testq	%r12, %r12
	je	.LBB108_103
# %bb.97:                               # %bb0.i229
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB108_99
# %bb.98:                               # %bb2.i239
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB108_103
.LBB108_99:                             # %bb3.i235
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_103
# %bb.100:                              # %bb3.i235
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB108_102
# %bb.101:                              # %bb5.i236
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_103
.LBB108_102:                            # %bb4.i238
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_103:                            # %cinch.rc.drop.obj.exit241
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %r12
	shrq	$63, %r12
	addq	-72(%rbp), %r12                 # 8-byte Folded Reload
	andl	$1023, %ebx                     # imm = 0x3FF
	xorq	%r15, %rbx
	movq	-56(%rbp), %r9                  # 8-byte Reload
	jmp	.LBB108_192
.LBB108_104:                            # %bb4.i305
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_105:                            # %bb31
                                        #   in Loop: Header=BB108_2 Depth=1
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %rbx
	bswapq	%r12
	movl	$1, %esi
	movq	%rax, %rdi
	movq	%r12, %rdx
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$17, %esi
	movq	%rbx, %rdi
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	%rbx, %rdi
	testq	%rbx, %rbx
	movq	%rbx, -56(%rbp)                 # 8-byte Spill
	je	.LBB108_110
# %bb.106:                              # %bb0.i89
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	movq	-64(%rbp), %r12                 # 8-byte Reload
	jne	.LBB108_112
# %bb.107:                              # %cinch.runtime.natCmp.exit.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rdi), %rax
	jmp	.LBB108_108
.LBB108_110:                            # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	$16, %rax
	jne	.LBB108_116
# %bb.111:                              # %cinch.runtime.bytesData.exit379.thread590
                                        #   in Loop: Header=BB108_2 Depth=1
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	(%rax), %r12
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	8(%rax), %rbx
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	jmp	.LBB108_125
.LBB108_112:                            # %cinch.runtime.natCmp.exit.thread682
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_len@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB108_108:                            # %cinch.runtime.natCmp.exit.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	$16, %rax
	jne	.LBB108_109
# %bb.113:                              # %bb0.i331
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB108_117
# %bb.114:                              # %cinch.runtime.bytesData.exit.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	cmpq	$0, (%rdi)
	movl	$1, %eax
	je	.LBB108_118
# %bb.115:                              # %cinch.runtime.bytesData.exit.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	8(%rdi), %rax
	jmp	.LBB108_118
.LBB108_109:                            #   in Loop: Header=BB108_2 Depth=1
	xorl	%ebx, %ebx
	jmp	.LBB108_128
.LBB108_116:                            #   in Loop: Header=BB108_2 Depth=1
	xorl	%ebx, %ebx
	jmp	.LBB108_127
.LBB108_117:                            # %cinch.runtime.bytesData.exit
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_data@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB108_118:                            # %bb0.i72
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rax), %r12
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	je	.LBB108_120
# %bb.119:                              # %cinch.runtime.bytesLen.exit87
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_len@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB108_120:                            # %bb0.i363
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB108_123
# %bb.121:                              # %cinch.runtime.bytesData.exit379.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	cmpq	$0, (%rdi)
	movl	$1, %eax
	je	.LBB108_124
# %bb.122:                              # %cinch.runtime.bytesData.exit379.thread
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	8(%rdi), %rax
	jmp	.LBB108_124
.LBB108_123:                            # %cinch.runtime.bytesData.exit379
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_data@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB108_124:                            # %bb0.i65
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	8(%rax), %rbx
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	je	.LBB108_126
.LBB108_125:                            # %bb2.i
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_bytes_len@PLT
.LBB108_126:                            # %cinch.runtime.bytesLen.exit
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r12, %rax
	movabsq	$67553994410557440, %rcx        # imm = 0xF0000000000000
	andq	%rcx, %rax
	movabsq	$18014398509481984, %rcx        # imm = 0x40000000000000
	xorq	%rcx, %rax
	movl	%ebx, %ecx
	andl	$192, %ecx
	xorq	$128, %rcx
	movabsq	$9223372036854775807, %rdx      # imm = 0x7FFFFFFFFFFFFFFF
	incq	%rdx
	orq	%rax, %rcx
	movl	$0, %eax
	cmovneq	%rax, %rdx
	movzbl	%r12b, %eax
	shrq	$56, %r12
	addq	%rax, %r12
	movzbl	%bl, %eax
	shrq	$56, %rbx
	addq	%rax, %rbx
	addq	%r12, %rbx
	orq	%rdx, %rbx
.LBB108_127:                            # %bb32
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-64(%rbp), %r12                 # 8-byte Reload
.LBB108_128:                            # %bb32
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%r12, %r12
	je	.LBB108_135
# %bb.129:                              # %bb0.i407
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB108_131
# %bb.130:                              # %bb2.i417
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB108_135
.LBB108_131:                            # %bb3.i413
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_135
# %bb.132:                              # %bb3.i413
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB108_134
# %bb.133:                              # %bb5.i414
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_135
.LBB108_134:                            # %bb4.i416
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_135:                            # %cinch.rc.drop.obj.exit419
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rbx, %r12
	shrq	$63, %r12
	addq	-72(%rbp), %r12                 # 8-byte Folded Reload
	andl	$1023, %ebx                     # imm = 0x3FF
	xorq	%r15, %rbx
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %rbx
	leal	-1(%r13), %eax
	cmpl	%eax, %r14d
	jbe	.LBB108_199
# %bb.136:                              # %bb35
                                        #   in Loop: Header=BB108_2 Depth=1
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %r15
	movq	%rax, %rdi
	callq	"Data.Uuid.uuid-v4-from-random"@PLT
	testq	%r15, %r15
	je	.LBB108_144
# %bb.137:                              # %bb0.i427
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_139
# %bb.138:                              # %bb2.i437
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB108_144
.LBB108_139:                            # %bb3.i433
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_144
# %bb.140:                              # %bb3.i433
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %r14
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_142
# %bb.141:                              # %bb5.i434
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_143
.LBB108_142:                            # %bb4.i436
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_143:                            # %cinch.rc.drop.obj.exit439
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_144:                            # %cinch.rc.drop.obj.exit439
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB108_146
# %bb.145:                              # %bb38
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB108_146:                            # %bb40
                                        #   in Loop: Header=BB108_2 Depth=1
	cmpq	$1, %rcx
	movq	-56(%rbp), %r9                  # 8-byte Reload
	je	.LBB108_151
# %bb.147:                              # %bb40
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	jne	.LBB108_195
# %bb.148:                              # %bb41
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_170
# %bb.149:                              # %bb0.i441
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_157
# %bb.150:                              # %bb2.i451
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB108_170
.LBB108_151:                            # %bb42
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%rax), %r15
	testq	%rax, %rax
	je	.LBB108_154
# %bb.152:                              # %cinch.rc.unique.exit466
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	cmpq	$1, %rcx
	jne	.LBB108_154
# %bb.153:                              # %bb43
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB108_172
.LBB108_154:                            # %bb44
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%r15, %r15
	je	.LBB108_162
# %bb.155:                              # %bb0.i468
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB108_160
# %bb.156:                              # %bb2.i478
                                        #   in Loop: Header=BB108_2 Depth=1
	incq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB108_162
.LBB108_157:                            # %bb3.i447
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_170
# %bb.158:                              # %bb3.i447
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_168
# %bb.159:                              # %bb5.i448
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_169
.LBB108_160:                            # %bb3.i474
                                        #   in Loop: Header=BB108_2 Depth=1
	je	.LBB108_162
# %bb.161:                              # %bb4.i476
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r15, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_162:                            # %cinch.rc.dup.obj.exit480
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_172
# %bb.163:                              # %bb0.i482
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_165
# %bb.164:                              # %bb2.i492
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB108_172
.LBB108_165:                            # %bb3.i488
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_172
# %bb.166:                              # %bb3.i488
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_171
# %bb.167:                              # %bb5.i489
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_172
.LBB108_168:                            # %bb4.i450
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_169:                            # %cinch.rc.drop.obj.exit453
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-56(%rbp), %r9                  # 8-byte Reload
.LBB108_170:                            # %cinch.rc.drop.obj.exit453
                                        #   in Loop: Header=BB108_2 Depth=1
	movl	$4294967295, %eax               # imm = 0xFFFFFFFF
	xorq	%rax, %rbx
	jmp	.LBB108_192
.LBB108_171:                            # %bb4.i491
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_172:                            # %bb45
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB108_174
# %bb.173:                              # %bb2.i515
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB108_174:                            # %bb4.i501
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_176
# %bb.175:                              #   in Loop: Header=BB108_2 Depth=1
	movq	$0, -64(%rbp)                   # 8-byte Folded Spill
	testq	%r15, %r15
	jne	.LBB108_177
	jmp	.LBB108_183
.LBB108_176:                            # %bb5.i505
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	(%r15), %rax
	movq	%rax, -80(%rbp)                 # 8-byte Spill
	movq	8(%r15), %rax
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movq	-80(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	-72(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$17, %esi
	movq	-64(%rbp), %rdi                 # 8-byte Reload
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	testq	%r15, %r15
	je	.LBB108_183
.LBB108_177:                            # %bb0.i522
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB108_179
# %bb.178:                              # %bb2.i532
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB108_183
.LBB108_179:                            # %bb3.i528
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rax, %rax
	je	.LBB108_183
# %bb.180:                              # %bb3.i528
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB108_182
# %bb.181:                              # %bb5.i529
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_183
.LBB108_182:                            # %bb4.i531
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_183:                            # %cinch.rc.drop.obj.exit534
                                        #   in Loop: Header=BB108_2 Depth=1
	movl	$4, %edi
	movq	-64(%rbp), %r15                 # 8-byte Reload
	movq	%r15, %rsi
	callq	"Main.observe-uuid"@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB108_191
# %bb.184:                              # %bb0.i536
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	-8(%rdi), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB108_186
# %bb.185:                              # %bb2.i546
                                        #   in Loop: Header=BB108_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rdi)
	jmp	.LBB108_191
.LBB108_186:                            # %bb3.i542
                                        #   in Loop: Header=BB108_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB108_191
# %bb.187:                              # %bb3.i542
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %r14
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB108_189
# %bb.188:                              # %bb5.i543
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB108_190
.LBB108_189:                            # %bb4.i545
                                        #   in Loop: Header=BB108_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB108_190:                            # %cinch.rc.drop.obj.exit548
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%r14, %rax
	movl	-44(%rbp), %r14d                # 4-byte Reload
.LBB108_191:                            # %cinch.rc.drop.obj.exit548
                                        #   in Loop: Header=BB108_2 Depth=1
	movq	%rax, %rcx
	shrq	$63, %rcx
	addq	%rcx, %r12
	andl	$1023, %eax                     # imm = 0x3FF
	xorq	%rax, %rbx
	movq	%r15, %r9
.LBB108_192:                            # %bb0.outer.backedge
                                        #   in Loop: Header=BB108_2 Depth=1
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %rbx
	movq	%r12, %rcx
	movq	%r9, -64(%rbp)                  # 8-byte Spill
	cmpl	%r13d, %r14d
	ja	.LBB108_2
# %bb.193:                              # %bb2
	movq	%r12, %rax
	jmp	.LBB108_198
.LBB108_194:
	movq	%rcx, %rax
	jmp	.LBB108_198
.LBB108_195:
	xorl	%ebx, %ebx
	xorl	%r9d, %r9d
	xorl	%eax, %eax
	jmp	.LBB108_198
.LBB108_196:                            # %bb10
	movq	-72(%rbp), %rax                 # 8-byte Reload
	movq	%r15, %rbx
	movq	-64(%rbp), %r9                  # 8-byte Reload
	jmp	.LBB108_198
.LBB108_197:
	movq	-64(%rbp), %r9                  # 8-byte Reload
	movq	-72(%rbp), %rax                 # 8-byte Reload
	movq	%r15, %rbx
.LBB108_198:                            # %common.ret
	movq	%rbx, %rdx
	movq	%r9, %rcx
	addq	$40, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB108_199:                            # %bb36
	.cfi_def_cfa %rbp, 16
	movq	%r12, %rax
	movq	-56(%rbp), %r9                  # 8-byte Reload
	jmp	.LBB108_198
.Lfunc_end108:
