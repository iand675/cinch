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

fn emit_driver_ir(name: &str, src: &str) -> Result<String, String> {
    let dir = std::env::temp_dir().join(format!("cinch-unboxed-ir-{}-{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    let out = dir.join("Main.ll");
    common::drive_with_profile(
        cinch::driver::Mode::EmitIr,
        cinch::check::StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(out.clone()),
        &TargetProfile::native(),
    )?;
    let ir = std::fs::read_to_string(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    let _ = std::fs::remove_dir_all(&dir);
    Ok(ir)
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
// Part 3: Unboxed-by-default polymorphism, Box/Boxed, inline ADTs & closures
// =========================================================================

#[test]
fn poly_layout_keyed_specialization_and_sharing() {
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

  #[repr(value)]
  record Point2 where
    px :: U64
    py :: U64

  #[repr(value)]
  record Vec3 where
    x :: U64
    y :: U64
    z :: U64

  #[repr(value, align(64))]
  record CacheLine where
    v :: U64

  #[inline(never)]
  myId :: forall a. a -> a
  myId x = x

  #[inline(never)]
  pickFirst :: forall a b. a -> b -> a
  pickFirst a _ = a

  main :: Unit -> Int
  main _ =
    let s = myId (Small8 3 4) in
    let v2 = myId (Vec2 10 20) in
    let p2 = myId (Point2 30 40) in
    let v3 = myId (Vec3 100 200 300) in
    let cl = myId (CacheLine 77) in
    let fa = myId (fixedArrayRepeat 4 (5 :: U64)) in
    let picked = pickFirst v3 cl in
    if s.a == 3 && s.b == 4
      && v2.x == 10 && v2.y == 20
      && p2.px == 30 && p2.py == 40
      && v3.x == 100 && v3.y == 200 && v3.z == 300
      && cl.v == 77
      && fixedArrayGet fa 2 == 5
      && picked.z == 300
    then 0 else 1
"#;

    let ir = emit_driver_ir("poly_layout_spec", src).expect("emit IR for poly_layout_keyed_specialization_and_sharing");
    // Vec2 and Point2 share the single @V16a8 specialization of myId.
    let my_id_defs: Vec<&str> = ir.lines().filter(|l| l.starts_with("define ") && l.contains("myId")).collect();
    assert!(
        ir.contains("Main.myId@V16a8") || ir.contains("Main.myId.rV16a8"),
        "expected layout-keyed V16a8 specialization in IR; got: {my_id_defs:?}"
    );
    assert!(
        ir.contains("Main.myId@V24a8") || ir.contains("Main.myId.rV24a8"),
        "expected layout-keyed V24a8 specialization in IR; got: {my_id_defs:?}"
    );
    assert!(
        ir.contains("Main.myId@V64a64") || ir.contains("Main.myId.rV64a64"),
        "expected layout-keyed V64a64 specialization in IR; got: {my_id_defs:?}"
    );
    let id24 = try_body_of(&ir, "Main.myId@V24a8")
        .or_else(|| try_body_of(&ir, "Main.myId.rV24a8"))
        .unwrap_or_else(|| panic!("Main.myId V24a8: no define found in IR; got: {my_id_defs:?}"));
    assert!(!id24.contains("@cinch_alloc"), "myId V24a8 must not heap-allocate: {id24}");

    let code = run_source(src).expect("run poly_layout_keyed_specialization_and_sharing");
    assert_eq!(code, 0);
}

#[test]
fn inline_val_in_adts_tuples_records_and_box() {
    let src = r#"
module Main where
  import Data.List

  #[repr(value)]
  record Vec3 where
    x :: U64
    y :: U64
    z :: U64

  #[repr(value, align(64))]
  record CacheLine where
    v :: U64

  record Particle where
    tag :: String
    pos :: Vec3
    note :: String

  addVec3 :: Vec3 -> Vec3 -> Vec3
  addVec3 a b = Vec3 (wrapping-add-u64 a.x b.x) (wrapping-add-u64 a.y b.y) (wrapping-add-u64 a.z b.z)

  sumList :: List Vec3 -> Vec3
  sumList xs = case xs of
    Nil -> Vec3 0 0 0
    Cons h t -> addVec3 h (sumList t)

  unwrapOpt :: Option Vec3 -> Vec3
  unwrapOpt o = case o of
    None -> Vec3 0 0 0
    Some v -> v

  checkTup :: (Vec3, CacheLine) -> Bool
  checkTup tup = case tup of
    (tv, tc) -> tv.x == 7 && tv.z == 9 && tc.v == 99

  unboxCl :: Box CacheLine -> U64
  unboxCl b = case b of
    Box cl -> cl.v

  main :: Unit -> Int
  main _ =
    let fromOpt = unwrapOpt (Some (Vec3 1 2 3)) in
    let xs = Cons (Vec3 10 20 30) (Cons (Vec3 1 2 3) Nil) in
    let tot = sumList xs in
    let tupOk = checkTup (Vec3 7 8 9, CacheLine 99) in
    let part = Particle "electron" (Vec3 4 5 6) "ok" in
    let b = Box (Vec3 11 22 33) in
    let u = unbox b in
    let m = unboxCl (Box (CacheLine 55)) in
    if fromOpt.x == 1 && fromOpt.z == 3
      && tot.x == 11 && tot.y == 22 && tot.z == 33
      && tupOk
      && part.pos.x == 4 && part.pos.y == 5 && part.pos.z == 6
      && string-compare part.tag "electron" == 0
      && u.x == 11 && u.y == 22 && u.z == 33
      && m == 55
    then 0 else 1
"#;

    let code = run_source(src).expect("run inline_val_in_adts_tuples_records_and_box");
    assert_eq!(code, 0);
}

#[test]
fn closures_indirect_calls_paps_and_typeclasses_over_val() {
    let src = r#"
module Main where
  #[repr(value)]
  record Vec3 where
    x :: U64
    y :: U64
    z :: U64

  #[repr(value, align(64))]
  record CacheLine where
    v :: U64

  addVec3 :: Vec3 -> Vec3 -> Vec3
  addVec3 a b = Vec3 (wrapping-add-u64 a.x b.x) (wrapping-add-u64 a.y b.y) (wrapping-add-u64 a.z b.z)

  #[inline(never)]
  applyTwice :: forall a. (a -> a) -> a -> a
  applyTwice f x = f (f x)

  #[inline(never)]
  applyBinary :: forall a. (a -> a -> a) -> a -> a -> a
  applyBinary f x y = f x y

  #[inline(never)]
  makeAdder :: Vec3 -> (Vec3 -> Vec3)
  makeAdder base = \delta -> addVec3 base delta

  #[inline(never)]
  makeTaggedAdder :: String -> Vec3 -> (Vec3 -> Vec3)
  makeTaggedAdder _ base = \delta -> addVec3 base delta

  #[inline(never)]
  bumpCl :: CacheLine -> CacheLine
  bumpCl c = CacheLine (wrapping-add-u64 c.v 5)

  implicit instance Eq_Vec3 :: Eq Vec3 where
    (==) a b = a.x == b.x && a.y == b.y && a.z == b.z
    (/=) a b = not (a == b)

  #[inline(never)]
  sameViaEq :: forall a. using (eq :: Eq a) => a -> a -> Bool
  sameViaEq x y = x == y

  main :: Unit -> Int
  main _ =
    -- Closure capturing Vec3 + nested indirect f (f x)
    let f = makeAdder (Vec3 1 2 3) in
    let r1 = applyTwice f (Vec3 10 20 30) in
    -- Mixed Obj + Val closure capture
    let g = makeTaggedAdder "hello" (Vec3 5 5 5) in
    let r2 = g (Vec3 1 2 3) in
    -- Partial application (PAP) of a Val-taking, Val-returning function
    let pap = addVec3 (Vec3 100 200 300) in
    let r3 = applyTwice pap (Vec3 1 1 1) in
    -- Indirect 2-arg call (over-application / multi-word argv + trailing sret)
    let r4 = applyBinary addVec3 (Vec3 2 3 4) (Vec3 8 7 6) in
    -- Curried indirect over-application (mk returns closure returning Vec3)
    let r5 = applyBinary makeAdder (Vec3 3 4 5) (Vec3 7 6 5) in
    -- Over-aligned align(64) through indirect applyTwice
    let c = applyTwice bumpCl (CacheLine 10) in
    -- Typeclass Eq dictionary method over Vec3
    let eqOk = sameViaEq r4 r5 && not (sameViaEq r1 r2) in
    if r1.x == 12 && r1.y == 24 && r1.z == 36
      && r2.x == 6 && r2.y == 7 && r2.z == 8
      && r3.x == 201 && r3.y == 401 && r3.z == 601
      && r4.x == 10 && r4.y == 10 && r4.z == 10
      && r5.x == 10 && r5.y == 10 && r5.z == 10
      && c.v == 20
      && eqOk
    then 0 else 1
"#;

    let code = run_source(src).expect("run closures_indirect_calls_paps_and_typeclasses_over_val");
    assert_eq!(code, 0);
}

#[test]
fn boxed_constraint_uniform_containers_and_shadowable_box() {
    // 1. Boxed constraint rejects #[repr(value)] and FixedArray, accepts boxed types & Box a
    expect_err(
        "boxed-constraint-rejects-value",
        r#"
module Main where
  #[repr(value)]
  record Small where
    x :: U64

  needBoxed :: forall a. using (b :: Boxed a) => a -> Int
  needBoxed _ = 0

  bad :: Unit -> Int
  bad _ = needBoxed (Small 1)

  main :: Unit -> Int
  main _ = 0
"#,
        &["Boxed", "unboxed", "Box"],
    );

    expect_err(
        "boxed-constraint-rejects-fixed-array",
        r#"
module Main where
  needBoxed :: forall a. using (b :: Boxed a) => a -> Int
  needBoxed _ = 0

  bad :: Unit -> Int
  bad _ = needBoxed (fixedArrayRepeat 2 (0 :: U64))

  main :: Unit -> Int
  main _ = 0
"#,
        &["Boxed", "FixedArray", "Box"],
    );

    // 2. Uniform-word runtime containers refuse bare unboxed values and suggest Box
    let tvar_err = emit_driver_ir(
        "tvar_rejects_val",
        r#"
module Main where
  import Effect.Stm

  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  main :: Unit ->{STM, Diverge} Int
  main _ =
    let _ = atomically (tvar-new (Pair 1 2)) in
    0
"#,
    )
    .expect_err("TVar of bare unboxed Pair must be refused");
    assert!(
        tvar_err.contains("Box") && (tvar_err.contains("TVar") || tvar_err.contains("unboxed")),
        "unexpected TVar refusal error: {tvar_err}"
    );

    // 3. Explicit Box satisfies Boxed constraint and works inside TVar / user containers
    let ok_src = r#"
module Main where
  import Effect.Stm

  #[repr(value)]
  record Pair where
    a :: U64
    b :: U64

  needBoxed :: forall a. using (b :: Boxed a) => a -> Int
  needBoxed _ = 1

  main :: Unit ->{STM, Diverge} Int
  main _ =
    let a = needBoxed (42 :: Int) in
    let b = needBoxed "hi" in
    let c = needBoxed (Box (Pair 10 20)) in
    let d = needBoxed (Some (Pair 1 2)) in
    let tv = atomically (tvar-new (Box (Pair 3 4))) in
    let v = unbox (atomically (tvar-read tv)) in
    if a + b + c + d == 4 && wrapping-add-u64 v.a v.b == 7 then 0 else 1
"#;
    let code = run_source(ok_src).expect("run explicit Box with Boxed and TVar");
    assert_eq!(code, 0);

    // 4. Local user `data Box` and `unbox` shadow the builtin cleanly
    let shadow_src = r#"
module Main where
  data Box where
    Box :: Int -> Box

  unbox :: Box -> Int
  unbox b = case b of
    Box n -> n

  main :: Unit -> Int
  main _ = if unbox (Box 42) == 42 then 0 else 1
"#;
    let code2 = run_source(shadow_src).expect("run shadowable Box");
    assert_eq!(code2, 0);
}

#[test]
fn cross_module_cpk_polymorphic_val_specialization() {
    let dir = std::env::temp_dir().join(format!("cinch-unboxed-cpk-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");

    let lib_src = r#"
module Lib where
  export Wrap, MkWrap, polyId, polyFirst, unwrap, libBase

  data Wrap a where
    MkWrap :: a -> Wrap a

  #[inline(never)]
  polyId :: forall a. a -> a
  polyId x = x

  #[inline(never)]
  polyFirst :: forall a b. a -> b -> a
  polyFirst a _ = a

  #[inline(never)]
  unwrap :: forall a. Wrap a -> a
  unwrap w = case w of
    MkWrap x -> x

  libBase :: U64
  libBase = 7
"#;

    let main_src = r#"
module Main where
  import Lib

  #[repr(value)]
  record Vec3 where
    x :: U64
    y :: U64
    z :: U64

  main :: Unit -> Int
  main _ =
    let a = polyId (Vec3 10 20 30) in
    let b = unwrap (MkWrap (Vec3 1 2 3)) in
    let c = polyFirst (Vec3 4 5 6) libBase in
    if a.x == 10 && a.z == 30
      && b.x == 1 && b.y == 2 && b.z == 3
      && c.x == 4 && c.z == 6
      && libBase == 7
    then 0 else 1
"#;

    std::fs::write(dir.join("Lib.cinch"), lib_src).expect("write Lib.cinch");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main.cinch");

    // 1. Multi-module compilation (runs on both seed and Stage 3): `Main`
    //    specializes `Lib`'s polymorphic functions and constructors at `Vec3`.
    let direct_code = common::drive_opts(
        cinch::driver::Mode::Run,
        &dir,
        "Main.cinch",
        None,
        &cinch::driver::DriverOptions::default(),
    )
    .expect("run multi-module Lib + Main");
    assert_eq!(direct_code, 0);

    // 2. Prebuilt package object `Lib.o` + `build/package.cpk` (carrying
    //    polymorphic templates for `Lib.polyId`, `Lib.polyFirst`, `Lib.MkWrap`,
    //    `Lib.unwrap` without knowing anything about downstream `Main.Vec3`).
    //    (Gated to seed mode because `common::external_frontend` does not
    //    forward `emit_modules` / `package_objects` flags yet.)
    if common::external_frontend::configured().expect("frontend").is_none() {
        let lib_obj = dir.join("Lib.o");
        let lib_opts = cinch::driver::DriverOptions {
            emit_modules: vec!["Lib".to_string()],
            ..cinch::driver::DriverOptions::default()
        };
        common::drive_opts(
            cinch::driver::Mode::EmitObject,
            &dir,
            "Lib.cinch",
            Some(lib_obj.clone()),
            &lib_opts,
        )
        .expect("emit Lib.o + package.cpk");

        let pkg_cpk = dir.join("build").join("package.cpk");
        assert!(pkg_cpk.exists(), "expected package.cpk at {}", pkg_cpk.display());
        let table = cinch::ifaceopt::load_package(&pkg_cpk).expect("load Lib package.cpk");
        let tpl_names: Vec<&str> = table.templates.iter().map(|t| t.id.0.as_str()).collect();
        assert!(
            tpl_names.contains(&"Lib.polyId") && tpl_names.contains(&"Lib.MkWrap") && tpl_names.contains(&"Lib.unwrap"),
            "expected polymorphic templates in Lib package.cpk, got: {tpl_names:?}"
        );

        let main_opts = cinch::driver::DriverOptions {
            package_objects: vec![lib_obj],
            ..cinch::driver::DriverOptions::default()
        };
        let code = common::drive_opts(
            cinch::driver::Mode::Run,
            &dir,
            "Main.cinch",
            None,
            &main_opts,
        )
        .expect("run Main linked against prebuilt Lib.o");
        assert_eq!(code, 0);
    }

    let _ = std::fs::remove_dir_all(&dir);
}
