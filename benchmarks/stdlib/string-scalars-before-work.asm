
/workspace/dasl-stdlib-strings/build/string-scalars-benchmark-phase1/before/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000002b6e0 <Main.work>:
   2b6e0:	55                   	push   %rbp
   2b6e1:	48 89 e5             	mov    %rsp,%rbp
   2b6e4:	41 57                	push   %r15
   2b6e6:	41 56                	push   %r14
   2b6e8:	41 55                	push   %r13
   2b6ea:	41 54                	push   %r12
   2b6ec:	53                   	push   %rbx
   2b6ed:	48 83 ec 28          	sub    $0x28,%rsp
   2b6f1:	4c 89 45 d0          	mov    %r8,-0x30(%rbp)
   2b6f5:	49 89 cc             	mov    %rcx,%r12
   2b6f8:	49 89 d7             	mov    %rdx,%r15
   2b6fb:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
   2b6ff:	48 89 7d c8          	mov    %rdi,-0x38(%rbp)
   2b703:	eb 22                	jmp    2b727 <Main.work+0x47>
   2b705:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2b70c:	00 00 00 00 
   2b710:	4c 89 e8             	mov    %r13,%rax
   2b713:	48 d1 e8             	shr    $1,%rax
   2b716:	48 ff c0             	inc    %rax
   2b719:	0f 88 6b 0a 00 00    	js     2c18a <Main.work+0xaaa>
   2b71f:	4c 8d 24 45 01 00 00 	lea    0x1(,%rax,2),%r12
   2b726:	00 
   2b727:	4d 85 ff             	test   %r15,%r15
   2b72a:	0f 94 c0             	sete   %al
   2b72d:	45 89 fd             	mov    %r15d,%r13d
   2b730:	41 08 c5             	or     %al,%r13b
   2b733:	41 f6 c5 01          	test   $0x1,%r13b
   2b737:	74 27                	je     2b760 <Main.work+0x80>
   2b739:	41 f6 c7 01          	test   $0x1,%r15b
   2b73d:	75 41                	jne    2b780 <Main.work+0xa0>
   2b73f:	be 01 00 00 00       	mov    $0x1,%esi
   2b744:	4c 89 ff             	mov    %r15,%rdi
   2b747:	e8 44 3d 04 00       	call   6f490 <cinch_nat_cmp>
   2b74c:	e9 af 00 00 00       	jmp    2b800 <Main.work+0x120>
   2b751:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b758:	0f 1f 84 00 00 00 00 
   2b75f:	00 
   2b760:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   2b764:	48 89 c1             	mov    %rax,%rcx
   2b767:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b76b:	48 85 c9             	test   %rcx,%rcx
   2b76e:	7e 24                	jle    2b794 <Main.work+0xb4>
   2b770:	48 ff c0             	inc    %rax
   2b773:	49 89 47 f8          	mov    %rax,-0x8(%r15)
   2b777:	eb 25                	jmp    2b79e <Main.work+0xbe>
   2b779:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2b780:	4c 89 f8             	mov    %r15,%rax
   2b783:	48 d1 e8             	shr    $1,%rax
   2b786:	48 85 c0             	test   %rax,%rax
   2b789:	0f 97 c0             	seta   %al
   2b78c:	1c 00                	sbb    $0x0,%al
   2b78e:	48 0f be c0          	movsbq %al,%rax
   2b792:	eb 6c                	jmp    2b800 <Main.work+0x120>
   2b794:	74 08                	je     2b79e <Main.work+0xbe>
   2b796:	4c 89 ff             	mov    %r15,%rdi
   2b799:	e8 d2 ea 03 00       	call   6a270 <cinch_dup_mt>
   2b79e:	be 01 00 00 00       	mov    $0x1,%esi
   2b7a3:	4c 89 ff             	mov    %r15,%rdi
   2b7a6:	e8 e5 3c 04 00       	call   6f490 <cinch_nat_cmp>
   2b7ab:	49 8b 57 f8          	mov    -0x8(%r15),%rdx
   2b7af:	48 89 d1             	mov    %rdx,%rcx
   2b7b2:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b7b6:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2b7bd:	7c 11                	jl     2b7d0 <Main.work+0xf0>
   2b7bf:	48 ff ca             	dec    %rdx
   2b7c2:	49 89 57 f8          	mov    %rdx,-0x8(%r15)
   2b7c6:	eb 38                	jmp    2b800 <Main.work+0x120>
   2b7c8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   2b7cf:	00 
   2b7d0:	48 85 c9             	test   %rcx,%rcx
   2b7d3:	74 2b                	je     2b800 <Main.work+0x120>
   2b7d5:	48 89 c3             	mov    %rax,%rbx
   2b7d8:	4c 89 ff             	mov    %r15,%rdi
   2b7db:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2b7e2:	75 07                	jne    2b7eb <Main.work+0x10b>
   2b7e4:	e8 a7 e7 03 00       	call   69f90 <cinch_drop_slow>
   2b7e9:	eb 05                	jmp    2b7f0 <Main.work+0x110>
   2b7eb:	e8 b0 e6 03 00       	call   69ea0 <cinch_drop_mt>
   2b7f0:	48 89 d8             	mov    %rbx,%rax
   2b7f3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b7fa:	84 00 00 00 00 00 
   2b800:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2b804:	48 85 c0             	test   %rax,%rax
   2b807:	0f 84 3c 0a 00 00    	je     2c249 <Main.work+0xb69>
   2b80d:	41 f6 c4 01          	test   $0x1,%r12b
   2b811:	75 2d                	jne    2b840 <Main.work+0x160>
   2b813:	be 05 00 00 00       	mov    $0x5,%esi
   2b818:	4c 89 e7             	mov    %r12,%rdi
   2b81b:	e8 30 46 04 00       	call   6fe50 <cinch_nat_rem>
   2b820:	49 89 c6             	mov    %rax,%r14
   2b823:	41 f6 c6 01          	test   $0x1,%r14b
   2b827:	74 24                	je     2b84d <Main.work+0x16d>
   2b829:	49 d1 ee             	shr    $1,%r14
   2b82c:	4d 85 f6             	test   %r14,%r14
   2b82f:	0f 97 c0             	seta   %al
   2b832:	1c 00                	sbb    $0x0,%al
   2b834:	48 0f be c0          	movsbq %al,%rax
   2b838:	eb 66                	jmp    2b8a0 <Main.work+0x1c0>
   2b83a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2b840:	45 89 e6             	mov    %r12d,%r14d
   2b843:	41 83 e6 03          	and    $0x3,%r14d
   2b847:	41 f6 c6 01          	test   $0x1,%r14b
   2b84b:	75 dc                	jne    2b829 <Main.work+0x149>
   2b84d:	be 01 00 00 00       	mov    $0x1,%esi
   2b852:	4c 89 f7             	mov    %r14,%rdi
   2b855:	e8 36 3c 04 00       	call   6f490 <cinch_nat_cmp>
   2b85a:	4d 85 f6             	test   %r14,%r14
   2b85d:	74 41                	je     2b8a0 <Main.work+0x1c0>
   2b85f:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   2b863:	48 89 d1             	mov    %rdx,%rcx
   2b866:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b86a:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2b871:	7c 09                	jl     2b87c <Main.work+0x19c>
   2b873:	48 ff ca             	dec    %rdx
   2b876:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   2b87a:	eb 24                	jmp    2b8a0 <Main.work+0x1c0>
   2b87c:	48 85 c9             	test   %rcx,%rcx
   2b87f:	74 1f                	je     2b8a0 <Main.work+0x1c0>
   2b881:	48 89 c3             	mov    %rax,%rbx
   2b884:	4c 89 f7             	mov    %r14,%rdi
   2b887:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2b88e:	75 07                	jne    2b897 <Main.work+0x1b7>
   2b890:	e8 fb e6 03 00       	call   69f90 <cinch_drop_slow>
   2b895:	eb 05                	jmp    2b89c <Main.work+0x1bc>
   2b897:	e8 04 e6 03 00       	call   69ea0 <cinch_drop_mt>
   2b89c:	48 89 d8             	mov    %rbx,%rax
   2b89f:	90                   	nop
   2b8a0:	48 85 c0             	test   %rax,%rax
   2b8a3:	74 2b                	je     2b8d0 <Main.work+0x1f0>
   2b8a5:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2b8a9:	48 85 ff             	test   %rdi,%rdi
   2b8ac:	74 52                	je     2b900 <Main.work+0x220>
   2b8ae:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2b8b2:	48 89 c1             	mov    %rax,%rcx
   2b8b5:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b8b9:	48 85 c9             	test   %rcx,%rcx
   2b8bc:	7f 2b                	jg     2b8e9 <Main.work+0x209>
   2b8be:	74 60                	je     2b920 <Main.work+0x240>
   2b8c0:	48 8b 5d c0          	mov    -0x40(%rbp),%rbx
   2b8c4:	eb 44                	jmp    2b90a <Main.work+0x22a>
   2b8c6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2b8cd:	00 00 00 
   2b8d0:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2b8d4:	48 85 ff             	test   %rdi,%rdi
   2b8d7:	74 27                	je     2b900 <Main.work+0x220>
   2b8d9:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2b8dd:	48 89 c1             	mov    %rax,%rcx
   2b8e0:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b8e4:	48 85 c9             	test   %rcx,%rcx
   2b8e7:	7e 1b                	jle    2b904 <Main.work+0x224>
   2b8e9:	48 ff c0             	inc    %rax
   2b8ec:	48 89 47 f8          	mov    %rax,-0x8(%rdi)
   2b8f0:	eb 2e                	jmp    2b920 <Main.work+0x240>
   2b8f2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b8f9:	1f 84 00 00 00 00 00 
   2b900:	31 ff                	xor    %edi,%edi
   2b902:	eb 1c                	jmp    2b920 <Main.work+0x240>
   2b904:	74 1a                	je     2b920 <Main.work+0x240>
   2b906:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
   2b90a:	48 89 df             	mov    %rbx,%rdi
   2b90d:	e8 5e e9 03 00       	call   6a270 <cinch_dup_mt>
   2b912:	48 89 df             	mov    %rbx,%rdi
   2b915:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2b91c:	00 00 00 00 
   2b920:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2b924:	48 c1 e8 28          	shr    $0x28,%rax
   2b928:	0f b6 c0             	movzbl %al,%eax
   2b92b:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   2b931:	75 07                	jne    2b93a <Main.work+0x25a>
   2b933:	48 8b 47 f0          	mov    -0x10(%rdi),%rax
   2b937:	48 8b 00             	mov    (%rax),%rax
   2b93a:	48 85 c0             	test   %rax,%rax
   2b93d:	0f 85 2f 09 00 00    	jne    2c272 <Main.work+0xb92>
   2b943:	4c 8b 37             	mov    (%rdi),%r14
   2b946:	48 85 ff             	test   %rdi,%rdi
   2b949:	74 25                	je     2b970 <Main.work+0x290>
   2b94b:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2b94f:	48 b9 ff ff ff ff 07 	movabs $0x7ffffffff,%rcx
   2b956:	00 00 00 
   2b959:	48 21 c8             	and    %rcx,%rax
   2b95c:	48 83 f8 01          	cmp    $0x1,%rax
   2b960:	75 0e                	jne    2b970 <Main.work+0x290>
   2b962:	e8 39 fc 03 00       	call   6b5a0 <cinch_free>
   2b967:	e9 84 00 00 00       	jmp    2b9f0 <Main.work+0x310>
   2b96c:	0f 1f 40 00          	nopl   0x0(%rax)
   2b970:	4d 85 f6             	test   %r14,%r14
   2b973:	74 2b                	je     2b9a0 <Main.work+0x2c0>
   2b975:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2b979:	48 89 c1             	mov    %rax,%rcx
   2b97c:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b980:	48 85 c9             	test   %rcx,%rcx
   2b983:	7e 09                	jle    2b98e <Main.work+0x2ae>
   2b985:	48 ff c0             	inc    %rax
   2b988:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2b98c:	eb 12                	jmp    2b9a0 <Main.work+0x2c0>
   2b98e:	74 10                	je     2b9a0 <Main.work+0x2c0>
   2b990:	48 89 fb             	mov    %rdi,%rbx
   2b993:	4c 89 f7             	mov    %r14,%rdi
   2b996:	e8 d5 e8 03 00       	call   6a270 <cinch_dup_mt>
   2b99b:	48 89 df             	mov    %rbx,%rdi
   2b99e:	66 90                	xchg   %ax,%ax
   2b9a0:	48 85 ff             	test   %rdi,%rdi
   2b9a3:	74 4b                	je     2b9f0 <Main.work+0x310>
   2b9a5:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   2b9a9:	48 89 c8             	mov    %rcx,%rax
   2b9ac:	48 c1 e0 1d          	shl    $0x1d,%rax
   2b9b0:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2b9b6:	7c 18                	jl     2b9d0 <Main.work+0x2f0>
   2b9b8:	48 ff c9             	dec    %rcx
   2b9bb:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   2b9bf:	eb 2f                	jmp    2b9f0 <Main.work+0x310>
   2b9c1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b9c8:	0f 1f 84 00 00 00 00 
   2b9cf:	00 
   2b9d0:	48 85 c0             	test   %rax,%rax
   2b9d3:	74 1b                	je     2b9f0 <Main.work+0x310>
   2b9d5:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2b9db:	75 07                	jne    2b9e4 <Main.work+0x304>
   2b9dd:	e8 ae e5 03 00       	call   69f90 <cinch_drop_slow>
   2b9e2:	eb 0c                	jmp    2b9f0 <Main.work+0x310>
   2b9e4:	e8 b7 e4 03 00       	call   69ea0 <cinch_drop_mt>
   2b9e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2b9f0:	4c 89 65 b8          	mov    %r12,-0x48(%rbp)
   2b9f4:	45 89 ec             	mov    %r13d,%r12d
   2b9f7:	be 01 00 00 00       	mov    $0x1,%esi
   2b9fc:	ba 01 00 00 00       	mov    $0x1,%edx
   2ba01:	4c 89 f7             	mov    %r14,%rdi
   2ba04:	e8 97 8d ff ff       	call   247a0 <Data.String.string-length-from>
   2ba09:	49 89 c5             	mov    %rax,%r13
   2ba0c:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2ba10:	89 fb                	mov    %edi,%ebx
   2ba12:	83 e3 01             	and    $0x1,%ebx
   2ba15:	44 85 eb             	test   %r13d,%ebx
   2ba18:	74 26                	je     2ba40 <Main.work+0x360>
   2ba1a:	48 89 f9             	mov    %rdi,%rcx
   2ba1d:	48 d1 e9             	shr    $1,%rcx
   2ba20:	4c 89 e8             	mov    %r13,%rax
   2ba23:	48 d1 e8             	shr    $1,%rax
   2ba26:	48 01 c8             	add    %rcx,%rax
   2ba29:	78 15                	js     2ba40 <Main.work+0x360>
   2ba2b:	4c 8d 34 45 01 00 00 	lea    0x1(,%rax,2),%r14
   2ba32:	00 
   2ba33:	eb 1a                	jmp    2ba4f <Main.work+0x36f>
   2ba35:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2ba3c:	00 00 00 00 
   2ba40:	4c 89 ee             	mov    %r13,%rsi
   2ba43:	e8 58 38 04 00       	call   6f2a0 <cinch_nat_add>
   2ba48:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2ba4c:	49 89 c6             	mov    %rax,%r14
   2ba4f:	48 85 ff             	test   %rdi,%rdi
   2ba52:	0f 94 c0             	sete   %al
   2ba55:	08 c3                	or     %al,%bl
   2ba57:	75 47                	jne    2baa0 <Main.work+0x3c0>
   2ba59:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   2ba5d:	48 89 c8             	mov    %rcx,%rax
   2ba60:	48 c1 e0 1d          	shl    $0x1d,%rax
   2ba64:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2ba6a:	7c 14                	jl     2ba80 <Main.work+0x3a0>
   2ba6c:	48 ff c9             	dec    %rcx
   2ba6f:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   2ba73:	eb 2b                	jmp    2baa0 <Main.work+0x3c0>
   2ba75:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2ba7c:	00 00 00 00 
   2ba80:	48 85 c0             	test   %rax,%rax
   2ba83:	74 1b                	je     2baa0 <Main.work+0x3c0>
   2ba85:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2ba8b:	75 07                	jne    2ba94 <Main.work+0x3b4>
   2ba8d:	e8 fe e4 03 00       	call   69f90 <cinch_drop_slow>
   2ba92:	eb 0c                	jmp    2baa0 <Main.work+0x3c0>
   2ba94:	e8 07 e4 03 00       	call   69ea0 <cinch_drop_mt>
   2ba99:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2baa0:	4d 85 ed             	test   %r13,%r13
   2baa3:	0f 94 c0             	sete   %al
   2baa6:	44 89 e9             	mov    %r13d,%ecx
   2baa9:	08 c1                	or     %al,%cl
   2baab:	f6 c1 01             	test   $0x1,%cl
   2baae:	74 40                	je     2baf0 <Main.work+0x410>
   2bab0:	41 f6 c6 01          	test   $0x1,%r14b
   2bab4:	75 5e                	jne    2bb14 <Main.work+0x434>
   2bab6:	be 0f 94 35 77       	mov    $0x7735940f,%esi
   2babb:	4c 89 f7             	mov    %r14,%rdi
   2babe:	e8 8d 43 04 00       	call   6fe50 <cinch_nat_rem>
   2bac3:	4d 85 f6             	test   %r14,%r14
   2bac6:	74 74                	je     2bb3c <Main.work+0x45c>
   2bac8:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2bacc:	48 89 ca             	mov    %rcx,%rdx
   2bacf:	48 c1 e2 1d          	shl    $0x1d,%rdx
   2bad3:	48 81 fa 01 00 00 20 	cmp    $0x20000001,%rdx
   2bada:	0f 8c 3e 01 00 00    	jl     2bc1e <Main.work+0x53e>
   2bae0:	48 ff c9             	dec    %rcx
   2bae3:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2bae7:	eb 53                	jmp    2bb3c <Main.work+0x45c>
   2bae9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2baf0:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2baf4:	48 89 c8             	mov    %rcx,%rax
   2baf7:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bafb:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bb01:	0f 8c d9 00 00 00    	jl     2bbe0 <Main.work+0x500>
   2bb07:	48 ff c9             	dec    %rcx
   2bb0a:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2bb0e:	41 f6 c6 01          	test   $0x1,%r14b
   2bb12:	74 a2                	je     2bab6 <Main.work+0x3d6>
   2bb14:	49 d1 ee             	shr    $1,%r14
   2bb17:	4c 89 f0             	mov    %r14,%rax
   2bb1a:	48 b9 e5 8f a2 12 31 	movabs $0x89705f3112a28fe5,%rcx
   2bb21:	5f 70 89 
   2bb24:	48 f7 e1             	mul    %rcx
   2bb27:	48 c1 ea 1d          	shr    $0x1d,%rdx
   2bb2b:	69 c2 07 ca 9a 3b    	imul   $0x3b9aca07,%edx,%eax
   2bb31:	41 29 c6             	sub    %eax,%r14d
   2bb34:	4a 8d 04 75 01 00 00 	lea    0x1(,%r14,2),%rax
   2bb3b:	00 
   2bb3c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   2bb40:	41 f6 c7 01          	test   $0x1,%r15b
   2bb44:	0f 94 c0             	sete   %al
   2bb47:	49 83 ff 02          	cmp    $0x2,%r15
   2bb4b:	0f 92 c1             	setb   %cl
   2bb4e:	08 c1                	or     %al,%cl
   2bb50:	74 1e                	je     2bb70 <Main.work+0x490>
   2bb52:	be 03 00 00 00       	mov    $0x3,%esi
   2bb57:	4c 89 ff             	mov    %r15,%rdi
   2bb5a:	e8 31 45 04 00       	call   70090 <cinch_nat_sub>
   2bb5f:	49 89 c6             	mov    %rax,%r14
   2bb62:	eb 10                	jmp    2bb74 <Main.work+0x494>
   2bb64:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bb6b:	00 00 00 00 00 
   2bb70:	4d 8d 77 fe          	lea    -0x2(%r15),%r14
   2bb74:	41 f6 c4 01          	test   $0x1,%r12b
   2bb78:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   2bb7c:	74 32                	je     2bbb0 <Main.work+0x4d0>
   2bb7e:	f6 c3 01             	test   $0x1,%bl
   2bb81:	0f 84 e9 00 00 00    	je     2bc70 <Main.work+0x590>
   2bb87:	48 89 d8             	mov    %rbx,%rax
   2bb8a:	48 d1 e8             	shr    $1,%rax
   2bb8d:	48 ff c0             	inc    %rax
   2bb90:	0f 88 da 00 00 00    	js     2bc70 <Main.work+0x590>
   2bb96:	4c 8d 2c 45 01 00 00 	lea    0x1(,%rax,2),%r13
   2bb9d:	00 
   2bb9e:	e9 2d 01 00 00       	jmp    2bcd0 <Main.work+0x5f0>
   2bba3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bbaa:	84 00 00 00 00 00 
   2bbb0:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2bbb4:	48 89 c8             	mov    %rcx,%rax
   2bbb7:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bbbb:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bbc1:	7c 3d                	jl     2bc00 <Main.work+0x520>
   2bbc3:	48 ff c9             	dec    %rcx
   2bbc6:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2bbca:	f6 c3 01             	test   $0x1,%bl
   2bbcd:	75 b8                	jne    2bb87 <Main.work+0x4a7>
   2bbcf:	e9 9c 00 00 00       	jmp    2bc70 <Main.work+0x590>
   2bbd4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bbdb:	00 00 00 00 00 
   2bbe0:	48 85 c0             	test   %rax,%rax
   2bbe3:	0f 84 c7 fe ff ff    	je     2bab0 <Main.work+0x3d0>
   2bbe9:	4c 89 ef             	mov    %r13,%rdi
   2bbec:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bbf2:	75 50                	jne    2bc44 <Main.work+0x564>
   2bbf4:	e8 97 e3 03 00       	call   69f90 <cinch_drop_slow>
   2bbf9:	e9 b2 fe ff ff       	jmp    2bab0 <Main.work+0x3d0>
   2bbfe:	66 90                	xchg   %ax,%ax
   2bc00:	48 85 c0             	test   %rax,%rax
   2bc03:	0f 84 75 ff ff ff    	je     2bb7e <Main.work+0x49e>
   2bc09:	4c 89 ff             	mov    %r15,%rdi
   2bc0c:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bc12:	75 44                	jne    2bc58 <Main.work+0x578>
   2bc14:	e8 77 e3 03 00       	call   69f90 <cinch_drop_slow>
   2bc19:	e9 60 ff ff ff       	jmp    2bb7e <Main.work+0x49e>
   2bc1e:	48 85 d2             	test   %rdx,%rdx
   2bc21:	0f 84 15 ff ff ff    	je     2bb3c <Main.work+0x45c>
   2bc27:	48 89 c3             	mov    %rax,%rbx
   2bc2a:	4c 89 f7             	mov    %r14,%rdi
   2bc2d:	48 81 fa 00 00 00 20 	cmp    $0x20000000,%rdx
   2bc34:	0f 85 3d 02 00 00    	jne    2be77 <Main.work+0x797>
   2bc3a:	e8 51 e3 03 00       	call   69f90 <cinch_drop_slow>
   2bc3f:	e9 38 02 00 00       	jmp    2be7c <Main.work+0x79c>
   2bc44:	e8 57 e2 03 00       	call   69ea0 <cinch_drop_mt>
   2bc49:	41 f6 c6 01          	test   $0x1,%r14b
   2bc4d:	0f 84 63 fe ff ff    	je     2bab6 <Main.work+0x3d6>
   2bc53:	e9 bc fe ff ff       	jmp    2bb14 <Main.work+0x434>
   2bc58:	e8 43 e2 03 00       	call   69ea0 <cinch_drop_mt>
   2bc5d:	f6 c3 01             	test   $0x1,%bl
   2bc60:	0f 85 21 ff ff ff    	jne    2bb87 <Main.work+0x4a7>
   2bc66:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2bc6d:	00 00 00 
   2bc70:	be 03 00 00 00       	mov    $0x3,%esi
   2bc75:	48 89 df             	mov    %rbx,%rdi
   2bc78:	e8 23 36 04 00       	call   6f2a0 <cinch_nat_add>
   2bc7d:	49 89 c5             	mov    %rax,%r13
   2bc80:	48 85 db             	test   %rbx,%rbx
   2bc83:	0f 94 c0             	sete   %al
   2bc86:	89 d9                	mov    %ebx,%ecx
   2bc88:	08 c1                	or     %al,%cl
   2bc8a:	f6 c1 01             	test   $0x1,%cl
   2bc8d:	75 41                	jne    2bcd0 <Main.work+0x5f0>
   2bc8f:	48 8b 4b f8          	mov    -0x8(%rbx),%rcx
   2bc93:	48 89 c8             	mov    %rcx,%rax
   2bc96:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bc9a:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bca0:	7c 09                	jl     2bcab <Main.work+0x5cb>
   2bca2:	48 ff c9             	dec    %rcx
   2bca5:	48 89 4b f8          	mov    %rcx,-0x8(%rbx)
   2bca9:	eb 25                	jmp    2bcd0 <Main.work+0x5f0>
   2bcab:	48 85 c0             	test   %rax,%rax
   2bcae:	74 20                	je     2bcd0 <Main.work+0x5f0>
   2bcb0:	48 89 df             	mov    %rbx,%rdi
   2bcb3:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bcb9:	75 07                	jne    2bcc2 <Main.work+0x5e2>
   2bcbb:	e8 d0 e2 03 00       	call   69f90 <cinch_drop_slow>
   2bcc0:	eb 0e                	jmp    2bcd0 <Main.work+0x5f0>
   2bcc2:	e8 d9 e1 03 00       	call   69ea0 <cinch_drop_mt>
   2bcc7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   2bcce:	00 00 
   2bcd0:	4d 85 f6             	test   %r14,%r14
   2bcd3:	0f 94 c0             	sete   %al
   2bcd6:	45 89 f7             	mov    %r14d,%r15d
   2bcd9:	41 08 c7             	or     %al,%r15b
   2bcdc:	41 f6 c7 01          	test   $0x1,%r15b
   2bce0:	74 1e                	je     2bd00 <Main.work+0x620>
   2bce2:	41 f6 c6 01          	test   $0x1,%r14b
   2bce6:	75 38                	jne    2bd20 <Main.work+0x640>
   2bce8:	be 01 00 00 00       	mov    $0x1,%esi
   2bced:	4c 89 f7             	mov    %r14,%rdi
   2bcf0:	e8 9b 37 04 00       	call   6f490 <cinch_nat_cmp>
   2bcf5:	e9 a6 00 00 00       	jmp    2bda0 <Main.work+0x6c0>
   2bcfa:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2bd00:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2bd04:	48 89 c1             	mov    %rax,%rcx
   2bd07:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2bd0b:	48 85 c9             	test   %rcx,%rcx
   2bd0e:	7e 24                	jle    2bd34 <Main.work+0x654>
   2bd10:	48 ff c0             	inc    %rax
   2bd13:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2bd17:	eb 25                	jmp    2bd3e <Main.work+0x65e>
   2bd19:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2bd20:	4c 89 f0             	mov    %r14,%rax
   2bd23:	48 d1 e8             	shr    $1,%rax
   2bd26:	48 85 c0             	test   %rax,%rax
   2bd29:	0f 97 c0             	seta   %al
   2bd2c:	1c 00                	sbb    $0x0,%al
   2bd2e:	48 0f be c0          	movsbq %al,%rax
   2bd32:	eb 6c                	jmp    2bda0 <Main.work+0x6c0>
   2bd34:	74 08                	je     2bd3e <Main.work+0x65e>
   2bd36:	4c 89 f7             	mov    %r14,%rdi
   2bd39:	e8 32 e5 03 00       	call   6a270 <cinch_dup_mt>
   2bd3e:	be 01 00 00 00       	mov    $0x1,%esi
   2bd43:	4c 89 f7             	mov    %r14,%rdi
   2bd46:	e8 45 37 04 00       	call   6f490 <cinch_nat_cmp>
   2bd4b:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   2bd4f:	48 89 d1             	mov    %rdx,%rcx
   2bd52:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2bd56:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2bd5d:	7c 11                	jl     2bd70 <Main.work+0x690>
   2bd5f:	48 ff ca             	dec    %rdx
   2bd62:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   2bd66:	eb 38                	jmp    2bda0 <Main.work+0x6c0>
   2bd68:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   2bd6f:	00 
   2bd70:	48 85 c9             	test   %rcx,%rcx
   2bd73:	74 2b                	je     2bda0 <Main.work+0x6c0>
   2bd75:	48 89 c3             	mov    %rax,%rbx
   2bd78:	4c 89 f7             	mov    %r14,%rdi
   2bd7b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2bd82:	75 07                	jne    2bd8b <Main.work+0x6ab>
   2bd84:	e8 07 e2 03 00       	call   69f90 <cinch_drop_slow>
   2bd89:	eb 05                	jmp    2bd90 <Main.work+0x6b0>
   2bd8b:	e8 10 e1 03 00       	call   69ea0 <cinch_drop_mt>
   2bd90:	48 89 d8             	mov    %rbx,%rax
   2bd93:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bd9a:	84 00 00 00 00 00 
   2bda0:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bda4:	48 85 c0             	test   %rax,%rax
   2bda7:	0f 84 cc 04 00 00    	je     2c279 <Main.work+0xb99>
   2bdad:	44 88 7d b8          	mov    %r15b,-0x48(%rbp)
   2bdb1:	41 f6 c5 01          	test   $0x1,%r13b
   2bdb5:	75 69                	jne    2be20 <Main.work+0x740>
   2bdb7:	be 05 00 00 00       	mov    $0x5,%esi
   2bdbc:	4c 89 ef             	mov    %r13,%rdi
   2bdbf:	e8 8c 40 04 00       	call   6fe50 <cinch_nat_rem>
   2bdc4:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bdc8:	49 89 c7             	mov    %rax,%r15
   2bdcb:	41 f6 c7 01          	test   $0x1,%r15b
   2bdcf:	74 5c                	je     2be2d <Main.work+0x74d>
   2bdd1:	49 d1 ef             	shr    $1,%r15
   2bdd4:	4d 85 ff             	test   %r15,%r15
   2bdd7:	0f 97 c0             	seta   %al
   2bdda:	1c 00                	sbb    $0x0,%al
   2bddc:	48 0f be c0          	movsbq %al,%rax
   2bde0:	48 85 c0             	test   %rax,%rax
   2bde3:	0f 84 b0 00 00 00    	je     2be99 <Main.work+0x7b9>
   2bde9:	4c 8b 65 c0          	mov    -0x40(%rbp),%r12
   2bded:	4d 85 e4             	test   %r12,%r12
   2bdf0:	0f 84 ca 00 00 00    	je     2bec0 <Main.work+0x7e0>
   2bdf6:	49 8b 44 24 f8       	mov    -0x8(%r12),%rax
   2bdfb:	48 89 c1             	mov    %rax,%rcx
   2bdfe:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2be02:	48 85 c9             	test   %rcx,%rcx
   2be05:	0f 8f a8 00 00 00    	jg     2beb3 <Main.work+0x7d3>
   2be0b:	0f 84 cf 00 00 00    	je     2bee0 <Main.work+0x800>
   2be11:	4c 8b 65 c0          	mov    -0x40(%rbp),%r12
   2be15:	e9 b1 00 00 00       	jmp    2becb <Main.work+0x7eb>
   2be1a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2be20:	45 89 ef             	mov    %r13d,%r15d
   2be23:	41 83 e7 03          	and    $0x3,%r15d
   2be27:	41 f6 c7 01          	test   $0x1,%r15b
   2be2b:	75 a4                	jne    2bdd1 <Main.work+0x6f1>
   2be2d:	be 01 00 00 00       	mov    $0x1,%esi
   2be32:	4c 89 ff             	mov    %r15,%rdi
   2be35:	e8 56 36 04 00       	call   6f490 <cinch_nat_cmp>
   2be3a:	4d 85 ff             	test   %r15,%r15
   2be3d:	74 4d                	je     2be8c <Main.work+0x7ac>
   2be3f:	49 8b 57 f8          	mov    -0x8(%r15),%rdx
   2be43:	48 89 d1             	mov    %rdx,%rcx
   2be46:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2be4a:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2be51:	7c 09                	jl     2be5c <Main.work+0x77c>
   2be53:	48 ff ca             	dec    %rdx
   2be56:	49 89 57 f8          	mov    %rdx,-0x8(%r15)
   2be5a:	eb 30                	jmp    2be8c <Main.work+0x7ac>
   2be5c:	48 85 c9             	test   %rcx,%rcx
   2be5f:	74 2b                	je     2be8c <Main.work+0x7ac>
   2be61:	48 89 c3             	mov    %rax,%rbx
   2be64:	4c 89 ff             	mov    %r15,%rdi
   2be67:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2be6e:	75 14                	jne    2be84 <Main.work+0x7a4>
   2be70:	e8 1b e1 03 00       	call   69f90 <cinch_drop_slow>
   2be75:	eb 12                	jmp    2be89 <Main.work+0x7a9>
   2be77:	e8 24 e0 03 00       	call   69ea0 <cinch_drop_mt>
   2be7c:	48 89 d8             	mov    %rbx,%rax
   2be7f:	e9 b8 fc ff ff       	jmp    2bb3c <Main.work+0x45c>
   2be84:	e8 17 e0 03 00       	call   69ea0 <cinch_drop_mt>
   2be89:	48 89 d8             	mov    %rbx,%rax
   2be8c:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2be90:	48 85 c0             	test   %rax,%rax
   2be93:	0f 85 50 ff ff ff    	jne    2bde9 <Main.work+0x709>
   2be99:	4c 8b 65 c8          	mov    -0x38(%rbp),%r12
   2be9d:	4d 85 e4             	test   %r12,%r12
   2bea0:	74 1e                	je     2bec0 <Main.work+0x7e0>
   2bea2:	49 8b 44 24 f8       	mov    -0x8(%r12),%rax
   2bea7:	48 89 c1             	mov    %rax,%rcx
   2beaa:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2beae:	48 85 c9             	test   %rcx,%rcx
   2beb1:	7e 12                	jle    2bec5 <Main.work+0x7e5>
   2beb3:	48 ff c0             	inc    %rax
   2beb6:	49 89 44 24 f8       	mov    %rax,-0x8(%r12)
   2bebb:	eb 23                	jmp    2bee0 <Main.work+0x800>
   2bebd:	0f 1f 00             	nopl   (%rax)
   2bec0:	45 31 e4             	xor    %r12d,%r12d
   2bec3:	eb 1b                	jmp    2bee0 <Main.work+0x800>
   2bec5:	74 19                	je     2bee0 <Main.work+0x800>
   2bec7:	4c 8b 65 c8          	mov    -0x38(%rbp),%r12
   2becb:	4c 89 e7             	mov    %r12,%rdi
   2bece:	e8 9d e3 03 00       	call   6a270 <cinch_dup_mt>
   2bed3:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bed7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   2bede:	00 00 
   2bee0:	49 8b 44 24 f8       	mov    -0x8(%r12),%rax
   2bee5:	48 c1 e8 28          	shr    $0x28,%rax
   2bee9:	0f b6 c0             	movzbl %al,%eax
   2beec:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   2bef2:	75 08                	jne    2befc <Main.work+0x81c>
   2bef4:	49 8b 44 24 f0       	mov    -0x10(%r12),%rax
   2bef9:	48 8b 00             	mov    (%rax),%rax
   2befc:	48 85 c0             	test   %rax,%rax
   2beff:	74 0f                	je     2bf10 <Main.work+0x830>
   2bf01:	45 31 ff             	xor    %r15d,%r15d
   2bf04:	4d 85 e4             	test   %r12,%r12
   2bf07:	75 52                	jne    2bf5b <Main.work+0x87b>
   2bf09:	e9 92 00 00 00       	jmp    2bfa0 <Main.work+0x8c0>
   2bf0e:	66 90                	xchg   %ax,%ax
   2bf10:	49 8b 3c 24          	mov    (%r12),%rdi
   2bf14:	48 85 ff             	test   %rdi,%rdi
   2bf17:	74 27                	je     2bf40 <Main.work+0x860>
   2bf19:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2bf1d:	48 89 c1             	mov    %rax,%rcx
   2bf20:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2bf24:	48 85 c9             	test   %rcx,%rcx
   2bf27:	7e 09                	jle    2bf32 <Main.work+0x852>
   2bf29:	48 ff c0             	inc    %rax
   2bf2c:	48 89 47 f8          	mov    %rax,-0x8(%rdi)
   2bf30:	eb 0e                	jmp    2bf40 <Main.work+0x860>
   2bf32:	74 0c                	je     2bf40 <Main.work+0x860>
   2bf34:	49 89 ff             	mov    %rdi,%r15
   2bf37:	e8 34 e3 03 00       	call   6a270 <cinch_dup_mt>
   2bf3c:	4c 89 ff             	mov    %r15,%rdi
   2bf3f:	90                   	nop
   2bf40:	be 01 00 00 00       	mov    $0x1,%esi
   2bf45:	ba 01 00 00 00       	mov    $0x1,%edx
   2bf4a:	e8 51 88 ff ff       	call   247a0 <Data.String.string-length-from>
   2bf4f:	49 89 c7             	mov    %rax,%r15
   2bf52:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bf56:	4d 85 e4             	test   %r12,%r12
   2bf59:	74 45                	je     2bfa0 <Main.work+0x8c0>
   2bf5b:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2bf60:	48 89 c8             	mov    %rcx,%rax
   2bf63:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bf67:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bf6d:	7c 11                	jl     2bf80 <Main.work+0x8a0>
   2bf6f:	48 ff c9             	dec    %rcx
   2bf72:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2bf77:	eb 27                	jmp    2bfa0 <Main.work+0x8c0>
   2bf79:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2bf80:	48 85 c0             	test   %rax,%rax
   2bf83:	74 1b                	je     2bfa0 <Main.work+0x8c0>
   2bf85:	4c 89 e7             	mov    %r12,%rdi
   2bf88:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bf8e:	75 07                	jne    2bf97 <Main.work+0x8b7>
   2bf90:	e8 fb df 03 00       	call   69f90 <cinch_drop_slow>
   2bf95:	eb 05                	jmp    2bf9c <Main.work+0x8bc>
   2bf97:	e8 04 df 03 00       	call   69ea0 <cinch_drop_mt>
   2bf9c:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bfa0:	89 fb                	mov    %edi,%ebx
   2bfa2:	83 e3 01             	and    $0x1,%ebx
   2bfa5:	44 85 fb             	test   %r15d,%ebx
   2bfa8:	74 26                	je     2bfd0 <Main.work+0x8f0>
   2bfaa:	48 89 f9             	mov    %rdi,%rcx
   2bfad:	48 d1 e9             	shr    $1,%rcx
   2bfb0:	4c 89 f8             	mov    %r15,%rax
   2bfb3:	48 d1 e8             	shr    $1,%rax
   2bfb6:	48 01 c8             	add    %rcx,%rax
   2bfb9:	78 15                	js     2bfd0 <Main.work+0x8f0>
   2bfbb:	4c 8d 24 45 01 00 00 	lea    0x1(,%rax,2),%r12
   2bfc2:	00 
   2bfc3:	eb 1a                	jmp    2bfdf <Main.work+0x8ff>
   2bfc5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2bfcc:	00 00 00 00 
   2bfd0:	4c 89 fe             	mov    %r15,%rsi
   2bfd3:	e8 c8 32 04 00       	call   6f2a0 <cinch_nat_add>
   2bfd8:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2bfdc:	49 89 c4             	mov    %rax,%r12
   2bfdf:	48 85 ff             	test   %rdi,%rdi
   2bfe2:	0f 94 c0             	sete   %al
   2bfe5:	08 c3                	or     %al,%bl
   2bfe7:	75 47                	jne    2c030 <Main.work+0x950>
   2bfe9:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   2bfed:	48 89 c8             	mov    %rcx,%rax
   2bff0:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bff4:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bffa:	7c 14                	jl     2c010 <Main.work+0x930>
   2bffc:	48 ff c9             	dec    %rcx
   2bfff:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   2c003:	eb 2b                	jmp    2c030 <Main.work+0x950>
   2c005:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2c00c:	00 00 00 00 
   2c010:	48 85 c0             	test   %rax,%rax
   2c013:	74 1b                	je     2c030 <Main.work+0x950>
   2c015:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c01b:	75 07                	jne    2c024 <Main.work+0x944>
   2c01d:	e8 6e df 03 00       	call   69f90 <cinch_drop_slow>
   2c022:	eb 0c                	jmp    2c030 <Main.work+0x950>
   2c024:	e8 77 de 03 00       	call   69ea0 <cinch_drop_mt>
   2c029:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2c030:	4d 85 ff             	test   %r15,%r15
   2c033:	0f 94 c0             	sete   %al
   2c036:	44 89 f9             	mov    %r15d,%ecx
   2c039:	08 c1                	or     %al,%cl
   2c03b:	f6 c1 01             	test   $0x1,%cl
   2c03e:	74 40                	je     2c080 <Main.work+0x9a0>
   2c040:	41 f6 c4 01          	test   $0x1,%r12b
   2c044:	75 5e                	jne    2c0a4 <Main.work+0x9c4>
   2c046:	be 0f 94 35 77       	mov    $0x7735940f,%esi
   2c04b:	4c 89 e7             	mov    %r12,%rdi
   2c04e:	e8 fd 3d 04 00       	call   6fe50 <cinch_nat_rem>
   2c053:	4d 85 e4             	test   %r12,%r12
   2c056:	74 74                	je     2c0cc <Main.work+0x9ec>
   2c058:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2c05d:	48 89 ca             	mov    %rcx,%rdx
   2c060:	48 c1 e2 1d          	shl    $0x1d,%rdx
   2c064:	48 81 fa 01 00 00 20 	cmp    $0x20000001,%rdx
   2c06b:	0f 8c 7a 01 00 00    	jl     2c1eb <Main.work+0xb0b>
   2c071:	48 ff c9             	dec    %rcx
   2c074:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2c079:	eb 51                	jmp    2c0cc <Main.work+0x9ec>
   2c07b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   2c080:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2c084:	48 89 c8             	mov    %rcx,%rax
   2c087:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c08b:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c091:	0f 8c 99 00 00 00    	jl     2c130 <Main.work+0xa50>
   2c097:	48 ff c9             	dec    %rcx
   2c09a:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2c09e:	41 f6 c4 01          	test   $0x1,%r12b
   2c0a2:	74 a2                	je     2c046 <Main.work+0x966>
   2c0a4:	49 d1 ec             	shr    $1,%r12
   2c0a7:	4c 89 e0             	mov    %r12,%rax
   2c0aa:	48 b9 e5 8f a2 12 31 	movabs $0x89705f3112a28fe5,%rcx
   2c0b1:	5f 70 89 
   2c0b4:	48 f7 e1             	mul    %rcx
   2c0b7:	48 c1 ea 1d          	shr    $0x1d,%rdx
   2c0bb:	69 c2 07 ca 9a 3b    	imul   $0x3b9aca07,%edx,%eax
   2c0c1:	41 29 c4             	sub    %eax,%r12d
   2c0c4:	4a 8d 04 65 01 00 00 	lea    0x1(,%r12,2),%rax
   2c0cb:	00 
   2c0cc:	41 f6 c6 01          	test   $0x1,%r14b
   2c0d0:	0f 94 c2             	sete   %dl
   2c0d3:	49 83 fe 02          	cmp    $0x2,%r14
   2c0d7:	0f 92 c1             	setb   %cl
   2c0da:	08 d1                	or     %dl,%cl
   2c0dc:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   2c0e0:	74 1e                	je     2c100 <Main.work+0xa20>
   2c0e2:	be 03 00 00 00       	mov    $0x3,%esi
   2c0e7:	4c 89 f7             	mov    %r14,%rdi
   2c0ea:	e8 a1 3f 04 00       	call   70090 <cinch_nat_sub>
   2c0ef:	49 89 c7             	mov    %rax,%r15
   2c0f2:	f6 45 b8 01          	testb  $0x1,-0x48(%rbp)
   2c0f6:	74 12                	je     2c10a <Main.work+0xa2a>
   2c0f8:	e9 83 00 00 00       	jmp    2c180 <Main.work+0xaa0>
   2c0fd:	0f 1f 00             	nopl   (%rax)
   2c100:	4d 8d 7e fe          	lea    -0x2(%r14),%r15
   2c104:	f6 45 b8 01          	testb  $0x1,-0x48(%rbp)
   2c108:	75 76                	jne    2c180 <Main.work+0xaa0>
   2c10a:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2c10e:	48 89 c8             	mov    %rcx,%rax
   2c111:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c115:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c11b:	7c 43                	jl     2c160 <Main.work+0xa80>
   2c11d:	48 ff c9             	dec    %rcx
   2c120:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2c124:	41 f6 c5 01          	test   $0x1,%r13b
   2c128:	0f 85 e2 f5 ff ff    	jne    2b710 <Main.work+0x30>
   2c12e:	eb 5a                	jmp    2c18a <Main.work+0xaaa>
   2c130:	48 85 c0             	test   %rax,%rax
   2c133:	0f 84 07 ff ff ff    	je     2c040 <Main.work+0x960>
   2c139:	4c 89 ff             	mov    %r15,%rdi
   2c13c:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c142:	0f 85 c2 00 00 00    	jne    2c20a <Main.work+0xb2a>
   2c148:	e8 43 de 03 00       	call   69f90 <cinch_drop_slow>
   2c14d:	e9 ee fe ff ff       	jmp    2c040 <Main.work+0x960>
   2c152:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2c159:	1f 84 00 00 00 00 00 
   2c160:	48 85 c0             	test   %rax,%rax
   2c163:	74 1b                	je     2c180 <Main.work+0xaa0>
   2c165:	4c 89 f7             	mov    %r14,%rdi
   2c168:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c16e:	0f 85 aa 00 00 00    	jne    2c21e <Main.work+0xb3e>
   2c174:	e8 17 de 03 00       	call   69f90 <cinch_drop_slow>
   2c179:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2c180:	41 f6 c5 01          	test   $0x1,%r13b
   2c184:	0f 85 86 f5 ff ff    	jne    2b710 <Main.work+0x30>
   2c18a:	be 03 00 00 00       	mov    $0x3,%esi
   2c18f:	4c 89 ef             	mov    %r13,%rdi
   2c192:	e8 09 31 04 00       	call   6f2a0 <cinch_nat_add>
   2c197:	49 89 c4             	mov    %rax,%r12
   2c19a:	4d 85 ed             	test   %r13,%r13
   2c19d:	0f 94 c0             	sete   %al
   2c1a0:	44 89 e9             	mov    %r13d,%ecx
   2c1a3:	08 c1                	or     %al,%cl
   2c1a5:	f6 c1 01             	test   $0x1,%cl
   2c1a8:	0f 85 79 f5 ff ff    	jne    2b727 <Main.work+0x47>
   2c1ae:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2c1b2:	48 89 c8             	mov    %rcx,%rax
   2c1b5:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c1b9:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c1bf:	7c 0c                	jl     2c1cd <Main.work+0xaed>
   2c1c1:	48 ff c9             	dec    %rcx
   2c1c4:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2c1c8:	e9 5a f5 ff ff       	jmp    2b727 <Main.work+0x47>
   2c1cd:	48 85 c0             	test   %rax,%rax
   2c1d0:	0f 84 51 f5 ff ff    	je     2b727 <Main.work+0x47>
   2c1d6:	4c 89 ef             	mov    %r13,%rdi
   2c1d9:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c1df:	75 51                	jne    2c232 <Main.work+0xb52>
   2c1e1:	e8 aa dd 03 00       	call   69f90 <cinch_drop_slow>
   2c1e6:	e9 3c f5 ff ff       	jmp    2b727 <Main.work+0x47>
   2c1eb:	48 85 d2             	test   %rdx,%rdx
   2c1ee:	0f 84 d8 fe ff ff    	je     2c0cc <Main.work+0x9ec>
   2c1f4:	48 89 c3             	mov    %rax,%rbx
   2c1f7:	4c 89 e7             	mov    %r12,%rdi
   2c1fa:	48 81 fa 00 00 00 20 	cmp    $0x20000000,%rdx
   2c201:	75 39                	jne    2c23c <Main.work+0xb5c>
   2c203:	e8 88 dd 03 00       	call   69f90 <cinch_drop_slow>
   2c208:	eb 37                	jmp    2c241 <Main.work+0xb61>
   2c20a:	e8 91 dc 03 00       	call   69ea0 <cinch_drop_mt>
   2c20f:	41 f6 c4 01          	test   $0x1,%r12b
   2c213:	0f 84 2d fe ff ff    	je     2c046 <Main.work+0x966>
   2c219:	e9 86 fe ff ff       	jmp    2c0a4 <Main.work+0x9c4>
   2c21e:	e8 7d dc 03 00       	call   69ea0 <cinch_drop_mt>
   2c223:	41 f6 c5 01          	test   $0x1,%r13b
   2c227:	0f 84 5d ff ff ff    	je     2c18a <Main.work+0xaaa>
   2c22d:	e9 de f4 ff ff       	jmp    2b710 <Main.work+0x30>
   2c232:	e8 69 dc 03 00       	call   69ea0 <cinch_drop_mt>
   2c237:	e9 eb f4 ff ff       	jmp    2b727 <Main.work+0x47>
   2c23c:	e8 5f dc 03 00       	call   69ea0 <cinch_drop_mt>
   2c241:	48 89 d8             	mov    %rbx,%rax
   2c244:	e9 83 fe ff ff       	jmp    2c0cc <Main.work+0x9ec>
   2c249:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   2c24d:	48 85 d2             	test   %rdx,%rdx
   2c250:	0f 84 97 00 00 00    	je     2c2ed <Main.work+0xc0d>
   2c256:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c25a:	48 89 c8             	mov    %rcx,%rax
   2c25d:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c261:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c267:	7c 3c                	jl     2c2a5 <Main.work+0xbc5>
   2c269:	48 ff c9             	dec    %rcx
   2c26c:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c270:	eb 7b                	jmp    2c2ed <Main.work+0xc0d>
   2c272:	31 ff                	xor    %edi,%edi
   2c274:	e9 3d 02 00 00       	jmp    2c4b6 <Main.work+0xdd6>
   2c279:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   2c27d:	48 85 d2             	test   %rdx,%rdx
   2c280:	0f 84 09 01 00 00    	je     2c38f <Main.work+0xcaf>
   2c286:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c28a:	48 89 c8             	mov    %rcx,%rax
   2c28d:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c291:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c297:	7c 24                	jl     2c2bd <Main.work+0xbdd>
   2c299:	48 ff c9             	dec    %rcx
   2c29c:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c2a0:	e9 ea 00 00 00       	jmp    2c38f <Main.work+0xcaf>
   2c2a5:	48 85 c0             	test   %rax,%rax
   2c2a8:	74 43                	je     2c2ed <Main.work+0xc0d>
   2c2aa:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c2b0:	75 2e                	jne    2c2e0 <Main.work+0xc00>
   2c2b2:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c2b6:	e8 d5 dc 03 00       	call   69f90 <cinch_drop_slow>
   2c2bb:	eb 2c                	jmp    2c2e9 <Main.work+0xc09>
   2c2bd:	48 85 c0             	test   %rax,%rax
   2c2c0:	0f 84 c9 00 00 00    	je     2c38f <Main.work+0xcaf>
   2c2c6:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c2cc:	0f 85 b0 00 00 00    	jne    2c382 <Main.work+0xca2>
   2c2d2:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c2d6:	e8 b5 dc 03 00       	call   69f90 <cinch_drop_slow>
   2c2db:	e9 ab 00 00 00       	jmp    2c38b <Main.work+0xcab>
   2c2e0:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c2e4:	e8 b7 db 03 00       	call   69ea0 <cinch_drop_mt>
   2c2e9:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c2ed:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   2c2f1:	48 85 d2             	test   %rdx,%rdx
   2c2f4:	74 41                	je     2c337 <Main.work+0xc57>
   2c2f6:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c2fa:	48 89 c8             	mov    %rcx,%rax
   2c2fd:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c301:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c307:	7c 09                	jl     2c312 <Main.work+0xc32>
   2c309:	48 ff c9             	dec    %rcx
   2c30c:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c310:	eb 25                	jmp    2c337 <Main.work+0xc57>
   2c312:	48 85 c0             	test   %rax,%rax
   2c315:	74 20                	je     2c337 <Main.work+0xc57>
   2c317:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c31d:	75 0b                	jne    2c32a <Main.work+0xc4a>
   2c31f:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c323:	e8 68 dc 03 00       	call   69f90 <cinch_drop_slow>
   2c328:	eb 09                	jmp    2c333 <Main.work+0xc53>
   2c32a:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c32e:	e8 6d db 03 00       	call   69ea0 <cinch_drop_mt>
   2c333:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c337:	41 f6 c5 01          	test   $0x1,%r13b
   2c33b:	0f 85 de 00 00 00    	jne    2c41f <Main.work+0xd3f>
   2c341:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2c345:	48 89 c8             	mov    %rcx,%rax
   2c348:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c34c:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c352:	7c 0c                	jl     2c360 <Main.work+0xc80>
   2c354:	48 ff c9             	dec    %rcx
   2c357:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2c35b:	e9 bf 00 00 00       	jmp    2c41f <Main.work+0xd3f>
   2c360:	48 85 c0             	test   %rax,%rax
   2c363:	0f 84 b6 00 00 00    	je     2c41f <Main.work+0xd3f>
   2c369:	4c 89 ff             	mov    %r15,%rdi
   2c36c:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c372:	0f 85 9e 00 00 00    	jne    2c416 <Main.work+0xd36>
   2c378:	e8 13 dc 03 00       	call   69f90 <cinch_drop_slow>
   2c37d:	e9 99 00 00 00       	jmp    2c41b <Main.work+0xd3b>
   2c382:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c386:	e8 15 db 03 00       	call   69ea0 <cinch_drop_mt>
   2c38b:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c38f:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   2c393:	48 85 d2             	test   %rdx,%rdx
   2c396:	74 41                	je     2c3d9 <Main.work+0xcf9>
   2c398:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c39c:	48 89 c8             	mov    %rcx,%rax
   2c39f:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c3a3:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c3a9:	7c 09                	jl     2c3b4 <Main.work+0xcd4>
   2c3ab:	48 ff c9             	dec    %rcx
   2c3ae:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c3b2:	eb 25                	jmp    2c3d9 <Main.work+0xcf9>
   2c3b4:	48 85 c0             	test   %rax,%rax
   2c3b7:	74 20                	je     2c3d9 <Main.work+0xcf9>
   2c3b9:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c3bf:	75 0b                	jne    2c3cc <Main.work+0xcec>
   2c3c1:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c3c5:	e8 c6 db 03 00       	call   69f90 <cinch_drop_slow>
   2c3ca:	eb 09                	jmp    2c3d5 <Main.work+0xcf5>
   2c3cc:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c3d0:	e8 cb da 03 00       	call   69ea0 <cinch_drop_mt>
   2c3d5:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c3d9:	41 f6 c7 01          	test   $0x1,%r15b
   2c3dd:	0f 85 8e 00 00 00    	jne    2c471 <Main.work+0xd91>
   2c3e3:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2c3e7:	48 89 c8             	mov    %rcx,%rax
   2c3ea:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c3ee:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c3f4:	7c 09                	jl     2c3ff <Main.work+0xd1f>
   2c3f6:	48 ff c9             	dec    %rcx
   2c3f9:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2c3fd:	eb 72                	jmp    2c471 <Main.work+0xd91>
   2c3ff:	48 85 c0             	test   %rax,%rax
   2c402:	74 6d                	je     2c471 <Main.work+0xd91>
   2c404:	4c 89 f7             	mov    %r14,%rdi
   2c407:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c40d:	75 59                	jne    2c468 <Main.work+0xd88>
   2c40f:	e8 7c db 03 00       	call   69f90 <cinch_drop_slow>
   2c414:	eb 57                	jmp    2c46d <Main.work+0xd8d>
   2c416:	e8 85 da 03 00       	call   69ea0 <cinch_drop_mt>
   2c41b:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c41f:	4d 85 e4             	test   %r12,%r12
   2c422:	0f 94 c0             	sete   %al
   2c425:	44 89 e1             	mov    %r12d,%ecx
   2c428:	08 c1                	or     %al,%cl
   2c42a:	f6 c1 01             	test   $0x1,%cl
   2c42d:	0f 85 83 00 00 00    	jne    2c4b6 <Main.work+0xdd6>
   2c433:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2c438:	48 89 c8             	mov    %rcx,%rax
   2c43b:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c43f:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c445:	7c 0a                	jl     2c451 <Main.work+0xd71>
   2c447:	48 ff c9             	dec    %rcx
   2c44a:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2c44f:	eb 65                	jmp    2c4b6 <Main.work+0xdd6>
   2c451:	48 85 c0             	test   %rax,%rax
   2c454:	74 60                	je     2c4b6 <Main.work+0xdd6>
   2c456:	4c 89 e7             	mov    %r12,%rdi
   2c459:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c45f:	75 4c                	jne    2c4ad <Main.work+0xdcd>
   2c461:	e8 2a db 03 00       	call   69f90 <cinch_drop_slow>
   2c466:	eb 4a                	jmp    2c4b2 <Main.work+0xdd2>
   2c468:	e8 33 da 03 00       	call   69ea0 <cinch_drop_mt>
   2c46d:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c471:	4d 85 ed             	test   %r13,%r13
   2c474:	0f 94 c0             	sete   %al
   2c477:	44 89 e9             	mov    %r13d,%ecx
   2c47a:	08 c1                	or     %al,%cl
   2c47c:	f6 c1 01             	test   $0x1,%cl
   2c47f:	75 35                	jne    2c4b6 <Main.work+0xdd6>
   2c481:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2c485:	48 89 c8             	mov    %rcx,%rax
   2c488:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c48c:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c492:	7c 09                	jl     2c49d <Main.work+0xdbd>
   2c494:	48 ff c9             	dec    %rcx
   2c497:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2c49b:	eb 19                	jmp    2c4b6 <Main.work+0xdd6>
   2c49d:	48 85 c0             	test   %rax,%rax
   2c4a0:	74 14                	je     2c4b6 <Main.work+0xdd6>
   2c4a2:	4c 89 ef             	mov    %r13,%rdi
   2c4a5:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c4ab:	74 b4                	je     2c461 <Main.work+0xd81>
   2c4ad:	e8 ee d9 03 00       	call   69ea0 <cinch_drop_mt>
   2c4b2:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2c4b6:	48 89 f8             	mov    %rdi,%rax
   2c4b9:	48 83 c4 28          	add    $0x28,%rsp
   2c4bd:	5b                   	pop    %rbx
   2c4be:	41 5c                	pop    %r12
   2c4c0:	41 5d                	pop    %r13
   2c4c2:	41 5e                	pop    %r14
   2c4c4:	41 5f                	pop    %r15
   2c4c6:	5d                   	pop    %rbp
   2c4c7:	c3                   	ret

Disassembly of section .fini:
