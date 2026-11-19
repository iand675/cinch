//! Caller-buffer ASCII85 codecs, consumed-input progress and independent finite-capacity fixtures.
mod common;

use serde_json::Value;

/// Quote a fixture's hex text without interpreting it through the codec under test.
fn bytes(hex: &str) -> String {
    format!("(read-bytes {hex:?})")
}

/// Execute the actual source modules with ordinary live-space and separate poisoned-free budgets.
fn run(body: &str) {
    run_observed(body, "", None);
}

/// Observe optional raw kernels and allocation totals without weakening ordinary live-space bounds.
fn run_observed(body: &str, extra: &str, allocation_budget: Option<u64>) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Text.Ascii85",
            include_str!("../prelude/Text/Ascii85.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), format!("{HELPERS}\n{extra}\n  -- | Observe the complete runtime fixture result.\n  main :: Int\n  main = if ({body}) then 0 else 1\n")).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::build_executable_with_profile(directory.path(), "Main.cinch", &exe, &profile)
    .unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env(
                "CINCH_ALLOC_STATS",
                if allocation_budget.is_some() {
                    "1"
                } else {
                    "0"
                },
            )
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
        if let Some(budget) = allocation_budget {
            let text = String::from_utf8_lossy(&output.stderr);
            let line = text
                .lines()
                .find(|line| line.starts_with("cinch-alloc:"))
                .expect("allocation census");
            let total: u64 = line
                .split_whitespace()
                .find_map(|word| word.strip_prefix("bytes="))
                .unwrap()
                .parse()
                .unwrap();
            assert!(
                total <= budget,
                "unconsumed source is being recopied: {total} > {budget}"
            );
        }
    }
}

/// Reference finite destinations cover exact boundaries, malformed prefixes, finalization and ignored controls.
#[test]
fn ascii85_destinations_match_independent_progress_and_untouched_bytes() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let checks = fixture["ascii85_progress"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let kind = match row["kind"].as_str().unwrap() {
                "" => 0,
                "byte" => 1,
                "tail" => 2,
                other => panic!("{other}"),
            };
            format!(
                "(decode-case {} {} {} {} {} {kind} {} {})",
                bytes(row["encoded_hex"].as_str().unwrap()),
                row["capacity"],
                if row["finish"].as_bool().unwrap() {
                    "True"
                } else {
                    "False"
                },
                row["written"],
                row["consumed"],
                row["offset"].as_i64().unwrap().max(0),
                bytes(row["destination_hex"].as_str().unwrap())
            )
        })
        .collect::<Vec<_>>();
    // Keep source/runtime fixture batches bounded rather than constructing one large strict Boolean list.
    for batch in checks.chunks(128) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Fresh bytes, interior encoding intervals and retrying an unfinished suffix match independent wire data.
#[test]
fn ascii85_encoding_buffers_and_all_two_chunk_progress_splits_match_wire_data() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let checks = fixture["buffers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let encoded: String = row["ascii85"]
                .as_str()
                .unwrap()
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            format!(
                "(encode-case {} {} && splits {} {} 0)",
                bytes(row["hex"].as_str().unwrap()),
                bytes(&encoded),
                bytes(&encoded),
                bytes(row["hex"].as_str().unwrap())
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(5) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Every byte value crosses each scalar fallback position after an already completed SIMD group.
#[test]
fn ascii85_progress_checks_every_byte_and_lane_after_a_valid_prefix() {
    let mut table = Vec::new();
    for position in 0..5 {
        for byte in 0u16..256 {
            let mut word = 0u32;
            for lane in 0..5 {
                word = word.wrapping_mul(85).wrapping_add(
                    if lane == position && (33..=117).contains(&byte) {
                        u32::from(byte - 33)
                    } else {
                        0
                    },
                );
            }
            table.extend_from_slice(&word.to_be_bytes());
        }
    }
    let hex: String = table.iter().map(|byte| format!("{byte:02x}")).collect();
    run(&format!("positions {} 0", bytes(&hex)));
}

/// Capacity failures precede mutation and overlapping decoding reads its original source snapshot.
#[test]
fn ascii85_destination_errors_and_overlap_preserve_contracts() {
    run("short-encoding && invalid-offsets && overlap && overlap-expansion && overlap-error && source-offsets");
}

/// Small output intervals advance through a large source without repeatedly copying its unread suffix.
#[test]
fn ascii85_source_offsets_keep_small_destination_allocation_linear() {
    run_observed("large-offsets", "", Some(64 * 1024 * 1024));
}

/// Unsigned interval arithmetic handles empty, adjacent, high-address and overlapping ranges without wrapping ends.
#[test]
fn ascii85_overlap_kernel_handles_empty_adjacent_and_high_addresses() {
    let source = include_str!("../prelude/Text/Ascii85.cinch");
    let kernel = &source[source.find("  prim address-overlap ::").unwrap()..];
    run_observed("address-overlap 100 5 104 1 && address-overlap 104 1 100 5 && not (address-overlap 100 4 104 1) && not (address-overlap 100 0 100 8) && not (address-overlap 100 8 100 0) && not (address-overlap (-16) 8 (-8) 8) && address-overlap (-16) 9 (-8) 1 && not (address-overlap 9223372036854775807 1 (-9223372036854775808) 1) && address-overlap 9223372036854775807 2 (-9223372036854775808) 1", kernel, None);
}

/// New public carriers and private progress loops have canonical module-owned documentation.
#[test]
fn ascii85_buffer_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        include_str!("../prelude/Text/Ascii85.cinch"),
        Some("Text.Ascii85"),
        "Text.Ascii85",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for item in declarations {
        let doc = item
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing docs", item.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Runtime fixtures observe complete output, exact boundaries and sentinels independently of the codec.
const HELPERS: &str = r#"module Main where
  import Text.Ascii85
  import Text.Hex

  -- | Decode fixture transport bytes with an independent encoding.
  read-bytes :: String -> Bytes
  read-bytes text = case hex-decode text of Ok out -> out; Err error -> bytes-new 0
  -- | Compare every byte, including untouched destination sentinels.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Initialize all available destination bytes to a visible sentinel.
  fill :: Bytes -> Nat -> Unit
  fill out i = if i < bytes-len out then let written = bytes-set out i 85 in fill out (i + 1) else ()
  -- | Check a strict fixture batch without hiding any result.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons value rest -> value && all rest
  -- | Match the original invalid-byte or incomplete-final-quantum position.
  error-is :: Bytes -> Option Ascii85Error -> Nat -> Nat -> Bool
  error-is input error kind offset = case error of
    None -> kind == 0
    Some (InvalidAscii85Byte at byte) -> kind == 1 && at == offset && (if offset < bytes-len input then byte == bytes-get input offset else False)
    Some (TruncatedAscii85Quantum at) -> kind == 2 && at == offset
  -- | Observe completed counts and every byte in the caller's reserved interval.
  decode-case :: Bytes -> Nat -> Bool -> Nat -> Nat -> Nat -> Nat -> Bytes -> Bool
  decode-case input capacity finish written consumed kind offset expected =
    let out = bytes-new capacity in let initialized = fill out 0 in
    case ascii85-decode-into out 0 input finish of
      Err error -> False
      Ok (n, used, failure) -> n == written && used == consumed && error-is input failure kind offset && same out expected
  -- | Join test fragments through checked bulk copying, independently of decoding.
  join :: Bytes -> Bytes -> Bytes
  join a b = let out = bytes-new (bytes-len a + bytes-len b) in
    let first = bytes-copy-range out a 0 0 (bytes-len a) in
    let last = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in out
  -- | Snapshot an exact interval after checking its endpoints.
  part :: Bytes -> Nat -> Nat -> Bytes
  part input start end = if start <= end && end <= bytes-len input then
    let out = bytes-new (end - start) in let copied = bytes-copy-range out input 0 start (end - start) in out
    else bytes-new 0
  -- | Observe both written bytes and every unwritten sentinel around an encoding interval.
  encoding-bytes :: Bytes -> Bytes -> Nat -> Bool
  encoding-bytes out expected i = if i < bytes-len out then
    (if i > 0 && i <= bytes-len expected then
      (if i - 1 < bytes-len expected then bytes-get out i == bytes-get expected (i - 1) else False)
      else bytes-get out i == 85) && encoding-bytes out expected (i + 1)
    else True
  -- | Encode to fresh bytes and an interior caller interval with extra capacity.
  encode-case :: Bytes -> Bytes -> Bool
  encode-case input expected = let out = bytes-new (ascii85-max-encoded-length (bytes-len input) + 2) in
    let initialized = fill out 0 in case ascii85-encode-into out 1 input of
      Err error -> False
      Ok n -> n == bytes-len expected && same (ascii85-encode-bytes input) expected && encoding-bytes out expected 0
  -- | Retry the unconsumed non-final input suffix together with the next chunk.
  splits :: Bytes -> Bytes -> Nat -> Bool
  splits input expected i = if i <= bytes-len input then
    let first = part input 0 i in let last = part input i (bytes-len input) in
    (case ascii85-decode-prefix first False of
      (a, used, None) -> used <= bytes-len first &&
        (case ascii85-decode-prefix (join (part first used (bytes-len first)) last) True of
          (b, consumed, None) -> consumed == bytes-len first - used + bytes-len last && same (join a b) expected
          _ -> False)
      _ -> False) && splits input expected (i + 1)
    else True
  -- | Reject insufficient encoding capacity before changing either destination byte.
  short-encoding :: Bool
  short-encoding = let out = bytes-from-u8s [85,85] in
    case ascii85-encode-into out 0 (bytes-from-u8s [1]) of
      Err (Ascii85ShortDestination required available) -> required == 5 && available == 2 && same out (bytes-from-u8s [85,85])
      _ -> False
  -- | Permit empty intervals at the end and reject offsets beyond it without mutation.
  invalid-offsets :: Bool
  invalid-offsets = let out = bytes-from-u8s [85] in
    (case ascii85-encode-into out 1 (bytes-new 0) of Ok n -> n == 0; _ -> False) &&
    (case ascii85-decode-into out 1 (bytes-new 0) True of Ok (n, used, None) -> n == 0 && used == 0; _ -> False) &&
    (case ascii85-decode-into out 2 (bytes-new 0) True of Err (Ascii85InvalidDestinationOffset requested available) -> requested == 2 && available == 1; _ -> False) &&
    ascii85-buffer-error-message (Ascii85InvalidSourceOffset 2 1) == "ASCII85 source offset 2 exceeds byte length 1" &&
    ascii85-buffer-error-message (Ascii85InvalidDestinationOffset 2 1) == "ASCII85 destination offset 2 exceeds byte length 1" &&
    ascii85-buffer-error-message (Ascii85ShortDestination 5 2) == "ASCII85 output requires 5 bytes; available 2" &&
    same out (bytes-from-u8s [85])
  -- | Overwrite encoded bytes only after snapshotting the complete original input.
  overlap :: Bool
  overlap = let out = bytes-from-u8s [33,33,33,33,33,33,33,33,33,33] in
    case ascii85-decode-into out 1 out True of
      Ok (n, used, None) -> n == 8 && used == 10 && same out (bytes-from-u8s [33,0,0,0,0,0,0,0,0,33])
      _ -> False
  -- | Expanding shorthand must not overwrite symbols before they are consumed from the source snapshot.
  overlap-expansion :: Bool
  overlap-expansion = let out = bytes-from-u8s [122,33,33,33,33,33,33,33,33] in
    case ascii85-decode-into out 0 out True of
      Ok (n, used, None) -> n == 8 && used == 6 && same out (bytes-from-u8s [0,0,0,0,0,0,0,0,33])
      _ -> False
  -- | Preserve an invalid byte even when earlier decoded output overwrites its original address.
  overlap-error :: Bool
  overlap-error = let out = bytes-from-u8s [122,118,33,33,33,33,33,33,33] in
    case ascii85-decode-into out 0 out True of
      Ok (n, used, Some (InvalidAscii85Byte at byte)) -> n == 4 && used == 1 && at == 1 && byte == 118 && same out (bytes-from-u8s [0,0,0,0,33,33,33,33,33])
      _ -> False
  -- | Distinguish invalid source positions from destination failures and keep absolute error offsets.
  source-offsets :: Bool
  source-offsets = let out = bytes-new 4 in
    (case ascii85-decode-at out 0 (bytes-from-u8s [33]) 2 True of Err (Ascii85InvalidSourceOffset requested available) -> requested == 2 && available == 1; _ -> False) &&
    (case ascii85-decode-at out 0 (bytes-from-u8s [118,118,33,33,33,33,33,118]) 2 True of Ok (n, used, None) -> n == 4 && used == 5; _ -> False) &&
    (case ascii85-decode-at out 0 (bytes-from-u8s [118,118,118]) 2 True of Ok (n, used, Some (InvalidAscii85Byte at byte)) -> n == 0 && used == 0 && at == 2 && byte == 118; _ -> False)
  -- | Check the bytes returned through one reused four-byte output buffer.
  zero-output :: Bytes -> Nat -> Nat -> Bool
  zero-output out written i = if i < written && i < bytes-len out then bytes-get out i == 0 && zero-output out written (i + 1) else i == written
  -- | Advance only the source offset and retain no copied suffix between calls.
  drain-offsets :: Bytes -> Bytes -> Nat -> Nat -> Bool
  drain-offsets input out at total =
    case ascii85-decode-at out 0 input at True of
      Err error -> False
      Ok (written, used, None) -> zero-output out written 0 &&
        (if at + used == bytes-len input then total + written == 65536
          else used > 0 && drain-offsets input out (at + used) (total + written))
      _ -> False
  -- | A compressible corpus magnifies any repeated whole-source snapshot into quadratic allocation.
  large-offsets :: Bool
  large-offsets = let input = ascii85-encode-bytes (bytes-new 65536) in drain-offsets input (bytes-new 4) 0 0
  -- | Check a destination byte against an independent precomputed four-byte word table.
  lane-bytes :: Bytes -> Bytes -> Nat -> Nat -> Nat -> Bool
  lane-bytes table out position byte i = if i < bytes-len out then
    let valid = byte >= 33 && byte <= 117 in
    let written = if valid then (8 :: Nat) else if byte <= 32 then 7 else if byte == 122 && position == 0 then 11 else 4 in
    let at = position * 1024 + byte * 4 + (if i >= 4 then i - 4 else 0) in
    (if i >= written then bytes-get out i == 85
      else if valid && i >= 4 && at < bytes-len table then bytes-get out i == bytes-get table at
      else bytes-get out i == 0) && lane-bytes table out position byte (i + 1)
    else True
  -- | Exhaust byte classification and progress at one position after a complete word.
  values :: Bytes -> Bytes -> Nat -> Nat -> Bool
  values table input position byte = if byte < 256 && position < 5 && position + 5 < bytes-len input then
    let changed = bytes-set input (position + 5) byte in let out = bytes-new 12 in let initialized = fill out 0 in
    let valid = byte <= 117 || (byte == 122 && position == 0) in
    let written = if byte >= 33 && byte <= 117 then (8 :: Nat) else if byte <= 32 then 7 else if byte == 122 && position == 0 then 11 else 4 in
    let passed = (case ascii85-decode-into out 0 input True of Err error -> False; Ok (n, used, failure) -> n == written && used == (if valid then 10 else 5) && error-is input failure (if valid then 0 else 1) (position + 5) && lane-bytes table out position byte 0) in
    let restored = bytes-set input (position + 5) 33 in
    passed && values table input position (byte + 1)
    else True
  -- | Move the replacement through every SIMD digit position and scalar fallback boundary.
  positions :: Bytes -> Nat -> Bool
  positions table position = if position < 5 then
    values table (bytes-from-u8s [33,33,33,33,33,33,33,33,33,33]) position 0 && positions table (position + 1)
    else True
"#;
