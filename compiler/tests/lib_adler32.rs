//! Independent Adler-32 segment boundaries, persistent checkpoints and hashing streams.
mod common;

use serde_json::Value;

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Text.Adler32",
            include_str!("../prelude/Text/Adler32.cinch"),
        ),
        (
            "Effect.Adler32",
            include_str!("../prelude/Effect/Adler32.cinch"),
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

/// Read fixed independent results without requiring an SDK during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/adler32-state-vectors.json")).unwrap()
}

/// Full digests, checkpoints and splits agree around every SIMD and 5552-byte segment boundary.
#[test]
fn adler32_states_match_independent_digest_checkpoint_and_split_vectors() {
    let checks = fixture()["valid"]
        .as_array()
        .unwrap()
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
                "valid-case {} {} {} {:?} {:?} {} {:?} {:?} {:?} [{points}]",
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
    for batch in checks.chunks(16) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Accepted arbitrary checkpoint payloads retain explicit wrapping, including all-ones full segments.
#[test]
fn adler32_arbitrary_restored_words_match_reference_segment_wrapping() {
    let checks = fixture()["arbitrary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "arbitrary-case {} {} {} {:?} {}",
                row["size"],
                if row["fill"].as_bool().unwrap() {
                    "True"
                } else {
                    "False"
                },
                row["seed"],
                row["state"].as_str().unwrap(),
                row["word"]
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(72) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Version/magic errors precede exact-size errors and no invalid prefix is accepted.
#[test]
fn adler32_checkpoint_errors_match_independent_identity_and_size_checks() {
    let checks = fixture()["loads"]
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
            format!("load-case {:?} {kind}", row["input"].as_str().unwrap())
        })
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Earlier states, copied append prefixes and loaded states remain isolated from mutation.
#[test]
fn adler32_state_buffers_and_retained_branches_are_independent() {
    run("aliases && adler32-state-error-message InvalidAdler32StateIdentifier == \"Adler-32 state identifier is invalid\" && adler32-state-error-message (InvalidAdler32StateSize 8 7) == \"Adler-32 state requires 8 bytes; received 7\"");
}

/// Explicit writes, generic adapters and copy failures preserve counts and advanced handles.
#[test]
fn adler32_streams_preserve_hashes_and_source_continuations() {
    run("stream-cases () && failed-copy () && invalid-copy () && zero-copy ()");
}

/// One-byte source chunks run within ordinary live-space limits and do not grow the call stack.
#[test]
fn adler32_single_byte_stream_copy_is_bounded_and_releases_buffers() {
    run("case adler32-copy 1 (reader-from-bytes (bytes-new 65536)) adler32-state of (Ok count, (source, state)) -> count == 65536 && eq-u32 (adler32-state-word state) (adler32-sum (bytes-new 65536)); other -> False");
}

/// New public and private declarations have canonical module-owned documentation.
#[test]
fn adler32_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Adler32",
            include_str!("../prelude/Text/Adler32.cinch"),
        ),
        (
            "Effect.Adler32",
            include_str!("../prelude/Effect/Adler32.cinch"),
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
const HELPERS: &str = r#"-- | Independent Adler-32 checkpoint and stream fixture consumers.
module Main where
  import Text.Adler32
  import Text.Binary
  import Text.Hex
  import Effect.Adler32
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
  -- | Resume every requested split and preserve the saved prefix state.
  splits :: Bytes -> U32 -> List Nat -> Bool
  splits raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = adler32-state-update adler32-state (part raw 0 point) in
      let saved = adler32-state-save prefix in
      case adler32-state-load saved of
        Err error -> False
        Ok restored -> eq-u32 (adler32-state-word (adler32-state-update restored (part raw point (bytes-len raw - point)))) expected
          && same saved (adler32-state-save prefix) && splits raw expected rest
      else False
  -- | Compare every digest/state observation with independent network-order fixtures.
  valid-case :: Nat -> Bool -> U32 -> String -> String -> Nat -> String -> String -> String -> List Nat -> Bool
  valid-case n fill expected digest saved mid checkpoint appended appended-save points =
    let raw = data-bytes n fill in let state = adler32-state-update adler32-state raw in
    eq-u32 (adler32-sum raw) expected && eq-u32 (adler32-state-word state) expected
      && adler32-state-size state == 4 && adler32-state-block-size state == 4
      && eq-u32 (adler32-state-word (adler32-state-reset state)) 1
      && same (adler32-state-sum state) (input digest) && same (adler32-state-save state) (input saved)
      && same (adler32-state-save (adler32-state-update adler32-state (part raw 0 mid))) (input checkpoint)
      && same (adler32-state-append (input "aabbcc") state) (input appended)
      && same (adler32-state-append-save (input "aabbcc") state) (input appended-save)
      && splits raw expected points
  -- | Restore arbitrary accepted bits before a single update, including overflow-sensitive long input.
  arbitrary-case :: Nat -> Bool -> U32 -> String -> U32 -> Bool
  arbitrary-case n fill seed saved expected = let raw = data-bytes n fill in
    eq-u32 (adler32-update seed raw) expected && (case adler32-state-load (input saved) of
      Err error -> False
      Ok state -> eq-u32 (adler32-state-word (adler32-state-update state raw)) expected)
  -- | Compare malformed checkpoint classification and complete required/actual byte lengths.
  load-case :: String -> Nat -> Bool
  load-case text kind = let b = input text in case adler32-state-load b of
    Ok state -> kind == 0 && same (adler32-state-save state) b
    Err InvalidAdler32StateIdentifier -> kind == 1
    Err (InvalidAdler32StateSize expected actual) -> kind == 2 && expected == 8 && actual == bytes-len b
  -- | Mutating inputs, returned bytes and append prefixes cannot change scalar state branches.
  aliases :: Bool
  aliases = let raw = string-to-bytes "hello" in let state = adler32-state-update adler32-state raw in
    let saved = adler32-state-save state in let observed = adler32-state-sum state in
    let prefix = input "aabbcc" in let appended = adler32-state-append prefix state in
    let changed-prefix = if 0 < bytes-len prefix then bytes-set prefix 0 0 else () in
    let changed-input = if 0 < bytes-len raw then bytes-set raw 0 0 else () in
    let changed-sum = if 0 < bytes-len observed then bytes-set observed 0 0 else () in
    case adler32-state-load saved of
      Err error -> False
      Ok loaded -> let changed-save = if 4 < bytes-len saved then bytes-set saved 4 0 else () in
        eq-u32 (adler32-state-word loaded) (adler32-state-word state)
          && eq-u32 (adler32-state-word state) (adler32-sum (string-to-bytes "hello"))
          && same appended (adler32-state-append (input "aabbcc") state)
          && eq-u32 (adler32-state-word adler32-state) 1
  -- | Explicit scalar writes and generic writer adaptation retain full counts without changing prior states.
  stream-cases :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-cases unit = case adler32-write adler32-state (string-to-bytes "hello") of
    (n, next) -> n == 5 && eq-u32 (adler32-state-word next) (adler32-sum (string-to-bytes "hello"))
      && eq-u32 (adler32-state-word adler32-state) 1
      && (case stream-copy 3 (reader-from-string "abcdefg") (adler32-hashing-writer next) of (Ok count, (source, sink)) -> count == 7; other -> False)
  -- | Emit one successful prefix followed by an error with a separately readable advanced source.
  failing :: forall r. using (io :: Io r) => ByteReader r
  failing = stream-reader (\n -> (Ok (string-to-bytes "ab"), stream-reader (\m -> (Err (StreamShortWrite 7 3), reader-from-string "z"))))
  -- | Failure preserves the checksum of earlier successful bytes and the source supplied with that error.
  failed-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  failed-copy unit = case adler32-copy 2 failing adler32-state of
    (Err (StreamShortWrite required accepted), (source, state)) -> required == 7 && accepted == 3 && eq-u32 (adler32-state-word state) (adler32-sum (string-to-bytes "ab"))
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "z"); other -> False)
    other -> False
  -- | Oversized source chunks do not change the checksum, and their advanced source remains observable.
  invalid-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  invalid-copy unit = let source = stream-reader (\n -> (Ok (string-to-bytes "bad"), reader-from-string "x")) in
    case adler32-copy 1 source adler32-state of
      (Err (StreamInvalidCount requested actual), (next, state)) -> requested == 1 && actual == 3 && eq-u32 (adler32-state-word state) 1
        && (case stream-read next 1 of (Ok b, after) -> same b (string-to-bytes "x"); other -> False)
      other -> False
  -- | A zero chunk rejects before consuming input and preserves the supplied checksum state.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case adler32-copy 0 (reader-from-string "a") adler32-state of
    (Err StreamInvalidChunkSize, (source, state)) -> eq-u32 (adler32-state-word state) 1
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "a"); other -> False)
    other -> False
"#;
