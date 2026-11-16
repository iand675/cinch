
/workspace/dasl-stdlib-strings/build/byte-search-benchmark-candidate1/rust/initial/consumer:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000024280 <Data.Bytes.Search.count-forward>:
   24280:	55                   	push   %rbp
   24281:	48 89 e5             	mov    %rsp,%rbp
   24284:	41 57                	push   %r15
   24286:	41 56                	push   %r14
   24288:	41 55                	push   %r13
   2428a:	41 54                	push   %r12
   2428c:	53                   	push   %rbx
   2428d:	48 89 d3             	mov    %rdx,%rbx
   24290:	48 29 f3             	sub    %rsi,%rbx
   24293:	48 83 fb 08          	cmp    $0x8,%rbx
   24297:	0f 8c 85 01 00 00    	jl     24422 <Data.Bytes.Search.count-forward+0x1a2>
   2429d:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
   242a1:	48 ba 80 80 80 80 80 	movabs $0x8080808080808080,%rdx
   242a8:	80 80 80 
   242ab:	49 bf 7f 7f 7f 7f 7f 	movabs $0x7f7f7f7f7f7f7f7f,%r15
   242b2:	7f 7f 7f 
   242b5:	48 89 7d d0          	mov    %rdi,-0x30(%rbp)
   242b9:	4c 8d 57 10          	lea    0x10(%rdi),%r10
   242bd:	45 31 db             	xor    %r11d,%r11d
   242c0:	49 bc 55 55 55 55 55 	movabs $0x5555555555555555,%r12
   242c7:	55 55 55 
   242ca:	49 bd 33 33 33 33 33 	movabs $0x3333333333333333,%r13
   242d1:	33 33 33 
   242d4:	49 be 0f 0f 0f 0f 0f 	movabs $0xf0f0f0f0f0f0f0f,%r14
   242db:	0f 0f 0f 
   242de:	48 bf 01 01 01 01 01 	movabs $0x101010101010101,%rdi
   242e5:	01 01 01 
   242e8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   242ef:	00 
   242f0:	49 8b 44 32 f0       	mov    -0x10(%r10,%rsi,1),%rax
   242f5:	48 31 c8             	xor    %rcx,%rax
   242f8:	49 89 c1             	mov    %rax,%r9
   242fb:	4d 21 f9             	and    %r15,%r9
   242fe:	4d 01 f9             	add    %r15,%r9
   24301:	49 09 c1             	or     %rax,%r9
   24304:	49 f7 d1             	not    %r9
   24307:	49 21 d1             	and    %rdx,%r9
   2430a:	4c 89 c8             	mov    %r9,%rax
   2430d:	48 d1 e8             	shr    $1,%rax
   24310:	4c 21 e0             	and    %r12,%rax
   24313:	49 29 c1             	sub    %rax,%r9
   24316:	4c 89 c8             	mov    %r9,%rax
   24319:	4c 21 e8             	and    %r13,%rax
   2431c:	49 c1 e9 02          	shr    $0x2,%r9
   24320:	4d 21 e9             	and    %r13,%r9
   24323:	49 01 c1             	add    %rax,%r9
   24326:	4c 89 c8             	mov    %r9,%rax
   24329:	48 c1 e8 04          	shr    $0x4,%rax
   2432d:	4c 01 c8             	add    %r9,%rax
   24330:	4c 21 f0             	and    %r14,%rax
   24333:	48 0f af c7          	imul   %rdi,%rax
   24337:	48 c1 e8 38          	shr    $0x38,%rax
   2433b:	4c 01 c0             	add    %r8,%rax
   2433e:	4e 8d 04 1b          	lea    (%rbx,%r11,1),%r8
   24342:	49 83 c0 f8          	add    $0xfffffffffffffff8,%r8
   24346:	49 83 f8 08          	cmp    $0x8,%r8
   2434a:	0f 8c 13 01 00 00    	jl     24463 <Data.Bytes.Search.count-forward+0x1e3>
   24350:	4d 8b 4c 32 f8       	mov    -0x8(%r10,%rsi,1),%r9
   24355:	49 31 c9             	xor    %rcx,%r9
   24358:	4d 89 c8             	mov    %r9,%r8
   2435b:	4d 21 f8             	and    %r15,%r8
   2435e:	4d 01 f8             	add    %r15,%r8
   24361:	4d 09 c8             	or     %r9,%r8
   24364:	49 f7 d0             	not    %r8
   24367:	49 21 d0             	and    %rdx,%r8
   2436a:	4d 89 c1             	mov    %r8,%r9
   2436d:	49 d1 e9             	shr    $1,%r9
   24370:	4d 21 e1             	and    %r12,%r9
   24373:	4d 29 c8             	sub    %r9,%r8
   24376:	4d 89 c1             	mov    %r8,%r9
   24379:	4d 21 e9             	and    %r13,%r9
   2437c:	49 c1 e8 02          	shr    $0x2,%r8
   24380:	4d 21 e8             	and    %r13,%r8
   24383:	4d 01 c8             	add    %r9,%r8
   24386:	4d 89 c1             	mov    %r8,%r9
   24389:	49 c1 e9 04          	shr    $0x4,%r9
   2438d:	4d 01 c1             	add    %r8,%r9
   24390:	4d 21 f1             	and    %r14,%r9
   24393:	4c 0f af cf          	imul   %rdi,%r9
   24397:	49 c1 e9 38          	shr    $0x38,%r9
   2439b:	49 01 c1             	add    %rax,%r9
   2439e:	4a 8d 04 1b          	lea    (%rbx,%r11,1),%rax
   243a2:	48 83 c0 f0          	add    $0xfffffffffffffff0,%rax
   243a6:	48 83 f8 08          	cmp    $0x8,%rax
   243aa:	0f 8c e7 00 00 00    	jl     24497 <Data.Bytes.Search.count-forward+0x217>
   243b0:	4d 8b 04 32          	mov    (%r10,%rsi,1),%r8
   243b4:	49 31 c8             	xor    %rcx,%r8
   243b7:	4c 89 c0             	mov    %r8,%rax
   243ba:	4c 21 f8             	and    %r15,%rax
   243bd:	4c 01 f8             	add    %r15,%rax
   243c0:	4c 09 c0             	or     %r8,%rax
   243c3:	48 f7 d0             	not    %rax
   243c6:	48 21 d0             	and    %rdx,%rax
   243c9:	49 89 c0             	mov    %rax,%r8
   243cc:	49 d1 e8             	shr    $1,%r8
   243cf:	4d 21 e0             	and    %r12,%r8
   243d2:	4c 29 c0             	sub    %r8,%rax
   243d5:	49 89 c0             	mov    %rax,%r8
   243d8:	4d 21 e8             	and    %r13,%r8
   243db:	48 c1 e8 02          	shr    $0x2,%rax
   243df:	4c 21 e8             	and    %r13,%rax
   243e2:	4c 01 c0             	add    %r8,%rax
   243e5:	49 89 c0             	mov    %rax,%r8
   243e8:	49 c1 e8 04          	shr    $0x4,%r8
   243ec:	49 01 c0             	add    %rax,%r8
   243ef:	4d 21 f0             	and    %r14,%r8
   243f2:	4c 0f af c7          	imul   %rdi,%r8
   243f6:	49 c1 e8 38          	shr    $0x38,%r8
   243fa:	4d 01 c8             	add    %r9,%r8
   243fd:	49 83 c2 18          	add    $0x18,%r10
   24401:	4a 8d 04 1b          	lea    (%rbx,%r11,1),%rax
   24405:	48 83 c0 e8          	add    $0xffffffffffffffe8,%rax
   24409:	49 83 c3 e8          	add    $0xffffffffffffffe8,%r11
   2440d:	48 83 f8 08          	cmp    $0x8,%rax
   24411:	0f 8d d9 fe ff ff    	jge    242f0 <Data.Bytes.Search.count-forward+0x70>
   24417:	4c 29 de             	sub    %r11,%rsi
   2441a:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   2441e:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   24422:	48 39 d6             	cmp    %rdx,%rsi
   24425:	7d 2e                	jge    24455 <Data.Bytes.Search.count-forward+0x1d5>
   24427:	31 c0                	xor    %eax,%eax
   24429:	38 0c 37             	cmp    %cl,(%rdi,%rsi,1)
   2442c:	0f 94 c0             	sete   %al
   2442f:	49 01 c0             	add    %rax,%r8
   24432:	48 8d 46 01          	lea    0x1(%rsi),%rax
   24436:	48 39 d0             	cmp    %rdx,%rax
   24439:	7d 1a                	jge    24455 <Data.Bytes.Search.count-forward+0x1d5>
   2443b:	0f b6 c1             	movzbl %cl,%eax
   2443e:	45 31 c9             	xor    %r9d,%r9d
   24441:	38 4c 37 01          	cmp    %cl,0x1(%rdi,%rsi,1)
   24445:	41 0f 94 c1          	sete   %r9b
   24449:	4d 01 c8             	add    %r9,%r8
   2444c:	48 83 c6 02          	add    $0x2,%rsi
   24450:	48 89 c1             	mov    %rax,%rcx
   24453:	eb 57                	jmp    244ac <Data.Bytes.Search.count-forward+0x22c>
   24455:	4c 89 c0             	mov    %r8,%rax
   24458:	5b                   	pop    %rbx
   24459:	41 5c                	pop    %r12
   2445b:	41 5d                	pop    %r13
   2445d:	41 5e                	pop    %r14
   2445f:	41 5f                	pop    %r15
   24461:	5d                   	pop    %rbp
   24462:	c3                   	ret
   24463:	49 89 f0             	mov    %rsi,%r8
   24466:	4d 29 d8             	sub    %r11,%r8
   24469:	49 8d 78 08          	lea    0x8(%r8),%rdi
   2446d:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   24471:	48 39 d7             	cmp    %rdx,%rdi
   24474:	7d e2                	jge    24458 <Data.Bytes.Search.count-forward+0x1d8>
   24476:	0f b6 c9             	movzbl %cl,%ecx
   24479:	31 ff                	xor    %edi,%edi
   2447b:	41 38 4c 32 f8       	cmp    %cl,-0x8(%r10,%rsi,1)
   24480:	40 0f 94 c7          	sete   %dil
   24484:	48 01 f8             	add    %rdi,%rax
   24487:	49 83 c0 09          	add    $0x9,%r8
   2448b:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   2448f:	4c 89 c6             	mov    %r8,%rsi
   24492:	49 89 c0             	mov    %rax,%r8
   24495:	eb 15                	jmp    244ac <Data.Bytes.Search.count-forward+0x22c>
   24497:	0f b6 c9             	movzbl %cl,%ecx
   2449a:	4c 29 de             	sub    %r11,%rsi
   2449d:	48 83 c6 10          	add    $0x10,%rsi
   244a1:	48 8b 7d d0          	mov    -0x30(%rbp),%rdi
   244a5:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   244a9:	4d 89 c8             	mov    %r9,%r8
   244ac:	5b                   	pop    %rbx
   244ad:	41 5c                	pop    %r12
   244af:	41 5d                	pop    %r13
   244b1:	41 5e                	pop    %r14
   244b3:	41 5f                	pop    %r15
   244b5:	5d                   	pop    %rbp
   244b6:	e9 05 00 00 00       	jmp    244c0 <Data.Bytes.Search.count-tail>

Disassembly of section .fini:
