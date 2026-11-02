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
fn compiled_cinch_checks_explicit_refinement_proofs_and_payload_identity() {
    let dir = common::project("refinement_constructor_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RefinementConstructors.cinch")),
    ]);

    let (_, interfaces) = cinch::driver::check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "refinement-constructor-base.cinch").unwrap();
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
        ("imported Int constructor", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> Some (NonZero n w); No other -> None".to_string(), true),
        ("imported Nat constructor", "make :: Nat -> Option (NonZero Nat)\n  make n = case decide (NonZero n) of Yes w -> Some (NonZero n w); No other -> None".to_string(), true),
        ("actual int-div caller", "divide :: Int -> Int\n  divide n = case decide (NonZero n) of Yes w -> int-div 1 (NonZero n w); No other -> 0".to_string(), true),
        ("wrong payload identity", "bad :: Int -> Int -> Option (NonZero Int)\n  bad n m = case decide (NonZero n) of Yes w -> Some (NonZero m w); No other -> None".to_string(), false),
        ("false branch witness", "bad :: Int -> Option (NonZero Int)\n  bad n = case decide (NonZero n) of Yes w -> None; No w -> Some (NonZero n w)".to_string(), false),
        ("ordinary value is not evidence", "bad :: Int -> NonZero Int\n  bad n = NonZero n True".to_string(), false),
        ("zero payload cannot borrow witness", "bad :: Int -> Option (NonZero Int)\n  bad n = case decide (NonZero n) of Yes w -> Some (NonZero (0 :: Int) w); No other -> None".to_string(), false),
        ("proof identity survives shadowing", "bad :: Int -> Option (NonZero Int)\n  bad n = case decide (NonZero n) of Yes w -> let n = (0 :: Int) in Some (NonZero n w); No other -> None".to_string(), false),
        ("out of scope witness", "bad :: Int -> NonZero Int\n  bad n = NonZero n missing".to_string(), false),
        ("proof cannot be a runtime result", "bad :: Int -> Int\n  bad n = case decide (NonZero n) of Yes w -> w; No other -> 0".to_string(), false),
        ("custom defining refinement", "type Positive = refine (value :: Int) with value > 0\n  make :: Int -> Option Positive\n  make n = case decide (n > 0) of Yes w -> Some (Positive n w); No other -> None".to_string(), true),
        ("same name changed defining predicate", "type Positive = refine (value :: Int) with value > 5\n  bad :: Int -> Option Positive\n  bad n = case decide (n > 0) of Yes w -> Some (Positive n w); No other -> None".to_string(), false),
        ("custom proof with actual threshold", "type Positive = refine (value :: Int) with value > 5\n  make :: Int -> Option Positive\n  make n = case decide (n > 5) of Yes w -> Some (Positive n w); No other -> None".to_string(), true),
        ("ambient proof does not excuse wrong explicit evidence", "bad :: Int -> Int -> Option (NonZero Int)\n  bad n m = case decide (NonZero n) of Yes good -> (case decide (NonZero m) of Yes wrong -> Some (NonZero n wrong); No other -> None); No other -> None".to_string(), false),
        ("explicit evidence matches despite another witness", "make :: Int -> Int -> Option (NonZero Int)\n  make n m = case decide (NonZero n) of Yes good -> (case decide (NonZero m) of Yes other -> Some (NonZero n good); No other -> None); No other -> None".to_string(), true),
        ("local value alias retains actual identity", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> let alias = n in Some (NonZero alias w); No other -> None".to_string(), true),
        ("expected Nat literal constructor", "make :: Option (NonZero Nat)\n  make = case decide (NonZero (1 :: Nat)) of Yes w -> Some (NonZero 1 w); No other -> None".to_string(), true),
        ("literal zero still rejects witness for one", "bad :: Option (NonZero Nat)\n  bad = case decide (NonZero (1 :: Nat)) of Yes w -> Some (NonZero 0 w); No other -> None".to_string(), false),
        ("constructor preserves payload effects", "make :: forall r. (Unit ->{IO r} Int) -> Unit ->{IO r} Option (NonZero Int)\n  make f u = let n = f u in case decide (NonZero n) of Yes w -> Some (NonZero n w); No other -> None".to_string(), true),
        ("constructor cannot erase payload effects", "bad :: forall r. (Unit ->{IO r} Int) -> Unit -> Option (NonZero Int)\n  bad f u = let n = f u in case decide (NonZero n) of Yes w -> Some (NonZero n w); No other -> None".to_string(), false),
    ];
    let mut requests = Vec::new();
    let mut oracle_errors = Vec::new();
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "native-refinement-constructor-oracle.cinch");
        if actual.is_ok() != accepted { oracle_errors.push(format!("{name}: {}", actual.err().map_or("accepted".into(), |e| e.message))); }
        let element = "";
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":{},\"wanted\":{}}}", quote(name), quote(&source), quote(element), if accepted { 0 } else { 4 }));
    }
    assert!(oracle_errors.is_empty(), "Native oracle differences: {oracle_errors:#?}");
    std::fs::write(dir.join("cases.json"), format!("[{}]", requests.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "refinement-constructors").expect("compile independent Cinch refinement constructor elaborator");
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "refinement-constructor-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
