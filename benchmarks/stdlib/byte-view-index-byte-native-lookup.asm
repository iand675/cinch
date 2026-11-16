
build/byte-view-index-byte-bench-native/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003e510 <cinch.typed.m.Main.lookup>:
   3e510:	41 57                	push   %r15
   3e512:	41 56                	push   %r14
   3e514:	41 55                	push   %r13
   3e516:	41 54                	push   %r12
   3e518:	53                   	push   %rbx
   3e519:	49 89 d6             	mov    %rdx,%r14
   3e51c:	48 89 f3             	mov    %rsi,%rbx
   3e51f:	49 89 ff             	mov    %rdi,%r15
   3e522:	48 89 f7             	mov    %rsi,%rdi
   3e525:	e8 36 d0 41 00       	call   45b560 <cinch_dup>
   3e52a:	48 89 df             	mov    %rbx,%rdi
   3e52d:	e8 2e d0 41 00       	call   45b560 <cinch_dup>
   3e532:	48 89 df             	mov    %rbx,%rdi
   3e535:	e8 26 d0 41 00       	call   45b560 <cinch_dup>
   3e53a:	48 8b 43 f8          	mov    -0x8(%rbx),%rax
   3e53e:	4c 8b 63 08          	mov    0x8(%rbx),%r12
   3e542:	4c 8b 6b 10          	mov    0x10(%rbx),%r13
   3e546:	4c 89 ef             	mov    %r13,%rdi
   3e549:	e8 12 d0 41 00       	call   45b560 <cinch_dup>
   3e54e:	4c 89 e7             	mov    %r12,%rdi
   3e551:	e8 0a d0 41 00       	call   45b560 <cinch_dup>
   3e556:	4c 89 ef             	mov    %r13,%rdi
   3e559:	4c 89 e6             	mov    %r12,%rsi
   3e55c:	e8 bf 43 12 00       	call   162920 <Data.Nat.Sub_Nat.->
   3e561:	49 89 c4             	mov    %rax,%r12
   3e564:	48 89 df             	mov    %rbx,%rdi
   3e567:	e8 b4 cb 41 00       	call   45b120 <cinch_drop>
   3e56c:	48 89 df             	mov    %rbx,%rdi
   3e56f:	e8 ac cb 41 00       	call   45b120 <cinch_drop>
   3e574:	be 01 00 00 00       	mov    $0x1,%esi
   3e579:	4c 89 e7             	mov    %r12,%rdi
   3e57c:	e8 2f 15 12 00       	call   15fab0 <Data.Nat.Eq_Nat.==>
   3e581:	49 89 c4             	mov    %rax,%r12
   3e584:	48 89 df             	mov    %rbx,%rdi
   3e587:	e8 94 cb 41 00       	call   45b120 <cinch_drop>
   3e58c:	4d 85 e4             	test   %r12,%r12
   3e58f:	74 0c                	je     3e59d <cinch.typed.m.Main.lookup+0x8d>
   3e591:	4c 8d 25 b0 1b 4d 00 	lea    0x4d1bb0(%rip),%r12        # 510148 <_IO_stdin_used+0x148>
   3e598:	e9 ac 00 00 00       	jmp    3e649 <cinch.typed.m.Main.lookup+0x139>
   3e59d:	4c 89 f7             	mov    %r14,%rdi
   3e5a0:	e8 bb cf 41 00       	call   45b560 <cinch_dup>
   3e5a5:	4d 85 ff             	test   %r15,%r15
   3e5a8:	74 46                	je     3e5f0 <cinch.typed.m.Main.lookup+0xe0>
   3e5aa:	4c 89 f7             	mov    %r14,%rdi
   3e5ad:	e8 0e 1b 00 00       	call   400c0 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dsnapshot>
   3e5b2:	49 89 c7             	mov    %rax,%r15
   3e5b5:	48 89 c7             	mov    %rax,%rdi
   3e5b8:	e8 a3 cf 41 00       	call   45b560 <cinch_dup>
   3e5bd:	4c 89 ff             	mov    %r15,%rdi
   3e5c0:	e8 2b 10 00 00       	call   3f5f0 <cinch.typed.m.Data$2eBytes$2eView.bytes$2dcompile$2dpattern>
   3e5c5:	49 89 c5             	mov    %rax,%r13
   3e5c8:	48 89 df             	mov    %rbx,%rdi
   3e5cb:	e8 90 cf 41 00       	call   45b560 <cinch_dup>
   3e5d0:	4c 89 ef             	mov    %r13,%rdi
   3e5d3:	e8 88 cf 41 00       	call   45b560 <cinch_dup>
   3e5d8:	48 89 df             	mov    %rbx,%rdi
   3e5db:	4c 89 ee             	mov    %r13,%rsi
   3e5de:	e8 0d 07 00 00       	call   3ecf0 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dpattern>
   3e5e3:	49 89 c4             	mov    %rax,%r12
   3e5e6:	4c 89 ef             	mov    %r13,%rdi
   3e5e9:	e8 32 cb 41 00       	call   45b120 <cinch_drop>
   3e5ee:	eb 51                	jmp    3e641 <cinch.typed.m.Main.lookup+0x131>
   3e5f0:	be 01 00 00 00       	mov    $0x1,%esi
   3e5f5:	4c 89 f7             	mov    %r14,%rdi
   3e5f8:	e8 43 04 00 00       	call   3ea40 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dget>
   3e5fd:	49 89 c7             	mov    %rax,%r15
   3e600:	48 8b 40 f8          	mov    -0x8(%rax),%rax
   3e604:	48 c1 e8 28          	shr    $0x28,%rax
   3e608:	0f b6 c0             	movzbl %al,%eax
   3e60b:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   3e611:	75 07                	jne    3e61a <cinch.typed.m.Main.lookup+0x10a>
   3e613:	49 8b 47 f0          	mov    -0x10(%r15),%rax
   3e617:	48 8b 00             	mov    (%rax),%rax
   3e61a:	48 85 c0             	test   %rax,%rax
   3e61d:	74 1b                	je     3e63a <cinch.typed.m.Main.lookup+0x12a>
   3e61f:	4d 8b 27             	mov    (%r15),%r12
   3e622:	48 89 df             	mov    %rbx,%rdi
   3e625:	e8 36 cf 41 00       	call   45b560 <cinch_dup>
   3e62a:	48 89 df             	mov    %rbx,%rdi
   3e62d:	4c 89 e6             	mov    %r12,%rsi
   3e630:	e8 3b 00 00 00       	call   3e670 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte>
   3e635:	49 89 c4             	mov    %rax,%r12
   3e638:	eb 07                	jmp    3e641 <cinch.typed.m.Main.lookup+0x131>
   3e63a:	4c 8d 25 07 1b 4d 00 	lea    0x4d1b07(%rip),%r12        # 510148 <_IO_stdin_used+0x148>
   3e641:	4c 89 ff             	mov    %r15,%rdi
   3e644:	e8 d7 ca 41 00       	call   45b120 <cinch_drop>
   3e649:	4c 89 f7             	mov    %r14,%rdi
   3e64c:	e8 cf ca 41 00       	call   45b120 <cinch_drop>
   3e651:	48 89 df             	mov    %rbx,%rdi
   3e654:	e8 c7 ca 41 00       	call   45b120 <cinch_drop>
   3e659:	4c 89 e0             	mov    %r12,%rax
   3e65c:	5b                   	pop    %rbx
   3e65d:	41 5c                	pop    %r12
   3e65f:	41 5d                	pop    %r13
   3e661:	41 5e                	pop    %r14
   3e663:	41 5f                	pop    %r15
   3e665:	c3                   	ret

Disassembly of section .fini:
