//! Strict let equations follow actual checked Nat bodies and lexical scopes.
use cinch::driver::check_sources;
fn check(body: &str) -> Result<(), String> {
    check_sources(&format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n"), "checked-nat-let.cinch").map(|_| ()).map_err(|e| e.message)
}
#[test]
fn guarded_subtraction_keeps_its_strict_let_equation() {
    check("offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = let result = total - extra in result").unwrap();
    check("offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = let result = total - extra in let alias = result in alias").unwrap();
    check("offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = let result = nat-sub total extra in result").unwrap();
}
#[test]
fn guarded_let_equations_do_not_supply_other_results_or_call_guards() {
    for body in [
        "bad :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total + 1) Nat\n  bad total extra = let result = total - extra in result",
        "bad :: (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  bad total extra = let result = total - extra in result",
        "bad :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  bad total extra = let result = total - extra in let result = (0 :: Nat) in result",
        "bad :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r == total) Nat\n  bad total extra = let total = total - extra in total",
        "bad :: using (sub :: Sub Nat) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  bad total extra = let result = total - extra in result",
    ] { assert!(check(body).is_err(), "forged let equation accepted: {body}"); }
}

#[test]
fn a_changed_selected_body_changes_the_retained_equation() {
    let provider = "module A where\n  export Difference, minus-op\n  signature Difference a where\n    minus-op :: a -> a -> a\n  implicit instance Difference_Nat :: Difference Nat where\n    minus-op :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat\n    minus-op x y = nat-sub x y\n";
    let client = "module Main where\n  import A\n  offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = let result = minus-op total extra in result\n  main :: Int\n  main = 0\n";
    assert!(check_sources(&format!("{provider}\n{client}"), "own-guarded-let.cinch").is_ok());
    let changed = provider.replace("= nat-sub x y", "= x");
    assert!(check_sources(&format!("{changed}\n{client}"), "changed-guarded-let.cinch").is_err());
    let changed_client = client.replace("r + extra == total", "r == total");
    assert!(check_sources(&format!("{changed}\n{changed_client}"), "actual-changed-guarded-let.cinch").is_ok());
    assert!(check_sources(&format!("{provider}\n{changed_client}"), "wrong-guarded-let.cinch").is_err());
}

#[test]
fn lexical_provenance_does_not_unfold_opaque_measure_facts() {
    check("previous-length :: String -> Nat\n  previous-length name = if string-length name == 0 then 0 else string-length name - 1").unwrap();
}

#[test]
fn refined_nat_length_aliases_keep_the_actual_buffer_equation() {
    check("first :: Bytes -> Nat\n  first bytes = let size = bytes-len bytes in if size > 0 then bytes-get bytes 0 else 0").unwrap();
    check("first :: Bytes -> Nat\n  first bytes = let size = bytes-len bytes in let alias = size in if alias > 0 then bytes-get bytes 0 else 0").unwrap();
    check("splice :: Bytes -> Nat -> Nat -> Bytes -> Bytes\n  splice hay lo hi repl =\n    let size = bytes-len hay in\n    if lo <= hi && hi <= size then\n      let suffix = size - hi in\n      let width = bytes-len repl in\n      let out = bytes-new (suffix + lo + width) in\n      let u = copy-bytes-into out hay 0 0 lo in\n      let v = copy-bytes-into out repl lo 0 width in\n      let w = copy-bytes-into out hay (lo + width) hi suffix in\n      out\n    else hay").unwrap();
}

#[test]
fn refined_nat_alias_equations_cannot_change_buffer_identity_or_bounds() {
    for body in [
        "bad :: Bytes -> Bytes -> Nat\n  bad bytes other = let size = bytes-len other in if size > 0 then bytes-get bytes 0 else 0",
        "bad :: Bytes -> Bytes -> Nat\n  bad bytes other = let size = bytes-len bytes in let bytes = other in if size > 0 then bytes-get bytes 0 else 0",
        "bad :: Bytes -> Nat\n  bad bytes = let size = bytes-len bytes in let size = (1 :: Nat) in if size > 0 then bytes-get bytes 0 else 0",
        "bad :: Bytes -> Nat\n  bad bytes = let size = bytes-len bytes in if size > 0 then bytes-get bytes size else 0",
        "bad :: Bytes -> (Bytes -> Nat) -> Nat\n  bad bytes length = let size = length bytes in if size > 0 then bytes-get bytes 0 else 0",
    ] {
        assert!(check(body).is_err(), "forged refined Nat alias equation accepted: {body}");
    }
}

#[test]
fn byte_offset_equations_retain_actual_observation_and_arithmetic_bodies() {
    assert!(check("offset :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> refined (r + extra + bytes-len x == bytes-len out) Nat\n  offset extra x out = bytes-len out - (extra + bytes-len x)").is_ok(), "byte offset result equation");
    assert!(check("offset :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> refined (r + extra + bytes-len x == bytes-len out + 1) Nat\n  offset extra x out = bytes-len out - (extra + bytes-len x)").is_err(), "wrong byte offset result equation");
    assert!(check("copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)").is_ok(), "raw byte copy inline");
    assert!(check("copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) in copy-bytes-into out x offset 0 (bytes-len x)").is_ok(), "raw byte copy strict offset");
    assert!(check("copy :: (extra :: Nat) -> (x :: Bytes) -> refined (extra + bytes-len x <= bytes-len out) Bytes -> Unit\n  copy extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)").is_ok(), "refined byte copy inline");
    assert!(check("copy :: (extra :: Nat) -> (x :: Bytes) -> refined (extra + bytes-len x <= bytes-len out) Bytes -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) in copy-bytes-into out x offset 0 (bytes-len x)").is_ok(), "refined byte copy strict offset");
    assert!(check("bad :: (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  bad extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)").is_err(), "byte copy missing guard");
    assert!(check("copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) + 1 in copy-bytes-into out x offset 0 (bytes-len x)").is_err(), "byte copy wrong offset");
    assert!(check("flag-result :: Bool -> Int\n  flag-result value = if value then 1 else 0\n  use-flag :: Bool -> Int\n  use-flag value = (let value = 0 in flag-result) value").is_ok(), "function let cannot capture argument");
    assert!(check("flag-result :: Bool -> Int\n  flag-result value = if value then 1 else 0\n  bad :: Int -> Int\n  bad value = (let value = True in flag-result) value").is_err(), "function let cannot repair wrong argument");
}
