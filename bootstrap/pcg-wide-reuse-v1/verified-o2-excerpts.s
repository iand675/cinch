
<verified archived ELF in memory>:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000001e5f0 <Data.Random.Pcg.pcg-next#cc>:
   1e5f0:	mov    -0x8(%rdi),%rax
   1e5f4:	shr    $0x28,%rax
   1e5f8:	movzbl %al,%eax
   1e5fb:	cmp    $0xff,%rax
   1e601:	jne    1e60a <Data.Random.Pcg.pcg-next#cc+0x1a>
   1e603:	mov    -0x10(%rdi),%rax
   1e607:	mov    (%rax),%rax
   1e60a:	push   %rbp
   1e60b:	mov    %rsp,%rbp
   1e60e:	push   %r15
   1e610:	push   %r14
   1e612:	push   %rbx
   1e613:	push   %rax
   1e614:	xor    %ebx,%ebx
   1e616:	mov    $0x0,%edx
   1e61b:	test   %rax,%rax
   1e61e:	jne    1e6ad <Data.Random.Pcg.pcg-next#cc+0xbd>
   1e624:	mov    (%rdi),%rcx
   1e627:	movabs $0x2360ed051fc65da4,%rsi
   1e631:	movabs $0x4385df649fccf645,%r8
   1e63b:	mov    %rcx,%rax
   1e63e:	mul    %r8
   1e641:	imul   %rcx,%rsi
   1e645:	imul   0x8(%rdi),%r8
   1e64a:	add    %rsi,%rdx
   1e64d:	add    %rdx,%r8
   1e650:	movabs $0x14057b7ef767814f,%r14
   1e65a:	add    %rax,%r14
   1e65d:	movabs $0x5851f42d4c957f2d,%r15
   1e667:	adc    %r8,%r15
   1e66a:	mov    %r15,%rax
   1e66d:	shr    $0x20,%rax
   1e671:	xor    %r15,%rax
   1e674:	movabs $0xda942042e4dd58b5,%rcx
   1e67e:	imul   %rax,%rcx
   1e682:	mov    %rcx,%rax
   1e685:	shr    $0x30,%rax
   1e689:	xor    %rcx,%rax
   1e68c:	mov    %r14,%rbx
   1e68f:	or     $0x1,%rbx
   1e693:	imul   %rax,%rbx
   1e697:	mov    $0x10,%edi
   1e69c:	xor    %esi,%esi
   1e69e:	call   679b0 <cinch_alloc_ctor>
   1e6a3:	mov    %rax,%rdx
   1e6a6:	mov    %r14,(%rax)
   1e6a9:	mov    %r15,0x8(%rax)
   1e6ad:	mov    %rbx,%rax
   1e6b0:	add    $0x8,%rsp
   1e6b4:	pop    %rbx
   1e6b5:	pop    %r14
   1e6b7:	pop    %r15
   1e6b9:	pop    %rbp
   1e6ba:	ret

Disassembly of section .fini:


<verified archived ELF in memory>:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000001f3d0 <Data.Random.Pcg.PcgState>:
   1f3d0:	push   %rbp
   1f3d1:	mov    %rsp,%rbp
   1f3d4:	push   %r14
   1f3d6:	push   %rbx
   1f3d7:	mov    %rsi,%rbx
   1f3da:	mov    %rdi,%r14
   1f3dd:	mov    $0x10,%edi
   1f3e2:	xor    %esi,%esi
   1f3e4:	call   679b0 <cinch_alloc_ctor>
   1f3e9:	mov    %rbx,0x8(%rax)
   1f3ed:	mov    %r14,(%rax)
   1f3f0:	pop    %rbx
   1f3f1:	pop    %r14
   1f3f3:	pop    %rbp
   1f3f4:	ret

Disassembly of section .fini:


<verified archived ELF in memory>:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000001f400 <Main.check-values>:
   1f400:	push   %rbp
   1f401:	mov    %rsp,%rbp
   1f404:	push   %r15
   1f406:	push   %r14
   1f408:	push   %r13
   1f40a:	push   %r12
   1f40c:	push   %rbx
   1f40d:	sub    $0x28,%rsp
   1f411:	mov    %rdx,-0x30(%rbp)
   1f415:	mov    %rsi,%r14
   1f418:	movabs $0x7fffffff9fffffff,%r13
   1f422:	mov    -0x8(%rdi),%rax
   1f426:	shr    $0x28,%rax
   1f42a:	movzbl %al,%eax
   1f42d:	cmp    $0xff,%rax
   1f433:	jne    1f43c <Main.check-values+0x3c>
   1f435:	mov    -0x10(%rdi),%rax
   1f439:	mov    (%rax),%rax
   1f43c:	cmp    $0x1,%rax
   1f440:	jne    1fa87 <Main.check-values+0x687>
   1f446:	mov    (%rdi),%r12
   1f449:	mov    0x8(%rdi),%rbx
   1f44d:	mov    -0x8(%r14),%rax
   1f451:	shr    $0x28,%rax
   1f455:	movzbl %al,%eax
   1f458:	cmp    $0xff,%rax
   1f45e:	jne    1f467 <Main.check-values+0x67>
   1f460:	mov    -0x10(%r14),%rax
   1f464:	mov    (%rax),%rax
   1f467:	test   %rax,%rax
   1f46a:	jne    1fd86 <Main.check-values+0x986>
   1f470:	mov    (%r14),%r15
   1f473:	mov    0x8(%r14),%r13
   1f477:	test   %r14,%r14
   1f47a:	je     1f530 <Main.check-values+0x130>
   1f480:	mov    -0x8(%r14),%rax
   1f484:	movabs $0x7ffffffff,%rcx
   1f48e:	and    %rcx,%rax
   1f491:	cmp    $0x1,%rax
   1f495:	jne    1f4b0 <Main.check-values+0xb0>
   1f497:	mov    %r14,%rdi
   1f49a:	call   781a0 <cinch_free>
   1f49f:	jmp    1f530 <Main.check-values+0x130>
   1f4a4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   1f4b0:	mov    -0x8(%r14),%rcx
   1f4b4:	mov    %rcx,%rax
   1f4b7:	shl    $0x1d,%rax
   1f4bb:	lea    -0x20000001(%rax),%rdx
   1f4c2:	movabs $0x7fffffff9fffffff,%rsi
   1f4cc:	cmp    %rsi,%rdx
   1f4cf:	jae    1f4da <Main.check-values+0xda>
   1f4d1:	dec    %rcx
   1f4d4:	mov    %rcx,-0x8(%r14)
   1f4d8:	jmp    1f530 <Main.check-values+0x130>
   1f4da:	movabs $0x7fffffffbfffffff,%rcx
   1f4e4:	cmp    %rcx,%rax
   1f4e7:	jg     1f500 <Main.check-values+0x100>
   1f4e9:	test   %rax,%rax
   1f4ec:	je     1f530 <Main.check-values+0x130>
   1f4ee:	cmp    $0x20000000,%rax
   1f4f4:	jne    1f528 <Main.check-values+0x128>
   1f4f6:	mov    %r14,%rdi
   1f4f9:	call   6b090 <cinch_drop_slow>
   1f4fe:	jmp    1f530 <Main.check-values+0x130>
   1f500:	movabs $0x7fffffffc0000000,%rcx
   1f50a:	cmp    %rcx,%rax
   1f50d:	je     1f51e <Main.check-values+0x11e>
   1f50f:	movabs $0x7fffffffe0000000,%rcx
   1f519:	cmp    %rcx,%rax
   1f51c:	jne    1f528 <Main.check-values+0x128>
   1f51e:	mov    %r14,%rdi
   1f521:	call   77fd0 <cinch_frame_release>
   1f526:	jmp    1f530 <Main.check-values+0x130>
   1f528:	mov    %r14,%rdi
   1f52b:	call   6afa0 <cinch_drop_mt>
   1f530:	mov    %r15,%rax
   1f533:	movabs $0x4385df649fccf645,%rcx
   1f53d:	mul    %rcx
   1f540:	mov    %rax,%r14
   1f543:	movabs $0x2360ed051fc65da4,%rax
   1f54d:	imul   %rax,%r15
   1f551:	add    %rdx,%r15
   1f554:	imul   %rcx,%r13
   1f558:	add    %r15,%r13
   1f55b:	movabs $0x14057b7ef767814f,%rax
   1f565:	add    %rax,%r14
   1f568:	movabs $0x5851f42d4c957f2d,%rax
   1f572:	adc    %rax,%r13
   1f575:	mov    %r13,%rax
   1f578:	shr    $0x20,%rax
   1f57c:	xor    %r13,%rax
   1f57f:	movabs $0xda942042e4dd58b5,%rcx
   1f589:	imul   %rcx,%rax
   1f58d:	mov    %rax,%rcx
   1f590:	shr    $0x30,%rcx
   1f594:	xor    %rax,%rcx
   1f597:	mov    %r14,%r15
   1f59a:	or     $0x1,%r15
   1f59e:	imul   %rcx,%r15
   1f5a2:	mov    $0x10,%edi
   1f5a7:	xor    %esi,%esi
   1f5a9:	call   679b0 <cinch_alloc_ctor>
   1f5ae:	mov    %r14,(%rax)
   1f5b1:	mov    %r13,0x8(%rax)
   1f5b5:	cmp    %r12,%r15
   1f5b8:	jne    1fb63 <Main.check-values+0x763>
   1f5be:	mov    -0x8(%rbx),%rcx

Disassembly of section .fini:


<verified archived ELF in memory>:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

000000000001e7a0 <Data.Random.Pcg.pcg-encode>:
   1e801:	call   7e430 <cinch_nat_to_len>
   1e809:	call   69b40 <cinch_bytes_new>
   1e83a:	call   69b00 <cinch_bytes_len>
   1e851:	call   69bd0 <cinch_bytes_set>
   1e862:	call   69bd0 <cinch_bytes_set>
   1e873:	call   69bd0 <cinch_bytes_set>
   1e880:	call   69b00 <cinch_bytes_len>
   1e8c1:	call   69bd0 <cinch_bytes_set>
   1e8ef:	call   69bd0 <cinch_bytes_set>
   1e91d:	call   69bd0 <cinch_bytes_set>
   1e957:	call   69bd0 <cinch_bytes_set>
   1e99c:	call   69bd0 <cinch_bytes_set>
   1e9d1:	call   69bd0 <cinch_bytes_set>
   1ea06:	call   69bd0 <cinch_bytes_set>
   1ea3b:	call   69bd0 <cinch_bytes_set>
   1ea70:	call   69bd0 <cinch_bytes_set>
   1eaa5:	call   69bd0 <cinch_bytes_set>
   1eada:	call   69bd0 <cinch_bytes_set>
   1eb20:	call   69bd0 <cinch_bytes_set>
   1eb33:	call   69bd0 <cinch_bytes_set>
   1eb49:	call   69bd0 <cinch_bytes_set>
   1eb5c:	call   69bd0 <cinch_bytes_set>
   1eb73:	call   69bd0 <cinch_bytes_set>
   1eb8a:	call   69bd0 <cinch_bytes_set>
   1eba1:	call   69bd0 <cinch_bytes_set>
   1ebb5:	call   69bd0 <cinch_bytes_set>
   1ebfa:	call   69bd0 <cinch_bytes_set>
   1ec2f:	call   69bd0 <cinch_bytes_set>
   1ec64:	call   69bd0 <cinch_bytes_set>
   1ec99:	call   69bd0 <cinch_bytes_set>
   1ecce:	call   69bd0 <cinch_bytes_set>
   1ed03:	call   69bd0 <cinch_bytes_set>
   1ed38:	call   69bd0 <cinch_bytes_set>
   1ed72:	call   69bd0 <cinch_bytes_set>
   1ed85:	call   69bd0 <cinch_bytes_set>
   1ed9b:	call   69bd0 <cinch_bytes_set>
   1edae:	call   69bd0 <cinch_bytes_set>
   1edc5:	call   69bd0 <cinch_bytes_set>
   1eddc:	call   69bd0 <cinch_bytes_set>
   1edf3:	call   69bd0 <cinch_bytes_set>
   1ee07:	call   69bd0 <cinch_bytes_set>

Disassembly of section .fini:
