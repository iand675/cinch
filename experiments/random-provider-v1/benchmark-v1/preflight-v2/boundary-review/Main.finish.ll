define i64 @"Main.finish"(i64 %p0, i64 %p1, i64 %p2, i64 %p3) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t2 = alloca i64
  %t3 = alloca i64
  %t5 = alloca i64
  %t8 = alloca i64
  %t23 = alloca i64
  %t26 = alloca i64
  %t29 = alloca i64
  %t35 = alloca i64
  %t36 = alloca i64
  %t39 = alloca i64
  %t42 = alloca i64
  %t50 = alloca i64
  %t54 = alloca i64
  %t57 = alloca i64
  %t61 = alloca i64
  %t62 = alloca i64
  %t65 = alloca i64
  %t69 = alloca i64
  %t77 = alloca i64
  %t82 = alloca i64
  %t86 = alloca i64
  %t90 = alloca i64
  %t91 = alloca i64
  %t98 = alloca i64
  %t103 = alloca i64
  %t104 = alloca i64
  %t107 = alloca i64
  %t111 = alloca i64
  %t119 = alloca i64
  %t126 = alloca i64
  %t130 = alloca i64
  %t134 = alloca i64
  %t135 = alloca i64
  %t136 = alloca i64
  %t140 = alloca i64
  %t147 = alloca i64
  %t153 = alloca i64
  %t154 = alloca i64
  %t163 = alloca i64
  %t170 = alloca i64
  %t174 = alloca i64
  %t178 = alloca i64
  %t183 = alloca i64
  %t186 = alloca i64
  %t188 = alloca i64
  %t191 = alloca i64
  %t193 = alloca i64
  %t196 = alloca i64
  %t198 = alloca i64
  %t201 = alloca i64
  %t204 = alloca i64
  %t207 = alloca i64
  %t211 = alloca i64
  %t214 = alloca i64
  %t217 = alloca i64
  %t218 = alloca i64
  %t222 = alloca i64
  %t226 = alloca i64
  %t230 = alloca i64
  %t234 = alloca i64
  %t237 = alloca i64
  %t239 = alloca i64
  %t245 = alloca i64
  %t249 = alloca i64
  %t255 = alloca i64
  %t259 = alloca i64
  %t265 = alloca i64
  %t269 = alloca i64
  %t275 = alloca i64
  %t279 = alloca i64
  %t285 = alloca i64
  %t289 = alloca i64
  %t295 = alloca i64
  %t299 = alloca i64
  %t305 = alloca i64
  %t309 = alloca i64
  %t312 = alloca i64
  %t315 = alloca i64
  %t319 = alloca i64
  %t322 = alloca i64
  %t325 = alloca i64
  %t328 = alloca i64
  %t331 = alloca i64
  %t335 = alloca i64
  %t338 = alloca i64
  %t341 = alloca i64
  %t342 = alloca i64
  %t346 = alloca i64
  %t350 = alloca i64
  %t354 = alloca i64
  %t358 = alloca i64
  %t362 = alloca i64
  store i64 %p0, ptr %t0
  store i64 %p1, ptr %t1
  store i64 %p2, ptr %t2
  store i64 %p3, ptr %t3
  br label %bb0
bb0:
  %t4 = call i64 @"Effect.Clock.raw-monotonic-ns"(i64 0)
  store i64 %t4, ptr %t5
  %t6 = load i64, ptr %t5
  %t7 = call i64 @"cinch.runtime.natFromI64"(i64 %t6)
  store i64 %t7, ptr %t8
  %t9 = load i64, ptr %t3
  %t10 = inttoptr i64 %t9 to ptr
  %t11 = getelementptr inbounds i8, ptr %t10, i64 -8
  %t12 = load atomic i64, ptr %t11 monotonic, align 8
  %t13 = lshr i64 %t12, 40
  %t14 = and i64 %t13, 255
  %t15 = icmp eq i64 %t14, 255
  br i1 %t15, label %bb2, label %bb3
bb2:
  %t16 = getelementptr inbounds i8, ptr %t10, i64 -16
  %t17 = load i64, ptr %t16
  %t18 = inttoptr i64 %t17 to ptr
  %t19 = load i64, ptr %t18
  br label %bb4
bb3:
  br label %bb4
bb4:
  %t20 = phi i64 [ %t19, %bb2 ], [ %t14, %bb3 ]
  switch i64 %t20, label %bb1 [
  i64 0, label %bb5
]
bb5:
  %t21 = getelementptr inbounds i8, ptr %t10, i64 0
  %t22 = load i64, ptr %t21
  store i64 %t22, ptr %t23
  %t24 = getelementptr inbounds i8, ptr %t10, i64 8
  %t25 = load i64, ptr %t24
  store i64 %t25, ptr %t26
  %t27 = getelementptr inbounds i8, ptr %t10, i64 16
  %t28 = load i64, ptr %t27
  store i64 %t28, ptr %t29
  %t30 = load i64, ptr %t3
  %t31 = call i1 @cinch.rc.unique(i64 %t30)
  br i1 %t31, label %bb6, label %bb7
bb6:
  call void @cinch_free(i64 %t30)
  br label %bb8
bb7:
  %t32 = load i64, ptr %t29
  call void @cinch.rc.dup.obj(i64 %t32)
  call void @cinch.rc.drop.obj(i64 %t30)
  br label %bb8
bb8:
  %t33 = load i64, ptr %t29
  %t34 = call i64 @"cinch.runtime.bytesLen"(i64 %t33)
  store i64 %t34, ptr %t35
  %t37 = load i64, ptr %t29
  %t38 = call i64 @"cinch.runtime.bytesLen"(i64 %t37)
  store i64 %t38, ptr %t39
  %t40 = load i64, ptr %t39
  %t41 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t40)
  store i64 %t41, ptr %t42
  %t43 = load i64, ptr %t39
  call void @cinch.rc.drop.tobj(i64 %t43)
  %t44 = load i64, ptr %t42
  %t45 = call i64 @"cinch.prim.ltI64"(i64 %t44, i64 0), !range !136
  %t46 = icmp ne i64 %t45, 0
  br i1 %t46, label %bb10, label %bb11
bb10:
  %t47 = load i64, ptr %t29
  %t48 = call i64 @"cinch.runtime.bytesGet"(i64 %t47, i64 1)
  store i64 %t48, ptr %t36
  br label %bb9
bb11:
  store i64 1, ptr %t36
  br label %bb9
bb9:
  %t49 = load i64, ptr %t36
  store i64 %t49, ptr %t50
  %t51 = load i64, ptr %t35
  %t52 = load i64, ptr %t50
  %t53 = call i64 @"cinch.runtime.natAdd"(i64 %t51, i64 %t52)
  store i64 %t53, ptr %t54
  %t55 = load i64, ptr %t50
  call void @cinch.rc.drop.tobj(i64 %t55)
  %t56 = load i64, ptr %t54
  store i64 %t56, ptr %t57
  %t58 = load i64, ptr %t35
  call void @cinch.rc.dup.tobj(i64 %t58)
  %t59 = load i64, ptr %t35
  %t60 = call i64 @"cinch.runtime.natDiv"(i64 %t59, i64 5)
  store i64 %t60, ptr %t61
  %t63 = load i64, ptr %t29
  %t64 = call i64 @"cinch.runtime.bytesLen"(i64 %t63)
  store i64 %t64, ptr %t65
  %t66 = load i64, ptr %t61
  %t67 = load i64, ptr %t65
  %t68 = call i64 @"cinch.runtime.natCmp"(i64 %t66, i64 %t67)
  store i64 %t68, ptr %t69
  %t70 = load i64, ptr %t65
  call void @cinch.rc.drop.tobj(i64 %t70)
  %t71 = load i64, ptr %t69
  %t72 = call i64 @"cinch.prim.ltI64"(i64 %t71, i64 0), !range !137
  %t73 = icmp ne i64 %t72, 0
  br i1 %t73, label %bb13, label %bb14
bb13:
  %t74 = load i64, ptr %t29
  %t75 = load i64, ptr %t61
  %t76 = call i64 @"cinch.runtime.bytesGet"(i64 %t74, i64 %t75)
  store i64 %t76, ptr %t77
  %t78 = load i64, ptr %t61
  call void @cinch.rc.drop.tobj(i64 %t78)
  %t79 = load i64, ptr %t77
  store i64 %t79, ptr %t62
  br label %bb12
bb14:
  %t80 = load i64, ptr %t61
  call void @cinch.rc.drop.tobj(i64 %t80)
  store i64 1, ptr %t62
  br label %bb12
bb12:
  %t81 = load i64, ptr %t62
  store i64 %t81, ptr %t82
  %t83 = load i64, ptr %t57
  %t84 = load i64, ptr %t82
  %t85 = call i64 @"cinch.runtime.natAdd"(i64 %t83, i64 %t84)
  store i64 %t85, ptr %t86
  %t87 = load i64, ptr %t57
  call void @cinch.rc.drop.tobj(i64 %t87)
  %t88 = load i64, ptr %t82
  call void @cinch.rc.drop.tobj(i64 %t88)
  %t89 = load i64, ptr %t86
  store i64 %t89, ptr %t90
  %t92 = load i64, ptr %t35
  %t93 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t92)
  %t94 = call i64 @"cinch.prim.leI64"(i64 %t93, i64 0), !range !138
  %t95 = icmp ne i64 %t94, 0
  br i1 %t95, label %bb16, label %bb17
bb16:
  %t96 = load i64, ptr %t35
  %t97 = call i64 @"cinch.runtime.natSub"(i64 %t96, i64 3)
  store i64 %t97, ptr %t98
  %t99 = load i64, ptr %t35
  call void @cinch.rc.drop.tobj(i64 %t99)
  %t100 = load i64, ptr %t98
  store i64 %t100, ptr %t91
  br label %bb15
bb17:
  %t101 = load i64, ptr %t35
  call void @cinch.rc.drop.tobj(i64 %t101)
  store i64 1, ptr %t91
  br label %bb15
bb15:
  %t102 = load i64, ptr %t91
  store i64 %t102, ptr %t103
  %t105 = load i64, ptr %t29
  %t106 = call i64 @"cinch.runtime.bytesLen"(i64 %t105)
  store i64 %t106, ptr %t107
  %t108 = load i64, ptr %t103
  %t109 = load i64, ptr %t107
  %t110 = call i64 @"cinch.runtime.natCmp"(i64 %t108, i64 %t109)
  store i64 %t110, ptr %t111
  %t112 = load i64, ptr %t107
  call void @cinch.rc.drop.tobj(i64 %t112)
  %t113 = load i64, ptr %t111
  %t114 = call i64 @"cinch.prim.ltI64"(i64 %t113, i64 0), !range !139
  %t115 = icmp ne i64 %t114, 0
  br i1 %t115, label %bb19, label %bb20
bb19:
  %t116 = load i64, ptr %t29
  %t117 = load i64, ptr %t103
  %t118 = call i64 @"cinch.runtime.bytesGet"(i64 %t116, i64 %t117)
  store i64 %t118, ptr %t119
  %t120 = load i64, ptr %t29
  call void @cinch.rc.drop.obj(i64 %t120)
  %t121 = load i64, ptr %t103
  call void @cinch.rc.drop.tobj(i64 %t121)
  %t122 = load i64, ptr %t119
  store i64 %t122, ptr %t104
  br label %bb18
bb20:
  %t123 = load i64, ptr %t29
  call void @cinch.rc.drop.obj(i64 %t123)
  %t124 = load i64, ptr %t103
  call void @cinch.rc.drop.tobj(i64 %t124)
  store i64 1, ptr %t104
  br label %bb18
bb18:
  %t125 = load i64, ptr %t104
  store i64 %t125, ptr %t126
  %t127 = load i64, ptr %t90
  %t128 = load i64, ptr %t126
  %t129 = call i64 @"cinch.runtime.natAdd"(i64 %t127, i64 %t128)
  store i64 %t129, ptr %t130
  %t131 = load i64, ptr %t90
  call void @cinch.rc.drop.tobj(i64 %t131)
  %t132 = load i64, ptr %t126
  call void @cinch.rc.drop.tobj(i64 %t132)
  %t133 = load i64, ptr %t130
  store i64 %t133, ptr %t134
  store i64 -1, ptr %t136
  %t137 = load i64, ptr %t134
  %t138 = load i64, ptr %t136
  %t139 = call i64 @"cinch.runtime.natCmp"(i64 %t137, i64 %t138)
  store i64 %t139, ptr %t140
  %t141 = load i64, ptr %t136
  call void @cinch.rc.drop.tobj(i64 %t141)
  %t142 = load i64, ptr %t140
  %t143 = call i64 @"cinch.prim.leI64"(i64 %t142, i64 0), !range !140
  %t144 = icmp ne i64 %t143, 0
  br i1 %t144, label %bb22, label %bb23
bb22:
  %t145 = load i64, ptr %t134
  %t146 = call i64 @"cinch.runtime.natToI64"(i64 %t145)
  store i64 %t146, ptr %t147
  %t148 = load i64, ptr %t134
  call void @cinch.rc.drop.tobj(i64 %t148)
  %t149 = load i64, ptr %t147
  %t150 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t149)
  store i64 %t150, ptr %t135
  br label %bb21
bb23:
  %t151 = load i64, ptr %t134
  call void @cinch.rc.drop.tobj(i64 %t151)
  store i64 0, ptr %t135
  br label %bb21
bb21:
  %t152 = load i64, ptr %t135
  store i64 %t152, ptr %t153
  %t155 = load i64, ptr %t1
  %t156 = load i64, ptr %t8
  %t157 = call i64 @"cinch.runtime.natCmp"(i64 %t155, i64 %t156)
  %t158 = call i64 @"cinch.prim.leI64"(i64 %t157, i64 0), !range !141
  %t159 = icmp ne i64 %t158, 0
  br i1 %t159, label %bb25, label %bb26
bb25:
  %t160 = load i64, ptr %t8
  %t161 = load i64, ptr %t1
  %t162 = call i64 @"cinch.runtime.natSub"(i64 %t160, i64 %t161)
  store i64 %t162, ptr %t163
  %t164 = load i64, ptr %t1
  call void @cinch.rc.drop.tobj(i64 %t164)
  %t165 = load i64, ptr %t8
  call void @cinch.rc.drop.tobj(i64 %t165)
  %t166 = load i64, ptr %t163
  store i64 %t166, ptr %t154
  br label %bb24
bb26:
  %t167 = load i64, ptr %t1
  call void @cinch.rc.drop.tobj(i64 %t167)
  %t168 = load i64, ptr %t8
  call void @cinch.rc.drop.tobj(i64 %t168)
  store i64 1, ptr %t154
  br label %bb24
bb24:
  %t169 = load i64, ptr %t154
  store i64 %t169, ptr %t170
  %t171 = load i64, ptr %t170
  call void @cinch.rc.dup.tobj(i64 %t171)
  %t172 = load i64, ptr %t170
  %t173 = call i64 @"Text.Show.nat-digit-count"(i64 %t172)
  store i64 %t173, ptr %t174
  %t175 = load i64, ptr %t174
  call void @cinch.rc.dup.tobj(i64 %t175)
  %t176 = load i64, ptr %t174
  %t177 = call i64 @"cinch.runtime.bytesNew"(i64 %t176)
  store i64 %t177, ptr %t178
  %t179 = load i64, ptr %t178
  %t180 = load i64, ptr %t170
  %t181 = load i64, ptr %t174
  %t182 = call i64 @"Text.Show.nat-digits-into"(i64 %t179, i64 1, i64 %t180, i64 %t181)
  store i64 %t182, ptr %t183
  %t184 = load i64, ptr %t178
  %t185 = call i64 @"Text.Show.wrap-bytes"(i64 %t184)
  store i64 %t185, ptr %t186
  %t187 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.finish.dec0", i64 0, i32 1) to i64
  store i64 %t187, ptr %t188
  %t189 = load i64, ptr %t23
  %t190 = call i64 @"Main.finish.shared0"(i64 %t189)
  store i64 %t190, ptr %t191
  %t192 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.finish.dec1", i64 0, i32 1) to i64
  store i64 %t192, ptr %t193
  %t194 = load i64, ptr %t26
  %t195 = call i64 @"Main.finish.shared0"(i64 %t194)
  store i64 %t195, ptr %t196
  %t197 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.finish.dec2", i64 0, i32 1) to i64
  store i64 %t197, ptr %t198
  %t199 = load i64, ptr %t153
  %t200 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t199), !range !142
  store i64 %t200, ptr %t201
  %t202 = load i64, ptr %t201
  %t203 = call i64 @"cinch.prim.u32ToI64"(i64 %t202)
  store i64 %t203, ptr %t204
  %t205 = load i64, ptr %t204
  %t206 = call i64 @"cinch.runtime.natFromI64"(i64 %t205)
  store i64 %t206, ptr %t207
  %t208 = load i64, ptr %t153
  %t209 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t208, i64 32)
  %t210 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t209), !range !143
  store i64 %t210, ptr %t211
  %t212 = load i64, ptr %t211
  %t213 = call i64 @"cinch.prim.u32ToI64"(i64 %t212)
  store i64 %t213, ptr %t214
  %t215 = load i64, ptr %t214
  %t216 = call i64 @"cinch.runtime.natFromI64"(i64 %t215)
  store i64 %t216, ptr %t217
  store i64 8589934593, ptr %t218
  %t219 = load i64, ptr %t217
  %t220 = load i64, ptr %t218
  %t221 = call i64 @"cinch.runtime.natMul"(i64 %t219, i64 %t220)
  store i64 %t221, ptr %t222
  %t223 = load i64, ptr %t217
  call void @cinch.rc.drop.tobj(i64 %t223)
  %t224 = load i64, ptr %t218
  call void @cinch.rc.drop.tobj(i64 %t224)
  %t225 = load i64, ptr %t222
  store i64 %t225, ptr %t226
  %t227 = load i64, ptr %t207
  %t228 = load i64, ptr %t226
  %t229 = call i64 @"cinch.runtime.natAdd"(i64 %t227, i64 %t228)
  store i64 %t229, ptr %t230
  %t231 = load i64, ptr %t207
  call void @cinch.rc.drop.tobj(i64 %t231)
  %t232 = load i64, ptr %t226
  call void @cinch.rc.drop.tobj(i64 %t232)
  %t233 = load i64, ptr %t230
  store i64 %t233, ptr %t234
  %t235 = load i64, ptr %t234
  %t236 = call i64 @"Text.Show.nat-to-string"(i64 %t235)
  store i64 %t236, ptr %t237
  %t238 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.finish.dec3", i64 0, i32 1) to i64
  store i64 %t238, ptr %t239
  %t240 = load i64, ptr %t237
  %t241 = load i64, ptr %t239
  %t242 = call { i64 } @"Data.String.string-concat#cc"(i64 %t240, i64 %t241)
  %t243 = extractvalue { i64 } %t242, 0
  %t244 = call i64 @"Data.String.StrBuf"(i64 %t243)
  store i64 %t244, ptr %t245
  %t246 = load i64, ptr %t237
  call void @cinch.rc.drop.obj(i64 %t246)
  %t247 = load i64, ptr %t239
  call void @cinch.rc.drop.obj(i64 %t247)
  %t248 = load i64, ptr %t245
  store i64 %t248, ptr %t249
  %t250 = load i64, ptr %t198
  %t251 = load i64, ptr %t249
  %t252 = call { i64 } @"Data.String.string-concat#cc"(i64 %t250, i64 %t251)
  %t253 = extractvalue { i64 } %t252, 0
  %t254 = call i64 @"Data.String.StrBuf"(i64 %t253)
  store i64 %t254, ptr %t255
  %t256 = load i64, ptr %t198
  call void @cinch.rc.drop.obj(i64 %t256)
  %t257 = load i64, ptr %t249
  call void @cinch.rc.drop.obj(i64 %t257)
  %t258 = load i64, ptr %t255
  store i64 %t258, ptr %t259
  %t260 = load i64, ptr %t196
  %t261 = load i64, ptr %t259
  %t262 = call { i64 } @"Data.String.string-concat#cc"(i64 %t260, i64 %t261)
  %t263 = extractvalue { i64 } %t262, 0
  %t264 = call i64 @"Data.String.StrBuf"(i64 %t263)
  store i64 %t264, ptr %t265
  %t266 = load i64, ptr %t196
  call void @cinch.rc.drop.obj(i64 %t266)
  %t267 = load i64, ptr %t259
  call void @cinch.rc.drop.obj(i64 %t267)
  %t268 = load i64, ptr %t265
  store i64 %t268, ptr %t269
  %t270 = load i64, ptr %t193
  %t271 = load i64, ptr %t269
  %t272 = call { i64 } @"Data.String.string-concat#cc"(i64 %t270, i64 %t271)
  %t273 = extractvalue { i64 } %t272, 0
  %t274 = call i64 @"Data.String.StrBuf"(i64 %t273)
  store i64 %t274, ptr %t275
  %t276 = load i64, ptr %t193
  call void @cinch.rc.drop.obj(i64 %t276)
  %t277 = load i64, ptr %t269
  call void @cinch.rc.drop.obj(i64 %t277)
  %t278 = load i64, ptr %t275
  store i64 %t278, ptr %t279
  %t280 = load i64, ptr %t191
  %t281 = load i64, ptr %t279
  %t282 = call { i64 } @"Data.String.string-concat#cc"(i64 %t280, i64 %t281)
  %t283 = extractvalue { i64 } %t282, 0
  %t284 = call i64 @"Data.String.StrBuf"(i64 %t283)
  store i64 %t284, ptr %t285
  %t286 = load i64, ptr %t191
  call void @cinch.rc.drop.obj(i64 %t286)
  %t287 = load i64, ptr %t279
  call void @cinch.rc.drop.obj(i64 %t287)
  %t288 = load i64, ptr %t285
  store i64 %t288, ptr %t289
  %t290 = load i64, ptr %t188
  %t291 = load i64, ptr %t289
  %t292 = call { i64 } @"Data.String.string-concat#cc"(i64 %t290, i64 %t291)
  %t293 = extractvalue { i64 } %t292, 0
  %t294 = call i64 @"Data.String.StrBuf"(i64 %t293)
  store i64 %t294, ptr %t295
  %t296 = load i64, ptr %t188
  call void @cinch.rc.drop.obj(i64 %t296)
  %t297 = load i64, ptr %t289
  call void @cinch.rc.drop.obj(i64 %t297)
  %t298 = load i64, ptr %t295
  store i64 %t298, ptr %t299
  %t300 = load i64, ptr %t186
  %t301 = load i64, ptr %t299
  %t302 = call { i64 } @"Data.String.string-concat#cc"(i64 %t300, i64 %t301)
  %t303 = extractvalue { i64 } %t302, 0
  %t304 = call i64 @"Data.String.StrBuf"(i64 %t303)
  store i64 %t304, ptr %t305
  %t306 = load i64, ptr %t186
  call void @cinch.rc.drop.obj(i64 %t306)
  %t307 = load i64, ptr %t299
  call void @cinch.rc.drop.obj(i64 %t307)
  %t308 = load i64, ptr %t305
  store i64 %t308, ptr %t309
  %t310 = load i64, ptr %t309
  %t311 = call i64 @"Data.String.string-to-bytes"(i64 %t310)
  store i64 %t311, ptr %t312
  %t313 = load i64, ptr %t309
  call void @cinch.rc.drop.obj(i64 %t313)
  %t314 = load i64, ptr %t312
  store i64 %t314, ptr %t315
  %t316 = load i64, ptr %t0
  %t317 = load i64, ptr %t315
  %t318 = call i64 @"cinch.runtime.ioWrite"(i64 %t316, i64 %t317)
  store i64 %t318, ptr %t319
  %t320 = load i64, ptr %t315
  call void @cinch.rc.drop.obj(i64 %t320)
  %t321 = load i64, ptr %t319
  store i64 %t321, ptr %t322
  %t323 = load i64, ptr %t23
  %t324 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t323), !range !144
  store i64 %t324, ptr %t325
  %t326 = load i64, ptr %t325
  %t327 = call i64 @"cinch.prim.u32ToI64"(i64 %t326)
  store i64 %t327, ptr %t328
  %t329 = load i64, ptr %t328
  %t330 = call i64 @"cinch.runtime.natFromI64"(i64 %t329)
  store i64 %t330, ptr %t331
  %t332 = load i64, ptr %t23
  %t333 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t332, i64 32)
  %t334 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t333), !range !145
  store i64 %t334, ptr %t335
  %t336 = load i64, ptr %t335
  %t337 = call i64 @"cinch.prim.u32ToI64"(i64 %t336)
  store i64 %t337, ptr %t338
  %t339 = load i64, ptr %t338
  %t340 = call i64 @"cinch.runtime.natFromI64"(i64 %t339)
  store i64 %t340, ptr %t341
  store i64 8589934593, ptr %t342
  %t343 = load i64, ptr %t341
  %t344 = load i64, ptr %t342
  %t345 = call i64 @"cinch.runtime.natMul"(i64 %t343, i64 %t344)
  store i64 %t345, ptr %t346
  %t347 = load i64, ptr %t341
  call void @cinch.rc.drop.tobj(i64 %t347)
  %t348 = load i64, ptr %t342
  call void @cinch.rc.drop.tobj(i64 %t348)
  %t349 = load i64, ptr %t346
  store i64 %t349, ptr %t350
  %t351 = load i64, ptr %t331
  %t352 = load i64, ptr %t350
  %t353 = call i64 @"cinch.runtime.natAdd"(i64 %t351, i64 %t352)
  store i64 %t353, ptr %t354
  %t355 = load i64, ptr %t331
  call void @cinch.rc.drop.tobj(i64 %t355)
  %t356 = load i64, ptr %t350
  call void @cinch.rc.drop.tobj(i64 %t356)
  %t357 = load i64, ptr %t354
  store i64 %t357, ptr %t358
  %t359 = load i64, ptr %t358
  %t360 = load i64, ptr %t2
  %t361 = call i64 @"cinch.runtime.natCmp"(i64 %t359, i64 %t360)
  store i64 %t361, ptr %t362
  %t363 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t363)
  %t364 = load i64, ptr %t358
  call void @cinch.rc.drop.tobj(i64 %t364)
  %t365 = load i64, ptr %t362
  %t366 = call i64 @"cinch.prim.eqI64"(i64 %t365, i64 0), !range !146
  %t367 = icmp ne i64 %t366, 0
  br i1 %t367, label %bb27, label %bb28
bb27:
  ret i64 1
bb28:
  ret i64 3
bb1:
  ret i64 0
}
