//! Tests for native target-width `Ptr`, `#[repr(value)]` POD structs,
//! `FixedArray n a`, multi-word value ABI, `withLocal` stack allocation,
//! in-place place reads/writes/sub-borrows, `refAsPtr`/`mutAsPtr`, and
//! deferred polymorphic uniform-word boundary rejections.

mod common;

use cinch::core::TargetProfile;
use common::source_frontend::{
    check_source_interfaces, check_source_interfaces_profile, check_sources, emit_ir,
    emit_ir_profile, run_source, run_source_objects,
};

fn c_fixture(name: &str, src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("cinch-unboxed-{}-{}", name, std::process::id()));
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
    assert!(status.success(), "{name}: clang failed to compile C fixture");
    o_path.to_string_lossy().into_owned()
}

fn llvm_hex_symbol(name: &str) -> String {
    let mut out = String::from("@\"");
    for b in name.as_bytes() {
        out.push_str(&format!("\\{:02X}", b));
    }
    out.push_str("\"(");
    out
}

fn matches_unit_define(line: &str, unit: &str) -> bool {
    if !line.starts_with("define ") {
        return false;
    }
    let candidates = [
        format!("@\"{unit}\"("),
        format!("@\"cinch.typed.m.{unit}\"("),
        format!("@\"cinch.typed.m.{unit}#b\"("),
        format!("@\"m.{unit}.llvm\"("),
        llvm_hex_symbol(&format!("{unit}.llvm")),
    ];
    if unit == "Main.main" && line.contains("@\"cinch.typed.main\"(") {
        return true;
    }
    candidates.iter().any(|c| line.contains(c))
}

fn try_body_of(ir: &str, unit: &str) -> Option<String> {
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if matches_unit_define(l, unit) {
            let mut body = String::new();
            for l in lines.by_ref() {
                if l == "}" {
                    return Some(body);
                }
                body.push_str(l);
                body.push('\n');
            }
        }
    }
    None
}

fn body_of(ir: &str, unit: &str) -> String {
    try_body_of(ir, unit).unwrap_or_else(|| panic!("{unit}: no define found in IR"))
}

fn define_of(ir: &str, unit: &str) -> String {
    ir.lines()
        .find(|l| matches_unit_define(l, unit))
        .unwrap_or_else(|| panic!("{unit}: no define found in IR"))
        .to_string()
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(()) => panic!("{name}: expected compile error matching one of {needles:?}, but checked"),
        Err(e) => {
            assert!(
                needles.iter().any(|n| e.contains(n)),
                "{name}: error {e:?} did not contain any of {needles:?}"
            );
        }
    }
}

// =========================================================================
// Part 0: Native target-width provenance-preserving Ptr
// =========================================================================

#[test]
fn prim_native_ptr_and_void_return_no_inttoptr() {
    let src = r#"
module Main where
  prim ptrOffset :: forall a. RawPtr a -> Isize -> RawPtr a where
    unsafe llvm {
      %p = getelementptr i8, ptr %0, i64 %1
      ret ptr %p
    }

  prim storeU64 :: RawPtr U64 -> U64 ->{IO} Unit where
    unsafe llvm {
      store i64 %1, ptr %0, align 8
      ret void
    }

  prim loadU64 :: Borrowed U64 ->{IO} U64 where
    unsafe llvm {
      %v = load i64, ptr %0, align 8
      ret i64 %v
    }

  #[no-alloc]
  roundTrip :: Unit ->{IO} U64
  roundTrip _ =
    withLocal (fixedArrayRepeat 4 (0 :: U64)) (\buf ->
      let p0 = mutAsPtr buf in
      let p2 = ptrOffset p0 16 in
      let _ = storeU64 p2 99 in
      let _ = storeU64 p0 11 in
      let r0 = loadU64 (refAsPtr buf) in
      let r2 = fixedArrayGet buf 2 in
      wrapping-add-u64 r0 r2
    )

  main :: Unit ->{IO} Int
  main _ =
    if roundTrip () == 110 then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit IR");
    let offset_def = define_of(&ir, "Main.ptrOffset");
    assert!(offset_def.contains("ptr") && !offset_def.contains("define i64"), "expected native ptr signature: {offset_def}");
    let offset_body = body_of(&ir, "Main.ptrOffset");
    assert!(!offset_body.contains("inttoptr") && !offset_body.contains("ptrtoint"), "unexpected ptr cast in ptrOffset: {offset_body}");

    let store_def = define_of(&ir, "Main.storeU64");
    assert!(store_def.contains("ptr %p0") || store_def.contains("ptr "), "expected ptr param on storeU64: {store_def}");

    let rt_body = body_of(&ir, "Main.roundTrip");
    assert!(!rt_body.contains("inttoptr") && !rt_body.contains("ptrtoint"), "unexpected ptr cast in roundTrip: {rt_body}");
    assert!(!rt_body.contains("@cinch_alloc"), "unexpected heap alloc in roundTrip: {rt_body}");

    let code = run_source(src).expect("run prim_native_ptr_and_void_return_no_inttoptr");
    assert_eq!(code, 0);
}

#[test]
fn wasm32_target_pointer_and_usize_width() {
    let src = r#"
module Main where
  export PtrPair

  #[repr(C, value)]
  record PtrPair where
    p :: RawPtr U8
    n :: Usize
    d :: Isize
    tag :: U32

  main :: Unit -> Int
  main _ = 0
"#;

    let wasm = TargetProfile::wasm32(cinch::core::ArtifactProfile::CoreWasm);

    let ifaces_wasm = check_source_interfaces_profile(src, "Main.cinch", &wasm)
        .expect("check wasm32 interfaces");
    let main_wasm = ifaces_wasm.iter().find(|i| i.module == "Main").expect("Main iface on wasm32");
    let layout_wasm = main_wasm
        .layouts
        .iter()
        .find(|l| l.name == "PtrPair")
        .expect("PtrPair layout on wasm32");
    assert_eq!(layout_wasm.size, 16, "4*4 bytes on wasm32");
    assert_eq!(layout_wasm.align, 4, "4-byte alignment on wasm32");
    assert_eq!(layout_wasm.fields[1].offset, 4);
    assert_eq!(layout_wasm.fields[2].offset, 8);
    assert_eq!(layout_wasm.fields[3].offset, 12);

    let ifaces_64 = check_source_interfaces(src, "Main.cinch").expect("check 64-bit interfaces");
    let main_64 = ifaces_64.iter().find(|i| i.module == "Main").expect("Main iface on 64-bit");
    let layout_64 = main_64
        .layouts
        .iter()
        .find(|l| l.name == "PtrPair")
        .expect("PtrPair layout on 64-bit");
    assert_eq!(layout_64.size, 32, "3*8 + 4 + 4 pad on 64-bit");
    assert_eq!(layout_64.align, 8, "8-byte alignment on 64-bit");
    assert_eq!(layout_64.fields[1].offset, 8);
    assert_eq!(layout_64.fields[2].offset, 16);
    assert_eq!(layout_64.fields[3].offset, 24);

    let _ir_wasm = emit_ir_profile(src, &wasm).expect("emit wasm32 IR");
}

// =========================================================================
// Part 1 & 2: #[repr(value)] POD structs, layout, ABI, and FFI
// =========================================================================

#[test]
fn repr_value_small_in_regs_and_no_alloc() {
    let src = r#"
module Main where
  #[repr(value)]
  record Small8 where
    a :: U32
    b :: U32

  #[repr(value)]
  record Vec2 where
    x :: U64
    y :: U64

  #[no-alloc]
  addSmall :: Small8 -> Small8 -> Small8
  addSmall p q = Small8 { a = wrapping-add-u32 p.a q.a, b = wrapping-add-u32 p.b q.b }

  #[no-alloc]
  addVec2 :: Vec2 -> Vec2 -> Vec2
  addVec2 p q =
    case p of
      Vec2 px py -> Vec2 (wrapping-add-u64 px q.x) (wrapping-add-u64 py q.y)

  main :: Unit -> Int
  main _ =
    let s = addSmall (Small8 10 20) (Small8 30 40) in
    let v = addVec2 (Vec2 100 200) (Vec2 5 7) in
    if s.a == 40 && s.b == 60 && v.x == 105 && v.y == 207 then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit IR");
    let def8 = define_of(&ir, "Main.addSmall");
    assert!(def8.contains("i64 @\"") && def8.contains("(i64 "), "8-byte value should pass/return in i64: {def8}");
    let def16 = define_of(&ir, "Main.addVec2");
    assert!(def16.contains("{ i64, i64 } @\"") && def16.contains("({ i64, i64 } "), "16-byte value should pass/return in {{ i64, i64 }}: {def16}");
    let body16 = body_of(&ir, "Main.addVec2");
    assert!(!body16.contains("@cinch_alloc"), "no heap alloc in addVec2: {body16}");

    let code = run_source(src).expect("run repr_value_small_in_regs_and_no_alloc");
    assert_eq!(code, 0);
}

#[test]
fn repr_value_large_and_overaligned_sret_and_nrvo() {
    let src = r#"
module Main where
  #[repr(value)]
  record Big4 where
    a :: U64
    b :: U64
    c :: U64
    d :: U64

  #[repr(value, align(64))]
  record CacheLine where
    val :: U64

  #[no-alloc]
  addBig :: Big4 -> Big4 -> Big4
  addBig x y =
    Big4
      (wrapping-add-u64 x.a y.a)
      (wrapping-add-u64 x.b y.b)
      (wrapping-add-u64 x.c y.c)
      (wrapping-add-u64 x.d y.d)

  #[no-alloc]
  bumpCache :: CacheLine -> CacheLine
  bumpCache c = CacheLine (wrapping-add-u64 c.val 1)

  main :: Unit -> Int
  main _ =
    let r = addBig (Big4 1 2 3 4) (Big4 10 20 30 40) in
    let c = bumpCache (CacheLine 41) in
    if r.a == 11 && r.b == 22 && r.c == 33 && r.d == 44 && c.val == 42 then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit IR");
    let def_big = define_of(&ir, "Main.addBig");
    assert!(def_big.contains("sret([32 x i8])"), "32-byte value should return via sret: {def_big}");
    assert!(def_big.contains("ptr readonly align 8 dereferenceable(32)"), "32-byte param should pass by readonly ptr: {def_big}");

    let def_cl = define_of(&ir, "Main.bumpCache");
    assert!(def_cl.contains("sret([64 x i8])") && def_cl.contains("align 64"), "over-aligned value should pass/return indirectly with align 64: {def_cl}");

    let code = run_source(src).expect("run repr_value_large_and_overaligned_sret_and_nrvo");
    assert_eq!(code, 0);
}

#[test]
fn repr_value_packed_and_nested_layout() {
    let src = r#"
module Main where
  export PackedHdr, Outer

  #[repr(C, value, packed(2))]
  record PackedHdr where
    tag :: U8
    len :: U32
    flags :: U16

  #[repr(C, value)]
  record Outer where
    hdr :: PackedHdr
    seq :: U64

  #[no-alloc]
  makeOuter :: U8 -> U32 -> U16 -> U64 -> Outer
  makeOuter t l f s = Outer (PackedHdr t l f) s

  main :: Unit -> Int
  main _ =
    let o = makeOuter 7 1000 55 99999 in
    if o.hdr.tag == 7 && o.hdr.len == 1000 && o.hdr.flags == 55 && o.seq == 99999 then 0 else 1
"#;

    let ifaces = check_source_interfaces(src, "Main.cinch").expect("check interfaces");
    let main_if = ifaces.iter().find(|i| i.module == "Main").expect("Main iface");
    let ph = main_if.layouts.iter().find(|l| l.name == "PackedHdr").expect("PackedHdr");
    assert_eq!(ph.size, 8);
    assert_eq!(ph.align, 2);
    assert_eq!(ph.fields[0].offset, 0);
    assert_eq!(ph.fields[1].offset, 2);
    assert_eq!(ph.fields[2].offset, 6);

    let outer = main_if.layouts.iter().find(|l| l.name == "Outer").expect("Outer");
    assert_eq!(outer.size, 16);
    assert_eq!(outer.align, 8);
    assert_eq!(outer.fields[0].offset, 0);
    assert_eq!(outer.fields[1].offset, 8);

    let code = run_source(src).expect("run repr_value_packed_and_nested_layout");
    assert_eq!(code, 0);
}

#[test]
fn repr_value_rejects_counted_fields() {
    expect_err(
        "repr-value-string",
        r#"
module Main where
  #[repr(value)]
  record Bad where
    s :: String
  main :: Unit -> Int
  main _ = 0
"#,
        &["POD", "non-refcounted", "repr(value)"],
    );

    expect_err(
        "repr-value-int",
        r#"
module Main where
  #[repr(value)]
  record Bad where
    n :: Int
  main :: Unit -> Int
  main _ = 0
"#,
        &["POD", "non-refcounted", "repr(value)"],
    );
}

#[test]
fn repr_value_ffi_zero_box_roundtrip() {
    let o = c_fixture(
        "val_ffi",
        r#"
#include <stdint.h>

typedef struct {
    uint64_t x;
    uint64_t y;
} Vec2;

typedef struct {
    uint64_t a;
    uint64_t b;
    uint64_t c;
    uint64_t d;
} Big4;

Vec2 c_add_vec2(Vec2 p, Vec2 q) {
    Vec2 r = { p.x + q.x, p.y + q.y };
    return r;
}

Big4 c_sum_big4(Big4 x, Big4 y) {
    Big4 r = { x.a + y.a, x.b + y.b, x.c + y.c, x.d + y.d };
    return r;
}

void c_bump_vec2_ptr(Vec2 *p, uint64_t dx, uint64_t dy) {
    p->x += dx;
    p->y += dy;
}
"#,
    );

    let src = r#"
module Main where
  #[repr(C, value)]
  record Vec2 where
    x :: U64
    y :: U64

  #[repr(C, value)]
  record Big4 where
    a :: U64
    b :: U64
    c :: U64
    d :: U64

  #[byval]
  #[sret]
  foreign import ccall "c_add_vec2" cAddVec2 :: Vec2 -> Vec2 ->{Foreign C} Vec2

  #[byval]
  #[sret]
  foreign import ccall "c_sum_big4" cSumBig4 :: Big4 -> Big4 ->{Foreign C} Big4

  foreign import ccall "c_bump_vec2_ptr" cBumpPtr :: RawPtr Vec2 -> U64 -> U64 ->{Foreign C} Unit

  main :: Unit ->{Foreign C} Int
  main _ =
    let v = cAddVec2 (Vec2 10 20) (Vec2 3 4) in
    let b = cSumBig4 (Big4 1 2 3 4) (Big4 10 20 30 40) in
    let bumped = withLocal v (\m ->
      let _ = cBumpPtr (mutAsPtr m) 5 6 in
      wrapping-add-u64 m.x m.y
    ) in
    if v.x == 13 && v.y == 24
      && b.a == 11 && b.b == 22 && b.c == 33 && b.d == 44
      && bumped == 48
    then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit FFI IR");
    let w_v = try_body_of(&ir, "Main.cAddVec2").unwrap_or_else(|| body_of(&ir, "Main.main"));
    assert!(!w_v.contains("@cinch_alloc"), "value FFI call must not heap-box: {w_v}");
    let w_b = try_body_of(&ir, "Main.cSumBig4").unwrap_or_else(|| body_of(&ir, "Main.main"));
    assert!(!w_b.contains("@cinch_alloc"), "large value FFI call must not heap-box: {w_b}");

    let code = run_source_objects(src, &[&o]).expect("run repr_value_ffi_zero_box_roundtrip");
    assert_eq!(code, 0);
}

// =========================================================================
// Part 1 & 4: FixedArray n a
// =========================================================================

#[test]
fn fixed_array_scalar_and_nested_no_alloc() {
    let src = r#"
module Main where
  #[repr(value)]
  record Pair where
    lo :: U32
    hi :: U32

  #[no-alloc]
  testArrays :: Unit -> U64
  testArrays _ =
    let a = fixedArrayRepeat 4 (7 :: U64) in
    let ps = fixedArrayRepeat 3 (Pair 2 5) in
    let p1 = fixedArrayGet ps 1 in
    let n = fixedArrayLen a in
    let s0 = wrapping-add-u64 (fixedArrayGet a 0) (fixedArrayGet a 3) in
    let s1 = wrapping-add-u64 (u32-to-u64 p1.lo) (u32-to-u64 p1.hi) in
    wrapping-add-u64 (wrapping-add-u64 s0 s1) (wrapping-usize-to-u64 n)

  main :: Unit -> Int
  main _ =
    if testArrays () == 25 then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit IR");
    let body = body_of(&ir, "Main.testArrays");
    assert!(!body.contains("@cinch_alloc"), "FixedArray must not heap allocate: {body}");

    let code = run_source(src).expect("run fixed_array_scalar_and_nested_no_alloc");
    assert_eq!(code, 0);
}

#[test]
fn fixed_array_rejects_out_of_bounds_and_counted_elements() {
    expect_err(
        "fixed-array-oob",
        r#"
module Main where
  bad :: Unit -> U64
  bad _ =
    let a = fixedArrayRepeat 4 (0 :: U64) in
    fixedArrayGet a 4
  main :: Unit -> Int
  main _ = 0
"#,
        &["bounds", "FixedArray", "<"],
    );

    expect_err(
        "fixed-array-counted",
        r#"
module Main where
  bad :: Unit -> FixedArray 4 String
  bad _ = fixedArrayRepeat 4 "hi"
  main :: Unit -> Int
  main _ = 0
"#,
        &["POD", "non-refcounted", "FixedArray"],
    );
}

// =========================================================================
// Part 4: withLocal, in-place field & array mutation, sub-borrows, ptrs
// =========================================================================

#[test]
fn with_local_scalar_value_and_fixed_array_no_alloc() {
    let src = r#"
module Main where
  #[repr(value)]
  record Inner where
    x :: U64
    y :: U64

  #[repr(value)]
  record Outer where
    inner :: Inner
    tag :: U64

  #[no-alloc]
  bumpInner :: forall r. Mut r Inner ->{State r} Unit
  bumpInner m =
    let _ = refSet m.x (wrapping-add-u64 m.x 10) in
    refSet m.y (wrapping-add-u64 m.y 20)

  #[no-alloc]
  readInner :: forall r. Ref r Inner ->{State r} U64
  readInner r = wrapping-add-u64 r.x r.y

  #[no-alloc]
  runLocal :: Unit -> U64
  runLocal _ =
    let s = withLocal (5 :: U64) (\m ->
      let _ = refSet m 12 in
      copyOut m
    ) in
    let v = withLocal (Outer (Inner 1 2) 3) (\m ->
      let _ = refSet m.inner.x 100 in
      let _ = refSet m.tag 7 in
      let _ = bumpInner (mutBorrow m.inner) in
      wrapping-add-u64 (readInner (borrow m.inner)) m.tag
    ) in
    let arrSum = withLocal (fixedArrayRepeat 4 (1 :: U64)) (\buf ->
      let _ = fixedArraySet buf 0 10 in
      let _ = fixedArraySet buf 3 40 in
      wrapping-add-u64 (wrapping-add-u64 (fixedArrayGet buf 0) (fixedArrayGet buf 1)) (fixedArrayGet buf 3)
    ) in
    wrapping-add-u64 (wrapping-add-u64 s v) arrSum

  main :: Unit -> Int
  main _ =
    -- s = 12, v = (110 + 22) + 7 = 139, arrSum = 10 + 1 + 40 = 51 => total 202
    if runLocal () == 202 then 0 else 1
"#;

    let ir = emit_ir(src).expect("emit IR");
    let body = body_of(&ir, "Main.runLocal");
    assert!(!body.contains("@cinch_alloc"), "withLocal must not heap allocate: {body}");

    let code = run_source(src).expect("run with_local_scalar_value_and_fixed_array_no_alloc");
    assert_eq!(code, 0);
}

#[test]
fn with_local_rejects_region_escape() {
    expect_err(
        "with-local-escape-direct",
        r#"
module Main where
  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  bad :: forall r. Unit -> Mut r Pair
  bad _ = withLocal (Pair 1 2) (\m -> m)
  main :: Unit -> Int
  main _ = 0
"#,
        &["escape", "withLocal", "region"],
    );

    expect_err(
        "with-local-escape-subborrow",
        r#"
module Main where
  #[repr(value)]
  record Inner where
    a :: U64
  #[repr(value)]
  record Outer where
    inner :: Inner

  bad :: forall r. Unit -> Ref r Inner
  bad _ = withLocal (Outer (Inner 1)) (\m -> borrow m.inner)
  main :: Unit -> Int
  main _ = 0
"#,
        &["escape", "withLocal", "region"],
    );
}

#[test]
fn packed_misaligned_sub_borrow_rejected() {
    expect_err(
        "packed-misaligned-borrow",
        r#"
module Main where
  #[repr(C, value, packed(1))]
  record Packed where
    tag :: U8
    val :: U64

  bad :: Unit -> U64
  bad _ =
    withLocal (Packed 1 2) (\m ->
      let r = borrow m.val in
      copyOut r
    )
  main :: Unit -> Int
  main _ = 0
"#,
        &["misaligned", "packed", "alignment"],
    );
}

#[test]
fn no_raw_ptr_rejects_mut_as_ptr() {
    expect_err(
        "no-raw-ptr-mut-as-ptr",
        r#"
module Main where
  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  #[no-raw-ptr]
  bad :: Unit -> U64
  bad _ =
    withLocal (Pair 1 2) (\m ->
      let _ = mutAsPtr m in
      0
    )

  main :: Unit -> Int
  main _ = 0
"#,
        &["no-raw-ptr", "raw pointer", "mutAsPtr"],
    );
}

// =========================================================================
// Item 3 (DEFERRED): Uniform-word polymorphic / closure / boxed ADT boundaries
// =========================================================================

#[test]
fn unboxed_value_rejected_at_polymorphic_closure_and_boxed_adt() {
    expect_err(
        "poly-id-value",
        r#"
module Main where
  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  id :: forall a. a -> a
  id x = x

  bad :: Unit -> Pair
  bad _ = id (Pair 1 2)

  main :: Unit -> Int
  main _ = 0
"#,
        &["unboxed", "polymorphic", "uniform-word"],
    );

    expect_err(
        "closure-capture-value",
        r#"
module Main where
  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  makeFn :: Pair -> (U64 -> U64)
  makeFn p = \k -> wrapping-add-u64 p.a k

  main :: Unit -> Int
  main _ = 0
"#,
        &["unboxed", "closure", "capture"],
    );

    expect_err(
        "boxed-adt-field-value",
        r#"
module Main where
  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  data Boxed where
    MkBoxed :: Pair -> Boxed

  main :: Unit -> Int
  main _ = 0
"#,
        &["unboxed", "boxed", "repr(value)"],
    );
}
