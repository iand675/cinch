//! End-to-end tests: compile real programs through the full pipeline
//! (lex, layout, parse, check, LLVM IR, clang link) and assert exit codes.

use cinch::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

#[test]
fn arithmetic_through_class_methods() {
    expect_ok(
        "arith",
        r#"
module Main where
  main :: Int
  main = if 2 + 3 * 4 == 14 then 0 else 1
"#,
        0,
    );
}

#[test]
fn custom_class_custom_operator_custom_instance() {
    expect_ok(
        "custom-class",
        r#"
module Main where
  signature Box a where
    unbox :: a -> Int

  infixl 5 |+|

  (|+|) :: Int -> Int -> Int
  (|+|) x y = x + y

  data Wrap where
    W :: Int -> Wrap

  implicit instance Box_Wrap :: Box Wrap where
    unbox w = case w of
      W n -> n

  twice :: forall a. using (b :: Box a) => a -> Int
  twice w = unbox w + unbox w

  main :: Int
  main = if twice (W 3) |+| (1 |+| 1) == 8 then 0 else 1
"#,
        0,
    );
}

#[test]
fn generic_constraint_and_partial_application() {
    expect_ok(
        "partial",
        r#"
module Main where
  addN :: forall a. using (add :: Add a) => a -> a -> a
  addN x y = x + y

  applyTwice :: (Int -> Int) -> Int -> Int
  applyTwice f x = f (f x)

  inc :: Int -> Int
  inc = addN 1

  main :: Int
  main = if applyTwice inc 0 == 2 then 0 else 1
"#,
        0,
    );
}

#[test]
fn constructor_patterns_in_equations() {
    expect_ok(
        "match",
        r#"
module Main where
  data Tree where
    Leaf :: Int -> Tree
    Node :: Tree -> Tree -> Tree

  sumTree :: Tree -> Int
  sumTree (Leaf n) = n
  sumTree (Node l r) = sumTree l + sumTree r

  main :: Int
  main = if sumTree (Node (Leaf 1) (Node (Leaf 2) (Leaf 3))) == 6 then 0 else 1
"#,
        0,
    );
}

#[test]
fn lambdas_capture_and_prelude_folds() {
    expect_ok(
        "folds",
        r#"
module Main where
  addSquares :: List Int -> Int
  addSquares xs = foldr (\x acc -> x * x + acc) 0 xs

  scale :: Int -> List Int -> Int
  scale k xs = foldr (\x acc -> x * k + acc) 0 xs

  main :: Int
  main = if addSquares (Cons 1 (Cons 2 (Cons 3 Nil))) == 14 then
           (if scale 2 (Cons 1 (Cons 2 (Cons 3 Nil))) == 12 then 0 else 1)
         else 1
"#,
        0,
    );
}

#[test]
fn recursion_negation_and_bool_operators() {
    expect_ok(
        "fib",
        r#"
module Main where
  fib :: Int -> Int
  fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n - 1) + fib (n - 2)

  pick :: Bool -> Int
  pick b = case b of
    True -> 10
    False -> 20

  main :: Int
  main = if fib 10 == 55 && -(3) == -3 && pick (not False && True || False) == 10
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn prelude_sum_list_generic() {
    expect_ok(
        "sum",
        r#"
module Main where
  main :: Int
  main = if sum-list 0 (Cons 1 (Cons 2 (Cons 3 Nil))) == 6 then 0 else 1
"#,
        0,
    );
}

#[test]
fn where_clauses_in_equations() {
    expect_ok(
        "where",
        r#"
module Main where
  hyp :: Int -> Int -> Int
  hyp a b = square + triple
    where square = a * a
          triple = b * 3

  main :: Int
  main = if hyp 2 3 == 13 then 0 else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ rejections

#[test]
fn thunks_delay_and_force() {
    expect_ok(
        "thunk",
        r#"
module Main where
  expensive :: Int -> Int
  expensive x = x * x

  main :: Int
  main = if force (delay (\u -> expensive 6)) == 36 then 0 else 1
"#,
        0,
    );
}

#[test]
fn memo_bindings_demand_on_use() {
    // `~` cells allocate at scope entry, demand on first use, and the
    // group's `Eval` label discharges at scope end (so the nullary `main`
    // stays pure).
    expect_ok(
        "memo",
        r#"
module Main where
  main :: Int
  main = if y * 2 == 42 then 0 else 1
    where ~y = 20 + 1
"#,
        0,
    );
}

#[test]
fn memo_cells_with_signatures_and_shared_use() {
    expect_ok(
        "memo-shared",
        r#"
module Main where
  main :: Int
  main = if a + b + a == 44 then 0 else 1
    where a :: Int
          ~a = 2
          b :: Int
          ~b = 40
"#,
        0,
    );
}

#[test]
fn local_let_polymorphism() {
    expect_ok(
        "let-poly",
        r#"
module Main where
  main :: Int
  main = let ident = \x -> x in
           if ident True then (if ident 3 == 3 then 0 else 1) else 1
"#,
        0,
    );
}

#[test]
fn memo_group_black_hole_is_rejected() {
    expect_err(
        "blackhole",
        r#"
module Main where
  main :: Int
  main = x
    where ~x = x + 1
"#,
        "memo group",
    );
}

#[test]
fn memo_group_mutual_demand_is_rejected() {
    expect_err(
        "memo-cycle",
        r#"
module Main where
  main :: Int
  main = a
    where ~a = b
          ~b = a
"#,
        "memo group",
    );
}

#[test]
fn rejects_missing_instance() {
    expect_err(
        "no-instance",
        r#"
module Main where
  main :: Int
  main = if True + False == True then 0 else 1
"#,
        "no instance for Add Bool",
    );
}

#[test]
fn rejects_type_mismatch() {
    expect_err(
        "mismatch",
        r#"
module Main where
  f :: Int -> Int
  f x = True

  main :: Int
  main = 0
"#,
        "type mismatch",
    );
}

#[test]
fn effect_rows_are_typed_and_honest() {
    // A function may declare an effect row; the row is checked against the
    // body. Here the body is pure and the declaration is open (`| e`), so
    // it checks, and the unused function is fine.
    expect_ok(
        "eff-rows",
        r#"
module Main where
  f :: forall e. Int ->{IO | e} Int
  f x = x

  main :: Int
  main = 0
"#,
        0,
    );
    // Performing an effect in a nullary (pure) definition is rejected: a
    // definition without an arrow has no row to carry `IO`.
    expect_err(
        "hidden-effect",
        r#"
module Main where
  f :: Int ->{IO} Int
  f x = x

  main :: Int
  main = f 1
"#,
        "must be pure",
    );
    // Declaring pure while performing an effect is rejected too.
    expect_err(
        "undeclared-effect",
        r#"
module Main where
  f :: Int ->{IO} Int
  f x = x

  g :: Int -> Int
  g x = f x

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
}

#[test]
fn rejects_non_exhaustive_match() {
    expect_err(
        "coverage",
        r#"
module Main where
  data Color where
    Red :: Color
    Green :: Color

  go :: Color -> Int
  go c = case c of
    Red -> 1

  main :: Int
  main = go Red
"#,
        "non-exhaustive",
    );
}

#[test]
fn rejects_unknown_name() {
    expect_err(
        "unknown",
        r#"
module Main where
  main :: Int
  main = if missingName == 1 then 0 else 1
"#,
        "unknown name",
    );
}

#[test]
fn rejects_wrong_main_type() {
    expect_err(
        "main-type",
        r#"
module Main where
  main :: Bool
  main = True
"#,
        "main must have type Int",
    );
}

#[test]
fn rejects_undeclared_operator() {
    expect_err(
        "undeclared-op",
        r#"
module Main where
  main :: Int
  main = if 1 <> 2 then 0 else 1
"#,
        "undeclared operator",
    );
}

// ------------------------------------------- P1 surface data completeness

#[test]
fn tuples_through_functions() {
    expect_ok(
        "tuples",
        r#"
module Main where
  swap :: (Int, Bool) -> (Bool, Int)
  swap p = case p of
    (a, b) -> (b, a)

  fst3 :: (Int, Int, Int) -> Int
  fst3 (a, b, c) = a

  snd3 :: (Int, Int, Int) -> Int
  snd3 (a, b, c) = b

  sumPair :: (Int, Int) -> Int
  sumPair (a, b) = a + b

  nested :: ((Int, Int), Int) -> Int
  nested t = case t of
    ((a, b), c) -> a * 100 + b * 10 + c

  main :: Int
  main = case swap (7, True) of
    (t, n) ->
      if t && n == 7 && fst3 (1, 2, 3) == 1 && snd3 (1, 2, 3) == 2
         && sumPair (3, 4) == 7 && nested ((1, 2), 3) == 123
      then 0 else 1
"#,
        0,
    );
}

#[test]
fn tuple_let_destructuring_and_types() {
    expect_ok(
        "tuple-let",
        r#"
module Main where
  point :: (Int, (Int, Int))
  point = (1, (2, 3))

  main :: Int
  main = let (a, (b, c)) = point in
           let p = (a + b, c) :: (Int, Int) in
             case p of
               (x, y) -> x * 10 + y
"#,
        33,
    );
}

#[test]
fn list_literals_and_list_patterns() {
    expect_ok(
        "lists",
        r#"
module Main where
  describe :: List Int -> Int
  describe xs = case xs of
    [a, b] -> a + b
    [a] -> a
    [] -> 0
    Cons _ _ -> 99

  total :: List (List Int) -> Int
  total xss = case xss of
    [] -> 0
    [ys] -> sum-list 0 ys
    [ys, zs] -> sum-list 0 ys + 10 * sum-list 0 zs
    Cons _ _ -> -1

  main :: Int
  main = if describe [1, 2] == 3 && describe [5] == 5 && describe [] == 0
            && describe [1, 2, 3] == 99
            && total [[1, 2], [3, 4]] == 73 && total [[5]] == 5 && total [] == 0
            && sum-list 0 [1, 2, 3, 4] == 10
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn literal_patterns_fall_through_in_source_order() {
    expect_ok(
        "literals",
        r#"
module Main where
  classify :: Int -> Int
  classify n = case n of
    0 -> 10
    0 -> 20
    1 -> 30
    n -> n

  negateIf :: Int -> Int
  negateIf n = case n of
    -3 -> 30
    3 -> 31
    n -> n

  inEquations :: Int -> Int
  inEquations 0 = 7
  inEquations 0 = 8
  inEquations n = n

  main :: Int
  main = if classify 0 == 10 && classify 1 == 30 && classify 5 == 5
            && negateIf (-3) == 30 && negateIf 3 == 31 && negateIf 0 == 0
            && inEquations 0 == 7 && inEquations 4 == 4
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn nested_constructor_patterns() {
    expect_ok(
        "nested",
        r#"
module Main where
  pick :: List (Option Int) -> Int
  pick xs = case xs of
    Cons (Some y) rest -> y
    Cons None rest -> pick rest
    Nil -> 0

  depth :: List (Option (List Int)) -> Int
  depth xs = case xs of
    Cons (Some [n]) rest -> n + depth rest
    Cons (Some []) rest -> 100 + depth rest
    Cons (Some _) rest -> 200 + depth rest
    Cons None rest -> depth rest
    Nil -> 0

  pairUp :: Option (Int, Int) -> Int
  pairUp o = case o of
    Some (a, b) -> a * 10 + b
    None -> 0

  main :: Int
  main = if pick (Cons None (Cons (Some 42) Nil)) == 42
            && depth (Cons (Some [1]) (Cons (Some []) (Cons (Some [2, 3]) (Cons None Nil)))) == 301
            && pairUp (Some (4, 5)) == 45 && pairUp None == 0
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn case_guards_fall_through_to_later_rows() {
    expect_ok(
        "guards",
        r#"
module Main where
  grade :: Int -> Int
  grade n = case n of
    x | x > 90 -> 1
    x | x > 80 -> 2
    x -> 3

  pickFirst :: Int -> Int -> Int
  pickFirst a b = case (a, b) of
    (x, y) | x > y -> x
    (x, y) | y > x -> y
    (x, y) -> 0

  onlyBig :: List Int -> Int
  onlyBig xs = case xs of
    Cons x rest | x > 10 -> x
    Cons x rest -> onlyBig rest
    Nil -> -1

  main :: Int
  main = if grade 95 == 1 && grade 85 == 2 && grade 10 == 3
            && pickFirst 3 4 == 4 && pickFirst 4 3 == 4 && pickFirst 2 2 == 0
            && onlyBig (Cons 1 (Cons 2 (Cons 55 Nil))) == 55
            && onlyBig (Cons 1 Nil) == -1
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn guard_failure_with_covered_patterns_is_rejected() {
    // A guarded row covers nothing: every applicable row can fail its
    // guard, so the match is rejected rather than faulting at runtime.
    expect_err(
        "guard-fault",
        r#"
module Main where
  f :: Int -> Int
  f n = case n of
    n | n > 10 -> 1

  main :: Int
  main = f 5
"#,
        "a scrutinee value is not covered by an unguarded row",
    );
}

#[test]
fn guard_failure_on_boxed_result_is_rejected() {
    // The review P0 repro (a boxed result behind an all-guarded match) no
    // longer reaches a runtime fallback at all.
    expect_err(
        "guard-fault-boxed",
        r#"
module Main where
  h :: Int -> Option Int
  h n = case n of
    n | n > 0 -> Some n

  main :: Int
  main = case h 0 of
    Some x -> x
    None -> 7
"#,
        "a scrutinee value is not covered by an unguarded row",
    );
}

#[test]
fn constructor_from_another_type_is_rejected() {
    // Review P0: a constructor of a different type used to be matched
    // against the same-position tag of the scrutinee's type.
    expect_err(
        "foreign-ctor",
        r#"
module Main where
  f :: Option Int -> Int
  f o = case o of
    Nil -> 5
    Some x -> x

  main :: Int
  main = f (Some 1)
"#,
        "does not belong to type",
    );
    expect_err(
        "foreign-ctor-arity",
        r#"
module Main where
  f :: Option Int -> Int
  f o = case o of
    Cons x xs -> x
    None -> 5

  main :: Int
  main = f (Some 1)
"#,
        "does not belong to type",
    );
}

#[test]
fn duplicate_binders_in_one_pattern_are_rejected() {
    expect_err(
        "dup-binders",
        r#"
module Main where
  main :: Int
  main = case (1, 2) of
    (x, x) -> x
"#,
        "bound twice",
    );
    expect_err(
        "dup-binders-as",
        r#"
module Main where
  main :: Int
  main = case Some 1 of
    x@(Some x) -> x
    None -> 0
"#,
        "bound twice",
    );
}

#[test]
fn integer_match_without_catch_all_is_rejected() {
    // Integer literals cannot cover every `Int`, so a literal match needs a
    // catch-all row; without one it is a compile error, never a fault.
    expect_err(
        "literal-fault",
        r#"
module Main where
  f :: Int -> Int
  f 0 = 1
  f 1 = 2

  main :: Int
  main = f 5
"#,
        "a value other than 0, 1 is not covered",
    );
    expect_err(
        "literal-fault-case",
        r#"
module Main where
  main :: Int
  main = case 5 of
    0 -> 1
    1 -> 2
"#,
        "a value other than 0, 1 is not covered",
    );
}

#[test]
fn records_construction_projection_and_patterns() {
    expect_ok(
        "records",
        r#"
module Main where
  record Point where
    x :: Int
    y :: Int

  record Box a where
    value :: a
    weight :: Int

  sumP :: Point -> Int
  sumP p = p.x + p.y

  flip :: Point -> Point
  flip p = case p of
    Point { y = b, x = a } -> Point { y = a, x = b }

  main :: Int
  main = case flip (Point { y = 2, x = 1 }) of
    Point { x = a, y = b } ->
      if sumP (Point { x = 3, y = 4 }) == 7 && a == 2 && b == 1
            && (Point { x = 5, y = 6 }).y == 6
      then
        case Box { weight = 9, value = 33 } of
          Box { value = l, weight = r } -> if l == 33 && r == 9 then 0 else 1
      else 1
"#,
        0,
    );
}

#[test]
fn record_rejections() {
    expect_err(
        "record-missing-field",
        r#"
module Main where
  record P where
    x :: Int
    y :: Int
  main :: Int
  main = case P { x = 1 } of
    _ -> 0
"#,
        "missing field",
    );
    expect_err(
        "record-unknown-field",
        r#"
module Main where
  record P where
    x :: Int
    y :: Int
  main :: Int
  main = case P { x = 1, y = 2, z = 3 } of
    _ -> 0
"#,
        "unknown record field",
    );
    expect_err(
        "record-pattern-missing-field",
        r#"
module Main where
  record P where
    x :: Int
    y :: Int
  main :: Int
  main = case P { x = 1, y = 2 } of
    P { x = a } -> a
"#,
        "missing field",
    );
    expect_err(
        "record-erased-field",
        r#"
module Main where
  record P where
    x :: Int
    erased y :: Int
  main :: Int
  main = 0
"#,
        "an `erased` field is evidence",
    );
}

#[test]
fn tuple_arity_mismatch_rejected() {
    expect_err(
        "tuple-pattern-arity",
        r#"
module Main where
  main :: Int
  main = case (1, 2) of
    (a, b, c) -> 0
"#,
        "tuple pattern has 3 components",
    );
    expect_err(
        "tuple-type-arity",
        r#"
module Main where
  f :: (Int, Int, Int) -> Int
  f (a, b) = a

  main :: Int
  main = 0
"#,
        "tuple pattern has 2 components",
    );
}

#[test]
fn nested_patterns_keep_source_order_fall_through() {
    expect_ok(
        "order",
        r#"
module Main where
  firstOf :: List (Option Int) -> Int
  firstOf xs = case xs of
    _ -> 1
    Cons (Some y) rest -> y

  overlap :: Option Int -> Int
  overlap o = case o of
    Some 0 -> 1
    Some n -> 2
    Some 0 -> 3
    None -> 4

  main :: Int
  main = if firstOf (Cons (Some 9) Nil) == 1
            && overlap (Some 0) == 1
            && overlap (Some 7) == 2
            && overlap None == 4
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn surface_and_or_short_circuit() {
    // Primitive types specification section 10: surface `&&`/`||` elaborate
    // to `if`, so the right operand is not evaluated when the result is
    // already determined. `int-div 1 0` faults at runtime (exit 72) if it
    // is ever forced, so these programs only exit 0 if it stays unevaluated.
    expect_ok(
        "and-short-circuit",
        "module Main where\n\
         main :: Int\n\
         main = if False && (int-div 1 0 == 0) then 1 else 0\n",
        0,
    );
    expect_ok(
        "or-short-circuit",
        "module Main where\n\
         main :: Int\n\
         main = if True || (int-div 1 0 == 0) then 0 else 1\n",
        0,
    );
    // When the result does depend on the right operand, it is evaluated.
    expect_ok(
        "and-or-evaluate-when-needed",
        "module Main where\n\
         main :: Int\n\
         main = if True && (int-div 4 2 == 2) then (if False || (int-div 6 3 == 2) then 0 else 1) else 2\n",
        0,
    );
    expect_ok(
        "and-false-propagates",
        "module Main where\n\
         main :: Int\n\
         main = if (True && (int-div 4 2 == 3)) && (int-div 1 0 == 0) then 1 else 0\n",
        0,
    );
}

#[test]
fn library_modules_check_without_an_entry_point() {
    // A module without `main` is a library: `check` accepts it (this is
    // how examples/source/06-counter-upgrade.cinch, a pure module, must
    // behave), while running it names the requirement.
    check_sources(
        "module Counter where\n\
         bump :: Nat -> Nat\n\
         bump n = n + 1\n",
        "<test>",
    )
    .expect("library module should check without main");
    let err = run_source(
        "module Counter where\n\
         bump :: Nat -> Nat\n\
         bump n = n + 1\n",
    )
    .expect_err("running a library module must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("main"),
        "error {msg:?} should name the missing `main` requirement"
    );
}

#[test]
fn wrong_main_type_still_rejected() {
    expect_err(
        "wrong-main-type",
        "module Main where\n\
         main :: Nat\n\
         main = 0\n",
        "main must have type Int",
    );
}
