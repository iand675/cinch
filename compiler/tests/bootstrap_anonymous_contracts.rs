//! The compiled Cinch checker consumes checked anonymous contracts.
mod common;
use cinch::interface::Iface;
const DEFINITION: &str = "module A where\n  import Algebra.Arithmetic (NonZero)\n  export bounded, under, identity, positive, require-positive, below-zero, argument-only, collide, by-positive, fixed\n  bounded :: (limit :: Nat) -> refined (out <= limit) Nat\n  bounded limit = (0 :: Nat)\n  under :: (limit :: Nat) -> refined (limit >= out) Nat\n  under limit = (0 :: Nat)\n  identity :: (x :: Nat) -> refined (out == x) Nat\n  identity x = x\n  positive :: refined NonZero Nat\n  positive = (1 :: Nat)\n  require-positive :: refined (n > 0) Nat -> Nat\n  require-positive n = n\n  below-zero :: refined (n < 0) Int\n  below-zero = (-1 :: Int)\n  argument-only :: (limit :: Nat) -> refined (limit >= 0) Nat\n  argument-only limit = (0 :: Nat)\n  collide :: (cinch-result :: Nat) -> refined (out == cinch-result) Nat\n  collide cinch-result = cinch-result\n  by-positive :: (d :: NonZero Nat) -> refined (r < d) Nat\n  by-positive d = (0 :: Nat)\n  fixed :: refined (n >= 0) I64\n  fixed = (1 :: I64)\n";
const FACADE: &str = "module Facade where\n  import A (bounded as bound, under, identity, positive as pos, require-positive as need, below-zero, argument-only, collide, by-positive, fixed)\n  export bound, under, identity, pos, need, below-zero, argument-only, collide, by-positive, fixed\n";
fn producer() -> std::path::PathBuf {
    let dir = common::project("anonymous_contract_producer", &[
        ("A.cinch", DEFINITION), ("Facade.cinch", FACADE),
        ("Main.cinch", "module Main where\n  import Facade\n  import Data.Bytes\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&dir, "Main.cinch").expect("check real anonymous refinement definitions");
    dir
}
fn quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
fn manifest(source: &std::path::Path, target: &std::path::Path) {
    let mut rows = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" { rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string()))); }
        }
    }
    rows.sort();
    std::fs::write(target.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
}

#[test]
fn imported_anonymous_results_and_arguments_keep_their_meaning() {
    let source = producer();
    let dir = common::project("anonymous_contract_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RefinementClosure.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "anonymous-contracts").expect("compile the Cinch interface consumer");
    manifest(&source, &dir);
    for (name, body, status) in [
        ("result names argument", "probe :: refined (r <= 3) Nat\n  probe = bound 3", 0),
        ("argument appears before result", "probe :: refined (r <= 4) Nat\n  probe = under 4", 0),
        ("result equality uses actual argument", "probe :: refined (r == 9) Nat\n  probe = identity 9", 0),
        ("fresh result avoids argument collision", "probe :: refined (r == 5) Nat\n  probe = collide 5", 0),
        ("bare structural result projects", "probe :: Nat\n  probe = pos", 0),
        ("anonymous parameter accepts proven value", "probe :: Nat\n  probe = need 2", 0),
        ("anonymous parameter rejects zero", "bad :: Nat\n  bad = need 0", 4),
        ("negative mathematical result", "probe :: refined (r < 0) Int\n  probe = below-zero", 0),
        ("argument-only condition keeps result independent", "probe :: Nat\n  probe = argument-only 0", 0),
        ("exported bound cannot strengthen", "bad :: refined (r < 3) Nat\n  bad = bound 3", 4),
        ("equality cannot change argument identity", "bad :: refined (r == 8) Nat\n  bad = identity 9", 4),
        ("negative bound cannot hold of Nat", "bad :: refined (r < 0) Nat\n  bad = bound 0", 4),
        ("positive result cannot prove stronger bound", "bad :: refined (r > 10) Nat\n  bad = pos", 4),
        ("result names refined argument carrier", "probe :: refined (r < 3) Nat\n  probe = by-positive 3", 0),
        ("refined argument still rejects zero", "bad :: Nat\n  bad = by-positive 0", 4),
        ("real remainder retains result bound", "probe :: refined (r < 3) Nat\n  probe = nat-rem 7 3", 0),
        ("real remainder rejects zero divisor", "bad :: Nat\n  bad = nat-rem 7 0", 4),
    ] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  import Facade (bound, under, identity, pos, need, below-zero, argument-only, collide, by-positive)\n  import Data.Nat (nat-rem)\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(status), "{name}: {report}\n{}", String::from_utf8_lossy(&output.stderr));
        if status == 0 { assert!(report.is_empty(), "{name}: {report}"); }
        else { assert!(report.contains("refinement predicate is not established"), "{name}: {report}"); }
        eprintln!("anonymous contract case passed: {name}");
    }
    common::cleanup(&source); common::cleanup(&dir);
}
