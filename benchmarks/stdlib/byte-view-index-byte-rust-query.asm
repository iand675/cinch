
build/byte-view-index-byte-bench-rust/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000035870 <Data.Bytes.View.byte-view-index-byte>:
   35870:	48 8b 47 f8          	mov    -0x8(%rdi),%rax
   35874:	48 c1 e8 28          	shr    $0x28,%rax
   35878:	0f b6 c0             	movzbl %al,%eax
   3587b:	48 3d ff 00 00 00    	cmp    $0xff,%rax
   35881:	75 07                	jne    3588a <Data.Bytes.View.byte-view-index-byte+0x1a>
   35883:	48 8b 47 f0          	mov    -0x10(%rdi),%rax
   35887:	48 8b 00             	mov    (%rax),%rax
   3588a:	55                   	push   %rbp
   3588b:	48 89 e5             	mov    %rsp,%rbp
   3588e:	41 57                	push   %r15
   35890:	41 56                	push   %r14
   35892:	41 55                	push   %r13
   35894:	41 54                	push   %r12
   35896:	53                   	push   %rbx
   35897:	48 83 ec 18          	sub    $0x18,%rsp
   3589b:	48 85 c0             	test   %rax,%rax
   3589e:	74 07                	je     358a7 <Data.Bytes.View.byte-view-index-byte+0x37>
   358a0:	31 c0                	xor    %eax,%eax
   358a2:	e9 e2 03 00 00       	jmp    35c89 <Data.Bytes.View.byte-view-index-byte+0x419>
   358a7:	89 75 d4             	mov    %esi,-0x2c(%rbp)
   358aa:	49 b8 ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%r8
   358b1:	ff ff 7f
   358b4:	4c 8b 37             	mov    (%rdi),%r14
   358b7:	4c 8b 67 08          	mov    0x8(%rdi),%r12
   358bb:	4c 8b 7f 10          	mov    0x10(%rdi),%r15
   358bf:	4d 85 e4             	test   %r12,%r12
   358c2:	0f 94 c0             	sete   %al
   358c5:	44 89 e3             	mov    %r12d,%ebx
   358c8:	83 e3 01             	and    $0x1,%ebx
   358cb:	45 89 e5             	mov    %r12d,%r13d
   358ce:	41 08 c5             	or     %al,%r13b
   358d1:	41 f6 c5 01          	test   $0x1,%r13b
   358d5:	75 4f                	jne    35926 <Data.Bytes.View.byte-view-index-byte+0xb6>
   358d7:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   358dc:	48 89 c8             	mov    %rcx,%rax
   358df:	48 c1 e0 1d          	shl    $0x1d,%rax
   358e3:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
   358e7:	49 8d b0 00 00 00 20 	lea    0x20000000(%r8),%rsi
   358ee:	48 39 f2             	cmp    %rsi,%rdx
   358f1:	73 0a                	jae    358fd <Data.Bytes.View.byte-view-index-byte+0x8d>
   358f3:	48 ff c1             	inc    %rcx
   358f6:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   358fb:	eb 29                	jmp    35926 <Data.Bytes.View.byte-view-index-byte+0xb6>
   358fd:	49 8d 88 01 00 00 20 	lea    0x20000001(%r8),%rcx
   35904:	48 39 c8             	cmp    %rcx,%rax
   35907:	0f 9d c1             	setge  %cl
   3590a:	48 85 c0             	test   %rax,%rax
   3590d:	0f 94 c0             	sete   %al
   35910:	08 c8                	or     %cl,%al
   35912:	75 12                	jne    35926 <Data.Bytes.View.byte-view-index-byte+0xb6>
   35914:	4c 89 e7             	mov    %r12,%rdi
   35917:	e8 e4 e3 04 00       	call   83d00 <cinch_dup_mt>
   3591c:	49 b8 ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%r8
   35923:	ff ff 7f
   35926:	4c 89 75 c8          	mov    %r14,-0x38(%rbp)
   3592a:	44 85 fb             	test   %r15d,%ebx
   3592d:	74 19                	je     35948 <Data.Bytes.View.byte-view-index-byte+0xd8>
   3592f:	4c 89 f8             	mov    %r15,%rax
   35932:	4c 31 e0             	xor    %r12,%rax
   35935:	48 83 f8 02          	cmp    $0x2,%rax
   35939:	41 0f 92 c6          	setb   %r14b
   3593d:	41 f6 c5 01          	test   $0x1,%r13b
   35941:	74 27                	je     3596a <Data.Bytes.View.byte-view-index-byte+0xfa>
   35943:	e9 9a 00 00 00       	jmp    359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   35948:	4c 89 e7             	mov    %r12,%rdi
   3594b:	4c 89 fe             	mov    %r15,%rsi
   3594e:	e8 cd 35 05 00       	call   88f20 <cinch_nat_cmp>
   35953:	49 b8 ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%r8
   3595a:	ff ff 7f
   3595d:	48 85 c0             	test   %rax,%rax
   35960:	41 0f 94 c6          	sete   %r14b
   35964:	41 f6 c5 01          	test   $0x1,%r13b
   35968:	75 78                	jne    359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   3596a:	49 8b 4c 24 f8       	mov    -0x8(%r12),%rcx
   3596f:	48 89 c8             	mov    %rcx,%rax
   35972:	48 c1 e0 1d          	shl    $0x1d,%rax
   35976:	48 8d 90 ff ff ff df 	lea    -0x20000001(%rax),%rdx
   3597d:	4c 39 c2             	cmp    %r8,%rdx
   35980:	73 0a                	jae    3598c <Data.Bytes.View.byte-view-index-byte+0x11c>
   35982:	48 ff c9             	dec    %rcx
   35985:	49 89 4c 24 f8       	mov    %rcx,-0x8(%r12)
   3598a:	eb 56                	jmp    359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   3598c:	48 b9 ff ff ff bf ff 	movabs $0x7fffffffbfffffff,%rcx
   35993:	ff ff 7f
   35996:	48 39 c8             	cmp    %rcx,%rax
   35999:	7f 17                	jg     359b2 <Data.Bytes.View.byte-view-index-byte+0x142>
   3599b:	48 85 c0             	test   %rax,%rax
   3599e:	74 42                	je     359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   359a0:	48 3d 00 00 00 20    	cmp    $0x20000000,%rax
   359a6:	75 32                	jne    359da <Data.Bytes.View.byte-view-index-byte+0x16a>
   359a8:	4c 89 e7             	mov    %r12,%rdi
   359ab:	e8 70 e0 04 00       	call   83a20 <cinch_drop_slow>
   359b0:	eb 30                	jmp    359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   359b2:	48 b9 00 00 00 c0 ff 	movabs $0x7fffffffc0000000,%rcx
   359b9:	ff ff 7f
   359bc:	48 39 c8             	cmp    %rcx,%rax
   359bf:	74 0f                	je     359d0 <Data.Bytes.View.byte-view-index-byte+0x160>
   359c1:	48 b9 00 00 00 e0 ff 	movabs $0x7fffffffe0000000,%rcx
   359c8:	ff ff 7f
   359cb:	48 39 c8             	cmp    %rcx,%rax
   359ce:	75 0a                	jne    359da <Data.Bytes.View.byte-view-index-byte+0x16a>
   359d0:	4c 89 e7             	mov    %r12,%rdi
   359d3:	e8 88 f4 04 00       	call   84e60 <cinch_frame_release>
   359d8:	eb 08                	jmp    359e2 <Data.Bytes.View.byte-view-index-byte+0x172>
   359da:	4c 89 e7             	mov    %r12,%rdi
   359dd:	e8 4e df 04 00       	call   83930 <cinch_drop_mt>
   359e2:	48 8d 05 37 30 10 00 	lea    0x103037(%rip),%rax        # 138a20 <Effect.Process.ProcessInvalidArgument.blk#h>
   359e9:	45 84 f6             	test   %r14b,%r14b
   359ec:	0f 85 97 02 00 00    	jne    35c89 <Data.Bytes.View.byte-view-index-byte+0x419>
   359f2:	48 85 db             	test   %rbx,%rbx
   359f5:	75 0d                	jne    35a04 <Data.Bytes.View.byte-view-index-byte+0x194>
   359f7:	4c 89 e7             	mov    %r12,%rdi
   359fa:	e8 41 45 05 00       	call   89f40 <cinch_nat_to_i64>
   359ff:	49 89 c4             	mov    %rax,%r12
   35a02:	eb 03                	jmp    35a07 <Data.Bytes.View.byte-view-index-byte+0x197>
   35a04:	49 d1 ec             	shr    $1,%r12
   35a07:	4c 8b 75 c8          	mov    -0x38(%rbp),%r14
   35a0b:	41 f6 c7 01          	test   $0x1,%r15b
   35a0f:	75 0d                	jne    35a1e <Data.Bytes.View.byte-view-index-byte+0x1ae>
   35a11:	4c 89 ff             	mov    %r15,%rdi
   35a14:	e8 27 45 05 00       	call   89f40 <cinch_nat_to_i64>
   35a19:	49 89 c7             	mov    %rax,%r15
   35a1c:	eb 03                	jmp    35a21 <Data.Bytes.View.byte-view-index-byte+0x1b1>
   35a1e:	49 d1 ef             	shr    $1,%r15
   35a21:	48 b9 00 00 00 00 f0 	movabs $0xf000000000,%rcx
   35a28:	00 00 00
   35a2b:	49 bd 00 00 00 00 40 	movabs $0x4000000000,%r13
   35a32:	00 00 00
   35a35:	4d 85 f6             	test   %r14,%r14
   35a38:	74 22                	je     35a5c <Data.Bytes.View.byte-view-index-byte+0x1ec>
   35a3a:	49 8b 46 f8          	mov    -0x8(%r14),%rax
   35a3e:	48 21 c8             	and    %rcx,%rax
   35a41:	4c 39 e8             	cmp    %r13,%rax
   35a44:	75 16                	jne    35a5c <Data.Bytes.View.byte-view-index-byte+0x1ec>
   35a46:	49 83 3e 00          	cmpq   $0x0,(%r14)
   35a4a:	b8 01 00 00 00       	mov    $0x1,%eax
   35a4f:	74 04                	je     35a55 <Data.Bytes.View.byte-view-index-byte+0x1e5>
   35a51:	49 8b 46 08          	mov    0x8(%r14),%rax
   35a55:	4d 39 fc             	cmp    %r15,%r12
   35a58:	7d 70                	jge    35aca <Data.Bytes.View.byte-view-index-byte+0x25a>
   35a5a:	eb 0d                	jmp    35a69 <Data.Bytes.View.byte-view-index-byte+0x1f9>
   35a5c:	4c 89 f7             	mov    %r14,%rdi
   35a5f:	e8 5c cf 04 00       	call   829c0 <cinch_bytes_data>
   35a64:	4d 39 fc             	cmp    %r15,%r12
   35a67:	7d 61                	jge    35aca <Data.Bytes.View.byte-view-index-byte+0x25a>
   35a69:	0f b6 55 d4          	movzbl -0x2c(%rbp),%edx
   35a6d:	48 b9 01 01 01 01 01 	movabs $0x101010101010101,%rcx
   35a74:	01 01 01
   35a77:	48 0f af ca          	imul   %rdx,%rcx
   35a7b:	4c 89 e3             	mov    %r12,%rbx
   35a7e:	42 38 0c 20          	cmp    %cl,(%rax,%r12,1)
   35a82:	74 4d                	je     35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35a84:	44 0f b6 c1          	movzbl %cl,%r8d
   35a88:	49 8d 5c 24 01       	lea    0x1(%r12),%rbx
   35a8d:	4c 89 fa             	mov    %r15,%rdx
   35a90:	48 29 da             	sub    %rbx,%rdx
   35a93:	48 83 fa 08          	cmp    $0x8,%rdx
   35a97:	0f 8d 56 01 00 00    	jge    35bf3 <Data.Bytes.View.byte-view-index-byte+0x383>
   35a9d:	49 39 df             	cmp    %rbx,%r15
   35aa0:	7e 28                	jle    35aca <Data.Bytes.View.byte-view-index-byte+0x25a>
   35aa2:	38 0c 18             	cmp    %cl,(%rax,%rbx,1)
   35aa5:	74 2a                	je     35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35aa7:	49 8d 54 24 02       	lea    0x2(%r12),%rdx
   35aac:	48 c7 c3 ff ff ff ff 	mov    $0xffffffffffffffff,%rbx
   35ab3:	4c 39 fa             	cmp    %r15,%rdx
   35ab6:	7d 19                	jge    35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35ab8:	48 89 d3             	mov    %rdx,%rbx
   35abb:	38 0c 10             	cmp    %cl,(%rax,%rdx,1)
   35abe:	74 11                	je     35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35ac0:	49 8d 74 24 03       	lea    0x3(%r12),%rsi
   35ac5:	e9 12 02 00 00       	jmp    35cdc <Data.Bytes.View.byte-view-index-byte+0x46c>
   35aca:	48 c7 c3 ff ff ff ff 	mov    $0xffffffffffffffff,%rbx
   35ad1:	4d 85 f6             	test   %r14,%r14
   35ad4:	74 2e                	je     35b04 <Data.Bytes.View.byte-view-index-byte+0x294>
   35ad6:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   35ada:	48 89 c8             	mov    %rcx,%rax
   35add:	48 c1 e0 1d          	shl    $0x1d,%rax
   35ae1:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
   35ae5:	48 bf ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%rdi
   35aec:	ff ff 7f
   35aef:	48 8d b7 00 00 00 20 	lea    0x20000000(%rdi),%rsi
   35af6:	48 39 f2             	cmp    %rsi,%rdx
   35af9:	73 15                	jae    35b10 <Data.Bytes.View.byte-view-index-byte+0x2a0>
   35afb:	48 ff c1             	inc    %rcx
   35afe:	49 89 4e f8          	mov    %rcx,-0x8(%r14)
   35b02:	eb 3c                	jmp    35b40 <Data.Bytes.View.byte-view-index-byte+0x2d0>
   35b04:	31 ff                	xor    %edi,%edi
   35b06:	e8 25 d0 04 00       	call   82b30 <cinch_bytes_len>
   35b0b:	e9 51 01 00 00       	jmp    35c61 <Data.Bytes.View.byte-view-index-byte+0x3f1>
   35b10:	48 8d 8f 01 00 00 20 	lea    0x20000001(%rdi),%rcx
   35b17:	48 39 c8             	cmp    %rcx,%rax
   35b1a:	0f 9d c1             	setge  %cl
   35b1d:	48 85 c0             	test   %rax,%rax
   35b20:	0f 94 c0             	sete   %al
   35b23:	08 c8                	or     %cl,%al
   35b25:	48 8d 05 f4 2e 10 00 	lea    0x102ef4(%rip),%rax        # 138a20 <Effect.Process.ProcessInvalidArgument.blk#h>
   35b2c:	75 19                	jne    35b47 <Data.Bytes.View.byte-view-index-byte+0x2d7>
   35b2e:	4c 89 f7             	mov    %r14,%rdi
   35b31:	e8 ca e1 04 00       	call   83d00 <cinch_dup_mt>
   35b36:	48 bf ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%rdi
   35b3d:	ff ff 7f
   35b40:	48 8d 05 d9 2e 10 00 	lea    0x102ed9(%rip),%rax        # 138a20 <Effect.Process.ProcessInvalidArgument.blk#h>
   35b47:	49 8b 4e f8          	mov    -0x8(%r14),%rcx
   35b4b:	48 ba 00 00 00 00 f0 	movabs $0xf000000000,%rdx
   35b52:	00 00 00
   35b55:	48 21 d1             	and    %rdx,%rcx
   35b58:	4c 39 e9             	cmp    %r13,%rcx
   35b5b:	74 19                	je     35b76 <Data.Bytes.View.byte-view-index-byte+0x306>
   35b5d:	4c 89 f7             	mov    %r14,%rdi
   35b60:	e8 cb cf 04 00       	call   82b30 <cinch_bytes_len>
   35b65:	48 bf ff ff ff 9f ff 	movabs $0x7fffffff9fffffff,%rdi
   35b6c:	ff ff 7f
   35b6f:	48 8d 05 aa 2e 10 00 	lea    0x102eaa(%rip),%rax        # 138a20 <Effect.Process.ProcessInvalidArgument.blk#h>
   35b76:	49 8b 56 f8          	mov    -0x8(%r14),%rdx
   35b7a:	48 89 d1             	mov    %rdx,%rcx
   35b7d:	48 c1 e1 1d          	shl    $0x1d,%rcx
   35b81:	48 8d b1 ff ff ff df 	lea    -0x20000001(%rcx),%rsi
   35b88:	48 39 fe             	cmp    %rdi,%rsi
   35b8b:	73 0c                	jae    35b99 <Data.Bytes.View.byte-view-index-byte+0x329>
   35b8d:	48 ff ca             	dec    %rdx
   35b90:	49 89 56 f8          	mov    %rdx,-0x8(%r14)
   35b94:	e9 cf 00 00 00       	jmp    35c68 <Data.Bytes.View.byte-view-index-byte+0x3f8>
   35b99:	48 ba ff ff ff bf ff 	movabs $0x7fffffffbfffffff,%rdx
   35ba0:	ff ff 7f
   35ba3:	48 39 d1             	cmp    %rdx,%rcx
   35ba6:	7f 23                	jg     35bcb <Data.Bytes.View.byte-view-index-byte+0x35b>
   35ba8:	48 85 c9             	test   %rcx,%rcx
   35bab:	0f 84 b7 00 00 00    	je     35c68 <Data.Bytes.View.byte-view-index-byte+0x3f8>
   35bb1:	48 81 f9 00 00 00 20 	cmp    $0x20000000,%rcx
   35bb8:	0f 85 9b 00 00 00    	jne    35c59 <Data.Bytes.View.byte-view-index-byte+0x3e9>
   35bbe:	4c 89 f7             	mov    %r14,%rdi
   35bc1:	e8 5a de 04 00       	call   83a20 <cinch_drop_slow>
   35bc6:	e9 96 00 00 00       	jmp    35c61 <Data.Bytes.View.byte-view-index-byte+0x3f1>
   35bcb:	48 b8 00 00 00 c0 ff 	movabs $0x7fffffffc0000000,%rax
   35bd2:	ff ff 7f
   35bd5:	48 39 c1             	cmp    %rax,%rcx
   35bd8:	74 0f                	je     35be9 <Data.Bytes.View.byte-view-index-byte+0x379>
   35bda:	48 b8 00 00 00 e0 ff 	movabs $0x7fffffffe0000000,%rax
   35be1:	ff ff 7f
   35be4:	48 39 c1             	cmp    %rax,%rcx
   35be7:	75 70                	jne    35c59 <Data.Bytes.View.byte-view-index-byte+0x3e9>
   35be9:	4c 89 f7             	mov    %r14,%rdi
   35bec:	e8 6f f2 04 00       	call   84e60 <cinch_frame_release>
   35bf1:	eb 6e                	jmp    35c61 <Data.Bytes.View.byte-view-index-byte+0x3f1>
   35bf3:	48 ba 80 80 80 80 80 	movabs $0x8080808080808080,%rdx
   35bfa:	80 80 80
   35bfd:	48 bf 7f 7f 7f 7f 7f 	movabs $0x7f7f7f7f7f7f7f7f,%rdi
   35c04:	7f 7f 7f
   35c07:	4c 8b 0c 18          	mov    (%rax,%rbx,1),%r9
   35c0b:	49 31 c9             	xor    %rcx,%r9
   35c0e:	4c 89 ce             	mov    %r9,%rsi
   35c11:	48 21 fe             	and    %rdi,%rsi
   35c14:	48 01 fe             	add    %rdi,%rsi
   35c17:	4c 09 ce             	or     %r9,%rsi
   35c1a:	48 21 d6             	and    %rdx,%rsi
   35c1d:	48 39 d6             	cmp    %rdx,%rsi
   35c20:	75 76                	jne    35c98 <Data.Bytes.View.byte-view-index-byte+0x428>
   35c22:	49 8d 74 24 09       	lea    0x9(%r12),%rsi
   35c27:	4d 89 f9             	mov    %r15,%r9
   35c2a:	49 29 f1             	sub    %rsi,%r9
   35c2d:	49 83 f9 08          	cmp    $0x8,%r9
   35c31:	7d 7d                	jge    35cb0 <Data.Bytes.View.byte-view-index-byte+0x440>
   35c33:	48 c7 c3 ff ff ff ff 	mov    $0xffffffffffffffff,%rbx
   35c3a:	49 39 f7             	cmp    %rsi,%r15
   35c3d:	0f 8e 8e fe ff ff    	jle    35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35c43:	48 89 f3             	mov    %rsi,%rbx
   35c46:	38 0c 30             	cmp    %cl,(%rax,%rsi,1)
   35c49:	0f 84 82 fe ff ff    	je     35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35c4f:	49 8d 74 24 0a       	lea    0xa(%r12),%rsi
   35c54:	e9 83 00 00 00       	jmp    35cdc <Data.Bytes.View.byte-view-index-byte+0x46c>
   35c59:	4c 89 f7             	mov    %r14,%rdi
   35c5c:	e8 cf dc 04 00       	call   83930 <cinch_drop_mt>
   35c61:	48 8d 05 b8 2d 10 00 	lea    0x102db8(%rip),%rax        # 138a20 <Effect.Process.ProcessInvalidArgument.blk#h>
   35c68:	4c 29 e3             	sub    %r12,%rbx
   35c6b:	7c 1c                	jl     35c89 <Data.Bytes.View.byte-view-index-byte+0x419>
   35c6d:	78 1a                	js     35c89 <Data.Bytes.View.byte-view-index-byte+0x419>
   35c6f:	48 8d 1c 5d 01 00 00 	lea    0x1(,%rbx,2),%rbx
   35c76:	00
   35c77:	bf 08 00 00 00       	mov    $0x8,%edi
   35c7c:	be 01 01 00 00       	mov    $0x101,%esi
   35c81:	e8 6a b7 04 00       	call   813f0 <cinch_alloc_ctor>
   35c86:	48 89 18             	mov    %rbx,(%rax)
   35c89:	48 83 c4 18          	add    $0x18,%rsp
   35c8d:	5b                   	pop    %rbx
   35c8e:	41 5c                	pop    %r12
   35c90:	41 5d                	pop    %r13
   35c92:	41 5e                	pop    %r14
   35c94:	41 5f                	pop    %r15
   35c96:	5d                   	pop    %rbp
   35c97:	c3                   	ret
   35c98:	48 31 d6             	xor    %rdx,%rsi
   35c9b:	b8 40 00 00 00       	mov    $0x40,%eax
   35ca0:	f3 48 0f bc c6       	tzcnt  %rsi,%rax
   35ca5:	c1 e8 03             	shr    $0x3,%eax
   35ca8:	48 01 c3             	add    %rax,%rbx
   35cab:	e9 21 fe ff ff       	jmp    35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35cb0:	4c 8b 14 30          	mov    (%rax,%rsi,1),%r10
   35cb4:	49 31 ca             	xor    %rcx,%r10
   35cb7:	4d 89 d1             	mov    %r10,%r9
   35cba:	49 21 f9             	and    %rdi,%r9
   35cbd:	49 01 f9             	add    %rdi,%r9
   35cc0:	4d 09 d1             	or     %r10,%r9
   35cc3:	49 21 d1             	and    %rdx,%r9
   35cc6:	49 39 d1             	cmp    %rdx,%r9
   35cc9:	75 27                	jne    35cf2 <Data.Bytes.View.byte-view-index-byte+0x482>
   35ccb:	49 8d 74 24 11       	lea    0x11(%r12),%rsi
   35cd0:	4d 89 f9             	mov    %r15,%r9
   35cd3:	49 29 f1             	sub    %rsi,%r9
   35cd6:	49 83 f9 08          	cmp    $0x8,%r9
   35cda:	7d 2e                	jge    35d0a <Data.Bytes.View.byte-view-index-byte+0x49a>
   35cdc:	48 89 c7             	mov    %rax,%rdi
   35cdf:	4c 89 fa             	mov    %r15,%rdx
   35ce2:	4c 89 c1             	mov    %r8,%rcx
   35ce5:	e8 26 ee ff ff       	call   34b10 <Data.Bytes.View.index-forward-tail>
   35cea:	48 89 c3             	mov    %rax,%rbx
   35ced:	e9 df fd ff ff       	jmp    35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35cf2:	49 31 d1             	xor    %rdx,%r9
   35cf5:	bb 40 00 00 00       	mov    $0x40,%ebx
   35cfa:	f3 49 0f bc d9       	tzcnt  %r9,%rbx
   35cff:	c1 eb 03             	shr    $0x3,%ebx
   35d02:	48 01 f3             	add    %rsi,%rbx
   35d05:	e9 c7 fd ff ff       	jmp    35ad1 <Data.Bytes.View.byte-view-index-byte+0x261>
   35d0a:	4c 8b 0c 30          	mov    (%rax,%rsi,1),%r9
   35d0e:	49 31 c9             	xor    %rcx,%r9
   35d11:	4d 89 c8             	mov    %r9,%r8
   35d14:	49 21 f8             	and    %rdi,%r8
   35d17:	49 01 f8             	add    %rdi,%r8
   35d1a:	4d 09 c8             	or     %r9,%r8
   35d1d:	49 21 d0             	and    %rdx,%r8
   35d20:	49 39 d0             	cmp    %rdx,%r8
   35d23:	75 12                	jne    35d37 <Data.Bytes.View.byte-view-index-byte+0x4c7>
   35d25:	49 8d 74 24 19       	lea    0x19(%r12),%rsi
   35d2a:	48 89 c7             	mov    %rax,%rdi
   35d2d:	4c 89 fa             	mov    %r15,%rdx
   35d30:	e8 2b ec ff ff       	call   34960 <Data.Bytes.View.index-forward-blocks>
   35d35:	eb b3                	jmp    35cea <Data.Bytes.View.byte-view-index-byte+0x47a>
   35d37:	49 31 d0             	xor    %rdx,%r8
   35d3a:	bb 40 00 00 00       	mov    $0x40,%ebx
   35d3f:	f3 49 0f bc d8       	tzcnt  %r8,%rbx
   35d44:	eb b9                	jmp    35cff <Data.Bytes.View.byte-view-index-byte+0x48f>

Disassembly of section .fini:
