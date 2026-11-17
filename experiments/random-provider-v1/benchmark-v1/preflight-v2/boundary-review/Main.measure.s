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
