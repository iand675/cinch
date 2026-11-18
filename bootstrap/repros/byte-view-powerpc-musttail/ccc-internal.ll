target triple = "powerpc64-unknown-linux-gnu"
define internal  i64 @callee(i64 %a0, i64 %a1, i64 %a2, i64 %a3, i64 %a4, i64 %a5, i64 %a6, i64 %a7, i64 %a8, i64 %a9) noinline {
 %r = add i64 %a0, %a9
 ret i64 %r
}
define  i64 @caller(i64 %a0, i64 %a1, i64 %a2, i64 %a3, i64 %a4, i64 %a5, i64 %a6, i64 %a7, i64 %a8, i64 %a9) noinline {
 %out = musttail call  i64 @callee(i64 %a0, i64 %a1, i64 %a2, i64 %a3, i64 %a4, i64 %a5, i64 %a6, i64 %a7, i64 %a8, i64 %a9)
 ret i64 %out
}
