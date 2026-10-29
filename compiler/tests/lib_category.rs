//! Category-family library tests: the real `Functor`/`Applicative`/
//! `Monad`/`Alternative` classes (compiler/prelude/Category/*.cinch)
//! with the shipped instance families (List, Option, Result e,
//! `((->) r)`), the `Identity` and `Const` universal visitors
//! (compiler/prelude/Data/Identity.cinch, Data/Const.cinch), and the law
//! groups the classes must satisfy, checked over sample values.
//!
//! Each test compiles a real multi-file project: the class modules,
//! the visitor modules, and a `Main` exercising them, driven through
//! the CLI pipeline like modules.rs and lib_foldable.rs. The law
//! statements are the polymorphic ones: quantified over the container
//! and decided through their `Eq` constraints, with `pure` stated
//! under an added `Applicative` constraint (decision D152) so a bare
//! `pure` reaches the class method through that constraint (decision
//! D153). The containers' `Eq` instances deliberately do not ship with
//! their types (D119 coherence: a shipped instance would be an
//! ambiguity with the locally declared ones below), so the tests
//! declare the ones they need. The law
//! groups:
//!   Functor:      identity and composition.
//!   Applicative:  homomorphism.
//!   Monad:        left and right identity, associativity in bind and
//!                 join form.
//!   Const:        the constant functor's identity, composition and
//!                 constant laws.
//! and the `((->) r)` family runs the same groups pointwise (there is
//! no `Eq` of functions).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const FUNCTOR: &str = include_str!("../prelude/Category/Functor.cinch");
const APPLICATIVE: &str = include_str!("../prelude/Category/Applicative.cinch");
const MONAD: &str = include_str!("../prelude/Category/Monad.cinch");
const ALTERNATIVE: &str = include_str!("../prelude/Category/Alternative.cinch");
const IDENTITY: &str = include_str!("../prelude/Data/Identity.cinch");
const CONST: &str = include_str!("../prelude/Data/Const.cinch");

/// The shared `Main` prefix: the imports, the structural equality
/// helpers and `Eq` instances the law statements need, and the law
/// statements themselves. `applicative-homomorphism` takes an `f b`
/// witness to pin the container at call sites: its other arguments
/// leave `f` undetermined, and dictionary resolution would be
/// ambiguous without it.
const PREFIX: &str = r#"module Main where
  import Category.Functor
  import Category.Applicative
  import Category.Monad
  import Category.Alternative
  import Data.Identity
  import Data.Const

  same-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool
  same-list a b = case a of
    Nil -> case b of
      Nil -> True
      Cons y ys -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then same-list xs ys else False

  same-option :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool
  same-option a b = case a of
    None -> case b of
      None -> True
      Some y -> False
    Some x -> case b of
      None -> False
      Some y -> x == y

  same-result :: forall e a. using (ee :: Eq e, ea :: Eq a) => Result e a -> Result e a -> Bool
  same-result a b = case a of
    Err p -> case b of
      Err q -> p == q
      Ok v -> False
    Ok x -> case b of
      Err q -> False
      Ok y -> x == y

  implicit instance Eq_List_T a :: using (ea :: Eq a) => Eq (List a) where
    (==) x y = same-list x y
    (/=) x y = if same-list x y then False else True

  implicit instance Eq_Option_T a :: using (ea :: Eq a) => Eq (Option a) where
    (==) x y = same-option x y
    (/=) x y = if same-option x y then False else True

  implicit instance Eq_Result_T e a :: using (ee :: Eq e, ea :: Eq a) => Eq (Result e a) where
    (==) x y = same-result x y
    (/=) x y = if same-result x y then False else True

  implicit instance Eq_Identity_T a :: using (ea :: Eq a) => Eq (Identity a) where
    (==) x y = run-identity x == run-identity y
    (/=) x y = if run-identity x == run-identity y then False else True

  implicit instance Eq_Const_T m a :: using (em :: Eq m) => Eq (Const m a) where
    (==) x y = get-const x == get-const y
    (/=) x y = if get-const x == get-const y then False else True

  functor-identity :: forall f a. using (fun :: Functor f, eq :: Eq (f a)) => f a -> Bool
  functor-identity v = map (\x -> x) v == v

  functor-composition :: forall f a b c. using (fun :: Functor f, eq :: Eq (f c)) => (b -> c) -> (a -> b) -> f a -> Bool
  functor-composition g f v = map g (map f v) == map (\x -> g (f x)) v

  applicative-homomorphism :: forall f a b. using (app :: Applicative f, eq :: Eq (f b)) => (a -> b) -> a -> f b -> Bool
  applicative-homomorphism f x w = pure (f x) == (<*>) (pure f) (pure x)

  monad-left-identity :: forall f a b. using (mon :: Monad f, app :: Applicative f, eq :: Eq (f b)) => a -> (a -> f b) -> Bool
  monad-left-identity x k = (>>=) (pure x) k == k x

  monad-right-identity :: forall f a. using (mon :: Monad f, app :: Applicative f, eq :: Eq (f a)) => f a -> Bool
  monad-right-identity v = (>>=) v pure == v

  monad-associativity :: forall f a b c. using (mon :: Monad f, eq :: Eq (f c)) => f a -> (a -> f b) -> (b -> f c) -> Bool
  monad-associativity v f g = (>>=) ((>>=) v f) g == (>>=) v (\x -> (>>=) (f x) g)

  monad-join-associativity :: forall f a. using (mon :: Monad f, fun :: Functor f, eq :: Eq (f a)) => f (f (f a)) -> Bool
  monad-join-associativity vvv = join (map join vvv) == join (join vvv)

  const-functor-identity :: forall m a. using (eq :: Eq m) => Const m a -> Bool
  const-functor-identity c = get-const (map (\x -> x) c) == get-const c

  const-functor-composition :: forall m a b r. using (eq :: Eq m) => (b -> r) -> (a -> b) -> Const m a -> Bool
  const-functor-composition g f c =
    let left = get-const (map g (map f c)) in
    let right = get-const (map (\x -> g (f x)) c) in
    left == right

  const-constant :: forall m a b. using (eq :: Eq m) => (a -> b) -> (a -> b) -> Const m a -> Bool
  const-constant f h c = get-const (map f c) == get-const (map h c)
"#;

fn compile(name: &str, main_src: &str, mode: Mode) -> Result<i32, String> {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_cattest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Functor.cinch", FUNCTOR),
        ("Applicative.cinch", APPLICATIVE),
        ("Monad.cinch", MONAD),
        ("Alternative.cinch", ALTERNATIVE),
        ("Identity.cinch", IDENTITY),
        ("Const.cinch", CONST),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    driver(mode, &dir.join("Main.cinch"), None).map_err(|e| e.to_string())
}

fn source(body: &str) -> String {
    format!("{PREFIX}\n{body}\n")
}

fn expect_ok(name: &str, body: &str, expected: i32) {
    match compile(name, &source(body), Mode::Run) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, body: &str, needle: &str) {
    match compile(name, &source(body), Mode::Check) {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}"),
        Err(e) => {
            assert!(
                e.contains(needle),
                "{name}: error {e:?} does not contain {needle:?}"
            );
        }
    }
}

#[test]
fn visitors_round_trip() {
    expect_ok(
        "visitors-round-trip",
        r#"  main :: Int
  main =
    let v = (Identity 7 :: Identity Int) in
    let w = Identity True in
    let c = (Const 11 :: Const Int Bool) in
    let s = (Const 'q' :: Const Char Int) in
    if run-identity v == 7 then
      if run-identity w == True then
        if get-const c == 11 then
          if get-const s == 'q' then 0 else 4
        else 3
      else 2
    else 1"#,
        0,
    );
}

#[test]
fn identity_functor_laws() {
    expect_ok(
        "identity-functor-laws",
        r#"  main :: Int
  main =
    let v = (Identity 7 :: Identity Int) in
    let w = Identity True in
    if functor-identity v then
      if functor-identity w then
        if functor-composition (\x -> x * 2) (\x -> x + 1) v then
          if functor-composition (\p -> if p then False else True) (\p -> if p then False else True) w then 0 else 4
        else 3
      else 2
    else 1"#,
        0,
    );
}

#[test]
fn identity_applicative_homomorphism() {
    expect_ok(
        "identity-applicative-homomorphism",
        r#"  main :: Int
  main =
    if applicative-homomorphism (\x -> x + 1) (5 :: Int) (Identity 0) then
      if applicative-homomorphism (\p -> if p then False else True) True (Identity False) then
        if run-identity ((<*>) (Identity (\x -> x + 1)) (Identity (5 :: Int))) == 6 then
          if run-identity (lift-a2 (\x y -> x + y) (Identity (2 :: Int)) (Identity (3 :: Int))) == 5 then 0 else 4
        else 3
      else 2
    else 1"#,
        0,
    );
}

#[test]
fn identity_monad_identity_laws() {
    expect_ok(
        "identity-monad-identity-laws",
        r#"  main :: Int
  main =
    let v = (Identity 10 :: Identity Int) in
    if monad-left-identity (4 :: Int) (\x -> Identity (x + 3)) then
      if monad-right-identity v then
        if monad-left-identity True (\p -> Identity (if p then False else True)) then
          if monad-right-identity (Identity False) then 0 else 4
        else 3
      else 2
    else 1"#,
        0,
    );
}

#[test]
fn identity_monad_associativity_laws() {
    expect_ok(
        "identity-monad-associativity-laws",
        r#"  main :: Int
  main =
    let v = (Identity 3 :: Identity Int) in
    if monad-associativity v (\x -> Identity (x + 4)) (\x -> Identity (x * 5)) then
      if monad-join-associativity (Identity (Identity (Identity (6 :: Int)))) then 0 else 2
    else 1"#,
        0,
    );
}

#[test]
fn const_functor_laws() {
    expect_ok(
        "const-functor-laws",
        r#"  main :: Int
  main =
    let c = (Const 11 :: Const Int Int) in
    if const-functor-identity c then
      if const-functor-composition (\x -> x * 2) (\x -> x + 1) c then
        if const-constant (\x -> x + 1) (\x -> x * 3) c then
          if const-functor-identity (Const 'q' :: Const Char Int) then 0 else 4
        else 3
      else 2
    else 1"#,
        0,
    );
}

#[test]
fn visitors_reject_wrong_shapes() {
    expect_err(
        "identity-rejects-non-identity",
        r#"  main :: Int
  main = run-identity 3"#,
        "cannot have type Identity",
    );
    expect_err(
        "const-rejects-non-const",
        r#"  main :: Int
  main = get-const 3"#,
        "cannot have type Const",
    );
}

/// The law groups over the shipped instance families: List, Option and
/// `Result e` through the polymorphic statements, `Const` through the
/// functor group. Each check is a top-level function returning 0 or
/// its bit (check k contributes bit 2^(k-1)); `main` sums them, so a
/// nonzero exit is the failing checks' bitmask (the 8-bit exit status
/// bounds the groups, and the checks are top-level because a `case` in
/// a `let` binder's right-hand side must indent past the binder's
/// column under the layout rules).
#[test]
fn family_laws_over_shipped_instances() {
    expect_ok(
        "family-laws",
        r#"  check-list :: Int
  check-list =
    let v = Cons (1 :: Int) (Cons 2 Nil) in
    if functor-identity v then
      if functor-composition (\x -> x * 2) (\x -> x + 1) v then
        if applicative-homomorphism (\x -> x + 1) (5 :: Int) (Nil :: List Int) then
          if monad-left-identity (4 :: Int) (\x -> Cons (x + 3) Nil) then
            if monad-right-identity v then
              if monad-associativity v (\x -> Cons (x + 1) Nil) (\y -> Cons (y * 5) Nil) then
                if monad-join-associativity (Cons (Cons (Cons (1 :: Int) Nil) Nil) Nil) then 0 else 64
              else 32
            else 16
          else 8
        else 4
      else 2
    else 1

  check-option :: Int
  check-option =
    let v = Some (1 :: Int) in
    if functor-identity v then
      if functor-composition (\x -> x * 2) (\x -> x + 1) v then
        if applicative-homomorphism (\x -> x + 1) (5 :: Int) (None :: Option Int) then
          if monad-left-identity (4 :: Int) (\x -> Some (x + 3)) then
            if monad-right-identity v then
              if monad-associativity v (\x -> Some (x + 1)) (\y -> Some (y * 5)) then
                if monad-join-associativity (Some (Some (Some (1 :: Int)))) then 0 else 64
              else 32
            else 16
          else 8
        else 4
      else 2
    else 1

  check-result :: Int
  check-result =
    let v = Ok (1 :: Int) :: Result Int Int in
    if functor-identity v then
      if functor-composition (\x -> x * 2) (\x -> x + 1) v then
        if applicative-homomorphism (\x -> x + 1) (5 :: Int) (Ok 0 :: Result Int Int) then
          if monad-left-identity (4 :: Int) (\x -> Ok (x + 3) :: Result Int Int) then
            if monad-right-identity v then
              if monad-associativity v (\x -> Ok (x + 1) :: Result Int Int) (\y -> Ok (y * 5) :: Result Int Int) then
                if monad-join-associativity (Ok (Ok (Ok (1 :: Int))) :: Result Int (Result Int (Result Int Int))) then 0 else 64
              else 32
            else 16
          else 8
        else 4
      else 2
    else 1

  check-const :: Int
  check-const =
    let c = (Const 11 :: Const Int Int) in
    if functor-identity c then
      if functor-composition (\x -> x * 2) (\x -> x + 1) c then 0 else 2
    else 1

  check-operators :: Int
  check-operators =
    let xs = (Cons (1 :: Int) (Cons 2 Nil)) in
    let ys = (Cons (11 :: Int) (Cons 12 Nil)) in
    let a = if same-list ((<$>) (\x -> x + 10) xs) ys then 0 else 1 in
    let b = if same-list (lift-a2 (\x y -> x + y) xs (Cons (10 :: Int) Nil)) (Cons 11 (Cons 12 Nil)) then 0 else 2 in
    let c = if same-list ((<*>) (Cons (\x -> x + 10) Nil) xs) ys then 0 else 4 in
    a + b + c

  check-alternative :: Int
  check-alternative =
    let a = if same-option ((<|>) (empty :: Option Int) (Some (1 :: Int))) (Some (1 :: Int)) then 0 else 1 in
    let b = if same-option ((<|>) (Some (2 :: Int)) (Some (1 :: Int))) (Some (2 :: Int)) then 0 else 2 in
    let c = if same-list ((<|>) (empty :: List Int) (Cons (1 :: Int) Nil)) (Cons (1 :: Int) Nil) then 0 else 4 in
    let d = if same-list ((<|>) (Cons (1 :: Int) Nil) (Cons (2 :: Int) Nil)) (Cons 1 (Cons 2 Nil)) then 0 else 8 in
    a + b + c + d

  main :: Int
  main = check-list + check-option + check-result + check-const + check-operators + check-alternative"#,
        0,
    );
}

/// The `((->) r)` family has no `Eq` of functions, so its law groups
/// run pointwise at sample arguments.
#[test]
fn function_family_laws_pointwise() {
    expect_ok(
        "function-family-laws",
        r#"  fn-id :: Int -> Int
  fn-id x = x

  fn-inc :: Int -> Int
  fn-inc x = x + 1

  fn-double :: Int -> Int
  fn-double x = x * 2

  fn-plus10 :: Int -> Int
  fn-plus10 r = r + 10

  fn-add :: Int -> Int -> Int
  fn-add x r = x + r

  fn-mul :: Int -> Int -> Int
  fn-mul y r = y * r

  fn-sum3 :: Int -> Int -> Int -> Int
  fn-sum3 x y z = x + y + z

  fn-pure6 :: Int -> Int
  fn-pure6 = pure (6 :: Int)

  fn-homomorphism :: Int -> Int
  fn-homomorphism = (<*>) (pure fn-inc) (pure (5 :: Int))

  fn-left-identity :: Int -> Int
  fn-left-identity = (>>=) (pure (5 :: Int)) fn-add

  fn-right-identity :: Int -> Int
  fn-right-identity = (>>=) fn-inc pure

  fn-associativity :: Int -> Int
  fn-associativity = (>>=) ((>>=) fn-inc fn-add) fn-mul

  fn-associativity-right :: Int -> Int
  fn-associativity-right = (>>=) fn-inc (\x -> (>>=) (fn-add x) fn-mul)

  fn-join :: Int -> Int
  fn-join = join (map join fn-sum3)

  fn-join-right :: Int -> Int
  fn-join-right = join (join fn-sum3)

  check-map-id :: Int
  check-map-id = if map fn-id fn-inc (3 :: Int) == 4 then 0 else 1

  check-map-comp :: Int
  check-map-comp =
    if map fn-double (map fn-inc fn-plus10) (3 :: Int) == map (\x -> (x + 1) * 2) fn-plus10 (3 :: Int) then 0 else 2

  check-homomorphism :: Int
  check-homomorphism = if fn-pure6 (0 :: Int) == fn-homomorphism (0 :: Int) then 0 else 4

  check-lift-a2 :: Int
  check-lift-a2 = if lift-a2 (\x y -> x + y) fn-inc fn-double (3 :: Int) == 10 then 0 else 8

  check-left-identity :: Int
  check-left-identity = if fn-left-identity (3 :: Int) == 8 then 0 else 16

  check-right-identity :: Int
  check-right-identity = if fn-right-identity (3 :: Int) == 4 then 0 else 32

  check-associativity :: Int
  check-associativity = if fn-associativity (2 :: Int) == fn-associativity-right (2 :: Int) then 0 else 64

  check-join :: Int
  check-join = if fn-join (2 :: Int) == fn-join-right (2 :: Int) then 0 else 128

  main :: Int
  main = check-map-id + check-map-comp + check-homomorphism + check-lift-a2 + check-left-identity + check-right-identity + check-associativity + check-join"#,
        0,
    );
}

/// The Applicative free combinators: `void` keeps the shape and drops
/// the value, and `when`/`unless` run their action on one of the two
/// conditions only. The action is `None`, which a condition that does
/// not run it cannot produce: the branch that does not run yields
/// `pure ()`, so the two conditions are distinguishable in the result.
#[test]
fn applicative_void_when_and_unless() {
    expect_ok(
        "applicative-void-when-unless",
        r#"  check1 :: Int
  check1 = case void (Some (3 :: Int)) of
    Some _ -> 0
    None -> 1

  check2 :: Int
  check2 = case void (None :: Option Int) of
    None -> 0
    Some _ -> 2

  check3 :: Int
  check3 = case when True (None :: Option Unit) of
    None -> 0
    Some _ -> 4

  check4 :: Int
  check4 = case when False (None :: Option Unit) of
    Some _ -> 0
    None -> 8

  check5 :: Int
  check5 = case unless True (None :: Option Unit) of
    Some _ -> 0
    None -> 16

  check6 :: Int
  check6 = case unless False (None :: Option Unit) of
    None -> 0
    Some _ -> 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6"#,
        0,
    );
}

/// `ap` (the `Control.Monad` name for `(<*>)`) and `replicate-m`: the
/// repetition's result count and order are the observed values, and
/// count 0 reports the empty list without running the action (a failing
/// action repeated zero times still succeeds).
#[test]
fn applicative_ap_and_replicate_m() {
    expect_ok(
        "applicative-ap-replicate-m",
        r#"  check1 :: Int
  check1 = if ap (Some (\y -> y + 1)) (Some (2 :: Int)) == Some 3 then 0 else 1

  check2 :: Int
  check2 = case ap (None :: Option (Int -> Int)) (Some (2 :: Int)) of
    None -> 0
    Some _ -> 2

  check3 :: Int
  check3 = case replicate-m 0 (None :: Option Int) of
    Some ys -> if same-list ys (Nil :: List Int) then 0 else 4
    None -> 4

  check4 :: Int
  check4 = case replicate-m 3 (Some (7 :: Int)) of
    Some ys -> if same-list ys (Cons 7 (Cons 7 (Cons 7 Nil))) then 0 else 8
    None -> 8

  check5 :: Int
  check5 = case replicate-m 3 (None :: Option Int) of
    None -> 0
    Some _ -> 16

  check6 :: Int
  check6 = case replicate-m 1 (Some (7 :: Int)) of
    Some ys -> if same-list ys (Cons 7 Nil) then 0 else 32
    None -> 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6"#,
        0,
    );
}

/// Kleisli composition: `>=>` runs the left step first and `<=<` is it
/// reversed (`(f <=< g) x == (g >=> f) x`), a failing step short
/// circuits either way, and the `Result` step pair shows the order in
/// which the two run (the failure reports the value the first step
/// produced). The `>=>` uses here are infix, so they also assert that
/// the operator's fixity travels with `import Category.Monad`.
#[test]
fn monad_kleisli_composition() {
    expect_ok(
        "monad-kleisli",
        r#"  step-a :: Int -> Option Int
  step-a y = Some (y + 1)

  step-b :: Int -> Option Int
  step-b y = Some (y * 2)

  step-fail :: Int -> Option Int
  step-fail y = None

  add-one :: Int -> Result Int Int
  add-one y = Ok (y + 1)

  fail-at :: Int -> Result Int Int
  fail-at y = Err y

  check1 :: Int
  check1 = if (step-a >=> step-b) 5 == Some (12 :: Int) then 0 else 1

  check2 :: Int
  check2 = if (step-b <=< step-a) 5 == Some (12 :: Int) then 0 else 2

  check3 :: Int
  check3 = if (step-a >=> step-a >=> step-a) 0 == Some (3 :: Int) then 0 else 4

  check4 :: Int
  check4 = case (step-a >=> step-fail) 5 of
    None -> 0
    Some _ -> 8

  check5 :: Int
  check5 = case (step-fail <=< step-a) 5 of
    None -> 0
    Some _ -> 16

  check6 :: Int
  check6 = case (add-one >=> fail-at) 5 of
    Err e -> if e == 6 then 0 else 32
    Ok _ -> 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6"#,
        0,
    );
}
