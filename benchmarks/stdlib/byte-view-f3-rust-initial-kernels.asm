
build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000293f0 <Data.Bytes.View.endpoint-blocks>:
   293f0:	55                   	push   rbp
   293f1:	48 89 e5             	mov    rbp,rsp
   293f4:	48 83 ec 10          	sub    rsp,0x10
   293f8:	e8 73 fe ff ff       	call   29270 <Data.Bytes.View.endpoint-blocks#cc>
   293fd:	48 83 c4 08          	add    rsp,0x8
   29401:	5d                   	pop    rbp
   29402:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029270 <Data.Bytes.View.endpoint-blocks#cc>:
   29270:	55                   	push   rbp
   29271:	48 89 e5             	mov    rbp,rsp
   29274:	41 57                	push   r15
   29276:	41 56                	push   r14
   29278:	53                   	push   rbx
   29279:	50                   	push   rax
   2927a:	48 89 d0             	mov    rax,rdx
   2927d:	48 29 f0             	sub    rax,rsi
   29280:	48 83 f8 08          	cmp    rax,0x8
   29284:	7d 30                	jge    292b6 <Data.Bytes.View.endpoint-blocks#cc+0x46>
   29286:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   2928d:	48 39 f2             	cmp    rdx,rsi
   29290:	0f 8e b2 00 00 00    	jle    29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   29296:	44 38 04 37          	cmp    BYTE PTR [rdi+rsi*1],r8b
   2929a:	0f 85 98 00 00 00    	jne    29338 <Data.Bytes.View.endpoint-blocks#cc+0xc8>
   292a0:	48 8d 04 37          	lea    rax,[rdi+rsi*1]
   292a4:	44 38 0c 01          	cmp    BYTE PTR [rcx+rax*1],r9b
   292a8:	0f 85 8a 00 00 00    	jne    29338 <Data.Bytes.View.endpoint-blocks#cc+0xc8>
   292ae:	48 89 f0             	mov    rax,rsi
   292b1:	e9 92 00 00 00       	jmp    29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   292b6:	49 bb 80 80 80 80 80 	movabs r11,0x8080808080808080
   292bd:	80 80 80 
   292c0:	48 b8 7f 7f 7f 7f 7f 	movabs rax,0x7f7f7f7f7f7f7f7f
   292c7:	7f 7f 7f 
   292ca:	48 8b 1c 37          	mov    rbx,QWORD PTR [rdi+rsi*1]
   292ce:	4c 31 c3             	xor    rbx,r8
   292d1:	49 89 da             	mov    r10,rbx
   292d4:	49 21 c2             	and    r10,rax
   292d7:	49 01 c2             	add    r10,rax
   292da:	49 09 da             	or     r10,rbx
   292dd:	4d 21 da             	and    r10,r11
   292e0:	4d 39 da             	cmp    r10,r11
   292e3:	75 24                	jne    29309 <Data.Bytes.View.endpoint-blocks#cc+0x99>
   292e5:	48 89 f8             	mov    rax,rdi
   292e8:	48 89 f7             	mov    rdi,rsi
   292eb:	48 89 d6             	mov    rsi,rdx
   292ee:	48 89 c2             	mov    rdx,rax
   292f1:	48 89 c8             	mov    rax,rcx
   292f4:	4c 89 c1             	mov    rcx,r8
   292f7:	49 89 c0             	mov    r8,rax
   292fa:	48 83 c4 08          	add    rsp,0x8
   292fe:	5b                   	pop    rbx
   292ff:	41 5e                	pop    r14
   29301:	41 5f                	pop    r15
   29303:	5d                   	pop    rbp
   29304:	e9 87 c1 00 00       	jmp    35490 <Data.Bytes.View.endpoint-blocks.shared0#cc>
   29309:	4d 31 da             	xor    r10,r11
   2930c:	48 8d 1c 0f          	lea    rbx,[rdi+rcx*1]
   29310:	4c 8b 3c 1e          	mov    r15,QWORD PTR [rsi+rbx*1]
   29314:	4d 31 cf             	xor    r15,r9
   29317:	4d 89 fe             	mov    r14,r15
   2931a:	49 21 c6             	and    r14,rax
   2931d:	49 01 c6             	add    r14,rax
   29320:	4d 09 fe             	or     r14,r15
   29323:	49 f7 d6             	not    r14
   29326:	4d 21 d6             	and    r14,r10
   29329:	74 2a                	je     29355 <Data.Bytes.View.endpoint-blocks#cc+0xe5>
   2932b:	f3 49 0f bc c6       	tzcnt  rax,r14
   29330:	c1 e8 03             	shr    eax,0x3
   29333:	48 01 f0             	add    rax,rsi
   29336:	eb 10                	jmp    29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   29338:	45 0f b6 c9          	movzx  r9d,r9b
   2933c:	45 0f b6 c0          	movzx  r8d,r8b
   29340:	48 ff c6             	inc    rsi
   29343:	e8 c8 00 00 00       	call   29410 <Data.Bytes.View.endpoint-tail>
   29348:	48 83 c4 08          	add    rsp,0x8
   2934c:	5b                   	pop    rbx
   2934d:	41 5e                	pop    r14
   2934f:	41 5f                	pop    r15
   29351:	5d                   	pop    rbp
   29352:	c2 08 00             	ret    0x8
   29355:	4c 8d 56 08          	lea    r10,[rsi+0x8]
   29359:	49 89 d6             	mov    r14,rdx
   2935c:	4d 29 d6             	sub    r14,r10
   2935f:	49 83 fe 08          	cmp    r14,0x8
   29363:	7d 29                	jge    2938e <Data.Bytes.View.endpoint-blocks#cc+0x11e>
   29365:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   2936c:	4c 39 d2             	cmp    rdx,r10
   2936f:	7e d7                	jle    29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   29371:	46 38 04 17          	cmp    BYTE PTR [rdi+r10*1],r8b
   29375:	75 09                	jne    29380 <Data.Bytes.View.endpoint-blocks#cc+0x110>
   29377:	4c 89 d0             	mov    rax,r10
   2937a:	46 38 0c 13          	cmp    BYTE PTR [rbx+r10*1],r9b
   2937e:	74 c8                	je     29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   29380:	45 0f b6 c9          	movzx  r9d,r9b
   29384:	45 0f b6 c0          	movzx  r8d,r8b
   29388:	48 83 c6 09          	add    rsi,0x9
   2938c:	eb b5                	jmp    29343 <Data.Bytes.View.endpoint-blocks#cc+0xd3>
   2938e:	4e 8b 34 17          	mov    r14,QWORD PTR [rdi+r10*1]
   29392:	4d 31 c6             	xor    r14,r8
   29395:	4c 89 f6             	mov    rsi,r14
   29398:	48 21 c6             	and    rsi,rax
   2939b:	48 01 c6             	add    rsi,rax
   2939e:	4c 09 f6             	or     rsi,r14
   293a1:	4c 21 de             	and    rsi,r11
   293a4:	4c 39 de             	cmp    rsi,r11
   293a7:	74 2e                	je     293d7 <Data.Bytes.View.endpoint-blocks#cc+0x167>
   293a9:	4c 31 de             	xor    rsi,r11
   293ac:	4a 8b 1c 13          	mov    rbx,QWORD PTR [rbx+r10*1]
   293b0:	4c 31 cb             	xor    rbx,r9
   293b3:	49 89 db             	mov    r11,rbx
   293b6:	49 21 c3             	and    r11,rax
   293b9:	49 01 c3             	add    r11,rax
   293bc:	49 09 db             	or     r11,rbx
   293bf:	49 f7 d3             	not    r11
   293c2:	49 21 f3             	and    r11,rsi
   293c5:	74 10                	je     293d7 <Data.Bytes.View.endpoint-blocks#cc+0x167>
   293c7:	f3 49 0f bc c3       	tzcnt  rax,r11
   293cc:	c1 e8 03             	shr    eax,0x3
   293cf:	4c 01 d0             	add    rax,r10
   293d2:	e9 71 ff ff ff       	jmp    29348 <Data.Bytes.View.endpoint-blocks#cc+0xd8>
   293d7:	48 89 f8             	mov    rax,rdi
   293da:	4c 89 d7             	mov    rdi,r10
   293dd:	e9 09 ff ff ff       	jmp    292eb <Data.Bytes.View.endpoint-blocks#cc+0x7b>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035550 <Data.Bytes.View.endpoint-blocks.shared0>:
   35550:	55                   	push   rbp
   35551:	48 89 e5             	mov    rbp,rsp
   35554:	48 83 ec 10          	sub    rsp,0x10
   35558:	e8 33 ff ff ff       	call   35490 <Data.Bytes.View.endpoint-blocks.shared0#cc>
   3555d:	48 83 c4 08          	add    rsp,0x8
   35561:	5d                   	pop    rbp
   35562:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035490 <Data.Bytes.View.endpoint-blocks.shared0#cc>:
   35490:	55                   	push   rbp
   35491:	48 89 e5             	mov    rbp,rsp
   35494:	41 56                	push   r14
   35496:	53                   	push   rbx
   35497:	48 89 f0             	mov    rax,rsi
   3549a:	48 8d 77 08          	lea    rsi,[rdi+0x8]
   3549e:	49 89 c2             	mov    r10,rax
   354a1:	49 29 f2             	sub    r10,rsi
   354a4:	49 83 fa 08          	cmp    r10,0x8
   354a8:	7d 1b                	jge    354c5 <Data.Bytes.View.endpoint-blocks.shared0#cc+0x35>
   354aa:	44 0f b6 d1          	movzx  r10d,cl
   354ae:	45 0f b6 c9          	movzx  r9d,r9b
   354b2:	48 89 d7             	mov    rdi,rdx
   354b5:	48 89 c2             	mov    rdx,rax
   354b8:	4c 89 c1             	mov    rcx,r8
   354bb:	4d 89 d0             	mov    r8,r10
   354be:	e8 4d 3f ff ff       	call   29410 <Data.Bytes.View.endpoint-tail>
   354c3:	eb 5c                	jmp    35521 <Data.Bytes.View.endpoint-blocks.shared0#cc+0x91>
   354c5:	48 bb 80 80 80 80 80 	movabs rbx,0x8080808080808080
   354cc:	80 80 80 
   354cf:	49 bb 7f 7f 7f 7f 7f 	movabs r11,0x7f7f7f7f7f7f7f7f
   354d6:	7f 7f 7f 
   354d9:	4c 8b 34 32          	mov    r14,QWORD PTR [rdx+rsi*1]
   354dd:	49 31 ce             	xor    r14,rcx
   354e0:	4d 89 f2             	mov    r10,r14
   354e3:	4d 21 da             	and    r10,r11
   354e6:	4d 01 da             	add    r10,r11
   354e9:	4d 09 f2             	or     r10,r14
   354ec:	49 21 da             	and    r10,rbx
   354ef:	49 39 da             	cmp    r10,rbx
   354f2:	74 34                	je     35528 <Data.Bytes.View.endpoint-blocks.shared0#cc+0x98>
   354f4:	49 31 da             	xor    r10,rbx
   354f7:	4a 8d 1c 02          	lea    rbx,[rdx+r8*1]
   354fb:	4c 8b 34 1e          	mov    r14,QWORD PTR [rsi+rbx*1]
   354ff:	4d 31 ce             	xor    r14,r9
   35502:	4c 89 f3             	mov    rbx,r14
   35505:	4c 21 db             	and    rbx,r11
   35508:	4c 01 db             	add    rbx,r11
   3550b:	4c 09 f3             	or     rbx,r14
   3550e:	48 f7 d3             	not    rbx
   35511:	4c 21 d3             	and    rbx,r10
   35514:	74 12                	je     35528 <Data.Bytes.View.endpoint-blocks.shared0#cc+0x98>
   35516:	f3 48 0f bc c3       	tzcnt  rax,rbx
   3551b:	c1 e8 03             	shr    eax,0x3
   3551e:	48 01 f0             	add    rax,rsi
   35521:	5b                   	pop    rbx
   35522:	41 5e                	pop    r14
   35524:	5d                   	pop    rbp
   35525:	c2 08 00             	ret    0x8
   35528:	48 83 c7 10          	add    rdi,0x10
   3552c:	48 89 fe             	mov    rsi,rdi
   3552f:	48 89 d7             	mov    rdi,rdx
   35532:	48 89 c2             	mov    rdx,rax
   35535:	48 89 c8             	mov    rax,rcx
   35538:	4c 89 c1             	mov    rcx,r8
   3553b:	49 89 c0             	mov    r8,rax
   3553e:	5b                   	pop    rbx
   3553f:	41 5e                	pop    r14
   35541:	5d                   	pop    rbp
   35542:	e9 29 3d ff ff       	jmp    29270 <Data.Bytes.View.endpoint-blocks#cc>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035570 <Data.Bytes.View.endpoint-blocks.shared1>:
   35570:	55                   	push   rbp
   35571:	48 89 e5             	mov    rbp,rsp
   35574:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   3557b:	48 39 ca             	cmp    rdx,rcx
   3557e:	7d 13                	jge    35593 <Data.Bytes.View.endpoint-blocks.shared1+0x23>
   35580:	41 38 3c 10          	cmp    BYTE PTR [r8+rdx*1],dil
   35584:	75 0f                	jne    35595 <Data.Bytes.View.endpoint-blocks.shared1+0x25>
   35586:	49 8d 04 10          	lea    rax,[r8+rdx*1]
   3558a:	41 38 34 01          	cmp    BYTE PTR [r9+rax*1],sil
   3558e:	75 05                	jne    35595 <Data.Bytes.View.endpoint-blocks.shared1+0x25>
   35590:	48 89 d0             	mov    rax,rdx
   35593:	5d                   	pop    rbp
   35594:	c3                   	ret
   35595:	40 0f b6 c6          	movzx  eax,sil
   35599:	44 0f b6 d7          	movzx  r10d,dil
   3559d:	48 ff c2             	inc    rdx
   355a0:	4c 89 c7             	mov    rdi,r8
   355a3:	48 89 d6             	mov    rsi,rdx
   355a6:	48 89 ca             	mov    rdx,rcx
   355a9:	4c 89 c9             	mov    rcx,r9
   355ac:	4d 89 d0             	mov    r8,r10
   355af:	49 89 c1             	mov    r9,rax
   355b2:	5d                   	pop    rbp
   355b3:	e9 58 3e ff ff       	jmp    29410 <Data.Bytes.View.endpoint-tail>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


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

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000002b880 <Data.Bytes.View.search-count>:
   2b880:	55                   	push   rbp
   2b881:	48 89 e5             	mov    rbp,rsp
   2b884:	41 57                	push   r15
   2b886:	41 56                	push   r14
   2b888:	41 55                	push   r13
   2b88a:	41 54                	push   r12
   2b88c:	53                   	push   rbx
   2b88d:	48 83 ec 18          	sub    rsp,0x18
   2b891:	4c 89 4d d0          	mov    QWORD PTR [rbp-0x30],r9
   2b895:	49 89 cf             	mov    r15,rcx
   2b898:	49 89 d4             	mov    r12,rdx
   2b89b:	49 89 f6             	mov    r14,rsi
   2b89e:	48 89 fb             	mov    rbx,rdi
   2b8a1:	4c 8b 6d 20          	mov    r13,QWORD PTR [rbp+0x20]
   2b8a5:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   2b8a9:	4c 89 45 c0          	mov    QWORD PTR [rbp-0x40],r8
   2b8ad:	49 8d 48 f8          	lea    rcx,[r8-0x8]
   2b8b1:	48 89 4d c8          	mov    QWORD PTR [rbp-0x38],rcx
   2b8b5:	eb 26                	jmp    2b8dd <Data.Bytes.View.search-count+0x5d>
   2b8b7:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   2b8be:	00 00 
   2b8c0:	49 83 c6 08          	add    r14,0x8
   2b8c4:	48 83 c0 08          	add    rax,0x8
   2b8c8:	31 c9                	xor    ecx,ecx
   2b8ca:	48 3b 45 c0          	cmp    rax,QWORD PTR [rbp-0x40]
   2b8ce:	0f 9d c1             	setge  cl
   2b8d1:	ba 00 00 00 00       	mov    edx,0x0
   2b8d6:	48 0f 4d c2          	cmovge rax,rdx
   2b8da:	49 01 cd             	add    r13,rcx
   2b8dd:	48 85 c0             	test   rax,rax
   2b8e0:	75 22                	jne    2b904 <Data.Bytes.View.search-count+0x84>
   2b8e2:	48 89 df             	mov    rdi,rbx
   2b8e5:	4c 89 f6             	mov    rsi,r14
   2b8e8:	4c 89 e2             	mov    rdx,r12
   2b8eb:	48 8b 4d 10          	mov    rcx,QWORD PTR [rbp+0x10]
   2b8ef:	e8 2c cd ff ff       	call   28620 <Data.Bytes.View.index-forward>
   2b8f4:	48 85 c0             	test   rax,rax
   2b8f7:	78 56                	js     2b94f <Data.Bytes.View.search-count+0xcf>
   2b8f9:	49 89 c6             	mov    r14,rax
   2b8fc:	49 ff c6             	inc    r14
   2b8ff:	b8 01 00 00 00       	mov    eax,0x1
   2b904:	4c 89 e1             	mov    rcx,r12
   2b907:	4c 29 f1             	sub    rcx,r14
   2b90a:	7e 43                	jle    2b94f <Data.Bytes.View.search-count+0xcf>
   2b90c:	48 3b 45 c8          	cmp    rax,QWORD PTR [rbp-0x38]
   2b910:	7f 1e                	jg     2b930 <Data.Bytes.View.search-count+0xb0>
   2b912:	48 83 f9 07          	cmp    rcx,0x7
   2b916:	7e 18                	jle    2b930 <Data.Bytes.View.search-count+0xb0>
   2b918:	4a 8b 0c 33          	mov    rcx,QWORD PTR [rbx+r14*1]
   2b91c:	49 3b 0c 07          	cmp    rcx,QWORD PTR [r15+rax*1]
   2b920:	74 9e                	je     2b8c0 <Data.Bytes.View.search-count+0x40>
   2b922:	eb 11                	jmp    2b935 <Data.Bytes.View.search-count+0xb5>
   2b924:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   2b92b:	00 00 00 00 00 
   2b930:	42 0f b6 0c 33       	movzx  ecx,BYTE PTR [rbx+r14*1]
   2b935:	0f b6 c9             	movzx  ecx,cl
   2b938:	4c 89 ff             	mov    rdi,r15
   2b93b:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   2b93f:	48 89 c2             	mov    rdx,rax
   2b942:	e8 39 dc ff ff       	call   29580 <Data.Bytes.View.search-step>
   2b947:	49 ff c6             	inc    r14
   2b94a:	e9 79 ff ff ff       	jmp    2b8c8 <Data.Bytes.View.search-count+0x48>
   2b94f:	4c 89 e8             	mov    rax,r13
   2b952:	48 83 c4 18          	add    rsp,0x18
   2b956:	5b                   	pop    rbx
   2b957:	41 5c                	pop    r12
   2b959:	41 5d                	pop    r13
   2b95b:	41 5e                	pop    r14
   2b95d:	41 5f                	pop    r15
   2b95f:	5d                   	pop    rbp
   2b960:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029a90 <Data.Bytes.View.search-first>:
   29a90:	55                   	push   rbp
   29a91:	48 89 e5             	mov    rbp,rsp
   29a94:	48 83 ec 20          	sub    rsp,0x20
   29a98:	0f 28 45 10          	movaps xmm0,XMMWORD PTR [rbp+0x10]
   29a9c:	0f 11 04 24          	movups XMMWORD PTR [rsp],xmm0
   29aa0:	e8 bb fb ff ff       	call   29660 <Data.Bytes.View.search-first#cc>
   29aa5:	48 83 c4 08          	add    rsp,0x8
   29aa9:	5d                   	pop    rbp
   29aaa:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000029660 <Data.Bytes.View.search-first#cc>:
   29660:	55                   	push   rbp
   29661:	48 89 e5             	mov    rbp,rsp
   29664:	41 57                	push   r15
   29666:	41 56                	push   r14
   29668:	41 55                	push   r13
   2966a:	41 54                	push   r12
   2966c:	53                   	push   rbx
   2966d:	48 83 ec 18          	sub    rsp,0x18
   29671:	4c 89 4d d0          	mov    QWORD PTR [rbp-0x30],r9
   29675:	4d 89 c6             	mov    r14,r8
   29678:	49 89 c8             	mov    r8,rcx
   2967b:	48 89 d3             	mov    rbx,rdx
   2967e:	49 89 fc             	mov    r12,rdi
   29681:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   29685:	4c 8b 4d 10          	mov    r9,QWORD PTR [rbp+0x10]
   29689:	48 85 c0             	test   rax,rax
   2968c:	4c 89 4d c8          	mov    QWORD PTR [rbp-0x38],r9
   29690:	74 48                	je     296da <Data.Bytes.View.search-first#cc+0x7a>
   29692:	49 89 f5             	mov    r13,rsi
   29695:	49 c7 c7 ff ff ff ff 	mov    r15,0xffffffffffffffff
   2969c:	48 89 d9             	mov    rcx,rbx
   2969f:	4c 29 e9             	sub    rcx,r13
   296a2:	0f 8e 2a 03 00 00    	jle    299d2 <Data.Bytes.View.search-first#cc+0x372>
   296a8:	49 8d 56 f8          	lea    rdx,[r14-0x8]
   296ac:	48 39 d0             	cmp    rax,rdx
   296af:	0f 8f 6f 02 00 00    	jg     29924 <Data.Bytes.View.search-first#cc+0x2c4>
   296b5:	48 83 f9 08          	cmp    rcx,0x8
   296b9:	0f 8c 65 02 00 00    	jl     29924 <Data.Bytes.View.search-first#cc+0x2c4>
   296bf:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   296c3:	49 3b 0c 00          	cmp    rcx,QWORD PTR [r8+rax*1]
   296c7:	0f 85 57 02 00 00    	jne    29924 <Data.Bytes.View.search-first#cc+0x2c4>
   296cd:	49 83 c5 08          	add    r13,0x8
   296d1:	48 83 c0 08          	add    rax,0x8
   296d5:	e9 53 03 00 00       	jmp    29a2d <Data.Bytes.View.search-first#cc+0x3cd>
   296da:	41 0f b6 c1          	movzx  eax,r9b
   296de:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
   296e2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   296e9:	1f 84 00 00 00 00 00 
   296f0:	49 c7 c7 ff ff ff ff 	mov    r15,0xffffffffffffffff
   296f7:	48 39 de             	cmp    rsi,rbx
   296fa:	0f 8d d2 02 00 00    	jge    299d2 <Data.Bytes.View.search-first#cc+0x372>
   29700:	45 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],r9b
   29704:	75 1a                	jne    29720 <Data.Bytes.View.search-first#cc+0xc0>
   29706:	49 89 f5             	mov    r13,rsi
   29709:	4d 85 ed             	test   r13,r13
   2970c:	0f 89 27 01 00 00    	jns    29839 <Data.Bytes.View.search-first#cc+0x1d9>
   29712:	e9 01 02 00 00       	jmp    29918 <Data.Bytes.View.search-first#cc+0x2b8>
   29717:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   2971e:	00 00 
   29720:	4c 8d 6e 01          	lea    r13,[rsi+0x1]
   29724:	48 89 d8             	mov    rax,rbx
   29727:	4c 29 e8             	sub    rax,r13
   2972a:	48 83 f8 08          	cmp    rax,0x8
   2972e:	7d 30                	jge    29760 <Data.Bytes.View.search-first#cc+0x100>
   29730:	4c 39 eb             	cmp    rbx,r13
   29733:	0f 8e 99 02 00 00    	jle    299d2 <Data.Bytes.View.search-first#cc+0x372>
   29739:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   2973d:	0f 84 ed 00 00 00    	je     29830 <Data.Bytes.View.search-first#cc+0x1d0>
   29743:	4c 8d 6e 02          	lea    r13,[rsi+0x2]
   29747:	49 39 dd             	cmp    r13,rbx
   2974a:	0f 8d 82 02 00 00    	jge    299d2 <Data.Bytes.View.search-first#cc+0x372>
   29750:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   29754:	0f 84 d6 00 00 00    	je     29830 <Data.Bytes.View.search-first#cc+0x1d0>
   2975a:	48 83 c6 03          	add    rsi,0x3
   2975e:	eb 5a                	jmp    297ba <Data.Bytes.View.search-first#cc+0x15a>
   29760:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   29764:	4c 31 c9             	xor    rcx,r9
   29767:	48 89 c8             	mov    rax,rcx
   2976a:	48 ba 7f 7f 7f 7f 7f 	movabs rdx,0x7f7f7f7f7f7f7f7f
   29771:	7f 7f 7f 
   29774:	48 21 d0             	and    rax,rdx
   29777:	48 01 d0             	add    rax,rdx
   2977a:	48 09 c8             	or     rax,rcx
   2977d:	48 b9 80 80 80 80 80 	movabs rcx,0x8080808080808080
   29784:	80 80 80 
   29787:	48 21 c8             	and    rax,rcx
   2978a:	48 39 c8             	cmp    rax,rcx
   2978d:	0f 85 8a 00 00 00    	jne    2981d <Data.Bytes.View.search-first#cc+0x1bd>
   29793:	4c 8d 6e 09          	lea    r13,[rsi+0x9]
   29797:	48 89 d8             	mov    rax,rbx
   2979a:	4c 29 e8             	sub    rax,r13
   2979d:	48 83 f8 08          	cmp    rax,0x8
   297a1:	7d 39                	jge    297dc <Data.Bytes.View.search-first#cc+0x17c>
   297a3:	4c 39 eb             	cmp    rbx,r13
   297a6:	0f 8e 26 02 00 00    	jle    299d2 <Data.Bytes.View.search-first#cc+0x372>
   297ac:	47 38 0c 2c          	cmp    BYTE PTR [r12+r13*1],r9b
   297b0:	0f 84 7a 00 00 00    	je     29830 <Data.Bytes.View.search-first#cc+0x1d0>
   297b6:	48 83 c6 0a          	add    rsi,0xa
   297ba:	4c 89 e7             	mov    rdi,r12
   297bd:	48 89 da             	mov    rdx,rbx
   297c0:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
   297c4:	4d 89 c5             	mov    r13,r8
   297c7:	e8 64 f1 ff ff       	call   28930 <Data.Bytes.View.index-forward-tail>
   297cc:	4d 89 e8             	mov    r8,r13
   297cf:	49 89 c5             	mov    r13,rax
   297d2:	4d 85 ed             	test   r13,r13
   297d5:	79 62                	jns    29839 <Data.Bytes.View.search-first#cc+0x1d9>
   297d7:	e9 3c 01 00 00       	jmp    29918 <Data.Bytes.View.search-first#cc+0x2b8>
   297dc:	4b 8b 0c 2c          	mov    rcx,QWORD PTR [r12+r13*1]
   297e0:	4c 31 c9             	xor    rcx,r9
   297e3:	48 89 c8             	mov    rax,rcx
   297e6:	48 21 d0             	and    rax,rdx
   297e9:	48 01 d0             	add    rax,rdx
   297ec:	48 09 c8             	or     rax,rcx
   297ef:	48 b9 80 80 80 80 80 	movabs rcx,0x8080808080808080
   297f6:	80 80 80 
   297f9:	48 21 c8             	and    rax,rcx
   297fc:	48 39 c8             	cmp    rax,rcx
   297ff:	75 1c                	jne    2981d <Data.Bytes.View.search-first#cc+0x1bd>
   29801:	48 8d 46 11          	lea    rax,[rsi+0x11]
   29805:	48 89 d9             	mov    rcx,rbx
   29808:	48 29 c1             	sub    rcx,rax
   2980b:	48 83 f9 08          	cmp    rcx,0x8
   2980f:	0f 8d 9c 00 00 00    	jge    298b1 <Data.Bytes.View.search-first#cc+0x251>
   29815:	4c 89 e7             	mov    rdi,r12
   29818:	48 89 c6             	mov    rsi,rax
   2981b:	eb a0                	jmp    297bd <Data.Bytes.View.search-first#cc+0x15d>
   2981d:	48 31 c8             	xor    rax,rcx
   29820:	b9 40 00 00 00       	mov    ecx,0x40
   29825:	f3 48 0f bc c8       	tzcnt  rcx,rax
   2982a:	c1 e9 03             	shr    ecx,0x3
   2982d:	49 01 cd             	add    r13,rcx
   29830:	4d 85 ed             	test   r13,r13
   29833:	0f 88 df 00 00 00    	js     29918 <Data.Bytes.View.search-first#cc+0x2b8>
   29839:	49 83 fe 01          	cmp    r14,0x1
   2983d:	0f 8e d5 00 00 00    	jle    29918 <Data.Bytes.View.search-first#cc+0x2b8>
   29843:	49 8d 45 01          	lea    rax,[r13+0x1]
   29847:	48 89 d9             	mov    rcx,rbx
   2984a:	48 29 c1             	sub    rcx,rax
   2984d:	0f 8e 7f 01 00 00    	jle    299d2 <Data.Bytes.View.search-first#cc+0x372>
   29853:	49 83 fe 09          	cmp    r14,0x9
   29857:	7c 17                	jl     29870 <Data.Bytes.View.search-first#cc+0x210>
   29859:	48 83 f9 07          	cmp    rcx,0x7
   2985d:	7e 11                	jle    29870 <Data.Bytes.View.search-first#cc+0x210>
   2985f:	49 8b 04 04          	mov    rax,QWORD PTR [r12+rax*1]
   29863:	49 3b 40 01          	cmp    rax,QWORD PTR [r8+0x1]
   29867:	75 0c                	jne    29875 <Data.Bytes.View.search-first#cc+0x215>
   29869:	e9 5b 01 00 00       	jmp    299c9 <Data.Bytes.View.search-first#cc+0x369>
   2986e:	66 90                	xchg   ax,ax
   29870:	41 0f b6 04 04       	movzx  eax,BYTE PTR [r12+rax*1]
   29875:	0f b6 c8             	movzx  ecx,al
   29878:	ba 01 00 00 00       	mov    edx,0x1
   2987d:	4d 89 c7             	mov    r15,r8
   29880:	4c 89 c7             	mov    rdi,r8
   29883:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   29887:	e8 f4 fc ff ff       	call   29580 <Data.Bytes.View.search-step>
   2988c:	49 83 c5 02          	add    r13,0x2
   29890:	4c 39 f0             	cmp    rax,r14
   29893:	0f 8d 2b 01 00 00    	jge    299c4 <Data.Bytes.View.search-first#cc+0x364>
   29899:	4c 89 ee             	mov    rsi,r13
   2989c:	48 85 c0             	test   rax,rax
   2989f:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   298a3:	4d 89 f8             	mov    r8,r15
   298a6:	0f 84 44 fe ff ff    	je     296f0 <Data.Bytes.View.search-first#cc+0x90>
   298ac:	e9 e4 fd ff ff       	jmp    29695 <Data.Bytes.View.search-first#cc+0x35>
   298b1:	49 8b 14 04          	mov    rdx,QWORD PTR [r12+rax*1]
   298b5:	4c 31 ca             	xor    rdx,r9
   298b8:	48 89 d1             	mov    rcx,rdx
   298bb:	48 bf 7f 7f 7f 7f 7f 	movabs rdi,0x7f7f7f7f7f7f7f7f
   298c2:	7f 7f 7f 
   298c5:	48 21 f9             	and    rcx,rdi
   298c8:	48 01 f9             	add    rcx,rdi
   298cb:	48 09 d1             	or     rcx,rdx
   298ce:	48 ba 80 80 80 80 80 	movabs rdx,0x8080808080808080
   298d5:	80 80 80 
   298d8:	48 21 d1             	and    rcx,rdx
   298db:	48 39 d1             	cmp    rcx,rdx
   298de:	75 1a                	jne    298fa <Data.Bytes.View.search-first#cc+0x29a>
   298e0:	48 83 c6 19          	add    rsi,0x19
   298e4:	4c 89 e7             	mov    rdi,r12
   298e7:	48 89 da             	mov    rdx,rbx
   298ea:	4c 89 c9             	mov    rcx,r9
   298ed:	4d 89 c5             	mov    r13,r8
   298f0:	e8 8b ee ff ff       	call   28780 <Data.Bytes.View.index-forward-blocks>
   298f5:	e9 d2 fe ff ff       	jmp    297cc <Data.Bytes.View.search-first#cc+0x16c>
   298fa:	48 31 d1             	xor    rcx,rdx
   298fd:	41 bd 40 00 00 00    	mov    r13d,0x40
   29903:	f3 4c 0f bc e9       	tzcnt  r13,rcx
   29908:	41 c1 ed 03          	shr    r13d,0x3
   2990c:	49 01 c5             	add    r13,rax
   2990f:	4d 85 ed             	test   r13,r13
   29912:	0f 89 21 ff ff ff    	jns    29839 <Data.Bytes.View.search-first#cc+0x1d9>
   29918:	4d 85 ed             	test   r13,r13
   2991b:	4d 0f 49 fd          	cmovns r15,r13
   2991f:	e9 ae 00 00 00       	jmp    299d2 <Data.Bytes.View.search-first#cc+0x372>
   29924:	43 0f b6 0c 2c       	movzx  ecx,BYTE PTR [r12+r13*1]
   29929:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   2992d:	0f 84 8c 00 00 00    	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   29933:	48 85 c0             	test   rax,rax
   29936:	0f 8e ec 00 00 00    	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   2993c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29940:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29945:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29949:	74 74                	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   2994b:	48 85 c0             	test   rax,rax
   2994e:	0f 8e d4 00 00 00    	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   29954:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29958:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   2995d:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29961:	74 5c                	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   29963:	48 85 c0             	test   rax,rax
   29966:	0f 8e bc 00 00 00    	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   2996c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29970:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29975:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29979:	74 44                	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   2997b:	48 85 c0             	test   rax,rax
   2997e:	0f 8e a4 00 00 00    	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   29984:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29988:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   2998d:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29991:	74 2c                	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   29993:	48 85 c0             	test   rax,rax
   29996:	0f 8e 8c 00 00 00    	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   2999c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   299a0:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   299a5:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   299a9:	74 14                	je     299bf <Data.Bytes.View.search-first#cc+0x35f>
   299ab:	48 85 c0             	test   rax,rax
   299ae:	7e 78                	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   299b0:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   299b4:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   299b9:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   299bd:	75 39                	jne    299f8 <Data.Bytes.View.search-first#cc+0x398>
   299bf:	48 ff c0             	inc    rax
   299c2:	eb 66                	jmp    29a2a <Data.Bytes.View.search-first#cc+0x3ca>
   299c4:	4d 29 f5             	sub    r13,r14
   299c7:	eb 06                	jmp    299cf <Data.Bytes.View.search-first#cc+0x36f>
   299c9:	49 83 fe 09          	cmp    r14,0x9
   299cd:	75 17                	jne    299e6 <Data.Bytes.View.search-first#cc+0x386>
   299cf:	4d 89 ef             	mov    r15,r13
   299d2:	4c 89 f8             	mov    rax,r15
   299d5:	48 83 c4 18          	add    rsp,0x18
   299d9:	5b                   	pop    rbx
   299da:	41 5c                	pop    r12
   299dc:	41 5d                	pop    r13
   299de:	41 5e                	pop    r14
   299e0:	41 5f                	pop    r15
   299e2:	5d                   	pop    rbp
   299e3:	c2 18 00             	ret    0x18
   299e6:	49 83 c5 09          	add    r13,0x9
   299ea:	b8 09 00 00 00       	mov    eax,0x9
   299ef:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   299f3:	e9 9d fc ff ff       	jmp    29695 <Data.Bytes.View.search-first#cc+0x35>
   299f8:	48 85 c0             	test   rax,rax
   299fb:	7e 2b                	jle    29a28 <Data.Bytes.View.search-first#cc+0x3c8>
   299fd:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29a01:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29a06:	4d 89 c7             	mov    r15,r8
   29a09:	41 38 0c 00          	cmp    BYTE PTR [r8+rax*1],cl
   29a0d:	74 14                	je     29a23 <Data.Bytes.View.search-first#cc+0x3c3>
   29a0f:	48 85 c0             	test   rax,rax
   29a12:	7e 66                	jle    29a7a <Data.Bytes.View.search-first#cc+0x41a>
   29a14:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   29a18:	48 8b 44 c2 f8       	mov    rax,QWORD PTR [rdx+rax*8-0x8]
   29a1d:	41 38 0c 07          	cmp    BYTE PTR [r15+rax*1],cl
   29a21:	75 38                	jne    29a5b <Data.Bytes.View.search-first#cc+0x3fb>
   29a23:	48 ff c0             	inc    rax
   29a26:	eb 54                	jmp    29a7c <Data.Bytes.View.search-first#cc+0x41c>
   29a28:	31 c0                	xor    eax,eax
   29a2a:	49 ff c5             	inc    r13
   29a2d:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   29a31:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   29a35:	4c 89 45 10          	mov    QWORD PTR [rbp+0x10],r8
   29a39:	48 89 c7             	mov    rdi,rax
   29a3c:	4c 89 f6             	mov    rsi,r14
   29a3f:	4c 89 ea             	mov    rdx,r13
   29a42:	4c 89 e1             	mov    rcx,r12
   29a45:	49 89 d8             	mov    r8,rbx
   29a48:	48 83 c4 18          	add    rsp,0x18
   29a4c:	5b                   	pop    rbx
   29a4d:	41 5c                	pop    r12
   29a4f:	41 5d                	pop    r13
   29a51:	41 5e                	pop    r14
   29a53:	41 5f                	pop    r15
   29a55:	5d                   	pop    rbp
   29a56:	e9 65 bb 00 00       	jmp    355c0 <Data.Bytes.View.search-first.shared0#cc>
   29a5b:	48 85 c0             	test   rax,rax
   29a5e:	7e 1a                	jle    29a7a <Data.Bytes.View.search-first#cc+0x41a>
   29a60:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
   29a64:	48 8b 54 c6 f8       	mov    rdx,QWORD PTR [rsi+rax*8-0x8]
   29a69:	4c 89 ff             	mov    rdi,r15
   29a6c:	e8 0f fb ff ff       	call   29580 <Data.Bytes.View.search-step>
   29a71:	4d 89 f8             	mov    r8,r15
   29a74:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29a78:	eb b0                	jmp    29a2a <Data.Bytes.View.search-first#cc+0x3ca>
   29a7a:	31 c0                	xor    eax,eax
   29a7c:	4c 8b 4d c8          	mov    r9,QWORD PTR [rbp-0x38]
   29a80:	4d 89 f8             	mov    r8,r15
   29a83:	eb a5                	jmp    29a2a <Data.Bytes.View.search-first#cc+0x3ca>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035720 <Data.Bytes.View.search-first.shared0>:
   35720:	55                   	push   rbp
   35721:	48 89 e5             	mov    rbp,rsp
   35724:	48 83 ec 20          	sub    rsp,0x20
   35728:	0f 28 45 10          	movaps xmm0,XMMWORD PTR [rbp+0x10]
   3572c:	0f 11 04 24          	movups XMMWORD PTR [rsp],xmm0
   35730:	e8 8b fe ff ff       	call   355c0 <Data.Bytes.View.search-first.shared0#cc>
   35735:	48 83 c4 08          	add    rsp,0x8
   35739:	5d                   	pop    rbp
   3573a:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000355c0 <Data.Bytes.View.search-first.shared0#cc>:
   355c0:	55                   	push   rbp
   355c1:	48 89 e5             	mov    rbp,rsp
   355c4:	41 57                	push   r15
   355c6:	41 56                	push   r14
   355c8:	41 55                	push   r13
   355ca:	41 54                	push   r12
   355cc:	53                   	push   rbx
   355cd:	48 83 ec 18          	sub    rsp,0x18
   355d1:	48 89 f3             	mov    rbx,rsi
   355d4:	48 39 f7             	cmp    rdi,rsi
   355d7:	7d 68                	jge    35641 <Data.Bytes.View.search-first.shared0#cc+0x81>
   355d9:	4c 8b 6d 18          	mov    r13,QWORD PTR [rbp+0x18]
   355dd:	4c 8b 7d 10          	mov    r15,QWORD PTR [rbp+0x10]
   355e1:	48 85 ff             	test   rdi,rdi
   355e4:	74 72                	je     35658 <Data.Bytes.View.search-first.shared0#cc+0x98>
   355e6:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   355ed:	4c 89 c6             	mov    rsi,r8
   355f0:	48 29 d6             	sub    rsi,rdx
   355f3:	7e 52                	jle    35647 <Data.Bytes.View.search-first.shared0#cc+0x87>
   355f5:	48 8d 43 f8          	lea    rax,[rbx-0x8]
   355f9:	48 39 c7             	cmp    rdi,rax
   355fc:	0f 9f c0             	setg   al
   355ff:	48 83 fe 08          	cmp    rsi,0x8
   35603:	40 0f 9c c6          	setl   sil
   35607:	40 08 c6             	or     sil,al
   3560a:	0f 85 9c 00 00 00    	jne    356ac <Data.Bytes.View.search-first.shared0#cc+0xec>
   35610:	48 8b 04 11          	mov    rax,QWORD PTR [rcx+rdx*1]
   35614:	49 3b 04 3f          	cmp    rax,QWORD PTR [r15+rdi*1]
   35618:	0f 85 8e 00 00 00    	jne    356ac <Data.Bytes.View.search-first.shared0#cc+0xec>
   3561e:	48 83 c2 08          	add    rdx,0x8
   35622:	48 83 c7 08          	add    rdi,0x8
   35626:	48 39 df             	cmp    rdi,rbx
   35629:	7d 16                	jge    35641 <Data.Bytes.View.search-first.shared0#cc+0x81>
   3562b:	48 89 7d 18          	mov    QWORD PTR [rbp+0x18],rdi
   3562f:	4c 89 4d 10          	mov    QWORD PTR [rbp+0x10],r9
   35633:	48 89 cf             	mov    rdi,rcx
   35636:	48 89 d6             	mov    rsi,rdx
   35639:	4c 89 c2             	mov    rdx,r8
   3563c:	e9 a9 00 00 00       	jmp    356ea <Data.Bytes.View.search-first.shared0#cc+0x12a>
   35641:	48 29 da             	sub    rdx,rbx
   35644:	48 89 d0             	mov    rax,rdx
   35647:	48 83 c4 18          	add    rsp,0x18
   3564b:	5b                   	pop    rbx
   3564c:	41 5c                	pop    r12
   3564e:	41 5d                	pop    r13
   35650:	41 5e                	pop    r14
   35652:	41 5f                	pop    r15
   35654:	5d                   	pop    rbp
   35655:	c2 18 00             	ret    0x18
   35658:	4c 89 7d d0          	mov    QWORD PTR [rbp-0x30],r15
   3565c:	4d 89 ef             	mov    r15,r13
   3565f:	49 89 cc             	mov    r12,rcx
   35662:	48 89 cf             	mov    rdi,rcx
   35665:	48 89 d6             	mov    rsi,rdx
   35668:	4d 89 c5             	mov    r13,r8
   3566b:	4c 89 c2             	mov    rdx,r8
   3566e:	4d 89 ce             	mov    r14,r9
   35671:	4c 89 c9             	mov    rcx,r9
   35674:	e8 a7 2f ff ff       	call   28620 <Data.Bytes.View.index-forward>
   35679:	48 85 c0             	test   rax,rax
   3567c:	0f 88 84 00 00 00    	js     35706 <Data.Bytes.View.search-first.shared0#cc+0x146>
   35682:	48 83 fb 02          	cmp    rbx,0x2
   35686:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   3568a:	7c bb                	jl     35647 <Data.Bytes.View.search-first.shared0#cc+0x87>
   3568c:	4c 89 ea             	mov    rdx,r13
   3568f:	4c 89 e7             	mov    rdi,r12
   35692:	48 ff c0             	inc    rax
   35695:	4c 89 75 10          	mov    QWORD PTR [rbp+0x10],r14
   35699:	48 c7 45 18 01 00 00 	mov    QWORD PTR [rbp+0x18],0x1
   356a0:	00 
   356a1:	48 89 c6             	mov    rsi,rax
   356a4:	49 89 d8             	mov    r8,rbx
   356a7:	4d 89 f9             	mov    r9,r15
   356aa:	eb 47                	jmp    356f3 <Data.Bytes.View.search-first.shared0#cc+0x133>
   356ac:	4d 89 cc             	mov    r12,r9
   356af:	4c 89 45 d0          	mov    QWORD PTR [rbp-0x30],r8
   356b3:	48 89 4d c8          	mov    QWORD PTR [rbp-0x38],rcx
   356b7:	0f b6 0c 11          	movzx  ecx,BYTE PTR [rcx+rdx*1]
   356bb:	48 89 f8             	mov    rax,rdi
   356be:	4c 89 ff             	mov    rdi,r15
   356c1:	4c 89 ee             	mov    rsi,r13
   356c4:	49 89 d6             	mov    r14,rdx
   356c7:	48 89 c2             	mov    rdx,rax
   356ca:	e8 b1 3e ff ff       	call   29580 <Data.Bytes.View.search-step>
   356cf:	4c 89 f6             	mov    rsi,r14
   356d2:	48 ff c6             	inc    rsi
   356d5:	48 39 d8             	cmp    rax,rbx
   356d8:	7d 38                	jge    35712 <Data.Bytes.View.search-first.shared0#cc+0x152>
   356da:	48 89 45 18          	mov    QWORD PTR [rbp+0x18],rax
   356de:	4c 89 65 10          	mov    QWORD PTR [rbp+0x10],r12
   356e2:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   356e6:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
   356ea:	4c 89 f9             	mov    rcx,r15
   356ed:	49 89 d8             	mov    r8,rbx
   356f0:	4d 89 e9             	mov    r9,r13
   356f3:	48 83 c4 18          	add    rsp,0x18
   356f7:	5b                   	pop    rbx
   356f8:	41 5c                	pop    r12
   356fa:	41 5d                	pop    r13
   356fc:	41 5e                	pop    r14
   356fe:	41 5f                	pop    r15
   35700:	5d                   	pop    rbp
   35701:	e9 5a 3f ff ff       	jmp    29660 <Data.Bytes.View.search-first#cc>
   35706:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   3570d:	e9 35 ff ff ff       	jmp    35647 <Data.Bytes.View.search-first.shared0#cc+0x87>
   35712:	48 29 de             	sub    rsi,rbx
   35715:	48 89 f0             	mov    rax,rsi
   35718:	e9 2a ff ff ff       	jmp    35647 <Data.Bytes.View.search-first.shared0#cc+0x87>

Disassembly of section .fini:

build/byte-view-bench/candidate3-rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000031dd0 <Main.work>:
   31dd0:	55                   	push   rbp
   31dd1:	48 89 e5             	mov    rbp,rsp
   31dd4:	41 57                	push   r15
   31dd6:	41 56                	push   r14
   31dd8:	41 55                	push   r13
   31dda:	41 54                	push   r12
   31ddc:	53                   	push   rbx
   31ddd:	48 83 ec 48          	sub    rsp,0x48
   31de1:	4d 89 cc             	mov    r12,r9
   31de4:	4c 89 45 98          	mov    QWORD PTR [rbp-0x68],r8
   31de8:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
   31dec:	48 89 55 a0          	mov    QWORD PTR [rbp-0x60],rdx
   31df0:	48 89 75 a8          	mov    QWORD PTR [rbp-0x58],rsi
   31df4:	48 89 7d b0          	mov    QWORD PTR [rbp-0x50],rdi
   31df8:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   31dfc:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   31e00:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   31e04:	eb 14                	jmp    31e1a <Main.work+0x4a>
   31e06:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   31e0d:	00 00 00 
   31e10:	48 ff ca             	dec    rdx
   31e13:	49 89 57 f8          	mov    QWORD PTR [r15-0x8],rdx
   31e17:	4d 89 ec             	mov    r12,r13
   31e1a:	49 89 c7             	mov    r15,rax
   31e1d:	4d 85 e4             	test   r12,r12
   31e20:	0f 94 c0             	sete   al
   31e23:	45 89 e6             	mov    r14d,r12d
   31e26:	41 08 c6             	or     r14b,al
   31e29:	41 f6 c6 01          	test   r14b,0x1
   31e2d:	74 21                	je     31e50 <Main.work+0x80>
   31e2f:	41 f6 c4 01          	test   r12b,0x1
   31e33:	75 3b                	jne    31e70 <Main.work+0xa0>
   31e35:	be 01 00 00 00       	mov    esi,0x1
   31e3a:	4c 89 e7             	mov    rdi,r12
   31e3d:	e8 9e 50 04 00       	call   76ee0 <cinch_nat_cmp>
   31e42:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   31e46:	e9 b5 00 00 00       	jmp    31f00 <Main.work+0x130>
   31e4b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   31e50:	49 8b 44 24 f8       	mov    rax,QWORD PTR [r12-0x8]
   31e55:	48 89 c1             	mov    rcx,rax
   31e58:	48 c1 e1 1d          	shl    rcx,0x1d
   31e5c:	48 85 c9             	test   rcx,rcx
   31e5f:	7e 27                	jle    31e88 <Main.work+0xb8>
   31e61:	48 ff c0             	inc    rax
   31e64:	49 89 44 24 f8       	mov    QWORD PTR [r12-0x8],rax
   31e69:	eb 27                	jmp    31e92 <Main.work+0xc2>
   31e6b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   31e70:	4c 89 e0             	mov    rax,r12
   31e73:	48 d1 e8             	shr    rax,1
   31e76:	48 85 c0             	test   rax,rax
   31e79:	0f 97 c0             	seta   al
   31e7c:	1c 00                	sbb    al,0x0
   31e7e:	48 0f be c0          	movsx  rax,al
   31e82:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   31e86:	eb 78                	jmp    31f00 <Main.work+0x130>
   31e88:	74 08                	je     31e92 <Main.work+0xc2>
   31e8a:	4c 89 e7             	mov    rdi,r12
   31e8d:	e8 2e fe 03 00       	call   71cc0 <cinch_dup_mt>
   31e92:	be 01 00 00 00       	mov    esi,0x1
   31e97:	4c 89 e7             	mov    rdi,r12
   31e9a:	e8 41 50 04 00       	call   76ee0 <cinch_nat_cmp>
   31e9f:	49 8b 54 24 f8       	mov    rdx,QWORD PTR [r12-0x8]
   31ea4:	48 89 d1             	mov    rcx,rdx
   31ea7:	48 c1 e1 1d          	shl    rcx,0x1d
   31eab:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   31eb2:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   31eb6:	7c 18                	jl     31ed0 <Main.work+0x100>
   31eb8:	48 ff ca             	dec    rdx
   31ebb:	49 89 54 24 f8       	mov    QWORD PTR [r12-0x8],rdx
   31ec0:	eb 3e                	jmp    31f00 <Main.work+0x130>
   31ec2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   31ec9:	1f 84 00 00 00 00 00 
   31ed0:	48 85 c9             	test   rcx,rcx
   31ed3:	74 2b                	je     31f00 <Main.work+0x130>
   31ed5:	48 89 c3             	mov    rbx,rax
   31ed8:	4c 89 e7             	mov    rdi,r12
   31edb:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   31ee2:	75 07                	jne    31eeb <Main.work+0x11b>
   31ee4:	e8 f7 fa 03 00       	call   719e0 <cinch_drop_slow>
   31ee9:	eb 05                	jmp    31ef0 <Main.work+0x120>
   31eeb:	e8 00 fa 03 00       	call   718f0 <cinch_drop_mt>
   31ef0:	48 89 d8             	mov    rax,rbx
   31ef3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   31efa:	84 00 00 00 00 00 
   31f00:	48 85 c0             	test   rax,rax
   31f03:	0f 84 55 03 00 00    	je     3225e <Main.work+0x48e>
   31f09:	44 88 75 d7          	mov    BYTE PTR [rbp-0x29],r14b
   31f0d:	4d 85 ff             	test   r15,r15
   31f10:	0f 94 c0             	sete   al
   31f13:	45 89 fe             	mov    r14d,r15d
   31f16:	41 08 c6             	or     r14b,al
   31f19:	41 f6 c6 01          	test   r14b,0x1
   31f1d:	75 31                	jne    31f50 <Main.work+0x180>
   31f1f:	49 8b 47 f8          	mov    rax,QWORD PTR [r15-0x8]
   31f23:	48 89 c1             	mov    rcx,rax
   31f26:	48 c1 e1 1d          	shl    rcx,0x1d
   31f2a:	48 85 c9             	test   rcx,rcx
   31f2d:	7e 09                	jle    31f38 <Main.work+0x168>
   31f2f:	48 ff c0             	inc    rax
   31f32:	49 89 47 f8          	mov    QWORD PTR [r15-0x8],rax
   31f36:	eb 18                	jmp    31f50 <Main.work+0x180>
   31f38:	74 16                	je     31f50 <Main.work+0x180>
   31f3a:	4c 89 ff             	mov    rdi,r15
   31f3d:	e8 7e fd 03 00       	call   71cc0 <cinch_dup_mt>
   31f42:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   31f49:	1f 84 00 00 00 00 00 
   31f50:	48 8b 7d a8          	mov    rdi,QWORD PTR [rbp-0x58]
   31f54:	48 8b 75 a0          	mov    rsi,QWORD PTR [rbp-0x60]
   31f58:	4c 89 fa             	mov    rdx,r15
   31f5b:	e8 f0 f2 ff ff       	call   31250 <Main.mutate>
   31f60:	4d 85 ed             	test   r13,r13
   31f63:	74 2b                	je     31f90 <Main.work+0x1c0>
   31f65:	49 8b 45 f8          	mov    rax,QWORD PTR [r13-0x8]
   31f69:	48 89 c1             	mov    rcx,rax
   31f6c:	48 c1 e1 1d          	shl    rcx,0x1d
   31f70:	48 85 c9             	test   rcx,rcx
   31f73:	7e 0b                	jle    31f80 <Main.work+0x1b0>
   31f75:	48 ff c0             	inc    rax
   31f78:	49 89 45 f8          	mov    QWORD PTR [r13-0x8],rax
   31f7c:	eb 12                	jmp    31f90 <Main.work+0x1c0>
   31f7e:	66 90                	xchg   ax,ax
   31f80:	74 0e                	je     31f90 <Main.work+0x1c0>
   31f82:	4c 89 ef             	mov    rdi,r13
   31f85:	e8 36 fd 03 00       	call   71cc0 <cinch_dup_mt>
   31f8a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   31f90:	4c 89 7d b8          	mov    QWORD PTR [rbp-0x48],r15
   31f94:	48 8b 7d b0          	mov    rdi,QWORD PTR [rbp-0x50]
   31f98:	4c 89 ee             	mov    rsi,r13
   31f9b:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
   31f9f:	e8 5c f6 ff ff       	call   31600 <Main.operation>
   31fa4:	49 89 c7             	mov    r15,rax
   31fa7:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   31fab:	89 fb                	mov    ebx,edi
   31fad:	83 e3 01             	and    ebx,0x1
   31fb0:	44 85 fb             	test   ebx,r15d
   31fb3:	74 2b                	je     31fe0 <Main.work+0x210>
   31fb5:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
   31fb9:	48 d1 e9             	shr    rcx,1
   31fbc:	4c 89 f8             	mov    rax,r15
   31fbf:	48 d1 e8             	shr    rax,1
   31fc2:	48 01 c8             	add    rax,rcx
   31fc5:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   31fc9:	78 15                	js     31fe0 <Main.work+0x210>
   31fcb:	4c 8d 2c 45 01 00 00 	lea    r13,[rax*2+0x1]
   31fd2:	00 
   31fd3:	eb 1a                	jmp    31fef <Main.work+0x21f>
   31fd5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   31fdc:	00 00 00 00 
   31fe0:	4c 89 fe             	mov    rsi,r15
   31fe3:	e8 08 4d 04 00       	call   76cf0 <cinch_nat_add>
   31fe8:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   31fec:	49 89 c5             	mov    r13,rax
   31fef:	48 85 ff             	test   rdi,rdi
   31ff2:	0f 94 c0             	sete   al
   31ff5:	08 c3                	or     bl,al
   31ff7:	75 47                	jne    32040 <Main.work+0x270>
   31ff9:	48 8b 4f f8          	mov    rcx,QWORD PTR [rdi-0x8]
   31ffd:	48 89 c8             	mov    rax,rcx
   32000:	48 c1 e0 1d          	shl    rax,0x1d
   32004:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   3200a:	7c 14                	jl     32020 <Main.work+0x250>
   3200c:	48 ff c9             	dec    rcx
   3200f:	48 89 4f f8          	mov    QWORD PTR [rdi-0x8],rcx
   32013:	eb 2b                	jmp    32040 <Main.work+0x270>
   32015:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3201c:	00 00 00 00 
   32020:	48 85 c0             	test   rax,rax
   32023:	74 1b                	je     32040 <Main.work+0x270>
   32025:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3202b:	75 07                	jne    32034 <Main.work+0x264>
   3202d:	e8 ae f9 03 00       	call   719e0 <cinch_drop_slow>
   32032:	eb 0c                	jmp    32040 <Main.work+0x270>
   32034:	e8 b7 f8 03 00       	call   718f0 <cinch_drop_mt>
   32039:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   32040:	4d 85 ff             	test   r15,r15
   32043:	0f 94 c0             	sete   al
   32046:	44 89 f9             	mov    ecx,r15d
   32049:	08 c1                	or     cl,al
   3204b:	f6 c1 01             	test   cl,0x1
   3204e:	75 50                	jne    320a0 <Main.work+0x2d0>
   32050:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   32054:	48 89 c8             	mov    rax,rcx
   32057:	48 c1 e0 1d          	shl    rax,0x1d
   3205b:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32061:	7c 1d                	jl     32080 <Main.work+0x2b0>
   32063:	48 ff c9             	dec    rcx
   32066:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   3206a:	41 f6 c5 01          	test   r13b,0x1
   3206e:	74 36                	je     320a6 <Main.work+0x2d6>
   32070:	e9 9b 00 00 00       	jmp    32110 <Main.work+0x340>
   32075:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3207c:	00 00 00 00 
   32080:	48 85 c0             	test   rax,rax
   32083:	74 1b                	je     320a0 <Main.work+0x2d0>
   32085:	4c 89 ff             	mov    rdi,r15
   32088:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3208e:	75 6b                	jne    320fb <Main.work+0x32b>
   32090:	e8 4b f9 03 00       	call   719e0 <cinch_drop_slow>
   32095:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3209c:	00 00 00 00 
   320a0:	41 f6 c5 01          	test   r13b,0x1
   320a4:	75 6a                	jne    32110 <Main.work+0x340>
   320a6:	be 0f 94 35 77       	mov    esi,0x7735940f
   320ab:	4c 89 ef             	mov    rdi,r13
   320ae:	e8 ed 57 04 00       	call   778a0 <cinch_nat_rem>
   320b3:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   320b7:	4d 85 ed             	test   r13,r13
   320ba:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   320be:	0f 84 7c 00 00 00    	je     32140 <Main.work+0x370>
   320c4:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   320c8:	48 89 c8             	mov    rax,rcx
   320cb:	48 c1 e0 1d          	shl    rax,0x1d
   320cf:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   320d5:	7c 09                	jl     320e0 <Main.work+0x310>
   320d7:	48 ff c9             	dec    rcx
   320da:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   320de:	eb 60                	jmp    32140 <Main.work+0x370>
   320e0:	48 85 c0             	test   rax,rax
   320e3:	74 5b                	je     32140 <Main.work+0x370>
   320e5:	4c 89 ef             	mov    rdi,r13
   320e8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   320ee:	0f 85 60 01 00 00    	jne    32254 <Main.work+0x484>
   320f4:	e8 e7 f8 03 00       	call   719e0 <cinch_drop_slow>
   320f9:	eb 45                	jmp    32140 <Main.work+0x370>
   320fb:	e8 f0 f7 03 00       	call   718f0 <cinch_drop_mt>
   32100:	41 f6 c5 01          	test   r13b,0x1
   32104:	74 a0                	je     320a6 <Main.work+0x2d6>
   32106:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3210d:	00 00 00 
   32110:	49 d1 ed             	shr    r13,1
   32113:	4c 89 e8             	mov    rax,r13
   32116:	48 b9 e5 8f a2 12 31 	movabs rcx,0x89705f3112a28fe5
   3211d:	5f 70 89 
   32120:	48 f7 e1             	mul    rcx
   32123:	48 c1 ea 1d          	shr    rdx,0x1d
   32127:	69 c2 07 ca 9a 3b    	imul   eax,edx,0x3b9aca07
   3212d:	41 29 c5             	sub    r13d,eax
   32130:	4a 8d 04 6d 01 00 00 	lea    rax,[r13*2+0x1]
   32137:	00 
   32138:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   3213c:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   32140:	41 f6 c4 01          	test   r12b,0x1
   32144:	0f 94 c0             	sete   al
   32147:	49 83 fc 02          	cmp    r12,0x2
   3214b:	0f 92 c1             	setb   cl
   3214e:	08 c1                	or     cl,al
   32150:	74 1e                	je     32170 <Main.work+0x3a0>
   32152:	be 03 00 00 00       	mov    esi,0x3
   32157:	4c 89 e7             	mov    rdi,r12
   3215a:	e8 81 59 04 00       	call   77ae0 <cinch_nat_sub>
   3215f:	49 89 c5             	mov    r13,rax
   32162:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   32166:	74 13                	je     3217b <Main.work+0x3ab>
   32168:	eb 56                	jmp    321c0 <Main.work+0x3f0>
   3216a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   32170:	4d 8d 6c 24 fe       	lea    r13,[r12-0x2]
   32175:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   32179:	75 45                	jne    321c0 <Main.work+0x3f0>
   3217b:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   32180:	48 89 c8             	mov    rax,rcx
   32183:	48 c1 e0 1d          	shl    rax,0x1d
   32187:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   3218d:	7c 11                	jl     321a0 <Main.work+0x3d0>
   3218f:	48 ff c9             	dec    rcx
   32192:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   32197:	41 f6 c7 01          	test   r15b,0x1
   3219b:	75 29                	jne    321c6 <Main.work+0x3f6>
   3219d:	eb 51                	jmp    321f0 <Main.work+0x420>
   3219f:	90                   	nop
   321a0:	48 85 c0             	test   rax,rax
   321a3:	74 1b                	je     321c0 <Main.work+0x3f0>
   321a5:	4c 89 e7             	mov    rdi,r12
   321a8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   321ae:	75 2b                	jne    321db <Main.work+0x40b>
   321b0:	e8 2b f8 03 00       	call   719e0 <cinch_drop_slow>
   321b5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   321bc:	00 00 00 00 
   321c0:	41 f6 c7 01          	test   r15b,0x1
   321c4:	74 2a                	je     321f0 <Main.work+0x420>
   321c6:	4c 89 f8             	mov    rax,r15
   321c9:	48 d1 e8             	shr    rax,1
   321cc:	48 ff c0             	inc    rax
   321cf:	78 1f                	js     321f0 <Main.work+0x420>
   321d1:	48 8d 04 45 01 00 00 	lea    rax,[rax*2+0x1]
   321d8:	00 
   321d9:	eb 22                	jmp    321fd <Main.work+0x42d>
   321db:	e8 10 f7 03 00       	call   718f0 <cinch_drop_mt>
   321e0:	41 f6 c7 01          	test   r15b,0x1
   321e4:	75 e0                	jne    321c6 <Main.work+0x3f6>
   321e6:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   321ed:	00 00 00 
   321f0:	be 03 00 00 00       	mov    esi,0x3
   321f5:	4c 89 ff             	mov    rdi,r15
   321f8:	e8 f3 4a 04 00       	call   76cf0 <cinch_nat_add>
   321fd:	4d 89 ec             	mov    r12,r13
   32200:	41 f6 c6 01          	test   r14b,0x1
   32204:	0f 85 10 fc ff ff    	jne    31e1a <Main.work+0x4a>
   3220a:	49 8b 57 f8          	mov    rdx,QWORD PTR [r15-0x8]
   3220e:	48 89 d1             	mov    rcx,rdx
   32211:	48 c1 e1 1d          	shl    rcx,0x1d
   32215:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   3221c:	0f 8d ee fb ff ff    	jge    31e10 <Main.work+0x40>
   32222:	4d 89 ec             	mov    r12,r13
   32225:	48 85 c9             	test   rcx,rcx
   32228:	0f 84 ec fb ff ff    	je     31e1a <Main.work+0x4a>
   3222e:	48 89 c3             	mov    rbx,rax
   32231:	4c 89 ff             	mov    rdi,r15
   32234:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   3223b:	75 07                	jne    32244 <Main.work+0x474>
   3223d:	e8 9e f7 03 00       	call   719e0 <cinch_drop_slow>
   32242:	eb 05                	jmp    32249 <Main.work+0x479>
   32244:	e8 a7 f6 03 00       	call   718f0 <cinch_drop_mt>
   32249:	4d 89 ec             	mov    r12,r13
   3224c:	48 89 d8             	mov    rax,rbx
   3224f:	e9 c6 fb ff ff       	jmp    31e1a <Main.work+0x4a>
   32254:	e8 97 f6 03 00       	call   718f0 <cinch_drop_mt>
   32259:	e9 e2 fe ff ff       	jmp    32140 <Main.work+0x370>
   3225e:	4d 85 ed             	test   r13,r13
   32261:	74 37                	je     3229a <Main.work+0x4ca>
   32263:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   32267:	48 89 c8             	mov    rax,rcx
   3226a:	48 c1 e0 1d          	shl    rax,0x1d
   3226e:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32274:	7c 0f                	jl     32285 <Main.work+0x4b5>
   32276:	48 ff c9             	dec    rcx
   32279:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   3227d:	41 f6 c6 01          	test   r14b,0x1
   32281:	74 1d                	je     322a0 <Main.work+0x4d0>
   32283:	eb 62                	jmp    322e7 <Main.work+0x517>
   32285:	48 85 c0             	test   rax,rax
   32288:	74 10                	je     3229a <Main.work+0x4ca>
   3228a:	4c 89 ef             	mov    rdi,r13
   3228d:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   32293:	75 40                	jne    322d5 <Main.work+0x505>
   32295:	e8 46 f7 03 00       	call   719e0 <cinch_drop_slow>
   3229a:	41 f6 c6 01          	test   r14b,0x1
   3229e:	75 47                	jne    322e7 <Main.work+0x517>
   322a0:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   322a5:	48 89 c8             	mov    rax,rcx
   322a8:	48 c1 e0 1d          	shl    rax,0x1d
   322ac:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   322b2:	7c 0a                	jl     322be <Main.work+0x4ee>
   322b4:	48 ff c9             	dec    rcx
   322b7:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   322bc:	eb 29                	jmp    322e7 <Main.work+0x517>
   322be:	48 85 c0             	test   rax,rax
   322c1:	74 24                	je     322e7 <Main.work+0x517>
   322c3:	4c 89 e7             	mov    rdi,r12
   322c6:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   322cc:	75 14                	jne    322e2 <Main.work+0x512>
   322ce:	e8 0d f7 03 00       	call   719e0 <cinch_drop_slow>
   322d3:	eb 12                	jmp    322e7 <Main.work+0x517>
   322d5:	e8 16 f6 03 00       	call   718f0 <cinch_drop_mt>
   322da:	41 f6 c6 01          	test   r14b,0x1
   322de:	74 c0                	je     322a0 <Main.work+0x4d0>
   322e0:	eb 05                	jmp    322e7 <Main.work+0x517>
   322e2:	e8 09 f6 03 00       	call   718f0 <cinch_drop_mt>
   322e7:	4d 85 ff             	test   r15,r15
   322ea:	0f 94 c0             	sete   al
   322ed:	44 89 f9             	mov    ecx,r15d
   322f0:	08 c1                	or     cl,al
   322f2:	f6 c1 01             	test   cl,0x1
   322f5:	75 38                	jne    3232f <Main.work+0x55f>
   322f7:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   322fb:	48 89 c8             	mov    rax,rcx
   322fe:	48 c1 e0 1d          	shl    rax,0x1d
   32302:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32308:	7c 09                	jl     32313 <Main.work+0x543>
   3230a:	48 ff c9             	dec    rcx
   3230d:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   32311:	eb 1c                	jmp    3232f <Main.work+0x55f>
   32313:	48 85 c0             	test   rax,rax
   32316:	74 17                	je     3232f <Main.work+0x55f>
   32318:	4c 89 ff             	mov    rdi,r15
   3231b:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   32321:	75 07                	jne    3232a <Main.work+0x55a>
   32323:	e8 b8 f6 03 00       	call   719e0 <cinch_drop_slow>
   32328:	eb 05                	jmp    3232f <Main.work+0x55f>
   3232a:	e8 c1 f5 03 00       	call   718f0 <cinch_drop_mt>
   3232f:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
   32333:	48 83 c4 48          	add    rsp,0x48
   32337:	5b                   	pop    rbx
   32338:	41 5c                	pop    r12
   3233a:	41 5d                	pop    r13
   3233c:	41 5e                	pop    r14
   3233e:	41 5f                	pop    r15
   32340:	5d                   	pop    rbp
   32341:	c3                   	ret

Disassembly of section .fini:
