//! Typed physical ABI facts and real public/closure entries from Cinch Core.
mod common;
use cinch::lsp::{parse_json, Json};
use std::process::Command;
fn global(name: &str) -> String {
    let encode = |s: &str| s.chars().map(|c| format!("{}_", c as u32)).collect::<String>();
    format!("m.{}.{}", encode("Library"), encode(name))
}
fn fixture_sources(dir: &std::path::Path) -> std::collections::BTreeMap<std::ffi::OsString, Vec<u8>> {
    std::fs::read_dir(dir).unwrap().map(|item| item.unwrap().path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "cinch"))
        .map(|p| (p.file_name().unwrap().to_owned(), std::fs::read(p).unwrap())).collect()
}
fn defined_symbol(line: &str) -> String {
    let quoted = line.strip_prefix("define i64 @\"").unwrap().split_once("\"(").unwrap().0;
    let bytes = quoted.as_bytes(); let mut index = 0; let mut decoded = Vec::new();
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            decoded.push(u8::from_str_radix(&quoted[index + 1..index + 3], 16).unwrap()); index += 3;
        } else { decoded.push(bytes[index]); index += 1; }
    }
    String::from_utf8(decoded).unwrap()
}
fn row<'a>(units: &'a Json, name: &str) -> &'a Json {
    units.as_arr().unwrap().iter().find(|u| u.get("symbol").and_then(Json::as_str) == Some(name)).unwrap_or_else(|| panic!("missing {name}: {units:?}"))
}
fn check_unit(units: &Json, name: &str, params: &[&str], result: &str) {
    let unit = row(units, name);
    assert_eq!(unit.get("closure_entry").and_then(Json::as_str), Some(format!("{name}.object-closure").as_str()));
    assert_eq!(unit.get("arity").and_then(Json::as_num), Some(params.len() as f64));
    assert_eq!(unit.get("params").and_then(Json::as_arr).unwrap().iter().map(|p| p.as_str().unwrap()).collect::<Vec<_>>(), params);
    assert_eq!(unit.get("result").and_then(Json::as_str), Some(result));
}
#[test]
fn library_abi_entries_describe_actual_workers_and_owned_closure_accessors() {
    let dir = common::project("runtime_library_abi", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeLibraryAbi.cinch")),
    ]);
    let emitter = if let Some(path) = std::env::var_os("CINCH_TEST_LIBRARY_ABI_EMITTER") {
        // This explicit Native-fixture reuse keeps a corrected assertion
        // from rebuilding identical Cinch sources. The final self-built
        // suite must always compile its fixture with its selected frontend.
        assert!(common::external_frontend::configured().unwrap().is_none(), "cached Native fixture cannot replace the selected Cinch frontend");
        let cached = std::path::PathBuf::from(path).canonicalize().unwrap();
        assert_eq!(fixture_sources(&dir), fixture_sources(cached.parent().unwrap()), "cached emitter inputs changed");
        cached
    } else {
        common::build(&dir, "Main.cinch", "library-abi-emitter").expect("build actual Cinch ABI emitter")
    };
    let source = r#"module Library where
  data Parcel a where
    Parcel :: a -> Parcel a
  identity :: forall a. a -> a
  identity x = x
  wrap :: Int -> Parcel Int
  wrap n = Parcel n
  unwrap :: Parcel Int -> Int
  unwrap p = case p of Parcel n -> n
  partial :: Int -> Int -> Int
  partial n = \x -> n + x
  deferred :: (h :: True) -> Int
  deferred h = 42
  indexed :: forall (n :: Nat). Nat -> Nat
  indexed x = x
"#;
    let root = |name: &str, codes: &[u8], public: &str| Json::Arr(vec![Json::Str(global(name)), Json::Arr(codes.iter().map(|c| Json::Num(*c as f64)).collect()), Json::Str(public.into())]);
    let valid = Json::Arr(vec![root("identity", &[0], "Library.identity@R"), root("identity", &[1], "Library.identity@O"), root("identity", &[2], "Library.identity@T"), root("wrap", &[], "Library.wrap"), root("unwrap", &[], "Library.unwrap"), root("partial", &[], "Library.partial"), root("deferred", &[], "Library.deferred"), root("indexed", &[], "Library.indexed"), root("wrap", &[], "Library.wrap")]);
    let cases = [
        (valid, 0, ""),
        (Json::Arr(vec![root("wrap", &[], "Library.same"), root("unwrap", &[], "Library.same")]), 5, "export symbols collide"),
        (Json::Arr(vec![root("wrap", &[], "Library.one"), root("unwrap", &[], "Library.one.object-closure")]), 5, "export symbols collide"),
        (Json::Arr(vec![root("wrap", &[], "")]), 5, "export symbol is empty"),
        (Json::Arr(vec![root("wrap", &[], "cinch.typed.forged")]), 5, "collides with typed workers"),
        (Json::Arr(vec![root("identity", &[3], "Library.invalid")]), 5, "checked physical parameter/result shape"),
        (Json::Arr(vec![root("missing", &[], "Library.missing")]), 5, "no checked definition"),
    ];
    for (index, (roots, _, _)) in cases.iter().enumerate() {
        std::fs::write(dir.join(format!("input-{index}.source")), source).unwrap();
        std::fs::write(dir.join(format!("input-{index}.roots")), roots.to_text()).unwrap();
    }
    std::fs::write(dir.join("batch.count"), vec![b'x'; cases.len()]).unwrap();
    let output = Command::new(&emitter).current_dir(&dir).env("CINCH_HEAP_LIMIT", "256M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert!(output.status.success(), "ABI emitter: {}\n{}", String::from_utf8_lossy(&output.stderr), dir.display());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("live=0 immortal=0 leaked=0"), "{stderr}");
    println!("library ABI emitter: {}", stderr.trim());
    for (index, (_, wanted, message)) in cases.iter().enumerate() {
        let status = common::read_output(&dir, &format!("output-{index}.status"));
        let diagnostics = common::read_output(&dir, &format!("output-{index}.diagnostics"));
        assert_eq!(status.trim(), wanted.to_string(), "case {index}: {diagnostics}\n{}", dir.display());
        assert!(diagnostics.contains(message), "case {index}: {diagnostics}");
        if *wanted != 0 {
            assert!(common::read_output(&dir, &format!("output-{index}.ll")).is_empty());
            assert_eq!(common::read_output(&dir, &format!("output-{index}.units.json")).trim(), "[]");
            assert_eq!(common::read_output(&dir, &format!("output-{index}.layouts.json")).trim(), "[]");
        }
    }
    let units = parse_json(&common::read_output(&dir, "output-0.units.json")).unwrap();
    assert_eq!(units.as_arr().unwrap().len(), 8, "duplicate checked root must emit one physical unit");
    for (name, class) in [("R", "R"), ("O", "O"), ("T", "T")] { check_unit(&units, &format!("Library.identity@{name}"), &[class], class); }
    check_unit(&units, "Library.wrap", &["T"], "O");
    check_unit(&units, "Library.unwrap", &["O"], "T");
    check_unit(&units, "Library.partial", &["T"], "O");
    check_unit(&units, "Library.deferred", &[], "O");
    check_unit(&units, "Library.indexed", &["T", "T"], "T");
    let mut ir = common::read_output(&dir, "output-0.ll");
    assert!(!ir.contains("define i32 @main("));
    let public = ir.lines().filter(|line| line.starts_with("define i64 ")).map(defined_symbol).collect::<Vec<_>>();
    let expected = units.as_arr().unwrap().iter().flat_map(|unit| ["symbol", "closure_entry"].map(|key| unit.get(key).and_then(Json::as_str).unwrap().to_string())).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(public.len(), expected.len(), "every ABI symbol is defined exactly once");
    assert_eq!(public.into_iter().collect::<std::collections::BTreeSet<_>>(), expected, "only real ABI bridges/accessors are public");
    let layout_line = ir.lines().find(|line| line.starts_with("@cinch.typed.layouts = ")).expect("physical layout table");
    let actual = layout_line.rsplit_once('[').unwrap().1.trim_end_matches(']').split(", ").map(|v| v.strip_prefix("i64 ").unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
    // Parse unsigned descriptor words exactly; a floating JSON projection
    // would lose integer precision for a full sixteen-field ownership map.
    let text = common::read_output(&dir, "output-0.layouts.json");
    let reported = text.trim().trim_start_matches('[').trim_end_matches(']').split(',').map(|n| n.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();
    assert_eq!(reported, actual, "metadata must use the exact final physical table, including accessor shapes");
    ir.push_str("\ndefine i64 @cinch_abi_apply0(i64 %closure) {\nentry:\n  %args = alloca i64\n  %result = call i64 @__apply(i64 %closure, i64 0, ptr %args)\n  ret i64 %result\n}\ndefine i64 @cinch_abi_apply1(i64 %closure, i64 %value) {\nentry:\n  %args = alloca i64\n  store i64 %value, ptr %args\n  %result = call i64 @__apply(i64 %closure, i64 1, ptr %args)\n  ret i64 %result\n}\n");
    std::fs::write(dir.join("library.ll"), ir).unwrap();
    std::fs::write(dir.join("caller.c"), r#"
#include <stdint.h>
#ifdef __APPLE__
#define CINCH_SYMBOL(name) "_" name
#else
#define CINCH_SYMBOL(name) name
#endif
extern uint64_t raw(uint64_t) __asm__(CINCH_SYMBOL("Library.identity@R"));
extern uint64_t object(uint64_t) __asm__(CINCH_SYMBOL("Library.identity@O"));
extern uint64_t tagged(uint64_t) __asm__(CINCH_SYMBOL("Library.identity@T"));
extern uint64_t wrap(uint64_t) __asm__(CINCH_SYMBOL("Library.wrap"));
extern uint64_t unwrap(uint64_t) __asm__(CINCH_SYMBOL("Library.unwrap"));
extern uint64_t partial(uint64_t) __asm__(CINCH_SYMBOL("Library.partial"));
extern uint64_t deferred(void) __asm__(CINCH_SYMBOL("Library.deferred"));
extern uint64_t indexed(uint64_t, uint64_t) __asm__(CINCH_SYMBOL("Library.indexed"));
extern uint64_t tagged_closure(void) __asm__(CINCH_SYMBOL("Library.identity@T.object-closure"));
extern uint64_t cinch_abi_apply0(uint64_t), cinch_abi_apply1(uint64_t, uint64_t);
extern uint64_t cinch_int_from_decimal(const char *, uint64_t), cinch_nat_from_decimal(const char *, uint64_t);
extern int64_t cinch_int_to_i64(uint64_t), cinch_int_cmp(uint64_t, uint64_t);
extern uint64_t cinch_nat_to_len(uint64_t);
extern void cinch_dup(uint64_t), cinch_drop(uint64_t), cinch_reclaim_enable(void);
extern int64_t cinch_exit_check(int64_t);
static uint64_t integer(const char *s, uint64_t n) { return cinch_int_from_decimal(s, n); }
int main(void) {
  cinch_reclaim_enable();
  if (raw(1) != 1) return 1;
  uint64_t large = integer("184467440737095516160", 21); cinch_dup(large);
  uint64_t kept = unwrap(object(wrap(tagged(large))));
  if (cinch_int_cmp(large, kept) != 0) return 2;
  cinch_drop(large); cinch_drop(kept);
  uint64_t value = cinch_abi_apply1(tagged_closure(), integer("42", 2));
  if (cinch_int_to_i64(value) != 42) return 3; cinch_drop(value);
  value = cinch_abi_apply1(partial(integer("10", 2)), integer("32", 2));
  if (cinch_int_to_i64(value) != 42) return 4; cinch_drop(value);
  value = cinch_abi_apply0(deferred());
  if (cinch_int_to_i64(value) != 42) return 5; cinch_drop(value);
  uint64_t count = indexed(cinch_nat_from_decimal("3", 1), cinch_nat_from_decimal("42", 2));
  if (cinch_nat_to_len(count) != 42) return 6; cinch_drop(count);
  return (int)cinch_exit_check(0);
}
"#).unwrap();
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let output = Command::new(clang).args(["-Wno-override-module", "library.ll", "caller.c"]).arg(runtime).args(["-lpthread", "-ldl", "-lm", "-o", "caller"]).current_dir(&dir).output().unwrap();
    assert!(output.status.success(), "actual ABI link: {}\n{}", String::from_utf8_lossy(&output.stderr), dir.display());
    let output = Command::new(dir.join("caller")).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "actual ABI caller: {}\n{}", String::from_utf8_lossy(&output.stderr), dir.display());
    let stderr = String::from_utf8_lossy(&output.stderr);assert!(stderr.contains("live=0 immortal=0 leaked=0"), "{stderr}");println!("public ABI caller: {}", stderr.trim());
    common::cleanup(&dir);
}
