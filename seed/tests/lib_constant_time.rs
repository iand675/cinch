//! Complete scalar and buffer observations, ownership, bounds, and portable kernels.
mod common;

/// Source-distributed operations retain exact bounds and public error dimensions.
const LIBRARY: &str = include_str!("../prelude/Crypto/ConstantTime.cinch");
/// Independent reference observations include every byte pair and complete output hashes.
const CONSUMER: &str = include_str!("fixtures/constant-time-consumer.cinch");
/// Packed read/write adapters preserve wire order on either host endianness.
const WORD: &str = include_str!("../prelude/Data/Bytes/Word.cinch");

/// Prepare the exact source closure without relying on installed package artifacts.
fn prepare(directory: &std::path::Path, consumer: &str) {
    for (name, source) in [
        ("Crypto.ConstantTime.cinch", LIBRARY),
        ("Data.Bytes.Word.cinch", WORD),
        ("Main.cinch", consumer),
    ] {
        std::fs::write(directory.join(name), source).unwrap();
    }
}

/// Both optimization levels consume every independent result with checked owner counts.
#[test]
fn constant_time_scalars_buffers_aliases_and_atomic_errors_match_reference() {
    for level in [0, 2] {
        let directory = tempfile::tempdir().unwrap();
        prepare(directory.path(), CONSUMER);
        let entry = directory.path().join("Main.cinch");
        let executable = directory.path().join("consumer");
        let output = executable.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let profile = cinch::core::TargetProfile {
                    reclaim: true,
                    rc_debug: true,
                    ..cinch::core::TargetProfile::native()
                };
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&output),
                    &profile,
                    &cinch::driver::DriverOptions {
                        opt_level: Some(level),
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        for poisoned in [false, true] {
            for group in 0..61 {
                let result = std::process::Command::new(&executable)
                    .arg(group.to_string())
                    .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
                    .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
                    .env("CINCH_STACK_SIZE", "8M")
                    .env("CINCH_LEAK_CHECK", "1")
                    .output()
                    .unwrap();
                let diagnostic = String::from_utf8_lossy(&result.stderr);
                if result.status.code() != Some(0)
                    || !diagnostic.contains("cinch-leak: live=0 immortal=0 leaked=0")
                {
                    let retained = directory.keep();
                    panic!("O{level}, poison={poisoned}, group={group}: {:?}; {diagnostic}; retained {}",
                           result.status.code(), retained.display());
                }
            }
        }
    }
}

/// Values outside the public comparison domain fail before executable generation.
#[test]
fn constant_time_ordering_rejects_out_of_domain_inputs() {
    for invalid in ["(-1)", "2147483648"] {
        let directory = tempfile::tempdir().unwrap();
        prepare(directory.path(), &format!(
            "-- | Reject a value outside the validated comparison domain.\nmodule Main where\n  import Crypto.ConstantTime\n  -- | An invalid input cannot inhabit this public operation's domain.\n  main :: Unit -> Bool\n  main unit = constant-less-or-equal {invalid} 0\n"
        ));
        assert!(
            common::drive_with_profile(
                cinch::driver::Mode::Check,
                cinch::check::StageMode::Refresh,
                directory.path(),
                "Main.cinch",
                None,
                &cinch::core::TargetProfile::native(),
            )
            .is_err(),
            "accepted invalid comparison input {invalid}"
        );
    }
}

/// Every public and private declaration has canonical documentation owned by this library.
#[test]
fn constant_time_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        LIBRARY,
        Some("Crypto.ConstantTime"),
        "Crypto.ConstantTime",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    assert!(declarations.len() >= 24);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing {}", declaration.name));
        assert!(!doc.legacy && !doc.markdown.trim().is_empty());
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Real program IR compiles to foreign objects with exact SIMD blocks and scalar fringes.
#[test]
fn constant_time_complete_consumers_compile_for_wasm_arm_and_big_endian() {
    let source = [WORD, LIBRARY, CONSUMER].join("\n");
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
                    "constant-time-complete.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("load i128") && ir.contains("load i8"));
        if endian == "big" {
            assert!(ir.contains("@llvm.bswap.i64") && ir.contains("@llvm.bswap.i128"));
        }
        let file = directory.join(format!("{arch}.ll"));
        std::fs::write(&file, ir).unwrap();
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
