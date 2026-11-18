# Retained diagnostic assembly; no library source change.

build/split-progress-regression/nat-cmp-readonly-treatment/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000025480 <Data.Bytes.Split.progress-roundtrips>:
   25480:	55                   	push   %rbp
   25481:	48 89 e5             	mov    %rsp,%rbp
   25484:	41 57                	push   %r15
   25486:	41 56                	push   %r14
   25488:	41 55                	push   %r13
   2548a:	41 54                	push   %r12
   2548c:	53                   	push   %rbx
   2548d:	48 83 ec 38          	sub    $0x38,%rsp
   25491:	49 89 ff             	mov    %rdi,%r15
   25494:	48 b8 f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rax
   2549b:	ff ff 7f
   2549e:	49 bc 00 00 00 00 f0 	movabs $0xf000000000,%r12
   254a5:	00 00 00
   254a8:	48 83 c0 07          	add    $0x7,%rax
   254ac:	48 89 45 b0          	mov    %rax,-0x50(%rbp)
   254b0:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   254b7:	00
   254b8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   254bf:	00
   254c0:	48 8b 46 f8          	mov    -0x8(%rsi),%rax
   254c4:	48 c1 e8 28          	shr    $0x28,%rax
   254c8:	0f b6 c0             	movzbl %al,%eax
   254cb:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   254d1:	75 07                	jne    254da <Data.Bytes.Split.progress-roundtrips+0x5a>
   254d3:	48 8b 46 f0          	mov    -0x10(%rsi),%rax
   254d7:	48 8b 00             	mov    (%rax),%rax
   254da:	48 83 f8 01          	cmp    $0x1,%rax
   254de:	0f 85 82 02 00 00    	jne    25766 <Data.Bytes.Split.progress-roundtrips+0x2e6>
   254e4:	4c 8b 36             	mov    (%rsi),%r14
   254e7:	48 8b 76 08          	mov    0x8(%rsi),%rsi
   254eb:	48 89 75 a8          	mov    %rsi,-0x58(%rbp)
   254ef:	4c 89 f7             	mov    %r14,%rdi
   254f2:	e8 89 02 04 00       	call   65780 <cinch_dup>
   254f7:	be 01 00 00 00       	mov    $0x1,%esi
   254fc:	4c 89 ff             	mov    %r15,%rdi
   254ff:	4c 89 f2             	mov    %r14,%rdx
   25502:	e8 89 f2 ff ff       	call   24790 <Data.Bytes.Split.store-count>
   25507:	4c 89 f7             	mov    %r14,%rdi
   2550a:	e8 71 02 04 00       	call   65780 <cinch_dup>
   2550f:	be 11 00 00 00       	mov    $0x11,%esi
   25514:	4c 89 ff             	mov    %r15,%rdi
   25517:	4c 89 f2             	mov    %r14,%rdx
   2551a:	e8 71 f2 ff ff       	call   24790 <Data.Bytes.Split.store-count>
   2551f:	be 01 00 00 00       	mov    $0x1,%esi
   25524:	4c 89 ff             	mov    %r15,%rdi
   25527:	e8 b4 fd ff ff       	call   252e0 <Data.Bytes.Split.read-count>
   2552c:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   25530:	be 11 00 00 00       	mov    $0x11,%esi
   25535:	4c 89 ff             	mov    %r15,%rdi
   25538:	e8 a3 fd ff ff       	call   252e0 <Data.Bytes.Split.read-count>
   2553d:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   25541:	4d 85 ff             	test   %r15,%r15
   25544:	74 2a                	je     25570 <Data.Bytes.Split.progress-roundtrips+0xf0>
   25546:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   2554a:	4c 21 e0             	and    %r12,%rax
   2554d:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   25554:	00 00 00
   25557:	48 39 c8             	cmp    %rcx,%rax
   2555a:	75 14                	jne    25570 <Data.Bytes.Split.progress-roundtrips+0xf0>
   2555c:	4d 8b 2f             	mov    (%r15),%r13
   2555f:	eb 1a                	jmp    2557b <Data.Bytes.Split.progress-roundtrips+0xfb>
   25561:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25568:	0f 1f 84 00 00 00 00
   2556f:	00
   25570:	4c 89 ff             	mov    %r15,%rdi
   25573:	e8 38 f1 03 00       	call   646b0 <cinch_bytes_len>
   25578:	49 89 c5             	mov    %rax,%r13
   2557b:	4a 8d 1c 6d 01 00 00 	lea    0x1(,%r13,2),%rbx
   25582:	00
   25583:	4c 23 6d b0          	and    -0x50(%rbp),%r13
   25587:	bf 21 00 00 00       	mov    $0x21,%edi
   2558c:	e8 af fd 03 00       	call   65340 <cinch_drop>
   25591:	48 89 df             	mov    %rbx,%rdi
   25594:	e8 a7 fd 03 00       	call   65340 <cinch_drop>
   25599:	b8 01 00 00 00       	mov    $0x1,%eax
   2559e:	49 83 fd 11          	cmp    $0x11,%r13
   255a2:	72 41                	jb     255e5 <Data.Bytes.Split.progress-roundtrips+0x165>
   255a4:	4d 85 ff             	test   %r15,%r15
   255a7:	74 27                	je     255d0 <Data.Bytes.Split.progress-roundtrips+0x150>
   255a9:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   255ad:	4c 21 e0             	and    %r12,%rax
   255b0:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   255b7:	00 00 00
   255ba:	48 39 c8             	cmp    %rcx,%rax
   255bd:	75 11                	jne    255d0 <Data.Bytes.Split.progress-roundtrips+0x150>
   255bf:	49 83 3f 11          	cmpq   $0x11,(%r15)
   255c3:	72 0b                	jb     255d0 <Data.Bytes.Split.progress-roundtrips+0x150>
   255c5:	49 8b 47 08          	mov    0x8(%r15),%rax
   255c9:	0f b6 40 10          	movzbl 0x10(%rax),%eax
   255cd:	eb 0e                	jmp    255dd <Data.Bytes.Split.progress-roundtrips+0x15d>
   255cf:	90                   	nop
   255d0:	be 10 00 00 00       	mov    $0x10,%esi
   255d5:	4c 89 ff             	mov    %r15,%rdi
   255d8:	e8 83 f0 03 00       	call   64660 <cinch_bytes_get>
   255dd:	48 8d 04 45 01 00 00 	lea    0x1(,%rax,2),%rax
   255e4:	00
   255e5:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   255e9:	4d 85 ff             	test   %r15,%r15
   255ec:	74 22                	je     25610 <Data.Bytes.Split.progress-roundtrips+0x190>
   255ee:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   255f2:	4c 21 e0             	and    %r12,%rax
   255f5:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   255fc:	00 00 00
   255ff:	48 39 c8             	cmp    %rcx,%rax
   25602:	75 0c                	jne    25610 <Data.Bytes.Split.progress-roundtrips+0x190>
   25604:	49 8b 1f             	mov    (%r15),%rbx
   25607:	eb 12                	jmp    2561b <Data.Bytes.Split.progress-roundtrips+0x19b>
   25609:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25610:	4c 89 ff             	mov    %r15,%rdi
   25613:	e8 98 f0 03 00       	call   646b0 <cinch_bytes_len>
   25618:	48 89 c3             	mov    %rax,%rbx
   2561b:	4c 8d 2c 5d 01 00 00 	lea    0x1(,%rbx,2),%r13
   25622:	00
   25623:	48 b8 f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rax
   2562a:	ff ff 7f
   2562d:	48 21 c3             	and    %rax,%rbx
   25630:	bf 2f 00 00 00       	mov    $0x2f,%edi
   25635:	e8 06 fd 03 00       	call   65340 <cinch_drop>
   2563a:	4c 89 ef             	mov    %r13,%rdi
   2563d:	e8 fe fc 03 00       	call   65340 <cinch_drop>
   25642:	41 bd 03 00 00 00    	mov    $0x3,%r13d
   25648:	48 83 fb 18          	cmp    $0x18,%rbx
   2564c:	72 47                	jb     25695 <Data.Bytes.Split.progress-roundtrips+0x215>
   2564e:	4d 85 ff             	test   %r15,%r15
   25651:	74 2d                	je     25680 <Data.Bytes.Split.progress-roundtrips+0x200>
   25653:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   25657:	4c 21 e0             	and    %r12,%rax
   2565a:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   25661:	00 00 00
   25664:	48 39 c8             	cmp    %rcx,%rax
   25667:	75 17                	jne    25680 <Data.Bytes.Split.progress-roundtrips+0x200>
   25669:	49 83 3f 18          	cmpq   $0x18,(%r15)
   2566d:	72 11                	jb     25680 <Data.Bytes.Split.progress-roundtrips+0x200>
   2566f:	49 8b 47 08          	mov    0x8(%r15),%rax
   25673:	0f b6 40 17          	movzbl 0x17(%rax),%eax
   25677:	eb 14                	jmp    2568d <Data.Bytes.Split.progress-roundtrips+0x20d>
   25679:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25680:	be 17 00 00 00       	mov    $0x17,%esi
   25685:	4c 89 ff             	mov    %r15,%rdi
   25688:	e8 d3 ef 03 00       	call   64660 <cinch_bytes_get>
   2568d:	4c 8d 2c 45 01 00 00 	lea    0x1(,%rax,2),%r13
   25694:	00
   25695:	45 89 f4             	mov    %r14d,%r12d
   25698:	41 83 e4 01          	and    $0x1,%r12d
   2569c:	48 8b 7d a0          	mov    -0x60(%rbp),%rdi
   256a0:	41 85 fc             	test   %edi,%r12d
   256a3:	74 1b                	je     256c0 <Data.Bytes.Split.progress-roundtrips+0x240>
   256a5:	48 89 fb             	mov    %rdi,%rbx
   256a8:	4c 31 f3             	xor    %r14,%rbx
   256ab:	e8 90 fc 03 00       	call   65340 <cinch_drop>
   256b0:	48 83 fb 02          	cmp    $0x2,%rbx
   256b4:	72 2c                	jb     256e2 <Data.Bytes.Split.progress-roundtrips+0x262>
   256b6:	e9 bb 00 00 00       	jmp    25776 <Data.Bytes.Split.progress-roundtrips+0x2f6>
   256bb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   256c0:	4c 89 f6             	mov    %r14,%rsi
   256c3:	48 89 fb             	mov    %rdi,%rbx
   256c6:	e8 d5 53 04 00       	call   6aaa0 <cinch_nat_cmp>
   256cb:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   256cf:	48 89 df             	mov    %rbx,%rdi
   256d2:	e8 69 fc 03 00       	call   65340 <cinch_drop>
   256d7:	48 83 7d b8 00       	cmpq   $0x0,-0x48(%rbp)
   256dc:	0f 85 94 00 00 00    	jne    25776 <Data.Bytes.Split.progress-roundtrips+0x2f6>
   256e2:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
   256e6:	41 85 c4             	test   %eax,%r12d
   256e9:	48 89 c7             	mov    %rax,%rdi
   256ec:	74 22                	je     25710 <Data.Bytes.Split.progress-roundtrips+0x290>
   256ee:	48 89 fb             	mov    %rdi,%rbx
   256f1:	4c 31 f3             	xor    %r14,%rbx
   256f4:	e8 47 fc 03 00       	call   65340 <cinch_drop>
   256f9:	48 83 fb 02          	cmp    $0x2,%rbx
   256fd:	49 bc 00 00 00 00 f0 	movabs $0xf000000000,%r12
   25704:	00 00 00
   25707:	72 2c                	jb     25735 <Data.Bytes.Split.progress-roundtrips+0x2b5>
   25709:	eb 74                	jmp    2577f <Data.Bytes.Split.progress-roundtrips+0x2ff>
   2570b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   25710:	49 89 fc             	mov    %rdi,%r12
   25713:	4c 89 f6             	mov    %r14,%rsi
   25716:	e8 85 53 04 00       	call   6aaa0 <cinch_nat_cmp>
   2571b:	48 89 c3             	mov    %rax,%rbx
   2571e:	4c 89 e7             	mov    %r12,%rdi
   25721:	e8 1a fc 03 00       	call   65340 <cinch_drop>
   25726:	48 85 db             	test   %rbx,%rbx
   25729:	49 bc 00 00 00 00 f0 	movabs $0xf000000000,%r12
   25730:	00 00 00
   25733:	75 4a                	jne    2577f <Data.Bytes.Split.progress-roundtrips+0x2ff>
   25735:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25739:	48 89 df             	mov    %rbx,%rdi
   2573c:	e8 ff fb 03 00       	call   65340 <cinch_drop>
   25741:	48 83 e3 fe          	and    $0xfffffffffffffffe,%rbx
   25745:	4c 89 ef             	mov    %r13,%rdi
   25748:	e8 f3 fb 03 00       	call   65340 <cinch_drop>
   2574d:	48 81 fb 5a 01 00 00 	cmp    $0x15a,%rbx
   25754:	75 42                	jne    25798 <Data.Bytes.Split.progress-roundtrips+0x318>
   25756:	49 83 fd 01          	cmp    $0x1,%r13
   2575a:	48 8b 75 a8          	mov    -0x58(%rbp),%rsi
   2575e:	0f 84 5c fd ff ff    	je     254c0 <Data.Bytes.Split.progress-roundtrips+0x40>
   25764:	eb 32                	jmp    25798 <Data.Bytes.Split.progress-roundtrips+0x318>
   25766:	b9 01 00 00 00       	mov    $0x1,%ecx
   2576b:	48 89 4d d0          	mov    %rcx,-0x30(%rbp)
   2576f:	48 85 c0             	test   %rax,%rax
   25772:	74 24                	je     25798 <Data.Bytes.Split.progress-roundtrips+0x318>
   25774:	eb 1a                	jmp    25790 <Data.Bytes.Split.progress-roundtrips+0x310>
   25776:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2577a:	e8 c1 fb 03 00       	call   65340 <cinch_drop>
   2577f:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   25783:	e8 b8 fb 03 00       	call   65340 <cinch_drop>
   25788:	4c 89 ef             	mov    %r13,%rdi
   2578b:	e8 b0 fb 03 00       	call   65340 <cinch_drop>
   25790:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   25797:	00
   25798:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   2579c:	48 83 c4 38          	add    $0x38,%rsp
   257a0:	5b                   	pop    %rbx
   257a1:	41 5c                	pop    %r12
   257a3:	41 5d                	pop    %r13
   257a5:	41 5e                	pop    %r14
   257a7:	41 5f                	pop    %r15
   257a9:	5d                   	pop    %rbp
   257aa:	c3                   	ret

Disassembly of section .fini:
