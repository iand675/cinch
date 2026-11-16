
/workspace/dasl-stdlib-strings/build/byte-search-benchmark-candidate1/native/packed/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003ebd0 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward>:
   3ebd0:	53                   	push   %rbx
   3ebd1:	4c 89 c0             	mov    %r8,%rax
   3ebd4:	49 89 d1             	mov    %rdx,%r9
   3ebd7:	49 29 f1             	sub    %rsi,%r9
   3ebda:	49 83 f9 08          	cmp    $0x8,%r9
   3ebde:	7d 08                	jge    3ebe8 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x18>
   3ebe0:	49 89 f0             	mov    %rsi,%r8
   3ebe3:	e9 ae 01 00 00       	jmp    3ed96 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x1c6>
   3ebe8:	49 83 c1 f8          	add    $0xfffffffffffffff8,%r9
   3ebec:	49 83 f9 18          	cmp    $0x18,%r9
   3ebf0:	73 08                	jae    3ebfa <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x2a>
   3ebf2:	49 89 f0             	mov    %rsi,%r8
   3ebf5:	e9 45 01 00 00       	jmp    3ed3f <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x16f>
   3ebfa:	49 c1 e9 03          	shr    $0x3,%r9
   3ebfe:	49 ff c1             	inc    %r9
   3ec01:	4d 89 ca             	mov    %r9,%r10
   3ec04:	49 83 e2 fc          	and    $0xfffffffffffffffc,%r10
   3ec08:	4e 8d 04 d6          	lea    (%rsi,%r10,8),%r8
   3ec0c:	66 48 0f 6e c0       	movq   %rax,%xmm0
   3ec11:	66 48 0f 6e c9       	movq   %rcx,%xmm1
   3ec16:	66 0f 70 d1 44       	pshufd $0x44,%xmm1,%xmm2
   3ec1b:	48 8d 04 3e          	lea    (%rsi,%rdi,1),%rax
   3ec1f:	48 83 c0 10          	add    $0x10,%rax
   3ec23:	66 0f ef c9          	pxor   %xmm1,%xmm1
   3ec27:	31 f6                	xor    %esi,%esi
   3ec29:	66 0f 6f 1d bf a4 4a 	movdqa 0x4aa4bf(%rip),%xmm3        # 4e90f0 <_IO_stdin_used+0xf0>
   3ec30:	00 
   3ec31:	66 0f 76 e4          	pcmpeqd %xmm4,%xmm4
   3ec35:	66 0f 6f 2d c3 a4 4a 	movdqa 0x4aa4c3(%rip),%xmm5        # 4e9100 <_IO_stdin_used+0x100>
   3ec3c:	00 
   3ec3d:	66 0f 6f 35 cb a4 4a 	movdqa 0x4aa4cb(%rip),%xmm6        # 4e9110 <_IO_stdin_used+0x110>
   3ec44:	00 
   3ec45:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   3ec4c:	00 00 00 00 
   3ec50:	f3 44 0f 6f 4c f0 f0 	movdqu -0x10(%rax,%rsi,8),%xmm9
   3ec57:	f3 44 0f 6f 14 f0    	movdqu (%rax,%rsi,8),%xmm10
   3ec5d:	66 44 0f ef ca       	pxor   %xmm2,%xmm9
   3ec62:	66 44 0f ef d2       	pxor   %xmm2,%xmm10
   3ec67:	66 45 0f 6f c1       	movdqa %xmm9,%xmm8
   3ec6c:	66 44 0f db c3       	pand   %xmm3,%xmm8
   3ec71:	66 41 0f 6f fa       	movdqa %xmm10,%xmm7
   3ec76:	66 0f db fb          	pand   %xmm3,%xmm7
   3ec7a:	66 44 0f d4 c3       	paddq  %xmm3,%xmm8
   3ec7f:	66 0f d4 fb          	paddq  %xmm3,%xmm7
   3ec83:	66 45 0f eb c1       	por    %xmm9,%xmm8
   3ec88:	66 41 0f eb fa       	por    %xmm10,%xmm7
   3ec8d:	66 44 0f ef c4       	pxor   %xmm4,%xmm8
   3ec92:	66 41 0f 73 d0 07    	psrlq  $0x7,%xmm8
   3ec98:	66 0f ef fc          	pxor   %xmm4,%xmm7
   3ec9c:	66 0f 73 d7 07       	psrlq  $0x7,%xmm7
   3eca1:	66 44 0f db c5       	pand   %xmm5,%xmm8
   3eca6:	66 0f db fd          	pand   %xmm5,%xmm7
   3ecaa:	66 45 0f 6f c8       	movdqa %xmm8,%xmm9
   3ecaf:	66 44 0f f4 ce       	pmuludq %xmm6,%xmm9
   3ecb4:	66 45 0f 6f d0       	movdqa %xmm8,%xmm10
   3ecb9:	66 41 0f 73 d2 20    	psrlq  $0x20,%xmm10
   3ecbf:	66 44 0f f4 d5       	pmuludq %xmm5,%xmm10
   3ecc4:	66 45 0f d4 d1       	paddq  %xmm9,%xmm10
   3ecc9:	66 41 0f 73 f2 20    	psllq  $0x20,%xmm10
   3eccf:	66 44 0f f4 c5       	pmuludq %xmm5,%xmm8
   3ecd4:	66 45 0f d4 c2       	paddq  %xmm10,%xmm8
   3ecd9:	66 44 0f 6f cf       	movdqa %xmm7,%xmm9
   3ecde:	66 44 0f f4 ce       	pmuludq %xmm6,%xmm9
   3ece3:	66 44 0f 6f d7       	movdqa %xmm7,%xmm10
   3ece8:	66 41 0f 73 d2 20    	psrlq  $0x20,%xmm10
   3ecee:	66 44 0f f4 d5       	pmuludq %xmm5,%xmm10
   3ecf3:	66 45 0f d4 d1       	paddq  %xmm9,%xmm10
   3ecf8:	66 41 0f 73 f2 20    	psllq  $0x20,%xmm10
   3ecfe:	66 0f f4 fd          	pmuludq %xmm5,%xmm7
   3ed02:	66 41 0f d4 fa       	paddq  %xmm10,%xmm7
   3ed07:	66 41 0f 73 d0 38    	psrlq  $0x38,%xmm8
   3ed0d:	66 41 0f d4 c0       	paddq  %xmm8,%xmm0
   3ed12:	66 0f 73 d7 38       	psrlq  $0x38,%xmm7
   3ed17:	66 0f d4 cf          	paddq  %xmm7,%xmm1
   3ed1b:	48 83 c6 04          	add    $0x4,%rsi
   3ed1f:	49 39 f2             	cmp    %rsi,%r10
   3ed22:	0f 85 28 ff ff ff    	jne    3ec50 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x80>
   3ed28:	66 0f d4 c8          	paddq  %xmm0,%xmm1
   3ed2c:	66 0f 70 c1 ee       	pshufd $0xee,%xmm1,%xmm0
   3ed31:	66 0f d4 c1          	paddq  %xmm1,%xmm0
   3ed35:	66 48 0f 7e c0       	movq   %xmm0,%rax
   3ed3a:	4d 39 d1             	cmp    %r10,%r9
   3ed3d:	74 57                	je     3ed96 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x1c6>
   3ed3f:	48 be 7f 7f 7f 7f 7f 	movabs $0x7f7f7f7f7f7f7f7f,%rsi
   3ed46:	7f 7f 7f 
   3ed49:	49 b9 01 01 01 01 01 	movabs $0x101010101010101,%r9
   3ed50:	01 01 01 
   3ed53:	49 89 d2             	mov    %rdx,%r10
   3ed56:	4d 29 c2             	sub    %r8,%r10
   3ed59:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   3ed60:	4e 8b 1c 07          	mov    (%rdi,%r8,1),%r11
   3ed64:	49 31 cb             	xor    %rcx,%r11
   3ed67:	4c 89 db             	mov    %r11,%rbx
   3ed6a:	48 21 f3             	and    %rsi,%rbx
   3ed6d:	48 01 f3             	add    %rsi,%rbx
   3ed70:	4c 09 db             	or     %r11,%rbx
   3ed73:	48 f7 d3             	not    %rbx
   3ed76:	48 c1 eb 07          	shr    $0x7,%rbx
   3ed7a:	4c 21 cb             	and    %r9,%rbx
   3ed7d:	49 0f af d9          	imul   %r9,%rbx
   3ed81:	48 c1 eb 38          	shr    $0x38,%rbx
   3ed85:	48 01 d8             	add    %rbx,%rax
   3ed88:	49 83 c0 08          	add    $0x8,%r8
   3ed8c:	49 83 c2 f8          	add    $0xfffffffffffffff8,%r10
   3ed90:	49 83 fa 07          	cmp    $0x7,%r10
   3ed94:	7f ca                	jg     3ed60 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x190>
   3ed96:	48 89 d6             	mov    %rdx,%rsi
   3ed99:	4c 29 c6             	sub    %r8,%rsi
   3ed9c:	0f 8e c3 00 00 00    	jle    3ee65 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x295>
   3eda2:	48 83 fe 04          	cmp    $0x4,%rsi
   3eda6:	0f 82 a4 00 00 00    	jb     3ee50 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x280>
   3edac:	49 89 f1             	mov    %rsi,%r9
   3edaf:	49 83 e1 fc          	and    $0xfffffffffffffffc,%r9
   3edb3:	66 48 0f 6e c0       	movq   %rax,%xmm0
   3edb8:	0f b6 c1             	movzbl %cl,%eax
   3edbb:	66 0f 6e c8          	movd   %eax,%xmm1
   3edbf:	66 0f 60 c9          	punpcklbw %xmm1,%xmm1
   3edc3:	49 8d 04 38          	lea    (%r8,%rdi,1),%rax
   3edc7:	48 83 c0 02          	add    $0x2,%rax
   3edcb:	4d 01 c8             	add    %r9,%r8
   3edce:	66 0f ef d2          	pxor   %xmm2,%xmm2
   3edd2:	45 31 d2             	xor    %r10d,%r10d
   3edd5:	66 0f 6f 1d 43 a3 4a 	movdqa 0x4aa343(%rip),%xmm3        # 4e9120 <_IO_stdin_used+0x120>
   3eddc:	00 
   3eddd:	0f 1f 00             	nopl   (%rax)
   3ede0:	46 0f b7 5c 10 fe    	movzwl -0x2(%rax,%r10,1),%r11d
   3ede6:	66 41 0f 6e e3       	movd   %r11d,%xmm4
   3edeb:	46 0f b7 1c 10       	movzwl (%rax,%r10,1),%r11d
   3edf0:	66 41 0f 6e eb       	movd   %r11d,%xmm5
   3edf5:	66 0f 74 e1          	pcmpeqb %xmm1,%xmm4
   3edf9:	66 0f 60 e4          	punpcklbw %xmm4,%xmm4
   3edfd:	f2 0f 70 e4 d4       	pshuflw $0xd4,%xmm4,%xmm4
   3ee02:	66 0f 70 e4 d4       	pshufd $0xd4,%xmm4,%xmm4
   3ee07:	66 0f db e3          	pand   %xmm3,%xmm4
   3ee0b:	66 0f d4 c4          	paddq  %xmm4,%xmm0
   3ee0f:	66 0f 74 e9          	pcmpeqb %xmm1,%xmm5
   3ee13:	66 0f 60 ed          	punpcklbw %xmm5,%xmm5
   3ee17:	f2 0f 70 e5 d4       	pshuflw $0xd4,%xmm5,%xmm4
   3ee1c:	66 0f 70 e4 d4       	pshufd $0xd4,%xmm4,%xmm4
   3ee21:	66 0f db e3          	pand   %xmm3,%xmm4
   3ee25:	66 0f d4 d4          	paddq  %xmm4,%xmm2
   3ee29:	49 83 c2 04          	add    $0x4,%r10
   3ee2d:	4d 39 d1             	cmp    %r10,%r9
   3ee30:	75 ae                	jne    3ede0 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x210>
   3ee32:	66 0f d4 d0          	paddq  %xmm0,%xmm2
   3ee36:	66 0f 70 c2 ee       	pshufd $0xee,%xmm2,%xmm0
   3ee3b:	66 0f d4 c2          	paddq  %xmm2,%xmm0
   3ee3f:	66 48 0f 7e c0       	movq   %xmm0,%rax
   3ee44:	4c 39 ce             	cmp    %r9,%rsi
   3ee47:	74 1c                	je     3ee65 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x295>
   3ee49:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   3ee50:	31 f6                	xor    %esi,%esi
   3ee52:	42 38 0c 07          	cmp    %cl,(%rdi,%r8,1)
   3ee56:	40 0f 94 c6          	sete   %sil
   3ee5a:	48 01 f0             	add    %rsi,%rax
   3ee5d:	49 ff c0             	inc    %r8
   3ee60:	4c 39 c2             	cmp    %r8,%rdx
   3ee63:	75 eb                	jne    3ee50 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x280>
   3ee65:	5b                   	pop    %rbx
   3ee66:	c3                   	ret

Disassembly of section .fini:
