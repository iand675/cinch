
build/mime/b004-chunked/kernels.o:     file format elf64-x86-64


Disassembly of section .text:

0000000000000000 <mime_fixture_classify16>:
   0:	66 48 0f 6e ce       	movq   %rsi,%xmm1
   5:	66 48 0f 6e c7       	movq   %rdi,%xmm0
   a:	66 0f 6c c1          	punpcklqdq %xmm1,%xmm0
   e:	66 0f 6f 0d 00 00 00 	movdqa 0x0(%rip),%xmm1        # 16 <mime_fixture_classify16+0x16>
  15:	00
  16:	66 0f fc c8          	paddb  %xmm0,%xmm1
  1a:	66 0f 6f 1d 00 00 00 	movdqa 0x0(%rip),%xmm3        # 22 <mime_fixture_classify16+0x22>
  21:	00
  22:	66 0f da d9          	pminub %xmm1,%xmm3
  26:	66 0f 74 d9          	pcmpeqb %xmm1,%xmm3
  2a:	66 0f 6f 25 00 00 00 	movdqa 0x0(%rip),%xmm4        # 32 <mime_fixture_classify16+0x32>
  31:	00
  32:	66 0f db e0          	pand   %xmm0,%xmm4
  36:	66 0f 6f 0d 00 00 00 	movdqa 0x0(%rip),%xmm1        # 3e <mime_fixture_classify16+0x3e>
  3d:	00
  3e:	66 0f 74 cc          	pcmpeqb %xmm4,%xmm1
  42:	66 0f 6f 15 00 00 00 	movdqa 0x0(%rip),%xmm2        # 4a <mime_fixture_classify16+0x4a>
  49:	00
  4a:	66 0f db d0          	pand   %xmm0,%xmm2
  4e:	66 0f 6f 2d 00 00 00 	movdqa 0x0(%rip),%xmm5        # 56 <mime_fixture_classify16+0x56>
  55:	00
  56:	66 0f 74 ea          	pcmpeqb %xmm2,%xmm5
  5a:	66 0f eb e9          	por    %xmm1,%xmm5
  5e:	66 0f 6f 0d 00 00 00 	movdqa 0x0(%rip),%xmm1        # 66 <mime_fixture_classify16+0x66>
  65:	00
  66:	66 0f 74 c8          	pcmpeqb %xmm0,%xmm1
  6a:	66 0f 6f 35 00 00 00 	movdqa 0x0(%rip),%xmm6        # 72 <mime_fixture_classify16+0x72>
  71:	00
  72:	66 0f 74 f0          	pcmpeqb %xmm0,%xmm6
  76:	66 0f 74 25 00 00 00 	pcmpeqb 0x0(%rip),%xmm4        # 7e <mime_fixture_classify16+0x7e>
  7d:	00
  7e:	66 0f eb f1          	por    %xmm1,%xmm6
  82:	66 0f eb e6          	por    %xmm6,%xmm4
  86:	66 0f eb e5          	por    %xmm5,%xmm4
  8a:	66 0f 6f 0d 00 00 00 	movdqa 0x0(%rip),%xmm1        # 92 <mime_fixture_classify16+0x92>
  91:	00
  92:	66 0f 74 c8          	pcmpeqb %xmm0,%xmm1
  96:	66 0f 6f 2d 00 00 00 	movdqa 0x0(%rip),%xmm5        # 9e <mime_fixture_classify16+0x9e>
  9d:	00
  9e:	66 0f 74 e8          	pcmpeqb %xmm0,%xmm5
  a2:	66 0f 6f 35 00 00 00 	movdqa 0x0(%rip),%xmm6        # aa <mime_fixture_classify16+0xaa>
  a9:	00
  aa:	66 0f fc f0          	paddb  %xmm0,%xmm6
  ae:	66 44 0f 6f 05 00 00 	movdqa 0x0(%rip),%xmm8        # b7 <mime_fixture_classify16+0xb7>
  b5:	00 00
  b7:	66 44 0f da c6       	pminub %xmm6,%xmm8
  bc:	66 44 0f 74 c6       	pcmpeqb %xmm6,%xmm8
  c1:	66 44 0f eb c5       	por    %xmm5,%xmm8
  c6:	66 44 0f eb c4       	por    %xmm4,%xmm8
  cb:	66 0f 6f 35 00 00 00 	movdqa 0x0(%rip),%xmm6        # d3 <mime_fixture_classify16+0xd3>
  d2:	00
  d3:	66 0f 74 f0          	pcmpeqb %xmm0,%xmm6
  d7:	66 0f 6f e6          	movdqa %xmm6,%xmm4
  db:	66 0f eb e1          	por    %xmm1,%xmm4
  df:	66 0f 6f 3d 00 00 00 	movdqa 0x0(%rip),%xmm7        # e7 <mime_fixture_classify16+0xe7>
  e6:	00
  e7:	66 0f 74 f8          	pcmpeqb %xmm0,%xmm7
  eb:	66 0f eb e7          	por    %xmm7,%xmm4
  ef:	66 41 0f eb e0       	por    %xmm8,%xmm4
  f4:	66 0f df e3          	pandn  %xmm3,%xmm4
  f8:	80 fa 01             	cmp    $0x1,%dl
  fb:	74 0b                	je     108 <mime_fixture_classify16+0x108>
  fd:	66 0f 6f ec          	movdqa %xmm4,%xmm5
 101:	80 fa 02             	cmp    $0x2,%dl
 104:	74 3a                	je     140 <mime_fixture_classify16+0x140>
 106:	eb 54                	jmp    15c <mime_fixture_classify16+0x15c>
 108:	66 44 0f 6f 05 00 00 	movdqa 0x0(%rip),%xmm8        # 111 <mime_fixture_classify16+0x111>
 10f:	00 00
 111:	66 44 0f 74 c0       	pcmpeqb %xmm0,%xmm8
 116:	66 44 0f 6f 0d 00 00 	movdqa 0x0(%rip),%xmm9        # 11f <mime_fixture_classify16+0x11f>
 11d:	00 00
 11f:	66 44 0f fc c8       	paddb  %xmm0,%xmm9
 124:	66 0f 6f 2d 00 00 00 	movdqa 0x0(%rip),%xmm5        # 12c <mime_fixture_classify16+0x12c>
 12b:	00
 12c:	66 41 0f da e9       	pminub %xmm9,%xmm5
 131:	66 41 0f 74 e9       	pcmpeqb %xmm9,%xmm5
 136:	66 41 0f eb e8       	por    %xmm8,%xmm5
 13b:	80 fa 02             	cmp    $0x2,%dl
 13e:	75 1c                	jne    15c <mime_fixture_classify16+0x15c>
 140:	66 0f 6f 2d 00 00 00 	movdqa 0x0(%rip),%xmm5        # 148 <mime_fixture_classify16+0x148>
 147:	00
 148:	66 0f 74 e8          	pcmpeqb %xmm0,%xmm5
 14c:	66 0f eb f5          	por    %xmm5,%xmm6
 150:	66 0f eb fe          	por    %xmm6,%xmm7
 154:	66 0f df fb          	pandn  %xmm3,%xmm7
 158:	66 0f 6f ef          	movdqa %xmm7,%xmm5
 15c:	80 fa 03             	cmp    $0x3,%dl
 15f:	75 20                	jne    181 <mime_fixture_classify16+0x181>
 161:	66 0f 6f 1d 00 00 00 	movdqa 0x0(%rip),%xmm3        # 169 <mime_fixture_classify16+0x169>
 168:	00
 169:	66 0f 74 15 00 00 00 	pcmpeqb 0x0(%rip),%xmm2        # 171 <mime_fixture_classify16+0x171>
 170:	00
 171:	66 0f 74 d8          	pcmpeqb %xmm0,%xmm3
 175:	66 0f eb d3          	por    %xmm3,%xmm2
 179:	66 0f df d4          	pandn  %xmm4,%xmm2
 17d:	66 0f 6f ea          	movdqa %xmm2,%xmm5
 181:	66 0f 76 d2          	pcmpeqd %xmm2,%xmm2
 185:	80 fa 04             	cmp    $0x4,%dl
 188:	75 38                	jne    1c2 <mime_fixture_classify16+0x1c2>
 18a:	66 0f 6f 1d 00 00 00 	movdqa 0x0(%rip),%xmm3        # 192 <mime_fixture_classify16+0x192>
 191:	00
 192:	66 0f 74 d8          	pcmpeqb %xmm0,%xmm3
 196:	66 0f 6f 25 00 00 00 	movdqa 0x0(%rip),%xmm4        # 19e <mime_fixture_classify16+0x19e>
 19d:	00
 19e:	66 0f 74 e0          	pcmpeqb %xmm0,%xmm4
 1a2:	66 0f eb e3          	por    %xmm3,%xmm4
 1a6:	66 0f 6f 1d 00 00 00 	movdqa 0x0(%rip),%xmm3        # 1ae <mime_fixture_classify16+0x1ae>
 1ad:	00
 1ae:	66 0f 74 d8          	pcmpeqb %xmm0,%xmm3
 1b2:	66 0f eb cb          	por    %xmm3,%xmm1
 1b6:	66 0f eb cc          	por    %xmm4,%xmm1
 1ba:	66 0f ef ca          	pxor   %xmm2,%xmm1
 1be:	66 0f 6f e9          	movdqa %xmm1,%xmm5
 1c2:	80 fa 05             	cmp    $0x5,%dl
 1c5:	75 08                	jne    1cf <mime_fixture_classify16+0x1cf>
 1c7:	66 0f 64 c2          	pcmpgtb %xmm2,%xmm0
 1cb:	66 0f 6f e8          	movdqa %xmm0,%xmm5
 1cf:	66 0f 71 f5 07       	psllw  $0x7,%xmm5
 1d4:	66 0f d7 c5          	pmovmskb %xmm5,%eax
 1d8:	35 ff ff 00 00       	xor    $0xffff,%eax
 1dd:	c3                   	ret
 1de:	66 90                	xchg   %ax,%ax

00000000000001e0 <mime_fixture_lower8>:
 1e0:	66 48 0f 6e c7       	movq   %rdi,%xmm0
 1e5:	66 0f 6f 0d 00 00 00 	movdqa 0x0(%rip),%xmm1        # 1ed <mime_fixture_lower8+0xd>
 1ec:	00
 1ed:	66 0f fc c8          	paddb  %xmm0,%xmm1
 1f1:	66 0f 6f 15 00 00 00 	movdqa 0x0(%rip),%xmm2        # 1f9 <mime_fixture_lower8+0x19>
 1f8:	00
 1f9:	66 0f de d1          	pmaxub %xmm1,%xmm2
 1fd:	66 0f 74 d1          	pcmpeqb %xmm1,%xmm2
 201:	66 0f df 15 00 00 00 	pandn  0x0(%rip),%xmm2        # 209 <mime_fixture_lower8+0x29>
 208:	00
 209:	66 0f fc d0          	paddb  %xmm0,%xmm2
 20d:	66 48 0f 7e d0       	movq   %xmm2,%rax
 212:	c3                   	ret
