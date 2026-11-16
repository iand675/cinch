
/workspace/dasl-stdlib-strings/build/byte-search-benchmark-candidate1/rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000024440 <Data.Bytes.Search.count-forward>:
   24440:	55                   	push   %rbp
   24441:	48 89 e5             	mov    %rsp,%rbp
   24444:	41 57                	push   %r15
   24446:	41 56                	push   %r14
   24448:	53                   	push   %rbx
   24449:	49 89 d3             	mov    %rdx,%r11
   2444c:	49 29 f3             	sub    %rsi,%r11
   2444f:	49 83 fb 08          	cmp    $0x8,%r11
   24453:	7d 3e                	jge    24493 <Data.Bytes.Search.count-forward+0x53>
   24455:	48 39 d6             	cmp    %rdx,%rsi
   24458:	0f 8d e1 01 00 00    	jge    2463f <Data.Bytes.Search.count-forward+0x1ff>
   2445e:	31 c0                	xor    %eax,%eax
   24460:	38 0c 37             	cmp    %cl,(%rdi,%rsi,1)
   24463:	0f 94 c0             	sete   %al
   24466:	49 01 c0             	add    %rax,%r8
   24469:	48 8d 46 01          	lea    0x1(%rsi),%rax
   2446d:	48 39 d0             	cmp    %rdx,%rax
   24470:	0f 8d c9 01 00 00    	jge    2463f <Data.Bytes.Search.count-forward+0x1ff>
   24476:	0f b6 c1             	movzbl %cl,%eax
   24479:	45 31 c9             	xor    %r9d,%r9d
   2447c:	38 4c 37 01          	cmp    %cl,0x1(%rdi,%rsi,1)
   24480:	41 0f 94 c1          	sete   %r9b
   24484:	4d 01 c8             	add    %r9,%r8
   24487:	48 83 c6 02          	add    $0x2,%rsi
   2448b:	48 89 c1             	mov    %rax,%rcx
   2448e:	e9 9e 01 00 00       	jmp    24631 <Data.Bytes.Search.count-forward+0x1f1>
   24493:	48 bb 7f 7f 7f 7f 7f 	movabs $0x7f7f7f7f7f7f7f7f,%rbx
   2449a:	7f 7f 7f 
   2449d:	49 be 01 01 01 01 01 	movabs $0x101010101010101,%r14
   244a4:	01 01 01 
   244a7:	4c 8d 57 18          	lea    0x18(%rdi),%r10
   244ab:	45 31 c9             	xor    %r9d,%r9d
   244ae:	66 90                	xchg   %ax,%ax
   244b0:	4d 8b 7c 32 e8       	mov    -0x18(%r10,%rsi,1),%r15
   244b5:	49 31 cf             	xor    %rcx,%r15
   244b8:	4c 89 f8             	mov    %r15,%rax
   244bb:	48 21 d8             	and    %rbx,%rax
   244be:	48 01 d8             	add    %rbx,%rax
   244c1:	4c 09 f8             	or     %r15,%rax
   244c4:	48 f7 d0             	not    %rax
   244c7:	48 c1 e8 07          	shr    $0x7,%rax
   244cb:	4c 21 f0             	and    %r14,%rax
   244ce:	49 0f af c6          	imul   %r14,%rax
   244d2:	48 c1 e8 38          	shr    $0x38,%rax
   244d6:	4c 01 c0             	add    %r8,%rax
   244d9:	4f 8d 04 0b          	lea    (%r11,%r9,1),%r8
   244dd:	49 83 c0 f8          	add    $0xfffffffffffffff8,%r8
   244e1:	49 83 f8 08          	cmp    $0x8,%r8
   244e5:	0f 8c c9 00 00 00    	jl     245b4 <Data.Bytes.Search.count-forward+0x174>
   244eb:	4d 8b 7c 32 f0       	mov    -0x10(%r10,%rsi,1),%r15
   244f0:	49 31 cf             	xor    %rcx,%r15
   244f3:	4d 89 f8             	mov    %r15,%r8
   244f6:	49 21 d8             	and    %rbx,%r8
   244f9:	49 01 d8             	add    %rbx,%r8
   244fc:	4d 09 f8             	or     %r15,%r8
   244ff:	49 f7 d0             	not    %r8
   24502:	49 c1 e8 07          	shr    $0x7,%r8
   24506:	4d 21 f0             	and    %r14,%r8
   24509:	4d 0f af c6          	imul   %r14,%r8
   2450d:	49 c1 e8 38          	shr    $0x38,%r8
   24511:	49 01 c0             	add    %rax,%r8
   24514:	4b 8d 04 0b          	lea    (%r11,%r9,1),%rax
   24518:	48 83 c0 f0          	add    $0xfffffffffffffff0,%rax
   2451c:	48 83 f8 08          	cmp    $0x8,%rax
   24520:	0f 8c d4 00 00 00    	jl     245fa <Data.Bytes.Search.count-forward+0x1ba>
   24526:	4d 8b 7c 32 f8       	mov    -0x8(%r10,%rsi,1),%r15
   2452b:	49 31 cf             	xor    %rcx,%r15
   2452e:	4c 89 f8             	mov    %r15,%rax
   24531:	48 21 d8             	and    %rbx,%rax
   24534:	48 01 d8             	add    %rbx,%rax
   24537:	4c 09 f8             	or     %r15,%rax
   2453a:	48 f7 d0             	not    %rax
   2453d:	48 c1 e8 07          	shr    $0x7,%rax
   24541:	4c 21 f0             	and    %r14,%rax
   24544:	49 0f af c6          	imul   %r14,%rax
   24548:	48 c1 e8 38          	shr    $0x38,%rax
   2454c:	4c 01 c0             	add    %r8,%rax
   2454f:	4f 8d 04 0b          	lea    (%r11,%r9,1),%r8
   24553:	49 83 c0 e8          	add    $0xffffffffffffffe8,%r8
   24557:	49 83 f8 08          	cmp    $0x8,%r8
   2455b:	0f 8c c3 00 00 00    	jl     24624 <Data.Bytes.Search.count-forward+0x1e4>
   24561:	4d 8b 3c 32          	mov    (%r10,%rsi,1),%r15
   24565:	49 31 cf             	xor    %rcx,%r15
   24568:	4d 89 f8             	mov    %r15,%r8
   2456b:	49 21 d8             	and    %rbx,%r8
   2456e:	49 01 d8             	add    %rbx,%r8
   24571:	4d 09 f8             	or     %r15,%r8
   24574:	49 f7 d0             	not    %r8
   24577:	49 c1 e8 07          	shr    $0x7,%r8
   2457b:	4d 21 f0             	and    %r14,%r8
   2457e:	4d 0f af c6          	imul   %r14,%r8
   24582:	49 c1 e8 38          	shr    $0x38,%r8
   24586:	49 01 c0             	add    %rax,%r8
   24589:	49 83 c2 20          	add    $0x20,%r10
   2458d:	4b 8d 04 0b          	lea    (%r11,%r9,1),%rax
   24591:	48 83 c0 e0          	add    $0xffffffffffffffe0,%rax
   24595:	49 83 c1 e0          	add    $0xffffffffffffffe0,%r9
   24599:	48 83 f8 08          	cmp    $0x8,%rax
   2459d:	0f 8d 0d ff ff ff    	jge    244b0 <Data.Bytes.Search.count-forward+0x70>
   245a3:	4c 29 ce             	sub    %r9,%rsi
   245a6:	48 39 d6             	cmp    %rdx,%rsi
   245a9:	0f 8c af fe ff ff    	jl     2445e <Data.Bytes.Search.count-forward+0x1e>
   245af:	e9 8b 00 00 00       	jmp    2463f <Data.Bytes.Search.count-forward+0x1ff>
   245b4:	49 89 f0             	mov    %rsi,%r8
   245b7:	4d 29 c8             	sub    %r9,%r8
   245ba:	4d 8d 48 08          	lea    0x8(%r8),%r9
   245be:	49 39 d1             	cmp    %rdx,%r9
   245c1:	7d 79                	jge    2463c <Data.Bytes.Search.count-forward+0x1fc>
   245c3:	45 31 c9             	xor    %r9d,%r9d
   245c6:	41 38 4c 32 f0       	cmp    %cl,-0x10(%r10,%rsi,1)
   245cb:	41 0f 94 c1          	sete   %r9b
   245cf:	4c 01 c8             	add    %r9,%rax
   245d2:	4d 8d 48 09          	lea    0x9(%r8),%r9
   245d6:	49 39 d1             	cmp    %rdx,%r9
   245d9:	7d 61                	jge    2463c <Data.Bytes.Search.count-forward+0x1fc>
   245db:	44 0f b6 c9          	movzbl %cl,%r9d
   245df:	45 31 db             	xor    %r11d,%r11d
   245e2:	41 38 4c 32 f1       	cmp    %cl,-0xf(%r10,%rsi,1)
   245e7:	41 0f 94 c3          	sete   %r11b
   245eb:	4c 01 d8             	add    %r11,%rax
   245ee:	49 83 c0 0a          	add    $0xa,%r8
   245f2:	4c 89 c6             	mov    %r8,%rsi
   245f5:	4c 89 c9             	mov    %r9,%rcx
   245f8:	eb 34                	jmp    2462e <Data.Bytes.Search.count-forward+0x1ee>
   245fa:	48 89 f0             	mov    %rsi,%rax
   245fd:	4c 29 c8             	sub    %r9,%rax
   24600:	4c 8d 48 10          	lea    0x10(%rax),%r9
   24604:	49 39 d1             	cmp    %rdx,%r9
   24607:	7d 36                	jge    2463f <Data.Bytes.Search.count-forward+0x1ff>
   24609:	0f b6 c9             	movzbl %cl,%ecx
   2460c:	45 31 c9             	xor    %r9d,%r9d
   2460f:	41 38 4c 32 f8       	cmp    %cl,-0x8(%r10,%rsi,1)
   24614:	41 0f 94 c1          	sete   %r9b
   24618:	4d 01 c8             	add    %r9,%r8
   2461b:	48 83 c0 11          	add    $0x11,%rax
   2461f:	48 89 c6             	mov    %rax,%rsi
   24622:	eb 0d                	jmp    24631 <Data.Bytes.Search.count-forward+0x1f1>
   24624:	0f b6 c9             	movzbl %cl,%ecx
   24627:	4c 29 ce             	sub    %r9,%rsi
   2462a:	48 83 c6 18          	add    $0x18,%rsi
   2462e:	49 89 c0             	mov    %rax,%r8
   24631:	5b                   	pop    %rbx
   24632:	41 5e                	pop    %r14
   24634:	41 5f                	pop    %r15
   24636:	5d                   	pop    %rbp
   24637:	e9 14 00 00 00       	jmp    24650 <Data.Bytes.Search.count-tail>
   2463c:	49 89 c0             	mov    %rax,%r8
   2463f:	4c 89 c0             	mov    %r8,%rax
   24642:	5b                   	pop    %rbx
   24643:	41 5e                	pop    %r14
   24645:	41 5f                	pop    %r15
   24647:	5d                   	pop    %rbp
   24648:	c3                   	ret

Disassembly of section .fini:
