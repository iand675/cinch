//! Native consumers compared with checked-in outputs from Go's standard library.
//! Regenerate fixtures with tooling/go_stdlib_vectors.go; Go is not needed to test.
mod common;

use serde_json::Value;
use std::fmt::Write;

fn vectors() -> Value {
    serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap()
}

fn field<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap()
}

fn bytes(hex: &str) -> String {
    let mut literal = String::from("(bytes-from-list ");
    for pair in hex.as_bytes().chunks_exact(2) {
        let value = u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
        write!(literal, "(Cons {value} ").unwrap();
    }
    literal.push_str("Nil");
    literal.extend(std::iter::repeat(')').take(hex.len() / 2 + 1));
    literal
}

fn run(name: &str, checks: Vec<String>) {
    run_with_profile(name, checks, None);
}

const HEX_CHECKS: &str = r#"  hex-is :: String -> Bytes -> Bool
  hex-is s expected = case hex-decode s of
    Err _ -> False
    Ok b -> bytes-equal b expected
"#;

const BASE32_CHECKS: &str = r#"  base32-is :: Base32Alphabet -> String -> Bytes -> Bool
  base32-is alphabet s expected = case base32-decode alphabet s of
    Err _ -> False
    Ok b -> bytes-equal b expected

  raw-is :: Base32Alphabet -> String -> Bytes -> Bool
  raw-is alphabet s expected = case base32-raw-decode alphabet s of
    Err _ -> False
    Ok b -> bytes-equal b expected
"#;

const FIXED_WIDTH_CHECKS: &str = r#"  read16-is :: ByteOrder -> Bytes -> Nat -> U16 -> Bool
  read16-is order b pos expected = case binary-read-u16 order b pos of
    Err _ -> False
    Ok x -> eq-u16 x expected

  read32-is :: ByteOrder -> Bytes -> Nat -> U32 -> Bool
  read32-is order b pos expected = case binary-read-u32 order b pos of
    Err _ -> False
    Ok x -> eq-u32 x expected

  read64-is :: ByteOrder -> Bytes -> Nat -> U64 -> Bool
  read64-is order b pos expected = case binary-read-u64 order b pos of
    Err _ -> False
    Ok x -> eq-u64 x expected

  put-ok :: Bool
  put-ok =
    let b = bytes-new 6 in
    case binary-put-u32 BigEndian b 1 0x12345678 of
      Err _ -> False
      Ok _ -> bytes-equal b (bytes-from-list (Cons 0 (Cons 18 (Cons 52 (Cons 86 (Cons 120 (Cons 0 Nil)))))))

  put-atomic :: Bool
  put-atomic =
    let b = bytes-from-list (Cons 1 (Cons 2 (Cons 3 Nil))) in
    case binary-put-u32 LittleEndian b 0 0xFFFFFFFF of
      Err UnexpectedBinaryEnd -> bytes-equal b (bytes-from-list (Cons 1 (Cons 2 (Cons 3 Nil))))
      _ -> False
"#;

const VARINT_CHECKS: &str = r#"  uvarint-is :: Bytes -> Nat -> U64 -> Nat -> Bool
  uvarint-is b pos expected consumed = case binary-read-uvarint b pos of
    Err _ -> False
    Ok (x, n) -> eq-u64 x expected && n == consumed

  varint-is :: Bytes -> I64 -> Nat -> Bool
  varint-is b expected consumed = case binary-read-varint b 0 of
    Err _ -> False
    Ok (x, n) -> eq-i64 x expected && n == consumed
"#;

/// Build a consumer under a selected instruction profile and inspect its IR.
fn run_with_profile(name: &str, checks: Vec<String>, profile: Option<cinch::core::TargetProfile>) {
    let dir = tempfile::tempdir().unwrap();
    for (module, source) in [
        ("Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Base32", include_str!("../prelude/Text/Base32.cinch")),
        ("Checksum", include_str!("../prelude/Text/Checksum.cinch")),
        ("Adler32", include_str!("../prelude/Text/Adler32.cinch")),
        ("Crc64", include_str!("../prelude/Text/Crc64.cinch")),
        ("Fnv", include_str!("../prelude/Text/Fnv.cinch")),
    ] {
        std::fs::write(dir.path().join(format!("Text.{module}.cinch")), source).unwrap();
    }
    std::fs::write(
        dir.path().join("Data.Bytes.Word.cinch"),
        include_str!("../prelude/Data/Bytes/Word.cinch"),
    )
    .unwrap();
    let (module, helpers, expected) = match name {
        "fixed-width" => ("Text.Binary", FIXED_WIDTH_CHECKS, "@llvm.bswap.i64"),
        "varints" => ("Text.Binary", VARINT_CHECKS, "lshr i64"),
        "hex" => ("Text.Hex", HEX_CHECKS, "shufflevector <16 x i8>"),
        "base32" => ("Text.Base32", BASE32_CHECKS, "icmp ugt <16 x i8>"),
        "base32-long-prefix-tails" => ("Text.Base32", BASE32_CHECKS, "icmp ult <16 x i8>"),
        "checksums" | "checksums-portable" => ("Text.Checksum", "", "load i128"),
        "fnv" => ("Text.Fnv", "", "mul i64"),
        _ => panic!("unknown consumer {name}"),
    };
    let mut source = format!("module Main where\n  import {module}\n");
    source.push_str(r#"  import Data.Bool
  import Data.Bytes
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String

  bytes-equal :: Bytes -> Bytes -> Bool
  bytes-equal a b = eq-i64 (bytes-compare a b) 0

"#);
    source.push_str(helpers);
    // Bound the depth of each expression without creating two globals for every vector.
    const CHUNK: usize = 8;
    for (batch, group) in checks.chunks(CHUNK).enumerate() {
        write!(source, "\n  step{batch} :: Int\n  step{batch} = ").unwrap();
        for (index, check) in group.iter().enumerate() {
            let failure = (batch * CHUNK + index) % 250 + 1;
            write!(source, "if not ({check}) then {failure} else ").unwrap();
        }
        writeln!(source, "step{}", batch + 1).unwrap();
    }
    let end = checks.len().div_ceil(CHUNK);
    writeln!(source, "\n  step{end} :: Int\n  step{end} = 0\n  main :: Int\n  main = step0").unwrap();
    std::fs::write(dir.path().join("Main.cinch"), source).unwrap();
    let executable = dir.path().join("consumer");
    match profile {
        Some(profile) => common::build_executable_with_profile(dir.path(), "Main.cinch", &executable, &profile).map(|_| ()),
        None => common::build_executable(dir.path(), "Main.cinch", "consumer").map(|_| ()),
    }
    .unwrap_or_else(|e| panic!("{name}: {e}"));
    let code = std::process::Command::new(&executable)
        .status()
        .unwrap()
        .code()
        .expect("consumer terminated normally");
    assert_eq!(
        code,
        0,
        "{name}: failed check (one-based) {code}: {:?}",
        checks.get((code as usize).wrapping_sub(1))
    );
    let ir = std::fs::read_to_string(executable.with_extension("ll")).unwrap();
    assert!(
        ir.contains(expected),
        "{name}: acceleration kernel missing from emitted LLVM"
    );
    if let Some((symbol, prim_callee)) = match name {
        "fnv" => Some(("Text.Fnv.fnv64-loop", "Prim.Machine.wrapping-mul-u64.llvm")),
        "varints" => Some((
            "Text.Binary.binary-read-uvarint",
            "Prim.Machine.trailing-zeros-u64.llvm",
        )),
        _ => None,
    } {
        let has_line = |prefix: &str, sym: &str| {
            let escaped: String = sym.bytes().map(|byte| format!("\\{byte:02X}")).collect();
            let names = [format!("@\"{sym}\"("), format!("@\"{escaped}\"(")];
            ir.lines()
                .any(|line| line.contains(prefix) && names.iter().any(|n| line.contains(n)))
        };
        assert!(
            ir.lines().any(|line| {
                let escaped: String = symbol.bytes().map(|byte| format!("\\{byte:02X}")).collect();
                line.starts_with("define ")
                    && (line.contains(&format!("@\"{symbol}\"("))
                        || line.contains(&format!("@\"{escaped}\"(")))
            }),
            "{symbol}: missing LLVM definition"
        );
        assert!(has_line("call ", symbol), "{symbol}: missing kernel call");
        assert!(
            ir.lines().any(|line| {
                let escaped: String = prim_callee
                    .bytes()
                    .map(|byte| format!("\\{byte:02X}"))
                    .collect();
                line.starts_with("define ")
                    && (line.contains(&format!("@\"{prim_callee}\"("))
                        || line.contains(&format!("@\"{escaped}\"(")))
            }),
            "{prim_callee}: missing primitive LLVM definition"
        );
        assert!(
            has_line("call ", prim_callee),
            "{prim_callee}: missing primitive call"
        );
        if name == "varints" {
            assert!(
                ir.contains("@llvm.cttz.i64"),
                "varints: missing SWAR trailing-zeros scan"
            );
        }
    }
    if name == "checksums-portable" {
        assert!(
            !ir.contains("asm \"crc32q"),
            "portable profile selected an x86 instruction"
        );
    }
}

/// Every declaration, including private kernels, has canonical documentation.
/// Public prose describes Cinch directly rather than another language's API.
#[test]
fn new_library_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Base32", include_str!("../prelude/Text/Base32.cinch")),
        (
            "Text.Checksum",
            include_str!("../prelude/Text/Checksum.cinch"),
        ),
        ("Text.Crc32", include_str!("../prelude/Text/Crc32.cinch")),
        ("Effect.Crc32", include_str!("../prelude/Effect/Crc32.cinch")),
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some(), "{name}: missing module documentation");
        assert!(
            parsed.diagnostics.is_empty(),
            "{name}: {:?}",
            parsed.diagnostics
        );
        assert!(parsed.orphans.is_empty(), "{name}: {:?}", parsed.orphans);
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("{}: missing documentation", declaration.id));
            assert!(
                !doc.legacy,
                "{}: use canonical documentation",
                declaration.id
            );
            let text = doc.markdown.to_lowercase();
            assert!(
                !text.contains("golang") && !text.contains("go's") && !text.contains("go "),
                "{}: documentation must describe Cinch",
                declaration.id
            );
        }
    }
}

/// Compile the real checksum, hex, base32 and FNV modules for 32-bit WebAssembly,
/// AArch64 (with and without ArmCrc32) and big-endian PowerPC. This catches
/// unavailable instructions, bad constraints and missing intrinsic declarations
/// without claiming execution on foreign hardware.
#[test]
fn inline_kernels_compile_for_portable_arm_crc_and_big_endian_targets() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Text/Binary.cinch"),
        include_str!("../prelude/Text/Adler32.cinch"),
        include_str!("../prelude/Text/Crc64.cinch"),
        include_str!("../prelude/Text/Checksum.cinch"),
        include_str!("../prelude/Text/Hex.cinch"),
        include_str!("../prelude/Text/Base32.cinch"),
        include_str!("../prelude/Text/Fnv.cinch"),
        r#"-- | Root public checksum, hex, base32 and FNV workers in portable foreign objects.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.String
  import Text.Adler32 as Adler
  import Text.Crc64 as Crc64
  import Text.Checksum as Checksum
  import Text.Hex
  import Text.Base32
  import Text.Fnv

  -- | Exercise Adler-32, slicing-by-4 CRC-32, CRC-64 tables, SIMD hex/base32 and FNV-64.
  main :: Int
  main =
    let input = bytes-new 32 in
    let a = Adler.adler32-sum input in
    let c-ieee = Checksum.crc32-sum input in
    let c-cast = Checksum.crc32-with Checksum.crc32-castagnoli input in
    let c-koop = Checksum.crc32-with Checksum.crc32-koopman input in
    let c-custom = Checksum.crc32-with 0x12345678 input in
    let t = Crc64.crc64-table Crc64.crc64-iso in
    let c64 = Crc64.crc64-table-sum t input in
    let c64-short = Crc64.crc64-sum Crc64.crc64-ecma input in
    let h = hex-encode input in
    let b32 = base32-encode Base32Standard input in
    let f = fnv64 input in
    let mix32 = bit-xor-u32 (bit-xor-u32 a c-ieee) (bit-xor-u32 (bit-xor-u32 c-cast c-koop) c-custom) in
    let mix64 = bit-xor-u64 (bit-xor-u64 c64 c64-short) f in
    if ne-u32 mix32 0 && ne-u64 mix64 0 && not (h == b32) then 0 else 1
"#,
    ]
    .join("\n");
    let dir = tempfile::tempdir().unwrap();
    for (name, triple, arch, endian, crc) in [
        (
            "wasm",
            "wasm32-unknown-unknown",
            "wasm32",
            "little",
            false,
        ),
        (
            "arm",
            "aarch64-unknown-linux-gnu",
            "aarch64",
            "little",
            false,
        ),
        (
            "arm-crc",
            "aarch64-unknown-linux-gnu",
            "aarch64",
            "little",
            true,
        ),
        (
            "big",
            "powerpc64-unknown-linux-gnu",
            "powerpc64",
            "big",
            false,
        ),
    ] {
        let mut profile = if arch == "wasm32" {
            cinch::driver::target_profile(&cinch::driver::DriverOptions {
                target: Some(triple.into()),
                ..Default::default()
            })
            .unwrap()
        } else {
            cinch::core::TargetProfile::native()
        };
        profile.triple = triple.into();
        profile.arch = arch.into();
        profile.endian = endian.into();
        if crc {
            profile.features.push("ArmCrc32".into());
        }
        let source_copy = source.clone();
        let program = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::check_sources_profile(&source_copy, "kernels.cinch", &profile)
                    .map(|program| (profile, program))
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        let ir = cinch::codegen::emit_for(&program.0, &program.1).unwrap();
        assert!(
            !ir.contains("crc32q"),
            "{name}: x86 instruction leaked into foreign target"
        );
        assert!(
            !ir.contains("crc32cx"),
            "{name}: inline asm crc32cx should not appear in pure slicing-by-4 checksum"
        );
        assert!(
            ir.contains("load i128"),
            "{name}: missing 128-bit Adler-32 block load"
        );
        assert!(
            ir.contains("switch "),
            "{name}: missing CRC-32 slicing table switch"
        );
        assert!(
            ir.contains("shufflevector <16 x i8>"),
            "{name}: missing SIMD hex/base32 splat"
        );
        assert!(
            ir.contains("mul i64"),
            "{name}: missing 64-bit multiply in FNV/Adler-32"
        );
        assert!(
            ir.contains("@llvm.bswap.i64"),
            "{name}: missing 64-bit network-order byte swap"
        );
        if endian == "big" {
            assert!(
                ir.contains("@llvm.bswap.i128"),
                "{name}: missing big-endian 128-bit swap"
            );
        }
        let path = dir.path().join(format!("{name}.ll"));
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
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn fixed_width_binary_matches_go_and_validates_writes() {
    let data = vectors();
    let mut checks = vec!["put-ok".into(), "put-atomic".into()];
    for v in data["words"].as_array().unwrap() {
        let x: u64 = field(v, "value").parse().unwrap();
        for (width, value) in [(16, x & 0xffff), (32, x & 0xffff_ffff), (64, x)] {
            for (prefix, order) in [("le", "LittleEndian"), ("be", "BigEndian")] {
                let encoded = field(v, &format!("{prefix}{width}"));
                checks.push(format!(
                    "bytes-equal (binary-u{width} {order} {value}) {}",
                    bytes(encoded)
                ));
                checks.push(format!(
                    "read{width}-is {order} {} 0 {value}",
                    bytes(encoded)
                ));
                checks.push(format!(
                    "read{width}-is {order} {} 1 {value}",
                    bytes(&format!("ff{encoded}ff"))
                ));
            }
        }
    }
    checks.extend([
        "case binary-read-u64 BigEndian (bytes-new 7) 0 of\n    Err UnexpectedBinaryEnd -> True\n    _ -> False".into(),
        "case binary-read-u16 LittleEndian (bytes-new 2) 3 of\n    Err UnexpectedBinaryEnd -> True\n    _ -> False".into(),
        "case binary-put-u16 LittleEndian (bytes-new 2) 3 1 of\n    Err UnexpectedBinaryEnd -> True\n    _ -> False".into(),
        "case binary-put-u64 LittleEndian (bytes-new 8) 0 0xFFFFFFFFFFFFFFFF of\n    Err _ -> False\n    Ok _ -> True".into(),
    ]);
    run("fixed-width", checks);
}

#[test]
fn varints_match_go_including_signed_extrema_and_malformed_input() {
    let data = vectors();
    let mut checks = Vec::new();
    for v in data["words"].as_array().unwrap() {
        let x = field(v, "value");
        let encoded = field(v, "uvarint");
        checks.push(format!(
            "bytes-equal (binary-uvarint {x}) {}",
            bytes(encoded)
        ));
        checks.push(format!(
            "uvarint-is {} 1 {x} {}",
            bytes(&format!("ff{encoded}ff")),
            encoded.len() / 2
        ));
    }
    for v in data["signed"].as_array().unwrap() {
        let x = field(v, "value");
        let encoded = field(v, "varint");
        checks.push(format!(
            "bytes-equal (binary-varint ({x})) {}",
            bytes(encoded)
        ));
        checks.push(format!(
            "varint-is {} ({x}) {}",
            bytes(encoded),
            encoded.len() / 2
        ));
    }
    for h in ["", "80", "808080808080808080"] {
        checks.push(format!(
            "case binary-read-uvarint {} 0 of\n    Err UnexpectedBinaryEnd -> True\n    _ -> False",
            bytes(h)
        ));
    }
    for h in [
        "80808080808080808002",
        "ffffffffffffffffff80",
        "8080808080808080808000",
    ] {
        checks.push(format!("case binary-read-uvarint {} 0 of\n    Err BinaryVarintOverflow -> True\n    _ -> False", bytes(h)));
    }
    checks.push(format!("uvarint-is {} 0 0 2", bytes("8000")));
    run("varints", checks);
}

#[test]
fn hexadecimal_matches_go_and_names_invalid_bytes() {
    let data = vectors();
    let mut checks = Vec::new();
    for v in data["buffers"].as_array().unwrap() {
        let h = field(v, "hex");
        checks.push(format!("hex-encode {} == {h:?}", bytes(h)));
        checks.push(format!("hex-is {h:?} {}", bytes(h)));
        checks.push(format!("hex-is {:?} {}", h.to_uppercase(), bytes(h)));
    }
    checks.extend([
        "hex-encoded-length 3 == 6 && hex-decoded-length 7 == 3".into(),
        "case hex-decode \"0\" of\n    Err OddHexLength -> True\n    _ -> False".into(),
        "case hex-decode \"00x\" of\n    Err (InvalidHexByte i v) -> i == 2 && v == 120\n    _ -> False".into(),
        "case hex-decode \"00é\" of\n    Err (InvalidHexByte i v) -> i == 2 && v == 195\n    _ -> False".into(),
        "case hex-decode \"0x\" of\n    Err (InvalidHexByte i v) -> i == 1 && v == 120\n    _ -> False".into(),
        "case hex-decode \" 0\" of\n    Err (InvalidHexByte i _) -> i == 0\n    _ -> False".into(),
    ]);
    for lane in 0..8 {
        let mut encoded = "0000000000000000".to_owned();
        encoded.replace_range(8 + lane..9 + lane, "!");
        checks.push(format!("case hex-decode {encoded:?} of\n    Err (InvalidHexByte i v) -> i == {} && v == 33\n    _ -> False", 8 + lane));
    }
    run("hex", checks);
}

#[test]
fn base32_matches_go_rfc4648_both_alphabets_and_raw_forms() {
    let data = vectors();
    let mut checks = Vec::new();
    for v in data["buffers"].as_array().unwrap() {
        let b = bytes(field(v, "hex"));
        for (key, alphabet) in [("base32", "Base32Standard"), ("base32_hex", "Base32Hex")] {
            let encoded = field(v, key);
            let raw = encoded.trim_end_matches('=');
            checks.push(format!("base32-encode {alphabet} {b} == {encoded:?}"));
            checks.push(format!("base32-is {alphabet} {encoded:?} {b}"));
            checks.push(format!("base32-raw-encode {alphabet} {b} == {raw:?}"));
            checks.push(format!("raw-is {alphabet} {raw:?} {b}"));
        }
    }
    checks.push(format!(
        "base32-is Base32Standard \"MZ======\" {}",
        bytes("66")
    ));
    checks.push(format!(
        "base32-is Base32Standard \"MZ\\r\\nXW6===\\n\" {}",
        bytes("666f6f")
    ));
    checks.push(format!(
        "base32-is Base32Standard \"MZXW6YTB\\r\\nOI======\\n\" {}",
        bytes("666f6f626172")
    ));
    for lane in 0..8 {
        let mut encoded = "MZXW6YTBMZXW6YTB".to_owned();
        encoded.replace_range(8 + lane..9 + lane, "!");
        checks.push(format!("case base32-decode Base32Standard {encoded:?} of\n    Err (InvalidBase32Character c) -> char-code c == 33\n    _ -> False"));
    }
    for s in [
        "M",
        "MZX",
        "MZXW6Y",
        "MY",
        "MY=====",
        "MY=======",
        "=Y======",
        "MY======A",
        "MY======MY======",
        "my======",
        "M!======",
    ] {
        checks.push(format!(
            "case base32-decode Base32Standard {s:?} of\n    Err _ -> True\n    Ok _ -> False"
        ));
    }
    for s in ["M", "MZX", "MZXW6Y", "MY======", "my", "M!"] {
        checks.push(format!(
            "case base32-raw-decode Base32Standard {s:?} of\n    Err _ -> True\n    Ok _ -> False"
        ));
    }
    run("base32", checks);
}

/// SIMD prefixes preserve scalar tail errors, symbol counts, CR/LF handling
/// and supplementary Unicode scalars without reparsing the decoded prefix.
#[test]
fn base32_scalar_tails_follow_long_simd_prefixes() {
    let mut checks = Vec::new();
    for (alphabet, quantum, tail) in [
        ("Base32Standard", "MZXW6YTB", "MY"),
        ("Base32Hex", "CPNMUOJ1", "CO"),
    ] {
        let prefix = format!("(string-repeat 256 {quantum:?})");
        let expected = "(string-to-bytes (string-concat (string-repeat 256 \"fooba\") \"f\"))";
        for suffix in [format!("{tail}======"), format!("\r\n{tail}===\r\n===\n")] {
            checks.push(format!("base32-is {alphabet} (string-concat {prefix} {suffix:?}) {expected}"));
        }
        checks.push(format!("raw-is {alphabet} (string-concat {prefix} {tail:?}) {expected}"));
        for scalar in ['é', '€', '🦀'] {
            for lane in 0..8 {
                let suffix = format!("{}{}", &quantum[..lane], scalar);
                checks.push(format!("case base32-decode {alphabet} (string-concat {prefix} {suffix:?}) of\n    Err (InvalidBase32Character c) -> char-code c == {}\n    _ -> False", scalar as u32));
            }
        }
        checks.push(format!("case base32-raw-decode {alphabet} (string-concat {prefix} \"M\") of\n    Err (InvalidBase32Length count) -> count == 2049\n    _ -> False"));
        checks.push(format!("case base32-decode {alphabet} (string-concat {prefix} \"====\") of\n    Err InvalidBase32Padding -> True\n    _ -> False"));
    }
    run("base32-long-prefix-tails", checks);
}

#[test]
fn checksums_match_go_and_update_over_every_split() {
    let data = vectors();
    let mut checks = Vec::new();
    for v in data["buffers"].as_array().unwrap() {
        let b = bytes(field(v, "hex"));
        checks.push(format!("eq-u32 (adler32-sum {b}) {}", field(v, "adler32")));
        checks.push(format!("eq-u32 (crc32-sum {b}) {}", field(v, "crc32_ieee")));
        for poly in ["ieee", "castagnoli", "koopman"] {
            checks.push(format!(
                "eq-u32 (crc32-with crc32-{poly} {b}) {}",
                field(v, &format!("crc32_{poly}"))
            ));
        }
        for poly in ["iso", "ecma"] {
            checks.push(format!(
                "eq-u64 (crc64-sum crc64-{poly} {b}) {}",
                field(v, &format!("crc64_{poly}"))
            ));
        }
    }
    // Every split, including empty chunks, of the published check string.
    let h = "313233343536373839";
    for split in 0..=9 {
        let a = bytes(&h[..split * 2]);
        let b = bytes(&h[split * 2..]);
        checks.push(format!(
            "eq-u32 (adler32-update (adler32-sum {a}) {b}) 152961502"
        ));
        for poly in ["ieee", "castagnoli", "koopman"] {
            checks.push(format!("eq-u32 (crc32-update crc32-{poly} (crc32-with crc32-{poly} {a}) {b}) (crc32-with crc32-{poly} {})", bytes(h)));
        }
        for poly in ["iso", "ecma"] {
            checks.push(format!("eq-u64 (crc64-update crc64-{poly} (crc64-sum crc64-{poly} {a}) {b}) (crc64-sum crc64-{poly} {})", bytes(h)));
        }
    }
    run("checksums", checks.clone());
    let mut portable = cinch::core::TargetProfile::native();
    portable.arch = "portable".into();
    run_with_profile("checksums-portable", checks, Some(portable));
}

#[test]
fn fnv_matches_go_and_updates_over_every_split() {
    let data = vectors();
    let mut checks = Vec::new();
    for v in data["buffers"].as_array().unwrap() {
        let b = bytes(field(v, "hex"));
        for (algorithm, width) in [("fnv32", 32), ("fnv32a", 32), ("fnv64", 64), ("fnv64a", 64)] {
            checks.push(format!(
                "eq-u{width} ({algorithm} {b}) {}",
                field(v, algorithm)
            ));
        }
    }
    let h = "666f6f626172";
    for split in 0..=6 {
        let a = bytes(&h[..split * 2]);
        let b = bytes(&h[split * 2..]);
        for (algorithm, width) in [("fnv32", 32), ("fnv32a", 32), ("fnv64", 64), ("fnv64a", 64)] {
            checks.push(format!(
                "eq-u{width} ({algorithm}-update ({algorithm} {a}) {b}) ({algorithm} {})",
                bytes(h)
            ));
        }
    }
    run("fnv", checks);
}
