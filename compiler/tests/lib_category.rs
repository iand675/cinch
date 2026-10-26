//! Category-family library tests: the `Identity` and `Const` universal
//! visitors and the law groups the category classes must satisfy, checked
//! over sample values.
//!
//! The `Functor`/`Applicative`/`Monad` classes themselves are blocked on
//! missing surface (the checker rejects type-constructor variables, so
//! `class Functor f` cannot be declared; see the slice report). What is
//! under test here is the per-type instance content of
//! `compiler/prelude/Data/Identity.lang` and `compiler/prelude/Data/Const.lang`
//! (the method bodies the classes will carry as instances), so the law
//! groups lock in their semantics now: functor identity and composition,
//! applicative homomorphism, monad left and right identity, and monad
//! associativity in both bind and join form.

use safelang::driver::{check_sources, run_source};

const IDENTITY: &str = include_str!("../prelude/Data/Identity.lang");
const CONST: &str = include_str!("../prelude/Data/Const.lang");

/// The law statements, polymorphic over the element types and decided
/// through their `Eq` constraints.
const LAWS: &str = r#"  functor-identity-law :: forall a. using (eq :: Eq a) => Identity a -> Bool
  functor-identity-law v =
    run-identity (map-identity (\x -> x) v) == run-identity v

  functor-composition-law :: forall a b c. using (eq :: Eq c) => (b -> c) -> (a -> b) -> Identity a -> Bool
  functor-composition-law g f v =
    let left = run-identity (map-identity g (map-identity f v)) in
    let right = run-identity (map-identity (\x -> g (f x)) v) in
    left == right

  applicative-homomorphism-law :: forall a b. using (eq :: Eq b) => (a -> b) -> a -> Bool
  applicative-homomorphism-law f x =
    run-identity (pure-identity (f x)) == run-identity (ap-identity (pure-identity f) (pure-identity x))

  monad-left-identity-law :: forall a b. using (eq :: Eq b) => a -> (a -> Identity b) -> Bool
  monad-left-identity-law x k =
    run-identity (bind-identity (pure-identity x) k) == run-identity (k x)

  monad-right-identity-law :: forall a. using (eq :: Eq a) => Identity a -> Bool
  monad-right-identity-law v =
    run-identity (bind-identity v pure-identity) == run-identity v

  monad-associativity-law :: forall a b c. using (eq :: Eq c) => Identity a -> (a -> Identity b) -> (b -> Identity c) -> Bool
  monad-associativity-law v f g =
    let left = run-identity (bind-identity (bind-identity v f) g) in
    let right = run-identity (bind-identity v (\x -> bind-identity (f x) g)) in
    left == right

  monad-join-associativity-law :: forall a. using (eq :: Eq a) => Identity (Identity (Identity a)) -> Bool
  monad-join-associativity-law vvv =
    let left = run-identity (join-identity (map-identity join-identity vvv)) in
    let right = run-identity (join-identity (join-identity vvv)) in
    left == right

  const-functor-identity-law :: forall m a. using (eq :: Eq m) => Const m a -> Bool
  const-functor-identity-law c =
    get-const (map-const (\x -> x) c) == get-const c

  const-functor-composition-law :: forall m a b r. using (eq :: Eq m) => (b -> r) -> (a -> b) -> Const m a -> Bool
  const-functor-composition-law g f c =
    let left = get-const (map-const g (map-const f c)) in
    let right = get-const (map-const (\x -> g (f x)) c) in
    left == right

  const-constant-law :: forall m a b. using (eq :: Eq m) => (a -> b) -> (a -> b) -> Const m a -> Bool
  const-constant-law f h c =
    get-const (map-const f c) == get-const (map-const h c)
"#;

/// The prelude modules plus a `Main` exercising them. The module
/// headers are single-segment pending the dotted-module-name fix, so
/// the imports name `Identity` and `Const`.
fn source(body: &str) -> String {
    format!("{IDENTITY}\n{CONST}\nmodule Main where\n  import Identity\n  import Const\n\n{LAWS}\n{body}\n")
}

fn expect_ok(name: &str, body: &str, expected: i32) {
    match run_source(&source(body)) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, body: &str, needle: &str) {
    match check_sources(&source(body), "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}"),
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
fn visitors_round_trip() {
    expect_ok(
        "visitors-round-trip",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let v = (Identity 7 :: Identity Int) in\n\
         \x20   let w = Identity True in\n\
         \x20   let c = (Const 11 :: Const Int Bool) in\n\
         \x20   let s = (Const 'q' :: Const Char Int) in\n\
         \x20   if run-identity v == 7 then\n\
         \x20     if run-identity w == True then\n\
         \x20       if get-const c == 11 then\n\
         \x20         if get-const s == 'q' then 0 else 4\n\
         \x20       else 3\n\
         \x20     else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn identity_functor_laws() {
    expect_ok(
        "identity-functor-laws",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let v = (Identity 7 :: Identity Int) in\n\
         \x20   let w = Identity True in\n\
         \x20   if functor-identity-law v then\n\
         \x20     if functor-identity-law w then\n\
         \x20       if functor-composition-law (\\x -> x * 2) (\\x -> x + 1) v then\n\
         \x20         if functor-composition-law (\\p -> if p then False else True) (\\p -> if p then False else True) w then 0 else 4\n\
         \x20       else 3\n\
         \x20     else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn identity_applicative_homomorphism() {
    expect_ok(
        "identity-applicative-homomorphism",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   if applicative-homomorphism-law (\\x -> x + 1) (5 :: Int) then\n\
         \x20     if applicative-homomorphism-law (\\p -> if p then False else True) True then\n\
         \x20       if run-identity (ap-identity (Identity (\\x -> x + 1)) (Identity (5 :: Int))) == 6 then\n\
         \x20         if run-identity (lift-a2-identity (\\x y -> x + y) (Identity (2 :: Int)) (Identity (3 :: Int))) == 5 then 0 else 4\n\
         \x20       else 3\n\
         \x20     else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn identity_monad_identity_laws() {
    expect_ok(
        "identity-monad-identity-laws",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let v = (Identity 10 :: Identity Int) in\n\
         \x20   if monad-left-identity-law (4 :: Int) (\\x -> Identity (x + 3)) then\n\
         \x20     if monad-right-identity-law v then\n\
         \x20       if monad-left-identity-law True (\\p -> Identity (if p then False else True)) then\n\
         \x20         if monad-right-identity-law (Identity False) then 0 else 4\n\
         \x20       else 3\n\
         \x20     else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn identity_monad_associativity_laws() {
    expect_ok(
        "identity-monad-associativity-laws",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let v = (Identity 3 :: Identity Int) in\n\
         \x20   if monad-associativity-law v (\\x -> Identity (x + 4)) (\\x -> Identity (x * 5)) then\n\
         \x20     if monad-join-associativity-law (Identity (Identity (Identity (6 :: Int)))) then 0 else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn const_functor_laws() {
    expect_ok(
        "const-functor-laws",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let c = (Const 11 :: Const Int Int) in\n\
         \x20   if const-functor-identity-law c then\n\
         \x20     if const-functor-composition-law (\\x -> x * 2) (\\x -> x + 1) c then\n\
         \x20       if const-constant-law (\\x -> x + 1) (\\x -> x * 3) c then\n\
         \x20         if const-functor-identity-law (Const 'q' :: Const Char Int) then 0 else 4\n\
         \x20       else 3\n\
         \x20     else 2\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn visitors_reject_wrong_shapes() {
    expect_err(
        "identity-rejects-non-identity",
        "  main :: Int\n\
         \x20 main = run-identity 3\n",
        "cannot have type Identity",
    );
    expect_err(
        "const-rejects-non-const",
        "  main :: Int\n\
         \x20 main = get-const 3\n",
        "cannot have type Const",
    );
}
