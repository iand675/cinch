"Main.allocation-loop#cc":              # @"Main.allocation-loop#cc"
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
	subq	$72, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r9, %r15
	movq	%r8, %r14
	movq	%rcx, %r13
	movl	%esi, -76(%rbp)                 # 4-byte Spill
	cmpl	%edx, %esi
	jbe	.LBB102_199
# %bb.1:                                # %bb1.lr.ph
	movq	%r15, -72(%rbp)                 # 8-byte Spill
	movl	%edx, %ebx
	testq	%rdi, %rdi
	sete	%al
	movl	%edi, %ecx
	andl	$1, %ecx
	movq	%rcx, -88(%rbp)                 # 8-byte Spill
	movl	%edi, %ecx
	orb	%al, %cl
	movb	%cl, -41(%rbp)                  # 1-byte Spill
	movq	%rdi, %rax
	shrq	%rax
	movq	%rax, -96(%rbp)                 # 8-byte Spill
	movq	%rdi, -56(%rbp)                 # 8-byte Spill
	jmp	.LBB102_4
	.p2align	4
.LBB102_2:                              # %bb2.i1263
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
.LBB102_3:                              # %cinch.rc.drop.obj.exit1265
                                        #   in Loop: Header=BB102_4 Depth=1
	movzbl	%bl, %eax
	movq	-104(%rbp), %r15                # 8-byte Reload
	addq	$2, %r15
	movl	%r15d, %ebx
	addq	%rax, %r13
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	callq	Main.fingerprint@PLT
	movq	%rax, %r14
	xorq	%r12, %r14
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r14
	cmpl	%r15d, -76(%rbp)                # 4-byte Folded Reload
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	jbe	.LBB102_203
.LBB102_4:                              # %bb1
                                        # =>This Inner Loop Header: Depth=1
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB102_9
# %bb.5:                                # %bb0.i612
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB102_7
# %bb.6:                                # %bb2.i621
                                        #   in Loop: Header=BB102_4 Depth=1
	incq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB102_9
.LBB102_7:                              # %bb3.i618
                                        #   in Loop: Header=BB102_4 Depth=1
	je	.LBB102_9
# %bb.8:                                # %bb4.i619
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_dup_mt@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	.p2align	4
.LBB102_9:                              # %cinch.rc.dup.tobj.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB102_16
# %bb.10:                               # %bb0.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	movq	-8(%rdx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_12
# %bb.11:                               # %bb2.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdx)
	jmp	.LBB102_16
	.p2align	4
.LBB102_12:                             # %bb3.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_16
# %bb.13:                               # %bb3.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_15
# %bb.14:                               # %bb5.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_16
.LBB102_15:                             # %bb4.i.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_16:                             # %cinch.runtime.natFromI64.exit313
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, -104(%rbp)                # 8-byte Spill
	movzbl	%bl, %r12d
	addl	%r12d, %r12d
	movl	$1, %edi
	movabsq	$8589934593, %rsi               # imm = 0x200000001
	callq	cinch_nat_mul@PLT
	movq	%rax, %rbx
	testb	$1, %bl
	je	.LBB102_19
# %bb.17:                               # %bb0.i135
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	%r12d, %ecx
	shrl	%ecx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB102_19
# %bb.18:                               # %cinch.rc.drop.tobj.exit662.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %r12
	testq	%r15, %r15
	jne	.LBB102_26
	jmp	.LBB102_29
	.p2align	4
.LBB102_19:                             # %cinch.rc.drop.tobj.exit662
                                        #   in Loop: Header=BB102_4 Depth=1
	incq	%r12
	movq	%r12, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB102_20
.LBB102_25:                             # %cinch.rc.drop.tobj.exit679
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r15, %r15
	je	.LBB102_29
.LBB102_26:                             # %bb0.i457
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_29
# %bb.27:                               # %bb1.i471
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%r15), %rax
	jmp	.LBB102_30
	.p2align	4
.LBB102_20:                             # %bb0.i667
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_22
# %bb.21:                               # %bb2.i677
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	testq	%r15, %r15
	jne	.LBB102_26
	jmp	.LBB102_29
.LBB102_22:                             # %bb3.i673
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_25
# %bb.23:                               # %bb3.i673
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_28
# %bb.24:                               # %bb5.i674
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_25
.LBB102_28:                             # %bb4.i676
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	testq	%r15, %r15
	jne	.LBB102_26
	.p2align	4
.LBB102_29:                             # %bb2.i465
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
.LBB102_30:                             # %cinch.runtime.natCmp.exit290
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	%r12d, %ebx
	andl	$1, %ebx
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	je	.LBB102_42
# %bb.31:                               # %bb5
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rbx, %rbx
	jne	.LBB102_34
# %bb.32:                               # %cinch.runtime.natCmp.exit280
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	$511, %esi                      # imm = 0x1FF
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB102_36
# %bb.33:                               #   in Loop: Header=BB102_4 Depth=1
	xorl	%ebx, %ebx
	jmp	.LBB102_42
	.p2align	4
.LBB102_34:                             # %bb0.i274
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$512, %r12                      # imm = 0x200
	jae	.LBB102_48
# %bb.35:                               # %bb3.i580
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r12, %rdx
	shrq	%rdx
	testq	%r15, %r15
	jne	.LBB102_37
	jmp	.LBB102_41
.LBB102_36:                             # %bb4.i602
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rdx
	testq	%r15, %r15
	je	.LBB102_41
.LBB102_37:                             # %bb6.i585
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_41
# %bb.38:                               # %bb7.i592
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$255, %rdx
	ja	.LBB102_41
# %bb.39:                               # %bb7.i592
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$0, (%r15)
	je	.LBB102_41
# %bb.40:                               # %bb10.i597
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	8(%r15), %rax
	movb	%dl, (%rax)
	jmp	.LBB102_42
	.p2align	4
.LBB102_41:                             # %bb8.i590
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_set@PLT
.LBB102_42:                             # %bb8
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r12, %r12
	sete	%al
	orb	%al, %bl
	je	.LBB102_43
.LBB102_48:                             # %bb3
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r15, %r15
	je	.LBB102_53
.LBB102_49:                             # %bb0.i439
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_53
# %bb.50:                               # %bb1.i453
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%r15), %rax
	cmpq	$0, -88(%rbp)                   # 8-byte Folded Reload
	je	.LBB102_54
.LBB102_51:                             # %bb0.i263
                                        #   in Loop: Header=BB102_4 Depth=1
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	-96(%rbp), %rax                 # 8-byte Folded Reload
	jmp	.LBB102_55
	.p2align	4
.LBB102_43:                             # %bb0.i722
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_45
# %bb.44:                               # %bb2.i715
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	testq	%r15, %r15
	jne	.LBB102_49
	jmp	.LBB102_53
	.p2align	4
.LBB102_45:                             # %bb3.i728
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_48
# %bb.46:                               # %bb3.i728
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_52
# %bb.47:                               # %bb5.i712
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_48
.LBB102_52:                             # %bb4.i731
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	testq	%r15, %r15
	jne	.LBB102_49
	.p2align	4
.LBB102_53:                             # %bb2.i447
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	cmpq	$0, -88(%rbp)                   # 8-byte Folded Reload
	jne	.LBB102_51
.LBB102_54:                             # %bb1.i268
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rdi
	movq	-56(%rbp), %rsi                 # 8-byte Reload
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
.LBB102_55:                             # %cinch.runtime.natCmp.exit270
                                        #   in Loop: Header=BB102_4 Depth=1
	sete	-57(%rbp)                       # 1-byte Folded Spill
	testq	%r15, %r15
	je	.LBB102_58
# %bb.56:                               # %bb0.i421
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_60
# %bb.57:                               # %cinch.runtime.bytesLen.exit437.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%r15), %rax
	jmp	.LBB102_61
	.p2align	4
.LBB102_58:                             # %cinch.runtime.natCmp.exit255
                                        #   in Loop: Header=BB102_4 Depth=1
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	leaq	1(,%rax,2), %r12
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	jne	.LBB102_68
	jmp	.LBB102_91
	.p2align	4
.LBB102_60:                             # %cinch.runtime.bytesLen.exit437
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
.LBB102_61:                             # %bb0.i403
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %r12
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_64
# %bb.62:                               # %cinch.runtime.natCmp.exit255.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	movabsq	$9223372036854775807, %rax      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rax, (%r15)
	je	.LBB102_91
.LBB102_65:                             # %bb3.i527
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_68
# %bb.66:                               # %bb4.i540
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$0, (%r15)
	je	.LBB102_68
# %bb.67:                               # %bb7.i543
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	8(%r15), %rax
	movzbl	(%rax), %eax
	jmp	.LBB102_69
	.p2align	4
.LBB102_68:                             # %bb5.i535
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_get@PLT
.LBB102_69:                             # %cinch.runtime.bytesGet.exit550
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rsi
	jmp	.LBB102_70
	.p2align	4
.LBB102_64:                             # %cinch.runtime.natCmp.exit255.thread1398
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	jne	.LBB102_65
.LBB102_91:                             #   in Loop: Header=BB102_4 Depth=1
	movl	$1, %esi
.LBB102_70:                             # %bb0.i120
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r12, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB102_72
# %bb.71:                               # %bb3.i125
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rbx
	testq	%r15, %r15
	jne	.LBB102_73
	jmp	.LBB102_75
	.p2align	4
.LBB102_72:                             # %bb1.i129
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r15, %r15
	je	.LBB102_75
.LBB102_73:                             # %bb0.i385
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_75
# %bb.74:                               # %bb0.i238.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%r15), %rax
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r12, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB102_82
	jmp	.LBB102_77
	.p2align	4
.LBB102_75:                             # %bb0.i238
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r12, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB102_82
# %bb.76:                               # %bb0.i492
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r15, %r15
	je	.LBB102_80
.LBB102_77:                             # %bb3.i497
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_80
# %bb.78:                               # %bb4.i510
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	(%r15), %rsi
	jae	.LBB102_80
# %bb.79:                               # %bb7.i513
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	8(%r15), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB102_81
	.p2align	4
.LBB102_80:                             # %bb5.i505
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_get@PLT
.LBB102_81:                             # %cinch.runtime.bytesGet.exit522
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rcx
.LBB102_82:                             # %bb19
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r13, -112(%rbp)                # 8-byte Spill
	testb	$1, %bl
	je	.LBB102_85
# %bb.83:                               # %bb0.i105
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdx
	shrq	%rdx
	movq	%rcx, %rax
	shrq	%rax
	addq	%rdx, %rax
	js	.LBB102_85
# %bb.84:                               # %cinch.runtime.natAdd.exit116.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %r13
	jmp	.LBB102_93
	.p2align	4
.LBB102_85:                             # %cinch.runtime.natAdd.exit116
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	movq	%rcx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r13
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB102_93
# %bb.86:                               # %bb0.i878
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_88
# %bb.87:                               # %bb2.i888
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB102_93
.LBB102_88:                             # %bb3.i884
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_93
# %bb.89:                               # %bb3.i884
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_92
# %bb.90:                               # %bb5.i885
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_93
.LBB102_92:                             # %bb4.i887
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_93:                             # %cinch.runtime.natCmp.exit234
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	-2(%r12), %rbx
	testq	%r15, %r15
	je	.LBB102_96
# %bb.94:                               # %bb0.i367
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_96
# %bb.95:                               # %cinch.runtime.bytesLen.exit383.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	shrq	%rbx
	cmpq	$2, %r12
	movl	$0, %eax
	cmovbq	%rax, %rbx
	movq	(%r15), %rax
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movl	$1, %esi
	cmpq	%rax, %rbx
	jb	.LBB102_98
	jmp	.LBB102_103
	.p2align	4
.LBB102_96:                             # %cinch.runtime.bytesLen.exit383
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	shrq	%rbx
	cmpq	$2, %r12
	movl	$0, %ecx
	cmovbq	%rcx, %rbx
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movl	$1, %esi
	cmpq	%rax, %rbx
	jae	.LBB102_103
# %bb.97:                               # %bb26
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r15, %r15
	je	.LBB102_101
.LBB102_98:                             # %bb3.i481
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_101
# %bb.99:                               # %bb4.i
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	(%r15), %rbx
	jae	.LBB102_101
# %bb.100:                              # %bb7.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	8(%r15), %rax
	movzbl	(%rax,%rbx), %eax
	jmp	.LBB102_102
	.p2align	4
.LBB102_101:                            # %bb5.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	movq	%rbx, %rsi
	callq	cinch_bytes_get@PLT
.LBB102_102:                            # %cinch.runtime.bytesGet.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rsi
.LBB102_103:                            # %bb25
                                        #   in Loop: Header=BB102_4 Depth=1
	testb	$1, %r13b
	je	.LBB102_106
# %bb.104:                              # %bb0.i90
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r13, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB102_106
# %bb.105:                              # %cinch.runtime.natAdd.exit101.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rbx
	jmp	.LBB102_113
	.p2align	4
.LBB102_106:                            # %cinch.runtime.natAdd.exit101
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r13, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB102_113
# %bb.107:                              # %bb0.i997
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_109
# %bb.108:                              # %bb2.i1007
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB102_113
.LBB102_109:                            # %bb3.i1003
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_113
# %bb.110:                              # %bb3.i1003
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_112
# %bb.111:                              # %bb5.i1004
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_113
.LBB102_112:                            # %bb4.i1006
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_113:                            # %cinch.rc.drop.tobj.exit1009
                                        #   in Loop: Header=BB102_4 Depth=1
	testb	$1, %bl
	movq	-112(%rbp), %r13                # 8-byte Reload
	jne	.LBB102_118
# %bb.114:                              # %cinch.runtime.natCmp.exit213
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB102_119
# %bb.115:                              # %bb30
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rbx, %rbx
	je	.LBB102_129
# %bb.116:                              # %bb0.i1050
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %r12
	shlq	$29, %r12
	cmpq	$536870913, %r12                # imm = 0x20000001
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	jl	.LBB102_122
# %bb.117:                              # %bb2.i1060
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rax
	movq	%rax, -8(%rbx)
	xorl	%r12d, %r12d
	jmp	.LBB102_131
	.p2align	4
.LBB102_118:                            # %cinch.runtime.natCmp.exit213.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	shrq	%rbx
	movq	%rbx, %r12
	jmp	.LBB102_130
.LBB102_119:                            # %cinch.runtime.natToI64.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r12
	testq	%rbx, %rbx
	je	.LBB102_130
# %bb.120:                              # %bb0.i1033
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	jl	.LBB102_125
# %bb.121:                              # %bb2.i1043
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB102_131
.LBB102_122:                            # %bb3.i1056
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r12, %r12
	je	.LBB102_131
# %bb.123:                              # %bb3.i1056
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %r12                # imm = 0x20000000
	jne	.LBB102_128
# %bb.124:                              # %bb5.i1057
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_129
.LBB102_125:                            # %bb3.i1039
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_131
# %bb.126:                              # %bb3.i1039
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_198
# %bb.127:                              # %bb5.i1040
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_130
.LBB102_128:                            # %bb4.i1059
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
.LBB102_129:                            # %bb28
                                        #   in Loop: Header=BB102_4 Depth=1
	xorl	%r12d, %r12d
.LBB102_130:                            # %bb28
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB102_131:                            # %bb28
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-72(%rbp), %rdx                 # 8-byte Reload
	testq	%rdx, %rdx
	movq	-104(%rbp), %rbx                # 8-byte Reload
	je	.LBB102_139
# %bb.132:                              # %bb0.i1065
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_134
# %bb.133:                              # %bb2.i1075
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdx)
	jmp	.LBB102_139
	.p2align	4
.LBB102_134:                            # %bb3.i1071
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_139
# %bb.135:                              # %bb3.i1071
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rdx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_137
# %bb.136:                              # %bb5.i1072
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_138
.LBB102_137:                            # %bb4.i1074
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
.LBB102_138:                            # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
.LBB102_139:                            # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	movzbl	-57(%rbp), %eax                 # 1-byte Folded Reload
	addq	%rax, %r13
	incq	%rbx
	xorq	%r14, %r12
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r12
	cmpl	%ebx, -76(%rbp)                 # 4-byte Folded Reload
	jbe	.LBB102_200
# %bb.140:                              # %bb31
                                        #   in Loop: Header=BB102_4 Depth=1
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB102_145
# %bb.141:                              # %bb0.i1085
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB102_143
# %bb.142:                              # %bb2.i1095
                                        #   in Loop: Header=BB102_4 Depth=1
	incq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB102_145
.LBB102_143:                            # %bb3.i1091
                                        #   in Loop: Header=BB102_4 Depth=1
	je	.LBB102_145
# %bb.144:                              # %bb4.i1093
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_dup_mt@PLT
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	.p2align	4
.LBB102_145:                            # %cinch.rc.dup.tobj.exit1097
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB102_152
# %bb.146:                              # %bb0.i.i1104
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	movq	-8(%rdx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_148
# %bb.147:                              # %bb2.i.i1113
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdx)
	jmp	.LBB102_152
	.p2align	4
.LBB102_148:                            # %bb3.i.i1110
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_152
# %bb.149:                              # %bb3.i.i1110
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_151
# %bb.150:                              # %bb5.i.i1111
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_152
.LBB102_151:                            # %bb4.i.i1112
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_152:                            # %cinch.runtime.natFromI64.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	movzbl	%bl, %r14d
	addl	%r14d, %r14d
	movl	$1, %edi
	movabsq	$8589934593, %rsi               # imm = 0x200000001
	callq	cinch_nat_mul@PLT
	movq	%rax, %rbx
	testb	$1, %bl
	je	.LBB102_155
# %bb.153:                              # %bb0.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	%r14d, %ecx
	shrl	%ecx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB102_155
# %bb.154:                              # %cinch.rc.drop.tobj.exit1155.thread
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %r14
	jmp	.LBB102_162
	.p2align	4
.LBB102_155:                            # %cinch.rc.drop.tobj.exit1155
                                        #   in Loop: Header=BB102_4 Depth=1
	incq	%r14
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB102_162
# %bb.156:                              # %bb0.i1160
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_158
# %bb.157:                              # %bb2.i1170
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB102_162
.LBB102_158:                            # %bb3.i1166
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_162
# %bb.159:                              # %bb3.i1166
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_161
# %bb.160:                              # %bb5.i1167
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_162
.LBB102_161:                            # %bb4.i1169
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_162:                            # %cinch.rc.drop.tobj.exit1172
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB102_165
# %bb.163:                              # %bb0.i349
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_165
# %bb.164:                              # %bb1.i363
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%rdi), %rax
	jmp	.LBB102_166
	.p2align	4
.LBB102_165:                            # %bb2.i357
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_bytes_len@PLT
.LBB102_166:                            # %cinch.runtime.natCmp.exit203
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	%r14d, %ebx
	andl	$1, %ebx
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rcx, %rax
	je	.LBB102_179
# %bb.167:                              # %bb35
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rbx, %rbx
	jne	.LBB102_170
# %bb.168:                              # %cinch.runtime.natCmp.exit193
                                        #   in Loop: Header=BB102_4 Depth=1
	movl	$511, %esi                      # imm = 0x1FF
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB102_172
# %bb.169:                              #   in Loop: Header=BB102_4 Depth=1
	xorl	%ebx, %ebx
	jmp	.LBB102_179
	.p2align	4
.LBB102_170:                            # %bb0.i187
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$512, %r14                      # imm = 0x200
	jae	.LBB102_186
# %bb.171:                              # %bb3.i555
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r14, %rdx
	shrq	%rdx
	jmp	.LBB102_173
.LBB102_172:                            # %bb4.i573
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r14, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rdx
.LBB102_173:                            # %bb5.i557
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB102_178
# %bb.174:                              # %bb6.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_178
# %bb.175:                              # %bb7.i565
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$255, %rdx
	ja	.LBB102_178
# %bb.176:                              # %bb7.i565
                                        #   in Loop: Header=BB102_4 Depth=1
	cmpq	$0, (%rdi)
	je	.LBB102_178
# %bb.177:                              # %bb10.i
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	8(%rdi), %rax
	movb	%dl, (%rax)
	jmp	.LBB102_179
	.p2align	4
.LBB102_178:                            # %bb8.i
                                        #   in Loop: Header=BB102_4 Depth=1
	xorl	%esi, %esi
	callq	cinch_bytes_set@PLT
.LBB102_179:                            # %bb38
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%r14, %r14
	sete	%al
	orb	%al, %bl
	jne	.LBB102_186
# %bb.180:                              # %bb0.i1215
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_182
# %bb.181:                              # %bb2.i1208
                                        #   in Loop: Header=BB102_4 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB102_186
	.p2align	4
.LBB102_182:                            # %bb3.i1221
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_186
# %bb.183:                              # %bb3.i1221
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_185
# %bb.184:                              # %bb5.i1205
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_186
.LBB102_185:                            # %bb4.i1224
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB102_186:                            # %bb33
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB102_190
# %bb.187:                              # %bb0.i337
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB102_190
# %bb.188:                              # %bb1.i346
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	(%rdi), %rax
	cmpq	$0, -88(%rbp)                   # 8-byte Folded Reload
	je	.LBB102_191
.LBB102_189:                            # %bb0.i177
                                        #   in Loop: Header=BB102_4 Depth=1
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	-96(%rbp), %rax                 # 8-byte Folded Reload
	jmp	.LBB102_192
	.p2align	4
.LBB102_190:                            # %bb2.i
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_bytes_len@PLT
	cmpq	$0, -88(%rbp)                   # 8-byte Folded Reload
	jne	.LBB102_189
.LBB102_191:                            # %bb1.i182
                                        #   in Loop: Header=BB102_4 Depth=1
	leaq	1(,%rax,2), %rdi
	movq	-56(%rbp), %rsi                 # 8-byte Reload
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
.LBB102_192:                            # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB102_4 Depth=1
	sete	%bl
	testq	%r15, %r15
	je	.LBB102_3
# %bb.193:                              # %bb0.i1253
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB102_2
# %bb.194:                              # %bb3.i1259
                                        #   in Loop: Header=BB102_4 Depth=1
	testq	%rax, %rax
	je	.LBB102_3
# %bb.195:                              # %bb3.i1259
                                        #   in Loop: Header=BB102_4 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_197
# %bb.196:                              # %bb5.i1260
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_3
.LBB102_197:                            # %bb4.i1262
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB102_3
.LBB102_198:                            # %bb4.i1042
                                        #   in Loop: Header=BB102_4 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB102_130
.LBB102_199:                            # %entry.bb2_crit_edge
	movl	%edi, %ecx
	andl	$1, %ecx
	jmp	.LBB102_204
.LBB102_200:                            # %bb32
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	jne	.LBB102_217
# %bb.201:                              # %bb0.i1275
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_211
# %bb.202:                              # %bb2.i1285
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB102_217
.LBB102_203:
	movq	-72(%rbp), %r15                 # 8-byte Reload
	movq	-88(%rbp), %rcx                 # 8-byte Reload
.LBB102_204:                            # %bb2
	testq	%rdi, %rdi
	sete	%al
	orb	%al, %cl
	je	.LBB102_206
.LBB102_205:
	movq	%r14, %r12
	jmp	.LBB102_217
.LBB102_206:                            # %bb0.i1292
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB102_208
# %bb.207:                              # %bb2.i1302
	decq	%rcx
	movq	%rcx, -8(%rdi)
	movq	%r14, %r12
	jmp	.LBB102_217
.LBB102_208:                            # %bb3.i1298
	testq	%rax, %rax
	je	.LBB102_205
# %bb.209:                              # %bb3.i1298
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_215
# %bb.210:                              # %bb5.i1299
	callq	cinch_drop_slow@PLT
	movq	%r14, %r12
	jmp	.LBB102_217
.LBB102_211:                            # %bb3.i1281
	testq	%rax, %rax
	je	.LBB102_217
# %bb.212:                              # %bb3.i1281
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB102_216
# %bb.213:                              # %bb5.i1282
	callq	cinch_drop_slow@PLT
	jmp	.LBB102_217
.LBB102_215:                            # %bb4.i1301
	callq	cinch_drop_mt@PLT
	movq	%r14, %r12
	jmp	.LBB102_217
.LBB102_216:                            # %bb4.i1284
	callq	cinch_drop_mt@PLT
.LBB102_217:                            # %common.ret
	movq	%r13, %rax
	movq	%r12, %rdx
	movq	%r15, %rcx
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end102:
