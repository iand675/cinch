
build/byte-view-index-byte-bench-native/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003e670 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte>:
   3e670:	55                   	push   %rbp
   3e671:	41 57                	push   %r15
   3e673:	41 56                	push   %r14
   3e675:	41 55                	push   %r13
   3e677:	41 54                	push   %r12
   3e679:	53                   	push   %rbx
   3e67a:	48 83 ec 18          	sub    $0x18,%rsp
   3e67e:	48 89 f5             	mov    %rsi,%rbp
   3e681:	48 89 fb             	mov    %rdi,%rbx
   3e684:	e8 d7 ce 41 00       	call   45b560 <cinch_dup>
   3e689:	48 8b 43 f8          	mov    -0x8(%rbx),%rax
   3e68d:	4c 8b 3b             	mov    (%rbx),%r15
   3e690:	4c 8b 63 08          	mov    0x8(%rbx),%r12
   3e694:	4c 8b 6b 10          	mov    0x10(%rbx),%r13
   3e698:	4c 89 e7             	mov    %r12,%rdi
   3e69b:	e8 c0 ce 41 00       	call   45b560 <cinch_dup>
   3e6a0:	4c 89 ef             	mov    %r13,%rdi
   3e6a3:	e8 b8 ce 41 00       	call   45b560 <cinch_dup>
   3e6a8:	4c 89 e7             	mov    %r12,%rdi
   3e6ab:	4c 89 ee             	mov    %r13,%rsi
   3e6ae:	e8 fd 13 12 00       	call   15fab0 <Data.Nat.Eq_Nat.==>
   3e6b3:	4c 8d 35 8e 1a 4d 00 	lea    0x4d1a8e(%rip),%r14        # 510148 <_IO_stdin_used+0x148>
   3e6ba:	48 85 c0             	test   %rax,%rax
   3e6bd:	74 22                	je     3e6e1 <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0x71>
   3e6bf:	48 89 df             	mov    %rbx,%rdi
   3e6c2:	e8 59 ca 41 00       	call   45b120 <cinch_drop>
   3e6c7:	48 89 df             	mov    %rbx,%rdi
   3e6ca:	e8 51 ca 41 00       	call   45b120 <cinch_drop>
   3e6cf:	4c 89 f0             	mov    %r14,%rax
   3e6d2:	48 83 c4 18          	add    $0x18,%rsp
   3e6d6:	5b                   	pop    %rbx
   3e6d7:	41 5c                	pop    %r12
   3e6d9:	41 5d                	pop    %r13
   3e6db:	41 5e                	pop    %r14
   3e6dd:	41 5f                	pop    %r15
   3e6df:	5d                   	pop    %rbp
   3e6e0:	c3                   	ret
   3e6e1:	48 89 6c 24 10       	mov    %rbp,0x10(%rsp)
   3e6e6:	4c 89 e7             	mov    %r12,%rdi
   3e6e9:	e8 72 ce 41 00       	call   45b560 <cinch_dup>
   3e6ee:	4c 89 e7             	mov    %r12,%rdi
   3e6f1:	e8 ca cd 00 00       	call   4b4c0 <Data.Nat.nat-to-i64>
   3e6f6:	49 89 c4             	mov    %rax,%r12
   3e6f9:	4c 89 ef             	mov    %r13,%rdi
   3e6fc:	e8 5f ce 41 00       	call   45b560 <cinch_dup>
   3e701:	4c 89 ef             	mov    %r13,%rdi
   3e704:	e8 b7 cd 00 00       	call   4b4c0 <Data.Nat.nat-to-i64>
   3e709:	49 89 c5             	mov    %rax,%r13
   3e70c:	4c 89 ff             	mov    %r15,%rdi
   3e70f:	e8 4c ce 41 00       	call   45b560 <cinch_dup>
   3e714:	4c 89 ff             	mov    %r15,%rdi
   3e717:	e8 14 9a 0b 00       	call   f8130 <Data.Bytes.bytes-data>
   3e71c:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   3e721:	4c 89 ff             	mov    %r15,%rdi
   3e724:	e8 37 ce 41 00       	call   45b560 <cinch_dup>
   3e729:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   3e730:	4d 39 ec             	cmp    %r13,%r12
   3e733:	7d 36                	jge    3e76b <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0xfb>
   3e735:	0f b6 44 24 10       	movzbl 0x10(%rsp),%eax
   3e73a:	48 b9 01 01 01 01 01 	movabs $0x101010101010101,%rcx
   3e741:	01 01 01
   3e744:	48 0f af c8          	imul   %rax,%rcx
   3e748:	4c 89 e5             	mov    %r12,%rbp
   3e74b:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   3e750:	42 38 0c 20          	cmp    %cl,(%rax,%r12,1)
   3e754:	74 15                	je     3e76b <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0xfb>
   3e756:	49 8d 74 24 01       	lea    0x1(%r12),%rsi
   3e75b:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   3e760:	4c 89 ea             	mov    %r13,%rdx
   3e763:	e8 08 01 00 00       	call   3e870 <cinch.typed.m.Data$2eBytes$2eView.index$2dforward$2dblocks>
   3e768:	48 89 c5             	mov    %rax,%rbp
   3e76b:	4c 89 ff             	mov    %r15,%rdi
   3e76e:	e8 ed cd 41 00       	call   45b560 <cinch_dup>
   3e773:	4c 89 ff             	mov    %r15,%rdi
   3e776:	e8 15 bd 41 00       	call   45a490 <cinch_bytes_len>
   3e77b:	48 89 c7             	mov    %rax,%rdi
   3e77e:	e8 1d 28 42 00       	call   460fa0 <cinch_nat_from_i64>
   3e783:	49 89 c5             	mov    %rax,%r13
   3e786:	4c 89 ff             	mov    %r15,%rdi
   3e789:	e8 92 c9 41 00       	call   45b120 <cinch_drop>
   3e78e:	4c 89 ef             	mov    %r13,%rdi
   3e791:	e8 8a c9 41 00       	call   45b120 <cinch_drop>
   3e796:	4c 89 ff             	mov    %r15,%rdi
   3e799:	e8 82 c9 41 00       	call   45b120 <cinch_drop>
   3e79e:	48 89 ef             	mov    %rbp,%rdi
   3e7a1:	4c 89 e6             	mov    %r12,%rsi
   3e7a4:	e8 f7 7b 1e 00       	call   2263a0 <Data.Machine.Ord_I64.>=>
   3e7a9:	48 85 c0             	test   %rax,%rax
   3e7ac:	0f 84 0d ff ff ff    	je     3e6bf <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0x4f>
   3e7b2:	4c 29 e5             	sub    %r12,%rbp
   3e7b5:	31 ff                	xor    %edi,%edi
   3e7b7:	48 89 ee             	mov    %rbp,%rsi
   3e7ba:	e8 61 75 1e 00       	call   225d20 <Data.Machine.Ord_I64.<=>
   3e7bf:	48 85 c0             	test   %rax,%rax
   3e7c2:	0f 84 f7 fe ff ff    	je     3e6bf <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0x4f>
   3e7c8:	48 89 ef             	mov    %rbp,%rdi
   3e7cb:	e8 c0 c9 00 00       	call   4b190 <Data.Nat.nat-from-i64>
   3e7d0:	49 89 c7             	mov    %rax,%r15
   3e7d3:	bf 08 00 00 00       	mov    $0x8,%edi
   3e7d8:	be 01 01 00 00       	mov    $0x101,%esi
   3e7dd:	e8 6e a5 41 00       	call   458d50 <cinch_alloc_ctor>
   3e7e2:	49 89 c6             	mov    %rax,%r14
   3e7e5:	4c 89 38             	mov    %r15,(%rax)
   3e7e8:	e9 d2 fe ff ff       	jmp    3e6bf <cinch.typed.m.Data$2eBytes$2eView.byte$2dview$2dindex$2dbyte+0x4f>

Disassembly of section .fini:
