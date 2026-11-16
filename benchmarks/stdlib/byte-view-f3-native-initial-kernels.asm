
build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000041320 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks>:
   41320:	41 57                	push   r15
   41322:	41 56                	push   r14
   41324:	53                   	push   rbx
   41325:	48 83 ec 10          	sub    rsp,0x10
   41329:	48 89 d0             	mov    rax,rdx
   4132c:	48 29 f0             	sub    rax,rsi
   4132f:	48 83 f8 08          	cmp    rax,0x8
   41333:	0f 8c cf 00 00 00    	jl     41408 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0xe8>
   41339:	48 b8 80 80 80 80 80 	movabs rax,0x8080808080808080
   41340:	80 80 80 
   41343:	49 ba 7f 7f 7f 7f 7f 	movabs r10,0x7f7f7f7f7f7f7f7f
   4134a:	7f 7f 7f 
   4134d:	4c 8d 1c 37          	lea    r11,[rdi+rsi*1]
   41351:	48 f7 de             	neg    rsi
   41354:	eb 24                	jmp    4137a <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x5a>
   41356:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   4135d:	00 00 00 
   41360:	48 8d 1c 32          	lea    rbx,[rdx+rsi*1]
   41364:	48 83 c3 f8          	add    rbx,0xfffffffffffffff8
   41368:	48 83 c6 f8          	add    rsi,0xfffffffffffffff8
   4136c:	49 83 c3 08          	add    r11,0x8
   41370:	48 83 fb 07          	cmp    rbx,0x7
   41374:	0f 8e 8b 00 00 00    	jle    41405 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0xe5>
   4137a:	49 8b 1b             	mov    rbx,QWORD PTR [r11]
   4137d:	4c 31 c3             	xor    rbx,r8
   41380:	49 89 de             	mov    r14,rbx
   41383:	4d 21 d6             	and    r14,r10
   41386:	4d 01 d6             	add    r14,r10
   41389:	49 09 de             	or     r14,rbx
   4138c:	49 21 c6             	and    r14,rax
   4138f:	49 39 c6             	cmp    r14,rax
   41392:	74 cc                	je     41360 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x40>
   41394:	49 31 c6             	xor    r14,rax
   41397:	4d 8b 3c 0b          	mov    r15,QWORD PTR [r11+rcx*1]
   4139b:	4d 31 cf             	xor    r15,r9
   4139e:	4c 89 fb             	mov    rbx,r15
   413a1:	4c 21 d3             	and    rbx,r10
   413a4:	4c 01 d3             	add    rbx,r10
   413a7:	4c 09 fb             	or     rbx,r15
   413aa:	48 f7 d3             	not    rbx
   413ad:	4c 21 f3             	and    rbx,r14
   413b0:	74 ae                	je     41360 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x40>
   413b2:	49 89 f7             	mov    r15,rsi
   413b5:	e8 36 c8 06 00       	call   adbf0 <Data.Machine.u32-to-i64.object-closure>
   413ba:	49 89 c6             	mov    r14,rax
   413bd:	48 89 df             	mov    rdi,rbx
   413c0:	e8 5b d3 04 00       	call   8e720 <Data.Machine.trailing-zeros-u64>
   413c5:	48 89 c3             	mov    rbx,rax
   413c8:	4c 89 f7             	mov    rdi,r14
   413cb:	e8 50 f6 40 00       	call   450a20 <cinch_dup>
   413d0:	48 89 5c 24 08       	mov    QWORD PTR [rsp+0x8],rbx
   413d5:	48 8d 54 24 08       	lea    rdx,[rsp+0x8]
   413da:	be 01 00 00 00       	mov    esi,0x1
   413df:	4c 89 f7             	mov    rdi,r14
   413e2:	41 ff 16             	call   QWORD PTR [r14]
   413e5:	be 03 00 00 00       	mov    esi,0x3
   413ea:	48 89 c7             	mov    rdi,rax
   413ed:	e8 ee d6 02 00       	call   6eae0 <Data.Machine.wrapping-shr-i64>
   413f2:	48 89 c3             	mov    rbx,rax
   413f5:	4c 89 f7             	mov    rdi,r14
   413f8:	e8 e3 f1 40 00       	call   4505e0 <cinch_drop>
   413fd:	48 89 d8             	mov    rax,rbx
   41400:	4c 29 f8             	sub    rax,r15
   41403:	eb 32                	jmp    41437 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x117>
   41405:	48 f7 de             	neg    rsi
   41408:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   4140f:	48 39 d6             	cmp    rsi,rdx
   41412:	7d 23                	jge    41437 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x117>
   41414:	48 01 f9             	add    rcx,rdi
   41417:	eb 0f                	jmp    41428 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x108>
   41419:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   41420:	48 ff c6             	inc    rsi
   41423:	48 39 f2             	cmp    rdx,rsi
   41426:	74 0f                	je     41437 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x117>
   41428:	44 38 04 37          	cmp    BYTE PTR [rdi+rsi*1],r8b
   4142c:	75 f2                	jne    41420 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x100>
   4142e:	44 38 0c 31          	cmp    BYTE PTR [rcx+rsi*1],r9b
   41432:	75 ec                	jne    41420 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks+0x100>
   41434:	48 89 f0             	mov    rax,rsi
   41437:	48 83 c4 10          	add    rsp,0x10
   4143b:	5b                   	pop    rbx
   4143c:	41 5e                	pop    r14
   4143e:	41 5f                	pop    r15
   41440:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


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
   3f9e5:	e8 06 e2 06 00       	call   adbf0 <Data.Machine.u32-to-i64.object-closure>
   3f9ea:	49 89 c6             	mov    r14,rax
   3f9ed:	48 89 df             	mov    rdi,rbx
   3f9f0:	e8 2b ed 04 00       	call   8e720 <Data.Machine.trailing-zeros-u64>
   3f9f5:	48 89 c3             	mov    rbx,rax
   3f9f8:	4c 89 f7             	mov    rdi,r14
   3f9fb:	e8 20 10 41 00       	call   450a20 <cinch_dup>
   3fa00:	48 89 5c 24 08       	mov    QWORD PTR [rsp+0x8],rbx
   3fa05:	48 8d 54 24 08       	lea    rdx,[rsp+0x8]
   3fa0a:	be 01 00 00 00       	mov    esi,0x1
   3fa0f:	4c 89 f7             	mov    rdi,r14
   3fa12:	41 ff 16             	call   QWORD PTR [r14]
   3fa15:	be 03 00 00 00       	mov    esi,0x3
   3fa1a:	48 89 c7             	mov    rdi,rax
   3fa1d:	e8 be f0 02 00       	call   6eae0 <Data.Machine.wrapping-shr-i64>
   3fa22:	48 89 c3             	mov    rbx,rax
   3fa25:	4c 89 f7             	mov    rdi,r14
   3fa28:	e8 b3 0b 41 00       	call   4505e0 <cinch_drop>
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

build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


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

build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000411c0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst>:
   411c0:	55                   	push   rbp
   411c1:	41 57                	push   r15
   411c3:	41 56                	push   r14
   411c5:	41 55                	push   r13
   411c7:	41 54                	push   r12
   411c9:	53                   	push   rbx
   411ca:	50                   	push   rax
   411cb:	4d 89 ce             	mov    r14,r9
   411ce:	4d 89 c5             	mov    r13,r8
   411d1:	49 89 cf             	mov    r15,rcx
   411d4:	49 89 fc             	mov    r12,rdi
   411d7:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
   411dc:	48 8b 6c 24 40       	mov    rbp,QWORD PTR [rsp+0x40]
   411e1:	49 8d 58 f8          	lea    rbx,[r8-0x8]
   411e5:	48 89 14 24          	mov    QWORD PTR [rsp],rdx
   411e9:	48 85 c0             	test   rax,rax
   411ec:	75 27                	jne    41215 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x55>
   411ee:	e9 8d 00 00 00       	jmp    41280 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   411f3:	48 83 c6 08          	add    rsi,0x8
   411f7:	48 83 c0 08          	add    rax,0x8
   411fb:	4c 39 e8             	cmp    rax,r13
   411fe:	0f 8d c3 00 00 00    	jge    412c7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x107>
   41204:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   4120b:	00 00 00 00 00 
   41210:	48 85 c0             	test   rax,rax
   41213:	74 6b                	je     41280 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   41215:	49 89 d1             	mov    r9,rdx
   41218:	49 29 f1             	sub    r9,rsi
   4121b:	0f 8e 9d 00 00 00    	jle    412be <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xfe>
   41221:	48 39 d8             	cmp    rax,rbx
   41224:	7f 2a                	jg     41250 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x90>
   41226:	49 83 f9 07          	cmp    r9,0x7
   4122a:	7e 24                	jle    41250 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x90>
   4122c:	4d 8b 0c 34          	mov    r9,QWORD PTR [r12+rsi*1]
   41230:	4d 8b 14 07          	mov    r10,QWORD PTR [r15+rax*1]
   41234:	4d 39 d1             	cmp    r9,r10
   41237:	74 ba                	je     411f3 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x33>
   41239:	45 38 d1             	cmp    r9b,r10b
   4123c:	75 22                	jne    41260 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xa0>
   4123e:	48 ff c0             	inc    rax
   41241:	eb 31                	jmp    41274 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xb4>
   41243:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   4124a:	84 00 00 00 00 00 
   41250:	45 0f b6 0c 34       	movzx  r9d,BYTE PTR [r12+rsi*1]
   41255:	45 0f b6 14 07       	movzx  r10d,BYTE PTR [r15+rax*1]
   4125a:	45 38 d1             	cmp    r9b,r10b
   4125d:	74 df                	je     4123e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x7e>
   4125f:	90                   	nop
   41260:	48 85 c0             	test   rax,rax
   41263:	7e 0d                	jle    41272 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xb2>
   41265:	49 8b 44 c6 f8       	mov    rax,QWORD PTR [r14+rax*8-0x8]
   4126a:	45 3a 0c 07          	cmp    r9b,BYTE PTR [r15+rax*1]
   4126e:	75 f0                	jne    41260 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xa0>
   41270:	eb cc                	jmp    4123e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x7e>
   41272:	31 c0                	xor    eax,eax
   41274:	48 ff c6             	inc    rsi
   41277:	4c 39 e8             	cmp    rax,r13
   4127a:	7c 94                	jl     41210 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x50>
   4127c:	eb 49                	jmp    412c7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x107>
   4127e:	66 90                	xchg   ax,ax
   41280:	48 39 f2             	cmp    rdx,rsi
   41283:	7e 39                	jle    412be <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xfe>
   41285:	41 38 2c 34          	cmp    BYTE PTR [r12+rsi*1],bpl
   41289:	74 15                	je     412a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xe0>
   4128b:	48 ff c6             	inc    rsi
   4128e:	4c 89 e7             	mov    rdi,r12
   41291:	48 89 e9             	mov    rcx,rbp
   41294:	e8 c7 e6 ff ff       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   41299:	48 8b 14 24          	mov    rdx,QWORD PTR [rsp]
   4129d:	48 89 c6             	mov    rsi,rax
   412a0:	48 85 f6             	test   rsi,rsi
   412a3:	78 2a                	js     412cf <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x10f>
   412a5:	49 83 fd 01          	cmp    r13,0x1
   412a9:	7e 24                	jle    412cf <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x10f>
   412ab:	48 ff c6             	inc    rsi
   412ae:	b8 01 00 00 00       	mov    eax,0x1
   412b3:	48 85 c0             	test   rax,rax
   412b6:	0f 85 59 ff ff ff    	jne    41215 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x55>
   412bc:	eb c2                	jmp    41280 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0xc0>
   412be:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   412c5:	eb 16                	jmp    412dd <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x11d>
   412c7:	4c 29 ee             	sub    rsi,r13
   412ca:	48 89 f0             	mov    rax,rsi
   412cd:	eb 0e                	jmp    412dd <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst+0x11d>
   412cf:	48 85 f6             	test   rsi,rsi
   412d2:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   412d9:	48 0f 49 c6          	cmovns rax,rsi
   412dd:	48 83 c4 08          	add    rsp,0x8
   412e1:	5b                   	pop    rbx
   412e2:	41 5c                	pop    r12
   412e4:	41 5d                	pop    r13
   412e6:	41 5e                	pop    r14
   412e8:	41 5f                	pop    r15
   412ea:	5d                   	pop    rbp
   412eb:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


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
   4101a:	50                   	push   rax
   4101b:	4d 89 c5             	mov    r13,r8
   4101e:	49 89 d6             	mov    r14,rdx
   41021:	49 89 fc             	mov    r12,rdi
   41024:	4c 8b 44 24 40       	mov    r8,QWORD PTR [rsp+0x40]
   41029:	49 83 fd 01          	cmp    r13,0x1
   4102d:	7e 72                	jle    410a1 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x91>
   4102f:	48 89 cb             	mov    rbx,rcx
   41032:	49 8d 4d ff          	lea    rcx,[r13-0x1]
   41036:	4c 89 f2             	mov    rdx,r14
   41039:	48 29 ca             	sub    rdx,rcx
   4103c:	48 39 d6             	cmp    rsi,rdx
   4103f:	0f 8d 53 01 00 00    	jge    41198 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x188>
   41045:	4c 89 cd             	mov    rbp,r9
   41048:	42 0f b6 44 2b ff    	movzx  eax,BYTE PTR [rbx+r13*1-0x1]
   4104e:	49 b9 01 01 01 01 01 	movabs r9,0x101010101010101
   41055:	01 01 01 
   41058:	4c 0f af c8          	imul   r9,rax
   4105c:	45 38 04 34          	cmp    BYTE PTR [r12+rsi*1],r8b
   41060:	4c 89 04 24          	mov    QWORD PTR [rsp],r8
   41064:	75 0a                	jne    41070 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x60>
   41066:	49 8d 04 0c          	lea    rax,[r12+rcx*1]
   4106a:	44 38 0c 06          	cmp    BYTE PTR [rsi+rax*1],r9b
   4106e:	74 0e                	je     4107e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x6e>
   41070:	48 ff c6             	inc    rsi
   41073:	4c 89 e7             	mov    rdi,r12
   41076:	e8 a5 02 00 00       	call   41320 <cinch.typed.m.Data$2eBytes$2eView.endpoint$2dblocks>
   4107b:	48 89 c6             	mov    rsi,rax
   4107e:	48 85 f6             	test   rsi,rsi
   41081:	0f 88 11 01 00 00    	js     41198 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x188>
   41087:	48 ff c6             	inc    rsi
   4108a:	4d 8d 7d f8          	lea    r15,[r13-0x8]
   4108e:	b8 01 00 00 00       	mov    eax,0x1
   41093:	48 8b 0c 24          	mov    rcx,QWORD PTR [rsp]
   41097:	48 85 c0             	test   rax,rax
   4109a:	75 59                	jne    410f5 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xe5>
   4109c:	e9 bf 00 00 00       	jmp    41160 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x150>
   410a1:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   410a8:	4c 39 f6             	cmp    rsi,r14
   410ab:	0f 8d f6 00 00 00    	jge    411a7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x197>
   410b1:	45 38 04 34          	cmp    BYTE PTR [r12+rsi*1],r8b
   410b5:	0f 84 e9 00 00 00    	je     411a4 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x194>
   410bb:	48 ff c6             	inc    rsi
   410be:	4c 89 e7             	mov    rdi,r12
   410c1:	4c 89 f2             	mov    rdx,r14
   410c4:	4c 89 c1             	mov    rcx,r8
   410c7:	48 83 c4 08          	add    rsp,0x8
   410cb:	5b                   	pop    rbx
   410cc:	41 5c                	pop    r12
   410ce:	41 5d                	pop    r13
   410d0:	41 5e                	pop    r14
   410d2:	41 5f                	pop    r15
   410d4:	5d                   	pop    rbp
   410d5:	e9 86 e8 ff ff       	jmp    3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   410da:	48 83 c6 08          	add    rsi,0x8
   410de:	48 83 c0 08          	add    rax,0x8
   410e2:	4c 39 e8             	cmp    rax,r13
   410e5:	0f 8d b6 00 00 00    	jge    411a1 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x191>
   410eb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   410f0:	48 85 c0             	test   rax,rax
   410f3:	74 6b                	je     41160 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x150>
   410f5:	4c 89 f2             	mov    rdx,r14
   410f8:	48 29 f2             	sub    rdx,rsi
   410fb:	0f 8e 97 00 00 00    	jle    41198 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x188>
   41101:	4c 39 f8             	cmp    rax,r15
   41104:	7f 2a                	jg     41130 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x120>
   41106:	48 83 fa 07          	cmp    rdx,0x7
   4110a:	7e 24                	jle    41130 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x120>
   4110c:	4d 8b 04 34          	mov    r8,QWORD PTR [r12+rsi*1]
   41110:	48 8b 14 03          	mov    rdx,QWORD PTR [rbx+rax*1]
   41114:	49 39 d0             	cmp    r8,rdx
   41117:	74 c1                	je     410da <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xca>
   41119:	41 38 d0             	cmp    r8b,dl
   4111c:	75 22                	jne    41140 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x130>
   4111e:	48 ff c0             	inc    rax
   41121:	eb 31                	jmp    41154 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x144>
   41123:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   4112a:	84 00 00 00 00 00 
   41130:	45 0f b6 04 34       	movzx  r8d,BYTE PTR [r12+rsi*1]
   41135:	0f b6 14 03          	movzx  edx,BYTE PTR [rbx+rax*1]
   41139:	41 38 d0             	cmp    r8b,dl
   4113c:	74 e0                	je     4111e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x10e>
   4113e:	66 90                	xchg   ax,ax
   41140:	48 85 c0             	test   rax,rax
   41143:	7e 0d                	jle    41152 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x142>
   41145:	48 8b 44 c5 f8       	mov    rax,QWORD PTR [rbp+rax*8-0x8]
   4114a:	44 3a 04 03          	cmp    r8b,BYTE PTR [rbx+rax*1]
   4114e:	75 f0                	jne    41140 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x130>
   41150:	eb cc                	jmp    4111e <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x10e>
   41152:	31 c0                	xor    eax,eax
   41154:	48 ff c6             	inc    rsi
   41157:	4c 39 e8             	cmp    rax,r13
   4115a:	7c 94                	jl     410f0 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xe0>
   4115c:	eb 43                	jmp    411a1 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x191>
   4115e:	66 90                	xchg   ax,ax
   41160:	49 39 f6             	cmp    r14,rsi
   41163:	7e 33                	jle    41198 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x188>
   41165:	41 38 0c 34          	cmp    BYTE PTR [r12+rsi*1],cl
   41169:	74 15                	je     41180 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x170>
   4116b:	48 ff c6             	inc    rsi
   4116e:	4c 89 e7             	mov    rdi,r12
   41171:	4c 89 f2             	mov    rdx,r14
   41174:	e8 e7 e7 ff ff       	call   3f960 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   41179:	48 8b 0c 24          	mov    rcx,QWORD PTR [rsp]
   4117d:	48 89 c6             	mov    rsi,rax
   41180:	48 85 f6             	test   rsi,rsi
   41183:	78 13                	js     41198 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x188>
   41185:	48 ff c6             	inc    rsi
   41188:	b8 01 00 00 00       	mov    eax,0x1
   4118d:	48 85 c0             	test   rax,rax
   41190:	0f 85 5f ff ff ff    	jne    410f5 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0xe5>
   41196:	eb c8                	jmp    41160 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x150>
   41198:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
   4119f:	eb 06                	jmp    411a7 <cinch.typed.m.Data$2eBytes$2eView.search$2dfirst$2dcandidates+0x197>
   411a1:	4c 29 ee             	sub    rsi,r13
   411a4:	48 89 f0             	mov    rax,rsi
   411a7:	48 83 c4 08          	add    rsp,0x8
   411ab:	5b                   	pop    rbx
   411ac:	41 5c                	pop    r12
   411ae:	41 5d                	pop    r13
   411b0:	41 5e                	pop    r14
   411b2:	41 5f                	pop    r15
   411b4:	5d                   	pop    rbp
   411b5:	c3                   	ret

Disassembly of section .fini:

build/byte-view-bench/candidate3-native/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003df90 <cinch.typed.m.Main.work>:
   3df90:	eb 0e                	jmp    3dfa0 <cinch.typed.m.Main.work+0x10>

Disassembly of section .fini:
