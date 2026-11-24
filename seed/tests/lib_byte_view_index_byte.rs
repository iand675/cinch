//! Bounded exact-byte observations preserve the private view owner and logical endpoints.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// The maintained public module owns its private backing and bounded packed traversal.
const VIEW: &str = include_str!("../prelude/Data/Bytes/View.cinch");
/// Shared input recipes are independent of the new query's implementation.
const INPUTS: &str = include_str!("fixtures/byte-view-consumer.cinchpart");
/// Every result is mixed in ordered complete-field fingerprints.
const HELPERS: &str = include_str!("fixtures/byte-view-index-byte-consumer.cinchpart");
/// Full public adapter calls force actual complete-block and scalar-fringe paths in object code.
const PORTABLE: &str = include_str!("fixtures/byte-view-index-byte-portable.cinch");

/// Inspect only a named complete definition so unrelated runtime header loads cannot satisfy coverage.
fn definition<'a>(ir: &'a str, symbol: &str) -> &'a str {
    let marker = format!("@\"{symbol}\"(");
    let mut offset = 0;
    for line in ir.split_inclusive('\n') {
        if line.starts_with("define ") && line.contains(&marker) {
            let end = offset + ir[offset..].find("\n}").unwrap() + 2;
            return &ir[offset..end];
        }
        offset += line.len();
    }
    panic!("missing actual function definition {symbol}");
}

/// Follow emitted direct calls while allowing the compiler to inline an intermediate adapter.
fn call_path(ir: &str, start: &str, wanted: &str) -> Option<Vec<String>> {
    let mut pending = vec![vec![start.to_owned()]];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(path) = pending.pop() {
        let symbol = path.last().unwrap();
        if symbol == wanted {
            return Some(path);
        }
        if !seen.insert(symbol.clone()) {
            continue;
        }
        for line in definition(ir, symbol)
            .lines()
            .filter(|line| line.contains("call "))
        {
            for suffix in line.split("@\"").skip(1) {
                let Some((callee, _)) = suffix.split_once('"') else {
                    continue;
                };
                if callee.starts_with("Data.Bytes.")
                    || callee.starts_with("Data.Wide.")
                    || callee.starts_with("Data.Machine.")
                    || callee.starts_with("cinch.runtime.")
                    || callee.starts_with("cinch.prim.")
                {
                    let mut next = path.clone();
                    next.push(callee.to_owned());
                    pending.push(next);
                }
            }
        }
    }
    None
}

/// Select complete independent fingerprints without accumulating poisoned output allocations.
fn consumer(checks: &[String]) -> String {
    let mut source = INPUTS[..INPUTS
        .find("  -- | Compare an optional relative byte offset")
        .unwrap()]
        .replace(
            "  import Data.String\n",
            "  import Data.String\n  import Text.Hex\n",
        );
    source.push_str(HELPERS);
    let mut dispatch = "False".to_owned();
    for (index, check) in checks.iter().enumerate().rev() {
        source.push_str(&format!("\n  -- | Observe one complete exact-byte owner group.\n  group{index} :: Unit -> Bool\n  group{index} unit = {check}\n"));
        dispatch = format!("if index == \"{index}\" then group{index} () else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one complete independent query group.\n  selected :: String -> Bool\n  selected index = {dispatch}\n  -- | Observe complete byte-query and ownership results.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n"));
    source
}

/// Compile only the exact reachable source-distributed closure beside an isolated entrypoint.
fn run(checks: Vec<String>, optimized: bool) {
    let directory = tempfile::tempdir().unwrap();
    for (name, text) in [
        ("Data.Bytes.View", VIEW),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), text).unwrap();
    }
    let entry = directory.path().join("Main.cinch");
    let exe = directory.path().join("consumer");
    std::fs::write(&entry, consumer(&checks)).unwrap();
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized {
        let output = exe.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&output),
                    &profile,
                    &cinch::driver::DriverOptions {
                        opt_level: Some(2),
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
    } else {
        common::drive_with_profile(
            Mode::Build,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            Some(exe.clone()),
            &profile,
        )
        .unwrap();
    }
    for debug in [false, true] {
        for index in 0..checks.len() {
            let output = std::process::Command::new(&exe)
                .arg(index.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "index={index}, debug={debug}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
        }
    }
}

/// Every short input span, exact byte value and larger packed-word fringe matches the independent SDK.
#[test]
fn byte_view_index_byte_matches_complete_independent_span_fingerprints() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-view-index-byte-vectors.json")).unwrap();
    let checks = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "index-case {} {} {} {}",
                row["mode"], row["size"], row["raw_sha256"], row["hash"]
            )
        })
        .collect();
    run(checks, false);
}

/// Temporary owners, mutation visibility, dropped prefixes and empty bounds remain valid at both optimization levels.
#[test]
fn byte_view_index_byte_preserves_owner_and_alias_semantics() {
    run(vec!["index-owner-case ()".into(); 10], false);
    run(vec!["index-owner-case ()".into(); 10], true);
}

/// The complete public view adapter and packed/scalar workers remain in real foreign-target object code.
#[test]
fn byte_view_index_byte_full_adapter_compiles_for_wasm_arm_and_big_endian() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        VIEW,
        PORTABLE,
    ]
    .join("\n");
    // Retain all target IR even when a subsequent graph assertion or object build fails.
    let directory = tempfile::tempdir().unwrap().keep();
    let mut artifacts = Vec::new();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..Default::default()
        })
        .unwrap_or_else(|_| {
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let text = source.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(
                    &text,
                    "byte-view-index-byte-full.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        let file = directory.join(format!("{arch}.ll"));
        std::fs::write(&file, &ir).unwrap();
        artifacts.push((triple, endian, file, ir));
    }
    for (triple, endian, file, ir) in artifacts {
        for (caller, callee) in [
            (
                "Data.Bytes.View.byte-view-index-byte",
                "Data.Bytes.View.index-forward-blocks",
            ),
            (
                "Data.Bytes.View.byte-view-index-byte",
                "Data.Bytes.View.index-forward-tail",
            ),
            (
                "Data.Bytes.View.index-forward-blocks",
                "cinch.runtime.bytesLoadU128",
            ),
            (
                "Data.Bytes.View.index-forward-blocks",
                "cinch.prim.simdU8x16CmpEq",
            ),
            (
                "Data.Bytes.View.index-forward-blocks",
                "cinch.prim.simdU8x16Movemask",
            ),
            (
                "Data.Bytes.View.index-forward-blocks",
                "cinch.prim.trailingZerosU32",
            ),
            (
                "Data.Bytes.View.index-forward-tail",
                "cinch.runtime.bytesGetU8At",
            ),
        ] {
            assert!(
                call_path(&ir, caller, callee).is_some(),
                "missing real call path {caller} -> {callee}; retained {}",
                file.display()
            );
        }
        if ir.lines().any(|line| {
            line.starts_with("define ") && line.contains("@\"Data.Bytes.View.index-forward\"(")
        }) {
            assert!(
                call_path(
                    &ir,
                    "Data.Bytes.View.byte-view-index-byte",
                    "Data.Bytes.View.index-forward"
                )
                .is_some(),
                "retained intermediary must be reachable; retained {}",
                file.display()
            );
            for worker in ["index-forward-blocks", "index-forward-tail"] {
                assert!(
                    call_path(
                        &ir,
                        "Data.Bytes.View.index-forward",
                        &format!("Data.Bytes.View.{worker}")
                    )
                    .is_some(),
                    "retained intermediary must reach {worker}; retained {}",
                    file.display()
                );
            }
        }
        let load128 = definition(&ir, "cinch.runtime.bytesLoadU128");
        assert!(load128.contains("load i128") && load128.contains("align 1"));
        if endian == "big" {
            assert!(load128.contains("@llvm.bswap.i128"));
        }
        let load8 = definition(&ir, "cinch.runtime.bytesGetU8At");
        assert!(load8.contains("load i8") && load8.contains("align 1"));
        assert!(definition(&ir, "cinch.prim.simdU8x16CmpEq").contains("icmp eq <16 x i8>"));
        assert!(definition(&ir, "cinch.prim.simdU8x16Movemask").contains("bitcast <16 x i1>"));
        assert!(definition(&ir, "cinch.prim.trailingZerosU32").contains("@llvm.cttz.i32"));
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&file)
        .arg("-o")
        .arg(file.with_extension("o"))
        .output()
        .unwrap();
        if !output.status.success() {
            panic!(
                "{triple}: {}\nretained full-adapter inputs: {}",
                String::from_utf8_lossy(&output.stderr),
                directory.display()
            );
        }
    }
}

/// The added exact-byte query has canonical owned documentation and an owner-sealed public type.
#[test]
fn byte_view_index_byte_has_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        VIEW,
        Some("Data.Bytes.View"),
        "Data.Bytes.View",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    let declaration = declarations
        .iter()
        .find(|item| item.name == "byte-view-index-byte")
        .unwrap();
    let doc = declaration.doc.as_ref().unwrap();
    assert!(!doc.legacy);
    let text = doc.markdown.to_lowercase();
    assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
}
