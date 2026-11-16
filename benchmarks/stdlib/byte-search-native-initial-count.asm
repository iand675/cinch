
/workspace/dasl-stdlib-strings/build/byte-search-benchmark-candidate1/native/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000003ea20 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward>:
   3ea20:	55                   	push   %rbp
   3ea21:	41 57                	push   %r15
   3ea23:	41 56                	push   %r14
   3ea25:	41 55                	push   %r13
   3ea27:	41 54                	push   %r12
   3ea29:	53                   	push   %rbx
   3ea2a:	48 83 ec 18          	sub    $0x18,%rsp
   3ea2e:	4c 89 c3             	mov    %r8,%rbx
   3ea31:	49 89 cd             	mov    %rcx,%r13
   3ea34:	49 89 d6             	mov    %rdx,%r14
   3ea37:	49 89 f4             	mov    %rsi,%r12
   3ea3a:	49 89 ff             	mov    %rdi,%r15
   3ea3d:	48 89 d7             	mov    %rdx,%rdi
   3ea40:	e8 7b 9e 02 00       	call   688c0 <Data.Machine.wrapping-sub-i64>
   3ea45:	be 08 00 00 00       	mov    $0x8,%esi
   3ea4a:	48 89 c7             	mov    %rax,%rdi
   3ea4d:	e8 ee 02 03 00       	call   6ed40 <Data.Machine.ge-i64>
   3ea52:	48 85 c0             	test   %rax,%rax
   3ea55:	0f 84 ab 00 00 00    	je     3eb06 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0xe6>
   3ea5b:	48 89 5c 24 08       	mov    %rbx,0x8(%rsp)
   3ea60:	4b 8b 04 27          	mov    (%r15,%r12,1),%rax
   3ea64:	4c 31 e8             	xor    %r13,%rax
   3ea67:	48 89 c5             	mov    %rax,%rbp
   3ea6a:	48 b9 7f 7f 7f 7f 7f 	movabs $0x7f7f7f7f7f7f7f7f,%rcx
   3ea71:	7f 7f 7f 
   3ea74:	48 21 cd             	and    %rcx,%rbp
   3ea77:	48 01 cd             	add    %rcx,%rbp
   3ea7a:	48 09 c5             	or     %rax,%rbp
   3ea7d:	48 f7 d5             	not    %rbp
   3ea80:	48 b8 80 80 80 80 80 	movabs $0x8080808080808080,%rax
   3ea87:	80 80 80 
   3ea8a:	48 21 c5             	and    %rax,%rbp
   3ea8d:	e8 be bc 06 00       	call   aa750 <Data.Machine.u32-to-i64.object-closure>
   3ea92:	4c 89 f3             	mov    %r14,%rbx
   3ea95:	4d 89 fe             	mov    %r15,%r14
   3ea98:	4d 89 ef             	mov    %r13,%r15
   3ea9b:	49 89 c5             	mov    %rax,%r13
   3ea9e:	48 89 ef             	mov    %rbp,%rdi
   3eaa1:	e8 da c0 04 00       	call   8ab80 <Data.Machine.count-ones-u64>
   3eaa6:	48 89 c5             	mov    %rax,%rbp
   3eaa9:	4c 89 ef             	mov    %r13,%rdi
   3eaac:	e8 9f 5e 3f 00       	call   434950 <cinch_dup>
   3eab1:	48 89 6c 24 10       	mov    %rbp,0x10(%rsp)
   3eab6:	be 01 00 00 00       	mov    $0x1,%esi
   3eabb:	4c 89 ef             	mov    %r13,%rdi
   3eabe:	48 8d 54 24 10       	lea    0x10(%rsp),%rdx
   3eac3:	41 ff 55 00          	call   *0x0(%r13)
   3eac7:	48 89 c5             	mov    %rax,%rbp
   3eaca:	4c 89 ef             	mov    %r13,%rdi
   3eacd:	4d 89 fd             	mov    %r15,%r13
   3ead0:	4d 89 f7             	mov    %r14,%r15
   3ead3:	49 89 de             	mov    %rbx,%r14
   3ead6:	e8 35 5a 3f 00       	call   434510 <cinch_drop>
   3eadb:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   3eae0:	48 89 ee             	mov    %rbp,%rsi
   3eae3:	e8 a8 9a 02 00       	call   68590 <Data.Machine.wrapping-add-i64>
   3eae8:	48 89 c3             	mov    %rax,%rbx
   3eaeb:	be 08 00 00 00       	mov    $0x8,%esi
   3eaf0:	4c 89 e7             	mov    %r12,%rdi
   3eaf3:	e8 98 9a 02 00       	call   68590 <Data.Machine.wrapping-add-i64>
   3eaf8:	49 89 c4             	mov    %rax,%r12
   3eafb:	4c 89 f7             	mov    %r14,%rdi
   3eafe:	48 89 c6             	mov    %rax,%rsi
   3eb01:	e9 3a ff ff ff       	jmp    3ea40 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x20>
   3eb06:	be ff 00 00 00       	mov    $0xff,%esi
   3eb0b:	4c 89 ef             	mov    %r13,%rdi
   3eb0e:	e8 fd b3 04 00       	call   89f10 <Data.Machine.bit-and-u64>
   3eb13:	49 89 c5             	mov    %rax,%r13
   3eb16:	4c 89 e7             	mov    %r12,%rdi
   3eb19:	eb 18                	jmp    3eb33 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x113>
   3eb1b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   3eb20:	be 01 00 00 00       	mov    $0x1,%esi
   3eb25:	4c 89 e7             	mov    %r12,%rdi
   3eb28:	e8 63 9a 02 00       	call   68590 <Data.Machine.wrapping-add-i64>
   3eb2d:	49 89 c4             	mov    %rax,%r12
   3eb30:	48 89 c7             	mov    %rax,%rdi
   3eb33:	4c 89 f6             	mov    %r14,%rsi
   3eb36:	e8 45 f8 02 00       	call   6e380 <Data.Machine.lt-i64>
   3eb3b:	48 85 c0             	test   %rax,%rax
   3eb3e:	74 24                	je     3eb64 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x144>
   3eb40:	43 0f b6 3c 27       	movzbl (%r15,%r12,1),%edi
   3eb45:	4c 89 ee             	mov    %r13,%rsi
   3eb48:	e8 f3 d0 04 00       	call   8bc40 <Data.Machine.eq-u64>
   3eb4d:	48 85 c0             	test   %rax,%rax
   3eb50:	74 ce                	je     3eb20 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x100>
   3eb52:	be 01 00 00 00       	mov    $0x1,%esi
   3eb57:	48 89 df             	mov    %rbx,%rdi
   3eb5a:	e8 31 9a 02 00       	call   68590 <Data.Machine.wrapping-add-i64>
   3eb5f:	48 89 c3             	mov    %rax,%rbx
   3eb62:	eb bc                	jmp    3eb20 <cinch.typed.m.Data$2eBytes$2eSearch.count$2dforward+0x100>
   3eb64:	48 89 d8             	mov    %rbx,%rax
   3eb67:	48 83 c4 18          	add    $0x18,%rsp
   3eb6b:	5b                   	pop    %rbx
   3eb6c:	41 5c                	pop    %r12
   3eb6e:	41 5d                	pop    %r13
   3eb70:	41 5e                	pop    %r14
   3eb72:	41 5f                	pop    %r15
   3eb74:	5d                   	pop    %rbp
   3eb75:	c3                   	ret

Disassembly of section .fini:
