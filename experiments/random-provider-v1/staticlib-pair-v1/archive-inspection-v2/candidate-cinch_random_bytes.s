0000000000000000 <cinch_random_bytes>:
       0:      	pushq	%rbp
       1:      	pushq	%r15
       3:      	pushq	%r14
       5:      	pushq	%r13
       7:      	pushq	%r12
       9:      	pushq	%rbx
       a:      	subq	$0x48, %rsp
       e:      	movq	%rdi, 0x30(%rsp)
      13:      	testq	%rdi, %rdi
      16:      	js	0x19d <cinch_random_bytes+0x19d>
      1c:      	je	0x22e <cinch_random_bytes+0x22e>
      22:      	movabsq	$-0x100000000, %rbx     # imm = 0xFFFFFFFF00000000
      2c:      	callq	*(%rip)                 # 0x32 <cinch_random_bytes+0x32>
      32:      	movl	$0x1, %esi
      37:      	movq	0x30(%rsp), %r12
      3c:      	movq	%r12, %rdi
      3f:      	callq	0x44 <cinch_random_bytes+0x44>
      44:      	movq	%rax, %r13
      47:      	leaq	(%rip), %r14            # 0x4e <cinch_random_bytes+0x4e>
      4e:      	movq	%rax, 0x8(%rsp)
      53:      	nopw	%cs:(%rax,%rax)
      60:      	movl	$0x13e, %eax            # imm = 0x13E
      65:      	movq	%r13, %rdi
      68:      	movq	%r12, %rsi
      6b:      	xorl	%edx, %edx
      6d:      	xorl	%r10d, %r10d
      70:      	xorl	%r8d, %r8d
      73:      	xorl	%r9d, %r9d
      76:      	syscall
      78:      	movq	%rax, %rbp
      7b:      	cmpq	$-0x1000, %rax          # imm = 0xF000
      81:      	ja	0xc0 <cinch_random_bytes+0xc0>
      83:      	testq	%rbp, %rbp
      86:      	jns	0x16b <cinch_random_bytes+0x16b>
      8c:      	movl	$0x15, %edx
      91:      	movl	$0x15, %edi
      96:      	leaq	(%rip), %rsi            # 0x9d <cinch_random_bytes+0x9d>
      9d:      	callq	*(%rip)                 # 0xa3 <cinch_random_bytes+0xa3>
      a3:      	movq	%rax, %rbp
      a6:      	movl	%ebp, %r15d
      a9:      	andl	$0x3, %r15d
      ad:      	movslq	(%r14,%r15,4), %rax
      b1:      	addq	%r14, %rax
      b4:      	jmpq	*%rax
      b6:      	nopw	%cs:(%rax,%rax)
      c0:      	movq	(%rip), %rax            # 0xc7 <cinch_random_bytes+0xc7>
      c7:      	movq	(%rax), %rax
      ca:      	leaq	(%rip), %rcx            # 0xd1 <cinch_random_bytes+0xd1>
      d1:      	cmpq	%rcx, %rax
      d4:      	jne	0x18b <cinch_random_bytes+0x18b>
      da:      	imulq	%rbx, %rbp
      de:      	orq	$0x2, %rbp
      e2:      	movl	%ebp, %r15d
      e5:      	andl	$0x3, %r15d
      e9:      	movslq	(%r14,%r15,4), %rax
      ed:      	addq	%r14, %rax
      f0:      	jmpq	*%rax
      f2:      	movzbl	0x10(%rbp), %eax
      f6:      	cmpb	$0x23, %al
      f8:      	je	0x138 <cinch_random_bytes+0x138>
      fa:      	jmp	0x1a9 <cinch_random_bytes+0x1a9>
      ff:      	nop
     100:      	callq	*(%rip)                 # 0x106 <cinch_random_bytes+0x106>
     106:      	movq	%rbp, %rdi
     109:      	shrq	$0x20, %rdi
     10d:      	callq	*0x8(%rax)
     110:      	jmp	0x127 <cinch_random_bytes+0x127>
     112:      	nopw	%cs:(%rax,%rax)
     120:      	movq	%rbp, %rax
     123:      	shrq	$0x20, %rax
     127:      	cmpb	$0x23, %al
     129:      	je	0x138 <cinch_random_bytes+0x138>
     12b:      	jmp	0x1a9 <cinch_random_bytes+0x1a9>
     12d:      	nopl	(%rax)
     130:      	movzbl	0x1f(%rbp), %eax
     134:      	cmpb	$0x23, %al
     136:      	jne	0x1a9 <cinch_random_bytes+0x1a9>
     138:      	leal	-0x2(%r15), %eax
     13c:      	cmpl	$0x2, %eax
     13f:      	jb	0x60 <cinch_random_bytes+0x60>
     145:      	testq	%r15, %r15
     148:      	je	0x60 <cinch_random_bytes+0x60>
     14e:      	decq	%rbp
     151:      	movq	%rbp, 0x18(%rsp)
     156:      	movb	$0x3, 0x10(%rsp)
     15b:      	leaq	0x18(%rsp), %rdi
     160:      	callq	*(%rip)                 # 0x166 <cinch_random_bytes+0x166>
     166:      	jmp	0x60 <cinch_random_bytes+0x60>
     16b:      	je	0x3e1 <cinch_random_bytes+0x3e1>
     171:      	cmpq	%rbp, %r12
     174:      	jb	0x419 <cinch_random_bytes+0x419>
     17a:      	addq	%rbp, %r13
     17d:      	subq	%rbp, %r12
     180:      	jne	0x60 <cinch_random_bytes+0x60>
     186:      	jmp	0x4f4 <cinch_random_bytes+0x4f4>
     18b:      	leaq	(%rip), %rdi            # 0x192 <cinch_random_bytes+0x192>
     192:      	callq	*(%rip)                 # 0x198 <cinch_random_bytes+0x198>
     198:      	jmp	0xda <cinch_random_bytes+0xda>
     19d:      	movq	$-0x6, %rbx
     1a4:      	jmp	0x4d0 <cinch_random_bytes+0x4d0>
     1a9:      	cmpl	$0x2, %r15d
     1ad:      	jb	0x438 <cinch_random_bytes+0x438>
     1b3:      	cmpl	$0x2, %r15d
     1b7:      	jne	0x438 <cinch_random_bytes+0x438>
     1bd:      	andq	%rbp, %rbx
     1c0:      	movabsq	$0x2600000000, %rax     # imm = 0x2600000000
     1ca:      	cmpq	%rax, %rbx
     1cd:      	jne	0x438 <cinch_random_bytes+0x438>
     1d3:      	movabsq	$0x1b600000000, %rax    # imm = 0x1B600000000
     1dd:      	movq	%rax, 0x10(%rsp)
     1e2:      	movq	$0x1, 0x18(%rsp)
     1eb:      	leaq	(%rip), %rdx            # 0x1f2 <cinch_random_bytes+0x1f2>
     1f2:      	leaq	0x38(%rsp), %rdi
     1f7:      	leaq	0x10(%rsp), %rsi
     1fc:      	movl	$0xc, %ecx
     201:      	callq	*(%rip)                 # 0x207 <cinch_random_bytes+0x207>
     207:      	cmpl	$0x1, 0x38(%rsp)
     20c:      	jne	0x2f7 <cinch_random_bytes+0x2f7>
     212:      	movq	0x40(%rsp), %r15
     217:      	movl	%r15d, %r14d
     21a:      	andl	$0x3, %r14d
     21e:      	leaq	(%rip), %rax            # 0x225 <cinch_random_bytes+0x225>
     225:      	movslq	(%rax,%r14,4), %rcx
     229:      	addq	%rax, %rcx
     22c:      	jmpq	*%rcx
     22e:      	movabsq	$0x1b600000000, %rax    # imm = 0x1B600000000
     238:      	movq	%rax, 0x10(%rsp)
     23d:      	movq	$0x1, 0x18(%rsp)
     246:      	movl	$0x1, %ebx
     24b:      	leaq	(%rip), %rdx            # 0x252 <cinch_random_bytes+0x252>
     252:      	leaq	0x38(%rsp), %rdi
     257:      	leaq	0x10(%rsp), %rsi
     25c:      	movl	$0xc, %ecx
     261:      	callq	*(%rip)                 # 0x267 <cinch_random_bytes+0x267>
     267:      	cmpl	$0x1, 0x38(%rsp)
     26c:      	jne	0x294 <cinch_random_bytes+0x294>
     26e:      	movq	0x40(%rsp), %r15
     273:      	movl	$0x1, %eax
     278:      	movq	%rax, 0x8(%rsp)
     27d:      	movl	%r15d, %r14d
     280:      	andl	$0x3, %r14d
     284:      	leaq	(%rip), %rax            # 0x28b <cinch_random_bytes+0x28b>
     28b:      	movslq	(%rax,%r14,4), %rcx
     28f:      	addq	%rax, %rcx
     292:      	jmpq	*%rcx
     294:      	movl	0x3c(%rsp), %edi
     298:      	callq	*(%rip)                 # 0x29e <cinch_random_bytes+0x29e>
     29e:      	movl	$0x1, %ecx
     2a3:      	movq	0x30(%rsp), %rax
     2a8:      	movq	%rax, 0x10(%rsp)
     2ad:      	movq	%rcx, 0x18(%rsp)
     2b2:      	movq	%rax, 0x20(%rsp)
     2b7:      	leaq	0x10(%rsp), %rdi
     2bc:      	callq	0x2c1 <cinch_random_bytes+0x2c1>
     2c1:      	movq	%rax, %r14
     2c4:      	movq	%rdx, %r15
     2c7:      	movabsq	$0x24000000001, %rsi    # imm = 0x24000000001
     2d1:      	movl	$0x10, %edi
     2d6:      	movl	$0x10, %ecx
     2db:      	xorl	%edx, %edx
     2dd:      	xorl	%r8d, %r8d
     2e0:      	xorl	%r9d, %r9d
     2e3:      	callq	0x2e8 <cinch_random_bytes+0x2e8>
     2e8:      	movq	%rax, %rbx
     2eb:      	movq	%r14, (%rax)
     2ee:      	movq	%r15, 0x8(%rax)
     2f2:      	jmp	0x4d0 <cinch_random_bytes+0x4d0>
     2f7:      	movl	0x3c(%rsp), %eax
     2fb:      	movl	%eax, 0x4(%rsp)
     2ff:      	leaq	0x4(%rsp), %r14
     304:      	leaq	(%rip), %rbx            # 0x30b <cinch_random_bytes+0x30b>
     30b:      	jmp	0x349 <cinch_random_bytes+0x349>
     30d:      	movq	%r15, %rax
     310:      	shrq	$0x20, %rax
     314:      	movabsq	$0x2c00000000, %rcx     # imm = 0x2C00000000
     31e:      	cmpq	%rcx, %r15
     321:      	movl	$0xff, %ecx
     326:      	cmovael	%ecx, %eax
     329:      	cmpb	$0x23, %al
     32b:      	jne	0x3f8 <cinch_random_bytes+0x3f8>
     331:      	nopw	%cs:(%rax,%rax)
     340:      	testq	%r12, %r12
     343:      	je	0x3cd <cinch_random_bytes+0x3cd>
     349:      	movq	%r14, %rdi
     34c:      	movq	%r13, %rsi
     34f:      	movq	%r12, %rdx
     352:      	callq	*(%rip)                 # 0x358 <cinch_random_bytes+0x358>
     358:      	movq	%rdx, %r15
     35b:      	cmpq	$0x1, %rax
     35f:      	jne	0x379 <cinch_random_bytes+0x379>
     361:      	movl	%r15d, %eax
     364:      	andl	$0x3, %eax
     367:      	movslq	(%rbx,%rax,4), %rax
     36b:      	addq	%rbx, %rax
     36e:      	jmpq	*%rax
     370:      	cmpb	$0x23, 0x10(%r15)
     375:      	je	0x340 <cinch_random_bytes+0x340>
     377:      	jmp	0x3f8 <cinch_random_bytes+0x3f8>
     379:      	testq	%r15, %r15
     37c:      	je	0x3f1 <cinch_random_bytes+0x3f1>
     37e:      	movq	%r12, %rax
     381:      	subq	%r15, %rax
     384:      	jb	0x4fe <cinch_random_bytes+0x4fe>
     38a:      	addq	%r15, %r13
     38d:      	movq	%rax, %r12
     390:      	jmp	0x340 <cinch_random_bytes+0x340>
     392:      	callq	*(%rip)                 # 0x398 <cinch_random_bytes+0x398>
     398:      	movq	%r15, %rdi
     39b:      	shrq	$0x20, %rdi
     39f:      	callq	*0x10(%rax)
     3a2:      	testb	%al, %al
     3a4:      	jne	0x340 <cinch_random_bytes+0x340>
     3a6:      	jmp	0x3f8 <cinch_random_bytes+0x3f8>
     3a8:      	cmpb	$0x23, 0x1f(%r15)
     3ad:      	jne	0x3f8 <cinch_random_bytes+0x3f8>
     3af:      	leaq	-0x1(%r15), %rax
     3b3:      	movq	%rax, 0x18(%rsp)
     3b8:      	movb	$0x3, 0x10(%rsp)
     3bd:      	leaq	0x18(%rsp), %rdi
     3c2:      	callq	*(%rip)                 # 0x3c8 <cinch_random_bytes+0x3c8>
     3c8:      	jmp	0x340 <cinch_random_bytes+0x340>
     3cd:      	movl	0x4(%rsp), %edi
     3d1:      	callq	*(%rip)                 # 0x3d7 <cinch_random_bytes+0x3d7>
     3d7:      	movq	0x8(%rsp), %rcx
     3dc:      	jmp	0x2a3 <cinch_random_bytes+0x2a3>
     3e1:      	movl	$0x1f, %edx
     3e6:      	movb	$0x25, %al
     3e8:      	leaq	(%rip), %rsi            # 0x3ef <cinch_random_bytes+0x3ef>
     3ef:      	jmp	0x427 <cinch_random_bytes+0x427>
     3f1:      	leaq	(%rip), %r15            # 0x3f8 <cinch_random_bytes+0x3f8>
     3f8:      	movl	0x4(%rsp), %edi
     3fc:      	callq	*(%rip)                 # 0x402 <cinch_random_bytes+0x402>
     402:      	movl	%r15d, %r14d
     405:      	andl	$0x3, %r14d
     409:      	leaq	(%rip), %rax            # 0x410 <cinch_random_bytes+0x410>
     410:      	movslq	(%rax,%r14,4), %rcx
     414:      	addq	%rax, %rcx
     417:      	jmpq	*%rcx
     419:      	movl	$0x2c, %edx
     41e:      	movb	$0x15, %al
     420:      	leaq	(%rip), %rsi            # 0x427 <cinch_random_bytes+0x427>
     427:      	movzbl	%al, %edi
     42a:      	movq	0x8(%rsp), %rbx
     42f:      	callq	*(%rip)                 # 0x435 <cinch_random_bytes+0x435>
     435:      	movq	%rax, %rbp
     438:      	movl	%ebp, %r14d
     43b:      	andl	$0x3, %r14d
     43f:      	leaq	(%rip), %rax            # 0x446 <cinch_random_bytes+0x446>
     446:      	movslq	(%rax,%r14,4), %rcx
     44a:      	addq	%rax, %rcx
     44d:      	movq	%rbp, %r15
     450:      	jmpq	*%rcx
     452:      	movzbl	0x10(%r15), %eax
     457:      	movq	$-0x7, %rbx
     45e:      	cmpb	$0x15, %al
     460:      	jb	0x488 <cinch_random_bytes+0x488>
     462:      	jmp	0x496 <cinch_random_bytes+0x496>
     464:      	callq	*(%rip)                 # 0x46a <cinch_random_bytes+0x46a>
     46a:      	movq	%r15, %rdi
     46d:      	shrq	$0x20, %rdi
     471:      	callq	*0x8(%rax)
     474:      	jmp	0x47d <cinch_random_bytes+0x47d>
     476:      	movq	%r15, %rax
     479:      	shrq	$0x20, %rax
     47d:      	movq	$-0x7, %rbx
     484:      	cmpb	$0x15, %al
     486:      	jae	0x496 <cinch_random_bytes+0x496>
     488:      	movzbl	%al, %eax
     48b:      	leaq	(%rip), %rcx            # 0x492 <cinch_random_bytes+0x492>
     492:      	movq	(%rcx,%rax,8), %rbx
     496:      	leal	-0x2(%r14), %eax
     49a:      	cmpl	$0x2, %eax
     49d:      	jb	0x4bc <cinch_random_bytes+0x4bc>
     49f:      	testq	%r14, %r14
     4a2:      	je	0x4bc <cinch_random_bytes+0x4bc>
     4a4:      	decq	%r15
     4a7:      	leaq	0x18(%rsp), %rdi
     4ac:      	movq	%r15, 0x18(%rsp)
     4b1:      	movb	$0x3, 0x10(%rsp)
     4b6:      	callq	*(%rip)                 # 0x4bc <cinch_random_bytes+0x4bc>
     4bc:      	movq	0x30(%rsp), %rsi
     4c1:      	testq	%rsi, %rsi
     4c4:      	je	0x4d0 <cinch_random_bytes+0x4d0>
     4c6:      	movq	0x8(%rsp), %rdi
     4cb:      	callq	0x4d0 <cinch_random_bytes+0x4d0>
     4d0:      	movq	%rbx, %rax
     4d3:      	addq	$0x48, %rsp
     4d7:      	popq	%rbx
     4d8:      	popq	%r12
     4da:      	popq	%r13
     4dc:      	popq	%r14
     4de:      	popq	%r15
     4e0:      	popq	%rbp
     4e1:      	retq
     4e2:      	movzbl	0x1f(%r15), %eax
     4e7:      	movq	$-0x7, %rbx
     4ee:      	cmpb	$0x15, %al
     4f0:      	jb	0x488 <cinch_random_bytes+0x488>
     4f2:      	jmp	0x496 <cinch_random_bytes+0x496>
     4f4:      	movq	0x8(%rsp), %rcx
     4f9:      	jmp	0x2a3 <cinch_random_bytes+0x2a3>
     4fe:      	leaq	(%rip), %rcx            # 0x505 <cinch_random_bytes+0x505>
     505:      	movq	%r15, %rdi
     508:      	movq	%r12, %rsi
     50b:      	movq	%r12, %rdx
     50e:      	callq	*(%rip)                 # 0x514 <cinch_random_bytes+0x514>
     514:      	ud2
     516:      	jmp	0x590 <cinch_random_bytes+0x590>
     518:      	movq	%r15, %rdi
     51b:      	callq	0x520 <cinch_random_bytes+0x520>
     520:      	jmp	0x586 <cinch_random_bytes+0x586>
     522:      	callq	*(%rip)                 # 0x528 <cinch_random_bytes+0x528>
     528:      	movq	0x30(%rsp), %rdi
     52d:      	movq	0x8(%rsp), %rsi
     532:      	callq	0x537 <cinch_random_bytes+0x537>
     537:      	callq	*(%rip)                 # 0x53d <cinch_random_bytes+0x53d>
     53d:      	movq	%r15, %rdi
     540:      	callq	0x545 <cinch_random_bytes+0x545>
     545:      	jmp	0x598 <cinch_random_bytes+0x598>
     547:      	callq	*(%rip)                 # 0x54d <cinch_random_bytes+0x54d>
     54d:      	movq	%rbx, 0x8(%rsp)
     552:      	movq	0x30(%rsp), %rdi
     557:      	movq	0x8(%rsp), %rsi
     55c:      	callq	0x561 <cinch_random_bytes+0x561>
     561:      	callq	*(%rip)                 # 0x567 <cinch_random_bytes+0x567>
     567:      	callq	*(%rip)                 # 0x56d <cinch_random_bytes+0x56d>
     56d:      	jmp	0x586 <cinch_random_bytes+0x586>
     56f:      	jmp	0x590 <cinch_random_bytes+0x590>
     571:      	movq	0x30(%rsp), %rdi
     576:      	movq	0x8(%rsp), %rsi
     57b:      	callq	0x580 <cinch_random_bytes+0x580>
     580:      	callq	*(%rip)                 # 0x586 <cinch_random_bytes+0x586>
     586:      	movl	0x4(%rsp), %edi
     58a:      	callq	*(%rip)                 # 0x590 <cinch_random_bytes+0x590>
     590:      	movq	%rbp, %rdi
     593:      	callq	0x598 <cinch_random_bytes+0x598>
     598:      	movq	0x30(%rsp), %rdi
     59d:      	movq	0x8(%rsp), %rsi
     5a2:      	callq	0x5a7 <cinch_random_bytes+0x5a7>
     5a7:      	callq	*(%rip)                 # 0x5ad <cinch_random_bytes+0x5ad>
     5ad:      	callq	*(%rip)                 # 0x5b3 <cinch_random_bytes+0x5b3>

