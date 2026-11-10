//! Source handler checks executed by the compiled Cinch elaborator.
//! The pinned Native reference has separate known limitations: type-parameter
//! labels and explicit Region syntax, and accepts some continuation aliases.
//! No aggregate Native/Cinch parity claim is made by this test.
//! Source effect declarations retain checked owner labels and operation slots.
mod common;
#[test]
fn source_handlers_preserve_declarations_effects_and_resumptions() {
    let dir = common::project("source_handlers", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceEffects.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-handlers").expect("build actual Cinch source effect checker");
    for (name, body, wanted) in [
        ("cannot erase a different region", "effect State (r :: Region) where\n    get :: Unit ->{State r} Int\n  run :: forall (r :: Region) (s :: Region). (Unit ->{State s} Int) -> Unit -> Int\n  run action u = handle action u with (handler State r where\n    return x -> x\n    get ignored resume k -> k 1)", 4),
        ("hidden operation through function", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  hidden :: Unit ->{Signal} Int\n  hidden u = signal u\n  run :: Unit -> Int\n  run u = handle hidden u with (handler Signal where\n    return x -> x\n    signal ignored resume k -> k 1)", 4),
        ("polymorphic result cannot specialize", "effect Magic where\n    magic :: forall a. Unit ->{Magic} a\n  run :: Unit -> Int\n  run u = handle (if magic u then 1 else 2) with (handler Magic where\n    return x -> x\n    magic ignored resume k -> k (123 :: Int))", 4),
        ("handle and resume", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> k 1)", 0),
        ("abort operation", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> 7)", 0),
        ("first class handler value", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = let h = (handler Signal where\n    return x -> x\n    signal u resume k -> k 1) in handle signal u with h", 0),
        ("resume once in each branch", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Bool -> Int\n  run choose = handle signal () with (handler Signal where\n    return x -> x\n    signal u resume k -> if choose then k 1 else k 2)", 0),
        ("resume twice", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> let x = k 1 in k x)", 4),
        ("resume escapes", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> let saved = k in saved 1)", 4),
        ("wrong resume argument", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> k ())", 4),
        ("missing operation", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x)", 4),
        ("duplicate operation", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> k 1\n    signal u resume k -> k 2)", 4),
        ("too many operation patterns", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u extra resume k -> k 1)", 4),
        ("too few operation patterns", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal resume k -> k 1)", 4),
        ("handle expects handler", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with 0", 4),
        ("preserve unrelated effects", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit ->{Other} Int\n  run u = handle (signal u + outside u) with (handler Signal where\n    return x -> x\n    signal u resume k -> k 1)", 0),
        ("cannot erase unrelated effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit -> Int\n  run u = handle (signal u + outside u) with (handler Signal where\n    return x -> x\n    signal u resume k -> k 1)", 4),
        ("preserve return clause effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit ->{Other} Int\n  run u = handle signal u with (handler Signal where\n    return x -> outside u\n    signal u resume k -> k 1)", 0),
        ("cannot erase return clause effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> outside u\n    signal u resume k -> k 1)", 4),
        ("preserve operation clause effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit ->{Other} Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> let n = outside u in k n)", 0),
        ("cannot erase operation clause effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> let n = outside u in k n)", 4),
        ("reperformed handled effect remains", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit ->{Signal} Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> signal u)", 0),
        ("cannot erase reperformed effect", "effect Signal where\n    signal :: Unit ->{Signal} Int\n  run :: Unit -> Int\n  run u = handle signal u with (handler Signal where\n    return x -> x\n    signal u resume k -> signal u)", 4),
        ("polymorphic failure and note", "effect Parse where\n    fail :: forall a. Int ->{Parse} a\n    note :: Int ->{Parse} Unit\n  run :: Int -> Int\n  run n = handle fail n with (handler Parse where\n    return x -> x\n    fail error resume k -> error\n    note error resume k -> k ())", 0),
        ("region parameter", "effect State (r :: Region) where\n    get :: Unit ->{State r} Int\n  run :: forall (r :: Region). Unit -> Int\n  run u = handle get u with (handler State r where\n    return x -> x\n    get ignored resume k -> k 1)", 0),
        ("type parameter", "effect State a where\n    put :: a ->{State a} Unit\n  run :: Int -> Unit\n  run n = handle put n with (handler State Int where\n    return x -> x\n    put value resume k -> k ())", 0),
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
