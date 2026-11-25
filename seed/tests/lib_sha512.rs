//! Independent SHA-384/SHA-512/SHA-512/t digests, persistent checkpoints and hashing streams.
//! Fixtures come from `tooling/sha512_state_vectors.py` (hashlib digests and a
//! separate Python compression model for checkpoint bytes).
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Sha512", include_str!("../prelude/Text/Sha512.cinch")),
        ("Effect.Sha512", include_str!("../prelude/Effect/Sha512.cinch")),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Effect.Stream", include_str!("../prelude/Effect/Stream.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(
        directory.path().join("Main.cinch"),
        format!("{HELPERS}\n  -- | Observe the complete runtime fixture result.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if ({body}) then 0 else 1\n"),
    )
    .unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile { reclaim: true, rc_debug: true, ..cinch::core::TargetProfile::native() };
    common::drive_with_profile(Mode::Build, cinch::check::StageMode::Refresh, directory.path(), "Main.cinch", Some(exe.clone()), &profile).unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}: {}", String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Read fixed independent results without requiring any reference toolchain during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/sha512-state-vectors.json")).unwrap()
}

/// Render fixture booleans as checked-language constructors.
fn flag(row: &Value, name: &str) -> &'static str {
    if row[name].as_bool().unwrap() { "True" } else { "False" }
}

/// Whole-buffer variants, padding boundaries, checkpoints and every requested split match independent bytes.
#[test]
fn sha512_variants_match_independent_digest_checkpoint_and_split_vectors() {
    let f = fixture();
    let checks = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let points = row["splits"].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(",");
            format!(
                "valid-case {} {} {} {:?} {:?} {} {:?} [{points}]",
                row["variant"],
                row["size"],
                flag(row, "fill"),
                row["digest"].as_str().unwrap(),
                row["saved"].as_str().unwrap(),
                row["mid"],
                row["checkpoint"].as_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(19) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Every variant rejects another identifier before checking the exact checkpoint size.
#[test]
fn sha512_checkpoint_errors_match_independent_variant_and_size_checks() {
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
            format!("load-case {} {:?} {kind}", row["variant"], row["input"].as_str().unwrap())
        })
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Retained states, mutable inputs, digest outputs and error messages remain isolated and exact.
#[test]
fn sha512_state_buffers_and_retained_branches_are_independent() {
    run("aliases 7 && aliases 4 && aliases 5 && sha512-state-error-message InvalidSha512StateIdentifier == \"SHA-512 state identifier is invalid\" && sha512-state-error-message (InvalidSha512StateSize 204 203) == \"SHA-512 state requires 204 bytes; received 203\"");
}

/// Explicit and generic writes and bounded copies retain advanced source handles and reject zero chunks.
#[test]
fn sha512_streams_preserve_digests_counts_and_source_continuations() {
    run("stream-case 7 && stream-case 6 && zero-copy ()");
}

/// The SHA-512 SIMD schedule, rotate and byte-order kernels remain legal on 32-bit and non-x86 object targets.
#[test]
fn sha512_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
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
            ("Text.Sha512", include_str!("../prelude/Text/Sha512.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "module Main where\n  import Text.Sha512\n  -- | Reach the kernels.\n  main :: Int\n  main = if bytes-len (sha512-sum (bytes-new 128)) == 64 then 0 else 1\n",
        )
        .unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<2 x i64>") && ir.contains("call <2 x i64> @llvm.fshr.v2i64"));
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn sha512_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Sha512", include_str!("../prelude/Text/Sha512.cinch")),
        ("Effect.Sha512", include_str!("../prelude/Effect/Sha512.cinch")),
    ] {
        common::assert_documented(name, source);
    }
}

/// Checked consumers build input bytes from compact independent corpus descriptions.
const HELPERS: &str = r#"-- | Independent SHA-512 family checkpoint and stream fixture consumers.
module Main where
  import Text.Sha512
  import Text.Binary
  import Text.Hex
  import Effect.Sha512
  import Effect.Stream
  -- | Build deterministic input independently of digest calculations.
  data-bytes :: Nat -> Bool -> Bytes
  data-bytes n fill =
    if n <= 65536 then
      let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) in
      let b = bytes-new size in let done = fill-bytes b fill 0 in b
    else bytes-new 0
  -- | Keep constant corpus setup in the executable instead of the checker's byte-buffer evaluator.
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
  -- | Compare every byte.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Join two buffers in fresh storage.
  concat :: Bytes -> Bytes -> Bytes
  concat a b = let out = bytes-new (bytes-len a + bytes-len b) in
    let x = bytes-copy-range out a 0 0 (bytes-len a) in let y = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in out
  -- | Overwrite one byte when it exists, modelling a caller mutating a buffer it still owns.
  poke :: Bytes -> Nat -> Nat -> Unit
  poke b i v = if i < bytes-len b && v < 256 then bytes-set b i v else ()
  -- | Conjoin bounded runtime fixture batches.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Copy an exactly checked source interval for independent update calls.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start n = if start + n <= bytes-len b then
    let out = bytes-new n in let copied = bytes-copy-range out b 0 start n in out
    else bytes-new 0
  -- | Select a fresh initial state by its checkpoint variant code.
  start :: Nat -> Sha512State
  start variant = if variant == 4 then sha384-state else if variant == 5 then sha512t224-state else if variant == 6 then sha512t256-state else sha512-state
  -- | Exercise the matching whole-buffer entry point.
  sum :: Nat -> Bytes -> Bytes
  sum variant bytes = if variant == 4 then sha384-sum bytes else if variant == 5 then sha512t224-sum bytes else if variant == 6 then sha512t256-sum bytes else sha512-sum bytes
  -- | Expected digest size for a variant code.
  size-of :: Nat -> Nat
  size-of variant = if variant == 4 then sha384-size else if variant == 5 then sha512t224-size else if variant == 6 then sha512t256-size else sha512-size
  -- | Resume every requested split from an independently saved partial prefix.
  splits :: Nat -> Bytes -> String -> List Nat -> Bool
  splits variant raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = sha512-state-update (start variant) (part raw 0 point) in
      let saved = sha512-state-save prefix in
      case sha512-state-load (start variant) saved of
        Err error -> False
        Ok restored -> same (sha512-state-sum (sha512-state-update restored (part raw point (bytes-len raw - point)))) (input expected)
          && same saved (sha512-state-save prefix) && splits variant raw expected rest
      else False
  -- | Compare each public digest, checkpoint and prefix append with independent complete bytes.
  valid-case :: Nat -> Nat -> Bool -> String -> String -> Nat -> String -> List Nat -> Bool
  valid-case variant n fill expected saved mid checkpoint points =
    let raw = data-bytes n fill in let state = sha512-state-update (start variant) raw in
    let prefix = bytes-from-u8s [1, 2, 3] in
    same (sum variant raw) (input expected) && same (sha512-state-sum state) (input expected)
      && sha512-block-size == 128 && sha512-state-block-size state == 128
      && sha512-state-size state == size-of variant && bytes-len (sum variant raw) == size-of variant
      && same (sha512-state-sum (sha512-state-reset state)) (sum variant (bytes-new 0))
      && same (sha512-state-save state) (input saved)
      && same (sha512-state-save (sha512-state-update (start variant) (part raw 0 mid))) (input checkpoint)
      && same (sha512-state-append prefix state) (concat prefix (input expected))
      && same (sha512-state-append-save prefix state) (concat prefix (input saved))
      && same (sha512-state-sum state) (input expected)
      && splits variant raw expected points
  -- | Check success, identifier rejection or size rejection for one checkpoint input.
  load-case :: Nat -> String -> Nat -> Bool
  load-case variant text kind = case sha512-state-load (start variant) (input text) of
    Ok state -> kind == 0 && same (sha512-state-save state) (input text)
    Err InvalidSha512StateIdentifier -> kind == 1
    Err (InvalidSha512StateSize expected actual) -> kind == 2 && expected == 204 && actual == bytes-len (input text)
  -- | Mutating a caller buffer after an update, digest or save must not change the state or results.
  aliases :: Nat -> Bool
  aliases variant =
    let raw = data-bytes 300 False in
    let state = sha512-state-update (start variant) raw in
    let before = sha512-state-sum state in
    let branch = sha512-state-update state (bytes-from-u8s [9]) in
    let scribble = poke raw 299 0 in
    let digest = sha512-state-sum state in
    let flipped = poke digest 0 7 in
    let saved = sha512-state-save state in
    let wipe = poke saved 100 7 in
    same (sha512-state-sum state) before && not (same (sha512-state-sum branch) before)
      && same (sha512-state-sum state) (sum variant (data-bytes 300 False))
  -- | Explicit writes, the generic writer and bounded copies agree with the whole-buffer digest.
  stream-case :: forall r. using (io :: Io r) => Nat ->{IO r} Bool
  stream-case variant =
    let raw = data-bytes 1000 False in
    let explicit = case sha512-write (start variant) (part raw 0 400) of
          (count, next) -> case sha512-write next (part raw 400 600) of (rest, last) -> count == 400 && rest == 600 && same (sha512-state-sum last) (sum variant raw) in
    let generic = case stream-write-all (sha512-hashing-writer (start variant)) raw of
          (Ok count, writer) -> count == 1000
          (Err error, writer) -> False in
    let copied = case sha512-copy 7 (reader-from-bytes raw) (start variant) of
          (Ok count, (source, state)) -> count == 1000 && same (sha512-state-sum state) (sum variant raw)
          other -> False in
    explicit && generic && copied
  -- | A zero chunk size is rejected before any source read.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case sha512-copy 0 (reader-from-bytes (bytes-new 5)) sha512-state of
    (Err StreamInvalidChunkSize, (source, state)) -> same (sha512-state-sum state) (sha512-sum (bytes-new 0))
    other -> False
"#;
