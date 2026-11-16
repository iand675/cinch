//! Independent CRC-64 prepared tables, persistent checkpoints and hashing streams.
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
        ("Text.Crc64", include_str!("../prelude/Text/Crc64.cinch")),
        (
            "Effect.Crc64",
            include_str!("../prelude/Effect/Crc64.cinch"),
        ),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
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
        common::drive_with_profile(
            Mode::Build,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            Some(exe.clone()),
            &profile,
        )
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

/// Read fixed independent results without requiring an SDK during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/crc64-state-vectors.json")).unwrap()
}

/// Every base entry, polynomial fingerprint and bounds check matches independent fixtures.
#[test]
fn crc64_tables_match_all_independent_entries_and_fingerprints() {
    for row in fixture()["tables"].as_array().unwrap() {
        let entries = row["entries"]
            .as_array()
            .unwrap()
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join(",");
        run(&format!(
            "table-case {} [{}] {:?}",
            row["poly"],
            entries,
            row["empty_saved"].as_str().unwrap()
        ));
    }
}

/// All digests, checkpoints and requested splits agree at packed-word and long-input boundaries.
#[test]
fn crc64_states_match_independent_digest_checkpoint_and_split_vectors() {
    let f = fixture();
    for batch in f["valid"].as_array().unwrap().chunks(25) {
        let poly = &batch[0]["poly"];
        assert!(batch.iter().all(|row| &row["poly"] == poly));
        let checks = batch
            .iter()
            .map(|row| {
                let points = row["splits"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                format!(
                    "valid-case {poly} table {} {} {} {:?} {:?} {} {:?} {:?} {:?} [{points}]",
                    row["size"],
                    if row["fill"].as_bool().unwrap() {
                        "True"
                    } else {
                        "False"
                    },
                    row["word"],
                    row["digest"].as_str().unwrap(),
                    row["saved"].as_str().unwrap(),
                    row["mid"],
                    row["checkpoint"].as_str().unwrap(),
                    row["append_sum"].as_str().unwrap(),
                    row["append_saved"].as_str().unwrap()
                )
            })
            .collect::<Vec<_>>();
        run(&format!(
            "let table = crc64-table {poly} in all [{}]",
            checks.join(",\n")
        ));
    }
}

/// Arbitrary previous words retain every bit through prepared and polynomial updates and restored states.
#[test]
fn crc64_arbitrary_restored_words_match_reference_updates() {
    let f = fixture();
    for batch in f["arbitrary"].as_array().unwrap().chunks(75) {
        let poly = &batch[0]["poly"];
        assert!(batch.iter().all(|row| &row["poly"] == poly));
        let checks = batch
            .iter()
            .map(|row| {
                format!(
                    "arbitrary-case {poly} table {} {} {:?} {}",
                    row["size"],
                    row["seed"],
                    row["state"].as_str().unwrap(),
                    row["word"]
                )
            })
            .collect::<Vec<_>>();
        run(&format!(
            "let table = crc64-table {poly} in all [{}]",
            checks.join(",\n")
        ));
    }
}

/// Identifier errors precede size errors, followed by table-fingerprint mismatches.
#[test]
fn crc64_checkpoint_errors_match_independent_identifier_size_and_table_checks() {
    let mut groups = std::collections::BTreeMap::<u64, Vec<String>>::new();
    for row in fixture()["loads"].as_array().unwrap() {
        let kind = match row["kind"].as_str().unwrap() {
            "" => 0,
            "identifier" => 1,
            "size" => 2,
            "table" => 3,
            other => panic!("{other}"),
        };
        groups
            .entry(row["poly"].as_u64().unwrap())
            .or_default()
            .push(format!(
                "load-case table {:?} {kind}",
                row["input"].as_str().unwrap()
            ));
    }
    let checks = groups
        .into_iter()
        .map(|(poly, checks)| {
            format!(
                "(let table = crc64-table {poly} in all [{}])",
                checks.join(",\n")
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(4) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Retained branches, mutable source buffers and fresh digest/checkpoint outputs stay independent.
#[test]
fn crc64_state_buffers_tables_and_retained_branches_are_independent() {
    run("aliases && crc64-state-error-message InvalidCrc64StateIdentifier == \"CRC-64 state identifier is invalid\" && crc64-state-error-message (InvalidCrc64StateSize 20 19) == \"CRC-64 state requires 20 bytes; received 19\" && crc64-state-error-message (Crc64StateTableMismatch 0 1) == \"CRC-64 state table does not match\"");
}

/// Explicit writes, generic adapters and copy failures retain advanced source and checksum handles.
#[test]
fn crc64_streams_preserve_hashes_and_source_continuations() {
    run("stream-cases () && failed-copy () && invalid-copy () && zero-copy ()");
}

/// One-byte stream chunks run with bounded live memory and constant stack usage.
#[test]
fn crc64_single_byte_stream_copy_is_bounded_and_releases_buffers() {
    run("case crc64-copy 1 (reader-from-bytes (bytes-new 65536)) initial of (Ok count, (source, state)) -> count == 65536 && eq-u64 (crc64-state-word state) (crc64-sum crc64-ecma (bytes-new 65536)); other -> False");
}

/// Raw block/tail reads retain ephemeral table and input owners under optimized poisoned-free execution.
#[test]
fn rust_crc64_optimized_raw_buffer_owners_survive_calls_and_tail_loops() {
    let f = fixture();
    let expected = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["poly"].as_u64() == Some(0xC96C5795D7870F42)
                && row["size"] == 65536
                && row["fill"] == false
        })
        .unwrap()["word"]
        .as_u64()
        .unwrap();
    run_with_optimization(&format!("lifetime 100 {expected}"), true);
}

/// New public and private declarations have canonical module-owned documentation.
#[test]
fn crc64_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Crc64", include_str!("../prelude/Text/Crc64.cinch")),
        (
            "Effect.Crc64",
            include_str!("../prelude/Effect/Crc64.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
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
}

/// Checked consumers build input bytes from compact independent corpus descriptions.
const HELPERS: &str = r#"-- | Independent CRC-64 checkpoint and stream fixture consumers.
module Main where
  import Text.Crc64
  import Text.Binary
  import Text.Hex
  import Effect.Crc64
  import Effect.Stream
  -- | Build deterministic input independently of checksum calculations.
  data-bytes :: Nat -> Bool -> Bytes
  data-bytes n fill =
    if n <= 65536 then
      let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) in
      let b = bytes-new size in let done = fill-bytes b fill 0 in b
    else bytes-new 0
  -- | Keep constant corpus setup in the executable instead of the checker's byte-buffer evaluator.
  -- Fixture lengths fit U32; the LLVM identity preserves their exact runtime values.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Fill either all-ones bytes or a full-byte permutation with an exact bounded loop.
  fill-bytes :: Bytes -> Bool -> Nat -> Unit
  fill-bytes b fill i = if i < bytes-len b then
    let written = bytes-set b i (if fill then 255 else nat-rem (i * 37 + 11) 256) in fill-bytes b fill (i + 1)
    else ()
  -- | Parse independently recorded network-order bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err error -> bytes-new 0
  -- | Compare every byte, including the full scalar state payload.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Conjoin bounded runtime fixture batches.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Copy an exactly checked source interval for independent update calls.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start n = if start + n <= bytes-len b then
    let out = bytes-new n in let copied = bytes-copy-range out b 0 start n in out
    else bytes-new 0
  -- | Start stream fixtures with the reflected ECMA polynomial and zero checksum.
  initial :: Crc64State
  initial = crc64-state (crc64-table crc64-ecma)
  -- | Rebuild both ephemeral owners on every optimized call and release them before the next iteration.
  lifetime :: Nat -> U64 -> Bool
  lifetime count expected = if count == 0 then True else
    eq-u64 (crc64-table-sum (crc64-table crc64-ecma) (data-bytes 65536 False)) expected && lifetime (count - 1) expected
  -- | Compare complete base-table views while observing every optional indexed entry.
  table-words :: Crc64Table -> Nat -> List U64 -> List U64 -> Bool
  table-words table index expected actual = case expected of
    Nil -> case actual of Nil -> index == 256; other -> False
    Cons word rest -> case actual of
      Nil -> False
      Cons observed tail -> eq-u64 word observed && (case crc64-table-entry table index of
        Some indexed -> eq-u64 word indexed && table-words table (index + 1) rest tail
        None -> False)
  -- | Check every table entry and fingerprint, plus indices too large for a machine word.
  table-case :: U64 -> List U64 -> String -> Bool
  table-case poly expected saved = let table = crc64-table poly in
    table-words table 0 expected (crc64-table-entries table)
      && same (crc64-state-save (crc64-state table)) (input saved)
      && (case crc64-table-entry table 256 of None -> True; other -> False)
      && (case crc64-table-entry table 1267650600228229401496703205376 of None -> True; other -> False)
      && (case crc64-state-load (crc64-table poly) (input saved) of Ok state -> eq-u64 (crc64-state-word state) 0; Err error -> False)
  -- | Resume every requested split and preserve the retained prefix state and immutable table.
  splits :: Crc64Table -> Bytes -> U64 -> List Nat -> Bool
  splits table raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = crc64-state-update (crc64-state table) (part raw 0 point) in
      let saved = crc64-state-save prefix in
      case crc64-state-load table saved of
        Err error -> False
        Ok restored -> eq-u64 (crc64-state-word (crc64-state-update restored (part raw point (bytes-len raw - point)))) expected
          && same saved (crc64-state-save prefix) && splits table raw expected rest
      else False
  -- | Compare prepared and polynomial checksums, digest bytes, checkpoint bytes and all requested splits.
  valid-case :: U64 -> Crc64Table -> Nat -> Bool -> U64 -> String -> String -> Nat -> String -> String -> String -> List Nat -> Bool
  valid-case poly table n fill expected digest saved mid checkpoint appended appended-save points =
    let raw = data-bytes n fill in let start = crc64-state table in let state = crc64-state-update start raw in
    eq-u64 (crc64-sum poly raw) expected && eq-u64 (crc64-table-sum table raw) expected
      && eq-u64 (crc64-state-word state) expected && crc64-size == 8
      && crc64-state-size state == 8 && crc64-state-block-size state == 1
      && eq-u64 (crc64-state-word (crc64-state-reset state)) 0
      && same (crc64-state-sum state) (input digest) && same (crc64-state-save state) (input saved)
      && same (crc64-state-save (crc64-state-update start (part raw 0 mid))) (input checkpoint)
      && same (crc64-state-append (input "aabbcc") state) (input appended)
      && same (crc64-state-append-save (input "aabbcc") state) (input appended-save)
      && splits table raw expected points
  -- | Restore arbitrary accepted checksum bits and compare both update paths on full-byte input.
  arbitrary-case :: U64 -> Crc64Table -> Nat -> U64 -> String -> U64 -> Bool
  arbitrary-case poly table n seed saved expected = let raw = data-bytes n False in
    eq-u64 (crc64-update poly seed raw) expected && eq-u64 (crc64-table-update table seed raw) expected
      && (case crc64-state-load table (input saved) of
        Err error -> False
        Ok state -> eq-u64 (crc64-state-word (crc64-state-update state raw)) expected)
  -- | Compare checkpoint classification and the complete expected/actual error payloads.
  load-case :: Crc64Table -> String -> Nat -> Bool
  load-case table text kind = let b = input text in case crc64-state-load table b of
    Ok state -> kind == 0 && same (crc64-state-save state) b
    Err InvalidCrc64StateIdentifier -> kind == 1
    Err (InvalidCrc64StateSize expected actual) -> kind == 2 && expected == 20 && actual == bytes-len b
    Err (Crc64StateTableMismatch expected actual) -> kind == 3 && ne-u64 expected actual
      && (case binary-read-u64 BigEndian b 4 of Ok word -> eq-u64 word actual; Err error -> False)
      && (case binary-read-u64 BigEndian (crc64-state-save (crc64-state table)) 4 of Ok word -> eq-u64 word expected; Err error -> False)
  -- | Mutations of borrowed inputs, returned digests and checkpoint bytes cannot change retained states.
  aliases :: Bool
  aliases = let table = crc64-table crc64-ecma in let start = crc64-state table in
    let raw = string-to-bytes "hello" in let state = crc64-state-update start raw in
    let first = crc64-state-update state (string-to-bytes "a") in
    let last = crc64-state-update state (string-to-bytes "b") in
    let saved = crc64-state-save state in let observed = crc64-state-sum state in
    let prefix = input "aabbcc" in let appended = crc64-state-append prefix state in
    let changed-prefix = if 0 < bytes-len prefix then bytes-set prefix 0 0 else () in
    let changed-input = if 0 < bytes-len raw then bytes-set raw 0 0 else () in
    let changed-sum = if 0 < bytes-len observed then bytes-set observed 0 0 else () in
    case crc64-state-load table saved of
      Err error -> False
      Ok loaded -> let changed-save = if 12 < bytes-len saved then bytes-set saved 12 0 else () in
        eq-u64 (crc64-state-word loaded) (crc64-state-word state)
          && eq-u64 (crc64-state-word state) (crc64-table-sum (crc64-table crc64-ecma) (string-to-bytes "hello"))
          && eq-u64 (crc64-state-word first) (crc64-table-sum table (string-to-bytes "helloa"))
          && eq-u64 (crc64-state-word last) (crc64-table-sum table (string-to-bytes "hellob"))
          && same appended (crc64-state-append (input "aabbcc") state)
          && eq-u64 (crc64-state-word start) 0
          && eq-u64 (crc64-state-word (crc64-state-update (crc64-state-reset state) (string-to-bytes "hello"))) (crc64-state-word state)
  -- | Explicit writes and generic writer adaptation preserve complete counts and earlier states.
  stream-cases :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-cases unit = let start = initial in case crc64-write start (string-to-bytes "hello") of
    (n, next) -> n == 5 && eq-u64 (crc64-state-word next) (crc64-sum crc64-ecma (string-to-bytes "hello"))
      && eq-u64 (crc64-state-word start) 0
      && (case stream-copy 3 (reader-from-string "abcdefg") (crc64-hashing-writer next) of (Ok count, (source, sink)) -> count == 7; other -> False)
      && (case crc64-copy 3 (reader-from-string "abcdefg") start of (Ok count, (source, state)) -> count == 7 && eq-u64 (crc64-state-word state) (crc64-sum crc64-ecma (string-to-bytes "abcdefg")); other -> False)
      && (case crc64-copy 3 (reader-from-string "") next of (Ok count, (source, state)) -> count == 0 && eq-u64 (crc64-state-word state) (crc64-state-word next); other -> False)
  -- | Emit one successful prefix and an error carrying an independently readable source continuation.
  failing :: forall r. using (io :: Io r) => ByteReader r
  failing = stream-reader (\n -> (Ok (string-to-bytes "ab"), stream-reader (\m -> (Err (StreamShortWrite 7 3), reader-from-string "z"))))
  -- | Failed copying retains the checksum of earlier successful bytes and the source returned with the error.
  failed-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  failed-copy unit = case crc64-copy 2 failing initial of
    (Err (StreamShortWrite required accepted), (source, state)) -> required == 7 && accepted == 3 && eq-u64 (crc64-state-word state) (crc64-sum crc64-ecma (string-to-bytes "ab"))
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "z"); other -> False)
    other -> False
  -- | Invalid source counts never enter the checksum; their advanced source remains observable.
  invalid-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  invalid-copy unit = let source = stream-reader (\n -> (Ok (string-to-bytes "bad"), reader-from-string "x")) in
    case crc64-copy 1 source initial of
      (Err (StreamInvalidCount requested actual), (next, state)) -> requested == 1 && actual == 3 && eq-u64 (crc64-state-word state) 0
        && (case stream-read next 1 of (Ok b, after) -> same b (string-to-bytes "x"); other -> False)
      other -> False
  -- | A zero chunk rejects before consuming input and preserves the supplied checksum state.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case crc64-copy 0 (reader-from-string "a") initial of
    (Err StreamInvalidChunkSize, (source, state)) -> eq-u64 (crc64-state-word state) 0
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "a"); other -> False)
    other -> False
"#;
