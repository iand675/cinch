//! Text.Flate and Effect.Flate: raw DEFLATE (RFC 1951) compression, decompression,
//! preset dictionaries, and Effect.Stream adapters, cross-checked bidirectionally
//! against Go's official `compress/flate` implementation.
mod common;

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const TEXT_FLATE: &str = include_str!("../prelude/Text/Flate.cinch");
const EFFECT_FLATE: &str = include_str!("../prelude/Effect/Flate.cinch");
const EFFECT_STREAM: &str = include_str!("../prelude/Effect/Stream.cinch");

fn flate_project(tag: &str, main_src: &str) -> PathBuf {
    common::project(
        tag,
        &[
            ("Text.Flate.cinch", TEXT_FLATE),
            ("Effect.Stream.cinch", EFFECT_STREAM),
            ("Effect.Flate.cinch", EFFECT_FLATE),
            ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
            ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
            ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Main.cinch", main_src),
        ],
    )
}

fn cinch_str(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn hex_decode(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "odd hex length");
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

/// Varied English prose + JSON corpus (non-trivial entropy, ~24 KiB) to verify real compression ratio.
fn varied_text_corpus() -> Vec<u8> {
    let sentences = [
        "RFC 1951 defines a lossless compressed data format using a combination of the LZ77 algorithm and Huffman coding.",
        "Each block begins with a three-bit header specifying whether it is the final block and which encoding type follows.",
        "Stored blocks align to the next byte boundary and carry a sixteen-bit length paired with its one's complement.",
        "Fixed Huffman blocks use predefined bit lengths for literals, match lengths, and five-bit distance codes.",
        "Dynamic Huffman blocks transmit compact run-length encoded code lengths using a secondary nineteen-symbol alphabet.",
        "A sliding window of thirty-two kilobytes permits back-references across recent output or a preset dictionary.",
        "Package-merge computes optimal length-limited canonical prefix codes bounded at fifteen or seven bits.",
        "Sync flushes emit an empty stored block so streaming readers can immediately decode every byte written so far.",
    ];
    let mut out = String::new();
    let mut state: u32 = 0x2468ace1;
    for i in 0..220 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let idx = ((state >> 24) as usize) % sentences.len();
        out.push_str(&format!(
            "{{\"seq\":{},\"code\":{},\"text\":\"{}\"}}\n",
            i,
            state % 997,
            sentences[idx]
        ));
    }
    out.into_bytes()
}

fn prng_bytes(n: usize, seed: u32) -> Vec<u8> {
    let mut out = vec![0u8; n];
    let mut s = seed;
    for b in &mut out {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        *b = (s >> 24) as u8;
    }
    out
}

fn repetitive_pattern(n: usize) -> Vec<u8> {
    let pat = b"alpha-beta-gamma-delta-epsilon-zeta-0123456789\n";
    (0..n).map(|i| pat[i % pat.len()]).collect()
}

fn geometric_bytes(n: usize) -> Vec<u8> {
    let mut out = vec![0u8; n];
    let mut s: u32 = 0x13579bdf;
    for b in &mut out {
        s = s.wrapping_mul(1103515245).wrapping_add(12345);
        *b = s.trailing_zeros().min(20) as u8;
    }
    out
}

/// Every declaration in Text.Flate and Effect.Flate carries canonical documentation.
#[test]
fn flate_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Flate", TEXT_FLATE);
    common::assert_documented("Effect.Flate", EFFECT_FLATE);
}

/// Text.Flate decodes all 73 Go 1.24 compress/flate reference streams bit-for-bit
/// (levels 0, 1, 6, 9, -2, preset dictionaries including 40 KiB, >64 KiB multi-block,
/// geometric Huffman trees, sync-flushed streams, and trailing-byte streams).
#[test]
fn flate_decodes_all_go_reference_streams() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/flate-vectors.json")).unwrap();
    let streams = fixture["streams"].as_array().unwrap();
    assert!(streams.len() >= 70, "expected >= 70 Go vectors, got {}", streams.len());

    let scratch = tempfile::tempdir().unwrap();
    let mut checks = Vec::new();
    for (i, row) in streams.iter().enumerate() {
        let packed = hex_decode(row["packed_hex"].as_str().unwrap());
        let dict = hex_decode(row["dict_hex"].as_str().unwrap());
        let size = row["size"].as_u64().unwrap();
        let trailing = row["trailing"].as_u64().unwrap();
        let sha = row["plain_sha256"].as_str().unwrap();
        let plain_hex = row["plain_hex"].as_str().unwrap();

        let packed_path = scratch.path().join(format!("packed_{i}.bin"));
        let dict_path = scratch.path().join(format!("dict_{i}.bin"));
        std::fs::write(&packed_path, &packed).unwrap();
        std::fs::write(&dict_path, &dict).unwrap();

        let expected_consumed = packed.len() as u64 - trailing;
        if !plain_hex.is_empty() {
            let plain = hex_decode(plain_hex);
            let plain_path = scratch.path().join(format!("plain_{i}.bin"));
            std::fs::write(&plain_path, &plain).unwrap();
            checks.push(format!(
                "check-exact {} {} {} {}",
                cinch_str(&packed_path),
                cinch_str(&dict_path),
                cinch_str(&plain_path),
                expected_consumed
            ));
        } else {
            checks.push(format!(
                "check-sha {} {} {} {:?} {}",
                cinch_str(&packed_path),
                cinch_str(&dict_path),
                size,
                sha,
                expected_consumed
            ));
        }
    }

    let main_src = format!(
        r#"-- | Verify all Go compress/flate reference streams.
module Main where
  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Text.Flate
  import Text.Hex
  import Text.Sha256

  -- | Read a fixture file from disk.
  load :: forall r. using (io :: Io r) => String ->{{IO r}} Bytes
  load p = case fs-read (path-from-string p) of
    Ok b -> b
    Err _ -> bytes-new 0

  -- | Verify exact byte-for-byte decode and consumed length.
  check-exact :: forall r. using (io :: Io r) => String -> String -> String -> Nat ->{{IO r}} Bool
  check-exact packed-p dict-p plain-p want-consumed =
    let packed = load packed-p in
    let dict = load dict-p in
    let plain = load plain-p in
    let res = if bytes-len dict == 0 then flate-decompress packed else flate-decompress-dict dict packed in
    case res of
      Err _ -> False
      Ok (got, consumed) -> got == plain && consumed == want-consumed

  -- | Verify decoded output by exact length, SHA-256 digest, and consumed length.
  check-sha :: forall r. using (io :: Io r) => String -> String -> Nat -> String -> Nat ->{{IO r}} Bool
  check-sha packed-p dict-p want-size want-sha want-consumed =
    let packed = load packed-p in
    let dict = load dict-p in
    let res = if bytes-len dict == 0 then flate-decompress packed else flate-decompress-dict dict packed in
    case res of
      Err _ -> False
      Ok (got, consumed) -> bytes-len got == want-size && consumed == want-consumed && hex-encode (sha256-sum got) == want-sha

  -- | Return the index of the first failing check, or 200 if every check passed.
  run-all :: forall r. using (io :: Io r) => List (Unit ->{{IO r}} Bool) -> Int ->{{IO r}} Int
  run-all xs i = case xs of
    Nil -> 200
    Cons f rest -> if f () then run-all rest (i + 1) else i

  -- | Execute all reference stream checks.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    run-all [{}] 0
"#,
        checks
            .iter()
            .map(|c| format!("(\\x -> {c})"))
            .collect::<Vec<_>>()
            .join(",\n      ")
    );

    let dir = flate_project("flate_decode_go", &main_src);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    if code != 200 {
        let name = streams
            .get(code as usize)
            .and_then(|v| v["name"].as_str())
            .unwrap_or("<unknown>");
        panic!(
            "Go vector #{code} ({name}) failed to decode accurately in Text.Flate (exit {code})"
        );
    }
}

/// Text.Flate rejects malformed streams, invalid levels, and output-limit violations with named errors.
#[test]
fn flate_rejects_malformed_streams_and_limits() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/flate-vectors.json")).unwrap();
    let errors = fixture["errors"].as_array().unwrap();

    let mut rows: Vec<String> = errors
        .iter()
        .map(|row| {
            format!(
                "check-malformed {:?} {:?}",
                row["packed_hex"].as_str().unwrap(),
                row["expected"].as_str().unwrap()
            )
        })
        .collect();
    rows.push("check-bad-levels ()".to_string());
    rows.push("check-output-limit ()".to_string());
    rows.push("check-messages ()".to_string());

    let main_src = format!(
        r#"-- | Verify error paths, bad levels, output limits, and error diagnostics.
module Main where
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Flate
  import Text.Hex

  -- | Decode a hex string into Bytes.
  unhex :: String -> Bytes
  unhex s = case hex-decode s of
    Ok b -> b
    Err _ -> bytes-new 0

  -- | Verify that malformed input yields its expected FlateError variant.
  check-malformed :: String -> String -> Bool
  check-malformed packed-hex expected = case flate-decompress (unhex packed-hex) of
    Ok _ -> False
    Err e -> case e of
      FlateUnexpectedEof -> expected == "eof"
      FlateUnsupportedBlock -> expected == "unsupported"
      FlateCorruptInput _ -> expected == "corrupt"
      _ -> False

  -- | Verify invalid compression levels are rejected with FlateBadLevel.
  check-bad-levels :: Unit -> Bool
  check-bad-levels u =
    let payload = string-to-bytes "abc" in
    let r0 = (case flate-compress (FlateLevel 0) payload of Err FlateBadLevel -> True; _ -> False) in
    let r10 = (case flate-compress (FlateLevel 10) payload of Err FlateBadLevel -> True; _ -> False) in
    let rm3 = (case flate-level-from-int (-3) of Err FlateBadLevel -> True; _ -> False) in
    let rp10 = (case flate-level-from-int 10 of Err FlateBadLevel -> True; _ -> False) in
    let rm2 = (case flate-level-from-int (-2) of Ok FlateHuffmanOnly -> True; _ -> False) in
    let rm1 = (case flate-level-from-int (-1) of Ok FlateDefaultCompression -> True; _ -> False) in
    let rz = (case flate-level-from-int 0 of Ok FlateNoCompression -> True; _ -> False) in
    r0 && r10 && rm3 && rp10 && rm2 && rm1 && rz

  -- | Verify flate-decompress-limit rejects over-limit output and accepts exact-limit output.
  check-output-limit :: Unit -> Bool
  check-output-limit u =
    let raw = string-to-bytes "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
    case flate-compress FlateDefaultCompression raw of
      Err _ -> False
      Ok packed ->
        let over = (case flate-decompress-limit 49 packed of Err FlateOutputLimitExceeded -> True; _ -> False) in
        let exact = (case flate-decompress-limit 50 packed of Ok (out, _) -> out == raw; _ -> False) in
        over && exact

  -- | Verify every FlateError constructor renders a descriptive message.
  check-messages :: Unit -> Bool
  check-messages u =
    flate-error-message (FlateCorruptInput 42) == "flate: corrupt input before offset 42"
      && flate-error-message FlateUnexpectedEof == "flate: unexpected end of input"
      && flate-error-message FlateBadLevel == "flate: invalid compression level"
      && flate-error-message FlateOutputLimitExceeded == "flate: decompressed output exceeds limit"
      && flate-error-message FlateUnsupportedBlock == "flate: unsupported reserved block type"

  -- | Index of first failure, or 200 when every check passes.
  first :: List Bool -> Int -> Int
  first xs i = case xs of
    Nil -> 200
    Cons x rest -> if x then first rest (i + 1) else i

  -- | Run all error-handling checks.
  main :: Int
  main = first [{}] 0
"#,
        rows.join(",\n    ")
    );

    let dir = flate_project("flate_errors", &main_src);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "malformed/error check #{code} failed");
}

/// Text.Flate compresses across levels 0/1/3/6/9/HuffmanOnly and preset dictionaries,
/// achieves real compression ratios (>3x on varied text, >50x on repeated patterns),
/// roundtrips under Text.Flate, AND every emitted stream decodes bit-for-bit under Go `compress/flate`.
#[test]
fn flate_encoder_compresses_and_interops_with_go() {
    let scratch = tempfile::tempdir().unwrap();

    let text_24k = varied_text_corpus();
    let rep_80k = repetitive_pattern(80000);
    let run_80k = vec![b'Z'; 80000];
    let prng_4k = prng_bytes(4096, 42);
    let prng_70k = prng_bytes(70000, 999);
    let geom_8k = geometric_bytes(8192);
    let short_dict = b"{\"seq\":1,\"code\":123,\"text\":\"RFC 1951 defines a lossless compressed data format using a combination of the LZ77 algorithm and Huffman coding.\"}\n".to_vec();
    let dict_target = b"{\"seq\":1,\"code\":123,\"text\":\"RFC 1951 defines a lossless compressed data format using a combination of the LZ77 algorithm and Huffman coding.\"}\n{\"seq\":2,\"code\":123,\"text\":\"RFC 1951 defines a lossless compressed data format\"}".to_vec();
    let mut dict_40k = vec![b'a'; 40000];
    for (i, b) in dict_40k.iter_mut().enumerate() {
        *b = b'a' + (i % 26) as u8;
    }
    dict_40k[40000 - 32768..40000 - 32768 + 38].copy_from_slice(b"WINDOW-START-32768-BOUNDARY-SENTINEL!!");
    dict_40k[40000 - 40..].copy_from_slice(b"WINDOW-END-40000-TAIL-SENTINEL-PAYLOAD!!");
    let dict_40k_target = b"lead WINDOW-START-32768-BOUNDARY-SENTINEL!! gap WINDOW-END-40000-TAIL-SENTINEL-PAYLOAD!! end".to_vec();

    let inputs: Vec<(&str, Vec<u8>, Vec<u8>)> = vec![
        ("empty", vec![], vec![]),
        ("one", b"A".to_vec(), vec![]),
        ("hello", b"Hello, RFC 1951 DEFLATE world!".to_vec(), vec![]),
        ("text_24k", text_24k.clone(), vec![]),
        ("rep_80k", rep_80k.clone(), vec![]),
        ("run_80k", run_80k.clone(), vec![]),
        ("geom_8k", geom_8k.clone(), vec![]),
        ("prng_4k", prng_4k.clone(), vec![]),
        ("prng_70k", prng_70k.clone(), vec![]),
        ("dict_short", dict_target.clone(), short_dict.clone()),
        ("dict_40k", dict_40k_target.clone(), dict_40k.clone()),
        ("dict_short_nodict", dict_target.clone(), vec![]),
    ];

    let levels: Vec<(&str, &str)> = vec![
        ("l0", "FlateNoCompression"),
        ("l1", "FlateBestSpeed"),
        ("l3", "(FlateLevel 3)"),
        ("l6", "FlateDefaultCompression"),
        ("l9", "FlateBestCompression"),
        ("huff", "FlateHuffmanOnly"),
    ];

    struct Job {
        name: String,
        plain_path: PathBuf,
        dict_path: PathBuf,
        out_path: PathBuf,
        plain_len: usize,
        has_dict: bool,
    }

    let mut jobs = Vec::new();
    let mut checks = Vec::new();

    for (in_name, plain, dict) in &inputs {
        let plain_path = scratch.path().join(format!("in_{in_name}.bin"));
        let dict_path = scratch.path().join(format!("dict_{in_name}.bin"));
        std::fs::write(&plain_path, plain).unwrap();
        std::fs::write(&dict_path, dict).unwrap();

        let lv_list: Vec<(&str, &str)> = if *in_name == "dict_short_nodict" {
            vec![("l6", "FlateDefaultCompression")]
        } else if *in_name == "prng_70k" {
            vec![("l0", "FlateNoCompression"), ("l1", "FlateBestSpeed"), ("l6", "FlateDefaultCompression")]
        } else {
            levels.clone()
        };

        for (lv_name, lv_expr) in lv_list {
            let job_name = format!("{in_name}_{lv_name}");
            let out_path = scratch.path().join(format!("out_{job_name}.flate"));
            checks.push(format!(
                "compress-and-verify {lv_expr} {} {} {}",
                cinch_str(&plain_path),
                cinch_str(&dict_path),
                cinch_str(&out_path)
            ));
            jobs.push(Job {
                name: job_name,
                plain_path: plain_path.clone(),
                dict_path: dict_path.clone(),
                out_path,
                plain_len: plain.len(),
                has_dict: !dict.is_empty(),
            });
        }
    }

    let main_src = format!(
        r#"-- | Compress every corpus across levels and preset dictionaries, verify Cinch roundtrip, and write outputs.
module Main where
  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Path
  import Data.Result
  import Effect.Fs
  import Text.Flate

  -- | Load a file from disk.
  load :: forall r. using (io :: Io r) => String ->{{IO r}} Bytes
  load p = case fs-read (path-from-string p) of
    Ok b -> b
    Err _ -> bytes-new 0

  -- | Compress plain with optional dict, write compressed stream to out-p, and verify self-roundtrip.
  compress-and-verify :: forall r. using (io :: Io r) => FlateCompression -> String -> String -> String ->{{IO r}} Bool
  compress-and-verify lv plain-p dict-p out-p =
    let plain = load plain-p in
    let dict = load dict-p in
    let comp = if bytes-len dict == 0 then flate-compress lv plain else flate-compress-dict lv dict plain in
    case comp of
      Err _ -> False
      Ok packed -> case fs-write (path-from-string out-p) packed of
        Err _ -> False
        Ok _ ->
          let dec = if bytes-len dict == 0 then flate-decompress packed else flate-decompress-dict dict packed in
          case dec of
            Err _ -> False
            Ok (got, consumed) -> got == plain && consumed == bytes-len packed

  -- | Return the index of the first failing job, or 200 when all pass.
  run-all :: forall r. using (io :: Io r) => List (Unit ->{{IO r}} Bool) -> Int ->{{IO r}} Int
  run-all xs i = case xs of
    Nil -> 200
    Cons f rest -> if f () then run-all rest (i + 1) else i

  -- | Run all compression jobs.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    run-all [{}] 0
"#,
        checks
            .iter()
            .map(|c| format!("(\\x -> {c})"))
            .collect::<Vec<_>>()
            .join(",\n      ")
    );

    let dir = flate_project("flate_encode", &main_src);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 200,
        "Cinch encode/self-roundtrip failed on job #{code} ({})",
        jobs.get(code as usize).map(|j| j.name.as_str()).unwrap_or("?")
    );

    // Verify REAL compression ratios and dynamic Huffman emission!
    let size_of = |name: &str| -> usize {
        let j = jobs.iter().find(|j| j.name == name).unwrap();
        std::fs::metadata(&j.out_path).unwrap().len() as usize
    };
    let bytes_of = |name: &str| -> Vec<u8> {
        let j = jobs.iter().find(|j| j.name == name).unwrap();
        std::fs::read(&j.out_path).unwrap()
    };

    let text_l6 = size_of("text_24k_l6");
    let text_l9 = size_of("text_24k_l9");
    let text_ratio = text_24k.len() as f64 / text_l6 as f64;
    assert!(
        text_ratio > 3.0,
        "expected >3x compression on varied text_24k at default level, got {:.2}x ({} -> {} bytes)",
        text_ratio,
        text_24k.len(),
        text_l6
    );
    assert!(
        text_l9 <= text_l6 + 64,
        "expected level 9 ({text_l9}) to be competitive with level 6 ({text_l6})"
    );
    // Verify dynamic Huffman block type (BTYPE = 2) is emitted on text_24k and geom_8k!
    assert_eq!((bytes_of("text_24k_l6")[0] >> 1) & 3, 2, "expected dynamic Huffman block on text_24k_l6");
    assert_eq!((bytes_of("geom_8k_huff")[0] >> 1) & 3, 2, "expected dynamic Huffman block on geom_8k_huff");

    let rep_l6 = size_of("rep_80k_l6");
    let rep_ratio = rep_80k.len() as f64 / rep_l6 as f64;
    assert!(
        rep_ratio > 50.0,
        "expected >50x compression on repeated pattern rep_80k, got {:.2}x ({} -> {} bytes)",
        rep_ratio,
        rep_80k.len(),
        rep_l6
    );

    let run_l6 = size_of("run_80k_l6");
    let run_ratio = run_80k.len() as f64 / run_l6 as f64;
    assert!(
        run_ratio > 200.0,
        "expected >200x compression on 80 KiB single-byte run, got {:.2}x ({} -> {} bytes)",
        run_ratio,
        run_80k.len(),
        run_l6
    );

    // Preset dictionary must strictly improve compression on dict_short!
    let with_dict = size_of("dict_short_l6");
    let without_dict = size_of("dict_short_nodict_l6");
    assert!(
        with_dict < without_dict,
        "expected preset dictionary to shrink compressed output: with_dict={with_dict} vs without_dict={without_dict}"
    );

    // Incompressible PRNG bytes must fall back to stored blocks (no significant expansion)!
    let prng_70k_l6 = size_of("prng_70k_l6");
    assert!(
        prng_70k_l6 <= prng_70k.len() + 20,
        "expected stored-block fallback on incompressible prng_70k, got {prng_70k_l6} bytes for {} input",
        prng_70k.len()
    );

    // Cross-check EVERY SINGLE Cinch-compressed stream with REAL Go 1.24 `compress/flate`!
    let verifier_path = scratch.path().join("verify_go.go");
    let manifest_path = scratch.path().join("manifest.json");
    let manifest: Vec<Value> = jobs
        .iter()
        .map(|j| {
            serde_json::json!({
                "name": j.name,
                "plain": j.plain_path.to_string_lossy(),
                "dict": if j.has_dict { j.dict_path.to_string_lossy().to_string() } else { String::new() },
                "packed": j.out_path.to_string_lossy(),
                "plain_len": j.plain_len,
            })
        })
        .collect();
    std::fs::write(&manifest_path, serde_json::to_string(&manifest).unwrap()).unwrap();
    std::fs::write(
        &verifier_path,
        r#"package main
import (
	"bytes"
	"compress/flate"
	"encoding/json"
	"fmt"
	"io"
	"os"
)
type Job struct {
	Name   string `json:"name"`
	Plain  string `json:"plain"`
	Dict   string `json:"dict"`
	Packed string `json:"packed"`
}
func main() {
	raw, err := os.ReadFile(os.Args[1])
	if err != nil { panic(err) }
	var jobs []Job
	if err := json.Unmarshal(raw, &jobs); err != nil { panic(err) }
	for _, j := range jobs {
		packed, err := os.ReadFile(j.Packed)
		if err != nil { panic(err) }
		want, err := os.ReadFile(j.Plain)
		if err != nil { panic(err) }
		var r io.ReadCloser
		if j.Dict != "" {
			dict, err := os.ReadFile(j.Dict)
			if err != nil { panic(err) }
			r = flate.NewReaderDict(bytes.NewReader(packed), dict)
		} else {
			r = flate.NewReader(bytes.NewReader(packed))
		}
		got, err := io.ReadAll(r)
		_ = r.Close()
		if err != nil {
			fmt.Fprintf(os.Stderr, "FAIL %s: Go flate.Reader error: %v\n", j.Name, err)
			os.Exit(1)
		}
		if !bytes.Equal(got, want) {
			fmt.Fprintf(os.Stderr, "FAIL %s: Go decoded %d bytes != expected %d bytes\n", j.Name, len(got), len(want))
			os.Exit(1)
		}
	}
	// Negative control: flipping a byte in a non-empty compressed stream must fail or mismatch in Go.
	for _, j := range jobs {
		if j.Name == "text_24k_l6" {
			packed, _ := os.ReadFile(j.Packed)
			want, _ := os.ReadFile(j.Plain)
			corrupt := append([]byte(nil), packed...)
			corrupt[len(corrupt)/2] ^= 0xFF
			r := flate.NewReader(bytes.NewReader(corrupt))
			got, err := io.ReadAll(r)
			_ = r.Close()
			if err == nil && bytes.Equal(got, want) {
				fmt.Fprintf(os.Stderr, "CONTROL FAIL: corrupted text_24k_l6 unexpectedly decoded identically in Go\n")
				os.Exit(1)
			}
		}
	}
	fmt.Printf("Go compress/flate verified %d Cinch-compressed streams bit-for-bit (+ negative control)!\n", len(jobs))
}
"#,
    )
    .unwrap();

    let go_out = Command::new("go")
        .env("GOCACHE", "/tmp/go-cache")
        .env("GOTELEMETRY", "off")
        .arg("run")
        .arg(&verifier_path)
        .arg(&manifest_path)
        .output()
        .expect("spawn go verifier");
    assert!(
        go_out.status.success(),
        "Go compress/flate verification failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&go_out.stdout),
        String::from_utf8_lossy(&go_out.stderr)
    );
    eprintln!(
        "{} (text_24k: {:.2}x, rep_80k: {:.2}x, run_80k: {:.2}x, dict: {} vs {} bytes)",
        String::from_utf8_lossy(&go_out.stdout).trim(),
        text_ratio,
        rep_ratio,
        run_ratio,
        with_dict,
        without_dict
    );
}

/// Effect.Flate streaming writer (with sync-flush, multi-block, preset dict, reset, finish)
/// and streaming reader (incremental small reads, fragmented 7-byte source, trailing bytes,
/// dict, output limit, corrupt/truncated error propagation) roundtrip and interop with Go.
#[test]
fn effect_flate_streaming_roundtrip_and_go_interop() {
    let scratch = tempfile::tempdir().unwrap();
    let stream_out_path = scratch.path().join("streamed_sync.flate");
    let big_input = repetitive_pattern(75000);
    let big_path = scratch.path().join("big_stream_in.bin");
    let big_out_path = scratch.path().join("big_stream.flate");
    std::fs::write(&big_path, &big_input).unwrap();

    let main_src = format!(
        r#"-- | Exercise Effect.Flate streaming writer and reader adapters and offset/dict-limit helpers.
module Main where
  import Data.Bytes
  import Data.Bytes.Build
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Flate
  import Effect.Fs
  import Effect.Stream
  import Text.Flate

  -- | Load a file from disk.
  load :: forall r. using (io :: Io r) => String ->{{IO r}} Bytes
  load p = case fs-read (path-from-string p) of
    Ok b -> b
    Err _ -> bytes-new 0

  -- | A fragmented source yielding at most 7 bytes per stream-read call.
  trickle-reader :: forall r. using (io :: Io r) => Bytes -> Nat -> ByteReader r
  trickle-reader buf pos = stream-reader (\n ->
    if pos >= bytes-len buf then (Ok (bytes-new 0), trickle-reader buf pos)
    else
      let rem = bytes-len buf - pos in
      let cap = if n < 7 then n else 7 in
      let take = if cap < rem then cap else rem in
      let out = bytes-new take in
      let u = if pos + take <= bytes-len buf then bytes-copy-range out buf 0 pos take else () in
      (Ok out, trickle-reader buf (pos + take)))

  -- | Test chunked streaming write + sync flush + finish + incremental small-chunk read + reset.
  test-sync-stream :: forall r. using (io :: Io r) => String ->{{IO r}} Bool
  test-sync-stream out-p =
    let c1 = string-to-bytes "First streaming chunk with repeated words words words!\n" in
    let c2 = string-to-bytes "Second streaming chunk referencing repeated words after sync flush!\n" in
    let c3 = string-to-bytes "Third final chunk completing the raw DEFLATE stream." in
    let full = string-to-bytes "First streaming chunk with repeated words words words!\nSecond streaming chunk referencing repeated words after sync flush!\nThird final chunk completing the raw DEFLATE stream." in
    case flate-writer FlateDefaultCompression writer-buffer of
      Err _ -> False
      Ok w0 -> case flate-write w0 c1 of
        (Err _, _) -> False
        (Ok n1, w1) -> case flate-flush w1 of
          (Err _, _) -> False
          (Ok _, w2) -> case flate-write w2 c2 of
            (Err _, _) -> False
            (Ok n2, w3) -> case flate-flush w3 of
              (Err _, _) -> False
              (Ok _, w4) -> case flate-write w4 c3 of
                (Err _, _) -> False
                (Ok n3, w5) -> case flate-finish w5 of
                  (Err _, _) -> False
                  (Ok _, w6) ->
                    -- Idempotent second finish, and write after finish yields StreamClosed.
                    case flate-finish w6 of
                      (Err _, _) -> False
                      (Ok _, w7) -> case flate-write w7 c1 of
                        (Err (StreamClosed _), _) ->
                          case writer-bytes (flate-writer-sink w7) of
                            None -> False
                            Some packed -> case fs-write (path-from-string out-p) packed of
                              Err _ -> False
                              Ok _ ->
                                -- Read back incrementally in tiny 13-byte chunks from a 7-byte fragmented source with trailing bytes!
                                let with-trail = bytes-concat [packed, string-to-bytes "TRAILING"] in
                                case stream-read-all 13 (flate-reader (trickle-reader with-trail 0)) of
                                  (Err _, _) -> False
                                  (Ok got, _) ->
                                    -- Also verify flate-decompress-at with leading prefix + trailing bytes!
                                    let framed = bytes-concat [string-to-bytes "HDR:", packed, string-to-bytes ":END"] in
                                    let at-ok = (case flate-decompress-at framed 4 of
                                      Ok (dec, next) -> dec == full && next == 4 + bytes-len packed
                                      Err _ -> False) in
                                    -- Also verify flate-writer-reset on a fresh sink!
                                    let w-reset = flate-writer-reset w7 writer-buffer in
                                    case flate-write w-reset c1 of
                                      (Err _, _) -> False
                                      (Ok _, wr1) -> case flate-finish wr1 of
                                        (Err _, _) -> False
                                        (Ok _, wr2) -> case writer-bytes (flate-writer-sink wr2) of
                                          None -> False
                                          Some p2 -> case stream-read-all 7 (flate-reader (reader-from-bytes p2)) of
                                            (Ok g2, _) -> got == full && at-ok && g2 == c1 && n1 == bytes-len c1 && n2 == bytes-len c2 && n3 == bytes-len c3
                                            _ -> False
                        _ -> False

  -- | Test preset dictionary streaming writer + reader and bounded dict decompression.
  test-dict-stream :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool
  test-dict-stream u =
    let dict = string-to-bytes "PRESET-DICTIONARY-CONTENT-FOR-STREAMING-DEFLATE-ADAPTERS-1234567890" in
    let msg = string-to-bytes "prefix PRESET-DICTIONARY-CONTENT-FOR-STREAMING-DEFLATE-ADAPTERS-1234567890 suffix" in
    case flate-writer-dict FlateBestCompression dict writer-buffer of
      Err _ -> False
      Ok w0 -> case flate-write w0 msg of
        (Err _, _) -> False
        (Ok _, w1) -> case flate-finish w1 of
          (Err _, _) -> False
          (Ok _, w2) -> case writer-bytes (flate-writer-sink w2) of
            None -> False
            Some packed ->
              let lim-over = (case flate-decompress-dict-limit 10 dict packed of Err FlateOutputLimitExceeded -> True; _ -> False) in
              let lim-ok = (case flate-decompress-dict-limit (bytes-len msg) dict packed of Ok (d, _) -> d == msg; _ -> False) in
              case stream-read-all 11 (flate-reader-dict dict (trickle-reader packed 0)) of
                (Ok got, _) -> got == msg && lim-over && lim-ok
                _ -> False

  -- | Test multi-block >64 KiB streaming roundtrip, output limit, and corrupt/truncated stream errors.
  test-large-and-errors :: forall r. using (io :: Io r) => String -> String ->{{IO r}} Bool
  test-large-and-errors in-p out-p =
    let big = load in-p in
    case flate-writer FlateDefaultCompression writer-buffer of
      Err _ -> False
      Ok w0 -> case flate-write w0 big of
        (Err _, _) -> False
        (Ok _, w1) -> case flate-finish w1 of
          (Err _, _) -> False
          (Ok _, w2) -> case writer-bytes (flate-writer-sink w2) of
            None -> False
            Some packed -> case fs-write (path-from-string out-p) packed of
              Err _ -> False
              Ok _ ->
                let full-ok = (case stream-read-all 4096 (flate-reader (reader-from-bytes packed)) of
                  (Ok got, _) -> got == big
                  _ -> False) in
                let limit-err = (case stream-read-all 4096 (flate-reader-limit 1000 (bytes-new 0) (reader-from-bytes packed)) of
                  (Err (StreamEncodingError _ _ _), _) -> True
                  _ -> False) in
                let trunc-err = (case stream-read-all 64 (flate-reader (reader-from-bytes (bytes-from-u8s [1, 4]))) of
                  (Err (StreamEncodingError _ _ _), _) -> True
                  _ -> False) in
                let corrupt-err = (case stream-read-all 64 (flate-reader (reader-from-bytes (bytes-from-u8s [7, 0]))) of
                  (Err (StreamEncodingError _ _ _), _) -> True
                  _ -> False) in
                full-ok && limit-err && trunc-err && corrupt-err

  -- | Return first failing index, or 200 when all pass.
  run-all :: forall r. using (io :: Io r) => List (Unit ->{{IO r}} Bool) -> Int ->{{IO r}} Int
  run-all xs i = case xs of
    Nil -> 200
    Cons f rest -> if f () then run-all rest (i + 1) else i

  -- | Run all streaming checks.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    run-all [
      (\x -> test-sync-stream {sync_out}),
      (\x -> test-dict-stream ()),
      (\x -> test-large-and-errors {big_in} {big_out})
    ] 0
"#,
        sync_out = cinch_str(&stream_out_path),
        big_in = cinch_str(&big_path),
        big_out = cinch_str(&big_out_path),
    );

    let dir = flate_project("flate_effect_stream", &main_src);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "Effect.Flate streaming test #{code} failed");

    // Cross-check BOTH streamed outputs with Go compress/flate!
    let verifier_path = scratch.path().join("verify_stream.go");
    std::fs::write(
        &verifier_path,
        r#"package main
import (
	"bytes"
	"compress/flate"
	"fmt"
	"io"
	"os"
)
func verify(packedPath string, want []byte) {
	packed, err := os.ReadFile(packedPath)
	if err != nil { panic(err) }
	r := flate.NewReader(bytes.NewReader(packed))
	got, err := io.ReadAll(r)
	_ = r.Close()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Go failed on %s: %v\n", packedPath, err)
		os.Exit(1)
	}
	if !bytes.Equal(got, want) {
		fmt.Fprintf(os.Stderr, "Go mismatch on %s: got %d want %d\n", packedPath, len(got), len(want))
		os.Exit(1)
	}
}
func main() {
	wantSync := []byte("First streaming chunk with repeated words words words!\nSecond streaming chunk referencing repeated words after sync flush!\nThird final chunk completing the raw DEFLATE stream.")
	verify(os.Args[1], wantSync)
	wantBig, err := os.ReadFile(os.Args[2])
	if err != nil { panic(err) }
	verify(os.Args[3], wantBig)
}
"#,
    )
    .unwrap();

    let go_out = Command::new("go")
        .env("GOCACHE", "/tmp/go-cache")
        .env("GOTELEMETRY", "off")
        .arg("run")
        .arg(&verifier_path)
        .arg(&stream_out_path)
        .arg(&big_path)
        .arg(&big_out_path)
        .output()
        .expect("spawn go stream verifier");
    assert!(
        go_out.status.success(),
        "Go verification of Effect.Flate streams failed:\n{}",
        String::from_utf8_lossy(&go_out.stderr)
    );
}
