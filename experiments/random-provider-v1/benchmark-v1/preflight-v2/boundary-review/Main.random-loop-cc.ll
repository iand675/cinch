define { i64, i64, i64 } @"Main.random-loop#cc"(i64 %p0, i64 %p1, i32 noundef %p2, i32 noundef %p3, i64 noundef %p4, i64 noundef %p5, i64 %p6) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t2 = alloca i64
  %t4 = alloca i64
  %t6 = alloca i64
  %t7 = alloca i64
  %t8 = alloca i64
  %t15 = alloca i64
  %t16 = alloca i64
  %t19 = alloca i64
  %t23 = alloca i64
  %t28 = alloca i64
  %t31 = alloca i64
  %t38 = alloca i64
  %t41 = alloca i64
  %t49 = alloca i64
  %t52 = alloca i64
  %t56 = alloca i64
  %t60 = alloca i64
  %t61 = alloca i64
  %t64 = alloca i64
  %t67 = alloca i64
  %t75 = alloca i64
  %t79 = alloca i64
  %t82 = alloca i64
  %t86 = alloca i64
  %t87 = alloca i64
  %t90 = alloca i64
  %t94 = alloca i64
  %t102 = alloca i64
  %t107 = alloca i64
  %t111 = alloca i64
  %t115 = alloca i64
  %t116 = alloca i64
  %t123 = alloca i64
  %t128 = alloca i64
  %t129 = alloca i64
  %t132 = alloca i64
  %t136 = alloca i64
  %t144 = alloca i64
  %t149 = alloca i64
  %t153 = alloca i64
  %t157 = alloca i64
  %t158 = alloca i64
  %t159 = alloca i64
  %t163 = alloca i64
  %t170 = alloca i64
  %t177 = alloca i64
  %t185 = alloca i64
  %t186 = alloca i64
  %t189 = alloca i64
  %t193 = alloca i64
  %t198 = alloca i64
  %t199 = alloca i64
  %t203 = alloca i64
  %t210 = alloca i64
  %t215 = alloca i64
  %t216 = alloca i64
  %t220 = alloca i64
  %t227 = alloca i64
  %t233 = alloca i64
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
  %t13 = load i64, ptr %t1
  %t14 = call i64 @"cinch.runtime.randomBytes"(i64 %t13)
  store i64 %t14, ptr %t15
  %t17 = load i64, ptr %t15
  %t18 = call i64 @"cinch.runtime.bytesLen"(i64 %t17)
  store i64 %t18, ptr %t19
  %t20 = load i64, ptr %t19
  %t21 = load i64, ptr %t1
  %t22 = call i64 @"cinch.runtime.natCmp"(i64 %t20, i64 %t21)
  store i64 %t22, ptr %t23
  %t24 = load i64, ptr %t19
  call void @cinch.rc.drop.tobj(i64 %t24)
  %t25 = load i64, ptr %t23
  %t26 = call i64 @"cinch.prim.eqI64"(i64 %t25, i64 0), !range !86
  %t27 = icmp ne i64 %t26, 0
  br i1 %t27, label %bb4, label %bb5
bb4:
  store i64 -1, ptr %t28
  %t29 = load i64, ptr %t28
  %t30 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t29)
  store i64 %t30, ptr %t31
  %t32 = load i64, ptr %t28
  call void @cinch.rc.drop.tobj(i64 %t32)
  %t33 = load i64, ptr %t31
  %t34 = call i64 @"cinch.prim.leI64"(i64 %t33, i64 0), !range !87
  %t35 = icmp ne i64 %t34, 0
  br i1 %t35, label %bb6, label %bb7
bb6:
  %t36 = call i64 @"cinch.runtime.natToI64"(i64 3)
  %t37 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t36)
  store i64 %t37, ptr %t16
  br label %bb3
bb7:
  store i64 0, ptr %t16
  br label %bb3
bb5:
  store i64 -1, ptr %t38
  %t39 = load i64, ptr %t38
  %t40 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t39)
  store i64 %t40, ptr %t41
  %t42 = load i64, ptr %t38
  call void @cinch.rc.drop.tobj(i64 %t42)
  %t43 = load i64, ptr %t41
  %t44 = call i64 @"cinch.prim.leI64"(i64 %t43, i64 0), !range !88
  %t45 = icmp ne i64 %t44, 0
  br i1 %t45, label %bb8, label %bb9
bb8:
  %t46 = call i64 @"cinch.runtime.natToI64"(i64 1)
  %t47 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t46)
  store i64 %t47, ptr %t16
  br label %bb3
bb9:
  store i64 0, ptr %t16
  br label %bb3
bb3:
  %t48 = load i64, ptr %t16
  store i64 %t48, ptr %t49
  %t50 = load i64, ptr %t4
  %t51 = call i64 @"Main.bump"(i64 %t50)
  store i64 %t51, ptr %t52
  %t53 = load i64, ptr %t6
  %t54 = load i64, ptr %t49
  %t55 = call i64 @"Main.add"(i64 %t53, i64 %t54)
  store i64 %t55, ptr %t56
  %t57 = load i64, ptr %t7
  %t58 = load i64, ptr %t15
  %t59 = call i64 @"cinch.runtime.bytesLen"(i64 %t58)
  store i64 %t59, ptr %t60
  %t62 = load i64, ptr %t15
  %t63 = call i64 @"cinch.runtime.bytesLen"(i64 %t62)
  store i64 %t63, ptr %t64
  %t65 = load i64, ptr %t64
  %t66 = call i64 @"cinch.runtime.natCmp"(i64 1, i64 %t65)
  store i64 %t66, ptr %t67
  %t68 = load i64, ptr %t64
  call void @cinch.rc.drop.tobj(i64 %t68)
  %t69 = load i64, ptr %t67
  %t70 = call i64 @"cinch.prim.ltI64"(i64 %t69, i64 0), !range !89
  %t71 = icmp ne i64 %t70, 0
  br i1 %t71, label %bb11, label %bb12
bb11:
  %t72 = load i64, ptr %t15
  %t73 = call i64 @"cinch.runtime.bytesGet"(i64 %t72, i64 1)
  store i64 %t73, ptr %t61
  br label %bb10
bb12:
  store i64 1, ptr %t61
  br label %bb10
bb10:
  %t74 = load i64, ptr %t61
  store i64 %t74, ptr %t75
  %t76 = load i64, ptr %t60
  %t77 = load i64, ptr %t75
  %t78 = call i64 @"cinch.runtime.natAdd"(i64 %t76, i64 %t77)
  store i64 %t78, ptr %t79
  %t80 = load i64, ptr %t75
  call void @cinch.rc.drop.tobj(i64 %t80)
  %t81 = load i64, ptr %t79
  store i64 %t81, ptr %t82
  %t83 = load i64, ptr %t60
  call void @cinch.rc.dup.tobj(i64 %t83)
  %t84 = load i64, ptr %t60
  %t85 = call i64 @"cinch.runtime.natDiv"(i64 %t84, i64 5)
  store i64 %t85, ptr %t86
  %t88 = load i64, ptr %t15
  %t89 = call i64 @"cinch.runtime.bytesLen"(i64 %t88)
  store i64 %t89, ptr %t90
  %t91 = load i64, ptr %t86
  %t92 = load i64, ptr %t90
  %t93 = call i64 @"cinch.runtime.natCmp"(i64 %t91, i64 %t92)
  store i64 %t93, ptr %t94
  %t95 = load i64, ptr %t90
  call void @cinch.rc.drop.tobj(i64 %t95)
  %t96 = load i64, ptr %t94
  %t97 = call i64 @"cinch.prim.ltI64"(i64 %t96, i64 0), !range !90
  %t98 = icmp ne i64 %t97, 0
  br i1 %t98, label %bb14, label %bb15
bb14:
  %t99 = load i64, ptr %t15
  %t100 = load i64, ptr %t86
  %t101 = call i64 @"cinch.runtime.bytesGet"(i64 %t99, i64 %t100)
  store i64 %t101, ptr %t102
  %t103 = load i64, ptr %t86
  call void @cinch.rc.drop.tobj(i64 %t103)
  %t104 = load i64, ptr %t102
  store i64 %t104, ptr %t87
  br label %bb13
bb15:
  %t105 = load i64, ptr %t86
  call void @cinch.rc.drop.tobj(i64 %t105)
  store i64 1, ptr %t87
  br label %bb13
bb13:
  %t106 = load i64, ptr %t87
  store i64 %t106, ptr %t107
  %t108 = load i64, ptr %t82
  %t109 = load i64, ptr %t107
  %t110 = call i64 @"cinch.runtime.natAdd"(i64 %t108, i64 %t109)
  store i64 %t110, ptr %t111
  %t112 = load i64, ptr %t82
  call void @cinch.rc.drop.tobj(i64 %t112)
  %t113 = load i64, ptr %t107
  call void @cinch.rc.drop.tobj(i64 %t113)
  %t114 = load i64, ptr %t111
  store i64 %t114, ptr %t115
  %t117 = load i64, ptr %t60
  %t118 = call i64 @"cinch.runtime.natCmp"(i64 3, i64 %t117)
  %t119 = call i64 @"cinch.prim.leI64"(i64 %t118, i64 0), !range !91
  %t120 = icmp ne i64 %t119, 0
  br i1 %t120, label %bb17, label %bb18
bb17:
  %t121 = load i64, ptr %t60
  %t122 = call i64 @"cinch.runtime.natSub"(i64 %t121, i64 3)
  store i64 %t122, ptr %t123
  %t124 = load i64, ptr %t60
  call void @cinch.rc.drop.tobj(i64 %t124)
  %t125 = load i64, ptr %t123
  store i64 %t125, ptr %t116
  br label %bb16
bb18:
  %t126 = load i64, ptr %t60
  call void @cinch.rc.drop.tobj(i64 %t126)
  store i64 1, ptr %t116
  br label %bb16
bb16:
  %t127 = load i64, ptr %t116
  store i64 %t127, ptr %t128
  %t130 = load i64, ptr %t15
  %t131 = call i64 @"cinch.runtime.bytesLen"(i64 %t130)
  store i64 %t131, ptr %t132
  %t133 = load i64, ptr %t128
  %t134 = load i64, ptr %t132
  %t135 = call i64 @"cinch.runtime.natCmp"(i64 %t133, i64 %t134)
  store i64 %t135, ptr %t136
  %t137 = load i64, ptr %t132
  call void @cinch.rc.drop.tobj(i64 %t137)
  %t138 = load i64, ptr %t136
  %t139 = call i64 @"cinch.prim.ltI64"(i64 %t138, i64 0), !range !92
  %t140 = icmp ne i64 %t139, 0
  br i1 %t140, label %bb20, label %bb21
bb20:
  %t141 = load i64, ptr %t15
  %t142 = load i64, ptr %t128
  %t143 = call i64 @"cinch.runtime.bytesGet"(i64 %t141, i64 %t142)
  store i64 %t143, ptr %t144
  %t145 = load i64, ptr %t128
  call void @cinch.rc.drop.tobj(i64 %t145)
  %t146 = load i64, ptr %t144
  store i64 %t146, ptr %t129
  br label %bb19
bb21:
  %t147 = load i64, ptr %t128
  call void @cinch.rc.drop.tobj(i64 %t147)
  store i64 1, ptr %t129
  br label %bb19
bb19:
  %t148 = load i64, ptr %t129
  store i64 %t148, ptr %t149
  %t150 = load i64, ptr %t115
  %t151 = load i64, ptr %t149
  %t152 = call i64 @"cinch.runtime.natAdd"(i64 %t150, i64 %t151)
  store i64 %t152, ptr %t153
  %t154 = load i64, ptr %t115
  call void @cinch.rc.drop.tobj(i64 %t154)
  %t155 = load i64, ptr %t149
  call void @cinch.rc.drop.tobj(i64 %t155)
  %t156 = load i64, ptr %t153
  store i64 %t156, ptr %t157
  store i64 -1, ptr %t159
  %t160 = load i64, ptr %t157
  %t161 = load i64, ptr %t159
  %t162 = call i64 @"cinch.runtime.natCmp"(i64 %t160, i64 %t161)
  store i64 %t162, ptr %t163
  %t164 = load i64, ptr %t159
  call void @cinch.rc.drop.tobj(i64 %t164)
  %t165 = load i64, ptr %t163
  %t166 = call i64 @"cinch.prim.leI64"(i64 %t165, i64 0), !range !93
  %t167 = icmp ne i64 %t166, 0
  br i1 %t167, label %bb23, label %bb24
bb23:
  %t168 = load i64, ptr %t157
  %t169 = call i64 @"cinch.runtime.natToI64"(i64 %t168)
  store i64 %t169, ptr %t170
  %t171 = load i64, ptr %t157
  call void @cinch.rc.drop.tobj(i64 %t171)
  %t172 = load i64, ptr %t170
  %t173 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t172)
  store i64 %t173, ptr %t158
  br label %bb22
bb24:
  %t174 = load i64, ptr %t157
  call void @cinch.rc.drop.tobj(i64 %t174)
  store i64 0, ptr %t158
  br label %bb22
bb22:
  %t175 = load i64, ptr %t158
  %t176 = call i64 @"Main.mix"(i64 %t57, i64 %t175)
  store i64 %t176, ptr %t177
  %t178 = load i64, ptr %t8
  call void @cinch.rc.drop.obj(i64 %t178)
  %t179 = load i64, ptr %t52
  %t180 = load i64, ptr %t2
  %t181 = call i64 @"Main.before"(i64 %t179, i64 %t180)
  %t182 = icmp ne i64 %t181, 0
  br i1 %t182, label %bb25, label %bb26
bb25:
  %t183 = load i64, ptr %t1
  %t184 = call i64 @"cinch.runtime.randomBytes"(i64 %t183)
  store i64 %t184, ptr %t185
  %t187 = load i64, ptr %t185
  %t188 = call i64 @"cinch.runtime.bytesLen"(i64 %t187)
  store i64 %t188, ptr %t189
  %t190 = load i64, ptr %t189
  %t191 = load i64, ptr %t1
  %t192 = call i64 @"cinch.runtime.natCmp"(i64 %t190, i64 %t191)
  store i64 %t192, ptr %t193
  %t194 = load i64, ptr %t189
  call void @cinch.rc.drop.tobj(i64 %t194)
  %t195 = load i64, ptr %t193
  %t196 = call i64 @"cinch.prim.eqI64"(i64 %t195, i64 0), !range !94
  %t197 = icmp ne i64 %t196, 0
  br i1 %t197, label %bb28, label %bb29
bb28:
  store i64 3, ptr %t198
  store i64 -1, ptr %t199
  %t200 = load i64, ptr %t198
  %t201 = load i64, ptr %t199
  %t202 = call i64 @"cinch.runtime.natCmp"(i64 %t200, i64 %t201)
  store i64 %t202, ptr %t203
  %t204 = load i64, ptr %t199
  call void @cinch.rc.drop.tobj(i64 %t204)
  %t205 = load i64, ptr %t203
  %t206 = call i64 @"cinch.prim.leI64"(i64 %t205, i64 0), !range !95
  %t207 = icmp ne i64 %t206, 0
  br i1 %t207, label %bb30, label %bb31
bb30:
  %t208 = load i64, ptr %t198
  %t209 = call i64 @"cinch.runtime.natToI64"(i64 %t208)
  store i64 %t209, ptr %t210
  %t211 = load i64, ptr %t198
  call void @cinch.rc.drop.tobj(i64 %t211)
  %t212 = load i64, ptr %t210
  %t213 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t212)
  store i64 %t213, ptr %t186
  br label %bb27
bb31:
  %t214 = load i64, ptr %t198
  call void @cinch.rc.drop.tobj(i64 %t214)
  store i64 0, ptr %t186
  br label %bb27
bb29:
  store i64 1, ptr %t215
  store i64 -1, ptr %t216
  %t217 = load i64, ptr %t215
  %t218 = load i64, ptr %t216
  %t219 = call i64 @"cinch.runtime.natCmp"(i64 %t217, i64 %t218)
  store i64 %t219, ptr %t220
  %t221 = load i64, ptr %t216
  call void @cinch.rc.drop.tobj(i64 %t221)
  %t222 = load i64, ptr %t220
  %t223 = call i64 @"cinch.prim.leI64"(i64 %t222, i64 0), !range !96
  %t224 = icmp ne i64 %t223, 0
  br i1 %t224, label %bb32, label %bb33
bb32:
  %t225 = load i64, ptr %t215
  %t226 = call i64 @"cinch.runtime.natToI64"(i64 %t225)
  store i64 %t226, ptr %t227
  %t228 = load i64, ptr %t215
  call void @cinch.rc.drop.tobj(i64 %t228)
  %t229 = load i64, ptr %t227
  %t230 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t229)
  store i64 %t230, ptr %t186
  br label %bb27
bb33:
  %t231 = load i64, ptr %t215
  call void @cinch.rc.drop.tobj(i64 %t231)
  store i64 0, ptr %t186
  br label %bb27
bb27:
  %t232 = load i64, ptr %t186
  store i64 %t232, ptr %t233
  %t234 = load i64, ptr %t15
  call void @cinch.rc.drop.obj(i64 %t234)
  %t235 = load i64, ptr %t0
  %t236 = load i64, ptr %t1
  %t237 = load i64, ptr %t2
  %t238 = load i64, ptr %t52
  %t239 = call i64 @"Main.bump"(i64 %t238)
  %t240 = load i64, ptr %t56
  %t241 = load i64, ptr %t233
  %t242 = call i64 @"Main.add"(i64 %t240, i64 %t241)
  %t243 = load i64, ptr %t177
  %t244 = load i64, ptr %t185
  %t245 = call i64 @"Main.fingerprint"(i64 %t244)
  %t246 = call i64 @"Main.mix"(i64 %t243, i64 %t245)
  %t247 = load i64, ptr %t185
  store i64 %t235, ptr %t0
  store i64 %t236, ptr %t1
  store i64 %t237, ptr %t2
  store i64 %t239, ptr %t4
  store i64 %t242, ptr %t6
  store i64 %t246, ptr %t7
  store i64 %t247, ptr %t8
  br label %bb0
bb26:
  %t248 = load i64, ptr %t56
  %t249 = load i64, ptr %t177
  %t250 = load i64, ptr %t15
  %t251 = insertvalue { i64, i64, i64 } poison, i64 %t248, 0
  %t252 = insertvalue { i64, i64, i64 } %t251, i64 %t249, 1
  %t253 = insertvalue { i64, i64, i64 } %t252, i64 %t250, 2
  ret { i64, i64, i64 } %t253
bb2:
  %t254 = load i64, ptr %t6
  %t255 = load i64, ptr %t7
  %t256 = load i64, ptr %t8
  %t257 = insertvalue { i64, i64, i64 } poison, i64 %t254, 0
  %t258 = insertvalue { i64, i64, i64 } %t257, i64 %t255, 1
  %t259 = insertvalue { i64, i64, i64 } %t258, i64 %t256, 2
  ret { i64, i64, i64 } %t259
}
