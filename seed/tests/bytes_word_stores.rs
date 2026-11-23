//! The little-endian word stores (`cinch.runtime.bytesSetLe32`/`bytesSetLe64`)
//! behind `Data.Bytes.Word`'s `bytes-write-le32`/`bytes-write-le64`: every
//! alignment writes exactly its range in wire order and preserves the bytes
//! around it, the compile-time evaluator agrees with the emitted code, a range
//! that leaves the buffer takes the runtime's out-of-bounds fault, and the
//! emitted unit is one unaligned store (byte swapped on a big-endian target).
//! The same programs pin down the open-coded scalar primitives: a byte loop's
//! comparisons and conversions are instructions, not calls.

mod common;

use cinch::core::TargetProfile;
use cinch::driver::Mode;

const WORD: &str = include_str!("../prelude/Data/Bytes/Word.cinch");

/// A project with `Data.Bytes.Word` beside `main`.
fn project(tag: &str, main: &str) -> std::path::PathBuf {
    common::project(tag, &[("Data.Bytes.Word.cinch", WORD), ("Main.cinch", main)])
}

/// Every offset of a 16-byte buffer pre-filled with 0xAA: the word lands at
/// its offset, low byte first, and every other byte is still 0xAA. The exit
/// status is the first failing case (1 + 2 * offset, plus one for the 64-bit
/// form), 0 when every case passes.
const ROUND_TRIP: &str = r#"module Main where
  import Data.Bytes.Word
  import Data.Machine
  import Data.Nat

  -- | A 16-byte buffer of 0xAA.
  filled :: Unit -> Bytes
  filled u = let b = bytes-new 16 in let w = fill b 0 in b

  -- | Fill from i with 0xAA.
  fill :: Bytes -> Nat -> Unit
  fill b i = if i < bytes-len b then let u = bytes-set b i 170 in fill b (i + 1) else ()

  -- | Byte i of a little-endian word (0 to 7).
  lane :: U64 -> Nat -> Nat
  lane x i = u8-to-nat (wrapping-u64-to-u8 (wrapping-shr-u64 x (shift i)))

  -- | The bit offset of byte lane i.
  shift :: Nat -> U32
  shift i = if i == 0 then 0 else if i == 1 then 8 else if i == 2 then 16 else if i == 3 then 24 else if i == 4 then 32 else if i == 5 then 40 else if i == 6 then 48 else 56

  -- | Whether byte j holds the word's lane or the fill.
  expect :: Bytes -> Nat -> Nat -> U64 -> Nat -> Bool
  expect b at width x j =
    if j < bytes-len b then
      let want = if at <= j && j < at + width then lane x (j - at) else 170 in
      bytes-get b j == want && expect b at width x (j + 1)
    else True

  -- | One 32-bit and one 64-bit store at offset `at`, each into a fresh buffer.
  case-at :: Nat -> Int
  case-at at =
    let x = word in
    let b = filled () in
    let u = if at + 4 <= bytes-len b then bytes-write-le32 b at low-word else () in
    let ok32 = expect b at 4 x 0 && (if at + 4 <= bytes-len b then bytes-read-le32 b at == low-word else False) in
    let c = filled () in
    let v = if at + 8 <= bytes-len c then bytes-write-le64 c at x else () in
    let ok64 = expect c at 8 x 0 && (if at + 8 <= bytes-len c then bytes-read-le64 c at == x else False) in
    if not ok32 then 1 + 2 * nat-to-int at else if not ok64 then 2 + 2 * nat-to-int at else 0

  -- | 0x123456789ABCDEF0: eight distinct, nonzero lanes.
  word :: U64
  word = 1311768467463790320

  -- | Its low 32 bits, 0x9ABCDEF0.
  low-word :: U32
  low-word = 2596069104

  -- | Offsets 0 to 8: aligned, every misalignment, and the last 64-bit slot.
  every :: Nat -> Int
  every at = if at > 8 then 0 else (case case-at at of 0 -> every (at + 1); failed -> failed)

  main :: Int
  main = every 0
"#;

#[test]
fn word_stores_write_exactly_their_range_at_every_alignment() {
    let dir = project("wordstores", ROUND_TRIP);
    assert_eq!(common::run(&dir, "Main.cinch"), 0, "exit is 1 + 2 * offset (32-bit) or 2 + 2 * offset (64-bit) for the first failing case");
    common::cleanup(&dir);
}

#[test]
fn the_compile_time_evaluator_stores_the_same_bytes() {
    let main = r#"module Main where
  -- | The intrinsics themselves: a generator runs only code it can see.
  foreign import ccall "cinch.runtime.bytesSetLe32" store32 :: Bytes -> Nat -> U32 -> Unit
  -- | The 64-bit store.
  foreign import ccall "cinch.runtime.bytesSetLe64" store64 :: Bytes -> Nat -> U64 -> Unit
  -- | The bytes a word store leaves, packed as a number, at compile time.
  stage packed :: Nat
  stage packed =
    let b = bytes-new 12 in
    let u = store32 b 1 305419896 in
    let v = store64 b 4 72623859790382856 in
    bytes-get b 0 + 256 * (bytes-get b 1 + 256 * (bytes-get b 2 + 256 * (bytes-get b 3 + 256 * (bytes-get b 4 + 256 * (bytes-get b 11)))))
  generate do
    emit (quoteDecls where
      folded :: Nat
      folded = $(quoteNat packed))
  -- | 0x12345678 at 1 (0x78 0x56 0x34, then 0x12 overwritten by 0x08),
  -- 0x0102030405060708 at 4 (0x08 .. 0x01 at 11).
  main :: Int
  main = if folded == 0 + 256 * (120 + 256 * (86 + 256 * (52 + 256 * (8 + 256 * 1)))) then 0 else 1
"#;
    // The self-hosted compiler's stage evaluator has no compile-time
    // buffers at all (it refuses `bytes-new` before any store), so this
    // is the Native seed's evaluator alone.
    if common::external_frontend::configured().expect("frontend selection").is_some() {
        return;
    }
    let dir = project("wordstage", main);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

#[test]
fn a_store_past_the_end_takes_the_out_of_bounds_fault() {
    // The raw intrinsic carries no proof: the runtime's check is what stands
    // between it and the buffer, exactly as for `bytes-set`.
    let main = r#"module Main where
  foreign import ccall "cinch.runtime.bytesSetLe32" store :: Bytes -> Nat -> U32 -> Unit
  main :: Int
  main = let b = bytes-new 6 in let u = store b 3 1 in 0
"#;
    let dir = project("wordfault", main);
    let code = common::run(&dir, "Main.cinch");
    assert_ne!(code, 0, "a store of bytes 3..7 into a 6-byte buffer must fault");
    common::cleanup(&dir);
}

/// The emitted LLVM for a program, at `profile`'s target.
fn emit_ir(tag: &str, main: &str, profile: &TargetProfile) -> String {
    let dir = project(tag, main);
    let out = dir.join("program.ll");
    common::drive_with_profile(Mode::EmitIr, cinch::check::StageMode::Refresh, &dir, "Main.cinch", Some(out.clone()), profile)
        .unwrap_or_else(|e| panic!("{tag}: emit failed: {e}"));
    let ir = std::fs::read_to_string(&out).unwrap();
    common::cleanup(&dir);
    ir
}

/// One unit's definition text.
fn body<'a>(ir: &'a str, unit: &str) -> &'a str {
    let head = format!("@\"{unit}\"(");
    let at = ir
        .lines()
        .position(|l| l.starts_with("define") && l.contains(&head))
        .unwrap_or_else(|| panic!("no definition of {unit}"));
    let start: usize = ir.lines().take(at).map(|l| l.len() + 1).sum();
    let end = ir[start..].find("\n}\n").map_or(ir.len(), |e| start + e + 3);
    &ir[start..end]
}

#[test]
fn a_word_store_is_one_unaligned_store_and_byte_loops_call_no_scalar_units() {
    let main = r#"module Main where
  import Data.Bytes.Word
  import Data.Machine
  import Data.Nat
  -- | Pack a byte buffer's words, comparing and narrowing on the way.
  pack :: Bytes -> Nat -> Nat
  pack b i = if i + 8 <= bytes-len b then
      let x = bytes-read-le64 b i in
      let w = bytes-write-le32 b i (wrapping-u64-to-u32 (wrapping-shr-u64 x 3)) in
      if nat-to-i64 i < 100 then pack b (i + 4) else i
    else i
  main :: Int
  main = let b = bytes-new 64 in let u = bytes-write-le64 b 0 7 in if pack b 0 == 60 then 0 else 1
"#;
    let ir = emit_ir("wordir", main, &TargetProfile::native());
    // The self-hosted backend lowers the pair through the externs the
    // pinned runtime exports: the byte store at the range's last index
    // validates it, and one unaligned store writes the word.
    if common::external_frontend::configured().expect("frontend selection").is_some() {
        for (unit, store) in [("cinch.typed.cinch.runtime.bytesSetLe32", "store i32"), ("cinch.typed.cinch.runtime.bytesSetLe64", "store i64")] {
            let unit_ir = body(&ir, unit);
            assert_eq!(unit_ir.matches(store).count(), 1, "{unit} is not one word store:\n{unit_ir}");
            assert!(unit_ir.contains(", align 1"), "{unit}'s store is not unaligned:\n{unit_ir}");
            assert_eq!(unit_ir.matches("@cinch_bytes_set(").count(), 1, "{unit} does not validate its range once:\n{unit_ir}");
        }
        return;
    }
    for (unit, store) in [("cinch.runtime.bytesSetLe32", "store i32"), ("cinch.runtime.bytesSetLe64", "store i64")] {
        let unit_ir = body(&ir, unit);
        assert_eq!(unit_ir.matches(store).count(), 1, "{unit} is not one word store:\n{unit_ir}");
        assert!(unit_ir.contains(", align 1"), "{unit}'s store is not unaligned:\n{unit_ir}");
        assert!(!unit_ir.contains("@llvm.bswap"), "{unit} swaps on a little-endian target:\n{unit_ir}");
        assert!(!unit_ir.contains("cinch_bytes_set("), "{unit} falls back to byte stores:\n{unit_ir}");
    }
    let pack = body(&ir, "Main.pack");
    assert!(!pack.contains("@\"cinch.prim."), "the loop still calls a scalar primitive unit:\n{pack}");
    assert!(!pack.contains("@\"cinch.runtime.bytesSet\""), "the word write is still byte stores:\n{pack}");

    let mut big = TargetProfile::native();
    big.triple = "powerpc64-unknown-linux-gnu".into();
    big.arch = "powerpc64".into();
    big.endian = "big".into();
    let ir = emit_ir("wordirbig", main, &big);
    for (unit, swap) in [("cinch.runtime.bytesSetLe32", "@llvm.bswap.i32"), ("cinch.runtime.bytesSetLe64", "@llvm.bswap.i64")] {
        let unit_ir = body(&ir, unit);
        assert!(unit_ir.contains(swap), "{unit} does not swap on a big-endian target:\n{unit_ir}");
    }
}
