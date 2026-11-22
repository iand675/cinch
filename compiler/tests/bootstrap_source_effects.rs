//! Source effect declarations retain checked owner labels and operation slots.
mod common;
#[test]
fn source_effect_declarations_and_calls_retain_checked_contracts() {
    let dir = common::project("source_effects", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceEffects.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-effects").expect("build actual Cinch source effect checker");
    for (name, body, wanted) in [
        ("simple operation", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  call :: Unit ->{Signal} Unit\n  call u = signal u", 0),
        ("polymorphic parse operation", "effect Parse where\n    fail :: forall a. Int ->{Parse} a\n    note :: Int ->{Parse} Unit\n  call :: Unit ->{Parse} Int\n  call u = fail 1", 0),
        ("curried operation", "effect Log where\n    log :: Int -> Unit ->{Log} Unit\n  call :: Unit ->{Log} Unit\n  call u = log 1 u", 0),
        ("first class operation", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  keep :: (Unit ->{Signal} Unit) -> (Unit ->{Signal} Unit)\n  keep f = f\n  value :: Unit ->{Signal} Unit\n  value = keep signal", 0),
        ("two source effects", "effect A where\n    a :: Unit ->{A} Unit\n  effect B where\n    b :: Unit ->{B} Unit\n  call :: Unit ->{A, B} Unit\n  call u = let ignored = a u in b u", 0),
        ("declared region parameter", "effect State (r :: Region) where\n    get :: Unit ->{State r} Int\n  call :: forall (r :: Region). Unit ->{State r} Int\n  call u = get u", 0),
        ("effect and operation quantifiers", "effect State (r :: Region) where\n    get :: forall a. a ->{State r} a\n  call :: forall (r :: Region). Int ->{State r} Int\n  call n = get n", 0),
        ("type parameter", "effect State a where\n    put :: a ->{State a} Unit\n  call :: Int ->{State Int} Unit\n  call n = put n", 0),
        ("dependent operation result", "effect Need where\n    need :: (n :: Nat) ->{Need} refined (out == n) Nat\n  keep :: (n :: Nat) ->{Need} refined (out == n) Nat\n  keep n = need n", 0),
        ("pure operation rejected", "effect Signal where\n    signal :: Unit -> Unit", 4),
        ("wrong owner label", "effect Signal where\n    signal :: Unit ->{Diverge} Unit", 4),
        ("extra operation effect", "effect Signal where\n    signal :: Unit ->{Signal, Diverge} Unit", 4),
        ("row on earlier arrow", "effect Signal where\n    signal :: Unit ->{Signal} Unit -> Unit", 4),
        ("operation cannot also be defined", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  signal u = u", 4),
        ("call cannot erase effect", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  bad :: Unit -> Unit\n  bad u = signal u", 4),
        ("effect is not a value type", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  bad :: Signal\n  bad = 0", 4),
        ("data is not an effect", "data Token where\n    Token :: Token\n  effect Signal where\n    signal :: Unit ->{Token} Unit", 4),
        ("wrong region kind", "effect State (r :: Region) where\n    get :: Unit ->{State Int} Int", 4),
        ("duplicate operation quantifier", "effect State a where\n    get :: forall a. a ->{State a} a", 4),
        ("wrong region identity", "effect State (r :: Region) where\n    get :: Unit ->{State r} Int\n  bad :: forall (r :: Region) (s :: Region). (Unit ->{State r} Int) -> Unit ->{State s} Int\n  bad f u = f u", 4),
    ] {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "effect-source-report.txt");
        assert_eq!(result.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        println!("checked source effect case passed: {name}");
    }
    common::cleanup(&dir);
}
