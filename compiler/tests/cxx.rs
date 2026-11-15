//! C++ interop through generated, ordinary checked foreign declarations.
mod common;
use cinch::cxx::{generate, Bindings};
use common::source_frontend::{check_sources, run_source_objects};
use std::process::Command;

fn spec() -> Bindings {
    serde_json::from_str(include_str!("../../examples/cxx/bindings.json")).unwrap()
}

fn source(bindings: &Bindings, body: &str) -> String {
    let (module, _) = generate(bindings).unwrap();
    // Keep one module for the source-string driver; the CLI example exercises
    // the separate-module form of exactly these declarations.
    format!(
        "{}\n{}",
        module.replace(
            &format!("module {} where", bindings.module),
            "module Main where"
        ),
        body
    )
}

#[test]
fn generated_bindings_check_and_ownership_is_enforced() {
    let valid = source(&spec(), "  main :: Unit ->{Foreign Cxx, IO, Diverge} Int\n  main _ = let c = newCounter 40 in let n = addCounter c 2 in let u = deleteCounter c in if n == 42 then 0 else 1\n");
    check_sources(&valid, "<cxx>").unwrap();
    let double = source(&spec(), "  main :: Unit ->{Foreign Cxx, IO, Diverge} Unit\n  main _ = let c = newCounter 0 in let u = deleteCounter c in deleteCounter c\n");
    assert!(
        check_sources(&double, "<cxx>").is_err(),
        "double destruction must fail"
    );
    let stale = source(&spec(), "  main :: Unit ->{Foreign Cxx, IO, Diverge} I64\n  main _ = let c = newCounter 0 in let u = deleteCounter c in getCounter c\n");
    assert!(
        check_sources(&stale, "<cxx>").is_err(),
        "use after destruction must fail"
    );
    let leak = source(&spec(), "  main :: Unit ->{Foreign Cxx, IO, Diverge} I64\n  main _ = let c = newCounter 0 in getCounter c\n");
    assert!(
        check_sources(&leak, "<cxx>").is_err(),
        "an owned object must be consumed"
    );
}

#[test]
fn rejects_unsupported_types_names_and_schema_typos() {
    let mut s = spec();
    s.classes[0].methods[0].result = "Text".into();
    assert!(generate(&s)
        .unwrap_err()
        .to_string()
        .contains("unsupported boundary type"));
    s = spec();
    s.classes[0].destructor = s.classes[0].constructor.clone();
    assert!(generate(&s).is_err());
    s = spec();
    s.classes[0].name = "I64".into();
    assert!(generate(&s).is_err());
    s = spec();
    s.functions[0].name = "signature".into();
    assert!(generate(&s).is_err());
    s = spec();
    s.header = "bad\"\n#error injected".into();
    assert!(generate(&s).is_err());
    s = spec();
    s.functions[0].cpp = "demo::f(); abort".into();
    assert!(generate(&s).is_err());
    assert!(
        serde_json::from_str::<Bindings>(r#"{"module":"X","header":"x.h","clasess":[]}"#).is_err()
    );
}

#[test]
fn cpp_construction_methods_and_destruction_execute_natively() {
    let dir = tempfile::tempdir().unwrap();
    let (_, cpp) = generate(&spec()).unwrap();
    std::fs::write(dir.path().join("Counter.cpp"), cpp).unwrap();
    let object = dir.path().join("Counter.o");
    let cxx = std::env::var("CINCH_CXX").unwrap_or_else(|_| "c++".into());
    let output = Command::new(&cxx)
        .args(["-std=c++17", "-c", "-fPIC"])
        .arg(dir.path().join("Counter.cpp"))
        .arg("-I")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/cxx"))
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let program = source(&spec(), "  main :: Unit ->{Foreign Cxx, IO, Diverge} Int\n  main _ = let c = newCounter 40 in let n = addCounter c 2 in let u = deleteCounter c in let alive = liveCount () in if n == 42 then (if alive == 0 then 0 else 2) else 1\n");
    let stdlib = if cfg!(target_os = "macos") {
        "-lc++"
    } else {
        "-lstdc++"
    };
    assert_eq!(
        run_source_objects(&program, &[object.to_str().unwrap(), stdlib]).unwrap(),
        0
    );
}

#[test]
fn exceptions_cannot_unwind_across_the_boundary() {
    let mut s = spec();
    s.functions[0].cpp = "demo::throwing".into();
    let (_, mut cpp) = generate(&s).unwrap();
    // A custom terminate handler makes this test deterministic without a core
    // dump; reaching the outer catch would mean the boundary leaked unwind.
    cpp.push_str("\n#include <cstdlib>\nint main() { std::set_terminate([] { std::_Exit(86); }); try { cinch_cxx_Counter_liveCount(); } catch (...) { return 1; } return 2; }\n");
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("exception.cpp");
    let exe = dir.path().join("exception");
    std::fs::write(&input, cpp).unwrap();
    let output = Command::new(std::env::var("CINCH_CXX").unwrap_or_else(|_| "c++".into()))
        .arg("-std=c++17")
        .arg(&input)
        .arg("-I")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/cxx"))
        .arg("-o")
        .arg(&exe)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(Command::new(exe).status().unwrap().code(), Some(86));
}

fn direct_spec() -> Bindings {
    serde_json::from_str(include_str!("../../examples/cxx/direct/bindings.json")).unwrap()
}

#[test]
fn direct_imports_call_real_cpp_symbols_without_shims() {
    let bindings = direct_spec();
    let (module, cpp) = generate(&bindings).unwrap();
    assert!(cpp.is_empty());
    assert!(!module.contains("cinch_cxx_"));
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("native.cpp");
    let object = dir.path().join("native.o");
    // This is the library implementation, with ordinary C++ linkage. There
    // are no extern-C wrappers and the generator's C++ output is empty.
    std::fs::write(&input, include_str!("../../examples/cxx/direct/Native.cpp")).unwrap();
    let output = Command::new(std::env::var("CINCH_CXX").unwrap_or_else(|_| "c++".into()))
        .args(["-std=c++17", "-c", "-fPIC"])
        .arg(&input)
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let program = source(&bindings, "  main :: Unit ->{Foreign Cxx, IO, Diverge} Int\n  main _ = let h = openHandle () in let n = readHandle h in let r = add n 2 in let u = closeHandle h in let alive = liveCount () in if r == 42 then (if alive == 0 then 0 else 2) else 1\n");
    let stdlib = if cfg!(target_os = "macos") {
        "-lc++"
    } else {
        "-lstdc++"
    };
    assert_eq!(
        run_source_objects(&program, &[object.to_str().unwrap(), stdlib]).unwrap(),
        0
    );
}

#[test]
fn direct_signatures_use_native_validation_and_reject_extra_declarations() {
    let mut s = direct_spec();
    s.imports[4].signature = "I64 ->{Foreign Cxx, IO} Text".into();
    let program = source(&s, "  main :: Unit -> Int\n  main _ = 0\n");
    assert!(check_sources(&program, "<direct>").is_err());
    s.imports[4].signature =
        "I64 ->{Foreign Cxx, IO} I64\n  injected :: I64\n  injected = 42".into();
    assert!(generate(&s).is_err());
    s = direct_spec();
    s.imports[4].symbol.clear();
    assert!(generate(&s).is_err());
    s = direct_spec();
    s.imports[4].name = s.imports[0].name.clone();
    assert!(generate(&s).is_err());
}

#[test]
fn direct_only_cli_emits_only_the_cinch_module() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("bindings.json");
    let out = dir.path().join("out");
    std::fs::write(&input, r#"{"module":"Native.Math","imports":[{"name":"add","symbol":"_ZN6direct3addExx","signature":"I64 -> I64 ->{Foreign Cxx, IO} I64"}]}"#).unwrap();
    assert_eq!(
        cinch::cxx::cli(&[
            input.to_str().unwrap().into(),
            "--out-dir".into(),
            out.to_str().unwrap().into()
        ]),
        0
    );
    let module = std::fs::read_to_string(out.join("Native/Math.cinch")).unwrap();
    assert!(module.contains("foreign import ccall \"_ZN6direct3addExx\" add"));
    assert!(!out.join("Native_Math.cpp").exists());
}
