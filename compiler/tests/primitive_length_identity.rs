//! Length facts follow the checked primitive, including executable index witnesses.
mod common;

#[test]
fn fresh_primitive_declaration_supplies_stable_length_facts() {
    let dir=common::project("length_primitive_alias", &[("Main.cinch", r#"
module Main where
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  first :: Bytes -> Nat
  first b = if extent b > 0 then bytes-get b 0 else 0
  main :: Int
  main = let b = bytes-new 1 in let u = bytes-set b 0 7 in if first b == 7 then 0 else 1
"#)]);
    assert_eq!(common::run(&dir,"Main.cinch"),0);
    common::cleanup(&dir);
}

#[test]
fn primitive_index_witness_executes_the_operation_not_a_missing_global() {
    let dir=common::project("length_index_witness", &[("Main.cinch", r#"
module Main where
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  data Mark (n :: Nat) where
    Mark :: Mark n
  make :: (b :: Bytes) -> Mark (extent b)
  make b = Mark
  width :: forall (n :: Nat). Mark n -> Nat
  width value = n
  main :: Int
  main = let b = bytes-new 3 in if width (make b) == 3 then 0 else 1
"#)]);
    assert_eq!(common::run(&dir,"Main.cinch"),0);
    common::cleanup(&dir);
}

#[test]
fn allocation_promise_agrees_with_an_actual_primitive_alias() {
    let dir=common::project("length_allocation_promise", &[("Main.cinch", r#"
module Main where
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  foreign import ccall "cinch.runtime.bytesNew" make-bytes :: (n :: Nat) -> refined (extent out == n) Bytes
  make :: (n :: Nat) -> refined (extent out == n) Bytes
  make n = make-bytes n
  main :: Int
  main = let b = make 4 in if extent b == 4 then 0 else 1
"#)]);
    assert_eq!(common::run(&dir,"Main.cinch"),0);
    common::cleanup(&dir);
}

#[test]
fn same_spelled_ordinary_reader_cannot_supply_immutable_facts() {
    let dir=common::project("length_ordinary_reader", &[("Main.cinch", r#"
module Main where
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  foreign import ccall "cinch.runtime.bytesGet" raw-get :: Bytes -> Nat -> InRange 0 255 Nat
  bytes-len :: Bytes -> Nat
  bytes-len b = raw-get b 0
  positive :: NonZero Nat -> Nat
  positive n = nat-div 1 n
  attempt :: using (extent b > 0) => (b :: Bytes) -> Nat
  attempt b = if bytes-len b > 0 then let u = bytes-set b 0 0 in positive (bytes-len b) else 0
  main :: Int
  main = 0
"#)]);
    let error=common::check(&dir,"Main.cinch").expect_err("ordinary mutable-content reader must not become the length primitive");
    assert!(error.contains("obligation") || error.contains("NonZero"),"{error}");
    common::cleanup(&dir);
}

#[test]
fn length_indices_reject_non_byte_carriers() {
    for carrier in ["Nat", "Int", "Bool"] {
        let source=format!("module Main where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n  data Mark (n :: Nat) where\n    Mark :: Mark n\n  bad :: (value :: {carrier}) -> Mark (extent value)\n  bad value = Mark\n  main :: Int\n  main = 0\n");
        let dir=common::project("length_wrong_carrier", &[("Main.cinch", &source)]);
        let error=common::check(&dir,"Main.cinch").expect_err("the actual observation operand must be Bytes");
        assert!(error.contains("Bytes") || error.contains("kind"),"{carrier}: {error}");
        common::cleanup(&dir);
    }
}

#[test]
fn primitive_indices_check_result_carriers_and_arity() {
    for index in ["extent b", "extent b b", "extent"] {
        let source=format!("module Main where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n  data Mark (n :: Bool) where\n    Mark :: Mark n\n  bad :: (b :: Bytes) -> Mark ({index})\n  bad b = Mark\n  main :: Int\n  main = 0\n");
        let dir=common::project("length_wrong_result", &[("Main.cinch", &source)]);
        let error=common::check(&dir,"Main.cinch").expect_err("length is a unary Bytes -> Nat primitive");
        assert!(error.contains("type") || error.contains("carrier") || error.contains("kind"),"{index}: {error}");
        common::cleanup(&dir);
    }
}

#[test]
fn index_operands_use_forall_and_nested_application_carriers() {
    for signature in ["forall (n :: Nat). Mark (extent n)", "(f :: Nat -> Nat) -> (n :: Nat) -> Mark (extent (f n))"] {
        let source=format!("module Main where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n  data Mark (n :: Nat) where\n    Mark :: Mark n\n  bad :: {signature}\n  main :: Int\n  main = 0\n");
        let dir=common::project("length_wrong_nested_carrier", &[("Main.cinch", &source)]);
        let error=common::check(&dir,"Main.cinch").expect_err("nested observation operands retain actual declared carriers");
        assert!(error.contains("Bytes"),"{signature}: {error}");
        common::cleanup(&dir);
    }
}

#[test]
fn refined_byte_operands_and_polymorphic_helpers_keep_their_carriers() {
    let dir=common::project("length_typed_operands", &[("Main.cinch", r#"
module Main where
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  id :: forall a. a -> a
  id x = x
  data Mark (n :: Nat) where
    Mark :: Mark n
  make :: (b :: refined (extent out >= 0) Bytes) -> Mark (extent (id b))
  make b = Mark
  main :: Int
  main = 0
"#)]);
    common::check(&dir,"Main.cinch").expect("refined Bytes and instantiated helper result are valid observation operands");
    common::cleanup(&dir);
}
