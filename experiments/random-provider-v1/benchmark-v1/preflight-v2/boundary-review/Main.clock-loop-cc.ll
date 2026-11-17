define { i64, i64, i64 } @"Main.clock-loop#cc"(i64 %p0, i32 noundef %p1, i32 noundef %p2, i64 noundef %p3, i64 noundef %p4, i64 %p5) #1 {
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
  %t22 = alloca i64
  %t24 = alloca i64
  %t25 = alloca i64
  %t29 = alloca i64
  %t36 = alloca i64
  %t43 = alloca i64
  %t49 = alloca i64
  %t52 = alloca i64
  %t55 = alloca i64
  %t58 = alloca i64
  %t60 = alloca i64
  %t61 = alloca i64
  %t65 = alloca i64
  %t72 = alloca i64
  %t79 = alloca i64
  %t85 = alloca i64
  %t88 = alloca i64
  %t96 = alloca i64
  %t97 = alloca i64
  %t101 = alloca i64
  %t108 = alloca i64
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
  %t12 = call i64 @"Effect.Clock.raw-wall-ns"(i64 0)
  store i64 %t12, ptr %t13
  %t14 = load i64, ptr %t13
  %t15 = call i64 @"cinch.runtime.natFromI64"(i64 %t14)
  store i64 %t15, ptr %t16
  %t17 = load i64, ptr %t3
  %t18 = call i64 @"Main.bump"(i64 %t17)
  store i64 %t18, ptr %t19
  %t20 = load i64, ptr %t5
  %t21 = call i64 @"Main.add"(i64 %t20, i64 1)
  store i64 %t21, ptr %t22
  %t23 = load i64, ptr %t6
  store i64 -1, ptr %t25
  %t26 = load i64, ptr %t16
  %t27 = load i64, ptr %t25
  %t28 = call i64 @"cinch.runtime.natCmp"(i64 %t26, i64 %t27)
  store i64 %t28, ptr %t29
  %t30 = load i64, ptr %t25
  call void @cinch.rc.drop.tobj(i64 %t30)
  %t31 = load i64, ptr %t29
  %t32 = call i64 @"cinch.prim.leI64"(i64 %t31, i64 0), !range !119
  %t33 = icmp ne i64 %t32, 0
  br i1 %t33, label %bb4, label %bb5
bb4:
  %t34 = load i64, ptr %t16
  %t35 = call i64 @"cinch.runtime.natToI64"(i64 %t34)
  store i64 %t35, ptr %t36
  %t37 = load i64, ptr %t16
  call void @cinch.rc.drop.tobj(i64 %t37)
  %t38 = load i64, ptr %t36
  %t39 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t38)
  store i64 %t39, ptr %t24
  br label %bb3
bb5:
  %t40 = load i64, ptr %t16
  call void @cinch.rc.drop.tobj(i64 %t40)
  store i64 0, ptr %t24
  br label %bb3
bb3:
  %t41 = load i64, ptr %t24
  %t42 = call i64 @"Main.mix"(i64 %t23, i64 %t41)
  store i64 %t42, ptr %t43
  %t44 = load i64, ptr %t19
  %t45 = load i64, ptr %t1
  %t46 = call i64 @"Main.before"(i64 %t44, i64 %t45)
  %t47 = icmp ne i64 %t46, 0
  br i1 %t47, label %bb6, label %bb7
bb6:
  %t48 = call i64 @"Effect.Clock.raw-wall-ns"(i64 0)
  store i64 %t48, ptr %t49
  %t50 = load i64, ptr %t49
  %t51 = call i64 @"cinch.runtime.natFromI64"(i64 %t50)
  store i64 %t51, ptr %t52
  %t53 = load i64, ptr %t19
  %t54 = call i64 @"Main.bump"(i64 %t53)
  store i64 %t54, ptr %t55
  %t56 = load i64, ptr %t22
  %t57 = call i64 @"Main.add"(i64 %t56, i64 1)
  store i64 %t57, ptr %t58
  %t59 = load i64, ptr %t43
  store i64 -1, ptr %t61
  %t62 = load i64, ptr %t52
  %t63 = load i64, ptr %t61
  %t64 = call i64 @"cinch.runtime.natCmp"(i64 %t62, i64 %t63)
  store i64 %t64, ptr %t65
  %t66 = load i64, ptr %t61
  call void @cinch.rc.drop.tobj(i64 %t66)
  %t67 = load i64, ptr %t65
  %t68 = call i64 @"cinch.prim.leI64"(i64 %t67, i64 0), !range !120
  %t69 = icmp ne i64 %t68, 0
  br i1 %t69, label %bb9, label %bb10
bb9:
  %t70 = load i64, ptr %t52
  %t71 = call i64 @"cinch.runtime.natToI64"(i64 %t70)
  store i64 %t71, ptr %t72
  %t73 = load i64, ptr %t52
  call void @cinch.rc.drop.tobj(i64 %t73)
  %t74 = load i64, ptr %t72
  %t75 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t74)
  store i64 %t75, ptr %t60
  br label %bb8
bb10:
  %t76 = load i64, ptr %t52
  call void @cinch.rc.drop.tobj(i64 %t76)
  store i64 0, ptr %t60
  br label %bb8
bb8:
  %t77 = load i64, ptr %t60
  %t78 = call i64 @"Main.mix"(i64 %t59, i64 %t77)
  store i64 %t78, ptr %t79
  %t80 = load i64, ptr %t55
  %t81 = load i64, ptr %t1
  %t82 = call i64 @"Main.before"(i64 %t80, i64 %t81)
  %t83 = icmp ne i64 %t82, 0
  br i1 %t83, label %bb11, label %bb12
bb11:
  %t84 = call i64 @"Effect.Clock.raw-wall-ns"(i64 0)
  store i64 %t84, ptr %t85
  %t86 = load i64, ptr %t85
  %t87 = call i64 @"cinch.runtime.natFromI64"(i64 %t86)
  store i64 %t87, ptr %t88
  %t89 = load i64, ptr %t0
  %t90 = load i64, ptr %t1
  %t91 = load i64, ptr %t55
  %t92 = call i64 @"Main.bump"(i64 %t91)
  %t93 = load i64, ptr %t58
  %t94 = call i64 @"Main.add"(i64 %t93, i64 1)
  %t95 = load i64, ptr %t79
  store i64 -1, ptr %t97
  %t98 = load i64, ptr %t88
  %t99 = load i64, ptr %t97
  %t100 = call i64 @"cinch.runtime.natCmp"(i64 %t98, i64 %t99)
  store i64 %t100, ptr %t101
  %t102 = load i64, ptr %t97
  call void @cinch.rc.drop.tobj(i64 %t102)
  %t103 = load i64, ptr %t101
  %t104 = call i64 @"cinch.prim.leI64"(i64 %t103, i64 0), !range !121
  %t105 = icmp ne i64 %t104, 0
  br i1 %t105, label %bb14, label %bb15
bb14:
  %t106 = load i64, ptr %t88
  %t107 = call i64 @"cinch.runtime.natToI64"(i64 %t106)
  store i64 %t107, ptr %t108
  %t109 = load i64, ptr %t88
  call void @cinch.rc.drop.tobj(i64 %t109)
  %t110 = load i64, ptr %t108
  %t111 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t110)
  store i64 %t111, ptr %t96
  br label %bb13
bb15:
  %t112 = load i64, ptr %t88
  call void @cinch.rc.drop.tobj(i64 %t112)
  store i64 0, ptr %t96
  br label %bb13
bb13:
  %t113 = load i64, ptr %t96
  %t114 = call i64 @"Main.mix"(i64 %t95, i64 %t113)
  %t115 = load i64, ptr %t7
  store i64 %t89, ptr %t0
  store i64 %t90, ptr %t1
  store i64 %t92, ptr %t3
  store i64 %t94, ptr %t5
  store i64 %t114, ptr %t6
  store i64 %t115, ptr %t7
  br label %bb0
bb12:
  %t116 = load i64, ptr %t58
  %t117 = load i64, ptr %t79
  %t118 = load i64, ptr %t7
  %t119 = insertvalue { i64, i64, i64 } poison, i64 %t116, 0
  %t120 = insertvalue { i64, i64, i64 } %t119, i64 %t117, 1
  %t121 = insertvalue { i64, i64, i64 } %t120, i64 %t118, 2
  ret { i64, i64, i64 } %t121
bb7:
  %t122 = load i64, ptr %t22
  %t123 = load i64, ptr %t43
  %t124 = load i64, ptr %t7
  %t125 = insertvalue { i64, i64, i64 } poison, i64 %t122, 0
  %t126 = insertvalue { i64, i64, i64 } %t125, i64 %t123, 1
  %t127 = insertvalue { i64, i64, i64 } %t126, i64 %t124, 2
  ret { i64, i64, i64 } %t127
bb2:
  %t128 = load i64, ptr %t5
  %t129 = load i64, ptr %t6
  %t130 = load i64, ptr %t7
  %t131 = insertvalue { i64, i64, i64 } poison, i64 %t128, 0
  %t132 = insertvalue { i64, i64, i64 } %t131, i64 %t129, 1
  %t133 = insertvalue { i64, i64, i64 } %t132, i64 %t130, 2
  ret { i64, i64, i64 } %t133
}
