//! The stage compiler's own lowering of the wide scalars (I128 and U128):
//! class Wide, an LLVM `i128` at every call, join and field, two raw words
//! in a payload, and a named refusal at every word position
//! (design/wide-scalars.md). `Data.Wide` is not reachable from stage
//! programs while the library provider pin stays where it is, so these
//! programs name the checked wide intrinsics themselves.
mod common;
use std::process::Command;

const PRIMS: &str = r#"  foreign import ccall "cinch.prim.eqI128" equal :: I128 -> I128 -> Bool
  foreign import ccall "cinch.prim.neI128" differ :: I128 -> I128 -> Bool
  foreign import ccall "cinch.prim.wrappingAddI128" plus :: I128 -> I128 -> I128
  foreign import ccall "cinch.prim.wrappingSubI128" minus :: I128 -> I128 -> I128
  foreign import ccall "cinch.prim.eqU128" equal-u :: U128 -> U128 -> Bool
  foreign import ccall "cinch.prim.wrappingAddU128" plus-u :: U128 -> U128 -> U128
  foreign import ccall "cinch.prim.wrappingSubU128" minus-u :: U128 -> U128 -> U128
"#;

/// (name, body after the intrinsics, expected exit code)
const RUNS: &[(&str, &str, i32)] = &[
    ("signed-wrapping", r#"  main :: Int
  main = let top = (170141183460469231731687303715884105727 :: I128) in
    let bottom = plus top 1 in
    if equal bottom (-170141183460469231731687303715884105728) && equal (minus bottom 1) top && equal (plus (-1) 1) 0 && differ top bottom then 201 else 1
"#, 201),
    // Above 2^127 a U128 literal is written in two's complement.
    ("unsigned-wrapping", r#"  main :: Int
  main = let top = (340282366920938463463374607431768211455 :: U128) in
    let half = (170141183460469231731687303715884105728 :: U128) in
    if equal-u (plus-u top 1) 0 && equal-u (minus-u 0 1) top && equal-u (plus-u half half) 0 && equal-u (minus-u half 1) 170141183460469231731687303715884105727 then 202 else 1
"#, 202),
    ("workers-joins-literal-patterns", r#"  double :: I128 -> I128
  double x = plus x x
  pick :: Bool -> I128 -> I128 -> I128
  pick flag a b = if flag then a else b
  constant :: Bool -> I128
  constant flag = if flag then 36893488147419103232 else -1
  classify :: I128 -> Int
  classify x = case x of
    0 -> 0
    18446744073709551616 -> 1
    _ -> 2
  rank :: Int -> I128
  rank n = case n of
    0 -> 18446744073709551616
    1 -> -18446744073709551616
    _ -> 0
  main :: Int
  main = if equal (double 9223372036854775808) 18446744073709551616 && classify (double 9223372036854775808) == 1
    && classify 0 == 0 && classify 5 == 2 && equal (pick False 1 (pick True 2 3)) 2
    && equal (constant True) 36893488147419103232 && equal (constant False) (-1)
    && equal (plus (rank 0) (rank 1)) 0 && equal (rank 7) 0 && double 2 == 4 && double 2 /= 5 then 203 else 1
"#, 203),
    // A wide field sits before a counted field, so the pointer map must
    // name the Nat at payload word 2, or the leak check fails.
    ("fields-maps-and-tuples", r#"  data Pair where
    Pair :: I128 -> Nat -> Pair
  data Box a where
    Box :: a -> Box a
  first :: Pair -> I128
  first p = case p of Pair w n -> w
  second :: Pair -> Nat
  second p = case p of Pair w n -> n
  unbox :: forall a. Box a -> a
  unbox b = case b of Box v -> v
  swap :: (I128, Nat) -> (Nat, I128)
  swap t = case t of (w, n) -> (n, w)
  main :: Int
  main = let p = Pair 18446744073709551617 100000000000000000000 in
    let boxed = Box (minus 0 1) in
    if equal (first p) 18446744073709551617 && second p == 100000000000000000000
      && equal (unbox boxed) (-1) && unbox (Box 7) == 7
      && (case swap (36893488147419103233, 200000000000000000000) of (n, w) -> n == 200000000000000000000 && equal w 36893488147419103233) then 204 else 1
"#, 204),
    // A wide specialization of a tail-group member is an ordinary worker;
    // a monomorphic wide loop never joins a group.
    ("generic-and-recursive", r#"  identity :: forall a. a -> a
  identity x = x
  repeat-keep :: forall a. a -> Int -> a
  repeat-keep acc n = if n == 0 then acc else repeat-keep acc (n - 1)
  sum-twos :: I128 -> Int -> I128
  sum-twos acc n = if n == 0 then acc else sum-twos (plus acc 2) (n - 1)
  main :: Int
  main = if equal (identity 85070591730234615865843651857942052864) 85070591730234615865843651857942052864 && identity 3 == 3
    && equal (repeat-keep (-7) 40) (-7) && repeat-keep 9 3 == 9
    && equal (sum-twos 170141183460469231731687303715884105700 20) (-170141183460469231731687303715884105716) then 205 else 1
"#, 205),
];

/// Programs the stage must refuse by name rather than emit.
const REFUSALS: &[(&str, &str)] = &[
    ("closure-argument", r#"  apply :: (I128 -> I128) -> I128 -> I128
  apply f x = f x
  main :: Int
  main = if equal (apply (plus 1) 1) 2 then 0 else 1
"#),
    ("lambda-result", r#"  later :: Unit -> I128
  later = \u -> plus 1 1
  main :: Int
  main = if equal (later ()) 2 then 0 else 1
"#),
    ("partial-wide-builtin", r#"  main :: Int
  main = let inc = plus 1 in if equal (inc 1) 2 then 0 else 1
"#),
];

#[test]
fn stage_lowers_wide_scalars_at_i128_and_refuses_word_positions() {
    let dir = common::project("bootstrap_wide", &[
        ("Bootstrap.Check.cinch", include_str!("../../compiler/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../compiler/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../compiler/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../compiler/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../compiler/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../compiler/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.RuntimeIr.cinch")),
    ]);
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    let emitter = common::build_executable(&dir, "Main.cinch", "runtime-ir").expect("build the Cinch runtime emitter");
    let runtime = common::host_runtime_archive();
    assert!(runtime.is_file(), "the Native-built emitter must have built its shared runtime");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());

    // The C boundary passes the wide scalars as `__int128`.
    let foreign = ("foreign-int128", r#"  foreign import ccall "cinch_test_wide_mix" mix :: I128 -> U128 ->{Foreign Wide} I128
  main :: Unit ->{Foreign Wide} Int
  main u = if equal (mix 18446744073709551616 129127208515966861312) 55340232221128654855 && equal (mix (-5) 0) (-15) then 206 else 1
"#, 206);
    let runs: Vec<(&str, &str, i32)> = RUNS.iter().copied().chain([foreign]).collect();
    let mut index = 0;
    for (_, body, _) in &runs {
        std::fs::write(dir.join(format!("input-{index}.source")), format!("module Main where\n{PRIMS}{body}")).unwrap();
        index += 1;
    }
    for (_, body) in REFUSALS {
        std::fs::write(dir.join(format!("input-{index}.source")), format!("module Main where\n{PRIMS}{body}")).unwrap();
        index += 1;
    }
    std::fs::write(dir.join("batch.count"), vec![b'x'; index]).unwrap();
    let output = Command::new(&emitter).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "batch input/output: {}", String::from_utf8_lossy(&output.stderr));

    let helper = dir.join("wide_helper.c");
    std::fs::write(&helper, "__int128 cinch_test_wide_mix(__int128 a, unsigned __int128 b) { return a * 3 + (__int128)(b >> 64); }\n").unwrap();
    for (index, (name, _, expected)) in runs.iter().enumerate() {
        let status = common::read_output(&dir, &format!("output-{index}.status"));
        assert_eq!(status, "0", "{name}: {}", common::read_output(&dir, &format!("output-{index}.diagnostics")));
        let ir = common::read_output(&dir, &format!("output-{index}.ll"));
        assert!(ir.contains("i128"), "{name}: wide values must travel as i128");
        assert!(!ir.contains("@cinch.typed.rc.dup(i128") && !ir.contains("@cinch.typed.rc.drop(i128"), "{name}: a wide value is never counted");
        match *name {
            "signed-wrapping" => {
                assert!(ir.contains(" = add i128 %arg0, %arg1") && ir.contains(" = sub i128 %arg0, %arg1"), "wrapping i128 workers");
                assert!(!ir.contains(" nsw ") && !ir.contains(" nuw "), "wrapping operations must preserve overflow");
            }
            "unsigned-wrapping" => assert!(ir.contains("-170141183460469231731687303715884105728"), "2^127 as a U128 is written in two's complement"),
            "workers-joins-literal-patterns" => {
                assert!(ir.lines().any(|line| line.starts_with("define ") && line.contains("i128 @") && line.contains("m.Main.double") && line.contains("(i128 %arg0)")), "a typed worker header:\n{ir}");
                assert!(ir.contains("phi i128") || ir.contains("select i1") || ir.contains("alloca i128"), "a typed join");
                assert!(ir.contains("icmp eq i128"), "a typed literal test");
            }
            "fields-maps-and-tuples" => {
                assert!(ir.contains("store i128") && ir.contains(", align 8") && ir.contains("load i128"), "an 8-aligned wide field");
                assert!(ir.lines().any(|line| line.starts_with("define ") && line.contains("m.Main.unbox.r3")), "a Wide specialization of a generic worker");
            }
            "generic-and-recursive" => {
                assert!(ir.lines().any(|line| line.starts_with("define ") && line.contains("m.Main.repeat$2dkeep.r3") && line.contains("(i128 %arg0, i64 %arg1)")), "the Wide specialization is an ordinary typed worker:\n{ir}");
            }
            "foreign-int128" => {
                // The object symbol spells every byte of the C name as an escape.
                let symbol: String = "cinch_test_wide_mix".bytes().map(|b| format!("\\{b:02X}")).collect();
                assert!(ir.contains(&format!("declare i128 @\"{symbol}\"(i128, i128)")), "the C boundary is __int128:\n{ir}");
            }
            _ => {}
        }
        std::fs::write(dir.join("output.ll"), &ir).unwrap();
        let exe = dir.join(format!("emitted-{name}"));
        let mut link = Command::new(&clang);
        link.arg(dir.join("output.ll"));
        if *name == "foreign-int128" { link.arg(&helper); }
        let linked = link.arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe).output().unwrap();
        assert!(linked.status.success(), "{name}: {}", String::from_utf8_lossy(&linked.stderr));
        let ran = Command::new(&exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(ran.status.code(), Some(*expected), "{name}: {}", String::from_utf8_lossy(&ran.stderr));
        let stderr = String::from_utf8_lossy(&ran.stderr);
        let leak = stderr.lines().find(|line| line.starts_with("cinch-leak:")).expect("runtime leak report");
        assert!(leak.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
        println!("{name}: exit {expected}; {leak}");
    }
    for (offset, (name, _)) in REFUSALS.iter().enumerate() {
        let index = runs.len() + offset;
        assert_eq!(common::read_output(&dir, &format!("output-{index}.status")), "5", "{name} must be refused");
        assert!(common::read_output(&dir, &format!("output-{index}.ll")).is_empty(), "{name}: a refusal emits nothing");
        let diagnostics = common::read_output(&dir, &format!("output-{index}.diagnostics"));
        assert!(diagnostics.contains("its class is Wide"), "{name}: {diagnostics}");
    }
    common::cleanup(&dir);
}
