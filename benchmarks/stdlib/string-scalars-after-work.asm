
/workspace/dasl-stdlib-strings/build/string-scalars-benchmark-phase1/after/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000002b800 <Main.work>:
   2b800:	55                   	push   %rbp
   2b801:	48 89 e5             	mov    %rsp,%rbp
   2b804:	41 57                	push   %r15
   2b806:	41 56                	push   %r14
   2b808:	41 55                	push   %r13
   2b80a:	41 54                	push   %r12
   2b80c:	53                   	push   %rbx
   2b80d:	48 83 ec 28          	sub    $0x28,%rsp
   2b811:	4c 89 45 c8          	mov    %r8,-0x38(%rbp)
   2b815:	49 89 cc             	mov    %rcx,%r12
   2b818:	49 89 d5             	mov    %rdx,%r13
   2b81b:	48 89 75 b8          	mov    %rsi,-0x48(%rbp)
   2b81f:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   2b823:	eb 22                	jmp    2b847 <Main.work+0x47>
   2b825:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2b82c:	00 00 00 00 
   2b830:	4c 89 f8             	mov    %r15,%rax
   2b833:	48 d1 e8             	shr    $1,%rax
   2b836:	48 ff c0             	inc    %rax
   2b839:	0f 88 7b 0c 00 00    	js     2c4ba <Main.work+0xcba>
   2b83f:	4c 8d 24 45 01 00 00 	lea    0x1(,%rax,2),%r12
   2b846:	00 
   2b847:	4d 85 ed             	test   %r13,%r13
   2b84a:	0f 94 c0             	sete   %al
   2b84d:	44 89 e9             	mov    %r13d,%ecx
   2b850:	08 c1                	or     %al,%cl
   2b852:	88 4d d7             	mov    %cl,-0x29(%rbp)
   2b855:	f6 c1 01             	test   $0x1,%cl
   2b858:	74 26                	je     2b880 <Main.work+0x80>
   2b85a:	41 f6 c5 01          	test   $0x1,%r13b
   2b85e:	75 40                	jne    2b8a0 <Main.work+0xa0>
   2b860:	be 01 00 00 00       	mov    $0x1,%esi
   2b865:	4c 89 ef             	mov    %r13,%rdi
   2b868:	e8 03 2c 04 00       	call   6e470 <cinch_nat_cmp>
   2b86d:	4c 8b 7d b8          	mov    -0x48(%rbp),%r15
   2b871:	e9 aa 00 00 00       	jmp    2b920 <Main.work+0x120>
   2b876:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2b87d:	00 00 00 
   2b880:	49 8b 45 f8          	mov    -0x8(%r13),%rax
   2b884:	48 89 c1             	mov    %rax,%rcx
   2b887:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b88b:	48 85 c9             	test   %rcx,%rcx
   2b88e:	7e 28                	jle    2b8b8 <Main.work+0xb8>
   2b890:	48 ff c0             	inc    %rax
   2b893:	49 89 45 f8          	mov    %rax,-0x8(%r13)
   2b897:	eb 29                	jmp    2b8c2 <Main.work+0xc2>
   2b899:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2b8a0:	4c 89 e8             	mov    %r13,%rax
   2b8a3:	48 d1 e8             	shr    $1,%rax
   2b8a6:	48 85 c0             	test   %rax,%rax
   2b8a9:	0f 97 c0             	seta   %al
   2b8ac:	1c 00                	sbb    $0x0,%al
   2b8ae:	48 0f be c0          	movsbq %al,%rax
   2b8b2:	4c 8b 7d b8          	mov    -0x48(%rbp),%r15
   2b8b6:	eb 68                	jmp    2b920 <Main.work+0x120>
   2b8b8:	74 08                	je     2b8c2 <Main.work+0xc2>
   2b8ba:	4c 89 ef             	mov    %r13,%rdi
   2b8bd:	e8 8e d9 03 00       	call   69250 <cinch_dup_mt>
   2b8c2:	be 01 00 00 00       	mov    $0x1,%esi
   2b8c7:	4c 89 ef             	mov    %r13,%rdi
   2b8ca:	e8 a1 2b 04 00       	call   6e470 <cinch_nat_cmp>
   2b8cf:	49 8b 55 f8          	mov    -0x8(%r13),%rdx
   2b8d3:	48 89 d1             	mov    %rdx,%rcx
   2b8d6:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b8da:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2b8e1:	4c 8b 7d b8          	mov    -0x48(%rbp),%r15
   2b8e5:	7c 09                	jl     2b8f0 <Main.work+0xf0>
   2b8e7:	48 ff ca             	dec    %rdx
   2b8ea:	49 89 55 f8          	mov    %rdx,-0x8(%r13)
   2b8ee:	eb 30                	jmp    2b920 <Main.work+0x120>
   2b8f0:	48 85 c9             	test   %rcx,%rcx
   2b8f3:	74 2b                	je     2b920 <Main.work+0x120>
   2b8f5:	48 89 c3             	mov    %rax,%rbx
   2b8f8:	4c 89 ef             	mov    %r13,%rdi
   2b8fb:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2b902:	75 07                	jne    2b90b <Main.work+0x10b>
   2b904:	e8 67 d6 03 00       	call   68f70 <cinch_drop_slow>
   2b909:	eb 05                	jmp    2b910 <Main.work+0x110>
   2b90b:	e8 70 d5 03 00       	call   68e80 <cinch_drop_mt>
   2b910:	48 89 d8             	mov    %rbx,%rax
   2b913:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b91a:	84 00 00 00 00 00 
   2b920:	48 85 c0             	test   %rax,%rax
   2b923:	0f 84 4c 0c 00 00    	je     2c575 <Main.work+0xd75>
   2b929:	41 f6 c4 01          	test   $0x1,%r12b
   2b92d:	75 31                	jne    2b960 <Main.work+0x160>
   2b92f:	be 05 00 00 00       	mov    $0x5,%esi
   2b934:	4c 89 e7             	mov    %r12,%rdi
   2b937:	e8 f4 34 04 00       	call   6ee30 <cinch_nat_rem>
   2b93c:	48 89 c3             	mov    %rax,%rbx
   2b93f:	f6 c3 01             	test   $0x1,%bl
   2b942:	74 27                	je     2b96b <Main.work+0x16b>
   2b944:	48 d1 eb             	shr    $1,%rbx
   2b947:	48 85 db             	test   %rbx,%rbx
   2b94a:	0f 97 c0             	seta   %al
   2b94d:	1c 00                	sbb    $0x0,%al
   2b94f:	48 0f be c0          	movsbq %al,%rax
   2b953:	eb 6b                	jmp    2b9c0 <Main.work+0x1c0>
   2b955:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2b95c:	00 00 00 00 
   2b960:	44 89 e3             	mov    %r12d,%ebx
   2b963:	83 e3 03             	and    $0x3,%ebx
   2b966:	f6 c3 01             	test   $0x1,%bl
   2b969:	75 d9                	jne    2b944 <Main.work+0x144>
   2b96b:	be 01 00 00 00       	mov    $0x1,%esi
   2b970:	48 89 df             	mov    %rbx,%rdi
   2b973:	e8 f8 2a 04 00       	call   6e470 <cinch_nat_cmp>
   2b978:	48 85 db             	test   %rbx,%rbx
   2b97b:	74 43                	je     2b9c0 <Main.work+0x1c0>
   2b97d:	48 8b 53 f8          	mov    -0x8(%rbx),%rdx
   2b981:	48 89 d1             	mov    %rdx,%rcx
   2b984:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b988:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2b98f:	7c 09                	jl     2b99a <Main.work+0x19a>
   2b991:	48 ff ca             	dec    %rdx
   2b994:	48 89 53 f8          	mov    %rdx,-0x8(%rbx)
   2b998:	eb 26                	jmp    2b9c0 <Main.work+0x1c0>
   2b99a:	48 85 c9             	test   %rcx,%rcx
   2b99d:	74 21                	je     2b9c0 <Main.work+0x1c0>
   2b99f:	49 89 c6             	mov    %rax,%r14
   2b9a2:	48 89 df             	mov    %rbx,%rdi
   2b9a5:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2b9ac:	75 07                	jne    2b9b5 <Main.work+0x1b5>
   2b9ae:	e8 bd d5 03 00       	call   68f70 <cinch_drop_slow>
   2b9b3:	eb 05                	jmp    2b9ba <Main.work+0x1ba>
   2b9b5:	e8 c6 d4 03 00       	call   68e80 <cinch_drop_mt>
   2b9ba:	4c 89 f0             	mov    %r14,%rax
   2b9bd:	0f 1f 00             	nopl   (%rax)
   2b9c0:	48 85 c0             	test   %rax,%rax
   2b9c3:	74 2b                	je     2b9f0 <Main.work+0x1f0>
   2b9c5:	4d 85 ff             	test   %r15,%r15
   2b9c8:	74 56                	je     2ba20 <Main.work+0x220>
   2b9ca:	49 8b 47 f8          	mov    -0x8(%r15),%rax
   2b9ce:	48 89 c1             	mov    %rax,%rcx
   2b9d1:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2b9d5:	48 85 c9             	test   %rcx,%rcx
   2b9d8:	7e 4a                	jle    2ba24 <Main.work+0x224>
   2b9da:	48 ff c0             	inc    %rax
   2b9dd:	49 89 47 f8          	mov    %rax,-0x8(%r15)
   2b9e1:	eb 4e                	jmp    2ba31 <Main.work+0x231>
   2b9e3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2b9ea:	84 00 00 00 00 00 
   2b9f0:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2b9f4:	48 85 ff             	test   %rdi,%rdi
   2b9f7:	74 27                	je     2ba20 <Main.work+0x220>
   2b9f9:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2b9fd:	48 89 c1             	mov    %rax,%rcx
   2ba00:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2ba04:	48 85 c9             	test   %rcx,%rcx
   2ba07:	0f 8e 19 05 00 00    	jle    2bf26 <Main.work+0x726>
   2ba0d:	48 ff c0             	inc    %rax
   2ba10:	48 89 47 f8          	mov    %rax,-0x8(%rdi)
   2ba14:	eb 1e                	jmp    2ba34 <Main.work+0x234>
   2ba16:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2ba1d:	00 00 00 
   2ba20:	31 ff                	xor    %edi,%edi
   2ba22:	eb 10                	jmp    2ba34 <Main.work+0x234>
   2ba24:	4c 89 ff             	mov    %r15,%rdi
   2ba27:	74 0b                	je     2ba34 <Main.work+0x234>
   2ba29:	4c 89 ff             	mov    %r15,%rdi
   2ba2c:	e8 1f d8 03 00       	call   69250 <cinch_dup_mt>
   2ba31:	4c 89 ff             	mov    %r15,%rdi
   2ba34:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2ba38:	48 c1 e8 28          	shr    $0x28,%rax
   2ba3c:	0f b6 c0             	movzbl %al,%eax
   2ba3f:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   2ba45:	75 07                	jne    2ba4e <Main.work+0x24e>
   2ba47:	48 8b 47 f0          	mov    -0x10(%rdi),%rax
   2ba4b:	48 8b 00             	mov    (%rax),%rax
   2ba4e:	48 85 c0             	test   %rax,%rax
   2ba51:	0f 85 47 0b 00 00    	jne    2c59e <Main.work+0xd9e>
   2ba57:	4c 8b 37             	mov    (%rdi),%r14
   2ba5a:	48 85 ff             	test   %rdi,%rdi
   2ba5d:	74 41                	je     2baa0 <Main.work+0x2a0>
   2ba5f:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   2ba63:	48 b9 ff ff ff ff 07 	movabs $0x7ffffffff,%rcx
   2ba6a:	00 00 00 
   2ba6d:	48 21 c8             	and    %rcx,%rax
   2ba70:	48 83 f8 01          	cmp    $0x1,%rax
   2ba74:	75 2a                	jne    2baa0 <Main.work+0x2a0>
   2ba76:	e8 05 eb 03 00       	call   6a580 <cinch_free>
   2ba7b:	4d 85 f6             	test   %r14,%r14
   2ba7e:	0f 85 a5 00 00 00    	jne    2bb29 <Main.work+0x329>
   2ba84:	31 ff                	xor    %edi,%edi
   2ba86:	e8 f5 c5 03 00       	call   68080 <cinch_bytes_len>
   2ba8b:	49 89 c7             	mov    %rax,%r15
   2ba8e:	48 b8 ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rax
   2ba95:	ff ff 7f 
   2ba98:	49 21 c7             	and    %rax,%r15
   2ba9b:	e9 70 01 00 00       	jmp    2bc10 <Main.work+0x410>
   2baa0:	4d 85 f6             	test   %r14,%r14
   2baa3:	74 2b                	je     2bad0 <Main.work+0x2d0>
   2baa5:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2baa9:	48 89 c1             	mov    %rax,%rcx
   2baac:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2bab0:	48 85 c9             	test   %rcx,%rcx
   2bab3:	7e 09                	jle    2babe <Main.work+0x2be>
   2bab5:	48 ff c0             	inc    %rax
   2bab8:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2babc:	eb 12                	jmp    2bad0 <Main.work+0x2d0>
   2babe:	74 10                	je     2bad0 <Main.work+0x2d0>
   2bac0:	48 89 fb             	mov    %rdi,%rbx
   2bac3:	4c 89 f7             	mov    %r14,%rdi
   2bac6:	e8 85 d7 03 00       	call   69250 <cinch_dup_mt>
   2bacb:	48 89 df             	mov    %rbx,%rdi
   2bace:	66 90                	xchg   %ax,%ax
   2bad0:	48 85 ff             	test   %rdi,%rdi
   2bad3:	74 4b                	je     2bb20 <Main.work+0x320>
   2bad5:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   2bad9:	48 89 c8             	mov    %rcx,%rax
   2badc:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bae0:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bae6:	7c 18                	jl     2bb00 <Main.work+0x300>
   2bae8:	48 ff c9             	dec    %rcx
   2baeb:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   2baef:	4d 85 f6             	test   %r14,%r14
   2baf2:	75 35                	jne    2bb29 <Main.work+0x329>
   2baf4:	eb 8e                	jmp    2ba84 <Main.work+0x284>
   2baf6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2bafd:	00 00 00 
   2bb00:	48 85 c0             	test   %rax,%rax
   2bb03:	74 1b                	je     2bb20 <Main.work+0x320>
   2bb05:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bb0b:	0f 85 2f 04 00 00    	jne    2bf40 <Main.work+0x740>
   2bb11:	e8 5a d4 03 00       	call   68f70 <cinch_drop_slow>
   2bb16:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2bb1d:	00 00 00 
   2bb20:	4d 85 f6             	test   %r14,%r14
   2bb23:	0f 84 5b ff ff ff    	je     2ba84 <Main.work+0x284>
   2bb29:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2bb2d:	48 89 c1             	mov    %rax,%rcx
   2bb30:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2bb34:	48 85 c9             	test   %rcx,%rcx
   2bb37:	7e 17                	jle    2bb50 <Main.work+0x350>
   2bb39:	48 ff c0             	inc    %rax
   2bb3c:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2bb40:	eb 18                	jmp    2bb5a <Main.work+0x35a>
   2bb42:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bb49:	1f 84 00 00 00 00 00 
   2bb50:	74 08                	je     2bb5a <Main.work+0x35a>
   2bb52:	4c 89 f7             	mov    %r14,%rdi
   2bb55:	e8 f6 d6 03 00       	call   69250 <cinch_dup_mt>
   2bb5a:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2bb5e:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   2bb65:	00 00 00 
   2bb68:	48 21 c8             	and    %rcx,%rax
   2bb6b:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   2bb72:	00 00 00 
   2bb75:	48 39 c8             	cmp    %rcx,%rax
   2bb78:	75 06                	jne    2bb80 <Main.work+0x380>
   2bb7a:	4d 8b 3e             	mov    (%r14),%r15
   2bb7d:	eb 0c                	jmp    2bb8b <Main.work+0x38b>
   2bb7f:	90                   	nop
   2bb80:	4c 89 f7             	mov    %r14,%rdi
   2bb83:	e8 f8 c4 03 00       	call   68080 <cinch_bytes_len>
   2bb88:	49 89 c7             	mov    %rax,%r15
   2bb8b:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2bb8f:	48 89 c8             	mov    %rcx,%rax
   2bb92:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bb96:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bb9c:	7c 12                	jl     2bbb0 <Main.work+0x3b0>
   2bb9e:	48 ff c9             	dec    %rcx
   2bba1:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2bba5:	eb 29                	jmp    2bbd0 <Main.work+0x3d0>
   2bba7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   2bbae:	00 00 
   2bbb0:	48 85 c0             	test   %rax,%rax
   2bbb3:	74 1b                	je     2bbd0 <Main.work+0x3d0>
   2bbb5:	4c 89 f7             	mov    %r14,%rdi
   2bbb8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bbbe:	75 07                	jne    2bbc7 <Main.work+0x3c7>
   2bbc0:	e8 ab d3 03 00       	call   68f70 <cinch_drop_slow>
   2bbc5:	eb 09                	jmp    2bbd0 <Main.work+0x3d0>
   2bbc7:	e8 b4 d2 03 00       	call   68e80 <cinch_drop_mt>
   2bbcc:	0f 1f 40 00          	nopl   0x0(%rax)
   2bbd0:	48 b8 ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rax
   2bbd7:	ff ff 7f 
   2bbda:	49 21 c7             	and    %rax,%r15
   2bbdd:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2bbe1:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   2bbe8:	00 00 00 
   2bbeb:	48 21 c8             	and    %rcx,%rax
   2bbee:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   2bbf5:	00 00 00 
   2bbf8:	48 39 c8             	cmp    %rcx,%rax
   2bbfb:	75 13                	jne    2bc10 <Main.work+0x410>
   2bbfd:	49 83 3e 00          	cmpq   $0x0,(%r14)
   2bc01:	bf 01 00 00 00       	mov    $0x1,%edi
   2bc06:	74 13                	je     2bc1b <Main.work+0x41b>
   2bc08:	49 8b 7e 08          	mov    0x8(%r14),%rdi
   2bc0c:	eb 0d                	jmp    2bc1b <Main.work+0x41b>
   2bc0e:	66 90                	xchg   %ax,%ax
   2bc10:	4c 89 f7             	mov    %r14,%rdi
   2bc13:	e8 f8 c2 03 00       	call   67f10 <cinch_bytes_data>
   2bc18:	48 89 c7             	mov    %rax,%rdi
   2bc1b:	48 b8 01 01 01 01 01 	movabs $0x101010101010101,%rax
   2bc22:	01 01 01 
   2bc25:	49 83 ff 10          	cmp    $0x10,%r15
   2bc29:	73 25                	jae    2bc50 <Main.work+0x450>
   2bc2b:	49 83 ff 08          	cmp    $0x8,%r15
   2bc2f:	73 6f                	jae    2bca0 <Main.work+0x4a0>
   2bc31:	4d 85 ff             	test   %r15,%r15
   2bc34:	0f 84 a4 01 00 00    	je     2bdde <Main.work+0x5de>
   2bc3a:	31 c9                	xor    %ecx,%ecx
   2bc3c:	80 3f c0             	cmpb   $0xc0,(%rdi)
   2bc3f:	0f 9d c1             	setge  %cl
   2bc42:	be 01 00 00 00       	mov    $0x1,%esi
   2bc47:	eb 4e                	jmp    2bc97 <Main.work+0x497>
   2bc49:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2bc50:	f3 0f 6f 07          	movdqu (%rdi),%xmm0
   2bc54:	66 0f 6f 15 54 14 0f 	movdqa 0xf1454(%rip),%xmm2        # 11d0b0 <_IO_stdin_used+0xb0>
   2bc5b:	00 
   2bc5c:	66 0f 64 c2          	pcmpgtb %xmm2,%xmm0
   2bc60:	66 0f 6f 1d 58 14 0f 	movdqa 0xf1458(%rip),%xmm3        # 11d0c0 <_IO_stdin_used+0xc0>
   2bc67:	00 
   2bc68:	66 0f db c3          	pand   %xmm3,%xmm0
   2bc6c:	66 0f 70 c8 ee       	pshufd $0xee,%xmm0,%xmm1
   2bc71:	66 0f d4 c8          	paddq  %xmm0,%xmm1
   2bc75:	66 48 0f 7e c9       	movq   %xmm1,%rcx
   2bc7a:	48 0f af c8          	imul   %rax,%rcx
   2bc7e:	48 c1 e9 38          	shr    $0x38,%rcx
   2bc82:	49 83 ff 20          	cmp    $0x20,%r15
   2bc86:	73 68                	jae    2bcf0 <Main.work+0x4f0>
   2bc88:	49 83 ff 18          	cmp    $0x18,%r15
   2bc8c:	0f 83 99 00 00 00    	jae    2bd2b <Main.work+0x52b>
   2bc92:	be 10 00 00 00       	mov    $0x10,%esi
   2bc97:	4c 89 fa             	mov    %r15,%rdx
   2bc9a:	e9 c1 00 00 00       	jmp    2bd60 <Main.work+0x560>
   2bc9f:	90                   	nop
   2bca0:	48 8b 1f             	mov    (%rdi),%rbx
   2bca3:	48 89 c1             	mov    %rax,%rcx
   2bca6:	48 8d 04 1b          	lea    (%rbx,%rbx,1),%rax
   2bcaa:	48 f7 d3             	not    %rbx
   2bcad:	48 09 c3             	or     %rax,%rbx
   2bcb0:	48 c1 eb 07          	shr    $0x7,%rbx
   2bcb4:	48 21 cb             	and    %rcx,%rbx
   2bcb7:	48 0f af d9          	imul   %rcx,%rbx
   2bcbb:	48 c1 eb 38          	shr    $0x38,%rbx
   2bcbf:	49 83 ff 08          	cmp    $0x8,%r15
   2bcc3:	0f 84 9f 00 00 00    	je     2bd68 <Main.work+0x568>
   2bcc9:	31 c0                	xor    %eax,%eax
   2bccb:	80 7f 08 c0          	cmpb   $0xc0,0x8(%rdi)
   2bccf:	0f 9d c0             	setge  %al
   2bcd2:	48 01 c3             	add    %rax,%rbx
   2bcd5:	be 09 00 00 00       	mov    $0x9,%esi
   2bcda:	4c 89 fa             	mov    %r15,%rdx
   2bcdd:	48 89 d9             	mov    %rbx,%rcx
   2bce0:	eb 7e                	jmp    2bd60 <Main.work+0x560>
   2bce2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2bce9:	1f 84 00 00 00 00 00 
   2bcf0:	f3 0f 6f 47 10       	movdqu 0x10(%rdi),%xmm0
   2bcf5:	66 0f 64 c2          	pcmpgtb %xmm2,%xmm0
   2bcf9:	66 0f db c3          	pand   %xmm3,%xmm0
   2bcfd:	66 0f 70 c8 ee       	pshufd $0xee,%xmm0,%xmm1
   2bd02:	66 0f d4 c8          	paddq  %xmm0,%xmm1
   2bd06:	48 89 c2             	mov    %rax,%rdx
   2bd09:	66 48 0f 7e c8       	movq   %xmm1,%rax
   2bd0e:	48 0f af c2          	imul   %rdx,%rax
   2bd12:	48 c1 e8 38          	shr    $0x38,%rax
   2bd16:	48 01 c8             	add    %rcx,%rax
   2bd19:	be 20 00 00 00       	mov    $0x20,%esi
   2bd1e:	4c 89 fa             	mov    %r15,%rdx
   2bd21:	48 89 c1             	mov    %rax,%rcx
   2bd24:	e8 c7 c6 ff ff       	call   283f0 <Data.String.string-count-from>
   2bd29:	eb 3a                	jmp    2bd65 <Main.work+0x565>
   2bd2b:	48 8b 47 10          	mov    0x10(%rdi),%rax
   2bd2f:	48 8d 14 00          	lea    (%rax,%rax,1),%rdx
   2bd33:	48 f7 d0             	not    %rax
   2bd36:	48 09 d0             	or     %rdx,%rax
   2bd39:	48 c1 e8 07          	shr    $0x7,%rax
   2bd3d:	48 ba 01 01 01 01 01 	movabs $0x101010101010101,%rdx
   2bd44:	01 01 01 
   2bd47:	48 21 d0             	and    %rdx,%rax
   2bd4a:	48 0f af c2          	imul   %rdx,%rax
   2bd4e:	48 c1 e8 38          	shr    $0x38,%rax
   2bd52:	48 01 c8             	add    %rcx,%rax
   2bd55:	be 18 00 00 00       	mov    $0x18,%esi
   2bd5a:	4c 89 fa             	mov    %r15,%rdx
   2bd5d:	48 89 c1             	mov    %rax,%rcx
   2bd60:	e8 fb c8 ff ff       	call   28660 <Data.String.string-count-tail>
   2bd65:	48 89 c3             	mov    %rax,%rbx
   2bd68:	4d 85 f6             	test   %r14,%r14
   2bd6b:	0f 84 7f 00 00 00    	je     2bdf0 <Main.work+0x5f0>
   2bd71:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2bd75:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   2bd7c:	00 00 00 
   2bd7f:	48 21 c8             	and    %rcx,%rax
   2bd82:	48 b9 00 00 00 00 40 	movabs $0x4000000000,%rcx
   2bd89:	00 00 00 
   2bd8c:	48 39 c8             	cmp    %rcx,%rax
   2bd8f:	74 08                	je     2bd99 <Main.work+0x599>
   2bd91:	4c 89 f7             	mov    %r14,%rdi
   2bd94:	e8 e7 c2 03 00       	call   68080 <cinch_bytes_len>
   2bd99:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2bd9d:	48 89 c8             	mov    %rcx,%rax
   2bda0:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bda4:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bdaa:	7c 14                	jl     2bdc0 <Main.work+0x5c0>
   2bdac:	48 ff c9             	dec    %rcx
   2bdaf:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2bdb3:	eb 42                	jmp    2bdf7 <Main.work+0x5f7>
   2bdb5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2bdbc:	00 00 00 00 
   2bdc0:	48 85 c0             	test   %rax,%rax
   2bdc3:	74 32                	je     2bdf7 <Main.work+0x5f7>
   2bdc5:	4c 89 f7             	mov    %r14,%rdi
   2bdc8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bdce:	75 07                	jne    2bdd7 <Main.work+0x5d7>
   2bdd0:	e8 9b d1 03 00       	call   68f70 <cinch_drop_slow>
   2bdd5:	eb 20                	jmp    2bdf7 <Main.work+0x5f7>
   2bdd7:	e8 a4 d0 03 00       	call   68e80 <cinch_drop_mt>
   2bddc:	eb 19                	jmp    2bdf7 <Main.work+0x5f7>
   2bdde:	31 db                	xor    %ebx,%ebx
   2bde0:	4d 85 f6             	test   %r14,%r14
   2bde3:	75 8c                	jne    2bd71 <Main.work+0x571>
   2bde5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2bdec:	00 00 00 00 
   2bdf0:	31 ff                	xor    %edi,%edi
   2bdf2:	e8 89 c2 03 00       	call   68080 <cinch_bytes_len>
   2bdf7:	48 85 db             	test   %rbx,%rbx
   2bdfa:	48 8d 74 1b 01       	lea    0x1(%rbx,%rbx,1),%rsi
   2bdff:	b8 01 00 00 00       	mov    $0x1,%eax
   2be04:	48 0f 48 f0          	cmovs  %rax,%rsi
   2be08:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2be0c:	40 f6 c7 01          	test   $0x1,%dil
   2be10:	74 5e                	je     2be70 <Main.work+0x670>
   2be12:	48 8b 4d c8          	mov    -0x38(%rbp),%rcx
   2be16:	48 d1 e9             	shr    $1,%rcx
   2be19:	48 89 f0             	mov    %rsi,%rax
   2be1c:	48 d1 e8             	shr    $1,%rax
   2be1f:	48 01 c8             	add    %rcx,%rax
   2be22:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2be26:	78 48                	js     2be70 <Main.work+0x670>
   2be28:	48 8d 1c 45 01 00 00 	lea    0x1(,%rax,2),%rbx
   2be2f:	00 
   2be30:	f6 c3 01             	test   $0x1,%bl
   2be33:	74 5b                	je     2be90 <Main.work+0x690>
   2be35:	48 d1 eb             	shr    $1,%rbx
   2be38:	48 89 d8             	mov    %rbx,%rax
   2be3b:	48 b9 e5 8f a2 12 31 	movabs $0x89705f3112a28fe5,%rcx
   2be42:	5f 70 89 
   2be45:	48 f7 e1             	mul    %rcx
   2be48:	48 c1 ea 1d          	shr    $0x1d,%rdx
   2be4c:	69 c2 07 ca 9a 3b    	imul   $0x3b9aca07,%edx,%eax
   2be52:	29 c3                	sub    %eax,%ebx
   2be54:	48 8d 04 5d 01 00 00 	lea    0x1(,%rbx,2),%rax
   2be5b:	00 
   2be5c:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   2be60:	e9 0b 01 00 00       	jmp    2bf70 <Main.work+0x770>
   2be65:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2be6c:	00 00 00 00 
   2be70:	49 89 fe             	mov    %rdi,%r14
   2be73:	e8 08 24 04 00       	call   6e280 <cinch_nat_add>
   2be78:	48 89 c3             	mov    %rax,%rbx
   2be7b:	4d 85 f6             	test   %r14,%r14
   2be7e:	0f 94 c0             	sete   %al
   2be81:	44 89 f1             	mov    %r14d,%ecx
   2be84:	08 c1                	or     %al,%cl
   2be86:	f6 c1 01             	test   $0x1,%cl
   2be89:	74 45                	je     2bed0 <Main.work+0x6d0>
   2be8b:	f6 c3 01             	test   $0x1,%bl
   2be8e:	75 a5                	jne    2be35 <Main.work+0x635>
   2be90:	be 0f 94 35 77       	mov    $0x7735940f,%esi
   2be95:	48 89 df             	mov    %rbx,%rdi
   2be98:	e8 93 2f 04 00       	call   6ee30 <cinch_nat_rem>
   2be9d:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   2bea1:	48 85 db             	test   %rbx,%rbx
   2bea4:	0f 84 c6 00 00 00    	je     2bf70 <Main.work+0x770>
   2beaa:	48 8b 4b f8          	mov    -0x8(%rbx),%rcx
   2beae:	48 89 c8             	mov    %rcx,%rax
   2beb1:	48 c1 e0 1d          	shl    $0x1d,%rax
   2beb5:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bebb:	7c 52                	jl     2bf0f <Main.work+0x70f>
   2bebd:	48 ff c9             	dec    %rcx
   2bec0:	48 89 4b f8          	mov    %rcx,-0x8(%rbx)
   2bec4:	e9 a7 00 00 00       	jmp    2bf70 <Main.work+0x770>
   2bec9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2bed0:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2bed4:	48 89 c8             	mov    %rcx,%rax
   2bed7:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bedb:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bee1:	7c 11                	jl     2bef4 <Main.work+0x6f4>
   2bee3:	48 ff c9             	dec    %rcx
   2bee6:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2beea:	f6 c3 01             	test   $0x1,%bl
   2beed:	74 a1                	je     2be90 <Main.work+0x690>
   2beef:	e9 41 ff ff ff       	jmp    2be35 <Main.work+0x635>
   2bef4:	48 85 c0             	test   %rax,%rax
   2bef7:	74 92                	je     2be8b <Main.work+0x68b>
   2bef9:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2beff:	75 52                	jne    2bf53 <Main.work+0x753>
   2bf01:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2bf05:	e8 66 d0 03 00       	call   68f70 <cinch_drop_slow>
   2bf0a:	e9 7c ff ff ff       	jmp    2be8b <Main.work+0x68b>
   2bf0f:	48 85 c0             	test   %rax,%rax
   2bf12:	74 5c                	je     2bf70 <Main.work+0x770>
   2bf14:	48 89 df             	mov    %rbx,%rdi
   2bf17:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bf1d:	75 4b                	jne    2bf6a <Main.work+0x76a>
   2bf1f:	e8 4c d0 03 00       	call   68f70 <cinch_drop_slow>
   2bf24:	eb 4a                	jmp    2bf70 <Main.work+0x770>
   2bf26:	0f 84 08 fb ff ff    	je     2ba34 <Main.work+0x234>
   2bf2c:	48 8b 5d c0          	mov    -0x40(%rbp),%rbx
   2bf30:	48 89 df             	mov    %rbx,%rdi
   2bf33:	e8 18 d3 03 00       	call   69250 <cinch_dup_mt>
   2bf38:	48 89 df             	mov    %rbx,%rdi
   2bf3b:	e9 f4 fa ff ff       	jmp    2ba34 <Main.work+0x234>
   2bf40:	e8 3b cf 03 00       	call   68e80 <cinch_drop_mt>
   2bf45:	4d 85 f6             	test   %r14,%r14
   2bf48:	0f 85 db fb ff ff    	jne    2bb29 <Main.work+0x329>
   2bf4e:	e9 31 fb ff ff       	jmp    2ba84 <Main.work+0x284>
   2bf53:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2bf57:	e8 24 cf 03 00       	call   68e80 <cinch_drop_mt>
   2bf5c:	f6 c3 01             	test   $0x1,%bl
   2bf5f:	0f 84 2b ff ff ff    	je     2be90 <Main.work+0x690>
   2bf65:	e9 cb fe ff ff       	jmp    2be35 <Main.work+0x635>
   2bf6a:	e8 11 cf 03 00       	call   68e80 <cinch_drop_mt>
   2bf6f:	90                   	nop
   2bf70:	41 f6 c5 01          	test   $0x1,%r13b
   2bf74:	0f 94 c0             	sete   %al
   2bf77:	49 83 fd 02          	cmp    $0x2,%r13
   2bf7b:	0f 92 c1             	setb   %cl
   2bf7e:	08 c1                	or     %al,%cl
   2bf80:	74 1e                	je     2bfa0 <Main.work+0x7a0>
   2bf82:	be 03 00 00 00       	mov    $0x3,%esi
   2bf87:	4c 89 ef             	mov    %r13,%rdi
   2bf8a:	e8 e1 30 04 00       	call   6f070 <cinch_nat_sub>
   2bf8f:	49 89 c6             	mov    %rax,%r14
   2bf92:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2bf96:	74 12                	je     2bfaa <Main.work+0x7aa>
   2bf98:	eb 56                	jmp    2bff0 <Main.work+0x7f0>
   2bf9a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2bfa0:	4d 8d 75 fe          	lea    -0x2(%r13),%r14
   2bfa4:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2bfa8:	75 46                	jne    2bff0 <Main.work+0x7f0>
   2bfaa:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2bfae:	48 89 c8             	mov    %rcx,%rax
   2bfb1:	48 c1 e0 1d          	shl    $0x1d,%rax
   2bfb5:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2bfbb:	7c 13                	jl     2bfd0 <Main.work+0x7d0>
   2bfbd:	48 ff c9             	dec    %rcx
   2bfc0:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2bfc4:	41 f6 c4 01          	test   $0x1,%r12b
   2bfc8:	75 2c                	jne    2bff6 <Main.work+0x7f6>
   2bfca:	eb 54                	jmp    2c020 <Main.work+0x820>
   2bfcc:	0f 1f 40 00          	nopl   0x0(%rax)
   2bfd0:	48 85 c0             	test   %rax,%rax
   2bfd3:	74 1b                	je     2bff0 <Main.work+0x7f0>
   2bfd5:	4c 89 ef             	mov    %r13,%rdi
   2bfd8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2bfde:	75 2b                	jne    2c00b <Main.work+0x80b>
   2bfe0:	e8 8b cf 03 00       	call   68f70 <cinch_drop_slow>
   2bfe5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2bfec:	00 00 00 00 
   2bff0:	41 f6 c4 01          	test   $0x1,%r12b
   2bff4:	74 2a                	je     2c020 <Main.work+0x820>
   2bff6:	4c 89 e0             	mov    %r12,%rax
   2bff9:	48 d1 e8             	shr    $1,%rax
   2bffc:	48 ff c0             	inc    %rax
   2bfff:	78 1f                	js     2c020 <Main.work+0x820>
   2c001:	4c 8d 3c 45 01 00 00 	lea    0x1(,%rax,2),%r15
   2c008:	00 
   2c009:	eb 75                	jmp    2c080 <Main.work+0x880>
   2c00b:	e8 70 ce 03 00       	call   68e80 <cinch_drop_mt>
   2c010:	41 f6 c4 01          	test   $0x1,%r12b
   2c014:	75 e0                	jne    2bff6 <Main.work+0x7f6>
   2c016:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2c01d:	00 00 00 
   2c020:	be 03 00 00 00       	mov    $0x3,%esi
   2c025:	4c 89 e7             	mov    %r12,%rdi
   2c028:	e8 53 22 04 00       	call   6e280 <cinch_nat_add>
   2c02d:	49 89 c7             	mov    %rax,%r15
   2c030:	4d 85 e4             	test   %r12,%r12
   2c033:	0f 94 c0             	sete   %al
   2c036:	44 89 e1             	mov    %r12d,%ecx
   2c039:	08 c1                	or     %al,%cl
   2c03b:	f6 c1 01             	test   $0x1,%cl
   2c03e:	75 40                	jne    2c080 <Main.work+0x880>
   2c040:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2c045:	48 89 c8             	mov    %rcx,%rax
   2c048:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c04c:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c052:	7c 0a                	jl     2c05e <Main.work+0x85e>
   2c054:	48 ff c9             	dec    %rcx
   2c057:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2c05c:	eb 22                	jmp    2c080 <Main.work+0x880>
   2c05e:	48 85 c0             	test   %rax,%rax
   2c061:	74 1d                	je     2c080 <Main.work+0x880>
   2c063:	4c 89 e7             	mov    %r12,%rdi
   2c066:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c06c:	75 07                	jne    2c075 <Main.work+0x875>
   2c06e:	e8 fd ce 03 00       	call   68f70 <cinch_drop_slow>
   2c073:	eb 0b                	jmp    2c080 <Main.work+0x880>
   2c075:	e8 06 ce 03 00       	call   68e80 <cinch_drop_mt>
   2c07a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2c080:	4d 85 f6             	test   %r14,%r14
   2c083:	0f 94 c0             	sete   %al
   2c086:	44 89 f1             	mov    %r14d,%ecx
   2c089:	08 c1                	or     %al,%cl
   2c08b:	88 4d d7             	mov    %cl,-0x29(%rbp)
   2c08e:	f6 c1 01             	test   $0x1,%cl
   2c091:	74 1d                	je     2c0b0 <Main.work+0x8b0>
   2c093:	41 f6 c6 01          	test   $0x1,%r14b
   2c097:	75 37                	jne    2c0d0 <Main.work+0x8d0>
   2c099:	be 01 00 00 00       	mov    $0x1,%esi
   2c09e:	4c 89 f7             	mov    %r14,%rdi
   2c0a1:	e8 ca 23 04 00       	call   6e470 <cinch_nat_cmp>
   2c0a6:	e9 a5 00 00 00       	jmp    2c150 <Main.work+0x950>
   2c0ab:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   2c0b0:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   2c0b4:	48 89 c1             	mov    %rax,%rcx
   2c0b7:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2c0bb:	48 85 c9             	test   %rcx,%rcx
   2c0be:	7e 24                	jle    2c0e4 <Main.work+0x8e4>
   2c0c0:	48 ff c0             	inc    %rax
   2c0c3:	49 89 46 f8          	mov    %rax,-0x8(%r14)
   2c0c7:	eb 25                	jmp    2c0ee <Main.work+0x8ee>
   2c0c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2c0d0:	4c 89 f0             	mov    %r14,%rax
   2c0d3:	48 d1 e8             	shr    $1,%rax
   2c0d6:	48 85 c0             	test   %rax,%rax
   2c0d9:	0f 97 c0             	seta   %al
   2c0dc:	1c 00                	sbb    $0x0,%al
   2c0de:	48 0f be c0          	movsbq %al,%rax
   2c0e2:	eb 6c                	jmp    2c150 <Main.work+0x950>
   2c0e4:	74 08                	je     2c0ee <Main.work+0x8ee>
   2c0e6:	4c 89 f7             	mov    %r14,%rdi
   2c0e9:	e8 62 d1 03 00       	call   69250 <cinch_dup_mt>
   2c0ee:	be 01 00 00 00       	mov    $0x1,%esi
   2c0f3:	4c 89 f7             	mov    %r14,%rdi
   2c0f6:	e8 75 23 04 00       	call   6e470 <cinch_nat_cmp>
   2c0fb:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   2c0ff:	48 89 d1             	mov    %rdx,%rcx
   2c102:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2c106:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2c10d:	7c 11                	jl     2c120 <Main.work+0x920>
   2c10f:	48 ff ca             	dec    %rdx
   2c112:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   2c116:	eb 38                	jmp    2c150 <Main.work+0x950>
   2c118:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   2c11f:	00 
   2c120:	48 85 c9             	test   %rcx,%rcx
   2c123:	74 2b                	je     2c150 <Main.work+0x950>
   2c125:	48 89 c3             	mov    %rax,%rbx
   2c128:	4c 89 f7             	mov    %r14,%rdi
   2c12b:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2c132:	75 07                	jne    2c13b <Main.work+0x93b>
   2c134:	e8 37 ce 03 00       	call   68f70 <cinch_drop_slow>
   2c139:	eb 05                	jmp    2c140 <Main.work+0x940>
   2c13b:	e8 40 cd 03 00       	call   68e80 <cinch_drop_mt>
   2c140:	48 89 d8             	mov    %rbx,%rax
   2c143:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2c14a:	84 00 00 00 00 00 
   2c150:	48 85 c0             	test   %rax,%rax
   2c153:	0f 84 4c 04 00 00    	je     2c5a5 <Main.work+0xda5>
   2c159:	41 f6 c7 01          	test   $0x1,%r15b
   2c15d:	75 31                	jne    2c190 <Main.work+0x990>
   2c15f:	be 05 00 00 00       	mov    $0x5,%esi
   2c164:	4c 89 ff             	mov    %r15,%rdi
   2c167:	e8 c4 2c 04 00       	call   6ee30 <cinch_nat_rem>
   2c16c:	48 89 c3             	mov    %rax,%rbx
   2c16f:	f6 c3 01             	test   $0x1,%bl
   2c172:	74 27                	je     2c19b <Main.work+0x99b>
   2c174:	48 d1 eb             	shr    $1,%rbx
   2c177:	48 85 db             	test   %rbx,%rbx
   2c17a:	0f 97 c0             	seta   %al
   2c17d:	1c 00                	sbb    $0x0,%al
   2c17f:	48 0f be c0          	movsbq %al,%rax
   2c183:	eb 6b                	jmp    2c1f0 <Main.work+0x9f0>
   2c185:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2c18c:	00 00 00 00 
   2c190:	44 89 fb             	mov    %r15d,%ebx
   2c193:	83 e3 03             	and    $0x3,%ebx
   2c196:	f6 c3 01             	test   $0x1,%bl
   2c199:	75 d9                	jne    2c174 <Main.work+0x974>
   2c19b:	be 01 00 00 00       	mov    $0x1,%esi
   2c1a0:	48 89 df             	mov    %rbx,%rdi
   2c1a3:	e8 c8 22 04 00       	call   6e470 <cinch_nat_cmp>
   2c1a8:	48 85 db             	test   %rbx,%rbx
   2c1ab:	74 43                	je     2c1f0 <Main.work+0x9f0>
   2c1ad:	48 8b 53 f8          	mov    -0x8(%rbx),%rdx
   2c1b1:	48 89 d1             	mov    %rdx,%rcx
   2c1b4:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2c1b8:	48 81 f9 01 00 00 20 	cmp    $0x20000001,%rcx
   2c1bf:	7c 09                	jl     2c1ca <Main.work+0x9ca>
   2c1c1:	48 ff ca             	dec    %rdx
   2c1c4:	48 89 53 f8          	mov    %rdx,-0x8(%rbx)
   2c1c8:	eb 26                	jmp    2c1f0 <Main.work+0x9f0>
   2c1ca:	48 85 c9             	test   %rcx,%rcx
   2c1cd:	74 21                	je     2c1f0 <Main.work+0x9f0>
   2c1cf:	49 89 c4             	mov    %rax,%r12
   2c1d2:	48 89 df             	mov    %rbx,%rdi
   2c1d5:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   2c1dc:	75 07                	jne    2c1e5 <Main.work+0x9e5>
   2c1de:	e8 8d cd 03 00       	call   68f70 <cinch_drop_slow>
   2c1e3:	eb 05                	jmp    2c1ea <Main.work+0x9ea>
   2c1e5:	e8 96 cc 03 00       	call   68e80 <cinch_drop_mt>
   2c1ea:	4c 89 e0             	mov    %r12,%rax
   2c1ed:	0f 1f 00             	nopl   (%rax)
   2c1f0:	48 85 c0             	test   %rax,%rax
   2c1f3:	74 2b                	je     2c220 <Main.work+0xa20>
   2c1f5:	48 83 7d b8 00       	cmpq   $0x0,-0x48(%rbp)
   2c1fa:	74 54                	je     2c250 <Main.work+0xa50>
   2c1fc:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   2c200:	48 8b 43 f8          	mov    -0x8(%rbx),%rax
   2c204:	48 89 c1             	mov    %rax,%rcx
   2c207:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2c20b:	48 85 c9             	test   %rcx,%rcx
   2c20e:	7f 2b                	jg     2c23b <Main.work+0xa3b>
   2c210:	74 5a                	je     2c26c <Main.work+0xa6c>
   2c212:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   2c216:	eb 4a                	jmp    2c262 <Main.work+0xa62>
   2c218:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   2c21f:	00 
   2c220:	48 83 7d c0 00       	cmpq   $0x0,-0x40(%rbp)
   2c225:	74 29                	je     2c250 <Main.work+0xa50>
   2c227:	48 8b 5d c0          	mov    -0x40(%rbp),%rbx
   2c22b:	48 8b 43 f8          	mov    -0x8(%rbx),%rax
   2c22f:	48 89 c1             	mov    %rax,%rcx
   2c232:	48 c1 e1 1d          	shl    $0x1d,%rcx
   2c236:	48 85 c9             	test   %rcx,%rcx
   2c239:	7e 21                	jle    2c25c <Main.work+0xa5c>
   2c23b:	48 ff c0             	inc    %rax
   2c23e:	48 89 43 f8          	mov    %rax,-0x8(%rbx)
   2c242:	eb 3c                	jmp    2c280 <Main.work+0xa80>
   2c244:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   2c24b:	00 00 00 00 00 
   2c250:	31 ff                	xor    %edi,%edi
   2c252:	e8 a9 87 ff ff       	call   24a00 <Data.String.string-length>
   2c257:	49 89 c4             	mov    %rax,%r12
   2c25a:	eb 74                	jmp    2c2d0 <Main.work+0xad0>
   2c25c:	74 14                	je     2c272 <Main.work+0xa72>
   2c25e:	48 8b 5d c0          	mov    -0x40(%rbp),%rbx
   2c262:	48 89 df             	mov    %rbx,%rdi
   2c265:	e8 e6 cf 03 00       	call   69250 <cinch_dup_mt>
   2c26a:	eb 14                	jmp    2c280 <Main.work+0xa80>
   2c26c:	48 8b 5d b8          	mov    -0x48(%rbp),%rbx
   2c270:	eb 0e                	jmp    2c280 <Main.work+0xa80>
   2c272:	48 8b 5d c0          	mov    -0x40(%rbp),%rbx
   2c276:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2c27d:	00 00 00 
   2c280:	49 89 dd             	mov    %rbx,%r13
   2c283:	48 89 df             	mov    %rbx,%rdi
   2c286:	e8 75 87 ff ff       	call   24a00 <Data.String.string-length>
   2c28b:	49 89 c4             	mov    %rax,%r12
   2c28e:	48 8b 4b f8          	mov    -0x8(%rbx),%rcx
   2c292:	48 89 c8             	mov    %rcx,%rax
   2c295:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c299:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c29f:	7c 0f                	jl     2c2b0 <Main.work+0xab0>
   2c2a1:	48 ff c9             	dec    %rcx
   2c2a4:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2c2a8:	eb 26                	jmp    2c2d0 <Main.work+0xad0>
   2c2aa:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   2c2b0:	48 85 c0             	test   %rax,%rax
   2c2b3:	74 1b                	je     2c2d0 <Main.work+0xad0>
   2c2b5:	48 89 df             	mov    %rbx,%rdi
   2c2b8:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c2be:	75 07                	jne    2c2c7 <Main.work+0xac7>
   2c2c0:	e8 ab cc 03 00       	call   68f70 <cinch_drop_slow>
   2c2c5:	eb 09                	jmp    2c2d0 <Main.work+0xad0>
   2c2c7:	e8 b4 cb 03 00       	call   68e80 <cinch_drop_mt>
   2c2cc:	0f 1f 40 00          	nopl   0x0(%rax)
   2c2d0:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c2d4:	89 fb                	mov    %edi,%ebx
   2c2d6:	83 e3 01             	and    $0x1,%ebx
   2c2d9:	44 85 e3             	test   %r12d,%ebx
   2c2dc:	74 22                	je     2c300 <Main.work+0xb00>
   2c2de:	48 8b 4d c8          	mov    -0x38(%rbp),%rcx
   2c2e2:	48 d1 e9             	shr    $1,%rcx
   2c2e5:	4c 89 e0             	mov    %r12,%rax
   2c2e8:	48 d1 e8             	shr    $1,%rax
   2c2eb:	48 01 c8             	add    %rcx,%rax
   2c2ee:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c2f2:	78 0c                	js     2c300 <Main.work+0xb00>
   2c2f4:	4c 8d 2c 45 01 00 00 	lea    0x1(,%rax,2),%r13
   2c2fb:	00 
   2c2fc:	eb 11                	jmp    2c30f <Main.work+0xb0f>
   2c2fe:	66 90                	xchg   %ax,%ax
   2c300:	4c 89 e6             	mov    %r12,%rsi
   2c303:	e8 78 1f 04 00       	call   6e280 <cinch_nat_add>
   2c308:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   2c30c:	49 89 c5             	mov    %rax,%r13
   2c30f:	48 85 ff             	test   %rdi,%rdi
   2c312:	0f 94 c0             	sete   %al
   2c315:	08 c3                	or     %al,%bl
   2c317:	75 47                	jne    2c360 <Main.work+0xb60>
   2c319:	48 8b 4f f8          	mov    -0x8(%rdi),%rcx
   2c31d:	48 89 c8             	mov    %rcx,%rax
   2c320:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c324:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c32a:	7c 14                	jl     2c340 <Main.work+0xb40>
   2c32c:	48 ff c9             	dec    %rcx
   2c32f:	48 89 4f f8          	mov    %rcx,-0x8(%rdi)
   2c333:	eb 2b                	jmp    2c360 <Main.work+0xb60>
   2c335:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2c33c:	00 00 00 00 
   2c340:	48 85 c0             	test   %rax,%rax
   2c343:	74 1b                	je     2c360 <Main.work+0xb60>
   2c345:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c34b:	75 07                	jne    2c354 <Main.work+0xb54>
   2c34d:	e8 1e cc 03 00       	call   68f70 <cinch_drop_slow>
   2c352:	eb 0c                	jmp    2c360 <Main.work+0xb60>
   2c354:	e8 27 cb 03 00       	call   68e80 <cinch_drop_mt>
   2c359:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2c360:	4d 85 e4             	test   %r12,%r12
   2c363:	0f 94 c0             	sete   %al
   2c366:	44 89 e1             	mov    %r12d,%ecx
   2c369:	08 c1                	or     %al,%cl
   2c36b:	f6 c1 01             	test   $0x1,%cl
   2c36e:	74 40                	je     2c3b0 <Main.work+0xbb0>
   2c370:	41 f6 c5 01          	test   $0x1,%r13b
   2c374:	75 60                	jne    2c3d6 <Main.work+0xbd6>
   2c376:	be 0f 94 35 77       	mov    $0x7735940f,%esi
   2c37b:	4c 89 ef             	mov    %r13,%rdi
   2c37e:	e8 ad 2a 04 00       	call   6ee30 <cinch_nat_rem>
   2c383:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   2c387:	4d 85 ed             	test   %r13,%r13
   2c38a:	74 76                	je     2c402 <Main.work+0xc02>
   2c38c:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2c390:	48 89 c8             	mov    %rcx,%rax
   2c393:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c397:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c39d:	0f 8c 78 01 00 00    	jl     2c51b <Main.work+0xd1b>
   2c3a3:	48 ff c9             	dec    %rcx
   2c3a6:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2c3aa:	eb 56                	jmp    2c402 <Main.work+0xc02>
   2c3ac:	0f 1f 40 00          	nopl   0x0(%rax)
   2c3b0:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2c3b5:	48 89 c8             	mov    %rcx,%rax
   2c3b8:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c3bc:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c3c2:	0f 8c 98 00 00 00    	jl     2c460 <Main.work+0xc60>
   2c3c8:	48 ff c9             	dec    %rcx
   2c3cb:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2c3d0:	41 f6 c5 01          	test   $0x1,%r13b
   2c3d4:	74 a0                	je     2c376 <Main.work+0xb76>
   2c3d6:	49 d1 ed             	shr    $1,%r13
   2c3d9:	4c 89 e8             	mov    %r13,%rax
   2c3dc:	48 b9 e5 8f a2 12 31 	movabs $0x89705f3112a28fe5,%rcx
   2c3e3:	5f 70 89 
   2c3e6:	48 f7 e1             	mul    %rcx
   2c3e9:	48 c1 ea 1d          	shr    $0x1d,%rdx
   2c3ed:	69 c2 07 ca 9a 3b    	imul   $0x3b9aca07,%edx,%eax
   2c3f3:	41 29 c5             	sub    %eax,%r13d
   2c3f6:	4a 8d 04 6d 01 00 00 	lea    0x1(,%r13,2),%rax
   2c3fd:	00 
   2c3fe:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   2c402:	41 f6 c6 01          	test   $0x1,%r14b
   2c406:	0f 94 c0             	sete   %al
   2c409:	49 83 fe 02          	cmp    $0x2,%r14
   2c40d:	0f 92 c1             	setb   %cl
   2c410:	08 c1                	or     %al,%cl
   2c412:	74 1c                	je     2c430 <Main.work+0xc30>
   2c414:	be 03 00 00 00       	mov    $0x3,%esi
   2c419:	4c 89 f7             	mov    %r14,%rdi
   2c41c:	e8 4f 2c 04 00       	call   6f070 <cinch_nat_sub>
   2c421:	49 89 c5             	mov    %rax,%r13
   2c424:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2c428:	74 10                	je     2c43a <Main.work+0xc3a>
   2c42a:	e9 81 00 00 00       	jmp    2c4b0 <Main.work+0xcb0>
   2c42f:	90                   	nop
   2c430:	4d 8d 6e fe          	lea    -0x2(%r14),%r13
   2c434:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2c438:	75 76                	jne    2c4b0 <Main.work+0xcb0>
   2c43a:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2c43e:	48 89 c8             	mov    %rcx,%rax
   2c441:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c445:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c44b:	7c 43                	jl     2c490 <Main.work+0xc90>
   2c44d:	48 ff c9             	dec    %rcx
   2c450:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2c454:	41 f6 c7 01          	test   $0x1,%r15b
   2c458:	0f 85 d2 f3 ff ff    	jne    2b830 <Main.work+0x30>
   2c45e:	eb 5a                	jmp    2c4ba <Main.work+0xcba>
   2c460:	48 85 c0             	test   %rax,%rax
   2c463:	0f 84 07 ff ff ff    	je     2c370 <Main.work+0xb70>
   2c469:	4c 89 e7             	mov    %r12,%rdi
   2c46c:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c472:	0f 85 c1 00 00 00    	jne    2c539 <Main.work+0xd39>
   2c478:	e8 f3 ca 03 00       	call   68f70 <cinch_drop_slow>
   2c47d:	e9 ee fe ff ff       	jmp    2c370 <Main.work+0xb70>
   2c482:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2c489:	1f 84 00 00 00 00 00 
   2c490:	48 85 c0             	test   %rax,%rax
   2c493:	74 1b                	je     2c4b0 <Main.work+0xcb0>
   2c495:	4c 89 f7             	mov    %r14,%rdi
   2c498:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c49e:	0f 85 a9 00 00 00    	jne    2c54d <Main.work+0xd4d>
   2c4a4:	e8 c7 ca 03 00       	call   68f70 <cinch_drop_slow>
   2c4a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   2c4b0:	41 f6 c7 01          	test   $0x1,%r15b
   2c4b4:	0f 85 76 f3 ff ff    	jne    2b830 <Main.work+0x30>
   2c4ba:	be 03 00 00 00       	mov    $0x3,%esi
   2c4bf:	4c 89 ff             	mov    %r15,%rdi
   2c4c2:	e8 b9 1d 04 00       	call   6e280 <cinch_nat_add>
   2c4c7:	49 89 c4             	mov    %rax,%r12
   2c4ca:	4d 85 ff             	test   %r15,%r15
   2c4cd:	0f 94 c0             	sete   %al
   2c4d0:	44 89 f9             	mov    %r15d,%ecx
   2c4d3:	08 c1                	or     %al,%cl
   2c4d5:	f6 c1 01             	test   $0x1,%cl
   2c4d8:	0f 85 69 f3 ff ff    	jne    2b847 <Main.work+0x47>
   2c4de:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2c4e2:	48 89 c8             	mov    %rcx,%rax
   2c4e5:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c4e9:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c4ef:	7c 0c                	jl     2c4fd <Main.work+0xcfd>
   2c4f1:	48 ff c9             	dec    %rcx
   2c4f4:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2c4f8:	e9 4a f3 ff ff       	jmp    2b847 <Main.work+0x47>
   2c4fd:	48 85 c0             	test   %rax,%rax
   2c500:	0f 84 41 f3 ff ff    	je     2b847 <Main.work+0x47>
   2c506:	4c 89 ff             	mov    %r15,%rdi
   2c509:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c50f:	75 50                	jne    2c561 <Main.work+0xd61>
   2c511:	e8 5a ca 03 00       	call   68f70 <cinch_drop_slow>
   2c516:	e9 2c f3 ff ff       	jmp    2b847 <Main.work+0x47>
   2c51b:	48 85 c0             	test   %rax,%rax
   2c51e:	0f 84 de fe ff ff    	je     2c402 <Main.work+0xc02>
   2c524:	4c 89 ef             	mov    %r13,%rdi
   2c527:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c52d:	75 3c                	jne    2c56b <Main.work+0xd6b>
   2c52f:	e8 3c ca 03 00       	call   68f70 <cinch_drop_slow>
   2c534:	e9 c9 fe ff ff       	jmp    2c402 <Main.work+0xc02>
   2c539:	e8 42 c9 03 00       	call   68e80 <cinch_drop_mt>
   2c53e:	41 f6 c5 01          	test   $0x1,%r13b
   2c542:	0f 84 2e fe ff ff    	je     2c376 <Main.work+0xb76>
   2c548:	e9 89 fe ff ff       	jmp    2c3d6 <Main.work+0xbd6>
   2c54d:	e8 2e c9 03 00       	call   68e80 <cinch_drop_mt>
   2c552:	41 f6 c7 01          	test   $0x1,%r15b
   2c556:	0f 84 5e ff ff ff    	je     2c4ba <Main.work+0xcba>
   2c55c:	e9 cf f2 ff ff       	jmp    2b830 <Main.work+0x30>
   2c561:	e8 1a c9 03 00       	call   68e80 <cinch_drop_mt>
   2c566:	e9 dc f2 ff ff       	jmp    2b847 <Main.work+0x47>
   2c56b:	e8 10 c9 03 00       	call   68e80 <cinch_drop_mt>
   2c570:	e9 8d fe ff ff       	jmp    2c402 <Main.work+0xc02>
   2c575:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   2c579:	48 85 d2             	test   %rdx,%rdx
   2c57c:	0f 84 93 00 00 00    	je     2c615 <Main.work+0xe15>
   2c582:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c586:	48 89 c8             	mov    %rcx,%rax
   2c589:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c58d:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c593:	7c 3c                	jl     2c5d1 <Main.work+0xdd1>
   2c595:	48 ff c9             	dec    %rcx
   2c598:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c59c:	eb 77                	jmp    2c615 <Main.work+0xe15>
   2c59e:	31 c0                	xor    %eax,%eax
   2c5a0:	e9 18 02 00 00       	jmp    2c7bd <Main.work+0xfbd>
   2c5a5:	48 8b 55 c0          	mov    -0x40(%rbp),%rdx
   2c5a9:	48 85 d2             	test   %rdx,%rdx
   2c5ac:	0f 84 f4 00 00 00    	je     2c6a6 <Main.work+0xea6>
   2c5b2:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c5b6:	48 89 c8             	mov    %rcx,%rax
   2c5b9:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c5bd:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c5c3:	7c 24                	jl     2c5e9 <Main.work+0xde9>
   2c5c5:	48 ff c9             	dec    %rcx
   2c5c8:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c5cc:	e9 d5 00 00 00       	jmp    2c6a6 <Main.work+0xea6>
   2c5d1:	48 85 c0             	test   %rax,%rax
   2c5d4:	74 3f                	je     2c615 <Main.work+0xe15>
   2c5d6:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c5dc:	75 2e                	jne    2c60c <Main.work+0xe0c>
   2c5de:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c5e2:	e8 89 c9 03 00       	call   68f70 <cinch_drop_slow>
   2c5e7:	eb 2c                	jmp    2c615 <Main.work+0xe15>
   2c5e9:	48 85 c0             	test   %rax,%rax
   2c5ec:	0f 84 b4 00 00 00    	je     2c6a6 <Main.work+0xea6>
   2c5f2:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c5f8:	0f 85 9f 00 00 00    	jne    2c69d <Main.work+0xe9d>
   2c5fe:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c602:	e8 69 c9 03 00       	call   68f70 <cinch_drop_slow>
   2c607:	e9 9a 00 00 00       	jmp    2c6a6 <Main.work+0xea6>
   2c60c:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c610:	e8 6b c8 03 00       	call   68e80 <cinch_drop_mt>
   2c615:	4d 85 ff             	test   %r15,%r15
   2c618:	74 38                	je     2c652 <Main.work+0xe52>
   2c61a:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2c61e:	48 89 c8             	mov    %rcx,%rax
   2c621:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c625:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c62b:	7c 09                	jl     2c636 <Main.work+0xe36>
   2c62d:	48 ff c9             	dec    %rcx
   2c630:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2c634:	eb 1c                	jmp    2c652 <Main.work+0xe52>
   2c636:	48 85 c0             	test   %rax,%rax
   2c639:	74 17                	je     2c652 <Main.work+0xe52>
   2c63b:	4c 89 ff             	mov    %r15,%rdi
   2c63e:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c644:	75 07                	jne    2c64d <Main.work+0xe4d>
   2c646:	e8 25 c9 03 00       	call   68f70 <cinch_drop_slow>
   2c64b:	eb 05                	jmp    2c652 <Main.work+0xe52>
   2c64d:	e8 2e c8 03 00       	call   68e80 <cinch_drop_mt>
   2c652:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2c656:	0f 85 d2 00 00 00    	jne    2c72e <Main.work+0xf2e>
   2c65c:	49 8b 4d f8          	mov    -0x8(%r13),%rcx
   2c660:	48 89 c8             	mov    %rcx,%rax
   2c663:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c667:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c66d:	7c 0c                	jl     2c67b <Main.work+0xe7b>
   2c66f:	48 ff c9             	dec    %rcx
   2c672:	49 89 4d f8          	mov    %rcx,-0x8(%r13)
   2c676:	e9 b3 00 00 00       	jmp    2c72e <Main.work+0xf2e>
   2c67b:	48 85 c0             	test   %rax,%rax
   2c67e:	0f 84 aa 00 00 00    	je     2c72e <Main.work+0xf2e>
   2c684:	4c 89 ef             	mov    %r13,%rdi
   2c687:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c68d:	0f 85 96 00 00 00    	jne    2c729 <Main.work+0xf29>
   2c693:	e8 d8 c8 03 00       	call   68f70 <cinch_drop_slow>
   2c698:	e9 91 00 00 00       	jmp    2c72e <Main.work+0xf2e>
   2c69d:	48 8b 7d c0          	mov    -0x40(%rbp),%rdi
   2c6a1:	e8 da c7 03 00       	call   68e80 <cinch_drop_mt>
   2c6a6:	48 8b 55 b8          	mov    -0x48(%rbp),%rdx
   2c6aa:	48 85 d2             	test   %rdx,%rdx
   2c6ad:	74 3d                	je     2c6ec <Main.work+0xeec>
   2c6af:	48 8b 4a f8          	mov    -0x8(%rdx),%rcx
   2c6b3:	48 89 c8             	mov    %rcx,%rax
   2c6b6:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c6ba:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c6c0:	7c 09                	jl     2c6cb <Main.work+0xecb>
   2c6c2:	48 ff c9             	dec    %rcx
   2c6c5:	48 89 4a f8          	mov    %rcx,-0x8(%rdx)
   2c6c9:	eb 21                	jmp    2c6ec <Main.work+0xeec>
   2c6cb:	48 85 c0             	test   %rax,%rax
   2c6ce:	74 1c                	je     2c6ec <Main.work+0xeec>
   2c6d0:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c6d6:	75 0b                	jne    2c6e3 <Main.work+0xee3>
   2c6d8:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   2c6dc:	e8 8f c8 03 00       	call   68f70 <cinch_drop_slow>
   2c6e1:	eb 09                	jmp    2c6ec <Main.work+0xeec>
   2c6e3:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   2c6e7:	e8 94 c7 03 00       	call   68e80 <cinch_drop_mt>
   2c6ec:	f6 45 d7 01          	testb  $0x1,-0x29(%rbp)
   2c6f0:	0f 85 82 00 00 00    	jne    2c778 <Main.work+0xf78>
   2c6f6:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   2c6fa:	48 89 c8             	mov    %rcx,%rax
   2c6fd:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c701:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c707:	7c 09                	jl     2c712 <Main.work+0xf12>
   2c709:	48 ff c9             	dec    %rcx
   2c70c:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   2c710:	eb 66                	jmp    2c778 <Main.work+0xf78>
   2c712:	48 85 c0             	test   %rax,%rax
   2c715:	74 61                	je     2c778 <Main.work+0xf78>
   2c717:	4c 89 f7             	mov    %r14,%rdi
   2c71a:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c720:	75 51                	jne    2c773 <Main.work+0xf73>
   2c722:	e8 49 c8 03 00       	call   68f70 <cinch_drop_slow>
   2c727:	eb 4f                	jmp    2c778 <Main.work+0xf78>
   2c729:	e8 52 c7 03 00       	call   68e80 <cinch_drop_mt>
   2c72e:	4d 85 e4             	test   %r12,%r12
   2c731:	0f 94 c0             	sete   %al
   2c734:	44 89 e1             	mov    %r12d,%ecx
   2c737:	08 c1                	or     %al,%cl
   2c739:	f6 c1 01             	test   $0x1,%cl
   2c73c:	75 7b                	jne    2c7b9 <Main.work+0xfb9>
   2c73e:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   2c743:	48 89 c8             	mov    %rcx,%rax
   2c746:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c74a:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c750:	7c 0a                	jl     2c75c <Main.work+0xf5c>
   2c752:	48 ff c9             	dec    %rcx
   2c755:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   2c75a:	eb 5d                	jmp    2c7b9 <Main.work+0xfb9>
   2c75c:	48 85 c0             	test   %rax,%rax
   2c75f:	74 58                	je     2c7b9 <Main.work+0xfb9>
   2c761:	4c 89 e7             	mov    %r12,%rdi
   2c764:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c76a:	75 48                	jne    2c7b4 <Main.work+0xfb4>
   2c76c:	e8 ff c7 03 00       	call   68f70 <cinch_drop_slow>
   2c771:	eb 46                	jmp    2c7b9 <Main.work+0xfb9>
   2c773:	e8 08 c7 03 00       	call   68e80 <cinch_drop_mt>
   2c778:	4d 85 ff             	test   %r15,%r15
   2c77b:	0f 94 c0             	sete   %al
   2c77e:	44 89 f9             	mov    %r15d,%ecx
   2c781:	08 c1                	or     %al,%cl
   2c783:	f6 c1 01             	test   $0x1,%cl
   2c786:	75 31                	jne    2c7b9 <Main.work+0xfb9>
   2c788:	49 8b 4f f8          	mov    -0x8(%r15),%rcx
   2c78c:	48 89 c8             	mov    %rcx,%rax
   2c78f:	48 c1 e0 1d          	shl    $0x1d,%rax
   2c793:	48 3d 01 00 00 20    	cmp    $0x20000001,%rax
   2c799:	7c 09                	jl     2c7a4 <Main.work+0xfa4>
   2c79b:	48 ff c9             	dec    %rcx
   2c79e:	49 89 4f f8          	mov    %rcx,-0x8(%r15)
   2c7a2:	eb 15                	jmp    2c7b9 <Main.work+0xfb9>
   2c7a4:	48 85 c0             	test   %rax,%rax
   2c7a7:	74 10                	je     2c7b9 <Main.work+0xfb9>
   2c7a9:	4c 89 ff             	mov    %r15,%rdi
   2c7ac:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   2c7b2:	74 b8                	je     2c76c <Main.work+0xf6c>
   2c7b4:	e8 c7 c6 03 00       	call   68e80 <cinch_drop_mt>
   2c7b9:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
   2c7bd:	48 83 c4 28          	add    $0x28,%rsp
   2c7c1:	5b                   	pop    %rbx
   2c7c2:	41 5c                	pop    %r12
   2c7c4:	41 5d                	pop    %r13
   2c7c6:	41 5e                	pop    %r14
   2c7c8:	41 5f                	pop    %r15
   2c7ca:	5d                   	pop    %rbp
   2c7cb:	c3                   	ret

Disassembly of section .fini:
