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
fn compiled_cinch_uses_actual_guarded_bodies_for_dependent_bounds() {
    let dir = common::project("guarded_call_consumer", &[
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
        ("byte offset result equation", "offset :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> refined (r + extra + bytes-len x == bytes-len out) Nat\n  offset extra x out = bytes-len out - (extra + bytes-len x)".to_string(), true),
        ("wrong byte offset result equation", "offset :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> refined (r + extra + bytes-len x == bytes-len out + 1) Nat\n  offset extra x out = bytes-len out - (extra + bytes-len x)".to_string(), false),
        ("raw byte copy inline", "copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)".to_string(), true),
        ("raw byte copy strict offset", "copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) in copy-bytes-into out x offset 0 (bytes-len x)".to_string(), true),
        ("refined byte copy inline", "copy :: (extra :: Nat) -> (x :: Bytes) -> refined (extra + bytes-len x <= bytes-len out) Bytes -> Unit\n  copy extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)".to_string(), true),
        ("refined byte copy strict offset", "copy :: (extra :: Nat) -> (x :: Bytes) -> refined (extra + bytes-len x <= bytes-len out) Bytes -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) in copy-bytes-into out x offset 0 (bytes-len x)".to_string(), true),
        ("byte copy missing guard", "bad :: (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  bad extra x out = copy-bytes-into out x (bytes-len out - (extra + bytes-len x)) 0 (bytes-len x)".to_string(), false),
        ("byte copy wrong offset", "copy :: using (extra + bytes-len x <= bytes-len out) => (extra :: Nat) -> (x :: Bytes) -> (out :: Bytes) -> Unit\n  copy extra x out = let offset = bytes-len out - (extra + bytes-len x) + 1 in copy-bytes-into out x offset 0 (bytes-len x)".to_string(), false),
        ("function let cannot capture argument", "flag-result :: Bool -> Int\n  flag-result value = if value then 1 else 0\n  use-flag :: Bool -> Int\n  use-flag value = (let value = 0 in flag-result) value".to_string(), true),
        ("function let cannot repair wrong argument", "flag-result :: Bool -> Int\n  flag-result value = if value then 1 else 0\n  bad :: Int -> Int\n  bad value = (let value = True in flag-result) value".to_string(), false),
        ("strict let retains guarded equation", "offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = let result = total - extra in result".to_string(), true),
        ("strict let cannot invent result equation", "bad :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total + 1) Nat\n  bad total extra = let result = total - extra in result".to_string(), false),
        ("actual subtraction result equation", "offset :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  offset total extra = total - extra".to_string(), true),
        ("wrong subtraction result equation", "bad :: using (extra <= total) => (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total + 1) Nat\n  bad total extra = total - extra".to_string(), false),
        ("guarded branch keeps actual equation", "offset :: (total :: Nat) -> (extra :: Nat) -> Option (refined (r + extra == total) Nat)\n  offset total extra = if extra <= total then Some (total - extra) else None".to_string(), true),
        ("guarded body cannot excuse missing call proof", "bad :: (total :: Nat) -> (extra :: Nat) -> refined (r + extra == total) Nat\n  bad total extra = total - extra".to_string(), false),
        ("guarded Nat predecessor", "previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else n - 1".to_string(), true),
        ("missing Nat subtraction precondition", "bad :: Nat -> Nat\n  bad n = n - 1".to_string(), false),
        ("literal valid subtraction", "value :: Nat\n  value = 7 - 2".to_string(), true),
        ("literal underflow refused", "bad :: Nat\n  bad = 1 - 2".to_string(), false),
        ("zero exact subtraction", "value :: Nat\n  value = 0 - 0".to_string(), true),
        ("wrong branch cannot discharge", "bad :: Nat -> Nat\n  bad n = if n == 0 then n - 1 else 0".to_string(), false),
        ("two runtime arguments with ordering", "difference :: Nat -> Nat -> Nat\n  difference x y = if y <= x then x - y else 0".to_string(), true),
        ("lexical unrestricted dictionary wins", "generic :: using (sub :: Sub Nat) => Nat -> Nat\n  generic n = n - 1".to_string(), true),
        ("conditional dictionary cannot escape", "generic :: forall a. using (sub :: Sub a) => a -> a -> a\n  generic x y = x - y\n  bad :: Nat -> Nat\n  bad n = if n == 0 then 0 else generic n 1".to_string(), false),
        ("conditional partial application cannot erase guard", "bad :: Nat -> Nat\n  bad = (-) 3".to_string(), false),
        ("shadowed value cannot borrow ordering", "bad :: Nat -> Nat\n  bad n = if n == 0 then 0 else let n = (0 :: Nat) in n - 1".to_string(), false),
        ("conditional call preserves effects", "previous :: forall r. (Unit ->{IO r} Nat) -> Unit ->{IO r} Nat\n  previous f u = let n = f u in if n == 0 then 0 else n - 1".to_string(), true),
        ("conditional call cannot erase effects", "bad :: forall r. (Unit ->{IO r} Nat) -> Unit -> Nat\n  bad f u = let n = f u in if n == 0 then 0 else n - 1".to_string(), false),
        ("ordinary Int subtraction stays unrestricted", "previous :: Int -> Int\n  previous n = n - 1".to_string(), true),
    ];
    let mut requests = Vec::new();
    let mut oracle_errors = Vec::new();
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "native-guarded-call-oracle.cinch");
        if actual.is_ok() != accepted { oracle_errors.push(format!("{name}: {}", actual.err().map_or("accepted".into(), |e| e.message))); }
        let element = "";
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":{},\"wanted\":{}}}", quote(name), quote(&source), quote(element), if accepted { 0 } else { 4 }));
    }

    // Imported user classes exercise canonical checked heads independently
    // of the built-in Sub spelling. Private measures retain declarations and
    // bodies while granting no public source name.
    let difference = "module A where\n  export Difference, subtract\n  signature Difference a where\n    subtract :: a -> a -> a\n  implicit instance Difference_Nat :: Difference Nat where\n    subtract :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat\n    subtract x y = x\n";
    let stronger = difference.replace("y <= x", "y + 1 <= x");
    let measured = "module A where\n  export Use, use\n  size :: Nat -> Nat\n  size n = n\n  signature Use a where\n    use :: a -> a\n  implicit instance Use_Nat :: Use Nat where\n    use :: using (size x > 0) => (x :: Nat) -> Nat\n    use x = x\n";
    for (variant, library) in [("custom", difference), ("stronger", stronger.as_str()), ("measure", measured)] {
        let (_, variants) = cinch::driver::check_sources_ifaces(&format!("{library}\nmodule Main where\n  import A\n  main :: Int\n  main = 0\n"), "conditional-variant-provider.cinch").unwrap();
        let folder = dir.join(variant); std::fs::create_dir_all(&folder).unwrap();
        let mut entries = Vec::new();
        for iface in variants.iter().filter(|iface| iface.module != "Main") {
            let path = folder.join(format!("{}.cinchif", iface.module)); std::fs::write(&path, iface.to_json()).unwrap();
            entries.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
        }
        entries.sort();
        std::fs::write(dir.join(format!("{variant}-interfaces.json")), format!("{{\"schema\":1,\"interfaces\":[{}]}}", entries.join(","))).unwrap();
    }
    for (name, variant, library, body, accepted) in [
        ("own declaring class and slot", "custom", difference, "value :: Nat\n  value = subtract 0 0", true),
        ("same slot stronger actual precondition", "stronger", stronger.as_str(), "bad :: Nat\n  bad = subtract 0 0", false),
        ("own class guarded arguments", "custom", difference, "previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else subtract n 1", true),
        ("private measure declaration closure", "measure", measured, "value :: Int\n  value = 0", true),
        ("private measure cannot become source name", "measure", measured, "bad :: Nat\n  bad = size 1", false),
    ] {
        let source = format!("module Main where\n  import A\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&format!("{library}\n{source}"), "native-conditional-variant-oracle.cinch");
        if actual.is_ok() != accepted { oracle_errors.push(format!("{name}: {}", actual.err().map_or("accepted".into(), |e| e.message))); }
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":{},\"wanted\":{}}}", quote(name), quote(&source), quote(variant), if accepted { 0 } else { 4 }));
    }
    assert!(oracle_errors.is_empty(), "Native oracle differences: {oracle_errors:#?}");
    std::fs::write(dir.join("cases.json"), format!("[{}]", requests.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "guarded-calls").expect("compile independent Cinch refinement constructor elaborator");
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "guarded-call-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
