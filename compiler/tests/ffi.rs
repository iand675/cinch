//! Foreign binding tests (P11): real C symbols across the boundary with
//! layout-checked binding contracts, effect honesty, ownership roles and
//! the no-unwind rule (foreign function interface specification
//! sections 2-4; attribute annotations specification section 4).

use cinch::driver::{check_sources, run_source_objects};
use cinch::core::TargetProfile;

/// Compile a C fixture to an object file and return its path.
fn c_fixture(name: &str, src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("slffi-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write fixture");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile the C fixture");
    o_path.to_string_lossy().into_owned()
}

fn expect_ok(name: &str, src: &str, objects: &[&str], expected: i32) {
    match run_source_objects(src, objects) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

// ------------------------------------- real symbols cross correctly (section 2)

#[test]
fn foreign_import_calls_a_real_c_symbol() {
    let o = c_fixture(
        "add3",
        r#"
long long fix_add3(long long a, long long b, long long c) { return a + b + c; }
int fix_neg(int x) { return x * 2; }
"#,
    );
    expect_ok(
        "import-call",
        r#"
module Main where
  foreign import ccall "fix_add3" add3 :: I64 -> I64 -> I64 ->{Foreign Fix, IO, Diverge} I64
  foreign import ccall "fix_neg" neg :: CInt ->{Foreign Fix, Fail} CInt

  main :: Unit ->{Foreign Fix, IO, Diverge, Fail} Int
  main _ =
    let s = add3 1000000000 2000000000 3000000000 in
    let n = neg 7 in
    if s == 6000000000 then if n == 14 then 0 else 1 else 2
"#,
        &[&o],
        0,
    );
}

#[test]
fn foreign_export_round_trips_through_c() {
    let o = c_fixture(
        "mul",
        r#"
extern long long fix_mul(long long, long long);
int main(void) { return fix_mul(6, 7) == 42 ? 0 : 3; }
"#,
    );
    expect_ok(
        "export-round-trip",
        r#"
module Main where
  fixMul :: I64 -> I64 -> I64
  fixMul x y = wrapping-mul-i64 x y

  foreign export ccall "fix_mul" fixMul :: I64 -> I64 -> I64
"#,
        &[&o],
        0,
    );
}

// ------------------------------------------------- effect honesty (section 4)

#[test]
fn effect_undeclared_foreign_is_rejected() {
    expect_err(
        "pure-foreign",
        r#"
module Main where
  foreign import ccall "fix_pure" pure :: I64 -> I64
"#,
        "a foreign import must declare its effects",
    );
    // A label outside the accounted vocabulary is not an accounting.
    expect_err(
        "unaccounted-foreign",
        r#"
module Main where
  foreign import ccall "fix_pure" pure :: I64 ->{MyLib} I64
"#,
        "an external function is not pure",
    );
    // The envelope is closed and explicit, never an open row.
    expect_err(
        "open-row-foreign",
        r#"
module Main where
  foreign import ccall "fix_pure" pure :: forall e. I64 ->{e} I64
"#,
        "closed and explicit",
    );
}

// -------------------------------------- boundary layouts (attribute chapter 4)

#[test]
fn foreign_signature_contradicting_repr_facts_is_rejected() {
    // A boxed (uniform-word) aggregate has no C representation.
    expect_err(
        "uniform-word-aggregate",
        r#"
module Main where
  record R where
    x :: I32
  foreign import ccall "fix_take" take :: R ->{Foreign Fix, IO} Unit
"#,
        "uniform-word",
    );
    // A fixed-discriminant representation crosses as a tag only through a
    // generated shim: rejected instead of guessed (section 2).
    expect_err(
        "discriminant-data",
        r#"
module Main where
  #[repr(u8)]
  data Color where
    Red :: Color
    Blue :: Color
  foreign import ccall "fix_color" color :: Color ->{Foreign Fix, IO} Unit
"#,
        "no C representation at the boundary",
    );
    // A bare aggregate result follows the platform's by-value struct ABI,
    // which this slice does not guess.
    expect_err(
        "aggregate-result",
        r#"
module Main where
  #[repr(C)]
  record P where
    a :: I32
  foreign import ccall "fix_make" make :: () ->{Foreign Fix, Diverge} P
"#,
        "a foreign result cannot be an aggregate",
    );
}

#[test]
fn packed_boundary_fields_marshal_without_padding_reads() {
    // The C fixture reads a packed struct at C offsets (a @0, b @2, c @6).
    let o = c_fixture(
        "packp",
        r#"
#pragma pack(push, 2)
struct P { unsigned char a; unsigned int b; unsigned short c; };
#pragma pack(pop)
long long fix_take_p(const struct P* p) {
  return (long long)p->a * 1000000 + (long long)p->b * 100 + (long long)p->c;
}
"#,
    );
    expect_ok(
        "packed-marshal",
        r#"
module Main where
  #[repr(C, packed(2))]
  record P where
    a :: U8
    b :: U32
    c :: U16

  foreign import ccall "fix_take_p" takeP :: P ->{Foreign Fix, IO} I64

  main :: Unit ->{Foreign Fix, IO} Int
  main _ = let p = P { a = 1, b = 300, c = 7 } in if takeP p == 1030007 then 0 else 1
"#,
        &[&o],
        0,
    );
    // The marshalling wrapper reads each field at its recorded offset with
    // its recorded size: no word-width load crosses the padding bytes.
    let program = check_sources(
        r#"
module Main where
  #[repr(C, packed(2))]
  record P where
    a :: U8
    b :: U32
    c :: U16

  foreign import ccall "fix_take_p" takeP :: P ->{Foreign Fix, IO} I64

  main :: Unit ->{Foreign Fix, IO} Int
  main _ = let p = P { a = 1, b = 300, c = 7 } in if takeP p == 1030007 then 0 else 1
"#,
        "<test>",
    )
    .expect("check");
    let ir = cinch::codegen::emit(&program).expect("emit");
    let start = ir.find("define i64 @\"Main.takeP\"(").expect("wrapper unit");
    let end = ir[start..].find("\n}\n").expect("wrapper end") + start;
    let wrapper = &ir[start..end];
    assert!(wrapper.contains("load i8,"), "wrapper loads the U8 field:\n{wrapper}");
    assert!(wrapper.contains("load i32,"), "wrapper loads the U32 field:\n{wrapper}");
    assert!(wrapper.contains("load i16,"), "wrapper loads the U16 field:\n{wrapper}");
    // The wrapper releases its parameter on return, which reads the
    // block's header (the word 8 bytes before the block); every other
    // load is a field read at the field's own width.
    let headers: Vec<&str> = wrapper
        .lines()
        .filter_map(|l| {
            let (lhs, rhs) = l.trim().split_once(" = ")?;
            (rhs.starts_with("getelementptr") && rhs.ends_with(", i64 -8")).then_some(lhs)
        })
        .collect();
    for l in wrapper.lines() {
        if let Some((_, rest)) = l.split_once("load i64, ptr ") {
            let ptr = rest.split([',', ' ']).next().unwrap_or(rest);
            assert!(
                headers.contains(&ptr),
                "the packed copy never reads a word across padding ({l}):\n{wrapper}"
            );
        }
    }
}

#[test]
fn repr_c_field_access_uses_the_recorded_offsets() {
    // Values constructed in Cinch land at their C offsets: the fixture
    // reads `a` at offsetof 0 and `b` at offsetof 4 of a repr(C) struct.
    let o = c_fixture(
        "cp",
        r#"
struct CP { unsigned char a; unsigned int b; };
long long fix_read_cp(const struct CP* p) {
  return (long long)p->a * 1000 + (long long)p->b;
}
"#,
    );
    expect_ok(
        "repr-c-offsets",
        r#"
module Main where
  #[repr(C)]
  record CP where
    a :: U8
    b :: U32

  foreign import ccall "fix_read_cp" readCp :: CP ->{Foreign Fix, IO} I64

  main :: Unit ->{Foreign Fix, IO} Int
  main _ =
    let p = CP { a = 5, b = 600 } in
    let x = readCp p in
    let y = p.b in
    if x == 5600 then if y == 600 then 0 else 1 else 2
"#,
        &[&o],
        0,
    );
}

// --------------------------------------------- opaque resources (section 2-3)

#[test]
fn opaque_values_are_unprojectable_and_foreign_only() {
    let o = c_fixture(
        "handle",
        r#"
static int cell = 7;
void* h_open(void) { return &cell; }
void h_close(void* p) { (void)p; }
long long h_read(void* p) { return *(int*)p; }
"#,
    );
    expect_ok(
        "opaque-round-trip",
        r#"
module Main where
  opaque type H :: Type
  foreign import ccall "h_open" hOpen :: () ->{Foreign Fix, Diverge} Owned FixHeap H
  foreign import ccall "h_read" hRead :: Borrowed H ->{Foreign Fix, IO} I64
  foreign import ccall "h_close" hClose :: Owned FixHeap H ->{Foreign Fix, Diverge} Unit

  main :: Unit ->{Foreign Fix, Diverge, IO} Int
  main _ =
    let h = hOpen () in
    let n = hRead h in
    let u = hClose h in
    if n == 7 then 0 else 1
"#,
        &[&o],
        0,
    );
    // No layout projection: an opaque type is not a record.
    expect_err(
        "opaque-unprojectable",
        r#"
module Main where
  opaque type H :: Type
  f :: H -> I32
  f x = x.field
"#,
        "is not a record",
    );
    // No construction outside a foreign result.
    expect_err(
        "opaque-unconstructible",
        r#"
module Main where
  opaque type H :: Type
  f :: Unit -> H
  f _ = H
"#,
        "unknown name",
    );
}

// --------------------------------------- pointer roles and casts (section 3)

#[test]
fn integer_to_pointer_cast_is_rejected() {
    expect_err(
        "int-to-ptr",
        r#"
module Main where
  foreign import ccall "fix_take" take :: RawPtr U8 ->{Foreign Fix, IO} Unit
  f :: U64 -> Unit
  f n = take n
"#,
        "never constructed from an integer",
    );
}

#[test]
fn owned_borrowed_roles_and_the_deallocator_rule() {
    let o = c_fixture(
        "alloc",
        r#"
#include <stdlib.h>
void* fix_alloc(unsigned long n) { return malloc(n); }
void fix_free(void* p) { free(p); }
void fix_free2(void* p) { free(p); }
static int held = 1;
void* fix_borrow(void) { return &held; }
"#,
    );
    // The matching deallocator releases the allocation.
    expect_ok(
        "owned-deallocator",
        r#"
module Main where
  foreign import ccall "fix_alloc" alloc :: Usize ->{Foreign Fix, Diverge} Owned FixHeap (RawPtr U8)
  foreign import ccall "fix_free" free :: Owned FixHeap (RawPtr U8) ->{Foreign Fix, Diverge} Unit

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = let p = alloc 16 in let u = free p in 0
"#,
        &[&o],
        0,
    );
    // Wrong deallocation: the allocator identities never convert.
    expect_err(
        "wrong-deallocation",
        r#"
module Main where
  foreign import ccall "fix_alloc" alloc :: Usize ->{Foreign Fix, Diverge} Owned FixHeap (RawPtr U8)
  foreign import ccall "fix_free2" free2 :: Owned OtherHeap (RawPtr U8) ->{Foreign Fix, Diverge} Unit

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = let p = alloc 16 in let u = free2 p in 0
"#,
        "pointer role mismatch",
    );
    // A borrowed return cannot be freed: no ownership to release.
    expect_err(
        "freeing-a-borrow",
        r#"
module Main where
  foreign import ccall "fix_borrow" borrow :: () ->{Foreign Fix, Diverge} Borrowed (RawPtr U8)
  foreign import ccall "fix_free" free :: Owned FixHeap (RawPtr U8) ->{Foreign Fix, Diverge} Unit

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = let p = borrow () in let u = free p in 0
"#,
        "pointer role mismatch",
    );
    // An owned result needs its matching deallocator somewhere in the
    // program (section 4's allocator-identity rule).
    expect_err(
        "missing-deallocator",
        r#"
module Main where
  foreign import ccall "fix_alloc" alloc :: Usize ->{Foreign Fix, Diverge} Owned Orphan (RawPtr U8)

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = let p = alloc 16 in 0
"#,
        "needs its matching deallocator",
    );
}

#[test]
fn out_cells_are_one_word() {
    expect_err(
        "out-over-aggregate",
        r#"
module Main where
  #[repr(C)]
  record P where
    a :: I32
  foreign import ccall "fix_fill" fill :: Out P ->{Foreign Fix, IO} Unit
"#,
        "an `Out` cell is one word",
    );
}

// ------------------------------------------------- no unwind across (section 4)

#[test]
fn faulting_foreign_path_exits_without_unwinding() {
    // This exported function used to fault inside the runtime with an
    // out-of-bounds byte read. Bytes reads are now total by construction,
    // so the same program is refused at compile time: the read's index
    // obligation is not provable, and no faulting export is ever built.
    // The fault-path exit across the boundary itself (71, no unwind into
    // the C caller) stays covered by the non-null contract test below.
    expect_err(
        "export-fault",
        r#"
module Main where
  boom :: Unit -> I64
  boom _ = nat-to-i64 (bytes-get (bytes-new 1) 5)

  foreign export ccall "fix_boom" boom :: Unit -> I64
"#,
        "the obligation `5 < (Data.Bytes.bytes-len (Data.Bytes.bytes-new 1))`",
    );
}

#[test]
fn non_null_contract_violation_faults_without_unwinding() {
    // A non-null boundary pointer crossing as null is a contract
    // violation: the wrapper's fault path exits (71).
    let o = c_fixture(
        "null",
        r#"
void* fix_null(void) { return 0; }
"#,
    );
    expect_ok(
        "null-result-fault",
        r#"
module Main where
  foreign import ccall "fix_null" nullPtr :: () ->{Foreign Fix, Diverge} RawPtr U8

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = let p = nullPtr () in 0
"#,
        &[&o],
        71,
    );
}
