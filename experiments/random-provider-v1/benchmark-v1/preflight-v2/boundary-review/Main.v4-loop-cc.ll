define { i64, i64, i64 } @"Main.v4-loop#cc"(i64 %p0, i32 noundef %p1, i32 noundef %p2, i64 noundef %p3, i64 noundef %p4, i64 %p5) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t3 = alloca i64
  %t5 = alloca i64
  %t6 = alloca i64
  %t7 = alloca i64
  %t13 = alloca i64
  %t16 = alloca i64
  %t19 = alloca i64
  %t34 = alloca i64
  %t37 = alloca i64
  %t40 = alloca i64
  %t47 = alloca i64
  %t50 = alloca i64
  %t53 = alloca i64
  %t68 = alloca i64
  %t80 = alloca i64
  %t86 = alloca i64
  %t89 = alloca i64
  %t92 = alloca i64
  %t115 = alloca i64
  %t119 = alloca i64
  %t134 = alloca i64
  %t137 = alloca i64
  %t141 = alloca i64
  %t146 = alloca i64
  %t151 = alloca i64
  %t154 = alloca i64
  %t155 = alloca i64
  %t158 = alloca i64
  %t161 = alloca i64
  %t170 = alloca i64
  %t173 = alloca i64
  %t180 = alloca i64
  %t183 = alloca i64
  %t188 = alloca i64
  %t191 = alloca i64
  %t196 = alloca i64
  %t201 = alloca i64
  %t208 = alloca i64
  %t211 = alloca i64
  %t214 = alloca i64
  %t229 = alloca i64
  %t241 = alloca i64
  %t247 = alloca i64
  %t250 = alloca i64
  %t253 = alloca i64
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
  %t12 = call i64 @"cinch.runtime.randomBytes"(i64 33)
  store i64 %t12, ptr %t13
  %t14 = load i64, ptr %t13
  %t15 = call i64 @"Main.v4-loop.shared0"(i64 %t14)
  store i64 %t15, ptr %t16
  %t17 = load i64, ptr %t13
  call void @cinch.rc.drop.obj(i64 %t17)
  %t18 = load i64, ptr %t16
  store i64 %t18, ptr %t19
  %t20 = load i64, ptr %t19
  %t21 = inttoptr i64 %t20 to ptr
  %t22 = getelementptr inbounds i8, ptr %t21, i64 -8
  %t23 = load atomic i64, ptr %t22 monotonic, align 8
  %t24 = lshr i64 %t23, 40
  %t25 = and i64 %t24, 255
  %t26 = icmp eq i64 %t25, 255
  br i1 %t26, label %bb4, label %bb5
bb4:
  %t27 = getelementptr inbounds i8, ptr %t21, i64 -16
  %t28 = load i64, ptr %t27
  %t29 = inttoptr i64 %t28 to ptr
  %t30 = load i64, ptr %t29
  br label %bb6
bb5:
  br label %bb6
bb6:
  %t31 = phi i64 [ %t30, %bb4 ], [ %t25, %bb5 ]
  switch i64 %t31, label %bb3 [
  i64 0, label %bb7
  i64 1, label %bb8
]
bb7:
  %t32 = getelementptr inbounds i8, ptr %t21, i64 0
  %t33 = load i64, ptr %t32
  store i64 %t33, ptr %t34
  %t35 = load i64, ptr %t3
  %t36 = call i64 @"Main.bump"(i64 %t35)
  store i64 %t36, ptr %t37
  %t38 = load i64, ptr %t6
  %t39 = call i64 @"Main.mix"(i64 %t38, i64 4294967295)
  store i64 %t39, ptr %t40
  %t41 = load i64, ptr %t19
  call void @cinch.rc.drop.obj(i64 %t41)
  %t42 = load i64, ptr %t37
  %t43 = load i64, ptr %t1
  %t44 = call i64 @"Main.before"(i64 %t42, i64 %t43)
  %t45 = icmp ne i64 %t44, 0
  br i1 %t45, label %bb9, label %bb10
bb9:
  %t46 = call i64 @"cinch.runtime.randomBytes"(i64 33)
  store i64 %t46, ptr %t47
  %t48 = load i64, ptr %t47
  %t49 = call i64 @"Main.v4-loop.shared0"(i64 %t48)
  store i64 %t49, ptr %t50
  %t51 = load i64, ptr %t47
  call void @cinch.rc.drop.obj(i64 %t51)
  %t52 = load i64, ptr %t50
  store i64 %t52, ptr %t53
  %t54 = load i64, ptr %t53
  %t55 = inttoptr i64 %t54 to ptr
  %t56 = getelementptr inbounds i8, ptr %t55, i64 -8
  %t57 = load atomic i64, ptr %t56 monotonic, align 8
  %t58 = lshr i64 %t57, 40
  %t59 = and i64 %t58, 255
  %t60 = icmp eq i64 %t59, 255
  br i1 %t60, label %bb12, label %bb13
bb12:
  %t61 = getelementptr inbounds i8, ptr %t55, i64 -16
  %t62 = load i64, ptr %t61
  %t63 = inttoptr i64 %t62 to ptr
  %t64 = load i64, ptr %t63
  br label %bb14
bb13:
  br label %bb14
bb14:
  %t65 = phi i64 [ %t64, %bb12 ], [ %t59, %bb13 ]
  switch i64 %t65, label %bb11 [
  i64 0, label %bb15
  i64 1, label %bb16
]
bb15:
  %t66 = getelementptr inbounds i8, ptr %t55, i64 0
  %t67 = load i64, ptr %t66
  store i64 %t67, ptr %t68
  %t69 = load i64, ptr %t53
  call void @cinch.rc.drop.obj(i64 %t69)
  %t70 = load i64, ptr %t0
  %t71 = load i64, ptr %t1
  %t72 = load i64, ptr %t37
  %t73 = call i64 @"Main.bump"(i64 %t72)
  %t74 = load i64, ptr %t5
  %t75 = load i64, ptr %t40
  %t76 = call i64 @"Main.mix"(i64 %t75, i64 4294967295)
  %t77 = load i64, ptr %t7
  store i64 %t70, ptr %t0
  store i64 %t71, ptr %t1
  store i64 %t73, ptr %t3
  store i64 %t74, ptr %t5
  store i64 %t76, ptr %t6
  store i64 %t77, ptr %t7
  br label %bb0
bb16:
  %t78 = getelementptr inbounds i8, ptr %t55, i64 0
  %t79 = load i64, ptr %t78
  store i64 %t79, ptr %t80
  %t81 = load i64, ptr %t53
  %t82 = call i1 @cinch.rc.unique(i64 %t81)
  br i1 %t82, label %bb17, label %bb18
bb17:
  call void @cinch_free(i64 %t81)
  br label %bb19
bb18:
  %t83 = load i64, ptr %t80
  call void @cinch.rc.dup.obj(i64 %t83)
  call void @cinch.rc.drop.obj(i64 %t81)
  br label %bb19
bb19:
  %t84 = load i64, ptr %t80
  %t85 = call i64 @"Data.Uuid.uuid-to-bytes"(i64 %t84)
  store i64 %t85, ptr %t86
  %t87 = load i64, ptr %t80
  call void @cinch.rc.drop.obj(i64 %t87)
  %t88 = load i64, ptr %t86
  store i64 %t88, ptr %t89
  %t90 = load i64, ptr %t89
  %t91 = call i64 @"Main.observe-uuid"(i64 4, i64 %t90)
  store i64 %t91, ptr %t92
  %t93 = load i64, ptr %t7
  call void @cinch.rc.drop.obj(i64 %t93)
  %t94 = load i64, ptr %t0
  %t95 = load i64, ptr %t1
  %t96 = load i64, ptr %t37
  %t97 = call i64 @"Main.bump"(i64 %t96)
  %t98 = load i64, ptr %t5
  %t99 = load i64, ptr %t92
  %t100 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t99, i64 63)
  %t101 = call i64 @"Main.add"(i64 %t98, i64 %t100)
  %t102 = load i64, ptr %t40
  %t103 = load i64, ptr %t92
  %t104 = call i64 @"cinch.prim.bitAndU64"(i64 %t103, i64 1023)
  %t105 = call i64 @"Main.mix"(i64 %t102, i64 %t104)
  %t106 = load i64, ptr %t89
  store i64 %t94, ptr %t0
  store i64 %t95, ptr %t1
  store i64 %t97, ptr %t3
  store i64 %t101, ptr %t5
  store i64 %t105, ptr %t6
  store i64 %t106, ptr %t7
  br label %bb0
bb11:
  ret { i64, i64, i64 } zeroinitializer
bb10:
  %t107 = load i64, ptr %t5
  %t108 = load i64, ptr %t40
  %t109 = load i64, ptr %t7
  %t110 = insertvalue { i64, i64, i64 } poison, i64 %t107, 0
  %t111 = insertvalue { i64, i64, i64 } %t110, i64 %t108, 1
  %t112 = insertvalue { i64, i64, i64 } %t111, i64 %t109, 2
  ret { i64, i64, i64 } %t112
bb8:
  %t113 = getelementptr inbounds i8, ptr %t21, i64 0
  %t114 = load i64, ptr %t113
  store i64 %t114, ptr %t115
  %t116 = load i64, ptr %t19
  %t117 = call i1 @cinch.rc.unique(i64 %t116)
  br i1 %t117, label %bb20, label %bb21
bb20:
  call void @cinch_free(i64 %t116)
  br label %bb22
bb21:
  %t118 = load i64, ptr %t115
  call void @cinch.rc.dup.obj(i64 %t118)
  call void @cinch.rc.drop.obj(i64 %t116)
  br label %bb22
bb22:
  %t120 = load i64, ptr %t115
  %t121 = inttoptr i64 %t120 to ptr
  %t122 = getelementptr inbounds i8, ptr %t121, i64 -8
  %t123 = load atomic i64, ptr %t122 monotonic, align 8
  %t124 = lshr i64 %t123, 40
  %t125 = and i64 %t124, 255
  %t126 = icmp eq i64 %t125, 255
  br i1 %t126, label %bb25, label %bb26
bb25:
  %t127 = getelementptr inbounds i8, ptr %t121, i64 -16
  %t128 = load i64, ptr %t127
  %t129 = inttoptr i64 %t128 to ptr
  %t130 = load i64, ptr %t129
  br label %bb27
bb26:
  br label %bb27
bb27:
  %t131 = phi i64 [ %t130, %bb25 ], [ %t125, %bb26 ]
  switch i64 %t131, label %bb24 [
  i64 0, label %bb28
]
bb28:
  %t132 = getelementptr inbounds i8, ptr %t121, i64 0
  %t133 = load i64, ptr %t132
  store i64 %t133, ptr %t134
  %t135 = getelementptr inbounds i8, ptr %t121, i64 8
  %t136 = load i64, ptr %t135
  store i64 %t136, ptr %t137
  %t138 = load i64, ptr %t115
  %t139 = call i1 @cinch.rc.unique(i64 %t138)
  br i1 %t139, label %bb29, label %bb30
bb29:
  call void @cinch_free(i64 %t138)
  br label %bb31
bb30:
  call void @cinch.rc.drop.obj(i64 %t138)
  br label %bb31
bb31:
  %t140 = call i64 @"cinch.runtime.bytesNew"(i64 33)
  store i64 %t140, ptr %t141
  %t142 = load i64, ptr %t141
  %t143 = load i64, ptr %t134
  %t144 = call i64 @"Data.Uuid.network-word"(i64 %t143)
  %t145 = call i64 @"Data.Bytes.Word.bytes-write-le64"(i64 %t142, i64 1, i64 %t144)
  store i64 %t145, ptr %t146
  %t147 = load i64, ptr %t141
  %t148 = load i64, ptr %t137
  %t149 = call i64 @"Data.Uuid.network-word"(i64 %t148)
  %t150 = call i64 @"Data.Bytes.Word.bytes-write-le64"(i64 %t147, i64 17, i64 %t149)
  store i64 %t150, ptr %t151
  %t152 = load i64, ptr %t141
  store i64 %t152, ptr %t119
  br label %bb23
bb24:
  ret { i64, i64, i64 } zeroinitializer
bb23:
  %t153 = load i64, ptr %t119
  store i64 %t153, ptr %t154
  %t156 = load i64, ptr %t154
  %t157 = call i64 @"cinch.runtime.bytesLen"(i64 %t156)
  store i64 %t157, ptr %t158
  %t159 = load i64, ptr %t158
  %t160 = call i64 @"cinch.runtime.natCmp"(i64 %t159, i64 33)
  store i64 %t160, ptr %t161
  %t162 = load i64, ptr %t158
  call void @cinch.rc.drop.tobj(i64 %t162)
  %t163 = load i64, ptr %t161
  %t164 = call i64 @"cinch.prim.eqI64"(i64 %t163, i64 0), !range !122
  %t165 = icmp ne i64 %t164, 0
  br i1 %t165, label %bb33, label %bb34
bb33:
  %t166 = load i64, ptr %t154
  %t167 = call i64 @"cinch.runtime.bytesData"(i64 %t166)
  %t168 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t169 = call i64 @"Data.Bytes.Word.load-le64"(i64 %t167, i64 %t168)
  store i64 %t169, ptr %t170
  %t171 = load i64, ptr %t154
  %t172 = call i64 @"cinch.runtime.bytesLen"(i64 %t171)
  store i64 %t172, ptr %t173
  %t174 = load i64, ptr %t173
  call void @cinch.rc.drop.tobj(i64 %t174)
  %t175 = load i64, ptr %t170
  %t176 = load i64, ptr %t154
  %t177 = call i64 @"cinch.runtime.bytesData"(i64 %t176)
  %t178 = call i64 @"cinch.runtime.natToI64"(i64 17)
  %t179 = call i64 @"Data.Bytes.Word.load-le64"(i64 %t177, i64 %t178)
  store i64 %t179, ptr %t180
  %t181 = load i64, ptr %t154
  %t182 = call i64 @"cinch.runtime.bytesLen"(i64 %t181)
  store i64 %t182, ptr %t183
  %t184 = load i64, ptr %t183
  call void @cinch.rc.drop.tobj(i64 %t184)
  %t185 = load i64, ptr %t180
  %t186 = call i64 @"Main.observed-words"(i64 %t175, i64 %t185, i64 4)
  store i64 %t186, ptr %t155
  br label %bb32
bb34:
  store i64 0, ptr %t155
  br label %bb32
bb32:
  %t187 = load i64, ptr %t155
  store i64 %t187, ptr %t188
  %t189 = load i64, ptr %t3
  %t190 = call i64 @"Main.bump"(i64 %t189)
  store i64 %t190, ptr %t191
  %t192 = load i64, ptr %t5
  %t193 = load i64, ptr %t188
  %t194 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t193, i64 63)
  %t195 = call i64 @"Main.add"(i64 %t192, i64 %t194)
  store i64 %t195, ptr %t196
  %t197 = load i64, ptr %t6
  %t198 = load i64, ptr %t188
  %t199 = call i64 @"cinch.prim.bitAndU64"(i64 %t198, i64 1023)
  %t200 = call i64 @"Main.mix"(i64 %t197, i64 %t199)
  store i64 %t200, ptr %t201
  %t202 = load i64, ptr %t7
  call void @cinch.rc.drop.obj(i64 %t202)
  %t203 = load i64, ptr %t191
  %t204 = load i64, ptr %t1
  %t205 = call i64 @"Main.before"(i64 %t203, i64 %t204)
  %t206 = icmp ne i64 %t205, 0
  br i1 %t206, label %bb35, label %bb36
bb35:
  %t207 = call i64 @"cinch.runtime.randomBytes"(i64 33)
  store i64 %t207, ptr %t208
  %t209 = load i64, ptr %t208
  %t210 = call i64 @"Data.Uuid.uuid-v4-from-random"(i64 %t209)
  store i64 %t210, ptr %t211
  %t212 = load i64, ptr %t208
  call void @cinch.rc.drop.obj(i64 %t212)
  %t213 = load i64, ptr %t211
  store i64 %t213, ptr %t214
  %t215 = load i64, ptr %t214
  %t216 = inttoptr i64 %t215 to ptr
  %t217 = getelementptr inbounds i8, ptr %t216, i64 -8
  %t218 = load atomic i64, ptr %t217 monotonic, align 8
  %t219 = lshr i64 %t218, 40
  %t220 = and i64 %t219, 255
  %t221 = icmp eq i64 %t220, 255
  br i1 %t221, label %bb38, label %bb39
bb38:
  %t222 = getelementptr inbounds i8, ptr %t216, i64 -16
  %t223 = load i64, ptr %t222
  %t224 = inttoptr i64 %t223 to ptr
  %t225 = load i64, ptr %t224
  br label %bb40
bb39:
  br label %bb40
bb40:
  %t226 = phi i64 [ %t225, %bb38 ], [ %t220, %bb39 ]
  switch i64 %t226, label %bb37 [
  i64 0, label %bb41
  i64 1, label %bb42
]
bb41:
  %t227 = getelementptr inbounds i8, ptr %t216, i64 0
  %t228 = load i64, ptr %t227
  store i64 %t228, ptr %t229
  %t230 = load i64, ptr %t214
  call void @cinch.rc.drop.obj(i64 %t230)
  %t231 = load i64, ptr %t0
  %t232 = load i64, ptr %t1
  %t233 = load i64, ptr %t191
  %t234 = call i64 @"Main.bump"(i64 %t233)
  %t235 = load i64, ptr %t196
  %t236 = load i64, ptr %t201
  %t237 = call i64 @"Main.mix"(i64 %t236, i64 4294967295)
  %t238 = load i64, ptr %t154
  store i64 %t231, ptr %t0
  store i64 %t232, ptr %t1
  store i64 %t234, ptr %t3
  store i64 %t235, ptr %t5
  store i64 %t237, ptr %t6
  store i64 %t238, ptr %t7
  br label %bb0
bb42:
  %t239 = getelementptr inbounds i8, ptr %t216, i64 0
  %t240 = load i64, ptr %t239
  store i64 %t240, ptr %t241
  %t242 = load i64, ptr %t214
  %t243 = call i1 @cinch.rc.unique(i64 %t242)
  br i1 %t243, label %bb43, label %bb44
bb43:
  call void @cinch_free(i64 %t242)
  br label %bb45
bb44:
  %t244 = load i64, ptr %t241
  call void @cinch.rc.dup.obj(i64 %t244)
  call void @cinch.rc.drop.obj(i64 %t242)
  br label %bb45
bb45:
  %t245 = load i64, ptr %t241
  %t246 = call i64 @"Data.Uuid.uuid-to-bytes"(i64 %t245)
  store i64 %t246, ptr %t247
  %t248 = load i64, ptr %t241
  call void @cinch.rc.drop.obj(i64 %t248)
  %t249 = load i64, ptr %t247
  store i64 %t249, ptr %t250
  %t251 = load i64, ptr %t250
  %t252 = call i64 @"Main.observe-uuid"(i64 4, i64 %t251)
  store i64 %t252, ptr %t253
  %t254 = load i64, ptr %t154
  call void @cinch.rc.drop.obj(i64 %t254)
  %t255 = load i64, ptr %t0
  %t256 = load i64, ptr %t1
  %t257 = load i64, ptr %t191
  %t258 = call i64 @"Main.bump"(i64 %t257)
  %t259 = load i64, ptr %t196
  %t260 = load i64, ptr %t253
  %t261 = call i64 @"cinch.prim.wrappingShrU64"(i64 %t260, i64 63)
  %t262 = call i64 @"Main.add"(i64 %t259, i64 %t261)
  %t263 = load i64, ptr %t201
  %t264 = load i64, ptr %t253
  %t265 = call i64 @"cinch.prim.bitAndU64"(i64 %t264, i64 1023)
  %t266 = call i64 @"Main.mix"(i64 %t263, i64 %t265)
  %t267 = load i64, ptr %t250
  store i64 %t255, ptr %t0
  store i64 %t256, ptr %t1
  store i64 %t258, ptr %t3
  store i64 %t262, ptr %t5
  store i64 %t266, ptr %t6
  store i64 %t267, ptr %t7
  br label %bb0
bb37:
  ret { i64, i64, i64 } zeroinitializer
bb36:
  %t268 = load i64, ptr %t196
  %t269 = load i64, ptr %t201
  %t270 = load i64, ptr %t154
  %t271 = insertvalue { i64, i64, i64 } poison, i64 %t268, 0
  %t272 = insertvalue { i64, i64, i64 } %t271, i64 %t269, 1
  %t273 = insertvalue { i64, i64, i64 } %t272, i64 %t270, 2
  ret { i64, i64, i64 } %t273
bb3:
  ret { i64, i64, i64 } zeroinitializer
bb2:
  %t274 = load i64, ptr %t5
  %t275 = load i64, ptr %t6
  %t276 = load i64, ptr %t7
  %t277 = insertvalue { i64, i64, i64 } poison, i64 %t274, 0
  %t278 = insertvalue { i64, i64, i64 } %t277, i64 %t275, 1
  %t279 = insertvalue { i64, i64, i64 } %t278, i64 %t276, 2
  ret { i64, i64, i64 } %t279
}
