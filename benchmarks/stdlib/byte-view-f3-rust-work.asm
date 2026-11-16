
build/byte-view-bench/candidate3-rust/packed/consumer:     file format elf64-x86-64


Disassembly of section .text:

0000000000032100 <Main.work>:
   32100:	55                   	push   rbp
   32101:	48 89 e5             	mov    rbp,rsp
   32104:	41 57                	push   r15
   32106:	41 56                	push   r14
   32108:	41 55                	push   r13
   3210a:	41 54                	push   r12
   3210c:	53                   	push   rbx
   3210d:	48 83 ec 48          	sub    rsp,0x48
   32111:	4d 89 cc             	mov    r12,r9
   32114:	4c 89 45 98          	mov    QWORD PTR [rbp-0x68],r8
   32118:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
   3211c:	48 89 55 a0          	mov    QWORD PTR [rbp-0x60],rdx
   32120:	48 89 75 a8          	mov    QWORD PTR [rbp-0x58],rsi
   32124:	48 89 7d b0          	mov    QWORD PTR [rbp-0x50],rdi
   32128:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   3212c:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   32130:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   32134:	eb 14                	jmp    3214a <Main.work+0x4a>
   32136:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3213d:	00 00 00 
   32140:	48 ff ca             	dec    rdx
   32143:	49 89 57 f8          	mov    QWORD PTR [r15-0x8],rdx
   32147:	4d 89 ec             	mov    r12,r13
   3214a:	49 89 c7             	mov    r15,rax
   3214d:	4d 85 e4             	test   r12,r12
   32150:	0f 94 c0             	sete   al
   32153:	45 89 e6             	mov    r14d,r12d
   32156:	41 08 c6             	or     r14b,al
   32159:	41 f6 c6 01          	test   r14b,0x1
   3215d:	74 21                	je     32180 <Main.work+0x80>
   3215f:	41 f6 c4 01          	test   r12b,0x1
   32163:	75 3b                	jne    321a0 <Main.work+0xa0>
   32165:	be 01 00 00 00       	mov    esi,0x1
   3216a:	4c 89 e7             	mov    rdi,r12
   3216d:	e8 7e 4f 04 00       	call   770f0 <cinch_nat_cmp>
   32172:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   32176:	e9 b5 00 00 00       	jmp    32230 <Main.work+0x130>
   3217b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   32180:	49 8b 44 24 f8       	mov    rax,QWORD PTR [r12-0x8]
   32185:	48 89 c1             	mov    rcx,rax
   32188:	48 c1 e1 1d          	shl    rcx,0x1d
   3218c:	48 85 c9             	test   rcx,rcx
   3218f:	7e 27                	jle    321b8 <Main.work+0xb8>
   32191:	48 ff c0             	inc    rax
   32194:	49 89 44 24 f8       	mov    QWORD PTR [r12-0x8],rax
   32199:	eb 27                	jmp    321c2 <Main.work+0xc2>
   3219b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   321a0:	4c 89 e0             	mov    rax,r12
   321a3:	48 d1 e8             	shr    rax,1
   321a6:	48 85 c0             	test   rax,rax
   321a9:	0f 97 c0             	seta   al
   321ac:	1c 00                	sbb    al,0x0
   321ae:	48 0f be c0          	movsx  rax,al
   321b2:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   321b6:	eb 78                	jmp    32230 <Main.work+0x130>
   321b8:	74 08                	je     321c2 <Main.work+0xc2>
   321ba:	4c 89 e7             	mov    rdi,r12
   321bd:	e8 0e fd 03 00       	call   71ed0 <cinch_dup_mt>
   321c2:	be 01 00 00 00       	mov    esi,0x1
   321c7:	4c 89 e7             	mov    rdi,r12
   321ca:	e8 21 4f 04 00       	call   770f0 <cinch_nat_cmp>
   321cf:	49 8b 54 24 f8       	mov    rdx,QWORD PTR [r12-0x8]
   321d4:	48 89 d1             	mov    rcx,rdx
   321d7:	48 c1 e1 1d          	shl    rcx,0x1d
   321db:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   321e2:	4c 8b 6d c0          	mov    r13,QWORD PTR [rbp-0x40]
   321e6:	7c 18                	jl     32200 <Main.work+0x100>
   321e8:	48 ff ca             	dec    rdx
   321eb:	49 89 54 24 f8       	mov    QWORD PTR [r12-0x8],rdx
   321f0:	eb 3e                	jmp    32230 <Main.work+0x130>
   321f2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   321f9:	1f 84 00 00 00 00 00 
   32200:	48 85 c9             	test   rcx,rcx
   32203:	74 2b                	je     32230 <Main.work+0x130>
   32205:	48 89 c3             	mov    rbx,rax
   32208:	4c 89 e7             	mov    rdi,r12
   3220b:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   32212:	75 07                	jne    3221b <Main.work+0x11b>
   32214:	e8 d7 f9 03 00       	call   71bf0 <cinch_drop_slow>
   32219:	eb 05                	jmp    32220 <Main.work+0x120>
   3221b:	e8 e0 f8 03 00       	call   71b00 <cinch_drop_mt>
   32220:	48 89 d8             	mov    rax,rbx
   32223:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   3222a:	84 00 00 00 00 00 
   32230:	48 85 c0             	test   rax,rax
   32233:	0f 84 55 03 00 00    	je     3258e <Main.work+0x48e>
   32239:	44 88 75 d7          	mov    BYTE PTR [rbp-0x29],r14b
   3223d:	4d 85 ff             	test   r15,r15
   32240:	0f 94 c0             	sete   al
   32243:	45 89 fe             	mov    r14d,r15d
   32246:	41 08 c6             	or     r14b,al
   32249:	41 f6 c6 01          	test   r14b,0x1
   3224d:	75 31                	jne    32280 <Main.work+0x180>
   3224f:	49 8b 47 f8          	mov    rax,QWORD PTR [r15-0x8]
   32253:	48 89 c1             	mov    rcx,rax
   32256:	48 c1 e1 1d          	shl    rcx,0x1d
   3225a:	48 85 c9             	test   rcx,rcx
   3225d:	7e 09                	jle    32268 <Main.work+0x168>
   3225f:	48 ff c0             	inc    rax
   32262:	49 89 47 f8          	mov    QWORD PTR [r15-0x8],rax
   32266:	eb 18                	jmp    32280 <Main.work+0x180>
   32268:	74 16                	je     32280 <Main.work+0x180>
   3226a:	4c 89 ff             	mov    rdi,r15
   3226d:	e8 5e fc 03 00       	call   71ed0 <cinch_dup_mt>
   32272:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   32279:	1f 84 00 00 00 00 00 
   32280:	48 8b 7d a8          	mov    rdi,QWORD PTR [rbp-0x58]
   32284:	48 8b 75 a0          	mov    rsi,QWORD PTR [rbp-0x60]
   32288:	4c 89 fa             	mov    rdx,r15
   3228b:	e8 f0 f2 ff ff       	call   31580 <Main.mutate>
   32290:	4d 85 ed             	test   r13,r13
   32293:	74 2b                	je     322c0 <Main.work+0x1c0>
   32295:	49 8b 45 f8          	mov    rax,QWORD PTR [r13-0x8]
   32299:	48 89 c1             	mov    rcx,rax
   3229c:	48 c1 e1 1d          	shl    rcx,0x1d
   322a0:	48 85 c9             	test   rcx,rcx
   322a3:	7e 0b                	jle    322b0 <Main.work+0x1b0>
   322a5:	48 ff c0             	inc    rax
   322a8:	49 89 45 f8          	mov    QWORD PTR [r13-0x8],rax
   322ac:	eb 12                	jmp    322c0 <Main.work+0x1c0>
   322ae:	66 90                	xchg   ax,ax
   322b0:	74 0e                	je     322c0 <Main.work+0x1c0>
   322b2:	4c 89 ef             	mov    rdi,r13
   322b5:	e8 16 fc 03 00       	call   71ed0 <cinch_dup_mt>
   322ba:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   322c0:	4c 89 7d b8          	mov    QWORD PTR [rbp-0x48],r15
   322c4:	48 8b 7d b0          	mov    rdi,QWORD PTR [rbp-0x50]
   322c8:	4c 89 ee             	mov    rsi,r13
   322cb:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
   322cf:	e8 5c f6 ff ff       	call   31930 <Main.operation>
   322d4:	49 89 c7             	mov    r15,rax
   322d7:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   322db:	89 fb                	mov    ebx,edi
   322dd:	83 e3 01             	and    ebx,0x1
   322e0:	44 85 fb             	test   ebx,r15d
   322e3:	74 2b                	je     32310 <Main.work+0x210>
   322e5:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
   322e9:	48 d1 e9             	shr    rcx,1
   322ec:	4c 89 f8             	mov    rax,r15
   322ef:	48 d1 e8             	shr    rax,1
   322f2:	48 01 c8             	add    rax,rcx
   322f5:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   322f9:	78 15                	js     32310 <Main.work+0x210>
   322fb:	4c 8d 2c 45 01 00 00 	lea    r13,[rax*2+0x1]
   32302:	00 
   32303:	eb 1a                	jmp    3231f <Main.work+0x21f>
   32305:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3230c:	00 00 00 00 
   32310:	4c 89 fe             	mov    rsi,r15
   32313:	e8 e8 4b 04 00       	call   76f00 <cinch_nat_add>
   32318:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   3231c:	49 89 c5             	mov    r13,rax
   3231f:	48 85 ff             	test   rdi,rdi
   32322:	0f 94 c0             	sete   al
   32325:	08 c3                	or     bl,al
   32327:	75 47                	jne    32370 <Main.work+0x270>
   32329:	48 8b 4f f8          	mov    rcx,QWORD PTR [rdi-0x8]
   3232d:	48 89 c8             	mov    rax,rcx
   32330:	48 c1 e0 1d          	shl    rax,0x1d
   32334:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   3233a:	7c 14                	jl     32350 <Main.work+0x250>
   3233c:	48 ff c9             	dec    rcx
   3233f:	48 89 4f f8          	mov    QWORD PTR [rdi-0x8],rcx
   32343:	eb 2b                	jmp    32370 <Main.work+0x270>
   32345:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   3234c:	00 00 00 00 
   32350:	48 85 c0             	test   rax,rax
   32353:	74 1b                	je     32370 <Main.work+0x270>
   32355:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3235b:	75 07                	jne    32364 <Main.work+0x264>
   3235d:	e8 8e f8 03 00       	call   71bf0 <cinch_drop_slow>
   32362:	eb 0c                	jmp    32370 <Main.work+0x270>
   32364:	e8 97 f7 03 00       	call   71b00 <cinch_drop_mt>
   32369:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   32370:	4d 85 ff             	test   r15,r15
   32373:	0f 94 c0             	sete   al
   32376:	44 89 f9             	mov    ecx,r15d
   32379:	08 c1                	or     cl,al
   3237b:	f6 c1 01             	test   cl,0x1
   3237e:	75 50                	jne    323d0 <Main.work+0x2d0>
   32380:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   32384:	48 89 c8             	mov    rax,rcx
   32387:	48 c1 e0 1d          	shl    rax,0x1d
   3238b:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32391:	7c 1d                	jl     323b0 <Main.work+0x2b0>
   32393:	48 ff c9             	dec    rcx
   32396:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   3239a:	41 f6 c5 01          	test   r13b,0x1
   3239e:	74 36                	je     323d6 <Main.work+0x2d6>
   323a0:	e9 9b 00 00 00       	jmp    32440 <Main.work+0x340>
   323a5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   323ac:	00 00 00 00 
   323b0:	48 85 c0             	test   rax,rax
   323b3:	74 1b                	je     323d0 <Main.work+0x2d0>
   323b5:	4c 89 ff             	mov    rdi,r15
   323b8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   323be:	75 6b                	jne    3242b <Main.work+0x32b>
   323c0:	e8 2b f8 03 00       	call   71bf0 <cinch_drop_slow>
   323c5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   323cc:	00 00 00 00 
   323d0:	41 f6 c5 01          	test   r13b,0x1
   323d4:	75 6a                	jne    32440 <Main.work+0x340>
   323d6:	be 0f 94 35 77       	mov    esi,0x7735940f
   323db:	4c 89 ef             	mov    rdi,r13
   323de:	e8 cd 56 04 00       	call   77ab0 <cinch_nat_rem>
   323e3:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   323e7:	4d 85 ed             	test   r13,r13
   323ea:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   323ee:	0f 84 7c 00 00 00    	je     32470 <Main.work+0x370>
   323f4:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   323f8:	48 89 c8             	mov    rax,rcx
   323fb:	48 c1 e0 1d          	shl    rax,0x1d
   323ff:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32405:	7c 09                	jl     32410 <Main.work+0x310>
   32407:	48 ff c9             	dec    rcx
   3240a:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   3240e:	eb 60                	jmp    32470 <Main.work+0x370>
   32410:	48 85 c0             	test   rax,rax
   32413:	74 5b                	je     32470 <Main.work+0x370>
   32415:	4c 89 ef             	mov    rdi,r13
   32418:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   3241e:	0f 85 60 01 00 00    	jne    32584 <Main.work+0x484>
   32424:	e8 c7 f7 03 00       	call   71bf0 <cinch_drop_slow>
   32429:	eb 45                	jmp    32470 <Main.work+0x370>
   3242b:	e8 d0 f6 03 00       	call   71b00 <cinch_drop_mt>
   32430:	41 f6 c5 01          	test   r13b,0x1
   32434:	74 a0                	je     323d6 <Main.work+0x2d6>
   32436:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3243d:	00 00 00 
   32440:	49 d1 ed             	shr    r13,1
   32443:	4c 89 e8             	mov    rax,r13
   32446:	48 b9 e5 8f a2 12 31 	movabs rcx,0x89705f3112a28fe5
   3244d:	5f 70 89 
   32450:	48 f7 e1             	mul    rcx
   32453:	48 c1 ea 1d          	shr    rdx,0x1d
   32457:	69 c2 07 ca 9a 3b    	imul   eax,edx,0x3b9aca07
   3245d:	41 29 c5             	sub    r13d,eax
   32460:	4a 8d 04 6d 01 00 00 	lea    rax,[r13*2+0x1]
   32467:	00 
   32468:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   3246c:	4c 8b 7d b8          	mov    r15,QWORD PTR [rbp-0x48]
   32470:	41 f6 c4 01          	test   r12b,0x1
   32474:	0f 94 c0             	sete   al
   32477:	49 83 fc 02          	cmp    r12,0x2
   3247b:	0f 92 c1             	setb   cl
   3247e:	08 c1                	or     cl,al
   32480:	74 1e                	je     324a0 <Main.work+0x3a0>
   32482:	be 03 00 00 00       	mov    esi,0x3
   32487:	4c 89 e7             	mov    rdi,r12
   3248a:	e8 61 58 04 00       	call   77cf0 <cinch_nat_sub>
   3248f:	49 89 c5             	mov    r13,rax
   32492:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   32496:	74 13                	je     324ab <Main.work+0x3ab>
   32498:	eb 56                	jmp    324f0 <Main.work+0x3f0>
   3249a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   324a0:	4d 8d 6c 24 fe       	lea    r13,[r12-0x2]
   324a5:	f6 45 d7 01          	test   BYTE PTR [rbp-0x29],0x1
   324a9:	75 45                	jne    324f0 <Main.work+0x3f0>
   324ab:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   324b0:	48 89 c8             	mov    rax,rcx
   324b3:	48 c1 e0 1d          	shl    rax,0x1d
   324b7:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   324bd:	7c 11                	jl     324d0 <Main.work+0x3d0>
   324bf:	48 ff c9             	dec    rcx
   324c2:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   324c7:	41 f6 c7 01          	test   r15b,0x1
   324cb:	75 29                	jne    324f6 <Main.work+0x3f6>
   324cd:	eb 51                	jmp    32520 <Main.work+0x420>
   324cf:	90                   	nop
   324d0:	48 85 c0             	test   rax,rax
   324d3:	74 1b                	je     324f0 <Main.work+0x3f0>
   324d5:	4c 89 e7             	mov    rdi,r12
   324d8:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   324de:	75 2b                	jne    3250b <Main.work+0x40b>
   324e0:	e8 0b f7 03 00       	call   71bf0 <cinch_drop_slow>
   324e5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   324ec:	00 00 00 00 
   324f0:	41 f6 c7 01          	test   r15b,0x1
   324f4:	74 2a                	je     32520 <Main.work+0x420>
   324f6:	4c 89 f8             	mov    rax,r15
   324f9:	48 d1 e8             	shr    rax,1
   324fc:	48 ff c0             	inc    rax
   324ff:	78 1f                	js     32520 <Main.work+0x420>
   32501:	48 8d 04 45 01 00 00 	lea    rax,[rax*2+0x1]
   32508:	00 
   32509:	eb 22                	jmp    3252d <Main.work+0x42d>
   3250b:	e8 f0 f5 03 00       	call   71b00 <cinch_drop_mt>
   32510:	41 f6 c7 01          	test   r15b,0x1
   32514:	75 e0                	jne    324f6 <Main.work+0x3f6>
   32516:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   3251d:	00 00 00 
   32520:	be 03 00 00 00       	mov    esi,0x3
   32525:	4c 89 ff             	mov    rdi,r15
   32528:	e8 d3 49 04 00       	call   76f00 <cinch_nat_add>
   3252d:	4d 89 ec             	mov    r12,r13
   32530:	41 f6 c6 01          	test   r14b,0x1
   32534:	0f 85 10 fc ff ff    	jne    3214a <Main.work+0x4a>
   3253a:	49 8b 57 f8          	mov    rdx,QWORD PTR [r15-0x8]
   3253e:	48 89 d1             	mov    rcx,rdx
   32541:	48 c1 e1 1d          	shl    rcx,0x1d
   32545:	48 81 f9 01 00 00 20 	cmp    rcx,0x20000001
   3254c:	0f 8d ee fb ff ff    	jge    32140 <Main.work+0x40>
   32552:	4d 89 ec             	mov    r12,r13
   32555:	48 85 c9             	test   rcx,rcx
   32558:	0f 84 ec fb ff ff    	je     3214a <Main.work+0x4a>
   3255e:	48 89 c3             	mov    rbx,rax
   32561:	4c 89 ff             	mov    rdi,r15
   32564:	48 81 f9 00 00 00 20 	cmp    rcx,0x20000000
   3256b:	75 07                	jne    32574 <Main.work+0x474>
   3256d:	e8 7e f6 03 00       	call   71bf0 <cinch_drop_slow>
   32572:	eb 05                	jmp    32579 <Main.work+0x479>
   32574:	e8 87 f5 03 00       	call   71b00 <cinch_drop_mt>
   32579:	4d 89 ec             	mov    r12,r13
   3257c:	48 89 d8             	mov    rax,rbx
   3257f:	e9 c6 fb ff ff       	jmp    3214a <Main.work+0x4a>
   32584:	e8 77 f5 03 00       	call   71b00 <cinch_drop_mt>
   32589:	e9 e2 fe ff ff       	jmp    32470 <Main.work+0x370>
   3258e:	4d 85 ed             	test   r13,r13
   32591:	74 37                	je     325ca <Main.work+0x4ca>
   32593:	49 8b 4d f8          	mov    rcx,QWORD PTR [r13-0x8]
   32597:	48 89 c8             	mov    rax,rcx
   3259a:	48 c1 e0 1d          	shl    rax,0x1d
   3259e:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   325a4:	7c 0f                	jl     325b5 <Main.work+0x4b5>
   325a6:	48 ff c9             	dec    rcx
   325a9:	49 89 4d f8          	mov    QWORD PTR [r13-0x8],rcx
   325ad:	41 f6 c6 01          	test   r14b,0x1
   325b1:	74 1d                	je     325d0 <Main.work+0x4d0>
   325b3:	eb 62                	jmp    32617 <Main.work+0x517>
   325b5:	48 85 c0             	test   rax,rax
   325b8:	74 10                	je     325ca <Main.work+0x4ca>
   325ba:	4c 89 ef             	mov    rdi,r13
   325bd:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   325c3:	75 40                	jne    32605 <Main.work+0x505>
   325c5:	e8 26 f6 03 00       	call   71bf0 <cinch_drop_slow>
   325ca:	41 f6 c6 01          	test   r14b,0x1
   325ce:	75 47                	jne    32617 <Main.work+0x517>
   325d0:	49 8b 4c 24 f8       	mov    rcx,QWORD PTR [r12-0x8]
   325d5:	48 89 c8             	mov    rax,rcx
   325d8:	48 c1 e0 1d          	shl    rax,0x1d
   325dc:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   325e2:	7c 0a                	jl     325ee <Main.work+0x4ee>
   325e4:	48 ff c9             	dec    rcx
   325e7:	49 89 4c 24 f8       	mov    QWORD PTR [r12-0x8],rcx
   325ec:	eb 29                	jmp    32617 <Main.work+0x517>
   325ee:	48 85 c0             	test   rax,rax
   325f1:	74 24                	je     32617 <Main.work+0x517>
   325f3:	4c 89 e7             	mov    rdi,r12
   325f6:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   325fc:	75 14                	jne    32612 <Main.work+0x512>
   325fe:	e8 ed f5 03 00       	call   71bf0 <cinch_drop_slow>
   32603:	eb 12                	jmp    32617 <Main.work+0x517>
   32605:	e8 f6 f4 03 00       	call   71b00 <cinch_drop_mt>
   3260a:	41 f6 c6 01          	test   r14b,0x1
   3260e:	74 c0                	je     325d0 <Main.work+0x4d0>
   32610:	eb 05                	jmp    32617 <Main.work+0x517>
   32612:	e8 e9 f4 03 00       	call   71b00 <cinch_drop_mt>
   32617:	4d 85 ff             	test   r15,r15
   3261a:	0f 94 c0             	sete   al
   3261d:	44 89 f9             	mov    ecx,r15d
   32620:	08 c1                	or     cl,al
   32622:	f6 c1 01             	test   cl,0x1
   32625:	75 38                	jne    3265f <Main.work+0x55f>
   32627:	49 8b 4f f8          	mov    rcx,QWORD PTR [r15-0x8]
   3262b:	48 89 c8             	mov    rax,rcx
   3262e:	48 c1 e0 1d          	shl    rax,0x1d
   32632:	48 3d 01 00 00 20    	cmp    rax,0x20000001
   32638:	7c 09                	jl     32643 <Main.work+0x543>
   3263a:	48 ff c9             	dec    rcx
   3263d:	49 89 4f f8          	mov    QWORD PTR [r15-0x8],rcx
   32641:	eb 1c                	jmp    3265f <Main.work+0x55f>
   32643:	48 85 c0             	test   rax,rax
   32646:	74 17                	je     3265f <Main.work+0x55f>
   32648:	4c 89 ff             	mov    rdi,r15
   3264b:	48 3d 00 00 00 20    	cmp    rax,0x20000000
   32651:	75 07                	jne    3265a <Main.work+0x55a>
   32653:	e8 98 f5 03 00       	call   71bf0 <cinch_drop_slow>
   32658:	eb 05                	jmp    3265f <Main.work+0x55f>
   3265a:	e8 a1 f4 03 00       	call   71b00 <cinch_drop_mt>
   3265f:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
   32663:	48 83 c4 48          	add    rsp,0x48
   32667:	5b                   	pop    rbx
   32668:	41 5c                	pop    r12
   3266a:	41 5d                	pop    r13
   3266c:	41 5e                	pop    r14
   3266e:	41 5f                	pop    r15
   32670:	5d                   	pop    rbp
   32671:	c3                   	ret
   32672:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   32679:	1f 84 00 00 00 00 00 
