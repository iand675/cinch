//! Real emitted contracts are consumed by the compiled Cinch frontend.
mod common;
use cinch::interface::Iface;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}


#[test]
fn imported_structural_nonzero_uses_checked_argument_identity() {
    let source = common::project("nonzero_condition_source", &[
        ("A.cinch", "module A where\n  pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  signature Gate a where\n    gate :: a -> a\n  keep-with :: forall a. using (witness :: Gate a, n > 0) => (n :: Nat) -> a -> a\n  keep-with n value = value\n  difference :: using (left >= right) => (left :: Nat) -> (right :: Nat) -> Nat\n  difference a b = a - b\n  need :: using (NonZero n) => (n :: Nat) -> Nat\n  need actual = actual\n  need-int :: using (NonZero n) => (n :: Int) -> Int\n  need-int actual = actual\n  both :: using (NonZero left, NonZero right) => (left :: Nat) -> (right :: Nat) -> Nat\n  both a b = a\n  scalar :: using (NonZero n) => (n :: I64) -> I64\n  scalar n = n\n  reflected :: using (NonZero (bytes-len bytes)) => (bytes :: Bytes) -> Bytes\n  reflected bytes = bytes\n"),
        ("Facade.cinch", "module Facade where\n  import A (pred as previous, difference, Gate, keep-with, need as require-value, need-int, both, scalar as require-scalar, reflected as require-buffer)\n  import Data.Bytes (bytes-len as measure)\n  export previous, difference, Gate, keep-with, require-value, need-int, both, require-scalar, require-buffer, measure\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&source, "Main.cinch").expect("emit portable preconditions");
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(source.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A");
    let facade = read("Facade");
    let get = |iface: &Iface, name: &str| iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone();
    let need = get(&a, "need").expect("checked native NonZero contract");
    assert!(need.scheme.contains("NonZero n") && need.scheme.contains("n :: Nat"), "{}", need.scheme);
    assert!(!need.scheme.contains('$'));
    assert_eq!(get(&facade, "require-value"), Some(need.clone()));
    assert_eq!(Iface::from_json(&a.to_json()).unwrap(), a);
    let scalar = get(&a, "scalar").expect("machine-value NonZero keeps its actual I64 carrier");
    assert!(scalar.scheme.contains("NonZero n") && scalar.scheme.contains("n :: I64"), "{}", scalar.scheme);
    let reflected = get(&a, "reflected").expect("checked byte-length NonZero predicate closes over its actual buffer");
    assert!(reflected.scheme.contains("NonZero (b.bytesLen bytes)") && reflected.scheme.contains("bytes :: Bytes"), "{}", reflected.scheme);
    let mut changed = a.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "need").unwrap().resolved.as_mut().unwrap().scheme = need.scheme.replace("NonZero n", "n > 0");
    assert_ne!(changed.iface_hash(), a.iface_hash(), "structural predicate identity must participate in hashing");
    let dir = common::project("nonzero_condition_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let mut rows = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" {
                rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
            }
        }
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "imported-preconditions").expect("build imported precondition consumer");
    let cases = [
        ("machine NonZero negative literal", "answer :: I64\n  answer = require-scalar (-1)", 0),
        ("machine NonZero zero literal", "bad :: I64\n  bad = require-scalar 0", 4),
        ("machine NonZero guard", "safe :: I64 -> I64\n  safe n = if n /= 0 then require-scalar n else 0", 0),
        ("machine NonZero wrong argument", "bad :: I64 -> I64 -> I64\n  bad n m = if n /= 0 then require-scalar m else 0", 4),
        ("primitive NonZero buffer", "safe :: Bytes -> Bytes\n  safe b = if measure b > 0 then require-buffer b else b", 0),
        ("primitive NonZero requires condition", "bad :: Bytes -> Bytes\n  bad b = require-buffer b", 4),
        ("primitive NonZero wrong buffer", "bad :: Bytes -> Bytes -> Bytes\n  bad b c = if measure b > 0 then require-buffer c else c", 4),
        ("primitive NonZero shadowed buffer", "bad :: Bytes -> Bytes -> Bytes\n  bad b c = if measure b > 0 then (let b = c in require-buffer b) else c", 4),
        ("primitive NonZero forwarded premise", "safe :: using (NonZero (measure b)) => (b :: Bytes) -> Bytes\n  safe b = require-buffer b", 0),
        ("NonZero literal", "answer :: Nat\n  answer = require-value 1", 0),
        ("NonZero rejects zero", "bad :: Nat\n  bad = require-value 0", 4),
        ("NonZero actual guarded argument", "safe :: Nat -> Nat\n  safe n = if n /= 0 then require-value n else 0", 0),
        ("NonZero wrong guarded identity", "bad :: Nat -> Nat -> Nat\n  bad n m = if n /= 0 then require-value m else 0", 4),
        ("NonZero shadowed identity", "bad :: Nat -> Nat\n  bad n = if n /= 0 then (let n = (0 :: Nat) in require-value n) else 0", 4),
        ("NonZero ignored bad call", "bad :: Nat\n  bad = let ignored = require-value 0 in 1", 4),
        ("NonZero forwarded structural premise", "safe :: using (NonZero x) => (x :: Nat) -> Nat\n  safe actual = require-value actual", 0),
        ("NonZero forwarded arithmetic premise", "safe :: using (x > 0) => (x :: Nat) -> Nat\n  safe actual = require-value actual", 0),
        ("NonZero insufficient arithmetic premise", "bad :: using (x >= 0) => (x :: Nat) -> Nat\n  bad actual = require-value actual", 4),
        ("NonZero integer negative literal", "answer :: Int\n  answer = need-int (-1)", 0),
        ("NonZero integer zero literal", "bad :: Int\n  bad = need-int 0", 4),
        ("NonZero rejects positive-or-zero unknown", "bad :: Int -> Int\n  bad n = need-int n", 4),
        ("NonZero two arguments", "answer :: Nat\n  answer = both 1 2", 0),
        ("NonZero second argument zero", "bad :: Nat\n  bad = both 1 0", 4),
        ("NonZero retained partial application", "answer :: Nat\n  answer = let partial = both 1 in partial 2", 0),
        ("NonZero partial call still checks second argument", "bad :: Nat\n  bad = let partial = both 1 in partial 0", 4),
        ("literal", "answer :: Nat\n  answer = previous 1", 0),
        ("branch", "safe :: Nat -> Nat\n  safe n = if n > 0 then previous n else 0", 0),
        ("forwarded condition", "safe :: using (x > 0) => (x :: Nat) -> Nat\n  safe actual = previous actual", 0),
        ("two arguments", "answer :: Nat\n  answer = difference 7 2", 0),
        ("partial application", "answer :: Nat\n  answer = let partial = difference 7 in partial 2", 0),
        ("mixed contexts", "forward :: forall a. using (witness :: Gate a) => a -> a\n  forward x = keep-with 1 x", 0),
        ("mixed contexts reject bad condition", "bad :: forall a. using (witness :: Gate a) => a -> a\n  bad x = keep-with 0 x", 4),
        ("rejected literal", "bad :: Nat\n  bad = previous 0", 4),
        ("ignored result", "bad :: Nat\n  bad = let ignored = previous 0 in 0", 4),
        ("wrong argument", "bad :: Nat -> Nat -> Nat\n  bad n m = if m > 0 then previous n else 0", 4),
        ("shadowed argument", "bad :: Nat -> Nat\n  bad n = if n > 0 then (let n = (0 :: Nat) in previous n) else 0", 4),
        ("reversed arguments", "bad :: Nat\n  bad = difference 2 7", 4),
        ("addition argument", "safe :: Nat -> Nat\n  safe n = previous (n + 1)", 0),
        ("addition guard", "safe :: Nat -> Nat\n  safe n = if n + 1 > 1 then previous n else 0", 0),
        ("multiplication argument", "safe :: Nat -> Nat\n  safe n = previous (n * 2 + 1)", 0),
        ("insufficient arithmetic guard", "bad :: Nat -> Nat\n  bad n = if n + 1 > 0 then previous n else 0", 4),
        ("integer subtraction argument", "step :: using (n > 0) => (n :: Int) -> Int\n  step actual = actual - 1\n  safe :: Int -> Int\n  safe n = if n > 1 then step (n - 1) else 0", 0),
        ("unsafe integer subtraction argument", "step :: using (n > 0) => (n :: Int) -> Int\n  step actual = actual - 1\n  bad :: Int -> Int\n  bad n = step (n - 1)", 4),
        ("shadowed comparison", "infix 3 >\n  (>) :: Nat -> Nat -> Bool\n  (>) x y = True\n  bad :: Nat -> Nat\n  bad n = if n > 0 then previous n else 0", 4),
        ("lexical ordering dictionary is opaque", "bad :: using (ordering :: Ord Nat) => Nat -> Nat\n  bad n = if n > 0 then previous n else 0", 4),
    ];
    for (name, body, wanted) in cases {
        let input = format!("module Main where\n  import Facade (previous, difference, Gate, keep-with, require-value, need-int, both, require-scalar, require-buffer, measure)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &input).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "imported-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}: {input}\n{report}\n{}", String::from_utf8_lossy(&output.stderr));
        if wanted == 4 { assert!(report.contains("precondition is not established"), "{name}: {report}"); }
    }
    common::cleanup(&source); common::cleanup(&dir);
}
