//! Compiled ASCII85 consumers: independent vectors, all chunk splits and malformed input.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Build a consumer whose exit code identifies the first failed fixture.
fn run(name: &str, checks: Vec<String>) {
    let mut source = String::from(HELPERS);
    for (index, check) in checks.iter().enumerate() {
        source.push_str(&format!(
            "\n  step{index} :: Int\n  step{index} = if ({check}) then step{} else {}\n",
            index + 1,
            index + 1
        ));
    }
    source.push_str(&format!(
        "\n  step{} :: Int\n  step{} = 0\n  main :: Int\n  main = step0\n",
        checks.len(),
        checks.len()
    ));
    execute(name, &source);
}

/// Compile actual modules and run with ownership validation, poisoning and a bounded heap.
fn execute(name: &str, source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (module, source) in [
        (
            "Text.Ascii85",
            include_str!("../prelude/Text/Ascii85.cinch"),
        ),
        (
            "Effect.Ascii85",
            include_str!("../prelude/Effect/Ascii85.cinch"),
        ),
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
        std::fs::write(directory.path().join(format!("{module}.cinch")), source).unwrap();
    }
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
    let result = std::process::Command::new(&executable)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .unwrap();
    let code = result.status.code().expect("normal termination");
    assert_eq!(
        code,
        0,
        "{name}: check {code}:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
    let ir = std::fs::read_to_string(executable.with_extension("ll")).unwrap();
    assert!(
        ir.contains("mul <4 x i32>"),
        "decoder SIMD arithmetic missing"
    );
    assert!(
        ir.contains("udiv <4 x i32>"),
        "encoder SIMD arithmetic missing"
    );
}

/// Runtime helpers keep split testing out of the compiler's constant-folding path.
const HELPERS: &str = r#"module Main where
  import Text.Ascii85
  import Text.Hex

  -- | Compare independent byte buffers.
  equal :: Bytes -> Bytes -> Bool
  equal a b = eq-i64 (bytes-compare a b) 0

  -- | Snapshot a proved input interval for independently owned chunk feeds.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start end =
    if start <= end && end <= bytes-len b then
      let out = bytes-new (end - start) in
      let copied = bytes-copy-range out b 0 start (end - start) in out
    else bytes-new 0

  -- | Join two complete output chunks.
  append :: Bytes -> Bytes -> Bytes
  append a b =
    let n = bytes-len a in
    let out = bytes-new (n + bytes-len b) in
    let first = bytes-copy-range out a 0 0 n in
    let last = bytes-copy-range out b n 0 (bytes-len b) in out

  -- | Check every encoding split, including empty feeds between live continuations.
  encode-splits :: Bytes -> Bytes -> Nat -> Bool
  encode-splits input expected split =
    if split > bytes-len input then True else
    case ascii85-encoder-feed ascii85-encoder (part input 0 split) of
      (first, state) -> case ascii85-encoder-feed state (bytes-new 0) of
        (empty, same) -> case ascii85-encoder-feed same (part input split (bytes-len input)) of
          (last, final) -> bytes-len empty == 0 &&
            equal (append first (append last (ascii85-encoder-finish final))) expected &&
            encode-splits input expected (split + 1)

  -- | Match the absolute invalid byte or end position returned by either decoder stage.
  error-at :: Bytes -> Nat -> Ascii85Error -> Bool
  error-at input expected error = case error of
    InvalidAscii85Byte offset byte ->
      if offset < bytes-len input then offset == expected && byte == bytes-get input offset else False
    TruncatedAscii85Quantum offset -> offset == expected && offset == bytes-len input

  -- | Validate one successful or failed completion against an independent fixture.
  finish-is :: Bytes -> Bytes -> Ascii85Decoder -> Bytes -> Bool -> Nat -> Bool
  finish-is input prefix state expected valid offset = case ascii85-decoder-finish state of
    Err error -> not valid && error-at input offset error
    Ok tail -> valid && equal (append prefix tail) expected

  -- | Decode at one chosen split and check a successful completion or absolute error.
  decode-split :: Bytes -> Bytes -> Bool -> Nat -> Nat -> Bool
  decode-split input expected valid offset split = case ascii85-decoder-feed ascii85-decoder (part input 0 split) of
    Err error -> not valid && error-at input offset error
    Ok pair -> case pair of
      (first, state) -> case ascii85-decoder-feed state (part input split (bytes-len input)) of
        Err error -> not valid && error-at input offset error
        Ok next -> case next of
          (last, final) -> finish-is input (append first last) final expected valid offset

  -- | Check every decoding split and the original input's absolute error position.
  decode-splits :: Bytes -> Bytes -> Bool -> Nat -> Nat -> Bool
  decode-splits input expected valid offset split =
    if split > bytes-len input then True else
    decode-split input expected valid offset split && decode-splits input expected valid offset (split + 1)

  -- | Compare a complete decode against its expected bytes or error offset.
  decoded-is :: Bytes -> Bytes -> Bool -> Nat -> Bool
  decoded-is input expected valid offset = case ascii85-decode-bytes input of
    Err error -> not valid && error-at input offset error
    Ok actual -> valid && equal actual expected

  -- | Decode the fixture's compact hexadecimal representation.
  vector :: String -> String -> Bool
  vector hex encoded = case hex-decode hex of
    Err _ -> False
    Ok input -> ascii85-encode input == encoded &&
      encode-splits input (string-to-bytes encoded) 0 &&
      decode-splits (string-to-bytes encoded) input True 0 0

  -- | Compare valid and malformed byte-level decoding samples with all split boundaries.
  decode-vector :: String -> String -> Bool -> Nat -> Bool
  decode-vector encoded decoded valid offset = case hex-decode encoded of
    Err _ -> False
    Ok input -> case hex-decode decoded of
      Err _ -> False
      Ok expected -> decoded-is input expected valid offset && decode-splits input expected valid offset 0
"#;

/// Reference encodings and decoding errors also agree across every two-chunk boundary.
#[test]
fn ascii85_matches_independent_vectors_and_all_chunk_splits() {
    let vectors: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in vectors["buffers"].as_array().unwrap() {
        checks.push(format!(
            "vector {:?} {:?}",
            row["hex"].as_str().unwrap(),
            row["ascii85"].as_str().unwrap()
        ));
    }
    for row in vectors["ascii85_decodes"].as_array().unwrap() {
        let offset = row["error_offset"].as_str().unwrap();
        checks.push(format!(
            "decode-vector {:?} {:?} {} {}",
            row["encoded_hex"].as_str().unwrap(),
            row["decoded_hex"].as_str().unwrap(),
            if offset.is_empty() { "True" } else { "False" },
            if offset.is_empty() { "0" } else { offset }
        ));
    }
    checks.extend([
        "vector \"00\" \"!!\"".into(),
        "vector \"0000\" \"!!!\"".into(),
        "vector \"000000\" \"!!!!\"".into(),
        "vector \"00000000\" \"z\"".into(),
        "vector \"0000000000\" \"z!!\"".into(),
        "vector \"0000000000000000\" \"zz\"".into(),
        "ascii85-max-encoded-length 0 == 0 && ascii85-max-encoded-length 1 == 5 && ascii85-max-encoded-length 5 == 10".into(),
    ]);
    run("ascii85-vectors-and-splits", checks);
}

/// Every declaration, including private kernels and continuation constructors, has canonical docs.
#[test]
fn ascii85_declarations_have_canonical_documentation() {
    for (module, source) in [
        (
            "Text.Ascii85",
            include_str!("../prelude/Text/Ascii85.cinch"),
        ),
        (
            "Effect.Ascii85",
            include_str!("../prelude/Effect/Ascii85.cinch"),
        ),
    ] {
        let parsed =
            cinch::cinchdoc::parse_source(source, Some(module), module, None, false).unwrap();
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
            assert!(
                !text.contains("golang") && !text.contains("go's") && !text.contains("go "),
                "{}",
                declaration.id
            );
        }
    }
}

/// Actual SIMD kernels compile on both little-endian ARM and big-endian PowerPC.
#[test]
fn ascii85_simd_kernels_compile_for_arm_and_big_endian() {
    let source = include_str!("../prelude/Text/Ascii85.cinch");
    let mut consumer = String::from("module Main where\n  import Data.Machine\n");
    for name in ["encode-word", "decode-word", "reverse-word"] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let tail = &source[start..];
        let end = tail.find("\n    }\n").unwrap() + 7;
        consumer.push_str(&tail[..end]);
    }
    consumer.push_str("  main :: Int\n  main = if eq-u64 (encode-word 0) 0x2121212121 && eq-u64 (decode-word 0x2121212121) 0 && eq-u32 (reverse-word 0x01020304) 0x04030201 then 0 else 1\n");
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let mut profile = cinch::core::TargetProfile::native();
        profile.triple = triple.into();
        profile.arch = arch.into();
        profile.endian = endian.into();
        let source = consumer.clone();
        let (profile, program) = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::check_sources_profile(&source, "kernels.cinch", &profile)
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

/// Both frontends prefer native-endian bodies independently of fallback order.
/// The target byte-order observation itself uses only safe vector instructions.
#[test]
fn native_endian_alternatives_precede_defaults_in_either_order() {
    let source = r#"module Main where
  import Data.Machine
  -- | Observe the native target without touching memory.
  prim first-byte :: U32 -> U8 where
    llvm {
      %bytes = bitcast i32 %0 to <4 x i8>
      %first = extractelement <4 x i8> %bytes, i32 0
      ret i8 %first
    }
  -- | Select the specific body even with a later generic fallback.
  prim later-fallback :: U32 -> U32 where
    llvm {
      %out = add i32 %0, 1
      ret i32 %out
    } when endian little
    llvm {
      %out = add i32 %0, 2
      ret i32 %out
    } when endian big
    llvm {
      %out = add i32 %0, 99
      ret i32 %out
    }
  -- | Select the specific body even with an earlier generic fallback.
  prim earlier-fallback :: U32 -> U32 where
    llvm {
      %out = add i32 %0, 99
      ret i32 %out
    }
    llvm {
      %out = add i32 %0, 1
      ret i32 %out
    } when endian little
    llvm {
      %out = add i32 %0, 2
      ret i32 %out
    } when endian big
  -- | Compare selection with the actual executable's native byte order.
  main :: Int
  main =
    let expected = if eq-u8 (first-byte 1) 1 then (41 :: U32) else (42 :: U32) in
    if eq-u32 (later-fallback 40) expected && eq-u32 (earlier-fallback 40) expected then 0 else 1
"#;
    assert_eq!(common::source_frontend::run_source(source).unwrap(), 0);
}

/// Explicit trust cannot conceal visible memory effects from a declared pure row.
#[test]
fn inline_containment_still_rejects_visible_effects_in_pure_rows() {
    for body in [
        "llvm {\n      %pointer = inttoptr i64 %0 to ptr\n      %word = load volatile i64, ptr %pointer, align 1\n      ret i64 %word\n    }",
        "unsafe llvm {\n      %pointer = inttoptr i64 %0 to ptr\n      %word = load volatile i64, ptr %pointer, align 1\n      ret i64 %word\n    }",
        "unsafe llvm {\n      %pointer = inttoptr i64 %0 to ptr\n      store i64 0, ptr %pointer, align 1\n      ret i64 %0\n    }",
    ] {
        let source = format!("module Main where\n  prim invalid :: I64 -> I64 where\n    {body}\n  main :: Int\n  main = 0\n");
        let error = common::source_frontend::check_sources(&source, "containment.cinch").unwrap_err();
        assert!(error.contains("containment"), "{error}");
    }
}

/// Stream adapters preserve partial words, short writes, pending output snapshots,
/// original error offsets and terminal failure state under ownership validation.
#[test]
fn ascii85_stream_adapters_preserve_chunk_and_failure_contracts() {
    let mut source = HELPERS.replace(
        "  import Text.Hex\n",
        "  import Text.Hex\n  import Effect.Ascii85\n  import Effect.Stream\n",
    );
    source.push_str(r#"
  -- | Force single-byte input chunks without changing the source's zero-read contract.
  one-at-a-time :: forall r. using (io :: Io r) => ByteReader r -> ByteReader r
  one-at-a-time source = stream-reader (\n -> case stream-read source 1 of
    (result, next) -> (result, one-at-a-time next))

  -- | Accept just one output byte once, then expose the advanced normal sink.
  short-once :: forall r. using (io :: Io r) => ByteWriter r -> ByteWriter r
  short-once sink = stream-writer (\input ->
    if bytes-len input > 0 then stream-write sink (part input 0 1) else (Ok 0, sink))

  -- | Compare encoded output after every split of a raw input buffer.
  writer-splits :: forall r. using (io :: Io r) => Bytes -> String -> Nat ->{IO r} Bool
  writer-splits input expected split =
    if split > bytes-len input then True else
    case ascii85-write (ascii85-writer (short-once writer-buffer)) (part input 0 split) of
      (Err _, _) -> False
      (Ok first, next) ->
        if first /= split then False else
        case ascii85-write next (part input split (bytes-len input)) of
          (Err _, _) -> False
          (Ok last, final) -> case ascii85-finish final of
            (Err _, _) -> False
            (Ok _, sink) -> case writer-bytes sink of
              None -> False
              Some out -> if equal out (string-to-bytes expected) then writer-splits input expected (split + 1) else False

  -- | A failed encoding never retries its underlying sink, even if that sink can now write.
  sticky-write :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  sticky-write unit =
    let stop = stream-writer (\input -> (Ok 0, writer-buffer)) in
    case ascii85-write (ascii85-writer stop) (string-to-bytes "abcd") of
      (Err (StreamShortWrite expected got), failed) ->
        if expected /= 5 || got /= 0 then False else
        case ascii85-write failed (string-to-bytes "more") of
          (Err _, same) -> case ascii85-finish same of
            (Err _, sink) -> case writer-bytes sink of
              Some out -> bytes-len out == 0
              None -> False
            _ -> False
          _ -> False
      _ -> False

  -- | Pending input is copied into the word state before the caller mutates its bytes.
  pending-input-snapshot :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  pending-input-snapshot unit =
    let input = string-to-bytes "abc" in
    case ascii85-write (ascii85-writer writer-buffer) input of
      (Err _, _) -> False
      (Ok count, writer) ->
        let changed = if bytes-len input > 0 then bytes-set input 0 88 else () in
        case ascii85-finish writer of
          (Ok _, sink) -> case writer-bytes sink of
            Some encoded -> equal encoded (string-to-bytes "@:E^")
            None -> False
          _ -> False

  -- | A returned decoded chunk cannot mutate retained surplus, even with one-byte source reads.
  pending-output-snapshot :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  pending-output-snapshot unit =
    let source = ascii85-reader (one-at-a-time (reader-from-string " \t z\r\nBOu!rDZ\n")) in
    case stream-read source 0 of
      (Ok empty, same) -> if bytes-len empty /= 0 then False else
        case stream-read same 1 of
          (Ok first, next) -> if bytes-len first /= 1 then False else
            let changed = if bytes-len first > 0 then bytes-set first 0 99 else () in
            case stream-read-all 2 next of
              (Ok rest, _) -> case hex-decode "00000068656c6c6f" of
                Ok expected -> equal rest expected
                Err _ -> False
              _ -> False
          _ -> False
      _ -> False

  -- | Errors retain original input offsets and remain sticky after zero-length requests.
  sticky-read :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  sticky-read unit =
    let source = ascii85-reader (one-at-a-time (reader-from-string "!!!!!v")) in
    case stream-read source 4 of
      (Ok first, next) -> if bytes-len first /= 4 then False else
        case stream-read next 1 of
          (Err (StreamEncodingError format offset message), failed) ->
            if format /= "ascii85" || offset /= 5 || message /= "invalid ASCII85 byte 118 at byte 5" then False else
            case stream-read failed 0 of
              (Ok empty, same) -> case stream-read same 9 of
                (Err (StreamEncodingError name position detail), _) -> name == format && position == offset && detail == message
                _ -> False
              _ -> False
          _ -> False
      _ -> False

  -- | Whitespace-only input ends normally; an unfinished symbol fails only at genuine EOF.
  decoder-endings :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  decoder-endings unit = case stream-read-all 4 (ascii85-reader (reader-from-string " \t\n\r")) of
    (Ok empty, _) -> if bytes-len empty /= 0 then False else
      case stream-read-all 4 (ascii85-reader (reader-from-string "!")) of
        (Err (StreamEncodingError format offset message), _) -> format == "ascii85" && offset == 1 && message == "incomplete ASCII85 quantum at byte 1"
        _ -> False
    _ -> False

  -- | Exercise adapters against independent text and all continuation contracts.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    if not (writer-splits (string-to-bytes "hello") "BOu!rDZ" 0) then 1
    else if not (pending-input-snapshot ()) then 2
    else if not (pending-output-snapshot ()) then 3
    else if not (sticky-write ()) then 4
    else if not (sticky-read ()) then 5
    else if not (decoder-endings ()) then 6
    else 0
"#);
    execute("ascii85-stream-contracts", &source);
}
