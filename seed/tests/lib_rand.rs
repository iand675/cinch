//! Pseudorandom distributions against the reference SDK's generators (`tooling/rand_vectors.go`): scripted
//! batches of raw, bounded, float, permutation, shuffle, normal, exponential and Zipf draws from PCG and
//! ChaCha8 sources produce identical outputs draw for draw; PCG state encodings round-trip and malformed ones
//! are refused; invalid Zipf parameters are refused. The kernels also lower for other targets.
mod common;

use serde_json::Value;

/// A Cinch operation constructor from a fixture operation.
fn op(o: &Value) -> String {
    let args: Vec<u64> = o["args"].as_array().map(|a| a.iter().map(|v| v.as_u64().unwrap()).collect()).unwrap_or_default();
    let n = args.last().copied().unwrap_or(0);
    let mut lead: Vec<u64> = if args.is_empty() { vec![] } else { args[..args.len() - 1].to_vec() };
    lead.resize(3, 0);
    format!("Op {:?} {} {} {} {n}", o["name"].as_str().unwrap(), lead[0], lead[1], lead[2])
}

/// Every script, refusal and Zipf parameter check matches the reference.
#[test]
fn rand_matches_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/rand-vectors.json")).unwrap();
    let mut rows = Vec::new();
    for s in f["scripts"].as_array().unwrap() {
        let ops = s["ops"].as_array().unwrap().iter().map(op).collect::<Vec<_>>().join(", ");
        let outs = s["outputs"].as_array().unwrap().iter().map(|o| format!("{:?}", o.as_str().unwrap())).collect::<Vec<_>>().join(", ");
        if s["kind"] == "pcg" {
            let seed = s["seed"].as_array().unwrap();
            rows.push(format!("run-pcg (pcg-new {} {}) [{ops}] [{outs}] (bytes-new 0)", seed[0], seed[1]));
        } else {
            rows.push(format!("chacha-script {:?} [{ops}] [{outs}]", s["chacha"].as_str().unwrap()));
        }
    }
    for r in f["refused"].as_array().unwrap() {
        let (data, msg) = r.as_str().unwrap().split_once(' ').unwrap();
        rows.push(format!("refused {data:?} {msg:?}"));
    }
    let zipf = f["zipf_refused"].as_array().unwrap();
    for (i, (s, v)) in [("0x3FF0000000000000", "0x3FF0000000000000"), ("0x4000000000000000", "0x3FE0000000000000"), ("0x4000000000000000", "0x3FF0000000000000")].iter().enumerate() {
        rows.push(format!("zipf-refused {s} {v} {}", if zipf[i].as_bool().unwrap() { "True" } else { "False" }));
    }
    let dir = common::project("rand", &[
        ("Data.Rand.cinch", include_str!("../prelude/Data/Rand.cinch")),
        ("Data.ChaCha8.cinch", include_str!("../prelude/Data/ChaCha8.cinch")),
        ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row; 200 means all passed");
    common::cleanup(&dir);
}

/// The kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn rand_kernels_compile_for_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("wasm32-unknown-unknown", "wasm32", "little"),
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
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Data.Rand", include_str!("../prelude/Data/Rand.cinch")),
            ("Data.ChaCha8", include_str!("../prelude/Data/ChaCha8.cinch")),
            ("Data.Float", include_str!("../prelude/Data/Float.cinch")),
            ("Data.Array", include_str!("../prelude/Data/Array.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Data.Rand\n  -- | Reach the kernels.\n  main :: Int\n  main = case rand-norm-float64 (pcg-new 1 2) of\n    (x, g) -> (case rand-exp-float64 g of (y, h) -> (case rand-perm h 10 of (p, k) -> (if flt-f64 (add-f64 x y) 100 then 0 else 1)))\n").unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("switch ") && ir.contains("mul i128"));
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

/// Sources seeded from the operating system are independent: two of each kind draw different values.
#[test]
fn entropy_seeded_sources_differ() {
    let dir = common::project("rand-effect", &[
        ("Effect.Rand.cinch", include_str!("../prelude/Effect/Rand.cinch")),
        ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
        ("Data.Rand.cinch", include_str!("../prelude/Data/Rand.cinch")),
        ("Data.ChaCha8.cinch", include_str!("../prelude/Data/ChaCha8.cinch")),
        ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Main.cinch", r#"module Main where
  import Data.Rand
  import Data.ChaCha8
  import Effect.Rand
  -- | The first draw of a source.
  first-draw :: forall s. using (src :: Source s) => s -> U64
  first-draw g = case rand-uint64 g of (x, h) -> x
  -- | The first draw of an entropy-seeded ChaCha8 source, or 0.
  chacha-draw :: forall r. using (io :: Io r) => Unit ->{IO r} U64
  chacha-draw u = case random-chacha8 () of Ok g -> first-draw g; Err e -> 0
  -- | 0 when both pairs differ.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let a = first-draw (random-pcg ()) in
    let b = first-draw (random-pcg ()) in
    let c = chacha-draw () in
    let d = chacha-draw () in
    (if eq-u64 a b then 1 else 0) + (if eq-u64 c d then 2 else 0)
"#),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn rand_declarations_have_canonical_documentation() {
    common::assert_documented("Data.Rand", include_str!("../prelude/Data/Rand.cinch"));
    common::assert_documented("Effect.Rand", include_str!("../prelude/Effect/Rand.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference distribution consumers.
module Main where
  import Data.Rand
  import Data.ChaCha8
  import Data.Float
  import Data.Bytes.Build
  import Data.Bytes.Word
  import Text.Hex
  -- | A scripted operation: name, up to three parameters and a count.
  data Op where
    -- | Fields in the order listed above.
    Op :: String -> U64 -> U64 -> U64 -> Nat -> Op
  -- | A 64-bit value big-endian.
  be64 :: U64 -> Bytes
  be64 v = let b = bytes-new 8 in let s = (if 8 <= bytes-len b then bytes-write-le64 b 0 (swap-bytes-u64 v) else ()) in b
  -- | A 32-bit value big-endian.
  be32 :: U32 -> Bytes
  be32 v = let b = bytes-new 4 in let s = (if 4 <= bytes-len b then bytes-write-le32 b 0 (swap-bytes-u32 v) else ()) in b
  -- | n results of a drawing function, concatenated.
  batch :: forall s. (s -> (Bytes, s)) -> s -> Nat -> List Bytes -> (Bytes, s)
  batch f g n acc = if n == 0 then (bytes-concat (reverse acc), g) else (case f g of (b, h) -> batch f h (n - 1) (Cons b acc))
  -- | A list of indices as concatenated 64-bit values.
  ints :: List I64 -> Bytes
  ints xs = bytes-concat (map-be xs)
  -- | Each index as 8 bytes.
  map-be :: List I64 -> List Bytes
  map-be xs = case xs of Nil -> Nil; Cons x rest -> Cons (be64 (wrapping-i64-to-u64 x)) (map-be rest)
  -- | 1000 .. 1000 + n - 1.
  items :: Nat -> Nat -> List I64
  items i n = if i >= n then Nil else Cons (wrapping-add-i64 1000 (wrapping-u64-to-i64 (nat-word i))) (items (i + 1) n)
  -- | A small Nat as a word.
  nat-word :: Nat -> U64
  nat-word n = if n < 4611686018427387904 then wrapping-i64-to-u64 (nat-to-i64 n) else 0
  -- | An operation's output and the next source.
  step :: forall s. using (src :: Source s) => s -> Op -> (Bytes, s)
  step g o = case o of
    Op name a b c n ->
      if name == "u64" then batch (\h -> (case rand-uint64 h of (x, k) -> (be64 x, k))) g n Nil
      else if name == "i64" then batch (\h -> (case rand-int64 h of (x, k) -> (be64 (wrapping-i64-to-u64 x), k))) g n Nil
      else if name == "u32" then batch (\h -> (case rand-uint32 h of (x, k) -> (be32 x, k))) g n Nil
      else if name == "i32" then batch (\h -> (case rand-int32 h of (x, k) -> (be32 (wrapping-i32-to-u32 x), k))) g n Nil
      else if name == "u64n" then batch (\h -> (case rand-uint64-n h a of Some r -> (case r of (x, k) -> (be64 x, k)); None -> (bytes-new 0, h))) g n Nil
      else if name == "i64n" then batch (\h -> (case rand-int64-n h (wrapping-u64-to-i64 a) of Some r -> (case r of (x, k) -> (be64 (wrapping-i64-to-u64 x), k)); None -> (bytes-new 0, h))) g n Nil
      else if name == "u32n" then batch (\h -> (case rand-uint32-n h (wrapping-u64-to-u32 a) of Some r -> (case r of (x, k) -> (be32 x, k)); None -> (bytes-new 0, h))) g n Nil
      else if name == "i32n" then batch (\h -> (case rand-int32-n h (wrapping-u64-to-i32 a) of Some r -> (case r of (x, k) -> (be32 (wrapping-i32-to-u32 x), k)); None -> (bytes-new 0, h))) g n Nil
      else if name == "f64" then batch (\h -> (case rand-float64 h of (x, k) -> (be64 (f64-bits x), k))) g n Nil
      else if name == "f32" then batch (\h -> (case rand-float32 h of (x, k) -> (be32 (f32-bits x), k))) g n Nil
      else if name == "perm" then (case rand-perm g n of (p, k) -> (ints p, k))
      else if name == "shuffle" then (case rand-shuffle g (items 0 n) of (p, k) -> (ints p, k))
      else if name == "norm" then batch (\h -> (case rand-norm-float64 h of (x, k) -> (be64 (f64-bits x), k))) g n Nil
      else if name == "exp" then batch (\h -> (case rand-exp-float64 h of (x, k) -> (be64 (f64-bits x), k))) g n Nil
      else (case zipf-new (f64-from-bits a) (f64-from-bits b) c of
        Some z -> batch (\h -> (case zipf-uint64 z h of (x, k) -> (be64 x, k))) g n Nil
        None -> (bytes-new 0, g))
  -- | Run operations from any source, comparing outputs.
  run :: forall s. using (src :: Source s) => s -> List Op -> List String -> Bool
  run g ops outs = case (ops, outs) of
    (Nil, Nil) -> True
    (Cons o rest, Cons want more) -> (case step g o of (got, h) -> hex-encode got == want && run h rest more)
    other -> False
  -- | The name of an operation.
  op-name :: Op -> String
  op-name o = case o of Op name a b c n -> name
  -- | Run operations from PCG, with encoding and restoring; last is the latest encoding.
  run-pcg :: Pcg -> List Op -> List String -> Bytes -> Bool
  run-pcg g ops outs last = case (ops, outs) of
    (Nil, Nil) -> True
    (Cons o rest, Cons want more) ->
      if op-name o == "marshal" then (let e = pcg-marshal g in hex-encode e == want && run-pcg g rest more e)
      else if op-name o == "restore" then (case pcg-unmarshal last of Ok h -> want == "" && run-pcg h rest more last; Err e -> False)
      else (case step g o of (got, h) -> hex-encode got == want && run-pcg h rest more last)
    other -> False
  -- | Bytes from hex.
  hb :: String -> Bytes
  hb text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | A ChaCha8 script.
  chacha-script :: String -> List Op -> List String -> Bool
  chacha-script seed ops outs = case chacha8-new (hb seed) of Ok g -> run g ops outs; Err e -> False
  -- | A refused PCG encoding.
  refused :: String -> String -> Bool
  refused encoded message = case pcg-unmarshal (hb encoded) of Err e -> pcg-error-message e == message; Ok g -> False
  -- | Whether Zipf parameters are refused.
  zipf-refused :: U64 -> U64 -> Bool -> Bool
  zipf-refused s v want = case zipf-new (f64-from-bits s) (f64-from-bits v) 10 of None -> want; Some z -> not want
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
