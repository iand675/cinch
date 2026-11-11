//! Dependent core (P7) acceptance: universes and Pi/Sigma with conversion
//! checking, implicit arguments, indexed families with index-aware
//! coverage, `Prop`/erasure with proof irrelevance, `decide` with
//! obligation statuses, totality of the logical fragment, and the
//! positivity discipline (roadmap gap A1).

mod common;

use common::source_frontend::{check_sources, run_source};

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

// ------------------------------------------------------------ universes, Pi/Sigma

#[test]
fn dependent_function_applied_at_a_computed_type() {
    // `bump`'s result type is `Tagged (n + 1)`, computed at the call site:
    // `bump 2` builds a `Tagged 3`, and the implicit index of `readTag`
    // is solved from the argument's type and passed at runtime.
    expect_ok(
        "computed-type",
        r#"
module Main where
  data Tagged (n :: Nat) where
    Tag :: Nat -> Tagged n

  bump :: (n :: Nat) -> Tagged n -> Tagged (n + 1)
  bump n t = Tag 10

  readTag :: forall (n :: Nat). Tagged n -> Nat
  readTag t = case t of
    Tag k -> k

  main :: Int
  main = if readTag (bump 2 (Tag 7)) == 10 then 0 else 1
"#,
        0,
    );
}

#[test]
fn pi_sigma_construction_and_elimination() {
    // Pi: `safeAt`/`finVal` take explicit dependent parameters; Sigma:
    // `pack` constructs `(n, xs)` and `unpack` eliminates it, retaining
    // the witness `m` (core specification section 3).
    expect_ok(
        "pi-sigma",
        r#"
module Main where
  safeAt :: (n :: Nat) -> Nat -> Option (Fin n)
  safeAt n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  finVal :: (n :: Nat) -> Fin n -> Nat
  finVal n f = case f of
    Fin i b -> i

  pack :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
  pack xs = (n, xs)

  unpack :: (Sigma (m :: Nat). Vec m Int) -> Nat
  unpack p = case p of
    (m, v) -> m

  main :: Int
  main = case safeAt 3 1 of
    Some f -> if finVal 3 f == 1 && unpack (pack (Cons 7 Nil)) == 1 then 0 else 1
    None -> 2
"#,
        0,
    );
}

#[test]
fn universe_stratification_rejects_type_in_type() {
    // `Type` has kind `Type 1`, so it cannot fill a `Type 0` parameter:
    // `Type : Type` is never admitted (core specification section 2).
    expect_err(
        "type-in-type",
        r#"
module Main where
  data Box (a :: Type) where
    Box :: a -> Box a

  f :: Box Type -> Nat
  f b = 0

  main :: Int
  main = 0
"#,
        "Type : Type",
    );
}

#[test]
fn conversion_on_definitionally_equal_indices() {
    // `three` has the computed index `2 + 1` and is used at `Vec 3`;
    // conversion reduces the arithmetic on numerals (design section 4).
    expect_ok(
        "conversion",
        r#"
module Main where
  three :: Vec (2 + 1) Int
  three = Cons 1 (Cons 2 (Cons 3 Nil))

  sum3 :: Vec 3 Int -> Int
  sum3 v = case v of
    Cons x xs -> x + sum2 xs

  sum2 :: Vec 2 Int -> Int
  sum2 v = case v of
    Cons x xs -> x + sum1 xs

  sum1 :: Vec 1 Int -> Int
  sum1 v = case v of
    Cons x xs -> x

  main :: Int
  main = if sum3 three == 6 then 0 else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ indexed families

#[test]
fn index_refuted_constructor_needs_no_branch() {
    // `Vec 0` has no `Cons`: matching `Nil` alone is exhaustive because
    // the indices refute the other constructor (core specification
    // section 2: coverage accounts for indices and impossible
    // constructors).
    expect_ok(
        "refuted-coverage",
        r#"
module Main where
  onlyNil :: Vec 0 Int -> Int
  onlyNil v = case v of
    Nil -> 5

  main :: Int
  main = if onlyNil Nil == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn index_refuted_arm_is_rejected() {
    // An arm for a constructor the indices refute can never match.
    expect_err(
        "refuted-arm",
        r#"
module Main where
  bad :: Vec 0 Int -> Int
  bad v = case v of
    Nil -> 5
    Cons x xs -> x

  main :: Int
  main = 0
"#,
        "impossible arm",
    );
}

#[test]
fn fin_construction_bounds_are_enforced() {
    // Fabricating `Fin 7` for `Fin 3` is an out-of-bounds construction:
    // the bound obligation `7 < 3` is statically disproved, and a failed
    // check must not produce evidence (core specification section 3).
    expect_err(
        "fin-bounds",
        r#"
module Main where
  badIndex :: Fin 3
  badIndex = Fin 7 refl

  main :: Int
  main = 0
"#,
        "obligation disproved",
    );
    // The catalogue's checked construction: the bound evidence comes from
    // the successful branch of a static decision.
    expect_ok(
        "fin-bounds-ok",
        r#"
module Main where
  mk :: Option (Fin 3)
  mk = case decide (2 < 3) of
    Yes bound -> Some (Fin 2 bound)
    No _ -> None

  take :: Option (Fin 3) -> Nat
  take o = case o of
    Some f -> case f of
      Fin i b -> i
    None -> 9

  main :: Int
  main = if take mk == 2 then 0 else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ Prop and erasure

#[test]
fn erased_fields_are_absent_from_runtime() {
    // The erased field is declared first: if its evidence leaked into the
    // block, `t.val` would read the wrong word. The field is present in
    // the type and absent from the runtime representation.
    expect_ok(
        "erased-field",
        r#"
module Main where
  record Tagged where
    erased ok :: Eq 3 3
    val :: Nat

  get :: Tagged -> Nat
  get t = t.val

  main :: Int
  main = if get (Tagged { val = 4, ok = refl }) == 4 then 0 else 1
"#,
        0,
    );
}

#[test]
fn proof_irrelevance_makes_refls_interchangeable() {
    // Two proofs of one proposition are interchangeable at runtime: both
    // erase to nothing (core specification section 3).
    expect_ok(
        "proof-irrelevance",
        r#"
module Main where
  proof p1 :: Eq 3 3
  proof p1 = refl
  proof p2 :: Eq 3 3
  proof p2 = refl

  use :: Eq 3 3 -> Nat -> Nat
  use h x = x + 1

  main :: Int
  main = if use p1 1 == use p2 1 then 0 else 1
"#,
        0,
    );
}

#[test]
fn non_positive_data_is_rejected() {
    // The positivity discipline applies to ordinary `data` too (core
    // specification section 2; roadmap gap A1).
    expect_err(
        "negative-occurrence",
        r#"
module Main where
  data Bad where
    BadC :: (Bad -> Bad) -> Bad

  main :: Int
  main = 0
"#,
        "positivity",
    );
    expect_err(
        "nested-occurrence",
        r#"
module Main where
  data Bush where
    B :: List Bush -> Bush

  main :: Int
  main = 0
"#,
        "positivity",
    );
}

// ------------------------------------------------------------ mutual data groups

#[test]
fn mutual_data_two_declaration_cycle_runs() {
    // Two data declarations that name each other, written with `Stmt`
    // after `Expr`. The module's data declarations resolve as one block and
    // the cycle is strictly positive on both sides (core specification
    // section 2, mutual inductive groups). The occurrences are direct: a
    // group member nested inside another type constructor (`List Stmt`)
    // stays refused until positivity-preserving nesting is decided.
    expect_ok(
        "mutual-two",
        r#"
module Main where
  data Expr where
    Lit :: Nat -> Expr
    Block :: Stmt -> Expr

  data Stmt where
    Eval :: Expr -> Stmt
    Seq :: Stmt -> Stmt -> Stmt
    Skip :: Stmt

  count :: Expr -> Nat
  count e = case e of
    Lit n -> 1
    Block s -> countStmt s

  countStmt :: Stmt -> Nat
  countStmt s = case s of
    Eval e -> count e
    Seq a b -> countStmt a + countStmt b
    Skip -> 0

  main :: Int
  main = if count (Block (Seq (Eval (Lit 7)) Skip)) == 1 then 0 else 1
"#,
        0,
    );
}

#[test]
fn mutual_data_three_declaration_cycle_runs() {
    // A three-declaration cycle, written in the order `A`, `C`, `B` so the
    // middle declaration of the cycle refers both backwards and forwards.
    expect_ok(
        "mutual-three",
        r#"
module Main where
  data A where
    AHere :: Nat -> A
    AToB :: B -> A

  data C where
    CToA :: A -> C
    CEnd :: C

  data B where
    BToC :: C -> B

  depth :: A -> Nat
  depth a = case a of
    AHere n -> n
    AToB b -> depthB b

  depthB :: B -> Nat
  depthB b = case b of
    BToC c -> depthC c

  depthC :: C -> Nat
  depthC c = case c of
    CToA a -> depth a + 1
    CEnd -> 0

  main :: Int
  main = if depth (AToB (BToC (CToA (AHere 4)))) == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn mutual_data_cycle_through_a_record_runs() {
    // A record is a data declaration with one constructor, so it is a group
    // member like any other and the cycle `A -> Wrap -> A` is judged as one
    // block. The occurrence in `Wrap.inner` is the field type itself.
    expect_ok(
        "mutual-record",
        r#"
module Main where
  data A where
    AHere :: Nat -> A
    AToR :: Wrap -> A

  record Wrap where
    inner :: A

  depth :: A -> Nat
  depth a = case a of
    AHere n -> n
    AToR w -> depth w.inner + 1

  main :: Int
  main = if depth (AToR (Wrap { inner = AHere 4 })) == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn mutual_data_rejections_remain() {
    // Nothing widened: a negative occurrence inside a cycle, a group
    // occurrence nested inside another type constructor, a negative
    // occurrence in a record that is part of the cycle, and a cycle
    // through a type declaration that is not a data declaration (this
    // surface has no type synonyms, so a refinement is the analogous
    // former) all stay refused.
    expect_err(
        "mutual-negative",
        r#"
module Main where
  data A where
    AHere :: A
    AToB :: (B -> A) -> A

  data B where
    BToA :: A -> B

  main :: Int
  main = 0
"#,
        "positivity",
    );
    expect_err(
        "mutual-nested",
        r#"
module Main where
  data A where
    AHere :: A
    AToB :: List B -> A

  data B where
    BToA :: A -> B

  main :: Int
  main = 0
"#,
        "positivity",
    );
    expect_err(
        "mutual-record-negative",
        r#"
module Main where
  data A where
    AHere :: A
    AToR :: Wrap -> A

  record Wrap where
    f :: A -> A

  main :: Int
  main = 0
"#,
        "positivity",
    );
    expect_err(
        "mutual-through-refinement",
        r#"
module Main where
  data A where
    AHere :: A
    AToR :: R -> A

  type R = refine (x :: A) with x == x

  main :: Int
  main = 0
"#,
        "unknown type R",
    );
}

// ------------------------------------------------------------ acceptance probe 01

#[test]
fn mutual_data_accepts_a_forward_reference_without_a_cycle() {
    // A declaration order that is not a cycle: `P` names `Q` written
    // later, and `Q` names `P` back, which is a cycle of two. `R` names
    // `Q` and nothing names `R`, so `R`'s group is itself.
    expect_ok(
        "forward-no-cycle",
        r#"
module Main where
  data R where
    RHere :: Nat -> R
    RQ :: Q -> R

  data P where
    PHere :: Nat -> P
    PQ :: Q -> P

  data Q where
    QNil :: Q
    QCons :: P -> Q

  size :: P -> Nat
  size p = case p of
    PHere n -> n
    PQ q -> sizeQ q

  sizeQ :: Q -> Nat
  sizeQ q = case q of
    QNil -> 0
    QCons p -> size p + 1

  main :: Int
  main = if size (PQ (QCons (PHere 2))) == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn acceptance_probe_01_bounded_index_runs() {
    // examples/source/01-bounded-index.cinch checks unchanged; this is the
    // same program with a `main` exercising it end to end (the example is
    // a library module, and `run` needs `main :: Int` per decision D20).
    expect_ok(
        "probe-01",
        r#"
module BoundedIndex where
  checkedIndex :: (n :: Nat) -> Nat -> Option (Fin n)
  checkedIndex n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  tryGet :: forall a n region. using (copy :: Copy a) =>
            Ref region (Vec n a) -> Nat -> Option a
  tryGet xs i = case checkedIndex n i of
    Some index -> Some (Vec.getCopy xs index)
    None -> None

  main :: Int
  main = case tryGet (borrow (Cons 10 (Cons 20 (Cons 30 Nil)))) 0 of
    Some a -> case tryGet (borrow (Cons 10 (Cons 20 (Cons 30 Nil)))) 2 of
      Some c -> case tryGet (borrow (Cons 10 (Cons 20 (Cons 30 Nil)))) 3 of
        Some _ -> 1
        None -> if a == 10 && c == 30 then 0 else 2
      None -> 3
    None -> 4
"#,
        0,
    );
}


#[test]
fn decide_success_and_runtime_branches() {
    // `decide` folds on numerals and compares at runtime otherwise; the
    // evidence is scoped to the examined values.
    expect_ok(
        "decide",
        r#"
module Main where
  runtime :: Nat -> Nat
  runtime x = case decide (x < 5) of
    Yes _ -> 1
    No _ -> 0

  staticYes :: Nat
  staticYes = case decide (2 < 3) of
    Yes _ -> 1
    No _ -> 0

  staticNo :: Nat
  staticNo = case decide (3 < 2) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = if runtime 3 == 1 && runtime 9 == 0 && staticYes == 1 && staticNo == 0
         then 0 else 1
"#,
        0,
    );
}

#[test]
fn decide_obligation_statuses_are_distinguished() {
    // Disproved: evidence for a false proposition cannot exist.
    expect_err(
        "status-disproved",
        r#"
module Main where
  proof lie :: 3 < 2
  proof lie = refl

  main :: Int
  main = 0
"#,
        "obligation disproved",
    );
    // Unsupported: no decision procedure applies.
    expect_err(
        "status-unsupported",
        r#"
module Main where
  f :: Nat -> Nat
  f x = case decide (Not (x < 5)) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
"#,
        "obligation status: unsupported",
    );
    // Timeout: static evaluation runs out of steps on a chain that cannot
    // fold to a numeral (every level mentions the parameter); that
    // outcome is not a proof either (core specification section 4). The
    // expression is deep, so the check runs on a large-stack thread.
    let mut expr = "x".to_string();
    for _ in 0..40 {
        expr = format!("x + (1 + ({expr}))");
    }
    let src = format!(
        "module Main where\n\
         \x20 f :: Nat -> Nat\n\
         \x20 f x = case decide ({expr} < 5) of\n\
         \x20   Yes _ -> 1\n\
         \x20   No _ -> 0\n\
         \n\
         \x20 main :: Int\n\
         \x20 main = 0\n"
    );
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || expect_err("status-timeout", &src, "obligation status: timeout"))
        .expect("spawn")
        .join()
        .expect("timeout case");
}

#[test]
fn logical_definitions_must_terminate() {
    // A structurally recursive `spec` definition checks...
    expect_ok(
        "total-spec",
        r#"
module Main where
  data Three where
    Z :: Three
    S :: Three -> Three

  spec size :: Three -> Nat
  size t = case t of
    Z -> 0
    S u -> 1 + size u

  main :: Int
  main = 0
"#,
        0,
    );
    // ... and one without a structural decrease is rejected: a looping
    // function cannot inhabit a proposition (core specification
    // sections 2 and 8).
    expect_err(
        "nonterminating-spec",
        r#"
module Main where
  data Three where
    Z :: Three
    S :: Three -> Three

  spec spin :: Three -> Nat
  spin t = spin t

  main :: Int
  main = 0
"#,
        "must terminate",
    );
    // Mutual recursion between logical definitions is a cycle, not a
    // termination argument.
    expect_err(
        "mutual-proofs",
        r#"
module Main where
  proof a :: Eq 3 3
  proof a = b
  proof b :: Eq 3 3
  proof b = a

  main :: Int
  main = 0
"#,
        "mutually recursive",
    );
}
