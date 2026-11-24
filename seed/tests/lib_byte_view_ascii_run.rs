//! Owner-sealed SIMD ASCII class runs with exact logical view bounds and scalar fringes.
mod common;
use cinch::driver::Mode;

/// The complete module retains backing privately and holds its owner around each bounded traversal.
const VIEW: &str = include_str!("../prelude/Data/Bytes/View.cinch");
/// Independent endpoint and alias observations use only the public sealed-view API.
const HELPERS: &str = include_str!("fixtures/byte-view-ascii-run-consumer.cinchpart");
/// A complete public consumer roots real packed and scalar code in every foreign object.
const PORTABLE: &str = include_str!("fixtures/byte-view-ascii-run-portable.cinch");

/// Encode an independent fixed byte recipe without invoking the implementation under test.
fn hex(raw: &[u8]) -> String {
    raw.iter().map(|value| format!("{value:02x}")).collect()
}

/// Cover all exact bytes, alignments, positions and mismatches on both sides of every SIMD fringe.
fn checks() -> Vec<String> {
    let mut result = Vec::new();
    for value in 0..=255u8 {
        let white = matches!(value, 9..=13 | 32);
        result.push(format!(
            "uniform-case \"{}\" {} {}",
            hex(&[value; 49]),
            if white { "True" } else { "False" },
            if value < 128 && !white {
                "True"
            } else {
                "False"
            }
        ));
    }
    for (fill, spaces) in [(32u8, true), (122, false)] {
        for stop in [9u8, 32, 122, 128, 133, 255] {
            if stop < 128 && matches!(stop, 9..=13 | 32) == spaces {
                continue;
            }
            for size in [15usize, 16, 17, 31, 32, 33, 63, 64, 65] {
                for lane in 0..size.min(17) {
                    let mut raw = vec![fill; size + 2];
                    raw[0] = 255;
                    raw[size + 1] = 255;
                    raw[lane + 1] = stop;
                    result.push(format!(
                        "mismatch-case \"{}\" {lane} {} 1 {}",
                        hex(&raw),
                        if spaces { "True" } else { "False" },
                        size + 1
                    ));
                }
            }
        }
    }
    result.push("ownership-case ()".into());
    result
}

/// Bound each ordinary/poison process independently of the total reference matrix size.
fn consumer(checks: &[String], width: usize) -> String {
    let mut text = HELPERS.to_owned();
    let mut dispatch = "False".to_owned();
    for (number, group) in checks.chunks(width).enumerate().rev() {
        text.push_str(&format!("\n  -- | Run exact run-endpoint group {number}.\n  group{number} :: Unit -> Bool\n  group{number} unit = all [\n{}\n    ]\n",
            group.iter().map(|value| format!("    ({value})")).collect::<Vec<_>>().join(",\n")));
        dispatch = format!("if index == \"{number}\" then group{number} () else {dispatch}");
    }
    text.push_str(&format!("\n  -- | Select one independent bounded run observation.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> let accepted = {dispatch} in if accepted then 0 else 1\n    other -> 2\n"));
    text
}

/// Keep the complete source-distributed worker closure beside the actual caller.
fn project(main: &str) -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Main", main),
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
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    directory
}

/// Compile checked-RC public consumers and retain an exact project if a memory profile fails.
fn run(optimized: bool) {
    let expressions = checks();
    assert_eq!(expressions.len(), 1607);
    let directory = project(&consumer(&expressions, 8));
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    let compiled = common::drive_with_profile_opts(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
        &cinch::driver::DriverOptions {
            opt_level: optimized.then_some(2),
            ..Default::default()
        },
    );
    if let Err(error) = compiled {
        let retained = directory.keep();
        panic!(
            "run observer compile failed: {error:?}; retained {}",
            retained.display()
        );
    }
    for debug in [false, true] {
        for group in 0..expressions.len().div_ceil(8) {
            let result = std::process::Command::new(&executable)
                .arg(group.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .unwrap();
            if result.status.code() != Some(0)
                || !String::from_utf8_lossy(&result.stderr).contains("leaked=0")
            {
                std::fs::write(
                    directory
                        .path()
                        .join(format!("run-{optimized}-{debug}-{group}.stderr")),
                    &result.stderr,
                )
                .unwrap();
                let retained = directory.keep();
                panic!(
                    "group={group}, O2={optimized}, poison={debug}: {}; retained {}",
                    String::from_utf8_lossy(&result.stderr),
                    retained.display()
                );
            }
        }
    }
}

/// Every exact byte, alignment, mismatch and fringe remains within the sealed span at both optimization levels.
#[test]
fn ascii_run_full_byte_alignment_fringe_and_owner_matrix() {
    run(false);
    run(true);
}

/// Find a complete emitted definition rather than accepting unrelated runtime loads.
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
    panic!("missing actual definition {symbol}");
}

/// Follow actual direct callee operands, excluding function constants passed as ordinary arguments.
fn reaches(ir: &str, start: &str, wanted: &str) -> bool {
    let mut pending = vec![start.to_owned()];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(symbol) = pending.pop() {
        if symbol == wanted {
            return true;
        }
        if !seen.insert(symbol.clone()) {
            continue;
        }
        for line in definition(ir, &symbol)
            .lines()
            .filter(|line| line.contains("call "))
        {
            for suffix in line.split("@\"").skip(1) {
                let Some((callee, following)) = suffix.split_once('"') else {
                    continue;
                };
                if following.starts_with('(')
                    && (callee.starts_with("Data.Bytes.")
                        || callee.starts_with("Data.Wide.")
                        || callee.starts_with("Data.Machine.")
                        || callee.starts_with("cinch.runtime.")
                        || callee.starts_with("cinch.prim.")
                        || callee.starts_with("Main."))
                {
                    pending.push(callee.to_owned());
                }
            }
        }
    }
    false
}

/// Emit and compile actual full adapter objects on little- and big-endian foreign targets.
#[test]
fn ascii_run_full_adapter_compiles_for_wasm_arm_and_big_endian() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        VIEW,
        PORTABLE,
    ]
    .join("\n");
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
                    "byte-ascii-run-full.cinch",
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
        for worker in [
            "Data.Bytes.View.ascii-space-blocks",
            "Data.Bytes.View.ascii-space-tail",
            "cinch.runtime.bytesLoadU128",
            "cinch.runtime.bytesGetU8At",
            "cinch.prim.simdU8x16Sub",
            "cinch.prim.simdU8x16CmpLt",
            "cinch.prim.simdU8x16CmpEq",
            "cinch.prim.simdU8x16Movemask",
            "cinch.prim.trailingZerosU32",
        ] {
            assert!(
                reaches(&ir, "Main.main", worker),
                "unreachable {worker}; retained {}",
                file.display()
            );
        }
        let load128 = definition(&ir, "cinch.runtime.bytesLoadU128");
        assert!(load128.contains("load i128") && load128.contains("align 1"));
        assert!(!load128.contains("store "));
        if endian == "big" {
            assert!(load128.contains("@llvm.bswap.i128"));
        }
        let cmplt = definition(&ir, "cinch.prim.simdU8x16CmpLt");
        assert!(cmplt.contains("icmp ult <16 x i8>") && !cmplt.contains("store "));
        let cmpeq = definition(&ir, "cinch.prim.simdU8x16CmpEq");
        assert!(cmpeq.contains("icmp eq <16 x i8>") && !cmpeq.contains("store "));
        let movemask = definition(&ir, "cinch.prim.simdU8x16Movemask");
        assert!(movemask.contains("bitcast <16 x i1>") && !movemask.contains("store "));
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
        assert!(
            output.status.success(),
            "{triple}: {}; retained {}",
            String::from_utf8_lossy(&output.stderr),
            directory.display()
        );
    }
}

/// Canonical documentation explains exact class sets, bounds, alias observation and owner retention.
#[test]
fn ascii_run_has_canonical_owned_documentation() {
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
    for name in [
        "ascii-space-mask",
        "ascii-space-byte",
        "ascii-space-tail",
        "ascii-space-blocks",
        "byte-view-ascii-space-run-end",
    ] {
        let item = declarations.iter().find(|item| item.name == name).unwrap();
        let doc = item.doc.as_ref().unwrap();
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}
