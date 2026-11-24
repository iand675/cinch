//! Complete retained fields and ordered effectful predicates/yields over arbitrary UTF8 byte spans.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// The owned module contains concrete state, effectful iteration and pure Producer adapters.
const FIELDS: &str = include_str!("../prelude/Data/Bytes/Fields.cinch");
/// Consumers compare complete literal bytes and exact ordered callback observations.
const HELPERS: &str = include_str!("fixtures/byte-fields-consumer.cinchpart");

/// Encode every expected raw fragment without hashing or Unicode replacement.
fn strings(values: &Value) -> String {
    values.to_string()
}

/// Render independently recorded scalar, yield and mutation events as typed caller observations.
fn observations(events: &Value, mutable: bool) -> String {
    let values = events
        .as_array()
        .unwrap()
        .iter()
        .map(|event| {
            let kind = event["kind"].as_str().unwrap();
            let rune = event["rune"].as_u64().unwrap_or(0);
            if mutable {
                let number = match kind {
                    "predicate" => 0,
                    "yield" => 1,
                    "mutation" => 2,
                    _ => panic!("unknown callback"),
                };
                format!(
                    "(Callback {number} {rune} {} {})",
                    event.get("value").unwrap_or(&Value::String(String::new())),
                    event["input"]
                )
            } else if kind == "predicate" {
                format!("(Predicate {rune})")
            } else {
                format!("(Output {})", event["value"])
            }
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

/// Keep stable predicate modes explicit and independent of the library's default scanner.
fn mode(row: &Value) -> u8 {
    match row["mode"].as_str().unwrap() {
        "default" => 0,
        "space" => 1,
        "comma" => 2,
        "error" => 3,
        "nonascii" => 4,
        "even" => 5,
        other => panic!("unknown mode {other}"),
    }
}

/// Observe eager, persistent and repeated callback-factory outputs for one independent bounded span.
fn expression(row: &Value) -> String {
    let take = row["take"].as_i64().unwrap();
    format!(
        "fields-case {} {} {} {} {} {} {} {} {}",
        row["input"],
        row["start"],
        row["end"],
        mode(row),
        take.max(0),
        if take == 0 { "False" } else { "True" },
        strings(&row["eager"]),
        observations(&row["eager_events"], false),
        observations(&row["invocations"][0]["events"], false)
    )
}

/// Observe callback-time bytes and final retained aliases after both shared-state traversals.
fn effect_expression(row: &Value, control: usize) -> String {
    let mut text = format!(
        "effect-case {} {control} {} {}",
        row["input"],
        mode(row),
        row["take"].as_i64().unwrap().max(0)
    );
    for invocation in row["invocations"].as_array().unwrap() {
        text.push_str(&format!(
            " {} {} {}",
            observations(&invocation["events"], true),
            strings(&invocation["held_after"]),
            invocation["input_after"]
        ));
    }
    text
}

/// Bound each poisoned process independently while keeping a genuine mutation effect in all callbacks.
fn consumer(checks: &[String], width: usize) -> String {
    let mut text = HELPERS.to_owned();
    let mut dispatch = "False".to_owned();
    for (number, group) in checks.chunks(width).enumerate().rev() {
        text.push_str(&format!("\n  -- | Check complete field/effect group {number}.\n  group{number} :: forall (r :: Region). Unit ->{{Mut r}} Bool\n  group{number} unit =\n"));
        for (index, check) in group.iter().enumerate() {
            text.push_str(&format!("    let check{index} = {check} in\n"));
        }
        text.push_str(&format!(
            "    {}\n",
            (0..group.len())
                .map(|index| format!("check{index}"))
                .collect::<Vec<_>>()
                .join(" && ")
        ));
        dispatch = format!("if index == \"{number}\" then group{number} () else {dispatch}");
    }
    text.push_str(&format!("\n  -- | Run one bounded complete field observation.\n  main :: Unit ->{{IO r, Mut s}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index (Cons size Nil)) -> if index == \"deep\" then (let accepted = deep-case (decimal size) in if accepted then 0 else 1) else 2\n    Ok (Cons index Nil) -> let accepted = {dispatch} in if accepted then 0 else 1\n    other -> 2\n"));
    text
}

/// Supply the exact owned source closure without a repository fallback for private buffers or Unicode tables.
fn project(main: &str) -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Main", main),
        ("Data.Bytes.Fields", FIELDS),
        (
            "Data.Bytes.View",
            include_str!("../prelude/Data/Bytes/View.cinch"),
        ),
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Data.Unicode.Kernel",
            include_str!("../prelude/Data/Unicode/Kernel.cinch"),
        ),
        (
            "Data.Unicode.Tables",
            include_str!("../prelude/Data/Unicode/Tables.cinch"),
        ),
        (
            "Data.Mutable.Var",
            include_str!("../prelude/Data/Mutable/Var.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    directory
}

/// Use checked RC compilation and independent normal/poison memory processes at both optimization levels.
fn run_depth(checks: Vec<String>, width: usize, optimized: bool, depth: Option<usize>) {
    run_text(
        &consumer(&checks, width),
        checks.len().div_ceil(width),
        optimized,
        depth,
        "",
    );
}

/// Keep exact IO observations distinct from callback state and runtime ownership diagnostics.
fn run_text(main: &str, batches: usize, optimized: bool, depth: Option<usize>, stdout: &str) {
    let directory = project(main);
    let entry = directory.path().join("Main.cinch");
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    let output = executable.clone();
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            cinch::driver::driver_with_profile_opts(
                Mode::Build,
                cinch::check::StageMode::Refresh,
                &entry,
                Some(&output),
                &profile,
                &cinch::driver::DriverOptions {
                    opt_level: optimized.then_some(2),
                    ..Default::default()
                },
            )
        })
        .unwrap()
        .join()
        .unwrap();
    if let Err(error) = compiled {
        let retained = directory.keep();
        panic!(
            "field compilation failed: {error:?}; retained {}",
            retained.display()
        );
    }
    for debug in [false, true] {
        for index in 0..batches {
            let mut command = std::process::Command::new(&executable);
            if let Some(size) = depth {
                command.args(["deep".to_owned(), size.to_string()]);
            } else {
                command.arg(index.to_string());
            }
            let output = command
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .unwrap();
            if output.status.code() != Some(0)
                || !String::from_utf8_lossy(&output.stderr).contains("leaked=0")
                || output.stdout != stdout.as_bytes()
            {
                std::fs::write(
                    directory
                        .path()
                        .join(format!("run-{optimized}-{debug}-{index}.stdout")),
                    &output.stdout,
                )
                .unwrap();
                std::fs::write(
                    directory
                        .path()
                        .join(format!("run-{optimized}-{debug}-{index}.stderr")),
                    &output.stderr,
                )
                .unwrap();
                let retained = directory.keep();
                panic!(
                    "field group={index}, optimized={optimized}, poison={debug}: {}; retained {}",
                    String::from_utf8_lossy(&output.stderr),
                    retained.display()
                );
            }
        }
    }
}

/// Preserve the ordinary bounded observation entry point without a runtime stress size.
fn run(checks: Vec<String>, width: usize, optimized: bool) {
    run_depth(checks, width, optimized, None);
}

/// Every complete field and scalar/yield trace agrees with the pinned reference, including invalid bounded UTF8.
#[test]
fn fields_complete_outputs_callbacks_and_replay_match_independent_reference() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-fields-vectors.json")).unwrap();
    let checks: Vec<_> = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(expression)
        .collect();
    for optimized in [false, true] {
        run(checks.clone(), 8, optimized);
    }
}

/// Saved decode widths, future-byte mutation, early stop and aliases remain correct through actual effectful callbacks.
#[test]
fn fields_mutation_callbacks_and_temporary_owners_are_preserved() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-fields-vectors.json")).unwrap();
    let mut checks: Vec<_> = fixture["effect_controls"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, row)| effect_expression(row, index))
        .collect();
    checks.push("owner-case ()".to_owned());
    checks.push("stateful-case ()".to_owned());
    for optimized in [false, true] {
        run(checks.clone(), 1, optimized);
    }
}

/// Runtime-generated dense input keeps eager construction and effectful traversal bounded at an 8 MiB stack.
#[test]
fn fields_dense_runtime_construction_has_bounded_stack_and_reclaims_owners() {
    for optimized in [false, true] {
        run_depth(vec!["True".into()], 1, optimized, Some(65536));
    }
}

/// A combined IO/mutation callback row remains accepted, while hiding predicate effects is rejected.
#[test]
fn fields_preserve_combined_callback_rows_and_reject_effect_erasure() {
    for (accepted, source) in [
        (
            true,
            include_str!("fixtures/byte-fields-row-positive.cinch"),
        ),
        (
            false,
            include_str!("fixtures/byte-fields-row-negative.cinch"),
        ),
    ] {
        let directory = project(source);
        let result = common::check(directory.path(), "Main.cinch");
        if accepted {
            result.unwrap();
        } else {
            let diagnostics = result.expect_err("predicate effects must not disappear");
            assert!(
                diagnostics.contains("effect rows do not match"),
                "{diagnostics}"
            );
        }
    }
}

/// Actual IO and mutation effects interleave through a retained factory and stop without touching the next field.
#[test]
fn fields_distinct_io_predicate_and_mutation_yield_execute_in_order() {
    for optimized in [false, true] {
        run_text(
            include_str!("fixtures/byte-fields-mixed-runtime.cinch"),
            1,
            optimized,
            None,
            "a b a b ",
        );
    }
}

/// Retain actual definitions so an unrelated runtime load or unused function constant cannot satisfy reachability.
fn definitions(ir: &str) -> std::collections::BTreeMap<&str, &str> {
    let mut values = std::collections::BTreeMap::new();
    let mut offset = 0;
    for line in ir.split_inclusive('\n') {
        if line.starts_with("define ") {
            if let Some((_, suffix)) = line.split_once("@\"") {
                let (name, _) = suffix.split_once('"').unwrap();
                let end = offset + ir[offset..].find("\n}").unwrap() + 2;
                values.insert(name, &ir[offset..end]);
            }
        }
        offset += line.len();
    }
    values
}

/// Follow only complete direct callee operands through the emitted consumer and module closure.
fn reaches(functions: &std::collections::BTreeMap<&str, &str>, wanted: &str) -> bool {
    let mut pending = vec!["Main.main"];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(symbol) = pending.pop() {
        let specialized_collector = wanted == "Data.Bytes.Fields.collect-with"
            && symbol
                .strip_prefix(wanted)
                .and_then(|suffix| suffix.strip_prefix(".sp"))
                .is_some_and(|suffix| {
                    !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
                });
        let unboxed_worker = symbol
            .strip_prefix(wanted)
            .is_some_and(|suffix| suffix == "#cc" || suffix == "#cpr");
        if symbol == wanted || unboxed_worker || specialized_collector {
            return true;
        }
        if !seen.insert(symbol) {
            continue;
        }
        for line in functions[symbol]
            .lines()
            .filter(|line| line.contains("call "))
        {
            for suffix in line.split("@\"").skip(1) {
                if let Some((callee, following)) = suffix.split_once('"') {
                    if following.starts_with('(') && functions.contains_key(callee) {
                        pending.push(callee);
                    }
                }
            }
        }
    }
    false
}

/// Complete collectors and genuine mutation-effect traversal retain actual packed/scalar code on every target.
#[test]
fn fields_complete_adapter_compiles_for_wasm_arm_and_big_endian() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        include_str!("../prelude/Data/Bytes/View.cinch"),
        include_str!("../prelude/Data/Unicode/Tables.cinch"),
        include_str!("../prelude/Data/Unicode/Kernel.cinch"),
        include_str!("../prelude/Data/Mutable/Var.cinch"),
        FIELDS,
        include_str!("fixtures/byte-fields-portable.cinch"),
    ]
    .join("\n");
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
        let result = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(
                    &text,
                    "byte-fields-full.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap();
        let ir = result.unwrap_or_else(|error| {
            panic!("{triple}: {error:?}; retained {}", directory.display())
        });
        let file = directory.join(format!("{arch}.ll"));
        std::fs::write(&file, &ir).unwrap();
        let functions = definitions(&ir);
        for symbol in [
            "Data.Bytes.Fields.collect-space",
            "Data.Bytes.Fields.collect-with",
            "Data.Bytes.Fields.visit-with",
            "Data.Bytes.Fields.scan-space",
            "Data.Bytes.Fields.scan-with",
            "Data.Bytes.View.ascii-space-blocks",
            "Data.Bytes.View.ascii-space-tail",
            "cinch.runtime.bytesLoadU128",
            "cinch.runtime.bytesGetU8At",
            "cinch.prim.simdU8x16CmpLt",
            "cinch.prim.simdU8x16CmpEq",
            "cinch.prim.simdU8x16Movemask",
            "Data.Bytes.Utf8.utf8-decode-between",
            "Data.Unicode.Kernel.unicode-is-space-code",
        ] {
            assert!(
                reaches(&functions, symbol),
                "unreachable {symbol}; retained {}",
                file.display()
            );
        }
        let load128 = functions["cinch.runtime.bytesLoadU128"];
        assert!(load128.contains("load i128") && load128.contains("align 1"));
        assert!(!load128.contains("store "));
        if endian == "big" {
            assert!(load128.contains("@llvm.bswap.i128"));
        }
        let cmplt = functions["cinch.prim.simdU8x16CmpLt"];
        assert!(cmplt.contains("icmp ult <16 x i8>") && !cmplt.contains("store "));
        let cmpeq = functions["cinch.prim.simdU8x16CmpEq"];
        assert!(cmpeq.contains("icmp eq <16 x i8>") && !cmpeq.contains("store "));
        let movemask = functions["cinch.prim.simdU8x16Movemask"];
        assert!(movemask.contains("bitcast <16 x i1>") && !movemask.contains("store "));
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

/// Every new module declaration has a canonical Cinch-owned documentation comment.
#[test]
fn fields_have_complete_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        FIELDS,
        Some("Data.Bytes.Fields"),
        "Data.Bytes.Fields",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    assert!(declarations.len() >= 31);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing {}", declaration.name));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}
