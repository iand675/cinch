//! Actual Cinch library emission: no main, explicit representation roots,
//! transitive workers, independently linked modules and runtime ownership.
mod common;
use std::process::Command;

fn global(module: &str, name: &str) -> String { cinch::interface::module_global(module, name) }
fn symbol(module: &str, name: &str) -> String { format!("cinch.typed.{}", global(module, name)) }


#[test]
fn checked_library_roots_emit_linkable_owned_and_specialized_workers() {
    let dir = common::project("runtime_library", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeLibrary.cinch")),
    ]);
    let emitter = common::build(&dir, "Main.cinch", "library-emitter").expect("build actual Cinch library emitter");
    let source = r#"module Library where
  data Parcel a where
    Parcel :: a -> Parcel a
  identity :: forall a. a -> a
  identity x = x
  wrap :: Int -> Parcel Int
  wrap x = Parcel x
  unwrap :: Parcel Int -> Int
  unwrap p = case p of Parcel x -> x
  helper :: Int -> Int
  helper n = n + 1
  answer :: Int
  answer = helper 41
  partial :: Int -> Int -> Int
  partial n = \x -> n + x
  apply :: (Int -> Int) -> Int -> Int
  apply f n = f n
  dead :: Int
  dead = 99
"#;
    let cases = [
        (source, r#"[["answer",[]],["identity",[0]],["identity",[1]],["identity",[2]],["wrap",[]],["unwrap",[]],["partial",[]],["apply",[]],["answer",[]]]"#, 0, ""),
        ("module Other where\n  other :: Int\n  other = 42\n", r#"[["other",[]]]"#, 0, ""),
        (source, r#"[["missing",[]]]"#, 5, "demanded runtime definition is absent"),
        (source, r#"[["identity",[]]]"#, 5, "first-order runtime representation specialization"),
        (source, r#"[["identity",[3]]]"#, 5, "first-order runtime representation specialization"),
        (source, r#"[["answer",[0]]]"#, 5, "first-order runtime representation specialization"),
        ("module Broken where\n  wrong :: Int\n  wrong = True\n", r#"[["wrong",[]]]"#, 4, ""),
    ];
    for (index, (source, roots, _, _)) in cases.iter().enumerate() {
        std::fs::write(dir.join(format!("input-{index}.source")), source).unwrap();
        // Roots are the checked Core identities, after source-module scope
        // elaboration. Source spelling alone is not a global identity.
        let mut selected = cinch::lsp::parse_json(roots).unwrap();
        let cinch::lsp::Json::Arr(ref mut rows) = selected else { panic!("roots array") };
        for row in rows {
            let cinch::lsp::Json::Arr(fields) = row else { panic!("root row") };
            let cinch::lsp::Json::Str(name) = &fields[0] else { panic!("root name") };
            fields[0] = cinch::lsp::Json::Str(global(if index == 1 { "Other" } else { "Library" }, name));
        }
        std::fs::write(dir.join(format!("input-{index}.roots")), selected.to_text()).unwrap();
    }
    std::fs::write(dir.join("batch.count"), vec![b'x'; cases.len()]).unwrap();
    // The source compiler uses the standard compiler heap; emitted user
    // objects below must still run and reclaim everything within32M.
    let output = Command::new(&emitter).current_dir(&dir)
        .env("CINCH_HEAP_LIMIT", "256M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert!(output.status.success(), "emitter: {}\n{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr), dir.display());
    let emitter_stderr = String::from_utf8_lossy(&output.stderr);
    assert!(emitter_stderr.contains("live=0 immortal=0 leaked=0"), "emitter ownership: {emitter_stderr}");
    println!("library emitter: {}", emitter_stderr.trim());
    for (index, (_, _, status, diagnostic)) in cases.iter().enumerate() {
        let actual = std::fs::read_to_string(dir.join(format!("output-{index}.status"))).unwrap();
        let diagnostics = std::fs::read_to_string(dir.join(format!("output-{index}.diagnostics"))).unwrap();
        assert_eq!(actual.trim(), status.to_string(), "case {index}: {diagnostics}\n{}", dir.display());
        assert!(diagnostics.contains(diagnostic), "case {index}: {diagnostics}");
        if *status != 0 { assert!(std::fs::read(dir.join(format!("output-{index}.ll"))).unwrap().is_empty()); }
    }
    let ir = std::fs::read_to_string(dir.join("output-0.ll")).unwrap();
    assert!(!ir.contains("define i32 @main("));
    assert!(!ir.contains("define i64 @cinch.typed.exit("));
    assert!(ir.contains("define internal i64 @__apply("));
    assert_eq!(ir.matches(&format!("define i64 @\"{}\"(", symbol("Library", "answer"))).count(), 1, "duplicate roots must emit once");
    assert!(ir.contains(&format!("define internal i64 @\"{}\"(", symbol("Library", "helper"))), "reachable private helper missing");
    assert!(!ir.contains(&format!("define i64 @\"{}\"(", symbol("Library", "dead"))), "unreachable definition emitted");
    assert_eq!(ir.lines().filter(|line| line.starts_with("define i64 ")).count(), 8, "only selected roots have external linkage");
    for class in 0..3 { assert!(ir.contains(&format!("define i64 @\"{}.r{class}\"(", symbol("Library", "identity")))); }
    let mut caller = r#"
#include <stdint.h>
#ifdef __APPLE__
#define CINCH_SYMBOL(name) "_" name
#else
#define CINCH_SYMBOL(name) name
#endif
extern uint64_t answer(void) __asm__(CINCH_SYMBOL("cinch.typed.answer"));
extern uint64_t other(void) __asm__(CINCH_SYMBOL("cinch.typed.other"));
extern uint64_t raw_identity(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.identity.r0"));
extern uint64_t object_identity(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.identity.r1"));
extern uint64_t tagged_identity(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.identity.r2"));
extern uint64_t wrap(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.wrap"));
extern uint64_t unwrap(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.unwrap"));
extern uint64_t partial(uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.partial"));
extern uint64_t apply(uint64_t, uint64_t) __asm__(CINCH_SYMBOL("cinch.typed.apply"));
extern uint64_t cinch_int_from_decimal(const char *, uint64_t);
extern int64_t cinch_int_to_i64(uint64_t);
extern int64_t cinch_int_cmp(uint64_t, uint64_t);
extern void cinch_dup(uint64_t);
extern void cinch_drop(uint64_t);
extern void cinch_reclaim_enable(void);
extern uint64_t cinch_exit_check(uint64_t);
int main(void) {
  cinch_reclaim_enable();
  if (raw_identity(1) != 1) return 1;
  uint64_t a = answer(), b = other();
  if (cinch_int_to_i64(a) != 42 || cinch_int_to_i64(b) != 42) return 2;
  cinch_drop(a); cinch_drop(b);
  uint64_t large = cinch_int_from_decimal("184467440737095516160", 21);
  cinch_dup(large);
  uint64_t kept = unwrap(object_identity(wrap(tagged_identity(large))));
  if (cinch_int_cmp(large, kept) != 0) return 3;
  cinch_drop(large); cinch_drop(kept);
  uint64_t ten = cinch_int_from_decimal("10", 2), thirty_two = cinch_int_from_decimal("32", 2);
  uint64_t result = apply(partial(ten), thirty_two);
  if (cinch_int_to_i64(result) != 42) return 4;
  cinch_drop(result);
  return (int)cinch_exit_check(0);
}
"#.to_string();
    for name in ["answer", "identity", "wrap", "unwrap", "partial", "apply"] {
        caller = caller.replace(&format!("\"cinch.typed.{name}"), &format!("\"{}", symbol("Library", name)));
    }
    caller = caller.replace("\"cinch.typed.other", &format!("\"{}", symbol("Other", "other")));
    std::fs::write(dir.join("caller.c"), caller).unwrap();
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
    for index in 0..2 {
        let output = Command::new(&clang).args(["-Wno-override-module", "-c", &format!("output-{index}.ll"), "-o", &format!("library-{index}.o")])
            .current_dir(&dir).output().unwrap();
        assert!(output.status.success(), "assemble library: {}", String::from_utf8_lossy(&output.stderr));
    }
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let exe = dir.join("caller");
    let output = Command::new(&clang).args(["caller.c", "library-0.o", "library-1.o"]).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe)
        .current_dir(&dir).output().unwrap();
    assert!(output.status.success(), "link independent libraries: {}\n{}", String::from_utf8_lossy(&output.stderr), dir.display());
    let output = Command::new(&exe).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "library caller: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("live=0 immortal=0 leaked=0"), "library ownership: {stderr}");
    println!("independent library caller: {}", stderr.trim());
    common::cleanup(&dir);
}
