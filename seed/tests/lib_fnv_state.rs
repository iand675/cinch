//! Independent FNV checkpoints, persistent branches and inspectable hashing streams.
mod common;

use serde_json::Value;

/// Compile actual source modules and verify normal reclamation separately from poisoned frees.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Text.Fnv.State",
            include_str!("../prelude/Text/Fnv/State.cinch"),
        ),
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Effect.Fnv", include_str!("../prelude/Effect/Fnv.cinch")),
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
    std::fs::write(directory.path().join("Main.cinch"), format!("{HELPERS}\n  -- | Observe the complete fixture result under one runtime identity.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if ({body}) then 0 else 1\n")).unwrap();
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

/// Read precomputed reference observations without invoking a reference SDK during tests.
fn fixtures() -> Value {
    serde_json::from_str(include_str!("fixtures/fnv-state-vectors.json")).unwrap()
}

/// All six variants match independent digest/checkpoint bytes and resume at every split.
#[test]
fn fnv_states_match_serialized_reference_and_all_chunk_splits() {
    let checks = fixtures()["valid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "valid-case {} {:?} {:?} {:?} {:?} {} {:?} {:?} {:?}",
                row["variant"],
                row["input"].as_str().unwrap(),
                row["digest"].as_str().unwrap(),
                row["initial"].as_str().unwrap(),
                row["saved"].as_str().unwrap(),
                row["mid"],
                row["checkpoint"].as_str().unwrap(),
                row["append_sum"].as_str().unwrap(),
                row["append_saved"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(36) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Arbitrary restored payloads preserve every high bit and use the selected recurrence order.
#[test]
fn fnv_restored_arbitrary_states_match_independent_updates() {
    let checks = fixtures()["arbitrary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "restore-case {} {:?} {:?} {:?}",
                row["variant"],
                row["seed"].as_str().unwrap(),
                row["input"].as_str().unwrap(),
                row["digest"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(72) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Missing/wrong identifiers precede size errors, including every other variant and extra bytes.
#[test]
fn fnv_checkpoint_errors_match_independent_variant_and_length_checks() {
    let checks = fixtures()["loads"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let kind = match row["kind"].as_str().unwrap() {
                "" => 0,
                "identifier" => 1,
                "size" => 2,
                other => panic!("{other}"),
            };
            format!(
                "load-case {} {:?} {kind}",
                row["variant"],
                row["input"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(100) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Mutation cannot affect retained hash branches, loaded states or later fresh observations.
#[test]
fn fnv_state_observations_and_inputs_do_not_create_mutable_aliases() {
    run("aliases && fnv-state-error-message InvalidFnvStateIdentifier == \"FNV state identifier does not match the selected variant\" && fnv-state-error-message (InvalidFnvStateSize 20 19) == \"FNV state requires 20 bytes; received 19\"");
}

/// Corrupt each identifier byte independently while retaining an otherwise valid complete checkpoint.
#[test]
fn fnv_checkpoint_rejects_each_corrupted_identifier_byte() {
    let checks = fixtures()["identifiers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "load-case {} {:?} 1",
                row["variant"],
                row["input"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Explicit sinks expose their advanced hash and retain it when sources fail or misuse the protocol.
#[test]
fn fnv_stream_writers_and_copies_preserve_counts_failures_and_continuations() {
    run("streams () && copy-failure () && zero-copy () && invalid-reader () && generic-writer ()");
}

/// One-byte reads keep a large input bounded in live heap and stack, under both reclamation modes.
#[test]
fn fnv_single_byte_stream_copy_releases_previous_continuations() {
    run("case fnv-copy 1 (reader-from-bytes (bytes-new 65536)) (fnv-writer fnv128a-state) of (Ok count, (source, writer)) -> count == 65536 && same (fnv-state-sum (fnv-writer-state writer)) (fnv-state-sum (fnv-state-update fnv128a-state (bytes-new 65536))); other -> False");
}

/// Every new library declaration uses canonical documentation and public prose names Cinch's contracts.
#[test]
fn fnv_state_and_stream_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Fnv.State",
            include_str!("../prelude/Text/Fnv/State.cinch"),
        ),
        ("Effect.Fnv", include_str!("../prelude/Effect/Fnv.cinch")),
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
            assert!(!doc.legacy, "{}: use canonical docs", declaration.id);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}

const HELPERS: &str = r#"-- | Runtime consumers for independent scalar checkpoints and streaming behavior.
module Main where
  import Text.Fnv.State
  import Text.Binary
  import Text.Hex
  import Effect.Fnv
  import Effect.Stream
  -- | Select one of six fixed-width variants in fixture order.
  initial :: Nat -> FnvState
  initial n = if n == 0 then fnv32-state else if n == 1 then fnv32a-state else if n == 2 then fnv64-state else if n == 3 then fnv64a-state else if n == 4 then fnv128-state else fnv128a-state
  -- | Convert independently recorded hexadecimal fixture bytes into a runtime buffer.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err error -> bytes-new 0
  -- | Compare full byte contents, including all high and low hash bits.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Conjoin bounded batches of runtime fixture observations.
  all :: List Bool -> Bool
  all checks = case checks of Nil -> True; Cons first rest -> first && all rest
  -- | Copy one checked interval without retaining mutable aliases.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start count = if start + count <= bytes-len b then
    let out = bytes-new count in let copied = bytes-copy-range out b 0 start count in out
    else bytes-new 0
  -- | Confirm numeric observations use the exact width and do not truncate wider states.
  numeric :: Nat -> FnvState -> Bytes -> Bool
  numeric variant state expected =
    if variant < 2 then (case fnv-state-word32 state of Some word -> same (binary-u32 BigEndian word) expected; None -> False)
      && (case fnv-state-word64 state of None -> True; other -> False) && (case fnv-state-words128 state of None -> True; other -> False)
    else if variant < 4 then (case fnv-state-word64 state of Some word -> same (binary-u64 BigEndian word) expected; None -> False)
      && (case fnv-state-word32 state of None -> True; other -> False) && (case fnv-state-words128 state of None -> True; other -> False)
    else (case fnv-state-words128 state of Some (high, low) -> same (binary-u64 BigEndian high) (part expected 0 8) && same (binary-u64 BigEndian low) (part expected 8 8); None -> False)
      && (case fnv-state-word32 state of None -> True; other -> False) && (case fnv-state-word64 state of None -> True; other -> False)
  -- | Save and independently resume every possible two-chunk split, retaining the prefix branch.
  splits :: FnvState -> Bytes -> Bytes -> Nat -> Bool
  splits base raw expected split = if split <= bytes-len raw then
    let prefix = fnv-state-update base (part raw 0 split) in
    let saved = fnv-state-save prefix in
    case fnv-state-load base saved of
      Err error -> False
      Ok loaded -> same (fnv-state-sum (fnv-state-update loaded (part raw split (bytes-len raw - split)))) expected
        && same (fnv-state-save prefix) saved && splits base raw expected (split + 1)
    else True
  -- | Observe digest, checkpoint, reset, append and mid-stream checkpoints against independent bytes.
  valid-case :: Nat -> String -> String -> String -> String -> Nat -> String -> String -> String -> Bool
  valid-case variant raw digest initial-save saved mid checkpoint appended appended-save =
    let base = initial variant in let bytes = input raw in let expected = input digest in
    let state = fnv-state-update base bytes in
    same (fnv-state-sum state) expected && same (fnv-state-save state) (input saved)
      && same (fnv-state-save base) (input initial-save) && same (fnv-state-save (fnv-state-reset state)) (input initial-save)
      && fnv-state-size state == bytes-len expected && fnv-state-block-size state == 1
      && same (fnv-state-save (fnv-state-update base (part bytes 0 mid))) (input checkpoint)
      && same (fnv-state-append (input "aabbcc") state) (input appended)
      && same (fnv-state-append-save (input "aabbcc") state) (input appended-save)
      && numeric variant state expected && splits base bytes expected 0
  -- | Resume an arbitrary externally supplied state and compare its resulting complete digest.
  restore-case :: Nat -> String -> String -> String -> Bool
  restore-case variant saved raw digest = case fnv-state-load (initial variant) (input saved) of
    Err error -> False
    Ok state -> same (fnv-state-sum (fnv-state-update state (input raw))) (input digest)
  -- | Classify independent malformed checkpoints, retaining both precise expected and actual lengths.
  load-case :: Nat -> String -> Nat -> Bool
  load-case variant saved kind = let b = input saved in let base = initial variant in
    case fnv-state-load base b of
      Ok state -> kind == 0 && same (fnv-state-save state) b
      Err InvalidFnvStateIdentifier -> kind == 1
      Err (InvalidFnvStateSize expected actual) -> kind == 2 && expected == 4 + fnv-state-size base && actual == bytes-len b
  -- | Mutate every returned or supplied byte carrier after consumption and retain earlier scalar states.
  aliases :: Bool
  aliases = let raw = string-to-bytes "hello" in let state = fnv-state-update fnv128a-state raw in
    let saved = fnv-state-save state in let observed = fnv-state-sum state in
    let changed-input = if 0 < bytes-len raw then bytes-set raw 0 0 else () in
    let changed-digest = if 0 < bytes-len observed then bytes-set observed 0 0 else () in
    case fnv-state-load fnv128a-state saved of
      Err error -> False
      Ok loaded -> let changed-save = if 0 < bytes-len saved then bytes-set saved 0 0 else () in
        same (fnv-state-sum loaded) (fnv-state-sum state)
          && same (fnv-state-sum state) (fnv-state-sum (fnv-state-update fnv128a-state (string-to-bytes "hello")))
  -- | Explicit writes accept full chunks, preserve earlier handles and allow reset/continued hashing.
  streams :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  streams unit = let writer = fnv-writer fnv64a-state in
    case fnv-write writer (string-to-bytes "hello") of
      (n, next) -> case fnv-write next (bytes-new 0) of
        (empty-count, advanced) -> n == 5 && empty-count == 0
          && same (fnv-state-sum (fnv-writer-state writer)) (fnv-state-sum fnv64a-state)
          && same (fnv-state-sum (fnv-writer-state advanced)) (fnv-state-sum (fnv-state-update fnv64a-state (string-to-bytes "hello")))
  -- | Supply one prefix, then a source error whose advanced continuation is separately readable.
  failing :: forall r. using (io :: Io r) => ByteReader r
  failing = stream-reader (\n -> (Ok (string-to-bytes "ab"), stream-reader (\m -> (Err (StreamShortWrite 7 3), reader-from-string "z"))))
  -- | A source failure retains the hash of its accepted prefix and the exact advanced source.
  copy-failure :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  copy-failure unit = case fnv-copy 2 failing (fnv-writer fnv32-state) of
    (Err (StreamShortWrite required accepted), (source, writer)) -> required == 7 && accepted == 3
      && same (fnv-state-sum (fnv-writer-state writer)) (fnv-state-sum (fnv-state-update fnv32-state (string-to-bytes "ab")))
      && (case stream-read source 1 of (Ok bytes, next) -> same bytes (string-to-bytes "z"); other -> False)
    other -> False
  -- | Reject zero chunks without advancing the reader or changing the hash.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case fnv-copy 0 (reader-from-string "a") (fnv-writer fnv32a-state) of
    (Err StreamInvalidChunkSize, (source, writer)) -> same (fnv-state-sum (fnv-writer-state writer)) (fnv-state-sum fnv32a-state)
      && (case stream-read source 1 of (Ok bytes, next) -> same bytes (string-to-bytes "a"); other -> False)
    other -> False
  -- | Invalid source counts do not hash the oversized chunk; its returned continuation is preserved.
  invalid-reader :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  invalid-reader unit = let source = stream-reader (\n -> (Ok (string-to-bytes "bad"), reader-from-string "x")) in
    case fnv-copy 1 source (fnv-writer fnv64-state) of
      (Err (StreamInvalidCount requested actual), (next, writer)) -> requested == 1 && actual == 3
        && same (fnv-state-sum (fnv-writer-state writer)) (fnv-state-sum fnv64-state)
        && (case stream-read next 1 of (Ok bytes, after) -> same bytes (string-to-bytes "x"); other -> False)
      other -> False
  -- | Generic writer callbacks compose with existing copy and byte-count validation.
  generic-writer :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  generic-writer unit = case stream-copy 3 (reader-from-string "abcdefg") (fnv-hashing-writer fnv128-state) of
    (Ok count, (source, sink)) -> count == 7 && (case stream-write sink (string-to-bytes "tail") of (Ok accepted, advanced) -> accepted == 4; other -> False)
    other -> False
"#;
