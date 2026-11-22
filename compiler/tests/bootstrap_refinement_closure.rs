//! The compiled Cinch frontend consumes real private defining templates.
mod common;
use cinch::interface::Iface;
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
fn private_refinements_keep_checked_bounds_without_exporting_names() {
    let source = common::project("refinement_closure_producer", &[
        ("A.cinch", "module A where\n  export bounded, widen, positive\n  type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  type Positive = refine (n :: Nat) with n > 0\n  bounded :: Window 0 10 Nat\n  bounded = 1\n  widen :: Window 2 8 Nat -> Window 0 10 Nat\n  widen n = n\n  positive :: Positive\n  positive = 1\n"),
        ("Facade.cinch", "module Facade where\n  import A (bounded as one, widen as wide, positive as pos)\n  export one, wide, pos\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&source, "Main.cinch").expect("produce native private contracts");
    let dir = common::project("refinement_closure_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RefinementClosure.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "private-refinements").expect("compile the actual Cinch consumer");
    manifest(&source, &dir);
    for (name, body, status, diagnostic) in [
        ("private bounded result projects", "probe :: Nat\n  probe = one", 0, ""),
        ("private positive result projects", "probe :: Nat\n  probe = pos", 0, ""),
        ("private bounds checked at call", "probe :: Nat\n  probe = wide 5", 0, ""),
        ("private lower bound rejects argument", "bad :: Nat\n  bad = wide 0", 4, "refinement predicate is not established"),
        ("private upper bound rejects argument", "bad :: Nat\n  bad = wide 9", 4, "refinement predicate is not established"),
        ("private name stays hidden", "bad :: Window 0 10 Nat\n  bad = 1", 4, "unknown type constructor"),
        ("private result cannot establish stronger bound", "bad :: refined (n > 10) Nat\n  bad = one", 4, "refinement predicate is not established"),
    ] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  import Facade (one, wide, pos)\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(status), "{name}: {report}\n{}", String::from_utf8_lossy(&output.stderr));
        if status == 0 { assert!(report.is_empty(), "{name}: {report}"); }
        else { assert!(report.contains(diagnostic), "{name}: {report}"); }
    }
    // Rehash a well-shaped but contradictory facade contract. Its own
    // canonical hash is valid; it must still agree with the defining A.
    let path = source.join("build/Facade.cinchif");
    let mut facade = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
    facade.exports.values.iter_mut().find(|v| v.name == "one").unwrap().resolved.as_mut().unwrap().nominals[0].refinement.as_mut().unwrap().predicate = "lo < x && x < hi".into();
    std::fs::write(&path, facade.to_json()).unwrap();
    manifest(&source, &dir);
    std::fs::write(dir.join("source.cinch"), "module Main where\n  import Facade (one)\n  probe :: Nat\n  probe = one\n  main :: Int\n  main = 0\n").unwrap();
    let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "refinement-report.txt");
    assert_eq!(output.status.code(), Some(89), "contradictory defining payload must be refused by the interface loader: {report}");
    assert!(report.contains("conflicting defining nominal contracts: A.Window"), "{report}");
    common::cleanup(&source); common::cleanup(&dir);
}
