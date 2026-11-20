//! Effectful hexadecimal codecs: independent wire results, progress, snapshots and finalization.
mod common;

use serde_json::Value;

/// Create fixture input through base byte operations, independently of hexadecimal decoding.
fn bytes(hex: &str) -> String {
    let values = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("(bytes-from-u8s [{values}])")
}

/// Check ordinary live-memory bounds separately from the debug runtime’s freed-block quarantine.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Effect.Hex", include_str!("../prelude/Effect/Hex.cinch")),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let source=format!("{HELPERS}\n  -- | Run the complete effectful consumer under one runtime identity.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in\n    if ({body}) then 0 else 1\n");
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
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
            // Poisoned blocks remain allocated intentionally; only the ordinary run measures live space.
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

/// Buffer and stream encodings agree with independent fixtures, including every dump split.
#[test]
fn hexadecimal_writers_and_dumps_match_reference_at_every_split() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let checks = fixture["buffers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let raw = row["hex"].as_str().unwrap();
            let encoded: String = raw
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            let dump: String = row["hex_dump"]
                .as_str()
                .unwrap()
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            format!(
                "(encoded-case {} {} && dump-splits {} {} 0)",
                bytes(raw),
                bytes(&encoded),
                bytes(raw),
                bytes(&dump)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n      ");
    run(&format!("all [\n      {checks}\n    ]"));
}

/// Valid prefixes precede exact malformed-input errors across every source split and small read size.
#[test]
fn hexadecimal_readers_preserve_prefixes_offsets_and_sticky_errors() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let checks = fixture["hex_decodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let kind = match row["kind"].as_str().unwrap() {
                "" => 0,
                "length" => 1,
                "byte" => 2,
                other => panic!("{other}"),
            };
            format!(
                "(decode-splits {} {} {kind} {} 0)",
                bytes(row["encoded_hex"].as_str().unwrap()),
                bytes(row["decoded_hex"].as_str().unwrap()),
                row["offset"].as_i64().unwrap().max(0)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n      ");
    run(&format!("all [\n      {checks}\n    ]"));
}

/// Retry odd short writes, preserve failed-sink progress, and keep failures from replaying output.
#[test]
fn hexadecimal_stream_short_writes_failures_and_finish_are_observable() {
    run("short-writes () && writer-failure () && dump-failure () && finish-state () && zero-and-source-failure () && mutable-tail () && isolated-output () && isolated-dump-tail () && generic-copy ()");
}

/// One-byte reads drain a large buffer without recopying unread surplus or retaining dead handles.
#[test]
fn hexadecimal_small_reads_keep_bounded_surplus_and_release_buffers() {
    run("large-small-reads ()");
}

/// Adapter declarations and private callbacks carry canonical language-owned documentation.
#[test]
fn hexadecimal_stream_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        include_str!("../prelude/Effect/Hex.cinch"),
        Some("Effect.Hex"),
        "Effect.Hex",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
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

/// Effectful fixture consumers retain only advanced handles, so leak checks cover actual continuation lifetimes.
const HELPERS: &str = r#"module Main where
  import Effect.Hex
  import Effect.Stream
  import Text.Hex

  -- | Compare arbitrary bytes without a text interpretation.
  equal :: Bytes -> Bytes -> Bool
  equal a b = eq-i64 (bytes-compare a b) 0

  -- | Snapshot a checked fixture interval.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start end = if start <= end && end <= bytes-len b then
    let out = bytes-new (end - start) in let copied = bytes-copy-range out b 0 start (end - start) in out
    else bytes-new 0

  -- | Concatenate independently owned expected chunks.
  join :: Bytes -> Bytes -> Bytes
  join a b = let out = bytes-new (bytes-len a + bytes-len b) in
    let first = bytes-copy-range out a 0 0 (bytes-len a) in
    let last = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in out

  -- | Consume the completed Boolean checks after their effects have run.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons valid rest -> valid && all rest

  -- | Observe one encoding through the explicit advanced writer interface.
  encoded-case :: forall r. using (io :: Io r) => Bytes -> Bytes ->{IO r} Bool
  encoded-case input expected = case hex-write (hex-writer writer-buffer) input of
    (Ok count, next) -> count == bytes-len input && (case writer-bytes (hex-writer-sink next) of Some out -> equal out expected; None -> False)
    _ -> False

  -- | Finish two dump writes, including empty prefixes and a partial last row.
  dump-at :: forall r. using (io :: Io r) => Bytes -> Bytes -> Nat ->{IO r} Bool
  dump-at input expected split = case hex-dump-write (hex-dumper writer-buffer) (part input 0 split) of
    (Ok first, one) -> case hex-dump-write one (part input split (bytes-len input)) of
      (Ok last, two) -> case hex-dump-finish two of
        (Ok unit, done) -> first == split && last == saturating-sub (bytes-len input) split && (case writer-bytes (hex-dumper-sink done) of Some out -> equal out expected; None -> False)
        _ -> False
      _ -> False
    _ -> False

  -- | Walk every possible input split at runtime.
  dump-splits :: forall r. using (io :: Io r) => Bytes -> Bytes -> Nat ->{IO r} Bool
  dump-splits input expected split = if split <= bytes-len input then dump-at input expected split && dump-splits input expected (split + 1) else True

  -- | Distinguish clean EOF, incomplete pairs and original invalid-byte positions.
  failure-is :: Option StreamError -> Nat -> Nat -> Bool
  failure-is error kind offset = case error of
    None -> kind == 0
    Some (StreamUnexpectedEnd required received) -> kind == 1 && required == 2 && received == 1
    Some (StreamEncodingError name position message) -> kind == 2 && name == "hex" && position == offset
    _ -> False

  -- | Read completed chunks before EOF or failure and retain the terminal reader for a retry.
  observe :: forall r. using (io :: Io r) => ByteReader r -> Nat -> Bytes ->{IO r} (Bytes, Option StreamError, ByteReader r)
  observe source size acc = case stream-read source size of
    (Err error, next) -> (acc, Some error, next)
    (Ok chunk, next) -> if bytes-len chunk == 0 then (acc, None, next) else observe next size (join acc chunk)

  -- | Check a segmented source, its output prefix and a repeated terminal read.
  decode-at :: forall r. using (io :: Io r) => Bytes -> Bytes -> Nat -> Nat -> Nat -> Nat ->{IO r} Bool
  decode-at input expected kind offset split size =
    let source = reader-concat [reader-from-bytes (part input 0 split), reader-from-bytes (part input split (bytes-len input))] in
    case observe (hex-reader source) size (bytes-new 0) of
      (out, failure, next) -> equal out expected && failure-is failure kind offset &&
        (case stream-read next 1 of
          (Err error, again) -> kind /= 0 && failure-is (Some error) kind offset
          (Ok empty, again) -> kind == 0 && bytes-len empty == 0)

  -- | Exercise every segmentation with single-byte and wider decoded read requests.
  decode-splits :: forall r. using (io :: Io r) => Bytes -> Bytes -> Nat -> Nat -> Nat ->{IO r} Bool
  decode-splits input expected kind offset split = if split <= bytes-len input then
    decode-at input expected kind offset split 1 && decode-at input expected kind offset split 7 && decode-splits input expected kind offset (split + 1)
    else True

  -- | Accept an odd encoded prefix once, then become an ordinary inspectable memory sink.
  short-first :: forall r. using (io :: Io r) => Nat -> ByteWriter r
  short-first maximum = stream-writer (\input -> stream-write writer-buffer (part input 0 (if maximum < bytes-len input then maximum else bytes-len input)))

  -- | Return a sentinel failure while exposing the already advanced underlying memory sink.
  failed-after :: forall r. using (io :: Io r) => ByteWriter r -> ByteWriter r
  failed-after sink = stream-writer (\input -> (Err (StreamEncodingError "sink" 0 "sentinel"), sink))

  -- | Accept three output bytes before the next write fails.
  short-then-fail :: forall r. using (io :: Io r) => ByteWriter r
  short-then-fail = stream-writer (\input -> case stream-write writer-buffer (part input 0 (if 3 < bytes-len input then 3 else bytes-len input)) of
    (result, next) -> (result, failed-after next))

  -- | Verify retrying an odd short write never duplicates an encoded nibble.
  short-writes :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  short-writes unit = case hex-write (hex-writer (short-first 3)) (bytes-from-u8s [0,1,2,3]) of
    (Ok count, next) -> count == 4 && (case writer-bytes (hex-writer-sink next) of Some out -> equal out (bytes-from-u8s [48,48,48,49,48,50,48,51]); None -> False)
    _ -> False

  -- | Preserve accepted encoded output on failure and never touch that sink on a retry.
  writer-failure :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  writer-failure unit = case hex-write (hex-writer short-then-fail) (bytes-from-u8s [0,1,2,3]) of
    (Err error, failed) -> case hex-write failed (bytes-from-u8s [255]) of
      (Err again, final) -> case writer-bytes (hex-writer-sink final) of Some out -> equal out (bytes-from-u8s [48,48,48]); None -> False
      _ -> False
    _ -> False

  -- | A dump output failure remains sticky through writes and finalization.
  dump-failure :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  dump-failure unit = case hex-dump-write (hex-dumper short-then-fail) (bytes-new 16) of
    (Err error, failed) -> case hex-dump-finish failed of
      (Err again, final) -> case writer-bytes (hex-dumper-sink final) of Some out -> equal out (bytes-from-u8s [48,48,48]); None -> False
      _ -> False
    _ -> False

  -- | Finish is idempotent, and even an empty write after finish reports a named closed adapter.
  finish-state :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  finish-state unit = case hex-dump-write (hex-dumper writer-buffer) (bytes-from-u8s [65]) of
    (Ok count, next) -> case hex-dump-finish next of
      (Ok unit, done) -> case hex-dump-finish done of
        (Ok again, still) -> case hex-dump-write still (bytes-new 0) of
          (Err (StreamClosed name), final) -> name == "hex-dump" && (case writer-bytes (hex-dumper-sink final) of Some out -> equal out (hex-dump-bytes 0 (bytes-from-u8s [65])); None -> False)
          _ -> False
        _ -> False
      _ -> False
    _ -> False

  -- | Record an underlying read and then fail, with an EOF continuation that must never be replayed.
  counted-failure :: forall r. using (io :: Io r) => Bytes -> ByteReader r
  counted-failure counter = stream-reader (\n ->
    let counted = if 0 < bytes-len counter then let old = bytes-get counter 0 in bytes-set counter 0 (if old < 255 then old + 1 else old) else () in
    (Err (StreamEncodingError "source" 0 "sentinel"), reader-from-bytes (bytes-new 0)))

  -- | Zero reads avoid the source; later failures remain sticky rather than turning into EOF.
  zero-and-source-failure :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-and-source-failure unit = let counter = bytes-new 1 in
    case stream-read (hex-reader (counted-failure counter)) 0 of
      (Ok empty, next) -> bytes-len empty == 0 && bytes-get counter 0 == 0 &&
        (case stream-read next 1 of
          (Err error, failed) -> bytes-get counter 0 == 1 && (case stream-read failed 1 of (Err again, final) -> bytes-get counter 0 == 1; _ -> False)
          _ -> False)
      _ -> False

  -- | Reuse one mutable source byte for two successive digits.
  reused-source :: forall r. using (io :: Io r) => Bytes -> Nat -> ByteReader r
  reused-source buffer phase = stream-reader (\n -> if phase < 2 then
    let stored = if 0 < bytes-len buffer then bytes-set buffer 0 (if phase == 0 then 52 else 49) else () in
    (Ok buffer, reused-source buffer (phase + 1))
    else (Ok (bytes-new 0), reader-from-bytes (bytes-new 0)))

  -- | The decoder must snapshot its unfinished digit before fetching a reused source buffer.
  mutable-tail :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  mutable-tail unit = case stream-read (hex-reader (reused-source (bytes-new 1) 0)) 1 of
    (Ok out, next) -> equal out (bytes-from-u8s [65])
    _ -> False

  -- | Mutating a returned byte cannot change the next byte retained by the decoder.
  isolated-output :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  isolated-output unit = case stream-read (hex-reader (reader-from-bytes (bytes-from-u8s [48,48,48,49,48,50,48,51]))) 1 of
    (Ok out, next) -> let changed = if 0 < bytes-len out then bytes-set out 0 255 else () in
      case stream-read next 1 of (Ok last, final) -> equal last (bytes-from-u8s [1]); _ -> False
    _ -> False

  -- | Pending dump bytes are snapshots rather than aliases of the caller's input.
  isolated-dump-tail :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  isolated-dump-tail unit = let input = bytes-from-u8s [65] in
    case hex-dump-write (hex-dumper writer-buffer) input of
      (Ok count, next) -> let changed = if 0 < bytes-len input then bytes-set input 0 90 else () in
        case hex-dump-finish next of (Ok unit, done) -> (case writer-bytes (hex-dumper-sink done) of Some out -> equal out (hex-dump-bytes 0 (bytes-from-u8s [65])); None -> False); _ -> False
      _ -> False

  -- | A generic encoding sink composes with bulk stream-copy and counts input bytes.
  generic-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  generic-copy unit = case stream-copy 3 (reader-from-bytes (bytes-new 17)) (hex-encoding-writer writer-discard) of (Ok count, pair) -> count == 17; _ -> False

  -- | Prepare changing byte values before the small-read lifetime test.
  fill :: Bytes -> Nat -> Unit
  fill input i = if i < bytes-len input then let stored = bytes-set input i (nat-rem i 256) in fill input (i + 1) else ()

  -- | Check one-byte results directly, avoiding a quadratic test-side concatenation.
  check-small :: forall r. using (io :: Io r) => ByteReader r -> Bytes -> Nat ->{IO r} Bool
  check-small source expected i = case stream-read source 1 of
    (Err error, next) -> False
    (Ok out, next) -> if bytes-len out == 0 then i == bytes-len expected
      else if i < bytes-len expected && 0 < bytes-len out then bytes-get out 0 == bytes-get expected i && check-small next expected (i + 1)
      else False

  -- | Exercise many decoder cursors under a heap bound with independently retained expected bytes.
  large-small-reads :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  large-small-reads unit = let input = bytes-new 65536 in
    let filled = fill input 0 in check-small (hex-reader (reader-from-bytes (hex-encode-bytes input))) input 0
"#;
