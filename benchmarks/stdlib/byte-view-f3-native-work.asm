
build/byte-view-bench/candidate3-native/packed/consumer:     file format elf64-x86-64


Disassembly of section .text:

000000000003df90 <cinch.typed.m.Main.work>:
   3df90:	eb 0e                	jmp    3dfa0 <cinch.typed.m.Main.work+0x10>
   3df92:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   3df99:	1f 84 00 00 00 00 00 
   3dfa0:	55                   	push   rbp
   3dfa1:	48 89 e5             	mov    rbp,rsp
   3dfa4:	41 57                	push   r15
   3dfa6:	41 56                	push   r14
   3dfa8:	41 55                	push   r13
   3dfaa:	41 54                	push   r12
   3dfac:	53                   	push   rbx
   3dfad:	48 83 ec 68          	sub    rsp,0x68
   3dfb1:	4d 89 cc             	mov    r12,r9
   3dfb4:	4c 89 45 c8          	mov    QWORD PTR [rbp-0x38],r8
   3dfb8:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
   3dfbc:	49 89 d5             	mov    r13,rdx
   3dfbf:	49 89 f7             	mov    r15,rsi
   3dfc2:	48 89 7d c0          	mov    QWORD PTR [rbp-0x40],rdi
   3dfc6:	48 8b 5d 18          	mov    rbx,QWORD PTR [rbp+0x18]
   3dfca:	4c 8b 75 10          	mov    r14,QWORD PTR [rbp+0x10]
   3dfce:	4c 89 cf             	mov    rdi,r9
   3dfd1:	e8 5a 2b 41 00       	call   450b30 <cinch_dup>
   3dfd6:	be 01 00 00 00       	mov    esi,0x1
   3dfdb:	4c 89 65 b8          	mov    QWORD PTR [rbp-0x48],r12
   3dfdf:	4c 89 e7             	mov    rdi,r12
   3dfe2:	e8 99 3a 12 00       	call   161a80 <Data.Nat.Eq_Nat.==>
   3dfe7:	48 85 c0             	test   rax,rax
   3dfea:	74 5e                	je     3e04a <cinch.typed.m.Main.work+0xba>
   3dfec:	48 89 df             	mov    rdi,rbx
   3dfef:	e8 3c 2b 41 00       	call   450b30 <cinch_dup>
   3dff4:	48 89 df             	mov    rdi,rbx
   3dff7:	e8 f4 26 41 00       	call   4506f0 <cinch_drop>
   3dffc:	4c 89 f7             	mov    rdi,r14
   3dfff:	e8 ec 26 41 00       	call   4506f0 <cinch_drop>
   3e004:	48 8b 7d b8          	mov    rdi,QWORD PTR [rbp-0x48]
   3e008:	e8 e3 26 41 00       	call   4506f0 <cinch_drop>
   3e00d:	48 8b 7d c8          	mov    rdi,QWORD PTR [rbp-0x38]
   3e011:	e8 da 26 41 00       	call   4506f0 <cinch_drop>
   3e016:	48 8b 7d d0          	mov    rdi,QWORD PTR [rbp-0x30]
   3e01a:	e8 d1 26 41 00       	call   4506f0 <cinch_drop>
   3e01f:	4c 89 ef             	mov    rdi,r13
   3e022:	e8 c9 26 41 00       	call   4506f0 <cinch_drop>
   3e027:	4c 89 ff             	mov    rdi,r15
   3e02a:	e8 c1 26 41 00       	call   4506f0 <cinch_drop>
   3e02f:	48 8b 7d c0          	mov    rdi,QWORD PTR [rbp-0x40]
   3e033:	e8 b8 26 41 00       	call   4506f0 <cinch_drop>
   3e038:	48 89 d8             	mov    rax,rbx
   3e03b:	48 8d 65 d8          	lea    rsp,[rbp-0x28]
   3e03f:	5b                   	pop    rbx
   3e040:	41 5c                	pop    r12
   3e042:	41 5d                	pop    r13
   3e044:	41 5e                	pop    r14
   3e046:	41 5f                	pop    r15
   3e048:	5d                   	pop    rbp
   3e049:	c3                   	ret
   3e04a:	4c 89 ff             	mov    rdi,r15
   3e04d:	4c 89 7d a0          	mov    QWORD PTR [rbp-0x60],r15
   3e051:	e8 da 2a 41 00       	call   450b30 <cinch_dup>
   3e056:	4c 89 6d 98          	mov    QWORD PTR [rbp-0x68],r13
   3e05a:	4c 89 ef             	mov    rdi,r13
   3e05d:	e8 ce 2a 41 00       	call   450b30 <cinch_dup>
   3e062:	4c 89 75 b0          	mov    QWORD PTR [rbp-0x50],r14
   3e066:	4c 89 f7             	mov    rdi,r14
   3e069:	e8 c2 2a 41 00       	call   450b30 <cinch_dup>
   3e06e:	4c 89 ff             	mov    rdi,r15
   3e071:	4c 89 ee             	mov    rsi,r13
   3e074:	4c 89 f2             	mov    rdx,r14
   3e077:	e8 f4 35 00 00       	call   41670 <cinch.typed.m.Main.mutate>
   3e07c:	4c 8b 75 c0          	mov    r14,QWORD PTR [rbp-0x40]
   3e080:	4c 89 f7             	mov    rdi,r14
   3e083:	e8 a8 2a 41 00       	call   450b30 <cinch_dup>
   3e088:	4c 8b 7d d0          	mov    r15,QWORD PTR [rbp-0x30]
   3e08c:	4c 89 ff             	mov    rdi,r15
   3e08f:	e8 9c 2a 41 00       	call   450b30 <cinch_dup>
   3e094:	4c 8b 65 c8          	mov    r12,QWORD PTR [rbp-0x38]
   3e098:	4c 89 e7             	mov    rdi,r12
   3e09b:	e8 90 2a 41 00       	call   450b30 <cinch_dup>
   3e0a0:	4c 89 f7             	mov    rdi,r14
   3e0a3:	4c 89 fe             	mov    rsi,r15
   3e0a6:	4c 89 e2             	mov    rdx,r12
   3e0a9:	e8 c2 01 00 00       	call   3e270 <cinch.typed.m.Main.operation>
   3e0ae:	49 89 c5             	mov    r13,rax
   3e0b1:	e8 ca e8 00 00       	call   4c980 <Data.Nat.nat-rem.object-closure>
   3e0b6:	49 89 c4             	mov    r12,rax
   3e0b9:	48 89 5d 80          	mov    QWORD PTR [rbp-0x80],rbx
   3e0bd:	48 89 df             	mov    rdi,rbx
   3e0c0:	e8 6b 2a 41 00       	call   450b30 <cinch_dup>
   3e0c5:	4c 89 ad 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],r13
   3e0cc:	4c 89 ef             	mov    rdi,r13
   3e0cf:	e8 5c 2a 41 00       	call   450b30 <cinch_dup>
   3e0d4:	48 89 df             	mov    rdi,rbx
   3e0d7:	4c 89 ee             	mov    rsi,r13
   3e0da:	e8 61 61 12 00       	call   164240 <Data.Nat.Add_Nat.+>
   3e0df:	48 89 e1             	mov    rcx,rsp
   3e0e2:	48 8d 51 f0          	lea    rdx,[rcx-0x10]
   3e0e6:	48 89 d4             	mov    rsp,rdx
   3e0e9:	48 89 41 f0          	mov    QWORD PTR [rcx-0x10],rax
   3e0ed:	be 01 00 00 00       	mov    esi,0x1
   3e0f2:	4c 89 e7             	mov    rdi,r12
   3e0f5:	41 ff 14 24          	call   QWORD PTR [r12]
   3e0f9:	48 89 c3             	mov    rbx,rax
   3e0fc:	48 89 85 78 ff ff ff 	mov    QWORD PTR [rbp-0x88],rax
   3e103:	48 89 c7             	mov    rdi,rax
   3e106:	e8 25 2a 41 00       	call   450b30 <cinch_dup>
   3e10b:	48 89 e0             	mov    rax,rsp
   3e10e:	48 8d 50 f0          	lea    rdx,[rax-0x10]
   3e112:	48 89 d4             	mov    rsp,rdx
   3e115:	48 c7 40 f0 0f 94 35 	mov    QWORD PTR [rax-0x10],0x7735940f
   3e11c:	77 
   3e11d:	be 01 00 00 00       	mov    esi,0x1
   3e122:	48 89 df             	mov    rdi,rbx
   3e125:	ff 13                	call   QWORD PTR [rbx]
   3e127:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
   3e12b:	48 8b 5d b8          	mov    rbx,QWORD PTR [rbp-0x48]
   3e12f:	48 89 df             	mov    rdi,rbx
   3e132:	e8 f9 29 41 00       	call   450b30 <cinch_dup>
   3e137:	be 03 00 00 00       	mov    esi,0x3
   3e13c:	48 89 df             	mov    rdi,rbx
   3e13f:	e8 ac 67 12 00       	call   1648f0 <Data.Nat.Sub_Nat.->
   3e144:	49 89 c4             	mov    r12,rax
   3e147:	48 89 45 88          	mov    QWORD PTR [rbp-0x78],rax
   3e14b:	48 8b 5d b0          	mov    rbx,QWORD PTR [rbp-0x50]
   3e14f:	48 89 df             	mov    rdi,rbx
   3e152:	e8 d9 29 41 00       	call   450b30 <cinch_dup>
   3e157:	be 03 00 00 00       	mov    esi,0x3
   3e15c:	48 89 df             	mov    rdi,rbx
   3e15f:	e8 dc 60 12 00       	call   164240 <Data.Nat.Add_Nat.+>
   3e164:	49 89 c5             	mov    r13,rax
   3e167:	48 89 45 90          	mov    QWORD PTR [rbp-0x70],rax
   3e16b:	4c 89 f7             	mov    rdi,r14
   3e16e:	e8 bd 29 41 00       	call   450b30 <cinch_dup>
   3e173:	48 8b 7d a0          	mov    rdi,QWORD PTR [rbp-0x60]
   3e177:	e8 b4 29 41 00       	call   450b30 <cinch_dup>
   3e17c:	48 8b 5d 98          	mov    rbx,QWORD PTR [rbp-0x68]
   3e180:	48 89 df             	mov    rdi,rbx
   3e183:	e8 a8 29 41 00       	call   450b30 <cinch_dup>
   3e188:	4c 8b 75 d0          	mov    r14,QWORD PTR [rbp-0x30]
   3e18c:	4c 89 f7             	mov    rdi,r14
   3e18f:	e8 9c 29 41 00       	call   450b30 <cinch_dup>
   3e194:	4c 8b 7d c8          	mov    r15,QWORD PTR [rbp-0x38]
   3e198:	4c 89 ff             	mov    rdi,r15
   3e19b:	e8 90 29 41 00       	call   450b30 <cinch_dup>
   3e1a0:	4c 89 e7             	mov    rdi,r12
   3e1a3:	e8 88 29 41 00       	call   450b30 <cinch_dup>
   3e1a8:	4c 89 ef             	mov    rdi,r13
   3e1ab:	e8 80 29 41 00       	call   450b30 <cinch_dup>
   3e1b0:	48 8b 7d a8          	mov    rdi,QWORD PTR [rbp-0x58]
   3e1b4:	e8 77 29 41 00       	call   450b30 <cinch_dup>
   3e1b9:	48 8b 7d 80          	mov    rdi,QWORD PTR [rbp-0x80]
   3e1bd:	e8 2e 25 41 00       	call   4506f0 <cinch_drop>
   3e1c2:	48 8b 7d b0          	mov    rdi,QWORD PTR [rbp-0x50]
   3e1c6:	e8 25 25 41 00       	call   4506f0 <cinch_drop>
   3e1cb:	48 8b 7d b8          	mov    rdi,QWORD PTR [rbp-0x48]
   3e1cf:	e8 1c 25 41 00       	call   4506f0 <cinch_drop>
   3e1d4:	4c 89 ff             	mov    rdi,r15
   3e1d7:	e8 14 25 41 00       	call   4506f0 <cinch_drop>
   3e1dc:	4c 89 f7             	mov    rdi,r14
   3e1df:	e8 0c 25 41 00       	call   4506f0 <cinch_drop>
   3e1e4:	48 89 df             	mov    rdi,rbx
   3e1e7:	e8 04 25 41 00       	call   4506f0 <cinch_drop>
   3e1ec:	4c 8b 75 a0          	mov    r14,QWORD PTR [rbp-0x60]
   3e1f0:	4c 89 f7             	mov    rdi,r14
   3e1f3:	e8 f8 24 41 00       	call   4506f0 <cinch_drop>
   3e1f8:	48 8b 5d c0          	mov    rbx,QWORD PTR [rbp-0x40]
   3e1fc:	48 89 df             	mov    rdi,rbx
   3e1ff:	e8 ec 24 41 00       	call   4506f0 <cinch_drop>
   3e204:	48 8b bd 70 ff ff ff 	mov    rdi,QWORD PTR [rbp-0x90]
   3e20b:	e8 e0 24 41 00       	call   4506f0 <cinch_drop>
   3e210:	48 8b bd 78 ff ff ff 	mov    rdi,QWORD PTR [rbp-0x88]
   3e217:	e8 d4 24 41 00       	call   4506f0 <cinch_drop>
   3e21c:	4c 8b 65 a8          	mov    r12,QWORD PTR [rbp-0x58]
   3e220:	4c 89 e7             	mov    rdi,r12
   3e223:	e8 c8 24 41 00       	call   4506f0 <cinch_drop>
   3e228:	4c 8b 6d 88          	mov    r13,QWORD PTR [rbp-0x78]
   3e22c:	4c 89 ef             	mov    rdi,r13
   3e22f:	e8 bc 24 41 00       	call   4506f0 <cinch_drop>
   3e234:	4c 8b 7d 90          	mov    r15,QWORD PTR [rbp-0x70]
   3e238:	4c 89 ff             	mov    rdi,r15
   3e23b:	e8 b0 24 41 00       	call   4506f0 <cinch_drop>
   3e240:	4c 89 65 18          	mov    QWORD PTR [rbp+0x18],r12
   3e244:	4c 89 7d 10          	mov    QWORD PTR [rbp+0x10],r15
   3e248:	48 89 df             	mov    rdi,rbx
   3e24b:	4c 89 f6             	mov    rsi,r14
   3e24e:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
   3e252:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   3e256:	4c 8b 45 c8          	mov    r8,QWORD PTR [rbp-0x38]
   3e25a:	4d 89 e9             	mov    r9,r13
   3e25d:	48 8d 65 d8          	lea    rsp,[rbp-0x28]
   3e261:	5b                   	pop    rbx
   3e262:	41 5c                	pop    r12
   3e264:	41 5d                	pop    r13
   3e266:	41 5e                	pop    r14
   3e268:	41 5f                	pop    r15
   3e26a:	5d                   	pop    rbp
   3e26b:	e9 30 fd ff ff       	jmp    3dfa0 <cinch.typed.m.Main.work+0x10>
