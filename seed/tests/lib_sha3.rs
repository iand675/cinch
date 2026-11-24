//! Independent SHA-3, SHAKE and cSHAKE digests, output streams, checkpoints and hashing streams.
//! Fixtures come from `tooling/sha3_state_vectors.py` (hashlib outputs, NIST SP 800-185
//! cSHAKE samples and a separate Python sponge model for checkpoints).
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Sha3", include_str!("../prelude/Text/Sha3.cinch")),
        ("Effect.Sha3", include_str!("../prelude/Effect/Sha3.cinch")),
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
    serde_json::from_str(include_str!("fixtures/sha3-state-vectors.json")).unwrap()
}

/// Render a JSON array of naturals as a checked-language list.
fn list(value: &Value) -> String {
    format!("[{}]", value.as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(","))
}

/// Every variant, padding boundary, customization, checkpoint and requested split matches independent bytes.
#[test]
fn sha3_variants_match_independent_digest_checkpoint_and_split_vectors() {
    let f = fixture();
    let checks = f["valid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "valid-case {} {:?} {:?} {} {:?} {:?} {} {:?} {}",
                row["variant"],
                row["name"].as_str().unwrap(),
                row["custom"].as_str().unwrap(),
                row["size"],
                row["digest"].as_str().unwrap(),
                row["saved"].as_str().unwrap(),
                row["mid"],
                row["checkpoint"].as_str().unwrap(),
                list(&row["splits"]),
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(34) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Successive reads of every size, including empty reads and block-crossing reads, concatenate to one stream.
#[test]
fn shake_readers_continue_one_output_stream_across_block_boundaries() {
    let f = fixture();
    let checks = f["streams"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| format!("stream-reads {} {} {:?} {}", row["variant"], row["size"], row["output"].as_str().unwrap(), list(&row["reads"])))
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Identifier checks precede exact size, which precedes the pending-length check.
#[test]
fn sha3_checkpoint_errors_match_independent_identifier_size_and_tail_checks() {
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
                "tail" => 3,
                other => panic!("{other}"),
            };
            format!("load-case {} {:?} {kind}", row["variant"], row["input"].as_str().unwrap())
        })
        .collect::<Vec<_>>();
    run(&format!("all [{}]", checks.join(",\n")));
}

/// Retained branches, mutated caller buffers, sizes and error messages behave exactly.
#[test]
fn sha3_state_buffers_and_retained_branches_are_independent() {
    run("aliases () && sizes () && sha3-state-error-message InvalidSha3StateIdentifier == \"SHA-3 state identifier is invalid\" && sha3-state-error-message (InvalidSha3StateSize 573 5) == \"SHA-3 state requires 573 bytes; received 5\" && sha3-state-error-message (InvalidSha3StateTail 200) == \"SHA-3 state pending length is invalid: 200\"");
}

/// Explicit, generic and bounded-copy hashing agree; SHAKE readers adapt to the generic reader protocol.
#[test]
fn sha3_streams_preserve_digests_counts_and_source_continuations() {
    run("stream-case () && zero-copy () && reader-case ()");
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn sha3_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Sha3", include_str!("../prelude/Text/Sha3.cinch"));
    common::assert_documented("Effect.Sha3", include_str!("../prelude/Effect/Sha3.cinch"));
}

/// The permutation and raw lane loads remain legal on 32-bit and non-x86 object targets.
#[test]
fn sha3_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let source = include_str!("../prelude/Text/Sha3.cinch");
    let marker = "  -- | Generated Keccak-f[1600] permutation:";
    let data = source.find("  data KeccakLanes where").unwrap();
    let data_end = data + source[data..].find("\n\n").unwrap();
    let mut kernels = format!("-- | Isolated permutation compilation.\nmodule Main where\n  import Data.Machine\n{}\n", &source[data..data_end]);
    kernels.push_str(&source[source.find(marker).unwrap()..]);
    let start = source.find("  prim lane-word ::").unwrap();
    let rest = &source[start..];
    kernels.push_str("\n  -- | Checked private raw load under target compilation.\n");
    kernels.push_str(&rest[..rest.find("\n  -- |").unwrap()]);
    kernels.push_str("\n  -- | Retain the fixture buffer while its private loads finish.\n  keep :: Bytes -> KeccakLanes -> KeccakLanes\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep every absorb step reachable.\n  main :: Int\n  main = let bytes = bytes-new 168 in let z = Lanes 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 in\n    let a = keep bytes (keccak-absorb21 (keccak-absorb18 (keccak-absorb17 (keccak-absorb13 (keccak-absorb9 z (bytes-data bytes)) (bytes-data bytes)) (bytes-data bytes)) (bytes-data bytes)) (bytes-data bytes)) in\n    case a of Lanes x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15 x16 x17 x18 x19 x20 x21 x22 x23 x24 -> if eq-u64 x0 0 then 0 else 1\n");
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
                let program = cinch::driver::check_sources_profile(&input, "keccak.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("@llvm.fshl.i64"));
        let path = directory.path().join(format!("{arch}.ll"));
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

/// Checked consumers build input bytes from compact independent corpus descriptions.
const HELPERS: &str = r#"-- | Independent SHA-3 fixture consumers.
module Main where
  import Text.Sha3
  import Text.Binary
  import Text.Hex
  import Effect.Sha3
  import Effect.Stream
  -- | Build deterministic input independently of digest calculations.
  data-bytes :: Nat -> Bytes
  data-bytes n =
    if n <= 65536 then
      let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) in
      let b = bytes-new size in let done = fill-bytes b 0 in b
    else bytes-new 0
  -- | Keep constant corpus setup in the executable instead of the checker's byte-buffer evaluator.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Fill a full-byte permutation with an exact bounded loop.
  fill-bytes :: Bytes -> Nat -> Unit
  fill-bytes b i = if i < bytes-len b then
    let written = bytes-set b i (nat-rem (i * 37 + 11) 256) in fill-bytes b (i + 1)
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
  -- | Select an initial state by variant code and, for cSHAKE, its hex-encoded name and customization.
  start :: Nat -> String -> String -> Sha3State
  start variant name custom =
    if variant == 1 then sha3-state224 else if variant == 2 then sha3-state256
    else if variant == 3 then sha3-state384 else if variant == 4 then sha3-state512
    else if variant == 5 then shake128-state else if variant == 6 then shake256-state
    else if variant == 7 then cshake128-state (input name) (input custom) else cshake256-state (input name) (input custom)
  -- | The matching whole-buffer convenience, where one exists.
  sum :: Nat -> Bytes -> Bytes -> Bytes
  sum variant raw fallback =
    if variant == 1 then sha3-sum224 raw else if variant == 2 then sha3-sum256 raw
    else if variant == 3 then sha3-sum384 raw else if variant == 4 then sha3-sum512 raw
    else if variant == 5 then shake128-sum raw 32 else if variant == 6 then shake256-sum raw 64 else fallback
  -- | Resume every requested split from an independently saved partial prefix.
  splits :: Sha3State -> Bytes -> String -> List Nat -> Bool
  splits initial raw expected points = case points of
    Nil -> True
    Cons point rest -> if point <= bytes-len raw then
      let prefix = sha3-state-update initial (part raw 0 point) in
      let saved = sha3-state-save prefix in
      case sha3-state-load initial saved of
        Err error -> False
        Ok restored -> same (sha3-state-sum (sha3-state-update restored (part raw point (bytes-len raw - point)))) (input expected)
          && same saved (sha3-state-save prefix) && splits initial raw expected rest
      else False
  -- | Compare every public digest, checkpoint, reset and append with independent bytes.
  valid-case :: Nat -> String -> String -> Nat -> String -> String -> Nat -> String -> List Nat -> Bool
  valid-case variant name custom n expected saved mid checkpoint points =
    let raw = data-bytes n in let initial = start variant name custom in
    let state = sha3-state-update initial raw in
    let prefix = bytes-from-u8s [1, 2, 3] in
    same (sum variant raw (input expected)) (input expected) && same (sha3-state-sum state) (input expected)
      && same (sha3-state-sum (sha3-state-reset state)) (sha3-state-sum initial)
      && same (sha3-state-save state) (input saved)
      && same (sha3-state-save (sha3-state-update initial (part raw 0 mid))) (input checkpoint)
      && same (sha3-state-append prefix state) (concat prefix (input expected))
      && same (sha3-state-append-save prefix state) (concat prefix (input saved))
      && splits initial raw expected points
  -- | Read a SHAKE stream in pieces and compare the concatenation with one independent output.
  stream-reads :: Nat -> Nat -> String -> List Nat -> Bool
  stream-reads variant n expected reads =
    case shake-read (sha3-state-update (start variant "" "") (data-bytes n)) 0 of
      (empty, reader) -> bytes-len empty == 0 && same (gather reader reads (bytes-new 0)) (input expected)
  -- | Concatenate successive reads.
  gather :: ShakeReader -> List Nat -> Bytes -> Bytes
  gather reader reads acc = case reads of
    Nil -> acc
    Cons count rest -> case shake-reader-read reader count of (out, next) -> gather next rest (concat acc out)
  -- | Check success, identifier, size or pending-length rejection for one checkpoint input.
  load-case :: Nat -> String -> Nat -> Bool
  load-case variant text kind = case sha3-state-load (start variant "" "78") (input text) of
    Ok state -> kind == 0 && same (sha3-state-save state) (input text)
    Err InvalidSha3StateIdentifier -> kind == 1
    Err (InvalidSha3StateSize expected actual) -> kind == 2 && expected == 573 && actual == bytes-len (input text)
    Err (InvalidSha3StateTail length) -> kind == 3
  -- | Digest sizes and rates of every variant.
  sizes :: Unit -> Bool
  sizes unit =
    sha3-state-size sha3-state224 == 28 && sha3-state-size sha3-state256 == 32 && sha3-state-size sha3-state384 == 48 && sha3-state-size sha3-state512 == 64
      && sha3-state-size shake128-state == 32 && sha3-state-size shake256-state == 64
      && sha3-state-block-size sha3-state224 == 144 && sha3-state-block-size sha3-state256 == 136 && sha3-state-block-size sha3-state384 == 104
      && sha3-state-block-size sha3-state512 == 72 && sha3-state-block-size shake128-state == 168 && sha3-state-block-size shake256-state == 136
      && sha3-state-block-size (cshake128-state (bytes-new 1) (bytes-new 0)) == 168
  -- | Mutating caller buffers after updates, digests or saves never changes a retained state.
  aliases :: Unit -> Bool
  aliases unit =
    let raw = data-bytes 300 in
    let state = sha3-state-update sha3-state256 raw in
    let before = sha3-state-sum state in
    let branch = sha3-state-update state (bytes-from-u8s [9]) in
    let scribble = poke raw 299 0 in
    let digest = sha3-state-sum state in
    let flipped = poke digest 0 7 in
    let saved = sha3-state-save state in
    let wipe = poke saved 300 7 in
    same (sha3-state-sum state) before && not (same (sha3-state-sum branch) before)
      && same (sha3-state-sum state) (sha3-sum256 (data-bytes 300))
  -- | Explicit writes, the generic writer and bounded copies agree with the whole-buffer digest.
  stream-case :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  stream-case unit =
    let raw = data-bytes 1000 in
    let explicit = case sha3-write sha3-state512 (part raw 0 400) of
          (count, next) -> case sha3-write next (part raw 400 600) of (rest, last) -> count == 400 && rest == 600 && same (sha3-state-sum last) (sha3-sum512 raw) in
    let generic = case stream-write-all (sha3-hashing-writer sha3-state256) raw of
          (Ok count, writer) -> count == 1000
          (Err error, writer) -> False in
    let copied = case sha3-copy 7 (reader-from-bytes raw) shake256-state of
          (Ok count, (source, state)) -> count == 1000 && same (sha3-state-sum state) (shake256-sum raw 64)
          other -> False in
    explicit && generic && copied
  -- | A zero chunk size is rejected before any source read.
  zero-copy :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  zero-copy unit = case sha3-copy 0 (reader-from-bytes (bytes-new 5)) sha3-state256 of
    (Err StreamInvalidChunkSize, (source, state)) -> same (sha3-state-sum state) (sha3-sum256 (bytes-new 0))
    other -> False
  -- | A SHAKE reader adapted to the generic protocol yields the same output stream.
  reader-case :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  reader-case unit = case shake-read (sha3-state-update shake128-state (data-bytes 10)) 0 of
    (empty, reader) -> case stream-read-exact (shake-byte-reader reader) 400 of
      (Ok out, next) -> same out (shake128-sum (data-bytes 10) 400)
      (Err error, next) -> False
"#;
