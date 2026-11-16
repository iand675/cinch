//! Exact raw-byte UTF-8 progress, fresh repair, atomic encoding and target-portable read-only kernels.
mod common;

use cinch::driver::Mode;

/// Compile the real byte module and verify ordinary live space separately from poisoned frees.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Data.Bytes.Utf8", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true, rc_debug: true, ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(Mode::Build, cinch::check::StageMode::Refresh,
        directory.path(), "Main.cinch", Some(executable.clone()), &profile).unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1").env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output().unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}: {}\n{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Invalid numeric values, grouped repair, aliasing and exact-capacity refusal preserve all bytes.
#[test]
fn utf8_invalid_numeric_values_fresh_repair_and_atomic_encoding() {
    run(include_str!("fixtures/utf8-ownership-consumer.cinch"));
}

/// Every byte-layer declaration, constructor and arithmetic helper has canonical documentation.
#[test]
fn utf8_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Data/Bytes/Utf8.cinch");
    let name = "Data.Bytes.Utf8";
    let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration.doc.as_ref().unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Actual byte arithmetic and exact-load kernels compile for wasm, ARM and a big-endian object.
#[test]
fn utf8_readonly_kernels_compile_for_foreign_targets() {
    let source = include_str!("../prelude/Data/Bytes/Utf8.cinch");
    let mut kernels = String::from("-- | Actual private UTF-8 arithmetic and source loads.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n");
    for name in ["load-one", "load-four", "load-eight", "ascii-eight", "decode-word", "encode-word"] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str("\n  -- | Isolated actual checked arithmetic or caller-proved exact source load.\n");
        kernels.push_str(&rest[..end]);
    }
    kernels.push_str("\n  -- | Retain source storage until raw reads finish.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Reach all kernels without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 8 in let base = bytes-data bytes in let one = keep bytes (load-one base) in let four = keep bytes (load-four base) in let eight = keep bytes (load-eight base) in let decoded = decode-word four one in let encoded = encode-word (wrapping-u32-to-i32 decoded) in if ascii-eight eight && eq-u64 encoded 0 then 0 else 1\n");
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()), ..Default::default()
        }).unwrap_or_else(|_| {
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into(); profile.arch = arch.into(); profile.endian = endian.into(); profile
        });
        let input = kernels.clone();
        let ir = std::thread::Builder::new().stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(&input, "utf8-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            }).unwrap().join().unwrap().unwrap();
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, &ir).unwrap();
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"]).arg(&path)
            .arg("-o").arg(path.with_extension("o")).output().unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}
