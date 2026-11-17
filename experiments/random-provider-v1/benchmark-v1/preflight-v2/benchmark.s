	.file	"benchmark.ll"
	.text
	.globl	__apply                         # -- Begin function __apply
	.p2align	4
	.type	__apply,@function
__apply:                                # @__apply
	.cfi_startproc
# %bb.0:                                # %entry
	movq	(%rdi), %rax
	jmpq	*%rax                           # TAILCALL
.Lfunc_end0:
	.size	__apply, .Lfunc_end0-__apply
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natAdd            # -- Begin function cinch.runtime.natAdd
	.p2align	4
	.type	cinch.runtime.natAdd,@function
cinch.runtime.natAdd:                   # @cinch.runtime.natAdd
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	%edi, %eax
	andl	%esi, %eax
	testb	$1, %al
	je	.LBB1_3
# %bb.1:                                # %bb0
	movq	%rdi, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB1_3
# %bb.2:                                # %bb3
	leaq	1(,%rax,2), %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB1_3:                                # %bb1
	.cfi_def_cfa %rbp, 16
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_add@PLT               # TAILCALL
.Lfunc_end1:
	.size	cinch.runtime.natAdd, .Lfunc_end1-cinch.runtime.natAdd
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natSub            # -- Begin function cinch.runtime.natSub
	.p2align	4
	.type	cinch.runtime.natSub,@function
cinch.runtime.natSub:                   # @cinch.runtime.natSub
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	%edi, %eax
	andl	%esi, %eax
	testb	$1, %al
	je	.LBB2_3
# %bb.1:                                # %bb0
	movq	%rdi, %rax
	shrq	%rax
	movq	%rsi, %rcx
	shrq	%rcx
	subq	%rcx, %rax
	jae	.LBB2_2
.LBB2_3:                                # %bb1
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_sub@PLT               # TAILCALL
.LBB2_2:                                # %bb3
	.cfi_def_cfa %rbp, 16
	leaq	1(,%rax,2), %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end2:
	.size	cinch.runtime.natSub, .Lfunc_end2-cinch.runtime.natSub
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natMul            # -- Begin function cinch.runtime.natMul
	.p2align	4
	.type	cinch.runtime.natMul,@function
cinch.runtime.natMul:                   # @cinch.runtime.natMul
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_mul@PLT               # TAILCALL
.Lfunc_end3:
	.size	cinch.runtime.natMul, .Lfunc_end3-cinch.runtime.natMul
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natDiv            # -- Begin function cinch.runtime.natDiv
	.p2align	4
	.type	cinch.runtime.natDiv,@function
cinch.runtime.natDiv:                   # @cinch.runtime.natDiv
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	movq	%rdi, %rbx
	movl	%ebx, %r14d
	andl	$1, %r14d
	testl	%esi, %r14d
	sete	%al
	cmpq	$1, %rsi
	sete	%cl
	orb	%al, %cl
	jne	.LBB4_3
# %bb.1:                                # %bb0
	movq	%rbx, %rax
	shrq	%rax
	shrq	%rsi
	movq	%rax, %rcx
	orq	%rsi, %rcx
	movabsq	$9223372032559808512, %rdx      # imm = 0x7FFFFFFF00000000
	testq	%rdx, %rcx
	je	.LBB4_4
# %bb.2:
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB4_5
.LBB4_3:                                # %bb1
	movq	%rbx, %rdi
	callq	cinch_nat_div@PLT
	jmp	.LBB4_6
.LBB4_4:
                                        # kill: def $eax killed $eax killed $rax
	xorl	%edx, %edx
	divl	%esi
                                        # kill: def $eax killed $eax def $rax
.LBB4_5:                                # %bb2
	leaq	1(,%rax,2), %rax
.LBB4_6:                                # %bb2
	testq	%rbx, %rbx
	sete	%cl
	orb	%cl, %r14b
	jne	.LBB4_14
# %bb.7:                                # %bb0.i
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB4_9
# %bb.8:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB4_14
.LBB4_9:                                # %bb3.i
	testq	%rcx, %rcx
	je	.LBB4_14
# %bb.10:                               # %bb3.i
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB4_12
# %bb.11:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB4_13
.LBB4_12:                               # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB4_13:                               # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
.LBB4_14:                               # %cinch.rc.drop.tobj.exit
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end4:
	.size	cinch.runtime.natDiv, .Lfunc_end4-cinch.runtime.natDiv
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natRem            # -- Begin function cinch.runtime.natRem
	.p2align	4
	.type	cinch.runtime.natRem,@function
cinch.runtime.natRem:                   # @cinch.runtime.natRem
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	cmpq	$1, %rsi
	je	.LBB5_6
# %bb.1:                                # %entry
	movl	%edi, %eax
	andl	%esi, %eax
	andl	$1, %eax
	je	.LBB5_6
# %bb.2:                                # %bb0
	shrq	%rdi
	shrq	%rsi
	movq	%rdi, %rax
	orq	%rsi, %rax
	movabsq	$9223372032559808512, %rcx      # imm = 0x7FFFFFFF00000000
	testq	%rcx, %rax
	je	.LBB5_3
# %bb.4:
	movq	%rdi, %rax
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB5_5
.LBB5_6:                                # %bb1
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_rem@PLT               # TAILCALL
.LBB5_3:
	.cfi_def_cfa %rbp, 16
	movl	%edi, %eax
	xorl	%edx, %edx
	divl	%esi
                                        # kill: def $edx killed $edx def $rdx
.LBB5_5:                                # %bb2
	leaq	1(,%rdx,2), %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end5:
	.size	cinch.runtime.natRem, .Lfunc_end5-cinch.runtime.natRem
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natCmp            # -- Begin function cinch.runtime.natCmp
	.p2align	4
	.type	cinch.runtime.natCmp,@function
cinch.runtime.natCmp:                   # @cinch.runtime.natCmp
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %eax
	andl	%esi, %eax
	testb	$1, %al
	je	.LBB6_2
# %bb.1:                                # %bb0
	shrq	%rdi
	shrq	%rsi
	cmpq	%rsi, %rdi
	seta	%al
	sbbb	$0, %al
	movsbq	%al, %rax
	popq	%rbp
	retq
.LBB6_2:                                # %bb1
	popq	%rbp
	jmp	cinch_nat_cmp@PLT               # TAILCALL
.Lfunc_end6:
	.size	cinch.runtime.natCmp, .Lfunc_end6-cinch.runtime.natCmp
                                        # -- End function
	.globl	cinch.runtime.natFromI64        # -- Begin function cinch.runtime.natFromI64
	.p2align	4
	.type	cinch.runtime.natFromI64,@function
cinch.runtime.natFromI64:               # @cinch.runtime.natFromI64
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	testq	%rdi, %rdi
	js	.LBB7_2
# %bb.1:                                # %bb0
	leaq	1(,%rdi,2), %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB7_2:                                # %bb1
	.cfi_def_cfa %rbp, 16
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_from_i64@PLT          # TAILCALL
.Lfunc_end7:
	.size	cinch.runtime.natFromI64, .Lfunc_end7-cinch.runtime.natFromI64
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.natToI64          # -- Begin function cinch.runtime.natToI64
	.p2align	4
	.type	cinch.runtime.natToI64,@function
cinch.runtime.natToI64:                 # @cinch.runtime.natToI64
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	testb	$1, %dil
	jne	.LBB8_1
# %bb.2:                                # %bb1
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_to_i64@PLT            # TAILCALL
.LBB8_1:                                # %bb0
	.cfi_def_cfa %rbp, 16
	shrq	%rdi
	movq	%rdi, %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end8:
	.size	cinch.runtime.natToI64, .Lfunc_end8-cinch.runtime.natToI64
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.intToI64          # -- Begin function cinch.runtime.intToI64
	.p2align	4
	.type	cinch.runtime.intToI64,@function
cinch.runtime.intToI64:                 # @cinch.runtime.intToI64
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_int_to_i64@PLT            # TAILCALL
.Lfunc_end9:
	.size	cinch.runtime.intToI64, .Lfunc_end9-cinch.runtime.intToI64
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesNew          # -- Begin function cinch.runtime.bytesNew
	.p2align	4
	.type	cinch.runtime.bytesNew,@function
cinch.runtime.bytesNew:                 # @cinch.runtime.bytesNew
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	movq	%rdi, %rbx
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	testq	%rbx, %rbx
	sete	%cl
	movl	%ebx, %edx
	orb	%cl, %dl
	testb	$1, %dl
	jne	.LBB10_8
# %bb.1:                                # %bb0.i
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB10_3
# %bb.2:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB10_8
.LBB10_3:                               # %bb3.i
	testq	%rcx, %rcx
	je	.LBB10_8
# %bb.4:                                # %bb3.i
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB10_6
# %bb.5:                                # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB10_7
.LBB10_6:                               # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB10_7:                               # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
.LBB10_8:                               # %cinch.rc.drop.tobj.exit
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end10:
	.size	cinch.runtime.bytesNew, .Lfunc_end10-cinch.runtime.bytesNew
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesLen          # -- Begin function cinch.runtime.bytesLen
	.p2align	4
	.type	cinch.runtime.bytesLen,@function
cinch.runtime.bytesLen:                 # @cinch.runtime.bytesLen
	.cfi_startproc
# %bb.0:                                # %entry
	testq	%rdi, %rdi
	je	.LBB11_3
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB11_3
# %bb.2:                                # %bb1
	movq	(%rdi), %rax
	leaq	1(,%rax,2), %rax
	retq
.LBB11_3:                               # %bb2
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_bytes_len@PLT
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	.cfi_restore %rbp
	leaq	1(,%rax,2), %rax
	retq
.Lfunc_end11:
	.size	cinch.runtime.bytesLen, .Lfunc_end11-cinch.runtime.bytesLen
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesGet          # -- Begin function cinch.runtime.bytesGet
	.p2align	4
	.type	cinch.runtime.bytesGet,@function
cinch.runtime.bytesGet:                 # @cinch.runtime.bytesGet
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -24
	testb	$1, %sil
	jne	.LBB12_1
# %bb.2:                                # %bb1
	movq	%rdi, %rbx
	movq	%rsi, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	testq	%rdi, %rdi
	jne	.LBB12_4
	jmp	.LBB12_7
.LBB12_1:                               # %bb0
	shrq	%rsi
	testq	%rdi, %rdi
	je	.LBB12_7
.LBB12_4:                               # %bb3
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB12_7
# %bb.5:                                # %bb4
	cmpq	(%rdi), %rsi
	jae	.LBB12_7
# %bb.6:                                # %bb7
	movq	8(%rdi), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB12_8
.LBB12_7:                               # %bb5
	callq	cinch_bytes_get@PLT
.LBB12_8:                               # %bb6
	leaq	1(,%rax,2), %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end12:
	.size	cinch.runtime.bytesGet, .Lfunc_end12-cinch.runtime.bytesGet
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesSet          # -- Begin function cinch.runtime.bytesSet
	.p2align	4
	.type	cinch.runtime.bytesSet,@function
cinch.runtime.bytesSet:                 # @cinch.runtime.bytesSet
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdx, %rbx
	movq	%rsi, %r14
	movq	%rdi, %r15
	testb	$1, %r14b
	jne	.LBB13_1
# %bb.2:                                # %bb1
	movq	%r14, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r12
	testb	$1, %bl
	je	.LBB13_5
.LBB13_4:                               # %bb3
	movq	%rbx, %rdx
	shrq	%rdx
	testq	%r15, %r15
	jne	.LBB13_7
	jmp	.LBB13_11
.LBB13_1:                               # %bb0
	movq	%r14, %r12
	shrq	%r12
	testb	$1, %bl
	jne	.LBB13_4
.LBB13_5:                               # %bb4
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rdx
	testq	%r15, %r15
	je	.LBB13_11
.LBB13_7:                               # %bb6
	movq	-8(%r15), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB13_11
# %bb.8:                                # %bb7
	cmpq	$255, %rdx
	ja	.LBB13_11
# %bb.9:                                # %bb7
	cmpq	(%r15), %r12
	jae	.LBB13_11
# %bb.10:                               # %bb10
	movq	8(%r15), %rax
	movb	%dl, (%rax,%r12)
	jmp	.LBB13_12
.LBB13_11:                              # %bb8
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	cinch_bytes_set@PLT
.LBB13_12:                              # %bb9
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB13_19
# %bb.13:                               # %bb0.i
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB13_15
# %bb.14:                               # %bb2.i
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB13_19
.LBB13_15:                              # %bb3.i
	testq	%rax, %rax
	je	.LBB13_19
# %bb.16:                               # %bb3.i
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB13_18
# %bb.17:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB13_19
.LBB13_18:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB13_19:                              # %cinch.rc.drop.tobj.exit
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB13_26
# %bb.20:                               # %bb0.i5
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB13_22
# %bb.21:                               # %bb2.i14
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB13_26
.LBB13_22:                              # %bb3.i11
	testq	%rax, %rax
	je	.LBB13_26
# %bb.23:                               # %bb3.i11
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB13_25
# %bb.24:                               # %bb5.i12
	callq	cinch_drop_slow@PLT
	jmp	.LBB13_26
.LBB13_25:                              # %bb4.i13
	callq	cinch_drop_mt@PLT
.LBB13_26:                              # %cinch.rc.drop.tobj.exit16
	xorl	%eax, %eax
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end13:
	.size	cinch.runtime.bytesSet, .Lfunc_end13-cinch.runtime.bytesSet
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesData         # -- Begin function cinch.runtime.bytesData
	.p2align	4
	.type	cinch.runtime.bytesData,@function
cinch.runtime.bytesData:                # @cinch.runtime.bytesData
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	testq	%rdi, %rdi
	je	.LBB14_5
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB14_5
# %bb.2:                                # %bb1
	cmpq	$0, (%rdi)
	movl	$1, %eax
	je	.LBB14_4
# %bb.3:                                # %bb1
	movq	8(%rdi), %rax
.LBB14_4:                               # %bb1
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB14_5:                               # %bb2
	.cfi_def_cfa %rbp, 16
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_bytes_data@PLT            # TAILCALL
.Lfunc_end14:
	.size	cinch.runtime.bytesData, .Lfunc_end14-cinch.runtime.bytesData
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesCopy         # -- Begin function cinch.runtime.bytesCopy
	.p2align	4
	.type	cinch.runtime.bytesCopy,@function
cinch.runtime.bytesCopy:                # @cinch.runtime.bytesCopy
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r8, %rbx
	movq	%rcx, %r14
	movq	%rdx, %r12
	movq	%rsi, %r15
	movq	%rdi, %r13
	testb	$1, %r12b
	jne	.LBB15_1
# %bb.2:                                # %bb1
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r12
	testb	$1, %r14b
	je	.LBB15_5
.LBB15_4:                               # %bb3
	shrq	%r14
	testb	$1, %bl
	je	.LBB15_8
.LBB15_7:                               # %bb6
	shrq	%rbx
	jmp	.LBB15_9
.LBB15_1:                               # %bb0
	shrq	%r12
	testb	$1, %r14b
	jne	.LBB15_4
.LBB15_5:                               # %bb4
	movq	%r14, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r14
	testb	$1, %bl
	jne	.LBB15_7
.LBB15_8:                               # %bb7
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rbx
.LBB15_9:                               # %bb8
	movq	%r13, %rdi
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%r14, %rcx
	movq	%rbx, %r8
	callq	cinch_bytes_copy@PLT
	xorl	%eax, %eax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end15:
	.size	cinch.runtime.bytesCopy, .Lfunc_end15-cinch.runtime.bytesCopy
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.bytesCompare      # -- Begin function cinch.runtime.bytesCompare
	.p2align	4
	.type	cinch.runtime.bytesCompare,@function
cinch.runtime.bytesCompare:             # @cinch.runtime.bytesCompare
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %rbx
	movq	%rdi, %r14
	callq	cinch_bytes_compare@PLT
	testq	%r14, %r14
	je	.LBB16_8
# %bb.1:                                # %bb0.i
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB16_3
# %bb.2:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%r14)
	jmp	.LBB16_8
.LBB16_3:                               # %bb3.i
	testq	%rcx, %rcx
	je	.LBB16_8
# %bb.4:                                # %bb3.i
	movq	%rax, %r15
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB16_6
# %bb.5:                                # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB16_7
.LBB16_6:                               # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB16_7:                               # %cinch.rc.drop.obj.exit
	movq	%r15, %rax
.LBB16_8:                               # %cinch.rc.drop.obj.exit
	testq	%rbx, %rbx
	je	.LBB16_16
# %bb.9:                                # %bb0.i2
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB16_11
# %bb.10:                               # %bb2.i11
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB16_16
.LBB16_11:                              # %bb3.i8
	testq	%rcx, %rcx
	je	.LBB16_16
# %bb.12:                               # %bb3.i8
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB16_14
# %bb.13:                               # %bb5.i9
	callq	cinch_drop_slow@PLT
	jmp	.LBB16_15
.LBB16_14:                              # %bb4.i10
	callq	cinch_drop_mt@PLT
.LBB16_15:                              # %cinch.rc.drop.obj.exit13
	movq	%r14, %rax
.LBB16_16:                              # %cinch.rc.drop.obj.exit13
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end16:
	.size	cinch.runtime.bytesCompare, .Lfunc_end16-cinch.runtime.bytesCompare
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.randomBytes       # -- Begin function cinch.runtime.randomBytes
	.p2align	4
	.type	cinch.runtime.randomBytes,@function
cinch.runtime.randomBytes:              # @cinch.runtime.randomBytes
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_random_bytes@PLT          # TAILCALL
.Lfunc_end17:
	.size	cinch.runtime.randomBytes, .Lfunc_end17-cinch.runtime.randomBytes
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.processArgCount   # -- Begin function cinch.runtime.processArgCount
	.p2align	4
	.type	cinch.runtime.processArgCount,@function
cinch.runtime.processArgCount:          # @cinch.runtime.processArgCount
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_argv_count@PLT
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_from_i64@PLT          # TAILCALL
.Lfunc_end18:
	.size	cinch.runtime.processArgCount, .Lfunc_end18-cinch.runtime.processArgCount
	.cfi_endproc
                                        # -- End function
	.globl	cinch.runtime.processArg        # -- Begin function cinch.runtime.processArg
	.p2align	4
	.type	cinch.runtime.processArg,@function
cinch.runtime.processArg:               # @cinch.runtime.processArg
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_argv_get@PLT              # TAILCALL
.Lfunc_end19:
	.size	cinch.runtime.processArg, .Lfunc_end19-cinch.runtime.processArg
	.cfi_endproc
                                        # -- End function
	.globl	cinch.prim.wrappingShlU64       # -- Begin function cinch.prim.wrappingShlU64
	.p2align	4
	.type	cinch.prim.wrappingShlU64,@function
cinch.prim.wrappingShlU64:              # @cinch.prim.wrappingShlU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rsi, %rcx
	movq	%rdi, %rax
                                        # kill: def $cl killed $cl killed $rcx
	shlq	%cl, %rax
	popq	%rbp
	retq
.Lfunc_end20:
	.size	cinch.prim.wrappingShlU64, .Lfunc_end20-cinch.prim.wrappingShlU64
                                        # -- End function
	.globl	cinch.prim.wrappingShrU64       # -- Begin function cinch.prim.wrappingShrU64
	.p2align	4
	.type	cinch.prim.wrappingShrU64,@function
cinch.prim.wrappingShrU64:              # @cinch.prim.wrappingShrU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rsi, %rcx
	movq	%rdi, %rax
                                        # kill: def $cl killed $cl killed $rcx
	shrq	%cl, %rax
	popq	%rbp
	retq
.Lfunc_end21:
	.size	cinch.prim.wrappingShrU64, .Lfunc_end21-cinch.prim.wrappingShrU64
                                        # -- End function
	.globl	cinch.prim.bitAndU64            # -- Begin function cinch.prim.bitAndU64
	.p2align	4
	.type	cinch.prim.bitAndU64,@function
cinch.prim.bitAndU64:                   # @cinch.prim.bitAndU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	andq	%rsi, %rax
	popq	%rbp
	retq
.Lfunc_end22:
	.size	cinch.prim.bitAndU64, .Lfunc_end22-cinch.prim.bitAndU64
                                        # -- End function
	.globl	cinch.prim.bitOrU64             # -- Begin function cinch.prim.bitOrU64
	.p2align	4
	.type	cinch.prim.bitOrU64,@function
cinch.prim.bitOrU64:                    # @cinch.prim.bitOrU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	orq	%rsi, %rax
	popq	%rbp
	retq
.Lfunc_end23:
	.size	cinch.prim.bitOrU64, .Lfunc_end23-cinch.prim.bitOrU64
                                        # -- End function
	.globl	cinch.prim.eqI64                # -- Begin function cinch.prim.eqI64
	.p2align	4
	.type	cinch.prim.eqI64,@function
cinch.prim.eqI64:                       # @cinch.prim.eqI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	sete	%al
	popq	%rbp
	retq
.Lfunc_end24:
	.size	cinch.prim.eqI64, .Lfunc_end24-cinch.prim.eqI64
                                        # -- End function
	.globl	cinch.prim.ltI64                # -- Begin function cinch.prim.ltI64
	.p2align	4
	.type	cinch.prim.ltI64,@function
cinch.prim.ltI64:                       # @cinch.prim.ltI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	setl	%al
	popq	%rbp
	retq
.Lfunc_end25:
	.size	cinch.prim.ltI64, .Lfunc_end25-cinch.prim.ltI64
                                        # -- End function
	.globl	cinch.prim.leI64                # -- Begin function cinch.prim.leI64
	.p2align	4
	.type	cinch.prim.leI64,@function
cinch.prim.leI64:                       # @cinch.prim.leI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	setle	%al
	popq	%rbp
	retq
.Lfunc_end26:
	.size	cinch.prim.leI64, .Lfunc_end26-cinch.prim.leI64
                                        # -- End function
	.globl	cinch.prim.gtI64                # -- Begin function cinch.prim.gtI64
	.p2align	4
	.type	cinch.prim.gtI64,@function
cinch.prim.gtI64:                       # @cinch.prim.gtI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	setg	%al
	popq	%rbp
	retq
.Lfunc_end27:
	.size	cinch.prim.gtI64, .Lfunc_end27-cinch.prim.gtI64
                                        # -- End function
	.globl	cinch.prim.geI64                # -- Begin function cinch.prim.geI64
	.p2align	4
	.type	cinch.prim.geI64,@function
cinch.prim.geI64:                       # @cinch.prim.geI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	setge	%al
	popq	%rbp
	retq
.Lfunc_end28:
	.size	cinch.prim.geI64, .Lfunc_end28-cinch.prim.geI64
                                        # -- End function
	.globl	cinch.prim.eqU64                # -- Begin function cinch.prim.eqU64
	.p2align	4
	.type	cinch.prim.eqU64,@function
cinch.prim.eqU64:                       # @cinch.prim.eqU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	sete	%al
	popq	%rbp
	retq
.Lfunc_end29:
	.size	cinch.prim.eqU64, .Lfunc_end29-cinch.prim.eqU64
                                        # -- End function
	.globl	cinch.prim.leU64                # -- Begin function cinch.prim.leU64
	.p2align	4
	.type	cinch.prim.leU64,@function
cinch.prim.leU64:                       # @cinch.prim.leU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpq	%rsi, %rdi
	setbe	%al
	popq	%rbp
	retq
.Lfunc_end30:
	.size	cinch.prim.leU64, .Lfunc_end30-cinch.prim.leU64
                                        # -- End function
	.globl	cinch.prim.charToU32            # -- Begin function cinch.prim.charToU32
	.p2align	4
	.type	cinch.prim.charToU32,@function
cinch.prim.charToU32:                   # @cinch.prim.charToU32
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end31:
	.size	cinch.prim.charToU32, .Lfunc_end31-cinch.prim.charToU32
                                        # -- End function
	.globl	cinch.prim.u32ToChar            # -- Begin function cinch.prim.u32ToChar
	.p2align	4
	.type	cinch.prim.u32ToChar,@function
cinch.prim.u32ToChar:                   # @cinch.prim.u32ToChar
# %bb.0:                                # %entry
	cmpq	$1114112, %rdi                  # imm = 0x110000
	setb	%al
	movq	%rdi, %rcx
	andq	$-2048, %rcx                    # imm = 0xF800
	cmpq	$55296, %rcx                    # imm = 0xD800
	setne	%cl
	testb	%cl, %al
	jne	.LBB32_3
# %bb.1:
	leaq	"cinch.prim.u32ToChar.blk#h"+8(%rip), %rax
	retq
.LBB32_3:                               # %bb1
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$1, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end32:
	.size	cinch.prim.u32ToChar, .Lfunc_end32-cinch.prim.u32ToChar
                                        # -- End function
	.globl	cinch.prim.wrappingI64ToU32     # -- Begin function cinch.prim.wrappingI64ToU32
	.p2align	4
	.type	cinch.prim.wrappingI64ToU32,@function
cinch.prim.wrappingI64ToU32:            # @cinch.prim.wrappingI64ToU32
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %eax
	popq	%rbp
	retq
.Lfunc_end33:
	.size	cinch.prim.wrappingI64ToU32, .Lfunc_end33-cinch.prim.wrappingI64ToU32
                                        # -- End function
	.globl	cinch.prim.wrappingI64ToU64     # -- Begin function cinch.prim.wrappingI64ToU64
	.p2align	4
	.type	cinch.prim.wrappingI64ToU64,@function
cinch.prim.wrappingI64ToU64:            # @cinch.prim.wrappingI64ToU64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end34:
	.size	cinch.prim.wrappingI64ToU64, .Lfunc_end34-cinch.prim.wrappingI64ToU64
                                        # -- End function
	.globl	cinch.prim.u8ToI64              # -- Begin function cinch.prim.u8ToI64
	.p2align	4
	.type	cinch.prim.u8ToI64,@function
cinch.prim.u8ToI64:                     # @cinch.prim.u8ToI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end35:
	.size	cinch.prim.u8ToI64, .Lfunc_end35-cinch.prim.u8ToI64
                                        # -- End function
	.globl	cinch.prim.u32ToI64             # -- Begin function cinch.prim.u32ToI64
	.p2align	4
	.type	cinch.prim.u32ToI64,@function
cinch.prim.u32ToI64:                    # @cinch.prim.u32ToI64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end36:
	.size	cinch.prim.u32ToI64, .Lfunc_end36-cinch.prim.u32ToI64
                                        # -- End function
	.globl	cinch.prim.wrappingU64ToU8      # -- Begin function cinch.prim.wrappingU64ToU8
	.p2align	4
	.type	cinch.prim.wrappingU64ToU8,@function
cinch.prim.wrappingU64ToU8:             # @cinch.prim.wrappingU64ToU8
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movzbl	%dil, %eax
	popq	%rbp
	retq
.Lfunc_end37:
	.size	cinch.prim.wrappingU64ToU8, .Lfunc_end37-cinch.prim.wrappingU64ToU8
                                        # -- End function
	.globl	cinch.prim.wrappingU64ToU32     # -- Begin function cinch.prim.wrappingU64ToU32
	.p2align	4
	.type	cinch.prim.wrappingU64ToU32,@function
cinch.prim.wrappingU64ToU32:            # @cinch.prim.wrappingU64ToU32
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %eax
	popq	%rbp
	retq
.Lfunc_end38:
	.size	cinch.prim.wrappingU64ToU32, .Lfunc_end38-cinch.prim.wrappingU64ToU32
                                        # -- End function
	.globl	cinch.runtime.ioWrite           # -- Begin function cinch.runtime.ioWrite
	.p2align	4
	.type	cinch.runtime.ioWrite,@function
cinch.runtime.ioWrite:                  # @cinch.runtime.ioWrite
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rdi
	callq	cinch_io_write@PLT
	xorl	%eax, %eax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end39:
	.size	cinch.runtime.ioWrite, .Lfunc_end39-cinch.runtime.ioWrite
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Clock.raw-monotonic-ns" # -- Begin function Effect.Clock.raw-monotonic-ns
	.p2align	4
	.type	"Effect.Clock.raw-monotonic-ns",@function
"Effect.Clock.raw-monotonic-ns":        # @Effect.Clock.raw-monotonic-ns
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_clock_monotonic_ns@PLT
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end40:
	.size	"Effect.Clock.raw-monotonic-ns", .Lfunc_end40-"Effect.Clock.raw-monotonic-ns"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Clock.raw-wall-ns"      # -- Begin function Effect.Clock.raw-wall-ns
	.p2align	4
	.type	"Effect.Clock.raw-wall-ns",@function
"Effect.Clock.raw-wall-ns":             # @Effect.Clock.raw-wall-ns
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_clock_wall_ns@PLT
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end41:
	.size	"Effect.Clock.raw-wall-ns", .Lfunc_end41-"Effect.Clock.raw-wall-ns"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Concurrent.MVar.cell-new@O" # -- Begin function Effect.Concurrent.MVar.cell-new@O
	.p2align	4
	.type	"Effect.Concurrent.MVar.cell-new@O",@function
"Effect.Concurrent.MVar.cell-new@O":    # @"Effect.Concurrent.MVar.cell-new@O"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$1, %esi
	movl	$1, %edx
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_mvar_new@PLT              # TAILCALL
.Lfunc_end42:
	.size	"Effect.Concurrent.MVar.cell-new@O", .Lfunc_end42-"Effect.Concurrent.MVar.cell-new@O"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Concurrent.MVar.cell-take@O" # -- Begin function Effect.Concurrent.MVar.cell-take@O
	.p2align	4
	.type	"Effect.Concurrent.MVar.cell-take@O",@function
"Effect.Concurrent.MVar.cell-take@O":   # @"Effect.Concurrent.MVar.cell-take@O"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	leaq	-8(%rbp), %r8
	xorl	%esi, %esi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	-8(%rbp), %rax
	addq	$16, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end43:
	.size	"Effect.Concurrent.MVar.cell-take@O", .Lfunc_end43-"Effect.Concurrent.MVar.cell-take@O"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Concurrent.MVar.cell-put@O" # -- Begin function Effect.Concurrent.MVar.cell-put@O
	.p2align	4
	.type	"Effect.Concurrent.MVar.cell-put@O",@function
"Effect.Concurrent.MVar.cell-put@O":    # @"Effect.Concurrent.MVar.cell-put@O"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	leaq	-8(%rbp), %r8
	movl	$1, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	xorl	%eax, %eax
	addq	$16, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end44:
	.size	"Effect.Concurrent.MVar.cell-put@O", .Lfunc_end44-"Effect.Concurrent.MVar.cell-put@O"
	.cfi_endproc
                                        # -- End function
	.globl	Data.Core.Nil                   # -- Begin function Data.Core.Nil
	.p2align	4
	.type	Data.Core.Nil,@function
Data.Core.Nil:                          # @Data.Core.Nil
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rax
	popq	%rbp
	retq
.Lfunc_end45:
	.size	Data.Core.Nil, .Lfunc_end45-Data.Core.Nil
                                        # -- End function
	.globl	"Data.Core.Cons@R"              # -- Begin function Data.Core.Cons@R
	.p2align	4
	.type	"Data.Core.Cons@R",@function
"Data.Core.Cons@R":                     # @"Data.Core.Cons@R"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end46:
	.size	"Data.Core.Cons@R", .Lfunc_end46-"Data.Core.Cons@R"
                                        # -- End function
	.globl	"Data.Core.Cons@O"              # -- Begin function Data.Core.Cons@O
	.p2align	4
	.type	"Data.Core.Cons@O",@function
"Data.Core.Cons@O":                     # @"Data.Core.Cons@O"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	movl	$769, %esi                      # imm = 0x301
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end47:
	.size	"Data.Core.Cons@O", .Lfunc_end47-"Data.Core.Cons@O"
                                        # -- End function
	.globl	Data.Core.None                  # -- Begin function Data.Core.None
	.p2align	4
	.type	Data.Core.None,@function
Data.Core.None:                         # @Data.Core.None
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	popq	%rbp
	retq
.Lfunc_end48:
	.size	Data.Core.None, .Lfunc_end48-Data.Core.None
                                        # -- End function
	.globl	"Data.Core.Some@O"              # -- Begin function Data.Core.Some@O
	.p2align	4
	.type	"Data.Core.Some@O",@function
"Data.Core.Some@O":                     # @"Data.Core.Some@O"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end49:
	.size	"Data.Core.Some@O", .Lfunc_end49-"Data.Core.Some@O"
                                        # -- End function
	.globl	"Data.Core.Some@T"              # -- Begin function Data.Core.Some@T
	.p2align	4
	.type	"Data.Core.Some@T",@function
"Data.Core.Some@T":                     # @"Data.Core.Some@T"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end50:
	.size	"Data.Core.Some@T", .Lfunc_end50-"Data.Core.Some@T"
                                        # -- End function
	.globl	"Data.Core.Err@OO"              # -- Begin function Data.Core.Err@OO
	.p2align	4
	.type	"Data.Core.Err@OO",@function
"Data.Core.Err@OO":                     # @"Data.Core.Err@OO"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end51:
	.size	"Data.Core.Err@OO", .Lfunc_end51-"Data.Core.Err@OO"
                                        # -- End function
	.globl	"Data.Core.Ok@OO"               # -- Begin function Data.Core.Ok@OO
	.p2align	4
	.type	"Data.Core.Ok@OO",@function
"Data.Core.Ok@OO":                      # @"Data.Core.Ok@OO"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end52:
	.size	"Data.Core.Ok@OO", .Lfunc_end52-"Data.Core.Ok@OO"
                                        # -- End function
	.globl	"Tuple.2@RT"                    # -- Begin function Tuple.2@RT
	.p2align	4
	.type	"Tuple.2@RT",@function
"Tuple.2@RT":                           # @"Tuple.2@RT"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	movl	$512, %esi                      # imm = 0x200
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end53:
	.size	"Tuple.2@RT", .Lfunc_end53-"Tuple.2@RT"
                                        # -- End function
	.globl	"Tuple.2@OR"                    # -- Begin function Tuple.2@OR
	.p2align	4
	.type	"Tuple.2@OR",@function
"Tuple.2@OR":                           # @"Tuple.2@OR"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end54:
	.size	"Tuple.2@OR", .Lfunc_end54-"Tuple.2@OR"
                                        # -- End function
	.globl	"Data.List.reverse@R"           # -- Begin function Data.List.reverse@R
	.p2align	4
	.type	"Data.List.reverse@R",@function
"Data.List.reverse@R":                  # @"Data.List.reverse@R"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB55_2
# %bb.1:                                # %bb2
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB55_2:                               # %bb4
	testq	%rax, %rax
	je	.LBB55_7
# %bb.3:                                # %bb4
	cmpq	$1, %rax
	jne	.LBB55_34
# %bb.4:                                # %bb6
	movq	(%rdi), %r14
	movq	8(%rdi), %rbx
	testq	%rdi, %rdi
	je	.LBB55_10
# %bb.5:                                # %cinch.rc.unique.exit
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB55_10
# %bb.6:                                # %bb11
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%rdi)
	jmp	.LBB55_27
.LBB55_7:                               # %bb5
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rax
	testq	%rdi, %rdi
	je	.LBB55_35
# %bb.8:                                # %bb0.i
	movq	-8(%rdi), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB55_13
# %bb.9:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%rdi)
	jmp	.LBB55_35
.LBB55_10:                              # %bb8
	testq	%rbx, %rbx
	je	.LBB55_18
# %bb.11:                               # %bb0.i16
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB55_16
# %bb.12:                               # %bb2.i24
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB55_18
.LBB55_13:                              # %bb3.i
	testq	%rcx, %rcx
	je	.LBB55_35
# %bb.14:                               # %bb3.i
	movq	%rax, %rbx
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB55_24
# %bb.15:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	movq	%rbx, %rax
	jmp	.LBB55_35
.LBB55_16:                              # %bb3.i22
	je	.LBB55_18
# %bb.17:                               # %bb4.i23
	movq	%rdi, %r15
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	movq	%r15, %rdi
.LBB55_18:                              # %cinch.rc.dup.obj.exit
	testq	%rdi, %rdi
	je	.LBB55_26
# %bb.19:                               # %bb0.i27
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB55_21
# %bb.20:                               # %bb2.i36
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB55_26
.LBB55_21:                              # %bb3.i33
	testq	%rax, %rax
	je	.LBB55_26
# %bb.22:                               # %bb3.i33
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB55_25
# %bb.23:                               # %bb5.i34
	callq	cinch_drop_slow@PLT
	jmp	.LBB55_26
.LBB55_24:                              # %bb4.i
	callq	cinch_drop_mt@PLT
	movq	%rbx, %rax
	jmp	.LBB55_35
.LBB55_25:                              # %bb4.i35
	callq	cinch_drop_mt@PLT
.LBB55_26:                              # %bb10
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%r14, (%rax)
.LBB55_27:                              # %bb12
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rax
	movq	%rax, 8(%rdi)
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB55_29
# %bb.28:                               # %bb14
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB55_29:                              # %bb16
	cmpq	$1, %rax
	je	.LBB55_36
# %bb.30:                               # %bb16
	testq	%rax, %rax
	jne	.LBB55_34
# %bb.31:                               # %bb17
	testq	%rbx, %rbx
	je	.LBB55_42
# %bb.32:                               # %bb0.i40
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB55_43
# %bb.33:                               # %bb2.i49
	decq	%rcx
	movq	%rcx, -8(%rbx)
	movq	%rdi, %rax
	jmp	.LBB55_35
.LBB55_34:
	xorl	%eax, %eax
.LBB55_35:                              # %common.ret
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
.LBB55_36:                              # %bb18
	movq	(%rbx), %r15
	movq	8(%rbx), %r14
	testq	%rbx, %rbx
	je	.LBB55_39
# %bb.37:                               # %cinch.rc.unique.exit60
	movq	-8(%rbx), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB55_39
# %bb.38:                               # %bb25
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%rbx)
	movq	%rdi, 8(%rbx)
	jmp	.LBB55_58
.LBB55_39:                              # %bb22
	testq	%r14, %r14
	je	.LBB55_48
# %bb.40:                               # %bb0.i62
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB55_46
# %bb.41:                               # %bb2.i71
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB55_48
.LBB55_43:                              # %bb3.i46
	testq	%rax, %rax
	je	.LBB55_42
# %bb.44:                               # %bb3.i46
	movq	%rdi, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB55_55
# %bb.45:                               # %bb5.i47
	callq	cinch_drop_slow@PLT
	movq	%r14, %rax
	jmp	.LBB55_35
.LBB55_42:
	movq	%rdi, %rax
	jmp	.LBB55_35
.LBB55_46:                              # %bb3.i68
	je	.LBB55_48
# %bb.47:                               # %bb4.i70
	movq	%rdi, %r12
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
	movq	%r12, %rdi
.LBB55_48:                              # %cinch.rc.dup.obj.exit73
	movq	%rdi, %r12
	testq	%rbx, %rbx
	je	.LBB55_57
# %bb.49:                               # %bb0.i75
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB55_51
# %bb.50:                               # %bb2.i84
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB55_57
.LBB55_51:                              # %bb3.i81
	testq	%rax, %rax
	je	.LBB55_57
# %bb.52:                               # %bb3.i81
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB55_56
# %bb.53:                               # %bb5.i82
	callq	cinch_drop_slow@PLT
	jmp	.LBB55_57
.LBB55_55:                              # %bb4.i48
	callq	cinch_drop_mt@PLT
	movq	%r14, %rax
	jmp	.LBB55_35
.LBB55_56:                              # %bb4.i83
	callq	cinch_drop_mt@PLT
.LBB55_57:                              # %bb24
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movq	%r12, 8(%rax)
.LBB55_58:                              # %bb26
	movq	%r14, %rdi
	movq	%rbx, %rsi
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	jmp	"Data.List.reverse-into@R"@PLT  # TAILCALL
.Lfunc_end55:
	.size	"Data.List.reverse@R", .Lfunc_end55-"Data.List.reverse@R"
                                        # -- End function
	.globl	"Data.List.reverse-into@R"      # -- Begin function Data.List.reverse-into@R
	.p2align	4
	.type	"Data.List.reverse-into@R",@function
"Data.List.reverse-into@R":             # @"Data.List.reverse-into@R"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	pushq	%rax
	movq	%rsi, %r14
	movq	%rdi, %r15
	movabsq	$34359738367, %r13              # imm = 0x7FFFFFFFF
	jmp	.LBB56_3
.LBB56_1:                               # %bb2.i456
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
.LBB56_2:                               # %bb150
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r12, (%rax)
	movq	%rbx, 8(%rax)
.LBB56_3:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_5
# %bb.4:                                # %bb2
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB56_5:                               # %bb4
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_153
# %bb.6:                                # %bb6
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r15), %rbx
	movq	8(%r15), %r12
	testq	%r15, %r15
	je	.LBB56_9
# %bb.7:                                # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB56_9
# %bb.8:                                # %bb11
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r15)
	movq	%r14, 8(%r15)
	jmp	.LBB56_22
	.p2align	4
.LBB56_9:                               # %bb8
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r12, %r12
	je	.LBB56_14
# %bb.10:                               # %bb0.i100
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_12
# %bb.11:                               # %bb2.i108
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB56_14
.LBB56_12:                              # %bb3.i106
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_14
# %bb.13:                               # %bb4.i107
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_14:                              # %cinch.rc.dup.obj.exit
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r15, %r15
	je	.LBB56_21
# %bb.15:                               # %bb0.i111
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_17
# %bb.16:                               # %bb2.i120
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB56_21
	.p2align	4
.LBB56_17:                              # %bb3.i117
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_21
# %bb.18:                               # %bb3.i117
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_20
# %bb.19:                               # %bb5.i118
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_21
.LBB56_20:                              # %bb4.i119
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_21:                              # %bb10
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
.LBB56_22:                              # %bb12
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_24
# %bb.23:                               # %bb14
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r12), %rax
	movq	(%rax), %rax
.LBB56_24:                              # %bb16
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_158
# %bb.25:                               # %bb18
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r12), %rbx
	movq	8(%r12), %r14
	testq	%r12, %r12
	je	.LBB56_28
# %bb.26:                               # %cinch.rc.unique.exit144
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB56_28
# %bb.27:                               # %bb25
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r12)
	movq	%r15, 8(%r12)
	jmp	.LBB56_41
	.p2align	4
.LBB56_28:                              # %bb22
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r14, %r14
	je	.LBB56_33
# %bb.29:                               # %bb0.i146
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_31
# %bb.30:                               # %bb2.i155
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB56_33
.LBB56_31:                              # %bb3.i152
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_33
# %bb.32:                               # %bb4.i154
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_33:                              # %cinch.rc.dup.obj.exit157
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r12, %r12
	je	.LBB56_40
# %bb.34:                               # %bb0.i159
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_36
# %bb.35:                               # %bb2.i168
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB56_40
.LBB56_36:                              # %bb3.i165
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_40
# %bb.37:                               # %bb3.i165
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_39
# %bb.38:                               # %bb5.i166
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_40
.LBB56_39:                              # %bb4.i167
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_40:                              # %bb24
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r12
	movq	%rbx, (%rax)
	movq	%r15, 8(%rax)
.LBB56_41:                              # %bb26
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_43
# %bb.42:                               # %bb28
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB56_43:                              # %bb30
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_163
# %bb.44:                               # %bb32
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r14), %rbx
	movq	8(%r14), %r15
	testq	%r14, %r14
	je	.LBB56_47
# %bb.45:                               # %cinch.rc.unique.exit192
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB56_47
# %bb.46:                               # %bb41
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r14)
	movq	%r12, 8(%r14)
	jmp	.LBB56_60
	.p2align	4
.LBB56_47:                              # %bb38
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r15, %r15
	je	.LBB56_52
# %bb.48:                               # %bb0.i194
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_50
# %bb.49:                               # %bb2.i203
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB56_52
.LBB56_50:                              # %bb3.i200
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_52
# %bb.51:                               # %bb4.i202
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_52:                              # %cinch.rc.dup.obj.exit205
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r14, %r14
	je	.LBB56_59
# %bb.53:                               # %bb0.i207
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_55
# %bb.54:                               # %bb2.i216
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB56_59
.LBB56_55:                              # %bb3.i213
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_59
# %bb.56:                               # %bb3.i213
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_58
# %bb.57:                               # %bb5.i214
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_59
.LBB56_58:                              # %bb4.i215
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_59:                              # %bb40
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	movq	%r12, 8(%rax)
.LBB56_60:                              # %bb42
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_62
# %bb.61:                               # %bb44
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB56_62:                              # %bb46
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_167
# %bb.63:                               # %bb48
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r15), %rbx
	movq	8(%r15), %r12
	testq	%r15, %r15
	je	.LBB56_66
# %bb.64:                               # %cinch.rc.unique.exit240
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB56_66
# %bb.65:                               # %bb59
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r15)
	movq	%r14, 8(%r15)
	jmp	.LBB56_79
	.p2align	4
.LBB56_66:                              # %bb56
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r12, %r12
	je	.LBB56_71
# %bb.67:                               # %bb0.i242
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_69
# %bb.68:                               # %bb2.i251
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB56_71
.LBB56_69:                              # %bb3.i248
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_71
# %bb.70:                               # %bb4.i250
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_71:                              # %cinch.rc.dup.obj.exit253
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r15, %r15
	je	.LBB56_78
# %bb.72:                               # %bb0.i255
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_74
# %bb.73:                               # %bb2.i264
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB56_78
.LBB56_74:                              # %bb3.i261
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_78
# %bb.75:                               # %bb3.i261
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_77
# %bb.76:                               # %bb5.i262
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_78
.LBB56_77:                              # %bb4.i263
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_78:                              # %bb58
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
.LBB56_79:                              # %bb60
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_81
# %bb.80:                               # %bb62
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r12), %rax
	movq	(%rax), %rax
.LBB56_81:                              # %bb64
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_158
# %bb.82:                               # %bb66
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r12), %rbx
	movq	8(%r12), %r13
	testq	%r12, %r12
	je	.LBB56_85
# %bb.83:                               # %cinch.rc.unique.exit288
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB56_85
# %bb.84:                               # %bb79
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r12)
	movq	%r15, 8(%r12)
	jmp	.LBB56_98
	.p2align	4
.LBB56_85:                              # %bb76
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r13, %r13
	je	.LBB56_90
# %bb.86:                               # %bb0.i290
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r13), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_88
# %bb.87:                               # %bb2.i299
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r13)
	jmp	.LBB56_90
.LBB56_88:                              # %bb3.i296
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_90
# %bb.89:                               # %bb4.i298
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r13, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_90:                              # %cinch.rc.dup.obj.exit301
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r12, %r12
	je	.LBB56_97
# %bb.91:                               # %bb0.i303
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_93
# %bb.92:                               # %bb2.i312
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB56_97
.LBB56_93:                              # %bb3.i309
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_97
# %bb.94:                               # %bb3.i309
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_96
# %bb.95:                               # %bb5.i310
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_97
.LBB56_96:                              # %bb4.i311
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_97:                              # %bb78
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r12
	movq	%rbx, (%rax)
	movq	%r15, 8(%rax)
.LBB56_98:                              # %bb80
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_100
# %bb.99:                               # %bb82
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB56_100:                             # %bb84
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_172
# %bb.101:                              # %bb86
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r13), %r14
	movq	8(%r13), %rbx
	testq	%r13, %r13
	je	.LBB56_104
# %bb.102:                              # %cinch.rc.unique.exit336
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r13), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB56_104
# %bb.103:                              # %bb101
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r13)
	movq	%r12, 8(%r13)
	jmp	.LBB56_117
	.p2align	4
.LBB56_104:                             # %bb98
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB56_109
# %bb.105:                              # %bb0.i338
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_107
# %bb.106:                              # %bb2.i347
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB56_109
.LBB56_107:                             # %bb3.i344
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_109
# %bb.108:                              # %bb4.i346
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_109:                             # %cinch.rc.dup.obj.exit349
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r13, %r13
	je	.LBB56_116
# %bb.110:                              # %bb0.i351
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_112
# %bb.111:                              # %bb2.i360
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB56_116
.LBB56_112:                             # %bb3.i357
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_116
# %bb.113:                              # %bb3.i357
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_115
# %bb.114:                              # %bb5.i358
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_116
.LBB56_115:                             # %bb4.i359
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_116:                             # %bb100
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%r14, (%rax)
	movq	%r12, 8(%rax)
.LBB56_117:                             # %bb102
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB56_119
# %bb.118:                              # %bb104
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB56_119:                             # %bb106
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_176
# %bb.120:                              # %bb108
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%rbx), %r15
	movq	8(%rbx), %r14
	testq	%rbx, %rbx
	je	.LBB56_123
# %bb.121:                              # %cinch.rc.unique.exit384
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB56_123
# %bb.122:                              # %bb125
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%rbx)
	movq	%r13, 8(%rbx)
	jmp	.LBB56_136
	.p2align	4
.LBB56_123:                             # %bb122
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r14, %r14
	je	.LBB56_128
# %bb.124:                              # %bb0.i386
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_126
# %bb.125:                              # %bb2.i395
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB56_128
.LBB56_126:                             # %bb3.i392
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_128
# %bb.127:                              # %bb4.i394
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_128:                             # %cinch.rc.dup.obj.exit397
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB56_135
# %bb.129:                              # %bb0.i399
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_131
# %bb.130:                              # %bb2.i408
                                        #   in Loop: Header=BB56_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB56_135
.LBB56_131:                             # %bb3.i405
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_135
# %bb.132:                              # %bb3.i405
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_134
# %bb.133:                              # %bb5.i406
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_135
.LBB56_134:                             # %bb4.i407
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB56_135:                             # %bb124
                                        #   in Loop: Header=BB56_3 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movq	%r13, 8(%rax)
.LBB56_136:                             # %bb126
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	movabsq	$34359738367, %r13              # imm = 0x7FFFFFFFF
	jne	.LBB56_138
# %bb.137:                              # %bb128
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB56_138:                             # %bb130
                                        #   in Loop: Header=BB56_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB56_180
# %bb.139:                              # %bb132
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	(%r14), %r12
	movq	8(%r14), %r15
	testq	%r14, %r14
	je	.LBB56_142
# %bb.140:                              # %cinch.rc.unique.exit432
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB56_142
# %bb.141:                              # %bb151
                                        #   in Loop: Header=BB56_3 Depth=1
	movabsq	$564083824787457, %rax          # imm = 0x2010800000001
	movq	%rax, -8(%r14)
	movq	%rbx, 8(%r14)
	jmp	.LBB56_3
	.p2align	4
.LBB56_142:                             # %bb148
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r15, %r15
	je	.LBB56_147
# %bb.143:                              # %bb0.i434
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB56_145
# %bb.144:                              # %bb2.i443
                                        #   in Loop: Header=BB56_3 Depth=1
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB56_147
.LBB56_145:                             # %bb3.i440
                                        #   in Loop: Header=BB56_3 Depth=1
	je	.LBB56_147
# %bb.146:                              # %bb4.i442
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB56_147:                             # %cinch.rc.dup.obj.exit445
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%r14, %r14
	je	.LBB56_2
# %bb.148:                              # %bb0.i447
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB56_1
# %bb.149:                              # %bb3.i453
                                        #   in Loop: Header=BB56_3 Depth=1
	testq	%rax, %rax
	je	.LBB56_2
# %bb.150:                              # %bb3.i453
                                        #   in Loop: Header=BB56_3 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_152
# %bb.151:                              # %bb5.i454
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_2
.LBB56_152:                             # %bb4.i455
                                        #   in Loop: Header=BB56_3 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB56_2
.LBB56_158:                             # %bb16
	testq	%rax, %rax
	jne	.LBB56_168
# %bb.159:                              # %bb17
	testq	%r12, %r12
	je	.LBB56_162
# %bb.160:                              # %bb0.i124
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_161
# %bb.171:                              # %bb2.i133
	decq	%rcx
	movq	%rcx, -8(%r12)
	movq	%r15, %rax
	jmp	.LBB56_206
.LBB56_153:                             # %bb4
	testq	%rax, %rax
	movl	$0, %eax
	jne	.LBB56_206
	jmp	.LBB56_154
.LBB56_163:                             # %bb30
	testq	%rax, %rax
	jne	.LBB56_168
# %bb.164:                              # %bb31
	testq	%r14, %r14
	je	.LBB56_203
# %bb.165:                              # %bb0.i172
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_184
# %bb.166:                              # %bb2.i181
	decq	%rcx
	movq	%rcx, -8(%r14)
	movq	%r12, %rax
	jmp	.LBB56_206
.LBB56_167:                             # %bb46
	testq	%rax, %rax
	jne	.LBB56_168
.LBB56_154:                             # %bb5
	testq	%r15, %r15
	je	.LBB56_157
# %bb.155:                              # %bb0.i
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_156
# %bb.169:                              # %bb2.i
	decq	%rcx
	movq	%rcx, -8(%r15)
	movq	%r14, %rax
	jmp	.LBB56_206
.LBB56_172:                             # %bb84
	testq	%rax, %rax
	jne	.LBB56_168
# %bb.173:                              # %bb85
	testq	%r13, %r13
	je	.LBB56_203
# %bb.174:                              # %bb0.i316
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_190
# %bb.175:                              # %bb2.i325
	decq	%rcx
	movq	%rcx, -8(%r13)
	movq	%r12, %rax
	jmp	.LBB56_206
.LBB56_176:                             # %bb106
	testq	%rax, %rax
	jne	.LBB56_168
# %bb.177:                              # %bb107
	testq	%rbx, %rbx
	je	.LBB56_205
# %bb.178:                              # %bb0.i364
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_194
# %bb.179:                              # %bb2.i373
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB56_205
.LBB56_180:                             # %bb130
	testq	%rax, %rax
	jne	.LBB56_168
# %bb.181:                              # %bb131
	testq	%r14, %r14
	je	.LBB56_207
# %bb.182:                              # %bb0.i412
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB56_197
# %bb.183:                              # %bb2.i421
	decq	%rcx
	movq	%rcx, -8(%r14)
	movq	%rbx, %rax
	jmp	.LBB56_206
.LBB56_168:
	xorl	%eax, %eax
.LBB56_206:                             # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
.LBB56_156:                             # %bb3.i
	testq	%rax, %rax
	je	.LBB56_157
# %bb.186:                              # %bb3.i
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_200
# %bb.187:                              # %bb5.i
	callq	cinch_drop_slow@PLT
	movq	%r14, %rax
	jmp	.LBB56_206
.LBB56_157:
	movq	%r14, %rax
	jmp	.LBB56_206
.LBB56_161:                             # %bb3.i130
	testq	%rax, %rax
	je	.LBB56_162
# %bb.188:                              # %bb3.i130
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_201
# %bb.189:                              # %bb5.i131
	callq	cinch_drop_slow@PLT
	movq	%r15, %rax
	jmp	.LBB56_206
.LBB56_162:
	movq	%r15, %rax
	jmp	.LBB56_206
.LBB56_184:                             # %bb3.i178
	testq	%rax, %rax
	je	.LBB56_203
# %bb.185:                              # %bb3.i178
	movq	%r14, %rdi
	jmp	.LBB56_192
.LBB56_190:                             # %bb3.i322
	testq	%rax, %rax
	je	.LBB56_203
# %bb.191:                              # %bb3.i322
	movq	%r13, %rdi
.LBB56_192:                             # %bb3.i178
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_202
# %bb.193:                              # %common.ret
	callq	cinch_drop_slow@PLT
	movq	%r12, %rax
	jmp	.LBB56_206
.LBB56_194:                             # %bb3.i370
	testq	%rax, %rax
	je	.LBB56_205
# %bb.195:                              # %bb3.i370
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_204
# %bb.196:                              # %bb5.i371
	callq	cinch_drop_slow@PLT
	jmp	.LBB56_205
.LBB56_197:                             # %bb3.i418
	testq	%rax, %rax
	je	.LBB56_207
# %bb.198:                              # %bb3.i418
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB56_208
# %bb.199:                              # %bb5.i419
	callq	cinch_drop_slow@PLT
	movq	%rbx, %rax
	jmp	.LBB56_206
.LBB56_207:
	movq	%rbx, %rax
	jmp	.LBB56_206
.LBB56_200:                             # %bb4.i
	callq	cinch_drop_mt@PLT
	movq	%r14, %rax
	jmp	.LBB56_206
.LBB56_201:                             # %bb4.i132
	callq	cinch_drop_mt@PLT
	movq	%r15, %rax
	jmp	.LBB56_206
.LBB56_202:                             # %common.ret
	callq	cinch_drop_mt@PLT
.LBB56_203:
	movq	%r12, %rax
	jmp	.LBB56_206
.LBB56_204:                             # %bb4.i372
	callq	cinch_drop_mt@PLT
.LBB56_205:                             # %common.ret
	movq	%r13, %rax
	jmp	.LBB56_206
.LBB56_208:                             # %bb4.i420
	callq	cinch_drop_mt@PLT
	movq	%rbx, %rax
	jmp	.LBB56_206
.Lfunc_end56:
	.size	"Data.List.reverse-into@R", .Lfunc_end56-"Data.List.reverse-into@R"
                                        # -- End function
	.globl	"Data.String.string-to-bytes"   # -- Begin function Data.String.string-to-bytes
	.p2align	4
	.type	"Data.String.string-to-bytes",@function
"Data.String.string-to-bytes":          # @Data.String.string-to-bytes
	.cfi_startproc
# %bb.0:                                # %entry
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB57_2
# %bb.1:                                # %bb3
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB57_2:                               # %bb5
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	testq	%rax, %rax
	je	.LBB57_4
# %bb.3:
	xorl	%r14d, %r14d
.LBB57_23:                              # %common.ret
	movq	%r14, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB57_4:                               # %bb6
	.cfi_def_cfa %rbp, 16
	movabsq	$1030792151040, %r12            # imm = 0xF000000000
	movabsq	$274877906944, %r13             # imm = 0x4000000000
	movq	(%rdi), %rbx
	testq	%rbx, %rbx
	je	.LBB57_11
# %bb.5:                                # %bb0.i32
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB57_7
# %bb.6:                                # %bb2.i39
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB57_9
.LBB57_7:                               # %bb3.i
	je	.LBB57_9
# %bb.8:                                # %bb4.i
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB57_9:                               # %bb0.i8
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB57_11
# %bb.10:                               # %bb1.i21
	movq	(%rbx), %rax
	jmp	.LBB57_12
.LBB57_11:                              # %bb2.i16
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB57_12:                              # %cinch.runtime.bytesLen.exit23
	leaq	1(,%rax,2), %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r14
	movabsq	$9223372036854775807, %r15      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rbx, %rbx
	je	.LBB57_22
# %bb.13:                               # %bb0.i
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB57_15
# %bb.14:                               # %cinch.runtime.bytesLen.exit.thread
	movq	(%rbx), %r8
	jmp	.LBB57_16
.LBB57_22:                              # %common.ret.critedge
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%rax, %r15
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%r15, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB57_23
.LBB57_15:                              # %cinch.runtime.bytesLen.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB57_16:                              # %bb0.i43
	andq	%r15, %r8
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB57_18
# %bb.17:                               # %bb2.i52
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB57_23
.LBB57_18:                              # %bb3.i49
	testq	%rax, %rax
	je	.LBB57_23
# %bb.19:                               # %bb3.i49
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB57_21
# %bb.20:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB57_23
.LBB57_21:                              # %bb4.i51
	callq	cinch_drop_mt@PLT
	jmp	.LBB57_23
.Lfunc_end57:
	.size	"Data.String.string-to-bytes", .Lfunc_end57-"Data.String.string-to-bytes"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.utf8-min"          # -- Begin function Data.String.utf8-min
	.p2align	4
	.type	"Data.String.utf8-min",@function
"Data.String.utf8-min":                 # @Data.String.utf8-min
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	testb	$1, %bl
	jne	.LBB58_1
# %bb.4:                                # %cinch.runtime.natCmp.exit22
	movl	$5, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB58_5
# %bb.7:                                # %cinch.runtime.natCmp.exit12
	movl	$7, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB58_8
# %bb.9:                                # %bb1.i
	movl	$9, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jmp	.LBB58_10
.LBB58_1:                               # %bb0.i16
	andq	$-2, %rbx
	movl	$257, %eax                      # imm = 0x101
	cmpq	$4, %rbx
	je	.LBB58_6
# %bb.2:                                # %bb0.i6
	movl	$4097, %eax                     # imm = 0x1001
	cmpq	$6, %rbx
	je	.LBB58_6
# %bb.3:                                # %bb0.i
	cmpq	$8, %rbx
.LBB58_10:                              # %cinch.runtime.natCmp.exit
	sete	%al
	movzbl	%al, %eax
	shll	$17, %eax
	incq	%rax
	jmp	.LBB58_6
.LBB58_5:
	movl	$257, %eax                      # imm = 0x101
	jmp	.LBB58_6
.LBB58_8:
	movl	$4097, %eax                     # imm = 0x1001
.LBB58_6:                               # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end58:
	.size	"Data.String.utf8-min", .Lfunc_end58-"Data.String.utf8-min"
                                        # -- End function
	.globl	"Data.String.utf8-cont"         # -- Begin function Data.String.utf8-cont
	.p2align	4
	.type	"Data.String.utf8-cont",@function
"Data.String.utf8-cont":                # @Data.String.utf8-cont
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
	movq	%rcx, %r12
	movq	%rdx, %r14
	movq	%rsi, %r15
	movq	%rdi, %r13
	movabsq	$9223372036854775680, %rax      # imm = 0x7FFFFFFFFFFFFF80
	addq	$64, %rax
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movq	%rdi, -56(%rbp)                 # 8-byte Spill
	jmp	.LBB59_3
	.p2align	4
.LBB59_1:                               # %bb0.i
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%r13, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB59_48
# %bb.2:                                # %cinch.runtime.natAdd.exit.thread
                                        #   in Loop: Header=BB59_3 Depth=1
	leaq	1(,%rax,2), %r12
	movq	-56(%rbp), %r13                 # 8-byte Reload
.LBB59_3:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	testb	$1, %r14b
	jne	.LBB59_8
# %bb.4:                                # %cinch.runtime.natCmp.exit92
                                        #   in Loop: Header=BB59_3 Depth=1
	movl	$1, %esi
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jne	.LBB59_9
	jmp	.LBB59_5
	.p2align	4
.LBB59_8:                               # %bb0.i86
                                        #   in Loop: Header=BB59_3 Depth=1
	cmpq	$2, %r14
	jb	.LBB59_5
.LBB59_9:                               # %bb2
                                        #   in Loop: Header=BB59_3 Depth=1
	testb	$1, %r15b
	jne	.LBB59_11
# %bb.10:                               # %bb1.i103
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r15, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
	jmp	.LBB59_12
	.p2align	4
.LBB59_11:                              # %bb0.i94
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r15, %rsi
	shrq	%rsi
.LBB59_12:                              # %bb2.i
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r12, -48(%rbp)                 # 8-byte Spill
	testq	%r13, %r13
	je	.LBB59_16
# %bb.13:                               # %bb3.i97
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB59_16
# %bb.14:                               # %bb4.i
                                        #   in Loop: Header=BB59_3 Depth=1
	cmpq	(%r13), %rsi
	jae	.LBB59_16
# %bb.15:                               # %bb7.i
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	8(%r13), %rax
	movzbl	(%rax,%rsi), %r13d
	jmp	.LBB59_17
	.p2align	4
.LBB59_16:                              # %bb5.i
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r13, %rdi
	callq	cinch_bytes_get@PLT
	movq	%rax, %r13
.LBB59_17:                              # %cinch.runtime.natCmp.exit82
                                        #   in Loop: Header=BB59_3 Depth=1
	movabsq	$9223372036854775680, %rax      # imm = 0x7FFFFFFFFFFFFF80
	testq	%rax, %r13
	je	.LBB59_57
# %bb.18:                               # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r13, %rax
	andq	-64(%rbp), %rax                 # 8-byte Folded Reload
	cmpq	$192, %rax
	jae	.LBB59_57
# %bb.19:                               # %bb6
                                        #   in Loop: Header=BB59_3 Depth=1
	testb	$1, %r15b
	je	.LBB59_22
# %bb.20:                               # %bb0.i26
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%r15, %rax
	shrq	%rax
	incq	%rax
	js	.LBB59_22
# %bb.21:                               # %cinch.runtime.natAdd.exit36.thread
                                        #   in Loop: Header=BB59_3 Depth=1
	leaq	1(,%rax,2), %r15
	jmp	.LBB59_31
	.p2align	4
.LBB59_22:                              # %cinch.runtime.natAdd.exit36
                                        #   in Loop: Header=BB59_3 Depth=1
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	testq	%r15, %r15
	sete	%cl
	movl	%r15d, %edx
	orb	%cl, %dl
	testb	$1, %dl
	je	.LBB59_24
.LBB59_23:                              #   in Loop: Header=BB59_3 Depth=1
	movq	%rax, %r15
	jmp	.LBB59_31
	.p2align	4
.LBB59_24:                              # %bb0.i284
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB59_26
# %bb.25:                               # %bb2.i294
                                        #   in Loop: Header=BB59_3 Depth=1
	decq	%rdx
	movq	%rdx, -8(%r15)
	movq	%rax, %r15
	jmp	.LBB59_31
.LBB59_26:                              # %bb3.i290
                                        #   in Loop: Header=BB59_3 Depth=1
	testq	%rcx, %rcx
	je	.LBB59_23
# %bb.27:                               # %bb3.i290
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rax, %rbx
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB59_29
# %bb.28:                               # %bb5.i291
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_30
.LBB59_29:                              # %bb4.i293
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_mt@PLT
.LBB59_30:                              # %cinch.rc.drop.tobj.exit296
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rbx, %r15
	.p2align	4
.LBB59_31:                              # %cinch.rc.drop.tobj.exit296
                                        #   in Loop: Header=BB59_3 Depth=1
	testb	$1, %r14b
	sete	%al
	cmpq	$2, %r14
	setb	%cl
	orb	%al, %cl
	je	.LBB59_34
# %bb.32:                               # %cinch.runtime.natSub.exit63
                                        #   in Loop: Header=BB59_3 Depth=1
	movl	$3, %esi
	movq	%r14, %rdi
	callq	cinch_nat_sub@PLT
	testq	%r14, %r14
	sete	%cl
	movl	%r14d, %edx
	orb	%cl, %dl
	testb	$1, %dl
	je	.LBB59_38
.LBB59_33:                              #   in Loop: Header=BB59_3 Depth=1
	movq	%rax, %r14
	jmp	.LBB59_35
	.p2align	4
.LBB59_34:                              # %cinch.runtime.natSub.exit63.thread
                                        #   in Loop: Header=BB59_3 Depth=1
	addq	$-2, %r14
.LBB59_35:                              # %cinch.rc.drop.tobj.exit313
                                        #   in Loop: Header=BB59_3 Depth=1
	movl	$129, %esi
	movq	-48(%rbp), %r12                 # 8-byte Reload
	movq	%r12, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %rbx
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB59_47
# %bb.36:                               # %bb0.i319
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_40
# %bb.37:                               # %bb2.i329
                                        #   in Loop: Header=BB59_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB59_47
	.p2align	4
.LBB59_38:                              # %bb0.i301
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB59_43
# %bb.39:                               # %bb2.i311
                                        #   in Loop: Header=BB59_3 Depth=1
	decq	%rdx
	movq	%rdx, -8(%r14)
	movq	%rax, %r14
	jmp	.LBB59_35
	.p2align	4
.LBB59_40:                              # %bb3.i325
                                        #   in Loop: Header=BB59_3 Depth=1
	testq	%rax, %rax
	je	.LBB59_47
# %bb.41:                               # %bb3.i325
                                        #   in Loop: Header=BB59_3 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_46
# %bb.42:                               # %bb5.i326
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_47
.LBB59_43:                              # %bb3.i307
                                        #   in Loop: Header=BB59_3 Depth=1
	testq	%rcx, %rcx
	je	.LBB59_33
# %bb.44:                               # %bb3.i307
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rax, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB59_55
# %bb.45:                               # %bb5.i308
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_56
.LBB59_46:                              # %bb4.i328
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB59_47:                              # %cinch.runtime.natSub.exit
                                        #   in Loop: Header=BB59_3 Depth=1
	addq	%r13, %r13
	addq	$-255, %r13
	testb	$1, %bl
	jne	.LBB59_1
.LBB59_48:                              # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rbx, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	movq	-56(%rbp), %r13                 # 8-byte Reload
	jne	.LBB59_3
# %bb.49:                               # %bb0.i352
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_51
# %bb.50:                               # %bb2.i362
                                        #   in Loop: Header=BB59_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB59_3
.LBB59_51:                              # %bb3.i358
                                        #   in Loop: Header=BB59_3 Depth=1
	testq	%rax, %rax
	je	.LBB59_3
# %bb.52:                               # %bb3.i358
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_54
# %bb.53:                               # %bb5.i359
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_3
.LBB59_54:                              # %bb4.i361
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB59_3
.LBB59_55:                              # %bb4.i310
                                        #   in Loop: Header=BB59_3 Depth=1
	callq	cinch_drop_mt@PLT
.LBB59_56:                              # %cinch.rc.drop.tobj.exit313
                                        #   in Loop: Header=BB59_3 Depth=1
	movq	%rbx, %r14
	jmp	.LBB59_35
.LBB59_57:                              # %bb3
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	movq	-48(%rbp), %r12                 # 8-byte Reload
	jne	.LBB59_77
# %bb.58:                               # %bb0.i148
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_61
# %bb.59:                               # %bb2.i158
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB59_77
.LBB59_5:                               # %bb1
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB59_68
# %bb.6:                                # %bb0.i111
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_64
# %bb.7:                                # %bb2.i121
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB59_68
.LBB59_61:                              # %bb3.i154
	testq	%rax, %rax
	je	.LBB59_77
# %bb.62:                               # %bb3.i154
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_76
# %bb.63:                               # %bb5.i155
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_77
.LBB59_64:                              # %bb3.i117
	testq	%rax, %rax
	je	.LBB59_68
# %bb.65:                               # %bb3.i117
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_67
# %bb.66:                               # %bb5.i118
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_68
.LBB59_76:                              # %bb4.i157
	callq	cinch_drop_mt@PLT
.LBB59_77:                              # %cinch.rc.drop.tobj.exit160
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB59_84
# %bb.78:                               # %bb0.i165
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_79
# %bb.82:                               # %bb2.i175
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB59_84
.LBB59_79:                              # %bb3.i171
	testq	%rax, %rax
	je	.LBB59_84
# %bb.80:                               # %bb3.i171
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_83
# %bb.81:                               # %bb5.i172
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_84
.LBB59_83:                              # %bb4.i174
	callq	cinch_drop_mt@PLT
.LBB59_84:                              # %cinch.rc.drop.tobj.exit177
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	testb	$1, %cl
	jne	.LBB59_92
# %bb.85:                               # %bb0.i182
	movq	-8(%r12), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB59_87
# %bb.86:                               # %bb2.i192
	decq	%rdx
	movq	%rdx, -8(%r12)
	jmp	.LBB59_92
.LBB59_87:                              # %bb3.i188
	testq	%rcx, %rcx
	je	.LBB59_92
# %bb.88:                               # %bb3.i188
	movq	%rax, %rbx
	movq	%r12, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB59_90
# %bb.89:                               # %bb5.i189
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_91
.LBB59_90:                              # %bb4.i191
	callq	cinch_drop_mt@PLT
.LBB59_91:                              # %common.ret
	movq	%rbx, %rax
	jmp	.LBB59_92
.LBB59_67:                              # %bb4.i120
	callq	cinch_drop_mt@PLT
.LBB59_68:                              # %cinch.rc.drop.tobj.exit
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB59_75
# %bb.69:                               # %bb0.i127
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB59_71
# %bb.70:                               # %bb2.i137
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB59_75
.LBB59_71:                              # %bb3.i133
	testq	%rax, %rax
	je	.LBB59_75
# %bb.72:                               # %bb3.i133
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB59_74
# %bb.73:                               # %bb5.i134
	callq	cinch_drop_slow@PLT
	jmp	.LBB59_75
.LBB59_74:                              # %bb4.i136
	callq	cinch_drop_mt@PLT
.LBB59_75:                              # %cinch.rc.drop.tobj.exit139
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%r12, (%rax)
.LBB59_92:                              # %common.ret
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end59:
	.size	"Data.String.utf8-cont", .Lfunc_end59-"Data.String.utf8-cont"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.utf8-lead-payload" # -- Begin function Data.String.utf8-lead-payload
	.p2align	4
	.type	"Data.String.utf8-lead-payload",@function
"Data.String.utf8-lead-payload":        # @Data.String.utf8-lead-payload
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	movq	%rsi, %r14
	movq	%rdi, %rbx
	testb	$1, %r14b
	jne	.LBB60_1
# %bb.6:                                # %cinch.runtime.natCmp.exit44
	movl	$5, %esi
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB60_7
# %bb.11:                               # %cinch.runtime.natCmp.exit
	movl	$7, %esi
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB60_12
.LBB60_3:                               # %bb4
	testb	$1, %bl
	jne	.LBB60_4
# %bb.15:                               # %bb1.i
	movl	$17, %esi
	jmp	.LBB60_10
.LBB60_1:                               # %bb0.i38
	andq	$-2, %r14
	cmpq	$4, %r14
	jne	.LBB60_2
.LBB60_7:                               # %bb1
	testb	$1, %bl
	jne	.LBB60_8
# %bb.9:                                # %bb1.i19
	movl	$65, %esi
	jmp	.LBB60_10
.LBB60_2:                               # %bb0.i30
	cmpq	$6, %r14
	jne	.LBB60_3
.LBB60_12:                              # %bb3
	testb	$1, %bl
	jne	.LBB60_13
# %bb.14:                               # %bb1.i7
	movl	$33, %esi
.LBB60_10:                              # %bb1.i19
	movq	%rbx, %rdi
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_rem@PLT               # TAILCALL
.LBB60_8:                               # %bb0.i22
	.cfi_def_cfa %rbp, 16
	andl	$63, %ebx
	jmp	.LBB60_5
.LBB60_4:                               # %bb0.i
	andl	$15, %ebx
	jmp	.LBB60_5
.LBB60_13:                              # %bb0.i10
	andl	$31, %ebx
.LBB60_5:                               # %common.ret
	movq	%rbx, %rax
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end60:
	.size	"Data.String.utf8-lead-payload", .Lfunc_end60-"Data.String.utf8-lead-payload"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.utf8-at"           # -- Begin function Data.String.utf8-at
	.p2align	4
	.type	"Data.String.utf8-at",@function
"Data.String.utf8-at":                  # @Data.String.utf8-at
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %r15
	movq	%rdi, %rbx
	testq	%rdi, %rdi
	je	.LBB61_5
# %bb.1:                                # %bb0.i196
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB61_5
# %bb.2:                                # %bb1.i210
	movq	(%rbx), %rax
	testb	$1, %r15b
	je	.LBB61_6
.LBB61_3:                               # %bb0.i171
	movq	%r15, %rsi
	shrq	%rsi
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	%rax, %rsi
	jae	.LBB61_7
# %bb.4:                                # %bb2.i217
	testq	%rbx, %rbx
	jne	.LBB61_10
	jmp	.LBB61_13
.LBB61_5:                               # %bb2.i204
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %r15b
	jne	.LBB61_3
.LBB61_6:                               # %cinch.runtime.natCmp.exit178
	leaq	1(,%rax,2), %rsi
	movq	%r15, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB61_9
.LBB61_7:
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
.LBB61_8:                               # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB61_9:                               # %bb1.i227
	.cfi_def_cfa %rbp, 16
	movq	%r15, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
	testq	%rbx, %rbx
	je	.LBB61_13
.LBB61_10:                              # %bb3.i220
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB61_13
# %bb.11:                               # %bb4.i
	cmpq	(%rbx), %rsi
	jae	.LBB61_13
# %bb.12:                               # %bb7.i
	movq	8(%rbx), %rax
	movzbl	(%rax,%rsi), %r12d
	jmp	.LBB61_14
.LBB61_13:                              # %bb5.i
	movq	%rbx, %rdi
	callq	cinch_bytes_get@PLT
	movq	%rax, %r12
.LBB61_14:                              # %cinch.runtime.natCmp.exit167
	movabsq	$9223372036854775807, %r14      # imm = 0x7FFFFFFFFFFFFFFF
	leaq	-127(%r14), %rax
	testq	%rax, %r12
	je	.LBB61_22
# %bb.15:                               # %cinch.runtime.natCmp.exit157
	leaq	-63(%r14), %rcx
	andq	%r12, %rcx
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	cmpq	$192, %rcx
	jb	.LBB61_8
# %bb.16:                               # %cinch.runtime.natCmp.exit147
	leaq	-31(%r14), %rcx
	andq	%r12, %rcx
	movl	$5, %esi
	cmpq	$224, %rcx
	jb	.LBB61_19
# %bb.17:                               # %cinch.runtime.natCmp.exit137
	leaq	-15(%r14), %rcx
	andq	%r12, %rcx
	movl	$7, %esi
	cmpq	$240, %rcx
	jb	.LBB61_19
# %bb.18:                               # %cinch.runtime.natCmp.exit127
	leaq	-7(%r14), %rcx
	andq	%r12, %rcx
	movl	$9, %esi
	cmpq	$247, %rcx
	ja	.LBB61_8
.LBB61_19:                              # %bb17
	testb	$1, %r15b
	movq	%rsi, -48(%rbp)                 # 8-byte Spill
	je	.LBB61_28
# %bb.20:                               # %bb0.i45
	movl	%esi, %ecx
	shrl	%ecx
	movq	%r15, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB61_28
# %bb.21:                               # %bb3.i49
	leaq	1(,%rax,2), %r13
	testq	%rbx, %rbx
	jne	.LBB61_29
	jmp	.LBB61_32
.LBB61_22:                              # %bb1.i315
	andl	$127, %r12d
	movl	$8, %edi
	movl	$1, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%r12, (%rax)
	movq	-8(%rax), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB61_24
# %bb.23:                               # %bb22
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB61_24:                              # %bb24
	cmpq	$1, %rax
	je	.LBB61_36
# %bb.25:                               # %bb24
	testq	%rax, %rax
	jne	.LBB61_40
# %bb.26:                               # %bb0.i351
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB61_41
# %bb.27:                               # %bb2.i361
	decq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB61_7
.LBB61_28:                              # %bb1.i53
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r13
	testq	%rbx, %rbx
	je	.LBB61_32
.LBB61_29:                              # %bb0.i185
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB61_32
# %bb.30:                               # %bb1.i193
	movq	(%rbx), %rax
	testb	$1, %r13b
	je	.LBB61_33
.LBB61_31:                              # %cinch.runtime.natCmp.exit89.thread
	shrq	%r13
	andq	%r14, %rax
	cmpq	%rax, %r13
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	ja	.LBB61_8
	jmp	.LBB61_52
.LBB61_32:                              # %bb2.i
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %r13b
	jne	.LBB61_31
.LBB61_33:                              # %cinch.runtime.natCmp.exit89
	leaq	1(,%rax,2), %rsi
	movq	%r13, %rdi
	callq	cinch_nat_cmp@PLT
	movq	%rax, %r14
	testq	%r13, %r13
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	je	.LBB61_48
# %bb.34:                               # %bb0.i373
	movq	-8(%r13), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB61_45
# %bb.35:                               # %bb2.i383
	decq	%rdx
	movq	%rdx, -8(%r13)
	jmp	.LBB61_48
.LBB61_36:                              # %cinch.rc.unique.exit
	movq	(%rdi), %r14
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	movq	%rdi, %rax
	cmpq	$1, %rcx
	je	.LBB61_38
# %bb.37:                               # %bb28
	callq	cinch.rc.drop.obj
	xorl	%eax, %eax
.LBB61_38:                              # %bb29
	movl	$16, %edi
	movl	$512, %esi                      # imm = 0x200
	movq	%rax, %r15
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r14, (%rax)
	movq	$3, 8(%rax)
	testq	%r15, %r15
	je	.LBB61_44
# %bb.39:                               # %bb31
	movq	%r15, %rax
	movabsq	$282608848076801, %rcx          # imm = 0x1010800000001
	movq	%rcx, -8(%r15)
	movq	%rbx, (%r15)
	jmp	.LBB61_8
.LBB61_41:                              # %bb3.i357
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	testq	%rcx, %rcx
	je	.LBB61_8
# %bb.42:                               # %bb3.i357
	movq	%rax, %rbx
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB61_49
# %bb.43:                               # %bb5.i358
	callq	cinch_drop_slow@PLT
	movq	%rbx, %rax
	jmp	.LBB61_8
.LBB61_44:                              # %bb30
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	jmp	.LBB61_8
.LBB61_45:                              # %bb3.i379
	testq	%rcx, %rcx
	je	.LBB61_48
# %bb.46:                               # %bb3.i379
	movq	%r13, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB61_50
# %bb.47:                               # %bb5.i380
	callq	cinch_drop_slow@PLT
	jmp	.LBB61_51
.LBB61_48:                              # %cinch.rc.drop.tobj.exit
	testq	%r14, %r14
	jg	.LBB61_8
	jmp	.LBB61_52
.LBB61_49:                              # %bb4.i360
	callq	cinch_drop_mt@PLT
	movq	%rbx, %rax
	jmp	.LBB61_8
.LBB61_50:                              # %bb4.i382
	callq	cinch_drop_mt@PLT
.LBB61_51:                              # %bb5.i380
	testq	%r14, %r14
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	jg	.LBB61_8
.LBB61_52:                              # %bb36
	leaq	1(,%r12,2), %r12
	testb	$1, %r15b
	je	.LBB61_55
# %bb.53:                               # %bb0.i
	movq	%r15, %rax
	shrq	%rax
	incq	%rax
	js	.LBB61_55
# %bb.54:                               # %bb3.i
	leaq	1(,%rax,2), %r14
	jmp	.LBB61_56
.LBB61_55:                              # %bb1.i
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
.LBB61_56:                              # %bb3.i62
	movq	-48(%rbp), %rsi                 # 8-byte Reload
	leaq	-2(%rsi), %r15
	movq	%r12, %rdi
	callq	"Data.String.utf8-lead-payload"@PLT
	movq	%rax, %r13
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	movq	%r13, %rcx
	callq	"Data.String.utf8-cont"@PLT
	movq	%rax, %r15
	movq	-8(%rax), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB61_58
# %bb.57:                               # %bb38
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB61_58:                              # %bb40
	cmpq	$1, %rax
	movq	-48(%rbp), %r12                 # 8-byte Reload
	je	.LBB61_61
# %bb.59:                               # %bb40
	testq	%rax, %rax
	jne	.LBB61_40
# %bb.60:                               # %bb41
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
	jmp	.LBB61_7
.LBB61_40:
	xorl	%eax, %eax
	jmp	.LBB61_8
.LBB61_61:                              # %bb42
	movq	(%r15), %rbx
	testq	%r15, %r15
	je	.LBB61_65
# %bb.62:                               # %cinch.rc.unique.exit443
	movq	-8(%r15), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB61_65
# %bb.63:                               # %bb43
	movq	%r15, %rdi
	callq	cinch_free@PLT
	jmp	.LBB61_66
.LBB61_65:                              # %bb44
	movq	%rbx, %rdi
	callq	cinch.rc.dup.tobj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
.LBB61_66:                              # %bb45
	movq	%r12, %rdi
	callq	"Data.String.utf8-min"@PLT
	movl	%eax, %ecx
	andl	%ebx, %ecx
	testb	$1, %cl
	je	.LBB61_69
# %bb.67:                               # %bb0.i72
	movq	%rbx, %r14
	shrq	%r14
	movq	%rax, %r15
	shrq	%r15
	movq	%rax, %rdi
	callq	cinch.rc.drop.tobj
	cmpq	%r15, %r14
	jae	.LBB61_70
.LBB61_68:                              # %bb46
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	movq	%rbx, %rdi
	callq	cinch.rc.drop.tobj
	jmp	.LBB61_7
.LBB61_69:                              # %cinch.runtime.natCmp.exit
	movq	%rbx, %rdi
	movq	%rax, %rsi
	movq	%rax, %r14
	callq	cinch_nat_cmp@PLT
	movq	%rax, %r15
	movq	%r14, %rdi
	callq	cinch.rc.drop.tobj
	testq	%r15, %r15
	js	.LBB61_68
.LBB61_70:                              # %bb47
	movq	%rbx, %rdi
	movq	%r12, %rsi
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Data.String.utf8-char"@PLT     # TAILCALL
.Lfunc_end61:
	.size	"Data.String.utf8-at", .Lfunc_end61-"Data.String.utf8-at"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.utf8-char"         # -- Begin function Data.String.utf8-char
	.p2align	4
	.type	"Data.String.utf8-char",@function
"Data.String.utf8-char":                # @Data.String.utf8-char
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %rbx
	movq	%rdi, %r15
	testb	$1, %r15b
	jne	.LBB62_1
# %bb.15:                               # %cinch.runtime.natCmp.exit
	movl	$2228223, %esi                  # imm = 0x21FFFF
	movq	%r15, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB62_4
# %bb.16:                               # %bb3
	leaq	"Data.Core.None.blk#h"+8(%rip), %r14
	testq	%r15, %r15
	je	.LBB62_23
# %bb.17:                               # %bb0.i34
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB62_19
# %bb.18:                               # %bb2.i44
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB62_23
.LBB62_1:                               # %bb0.i
	cmpq	$2228224, %r15                  # imm = 0x220000
	jae	.LBB62_2
# %bb.3:                                # %cinch.runtime.natToI64.exit.thread
	shrq	%r15
	movq	%r15, %rax
	jmp	.LBB62_12
.LBB62_2:
	leaq	"Data.Core.None.blk#h"+8(%rip), %r14
	jmp	.LBB62_23
.LBB62_4:                               # %cinch.runtime.natToI64.exit
	movq	%r15, %rdi
	callq	cinch_nat_to_i64@PLT
	testq	%r15, %r15
	je	.LBB62_12
# %bb.5:                                # %bb0.i18
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB62_7
# %bb.6:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB62_12
.LBB62_19:                              # %bb3.i40
	testq	%rax, %rax
	je	.LBB62_23
# %bb.20:                               # %bb3.i40
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB62_22
# %bb.21:                               # %bb5.i41
	callq	cinch_drop_slow@PLT
	jmp	.LBB62_23
.LBB62_7:                               # %bb3.i
	testq	%rcx, %rcx
	je	.LBB62_12
# %bb.8:                                # %bb3.i
	movq	%rax, %r14
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB62_10
# %bb.9:                                # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB62_11
.LBB62_22:                              # %bb4.i43
	callq	cinch_drop_mt@PLT
	jmp	.LBB62_23
.LBB62_10:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB62_11:                              # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
.LBB62_12:                              # %cinch.rc.drop.tobj.exit
	movl	%eax, %r15d
	cmpq	$1114112, %r15                  # imm = 0x110000
	setae	%cl
	andl	$-2048, %eax                    # imm = 0xF800
	cmpl	$55296, %eax                    # imm = 0xD800
	sete	%al
	orb	%cl, %al
	je	.LBB62_14
# %bb.13:
	leaq	"cinch.prim.u32ToChar.blk#h"+8(%rip), %r14
	jmp	.LBB62_23
.LBB62_14:                              # %bb1.i27
	movl	$8, %edi
	movl	$1, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r15, (%rax)
.LBB62_23:                              # %bb1
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB62_25
# %bb.24:                               # %bb5
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB62_25:                              # %bb7
	testq	%rax, %rax
	je	.LBB62_29
# %bb.26:                               # %bb7
	cmpq	$1, %rax
	jne	.LBB62_27
# %bb.42:                               # %cinch.rc.unique.exit
	movq	(%r14), %r12
	movq	-8(%r14), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB62_44
# %bb.43:
	movq	%r14, %r15
	jmp	.LBB62_51
.LBB62_29:                              # %bb8
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB62_36
# %bb.30:                               # %bb0.i51
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB62_32
# %bb.31:                               # %bb2.i61
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB62_36
.LBB62_27:
	xorl	%r15d, %r15d
	jmp	.LBB62_28
.LBB62_44:                              # %bb0.i81
	movq	-8(%r14), %rax
	movq	%rax, %r15
	shlq	$29, %r15
	cmpq	$536870913, %r15                # imm = 0x20000001
	jl	.LBB62_46
# %bb.45:                               # %bb2.i91
	decq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB62_50
.LBB62_46:                              # %bb3.i87
	testq	%r15, %r15
	je	.LBB62_51
# %bb.47:                               # %bb3.i87
	movq	%r14, %rdi
	cmpq	$536870912, %r15                # imm = 0x20000000
	jne	.LBB62_49
# %bb.48:                               # %bb5.i88
	callq	cinch_drop_slow@PLT
	jmp	.LBB62_50
.LBB62_32:                              # %bb3.i57
	testq	%rax, %rax
	je	.LBB62_36
# %bb.33:                               # %bb3.i57
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB62_35
# %bb.34:                               # %bb5.i58
	callq	cinch_drop_slow@PLT
	jmp	.LBB62_36
.LBB62_49:                              # %bb4.i90
	callq	cinch_drop_mt@PLT
.LBB62_50:                              # %bb12
	xorl	%r15d, %r15d
.LBB62_51:                              # %bb12
	movl	$16, %edi
	movl	$512, %esi                      # imm = 0x200
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r12, (%rax)
	movq	%rbx, 8(%rax)
	testq	%r15, %r15
	je	.LBB62_52
# %bb.53:                               # %bb14
	movabsq	$282608848076801, %rax          # imm = 0x1010800000001
	movq	%rax, -8(%r15)
	movq	%r14, (%r15)
	jmp	.LBB62_28
.LBB62_52:                              # %bb13
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%r14, (%rax)
	jmp	.LBB62_28
.LBB62_35:                              # %bb4.i60
	callq	cinch_drop_mt@PLT
.LBB62_36:                              # %bb0.i65
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB62_38
# %bb.37:                               # %bb2.i75
	decq	%rcx
	movq	%rcx, -8(%r14)
	leaq	"Data.Core.None.blk#h"+8(%rip), %r15
	jmp	.LBB62_28
.LBB62_38:                              # %bb3.i71
	leaq	"Data.Core.None.blk#h"+8(%rip), %r15
	testq	%rax, %rax
	je	.LBB62_28
# %bb.39:                               # %bb3.i71
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB62_41
# %bb.40:                               # %bb5.i72
	callq	cinch_drop_slow@PLT
	jmp	.LBB62_28
.LBB62_41:                              # %bb4.i74
	callq	cinch_drop_mt@PLT
.LBB62_28:                              # %common.ret
	movq	%r15, %rax
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end62:
	.size	"Data.String.utf8-char", .Lfunc_end62-"Data.String.utf8-char"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.utf8-valid-from"   # -- Begin function Data.String.utf8-valid-from
	.p2align	4
	.type	"Data.String.utf8-valid-from",@function
"Data.String.utf8-valid-from":          # @Data.String.utf8-valid-from
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
	movq	%rsi, %r13
	movq	%rdi, %r14
	jmp	.LBB63_2
.LBB63_1:                               # %bb2.i1416
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	movq	%r12, %r13
	.p2align	4
.LBB63_2:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	testq	%r14, %r14
	je	.LBB63_5
# %bb.3:                                # %bb0.i499
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_5
# %bb.4:                                # %bb1.i513
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	jmp	.LBB63_6
	.p2align	4
.LBB63_5:                               # %bb2.i507
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB63_6:                               # %cinch.runtime.bytesLen.exit515
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %r15
	andq	$1, %r15
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	jne	.LBB63_14
# %bb.7:                                # %cinch.runtime.natCmp.exit357
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%r13, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB63_8
	jmp	.LBB63_300
	.p2align	4
.LBB63_14:                              # %bb0.i350
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rcx
	shrq	%rcx
	andq	%r12, %rax
	cmpq	%rax, %rcx
	jae	.LBB63_329
# %bb.15:                               # %bb0.i180
                                        #   in Loop: Header=BB63_2 Depth=1
	addq	$8, %rcx
	js	.LBB63_8
# %bb.16:                               # %bb3.i184
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rcx,2), %rbx
	testq	%r14, %r14
	jne	.LBB63_17
	jmp	.LBB63_9
	.p2align	4
.LBB63_8:                               # %bb1.i188
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$17, %esi
	movq	%r13, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB63_9
.LBB63_17:                              # %bb0.i481
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_9
# %bb.18:                               # %bb1.i495
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	testb	$1, %bl
	je	.LBB63_10
.LBB63_19:                              # %cinch.runtime.natCmp.exit346.thread
                                        #   in Loop: Header=BB63_2 Depth=1
	shrq	%rbx
	andq	%r12, %rax
	cmpq	%rax, %rbx
	jbe	.LBB63_25
	jmp	.LBB63_37
	.p2align	4
.LBB63_9:                               # %bb2.i489
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %bl
	jne	.LBB63_19
.LBB63_10:                              # %cinch.runtime.natCmp.exit346
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rbx, %rbx
	je	.LBB63_13
# %bb.11:                               # %bb0.i570
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_20
# %bb.12:                               # %bb2.i580
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB63_13
.LBB63_20:                              # %bb3.i576
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB63_13
# %bb.21:                               # %bb3.i576
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %r12
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB63_23
# %bb.22:                               # %bb5.i577
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_24
	.p2align	4
.LBB63_13:                              # %cinch.rc.drop.tobj.exit582
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	jg	.LBB63_37
	jmp	.LBB63_25
.LBB63_23:                              # %bb4.i579
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_24:                              # %bb5.i577
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r12, %r12
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	jg	.LBB63_37
	.p2align	4
.LBB63_25:                              # %bb4
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r14, %r14
	je	.LBB63_31
# %bb.26:                               # %bb0.i602
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_31
# %bb.27:                               # %bb1.i611
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$0, (%r14)
	movl	$1, %ebx
	je	.LBB63_29
# %bb.28:                               # %bb1.i611
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	8(%r14), %rbx
	.p2align	4
.LBB63_29:                              # %cinch.runtime.bytesData.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	jne	.LBB63_32
.LBB63_30:                              # %bb1.i376
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	callq	cinch_nat_to_i64@PLT
	jmp	.LBB63_33
	.p2align	4
.LBB63_31:                              # %bb2.i607
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_data@PLT
	movq	%rax, %rbx
	testq	%r15, %r15
	je	.LBB63_30
.LBB63_32:                              # %bb0.i373
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rax
	shrq	%rax
.LBB63_33:                              # %cinch.runtime.natToI64.exit378
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%rbx,%rax), %rbx
	testq	%r14, %r14
	je	.LBB63_35
# %bb.34:                               # %bb0.i463
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	je	.LBB63_36
.LBB63_35:                              # %bb2.i471
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB63_36:                              # %cinch.runtime.bytesLen.exit479
                                        #   in Loop: Header=BB63_2 Depth=1
	movabsq	$-9187201950435737472, %rax     # imm = 0x8080808080808080
	testq	%rax, %rbx
	je	.LBB63_63
.LBB63_37:                              # %bb7
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r14, %r14
	je	.LBB63_40
# %bb.38:                               # %bb0.i391
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_40
# %bb.39:                               # %bb1.i405
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	jmp	.LBB63_41
	.p2align	4
.LBB63_40:                              # %bb2.i399
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB63_41:                              # %cinch.runtime.bytesLen.exit407
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	movq	%r13, -56(%rbp)                 # 8-byte Spill
	jne	.LBB63_44
# %bb.42:                               # %cinch.runtime.natCmp.exit313
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%r13, %rdi
	callq	cinch_nat_cmp@PLT
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rax, %rax
	jns	.LBB63_240
# %bb.43:                               # %bb1.i530
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
	jmp	.LBB63_45
	.p2align	4
.LBB63_44:                              # %bb0.i306
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rsi
	shrq	%rsi
	andq	%r12, %rax
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	%rax, %rsi
	jae	.LBB63_240
.LBB63_45:                              # %bb2.i520
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r14, %r14
	je	.LBB63_49
# %bb.46:                               # %bb3.i523
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_49
# %bb.47:                               # %bb4.i
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	(%r14), %rsi
	jae	.LBB63_49
# %bb.48:                               # %bb7.i
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	8(%r14), %rax
	movzbl	(%rax,%rsi), %ebx
	jmp	.LBB63_50
	.p2align	4
.LBB63_49:                              # %bb5.i
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_get@PLT
	movq	%rax, %rbx
.LBB63_50:                              # %cinch.runtime.natCmp.exit302
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	-127(%r12), %rax
	testq	%rax, %rbx
	je	.LBB63_66
# %bb.51:                               # %cinch.runtime.natCmp.exit292
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	-63(%r12), %rax
	andq	%rbx, %rax
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	$192, %rax
	jb	.LBB63_240
# %bb.52:                               # %cinch.runtime.natCmp.exit282
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	-31(%r12), %rax
	andq	%rbx, %rax
	movl	$5, %ecx
	movq	%rcx, -48(%rbp)                 # 8-byte Spill
	cmpq	$224, %rax
	jb	.LBB63_55
# %bb.53:                               # %cinch.runtime.natCmp.exit272
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	-15(%r12), %rax
	andq	%rbx, %rax
	movl	$7, %ecx
	movq	%rcx, -48(%rbp)                 # 8-byte Spill
	cmpq	$240, %rax
	jb	.LBB63_55
# %bb.54:                               # %cinch.runtime.natCmp.exit262
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	-7(%r12), %rax
	andq	%rbx, %rax
	movl	$9, %ecx
	movq	%rcx, -48(%rbp)                 # 8-byte Spill
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	$247, %rax
	ja	.LBB63_240
.LBB63_55:                              # %bb49
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	je	.LBB63_75
# %bb.56:                               # %bb0.i108
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-48(%rbp), %rcx                 # 8-byte Reload
                                        # kill: def $ecx killed $ecx killed $rcx def $rcx
	shrl	%ecx
	movq	%rdi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB63_75
# %bb.57:                               # %bb3.i113
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %r12
	testq	%r14, %r14
	jne	.LBB63_76
	jmp	.LBB63_58
.LBB63_63:                              # %bb6
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	je	.LBB63_72
# %bb.64:                               # %bb0.i166
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rax
	shrq	%rax
	addq	$8, %rax
	js	.LBB63_72
# %bb.65:                               # %cinch.runtime.natAdd.exit176.thread
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %r12
	testq	%r14, %r14
	jne	.LBB63_89
	jmp	.LBB63_98
.LBB63_66:                              # %bb1.i996
                                        #   in Loop: Header=BB63_2 Depth=1
	andl	$127, %ebx
	movl	$8, %edi
	movl	$1, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB63_68
# %bb.67:                               # %bb54
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB63_68:                              # %bb56
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$1, %rcx
	je	.LBB63_79
# %bb.69:                               # %bb56
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rcx, %rcx
	jne	.LBB63_336
# %bb.70:                               # %bb0.i1033
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_82
# %bb.71:                               # %bb2.i1043
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	jmp	.LBB63_240
.LBB63_72:                              # %cinch.runtime.natAdd.exit176
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$17, %esi
	movq	%r13, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%r13, %r13
	sete	%al
	orb	%al, %r15b
	je	.LBB63_73
.LBB63_88:                              # %cinch.rc.drop.tobj.exit652
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r14, %r14
	je	.LBB63_98
.LBB63_89:                              # %bb0.i445
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_98
# %bb.90:                               # %bb1.i459
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	jmp	.LBB63_99
.LBB63_75:                              # %bb1.i117
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-48(%rbp), %rsi                 # 8-byte Reload
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%r14, %r14
	je	.LBB63_58
.LBB63_76:                              # %bb0.i380
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_58
# %bb.77:                               # %bb1.i388
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	testb	$1, %r12b
	je	.LBB63_59
.LBB63_78:                              # %cinch.runtime.natCmp.exit224.thread
                                        #   in Loop: Header=BB63_2 Depth=1
	shrq	%r12
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	%rax, %r12
	jbe	.LBB63_197
	jmp	.LBB63_240
.LBB63_58:                              # %bb2.i
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %r12b
	jne	.LBB63_78
.LBB63_59:                              # %cinch.runtime.natCmp.exit224
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%r12, %r12
	je	.LBB63_62
# %bb.60:                               # %bb0.i1076
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	cmpq	$536870913, %rdx                # imm = 0x20000001
	jl	.LBB63_94
# %bb.61:                               # %bb2.i1086
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB63_62
.LBB63_79:                              # %cinch.rc.unique.exit1054
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%rax), %r12
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	movq	%rax, %r13
	cmpq	$1, %rcx
	je	.LBB63_192
# %bb.80:                               # %bb0.i1056
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rax), %rcx
	movq	%rcx, %r13
	shlq	$29, %r13
	cmpq	$536870913, %r13                # imm = 0x20000001
	jl	.LBB63_91
# %bb.81:                               # %bb2.i1066
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rax)
	jmp	.LBB63_191
.LBB63_73:                              # %bb0.i640
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_85
# %bb.74:                               # %bb2.i650
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	testq	%r14, %r14
	jne	.LBB63_89
	jmp	.LBB63_98
.LBB63_82:                              # %bb3.i1039
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rcx, %rcx
	je	.LBB63_240
# %bb.83:                               # %bb3.i1039
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB63_216
.LBB63_84:                              # %bb32
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	jmp	.LBB63_240
.LBB63_85:                              # %bb3.i646
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_88
# %bb.86:                               # %bb3.i646
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_97
# %bb.87:                               # %bb5.i647
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_88
.LBB63_91:                              # %bb3.i1062
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r13, %r13
	je	.LBB63_192
# %bb.92:                               # %bb3.i1062
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %r13                # imm = 0x20000000
	jne	.LBB63_190
# %bb.93:                               # %bb5.i1063
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_191
.LBB63_94:                              # %bb3.i1082
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rdx, %rdx
	je	.LBB63_62
# %bb.95:                               # %bb3.i1082
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movq	%r12, %rdi
	cmpq	$536870912, %rdx                # imm = 0x20000000
	jne	.LBB63_195
# %bb.96:                               # %bb5.i1083
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_196
.LBB63_62:                              # %cinch.rc.drop.tobj.exit1088
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rax, %rax
	jg	.LBB63_240
	jmp	.LBB63_197
.LBB63_97:                              # %bb4.i649
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	testq	%r14, %r14
	jne	.LBB63_89
.LBB63_98:                              # %bb2.i453
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB63_99:                              # %cinch.runtime.bytesLen.exit461
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %r13
	andq	$1, %r13
	jne	.LBB63_107
# %bb.100:                              # %cinch.runtime.natCmp.exit335
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB63_101
	jmp	.LBB63_313
.LBB63_107:                             # %bb0.i328
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rcx
	shrq	%rcx
	movabsq	$9223372036854775807, %rdx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rdx, %rax
	cmpq	%rax, %rcx
	jae	.LBB63_329
# %bb.108:                              # %bb0.i152
                                        #   in Loop: Header=BB63_2 Depth=1
	addq	$8, %rcx
	js	.LBB63_101
# %bb.109:                              # %bb3.i156
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rcx,2), %rbx
	testq	%r14, %r14
	jne	.LBB63_110
	jmp	.LBB63_102
.LBB63_101:                             # %bb1.i160
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$17, %esi
	movq	%r12, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB63_102
.LBB63_110:                             # %bb0.i427
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_102
# %bb.111:                              # %bb1.i441
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r14), %rax
	testb	$1, %bl
	je	.LBB63_103
.LBB63_112:                             # %cinch.runtime.natCmp.exit324.thread
                                        #   in Loop: Header=BB63_2 Depth=1
	shrq	%rbx
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	cmpq	%rax, %rbx
	jbe	.LBB63_118
	jmp	.LBB63_130
.LBB63_102:                             # %bb2.i435
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %bl
	jne	.LBB63_112
.LBB63_103:                             # %cinch.runtime.natCmp.exit324
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rsi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rbx, %rbx
	je	.LBB63_106
# %bb.104:                              # %bb0.i692
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_113
# %bb.105:                              # %bb2.i702
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB63_106
.LBB63_113:                             # %bb3.i698
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB63_106
# %bb.114:                              # %bb3.i698
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %r15
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB63_116
# %bb.115:                              # %bb5.i699
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	testq	%r15, %r15
	jg	.LBB63_130
	jmp	.LBB63_118
.LBB63_106:                             # %cinch.rc.drop.tobj.exit704
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	jg	.LBB63_130
.LBB63_118:                             # %bb11
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r14, %r14
	je	.LBB63_124
# %bb.119:                              # %bb0.i724
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB63_124
# %bb.120:                              # %bb1.i733
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$0, (%r14)
	movl	$1, %ebx
	je	.LBB63_122
# %bb.121:                              # %bb1.i733
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	8(%r14), %rbx
.LBB63_122:                             # %cinch.runtime.bytesData.exit740
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r13, %r13
	jne	.LBB63_125
.LBB63_123:                             # %bb1.i368
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	jmp	.LBB63_126
.LBB63_124:                             # %bb2.i729
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_data@PLT
	movq	%rax, %rbx
	testq	%r13, %r13
	je	.LBB63_123
.LBB63_125:                             # %bb0.i365
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rax
	shrq	%rax
.LBB63_126:                             # %cinch.runtime.natToI64.exit370
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%rbx,%rax), %rbx
	testq	%r14, %r14
	je	.LBB63_128
# %bb.127:                              # %bb0.i409
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	je	.LBB63_129
.LBB63_128:                             # %bb2.i417
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB63_129:                             # %cinch.runtime.bytesLen.exit425
                                        #   in Loop: Header=BB63_2 Depth=1
	movabsq	$-9187201950435737472, %rax     # imm = 0x8080808080808080
	testq	%rax, %rbx
	je	.LBB63_139
.LBB63_130:                             # %bb14
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r14, %rdi
	movq	%r12, %rsi
	callq	"Data.String.utf8-at"@PLT
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB63_132
# %bb.131:                              # %bb16
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB63_132:                             # %bb18
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$1, %rcx
	jne	.LBB63_303
# %bb.133:                              # %bb20
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%rax), %r15
	testq	%rax, %rax
	je	.LBB63_136
# %bb.134:                              # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	cmpq	$1, %rcx
	jne	.LBB63_136
# %bb.135:                              # %bb21
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB63_154
.LBB63_136:                             # %bb22
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	je	.LBB63_144
# %bb.137:                              # %bb0.i813
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB63_142
# %bb.138:                              # %bb2.i822
                                        #   in Loop: Header=BB63_2 Depth=1
	incq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB63_144
.LBB63_139:                             # %bb13
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r13, %r13
	je	.LBB63_150
# %bb.140:                              # %bb0.i138
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rax
	shrq	%rax
	addq	$8, %rax
	js	.LBB63_150
# %bb.141:                              # %cinch.runtime.natAdd.exit148.thread
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %r13
	jmp	.LBB63_2
.LBB63_142:                             # %bb3.i819
                                        #   in Loop: Header=BB63_2 Depth=1
	je	.LBB63_144
# %bb.143:                              # %bb4.i820
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r15, %rdi
	movq	%rax, %rbx
	callq	cinch_dup_mt@PLT
	movq	%rbx, %rax
.LBB63_144:                             # %cinch.rc.dup.obj.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_154
# %bb.145:                              # %bb0.i825
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_147
# %bb.146:                              # %bb2.i835
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB63_154
.LBB63_147:                             # %bb3.i831
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rcx, %rcx
	je	.LBB63_154
# %bb.148:                              # %bb3.i831
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB63_153
# %bb.149:                              # %bb5.i832
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_154
.LBB63_150:                             # %cinch.runtime.natAdd.exit148
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$17, %esi
	movq	%r12, %rdi
	callq	cinch_nat_add@PLT
	testq	%r12, %r12
	sete	%cl
	orb	%cl, %r13b
	movq	%rax, %r13
	jne	.LBB63_2
# %bb.151:                              # %bb0.i767
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_217
# %bb.152:                              # %bb2.i777
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB63_2
.LBB63_153:                             # %bb4.i834
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_154:                             # %bb23
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB63_156
# %bb.155:                              # %bb25
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB63_156:                             # %bb27
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	jne	.LBB63_336
# %bb.157:                              # %bb28
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	8(%r15), %rbx
	testq	%r15, %r15
	je	.LBB63_160
# %bb.158:                              # %cinch.rc.unique.exit846
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r15), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB63_160
# %bb.159:                              # %bb29
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r15, %rdi
	callq	cinch_free@PLT
	testl	%ebx, %r13d
	jne	.LBB63_172
	jmp	.LBB63_175
.LBB63_160:                             # %bb30
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB63_165
# %bb.161:                              # %bb0.i851
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB63_163
# %bb.162:                              # %bb2.i860
                                        #   in Loop: Header=BB63_2 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB63_165
.LBB63_163:                             # %bb3.i857
                                        #   in Loop: Header=BB63_2 Depth=1
	je	.LBB63_165
# %bb.164:                              # %bb4.i858
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB63_165:                             # %cinch.rc.dup.tobj.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	je	.LBB63_171
# %bb.166:                              # %bb0.i863
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_168
# %bb.167:                              # %bb2.i873
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	testl	%ebx, %r13d
	jne	.LBB63_172
	jmp	.LBB63_175
.LBB63_168:                             # %bb3.i869
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_171
# %bb.169:                              # %bb3.i869
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_174
# %bb.170:                              # %bb5.i870
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
.LBB63_171:                             # %bb31
                                        #   in Loop: Header=BB63_2 Depth=1
	testl	%ebx, %r13d
	je	.LBB63_175
.LBB63_172:                             # %bb0.i123
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rcx
	shrq	%rcx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB63_175
# %bb.173:                              # %bb3.i128
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %rax
	jmp	.LBB63_176
.LBB63_174:                             # %bb4.i872
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	testl	%ebx, %r13d
	jne	.LBB63_172
.LBB63_175:                             # %bb1.i132
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
.LBB63_176:                             # %cinch.runtime.natAdd.exit134
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r12, %r12
	sete	%cl
	orb	%cl, %r13b
	movq	%rax, %r13
	jne	.LBB63_183
# %bb.177:                              # %bb0.i880
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_179
# %bb.178:                              # %bb2.i890
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB63_183
.LBB63_179:                             # %bb3.i886
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_183
# %bb.180:                              # %bb3.i886
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_182
# %bb.181:                              # %bb5.i887
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_183
.LBB63_182:                             # %bb4.i889
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_183:                             # %cinch.rc.drop.tobj.exit892
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB63_2
# %bb.184:                              # %bb0.i897
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_186
# %bb.185:                              # %bb2.i907
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB63_2
.LBB63_186:                             # %bb3.i903
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_2
# %bb.187:                              # %bb3.i903
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_189
# %bb.188:                              # %bb5.i904
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_2
.LBB63_189:                             # %bb4.i906
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB63_2
.LBB63_116:                             # %bb4.i701
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	testq	%r15, %r15
	jg	.LBB63_130
	jmp	.LBB63_118
.LBB63_190:                             # %bb4.i1065
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_191:                             # %bb61
                                        #   in Loop: Header=BB63_2 Depth=1
	xorl	%r13d, %r13d
.LBB63_192:                             # %bb61
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$16, %edi
	movl	$512, %esi                      # imm = 0x200
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	movq	$3, 8(%rax)
	testq	%r13, %r13
	je	.LBB63_194
# %bb.193:                              # %bb63
                                        #   in Loop: Header=BB63_2 Depth=1
	movabsq	$282608848076801, %rax          # imm = 0x1010800000001
	movq	%rax, -8(%r13)
	movq	%rbx, (%r13)
	jmp	.LBB63_240
.LBB63_194:                             # %bb62
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%rbx, (%rax)
	jmp	.LBB63_240
.LBB63_195:                             # %bb4.i1085
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_196:                             # %bb5.i1083
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	$0, -64(%rbp)                   # 8-byte Folded Reload
	jg	.LBB63_240
.LBB63_197:                             # %bb68
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r15, %r15
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	je	.LBB63_200
# %bb.198:                              # %bb0.i94
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rdi, %rax
	shrq	%rax
	incq	%rax
	js	.LBB63_200
# %bb.199:                              # %bb3.i98
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %r12
	jmp	.LBB63_201
.LBB63_200:                             # %bb1.i102
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$3, %esi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
.LBB63_201:                             # %bb3.i197
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rbx,2), %rdi
	movq	-48(%rbp), %rsi                 # 8-byte Reload
	leaq	-2(%rsi), %rbx
	callq	"Data.String.utf8-lead-payload"@PLT
	movq	%r14, %rdi
	movq	%r12, %rsi
	movq	%rbx, %rdx
	movq	%rax, %rcx
	callq	"Data.String.utf8-cont"@PLT
	movq	%rax, %rbx
	movq	-8(%rax), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB63_203
# %bb.202:                              # %bb70
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB63_203:                             # %bb72
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$1, %rax
	je	.LBB63_208
# %bb.204:                              # %bb72
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	jne	.LBB63_336
# %bb.205:                              # %bb73
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rbx, %rbx
	je	.LBB63_240
# %bb.206:                              # %bb0.i1172
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_214
# %bb.207:                              # %bb2.i1182
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	jmp	.LBB63_240
.LBB63_208:                             # %bb74
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%rbx), %r12
	testq	%rbx, %rbx
	je	.LBB63_211
# %bb.209:                              # %cinch.rc.unique.exit1193
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB63_211
# %bb.210:                              # %bb75
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	jmp	.LBB63_229
.LBB63_211:                             # %bb76
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB63_222
# %bb.212:                              # %bb0.i1198
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB63_220
# %bb.213:                              # %bb2.i1208
                                        #   in Loop: Header=BB63_2 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB63_222
.LBB63_214:                             # %bb3.i1178
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rax, %rax
	je	.LBB63_240
# %bb.215:                              # %bb3.i1178
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB63_84
.LBB63_216:                             # %bb32
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	jmp	.LBB63_240
.LBB63_217:                             # %bb3.i773
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_2
# %bb.218:                              # %bb3.i773
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_293
# %bb.219:                              # %bb5.i774
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_2
.LBB63_220:                             # %bb3.i1204
                                        #   in Loop: Header=BB63_2 Depth=1
	je	.LBB63_222
# %bb.221:                              # %bb4.i1206
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
.LBB63_222:                             # %cinch.rc.dup.tobj.exit1210
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rbx, %rbx
	je	.LBB63_229
# %bb.223:                              # %bb0.i1212
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_225
# %bb.224:                              # %bb2.i1222
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB63_229
.LBB63_225:                             # %bb3.i1218
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_229
# %bb.226:                              # %bb3.i1218
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_228
# %bb.227:                              # %bb5.i1219
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_229
.LBB63_228:                             # %bb4.i1221
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
.LBB63_229:                             # %bb77
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-48(%rbp), %rax                 # 8-byte Reload
                                        # kill: def $eax killed $eax killed $rax def $rax
	andl	$-2, %eax
	cmpq	$4, %rax
	je	.LBB63_232
# %bb.230:                              # %bb77
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpl	$6, %eax
	jne	.LBB63_237
# %bb.231:                              # %Data.String.utf8-min.exit.fold.split
                                        #   in Loop: Header=BB63_2 Depth=1
	movl	$4097, %esi                     # imm = 0x1001
	testb	$1, %r12b
	jne	.LBB63_238
.LBB63_233:                             # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jns	.LBB63_239
# %bb.234:                              # %bb78
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%r12, %r12
	je	.LBB63_240
# %bb.235:                              # %bb0.i1264
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_294
# %bb.236:                              # %bb2.i1274
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	jmp	.LBB63_240
.LBB63_232:                             #   in Loop: Header=BB63_2 Depth=1
	movl	$257, %esi                      # imm = 0x101
	testb	$1, %r12b
	je	.LBB63_233
.LBB63_238:                             # %bb0.i207
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rax
	shrq	%rax
	shrl	%esi
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	cmpq	%rsi, %rax
	jb	.LBB63_240
.LBB63_239:                             # %bb79
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	movq	-48(%rbp), %rsi                 # 8-byte Reload
	callq	"Data.String.utf8-char"@PLT
	movq	%rax, %r13
	.p2align	4
.LBB63_240:                             # %bb32
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB63_242
# %bb.241:                              # %bb81
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB63_242:                             # %bb83
                                        #   in Loop: Header=BB63_2 Depth=1
	cmpq	$1, %rax
	jne	.LBB63_296
# %bb.243:                              # %bb85
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	(%r13), %r12
	testq	%r13, %r13
	je	.LBB63_246
# %bb.244:                              # %cinch.rc.unique.exit1316
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r13), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB63_246
# %bb.245:                              # %bb86
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB63_258
	.p2align	4
.LBB63_246:                             # %bb87
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r12, %r12
	je	.LBB63_251
# %bb.247:                              # %bb0.i1318
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB63_249
# %bb.248:                              # %bb2.i1328
                                        #   in Loop: Header=BB63_2 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB63_251
.LBB63_249:                             # %bb3.i1324
                                        #   in Loop: Header=BB63_2 Depth=1
	je	.LBB63_251
# %bb.250:                              # %bb4.i1326
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB63_251:                             # %cinch.rc.dup.obj.exit1330
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r13, %r13
	je	.LBB63_258
# %bb.252:                              # %bb0.i1332
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_254
# %bb.253:                              # %bb2.i1342
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB63_258
.LBB63_254:                             # %bb3.i1338
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_258
# %bb.255:                              # %bb3.i1338
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_257
# %bb.256:                              # %bb5.i1339
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_258
.LBB63_257:                             # %bb4.i1341
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB63_258:                             # %bb88
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB63_260
# %bb.259:                              # %bb90
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-16(%r12), %rax
	movq	(%rax), %rax
.LBB63_260:                             # %bb92
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	movq	-56(%rbp), %r13                 # 8-byte Reload
	jne	.LBB63_336
# %bb.261:                              # %bb93
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	8(%r12), %rbx
	testq	%r12, %r12
	je	.LBB63_264
# %bb.262:                              # %cinch.rc.unique.exit1353
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB63_264
# %bb.263:                              # %bb94
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_free@PLT
	testl	%ebx, %r15d
	jne	.LBB63_276
	jmp	.LBB63_279
	.p2align	4
.LBB63_264:                             # %bb95
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB63_269
# %bb.265:                              # %bb0.i1358
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB63_267
# %bb.266:                              # %bb2.i1368
                                        #   in Loop: Header=BB63_2 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB63_269
.LBB63_267:                             # %bb3.i1364
                                        #   in Loop: Header=BB63_2 Depth=1
	je	.LBB63_269
# %bb.268:                              # %bb4.i1366
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB63_269:                             # %cinch.rc.dup.tobj.exit1370
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r12, %r12
	je	.LBB63_275
# %bb.270:                              # %bb0.i1372
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_272
# %bb.271:                              # %bb2.i1382
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	testl	%ebx, %r15d
	jne	.LBB63_276
	jmp	.LBB63_279
.LBB63_272:                             # %bb3.i1378
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_275
# %bb.273:                              # %bb3.i1378
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_278
# %bb.274:                              # %bb5.i1379
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	.p2align	4
.LBB63_275:                             # %bb96
                                        #   in Loop: Header=BB63_2 Depth=1
	testl	%ebx, %r15d
	je	.LBB63_279
.LBB63_276:                             # %bb0.i
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rcx
	shrq	%rcx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB63_279
# %bb.277:                              # %bb3.i
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	1(,%rax,2), %r12
	jmp	.LBB63_280
.LBB63_278:                             # %bb4.i1381
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	testl	%ebx, %r15d
	jne	.LBB63_276
	.p2align	4
.LBB63_279:                             # %bb1.i
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
.LBB63_280:                             # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%r13, %r13
	sete	%al
	orb	%al, %r15b
	jne	.LBB63_287
# %bb.281:                              # %bb0.i1389
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_283
# %bb.282:                              # %bb2.i1399
                                        #   in Loop: Header=BB63_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB63_287
.LBB63_283:                             # %bb3.i1395
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rax, %rax
	je	.LBB63_287
# %bb.284:                              # %bb3.i1395
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_286
# %bb.285:                              # %bb5.i1396
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_287
.LBB63_286:                             # %bb4.i1398
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB63_287:                             # %cinch.rc.drop.tobj.exit1401
                                        #   in Loop: Header=BB63_2 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	movq	%r12, %r13
	testb	$1, %cl
	jne	.LBB63_2
# %bb.288:                              # %bb0.i1406
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB63_1
# %bb.289:                              # %bb3.i1412
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %r13
	testq	%rax, %rax
	je	.LBB63_2
# %bb.290:                              # %bb3.i1412
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_292
# %bb.291:                              # %bb5.i1413
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_slow@PLT
	movq	%r12, %r13
	jmp	.LBB63_2
.LBB63_292:                             # %bb4.i1415
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	movq	%r12, %r13
	jmp	.LBB63_2
.LBB63_237:                             # %bb0.i.i
                                        #   in Loop: Header=BB63_2 Depth=1
	xorl	%esi, %esi
	cmpl	$8, %eax
	sete	%sil
	shll	$17, %esi
	incq	%rsi
	testb	$1, %r12b
	je	.LBB63_233
	jmp	.LBB63_238
.LBB63_293:                             # %bb4.i776
                                        #   in Loop: Header=BB63_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB63_2
.LBB63_294:                             # %bb3.i1270
                                        #   in Loop: Header=BB63_2 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %r13
	testq	%rax, %rax
	je	.LBB63_240
# %bb.295:                              # %bb3.i1270
                                        #   in Loop: Header=BB63_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB63_84
	jmp	.LBB63_216
.LBB63_296:                             # %bb83
	testq	%rax, %rax
	jne	.LBB63_336
# %bb.297:                              # %bb84
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	testq	%rdx, %rdx
	sete	%al
	orb	%al, %r15b
	jne	.LBB63_317
# %bb.298:                              # %bb0.i1281
	movq	-8(%rdx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_307
# %bb.299:                              # %bb2.i1291
	decq	%rcx
	movq	%rcx, -8(%rdx)
	jmp	.LBB63_317
.LBB63_300:                             # %bb1
	movl	$1, %eax
	testq	%r13, %r13
	je	.LBB63_330
# %bb.301:                              # %bb0.i554
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_310
# %bb.302:                              # %bb2.i564
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB63_329
.LBB63_303:                             # %bb18
	testq	%rcx, %rcx
	jne	.LBB63_336
# %bb.304:                              # %bb19
	testq	%r12, %r12
	sete	%cl
	orb	%cl, %r13b
	jne	.LBB63_333
# %bb.305:                              # %bb0.i784
	movq	-8(%r12), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_323
# %bb.306:                              # %bb2.i794
	decq	%rdx
	movq	%rdx, -8(%r12)
	jmp	.LBB63_333
.LBB63_307:                             # %bb3.i1287
	testq	%rax, %rax
	je	.LBB63_317
# %bb.308:                              # %bb3.i1287
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_316
# %bb.309:                              # %bb5.i1288
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_317
.LBB63_310:                             # %bb3.i560
	testq	%rax, %rax
	je	.LBB63_329
# %bb.311:                              # %bb3.i560
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_328
.LBB63_312:                             # %common.ret
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_329
.LBB63_313:                             # %bb8
	movl	$1, %eax
	testq	%r12, %r12
	je	.LBB63_330
# %bb.314:                              # %bb0.i675
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB63_326
# %bb.315:                              # %bb2.i685
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB63_329
.LBB63_316:                             # %bb4.i1290
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_drop_mt@PLT
.LBB63_317:                             # %cinch.rc.drop.tobj.exit1293
	testq	%r13, %r13
	je	.LBB63_336
# %bb.318:                              # %bb0.i1295
	movq	-8(%r13), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB63_320
# %bb.319:                              # %bb2.i1305
	decq	%rax
	movq	%rax, -8(%r13)
	xorl	%eax, %eax
	jmp	.LBB63_330
.LBB63_320:                             # %bb3.i1301
	movq	%rcx, %rdx
	movq	%rcx, %rax
	testq	%rdx, %rdx
	je	.LBB63_330
# %bb.321:                              # %bb3.i1301
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB63_339
.LBB63_322:                             # %common.ret
	callq	cinch_drop_slow@PLT
	xorl	%eax, %eax
	jmp	.LBB63_330
.LBB63_323:                             # %bb3.i790
	testq	%rcx, %rcx
	je	.LBB63_333
# %bb.324:                              # %bb3.i790
	movq	%rax, %rbx
	movq	%r12, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB63_331
# %bb.325:                              # %bb5.i791
	callq	cinch_drop_slow@PLT
	jmp	.LBB63_332
.LBB63_326:                             # %bb3.i681
	testq	%rax, %rax
	je	.LBB63_329
# %bb.327:                              # %bb3.i681
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB63_312
.LBB63_328:                             # %common.ret
	callq	cinch_drop_mt@PLT
.LBB63_329:
	movl	$1, %eax
	jmp	.LBB63_330
.LBB63_331:                             # %bb4.i793
	callq	cinch_drop_mt@PLT
.LBB63_332:                             # %cinch.rc.drop.tobj.exit796
	movq	%rbx, %rax
.LBB63_333:                             # %cinch.rc.drop.tobj.exit796
	testq	%rax, %rax
	je	.LBB63_336
# %bb.334:                              # %bb0.i798
	movq	-8(%rax), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	cmpq	$536870913, %rdx                # imm = 0x20000001
	jl	.LBB63_337
# %bb.335:                              # %bb2.i808
	decq	%rcx
	movq	%rcx, -8(%rax)
.LBB63_336:
	xorl	%eax, %eax
.LBB63_330:                             # %common.ret
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB63_337:                             # %bb3.i804
	.cfi_def_cfa %rbp, 16
	movq	%rdx, %rsi
	testq	%rdx, %rdx
	je	.LBB63_340
# %bb.338:                              # %bb3.i804
	movq	%rax, %rdi
	cmpq	$536870912, %rsi                # imm = 0x20000000
	je	.LBB63_322
.LBB63_339:                             # %common.ret
	callq	cinch_drop_mt@PLT
	xorl	%eax, %eax
	jmp	.LBB63_330
.LBB63_340:
	movq	%rsi, %rax
	jmp	.LBB63_330
.Lfunc_end63:
	.size	"Data.String.utf8-valid-from", .Lfunc_end63-"Data.String.utf8-valid-from"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.empty-string#cc"   # -- Begin function Data.String.empty-string#cc
	.p2align	4
	.type	"Data.String.empty-string#cc",@function
"Data.String.empty-string#cc":          # @"Data.String.empty-string#cc"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$1, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_bytes_new@PLT             # TAILCALL
.Lfunc_end64:
	.size	"Data.String.empty-string#cc", .Lfunc_end64-"Data.String.empty-string#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.empty-string"      # -- Begin function Data.String.empty-string
	.p2align	4
	.type	"Data.String.empty-string",@function
"Data.String.empty-string":             # @Data.String.empty-string
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movl	$1, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end65:
	.size	"Data.String.empty-string", .Lfunc_end65-"Data.String.empty-string"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.string-concat#cc"  # -- Begin function Data.String.string-concat#cc
	.p2align	4
	.type	"Data.String.string-concat#cc",@function
"Data.String.string-concat#cc":         # @"Data.String.string-concat#cc"
	.cfi_startproc
# %bb.0:                                # %entry
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB66_2
# %bb.1:                                # %bb3
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB66_2:                               # %bb5
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	testq	%rax, %rax
	jne	.LBB66_12
# %bb.4:                                # %bb6
	movq	(%rdi), %r14
	testq	%r14, %r14
	je	.LBB66_9
# %bb.5:                                # %bb0.i122
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB66_7
# %bb.6:                                # %bb2.i130
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB66_9
.LBB66_7:                               # %bb3.i128
	je	.LBB66_9
# %bb.8:                                # %bb4.i
	movq	%r14, %rdi
	movq	%rsi, %rbx
	callq	cinch_dup_mt@PLT
	movq	%rbx, %rsi
.LBB66_9:                               # %cinch.rc.dup.obj.exit
	movq	-8(%rsi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB66_11
# %bb.10:                               # %bb9
	movq	-16(%rsi), %rax
	movq	(%rax), %rax
.LBB66_11:                              # %bb11
	testq	%rax, %rax
	je	.LBB66_14
.LBB66_12:
	xorl	%r15d, %r15d
.LBB66_13:                              # %common.ret
	movq	%r15, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB66_14:                              # %bb12
	.cfi_def_cfa %rbp, 16
	movq	(%rsi), %rbx
	testq	%rbx, %rbx
	je	.LBB66_19
# %bb.15:                               # %bb0.i133
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB66_17
# %bb.16:                               # %bb2.i143
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB66_19
.LBB66_17:                              # %bb3.i139
	je	.LBB66_19
# %bb.18:                               # %bb4.i141
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB66_19:                              # %cinch.rc.dup.obj.exit145
	movabsq	$1030792151040, %r12            # imm = 0xF000000000
	movabsq	$274877906944, %r13             # imm = 0x4000000000
	testq	%r14, %r14
	je	.LBB66_22
# %bb.20:                               # %bb0.i83
	movq	-8(%r14), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB66_22
# %bb.21:                               # %bb1.i97
	movq	(%r14), %r15
	testq	%rbx, %rbx
	jne	.LBB66_23
	jmp	.LBB66_25
.LBB66_22:                              # %bb2.i91
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r15
	testq	%rbx, %rbx
	je	.LBB66_25
.LBB66_23:                              # %bb0.i65
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB66_25
# %bb.24:                               # %bb1.i79
	movq	(%rbx), %rax
	jmp	.LBB66_26
.LBB66_25:                              # %bb2.i73
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB66_26:                              # %bb0.i
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	movq	%r15, %rdx
	andq	%r12, %rdx
	movq	%rax, %rcx
	andq	%r12, %rcx
	addq	%rdx, %rcx
	js	.LBB66_28
# %bb.27:                               # %bb3.i
	leaq	1(,%rcx,2), %r13
	jmp	.LBB66_29
.LBB66_28:                              # %bb1.i
	leaq	1(,%r15,2), %rdi
	leaq	1(,%rax,2), %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r13
.LBB66_29:                              # %cinch.runtime.natAdd.exit
	movq	%r13, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB66_36
# %bb.30:                               # %bb0.i.i
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB66_32
# %bb.31:                               # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB66_36
.LBB66_32:                              # %bb3.i.i
	testq	%rax, %rax
	je	.LBB66_36
# %bb.33:                               # %bb3.i.i
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB66_35
# %bb.34:                               # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB66_36
.LBB66_35:                              # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB66_36:                              # %cinch.runtime.bytesNew.exit
	testq	%r14, %r14
	je	.LBB66_39
# %bb.37:                               # %bb0.i47
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %r13            # imm = 0xF000000000
	andq	%r13, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB66_40
# %bb.38:                               # %cinch.runtime.bytesLen.exit63.thread
	movq	(%r14), %r8
	jmp	.LBB66_41
.LBB66_39:                              # %cinch.runtime.bytesLen.exit45.thread256.critedge
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r12, %rax
	movq	%r15, %rdi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%rax, %r8
	callq	cinch_bytes_copy@PLT
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r13
	jmp	.LBB66_50
.LBB66_40:                              # %cinch.runtime.bytesLen.exit63
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB66_41:                              # %bb0.i29
	andq	%r12, %r8
	movq	%r15, %rdi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%r14), %rax
	andq	%r13, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB66_43
# %bb.42:                               # %cinch.runtime.bytesLen.exit45.thread
	movq	(%r14), %r13
	jmp	.LBB66_44
.LBB66_43:                              # %cinch.runtime.bytesLen.exit45
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r13
.LBB66_44:                              # %bb0.i194
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB66_46
# %bb.45:                               # %bb2.i203
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB66_50
.LBB66_46:                              # %bb3.i200
	testq	%rax, %rax
	je	.LBB66_50
# %bb.47:                               # %bb3.i200
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB66_49
# %bb.48:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB66_50
.LBB66_49:                              # %bb4.i202
	callq	cinch_drop_mt@PLT
.LBB66_50:                              # %cinch.rc.drop.obj.exit
	andq	%r12, %r13
	testq	%rbx, %rbx
	je	.LBB66_53
# %bb.51:                               # %bb0.i19
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB66_54
# %bb.52:                               # %cinch.runtime.bytesLen.exit.thread
	movq	(%rbx), %r8
	jmp	.LBB66_55
.LBB66_53:                              # %cinch.rc.drop.obj.exit218.critedge
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%rax, %r12
	movq	%r15, %rdi
	movq	%rbx, %rsi
	movq	%r13, %rdx
	xorl	%ecx, %ecx
	movq	%r12, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB66_13
.LBB66_54:                              # %cinch.runtime.bytesLen.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB66_55:                              # %bb0.i206
	andq	%r12, %r8
	movq	%r15, %rdi
	movq	%rbx, %rsi
	movq	%r13, %rdx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB66_57
# %bb.56:                               # %bb2.i216
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB66_13
.LBB66_57:                              # %bb3.i212
	testq	%rax, %rax
	je	.LBB66_13
# %bb.58:                               # %bb3.i212
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB66_60
# %bb.59:                               # %bb5.i213
	callq	cinch_drop_slow@PLT
	jmp	.LBB66_13
.LBB66_60:                              # %bb4.i215
	callq	cinch_drop_mt@PLT
	jmp	.LBB66_13
.Lfunc_end66:
	.size	"Data.String.string-concat#cc", .Lfunc_end66-"Data.String.string-concat#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.string-concat"     # -- Begin function Data.String.string-concat
	.p2align	4
	.type	"Data.String.string-concat",@function
"Data.String.string-concat":            # @Data.String.string-concat
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end67:
	.size	"Data.String.string-concat", .Lfunc_end67-"Data.String.string-concat"
	.cfi_endproc
                                        # -- End function
	.globl	Data.String.unpack              # -- Begin function Data.String.unpack
	.p2align	4
	.type	Data.String.unpack,@function
Data.String.unpack:                     # @Data.String.unpack
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB68_2
# %bb.1:                                # %bb3
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB68_2:                               # %bb5
	testq	%rax, %rax
	je	.LBB68_3
.LBB68_71:                              # %common.ret
	xorl	%eax, %eax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB68_3:                               # %bb6
	.cfi_def_cfa %rbp, 16
	movq	(%rdi), %rbx
	testq	%rbx, %rbx
	je	.LBB68_8
# %bb.4:                                # %bb0.i13
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB68_6
# %bb.5:                                # %bb2.i
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB68_8
.LBB68_6:                               # %bb3.i
	je	.LBB68_8
# %bb.7:                                # %bb4.i
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB68_8:                               # %cinch.rc.dup.obj.exit
	movl	$1, %esi
	movq	%rbx, %rdi
	callq	"Data.String.utf8-at"@PLT
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB68_10
# %bb.9:                                # %bb8
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB68_10:                              # %bb10
	cmpq	$1, %rcx
	je	.LBB68_28
# %bb.11:                               # %bb10
	testq	%rcx, %rcx
	jne	.LBB68_71
# %bb.12:                               # %bb11
	testq	%rbx, %rbx
	je	.LBB68_20
# %bb.13:                               # %bb0.i17
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB68_15
# %bb.14:                               # %bb2.i26
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB68_20
.LBB68_28:                              # %bb12
	movq	(%rax), %r14
	testq	%rax, %rax
	je	.LBB68_31
# %bb.29:                               # %cinch.rc.unique.exit
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB68_31
# %bb.30:                               # %bb13
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB68_43
.LBB68_31:                              # %bb14
	testq	%r14, %r14
	je	.LBB68_36
# %bb.32:                               # %bb0.i45
	movq	-8(%r14), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB68_34
# %bb.33:                               # %bb2.i55
	incq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB68_36
.LBB68_15:                              # %bb3.i23
	testq	%rcx, %rcx
	je	.LBB68_20
# %bb.16:                               # %bb3.i23
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB68_18
# %bb.17:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB68_19
.LBB68_34:                              # %bb3.i51
	je	.LBB68_36
# %bb.35:                               # %bb4.i53
	movq	%r14, %rdi
	movq	%rax, %r15
	callq	cinch_dup_mt@PLT
	movq	%r15, %rax
.LBB68_36:                              # %cinch.rc.dup.obj.exit57
	testq	%rax, %rax
	je	.LBB68_43
# %bb.37:                               # %bb0.i59
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB68_39
# %bb.38:                               # %bb2.i69
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB68_43
.LBB68_39:                              # %bb3.i65
	testq	%rcx, %rcx
	je	.LBB68_43
# %bb.40:                               # %bb3.i65
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB68_42
# %bb.41:                               # %bb5.i66
	callq	cinch_drop_slow@PLT
	jmp	.LBB68_43
.LBB68_18:                              # %bb4.i25
	callq	cinch_drop_mt@PLT
.LBB68_19:                              # %cinch.rc.drop.obj.exit
	movq	%r14, %rax
.LBB68_20:                              # %cinch.rc.drop.obj.exit
	testq	%rax, %rax
	je	.LBB68_27
# %bb.21:                               # %bb0.i29
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB68_23
# %bb.22:                               # %bb2.i39
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB68_27
.LBB68_23:                              # %bb3.i35
	testq	%rcx, %rcx
	je	.LBB68_27
# %bb.24:                               # %bb3.i35
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB68_26
# %bb.25:                               # %bb5.i36
	callq	cinch_drop_slow@PLT
	jmp	.LBB68_27
.LBB68_26:                              # %bb4.i38
	callq	cinch_drop_mt@PLT
.LBB68_27:                              # %cinch.rc.drop.obj.exit41
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Data.List.reverse@R"@PLT       # TAILCALL
.LBB68_42:                              # %bb4.i68
	.cfi_def_cfa %rbp, 16
	callq	cinch_drop_mt@PLT
.LBB68_43:                              # %bb15
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB68_45
# %bb.44:                               # %bb17
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB68_45:                              # %bb19
	testq	%rax, %rax
	jne	.LBB68_71
# %bb.46:                               # %bb20
	movq	(%r14), %r12
	movq	8(%r14), %r15
	testq	%r14, %r14
	je	.LBB68_49
# %bb.47:                               # %cinch.rc.unique.exit80
	movq	-8(%r14), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB68_49
# %bb.48:                               # %bb21
	movq	%r14, %rdi
	callq	cinch_free@PLT
	movl	%r15d, %r13d
	andl	$1, %r13d
	jmp	.LBB68_61
.LBB68_49:                              # %bb22
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %r13d
	andl	$1, %r13d
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB68_54
# %bb.50:                               # %bb0.i85
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB68_52
# %bb.51:                               # %bb2.i94
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB68_54
.LBB68_52:                              # %bb3.i91
	je	.LBB68_54
# %bb.53:                               # %bb4.i92
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB68_54:                              # %cinch.rc.dup.tobj.exit
	testq	%r14, %r14
	je	.LBB68_61
# %bb.55:                               # %bb0.i97
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB68_57
# %bb.56:                               # %bb2.i107
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB68_61
.LBB68_57:                              # %bb3.i103
	testq	%rax, %rax
	je	.LBB68_61
# %bb.58:                               # %bb3.i103
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB68_60
# %bb.59:                               # %bb5.i104
	callq	cinch_drop_slow@PLT
	jmp	.LBB68_61
.LBB68_60:                              # %bb4.i106
	callq	cinch_drop_mt@PLT
.LBB68_61:                              # %bb23
	testq	%r13, %r13
	je	.LBB68_63
# %bb.62:
	movq	%r15, %r14
.LBB68_70:                              # %cinch.rc.drop.tobj.exit
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	%r12, (%rax)
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rcx
	movq	%rcx, 8(%rax)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%rax, %rdx
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Data.String.unpack-from"@PLT   # TAILCALL
.LBB68_63:                              # %cinch.runtime.natAdd.exit
	.cfi_def_cfa %rbp, 16
	movl	$1, %edi
	movq	%r15, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%r15, %r15
	je	.LBB68_70
# %bb.64:                               # %bb0.i114
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB68_66
# %bb.65:                               # %bb2.i124
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB68_70
.LBB68_66:                              # %bb3.i120
	testq	%rax, %rax
	je	.LBB68_70
# %bb.67:                               # %bb3.i120
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB68_69
# %bb.68:                               # %bb5.i121
	callq	cinch_drop_slow@PLT
	jmp	.LBB68_70
.LBB68_69:                              # %bb4.i123
	callq	cinch_drop_mt@PLT
	jmp	.LBB68_70
.Lfunc_end68:
	.size	Data.String.unpack, .Lfunc_end68-Data.String.unpack
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.unpack-from"       # -- Begin function Data.String.unpack-from
	.p2align	4
	.type	"Data.String.unpack-from",@function
"Data.String.unpack-from":              # @Data.String.unpack-from
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
	movq	%rdx, %r15
	movq	%rsi, %r13
	movq	%rdi, %r12
	movq	%rdi, -48(%rbp)                 # 8-byte Spill
	jmp	.LBB69_1
	.p2align	4
.LBB69_75:                              # %bb2.i149
                                        #   in Loop: Header=BB69_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
.LBB69_80:                              # %cinch.rc.drop.tobj.exit151
                                        #   in Loop: Header=BB69_1 Depth=1
	movl	$16, %edi
	movl	$513, %esi                      # imm = 0x201
	callq	cinch_alloc_ctor@PLT
	movq	-56(%rbp), %rcx                 # 8-byte Reload
	movq	%rcx, (%rax)
	movq	%r14, 8(%rax)
	movq	%rax, %r15
	movq	-48(%rbp), %r12                 # 8-byte Reload
.LBB69_1:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	movq	%r13, %rbx
	movq	%r12, %rdi
	movq	%r13, %rsi
	callq	"Data.String.utf8-at"@PLT
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB69_3
# %bb.2:                                # %bb2
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB69_3:                               # %bb4
                                        #   in Loop: Header=BB69_1 Depth=1
	cmpq	$1, %rcx
	jne	.LBB69_4
# %bb.29:                               # %bb6
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	(%rax), %r13
	testq	%rax, %rax
	je	.LBB69_32
# %bb.30:                               # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rdx, %rcx
	cmpq	$1, %rcx
	jne	.LBB69_32
# %bb.31:                               # %bb7
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB69_44
	.p2align	4
.LBB69_32:                              # %bb8
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%r13, %r13
	je	.LBB69_37
# %bb.33:                               # %bb0.i55
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB69_35
# %bb.34:                               # %bb2.i64
                                        #   in Loop: Header=BB69_1 Depth=1
	incq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB69_37
.LBB69_35:                              # %bb3.i61
                                        #   in Loop: Header=BB69_1 Depth=1
	je	.LBB69_37
# %bb.36:                               # %bb4.i62
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r13, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
	.p2align	4
.LBB69_37:                              # %cinch.rc.dup.obj.exit
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rax, %rax
	je	.LBB69_44
# %bb.38:                               # %bb0.i67
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB69_40
# %bb.39:                               # %bb2.i77
                                        #   in Loop: Header=BB69_1 Depth=1
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB69_44
	.p2align	4
.LBB69_40:                              # %bb3.i73
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rcx, %rcx
	je	.LBB69_44
# %bb.41:                               # %bb3.i73
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB69_43
# %bb.42:                               # %bb5.i74
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_44
.LBB69_43:                              # %bb4.i76
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB69_44:                              # %bb9
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB69_46
# %bb.45:                               # %bb11
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB69_46:                              # %bb13
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rax, %rax
	jne	.LBB69_81
# %bb.47:                               # %bb14
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	(%r13), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movq	8(%r13), %r12
	testq	%r13, %r13
	je	.LBB69_50
# %bb.48:                               # %cinch.rc.unique.exit88
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r13), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB69_50
# %bb.49:                               # %bb15
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB69_62
	.p2align	4
.LBB69_50:                              # %bb16
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB69_55
# %bb.51:                               # %bb0.i93
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB69_53
# %bb.52:                               # %bb2.i102
                                        #   in Loop: Header=BB69_1 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB69_55
.LBB69_53:                              # %bb3.i99
                                        #   in Loop: Header=BB69_1 Depth=1
	je	.LBB69_55
# %bb.54:                               # %bb4.i100
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
	.p2align	4
.LBB69_55:                              # %cinch.rc.dup.tobj.exit
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%r13, %r13
	je	.LBB69_62
# %bb.56:                               # %bb0.i105
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB69_58
# %bb.57:                               # %bb2.i115
                                        #   in Loop: Header=BB69_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB69_62
.LBB69_58:                              # %bb3.i111
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rax, %rax
	je	.LBB69_62
# %bb.59:                               # %bb3.i111
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB69_61
# %bb.60:                               # %bb5.i112
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_62
.LBB69_61:                              # %bb4.i114
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB69_62:                              # %bb17
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r15, %r14
	movl	%ebx, %r15d
	andl	$1, %r15d
	testl	%r12d, %r15d
	je	.LBB69_65
# %bb.63:                               # %bb0.i
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%r12, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB69_65
# %bb.64:                               # %bb3.i
                                        #   in Loop: Header=BB69_1 Depth=1
	leaq	1(,%rax,2), %r13
	jmp	.LBB69_66
	.p2align	4
.LBB69_65:                              # %bb1.i
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%rbx, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r13
.LBB69_66:                              # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rbx, %rbx
	sete	%al
	orb	%al, %r15b
	jne	.LBB69_73
# %bb.67:                               # %bb0.i122
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB69_69
# %bb.68:                               # %bb2.i132
                                        #   in Loop: Header=BB69_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB69_73
	.p2align	4
.LBB69_69:                              # %bb3.i128
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rax, %rax
	je	.LBB69_73
# %bb.70:                               # %bb3.i128
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB69_72
# %bb.71:                               # %bb5.i129
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_73
.LBB69_72:                              # %bb4.i131
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB69_73:                              # %cinch.rc.drop.tobj.exit134
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB69_80
# %bb.74:                               # %bb0.i139
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB69_75
# %bb.76:                               # %bb3.i145
                                        #   in Loop: Header=BB69_1 Depth=1
	testq	%rax, %rax
	je	.LBB69_80
# %bb.77:                               # %bb3.i145
                                        #   in Loop: Header=BB69_1 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB69_79
# %bb.78:                               # %bb5.i146
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_80
.LBB69_79:                              # %bb4.i148
                                        #   in Loop: Header=BB69_1 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB69_80
.LBB69_4:                               # %bb4
	testq	%rcx, %rcx
	jne	.LBB69_81
# %bb.5:                                # %bb5
	testq	%r12, %r12
	je	.LBB69_13
# %bb.6:                                # %bb0.i14
	movq	-8(%r12), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB69_8
# %bb.7:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%r12)
	jmp	.LBB69_13
.LBB69_81:                              # %common.ret
	xorl	%eax, %eax
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB69_8:                               # %bb3.i19
	.cfi_def_cfa %rbp, 16
	testq	%rcx, %rcx
	je	.LBB69_13
# %bb.9:                                # %bb3.i19
	movq	%rax, %r14
	movq	%r12, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB69_11
# %bb.10:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_12
.LBB69_11:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB69_12:                              # %cinch.rc.drop.obj.exit
	movq	%r14, %rax
.LBB69_13:                              # %cinch.rc.drop.obj.exit
	testq	%rbx, %rbx
	sete	%cl
	movl	%ebx, %edx
	orb	%cl, %dl
	testb	$1, %dl
	jne	.LBB69_21
# %bb.14:                               # %bb0.i26
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB69_16
# %bb.15:                               # %bb2.i36
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB69_21
.LBB69_16:                              # %bb3.i32
	testq	%rcx, %rcx
	je	.LBB69_21
# %bb.17:                               # %bb3.i32
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB69_19
# %bb.18:                               # %bb5.i33
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_20
.LBB69_19:                              # %bb4.i35
	callq	cinch_drop_mt@PLT
.LBB69_20:                              # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
.LBB69_21:                              # %cinch.rc.drop.tobj.exit
	testq	%rax, %rax
	je	.LBB69_28
# %bb.22:                               # %bb0.i39
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB69_24
# %bb.23:                               # %bb2.i49
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB69_28
.LBB69_24:                              # %bb3.i45
	testq	%rcx, %rcx
	je	.LBB69_28
# %bb.25:                               # %bb3.i45
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB69_27
# %bb.26:                               # %bb5.i46
	callq	cinch_drop_slow@PLT
	jmp	.LBB69_28
.LBB69_27:                              # %bb4.i48
	callq	cinch_drop_mt@PLT
.LBB69_28:                              # %cinch.rc.drop.obj.exit51
	movq	%r15, %rdi
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Data.List.reverse@R"@PLT       # TAILCALL
.Lfunc_end69:
	.size	"Data.String.unpack-from", .Lfunc_end69-"Data.String.unpack-from"
	.cfi_endproc
                                        # -- End function
	.globl	Data.String.StrBuf              # -- Begin function Data.String.StrBuf
	.p2align	4
	.type	Data.String.StrBuf,@function
Data.String.StrBuf:                     # @Data.String.StrBuf
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end70:
	.size	Data.String.StrBuf, .Lfunc_end70-Data.String.StrBuf
                                        # -- End function
	.globl	"Data.String.Eq_String.=="      # -- Begin function Data.String.Eq_String.==
	.p2align	4
	.type	"Data.String.Eq_String.==",@function
"Data.String.Eq_String.==":             # @"Data.String.Eq_String.=="
	.cfi_startproc
# %bb.0:                                # %entry
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB71_2
# %bb.1:                                # %bb3
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB71_2:                               # %bb5
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	testq	%rax, %rax
	jne	.LBB71_3
# %bb.4:                                # %bb6
	movq	(%rdi), %rbx
	testq	%rbx, %rbx
	je	.LBB71_9
# %bb.5:                                # %bb0.i
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB71_7
# %bb.6:                                # %bb2.i
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB71_9
.LBB71_7:                               # %bb3.i
	je	.LBB71_9
# %bb.8:                                # %bb4.i
	movq	%rbx, %rdi
	movq	%rsi, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rsi
.LBB71_9:                               # %cinch.rc.dup.obj.exit
	movq	-8(%rsi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB71_11
# %bb.10:                               # %bb9
	movq	-16(%rsi), %rax
	movq	(%rax), %rax
.LBB71_11:                              # %bb11
	testq	%rax, %rax
	je	.LBB71_13
.LBB71_3:
	xorl	%eax, %eax
.LBB71_19:                              # %common.ret19
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB71_13:                              # %bb12
	.cfi_def_cfa %rbp, 16
	movq	(%rsi), %rsi
	testq	%rsi, %rsi
	je	.LBB71_18
# %bb.14:                               # %bb0.i5
	movq	-8(%rsi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB71_16
# %bb.15:                               # %bb2.i14
	incq	%rax
	movq	%rax, -8(%rsi)
	jmp	.LBB71_18
.LBB71_16:                              # %bb3.i11
	je	.LBB71_18
# %bb.17:                               # %bb4.i13
	movq	%rsi, %rdi
	movq	%rsi, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rsi
.LBB71_18:                              # %cinch.rc.dup.obj.exit16
	movq	%rbx, %rdi
	callq	cinch.runtime.bytesCompare@PLT
	movq	%rax, %rcx
	xorl	%eax, %eax
	testq	%rcx, %rcx
	sete	%al
	jmp	.LBB71_19
.Lfunc_end71:
	.size	"Data.String.Eq_String.==", .Lfunc_end71-"Data.String.Eq_String.=="
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.Eq_String.==.app"  # -- Begin function Data.String.Eq_String.==.app
	.p2align	4
	.type	"Data.String.Eq_String.==.app",@function
	.quad	4848492524724309042             # 0x43494e4348544c32
	.quad	"Data.String.Eq_String.==.tapp"
"Data.String.Eq_String.==.app":         # @"Data.String.Eq_String.==.app"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %rbx
	movq	%rdi, %r14
	movq	16(%rdi), %rbp
	leaq	(%rsi,%rbp), %r15
	cmpq	$1, %r15
	jbe	.LBB72_1
# %bb.20:                               # %bb1
	leaq	(,%rbp,8), %rax
	leaq	24(%r14), %rcx
	testq	%rbp, %rbp
	cmoveq	%rbx, %rcx
	movq	(%rcx), %r13
	leaq	32(%r14), %rcx
	subq	%rax, %rbx
	leaq	8(%rbx), %rax
	cmpq	$2, %rbp
	cmovaeq	%rcx, %rax
	movq	(%rax), %rax
	movq	%rax, 8(%rsp)                   # 8-byte Spill
	movq	-8(%r14), %rsi
	movq	%rsi, %r12
	shlq	$29, %r12
	cmpq	$536870912, %r12                # imm = 0x20000000
	jne	.LBB72_22
# %bb.21:                               # %bb12
	movq	%r14, %rdi
	callq	cinch_free@PLT
	movq	8(%rsp), %rbp                   # 8-byte Reload
	jmp	.LBB72_38
.LBB72_1:                               # %bb0
	movq	%rsi, %r13
	leaq	24(,%r15,8), %rdi
	movl	$2, %esi
	movl	$768, %edx                      # imm = 0x300
	callq	cinch_alloc_shaped@PLT
	movq	%rax, %r12
	movq	(%r14), %rax
	movq	%rax, (%r12)
	movq	$2, 8(%r12)
	movq	%r15, 16(%r12)
	leaq	24(%r12), %r15
	leaq	24(%r14), %rsi
	leaq	(,%rbp,8), %rdx
	movq	%r15, %rdi
	callq	memcpy@PLT
	shlq	$3, %r13
	leaq	(%r12,%rbp,8), %rdi
	addq	$24, %rdi
	movq	%rbx, %rsi
	movq	%r13, %rdx
	callq	memcpy@PLT
	movq	-8(%r14), %rbx
	movq	%rbx, %r13
	shlq	$29, %r13
	cmpq	$536870912, %r13                # imm = 0x20000000
	jne	.LBB72_3
# %bb.2:                                # %bb2
	movq	%r14, %rdi
	callq	cinch_free@PLT
	jmp	.LBB72_19
.LBB72_22:                              # %bb13
	testq	%rbp, %rbp
	je	.LBB72_34
# %bb.23:                               # %bb15
	testq	%r13, %r13
	je	.LBB72_28
# %bb.24:                               # %bb0.i17
	movq	-8(%r13), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB72_26
# %bb.25:                               # %bb2.i26
	incq	%rax
	movq	%rax, -8(%r13)
	jmp	.LBB72_28
.LBB72_3:                               # %bb3
	testq	%rbp, %rbp
	je	.LBB72_15
# %bb.4:                                # %bb5
	movq	(%r15), %rdi
	testq	%rdi, %rdi
	je	.LBB72_9
# %bb.5:                                # %bb0.i
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB72_7
# %bb.6:                                # %bb2.i
	incq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB72_9
.LBB72_26:                              # %bb3.i23
	je	.LBB72_28
# %bb.27:                               # %bb4.i25
	movq	%r13, %rdi
	movq	%rsi, 16(%rsp)                  # 8-byte Spill
	callq	cinch_dup_mt@PLT
	movq	16(%rsp), %rsi                  # 8-byte Reload
.LBB72_28:                              # %bb16
	cmpq	$2, %rbp
	jb	.LBB72_34
# %bb.29:                               # %bb16
	cmpq	$0, 8(%rsp)                     # 8-byte Folded Reload
	je	.LBB72_34
# %bb.30:                               # %bb0.i30
	movq	8(%rsp), %rdx                   # 8-byte Reload
	movq	-8(%rdx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB72_32
# %bb.31:                               # %bb2.i39
	incq	%rax
	movq	%rax, -8(%rdx)
	jmp	.LBB72_34
.LBB72_7:                               # %bb3.i
	je	.LBB72_9
# %bb.8:                                # %bb4.i
	callq	cinch_dup_mt@PLT
.LBB72_9:                               # %bb6
	cmpq	$1, %rbp
	je	.LBB72_15
# %bb.10:                               # %bb7
	movq	32(%r12), %rdi
	testq	%rdi, %rdi
	je	.LBB72_15
# %bb.11:                               # %bb0.i4
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB72_13
# %bb.12:                               # %bb2.i13
	incq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB72_15
.LBB72_32:                              # %bb3.i36
	je	.LBB72_34
# %bb.33:                               # %bb4.i38
	movq	8(%rsp), %rdi                   # 8-byte Reload
	movq	%rsi, %rbp
	callq	cinch_dup_mt@PLT
	movq	%rbp, %rsi
.LBB72_34:                              # %bb18
	testq	%r12, %r12
	jle	.LBB72_36
# %bb.35:                               # %bb19
	decq	%rsi
	movq	%rsi, -8(%r14)
	movq	8(%rsp), %rbp                   # 8-byte Reload
	jmp	.LBB72_38
.LBB72_36:                              # %bb20
	movq	8(%rsp), %rbp                   # 8-byte Reload
	je	.LBB72_38
# %bb.37:                               # %bb21
	movq	%r14, %rdi
	callq	cinch_drop_mt@PLT
.LBB72_38:                              # %bb14
	movq	%r13, %rdi
	movq	%rbp, %rsi
	callq	"Data.String.Eq_String.=="@PLT
	movq	%rax, %r12
	testq	%r13, %r13
	je	.LBB72_45
# %bb.39:                               # %bb0.i43
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB72_41
# %bb.40:                               # %bb2.i51
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB72_45
.LBB72_41:                              # %bb3.i49
	testq	%rax, %rax
	je	.LBB72_45
# %bb.42:                               # %bb3.i49
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB72_44
# %bb.43:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB72_45
.LBB72_44:                              # %bb4.i50
	callq	cinch_drop_mt@PLT
.LBB72_45:                              # %cinch.rc.drop.obj.exit
	testq	%rbp, %rbp
	je	.LBB72_52
# %bb.46:                               # %bb0.i54
	movq	-8(%rbp), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB72_48
# %bb.47:                               # %bb2.i63
	decq	%rcx
	movq	%rcx, -8(%rbp)
	addq	$-2, %r15
	jne	.LBB72_53
	jmp	.LBB72_19
.LBB72_48:                              # %bb3.i60
	testq	%rax, %rax
	je	.LBB72_52
# %bb.49:                               # %bb3.i60
	movq	%rbp, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB72_51
# %bb.50:                               # %bb5.i61
	callq	cinch_drop_slow@PLT
.LBB72_52:                              # %cinch.rc.drop.obj.exit65
	addq	$-2, %r15
	je	.LBB72_19
.LBB72_53:                              # %bb22
	addq	$16, %rbx
	movq	(%r12), %rax
	movq	%r12, %rdi
	movq	%r15, %rsi
	movq	%rbx, %rdx
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	jmpq	*%rax                           # TAILCALL
.LBB72_51:                              # %bb4.i62
	.cfi_def_cfa_offset 80
	callq	cinch_drop_mt@PLT
	addq	$-2, %r15
	jne	.LBB72_53
	jmp	.LBB72_19
.LBB72_13:                              # %bb3.i10
	je	.LBB72_15
# %bb.14:                               # %bb4.i12
	callq	cinch_dup_mt@PLT
.LBB72_15:                              # %bb8
	testq	%r13, %r13
	jle	.LBB72_17
# %bb.16:                               # %bb9
	decq	%rbx
	movq	%rbx, -8(%r14)
	jmp	.LBB72_19
.LBB72_17:                              # %bb10
	je	.LBB72_19
# %bb.18:                               # %bb11
	movq	%r14, %rdi
	callq	cinch_drop_mt@PLT
.LBB72_19:                              # %common.ret
	movq	%r12, %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end72:
	.size	"Data.String.Eq_String.==.app", .Lfunc_end72-"Data.String.Eq_String.==.app"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.String.Eq_String.==.tapp" # -- Begin function Data.String.Eq_String.==.tapp
	.p2align	4
	.type	"Data.String.Eq_String.==.tapp",@function
"Data.String.Eq_String.==.tapp":        # @"Data.String.Eq_String.==.tapp"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$152, %rsp
	.cfi_def_cfa_offset 208
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r15
	movq	%rdi, %rbx
	movq	%rdx, 24(%rsp)
	movq	%rcx, 32(%rsp)
	movq	%r8, 40(%rsp)
	movq	%r9, 48(%rsp)
	cmpq	$5, %rsi
	jb	.LBB73_2
# %bb.1:                                # %bb0
	leaq	-32(,%r15,8), %r14
	leaq	56(%rsp), %r12
	leaq	cinch.tail.argv@TLSLD(%rip), %rdi
	callq	__tls_get_addr@PLT
	leaq	cinch.tail.argv@DTPOFF+32(%rax), %rsi
	movq	%r12, %rdi
	movq	%r14, %rdx
	callq	memcpy@PLT
.LBB73_2:                               # %bb1
	movq	16(%rbx), %rbp
	leaq	(%r15,%rbp), %r14
	cmpq	$1, %r14
	jbe	.LBB73_3
# %bb.23:                               # %bb3
	leaq	24(%rsp), %r15
	leaq	(,%rbp,8), %rax
	leaq	24(%rbx), %rcx
	testq	%rbp, %rbp
	cmoveq	%r15, %rcx
	movq	(%rcx), %r13
	leaq	32(%rbx), %rcx
	subq	%rax, %r15
	leaq	8(%r15), %rax
	cmpq	$2, %rbp
	cmovaeq	%rcx, %rax
	movq	(%rax), %rax
	movq	%rax, 8(%rsp)                   # 8-byte Spill
	movq	-8(%rbx), %rsi
	movq	%rsi, %r12
	shlq	$29, %r12
	cmpq	$536870912, %r12                # imm = 0x20000000
	jne	.LBB73_25
# %bb.24:                               # %bb14
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	movq	8(%rsp), %r12                   # 8-byte Reload
	jmp	.LBB73_41
.LBB73_3:                               # %bb2
	leaq	24(,%r14,8), %rdi
	movl	$2, %esi
	movl	$768, %edx                      # imm = 0x300
	callq	cinch_alloc_shaped@PLT
	movq	%rax, %r12
	movq	(%rbx), %rax
	movq	%rax, (%r12)
	movq	$2, 8(%r12)
	movq	%r14, 16(%r12)
	leaq	24(%r12), %r14
	leaq	24(%rbx), %rsi
	leaq	(,%rbp,8), %rdx
	movq	%r14, %rdi
	callq	memcpy@PLT
	shlq	$3, %r15
	movq	%r12, %r13
	leaq	(%r12,%rbp,8), %rdi
	addq	$24, %rdi
	leaq	24(%rsp), %rsi
	movq	%r15, %rdx
	callq	memcpy@PLT
	movq	-8(%rbx), %r15
	movq	%r15, %r12
	shlq	$29, %r12
	cmpq	$536870912, %r12                # imm = 0x20000000
	jne	.LBB73_5
# %bb.4:                                # %bb4
	movq	%rbx, %rdi
	callq	cinch_free@PLT
.LBB73_21:                              # %common.ret
	movq	%r13, %rax
	jmp	.LBB73_22
.LBB73_25:                              # %bb15
	testq	%rbp, %rbp
	je	.LBB73_37
# %bb.26:                               # %bb17
	testq	%r13, %r13
	je	.LBB73_31
# %bb.27:                               # %bb0.i17
	movq	-8(%r13), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB73_29
# %bb.28:                               # %bb2.i26
	incq	%rax
	movq	%rax, -8(%r13)
	jmp	.LBB73_31
.LBB73_5:                               # %bb5
	testq	%rbp, %rbp
	movq	%r13, %rax
	je	.LBB73_17
# %bb.6:                                # %bb7
	movq	(%r14), %rdi
	testq	%rdi, %rdi
	je	.LBB73_11
# %bb.7:                                # %bb0.i
	movq	-8(%rdi), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB73_9
# %bb.8:                                # %bb2.i
	incq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB73_11
.LBB73_29:                              # %bb3.i23
	je	.LBB73_31
# %bb.30:                               # %bb4.i25
	movq	%r13, %rdi
	movq	%rsi, 16(%rsp)                  # 8-byte Spill
	callq	cinch_dup_mt@PLT
	movq	16(%rsp), %rsi                  # 8-byte Reload
.LBB73_31:                              # %bb18
	cmpq	$2, %rbp
	jb	.LBB73_37
# %bb.32:                               # %bb18
	cmpq	$0, 8(%rsp)                     # 8-byte Folded Reload
	je	.LBB73_37
# %bb.33:                               # %bb0.i30
	movq	8(%rsp), %rdx                   # 8-byte Reload
	movq	-8(%rdx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB73_35
# %bb.34:                               # %bb2.i39
	incq	%rax
	movq	%rax, -8(%rdx)
	jmp	.LBB73_37
.LBB73_9:                               # %bb3.i
	je	.LBB73_11
# %bb.10:                               # %bb4.i
	callq	cinch_dup_mt@PLT
	movq	%r13, %rax
.LBB73_11:                              # %bb8
	cmpq	$1, %rbp
	je	.LBB73_17
# %bb.12:                               # %bb9
	movq	32(%rax), %rdi
	testq	%rdi, %rdi
	je	.LBB73_17
# %bb.13:                               # %bb0.i4
	movq	-8(%rdi), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB73_15
# %bb.14:                               # %bb2.i13
	incq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB73_17
.LBB73_35:                              # %bb3.i36
	je	.LBB73_37
# %bb.36:                               # %bb4.i38
	movq	8(%rsp), %rdi                   # 8-byte Reload
	movq	%rsi, %rbp
	callq	cinch_dup_mt@PLT
	movq	%rbp, %rsi
.LBB73_37:                              # %bb20
	testq	%r12, %r12
	jle	.LBB73_39
# %bb.38:                               # %bb21
	decq	%rsi
	movq	%rsi, -8(%rbx)
	movq	8(%rsp), %r12                   # 8-byte Reload
	jmp	.LBB73_41
.LBB73_39:                              # %bb22
	movq	8(%rsp), %r12                   # 8-byte Reload
	je	.LBB73_41
# %bb.40:                               # %bb23
	movq	%rbx, %rdi
	callq	cinch_drop_mt@PLT
.LBB73_41:                              # %bb16
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	"Data.String.Eq_String.=="@PLT
	testq	%r13, %r13
	je	.LBB73_49
# %bb.42:                               # %bb0.i43
	movq	-8(%r13), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB73_44
# %bb.43:                               # %bb2.i51
	decq	%rdx
	movq	%rdx, -8(%r13)
	jmp	.LBB73_49
.LBB73_44:                              # %bb3.i49
	testq	%rcx, %rcx
	je	.LBB73_49
# %bb.45:                               # %bb3.i49
	movq	%rax, %rbx
	movq	%r13, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB73_47
# %bb.46:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB73_48
.LBB73_47:                              # %bb4.i50
	callq	cinch_drop_mt@PLT
.LBB73_48:                              # %cinch.rc.drop.obj.exit
	movq	%rbx, %rax
.LBB73_49:                              # %cinch.rc.drop.obj.exit
	testq	%r12, %r12
	je	.LBB73_57
# %bb.50:                               # %bb0.i54
	movq	-8(%r12), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB73_52
# %bb.51:                               # %bb2.i63
	decq	%rdx
	movq	%rdx, -8(%r12)
	jmp	.LBB73_57
.LBB73_52:                              # %bb3.i60
	testq	%rcx, %rcx
	je	.LBB73_57
# %bb.53:                               # %bb3.i60
	movq	%rax, %rbx
	movq	%r12, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB73_55
# %bb.54:                               # %bb5.i61
	callq	cinch_drop_slow@PLT
	jmp	.LBB73_56
.LBB73_55:                              # %bb4.i62
	callq	cinch_drop_mt@PLT
.LBB73_56:                              # %cinch.rc.drop.obj.exit65
	movq	%rbx, %rax
.LBB73_57:                              # %cinch.rc.drop.obj.exit65
	addq	$-2, %r14
	je	.LBB73_22
# %bb.58:                               # %bb24
	addq	$16, %r15
	movq	%rax, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*(%rax)
	jmp	.LBB73_22
.LBB73_15:                              # %bb3.i10
	je	.LBB73_17
# %bb.16:                               # %bb4.i12
	callq	cinch_dup_mt@PLT
	movq	%r13, %rax
.LBB73_17:                              # %bb10
	testq	%r12, %r12
	jle	.LBB73_19
# %bb.18:                               # %bb11
	decq	%r15
	movq	%r15, -8(%rbx)
.LBB73_22:                              # %common.ret
	addq	$152, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq	$8
.LBB73_19:                              # %bb12
	.cfi_def_cfa_offset 208
	je	.LBB73_22
# %bb.20:                               # %bb13
	movq	%rbx, %rdi
	callq	cinch_drop_mt@PLT
	jmp	.LBB73_21
.Lfunc_end73:
	.size	"Data.String.Eq_String.==.tapp", .Lfunc_end73-"Data.String.Eq_String.==.tapp"
	.cfi_endproc
                                        # -- End function
	.globl	"Text.Show.wrap-bytes"          # -- Begin function Text.Show.wrap-bytes
	.p2align	4
	.type	"Text.Show.wrap-bytes",@function
"Text.Show.wrap-bytes":                 # @Text.Show.wrap-bytes
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	movl	$1, %esi
	callq	"Data.String.utf8-valid-from"@PLT
	testq	%rax, %rax
	je	.LBB74_17
# %bb.1:                                # %bb2
	movabsq	$1030792151040, %r12            # imm = 0xF000000000
	movabsq	$274877906944, %r13             # imm = 0x4000000000
	testq	%rbx, %rbx
	je	.LBB74_4
# %bb.2:                                # %bb0.i12
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB74_4
# %bb.3:                                # %bb1.i25
	movq	(%rbx), %rax
	jmp	.LBB74_5
.LBB74_4:                               # %bb2.i20
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB74_5:                               # %cinch.runtime.bytesLen.exit27
	leaq	1(,%rax,2), %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r14
	movabsq	$9223372036854775807, %r15      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rbx, %rbx
	je	.LBB74_15
# %bb.6:                                # %bb0.i
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	%r13, %rax
	jne	.LBB74_8
# %bb.7:                                # %cinch.runtime.bytesLen.exit.thread
	movq	(%rbx), %r8
	jmp	.LBB74_9
.LBB74_17:                              # %bb3
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	testq	%rbx, %rbx
	je	.LBB74_25
# %bb.18:                               # %bb0.i65
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB74_20
# %bb.19:                               # %bb2.i75
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB74_25
.LBB74_15:                              # %cinch.rc.drop.obj.exit.critedge
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%rax, %r15
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%r15, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB74_16
.LBB74_8:                               # %cinch.runtime.bytesLen.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB74_9:                               # %bb0.i38
	andq	%r15, %r8
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB74_11
# %bb.10:                               # %bb2.i45
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB74_16
.LBB74_11:                              # %bb3.i
	testq	%rax, %rax
	je	.LBB74_16
# %bb.12:                               # %bb3.i
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB74_14
# %bb.13:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB74_16
.LBB74_20:                              # %bb3.i71
	testq	%rcx, %rcx
	je	.LBB74_25
# %bb.21:                               # %bb3.i71
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB74_23
# %bb.22:                               # %bb5.i72
	callq	cinch_drop_slow@PLT
	jmp	.LBB74_24
.LBB74_14:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB74_16:                              # %cinch.rc.drop.obj.exit
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r14, (%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
.LBB74_25:                              # %bb1
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB74_27
# %bb.26:                               # %bb5
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB74_27:                              # %bb7
	testq	%rcx, %rcx
	je	.LBB74_31
# %bb.28:                               # %bb7
	cmpq	$1, %rcx
	jne	.LBB74_29
# %bb.38:                               # %cinch.rc.unique.exit
	movq	(%rax), %rbx
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB74_40
# %bb.39:                               # %bb10
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB74_30
.LBB74_31:                              # %bb0.i79
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB74_33
# %bb.32:                               # %bb2.i89
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB74_37
.LBB74_29:
	xorl	%ebx, %ebx
	jmp	.LBB74_30
.LBB74_40:                              # %bb11
	testq	%rbx, %rbx
	je	.LBB74_45
# %bb.41:                               # %bb0.i98
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB74_43
# %bb.42:                               # %bb2.i107
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB74_45
.LBB74_33:                              # %bb3.i85
	testq	%rcx, %rcx
	je	.LBB74_37
# %bb.34:                               # %bb3.i85
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB74_36
# %bb.35:                               # %bb5.i86
	callq	cinch_drop_slow@PLT
	jmp	.LBB74_37
.LBB74_36:                              # %bb4.i88
	callq	cinch_drop_mt@PLT
.LBB74_37:                              # %cinch.rc.drop.obj.exit91
	movl	$1, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r14
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r14, (%rax)
.LBB74_30:                              # %common.ret
	movq	%rbx, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB74_43:                              # %bb3.i104
	.cfi_def_cfa %rbp, 16
	je	.LBB74_45
# %bb.44:                               # %bb4.i105
	movq	%rbx, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
.LBB74_45:                              # %bb0.i110
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB74_47
# %bb.46:                               # %bb2.i120
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB74_30
.LBB74_47:                              # %bb3.i116
	testq	%rcx, %rcx
	je	.LBB74_30
# %bb.48:                               # %bb3.i116
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB74_50
# %bb.49:                               # %bb5.i117
	callq	cinch_drop_slow@PLT
	jmp	.LBB74_30
.LBB74_23:                              # %bb4.i74
	callq	cinch_drop_mt@PLT
.LBB74_24:                              # %bb1
	movq	%r14, %rax
	jmp	.LBB74_25
.LBB74_50:                              # %bb4.i119
	callq	cinch_drop_mt@PLT
	jmp	.LBB74_30
.Lfunc_end74:
	.size	"Text.Show.wrap-bytes", .Lfunc_end74-"Text.Show.wrap-bytes"
	.cfi_endproc
                                        # -- End function
	.globl	"Text.Show.nat-digits-into"     # -- Begin function Text.Show.nat-digits-into
	.p2align	4
	.type	"Text.Show.nat-digits-into",@function
"Text.Show.nat-digits-into":            # @Text.Show.nat-digits-into
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
	movq	%rcx, %r15
	movq	%rdx, %r13
	movq	%rdi, -48(%rbp)                 # 8-byte Spill
	movl	%esi, %eax
	andl	$1, %eax
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movq	%rsi, -56(%rbp)                 # 8-byte Spill
	shrq	%rsi
	movq	%rsi, -64(%rbp)                 # 8-byte Spill
	jmp	.LBB75_2
	.p2align	4
.LBB75_1:                               # %bb2.i506
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
.LBB75_2:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	testb	$1, %r15b
	jne	.LBB75_5
# %bb.3:                                # %cinch.runtime.natCmp.exit185
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$1, %esi
	movq	%r15, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB75_143
# %bb.4:                                # %bb1.i116
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r12
	testl	%r12d, -72(%rbp)                # 4-byte Folded Reload
	jne	.LBB75_7
	jmp	.LBB75_9
	.p2align	4
.LBB75_5:                               # %bb0.i179
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	$2, %r15
	jb	.LBB75_143
# %bb.6:                                # %bb3.i111
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	-2(%r15), %r12
	testl	%r12d, -72(%rbp)                # 4-byte Folded Reload
	je	.LBB75_9
.LBB75_7:                               # %bb0.i52
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rax
	shrq	%rax
	addq	-64(%rbp), %rax                 # 8-byte Folded Reload
	js	.LBB75_9
# %bb.8:                                # %bb3.i57
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	1(,%rax,2), %rbx
	jmp	.LBB75_10
	.p2align	4
.LBB75_9:                               # %bb1.i61
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	movq	%r12, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
.LBB75_10:                              # %cinch.runtime.natAdd.exit63
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_11
.LBB75_16:                              # %cinch.rc.drop.tobj.exit285
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r13b
	jne	.LBB75_19
.LBB75_17:                              # %bb1.i158
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$21, %esi
	movq	%r13, %rdi
	callq	cinch_nat_rem@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	jne	.LBB75_20
	jmp	.LBB75_22
	.p2align	4
.LBB75_11:                              # %bb0.i273
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_13
# %bb.12:                               # %bb2.i283
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	$1, %r13b
	je	.LBB75_17
.LBB75_19:                              # %bb0.i161
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rcx
	shrq	%rcx
	movq	%rcx, %rax
	movabsq	$7378697629483820647, %rdx      # imm = 0x6666666666666667
	mulq	%rdx
	shrq	$2, %rdx
	addl	%edx, %edx
	leal	(%rdx,%rdx,4), %eax
	subl	%eax, %ecx
	leaq	1(,%rcx,2), %r14
	testb	$1, %r14b
	je	.LBB75_22
.LBB75_20:                              # %bb0.i38
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rax
	shrq	%rax
	addq	$48, %rax
	js	.LBB75_22
# %bb.21:                               # %cinch.runtime.natAdd.exit48.thread
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	1(,%rax,2), %r12
	testb	$1, %bl
	je	.LBB75_29
.LBB75_32:                              # %bb0.i201
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %r14
	shrq	%r14
	testb	$1, %r12b
	jne	.LBB75_30
.LBB75_33:                              # %bb4.i229
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rdx
	jmp	.LBB75_34
	.p2align	4
.LBB75_22:                              # %cinch.runtime.natAdd.exit48
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$97, %edi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_23
.LBB75_28:                              # %cinch.rc.drop.tobj.exit302
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %bl
	jne	.LBB75_32
.LBB75_29:                              # %bb1.i231
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r14
	testb	$1, %r12b
	je	.LBB75_33
.LBB75_30:                              # %bb3.i207
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rdx
	shrq	%rdx
.LBB75_34:                              # %bb5.i209
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB75_39
# %bb.35:                               # %bb6.i212
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB75_39
# %bb.36:                               # %bb7.i219
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	$255, %rdx
	ja	.LBB75_39
# %bb.37:                               # %bb7.i219
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	(%rdi), %r14
	jae	.LBB75_39
# %bb.38:                               # %bb10.i224
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	8(%rdi), %rax
	movb	%dl, (%rax,%r14)
	jmp	.LBB75_40
	.p2align	4
.LBB75_39:                              # %bb8.i217
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rsi
	callq	cinch_bytes_set@PLT
.LBB75_40:                              # %cinch.runtime.bytesSet.exit233
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_47
# %bb.41:                               # %bb0.i307
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_43
# %bb.42:                               # %bb2.i317
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB75_47
	.p2align	4
.LBB75_23:                              # %bb0.i290
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_25
# %bb.24:                               # %bb2.i300
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	testb	$1, %bl
	je	.LBB75_29
	jmp	.LBB75_32
	.p2align	4
.LBB75_13:                              # %bb3.i279
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_16
# %bb.14:                               # %bb3.i279
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_18
# %bb.15:                               # %bb5.i280
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_16
	.p2align	4
.LBB75_43:                              # %bb3.i313
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_47
# %bb.44:                               # %bb3.i313
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_46
# %bb.45:                               # %bb5.i314
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_47
.LBB75_25:                              # %bb3.i296
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_28
# %bb.26:                               # %bb3.i296
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_31
# %bb.27:                               # %bb5.i297
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_28
.LBB75_18:                              # %bb4.i282
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r13b
	je	.LBB75_17
	jmp	.LBB75_19
.LBB75_46:                              # %bb4.i316
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB75_47:                              # %cinch.rc.drop.tobj.exit319
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_48
.LBB75_53:                              # %cinch.rc.drop.tobj.exit336
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r13b
	jne	.LBB75_61
.LBB75_54:                              # %cinch.runtime.natDiv.exit141
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$21, %esi
	movq	%r13, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rbx
	testq	%r13, %r13
	je	.LBB75_62
# %bb.55:                               # %bb0.i341
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_57
# %bb.56:                               # %bb2.i351
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	testb	$1, %r15b
	je	.LBB75_63
	.p2align	4
.LBB75_65:                              # %bb3.i97
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	-2(%r15), %r12
	jmp	.LBB75_66
	.p2align	4
.LBB75_48:                              # %bb0.i324
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_50
# %bb.49:                               # %bb2.i334
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	$1, %r13b
	je	.LBB75_54
.LBB75_61:                              # %cinch.runtime.natDiv.exit141.thread
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rax
	movabsq	$-3689348814741910323, %rcx     # imm = 0xCCCCCCCCCCCCCCCD
	mulq	%rcx
	movq	%rdx, %rbx
	shrq	$3, %rbx
	orq	$1, %rbx
.LBB75_62:                              # %cinch.rc.drop.tobj.exit353
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r15b
	jne	.LBB75_65
.LBB75_63:                              # %bb1.i102
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r12
.LBB75_66:                              # %cinch.runtime.natSub.exit104
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_67
.LBB75_72:                              # %cinch.rc.drop.tobj.exit370
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r12b
	jne	.LBB75_76
.LBB75_73:                              # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$1, %esi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	je	.LBB75_146
# %bb.74:                               # %bb1.i88
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$3, %esi
	movq	%r12, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r13
	testl	%r13d, -72(%rbp)                # 4-byte Folded Reload
	jne	.LBB75_78
	jmp	.LBB75_80
	.p2align	4
.LBB75_67:                              # %bb0.i358
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_69
# %bb.68:                               # %bb2.i368
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	testb	$1, %r12b
	je	.LBB75_73
.LBB75_76:                              # %bb0.i170
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	$2, %r12
	jb	.LBB75_146
# %bb.77:                               # %bb3.i83
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	-2(%r12), %r13
	testl	%r13d, -72(%rbp)                # 4-byte Folded Reload
	je	.LBB75_80
.LBB75_78:                              # %bb0.i24
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rax
	shrq	%rax
	addq	-64(%rbp), %rax                 # 8-byte Folded Reload
	js	.LBB75_80
# %bb.79:                               # %bb3.i28
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	1(,%rax,2), %r15
	jmp	.LBB75_81
	.p2align	4
.LBB75_80:                              # %bb1.i32
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	movq	%r13, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r15
.LBB75_81:                              # %cinch.runtime.natAdd.exit34
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_82
.LBB75_87:                              # %cinch.rc.drop.tobj.exit423
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %bl
	jne	.LBB75_90
.LBB75_88:                              # %bb1.i146
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$21, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_rem@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	jne	.LBB75_91
	jmp	.LBB75_93
	.p2align	4
.LBB75_82:                              # %bb0.i411
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_84
# %bb.83:                               # %bb2.i421
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	testb	$1, %bl
	je	.LBB75_88
.LBB75_90:                              # %bb0.i149
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%rcx, %rax
	movabsq	$7378697629483820647, %rdx      # imm = 0x6666666666666667
	mulq	%rdx
	shrq	$2, %rdx
	addl	%edx, %edx
	leal	(%rdx,%rdx,4), %eax
	subl	%eax, %ecx
	leaq	1(,%rcx,2), %r14
	testb	$1, %r14b
	je	.LBB75_93
.LBB75_91:                              # %bb0.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rax
	shrq	%rax
	addq	$48, %rax
	js	.LBB75_93
# %bb.92:                               # %cinch.runtime.natAdd.exit.thread
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	1(,%rax,2), %r13
	testb	$1, %r15b
	je	.LBB75_100
.LBB75_103:                             # %bb0.i187
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r15, %r14
	shrq	%r14
	testb	$1, %r13b
	jne	.LBB75_101
.LBB75_104:                             # %bb4.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rdx
	jmp	.LBB75_105
	.p2align	4
.LBB75_93:                              # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$97, %edi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r13
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_94
.LBB75_99:                              # %cinch.rc.drop.tobj.exit440
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r15b
	jne	.LBB75_103
.LBB75_100:                             # %bb1.i197
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r15, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %r14
	testb	$1, %r13b
	je	.LBB75_104
.LBB75_101:                             # %bb3.i191
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rdx
	shrq	%rdx
.LBB75_105:                             # %bb5.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB75_110
# %bb.106:                              # %bb6.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%rdi), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB75_110
# %bb.107:                              # %bb7.i
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	$255, %rdx
	ja	.LBB75_110
# %bb.108:                              # %bb7.i
                                        #   in Loop: Header=BB75_2 Depth=1
	cmpq	(%rdi), %r14
	jae	.LBB75_110
# %bb.109:                              # %bb10.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	8(%rdi), %rax
	movb	%dl, (%rax,%r14)
	jmp	.LBB75_111
	.p2align	4
.LBB75_110:                             # %bb8.i
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rsi
	callq	cinch_bytes_set@PLT
.LBB75_111:                             # %cinch.runtime.bytesSet.exit
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_118
# %bb.112:                              # %bb0.i445
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_114
# %bb.113:                              # %bb2.i455
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB75_118
	.p2align	4
.LBB75_94:                              # %bb0.i428
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_96
# %bb.95:                               # %bb2.i438
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	testb	$1, %r15b
	je	.LBB75_100
	jmp	.LBB75_103
	.p2align	4
.LBB75_50:                              # %bb3.i330
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_53
# %bb.51:                               # %bb3.i330
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_60
# %bb.52:                               # %bb5.i331
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_53
	.p2align	4
.LBB75_69:                              # %bb3.i364
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_72
# %bb.70:                               # %bb3.i364
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_75
# %bb.71:                               # %bb5.i365
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_72
	.p2align	4
.LBB75_84:                              # %bb3.i417
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_87
# %bb.85:                               # %bb3.i417
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_89
# %bb.86:                               # %bb5.i418
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_87
	.p2align	4
.LBB75_114:                             # %bb3.i451
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_118
# %bb.115:                              # %bb3.i451
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_117
# %bb.116:                              # %bb5.i452
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_118
.LBB75_96:                              # %bb3.i434
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_99
# %bb.97:                               # %bb3.i434
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_102
# %bb.98:                               # %bb5.i435
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_99
.LBB75_57:                              # %bb3.i347
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_62
# %bb.58:                               # %bb3.i347
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_64
# %bb.59:                               # %bb5.i348
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	testb	$1, %r15b
	je	.LBB75_63
	jmp	.LBB75_65
.LBB75_60:                              # %bb4.i333
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r13b
	je	.LBB75_54
	jmp	.LBB75_61
.LBB75_75:                              # %bb4.i367
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r12b
	je	.LBB75_73
	jmp	.LBB75_76
.LBB75_89:                              # %bb4.i420
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %bl
	je	.LBB75_88
	jmp	.LBB75_90
.LBB75_117:                             # %bb4.i454
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB75_118:                             # %cinch.rc.drop.tobj.exit457
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB75_119
.LBB75_124:                             # %cinch.rc.drop.tobj.exit474
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %bl
	jne	.LBB75_132
.LBB75_125:                             # %cinch.runtime.natDiv.exit
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$21, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %r13
	testq	%rbx, %rbx
	je	.LBB75_133
# %bb.126:                              # %bb0.i479
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_128
# %bb.127:                              # %bb2.i489
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	testb	$1, %r12b
	je	.LBB75_134
	.p2align	4
.LBB75_136:                             # %bb3.i70
                                        #   in Loop: Header=BB75_2 Depth=1
	leaq	-2(%r12), %r15
	jmp	.LBB75_137
	.p2align	4
.LBB75_119:                             # %bb0.i462
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_121
# %bb.120:                              # %bb2.i472
                                        #   in Loop: Header=BB75_2 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	testb	$1, %bl
	je	.LBB75_125
.LBB75_132:                             # %cinch.runtime.natDiv.exit.thread
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %rax
	movabsq	$-3689348814741910323, %rcx     # imm = 0xCCCCCCCCCCCCCCCD
	mulq	%rcx
	movq	%rdx, %r13
	shrq	$3, %r13
	orq	$1, %r13
.LBB75_133:                             # %cinch.rc.drop.tobj.exit491
                                        #   in Loop: Header=BB75_2 Depth=1
	testb	$1, %r12b
	jne	.LBB75_136
.LBB75_134:                             # %bb1.i75
                                        #   in Loop: Header=BB75_2 Depth=1
	movl	$3, %esi
	movq	%r12, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r15
.LBB75_137:                             # %cinch.runtime.natSub.exit
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_2
# %bb.138:                              # %bb0.i496
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jge	.LBB75_1
# %bb.139:                              # %bb3.i502
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_2
# %bb.140:                              # %bb3.i502
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_142
# %bb.141:                              # %bb5.i503
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_2
	.p2align	4
.LBB75_121:                             # %bb3.i468
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_124
# %bb.122:                              # %bb3.i468
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_131
# %bb.123:                              # %bb5.i469
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_124
.LBB75_128:                             # %bb3.i485
                                        #   in Loop: Header=BB75_2 Depth=1
	testq	%rax, %rax
	je	.LBB75_133
# %bb.129:                              # %bb3.i485
                                        #   in Loop: Header=BB75_2 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_135
# %bb.130:                              # %bb5.i486
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_slow@PLT
	testb	$1, %r12b
	je	.LBB75_134
	jmp	.LBB75_136
.LBB75_142:                             # %bb4.i505
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB75_2
.LBB75_131:                             # %bb4.i471
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %bl
	je	.LBB75_125
	jmp	.LBB75_132
.LBB75_31:                              # %bb4.i299
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %bl
	je	.LBB75_29
	jmp	.LBB75_32
.LBB75_102:                             # %bb4.i437
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r15b
	je	.LBB75_100
	jmp	.LBB75_103
.LBB75_64:                              # %bb4.i350
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r15b
	je	.LBB75_63
	jmp	.LBB75_65
.LBB75_135:                             # %bb4.i488
                                        #   in Loop: Header=BB75_2 Depth=1
	callq	cinch_drop_mt@PLT
	testb	$1, %r12b
	je	.LBB75_134
	jmp	.LBB75_136
.LBB75_143:                             # %bb1
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_156
# %bb.144:                              # %bb0.i240
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_149
# %bb.145:                              # %bb2.i250
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB75_156
.LBB75_146:                             # %bb3
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_163
# %bb.147:                              # %bb0.i377
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_152
# %bb.148:                              # %bb2.i387
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB75_163
.LBB75_149:                             # %bb3.i246
	testq	%rax, %rax
	je	.LBB75_156
# %bb.150:                              # %bb3.i246
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_155
# %bb.151:                              # %bb5.i247
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_156
.LBB75_152:                             # %bb3.i383
	testq	%rax, %rax
	je	.LBB75_163
# %bb.153:                              # %bb3.i383
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_162
# %bb.154:                              # %bb5.i384
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_163
.LBB75_155:                             # %bb4.i249
	callq	cinch_drop_mt@PLT
.LBB75_156:                             # %cinch.rc.drop.tobj.exit
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_170
# %bb.157:                              # %bb0.i256
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_159
# %bb.158:                              # %bb2.i266
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB75_170
.LBB75_159:                             # %bb3.i262
	testq	%rax, %rax
	je	.LBB75_170
# %bb.160:                              # %bb3.i262
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB75_169
.LBB75_168:                             # %bb5.i401
	callq	cinch_drop_slow@PLT
	jmp	.LBB75_170
.LBB75_162:                             # %bb4.i386
	callq	cinch_drop_mt@PLT
.LBB75_163:                             # %cinch.rc.drop.tobj.exit389
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB75_170
# %bb.164:                              # %bb0.i394
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB75_166
# %bb.165:                              # %bb2.i404
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB75_170
.LBB75_166:                             # %bb3.i400
	testq	%rax, %rax
	je	.LBB75_170
# %bb.167:                              # %bb3.i400
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB75_168
.LBB75_169:                             # %bb4.i403
	callq	cinch_drop_mt@PLT
.LBB75_170:                             # %common.ret
	xorl	%eax, %eax
	addq	$40, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end75:
	.size	"Text.Show.nat-digits-into", .Lfunc_end75-"Text.Show.nat-digits-into"
	.cfi_endproc
                                        # -- End function
	.globl	"Text.Show.nat-digit-count"     # -- Begin function Text.Show.nat-digit-count
	.p2align	4
	.type	"Text.Show.nat-digit-count",@function
"Text.Show.nat-digit-count":            # @Text.Show.nat-digit-count
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, %r14
	testb	$1, %r14b
	jne	.LBB76_5
# %bb.1:                                # %cinch.runtime.natCmp.exit101.thread
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB76_19
# %bb.2:                                # %cinch.runtime.natDiv.exit73
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB76_7
# %bb.3:                                # %bb0.i121
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_29
# %bb.4:                                # %bb2.i131
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB76_7
.LBB76_5:                               # %cinch.runtime.natCmp.exit101
	movl	$3, %eax
	cmpq	$19, %r14
	jbe	.LBB76_81
# %bb.6:                                # %cinch.runtime.natDiv.exit73.thread
	movabsq	$-3689348814741910323, %rcx     # imm = 0xCCCCCCCCCCCCCCCD
	movq	%r14, %rax
	mulq	%rcx
	movq	%rdx, %rbx
	shrq	$3, %rbx
	orq	$1, %rbx
.LBB76_7:                               # %cinch.rc.drop.tobj.exit133
	testb	$1, %bl
	jne	.LBB76_12
# %bb.8:                                # %cinch.runtime.natCmp.exit91.thread
	movl	$21, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB76_26
# %bb.9:                                # %cinch.runtime.natDiv.exit60
	movl	$21, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	je	.LBB76_14
# %bb.10:                               # %bb0.i157
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_35
# %bb.11:                               # %bb2.i167
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB76_14
.LBB76_12:                              # %cinch.runtime.natCmp.exit91
	movl	$2, %r15d
	cmpq	$19, %rbx
	jbe	.LBB76_77
# %bb.13:                               # %cinch.runtime.natDiv.exit60.thread
	movabsq	$-3689348814741910323, %rcx     # imm = 0xCCCCCCCCCCCCCCCD
	movq	%rbx, %rax
	mulq	%rcx
	movq	%rdx, %r14
	shrq	$3, %r14
	orq	$1, %r14
.LBB76_14:                              # %cinch.rc.drop.tobj.exit169
	testb	$1, %r14b
	jne	.LBB76_24
# %bb.15:                               # %cinch.runtime.natCmp.exit.thread
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB76_32
# %bb.16:                               # %cinch.runtime.natDiv.exit
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rdi
	testq	%r14, %r14
	je	.LBB76_53
# %bb.17:                               # %bb0.i193
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_42
# %bb.18:                               # %bb2.i203
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB76_53
.LBB76_19:                              # %bb1
	movl	$3, %eax
	testq	%r14, %r14
	je	.LBB76_81
# %bb.20:                               # %bb0.i108
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jge	.LBB76_80
# %bb.21:                               # %bb3.i114
	testq	%rcx, %rcx
	je	.LBB76_81
# %bb.22:                               # %bb3.i114
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB76_45
# %bb.23:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	movl	$3, %eax
	jmp	.LBB76_81
.LBB76_24:                              # %cinch.runtime.natCmp.exit
	movl	$2, %r15d
	cmpq	$19, %r14
	jbe	.LBB76_74
# %bb.25:                               # %cinch.runtime.natDiv.exit.thread
	movabsq	$-3689348814741910323, %rcx     # imm = 0xCCCCCCCCCCCCCCCD
	movq	%r14, %rax
	mulq	%rcx
	movq	%rdx, %rdi
	shrq	$3, %rdi
	orq	$1, %rdi
	jmp	.LBB76_53
.LBB76_26:                              # %bb4
	movl	$2, %r15d
	testq	%rbx, %rbx
	je	.LBB76_77
# %bb.27:                               # %bb0.i140
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_38
# %bb.28:                               # %bb2.i150
	decq	%rcx
	movq	%rcx, -8(%rbx)
	leaq	1(,%r15,2), %rax
	jmp	.LBB76_81
.LBB76_29:                              # %bb3.i127
	testq	%rax, %rax
	je	.LBB76_7
# %bb.30:                               # %bb3.i127
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_41
# %bb.31:                               # %bb5.i128
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_7
.LBB76_32:                              # %bb7
	movl	$2, %r15d
	testq	%r14, %r14
	je	.LBB76_74
# %bb.33:                               # %bb0.i176
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_46
# %bb.34:                               # %bb2.i186
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB76_74
.LBB76_35:                              # %bb3.i163
	testq	%rax, %rax
	je	.LBB76_14
# %bb.36:                               # %bb3.i163
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_49
# %bb.37:                               # %bb5.i164
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_14
.LBB76_38:                              # %bb3.i146
	testq	%rax, %rax
	je	.LBB76_77
# %bb.39:                               # %bb3.i146
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_50
# %bb.40:                               # %bb5.i147
	callq	cinch_drop_slow@PLT
	leaq	1(,%r15,2), %rax
	jmp	.LBB76_81
.LBB76_41:                              # %bb4.i130
	callq	cinch_drop_mt@PLT
	jmp	.LBB76_7
.LBB76_42:                              # %bb3.i199
	testq	%rax, %rax
	je	.LBB76_53
# %bb.43:                               # %bb3.i199
	movq	%rdi, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_51
# %bb.44:                               # %bb5.i200
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_52
.LBB76_45:                              # %bb4.i
	callq	cinch_drop_mt@PLT
	movl	$3, %eax
	jmp	.LBB76_81
.LBB76_46:                              # %bb3.i182
	testq	%rax, %rax
	je	.LBB76_74
# %bb.47:                               # %bb3.i182
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_73
# %bb.48:                               # %bb5.i183
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_74
.LBB76_49:                              # %bb4.i166
	callq	cinch_drop_mt@PLT
	jmp	.LBB76_14
.LBB76_50:                              # %bb4.i149
	callq	cinch_drop_mt@PLT
	leaq	1(,%r15,2), %rax
	jmp	.LBB76_81
.LBB76_51:                              # %bb4.i202
	callq	cinch_drop_mt@PLT
.LBB76_52:                              # %cinch.rc.drop.tobj.exit205
	movq	%rbx, %rdi
.LBB76_53:                              # %cinch.rc.drop.tobj.exit205
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	je	.LBB76_56
# %bb.54:                               # %bb0.i27
	movq	%r14, %rax
	shrq	%rax
	incq	%rax
	js	.LBB76_56
# %bb.55:                               # %cinch.runtime.natAdd.exit37.thread
	leaq	1(,%rax,2), %rbx
	jmp	.LBB76_63
.LBB76_56:                              # %cinch.runtime.natAdd.exit37
	movl	$3, %edi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB76_63
# %bb.57:                               # %bb0.i210
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_59
# %bb.58:                               # %bb2.i220
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB76_63
.LBB76_59:                              # %bb3.i216
	testq	%rax, %rax
	je	.LBB76_63
# %bb.60:                               # %bb3.i216
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_62
# %bb.61:                               # %bb5.i217
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_63
.LBB76_62:                              # %bb4.i219
	callq	cinch_drop_mt@PLT
.LBB76_63:                              # %bb6
	testb	$1, %bl
	je	.LBB76_66
# %bb.64:                               # %bb0.i13
	movq	%rbx, %r15
	shrq	%r15
	incq	%r15
	js	.LBB76_66
.LBB76_74:                              # %bb3
	leaq	1(,%r15,2), %r14
	jmp	.LBB76_75
.LBB76_66:                              # %cinch.runtime.natAdd.exit23
	movl	$3, %edi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	sete	%al
	orb	%bl, %al
	testb	$1, %al
	jne	.LBB76_75
# %bb.67:                               # %bb0.i227
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB76_69
# %bb.68:                               # %bb2.i237
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB76_75
.LBB76_69:                              # %bb3.i233
	testq	%rax, %rax
	je	.LBB76_75
# %bb.70:                               # %bb3.i233
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB76_72
# %bb.71:                               # %bb5.i234
	callq	cinch_drop_slow@PLT
	jmp	.LBB76_75
.LBB76_72:                              # %bb4.i236
	callq	cinch_drop_mt@PLT
.LBB76_75:                              # %bb3
	testb	$1, %r14b
	je	.LBB76_78
# %bb.76:                               # %bb0.i
	movq	%r14, %r15
	shrq	%r15
	incq	%r15
	js	.LBB76_78
.LBB76_77:                              # %cinch.runtime.natAdd.exit.thread
	leaq	1(,%r15,2), %rax
	jmp	.LBB76_81
.LBB76_78:                              # %cinch.runtime.natAdd.exit
	movl	$3, %edi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	testq	%r14, %r14
	sete	%cl
	orb	%r14b, %cl
	testb	$1, %cl
	jne	.LBB76_81
# %bb.79:                               # %bb0.i244
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB76_82
.LBB76_80:                              # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%r14)
.LBB76_81:                              # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB76_82:                              # %bb3.i250
	.cfi_def_cfa %rbp, 16
	testq	%rcx, %rcx
	je	.LBB76_81
# %bb.83:                               # %bb3.i250
	movq	%rax, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB76_85
# %bb.84:                               # %bb5.i251
	callq	cinch_drop_slow@PLT
	movq	%rbx, %rax
	jmp	.LBB76_81
.LBB76_85:                              # %bb4.i253
	callq	cinch_drop_mt@PLT
	movq	%rbx, %rax
	jmp	.LBB76_81
.LBB76_73:                              # %bb4.i185
	callq	cinch_drop_mt@PLT
	jmp	.LBB76_74
.Lfunc_end76:
	.size	"Text.Show.nat-digit-count", .Lfunc_end76-"Text.Show.nat-digit-count"
	.cfi_endproc
                                        # -- End function
	.globl	"Text.Show.nat-to-string"       # -- Begin function Text.Show.nat-to-string
	.p2align	4
	.type	"Text.Show.nat-to-string",@function
"Text.Show.nat-to-string":              # @Text.Show.nat-to-string
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	testq	%rdi, %rdi
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB77_5
# %bb.1:                                # %bb0.i
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB77_3
# %bb.2:                                # %bb2.i
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB77_5
.LBB77_3:                               # %bb3.i
	je	.LBB77_5
# %bb.4:                                # %bb4.i
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB77_5:                               # %cinch.rc.dup.tobj.exit
	movq	%rbx, %rdi
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r14
	testq	%rax, %rax
	sete	%al
	movl	%r14d, %r12d
	orb	%al, %r12b
	testb	$1, %r12b
	jne	.LBB77_10
# %bb.6:                                # %bb0.i10
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB77_8
# %bb.7:                                # %bb2.i19
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB77_10
.LBB77_8:                               # %bb3.i16
	je	.LBB77_10
# %bb.9:                                # %bb4.i18
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB77_10:                              # %cinch.rc.dup.tobj.exit21
	movq	%r14, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testb	$1, %r12b
	jne	.LBB77_17
# %bb.11:                               # %bb0.i.i
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB77_13
# %bb.12:                               # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB77_17
.LBB77_13:                              # %bb3.i.i
	testq	%rax, %rax
	je	.LBB77_17
# %bb.14:                               # %bb3.i.i
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB77_16
# %bb.15:                               # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB77_17
.LBB77_16:                              # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB77_17:                              # %cinch.runtime.bytesNew.exit
	movl	$1, %esi
	movq	%r15, %rdi
	movq	%rbx, %rdx
	movq	%r14, %rcx
	callq	"Text.Show.nat-digits-into"@PLT
	movq	%r15, %rdi
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Text.Show.wrap-bytes"@PLT      # TAILCALL
.Lfunc_end77:
	.size	"Text.Show.nat-to-string", .Lfunc_end77-"Text.Show.nat-to-string"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.Bytes.Word.bytes-write-le64" # -- Begin function Data.Bytes.Word.bytes-write-le64
	.p2align	4
	.type	"Data.Bytes.Word.bytes-write-le64",@function
"Data.Bytes.Word.bytes-write-le64":     # @Data.Bytes.Word.bytes-write-le64
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdx, %rbx
	movq	%rsi, %r15
	movq	%rdi, %r14
	testq	%rsi, %rsi
	sete	%al
	movl	%r15d, %r13d
	orb	%al, %r13b
	testb	$1, %r13b
	je	.LBB78_1
# %bb.7:                                # %cinch.runtime.natFromI64.exit168
	movzbl	%bl, %r12d
	addl	%r12d, %r12d
	testb	$1, %r15b
	je	.LBB78_6
# %bb.8:                                # %bb0.i393
	movq	%r15, %rsi
	shrq	%rsi
	jmp	.LBB78_9
.LBB78_1:                               # %bb0.i430
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB78_3
# %bb.2:                                # %bb2.i439
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB78_5
.LBB78_3:                               # %bb3.i436
	je	.LBB78_5
# %bb.4:                                # %bb4.i437
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB78_5:                               # %cinch.runtime.natFromI64.exit168.thread
	movzbl	%bl, %r12d
	addl	%r12d, %r12d
.LBB78_6:                               # %bb1.i423
	movq	%r15, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_9:                               # %bb3.i399
	shrl	%r12d
	testq	%r14, %r14
	je	.LBB78_13
# %bb.10:                               # %bb6.i404
	movq	-8(%r14), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB78_13
# %bb.11:                               # %bb7.i411
	cmpq	(%r14), %rsi
	jae	.LBB78_13
# %bb.12:                               # %bb10.i416
	movq	8(%r14), %rax
	movb	%r12b, (%rax,%rsi)
	jmp	.LBB78_14
.LBB78_13:                              # %bb8.i409
	movq	%r14, %rdi
	movq	%r12, %rdx
	callq	cinch_bytes_set@PLT
.LBB78_14:                              # %cinch.runtime.bytesSet.exit425
	testb	$1, %r13b
	movb	%r13b, -41(%rbp)                # 1-byte Spill
	je	.LBB78_15
# %bb.20:                               # %cinch.rc.drop.tobj.exit474
	movb	$1, %r13b
	testb	$1, %r15b
	je	.LBB78_25
# %bb.21:                               # %bb0.i97
	movq	%r15, %rax
	shrq	%rax
	incq	%rax
	js	.LBB78_22
# %bb.23:                               # %bb3.i101
	leaq	1(,%rax,2), %r12
	xorl	%r13d, %r13d
	testb	$1, %r12b
	je	.LBB78_28
.LBB78_27:                              # %bb0.i358
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_29
.LBB78_15:                              # %bb0.i446
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_17
# %bb.16:                               # %bb2.i456
	decq	%rcx
	movq	%rcx, -8(%r15)
	movb	$1, %r13b
	jmp	.LBB78_25
.LBB78_17:                              # %bb3.i452
	movb	$1, %r13b
	testq	%rax, %rax
	je	.LBB78_25
# %bb.18:                               # %bb3.i452
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_24
# %bb.19:                               # %bb5.i453
	callq	cinch_drop_slow@PLT
	jmp	.LBB78_25
.LBB78_22:
	xorl	%r13d, %r13d
	jmp	.LBB78_25
.LBB78_24:                              # %bb4.i455
	callq	cinch_drop_mt@PLT
.LBB78_25:                              # %bb1.i105
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_27
.LBB78_28:                              # %bb1.i388
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_29:                              # %bb5.i366
	movq	%rbx, %rax
	shrq	$8, %rax
	testq	%r14, %r14
	je	.LBB78_33
# %bb.30:                               # %bb6.i369
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_33
# %bb.31:                               # %bb7.i376
	cmpq	(%r14), %rsi
	jae	.LBB78_33
# %bb.32:                               # %bb10.i381
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_34
.LBB78_33:                              # %bb8.i374
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_34:                              # %cinch.runtime.bytesSet.exit390
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_41
# %bb.35:                               # %bb0.i481
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_37
# %bb.36:                               # %bb2.i491
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_42
	jmp	.LBB78_44
.LBB78_37:                              # %bb3.i487
	testq	%rax, %rax
	je	.LBB78_41
# %bb.38:                               # %bb3.i487
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_40
# %bb.39:                               # %bb5.i488
	callq	cinch_drop_slow@PLT
.LBB78_41:                              # %cinch.rc.drop.tobj.exit510
	testb	%r13b, %r13b
	jne	.LBB78_44
.LBB78_42:                              # %bb0.i83
	movq	%r15, %rax
	shrq	%rax
	addq	$2, %rax
	js	.LBB78_44
# %bb.43:                               # %bb3.i87
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	je	.LBB78_47
.LBB78_46:                              # %bb0.i323
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_48
.LBB78_40:                              # %bb4.i490
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_42
.LBB78_44:                              # %bb1.i91
	movl	$5, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_46
.LBB78_47:                              # %bb1.i353
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_48:                              # %bb5.i331
	movq	%rbx, %rax
	shrq	$16, %rax
	testq	%r14, %r14
	je	.LBB78_52
# %bb.49:                               # %bb6.i334
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_52
# %bb.50:                               # %bb7.i341
	cmpq	(%r14), %rsi
	jae	.LBB78_52
# %bb.51:                               # %bb10.i346
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_53
.LBB78_52:                              # %bb8.i339
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_53:                              # %cinch.runtime.bytesSet.exit355
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_60
# %bb.54:                               # %bb0.i517
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_56
# %bb.55:                               # %bb2.i527
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_61
	jmp	.LBB78_63
.LBB78_56:                              # %bb3.i523
	testq	%rax, %rax
	je	.LBB78_60
# %bb.57:                               # %bb3.i523
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_59
# %bb.58:                               # %bb5.i524
	callq	cinch_drop_slow@PLT
.LBB78_60:                              # %cinch.rc.drop.tobj.exit546
	testb	%r13b, %r13b
	jne	.LBB78_63
.LBB78_61:                              # %bb0.i69
	movq	%r15, %rax
	shrq	%rax
	addq	$3, %rax
	js	.LBB78_63
# %bb.62:                               # %bb3.i73
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	je	.LBB78_66
.LBB78_65:                              # %bb0.i288
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_67
.LBB78_59:                              # %bb4.i526
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_61
.LBB78_63:                              # %bb1.i77
	movl	$7, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_65
.LBB78_66:                              # %bb1.i318
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_67:                              # %bb5.i296
	movq	%rbx, %rax
	shrq	$24, %rax
	testq	%r14, %r14
	je	.LBB78_71
# %bb.68:                               # %bb6.i299
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_71
# %bb.69:                               # %bb7.i306
	cmpq	(%r14), %rsi
	jae	.LBB78_71
# %bb.70:                               # %bb10.i311
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_72
.LBB78_71:                              # %bb8.i304
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_72:                              # %cinch.runtime.bytesSet.exit320
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_79
# %bb.73:                               # %bb0.i553
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_75
# %bb.74:                               # %bb2.i563
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_80
	jmp	.LBB78_82
.LBB78_75:                              # %bb3.i559
	testq	%rax, %rax
	je	.LBB78_79
# %bb.76:                               # %bb3.i559
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_78
# %bb.77:                               # %bb5.i560
	callq	cinch_drop_slow@PLT
.LBB78_79:                              # %cinch.rc.drop.tobj.exit582
	testb	%r13b, %r13b
	jne	.LBB78_82
.LBB78_80:                              # %bb0.i55
	movq	%r15, %rax
	shrq	%rax
	addq	$4, %rax
	js	.LBB78_82
# %bb.81:                               # %bb3.i59
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	je	.LBB78_85
.LBB78_84:                              # %bb0.i253
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_86
.LBB78_78:                              # %bb4.i562
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_80
.LBB78_82:                              # %bb1.i63
	movl	$9, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_84
.LBB78_85:                              # %bb1.i283
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_86:                              # %bb5.i261
	movq	%rbx, %rax
	shrq	$32, %rax
	testq	%r14, %r14
	je	.LBB78_90
# %bb.87:                               # %bb6.i264
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_90
# %bb.88:                               # %bb7.i271
	cmpq	(%r14), %rsi
	jae	.LBB78_90
# %bb.89:                               # %bb10.i276
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_91
.LBB78_90:                              # %bb8.i269
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_91:                              # %cinch.runtime.bytesSet.exit285
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_98
# %bb.92:                               # %bb0.i589
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_94
# %bb.93:                               # %bb2.i599
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_99
	jmp	.LBB78_101
.LBB78_94:                              # %bb3.i595
	testq	%rax, %rax
	je	.LBB78_98
# %bb.95:                               # %bb3.i595
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_97
# %bb.96:                               # %bb5.i596
	callq	cinch_drop_slow@PLT
.LBB78_98:                              # %cinch.rc.drop.tobj.exit618
	testb	%r13b, %r13b
	jne	.LBB78_101
.LBB78_99:                              # %bb0.i41
	movq	%r15, %rax
	shrq	%rax
	addq	$5, %rax
	js	.LBB78_101
# %bb.100:                              # %bb3.i45
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	je	.LBB78_104
.LBB78_103:                             # %bb0.i218
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_105
.LBB78_97:                              # %bb4.i598
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_99
.LBB78_101:                             # %bb1.i49
	movl	$11, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_103
.LBB78_104:                             # %bb1.i248
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_105:                             # %bb5.i226
	movq	%rbx, %rax
	shrq	$40, %rax
	testq	%r14, %r14
	je	.LBB78_109
# %bb.106:                              # %bb6.i229
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_109
# %bb.107:                              # %bb7.i236
	cmpq	(%r14), %rsi
	jae	.LBB78_109
# %bb.108:                              # %bb10.i241
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_110
.LBB78_109:                             # %bb8.i234
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_110:                             # %cinch.runtime.bytesSet.exit250
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_117
# %bb.111:                              # %bb0.i625
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_113
# %bb.112:                              # %bb2.i635
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_118
	jmp	.LBB78_120
.LBB78_113:                             # %bb3.i631
	testq	%rax, %rax
	je	.LBB78_117
# %bb.114:                              # %bb3.i631
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_116
# %bb.115:                              # %bb5.i632
	callq	cinch_drop_slow@PLT
.LBB78_117:                             # %cinch.rc.drop.tobj.exit654
	testb	%r13b, %r13b
	jne	.LBB78_120
.LBB78_118:                             # %bb0.i27
	movq	%r15, %rax
	shrq	%rax
	addq	$6, %rax
	js	.LBB78_120
# %bb.119:                              # %bb3.i31
	leaq	1(,%rax,2), %r12
	testb	$1, %r12b
	je	.LBB78_123
.LBB78_122:                             # %bb0.i183
	movq	%r12, %rsi
	shrq	%rsi
	jmp	.LBB78_124
.LBB78_116:                             # %bb4.i634
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_118
.LBB78_120:                             # %bb1.i35
	movl	$13, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	jne	.LBB78_122
.LBB78_123:                             # %bb1.i213
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
.LBB78_124:                             # %bb5.i191
	movq	%rbx, %rax
	shrq	$48, %rax
	testq	%r14, %r14
	je	.LBB78_128
# %bb.125:                              # %bb6.i194
	movq	-8(%r14), %rcx
	movabsq	$1065151889408, %rdx            # imm = 0xF800000000
	andq	%rdx, %rcx
	movabsq	$274877906944, %rdx             # imm = 0x4000000000
	cmpq	%rdx, %rcx
	jne	.LBB78_128
# %bb.126:                              # %bb7.i201
	cmpq	(%r14), %rsi
	jae	.LBB78_128
# %bb.127:                              # %bb10.i206
	movq	8(%r14), %rcx
	movb	%al, (%rcx,%rsi)
	jmp	.LBB78_129
.LBB78_128:                             # %bb8.i199
	movzbl	%al, %edx
	movq	%r14, %rdi
	callq	cinch_bytes_set@PLT
.LBB78_129:                             # %cinch.runtime.bytesSet.exit215
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_136
# %bb.130:                              # %bb0.i661
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_132
# %bb.131:                              # %bb2.i671
	decq	%rcx
	movq	%rcx, -8(%r12)
	testb	%r13b, %r13b
	je	.LBB78_137
	jmp	.LBB78_139
.LBB78_132:                             # %bb3.i667
	testq	%rax, %rax
	je	.LBB78_136
# %bb.133:                              # %bb3.i667
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_135
# %bb.134:                              # %bb5.i668
	callq	cinch_drop_slow@PLT
.LBB78_136:                             # %cinch.rc.drop.tobj.exit690
	testb	%r13b, %r13b
	jne	.LBB78_139
.LBB78_137:                             # %bb0.i
	movq	%r15, %rax
	shrq	%rax
	addq	$7, %rax
	js	.LBB78_139
# %bb.138:                              # %bb3.i
	leaq	1(,%rax,2), %r12
	jmp	.LBB78_140
.LBB78_135:                             # %bb4.i670
	callq	cinch_drop_mt@PLT
	testb	%r13b, %r13b
	je	.LBB78_137
.LBB78_139:                             # %bb1.i
	movl	$15, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
.LBB78_140:                             # %cinch.runtime.natAdd.exit
	movzbl	-41(%rbp), %eax                 # 1-byte Folded Reload
	testb	$1, %al
	jne	.LBB78_147
# %bb.141:                              # %bb0.i695
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_143
# %bb.142:                              # %bb2.i705
	decq	%rcx
	movq	%rcx, -8(%r15)
	testb	$1, %r12b
	je	.LBB78_149
	jmp	.LBB78_148
.LBB78_143:                             # %bb3.i701
	testq	%rax, %rax
	je	.LBB78_147
# %bb.144:                              # %bb3.i701
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_146
# %bb.145:                              # %bb5.i702
	callq	cinch_drop_slow@PLT
.LBB78_147:                             # %cinch.runtime.natFromI64.exit
	testb	$1, %r12b
	jne	.LBB78_148
.LBB78_149:                             # %bb1.i179
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%rax, %rsi
	jmp	.LBB78_150
.LBB78_146:                             # %bb4.i704
	callq	cinch_drop_mt@PLT
	testb	$1, %r12b
	je	.LBB78_149
.LBB78_148:                             # %bb0.i170
	movq	%r12, %rsi
	shrq	%rsi
.LBB78_150:                             # %bb3.i174
	shrq	$56, %rbx
	testq	%r14, %r14
	je	.LBB78_154
# %bb.151:                              # %bb6.i
	movq	-8(%r14), %rax
	movabsq	$1065151889408, %rcx            # imm = 0xF800000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB78_154
# %bb.152:                              # %bb7.i
	cmpq	(%r14), %rsi
	jae	.LBB78_154
# %bb.153:                              # %bb10.i
	movq	8(%r14), %rax
	movb	%bl, (%rax,%rsi)
	jmp	.LBB78_155
.LBB78_154:                             # %bb8.i
	movq	%r14, %rdi
	movq	%rbx, %rdx
	callq	cinch_bytes_set@PLT
.LBB78_155:                             # %cinch.runtime.bytesSet.exit
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB78_162
# %bb.156:                              # %bb0.i714
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB78_158
# %bb.157:                              # %bb2.i724
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB78_162
.LBB78_158:                             # %bb3.i720
	testq	%rax, %rax
	je	.LBB78_162
# %bb.159:                              # %bb3.i720
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB78_161
# %bb.160:                              # %bb5.i721
	callq	cinch_drop_slow@PLT
	jmp	.LBB78_162
.LBB78_161:                             # %bb4.i723
	callq	cinch_drop_mt@PLT
.LBB78_162:                             # %cinch.rc.drop.tobj.exit743
	xorl	%eax, %eax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end78:
	.size	"Data.Bytes.Word.bytes-write-le64", .Lfunc_end78-"Data.Bytes.Word.bytes-write-le64"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.Uuid.uuid-to-bytes"       # -- Begin function Data.Uuid.uuid-to-bytes
	.p2align	4
	.type	"Data.Uuid.uuid-to-bytes",@function
"Data.Uuid.uuid-to-bytes":              # @Data.Uuid.uuid-to-bytes
	.cfi_startproc
# %bb.0:                                # %entry
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB79_2
# %bb.1:                                # %bb2
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB79_2:                               # %bb4
	testq	%rax, %rax
	je	.LBB79_5
# %bb.3:
	xorl	%eax, %eax
	retq
.LBB79_5:                               # %bb5
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	(%rdi), %rbx
	movq	8(%rdi), %r14
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
	bswapq	%r14
	movl	$17, %esi
	movq	%r15, %rdi
	movq	%r14, %rdx
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	%r15, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	.cfi_restore %rbx
	.cfi_restore %r14
	.cfi_restore %r15
	.cfi_restore %rbp
	retq
.Lfunc_end79:
	.size	"Data.Uuid.uuid-to-bytes", .Lfunc_end79-"Data.Uuid.uuid-to-bytes"
	.cfi_endproc
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function Data.Uuid.uuid-v4-from-random
.LCPI80_0:
	.quad	-61441                          # 0xffffffffffff0fff
	.quad	4611686018427387903             # 0x3fffffffffffffff
.LCPI80_1:
	.quad	16384                           # 0x4000
	.quad	-9223372036854775808            # 0x8000000000000000
	.text
	.globl	"Data.Uuid.uuid-v4-from-random"
	.p2align	4
	.type	"Data.Uuid.uuid-v4-from-random",@function
"Data.Uuid.uuid-v4-from-random":        # @Data.Uuid.uuid-v4-from-random
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
	movq	%rdi, %rbx
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rdi, %rdi
	je	.LBB80_5
# %bb.1:                                # %bb0.i58
	movabsq	$1030792151040, %r15            # imm = 0xF000000000
	movabsq	$274877906944, %r14             # imm = 0x4000000000
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB80_9
# %bb.2:                                # %cinch.runtime.natCmp.exit.thread
	andq	(%rbx), %r12
	cmpq	$16, %r12
	je	.LBB80_10
.LBB80_3:                               # %bb0.i17
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB80_7
# %bb.4:                                # %bb1.i21
	movq	(%rbx), %rax
	jmp	.LBB80_8
.LBB80_5:                               # %cinch.runtime.natCmp.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r12, %rax
	cmpq	$16, %rax
	jne	.LBB80_7
# %bb.6:                                # %cinch.runtime.bytesData.exit127.thread268
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	(%rax), %r12
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	bswapq	%r12
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	8(%rax), %r13
	jmp	.LBB80_21
.LBB80_7:                               # %bb2.i
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB80_8:                               # %cinch.runtime.bytesLen.exit
	leaq	1(,%rax,2), %r15
	movl	$256, %r14d                     # imm = 0x100
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	jmp	.LBB80_23
.LBB80_9:                               # %cinch.runtime.natCmp.exit.thread295
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r12, %rax
	cmpq	$16, %rax
	jne	.LBB80_3
.LBB80_10:                              # %bb0.i88
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB80_13
# %bb.11:                               # %cinch.runtime.bytesData.exit.thread
	cmpq	$0, (%rbx)
	movl	$1, %eax
	je	.LBB80_14
# %bb.12:                               # %cinch.runtime.bytesData.exit.thread
	movq	8(%rbx), %rax
	jmp	.LBB80_14
.LBB80_13:                              # %cinch.runtime.bytesData.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_data@PLT
.LBB80_14:                              # %bb0.i41
	movq	(%rax), %r12
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	bswapq	%r12
	cmpq	%r14, %rax
	je	.LBB80_16
# %bb.15:                               # %cinch.runtime.bytesLen.exit56.bb0.i112_crit_edge
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB80_16:                              # %bb0.i112
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB80_19
# %bb.17:                               # %cinch.runtime.bytesData.exit127.thread
	cmpq	$0, (%rbx)
	movl	$1, %eax
	je	.LBB80_20
# %bb.18:                               # %cinch.runtime.bytesData.exit127.thread
	movq	8(%rbx), %rax
	jmp	.LBB80_20
.LBB80_19:                              # %cinch.runtime.bytesData.exit127
	movq	%rbx, %rdi
	callq	cinch_bytes_data@PLT
.LBB80_20:                              # %bb0.i24
	movq	8(%rax), %r13
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	je	.LBB80_22
.LBB80_21:                              # %bb2.i32
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB80_22:                              # %cinch.runtime.bytesLen.exit39
	bswapq	%r13
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	movq	%r13, 8(%rax)
	movl	$257, %r14d                     # imm = 0x101
.LBB80_23:                              # %bb1
	movl	$8, %edi
	movq	%r14, %rsi
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB80_25
# %bb.24:                               # %bb5
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB80_25:                              # %bb7
	cmpq	$1, %rcx
	je	.LBB80_29
# %bb.26:                               # %bb7
	testq	%rcx, %rcx
	jne	.LBB80_31
# %bb.27:                               # %cinch.rc.unique.exit
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB80_32
# %bb.28:                               # %bb14
	movabsq	$281509336449025, %rcx          # imm = 0x1000800000001
	movq	%rcx, -8(%rax)
	jmp	.LBB80_68
.LBB80_29:                              # %cinch.rc.unique.exit182
	movabsq	$34359738367, %r14              # imm = 0x7FFFFFFFF
	movq	-8(%rax), %rcx
	andq	%r14, %rcx
	cmpq	$1, %rcx
	jne	.LBB80_34
# %bb.30:                               # %bb18
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB80_53
.LBB80_32:                              # %bb0.i154
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB80_36
# %bb.33:                               # %bb2.i161
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB80_38
.LBB80_34:                              # %bb0.i184
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB80_43
# %bb.35:                               # %bb2.i194
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB80_45
.LBB80_36:                              # %bb3.i
	je	.LBB80_38
# %bb.37:                               # %bb4.i
	movq	%rbx, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
.LBB80_38:                              # %bb0.i163
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB80_40
# %bb.39:                               # %bb2.i172
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB80_51
.LBB80_40:                              # %bb3.i169
	testq	%rcx, %rcx
	je	.LBB80_51
# %bb.41:                               # %bb3.i169
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB80_50
# %bb.42:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB80_51
.LBB80_43:                              # %bb3.i190
	je	.LBB80_45
# %bb.44:                               # %bb4.i192
	movq	%rbx, %rdi
	movq	%rax, %r15
	callq	cinch_dup_mt@PLT
	movq	%r15, %rax
.LBB80_45:                              # %bb0.i198
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB80_47
# %bb.46:                               # %bb2.i208
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB80_53
.LBB80_47:                              # %bb3.i204
	testq	%rcx, %rcx
	je	.LBB80_53
# %bb.48:                               # %bb3.i204
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB80_52
# %bb.49:                               # %bb5.i205
	callq	cinch_drop_slow@PLT
	jmp	.LBB80_53
.LBB80_50:                              # %bb4.i171
	callq	cinch_drop_mt@PLT
.LBB80_51:                              # %bb13
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB80_67
.LBB80_52:                              # %bb4.i207
	callq	cinch_drop_mt@PLT
.LBB80_53:                              # %bb20
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB80_55
# %bb.54:                               # %bb22
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB80_55:                              # %bb24
	testq	%rax, %rax
	je	.LBB80_57
.LBB80_31:
	xorl	%eax, %eax
	jmp	.LBB80_68
.LBB80_57:                              # %cinch.rc.unique.exit219
	movups	(%rbx), %xmm0
	movq	-8(%rbx), %rax
	andq	%r14, %rax
	cmpq	$1, %rax
	jne	.LBB80_59
# %bb.58:                               # %bb30
	andps	.LCPI80_0(%rip), %xmm0
	addq	$2, %r14
	movq	%r14, -8(%rbx)
	orps	.LCPI80_1(%rip), %xmm0
	movups	%xmm0, (%rbx)
	jmp	.LBB80_66
.LBB80_59:                              # %bb0.i221
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	movaps	%xmm0, -64(%rbp)                # 16-byte Spill
	jl	.LBB80_61
# %bb.60:                               # %bb2.i231
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB80_65
.LBB80_61:                              # %bb3.i227
	testq	%rax, %rax
	je	.LBB80_65
# %bb.62:                               # %bb3.i227
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB80_64
# %bb.63:                               # %bb5.i228
	callq	cinch_drop_slow@PLT
	jmp	.LBB80_65
.LBB80_64:                              # %bb4.i230
	callq	cinch_drop_mt@PLT
.LBB80_65:                              # %bb29
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movaps	-64(%rbp), %xmm0                # 16-byte Reload
	andps	.LCPI80_0(%rip), %xmm0
	movq	%rax, %rbx
	orps	.LCPI80_1(%rip), %xmm0
	movups	%xmm0, (%rax)
.LBB80_66:                              # %bb33
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
.LBB80_67:                              # %common.ret285
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
.LBB80_68:                              # %common.ret285
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end80:
	.size	"Data.Uuid.uuid-v4-from-random", .Lfunc_end80-"Data.Uuid.uuid-v4-from-random"
	.cfi_endproc
                                        # -- End function
	.globl	"Data.Uuid.uuid-v7-initial#cc"  # -- Begin function Data.Uuid.uuid-v7-initial#cc
	.p2align	4
	.type	"Data.Uuid.uuid-v7-initial#cc",@function
"Data.Uuid.uuid-v7-initial#cc":         # @"Data.Uuid.uuid-v7-initial#cc"
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$1, %eax
	movl	$1, %edx
	popq	%rbp
	retq
.Lfunc_end81:
	.size	"Data.Uuid.uuid-v7-initial#cc", .Lfunc_end81-"Data.Uuid.uuid-v7-initial#cc"
                                        # -- End function
	.globl	"Data.Uuid.uuid-v7-initial"     # -- Begin function Data.Uuid.uuid-v7-initial
	.p2align	4
	.type	"Data.Uuid.uuid-v7-initial",@function
"Data.Uuid.uuid-v7-initial":            # @Data.Uuid.uuid-v7-initial
# %bb.0:                                # %entry
	pushq	%rax
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	$1, (%rax)
	movq	$1, 8(%rax)
	popq	%rcx
	retq
.Lfunc_end82:
	.size	"Data.Uuid.uuid-v7-initial", .Lfunc_end82-"Data.Uuid.uuid-v7-initial"
                                        # -- End function
	.globl	Data.Uuid.UuidBits              # -- Begin function Data.Uuid.UuidBits
	.p2align	4
	.type	Data.Uuid.UuidBits,@function
Data.Uuid.UuidBits:                     # @Data.Uuid.UuidBits
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end83:
	.size	Data.Uuid.UuidBits, .Lfunc_end83-Data.Uuid.UuidBits
                                        # -- End function
	.globl	Data.Uuid.UuidInvalidLength     # -- Begin function Data.Uuid.UuidInvalidLength
	.p2align	4
	.type	Data.Uuid.UuidInvalidLength,@function
Data.Uuid.UuidInvalidLength:            # @Data.Uuid.UuidInvalidLength
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end84:
	.size	Data.Uuid.UuidInvalidLength, .Lfunc_end84-Data.Uuid.UuidInvalidLength
                                        # -- End function
	.globl	Data.Uuid.UuidClockOutOfRange   # -- Begin function Data.Uuid.UuidClockOutOfRange
	.p2align	4
	.type	Data.Uuid.UuidClockOutOfRange,@function
Data.Uuid.UuidClockOutOfRange:          # @Data.Uuid.UuidClockOutOfRange
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	"Data.Uuid.UuidClockOutOfRange.blk#h"+8(%rip), %rax
	popq	%rbp
	retq
.Lfunc_end85:
	.size	Data.Uuid.UuidClockOutOfRange, .Lfunc_end85-Data.Uuid.UuidClockOutOfRange
                                        # -- End function
	.globl	Data.Uuid.UuidClockExhausted    # -- Begin function Data.Uuid.UuidClockExhausted
	.p2align	4
	.type	Data.Uuid.UuidClockExhausted,@function
Data.Uuid.UuidClockExhausted:           # @Data.Uuid.UuidClockExhausted
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	"Data.Uuid.UuidClockExhausted.blk#h"+8(%rip), %rax
	popq	%rbp
	retq
.Lfunc_end86:
	.size	Data.Uuid.UuidClockExhausted, .Lfunc_end86-Data.Uuid.UuidClockExhausted
                                        # -- End function
	.globl	Data.Uuid.V7Clock               # -- Begin function Data.Uuid.V7Clock
	.p2align	4
	.type	Data.Uuid.V7Clock,@function
Data.Uuid.V7Clock:                      # @Data.Uuid.V7Clock
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%rbx, 8(%rax)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end87:
	.size	Data.Uuid.V7Clock, .Lfunc_end87-Data.Uuid.V7Clock
                                        # -- End function
	.globl	"Effect.Clock.clock-monotonic-ns" # -- Begin function Effect.Clock.clock-monotonic-ns
	.p2align	4
	.type	"Effect.Clock.clock-monotonic-ns",@function
"Effect.Clock.clock-monotonic-ns":      # @Effect.Clock.clock-monotonic-ns
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_clock_monotonic_ns@PLT
	testq	%rax, %rax
	js	.LBB88_2
# %bb.1:                                # %bb0.i
	leaq	1(,%rax,2), %rax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB88_2:                               # %bb1.i
	.cfi_def_cfa %rbp, 16
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_from_i64@PLT          # TAILCALL
.Lfunc_end88:
	.size	"Effect.Clock.clock-monotonic-ns", .Lfunc_end88-"Effect.Clock.clock-monotonic-ns"
	.cfi_endproc
                                        # -- End function
	.globl	Effect.Concurrent.MVar.MkMVar   # -- Begin function Effect.Concurrent.MVar.MkMVar
	.p2align	4
	.type	Effect.Concurrent.MVar.MkMVar,@function
Effect.Concurrent.MVar.MkMVar:          # @Effect.Concurrent.MVar.MkMVar
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end89:
	.size	Effect.Concurrent.MVar.MkMVar, .Lfunc_end89-Effect.Concurrent.MVar.MkMVar
                                        # -- End function
	.globl	"Effect.Process.arg-count"      # -- Begin function Effect.Process.arg-count
	.p2align	4
	.type	"Effect.Process.arg-count",@function
"Effect.Process.arg-count":             # @Effect.Process.arg-count
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	cinch_argv_count@PLT
	movq	%rax, %rdi
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	cinch_nat_from_i64@PLT          # TAILCALL
.Lfunc_end90:
	.size	"Effect.Process.arg-count", .Lfunc_end90-"Effect.Process.arg-count"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Process.process-args"   # -- Begin function Effect.Process.process-args
	.p2align	4
	.type	"Effect.Process.process-args",@function
"Effect.Process.process-args":          # @Effect.Process.process-args
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	callq	cinch_argv_count@PLT
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %rbx
	testb	$1, %bl
	jne	.LBB91_4
# %bb.1:                                # %cinch.runtime.natCmp.exit52
	movl	$1, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rbx, %rbx
	je	.LBB91_5
# %bb.2:                                # %bb0.i92
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB91_21
# %bb.3:                                # %bb2.i100
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB91_5
.LBB91_4:                               # %cinch.runtime.natCmp.exit52.thread
	shrq	%rbx
	testq	%rbx, %rbx
	seta	%al
	sbbb	$0, %al
	movsbq	%al, %rax
.LBB91_5:                               # %cinch.rc.drop.tobj.exit
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %r12
	movl	$257, %r14d                     # imm = 0x101
	testq	%rax, %rax
	je	.LBB91_86
# %bb.6:                                # %bb2
	callq	cinch_argv_count@PLT
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r15
	testb	$1, %r15b
	sete	%al
	cmpq	$2, %r15
	setb	%cl
	orb	%al, %cl
	je	.LBB91_10
# %bb.7:                                # %cinch.runtime.natSub.exit33
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %rbx
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB91_11
# %bb.8:                                # %bb0.i112
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB91_32
# %bb.9:                                # %bb2.i122
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB91_11
.LBB91_10:                              # %cinch.runtime.natSub.exit33.thread
	addq	$-2, %r15
	movq	%r15, %rbx
.LBB91_11:                              # %cinch.rc.drop.tobj.exit124
	testb	$1, %bl
	jne	.LBB91_16
# %bb.12:                               # %cinch.runtime.natCmp.exit
	movl	$1, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jne	.LBB91_17
# %bb.13:                               # %bb3
	testq	%rbx, %rbx
	je	.LBB91_86
# %bb.14:                               # %bb0.i131
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB91_46
# %bb.15:                               # %bb2.i141
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB91_86
.LBB91_16:                              # %bb0.i37
	cmpq	$2, %rbx
	jb	.LBB91_86
.LBB91_17:                              # %bb4
	movq	%rbx, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_argv_get@PLT
	movq	%rax, %r14
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.String.utf8-valid-from"@PLT
	testq	%rax, %rax
	je	.LBB91_28
# %bb.18:                               # %bb6
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	movabsq	$274877906944, %r13             # imm = 0x4000000000
	testq	%r14, %r14
	je	.LBB91_24
# %bb.19:                               # %bb0.i63
	movq	-8(%r14), %rax
	andq	%rcx, %rax
	cmpq	%r13, %rax
	jne	.LBB91_24
# %bb.20:                               # %bb1.i77
	movq	(%r14), %rax
	jmp	.LBB91_25
.LBB91_21:                              # %bb3.i98
	testq	%rcx, %rcx
	je	.LBB91_5
# %bb.22:                               # %bb3.i98
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB91_41
# %bb.23:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_42
.LBB91_24:                              # %bb2.i71
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB91_25:                              # %cinch.runtime.bytesLen.exit79
	leaq	1(,%rax,2), %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%r14, %r14
	je	.LBB91_31
# %bb.26:                               # %bb0.i54
	movq	-8(%r14), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	cmpq	%r13, %rax
	jne	.LBB91_35
# %bb.27:                               # %cinch.runtime.bytesLen.exit.thread
	movq	(%r14), %r8
	jmp	.LBB91_36
.LBB91_28:                              # %bb7
	leaq	"Data.Core.None.blk#h"+8(%rip), %rax
	testq	%r14, %r14
	je	.LBB91_52
# %bb.29:                               # %bb0.i186
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB91_43
# %bb.30:                               # %bb2.i196
	decq	%rdx
	movq	%rdx, -8(%r14)
	jmp	.LBB91_52
.LBB91_31:                              # %cinch.rc.drop.obj.exit.critedge
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	andq	%rax, %r12
	movq	%r15, %rdi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%r12, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB91_51
.LBB91_32:                              # %bb3.i118
	testq	%rax, %rax
	je	.LBB91_11
# %bb.33:                               # %bb3.i118
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB91_49
# %bb.34:                               # %bb5.i119
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_11
.LBB91_35:                              # %cinch.runtime.bytesLen.exit
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB91_36:                              # %bb0.i153
	andq	%r12, %r8
	movq	%r15, %rdi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB91_38
# %bb.37:                               # %bb2.i163
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB91_51
.LBB91_38:                              # %bb3.i159
	testq	%rax, %rax
	je	.LBB91_51
# %bb.39:                               # %bb3.i159
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB91_50
# %bb.40:                               # %bb5.i160
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_51
.LBB91_41:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB91_42:                              # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
	jmp	.LBB91_5
.LBB91_43:                              # %bb3.i192
	testq	%rcx, %rcx
	je	.LBB91_52
# %bb.44:                               # %bb3.i192
	movq	%rax, %r15
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB91_75
# %bb.45:                               # %bb5.i193
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_76
.LBB91_46:                              # %bb3.i137
	testq	%rax, %rax
	je	.LBB91_86
# %bb.47:                               # %bb3.i137
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB91_48
.LBB91_85:                              # %bb4.i229
	callq	cinch_drop_mt@PLT
	jmp	.LBB91_86
.LBB91_49:                              # %bb4.i121
	callq	cinch_drop_mt@PLT
	jmp	.LBB91_11
.LBB91_50:                              # %bb4.i162
	callq	cinch_drop_mt@PLT
.LBB91_51:                              # %cinch.rc.drop.obj.exit
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%r15, (%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
.LBB91_52:                              # %bb5
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB91_54
# %bb.53:                               # %bb9
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB91_54:                              # %bb11
	cmpq	$1, %rcx
	je	.LBB91_59
# %bb.55:                               # %bb11
	testq	%rcx, %rcx
	jne	.LBB91_61
# %bb.56:                               # %bb12
	testq	%rbx, %rbx
	sete	%cl
	movl	%ebx, %edx
	orb	%cl, %dl
	testb	$1, %dl
	jne	.LBB91_80
# %bb.57:                               # %bb0.i203
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB91_65
# %bb.58:                               # %bb2.i213
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB91_80
.LBB91_59:                              # %cinch.rc.unique.exit
	movq	(%rax), %r14
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB91_62
# %bb.60:                               # %bb14
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB91_89
.LBB91_61:
	xorl	%eax, %eax
	jmp	.LBB91_87
.LBB91_62:                              # %bb15
	testq	%r14, %r14
	je	.LBB91_70
# %bb.63:                               # %bb0.i238
	movq	-8(%r14), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB91_68
# %bb.64:                               # %bb2.i247
	incq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB91_70
.LBB91_65:                              # %bb3.i209
	testq	%rcx, %rcx
	je	.LBB91_80
# %bb.66:                               # %bb3.i209
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB91_78
# %bb.67:                               # %bb5.i210
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_79
.LBB91_68:                              # %bb3.i244
	je	.LBB91_70
# %bb.69:                               # %bb4.i245
	movq	%r14, %rdi
	movq	%rax, %r15
	callq	cinch_dup_mt@PLT
	movq	%r15, %rax
.LBB91_70:                              # %bb0.i250
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB91_72
# %bb.71:                               # %bb2.i260
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB91_89
.LBB91_72:                              # %bb3.i256
	testq	%rcx, %rcx
	je	.LBB91_89
# %bb.73:                               # %bb3.i256
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB91_88
# %bb.74:                               # %bb5.i257
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_89
.LBB91_75:                              # %bb4.i195
	callq	cinch_drop_mt@PLT
.LBB91_76:                              # %bb5
	movq	%r15, %rax
	jmp	.LBB91_52
.LBB91_78:                              # %bb4.i212
	callq	cinch_drop_mt@PLT
.LBB91_79:                              # %bb0.i220
	movq	%r14, %rax
.LBB91_80:                              # %bb0.i220
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB91_82
# %bb.81:                               # %bb2.i230
	decq	%rdx
	movq	%rdx, -8(%rax)
	leaq	"Effect.Process.ProcessInvalidArgument.blk#h"+8(%rip), %r12
	movl	$256, %r14d                     # imm = 0x100
	jmp	.LBB91_86
.LBB91_82:                              # %bb3.i226
	leaq	"Effect.Process.ProcessInvalidArgument.blk#h"+8(%rip), %r12
	movl	$256, %r14d                     # imm = 0x100
	testq	%rcx, %rcx
	je	.LBB91_86
# %bb.83:                               # %bb3.i226
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB91_85
.LBB91_48:                              # %bb5.i138
	callq	cinch_drop_slow@PLT
.LBB91_86:                              # %common.ret.sink.split
	movl	$8, %edi
	movq	%r14, %rsi
	callq	cinch_alloc_ctor@PLT
	movq	%r12, (%rax)
.LBB91_87:                              # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB91_88:                              # %bb4.i259
	.cfi_def_cfa %rbp, 16
	callq	cinch_drop_mt@PLT
.LBB91_89:                              # %bb16
	testb	$1, %bl
	sete	%al
	cmpq	$2, %rbx
	setb	%cl
	orb	%al, %cl
	je	.LBB91_93
# %bb.90:                               # %cinch.runtime.natSub.exit
	movl	$3, %esi
	movq	%rbx, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r15
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB91_94
# %bb.91:                               # %bb0.i267
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB91_95
# %bb.92:                               # %bb2.i277
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB91_94
.LBB91_93:                              # %cinch.runtime.natSub.exit.thread
	addq	$-2, %rbx
	movq	%rbx, %r15
.LBB91_94:                              # %cinch.rc.drop.tobj.exit279
	movl	$16, %edi
	movl	$769, %esi                      # imm = 0x301
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	leaq	"Data.Core.Nil.blk#h"+8(%rip), %rcx
	movq	%rcx, 8(%rax)
	movq	%r15, %rdi
	movq	%rax, %rsi
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Effect.Process.args-down"@PLT  # TAILCALL
.LBB91_95:                              # %bb3.i273
	.cfi_def_cfa %rbp, 16
	testq	%rax, %rax
	je	.LBB91_94
# %bb.96:                               # %bb3.i273
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB91_98
# %bb.97:                               # %bb5.i274
	callq	cinch_drop_slow@PLT
	jmp	.LBB91_94
.LBB91_98:                              # %bb4.i276
	callq	cinch_drop_mt@PLT
	jmp	.LBB91_94
.Lfunc_end91:
	.size	"Effect.Process.process-args", .Lfunc_end91-"Effect.Process.process-args"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Process.args-down"      # -- Begin function Effect.Process.args-down
	.p2align	4
	.type	"Effect.Process.args-down",@function
"Effect.Process.args-down":             # @Effect.Process.args-down
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %r14
	movq	%rdi, %r12
	jmp	.LBB92_3
	.p2align	4
.LBB92_1:                               # %cinch.runtime.natSub.exit.thread
                                        #   in Loop: Header=BB92_3 Depth=1
	addq	$-2, %r13
	movq	%r13, %r12
.LBB92_2:                               # %cinch.rc.drop.tobj.exit485
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$16, %edi
	movl	$769, %esi                      # imm = 0x301
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	movq	%r15, 8(%rax)
.LBB92_3:                               # %bb0
                                        # =>This Inner Loop Header: Depth=1
	xorl	%r13d, %r13d
	testb	$1, %r12b
	jne	.LBB92_5
# %bb.4:                                # %cinch.runtime.natCmp.exit67
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$1, %esi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jne	.LBB92_6
	jmp	.LBB92_117
	.p2align	4
.LBB92_5:                               # %bb0.i61
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$2, %r12
	jb	.LBB92_164
.LBB92_6:                               # %bb2
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%r12, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_argv_get@PLT
	movq	%rax, %rbx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.String.utf8-valid-from"@PLT
	testq	%rax, %rax
	je	.LBB92_14
# %bb.7:                                # %bb4
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB92_10
# %bb.8:                                # %bb0.i114
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB92_10
# %bb.9:                                # %bb1.i128
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rbx), %rax
	jmp	.LBB92_11
	.p2align	4
.LBB92_10:                              # %bb2.i122
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB92_11:                              # %cinch.runtime.bytesLen.exit130
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	1(,%rax,2), %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testq	%rbx, %rbx
	je	.LBB92_17
# %bb.12:                               # %bb0.i96
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB92_18
# %bb.13:                               # %cinch.runtime.bytesLen.exit112.thread
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rbx), %r8
	jmp	.LBB92_19
	.p2align	4
.LBB92_14:                              # %bb5
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rbx, %rbx
	je	.LBB92_31
# %bb.15:                               # %bb0.i204
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_24
# %bb.16:                               # %bb2.i214
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB92_30
	.p2align	4
.LBB92_17:                              # %cinch.rc.drop.obj.exit.critedge
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r15, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%rax, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB92_28
.LBB92_18:                              # %cinch.runtime.bytesLen.exit112
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB92_19:                              # %bb0.i171
                                        #   in Loop: Header=BB92_3 Depth=1
	movabsq	$9223372036854775807, %rax      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rax, %r8
	movq	%r15, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_21
# %bb.20:                               # %bb2.i181
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB92_28
.LBB92_21:                              # %bb3.i177
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_28
# %bb.22:                               # %bb3.i177
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_27
# %bb.23:                               # %bb5.i178
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_28
.LBB92_24:                              # %bb3.i210
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rax, %rax
	je	.LBB92_31
# %bb.25:                               # %bb3.i210
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_29
# %bb.26:                               # %bb5.i211
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_30
.LBB92_27:                              # %bb4.i180
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB92_28:                              # %cinch.rc.drop.obj.exit
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%rbx, (%rax)
	jmp	.LBB92_31
.LBB92_29:                              # %bb4.i213
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
.LBB92_30:                              # %bb3
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	.p2align	4
.LBB92_31:                              # %bb3
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB92_33
# %bb.32:                               # %bb7
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB92_33:                              # %bb9
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB92_108
# %bb.34:                               # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rdi), %rbx
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB92_36
# %bb.35:                               # %bb12
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_free@PLT
	jmp	.LBB92_47
	.p2align	4
.LBB92_36:                              # %bb13
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB92_41
# %bb.37:                               # %bb0.i267
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB92_39
# %bb.38:                               # %bb2.i276
                                        #   in Loop: Header=BB92_3 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB92_41
.LBB92_39:                              # %bb3.i273
                                        #   in Loop: Header=BB92_3 Depth=1
	je	.LBB92_41
# %bb.40:                               # %bb4.i274
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rdi, %r15
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	movq	%r15, %rdi
	.p2align	4
.LBB92_41:                              # %bb0.i279
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_43
# %bb.42:                               # %bb2.i289
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB92_47
	.p2align	4
.LBB92_43:                              # %bb3.i285
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_47
# %bb.44:                               # %bb3.i285
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_46
# %bb.45:                               # %bb5.i286
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_47
.LBB92_46:                              # %bb4.i288
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB92_47:                              # %bb14
                                        #   in Loop: Header=BB92_3 Depth=1
	testb	$1, %r12b
	sete	%al
	cmpq	$2, %r12
	setb	%cl
	orb	%al, %cl
	je	.LBB92_51
# %bb.48:                               # %cinch.runtime.natSub.exit48
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$3, %esi
	movq	%r12, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r13
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB92_52
# %bb.49:                               # %bb0.i296
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_73
# %bb.50:                               # %bb2.i306
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB92_52
	.p2align	4
.LBB92_51:                              # %cinch.runtime.natSub.exit48.thread
                                        #   in Loop: Header=BB92_3 Depth=1
	addq	$-2, %r12
	movq	%r12, %r13
.LBB92_52:                              # %cinch.rc.drop.tobj.exit308
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$16, %edi
	movl	$769, %esi                      # imm = 0x301
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	testb	$1, %r13b
	jne	.LBB92_54
# %bb.53:                               # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$1, %esi
	movq	%r13, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jne	.LBB92_55
	jmp	.LBB92_120
	.p2align	4
.LBB92_54:                              # %bb0.i52
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$2, %r13
	jb	.LBB92_123
.LBB92_55:                              # %bb16
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%r13, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_argv_get@PLT
	movq	%rax, %rbx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.String.utf8-valid-from"@PLT
	testq	%rax, %rax
	je	.LBB92_63
# %bb.56:                               # %bb18
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB92_59
# %bb.57:                               # %bb0.i78
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB92_59
# %bb.58:                               # %bb1.i92
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rbx), %rax
	jmp	.LBB92_60
	.p2align	4
.LBB92_59:                              # %bb2.i86
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB92_60:                              # %cinch.runtime.bytesLen.exit94
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	1(,%rax,2), %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r14
	testq	%rbx, %rbx
	je	.LBB92_66
# %bb.61:                               # %bb0.i69
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB92_67
# %bb.62:                               # %cinch.runtime.bytesLen.exit.thread
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rbx), %r8
	jmp	.LBB92_68
	.p2align	4
.LBB92_63:                              # %bb19
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rbx, %rbx
	je	.LBB92_84
# %bb.64:                               # %bb0.i372
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_76
# %bb.65:                               # %bb2.i382
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB92_83
	.p2align	4
.LBB92_66:                              # %cinch.rc.drop.obj.exit350.critedge
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	%rax, %r8
	callq	cinch_bytes_copy@PLT
	jmp	.LBB92_80
.LBB92_67:                              # %cinch.runtime.bytesLen.exit
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	movq	%rax, %r8
.LBB92_68:                              # %bb0.i338
                                        #   in Loop: Header=BB92_3 Depth=1
	movabsq	$9223372036854775807, %rax      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rax, %r8
	movq	%r14, %rdi
	movq	%rbx, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_bytes_copy@PLT
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_70
# %bb.69:                               # %bb2.i348
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB92_80
.LBB92_70:                              # %bb3.i344
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_80
# %bb.71:                               # %bb3.i344
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_79
# %bb.72:                               # %bb5.i345
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_80
.LBB92_73:                              # %bb3.i302
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_52
# %bb.74:                               # %bb3.i302
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_81
# %bb.75:                               # %bb5.i303
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_52
.LBB92_76:                              # %bb3.i378
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rax, %rax
	je	.LBB92_84
# %bb.77:                               # %bb3.i378
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_82
# %bb.78:                               # %bb5.i379
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_83
.LBB92_79:                              # %bb4.i347
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB92_80:                              # %cinch.rc.drop.obj.exit350
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r14, (%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%rbx, (%rax)
	jmp	.LBB92_84
.LBB92_81:                              # %bb4.i305
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB92_52
.LBB92_82:                              # %bb4.i381
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
.LBB92_83:                              # %bb17
                                        #   in Loop: Header=BB92_3 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	.p2align	4
.LBB92_84:                              # %bb17
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB92_86
# %bb.85:                               # %bb21
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB92_86:                              # %bb23
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$1, %rax
	jne	.LBB92_112
# %bb.87:                               # %cinch.rc.unique.exit440
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	(%rdi), %rbx
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB92_89
# %bb.88:                               # %bb26
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_free@PLT
	jmp	.LBB92_100
	.p2align	4
.LBB92_89:                              # %bb27
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rbx, %rbx
	je	.LBB92_94
# %bb.90:                               # %bb0.i442
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB92_92
# %bb.91:                               # %bb2.i452
                                        #   in Loop: Header=BB92_3 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB92_94
.LBB92_92:                              # %bb3.i448
                                        #   in Loop: Header=BB92_3 Depth=1
	je	.LBB92_94
# %bb.93:                               # %bb4.i450
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%rdi, %r14
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	movq	%r14, %rdi
	.p2align	4
.LBB92_94:                              # %bb0.i456
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_96
# %bb.95:                               # %bb2.i466
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB92_100
	.p2align	4
.LBB92_96:                              # %bb3.i462
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_100
# %bb.97:                               # %bb3.i462
                                        #   in Loop: Header=BB92_3 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_99
# %bb.98:                               # %bb5.i463
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_100
.LBB92_99:                              # %bb4.i465
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB92_100:                             # %bb28
                                        #   in Loop: Header=BB92_3 Depth=1
	testb	$1, %r13b
	sete	%al
	cmpq	$2, %r13
	setb	%cl
	orb	%al, %cl
	je	.LBB92_1
# %bb.101:                              # %cinch.runtime.natSub.exit
                                        #   in Loop: Header=BB92_3 Depth=1
	movl	$3, %esi
	movq	%r13, %rdi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r12
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB92_2
# %bb.102:                              # %bb0.i473
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_104
# %bb.103:                              # %bb2.i483
                                        #   in Loop: Header=BB92_3 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB92_2
.LBB92_104:                             # %bb3.i479
                                        #   in Loop: Header=BB92_3 Depth=1
	testq	%rax, %rax
	je	.LBB92_2
# %bb.105:                              # %bb3.i479
                                        #   in Loop: Header=BB92_3 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_107
# %bb.106:                              # %bb5.i480
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_2
.LBB92_107:                             # %bb4.i482
                                        #   in Loop: Header=BB92_3 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB92_2
.LBB92_108:                             # %bb9
	testq	%rax, %rax
	jne	.LBB92_166
# %bb.109:                              # %bb10
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB92_139
# %bb.110:                              # %bb0.i221
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_124
# %bb.111:                              # %bb2.i231
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB92_139
.LBB92_112:                             # %bb23
	testq	%rax, %rax
	jne	.LBB92_116
# %bb.113:                              # %bb24
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB92_147
# %bb.114:                              # %bb0.i389
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_127
# %bb.115:                              # %bb2.i399
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB92_147
.LBB92_116:
	xorl	%r13d, %r13d
	jmp	.LBB92_166
.LBB92_117:                             # %bb1
	movl	$257, %esi                      # imm = 0x101
	testq	%r12, %r12
	je	.LBB92_165
# %bb.118:                              # %bb0.i154
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_131
# %bb.119:                              # %bb2.i162
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB92_165
.LBB92_120:                             # %bb15
	movl	$257, %esi                      # imm = 0x101
	testq	%r13, %r13
	je	.LBB92_130
# %bb.121:                              # %bb0.i318
	movq	-8(%r13), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB92_134
# %bb.122:                              # %bb2.i328
	decq	%rax
	movq	%rax, -8(%r13)
	movq	%r15, %r14
	jmp	.LBB92_165
.LBB92_123:
	movl	$257, %esi                      # imm = 0x101
	movq	%r15, %r14
	jmp	.LBB92_165
.LBB92_124:                             # %bb3.i227
	testq	%rax, %rax
	je	.LBB92_139
# %bb.125:                              # %bb3.i227
	movq	%rdi, %rbx
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_137
# %bb.126:                              # %bb5.i228
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_138
.LBB92_127:                             # %bb3.i395
	testq	%rax, %rax
	je	.LBB92_147
# %bb.128:                              # %bb3.i395
	movq	%rdi, %rbx
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_145
# %bb.129:                              # %bb5.i396
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_146
.LBB92_131:                             # %bb3.i160
	testq	%rax, %rax
	je	.LBB92_165
# %bb.132:                              # %bb3.i160
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_160
# %bb.133:                              # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_164
.LBB92_134:                             # %bb3.i324
	testq	%rcx, %rcx
	je	.LBB92_130
# %bb.135:                              # %bb3.i324
	movq	%r13, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB92_162
# %bb.136:                              # %bb5.i325
	callq	cinch_drop_slow@PLT
	jmp	.LBB92_163
.LBB92_130:
	movq	%r15, %r14
	jmp	.LBB92_165
.LBB92_137:                             # %bb4.i230
	callq	cinch_drop_mt@PLT
.LBB92_138:                             # %cinch.rc.drop.tobj.exit233
	movq	%rbx, %rdi
.LBB92_139:                             # %cinch.rc.drop.tobj.exit233
	testq	%r14, %r14
	je	.LBB92_154
# %bb.140:                              # %bb0.i235
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_142
# %bb.141:                              # %bb2.i245
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB92_154
.LBB92_142:                             # %bb3.i241
	testq	%rax, %rax
	je	.LBB92_154
# %bb.143:                              # %bb3.i241
	movq	%rdi, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	.LBB92_144
.LBB92_152:                             # %bb4.i244
	callq	cinch_drop_mt@PLT
	jmp	.LBB92_153
.LBB92_145:                             # %bb4.i398
	callq	cinch_drop_mt@PLT
.LBB92_146:                             # %bb0.i403
	movq	%rbx, %rdi
.LBB92_147:                             # %bb0.i403
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_149
# %bb.148:                              # %bb2.i413
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB92_154
.LBB92_149:                             # %bb3.i409
	testq	%rax, %rax
	je	.LBB92_154
# %bb.150:                              # %bb3.i409
	movq	%rdi, %rbx
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_152
.LBB92_144:                             # %bb5.i242
	callq	cinch_drop_slow@PLT
.LBB92_153:                             # %bb0.i249
	movq	%rbx, %rdi
.LBB92_154:                             # %bb0.i249
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB92_156
# %bb.155:                              # %bb2.i259
	decq	%rcx
	movq	%rcx, -8(%rdi)
	leaq	"Effect.Process.ProcessInvalidArgument.blk#h"+8(%rip), %r14
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB92_165
.LBB92_156:                             # %bb3.i255
	leaq	"Effect.Process.ProcessInvalidArgument.blk#h"+8(%rip), %r14
	movl	$256, %esi                      # imm = 0x100
	testq	%rax, %rax
	je	.LBB92_165
# %bb.157:                              # %bb3.i255
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB92_159
# %bb.158:                              # %bb5.i256
	callq	cinch_drop_slow@PLT
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB92_165
.LBB92_159:                             # %bb4.i258
	callq	cinch_drop_mt@PLT
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB92_165
.LBB92_160:                             # %bb4.i
	callq	cinch_drop_mt@PLT
	jmp	.LBB92_164
.LBB92_162:                             # %bb4.i327
	callq	cinch_drop_mt@PLT
.LBB92_163:
	movq	%r15, %r14
.LBB92_164:
	movl	$257, %esi                      # imm = 0x101
.LBB92_165:                             # %common.ret.sink.split
	movl	$8, %edi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%r14, (%rax)
.LBB92_166:                             # %common.ret
	movq	%r13, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end92:
	.size	"Effect.Process.args-down", .Lfunc_end92-"Effect.Process.args-down"
	.cfi_endproc
                                        # -- End function
	.globl	Effect.Process.ProcessInvalidArgument # -- Begin function Effect.Process.ProcessInvalidArgument
	.p2align	4
	.type	Effect.Process.ProcessInvalidArgument,@function
Effect.Process.ProcessInvalidArgument:  # @Effect.Process.ProcessInvalidArgument
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	"Effect.Process.ProcessInvalidArgument.blk#h"+8(%rip), %rax
	popq	%rbp
	retq
.Lfunc_end93:
	.size	Effect.Process.ProcessInvalidArgument, .Lfunc_end93-Effect.Process.ProcessInvalidArgument
                                        # -- End function
	.globl	"Effect.Uuid.uuid-v7-generator#cc" # -- Begin function Effect.Uuid.uuid-v7-generator#cc
	.p2align	4
	.type	"Effect.Uuid.uuid-v7-generator#cc",@function
"Effect.Uuid.uuid-v7-generator#cc":     # @"Effect.Uuid.uuid-v7-generator#cc"
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -24
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	$1, (%rax)
	movq	$1, 8(%rax)
	movl	$1, %esi
	movl	$1, %edx
	movq	%rax, %rdi
	callq	cinch_mvar_new@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end94:
	.size	"Effect.Uuid.uuid-v7-generator#cc", .Lfunc_end94-"Effect.Uuid.uuid-v7-generator#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Uuid.uuid-v7-generator" # -- Begin function Effect.Uuid.uuid-v7-generator
	.p2align	4
	.type	"Effect.Uuid.uuid-v7-generator",@function
"Effect.Uuid.uuid-v7-generator":        # @Effect.Uuid.uuid-v7-generator
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	$1, (%rax)
	movq	$1, 8(%rax)
	movl	$1, %esi
	movl	$1, %edx
	movq	%rax, %rdi
	callq	cinch_mvar_new@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end95:
	.size	"Effect.Uuid.uuid-v7-generator", .Lfunc_end95-"Effect.Uuid.uuid-v7-generator"
	.cfi_endproc
                                        # -- End function
	.globl	"Effect.Uuid.uuid-new-v7"       # -- Begin function Effect.Uuid.uuid-new-v7
	.p2align	4
	.type	"Effect.Uuid.uuid-new-v7",@function
"Effect.Uuid.uuid-new-v7":              # @Effect.Uuid.uuid-new-v7
	.cfi_startproc
# %bb.0:                                # %entry
	movq	-8(%rsi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB96_2
# %bb.1:                                # %bb2
	movq	-16(%rsi), %rax
	movq	(%rax), %rax
.LBB96_2:                               # %bb4
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
	testq	%rax, %rax
	jne	.LBB96_41
# %bb.4:                                # %bb5
	movq	(%rsi), %rbx
	testq	%rsi, %rsi
	je	.LBB96_7
# %bb.5:                                # %cinch.rc.unique.exit
	movq	-8(%rsi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB96_7
# %bb.6:                                # %bb6
	movq	%rsi, %rdi
	callq	cinch_free@PLT
	jmp	.LBB96_19
.LBB96_7:                               # %bb7
	testq	%rbx, %rbx
	je	.LBB96_12
# %bb.8:                                # %bb0.i249
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_10
# %bb.9:                                # %bb2.i257
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB96_12
.LBB96_10:                              # %bb3.i255
	je	.LBB96_12
# %bb.11:                               # %bb4.i
	movq	%rbx, %rdi
	movq	%rsi, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rsi
.LBB96_12:                              # %cinch.rc.dup.obj.exit
	testq	%rsi, %rsi
	je	.LBB96_19
# %bb.13:                               # %bb0.i260
	movq	-8(%rsi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_15
# %bb.14:                               # %bb2.i269
	decq	%rcx
	movq	%rcx, -8(%rsi)
	jmp	.LBB96_19
.LBB96_15:                              # %bb3.i266
	testq	%rax, %rax
	je	.LBB96_19
# %bb.16:                               # %bb3.i266
	movq	%rsi, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_18
# %bb.17:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_19
.LBB96_18:                              # %bb4.i268
	callq	cinch_drop_mt@PLT
.LBB96_19:                              # %bb8
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB96_21
# %bb.20:                               # %bb11
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB96_21:                              # %bb13
	testq	%rax, %rax
	jne	.LBB96_41
# %bb.23:                               # %bb14
	movq	(%rbx), %r14
	testq	%r14, %r14
	je	.LBB96_26
# %bb.24:                               # %bb0.i272
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_27
# %bb.25:                               # %bb2.i282
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB96_29
.LBB96_26:                              # %cinch.rc.dup.obj.exit284
	leaq	-88(%rbp), %r8
	xorl	%edi, %edi
	xorl	%esi, %esi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	-88(%rbp), %r13
	jmp	.LBB96_35
.LBB96_27:                              # %bb3.i278
	je	.LBB96_29
# %bb.28:                               # %bb4.i280
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_29:                              # %bb0.i289
	leaq	-88(%rbp), %r8
	movq	%r14, %rdi
	xorl	%esi, %esi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	-88(%rbp), %r13
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_31
# %bb.30:                               # %bb2.i299
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB96_35
.LBB96_31:                              # %bb3.i295
	testq	%rax, %rax
	je	.LBB96_35
# %bb.32:                               # %bb3.i295
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_34
# %bb.33:                               # %bb5.i296
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_35
.LBB96_34:                              # %bb4.i298
	callq	cinch_drop_mt@PLT
.LBB96_35:                              # %cinch.rc.drop.obj.exit301
	callq	cinch_clock_wall_ns@PLT
	testq	%rax, %rax
	js	.LBB96_37
# %bb.36:                               # %bb0.i187
	leaq	1(,%rax,2), %r15
	jmp	.LBB96_38
.LBB96_37:                              # %bb1.i184
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r15
.LBB96_38:                              # %cinch.runtime.natFromI64.exit
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB96_40
# %bb.39:                               # %bb17
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB96_40:                              # %bb19
	testq	%rax, %rax
	je	.LBB96_43
.LBB96_41:
	xorl	%eax, %eax
.LBB96_42:                              # %common.ret
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB96_43:                              # %bb20
	.cfi_def_cfa %rbp, 16
	movq	(%r13), %r12
	movq	8(%r13), %r14
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	movb	%cl, -42(%rbp)                  # 1-byte Spill
	testb	$1, %cl
	jne	.LBB96_48
# %bb.44:                               # %bb0.i308
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_46
# %bb.45:                               # %bb2.i317
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB96_48
.LBB96_46:                              # %bb3.i314
	je	.LBB96_48
# %bb.47:                               # %bb4.i315
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_48:                              # %cinch.rc.dup.tobj.exit
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	movb	%cl, -41(%rbp)                  # 1-byte Spill
	testb	$1, %cl
	jne	.LBB96_53
# %bb.49:                               # %bb0.i323
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_51
# %bb.50:                               # %bb2.i333
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB96_53
.LBB96_51:                              # %bb3.i329
	je	.LBB96_53
# %bb.52:                               # %bb4.i331
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_53:                              # %cinch.rc.dup.tobj.exit335
	movq	%r12, -72(%rbp)                 # 8-byte Spill
	movq	%r14, -96(%rbp)                 # 8-byte Spill
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %r12d
	orb	%al, %r12b
	testb	$1, %r12b
	je	.LBB96_56
# %bb.54:                               # %cinch.rc.dup.tobj.exit352
	testb	$1, %r15b
	jne	.LBB96_58
# %bb.55:                               # %cinch.runtime.natDiv.exit119.thread687
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r15, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rdi
	jmp	.LBB96_63
.LBB96_56:                              # %bb0.i340
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_59
# %bb.57:                               # %bb2.i350
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB96_61
.LBB96_58:                              # %cinch.runtime.natDiv.exit119.thread
	movabsq	$4835703278458516699, %rcx      # imm = 0x431BDE82D7B634DB
	movq	%r15, %rax
	mulq	%rcx
	movq	%rdx, %rdi
	shrq	$18, %rdi
	orq	$1, %rdi
	movl	$0, -56(%rbp)                   # 4-byte Folded Spill
	jmp	.LBB96_69
.LBB96_59:                              # %bb3.i346
	je	.LBB96_61
# %bb.60:                               # %bb4.i348
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_61:                              # %bb0.i357
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r15, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %rdi
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_64
# %bb.62:                               # %bb2.i367
	decq	%rcx
	movq	%rcx, -8(%r15)
.LBB96_63:                              # %cinch.rc.drop.tobj.exit
	movb	$1, %al
	movl	%eax, -56(%rbp)                 # 4-byte Spill
	jmp	.LBB96_69
.LBB96_64:                              # %bb3.i363
	movb	$1, %cl
	movl	%ecx, -56(%rbp)                 # 4-byte Spill
	testq	%rax, %rax
	je	.LBB96_69
# %bb.65:                               # %bb3.i363
	movq	%r13, %r14
	movq	%rdi, %r13
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_67
# %bb.66:                               # %bb5.i364
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_68
.LBB96_67:                              # %bb4.i366
	callq	cinch_drop_mt@PLT
.LBB96_68:                              # %cinch.rc.drop.tobj.exit
	movq	%r13, %rdi
	movq	%r14, %r13
.LBB96_69:                              # %cinch.rc.drop.tobj.exit
	movq	%r13, %r14
	movabsq	$562949953421312, %rsi          # imm = 0x2000000000000
	testb	$1, %dil
	jne	.LBB96_72
# %bb.70:                               # %cinch.runtime.natCmp.exit182
	incq	%rsi
	movq	%rdi, %r13
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB96_73
.LBB96_71:                              # %bb22
	movq	%r15, %rdi
	callq	cinch.rc.drop.tobj
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.tobj
	movq	-96(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.tobj
	movq	%r13, %rdi
	callq	cinch.rc.drop.tobj
	leaq	"Data.Uuid.UuidClockOutOfRange.blk#h"+8(%rip), %r15
	movl	$256, %esi                      # imm = 0x100
	movq	%r14, %r13
	jmp	.LBB96_185
.LBB96_72:                              # %bb0.i176
	cmpq	%rsi, %rdi
	movq	%rdi, %r13
	jae	.LBB96_71
.LBB96_73:                              # %bb21
	movq	%r13, -64(%rbp)                 # 8-byte Spill
	testb	$1, %r12b
	movq	%r14, -104(%rbp)                # 8-byte Spill
	movl	-56(%rbp), %r14d                # 4-byte Reload
	jne	.LBB96_78
# %bb.74:                               # %bb0.i376
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_76
# %bb.75:                               # %bb2.i386
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB96_78
.LBB96_76:                              # %bb3.i382
	je	.LBB96_78
# %bb.77:                               # %bb4.i384
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_78:                              # %cinch.rc.dup.tobj.exit388
	testb	%r14b, %r14b
	je	.LBB96_80
# %bb.79:                               # %bb1.i98
	movl	$2000000001, %esi               # imm = 0x77359401
	movq	%r15, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, -80(%rbp)                 # 8-byte Spill
	jmp	.LBB96_81
.LBB96_80:                              # %bb0.i101
	movq	%r15, %rax
	shrq	$10, %rax
	movabsq	$19342813113834067, %rcx        # imm = 0x44B82FA09B5A53
	mulq	%rcx
	shrq	$10, %rdx
	orq	$1, %rdx
	movq	%rdx, -80(%rbp)                 # 8-byte Spill
.LBB96_81:                              # %cinch.runtime.natDiv.exit106
	testb	$1, %r12b
	jne	.LBB96_88
# %bb.82:                               # %bb0.i393
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_84
# %bb.83:                               # %bb2.i403
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB96_88
.LBB96_84:                              # %bb3.i399
	testq	%rax, %rax
	je	.LBB96_88
# %bb.85:                               # %bb3.i399
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_87
# %bb.86:                               # %bb5.i400
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_88
.LBB96_87:                              # %bb4.i402
	callq	cinch_drop_mt@PLT
.LBB96_88:                              # %cinch.rc.drop.tobj.exit405
	movabsq	$4835703278458516699, %rdx      # imm = 0x431BDE82D7B634DB
	testb	%r14b, %r14b
	je	.LBB96_90
# %bb.89:                               # %bb1.i124
	movq	%rdx, %r13
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r15, %rdi
	callq	cinch_nat_rem@PLT
	movq	%rax, %r14
	jmp	.LBB96_91
.LBB96_90:                              # %bb0.i127
	movq	%r15, %rcx
	shrq	%rcx
	movq	%rcx, %rax
	movq	%rdx, %r13
	mulq	%rdx
	shrq	$18, %rdx
	imull	$1000000, %edx, %eax            # imm = 0xF4240
	subl	%eax, %ecx
	leaq	1(,%rcx,2), %r14
.LBB96_91:                              # %cinch.runtime.natRem.exit
	testb	$1, %r12b
	movq	-72(%rbp), %r12                 # 8-byte Reload
	jne	.LBB96_98
# %bb.92:                               # %bb0.i411
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_94
# %bb.93:                               # %bb2.i421
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB96_98
.LBB96_94:                              # %bb3.i417
	testq	%rax, %rax
	je	.LBB96_98
# %bb.95:                               # %bb3.i417
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_97
# %bb.96:                               # %bb5.i418
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_98
.LBB96_97:                              # %bb4.i420
	callq	cinch_drop_mt@PLT
.LBB96_98:                              # %cinch.rc.drop.tobj.exit423
	movl	$8193, %esi                     # imm = 0x2001
	movq	%r14, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r15
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB96_105
# %bb.99:                               # %bb0.i429
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_101
# %bb.100:                              # %bb2.i439
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB96_105
.LBB96_101:                             # %bb3.i435
	testq	%rax, %rax
	je	.LBB96_105
# %bb.102:                              # %bb3.i435
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_104
# %bb.103:                              # %bb5.i436
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_105
.LBB96_104:                             # %bb4.i438
	callq	cinch_drop_mt@PLT
.LBB96_105:                             # %cinch.rc.drop.tobj.exit441
	testb	$1, %r15b
	jne	.LBB96_109
# %bb.106:                              # %cinch.runtime.natDiv.exit
	movl	$2000001, %esi                  # imm = 0x1E8481
	movq	%r15, %rdi
	callq	cinch_nat_div@PLT
	movq	%rax, %r13
	testq	%r15, %r15
	je	.LBB96_110
# %bb.107:                              # %bb0.i446
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	movq	-64(%rbp), %rdi                 # 8-byte Reload
	jl	.LBB96_117
# %bb.108:                              # %bb2.i456
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB96_111
.LBB96_109:                             # %cinch.runtime.natDiv.exit.thread
	movq	%r15, %rax
	mulq	%r13
	movq	%rdx, %r13
	shrq	$18, %r13
	orq	$1, %r13
.LBB96_110:                             # %cinch.rc.drop.tobj.exit458
	movq	-64(%rbp), %rdi                 # 8-byte Reload
.LBB96_111:                             # %cinch.rc.drop.tobj.exit458
	movl	$8193, %esi                     # imm = 0x2001
	callq	cinch_nat_mul@PLT
	movq	-64(%rbp), %rdi                 # 8-byte Reload
	movq	%rax, %r15
	testq	%rdi, %rdi
	sete	%al
	movl	%edi, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB96_121
# %bb.112:                              # %bb0.i464
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_114
# %bb.113:                              # %bb2.i474
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB96_121
.LBB96_114:                             # %bb3.i470
	testq	%rax, %rax
	je	.LBB96_121
# %bb.115:                              # %bb3.i470
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_120
# %bb.116:                              # %bb5.i471
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_121
.LBB96_117:                             # %bb3.i452
	testq	%rax, %rax
	je	.LBB96_111
# %bb.118:                              # %bb3.i452
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_173
# %bb.119:                              # %bb5.i453
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_110
.LBB96_120:                             # %bb4.i473
	callq	cinch_drop_mt@PLT
.LBB96_121:                             # %cinch.rc.drop.tobj.exit476
	movl	%r13d, %r14d
	andl	$1, %r14d
	testl	%r15d, %r14d
	je	.LBB96_124
# %bb.122:                              # %bb0.i73
	movq	%r15, %rcx
	shrq	%rcx
	movq	%r13, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB96_124
# %bb.123:                              # %bb3.i77
	leaq	1(,%rax,2), %rax
	jmp	.LBB96_125
.LBB96_124:                             # %bb1.i81
	movq	%r15, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_add@PLT
.LBB96_125:                             # %cinch.runtime.natAdd.exit83
	movq	%rax, -56(%rbp)                 # 8-byte Spill
                                        # kill: def $r12d killed $r12d killed $r12 def $r12
	andl	$1, %r12d
	testq	%r13, %r13
	sete	%al
	orb	%al, %r14b
	jne	.LBB96_132
# %bb.126:                              # %bb0.i481
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_128
# %bb.127:                              # %bb2.i491
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB96_132
.LBB96_128:                             # %bb3.i487
	testq	%rax, %rax
	je	.LBB96_132
# %bb.129:                              # %bb3.i487
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_131
# %bb.130:                              # %bb5.i488
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_132
.LBB96_131:                             # %bb4.i490
	callq	cinch_drop_mt@PLT
.LBB96_132:                             # %cinch.rc.drop.tobj.exit493
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	movq	-104(%rbp), %r13                # 8-byte Reload
	jne	.LBB96_139
# %bb.133:                              # %bb0.i498
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_135
# %bb.134:                              # %bb2.i508
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB96_139
.LBB96_135:                             # %bb3.i504
	testq	%rax, %rax
	je	.LBB96_139
# %bb.136:                              # %bb3.i504
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_138
# %bb.137:                              # %bb5.i505
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_139
.LBB96_138:                             # %bb4.i507
	callq	cinch_drop_mt@PLT
.LBB96_139:                             # %cinch.rc.drop.tobj.exit510
	movq	-80(%rbp), %rsi                 # 8-byte Reload
	testl	%esi, %r12d
	je	.LBB96_141
# %bb.140:                              # %bb0.i165
	movq	-72(%rbp), %r12                 # 8-byte Reload
	movq	%r12, %rax
	shrq	%rax
	movq	%rsi, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
	seta	%r14b
	jmp	.LBB96_142
.LBB96_141:                             # %bb1.i170
	movq	-72(%rbp), %r12                 # 8-byte Reload
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	setg	%r14b
.LBB96_142:                             # %cinch.runtime.natCmp.exit172
	movq	-96(%rbp), %r15                 # 8-byte Reload
	testb	$1, -42(%rbp)                   # 1-byte Folded Reload
	jne	.LBB96_149
# %bb.143:                              # %bb0.i515
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_145
# %bb.144:                              # %bb2.i525
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB96_149
.LBB96_145:                             # %bb3.i521
	testq	%rax, %rax
	je	.LBB96_149
# %bb.146:                              # %bb3.i521
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_148
# %bb.147:                              # %bb5.i522
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_149
.LBB96_148:                             # %bb4.i524
	callq	cinch_drop_mt@PLT
.LBB96_149:                             # %cinch.rc.drop.tobj.exit527
	testb	%r14b, %r14b
	je	.LBB96_153
# %bb.150:                              # %bb24
	testb	$1, -41(%rbp)                   # 1-byte Folded Reload
	movq	-56(%rbp), %r12                 # 8-byte Reload
	jne	.LBB96_162
# %bb.151:                              # %bb0.i534
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_170
# %bb.152:                              # %bb2.i544
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB96_162
.LBB96_153:                             # %bb25
	movl	%r15d, %r14d
	andl	$1, %r14d
	movq	-56(%rbp), %r12                 # 8-byte Reload
	testl	%r12d, %r14d
	je	.LBB96_158
# %bb.154:                              # %bb0.i154
	movq	%r12, %rax
	shrq	%rax
	movq	%r15, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
	ja	.LBB96_159
.LBB96_155:                             # %bb26
	testq	%r14, %r14
	je	.LBB96_160
# %bb.156:                              # %bb0.i
	movq	%r15, %rax
	shrq	%rax
	incq	%rax
	js	.LBB96_160
# %bb.157:                              # %bb3.i
	leaq	1(,%rax,2), %r14
	jmp	.LBB96_161
.LBB96_158:                             # %cinch.runtime.natCmp.exit161
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB96_155
.LBB96_159:                             # %bb27
	movq	%r15, %rdi
	callq	cinch.rc.drop.tobj
	jmp	.LBB96_162
.LBB96_160:                             # %bb1.i
	movl	$3, %esi
	movq	%r15, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
.LBB96_161:                             # %cinch.runtime.natAdd.exit
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	movq	%r15, %rdi
	callq	cinch.rc.drop.tobj
	movq	%r14, %r12
.LBB96_162:                             # %bb23
	movabsq	$2305843009213693951, %rsi      # imm = 0x1FFFFFFFFFFFFFFF
	testb	$1, %r12b
	jne	.LBB96_167
# %bb.163:                              # %cinch.runtime.natCmp.exit150
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jg	.LBB96_169
# %bb.164:                              # %bb28
	testq	%r12, %r12
	je	.LBB96_176
# %bb.165:                              # %bb0.i556
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB96_174
# %bb.166:                              # %bb2.i566
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB96_176
.LBB96_167:                             # %bb0.i144
	incq	%rsi
	cmpq	%rsi, %r12
	jae	.LBB96_169
# %bb.168:                              # %bb0.i193
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	-80(%rbp), %rax                 # 8-byte Reload
	movq	%rax, (%r14)
	movq	%r12, 8(%r14)
	shrq	%r12
	jmp	.LBB96_184
.LBB96_169:                             # %bb29
	movq	-80(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.tobj
	movq	%r12, %rdi
	callq	cinch.rc.drop.tobj
	leaq	"Data.Uuid.UuidClockExhausted.blk#h"+8(%rip), %r15
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB96_185
.LBB96_170:                             # %bb3.i540
	testq	%rax, %rax
	je	.LBB96_162
# %bb.171:                              # %bb3.i540
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_182
# %bb.172:                              # %bb5.i541
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_162
.LBB96_173:                             # %bb4.i455
	callq	cinch_drop_mt@PLT
	jmp	.LBB96_110
.LBB96_174:                             # %bb3.i562
	je	.LBB96_176
# %bb.175:                              # %bb4.i564
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
.LBB96_176:                             # %cinch.runtime.natToI64.exit
	movl	$16, %edi
	movl	$768, %esi                      # imm = 0x300
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	-80(%rbp), %rax                 # 8-byte Reload
	movq	%rax, (%r14)
	movq	%r12, 8(%r14)
	movq	%r12, %rdi
	callq	cinch_nat_to_i64@PLT
	movq	%r12, %rdi
	movq	%rax, %r12
	testq	%rdi, %rdi
	je	.LBB96_184
# %bb.177:                              # %bb0.i576
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB96_179
# %bb.178:                              # %bb2.i586
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB96_184
.LBB96_179:                             # %bb3.i582
	testq	%rax, %rax
	je	.LBB96_184
# %bb.180:                              # %bb3.i582
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB96_183
# %bb.181:                              # %bb5.i583
	callq	cinch_drop_slow@PLT
	jmp	.LBB96_184
.LBB96_182:                             # %bb4.i543
	callq	cinch_drop_mt@PLT
	jmp	.LBB96_162
.LBB96_183:                             # %bb4.i585
	callq	cinch_drop_mt@PLT
.LBB96_184:                             # %cinch.rc.drop.tobj.exit588
	movl	$16, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%r14, (%rax)
	movq	%r12, 8(%rax)
	movl	$257, %esi                      # imm = 0x101
.LBB96_185:                             # %bb15
	movl	$8, %edi
	callq	cinch_alloc_ctor@PLT
	movq	%r15, (%rax)
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB96_187
# %bb.186:                              # %bb31
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB96_187:                             # %bb33
	cmpq	$1, %rcx
	je	.LBB96_195
# %bb.188:                              # %bb33
	testq	%rcx, %rcx
	jne	.LBB96_41
# %bb.189:                              # %cinch.rc.unique.exit606
	movabsq	$34359738367, %r12              # imm = 0x7FFFFFFFF
	movq	-8(%rax), %rcx
	andq	%r12, %rcx
	cmpq	$1, %rcx
	je	.LBB96_191
# %bb.190:                              # %bb37
	movq	%r15, %rdi
	movq	%rax, %r14
	callq	cinch.rc.dup.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	xorl	%eax, %eax
.LBB96_191:                             # %bb38
	movq	-8(%rbx), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB96_193
# %bb.192:                              # %bb41
	movq	-16(%rbx), %rcx
	movq	(%rcx), %rcx
.LBB96_193:                             # %bb43
	testq	%rcx, %rcx
	jne	.LBB96_41
# %bb.203:                              # %bb44
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movq	(%rbx), %r14
	testq	%rbx, %rbx
	je	.LBB96_208
# %bb.204:                              # %cinch.rc.unique.exit615
	movq	-8(%rbx), %rax
	andq	%r12, %rax
	cmpq	$1, %rax
	jne	.LBB96_208
# %bb.205:                              # %bb45
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	jmp	.LBB96_209
.LBB96_195:                             # %cinch.rc.unique.exit626
	movq	%r13, %r12
	movabsq	$34359738367, %r13              # imm = 0x7FFFFFFFF
	movq	-8(%rax), %rcx
	andq	%r13, %rcx
	cmpq	$1, %rcx
	jne	.LBB96_198
# %bb.196:                              # %bb53
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB96_199
.LBB96_198:                             # %bb54
	movq	%r15, %rdi
	movq	%rax, %r14
	callq	cinch.rc.dup.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
.LBB96_199:                             # %bb55
	movq	%r12, %rdi
	callq	cinch.rc.drop.obj
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB96_201
# %bb.200:                              # %bb57
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB96_201:                             # %bb59
	testq	%rax, %rax
	jne	.LBB96_41
# %bb.206:                              # %cinch.rc.unique.exit635
	movq	(%r15), %r14
	movq	8(%r15), %r12
	movq	-8(%r15), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB96_211
# %bb.207:                              # %bb61
	movq	%r15, %rdi
	callq	cinch_free@PLT
	jmp	.LBB96_212
.LBB96_208:                             # %bb46
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
.LBB96_209:                             # %bb47
	leaq	-88(%rbp), %r8
	movl	$1, %edx
	movq	%r14, %rdi
	movq	%r13, %rsi
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movq	-72(%rbp), %rax                 # 8-byte Reload
	testq	%rax, %rax
	je	.LBB96_216
# %bb.210:                              # %bb49
	movabsq	$281509336449025, %rcx          # imm = 0x1000800000001
	movq	%rcx, -8(%rax)
	jmp	.LBB96_42
.LBB96_211:                             # %bb62
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
.LBB96_212:                             # %bb63
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB96_214
# %bb.213:                              # %bb66
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB96_214:                             # %bb68
	testq	%rax, %rax
	jne	.LBB96_41
# %bb.217:                              # %bb69
	movq	(%rbx), %r15
	testq	%rbx, %rbx
	je	.LBB96_220
# %bb.218:                              # %cinch.rc.unique.exit644
	movq	-8(%rbx), %rax
	andq	%r13, %rax
	cmpq	$1, %rax
	jne	.LBB96_220
# %bb.219:                              # %bb70
	movq	%rbx, %rdi
	callq	cinch_free@PLT
	jmp	.LBB96_221
.LBB96_216:                             # %bb48
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%r15, (%rax)
	jmp	.LBB96_42
.LBB96_220:                             # %bb71
	movq	%r15, %rdi
	callq	cinch.rc.dup.obj
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
.LBB96_221:                             # %bb72
	leaq	-88(%rbp), %r8
	movl	$1, %edx
	movq	%r15, %rdi
	movq	%r14, %rsi
	xorl	%ecx, %ecx
	callq	cinch_mvar_access@PLT
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
	movl	$17, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %rbx
	movq	%r12, %rax
	shrq	$60, %rax
	je	.LBB96_223
# %bb.222:                              # %bb74
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	leaq	"Data.Uuid.UuidClockOutOfRange.blk#h"+8(%rip), %rcx
	movq	%rcx, (%rax)
	jmp	.LBB96_42
.LBB96_223:                             # %bb73
	movabsq	$1030792151040, %r15            # imm = 0xF000000000
	movabsq	$274877906944, %r14             # imm = 0x4000000000
	testq	%rbx, %rbx
	je	.LBB96_226
# %bb.224:                              # %bb0.i229
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB96_226
# %bb.225:                              # %bb1.i243
	movq	(%rbx), %rax
	jmp	.LBB96_227
.LBB96_226:                             # %bb2.i237
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB96_227:                             # %cinch.runtime.natCmp.exit
	leaq	1(,%rax,2), %rdi
	movabsq	$9223372036854775807, %r13      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rax, %r13
	callq	cinch.rc.drop.tobj
	cmpq	$8, %r13
	jne	.LBB96_231
# %bb.228:                              # %bb75
	movq	%r12, %r13
	shlq	$4, %r13
	andq	$-65536, %r13                   # imm = 0xFFFF0000
	andl	$4095, %r12d                    # imm = 0xFFF
	movq	%rbx, %rdi
	callq	cinch.runtime.bytesData@PLT
	addq	%r13, %r12
	addq	$28672, %r12                    # imm = 0x7000
	movq	(%rax), %r13
	testq	%rbx, %rbx
	je	.LBB96_234
# %bb.229:                              # %bb0.i211
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB96_234
# %bb.230:                              # %bb1.i225
	movq	(%rbx), %rax
	jmp	.LBB96_235
.LBB96_231:                             # %bb76
	testq	%rbx, %rbx
	je	.LBB96_236
# %bb.232:                              # %bb0.i199
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB96_236
# %bb.233:                              # %bb1.i208
	movq	(%rbx), %rax
	jmp	.LBB96_237
.LBB96_234:                             # %bb2.i219
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB96_235:                             # %cinch.runtime.bytesLen.exit227
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
	movq	%r12, (%rax)
	movq	%r13, 8(%rax)
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	jmp	.LBB96_238
.LBB96_236:                             # %bb2.i
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB96_237:                             # %cinch.runtime.bytesLen.exit
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
.LBB96_238:                             # %common.ret
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	jmp	.LBB96_42
.Lfunc_end96:
	.size	"Effect.Uuid.uuid-new-v7", .Lfunc_end96-"Effect.Uuid.uuid-new-v7"
	.cfi_endproc
                                        # -- End function
	.globl	Effect.Uuid.Generator           # -- Begin function Effect.Uuid.Generator
	.p2align	4
	.type	Effect.Uuid.Generator,@function
Effect.Uuid.Generator:                  # @Effect.Uuid.Generator
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
.Lfunc_end97:
	.size	Effect.Uuid.Generator, .Lfunc_end97-Effect.Uuid.Generator
                                        # -- End function
	.globl	Main.fingerprint                # -- Begin function Main.fingerprint
	.p2align	4
	.type	Main.fingerprint,@function
Main.fingerprint:                       # @Main.fingerprint
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	movabsq	$9223372036854775807, %r13      # imm = 0x7FFFFFFFFFFFFFFF
	movabsq	$1030792151040, %r15            # imm = 0xF000000000
	movabsq	$274877906944, %r12             # imm = 0x4000000000
	testq	%rdi, %rdi
	je	.LBB98_3
# %bb.1:                                # %bb0.i190
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB98_5
# %bb.2:                                # %cinch.runtime.bytesLen.exit206.thread
	movq	(%rbx), %rax
	jmp	.LBB98_6
.LBB98_3:                               # %cinch.runtime.natCmp.exit134
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	leaq	1(,%rax,2), %r14
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testq	%r13, %rax
	jne	.LBB98_13
	jmp	.LBB98_36
.LBB98_5:                               # %cinch.runtime.bytesLen.exit206
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB98_6:                               # %bb0.i172
	leaq	1(,%rax,2), %r14
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB98_9
# %bb.7:                                # %cinch.runtime.natCmp.exit134.thread
	testq	%r13, (%rbx)
	je	.LBB98_36
.LBB98_10:                              # %bb3.i260
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB98_13
# %bb.11:                               # %bb4.i273
	cmpq	$0, (%rbx)
	je	.LBB98_13
# %bb.12:                               # %bb7.i276
	movq	8(%rbx), %rax
	movzbl	(%rax), %eax
	jmp	.LBB98_14
.LBB98_13:                              # %bb5.i268
	movq	%rbx, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_get@PLT
.LBB98_14:                              # %cinch.runtime.bytesGet.exit283
	leaq	1(,%rax,2), %rsi
	jmp	.LBB98_15
.LBB98_9:                               # %cinch.runtime.natCmp.exit134.thread646
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testq	%r13, %rax
	jne	.LBB98_10
.LBB98_36:
	movl	$1, %esi
.LBB98_15:                              # %bb0.i49
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB98_17
# %bb.16:                               # %bb3.i54
	leaq	1(,%rax,2), %r12
	testq	%rbx, %rbx
	jne	.LBB98_18
	jmp	.LBB98_20
.LBB98_17:                              # %bb1.i58
	movq	%r14, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%rbx, %rbx
	je	.LBB98_20
.LBB98_18:                              # %bb0.i154
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB98_20
# %bb.19:                               # %bb0.i117.thread
	movq	(%rbx), %rax
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB98_27
	jmp	.LBB98_22
.LBB98_20:                              # %bb0.i117
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB98_27
# %bb.21:                               # %bb0.i225
	testq	%rbx, %rbx
	je	.LBB98_25
.LBB98_22:                              # %bb3.i230
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB98_25
# %bb.23:                               # %bb4.i243
	cmpq	(%rbx), %rsi
	jae	.LBB98_25
# %bb.24:                               # %bb7.i246
	movq	8(%rbx), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB98_26
.LBB98_25:                              # %bb5.i238
	movq	%rbx, %rdi
	callq	cinch_bytes_get@PLT
.LBB98_26:                              # %cinch.runtime.bytesGet.exit255
	leaq	1(,%rax,2), %rcx
.LBB98_27:                              # %bb4
	testb	$1, %r12b
	je	.LBB98_30
# %bb.28:                               # %bb0.i34
	movq	%r12, %rdx
	shrq	%rdx
	movq	%rcx, %rax
	shrq	%rax
	addq	%rdx, %rax
	js	.LBB98_30
# %bb.29:                               # %cinch.runtime.natAdd.exit45.thread
	leaq	1(,%rax,2), %r15
	jmp	.LBB98_38
.LBB98_30:                              # %cinch.runtime.natAdd.exit45
	movq	%r12, %rdi
	movq	%rcx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r15
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB98_38
# %bb.31:                               # %bb0.i404
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB98_33
# %bb.32:                               # %bb2.i414
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB98_38
.LBB98_33:                              # %bb3.i410
	testq	%rax, %rax
	je	.LBB98_38
# %bb.34:                               # %bb3.i410
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB98_37
# %bb.35:                               # %bb5.i411
	callq	cinch_drop_slow@PLT
	jmp	.LBB98_38
.LBB98_37:                              # %bb4.i413
	callq	cinch_drop_mt@PLT
.LBB98_38:                              # %cinch.runtime.natCmp.exit113
	leaq	-2(%r14), %r12
	cmpq	$2, %r14
	movl	$1, %r14d
	cmovbq	%r14, %r12
	testq	%rbx, %rbx
	je	.LBB98_41
# %bb.39:                               # %bb0.i142
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB98_41
# %bb.40:                               # %cinch.runtime.natCmp.exit103.thread
	shrq	%r12
	andq	(%rbx), %r13
	cmpq	%r13, %r12
	jb	.LBB98_43
	jmp	.LBB98_48
.LBB98_41:                              # %cinch.runtime.natCmp.exit103
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	shrq	%r12
	andq	%r13, %rax
	cmpq	%rax, %r12
	jae	.LBB98_48
# %bb.42:                               # %bb2.i211
	testq	%rbx, %rbx
	je	.LBB98_46
.LBB98_43:                              # %bb3.i214
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB98_46
# %bb.44:                               # %bb4.i
	cmpq	(%rbx), %r12
	jae	.LBB98_46
# %bb.45:                               # %bb7.i
	movq	8(%rbx), %rax
	movzbl	(%rax,%r12), %eax
	jmp	.LBB98_47
.LBB98_46:                              # %bb5.i
	movq	%rbx, %rdi
	movq	%r12, %rsi
	callq	cinch_bytes_get@PLT
.LBB98_47:                              # %cinch.runtime.bytesGet.exit
	leaq	1(,%rax,2), %r14
.LBB98_48:                              # %bb10
	testb	$1, %r15b
	je	.LBB98_51
# %bb.49:                               # %bb0.i
	movq	%r15, %rcx
	shrq	%rcx
	movq	%r14, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB98_51
# %bb.50:                               # %cinch.runtime.natAdd.exit.thread
	leaq	1(,%rax,2), %rbx
	testb	$1, %bl
	je	.LBB98_58
.LBB98_73:                              # %cinch.runtime.natCmp.exit.thread
	shrq	%rbx
	movq	%rbx, %rax
.LBB98_74:                              # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB98_51:                              # %cinch.runtime.natAdd.exit
	.cfi_def_cfa %rbp, 16
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB98_52
.LBB98_57:                              # %cinch.rc.drop.tobj.exit535
	testb	$1, %bl
	jne	.LBB98_73
.LBB98_58:                              # %cinch.runtime.natCmp.exit
	movq	%rbx, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB98_62
# %bb.59:                               # %bb14
	testq	%rbx, %rbx
	je	.LBB98_65
# %bb.60:                               # %bb0.i576
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB98_66
# %bb.61:                               # %bb2.i586
	decq	%rcx
	movq	%rcx, -8(%rbx)
	xorl	%eax, %eax
	jmp	.LBB98_74
.LBB98_52:                              # %bb0.i523
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB98_54
# %bb.53:                               # %bb2.i533
	decq	%rcx
	movq	%rcx, -8(%r15)
	testb	$1, %bl
	je	.LBB98_58
	jmp	.LBB98_73
.LBB98_62:                              # %cinch.runtime.natToI64.exit
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	testq	%rbx, %rbx
	je	.LBB98_74
# %bb.63:                               # %bb0.i559
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB98_69
# %bb.64:                               # %bb2.i569
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB98_74
.LBB98_54:                              # %bb3.i529
	testq	%rax, %rax
	je	.LBB98_57
# %bb.55:                               # %bb3.i529
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB98_72
# %bb.56:                               # %bb5.i530
	callq	cinch_drop_slow@PLT
	jmp	.LBB98_57
.LBB98_65:
	xorl	%eax, %eax
	jmp	.LBB98_74
.LBB98_66:                              # %bb3.i582
	testq	%rax, %rax
	je	.LBB98_74
# %bb.67:                               # %bb3.i582
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB98_75
# %bb.68:                               # %bb5.i583
	callq	cinch_drop_slow@PLT
	xorl	%eax, %eax
	jmp	.LBB98_74
.LBB98_69:                              # %bb3.i565
	testq	%rcx, %rcx
	je	.LBB98_74
# %bb.70:                               # %bb3.i565
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB98_76
# %bb.71:                               # %bb5.i566
	callq	cinch_drop_slow@PLT
	jmp	.LBB98_77
.LBB98_72:                              # %bb4.i532
	callq	cinch_drop_mt@PLT
	testb	$1, %bl
	je	.LBB98_58
	jmp	.LBB98_73
.LBB98_75:                              # %bb4.i585
	callq	cinch_drop_mt@PLT
	xorl	%eax, %eax
	jmp	.LBB98_74
.LBB98_76:                              # %bb4.i568
	callq	cinch_drop_mt@PLT
.LBB98_77:                              # %common.ret
	movq	%r14, %rax
	jmp	.LBB98_74
.Lfunc_end98:
	.size	Main.fingerprint, .Lfunc_end98-Main.fingerprint
	.cfi_endproc
                                        # -- End function
	.globl	"Main.observe-uuid"             # -- Begin function Main.observe-uuid
	.p2align	4
	.type	"Main.observe-uuid",@function
"Main.observe-uuid":                    # @Main.observe-uuid
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %r14
	movq	%rdi, %r13
	movabsq	$9223372036854775807, %r15      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rsi, %rsi
	je	.LBB99_7
# %bb.1:                                # %bb0.i33
	movq	%r13, -48(%rbp)                 # 8-byte Spill
	movabsq	$1030792151040, %r13            # imm = 0xF000000000
	movabsq	$274877906944, %rbx             # imm = 0x4000000000
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	%rbx, %rax
	jne	.LBB99_2
# %bb.9:                                # %cinch.runtime.natCmp.exit.thread
	movq	(%r14), %rax
	jmp	.LBB99_3
.LBB99_7:                               # %cinch.runtime.natCmp.exit
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r15, %rax
	cmpq	$16, %rax
	jne	.LBB99_10
# %bb.8:                                # %cinch.runtime.bytesData.exit102.thread147
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	(%rax), %r12
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	8(%rax), %rax
	jmp	.LBB99_20
.LBB99_2:                               # %cinch.runtime.natCmp.exit.thread156
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB99_3:                               # %cinch.runtime.natCmp.exit.thread156
	andq	%r15, %rax
	cmpq	$16, %rax
	jne	.LBB99_10
# %bb.4:                                # %bb0.i63
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	%rbx, %rax
	jne	.LBB99_12
# %bb.5:                                # %cinch.runtime.bytesData.exit.thread
	cmpq	$0, (%r14)
	movl	$1, %eax
	je	.LBB99_13
# %bb.6:                                # %cinch.runtime.bytesData.exit.thread
	movq	8(%r14), %rax
	jmp	.LBB99_13
.LBB99_10:
	xorl	%eax, %eax
	jmp	.LBB99_11
.LBB99_12:                              # %cinch.runtime.bytesData.exit
	movq	%r14, %rdi
	callq	cinch_bytes_data@PLT
.LBB99_13:                              # %bb0.i16
	movq	(%rax), %r12
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	%rbx, %rax
	je	.LBB99_15
# %bb.14:                               # %cinch.runtime.bytesLen.exit31
	movq	%r14, %rdi
	callq	cinch_bytes_len@PLT
.LBB99_15:                              # %bb0.i87
	movq	-8(%r14), %rax
	andq	%r13, %rax
	cmpq	%rbx, %rax
	jne	.LBB99_18
# %bb.16:                               # %cinch.runtime.bytesData.exit102.thread
	cmpq	$0, (%r14)
	movl	$1, %eax
	je	.LBB99_19
# %bb.17:                               # %cinch.runtime.bytesData.exit102.thread
	movq	8(%r14), %rax
	jmp	.LBB99_19
.LBB99_18:                              # %cinch.runtime.bytesData.exit102
	movq	%r14, %rdi
	callq	cinch_bytes_data@PLT
.LBB99_19:                              # %bb0.i9
	movq	8(%rax), %rax
	movq	-8(%r14), %rcx
	andq	%r13, %rcx
	cmpq	%rbx, %rcx
	movq	-48(%rbp), %r13                 # 8-byte Reload
	je	.LBB99_21
.LBB99_20:                              # %bb2.i
	movq	%r14, %rdi
	movq	%rax, %rbx
	callq	cinch_bytes_len@PLT
	movq	%rbx, %rax
.LBB99_21:                              # %cinch.runtime.bytesLen.exit
	movq	%r12, %rcx
	shrq	$52, %rcx
	andl	$15, %ecx
	xorq	%r13, %rcx
	movl	%eax, %edx
	andl	$192, %edx
	xorq	$128, %rdx
	incq	%r15
	xorl	%esi, %esi
	orq	%rcx, %rdx
	cmoveq	%r15, %rsi
	movzbl	%r12b, %ecx
	shrq	$56, %r12
	addq	%rcx, %r12
	movzbl	%al, %ecx
	shrq	$56, %rax
	addq	%rcx, %rax
	addq	%r12, %rax
	orq	%rsi, %rax
.LBB99_11:                              # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end99:
	.size	"Main.observe-uuid", .Lfunc_end99-"Main.observe-uuid"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.random-loop#cc"           # -- Begin function Main.random-loop#cc
	.p2align	4
	.type	"Main.random-loop#cc",@function
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
	.size	"Main.random-loop#cc", .Lfunc_end100-"Main.random-loop#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.random-loop"              # -- Begin function Main.random-loop
	.p2align	4
	.type	"Main.random-loop",@function
"Main.random-loop":                     # @Main.random-loop
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$16, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	48(%rsp), %rax
	movq	%rax, (%rsp)
	callq	"Main.random-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	addq	$16, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end101:
	.size	"Main.random-loop", .Lfunc_end101-"Main.random-loop"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.allocation-loop#cc"       # -- Begin function Main.allocation-loop#cc
	.p2align	4
	.type	"Main.allocation-loop#cc",@function
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
	.size	"Main.allocation-loop#cc", .Lfunc_end102-"Main.allocation-loop#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.allocation-loop"          # -- Begin function Main.allocation-loop
	.p2align	4
	.type	"Main.allocation-loop",@function
"Main.allocation-loop":                 # @Main.allocation-loop
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	callq	"Main.allocation-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end103:
	.size	"Main.allocation-loop", .Lfunc_end103-"Main.allocation-loop"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.counter-loop#cc"          # -- Begin function Main.counter-loop#cc
	.p2align	4
	.type	"Main.counter-loop#cc",@function
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
	.size	"Main.counter-loop#cc", .Lfunc_end104-"Main.counter-loop#cc"
                                        # -- End function
	.globl	"Main.counter-loop"             # -- Begin function Main.counter-loop
	.p2align	4
	.type	"Main.counter-loop",@function
"Main.counter-loop":                    # @Main.counter-loop
# %bb.0:                                # %entry
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	movq	%r8, %rbx
	movq	%rcx, %r14
	movq	%rdx, %r15
	cmpl	%esi, %edi
	jbe	.LBB105_7
# %bb.1:                                # %cinch.runtime.natFromI64.exit68.preheader.i
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	movl	%esi, %ecx
	.p2align	4
.LBB105_2:                              # %cinch.runtime.natFromI64.exit68.i
                                        # =>This Inner Loop Header: Depth=1
	leaq	1(%rcx), %rsi
	xorq	%rcx, %r14
	imulq	%rax, %r14
	cmpl	%esi, %edi
	jbe	.LBB105_5
# %bb.3:                                # %cinch.runtime.natFromI64.exit60.i
                                        #   in Loop: Header=BB105_2 Depth=1
	leaq	2(%rcx), %rdx
	movl	%esi, %esi
	xorq	%rsi, %r14
	imulq	%rax, %r14
	cmpl	%edx, %edi
	jbe	.LBB105_6
# %bb.4:                                # %cinch.runtime.natFromI64.exit.i
                                        #   in Loop: Header=BB105_2 Depth=1
	addq	$3, %rcx
	movl	%ecx, %esi
	addq	$3, %r15
	movl	%edx, %edx
	xorq	%rdx, %r14
	imulq	%rax, %r14
	cmpl	%ecx, %edi
	movq	%rsi, %rcx
	ja	.LBB105_2
	jmp	.LBB105_7
.LBB105_5:                              # %common.ret.split.loop.exit253.i
	incq	%r15
	jmp	.LBB105_7
.LBB105_6:                              # %common.ret.split.loop.exit256.i
	addq	$2, %r15
.LBB105_7:                              # %"Main.counter-loop#cc.exit"
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%r15, (%rax)
	movq	%r14, 8(%rax)
	movq	%rbx, 16(%rax)
	popq	%rbx
	popq	%r14
	popq	%r15
	retq
.Lfunc_end105:
	.size	"Main.counter-loop", .Lfunc_end105-"Main.counter-loop"
                                        # -- End function
	.globl	"Main.clock-loop#cc"            # -- Begin function Main.clock-loop#cc
	.p2align	4
	.type	"Main.clock-loop#cc",@function
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
	.size	"Main.clock-loop#cc", .Lfunc_end106-"Main.clock-loop#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.clock-loop"               # -- Begin function Main.clock-loop
	.p2align	4
	.type	"Main.clock-loop",@function
"Main.clock-loop":                      # @Main.clock-loop
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	callq	"Main.clock-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end107:
	.size	"Main.clock-loop", .Lfunc_end107-"Main.clock-loop"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v4-loop#cc"               # -- Begin function Main.v4-loop#cc
	.p2align	4
	.type	"Main.v4-loop#cc",@function
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
	.size	"Main.v4-loop#cc", .Lfunc_end108-"Main.v4-loop#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v4-loop"                  # -- Begin function Main.v4-loop
	.p2align	4
	.type	"Main.v4-loop",@function
"Main.v4-loop":                         # @Main.v4-loop
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	callq	"Main.v4-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end109:
	.size	"Main.v4-loop", .Lfunc_end109-"Main.v4-loop"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v7-loop#cc"               # -- Begin function Main.v7-loop#cc
	.p2align	4
	.type	"Main.v7-loop#cc",@function
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
	.size	"Main.v7-loop#cc", .Lfunc_end110-"Main.v7-loop#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v7-loop"                  # -- Begin function Main.v7-loop
	.p2align	4
	.type	"Main.v7-loop",@function
"Main.v7-loop":                         # @Main.v7-loop
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$16, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	48(%rsp), %rax
	movq	%rax, (%rsp)
	callq	"Main.v7-loop#cc"@PLT
	subq	$8, %rsp
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	addq	$16, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end111:
	.size	"Main.v7-loop", .Lfunc_end111-"Main.v7-loop"
	.cfi_endproc
                                        # -- End function
	.globl	Main.decimal                    # -- Begin function Main.decimal
	.p2align	4
	.type	Main.decimal,@function
Main.decimal:                           # @Main.decimal
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rsi, %r14
	movq	%rdi, %r15
.LBB112_1:                              # %bb0
                                        # =>This Inner Loop Header: Depth=1
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB112_3
# %bb.2:                                # %bb2
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB112_3:                              # %bb4
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$1, %rax
	jne	.LBB112_137
# %bb.4:                                # %bb6
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	(%r15), %rdi
	movq	8(%r15), %r13
	testq	%rdi, %rdi
	js	.LBB112_11
# %bb.5:                                # %bb0.i116
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	1(,%rdi,2), %r15
	testb	$1, %r15b
	jne	.LBB112_12
.LBB112_6:                              # %cinch.runtime.natCmp.exit105
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$97, %esi
	movq	%r15, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB112_8
# %bb.7:                                # %cinch.runtime.natCmp.exit95
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$115, %esi
	movq	%r15, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB112_17
.LBB112_8:                              # %bb12
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%r15, %r15
	je	.LBB112_23
# %bb.9:                                # %bb0.i143
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_14
# %bb.10:                               # %bb2.i153
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB112_22
	.p2align	4
.LBB112_11:                             # %bb1.i113
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r15
	testb	$1, %r15b
	je	.LBB112_6
.LBB112_12:                             # %bb0.i99
                                        #   in Loop: Header=BB112_1 Depth=1
	addq	$-96, %r15
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	cmpq	$20, %r15
	jae	.LBB112_23
	jmp	.LBB112_13
.LBB112_14:                             # %bb3.i149
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rax, %rax
	je	.LBB112_23
# %bb.15:                               # %bb3.i149
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_21
# %bb.16:                               # %bb5.i150
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_22
.LBB112_17:                             # %cinch.runtime.natSub.exit66
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$97, %esi
	movq	%r15, %rdi
	callq	cinch_nat_sub@PLT
	testq	%r15, %r15
	je	.LBB112_20
# %bb.18:                               # %bb0.i128
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB112_84
# %bb.19:                               # %bb2.i
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB112_20
.LBB112_21:                             # %bb4.i152
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
.LBB112_22:                             # %bb7
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	jmp	.LBB112_23
.LBB112_84:                             # %bb3.i134
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rcx, %rcx
	je	.LBB112_20
# %bb.85:                               # %bb3.i134
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%rax, %r12
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB112_133
# %bb.86:                               # %bb5.i
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_134
.LBB112_20:                             #   in Loop: Header=BB112_1 Depth=1
	movq	%rax, %r15
	jmp	.LBB112_13
.LBB112_133:                            # %bb4.i
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
.LBB112_134:                            # %cinch.rc.drop.tobj.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %r15
	.p2align	4
.LBB112_13:                             # %cinch.rc.drop.tobj.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%r15, (%rax)
.LBB112_23:                             # %bb7
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB112_25
# %bb.24:                               # %bb14
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB112_25:                             # %bb16
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$1, %rax
	jne	.LBB112_138
# %bb.26:                               # %cinch.rc.unique.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	(%rdi), %rbx
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB112_28
# %bb.27:                               # %bb19
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_free@PLT
	jmp	.LBB112_39
	.p2align	4
.LBB112_28:                             # %bb20
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_33
# %bb.29:                               # %bb0.i192
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB112_31
# %bb.30:                               # %bb2.i201
                                        #   in Loop: Header=BB112_1 Depth=1
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB112_33
.LBB112_31:                             # %bb3.i198
                                        #   in Loop: Header=BB112_1 Depth=1
	je	.LBB112_33
# %bb.32:                               # %bb4.i199
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%rdi, %r15
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
	movq	%r15, %rdi
	.p2align	4
.LBB112_33:                             # %bb0.i204
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_35
# %bb.34:                               # %bb2.i214
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB112_39
	.p2align	4
.LBB112_35:                             # %bb3.i210
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_39
# %bb.36:                               # %bb3.i210
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_38
# %bb.37:                               # %bb5.i211
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_39
.LBB112_38:                             # %bb4.i213
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_39:                             # %bb21
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r12
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_46
# %bb.40:                               # %bb0.i222
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_42
# %bb.41:                               # %bb2.i232
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB112_46
	.p2align	4
.LBB112_42:                             # %bb3.i228
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_46
# %bb.43:                               # %bb3.i228
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_45
# %bb.44:                               # %bb5.i229
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_46
.LBB112_45:                             # %bb4.i231
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_46:                             # %cinch.rc.drop.tobj.exit234
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	%ebx, %r15d
	andl	$1, %r15d
	testl	%r12d, %r15d
	je	.LBB112_49
# %bb.47:                               # %bb0.i28
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %rcx
	shrq	%rcx
	movq	%rbx, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB112_49
# %bb.48:                               # %bb3.i33
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	1(,%rax,2), %r14
	jmp	.LBB112_50
	.p2align	4
.LBB112_49:                             # %bb1.i37
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %rdi
	movq	%rbx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
.LBB112_50:                             # %cinch.runtime.natAdd.exit39
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rbx, %rbx
	sete	%al
	orb	%al, %r15b
	jne	.LBB112_57
# %bb.51:                               # %bb0.i239
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_53
# %bb.52:                               # %bb2.i249
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB112_57
	.p2align	4
.LBB112_53:                             # %bb3.i245
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_57
# %bb.54:                               # %bb3.i245
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_56
# %bb.55:                               # %bb5.i246
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_57
.LBB112_56:                             # %bb4.i248
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_57:                             # %cinch.rc.drop.tobj.exit251
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_64
# %bb.58:                               # %bb0.i256
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_60
# %bb.59:                               # %bb2.i266
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB112_64
	.p2align	4
.LBB112_60:                             # %bb3.i262
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_64
# %bb.61:                               # %bb3.i262
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_63
# %bb.62:                               # %bb5.i263
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_64
.LBB112_63:                             # %bb4.i265
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_64:                             # %cinch.rc.drop.tobj.exit268
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB112_66
# %bb.65:                               # %bb23
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB112_66:                             # %bb25
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$1, %rax
	jne	.LBB112_142
# %bb.67:                               # %bb27
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	(%r13), %rdi
	movq	8(%r13), %r15
	testq	%rdi, %rdi
	js	.LBB112_74
# %bb.68:                               # %bb0.i110
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	1(,%rdi,2), %r12
	testb	$1, %r12b
	jne	.LBB112_75
.LBB112_69:                             # %cinch.runtime.natCmp.exit85
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$97, %esi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	js	.LBB112_71
# %bb.70:                               # %cinch.runtime.natCmp.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$115, %esi
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB112_80
.LBB112_71:                             # %bb33
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%r12, %r12
	je	.LBB112_89
# %bb.72:                               # %bb0.i296
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_77
# %bb.73:                               # %bb2.i306
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB112_88
	.p2align	4
.LBB112_74:                             # %bb1.i107
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	je	.LBB112_69
.LBB112_75:                             # %bb0.i79
                                        #   in Loop: Header=BB112_1 Depth=1
	addq	$-96, %r12
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	cmpq	$20, %r12
	jae	.LBB112_89
	jmp	.LBB112_76
.LBB112_77:                             # %bb3.i302
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	testq	%rax, %rax
	je	.LBB112_89
# %bb.78:                               # %bb3.i302
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_87
# %bb.79:                               # %bb5.i303
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_88
.LBB112_80:                             # %cinch.runtime.natSub.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$97, %esi
	movq	%r12, %rdi
	callq	cinch_nat_sub@PLT
	testq	%r12, %r12
	je	.LBB112_83
# %bb.81:                               # %bb0.i277
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r12), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB112_130
# %bb.82:                               # %bb2.i287
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rdx
	movq	%rdx, -8(%r12)
	jmp	.LBB112_83
.LBB112_87:                             # %bb4.i305
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
.LBB112_88:                             # %bb28
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	"Data.Core.None.blk#h"+8(%rip), %rdi
	jmp	.LBB112_89
.LBB112_130:                            # %bb3.i283
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rcx, %rcx
	je	.LBB112_83
# %bb.131:                              # %bb3.i283
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%rax, %r13
	movq	%r12, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB112_135
# %bb.132:                              # %bb5.i284
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_136
.LBB112_83:                             #   in Loop: Header=BB112_1 Depth=1
	movq	%rax, %r12
	jmp	.LBB112_76
.LBB112_135:                            # %bb4.i286
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
.LBB112_136:                            # %cinch.rc.drop.tobj.exit289
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r13, %r12
	.p2align	4
.LBB112_76:                             # %cinch.rc.drop.tobj.exit289
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rdi
	movq	%r12, (%rax)
.LBB112_89:                             # %bb28
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB112_91
# %bb.90:                               # %bb35
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB112_91:                             # %bb37
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$1, %rax
	jne	.LBB112_138
# %bb.92:                               # %cinch.rc.unique.exit348
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	(%rdi), %r12
	movq	-8(%rdi), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB112_94
# %bb.93:                               # %bb40
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_free@PLT
	jmp	.LBB112_105
	.p2align	4
.LBB112_94:                             # %bb41
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_99
# %bb.95:                               # %bb0.i353
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB112_97
# %bb.96:                               # %bb2.i363
                                        #   in Loop: Header=BB112_1 Depth=1
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB112_99
.LBB112_97:                             # %bb3.i359
                                        #   in Loop: Header=BB112_1 Depth=1
	je	.LBB112_99
# %bb.98:                               # %bb4.i361
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%rdi, %rbx
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
	movq	%rbx, %rdi
	.p2align	4
.LBB112_99:                             # %bb0.i367
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_101
# %bb.100:                              # %bb2.i377
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB112_105
.LBB112_101:                            # %bb3.i373
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_105
# %bb.102:                              # %bb3.i373
                                        #   in Loop: Header=BB112_1 Depth=1
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_104
# %bb.103:                              # %bb5.i374
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_105
.LBB112_104:                            # %bb4.i376
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_105:                            # %bb42
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	$21, %esi
	movq	%r14, %rdi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r13
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_112
# %bb.106:                              # %bb0.i385
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_108
# %bb.107:                              # %bb2.i395
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB112_112
.LBB112_108:                            # %bb3.i391
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_112
# %bb.109:                              # %bb3.i391
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_111
# %bb.110:                              # %bb5.i392
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_112
.LBB112_111:                            # %bb4.i394
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_112:                            # %cinch.rc.drop.tobj.exit397
                                        #   in Loop: Header=BB112_1 Depth=1
	movl	%r12d, %ebx
	andl	$1, %ebx
	testl	%r13d, %ebx
	je	.LBB112_115
# %bb.113:                              # %bb0.i
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r13, %rcx
	shrq	%rcx
	movq	%r12, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB112_115
# %bb.114:                              # %bb3.i
                                        #   in Loop: Header=BB112_1 Depth=1
	leaq	1(,%rax,2), %r14
	jmp	.LBB112_116
	.p2align	4
.LBB112_115:                            # %bb1.i
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
.LBB112_116:                            # %cinch.runtime.natAdd.exit
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%r12, %r12
	sete	%al
	orb	%al, %bl
	jne	.LBB112_123
# %bb.117:                              # %bb0.i402
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_119
# %bb.118:                              # %bb2.i412
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB112_123
.LBB112_119:                            # %bb3.i408
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_123
# %bb.120:                              # %bb3.i408
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_122
# %bb.121:                              # %bb5.i409
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_123
.LBB112_122:                            # %bb4.i411
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	.p2align	4
.LBB112_123:                            # %cinch.rc.drop.tobj.exit414
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%r13, %r13
	sete	%al
	movl	%r13d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_1
# %bb.124:                              # %bb0.i419
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_126
# %bb.125:                              # %bb2.i429
                                        #   in Loop: Header=BB112_1 Depth=1
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB112_1
.LBB112_126:                            # %bb3.i425
                                        #   in Loop: Header=BB112_1 Depth=1
	testq	%rax, %rax
	je	.LBB112_1
# %bb.127:                              # %bb3.i425
                                        #   in Loop: Header=BB112_1 Depth=1
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_129
# %bb.128:                              # %bb5.i426
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_1
.LBB112_129:                            # %bb4.i428
                                        #   in Loop: Header=BB112_1 Depth=1
	callq	cinch_drop_mt@PLT
	jmp	.LBB112_1
.LBB112_138:                            # %bb16
	testq	%rax, %rax
	jne	.LBB112_145
# %bb.139:                              # %bb17
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB112_151
# %bb.140:                              # %bb0.i160
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_146
# %bb.141:                              # %bb2.i170
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB112_151
.LBB112_137:                            # %bb4
	testq	%rax, %rax
	movl	$0, %eax
	je	.LBB112_143
	jmp	.LBB112_158
.LBB112_142:                            # %bb25
	testq	%rax, %rax
	jne	.LBB112_145
.LBB112_143:                            # %bb26
	movq	%r14, %rax
	jmp	.LBB112_158
.LBB112_145:
	xorl	%eax, %eax
	jmp	.LBB112_158
.LBB112_146:                            # %bb3.i166
	testq	%rax, %rax
	je	.LBB112_151
# %bb.147:                              # %bb3.i166
	movq	%rdi, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_149
# %bb.148:                              # %bb5.i167
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_150
.LBB112_149:                            # %bb4.i169
	callq	cinch_drop_mt@PLT
.LBB112_150:                            # %bb0.i174
	movq	%rbx, %rdi
.LBB112_151:                            # %bb0.i174
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB112_152
# %bb.155:                              # %bb2.i184
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB112_157
.LBB112_152:                            # %bb3.i333
	testq	%rax, %rax
	je	.LBB112_157
# %bb.153:                              # %bb3.i180
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB112_156
# %bb.154:                              # %bb5.i181
	callq	cinch_drop_slow@PLT
	jmp	.LBB112_157
.LBB112_156:                            # %bb4.i183
	callq	cinch_drop_mt@PLT
.LBB112_157:
	movl	$1, %eax
.LBB112_158:                            # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end112:
	.size	Main.decimal, .Lfunc_end112-Main.decimal
	.cfi_endproc
                                        # -- End function
	.globl	Main.finish                     # -- Begin function Main.finish
	.p2align	4
	.type	Main.finish,@function
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
	.size	Main.finish, .Lfunc_end113-Main.finish
	.cfi_endproc
                                        # -- End function
	.globl	Main.measure                    # -- Begin function Main.measure
	.p2align	4
	.type	Main.measure,@function
Main.measure:                           # @Main.measure
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
	movq	%rcx, %r13
	movq	%rdx, %r12
	movq	%rsi, %r14
	movq	%rdi, %r15
	movabsq	$8589934590, %rax               # imm = 0x1FFFFFFFE
	movq	%rcx, %rbx
	andq	$1, %rbx
	jne	.LBB114_1
# %bb.5:                                # %cinch.runtime.natCmp.exit229
	leaq	1(%rax), %rsi
	movq	%r13, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jg	.LBB114_2
	jmp	.LBB114_6
.LBB114_1:                              # %bb0.i223
	addq	$2, %rax
	cmpq	%rax, %r13
	jae	.LBB114_2
.LBB114_6:                              # %bb2
	testb	$1, %r12b
	jne	.LBB114_7
# %bb.8:                                # %cinch.runtime.natCmp.exit219
	movl	$8193, %esi                     # imm = 0x2001
	movq	%r12, %rdi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB114_9
.LBB114_2:                              # %bb5
	testq	%r14, %r14
	je	.LBB114_328
# %bb.3:                                # %bb0.i1072
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_324
# %bb.4:                                # %bb2.i1082
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_328
.LBB114_324:                            # %bb3.i1078
	testq	%rax, %rax
	je	.LBB114_328
# %bb.325:                              # %bb3.i1078
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_327
# %bb.326:                              # %bb5.i1079
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_328
.LBB114_7:                              # %bb0.i213
	cmpq	$8194, %r12                     # imm = 0x2002
	jae	.LBB114_2
.LBB114_9:                              # %bb4
	testq	%rbx, %rbx
	jne	.LBB114_10
# %bb.11:                               # %bb1.i296
	movq	%r13, %rdi
	callq	cinch_nat_to_i64@PLT
	jmp	.LBB114_12
.LBB114_10:                             # %bb0.i293
	movq	%r13, %rax
	shrq	%rax
.LBB114_12:                             # %cinch.runtime.natToI64.exit
	movq	%rax, -48(%rbp)                 # 8-byte Spill
	movl	$1, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	leaq	Main.measure.dec0+8(%rip), %rsi
	movq	%r14, %rdi
	callq	"Data.String.Eq_String.=="@PLT
	testq	%rax, %rax
	je	.LBB114_273
# %bb.13:                               # %bb6
	callq	cinch_clock_monotonic_ns@PLT
	testq	%rax, %rax
	movq	%rbx, -64(%rbp)                 # 8-byte Spill
	js	.LBB114_15
# %bb.14:                               # %bb0.i280
	leaq	1(,%rax,2), %rax
	jmp	.LBB114_16
.LBB114_327:                            # %bb4.i1081
	callq	cinch_drop_mt@PLT
.LBB114_328:                            # %cinch.rc.drop.obj.exit1084
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_335
# %bb.329:                              # %bb0.i1089
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_331
# %bb.330:                              # %bb2.i1099
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_335
.LBB114_331:                            # %bb3.i1095
	testq	%rax, %rax
	je	.LBB114_335
# %bb.332:                              # %bb3.i1095
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_334
# %bb.333:                              # %bb5.i1096
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_335
.LBB114_273:                            # %bb7
	leaq	Main.measure.dec5+8(%rip), %rsi
	movq	%r14, %rdi
	callq	"Data.String.Eq_String.=="@PLT
	testq	%rax, %rax
	je	.LBB114_286
# %bb.274:                              # %bb40
	callq	cinch_clock_monotonic_ns@PLT
	movq	%r13, %r15
	testq	%rax, %rax
	js	.LBB114_276
# %bb.275:                              # %bb0.i234
	leaq	1(,%rax,2), %rax
	jmp	.LBB114_277
.LBB114_334:                            # %bb4.i1098
	callq	cinch_drop_mt@PLT
.LBB114_335:                            # %cinch.rc.drop.tobj.exit1101
	testq	%r13, %r13
	sete	%al
	orb	%al, %bl
	movl	$5, %eax
	jne	.LBB114_272
# %bb.336:                              # %bb0.i1106
	movq	-8(%r13), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB114_338
# %bb.337:                              # %bb2.i1116
	decq	%rdx
	movq	%rdx, -8(%r13)
	jmp	.LBB114_272
.LBB114_338:                            # %bb3.i1112
	testq	%rcx, %rcx
	je	.LBB114_272
# %bb.339:                              # %bb3.i1112
	movq	%r13, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB114_341
# %bb.340:                              # %bb5.i1113
	callq	cinch_drop_slow@PLT
	movl	$5, %eax
	jmp	.LBB114_272
.LBB114_341:                            # %bb4.i1115
	callq	cinch_drop_mt@PLT
	movl	$5, %eax
	jmp	.LBB114_272
.LBB114_15:                             # %bb1.i277
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
.LBB114_16:                             # %cinch.runtime.natFromI64.exit283
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movabsq	$-3750763034362895579, %rbx     # imm = 0xCBF29CE484222325
	cmpl	$0, -48(%rbp)                   # 4-byte Folded Reload
	movq	%r13, -80(%rbp)                 # 8-byte Spill
	je	.LBB114_62
# %bb.17:                               # %bb9
	movq	%r12, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %r15
	testq	%rax, %rax
	je	.LBB114_20
# %bb.18:                               # %bb0.i310
	movq	-8(%r15), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB114_20
# %bb.19:                               # %bb1.i324
	movq	(%r15), %rax
	testb	$1, %r12b
	je	.LBB114_23
.LBB114_22:                             # %bb0.i202
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r12, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
	jmp	.LBB114_24
.LBB114_20:                             # %bb2.i318
	movq	%r15, %rdi
	callq	cinch_bytes_len@PLT
	testb	$1, %r12b
	jne	.LBB114_22
.LBB114_23:                             # %bb1.i207
	leaq	1(,%rax,2), %rdi
	movq	%r12, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
.LBB114_24:                             # %cinch.runtime.natCmp.exit209
	sete	%al
	movzbl	%al, %eax
	movq	%rax, -88(%rbp)                 # 8-byte Spill
	movq	%r15, -96(%rbp)                 # 8-byte Spill
	movq	%r15, %rdi
	callq	Main.measure.shared0@PLT
	movq	%rax, %r15
	xorq	%rbx, %r15
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	movq	-48(%rbp), %rdx                 # 8-byte Reload
	je	.LBB114_32
# %bb.25:                               # %bb0.i363
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_27
# %bb.26:                               # %bb2.i371
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB114_32
.LBB114_62:                             # %bb10
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_69
# %bb.63:                               # %bb0.i464
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_65
# %bb.64:                               # %bb2.i474
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_69
.LBB114_286:                            # %bb41
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_293
# %bb.287:                              # %bb0.i1013
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_289
# %bb.288:                              # %bb2.i1023
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_293
.LBB114_27:                             # %bb3.i369
	testq	%rax, %rax
	je	.LBB114_32
# %bb.28:                               # %bb3.i369
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_30
# %bb.29:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_31
.LBB114_276:                            # %bb1.i231
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
.LBB114_277:                            # %cinch.runtime.natFromI64.exit
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movabsq	$-3750763034362895579, %r8      # imm = 0xCBF29CE484222325
	movq	%r12, %rdi
	movq	-48(%rbp), %rsi                 # 8-byte Reload
                                        # kill: def $esi killed $esi killed $rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	-56(%rbp), %r9                  # 8-byte Reload
	callq	"Main.allocation-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r12
	movq	%rcx, %r13
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r12, 8(%rax)
	movq	%r13, 16(%rax)
	testq	%r14, %r14
	je	.LBB114_285
# %bb.278:                              # %bb0.i996
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB114_280
# %bb.279:                              # %bb2.i1006
	decq	%rdx
	movq	%rdx, -8(%r14)
	jmp	.LBB114_285
.LBB114_280:                            # %bb3.i1002
	testq	%rcx, %rcx
	je	.LBB114_285
# %bb.281:                              # %bb3.i1002
	movq	%rax, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB114_283
# %bb.282:                              # %bb5.i1003
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_284
.LBB114_65:                             # %bb3.i470
	testq	%rax, %rax
	je	.LBB114_69
# %bb.66:                               # %bb3.i470
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_68
# %bb.67:                               # %bb5.i471
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_69
.LBB114_30:                             # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB114_31:                             # %cinch.rc.drop.obj.exit
	movq	-48(%rbp), %rdx                 # 8-byte Reload
.LBB114_32:                             # %cinch.rc.drop.obj.exit
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r15
	cmpl	$1, %edx
	jne	.LBB114_33
# %bb.54:                               # %bb19
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_61
# %bb.55:                               # %bb0.i443
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_57
# %bb.56:                               # %bb2.i453
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_61
.LBB114_33:                             # %bb18
	movq	%r12, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_random_bytes@PLT
	movq	%rax, %r13
	testq	%rax, %rax
	je	.LBB114_36
# %bb.34:                               # %bb0.i299
	movq	-8(%r13), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rax, %rcx
	movabsq	$274877906944, %rax             # imm = 0x4000000000
	cmpq	%rax, %rcx
	jne	.LBB114_36
# %bb.35:                               # %bb1.i307
	movq	(%r13), %rax
	jmp	.LBB114_37
.LBB114_36:                             # %bb2.i
	movq	%r13, %rdi
	callq	cinch_bytes_len@PLT
.LBB114_37:                             # %cinch.runtime.bytesLen.exit
	testb	$1, %r12b
	jne	.LBB114_38
# %bb.39:                               # %bb1.i192
	leaq	1(,%rax,2), %rdi
	movq	%r12, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jmp	.LBB114_40
.LBB114_38:                             # %bb0.i187
	movabsq	$9223372036854775807, %rcx      # imm = 0x7FFFFFFFFFFFFFFF
	andq	%rcx, %rax
	movq	%r12, %rcx
	shrq	%rcx
	cmpq	%rcx, %rax
.LBB114_40:                             # %cinch.runtime.natCmp.exit194
	sete	%al
	movzbl	%al, %ebx
	addq	-88(%rbp), %rbx                 # 8-byte Folded Reload
	movq	%r13, %rdi
	callq	Main.fingerprint@PLT
	xorq	%rax, %r15
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rax, %r15
	movq	%r13, (%rsp)
	movq	%r12, %rsi
	movq	-48(%rbp), %rdx                 # 8-byte Reload
                                        # kill: def $edx killed $edx killed $rdx
	movl	$2, %ecx
	movq	%rbx, %r8
	movq	%r15, %r9
	callq	"Main.random-loop#cc"@PLT
	movq	%rax, %rbx
	movq	%rdx, %r15
	movq	%rcx, -48(%rbp)                 # 8-byte Spill
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%rbx, (%rax)
	movq	%r15, 8(%rax)
	movq	-48(%rbp), %rax                 # 8-byte Reload
	movq	%rax, 16(%r13)
	movq	-96(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB114_47
# %bb.41:                               # %bb0.i410
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_43
# %bb.42:                               # %bb2.i420
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB114_47
.LBB114_43:                             # %bb3.i416
	testq	%rax, %rax
	je	.LBB114_47
# %bb.44:                               # %bb3.i416
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_46
# %bb.45:                               # %bb5.i417
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_47
.LBB114_57:                             # %bb3.i449
	testq	%rax, %rax
	je	.LBB114_61
# %bb.58:                               # %bb3.i449
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_60
# %bb.59:                               # %bb5.i450
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_61
.LBB114_289:                            # %bb3.i1019
	testq	%rax, %rax
	je	.LBB114_293
# %bb.290:                              # %bb3.i1019
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_292
# %bb.291:                              # %bb5.i1020
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_293
.LBB114_283:                            # %bb4.i1005
	callq	cinch_drop_mt@PLT
.LBB114_284:                            # %cinch.rc.drop.obj.exit1008
	movq	%rbx, %rax
.LBB114_285:                            # %cinch.rc.drop.obj.exit1008
	movq	-64(%rbp), %rsi                 # 8-byte Reload
	movq	%r15, %rdx
	jmp	.LBB114_314
.LBB114_46:                             # %bb4.i419
	callq	cinch_drop_mt@PLT
.LBB114_47:                             # %cinch.rc.drop.obj.exit422
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_71
# %bb.48:                               # %bb0.i427
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_50
# %bb.49:                               # %bb2.i437
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_71
.LBB114_50:                             # %bb3.i433
	testq	%rax, %rax
	je	.LBB114_71
# %bb.51:                               # %bb3.i433
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_53
# %bb.52:                               # %bb5.i434
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_71
.LBB114_68:                             # %bb4.i473
	callq	cinch_drop_mt@PLT
.LBB114_69:                             # %cinch.rc.drop.tobj.exit476
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	$0, (%rax)
	movq	%rbx, 8(%rax)
	movq	-56(%rbp), %rax                 # 8-byte Reload
	jmp	.LBB114_70
.LBB114_60:                             # %bb4.i452
	callq	cinch_drop_mt@PLT
.LBB114_61:                             # %cinch.rc.drop.tobj.exit455
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	-88(%rbp), %rax                 # 8-byte Reload
	movq	%rax, (%r13)
	movq	%r15, 8(%r13)
	movq	-96(%rbp), %rax                 # 8-byte Reload
.LBB114_70:                             # %bb8
	movq	%rax, 16(%r13)
.LBB114_71:                             # %bb8
	callq	cinch_clock_monotonic_ns@PLT
	testq	%rax, %rax
	js	.LBB114_73
# %bb.72:                               # %bb0.i272
	leaq	1(,%rax,2), %r15
	testq	%r14, %r14
	jne	.LBB114_75
	jmp	.LBB114_81
.LBB114_73:                             # %bb1.i269
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r15
	testq	%r14, %r14
	je	.LBB114_81
.LBB114_75:                             # %bb0.i484
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_77
# %bb.76:                               # %bb2.i494
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_81
.LBB114_77:                             # %bb3.i490
	testq	%rax, %rax
	je	.LBB114_81
# %bb.78:                               # %bb3.i490
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_80
# %bb.79:                               # %bb5.i491
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_81
.LBB114_80:                             # %bb4.i493
	callq	cinch_drop_mt@PLT
.LBB114_81:                             # %cinch.rc.drop.obj.exit496
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB114_83
# %bb.82:                               # %bb28
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB114_83:                             # %bb30
	testq	%rax, %rax
	je	.LBB114_85
# %bb.84:
	xorl	%eax, %eax
	jmp	.LBB114_272
.LBB114_85:                             # %cinch.rc.unique.exit
	movq	(%r13), %rax
	movq	%rax, -48(%rbp)                 # 8-byte Spill
	movq	8(%r13), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movq	16(%r13), %r14
	movq	-8(%r13), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB114_87
# %bb.86:                               # %bb32
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB114_98
.LBB114_87:                             # %bb33
	testq	%r14, %r14
	je	.LBB114_92
# %bb.88:                               # %bb0.i500
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB114_90
# %bb.89:                               # %bb2.i509
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB114_92
.LBB114_90:                             # %bb3.i506
	je	.LBB114_92
# %bb.91:                               # %bb4.i507
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB114_92:                             # %bb0.i512
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_94
# %bb.93:                               # %bb2.i522
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB114_98
.LBB114_94:                             # %bb3.i518
	testq	%rax, %rax
	je	.LBB114_98
# %bb.95:                               # %bb3.i518
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_97
# %bb.96:                               # %bb5.i519
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_98
.LBB114_97:                             # %bb4.i521
	callq	cinch_drop_mt@PLT
.LBB114_98:                             # %bb34
	movq	%r14, %rdi
	callq	Main.measure.shared0@PLT
	movq	%rax, %r13
	testq	%r14, %r14
	je	.LBB114_105
# %bb.99:                               # %bb0.i526
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_101
# %bb.100:                              # %bb2.i536
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_105
.LBB114_101:                            # %bb3.i532
	testq	%rax, %rax
	je	.LBB114_105
# %bb.102:                              # %bb3.i532
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_104
# %bb.103:                              # %bb5.i533
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_105
.LBB114_104:                            # %bb4.i535
	callq	cinch_drop_mt@PLT
.LBB114_105:                            # %cinch.rc.drop.obj.exit538
	movq	-72(%rbp), %r14                 # 8-byte Reload
	movl	%r14d, %ebx
	andl	$1, %ebx
	movl	%r15d, %eax
	andl	%r14d, %eax
	testb	$1, %al
	je	.LBB114_106
# %bb.108:                              # %cinch.runtime.natCmp.exit179.thread
	movq	%r14, %rcx
	shrq	%rcx
	movq	%r15, %rax
	shrq	%rax
	subq	%rcx, %rax
	jb	.LBB114_123
# %bb.109:                              # %bb3.i152
	leaq	1(,%rax,2), %r12
	jmp	.LBB114_110
.LBB114_106:                            # %cinch.runtime.natCmp.exit179
	movq	%r14, %rdi
	movq	%r15, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB114_107
.LBB114_123:                            # %bb37
	testq	%r14, %r14
	sete	%al
	orb	%al, %bl
	jne	.LBB114_130
# %bb.124:                              # %bb0.i579
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_126
# %bb.125:                              # %bb2.i589
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_130
.LBB114_107:                            # %bb1.i157
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	cinch_nat_sub@PLT
	movq	%rax, %r12
.LBB114_110:                            # %cinch.runtime.natSub.exit
	testq	%r14, %r14
	sete	%al
	orb	%al, %bl
	jne	.LBB114_117
# %bb.111:                              # %bb0.i545
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_113
# %bb.112:                              # %bb2.i555
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_117
.LBB114_126:                            # %bb3.i585
	testq	%rax, %rax
	je	.LBB114_130
# %bb.127:                              # %bb3.i585
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_129
# %bb.128:                              # %bb5.i586
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_130
.LBB114_113:                            # %bb3.i551
	testq	%rax, %rax
	je	.LBB114_117
# %bb.114:                              # %bb3.i551
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_116
# %bb.115:                              # %bb5.i552
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_117
.LBB114_53:                             # %bb4.i436
	callq	cinch_drop_mt@PLT
	jmp	.LBB114_71
.LBB114_292:                            # %bb4.i1022
	callq	cinch_drop_mt@PLT
.LBB114_293:                            # %cinch.rc.drop.tobj.exit1025
	leaq	Main.measure.dec6+8(%rip), %rsi
	movq	%r14, %rdi
	callq	"Data.String.Eq_String.=="@PLT
	testq	%rax, %rax
	je	.LBB114_316
# %bb.294:                              # %bb42
	callq	cinch_clock_monotonic_ns@PLT
	testq	%rax, %rax
	js	.LBB114_296
# %bb.295:                              # %bb0.i.i1032
	leaq	1(,%rax,2), %r15
	jmp	.LBB114_297
.LBB114_316:                            # %bb43
	leaq	Main.measure.dec7+8(%rip), %rsi
	movq	%r14, %rdi
	callq	"Data.String.Eq_String.=="@PLT
	testq	%rax, %rax
	je	.LBB114_319
# %bb.317:                              # %bb44
	callq	"Effect.Clock.clock-monotonic-ns"@PLT
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movabsq	$-3750763034362895579, %r8      # imm = 0xCBF29CE484222325
	movq	-48(%rbp), %rsi                 # 8-byte Reload
                                        # kill: def $esi killed $esi killed $rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	-56(%rbp), %r9                  # 8-byte Reload
	callq	"Main.clock-loop#cc"@PLT
	jmp	.LBB114_318
.LBB114_296:                            # %bb1.i.i
	movq	%rax, %rdi
	callq	cinch_nat_from_i64@PLT
	movq	%rax, %r15
.LBB114_297:                            # %Effect.Clock.clock-monotonic-ns.exit
	movabsq	$-3750763034362895579, %rbx     # imm = 0xCBF29CE484222325
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	testl	%edi, %edi
	je	.LBB114_298
# %bb.299:                              # %cinch.runtime.natFromI64.exit68.i.preheader
	movl	$2, %r12d
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	xorl	%ecx, %ecx
.LBB114_300:                            # %cinch.runtime.natFromI64.exit68.i
                                        # =>This Inner Loop Header: Depth=1
	leaq	1(%rcx), %rsi
	xorq	%rcx, %rbx
	imulq	%rax, %rbx
	cmpl	%esi, %edi
	jbe	.LBB114_342
# %bb.301:                              # %cinch.runtime.natFromI64.exit60.i
                                        #   in Loop: Header=BB114_300 Depth=1
	leaq	2(%rcx), %rdx
	movl	%esi, %esi
	xorq	%rsi, %rbx
	imulq	%rax, %rbx
	cmpl	%edx, %edi
	jbe	.LBB114_304
# %bb.302:                              # %cinch.runtime.natFromI64.exit.i
                                        #   in Loop: Header=BB114_300 Depth=1
	addq	$3, %rcx
	movl	%ecx, %esi
	movl	%edx, %edx
	xorq	%rdx, %rbx
	imulq	%rax, %rbx
	addq	$3, %r12
	cmpl	%ecx, %edi
	movq	%rsi, %rcx
	ja	.LBB114_300
# %bb.303:                              # %"Main.counter-loop#cc.exit.loopexit"
	addq	$-2, %r12
	jmp	.LBB114_304
.LBB114_298:
	xorl	%r12d, %r12d
	jmp	.LBB114_304
.LBB114_319:                            # %bb45
	leaq	Main.measure.dec8+8(%rip), %rbx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	"Data.String.Eq_String.=="@PLT
	movq	%rax, %r12
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	testq	%r12, %r12
	je	.LBB114_321
# %bb.320:                              # %bb46
	callq	"Effect.Clock.clock-monotonic-ns"@PLT
	movq	%rax, -64(%rbp)                 # 8-byte Spill
	movabsq	$-3750763034362895579, %r8      # imm = 0xCBF29CE484222325
	movq	-48(%rbp), %rsi                 # 8-byte Reload
                                        # kill: def $esi killed $esi killed $rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movq	-56(%rbp), %r9                  # 8-byte Reload
	callq	"Main.v4-loop#cc"@PLT
.LBB114_318:                            # %bb44
	movq	%rax, %rbx
	movq	%rdx, %r12
	movq	%r13, -80(%rbp)                 # 8-byte Spill
	movq	%rcx, %r13
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
	movq	%r12, 8(%rax)
	movq	%r13, 16(%rax)
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movq	-64(%rbp), %rsi                 # 8-byte Reload
	movq	-80(%rbp), %rdx                 # 8-byte Reload
	movq	%r15, %rcx
	jmp	.LBB114_315
.LBB114_342:                            # %common.ret.split.loop.exit253.i
	decq	%r12
.LBB114_304:                            # %"Main.counter-loop#cc.exit"
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%r12, (%rax)
	movq	%rbx, 8(%rax)
	movq	-56(%rbp), %rcx                 # 8-byte Reload
	movq	%rcx, 16(%rax)
	testq	%r14, %r14
	je	.LBB114_312
# %bb.305:                              # %bb0.i1041
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB114_307
# %bb.306:                              # %bb2.i1051
	decq	%rdx
	movq	%rdx, -8(%r14)
	jmp	.LBB114_312
.LBB114_307:                            # %bb3.i1047
	testq	%rcx, %rcx
	je	.LBB114_312
# %bb.308:                              # %bb3.i1047
	movq	%rax, %rbx
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB114_310
# %bb.309:                              # %bb5.i1048
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_311
.LBB114_129:                            # %bb4.i588
	callq	cinch_drop_mt@PLT
.LBB114_130:                            # %cinch.rc.drop.tobj.exit591
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	movl	$1, %r12d
	testb	$1, %cl
	jne	.LBB114_143
# %bb.131:                              # %bb0.i596
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_133
# %bb.132:                              # %bb2.i606
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_143
.LBB114_133:                            # %bb3.i602
	testq	%rax, %rax
	je	.LBB114_143
# %bb.134:                              # %bb3.i602
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_136
# %bb.135:                              # %bb5.i603
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_143
.LBB114_136:                            # %bb4.i605
	callq	cinch_drop_mt@PLT
	jmp	.LBB114_143
.LBB114_310:                            # %bb4.i1050
	callq	cinch_drop_mt@PLT
.LBB114_311:                            # %cinch.rc.drop.obj.exit1053
	movq	%rbx, %rax
.LBB114_312:                            # %cinch.rc.drop.obj.exit1053
	movq	%r15, %rsi
.LBB114_313:                            # %cinch.rc.drop.obj.exit1053
	movq	%r13, %rdx
.LBB114_314:                            # %cinch.rc.drop.obj.exit1053
	movq	%rax, %rcx
.LBB114_315:                            # %cinch.rc.drop.obj.exit1053
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	Main.finish@PLT                 # TAILCALL
.LBB114_116:                            # %bb4.i554
	.cfi_def_cfa %rbp, 16
	callq	cinch_drop_mt@PLT
.LBB114_117:                            # %cinch.rc.drop.tobj.exit557
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_138
# %bb.118:                              # %bb0.i562
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_120
# %bb.119:                              # %bb2.i572
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_138
.LBB114_120:                            # %bb3.i568
	testq	%rax, %rax
	je	.LBB114_138
# %bb.121:                              # %bb3.i568
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_137
# %bb.122:                              # %bb5.i569
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_138
.LBB114_137:                            # %bb4.i571
	callq	cinch_drop_mt@PLT
.LBB114_138:                            # %bb35
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_143
# %bb.139:                              # %bb0.i613
	movq	-8(%r12), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB114_141
# %bb.140:                              # %bb2.i622
	incq	%rax
	movq	%rax, -8(%r12)
	jmp	.LBB114_143
.LBB114_141:                            # %bb3.i619
	je	.LBB114_143
# %bb.142:                              # %bb4.i620
	movq	%r12, %rdi
	callq	cinch_dup_mt@PLT
.LBB114_143:                            # %cinch.rc.dup.tobj.exit
	movq	%r12, %rdi
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r14
	testq	%rax, %rax
	sete	%al
	movl	%r14d, %ebx
	orb	%al, %bl
	testb	$1, %bl
	jne	.LBB114_148
# %bb.144:                              # %bb0.i628
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB114_146
# %bb.145:                              # %bb2.i638
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB114_148
.LBB114_146:                            # %bb3.i634
	je	.LBB114_148
# %bb.147:                              # %bb4.i636
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB114_148:                            # %cinch.rc.dup.tobj.exit640
	movq	%r14, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testb	$1, %bl
	jne	.LBB114_155
# %bb.149:                              # %bb0.i.i
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_151
# %bb.150:                              # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_155
.LBB114_151:                            # %bb3.i.i
	testq	%rax, %rax
	je	.LBB114_155
# %bb.152:                              # %bb3.i.i
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_154
# %bb.153:                              # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_155
.LBB114_154:                            # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB114_155:                            # %cinch.runtime.natFromI64.exit259
	movl	$1, %esi
	movq	%r15, %rdi
	movq	%r12, %rdx
	movq	%r14, %rcx
	callq	"Text.Show.nat-digits-into"@PLT
	movq	%r15, %rdi
	callq	"Text.Show.wrap-bytes"@PLT
	movq	%rax, -72(%rbp)                 # 8-byte Spill
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	callq	Main.measure.shared1@PLT
	movq	%rax, %rbx
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	Main.measure.shared1@PLT
	movq	%rax, %r14
	leaq	(,%r13,2), %r15
	movabsq	$8589934590, %rax               # imm = 0x1FFFFFFFE
	andq	%rax, %r15
	shrq	$31, %r13
	orq	$1, %r13
	leaq	3(%rax), %rsi
	movq	%r13, %rdi
	movq	%rsi, -56(%rbp)                 # 8-byte Spill
	callq	cinch_nat_mul@PLT
	movq	%rax, %r12
	testb	$1, %r12b
	je	.LBB114_158
# %bb.156:                              # %bb0.i133
	movq	%r15, %rcx
	shrq	%rcx
	movq	%r12, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB114_158
# %bb.157:                              # %cinch.rc.drop.tobj.exit681.thread
	leaq	1(,%rax,2), %rdi
	movq	%rbx, %r15
	jmp	.LBB114_167
.LBB114_158:                            # %cinch.rc.drop.tobj.exit681
	incq	%r15
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rdi
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB114_160
# %bb.159:
	movq	%rbx, %r15
	jmp	.LBB114_167
.LBB114_160:                            # %bb0.i686
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	movq	%rbx, %r15
	jl	.LBB114_162
# %bb.161:                              # %bb2.i696
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_167
.LBB114_162:                            # %bb3.i692
	testq	%rax, %rax
	je	.LBB114_167
# %bb.163:                              # %bb3.i692
	movq	%rdi, %rbx
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_165
# %bb.164:                              # %bb5.i693
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_166
.LBB114_165:                            # %bb4.i695
	callq	cinch_drop_mt@PLT
.LBB114_166:                            # %cinch.rc.drop.tobj.exit698
	movq	%rbx, %rdi
.LBB114_167:                            # %cinch.rc.drop.tobj.exit698
	callq	"Text.Show.nat-to-string"@PLT
	movq	%rax, %r12
	leaq	Main.measure.dec4+8(%rip), %rsi
	movq	%rax, %rdi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%rbx, (%rax)
	testq	%r12, %r12
	je	.LBB114_174
# %bb.168:                              # %bb0.i702
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_170
# %bb.169:                              # %bb2.i712
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_174
.LBB114_170:                            # %bb3.i708
	testq	%rax, %rax
	je	.LBB114_174
# %bb.171:                              # %bb3.i708
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_173
# %bb.172:                              # %bb5.i709
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_174
.LBB114_173:                            # %bb4.i711
	callq	cinch_drop_mt@PLT
.LBB114_174:                            # %bb0.i724
	leaq	Main.measure.dec3+8(%rip), %rdi
	movq	%r13, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r12
	movq	%rbx, (%rax)
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_176
# %bb.175:                              # %bb2.i734
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB114_180
.LBB114_176:                            # %bb3.i730
	testq	%rax, %rax
	je	.LBB114_180
# %bb.177:                              # %bb3.i730
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_179
# %bb.178:                              # %bb5.i731
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_180
.LBB114_179:                            # %bb4.i733
	callq	cinch_drop_mt@PLT
.LBB114_180:                            # %cinch.rc.drop.obj.exit736
	movq	%r14, %rdi
	movq	%r12, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r13
	movq	%rbx, (%rax)
	testq	%r14, %r14
	je	.LBB114_187
# %bb.181:                              # %bb0.i740
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_183
# %bb.182:                              # %bb2.i750
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_187
.LBB114_183:                            # %bb3.i746
	testq	%rax, %rax
	je	.LBB114_187
# %bb.184:                              # %bb3.i746
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_186
# %bb.185:                              # %bb5.i747
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_187
.LBB114_186:                            # %bb4.i749
	callq	cinch_drop_mt@PLT
.LBB114_187:                            # %bb0.i754
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_189
# %bb.188:                              # %bb2.i764
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_193
.LBB114_189:                            # %bb3.i760
	testq	%rax, %rax
	je	.LBB114_193
# %bb.190:                              # %bb3.i760
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_192
# %bb.191:                              # %bb5.i761
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_193
.LBB114_192:                            # %bb4.i763
	callq	cinch_drop_mt@PLT
.LBB114_193:                            # %bb0.i773
	leaq	Main.measure.dec2+8(%rip), %rdi
	movq	%r13, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r12
	movq	%rbx, (%rax)
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_195
# %bb.194:                              # %bb2.i783
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB114_199
.LBB114_195:                            # %bb3.i779
	testq	%rax, %rax
	je	.LBB114_199
# %bb.196:                              # %bb3.i779
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_198
# %bb.197:                              # %bb5.i780
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_199
.LBB114_198:                            # %bb4.i782
	callq	cinch_drop_mt@PLT
.LBB114_199:                            # %cinch.rc.drop.obj.exit785
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	testq	%r15, %r15
	movq	-80(%rbp), %r13                 # 8-byte Reload
	je	.LBB114_206
# %bb.200:                              # %bb0.i789
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_202
# %bb.201:                              # %bb2.i799
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_206
.LBB114_202:                            # %bb3.i795
	testq	%rax, %rax
	je	.LBB114_206
# %bb.203:                              # %bb3.i795
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_205
# %bb.204:                              # %bb5.i796
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_206
.LBB114_205:                            # %bb4.i798
	callq	cinch_drop_mt@PLT
.LBB114_206:                            # %bb0.i803
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_208
# %bb.207:                              # %bb2.i813
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_212
.LBB114_208:                            # %bb3.i809
	testq	%rax, %rax
	je	.LBB114_212
# %bb.209:                              # %bb3.i809
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_211
# %bb.210:                              # %bb5.i810
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_212
.LBB114_211:                            # %bb4.i812
	callq	cinch_drop_mt@PLT
.LBB114_212:                            # %bb0.i822
	leaq	Main.measure.dec1+8(%rip), %rdi
	movq	%r14, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r15
	movq	%rbx, (%rax)
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_214
# %bb.213:                              # %bb2.i832
	decq	%rcx
	movq	%rcx, -8(%r14)
	movq	-72(%rbp), %r12                 # 8-byte Reload
	jmp	.LBB114_218
.LBB114_214:                            # %bb3.i828
	testq	%rax, %rax
	movq	-72(%rbp), %r12                 # 8-byte Reload
	je	.LBB114_218
# %bb.215:                              # %bb3.i828
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_217
# %bb.216:                              # %bb5.i829
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_218
.LBB114_217:                            # %bb4.i831
	callq	cinch_drop_mt@PLT
.LBB114_218:                            # %cinch.rc.drop.obj.exit834
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	"Data.String.string-concat#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	testq	%r12, %r12
	movq	-64(%rbp), %rbx                 # 8-byte Reload
	je	.LBB114_225
# %bb.219:                              # %bb0.i838
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_221
# %bb.220:                              # %bb2.i848
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB114_225
.LBB114_221:                            # %bb3.i844
	testq	%rax, %rax
	je	.LBB114_225
# %bb.222:                              # %bb3.i844
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_224
# %bb.223:                              # %bb5.i845
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_225
.LBB114_224:                            # %bb4.i847
	callq	cinch_drop_mt@PLT
.LBB114_225:                            # %bb0.i852
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_227
# %bb.226:                              # %bb2.i862
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_231
.LBB114_227:                            # %bb3.i858
	testq	%rax, %rax
	je	.LBB114_231
# %bb.228:                              # %bb3.i858
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_230
# %bb.229:                              # %bb5.i859
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_231
.LBB114_230:                            # %bb4.i861
	callq	cinch_drop_mt@PLT
.LBB114_231:                            # %bb0.i866
	movq	%r14, %rdi
	callq	"Data.String.string-to-bytes"@PLT
	movq	%rax, %r15
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_233
# %bb.232:                              # %bb2.i876
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_237
.LBB114_233:                            # %bb3.i872
	testq	%rax, %rax
	je	.LBB114_237
# %bb.234:                              # %bb3.i872
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_236
# %bb.235:                              # %bb5.i873
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_237
.LBB114_236:                            # %bb4.i875
	callq	cinch_drop_mt@PLT
.LBB114_237:                            # %cinch.rc.drop.obj.exit878
	movq	%r15, %rdi
	callq	cinch_io_write@PLT
	testq	%r15, %r15
	je	.LBB114_244
# %bb.238:                              # %bb0.i881
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_240
# %bb.239:                              # %bb2.i891
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_244
.LBB114_240:                            # %bb3.i887
	testq	%rax, %rax
	je	.LBB114_244
# %bb.241:                              # %bb3.i887
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_243
# %bb.242:                              # %bb5.i888
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_244
.LBB114_243:                            # %bb4.i890
	callq	cinch_drop_mt@PLT
.LBB114_244:                            # %cinch.runtime.natFromI64.exit243
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	leaq	(%rdi,%rdi), %r14
	movabsq	$8589934590, %rax               # imm = 0x1FFFFFFFE
	andq	%rax, %r14
	shrq	$31, %rdi
	orq	$1, %rdi
	movq	-56(%rbp), %rsi                 # 8-byte Reload
	callq	cinch_nat_mul@PLT
	movq	%rax, %r15
	testb	$1, %r15b
	je	.LBB114_247
# %bb.245:                              # %bb0.i
	movq	%r14, %rcx
	shrq	%rcx
	movq	%r15, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB114_247
# %bb.246:                              # %cinch.rc.drop.tobj.exit932.thread
	leaq	1(,%rax,2), %r14
	jmp	.LBB114_254
.LBB114_247:                            # %cinch.rc.drop.tobj.exit932
	incq	%r14
	movq	%r14, %rdi
	movq	%r15, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r14
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_254
# %bb.248:                              # %bb0.i937
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_250
# %bb.249:                              # %bb2.i947
	decq	%rcx
	movq	%rcx, -8(%r15)
	jmp	.LBB114_254
.LBB114_250:                            # %bb3.i943
	testq	%rax, %rax
	je	.LBB114_254
# %bb.251:                              # %bb3.i943
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_253
# %bb.252:                              # %bb5.i944
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_254
.LBB114_253:                            # %bb4.i946
	callq	cinch_drop_mt@PLT
.LBB114_254:                            # %cinch.rc.drop.tobj.exit949
	testl	%r14d, %ebx
	je	.LBB114_256
# %bb.255:                              # %bb0.i162
	movq	%r14, %rax
	xorq	%r13, %rax
	cmpq	$2, %rax
	setb	%bl
	jmp	.LBB114_257
.LBB114_256:                            # %bb1.i167
	movq	%r14, %rdi
	movq	%r13, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	sete	%bl
.LBB114_257:                            # %cinch.runtime.natCmp.exit
	testq	%r13, %r13
	sete	%al
	movq	-64(%rbp), %rcx                 # 8-byte Reload
	orb	%al, %cl
	jne	.LBB114_264
# %bb.258:                              # %bb0.i954
	movq	-8(%r13), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_260
# %bb.259:                              # %bb2.i964
	decq	%rcx
	movq	%rcx, -8(%r13)
	jmp	.LBB114_264
.LBB114_260:                            # %bb3.i960
	testq	%rax, %rax
	je	.LBB114_264
# %bb.261:                              # %bb3.i960
	movq	%r13, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_263
# %bb.262:                              # %bb5.i961
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_264
.LBB114_263:                            # %bb4.i963
	callq	cinch_drop_mt@PLT
.LBB114_264:                            # %cinch.rc.drop.tobj.exit966
	testq	%r14, %r14
	sete	%al
	movl	%r14d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB114_271
# %bb.265:                              # %bb0.i971
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB114_267
# %bb.266:                              # %bb2.i981
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB114_271
.LBB114_267:                            # %bb3.i977
	testq	%rax, %rax
	je	.LBB114_271
# %bb.268:                              # %bb3.i977
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB114_270
# %bb.269:                              # %bb5.i978
	callq	cinch_drop_slow@PLT
	jmp	.LBB114_271
.LBB114_270:                            # %bb4.i980
	callq	cinch_drop_mt@PLT
.LBB114_271:                            # %cinch.rc.drop.tobj.exit983
	xorb	$1, %bl
	movzbl	%bl, %eax
	leaq	1(,%rax,2), %rax
.LBB114_272:                            # %common.ret
	addq	$72, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB114_321:                            # %bb47
	.cfi_def_cfa %rbp, 16
	leaq	Main.measure.dec9+8(%rip), %rbx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	"Data.String.Eq_String.=="@PLT
	movq	%rax, %r12
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	testq	%r12, %r12
	je	.LBB114_323
# %bb.322:                              # %bb48
	callq	"Effect.Uuid.uuid-v7-generator#cc"@PLT
	movq	%rax, %rbx
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %r14
	movq	%rbx, (%rax)
	callq	"Effect.Clock.clock-monotonic-ns"@PLT
	movq	%rax, %rbx
	movq	-56(%rbp), %rax                 # 8-byte Reload
	movq	%rax, (%rsp)
	movabsq	$-3750763034362895579, %r9      # imm = 0xCBF29CE484222325
	movq	%r15, %rdi
	movq	%r14, %rsi
	movq	-48(%rbp), %rdx                 # 8-byte Reload
                                        # kill: def $edx killed $edx killed $rdx
	xorl	%ecx, %ecx
	xorl	%r8d, %r8d
	callq	"Main.v7-loop#cc"@PLT
	subq	$8, %rsp
	movq	%rax, %r14
	movq	%rdx, %r15
	movq	%rcx, %r12
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%r14, (%rax)
	movq	%r15, 8(%rax)
	movq	%r12, 16(%rax)
	movq	%rbx, %rsi
	jmp	.LBB114_313
.LBB114_323:                            # %bb49
	movq	%r13, %rdi
	callq	cinch.rc.drop.tobj
	movq	-56(%rbp), %rdi                 # 8-byte Reload
	callq	cinch.rc.drop.obj
	movl	$5, %eax
	jmp	.LBB114_272
.Lfunc_end114:
	.size	Main.measure, .Lfunc_end114-Main.measure
	.cfi_endproc
                                        # -- End function
	.globl	Main.main                       # -- Begin function Main.main
	.p2align	4
	.type	Main.main,@function
Main.main:                              # @Main.main
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	callq	"Effect.Process.process-args"@PLT
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB115_2
# %bb.1:                                # %bb2
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB115_2:                              # %bb4
	testq	%rcx, %rcx
	je	.LBB115_7
# %bb.3:                                # %bb4
	cmpq	$1, %rcx
	jne	.LBB115_84
# %bb.4:                                # %bb6
	movq	(%rax), %r14
	testq	%rax, %rax
	je	.LBB115_10
# %bb.5:                                # %cinch.rc.unique.exit
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB115_10
# %bb.6:                                # %bb7
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB115_26
.LBB115_7:                              # %bb5
	testq	%rax, %rax
	je	.LBB115_13
# %bb.8:                                # %bb0.i
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB115_14
# %bb.9:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%rax)
	movl	$5, %eax
	jmp	.LBB115_85
.LBB115_10:                             # %bb8
	testq	%r14, %r14
	je	.LBB115_18
# %bb.11:                               # %bb0.i32
	movq	-8(%r14), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB115_16
# %bb.12:                               # %bb2.i40
	incq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB115_18
.LBB115_14:                             # %bb3.i
	testq	%rcx, %rcx
	je	.LBB115_13
# %bb.15:                               # %bb3.i
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB115_94
.LBB115_41:                             # %common.ret
	callq	cinch_drop_slow@PLT
	movl	$5, %eax
	jmp	.LBB115_85
.LBB115_13:
	movl	$5, %eax
	jmp	.LBB115_85
.LBB115_16:                             # %bb3.i38
	je	.LBB115_18
# %bb.17:                               # %bb4.i39
	movq	%r14, %rdi
	movq	%rax, %rbx
	callq	cinch_dup_mt@PLT
	movq	%rbx, %rax
.LBB115_18:                             # %cinch.rc.dup.obj.exit
	testq	%rax, %rax
	je	.LBB115_26
# %bb.19:                               # %bb0.i43
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB115_21
# %bb.20:                               # %bb2.i52
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB115_26
.LBB115_21:                             # %bb3.i49
	testq	%rcx, %rcx
	je	.LBB115_26
# %bb.22:                               # %bb3.i49
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB115_25
# %bb.23:                               # %bb5.i50
	callq	cinch_drop_slow@PLT
	jmp	.LBB115_26
.LBB115_25:                             # %bb4.i51
	callq	cinch_drop_mt@PLT
.LBB115_26:                             # %bb9
	movq	-8(%r14), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB115_28
# %bb.27:                               # %bb11
	movq	-16(%r14), %rax
	movq	(%rax), %rax
.LBB115_28:                             # %bb13
	cmpq	$1, %rax
	je	.LBB115_33
# %bb.29:                               # %bb13
	testq	%rax, %rax
	jne	.LBB115_84
# %bb.30:                               # %bb14
	movl	$5, %eax
	testq	%r14, %r14
	je	.LBB115_85
# %bb.31:                               # %bb0.i56
	movq	-8(%r14), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB115_39
# %bb.32:                               # %bb2.i65
	decq	%rdx
	movq	%rdx, -8(%r14)
	jmp	.LBB115_85
.LBB115_33:                             # %bb15
	movq	(%r14), %rbx
	movq	8(%r14), %r15
	testq	%r14, %r14
	je	.LBB115_36
# %bb.34:                               # %cinch.rc.unique.exit76
	movq	-8(%r14), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB115_36
# %bb.35:                               # %bb16
	movq	%r14, %rdi
	callq	cinch_free@PLT
	jmp	.LBB115_56
.LBB115_36:                             # %bb17
	testq	%rbx, %rbx
	je	.LBB115_44
# %bb.37:                               # %bb0.i78
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB115_42
# %bb.38:                               # %bb2.i87
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB115_44
.LBB115_39:                             # %bb3.i62
	testq	%rcx, %rcx
	je	.LBB115_85
# %bb.40:                               # %bb3.i62
	movq	%r14, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	je	.LBB115_41
.LBB115_94:                             # %common.ret
	callq	cinch_drop_mt@PLT
	movl	$5, %eax
	jmp	.LBB115_85
.LBB115_42:                             # %bb3.i84
	je	.LBB115_44
# %bb.43:                               # %bb4.i86
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB115_44:                             # %cinch.rc.dup.obj.exit89
	testq	%r15, %r15
	je	.LBB115_49
# %bb.45:                               # %bb0.i91
	movq	-8(%r15), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB115_47
# %bb.46:                               # %bb2.i100
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB115_49
.LBB115_47:                             # %bb3.i97
	je	.LBB115_49
# %bb.48:                               # %bb4.i99
	movq	%r15, %rdi
	callq	cinch_dup_mt@PLT
.LBB115_49:                             # %cinch.rc.dup.obj.exit102
	testq	%r14, %r14
	je	.LBB115_56
# %bb.50:                               # %bb0.i104
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB115_52
# %bb.51:                               # %bb2.i113
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB115_56
.LBB115_52:                             # %bb3.i110
	testq	%rax, %rax
	je	.LBB115_56
# %bb.53:                               # %bb3.i110
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB115_55
# %bb.54:                               # %bb5.i111
	callq	cinch_drop_slow@PLT
	jmp	.LBB115_56
.LBB115_55:                             # %bb4.i112
	callq	cinch_drop_mt@PLT
.LBB115_56:                             # %bb18
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB115_58
# %bb.57:                               # %bb20
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB115_58:                             # %bb22
	cmpq	$1, %rax
	je	.LBB115_63
# %bb.59:                               # %bb22
	testq	%rax, %rax
	jne	.LBB115_84
# %bb.60:                               # %bb23
	testq	%rbx, %rbx
	je	.LBB115_89
# %bb.61:                               # %bb0.i117
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB115_75
# %bb.62:                               # %bb2.i126
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB115_89
.LBB115_63:                             # %bb24
	movq	(%r15), %r14
	movq	8(%r15), %r13
	testq	%r15, %r15
	je	.LBB115_66
# %bb.64:                               # %cinch.rc.unique.exit150
	movq	-8(%r15), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB115_66
# %bb.65:                               # %bb25
	movq	%r15, %rdi
	callq	cinch_free@PLT
	jmp	.LBB115_67
.LBB115_66:                             # %bb26
	movq	%r14, %rdi
	callq	cinch.rc.dup.obj
	movq	%r13, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
.LBB115_67:                             # %bb27
	movq	-8(%r13), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB115_69
# %bb.68:                               # %bb29
	movq	-16(%r13), %rax
	movq	(%rax), %rax
.LBB115_69:                             # %bb31
	cmpq	$1, %rax
	je	.LBB115_72
# %bb.70:                               # %bb31
	testq	%rax, %rax
	jne	.LBB115_84
# %bb.71:                               # %bb32
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movq	%r13, %rdi
	jmp	.LBB115_87
.LBB115_72:                             # %bb33
	movq	(%r13), %r12
	movq	8(%r13), %r15
	testq	%r13, %r13
	je	.LBB115_78
# %bb.73:                               # %cinch.rc.unique.exit159
	movq	-8(%r13), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB115_78
# %bb.74:                               # %bb34
	movq	%r13, %rdi
	callq	cinch_free@PLT
	jmp	.LBB115_79
.LBB115_75:                             # %bb3.i123
	testq	%rax, %rax
	je	.LBB115_89
# %bb.76:                               # %bb3.i123
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB115_88
# %bb.77:                               # %bb5.i124
	callq	cinch_drop_slow@PLT
	jmp	.LBB115_89
.LBB115_78:                             # %bb35
	movq	%r12, %rdi
	callq	cinch.rc.dup.obj
	movq	%r15, %rdi
	callq	cinch.rc.dup.obj
	movq	%r13, %rdi
	callq	cinch.rc.drop.obj
.LBB115_79:                             # %bb36
	movq	-8(%r15), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB115_81
# %bb.80:                               # %bb38
	movq	-16(%r15), %rax
	movq	(%rax), %rax
.LBB115_81:                             # %bb40
	cmpq	$1, %rax
	je	.LBB115_86
# %bb.82:                               # %bb40
	testq	%rax, %rax
	jne	.LBB115_84
# %bb.83:                               # %bb41
	movq	%r14, %rdi
	callq	Data.String.unpack@PLT
	movq	%rax, %r13
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movl	$1, %esi
	movq	%r13, %rdi
	callq	Main.decimal@PLT
	movq	%rax, %r14
	movq	%r13, %rdi
	callq	cinch.rc.drop.obj
	movq	%r12, %rdi
	callq	Data.String.unpack@PLT
	movq	%rax, %r13
	movq	%r12, %rdi
	callq	cinch.rc.drop.obj
	movl	$1, %esi
	movq	%r13, %rdi
	callq	Main.decimal@PLT
	movq	%rax, %r12
	movq	%r13, %rdi
	callq	cinch.rc.drop.obj
	movq	%r15, %rdi
	callq	cinch.rc.drop.obj
	xorl	%edi, %edi
	movq	%rbx, %rsi
	movq	%r14, %rdx
	movq	%r12, %rcx
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	Main.measure@PLT                # TAILCALL
.LBB115_84:
	.cfi_def_cfa %rbp, 16
	xorl	%eax, %eax
.LBB115_85:                             # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB115_86:                             # %bb42
	.cfi_def_cfa %rbp, 16
	movq	%rbx, %rdi
	callq	cinch.rc.drop.obj
	movq	%r14, %rdi
	callq	cinch.rc.drop.obj
	movq	%r12, %rdi
	callq	cinch.rc.drop.obj
	movq	%r15, %rdi
.LBB115_87:                             # %common.ret
	callq	cinch.rc.drop.obj
	movl	$5, %eax
	jmp	.LBB115_85
.LBB115_88:                             # %bb4.i125
	callq	cinch_drop_mt@PLT
.LBB115_89:                             # %cinch.rc.drop.obj.exit128
	movl	$5, %eax
	testq	%r15, %r15
	je	.LBB115_85
# %bb.90:                               # %bb0.i130
	movq	-8(%r15), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB115_92
# %bb.91:                               # %bb2.i139
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB115_85
.LBB115_92:                             # %bb3.i136
	testq	%rcx, %rcx
	je	.LBB115_85
# %bb.93:                               # %bb3.i136
	movq	%r15, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	je	.LBB115_41
	jmp	.LBB115_94
.Lfunc_end115:
	.size	Main.main, .Lfunc_end115-Main.main
	.cfi_endproc
                                        # -- End function
	.globl	Main.ByteRun                    # -- Begin function Main.ByteRun
	.p2align	4
	.type	Main.ByteRun,@function
Main.ByteRun:                           # @Main.ByteRun
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
	movq	%rdx, %rbx
	movq	%rsi, %r14
	movq	%rdi, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%r15, (%rax)
	movq	%r14, 8(%rax)
	movq	%rbx, 16(%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
.Lfunc_end116:
	.size	Main.ByteRun, .Lfunc_end116-Main.ByteRun
                                        # -- End function
	.globl	"Data.String.ascii-mask.llvm"   # -- Begin function Data.String.ascii-mask.llvm
	.p2align	4
	.type	"Data.String.ascii-mask.llvm",@function
"Data.String.ascii-mask.llvm":          # @Data.String.ascii-mask.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movabsq	$-9187201950435737472, %rax     # imm = 0x8080808080808080
	andq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end117:
	.size	"Data.String.ascii-mask.llvm", .Lfunc_end117-"Data.String.ascii-mask.llvm"
                                        # -- End function
	.globl	"Data.String.ascii-mask"        # -- Begin function Data.String.ascii-mask
	.p2align	4
	.type	"Data.String.ascii-mask",@function
"Data.String.ascii-mask":               # @Data.String.ascii-mask
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movabsq	$-9187201950435737472, %rax     # imm = 0x8080808080808080
	andq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end118:
	.size	"Data.String.ascii-mask", .Lfunc_end118-"Data.String.ascii-mask"
                                        # -- End function
	.globl	"Data.Bytes.Word.load-le64.llvm" # -- Begin function Data.Bytes.Word.load-le64.llvm
	.p2align	4
	.type	"Data.Bytes.Word.load-le64.llvm",@function
"Data.Bytes.Word.load-le64.llvm":       # @Data.Bytes.Word.load-le64.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end119:
	.size	"Data.Bytes.Word.load-le64.llvm", .Lfunc_end119-"Data.Bytes.Word.load-le64.llvm"
                                        # -- End function
	.globl	"Data.Bytes.Word.load-le64"     # -- Begin function Data.Bytes.Word.load-le64
	.p2align	4
	.type	"Data.Bytes.Word.load-le64",@function
"Data.Bytes.Word.load-le64":            # @Data.Bytes.Word.load-le64
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end120:
	.size	"Data.Bytes.Word.load-le64", .Lfunc_end120-"Data.Bytes.Word.load-le64"
                                        # -- End function
	.globl	"Data.Uuid.network-word.llvm"   # -- Begin function Data.Uuid.network-word.llvm
	.p2align	4
	.type	"Data.Uuid.network-word.llvm",@function
"Data.Uuid.network-word.llvm":          # @Data.Uuid.network-word.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	bswapq	%rax
	popq	%rbp
	retq
.Lfunc_end121:
	.size	"Data.Uuid.network-word.llvm", .Lfunc_end121-"Data.Uuid.network-word.llvm"
                                        # -- End function
	.globl	"Data.Uuid.network-word"        # -- Begin function Data.Uuid.network-word
	.p2align	4
	.type	"Data.Uuid.network-word",@function
"Data.Uuid.network-word":               # @Data.Uuid.network-word
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	bswapq	%rax
	popq	%rbp
	retq
.Lfunc_end122:
	.size	"Data.Uuid.network-word", .Lfunc_end122-"Data.Uuid.network-word"
                                        # -- End function
	.globl	Main.bump.llvm                  # -- Begin function Main.bump.llvm
	.p2align	4
	.type	Main.bump.llvm,@function
Main.bump.llvm:                         # @Main.bump.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
                                        # kill: def $edi killed $edi def $rdi
	leal	1(%rdi), %eax
	popq	%rbp
	retq
.Lfunc_end123:
	.size	Main.bump.llvm, .Lfunc_end123-Main.bump.llvm
                                        # -- End function
	.globl	Main.bump                       # -- Begin function Main.bump
	.p2align	4
	.type	Main.bump,@function
Main.bump:                              # @Main.bump
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leal	1(%rdi), %eax
	popq	%rbp
	retq
.Lfunc_end124:
	.size	Main.bump, .Lfunc_end124-Main.bump
                                        # -- End function
	.globl	Main.before.llvm                # -- Begin function Main.before.llvm
	.p2align	4
	.type	Main.before.llvm,@function
Main.before.llvm:                       # @Main.before.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	cmpl	%esi, %edi
	setb	%al
	popq	%rbp
	retq
.Lfunc_end125:
	.size	Main.before.llvm, .Lfunc_end125-Main.before.llvm
                                        # -- End function
	.globl	Main.before                     # -- Begin function Main.before
	.p2align	4
	.type	Main.before,@function
Main.before:                            # @Main.before
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	cmpl	%esi, %edi
	setb	%al
	popq	%rbp
	retq
.Lfunc_end126:
	.size	Main.before, .Lfunc_end126-Main.before
                                        # -- End function
	.globl	Main.add.llvm                   # -- Begin function Main.add.llvm
	.p2align	4
	.type	Main.add.llvm,@function
Main.add.llvm:                          # @Main.add.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end127:
	.size	Main.add.llvm, .Lfunc_end127-Main.add.llvm
                                        # -- End function
	.globl	Main.add                        # -- Begin function Main.add
	.p2align	4
	.type	Main.add,@function
Main.add:                               # @Main.add
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	leaq	(%rdi,%rsi), %rax
	popq	%rbp
	retq
.Lfunc_end128:
	.size	Main.add, .Lfunc_end128-Main.add
                                        # -- End function
	.globl	Main.mix.llvm                   # -- Begin function Main.mix.llvm
	.p2align	4
	.type	Main.mix.llvm,@function
Main.mix.llvm:                          # @Main.mix.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorq	%rsi, %rdi
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end129:
	.size	Main.mix.llvm, .Lfunc_end129-Main.mix.llvm
                                        # -- End function
	.globl	Main.mix                        # -- Begin function Main.mix
	.p2align	4
	.type	Main.mix,@function
Main.mix:                               # @Main.mix
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	xorq	%rsi, %rdi
	movabsq	$1099511628211, %rax            # imm = 0x100000001B3
	imulq	%rdi, %rax
	popq	%rbp
	retq
.Lfunc_end130:
	.size	Main.mix, .Lfunc_end130-Main.mix
                                        # -- End function
	.globl	Main.marker.llvm                # -- Begin function Main.marker.llvm
	.p2align	4
	.type	Main.marker.llvm,@function
Main.marker.llvm:                       # @Main.marker.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movzbl	%dil, %eax
	popq	%rbp
	retq
.Lfunc_end131:
	.size	Main.marker.llvm, .Lfunc_end131-Main.marker.llvm
                                        # -- End function
	.globl	Main.marker                     # -- Begin function Main.marker
	.p2align	4
	.type	Main.marker,@function
Main.marker:                            # @Main.marker
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movzbl	%dil, %eax
	popq	%rbp
	retq
.Lfunc_end132:
	.size	Main.marker, .Lfunc_end132-Main.marker
                                        # -- End function
	.globl	"Main.observed-words.llvm"      # -- Begin function Main.observed-words.llvm
	.p2align	4
	.type	"Main.observed-words.llvm",@function
"Main.observed-words.llvm":             # @Main.observed-words.llvm
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	shrq	$52, %rax
	andl	$15, %eax
	xorq	%rdx, %rax
	movl	%esi, %ecx
	andl	$192, %ecx
	xorq	$128, %rcx
	xorl	%edx, %edx
	orq	%rax, %rcx
	sete	%dl
	shlq	$63, %rdx
	movzbl	%dil, %ecx
	shrq	$56, %rdi
	addq	%rdi, %rcx
	movzbl	%sil, %eax
	shrq	$56, %rsi
	addq	%rsi, %rax
	addq	%rcx, %rax
	orq	%rdx, %rax
	popq	%rbp
	retq
.Lfunc_end133:
	.size	"Main.observed-words.llvm", .Lfunc_end133-"Main.observed-words.llvm"
                                        # -- End function
	.globl	"Main.observed-words"           # -- Begin function Main.observed-words
	.p2align	4
	.type	"Main.observed-words",@function
"Main.observed-words":                  # @Main.observed-words
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rdi, %rax
	shrq	$52, %rax
	andl	$15, %eax
	xorq	%rdx, %rax
	movl	%esi, %ecx
	andl	$192, %ecx
	xorq	$128, %rcx
	xorl	%edx, %edx
	orq	%rax, %rcx
	sete	%dl
	shlq	$63, %rdx
	movzbl	%dil, %ecx
	shrq	$56, %rdi
	addq	%rdi, %rcx
	movzbl	%sil, %eax
	shrq	$56, %rsi
	addq	%rsi, %rax
	addq	%rcx, %rax
	orq	%rdx, %rax
	popq	%rbp
	retq
.Lfunc_end134:
	.size	"Main.observed-words", .Lfunc_end134-"Main.observed-words"
                                        # -- End function
	.globl	Main.main.exit                  # -- Begin function Main.main.exit
	.p2align	4
	.type	Main.main.exit,@function
Main.main.exit:                         # @Main.main.exit
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	callq	Main.main@PLT
	movq	%rax, %rbx
	movq	%rax, %rdi
	callq	cinch_int_to_i64@PLT
	testq	%rbx, %rbx
	sete	%cl
	movl	%ebx, %edx
	orb	%cl, %dl
	testb	$1, %dl
	jne	.LBB135_8
# %bb.1:                                # %bb0.i
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB135_3
# %bb.2:                                # %bb2.i
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB135_8
.LBB135_3:                              # %bb3.i
	testq	%rcx, %rcx
	je	.LBB135_8
# %bb.4:                                # %bb3.i
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB135_6
# %bb.5:                                # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB135_7
.LBB135_6:                              # %bb4.i
	callq	cinch_drop_mt@PLT
.LBB135_7:                              # %cinch.rc.drop.tobj.exit
	movq	%r14, %rax
.LBB135_8:                              # %cinch.rc.drop.tobj.exit
	popq	%rbx
	popq	%r14
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end135:
	.size	Main.main.exit, .Lfunc_end135-Main.main.exit
	.cfi_endproc
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function Main.v4-loop.shared0
.LCPI136_0:
	.quad	-61441                          # 0xffffffffffff0fff
	.quad	4611686018427387903             # 0x3fffffffffffffff
.LCPI136_1:
	.quad	16384                           # 0x4000
	.quad	-9223372036854775808            # 0x8000000000000000
	.text
	.globl	"Main.v4-loop.shared0"
	.p2align	4
	.type	"Main.v4-loop.shared0",@function
"Main.v4-loop.shared0":                 # @Main.v4-loop.shared0
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
	movq	%rdi, %rbx
	movabsq	$9223372036854775807, %r12      # imm = 0x7FFFFFFFFFFFFFFF
	testq	%rdi, %rdi
	je	.LBB136_5
# %bb.1:                                # %bb0.i58
	movabsq	$1030792151040, %r15            # imm = 0xF000000000
	movabsq	$274877906944, %r14             # imm = 0x4000000000
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB136_9
# %bb.2:                                # %cinch.runtime.natCmp.exit.thread
	andq	(%rbx), %r12
	cmpq	$16, %r12
	je	.LBB136_10
.LBB136_3:                              # %bb0.i17
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB136_7
# %bb.4:                                # %bb1.i21
	movq	(%rbx), %rax
	jmp	.LBB136_8
.LBB136_5:                              # %cinch.runtime.natCmp.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r12, %rax
	cmpq	$16, %rax
	jne	.LBB136_7
# %bb.6:                                # %cinch.runtime.bytesData.exit127.thread268
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	(%rax), %r12
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	bswapq	%r12
	xorl	%edi, %edi
	callq	cinch_bytes_data@PLT
	movq	8(%rax), %r13
	jmp	.LBB136_21
.LBB136_7:                              # %bb2.i
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB136_8:                              # %cinch.runtime.bytesLen.exit
	leaq	1(,%rax,2), %r15
	movl	$256, %r14d                     # imm = 0x100
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r15, (%rax)
	jmp	.LBB136_23
.LBB136_9:                              # %cinch.runtime.natCmp.exit.thread295
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r12, %rax
	cmpq	$16, %rax
	jne	.LBB136_3
.LBB136_10:                             # %bb0.i88
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB136_13
# %bb.11:                               # %cinch.runtime.bytesData.exit.thread
	cmpq	$0, (%rbx)
	movl	$1, %eax
	je	.LBB136_14
# %bb.12:                               # %cinch.runtime.bytesData.exit.thread
	movq	8(%rbx), %rax
	jmp	.LBB136_14
.LBB136_13:                             # %cinch.runtime.bytesData.exit
	movq	%rbx, %rdi
	callq	cinch_bytes_data@PLT
.LBB136_14:                             # %bb0.i41
	movq	(%rax), %r12
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	bswapq	%r12
	cmpq	%r14, %rax
	je	.LBB136_16
# %bb.15:                               # %cinch.runtime.bytesLen.exit56.bb0.i112_crit_edge
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB136_16:                             # %bb0.i112
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	jne	.LBB136_19
# %bb.17:                               # %cinch.runtime.bytesData.exit127.thread
	cmpq	$0, (%rbx)
	movl	$1, %eax
	je	.LBB136_20
# %bb.18:                               # %cinch.runtime.bytesData.exit127.thread
	movq	8(%rbx), %rax
	jmp	.LBB136_20
.LBB136_19:                             # %cinch.runtime.bytesData.exit127
	movq	%rbx, %rdi
	callq	cinch_bytes_data@PLT
.LBB136_20:                             # %bb0.i24
	movq	8(%rax), %r13
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r14, %rax
	je	.LBB136_22
.LBB136_21:                             # %bb2.i32
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB136_22:                             # %cinch.runtime.bytesLen.exit39
	bswapq	%r13
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movq	%rax, %rbx
	movq	%r12, (%rax)
	movq	%r13, 8(%rax)
	movl	$257, %r14d                     # imm = 0x101
.LBB136_23:                             # %bb1
	movl	$8, %edi
	movq	%r14, %rsi
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	-8(%rax), %rcx
	shrq	$40, %rcx
	movzbl	%cl, %ecx
	cmpq	$255, %rcx
	jne	.LBB136_25
# %bb.24:                               # %bb5
	movq	-16(%rax), %rcx
	movq	(%rcx), %rcx
.LBB136_25:                             # %bb7
	cmpq	$1, %rcx
	je	.LBB136_29
# %bb.26:                               # %bb7
	testq	%rcx, %rcx
	jne	.LBB136_31
# %bb.27:                               # %cinch.rc.unique.exit
	movq	-8(%rax), %rcx
	movabsq	$34359738367, %rdx              # imm = 0x7FFFFFFFF
	andq	%rcx, %rdx
	cmpq	$1, %rdx
	jne	.LBB136_32
# %bb.28:                               # %bb14
	movabsq	$281509336449025, %rcx          # imm = 0x1000800000001
	movq	%rcx, -8(%rax)
	jmp	.LBB136_68
.LBB136_29:                             # %cinch.rc.unique.exit182
	movabsq	$34359738367, %r14              # imm = 0x7FFFFFFFF
	movq	-8(%rax), %rcx
	andq	%r14, %rcx
	cmpq	$1, %rcx
	jne	.LBB136_34
# %bb.30:                               # %bb18
	movq	%rax, %rdi
	callq	cinch_free@PLT
	jmp	.LBB136_53
.LBB136_32:                             # %bb0.i154
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB136_36
# %bb.33:                               # %bb2.i161
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB136_38
.LBB136_34:                             # %bb0.i184
	movq	-8(%rbx), %rcx
	movq	%rcx, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB136_43
# %bb.35:                               # %bb2.i194
	incq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB136_45
.LBB136_36:                             # %bb3.i
	je	.LBB136_38
# %bb.37:                               # %bb4.i
	movq	%rbx, %rdi
	movq	%rax, %r14
	callq	cinch_dup_mt@PLT
	movq	%r14, %rax
.LBB136_38:                             # %bb0.i163
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB136_40
# %bb.39:                               # %bb2.i172
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB136_51
.LBB136_40:                             # %bb3.i169
	testq	%rcx, %rcx
	je	.LBB136_51
# %bb.41:                               # %bb3.i169
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB136_50
# %bb.42:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB136_51
.LBB136_43:                             # %bb3.i190
	je	.LBB136_45
# %bb.44:                               # %bb4.i192
	movq	%rbx, %rdi
	movq	%rax, %r15
	callq	cinch_dup_mt@PLT
	movq	%r15, %rax
.LBB136_45:                             # %bb0.i198
	movq	-8(%rax), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB136_47
# %bb.46:                               # %bb2.i208
	decq	%rdx
	movq	%rdx, -8(%rax)
	jmp	.LBB136_53
.LBB136_47:                             # %bb3.i204
	testq	%rcx, %rcx
	je	.LBB136_53
# %bb.48:                               # %bb3.i204
	movq	%rax, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB136_52
# %bb.49:                               # %bb5.i205
	callq	cinch_drop_slow@PLT
	jmp	.LBB136_53
.LBB136_50:                             # %bb4.i171
	callq	cinch_drop_mt@PLT
.LBB136_51:                             # %bb13
	movl	$8, %edi
	movl	$256, %esi                      # imm = 0x100
	jmp	.LBB136_67
.LBB136_52:                             # %bb4.i207
	callq	cinch_drop_mt@PLT
.LBB136_53:                             # %bb20
	movq	-8(%rbx), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB136_55
# %bb.54:                               # %bb22
	movq	-16(%rbx), %rax
	movq	(%rax), %rax
.LBB136_55:                             # %bb24
	testq	%rax, %rax
	je	.LBB136_57
.LBB136_31:
	xorl	%eax, %eax
	jmp	.LBB136_68
.LBB136_57:                             # %cinch.rc.unique.exit219
	movups	(%rbx), %xmm0
	movq	-8(%rbx), %rax
	andq	%r14, %rax
	cmpq	$1, %rax
	jne	.LBB136_59
# %bb.58:                               # %bb30
	andps	.LCPI136_0(%rip), %xmm0
	addq	$2, %r14
	movq	%r14, -8(%rbx)
	orps	.LCPI136_1(%rip), %xmm0
	movups	%xmm0, (%rbx)
	jmp	.LBB136_66
.LBB136_59:                             # %bb0.i221
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	movaps	%xmm0, -64(%rbp)                # 16-byte Spill
	jl	.LBB136_61
# %bb.60:                               # %bb2.i231
	decq	%rcx
	movq	%rcx, -8(%rbx)
	jmp	.LBB136_65
.LBB136_61:                             # %bb3.i227
	testq	%rax, %rax
	je	.LBB136_65
# %bb.62:                               # %bb3.i227
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB136_64
# %bb.63:                               # %bb5.i228
	callq	cinch_drop_slow@PLT
	jmp	.LBB136_65
.LBB136_64:                             # %bb4.i230
	callq	cinch_drop_mt@PLT
.LBB136_65:                             # %bb29
	movl	$16, %edi
	xorl	%esi, %esi
	callq	cinch_alloc_ctor@PLT
	movaps	-64(%rbp), %xmm0                # 16-byte Reload
	andps	.LCPI136_0(%rip), %xmm0
	movq	%rax, %rbx
	orps	.LCPI136_1(%rip), %xmm0
	movups	%xmm0, (%rax)
.LBB136_66:                             # %bb33
	movl	$8, %edi
	movl	$257, %esi                      # imm = 0x101
.LBB136_67:                             # %common.ret285
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
.LBB136_68:                             # %common.ret285
	addq	$24, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end136:
	.size	"Main.v4-loop.shared0", .Lfunc_end136-"Main.v4-loop.shared0"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v7-loop.shared0#cc"       # -- Begin function Main.v7-loop.shared0#cc
	.p2align	4
	.type	"Main.v7-loop.shared0#cc",@function
"Main.v7-loop.shared0#cc":              # @"Main.v7-loop.shared0#cc"
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
	movq	%r9, %r14
	movq	%rcx, %r15
	movq	%rsi, %rbx
	movq	%rdi, %r13
	movq	16(%rbp), %rcx
	cmpl	%ebx, %r13d
	jae	.LBB137_57
# %bb.1:                                # %bb1
	movq	%rdx, -72(%rbp)                 # 8-byte Spill
	testq	%r15, %r15
	movq	%r8, -64(%rbp)                  # 8-byte Spill
	je	.LBB137_6
# %bb.2:                                # %bb0.i
	movq	-8(%r15), %rax
	movq	%rax, %rdx
	shlq	$29, %rdx
	testq	%rdx, %rdx
	jle	.LBB137_4
# %bb.3:                                # %bb2.i
	incq	%rax
	movq	%rax, -8(%r15)
	jmp	.LBB137_6
.LBB137_57:                             # %bb2
	testq	%r15, %r15
	je	.LBB137_65
# %bb.58:                               # %bb0.i109
	movq	-8(%r15), %rdx
	movq	%rdx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB137_60
# %bb.59:                               # %bb2.i118
	decq	%rdx
	movq	%rdx, -8(%r15)
	jmp	.LBB137_65
.LBB137_60:                             # %bb3.i115
	testq	%rax, %rax
	je	.LBB137_65
# %bb.61:                               # %bb3.i115
	movq	%rcx, %r12
	movq	%r8, %rbx
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB137_63
# %bb.62:                               # %bb5.i116
	callq	cinch_drop_slow@PLT
	jmp	.LBB137_64
.LBB137_4:                              # %bb3.i
	je	.LBB137_6
# %bb.5:                                # %bb4.i
	movq	%r15, %rdi
	movq	%rcx, %r12
	callq	cinch_dup_mt@PLT
	movq	%r12, %rcx
.LBB137_6:                              # %cinch.rc.dup.obj.exit
	movq	%rcx, -80(%rbp)                 # 8-byte Spill
	movq	%r15, %rsi
	callq	"Effect.Uuid.uuid-new-v7"@PLT
	movq	%rax, %r12
	movq	-8(%rax), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB137_8
# %bb.7:                                # %bb4
	movq	-16(%r12), %rax
	movq	(%rax), %rax
.LBB137_8:                              # %bb6
	cmpq	$1, %rax
	je	.LBB137_20
# %bb.9:                                # %bb6
	testq	%rax, %rax
	jne	.LBB137_10
# %bb.11:                               # %bb7
	testq	%r12, %r12
	je	.LBB137_18
# %bb.12:                               # %bb0.i27
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB137_14
# %bb.13:                               # %bb2.i35
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB137_18
.LBB137_20:                             # %bb8
	movq	(%r12), %rdi
	testq	%r12, %r12
	movq	%rdi, -48(%rbp)                 # 8-byte Spill
	je	.LBB137_23
# %bb.21:                               # %cinch.rc.unique.exit
	movq	-8(%r12), %rax
	movabsq	$34359738367, %rcx              # imm = 0x7FFFFFFFF
	andq	%rax, %rcx
	cmpq	$1, %rcx
	jne	.LBB137_23
# %bb.22:                               # %bb9
	movq	%r12, %rdi
	callq	cinch_free@PLT
.LBB137_35:                             # %bb11
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	jmp	.LBB137_36
.LBB137_10:
	xorl	%r8d, %r8d
	xorl	%r14d, %r14d
	xorl	%ecx, %ecx
	jmp	.LBB137_65
.LBB137_23:                             # %bb10
	testq	%rdi, %rdi
	je	.LBB137_28
# %bb.24:                               # %bb0.i41
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB137_26
# %bb.25:                               # %bb2.i50
	incq	%rax
	movq	%rax, -8(%rdi)
	jmp	.LBB137_28
.LBB137_14:                             # %bb3.i33
	testq	%rax, %rax
	je	.LBB137_18
# %bb.15:                               # %bb3.i33
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB137_17
# %bb.16:                               # %bb5.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB137_18
.LBB137_63:                             # %bb4.i117
	callq	cinch_drop_mt@PLT
.LBB137_64:                             # %cinch.rc.drop.obj.exit120
	movq	%rbx, %r8
	movq	%r12, %rcx
.LBB137_65:                             # %common.ret
	movq	%r8, %rax
	movq	%r14, %rdx
	addq	$56, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq	$8
.LBB137_26:                             # %bb3.i47
	.cfi_def_cfa %rbp, 16
	je	.LBB137_28
# %bb.27:                               # %bb4.i49
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	callq	cinch_dup_mt@PLT
	movq	-48(%rbp), %rdi                 # 8-byte Reload
.LBB137_28:                             # %cinch.rc.dup.obj.exit52
	testq	%r12, %r12
	je	.LBB137_36
# %bb.29:                               # %bb0.i54
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB137_31
# %bb.30:                               # %bb2.i63
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB137_36
.LBB137_31:                             # %bb3.i60
	testq	%rax, %rax
	je	.LBB137_36
# %bb.32:                               # %bb3.i60
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB137_34
# %bb.33:                               # %bb5.i61
	callq	cinch_drop_slow@PLT
	jmp	.LBB137_35
.LBB137_36:                             # %bb11
	movq	-8(%rdi), %rax
	shrq	$40, %rax
	movzbl	%al, %eax
	cmpq	$255, %rax
	jne	.LBB137_38
# %bb.37:                               # %bb2.i74
	movq	-16(%rdi), %rax
	movq	(%rax), %rax
.LBB137_38:                             # %bb4.i71
	testq	%rax, %rax
	movq	-64(%rbp), %r12                 # 8-byte Reload
	je	.LBB137_40
# %bb.39:
	movq	$0, -56(%rbp)                   # 8-byte Folded Spill
	testq	%rdi, %rdi
	jne	.LBB137_42
	jmp	.LBB137_48
.LBB137_40:                             # %bb5.i73
	movq	(%rdi), %rax
	movq	%rax, -88(%rbp)                 # 8-byte Spill
	movq	8(%rdi), %rax
	movq	%rax, -56(%rbp)                 # 8-byte Spill
	movl	$33, %edi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r12
	movq	-88(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$1, %esi
	movq	%rax, %rdi
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	-56(%rbp), %rdx                 # 8-byte Reload
	bswapq	%rdx
	movl	$17, %esi
	movq	%r12, -56(%rbp)                 # 8-byte Spill
	movq	%r12, %rdi
	movq	-64(%rbp), %r12                 # 8-byte Reload
	callq	"Data.Bytes.Word.bytes-write-le64"@PLT
	movq	-48(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB137_48
.LBB137_42:                             # %bb0.i76
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB137_44
# %bb.43:                               # %bb2.i85
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB137_48
.LBB137_44:                             # %bb3.i82
	testq	%rax, %rax
	je	.LBB137_48
# %bb.45:                               # %bb3.i82
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB137_47
# %bb.46:                               # %bb5.i83
	callq	cinch_drop_slow@PLT
	jmp	.LBB137_48
.LBB137_17:                             # %bb4.i34
	callq	cinch_drop_mt@PLT
.LBB137_18:                             # %cinch.rc.drop.obj.exit
	movl	$4294967295, %eax               # imm = 0xFFFFFFFF
	xorq	%rax, %r14
	movabsq	$1099511628211, %r9             # imm = 0x100000001B3
	imulq	%r14, %r9
	incl	%r13d
	movq	-80(%rbp), %rax                 # 8-byte Reload
	movq	%rax, 16(%rbp)
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	movq	%r15, %rsi
	movl	%ebx, %edx
	movl	%r13d, %ecx
	movq	-64(%rbp), %r8                  # 8-byte Reload
	jmp	.LBB137_19
.LBB137_47:                             # %bb4.i84
	callq	cinch_drop_mt@PLT
.LBB137_48:                             # %cinch.rc.drop.obj.exit87
	movl	$7, %edi
	movq	-56(%rbp), %rsi                 # 8-byte Reload
	callq	"Main.observe-uuid"@PLT
	movq	-80(%rbp), %rdi                 # 8-byte Reload
	testq	%rdi, %rdi
	je	.LBB137_56
# %bb.49:                               # %bb0.i89
	movq	-8(%rdi), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB137_51
# %bb.50:                               # %bb2.i98
	decq	%rdx
	movq	%rdx, -8(%rdi)
	jmp	.LBB137_56
.LBB137_51:                             # %bb3.i95
	testq	%rcx, %rcx
	je	.LBB137_56
# %bb.52:                               # %bb3.i95
	movq	%rax, -48(%rbp)                 # 8-byte Spill
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB137_54
# %bb.53:                               # %bb5.i96
	callq	cinch_drop_slow@PLT
	jmp	.LBB137_55
.LBB137_54:                             # %bb4.i97
	callq	cinch_drop_mt@PLT
.LBB137_55:                             # %cinch.rc.drop.obj.exit100
	movq	-48(%rbp), %rax                 # 8-byte Reload
	movq	-64(%rbp), %r12                 # 8-byte Reload
.LBB137_56:                             # %cinch.rc.drop.obj.exit100
	movq	%rax, %r8
	shrq	$63, %r8
	addq	%r12, %r8
	andl	$1023, %eax                     # imm = 0x3FF
	xorq	%r14, %rax
	movabsq	$1099511628211, %r9             # imm = 0x100000001B3
	imulq	%rax, %r9
	incl	%r13d
	movq	-56(%rbp), %rax                 # 8-byte Reload
	movq	%rax, 16(%rbp)
	movq	-72(%rbp), %rdi                 # 8-byte Reload
	movq	%r15, %rsi
	movl	%ebx, %edx
	movl	%r13d, %ecx
.LBB137_19:                             # %cinch.rc.drop.obj.exit
	addq	$56, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Main.v7-loop#cc"@PLT           # TAILCALL
.LBB137_34:                             # %bb4.i62
	.cfi_def_cfa %rbp, 16
	callq	cinch_drop_mt@PLT
	jmp	.LBB137_35
.Lfunc_end137:
	.size	"Main.v7-loop.shared0#cc", .Lfunc_end137-"Main.v7-loop.shared0#cc"
	.cfi_endproc
                                        # -- End function
	.globl	"Main.v7-loop.shared0"          # -- Begin function Main.v7-loop.shared0
	.p2align	4
	.type	"Main.v7-loop.shared0",@function
"Main.v7-loop.shared0":                 # @Main.v7-loop.shared0
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$16, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	48(%rsp), %rax
	movq	%rax, (%rsp)
	callq	"Main.v7-loop.shared0#cc"@PLT
	subq	$8, %rsp
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	%rcx, %r15
	movl	$24, %edi
	movl	$1024, %esi                     # imm = 0x400
	callq	cinch_alloc_ctor@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%r15, 16(%rax)
	addq	$16, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end138:
	.size	"Main.v7-loop.shared0", .Lfunc_end138-"Main.v7-loop.shared0"
	.cfi_endproc
                                        # -- End function
	.globl	Main.finish.shared0             # -- Begin function Main.finish.shared0
	.p2align	4
	.type	Main.finish.shared0,@function
Main.finish.shared0:                    # @Main.finish.shared0
	.cfi_startproc
# %bb.0:                                # %cinch.runtime.natFromI64.exit
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movl	%edi, %ebx
	addq	%rbx, %rbx
	shrq	$31, %rdi
	orq	$1, %rdi
	movabsq	$8589934590, %rsi               # imm = 0x1FFFFFFFE
	addq	$3, %rsi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	je	.LBB139_3
# %bb.1:                                # %bb0.i
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%r14, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB139_3
# %bb.2:                                # %cinch.rc.drop.tobj.exit59.thread
	leaq	1(,%rax,2), %rbx
	jmp	.LBB139_10
.LBB139_3:                              # %cinch.rc.drop.tobj.exit59
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
	jne	.LBB139_10
# %bb.4:                                # %bb0.i64
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB139_6
# %bb.5:                                # %bb2.i74
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB139_10
.LBB139_6:                              # %bb3.i70
	testq	%rax, %rax
	je	.LBB139_10
# %bb.7:                                # %bb3.i70
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB139_9
# %bb.8:                                # %bb5.i71
	callq	cinch_drop_slow@PLT
	jmp	.LBB139_10
.LBB139_9:                              # %bb4.i73
	callq	cinch_drop_mt@PLT
.LBB139_10:                             # %cinch.rc.drop.tobj.exit76
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB139_15
# %bb.11:                               # %bb0.i81
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB139_13
# %bb.12:                               # %bb2.i90
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB139_15
.LBB139_13:                             # %bb3.i87
	je	.LBB139_15
# %bb.14:                               # %bb4.i88
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB139_15:                             # %cinch.rc.dup.tobj.exit
	movq	%rbx, %rdi
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r14
	testq	%rax, %rax
	sete	%al
	movl	%r14d, %r12d
	orb	%al, %r12b
	testb	$1, %r12b
	jne	.LBB139_20
# %bb.16:                               # %bb0.i96
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB139_18
# %bb.17:                               # %bb2.i106
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB139_20
.LBB139_18:                             # %bb3.i102
	je	.LBB139_20
# %bb.19:                               # %bb4.i104
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB139_20:                             # %cinch.rc.dup.tobj.exit108
	movq	%r14, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testb	$1, %r12b
	jne	.LBB139_27
# %bb.21:                               # %bb0.i.i
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB139_23
# %bb.22:                               # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB139_27
.LBB139_23:                             # %bb3.i.i
	testq	%rax, %rax
	je	.LBB139_27
# %bb.24:                               # %bb3.i.i
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB139_26
# %bb.25:                               # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB139_27
.LBB139_26:                             # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB139_27:                             # %cinch.runtime.bytesNew.exit
	movl	$1, %esi
	movq	%r15, %rdi
	movq	%rbx, %rdx
	movq	%r14, %rcx
	callq	"Text.Show.nat-digits-into"@PLT
	movq	%r15, %rdi
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Text.Show.wrap-bytes"@PLT      # TAILCALL
.Lfunc_end139:
	.size	Main.finish.shared0, .Lfunc_end139-Main.finish.shared0
	.cfi_endproc
                                        # -- End function
	.globl	Main.measure.shared0            # -- Begin function Main.measure.shared0
	.p2align	4
	.type	Main.measure.shared0,@function
Main.measure.shared0:                   # @Main.measure.shared0
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
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, %rbx
	movabsq	$9223372036854775807, %r13      # imm = 0x7FFFFFFFFFFFFFFF
	movabsq	$1030792151040, %r15            # imm = 0xF000000000
	movabsq	$274877906944, %r12             # imm = 0x4000000000
	testq	%rdi, %rdi
	je	.LBB140_3
# %bb.1:                                # %bb0.i190
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB140_5
# %bb.2:                                # %cinch.runtime.bytesLen.exit206.thread
	movq	(%rbx), %rax
	jmp	.LBB140_6
.LBB140_3:                              # %cinch.runtime.natCmp.exit134
	xorl	%edi, %edi
	callq	cinch_bytes_len@PLT
	leaq	1(,%rax,2), %r14
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testq	%r13, %rax
	jne	.LBB140_13
	jmp	.LBB140_36
.LBB140_5:                              # %cinch.runtime.bytesLen.exit206
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
.LBB140_6:                              # %bb0.i172
	leaq	1(,%rax,2), %r14
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB140_9
# %bb.7:                                # %cinch.runtime.natCmp.exit134.thread
	testq	%r13, (%rbx)
	je	.LBB140_36
.LBB140_10:                             # %bb3.i260
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	cmpq	%r12, %rax
	jne	.LBB140_13
# %bb.11:                               # %bb4.i273
	cmpq	$0, (%rbx)
	je	.LBB140_13
# %bb.12:                               # %bb7.i276
	movq	8(%rbx), %rax
	movzbl	(%rax), %eax
	jmp	.LBB140_14
.LBB140_13:                             # %bb5.i268
	movq	%rbx, %rdi
	xorl	%esi, %esi
	callq	cinch_bytes_get@PLT
.LBB140_14:                             # %cinch.runtime.bytesGet.exit283
	leaq	1(,%rax,2), %rsi
	jmp	.LBB140_15
.LBB140_9:                              # %cinch.runtime.natCmp.exit134.thread646
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	testq	%r13, %rax
	jne	.LBB140_10
.LBB140_36:
	movl	$1, %esi
.LBB140_15:                             # %bb0.i49
	movq	%r14, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB140_17
# %bb.16:                               # %bb3.i54
	leaq	1(,%rax,2), %r12
	testq	%rbx, %rbx
	jne	.LBB140_18
	jmp	.LBB140_20
.LBB140_17:                             # %bb1.i58
	movq	%r14, %rdi
	callq	cinch_nat_add@PLT
	movq	%rax, %r12
	testq	%rbx, %rbx
	je	.LBB140_20
.LBB140_18:                             # %bb0.i154
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB140_20
# %bb.19:                               # %bb0.i117.thread
	movq	(%rbx), %rax
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB140_27
	jmp	.LBB140_22
.LBB140_20:                             # %bb0.i117
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	andq	%r13, %rax
	movq	%r14, %rsi
	shrq	$2, %rsi
	movl	$1, %ecx
	cmpq	%rax, %rsi
	jae	.LBB140_27
# %bb.21:                               # %bb0.i225
	testq	%rbx, %rbx
	je	.LBB140_25
.LBB140_22:                             # %bb3.i230
	movq	-8(%rbx), %rax
	andq	%r15, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB140_25
# %bb.23:                               # %bb4.i243
	cmpq	(%rbx), %rsi
	jae	.LBB140_25
# %bb.24:                               # %bb7.i246
	movq	8(%rbx), %rax
	movzbl	(%rax,%rsi), %eax
	jmp	.LBB140_26
.LBB140_25:                             # %bb5.i238
	movq	%rbx, %rdi
	callq	cinch_bytes_get@PLT
.LBB140_26:                             # %cinch.runtime.bytesGet.exit255
	leaq	1(,%rax,2), %rcx
.LBB140_27:                             # %bb4
	testb	$1, %r12b
	je	.LBB140_30
# %bb.28:                               # %bb0.i34
	movq	%r12, %rdx
	shrq	%rdx
	movq	%rcx, %rax
	shrq	%rax
	addq	%rdx, %rax
	js	.LBB140_30
# %bb.29:                               # %cinch.runtime.natAdd.exit45.thread
	leaq	1(,%rax,2), %r15
	jmp	.LBB140_38
.LBB140_30:                             # %cinch.runtime.natAdd.exit45
	movq	%r12, %rdi
	movq	%rcx, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %r15
	testq	%r12, %r12
	sete	%al
	movl	%r12d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB140_38
# %bb.31:                               # %bb0.i404
	movq	-8(%r12), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB140_33
# %bb.32:                               # %bb2.i414
	decq	%rcx
	movq	%rcx, -8(%r12)
	jmp	.LBB140_38
.LBB140_33:                             # %bb3.i410
	testq	%rax, %rax
	je	.LBB140_38
# %bb.34:                               # %bb3.i410
	movq	%r12, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB140_37
# %bb.35:                               # %bb5.i411
	callq	cinch_drop_slow@PLT
	jmp	.LBB140_38
.LBB140_37:                             # %bb4.i413
	callq	cinch_drop_mt@PLT
.LBB140_38:                             # %cinch.runtime.natCmp.exit113
	leaq	-2(%r14), %r12
	cmpq	$2, %r14
	movl	$1, %r14d
	cmovbq	%r14, %r12
	testq	%rbx, %rbx
	je	.LBB140_41
# %bb.39:                               # %bb0.i142
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB140_41
# %bb.40:                               # %cinch.runtime.natCmp.exit103.thread
	shrq	%r12
	andq	(%rbx), %r13
	cmpq	%r13, %r12
	jb	.LBB140_43
	jmp	.LBB140_48
.LBB140_41:                             # %cinch.runtime.natCmp.exit103
	movq	%rbx, %rdi
	callq	cinch_bytes_len@PLT
	shrq	%r12
	andq	%r13, %rax
	cmpq	%rax, %r12
	jae	.LBB140_48
# %bb.42:                               # %bb2.i211
	testq	%rbx, %rbx
	je	.LBB140_46
.LBB140_43:                             # %bb3.i214
	movq	-8(%rbx), %rax
	movabsq	$1030792151040, %rcx            # imm = 0xF000000000
	andq	%rcx, %rax
	movabsq	$274877906944, %rcx             # imm = 0x4000000000
	cmpq	%rcx, %rax
	jne	.LBB140_46
# %bb.44:                               # %bb4.i
	cmpq	(%rbx), %r12
	jae	.LBB140_46
# %bb.45:                               # %bb7.i
	movq	8(%rbx), %rax
	movzbl	(%rax,%r12), %eax
	jmp	.LBB140_47
.LBB140_46:                             # %bb5.i
	movq	%rbx, %rdi
	movq	%r12, %rsi
	callq	cinch_bytes_get@PLT
.LBB140_47:                             # %cinch.runtime.bytesGet.exit
	leaq	1(,%rax,2), %r14
.LBB140_48:                             # %bb10
	testb	$1, %r15b
	je	.LBB140_51
# %bb.49:                               # %bb0.i
	movq	%r15, %rcx
	shrq	%rcx
	movq	%r14, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB140_51
# %bb.50:                               # %cinch.runtime.natAdd.exit.thread
	leaq	1(,%rax,2), %rbx
	testb	$1, %bl
	je	.LBB140_58
.LBB140_73:                             # %cinch.runtime.natCmp.exit.thread
	shrq	%rbx
	movq	%rbx, %rax
.LBB140_74:                             # %common.ret
	addq	$8, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB140_51:                             # %cinch.runtime.natAdd.exit
	.cfi_def_cfa %rbp, 16
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	cinch_nat_add@PLT
	movq	%rax, %rbx
	testq	%r15, %r15
	sete	%al
	movl	%r15d, %ecx
	orb	%al, %cl
	testb	$1, %cl
	je	.LBB140_52
.LBB140_57:                             # %cinch.rc.drop.tobj.exit535
	testb	$1, %bl
	jne	.LBB140_73
.LBB140_58:                             # %cinch.runtime.natCmp.exit
	movq	%rbx, %rdi
	movq	$-1, %rsi
	callq	cinch_nat_cmp@PLT
	testq	%rax, %rax
	jle	.LBB140_62
# %bb.59:                               # %bb14
	testq	%rbx, %rbx
	je	.LBB140_65
# %bb.60:                               # %bb0.i576
	movq	-8(%rbx), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB140_66
# %bb.61:                               # %bb2.i586
	decq	%rcx
	movq	%rcx, -8(%rbx)
	xorl	%eax, %eax
	jmp	.LBB140_74
.LBB140_52:                             # %bb0.i523
	movq	-8(%r15), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB140_54
# %bb.53:                               # %bb2.i533
	decq	%rcx
	movq	%rcx, -8(%r15)
	testb	$1, %bl
	je	.LBB140_58
	jmp	.LBB140_73
.LBB140_62:                             # %cinch.runtime.natToI64.exit
	movq	%rbx, %rdi
	callq	cinch_nat_to_i64@PLT
	testq	%rbx, %rbx
	je	.LBB140_74
# %bb.63:                               # %bb0.i559
	movq	-8(%rbx), %rdx
	movq	%rdx, %rcx
	shlq	$29, %rcx
	cmpq	$536870913, %rcx                # imm = 0x20000001
	jl	.LBB140_69
# %bb.64:                               # %bb2.i569
	decq	%rdx
	movq	%rdx, -8(%rbx)
	jmp	.LBB140_74
.LBB140_54:                             # %bb3.i529
	testq	%rax, %rax
	je	.LBB140_57
# %bb.55:                               # %bb3.i529
	movq	%r15, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB140_72
# %bb.56:                               # %bb5.i530
	callq	cinch_drop_slow@PLT
	jmp	.LBB140_57
.LBB140_65:
	xorl	%eax, %eax
	jmp	.LBB140_74
.LBB140_66:                             # %bb3.i582
	testq	%rax, %rax
	je	.LBB140_74
# %bb.67:                               # %bb3.i582
	movq	%rbx, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB140_75
# %bb.68:                               # %bb5.i583
	callq	cinch_drop_slow@PLT
	xorl	%eax, %eax
	jmp	.LBB140_74
.LBB140_69:                             # %bb3.i565
	testq	%rcx, %rcx
	je	.LBB140_74
# %bb.70:                               # %bb3.i565
	movq	%rax, %r14
	movq	%rbx, %rdi
	cmpq	$536870912, %rcx                # imm = 0x20000000
	jne	.LBB140_76
# %bb.71:                               # %bb5.i566
	callq	cinch_drop_slow@PLT
	jmp	.LBB140_77
.LBB140_72:                             # %bb4.i532
	callq	cinch_drop_mt@PLT
	testb	$1, %bl
	je	.LBB140_58
	jmp	.LBB140_73
.LBB140_75:                             # %bb4.i585
	callq	cinch_drop_mt@PLT
	xorl	%eax, %eax
	jmp	.LBB140_74
.LBB140_76:                             # %bb4.i568
	callq	cinch_drop_mt@PLT
.LBB140_77:                             # %common.ret
	movq	%r14, %rax
	jmp	.LBB140_74
.Lfunc_end140:
	.size	Main.measure.shared0, .Lfunc_end140-Main.measure.shared0
	.cfi_endproc
                                        # -- End function
	.globl	Main.measure.shared1            # -- Begin function Main.measure.shared1
	.p2align	4
	.type	Main.measure.shared1,@function
Main.measure.shared1:                   # @Main.measure.shared1
	.cfi_startproc
# %bb.0:                                # %cinch.runtime.natFromI64.exit
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r12
	pushq	%rbx
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movl	%edi, %ebx
	addq	%rbx, %rbx
	shrq	$31, %rdi
	orq	$1, %rdi
	movabsq	$8589934590, %rsi               # imm = 0x1FFFFFFFE
	addq	$3, %rsi
	callq	cinch_nat_mul@PLT
	movq	%rax, %r14
	testb	$1, %r14b
	je	.LBB141_3
# %bb.1:                                # %bb0.i
	movq	%rbx, %rcx
	shrq	%rcx
	movq	%r14, %rax
	shrq	%rax
	addq	%rcx, %rax
	js	.LBB141_3
# %bb.2:                                # %cinch.rc.drop.tobj.exit59.thread
	leaq	1(,%rax,2), %rbx
	jmp	.LBB141_10
.LBB141_3:                              # %cinch.rc.drop.tobj.exit59
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
	jne	.LBB141_10
# %bb.4:                                # %bb0.i64
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB141_6
# %bb.5:                                # %bb2.i74
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB141_10
.LBB141_6:                              # %bb3.i70
	testq	%rax, %rax
	je	.LBB141_10
# %bb.7:                                # %bb3.i70
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB141_9
# %bb.8:                                # %bb5.i71
	callq	cinch_drop_slow@PLT
	jmp	.LBB141_10
.LBB141_9:                              # %bb4.i73
	callq	cinch_drop_mt@PLT
.LBB141_10:                             # %cinch.rc.drop.tobj.exit76
	testq	%rbx, %rbx
	sete	%al
	movl	%ebx, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB141_15
# %bb.11:                               # %bb0.i81
	movq	-8(%rbx), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB141_13
# %bb.12:                               # %bb2.i90
	incq	%rax
	movq	%rax, -8(%rbx)
	jmp	.LBB141_15
.LBB141_13:                             # %bb3.i87
	je	.LBB141_15
# %bb.14:                               # %bb4.i88
	movq	%rbx, %rdi
	callq	cinch_dup_mt@PLT
.LBB141_15:                             # %cinch.rc.dup.tobj.exit
	movq	%rbx, %rdi
	callq	"Text.Show.nat-digit-count"@PLT
	movq	%rax, %r14
	testq	%rax, %rax
	sete	%al
	movl	%r14d, %r12d
	orb	%al, %r12b
	testb	$1, %r12b
	jne	.LBB141_20
# %bb.16:                               # %bb0.i96
	movq	-8(%r14), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB141_18
# %bb.17:                               # %bb2.i106
	incq	%rax
	movq	%rax, -8(%r14)
	jmp	.LBB141_20
.LBB141_18:                             # %bb3.i102
	je	.LBB141_20
# %bb.19:                               # %bb4.i104
	movq	%r14, %rdi
	callq	cinch_dup_mt@PLT
.LBB141_20:                             # %cinch.rc.dup.tobj.exit108
	movq	%r14, %rdi
	callq	cinch_nat_to_len@PLT
	movq	%rax, %rdi
	callq	cinch_bytes_new@PLT
	movq	%rax, %r15
	testb	$1, %r12b
	jne	.LBB141_27
# %bb.21:                               # %bb0.i.i
	movq	-8(%r14), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB141_23
# %bb.22:                               # %bb2.i.i
	decq	%rcx
	movq	%rcx, -8(%r14)
	jmp	.LBB141_27
.LBB141_23:                             # %bb3.i.i
	testq	%rax, %rax
	je	.LBB141_27
# %bb.24:                               # %bb3.i.i
	movq	%r14, %rdi
	cmpq	$536870912, %rax                # imm = 0x20000000
	jne	.LBB141_26
# %bb.25:                               # %bb5.i.i
	callq	cinch_drop_slow@PLT
	jmp	.LBB141_27
.LBB141_26:                             # %bb4.i.i
	callq	cinch_drop_mt@PLT
.LBB141_27:                             # %cinch.runtime.bytesNew.exit
	movl	$1, %esi
	movq	%r15, %rdi
	movq	%rbx, %rdx
	movq	%r14, %rcx
	callq	"Text.Show.nat-digits-into"@PLT
	movq	%r15, %rdi
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	jmp	"Text.Show.wrap-bytes"@PLT      # TAILCALL
.Lfunc_end141:
	.size	Main.measure.shared1, .Lfunc_end141-Main.measure.shared1
	.cfi_endproc
                                        # -- End function
	.p2align	4                               # -- Begin function cinch.rc.dup.obj
	.type	cinch.rc.dup.obj,@function
cinch.rc.dup.obj:                       # @cinch.rc.dup.obj
	.cfi_startproc
# %bb.0:
	testq	%rdi, %rdi
	je	.LBB142_4
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB142_3
# %bb.2:                                # %bb2
	incq	%rax
	movq	%rax, -8(%rdi)
	retq
.LBB142_3:                              # %bb3
	jne	cinch_dup_mt@PLT                # TAILCALL
.LBB142_4:                              # %bb1
	retq
.Lfunc_end142:
	.size	cinch.rc.dup.obj, .Lfunc_end142-cinch.rc.dup.obj
	.cfi_endproc
                                        # -- End function
	.p2align	4                               # -- Begin function cinch.rc.drop.obj
	.type	cinch.rc.drop.obj,@function
cinch.rc.drop.obj:                      # @cinch.rc.drop.obj
	.cfi_startproc
# %bb.0:
	testq	%rdi, %rdi
	je	.LBB143_3
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB143_4
# %bb.2:                                # %bb2
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB143_3
.LBB143_4:                              # %bb3
	testq	%rax, %rax
	je	.LBB143_3
# %bb.5:                                # %bb3
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	cinch_drop_slow@PLT             # TAILCALL
# %bb.6:                                # %bb4
	jmp	cinch_drop_mt@PLT               # TAILCALL
.LBB143_3:                              # %bb1
	retq
.Lfunc_end143:
	.size	cinch.rc.drop.obj, .Lfunc_end143-cinch.rc.drop.obj
	.cfi_endproc
                                        # -- End function
	.p2align	4                               # -- Begin function cinch.rc.dup.tobj
	.type	cinch.rc.dup.tobj,@function
cinch.rc.dup.tobj:                      # @cinch.rc.dup.tobj
	.cfi_startproc
# %bb.0:
	testq	%rdi, %rdi
	sete	%al
	movl	%edi, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB144_4
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rax
	movq	%rax, %rcx
	shlq	$29, %rcx
	testq	%rcx, %rcx
	jle	.LBB144_3
# %bb.2:                                # %bb2
	incq	%rax
	movq	%rax, -8(%rdi)
	retq
.LBB144_3:                              # %bb3
	jne	cinch_dup_mt@PLT                # TAILCALL
.LBB144_4:                              # %bb1
	retq
.Lfunc_end144:
	.size	cinch.rc.dup.tobj, .Lfunc_end144-cinch.rc.dup.tobj
	.cfi_endproc
                                        # -- End function
	.p2align	4                               # -- Begin function cinch.rc.drop.tobj
	.type	cinch.rc.drop.tobj,@function
cinch.rc.drop.tobj:                     # @cinch.rc.drop.tobj
	.cfi_startproc
# %bb.0:
	testq	%rdi, %rdi
	sete	%al
	movl	%edi, %ecx
	orb	%al, %cl
	testb	$1, %cl
	jne	.LBB145_3
# %bb.1:                                # %bb0
	movq	-8(%rdi), %rcx
	movq	%rcx, %rax
	shlq	$29, %rax
	cmpq	$536870913, %rax                # imm = 0x20000001
	jl	.LBB145_4
# %bb.2:                                # %bb2
	decq	%rcx
	movq	%rcx, -8(%rdi)
	jmp	.LBB145_3
.LBB145_4:                              # %bb3
	testq	%rax, %rax
	je	.LBB145_3
# %bb.5:                                # %bb3
	cmpq	$536870912, %rax                # imm = 0x20000000
	je	cinch_drop_slow@PLT             # TAILCALL
# %bb.6:                                # %bb4
	jmp	cinch_drop_mt@PLT               # TAILCALL
.LBB145_3:                              # %bb1
	retq
.Lfunc_end145:
	.size	cinch.rc.drop.tobj, .Lfunc_end145-cinch.rc.drop.tobj
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movslq	%edi, %rdi
	callq	cinch_argv_init@PLT
	movq	Main.main.exit@GOTPCREL(%rip), %rdi
	callq	cinch_run_main@PLT
	movq	%rax, %rdi
	popq	%rax
	.cfi_def_cfa_offset 8
	jmp	cinch_exit_check@PLT            # TAILCALL
.Lfunc_end146:
	.size	main, .Lfunc_end146-main
	.cfi_endproc
                                        # -- End function
	.p2align	4                               # -- Begin function __cinch_reclaim_init
	.type	__cinch_reclaim_init,@function
__cinch_reclaim_init:                   # @__cinch_reclaim_init
	.cfi_startproc
# %bb.0:                                # %entry
	jmp	cinch_reclaim_enable@PLT        # TAILCALL
.Lfunc_end147:
	.size	__cinch_reclaim_init, .Lfunc_end147-__cinch_reclaim_init
	.cfi_endproc
                                        # -- End function
	.type	"Data.Core.Nil.blk#h",@object   # @"Data.Core.Nil.blk#h"
	.section	.rodata,"a",@progbits
	.p2align	3, 0x0
"Data.Core.Nil.blk#h":
	.quad	34359738368                     # 0x800000000
	.size	"Data.Core.Nil.blk#h", 8

	.type	"Data.Core.None.blk#h",@object  # @"Data.Core.None.blk#h"
	.p2align	3, 0x0
"Data.Core.None.blk#h":
	.quad	34359738368                     # 0x800000000
	.size	"Data.Core.None.blk#h", 8

	.type	"Data.Uuid.UuidClockExhausted.blk#h",@object # @"Data.Uuid.UuidClockExhausted.blk#h"
	.p2align	3, 0x0
"Data.Uuid.UuidClockExhausted.blk#h":
	.quad	4432406249472                   # 0x40800000000
	.size	"Data.Uuid.UuidClockExhausted.blk#h", 8

	.type	"Data.Uuid.UuidClockOutOfRange.blk#h",@object # @"Data.Uuid.UuidClockOutOfRange.blk#h"
	.p2align	3, 0x0
"Data.Uuid.UuidClockOutOfRange.blk#h":
	.quad	3332894621696                   # 0x30800000000
	.size	"Data.Uuid.UuidClockOutOfRange.blk#h", 8

	.type	"Effect.Process.ProcessInvalidArgument.blk#h",@object # @"Effect.Process.ProcessInvalidArgument.blk#h"
	.p2align	3, 0x0
"Effect.Process.ProcessInvalidArgument.blk#h":
	.quad	2233382993920                   # 0x20800000000
	.size	"Effect.Process.ProcessInvalidArgument.blk#h", 8

	.type	cinch.tail.argv,@object         # @cinch.tail.argv
	.section	.tdata,"awT",@progbits
	.p2align	4, 0x0
cinch.tail.argv:
	.zero	128
	.size	cinch.tail.argv, 128

	.type	"cinch.prim.u32ToChar.blk#h",@object # @"cinch.prim.u32ToChar.blk#h"
	.section	.rodata,"a",@progbits
	.p2align	3, 0x0
"cinch.prim.u32ToChar.blk#h":
	.quad	34359738368                     # 0x800000000
	.size	"cinch.prim.u32ToChar.blk#h", 8

	.type	Main.finish.dec0,@object        # @Main.finish.dec0
	.section	.data.rel.ro,"aw",@progbits
	.p2align	4, 0x0
Main.finish.dec0:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.finish.dec0+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.finish.dec0+40
	.byte	32
	.zero	7
	.size	Main.finish.dec0, 48

	.type	Main.finish.dec1,@object        # @Main.finish.dec1
	.p2align	4, 0x0
Main.finish.dec1:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.finish.dec1+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.finish.dec1+40
	.byte	32
	.zero	7
	.size	Main.finish.dec1, 48

	.type	Main.finish.dec2,@object        # @Main.finish.dec2
	.p2align	4, 0x0
Main.finish.dec2:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.finish.dec2+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.finish.dec2+40
	.byte	32
	.zero	7
	.size	Main.finish.dec2, 48

	.type	Main.finish.dec3,@object        # @Main.finish.dec3
	.p2align	4, 0x0
Main.finish.dec3:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.finish.dec3+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.finish.dec3+40
	.byte	10
	.zero	7
	.size	Main.finish.dec3, 48

	.type	Main.measure.dec0,@object       # @Main.measure.dec0
	.p2align	4, 0x0
Main.measure.dec0:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec0+24
	.quad	2508260900864                   # 0x24800000000
	.quad	6                               # 0x6
	.quad	Main.measure.dec0+40
	.ascii	"random"
	.zero	2
	.size	Main.measure.dec0, 48

	.type	Main.measure.dec1,@object       # @Main.measure.dec1
	.p2align	4, 0x0
Main.measure.dec1:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec1+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.measure.dec1+40
	.byte	32
	.zero	7
	.size	Main.measure.dec1, 48

	.type	Main.measure.dec2,@object       # @Main.measure.dec2
	.p2align	4, 0x0
Main.measure.dec2:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec2+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.measure.dec2+40
	.byte	32
	.zero	7
	.size	Main.measure.dec2, 48

	.type	Main.measure.dec3,@object       # @Main.measure.dec3
	.p2align	4, 0x0
Main.measure.dec3:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec3+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.measure.dec3+40
	.byte	32
	.zero	7
	.size	Main.measure.dec3, 48

	.type	Main.measure.dec4,@object       # @Main.measure.dec4
	.p2align	4, 0x0
Main.measure.dec4:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec4+24
	.quad	2508260900864                   # 0x24800000000
	.quad	1                               # 0x1
	.quad	Main.measure.dec4+40
	.byte	10
	.zero	7
	.size	Main.measure.dec4, 48

	.type	Main.measure.dec5,@object       # @Main.measure.dec5
	.p2align	4, 0x0
Main.measure.dec5:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec5+24
	.quad	2508260900864                   # 0x24800000000
	.quad	10                              # 0xa
	.quad	Main.measure.dec5+40
	.ascii	"allocation"
	.zero	6
	.size	Main.measure.dec5, 56

	.type	Main.measure.dec6,@object       # @Main.measure.dec6
	.p2align	4, 0x0
Main.measure.dec6:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec6+24
	.quad	2508260900864                   # 0x24800000000
	.quad	7                               # 0x7
	.quad	Main.measure.dec6+40
	.ascii	"counter"
	.zero	1
	.size	Main.measure.dec6, 48

	.type	Main.measure.dec7,@object       # @Main.measure.dec7
	.p2align	4, 0x0
Main.measure.dec7:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec7+24
	.quad	2508260900864                   # 0x24800000000
	.quad	5                               # 0x5
	.quad	Main.measure.dec7+40
	.ascii	"clock"
	.zero	3
	.size	Main.measure.dec7, 48

	.type	Main.measure.dec8,@object       # @Main.measure.dec8
	.p2align	4, 0x0
Main.measure.dec8:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec8+24
	.quad	2508260900864                   # 0x24800000000
	.quad	2                               # 0x2
	.quad	Main.measure.dec8+40
	.ascii	"v4"
	.zero	6
	.size	Main.measure.dec8, 48

	.type	Main.measure.dec9,@object       # @Main.measure.dec9
	.p2align	4, 0x0
Main.measure.dec9:
	.quad	281509336449024                 # 0x1000800000000
	.quad	Main.measure.dec9+24
	.quad	2508260900864                   # 0x24800000000
	.quad	2                               # 0x2
	.quad	Main.measure.dec9+40
	.ascii	"v7"
	.zero	6
	.size	Main.measure.dec9, 48

	.section	.init_array,"aw",@init_array
	.p2align	3, 0x0
	.quad	__cinch_reclaim_init
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym "Data.String.Eq_String.==.tapp"
	.addrsig_sym Main.main.exit
	.addrsig_sym __cinch_reclaim_init
	.addrsig_sym "Data.Core.Nil.blk#h"
	.addrsig_sym "Data.Core.None.blk#h"
	.addrsig_sym "Data.Uuid.UuidClockExhausted.blk#h"
	.addrsig_sym "Data.Uuid.UuidClockOutOfRange.blk#h"
	.addrsig_sym "Effect.Process.ProcessInvalidArgument.blk#h"
	.addrsig_sym "cinch.prim.u32ToChar.blk#h"
	.addrsig_sym Main.finish.dec0
	.addrsig_sym Main.finish.dec1
	.addrsig_sym Main.finish.dec2
	.addrsig_sym Main.finish.dec3
	.addrsig_sym Main.measure.dec0
	.addrsig_sym Main.measure.dec1
	.addrsig_sym Main.measure.dec2
	.addrsig_sym Main.measure.dec3
	.addrsig_sym Main.measure.dec4
	.addrsig_sym Main.measure.dec5
	.addrsig_sym Main.measure.dec6
	.addrsig_sym Main.measure.dec7
	.addrsig_sym Main.measure.dec8
	.addrsig_sym Main.measure.dec9
