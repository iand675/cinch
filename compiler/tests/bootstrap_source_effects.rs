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
        ("direct do named binding", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { x <- 41; x + 1 }", 0),
        ("direct do layout", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do\n    x <- 41\n    x + 1", 0),
        ("direct do shadowing", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { x <- 1; x <- x + 1; x }", 0),
        ("direct do effects retained", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Unit ->{Marker} Int\n  value u = do { marker u; 42 }", 0),
        ("direct do effects cannot escape", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Unit -> Int\n  value u = do { marker u; 42 }", 4),
        ("direct do pure let polymorphism", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { let { identity x = x }; if identity True then identity 42 else 0 }", 0),
        ("direct do binding is monomorphic", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { identity <- (\\x -> x); if identity True then identity 42 else 0 }", 4),
        ("direct do let remains pure", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Unit ->{Marker} Int\n  value u = do { let { ignored = marker u }; 42 }", 4),
        ("direct do binding cannot escape scope", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = (do { x <- 41; x }) + x", 4),
        ("direct do refutable binding rejected", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { True <- False; 42 }", 4),
        ("direct do must end in expression", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do { x <- 41 }", 4),
        ("direct do empty block rejected", "effect Marker where\n    marker :: Unit ->{Marker} Unit\n  value :: Int\n  value = do {}", 4),
        ("inferred open effect row", "call :: forall e. (Unit ->{|e} Unit) ->{|e} Unit\n  call f = f ()", 0),
        ("explicit open effect row", "call :: forall (e :: Row). (Unit ->{|e} Unit) ->{|e} Unit\n  call f = f ()", 0),
        ("open effect row with labels", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  call :: forall e. (Unit ->{|e} Unit) ->{Signal|e} Unit\n  call f = do { signal (); f () }", 0),
        ("open effect row cannot erase tail", "call :: forall e. (Unit ->{|e} Unit) -> Unit\n  call f = f ()", 4),
        ("open effect row cannot substitute identity", "call :: forall e f. (Unit ->{|e} Unit) ->{|f} Unit\n  call f = f ()", 4),
        ("open effect row explicit Type refused", "call :: forall (e :: Type). (Unit ->{|e} Unit) ->{|e} Unit\n  call f = f ()", 4),
        ("open effect row explicit Region refused", "call :: forall (e :: Region). (Unit ->{|e} Unit) ->{|e} Unit\n  call f = f ()", 4),
        ("open effect row unbound refused", "call :: (Unit ->{|e} Unit) ->{|e} Unit\n  call f = f ()", 4),
        ("open effect row conflicting type use refused", "call :: forall e. e -> (Unit ->{|e} Unit) ->{|e} Unit\n  call x f = f ()", 4),
        ("open effect row cannot hide named effect", "effect Signal where\n    signal :: Unit ->{Signal} Unit\n  call :: forall e. (Unit ->{|e} Unit) ->{|e} Unit\n  call f = do { signal (); f () }", 4),
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
        // This runner independently requires a retained source effect declaration.
        // Row-only callbacks still need that fixture invariant, even when pure.
        let body = if name.starts_with("inferred open") || name.starts_with("explicit open") || name.starts_with("open effect row") {
            format!("effect FixtureMarker where\n    fixture-marker :: Unit ->{{FixtureMarker}} Unit\n  {body}")
        } else { body.to_owned() };
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        if name.starts_with("direct do") || name.starts_with("inferred open") || name.starts_with("explicit open") || name.starts_with("open effect row") {
            let reference = cinch::driver::check_sources(&source, "native-direct-do-oracle.cinch");
            assert_eq!(reference.is_ok(), wanted == 0, "Native reference {name}: {reference:?}");
        }
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "effect-source-report.txt");
        assert_eq!(result.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        println!("checked source effect case passed: {name}");
    }
    common::cleanup(&dir);
}
