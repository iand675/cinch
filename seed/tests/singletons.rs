//! Integration tests for first-class promoted functions (native compile-time
//! evaluator) and zero-cost runtime singletons (`Sing`, `Same`/`Refl`, `sing`,
//! `from-sing`, `with-sing`, `sing-eq`, direct pattern matching, and direct
//! singleton-lifted calls).

mod common;

use std::process::Command;

const SOURCES: &[(&str, &str)] = &[
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Array.Packed.cinch", include_str!("../prelude/Data/Array/Packed.cinch")),
    ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
    ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
    ("Data.Array.Compute.cinch", include_str!("../prelude/Data/Array/Compute.cinch")),
    ("Data.Array.ND.cinch", include_str!("../prelude/Data/Array/ND.cinch")),
    ("Data.Sing.cinch", include_str!("../prelude/Data/Sing.cinch")),
];

fn run_cinch(tag: &str, extra_modules: &[(&str, &str)], main_src: &str) {
    let mut sources = SOURCES.to_vec();
    sources.extend_from_slice(extra_modules);
    sources.push(("Main.cinch", main_src));
    let dir = common::project(tag, &sources);
    let exe = common::build(&dir, "Main.cinch", "program").expect("build program");
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "tag={tag}\nstdout:\n{stdout}\nstderr:\n{stderr}\ndir: {}",
        dir.display()
    );
    common::cleanup(&dir);
}

fn expect_build_error(
    tag: &str,
    extra_modules: &[(&str, &str)],
    main_src: &str,
    expected_substrings: &[&str],
) {
    let mut sources = SOURCES.to_vec();
    sources.extend_from_slice(extra_modules);
    sources.push(("Main.cinch", main_src));
    let dir = common::project(tag, &sources);
    let res = common::build(&dir, "Main.cinch", "program");
    common::cleanup(&dir);
    let err = match res {
        Ok(exe) => panic!(
            "tag={tag}: expected compile error, but succeeded producing {}",
            exe.display()
        ),
        Err(e) => e,
    };
    for sub in expected_substrings {
        assert!(
            err.to_lowercase().contains(&sub.to_lowercase()),
            "tag={tag}: expected error containing `{sub}`, got:\n{err}"
        );
    }
}

#[test]
fn promoted_first_order_and_recursive_functions() {
    run_cinch(
        "sing_promoted_first_order",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Data.Core
  import Data.Nat
  import Data.Sing
  import Data.String

  -- Ordinary user data type + pure functions (no type family boilerplate!).
  data Layout where
    RowMajor :: Layout
    ColMajor :: Layout

  flip-layout :: Layout -> Layout
  flip-layout l = case l of
    RowMajor -> ColMajor
    ColMajor -> RowMajor

  -- Multi-equation recursive functions promoted automatically!
  fact :: Nat -> Nat
  fact 0 = 1
  fact n = n * fact (n - 1)

  add-nat :: Nat -> Nat -> Nat
  add-nat a b = a + b

  label-pair :: String -> String -> String
  label-pair a b = string-concat a b

  data Tagged (l :: Layout) (name :: String) where
    MkTagged :: Int -> Tagged l name

  -- Verify promoted functions work directly in types AND `using` obligations!
  check-facts ::
    using (fact 5 == 120, dims-product (dims-append [2, 3] [4]) == 24, dims-rank [10, 20, 30] == 3) =>
    Int -> Int
  check-facts x = x

  -- Symbolic arithmetic promotion: `add-nat m 0` reduces to `m` and `add-nat m 1` reduces to `m + 1`!
  check-sym-add :: forall (m :: Nat). using (add-nat m 0 == m, add-nat m 1 == m + 1) => Sing m -> Nat
  check-sym-add s = from-sing s

  -- Symbolic partial list evaluation: `dims-append [2, 3] rest` reduces to `Cons 2 (Cons 3 rest)`!
  partial-append :: forall (rest :: List Nat).
    NDView (dims-append [2, 3] rest) F32 -> NDView (Cons 2 (Cons 3 rest)) F32
  partial-append v = v

  -- Promoted ADT & String functions in types!
  flip-tagged :: Tagged RowMajor (label-pair "img-" "batch") -> Tagged (flip-layout ColMajor) "img-batch"
  flip-tagged t = t

  -- Promoted shape algebra from `Data.Sing` directly indexing `NDView`!
  transpose-eq ::
    NDView (dims-transpose2 [2, 3, 4]) F32 -> NDView (dims-reverse [4, 2, 3]) F32
  transpose-eq v = v

  main :: Int
  main =
    let t = flip-tagged (MkTagged 7)
        v = case t of
              MkTagged n -> n
        r1 = fact 5
        r2 = dims-product (dims-append [2, 3] [4])
        r3 = flip-layout RowMajor
        ok-l = case r3 of
          ColMajor -> True
          RowMajor -> False
    in if v == 7 && r1 == 120 && r2 == 24 && ok-l && check-sym-add (sing :: Sing 5) == 5
       then check-facts 0
       else 1
"#,
    );
}

#[test]
fn promoted_higher_order_and_polymorphic_functions() {
    run_cinch(
        "sing_promoted_higher_order",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Data.Core
  import Data.List
  import Data.Nat
  import Data.Sing

  double :: Nat -> Nat
  double n = n * 2

  add-n :: Nat -> Nat -> Nat
  add-n k n = k + n

  map-nat :: (Nat -> Nat) -> List Nat -> List Nat
  map-nat f xs = case xs of
    Nil -> Nil
    Cons h t -> Cons (f h) (map-nat f t)

  foldl-nat :: (Nat -> Nat -> Nat) -> Nat -> List Nat -> Nat
  foldl-nat f acc xs = case xs of
    Nil -> acc
    Cons h t -> foldl-nat f (f acc h) t

  -- 1. Higher-order promoted function with named function (`map-nat double [2, 3, 4] == [4, 6, 8]`)
  ho-named :: NDView (map-nat double [2, 3, 4]) F32 -> NDView [4, 6, 8] F32
  ho-named v = v

  -- 2. Higher-order promoted function with inline type-level lambda (`\x -> x * 3`) — ZERO defunctionalization!
  ho-lam :: NDView (map-nat (\x -> x * 3) [1, 2, 3]) F32 -> NDView [3, 6, 9] F32
  ho-lam v = v

  -- 3. Higher-order promoted function with partial application (`add-n 10`)!
  ho-partial :: NDView (map-nat (add-n 10) [1, 2, 3]) F32 -> NDView [11, 12, 13] F32
  ho-partial v = v

  -- 4. Higher-order fold in `using` proposition (`foldl-nat add-n 0 [10, 20, 30] == 60`)!
  ho-fold :: using (foldl-nat add-n 0 [10, 20, 30] == 60) => Int -> Int
  ho-fold x = x

  -- 5. Polymorphic `Data.List.reverse` & `Data.List.append` working on BOTH `[Nat]` AND `[(Nat, Type)]`!
  poly-ndarray ::
    NDArray (reverse [(10, Int32), (20, Int32), (30, Int32)]) ->
    NDArray [(30, Int32), (20, Int32), (10, Int32)]
  poly-ndarray a = a

  poly-append-nd ::
    NDArray (list-append [(2, Int32)] [(3, Int32), (4, Int32)]) ->
    NDArray [(2, Int32), (3, Int32), (4, Int32)]
  poly-append-nd a = a

  -- 6. Builtin prelude `Data.List.map` + `Data.List.length`!
  prelude-ho :: using (length (map double [2, 3, 4]) == 3) =>
    NDView (map double [2, 3, 4]) F32 -> NDView [4, 6, 8] F32
  prelude-ho v = v

  main :: Int
  main =
    let xs = map-nat double [2, 3, 4]
        s  = foldl-nat add-n 0 [10, 20, 30]
    in if dims-product xs == 192 && s == 60
       then ho-fold 0
       else 1
"#,
    );
}

#[test]
fn zero_cost_sing_reflection_and_pattern_matching() {
    run_cinch(
        "sing_zero_cost_match",
        &[],
        r#"module Main where
  import Data.Core
  import Data.Nat
  import Data.Sing
  import Data.String

  data Layout where
    RowMajor :: Layout
    ColMajor :: Layout

  -- Type family indexed by `Layout` to test branch refinement on `Sing l`!
  type family LayoutTag (l :: Layout) :: Type where
    LayoutTag RowMajor = Int
    LayoutTag ColMajor = String

  -- Direct pattern matching on `Sing l` (NO `SLayout` GADT!): refines `l` in each branch!
  layout-default :: forall (l :: Layout). Sing l -> LayoutTag l
  layout-default s = case s of
    RowMajor -> 42
    ColMajor -> "col"

  -- Direct pattern matching on `Sing (Cons h t)`: impossible `Nil` arm is statically refuted!
  sing-head :: forall (h :: Nat) (t :: List Nat). Sing (Cons h t) -> Sing h
  sing-head s = case s of
    Cons x _ -> x

  -- Direct recursive pattern matching on `Sing (xs :: List Nat)` with zero wrapper overhead!
  sing-len :: forall (xs :: List Nat). Sing xs -> Nat
  sing-len s = case s of
    Nil -> 0
    Cons _ rest -> 1 + sing-len rest

  -- Direct pattern matching on `Sing (b :: Bool)` refining a type family!
  type family BoolChoice (b :: Bool) :: Type where
    BoolChoice True = Int
    BoolChoice False = String

  pick-by-bool :: forall (b :: Bool). Sing b -> BoolChoice b
  pick-by-bool sb = case sb of
    True -> 99
    False -> "no"

  -- Direct pattern matching on `Sing (p :: (Nat, Bool))`!
  sing-fst :: forall (a :: Nat) (b :: Bool). Sing (a, b) -> Sing a
  sing-fst sp = case sp of
    (sa, _) -> sa

  -- Synthesizing `sing` from in-scope `Sing` witnesses!
  prepend-ten :: forall (xs :: List Nat). Sing xs -> Sing (Cons 10 xs)
  prepend-ten s = sing

  main :: Int
  main =
    let s-row  = (sing :: Sing RowMajor)
        s-col  = (sing :: Sing ColMajor)
        v-row  = layout-default s-row
        v-col  = layout-default s-col
        s-sh   = (sing :: Sing (dims-append [2, 3] [4]))
        h-sh   = sing-head s-sh
        len-sh = sing-len s-sh
        s-ext  = prepend-ten s-sh
        b1     = pick-by-bool (sing :: Sing True)
        b2     = pick-by-bool (sing :: Sing False)
        p-fst  = sing-fst (sing :: Sing (7, True))
        s-str  = (sing :: Sing "batch")
    in if v-row == 42
          && v-col == "col"
          && from-sing h-sh == 2
          && len-sh == 3
          && from-sing (sing-head s-ext) == 10
          && b1 == 99
          && b2 == "no"
          && from-sing p-fst == 7
          && from-sing s-str == "batch"
       then 0
       else 1
"#,
    );
}

#[test]
fn zero_cost_sing_lifted_calls_with_sing_and_nd_view_eq() {
    run_cinch(
        "sing_lifted_calls_and_eq",
        &[],
        r#"module Main where
  import Data.Array.Compute
  import Data.Array.ND
  import Data.Core
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Sing

  data Layout where
    RowMajor :: Layout
    ColMajor :: Layout

  -- Direct singleton-lifted calls to promoted functions — NO `sDimsAppend` or `sDimsProduct`!
  combine-and-measure :: forall (s1 :: List Nat) (s2 :: List Nat).
    Sing s1 -> Sing s2 -> (Sing (dims-append s1 s2), Sing (dims-product (dims-append s1 s2)))
  combine-and-measure a b =
    let combined = dims-append a b
        total    = dims-product combined
    in (combined, total)

  -- Direct singleton-lifted arithmetic & constructors (`sa + sb`, `Cons sa rest`)!
  add-sings :: forall (m :: Nat) (n :: Nat). Sing m -> Sing n -> Sing (m + n)
  add-sings a b = a + b

  cons-sing :: forall (h :: Nat) (t :: List Nat). Sing h -> Sing t -> Sing (Cons h t)
  cons-sing h t = Cons h t

  same-sings :: forall (x :: List Nat). Sing x -> Sing x -> Bool
  same-sings a b = from-sing (dims-product a) == from-sing (dims-product b)

  -- Runtime dynamic shapes bridged into type-level shapes via zero-copy `with-sing` + `sing-eq` + `Refl`!
  -- Also validates `sing-eq` on user `data Layout` WITHOUT an `Eq` instance!
  check-dynamic :: List Nat -> List Nat -> Layout -> Layout -> Bool
  check-dynamic sh1 sh2 l1 l2 =
    with-sing sh1 (\s1 ->
      with-sing sh2 (\s2 ->
        let (comb, prod) = combine-and-measure s1 s2
            eq-sh = case sing-eq s1 s2 of
              Some Refl ->
                -- Inside `Some Refl`, the existential shapes of `s1` and `s2` are refined equal,
                -- so `dims-append s1 s2` and `dims-append s1 s1` unify at the SAME `Sing x`!
                same-sings (dims-append s1 s2) (dims-append s1 s1)
              None -> False
            eq-l = with-sing l1 (\sl1 ->
              with-sing l2 (\sl2 ->
                case sing-eq sl1 sl2 of
                  Some Refl -> True
                  None -> False))
        in eq-sh && eq-l && from-sing prod == 36 && dims-rank (from-sing comb) == 4))

  -- Use `sing-eq` + `Refl` to unify two `NDView` shapes and run a real `nd-dot-f32`!
  safe-dot :: forall (s1 :: List Nat) (s2 :: List Nat).
    Sing s1 -> Sing s2 -> NDView s1 F32 -> NDView s2 F32 -> Option F32
  safe-dot k1 k2 v1 v2 = case sing-eq k1 k2 of
    Some Refl -> Some (nd-dot-f32 v1 v2 FloatExactTree)
    None -> None

  main :: Int
  main =
    let ok-dyn = check-dynamic [2, 3] [2, 3] RowMajor RowMajor
        diff-l = check-dynamic [2, 3] [2, 3] RowMajor ColMajor
        sum-s  = add-sings (sing :: Sing 10) (sing :: Sing 32)
        list-s = cons-sing (sing :: Sing 5) (sing :: Sing [6, 7])
        w23    = SWCons 2 (SWCons 3 SWNil)
        v1     = nd-tabulate-f32 w23 (\i -> if i + 1 <= 1000000 then i64-to-f32 (nat-to-i64 (i + 1)) else 0.0)
        v2     = nd-tabulate-f32 w23 (\_ -> 2.0)
        dot-ok = case safe-dot (sing :: Sing [2, 3]) (sing :: Sing [2, 3]) v1 v2 of
          Some d -> saturating-f32-to-i64 d == 42
          None -> False
        w32    = SWCons 3 (SWCons 2 SWNil)
        v3     = nd-tabulate-f32 w32 (\_ -> 1.0)
        dot-no = case safe-dot (sing :: Sing [2, 3]) (sing :: Sing [3, 2]) v1 v3 of
          Some _ -> False
          None -> True
    in if ok-dyn && not diff-l && from-sing sum-s == 42 && dims-product (from-sing list-s) == 210 && dot-ok && dot-no
       then 0
       else 1
"#,
    );
}

#[test]
fn promoted_compile_time_sorting_and_arithmetic_performance() {
    let start = std::time::Instant::now();
    run_cinch(
        "sing_perf_sort",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Data.Core
  import Data.Nat
  import Data.Sing

  -- Insertion sort written ONCE as an ordinary Cinch function — runs natively at compile time!
  insert-nat :: Nat -> List Nat -> List Nat
  insert-nat x xs = case xs of
    Nil -> Cons x Nil
    Cons h t -> if x <= h then Cons x (Cons h t) else Cons h (insert-nat x t)

  sort-nat :: List Nat -> List Nat
  sort-nat xs = case xs of
    Nil -> Nil
    Cons h t -> insert-nat h (sort-nat t)

  -- Fast tail-recursive fibonacci at the type level!
  fib-go :: Nat -> Nat -> Nat -> Nat
  fib-go 0 a _ = a
  fib-go n a b = fib-go (n - 1) b (a + b)

  fib :: Nat -> Nat
  fib n = fib-go n 0 1

  -- Verify sorting a 10-element list and computing fib(30) = 832040 at compile time!
  check-sorted ::
    using (fib 30 == 832040, dims-product (sort-nat [5, 2, 4, 3]) == 120) =>
    NDView (sort-nat [9, 1, 8, 2, 7, 3, 6, 4, 5, 0]) F32 ->
    NDView [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] F32
  check-sorted v = v

  main :: Int
  main =
    let s = sort-nat [3, 1, 2]
    in if fib 30 == 832040 && dims-product s == 6
       then 0
       else 1
"#,
    );
    let elapsed = start.elapsed();
    // Entire compile + link + run should stay snappy (no exponential type-family blowup).
    assert!(
        elapsed.as_secs() < 60,
        "promoted sort + fib(30) took unexpectedly long: {:?}",
        elapsed
    );
}

#[test]
fn singletons_and_promotion_compile_time_rejections() {
    // 1. Impure IO function cannot be promoted to the type level.
    expect_build_error(
        "sing_err_impure_io",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Effect.Io

  impure-shape :: Unit -> {IO} List Nat
  impure-shape _ = [2, 3]

  bad :: NDView (impure-shape ()) F32 -> Int
  bad _ = 0

  main :: Int
  main = 0
"#,
        &["cannot promote", "IO"],
    );

    // 2. Foreign function cannot be promoted to the type level.
    expect_build_error(
        "sing_err_foreign",
        &[],
        r#"module Main where
  import Data.Nat
  import Effect.Io

  foreign import ccall "abs" c-abs :: I32 -> {IO} I32

  bad :: Sing (c-abs 5) -> Int
  bad _ = 0

  main :: Int
  main = 0
"#,
        &["cannot promote", "foreign"],
    );

    // 3. Existential singleton variable cannot escape `with-sing`!
    expect_build_error(
        "sing_err_skolem_escape",
        &[],
        r#"module Main where
  import Data.Nat

  bad :: Nat -> Int
  bad n =
    let leaked = with-sing n (\s -> s)
    in 0

  main :: Int
  main = 0
"#,
        &["escapes"],
    );

    // 4. Cannot form runtime `Sing` over `Type` or `List (Nat, Type)` (erased universe).
    expect_build_error(
        "sing_err_erased_type_kind",
        &[],
        r#"module Main where
  import Data.Array.ND

  bad :: Sing [(10, Int32)] -> Int
  bad _ = 0

  main :: Int
  main = 0
"#,
        &["erased"],
    );

    // 5. Impossible constructor arm on concrete `Sing [10, 20]` is rejected at compile time!
    expect_build_error(
        "sing_err_impossible_arm",
        &[],
        r#"module Main where
  import Data.Core
  import Data.Nat

  bad :: Sing [10, 20] -> Nat
  bad s = case s of
    Nil -> 0
    Cons h _ -> from-sing h

  main :: Int
  main = 0
"#,
        &["impossible arm"],
    );

    // 6. Calling `sing` for an unconstrained rigid variable with no `Sing` witness in scope is rejected.
    expect_build_error(
        "sing_err_no_witness",
        &[],
        r#"module Main where
  import Data.Nat

  bad :: forall (n :: Nat). Int -> Sing n
  bad _ = sing

  main :: Int
  main = 0
"#,
        &["witness"],
    );

    // 7. Mismatched promoted function result is caught at compile time!
    expect_build_error(
        "sing_err_shape_mismatch",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Data.Sing

  bad :: NDView (dims-append [2, 3] [4]) F32 -> NDView [2, 3, 5] F32
  bad v = v

  main :: Int
  main = 0
"#,
        &["mismatch"],
    );

    // 8. Non-terminating promoted function terminates cleanly via fuel bound (does not hang!).
    expect_build_error(
        "sing_err_nonterm_fuel",
        &[],
        r#"module Main where
  import Data.Array.ND
  import Data.Nat

  spin :: Nat -> List Nat
  spin n = spin (n + 1)

  bad :: NDView (spin 0) F32 -> NDView [1, 2] F32
  bad v = v

  main :: Int
  main = 0
"#,
        &["mismatch"],
    );
}
