//! Canonical documentation and real public-CLI multipart runtime regressions.

/// Every public/private scanner, session, backend adapter and constructor has canonical prose.
#[test]
fn multipart_declarations_have_canonical_documentation() {
    for (module, source) in [
        (
            "Text.Multipart",
            include_str!("../prelude/Text/Multipart.cinch"),
        ),
        (
            "Effect.Multipart",
            include_str!("../prelude/Effect/Multipart.cinch"),
        ),
    ] {
        let parsed =
            cinch::cinchdoc::parse_source(source, Some(module), module, None, false).unwrap();
        assert!(parsed.doc.is_some(), "{module}");
        assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
        assert!(parsed.orphans.is_empty(), "{:?}", parsed.orphans);
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("{}: missing canonical documentation", declaration.id));
            assert!(!doc.legacy, "{}", declaration.id);
            let prose = doc.markdown.to_lowercase();
            assert!(!prose.contains("golang") && !prose.contains("go's") && !prose.contains("go "));
        }
    }
}

/// Current Rust-only CLI consumers execute committed values, exact stream traces
/// and mutation controls with the real foreign object under both heap profiles.
/// An ordinary compiler build refreshes and captures the exact host runtime.
#[test]
fn rust_cli_executes_multipart_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;

    for selector in [
        "CINCH_TEST_COMPILER",
        "CINCH_TEST_INTERFACES",
        "CINCH_TEST_OBJECTS",
        "CINCH_TEST_RUNTIME_ARCHIVE",
    ] {
        assert!(std::env::var_os(selector).is_none(), "Rust-only multipart regression refuses {selector}; use the explicit standalone Native runner");
    }
    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = tempfile::tempdir().unwrap();
    let result = Command::new("python3")
        .arg(repository.join("tooling/check_multipart.py"))
        .arg("--work")
        .arg(work.path())
        .arg("--rust")
        .arg(env!("CARGO_BIN_EXE_cinch"))
        .arg("--rust-runtime")
        .arg(&runtime)
        .arg("--initialize-rust-runtime")
        .args([
            "--group-size",
            "32",
            "--form-group-size",
            "32",
            "--only",
            "dispositions-0288",
            "readers-0000",
            "late-headers-0000",
            "writers-0000",
            "forms-0000",
            "ownership-0000",
            "entropy-0000",
        ])
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("execute committed multipart corpus through the current Rust CLI");
    assert!(
        result.status.success(),
        "Rust multipart regression failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(work.path().join("report.json")).unwrap())
            .unwrap();
    assert_eq!(report["compiler_host_stack_bytes"], 64 * 1024 * 1024);
    assert_eq!(report["frontends"].as_object().unwrap().len(), 1);
    assert_eq!(
        report["runtime_initialization"]["archive_path"].as_str(),
        runtime.to_str()
    );
    assert_eq!(
        report["frontends"]["rust"]["runtime_sha256"],
        report["runtime_initialization"]["archive_sha256"]
    );
    assert_eq!(
        report["runtime_initialization"]["snapshot_sha256"],
        report["runtime_initialization"]["archive_sha256"]
    );
    let rows = report["runs"].as_array().unwrap();
    assert_eq!(rows.len(), 7);
    for row in rows {
        assert_eq!(row["frontend"], "rust");
        let profiles = row["profiles"].as_array().unwrap();
        assert_eq!(profiles.len(), 2);
        for (profile, (poison, heap)) in profiles.iter().zip([("0", "32M"), ("1", "256M")]) {
            assert_eq!(profile["poison"], poison);
            assert_eq!(profile["heap"], heap);
            assert_eq!(profile["stack"], "8M");
            assert!(profile["stderr"].as_str().unwrap().contains("leaked=0"));
        }
    }
}

/// Find the actual checked body rather than an unrelated intrinsic declaration.
fn multipart_definition<'a>(ir: &'a str, symbol: &str) -> &'a str {
    let marker = format!("@\"{symbol}\"(");
    let mut offset = 0;
    for line in ir.split_inclusive('\n') {
        if line.starts_with("define ") && line.contains(&marker) {
            let end = offset + ir[offset..].find("\n}").unwrap() + 2;
            return &ir[offset..end];
        }
        offset += line.len();
    }
    panic!("missing actual multipart definition {symbol}");
}

/// Follow emitted calls through public aliases and private scanner continuations.
fn multipart_call_path(ir: &str, wanted: &str) -> Option<Vec<String>> {
    let mut pending = vec![vec!["Main.main".to_owned()]];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(path) = pending.pop() {
        let symbol = path.last().unwrap();
        if symbol == wanted {
            return Some(path);
        }
        if !seen.insert(symbol.clone()) {
            continue;
        }
        for line in multipart_definition(ir, symbol)
            .lines()
            .filter(|line| line.contains("call "))
        {
            for suffix in line.split("@\"").skip(1) {
                let Some((callee, after)) = suffix.split_once('"') else {
                    continue;
                };
                if after.starts_with('(')
                    && (callee.starts_with("Text.Multipart.") || callee.starts_with("Main."))
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

/// Complete public scanner and disposition paths produce real little/big-endian foreign objects.
#[test]
fn multipart_public_paths_compile_for_wasm_arm_and_big_endian() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Data/Unicode/Tables.cinch"),
        include_str!("../prelude/Data/Unicode/Kernel.cinch"),
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        include_str!("../prelude/Text/Base64/Encoding.cinch"),
        include_str!("../prelude/Text/Mime.cinch"),
        include_str!("../prelude/Text/Multipart.cinch"),
        include_str!("fixtures/multipart-portable.cinch"),
    ]
    .join("\n");
    // Keep all target IR and object diagnostics if a later assertion fails.
    let directory = tempfile::tempdir().unwrap().keep();
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
                    "multipart-public.cinch",
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
        for worker in [
            "Text.Multipart.find-byte",
            "Text.Multipart.prefix-at",
            "Text.Multipart.escape-end",
            "Text.Multipart.equal-sixteen.llvm",
        ] {
            assert!(
                multipart_call_path(&ir, worker).is_some(),
                "missing public path to {worker}; retained {}",
                file.display()
            );
        }
        let matcher = multipart_definition(&ir, "Text.Multipart.equal-sixteen.llvm");
        assert!(matcher.contains("icmp eq <16 x i8>") && matcher.contains("bitcast <16 x i1>"));
        assert_eq!(
            matcher.contains("%swapped0 = call i64 @llvm.bswap.i64"),
            endian == "big"
        );
        assert_eq!(
            matcher.contains("%reversed = shufflevector <16 x i1>"),
            endian == "big"
        );
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&file)
        .arg("-o")
        .arg(file.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}\nretained {}",
            String::from_utf8_lossy(&output.stderr),
            directory.display()
        );
    }
}
