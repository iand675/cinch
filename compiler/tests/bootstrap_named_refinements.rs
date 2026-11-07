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
fn named_refinements_preserve_scope_bounds_and_proof_obligations() {
    let dir = common::project("named_refinements", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.NamedRefinements.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "named-refinements").expect("build the named refinement checker");
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    let cases = [
        ("generic structural direct result keeps NonZero", "type NZ = refine (x :: Int) with NonZero x\n  identity :: forall a. a -> a\n  identity value = value\n  copy :: NZ -> NZ\n  copy value = identity value", 0, ""),
        ("generic structural nested direct result keeps NonZero", "type NZ = refine (x :: Int) with NonZero x\n  identity :: forall a. a -> a\n  identity value = value\n  copy :: NZ -> NZ\n  copy value = identity (identity value)", 0, ""),
        ("generic structural direct result cannot forge NonZero", "type NZ = refine (x :: Int) with NonZero x\n  identity :: forall a. a -> a\n  identity value = value\n  bad :: Int -> NZ\n  bad value = identity value", 4, ""),
        ("generic structural extracted result keeps NonZero", "data Cell a where\n    Cell :: a -> Cell a\n  type NZ = refine (x :: Int) with NonZero x\n  get :: forall a. Cell a -> a\n  get cell = case cell of Cell value -> value\n  copy :: Cell NZ -> NZ\n  copy cell = get cell", 0, ""),
        ("generic structural direct result keeps mixed predicate", "type NZRange = refine (x :: Int) with NonZero x && x < 10\n  identity :: forall a. a -> a\n  identity value = value\n  copy :: NZRange -> NZRange\n  copy value = identity value", 0, ""),
        ("generic numeric direct result still projects to Int", "type Positive = refine (x :: Int) with x > 0\n  identity :: forall a. a -> a\n  identity value = value\n  copy :: Positive -> Int\n  copy value = identity value", 0, ""),
        ("generic structural list keeps NonZero", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  singleton :: NZ -> Sequence NZ\n  singleton value = let values = SCons value SNil in values", 0, ""),
        ("generic structural identity keeps NonZero", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  identity :: forall a. a -> a\n  identity value = value\n  copy :: NZ -> NZ\n  copy value = let result = identity value in result", 0, ""),
        ("generic structural list needs explicit projection", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  bad :: NZ -> Sequence Int\n  bad value = SCons value SNil", 4, ""),
        ("generic structural list accepts explicit projection", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  plain :: NZ -> Sequence Int\n  plain value = SCons (unrefine value) SNil", 0, ""),
        ("generic structural result needs explicit projection", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  bad :: NZ -> Int\n  bad value = value", 4, ""),
        ("generic structural list cannot forge NonZero", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  bad :: Int -> Sequence NZ\n  bad value = let values = SCons value SNil in values", 4, ""),
        ("generic structural identity cannot forge NonZero", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZ = refine (x :: Int) with NonZero x\n  identity :: forall a. a -> a\n  identity value = value\n  bad :: Int -> NZ\n  bad value = let copy = identity value in copy", 4, ""),
        ("generic numeric range still widens an inferred list", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type Positive = refine (x :: Int) with x > 0\n  plain :: Positive -> Sequence Int\n  plain value = let values = SCons value SNil in values", 0, ""),
        ("generic numeric range still widens an open identity", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type Positive = refine (x :: Int) with x > 0\n  identity :: forall a. a -> a\n  identity value = value\n  plain :: Positive -> Int\n  plain value = let copy = identity value in copy", 0, ""),
        ("generic numeric inferred list cannot claim its former range", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type Positive = refine (x :: Int) with x > 0\n  bad :: Positive -> Sequence Positive\n  bad value = let values = SCons value SNil in values", 4, ""),
        ("generic mixed structural predicate keeps its point", "data Sequence a where\n    SNil :: Sequence a\n    SCons :: a -> Sequence a -> Sequence a\n  type NZRange = refine (x :: Int) with NonZero x && x < 10\n  singleton :: NZRange -> Sequence NZRange\n  singleton value = let values = SCons value SNil in values", 0, ""),
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
        if name.starts_with("generic ") {
            let oracle = cinch::driver::check_sources(&source, "generic-refinement-oracle.cinch");
            assert_eq!(oracle.is_ok(), wanted == 0, "Native generic refinement oracle: {name}\n{oracle:?}");
        }
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
        ("Bounds.cinch", "module Bounds where\n  export Positive, Window\n  type Positive = refine (x :: Nat) with x > 0\n  type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n"),
        ("Facade.cinch", "module Facade where\n  import Bounds (Positive as Pos, Window)\n  export Pos, Window\n"),
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
