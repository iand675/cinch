//! Higher-order and real cross-target object coverage for boxed wide arrays.
//! Wide parallel payloads, packed-wide and direct-wide columnar zip remain open.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode};
use std::path::{Path, PathBuf};

/// Prepare the actual public library source and one independent wide client.
fn project() -> PathBuf {
    common::project("wide_array_followups", &[
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Main.cinch", include_str!("fixtures/wide-array-dynamic.cinch")),
    ])
}

/// Preserve actual dynamic call boundaries while using the ordinary driver
/// pipeline and explicit checked-RC profile, not a substituted foreign ABI.
fn compile(dir: &Path, out: &Path, mode: Mode, profile: TargetProfile, level: u8) {
    let (source, output) = (dir.join("Main.cinch"), out.to_path_buf());
    let result = std::thread::Builder::new().stack_size(64 * 1024 * 1024)
        .spawn(move || cinch::driver::driver_with_profile_opts(
            mode, StageMode::Refresh, &source, Some(&output), &profile,
            &DriverOptions {
                opt_level: Some(level),
                no_specialise_arguments: true,
                no_saturate_partial: true,
                ..DriverOptions::default()
            },
        )).unwrap().join().unwrap();
    assert_eq!(result.unwrap_or_else(|e| panic!("{e}; source: {}", dir.display())), 0);
}

/// Every higher-order output and retained alias survives O0/O2 with dynamic
/// partial calls retained and both memory profiles leak-free.
#[test]
fn wide_partial_calls_preserve_aliases() {
    let dir = project();
    for level in [0, 2] {
        let exe = dir.join(format!("wide-followups-o{level}"));
        compile(&dir, &exe, Mode::Build,
            TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() }, level);
        for (debug, heap) in [("0", "32M"), ("1", "256M")] {
            let output = std::process::Command::new(&exe)
                .env("CINCH_RC_DEBUG", debug).env("CINCH_HEAP_LIMIT", heap)
                .env("CINCH_STACK_SIZE", "8M").env("CINCH_LEAK_CHECK", "1")
                .output().unwrap();
            let err = String::from_utf8_lossy(&output.stderr);
            assert_eq!(output.status.code(), Some(0), "O{level}, debug={debug}: {err}; {}", dir.display());
            assert!(err.split_whitespace().any(|w| w == "leaked=0"), "{err}");
        }
    }
    common::cleanup(&dir);
}

/// Assemble actual Wasm32, AArch64 and synthetic big-endian PPC64 objects
/// from the same typed public client; this is codegen qualification, not a
/// claim of executing any cross-target object on the build host.
#[test]
fn boxed_wide_array_boundaries_assemble_for_wasm_arm_and_ppc() {
    let dir = project();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let mut profile = cinch::driver::target_profile(&DriverOptions {
            target: Some(triple.into()), ..DriverOptions::default()
        }).unwrap_or_else(|_| {
            assert_eq!(arch, "powerpc64", "only the explicit PPC qualification profile is synthetic");
            let mut profile = TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        profile.reclaim = true;
        profile.rc_debug = true;
        let ir = dir.join(format!("wide-{arch}.ll"));
        compile(&dir, &ir, Mode::EmitIr, profile, 2);
        let object = ir.with_extension("o");
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(&ir).arg("-o").arg(&object).output().unwrap();
        assert!(output.status.success(), "{triple}: {}; inputs: {}",
            String::from_utf8_lossy(&output.stderr), dir.display());
        assert!(std::fs::metadata(&object).unwrap().len() > 0);
    }
    common::cleanup(&dir);
}

/// A genuine wide parallel payload stays refused until a complete typed bridge
/// covers task storage, callbacks, arrays and ParBoth tuple result layouts.
#[test]
fn wide_parallel_payloads_remain_a_named_refusal() {
    let dir = common::project("wide_parallel_refusal", &[
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Main.cinch", include_str!("fixtures/wide-array-followups.cinch")),
    ]);
    let result = common::drive(Mode::EmitIr, &dir, "Main.cinch", Some(dir.join("refused.ll")));
    let message = result.expect_err("wide parallel payload must remain refused");
    assert!(message.contains("parallel task's payload") && message.contains("code generation does not lower yet"), "{message}");
    common::cleanup(&dir);
}
