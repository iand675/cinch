define { i64, i64, i64 } @"Main.allocation-loop#cc"(i64 %p0, i32 noundef %p1, i32 noundef %p2, i64 noundef %p3, i64 noundef %p4, i64 %p5) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t3 = alloca i64
  %t5 = alloca i64
  %t6 = alloca i64
  %t7 = alloca i64
  %t15 = alloca i64
  %t18 = alloca i64
  %t21 = alloca i64
  %t24 = alloca i64
  %t27 = alloca i64
  %t31 = alloca i64
  %t34 = alloca i64
  %t37 = alloca i64
  %t38 = alloca i64
  %t42 = alloca i64
  %t46 = alloca i64
  %t50 = alloca i64
  %t54 = alloca i64
  %t55 = alloca i64
  %t56 = alloca i64
  %t59 = alloca i64
  %t62 = alloca i64
  %t77 = alloca i64
  %t78 = alloca i64
  %t81 = alloca i64
  %t85 = alloca i64
  %t90 = alloca i64
  %t93 = alloca i64
  %t100 = alloca i64
  %t103 = alloca i64
  %t111 = alloca i64
  %t114 = alloca i64
  %t118 = alloca i64
  %t122 = alloca i64
  %t123 = alloca i64
  %t126 = alloca i64
  %t129 = alloca i64
  %t137 = alloca i64
  %t141 = alloca i64
  %t144 = alloca i64
  %t148 = alloca i64
  %t149 = alloca i64
  %t152 = alloca i64
  %t156 = alloca i64
  %t164 = alloca i64
  %t169 = alloca i64
  %t173 = alloca i64
  %t177 = alloca i64
  %t178 = alloca i64
  %t185 = alloca i64
  %t190 = alloca i64
  %t191 = alloca i64
  %t194 = alloca i64
  %t198 = alloca i64
  %t206 = alloca i64
  %t211 = alloca i64
  %t215 = alloca i64
  %t219 = alloca i64
  %t220 = alloca i64
  %t221 = alloca i64
  %t225 = alloca i64
  %t232 = alloca i64
  %t239 = alloca i64
  %t248 = alloca i64
  %t251 = alloca i64
  %t254 = alloca i64
  %t257 = alloca i64
  %t260 = alloca i64
  %t264 = alloca i64
  %t267 = alloca i64
  %t270 = alloca i64
  %t271 = alloca i64
  %t275 = alloca i64
  %t279 = alloca i64
  %t283 = alloca i64
  %t287 = alloca i64
  %t288 = alloca i64
  %t289 = alloca i64
  %t292 = alloca i64
  %t295 = alloca i64
  %t310 = alloca i64
  %t311 = alloca i64
  %t314 = alloca i64
  %t318 = alloca i64
  %t323 = alloca i64
  %t324 = alloca i64
  %t328 = alloca i64
  %t335 = alloca i64
  %t340 = alloca i64
  %t341 = alloca i64
  %t345 = alloca i64
  %t352 = alloca i64
  %t358 = alloca i64
  store i64 %p0, ptr %t0
  %t2 = zext i32 %p1 to i64
  store i64 %t2, ptr %t1
  %t4 = zext i32 %p2 to i64
  store i64 %t4, ptr %t3
  store i64 %p3, ptr %t5
  store i64 %p4, ptr %t6
  store i64 %p5, ptr %t7
  br label %bb0
bb0:
  %t8 = load i64, ptr %t3
  %t9 = load i64, ptr %t1
  %t10 = call i64 @"Main.before"(i64 %t8, i64 %t9)
  %t11 = icmp ne i64 %t10, 0
  br i1 %t11, label %bb1, label %bb2
bb1:
  %t12 = load i64, ptr %t0
  call void @cinch.rc.dup.tobj(i64 %t12)
  %t13 = load i64, ptr %t0
  %t14 = call i64 @"cinch.runtime.bytesNew"(i64 %t13)
  store i64 %t14, ptr %t15
  %t16 = load i64, ptr %t3
  %t17 = call i64 @"Main.marker"(i64 %t16)
  store i64 %t17, ptr %t18
  %t19 = load i64, ptr %t18
  %t20 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t19), !range !97
  store i64 %t20, ptr %t21
  %t22 = load i64, ptr %t21
  %t23 = call i64 @"cinch.prim.u32ToI64"(i64 %t22)
  store i64 %t23, ptr %t24
  %t25 = load i64, ptr %t24
  %t26 = call i64 @"cinch.runtime.natFromI64"(i64 %t25)
  store i64 %t26, ptr %t27
  %t28 = load i64, ptr %t18
  %t29 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t28, i64 32)
  %t30 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t29), !range !98
  store i64 %t30, ptr %t31
  %t32 = load i64, ptr %t31
  %t33 = call i64 @"cinch.prim.u32ToI64"(i64 %t32)
  store i64 %t33, ptr %t34
  %t35 = load i64, ptr %t34
  %t36 = call i64 @"cinch.runtime.natFromI64"(i64 %t35)
  store i64 %t36, ptr %t37
  store i64 8589934593, ptr %t38
  %t39 = load i64, ptr %t37
  %t40 = load i64, ptr %t38
  %t41 = call i64 @"cinch.runtime.natMul"(i64 %t39, i64 %t40)
  store i64 %t41, ptr %t42
  %t43 = load i64, ptr %t37
  call void @cinch.rc.drop.tobj(i64 %t43)
  %t44 = load i64, ptr %t38
  call void @cinch.rc.drop.tobj(i64 %t44)
  %t45 = load i64, ptr %t42
  store i64 %t45, ptr %t46
  %t47 = load i64, ptr %t27
  %t48 = load i64, ptr %t46
  %t49 = call i64 @"cinch.runtime.natAdd"(i64 %t47, i64 %t48)
  store i64 %t49, ptr %t50
  %t51 = load i64, ptr %t27
  call void @cinch.rc.drop.tobj(i64 %t51)
  %t52 = load i64, ptr %t46
  call void @cinch.rc.drop.tobj(i64 %t52)
  %t53 = load i64, ptr %t50
  store i64 %t53, ptr %t54
  %t57 = load i64, ptr %t15
  %t58 = call i64 @"cinch.runtime.bytesLen"(i64 %t57)
  store i64 %t58, ptr %t59
  %t60 = load i64, ptr %t59
  %t61 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t60)
  store i64 %t61, ptr %t62
  %t63 = load i64, ptr %t59
  call void @cinch.rc.drop.tobj(i64 %t63)
  %t64 = load i64, ptr %t62
  %t65 = call i64 @"cinch.prim.ltI64"(i64 %t64, i64 0), !range !99
  %t66 = icmp ne i64 %t65, 0
  br i1 %t66, label %bb5, label %bb6
bb5:
  %t67 = load i64, ptr %t54
  %t68 = call i64 @"cinch.runtime.natCmp"(i64 %t67, i64 511)
  %t69 = call i64 @"cinch.prim.leI64"(i64 %t68, i64 0), !range !100
  store i64 %t69, ptr %t56
  br label %bb4
bb6:
  store i64 0, ptr %t56
  br label %bb4
bb4:
  %t70 = load i64, ptr %t56
  %t71 = icmp ne i64 %t70, 0
  br i1 %t71, label %bb7, label %bb8
bb7:
  %t72 = load i64, ptr %t15
  %t73 = load i64, ptr %t54
  %t74 = call i64 @"cinch.runtime.bytesSet"(i64 %t72, i64 1, i64 %t73)
  store i64 %t74, ptr %t55
  br label %bb3
bb8:
  %t75 = load i64, ptr %t54
  call void @cinch.rc.drop.tobj(i64 %t75)
  store i64 0, ptr %t55
  br label %bb3
bb3:
  %t76 = load i64, ptr %t55
  store i64 %t76, ptr %t77
  %t79 = load i64, ptr %t15
  %t80 = call i64 @"cinch.runtime.bytesLen"(i64 %t79)
  store i64 %t80, ptr %t81
  %t82 = load i64, ptr %t81
  %t83 = load i64, ptr %t0
  %t84 = call i64 @"cinch.runtime.natCmp"(i64 %t82, i64 %t83)
  store i64 %t84, ptr %t85
  %t86 = load i64, ptr %t81
  call void @cinch.rc.drop.tobj(i64 %t86)
  %t87 = load i64, ptr %t85
  %t88 = call i64 @"cinch.prim.eqI64"(i64 %t87, i64 0), !range !101
  %t89 = icmp ne i64 %t88, 0
  br i1 %t89, label %bb10, label %bb11
bb10:
  store i64 -1, ptr %t90
  %t91 = load i64, ptr %t90
  %t92 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t91)
  store i64 %t92, ptr %t93
  %t94 = load i64, ptr %t90
  call void @cinch.rc.drop.tobj(i64 %t94)
  %t95 = load i64, ptr %t93
  %t96 = call i64 @"cinch.prim.leI64"(i64 %t95, i64 0), !range !102
  %t97 = icmp ne i64 %t96, 0
  br i1 %t97, label %bb12, label %bb13
bb12:
  %t98 = call i64 @"cinch.runtime.natToI64"(i64 3)
  %t99 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t98)
  store i64 %t99, ptr %t78
  br label %bb9
bb13:
  store i64 0, ptr %t78
  br label %bb9
bb11:
  store i64 -1, ptr %t100
  %t101 = load i64, ptr %t100
  %t102 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t101)
  store i64 %t102, ptr %t103
  %t104 = load i64, ptr %t100
  call void @cinch.rc.drop.tobj(i64 %t104)
  %t105 = load i64, ptr %t103
  %t106 = call i64 @"cinch.prim.leI64"(i64 %t105, i64 0), !range !103
  %t107 = icmp ne i64 %t106, 0
  br i1 %t107, label %bb14, label %bb15
bb14:
  %t108 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t109 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t108)
  store i64 %t109, ptr %t78
  br label %bb9
bb15:
  store i64 0, ptr %t78
  br label %bb9
bb9:
  %t110 = load i64, ptr %t78
  store i64 %t110, ptr %t111
  %t112 = load i64, ptr %t3
  %t113 = call i64 @"Main.bump"(i64 %t112)
  store i64 %t113, ptr %t114
  %t115 = load i64, ptr %t5
  %t116 = load i64, ptr %t111
  %t117 = call i64 @"Main.add"(i64 %t115, i64 %t116)
  store i64 %t117, ptr %t118
  %t119 = load i64, ptr %t6
  %t120 = load i64, ptr %t15
  %t121 = call i64 @"cinch.runtime.bytesLen"(i64 %t120)
  store i64 %t121, ptr %t122
  %t124 = load i64, ptr %t15
  %t125 = call i64 @"cinch.runtime.bytesLen"(i64 %t124)
  store i64 %t125, ptr %t126
  %t127 = load i64, ptr %t126
  %t128 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t127)
  store i64 %t128, ptr %t129
  %t130 = load i64, ptr %t126
  call void @cinch.rc.drop.tobj(i64 %t130)
  %t131 = load i64, ptr %t129
  %t132 = call i64 @"cinch.prim.ltI64"(i64 %t131, i64 0), !range !104
  %t133 = icmp ne i64 %t132, 0
  br i1 %t133, label %bb17, label %bb18
bb17:
  %t134 = load i64, ptr %t15
  %t135 = call i64 @"cinch.runtime.bytesGet"(i64 %t134, i64 1)
  store i64 %t135, ptr %t123
  br label %bb16
bb18:
  store i64 1, ptr %t123
  br label %bb16
bb16:
  %t136 = load i64, ptr %t123
  store i64 %t136, ptr %t137
  %t138 = load i64, ptr %t122
  %t139 = load i64, ptr %t137
  %t140 = call i64 @"cinch.runtime.natAdd"(i64 %t138, i64 %t139)
  store i64 %t140, ptr %t141
  %t142 = load i64, ptr %t137
  call void @cinch.rc.drop.tobj(i64 %t142)
  %t143 = load i64, ptr %t141
  store i64 %t143, ptr %t144
  %t145 = load i64, ptr %t122
  call void @cinch.rc.dup.tobj(i64 %t145)
  %t146 = load i64, ptr %t122
  %t147 = call i64 @"cinch.runtime.natDiv"(i64 %t146, i64 5)
  store i64 %t147, ptr %t148
  %t150 = load i64, ptr %t15
  %t151 = call i64 @"cinch.runtime.bytesLen"(i64 %t150)
  store i64 %t151, ptr %t152
  %t153 = load i64, ptr %t148
  %t154 = load i64, ptr %t152
  %t155 = call i64 @"cinch.runtime.natCmp"(i64 %t153, i64 %t154)
  store i64 %t155, ptr %t156
  %t157 = load i64, ptr %t152
  call void @cinch.rc.drop.tobj(i64 %t157)
  %t158 = load i64, ptr %t156
  %t159 = call i64 @"cinch.prim.ltI64"(i64 %t158, i64 0), !range !105
  %t160 = icmp ne i64 %t159, 0
  br i1 %t160, label %bb20, label %bb21
bb20:
  %t161 = load i64, ptr %t15
  %t162 = load i64, ptr %t148
  %t163 = call i64 @"cinch.runtime.bytesGet"(i64 %t161, i64 %t162)
  store i64 %t163, ptr %t164
  %t165 = load i64, ptr %t148
  call void @cinch.rc.drop.tobj(i64 %t165)
  %t166 = load i64, ptr %t164
  store i64 %t166, ptr %t149
  br label %bb19
bb21:
  %t167 = load i64, ptr %t148
  call void @cinch.rc.drop.tobj(i64 %t167)
  store i64 1, ptr %t149
  br label %bb19
bb19:
  %t168 = load i64, ptr %t149
  store i64 %t168, ptr %t169
  %t170 = load i64, ptr %t144
  %t171 = load i64, ptr %t169
  %t172 = call i64 @"cinch.runtime.natAdd"(i64 %t170, i64 %t171)
  store i64 %t172, ptr %t173
  %t174 = load i64, ptr %t144
  call void @cinch.rc.drop.tobj(i64 %t174)
  %t175 = load i64, ptr %t169
  call void @cinch.rc.drop.tobj(i64 %t175)
  %t176 = load i64, ptr %t173
  store i64 %t176, ptr %t177
  %t179 = load i64, ptr %t122
  %t180 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t179)
  %t181 = call i64 @"cinch.prim.leI64"(i64 %t180, i64 0), !range !106
  %t182 = icmp ne i64 %t181, 0
  br i1 %t182, label %bb23, label %bb24
bb23:
  %t183 = load i64, ptr %t122
  %t184 = call i64 @"cinch.runtime.natSub"(i64 %t183, i64 3)
  store i64 %t184, ptr %t185
  %t186 = load i64, ptr %t122
  call void @cinch.rc.drop.tobj(i64 %t186)
  %t187 = load i64, ptr %t185
  store i64 %t187, ptr %t178
  br label %bb22
bb24:
  %t188 = load i64, ptr %t122
  call void @cinch.rc.drop.tobj(i64 %t188)
  store i64 1, ptr %t178
  br label %bb22
bb22:
  %t189 = load i64, ptr %t178
  store i64 %t189, ptr %t190
  %t192 = load i64, ptr %t15
  %t193 = call i64 @"cinch.runtime.bytesLen"(i64 %t192)
  store i64 %t193, ptr %t194
  %t195 = load i64, ptr %t190
  %t196 = load i64, ptr %t194
  %t197 = call i64 @"cinch.runtime.natCmp"(i64 %t195, i64 %t196)
  store i64 %t197, ptr %t198
  %t199 = load i64, ptr %t194
  call void @cinch.rc.drop.tobj(i64 %t199)
  %t200 = load i64, ptr %t198
  %t201 = call i64 @"cinch.prim.ltI64"(i64 %t200, i64 0), !range !107
  %t202 = icmp ne i64 %t201, 0
  br i1 %t202, label %bb26, label %bb27
bb26:
  %t203 = load i64, ptr %t15
  %t204 = load i64, ptr %t190
  %t205 = call i64 @"cinch.runtime.bytesGet"(i64 %t203, i64 %t204)
  store i64 %t205, ptr %t206
  %t207 = load i64, ptr %t190
  call void @cinch.rc.drop.tobj(i64 %t207)
  %t208 = load i64, ptr %t206
  store i64 %t208, ptr %t191
  br label %bb25
bb27:
  %t209 = load i64, ptr %t190
  call void @cinch.rc.drop.tobj(i64 %t209)
  store i64 1, ptr %t191
  br label %bb25
bb25:
  %t210 = load i64, ptr %t191
  store i64 %t210, ptr %t211
  %t212 = load i64, ptr %t177
  %t213 = load i64, ptr %t211
  %t214 = call i64 @"cinch.runtime.natAdd"(i64 %t212, i64 %t213)
  store i64 %t214, ptr %t215
  %t216 = load i64, ptr %t177
  call void @cinch.rc.drop.tobj(i64 %t216)
  %t217 = load i64, ptr %t211
  call void @cinch.rc.drop.tobj(i64 %t217)
  %t218 = load i64, ptr %t215
  store i64 %t218, ptr %t219
  store i64 -1, ptr %t221
  %t222 = load i64, ptr %t219
  %t223 = load i64, ptr %t221
  %t224 = call i64 @"cinch.runtime.natCmp"(i64 %t222, i64 %t223)
  store i64 %t224, ptr %t225
  %t226 = load i64, ptr %t221
  call void @cinch.rc.drop.tobj(i64 %t226)
  %t227 = load i64, ptr %t225
  %t228 = call i64 @"cinch.prim.leI64"(i64 %t227, i64 0), !range !108
  %t229 = icmp ne i64 %t228, 0
  br i1 %t229, label %bb29, label %bb30
bb29:
  %t230 = load i64, ptr %t219
  %t231 = call i64 @"cinch.runtime.natToI64"(i64 %t230)
  store i64 %t231, ptr %t232
  %t233 = load i64, ptr %t219
  call void @cinch.rc.drop.tobj(i64 %t233)
  %t234 = load i64, ptr %t232
  %t235 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t234)
  store i64 %t235, ptr %t220
  br label %bb28
bb30:
  %t236 = load i64, ptr %t219
  call void @cinch.rc.drop.tobj(i64 %t236)
  store i64 0, ptr %t220
  br label %bb28
bb28:
  %t237 = load i64, ptr %t220
  %t238 = call i64 @"Main.mix"(i64 %t119, i64 %t237)
  store i64 %t238, ptr %t239
  %t240 = load i64, ptr %t7
  call void @cinch.rc.drop.obj(i64 %t240)
  %t241 = load i64, ptr %t114
  %t242 = load i64, ptr %t1
  %t243 = call i64 @"Main.before"(i64 %t241, i64 %t242)
  %t244 = icmp ne i64 %t243, 0
  br i1 %t244, label %bb31, label %bb32
bb31:
  %t245 = load i64, ptr %t0
  call void @cinch.rc.dup.tobj(i64 %t245)
  %t246 = load i64, ptr %t0
  %t247 = call i64 @"cinch.runtime.bytesNew"(i64 %t246)
  store i64 %t247, ptr %t248
  %t249 = load i64, ptr %t114
  %t250 = call i64 @"Main.marker"(i64 %t249)
  store i64 %t250, ptr %t251
  %t252 = load i64, ptr %t251
  %t253 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t252), !range !109
  store i64 %t253, ptr %t254
  %t255 = load i64, ptr %t254
  %t256 = call i64 @"cinch.prim.u32ToI64"(i64 %t255)
  store i64 %t256, ptr %t257
  %t258 = load i64, ptr %t257
  %t259 = call i64 @"cinch.runtime.natFromI64"(i64 %t258)
  store i64 %t259, ptr %t260
  %t261 = load i64, ptr %t251
  %t262 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t261, i64 32)
  %t263 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t262), !range !110
  store i64 %t263, ptr %t264
  %t265 = load i64, ptr %t264
  %t266 = call i64 @"cinch.prim.u32ToI64"(i64 %t265)
  store i64 %t266, ptr %t267
  %t268 = load i64, ptr %t267
  %t269 = call i64 @"cinch.runtime.natFromI64"(i64 %t268)
  store i64 %t269, ptr %t270
  store i64 8589934593, ptr %t271
  %t272 = load i64, ptr %t270
  %t273 = load i64, ptr %t271
  %t274 = call i64 @"cinch.runtime.natMul"(i64 %t272, i64 %t273)
  store i64 %t274, ptr %t275
  %t276 = load i64, ptr %t270
  call void @cinch.rc.drop.tobj(i64 %t276)
  %t277 = load i64, ptr %t271
  call void @cinch.rc.drop.tobj(i64 %t277)
  %t278 = load i64, ptr %t275
  store i64 %t278, ptr %t279
  %t280 = load i64, ptr %t260
  %t281 = load i64, ptr %t279
  %t282 = call i64 @"cinch.runtime.natAdd"(i64 %t280, i64 %t281)
  store i64 %t282, ptr %t283
  %t284 = load i64, ptr %t260
  call void @cinch.rc.drop.tobj(i64 %t284)
  %t285 = load i64, ptr %t279
  call void @cinch.rc.drop.tobj(i64 %t285)
  %t286 = load i64, ptr %t283
  store i64 %t286, ptr %t287
  %t290 = load i64, ptr %t248
  %t291 = call i64 @"cinch.runtime.bytesLen"(i64 %t290)
  store i64 %t291, ptr %t292
  %t293 = load i64, ptr %t292
  %t294 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t293)
  store i64 %t294, ptr %t295
  %t296 = load i64, ptr %t292
  call void @cinch.rc.drop.tobj(i64 %t296)
  %t297 = load i64, ptr %t295
  %t298 = call i64 @"cinch.prim.ltI64"(i64 %t297, i64 0), !range !111
  %t299 = icmp ne i64 %t298, 0
  br i1 %t299, label %bb35, label %bb36
bb35:
  %t300 = load i64, ptr %t287
  %t301 = call i64 @"cinch.runtime.natCmp"(i64 %t300, i64 511)
  %t302 = call i64 @"cinch.prim.leI64"(i64 %t301, i64 0), !range !112
  store i64 %t302, ptr %t289
  br label %bb34
bb36:
  store i64 0, ptr %t289
  br label %bb34
bb34:
  %t303 = load i64, ptr %t289
  %t304 = icmp ne i64 %t303, 0
  br i1 %t304, label %bb37, label %bb38
bb37:
  %t305 = load i64, ptr %t248
  %t306 = load i64, ptr %t287
  %t307 = call i64 @"cinch.runtime.bytesSet"(i64 %t305, i64 1, i64 %t306)
  store i64 %t307, ptr %t288
  br label %bb33
bb38:
  %t308 = load i64, ptr %t287
  call void @cinch.rc.drop.tobj(i64 %t308)
  store i64 0, ptr %t288
  br label %bb33
bb33:
  %t309 = load i64, ptr %t288
  store i64 %t309, ptr %t310
  %t312 = load i64, ptr %t248
  %t313 = call i64 @"cinch.runtime.bytesLen"(i64 %t312)
  store i64 %t313, ptr %t314
  %t315 = load i64, ptr %t314
  %t316 = load i64, ptr %t0
  %t317 = call i64 @"cinch.runtime.natCmp"(i64 %t315, i64 %t316)
  store i64 %t317, ptr %t318
  %t319 = load i64, ptr %t314
  call void @cinch.rc.drop.tobj(i64 %t319)
  %t320 = load i64, ptr %t318
  %t321 = call i64 @"cinch.prim.eqI64"(i64 %t320, i64 0), !range !113
  %t322 = icmp ne i64 %t321, 0
  br i1 %t322, label %bb40, label %bb41
bb40:
  store i64 3, ptr %t323
  store i64 -1, ptr %t324
  %t325 = load i64, ptr %t323
  %t326 = load i64, ptr %t324
  %t327 = call i64 @"cinch.runtime.natCmp"(i64 %t325, i64 %t326)
  store i64 %t327, ptr %t328
  %t329 = load i64, ptr %t324
  call void @cinch.rc.drop.tobj(i64 %t329)
  %t330 = load i64, ptr %t328
  %t331 = call i64 @"cinch.prim.leI64"(i64 %t330, i64 0), !range !114
  %t332 = icmp ne i64 %t331, 0
  br i1 %t332, label %bb42, label %bb43
bb42:
  %t333 = load i64, ptr %t323
  %t334 = call i64 @"cinch.runtime.natToI64"(i64 %t333)
  store i64 %t334, ptr %t335
  %t336 = load i64, ptr %t323
  call void @cinch.rc.drop.tobj(i64 %t336)
  %t337 = load i64, ptr %t335
  %t338 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t337)
  store i64 %t338, ptr %t311
  br label %bb39
bb43:
  %t339 = load i64, ptr %t323
  call void @cinch.rc.drop.tobj(i64 %t339)
  store i64 0, ptr %t311
  br label %bb39
bb41:
  store i64 1, ptr %t340
  store i64 -1, ptr %t341
  %t342 = load i64, ptr %t340
  %t343 = load i64, ptr %t341
  %t344 = call i64 @"cinch.runtime.natCmp"(i64 %t342, i64 %t343)
  store i64 %t344, ptr %t345
  %t346 = load i64, ptr %t341
  call void @cinch.rc.drop.tobj(i64 %t346)
  %t347 = load i64, ptr %t345
  %t348 = call i64 @"cinch.prim.leI64"(i64 %t347, i64 0), !range !115
  %t349 = icmp ne i64 %t348, 0
  br i1 %t349, label %bb44, label %bb45
bb44:
  %t350 = load i64, ptr %t340
  %t351 = call i64 @"cinch.runtime.natToI64"(i64 %t350)
  store i64 %t351, ptr %t352
  %t353 = load i64, ptr %t340
  call void @cinch.rc.drop.tobj(i64 %t353)
  %t354 = load i64, ptr %t352
  %t355 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t354)
  store i64 %t355, ptr %t311
  br label %bb39
bb45:
  %t356 = load i64, ptr %t340
  call void @cinch.rc.drop.tobj(i64 %t356)
  store i64 0, ptr %t311
  br label %bb39
bb39:
  %t357 = load i64, ptr %t311
  store i64 %t357, ptr %t358
  %t359 = load i64, ptr %t15
  call void @cinch.rc.drop.obj(i64 %t359)
  %t360 = load i64, ptr %t0
  %t361 = load i64, ptr %t1
  %t362 = load i64, ptr %t114
  %t363 = call i64 @"Main.bump"(i64 %t362)
  %t364 = load i64, ptr %t118
  %t365 = load i64, ptr %t358
  %t366 = call i64 @"Main.add"(i64 %t364, i64 %t365)
  %t367 = load i64, ptr %t239
  %t368 = load i64, ptr %t248
  %t369 = call i64 @"Main.fingerprint"(i64 %t368)
  %t370 = call i64 @"Main.mix"(i64 %t367, i64 %t369)
  %t371 = load i64, ptr %t248
  store i64 %t360, ptr %t0
  store i64 %t361, ptr %t1
  store i64 %t363, ptr %t3
  store i64 %t366, ptr %t5
  store i64 %t370, ptr %t6
  store i64 %t371, ptr %t7
  br label %bb0
bb32:
  %t372 = load i64, ptr %t0
  call void @cinch.rc.drop.tobj(i64 %t372)
  %t373 = load i64, ptr %t118
  %t374 = load i64, ptr %t239
  %t375 = load i64, ptr %t15
  %t376 = insertvalue { i64, i64, i64 } poison, i64 %t373, 0
  %t377 = insertvalue { i64, i64, i64 } %t376, i64 %t374, 1
  %t378 = insertvalue { i64, i64, i64 } %t377, i64 %t375, 2
  ret { i64, i64, i64 } %t378
bb2:
  %t379 = load i64, ptr %t0
  call void @cinch.rc.drop.tobj(i64 %t379)
  %t380 = load i64, ptr %t5
  %t381 = load i64, ptr %t6
  %t382 = load i64, ptr %t7
  %t383 = insertvalue { i64, i64, i64 } poison, i64 %t380, 0
  %t384 = insertvalue { i64, i64, i64 } %t383, i64 %t381, 1
  %t385 = insertvalue { i64, i64, i64 } %t384, i64 %t382, 2
  ret { i64, i64, i64 } %t385
}
