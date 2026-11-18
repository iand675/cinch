define tailcc { i64, i64, i64 } @"Main.v7-loop#cc"(i64 %p0, i64 %p1, i32 noundef %p2, i32 noundef %p3, i64 noundef %p4, i64 noundef %p5, i64 %p6) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t2 = alloca i64
  %t4 = alloca i64
  %t6 = alloca i64
  %t7 = alloca i64
  %t8 = alloca i64
  %t13 = alloca i64
  %t28 = alloca i64
  %t30 = alloca i64
  %t45 = alloca i64
  %t49 = alloca i64
  %t53 = alloca i64
  %t55 = alloca i64
  %t58 = alloca i64
  %t59 = alloca i64
  %t74 = alloca i64
  %t77 = alloca i64
  %t83 = alloca i64
  %t84 = alloca i64
  %t88 = alloca i64
  %t93 = alloca i64
  %t98 = alloca i64
  %t101 = alloca i64
  %t104 = alloca i64
  %t107 = alloca i64
  %t110 = alloca i64
  %t114 = alloca i64
  %t117 = alloca i64
  %t120 = alloca i64
  %t124 = alloca i64
  %t128 = alloca i64
  %t129 = alloca i64
  %t133 = alloca i64
  %t147 = alloca i64
  %t154 = alloca i64
  %t155 = alloca i64
  %t159 = alloca i64
  %t170 = alloca i64
  %t187 = alloca i64
  %t202 = alloca i64
  %t203 = alloca i64
  %t207 = alloca i64
  %t222 = alloca i64
  %t229 = alloca i64
  %t233 = alloca i64
  %t247 = alloca i64
  %t266 = alloca i64
  %t269 = alloca i64
  %t273 = alloca i64
  %t288 = alloca i64
  %t295 = alloca i64
  %t299 = alloca i64
  %t301 = alloca i64
  %t307 = alloca i64
  %t310 = alloca i64
  %t322 = alloca i64
  %t327 = alloca i64
  %t330 = alloca i64
  %t333 = alloca i64
  %t339 = alloca i64
  %t346 = alloca i64
  %t355 = alloca i64
  %t370 = alloca i64
  %t373 = alloca i64
  %t376 = alloca i64
  %t388 = alloca i64
  %t392 = alloca i64
  %t407 = alloca i64
  %t410 = alloca i64
  %t414 = alloca i64
  %t419 = alloca i64
  %t424 = alloca i64
  %t427 = alloca i64
  %t428 = alloca i64
  %t431 = alloca i64
  %t434 = alloca i64
  %t443 = alloca i64
  %t446 = alloca i64
  %t453 = alloca i64
  %t456 = alloca i64
  %t461 = alloca i64
  %t464 = alloca i64
  %t469 = alloca i64
  %t474 = alloca i64
  store i64 %p0, ptr %t0
  store i64 %p1, ptr %t1
  %t3 = zext i32 %p2 to i64
  store i64 %t3, ptr %t2
  %t5 = zext i32 %p3 to i64
  store i64 %t5, ptr %t4
  store i64 %p4, ptr %t6
  store i64 %p5, ptr %t7
  store i64 %p6, ptr %t8
  br label %bb0
bb0:
  %t9 = load i64, ptr %t4
  %t10 = load i64, ptr %t2
  %t11 = call i64 @"Main.before"(i64 %t9, i64 %t10)
  %t12 = icmp ne i64 %t11, 0
  br i1 %t12, label %bb1, label %bb2
bb1:
  %t14 = load i64, ptr %t1
  %t15 = inttoptr i64 %t14 to ptr
  %t16 = getelementptr inbounds i8, ptr %t15, i64 -8
  %t17 = load atomic i64, ptr %t16 monotonic, align 8
  %t18 = lshr i64 %t17, 40
  %t19 = and i64 %t18, 255
  %t20 = icmp eq i64 %t19, 255
  br i1 %t20, label %bb5, label %bb6
bb5:
  %t21 = getelementptr inbounds i8, ptr %t15, i64 -16
  %t22 = load i64, ptr %t21
  %t23 = inttoptr i64 %t22 to ptr
  %t24 = load i64, ptr %t23
  br label %bb7
bb6:
  br label %bb7
bb7:
  %t25 = phi i64 [ %t24, %bb5 ], [ %t19, %bb6 ]
  switch i64 %t25, label %bb4 [
  i64 0, label %bb8
]
bb8:
  %t26 = getelementptr inbounds i8, ptr %t15, i64 0
  %t27 = load i64, ptr %t26
  store i64 %t27, ptr %t28
  %t29 = load i64, ptr %t28
  call void @cinch.rc.dup.obj(i64 %t29)
  %t31 = load i64, ptr %t28
  %t32 = inttoptr i64 %t31 to ptr
  %t33 = getelementptr inbounds i8, ptr %t32, i64 -8
  %t34 = load atomic i64, ptr %t33 monotonic, align 8
  %t35 = lshr i64 %t34, 40
  %t36 = and i64 %t35, 255
  %t37 = icmp eq i64 %t36, 255
  br i1 %t37, label %bb11, label %bb12
bb11:
  %t38 = getelementptr inbounds i8, ptr %t32, i64 -16
  %t39 = load i64, ptr %t38
  %t40 = inttoptr i64 %t39 to ptr
  %t41 = load i64, ptr %t40
  br label %bb13
bb12:
  br label %bb13
bb13:
  %t42 = phi i64 [ %t41, %bb11 ], [ %t36, %bb12 ]
  switch i64 %t42, label %bb10 [
  i64 0, label %bb14
]
bb14:
  %t43 = getelementptr inbounds i8, ptr %t32, i64 0
  %t44 = load i64, ptr %t43
  store i64 %t44, ptr %t45
  %t46 = load i64, ptr %t45
  call void @cinch.rc.dup.obj(i64 %t46)
  %t47 = load i64, ptr %t45
  %t48 = call i64 @"Effect.Concurrent.MVar.cell-take@O"(i64 %t47)
  store i64 %t48, ptr %t49
  %t50 = load i64, ptr %t45
  call void @cinch.rc.drop.obj(i64 %t50)
  %t51 = load i64, ptr %t49
  store i64 %t51, ptr %t30
  br label %bb9
bb10:
  ret { i64, i64, i64 } zeroinitializer
bb9:
  %t52 = load i64, ptr %t30
  store i64 %t52, ptr %t53
  %t54 = call i64 @"Effect.Clock.raw-wall-ns"(i64 0)
  store i64 %t54, ptr %t55
  %t56 = load i64, ptr %t55
  %t57 = call i64 @"cinch.runtime.natFromI64"(i64 %t56)
  store i64 %t57, ptr %t58
  %t60 = load i64, ptr %t53
  %t61 = inttoptr i64 %t60 to ptr
  %t62 = getelementptr inbounds i8, ptr %t61, i64 -8
  %t63 = load atomic i64, ptr %t62 monotonic, align 8
  %t64 = lshr i64 %t63, 40
  %t65 = and i64 %t64, 255
  %t66 = icmp eq i64 %t65, 255
  br i1 %t66, label %bb17, label %bb18
bb17:
  %t67 = getelementptr inbounds i8, ptr %t61, i64 -16
  %t68 = load i64, ptr %t67
  %t69 = inttoptr i64 %t68 to ptr
  %t70 = load i64, ptr %t69
  br label %bb19
bb18:
  br label %bb19
bb19:
  %t71 = phi i64 [ %t70, %bb17 ], [ %t65, %bb18 ]
  switch i64 %t71, label %bb16 [
  i64 0, label %bb20
]
bb20:
  %t72 = getelementptr inbounds i8, ptr %t61, i64 0
  %t73 = load i64, ptr %t72
  store i64 %t73, ptr %t74
  %t75 = getelementptr inbounds i8, ptr %t61, i64 8
  %t76 = load i64, ptr %t75
  store i64 %t76, ptr %t77
  %t78 = load i64, ptr %t74
  call void @cinch.rc.dup.tobj(i64 %t78)
  %t79 = load i64, ptr %t77
  call void @cinch.rc.dup.tobj(i64 %t79)
  %t80 = load i64, ptr %t58
  call void @cinch.rc.dup.tobj(i64 %t80)
  %t81 = load i64, ptr %t58
  %t82 = call i64 @"cinch.runtime.natDiv"(i64 %t81, i64 2000001)
  store i64 %t82, ptr %t83
  store i64 562949953421313, ptr %t84
  %t85 = load i64, ptr %t83
  %t86 = load i64, ptr %t84
  %t87 = call i64 @"cinch.runtime.natCmp"(i64 %t85, i64 %t86)
  store i64 %t87, ptr %t88
  %t89 = load i64, ptr %t84
  call void @cinch.rc.drop.tobj(i64 %t89)
  %t90 = load i64, ptr %t88
  %t91 = call i64 @"cinch.prim.ltI64"(i64 %t90, i64 0), !range !123
  %t92 = icmp ne i64 %t91, 0
  br i1 %t92, label %bb21, label %bb22
bb21:
  store i64 2000000001, ptr %t93
  %t94 = load i64, ptr %t58
  call void @cinch.rc.dup.tobj(i64 %t94)
  %t95 = load i64, ptr %t58
  %t96 = load i64, ptr %t93
  %t97 = call i64 @"cinch.runtime.natDiv"(i64 %t95, i64 %t96)
  store i64 %t97, ptr %t98
  %t99 = load i64, ptr %t93
  call void @cinch.rc.drop.tobj(i64 %t99)
  %t100 = load i64, ptr %t98
  store i64 %t100, ptr %t101
  %t102 = load i64, ptr %t58
  %t103 = call i64 @"cinch.runtime.natRem"(i64 %t102, i64 2000001)
  store i64 %t103, ptr %t104
  %t105 = load i64, ptr %t58
  call void @cinch.rc.drop.tobj(i64 %t105)
  %t106 = load i64, ptr %t104
  store i64 %t106, ptr %t107
  %t108 = load i64, ptr %t107
  %t109 = call i64 @"cinch.runtime.natMul"(i64 %t108, i64 8193)
  store i64 %t109, ptr %t110
  %t111 = load i64, ptr %t107
  call void @cinch.rc.drop.tobj(i64 %t111)
  %t112 = load i64, ptr %t110
  %t113 = call i64 @"cinch.runtime.natDiv"(i64 %t112, i64 2000001)
  store i64 %t113, ptr %t114
  %t115 = load i64, ptr %t83
  %t116 = call i64 @"cinch.runtime.natMul"(i64 %t115, i64 8193)
  store i64 %t116, ptr %t117
  %t118 = load i64, ptr %t83
  call void @cinch.rc.drop.tobj(i64 %t118)
  %t119 = load i64, ptr %t117
  store i64 %t119, ptr %t120
  %t121 = load i64, ptr %t120
  %t122 = load i64, ptr %t114
  %t123 = call i64 @"cinch.runtime.natAdd"(i64 %t121, i64 %t122)
  store i64 %t123, ptr %t124
  %t125 = load i64, ptr %t114
  call void @cinch.rc.drop.tobj(i64 %t125)
  %t126 = load i64, ptr %t120
  call void @cinch.rc.drop.tobj(i64 %t126)
  %t127 = load i64, ptr %t124
  store i64 %t127, ptr %t128
  %t130 = load i64, ptr %t74
  %t131 = load i64, ptr %t101
  %t132 = call i64 @"cinch.runtime.natCmp"(i64 %t130, i64 %t131)
  store i64 %t132, ptr %t133
  %t134 = load i64, ptr %t74
  call void @cinch.rc.drop.tobj(i64 %t134)
  %t135 = load i64, ptr %t133
  %t136 = call i64 @"cinch.prim.gtI64"(i64 %t135, i64 0), !range !124
  %t137 = icmp ne i64 %t136, 0
  br i1 %t137, label %bb24, label %bb25
bb24:
  %t138 = load i64, ptr %t77
  call void @cinch.rc.drop.tobj(i64 %t138)
  %t139 = load i64, ptr %t128
  store i64 %t139, ptr %t129
  br label %bb23
bb25:
  %t140 = load i64, ptr %t128
  %t141 = load i64, ptr %t77
  %t142 = call i64 @"cinch.runtime.natCmp"(i64 %t140, i64 %t141)
  %t143 = call i64 @"cinch.prim.leI64"(i64 %t142, i64 0), !range !125
  %t144 = icmp ne i64 %t143, 0
  br i1 %t144, label %bb26, label %bb27
bb26:
  %t145 = load i64, ptr %t77
  %t146 = call i64 @"cinch.runtime.natAdd"(i64 %t145, i64 3)
  store i64 %t146, ptr %t147
  %t148 = load i64, ptr %t128
  call void @cinch.rc.drop.tobj(i64 %t148)
  %t149 = load i64, ptr %t77
  call void @cinch.rc.drop.tobj(i64 %t149)
  %t150 = load i64, ptr %t147
  store i64 %t150, ptr %t129
  br label %bb23
bb27:
  %t151 = load i64, ptr %t77
  call void @cinch.rc.drop.tobj(i64 %t151)
  %t152 = load i64, ptr %t128
  store i64 %t152, ptr %t129
  br label %bb23
bb23:
  %t153 = load i64, ptr %t129
  store i64 %t153, ptr %t154
  store i64 2305843009213693951, ptr %t155
  %t156 = load i64, ptr %t154
  %t157 = load i64, ptr %t155
  %t158 = call i64 @"cinch.runtime.natCmp"(i64 %t156, i64 %t157)
  store i64 %t158, ptr %t159
  %t160 = load i64, ptr %t155
  call void @cinch.rc.drop.tobj(i64 %t160)
  %t161 = load i64, ptr %t159
  %t162 = call i64 @"cinch.prim.leI64"(i64 %t161, i64 0), !range !126
  %t163 = icmp ne i64 %t162, 0
  br i1 %t163, label %bb28, label %bb29
bb28:
  %t164 = load i64, ptr %t101
  %t165 = load i64, ptr %t154
  call void @cinch.rc.dup.tobj(i64 %t165)
  %t166 = load i64, ptr %t154
  %t167 = call i64 @"Data.Uuid.V7Clock"(i64 %t164, i64 %t166)
  %t168 = load i64, ptr %t154
  %t169 = call i64 @"cinch.runtime.natToI64"(i64 %t168)
  store i64 %t169, ptr %t170
  %t171 = load i64, ptr %t154
  call void @cinch.rc.drop.tobj(i64 %t171)
  %t172 = load i64, ptr %t170
  %t173 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t172)
  %t174 = call i64 @"Tuple.2@OR"(i64 %t167, i64 %t173)
  %t175 = call i64 @"Data.Core.Ok@OO"(i64 %t174)
  store i64 %t175, ptr %t59
  br label %bb15
bb29:
  %t176 = load i64, ptr %t101
  call void @cinch.rc.drop.tobj(i64 %t176)
  %t177 = load i64, ptr %t154
  call void @cinch.rc.drop.tobj(i64 %t177)
  %t178 = ptrtoint ptr getelementptr inbounds (i8, ptr @"Data.Uuid.UuidClockExhausted.blk#h", i64 8) to i64
  %t179 = call i64 @"Data.Core.Err@OO"(i64 %t178)
  store i64 %t179, ptr %t59
  br label %bb15
bb22:
  %t180 = load i64, ptr %t58
  call void @cinch.rc.drop.tobj(i64 %t180)
  %t181 = load i64, ptr %t74
  call void @cinch.rc.drop.tobj(i64 %t181)
  %t182 = load i64, ptr %t77
  call void @cinch.rc.drop.tobj(i64 %t182)
  %t183 = load i64, ptr %t83
  call void @cinch.rc.drop.tobj(i64 %t183)
  %t184 = ptrtoint ptr getelementptr inbounds (i8, ptr @"Data.Uuid.UuidClockOutOfRange.blk#h", i64 8) to i64
  %t185 = call i64 @"Data.Core.Err@OO"(i64 %t184)
  store i64 %t185, ptr %t59
  br label %bb15
bb16:
  ret { i64, i64, i64 } zeroinitializer
bb15:
  %t186 = load i64, ptr %t59
  store i64 %t186, ptr %t187
  %t188 = load i64, ptr %t187
  %t189 = inttoptr i64 %t188 to ptr
  %t190 = getelementptr inbounds i8, ptr %t189, i64 -8
  %t191 = load atomic i64, ptr %t190 monotonic, align 8
  %t192 = lshr i64 %t191, 40
  %t193 = and i64 %t192, 255
  %t194 = icmp eq i64 %t193, 255
  br i1 %t194, label %bb31, label %bb32
bb31:
  %t195 = getelementptr inbounds i8, ptr %t189, i64 -16
  %t196 = load i64, ptr %t195
  %t197 = inttoptr i64 %t196 to ptr
  %t198 = load i64, ptr %t197
  br label %bb33
bb32:
  br label %bb33
bb33:
  %t199 = phi i64 [ %t198, %bb31 ], [ %t193, %bb32 ]
  switch i64 %t199, label %bb30 [
  i64 0, label %bb34
  i64 1, label %bb35
]
bb34:
  %t200 = getelementptr inbounds i8, ptr %t189, i64 0
  %t201 = load i64, ptr %t200
  store i64 %t201, ptr %t202
  %t204 = load i64, ptr %t187
  %t205 = call i1 @cinch.rc.unique(i64 %t204)
  br i1 %t205, label %bb36, label %bb37
bb36:
  store i64 %t204, ptr %t203
  br label %bb38
bb37:
  %t206 = load i64, ptr %t202
  call void @cinch.rc.dup.obj(i64 %t206)
  call void @cinch.rc.drop.obj(i64 %t204)
  store i64 0, ptr %t203
  br label %bb38
bb38:
  %t208 = load i64, ptr %t28
  %t209 = inttoptr i64 %t208 to ptr
  %t210 = getelementptr inbounds i8, ptr %t209, i64 -8
  %t211 = load atomic i64, ptr %t210 monotonic, align 8
  %t212 = lshr i64 %t211, 40
  %t213 = and i64 %t212, 255
  %t214 = icmp eq i64 %t213, 255
  br i1 %t214, label %bb41, label %bb42
bb41:
  %t215 = getelementptr inbounds i8, ptr %t209, i64 -16
  %t216 = load i64, ptr %t215
  %t217 = inttoptr i64 %t216 to ptr
  %t218 = load i64, ptr %t217
  br label %bb43
bb42:
  br label %bb43
bb43:
  %t219 = phi i64 [ %t218, %bb41 ], [ %t213, %bb42 ]
  switch i64 %t219, label %bb40 [
  i64 0, label %bb44
]
bb44:
  %t220 = getelementptr inbounds i8, ptr %t209, i64 0
  %t221 = load i64, ptr %t220
  store i64 %t221, ptr %t222
  %t223 = load i64, ptr %t28
  %t224 = call i1 @cinch.rc.unique(i64 %t223)
  br i1 %t224, label %bb45, label %bb46
bb45:
  call void @cinch_free(i64 %t223)
  br label %bb47
bb46:
  %t225 = load i64, ptr %t222
  call void @cinch.rc.dup.obj(i64 %t225)
  call void @cinch.rc.drop.obj(i64 %t223)
  br label %bb47
bb47:
  %t226 = load i64, ptr %t222
  %t227 = load i64, ptr %t53
  %t228 = call i64 @"Effect.Concurrent.MVar.cell-put@O"(i64 %t226, i64 %t227)
  store i64 %t228, ptr %t229
  %t230 = load i64, ptr %t222
  call void @cinch.rc.drop.obj(i64 %t230)
  %t231 = load i64, ptr %t229
  store i64 %t231, ptr %t207
  br label %bb39
bb40:
  ret { i64, i64, i64 } zeroinitializer
bb39:
  %t232 = load i64, ptr %t207
  store i64 %t232, ptr %t233
  %t234 = load i64, ptr %t202
  %t235 = load i64, ptr %t203
  %t236 = icmp ne i64 %t235, 0
  br i1 %t236, label %bb49, label %bb48
bb48:
  %t237 = call ptr @cinch_alloc_ctor(i64 8, i64 256)
  %t238 = getelementptr inbounds i8, ptr %t237, i64 0
  store i64 %t234, ptr %t238
  br label %bb50
bb49:
  store i64 0, ptr %t203
  %t239 = inttoptr i64 %t235 to ptr
  %t240 = getelementptr inbounds i8, ptr %t239, i64 -8
  store i64 281509336449025, ptr %t240
  br label %bb50
bb50:
  %t241 = phi ptr [ %t237, %bb48 ], [ %t239, %bb49 ]
  %t242 = ptrtoint ptr %t241 to i64
  %t243 = load i64, ptr %t203
  %t244 = icmp ne i64 %t243, 0
  br i1 %t244, label %bb51, label %bb52
bb51:
  store i64 0, ptr %t203
  call void @cinch_free(i64 %t243)
  br label %bb52
bb52:
  store i64 %t242, ptr %t13
  br label %bb3
bb35:
  %t245 = getelementptr inbounds i8, ptr %t189, i64 0
  %t246 = load i64, ptr %t245
  store i64 %t246, ptr %t247
  %t248 = load i64, ptr %t187
  %t249 = call i1 @cinch.rc.unique(i64 %t248)
  br i1 %t249, label %bb53, label %bb54
bb53:
  call void @cinch_free(i64 %t248)
  br label %bb55
bb54:
  %t250 = load i64, ptr %t247
  call void @cinch.rc.dup.obj(i64 %t250)
  call void @cinch.rc.drop.obj(i64 %t248)
  br label %bb55
bb55:
  %t251 = load i64, ptr %t53
  call void @cinch.rc.drop.obj(i64 %t251)
  %t252 = load i64, ptr %t247
  %t253 = inttoptr i64 %t252 to ptr
  %t254 = getelementptr inbounds i8, ptr %t253, i64 -8
  %t255 = load atomic i64, ptr %t254 monotonic, align 8
  %t256 = lshr i64 %t255, 40
  %t257 = and i64 %t256, 255
  %t258 = icmp eq i64 %t257, 255
  br i1 %t258, label %bb57, label %bb58
bb57:
  %t259 = getelementptr inbounds i8, ptr %t253, i64 -16
  %t260 = load i64, ptr %t259
  %t261 = inttoptr i64 %t260 to ptr
  %t262 = load i64, ptr %t261
  br label %bb59
bb58:
  br label %bb59
bb59:
  %t263 = phi i64 [ %t262, %bb57 ], [ %t257, %bb58 ]
  switch i64 %t263, label %bb56 [
  i64 0, label %bb60
]
bb60:
  %t264 = getelementptr inbounds i8, ptr %t253, i64 0
  %t265 = load i64, ptr %t264
  store i64 %t265, ptr %t266
  %t267 = getelementptr inbounds i8, ptr %t253, i64 8
  %t268 = load i64, ptr %t267
  store i64 %t268, ptr %t269
  %t270 = load i64, ptr %t247
  %t271 = call i1 @cinch.rc.unique(i64 %t270)
  br i1 %t271, label %bb61, label %bb62
bb61:
  call void @cinch_free(i64 %t270)
  br label %bb63
bb62:
  %t272 = load i64, ptr %t266
  call void @cinch.rc.dup.obj(i64 %t272)
  call void @cinch.rc.drop.obj(i64 %t270)
  br label %bb63
bb63:
  %t274 = load i64, ptr %t28
  %t275 = inttoptr i64 %t274 to ptr
  %t276 = getelementptr inbounds i8, ptr %t275, i64 -8
  %t277 = load atomic i64, ptr %t276 monotonic, align 8
  %t278 = lshr i64 %t277, 40
  %t279 = and i64 %t278, 255
  %t280 = icmp eq i64 %t279, 255
  br i1 %t280, label %bb66, label %bb67
bb66:
  %t281 = getelementptr inbounds i8, ptr %t275, i64 -16
  %t282 = load i64, ptr %t281
  %t283 = inttoptr i64 %t282 to ptr
  %t284 = load i64, ptr %t283
  br label %bb68
bb67:
  br label %bb68
bb68:
  %t285 = phi i64 [ %t284, %bb66 ], [ %t279, %bb67 ]
  switch i64 %t285, label %bb65 [
  i64 0, label %bb69
]
bb69:
  %t286 = getelementptr inbounds i8, ptr %t275, i64 0
  %t287 = load i64, ptr %t286
  store i64 %t287, ptr %t288
  %t289 = load i64, ptr %t28
  %t290 = call i1 @cinch.rc.unique(i64 %t289)
  br i1 %t290, label %bb70, label %bb71
bb70:
  call void @cinch_free(i64 %t289)
  br label %bb72
bb71:
  %t291 = load i64, ptr %t288
  call void @cinch.rc.dup.obj(i64 %t291)
  call void @cinch.rc.drop.obj(i64 %t289)
  br label %bb72
bb72:
  %t292 = load i64, ptr %t288
  %t293 = load i64, ptr %t266
  %t294 = call i64 @"Effect.Concurrent.MVar.cell-put@O"(i64 %t292, i64 %t293)
  store i64 %t294, ptr %t295
  %t296 = load i64, ptr %t288
  call void @cinch.rc.drop.obj(i64 %t296)
  %t297 = load i64, ptr %t295
  store i64 %t297, ptr %t273
  br label %bb64
bb65:
  ret { i64, i64, i64 } zeroinitializer
bb64:
  %t298 = load i64, ptr %t273
  store i64 %t298, ptr %t299
  %t300 = call i64 @"cinch.runtime.randomBytes"(i64 17)
  store i64 %t300, ptr %t301
  %t302 = load i64, ptr %t269
  %t303 = call i64 @"cinch.prim.leU64"(i64 %t302, i64 1152921504606846975), !range !127
  %t304 = icmp ne i64 %t303, 0
  br i1 %t304, label %bb73, label %bb74
bb73:
  %t305 = load i64, ptr %t301
  %t306 = call i64 @"cinch.runtime.bytesLen"(i64 %t305)
  store i64 %t306, ptr %t307
  %t308 = load i64, ptr %t307
  %t309 = call i64 @"cinch.runtime.natCmp"(i64 %t308, i64 17)
  store i64 %t309, ptr %t310
  %t311 = load i64, ptr %t307
  call void @cinch.rc.drop.tobj(i64 %t311)
  %t312 = load i64, ptr %t310
  %t313 = call i64 @"cinch.prim.eqI64"(i64 %t312, i64 0), !range !128
  %t314 = icmp ne i64 %t313, 0
  br i1 %t314, label %bb75, label %bb76
bb75:
  %t315 = load i64, ptr %t269
  %t316 = call i64 @"cinch.prim.wrappingShlU64"(i64 %t315, i64 4)
  %t317 = call i64 @"cinch.prim.bitAndU64"(i64 %t316, i64 -65536)
  %t318 = load i64, ptr %t269
  %t319 = call i64 @"cinch.prim.bitAndU64"(i64 %t318, i64 4095)
  %t320 = call i64 @"cinch.prim.bitOrU64"(i64 %t317, i64 %t319)
  %t321 = call i64 @"cinch.prim.bitOrU64"(i64 %t320, i64 28672)
  store i64 %t321, ptr %t322
  %t323 = load i64, ptr %t301
  %t324 = call i64 @"cinch.runtime.bytesData"(i64 %t323)
  %t325 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t326 = call i64 @"Data.Bytes.Word.load-le64"(i64 %t324, i64 %t325)
  store i64 %t326, ptr %t327
  %t328 = load i64, ptr %t301
  %t329 = call i64 @"cinch.runtime.bytesLen"(i64 %t328)
  store i64 %t329, ptr %t330
  %t331 = load i64, ptr %t301
  call void @cinch.rc.drop.obj(i64 %t331)
  %t332 = load i64, ptr %t330
  store i64 %t332, ptr %t333
  %t334 = load i64, ptr %t333
  call void @cinch.rc.drop.tobj(i64 %t334)
  %t335 = load i64, ptr %t327
  %t336 = call i64 @"Data.Uuid.network-word"(i64 %t335)
  %t337 = call i64 @"cinch.prim.bitAndU64"(i64 %t336, i64 4611686018427387903)
  %t338 = call i64 @"cinch.prim.bitOrU64"(i64 %t337, i64 -9223372036854775808)
  store i64 %t338, ptr %t339
  %t340 = load i64, ptr %t322
  %t341 = load i64, ptr %t339
  %t342 = call i64 @"Data.Uuid.UuidBits"(i64 %t340, i64 %t341)
  %t343 = call i64 @"Data.Core.Ok@OO"(i64 %t342)
  store i64 %t343, ptr %t13
  br label %bb3
bb76:
  %t344 = load i64, ptr %t301
  %t345 = call i64 @"cinch.runtime.bytesLen"(i64 %t344)
  store i64 %t345, ptr %t346
  %t347 = load i64, ptr %t301
  call void @cinch.rc.drop.obj(i64 %t347)
  %t348 = load i64, ptr %t346
  %t349 = call i64 @"Data.Uuid.UuidInvalidLength"(i64 %t348)
  %t350 = call i64 @"Data.Core.Err@OO"(i64 %t349)
  store i64 %t350, ptr %t13
  br label %bb3
bb74:
  %t351 = load i64, ptr %t301
  call void @cinch.rc.drop.obj(i64 %t351)
  %t352 = ptrtoint ptr getelementptr inbounds (i8, ptr @"Data.Uuid.UuidClockOutOfRange.blk#h", i64 8) to i64
  %t353 = call i64 @"Data.Core.Err@OO"(i64 %t352)
  store i64 %t353, ptr %t13
  br label %bb3
bb56:
  ret { i64, i64, i64 } zeroinitializer
bb30:
  ret { i64, i64, i64 } zeroinitializer
bb4:
  ret { i64, i64, i64 } zeroinitializer
bb3:
  %t354 = load i64, ptr %t13
  store i64 %t354, ptr %t355
  %t356 = load i64, ptr %t355
  %t357 = inttoptr i64 %t356 to ptr
  %t358 = getelementptr inbounds i8, ptr %t357, i64 -8
  %t359 = load atomic i64, ptr %t358 monotonic, align 8
  %t360 = lshr i64 %t359, 40
  %t361 = and i64 %t360, 255
  %t362 = icmp eq i64 %t361, 255
  br i1 %t362, label %bb78, label %bb79
bb78:
  %t363 = getelementptr inbounds i8, ptr %t357, i64 -16
  %t364 = load i64, ptr %t363
  %t365 = inttoptr i64 %t364 to ptr
  %t366 = load i64, ptr %t365
  br label %bb80
bb79:
  br label %bb80
bb80:
  %t367 = phi i64 [ %t366, %bb78 ], [ %t361, %bb79 ]
  switch i64 %t367, label %bb77 [
  i64 0, label %bb81
  i64 1, label %bb82
]
bb81:
  %t368 = getelementptr inbounds i8, ptr %t357, i64 0
  %t369 = load i64, ptr %t368
  store i64 %t369, ptr %t370
  %t371 = load i64, ptr %t4
  %t372 = call i64 @"Main.bump"(i64 %t371)
  store i64 %t372, ptr %t373
  %t374 = load i64, ptr %t7
  %t375 = call i64 @"Main.mix"(i64 %t374, i64 4294967295)
  store i64 %t375, ptr %t376
  %t377 = load i64, ptr %t355
  call void @cinch.rc.drop.obj(i64 %t377)
  %t378 = load i64, ptr %t373
  %t379 = load i64, ptr %t2
  %t380 = load i64, ptr %t0
  %t381 = load i64, ptr %t1
  %t382 = load i64, ptr %t6
  %t383 = load i64, ptr %t376
  %t384 = load i64, ptr %t8
  %t385 = musttail call tailcc { i64, i64, i64 } @"Main.v7-loop.shared0#cc"(i64 %t378, i64 %t379, i64 %t380, i64 %t381, i64 %t382, i64 %t383, i64 %t384)
  ret { i64, i64, i64 } %t385
bb82:
  %t386 = getelementptr inbounds i8, ptr %t357, i64 0
  %t387 = load i64, ptr %t386
  store i64 %t387, ptr %t388
  %t389 = load i64, ptr %t355
  %t390 = call i1 @cinch.rc.unique(i64 %t389)
  br i1 %t390, label %bb83, label %bb84
bb83:
  call void @cinch_free(i64 %t389)
  br label %bb85
bb84:
  %t391 = load i64, ptr %t388
  call void @cinch.rc.dup.obj(i64 %t391)
  call void @cinch.rc.drop.obj(i64 %t389)
  br label %bb85
bb85:
  %t393 = load i64, ptr %t388
  %t394 = inttoptr i64 %t393 to ptr
  %t395 = getelementptr inbounds i8, ptr %t394, i64 -8
  %t396 = load atomic i64, ptr %t395 monotonic, align 8
  %t397 = lshr i64 %t396, 40
  %t398 = and i64 %t397, 255
  %t399 = icmp eq i64 %t398, 255
  br i1 %t399, label %bb88, label %bb89
bb88:
  %t400 = getelementptr inbounds i8, ptr %t394, i64 -16
  %t401 = load i64, ptr %t400
  %t402 = inttoptr i64 %t401 to ptr
  %t403 = load i64, ptr %t402
  br label %bb90
bb89:
  br label %bb90
bb90:
  %t404 = phi i64 [ %t403, %bb88 ], [ %t398, %bb89 ]
  switch i64 %t404, label %bb87 [
  i64 0, label %bb91
]
bb91:
  %t405 = getelementptr inbounds i8, ptr %t394, i64 0
  %t406 = load i64, ptr %t405
  store i64 %t406, ptr %t407
  %t408 = getelementptr inbounds i8, ptr %t394, i64 8
  %t409 = load i64, ptr %t408
  store i64 %t409, ptr %t410
  %t411 = load i64, ptr %t388
  %t412 = call i1 @cinch.rc.unique(i64 %t411)
  br i1 %t412, label %bb92, label %bb93
bb92:
  call void @cinch_free(i64 %t411)
  br label %bb94
bb93:
  call void @cinch.rc.drop.obj(i64 %t411)
  br label %bb94
bb94:
  %t413 = call i64 @"cinch.runtime.bytesNew"(i64 33)
  store i64 %t413, ptr %t414
  %t415 = load i64, ptr %t414
  %t416 = load i64, ptr %t407
  %t417 = call i64 @"Data.Uuid.network-word"(i64 %t416)
  %t418 = call i64 @"Data.Bytes.Word.bytes-write-le64"(i64 %t415, i64 1, i64 %t417)
  store i64 %t418, ptr %t419
  %t420 = load i64, ptr %t414
  %t421 = load i64, ptr %t410
  %t422 = call i64 @"Data.Uuid.network-word"(i64 %t421)
  %t423 = call i64 @"Data.Bytes.Word.bytes-write-le64"(i64 %t420, i64 17, i64 %t422)
  store i64 %t423, ptr %t424
  %t425 = load i64, ptr %t414
  store i64 %t425, ptr %t392
  br label %bb86
bb87:
  ret { i64, i64, i64 } zeroinitializer
bb86:
  %t426 = load i64, ptr %t392
  store i64 %t426, ptr %t427
  %t429 = load i64, ptr %t427
  %t430 = call i64 @"cinch.runtime.bytesLen"(i64 %t429)
  store i64 %t430, ptr %t431
  %t432 = load i64, ptr %t431
  %t433 = call i64 @"cinch.runtime.natCmp"(i64 %t432, i64 33)
  store i64 %t433, ptr %t434
  %t435 = load i64, ptr %t431
  call void @cinch.rc.drop.tobj(i64 %t435)
  %t436 = load i64, ptr %t434
  %t437 = call i64 @"cinch.prim.eqI64"(i64 %t436, i64 0), !range !129
  %t438 = icmp ne i64 %t437, 0
  br i1 %t438, label %bb96, label %bb97
bb96:
  %t439 = load i64, ptr %t427
  %t440 = call i64 @"cinch.runtime.bytesData"(i64 %t439)
  %t441 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t442 = call i64 @"Data.Bytes.Word.load-le64"(i64 %t440, i64 %t441)
  store i64 %t442, ptr %t443
  %t444 = load i64, ptr %t427
  %t445 = call i64 @"cinch.runtime.bytesLen"(i64 %t444)
  store i64 %t445, ptr %t446
  %t447 = load i64, ptr %t446
  call void @cinch.rc.drop.tobj(i64 %t447)
  %t448 = load i64, ptr %t443
  %t449 = load i64, ptr %t427
  %t450 = call i64 @"cinch.runtime.bytesData"(i64 %t449)
  %t451 = call i64 @"cinch.runtime.natToI64"(i64 17)
  %t452 = call i64 @"Data.Bytes.Word.load-le64"(i64 %t450, i64 %t451)
  store i64 %t452, ptr %t453
  %t454 = load i64, ptr %t427
  %t455 = call i64 @"cinch.runtime.bytesLen"(i64 %t454)
  store i64 %t455, ptr %t456
  %t457 = load i64, ptr %t456
  call void @cinch.rc.drop.tobj(i64 %t457)
  %t458 = load i64, ptr %t453
  %t459 = call i64 @"Main.observed-words"(i64 %t448, i64 %t458, i64 7)
  store i64 %t459, ptr %t428
  br label %bb95
bb97:
  store i64 0, ptr %t428
  br label %bb95
bb95:
  %t460 = load i64, ptr %t428
  store i64 %t460, ptr %t461
  %t462 = load i64, ptr %t4
  %t463 = call i64 @"Main.bump"(i64 %t462)
  store i64 %t463, ptr %t464
  %t465 = load i64, ptr %t6
  %t466 = load i64, ptr %t461
  %t467 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t466, i64 63)
  %t468 = call i64 @"Main.add"(i64 %t465, i64 %t467)
  store i64 %t468, ptr %t469
  %t470 = load i64, ptr %t7
  %t471 = load i64, ptr %t461
  %t472 = call i64 @"cinch.prim.bitAndU64"(i64 %t471, i64 1023)
  %t473 = call i64 @"Main.mix"(i64 %t470, i64 %t472)
  store i64 %t473, ptr %t474
  %t475 = load i64, ptr %t8
  call void @cinch.rc.drop.obj(i64 %t475)
  %t476 = load i64, ptr %t464
  %t477 = load i64, ptr %t2
  %t478 = load i64, ptr %t0
  %t479 = load i64, ptr %t1
  %t480 = load i64, ptr %t469
  %t481 = load i64, ptr %t474
  %t482 = load i64, ptr %t427
  %t483 = musttail call tailcc { i64, i64, i64 } @"Main.v7-loop.shared0#cc"(i64 %t476, i64 %t477, i64 %t478, i64 %t479, i64 %t480, i64 %t481, i64 %t482)
  ret { i64, i64, i64 } %t483
bb77:
  ret { i64, i64, i64 } zeroinitializer
bb2:
  %t484 = load i64, ptr %t1
  call void @cinch.rc.drop.obj(i64 %t484)
  %t485 = load i64, ptr %t6
  %t486 = load i64, ptr %t7
  %t487 = load i64, ptr %t8
  %t488 = insertvalue { i64, i64, i64 } poison, i64 %t485, 0
  %t489 = insertvalue { i64, i64, i64 } %t488, i64 %t486, 1
  %t490 = insertvalue { i64, i64, i64 } %t489, i64 %t487, 2
  ret { i64, i64, i64 } %t490
}
