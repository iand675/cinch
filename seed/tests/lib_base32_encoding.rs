//! Configurable Base32 byte alphabets, strict progress, immutable states and stream adapters.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(body: &str) {
    run_with_optimization(body, false);
}

/// Compile the ownership stress case explicitly with Rust's O2 driver and RC-debug capability.
/// Other cases continue to exercise the selected frontend without any fallback.
fn run_with_optimization(body: &str, optimized_rust: bool) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Base32", include_str!("../prelude/Text/Base32.cinch")),
        (
            "Effect.Base32",
            include_str!("../prelude/Effect/Base32.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), format!("{HELPERS}\n  -- | Observe the complete runtime fixture result.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if ({body}) then 0 else 1\n")).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized_rust {
        let (source, output, profile) = (
            directory.path().join("Main.cinch"),
            exe.clone(),
            profile.clone(),
        );
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &source,
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
        common::build_executable_with_profile(directory.path(), "Main.cinch", &exe, &profile)
        .unwrap();
    }
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

/// Read independent alphabet/padding and byte-level reference results without an SDK at test time.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/base32-encoding-vectors.json")).unwrap()
}

/// Preserve arbitrary alphabet and padding bytes, including non-UTF-8 values and every short fringe.
#[test]
fn base32_configurable_bytes_match_independent_alphabets_padding_and_lengths() {
    let f = fixture();
    let checks = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            format!(
                "valid-case {} {:?} {} {} {} {} {} {:?} {:?} {} {}",
                r["id"],
                r["alphabet"].as_str().unwrap(),
                if r["padding"] == -1 { "True" } else { "False" },
                r["padding"].as_i64().unwrap().max(0),
                r["size"],
                flag(r, "fill"),
                r["encoded_size"],
                r["encoded_hex"].as_str().unwrap(),
                r["encoded_sha256"].as_str().unwrap(),
                r["decoded_bound"],
                flag(r, "text_valid")
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(30) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Invalid bytes in every SIMD lane preserve exactly the preceding complete decoded quanta.
#[test]
fn base32_prefix_errors_match_independent_complete_quantum_progress() {
    let f = fixture();
    let checks = f["invalid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            format!(
                "invalid-case {} {:?} {} {} {:?} {:?} {} {}",
                r["id"],
                r["alphabet"].as_str().unwrap(),
                if r["padding"] == -1 { "True" } else { "False" },
                r["padding"].as_i64().unwrap().max(0),
                r["input"].as_str().unwrap(),
                r["output"].as_str().unwrap(),
                r["position"],
                r["byte"]
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(72) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Alphabet construction and padding changes reject precise invalid values without mutating inputs.
#[test]
fn base32_configuration_errors_match_independent_validation_categories() {
    let f = fixture();
    let checks = f["config"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            format!(
                "configuration-case {:?} {} {} {:?}",
                r["alphabet"].as_str().unwrap(),
                if r["padding"] == -1 { "True" } else { "False" },
                r["padding"].as_i64().unwrap().max(0),
                r["kind"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Every split of representative fringes preserves encoder and decoder state without retained input aliases.
#[test]
fn base32_incremental_states_match_every_small_input_and_output_split() {
    let f = fixture();
    let sizes = [0, 1, 4, 5, 6, 9, 10, 17, 33, 65];
    let checks = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["fill"] == false && sizes.contains(&r["size"].as_u64().unwrap()))
        .map(|r| {
            format!(
                "split-case {} {:?} {} {} {}",
                r["id"],
                r["alphabet"].as_str().unwrap(),
                if r["padding"] == -1 { "True" } else { "False" },
                r["padding"].as_i64().unwrap().max(0),
                r["size"]
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(30) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Fresh buffer, append and finite-destination APIs preserve prefixes, capacity checks and overlap safety.
#[test]
fn base32_buffers_and_persistent_branches_remain_independent() {
    run("buffer-cases && aliases && strict-cases");
}

/// Bounded stream adapters flush tails, retry short writes and delay sticky errors until output drains.
#[test]
fn base32_stream_adapters_preserve_tails_surplus_and_failures() {
    run("stream-cases ()");
}

/// One-byte decoder reads remain bounded under ordinary and poisoned-free memory validation.
#[test]
fn base32_small_stream_reads_release_buffers_and_preserve_every_byte() {
    run("small-reads ()");
}

/// An early CR/LF sends a large fixed-text suffix through bounded scalar collection and decoding.
#[test]
fn base32_wrapped_fixed_text_decoding_uses_bounded_stack() {
    run("let raw = corpus 65536 False in let encoded = base32-encode Base32Standard raw in case base32-decode Base32Standard (string-concat \"\\r\\n\" encoded) of Ok out -> same out raw; Err error -> False");
}

/// Optimized private table loads retain temporary configurations and input owners through the final use.
#[test]
fn rust_base32_optimized_custom_table_and_buffer_owners_survive_tail_loops() {
    run_with_optimization("owner-loop 100 && aliases && buffer-cases", true);
}

/// Every new declaration has canonical module-owned documentation.
#[test]
fn base32_all_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Base32", include_str!("../prelude/Text/Base32.cinch")),
        (
            "Effect.Base32",
            include_str!("../prelude/Effect/Base32.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some() && parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}

/// Actual numeric SIMD and private byte-gather kernels compile on 32-bit and non-x86 targets.
/// This explicitly tests the Rust emitter without claiming execution of foreign objects.
#[test]
fn base32_custom_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let source = include_str!("../prelude/Text/Base32.cinch");
    let kernels = &source[source.find("  prim encode-value5 ::").unwrap()..];
    let program = format!("-- | Foreign object fixture for the actual custom-alphabet kernels.\nmodule Main where\n  import Data.Machine\n  import Data.Bytes\n  -- | Exact five-bit splitting, validation and byte lookup kernels under test.\n{kernels}\n  -- | Keep the allocated complete table alive through its last private load.\n  hold :: Bytes -> U64 -> U64\n  hold bytes value = case bytes-len bytes of _ -> value\n  -- | Keep every kernel reachable while producing an observable result.\n  main :: Int\n  main = let table = bytes-new 256 in let indices = encode-value5 0 in let values = hold table (lookup-config8 (bytes-data table) indices) in if eq-u64 (decode-value8 values) 0 then 0 else 1\n");
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
            let mut p = cinch::core::TargetProfile::native();
            p.triple = triple.into();
            p.arch = arch.into();
            p.endian = endian.into();
            p
        });
        let input = program.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let checked =
                    cinch::driver::check_sources_profile(&input, "base32-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &checked)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<8 x i8>"));
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

/// Render independent fixture booleans with explicit language constructors.
fn flag(row: &Value, name: &str) -> &'static str {
    if row[name].as_bool().unwrap() {
        "True"
    } else {
        "False"
    }
}

/// Compact runtime consumers compare full bytes, independent digests, progress and persistent continuations.
const HELPERS: &str = r#"-- | Configurable Base32 fixture consumers with explicit stream capabilities.
module Main where
  import Text.Base32
  import Effect.Base32
  import Effect.Stream
  import Text.Hex
  import Data.Machine
  import Data.Bytes
  import Data.Option
  import Data.Result
  import Data.Nat
  import Data.String
  import Data.List
  import Data.Bool

  -- | Parse independent fixture bytes without using the codec under test.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err error -> bytes-new 0
  -- | Compare complete byte buffers including exact lengths.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Conjoin a bounded batch of runtime results.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Copy a checked half-open interval for independent update calls.
  part :: Bytes -> Nat -> Nat -> Bytes
  part bytes start end = if start <= end && end <= bytes-len bytes then
    let out = bytes-new (end - start) in let copied = bytes-copy-range out bytes 0 start (end - start) in out
    else bytes-new 0
  -- | Join two independent observations without retaining caller aliases.
  joined :: Bytes -> Bytes -> Bytes
  joined a b = let n = bytes-len a in let out = bytes-new (n + bytes-len b) in
    let first = bytes-copy-range out a 0 0 n in let last = bytes-copy-range out b n 0 (bytes-len b) in out
  -- | Keep compact corpus construction in the executable rather than the compile-time evaluator.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %n = or i32 %0, 0
      ret i32 %n
    }
  -- | Prepare deterministic input independently of all encode/decode functions.
  corpus :: Nat -> Bool -> Bytes
  corpus n fill = if n <= 65536 then
    let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) in
    let out = bytes-new size in let filled = fill-bytes out fill 0 in out
    else bytes-new 0
  -- | Fill with all ones or an independently defined full-byte permutation.
  fill-bytes :: Bytes -> Bool -> Nat -> Unit
  fill-bytes out fill i = if i < bytes-len out then
    let written = bytes-set out i (if fill then 255 else nat-rem (i * 37 + 11) 256) in fill-bytes out fill (i + 1)
    else ()
  -- | Select the actual fixed SIMD modes or construct an independent custom byte table.
  configured :: Nat -> String -> Bool -> Nat -> Result Base32ConfigError Base32Encoding
  configured id alphabet raw padding =
    let initial = if id < 2 then Ok base32-standard-encoding else if id < 4 then Ok base32-hex-encoding else base32-encoding (input alphabet) in
    case initial of
      Err error -> Err error
      Ok encoding -> base32-with-padding encoding (if raw then None else Some padding)
  -- | Check independently recorded lengths, byte contents, digest, text validity and complete round trips.
  valid-case :: Nat -> String -> Bool -> Nat -> Nat -> Bool -> Nat -> String -> String -> Nat -> Bool -> Bool
  valid-case id alphabet raw padding n fill size expected hash bound text-valid = case configured id alphabet raw padding of
    Err error -> False
    Ok encoding ->
      let bytes = corpus n fill in let out = base32-encode-bytes encoding bytes in
      bytes-len out == size && base32-encoded-length encoding n == size && base32-decoded-length encoding size == bound
        && same (sha256-bytes out) (input hash) && (if n <= 257 then same out (input expected) else True)
        && same (base32-encoding-alphabet encoding) (input alphabet)
        && (case base32-encoding-padding encoding of None -> raw; Some value -> not raw && value == padding)
        && (case base32-encode-text encoding bytes of None -> not text-valid; Some text -> text-valid && same (string-to-bytes text) out)
        && (case base32-decode-bytes encoding out of Err error -> False; Ok decoded -> same decoded bytes)
        && (case base32-decode-prefix encoding out of (decoded, None) -> same decoded bytes; other -> False)
        && (if id < 4 then
          let fixed-alphabet = if id < 2 then Base32Standard else Base32Hex in
          let legacy = if raw then base32-raw-encode fixed-alphabet bytes else base32-encode fixed-alphabet bytes in
          same (string-to-bytes legacy) out && (case (if raw then base32-raw-decode fixed-alphabet legacy else base32-decode fixed-alphabet legacy) of
            Ok decoded -> same decoded bytes
            Err error -> False)
          else True)
  -- | Preserve independently decoded prefix bytes and the exact bad byte in each complete block.
  invalid-case :: Nat -> String -> Bool -> Nat -> String -> String -> Nat -> Nat -> Bool
  invalid-case id alphabet raw padding encoded expected position byte = case configured id alphabet raw padding of
    Err error -> False
    Ok encoding -> case base32-decode-prefix encoding (input encoded) of
      (out, Some (InvalidBase32Byte at value)) -> same out (input expected) && at == position && value == byte
        && base32-input-error-offset (InvalidBase32Byte at value) == position
        && (case base32-decode-bytes encoding (input encoded) of Err error -> True; Ok decoded -> False)
      other -> False
  -- | Match independent configuration failure categories while accepting every valid byte padding policy.
  configuration-case :: String -> Bool -> Nat -> String -> Bool
  configuration-case alphabet raw padding kind =
    let original = input alphabet in let saved = input alphabet in
    let result = case base32-encoding original of Err error -> Err error; Ok encoding -> base32-with-padding encoding (if raw then None else Some padding) in
    same original saved && (case result of
      Ok encoding -> kind == ""
      Err error -> case error of
        Base32AlphabetLength n -> kind == "length" && n == bytes-len original
        Base32AlphabetNewline at -> kind == "newline"
        Base32AlphabetDuplicate at -> kind == "duplicate"
        Base32PaddingValue n -> kind == "padding" && n == padding
        Base32PaddingCollision n -> kind == "collision" && n == padding)
  -- | Compare all two-feed encoding splits with the independently checked complete output.
  encoder-splits :: Base32Encoding -> Bytes -> Bytes -> Nat -> Bool
  encoder-splits encoding raw expected at = if at <= bytes-len raw then
    case base32-encoder-feed (base32-encoder encoding) (part raw 0 at) of
      (first, state) -> case base32-encoder-feed state (part raw at (bytes-len raw)) of
        (last, next) -> same (joined first (joined last (base32-encoder-finish next))) expected && encoder-splits encoding raw expected (at + 1)
    else True
  -- | Compare all encoded-input splits, including every padding boundary, with the original bytes.
  decoder-splits :: Base32Encoding -> Bytes -> Bytes -> Nat -> Bool
  decoder-splits encoding encoded expected at = if at <= bytes-len encoded then
    case base32-decoder-feed (base32-decoder encoding) (part encoded 0 at) of
      (first, state) -> case base32-decoder-feed state (part encoded at (bytes-len encoded)) of
        (last, next) -> case base32-decoder-finish next of
          (tail, None) -> same (joined first (joined last tail)) expected && decoder-splits encoding encoded expected (at + 1)
          other -> False
    else True
  -- | Test persistent two-feed operation at every small input and output position.
  split-case :: Nat -> String -> Bool -> Nat -> Nat -> Bool
  split-case id alphabet raw padding n = case configured id alphabet raw padding of
    Err error -> False
    Ok encoding -> let bytes = corpus n False in let encoded = base32-encode-bytes encoding bytes in
      encoder-splits encoding bytes encoded 0 && decoder-splits encoding encoded bytes 0

  -- | Check fresh append storage, exact padded capacity, untouched margins and an overlapping decode.
  buffer-cases :: Bool
  buffer-cases =
    let encoding = base32-standard-encoding in let raw = string-to-bytes "foobar" in
    let encoded = string-to-bytes "MZXW6YTBOI======" in let prefix = input "aabbcc" in
    let destination = bytes-new 20 in
    same (base32-encode-append encoding prefix raw) (joined prefix encoded)
      && (case base32-decode-append encoding prefix encoded of (out, None) -> same out (joined prefix raw); other -> False)
      && (case base32-encode-into encoding destination 2 raw of
        Ok n -> n == 16 && same (part destination 2 18) encoded && bytes-get destination 0 == 0 && bytes-get destination 19 == 0
        Err error -> False)
      && (let exact = bytes-new 6 in case base32-decode-into encoding exact 0 encoded of
        (n, None) -> n == 6 && same exact raw
        other -> False)
      && (let overlap = string-to-bytes "MZXW6YTBOI======" in case base32-decode-into encoding overlap 0 overlap of
        (n, None) -> n == 6 && same (part overlap 0 6) raw
        other -> False)
      && (let short = string-to-bytes "zzzz" in case base32-decode-into encoding short 0 encoded of
        (0, Some (Base32ShortDestination required available)) -> required == 6 && available == 4 && same short (string-to-bytes "zzzz")
        other -> False)
      && (let alias = string-to-bytes "foobar" in case base32-encode-into encoding alias 0 alias of
        Err (Base32ShortDestination required available) -> required == 16 && available == 6 && same alias raw
        other -> False)
      && (case base32-decode-append encoding prefix (string-to-bytes "MZXW6YTB!") of
        (out, Some (InvalidBase32Byte at byte)) -> same out (joined prefix (string-to-bytes "fooba")) && at == 8 && byte == 33
        other -> False)

  -- | Mutate a caller-owned fixture byte after its previous value has been observed.
  mutate :: Bytes -> Nat -> Unit
  mutate bytes value = if 0 < bytes-len bytes && value <= 255 then bytes-set bytes 0 value else ()

  -- | Alphabet tables, pending input and retained decoder branches have no mutable caller aliases.
  aliases :: Bool
  aliases =
    let alphabet = string-to-bytes "abcdefghijklmnopqrstuvwxyz234567" in
    case base32-encoding alphabet of
      Err error -> False
      Ok encoding ->
        let exposed = base32-encoding-alphabet encoding in
        let changed-a = mutate alphabet 33 in let changed-b = mutate exposed 63 in
        same (base32-encode-bytes encoding (string-to-bytes "foobar")) (string-to-bytes "mzxw6ytboi======")
          && (let raw = string-to-bytes "f" in case base32-encoder-feed (base32-encoder encoding) raw of
            (empty, first) -> let changed = mutate raw 122 in
              case base32-encoder-feed first (string-to-bytes "oobar") of
                (out, next) -> bytes-len empty == 0 && same (joined out (base32-encoder-finish next)) (string-to-bytes "mzxw6ytboi======")
                  && same (base32-encoder-finish first) (string-to-bytes "my======"))
          && (let raw = string-to-bytes "m" in case base32-decoder-feed (base32-decoder encoding) raw of
            (empty, first) -> let changed = mutate raw 33 in
              case base32-decoder-feed first (string-to-bytes "y======") of
                (out, next) -> same out (string-to-bytes "f") && bytes-len empty == 0
                  && (case base32-decoder-finish next of (tail, None) -> bytes-len tail == 0; other -> False)
                  && (case base32-decoder-feed first (string-to-bytes "4======") of
                    (branched, last) -> same branched (string-to-bytes "g") && (case base32-decoder-error last of None -> True; Some error -> False)))

  -- | Reject unfinished raw groups and padded suffixes, preserving complete output and original byte offsets.
  strict-cases :: Bool
  strict-cases =
    let encoding = base32-standard-encoding in
    (case base32-with-padding encoding None of
      Err error -> False
      Ok raw -> all [
        (case base32-decode-prefix raw (string-to-bytes "M") of (out, Some (TruncatedBase32Quantum at count)) -> bytes-len out == 0 && at == 0 && count == 1; other -> False),
        (case base32-decode-prefix raw (string-to-bytes "MZX") of (out, Some (TruncatedBase32Quantum at count)) -> bytes-len out == 0 && at == 0 && count == 3; other -> False),
        (case base32-decode-prefix raw (string-to-bytes "MZXW6Y") of (out, Some (TruncatedBase32Quantum at count)) -> bytes-len out == 0 && at == 0 && count == 6; other -> False),
        (case base32-decode-prefix raw (string-to-bytes "MZ") of (out, None) -> same out (string-to-bytes "f"); other -> False)])
      && (case base32-decode-prefix encoding (string-to-bytes "MY=====") of
        (out, Some (TruncatedBase32Quantum at count)) -> bytes-len out == 0 && at == 0 && count == 7
        other -> False)
      && (case base32-decode-prefix encoding (string-to-bytes "MZXW6YTBM") of
        (out, Some (TruncatedBase32Quantum at count)) -> same out (string-to-bytes "fooba") && at == 8 && count == 1
        other -> False)
      && (case base32-decode-prefix encoding (string-to-bytes "MY======A") of
        (out, Some (InvalidBase32Byte at byte)) -> same out (string-to-bytes "f") && at == 8 && byte == 65
        other -> False)
      && (case base32-decode-prefix encoding (string-to-bytes "M\r\nY==\nX===") of
        (out, Some (InvalidBase32Byte at byte)) -> bytes-len out == 0 && at == 7 && byte == 88
        other -> False)
      && (case base32-decode-prefix encoding (string-to-bytes "MZ\r\n======\n") of
        (out, None) -> same out (string-to-bytes "f")
        other -> False)
      && (case base32-decoder-feed (base32-decoder encoding) (string-to-bytes "MZXW6YTB!") of
        (out, failed) -> same out (string-to-bytes "fooba") && (case base32-decoder-feed failed (string-to-bytes "MZXW6YTB") of
          (empty, still) -> bytes-len empty == 0 && (case base32-decoder-error still of Some (InvalidBase32Byte at byte) -> at == 8 && byte == 33; other -> False)))

  -- | Consume one bounded encoded stream and compare all returned bytes with its original input.
  stream-case :: forall r. using (io :: Io r) => Base32Encoding -> Bytes ->{IO r} Bool
  stream-case encoding raw = case base32-write (base32-writer encoding writer-buffer) (part raw 0 1) of
    (Ok first, state) -> case base32-write state (part raw 1 (bytes-len raw)) of
      (Ok last, next) -> case base32-finish next of
        (Ok unit, sink) -> first == 1 && last + 1 == bytes-len raw
          && (case writer-bytes sink of Some out -> same out (base32-encode-bytes encoding raw); None -> False)
          && (case stream-read-all 7 (base32-reader encoding (reader-from-bytes (base32-encode-bytes encoding raw))) of
            (Ok out, advanced) -> same out raw
            other -> False)
        other -> False
      other -> False
    other -> False

  -- | Accept one short output prefix, then preserve an inspectable memory sink for retries.
  short-first :: forall r. using (io :: Io r) => ByteWriter r
  short-first = stream-writer (\bytes -> stream-write writer-buffer (part bytes 0 (if bytes-len bytes < 3 then bytes-len bytes else 3)))

  -- | Return a sentinel failure with a memory sink advanced by an independently visible marker.
  failing-writer :: forall r. using (io :: Io r) => ByteWriter r
  failing-writer = stream-writer (\bytes -> case stream-write writer-buffer (string-to-bytes "!") of
    (result, next) -> (Err (StreamEncodingError "sink" 9 "sentinel"), next))

  -- | Short writes retry exact encoded suffixes; sticky failures never touch their advanced sink again.
  writer-cases :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  writer-cases unit =
    (case base32-write (base32-writer base32-standard-encoding short-first) (string-to-bytes "foobar") of
      (Ok n, next) -> n == 6 && (case base32-finish next of
        (Ok unit, sink) -> (case writer-bytes sink of Some out -> same out (string-to-bytes "MZXW6YTBOI======"); None -> False)
        other -> False)
      other -> False)
      && (case base32-write (base32-writer base32-standard-encoding failing-writer) (string-to-bytes "fooba") of
        (Err (StreamEncodingError name at message), next) -> name == "sink" && at == 9 && message == "sentinel"
          && (case base32-write next (string-to-bytes "more") of
            (Err again, failed) -> (case base32-finish failed of
              (Err last, sink) -> (case writer-bytes sink of Some out -> same out (string-to-bytes "!"); None -> False)
              other -> False)
            other -> False)
        other -> False)

  -- | Emit one successful chunk, counting source reads before a terminal independently tagged failure.
  counted-source :: forall r. using (io :: Io r) => Bytes -> Bytes -> ByteReader r
  counted-source counter chunk = stream-reader (\n ->
    let counted = if 0 < bytes-len counter then let old = bytes-get counter 0 in bytes-set counter 0 (if old < 255 then old + 1 else old) else () in
    (Ok chunk, counted-failure counter))

  -- | Fail with a preserved advanced source after incrementing the observable source-read count.
  counted-failure :: forall r. using (io :: Io r) => Bytes -> ByteReader r
  counted-failure counter = stream-reader (\n ->
    let counted = if 0 < bytes-len counter then let old = bytes-get counter 0 in bytes-set counter 0 (if old < 255 then old + 1 else old) else () in
    (Err (StreamEncodingError "source" 17 "sentinel"), reader-from-bytes (bytes-new 0)))

  -- | Drain completed bytes before a positioned sticky encoding error without another source read.
  delayed-error :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  delayed-error unit = let counter = bytes-new 1 in
    case stream-read (base32-reader base32-standard-encoding (counted-source counter (string-to-bytes "MY======A"))) 0 of
      (Ok empty, initial) -> bytes-len empty == 0 && bytes-get counter 0 == 0 &&
        (case stream-read initial 1 of
          (Ok out, next) -> same out (string-to-bytes "f") && bytes-get counter 0 == 1 &&
            (case stream-read next 1 of
              (Err (StreamEncodingError name at message), failed) -> name == "base32" && at == 8 && bytes-get counter 0 == 1 &&
                (case stream-read failed 1 of (Err again, final) -> bytes-get counter 0 == 1; other -> False)
              other -> False)
          other -> False)
      other -> False

  -- | Decoder EOF errors are delayed until preceding complete output has been returned.
  truncated-stream :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  truncated-stream unit = case stream-read (base32-reader base32-standard-encoding (reader-from-string "MZXW6YTBM")) 5 of
    (Ok out, next) -> same out (string-to-bytes "fooba") && (case stream-read next 1 of
      (Err (StreamUnexpectedEnd required actual), final) -> required == 8 && actual == 1
      other -> False)
    other -> False

  -- | Mutating a returned byte never changes decoded surplus retained for the next read.
  stream-output-isolation :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-output-isolation unit = case stream-read (base32-reader base32-standard-encoding (reader-from-string "MZXW6YTB")) 1 of
    (Ok out, next) -> let changed = mutate out 255 in
      case stream-read next 1 of (Ok rest, final) -> same rest (string-to-bytes "o"); other -> False
    other -> False

  -- | Check fixed, unpadded and arbitrary-byte stream configurations and all terminal protocol behavior.
  stream-cases :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-cases unit = let raw = string-to-bytes "foobarbazquux" in
    stream-case base32-standard-encoding raw && stream-case base32-hex-encoding raw
      && (case base32-with-padding base32-standard-encoding None of Ok encoding -> stream-case encoding raw; Err error -> False)
      && (case base32-encoding (input "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f") of
        Err error -> False
        Ok encoding -> case base32-with-padding encoding (Some 255) of Ok selected -> stream-case selected raw; Err error -> False)
      && writer-cases () && delayed-error () && truncated-stream () && stream-output-isolation ()

  -- | Compare every one-byte result directly, avoiding a quadratic concatenation in the test itself.
  check-small :: forall r. using (io :: Io r) => ByteReader r -> Bytes -> Nat ->{IO r} Bool
  check-small source expected i = case stream-read source 1 of
    (Err error, next) -> False
    (Ok out, next) -> if bytes-len out == 0 then i == bytes-len expected
      else if i < bytes-len expected && 0 < bytes-len out then bytes-get out 0 == bytes-get expected i && check-small next expected (i + 1)
      else False

  -- | Exercise a full large stream under bounded live heap and an eight-mebibyte stack.
  small-reads :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  small-reads unit = let raw = corpus 65536 False in
    check-small (base32-reader base32-standard-encoding (reader-from-bytes (base32-encode-bytes base32-standard-encoding raw))) raw 0

  -- | Recreate custom tables and ephemeral inputs before every optimized exact-load traversal.
  owner-loop :: Nat -> Bool
  owner-loop n = if n == 0 then True else
    case base32-encoding (string-to-bytes "abcdefghijklmnopqrstuvwxyz234567") of
      Err error -> False
      Ok encoding -> let raw = corpus 4096 False in let encoded = base32-encode-bytes encoding raw in
        (case base32-decode-bytes encoding encoded of Ok out -> same out raw; Err error -> False) && owner-loop (n - 1)
"#;
