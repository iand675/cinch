0000000000000000 <cinch_random_bytes>:
       0:      	pushq	%rbp
       1:      	pushq	%r15
       3:      	pushq	%r14
       5:      	pushq	%r13
       7:      	pushq	%r12
       9:      	pushq	%rbx
       a:      	subq	$0x48, %rsp
       e:      	testq	%rdi, %rdi
      11:      	js	0x30 <cinch_random_bytes+0x30>
      13:      	movq	%rdi, %rbp
      16:      	je	0x3c <cinch_random_bytes+0x3c>
      18:      	callq	*(%rip)                 # 0x1e <cinch_random_bytes+0x1e>
      1e:      	movl	$0x1, %esi
      23:      	movq	%rbp, %rdi
      26:      	callq	0x2b <cinch_random_bytes+0x2b>
      2b:      	movq	%rax, %r14
      2e:      	jmp	0x42 <cinch_random_bytes+0x42>
      30:      	movq	$-0x6, %rax
      37:      	jmp	0x2b5 <cinch_random_bytes+0x2b5>
      3c:      	movl	$0x1, %r14d
      42:      	movabsq	$0x1b600000000, %rax    # imm = 0x1B600000000
      4c:      	movq	%rax, 0x20(%rsp)
      51:      	movq	$0x1, 0x28(%rsp)
      5a:      	leaq	(%rip), %rdx            # 0x61 <cinch_random_bytes+0x61>
      61:      	leaq	0x38(%rsp), %rdi
      66:      	leaq	0x20(%rsp), %rsi
      6b:      	movl	$0xc, %ecx
      70:      	callq	*(%rip)                 # 0x76 <cinch_random_bytes+0x76>
      76:      	cmpl	$0x1, 0x38(%rsp)
      7b:      	jne	0x98 <cinch_random_bytes+0x98>
      7d:      	movq	0x40(%rsp), %r15
      82:      	movl	%r15d, %ebx
      85:      	andl	$0x3, %ebx
      88:      	leaq	(%rip), %rax            # 0x8f <cinch_random_bytes+0x8f>
      8f:      	movslq	(%rax,%rbx,4), %rcx
      93:      	addq	%rax, %rcx
      96:      	jmpq	*%rcx
      98:      	movl	0x3c(%rsp), %edi
      9c:      	movl	%edi, 0xc(%rsp)
      a0:      	testq	%rbp, %rbp
      a3:      	je	0x19d <cinch_random_bytes+0x19d>
      a9:      	leaq	0xc(%rsp), %r13
      ae:      	movq	(%rip), %r12            # 0xb5 <cinch_random_bytes+0xb5>
      b5:      	leaq	(%rip), %rbx            # 0xbc <cinch_random_bytes+0xbc>
      bc:      	movq	%r14, 0x10(%rsp)
      c1:      	movq	%rbp, 0x18(%rsp)
      c6:      	jmp	0xf9 <cinch_random_bytes+0xf9>
      c8:      	movq	%r15, %rax
      cb:      	shrq	$0x20, %rax
      cf:      	movabsq	$0x2c00000000, %rcx     # imm = 0x2C00000000
      d9:      	cmpq	%rcx, %r15
      dc:      	movl	$0xff, %ecx
      e1:      	cmovael	%ecx, %eax
      e4:      	cmpb	$0x23, %al
      e6:      	jne	0x1f6 <cinch_random_bytes+0x1f6>
      ec:      	nopl	(%rax)
      f0:      	testq	%rbp, %rbp
      f3:      	je	0x18f <cinch_random_bytes+0x18f>
      f9:      	movq	%r13, %rdi
      fc:      	movq	%r14, %rsi
      ff:      	movq	%rbp, %rdx
     102:      	callq	*%r12
     105:      	movq	%rdx, %r15
     108:      	cmpq	$0x1, %rax
     10c:      	jne	0x130 <cinch_random_bytes+0x130>
     10e:      	movl	%r15d, %eax
     111:      	andl	$0x3, %eax
     114:      	movslq	(%rbx,%rax,4), %rax
     118:      	addq	%rbx, %rax
     11b:      	jmpq	*%rax
     11d:      	cmpb	$0x23, 0x10(%r15)
     122:      	je	0xf0 <cinch_random_bytes+0xf0>
     124:      	jmp	0x1f6 <cinch_random_bytes+0x1f6>
     129:      	nopl	(%rax)
     130:      	testq	%r15, %r15
     133:      	je	0x1ef <cinch_random_bytes+0x1ef>
     139:      	movq	%rbp, %rax
     13c:      	subq	%r15, %rax
     13f:      	jb	0x2d7 <cinch_random_bytes+0x2d7>
     145:      	addq	%r15, %r14
     148:      	movq	%rax, %rbp
     14b:      	jmp	0xf0 <cinch_random_bytes+0xf0>
     14d:      	callq	*(%rip)                 # 0x153 <cinch_random_bytes+0x153>
     153:      	movq	%r15, %rdi
     156:      	shrq	$0x20, %rdi
     15a:      	callq	*0x10(%rax)
     15d:      	testb	%al, %al
     15f:      	jne	0xf0 <cinch_random_bytes+0xf0>
     161:      	jmp	0x1f6 <cinch_random_bytes+0x1f6>
     166:      	cmpb	$0x23, 0x1f(%r15)
     16b:      	jne	0x1f6 <cinch_random_bytes+0x1f6>
     171:      	leaq	-0x1(%r15), %rax
     175:      	movq	%rax, 0x28(%rsp)
     17a:      	movb	$0x3, 0x20(%rsp)
     17f:      	leaq	0x28(%rsp), %rdi
     184:      	callq	*(%rip)                 # 0x18a <cinch_random_bytes+0x18a>
     18a:      	jmp	0xf0 <cinch_random_bytes+0xf0>
     18f:      	movl	0xc(%rsp), %edi
     193:      	movq	0x18(%rsp), %rbp
     198:      	movq	0x10(%rsp), %r14
     19d:      	callq	*(%rip)                 # 0x1a3 <cinch_random_bytes+0x1a3>
     1a3:      	movq	%rbp, 0x20(%rsp)
     1a8:      	movq	%r14, 0x28(%rsp)
     1ad:      	movq	%rbp, 0x30(%rsp)
     1b2:      	leaq	0x20(%rsp), %rdi
     1b7:      	callq	0x1bc <cinch_random_bytes+0x1bc>
     1bc:      	movq	%rax, %rbx
     1bf:      	movq	%rdx, %r14
     1c2:      	movabsq	$0x24000000001, %rsi    # imm = 0x24000000001
     1cc:      	movl	$0x10, %edi
     1d1:      	movl	$0x10, %ecx
     1d6:      	xorl	%edx, %edx
     1d8:      	xorl	%r8d, %r8d
     1db:      	xorl	%r9d, %r9d
     1de:      	callq	0x1e3 <cinch_random_bytes+0x1e3>
     1e3:      	movq	%rbx, (%rax)
     1e6:      	movq	%r14, 0x8(%rax)
     1ea:      	jmp	0x2b5 <cinch_random_bytes+0x2b5>
     1ef:      	leaq	(%rip), %r15            # 0x1f6 <cinch_random_bytes+0x1f6>
     1f6:      	movl	0xc(%rsp), %edi
     1fa:      	callq	*(%rip)                 # 0x200 <cinch_random_bytes+0x200>
     200:      	movl	%r15d, %ebx
     203:      	andl	$0x3, %ebx
     206:      	leaq	(%rip), %rax            # 0x20d <cinch_random_bytes+0x20d>
     20d:      	movslq	(%rax,%rbx,4), %rcx
     211:      	addq	%rax, %rcx
     214:      	movq	0x18(%rsp), %rbp
     219:      	movq	0x10(%rsp), %r14
     21e:      	jmpq	*%rcx
     220:      	movzbl	0x10(%r15), %ecx
     225:      	movq	$-0x7, %rax
     22c:      	cmpb	$0x15, %cl
     22f:      	jb	0x266 <cinch_random_bytes+0x266>
     231:      	jmp	0x274 <cinch_random_bytes+0x274>
     233:      	callq	*(%rip)                 # 0x239 <cinch_random_bytes+0x239>
     239:      	movq	%r15, %rdi
     23c:      	shrq	$0x20, %rdi
     240:      	callq	*0x8(%rax)
     243:      	movl	%eax, %ecx
     245:      	movq	$-0x7, %rax
     24c:      	cmpb	$0x15, %cl
     24f:      	jb	0x266 <cinch_random_bytes+0x266>
     251:      	jmp	0x274 <cinch_random_bytes+0x274>
     253:      	movq	%r15, %rcx
     256:      	shrq	$0x20, %rcx
     25a:      	movq	$-0x7, %rax
     261:      	cmpb	$0x15, %cl
     264:      	jae	0x274 <cinch_random_bytes+0x274>
     266:      	movzbl	%cl, %eax
     269:      	leaq	(%rip), %rcx            # 0x270 <cinch_random_bytes+0x270>
     270:      	movq	(%rcx,%rax,8), %rax
     274:      	leal	-0x2(%rbx), %ecx
     277:      	cmpl	$0x2, %ecx
     27a:      	jb	0x29f <cinch_random_bytes+0x29f>
     27c:      	testq	%rbx, %rbx
     27f:      	je	0x29f <cinch_random_bytes+0x29f>
     281:      	decq	%r15
     284:      	leaq	0x28(%rsp), %rdi
     289:      	movq	%r15, 0x28(%rsp)
     28e:      	movb	$0x3, 0x20(%rsp)
     293:      	movq	%rax, %rbx
     296:      	callq	*(%rip)                 # 0x29c <cinch_random_bytes+0x29c>
     29c:      	movq	%rbx, %rax
     29f:      	testq	%rbp, %rbp
     2a2:      	je	0x2b5 <cinch_random_bytes+0x2b5>
     2a4:      	movq	%r14, %rdi
     2a7:      	movq	%rbp, %rsi
     2aa:      	movq	%rax, %rbx
     2ad:      	callq	0x2b2 <cinch_random_bytes+0x2b2>
     2b2:      	movq	%rbx, %rax
     2b5:      	addq	$0x48, %rsp
     2b9:      	popq	%rbx
     2ba:      	popq	%r12
     2bc:      	popq	%r13
     2be:      	popq	%r14
     2c0:      	popq	%r15
     2c2:      	popq	%rbp
     2c3:      	retq
     2c4:      	movzbl	0x1f(%r15), %ecx
     2c9:      	movq	$-0x7, %rax
     2d0:      	cmpb	$0x15, %cl
     2d3:      	jb	0x266 <cinch_random_bytes+0x266>
     2d5:      	jmp	0x274 <cinch_random_bytes+0x274>
     2d7:      	leaq	(%rip), %rcx            # 0x2de <cinch_random_bytes+0x2de>
     2de:      	movq	%r15, %rdi
     2e1:      	movq	%rbp, %rsi
     2e4:      	movq	%rbp, %rdx
     2e7:      	callq	*(%rip)                 # 0x2ed <cinch_random_bytes+0x2ed>
     2ed:      	ud2
     2ef:      	movq	%rbp, %rdi
     2f2:      	movq	%r14, %rsi
     2f5:      	callq	0x2fa <cinch_random_bytes+0x2fa>
     2fa:      	callq	*(%rip)                 # 0x300 <cinch_random_bytes+0x300>
     300:      	movq	%r15, %rdi
     303:      	callq	0x308 <cinch_random_bytes+0x308>
     308:      	jmp	0x357 <cinch_random_bytes+0x357>
     30a:      	callq	*(%rip)                 # 0x310 <cinch_random_bytes+0x310>
     310:      	callq	*(%rip)                 # 0x316 <cinch_random_bytes+0x316>
     316:      	movq	%rbp, %rdi
     319:      	movq	%r14, %rsi
     31c:      	callq	0x321 <cinch_random_bytes+0x321>
     321:      	callq	*(%rip)                 # 0x327 <cinch_random_bytes+0x327>
     327:      	movq	%r15, %rdi
     32a:      	callq	0x32f <cinch_random_bytes+0x32f>
     32f:      	movq	0x18(%rsp), %rbp
     334:      	movq	0x10(%rsp), %r14
     339:      	jmp	0x34d <cinch_random_bytes+0x34d>
     33b:      	callq	*(%rip)                 # 0x341 <cinch_random_bytes+0x341>
     341:      	jmp	0x343 <cinch_random_bytes+0x343>
     343:      	movq	0x18(%rsp), %rbp
     348:      	movq	0x10(%rsp), %r14
     34d:      	movl	0xc(%rsp), %edi
     351:      	callq	*(%rip)                 # 0x357 <cinch_random_bytes+0x357>
     357:      	movq	%rbp, %rdi
     35a:      	movq	%r14, %rsi
     35d:      	callq	0x362 <cinch_random_bytes+0x362>
     362:      	callq	*(%rip)                 # 0x368 <cinch_random_bytes+0x368>

