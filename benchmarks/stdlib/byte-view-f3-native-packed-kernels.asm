
build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000413a0 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks>:
   413a0:	eb 0e                	jmp    413b0 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x10>

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>:
   3f960:	41 57                	push   r15
   3f962:	41 56                	push   r14
   3f964:	53                   	push   rbx
   3f965:	48 83 ec 10          	sub    rsp,0x10
   3f969:	48 89 d0             	mov    rax,rdx
   3f96c:	48 29 f0             	sub    rax,rsi
   3f96f:	48 83 f8 08          	cmp    rax,0x8
   3f973:	7c 4e                	jl     3f9c3 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0x63>
   3f975:	48 b8 80 80 80 80 80 	movabs rax,0x8080808080808080
   3f97c:	80 80 80 
   3f97f:	49 b8 7f 7f 7f 7f 7f 	movabs r8,0x7f7f7f7f7f7f7f7f
   3f986:	7f 7f 7f 
   3f989:	4c 8d 0c 37          	lea    r9,[rdi+rsi*1]
   3f98d:	48 f7 de             	neg    rsi
   3f990:	4d 8b 11             	mov    r10,QWORD PTR [r9]
   3f993:	49 31 ca             	xor    r10,rcx
   3f996:	4c 89 d3             	mov    rbx,r10
   3f999:	4c 21 c3             	and    rbx,r8
   3f99c:	4c 01 c3             	add    rbx,r8
   3f99f:	4c 09 d3             	or     rbx,r10
   3f9a2:	48 21 c3             	and    rbx,rax
   3f9a5:	48 39 c3             	cmp    rbx,rax
   3f9a8:	75 35                	jne    3f9df <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0x7f>
   3f9aa:	4c 8d 14 32          	lea    r10,[rdx+rsi*1]
   3f9ae:	49 83 c2 f8          	add    r10,0xfffffffffffffff8
   3f9b2:	48 83 c6 f8          	add    rsi,0xfffffffffffffff8
   3f9b6:	49 83 c1 08          	add    r9,0x8
   3f9ba:	49 83 fa 07          	cmp    r10,0x7
   3f9be:	7f d0                	jg     3f990 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0x30>
   3f9c0:	48 f7 de             	neg    rsi
   3f9c3:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   3f9ca:	48 39 d6             	cmp    rsi,rdx
   3f9cd:	7d 69                	jge    3fa38 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0xd8>
   3f9cf:	90                   	nop
   3f9d0:	38 0c 37             	cmp    BYTE PTR [rdi+rsi*1],cl
   3f9d3:	74 60                	je     3fa35 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0xd5>
   3f9d5:	48 ff c6             	inc    rsi
   3f9d8:	48 39 f2             	cmp    rdx,rsi
   3f9db:	75 f3                	jne    3f9d0 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0x70>
   3f9dd:	eb 59                	jmp    3fa38 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0xd8>
   3f9df:	48 31 c3             	xor    rbx,rax
   3f9e2:	49 89 f7             	mov    r15,rsi
   3f9e5:	e8 16 e3 06 00       	call   add00 <Data.Machine.u32-to-i64.object-closure>
   3f9ea:	49 89 c6             	mov    r14,rax
   3f9ed:	48 89 df             	mov    rdi,rbx
   3f9f0:	e8 3b ee 04 00       	call   8e830 <Data.Machine.trailing-zeros-u64>
   3f9f5:	48 89 c3             	mov    rbx,rax
   3f9f8:	4c 89 f7             	mov    rdi,r14
   3f9fb:	e8 30 11 41 00       	call   450b30 <cinch_dup>
   3fa00:	48 89 5c 24 08       	mov    QWORD PTR [rsp+0x8],rbx
   3fa05:	48 8d 54 24 08       	lea    rdx,[rsp+0x8]
   3fa0a:	be 01 00 00 00       	mov    esi,0x1
   3fa0f:	4c 89 f7             	mov    rdi,r14
   3fa12:	41 ff 16             	call   QWORD PTR [r14]
   3fa15:	be 03 00 00 00       	mov    esi,0x3
   3fa1a:	48 89 c7             	mov    rdi,rax
   3fa1d:	e8 ce f1 02 00       	call   6ebf0 <Data.Machine.wrapping-shr-i64>
   3fa22:	48 89 c3             	mov    rbx,rax
   3fa25:	4c 89 f7             	mov    rdi,r14
   3fa28:	e8 c3 0c 41 00       	call   4506f0 <cinch_drop>
   3fa2d:	48 89 d8             	mov    rax,rbx
   3fa30:	4c 29 f8             	sub    rax,r15
   3fa33:	eb 03                	jmp    3fa38 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks+0xd8>
   3fa35:	48 89 f0             	mov    rax,rsi
   3fa38:	48 83 c4 10          	add    rsp,0x10
   3fa3c:	5b                   	pop    rbx
   3fa3d:	41 5e                	pop    r14
   3fa3f:	41 5f                	pop    r15
   3fa41:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003f7f0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount>:
   3f7f0:	55                   	push   rbp
   3f7f1:	41 57                	push   r15
   3f7f3:	41 56                	push   r14
   3f7f5:	41 55                	push   r13
   3f7f7:	41 54                	push   r12
   3f7f9:	53                   	push   rbx
   3f7fa:	48 83 ec 18          	sub    rsp,0x18
   3f7fe:	4c 89 cb             	mov    rbx,r9
   3f801:	4c 89 c5             	mov    rbp,r8
   3f804:	49 89 cf             	mov    r15,rcx
   3f807:	49 89 d5             	mov    r13,rdx
   3f80a:	49 89 fc             	mov    r12,rdi
   3f80d:	4c 8b 5c 24 60       	mov    r11,QWORD PTR [rsp+0x60]
   3f812:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
   3f817:	48 8b 4c 24 50       	mov    rcx,QWORD PTR [rsp+0x50]
   3f81c:	4d 8d 70 f8          	lea    r14,[r8-0x8]
   3f820:	4c 89 74 24 08       	mov    QWORD PTR [rsp+0x8],r14
   3f825:	eb 2a                	jmp    3f851 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x61>
   3f827:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   3f82e:	00 00 
   3f830:	48 83 c6 08          	add    rsi,0x8
   3f834:	48 83 c0 08          	add    rax,0x8
   3f838:	49 89 c1             	mov    r9,rax
   3f83b:	45 31 d2             	xor    r10d,r10d
   3f83e:	48 39 e8             	cmp    rax,rbp
   3f841:	41 0f 9d c2          	setge  r10b
   3f845:	b8 00 00 00 00       	mov    eax,0x0
   3f84a:	49 0f 4c c1          	cmovl  rax,r9
   3f84e:	4d 01 d3             	add    r11,r10
   3f851:	48 85 c0             	test   rax,rax
   3f854:	75 42                	jne    3f898 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xa8>
   3f856:	4c 39 ee             	cmp    rsi,r13
   3f859:	0f 8d 9d 00 00 00    	jge    3f8fc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x10c>
   3f85f:	41 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],cl
   3f863:	74 26                	je     3f88b <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x9b>
   3f865:	48 ff c6             	inc    rsi
   3f868:	4c 89 e7             	mov    rdi,r12
   3f86b:	4c 89 ea             	mov    rdx,r13
   3f86e:	49 89 ce             	mov    r14,rcx
   3f871:	4c 89 5c 24 10       	mov    QWORD PTR [rsp+0x10],r11
   3f876:	e8 e5 00 00 00       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   3f87b:	4c 8b 5c 24 10       	mov    r11,QWORD PTR [rsp+0x10]
   3f880:	4c 89 f1             	mov    rcx,r14
   3f883:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
   3f888:	48 89 c6             	mov    rsi,rax
   3f88b:	48 85 f6             	test   rsi,rsi
   3f88e:	78 6c                	js     3f8fc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x10c>
   3f890:	48 ff c6             	inc    rsi
   3f893:	b8 01 00 00 00       	mov    eax,0x1
   3f898:	4d 89 e9             	mov    r9,r13
   3f89b:	49 29 f1             	sub    r9,rsi
   3f89e:	7e 5c                	jle    3f8fc <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x10c>
   3f8a0:	4c 39 f0             	cmp    rax,r14
   3f8a3:	7f 2b                	jg     3f8d0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xe0>
   3f8a5:	49 83 f9 07          	cmp    r9,0x7
   3f8a9:	7e 25                	jle    3f8d0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xe0>
   3f8ab:	4d 8b 0c 34          	mov    r9,QWORD PTR [r12+rsi*1]
   3f8af:	4d 8b 14 07          	mov    r10,QWORD PTR [r15+rax*1]
   3f8b3:	4d 39 d1             	cmp    r9,r10
   3f8b6:	0f 84 74 ff ff ff    	je     3f830 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x40>
   3f8bc:	45 38 d1             	cmp    r9b,r10b
   3f8bf:	75 1f                	jne    3f8e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xf0>
   3f8c1:	48 ff c0             	inc    rax
   3f8c4:	eb 2e                	jmp    3f8f4 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x104>
   3f8c6:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3f8cd:	00 00 00 
   3f8d0:	45 0f b6 0c 34       	movzx  r9d,BYTE PTR [r12+rsi*1]
   3f8d5:	45 0f b6 14 07       	movzx  r10d,BYTE PTR [r15+rax*1]
   3f8da:	45 38 d1             	cmp    r9b,r10b
   3f8dd:	74 e2                	je     3f8c1 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xd1>
   3f8df:	90                   	nop
   3f8e0:	48 85 c0             	test   rax,rax
   3f8e3:	7e 0d                	jle    3f8f2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x102>
   3f8e5:	48 8b 44 c3 f8       	mov    rax,QWORD PTR [rbx+rax*8-0x8]
   3f8ea:	45 3a 0c 07          	cmp    r9b,BYTE PTR [r15+rax*1]
   3f8ee:	75 f0                	jne    3f8e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xf0>
   3f8f0:	eb cf                	jmp    3f8c1 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xd1>
   3f8f2:	31 c0                	xor    eax,eax
   3f8f4:	48 ff c6             	inc    rsi
   3f8f7:	e9 3c ff ff ff       	jmp    3f838 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x48>
   3f8fc:	4c 89 d8             	mov    rax,r11
   3f8ff:	48 83 c4 18          	add    rsp,0x18
   3f903:	5b                   	pop    rbx
   3f904:	41 5c                	pop    r12
   3f906:	41 5d                	pop    r13
   3f908:	41 5e                	pop    r14
   3f90a:	41 5f                	pop    r15
   3f90c:	5d                   	pop    rbp
   3f90d:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000411e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst>:
   411e0:	55                   	push   rbp
   411e1:	41 57                	push   r15
   411e3:	41 56                	push   r14
   411e5:	41 55                	push   r13
   411e7:	41 54                	push   r12
   411e9:	53                   	push   rbx
   411ea:	50                   	push   rax
   411eb:	4d 89 ce             	mov    r14,r9
   411ee:	4d 89 c5             	mov    r13,r8
   411f1:	49 89 cf             	mov    r15,rcx
   411f4:	49 89 fc             	mov    r12,rdi
   411f7:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
   411fc:	48 8b 6c 24 40       	mov    rbp,QWORD PTR [rsp+0x40]
   41201:	49 8d 58 f8          	lea    rbx,[r8-0x8]
   41205:	48 89 14 24          	mov    QWORD PTR [rsp],rdx
   41209:	48 85 c0             	test   rax,rax
   4120c:	75 27                	jne    41235 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x55>
   4120e:	e9 8d 00 00 00       	jmp    412a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   41213:	48 83 c6 08          	add    rsi,0x8
   41217:	48 83 c0 08          	add    rax,0x8
   4121b:	4c 39 e8             	cmp    rax,r13
   4121e:	0f 8d c3 00 00 00    	jge    412e7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x107>
   41224:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   4122b:	00 00 00 00 00 
   41230:	48 85 c0             	test   rax,rax
   41233:	74 6b                	je     412a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   41235:	49 89 d1             	mov    r9,rdx
   41238:	49 29 f1             	sub    r9,rsi
   4123b:	0f 8e 9d 00 00 00    	jle    412de <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xfe>
   41241:	48 39 d8             	cmp    rax,rbx
   41244:	7f 2a                	jg     41270 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x90>
   41246:	49 83 f9 07          	cmp    r9,0x7
   4124a:	7e 24                	jle    41270 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x90>
   4124c:	4d 8b 0c 34          	mov    r9,QWORD PTR [r12+rsi*1]
   41250:	4d 8b 14 07          	mov    r10,QWORD PTR [r15+rax*1]
   41254:	4d 39 d1             	cmp    r9,r10
   41257:	74 ba                	je     41213 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x33>
   41259:	45 38 d1             	cmp    r9b,r10b
   4125c:	75 22                	jne    41280 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xa0>
   4125e:	48 ff c0             	inc    rax
   41261:	eb 31                	jmp    41294 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xb4>
   41263:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   4126a:	84 00 00 00 00 00 
   41270:	45 0f b6 0c 34       	movzx  r9d,BYTE PTR [r12+rsi*1]
   41275:	45 0f b6 14 07       	movzx  r10d,BYTE PTR [r15+rax*1]
   4127a:	45 38 d1             	cmp    r9b,r10b
   4127d:	74 df                	je     4125e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x7e>
   4127f:	90                   	nop
   41280:	48 85 c0             	test   rax,rax
   41283:	7e 0d                	jle    41292 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xb2>
   41285:	49 8b 44 c6 f8       	mov    rax,QWORD PTR [r14+rax*8-0x8]
   4128a:	45 3a 0c 07          	cmp    r9b,BYTE PTR [r15+rax*1]
   4128e:	75 f0                	jne    41280 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xa0>
   41290:	eb cc                	jmp    4125e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x7e>
   41292:	31 c0                	xor    eax,eax
   41294:	48 ff c6             	inc    rsi
   41297:	4c 39 e8             	cmp    rax,r13
   4129a:	7c 94                	jl     41230 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x50>
   4129c:	eb 49                	jmp    412e7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x107>
   4129e:	66 90                	xchg   ax,ax
   412a0:	48 39 f2             	cmp    rdx,rsi
   412a3:	7e 39                	jle    412de <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xfe>
   412a5:	41 38 2c 34          	cmp    BYTE PTR [r12+rsi*1],bpl
   412a9:	74 15                	je     412c0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xe0>
   412ab:	48 ff c6             	inc    rsi
   412ae:	4c 89 e7             	mov    rdi,r12
   412b1:	48 89 e9             	mov    rcx,rbp
   412b4:	e8 a7 e6 ff ff       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   412b9:	48 8b 14 24          	mov    rdx,QWORD PTR [rsp]
   412bd:	48 89 c6             	mov    rsi,rax
   412c0:	48 85 f6             	test   rsi,rsi
   412c3:	78 2a                	js     412ef <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x10f>
   412c5:	49 83 fd 01          	cmp    r13,0x1
   412c9:	7e 24                	jle    412ef <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x10f>
   412cb:	48 ff c6             	inc    rsi
   412ce:	b8 01 00 00 00       	mov    eax,0x1
   412d3:	48 85 c0             	test   rax,rax
   412d6:	0f 85 59 ff ff ff    	jne    41235 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x55>
   412dc:	eb c2                	jmp    412a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   412de:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   412e5:	eb 16                	jmp    412fd <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x11d>
   412e7:	4c 29 ee             	sub    rsi,r13
   412ea:	48 89 f0             	mov    rax,rsi
   412ed:	eb 0e                	jmp    412fd <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x11d>
   412ef:	48 85 f6             	test   rsi,rsi
   412f2:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   412f9:	48 0f 49 c6          	cmovns rax,rsi
   412fd:	48 83 c4 08          	add    rsp,0x8
   41301:	5b                   	pop    rbx
   41302:	41 5c                	pop    r12
   41304:	41 5d                	pop    r13
   41306:	41 5e                	pop    r14
   41308:	41 5f                	pop    r15
   4130a:	5d                   	pop    rbp
   4130b:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000041010 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates>:
   41010:	55                   	push   rbp
   41011:	41 57                	push   r15
   41013:	41 56                	push   r14
   41015:	41 55                	push   r13
   41017:	41 54                	push   r12
   41019:	53                   	push   rbx
   4101a:	48 83 ec 18          	sub    rsp,0x18
   4101e:	48 89 cb             	mov    rbx,rcx
   41021:	49 89 d7             	mov    r15,rdx
   41024:	49 89 fc             	mov    r12,rdi
   41027:	48 8b 4c 24 50       	mov    rcx,QWORD PTR [rsp+0x50]
   4102c:	49 83 f8 01          	cmp    r8,0x1
   41030:	0f 8e 57 01 00 00    	jle    4118d <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x17d>
   41036:	4d 89 c5             	mov    r13,r8
   41039:	49 ff c8             	dec    r8
   4103c:	4d 89 fe             	mov    r14,r15
   4103f:	4d 29 c6             	sub    r14,r8
   41042:	4c 39 f6             	cmp    rsi,r14
   41045:	0f 8d 70 01 00 00    	jge    411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   4104b:	4c 89 cd             	mov    rbp,r9
   4104e:	42 0f b6 44 2b ff    	movzx  eax,BYTE PTR [rbx+r13*1-0x1]
   41054:	41 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],cl
   41058:	48 89 0c 24          	mov    QWORD PTR [rsp],rcx
   4105c:	74 29                	je     41087 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x77>
   4105e:	48 ff c6             	inc    rsi
   41061:	4c 89 e7             	mov    rdi,r12
   41064:	4c 89 f2             	mov    rdx,r14
   41067:	4c 89 44 24 10       	mov    QWORD PTR [rsp+0x10],r8
   4106c:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
   41071:	e8 ea e8 ff ff       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   41076:	4c 8b 44 24 10       	mov    r8,QWORD PTR [rsp+0x10]
   4107b:	48 8b 0c 24          	mov    rcx,QWORD PTR [rsp]
   4107f:	48 89 c6             	mov    rsi,rax
   41082:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
   41087:	48 85 f6             	test   rsi,rsi
   4108a:	0f 88 2b 01 00 00    	js     411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   41090:	49 b9 01 01 01 01 01 	movabs r9,0x101010101010101
   41097:	01 01 01 
   4109a:	4c 0f af c8          	imul   r9,rax
   4109e:	4b 8d 04 04          	lea    rax,[r12+r8*1]
   410a2:	44 38 0c 06          	cmp    BYTE PTR [rsi+rax*1],r9b
   410a6:	74 25                	je     410cd <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xbd>
   410a8:	48 ff c6             	inc    rsi
   410ab:	4c 89 e7             	mov    rdi,r12
   410ae:	4c 89 f2             	mov    rdx,r14
   410b1:	4c 89 c1             	mov    rcx,r8
   410b4:	4c 8b 04 24          	mov    r8,QWORD PTR [rsp]
   410b8:	e8 f3 02 00 00       	call   413b0 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x10>
   410bd:	48 8b 0c 24          	mov    rcx,QWORD PTR [rsp]
   410c1:	48 89 c6             	mov    rsi,rax
   410c4:	48 85 c0             	test   rax,rax
   410c7:	0f 88 ee 00 00 00    	js     411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   410cd:	48 ff c6             	inc    rsi
   410d0:	4d 8d 75 f8          	lea    r14,[r13-0x8]
   410d4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   410db:	00 00 00 00 00 
   410e0:	b8 01 00 00 00       	mov    eax,0x1
   410e5:	48 85 c0             	test   rax,rax
   410e8:	74 76                	je     41160 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x150>
   410ea:	4c 89 fa             	mov    rdx,r15
   410ed:	48 29 f2             	sub    rdx,rsi
   410f0:	0f 8e c5 00 00 00    	jle    411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   410f6:	4c 39 f0             	cmp    rax,r14
   410f9:	7f 25                	jg     41120 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x110>
   410fb:	48 83 fa 07          	cmp    rdx,0x7
   410ff:	7e 1f                	jle    41120 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x110>
   41101:	49 8b 3c 34          	mov    rdi,QWORD PTR [r12+rsi*1]
   41105:	48 8b 14 03          	mov    rdx,QWORD PTR [rbx+rax*1]
   41109:	48 39 d7             	cmp    rdi,rdx
   4110c:	75 1b                	jne    41129 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x119>
   4110e:	48 83 c6 08          	add    rsi,0x8
   41112:	48 83 c0 08          	add    rax,0x8
   41116:	4c 39 e8             	cmp    rax,r13
   41119:	7c ca                	jl     410e5 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xd5>
   4111b:	e9 a4 00 00 00       	jmp    411c4 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1b4>
   41120:	41 0f b6 3c 34       	movzx  edi,BYTE PTR [r12+rsi*1]
   41125:	0f b6 14 03          	movzx  edx,BYTE PTR [rbx+rax*1]
   41129:	40 38 d7             	cmp    dil,dl
   4112c:	74 12                	je     41140 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x130>
   4112e:	66 90                	xchg   ax,ax
   41130:	48 85 c0             	test   rax,rax
   41133:	7e 10                	jle    41145 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x135>
   41135:	48 8b 44 c5 f8       	mov    rax,QWORD PTR [rbp+rax*8-0x8]
   4113a:	40 3a 3c 03          	cmp    dil,BYTE PTR [rbx+rax*1]
   4113e:	75 f0                	jne    41130 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x120>
   41140:	48 ff c0             	inc    rax
   41143:	eb 02                	jmp    41147 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x137>
   41145:	31 c0                	xor    eax,eax
   41147:	48 ff c6             	inc    rsi
   4114a:	4c 39 e8             	cmp    rax,r13
   4114d:	7c 96                	jl     410e5 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xd5>
   4114f:	eb 73                	jmp    411c4 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1b4>
   41151:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   41158:	0f 1f 84 00 00 00 00 
   4115f:	00 
   41160:	49 39 f7             	cmp    r15,rsi
   41163:	7e 56                	jle    411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   41165:	41 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],cl
   41169:	74 15                	je     41180 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x170>
   4116b:	48 ff c6             	inc    rsi
   4116e:	4c 89 e7             	mov    rdi,r12
   41171:	4c 89 fa             	mov    rdx,r15
   41174:	e8 e7 e7 ff ff       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   41179:	48 8b 0c 24          	mov    rcx,QWORD PTR [rsp]
   4117d:	48 89 c6             	mov    rsi,rax
   41180:	48 85 f6             	test   rsi,rsi
   41183:	78 36                	js     411bb <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ab>
   41185:	48 ff c6             	inc    rsi
   41188:	e9 53 ff ff ff       	jmp    410e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xd0>
   4118d:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   41194:	4c 39 fe             	cmp    rsi,r15
   41197:	7d 31                	jge    411ca <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ba>
   41199:	41 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],cl
   4119d:	74 28                	je     411c7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1b7>
   4119f:	48 ff c6             	inc    rsi
   411a2:	4c 89 e7             	mov    rdi,r12
   411a5:	4c 89 fa             	mov    rdx,r15
   411a8:	48 83 c4 18          	add    rsp,0x18
   411ac:	5b                   	pop    rbx
   411ad:	41 5c                	pop    r12
   411af:	41 5d                	pop    r13
   411b1:	41 5e                	pop    r14
   411b3:	41 5f                	pop    r15
   411b5:	5d                   	pop    rbp
   411b6:	e9 a5 e7 ff ff       	jmp    3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   411bb:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   411c2:	eb 06                	jmp    411ca <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x1ba>
   411c4:	4c 29 ee             	sub    rsi,r13
   411c7:	48 89 f0             	mov    rax,rsi
   411ca:	48 83 c4 18          	add    rsp,0x18
   411ce:	5b                   	pop    rbx
   411cf:	41 5c                	pop    r12
   411d1:	41 5d                	pop    r13
   411d3:	41 5e                	pop    r14
   411d5:	41 5f                	pop    r15
   411d7:	5d                   	pop    rbp
   411d8:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003df90 <cinch.typed.m.Main.work>:
   3df90:	eb 0e                	jmp    3dfa0 <cinch.typed.m.Main.work+0x10>

Disassembly of section .fini:
