//! Independent SHA-224/SHA-256 states, persistent checkpoints and hashing streams.
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
        ("Text.Sha256", include_str!("../prelude/Text/Sha256.cinch")),
        (
            "Effect.Sha256",
            include_str!("../prelude/Effect/Sha256.cinch"),
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
    serde_json::from_str(include_str!("fixtures/sha256-state-vectors.json")).unwrap()
}

/// Whole-buffer variants, padding blocks, checkpoints and every requested split match independent bytes.
#[test]
fn sha256_variants_match_independent_digest_checkpoint_and_split_vectors() {
    let f = fixture();
    let checks = f["valid"]
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
                flag(row, "short"),
                row["size"],
                flag(row, "fill"),
                row["digest"].as_str().unwrap(),
                row["saved"].as_str().unwrap(),
                row["mid"],
                row["checkpoint"].as_str().unwrap(),
                row["append_sum"].as_str().unwrap(),
                row["append_saved"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(11) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Arbitrary chaining words, unused tail bytes and wrapping byte counts retain complete state semantics.
#[test]
fn sha256_restored_states_match_reference_canonicalization_and_wrapped_counts() {
    let f = fixture();
    let checks = f["arbitrary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "arbitrary-case {} {} {:?} {:?} {:?} {:?} {:?}",
                flag(row, "short"),
                row["size"],
                row["state"].as_str().unwrap(),
                row["canonical"].as_str().unwrap(),
                row["before"].as_str().unwrap(),
                row["digest"].as_str().unwrap(),
                row["saved"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(20) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Both variants reject another identifier before checking the exact checkpoint size.
#[test]
fn sha256_checkpoint_errors_match_independent_variant_and_size_checks() {
    let f = fixture();
    let checks = f["loads"]
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
                "load-case {} {:?} {kind} {:?}",
                flag(row, "short"),
                row["input"].as_str().unwrap(),
                row["canonical"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(80) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Retained nonzero states, mutable inputs, digest outputs and checkpoint sources remain isolated.
#[test]
fn sha256_state_buffers_and_retained_branches_are_independent() {
    run("aliases False && aliases True && sha256-state-error-message InvalidSha256StateIdentifier == \"SHA-2 state identifier is invalid\" && sha256-state-error-message (InvalidSha256StateSize 108 107) == \"SHA-2 state requires 108 bytes; received 107\"");
}

/// Explicit and generic writes and bounded copies retain advanced source handles across errors.
#[test]
fn sha256_streams_preserve_digests_counts_and_source_continuations() {
    run("stream-cases False () && stream-cases True () && failed-copy () && invalid-copy () && zero-copy ()");
}

/// A full input delivered one byte at a time preserves bounded live memory and stack usage.
#[test]
fn sha256_single_byte_stream_copy_is_bounded_and_releases_buffers() {
    for short in [false, true] {
        run(&format!("case sha256-copy 1 (reader-from-bytes (bytes-new 65536)) (start {}) of (Ok count, (source, state)) -> count == 65536 && same (sha256-state-sum state) (sum {} (bytes-new 65536)); other -> False",if short{"True"}else{"False"},if short{"True"}else{"False"}));
    }
}

/// O2 must keep ephemeral input and padding owners live through the last private raw block read.
#[test]
fn rust_sha256_optimized_raw_buffer_owners_survive_calls_and_tail_loops() {
    let f = fixture();
    let digest = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["short"] == false && row["size"] == 65536 && row["fill"] == false)
        .unwrap()["digest"]
        .as_str()
        .unwrap();
    run_with_optimization(
        &format!("lifetime 100 {:?} && aliases False && aliases True", digest),
        true,
    );
}

/// The SHA-256 SIMD schedule, rotate and byte-order kernels remain legal on 32-bit and non-x86 object targets.
/// This explicitly tests Rust's cross-target emitter, independently of selected host execution.
#[test]
fn sha256_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
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
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Text.Sha256", include_str!("../prelude/Text/Sha256.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "module Main where\n  import Text.Sha256\n  -- | Reach the kernels.\n  main :: Int\n  main = if bytes-len (sha256-sum (bytes-new 64)) == 32 then 0 else 1\n",
        )
        .unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::EmitIr,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&ir_path),
                    &profile,
                    &cinch::driver::DriverOptions::default(),
                )
                .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<4 x i32>") && ir.contains("call <4 x i32> @llvm.fshr.v4i32"));
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(project.join("program.ll"))
        .arg("-o")
        .arg(project.join("program.o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Render fixture booleans without relying on truthy integer coercion in the checked language.
fn flag(row: &Value, name: &str) -> &'static str {
    if row[name].as_bool().unwrap() {
        "True"
    } else {
        "False"
    }
}

/// New public and private declarations have canonical module-owned documentation.
#[test]
fn sha256_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Sha256", include_str!("../prelude/Text/Sha256.cinch")),
        (
            "Effect.Sha256",
            include_str!("../prelude/Effect/Sha256.cinch"),
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
const HELPERS: &str = r#"-- | Independent SHA-224/SHA-256 checkpoint and stream fixture consumers.
module Main where
  import Text.Sha256
  import Text.Binary
  import Text.Hex
  import Effect.Sha256
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
  -- | Select a fresh initial state by its explicit digest variant.
  start :: Bool -> Sha256State
  start short = if short then sha224-state else sha256-state
  -- | Exercise either whole-buffer entry point with the same independent corpus.
  sum :: Bool -> Bytes -> Bytes
  sum short bytes = if short then sha224-sum bytes else sha256-sum bytes
  -- | Resume every requested split from an independently saved partial prefix.
  splits :: Bool -> Bytes -> String -> List Nat -> Bool
  splits short raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = sha256-state-update (start short) (part raw 0 point) in
      let saved = sha256-state-save prefix in
      case sha256-state-load (start short) saved of
        Err error -> False
        Ok restored -> same (sha256-state-sum (sha256-state-update restored (part raw point (bytes-len raw - point)))) (input expected)
          && same saved (sha256-state-save prefix) && splits short raw expected rest
      else False
  -- | Compare each public digest, checkpoint and prefix append with independent complete bytes.
  valid-case :: Bool -> Nat -> Bool -> String -> String -> Nat -> String -> String -> String -> List Nat -> Bool
  valid-case short n fill expected saved mid checkpoint appended appended-save points =
    let raw = data-bytes n fill in let initial = start short in let state = sha256-state-update initial raw in
    same (sum short raw) (input expected) && same (sha256-state-sum state) (input expected)
      && sha256-size == 32 && sha224-size == 28 && sha256-block-size == 64
      && sha256-state-size state == (if short then 28 else 32) && sha256-state-block-size state == 64
      && same (sha256-state-sum (sha256-state-reset state)) (sum short (bytes-new 0))
      && same (sha256-state-save state) (input saved)
      && same (sha256-state-save (sha256-state-update initial (part raw 0 mid))) (input checkpoint)
      && same (sha256-state-append (input "aabbcc") state) (input appended)
      && same (sha256-state-append-save (input "aabbcc") state) (input appended-save)
      && splits short raw expected points
  -- | Restore arbitrary accepted bits, canonicalize unused buffer bytes and continue across count wrapping.
  arbitrary-case :: Bool -> Nat -> String -> String -> String -> String -> String -> Bool
  arbitrary-case short n saved canonical before expected after = case sha256-state-load (start short) (input saved) of
    Err error -> False
    Ok state -> same (sha256-state-save state) (input canonical) && same (sha256-state-sum state) (input before)
      && (let next = sha256-state-update state (data-bytes n False) in same (sha256-state-sum next) (input expected)
        && same (sha256-state-save next) (input after) && same (sha256-state-save state) (input canonical))
  -- | Compare checkpoint classification and complete size payloads without mutating the template.
  load-case :: Bool -> String -> Nat -> String -> Bool
  load-case short text kind canonical = let b = input text in case sha256-state-load (start short) b of
    Ok state -> kind == 0 && same (sha256-state-save state) (input canonical)
    Err InvalidSha256StateIdentifier -> kind == 1
    Err (InvalidSha256StateSize expected actual) -> kind == 2 && expected == 108 && actual == bytes-len b
  -- | Branch from nonzero chaining words and a live tail while preserving all earlier states and bytes.
  aliases :: Bool -> Bool
  aliases short = let raw = data-bytes 129 False in let retained = part raw 0 (bytes-len raw) in
    let state = sha256-state-update (start short) raw in
    let first = sha256-state-update state (string-to-bytes "a") in
    let last = sha256-state-update state (string-to-bytes "b") in
    let saved = sha256-state-save state in let observed = sha256-state-sum state in
    let prefix = input "aabbcc" in let appended = sha256-state-append prefix state in
    let changed-prefix = if 0 < bytes-len prefix then bytes-set prefix 0 0 else () in
    let changed-input = if 0 < bytes-len raw then bytes-set raw 0 0 else () in
    let changed-sum = if 0 < bytes-len observed then bytes-set observed 0 0 else () in
    case sha256-state-load (start short) saved of
      Err error -> False
      Ok loaded -> let changed-save = if 36 < bytes-len saved then bytes-set saved 36 0 else () in
        same (sha256-state-sum loaded) (sha256-state-sum state)
          && same (sha256-state-sum state) (sum short retained)
          && same (sha256-state-sum first) (sum short (joined retained (string-to-bytes "a")))
          && same (sha256-state-sum last) (sum short (joined retained (string-to-bytes "b")))
          && same appended (sha256-state-append (input "aabbcc") state)
          && same (sha256-state-save (start short)) (sha256-state-save (sha256-state-reset state))
  -- | Join source intervals for independent one-shot branching observations.
  joined :: Bytes -> Bytes -> Bytes
  joined a b = let n = bytes-len a in let out = bytes-new (n + bytes-len b) in
    let first = bytes-copy-range out a 0 0 n in let last = bytes-copy-range out b n 0 (bytes-len b) in out
  -- | Rebuild and release input and padding owners on every optimized traversal.
  lifetime :: Nat -> String -> Bool
  lifetime count expected = if count == 0 then True else
    same (sha256-sum (data-bytes 65536 False)) (input expected) && lifetime (count - 1) expected
  -- | Explicit and generic writes retain counts, previous states and both digest variants.
  stream-cases :: forall r. using (io :: Io r) => Bool -> Unit ->{IO r} Bool
  stream-cases short unit = let initial = start short in case sha256-write initial (string-to-bytes "hello") of
    (n, next) -> n == 5 && same (sha256-state-sum next) (sum short (string-to-bytes "hello"))
      && same (sha256-state-sum initial) (sum short (bytes-new 0))
      && (case stream-copy 3 (reader-from-string "abcdefg") (sha256-hashing-writer next) of (Ok count, (source, sink)) -> count == 7; other -> False)
      && (case sha256-copy 3 (reader-from-string "abcdefg") initial of (Ok count, (source, state)) -> count == 7 && same (sha256-state-sum state) (sum short (string-to-bytes "abcdefg")); other -> False)
      && (case sha256-copy 3 (reader-from-string "") next of (Ok count, (source, state)) -> count == 0 && same (sha256-state-sum state) (sha256-state-sum next); other -> False)
  -- | Emit a successful prefix and an error carrying an independently readable source continuation.
  failing :: forall r. using (io :: Io r) => ByteReader r
  failing = stream-reader (\n -> (Ok (string-to-bytes "ab"), stream-reader (\m -> (Err (StreamShortWrite 7 3), reader-from-string "z"))))
  -- | Failed copies preserve the successful prefix digest and the advanced source supplied with the error.
  failed-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  failed-copy unit = case sha256-copy 2 failing sha224-state of
    (Err (StreamShortWrite required accepted), (source, state)) -> required == 7 && accepted == 3 && same (sha256-state-sum state) (sha224-sum (string-to-bytes "ab"))
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "z"); other -> False)
    other -> False
  -- | Invalid counts never enter the digest, and the returned source remains observable.
  invalid-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  invalid-copy unit = let source = stream-reader (\n -> (Ok (string-to-bytes "bad"), reader-from-string "x")) in
    case sha256-copy 1 source sha256-state of
      (Err (StreamInvalidCount requested actual), (next, state)) -> requested == 1 && actual == 3 && same (sha256-state-sum state) (sha256-sum (bytes-new 0))
        && (case stream-read next 1 of (Ok b, after) -> same b (string-to-bytes "x"); other -> False)
      other -> False
  -- | A zero chunk rejects before consuming source bytes and retains the supplied state.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case sha256-copy 0 (reader-from-string "a") sha224-state of
    (Err StreamInvalidChunkSize, (source, state)) -> same (sha256-state-sum state) (sha224-sum (bytes-new 0))
      && (case stream-read source 1 of (Ok b, next) -> same b (string-to-bytes "a"); other -> False)
    other -> False
"#;
