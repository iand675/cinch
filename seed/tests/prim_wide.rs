//! Wide inline primitives preserve their declared ABI through direct calls and closure frames.
mod common;

use cinch::driver::Mode;

/// Compile a complete consumer and check ordinary ownership separately from poisoned frees.
fn run(body: &str) {
    run_with(body, false);
}

/// Exercise Rust's wide closures and tail groups, beyond the self-built backend's documented scope.
fn run_rust(body: &str) {
    run_with(body, true);
}

/// Select the requested frontend explicitly; an error never changes the compiler being tested.
fn run_with(body: &str, rust_only: bool) {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Main.cinch"), format!("{PRIMITIVES}\n  -- | Observe all bits of the computed wide result.\n  main :: Int\n  main = if ({body}) then 0 else 1\n")).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    compile_without_inlining(directory.path(), &exe, &profile, rust_only);
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Disable Rust unit copying; the selected self-built frontend emits these boundaries directly.
fn compile_without_inlining(
    directory: &std::path::Path,
    exe: &std::path::Path,
    profile: &cinch::core::TargetProfile,
    rust_only: bool,
) {
    let frontend = if rust_only {
        None
    } else {
        common::external_frontend::configured().unwrap()
    };
    if let Some(frontend) = frontend {
        let request = common::external_frontend::profile_request(
            Mode::Build,
            cinch::check::StageMode::Refresh,
            profile,
        )
        .unwrap();
        common::external_frontend::drive_profile(
            &frontend,
            Mode::Build,
            directory,
            "Main.cinch",
            Some(exe.to_owned()),
            &request,
        )
        .unwrap();
    } else {
        let (source, exe, profile) = (
            directory.join("Main.cinch"),
            exe.to_owned(),
            profile.clone(),
        );
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &source,
                    Some(&exe),
                    &profile,
                    &cinch::driver::DriverOptions {
                        no_inline: true,
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
    }
}

/// A packed SIMD-style return retains both complete machine words through further wide parameters.
#[test]
fn wide_inline_primitives_keep_unsigned_high_and_low_bits() {
    run("let value = pack 0xFEDCBA9876543210 0x89ABCDEF01234567 in eq-u64 (high value) 0xFEDCBA9876543210 && eq-u64 (low value) 0x89ABCDEF01234567 && eq-u128 (wide-xor value value) 0");
}

/// Signed wide values and neighboring narrow/float operands keep their independent ABI conversions.
#[test]
fn wide_inline_primitives_preserve_mixed_signed_narrow_and_float_arguments() {
    run("let value = mixed 255 (-18446744073709551616) (-1) (float-bits 0x4000000000000000) in eq-i64 (signed-high value) (-1) && eq-u64 (signed-low value) 255");
}

/// Rust escaping partial applications box captured wide arguments and release them after indirect calls.
/// The self-built backend deliberately rejects wide closures (design/wide-scalars.md, section 12).
#[test]
fn rust_wide_inline_primitives_flow_through_escaping_partial_applications() {
    run_rust("let next = make (pack 0xFEDCBA9876543210 0x89ABCDEF01234567) in let first = invoke next 171 in let last = invoke next 85 in eq-u64 (high first) 0xFEDCBA9876543210 && eq-u64 (low first) 0x89ABCDEF012345CC && eq-u64 (high last) 0xFEDCBA9876543210 && eq-u64 (low last) 0x89ABCDEF01234532");
}

/// Rust wide tail groups neither truncate primitive results nor grow the call stack.
/// The self-built backend currently plans tail groups only for word positions.
#[test]
fn rust_wide_inline_primitives_keep_long_tail_loops_bounded() {
    run_rust("let value = loop (pack 0xFEDCBA9876543210 0x89ABCDEF01234567) 200000 in eq-u64 (high value) 0xFEDCBA9876543210 && eq-u64 (low value) 0x89ABCDEF01234567");
}

/// Admitting wide results does not let visible stores escape the declared pure effect row.
#[test]
fn wide_inline_primitives_keep_memory_effect_containment() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Main.cinch"),
        r#"-- | A rejected pure primitive with a visible memory write.
module Main where
  -- | Deliberately invalid: an unsafe pointer does not authorize an undeclared store.
  prim invalid :: I64 -> U128 where
    unsafe llvm {
      %pointer = inttoptr i64 %0 to ptr
      store i128 0, ptr %pointer, align 1
      ret i128 0
    }
  -- | Checking must reject the unused primitive before any execution.
  main :: Int
  main = 0
"#,
    )
    .unwrap();
    let error = common::drive(Mode::Check, directory.path(), "Main.cinch", None).unwrap_err();
    assert!(error.contains("containment"), "{error}");
}

/// Rust object consumers learn wide positions from the actual ABI artifact.
/// This exercises the changed Rust publisher directly, independently of frontend selection.
#[test]
fn wide_inline_primitives_publish_wide_object_positions() {
    let directory = tempfile::tempdir().unwrap();
    let source = PRIMITIVES.replacen(
        "module Main where",
        "module Packed where\n  export pack, high, mixed, amend",
        1,
    );
    std::fs::write(directory.path().join("Packed.cinch"), source).unwrap();
    let object = directory.path().join("packed.o");
    let (source, output) = (directory.path().join("Packed.cinch"), object.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || cinch::driver::driver(Mode::EmitObject, &source, Some(&output)))
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    let manifest: serde_json::Value =
        serde_json::from_str(&common::object_metadata(&object))
            .unwrap();
    let units = manifest.as_object().unwrap()["units"].as_array().unwrap();
    for (name, params, wide_result) in [
        ("pack", r#"["R","R"]"#, true),
        ("high", r#"["W"]"#, false),
        ("mixed", r#"["R","W","R","R"]"#, true),
        ("amend", r#"["W","R"]"#, true),
    ] {
        let symbol = format!("Packed.{name}");
        let row = units
            .iter()
            .map(|unit| unit.as_object().unwrap())
            .find(|unit| unit["symbol"].as_str() == Some(symbol.as_str()))
            .unwrap();
        assert_eq!(row["params"].to_string(), params, "{symbol}");
        if wide_result {
            assert_eq!(row["result"].as_str(), Some("W"), "{symbol}");
        }
        assert_eq!(
            row["closure_entry"].as_str(),
            Some(format!("{symbol}.object-closure").as_str())
        );
    }
}

/// Wide LLVM scalar signatures remain legal on 32-bit and non-x86 object targets.
#[test]
fn wide_inline_primitives_compile_on_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
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
            profile.name = "wide-prim-powerpc64".into();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let source = format!("{PRIMITIVES}\n  -- | Keep wide parameter and result units reachable in the target object.\n  main :: Int\n  main = if eq-u64 (low (pack 0xFEDCBA9876543210 0x89ABCDEF01234567)) 0x89ABCDEF01234567 then 0 else 1\n");
        let program = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let checked =
                    cinch::driver::check_sources_profile(&source, "wide-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &checked)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, program).unwrap();
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Runtime helpers cover both wide signednesses and a word-uniform indirect call boundary.
const PRIMITIVES: &str = r#"-- | Wide scalar inline-LLVM ABI consumers.
module Main where
  import Data.Machine
  import Data.Wide
  -- | Pack a high and low machine word into one unsigned wide result.
  prim pack :: U64 -> U64 -> U128 where
    llvm {
      %high = zext i64 %0 to i128
      %low = zext i64 %1 to i128
      %upper = shl i128 %high, 64
      %result = or i128 %upper, %low
      ret i128 %result
    }
  -- | Observe the complete upper word of a wide argument.
  prim high :: U128 -> U64 where
    llvm {
      %shifted = lshr i128 %0, 64
      %result = trunc i128 %shifted to i64
      ret i64 %result
    }
  -- | Observe the complete lower word of a wide argument.
  prim low :: U128 -> U64 where
    llvm {
      %result = trunc i128 %0 to i64
      ret i64 %result
    }
  -- | Combine two wide arguments without changing their representation.
  prim wide-xor :: U128 -> U128 -> U128 where
    llvm {
      %result = xor i128 %0, %1
      ret i128 %result
    }
  -- | Construct an IEEE payload without relying on a floating-literal syntax.
  prim float-bits :: U64 -> F64 where
    llvm {
      %result = bitcast i64 %0 to double
      ret double %result
    }
  -- | Apply narrow signedness and floating payload bits to a signed wide value.
  prim mixed :: U8 -> I128 -> I8 -> F64 -> I128 where
    llvm {
      %unsigned = zext i8 %0 to i128
      %signed = sext i8 %2 to i128
      %bits = bitcast double %3 to i64
      %top = lshr i64 %bits, 62
      %floating = zext i64 %top to i128
      %a = add i128 %1, %unsigned
      %b = add i128 %a, %signed
      %result = add i128 %b, %floating
      ret i128 %result
    }
  -- | Preserve sign bits in the upper half of a signed wide argument.
  prim signed-high :: I128 -> I64 where
    llvm {
      %shifted = ashr i128 %0, 64
      %result = trunc i128 %shifted to i64
      ret i64 %result
    }
  -- | Observe the low bits of a signed wide argument as an unsigned machine word.
  prim signed-low :: I128 -> U64 where
    llvm {
      %result = trunc i128 %0 to i64
      ret i64 %result
    }
  -- | Change only the selected low byte while retaining the captured wide state.
  prim amend :: U128 -> U8 -> U128 where
    llvm {
      %byte = zext i8 %1 to i128
      %result = xor i128 %0, %byte
      ret i128 %result
    }
  -- | Return a partial application whose captured argument outlives this function.
  make :: U128 -> (U8 -> U128)
  make value = amend value
  -- | Invoke a callback across a boxed closure frame rather than a known direct call.
  invoke :: (U8 -> U128) -> U8 -> U128
  invoke callback byte = callback byte
  -- | Consume an even number of full-width xor steps in constant stack space.
  loop :: U128 -> Nat -> U128
  loop value count = if count == 0 then value else loop (wide-xor value 0xFEDCBA987654321089ABCDEF01234567) (count - 1)
"#;
