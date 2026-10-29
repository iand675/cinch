//! `Data.StringIo` consumer semantics (P40, the Ruby `StringIO` port
//! named in design/stdlib-parity-ruby.md's collections/text-utilities
//! bullet): the in-memory byte buffer and its read/write cursor.
//!
//! Each test compiles a real two-module project (the module under test
//! beside `Main.cinch`) in a fresh temp directory and runs it through
//! the same `driver` the CLI uses. The checks are the module's
//! consumer-visible semantics:
//!   write then read round trips with the cursor moved between them,
//!     including a read that overwrites at the cursor and one that
//!     extends past the end (a write leaves the cursor at its end, so
//!     the reads go through `rewind` or `seek`);
//!   `read-line` over a buffer with and without a trailing LF, over an
//!     empty buffer, and over leading LFs (one-byte lines);
//!   a read at end of buffer returning empty WITHOUT moving the cursor,
//!     the same for a zero-byte read at the start, and a read larger
//!     than what remains taking the remainder;
//!   `rewind` restoring the initial state, with the conversions' copy
//!     discipline (in and out) observable;
//!   `seek`'s named refusal past the end, and the accepted target at
//!     exactly the length;
//!   `truncate` shrinking and clamping the cursor, and its no-op above
//!     the length.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit
//! and `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks: check k contributes bit 2^(k-1) and every failure
//! mode of one check shares its bit (the sub-conditions join with
//! `&&`, so one check never contributes more than one bit). The sum
//! stays inside the 8-bit exit status. The last test asserts an exact
//! nonzero bit for a false claim, so the naming is observable: a check
//! that could never fire would be a tautology.
//!
//! The checks are top-level because a `case` in a `let` binder's
//! right-hand side must indent past the binder's column under the
//! layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const STRING_IO: &str = include_str!("../prelude/Data/StringIo.cinch");

/// Structural equality on `Bytes` (the prelude has no `Eq Bytes`
/// instance) plus the byte constructors and the infallible seek the
/// checks share.
const HELPERS: &str = r#"
  bytes-of :: List Nat -> Bytes
  bytes-of xs = bytes-from-list xs

  same :: Bytes -> Bytes -> Bool
  same a b = if bytes-len a == bytes-len b then same-at a b 0 else False

  same-at :: Bytes -> Bytes -> Nat -> Bool
  same-at a b i =
    if i >= bytes-len a then True
    else if bytes-get a i == bytes-get b i then same-at a b (i + 1)
    else False

  seek-or :: StringIo -> Nat -> StringIo
  seek-or io offset = case string-io-seek io offset of
    Err _ -> io
    Ok io2 -> io2
"#;

fn main_source(body: &str) -> String {
    format!("module Main where\n  import Data.StringIo\n{HELPERS}{body}")
}

/// Compile and run the two-module project in a fresh temp directory,
/// returning the program's exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_stringio_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("StringIo.cinch", STRING_IO), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_exit(name: &str, body: &str, expected: i32) {
    let code = run_main(name, &main_source(body));
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

#[test]
fn write_then_read_round_trips_with_interleaved_cursor_movement() {
    expect_exit(
        "string-io-roundtrip",
        r#"
  check1 :: Int
  check1 =
    let io1 = string-io-write string-io-new (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let io0 = string-io-rewind io1 in
    let (b1, io2) = string-io-read io0 2 in
    let (b2, io3) = string-io-read io2 1 in
    let (b3, io4) = string-io-read io3 5 in
    let ok1 = same b1 (bytes-of (Cons 97 (Cons 98 Nil))) in
    let ok2 = same b2 (bytes-of (Cons 99 Nil)) in
    let ok3 = bytes-len b3 == 0 in
    let ok4 = string-io-pos io1 == 3 in
    let ok5 = string-io-pos io0 == 0 in
    let ok6 = string-io-pos io4 == 3 in
    let ok7 = string-io-eof io3 in
    if ok1 && ok2 && ok3 && ok4 && ok5 && ok6 && ok7 then 0 else 1

  check2 :: Int
  check2 =
    let io1 = string-io-write string-io-new (bytes-of (Cons 97 (Cons 98 Nil))) in
    let io2 = string-io-write io1 (bytes-of (Cons 99 Nil)) in
    let (b1, io3) = string-io-read (string-io-rewind io2) 1 in
    let io4 = string-io-write io3 (bytes-of (Cons 100 Nil)) in
    let (b2, io5) = string-io-read io4 2 in
    let ok1 = string-io-len io2 == 3 in
    let ok2 = same (string-io-to-bytes io2) (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let ok3 = same b1 (bytes-of (Cons 97 Nil)) in
    let ok4 = same (string-io-to-bytes io4) (bytes-of (Cons 97 (Cons 100 (Cons 99 Nil)))) in
    let ok5 = same b2 (bytes-of (Cons 99 Nil)) in
    let ok6 = string-io-pos io5 == 3 in
    if ok1 && ok2 && ok3 && ok4 && ok5 && ok6 then 0 else 2

  main :: Int
  main = check1 + check2
"#,
        0,
    );
}

#[test]
fn read_line_through_the_next_lf_with_and_without_a_terminator() {
    expect_exit(
        "string-io-readline",
        r#"
  check1 :: Int
  check1 =
    let io0 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 10 (Cons 99 (Cons 100 (Cons 10 Nil))))))) in
    let (l1, io1) = string-io-read-line io0 in
    let (l2, io2) = string-io-read-line io1 in
    let (l3, io3) = string-io-read-line io2 in
    let ok1 = same l1 (bytes-of (Cons 97 (Cons 98 (Cons 10 Nil)))) in
    let ok2 = same l2 (bytes-of (Cons 99 (Cons 100 (Cons 10 Nil)))) in
    let ok3 = bytes-len l3 == 0 in
    let ok4 = string-io-pos io2 == 6 in
    let ok5 = string-io-pos io3 == 6 in
    let ok6 = string-io-eof io2 in
    if ok1 && ok2 && ok3 && ok4 && ok5 && ok6 then 0 else 1

  check2 :: Int
  check2 =
    let io0 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 10 (Cons 99 (Cons 100 Nil)))))) in
    let (l1, io1) = string-io-read-line io0 in
    let (l2, io2) = string-io-read-line io1 in
    let (l3, io3) = string-io-read-line io2 in
    let ok1 = same l1 (bytes-of (Cons 97 (Cons 98 (Cons 10 Nil)))) in
    let ok2 = same l2 (bytes-of (Cons 99 (Cons 100 Nil))) in
    let ok3 = bytes-len l3 == 0 in
    let ok4 = string-io-pos io2 == 5 in
    let ok5 = string-io-eof io2 in
    if ok1 && ok2 && ok3 && ok4 && ok5 then 0 else 2

  check3 :: Int
  check3 =
    let (l1, io1) = string-io-read-line string-io-new in
    let ok1 = bytes-len l1 == 0 in
    let ok2 = string-io-pos io1 == 0 in
    let ok3 = string-io-eof io1 in
    if ok1 && ok2 && ok3 then 0 else 4

  check4 :: Int
  check4 =
    let io0 = string-io-from-bytes (bytes-of (Cons 10 (Cons 10 (Cons 97 Nil)))) in
    let (l1, io1) = string-io-read-line io0 in
    let (l2, io2) = string-io-read-line io1 in
    let (l3, io3) = string-io-read-line io2 in
    let ok1 = same l1 (bytes-of (Cons 10 Nil)) in
    let ok2 = same l2 (bytes-of (Cons 10 Nil)) in
    let ok3 = same l3 (bytes-of (Cons 97 Nil)) in
    let ok4 = string-io-eof io3 in
    if ok1 && ok2 && ok3 && ok4 then 0 else 8

  main :: Int
  main = check1 + check2 + check3 + check4
"#,
        0,
    );
}

#[test]
fn read_at_end_of_buffer_is_empty_and_does_not_move_the_cursor() {
    expect_exit(
        "string-io-read-end",
        r#"
  check1 :: Int
  check1 =
    let io1 = string-io-write string-io-new (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let io0 = string-io-rewind io1 in
    let (b1, io2) = string-io-read io0 3 in
    let (b2, io3) = string-io-read io2 2 in
    let ok1 = same b1 (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let ok2 = bytes-len b2 == 0 in
    let ok3 = string-io-pos io3 == 3 in
    let ok4 = string-io-eof io2 in
    if ok1 && ok2 && ok3 && ok4 then 0 else 1

  check2 :: Int
  check2 =
    let io1 = string-io-write string-io-new (bytes-of (Cons 97 Nil)) in
    let io0 = string-io-rewind io1 in
    let (b1, io2) = string-io-read io0 0 in
    let (b2, io3) = string-io-read io2 1 in
    let (b3, io4) = string-io-read io3 1 in
    let ok1 = bytes-len b1 == 0 in
    let ok2 = string-io-pos io2 == 0 in
    let ok3 = same b2 (bytes-of (Cons 97 Nil)) in
    let ok4 = bytes-len b3 == 0 in
    let ok5 = string-io-pos io4 == 1 in
    if ok1 && ok2 && ok3 && ok4 && ok5 then 0 else 2

  check3 :: Int
  check3 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 Nil))) in
    let (b1, io2) = string-io-read io1 9 in
    let ok1 = same b1 (bytes-of (Cons 97 (Cons 98 Nil))) in
    let ok2 = string-io-pos io2 == 2 in
    if ok1 && ok2 then 0 else 4

  main :: Int
  main = check1 + check2 + check3
"#,
        0,
    );
}

#[test]
fn rewind_restores_the_initial_state_and_conversions_copy() {
    expect_exit(
        "string-io-rewind",
        r#"
  check1 :: Int
  check1 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let (b1, io2) = string-io-read io1 2 in
    let io3 = string-io-rewind io2 in
    let (b2, io4) = string-io-read io3 2 in
    let ok1 = same b1 b2 in
    let ok2 = string-io-pos io3 == 0 in
    let ok3 = string-io-pos io2 == 2 in
    let ok4 = string-io-pos io4 == 2 in
    if ok1 && ok2 && ok3 && ok4 then 0 else 1

  check2 :: Int
  check2 =
    let b = bytes-of (Cons 0 (Cons 255 (Cons 10 Nil))) in
    let io1 = string-io-from-bytes b in
    let ok1 = same (string-io-to-bytes io1) b in
    let ok2 = string-io-pos io1 == 0 in
    let ok3 = string-io-len io1 == 3 in
    if ok1 && ok2 && ok3 then 0 else 2

  check3 :: Int
  check3 =
    let b = bytes-of (Cons 97 (Cons 98 Nil)) in
    let io1 = string-io-from-bytes b in
    let u = bytes-set b 0 122 in
    let ok1 = same (string-io-to-bytes io1) (bytes-of (Cons 97 (Cons 98 Nil))) in
    let ok2 = bytes-get b 0 == 122 in
    if ok1 && ok2 then 0 else 4

  check4 :: Int
  check4 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 Nil))) in
    let out = string-io-to-bytes io1 in
    let u = bytes-set out 0 122 in
    let ok1 = same (string-io-to-bytes io1) (bytes-of (Cons 97 (Cons 98 Nil))) in
    if ok1 then 0 else 8

  main :: Int
  main = check1 + check2 + check3 + check4
"#,
        0,
    );
}

#[test]
fn seek_past_the_end_is_refused_by_name() {
    expect_exit(
        "string-io-seek",
        r#"
  check1 :: Int
  check1 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    case string-io-seek io1 4 of
      Err e -> case e of
        SeekOutOfRange t l -> if t == 4 then (if l == 3 then 0 else 1) else 1
      Ok _ -> 1

  check2 :: Int
  check2 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    case string-io-seek io1 3 of
      Err _ -> 2
      Ok io2 -> if string-io-eof io2 then (if string-io-pos io2 == 3 then 0 else 2) else 2

  check3 :: Int
  check3 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    case string-io-seek io1 1 of
      Err _ -> 4
      Ok io2 -> case string-io-read io2 1 of
        (b, io3) -> if same b (bytes-of (Cons 98 Nil)) then (if string-io-pos io3 == 2 then 0 else 4) else 4

  main :: Int
  main = check1 + check2 + check3
"#,
        0,
    );
}

#[test]
fn write_overwrites_at_the_cursor_and_truncate_clamps() {
    expect_exit(
        "string-io-write-truncate",
        r#"
  check1 :: Int
  check1 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 (Cons 100 Nil))))) in
    let io2 = string-io-write (seek-or io1 1) (bytes-of (Cons 88 (Cons 89 Nil))) in
    let ok1 = same (string-io-to-bytes io2) (bytes-of (Cons 97 (Cons 88 (Cons 89 (Cons 100 Nil))))) in
    let ok2 = string-io-len io2 == 4 in
    let ok3 = string-io-pos io2 == 3 in
    if ok1 && ok2 && ok3 then 0 else 1

  check2 :: Int
  check2 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 Nil))) in
    let io2 = string-io-write (seek-or io1 2) (bytes-of (Cons 90 (Cons 90 (Cons 90 Nil)))) in
    let ok1 = same (string-io-to-bytes io2) (bytes-of (Cons 97 (Cons 98 (Cons 90 (Cons 90 (Cons 90 Nil)))))) in
    let ok2 = string-io-len io2 == 5 in
    let ok3 = string-io-pos io2 == 5 in
    let ok4 = string-io-eof io2 in
    if ok1 && ok2 && ok3 && ok4 then 0 else 2

  check3 :: Int
  check3 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 Nil)) in
    let io2 = string-io-write (seek-or io1 1) (bytes-of Nil) in
    let ok1 = same (string-io-to-bytes io2) (bytes-of (Cons 97 Nil)) in
    let ok2 = string-io-pos io2 == 1 in
    let ok3 = string-io-len io2 == 1 in
    if ok1 && ok2 && ok3 then 0 else 4

  check4 :: Int
  check4 =
    let io1 = string-io-from-bytes (bytes-of (Cons 97 (Cons 98 (Cons 99 (Cons 100 (Cons 101 (Cons 102 Nil))))))) in
    let io2 = string-io-truncate io1 3 in
    let ok1 = same (string-io-to-bytes io2) (bytes-of (Cons 97 (Cons 98 (Cons 99 Nil)))) in
    let ok2 = string-io-len io2 == 3 in
    let ok3 = string-io-pos (string-io-truncate (seek-or io1 6) 3) == 3 in
    let ok4 = same (string-io-to-bytes (string-io-truncate io1 9)) (string-io-to-bytes io1) in
    let ok5 = string-io-len (string-io-truncate io1 9) == 6 in
    if ok1 && ok2 && ok3 && ok4 && ok5 then 0 else 8

  main :: Int
  main = check1 + check2 + check3 + check4
"#,
        0,
    );
}

#[test]
fn a_false_claim_names_exactly_its_bit() {
    // The control: the claim "a one-byte read at the start returns no
    // bytes" is FALSE, so this check fires and the exit code is exactly
    // bit 1. The naming is observable, which is what makes the zeros
    // above meaningful (a check that could never fire would be a
    // tautology).
    expect_exit(
        "string-io-bit-control",
        r#"
  main :: Int
  main =
    let io1 = string-io-write string-io-new (bytes-of (Cons 97 Nil)) in
    let io0 = string-io-rewind io1 in
    let (b1, io2) = string-io-read io0 1 in
    if bytes-len b1 == 0 then 0 else 1
"#,
        1,
    );
}
