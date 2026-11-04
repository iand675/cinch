//! One-way Boolean branch implications preserve actual checked numeric guards.
mod common;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
#[test]
fn compiled_cinch_retains_only_valid_one_way_boolean_facts() {
    let dir = common::project("boolean_branch_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.GuardedCalls.cinch")),
    ]);

    let (_, interfaces) = cinch::driver::check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "guarded-call-base.cinch").unwrap();
    let mut rows = Vec::new();
    let folder = dir.join("interfaces"); std::fs::create_dir_all(&folder).unwrap();
    for iface in interfaces.iter().filter(|i| i.module != "Main") {
        let path = folder.join(format!("{}.cinchif", iface.module));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let mut cases = vec![
        ("guarded JSON slot conversion", "import Data.Nat\n  probe :: Nat -> Int -> Option Nat\n  probe count slot = if slot >= 0 && nat-from-int slot < count then Some (nat-from-int slot) else None", true),
        ("true conjunction keeps left guard", "import Data.Nat\n  probe :: (Int -> Bool) -> Int -> Nat\n  probe test slot = if slot >= 0 && test slot then nat-from-int slot else 0", true),
        ("false disjunction keeps negative left guard", "import Data.Nat\n  probe :: (Int -> Bool) -> Int -> Nat\n  probe test slot = if slot < 0 || test slot then 0 else nat-from-int slot", true),
        ("false conjunction cannot negate left guard", "import Data.Nat\n  probe :: (Int -> Bool) -> Int -> Nat\n  probe test slot = if slot < 0 && test slot then 0 else nat-from-int slot", false),
        ("true disjunction cannot assert left guard", "import Data.Nat\n  probe :: (Int -> Bool) -> Int -> Nat\n  probe test slot = if slot >= 0 || test slot then nat-from-int slot else 0", false),
        ("nested conjunction keeps outer guard", "import Data.Nat\n  probe :: (Int -> Bool) -> (Int -> Bool) -> Int -> Nat\n  probe first second slot = if slot >= 0 && first slot && second slot then nat-from-int slot else 0", true),
    ].into_iter().map(|(name, body, accepted)| (name, body.to_string(), accepted)).collect::<Vec<_>>();
    for (name, expression, accepted) in [
        ("case false comparison keeps nonzero", "case d == 0 of\n    True -> 0\n    False -> keep d d", true),
        ("case true equality cannot prove nonzero", "case d == 0 of\n    True -> keep d d\n    False -> 0", false),
        ("case literal fallthrough excludes zero", "case d of\n    0 -> 0\n    _ -> keep d d", true),
        ("case matched zero cannot prove nonzero", "case d of\n    0 -> keep d d\n    _ -> 0", false),
        ("guarded literal failure is not pattern refutation", "case d of\n    0 | guard -> 0\n    _ -> keep d d", false),
        ("false literal guard leaves zero possible", "case d of\n    0 | False -> 0\n    _ -> keep d d", false),
        ("true literal guard covers its pattern", "case d of\n    0 | True -> 0\n    _ -> keep d d", true),
        ("case alias keeps actual scalar identity", "case d of\n    0 -> 0\n    value -> keep value value", true),
        ("case shadow keeps actual scalar identity", "case d of\n    0 -> 0\n    d -> keep d d", true),
        ("case Boolean wildcard uses preceding refutation", "case d == 0 of\n    True -> 0\n    _ -> keep d d", true),
        ("case Boolean wildcard cannot invent a polarity", "case d == 0 of\n    _ -> keep d d", false),
        ("case true inequality keeps nonzero", "case d /= 0 of\n    True -> keep d d\n    False -> 0", true),
        ("case false inequality cannot prove nonzero", "case d /= 0 of\n    True -> 0\n    False -> keep d d", false),
        ("case successive literals keep ordered refutations", "case d of\n    0 -> 0\n    1 -> keep d d\n    _ -> keep d d", true),
        ("case facts do not escape to another local", "case d of\n    0 -> 0\n    _ -> let other = (0 :: I64) in keep other other", false),
    ] {
        cases.push((name, format!("type NZ = refine (n :: I64) with NonZero n\n  keep :: I64 -> NZ -> I64\n  keep n d = n\n  probe :: Bool -> I64 -> I64\n  probe guard d = {expression}"), accepted));
    }
    let mut requests = Vec::new();
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "native-boolean-branch-oracle.cinch");
        assert_eq!(actual.is_ok(), accepted, "Native reference {name}: {actual:?}");
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":\"\",\"wanted\":{}}}", quote(name), quote(&source), if accepted { 0 } else { 4 }));
    }
    let base = std::fs::read(dir.join("interfaces.json")).unwrap();
    for name in ["custom", "stronger", "measure"] {
        std::fs::write(dir.join(format!("{name}-interfaces.json")), &base).unwrap();
    }
    std::fs::write(dir.join("cases.json"), format!("[{}]", requests.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "boolean-branches").expect("compile actual Cinch branch checker");
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "guarded-call-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
