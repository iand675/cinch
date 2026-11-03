//! Explicit refinement introduction retains checked erased evidence and payload identity.
mod common;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
#[test]
fn compiled_cinch_keeps_argument_witness_scopes_and_effects() {
    let dir = common::project("argument_scope_consumer", &[
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
    let cases = vec![
        ("bounded byte length in nested guarded subtraction", "probe :: Nat -> Bytes -> Nat\n  probe remaining bytes = let count = bytes-len bytes in if count + 1 <= remaining then remaining - (count + 1) else 0", true),
        ("bounded byte length with explicit sum binding", "probe :: Nat -> Bytes -> Nat\n  probe remaining bytes = let count = bytes-len bytes in let width = count + 1 in if width <= remaining then remaining - width else 0", true),
        ("bounded byte length in direct guarded subtraction", "probe :: Nat -> Bytes -> Nat\n  probe remaining bytes = let count = bytes-len bytes in if count <= remaining then remaining - count else 0", true),
        ("bounded byte length does not prove reversed subtraction", "probe :: Nat -> Bytes -> Nat\n  probe remaining bytes = let count = bytes-len bytes in if count + 1 <= remaining then count - (remaining + 1) else 0", false),
        ("sum does not retain byte length upper bound", "need :: InRange 0 9223372036854775807 Nat -> Nat\n  need n = n\n  probe :: Bytes -> Nat\n  probe bytes = let count = bytes-len bytes in need (count + 1)", false),
        ("nested remainder witnesses", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: Nat -> Nat\n  probe n = need (nat-rem n 16)", true),
        ("explicit remainder binding", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: Nat -> Nat\n  probe n = let remainder = nat-rem n 16 in need remainder", true),
        ("wrong remainder bound", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: Nat -> Nat\n  probe n = need (nat-rem n 32)", false),
        ("zero remainder divisor", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: Nat -> Nat\n  probe n = need (nat-rem n 0)", false),
        ("shadowed source name", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: Nat -> Nat\n  probe n = let remainder = nat-rem n 16 in let n = 32 in need remainder", true),
        ("effectful function prefix", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: forall r. (Unit ->{IO r} (Nat -> Nat)) -> Unit ->{IO r} Nat\n  probe provider u = provider u (nat-rem 7 16)", true),
        ("cannot erase function effects", "import Data.Nat (nat-rem, nat-div)\n  need :: using (n < 16) => (n :: Nat) -> Nat\n  need n = n\n  probe :: forall r. (Unit ->{IO r} (Nat -> Nat)) -> Unit -> Nat\n  probe provider u = provider u (nat-rem 7 16)", false),
    ];
    let mut requests = Vec::new();
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "native-argument-scope-oracle.cinch");
        assert_eq!(actual.is_ok(), accepted, "Native reference {name}: {actual:?}");
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":\"\",\"wanted\":{}}}", quote(name), quote(&source), if accepted { 0 } else { 4 }));
    }
    let base = std::fs::read(dir.join("interfaces.json")).unwrap();
    for name in ["custom", "stronger", "measure"] {
        std::fs::write(dir.join(format!("{name}-interfaces.json")), &base).unwrap();
    }
    std::fs::write(dir.join("cases.json"), format!("[{}]", requests.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "argument-scopes").expect("compile actual Cinch argument-scope checker");
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "guarded-call-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
