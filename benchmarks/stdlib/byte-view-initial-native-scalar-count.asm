
build/byte-view-bench/native-final/scalar/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003f750 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount>:
   3f750:	55                   	push   rbp
   3f751:	41 56                	push   r14
   3f753:	53                   	push   rbx
   3f754:	48 8b 44 24 30       	mov    rax,QWORD PTR [rsp+0x30]
   3f759:	48 8b 5c 24 28       	mov    rbx,QWORD PTR [rsp+0x28]
   3f75e:	44 0f b6 54 24 20    	movzx  r10d,BYTE PTR [rsp+0x20]
   3f764:	45 31 db             	xor    r11d,r11d
   3f767:	48 85 db             	test   rbx,rbx
   3f76a:	74 34                	je     3f7a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x50>
   3f76c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
   3f770:	48 39 d6             	cmp    rsi,rdx
   3f773:	7d 7d                	jge    3f7f2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xa2>
   3f775:	0f b6 2c 37          	movzx  ebp,BYTE PTR [rdi+rsi*1]
   3f779:	40 3a 2c 19          	cmp    bpl,BYTE PTR [rcx+rbx*1]
   3f77d:	74 11                	je     3f790 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x40>
   3f77f:	90                   	nop
   3f780:	48 85 db             	test   rbx,rbx
   3f783:	7e 3b                	jle    3f7c0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x70>
   3f785:	49 8b 5c d9 f8       	mov    rbx,QWORD PTR [r9+rbx*8-0x8]
   3f78a:	40 3a 2c 19          	cmp    bpl,BYTE PTR [rcx+rbx*1]
   3f78e:	75 f0                	jne    3f780 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x30>
   3f790:	48 ff c3             	inc    rbx
   3f793:	eb 2d                	jmp    3f7c2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x72>
   3f795:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3f79c:	00 00 00 00 
   3f7a0:	48 39 d6             	cmp    rsi,rdx
   3f7a3:	7d 4d                	jge    3f7f2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xa2>
   3f7a5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3f7ac:	00 00 00 00 
   3f7b0:	44 38 14 37          	cmp    BYTE PTR [rdi+rsi*1],r10b
   3f7b4:	74 2a                	je     3f7e0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x90>
   3f7b6:	48 ff c6             	inc    rsi
   3f7b9:	48 39 f2             	cmp    rdx,rsi
   3f7bc:	75 f2                	jne    3f7b0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x60>
   3f7be:	eb 32                	jmp    3f7f2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xa2>
   3f7c0:	31 db                	xor    ebx,ebx
   3f7c2:	48 ff c6             	inc    rsi
   3f7c5:	45 31 f6             	xor    r14d,r14d
   3f7c8:	4c 39 c3             	cmp    rbx,r8
   3f7cb:	41 0f 9d c6          	setge  r14b
   3f7cf:	49 0f 4d db          	cmovge rbx,r11
   3f7d3:	4c 01 f0             	add    rax,r14
   3f7d6:	48 85 db             	test   rbx,rbx
   3f7d9:	75 95                	jne    3f770 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x20>
   3f7db:	eb c3                	jmp    3f7a0 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x50>
   3f7dd:	0f 1f 00             	nop    DWORD PTR [rax]
   3f7e0:	48 85 f6             	test   rsi,rsi
   3f7e3:	78 0d                	js     3f7f2 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0xa2>
   3f7e5:	48 ff c6             	inc    rsi
   3f7e8:	bb 01 00 00 00       	mov    ebx,0x1
   3f7ed:	48 39 d6             	cmp    rsi,rdx
   3f7f0:	7c 83                	jl     3f775 <cinch.typed.m.Data$2eBytes$2eView.search$2dcount+0x25>
   3f7f2:	5b                   	pop    rbx
   3f7f3:	41 5e                	pop    r14
   3f7f5:	5d                   	pop    rbp
   3f7f6:	c3                   	ret

Disassembly of section .fini:
