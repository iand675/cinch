//! Text.Zlib: the DEFLATE decoder and the stored-block encoder.
//!
//! Both directions are pinned against something that does not share
//! this code, because a decoder that merely agrees with its own
//! encoder proves nothing:
//!
//!   * `inflate` decodes streams a REAL encoder wrote. `/usr/bin/python3`
//!     compresses the fixtures (level 0 for stored blocks, levels 1 and
//!     9 for dynamic Huffman codes, `zlib.compressobj(..., Z_FIXED)` for
//!     a fixed-Huffman block), and every fixture's block type is
//!     asserted as generated, so the three block types are covered by
//!     construction rather than by hope. The fixtures also cover the
//!     empty stream, a single byte, a max-length stored block (LEN
//!     65535, byte-built here), the 65535-byte stored-block boundary and
//!     a 105000-byte highly repetitive input (long back-references over
//!     dynamic codes).
//!   * `deflate` output is decoded by REAL decoder: the Cinch program
//!     writes each stream to a file and the test hands it to
//!     `zlib.decompress` in `/usr/bin/python3`, and a control asserts
//!     that the same call REJECTS a copy of that stream with one
//!     flipped Adler-32 byte, so its acceptance measures the stream and
//!     not the harness. The Cinch side pins `inflate (deflate x) == x`
//!     for the same inputs.
//!   * `adler32` and `crc32` are pinned to published vectors
//!     (Adler-32 of "Wikipedia", CRC-32 of "123456789", both empty
//!     inputs) and to the checksums of the byte-built inputs, which the
//!     same python3 verifies when it is present.
//!   * Every named refusal is exercised: a broken FCHECK pair, an FDICT
//!     byte, a CM other than 8 (all `BadHeader`), a flipped Adler-32
//!     byte (`BadChecksum`), a stream cut mid-block and one whose
//!     Adler-32 is missing (`TruncatedStream`), and the reserved block
//!     type BTYPE 3 (`InvalidCode`).
//!
//! Each test compiles a real project: the module under test written
//! beside a `Main`, driven through the CLI pipeline (`driver`). The
//! Cinch programs return a bitmask (one bit per check, `main` sums
//! them), so a nonzero exit names the failed check. Compilation runs on
//! a wide-stack thread because the checker's per-expression recursion
//! sits near the default test stack (D337).
//!
//! If `/usr/bin/python3` is missing (or has no `zlib`), the assertions
//! that need the real encoder and decoder are SKIPPED with a message:
//! the byte-built stored fixture, the round trip, the checksum vectors
//! and every error path still run.

use cinch::driver::{driver, Mode};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

const ZLIB: &str = include_str!("../prelude/Text/Zlib.cinch");

// ---------------------------------------------------------------- fixtures

/// The varied text the dynamic-Huffman fixtures compress: a small
/// vocabulary cycled by a fixed LCG, which gives the encoder skewed
/// enough frequencies to reach for dynamic codes.
fn words_text() -> Vec<u8> {
    let words = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta"];
    let mut x: u64 = 7;
    let mut out = String::new();
    for i in 0..2000 {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        out.push_str(words[((x >> 33) % 8) as usize]);
        if i + 1 < 2000 {
            out.push(' ');
        }
    }
    out.into_bytes()
}

/// Exactly 65535 bytes (the largest stored block) of incompressible-ish
/// data.
fn boundary_bytes() -> Vec<u8> {
    (0..65535u32).map(|i| (i % 251) as u8).collect()
}

/// A highly repetitive 105000-byte input: long matches at a short
/// distance, which is what forces the long back-references.
fn repetitive_bytes() -> Vec<u8> {
    b"abcdefghijklmnopqrstu".repeat(5000)
}

/// The zlib stream a stored-block encoder writes for `data`, built here
/// so the checksum and error fixtures need no encoder at all: the
/// header 0x78 0x01, one stored block per 65535-byte chunk, and the
/// Adler-32 of `data` big-endian.
fn stored_stream(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let mut i = 0usize;
    loop {
        let end = (i + 65535).min(data.len());
        let chunk = &data[i..end];
        let last = end == data.len();
        out.push(u8::from(last));
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
        i = end;
        if last {
            break;
        }
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + u32::from(x)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// A fixture's first block type: BFINAL in bit 0 of byte 2, BTYPE in
/// bits 1-2.
fn block_type(stream: &[u8]) -> u8 {
    (stream[2] >> 1) & 3
}

// ---------------------------------------------------------------- python3

fn py_path() -> Option<PathBuf> {
    let p = PathBuf::from("/usr/bin/python3");
    if !p.exists() {
        return None;
    }
    let ok = Command::new(&p)
        .args(["-c", "import zlib"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    ok.then_some(p)
}

fn py_run(py: &Path, code: &str, input: &[u8], args: &[&Path]) -> Vec<u8> {
    let mut cmd = Command::new(py);
    cmd.arg("-c").arg(code);
    for a in args {
        cmd.arg(a);
    }
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    child
        .stdin
        .take()
        .expect("python3 stdin")
        .write_all(input)
        .expect("write python3 stdin");
    let out = child.wait_with_output().expect("python3 output");
    assert!(
        out.status.success(),
        "python3 failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

/// The real encoder at `level` (0 stored, 1 and 9 dynamic).
fn py_compress(py: &Path, data: &[u8], level: u32) -> Vec<u8> {
    let code = format!(
        "import sys,zlib; sys.stdout.buffer.write(zlib.compress(sys.stdin.buffer.read(), {level}))"
    );
    py_run(py, &code, data, &[])
}

/// The real encoder forced to fixed Huffman codes (Z_FIXED).
fn py_compress_fixed(py: &Path, data: &[u8]) -> Vec<u8> {
    py_run(
        py,
        "import sys,zlib; d=sys.stdin.buffer.read(); c=zlib.compressobj(9,zlib.DEFLATED,15,9,zlib.Z_FIXED); sys.stdout.buffer.write(c.compress(d)+c.flush())",
        data,
        &[],
    )
}

/// The real decoder over a file (the same call the acceptance names).
fn py_decompress(py: &Path, path: &Path) -> Vec<u8> {
    py_run(
        py,
        "import zlib,sys; sys.stdout.buffer.write(zlib.decompress(open(sys.argv[1],'rb').read()))",
        b"",
        &[path],
    )
}

/// Whether the real decoder REJECTS the file (the control for the
/// accepted-stream checks: a decoder that accepts anything measures
/// nothing).
fn py_decompress_rejected(py: &Path, path: &Path) -> bool {
    let status = Command::new(py)
        .arg("-c")
        .arg("import zlib,sys; zlib.decompress(open(sys.argv[1],'rb').read())")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("spawn python3");
    !status.success()
}

// ---------------------------------------------------------------- driver

fn scratch(tag: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "cinch_zlib_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        tag
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn write_fixture(dir: &Path, name: &str, bytes: &[u8]) {
    std::fs::write(dir.join(name), bytes).expect("write fixture");
}

/// A path as a Cinch string literal.
fn lit(p: &Path) -> String {
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

fn path_lit(dir: &Path, name: &str) -> String {
    lit(&dir.join(name))
}

/// Compile and run `main_src` (with `Text.Zlib` written beside it) and
/// return the program's exit code. The compilation runs on a 64 MiB
/// stack: the checker's per-expression recursion sits near the default
/// test thread's 2 MiB (D337), and this module's block decoders nest
/// deeper than the prelude baseline.
fn run_project(name: &str, dir: &Path, main_src: &str) -> i32 {
    write_fixture(dir, "Zlib.cinch", ZLIB.as_bytes());
    let main_path = dir.join("Main.cinch");
    std::fs::write(&main_path, main_src).expect("write Main.cinch");
    let compiled = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &main_path, None).map_err(|e| e.to_string()))
        .expect("spawn compile thread")
        .join()
        .expect("compile thread panicked");
    match compiled {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// The shared Cinch preamble: reading a fixture, comparing buffers.
const PREAMBLE: &str = r#"module Main where

  import Data.Bytes
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.Path
  import Data.String
  import Data.Machine
  import Effect.Fs
  import Text.Zlib

  read-bytes :: forall r. using (io :: Io r) => String ->{IO r} Option Bytes
  read-bytes p = case fs-read (path-from-string p) of
    Err e -> None
    Ok b -> Some b

  bytes-same :: Bytes -> Bytes -> Bool
  bytes-same a b = case bytes-len a == bytes-len b of
    False -> False
    True -> bytes-same-at a b 0

  bytes-same-at :: Bytes -> Bytes -> Nat -> Bool
  bytes-same-at a b i = case i >= bytes-len a of
    True -> True
    False -> case bytes-get a i == bytes-get b i of
      True -> bytes-same-at a b (i + 1)
      False -> False
"#;

/// The `main` declaration that sums the checks (one bit per check, so a
/// nonzero exit names the failed one).
fn main_body(checks: &[String]) -> String {
    let mut body = String::new();
    let mut sum = String::new();
    for (i, c) in checks.iter().enumerate() {
        body.push_str(&format!("    let c{i} = {c} in\n"));
        if i > 0 {
            sum.push_str(" + ");
        }
        sum.push_str(&format!("c{i}"));
    }
    format!("  main :: Unit ->{{IO r}} Int\n  main u = with (io = newIO ()) in\n{body}    {sum}\n")
}

// ---------------------------------------------------------------- inflate

/// One fixture check: the compressed stream, the plaintext it must
/// decode to, and the name a failure message uses. A case's exit-code
/// bit is its position in its batch (see `bit_code`).
struct Case {
    bin: String,
    want: String,
    label: String,
}

/// The exit code is the OR of the failed checks' bits (one bit per
/// check, so a nonzero code names which ones). At most eight checks fit
/// one program's status byte, so callers batch.
fn bit_code(cases: &[Case]) -> Vec<(u32, String, String)> {
    cases
        .iter()
        .enumerate()
        .map(|(k, c)| (1u32 << k, c.bin.clone(), c.want.clone()))
        .collect()
}

/// The cases a nonzero `code` names (its bits are the batch positions).
fn failed_labels(code: i32, cases: &[Case]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (k, c) in cases.iter().enumerate() {
        if code & (1 << k) != 0 {
            out.push(c.label.clone());
        }
    }
    if code != 0 && out.is_empty() {
        out.push(format!("exit {code} (no check bit)"));
    }
    out
}

fn inflate_program(checks: &[(u32, String, String)]) -> String {
    let mut src = String::from(PREAMBLE);
    src.push_str(
        "\n  -- `bit` when inflating the stream in `bin` does not yield exactly the\n\
         \x20 -- bytes of `want`, 0 when it does (the bit names the failed fixture).\n\
         \x20 check-file :: forall r. using (io :: Io r) => Int -> String -> String ->{IO r} Int\n\
         \x20 check-file bit bin want = case read-bytes bin of\n\
         \x20   None -> bit\n\
         \x20   Some c -> case read-bytes want of\n\
         \x20     None -> bit\n\
         \x20     Some w -> case inflate c of\n\
         \x20       Err e -> bit\n\
         \x20       Ok got -> case bytes-same got w of\n\
         \x20         True -> 0\n\
         \x20         False -> bit\n",
    );
    let body: Vec<String> = checks
        .iter()
        .map(|(bit, b, w)| format!("check-file {bit} {b} {w}"))
        .collect();
    src.push_str(&main_body(&body));
    src
}

/// Run the fixture cases (five per program) and panic naming every case
/// the program reported failed.
fn expect_inflate_ok(dir: &Path, cases: &[Case]) {
    let mut failed: Vec<String> = Vec::new();
    for (i, chunk) in cases.chunks(5).enumerate() {
        let code = run_project(
            &format!("zlib-inflate-{i}"),
            dir,
            &inflate_program(&bit_code(chunk)),
        );
        failed.extend(failed_labels(code, chunk));
    }
    assert!(
        failed.is_empty(),
        "inflate did not reproduce the fixture plaintexts for: {}",
        failed.join(", ")
    );
}

#[test]
fn inflate_decodes_real_encoder_streams() {
    let dir = scratch("inflate");
    let words = words_text();
    let big = boundary_bytes();
    let rep = repetitive_bytes();
    write_fixture(&dir, "words.txt", &words);
    write_fixture(&dir, "big.txt", &big);
    write_fixture(&dir, "rep.txt", &rep);
    write_fixture(&dir, "empty.txt", b"");
    write_fixture(&dir, "one.txt", b"A");

    let mut cases: Vec<Case> = Vec::new();
    // Byte-built stored streams first, so the stored path, the
    // max-length stored block (LEN 65535) and the stored-block
    // boundary are covered even with no python3 on the machine.
    let max_stored = stored_stream(&big);
    assert_eq!(block_type(&max_stored), 0, "the byte-built stream must be a stored block");
    assert_eq!(
        u16::from_le_bytes([max_stored[3], max_stored[4]]),
        65535,
        "the byte-built boundary fixture must carry the maximum stored LEN"
    );
    write_fixture(&dir, "max-stored.z", &max_stored);
    cases.push(Case {
        bin: path_lit(&dir, "max-stored.z"),
        want: path_lit(&dir, "big.txt"),
        label: "the byte-built max-length stored stream (LEN 65535)".to_string(),
    });
    write_fixture(&dir, "words-stored.z", &stored_stream(&words));
    cases.push(Case {
        bin: path_lit(&dir, "words-stored.z"),
        want: path_lit(&dir, "words.txt"),
        label: "the byte-built stored stream".to_string(),
    });

    match py_path() {
        None => eprintln!(
            "lib_zlib: /usr/bin/python3 with zlib is missing; the real-encoder fixtures are skipped (the byte-built stored fixtures still run)"
        ),
        Some(py) => {
            // The real encoder, one fixture per block type and per
            // required boundary. Level 0 emits stored blocks, levels 1
            // and 9 dynamic-Huffman ones, Z_FIXED a fixed-Huffman one:
            // each type is asserted as generated, so the three block
            // types cannot silently collapse into one.
            let fixtures: Vec<(&str, Vec<u8>, &str, u8, &str)> = vec![
                ("words-0.z", py_compress(&py, &words, 0), "words.txt", 0, "level 0"),
                ("words-1.z", py_compress(&py, &words, 1), "words.txt", 2, "level 1"),
                ("words-9.z", py_compress(&py, &words, 9), "words.txt", 2, "level 9"),
                ("words-fixed.z", py_compress_fixed(&py, &words), "words.txt", 1, "Z_FIXED"),
                ("empty-9.z", py_compress(&py, b"", 9), "empty.txt", 1, "level 9 of the empty input"),
                ("one-9.z", py_compress(&py, b"A", 9), "one.txt", 1, "level 9 of one byte"),
                ("big-0.z", py_compress(&py, &big, 0), "big.txt", 0, "level 0 of 65535 bytes"),
                ("rep-9.z", py_compress(&py, &rep, 9), "rep.txt", 2, "level 9 of 105000 repetitive bytes"),
            ];
            for (name, stream, want, want_type, what) in &fixtures {
                assert_eq!(
                    block_type(stream),
                    *want_type,
                    "{what} did not emit the block type this test needs (got {})",
                    block_type(stream)
                );
                write_fixture(&dir, name, stream);
                // The fixture is what it claims to be: the real decoder
                // recovers the plaintext from it.
                let expected = std::fs::read(dir.join(want)).expect("plaintext fixture");
                assert_eq!(
                    py_decompress(&py, &dir.join(name)),
                    expected,
                    "{name} does not decode to {want} under the real decoder"
                );
                cases.push(Case {
                    bin: path_lit(&dir, name),
                    want: path_lit(&dir, want),
                    label: format!("{name} ({what})"),
                });
            }
        }
    }

    expect_inflate_ok(&dir, &cases);
}

// ---------------------------------------------------------------- deflate

const DEFLATE_CHECK: &str = r#"
  -- `bit` when `deflate`'s stream for `plain` cannot be written to `out`
  -- or does not inflate back to `plain`, 0 when both hold.
  check-round :: forall r. using (io :: Io r) => Int -> String -> String ->{IO r} Int
  check-round bit plain out = case read-bytes plain of
    None -> bit
    Some b ->
      let s = deflate b in
      case fs-write (path-from-string out) s of
        Err e -> bit
        Ok _ -> case inflate s of
          Err e -> bit
          Ok got -> case bytes-same got b of
            True -> 0
            False -> bit
"#;

#[test]
fn deflate_streams_are_legal_and_decode_under_a_real_decoder() {
    let dir = scratch("deflate");
    let words = words_text();
    let big = boundary_bytes();
    let rep = repetitive_bytes();
    write_fixture(&dir, "words.txt", &words);
    write_fixture(&dir, "big.txt", &big);
    write_fixture(&dir, "rep.txt", &rep);
    write_fixture(&dir, "empty.txt", b"");
    write_fixture(&dir, "one.txt", b"A");

    let mut src = String::from(PREAMBLE);
    src.push_str(DEFLATE_CHECK);
    let pairs: Vec<(&str, &str)> = vec![
        ("words.txt", "out-words.z"),
        ("big.txt", "out-big.z"),
        ("rep.txt", "out-rep.z"),
        ("empty.txt", "out-empty.z"),
        ("one.txt", "out-one.z"),
    ];
    let checks: Vec<String> = pairs
        .iter()
        .enumerate()
        .map(|(k, (plain, out))| {
            format!(
                "check-round {} {} {}",
                1u32 << k,
                path_lit(&dir, plain),
                path_lit(&dir, out)
            )
        })
        .collect();
    src.push_str(&main_body(&checks));

    let code = run_project("zlib-deflate", &dir, &src);
    let failed: Vec<String> = pairs
        .iter()
        .enumerate()
        .filter(|(k, _)| code & (1 << k) != 0)
        .map(|(_, (plain, out))| format!("{plain} -> {out}"))
        .collect();
    assert!(
        failed.is_empty(),
        "deflate's stream did not round-trip for: {} (exit {code})",
        failed.join(", ")
    );

    match py_path() {
        None => eprintln!(
            "lib_zlib: /usr/bin/python3 with zlib is missing; the real-decoder check of deflate's output is skipped (the round trips above still ran)"
        ),
        Some(py) => {
            for (plain, out) in &pairs {
                let want = std::fs::read(dir.join(plain)).expect("plaintext fixture");
                let got = py_decompress(&py, &dir.join(out));
                assert_eq!(
                    got, want,
                    "python3 zlib.decompress could not recover {plain} from the stream Text.Zlib.deflate wrote ({out})"
                );
            }
            // The control: the same call must REJECT a corrupted copy of
            // one of those streams, so "python3 decoded it" is a
            // property of the stream and not of the harness.
            let mut corrupted = std::fs::read(dir.join("out-words.z")).expect("deflate output");
            let last = corrupted.len() - 1;
            corrupted[last] ^= 0x01;
            write_fixture(&dir, "out-words-corrupt.z", &corrupted);
            assert!(
                py_decompress_rejected(&py, &dir.join("out-words-corrupt.z")),
                "python3 accepted a stream with a corrupted Adler-32, so its acceptance above proves nothing"
            );
        }
    }
}

// ---------------------------------------------------------------- errors

const ERROR_CHECKS: &str = r#"
  -- `bit` when `inflate` does not refuse the stream in `p` with
  -- `BadHeader`, 0 when it does.
  check-bad-header :: forall r. using (io :: Io r) => Int -> String ->{IO r} Int
  check-bad-header bit p = case read-bytes p of
    None -> bit
    Some b -> case inflate b of
      Ok _ -> bit
      Err e -> case e of
        BadHeader -> 0
        _ -> bit

  -- `bit` when `inflate` does not refuse the stream in `p` with
  -- `BadChecksum`, 0 when it does.
  check-bad-checksum :: forall r. using (io :: Io r) => Int -> String ->{IO r} Int
  check-bad-checksum bit p = case read-bytes p of
    None -> bit
    Some b -> case inflate b of
      Ok _ -> bit
      Err e -> case e of
        BadChecksum -> 0
        _ -> bit

  -- `bit` when `inflate` does not refuse the stream in `p` with
  -- `TruncatedStream`, 0 when it does.
  check-truncated :: forall r. using (io :: Io r) => Int -> String ->{IO r} Int
  check-truncated bit p = case read-bytes p of
    None -> bit
    Some b -> case inflate b of
      Ok _ -> bit
      Err e -> case e of
        TruncatedStream -> 0
        _ -> bit

  -- `bit` when `inflate` does not refuse the stream in `p` with
  -- `InvalidCode`, 0 when it does.
  check-invalid :: forall r. using (io :: Io r) => Int -> String ->{IO r} Int
  check-invalid bit p = case read-bytes p of
    None -> bit
    Some b -> case inflate b of
      Ok _ -> bit
      Err e -> case e of
        InvalidCode -> 0
        _ -> bit
"#;

#[test]
fn inflate_names_every_error_path() {
    let dir = scratch("errors");
    let big = boundary_bytes();
    let valid = stored_stream(&big);

    // Header refusals: the FCHECK pair not a multiple of 31, FDICT set
    // (with FCHECK still satisfied, so the refusal can only be FDICT's),
    // and CM other than 8 (again with FCHECK satisfied).
    let mut bad_fcheck = valid.clone();
    bad_fcheck[1] = 0x00;
    write_fixture(&dir, "bad-fcheck.z", &bad_fcheck);
    let mut bad_fdict = valid.clone();
    bad_fdict[1] = 0x20;
    write_fixture(&dir, "bad-fdict.z", &bad_fdict);
    let mut bad_cm = valid.clone();
    bad_cm[0] = 0x79;
    bad_cm[1] = 0x18;
    write_fixture(&dir, "bad-cm.z", &bad_cm);
    assert_ne!(0x7800u16 % 31, 0, "the FCHECK fixture must break the pair's multiple-of-31 rule");
    assert_eq!(0x7820u16 % 31, 0, "the FDICT fixture must keep FCHECK satisfied");
    assert_eq!(0x7918u16 % 31, 0, "the CM fixture must keep FCHECK satisfied");

    // A flipped Adler-32 byte: the decoded bytes are right and only the
    // checksum is wrong.
    let mut bad_adler = valid.clone();
    let last = bad_adler.len() - 1;
    bad_adler[last] ^= 0x01;
    write_fixture(&dir, "bad-adler.z", &bad_adler);

    // Truncation: the Adler-32 missing, and a stream cut inside the
    // stored block's data.
    write_fixture(&dir, "trunc-adler.z", &valid[..valid.len() - 4]);
    write_fixture(&dir, "trunc-mid.z", &valid[..100]);

    // The reserved block type: BFINAL kept, BTYPE forced to 3.
    let mut bad_type = valid.clone();
    bad_type[2] |= 0x06;
    write_fixture(&dir, "bad-type.z", &bad_type);

    let errors: Vec<(&str, &str, &str)> = vec![
        ("the FCHECK pair is not a multiple of 31", "check-bad-header", "bad-fcheck.z"),
        ("FDICT is set", "check-bad-header", "bad-fdict.z"),
        ("CM is not 8", "check-bad-header", "bad-cm.z"),
        ("the trailing Adler-32 byte was flipped", "check-bad-checksum", "bad-adler.z"),
        ("the Adler-32 is missing", "check-truncated", "trunc-adler.z"),
        ("the stream is cut inside the stored block", "check-truncated", "trunc-mid.z"),
        ("the reserved block type BTYPE 3", "check-invalid", "bad-type.z"),
    ];
    let checks: Vec<String> = errors
        .iter()
        .enumerate()
        .map(|(k, (_, check, name))| {
            format!("{check} {} {}", 1u32 << k, path_lit(&dir, name))
        })
        .collect();
    let mut src = String::from(PREAMBLE);
    src.push_str(ERROR_CHECKS);
    src.push_str(&main_body(&checks));

    let code = run_project("zlib-errors", &dir, &src);
    let failed: Vec<String> = errors
        .iter()
        .enumerate()
        .filter(|(k, _)| code & (1 << k) != 0)
        .map(|(_, (label, check, name))| format!("{name} ({label}, expected {check} to accept the refusal)"))
        .collect();
    assert!(
        failed.is_empty(),
        "inflate did not name the expected refusal for: {} (exit {code})",
        failed.join(", ")
    );
}

// ---------------------------------------------------------------- checksums

const CHECKSUM_CHECKS: &str = r#"
  -- `bit` when Adler-32 of the file in `p` is not `want`, 0 when it is.
  check-adler-file :: forall r. using (io :: Io r) => Int -> String -> U32 ->{IO r} Int
  check-adler-file bit p want = case read-bytes p of
    None -> bit
    Some b -> case adler32 b == want of
      True -> 0
      False -> bit

  -- `bit` when CRC-32 of the file in `p` is not `want`, 0 when it is.
  check-crc-file :: forall r. using (io :: Io r) => Int -> String -> U32 ->{IO r} Int
  check-crc-file bit p want = case read-bytes p of
    None -> bit
    Some b -> case crc32 b == want of
      True -> 0
      False -> bit

  -- `bit` when Adler-32 of the literal text is not `want`, 0 when it is.
  check-adler-text :: Int -> String -> U32 -> Int
  check-adler-text bit s want = case adler32 (string-to-bytes s) == want of
    True -> 0
    False -> bit

  -- `bit` when CRC-32 of the literal text is not `want`, 0 when it is.
  check-crc-text :: Int -> String -> U32 -> Int
  check-crc-text bit s want = case crc32 (string-to-bytes s) == want of
    True -> 0
    False -> bit
"#;

/// The published vectors, plus this test's own inputs (verified against
/// python3 when it is present).
const WIKIPEDIA_ADLER: u32 = 0x11E6_0398;
const WIKIPEDIA_CRC: u32 = 0xADAA_C02E;
const CRC_CHECK_789: u32 = 0xCBF4_3926;
const HELLO_ADLER: u32 = 0x1A0B_045D;
const HELLO_CRC: u32 = 0x0D4A_1185;
const WORDS_ADLER: u32 = 0xBC96_3C71;
const WORDS_CRC: u32 = 0x05F3_D47E;
const BIG_ADLER: u32 = 0x76E7_FC24;
const BIG_CRC: u32 = 0xCF37_1872;
const REP_ADLER: u32 = 0x070D_78BE;
const REP_CRC: u32 = 0x1AF2_75DA;

#[test]
fn checksums_match_published_vectors_and_real_checksums() {
    let dir = scratch("checksums");
    let words = words_text();
    let big = boundary_bytes();
    let rep = repetitive_bytes();
    write_fixture(&dir, "words.txt", &words);
    write_fixture(&dir, "big.txt", &big);
    write_fixture(&dir, "rep.txt", &rep);

    if let Some(py) = py_path() {
        // The constants above are this test's fixtures too: the real
        // library is the authority, so a disagreement is a failure.
        let fixtures: [(&str, &[u8], u32, u32); 5] = [
            ("words.txt", &words, WORDS_ADLER, WORDS_CRC),
            ("big.txt", &big, BIG_ADLER, BIG_CRC),
            ("rep.txt", &rep, REP_ADLER, REP_CRC),
            ("hello world", b"hello world", HELLO_ADLER, HELLO_CRC),
            ("Wikipedia", b"Wikipedia", WIKIPEDIA_ADLER, WIKIPEDIA_CRC),
        ];
        let parse = |out: &[u8]| -> u32 {
            String::from_utf8_lossy(out)
                .trim()
                .parse::<u32>()
                .expect("python3 checksum")
        };
        for (what, data, adler, crc) in fixtures {
            let py_adler = py_run(
                py.as_path(),
                "import sys,zlib; print(zlib.adler32(sys.stdin.buffer.read()))",
                data,
                &[],
            );
            let py_crc = py_run(
                py.as_path(),
                "import sys,zlib; print(zlib.crc32(sys.stdin.buffer.read()))",
                data,
                &[],
            );
            assert_eq!(parse(&py_adler), adler, "adler32({what}) fixture constant is wrong");
            assert_eq!(parse(&py_crc), crc, "crc32({what}) fixture constant is wrong");
        }
        let py_crc_789 = py_run(
            py.as_path(),
            "import sys,zlib; print(zlib.crc32(b'123456789'))",
            b"",
            &[],
        );
        assert_eq!(parse(&py_crc_789), CRC_CHECK_789, "the CRC-32 check value is wrong");
        let py_adler_empty = py_run(
            py.as_path(),
            "import sys,zlib; print(zlib.adler32(b''))",
            b"",
            &[],
        );
        assert_eq!(parse(&py_adler_empty), 1, "Adler-32 of the empty input must be 1");
        let py_crc_empty = py_run(
            py.as_path(),
            "import sys,zlib; print(zlib.crc32(b''))",
            b"",
            &[],
        );
        assert_eq!(parse(&py_crc_empty), 0, "CRC-32 of the empty input must be 0");
    } else {
        eprintln!("lib_zlib: /usr/bin/python3 with zlib is missing; the fixture checksums are assumed (the vectors below still pin the implementation)");
    }

    // Every check is a Cinch expression with `{b}` standing for its bit
    // in the batch (published vectors first, then the byte-built
    // inputs' checksums).
    let vectors: Vec<(String, String)> = vec![
        (
            "the published Adler-32 of \"Wikipedia\"".to_string(),
            "check-adler-text {b} \"Wikipedia\" 0x11E60398".to_string(),
        ),
        (
            "Adler-32 of the empty input must be 1".to_string(),
            "check-adler-text {b} \"\" 1".to_string(),
        ),
        (
            "the published CRC-32 check value of \"123456789\"".to_string(),
            "check-crc-text {b} \"123456789\" 0xCBF43926".to_string(),
        ),
        (
            "CRC-32 of the empty input must be 0".to_string(),
            "check-crc-text {b} \"\" 0".to_string(),
        ),
        (
            "Adler-32 of \"hello world\"".to_string(),
            "check-adler-text {b} \"hello world\" 0x1A0B045D".to_string(),
        ),
        (
            "CRC-32 of \"hello world\"".to_string(),
            "check-crc-text {b} \"hello world\" 0x0D4A1185".to_string(),
        ),
        (
            "the published CRC-32 of \"Wikipedia\"".to_string(),
            "check-crc-text {b} \"Wikipedia\" 0xADAAC02E".to_string(),
        ),
        (
            "Adler-32 of the 11469-byte varied text".to_string(),
            format!("check-adler-file {{b}} {} 0xBC963C71", path_lit(&dir, "words.txt")),
        ),
        (
            "CRC-32 of the 11469-byte varied text".to_string(),
            format!("check-crc-file {{b}} {} 0x05F3D47E", path_lit(&dir, "words.txt")),
        ),
        (
            "Adler-32 of the 65535-byte input".to_string(),
            format!("check-adler-file {{b}} {} 0x76E7FC24", path_lit(&dir, "big.txt")),
        ),
        (
            "CRC-32 of the 65535-byte input".to_string(),
            format!("check-crc-file {{b}} {} 0xCF371872", path_lit(&dir, "big.txt")),
        ),
        (
            "Adler-32 of the 105000-byte repetitive input".to_string(),
            format!("check-adler-file {{b}} {} 0x070D78BE", path_lit(&dir, "rep.txt")),
        ),
        (
            "CRC-32 of the 105000-byte repetitive input".to_string(),
            format!("check-crc-file {{b}} {} 0x1AF275DA", path_lit(&dir, "rep.txt")),
        ),
    ];

    let mut failed: Vec<String> = Vec::new();
    for (i, chunk) in vectors.chunks(6).enumerate() {
        let checks: Vec<String> = chunk
            .iter()
            .enumerate()
            .map(|(k, (_, body))| body.replace("{b}", &(1u32 << k).to_string()))
            .collect();
        let mut src = String::from(PREAMBLE);
        src.push_str(CHECKSUM_CHECKS);
        src.push_str(&main_body(&checks));
        let code = run_project(&format!("zlib-checksums-{i}"), &dir, &src);
        for (k, (label, _)) in chunk.iter().enumerate() {
            if code & (1 << k) != 0 {
                failed.push(label.clone());
            }
        }
        if code != 0 && failed.is_empty() {
            failed.push(format!("batch {i} exit {code} (no check bit)"));
        }
    }
    assert!(
        failed.is_empty(),
        "the checksums did not match for: {}",
        failed.join(", ")
    );
}
