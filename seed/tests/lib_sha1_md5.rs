//! Independent SHA-1 and MD5 digests, persistent checkpoints and hashing streams.
//! Fixtures come from `tooling/sha1_md5_vectors.py` (hashlib digests and separate
//! Python compression models for checkpoint bytes).
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// One algorithm's module pair and naming stem.
struct Algorithm {
    stem: &'static str,
    module: &'static str,
    text: &'static str,
    effect: &'static str,
    size: usize,
    checkpoint: usize,
}

const SHA1: Algorithm = Algorithm {
    stem: "sha1",
    module: "Sha1",
    text: include_str!("../prelude/Text/Sha1.cinch"),
    effect: include_str!("../prelude/Effect/Sha1.cinch"),
    size: 20,
    checkpoint: 96,
};

const MD5: Algorithm = Algorithm {
    stem: "md5",
    module: "Md5",
    text: include_str!("../prelude/Text/Md5.cinch"),
    effect: include_str!("../prelude/Effect/Md5.cinch"),
    size: 16,
    checkpoint: 92,
};

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(algorithm: &Algorithm, body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (format!("Text.{}", algorithm.module), algorithm.text),
        (format!("Effect.{}", algorithm.module), algorithm.effect),
        ("Text.Binary".into(), include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex".into(), include_str!("../prelude/Text/Hex.cinch")),
        ("Effect.Stream".into(), include_str!("../prelude/Effect/Stream.cinch")),
        ("Data.Bytes.Word".into(), include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let helpers = HELPERS.replace("ALG", algorithm.stem).replace("MODULE", algorithm.module)
        .replace("DIGEST_SIZE", &algorithm.size.to_string()).replace("CHECKPOINT_SIZE", &algorithm.checkpoint.to_string());
    std::fs::write(
        directory.path().join("Main.cinch"),
        format!("{helpers}\n  -- | Observe the complete runtime fixture result.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if ({body}) then 0 else 1\n"),
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
        assert_eq!(output.status.code(), Some(0), "{} debug={debug}: {}", algorithm.stem, String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Read fixed independent results without requiring any reference toolchain during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/sha1-md5-vectors.json")).unwrap()
}

/// Rows of the fixture section that belong to one algorithm.
fn rows<'a>(f: &'a Value, section: &str, algorithm: &Algorithm) -> Vec<&'a Value> {
    f[section].as_array().unwrap().iter().filter(|row| row["algorithm"] == algorithm.stem).collect()
}

/// Whole-buffer digests, padding boundaries, checkpoints and every requested split match independent bytes.
#[test]
fn sha1_and_md5_match_independent_digest_checkpoint_and_split_vectors() {
    let f = fixture();
    for algorithm in [&SHA1, &MD5] {
        let checks = rows(&f, "valid", algorithm)
            .iter()
            .map(|row| {
                let points = row["splits"].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(",");
                format!(
                    "valid-case {} {} {:?} {:?} {} {:?} [{points}]",
                    row["size"],
                    if row["fill"].as_bool().unwrap() { "True" } else { "False" },
                    row["digest"].as_str().unwrap(),
                    row["saved"].as_str().unwrap(),
                    row["mid"],
                    row["checkpoint"].as_str().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        run(algorithm, &format!("all [{}]", checks.join(",\n")));
    }
}

/// Identifier checks precede exact-size checks, and accepted checkpoints round-trip exactly.
#[test]
fn sha1_and_md5_checkpoint_errors_match_independent_identifier_and_size_checks() {
    let f = fixture();
    for algorithm in [&SHA1, &MD5] {
        let checks = rows(&f, "loads", algorithm)
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
        run(algorithm, &format!("all [{}] && aliases ()", checks.join(",\n")));
    }
}

/// Explicit and generic writes and bounded copies agree and reject zero chunks before reading.
#[test]
fn sha1_and_md5_streams_preserve_digests_counts_and_source_continuations() {
    for algorithm in [&SHA1, &MD5] {
        run(algorithm, "stream-case () && zero-copy ()");
    }
}

/// The SIMD schedule and rotate fragments remain legal on 32-bit and non-x86 object targets.
#[test]
fn sha1_and_md5_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (algorithm, marker, data, words) in [
        (&SHA1, "  -- | Generated SHA-1 compression:", "  data Sha1Words where\n    -- | Working words.\n    Words160 :: U32 -> U32 -> U32 -> U32 -> U32 -> Sha1Words\n", "Words160 a b c d e"),
        (&MD5, "  -- | Generated MD5 compression:", "  data Md5Words where\n    -- | Working words.\n    Words128 :: U32 -> U32 -> U32 -> U32 -> Md5Words\n", "Words128 a b c d"),
    ] {
        let source = algorithm.text;
        let mut kernels = format!("-- | Isolated kernel compilation.\nmodule Main where\n  import Data.Machine\n  import Data.Wide\n  -- | Raw words.\n{data}");
        kernels.push_str(&source[source.find(marker).unwrap()..]);
        let start = source.find("  prim block-word ::").unwrap();
        let rest = &source[start..];
        kernels.push_str("\n  -- | Checked private raw load under target compilation.\n");
        kernels.push_str(&rest[..rest.find("\n  -- |").unwrap()]);
        let first = words.split(' ').nth(1).unwrap();
        let initial = if algorithm.stem == "sha1" { "Words160 1 2 3 4 5" } else { "Words128 1 2 3 4" };
        kernels.push_str(&format!("\n  -- | Retain the fixture buffer while its private load finishes.\n  keep :: Bytes -> U64 -> U64\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep compression units reachable.\n  main :: Int\n  main = let bytes = bytes-new 64 in let word = keep bytes (block-word (bytes-data bytes) 0) in case {}-compress ({initial}) word 0 0 0 0 0 0 0 of {words} -> if eq-u32 {first} 0 then 0 else 1\n", algorithm.stem));
        for (triple, arch, endian) in [
            ("wasm32-unknown-unknown", "wasm32", "little"),
            ("aarch64-unknown-linux-gnu", "aarch64", "little"),
            ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
        ] {
            let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
                .unwrap_or_else(|_| {
                    let mut profile = cinch::core::TargetProfile::native();
                    profile.triple = triple.into();
                    profile.arch = arch.into();
                    profile.endian = endian.into();
                    profile
                });
            let input = kernels.clone();
            let ir = std::thread::Builder::new()
                .stack_size(64 * 1024 * 1024)
                .spawn(move || {
                    let program = cinch::driver::check_sources_profile(&input, "kernels.cinch", &profile)?;
                    cinch::codegen::emit_for(&profile, &program)
                })
                .unwrap()
                .join()
                .unwrap()
                .unwrap();
            if algorithm.stem == "sha1" {
                assert!(ir.contains("<4 x i32>") && ir.contains("@llvm.fshl.v4i32"));
            }
            let path = directory.path().join(format!("{}-{arch}.ll", algorithm.stem));
            std::fs::write(&path, &ir).unwrap();
            let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
                .args(["-target", triple, "-O2", "-c", "-x", "ir"])
                .arg(&path)
                .arg("-o")
                .arg(path.with_extension("o"))
                .output()
                .unwrap();
            assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn sha1_and_md5_declarations_have_canonical_documentation() {
    for algorithm in [&SHA1, &MD5] {
        common::assert_documented(&format!("Text.{}", algorithm.module), algorithm.text);
        common::assert_documented(&format!("Effect.{}", algorithm.module), algorithm.effect);
    }
}

/// Checked consumers; ALG, MODULE and the sizes are substituted per algorithm.
const HELPERS: &str = r#"-- | Independent digest checkpoint and stream fixture consumers.
module Main where
  import Text.MODULE
  import Text.Binary
  import Text.Hex
  import Effect.MODULE
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
  -- | Parse independently recorded bytes.
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
  -- | Resume every requested split from an independently saved partial prefix.
  splits :: Bytes -> String -> List Nat -> Bool
  splits raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = ALG-state-update ALG-state (part raw 0 point) in
      let saved = ALG-state-save prefix in
      case ALG-state-load saved of
        Err error -> False
        Ok restored -> same (ALG-state-sum (ALG-state-update restored (part raw point (bytes-len raw - point)))) (input expected)
          && same saved (ALG-state-save prefix) && splits raw expected rest
      else False
  -- | Compare each public digest, checkpoint and prefix append with independent complete bytes.
  valid-case :: Nat -> Bool -> String -> String -> Nat -> String -> List Nat -> Bool
  valid-case n fill expected saved mid checkpoint points =
    let raw = data-bytes n fill in let state = ALG-state-update ALG-state raw in
    let prefix = bytes-from-u8s [1, 2, 3] in
    same (ALG-sum raw) (input expected) && same (ALG-state-sum state) (input expected)
      && ALG-size == DIGEST_SIZE && ALG-state-size state == DIGEST_SIZE && ALG-block-size == 64 && ALG-state-block-size state == 64
      && same (ALG-state-sum (ALG-state-reset state)) (ALG-sum (bytes-new 0))
      && same (ALG-state-save state) (input saved)
      && same (ALG-state-save (ALG-state-update ALG-state (part raw 0 mid))) (input checkpoint)
      && same (ALG-state-append prefix state) (concat prefix (input expected))
      && same (ALG-state-append-save prefix state) (concat prefix (input saved))
      && splits raw expected points
  -- | Check success, identifier rejection or size rejection for one checkpoint input.
  load-case :: String -> Nat -> Bool
  load-case text kind = case ALG-state-load (input text) of
    Ok state -> kind == 0 && same (ALG-state-save state) (input text)
    Err error -> case error of
      InvalidMODULEStateIdentifier -> kind == 1
      InvalidMODULEStateSize expected actual -> kind == 2 && expected == CHECKPOINT_SIZE && actual == bytes-len (input text)
  -- | Mutating caller buffers after updates, digests or saves never changes a retained state.
  aliases :: Unit -> Bool
  aliases unit =
    let raw = data-bytes 300 False in
    let state = ALG-state-update ALG-state raw in
    let before = ALG-state-sum state in
    let branch = ALG-state-update state (bytes-from-u8s [9]) in
    let scribble = poke raw 299 0 in
    let digest = ALG-state-sum state in
    let flipped = poke digest 0 7 in
    let saved = ALG-state-save state in
    let wipe = poke saved 30 7 in
    same (ALG-state-sum state) before && not (same (ALG-state-sum branch) before)
      && same (ALG-state-sum state) (ALG-sum (data-bytes 300 False))
  -- | Explicit writes, the generic writer and bounded copies agree with the whole-buffer digest.
  stream-case :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-case unit =
    let raw = data-bytes 1000 False in
    let explicit = case ALG-write ALG-state (part raw 0 400) of
          (count, next) -> case ALG-write next (part raw 400 600) of (rest, last) -> count == 400 && rest == 600 && same (ALG-state-sum last) (ALG-sum raw) in
    let generic = case stream-write-all (ALG-hashing-writer ALG-state) raw of
          (Ok count, writer) -> count == 1000
          (Err error, writer) -> False in
    let copied = case ALG-copy 7 (reader-from-bytes raw) ALG-state of
          (Ok count, (source, state)) -> count == 1000 && same (ALG-state-sum state) (ALG-sum raw)
          other -> False in
    explicit && generic && copied
  -- | A zero chunk size is rejected before any source read.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case ALG-copy 0 (reader-from-bytes (bytes-new 5)) ALG-state of
    (Err StreamInvalidChunkSize, (source, state)) -> same (ALG-state-sum state) (ALG-sum (bytes-new 0))
    other -> False
"#;
