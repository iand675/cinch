//! Word utilities against independent arithmetic fixtures and actual native consumers.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Turn a fixture value into runtime byte input, avoiding literal-only arithmetic checks.
fn input(value: &Value, width: u64) -> String {
    let hex: String = value
        .as_u64()
        .unwrap()
        .to_le_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let read = format!("read64 \"{hex}\"");
    if width == 64 {
        format!("({read})")
    } else {
        format!("(wrapping-u64-to-u{width} ({read}))")
    }
}

/// Render a reference carry or borrow as the language's explicit Boolean carrier.
fn flag(value: &Value) -> &'static str {
    if value.as_bool().unwrap() {
        "True"
    } else {
        "False"
    }
}

/// Compile source modules and validate their consumers with poisoned frees and a bounded heap.
fn run(checks: Vec<String>) {
    let dir = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Data.Word", include_str!("../prelude/Data/Word.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
    ] {
        std::fs::write(dir.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let mut source = HELPERS.to_string();
    // One runtime list avoids a quadratic collection of top-level test bindings
    // in the self-built frontend, while keeping every fixture operation live.
    source.push_str("\n  checks :: List Bool\n  checks = [\n");
    source.push_str(
        &checks
            .iter()
            .map(|check| format!("    ({check})"))
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    source.push_str("\n    ]\n  main :: Int\n  main = inspect checks 0\n");
    std::fs::write(dir.path().join("Main.cinch"), source).unwrap();
    let exe = dir.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        dir.path(),
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .unwrap();
    let output = std::process::Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
}

/// Bit counts, reversals and signed rotations agree at every width and modulo boundary.
#[test]
fn word_bit_operations_match_reference_widths_and_rotations() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = vec![format!("eq-u32 word-bits {}", usize::BITS)];
    for row in fixture["word_ops"]["scalars"].as_array().unwrap() {
        let w = row["width"].as_u64().unwrap();
        let value = input(&row["value"], w);
        for (name, field) in [
            ("bit-length", "length"),
            ("leading-zeros", "leading"),
            ("trailing-zeros", "trailing"),
            ("count-ones", "ones"),
        ] {
            checks.push(format!("eq-u32 ({name}-u{w} {value}) {}", row[field]));
        }
        checks.push(format!(
            "eq-u{w} (reverse-bits-u{w} {value}) {}",
            row["reverse"]
        ));
        if w != 8 {
            checks.push(format!(
                "eq-u{w} (reverse-bytes-u{w} {value}) {}",
                row["swap"]
            ));
        }
        for rotate in row["rotates"].as_array().unwrap() {
            checks.push(format!(
                "eq-u{w} (rotate-signed-u{w} {value} ({} :: I64)) {}",
                rotate["count"], rotate["value"]
            ));
        }
        if w == u64::from(usize::BITS) {
            let native = format!("(wrapping-u64-to-usize {})", input(&row["value"], 64));
            checks.push(format!(
                "eq-usize (reverse-bits-usize {native}) {}",
                row["reverse"]
            ));
            checks.push(format!(
                "eq-usize (reverse-bytes-usize {native}) {}",
                row["swap"]
            ));
            checks.push(format!(
                "eq-u32 (bit-length-usize {native}) {}",
                row["length"]
            ));
            for rotate in row["rotates"].as_array().unwrap() {
                checks.push(format!(
                    "eq-usize (rotate-signed-usize {native} ({} :: I64)) {}",
                    rotate["count"], rotate["value"]
                ));
            }
        }
    }
    checks.push(format!(
        "reverse-all (read-bytes \"{}\") 0",
        fixture["word_ops"]["reverse8_hex"].as_str().unwrap()
    ));
    run(checks);
}

/// Wide products and carry/borrow chains retain every output bit, including an incoming flag.
#[test]
fn word_wide_arithmetic_matches_reference_carry_borrow_and_products() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in fixture["word_ops"]["arithmetic"].as_array().unwrap() {
        let w = row["width"].as_u64().unwrap();
        let a = input(&row["a"], w);
        let b = input(&row["b"], w);
        let c = flag(&row["carry"]);
        checks.push(format!(
            "(case add-carry-u{w} {a} {b} {c} of (sum,carry) -> eq-u{w} sum {} && carry == {})",
            row["sum"],
            flag(&row["out_carry"])
        ));
        checks.push(format!("(case sub-borrow-u{w} {a} {b} {c} of (difference,borrow) -> eq-u{w} difference {} && borrow == {})",row["difference"],flag(&row["borrow"])));
        checks.push(format!(
            "(case mul-wide-u{w} {a} {b} of (high,low) -> eq-u{w} high {} && eq-u{w} low {})",
            row["high"], row["low"]
        ));
        if w == u64::from(usize::BITS) {
            let a = format!("(wrapping-u64-to-usize {})", input(&row["a"], 64));
            let b = format!("(wrapping-u64-to-usize {})", input(&row["b"], 64));
            checks.push(format!("(case add-carry-usize {a} {b} {c} of (sum,carry) -> eq-usize sum {} && carry == {})",row["sum"],flag(&row["out_carry"])));
            checks.push(format!("(case sub-borrow-usize {a} {b} {c} of (difference,borrow) -> eq-usize difference {} && borrow == {})",row["difference"],flag(&row["borrow"])));
            checks.push(format!("(case mul-wide-usize {a} {b} of (high,low) -> eq-usize high {} && eq-usize low {})",row["high"],row["low"]));
        }
    }
    run(checks);
}

/// Division rejects zero and quotient overflow; remainder accepts high words above the divisor.
#[test]
fn word_division_matches_reference_and_names_invalid_inputs() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in fixture["word_ops"]["divisions"].as_array().unwrap() {
        let w = row["width"].as_u64().unwrap();
        let high = input(&row["high"], w);
        let low = input(&row["low"], w);
        let divisor = input(&row["divisor"], w);
        let widths: Vec<String> = if w == u64::from(usize::BITS) {
            vec![format!("u{w}"), "usize".into()]
        } else {
            vec![format!("u{w}")]
        };
        for ty in widths {
            let (h, l, d) = if ty == "usize" {
                (
                    format!("(wrapping-u{w}-to-usize {high})"),
                    format!("(wrapping-u{w}-to-usize {low})"),
                    format!("(wrapping-u{w}-to-usize {divisor})"),
                )
            } else {
                (high.clone(), low.clone(), divisor.clone())
            };
            let condition = if row["failed"].as_bool().unwrap() {
                if row["divisor"].as_u64().unwrap() == 0 {
                    "Err WordDivideByZero -> True; other -> False".into()
                } else {
                    "Err WordQuotientOverflow -> True; other -> False".into()
                }
            } else {
                format!(
                    "Ok (q,r) -> eq-{ty} q {} && eq-{ty} r {}; Err error -> False",
                    row["quotient"], row["remainder"]
                )
            };
            checks.push(format!("(case div-wide-{ty} {h} {l} {d} of {condition})"));
            if ty != "usize" && !row["failed"].as_bool().unwrap() {
                checks.push(format!("(let h = {h} in let l = {l} in let d = {d} in case decide (d /= 0) of No witness -> False; Yes nonzero -> case decide (h < d) of No witness -> False; Yes fits -> case div-wide-proven-{ty} h l d of (q,r) -> eq-{ty} q {} && eq-{ty} r {})",row["quotient"],row["remainder"]));
            }
            let remainder = if row["divisor"].as_u64().unwrap() == 0 {
                "Err WordDivideByZero -> True; other -> False".into()
            } else {
                format!(
                    "Ok value -> eq-{ty} value {}; Err error -> False",
                    row["modulus"]
                )
            };
            checks.push(format!("(case rem-wide-{ty} {h} {l} {d} of {remainder})"));
        }
    }
    run(checks);
}

/// Proof comparisons use unsigned high bits and the ABI extension of narrow signed scalars.
#[test]
fn word_machine_proof_ordering_preserves_signedness() {
    let mut checks = Vec::new();
    for width in [8, 16, 32, 64] {
        let maximum = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let unsigned = input(&Value::from(maximum), width);
        checks.push(format!("order-u{width} {unsigned} 0 == 12"));
        checks.push(format!("order-u{width} 0 {unsigned} == 3"));
        checks.push(format!("order-u{width} {unsigned} {unsigned} == 10"));
        let signed = format!(
            "(wrapping-u64-to-i{width} {})",
            input(&Value::from(1u64 << (width - 1)), 64)
        );
        checks.push(format!("order-i{width} {signed} 0 == 3"));
        checks.push(format!("order-i{width} 0 {signed} == 12"));
        checks.push(format!("order-i{width} {signed} {signed} == 10"));
    }
    run(checks);
}

/// Public operations and private arithmetic kernels use canonical, language-owned documentation.
#[test]
fn word_operation_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Data/Word.cinch");
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Data.Word"), "Data.Word", None, false).unwrap();
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing docs", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// The private checked wrapper rejects a zero divisor before its trusted LLVM fragment is reachable.
#[test]
fn word_private_division_wrapper_requires_nonzero_evidence() {
    let source=format!("{}\n  export checked-zero\n  checked-zero :: U64\n  checked-zero = quotient64 0 0 0\n\nmodule Main where\n  import Data.Word\n  main :: Int\n  main = if eq-u64 checked-zero 0 then 0 else 1\n",include_str!("../prelude/Data/Word.cinch"));
    let result = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || common::source_frontend::check_sources(&source, "zero-word-divisor.cinch"))
        .unwrap()
        .join()
        .unwrap();
    let error = result.expect_err("a zero divisor reached a trusted raw kernel");
    assert!(
        error.contains("precondition") || error.contains("obligation"),
        "{error}"
    );
}

/// Cross-target compilation checks native word width and the portable LLVM kernels.
#[test]
fn word_kernels_compile_for_32_bit_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian, width, maximum) in [
        (
            "wasm32-unknown-unknown",
            "wasm32",
            "little",
            32,
            "4294967295",
        ),
        (
            "aarch64-unknown-linux-gnu",
            "aarch64",
            "little",
            64,
            "18446744073709551615",
        ),
        (
            "powerpc64-unknown-linux-gnu",
            "powerpc64",
            "big",
            64,
            "18446744073709551615",
        ),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..cinch::driver::DriverOptions::default()
        })
        .unwrap_or_else(|_| {
            // This target checks LLVM portability, rather than driver linking support.
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.name = "powerpc64-kernel-check".into();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile.word_size = width;
            profile
        });
        assert_eq!(profile.arch, arch);
        assert_eq!(profile.endian, endian);
        assert_eq!(profile.word_size, width);
        let source=format!("{}\nmodule Main where\n  import Data.Word\n  main :: Int\n  main = if eq-u32 word-bits {width} then (case add-carry-usize {maximum} 1 False of (value,carry) -> if eq-usize value 0 && carry then 0 else 2) else 3\n",include_str!("../prelude/Data/Word.cinch"));
        let (profile, program) = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::check_sources_profile(&source, "word-target.cinch", &profile)
                    .map(|program| (profile, program))
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        let ir = cinch::codegen::emit_for(&profile, &program).unwrap();
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, ir).unwrap();
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

/// Runtime fixture readers keep the observed arithmetic independent of constant folding.
const HELPERS: &str = r#"module Main where
  import Data.Word
  import Data.Bytes.Word
  import Data.Machine
  import Text.Hex

  -- | Decode independent fixture bytes, reserving empty input for an invalid fixture.
  read-bytes :: String -> Bytes
  read-bytes text = case hex-decode text of Ok b -> b; Err error -> bytes-new 0
  -- | Read an exact little-endian operand from a fresh fixture buffer.
  read64 :: String -> U64
  read64 text = let b = read-bytes text in
    if 8 <= bytes-len b then bytes-read-le64 b 0 else 0
  -- | Report the first unsuccessful runtime check with a bounded process exit code.
  inspect :: List Bool -> Nat -> Int
  inspect checks i = case checks of
    Nil -> 0
    Cons passed rest -> if passed then inspect rest (i + 1) else nat-to-int (1 + nat-rem i 250)
  -- | Exhaust all eight-bit inputs against an independent reversal table at runtime.
  reverse-all :: Bytes -> Nat -> Bool
  reverse-all table i = if i >= 256 then True else
    if i < bytes-len table then
      eq-u8 (reverse-bits-u8 (wrapping-i64-to-u8 (nat-to-i64 i))) (wrapping-i64-to-u8 (nat-to-i64 (bytes-get table i))) && reverse-all table (i + 1)
    else False

  -- | Observe all four ordered proof decisions for U8 runtime operands.
  order-u8 :: U8 -> U8 -> Nat
  order-u8 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for U16 runtime operands.
  order-u16 :: U16 -> U16 -> Nat
  order-u16 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for U32 runtime operands.
  order-u32 :: U32 -> U32 -> Nat
  order-u32 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for U64 runtime operands.
  order-u64 :: U64 -> U64 -> Nat
  order-u64 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for I8 runtime operands.
  order-i8 :: I8 -> I8 -> Nat
  order-i8 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for I16 runtime operands.
  order-i16 :: I16 -> I16 -> Nat
  order-i16 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for I32 runtime operands.
  order-i32 :: I32 -> I32 -> Nat
  order-i32 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)

  -- | Observe all four ordered proof decisions for I64 runtime operands.
  order-i64 :: I64 -> I64 -> Nat
  order-i64 a b =
    let less = case decide (a < b) of Yes witness -> True; No witness -> False in
    let less-equal = case decide (a <= b) of Yes witness -> True; No witness -> False in
    let greater = case decide (a > b) of Yes witness -> True; No witness -> False in
    let greater-equal = case decide (a >= b) of Yes witness -> True; No witness -> False in
    (if less then 1 else 0) + (if less-equal then 2 else 0) + (if greater then 4 else 0) + (if greater-equal then 8 else 0)
"#;
