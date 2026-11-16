
/workspace/dasl-stdlib-strings/build/string-scalars-benchmark-phase1/after/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000283f0 <Data.String.string-count-from>:
   283f0:	55                   	push   %rbp
   283f1:	48 89 e5             	mov    %rsp,%rbp
   283f4:	53                   	push   %rbx
   283f5:	49 89 d3             	mov    %rdx,%r11
   283f8:	49 29 f3             	sub    %rsi,%r11
   283fb:	49 83 fb 10          	cmp    $0x10,%r11
   283ff:	0f 8c e6 00 00 00    	jl     284eb <Data.String.string-count-from+0xfb>
   28405:	49 ba 01 01 01 01 01 	movabs $0x101010101010101,%r10
   2840c:	01 01 01 
   2840f:	4c 8d 4f 20          	lea    0x20(%rdi),%r9
   28413:	31 db                	xor    %ebx,%ebx
   28415:	66 0f 6f 05 93 4c 0f 	movdqa 0xf4c93(%rip),%xmm0        # 11d0b0 <_IO_stdin_used+0xb0>
   2841c:	00 
   2841d:	66 0f 6f 0d 9b 4c 0f 	movdqa 0xf4c9b(%rip),%xmm1        # 11d0c0 <_IO_stdin_used+0xc0>
   28424:	00 
   28425:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2842c:	00 00 00 00 
   28430:	f3 41 0f 6f 54 31 e0 	movdqu -0x20(%r9,%rsi,1),%xmm2
   28437:	66 0f 64 d0          	pcmpgtb %xmm0,%xmm2
   2843b:	66 0f db d1          	pand   %xmm1,%xmm2
   2843f:	66 0f 70 da ee       	pshufd $0xee,%xmm2,%xmm3
   28444:	66 0f d4 da          	paddq  %xmm2,%xmm3
   28448:	66 49 0f 7e d8       	movq   %xmm3,%r8
   2844d:	4d 0f af c2          	imul   %r10,%r8
   28451:	49 c1 e8 38          	shr    $0x38,%r8
   28455:	49 01 c8             	add    %rcx,%r8
   28458:	49 8d 04 1b          	lea    (%r11,%rbx,1),%rax
   2845c:	48 83 c0 f0          	add    $0xfffffffffffffff0,%rax
   28460:	48 83 f8 10          	cmp    $0x10,%rax
   28464:	0f 8c 20 01 00 00    	jl     2858a <Data.String.string-count-from+0x19a>
   2846a:	f3 41 0f 6f 54 31 f0 	movdqu -0x10(%r9,%rsi,1),%xmm2
   28471:	66 0f 64 d0          	pcmpgtb %xmm0,%xmm2
   28475:	66 0f db d1          	pand   %xmm1,%xmm2
   28479:	66 0f 70 da ee       	pshufd $0xee,%xmm2,%xmm3
   2847e:	66 0f d4 da          	paddq  %xmm2,%xmm3
   28482:	66 48 0f 7e d8       	movq   %xmm3,%rax
   28487:	49 0f af c2          	imul   %r10,%rax
   2848b:	48 c1 e8 38          	shr    $0x38,%rax
   2848f:	4c 01 c0             	add    %r8,%rax
   28492:	49 8d 0c 1b          	lea    (%r11,%rbx,1),%rcx
   28496:	48 83 c1 e0          	add    $0xffffffffffffffe0,%rcx
   2849a:	48 83 f9 10          	cmp    $0x10,%rcx
   2849e:	0f 8c 1c 01 00 00    	jl     285c0 <Data.String.string-count-from+0x1d0>
   284a4:	f3 41 0f 6f 14 31    	movdqu (%r9,%rsi,1),%xmm2
   284aa:	66 0f 64 d0          	pcmpgtb %xmm0,%xmm2
   284ae:	66 0f db d1          	pand   %xmm1,%xmm2
   284b2:	66 0f 70 da ee       	pshufd $0xee,%xmm2,%xmm3
   284b7:	66 0f d4 da          	paddq  %xmm2,%xmm3
   284bb:	66 48 0f 7e d9       	movq   %xmm3,%rcx
   284c0:	49 0f af ca          	imul   %r10,%rcx
   284c4:	48 c1 e9 38          	shr    $0x38,%rcx
   284c8:	48 01 c1             	add    %rax,%rcx
   284cb:	49 83 c1 30          	add    $0x30,%r9
   284cf:	49 8d 04 1b          	lea    (%r11,%rbx,1),%rax
   284d3:	48 83 c0 d0          	add    $0xffffffffffffffd0,%rax
   284d7:	48 83 c3 d0          	add    $0xffffffffffffffd0,%rbx
   284db:	48 83 f8 10          	cmp    $0x10,%rax
   284df:	0f 8d 4b ff ff ff    	jge    28430 <Data.String.string-count-from+0x40>
   284e5:	48 29 de             	sub    %rbx,%rsi
   284e8:	49 01 db             	add    %rbx,%r11
   284eb:	49 83 fb 08          	cmp    $0x8,%r11
   284ef:	7d 34                	jge    28525 <Data.String.string-count-from+0x135>
   284f1:	48 39 d6             	cmp    %rdx,%rsi
   284f4:	0f 8d 8a 00 00 00    	jge    28584 <Data.String.string-count-from+0x194>
   284fa:	31 c0                	xor    %eax,%eax
   284fc:	80 3c 37 c0          	cmpb   $0xc0,(%rdi,%rsi,1)
   28500:	0f 9d c0             	setge  %al
   28503:	48 01 c1             	add    %rax,%rcx
   28506:	48 8d 46 01          	lea    0x1(%rsi),%rax
   2850a:	48 39 d0             	cmp    %rdx,%rax
   2850d:	7d 75                	jge    28584 <Data.String.string-count-from+0x194>
   2850f:	31 c0                	xor    %eax,%eax
   28511:	80 7c 37 01 c0       	cmpb   $0xc0,0x1(%rdi,%rsi,1)
   28516:	0f 9d c0             	setge  %al
   28519:	48 01 c1             	add    %rax,%rcx
   2851c:	48 83 c6 02          	add    $0x2,%rsi
   28520:	e9 1d 01 00 00       	jmp    28642 <Data.String.string-count-from+0x252>
   28525:	48 8b 04 37          	mov    (%rdi,%rsi,1),%rax
   28529:	4c 8d 04 00          	lea    (%rax,%rax,1),%r8
   2852d:	48 f7 d0             	not    %rax
   28530:	4c 09 c0             	or     %r8,%rax
   28533:	48 c1 e8 07          	shr    $0x7,%rax
   28537:	49 b8 01 01 01 01 01 	movabs $0x101010101010101,%r8
   2853e:	01 01 01 
   28541:	4c 21 c0             	and    %r8,%rax
   28544:	49 0f af c0          	imul   %r8,%rax
   28548:	48 c1 e8 38          	shr    $0x38,%rax
   2854c:	48 01 c8             	add    %rcx,%rax
   2854f:	48 8d 4e 08          	lea    0x8(%rsi),%rcx
   28553:	48 39 d1             	cmp    %rdx,%rcx
   28556:	7d 2f                	jge    28587 <Data.String.string-count-from+0x197>
   28558:	31 c9                	xor    %ecx,%ecx
   2855a:	80 7c 37 08 c0       	cmpb   $0xc0,0x8(%rdi,%rsi,1)
   2855f:	0f 9d c1             	setge  %cl
   28562:	48 01 c8             	add    %rcx,%rax
   28565:	48 8d 4e 09          	lea    0x9(%rsi),%rcx
   28569:	48 39 d1             	cmp    %rdx,%rcx
   2856c:	7d 19                	jge    28587 <Data.String.string-count-from+0x197>
   2856e:	31 c9                	xor    %ecx,%ecx
   28570:	80 7c 37 09 c0       	cmpb   $0xc0,0x9(%rdi,%rsi,1)
   28575:	0f 9d c1             	setge  %cl
   28578:	48 01 c8             	add    %rcx,%rax
   2857b:	48 83 c6 0a          	add    $0xa,%rsi
   2857f:	e9 92 00 00 00       	jmp    28616 <Data.String.string-count-from+0x226>
   28584:	48 89 c8             	mov    %rcx,%rax
   28587:	5b                   	pop    %rbx
   28588:	5d                   	pop    %rbp
   28589:	c3                   	ret
   2858a:	48 89 f1             	mov    %rsi,%rcx
   2858d:	48 29 d9             	sub    %rbx,%rcx
   28590:	48 83 f8 08          	cmp    $0x8,%rax
   28594:	7d 3f                	jge    285d5 <Data.String.string-count-from+0x1e5>
   28596:	48 8d 41 10          	lea    0x10(%rcx),%rax
   2859a:	48 39 d0             	cmp    %rdx,%rax
   2859d:	0f 8d a6 00 00 00    	jge    28649 <Data.String.string-count-from+0x259>
   285a3:	31 c0                	xor    %eax,%eax
   285a5:	41 80 7c 31 f0 c0    	cmpb   $0xc0,-0x10(%r9,%rsi,1)
   285ab:	0f 9d c0             	setge  %al
   285ae:	49 01 c0             	add    %rax,%r8
   285b1:	48 83 c1 11          	add    $0x11,%rcx
   285b5:	48 89 ce             	mov    %rcx,%rsi
   285b8:	4c 89 c1             	mov    %r8,%rcx
   285bb:	e9 82 00 00 00       	jmp    28642 <Data.String.string-count-from+0x252>
   285c0:	49 89 f0             	mov    %rsi,%r8
   285c3:	49 29 d8             	sub    %rbx,%r8
   285c6:	48 83 f9 08          	cmp    $0x8,%rcx
   285ca:	7d 4f                	jge    2861b <Data.String.string-count-from+0x22b>
   285cc:	49 83 c0 20          	add    $0x20,%r8
   285d0:	4c 89 c6             	mov    %r8,%rsi
   285d3:	eb 41                	jmp    28616 <Data.String.string-count-from+0x226>
   285d5:	49 8b 44 31 f0       	mov    -0x10(%r9,%rsi,1),%rax
   285da:	4c 8d 1c 00          	lea    (%rax,%rax,1),%r11
   285de:	48 f7 d0             	not    %rax
   285e1:	4c 09 d8             	or     %r11,%rax
   285e4:	48 c1 e8 07          	shr    $0x7,%rax
   285e8:	4c 21 d0             	and    %r10,%rax
   285eb:	49 0f af c2          	imul   %r10,%rax
   285ef:	48 c1 e8 38          	shr    $0x38,%rax
   285f3:	4c 01 c0             	add    %r8,%rax
   285f6:	4c 8d 41 18          	lea    0x18(%rcx),%r8
   285fa:	49 39 d0             	cmp    %rdx,%r8
   285fd:	7d 88                	jge    28587 <Data.String.string-count-from+0x197>
   285ff:	45 31 c0             	xor    %r8d,%r8d
   28602:	41 80 7c 31 f8 c0    	cmpb   $0xc0,-0x8(%r9,%rsi,1)
   28608:	41 0f 9d c0          	setge  %r8b
   2860c:	4c 01 c0             	add    %r8,%rax
   2860f:	48 83 c1 19          	add    $0x19,%rcx
   28613:	48 89 ce             	mov    %rcx,%rsi
   28616:	48 89 c1             	mov    %rax,%rcx
   28619:	eb 27                	jmp    28642 <Data.String.string-count-from+0x252>
   2861b:	49 8b 0c 31          	mov    (%r9,%rsi,1),%rcx
   2861f:	48 8d 34 09          	lea    (%rcx,%rcx,1),%rsi
   28623:	48 f7 d1             	not    %rcx
   28626:	48 09 f1             	or     %rsi,%rcx
   28629:	48 c1 e9 07          	shr    $0x7,%rcx
   2862d:	4c 21 d1             	and    %r10,%rcx
   28630:	49 0f af ca          	imul   %r10,%rcx
   28634:	48 c1 e9 38          	shr    $0x38,%rcx
   28638:	48 01 c1             	add    %rax,%rcx
   2863b:	49 83 c0 28          	add    $0x28,%r8
   2863f:	4c 89 c6             	mov    %r8,%rsi
   28642:	5b                   	pop    %rbx
   28643:	5d                   	pop    %rbp
   28644:	e9 17 00 00 00       	jmp    28660 <Data.String.string-count-tail>
   28649:	4c 89 c0             	mov    %r8,%rax
   2864c:	e9 36 ff ff ff       	jmp    28587 <Data.String.string-count-from+0x197>

Disassembly of section .fini:
