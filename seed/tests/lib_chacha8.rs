//! The ChaCha8 generator against the reference SDK's (`tooling/chacha8_vectors.go`): scripted draws of 64-bit
//! values and byte reads across block, reseed-period and partial-read boundaries, state encodings and
//! generators restored from them produce identical outputs; malformed encodings are refused. The kernels also
//! lower for other targets.
mod common;

use serde_json::Value;

/// Every script and refusal matches the reference.
#[test]
fn chacha8_matches_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/chacha8-vectors.json")).unwrap();
    let mut rows = f["scripts"].as_array().unwrap().iter().map(|r| {
        let ops = r["ops"].as_array().unwrap().iter().map(|o| format!("{:?}", o.as_str().unwrap())).collect::<Vec<_>>().join(", ");
        let outs = r["outputs"].as_array().unwrap().iter().map(|o| format!("{:?}", o.as_str().unwrap())).collect::<Vec<_>>().join(", ");
        format!("script {:?} [{ops}] [{outs}]", r["seed"].as_str().unwrap())
    }).collect::<Vec<_>>();
    rows.extend(f["refused"].as_array().unwrap().iter().map(|r| {
        let (data, msg) = r.as_str().unwrap().split_once(' ').unwrap();
        format!("refused {data:?} {msg:?}")
    }));
    let dir = common::project("chacha8", &[
        ("Data.ChaCha8.cinch", include_str!("../prelude/Data/ChaCha8.cinch")),
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
fn chacha8_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Data.ChaCha8", include_str!("../prelude/Data/ChaCha8.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Data.ChaCha8\n  -- | Reach the kernels.\n  main :: Int\n  main = case chacha8-new (bytes-new 32) of\n    Ok g -> (case chacha8-read g 300 of (b, h) -> (if bytes-len b == 300 then 0 else 2))\n    Err e -> 1\n").unwrap();
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
        assert!(ir.contains("<4 x i32>"));
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
fn chacha8_declarations_have_canonical_documentation() {
    common::assert_documented("Data.ChaCha8", include_str!("../prelude/Data/ChaCha8.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference ChaCha8 consumers.
module Main where
  import Data.ChaCha8
  import Data.Bytes.Build
  import Text.Hex
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The number after an operation's name.
  arg :: String -> Nat
  arg op = digits (string-to-bytes op) 0 0 False
  -- | Decimal digits after the first space, accumulated.
  digits :: Bytes -> Nat -> Nat -> Bool -> Nat
  digits x i acc seen = if i >= bytes-len x then acc else if bytes-get x i == 32 then digits x (i + 1) acc True else if seen then digits x (i + 1) (10 * acc + minus (bytes-get x i) 48) seen else digits x (i + 1) acc seen
  -- | Saturating subtraction.
  minus :: Nat -> Nat -> Nat
  minus a c = if c <= a then a - c else 0
  -- | A small number as a machine integer.
  small :: Nat -> I64
  small n = if n < 1000 then nat-to-i64 n else 0
  -- | Whether an operation has a name.
  is :: String -> String -> Bool
  is op name = case string-to-bytes op of
    x -> bytes-len x >= bytes-len (string-to-bytes name) && eq-i64 (bytes-compare (part x (bytes-len (string-to-bytes name))) (string-to-bytes name)) 0
  -- | The first n bytes.
  part :: Bytes -> Nat -> Bytes
  part x n = let out = bytes-new n in let c = (if n <= bytes-len x then bytes-copy-range out x 0 0 n else ()) in out
  -- | n values as big-endian hex.
  draws :: ChaCha8 -> Nat -> List Bytes -> (String, ChaCha8)
  draws g n acc = if n == 0 then (hex-encode (bytes-concat (reverse acc)), g) else case chacha8-uint64 g of
    (v, h) -> let w = bytes-new 8 in let s = be w v 0 in draws h (minus n 1) (Cons w acc)
  -- | Store a value big-endian.
  be :: Bytes -> U64 -> Nat -> Unit
  be w v i = if i >= 8 then () else let s = (if i < bytes-len w then bytes-set w i (u8-to-nat (wrapping-u64-to-u8 (wrapping-shr-u64 v (wrapping-i64-to-u32 (small (minus 56 (8 * i))))))) else ()) in be w v (i + 1)
  -- | Run operations, comparing outputs; last is the latest encoding.
  run :: ChaCha8 -> List String -> List String -> Bytes -> Bool
  run g ops outs last = case (ops, outs) of
    (Nil, Nil) -> True
    (Cons op rest, Cons want more) ->
      if is op "u64" then (case draws g (arg op) Nil of (got, h) -> got == want && run h rest more last)
      else if is op "read" then (case chacha8-read g (arg op) of (got, h) -> hex-encode got == want && run h rest more last)
      else if is op "marshal" then (let e = chacha8-marshal g in hex-encode e == want && run g rest more e)
      else (case chacha8-unmarshal last of Ok h -> run h rest more last; Err e -> False)
    other -> False
  -- | A script row.
  script :: String -> List String -> List String -> Bool
  script seed ops outs = case chacha8-new (b seed) of
    Ok g -> run g ops outs (bytes-new 0)
    Err e -> False
  -- | A refused encoding.
  refused :: String -> String -> Bool
  refused encoded message = case chacha8-unmarshal (b encoded) of
    Err e -> chacha8-error-message e == message
    Ok g -> False
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
