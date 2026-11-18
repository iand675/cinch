define i64 @"Main.measure"(i64 %p0, i64 %p1, i64 %p2, i64 %p3) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t2 = alloca i64
  %t3 = alloca i64
  %t4 = alloca i64
  %t5 = alloca i64
  %t9 = alloca i64
  %t22 = alloca i64
  %t24 = alloca i64
  %t26 = alloca i64
  %t30 = alloca i64
  %t35 = alloca i64
  %t38 = alloca i64
  %t39 = alloca i64
  %t45 = alloca i64
  %t46 = alloca i64
  %t49 = alloca i64
  %t53 = alloca i64
  %t58 = alloca i64
  %t61 = alloca i64
  %t68 = alloca i64
  %t71 = alloca i64
  %t79 = alloca i64
  %t81 = alloca i64
  %t84 = alloca i64
  %t88 = alloca i64
  %t96 = alloca i64
  %t97 = alloca i64
  %t100 = alloca i64
  %t104 = alloca i64
  %t109 = alloca i64
  %t110 = alloca i64
  %t114 = alloca i64
  %t121 = alloca i64
  %t126 = alloca i64
  %t127 = alloca i64
  %t131 = alloca i64
  %t138 = alloca i64
  %t144 = alloca i64
  %t165 = alloca i64
  %t178 = alloca i64
  %t180 = alloca i64
  %t183 = alloca i64
  %t199 = alloca i64
  %t202 = alloca i64
  %t205 = alloca i64
  %t211 = alloca i64
  %t214 = alloca i64
  %t215 = alloca i64
  %t224 = alloca i64
  %t231 = alloca i64
  %t235 = alloca i64
  %t239 = alloca i64
  %t244 = alloca i64
  %t247 = alloca i64
  %t249 = alloca i64
  %t252 = alloca i64
  %t254 = alloca i64
  %t257 = alloca i64
  %t259 = alloca i64
  %t262 = alloca i64
  %t265 = alloca i64
  %t268 = alloca i64
  %t272 = alloca i64
  %t275 = alloca i64
  %t278 = alloca i64
  %t279 = alloca i64
  %t283 = alloca i64
  %t287 = alloca i64
  %t291 = alloca i64
  %t295 = alloca i64
  %t298 = alloca i64
  %t300 = alloca i64
  %t306 = alloca i64
  %t310 = alloca i64
  %t316 = alloca i64
  %t320 = alloca i64
  %t326 = alloca i64
  %t330 = alloca i64
  %t336 = alloca i64
  %t340 = alloca i64
  %t346 = alloca i64
  %t350 = alloca i64
  %t356 = alloca i64
  %t360 = alloca i64
  %t366 = alloca i64
  %t370 = alloca i64
  %t373 = alloca i64
  %t376 = alloca i64
  %t380 = alloca i64
  %t383 = alloca i64
  %t386 = alloca i64
  %t389 = alloca i64
  %t392 = alloca i64
  %t396 = alloca i64
  %t399 = alloca i64
  %t402 = alloca i64
  %t403 = alloca i64
  %t407 = alloca i64
  %t411 = alloca i64
  %t415 = alloca i64
  %t419 = alloca i64
  %t423 = alloca i64
  %t430 = alloca i64
  %t434 = alloca i64
  %t439 = alloca i64
  %t442 = alloca i64
  %t453 = alloca i64
  %t462 = alloca i64
  %t466 = alloca i64
  %t472 = alloca i64
  %t482 = alloca i64
  %t490 = alloca i64
  %t494 = alloca i64
  %t500 = alloca i64
  %t511 = alloca i64
  %t519 = alloca i64
  %t523 = alloca i64
  %t529 = alloca i64
  %t540 = alloca i64
  %t548 = alloca i64
  %t552 = alloca i64
  %t560 = alloca i64
  %t563 = alloca i64
  %t575 = alloca i64
  store i64 %p0, ptr %t0
  store i64 %p1, ptr %t1
  store i64 %p2, ptr %t2
  store i64 %p3, ptr %t3
  br label %bb0
bb0:
  store i64 8589934591, ptr %t5
  %t6 = load i64, ptr %t3
  %t7 = load i64, ptr %t5
  %t8 = call i64 @"cinch.runtime.natCmp"(i64 %t6, i64 %t7)
  store i64 %t8, ptr %t9
  %t10 = load i64, ptr %t5
  call void @cinch.rc.drop.tobj(i64 %t10)
  %t11 = load i64, ptr %t9
  %t12 = call i64 @"cinch.prim.leI64"(i64 %t11, i64 0), !range !147
  %t13 = icmp ne i64 %t12, 0
  br i1 %t13, label %bb2, label %bb3
bb2:
  %t14 = load i64, ptr %t2
  %t15 = call i64 @"cinch.runtime.natCmp"(i64 %t14, i64 8193)
  %t16 = call i64 @"cinch.prim.leI64"(i64 %t15, i64 0), !range !148
  store i64 %t16, ptr %t4
  br label %bb1
bb3:
  store i64 0, ptr %t4
  br label %bb1
bb1:
  %t17 = load i64, ptr %t4
  %t18 = icmp ne i64 %t17, 0
  br i1 %t18, label %bb4, label %bb5
bb4:
  %t19 = load i64, ptr %t3
  %t20 = call i64 @"cinch.runtime.natToI64"(i64 %t19)
  %t21 = call i64 @"cinch.prim.wrappingI64ToU32"(i64 %t20), !range !149
  store i64 %t21, ptr %t22
  %t23 = call i64 @"cinch.runtime.bytesNew"(i64 1)
  store i64 %t23, ptr %t24
  %t25 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [6 x i8] }, ptr @"Main.measure.dec0", i64 0, i32 1) to i64
  store i64 %t25, ptr %t26
  %t27 = load i64, ptr %t1
  %t28 = load i64, ptr %t26
  %t29 = call i64 @"Data.String.Eq_String.=="(i64 %t27, i64 %t28), !range !150
  store i64 %t29, ptr %t30
  %t31 = load i64, ptr %t26
  call void @cinch.rc.drop.obj(i64 %t31)
  %t32 = load i64, ptr %t30
  %t33 = icmp ne i64 %t32, 0
  br i1 %t33, label %bb6, label %bb7
bb6:
  %t34 = call i64 @"Effect.Clock.raw-monotonic-ns"(i64 0)
  store i64 %t34, ptr %t35
  %t36 = load i64, ptr %t35
  %t37 = call i64 @"cinch.runtime.natFromI64"(i64 %t36)
  store i64 %t37, ptr %t38
  %t40 = load i64, ptr %t22
  %t41 = call i64 @"Main.before"(i64 0, i64 %t40)
  %t42 = icmp ne i64 %t41, 0
  br i1 %t42, label %bb9, label %bb10
bb9:
  %t43 = load i64, ptr %t2
  %t44 = call i64 @"cinch.runtime.randomBytes"(i64 %t43)
  store i64 %t44, ptr %t45
  %t47 = load i64, ptr %t45
  %t48 = call i64 @"cinch.runtime.bytesLen"(i64 %t47)
  store i64 %t48, ptr %t49
  %t50 = load i64, ptr %t49
  %t51 = load i64, ptr %t2
  %t52 = call i64 @"cinch.runtime.natCmp"(i64 %t50, i64 %t51)
  store i64 %t52, ptr %t53
  %t54 = load i64, ptr %t49
  call void @cinch.rc.drop.tobj(i64 %t54)
  %t55 = load i64, ptr %t53
  %t56 = call i64 @"cinch.prim.eqI64"(i64 %t55, i64 0), !range !151
  %t57 = icmp ne i64 %t56, 0
  br i1 %t57, label %bb12, label %bb13
bb12:
  store i64 -1, ptr %t58
  %t59 = load i64, ptr %t58
  %t60 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t59)
  store i64 %t60, ptr %t61
  %t62 = load i64, ptr %t58
  call void @cinch.rc.drop.tobj(i64 %t62)
  %t63 = load i64, ptr %t61
  %t64 = call i64 @"cinch.prim.leI64"(i64 %t63, i64 0), !range !152
  %t65 = icmp ne i64 %t64, 0
  br i1 %t65, label %bb14, label %bb15
bb14:
  %t66 = call i64 @"cinch.runtime.natToI64"(i64 3)
  %t67 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t66)
  store i64 %t67, ptr %t46
  br label %bb11
bb15:
  store i64 0, ptr %t46
  br label %bb11
bb13:
  store i64 -1, ptr %t68
  %t69 = load i64, ptr %t68
  %t70 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t69)
  store i64 %t70, ptr %t71
  %t72 = load i64, ptr %t68
  call void @cinch.rc.drop.tobj(i64 %t72)
  %t73 = load i64, ptr %t71
  %t74 = call i64 @"cinch.prim.leI64"(i64 %t73, i64 0), !range !153
  %t75 = icmp ne i64 %t74, 0
  br i1 %t75, label %bb16, label %bb17
bb16:
  %t76 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t77 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t76)
  store i64 %t77, ptr %t46
  br label %bb11
bb17:
  store i64 0, ptr %t46
  br label %bb11
bb11:
  %t78 = load i64, ptr %t46
  store i64 %t78, ptr %t79
  %t80 = call i64 @"Main.bump"(i64 0)
  store i64 %t80, ptr %t81
  %t82 = load i64, ptr %t79
  %t83 = call i64 @"Main.add"(i64 0, i64 %t82)
  store i64 %t83, ptr %t84
  %t85 = load i64, ptr %t45
  %t86 = call i64 @"Main.measure.shared0"(i64 %t85)
  %t87 = call i64 @"Main.mix"(i64 -3750763034362895579, i64 %t86)
  store i64 %t87, ptr %t88
  %t89 = load i64, ptr %t24
  call void @cinch.rc.drop.obj(i64 %t89)
  %t90 = load i64, ptr %t81
  %t91 = load i64, ptr %t22
  %t92 = call i64 @"Main.before"(i64 %t90, i64 %t91)
  %t93 = icmp ne i64 %t92, 0
  br i1 %t93, label %bb18, label %bb19
bb18:
  %t94 = load i64, ptr %t2
  %t95 = call i64 @"cinch.runtime.randomBytes"(i64 %t94)
  store i64 %t95, ptr %t96
  %t98 = load i64, ptr %t96
  %t99 = call i64 @"cinch.runtime.bytesLen"(i64 %t98)
  store i64 %t99, ptr %t100
  %t101 = load i64, ptr %t100
  %t102 = load i64, ptr %t2
  %t103 = call i64 @"cinch.runtime.natCmp"(i64 %t101, i64 %t102)
  store i64 %t103, ptr %t104
  %t105 = load i64, ptr %t100
  call void @cinch.rc.drop.tobj(i64 %t105)
  %t106 = load i64, ptr %t104
  %t107 = call i64 @"cinch.prim.eqI64"(i64 %t106, i64 0), !range !154
  %t108 = icmp ne i64 %t107, 0
  br i1 %t108, label %bb21, label %bb22
bb21:
  store i64 3, ptr %t109
  store i64 -1, ptr %t110
  %t111 = load i64, ptr %t109
  %t112 = load i64, ptr %t110
  %t113 = call i64 @"cinch.runtime.natCmp"(i64 %t111, i64 %t112)
  store i64 %t113, ptr %t114
  %t115 = load i64, ptr %t110
  call void @cinch.rc.drop.tobj(i64 %t115)
  %t116 = load i64, ptr %t114
  %t117 = call i64 @"cinch.prim.leI64"(i64 %t116, i64 0), !range !155
  %t118 = icmp ne i64 %t117, 0
  br i1 %t118, label %bb23, label %bb24
bb23:
  %t119 = load i64, ptr %t109
  %t120 = call i64 @"cinch.runtime.natToI64"(i64 %t119)
  store i64 %t120, ptr %t121
  %t122 = load i64, ptr %t109
  call void @cinch.rc.drop.tobj(i64 %t122)
  %t123 = load i64, ptr %t121
  %t124 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t123)
  store i64 %t124, ptr %t97
  br label %bb20
bb24:
  %t125 = load i64, ptr %t109
  call void @cinch.rc.drop.tobj(i64 %t125)
  store i64 0, ptr %t97
  br label %bb20
bb22:
  store i64 1, ptr %t126
  store i64 -1, ptr %t127
  %t128 = load i64, ptr %t126
  %t129 = load i64, ptr %t127
  %t130 = call i64 @"cinch.runtime.natCmp"(i64 %t128, i64 %t129)
  store i64 %t130, ptr %t131
  %t132 = load i64, ptr %t127
  call void @cinch.rc.drop.tobj(i64 %t132)
  %t133 = load i64, ptr %t131
  %t134 = call i64 @"cinch.prim.leI64"(i64 %t133, i64 0), !range !156
  %t135 = icmp ne i64 %t134, 0
  br i1 %t135, label %bb25, label %bb26
bb25:
  %t136 = load i64, ptr %t126
  %t137 = call i64 @"cinch.runtime.natToI64"(i64 %t136)
  store i64 %t137, ptr %t138
  %t139 = load i64, ptr %t126
  call void @cinch.rc.drop.tobj(i64 %t139)
  %t140 = load i64, ptr %t138
  %t141 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t140)
  store i64 %t141, ptr %t97
  br label %bb20
bb26:
  %t142 = load i64, ptr %t126
  call void @cinch.rc.drop.tobj(i64 %t142)
  store i64 0, ptr %t97
  br label %bb20
bb20:
  %t143 = load i64, ptr %t97
  store i64 %t143, ptr %t144
  %t145 = load i64, ptr %t0
  %t146 = load i64, ptr %t2
  %t147 = load i64, ptr %t22
  %t148 = load i64, ptr %t81
  %t149 = call i64 @"Main.bump"(i64 %t148)
  %t150 = load i64, ptr %t84
  %t151 = load i64, ptr %t144
  %t152 = call i64 @"Main.add"(i64 %t150, i64 %t151)
  %t153 = load i64, ptr %t88
  %t154 = load i64, ptr %t96
  %t155 = call i64 @"Main.fingerprint"(i64 %t154)
  %t156 = call i64 @"Main.mix"(i64 %t153, i64 %t155)
  %t157 = load i64, ptr %t96
  %t158 = trunc i64 %t147 to i32
  %t159 = trunc i64 %t149 to i32
  %t160 = call { i64, i64, i64 } @"Main.random-loop#cc"(i64 %t145, i64 %t146, i32 %t158, i32 %t159, i64 %t152, i64 %t156, i64 %t157)
  %t161 = extractvalue { i64, i64, i64 } %t160, 0
  %t162 = extractvalue { i64, i64, i64 } %t160, 1
  %t163 = extractvalue { i64, i64, i64 } %t160, 2
  %t164 = call i64 @"Main.ByteRun"(i64 %t161, i64 %t162, i64 %t163)
  store i64 %t164, ptr %t165
  %t166 = load i64, ptr %t45
  call void @cinch.rc.drop.obj(i64 %t166)
  %t167 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t167)
  %t168 = load i64, ptr %t165
  store i64 %t168, ptr %t39
  br label %bb8
bb19:
  %t169 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t169)
  %t170 = load i64, ptr %t84
  %t171 = load i64, ptr %t88
  %t172 = load i64, ptr %t45
  %t173 = call i64 @"Main.ByteRun"(i64 %t170, i64 %t171, i64 %t172)
  store i64 %t173, ptr %t39
  br label %bb8
bb10:
  %t174 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t174)
  %t175 = load i64, ptr %t24
  %t176 = call i64 @"Main.ByteRun"(i64 0, i64 -3750763034362895579, i64 %t175)
  store i64 %t176, ptr %t39
  br label %bb8
bb8:
  %t177 = load i64, ptr %t39
  store i64 %t177, ptr %t178
  %t179 = call i64 @"Effect.Clock.raw-monotonic-ns"(i64 0)
  store i64 %t179, ptr %t180
  %t181 = load i64, ptr %t180
  %t182 = call i64 @"cinch.runtime.natFromI64"(i64 %t181)
  store i64 %t182, ptr %t183
  %t184 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t184)
  %t185 = load i64, ptr %t178
  %t186 = inttoptr i64 %t185 to ptr
  %t187 = getelementptr inbounds i8, ptr %t186, i64 -8
  %t188 = load atomic i64, ptr %t187 monotonic, align 8
  %t189 = lshr i64 %t188, 40
  %t190 = and i64 %t189, 255
  %t191 = icmp eq i64 %t190, 255
  br i1 %t191, label %bb28, label %bb29
bb28:
  %t192 = getelementptr inbounds i8, ptr %t186, i64 -16
  %t193 = load i64, ptr %t192
  %t194 = inttoptr i64 %t193 to ptr
  %t195 = load i64, ptr %t194
  br label %bb30
bb29:
  br label %bb30
bb30:
  %t196 = phi i64 [ %t195, %bb28 ], [ %t190, %bb29 ]
  switch i64 %t196, label %bb27 [
  i64 0, label %bb31
]
bb31:
  %t197 = getelementptr inbounds i8, ptr %t186, i64 0
  %t198 = load i64, ptr %t197
  store i64 %t198, ptr %t199
  %t200 = getelementptr inbounds i8, ptr %t186, i64 8
  %t201 = load i64, ptr %t200
  store i64 %t201, ptr %t202
  %t203 = getelementptr inbounds i8, ptr %t186, i64 16
  %t204 = load i64, ptr %t203
  store i64 %t204, ptr %t205
  %t206 = load i64, ptr %t178
  %t207 = call i1 @cinch.rc.unique(i64 %t206)
  br i1 %t207, label %bb32, label %bb33
bb32:
  call void @cinch_free(i64 %t206)
  br label %bb34
bb33:
  %t208 = load i64, ptr %t205
  call void @cinch.rc.dup.obj(i64 %t208)
  call void @cinch.rc.drop.obj(i64 %t206)
  br label %bb34
bb34:
  %t209 = load i64, ptr %t205
  %t210 = call i64 @"Main.measure.shared0"(i64 %t209)
  store i64 %t210, ptr %t211
  %t212 = load i64, ptr %t205
  call void @cinch.rc.drop.obj(i64 %t212)
  %t213 = load i64, ptr %t211
  store i64 %t213, ptr %t214
  %t216 = load i64, ptr %t38
  %t217 = load i64, ptr %t183
  %t218 = call i64 @"cinch.runtime.natCmp"(i64 %t216, i64 %t217)
  %t219 = call i64 @"cinch.prim.leI64"(i64 %t218, i64 0), !range !157
  %t220 = icmp ne i64 %t219, 0
  br i1 %t220, label %bb36, label %bb37
bb36:
  %t221 = load i64, ptr %t183
  %t222 = load i64, ptr %t38
  %t223 = call i64 @"cinch.runtime.natSub"(i64 %t221, i64 %t222)
  store i64 %t223, ptr %t224
  %t225 = load i64, ptr %t38
  call void @cinch.rc.drop.tobj(i64 %t225)
  %t226 = load i64, ptr %t183
  call void @cinch.rc.drop.tobj(i64 %t226)
  %t227 = load i64, ptr %t224
  store i64 %t227, ptr %t215
  br label %bb35
bb37:
  %t228 = load i64, ptr %t38
  call void @cinch.rc.drop.tobj(i64 %t228)
  %t229 = load i64, ptr %t183
  call void @cinch.rc.drop.tobj(i64 %t229)
  store i64 1, ptr %t215
  br label %bb35
bb35:
  %t230 = load i64, ptr %t215
  store i64 %t230, ptr %t231
  %t232 = load i64, ptr %t231
  call void @cinch.rc.dup.tobj(i64 %t232)
  %t233 = load i64, ptr %t231
  %t234 = call i64 @"Text.Show.nat-digit-count"(i64 %t233)
  store i64 %t234, ptr %t235
  %t236 = load i64, ptr %t235
  call void @cinch.rc.dup.tobj(i64 %t236)
  %t237 = load i64, ptr %t235
  %t238 = call i64 @"cinch.runtime.bytesNew"(i64 %t237)
  store i64 %t238, ptr %t239
  %t240 = load i64, ptr %t239
  %t241 = load i64, ptr %t231
  %t242 = load i64, ptr %t235
  %t243 = call i64 @"Text.Show.nat-digits-into"(i64 %t240, i64 1, i64 %t241, i64 %t242)
  store i64 %t243, ptr %t244
  %t245 = load i64, ptr %t239
  %t246 = call i64 @"Text.Show.wrap-bytes"(i64 %t245)
  store i64 %t246, ptr %t247
  %t248 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.measure.dec1", i64 0, i32 1) to i64
  store i64 %t248, ptr %t249
  %t250 = load i64, ptr %t199
  %t251 = call i64 @"Main.measure.shared1"(i64 %t250)
  store i64 %t251, ptr %t252
  %t253 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.measure.dec2", i64 0, i32 1) to i64
  store i64 %t253, ptr %t254
  %t255 = load i64, ptr %t202
  %t256 = call i64 @"Main.measure.shared1"(i64 %t255)
  store i64 %t256, ptr %t257
  %t258 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.measure.dec3", i64 0, i32 1) to i64
  store i64 %t258, ptr %t259
  %t260 = load i64, ptr %t214
  %t261 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t260), !range !158
  store i64 %t261, ptr %t262
  %t263 = load i64, ptr %t262
  %t264 = call i64 @"cinch.prim.u32ToI64"(i64 %t263)
  store i64 %t264, ptr %t265
  %t266 = load i64, ptr %t265
  %t267 = call i64 @"cinch.runtime.natFromI64"(i64 %t266)
  store i64 %t267, ptr %t268
  %t269 = load i64, ptr %t214
  %t270 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t269, i64 32)
  %t271 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t270), !range !159
  store i64 %t271, ptr %t272
  %t273 = load i64, ptr %t272
  %t274 = call i64 @"cinch.prim.u32ToI64"(i64 %t273)
  store i64 %t274, ptr %t275
  %t276 = load i64, ptr %t275
  %t277 = call i64 @"cinch.runtime.natFromI64"(i64 %t276)
  store i64 %t277, ptr %t278
  store i64 8589934593, ptr %t279
  %t280 = load i64, ptr %t278
  %t281 = load i64, ptr %t279
  %t282 = call i64 @"cinch.runtime.natMul"(i64 %t280, i64 %t281)
  store i64 %t282, ptr %t283
  %t284 = load i64, ptr %t278
  call void @cinch.rc.drop.tobj(i64 %t284)
  %t285 = load i64, ptr %t279
  call void @cinch.rc.drop.tobj(i64 %t285)
  %t286 = load i64, ptr %t283
  store i64 %t286, ptr %t287
  %t288 = load i64, ptr %t268
  %t289 = load i64, ptr %t287
  %t290 = call i64 @"cinch.runtime.natAdd"(i64 %t288, i64 %t289)
  store i64 %t290, ptr %t291
  %t292 = load i64, ptr %t268
  call void @cinch.rc.drop.tobj(i64 %t292)
  %t293 = load i64, ptr %t287
  call void @cinch.rc.drop.tobj(i64 %t293)
  %t294 = load i64, ptr %t291
  store i64 %t294, ptr %t295
  %t296 = load i64, ptr %t295
  %t297 = call i64 @"Text.Show.nat-to-string"(i64 %t296)
  store i64 %t297, ptr %t298
  %t299 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [1 x i8] }, ptr @"Main.measure.dec4", i64 0, i32 1) to i64
  store i64 %t299, ptr %t300
  %t301 = load i64, ptr %t298
  %t302 = load i64, ptr %t300
  %t303 = call { i64 } @"Data.String.string-concat#cc"(i64 %t301, i64 %t302)
  %t304 = extractvalue { i64 } %t303, 0
  %t305 = call i64 @"Data.String.StrBuf"(i64 %t304)
  store i64 %t305, ptr %t306
  %t307 = load i64, ptr %t298
  call void @cinch.rc.drop.obj(i64 %t307)
  %t308 = load i64, ptr %t300
  call void @cinch.rc.drop.obj(i64 %t308)
  %t309 = load i64, ptr %t306
  store i64 %t309, ptr %t310
  %t311 = load i64, ptr %t259
  %t312 = load i64, ptr %t310
  %t313 = call { i64 } @"Data.String.string-concat#cc"(i64 %t311, i64 %t312)
  %t314 = extractvalue { i64 } %t313, 0
  %t315 = call i64 @"Data.String.StrBuf"(i64 %t314)
  store i64 %t315, ptr %t316
  %t317 = load i64, ptr %t259
  call void @cinch.rc.drop.obj(i64 %t317)
  %t318 = load i64, ptr %t310
  call void @cinch.rc.drop.obj(i64 %t318)
  %t319 = load i64, ptr %t316
  store i64 %t319, ptr %t320
  %t321 = load i64, ptr %t257
  %t322 = load i64, ptr %t320
  %t323 = call { i64 } @"Data.String.string-concat#cc"(i64 %t321, i64 %t322)
  %t324 = extractvalue { i64 } %t323, 0
  %t325 = call i64 @"Data.String.StrBuf"(i64 %t324)
  store i64 %t325, ptr %t326
  %t327 = load i64, ptr %t257
  call void @cinch.rc.drop.obj(i64 %t327)
  %t328 = load i64, ptr %t320
  call void @cinch.rc.drop.obj(i64 %t328)
  %t329 = load i64, ptr %t326
  store i64 %t329, ptr %t330
  %t331 = load i64, ptr %t254
  %t332 = load i64, ptr %t330
  %t333 = call { i64 } @"Data.String.string-concat#cc"(i64 %t331, i64 %t332)
  %t334 = extractvalue { i64 } %t333, 0
  %t335 = call i64 @"Data.String.StrBuf"(i64 %t334)
  store i64 %t335, ptr %t336
  %t337 = load i64, ptr %t254
  call void @cinch.rc.drop.obj(i64 %t337)
  %t338 = load i64, ptr %t330
  call void @cinch.rc.drop.obj(i64 %t338)
  %t339 = load i64, ptr %t336
  store i64 %t339, ptr %t340
  %t341 = load i64, ptr %t252
  %t342 = load i64, ptr %t340
  %t343 = call { i64 } @"Data.String.string-concat#cc"(i64 %t341, i64 %t342)
  %t344 = extractvalue { i64 } %t343, 0
  %t345 = call i64 @"Data.String.StrBuf"(i64 %t344)
  store i64 %t345, ptr %t346
  %t347 = load i64, ptr %t252
  call void @cinch.rc.drop.obj(i64 %t347)
  %t348 = load i64, ptr %t340
  call void @cinch.rc.drop.obj(i64 %t348)
  %t349 = load i64, ptr %t346
  store i64 %t349, ptr %t350
  %t351 = load i64, ptr %t249
  %t352 = load i64, ptr %t350
  %t353 = call { i64 } @"Data.String.string-concat#cc"(i64 %t351, i64 %t352)
  %t354 = extractvalue { i64 } %t353, 0
  %t355 = call i64 @"Data.String.StrBuf"(i64 %t354)
  store i64 %t355, ptr %t356
  %t357 = load i64, ptr %t249
  call void @cinch.rc.drop.obj(i64 %t357)
  %t358 = load i64, ptr %t350
  call void @cinch.rc.drop.obj(i64 %t358)
  %t359 = load i64, ptr %t356
  store i64 %t359, ptr %t360
  %t361 = load i64, ptr %t247
  %t362 = load i64, ptr %t360
  %t363 = call { i64 } @"Data.String.string-concat#cc"(i64 %t361, i64 %t362)
  %t364 = extractvalue { i64 } %t363, 0
  %t365 = call i64 @"Data.String.StrBuf"(i64 %t364)
  store i64 %t365, ptr %t366
  %t367 = load i64, ptr %t247
  call void @cinch.rc.drop.obj(i64 %t367)
  %t368 = load i64, ptr %t360
  call void @cinch.rc.drop.obj(i64 %t368)
  %t369 = load i64, ptr %t366
  store i64 %t369, ptr %t370
  %t371 = load i64, ptr %t370
  %t372 = call i64 @"Data.String.string-to-bytes"(i64 %t371)
  store i64 %t372, ptr %t373
  %t374 = load i64, ptr %t370
  call void @cinch.rc.drop.obj(i64 %t374)
  %t375 = load i64, ptr %t373
  store i64 %t375, ptr %t376
  %t377 = load i64, ptr %t0
  %t378 = load i64, ptr %t376
  %t379 = call i64 @"cinch.runtime.ioWrite"(i64 %t377, i64 %t378)
  store i64 %t379, ptr %t380
  %t381 = load i64, ptr %t376
  call void @cinch.rc.drop.obj(i64 %t381)
  %t382 = load i64, ptr %t380
  store i64 %t382, ptr %t383
  %t384 = load i64, ptr %t199
  %t385 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t384), !range !160
  store i64 %t385, ptr %t386
  %t387 = load i64, ptr %t386
  %t388 = call i64 @"cinch.prim.u32ToI64"(i64 %t387)
  store i64 %t388, ptr %t389
  %t390 = load i64, ptr %t389
  %t391 = call i64 @"cinch.runtime.natFromI64"(i64 %t390)
  store i64 %t391, ptr %t392
  %t393 = load i64, ptr %t199
  %t394 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t393, i64 32)
  %t395 = call i64 @"cinch.prim.wrappingU64ToU32"(i64 %t394), !range !161
  store i64 %t395, ptr %t396
  %t397 = load i64, ptr %t396
  %t398 = call i64 @"cinch.prim.u32ToI64"(i64 %t397)
  store i64 %t398, ptr %t399
  %t400 = load i64, ptr %t399
  %t401 = call i64 @"cinch.runtime.natFromI64"(i64 %t400)
  store i64 %t401, ptr %t402
  store i64 8589934593, ptr %t403
  %t404 = load i64, ptr %t402
  %t405 = load i64, ptr %t403
  %t406 = call i64 @"cinch.runtime.natMul"(i64 %t404, i64 %t405)
  store i64 %t406, ptr %t407
  %t408 = load i64, ptr %t402
  call void @cinch.rc.drop.tobj(i64 %t408)
  %t409 = load i64, ptr %t403
  call void @cinch.rc.drop.tobj(i64 %t409)
  %t410 = load i64, ptr %t407
  store i64 %t410, ptr %t411
  %t412 = load i64, ptr %t392
  %t413 = load i64, ptr %t411
  %t414 = call i64 @"cinch.runtime.natAdd"(i64 %t412, i64 %t413)
  store i64 %t414, ptr %t415
  %t416 = load i64, ptr %t392
  call void @cinch.rc.drop.tobj(i64 %t416)
  %t417 = load i64, ptr %t411
  call void @cinch.rc.drop.tobj(i64 %t417)
  %t418 = load i64, ptr %t415
  store i64 %t418, ptr %t419
  %t420 = load i64, ptr %t419
  %t421 = load i64, ptr %t3
  %t422 = call i64 @"cinch.runtime.natCmp"(i64 %t420, i64 %t421)
  store i64 %t422, ptr %t423
  %t424 = load i64, ptr %t3
  call void @cinch.rc.drop.tobj(i64 %t424)
  %t425 = load i64, ptr %t419
  call void @cinch.rc.drop.tobj(i64 %t425)
  %t426 = load i64, ptr %t423
  %t427 = call i64 @"cinch.prim.eqI64"(i64 %t426, i64 0), !range !162
  %t428 = icmp ne i64 %t427, 0
  br i1 %t428, label %bb38, label %bb39
bb38:
  ret i64 1
bb39:
  ret i64 3
bb27:
  ret i64 0
bb7:
  %t429 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [10 x i8] }, ptr @"Main.measure.dec5", i64 0, i32 1) to i64
  store i64 %t429, ptr %t430
  %t431 = load i64, ptr %t1
  %t432 = load i64, ptr %t430
  %t433 = call i64 @"Data.String.Eq_String.=="(i64 %t431, i64 %t432), !range !163
  store i64 %t433, ptr %t434
  %t435 = load i64, ptr %t430
  call void @cinch.rc.drop.obj(i64 %t435)
  %t436 = load i64, ptr %t434
  %t437 = icmp ne i64 %t436, 0
  br i1 %t437, label %bb40, label %bb41
bb40:
  %t438 = call i64 @"Effect.Clock.raw-monotonic-ns"(i64 0)
  store i64 %t438, ptr %t439
  %t440 = load i64, ptr %t439
  %t441 = call i64 @"cinch.runtime.natFromI64"(i64 %t440)
  store i64 %t441, ptr %t442
  %t443 = load i64, ptr %t2
  %t444 = load i64, ptr %t22
  %t445 = load i64, ptr %t24
  %t446 = trunc i64 %t444 to i32
  %t447 = trunc i64 0 to i32
  %t448 = call { i64, i64, i64 } @"Main.allocation-loop#cc"(i64 %t443, i32 %t446, i32 %t447, i64 0, i64 -3750763034362895579, i64 %t445)
  %t449 = extractvalue { i64, i64, i64 } %t448, 0
  %t450 = extractvalue { i64, i64, i64 } %t448, 1
  %t451 = extractvalue { i64, i64, i64 } %t448, 2
  %t452 = call i64 @"Main.ByteRun"(i64 %t449, i64 %t450, i64 %t451)
  store i64 %t452, ptr %t453
  %t454 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t454)
  %t455 = load i64, ptr %t0
  %t456 = load i64, ptr %t442
  %t457 = load i64, ptr %t3
  %t458 = load i64, ptr %t453
  %t459 = musttail call i64 @"Main.finish"(i64 %t455, i64 %t456, i64 %t457, i64 %t458)
  ret i64 %t459
bb41:
  %t460 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t460)
  %t461 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [7 x i8] }, ptr @"Main.measure.dec6", i64 0, i32 1) to i64
  store i64 %t461, ptr %t462
  %t463 = load i64, ptr %t1
  %t464 = load i64, ptr %t462
  %t465 = call i64 @"Data.String.Eq_String.=="(i64 %t463, i64 %t464), !range !164
  store i64 %t465, ptr %t466
  %t467 = load i64, ptr %t462
  call void @cinch.rc.drop.obj(i64 %t467)
  %t468 = load i64, ptr %t466
  %t469 = icmp ne i64 %t468, 0
  br i1 %t469, label %bb42, label %bb43
bb42:
  %t470 = load i64, ptr %t0
  %t471 = call i64 @"Effect.Clock.clock-monotonic-ns"(i64 %t470, i64 0)
  store i64 %t471, ptr %t472
  %t473 = load i64, ptr %t22
  %t474 = load i64, ptr %t24
  %t475 = trunc i64 %t473 to i32
  %t476 = trunc i64 0 to i32
  %t477 = call { i64, i64, i64 } @"Main.counter-loop#cc"(i32 %t475, i32 %t476, i64 0, i64 -3750763034362895579, i64 %t474)
  %t478 = extractvalue { i64, i64, i64 } %t477, 0
  %t479 = extractvalue { i64, i64, i64 } %t477, 1
  %t480 = extractvalue { i64, i64, i64 } %t477, 2
  %t481 = call i64 @"Main.ByteRun"(i64 %t478, i64 %t479, i64 %t480)
  store i64 %t481, ptr %t482
  %t483 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t483)
  %t484 = load i64, ptr %t0
  %t485 = load i64, ptr %t472
  %t486 = load i64, ptr %t3
  %t487 = load i64, ptr %t482
  %t488 = musttail call i64 @"Main.finish"(i64 %t484, i64 %t485, i64 %t486, i64 %t487)
  ret i64 %t488
bb43:
  %t489 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [5 x i8] }, ptr @"Main.measure.dec7", i64 0, i32 1) to i64
  store i64 %t489, ptr %t490
  %t491 = load i64, ptr %t1
  %t492 = load i64, ptr %t490
  %t493 = call i64 @"Data.String.Eq_String.=="(i64 %t491, i64 %t492), !range !165
  store i64 %t493, ptr %t494
  %t495 = load i64, ptr %t490
  call void @cinch.rc.drop.obj(i64 %t495)
  %t496 = load i64, ptr %t494
  %t497 = icmp ne i64 %t496, 0
  br i1 %t497, label %bb44, label %bb45
bb44:
  %t498 = load i64, ptr %t0
  %t499 = call i64 @"Effect.Clock.clock-monotonic-ns"(i64 %t498, i64 0)
  store i64 %t499, ptr %t500
  %t501 = load i64, ptr %t0
  %t502 = load i64, ptr %t22
  %t503 = load i64, ptr %t24
  %t504 = trunc i64 %t502 to i32
  %t505 = trunc i64 0 to i32
  %t506 = call { i64, i64, i64 } @"Main.clock-loop#cc"(i64 %t501, i32 %t504, i32 %t505, i64 0, i64 -3750763034362895579, i64 %t503)
  %t507 = extractvalue { i64, i64, i64 } %t506, 0
  %t508 = extractvalue { i64, i64, i64 } %t506, 1
  %t509 = extractvalue { i64, i64, i64 } %t506, 2
  %t510 = call i64 @"Main.ByteRun"(i64 %t507, i64 %t508, i64 %t509)
  store i64 %t510, ptr %t511
  %t512 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t512)
  %t513 = load i64, ptr %t0
  %t514 = load i64, ptr %t500
  %t515 = load i64, ptr %t3
  %t516 = load i64, ptr %t511
  %t517 = musttail call i64 @"Main.finish"(i64 %t513, i64 %t514, i64 %t515, i64 %t516)
  ret i64 %t517
bb45:
  %t518 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [2 x i8] }, ptr @"Main.measure.dec8", i64 0, i32 1) to i64
  store i64 %t518, ptr %t519
  %t520 = load i64, ptr %t1
  %t521 = load i64, ptr %t519
  %t522 = call i64 @"Data.String.Eq_String.=="(i64 %t520, i64 %t521), !range !166
  store i64 %t522, ptr %t523
  %t524 = load i64, ptr %t519
  call void @cinch.rc.drop.obj(i64 %t524)
  %t525 = load i64, ptr %t523
  %t526 = icmp ne i64 %t525, 0
  br i1 %t526, label %bb46, label %bb47
bb46:
  %t527 = load i64, ptr %t0
  %t528 = call i64 @"Effect.Clock.clock-monotonic-ns"(i64 %t527, i64 0)
  store i64 %t528, ptr %t529
  %t530 = load i64, ptr %t0
  %t531 = load i64, ptr %t22
  %t532 = load i64, ptr %t24
  %t533 = trunc i64 %t531 to i32
  %t534 = trunc i64 0 to i32
  %t535 = call { i64, i64, i64 } @"Main.v4-loop#cc"(i64 %t530, i32 %t533, i32 %t534, i64 0, i64 -3750763034362895579, i64 %t532)
  %t536 = extractvalue { i64, i64, i64 } %t535, 0
  %t537 = extractvalue { i64, i64, i64 } %t535, 1
  %t538 = extractvalue { i64, i64, i64 } %t535, 2
  %t539 = call i64 @"Main.ByteRun"(i64 %t536, i64 %t537, i64 %t538)
  store i64 %t539, ptr %t540
  %t541 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t541)
  %t542 = load i64, ptr %t0
  %t543 = load i64, ptr %t529
  %t544 = load i64, ptr %t3
  %t545 = load i64, ptr %t540
  %t546 = musttail call i64 @"Main.finish"(i64 %t542, i64 %t543, i64 %t544, i64 %t545)
  ret i64 %t546
bb47:
  %t547 = ptrtoint ptr getelementptr inbounds ({ i64, ptr, i64, i64, ptr, [2 x i8] }, ptr @"Main.measure.dec9", i64 0, i32 1) to i64
  store i64 %t547, ptr %t548
  %t549 = load i64, ptr %t1
  %t550 = load i64, ptr %t548
  %t551 = call i64 @"Data.String.Eq_String.=="(i64 %t549, i64 %t550), !range !167
  store i64 %t551, ptr %t552
  %t553 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t553)
  %t554 = load i64, ptr %t548
  call void @cinch.rc.drop.obj(i64 %t554)
  %t555 = load i64, ptr %t552
  %t556 = icmp ne i64 %t555, 0
  br i1 %t556, label %bb48, label %bb49
bb48:
  %t557 = call { i64 } @"Effect.Uuid.uuid-v7-generator#cc"(i64 0)
  %t558 = extractvalue { i64 } %t557, 0
  %t559 = call i64 @"Effect.Uuid.Generator"(i64 %t558)
  store i64 %t559, ptr %t560
  %t561 = load i64, ptr %t0
  %t562 = call i64 @"Effect.Clock.clock-monotonic-ns"(i64 %t561, i64 0)
  store i64 %t562, ptr %t563
  %t564 = load i64, ptr %t0
  %t565 = load i64, ptr %t560
  %t566 = load i64, ptr %t22
  %t567 = load i64, ptr %t24
  %t568 = trunc i64 %t566 to i32
  %t569 = trunc i64 0 to i32
  %t570 = call tailcc { i64, i64, i64 } @"Main.v7-loop#cc"(i64 %t564, i64 %t565, i32 %t568, i32 %t569, i64 0, i64 -3750763034362895579, i64 %t567)
  %t571 = extractvalue { i64, i64, i64 } %t570, 0
  %t572 = extractvalue { i64, i64, i64 } %t570, 1
  %t573 = extractvalue { i64, i64, i64 } %t570, 2
  %t574 = call i64 @"Main.ByteRun"(i64 %t571, i64 %t572, i64 %t573)
  store i64 %t574, ptr %t575
  %t576 = load i64, ptr %t0
  %t577 = load i64, ptr %t563
  %t578 = load i64, ptr %t3
  %t579 = load i64, ptr %t575
  %t580 = musttail call i64 @"Main.finish"(i64 %t576, i64 %t577, i64 %t578, i64 %t579)
  ret i64 %t580
bb49:
  %t581 = load i64, ptr %t3
  call void @cinch.rc.drop.tobj(i64 %t581)
  %t582 = load i64, ptr %t24
  call void @cinch.rc.drop.obj(i64 %t582)
  ret i64 5
bb5:
  %t583 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t583)
  %t584 = load i64, ptr %t2
  call void @cinch.rc.drop.tobj(i64 %t584)
  %t585 = load i64, ptr %t3
  call void @cinch.rc.drop.tobj(i64 %t585)
  ret i64 5
}
