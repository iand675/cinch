
build/byte-view-bench/native-final/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003f7e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount>:
   3f7e0:	55                   	push   rbp
   3f7e1:	41 57                	push   r15
   3f7e3:	41 56                	push   r14
   3f7e5:	41 55                	push   r13
   3f7e7:	41 54                	push   r12
   3f7e9:	53                   	push   rbx
   3f7ea:	50                   	push   rax
   3f7eb:	4c 89 cb             	mov    rbx,r9
   3f7ee:	4d 89 c5             	mov    r13,r8
   3f7f1:	49 89 cf             	mov    r15,rcx
   3f7f4:	49 89 d6             	mov    r14,rdx
   3f7f7:	49 89 fc             	mov    r12,rdi
   3f7fa:	4c 8b 5c 24 50       	mov    r11,QWORD PTR [rsp+0x50]
   3f7ff:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
   3f804:	48 8b 6c 24 40       	mov    rbp,QWORD PTR [rsp+0x40]
   3f809:	eb 26                	jmp    3f831 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x51>
   3f80b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   3f810:	48 83 c6 08          	add    rsi,0x8
   3f814:	48 83 c0 08          	add    rax,0x8
   3f818:	49 89 c2             	mov    r10,rax
   3f81b:	45 31 c9             	xor    r9d,r9d
   3f81e:	4c 39 e8             	cmp    rax,r13
   3f821:	41 0f 9d c1          	setge  r9b
   3f825:	b8 00 00 00 00       	mov    eax,0x0
   3f82a:	49 0f 4c c2          	cmovl  rax,r10
   3f82e:	4d 01 cb             	add    r11,r9
   3f831:	48 85 c0             	test   rax,rax
   3f834:	75 38                	jne    3f86e <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x8e>
   3f836:	4c 39 f6             	cmp    rsi,r14
   3f839:	0f 8d 9d 00 00 00    	jge    3f8dc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xfc>
   3f83f:	41 38 2c 34          	cmp    BYTE PTR [r12+rsi*1],bpl
   3f843:	74 1c                	je     3f861 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x81>
   3f845:	48 ff c6             	inc    rsi
   3f848:	4c 89 e7             	mov    rdi,r12
   3f84b:	4c 89 f2             	mov    rdx,r14
   3f84e:	48 89 e9             	mov    rcx,rbp
   3f851:	4c 89 1c 24          	mov    QWORD PTR [rsp],r11
   3f855:	e8 e6 00 00 00       	call   3f940 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   3f85a:	4c 8b 1c 24          	mov    r11,QWORD PTR [rsp]
   3f85e:	48 89 c6             	mov    rsi,rax
   3f861:	48 85 f6             	test   rsi,rsi
   3f864:	78 76                	js     3f8dc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xfc>
   3f866:	48 ff c6             	inc    rsi
   3f869:	b8 01 00 00 00       	mov    eax,0x1
   3f86e:	4d 89 f1             	mov    r9,r14
   3f871:	49 29 f1             	sub    r9,rsi
   3f874:	7e 66                	jle    3f8dc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xfc>
   3f876:	49 83 f9 08          	cmp    r9,0x8
   3f87a:	7c 34                	jl     3f8b0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xd0>
   3f87c:	4d 89 e9             	mov    r9,r13
   3f87f:	49 29 c1             	sub    r9,rax
   3f882:	49 83 f9 07          	cmp    r9,0x7
   3f886:	7e 28                	jle    3f8b0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xd0>
   3f888:	4d 8b 14 34          	mov    r10,QWORD PTR [r12+rsi*1]
   3f88c:	4d 8b 0c 07          	mov    r9,QWORD PTR [r15+rax*1]
   3f890:	4d 39 ca             	cmp    r10,r9
   3f893:	0f 84 77 ff ff ff    	je     3f810 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x30>
   3f899:	45 38 ca             	cmp    r10b,r9b
   3f89c:	75 22                	jne    3f8c0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xe0>
   3f89e:	48 ff c0             	inc    rax
   3f8a1:	eb 31                	jmp    3f8d4 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xf4>
   3f8a3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   3f8aa:	84 00 00 00 00 00 
   3f8b0:	45 0f b6 14 34       	movzx  r10d,BYTE PTR [r12+rsi*1]
   3f8b5:	45 0f b6 0c 07       	movzx  r9d,BYTE PTR [r15+rax*1]
   3f8ba:	45 38 ca             	cmp    r10b,r9b
   3f8bd:	74 df                	je     3f89e <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xbe>
   3f8bf:	90                   	nop
   3f8c0:	48 85 c0             	test   rax,rax
   3f8c3:	7e 0d                	jle    3f8d2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xf2>
   3f8c5:	48 8b 44 c3 f8       	mov    rax,QWORD PTR [rbx+rax*8-0x8]
   3f8ca:	45 3a 14 07          	cmp    r10b,BYTE PTR [r15+rax*1]
   3f8ce:	75 f0                	jne    3f8c0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xe0>
   3f8d0:	eb cc                	jmp    3f89e <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xbe>
   3f8d2:	31 c0                	xor    eax,eax
   3f8d4:	48 ff c6             	inc    rsi
   3f8d7:	e9 3c ff ff ff       	jmp    3f818 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x38>
   3f8dc:	4c 89 d8             	mov    rax,r11
   3f8df:	48 83 c4 08          	add    rsp,0x8
   3f8e3:	5b                   	pop    rbx
   3f8e4:	41 5c                	pop    r12
   3f8e6:	41 5d                	pop    r13
   3f8e8:	41 5e                	pop    r14
   3f8ea:	41 5f                	pop    r15
   3f8ec:	5d                   	pop    rbp
   3f8ed:	c3                   	ret

Disassembly of section .fini:
