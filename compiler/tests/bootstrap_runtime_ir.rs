//! Runtime-fed Cinch source becomes typed Core and shared-runtime LLVM.
//! Bignums, tuples and source ADTs run with reclamation and leak checks enabled.
mod common;
use std::process::Command;

#[test]
fn typed_runtime_numbers_preserve_exact_values_and_reclaim_owned_words() {
    let dir = common::project("runtime_ir", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeIr.cinch")),
    ]);
    let compiler = common::build(&dir, "Main.cinch", "runtime-ir").expect("build the Cinch runtime emitter");
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    assert!(runtime.is_file(), "the Native-built emitter must have built its shared runtime");
    let tag64 = format!("  data Tag where\n{}  main :: Int\n  main = case C63 of\n    C63 -> 137\n    _ -> 1\n",
        (0..64).map(|i| format!("    C{i} :: Tag\n")).collect::<String>());
    let cases = [
        ("huge", "  main :: Int\n  main = if 340282366920938463463374607431768211456 + 7 == 340282366920938463463374607431768211463 then 37 else 1\n", 37),
        ("alias", "  main :: Int\n  main = let x = 9223372036854775808 in let y = x * x in if y - x * x == 0 then 39 else 1\n", 39),
        ("call", "  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = if identity 340282366920938463463374607431768211456 == 340282366920938463463374607431768211456 then 41 else 1\n", 41),
        ("branch", "  pick :: Bool -> Int -> Int -> Int\n  pick flag x y = if flag then x else y\n  main :: Int\n  main = if pick False 9223372036854775808 18446744073709551616 == 18446744073709551616 then 43 else 1\n", 43),
        ("recursive", "  sum :: Int -> Int\n  sum n = if n <= 0 then 9223372036854775808 else n + sum (n - 1)\n  main :: Int\n  main = if sum 20 == 9223372036854776018 then 47 else 1\n", 47),
        ("negative", "  main :: Int\n  main = let x = -9223372036854775809 in if -x == 9223372036854775809 then 53 else 1\n", 53),
        ("natural", "  previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else n - 1\n  main :: Int\n  main = if previous 340282366920938463463374607431768211456 == 340282366920938463463374607431768211455 then 59 else 1\n", 59),
        ("wildcard", "  pick :: Int -> Int -> Int\n  pick _ x = x\n  main :: Int\n  main = if pick 18446744073709551616 9223372036854775808 == 9223372036854775808 then 61 else 1\n", 61),
        ("tuple-field", "  first :: (Int, Int) -> Int\n  first (x, y) = x\n  main :: Int\n  main = if first (18446744073709551616, 9223372036854775808) == 18446744073709551616 then 67 else 1\n", 67),
        ("tuple-case", "  main :: Int\n  main = let pair = (9223372036854775808, 9223372036854775808) in case pair of\n    (x, y) -> if x == y then 71 else 1\n", 71),
        ("nested-tuple", "  pick :: (Bool, (Int, Nat)) -> Int\n  pick (flag, (x, y)) = if flag then x else 0\n  main :: Int\n  main = if pick (True, (18446744073709551616, 340282366920938463463374607431768211456)) == 18446744073709551616 then 73 else 1\n", 73),
        ("dropped-tuple", "  ignore :: (Int, (Nat, Int)) -> Int\n  ignore _ = 79\n  main :: Int\n  main = ignore (18446744073709551616, (340282366920938463463374607431768211456, -9223372036854775809))\n", 79),
        ("full-inline-map", "  ignore :: (Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) -> Int\n  ignore _ = 83\n  main :: Int\n  main = ignore (9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808)\n", 83),
        ("nullary-constructors", "  data Colour where\n    Red :: Colour\n    Green :: Colour\n    Blue :: Colour\n  main :: Int\n  main = case Blue of\n    Red -> 1\n    Green -> 2\n    Blue -> 89\n", 89),
        ("recursive-tree", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  total :: Tree -> Int\n  total (Leaf n) = n\n  total (Node left right) = total left + total right\n  main :: Int\n  main = if total (Node (Leaf 18446744073709551616) (Leaf 9223372036854775808)) == 27670116110564327424 then 97 else 1\n", 97),
        ("failed-nested-pattern", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  main :: Int\n  main = case Node (Leaf 340282366920938463463374607431768211457) (Leaf 9223372036854775808) of\n    Node (Leaf 340282366920938463463374607431768211456) (Leaf n) -> 1\n    Node (Leaf x) (Leaf y) -> if x > y then 101 else 2\n    _ -> 3\n", 101),
        ("shared-constructor-root", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  total :: Tree -> Int\n  total (Leaf n) = n\n  total (Node left right) = total left + total right\n  main :: Int\n  main = let leaf = Leaf 18446744073709551616 in let tree = Node leaf leaf in if total tree == 36893488147419103232 then 103 else 1\n", 103),
        ("raw-bool-patterns", "  choose :: Bool -> Int -> Int\n  choose True n = n\n  choose False n = 0\n  main :: Int\n  main = if choose False 18446744073709551616 == 0 && choose True 18446744073709551616 == 18446744073709551616 then 107 else 1\n", 107),
        ("numeric-equations", "  classify :: Int -> Int\n  classify 0 = 2\n  classify n = 3\n  main :: Int\n  main = if classify 0 == 2 && classify 340282366920938463463374607431768211456 == 3 then 109 else 1\n", 109),
        ("nested-case-guards", "  data Box where\n    Box :: Int -> Box\n  get :: Box -> Int\n  get (Box n) = case n of\n    value | value < 0 -> 0\n    value -> value\n  main :: Int\n  main = if get (Box (-18446744073709551616)) == 0 && get (Box 18446744073709551616) == 18446744073709551616 then 113 else 1\n", 113),
        ("failed-case-guard", "  data Box where\n    Box :: Int -> Box\n  main :: Int\n  main = case Box 18446744073709551616 of\n    Box n | n < 0 -> 1\n    Box n -> if n == 18446744073709551616 then 127 else 2\n", 127),
        ("natural-literal-pattern", "  classify :: Nat -> Int\n  classify 340282366920938463463374607431768211456 = 131\n  classify _ = 1\n  main :: Int\n  main = classify 340282366920938463463374607431768211456\n", 131),
        ("last-regular-tag", tag64.as_str(), 137),
    ];
    for (name, body, expected) in cases {
        std::fs::write(dir.join("input.source"), format!("module Main where\n{body}")).unwrap();
        let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{name}: {} {}", common::read_output(&dir, "output.diagnostics"), String::from_utf8_lossy(&output.stderr));
        let exe = dir.join(format!("emitted-{name}"));
        let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe).output().unwrap();
        assert!(output.status.success(), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        let output = Command::new(&exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        let stderr = String::from_utf8_lossy(&output.stderr);
        let leak = stderr.lines().find(|line| line.starts_with("cinch-leak:")).expect("runtime leak report");
        assert!(leak.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
        println!("{name}: exit {expected}; {leak}");
    }
    // Verify the physical ABI through C, including the anonymous tuple's
    // tag0 when its map matches a named tag1 constructor's map.
    std::fs::write(dir.join("input.source"), "module Main where\n  data D where\n    A :: Int -> D\n    B :: D -> D -> D\n  makePair :: (D, D)\n  makePair = let leaf = A 18446744073709551616 in let node = B leaf leaf in (node, leaf)\n  main :: Int\n  main = 0\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "ABI: {} {}", common::read_output(&dir, "output.diagnostics"), String::from_utf8_lossy(&output.stderr));
    let ir = common::read_output(&dir, "output.ll");
    assert!(ir.contains("define i32 @main(i32 %argc, ptr %argv)"));
    let mut ir = ir.replace("define i32 @main(i32 %argc, ptr %argv)", "define i32 @cinch_program_main(i32 %argc, ptr %argv)");
    let encode = |name: &str| name.chars().map(|ch| format!("{}_", ch as u32)).collect::<String>();
    let symbol = format!("cinch.typed.m.{}.{}", encode("Main"), encode("makePair"));
    ir.push_str(&format!("\ndefine i64 @cinch_abi_pair() {{\nentry:\n  %pair = call i64 @\"{symbol}\"()\n  ret i64 %pair\n}}\n"));
    std::fs::write(dir.join("abi.ll"), ir).unwrap();
    std::fs::write(dir.join("abi.c"), r#"
#include <stdint.h>
extern uint64_t cinch_abi_pair(void);
extern void cinch_reclaim_enable(void);
extern void cinch_drop(uint64_t);
extern int64_t cinch_exit_check(int64_t);
static uint64_t tag(uint64_t word) { return (((uint64_t *)(uintptr_t)word)[-1] >> 48) & 63; }
int main(void) {
  cinch_reclaim_enable();
  uint64_t pair = cinch_abi_pair();
  uint64_t *fields = (uint64_t *)(uintptr_t)pair;
  uint64_t node = fields[0], leaf = fields[1];
  uint64_t *children = (uint64_t *)(uintptr_t)node;
  int good = tag(pair) == 0 && tag(node) == 1 && tag(leaf) == 0
    && children[0] == leaf && children[1] == leaf;
  cinch_drop(pair);
  return (int)cinch_exit_check(good ? 0 : 149);
}
"#).unwrap();
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("abi.ll")).arg(dir.join("abi.c")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("abi-program")).output().unwrap();
    assert!(output.status.success(), "ABI link: {}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("abi-program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "physical ABI: {}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    println!("C ABI tags and aliases: exit0; {}", String::from_utf8_lossy(&output.stderr).trim());
    // The typed frontend accepts this; unavailable lowering must refuse it
    // explicitly and clear the preceding successful IR artifact.
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = identity 1\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("polymorphic"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    std::fs::write(dir.join("input.source"), "module Main where\n  main :: Int\n  main = case (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17) of\n    _ -> 0\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("extended"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    let source = format!("module Main where\n  data Tag where\n{}  main :: Int\n  main = case C64 of\n    C64 -> 1\n    _ -> 2\n", (0..65).map(|i| format!("    C{i} :: Tag\n")).collect::<String>());
    std::fs::write(dir.join("input.source"), source).unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("extended"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    common::cleanup(&dir);
}
