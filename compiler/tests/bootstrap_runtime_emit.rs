//! The real compiler CLI exposes checked typed IR without linking or running.
mod common;
use std::process::Command;

#[test]
fn compiler_emit_mode_uses_the_typed_runtime_backend_and_preserves_refusals() {
    let dir = common::project("runtime_emit", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Expand.cinch", include_str!("../../bootstrap/Bootstrap.Expand.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Ir.cinch", include_str!("../../bootstrap/Bootstrap.Ir.cinch")),
        ("Bootstrap.Driver.cinch", include_str!("../../bootstrap/Bootstrap.Driver.cinch")),
        ("Bootstrap.Pipeline.cinch", include_str!("../../bootstrap/Bootstrap.Pipeline.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Pipeline.cinch")),
    ]);
    let compiler = common::build(&dir, "Main.cinch", "compiler").expect("build the actual CLI compiler");
    let invoke = |args: &[&str]| Command::new(&compiler).args(args).current_dir(&dir).output().unwrap();
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = if identity 340282366920938463463374607431768211456 + 3 == 340282366920938463463374607431768211459 then 67 else 1\n").unwrap();
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    for args in [
        vec!["--emit-ir", "input.source", "output.ll"],
        vec!["--emit-ir", "--interfaces", "interfaces.json", "input.source", "output.ll"],
        vec!["--interfaces", "interfaces.json", "--emit-ir", "input.source", "output.ll"],
    ] {
        let output = invoke(&args);
        assert_eq!(output.status.code(), Some(0), "{} {}", common::read_output(&dir, "output.ll.diagnostics"), String::from_utf8_lossy(&output.stderr));
        assert!(common::read_output(&dir, "output.ll").contains("@cinch_int_add"));
        assert!(common::read_output(&dir, "output.ll.diagnostics").is_empty());
    }
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("artifact")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("artifact")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(67), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    // Use the public Native apply ABI from C. This checks the real heap
    // header, prefix, partial-copy ownership and bulk overapplication.
    std::fs::write(dir.join("input.source"), "module Main where\n  add :: Int -> Int -> Int\n  add x y = x + y\n  returning :: Int -> Int -> Int\n  returning x = \\y -> x + y\n  getPartial :: Int -> Int\n  getPartial = add 18446744073709551616\n  getOver :: Int -> Int -> Int\n  getOver = returning\n  main :: Int\n  main = if getPartial 2 > 0 && getOver 2 3 == 5 then 0 else 1\n").unwrap();
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(0), "{}", common::read_output(&dir, "output.ll.diagnostics"));
    let mut ir = common::read_output(&dir, "output.ll").replace("define i32 @main(i32 %argc, ptr %argv)", "define i32 @cinch_program_main(i32 %argc, ptr %argv)");
    for (source, bridge) in [("getPartial", "cinch_abi_partial"), ("getOver", "cinch_abi_over")] {
        let symbol = format!("cinch.typed.{}", cinch::interface::module_global("Main", source));
        ir.push_str(&format!("\ndefine i64 @{bridge}() {{\nentry:\n  %value = call i64 @\"{symbol}\"()\n  ret i64 %value\n}}\n"));
    }
    ir.push_str("\ndefine i64 @cinch_abi_shape(i64 %index) {\nentry:\n  %slot = getelementptr i64, ptr @cinch.typed.layouts, i64 %index\n  %value = load i64, ptr %slot\n  ret i64 %value\n}\n");
    std::fs::write(dir.join("closures.ll"), ir).unwrap();
    std::fs::write(dir.join("closures.c"), r#"
#include <stdint.h>
extern uint64_t cinch_abi_partial(void);
extern uint64_t cinch_abi_over(void);
extern uint64_t cinch_abi_shape(uint64_t);
extern uint64_t __apply(uint64_t, uint64_t, uint64_t *);
extern void cinch_reclaim_enable(void);
extern void cinch_dup(uint64_t);
extern void cinch_drop(uint64_t);
extern uint64_t cinch_int_from_decimal(const char *, uint64_t);
extern int64_t cinch_int_cmp(uint64_t, uint64_t);
extern int64_t cinch_exit_check(int64_t);
static int closure(uint64_t word, uint64_t arity, uint64_t filled) {
  uint64_t *p = (uint64_t *)(uintptr_t)word;
  uint64_t header = p[-1], shape = header >> 48;
  uint64_t map = (arity == 2 ? UINT64_C(640) : UINT64_C(128)) << 32;
  return ((header >> 40) & 255) == 2 && p[0] != 0 && p[1] == arity && p[2] == filled
    && cinch_abi_shape(shape) == (3 + arity + map);
}
int main(void) {
  cinch_reclaim_enable();
  uint64_t original = cinch_abi_partial();
  int good = closure(original, 2, 1);
  uint64_t saved = ((uint64_t *)(uintptr_t)original)[3];
  cinch_dup(original);
  uint64_t empty = 0;
  uint64_t cloned = __apply(original, 0, &empty);
  good = good && closure(cloned, 2, 1) && ((uint64_t *)(uintptr_t)cloned)[3] == saved;
  cinch_drop(original);
  uint64_t arg = cinch_int_from_decimal("2", 1);
  uint64_t result = __apply(cloned, 1, &arg);
  uint64_t expected = cinch_int_from_decimal("18446744073709551618", 20);
  good = good && cinch_int_cmp(result, expected) == 0;
  cinch_drop(result);
  cinch_drop(expected);
  uint64_t over = cinch_abi_over();
  good = good && closure(over, 1, 0);
  uint64_t args[2] = { cinch_int_from_decimal("18446744073709551616", 20), cinch_int_from_decimal("3", 1) };
  result = __apply(over, 2, args);
  expected = cinch_int_from_decimal("18446744073709551619", 20);
  good = good && cinch_int_cmp(result, expected) == 0;
  cinch_drop(result);
  cinch_drop(expected);
  return (int)cinch_exit_check(good ? 0 : 237);
}
"#).unwrap();
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("closures.ll")).arg(dir.join("closures.c")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("closure-abi")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("closure-abi")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    println!("C closure ABI: kind2, exact prefix/maps, shared PAP clone and bulk overapplication; {}", String::from_utf8_lossy(&output.stderr).trim());
    std::fs::write(dir.join("input.source"), "module Main where\n  data Box where\n    Box :: Int -> Box\n  main :: Int\n  main = case Box 340282366920938463463374607431768211456 of\n    Box n -> if n > 0 then 139 else 1\n").unwrap();
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(0), "{}", common::read_output(&dir, "output.ll.diagnostics"));
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("adt-artifact")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("adt-artifact")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(139), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    // Literal True alone completes coverage; computed or false guards do not.
    for (guard, expected) in [("True", 0), ("False", 4), ("if True then True else True", 4)] {
        std::fs::write(dir.join("input.source"), format!("module Main where\n  main :: Int\n  main = case 18446744073709551616 of value | ({guard}) -> 149\n")).unwrap();
        let output = invoke(&["--check", "input.source", "guard-coverage"]);
        assert_eq!(output.status.code(), Some(expected), "{guard}: {} {}", common::read_output(&dir, "guard-coverage.diagnostics"), String::from_utf8_lossy(&output.stderr));
    }
    let guarded_precondition = r#"
module Main where
  guarded :: using (n > 0) => (n :: Int) -> Int
  guarded n | n > 2 = 1
  guarded n = 2
  main :: Int
  main = case 18446744073709551616 of
    value | True -> if value > 0 && guarded 3 == 1 && guarded 1 == 2 then 149 else 1
"#;
    std::fs::write(dir.join("input.source"), guarded_precondition).unwrap();
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(0), "{}", common::read_output(&dir, "output.ll.diagnostics"));
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("guard-artifact")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("guard-artifact")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(149), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    std::fs::write(dir.join("input.source"), guarded_precondition.replace("guarded 1 == 2", "guarded 0 == 2")).unwrap();
    let output = invoke(&["--emit-ir", "input.source", "output.ll"]);
    assert_eq!(output.status.code(), Some(4), "{}", common::read_output(&dir, "output.ll.diagnostics"));
    assert!(common::read_output(&dir, "output.ll.diagnostics").contains("precondition is not established"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    // Constructor field carriers are resolved by their enclosing type;
    // literals must not default before that relationship is known.
    for (carrier, literal, expected) in [("Nat", "0", 0), ("I8", "127", 0), ("Bool", "0", 4), ("I8", "128", 4)] {
        std::fs::write(dir.join("input.source"), format!("module Main where\n  data Crate a where\n    Pack :: a -> Crate a\n  inspect :: Crate {carrier} -> Bool\n  inspect value = case value of Pack {literal} -> True; Pack n -> False\n  main :: Int\n  main = 0\n")).unwrap();
        let output = invoke(&["--check", "input.source", "pattern-literal"]);
        assert_eq!(output.status.code(), Some(expected), "{carrier}/{literal}: {} {}", common::read_output(&dir, "pattern-literal.diagnostics"), String::from_utf8_lossy(&output.stderr));
    }
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: forall (f :: Type -> Type) a. f a -> f a\n  identity x = x\n  data Wrap a where\n    Wrap :: a -> Wrap a\n  main :: Int\n  main = case identity (Wrap 1) of Wrap n -> n\n").unwrap();
    assert_eq!(invoke(&["--check", "input.source", "checked"]).status.code(), Some(0));
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(0), "{}", common::read_output(&dir, "output.ll.diagnostics"));
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("higher-kind-artifact")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("higher-kind-artifact")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(1), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    std::fs::write(dir.join("input.source"), "module Main where\n  main :: Int\n  main = case (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17) of\n    _ -> 0\n").unwrap();
    assert_eq!(invoke(&["--check", "input.source", "checked"]).status.code(), Some(0));
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(5));
    assert!(common::read_output(&dir, "output.ll").is_empty(), "failed emission retained stale IR");
    assert!(common::read_output(&dir, "output.ll.diagnostics").contains("tuple fields need an extended or polymorphic runtime layout"));
    assert_eq!(invoke(&["--check", "--emit-ir", "input.source", "output.ll"]).status.code(), Some(1));
    let provider = common::project("cli_object_library", &[("Library.cinch", "module Library where\n  marker :: Bool -> Bool\n  marker b = b\n")]);
    let object = provider.join("provider.o");
    cinch::driver::driver(cinch::driver::Mode::EmitObject, &provider.join("Library.cinch"), Some(&object)).unwrap();
    let mut rows = Vec::new();
    for entry in std::fs::read_dir(provider.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "cinchif") {
            let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            // All generated paths here are conventional temporary paths;
            // the producer sources and hashes remain independently checked.
            rows.push(format!("[\"{}\",\"{}\",\"{}\"]", iface.module, iface.iface_hash(), path.display()));
        }
    }
    rows.sort(); std::fs::write(dir.join("object-interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let digest = |path: &std::path::Path| {
        let output = Command::new("python3").args(["-c", "import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())"]).arg(path).output().unwrap();
        assert!(output.status.success()); String::from_utf8(output.stdout).unwrap().trim().to_string()
    };
    let metadata = object.with_extension("abi.json");
    let configuration = format!("{{\"schema\":1,\"target\":\"{}\",\"object\":\"{}\",\"metadata\":\"{}\",\"object_sha256\":\"{}\",\"metadata_sha256\":\"{}\"}}", cinch::core::TargetProfile::native().triple, object.display(), metadata.display(), digest(&object), digest(&metadata));
    std::fs::write(dir.join("objects.json"), &configuration).unwrap();
    std::fs::write(dir.join("input.source"), "module Main where\n  import Data.Bool (not)\n  main :: Int\n  main = if not False then 237 else 1\n").unwrap();
    let emitted = invoke(&["--emit-ir", "--interfaces", "object-interfaces.json", "--objects", "objects.json", "input.source", "object.ll"]);
    assert_eq!(emitted.status.code(), Some(0), "{} {}", common::read_output(&dir,"object.ll.diagnostics"), String::from_utf8_lossy(&emitted.stderr));
    let linked = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into())).arg(dir.join("object.ll")).arg(&object).arg(&runtime).arg("-o").arg(dir.join("object-program")).output().unwrap();
    assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
    let ran = Command::new(dir.join("object-program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
    assert_eq!(ran.status.code(), Some(237), "{}", String::from_utf8_lossy(&ran.stderr));
    assert!(String::from_utf8_lossy(&ran.stderr).split_whitespace().any(|s| s == "leaked=0"));
    println!("object CLI: exit237; {}", String::from_utf8_lossy(&ran.stderr).trim());
    std::fs::write(dir.join("objects.json"), configuration.replace(&digest(&metadata), &"0".repeat(64))).unwrap();
    assert_eq!(invoke(&["--emit-ir", "--objects", "objects.json", "--interfaces", "object-interfaces.json", "input.source", "object.ll"]).status.code(), Some(7));
    assert!(common::read_output(&dir,"object.ll").is_empty());
    assert!(common::read_output(&dir,"object.ll.diagnostics").contains("artifact digest mismatch"));
    common::cleanup(&provider);
    common::cleanup(&dir);
}
