define { i64, i64, i64 } @"Main.counter-loop#cc"(i32 noundef %p0, i32 noundef %p1, i64 noundef %p2, i64 noundef %p3, i64 %p4) #1 {
entry:
  %t0 = alloca i64
  %t2 = alloca i64
  %t4 = alloca i64
  %t5 = alloca i64
  %t6 = alloca i64
  %t13 = alloca i64
  %t16 = alloca i64
  %t20 = alloca i64
  %t23 = alloca i64
  %t24 = alloca i64
  %t25 = alloca i64
  %t29 = alloca i64
  %t36 = alloca i64
  %t43 = alloca i64
  %t50 = alloca i64
  %t53 = alloca i64
  %t57 = alloca i64
  %t60 = alloca i64
  %t61 = alloca i64
  %t62 = alloca i64
  %t66 = alloca i64
  %t73 = alloca i64
  %t80 = alloca i64
  %t93 = alloca i64
  %t96 = alloca i64
  %t97 = alloca i64
  %t98 = alloca i64
  %t102 = alloca i64
  %t109 = alloca i64
  %t1 = zext i32 %p0 to i64
  store i64 %t1, ptr %t0
  %t3 = zext i32 %p1 to i64
  store i64 %t3, ptr %t2
  store i64 %p2, ptr %t4
  store i64 %p3, ptr %t5
  store i64 %p4, ptr %t6
  br label %bb0
bb0:
  %t7 = load i64, ptr %t2
  %t8 = load i64, ptr %t0
  %t9 = call i64 @"Main.before"(i64 %t7, i64 %t8)
  %t10 = icmp ne i64 %t9, 0
  br i1 %t10, label %bb1, label %bb2
bb1:
  %t11 = load i64, ptr %t2
  %t12 = call i64 @"Main.bump"(i64 %t11)
  store i64 %t12, ptr %t13
  %t14 = load i64, ptr %t4
  %t15 = call i64 @"Main.add"(i64 %t14, i64 1)
  store i64 %t15, ptr %t16
  %t17 = load i64, ptr %t5
  %t18 = load i64, ptr %t2
  %t19 = call i64 @"cinch.prim.u32ToI64"(i64 %t18)
  store i64 %t19, ptr %t20
  %t21 = load i64, ptr %t20
  %t22 = call i64 @"cinch.runtime.natFromI64"(i64 %t21)
  store i64 %t22, ptr %t23
  store i64 -1, ptr %t25
  %t26 = load i64, ptr %t23
  %t27 = load i64, ptr %t25
  %t28 = call i64 @"cinch.runtime.natCmp"(i64 %t26, i64 %t27)
  store i64 %t28, ptr %t29
  %t30 = load i64, ptr %t25
  call void @cinch.rc.drop.tobj(i64 %t30)
  %t31 = load i64, ptr %t29
  %t32 = call i64 @"cinch.prim.leI64"(i64 %t31, i64 0), !range !116
  %t33 = icmp ne i64 %t32, 0
  br i1 %t33, label %bb4, label %bb5
bb4:
  %t34 = load i64, ptr %t23
  %t35 = call i64 @"cinch.runtime.natToI64"(i64 %t34)
  store i64 %t35, ptr %t36
  %t37 = load i64, ptr %t23
  call void @cinch.rc.drop.tobj(i64 %t37)
  %t38 = load i64, ptr %t36
  %t39 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t38)
  store i64 %t39, ptr %t24
  br label %bb3
bb5:
  %t40 = load i64, ptr %t23
  call void @cinch.rc.drop.tobj(i64 %t40)
  store i64 0, ptr %t24
  br label %bb3
bb3:
  %t41 = load i64, ptr %t24
  %t42 = call i64 @"Main.mix"(i64 %t17, i64 %t41)
  store i64 %t42, ptr %t43
  %t44 = load i64, ptr %t13
  %t45 = load i64, ptr %t0
  %t46 = call i64 @"Main.before"(i64 %t44, i64 %t45)
  %t47 = icmp ne i64 %t46, 0
  br i1 %t47, label %bb6, label %bb7
bb6:
  %t48 = load i64, ptr %t13
  %t49 = call i64 @"Main.bump"(i64 %t48)
  store i64 %t49, ptr %t50
  %t51 = load i64, ptr %t16
  %t52 = call i64 @"Main.add"(i64 %t51, i64 1)
  store i64 %t52, ptr %t53
  %t54 = load i64, ptr %t43
  %t55 = load i64, ptr %t13
  %t56 = call i64 @"cinch.prim.u32ToI64"(i64 %t55)
  store i64 %t56, ptr %t57
  %t58 = load i64, ptr %t57
  %t59 = call i64 @"cinch.runtime.natFromI64"(i64 %t58)
  store i64 %t59, ptr %t60
  store i64 -1, ptr %t62
  %t63 = load i64, ptr %t60
  %t64 = load i64, ptr %t62
  %t65 = call i64 @"cinch.runtime.natCmp"(i64 %t63, i64 %t64)
  store i64 %t65, ptr %t66
  %t67 = load i64, ptr %t62
  call void @cinch.rc.drop.tobj(i64 %t67)
  %t68 = load i64, ptr %t66
  %t69 = call i64 @"cinch.prim.leI64"(i64 %t68, i64 0), !range !117
  %t70 = icmp ne i64 %t69, 0
  br i1 %t70, label %bb9, label %bb10
bb9:
  %t71 = load i64, ptr %t60
  %t72 = call i64 @"cinch.runtime.natToI64"(i64 %t71)
  store i64 %t72, ptr %t73
  %t74 = load i64, ptr %t60
  call void @cinch.rc.drop.tobj(i64 %t74)
  %t75 = load i64, ptr %t73
  %t76 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t75)
  store i64 %t76, ptr %t61
  br label %bb8
bb10:
  %t77 = load i64, ptr %t60
  call void @cinch.rc.drop.tobj(i64 %t77)
  store i64 0, ptr %t61
  br label %bb8
bb8:
  %t78 = load i64, ptr %t61
  %t79 = call i64 @"Main.mix"(i64 %t54, i64 %t78)
  store i64 %t79, ptr %t80
  %t81 = load i64, ptr %t50
  %t82 = load i64, ptr %t0
  %t83 = call i64 @"Main.before"(i64 %t81, i64 %t82)
  %t84 = icmp ne i64 %t83, 0
  br i1 %t84, label %bb11, label %bb12
bb11:
  %t85 = load i64, ptr %t0
  %t86 = load i64, ptr %t50
  %t87 = call i64 @"Main.bump"(i64 %t86)
  %t88 = load i64, ptr %t53
  %t89 = call i64 @"Main.add"(i64 %t88, i64 1)
  %t90 = load i64, ptr %t80
  %t91 = load i64, ptr %t50
  %t92 = call i64 @"cinch.prim.u32ToI64"(i64 %t91)
  store i64 %t92, ptr %t93
  %t94 = load i64, ptr %t93
  %t95 = call i64 @"cinch.runtime.natFromI64"(i64 %t94)
  store i64 %t95, ptr %t96
  store i64 -1, ptr %t98
  %t99 = load i64, ptr %t96
  %t100 = load i64, ptr %t98
  %t101 = call i64 @"cinch.runtime.natCmp"(i64 %t99, i64 %t100)
  store i64 %t101, ptr %t102
  %t103 = load i64, ptr %t98
  call void @cinch.rc.drop.tobj(i64 %t103)
  %t104 = load i64, ptr %t102
  %t105 = call i64 @"cinch.prim.leI64"(i64 %t104, i64 0), !range !118
  %t106 = icmp ne i64 %t105, 0
  br i1 %t106, label %bb14, label %bb15
bb14:
  %t107 = load i64, ptr %t96
  %t108 = call i64 @"cinch.runtime.natToI64"(i64 %t107)
  store i64 %t108, ptr %t109
  %t110 = load i64, ptr %t96
  call void @cinch.rc.drop.tobj(i64 %t110)
  %t111 = load i64, ptr %t109
  %t112 = call i64 @"cinch.prim.wrappingI64ToU64"(i64 %t111)
  store i64 %t112, ptr %t97
  br label %bb13
bb15:
  %t113 = load i64, ptr %t96
  call void @cinch.rc.drop.tobj(i64 %t113)
  store i64 0, ptr %t97
  br label %bb13
bb13:
  %t114 = load i64, ptr %t97
  %t115 = call i64 @"Main.mix"(i64 %t90, i64 %t114)
  %t116 = load i64, ptr %t6
  store i64 %t85, ptr %t0
  store i64 %t87, ptr %t2
  store i64 %t89, ptr %t4
  store i64 %t115, ptr %t5
  store i64 %t116, ptr %t6
  br label %bb0
bb12:
  %t117 = load i64, ptr %t53
  %t118 = load i64, ptr %t80
  %t119 = load i64, ptr %t6
  %t120 = insertvalue { i64, i64, i64 } poison, i64 %t117, 0
  %t121 = insertvalue { i64, i64, i64 } %t120, i64 %t118, 1
  %t122 = insertvalue { i64, i64, i64 } %t121, i64 %t119, 2
  ret { i64, i64, i64 } %t122
bb7:
  %t123 = load i64, ptr %t16
  %t124 = load i64, ptr %t43
  %t125 = load i64, ptr %t6
  %t126 = insertvalue { i64, i64, i64 } poison, i64 %t123, 0
  %t127 = insertvalue { i64, i64, i64 } %t126, i64 %t124, 1
  %t128 = insertvalue { i64, i64, i64 } %t127, i64 %t125, 2
  ret { i64, i64, i64 } %t128
bb2:
  %t129 = load i64, ptr %t4
  %t130 = load i64, ptr %t5
  %t131 = load i64, ptr %t6
  %t132 = insertvalue { i64, i64, i64 } poison, i64 %t129, 0
  %t133 = insertvalue { i64, i64, i64 } %t132, i64 %t130, 1
  %t134 = insertvalue { i64, i64, i64 } %t133, i64 %t131, 2
  ret { i64, i64, i64 } %t134
}
