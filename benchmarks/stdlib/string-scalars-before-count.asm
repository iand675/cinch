
/workspace/dasl-stdlib-strings/build/string-scalars-benchmark-phase1/before/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

00000000000247a0 <Data.String.string-length-from>:
   247a0:	55                   	push   %rbp
   247a1:	48 89 e5             	mov    %rsp,%rbp
   247a4:	41 57                	push   %r15
   247a6:	41 56                	push   %r14
   247a8:	41 55                	push   %r13
   247aa:	41 54                	push   %r12
   247ac:	53                   	push   %rbx
   247ad:	48 83 ec 38          	sub    $0x38,%rsp
   247b1:	49 89 d7             	mov    %rdx,%r15
   247b4:	49 89 f5             	mov    %rsi,%r13
   247b7:	49 89 fe             	mov    %rdi,%r14
   247ba:	49 bc ff ff ff ff ff 	movabs $0x7fffffffffffffff,%r12
   247c1:	ff ff 7f 
   247c4:	48 89 7d d0          	mov    %rdi,-0x30(%rbp)
   247c8:	4d 85 f6             	test   %r14,%r14
   247cb:	74 23                	je     247f0 <Data.String.string-length-from+0x50>
   247cd:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   247d1:	48 89 c1             	mov    %rax,%rcx
   247d4:	48 c1 e1 1d          	shl    $0x1d,%rcx
   247d8:	48 85 c9             	test   %rcx,%rcx
   247db:	7e 23                	jle    24800 <Data.String.string-length-from+0x60>
   247dd:	48 ff c0             	inc    %rax
   247e0:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   247e4:	eb 24                	jmp    2480a <Data.String.string-length-from+0x6a>
   247e6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   247ed:	00 00 00 
   247f0:	31 ff                	xor    %edi,%edi
   247f2:	e8 a9 48 04 00       	call   690a0 <cinch_bytes_len>
   247f7:	e9 87 00 00 00       	jmp    24883 <Data.String.string-length-from+0xe3>
   247fc:	0f 1f 40 00          	nopl   0x0(%rax)
   24800:	74 08                	je     2480a <Data.String.string-length-from+0x6a>
   24802:	4c 89 f7             	mov    %r14,%rdi
   24805:	e8 66 5a 04 00       	call   6a270 <cinch_dup_mt>
   2480a:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2480e:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   24815:	00 00 00 
   24818:	48 21 c8             	and    %rcx,%rax
   2481b:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   24822:	00 00 00 
   24825:	48 39 c8             	cmp    %rcx,%rax
   24828:	75 06                	jne    24830 <Data.String.string-length-from+0x90>
   2482a:	49 8b 06             	mov    (%r14),%rax
   2482d:	eb 09                	jmp    24838 <Data.String.string-length-from+0x98>
   2482f:	90                   	nop
   24830:	4c 89 f7             	mov    %r14,%rdi
   24833:	e8 68 48 04 00       	call   690a0 <cinch_bytes_len>
   24838:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   2483c:	48 89 d1             	mov    %rdx,%rcx
   2483f:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24843:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2484a:	7c 14                	jl     24860 <Data.String.string-length-from+0xc0>
   2484c:	48 ff ca             	dec    %rdx
   2484f:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   24853:	eb 2e                	jmp    24883 <Data.String.string-length-from+0xe3>
   24855:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2485c:	00 00 00 00 
   24860:	48 85 c9             	test   %rcx,%rcx
   24863:	74 1e                	je     24883 <Data.String.string-length-from+0xe3>
   24865:	48 89 c3             	mov    %rax,%rbx
   24868:	4c 89 f7             	mov    %r14,%rdi
   2486b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   24872:	75 07                	jne    2487b <Data.String.string-length-from+0xdb>
   24874:	e8 17 57 04 00       	call   69f90 <cinch_drop_slow>
   24879:	eb 05                	jmp    24880 <Data.String.string-length-from+0xe0>
   2487b:	e8 20 56 04 00       	call   69ea0 <cinch_drop_mt>
   24880:	48 89 d8             	mov    %rbx,%rax
   24883:	48 8d 34 45 01 00 00 	lea    0x1(,%rax,2),%rsi
   2488a:	00 
   2488b:	4d 85 ed             	test   %r13,%r13
   2488e:	0f 94 c1             	sete   %cl
   24891:	44 89 ef             	mov    %r13d,%edi
   24894:	83 e7 01             	and    $0x1,%edi
   24897:	44 89 ea             	mov    %r13d,%edx
   2489a:	08 ca                	or     %cl,%dl
   2489c:	88 55 c8             	mov    %dl,-0x38(%rbp)
   2489f:	f6 c2 01             	test   $0x1,%dl
   248a2:	48 89 7d a8          	mov    %rdi,-0x58(%rbp)
   248a6:	4c 89 7d b0          	mov    %r15,-0x50(%rbp)
   248aa:	74 14                	je     248c0 <Data.String.string-length-from+0x120>
   248ac:	48 85 ff             	test   %rdi,%rdi
   248af:	75 2f                	jne    248e0 <Data.String.string-length-from+0x140>
   248b1:	4c 89 ef             	mov    %r13,%rdi
   248b4:	e8 d7 ab 04 00       	call   6f490 <cinch_nat_cmp>
   248b9:	e9 92 00 00 00       	jmp    24950 <Data.String.string-length-from+0x1b0>
   248be:	66 90                	xchg   %ax,%ax
   248c0:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   248c4:	48 89 c1             	mov    %rax,%rcx
   248c7:	48 c1 e1 1d          	shl    $0x1d,%rcx
   248cb:	48 85 c9             	test   %rcx,%rcx
   248ce:	7e 27                	jle    248f7 <Data.String.string-length-from+0x157>
   248d0:	48 ff c0             	inc    %rax
   248d3:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   248d7:	eb 2e                	jmp    24907 <Data.String.string-length-from+0x167>
   248d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   248e0:	4c 89 e9             	mov    %r13,%rcx
   248e3:	48 d1 e9             	shr    $1,%rcx
   248e6:	4c 21 e0             	and    %r12,%rax
   248e9:	48 39 c1             	cmp    %rax,%rcx
   248ec:	0f 83 db 11 00 00    	jae    25acd <Data.String.string-length-from+0x132d>
   248f2:	45 31 ff             	xor    %r15d,%r15d
   248f5:	eb 79                	jmp    24970 <Data.String.string-length-from+0x1d0>
   248f7:	74 0e                	je     24907 <Data.String.string-length-from+0x167>
   248f9:	4c 89 ef             	mov    %r13,%rdi
   248fc:	48 89 f3             	mov    %rsi,%rbx
   248ff:	e8 6c 59 04 00       	call   6a270 <cinch_dup_mt>
   24904:	48 89 de             	mov    %rbx,%rsi
   24907:	4c 89 ef             	mov    %r13,%rdi
   2490a:	e8 81 ab 04 00       	call   6f490 <cinch_nat_cmp>
   2490f:	49 8b 55 f8          	mov    -0x8(%r13),%rdx
   24913:	48 89 d1             	mov    %rdx,%rcx
   24916:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2491a:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   24921:	7c 0d                	jl     24930 <Data.String.string-length-from+0x190>
   24923:	48 ff ca             	dec    %rdx
   24926:	49 89 55 f8          	mov    %rdx,-0x8(%r13)
   2492a:	eb 24                	jmp    24950 <Data.String.string-length-from+0x1b0>
   2492c:	0f 1f 40 00          	nopl   0x0(%rax)
   24930:	48 85 c9             	test   %rcx,%rcx
   24933:	74 1b                	je     24950 <Data.String.string-length-from+0x1b0>
   24935:	48 89 c3             	mov    %rax,%rbx
   24938:	4c 89 ef             	mov    %r13,%rdi
   2493b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   24942:	75 19                	jne    2495d <Data.String.string-length-from+0x1bd>
   24944:	e8 47 56 04 00       	call   69f90 <cinch_drop_slow>
   24949:	eb 17                	jmp    24962 <Data.String.string-length-from+0x1c2>
   2494b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   24950:	41 b7 01             	mov    $0x1,%r15b
   24953:	48 85 c0             	test   %rax,%rax
   24956:	78 18                	js     24970 <Data.String.string-length-from+0x1d0>
   24958:	e9 70 11 00 00       	jmp    25acd <Data.String.string-length-from+0x132d>
   2495d:	e8 3e 55 04 00       	call   69ea0 <cinch_drop_mt>
   24962:	41 b7 01             	mov    $0x1,%r15b
   24965:	48 85 db             	test   %rbx,%rbx
   24968:	0f 89 5f 11 00 00    	jns    25acd <Data.String.string-length-from+0x132d>
   2496e:	66 90                	xchg   %ax,%ax
   24970:	4d 85 f6             	test   %r14,%r14
   24973:	74 1b                	je     24990 <Data.String.string-length-from+0x1f0>
   24975:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   24979:	48 89 c1             	mov    %rax,%rcx
   2497c:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24980:	48 85 c9             	test   %rcx,%rcx
   24983:	7e 2b                	jle    249b0 <Data.String.string-length-from+0x210>
   24985:	48 ff c0             	inc    %rax
   24988:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2498c:	eb 2c                	jmp    249ba <Data.String.string-length-from+0x21a>
   2498e:	66 90                	xchg   %ax,%ax
   24990:	31 ff                	xor    %edi,%edi
   24992:	e8 09 47 04 00       	call   690a0 <cinch_bytes_len>
   24997:	49 89 c6             	mov    %rax,%r14
   2499a:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   2499e:	0f 84 a2 00 00 00    	je     24a46 <Data.String.string-length-from+0x2a6>
   249a4:	e9 d7 00 00 00       	jmp    24a80 <Data.String.string-length-from+0x2e0>
   249a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   249b0:	74 08                	je     249ba <Data.String.string-length-from+0x21a>
   249b2:	4c 89 f7             	mov    %r14,%rdi
   249b5:	e8 b6 58 04 00       	call   6a270 <cinch_dup_mt>
   249ba:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   249be:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   249c5:	00 00 00 
   249c8:	48 21 c8             	and    %rcx,%rax
   249cb:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   249d2:	00 00 00 
   249d5:	48 39 c8             	cmp    %rcx,%rax
   249d8:	75 16                	jne    249f0 <Data.String.string-length-from+0x250>
   249da:	4c 89 f2             	mov    %r14,%rdx
   249dd:	4d 8b 36             	mov    (%r14),%r14
   249e0:	eb 1c                	jmp    249fe <Data.String.string-length-from+0x25e>
   249e2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   249e9:	1f 84 00 00 00 00 00 
   249f0:	4c 89 f7             	mov    %r14,%rdi
   249f3:	e8 a8 46 04 00       	call   690a0 <cinch_bytes_len>
   249f8:	4c 89 f2             	mov    %r14,%rdx
   249fb:	49 89 c6             	mov    %rax,%r14
   249fe:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   24a02:	48 89 c8             	mov    %rcx,%rax
   24a05:	48 c1 e0 1d          	shl    $0x1d,%rax
   24a09:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   24a0f:	7c 0f                	jl     24a20 <Data.String.string-length-from+0x280>
   24a11:	48 ff c9             	dec    %rcx
   24a14:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   24a18:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   24a1c:	74 28                	je     24a46 <Data.String.string-length-from+0x2a6>
   24a1e:	eb 60                	jmp    24a80 <Data.String.string-length-from+0x2e0>
   24a20:	48 85 c0             	test   %rax,%rax
   24a23:	74 1b                	je     24a40 <Data.String.string-length-from+0x2a0>
   24a25:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   24a2b:	75 3e                	jne    24a6b <Data.String.string-length-from+0x2cb>
   24a2d:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24a31:	e8 5a 55 04 00       	call   69f90 <cinch_drop_slow>
   24a36:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   24a3d:	00 00 00 
   24a40:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   24a44:	75 3a                	jne    24a80 <Data.String.string-length-from+0x2e0>
   24a46:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   24a4a:	48 89 c1             	mov    %rax,%rcx
   24a4d:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24a51:	48 85 c9             	test   %rcx,%rcx
   24a54:	7e 09                	jle    24a5f <Data.String.string-length-from+0x2bf>
   24a56:	48 ff c0             	inc    %rax
   24a59:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   24a5d:	eb 21                	jmp    24a80 <Data.String.string-length-from+0x2e0>
   24a5f:	74 1f                	je     24a80 <Data.String.string-length-from+0x2e0>
   24a61:	4c 89 ef             	mov    %r13,%rdi
   24a64:	e8 07 58 04 00       	call   6a270 <cinch_dup_mt>
   24a69:	eb 15                	jmp    24a80 <Data.String.string-length-from+0x2e0>
   24a6b:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24a6f:	e8 2c 54 04 00       	call   69ea0 <cinch_drop_mt>
   24a74:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   24a78:	74 cc                	je     24a46 <Data.String.string-length-from+0x2a6>
   24a7a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   24a80:	45 84 ff             	test   %r15b,%r15b
   24a83:	74 1b                	je     24aa0 <Data.String.string-length-from+0x300>
   24a85:	4a 8d 34 75 01 00 00 	lea    0x1(,%r14,2),%rsi
   24a8c:	00 
   24a8d:	4c 89 ef             	mov    %r13,%rdi
   24a90:	e8 fb a9 04 00       	call   6f490 <cinch_nat_cmp>
   24a95:	48 85 c0             	test   %rax,%rax
   24a98:	0f 99 c3             	setns  %bl
   24a9b:	eb 12                	jmp    24aaf <Data.String.string-length-from+0x30f>
   24a9d:	0f 1f 00             	nopl   (%rax)
   24aa0:	4c 89 e8             	mov    %r13,%rax
   24aa3:	48 d1 e8             	shr    $1,%rax
   24aa6:	4d 21 e6             	and    %r12,%r14
   24aa9:	4c 39 f0             	cmp    %r14,%rax
   24aac:	0f 93 c3             	setae  %bl
   24aaf:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   24ab3:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   24ab7:	75 47                	jne    24b00 <Data.String.string-length-from+0x360>
   24ab9:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   24abd:	48 89 c8             	mov    %rcx,%rax
   24ac0:	48 c1 e0 1d          	shl    $0x1d,%rax
   24ac4:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   24aca:	7c 14                	jl     24ae0 <Data.String.string-length-from+0x340>
   24acc:	48 ff c9             	dec    %rcx
   24acf:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   24ad3:	eb 2b                	jmp    24b00 <Data.String.string-length-from+0x360>
   24ad5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   24adc:	00 00 00 00 
   24ae0:	48 85 c0             	test   %rax,%rax
   24ae3:	74 1b                	je     24b00 <Data.String.string-length-from+0x360>
   24ae5:	4c 89 ef             	mov    %r13,%rdi
   24ae8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   24aee:	75 07                	jne    24af7 <Data.String.string-length-from+0x357>
   24af0:	e8 9b 54 04 00       	call   69f90 <cinch_drop_slow>
   24af5:	eb 09                	jmp    24b00 <Data.String.string-length-from+0x360>
   24af7:	e8 a4 53 04 00       	call   69ea0 <cinch_drop_mt>
   24afc:	0f 1f 40 00          	nopl   0x0(%rax)
   24b00:	4c 89 6d c0          	mov    %r13,-0x40(%rbp)
   24b04:	4c 8d 2d 15 9f 0f 00 	lea    0xf9f15(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24b0b:	84 db                	test   %bl,%bl
   24b0d:	0f 85 9d 06 00 00    	jne    251b0 <Data.String.string-length-from+0xa10>
   24b13:	45 84 ff             	test   %r15b,%r15b
   24b16:	74 18                	je     24b30 <Data.String.string-length-from+0x390>
   24b18:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   24b1c:	e8 8f b9 04 00       	call   704b0 <cinch_nat_to_i64>
   24b21:	48 89 c6             	mov    %rax,%rsi
   24b24:	eb 11                	jmp    24b37 <Data.String.string-length-from+0x397>
   24b26:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   24b2d:	00 00 00 
   24b30:	48 8b 75 c0          	mov    -0x40(%rbp),%rsi
   24b34:	48 d1 ee             	shr    $1,%rsi
   24b37:	48 bb ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rbx
   24b3e:	ff ff 7f 
   24b41:	4d 85 f6             	test   %r14,%r14
   24b44:	74 3a                	je     24b80 <Data.String.string-length-from+0x3e0>
   24b46:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   24b4a:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   24b51:	00 00 00 
   24b54:	48 21 c8             	and    %rcx,%rax
   24b57:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   24b5e:	00 00 00 
   24b61:	48 39 c8             	cmp    %rcx,%rax
   24b64:	75 1a                	jne    24b80 <Data.String.string-length-from+0x3e0>
   24b66:	49 3b 36             	cmp    (%r14),%rsi
   24b69:	73 15                	jae    24b80 <Data.String.string-length-from+0x3e0>
   24b6b:	49 8b 46 08          	mov    0x8(%r14),%rax
   24b6f:	44 0f b6 24 30       	movzbl (%rax,%rsi,1),%r12d
   24b74:	eb 15                	jmp    24b8b <Data.String.string-length-from+0x3eb>
   24b76:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   24b7d:	00 00 00 
   24b80:	4c 89 f7             	mov    %r14,%rdi
   24b83:	e8 c8 44 04 00       	call   69050 <cinch_bytes_get>
   24b88:	49 89 c4             	mov    %rax,%r12
   24b8b:	48 8d 43 81          	lea    -0x7f(%rbx),%rax
   24b8f:	49 85 c4             	test   %rax,%r12
   24b92:	0f 84 ea 00 00 00    	je     24c82 <Data.String.string-length-from+0x4e2>
   24b98:	48 8d 43 c1          	lea    -0x3f(%rbx),%rax
   24b9c:	4c 21 e0             	and    %r12,%rax
   24b9f:	4c 8d 2d 7a 9e 0f 00 	lea    0xf9e7a(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24ba6:	48 3d c0 00 00 00    	cmp    $0xc0,%rax
   24bac:	0f 82 fe 05 00 00    	jb     251b0 <Data.String.string-length-from+0xa10>
   24bb2:	48 b8 ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rax
   24bb9:	ff ff 7f 
   24bbc:	48 83 c0 e1          	add    $0xffffffffffffffe1,%rax
   24bc0:	4c 21 e0             	and    %r12,%rax
   24bc3:	be 05 00 00 00       	mov    $0x5,%esi
   24bc8:	48 3d e0 00 00 00    	cmp    $0xe0,%rax
   24bce:	72 47                	jb     24c17 <Data.String.string-length-from+0x477>
   24bd0:	48 b8 ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rax
   24bd7:	ff ff 7f 
   24bda:	48 83 c0 f1          	add    $0xfffffffffffffff1,%rax
   24bde:	4c 21 e0             	and    %r12,%rax
   24be1:	be 07 00 00 00       	mov    $0x7,%esi
   24be6:	48 3d f0 00 00 00    	cmp    $0xf0,%rax
   24bec:	72 29                	jb     24c17 <Data.String.string-length-from+0x477>
   24bee:	48 b8 ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rax
   24bf5:	ff ff 7f 
   24bf8:	48 83 c0 f9          	add    $0xfffffffffffffff9,%rax
   24bfc:	4c 21 e0             	and    %r12,%rax
   24bff:	be 09 00 00 00       	mov    $0x9,%esi
   24c04:	4c 8d 2d 15 9e 0f 00 	lea    0xf9e15(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24c0b:	48 3d f7 00 00 00    	cmp    $0xf7,%rax
   24c11:	0f 87 99 05 00 00    	ja     251b0 <Data.String.string-length-from+0xa10>
   24c17:	45 84 ff             	test   %r15b,%r15b
   24c1a:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   24c1e:	48 89 75 b8          	mov    %rsi,-0x48(%rbp)
   24c22:	0f 85 ba 00 00 00    	jne    24ce2 <Data.String.string-length-from+0x542>
   24c28:	89 f1                	mov    %esi,%ecx
   24c2a:	d1 e9                	shr    $1,%ecx
   24c2c:	48 89 f8             	mov    %rdi,%rax
   24c2f:	48 d1 e8             	shr    $1,%rax
   24c32:	48 01 c8             	add    %rcx,%rax
   24c35:	0f 88 a7 00 00 00    	js     24ce2 <Data.String.string-length-from+0x542>
   24c3b:	4c 8d 2c 45 01 00 00 	lea    0x1(,%rax,2),%r13
   24c42:	00 
   24c43:	4d 85 f6             	test   %r14,%r14
   24c46:	0f 85 a7 00 00 00    	jne    24cf3 <Data.String.string-length-from+0x553>
   24c4c:	31 ff                	xor    %edi,%edi
   24c4e:	e8 4d 44 04 00       	call   690a0 <cinch_bytes_len>
   24c53:	49 89 c6             	mov    %rax,%r14
   24c56:	41 f6 c5 01          	test   $0x1,%r13b
   24c5a:	0f 84 a4 01 00 00    	je     24e04 <Data.String.string-length-from+0x664>
   24c60:	49 d1 ed             	shr    $1,%r13
   24c63:	49 21 de             	and    %rbx,%r14
   24c66:	4c 89 e8             	mov    %r13,%rax
   24c69:	4c 8d 2d b0 9d 0f 00 	lea    0xf9db0(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24c70:	4c 39 f0             	cmp    %r14,%rax
   24c73:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   24c77:	0f 87 33 05 00 00    	ja     251b0 <Data.String.string-length-from+0xa10>
   24c7d:	e9 95 02 00 00       	jmp    24f17 <Data.String.string-length-from+0x777>
   24c82:	41 83 e4 7f          	and    $0x7f,%r12d
   24c86:	bf 08 00 00 00       	mov    $0x8,%edi
   24c8b:	be 01 00 00 00       	mov    $0x1,%esi
   24c90:	e8 cb 2c 04 00       	call   67960 <cinch_alloc_ctor>
   24c95:	4c 89 20             	mov    %r12,(%rax)
   24c98:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   24c9c:	48 c1 e9 28          	shr    $0x28,%rcx
   24ca0:	0f b6 c9             	movzbl %cl,%ecx
   24ca3:	48 81 f9 ff 00 00 00 	cmp    $0xff,%rcx
   24caa:	75 07                	jne    24cb3 <Data.String.string-length-from+0x513>
   24cac:	48 8b 48 f0          	mov    -0x10(%rax),%rcx
   24cb0:	48 8b 09             	mov    (%rcx),%rcx
   24cb3:	48 83 f9 01          	cmp    $0x1,%rcx
   24cb7:	74 53                	je     24d0c <Data.String.string-length-from+0x56c>
   24cb9:	48 85 c9             	test   %rcx,%rcx
   24cbc:	0f 85 f7 0d 00 00    	jne    25ab9 <Data.String.string-length-from+0x1319>
   24cc2:	48 8b 50 f8          	mov    -0x8(%rax),%rdx
   24cc6:	48 89 d1             	mov    %rdx,%rcx
   24cc9:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24ccd:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   24cd4:	7c 7b                	jl     24d51 <Data.String.string-length-from+0x5b1>
   24cd6:	48 ff ca             	dec    %rdx
   24cd9:	48 89 50 f8          	mov    %rdx,-0x8(%rax)
   24cdd:	e9 c7 04 00 00       	jmp    251a9 <Data.String.string-length-from+0xa09>
   24ce2:	e8 b9 a5 04 00       	call   6f2a0 <cinch_nat_add>
   24ce7:	49 89 c5             	mov    %rax,%r13
   24cea:	4d 85 f6             	test   %r14,%r14
   24ced:	0f 84 59 ff ff ff    	je     24c4c <Data.String.string-length-from+0x4ac>
   24cf3:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   24cf7:	48 89 c1             	mov    %rax,%rcx
   24cfa:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24cfe:	48 85 c9             	test   %rcx,%rcx
   24d01:	7e 78                	jle    24d7b <Data.String.string-length-from+0x5db>
   24d03:	48 ff c0             	inc    %rax
   24d06:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   24d0a:	eb 79                	jmp    24d85 <Data.String.string-length-from+0x5e5>
   24d0c:	48 8b 18             	mov    (%rax),%rbx
   24d0f:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   24d13:	48 ba ff ff ff ff 07 	movabs $0x7ffffffff,%rdx
   24d1a:	00 00 00 
   24d1d:	48 21 d1             	and    %rdx,%rcx
   24d20:	49 89 c5             	mov    %rax,%r13
   24d23:	48 83 f9 01          	cmp    $0x1,%rcx
   24d27:	0f 84 7f 01 00 00    	je     24eac <Data.String.string-length-from+0x70c>
   24d2d:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   24d31:	49 89 cd             	mov    %rcx,%r13
   24d34:	49 c1 e5 1d          	shl    $0x1d,%r13
   24d38:	49 81 fd 01 00 00 20 	cmp    $0x20000001,%r13
   24d3f:	0f 8c f5 00 00 00    	jl     24e3a <Data.String.string-length-from+0x69a>
   24d45:	48 ff c9             	dec    %rcx
   24d48:	48 89 48 f8          	mov    %rcx,-0x8(%rax)
   24d4c:	e9 58 01 00 00       	jmp    24ea9 <Data.String.string-length-from+0x709>
   24d51:	4c 8d 2d c8 9c 0f 00 	lea    0xf9cc8(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24d58:	48 85 c9             	test   %rcx,%rcx
   24d5b:	0f 84 4f 04 00 00    	je     251b0 <Data.String.string-length-from+0xa10>
   24d61:	48 89 c7             	mov    %rax,%rdi
   24d64:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   24d6b:	0f 85 33 04 00 00    	jne    251a4 <Data.String.string-length-from+0xa04>
   24d71:	e8 1a 52 04 00       	call   69f90 <cinch_drop_slow>
   24d76:	e9 2e 04 00 00       	jmp    251a9 <Data.String.string-length-from+0xa09>
   24d7b:	74 08                	je     24d85 <Data.String.string-length-from+0x5e5>
   24d7d:	4c 89 f7             	mov    %r14,%rdi
   24d80:	e8 eb 54 04 00       	call   6a270 <cinch_dup_mt>
   24d85:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   24d89:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   24d90:	00 00 00 
   24d93:	48 21 c8             	and    %rcx,%rax
   24d96:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   24d9d:	00 00 00 
   24da0:	48 39 c8             	cmp    %rcx,%rax
   24da3:	75 08                	jne    24dad <Data.String.string-length-from+0x60d>
   24da5:	4c 89 f2             	mov    %r14,%rdx
   24da8:	4d 8b 36             	mov    (%r14),%r14
   24dab:	eb 0e                	jmp    24dbb <Data.String.string-length-from+0x61b>
   24dad:	4c 89 f7             	mov    %r14,%rdi
   24db0:	e8 eb 42 04 00       	call   690a0 <cinch_bytes_len>
   24db5:	4c 89 f2             	mov    %r14,%rdx
   24db8:	49 89 c6             	mov    %rax,%r14
   24dbb:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   24dbf:	48 89 c8             	mov    %rcx,%rax
   24dc2:	48 c1 e0 1d          	shl    $0x1d,%rax
   24dc6:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   24dcc:	7c 12                	jl     24de0 <Data.String.string-length-from+0x640>
   24dce:	48 ff c9             	dec    %rcx
   24dd1:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   24dd5:	41 f6 c5 01          	test   $0x1,%r13b
   24dd9:	74 29                	je     24e04 <Data.String.string-length-from+0x664>
   24ddb:	e9 80 fe ff ff       	jmp    24c60 <Data.String.string-length-from+0x4c0>
   24de0:	48 85 c0             	test   %rax,%rax
   24de3:	74 15                	je     24dfa <Data.String.string-length-from+0x65a>
   24de5:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   24deb:	0f 85 9b 00 00 00    	jne    24e8c <Data.String.string-length-from+0x6ec>
   24df1:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24df5:	e8 96 51 04 00       	call   69f90 <cinch_drop_slow>
   24dfa:	41 f6 c5 01          	test   $0x1,%r13b
   24dfe:	0f 85 5c fe ff ff    	jne    24c60 <Data.String.string-length-from+0x4c0>
   24e04:	4a 8d 34 75 01 00 00 	lea    0x1(,%r14,2),%rsi
   24e0b:	00 
   24e0c:	4c 89 ef             	mov    %r13,%rdi
   24e0f:	e8 7c a6 04 00       	call   6f490 <cinch_nat_cmp>
   24e14:	4d 85 ed             	test   %r13,%r13
   24e17:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   24e1b:	74 5a                	je     24e77 <Data.String.string-length-from+0x6d7>
   24e1d:	49 8b 55 f8          	mov    -0x8(%r13),%rdx
   24e21:	48 89 d1             	mov    %rdx,%rcx
   24e24:	48 c1 e1 1d          	shl    $0x1d,%rcx
   24e28:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   24e2f:	7c 21                	jl     24e52 <Data.String.string-length-from+0x6b2>
   24e31:	48 ff ca             	dec    %rdx
   24e34:	49 89 55 f8          	mov    %rdx,-0x8(%r13)
   24e38:	eb 3d                	jmp    24e77 <Data.String.string-length-from+0x6d7>
   24e3a:	4d 85 ed             	test   %r13,%r13
   24e3d:	74 6d                	je     24eac <Data.String.string-length-from+0x70c>
   24e3f:	48 89 c7             	mov    %rax,%rdi
   24e42:	49 81 fd 00 00 00 20 	cmp    $0x20000000,%r13
   24e49:	75 59                	jne    24ea4 <Data.String.string-length-from+0x704>
   24e4b:	e8 40 51 04 00       	call   69f90 <cinch_drop_slow>
   24e50:	eb 57                	jmp    24ea9 <Data.String.string-length-from+0x709>
   24e52:	48 85 c9             	test   %rcx,%rcx
   24e55:	74 20                	je     24e77 <Data.String.string-length-from+0x6d7>
   24e57:	49 89 de             	mov    %rbx,%r14
   24e5a:	48 89 c3             	mov    %rax,%rbx
   24e5d:	4c 89 ef             	mov    %r13,%rdi
   24e60:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   24e67:	0f 85 92 00 00 00    	jne    24eff <Data.String.string-length-from+0x75f>
   24e6d:	e8 1e 51 04 00       	call   69f90 <cinch_drop_slow>
   24e72:	e9 8d 00 00 00       	jmp    24f04 <Data.String.string-length-from+0x764>
   24e77:	4c 8d 2d a2 9b 0f 00 	lea    0xf9ba2(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24e7e:	48 85 c0             	test   %rax,%rax
   24e81:	0f 8f 29 03 00 00    	jg     251b0 <Data.String.string-length-from+0xa10>
   24e87:	e9 8b 00 00 00       	jmp    24f17 <Data.String.string-length-from+0x777>
   24e8c:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24e90:	e8 0b 50 04 00       	call   69ea0 <cinch_drop_mt>
   24e95:	41 f6 c5 01          	test   $0x1,%r13b
   24e99:	0f 84 65 ff ff ff    	je     24e04 <Data.String.string-length-from+0x664>
   24e9f:	e9 bc fd ff ff       	jmp    24c60 <Data.String.string-length-from+0x4c0>
   24ea4:	e8 f7 4f 04 00       	call   69ea0 <cinch_drop_mt>
   24ea9:	45 31 ed             	xor    %r13d,%r13d
   24eac:	bf 10 00 00 00       	mov    $0x10,%edi
   24eb1:	be 00 02 00 00       	mov    $0x200,%esi
   24eb6:	e8 a5 2a 04 00       	call   67960 <cinch_alloc_ctor>
   24ebb:	49 89 c6             	mov    %rax,%r14
   24ebe:	48 89 18             	mov    %rbx,(%rax)
   24ec1:	48 c7 40 08 03 00 00 	movq   $0x3,0x8(%rax)
   24ec8:	00 
   24ec9:	4d 85 ed             	test   %r13,%r13
   24ecc:	74 17                	je     24ee5 <Data.String.string-length-from+0x745>
   24ece:	48 b8 01 00 00 00 08 	movabs $0x1010800000001,%rax
   24ed5:	01 01 00 
   24ed8:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   24edc:	4d 89 75 00          	mov    %r14,0x0(%r13)
   24ee0:	e9 cb 02 00 00       	jmp    251b0 <Data.String.string-length-from+0xa10>
   24ee5:	bf 08 00 00 00       	mov    $0x8,%edi
   24eea:	be 01 01 00 00       	mov    $0x101,%esi
   24eef:	e8 6c 2a 04 00       	call   67960 <cinch_alloc_ctor>
   24ef4:	49 89 c5             	mov    %rax,%r13
   24ef7:	4c 89 30             	mov    %r14,(%rax)
   24efa:	e9 b1 02 00 00       	jmp    251b0 <Data.String.string-length-from+0xa10>
   24eff:	e8 9c 4f 04 00       	call   69ea0 <cinch_drop_mt>
   24f04:	4c 8d 2d 15 9b 0f 00 	lea    0xf9b15(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24f0b:	48 85 db             	test   %rbx,%rbx
   24f0e:	4c 89 f3             	mov    %r14,%rbx
   24f11:	0f 8f 99 02 00 00    	jg     251b0 <Data.String.string-length-from+0xa10>
   24f17:	45 84 ff             	test   %r15b,%r15b
   24f1a:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   24f1e:	75 15                	jne    24f35 <Data.String.string-length-from+0x795>
   24f20:	48 89 f8             	mov    %rdi,%rax
   24f23:	48 d1 e8             	shr    $1,%rax
   24f26:	48 ff c0             	inc    %rax
   24f29:	78 0a                	js     24f35 <Data.String.string-length-from+0x795>
   24f2b:	4c 8d 34 45 01 00 00 	lea    0x1(,%rax,2),%r14
   24f32:	00 
   24f33:	eb 0d                	jmp    24f42 <Data.String.string-length-from+0x7a2>
   24f35:	be 03 00 00 00       	mov    $0x3,%esi
   24f3a:	e8 61 a3 04 00       	call   6f2a0 <cinch_nat_add>
   24f3f:	49 89 c6             	mov    %rax,%r14
   24f42:	4a 8d 3c 65 01 00 00 	lea    0x1(,%r12,2),%rdi
   24f49:	00 
   24f4a:	4c 8d 63 fe          	lea    -0x2(%rbx),%r12
   24f4e:	48 89 de             	mov    %rbx,%rsi
   24f51:	e8 8a cd ff ff       	call   21ce0 <Data.String.utf8-lead-payload>
   24f56:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24f5a:	4c 89 f6             	mov    %r14,%rsi
   24f5d:	4c 89 e2             	mov    %r12,%rdx
   24f60:	48 89 c1             	mov    %rax,%rcx
   24f63:	e8 e8 c7 ff ff       	call   21750 <Data.String.utf8-cont>
   24f68:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   24f6c:	48 c1 e9 28          	shr    $0x28,%rcx
   24f70:	0f b6 c9             	movzbl %cl,%ecx
   24f73:	48 81 f9 ff 00 00 00 	cmp    $0xff,%rcx
   24f7a:	75 07                	jne    24f83 <Data.String.string-length-from+0x7e3>
   24f7c:	48 8b 48 f0          	mov    -0x10(%rax),%rcx
   24f80:	48 8b 09             	mov    (%rcx),%rcx
   24f83:	48 83 f9 01          	cmp    $0x1,%rcx
   24f87:	74 1e                	je     24fa7 <Data.String.string-length-from+0x807>
   24f89:	48 85 c9             	test   %rcx,%rcx
   24f8c:	0f 85 27 0b 00 00    	jne    25ab9 <Data.String.string-length-from+0x1319>
   24f92:	4c 8d 2d 87 9a 0f 00 	lea    0xf9a87(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   24f99:	48 85 c0             	test   %rax,%rax
   24f9c:	0f 85 20 fd ff ff    	jne    24cc2 <Data.String.string-length-from+0x522>
   24fa2:	e9 09 02 00 00       	jmp    251b0 <Data.String.string-length-from+0xa10>
   24fa7:	4c 8b 20             	mov    (%rax),%r12
   24faa:	48 85 c0             	test   %rax,%rax
   24fad:	74 2b                	je     24fda <Data.String.string-length-from+0x83a>
   24faf:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   24fb3:	48 ba ff ff ff ff 07 	movabs $0x7ffffffff,%rdx
   24fba:	00 00 00 
   24fbd:	48 21 d1             	and    %rdx,%rcx
   24fc0:	48 83 f9 01          	cmp    $0x1,%rcx
   24fc4:	75 14                	jne    24fda <Data.String.string-length-from+0x83a>
   24fc6:	48 89 c7             	mov    %rax,%rdi
   24fc9:	e8 d2 65 04 00       	call   6b5a0 <cinch_free>
   24fce:	45 89 e7             	mov    %r12d,%r15d
   24fd1:	41 83 e7 01          	and    $0x1,%r15d
   24fd5:	e9 87 00 00 00       	jmp    25061 <Data.String.string-length-from+0x8c1>
   24fda:	4d 85 e4             	test   %r12,%r12
   24fdd:	0f 94 c1             	sete   %cl
   24fe0:	44 89 e2             	mov    %r12d,%edx
   24fe3:	08 ca                	or     %cl,%dl
   24fe5:	f6 c2 01             	test   $0x1,%dl
   24fe8:	75 31                	jne    2501b <Data.String.string-length-from+0x87b>
   24fea:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   24fef:	48 89 ca             	mov    %rcx,%rdx
   24ff2:	48 c1 e2 1d          	shl    $0x1d,%rdx
   24ff6:	48 85 d2             	test   %rdx,%rdx
   24ff9:	7e 0a                	jle    25005 <Data.String.string-length-from+0x865>
   24ffb:	48 ff c1             	inc    %rcx
   24ffe:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   25003:	eb 16                	jmp    2501b <Data.String.string-length-from+0x87b>
   25005:	74 14                	je     2501b <Data.String.string-length-from+0x87b>
   25007:	4c 89 e7             	mov    %r12,%rdi
   2500a:	49 89 de             	mov    %rbx,%r14
   2500d:	48 89 c3             	mov    %rax,%rbx
   25010:	e8 5b 52 04 00       	call   6a270 <cinch_dup_mt>
   25015:	48 89 d8             	mov    %rbx,%rax
   25018:	4c 89 f3             	mov    %r14,%rbx
   2501b:	45 89 e7             	mov    %r12d,%r15d
   2501e:	41 83 e7 01          	and    $0x1,%r15d
   25022:	48 85 c0             	test   %rax,%rax
   25025:	74 3a                	je     25061 <Data.String.string-length-from+0x8c1>
   25027:	48 8b 50 f8          	mov    -0x8(%rax),%rdx
   2502b:	48 89 d1             	mov    %rdx,%rcx
   2502e:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25032:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   25039:	7c 09                	jl     25044 <Data.String.string-length-from+0x8a4>
   2503b:	48 ff ca             	dec    %rdx
   2503e:	48 89 50 f8          	mov    %rdx,-0x8(%rax)
   25042:	eb 1d                	jmp    25061 <Data.String.string-length-from+0x8c1>
   25044:	48 85 c9             	test   %rcx,%rcx
   25047:	74 18                	je     25061 <Data.String.string-length-from+0x8c1>
   25049:	48 89 c7             	mov    %rax,%rdi
   2504c:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   25053:	75 07                	jne    2505c <Data.String.string-length-from+0x8bc>
   25055:	e8 36 4f 04 00       	call   69f90 <cinch_drop_slow>
   2505a:	eb 05                	jmp    25061 <Data.String.string-length-from+0x8c1>
   2505c:	e8 3f 4e 04 00       	call   69ea0 <cinch_drop_mt>
   25061:	48 89 df             	mov    %rbx,%rdi
   25064:	e8 37 c4 ff ff       	call   214a0 <Data.String.utf8-min>
   25069:	49 89 c6             	mov    %rax,%r14
   2506c:	4d 85 e4             	test   %r12,%r12
   2506f:	0f 94 c0             	sete   %al
   25072:	44 89 fb             	mov    %r15d,%ebx
   25075:	08 c3                	or     %al,%bl
   25077:	75 25                	jne    2509e <Data.String.string-length-from+0x8fe>
   25079:	49 8b 44 24 f8       	mov    -0x8(%r12),%rax
   2507e:	48 89 c1             	mov    %rax,%rcx
   25081:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25085:	48 85 c9             	test   %rcx,%rcx
   25088:	7e 0a                	jle    25094 <Data.String.string-length-from+0x8f4>
   2508a:	48 ff c0             	inc    %rax
   2508d:	49 89 44 24 f8       	mov    %rax,-0x8(%r12)
   25092:	eb 0a                	jmp    2509e <Data.String.string-length-from+0x8fe>
   25094:	74 08                	je     2509e <Data.String.string-length-from+0x8fe>
   25096:	4c 89 e7             	mov    %r12,%rdi
   25099:	e8 d2 51 04 00       	call   6a270 <cinch_dup_mt>
   2509e:	45 85 f7             	test   %r14d,%r15d
   250a1:	74 19                	je     250bc <Data.String.string-length-from+0x91c>
   250a3:	4c 89 e0             	mov    %r12,%rax
   250a6:	48 d1 e8             	shr    $1,%rax
   250a9:	4c 89 f1             	mov    %r14,%rcx
   250ac:	48 d1 e9             	shr    $1,%rcx
   250af:	48 39 c8             	cmp    %rcx,%rax
   250b2:	41 0f 92 c5          	setb   %r13b
   250b6:	84 db                	test   %bl,%bl
   250b8:	74 18                	je     250d2 <Data.String.string-length-from+0x932>
   250ba:	eb 50                	jmp    2510c <Data.String.string-length-from+0x96c>
   250bc:	4c 89 e7             	mov    %r12,%rdi
   250bf:	4c 89 f6             	mov    %r14,%rsi
   250c2:	e8 c9 a3 04 00       	call   6f490 <cinch_nat_cmp>
   250c7:	49 89 c5             	mov    %rax,%r13
   250ca:	49 c1 ed 3f          	shr    $0x3f,%r13
   250ce:	84 db                	test   %bl,%bl
   250d0:	75 3a                	jne    2510c <Data.String.string-length-from+0x96c>
   250d2:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   250d7:	48 89 c8             	mov    %rcx,%rax
   250da:	48 c1 e0 1d          	shl    $0x1d,%rax
   250de:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   250e4:	7c 0a                	jl     250f0 <Data.String.string-length-from+0x950>
   250e6:	48 ff c9             	dec    %rcx
   250e9:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   250ee:	eb 1c                	jmp    2510c <Data.String.string-length-from+0x96c>
   250f0:	48 85 c0             	test   %rax,%rax
   250f3:	74 17                	je     2510c <Data.String.string-length-from+0x96c>
   250f5:	4c 89 e7             	mov    %r12,%rdi
   250f8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   250fe:	75 07                	jne    25107 <Data.String.string-length-from+0x967>
   25100:	e8 8b 4e 04 00       	call   69f90 <cinch_drop_slow>
   25105:	eb 05                	jmp    2510c <Data.String.string-length-from+0x96c>
   25107:	e8 94 4d 04 00       	call   69ea0 <cinch_drop_mt>
   2510c:	41 f6 c6 01          	test   $0x1,%r14b
   25110:	75 38                	jne    2514a <Data.String.string-length-from+0x9aa>
   25112:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   25116:	48 89 c8             	mov    %rcx,%rax
   25119:	48 c1 e0 1d          	shl    $0x1d,%rax
   2511d:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25123:	7c 09                	jl     2512e <Data.String.string-length-from+0x98e>
   25125:	48 ff c9             	dec    %rcx
   25128:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2512c:	eb 1c                	jmp    2514a <Data.String.string-length-from+0x9aa>
   2512e:	48 85 c0             	test   %rax,%rax
   25131:	74 17                	je     2514a <Data.String.string-length-from+0x9aa>
   25133:	4c 89 f7             	mov    %r14,%rdi
   25136:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2513c:	75 07                	jne    25145 <Data.String.string-length-from+0x9a5>
   2513e:	e8 4d 4e 04 00       	call   69f90 <cinch_drop_slow>
   25143:	eb 05                	jmp    2514a <Data.String.string-length-from+0x9aa>
   25145:	e8 56 4d 04 00       	call   69ea0 <cinch_drop_mt>
   2514a:	45 84 ed             	test   %r13b,%r13b
   2514d:	74 29                	je     25178 <Data.String.string-length-from+0x9d8>
   2514f:	4c 8d 2d ca 98 0f 00 	lea    0xf98ca(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   25156:	84 db                	test   %bl,%bl
   25158:	75 56                	jne    251b0 <Data.String.string-length-from+0xa10>
   2515a:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2515f:	48 89 c8             	mov    %rcx,%rax
   25162:	48 c1 e0 1d          	shl    $0x1d,%rax
   25166:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2516c:	7c 1b                	jl     25189 <Data.String.string-length-from+0x9e9>
   2516e:	48 ff c9             	dec    %rcx
   25171:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   25176:	eb 31                	jmp    251a9 <Data.String.string-length-from+0xa09>
   25178:	4c 89 e7             	mov    %r12,%rdi
   2517b:	48 8b 75 b8          	mov    -0x48(%rbp),%rsi
   2517f:	e8 3c d5 ff ff       	call   226c0 <Data.String.utf8-char>
   25184:	49 89 c5             	mov    %rax,%r13
   25187:	eb 27                	jmp    251b0 <Data.String.string-length-from+0xa10>
   25189:	4c 8d 2d 90 98 0f 00 	lea    0xf9890(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   25190:	48 85 c0             	test   %rax,%rax
   25193:	74 1b                	je     251b0 <Data.String.string-length-from+0xa10>
   25195:	4c 89 e7             	mov    %r12,%rdi
   25198:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2519e:	0f 84 cd fb ff ff    	je     24d71 <Data.String.string-length-from+0x5d1>
   251a4:	e8 f7 4c 04 00       	call   69ea0 <cinch_drop_mt>
   251a9:	4c 8d 2d 70 98 0f 00 	lea    0xf9870(%rip),%r13        # 11ea20 <Effect.Process.ProcessInvalidArgument.blk#h>
   251b0:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   251b4:	48 c1 e8 28          	shr    $0x28,%rax
   251b8:	0f b6 c0             	movzbl %al,%eax
   251bb:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   251c1:	75 07                	jne    251ca <Data.String.string-length-from+0xa2a>
   251c3:	49 8b 45 f0          	mov    -0x10(%r13),%rax
   251c7:	48 8b 00             	mov    (%rax),%rax
   251ca:	48 83 f8 01          	cmp    $0x1,%rax
   251ce:	48 8b 5d b0          	mov    -0x50(%rbp),%rbx
   251d2:	0f 85 76 08 00 00    	jne    25a4e <Data.String.string-length-from+0x12ae>
   251d8:	4d 8b 75 00          	mov    0x0(%r13),%r14
   251dc:	4d 85 ed             	test   %r13,%r13
   251df:	74 2f                	je     25210 <Data.String.string-length-from+0xa70>
   251e1:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   251e5:	48 b9 ff ff ff ff 07 	movabs $0x7ffffffff,%rcx
   251ec:	00 00 00 
   251ef:	48 21 c8             	and    %rcx,%rax
   251f2:	48 83 f8 01          	cmp    $0x1,%rax
   251f6:	75 18                	jne    25210 <Data.String.string-length-from+0xa70>
   251f8:	4c 89 ef             	mov    %r13,%rdi
   251fb:	e8 a0 63 04 00       	call   6b5a0 <cinch_free>
   25200:	eb 7e                	jmp    25280 <Data.String.string-length-from+0xae0>
   25202:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25209:	1f 84 00 00 00 00 00 
   25210:	4d 85 f6             	test   %r14,%r14
   25213:	74 2b                	je     25240 <Data.String.string-length-from+0xaa0>
   25215:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   25219:	48 89 c1             	mov    %rax,%rcx
   2521c:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25220:	48 85 c9             	test   %rcx,%rcx
   25223:	7e 09                	jle    2522e <Data.String.string-length-from+0xa8e>
   25225:	48 ff c0             	inc    %rax
   25228:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2522c:	eb 12                	jmp    25240 <Data.String.string-length-from+0xaa0>
   2522e:	74 10                	je     25240 <Data.String.string-length-from+0xaa0>
   25230:	4c 89 f7             	mov    %r14,%rdi
   25233:	e8 38 50 04 00       	call   6a270 <cinch_dup_mt>
   25238:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   2523f:	00 
   25240:	4d 85 ed             	test   %r13,%r13
   25243:	74 3b                	je     25280 <Data.String.string-length-from+0xae0>
   25245:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   25249:	48 89 c8             	mov    %rcx,%rax
   2524c:	48 c1 e0 1d          	shl    $0x1d,%rax
   25250:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25256:	7c 09                	jl     25261 <Data.String.string-length-from+0xac1>
   25258:	48 ff c9             	dec    %rcx
   2525b:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2525f:	eb 1f                	jmp    25280 <Data.String.string-length-from+0xae0>
   25261:	48 85 c0             	test   %rax,%rax
   25264:	74 1a                	je     25280 <Data.String.string-length-from+0xae0>
   25266:	4c 89 ef             	mov    %r13,%rdi
   25269:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2526f:	75 07                	jne    25278 <Data.String.string-length-from+0xad8>
   25271:	e8 1a 4d 04 00       	call   69f90 <cinch_drop_slow>
   25276:	eb 08                	jmp    25280 <Data.String.string-length-from+0xae0>
   25278:	e8 23 4c 04 00       	call   69ea0 <cinch_drop_mt>
   2527d:	0f 1f 00             	nopl   (%rax)
   25280:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   25284:	48 c1 e8 28          	shr    $0x28,%rax
   25288:	0f b6 c0             	movzbl %al,%eax
   2528b:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   25291:	75 07                	jne    2529a <Data.String.string-length-from+0xafa>
   25293:	49 8b 46 f0          	mov    -0x10(%r14),%rax
   25297:	48 8b 00             	mov    (%rax),%rax
   2529a:	48 85 c0             	test   %rax,%rax
   2529d:	4c 8b 7d c0          	mov    -0x40(%rbp),%r15
   252a1:	0f 85 12 08 00 00    	jne    25ab9 <Data.String.string-length-from+0x1319>
   252a7:	4d 8b 66 08          	mov    0x8(%r14),%r12
   252ab:	4d 85 f6             	test   %r14,%r14
   252ae:	74 30                	je     252e0 <Data.String.string-length-from+0xb40>
   252b0:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   252b4:	48 b9 ff ff ff ff 07 	movabs $0x7ffffffff,%rcx
   252bb:	00 00 00 
   252be:	48 21 c8             	and    %rcx,%rax
   252c1:	48 83 f8 01          	cmp    $0x1,%rax
   252c5:	75 19                	jne    252e0 <Data.String.string-length-from+0xb40>
   252c7:	4c 89 f7             	mov    %r14,%rdi
   252ca:	e8 d1 62 04 00       	call   6b5a0 <cinch_free>
   252cf:	44 85 65 a8          	test   %r12d,-0x58(%rbp)
   252d3:	0f 85 8d 00 00 00    	jne    25366 <Data.String.string-length-from+0xbc6>
   252d9:	e9 b2 00 00 00       	jmp    25390 <Data.String.string-length-from+0xbf0>
   252de:	66 90                	xchg   %ax,%ax
   252e0:	4d 85 e4             	test   %r12,%r12
   252e3:	0f 94 c0             	sete   %al
   252e6:	44 89 e1             	mov    %r12d,%ecx
   252e9:	08 c1                	or     %al,%cl
   252eb:	f6 c1 01             	test   $0x1,%cl
   252ee:	75 30                	jne    25320 <Data.String.string-length-from+0xb80>
   252f0:	49 8b 44 24 f8       	mov    -0x8(%r12),%rax
   252f5:	48 89 c1             	mov    %rax,%rcx
   252f8:	48 c1 e1 1d          	shl    $0x1d,%rcx
   252fc:	48 85 c9             	test   %rcx,%rcx
   252ff:	7e 0a                	jle    2530b <Data.String.string-length-from+0xb6b>
   25301:	48 ff c0             	inc    %rax
   25304:	49 89 44 24 f8       	mov    %rax,-0x8(%r12)
   25309:	eb 15                	jmp    25320 <Data.String.string-length-from+0xb80>
   2530b:	74 13                	je     25320 <Data.String.string-length-from+0xb80>
   2530d:	4c 89 e7             	mov    %r12,%rdi
   25310:	e8 5b 4f 04 00       	call   6a270 <cinch_dup_mt>
   25315:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2531c:	00 00 00 00 
   25320:	4d 85 f6             	test   %r14,%r14
   25323:	74 3b                	je     25360 <Data.String.string-length-from+0xbc0>
   25325:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   25329:	48 89 c8             	mov    %rcx,%rax
   2532c:	48 c1 e0 1d          	shl    $0x1d,%rax
   25330:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25336:	7c 0f                	jl     25347 <Data.String.string-length-from+0xba7>
   25338:	48 ff c9             	dec    %rcx
   2533b:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2533f:	44 85 65 a8          	test   %r12d,-0x58(%rbp)
   25343:	75 21                	jne    25366 <Data.String.string-length-from+0xbc6>
   25345:	eb 49                	jmp    25390 <Data.String.string-length-from+0xbf0>
   25347:	48 85 c0             	test   %rax,%rax
   2534a:	74 14                	je     25360 <Data.String.string-length-from+0xbc0>
   2534c:	4c 89 f7             	mov    %r14,%rdi
   2534f:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25355:	75 2a                	jne    25381 <Data.String.string-length-from+0xbe1>
   25357:	e8 34 4c 04 00       	call   69f90 <cinch_drop_slow>
   2535c:	0f 1f 40 00          	nopl   0x0(%rax)
   25360:	44 85 65 a8          	test   %r12d,-0x58(%rbp)
   25364:	74 2a                	je     25390 <Data.String.string-length-from+0xbf0>
   25366:	4c 89 f9             	mov    %r15,%rcx
   25369:	48 d1 e9             	shr    $1,%rcx
   2536c:	4c 89 e0             	mov    %r12,%rax
   2536f:	48 d1 e8             	shr    $1,%rax
   25372:	48 01 c8             	add    %rcx,%rax
   25375:	78 19                	js     25390 <Data.String.string-length-from+0xbf0>
   25377:	4c 8d 2c 45 01 00 00 	lea    0x1(,%rax,2),%r13
   2537e:	00 
   2537f:	eb 1d                	jmp    2539e <Data.String.string-length-from+0xbfe>
   25381:	e8 1a 4b 04 00       	call   69ea0 <cinch_drop_mt>
   25386:	44 85 65 a8          	test   %r12d,-0x58(%rbp)
   2538a:	75 da                	jne    25366 <Data.String.string-length-from+0xbc6>
   2538c:	0f 1f 40 00          	nopl   0x0(%rax)
   25390:	4c 89 ff             	mov    %r15,%rdi
   25393:	4c 89 e6             	mov    %r12,%rsi
   25396:	e8 05 9f 04 00       	call   6f2a0 <cinch_nat_add>
   2539b:	49 89 c5             	mov    %rax,%r13
   2539e:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   253a2:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   253a6:	75 58                	jne    25400 <Data.String.string-length-from+0xc60>
   253a8:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   253ac:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   253b0:	48 89 c8             	mov    %rcx,%rax
   253b3:	48 c1 e0 1d          	shl    $0x1d,%rax
   253b7:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   253bd:	7c 11                	jl     253d0 <Data.String.string-length-from+0xc30>
   253bf:	48 ff c9             	dec    %rcx
   253c2:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   253c6:	eb 38                	jmp    25400 <Data.String.string-length-from+0xc60>
   253c8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   253cf:	00 
   253d0:	48 85 c0             	test   %rax,%rax
   253d3:	74 2b                	je     25400 <Data.String.string-length-from+0xc60>
   253d5:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   253db:	75 0b                	jne    253e8 <Data.String.string-length-from+0xc48>
   253dd:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   253e1:	e8 aa 4b 04 00       	call   69f90 <cinch_drop_slow>
   253e6:	eb 18                	jmp    25400 <Data.String.string-length-from+0xc60>
   253e8:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   253ec:	e8 af 4a 04 00       	call   69ea0 <cinch_drop_mt>
   253f1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   253f8:	0f 1f 84 00 00 00 00 
   253ff:	00 
   25400:	4d 85 e4             	test   %r12,%r12
   25403:	0f 94 c0             	sete   %al
   25406:	44 89 e1             	mov    %r12d,%ecx
   25409:	08 c1                	or     %al,%cl
   2540b:	f6 c1 01             	test   $0x1,%cl
   2540e:	74 20                	je     25430 <Data.String.string-length-from+0xc90>
   25410:	f6 c3 01             	test   $0x1,%bl
   25413:	74 7b                	je     25490 <Data.String.string-length-from+0xcf0>
   25415:	48 89 d8             	mov    %rbx,%rax
   25418:	48 d1 e8             	shr    $1,%rax
   2541b:	48 ff c0             	inc    %rax
   2541e:	78 70                	js     25490 <Data.String.string-length-from+0xcf0>
   25420:	4c 8d 3c 45 01 00 00 	lea    0x1(,%rax,2),%r15
   25427:	00 
   25428:	e9 c3 00 00 00       	jmp    254f0 <Data.String.string-length-from+0xd50>
   2542d:	0f 1f 00             	nopl   (%rax)
   25430:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   25435:	48 89 c8             	mov    %rcx,%rax
   25438:	48 c1 e0 1d          	shl    $0x1d,%rax
   2543c:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25442:	7c 1c                	jl     25460 <Data.String.string-length-from+0xcc0>
   25444:	48 ff c9             	dec    %rcx
   25447:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2544c:	f6 c3 01             	test   $0x1,%bl
   2544f:	75 c4                	jne    25415 <Data.String.string-length-from+0xc75>
   25451:	eb 3d                	jmp    25490 <Data.String.string-length-from+0xcf0>
   25453:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2545a:	84 00 00 00 00 00 
   25460:	48 85 c0             	test   %rax,%rax
   25463:	74 ab                	je     25410 <Data.String.string-length-from+0xc70>
   25465:	4c 89 e7             	mov    %r12,%rdi
   25468:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2546e:	75 07                	jne    25477 <Data.String.string-length-from+0xcd7>
   25470:	e8 1b 4b 04 00       	call   69f90 <cinch_drop_slow>
   25475:	eb 99                	jmp    25410 <Data.String.string-length-from+0xc70>
   25477:	e8 24 4a 04 00       	call   69ea0 <cinch_drop_mt>
   2547c:	f6 c3 01             	test   $0x1,%bl
   2547f:	75 94                	jne    25415 <Data.String.string-length-from+0xc75>
   25481:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25488:	0f 1f 84 00 00 00 00 
   2548f:	00 
   25490:	be 03 00 00 00       	mov    $0x3,%esi
   25495:	48 89 df             	mov    %rbx,%rdi
   25498:	e8 03 9e 04 00       	call   6f2a0 <cinch_nat_add>
   2549d:	49 89 c7             	mov    %rax,%r15
   254a0:	48 85 db             	test   %rbx,%rbx
   254a3:	0f 94 c0             	sete   %al
   254a6:	89 d9                	mov    %ebx,%ecx
   254a8:	08 c1                	or     %al,%cl
   254aa:	f6 c1 01             	test   $0x1,%cl
   254ad:	75 41                	jne    254f0 <Data.String.string-length-from+0xd50>
   254af:	48 8b 4b f8          	mov    -0x8(%rbx),%rcx
   254b3:	48 89 c8             	mov    %rcx,%rax
   254b6:	48 c1 e0 1d          	shl    $0x1d,%rax
   254ba:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   254c0:	7c 09                	jl     254cb <Data.String.string-length-from+0xd2b>
   254c2:	48 ff c9             	dec    %rcx
   254c5:	48 89 4b f8          	mov    %rcx,-0x8(%rbx)
   254c9:	eb 25                	jmp    254f0 <Data.String.string-length-from+0xd50>
   254cb:	48 85 c0             	test   %rax,%rax
   254ce:	74 20                	je     254f0 <Data.String.string-length-from+0xd50>
   254d0:	48 89 df             	mov    %rbx,%rdi
   254d3:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   254d9:	75 07                	jne    254e2 <Data.String.string-length-from+0xd42>
   254db:	e8 b0 4a 04 00       	call   69f90 <cinch_drop_slow>
   254e0:	eb 0e                	jmp    254f0 <Data.String.string-length-from+0xd50>
   254e2:	e8 b9 49 04 00       	call   69ea0 <cinch_drop_mt>
   254e7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   254ee:	00 00 
   254f0:	4c 89 7d c8          	mov    %r15,-0x38(%rbp)
   254f4:	4d 85 f6             	test   %r14,%r14
   254f7:	74 27                	je     25520 <Data.String.string-length-from+0xd80>
   254f9:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   254fd:	48 89 c1             	mov    %rax,%rcx
   25500:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25504:	48 85 c9             	test   %rcx,%rcx
   25507:	7e 27                	jle    25530 <Data.String.string-length-from+0xd90>
   25509:	48 ff c0             	inc    %rax
   2550c:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   25510:	eb 28                	jmp    2553a <Data.String.string-length-from+0xd9a>
   25512:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25519:	1f 84 00 00 00 00 00 
   25520:	31 ff                	xor    %edi,%edi
   25522:	e8 79 3b 04 00       	call   690a0 <cinch_bytes_len>
   25527:	e9 87 00 00 00       	jmp    255b3 <Data.String.string-length-from+0xe13>
   2552c:	0f 1f 40 00          	nopl   0x0(%rax)
   25530:	74 08                	je     2553a <Data.String.string-length-from+0xd9a>
   25532:	4c 89 f7             	mov    %r14,%rdi
   25535:	e8 36 4d 04 00       	call   6a270 <cinch_dup_mt>
   2553a:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2553e:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   25545:	00 00 00 
   25548:	48 21 c8             	and    %rcx,%rax
   2554b:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   25552:	00 00 00 
   25555:	48 39 c8             	cmp    %rcx,%rax
   25558:	75 06                	jne    25560 <Data.String.string-length-from+0xdc0>
   2555a:	49 8b 06             	mov    (%r14),%rax
   2555d:	eb 09                	jmp    25568 <Data.String.string-length-from+0xdc8>
   2555f:	90                   	nop
   25560:	4c 89 f7             	mov    %r14,%rdi
   25563:	e8 38 3b 04 00       	call   690a0 <cinch_bytes_len>
   25568:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   2556c:	48 89 d1             	mov    %rdx,%rcx
   2556f:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25573:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2557a:	7c 14                	jl     25590 <Data.String.string-length-from+0xdf0>
   2557c:	48 ff ca             	dec    %rdx
   2557f:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   25583:	eb 2e                	jmp    255b3 <Data.String.string-length-from+0xe13>
   25585:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2558c:	00 00 00 00 
   25590:	48 85 c9             	test   %rcx,%rcx
   25593:	74 1e                	je     255b3 <Data.String.string-length-from+0xe13>
   25595:	48 89 c3             	mov    %rax,%rbx
   25598:	4c 89 f7             	mov    %r14,%rdi
   2559b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   255a2:	75 07                	jne    255ab <Data.String.string-length-from+0xe0b>
   255a4:	e8 e7 49 04 00       	call   69f90 <cinch_drop_slow>
   255a9:	eb 05                	jmp    255b0 <Data.String.string-length-from+0xe10>
   255ab:	e8 f0 48 04 00       	call   69ea0 <cinch_drop_mt>
   255b0:	48 89 d8             	mov    %rbx,%rax
   255b3:	48 8d 34 45 01 00 00 	lea    0x1(,%rax,2),%rsi
   255ba:	00 
   255bb:	4d 85 ed             	test   %r13,%r13
   255be:	0f 94 c1             	sete   %cl
   255c1:	44 89 eb             	mov    %r13d,%ebx
   255c4:	83 e3 01             	and    $0x1,%ebx
   255c7:	45 89 ec             	mov    %r13d,%r12d
   255ca:	41 08 cc             	or     %cl,%r12b
   255cd:	41 f6 c4 01          	test   $0x1,%r12b
   255d1:	74 1d                	je     255f0 <Data.String.string-length-from+0xe50>
   255d3:	48 85 db             	test   %rbx,%rbx
   255d6:	75 38                	jne    25610 <Data.String.string-length-from+0xe70>
   255d8:	4c 89 ef             	mov    %r13,%rdi
   255db:	e8 b0 9e 04 00       	call   6f490 <cinch_nat_cmp>
   255e0:	e9 ab 00 00 00       	jmp    25690 <Data.String.string-length-from+0xef0>
   255e5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   255ec:	00 00 00 00 
   255f0:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   255f4:	48 89 c1             	mov    %rax,%rcx
   255f7:	48 c1 e1 1d          	shl    $0x1d,%rcx
   255fb:	48 85 c9             	test   %rcx,%rcx
   255fe:	7e 31                	jle    25631 <Data.String.string-length-from+0xe91>
   25600:	48 ff c0             	inc    %rax
   25603:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   25607:	eb 3c                	jmp    25645 <Data.String.string-length-from+0xea5>
   25609:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25610:	4c 89 e9             	mov    %r13,%rcx
   25613:	48 d1 e9             	shr    $1,%rcx
   25616:	48 ba ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rdx
   2561d:	ff ff 7f 
   25620:	48 21 d0             	and    %rdx,%rax
   25623:	48 39 c1             	cmp    %rax,%rcx
   25626:	0f 82 84 00 00 00    	jb     256b0 <Data.String.string-length-from+0xf10>
   2562c:	e9 c8 04 00 00       	jmp    25af9 <Data.String.string-length-from+0x1359>
   25631:	74 12                	je     25645 <Data.String.string-length-from+0xea5>
   25633:	4c 89 ef             	mov    %r13,%rdi
   25636:	49 89 f6             	mov    %rsi,%r14
   25639:	e8 32 4c 04 00       	call   6a270 <cinch_dup_mt>
   2563e:	4c 89 f6             	mov    %r14,%rsi
   25641:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   25645:	4c 89 ef             	mov    %r13,%rdi
   25648:	e8 43 9e 04 00       	call   6f490 <cinch_nat_cmp>
   2564d:	49 8b 55 f8          	mov    -0x8(%r13),%rdx
   25651:	48 89 d1             	mov    %rdx,%rcx
   25654:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25658:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2565f:	7c 0f                	jl     25670 <Data.String.string-length-from+0xed0>
   25661:	48 ff ca             	dec    %rdx
   25664:	49 89 55 f8          	mov    %rdx,-0x8(%r13)
   25668:	eb 26                	jmp    25690 <Data.String.string-length-from+0xef0>
   2566a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   25670:	48 85 c9             	test   %rcx,%rcx
   25673:	74 1b                	je     25690 <Data.String.string-length-from+0xef0>
   25675:	49 89 c6             	mov    %rax,%r14
   25678:	4c 89 ef             	mov    %r13,%rdi
   2567b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   25682:	75 16                	jne    2569a <Data.String.string-length-from+0xefa>
   25684:	e8 07 49 04 00       	call   69f90 <cinch_drop_slow>
   25689:	eb 14                	jmp    2569f <Data.String.string-length-from+0xeff>
   2568b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   25690:	48 85 c0             	test   %rax,%rax
   25693:	78 1b                	js     256b0 <Data.String.string-length-from+0xf10>
   25695:	e9 5f 04 00 00       	jmp    25af9 <Data.String.string-length-from+0x1359>
   2569a:	e8 01 48 04 00       	call   69ea0 <cinch_drop_mt>
   2569f:	4d 85 f6             	test   %r14,%r14
   256a2:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   256a6:	0f 89 4d 04 00 00    	jns    25af9 <Data.String.string-length-from+0x1359>
   256ac:	0f 1f 40 00          	nopl   0x0(%rax)
   256b0:	4d 85 f6             	test   %r14,%r14
   256b3:	74 2b                	je     256e0 <Data.String.string-length-from+0xf40>
   256b5:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   256b9:	48 89 c1             	mov    %rax,%rcx
   256bc:	48 c1 e1 1d          	shl    $0x1d,%rcx
   256c0:	48 85 c9             	test   %rcx,%rcx
   256c3:	7e 09                	jle    256ce <Data.String.string-length-from+0xf2e>
   256c5:	48 ff c0             	inc    %rax
   256c8:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   256cc:	eb 12                	jmp    256e0 <Data.String.string-length-from+0xf40>
   256ce:	74 10                	je     256e0 <Data.String.string-length-from+0xf40>
   256d0:	4c 89 f7             	mov    %r14,%rdi
   256d3:	e8 98 4b 04 00       	call   6a270 <cinch_dup_mt>
   256d8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   256df:	00 
   256e0:	41 f6 c4 01          	test   $0x1,%r12b
   256e4:	75 2a                	jne    25710 <Data.String.string-length-from+0xf70>
   256e6:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   256ea:	48 89 c1             	mov    %rax,%rcx
   256ed:	48 c1 e1 1d          	shl    $0x1d,%rcx
   256f1:	48 85 c9             	test   %rcx,%rcx
   256f4:	7e 09                	jle    256ff <Data.String.string-length-from+0xf5f>
   256f6:	48 ff c0             	inc    %rax
   256f9:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   256fd:	eb 11                	jmp    25710 <Data.String.string-length-from+0xf70>
   256ff:	74 0f                	je     25710 <Data.String.string-length-from+0xf70>
   25701:	4c 89 ef             	mov    %r13,%rdi
   25704:	e8 67 4b 04 00       	call   6a270 <cinch_dup_mt>
   25709:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25710:	4c 89 f7             	mov    %r14,%rdi
   25713:	4c 89 ee             	mov    %r13,%rsi
   25716:	e8 85 c7 ff ff       	call   21ea0 <Data.String.utf8-at>
   2571b:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   2571f:	48 c1 e9 28          	shr    $0x28,%rcx
   25723:	0f b6 c9             	movzbl %cl,%ecx
   25726:	48 81 f9 ff 00 00 00 	cmp    $0xff,%rcx
   2572d:	75 07                	jne    25736 <Data.String.string-length-from+0xf96>
   2572f:	48 8b 48 f0          	mov    -0x10(%rax),%rcx
   25733:	48 8b 09             	mov    (%rcx),%rcx
   25736:	48 83 f9 01          	cmp    $0x1,%rcx
   2573a:	0f 85 43 03 00 00    	jne    25a83 <Data.String.string-length-from+0x12e3>
   25740:	4c 8b 38             	mov    (%rax),%r15
   25743:	48 85 c0             	test   %rax,%rax
   25746:	74 28                	je     25770 <Data.String.string-length-from+0xfd0>
   25748:	48 8b 48 f8          	mov    -0x8(%rax),%rcx
   2574c:	48 ba ff ff ff ff 07 	movabs $0x7ffffffff,%rdx
   25753:	00 00 00 
   25756:	48 21 d1             	and    %rdx,%rcx
   25759:	48 83 f9 01          	cmp    $0x1,%rcx
   2575d:	75 11                	jne    25770 <Data.String.string-length-from+0xfd0>
   2575f:	48 89 c7             	mov    %rax,%rdi
   25762:	e8 39 5e 04 00       	call   6b5a0 <cinch_free>
   25767:	eb 77                	jmp    257e0 <Data.String.string-length-from+0x1040>
   25769:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25770:	4d 85 ff             	test   %r15,%r15
   25773:	74 2b                	je     257a0 <Data.String.string-length-from+0x1000>
   25775:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   25779:	48 89 ca             	mov    %rcx,%rdx
   2577c:	48 c1 e2 1d          	shl    $0x1d,%rdx
   25780:	48 85 d2             	test   %rdx,%rdx
   25783:	7e 09                	jle    2578e <Data.String.string-length-from+0xfee>
   25785:	48 ff c1             	inc    %rcx
   25788:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2578c:	eb 12                	jmp    257a0 <Data.String.string-length-from+0x1000>
   2578e:	74 10                	je     257a0 <Data.String.string-length-from+0x1000>
   25790:	4c 89 ff             	mov    %r15,%rdi
   25793:	49 89 c6             	mov    %rax,%r14
   25796:	e8 d5 4a 04 00       	call   6a270 <cinch_dup_mt>
   2579b:	4c 89 f0             	mov    %r14,%rax
   2579e:	66 90                	xchg   %ax,%ax
   257a0:	48 85 c0             	test   %rax,%rax
   257a3:	74 3b                	je     257e0 <Data.String.string-length-from+0x1040>
   257a5:	48 8b 50 f8          	mov    -0x8(%rax),%rdx
   257a9:	48 89 d1             	mov    %rdx,%rcx
   257ac:	48 c1 e1 1d          	shl    $0x1d,%rcx
   257b0:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   257b7:	7c 09                	jl     257c2 <Data.String.string-length-from+0x1022>
   257b9:	48 ff ca             	dec    %rdx
   257bc:	48 89 50 f8          	mov    %rdx,-0x8(%rax)
   257c0:	eb 1e                	jmp    257e0 <Data.String.string-length-from+0x1040>
   257c2:	48 85 c9             	test   %rcx,%rcx
   257c5:	74 19                	je     257e0 <Data.String.string-length-from+0x1040>
   257c7:	48 89 c7             	mov    %rax,%rdi
   257ca:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   257d1:	75 07                	jne    257da <Data.String.string-length-from+0x103a>
   257d3:	e8 b8 47 04 00       	call   69f90 <cinch_drop_slow>
   257d8:	eb 06                	jmp    257e0 <Data.String.string-length-from+0x1040>
   257da:	e8 c1 46 04 00       	call   69ea0 <cinch_drop_mt>
   257df:	90                   	nop
   257e0:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   257e4:	48 c1 e8 28          	shr    $0x28,%rax
   257e8:	0f b6 c0             	movzbl %al,%eax
   257eb:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   257f1:	75 07                	jne    257fa <Data.String.string-length-from+0x105a>
   257f3:	49 8b 47 f0          	mov    -0x10(%r15),%rax
   257f7:	48 8b 00             	mov    (%rax),%rax
   257fa:	48 85 c0             	test   %rax,%rax
   257fd:	0f 85 b6 02 00 00    	jne    25ab9 <Data.String.string-length-from+0x1319>
   25803:	4d 8b 77 08          	mov    0x8(%r15),%r14
   25807:	4d 85 ff             	test   %r15,%r15
   2580a:	74 34                	je     25840 <Data.String.string-length-from+0x10a0>
   2580c:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   25810:	48 b9 ff ff ff ff 07 	movabs $0x7ffffffff,%rcx
   25817:	00 00 00 
   2581a:	48 21 c8             	and    %rcx,%rax
   2581d:	48 83 f8 01          	cmp    $0x1,%rax
   25821:	75 1d                	jne    25840 <Data.String.string-length-from+0x10a0>
   25823:	4c 89 ff             	mov    %r15,%rdi
   25826:	e8 75 5d 04 00       	call   6b5a0 <cinch_free>
   2582b:	44 85 f3             	test   %r14d,%ebx
   2582e:	0f 85 91 00 00 00    	jne    258c5 <Data.String.string-length-from+0x1125>
   25834:	e9 b7 00 00 00       	jmp    258f0 <Data.String.string-length-from+0x1150>
   25839:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   25840:	4d 85 f6             	test   %r14,%r14
   25843:	0f 94 c0             	sete   %al
   25846:	44 89 f1             	mov    %r14d,%ecx
   25849:	08 c1                	or     %al,%cl
   2584b:	f6 c1 01             	test   $0x1,%cl
   2584e:	75 30                	jne    25880 <Data.String.string-length-from+0x10e0>
   25850:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   25854:	48 89 c1             	mov    %rax,%rcx
   25857:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2585b:	48 85 c9             	test   %rcx,%rcx
   2585e:	7e 09                	jle    25869 <Data.String.string-length-from+0x10c9>
   25860:	48 ff c0             	inc    %rax
   25863:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   25867:	eb 17                	jmp    25880 <Data.String.string-length-from+0x10e0>
   25869:	74 15                	je     25880 <Data.String.string-length-from+0x10e0>
   2586b:	4c 89 f7             	mov    %r14,%rdi
   2586e:	e8 fd 49 04 00       	call   6a270 <cinch_dup_mt>
   25873:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2587a:	84 00 00 00 00 00 
   25880:	4d 85 ff             	test   %r15,%r15
   25883:	74 3b                	je     258c0 <Data.String.string-length-from+0x1120>
   25885:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   25889:	48 89 c8             	mov    %rcx,%rax
   2588c:	48 c1 e0 1d          	shl    $0x1d,%rax
   25890:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25896:	7c 0e                	jl     258a6 <Data.String.string-length-from+0x1106>
   25898:	48 ff c9             	dec    %rcx
   2589b:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2589f:	44 85 f3             	test   %r14d,%ebx
   258a2:	75 21                	jne    258c5 <Data.String.string-length-from+0x1125>
   258a4:	eb 4a                	jmp    258f0 <Data.String.string-length-from+0x1150>
   258a6:	48 85 c0             	test   %rax,%rax
   258a9:	74 15                	je     258c0 <Data.String.string-length-from+0x1120>
   258ab:	4c 89 ff             	mov    %r15,%rdi
   258ae:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   258b4:	75 30                	jne    258e6 <Data.String.string-length-from+0x1146>
   258b6:	e8 d5 46 04 00       	call   69f90 <cinch_drop_slow>
   258bb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   258c0:	44 85 f3             	test   %r14d,%ebx
   258c3:	74 2b                	je     258f0 <Data.String.string-length-from+0x1150>
   258c5:	4c 89 e9             	mov    %r13,%rcx
   258c8:	48 d1 e9             	shr    $1,%rcx
   258cb:	4c 89 f0             	mov    %r14,%rax
   258ce:	48 d1 e8             	shr    $1,%rax
   258d1:	48 01 c8             	add    %rcx,%rax
   258d4:	78 1a                	js     258f0 <Data.String.string-length-from+0x1150>
   258d6:	48 8d 1c 45 01 00 00 	lea    0x1(,%rax,2),%rbx
   258dd:	00 
   258de:	41 f6 c4 01          	test   $0x1,%r12b
   258e2:	74 20                	je     25904 <Data.String.string-length-from+0x1164>
   258e4:	eb 5a                	jmp    25940 <Data.String.string-length-from+0x11a0>
   258e6:	e8 b5 45 04 00       	call   69ea0 <cinch_drop_mt>
   258eb:	44 85 f3             	test   %r14d,%ebx
   258ee:	75 d5                	jne    258c5 <Data.String.string-length-from+0x1125>
   258f0:	4c 89 ef             	mov    %r13,%rdi
   258f3:	4c 89 f6             	mov    %r14,%rsi
   258f6:	e8 a5 99 04 00       	call   6f2a0 <cinch_nat_add>
   258fb:	48 89 c3             	mov    %rax,%rbx
   258fe:	41 f6 c4 01          	test   $0x1,%r12b
   25902:	75 3c                	jne    25940 <Data.String.string-length-from+0x11a0>
   25904:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   25908:	48 89 c8             	mov    %rcx,%rax
   2590b:	48 c1 e0 1d          	shl    $0x1d,%rax
   2590f:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25915:	7c 09                	jl     25920 <Data.String.string-length-from+0x1180>
   25917:	48 ff c9             	dec    %rcx
   2591a:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2591e:	eb 20                	jmp    25940 <Data.String.string-length-from+0x11a0>
   25920:	48 85 c0             	test   %rax,%rax
   25923:	74 1b                	je     25940 <Data.String.string-length-from+0x11a0>
   25925:	4c 89 ef             	mov    %r13,%rdi
   25928:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2592e:	75 07                	jne    25937 <Data.String.string-length-from+0x1197>
   25930:	e8 5b 46 04 00       	call   69f90 <cinch_drop_slow>
   25935:	eb 09                	jmp    25940 <Data.String.string-length-from+0x11a0>
   25937:	e8 64 45 04 00       	call   69ea0 <cinch_drop_mt>
   2593c:	0f 1f 40 00          	nopl   0x0(%rax)
   25940:	4d 85 f6             	test   %r14,%r14
   25943:	0f 94 c0             	sete   %al
   25946:	44 89 f1             	mov    %r14d,%ecx
   25949:	08 c1                	or     %al,%cl
   2594b:	f6 c1 01             	test   $0x1,%cl
   2594e:	49 bc ff ff ff ff ff 	movabs $0x7fffffffffffffff,%r12
   25955:	ff ff 7f 
   25958:	49 89 dd             	mov    %rbx,%r13
   2595b:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   2595f:	74 2f                	je     25990 <Data.String.string-length-from+0x11f0>
   25961:	f6 c3 01             	test   $0x1,%bl
   25964:	74 7a                	je     259e0 <Data.String.string-length-from+0x1240>
   25966:	48 89 d8             	mov    %rbx,%rax
   25969:	48 d1 e8             	shr    $1,%rax
   2596c:	48 ff c0             	inc    %rax
   2596f:	78 6f                	js     259e0 <Data.String.string-length-from+0x1240>
   25971:	4c 8d 3c 45 01 00 00 	lea    0x1(,%rax,2),%r15
   25978:	00 
   25979:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   2597d:	e9 46 ee ff ff       	jmp    247c8 <Data.String.string-length-from+0x28>
   25982:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25989:	1f 84 00 00 00 00 00 
   25990:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   25994:	48 89 c8             	mov    %rcx,%rax
   25997:	48 c1 e0 1d          	shl    $0x1d,%rax
   2599b:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   259a1:	7c 0e                	jl     259b1 <Data.String.string-length-from+0x1211>
   259a3:	48 ff c9             	dec    %rcx
   259a6:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   259aa:	f6 c3 01             	test   $0x1,%bl
   259ad:	75 b7                	jne    25966 <Data.String.string-length-from+0x11c6>
   259af:	eb 2f                	jmp    259e0 <Data.String.string-length-from+0x1240>
   259b1:	48 85 c0             	test   %rax,%rax
   259b4:	74 ab                	je     25961 <Data.String.string-length-from+0x11c1>
   259b6:	4c 89 f7             	mov    %r14,%rdi
   259b9:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   259bf:	75 07                	jne    259c8 <Data.String.string-length-from+0x1228>
   259c1:	e8 ca 45 04 00       	call   69f90 <cinch_drop_slow>
   259c6:	eb 99                	jmp    25961 <Data.String.string-length-from+0x11c1>
   259c8:	e8 d3 44 04 00       	call   69ea0 <cinch_drop_mt>
   259cd:	f6 c3 01             	test   $0x1,%bl
   259d0:	75 94                	jne    25966 <Data.String.string-length-from+0x11c6>
   259d2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   259d9:	1f 84 00 00 00 00 00 
   259e0:	be 03 00 00 00       	mov    $0x3,%esi
   259e5:	48 89 df             	mov    %rbx,%rdi
   259e8:	e8 b3 98 04 00       	call   6f2a0 <cinch_nat_add>
   259ed:	49 89 c7             	mov    %rax,%r15
   259f0:	48 85 db             	test   %rbx,%rbx
   259f3:	0f 94 c0             	sete   %al
   259f6:	89 d9                	mov    %ebx,%ecx
   259f8:	08 c1                	or     %al,%cl
   259fa:	f6 c1 01             	test   $0x1,%cl
   259fd:	4c 8b 75 d0          	mov    -0x30(%rbp),%r14
   25a01:	0f 85 c1 ed ff ff    	jne    247c8 <Data.String.string-length-from+0x28>
   25a07:	48 8b 4b f8          	mov    -0x8(%rbx),%rcx
   25a0b:	48 89 c8             	mov    %rcx,%rax
   25a0e:	48 c1 e0 1d          	shl    $0x1d,%rax
   25a12:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25a18:	7c 0c                	jl     25a26 <Data.String.string-length-from+0x1286>
   25a1a:	48 ff c9             	dec    %rcx
   25a1d:	48 89 4b f8          	mov    %rcx,-0x8(%rbx)
   25a21:	e9 a2 ed ff ff       	jmp    247c8 <Data.String.string-length-from+0x28>
   25a26:	48 85 c0             	test   %rax,%rax
   25a29:	0f 84 99 ed ff ff    	je     247c8 <Data.String.string-length-from+0x28>
   25a2f:	48 89 df             	mov    %rbx,%rdi
   25a32:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25a38:	75 0a                	jne    25a44 <Data.String.string-length-from+0x12a4>
   25a3a:	e8 51 45 04 00       	call   69f90 <cinch_drop_slow>
   25a3f:	e9 84 ed ff ff       	jmp    247c8 <Data.String.string-length-from+0x28>
   25a44:	e8 57 44 04 00       	call   69ea0 <cinch_drop_mt>
   25a49:	e9 7a ed ff ff       	jmp    247c8 <Data.String.string-length-from+0x28>
   25a4e:	48 85 c0             	test   %rax,%rax
   25a51:	75 66                	jne    25ab9 <Data.String.string-length-from+0x1319>
   25a53:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   25a57:	48 85 ff             	test   %rdi,%rdi
   25a5a:	0f 84 47 01 00 00    	je     25ba7 <Data.String.string-length-from+0x1407>
   25a60:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   25a64:	48 89 c8             	mov    %rcx,%rax
   25a67:	48 c1 e0 1d          	shl    $0x1d,%rax
   25a6b:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25a71:	0f 8c c5 00 00 00    	jl     25b3c <Data.String.string-length-from+0x139c>
   25a77:	48 ff c9             	dec    %rcx
   25a7a:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   25a7e:	e9 24 01 00 00       	jmp    25ba7 <Data.String.string-length-from+0x1407>
   25a83:	48 85 c9             	test   %rcx,%rcx
   25a86:	75 31                	jne    25ab9 <Data.String.string-length-from+0x1319>
   25a88:	4d 85 f6             	test   %r14,%r14
   25a8b:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25a8f:	0f 84 70 01 00 00    	je     25c05 <Data.String.string-length-from+0x1465>
   25a95:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   25a99:	48 89 d1             	mov    %rdx,%rcx
   25a9c:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25aa0:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   25aa7:	0f 8c c2 00 00 00    	jl     25b6f <Data.String.string-length-from+0x13cf>
   25aad:	48 ff ca             	dec    %rdx
   25ab0:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   25ab4:	e9 4c 01 00 00       	jmp    25c05 <Data.String.string-length-from+0x1465>
   25ab9:	31 db                	xor    %ebx,%ebx
   25abb:	48 89 d8             	mov    %rbx,%rax
   25abe:	48 83 c4 38          	add    $0x38,%rsp
   25ac2:	5b                   	pop    %rbx
   25ac3:	41 5c                	pop    %r12
   25ac5:	41 5d                	pop    %r13
   25ac7:	41 5e                	pop    %r14
   25ac9:	41 5f                	pop    %r15
   25acb:	5d                   	pop    %rbp
   25acc:	c3                   	ret
   25acd:	4d 85 f6             	test   %r14,%r14
   25ad0:	48 8b 5d b0          	mov    -0x50(%rbp),%rbx
   25ad4:	0f 84 b9 00 00 00    	je     25b93 <Data.String.string-length-from+0x13f3>
   25ada:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   25ade:	48 89 c8             	mov    %rcx,%rax
   25ae1:	48 c1 e0 1d          	shl    $0x1d,%rax
   25ae5:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25aeb:	7c 38                	jl     25b25 <Data.String.string-length-from+0x1385>
   25aed:	48 ff c9             	dec    %rcx
   25af0:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   25af4:	e9 9a 00 00 00       	jmp    25b93 <Data.String.string-length-from+0x13f3>
   25af9:	4d 85 f6             	test   %r14,%r14
   25afc:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25b00:	0f 84 e8 00 00 00    	je     25bee <Data.String.string-length-from+0x144e>
   25b06:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   25b0a:	48 89 c8             	mov    %rcx,%rax
   25b0d:	48 c1 e0 1d          	shl    $0x1d,%rax
   25b11:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25b17:	7c 37                	jl     25b50 <Data.String.string-length-from+0x13b0>
   25b19:	48 ff c9             	dec    %rcx
   25b1c:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   25b20:	e9 c9 00 00 00       	jmp    25bee <Data.String.string-length-from+0x144e>
   25b25:	48 85 c0             	test   %rax,%rax
   25b28:	74 69                	je     25b93 <Data.String.string-length-from+0x13f3>
   25b2a:	4c 89 f7             	mov    %r14,%rdi
   25b2d:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25b33:	75 59                	jne    25b8e <Data.String.string-length-from+0x13ee>
   25b35:	e8 56 44 04 00       	call   69f90 <cinch_drop_slow>
   25b3a:	eb 57                	jmp    25b93 <Data.String.string-length-from+0x13f3>
   25b3c:	48 85 c0             	test   %rax,%rax
   25b3f:	74 66                	je     25ba7 <Data.String.string-length-from+0x1407>
   25b41:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25b47:	75 59                	jne    25ba2 <Data.String.string-length-from+0x1402>
   25b49:	e8 42 44 04 00       	call   69f90 <cinch_drop_slow>
   25b4e:	eb 57                	jmp    25ba7 <Data.String.string-length-from+0x1407>
   25b50:	48 85 c0             	test   %rax,%rax
   25b53:	0f 84 95 00 00 00    	je     25bee <Data.String.string-length-from+0x144e>
   25b59:	4c 89 f7             	mov    %r14,%rdi
   25b5c:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25b62:	0f 85 81 00 00 00    	jne    25be9 <Data.String.string-length-from+0x1449>
   25b68:	e8 23 44 04 00       	call   69f90 <cinch_drop_slow>
   25b6d:	eb 7f                	jmp    25bee <Data.String.string-length-from+0x144e>
   25b6f:	48 85 c9             	test   %rcx,%rcx
   25b72:	0f 84 8d 00 00 00    	je     25c05 <Data.String.string-length-from+0x1465>
   25b78:	48 89 c3             	mov    %rax,%rbx
   25b7b:	4c 89 f7             	mov    %r14,%rdi
   25b7e:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   25b85:	75 72                	jne    25bf9 <Data.String.string-length-from+0x1459>
   25b87:	e8 04 44 04 00       	call   69f90 <cinch_drop_slow>
   25b8c:	eb 70                	jmp    25bfe <Data.String.string-length-from+0x145e>
   25b8e:	e8 0d 43 04 00       	call   69ea0 <cinch_drop_mt>
   25b93:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   25b97:	0f 84 bc 00 00 00    	je     25c59 <Data.String.string-length-from+0x14b9>
   25b9d:	e9 19 ff ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>
   25ba2:	e8 f9 42 04 00       	call   69ea0 <cinch_drop_mt>
   25ba7:	f6 45 c8 01          	testb  $0x1,-0x38(%rbp)
   25bab:	0f 85 9f 00 00 00    	jne    25c50 <Data.String.string-length-from+0x14b0>
   25bb1:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   25bb5:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   25bb9:	48 89 c8             	mov    %rcx,%rax
   25bbc:	48 c1 e0 1d          	shl    $0x1d,%rax
   25bc0:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25bc6:	7c 09                	jl     25bd1 <Data.String.string-length-from+0x1431>
   25bc8:	48 ff c9             	dec    %rcx
   25bcb:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   25bcf:	eb 7f                	jmp    25c50 <Data.String.string-length-from+0x14b0>
   25bd1:	48 85 c0             	test   %rax,%rax
   25bd4:	74 7a                	je     25c50 <Data.String.string-length-from+0x14b0>
   25bd6:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25bdc:	75 69                	jne    25c47 <Data.String.string-length-from+0x14a7>
   25bde:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   25be2:	e8 a9 43 04 00       	call   69f90 <cinch_drop_slow>
   25be7:	eb 67                	jmp    25c50 <Data.String.string-length-from+0x14b0>
   25be9:	e8 b2 42 04 00       	call   69ea0 <cinch_drop_mt>
   25bee:	41 f6 c4 01          	test   $0x1,%r12b
   25bf2:	74 65                	je     25c59 <Data.String.string-length-from+0x14b9>
   25bf4:	e9 c2 fe ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>
   25bf9:	e8 a2 42 04 00       	call   69ea0 <cinch_drop_mt>
   25bfe:	48 89 d8             	mov    %rbx,%rax
   25c01:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25c05:	41 f6 c4 01          	test   $0x1,%r12b
   25c09:	0f 85 93 00 00 00    	jne    25ca2 <Data.String.string-length-from+0x1502>
   25c0f:	49 8b 55 f8          	mov    -0x8(%r13),%rdx
   25c13:	48 89 d1             	mov    %rdx,%rcx
   25c16:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25c1a:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   25c21:	7c 09                	jl     25c2c <Data.String.string-length-from+0x148c>
   25c23:	48 ff ca             	dec    %rdx
   25c26:	49 89 55 f8          	mov    %rdx,-0x8(%r13)
   25c2a:	eb 76                	jmp    25ca2 <Data.String.string-length-from+0x1502>
   25c2c:	48 85 c9             	test   %rcx,%rcx
   25c2f:	74 71                	je     25ca2 <Data.String.string-length-from+0x1502>
   25c31:	48 89 c3             	mov    %rax,%rbx
   25c34:	4c 89 ef             	mov    %r13,%rdi
   25c37:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   25c3e:	75 56                	jne    25c96 <Data.String.string-length-from+0x14f6>
   25c40:	e8 4b 43 04 00       	call   69f90 <cinch_drop_slow>
   25c45:	eb 54                	jmp    25c9b <Data.String.string-length-from+0x14fb>
   25c47:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   25c4b:	e8 50 42 04 00       	call   69ea0 <cinch_drop_mt>
   25c50:	4d 85 ed             	test   %r13,%r13
   25c53:	0f 84 62 fe ff ff    	je     25abb <Data.String.string-length-from+0x131b>
   25c59:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   25c5d:	48 89 c8             	mov    %rcx,%rax
   25c60:	48 c1 e0 1d          	shl    $0x1d,%rax
   25c64:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   25c6a:	7c 0c                	jl     25c78 <Data.String.string-length-from+0x14d8>
   25c6c:	48 ff c9             	dec    %rcx
   25c6f:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   25c73:	e9 43 fe ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>
   25c78:	48 85 c0             	test   %rax,%rax
   25c7b:	0f 84 3a fe ff ff    	je     25abb <Data.String.string-length-from+0x131b>
   25c81:	4c 89 ef             	mov    %r13,%rdi
   25c84:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   25c8a:	75 54                	jne    25ce0 <Data.String.string-length-from+0x1540>
   25c8c:	e8 ff 42 04 00       	call   69f90 <cinch_drop_slow>
   25c91:	e9 25 fe ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>
   25c96:	e8 05 42 04 00       	call   69ea0 <cinch_drop_mt>
   25c9b:	48 89 d8             	mov    %rbx,%rax
   25c9e:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   25ca2:	48 85 c0             	test   %rax,%rax
   25ca5:	0f 84 10 fe ff ff    	je     25abb <Data.String.string-length-from+0x131b>
   25cab:	48 8b 50 f8          	mov    -0x8(%rax),%rdx
   25caf:	48 89 d1             	mov    %rdx,%rcx
   25cb2:	48 c1 e1 1d          	shl    $0x1d,%rcx
   25cb6:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   25cbd:	7c 0c                	jl     25ccb <Data.String.string-length-from+0x152b>
   25cbf:	48 ff ca             	dec    %rdx
   25cc2:	48 89 50 f8          	mov    %rdx,-0x8(%rax)
   25cc6:	e9 f0 fd ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>
   25ccb:	48 85 c9             	test   %rcx,%rcx
   25cce:	0f 84 e7 fd ff ff    	je     25abb <Data.String.string-length-from+0x131b>
   25cd4:	48 89 c7             	mov    %rax,%rdi
   25cd7:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   25cde:	74 ac                	je     25c8c <Data.String.string-length-from+0x14ec>
   25ce0:	e8 bb 41 04 00       	call   69ea0 <cinch_drop_mt>
   25ce5:	e9 d1 fd ff ff       	jmp    25abb <Data.String.string-length-from+0x131b>

Disassembly of section .fini:
