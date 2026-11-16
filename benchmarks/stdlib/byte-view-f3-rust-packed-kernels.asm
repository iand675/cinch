
build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029720 <Data.Bytes.View.endpoint-blocks>:
   29720:	55                   	push   rbp
   29721:	48 89 e5             	mov    rbp,rsp
   29724:	48 83 ec 10          	sub    rsp,0x10
   29728:	e8 23 fd ff ff       	call   29450 <Data.Bytes.View.endpoint-blocks#cc>
   2972d:	48 83 c4 08          	add    rsp,0x8
   29731:	5d                   	pop    rbp
   29732:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029450 <Data.Bytes.View.endpoint-blocks#cc>:
   29450:	55                   	push   rbp
   29451:	48 89 e5             	mov    rbp,rsp
   29454:	41 57                	push   r15
   29456:	41 56                	push   r14
   29458:	41 55                	push   r13
   2945a:	41 54                	push   r12
   2945c:	53                   	push   rbx
   2945d:	50                   	push   rax
   2945e:	48 89 d0             	mov    rax,rdx
   29461:	48 29 f0             	sub    rax,rsi
   29464:	48 83 f8 08          	cmp    rax,0x8
   29468:	7d 30                	jge    2949a <Data.Bytes.View.endpoint-blocks#cc+0x4a>
   2946a:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   29471:	48 39 f2             	cmp    rdx,rsi
   29474:	0f 8e 04 02 00 00    	jle    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   2947a:	44 38 04 37          	cmp    BYTE PTR [rdi+rsi*1],r8b
   2947e:	0f 85 e0 00 00 00    	jne    29564 <Data.Bytes.View.endpoint-blocks#cc+0x114>
   29484:	48 8d 04 37          	lea    rax,[rdi+rsi*1]
   29488:	44 38 0c 01          	cmp    BYTE PTR [rcx+rax*1],r9b
   2948c:	0f 85 d2 00 00 00    	jne    29564 <Data.Bytes.View.endpoint-blocks#cc+0x114>
   29492:	48 89 f0             	mov    rax,rsi
   29495:	e9 e4 01 00 00       	jmp    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   2949a:	49 be 80 80 80 80 80 	movabs r14,0x8080808080808080
   294a1:	80 80 80 
   294a4:	49 bf 7f 7f 7f 7f 7f 	movabs r15,0x7f7f7f7f7f7f7f7f
   294ab:	7f 7f 7f 
   294ae:	4c 8b 14 37          	mov    r10,QWORD PTR [rdi+rsi*1]
   294b2:	4d 31 c2             	xor    r10,r8
   294b5:	4c 89 d0             	mov    rax,r10
   294b8:	4c 21 f8             	and    rax,r15
   294bb:	4c 01 f8             	add    rax,r15
   294be:	4c 09 d0             	or     rax,r10
   294c1:	4c 21 f0             	and    rax,r14
   294c4:	4c 39 f0             	cmp    rax,r14
   294c7:	75 69                	jne    29532 <Data.Bytes.View.endpoint-blocks#cc+0xe2>
   294c9:	4c 8d 56 08          	lea    r10,[rsi+0x8]
   294cd:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   294d4:	49 39 d2             	cmp    r10,rdx
   294d7:	0f 8d a1 01 00 00    	jge    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   294dd:	46 38 04 17          	cmp    BYTE PTR [rdi+r10*1],r8b
   294e1:	0f 84 85 01 00 00    	je     2966c <Data.Bytes.View.endpoint-blocks#cc+0x21c>
   294e7:	45 0f b6 d8          	movzx  r11d,r8b
   294eb:	4c 8d 56 09          	lea    r10,[rsi+0x9]
   294ef:	48 89 d3             	mov    rbx,rdx
   294f2:	4c 29 d3             	sub    rbx,r10
   294f5:	48 83 fb 08          	cmp    rbx,0x8
   294f9:	0f 8d 89 00 00 00    	jge    29588 <Data.Bytes.View.endpoint-blocks#cc+0x138>
   294ff:	4c 39 d2             	cmp    rdx,r10
   29502:	0f 8e 76 01 00 00    	jle    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   29508:	46 38 04 17          	cmp    BYTE PTR [rdi+r10*1],r8b
   2950c:	0f 84 5a 01 00 00    	je     2966c <Data.Bytes.View.endpoint-blocks#cc+0x21c>
   29512:	4c 8d 56 0a          	lea    r10,[rsi+0xa]
   29516:	49 39 d2             	cmp    r10,rdx
   29519:	0f 8d 5f 01 00 00    	jge    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   2951f:	46 38 04 17          	cmp    BYTE PTR [rdi+r10*1],r8b
   29523:	0f 84 43 01 00 00    	je     2966c <Data.Bytes.View.endpoint-blocks#cc+0x21c>
   29529:	48 83 c6 0b          	add    rsi,0xb
   2952d:	e9 9c 00 00 00       	jmp    295ce <Data.Bytes.View.endpoint-blocks#cc+0x17e>
   29532:	4c 31 f0             	xor    rax,r14
   29535:	4c 8d 14 0f          	lea    r10,[rdi+rcx*1]
   29539:	4e 8b 1c 16          	mov    r11,QWORD PTR [rsi+r10*1]
   2953d:	4d 31 cb             	xor    r11,r9
   29540:	4d 89 da             	mov    r10,r11
   29543:	4d 21 fa             	and    r10,r15
   29546:	4d 01 fa             	add    r10,r15
   29549:	4d 09 da             	or     r10,r11
   2954c:	49 f7 d2             	not    r10
   2954f:	49 21 c2             	and    r10,rax
   29552:	74 25                	je     29579 <Data.Bytes.View.endpoint-blocks#cc+0x129>
   29554:	f3 49 0f bc c2       	tzcnt  rax,r10
   29559:	c1 e8 03             	shr    eax,0x3
   2955c:	48 01 f0             	add    rax,rsi
   2955f:	e9 1a 01 00 00       	jmp    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   29564:	45 0f b6 c0          	movzx  r8d,r8b
   29568:	45 0f b6 c9          	movzx  r9d,r9b
   2956c:	48 ff c6             	inc    rsi
   2956f:	e8 cc 01 00 00       	call   29740 <Data.Bytes.View.endpoint-tail>
   29574:	e9 05 01 00 00       	jmp    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   29579:	48 83 c6 08          	add    rsi,0x8
   2957d:	48 89 f8             	mov    rax,rdi
   29580:	48 89 d7             	mov    rdi,rdx
   29583:	e9 13 01 00 00       	jmp    2969b <Data.Bytes.View.endpoint-blocks#cc+0x24b>
   29588:	4e 8b 24 17          	mov    r12,QWORD PTR [rdi+r10*1]
   2958c:	4d 31 c4             	xor    r12,r8
   2958f:	4c 89 e3             	mov    rbx,r12
   29592:	4c 21 fb             	and    rbx,r15
   29595:	4c 01 fb             	add    rbx,r15
   29598:	4c 09 e3             	or     rbx,r12
   2959b:	4c 21 f3             	and    rbx,r14
   2959e:	4c 39 f3             	cmp    rbx,r14
   295a1:	0f 85 b2 00 00 00    	jne    29659 <Data.Bytes.View.endpoint-blocks#cc+0x209>
   295a7:	4c 8d 56 11          	lea    r10,[rsi+0x11]
   295ab:	48 89 d3             	mov    rbx,rdx
   295ae:	4c 29 d3             	sub    rbx,r10
   295b1:	48 83 fb 08          	cmp    rbx,0x8
   295b5:	7d 39                	jge    295f0 <Data.Bytes.View.endpoint-blocks#cc+0x1a0>
   295b7:	4c 39 d2             	cmp    rdx,r10
   295ba:	0f 8e be 00 00 00    	jle    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   295c0:	46 38 04 17          	cmp    BYTE PTR [rdi+r10*1],r8b
   295c4:	0f 84 a2 00 00 00    	je     2966c <Data.Bytes.View.endpoint-blocks#cc+0x21c>
   295ca:	48 83 c6 12          	add    rsi,0x12
   295ce:	48 89 fb             	mov    rbx,rdi
   295d1:	49 89 d6             	mov    r14,rdx
   295d4:	49 89 cf             	mov    r15,rcx
   295d7:	4c 89 d9             	mov    rcx,r11
   295da:	4d 89 cc             	mov    r12,r9
   295dd:	4d 89 c5             	mov    r13,r8
   295e0:	e8 4b f3 ff ff       	call   28930 <Data.Bytes.View.index-forward-tail>
   295e5:	4d 89 e8             	mov    r8,r13
   295e8:	4c 89 f2             	mov    rdx,r14
   295eb:	48 89 df             	mov    rdi,rbx
   295ee:	eb 52                	jmp    29642 <Data.Bytes.View.endpoint-blocks#cc+0x1f2>
   295f0:	4e 8b 24 17          	mov    r12,QWORD PTR [rdi+r10*1]
   295f4:	4d 31 c4             	xor    r12,r8
   295f7:	4c 89 e3             	mov    rbx,r12
   295fa:	4c 21 fb             	and    rbx,r15
   295fd:	4c 01 fb             	add    rbx,r15
   29600:	4c 09 e3             	or     rbx,r12
   29603:	4c 21 f3             	and    rbx,r14
   29606:	4c 39 f3             	cmp    rbx,r14
   29609:	75 4e                	jne    29659 <Data.Bytes.View.endpoint-blocks#cc+0x209>
   2960b:	48 8d 5e 19          	lea    rbx,[rsi+0x19]
   2960f:	49 89 d2             	mov    r10,rdx
   29612:	49 29 da             	sub    r10,rbx
   29615:	49 83 fa 08          	cmp    r10,0x8
   29619:	0f 8d 9b 00 00 00    	jge    296ba <Data.Bytes.View.endpoint-blocks#cc+0x26a>
   2961f:	49 89 fe             	mov    r14,rdi
   29622:	48 89 de             	mov    rsi,rbx
   29625:	48 89 d3             	mov    rbx,rdx
   29628:	49 89 cf             	mov    r15,rcx
   2962b:	4c 89 d9             	mov    rcx,r11
   2962e:	4d 89 cc             	mov    r12,r9
   29631:	4d 89 c5             	mov    r13,r8
   29634:	e8 f7 f2 ff ff       	call   28930 <Data.Bytes.View.index-forward-tail>
   29639:	4d 89 e8             	mov    r8,r13
   2963c:	48 89 da             	mov    rdx,rbx
   2963f:	4c 89 f7             	mov    rdi,r14
   29642:	4d 89 e1             	mov    r9,r12
   29645:	4c 89 f9             	mov    rcx,r15
   29648:	49 89 c2             	mov    r10,rax
   2964b:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   29652:	4d 85 d2             	test   r10,r10
   29655:	79 1a                	jns    29671 <Data.Bytes.View.endpoint-blocks#cc+0x221>
   29657:	eb 25                	jmp    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   29659:	4c 31 f3             	xor    rbx,r14
   2965c:	be 40 00 00 00       	mov    esi,0x40
   29661:	f3 48 0f bc f3       	tzcnt  rsi,rbx
   29666:	c1 ee 03             	shr    esi,0x3
   29669:	49 01 f2             	add    r10,rsi
   2966c:	4d 85 d2             	test   r10,r10
   2966f:	78 0d                	js     2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>
   29671:	48 8d 04 0f          	lea    rax,[rdi+rcx*1]
   29675:	45 38 0c 02          	cmp    BYTE PTR [r10+rax*1],r9b
   29679:	75 14                	jne    2968f <Data.Bytes.View.endpoint-blocks#cc+0x23f>
   2967b:	4c 89 d0             	mov    rax,r10
   2967e:	48 83 c4 08          	add    rsp,0x8
   29682:	5b                   	pop    rbx
   29683:	41 5c                	pop    r12
   29685:	41 5d                	pop    r13
   29687:	41 5e                	pop    r14
   29689:	41 5f                	pop    r15
   2968b:	5d                   	pop    rbp
   2968c:	c2 08 00             	ret    0x8
   2968f:	49 ff c2             	inc    r10
   29692:	48 89 f8             	mov    rax,rdi
   29695:	48 89 d7             	mov    rdi,rdx
   29698:	4c 89 d6             	mov    rsi,r10
   2969b:	48 89 c2             	mov    rdx,rax
   2969e:	48 89 c8             	mov    rax,rcx
   296a1:	4c 89 c1             	mov    rcx,r8
   296a4:	49 89 c0             	mov    r8,rax
   296a7:	48 83 c4 08          	add    rsp,0x8
   296ab:	5b                   	pop    rbx
   296ac:	41 5c                	pop    r12
   296ae:	41 5d                	pop    r13
   296b0:	41 5e                	pop    r14
   296b2:	41 5f                	pop    r15
   296b4:	5d                   	pop    rbp
   296b5:	e9 26 c0 00 00       	jmp    356e0 <Data.Bytes.View.endpoint-blocks.shared0#cc>
   296ba:	4c 8b 14 1f          	mov    r10,QWORD PTR [rdi+rbx*1]
   296be:	4d 31 c2             	xor    r10,r8
   296c1:	4d 89 d3             	mov    r11,r10
   296c4:	4d 21 fb             	and    r11,r15
   296c7:	4d 01 fb             	add    r11,r15
   296ca:	4d 09 d3             	or     r11,r10
   296cd:	4d 21 f3             	and    r11,r14
   296d0:	4d 39 f3             	cmp    r11,r14
   296d3:	75 20                	jne    296f5 <Data.Bytes.View.endpoint-blocks#cc+0x2a5>
   296d5:	48 83 c6 21          	add    rsi,0x21
   296d9:	48 89 fb             	mov    rbx,rdi
   296dc:	49 89 d6             	mov    r14,rdx
   296df:	49 89 cf             	mov    r15,rcx
   296e2:	4c 89 c1             	mov    rcx,r8
   296e5:	4d 89 cc             	mov    r12,r9
   296e8:	4d 89 c5             	mov    r13,r8
   296eb:	e8 90 f0 ff ff       	call   28780 <Data.Bytes.View.index-forward-blocks>
   296f0:	e9 f0 fe ff ff       	jmp    295e5 <Data.Bytes.View.endpoint-blocks#cc+0x195>
   296f5:	4d 31 f3             	xor    r11,r14
   296f8:	41 ba 40 00 00 00    	mov    r10d,0x40
   296fe:	f3 4d 0f bc d3       	tzcnt  r10,r11
   29703:	41 c1 ea 03          	shr    r10d,0x3
   29707:	49 01 da             	add    r10,rbx
   2970a:	4d 85 d2             	test   r10,r10
   2970d:	0f 89 5e ff ff ff    	jns    29671 <Data.Bytes.View.endpoint-blocks#cc+0x221>
   29713:	e9 66 ff ff ff       	jmp    2967e <Data.Bytes.View.endpoint-blocks#cc+0x22e>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000357b0 <Data.Bytes.View.endpoint-blocks.shared0>:
   357b0:	55                   	push   rbp
   357b1:	48 89 e5             	mov    rbp,rsp
   357b4:	48 83 ec 10          	sub    rsp,0x10
   357b8:	e8 23 ff ff ff       	call   356e0 <Data.Bytes.View.endpoint-blocks.shared0#cc>
   357bd:	48 83 c4 08          	add    rsp,0x8
   357c1:	5d                   	pop    rbp
   357c2:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000356e0 <Data.Bytes.View.endpoint-blocks.shared0#cc>:
   356e0:	55                   	push   rbp
   356e1:	48 89 e5             	mov    rbp,rsp
   356e4:	53                   	push   rbx
   356e5:	50                   	push   rax
   356e6:	48 89 f8             	mov    rax,rdi
   356e9:	48 29 f7             	sub    rdi,rsi
   356ec:	48 83 ff 08          	cmp    rdi,0x8
   356f0:	7d 1b                	jge    3570d <Data.Bytes.View.endpoint-blocks.shared0#cc+0x2d>
   356f2:	44 0f b6 d1          	movzx  r10d,cl
   356f6:	45 0f b6 c9          	movzx  r9d,r9b
   356fa:	48 89 d7             	mov    rdi,rdx
   356fd:	48 89 c2             	mov    rdx,rax
   35700:	4c 89 c1             	mov    rcx,r8
   35703:	4d 89 d0             	mov    r8,r10
   35706:	e8 35 40 ff ff       	call   29740 <Data.Bytes.View.endpoint-tail>
   3570b:	eb 7a                	jmp    35787 <Data.Bytes.View.endpoint-blocks.shared0#cc+0xa7>
   3570d:	49 bb 80 80 80 80 80 	movabs r11,0x8080808080808080
   35714:	80 80 80 
   35717:	49 ba 7f 7f 7f 7f 7f 	movabs r10,0x7f7f7f7f7f7f7f7f
   3571e:	7f 7f 7f 
   35721:	48 8b 1c 32          	mov    rbx,QWORD PTR [rdx+rsi*1]
   35725:	48 31 cb             	xor    rbx,rcx
   35728:	48 89 df             	mov    rdi,rbx
   3572b:	4c 21 d7             	and    rdi,r10
   3572e:	4c 01 d7             	add    rdi,r10
   35731:	48 09 df             	or     rdi,rbx
   35734:	4c 21 df             	and    rdi,r11
   35737:	4c 39 df             	cmp    rdi,r11
   3573a:	75 1e                	jne    3575a <Data.Bytes.View.endpoint-blocks.shared0#cc+0x7a>
   3573c:	48 83 c6 08          	add    rsi,0x8
   35740:	48 89 d7             	mov    rdi,rdx
   35743:	48 89 c2             	mov    rdx,rax
   35746:	48 89 c8             	mov    rax,rcx
   35749:	4c 89 c1             	mov    rcx,r8
   3574c:	49 89 c0             	mov    r8,rax
   3574f:	48 83 c4 08          	add    rsp,0x8
   35753:	5b                   	pop    rbx
   35754:	5d                   	pop    rbp
   35755:	e9 16 3a ff ff       	jmp    29170 <Data.Bytes.View.endpoint-first#cc>
   3575a:	4c 31 df             	xor    rdi,r11
   3575d:	4e 8d 1c 02          	lea    r11,[rdx+r8*1]
   35761:	4a 8b 1c 1e          	mov    rbx,QWORD PTR [rsi+r11*1]
   35765:	4c 31 cb             	xor    rbx,r9
   35768:	49 89 db             	mov    r11,rbx
   3576b:	4d 21 d3             	and    r11,r10
   3576e:	4d 01 d3             	add    r11,r10
   35771:	49 09 db             	or     r11,rbx
   35774:	49 f7 d3             	not    r11
   35777:	49 21 fb             	and    r11,rdi
   3577a:	74 14                	je     35790 <Data.Bytes.View.endpoint-blocks.shared0#cc+0xb0>
   3577c:	f3 49 0f bc c3       	tzcnt  rax,r11
   35781:	c1 e8 03             	shr    eax,0x3
   35784:	48 01 f0             	add    rax,rsi
   35787:	48 83 c4 08          	add    rsp,0x8
   3578b:	5b                   	pop    rbx
   3578c:	5d                   	pop    rbp
   3578d:	c2 08 00             	ret    0x8
   35790:	48 83 c6 08          	add    rsi,0x8
   35794:	48 89 d7             	mov    rdi,rdx
   35797:	48 89 c2             	mov    rdx,rax
   3579a:	48 89 c8             	mov    rax,rcx
   3579d:	4c 89 c1             	mov    rcx,r8
   357a0:	49 89 c0             	mov    r8,rax
   357a3:	48 83 c4 08          	add    rsp,0x8
   357a7:	5b                   	pop    rbx
   357a8:	5d                   	pop    rbp
   357a9:	e9 a2 3c ff ff       	jmp    29450 <Data.Bytes.View.endpoint-blocks#cc>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000028780 <Data.Bytes.View.index-forward-blocks>:
   28780:	55                   	push   rbp
   28781:	48 89 e5             	mov    rbp,rsp
   28784:	41 56                	push   r14
   28786:	53                   	push   rbx
   28787:	49 89 d0             	mov    r8,rdx
   2878a:	49 29 f0             	sub    r8,rsi
   2878d:	49 83 f8 08          	cmp    r8,0x8
   28791:	7d 21                	jge    287b4 <Data.Bytes.View.index-forward-blocks+0x34>
   28793:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   2879a:	48 39 d6             	cmp    rsi,rdx
   2879d:	0f 8d 82 01 00 00    	jge    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   287a3:	38 0c 37             	cmp    BYTE PTR [rdi+rsi*1],cl
   287a6:	0f 85 c9 00 00 00    	jne    28875 <Data.Bytes.View.index-forward-blocks+0xf5>
   287ac:	48 89 f0             	mov    rax,rsi
   287af:	e9 71 01 00 00       	jmp    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   287b4:	48 b8 80 80 80 80 80 	movabs rax,0x8080808080808080
   287bb:	80 80 80 
   287be:	49 bb 7f 7f 7f 7f 7f 	movabs r11,0x7f7f7f7f7f7f7f7f
   287c5:	7f 7f 7f 
   287c8:	4c 8d 57 10          	lea    r10,[rdi+0x10]
   287cc:	45 31 c9             	xor    r9d,r9d
   287cf:	90                   	nop
   287d0:	4d 8b 74 32 f0       	mov    r14,QWORD PTR [r10+rsi*1-0x10]
   287d5:	49 31 ce             	xor    r14,rcx
   287d8:	4c 89 f3             	mov    rbx,r14
   287db:	4c 21 db             	and    rbx,r11
   287de:	4c 01 db             	add    rbx,r11
   287e1:	4c 09 f3             	or     rbx,r14
   287e4:	48 21 c3             	and    rbx,rax
   287e7:	48 39 c3             	cmp    rbx,rax
   287ea:	0f 85 a8 00 00 00    	jne    28898 <Data.Bytes.View.index-forward-blocks+0x118>
   287f0:	4b 8d 1c 08          	lea    rbx,[r8+r9*1]
   287f4:	48 83 c3 f8          	add    rbx,0xfffffffffffffff8
   287f8:	48 83 fb 08          	cmp    rbx,0x8
   287fc:	0f 8c ae 00 00 00    	jl     288b0 <Data.Bytes.View.index-forward-blocks+0x130>
   28802:	4d 8b 74 32 f8       	mov    r14,QWORD PTR [r10+rsi*1-0x8]
   28807:	49 31 ce             	xor    r14,rcx
   2880a:	4c 89 f3             	mov    rbx,r14
   2880d:	4c 21 db             	and    rbx,r11
   28810:	4c 01 db             	add    rbx,r11
   28813:	4c 09 f3             	or     rbx,r14
   28816:	48 21 c3             	and    rbx,rax
   28819:	48 39 c3             	cmp    rbx,rax
   2881c:	0f 85 ba 00 00 00    	jne    288dc <Data.Bytes.View.index-forward-blocks+0x15c>
   28822:	4b 8d 1c 08          	lea    rbx,[r8+r9*1]
   28826:	48 83 c3 f0          	add    rbx,0xfffffffffffffff0
   2882a:	48 83 fb 08          	cmp    rbx,0x8
   2882e:	0f 8c c4 00 00 00    	jl     288f8 <Data.Bytes.View.index-forward-blocks+0x178>
   28834:	4d 8b 34 32          	mov    r14,QWORD PTR [r10+rsi*1]
   28838:	49 31 ce             	xor    r14,rcx
   2883b:	4c 89 f3             	mov    rbx,r14
   2883e:	4c 21 db             	and    rbx,r11
   28841:	4c 01 db             	add    rbx,r11
   28844:	4c 09 f3             	or     rbx,r14
   28847:	48 21 c3             	and    rbx,rax
   2884a:	48 39 c3             	cmp    rbx,rax
   2884d:	0f 85 b8 00 00 00    	jne    2890b <Data.Bytes.View.index-forward-blocks+0x18b>
   28853:	49 83 c2 18          	add    r10,0x18
   28857:	4b 8d 1c 08          	lea    rbx,[r8+r9*1]
   2885b:	48 83 c3 e8          	add    rbx,0xffffffffffffffe8
   2885f:	49 83 c1 e8          	add    r9,0xffffffffffffffe8
   28863:	48 83 fb 08          	cmp    rbx,0x8
   28867:	0f 8d 63 ff ff ff    	jge    287d0 <Data.Bytes.View.index-forward-blocks+0x50>
   2886d:	4c 29 ce             	sub    rsi,r9
   28870:	e9 1e ff ff ff       	jmp    28793 <Data.Bytes.View.index-forward-blocks+0x13>
   28875:	4c 8d 46 01          	lea    r8,[rsi+0x1]
   28879:	49 39 d0             	cmp    r8,rdx
   2887c:	0f 8d a3 00 00 00    	jge    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   28882:	4c 89 c0             	mov    rax,r8
   28885:	42 38 0c 07          	cmp    BYTE PTR [rdi+r8*1],cl
   28889:	0f 84 96 00 00 00    	je     28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   2888f:	0f b6 c9             	movzx  ecx,cl
   28892:	48 83 c6 02          	add    rsi,0x2
   28896:	eb 6a                	jmp    28902 <Data.Bytes.View.index-forward-blocks+0x182>
   28898:	48 31 c3             	xor    rbx,rax
   2889b:	b8 40 00 00 00       	mov    eax,0x40
   288a0:	f3 48 0f bc c3       	tzcnt  rax,rbx
   288a5:	c1 e8 03             	shr    eax,0x3
   288a8:	48 01 f0             	add    rax,rsi
   288ab:	4c 29 c8             	sub    rax,r9
   288ae:	eb 75                	jmp    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   288b0:	49 89 f0             	mov    r8,rsi
   288b3:	4d 29 c8             	sub    r8,r9
   288b6:	4d 8d 48 08          	lea    r9,[r8+0x8]
   288ba:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   288c1:	49 39 d1             	cmp    r9,rdx
   288c4:	7d 5f                	jge    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   288c6:	4c 89 c8             	mov    rax,r9
   288c9:	41 38 4c 32 f8       	cmp    BYTE PTR [r10+rsi*1-0x8],cl
   288ce:	74 55                	je     28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   288d0:	0f b6 c9             	movzx  ecx,cl
   288d3:	49 83 c0 09          	add    r8,0x9
   288d7:	4c 89 c6             	mov    rsi,r8
   288da:	eb 26                	jmp    28902 <Data.Bytes.View.index-forward-blocks+0x182>
   288dc:	48 31 c3             	xor    rbx,rax
   288df:	b8 40 00 00 00       	mov    eax,0x40
   288e4:	f3 48 0f bc c3       	tzcnt  rax,rbx
   288e9:	c1 e8 03             	shr    eax,0x3
   288ec:	48 01 f0             	add    rax,rsi
   288ef:	4c 29 c8             	sub    rax,r9
   288f2:	48 83 c0 08          	add    rax,0x8
   288f6:	eb 2d                	jmp    28925 <Data.Bytes.View.index-forward-blocks+0x1a5>
   288f8:	0f b6 c9             	movzx  ecx,cl
   288fb:	4c 29 ce             	sub    rsi,r9
   288fe:	48 83 c6 10          	add    rsi,0x10
   28902:	5b                   	pop    rbx
   28903:	41 5e                	pop    r14
   28905:	5d                   	pop    rbp
   28906:	e9 25 00 00 00       	jmp    28930 <Data.Bytes.View.index-forward-tail>
   2890b:	48 31 c3             	xor    rbx,rax
   2890e:	b8 40 00 00 00       	mov    eax,0x40
   28913:	f3 48 0f bc c3       	tzcnt  rax,rbx
   28918:	c1 e8 03             	shr    eax,0x3
   2891b:	48 01 f0             	add    rax,rsi
   2891e:	4c 29 c8             	sub    rax,r9
   28921:	48 83 c0 10          	add    rax,0x10
   28925:	5b                   	pop    rbx
   28926:	41 5e                	pop    r14
   28928:	5d                   	pop    rbp
   28929:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000002bbb0 <Data.Bytes.View.search-count>:
   2bbb0:	55                   	push   rbp
   2bbb1:	48 89 e5             	mov    rbp,rsp
   2bbb4:	41 57                	push   r15
   2bbb6:	41 56                	push   r14
   2bbb8:	41 55                	push   r13
   2bbba:	41 54                	push   r12
   2bbbc:	53                   	push   rbx
   2bbbd:	48 83 ec 18          	sub    rsp,0x18
   2bbc1:	4c 89 4d d0          	mov    QWORD PTR [rbp-0x30],r9
   2bbc5:	49 89 cf             	mov    r15,rcx
   2bbc8:	49 89 d4             	mov    r12,rdx
   2bbcb:	49 89 f6             	mov    r14,rsi
   2bbce:	48 89 fb             	mov    rbx,rdi
   2bbd1:	4c 8b 6d 20          	mov    r13,QWORD PTR [rbp+0x20]
   2bbd5:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   2bbd9:	4c 89 45 c0          	mov    QWORD PTR [rbp-0x40],r8
   2bbdd:	49 8d 48 f8          	lea    rcx,[r8-0x8]
   2bbe1:	48 89 4d c8          	mov    QWORD PTR [rbp-0x38],rcx
   2bbe5:	eb 26                	jmp    2bc0d <Data.Bytes.View.search-count+0x5d>
   2bbe7:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   2bbee:	00 00 
   2bbf0:	49 83 c6 08          	add    r14,0x8
   2bbf4:	48 83 c0 08          	add    rax,0x8
   2bbf8:	31 c9                	xor    ecx,ecx
   2bbfa:	48 3b 45 c0          	cmp    rax,QWORD PTR [rbp-0x40]
   2bbfe:	0f 9d c1             	setge  cl
   2bc01:	ba 00 00 00 00       	mov    edx,0x0
   2bc06:	48 0f 4d c2          	cmovge rax,rdx
   2bc0a:	49 01 cd             	add    r13,rcx
   2bc0d:	48 85 c0             	test   rax,rax
   2bc10:	75 22                	jne    2bc34 <Data.Bytes.View.search-count+0x84>
   2bc12:	48 89 df             	mov    rdi,rbx
   2bc15:	4c 89 f6             	mov    rsi,r14
   2bc18:	4c 89 e2             	mov    rdx,r12
   2bc1b:	48 8b 4d 10          	mov    rcx,QWORD PTR [rbp+0x10]
   2bc1f:	e8 fc c9 ff ff       	call   28620 <Data.Bytes.View.index-forward>
   2bc24:	48 85 c0             	test   rax,rax
   2bc27:	78 56                	js     2bc7f <Data.Bytes.View.search-count+0xcf>
   2bc29:	49 89 c6             	mov    r14,rax
   2bc2c:	49 ff c6             	inc    r14
   2bc2f:	b8 01 00 00 00       	mov    eax,0x1
   2bc34:	4c 89 e1             	mov    rcx,r12
   2bc37:	4c 29 f1             	sub    rcx,r14
   2bc3a:	7e 43                	jle    2bc7f <Data.Bytes.View.search-count+0xcf>
   2bc3c:	48 3b 45 c8          	cmp    rax,QWORD PTR [rbp-0x38]
   2bc40:	7f 1e                	jg     2bc60 <Data.Bytes.View.search-count+0xb0>
   2bc42:	48 83 f9 07          	cmp    rcx,0x7
   2bc46:	7e 18                	jle    2bc60 <Data.Bytes.View.search-count+0xb0>
   2bc48:	4a 8b 0c 33          	mov    rcx,QWORD PTR [rbx+r14*1]
   2bc4c:	49 3b 0c 07          	cmp    rcx,QWORD PTR [r15+rax*1]
   2bc50:	74 9e                	je     2bbf0 <Data.Bytes.View.search-count+0x40>
   2bc52:	eb 11                	jmp    2bc65 <Data.Bytes.View.search-count+0xb5>
   2bc54:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   2bc5b:	00 00 00 00 00 
   2bc60:	42 0f b6 0c 33       	movzx  ecx,BYTE PTR [rbx+r14*1]
   2bc65:	0f b6 c9             	movzx  ecx,cl
   2bc68:	4c 89 ff             	mov    rdi,r15
   2bc6b:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   2bc6f:	48 89 c2             	mov    rdx,rax
   2bc72:	e8 39 dc ff ff       	call   298b0 <Data.Bytes.View.search-step>
   2bc77:	49 ff c6             	inc    r14
   2bc7a:	e9 79 ff ff ff       	jmp    2bbf8 <Data.Bytes.View.search-count+0x48>
   2bc7f:	4c 89 e8             	mov    rax,r13
   2bc82:	48 83 c4 18          	add    rsp,0x18
   2bc86:	5b                   	pop    rbx
   2bc87:	41 5c                	pop    r12
   2bc89:	41 5d                	pop    r13
   2bc8b:	41 5e                	pop    r14
   2bc8d:	41 5f                	pop    r15
   2bc8f:	5d                   	pop    rbp
   2bc90:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029dc0 <Data.Bytes.View.search-first>:
   29dc0:	55                   	push   rbp
   29dc1:	48 89 e5             	mov    rbp,rsp
   29dc4:	48 83 ec 20          	sub    rsp,0x20
   29dc8:	0f 28 45 10          	movaps xmm0,XMMWORD PTR [rbp+0x10]
   29dcc:	0f 11 04 24          	movups XMMWORD PTR [rsp],xmm0
   29dd0:	e8 bb fb ff ff       	call   29990 <Data.Bytes.View.search-first#cc>
   29dd5:	48 83 c4 08          	add    rsp,0x8
   29dd9:	5d                   	pop    rbp
   29dda:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029990 <Data.Bytes.View.search-first#cc>:
   29990:	55                   	push   rbp
   29991:	48 89 e5             	mov    rbp,rsp
   29994:	41 57                	push   r15
   29996:	41 56                	push   r14
   29998:	41 55                	push   r13
   2999a:	41 54                	push   r12
   2999c:	53                   	push   rbx
   2999d:	48 83 ec 18          	sub    rsp,0x18
   299a1:	4c 89 4d d0          	mov    QWORD PTR [rbp-0x30],r9
   299a5:	4d 89 c6             	mov    r14,r8
   299a8:	49 89 c8             	mov    r8,rcx
   299ab:	48 89 d3             	mov    rbx,rdx
   299ae:	49 89 fc             	mov    r12,rdi
   299b1:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   299b5:	4c 8b 4d 10          	mov    r9,QWORD PTR [rbp+0x10]
   299b9:	48 85 c0             	test   rax,rax
   299bc:	4c 89 4d c8          	mov    QWORD PTR [rbp-0x38],r9
   299c0:	74 48                	je     29a0a <Data.Bytes.View.search-first#cc+0x7a>
   299c2:	49 89 f5             	mov    r13,rsi
   299c5:	49 c7 c7 ff ff ff ff 	mov    r15,0xffffffffffffffff
   299cc:	48 89 d9             	mov    rcx,rbx
   299cf:	4c 29 e9             	sub    rcx,r13
   299d2:	0f 8e 2a 03 00 00    	jle    29d02 <Data.Bytes.View.search-first#cc+0x372>
   299d8:	49 8d 56 f8          	lea    rdx,[r14-0x8]
   299dc:	48 39 d0             	cmp    rax,rdx
   299df:	0f 8f 6f 02 00 00    	jg     29c54 <Data.Bytes.View.search-first#cc+0x2c4>
   299e5:	48 83 f9 08          	cmp    rcx,0x8
   299e9:	0f 8c 65 02 00 00    	jl     29c54 <Data.Bytes.View.search-first#cc+0x2c4>
   299ef:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   299f3:	49 3b 0c 00          	cmp    rcx,QWORD PTR [r8+rax*1]
   299f7:	0f 85 57 02 00 00    	jne    29c54 <Data.Bytes.View.search-first#cc+0x2c4>
   299fd:	49 83 c5 08          	add    r13,0x8
   29a01:	48 83 c0 08          	add    rax,0x8
   29a05:	e9 53 03 00 00       	jmp    29d5d <Data.Bytes.View.search-first#cc+0x3cd>
   29a0a:	41 0f b6 c1          	movzx  eax,r9b
   29a0e:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
   29a12:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   29a19:	1f 84 00 00 00 00 00 
   29a20:	49 c7 c7 ff ff ff ff 	mov    r15,0xffffffffffffffff
   29a27:	48 39 de             	cmp    rsi,rbx
   29a2a:	0f 8d d2 02 00 00    	jge    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29a30:	45 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],r9b
   29a34:	75 1a                	jne    29a50 <Data.Bytes.View.search-first#cc+0xc0>
   29a36:	49 89 f5             	mov    r13,rsi
   29a39:	4d 85 ed             	test   r13,r13
   29a3c:	0f 89 27 01 00 00    	jns    29b69 <Data.Bytes.View.search-first#cc+0x1d9>
   29a42:	e9 01 02 00 00       	jmp    29c48 <Data.Bytes.View.search-first#cc+0x2b8>
   29a47:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   29a4e:	00 00 
   29a50:	4c 8d 6e 01          	lea    r13,[rsi+0x1]
   29a54:	48 89 d8             	mov    rax,rbx
   29a57:	4c 29 e8             	sub    rax,r13
   29a5a:	48 83 f8 08          	cmp    rax,0x8
   29a5e:	7d 30                	jge    29a90 <Data.Bytes.View.search-first#cc+0x100>
   29a60:	4c 39 eb             	cmp    rbx,r13
   29a63:	0f 8e 99 02 00 00    	jle    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29a69:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   29a6d:	0f 84 ed 00 00 00    	je     29b60 <Data.Bytes.View.search-first#cc+0x1d0>
   29a73:	4c 8d 6e 02          	lea    r13,[rsi+0x2]
   29a77:	49 39 dd             	cmp    r13,rbx
   29a7a:	0f 8d 82 02 00 00    	jge    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29a80:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   29a84:	0f 84 d6 00 00 00    	je     29b60 <Data.Bytes.View.search-first#cc+0x1d0>
   29a8a:	48 83 c6 03          	add    rsi,0x3
   29a8e:	eb 5a                	jmp    29aea <Data.Bytes.View.search-first#cc+0x15a>
   29a90:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   29a94:	4c 31 c9             	xor    rcx,r9
   29a97:	48 89 c8             	mov    rax,rcx
   29a9a:	48 ba 7f 7f 7f 7f 7f 	movabs rdx,0x7f7f7f7f7f7f7f7f
   29aa1:	7f 7f 7f 
   29aa4:	48 21 d0             	and    rax,rdx
   29aa7:	48 01 d0             	add    rax,rdx
   29aaa:	48 09 c8             	or     rax,rcx
   29aad:	48 b9 80 80 80 80 80 	movabs rcx,0x8080808080808080
   29ab4:	80 80 80 
   29ab7:	48 21 c8             	and    rax,rcx
   29aba:	48 39 c8             	cmp    rax,rcx
   29abd:	0f 85 8a 00 00 00    	jne    29b4d <Data.Bytes.View.search-first#cc+0x1bd>
   29ac3:	4c 8d 6e 09          	lea    r13,[rsi+0x9]
   29ac7:	48 89 d8             	mov    rax,rbx
   29aca:	4c 29 e8             	sub    rax,r13
   29acd:	48 83 f8 08          	cmp    rax,0x8
   29ad1:	7d 39                	jge    29b0c <Data.Bytes.View.search-first#cc+0x17c>
   29ad3:	4c 39 eb             	cmp    rbx,r13
   29ad6:	0f 8e 26 02 00 00    	jle    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29adc:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   29ae0:	0f 84 7a 00 00 00    	je     29b60 <Data.Bytes.View.search-first#cc+0x1d0>
   29ae6:	48 83 c6 0a          	add    rsi,0xa
   29aea:	4c 89 e7             	mov    rdi,r12
   29aed:	48 89 da             	mov    rdx,rbx
   29af0:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
   29af4:	4d 89 c5             	mov    r13,r8
   29af7:	e8 34 ee ff ff       	call   28930 <Data.Bytes.View.index-forward-tail>
   29afc:	4d 89 e8             	mov    r8,r13
   29aff:	49 89 c5             	mov    r13,rax
   29b02:	4d 85 ed             	test   r13,r13
   29b05:	79 62                	jns    29b69 <Data.Bytes.View.search-first#cc+0x1d9>
   29b07:	e9 3c 01 00 00       	jmp    29c48 <Data.Bytes.View.search-first#cc+0x2b8>
   29b0c:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   29b10:	4c 31 c9             	xor    rcx,r9
   29b13:	48 89 c8             	mov    rax,rcx
   29b16:	48 21 d0             	and    rax,rdx
   29b19:	48 01 d0             	add    rax,rdx
   29b1c:	48 09 c8             	or     rax,rcx
   29b1f:	48 b9 80 80 80 80 80 	movabs rcx,0x8080808080808080
   29b26:	80 80 80 
   29b29:	48 21 c8             	and    rax,rcx
   29b2c:	48 39 c8             	cmp    rax,rcx
   29b2f:	75 1c                	jne    29b4d <Data.Bytes.View.search-first#cc+0x1bd>
   29b31:	48 8d 46 11          	lea    rax,[rsi+0x11]
   29b35:	48 89 d9             	mov    rcx,rbx
   29b38:	48 29 c1             	sub    rcx,rax
   29b3b:	48 83 f9 08          	cmp    rcx,0x8
   29b3f:	0f 8d 9c 00 00 00    	jge    29be1 <Data.Bytes.View.search-first#cc+0x251>
   29b45:	4c 89 e7             	mov    rdi,r12
   29b48:	48 89 c6             	mov    rsi,rax
   29b4b:	eb a0                	jmp    29aed <Data.Bytes.View.search-first#cc+0x15d>
   29b4d:	48 31 c8             	xor    rax,rcx
   29b50:	b9 40 00 00 00       	mov    ecx,0x40
   29b55:	f3 48 0f bc c8       	tzcnt  rcx,rax
   29b5a:	c1 e9 03             	shr    ecx,0x3
   29b5d:	49 01 cd             	add    r13,rcx
   29b60:	4d 85 ed             	test   r13,r13
   29b63:	0f 88 df 00 00 00    	js     29c48 <Data.Bytes.View.search-first#cc+0x2b8>
   29b69:	49 83 fe 01          	cmp    r14,0x1
   29b6d:	0f 8e d5 00 00 00    	jle    29c48 <Data.Bytes.View.search-first#cc+0x2b8>
   29b73:	49 8d 45 01          	lea    rax,[r13+0x1]
   29b77:	48 89 d9             	mov    rcx,rbx
   29b7a:	48 29 c1             	sub    rcx,rax
   29b7d:	0f 8e 7f 01 00 00    	jle    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29b83:	49 83 fe 09          	cmp    r14,0x9
   29b87:	7c 17                	jl     29ba0 <Data.Bytes.View.search-first#cc+0x210>
   29b89:	48 83 f9 07          	cmp    rcx,0x7
   29b8d:	7e 11                	jle    29ba0 <Data.Bytes.View.search-first#cc+0x210>
   29b8f:	49 8b 04 04          	mov    rax,QWORD PTR [r12+rax*1]
   29b93:	49 3b 40 01          	cmp    rax,QWORD PTR [r8+0x1]
   29b97:	75 0c                	jne    29ba5 <Data.Bytes.View.search-first#cc+0x215>
   29b99:	e9 5b 01 00 00       	jmp    29cf9 <Data.Bytes.View.search-first#cc+0x369>
   29b9e:	66 90                	xchg   ax,ax
   29ba0:	41 0f b6 04 04       	movzx  eax,BYTE PTR [r12+rax*1]
   29ba5:	0f b6 c8             	movzx  ecx,al
   29ba8:	ba 01 00 00 00       	mov    edx,0x1
   29bad:	4d 89 c7             	mov    r15,r8
   29bb0:	4c 89 c7             	mov    rdi,r8
   29bb3:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   29bb7:	e8 f4 fc ff ff       	call   298b0 <Data.Bytes.View.search-step>
   29bbc:	49 83 c5 02          	add    r13,0x2
   29bc0:	4c 39 f0             	cmp    rax,r14
   29bc3:	0f 8d 2b 01 00 00    	jge    29cf4 <Data.Bytes.View.search-first#cc+0x364>
   29bc9:	4c 89 ee             	mov    rsi,r13
   29bcc:	48 85 c0             	test   rax,rax
   29bcf:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29bd3:	4d 89 f8             	mov    r8,r15
   29bd6:	0f 84 44 fe ff ff    	je     29a20 <Data.Bytes.View.search-first#cc+0x90>
   29bdc:	e9 e4 fd ff ff       	jmp    299c5 <Data.Bytes.View.search-first#cc+0x35>
   29be1:	49 8b 14 04          	mov    rdx,QWORD PTR [r12+rax*1]
   29be5:	4c 31 ca             	xor    rdx,r9
   29be8:	48 89 d1             	mov    rcx,rdx
   29beb:	48 bf 7f 7f 7f 7f 7f 	movabs rdi,0x7f7f7f7f7f7f7f7f
   29bf2:	7f 7f 7f 
   29bf5:	48 21 f9             	and    rcx,rdi
   29bf8:	48 01 f9             	add    rcx,rdi
   29bfb:	48 09 d1             	or     rcx,rdx
   29bfe:	48 ba 80 80 80 80 80 	movabs rdx,0x8080808080808080
   29c05:	80 80 80 
   29c08:	48 21 d1             	and    rcx,rdx
   29c0b:	48 39 d1             	cmp    rcx,rdx
   29c0e:	75 1a                	jne    29c2a <Data.Bytes.View.search-first#cc+0x29a>
   29c10:	48 83 c6 19          	add    rsi,0x19
   29c14:	4c 89 e7             	mov    rdi,r12
   29c17:	48 89 da             	mov    rdx,rbx
   29c1a:	4c 89 c9             	mov    rcx,r9
   29c1d:	4d 89 c5             	mov    r13,r8
   29c20:	e8 5b eb ff ff       	call   28780 <Data.Bytes.View.index-forward-blocks>
   29c25:	e9 d2 fe ff ff       	jmp    29afc <Data.Bytes.View.search-first#cc+0x16c>
   29c2a:	48 31 d1             	xor    rcx,rdx
   29c2d:	41 bd 40 00 00 00    	mov    r13d,0x40
   29c33:	f3 4c 0f bc e9       	tzcnt  r13,rcx
   29c38:	41 c1 ed 03          	shr    r13d,0x3
   29c3c:	49 01 c5             	add    r13,rax
   29c3f:	4d 85 ed             	test   r13,r13
   29c42:	0f 89 21 ff ff ff    	jns    29b69 <Data.Bytes.View.search-first#cc+0x1d9>
   29c48:	4d 85 ed             	test   r13,r13
   29c4b:	4d 0f 49 fd          	cmovns r15,r13
   29c4f:	e9 ae 00 00 00       	jmp    29d02 <Data.Bytes.View.search-first#cc+0x372>
   29c54:	43 0f b6 0c 2c       	movzx  ecx,BYTE PTR [r12+r13*1]
   29c59:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29c5d:	0f 84 8c 00 00 00    	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29c63:	48 85 c0             	test   rax,rax
   29c66:	0f 8e ec 00 00 00    	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29c6c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29c70:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29c75:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29c79:	74 74                	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29c7b:	48 85 c0             	test   rax,rax
   29c7e:	0f 8e d4 00 00 00    	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29c84:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29c88:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29c8d:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29c91:	74 5c                	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29c93:	48 85 c0             	test   rax,rax
   29c96:	0f 8e bc 00 00 00    	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29c9c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29ca0:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29ca5:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29ca9:	74 44                	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29cab:	48 85 c0             	test   rax,rax
   29cae:	0f 8e a4 00 00 00    	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29cb4:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29cb8:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29cbd:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29cc1:	74 2c                	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29cc3:	48 85 c0             	test   rax,rax
   29cc6:	0f 8e 8c 00 00 00    	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29ccc:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29cd0:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29cd5:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29cd9:	74 14                	je     29cef <Data.Bytes.View.search-first#cc+0x35f>
   29cdb:	48 85 c0             	test   rax,rax
   29cde:	7e 78                	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29ce0:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29ce4:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29ce9:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29ced:	75 39                	jne    29d28 <Data.Bytes.View.search-first#cc+0x398>
   29cef:	48 ff c0             	inc    rax
   29cf2:	eb 66                	jmp    29d5a <Data.Bytes.View.search-first#cc+0x3ca>
   29cf4:	4d 29 f5             	sub    r13,r14
   29cf7:	eb 06                	jmp    29cff <Data.Bytes.View.search-first#cc+0x36f>
   29cf9:	49 83 fe 09          	cmp    r14,0x9
   29cfd:	75 17                	jne    29d16 <Data.Bytes.View.search-first#cc+0x386>
   29cff:	4d 89 ef             	mov    r15,r13
   29d02:	4c 89 f8             	mov    rax,r15
   29d05:	48 83 c4 18          	add    rsp,0x18
   29d09:	5b                   	pop    rbx
   29d0a:	41 5c                	pop    r12
   29d0c:	41 5d                	pop    r13
   29d0e:	41 5e                	pop    r14
   29d10:	41 5f                	pop    r15
   29d12:	5d                   	pop    rbp
   29d13:	c2 18 00             	ret    0x18
   29d16:	49 83 c5 09          	add    r13,0x9
   29d1a:	b8 09 00 00 00       	mov    eax,0x9
   29d1f:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29d23:	e9 9d fc ff ff       	jmp    299c5 <Data.Bytes.View.search-first#cc+0x35>
   29d28:	48 85 c0             	test   rax,rax
   29d2b:	7e 2b                	jle    29d58 <Data.Bytes.View.search-first#cc+0x3c8>
   29d2d:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29d31:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29d36:	4d 89 c7             	mov    r15,r8
   29d39:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29d3d:	74 14                	je     29d53 <Data.Bytes.View.search-first#cc+0x3c3>
   29d3f:	48 85 c0             	test   rax,rax
   29d42:	7e 66                	jle    29daa <Data.Bytes.View.search-first#cc+0x41a>
   29d44:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29d48:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29d4d:	41 38 0c 07          	cmp    BYTE PTR [r15+rax*1],cl
   29d51:	75 38                	jne    29d8b <Data.Bytes.View.search-first#cc+0x3fb>
   29d53:	48 ff c0             	inc    rax
   29d56:	eb 54                	jmp    29dac <Data.Bytes.View.search-first#cc+0x41c>
   29d58:	31 c0                	xor    eax,eax
   29d5a:	49 ff c5             	inc    r13
   29d5d:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   29d61:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   29d65:	4c 89 45 10          	mov    QWORD PTR [rbp+0x10],r8
   29d69:	48 89 c7             	mov    rdi,rax
   29d6c:	4c 89 f6             	mov    rsi,r14
   29d6f:	4c 89 ea             	mov    rdx,r13
   29d72:	4c 89 e1             	mov    rcx,r12
   29d75:	49 89 d8             	mov    r8,rbx
   29d78:	48 83 c4 18          	add    rsp,0x18
   29d7c:	5b                   	pop    rbx
   29d7d:	41 5c                	pop    r12
   29d7f:	41 5d                	pop    r13
   29d81:	41 5e                	pop    r14
   29d83:	41 5f                	pop    r15
   29d85:	5d                   	pop    rbp
   29d86:	e9 45 ba 00 00       	jmp    357d0 <Data.Bytes.View.search-first.shared0#cc>
   29d8b:	48 85 c0             	test   rax,rax
   29d8e:	7e 1a                	jle    29daa <Data.Bytes.View.search-first#cc+0x41a>
   29d90:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   29d94:	48 8b 54 c6 f8       	mov    rdx,QWORD PTR [rsi+rax*8-0x8]
   29d99:	4c 89 ff             	mov    rdi,r15
   29d9c:	e8 0f fb ff ff       	call   298b0 <Data.Bytes.View.search-step>
   29da1:	4d 89 f8             	mov    r8,r15
   29da4:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29da8:	eb b0                	jmp    29d5a <Data.Bytes.View.search-first#cc+0x3ca>
   29daa:	31 c0                	xor    eax,eax
   29dac:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29db0:	4d 89 f8             	mov    r8,r15
   29db3:	eb a5                	jmp    29d5a <Data.Bytes.View.search-first#cc+0x3ca>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035930 <Data.Bytes.View.search-first.shared0>:
   35930:	55                   	push   rbp
   35931:	48 89 e5             	mov    rbp,rsp
   35934:	48 83 ec 20          	sub    rsp,0x20
   35938:	0f 28 45 10          	movaps xmm0,XMMWORD PTR [rbp+0x10]
   3593c:	0f 11 04 24          	movups XMMWORD PTR [rsp],xmm0
   35940:	e8 8b fe ff ff       	call   357d0 <Data.Bytes.View.search-first.shared0#cc>
   35945:	48 83 c4 08          	add    rsp,0x8
   35949:	5d                   	pop    rbp
   3594a:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000357d0 <Data.Bytes.View.search-first.shared0#cc>:
   357d0:	55                   	push   rbp
   357d1:	48 89 e5             	mov    rbp,rsp
   357d4:	41 57                	push   r15
   357d6:	41 56                	push   r14
   357d8:	41 55                	push   r13
   357da:	41 54                	push   r12
   357dc:	53                   	push   rbx
   357dd:	48 83 ec 18          	sub    rsp,0x18
   357e1:	48 89 f3             	mov    rbx,rsi
   357e4:	48 39 f7             	cmp    rdi,rsi
   357e7:	7d 68                	jge    35851 <Data.Bytes.View.search-first.shared0#cc+0x81>
   357e9:	4c 8b 6d 18          	mov    r13,QWORD PTR [rbp+0x18]
   357ed:	4c 8b 7d 10          	mov    r15,QWORD PTR [rbp+0x10]
   357f1:	48 85 ff             	test   rdi,rdi
   357f4:	74 72                	je     35868 <Data.Bytes.View.search-first.shared0#cc+0x98>
   357f6:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   357fd:	4c 89 c6             	mov    rsi,r8
   35800:	48 29 d6             	sub    rsi,rdx
   35803:	7e 52                	jle    35857 <Data.Bytes.View.search-first.shared0#cc+0x87>
   35805:	48 8d 43 f8          	lea    rax,[rbx-0x8]
   35809:	48 39 c7             	cmp    rdi,rax
   3580c:	0f 9f c0             	setg   al
   3580f:	48 83 fe 08          	cmp    rsi,0x8
   35813:	40 0f 9c c6          	setl   sil
   35817:	40 08 c6             	or     sil,al
   3581a:	0f 85 9c 00 00 00    	jne    358bc <Data.Bytes.View.search-first.shared0#cc+0xec>
   35820:	48 8b 04 11          	mov    rax,QWORD PTR [rcx+rdx*1]
   35824:	49 3b 04 3f          	cmp    rax,QWORD PTR [r15+rdi*1]
   35828:	0f 85 8e 00 00 00    	jne    358bc <Data.Bytes.View.search-first.shared0#cc+0xec>
   3582e:	48 83 c2 08          	add    rdx,0x8
   35832:	48 83 c7 08          	add    rdi,0x8
   35836:	48 39 df             	cmp    rdi,rbx
   35839:	7d 16                	jge    35851 <Data.Bytes.View.search-first.shared0#cc+0x81>
   3583b:	48 89 7d 18          	mov    QWORD PTR [rbp+0x18],rdi
   3583f:	4c 89 4d 10          	mov    QWORD PTR [rbp+0x10],r9
   35843:	48 89 cf             	mov    rdi,rcx
   35846:	48 89 d6             	mov    rsi,rdx
   35849:	4c 89 c2             	mov    rdx,r8
   3584c:	e9 a9 00 00 00       	jmp    358fa <Data.Bytes.View.search-first.shared0#cc+0x12a>
   35851:	48 29 da             	sub    rdx,rbx
   35854:	48 89 d0             	mov    rax,rdx
   35857:	48 83 c4 18          	add    rsp,0x18
   3585b:	5b                   	pop    rbx
   3585c:	41 5c                	pop    r12
   3585e:	41 5d                	pop    r13
   35860:	41 5e                	pop    r14
   35862:	41 5f                	pop    r15
   35864:	5d                   	pop    rbp
   35865:	c2 18 00             	ret    0x18
   35868:	4c 89 7d d0          	mov    QWORD PTR [rbp-0x30],r15
   3586c:	4d 89 ef             	mov    r15,r13
   3586f:	49 89 cc             	mov    r12,rcx
   35872:	48 89 cf             	mov    rdi,rcx
   35875:	48 89 d6             	mov    rsi,rdx
   35878:	4d 89 c5             	mov    r13,r8
   3587b:	4c 89 c2             	mov    rdx,r8
   3587e:	4d 89 ce             	mov    r14,r9
   35881:	4c 89 c9             	mov    rcx,r9
   35884:	e8 97 2d ff ff       	call   28620 <Data.Bytes.View.index-forward>
   35889:	48 85 c0             	test   rax,rax
   3588c:	0f 88 84 00 00 00    	js     35916 <Data.Bytes.View.search-first.shared0#cc+0x146>
   35892:	48 83 fb 02          	cmp    rbx,0x2
   35896:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   3589a:	7c bb                	jl     35857 <Data.Bytes.View.search-first.shared0#cc+0x87>
   3589c:	4c 89 ea             	mov    rdx,r13
   3589f:	4c 89 e7             	mov    rdi,r12
   358a2:	48 ff c0             	inc    rax
   358a5:	4c 89 75 10          	mov    QWORD PTR [rbp+0x10],r14
   358a9:	48 c7 45 18 01 00 00 	mov    QWORD PTR [rbp+0x18],0x1
   358b0:	00 
   358b1:	48 89 c6             	mov    rsi,rax
   358b4:	49 89 d8             	mov    r8,rbx
   358b7:	4d 89 f9             	mov    r9,r15
   358ba:	eb 47                	jmp    35903 <Data.Bytes.View.search-first.shared0#cc+0x133>
   358bc:	4d 89 cc             	mov    r12,r9
   358bf:	4c 89 45 d0          	mov    QWORD PTR [rbp-0x30],r8
   358c3:	48 89 4d c8          	mov    QWORD PTR [rbp-0x38],rcx
   358c7:	0f b6 0c 11          	movzx  ecx,BYTE PTR [rcx+rdx*1]
   358cb:	48 89 f8             	mov    rax,rdi
   358ce:	4c 89 ff             	mov    rdi,r15
   358d1:	4c 89 ee             	mov    rsi,r13
   358d4:	49 89 d6             	mov    r14,rdx
   358d7:	48 89 c2             	mov    rdx,rax
   358da:	e8 d1 3f ff ff       	call   298b0 <Data.Bytes.View.search-step>
   358df:	4c 89 f6             	mov    rsi,r14
   358e2:	48 ff c6             	inc    rsi
   358e5:	48 39 d8             	cmp    rax,rbx
   358e8:	7d 38                	jge    35922 <Data.Bytes.View.search-first.shared0#cc+0x152>
   358ea:	48 89 45 18          	mov    QWORD PTR [rbp+0x18],rax
   358ee:	4c 89 65 10          	mov    QWORD PTR [rbp+0x10],r12
   358f2:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   358f6:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   358fa:	4c 89 f9             	mov    rcx,r15
   358fd:	49 89 d8             	mov    r8,rbx
   35900:	4d 89 e9             	mov    r9,r13
   35903:	48 83 c4 18          	add    rsp,0x18
   35907:	5b                   	pop    rbx
   35908:	41 5c                	pop    r12
   3590a:	41 5d                	pop    r13
   3590c:	41 5e                	pop    r14
   3590e:	41 5f                	pop    r15
   35910:	5d                   	pop    rbp
   35911:	e9 7a 40 ff ff       	jmp    29990 <Data.Bytes.View.search-first#cc>
   35916:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   3591d:	e9 35 ff ff ff       	jmp    35857 <Data.Bytes.View.search-first.shared0#cc+0x87>
   35922:	48 29 de             	sub    rsi,rbx
   35925:	48 89 f0             	mov    rax,rsi
   35928:	e9 2a ff ff ff       	jmp    35857 <Data.Bytes.View.search-first.shared0#cc+0x87>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000032100 <Main.work>:
   32100:	55                   	push   rbp
   32101:	48 89 e5             	mov    rbp,rsp
   32104:	41 57                	push   r15
   32106:	41 56                	push   r14
   32108:	41 55                	push   r13
   3210a:	41 54                	push   r12
   3210c:	53                   	push   rbx
   3210d:	48 83 ec 48          	sub    rsp,0x48
   32111:	4d 89 cc             	mov    r12,r9
   32114:	4c 89 45 98          	mov    QWORD PTR [rbp-0x68],r8
   32118:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
   3211c:	48 89 55 a0          	mov    QWORD PTR [rbp-0x60],rdx
   32120:	48 89 75 a8          	mov    QWORD PTR [rbp-0x58],rsi
   32124:	48 89 7d b0          	mov    QWORD PTR [rbp-0x50],rdi
   32128:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   3212c:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   32130:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   32134:	eb 14                	jmp    3214a <Main.work+0x4a>
   32136:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3213d:	00 00 00 
   32140:	48 ff ca             	dec    rdx
   32143:	49 89 57 f8          	mov    QWORD PTR [r15-0x8],rdx
   32147:	4d 89 ec             	mov    r12,r13
   3214a:	49 89 c7             	mov    r15,rax
   3214d:	4d 85 e4             	test   r12,r12
   32150:	0f 94 c0             	sete   al
   32153:	45 89 e6             	mov    r14d,r12d
   32156:	41 08 c6             	or     r14b,al
   32159:	41 f6 c6 01          	test   r14b,0x1
   3215d:	74 21                	je     32180 <Main.work+0x80>
   3215f:	41 f6 c4 01          	test   r12b,0x1
   32163:	75 3b                	jne    321a0 <Main.work+0xa0>
   32165:	be 01 00 00 00       	mov    esi,0x1
   3216a:	4c 89 e7             	mov    rdi,r12
   3216d:	e8 7e 4f 04 00       	call   770f0 <cinch_nat_cmp>
   32172:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   32176:	e9 b5 00 00 00       	jmp    32230 <Main.work+0x130>
   3217b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   32180:	49 8b 44 24 f8       	mov    rax,QWORD PTR [r12-0x8]
   32185:	48 89 c1             	mov    rcx,rax
   32188:	48 c1 e1 1d          	shl    rcx,0x1d
   3218c:	48 85 c9             	test   rcx,rcx
   3218f:	7e 27                	jle    321b8 <Main.work+0xb8>
   32191:	48 ff c0             	inc    rax
   32194:	49 89 44 24 f8       	mov    QWORD PTR [r12-0x8],rax
   32199:	eb 27                	jmp    321c2 <Main.work+0xc2>
   3219b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   321a0:	4c 89 e0             	mov    rax,r12
   321a3:	48 d1 e8             	shr    rax,1
   321a6:	48 85 c0             	test   rax,rax
   321a9:	0f 97 c0             	seta   al
   321ac:	1c 00                	sbb    al,0x0
   321ae:	48 0f be c0          	movsx  rax,al
   321b2:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   321b6:	eb 78                	jmp    32230 <Main.work+0x130>
   321b8:	74 08                	je     321c2 <Main.work+0xc2>
   321ba:	4c 89 e7             	mov    rdi,r12
   321bd:	e8 0e fd 03 00       	call   71ed0 <cinch_dup_mt>
   321c2:	be 01 00 00 00       	mov    esi,0x1
   321c7:	4c 89 e7             	mov    rdi,r12
   321ca:	e8 21 4f 04 00       	call   770f0 <cinch_nat_cmp>
   321cf:	49 8b 54 24 f8       	mov    rdx,QWORD PTR [r12-0x8]
   321d4:	48 89 d1             	mov    rcx,rdx
   321d7:	48 c1 e1 1d          	shl    rcx,0x1d
   321db:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   321e2:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   321e6:	7c 18                	jl     32200 <Main.work+0x100>
   321e8:	48 ff ca             	dec    rdx
   321eb:	49 89 54 24 f8       	mov    QWORD PTR [r12-0x8],rdx
   321f0:	eb 3e                	jmp    32230 <Main.work+0x130>
   321f2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   321f9:	1f 84 00 00 00 00 00 
   32200:	48 85 c9             	test   rcx,rcx
   32203:	74 2b                	je     32230 <Main.work+0x130>
   32205:	48 89 c3             	mov    rbx,rax
   32208:	4c 89 e7             	mov    rdi,r12
   3220b:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   32212:	75 07                	jne    3221b <Main.work+0x11b>
   32214:	e8 d7 f9 03 00       	call   71bf0 <cinch_drop_slow>
   32219:	eb 05                	jmp    32220 <Main.work+0x120>
   3221b:	e8 e0 f8 03 00       	call   71b00 <cinch_drop_mt>
   32220:	48 89 d8             	mov    rax,rbx
   32223:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   3222a:	84 00 00 00 00 00 
   32230:	48 85 c0             	test   rax,rax
   32233:	0f 84 55 03 00 00    	je     3258e <Main.work+0x48e>
   32239:	44 88 75 d7          	mov    BYTE PTR [rbp-0x29],r14b
   3223d:	4d 85 ff             	test   r15,r15
   32240:	0f 94 c0             	sete   al
   32243:	45 89 fe             	mov    r14d,r15d
   32246:	41 08 c6             	or     r14b,al
   32249:	41 f6 c6 01          	test   r14b,0x1
   3224d:	75 31                	jne    32280 <Main.work+0x180>
   3224f:	49 8b 47 f8          	mov    rax,QWORD PTR [r15-0x8]
   32253:	48 89 c1             	mov    rcx,rax
   32256:	48 c1 e1 1d          	shl    rcx,0x1d
   3225a:	48 85 c9             	test   rcx,rcx
   3225d:	7e 09                	jle    32268 <Main.work+0x168>
   3225f:	48 ff c0             	inc    rax
   32262:	49 89 47 f8          	mov    QWORD PTR [r15-0x8],rax
   32266:	eb 18                	jmp    32280 <Main.work+0x180>
   32268:	74 16                	je     32280 <Main.work+0x180>
   3226a:	4c 89 ff             	mov    rdi,r15
   3226d:	e8 5e fc 03 00       	call   71ed0 <cinch_dup_mt>
   32272:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   32279:	1f 84 00 00 00 00 00 
   32280:	48 8b 7d a8          	mov    rdi,QWORD PTR [rbp-0x58]
   32284:	48 8b 75 a0          	mov    rsi,QWORD PTR [rbp-0x60]
   32288:	4c 89 fa             	mov    rdx,r15
   3228b:	e8 f0 f2 ff ff       	call   31580 <Main.mutate>
   32290:	4d 85 ed             	test   r13,r13
   32293:	74 2b                	je     322c0 <Main.work+0x1c0>
   32295:	49 8b 45 f8          	mov    rax,QWORD PTR [r13-0x8]
   32299:	48 89 c1             	mov    rcx,rax
   3229c:	48 c1 e1 1d          	shl    rcx,0x1d
   322a0:	48 85 c9             	test   rcx,rcx
   322a3:	7e 0b                	jle    322b0 <Main.work+0x1b0>
   322a5:	48 ff c0             	inc    rax
   322a8:	49 89 45 f8          	mov    QWORD PTR [r13-0x8],rax
   322ac:	eb 12                	jmp    322c0 <Main.work+0x1c0>
   322ae:	66 90                	xchg   ax,ax
   322b0:	74 0e                	je     322c0 <Main.work+0x1c0>
   322b2:	4c 89 ef             	mov    rdi,r13
   322b5:	e8 16 fc 03 00       	call   71ed0 <cinch_dup_mt>
   322ba:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   322c0:	4c 89 7d b8          	mov    QWORD PTR [rbp-0x48],r15
   322c4:	48 8b 7d b0          	mov    rdi,QWORD PTR [rbp-0x50]
   322c8:	4c 89 ee             	mov    rsi,r13
   322cb:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
   322cf:	e8 5c f6 ff ff       	call   31930 <Main.operation>
   322d4:	49 89 c7             	mov    r15,rax
   322d7:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   322db:	89 fb                	mov    ebx,edi
   322dd:	83 e3 01             	and    ebx,0x1
   322e0:	44 85 fb             	test   ebx,r15d
   322e3:	74 2b                	je     32310 <Main.work+0x210>
   322e5:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
   322e9:	48 d1 e9             	shr    rcx,1
   322ec:	4c 89 f8             	mov    rax,r15
   322ef:	48 d1 e8             	shr    rax,1
   322f2:	48 01 c8             	add    rax,rcx
   322f5:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   322f9:	78 15                	js     32310 <Main.work+0x210>
   322fb:	4c 8d 2c 45 01 00 00 	lea    r13,[rax*2+0x1]
   32302:	00 
   32303:	eb 1a                	jmp    3231f <Main.work+0x21f>
   32305:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3230c:	00 00 00 00 
   32310:	4c 89 fe             	mov    rsi,r15
   32313:	e8 e8 4b 04 00       	call   76f00 <cinch_nat_add>
   32318:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   3231c:	49 89 c5             	mov    r13,rax
   3231f:	48 85 ff             	test   rdi,rdi
   32322:	0f 94 c0             	sete   al
   32325:	08 c3                	or     bl,al
   32327:	75 47                	jne    32370 <Main.work+0x270>
   32329:	48 8b 4f f8          	mov    rcx,QWORD PTR [rdi-0x8]
   3232d:	48 89 c8             	mov    rax,rcx
   32330:	48 c1 e0 1d          	shl    rax,0x1d
   32334:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   3233a:	7c 14                	jl     32350 <Main.work+0x250>
   3233c:	48 ff c9             	dec    rcx
   3233f:	48 89 4f f8          	mov    QWORD PTR [rdi-0x8],rcx
   32343:	eb 2b                	jmp    32370 <Main.work+0x270>
   32345:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3234c:	00 00 00 00 
   32350:	48 85 c0             	test   rax,rax
   32353:	74 1b                	je     32370 <Main.work+0x270>
   32355:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3235b:	75 07                	jne    32364 <Main.work+0x264>
   3235d:	e8 8e f8 03 00       	call   71bf0 <cinch_drop_slow>
   32362:	eb 0c                	jmp    32370 <Main.work+0x270>
   32364:	e8 97 f7 03 00       	call   71b00 <cinch_drop_mt>
   32369:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   32370:	4d 85 ff             	test   r15,r15
   32373:	0f 94 c0             	sete   al
   32376:	44 89 f9             	mov    ecx,r15d
   32379:	08 c1                	or     cl,al
   3237b:	f6 c1 01             	test   cl,0x1
   3237e:	75 50                	jne    323d0 <Main.work+0x2d0>
   32380:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   32384:	48 89 c8             	mov    rax,rcx
   32387:	48 c1 e0 1d          	shl    rax,0x1d
   3238b:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32391:	7c 1d                	jl     323b0 <Main.work+0x2b0>
   32393:	48 ff c9             	dec    rcx
   32396:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   3239a:	41 f6 c5 01          	test   r13b,0x1
   3239e:	74 36                	je     323d6 <Main.work+0x2d6>
   323a0:	e9 9b 00 00 00       	jmp    32440 <Main.work+0x340>
   323a5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   323ac:	00 00 00 00 
   323b0:	48 85 c0             	test   rax,rax
   323b3:	74 1b                	je     323d0 <Main.work+0x2d0>
   323b5:	4c 89 ff             	mov    rdi,r15
   323b8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   323be:	75 6b                	jne    3242b <Main.work+0x32b>
   323c0:	e8 2b f8 03 00       	call   71bf0 <cinch_drop_slow>
   323c5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   323cc:	00 00 00 00 
   323d0:	41 f6 c5 01          	test   r13b,0x1
   323d4:	75 6a                	jne    32440 <Main.work+0x340>
   323d6:	be 0f 94 35 77       	mov    esi,0x7735940f
   323db:	4c 89 ef             	mov    rdi,r13
   323de:	e8 cd 56 04 00       	call   77ab0 <cinch_nat_rem>
   323e3:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   323e7:	4d 85 ed             	test   r13,r13
   323ea:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   323ee:	0f 84 7c 00 00 00    	je     32470 <Main.work+0x370>
   323f4:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   323f8:	48 89 c8             	mov    rax,rcx
   323fb:	48 c1 e0 1d          	shl    rax,0x1d
   323ff:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32405:	7c 09                	jl     32410 <Main.work+0x310>
   32407:	48 ff c9             	dec    rcx
   3240a:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   3240e:	eb 60                	jmp    32470 <Main.work+0x370>
   32410:	48 85 c0             	test   rax,rax
   32413:	74 5b                	je     32470 <Main.work+0x370>
   32415:	4c 89 ef             	mov    rdi,r13
   32418:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3241e:	0f 85 60 01 00 00    	jne    32584 <Main.work+0x484>
   32424:	e8 c7 f7 03 00       	call   71bf0 <cinch_drop_slow>
   32429:	eb 45                	jmp    32470 <Main.work+0x370>
   3242b:	e8 d0 f6 03 00       	call   71b00 <cinch_drop_mt>
   32430:	41 f6 c5 01          	test   r13b,0x1
   32434:	74 a0                	je     323d6 <Main.work+0x2d6>
   32436:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3243d:	00 00 00 
   32440:	49 d1 ed             	shr    r13,1
   32443:	4c 89 e8             	mov    rax,r13
   32446:	48 b9 e5 8f a2 12 31 	movabs rcx,0x89705f3112a28fe5
   3244d:	5f 70 89 
   32450:	48 f7 e1             	mul    rcx
   32453:	48 c1 ea 1d          	shr    rdx,0x1d
   32457:	69 c2 07 ca 9a 3b    	imul   eax,edx,0x3b9aca07
   3245d:	41 29 c5             	sub    r13d,eax
   32460:	4a 8d 04 6d 01 00 00 	lea    rax,[r13*2+0x1]
   32467:	00 
   32468:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   3246c:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   32470:	41 f6 c4 01          	test   r12b,0x1
   32474:	0f 94 c0             	sete   al
   32477:	49 83 fc 02          	cmp    r12,0x2
   3247b:	0f 92 c1             	setb   cl
   3247e:	08 c1                	or     cl,al
   32480:	74 1e                	je     324a0 <Main.work+0x3a0>
   32482:	be 03 00 00 00       	mov    esi,0x3
   32487:	4c 89 e7             	mov    rdi,r12
   3248a:	e8 61 58 04 00       	call   77cf0 <cinch_nat_sub>
   3248f:	49 89 c5             	mov    r13,rax
   32492:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   32496:	74 13                	je     324ab <Main.work+0x3ab>
   32498:	eb 56                	jmp    324f0 <Main.work+0x3f0>
   3249a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   324a0:	4d 8d 6c 24 fe       	lea    r13,[r12-0x2]
   324a5:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   324a9:	75 45                	jne    324f0 <Main.work+0x3f0>
   324ab:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   324b0:	48 89 c8             	mov    rax,rcx
   324b3:	48 c1 e0 1d          	shl    rax,0x1d
   324b7:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   324bd:	7c 11                	jl     324d0 <Main.work+0x3d0>
   324bf:	48 ff c9             	dec    rcx
   324c2:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   324c7:	41 f6 c7 01          	test   r15b,0x1
   324cb:	75 29                	jne    324f6 <Main.work+0x3f6>
   324cd:	eb 51                	jmp    32520 <Main.work+0x420>
   324cf:	90                   	nop
   324d0:	48 85 c0             	test   rax,rax
   324d3:	74 1b                	je     324f0 <Main.work+0x3f0>
   324d5:	4c 89 e7             	mov    rdi,r12
   324d8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   324de:	75 2b                	jne    3250b <Main.work+0x40b>
   324e0:	e8 0b f7 03 00       	call   71bf0 <cinch_drop_slow>
   324e5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   324ec:	00 00 00 00 
   324f0:	41 f6 c7 01          	test   r15b,0x1
   324f4:	74 2a                	je     32520 <Main.work+0x420>
   324f6:	4c 89 f8             	mov    rax,r15
   324f9:	48 d1 e8             	shr    rax,1
   324fc:	48 ff c0             	inc    rax
   324ff:	78 1f                	js     32520 <Main.work+0x420>
   32501:	48 8d 04 45 01 00 00 	lea    rax,[rax*2+0x1]
   32508:	00 
   32509:	eb 22                	jmp    3252d <Main.work+0x42d>
   3250b:	e8 f0 f5 03 00       	call   71b00 <cinch_drop_mt>
   32510:	41 f6 c7 01          	test   r15b,0x1
   32514:	75 e0                	jne    324f6 <Main.work+0x3f6>
   32516:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3251d:	00 00 00 
   32520:	be 03 00 00 00       	mov    esi,0x3
   32525:	4c 89 ff             	mov    rdi,r15
   32528:	e8 d3 49 04 00       	call   76f00 <cinch_nat_add>
   3252d:	4d 89 ec             	mov    r12,r13
   32530:	41 f6 c6 01          	test   r14b,0x1
   32534:	0f 85 10 fc ff ff    	jne    3214a <Main.work+0x4a>
   3253a:	49 8b 57 f8          	mov    rdx,QWORD PTR [r15-0x8]
   3253e:	48 89 d1             	mov    rcx,rdx
   32541:	48 c1 e1 1d          	shl    rcx,0x1d
   32545:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   3254c:	0f 8d ee fb ff ff    	jge    32140 <Main.work+0x40>
   32552:	4d 89 ec             	mov    r12,r13
   32555:	48 85 c9             	test   rcx,rcx
   32558:	0f 84 ec fb ff ff    	je     3214a <Main.work+0x4a>
   3255e:	48 89 c3             	mov    rbx,rax
   32561:	4c 89 ff             	mov    rdi,r15
   32564:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   3256b:	75 07                	jne    32574 <Main.work+0x474>
   3256d:	e8 7e f6 03 00       	call   71bf0 <cinch_drop_slow>
   32572:	eb 05                	jmp    32579 <Main.work+0x479>
   32574:	e8 87 f5 03 00       	call   71b00 <cinch_drop_mt>
   32579:	4d 89 ec             	mov    r12,r13
   3257c:	48 89 d8             	mov    rax,rbx
   3257f:	e9 c6 fb ff ff       	jmp    3214a <Main.work+0x4a>
   32584:	e8 77 f5 03 00       	call   71b00 <cinch_drop_mt>
   32589:	e9 e2 fe ff ff       	jmp    32470 <Main.work+0x370>
   3258e:	4d 85 ed             	test   r13,r13
   32591:	74 37                	je     325ca <Main.work+0x4ca>
   32593:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   32597:	48 89 c8             	mov    rax,rcx
   3259a:	48 c1 e0 1d          	shl    rax,0x1d
   3259e:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   325a4:	7c 0f                	jl     325b5 <Main.work+0x4b5>
   325a6:	48 ff c9             	dec    rcx
   325a9:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   325ad:	41 f6 c6 01          	test   r14b,0x1
   325b1:	74 1d                	je     325d0 <Main.work+0x4d0>
   325b3:	eb 62                	jmp    32617 <Main.work+0x517>
   325b5:	48 85 c0             	test   rax,rax
   325b8:	74 10                	je     325ca <Main.work+0x4ca>
   325ba:	4c 89 ef             	mov    rdi,r13
   325bd:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   325c3:	75 40                	jne    32605 <Main.work+0x505>
   325c5:	e8 26 f6 03 00       	call   71bf0 <cinch_drop_slow>
   325ca:	41 f6 c6 01          	test   r14b,0x1
   325ce:	75 47                	jne    32617 <Main.work+0x517>
   325d0:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   325d5:	48 89 c8             	mov    rax,rcx
   325d8:	48 c1 e0 1d          	shl    rax,0x1d
   325dc:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   325e2:	7c 0a                	jl     325ee <Main.work+0x4ee>
   325e4:	48 ff c9             	dec    rcx
   325e7:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   325ec:	eb 29                	jmp    32617 <Main.work+0x517>
   325ee:	48 85 c0             	test   rax,rax
   325f1:	74 24                	je     32617 <Main.work+0x517>
   325f3:	4c 89 e7             	mov    rdi,r12
   325f6:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   325fc:	75 14                	jne    32612 <Main.work+0x512>
   325fe:	e8 ed f5 03 00       	call   71bf0 <cinch_drop_slow>
   32603:	eb 12                	jmp    32617 <Main.work+0x517>
   32605:	e8 f6 f4 03 00       	call   71b00 <cinch_drop_mt>
   3260a:	41 f6 c6 01          	test   r14b,0x1
   3260e:	74 c0                	je     325d0 <Main.work+0x4d0>
   32610:	eb 05                	jmp    32617 <Main.work+0x517>
   32612:	e8 e9 f4 03 00       	call   71b00 <cinch_drop_mt>
   32617:	4d 85 ff             	test   r15,r15
   3261a:	0f 94 c0             	sete   al
   3261d:	44 89 f9             	mov    ecx,r15d
   32620:	08 c1                	or     cl,al
   32622:	f6 c1 01             	test   cl,0x1
   32625:	75 38                	jne    3265f <Main.work+0x55f>
   32627:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   3262b:	48 89 c8             	mov    rax,rcx
   3262e:	48 c1 e0 1d          	shl    rax,0x1d
   32632:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32638:	7c 09                	jl     32643 <Main.work+0x543>
   3263a:	48 ff c9             	dec    rcx
   3263d:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   32641:	eb 1c                	jmp    3265f <Main.work+0x55f>
   32643:	48 85 c0             	test   rax,rax
   32646:	74 17                	je     3265f <Main.work+0x55f>
   32648:	4c 89 ff             	mov    rdi,r15
   3264b:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   32651:	75 07                	jne    3265a <Main.work+0x55a>
   32653:	e8 98 f5 03 00       	call   71bf0 <cinch_drop_slow>
   32658:	eb 05                	jmp    3265f <Main.work+0x55f>
   3265a:	e8 a1 f4 03 00       	call   71b00 <cinch_drop_mt>
   3265f:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
   32663:	48 83 c4 48          	add    rsp,0x48
   32667:	5b                   	pop    rbx
   32668:	41 5c                	pop    r12
   3266a:	41 5d                	pop    r13
   3266c:	41 5e                	pop    r14
   3266e:	41 5f                	pop    r15
   32670:	5d                   	pop    rbp
   32671:	c3                   	ret

Disassembly of section .fini:
