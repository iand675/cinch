"Main.v7-loop#cc":                      # @"Main.v7-loop#cc"
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
	subq	$120, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movl	%ecx, %eax
	movq	16(%rbp), %rcx
	cmpl	%edx, %eax
	jae	.LBB110_236
# %bb.1:                                # %bb1
	movq	-8(%rsi), %r10
	shrq	$40, %r10
	movzbl	%r10b, %r10d
	cmpq	$255, %r10
	jne	.LBB110_3
# %bb.2:                                # %bb5
	movq	-16(%rsi), %r10
	movq	(%r10), %r10
.LBB110_3:                              # %bb7
	testq	%r10, %r10
	jne	.LBB110_4
# %bb.5:                                # %bb8
	movq	(%rsi), %r13
	testq	%r13, %r13
	je	.LBB110_10
# %bb.6:                                # %bb0.i345
	movq	-8(%r13), %r10
	movq	%r10, %r11
	shlq	$29, %r11
	testq	%r11, %r11
	jle	.LBB110_8
# %bb.7:                                # %bb2.i353
	incq	%r10
	movq	%r10, -8(%r13)
	jmp	.LBB110_10
.LBB110_236:                            # %bb2
	testq	%rsi, %rsi
	je	.LBB110_244
# %bb.237:                              # %bb0.i733
	movq	-8(%rsi), %rdx
	movq	%rdx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_239
# %bb.238:                              # %bb2.i743
	decq	%rdx
	movq	%rdx, -8(%rsi)
	jmp	.LBB110_244
.LBB110_239:                            # %bb3.i739
	testq	%rax, %rax
	je	.LBB110_244
# %bb.240:                              # %bb3.i739
	movq	%rcx, %rbx
	movq	%r9, %r14
	movq	%r8, %r15
	movq	%rsi, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_242
# %bb.241:                              # %bb5.i740
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_243
.LBB110_242:                            # %bb4.i742
	callq	cinch_drop_mt@PLT
.LBB110_243:                            # %cinch.rc.drop.obj.exit745
	movq	%r15, %r8
	movq	%r14, %r9
	movq	%rbx, %rcx
	jmp	.LBB110_244
.LBB110_8:                              # %bb3.i351
	je	.LBB110_10
# %bb.9:                                # %bb4.i
	movq	%rdi, -80(%rbp)                 # 8-byte Spill
	movq	%r13, %rdi
	movq	%r8, -72(%rbp)                  # 8-byte Spill
	movq	%r13, -56(%rbp)                 # 8-byte Spill
	movq	%r9, %r13
	movq	%rcx, %r15
	movq	%rsi, %rbx
	movl	%eax, %r12d
	movl	%edx, %r14d
	callq	cinch_dup_mt@PLT
	movl	%r14d, %edx
	movl	%r12d, %eax
	movq	-80(%rbp), %rdi                 # 8-byte Reload
	movq	%rbx, %rsi
	movq	%r15, %rcx
	movq	%r13, %r9
	movq	-56(%rbp), %r13                 # 8-byte Reload
	movq	-72(%rbp), %r8                  # 8-byte Reload
.LBB110_10:                             # %cinch.rc.dup.obj.exit
	movq	-8(%r13), %r10
	shrq	$40, %r10
	movzbl	%r10b, %r10d
	cmpq	$255, %r10
	jne	.LBB110_12
# %bb.11:                               # %bb11
	movq	-16(%r13), %r10
	movq	(%r10), %r10
.LBB110_12:                             # %bb13
	testq	%r10, %r10
	jne	.LBB110_4
# %bb.13:                               # %bb14
	movq	(%r13), %rbx
	testq	%rbx, %rbx
	movq	%rsi, -144(%rbp)                # 8-byte Spill
	movq	%rdi, -80(%rbp)                 # 8-byte Spill
	movq	%rcx, -136(%rbp)                # 8-byte Spill
	movq	%r8, -72(%rbp)                  # 8-byte Spill
	movq	%r9, -152(%rbp)                 # 8-byte Spill
	movl	%eax, -104(%rbp)                # 4-byte Spill
	movl	%edx, -108(%rbp)                # 4-byte Spill
	je	.LBB110_245
# %bb.14:                               # %bb0.i356
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB110_16
# %bb.15:                               # %bb2.i366
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB110_18
.LBB110_245:                            # %cinch.rc.dup.obj.exit368
	leaq	-64(%rbp), %r8
	xorl	%edi, %edi
	xorl	%esi, %esi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	-64(%rbp), %r15
	jmp	.LBB110_24
.LBB110_16:                             # %bb3.i362
	je	.LBB110_18
# %bb.17:                               # %bb4.i364
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB110_18:                             # %bb0.i373
	leaq	-64(%rbp), %r8
	movq	%rbx, %rdi
	xorl	%esi, %esi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	-64(%rbp), %r15
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_20
# %bb.19:                               # %bb2.i382
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB110_24
.LBB110_20:                             # %bb3.i379
	testq	%rax, %rax
	je	.LBB110_24
# %bb.21:                               # %bb3.i379
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_23
# %bb.22:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_24
.LBB110_23:                             # %bb4.i381
	callq	cinch_drop_mt@PLT
.LBB110_24:                             # %cinch.rc.drop.obj.exit
	callq	cinch_clock_wall_ns@PLT
	testq	%rax, %rax
	js	.LBB110_26
# %bb.25:                               # %bb0.i224
	leaq	1(,%rax,2), %r14
	jmp	.LBB110_27
.LBB110_26:                             # %bb1.i221
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r14
.LBB110_27:                             # %cinch.runtime.natFromI64.exit
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_29
# %bb.28:                               # %bb17
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB110_29:                             # %bb19
	testq	%rax, %rax
	je	.LBB110_30
.LBB110_4:
	xorl	%r8d, %r8d
	xorl	%r9d, %r9d
	xorl	%ecx, %ecx
.LBB110_244:                            # %common.ret
	movq	%r8, %rax
	movq	%r9, %rdx
	addq	$120, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq	$8
.LBB110_30:                             # %bb20
	.cfi_def_cfa %rbp, 16
	movq	(%r15), %r12
	movq	8(%r15), %rbx
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	movb	%cl, -41(%rbp)                  # 1-byte Spill
	testb	$1, %cl
	jne	.LBB110_35
# %bb.31:                               # %bb0.i390
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB110_33
# %bb.32:                               # %bb2.i399
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB110_35
.LBB110_33:                             # %bb3.i396
	je	.LBB110_35
# %bb.34:                               # %bb4.i397
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
.LBB110_35:                             # %cinch.rc.dup.tobj.exit
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB110_40
# %bb.36:                               # %bb0.i405
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB110_38
# %bb.37:                               # %bb2.i415
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB110_40
.LBB110_38:                             # %bb3.i411
	je	.LBB110_40
# %bb.39:                               # %bb4.i413
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB110_40:                             # %cinch.rc.dup.tobj.exit417
	movq	%r15, -128(%rbp)                # 8-byte Spill
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %r15d
	orb	%al, %r15b
	testb	$1, %r15b
	movq	%r12, -56(%rbp)                 # 8-byte Spill
	movq	%rbx, -96(%rbp)                 # 8-byte Spill
	je	.LBB110_41
# %bb.47:                               # %cinch.rc.dup.tobj.exit434
	testb	$1, %r14b
	jne	.LBB110_49
# %bb.48:                               # %cinch.runtime.natDiv.exit146.thread762
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %r12
	movb	$1, %bl
	jmp	.LBB110_54
.LBB110_41:                             # %bb0.i422
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB110_43
# %bb.42:                               # %bb2.i432
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB110_45
.LBB110_49:                             # %cinch.runtime.natDiv.exit146.thread
	movabsq	$4835703278458516699, %rcx      # imm = 0x431BDE82D7B634DB
	movq	%r14, %rax
	mulq	%rcx
	movq	%rdx, %r12
	shrq	$18, %r12
	orq	$1, %r12
	xorl	%ebx, %ebx
	jmp	.LBB110_54
.LBB110_43:                             # %bb3.i428
	je	.LBB110_45
# %bb.44:                               # %bb4.i430
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB110_45:                             # %bb0.i439
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %r12
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_50
# %bb.46:                               # %bb2.i449
	decq	%rcx
	movq	%rcx, -8(%r14)
	movb	$1, %bl
	jmp	.LBB110_54
.LBB110_50:                             # %bb3.i445
	movb	$1, %bl
	testq	%rax, %rax
	je	.LBB110_54
# %bb.51:                               # %bb3.i445
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_53
# %bb.52:                               # %bb5.i446
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_54
.LBB110_53:                             # %bb4.i448
	callq	cinch_drop_mt@PLT
.LBB110_54:                             # %cinch.rc.drop.tobj.exit
	movabsq	$562949953421312, %rsi          # imm = 0x2000000000000
	testb	$1, %r12b
	jne	.LBB110_55
# %bb.58:                               # %cinch.runtime.natCmp.exit219
	incq	%rsi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB110_59
.LBB110_56:                             # %bb22
	movq	%r14, %rdi
	callq	cinch.rc.drop.tobj
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.tobj
	movq	-96(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.tobj
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	leaq	"Data.Uuid.UuidClockOutOfRange.blk#h"+8(%rip), %r14
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB110_154
.LBB110_55:                             # %bb0.i213
	cmpq	%rsi, %r12
	jae	.LBB110_56
.LBB110_59:                             # %bb21
	testb	$1, %r15b
	jne	.LBB110_64
# %bb.60:                               # %bb0.i458
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB110_62
# %bb.61:                               # %bb2.i468
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB110_64
.LBB110_62:                             # %bb3.i464
	je	.LBB110_64
# %bb.63:                               # %bb4.i466
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB110_64:                             # %cinch.rc.dup.tobj.exit470
	testb	%bl, %bl
	je	.LBB110_65
# %bb.66:                               # %bb1.i125
	movl	$2000000001, %esi               # imm = 0x77359401
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, -120(%rbp)                # 8-byte Spill
	jmp	.LBB110_67
.LBB110_65:                             # %bb0.i128
	movq	%r14, %rax
	shrq	$10, %rax
	movabsq	$19342813113834067, %rcx        # imm = 0x44B82FA09B5A53
	mulq	%rcx
	shrq	$10, %rdx
	orq	$1, %rdx
	movq	%rdx, -120(%rbp)                # 8-byte Spill
.LBB110_67:                             # %cinch.runtime.natDiv.exit133
	testb	$1, %r15b
	jne	.LBB110_74
# %bb.68:                               # %bb0.i475
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_70
# %bb.69:                               # %bb2.i485
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB110_74
.LBB110_70:                             # %bb3.i481
	testq	%rax, %rax
	je	.LBB110_74
# %bb.71:                               # %bb3.i481
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_73
# %bb.72:                               # %bb5.i482
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_74
.LBB110_73:                             # %bb4.i484
	callq	cinch_drop_mt@PLT
.LBB110_74:                             # %cinch.rc.drop.tobj.exit487
	movabsq	$4835703278458516699, %rdx      # imm = 0x431BDE82D7B634DB
	testb	%bl, %bl
	je	.LBB110_75
# %bb.76:                               # %bb1.i151
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r14, %rdi
	callq	cinch_nat_rem@PLT
	movq	%rax, %rbx
	jmp	.LBB110_77
.LBB110_75:                             # %bb0.i154
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rcx, %rax
	mulq	%rdx
	shrq	$18, %rdx
	imull	$1000000, %edx, %eax            # imm = 0xF4240
	subl	%eax, %ecx
	leaq	1(,%rcx,2), %rbx
.LBB110_77:                             # %cinch.runtime.natRem.exit
	testb	$1, %r15b
	jne	.LBB110_84
# %bb.78:                               # %bb0.i493
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_80
# %bb.79:                               # %bb2.i503
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB110_84
.LBB110_80:                             # %bb3.i499
	testq	%rax, %rax
	je	.LBB110_84
# %bb.81:                               # %bb3.i499
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_83
# %bb.82:                               # %bb5.i500
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_84
.LBB110_83:                             # %bb4.i502
	callq	cinch_drop_mt@PLT
.LBB110_84:                             # %cinch.rc.drop.tobj.exit505
	movl	$8193, %esi                     # imm = 0x2001
	movq	%rbx, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	movabsq	$4835703278458516699, %rdx      # imm = 0x431BDE82D7B634DB
	jne	.LBB110_92
# %bb.85:                               # %bb0.i511
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_87
# %bb.86:                               # %bb2.i521
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB110_92
.LBB110_87:                             # %bb3.i517
	testq	%rax, %rax
	je	.LBB110_92
# %bb.88:                               # %bb3.i517
	movq	%rdx, %r15
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_90
# %bb.89:                               # %bb5.i518
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_91
.LBB110_90:                             # %bb4.i520
	callq	cinch_drop_mt@PLT
.LBB110_91:                             # %cinch.rc.drop.tobj.exit523
	movq	%r15, %rdx
.LBB110_92:                             # %cinch.rc.drop.tobj.exit523
	testb	$1, %r14b
	jne	.LBB110_93
# %bb.94:                               # %cinch.runtime.natDiv.exit
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB110_101
# %bb.95:                               # %bb0.i528
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_97
# %bb.96:                               # %bb2.i538
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB110_101
.LBB110_93:                             # %cinch.runtime.natDiv.exit.thread
	movq	%r14, %rax
	mulq	%rdx
	movq	%rdx, %rbx
	shrq	$18, %rbx
	orq	$1, %rbx
.LBB110_101:                            # %cinch.rc.drop.tobj.exit540
	movl	$8193, %esi                     # imm = 0x2001
	movq	%r12, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r14
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB110_108
# %bb.102:                              # %bb0.i546
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_104
# %bb.103:                              # %bb2.i556
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB110_108
.LBB110_104:                            # %bb3.i552
	testq	%rax, %rax
	je	.LBB110_108
# %bb.105:                              # %bb3.i552
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_107
# %bb.106:                              # %bb5.i553
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_108
.LBB110_97:                             # %bb3.i534
	testq	%rax, %rax
	je	.LBB110_101
# %bb.98:                               # %bb3.i534
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_100
# %bb.99:                               # %bb5.i535
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_101
.LBB110_107:                            # %bb4.i555
	callq	cinch_drop_mt@PLT
.LBB110_108:                            # %cinch.rc.drop.tobj.exit558
	movl	%ebx, %r15d
	andl	$1, %r15d
	testl	%r14d, %r15d
	je	.LBB110_111
# %bb.109:                              # %bb0.i100
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB110_111
# %bb.110:                              # %bb3.i104
	leaq	1(,%rax,2), %rax
	jmp	.LBB110_112
.LBB110_111:                            # %bb1.i108
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
.LBB110_112:                            # %cinch.runtime.natAdd.exit110
	movq	%rax, -88(%rbp)                 # 8-byte Spill
	movq	-56(%rbp), %r12                 # 8-byte Reload
                                        # kill: def $r12d killed $r12d killed $r12 def $r12
	andl	$1, %r12d
	testq	%rbx, %rbx
	sete	%al
	orb	%al, %r15b
	jne	.LBB110_119
# %bb.113:                              # %bb0.i563
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_115
# %bb.114:                              # %bb2.i573
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB110_119
.LBB110_115:                            # %bb3.i569
	testq	%rax, %rax
	je	.LBB110_119
# %bb.116:                              # %bb3.i569
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_118
# %bb.117:                              # %bb5.i570
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_119
.LBB110_118:                            # %bb4.i572
	callq	cinch_drop_mt@PLT
.LBB110_119:                            # %cinch.rc.drop.tobj.exit575
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB110_126
# %bb.120:                              # %bb0.i580
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_122
# %bb.121:                              # %bb2.i590
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB110_126
.LBB110_122:                            # %bb3.i586
	testq	%rax, %rax
	je	.LBB110_126
# %bb.123:                              # %bb3.i586
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_125
# %bb.124:                              # %bb5.i587
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_126
.LBB110_125:                            # %bb4.i589
	callq	cinch_drop_mt@PLT
.LBB110_126:                            # %cinch.rc.drop.tobj.exit592
	movq	-120(%rbp), %rax                # 8-byte Reload
	testl	%eax, %r12d
	movq	%rax, %r12
	je	.LBB110_128
# %bb.127:                              # %bb0.i202
	movq	-56(%rbp), %r15                 # 8-byte Reload
	movq	%r15, %rax
	shrq	%rax
	movq	%r12, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
	seta	%bl
	jmp	.LBB110_129
.LBB110_128:                            # %bb1.i207
	movq	-56(%rbp), %r15                 # 8-byte Reload
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	setg	%bl
.LBB110_129:                            # %cinch.runtime.natCmp.exit209
	movq	-96(%rbp), %r14                 # 8-byte Reload
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB110_136
# %bb.130:                              # %bb0.i597
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB110_132
# %bb.131:                              # %bb2.i607
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB110_136
.LBB110_132:                            # %bb3.i603
	testq	%rax, %rax
	je	.LBB110_136
# %bb.133:                              # %bb3.i603
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB110_135
# %bb.134:                              # %bb5.i604
	callq	cinch_drop_slow@PLT
	jmp	.LBB110_136
.LBB110_135:                            # %bb4.i606
	callq	cinch_drop_mt@PLT
.LBB110_136:                            # %cinch.rc.drop.tobj.exit609
	testb	%bl, %bl
	je	.LBB110_138
# %bb.137:
	movq	-88(%rbp), %r15                 # 8-byte Reload
	jmp	.LBB110_146
.LBB110_138:                            # %bb25
	movl	%r14d, %ebx
	andl	$1, %ebx
	movq	-88(%rbp), %r15                 # 8-byte Reload
	testl	%r15d, %ebx
	je	.LBB110_140
# %bb.139:                              # %bb0.i191
	movq	%r15, %rax
	shrq	%rax
	movq	%r14, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
	jbe	.LBB110_141
	jmp	.LBB110_146
.LBB110_140:                            # %cinch.runtime.natCmp.exit198
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jg	.LBB110_146
.LBB110_141:                            # %bb26
	testq	%rbx, %rbx
	je	.LBB110_144
# %bb.142:                              # %bb0.i
	movq	%r14, %rax
	shrq	%rax
	incq	%rax
	js	.LBB110_144
# %bb.143:                              # %bb3.i
	leaq	1(,%rax,2), %rbx
	jmp	.LBB110_145
.LBB110_144:                            # %bb1.i
	movl	$3, %esi
	movq	%r14, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
.LBB110_145:                            # %cinch.runtime.natAdd.exit
	movq	%r15, %rdi
	callq	cinch.rc.drop.tobj
	movq	%rbx, %r15
.LBB110_146:                            # %bb23
	movabsq	$2305843009213693951, %rbx      # imm = 0x1FFFFFFFFFFFFFFF
	movq	%r14, %rdi
	callq	cinch.rc.drop.tobj
	testb	$1, %r15b
	jne	.LBB110_147
# %bb.149:                              # %cinch.runtime.natCmp.exit187
	movq	%r15, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB110_150
.LBB110_148:                            # %bb29
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	movq	%r15, %rdi
	callq	cinch.rc.drop.tobj
	leaq	"Data.Uuid.UuidClockExhausted.blk#h"+8(%rip), %r14
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB110_154
.LBB110_147:                            # %bb0.i181
	incq	%rbx
	cmpq	%rbx, %r15
	jae	.LBB110_148
.LBB110_150:                            # %bb28
	movq	%r15, %rdi
	callq	cinch.rc.dup.tobj
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	movq	%r15, 8(%rax)
	movq	%r15, %rdi
	testb	$1, %r15b
	jne	.LBB110_151
# %bb.152:                              # %bb1.i237
	callq	cinch_nat_to_i64@PLT
	movq	%r15, %rdi
	movq	%rax, %r15
	jmp	.LBB110_153
.LBB110_151:                            # %bb0.i234
	shrq	%r15
.LBB110_153:                            # %cinch.runtime.natToI64.exit
	callq	cinch.rc.drop.tobj
	movl	$16, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	movq	%r15, 8(%rax)
	movl	$257, %esi                      # imm = 0x101
.LBB110_154:                            # %bb15
	movl	$8, %edi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%r14, (%rax)
	movq	-8(%rax), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_156
# %bb.155:                              # %bb31
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB110_156:                            # %bb33
	cmpq	$1, %rax
	je	.LBB110_170
# %bb.157:                              # %bb33
	testq	%rax, %rax
	jne	.LBB110_4
# %bb.158:                              # %cinch.rc.unique.exit
	movabsq	$34359738367, %r12              # imm = 0x7FFFFFFFF
	movq	-8(%r15), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	je	.LBB110_160
# %bb.159:                              # %bb37
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
	xorl	%r15d, %r15d
.LBB110_160:                            # %bb38
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_162
# %bb.161:                              # %bb41
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB110_162:                            # %bb43
	testq	%rax, %rax
	jne	.LBB110_4
# %bb.163:                              # %bb44
	movq	(%r13), %rbx
	testq	%r13, %r13
	je	.LBB110_166
# %bb.164:                              # %cinch.rc.unique.exit639
	movq	-8(%r13), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	jne	.LBB110_166
# %bb.165:                              # %bb45
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_167
.LBB110_170:                            # %cinch.rc.unique.exit650
	movabsq	$34359738367, %r12              # imm = 0x7FFFFFFFF
	movq	-8(%r15), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	jne	.LBB110_172
# %bb.171:                              # %bb53
	movq	%r15, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_173
.LBB110_172:                            # %bb54
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
.LBB110_173:                            # %bb55
	movq	-128(%rbp), %rdi                # 8-byte Reload
	callq	cinch.rc.drop.obj
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_175
# %bb.174:                              # %bb57
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB110_175:                            # %bb59
	testq	%rax, %rax
	jne	.LBB110_4
# %bb.176:                              # %cinch.rc.unique.exit659
	movq	(%r14), %rbx
	movq	8(%r14), %r15
	movq	-8(%r14), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	jne	.LBB110_178
# %bb.177:                              # %bb61
	movq	%r14, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_179
.LBB110_166:                            # %bb46
	movq	%rbx, %rdi
	callq	cinch.rc.dup.obj
	movq	%r13, %rdi
	callq	cinch.rc.drop.obj
.LBB110_167:                            # %bb47
	leaq	-64(%rbp), %r8
	movl	$1, %edx
	movq	%rbx, %rdi
	movq	-128(%rbp), %rsi                # 8-byte Reload
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	testq	%r15, %r15
	je	.LBB110_168
# %bb.169:                              # %bb49
	movabsq	$281509336449025, %rax          # imm = 0x1000800000001
	movq	%rax, -8(%r15)
	jmp	.LBB110_204
.LBB110_178:                            # %bb62
	movq	%rbx, %rdi
	callq	cinch.rc.dup.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
.LBB110_179:                            # %bb63
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_181
# %bb.180:                              # %bb66
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB110_181:                            # %bb68
	testq	%rax, %rax
	jne	.LBB110_4
# %bb.182:                              # %bb69
	movq	(%r13), %r14
	testq	%r13, %r13
	je	.LBB110_185
# %bb.183:                              # %cinch.rc.unique.exit668
	movq	-8(%r13), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	jne	.LBB110_185
# %bb.184:                              # %bb70
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_186
.LBB110_168:                            # %bb48
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%r14, (%rax)
	jmp	.LBB110_204
.LBB110_185:                            # %bb71
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%r13, %rdi
	callq	cinch.rc.drop.obj
.LBB110_186:                            # %bb72
	leaq	-64(%rbp), %r8
	movl	$1, %edx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movl	$17, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %rbx
	movq	%r15, %rax
	shrq	$60, %rax
	je	.LBB110_187
# %bb.203:                              # %bb74
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	leaq	"Data.Uuid.UuidClockOutOfRange.blk#h"+8(%rip), %rax
	movq	%rax, (%r15)
	jmp	.LBB110_204
.LBB110_187:                            # %bb73
	movabsq	$1030792151040, %r12            # imm = 0xF000000000
	movabsq	$274877906944, %r14             # imm = 0x4000000000
	testq	%rbx, %rbx
	je	.LBB110_190
# %bb.188:                              # %bb0.i324
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r14, %rax
	jne	.LBB110_190
# %bb.189:                              # %bb1.i338
	movq	(%rbx), %rax
	jmp	.LBB110_191
.LBB110_100:                            # %bb4.i537
	callq	cinch_drop_mt@PLT
	jmp	.LBB110_101
.LBB110_190:                            # %bb2.i332
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB110_191:                            # %cinch.runtime.natCmp.exit177
	leaq	1(,%rax,2), %rdi
	movabsq	$9223372036854775807, %r13      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rax, %r13
	callq	cinch.rc.drop.tobj
	cmpq	$8, %r13
	jne	.LBB110_197
# %bb.192:                              # %bb75
	movq	%r15, %r13
	shlq	$4, %r13
	andq	$-65536, %r13                   # imm = 0xFFFF0000
	andl	$4095, %r15d                    # imm = 0xFFF
	movq	%rbx, %rdi
	callq	cinch.runtime.bytesData@PLT
	addq	%r13, %r15
	addq	$28672, %r15                    # imm = 0x7000
	movq	(%rax), %r13
	testq	%rbx, %rbx
	je	.LBB110_195
# %bb.193:                              # %bb0.i306
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r14, %rax
	jne	.LBB110_195
# %bb.194:                              # %bb1.i320
	movq	(%rbx), %rax
	jmp	.LBB110_196
.LBB110_197:                            # %bb76
	testq	%rbx, %rbx
	je	.LBB110_200
# %bb.198:                              # %bb0.i288
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r14, %rax
	jne	.LBB110_200
# %bb.199:                              # %bb1.i302
	movq	(%rbx), %rax
	jmp	.LBB110_201
.LBB110_195:                            # %bb2.i314
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB110_196:                            # %cinch.runtime.bytesLen.exit322
	leaq	1(,%rax,2), %r14
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.tobj
	andq	$-193, %r13
	orq	$128, %r13
	bswapq	%r13
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movq	%r13, 8(%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	jmp	.LBB110_202
.LBB110_200:                            # %bb2.i296
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB110_201:                            # %cinch.runtime.bytesLen.exit304
	leaq	1(,%rax,2), %r14
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r14, (%rax)
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
.LBB110_202:                            # %bb3
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
.LBB110_204:                            # %bb3
	movl	-108(%rbp), %r13d               # 4-byte Reload
	movl	-104(%rbp), %r14d               # 4-byte Reload
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %ecx
	cmpq	$255, %rcx
	movq	-152(%rbp), %rax                # 8-byte Reload
	jne	.LBB110_206
# %bb.205:                              # %bb78
	movq	-16(%r15), %rcx
	movq	(%rcx), %rcx
.LBB110_206:                            # %bb80
	cmpq	$1, %rcx
	je	.LBB110_210
# %bb.207:                              # %bb80
	testq	%rcx, %rcx
	jne	.LBB110_4
# %bb.208:                              # %bb81
	incl	%r14d
	movl	$4294967295, %ecx               # imm = 0xFFFFFFFF
	xorq	%rcx, %rax
	movabsq	$1099511628211, %rbx            # imm = 0x100000001B3
	imulq	%rax, %rbx
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
	movq	-136(%rbp), %rax                # 8-byte Reload
	movq	%rax, 16(%rbp)
	movq	%r14, %rdi
	movq	%r13, %rsi
	movq	-80(%rbp), %rdx                 # 8-byte Reload
	movq	-144(%rbp), %rcx                # 8-byte Reload
	movq	-72(%rbp), %r8                  # 8-byte Reload
	movq	%rbx, %r9
	jmp	.LBB110_209
.LBB110_210:                            # %cinch.rc.unique.exit704
	movabsq	$34359738367, %rbx              # imm = 0x7FFFFFFFF
	movq	(%r15), %r12
	movq	-8(%r15), %rax
	andq	%rbx, %rax
	cmpq	$1, %rax
	jne	.LBB110_212
# %bb.211:                              # %bb83
	movq	%r15, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_213
.LBB110_212:                            # %bb84
	movq	%r12, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
.LBB110_213:                            # %bb85
	movq	-8(%r12), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB110_215
# %bb.214:                              # %bb88
	movq	-16(%r12), %rax
	movq	(%rax), %rax
.LBB110_215:                            # %bb90
	testq	%rax, %rax
	jne	.LBB110_4
# %bb.216:                              # %bb91
	movq	%rbx, %rcx
	movq	%r13, -104(%rbp)                # 8-byte Spill
	movq	(%r12), %rbx
	movq	8(%r12), %r13
	testq	%r12, %r12
	je	.LBB110_219
# %bb.217:                              # %cinch.rc.unique.exit713
	movq	-8(%r12), %rax
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB110_219
# %bb.218:                              # %bb92
	movq	%r12, %rdi
	callq	cinch_free@PLT
	jmp	.LBB110_220
.LBB110_219:                            # %bb93
	movq	%r12, %rdi
	callq	cinch.rc.drop.obj
.LBB110_220:                            # %bb94
	movabsq	$1030792151040, %r12            # imm = 0xF000000000
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	bswapq	%rbx
	movl	$1, %esi
	movq	%rax, %rdi
	movq	%rbx, %rdx
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	bswapq	%r13
	movl	$17, %esi
	movq	%r15, %rdi
	movq	%r13, %rdx
	movabsq	$274877906944, %r13             # imm = 0x4000000000
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	testq	%r15, %r15
	je	.LBB110_223
# %bb.221:                              # %bb0.i270
	movq	-8(%r15), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB110_223
# %bb.222:                              # %bb1.i284
	movq	(%r15), %rbx
	jmp	.LBB110_224
.LBB110_223:                            # %bb2.i278
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %rbx
.LBB110_224:                            # %cinch.runtime.natCmp.exit
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	leaq	1(,%rbx,2), %rdi
	andq	%r12, %rbx
	callq	cinch.rc.drop.tobj
	cmpq	$16, %rbx
	jne	.LBB110_225
# %bb.226:                              # %bb96
	movq	%r15, %rdi
	callq	cinch.runtime.bytesData@PLT
	movq	(%rax), %rbx
	testq	%r15, %r15
	je	.LBB110_229
# %bb.227:                              # %bb0.i252
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	cmpq	%r13, %rax
	jne	.LBB110_229
# %bb.228:                              # %bb1.i266
	movq	(%r15), %rax
	jmp	.LBB110_230
.LBB110_225:
	xorl	%r13d, %r13d
	jmp	.LBB110_235
.LBB110_229:                            # %bb2.i260
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
.LBB110_230:                            # %cinch.runtime.bytesLen.exit268
	leaq	1(,%rax,2), %rdi
	callq	cinch.rc.drop.tobj
	movq	%r15, %rdi
	callq	cinch.runtime.bytesData@PLT
	movq	8(%rax), %r13
	testq	%r15, %r15
	je	.LBB110_233
# %bb.231:                              # %bb0.i240
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB110_233
# %bb.232:                              # %bb1.i249
	movq	(%r15), %rax
	jmp	.LBB110_234
.LBB110_233:                            # %bb2.i
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
.LBB110_234:                            # %cinch.runtime.bytesLen.exit
	leaq	1(,%rax,2), %rdi
	callq	cinch.rc.drop.tobj
	movabsq	$67553994410557440, %rax        # imm = 0xF0000000000000
	andq	%rbx, %rax
	movabsq	$31525197391593472, %rcx        # imm = 0x70000000000000
	xorq	%rax, %rcx
	movl	%r13d, %eax
	andl	$192, %eax
	xorq	$128, %rax
	incq	%r12
	xorl	%edx, %edx
	orq	%rcx, %rax
	cmoveq	%r12, %rdx
	movzbl	%bl, %eax
	shrq	$56, %rbx
	addq	%rax, %rbx
	movzbl	%r13b, %eax
	shrq	$56, %r13
	addq	%rax, %r13
	addq	%rbx, %r13
	orq	%rdx, %r13
.LBB110_235:                            # %bb95
	incl	%r14d
	movq	%r13, %rbx
	shrq	$63, %rbx
	addq	-72(%rbp), %rbx                 # 8-byte Folded Reload
	andl	$1023, %r13d                    # imm = 0x3FF
	xorq	-152(%rbp), %r13                # 8-byte Folded Reload
	movabsq	$1099511628211, %r12            # imm = 0x100000001B3
	imulq	%r13, %r12
	movq	-136(%rbp), %rdi                # 8-byte Reload
	callq	cinch.rc.drop.obj
	movq	%r15, 16(%rbp)
	movq	%r14, %rdi
	movq	-104(%rbp), %rsi                # 8-byte Reload
	movq	-80(%rbp), %rdx                 # 8-byte Reload
	movq	-144(%rbp), %rcx                # 8-byte Reload
	movq	%rbx, %r8
	movq	%r12, %r9
.LBB110_209:                            # %bb81
	addq	$120, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Main.v7-loop.shared0#cc"@PLT   # TAILCALL
.Lfunc_end110:
