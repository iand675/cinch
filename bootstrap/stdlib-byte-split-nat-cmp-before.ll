; Diagnostic excerpt from checked-RC IR SHA7ecc5fb3; not a standalone module.
; Original source and declaration are preserved, not corrected in this artifact.
declare i64 @cinch_nat_cmp(i64, i64) readnone nounwind willreturn

define i64 @"cinch.runtime.natCmp"(i64 %p0, i64 %p1) #0 {
entry:
  %t0 = and i64 %p0, %p1
  %t1 = and i64 %t0, 1
  %t2 = icmp ne i64 %t1, 0
  br i1 %t2, label %bb0, label %bb1
  bb0:
  %t3 = lshr i64 %p0, 1
  %t4 = lshr i64 %p1, 1
  %t5 = icmp ult i64 %t3, %t4
  %t6 = icmp ugt i64 %t3, %t4
  %t7 = select i1 %t5, i64 -1, i64 0
  %t8 = select i1 %t6, i64 1, i64 %t7
  br label %bb2
  bb1:
  %t9 = call i64 @cinch_nat_cmp(i64 %p0, i64 %p1)
  br label %bb2
  bb2:
  %t10 = phi i64 [ %t8, %bb0 ], [ %t9, %bb1 ]
  call void @cinch_drop(i64 %p0)
  ret i64 %t10
}

define range(i64 0, 2) i64 @"Data.Bytes.Split.progress-roundtrips"(i64 %p0, i64 %p1) #1 {
entry:
  %t0 = alloca i64
  %t1 = alloca i64
  %t16 = alloca i64
  %t19 = alloca i64
  %t24 = alloca i64
  %t29 = alloca i64
  %t32 = alloca i64
  %t35 = alloca i64
  %t36 = alloca i64
  %t39 = alloca i64
  %t42 = alloca i64
  %t50 = alloca i64
  %t51 = alloca i64
  %t54 = alloca i64
  %t57 = alloca i64
  %t65 = alloca i64
  store i64 %p0, ptr %t0
  store i64 %p1, ptr %t1
  br label %bb0
bb0:
  %t2 = load i64, ptr %t1
  %t3 = inttoptr i64 %t2 to ptr
  %t4 = getelementptr inbounds i8, ptr %t3, i64 -8
  %t5 = load atomic i64, ptr %t4 monotonic, align 8
  %t6 = lshr i64 %t5, 40
  %t7 = and i64 %t6, 255
  %t8 = icmp eq i64 %t7, 255
  br i1 %t8, label %bb2, label %bb3
bb2:
  %t9 = getelementptr inbounds i8, ptr %t3, i64 -16
  %t10 = load i64, ptr %t9
  %t11 = inttoptr i64 %t10 to ptr
  %t12 = load i64, ptr %t11
  br label %bb4
bb3:
  br label %bb4
bb4:
  %t13 = phi i64 [ %t12, %bb2 ], [ %t7, %bb3 ]
  switch i64 %t13, label %bb1 [
  i64 0, label %bb5
  i64 1, label %bb6
]
bb5:
  ret i64 1
bb6:
  %t14 = getelementptr inbounds i8, ptr %t3, i64 0
  %t15 = load i64, ptr %t14
  store i64 %t15, ptr %t16
  %t17 = getelementptr inbounds i8, ptr %t3, i64 8
  %t18 = load i64, ptr %t17
  store i64 %t18, ptr %t19
  %t20 = load i64, ptr %t0
  %t21 = load i64, ptr %t16
  call void @cinch_dup(i64 %t21)
  %t22 = load i64, ptr %t16
  %t23 = call i64 @"Data.Bytes.Split.store-count"(i64 %t20, i64 1, i64 %t22)
  store i64 %t23, ptr %t24
  %t25 = load i64, ptr %t0
  %t26 = load i64, ptr %t16
  call void @cinch_dup(i64 %t26)
  %t27 = load i64, ptr %t16
  %t28 = call i64 @"Data.Bytes.Split.store-count"(i64 %t25, i64 17, i64 %t27)
  store i64 %t28, ptr %t29
  %t30 = load i64, ptr %t0
  %t31 = call i64 @"Data.Bytes.Split.read-count"(i64 %t30, i64 1)
  store i64 %t31, ptr %t32
  %t33 = load i64, ptr %t0
  %t34 = call i64 @"Data.Bytes.Split.read-count"(i64 %t33, i64 17)
  store i64 %t34, ptr %t35
  %t37 = load i64, ptr %t0
  %t38 = call i64 @"cinch.runtime.bytesLen"(i64 %t37)
  store i64 %t38, ptr %t39
  %t40 = load i64, ptr %t39
  %t41 = call i64 @"cinch.runtime.natCmp"(i64 33, i64 %t40)
  store i64 %t41, ptr %t42
  %t43 = load i64, ptr %t39
  call void @cinch_drop(i64 %t43)
  %t44 = load i64, ptr %t42
  %t45 = call i64 @"cinch.prim.ltI64"(i64 %t44, i64 0), !range !163
  %t46 = icmp ne i64 %t45, 0
  br i1 %t46, label %bb8, label %bb9
bb8:
  %t47 = load i64, ptr %t0
  %t48 = call i64 @"cinch.runtime.bytesGet"(i64 %t47, i64 33)
  store i64 %t48, ptr %t36
  br label %bb7
bb9:
  store i64 1, ptr %t36
  br label %bb7
bb7:
  %t49 = load i64, ptr %t36
  store i64 %t49, ptr %t50
  %t52 = load i64, ptr %t0
  %t53 = call i64 @"cinch.runtime.bytesLen"(i64 %t52)
  store i64 %t53, ptr %t54
  %t55 = load i64, ptr %t54
  %t56 = call i64 @"cinch.runtime.natCmp"(i64 47, i64 %t55)
  store i64 %t56, ptr %t57
  %t58 = load i64, ptr %t54
  call void @cinch_drop(i64 %t58)
  %t59 = load i64, ptr %t57
  %t60 = call i64 @"cinch.prim.ltI64"(i64 %t59, i64 0), !range !164
  %t61 = icmp ne i64 %t60, 0
  br i1 %t61, label %bb11, label %bb12
bb11:
  %t62 = load i64, ptr %t0
  %t63 = call i64 @"cinch.runtime.bytesGet"(i64 %t62, i64 47)
  store i64 %t63, ptr %t51
  br label %bb10
bb12:
  store i64 3, ptr %t51
  br label %bb10
bb10:
  %t64 = load i64, ptr %t51
  store i64 %t64, ptr %t65
  %t66 = load i64, ptr %t32
  %t67 = load i64, ptr %t16
  %t68 = call i64 @"cinch.runtime.natCmp"(i64 %t66, i64 %t67)
  %t69 = call i64 @"cinch.prim.eqI64"(i64 %t68, i64 0), !range !165
  %t70 = icmp ne i64 %t69, 0
  br i1 %t70, label %bb13, label %bb14
bb13:
  %t71 = load i64, ptr %t35
  %t72 = load i64, ptr %t16
  %t73 = call i64 @"cinch.runtime.natCmp"(i64 %t71, i64 %t72)
  %t74 = call i64 @"cinch.prim.eqI64"(i64 %t73, i64 0), !range !166
  %t75 = icmp ne i64 %t74, 0
  br i1 %t75, label %bb15, label %bb16
bb15:
  %t76 = load i64, ptr %t50
  %t77 = call i64 @"cinch.runtime.natCmp"(i64 %t76, i64 347)
  %t78 = call i64 @"cinch.prim.eqI64"(i64 %t77, i64 0), !range !167
  %t79 = icmp ne i64 %t78, 0
  br i1 %t79, label %bb17, label %bb18
bb17:
  %t80 = load i64, ptr %t65
  %t81 = call i64 @"cinch.runtime.natCmp"(i64 %t80, i64 1)
  %t82 = call i64 @"cinch.prim.eqI64"(i64 %t81, i64 0), !range !168
  %t83 = icmp ne i64 %t82, 0
  br i1 %t83, label %bb19, label %bb20
bb19:
  %t84 = load i64, ptr %t0
  %t85 = load i64, ptr %t19
  store i64 %t84, ptr %t0
  store i64 %t85, ptr %t1
  br label %bb0
bb20:
  ret i64 0
bb18:
  %t86 = load i64, ptr %t65
  call void @cinch_drop(i64 %t86)
  ret i64 0
bb16:
  %t87 = load i64, ptr %t50
  call void @cinch_drop(i64 %t87)
  %t88 = load i64, ptr %t65
  call void @cinch_drop(i64 %t88)
  ret i64 0
bb14:
  %t89 = load i64, ptr %t35
  call void @cinch_drop(i64 %t89)
  %t90 = load i64, ptr %t50
  call void @cinch_drop(i64 %t90)
  %t91 = load i64, ptr %t65
  call void @cinch_drop(i64 %t91)
  ret i64 0
bb1:
  ret i64 0
}
