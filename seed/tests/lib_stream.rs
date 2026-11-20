//! Byte-stream state, bounds, errors, composition and real descriptor consumers.
mod common;

use cinch::driver::Mode;

/// Compile a consumer on the generous driver stack and execute its I/O entry.
fn run(name: &str, body: &str) {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Effect.Stream.cinch"),
        include_str!("../prelude/Effect/Stream.cinch"),
    )
    .unwrap();
    let source = format!("module Main where\n  import Effect.Stream\n  import Prim.Sys\n{body}\n");
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .unwrap_or_else(|error| panic!("{name}: {error}"));
    let result = std::process::Command::new(executable)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .unwrap();
    assert_eq!(
        result.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("leaked=0"),
        "{name}: missing leak evidence"
    );
}

/// A reader snapshots its input, advances explicitly, handles zero reads, and
/// returns its consumed continuation after an unsuccessful exact read.
#[test]
fn memory_readers_preserve_snapshot_cursor_and_eof_contracts() {
    run(
        "memory",
        r#"
  equals :: Bytes -> String -> Bool
  equals b text = eq-i64 (bytes-compare b (string-to-bytes text)) 0

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let input = string-to-bytes "abc" in
    let reader = reader-from-bytes input in
    let changed = if 0 < bytes-len input then bytes-set input 0 88 else () in
    case stream-read reader 0 of
      (Err _, _) -> 1
      (Ok empty, start) ->
        if bytes-len empty /= 0 then 2 else
        case stream-read start 2 of
          (Err _, _) -> 3
          (Ok first, after) -> if not (equals first "ab") then 4 else
            case stream-read-exact after 3 of
              (Err (StreamUnexpectedEnd required got), consumed) ->
                if required /= 3 || got /= 1 then 5 else
                case stream-read consumed 1 of
                  (Ok last, _) -> if bytes-len last == 0 then 0 else 6
                  _ -> 7
              _ -> 8
"#,
    );
}

/// Partial reads, concatenation, minimum reads and copy limits retain unread data.
#[test]
fn composed_readers_and_bounded_copy_keep_the_remaining_input() {
    run(
        "composition",
        r#"
  equals :: Bytes -> String -> Bool
  equals b text = eq-i64 (bytes-compare b (string-to-bytes text)) 0

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let joined = reader-concat (Cons (reader-from-string "") (Cons (reader-from-string "ab") (Cons (reader-from-string "cdef") Nil))) in
    case stream-copy-n 2 3 joined writer-buffer of
      (Ok count, (after, sink)) ->
        if count /= 3 then 1 else
        case writer-bytes sink of
          None -> 2
          Some prefix -> if not (equals prefix "abc") then 3 else
            case stream-read-all 2 after of
              (Ok suffix, _) -> if not (equals suffix "def") then 4 else
                case stream-read-all 2 (reader-limit 3 (reader-from-string "hello")) of
                  (Ok limited, _) -> if not (equals limited "hel") then 5 else
                    case stream-read-at-least (reader-from-string "abcde") 2 4 of
                      (Ok minimum, _) -> if equals minimum "abcd" then 0 else 6
                      _ -> 7
                  _ -> 8
              _ -> 9
      _ -> 10
"#,
    );
}

/// Memory writers snapshot input and output; short writes retry the exact suffix.
#[test]
fn writers_are_persistent_and_short_writes_advance_the_suffix() {
    run(
        "writers",
        r#"
  equals :: Bytes -> String -> Bool
  equals b text = eq-i64 (bytes-compare b (string-to-bytes text)) 0

  take-two :: forall r. using (io :: Io r) => Bytes ->{IO r} (Result StreamError Nat, ByteWriter r)
  take-two b = (Ok 2, writer-buffer)

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let input = string-to-bytes "abc" in
    let original = writer-buffer in
    case stream-write original input of
      (Ok n, advanced) ->
        let changed = if 0 < bytes-len input then bytes-set input 0 88 else () in
        if n /= 3 then 1 else
        case writer-bytes original of
          Some old -> if bytes-len old /= 0 then 2 else
            case writer-bytes advanced of
              Some saved -> if not (equals saved "abc") then 3 else
                let mutated = if 0 < bytes-len saved then bytes-set saved 0 89 else () in
                case writer-bytes advanced of
                  Some again -> if not (equals again "abc") then 4 else
                    case stream-write-all (stream-writer take-two) (string-to-bytes "abcdef") of
                      (Ok count, last) -> if count /= 6 then 5 else
                        case writer-bytes last of
                          Some tail -> if equals tail "cdef" then 0 else 6
                          None -> 7
                      _ -> 8
                  None -> 9
              None -> 10
          None -> 11
      _ -> 12
"#,
    );
}

/// Invalid callback counts, zero progress and invalid chunk sizes are named errors.
#[test]
fn invalid_callback_protocols_are_rejected_without_infinite_retries() {
    run(
        "protocol-errors",
        r#"
  bad-read :: forall r. using (io :: Io r) => Nat ->{IO r} (Result StreamError Bytes, ByteReader r)
  bad-read count = (Ok (bytes-new (count + 1)), reader-from-string "")
  bad-write :: forall r. Bytes ->{IO r} (Result StreamError Nat, ByteWriter r)
  bad-write b = (Ok (bytes-len b + 1), writer-discard)
  stopped :: forall r. Bytes ->{IO r} (Result StreamError Nat, ByteWriter r)
  stopped b = (Ok 0, writer-discard)

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    case stream-read (stream-reader bad-read) 2 of
      (Err (StreamInvalidCount wanted actual), _) ->
        if wanted /= 2 || actual /= 3 then 1 else
        case stream-write (stream-writer bad-write) (string-to-bytes "abc") of
          (Err (StreamInvalidCount _ _), _) ->
            case stream-write-all (stream-writer stopped) (string-to-bytes "abc") of
              (Err (StreamShortWrite expected got), _) ->
                if expected /= 3 || got /= 0 then 2 else
                case stream-copy 0 (reader-from-string "abc") writer-discard of
                  (Err StreamInvalidChunkSize, _) ->
                    case stream-read-at-least (reader-from-string "abc") 4 3 of
                      (Err (StreamInvalidCount _ _), _) -> 0
                      _ -> 3
                  _ -> 4
              _ -> 5
          _ -> 6
      _ -> 7
"#,
    );
}

/// Tee and broadcast composition expose their advanced persistent memory sinks.
#[test]
fn tee_and_broadcast_return_observable_updated_children() {
    run(
        "tee-broadcast",
        r#"
  saved-is :: forall r. ByteWriter r -> String -> Bool
  saved-is sink expected = case writer-bytes sink of
    None -> False
    Some b -> eq-i64 (bytes-compare b (string-to-bytes expected)) 0

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let mirrors = writer-concat (Cons writer-buffer (Cons writer-buffer Nil)) in
    let tee = reader-tee (reader-from-string "abcdef") mirrors in
    case stream-copy 2 tee writer-discard of
      (Ok count, (after, _)) ->
        if count /= 6 then 1 else
        case reader-tee-writer after of
          None -> 2
          Some mirror -> case writer-components mirror of
            Some (Cons first (Cons second Nil)) ->
              if saved-is first "abcdef" && saved-is second "abcdef" then 0 else 3
            _ -> 4
      _ -> 5
"#,
    );
}

/// Borrowed descriptors work against actual files and remain caller-owned.
#[test]
fn descriptor_adapters_transfer_real_files_and_preserve_ownership() {
    let directory = tempfile::tempdir().unwrap();
    let path =
        serde_json::to_string(&directory.path().join("stream.bin").to_string_lossy()).unwrap();
    run(
        "descriptors",
        &format!(
            r#"
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in
    case sys-create {path} of
      Err _ -> 1
      Ok fd -> case stream-write-string (writer-from-fd fd) "streamed bytes" of
        (Err _, _) -> 2
        (Ok count, _) -> case sys-close fd of
          Err _ -> 3
          Ok _ -> case sys-open-read {path} of
            Err _ -> 4
            Ok input -> case stream-read-all 3 (reader-from-fd input) of
              (Err _, _) -> 5
              (Ok b, _) -> case sys-close input of
                Err _ -> 6
                Ok _ -> if count == 14 && eq-i64 (bytes-compare b (string-to-bytes "streamed bytes")) 0 then 0 else 7
"#
        ),
    );
}

/// All declarations, including protocol helpers, carry canonical documentation.
#[test]
fn stream_documentation_is_complete_and_well_formed() {
    let parsed = cinch::cinchdoc::parse_source(
        include_str!("../prelude/Effect/Stream.cinch"),
        Some("Effect.Stream"),
        "Effect.Stream.cinch",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert!(parsed.orphans.is_empty(), "{:?}", parsed.orphans);
    assert!(parsed.doc.is_some());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        assert!(
            declaration.doc.is_some(),
            "missing documentation for {}",
            declaration.id
        );
    }
}

/// Large one-byte/variable writes stay linear, preserve retained offers and return advanced failure sinks.
#[test]
fn write_all_uses_linear_fresh_windows_with_bounded_memory() {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
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
    std::fs::write(
        directory.path().join("Main.cinch"),
        include_str!("fixtures/stream-write-all-controls.cinch"),
    )
    .unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .unwrap();
    for poison in [false, true] {
        let result = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if poison { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if poison { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(0),
            "poison={poison}: stdout={} stderr={}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let diagnostic = String::from_utf8_lossy(&result.stderr);
        assert!(diagnostic.contains("live=0") && diagnostic.contains("leaked=0"));
    }
}
