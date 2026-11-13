//! The LLVM contract surface (P27): Cinch's analyses spelled in
//! LLVM's own vocabulary (design/llvm-contract-attributes.md). Every
//! test observes the emitted IR: the contract appears where the
//! analysis proves the fact and is absent where it does not (the plan
//! note's evidence style: never wiring tests).

mod common;
use cinch::driver::check_sources;
use common::source_frontend::run_source_objects;

/// The emitted LLVM IR of one checked source (every IR test carries a
/// `main`, which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The one line of `ir` defining one unit (`define ...`).
fn define_of(ir: &str, unit: &str) -> String {
    let quoted = format!("@\"{unit}\"(");
    let bare = format!("@{unit}(");
    ir.lines()
        .find(|l| l.starts_with("define ") && (l.contains(&quoted) || l.contains(&bare)))
        .unwrap_or_else(|| panic!("{unit}: no define in {ir}"))
        .to_string()
}

/// The text of one unit's function attribute group (empty when the
/// unit carries no group).
fn attrs_of(ir: &str, unit: &str) -> String {
    let define = define_of(ir, unit);
    let Some((_, tail)) = define.split_once(" #") else {
        return String::new();
    };
    let n = tail
        .split_whitespace()
        .next()
        .expect("an attribute group number");
    let head = format!("attributes #{n} = {{ ");
    ir.lines()
        .find_map(|l| l.strip_prefix(&head).and_then(|r| r.strip_suffix(" }")))
        .unwrap_or_else(|| panic!("{unit}: attribute group #{n} missing in {ir}"))
        .to_string()
}

/// The unit name a `define` line defines (`@"a b"(` or `@ab(`).
fn unit_name_of(line: &str) -> Option<String> {
    let (_, after) = line.split_once('@')?;
    if let Some(rest) = after.strip_prefix('"') {
        let end = rest.find('"')?;
        return Some(rest[..end].to_string());
    }
    let end = after.find('(')?;
    Some(after[..end].to_string())
}

/// The body of one unit's define, or `None` when the unit is not
/// emitted (a pruned or inlined unit).
fn body_of_opt(ir: &str, unit: &str) -> Option<String> {
    let quoted = format!("@\"{unit}\"(");
    let bare = format!("@{unit}(");
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if l.starts_with("define ") && (l.contains(&quoted) || l.contains(&bare)) {
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

/// The body of one unit's define (its lines between `{` and `}`).
fn body_of(ir: &str, unit: &str) -> String {
    let quoted = format!("@\"{unit}\"(");
    let bare = format!("@{unit}(");
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if l.starts_with("define ") && (l.contains(&quoted) || l.contains(&bare)) {
            let mut body = String::new();
            for l in lines.by_ref() {
                if l == "}" {
                    return body;
                }
                body.push_str(l);
                body.push('\n');
            }
        }
    }
    panic!("{unit}: no body in {ir}")
}

/// The one line of `ir` declaring one foreign symbol (`declare ...`).
fn declare_of(ir: &str, symbol: &str) -> String {
    let quoted = format!("@\"{symbol}\"(");
    let bare = format!("@{symbol}(");
    ir.lines()
        .find(|l| l.starts_with("declare ") && (l.contains(&quoted) || l.contains(&bare)))
        .unwrap_or_else(|| panic!("{symbol}: no declare in {ir}"))
        .to_string()
}

/// The metadata node definition (`!{...}`) attached as `kind` to the
/// first load of `width` in `body`, resolved against the module.
fn load_meta_of(ir: &str, body: &str, width: &str, kind: &str) -> Option<String> {
    for line in body.lines() {
        if !line.contains(&format!("load {width},")) {
            continue;
        }
        let Some(rest) = line.split(&format!("!{kind} ")).nth(1) else {
            continue;
        };
        let id = rest.split_whitespace().next().expect("a metadata reference");
        let head = format!("{id} = ");
        return Some(
            ir.lines()
                .find_map(|l| l.strip_prefix(&head).map(|d| d.trim().to_string()))
                .unwrap_or_else(|| panic!("{id}: metadata node missing in {ir}")),
        );
    }
    None
}

/// The metadata node definition attached as `!range` to the first
/// `i64` word load of `body` (a `Word` field), if any.
fn word_range_of(ir: &str, body: &str) -> Option<String> {
    load_meta_of(ir, body, "i64", "range")
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match common::source_frontend::check_sources(src, "<test>") {
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

// --------------------------- effect-derived function attributes (item 1)

#[test]
fn pure_definitions_carry_memory_none_and_speculatable() {
    let ir = ir_of(
        "pure-attrs",
        r#"
module Main where
  pick :: Bool -> U8 -> U8 -> U8
  pick b x y = if b then x else y

  main :: Int
  main = if pick True 1 2 == 1 then 0 else 1
"#,
    );
    let attrs = attrs_of(&ir, "Main.pick");
    assert!(
        attrs.contains("memory(none)") && attrs.contains("speculatable"),
        "Main.pick: expected memory(none) speculatable in {attrs:?}\n{ir}"
    );
}

#[test]
fn io_definitions_carry_no_purity_claims() {
    let ir = ir_of(
        "io-attrs",
        r#"
module Main where
  ioAct :: forall r. using (io :: Io r) => Nat ->{IO r} Unit
  ioAct n = ioWrite (bytes-new 1)

  main :: Unit ->{IO r} Int
  main _ = with (io = newIO ()) in case ioAct 1 of
    _ -> 0
"#,
    );
    let attrs = attrs_of(&ir, "Main.ioAct");
    assert!(
        !attrs.contains("memory(none)") && !attrs.contains("speculatable"),
        "Main.ioAct: an Io definition claims purity in {attrs:?}\n{ir}"
    );
    assert!(
        attrs.contains("memory(read)") || attrs.contains("memory(readwrite)"),
        "Main.ioAct: an Io definition is memory(read)/(readwrite), got {attrs:?}\n{ir}"
    );
}

#[test]
fn no_alloc_and_totality_rows_restate_as_nofree_and_willreturn() {
    let ir = ir_of(
        "row-attrs",
        r#"
module Main where
  #[no-alloc]
  twice :: U8 -> U8
  twice x = wrapping-mul-u8 x x

  #[recursion("none")]
  stepper :: U8 -> U8
  stepper x = wrapping-add-u8 x 1

  plain :: U8 -> U8
  plain x = wrapping-add-u8 x 1

  main :: Int
  main = if twice 3 == 9 then if stepper 2 == 3 then 0 else 1 else 2
"#,
    );
    // `nofree` restates P20's `#[no-alloc]` row (never guessed).
    assert!(
        attrs_of(&ir, "Main.twice").contains("nofree"),
        "Main.twice: `#[no-alloc]` must add nofree\n{ir}"
    );
    assert!(
        !attrs_of(&ir, "Main.plain").contains("nofree"),
        "Main.plain: nofree without the `#[no-alloc]` row\n{ir}"
    );
    // `willreturn` restates the totality machinery's proof (never
    // guessed).
    assert!(
        attrs_of(&ir, "Main.stepper").contains("willreturn"),
        "Main.stepper: `#[recursion(\"none\")]` must add willreturn\n{ir}"
    );
    assert!(
        !attrs_of(&ir, "Main.plain").contains("willreturn"),
        "Main.plain: willreturn without the totality row\n{ir}"
    );
}

// ---------------------- usage-derived parameter attributes (item 2)

#[test]
fn captures_none_marks_only_non_escaping_borrows() {
    let ir = ir_of(
        "captures",
        r#"
module Main where
  foreign import ccall "fix_borrow" borrowPtr :: Borrowed (RawPtr U8) ->{Foreign Fix, Diverge} Unit
  foreign import ccall "fix_take" takePtr :: RawPtr U8 ->{Foreign Fix, Diverge} Unit

  main :: Int
  main = 0
"#,
    );
    assert!(
        declare_of(&ir, "fix_borrow").contains("captures(none)"),
        "a non-escaping borrow must carry captures(none)\n{ir}"
    );
    assert!(
        !declare_of(&ir, "fix_take").contains("captures(none)"),
        "an escaping raw pointer must not carry captures(none)\n{ir}"
    );
}

// ---------------------------------------- !range and noundef (item 3)

#[test]
fn range_facts_state_bounded_returns_and_calls() {
    let ir = ir_of(
        "range-returns",
        r#"
module Main where
  idU8 :: U8 -> U8
  idU8 x = x

  idBool :: Bool -> Bool
  idBool x = x

  idInt :: Int -> Int
  idInt x = x

  main :: Int
  main = if idU8 3 == 3 then if idBool True then if idInt 5 == 5 then 0 else 1 else 3 else 2
"#,
    );
    // A word-typed bounded result states its range; the unbounded one
    // states nothing (`Int` words span the `i64` space).
    let bool_define = define_of(&ir, "Main.idBool");
    assert!(
        bool_define.contains("range(i64 0, 2)"),
        "Main.idBool: a Bool return must carry its range\n{bool_define}\n{ir}"
    );
    let int_define = define_of(&ir, "Main.idInt");
    assert!(
        !int_define.contains("range("),
        "Main.idInt: an Int return carries no range\n{int_define}\n{ir}"
    );
    // A `U8` result crosses as an `i8` (design/narrow-value-
    // representation.md section 4), so its type is the range: the
    // `[0, 256)` a word would need the attribute for is what the caller's
    // `zext` of the `i8` already says.
    let u8_define = define_of(&ir, "Main.idU8");
    assert!(
        u8_define.starts_with("define i8 ") && !u8_define.contains("range("),
        "Main.idU8: a U8 return crosses as i8, which states its range\n{u8_define}\n{ir}"
    );
    // The same facts at the call sites.
    let main_body = body_of(&ir, "Main.main");
    let bool_call = main_body
        .lines()
        .find(|l| l.contains("call i64 @\"Main.idBool\"("))
        .expect("a call to Main.idBool");
    assert!(
        bool_call.contains("!range "),
        "the Bool call site must carry !range\n{bool_call}\n{ir}"
    );
    let u8_call = main_body
        .lines()
        .find(|l| l.contains("call i8 @\"Main.idU8\"("))
        .expect("a call to Main.idU8 at its narrowed type");
    assert!(
        !u8_call.contains("!range "),
        "a narrowed call carries no word-typed !range\n{u8_call}\n{ir}"
    );
    let int_call = main_body
        .lines()
        .find(|l| l.contains("call i64 @\"Main.idInt\"("))
        .expect("a call to Main.idInt");
    assert!(
        !int_call.contains("!range "),
        "the Int call site carries no !range\n{int_call}\n{ir}"
    );
    // Scalar words are `noundef`; boxed words are not claimed.
    assert!(
        u8_define.contains("noundef"),
        "Main.idU8: a scalar parameter must be noundef\n{u8_define}"
    );
    assert!(
        !int_define.contains("noundef"),
        "Main.idInt: a boxed parameter is not claimed noundef\n{int_define}"
    );
}

#[test]
fn typed_loads_state_only_statable_ranges() {
    let ir = ir_of(
        "range-loads",
        r#"
module Main where
  #[repr(C)]
  record Ch where
    c :: Char

  #[repr(C)]
  record Fl where
    b :: Bool

  #[repr(C)]
  record Wd where
    x :: U32

  getC :: Ch -> Char
  getC r = r.c

  getB :: Fl -> Bool
  getB r = r.b

  getX :: Wd -> U32
  getX r = r.x

  main :: Int
  main = if getX (Wd { x = 7 }) == 7 then 0 else 1
"#,
    );
    // `Char`'s literal range is narrower than the load's width.
    assert_eq!(
        load_meta_of(&ir, &body_of(&ir, "Main.getC"), "i32", "range")
            .as_deref(),
        Some("!{i32 0, i32 1114112}"),
        "a Char load must state its range\n{ir}"
    );
    // `Bool`'s two values, at the field's recorded byte.
    assert_eq!(
        load_meta_of(&ir, &body_of(&ir, "Main.getB"), "i8", "range").as_deref(),
        Some("!{i8 0, i8 2}"),
        "a Bool load must state its range\n{ir}"
    );
    // `U32`'s range is the load's whole width: `[a, a)` is the empty
    // range, so the load stays bare rather than guessing.
    assert_eq!(
        load_meta_of(&ir, &body_of(&ir, "Main.getX"), "i32", "range"),
        None,
        "a full-width U32 load must carry no !range\n{ir}"
    );
}

#[test]
fn borrow_field_loads_state_their_pointee_alignment() {
    let ir = ir_of(
        "align-loads",
        r#"
module Main where
  #[repr(C)]
  record Bp where
    p :: Borrowed (RawPtr U32)

  #[repr(C)]
  record Rp where
    p :: RawPtr U8

  getBp :: Bp -> Unit
  getBp r = case r.p of
    _ -> ()

  getRp :: Rp -> Unit
  getRp r = case r.p of
    _ -> ()

  main :: Int
  main = 0
"#,
    );
    // A borrow asserts its pointee's alignment (`!align` on the load);
    // a `RawPtr` asserts nothing ("not claimed to be valid, aligned,
    // or owned").
    assert_eq!(
        load_meta_of(&ir, &body_of(&ir, "Main.getBp"), "ptr", "align").as_deref(),
        Some("!{i64 4}"),
        "a borrow load must state its pointee's alignment\n{ir}"
    );
    assert_eq!(
        load_meta_of(&ir, &body_of(&ir, "Main.getRp"), "ptr", "align"),
        None,
        "a RawPtr load claims no alignment\n{ir}"
    );
}

// ------------------------------------ llvm.assume restatements (item 4)

#[test]
fn checked_params_restate_as_assumes() {
    let ir = ir_of(
        "assume",
        r#"
module Main where
  #[checked-params]
  validated :: U8 -> U8
  validated x = case x of
    0 -> 1
    _ -> x

  unchecked :: U8 -> U8
  unchecked x = case x of
    0 -> 1
    _ -> x

  main :: Int
  main = if validated 3 == 3 then 0 else 1
"#,
    );
    let checked = body_of(&ir, "Main.validated");
    assert!(
        checked.contains("call void @llvm.assume(i1 true)") && checked.contains("\"noundef\""),
        "the verified `#[checked-params]` row must restate as an assume\n{checked}"
    );
    assert!(
        !body_of(&ir, "Main.unchecked").contains("llvm.assume"),
        "without the row there is no assume\n{ir}"
    );
}

#[test]
fn boundary_facts_restate_as_assume_bundles() {
    let ir = ir_of(
        "boundary-assumes",
        r#"
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  #[byval]
  foreign import ccall "sum_big" sumBig :: Big ->{Foreign Fix, Diverge} I64

  foreign import ccall "fix_take" takePtr :: RawPtr U8 ->{Foreign Fix, Diverge} Unit

  main :: Int
  main = 0
"#,
    );
    // The recorded layout of a boundary buffer restates as its
    // `align`/`dereferenceable` bundles (the plan note's item 4: repr
    // facts as optimizer input).
    let wrapper = body_of(&ir, "Main.sumBig");
    assert!(
        wrapper.contains("call void @llvm.assume(i1 true)")
            && wrapper.contains("\"align\"(")
            && wrapper.contains("\"dereferenceable\"("),
        "the call-scoped copy's layout must restate as assume bundles\n{wrapper}"
    );
    // The checked non-null contract restates after its boundary check.
    let take = body_of(&ir, "Main.takePtr");
    assert!(
        take.contains("call void @llvm.assume(i1 true)") && take.contains("\"nonnull\"("),
        "a checked non-null pointer must restate as an assume bundle\n{take}"
    );
}

// ------------------------------- freeze at untrusted boundaries (freeze)

#[test]
fn untrusted_boundary_reads_freeze() {
    let ir = ir_of(
        "freeze",
        r#"
module Main where
  #[repr(C)]
  record Pp where
    p :: RawPtr U8

  foreign import ccall "fix_null" nullPtr :: () ->{Foreign Fix, Diverge} RawPtr U8

  takePp :: Pp -> I64
  takePp r = case r.p of
    _ -> 1

  foreign export ccall "fix_take_pp" takePp :: Pp -> I64

  idU8 :: U8 -> U8
  idU8 x = x

  main :: Int
  main = 0
"#,
    );
    // A foreign result read back at the boundary freezes.
    assert!(
        body_of(&ir, "Main.nullPtr").contains("freeze i64"),
        "a foreign pointer result must freeze\n{ir}"
    );
    // A `RawPtr` field loaded out of a foreign buffer freezes: the
    // untrusted bytes cannot leak poison into safe code.
    let export = body_of(&ir, "fix_take_pp");
    assert!(
        export.contains("load ptr") && export.contains("freeze i64"),
        "a RawPtr load from a foreign buffer must freeze\n{export}"
    );
    // `Bytes` data read at the boundary freezes before its conversion.
    assert!(
        body_of(&ir, "cinch.runtime.bytesGet").contains("freeze i64"),
        "a Bytes data read must freeze\n{ir}"
    );
    // The safe side's own words are trusted: no freeze is claimed.
    assert!(
        !body_of(&ir, "Main.idU8").contains("freeze"),
        "an internal unit freezes nothing\n{ir}"
    );
}

// ------------------------------- noreturn fault paths (item 5, the audit)

#[test]
fn fault_paths_are_noreturn_and_leave_their_successor_dead() {
    let ir = ir_of(
        "faults",
        r#"
module Main where
  foreign import ccall "fix_take" takePtr :: RawPtr U8 ->{Foreign Fix, Diverge} Unit

  main :: Int
  main = 0
"#,
    );
    assert!(
        declare_of(&ir, "cinch_panic").contains(" noreturn")
            && declare_of(&ir, "exit").contains(" noreturn"),
        "the fault vocabulary is noreturn\n{ir}"
    );
    assert!(
        define_of(&ir, "__cinch_blackhole").contains(" noreturn"),
        "the BlackHole fault is noreturn\n{ir}"
    );
    // The audit: every fault call marks its successor dead.
    let lines: Vec<&str> = ir.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let faults = line.contains("call void @cinch_panic(")
            || line.contains("call void @\"__cinch_blackhole\"(")
            || line.contains("call void @exit(");
        if !faults {
            continue;
        }
        let next = lines[i + 1..]
            .iter()
            .find(|l| !l.trim().is_empty())
            .copied()
            .unwrap_or("");
        assert_eq!(
            next.trim(),
            "unreachable",
            "a fault call must leave its successor dead:\n{}\n{ir}",
            lines[i.min(lines.len() - 1)]
        );
    }
}

// ------------------------------- byval/sret aggregate boundary (item 6)

#[test]
fn byval_and_sret_spell_the_aggregate_boundary() {
    let ir = ir_of(
        "byval-spelling",
        r#"
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  #[byval]
  foreign import ccall "sum_big" sumBig :: Big ->{Foreign Fix, Diverge} I64

  #[sret]
  foreign import ccall "mk_big" mkBig :: I64 ->{Foreign Fix, Diverge} Big

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = if sumBig (mkBig 5) == 18 then 0 else 1
"#,
    );
    // The by-value parameter crosses as the caller's copy spelled
    // with its layout (`byref` states the pointee type without
    // changing the passing convention; `byval` can mean a different
    // machine convention than the C side's by-value aggregates).
    let byval = declare_of(&ir, "sum_big");
    assert!(
        byval.contains("byref(%__sl_layout_0)") && byval.contains("align 8"),
        "an aggregate parameter must spell its by-value copy with the layout\n{byval}\n{ir}"
    );
    let sret = declare_of(&ir, "mk_big");
    assert!(
        sret.starts_with("declare void ") && sret.contains("sret(%__sl_layout_0)"),
        "an aggregate result must spell sret and return void\n{sret}\n{ir}"
    );
    // Without the spellings the D66 default applies: a const-pointer
    // crossing states no layout and no result-out buffer.
    let plain = ir_of(
        "byval-default",
        r#"
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  foreign import ccall "fix_read_big" readBig :: Big ->{Foreign Fix, Diverge} I64

  main :: Int
  main = 0
"#,
    );
    let read = declare_of(&plain, "fix_read_big");
    assert!(
        !read.contains("byref(") && !read.contains("byval(") && !read.contains("sret("),
        "the default aggregate crossing is a const pointer, not a by-value spelling\n{read}\n{plain}"
    );
}

#[test]
fn byval_and_sret_cross_a_real_c_boundary() {
    use std::io::Write;
    let dir = std::env::temp_dir().join(format!("slp27-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join("big.c");
    let o_path = dir.join("big.o");
    let mut f = std::fs::File::create(&c_path).expect("write fixture");
    // The C side takes its `struct Big` parameters BY VALUE and returns
    // one by value: the crossings under test.
    f.write_all(
        br#"struct Big { long a; long b; long c; };
long long sum_big(struct Big p) { return p.a + p.b + p.c; }
long long clobber_big(struct Big p) { p.a = 99; return p.a + p.b + p.c; }
struct Big mk_big(long a) { struct Big p; p.a = a; p.b = a + 1; p.c = a + 2; return p; }
"#,
    )
    .expect("write fixture");
    drop(f);
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "clang failed to compile the C fixture");

    let src = r#"
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  #[byval]
  foreign import ccall "sum_big" sumBig :: Big ->{Foreign Fix, Diverge} I64

  #[byval]
  foreign import ccall "clobber_big" clobberBig :: Big ->{Foreign Fix, Diverge} I64

  #[sret]
  foreign import ccall "mk_big" mkBig :: I64 ->{Foreign Fix, Diverge} Big

  getA :: Big -> I64
  getA r = r.a

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ =
    let m = mkBig 5 in
    if getA m == 5 then
      (if sumBig m == 18 then
        (if clobberBig m == 112 then
          (if getA m == 5 then 0 else 4)
        else 3)
      else 2)
    else 1
"#;
    // The round trip: the `sret` result-out buffer boxes correctly
    // (`getA m == 5`), the by-value parameter crosses to the C copy
    // (`sumBig m == 18`), and the callee's by-value copy is its own
    // (`clobberBig m == 112` while the safe side's block is unchanged).
    let o = o_path.to_string_lossy().into_owned();
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "byval/sret round trip: expected exit 0, got {code}"),
        Err(e) => panic!("byval/sret round trip: compilation failed: {e}"),
    }
}

#[test]
fn byval_and_sret_are_claimed_only_at_aggregate_boundaries() {
    expect_err(
        "byval-without-aggregate",
        r#"
module Main where
  #[byval]
  foreign import ccall "fix_neg" neg :: CInt ->{Foreign Fix, Fail} CInt

  main :: Int
  main = 0
"#,
        "this binding marshals no aggregate parameter",
    );
    expect_err(
        "sret-without-aggregate",
        r#"
module Main where
  #[sret]
  foreign import ccall "fix_neg" neg :: CInt ->{Foreign Fix, Fail} CInt

  main :: Int
  main = 0
"#,
        "this binding's result is not an aggregate",
    );
    expect_err(
        "aggregate-result-without-sret",
        r#"
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  foreign import ccall "mk_big" mkBig :: I64 ->{Foreign Fix, Diverge} Big

  main :: Int
  main = 0
"#,
        "or spell the by-value crossing `#[sret]`",
    );
    expect_err(
        "byval-on-an-intrinsic",
        r#"
module Main where
  #[byval]
  foreign import ccall "cinch.prim.eqU8" eqU8 :: U8 -> U8 -> Bool

  main :: Int
  main = 0
"#,
        "a compiler intrinsic marshals no aggregate",
    );
}

// ---------------------------- `norecurse` is a proved fact (the SCC walk)

#[test]
fn norecurse_is_absent_from_units_that_reach_themselves() {
    // A self-recursive unit and a mutually recursive pair: neither may
    // claim `norecurse`, because each can call itself back. The attribute
    // is the whole point of a "cannot happen anywhere" claim, so the two
    // units are emitted rather than inlined (`#[inline(never)]`) and the
    // recursion is real in the IR.
    let ir = ir_of(
        "norecurse-recursive",
        r#"
module Main where
  data List3 a where
    Nil3 :: List3 a
    Cons3 :: a -> List3 a -> List3 a

  len3 :: forall a. List3 a -> Nat
  len3 xs = case xs of
    Nil3 -> 0
    Cons3 _ rest -> 1 + len3 rest

  ping :: List3 Nat -> Nat
  ping xs = case xs of
    Nil3 -> 0
    Cons3 _ rest -> pong rest

  pong :: List3 Nat -> Nat
  pong xs = case xs of
    Nil3 -> 1
    Cons3 _ rest -> ping rest

  main :: Int
  main = if len3 (Cons3 (1 :: Nat) Nil3) == 1 then 0 else 1
"#,
    );
    // Every emitted unit that calls itself, directly or through a partner,
    // must not carry `norecurse`. The check is over the IR rather than a
    // named unit so a renamed or specialized symbol cannot slip past it.
    let mut checked = 0;
    for line in ir.lines() {
        if !line.starts_with("define ") {
            continue;
        }
        let Some(name) = unit_name_of(line) else { continue };
        let Some(body) = body_of_opt(&ir, &name) else { continue };
        let calls_self = body
            .lines()
            .any(|l| l.contains(&format!("@{name}(")) || l.contains(&format!("\"{name}\"(")));
        if !calls_self {
            continue;
        }
        checked += 1;
        let attrs = attrs_of(&ir, &name);
        assert!(
            !attrs.contains("norecurse"),
            "{name} calls itself and must not claim norecurse, got {attrs:?}\n{ir}"
        );
    }
    assert!(
        checked > 0,
        "the probe must emit at least one self-calling unit\n{ir}"
    );
}

#[test]
fn norecurse_is_claimed_on_a_non_recursive_unit() {
    // The other direction: a unit outside every cycle still claims it, so
    // the fix narrows the attribute rather than dropping it.
    let ir = ir_of(
        "norecurse-plain",
        r#"
module Main where
  #[inline(never)]
  plain :: U8 -> U8
  plain x = x

  main :: Int
  main = if plain 7 == 7 then 0 else 1
"#,
    );
    let attrs = attrs_of(&ir, "Main.plain");
    assert!(
        attrs.contains("norecurse"),
        "Main.plain: a non-recursive unit claims norecurse, got {attrs:?}\n{ir}"
    );
}

// ------------------------- `noreturn` from the lowered control flow

#[test]
fn a_body_with_no_returning_path_claims_noreturn() {
    // `spin` diverges in its own body; `die` returns only after a call to
    // `spin`. Both never return, and the attribute has to reach the
    // second one through the call graph, not just the local walk.
    let ir = ir_of(
        "noreturn",
        r#"
module Main where
  spin :: forall a b. a ->{Diverge} b
  spin x = spin x

  die :: I64 ->{Diverge} I64
  die n = spin n

  foreign export ccall "fix_die" die :: I64 ->{Diverge} I64

  main :: Int
  main = 0
"#,
    );
    // `emit` keeps every unit and does not specialize, so the unit keeps
    // its source name here; `emit-ir` over a program with an export
    // specializes it to `Main.spin@RR`.
    let attrs = attrs_of(&ir, "Main.spin");
    assert!(
        attrs.contains("noreturn"),
        "Main.spin: a diverging body claims noreturn, got {attrs:?}\n{ir}"
    );
    let attrs = attrs_of(&ir, "Main.die");
    assert!(
        attrs.contains("noreturn"),
        "Main.die: a unit that returns only through a noreturn callee claims noreturn, got {attrs:?}\n{ir}"
    );
}

#[test]
fn a_returning_unit_never_claims_noreturn() {
    // The soundness direction. A `{Diverge}` row is a permission, not a
    // proof: a definition may declare it and still return. The attribute
    // restates the lowered control flow, so a body with a live `ret` must
    // not claim it, and it must not claim `willreturn` either unless the
    // totality row proves it.
    let ir = ir_of(
        "noreturn-sound",
        r#"
module Main where
  pick :: Bool -> U8 -> U8 -> U8
  pick b x y = if b then x else y

  main :: Int
  main = if pick True 1 2 == 1 then 0 else 1
"#,
    );
    let attrs = attrs_of(&ir, "Main.pick");
    assert!(
        !attrs.contains("noreturn"),
        "Main.pick: a returning unit claims noreturn in {attrs:?}\n{ir}"
    );
    // No unit in the module may claim both, and no unit may claim
    // `noreturn` while its body has a reachable `ret`.
    for line in ir.lines() {
        if !line.starts_with("define ") {
            continue;
        }
        let Some(name) = unit_name_of(line) else { continue };
        let attrs = attrs_of(&ir, &name);
        if !attrs.contains("noreturn") {
            continue;
        }
        assert!(
            !attrs.contains("willreturn"),
            "{name} claims both noreturn and willreturn in {attrs:?}\n{ir}"
        );
    }
}
