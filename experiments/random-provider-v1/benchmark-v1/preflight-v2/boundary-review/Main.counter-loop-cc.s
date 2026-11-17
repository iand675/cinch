"Main.counter-loop#cc":                 # @"Main.counter-loop#cc"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdx, %rax
	cmpl	%esi, %edi
	jbe	.LBB104_1
# %bb.2:                                # %cinch.runtime.natFromI64.exit68.preheader
	movabsq	$1099511628211, %r9             # imm = 0x100000001B3
	movl	%esi, %esi
	.p2align	4
.LBB104_3:                              # %cinch.runtime.natFromI64.exit68
                                        # =>This Inner Loop Header: Depth=1
	leaq	1(%rsi), %rdx
	xorq	%rsi, %rcx
	imulq	%r9, %rcx
	cmpl	%edx, %edi
	jbe	.LBB104_6
# %bb.4:                                # %cinch.runtime.natFromI64.exit60
                                        #   in Loop: Header=BB104_3 Depth=1
	leaq	2(%rsi), %r10
	movl	%edx, %edx
	xorq	%rcx, %rdx
	imulq	%r9, %rdx
	cmpl	%r10d, %edi
	jbe	.LBB104_7
# %bb.5:                                # %cinch.runtime.natFromI64.exit
                                        #   in Loop: Header=BB104_3 Depth=1
	addq	$3, %rsi
	movl	%esi, %r11d
	addq	$3, %rax
	movl	%r10d, %ecx
	xorq	%rcx, %rdx
	imulq	%r9, %rdx
	movq	%rdx, %rcx
	cmpl	%esi, %edi
	movq	%r11, %rsi
	ja	.LBB104_3
	jmp	.LBB104_8
.LBB104_1:
	movq	%rcx, %rdx
	jmp	.LBB104_8
.LBB104_6:                              # %common.ret.split.loop.exit253
	incq	%rax
	movq	%rcx, %rdx
	jmp	.LBB104_8
.LBB104_7:                              # %common.ret.split.loop.exit256
	addq	$2, %rax
.LBB104_8:                              # %common.ret
	movq	%r8, %rcx
	popq	%rbp
	retq
.Lfunc_end104:
