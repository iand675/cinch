//! Named refinements expand their defining templates with actual typed arguments.
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
fn structural_nonzero_source_preserves_value_identity_and_contracts() {
    let dir = common::project("nonzero_source", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.NonZeroSource.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "nonzero-source").expect("build the named refinement checker");
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    let cases = [
        ("structural predicate ignores same-spelled refinement template", "type NonZero a = refine (n :: Nat) with n > 5\n  one :: refined (NonZero n) Nat\n  one = 1", 0, ""),
        ("carrier application retains same-spelled defining template", "type NonZero a = refine (n :: Nat) with n > 5\n  bad :: NonZero Nat\n  bad = 1", 4, "refinement predicate is not established"),
        ("fixed integer predicate keeps actual carrier", "need :: using (NonZero n) => (n :: I64) -> I64\n  need n = n", 0, ""),
        ("float argument cannot supply structural premise", "need :: using (NonZero n) => (n :: F64) -> F64\n  need n = n", 4, "NonZero needs a checked mathematical carrier"),
        ("structural named alias Nat", "type NonZero a = refine (n :: a) with NonZero n\n  one :: NonZero Nat\n  one = 1", 0, ""),
        ("structural named alias Int", "type NonZero a = refine (n :: a) with NonZero n\n  one :: NonZero Int\n  one = -1", 0, ""),
        ("structural alias refuses zero", "type NonZero a = refine (n :: a) with NonZero n\n  bad :: NonZero Nat\n  bad = 0", 4, "refinement predicate is not established"),
        ("anonymous NonZero", "one :: refined (NonZero n) Nat\n  one = 1", 0, ""),
        ("anonymous refuses zero", "bad :: refined (NonZero n) Nat\n  bad = 0", 4, "refinement predicate is not established"),
        ("NonZero result binder avoids alias recursion", "type NonZero a = refine (n :: a) with NonZero n\n  one :: refined (NonZero n) Nat\n  one = 1", 0, ""),
        ("NonZero contract discharges literal", "need :: using (NonZero n) => (n :: Nat) -> Nat\n  need n = n\n  answer :: Nat\n  answer = need 1", 0, ""),
        ("NonZero contract rejects zero", "need :: using (NonZero n) => (n :: Nat) -> Nat\n  need n = n\n  bad :: Nat\n  bad = need 0", 4, "function precondition is not established"),
        ("numeric guard proves structural obligation", "need :: using (NonZero n) => (n :: Nat) -> Nat\n  need n = n\n  guarded :: Nat -> Nat\n  guarded n = if n /= 0 then need n else 0", 0, ""),
        ("guard on other identity refuses obligation", "need :: using (NonZero n) => (n :: Nat) -> Nat\n  need n = n\n  bad :: Nat -> Nat -> Nat\n  bad n m = if n /= 0 then need m else 0", 4, "function precondition is not established"),
        ("structural premise proves numeric result", "positive :: using (NonZero n) => (n :: Nat) -> refined (r > 0) Nat\n  positive n = n", 0, ""),
        ("NonZero carrier projection", "type NonZero a = refine (n :: a) with NonZero n\n  forget :: NonZero Nat -> Nat\n  forget n = n", 0, ""),
        ("comparison evidence introduces structural alias", "type NonZero a = refine (n :: a) with NonZero n\n  guarded :: Nat -> NonZero Nat\n  guarded n = if n > 0 then n else 1", 0, ""),
        ("structural alias weakens to comparison", "type NonZero a = refine (n :: a) with NonZero n\n  weaken :: NonZero Nat -> refined (r > 0) Nat\n  weaken n = n", 0, ""),
        ("comparison refines to structural alias", "type NonZero a = refine (n :: a) with NonZero n\n  strengthen :: refined (r > 0) Nat -> NonZero Nat\n  strengthen n = n", 0, ""),
        ("NonZero Int does not imply positive", "positive :: using (NonZero n) => (n :: Int) -> refined (r > 0) Int\n  positive n = n", 4, "refinement predicate is not established"),
        ("float structural predicate refused", "type Bad = refine (x :: F64) with NonZero x", 4, "NonZero needs a checked mathematical carrier"),
        ("Boolean structural predicate refused", "type Bad = refine (x :: Bool) with NonZero x", 4, "NonZero needs a checked mathematical carrier"),
        ("NonZero argument cannot be a type", "bad :: using (NonZero Nat) => Nat\n  bad = 1", 4, "source index has no checked local binding"),
        ("unbound structural argument refused", "type Bad = refine (x :: Nat) with NonZero other", 4, "predicate must mention its binder"),
        ("unchecked call stays opaque", "type Bad = refine (x :: Nat) with NonZero (mystery x)", 4, "named refinement predicate needs checked scope"),
        ("ignored structural refinement remains strict", "bad :: Nat\n  bad = let ignored = (0 :: refined (NonZero n) Nat) in 1", 4, "refinement predicate is not established"),
        ("NonZero conjunction", "one :: refined (NonZero n && n < 2) Nat\n  one = 1", 0, ""),
        ("NonZero conjunction retains bounds", "bad :: refined (NonZero n && n < 1) Nat\n  bad = 1", 4, "refinement predicate is not established"),
        ("zero parameter alias", "type Positive = refine (x :: Nat) with x > 0\n  one :: Positive\n  one = 1", 0, ""),
        ("alias rejects zero", "type Positive = refine (x :: Nat) with x > 0\n  bad :: Positive\n  bad = 0", 4, "refinement predicate is not established"),
        ("generic range over Nat", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  one :: Window 0 10 Nat\n  one = 1", 0, ""),
        ("generic range over Int", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  one :: Window (-10) 0 Int\n  one = -1", 0, ""),
        ("invalid interval", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  bad :: Window 10 0 Nat\n  bad = 1", 4, "refinement predicate is not established"),
        ("out of bounds", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  bad :: Window 0 10 Nat\n  bad = 11", 4, "refinement predicate is not established"),
        ("numeric alias projection", "type Positive = refine (x :: Nat) with x > 0\n  plain :: Positive -> Nat\n  plain x = x", 0, ""),
        ("alias weakening", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  widen :: Window 2 8 Nat -> Window 0 10 Nat\n  widen n = n", 0, ""),
        ("alias cannot strengthen", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  narrow :: Window 0 10 Nat -> Window 2 8 Nat\n  narrow n = n", 4, "refinement predicate is not established"),
        ("bounds use actual source identities", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  within :: using (lo <= n, n <= hi) => (lo :: Int) -> (hi :: Int) -> (n :: Nat) -> Window lo hi Nat\n  within lo hi n = n", 0, ""),
        ("another variable cannot supply membership", "type Positive = refine (x :: Nat) with x > 0\n  bad :: Nat -> Nat -> Positive\n  bad n m = if n > 0 then m else 1", 4, "refinement predicate is not established"),
        ("duplicate binder", "type Bad a = refine (a :: Nat) with a > 0", 4, "duplicate refinement binder"),
        ("duplicate parameter", "type Bad a a = refine (x :: Nat) with x > 0", 4, "duplicate quantified parameter"),
        ("unbound predicate variable", "type Bad = refine (x :: Nat) with x > other", 4, "named refinement predicate needs checked scope"),
        ("unused alias still checks scope", "type Bad a = refine (x :: a) with x > other", 4, "named refinement predicate needs checked scope"),
        ("binder must occur", "type Bad = refine (x :: Nat) with True", 4, "predicate must mention its binder"),
        ("unknown operator remains opaque", "type Bad = refine (x :: Nat) with x / 1 > 0", 4, "named refinement predicate needs checked scope"),
        ("unsaturated alias refused", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  bad :: Window\n  bad = 0", 4, "unsaturated refinement alias"),
        ("recursive alias refused", "type Loop = refine (x :: Loop) with x > 0", 4, "recursive refinement alias expansion"),
        ("mutual alias cycle refused", "type First = refine (x :: Second) with x > 0\n  type Second = refine (x :: First) with x > 0", 4, "recursive refinement alias expansion"),
        ("float comparison is opaque", "type Bad = refine (x :: F64) with x > 0", 4, "proof predicate needs checked mathematical carriers"),
        ("ignored invalid alias ascription stays strict", "type Positive = refine (x :: Nat) with x > 0\n  bad :: Int\n  bad = let ignored = (0 :: Positive) in 0", 4, "refinement predicate is not established"),
        ("generic range checks actual carrier", "type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  bad :: Window 0 10 Bool\n  bad = True", 4, "proof predicate needs checked mathematical carriers"),
        ("template parameters do not capture globals", "lo :: Nat\n  lo = 999\n  type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  one :: Window 0 10 Nat\n  one = 1", 0, ""),
    ];
    for (name, body, wanted, diagnostic) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if wanted == 0 { assert!(report.is_empty(), "{name}: {report}"); }
        else { assert!(report.contains(diagnostic), "{name}: expected {diagnostic}: {report}"); }
    }
    // The same executable now consumes actual native-produced contracts,
    // including a facade whose public name differs from the definition.
    let producer = common::project("named_refinement_producer", &[
        ("Bounds.cinch", "module Bounds where\n  export Positive, Window, NonZero\n  type Positive = refine (x :: Nat) with x > 0\n  type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  type NonZero a = refine (n :: a) with NonZero n\n"),
        ("Facade.cinch", "module Facade where\n  import Bounds (Positive as Pos, Window, NonZero as Nz)\n  export Pos, Window, Nz\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&producer, "Main.cinch").expect("emit checked defining refinement contracts");
    let mut rows = Vec::new();
    for file in std::fs::read_dir(producer.join("build")).unwrap() {
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
    for (name, body, wanted) in [
        ("renamed structural import", "import Facade (Nz)\n  one :: Nz Nat\n  one = 1", 0),
        ("renamed import refuses zero", "import Facade (Nz)\n  bad :: Nz Nat\n  bad = 0", 4),
        ("qualified structural import", "import Facade as B\n  one :: B.Nz Int\n  one = -1", 0),
        ("imported structural evidence projects", "import Facade (Nz)\n  forget :: Nz Nat -> Nat\n  forget n = n", 0),
        ("facade alias keeps defining predicate", "import Facade (Pos)\n  one :: Pos\n  one = 1", 0),
        ("facade alias refuses zero", "import Facade (Pos)\n  bad :: Pos\n  bad = 0", 4),
        ("qualified facade alias", "import Facade as B\n  one :: B.Pos\n  one = 1", 0),
        ("imported generic range", "import Facade (Window)\n  one :: Window 0 10 Nat\n  one = 1", 0),
        ("imported range refuses bad value", "import Facade (Window)\n  bad :: Window 0 10 Nat\n  bad = 11", 4),
        ("imported range weakens with evidence", "import Facade (Window)\n  widen :: Window 2 8 Nat -> Window 0 10 Nat\n  widen n = n", 0),
    ] {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}\n{source}\n{report}");
        if wanted == 0 { assert!(report.is_empty(), "{name}: {report}"); }
        else { assert!(report.contains("refinement predicate is not established"), "{name}: {report}"); }
    }
    common::cleanup(&producer);
    common::cleanup(&dir);
}
