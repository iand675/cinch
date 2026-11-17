# Retained diagnostic assembly; no library source change.

build/split-progress-regression/api-o2/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000025470 <Data.Bytes.Split.progress-roundtrips>:
   25470:	55                   	push   %rbp
   25471:	48 89 e5             	mov    %rsp,%rbp
   25474:	41 57                	push   %r15
   25476:	41 56                	push   %r14
   25478:	41 55                	push   %r13
   2547a:	41 54                	push   %r12
   2547c:	53                   	push   %rbx
   2547d:	48 83 ec 38          	sub    $0x38,%rsp
   25481:	49 89 ff             	mov    %rdi,%r15
   25484:	48 b8 f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rax
   2548b:	ff ff 7f
   2548e:	48 83 c0 07          	add    $0x7,%rax
   25492:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   25496:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   2549d:	00
   2549e:	66 90                	xchg   %ax,%ax
   254a0:	48 8b 46 f8          	mov    -0x8(%rsi),%rax
   254a4:	48 c1 e8 28          	shr    $0x28,%rax
   254a8:	0f b6 c0             	movzbl %al,%eax
   254ab:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   254b1:	75 07                	jne    254ba <Data.Bytes.Split.progress-roundtrips+0x4a>
   254b3:	48 8b 46 f0          	mov    -0x10(%rsi),%rax
   254b7:	48 8b 00             	mov    (%rax),%rax
   254ba:	48 83 f8 01          	cmp    $0x1,%rax
   254be:	0f 85 6d 02 00 00    	jne    25731 <Data.Bytes.Split.progress-roundtrips+0x2c1>
   254c4:	4c 8b 26             	mov    (%rsi),%r12
   254c7:	48 8b 76 08          	mov    0x8(%rsi),%rsi
   254cb:	48 89 75 b0          	mov    %rsi,-0x50(%rbp)
   254cf:	4c 89 e7             	mov    %r12,%rdi
   254d2:	e8 69 02 04 00       	call   65740 <cinch_dup>
   254d7:	be 01 00 00 00       	mov    $0x1,%esi
   254dc:	4c 89 ff             	mov    %r15,%rdi
   254df:	4c 89 e2             	mov    %r12,%rdx
   254e2:	e8 99 f2 ff ff       	call   24780 <Data.Bytes.Split.store-count>
   254e7:	4c 89 e7             	mov    %r12,%rdi
   254ea:	e8 51 02 04 00       	call   65740 <cinch_dup>
   254ef:	be 11 00 00 00       	mov    $0x11,%esi
   254f4:	4c 89 ff             	mov    %r15,%rdi
   254f7:	4c 89 e2             	mov    %r12,%rdx
   254fa:	e8 81 f2 ff ff       	call   24780 <Data.Bytes.Split.store-count>
   254ff:	be 01 00 00 00       	mov    $0x1,%esi
   25504:	4c 89 ff             	mov    %r15,%rdi
   25507:	e8 c4 fd ff ff       	call   252d0 <Data.Bytes.Split.read-count>
   2550c:	48 89 45 a8          	mov    %rax,-0x58(%rbp)
   25510:	be 11 00 00 00       	mov    $0x11,%esi
   25515:	4c 89 ff             	mov    %r15,%rdi
   25518:	e8 b3 fd ff ff       	call   252d0 <Data.Bytes.Split.read-count>
   2551d:	4d 85 ff             	test   %r15,%r15
   25520:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   25524:	74 2a                	je     25550 <Data.Bytes.Split.progress-roundtrips+0xe0>
   25526:	49 8b 57 f8          	mov    -0x8(%r15),%rdx
   2552a:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   25531:	00 00 00
   25534:	48 21 ca             	and    %rcx,%rdx
   25537:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   2553e:	00 00 00
   25541:	48 39 ca             	cmp    %rcx,%rdx
   25544:	75 0a                	jne    25550 <Data.Bytes.Split.progress-roundtrips+0xe0>
   25546:	4d 8b 2f             	mov    (%r15),%r13
   25549:	eb 10                	jmp    2555b <Data.Bytes.Split.progress-roundtrips+0xeb>
   2554b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   25550:	4c 89 ff             	mov    %r15,%rdi
   25553:	e8 18 f1 03 00       	call   64670 <cinch_bytes_len>
   25558:	49 89 c5             	mov    %rax,%r13
   2555b:	4a 8d 1c 6d 01 00 00 	lea    0x1(,%r13,2),%rbx
   25562:	00
   25563:	4c 23 6d b8          	and    -0x48(%rbp),%r13
   25567:	bf 21 00 00 00       	mov    $0x21,%edi
   2556c:	e8 8f fd 03 00       	call   65300 <cinch_drop>
   25571:	48 89 df             	mov    %rbx,%rdi
   25574:	e8 87 fd 03 00       	call   65300 <cinch_drop>
   25579:	41 be 01 00 00 00    	mov    $0x1,%r14d
   2557f:	49 83 fd 11          	cmp    $0x11,%r13
   25583:	72 50                	jb     255d5 <Data.Bytes.Split.progress-roundtrips+0x165>
   25585:	4d 85 ff             	test   %r15,%r15
   25588:	74 36                	je     255c0 <Data.Bytes.Split.progress-roundtrips+0x150>
   2558a:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   2558e:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   25595:	00 00 00
   25598:	48 21 c8             	and    %rcx,%rax
   2559b:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   255a2:	00 00 00
   255a5:	48 39 c8             	cmp    %rcx,%rax
   255a8:	75 16                	jne    255c0 <Data.Bytes.Split.progress-roundtrips+0x150>
   255aa:	49 83 3f 11          	cmpq   $0x11,(%r15)
   255ae:	72 10                	jb     255c0 <Data.Bytes.Split.progress-roundtrips+0x150>
   255b0:	49 8b 47 08          	mov    0x8(%r15),%rax
   255b4:	0f b6 40 10          	movzbl 0x10(%rax),%eax
   255b8:	eb 13                	jmp    255cd <Data.Bytes.Split.progress-roundtrips+0x15d>
   255ba:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   255c0:	be 10 00 00 00       	mov    $0x10,%esi
   255c5:	4c 89 ff             	mov    %r15,%rdi
   255c8:	e8 53 f0 03 00       	call   64620 <cinch_bytes_get>
   255cd:	4c 8d 34 45 01 00 00 	lea    0x1(,%rax,2),%r14
   255d4:	00
   255d5:	4d 85 ff             	test   %r15,%r15
   255d8:	74 26                	je     25600 <Data.Bytes.Split.progress-roundtrips+0x190>
   255da:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   255de:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   255e5:	00 00 00
   255e8:	48 21 c8             	and    %rcx,%rax
   255eb:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   255f2:	00 00 00
   255f5:	48 39 c8             	cmp    %rcx,%rax
   255f8:	75 06                	jne    25600 <Data.Bytes.Split.progress-roundtrips+0x190>
   255fa:	49 8b 1f             	mov    (%r15),%rbx
   255fd:	eb 0c                	jmp    2560b <Data.Bytes.Split.progress-roundtrips+0x19b>
   255ff:	90                   	nop
   25600:	4c 89 ff             	mov    %r15,%rdi
   25603:	e8 68 f0 03 00       	call   64670 <cinch_bytes_len>
   25608:	48 89 c3             	mov    %rax,%rbx
   2560b:	4c 8d 2c 5d 01 00 00 	lea    0x1(,%rbx,2),%r13
   25612:	00
   25613:	48 b8 f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rax
   2561a:	ff ff 7f
   2561d:	48 21 c3             	and    %rax,%rbx
   25620:	bf 2f 00 00 00       	mov    $0x2f,%edi
   25625:	e8 d6 fc 03 00       	call   65300 <cinch_drop>
   2562a:	4c 89 ef             	mov    %r13,%rdi
   2562d:	e8 ce fc 03 00       	call   65300 <cinch_drop>
   25632:	b8 03 00 00 00       	mov    $0x3,%eax
   25637:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   2563b:	48 83 fb 18          	cmp    $0x18,%rbx
   2563f:	4c 8b 6d a8          	mov    -0x58(%rbp),%r13
   25643:	72 54                	jb     25699 <Data.Bytes.Split.progress-roundtrips+0x229>
   25645:	4d 85 ff             	test   %r15,%r15
   25648:	74 36                	je     25680 <Data.Bytes.Split.progress-roundtrips+0x210>
   2564a:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   2564e:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   25655:	00 00 00
   25658:	48 21 c8             	and    %rcx,%rax
   2565b:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   25662:	00 00 00
   25665:	48 39 c8             	cmp    %rcx,%rax
   25668:	75 16                	jne    25680 <Data.Bytes.Split.progress-roundtrips+0x210>
   2566a:	49 83 3f 18          	cmpq   $0x18,(%r15)
   2566e:	72 10                	jb     25680 <Data.Bytes.Split.progress-roundtrips+0x210>
   25670:	49 8b 47 08          	mov    0x8(%r15),%rax
   25674:	0f b6 40 17          	movzbl 0x17(%rax),%eax
   25678:	eb 13                	jmp    2568d <Data.Bytes.Split.progress-roundtrips+0x21d>
   2567a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   25680:	be 17 00 00 00       	mov    $0x17,%esi
   25685:	4c 89 ff             	mov    %r15,%rdi
   25688:	e8 93 ef 03 00       	call   64620 <cinch_bytes_get>
   2568d:	48 8d 04 45 01 00 00 	lea    0x1(,%rax,2),%rax
   25694:	00
   25695:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   25699:	44 89 e3             	mov    %r12d,%ebx
   2569c:	83 e3 01             	and    $0x1,%ebx
   2569f:	4c 89 ef             	mov    %r13,%rdi
   256a2:	e8 59 fc 03 00       	call   65300 <cinch_drop>
   256a7:	44 85 eb             	test   %r13d,%ebx
   256aa:	74 14                	je     256c0 <Data.Bytes.Split.progress-roundtrips+0x250>
   256ac:	4d 31 e5             	xor    %r12,%r13
   256af:	49 83 fd 02          	cmp    $0x2,%r13
   256b3:	72 1b                	jb     256d0 <Data.Bytes.Split.progress-roundtrips+0x260>
   256b5:	e9 87 00 00 00       	jmp    25741 <Data.Bytes.Split.progress-roundtrips+0x2d1>
   256ba:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   256c0:	4c 89 ef             	mov    %r13,%rdi
   256c3:	4c 89 e6             	mov    %r12,%rsi
   256c6:	e8 95 53 04 00       	call   6aa60 <cinch_nat_cmp>
   256cb:	48 85 c0             	test   %rax,%rax
   256ce:	75 71                	jne    25741 <Data.Bytes.Split.progress-roundtrips+0x2d1>
   256d0:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   256d4:	4c 89 ef             	mov    %r13,%rdi
   256d7:	e8 24 fc 03 00       	call   65300 <cinch_drop>
   256dc:	44 85 eb             	test   %r13d,%ebx
   256df:	74 0f                	je     256f0 <Data.Bytes.Split.progress-roundtrips+0x280>
   256e1:	4d 31 e5             	xor    %r12,%r13
   256e4:	49 83 fd 02          	cmp    $0x2,%r13
   256e8:	72 16                	jb     25700 <Data.Bytes.Split.progress-roundtrips+0x290>
   256ea:	eb 5e                	jmp    2574a <Data.Bytes.Split.progress-roundtrips+0x2da>
   256ec:	0f 1f 40 00          	nopl   0x0(%rax)
   256f0:	4c 89 ef             	mov    %r13,%rdi
   256f3:	4c 89 e6             	mov    %r12,%rsi
   256f6:	e8 65 53 04 00       	call   6aa60 <cinch_nat_cmp>
   256fb:	48 85 c0             	test   %rax,%rax
   256fe:	75 4a                	jne    2574a <Data.Bytes.Split.progress-roundtrips+0x2da>
   25700:	4c 89 f7             	mov    %r14,%rdi
   25703:	e8 f8 fb 03 00       	call   65300 <cinch_drop>
   25708:	49 83 e6 fe          	and    $0xfffffffffffffffe,%r14
   2570c:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25710:	48 89 df             	mov    %rbx,%rdi
   25713:	e8 e8 fb 03 00       	call   65300 <cinch_drop>
   25718:	49 81 fe 5a 01 00 00 	cmp    $0x15a,%r14
   2571f:	75 42                	jne    25763 <Data.Bytes.Split.progress-roundtrips+0x2f3>
   25721:	48 83 fb 01          	cmp    $0x1,%rbx
   25725:	48 8b 75 b0          	mov    -0x50(%rbp),%rsi
   25729:	0f 84 71 fd ff ff    	je     254a0 <Data.Bytes.Split.progress-roundtrips+0x30>
   2572f:	eb 32                	jmp    25763 <Data.Bytes.Split.progress-roundtrips+0x2f3>
   25731:	b9 01 00 00 00       	mov    $0x1,%ecx
   25736:	48 89 4d d0          	mov    %rcx,-0x30(%rbp)
   2573a:	48 85 c0             	test   %rax,%rax
   2573d:	74 24                	je     25763 <Data.Bytes.Split.progress-roundtrips+0x2f3>
   2573f:	eb 1a                	jmp    2575b <Data.Bytes.Split.progress-roundtrips+0x2eb>
   25741:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   25745:	e8 b6 fb 03 00       	call   65300 <cinch_drop>
   2574a:	4c 89 f7             	mov    %r14,%rdi
   2574d:	e8 ae fb 03 00       	call   65300 <cinch_drop>
   25752:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   25756:	e8 a5 fb 03 00       	call   65300 <cinch_drop>
   2575b:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   25762:	00
   25763:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   25767:	48 83 c4 38          	add    $0x38,%rsp
   2576b:	5b                   	pop    %rbx
   2576c:	41 5c                	pop    %r12
   2576e:	41 5d                	pop    %r13
   25770:	41 5e                	pop    %r14
   25772:	41 5f                	pop    %r15
   25774:	5d                   	pop    %rbp
   25775:	c3                   	ret

Disassembly of section .fini:
