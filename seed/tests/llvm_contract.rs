//! The LLVM contract surface (P27): Cinch's analyses spelled in
//! LLVM's own vocabulary (design/llvm-contract-attributes.md). Every
//! test observes the emitted IR: the contract appears where the
//! analysis proves the fact and is absent where it does not (the plan
//! note's evidence style: never wiring tests).

mod common;
use cinch::driver::check_sources;
use common::source_frontend::run_source_objects;
use std::collections::HashMap;

/// The emitted LLVM IR of one checked source (every IR test carries a
/// `main`, which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The emitted LLVM IR of one checked source for another target: the
/// host's profile with its triple, architecture and operating system
/// replaced (a C convention is a function of these alone).
fn ir_for(name: &str, src: &str, arch: &str, vendor: &str, os: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    let profile = cinch::core::TargetProfile {
        triple: format!("{arch}-{vendor}-{os}"),
        arch: arch.to_string(),
        os: os.to_string(),
        ..cinch::core::TargetProfile::native()
    };
    cinch::codegen::emit_for(&profile, &program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
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

/// The attribute group number a `define` line references (`#N`), if
/// it carries one.
fn group_ref(define: &str) -> Option<&str> {
    let (_, tail) = define.split_once(" #")?;
    Some(
        tail.split_whitespace()
            .next()
            .expect("an attribute group number"),
    )
}

/// The text of one unit's function attribute group (empty when the
/// unit carries no group).
fn attrs_of(ir: &str, unit: &str) -> String {
    let define = define_of(ir, unit);
    let Some(n) = group_ref(&define) else {
        return String::new();
    };
    let head = format!("attributes #{n} = {{ ");
    ir.lines()
        .find_map(|l| l.strip_prefix(&head).and_then(|r| r.strip_suffix(" }")))
        .unwrap_or_else(|| panic!("{unit}: attribute group #{n} missing in {ir}"))
        .to_string()
}

/// The unit name a `define` line defines (`@"a b"(` or `@ab(`).
fn unit_name_of(line: &str) -> Option<&str> {
    let (_, after) = line.split_once('@')?;
    if let Some(rest) = after.strip_prefix('"') {
        let end = rest.find('"')?;
        return Some(&rest[..end]);
    }
    let end = after.find('(')?;
    Some(&after[..end])
}

/// One emitted `define`: its unit name, its body lines (between `{` and
/// `}`) and its attribute group text.
struct Define<'a> {
    name: &'a str,
    body: Vec<&'a str>,
    attrs: &'a str,
}

/// Every emitted `define` of a module, read in one pass over the IR.
///
/// A whole-module check that resolves each define by name rescans the
/// module once per define, and the module carries the whole prelude, so
/// that check is quadratic in the IR. Here the attribute groups are
/// indexed once and each body is collected as the walk passes it, which
/// keeps the check linear. A define whose body never closes (truncated
/// IR) is left out, as a by-name lookup would leave it out.
fn defines_of(ir: &str) -> Vec<Define<'_>> {
    let groups: HashMap<&str, &str> = ir
        .lines()
        .filter_map(|l| {
            let (n, text) = l.strip_prefix("attributes #")?.split_once(" = { ")?;
            Some((n, text.strip_suffix(" }")?))
        })
        .collect();
    let mut defines = Vec::new();
    let mut lines = ir.lines();
    while let Some(line) = lines.next() {
        if !line.starts_with("define ") {
            continue;
        }
        let mut body = Vec::new();
        let mut closed = false;
        for l in lines.by_ref() {
            if l == "}" {
                closed = true;
                break;
            }
            body.push(l);
        }
        let Some(name) = unit_name_of(line) else { continue };
        if !closed {
            continue;
        }
        let attrs = match group_ref(line) {
            None => "",
            Some(n) => groups
                .get(n)
                .copied()
                .unwrap_or_else(|| panic!("{name}: attribute group #{n} missing in {ir}")),
        };
        defines.push(Define { name, body, attrs });
    }
    defines
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
fn nocapture_marks_only_non_escaping_borrows() {
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
    // Spelled `nocapture`, which every supported LLVM reads and 21 onwards
    // upgrades to `captures(none)` (`driver::CLANG_MIN_MAJOR`).
    assert!(
        declare_of(&ir, "fix_borrow").contains("nocapture"),
        "a non-escaping borrow must carry nocapture\n{ir}"
    );
    assert!(
        !declare_of(&ir, "fix_take").contains("nocapture"),
        "an escaping raw pointer must not carry nocapture\n{ir}"
    );
    // The spelling every supported LLVM reads: `captures(...)` is new in
    // LLVM 20, and LLVM 21 onward reads `nocapture` as `captures(none)`.
    assert!(
        !ir.contains("captures("),
        "the emitted IR must not use the LLVM 20+ `captures(...)` spelling\n{ir}"
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
        body_of(&ir, "Main.nullPtr").contains("freeze ptr"),
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
    let src = r#"
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
"#;
    // A 24-byte struct is past every register rule. System V passes it
    // as a stack copy (`byval`, with its layout); AAPCS64 passes a
    // pointer to the caller's copy, a plain `ptr`. Both return it through
    // the caller's result buffer (`sret`, returning void).
    let sysv = ir_for("byval-sysv", src, "x86_64", "unknown", "linux");
    let byval = declare_of(&sysv, "sum_big");
    assert!(
        byval.contains("ptr byval(%__sl_layout_0) align 8"),
        "System V passes a 24-byte aggregate as a byval copy\n{byval}\n{sysv}"
    );
    let aapcs = ir_for("byval-aapcs", src, "aarch64", "apple", "darwin");
    let reference = declare_of(&aapcs, "sum_big");
    assert!(
        reference.contains("(ptr nonnull align 8)"),
        "AAPCS64 passes a 24-byte aggregate by reference, as a plain pointer\n{reference}\n{aapcs}"
    );
    for ir in [&sysv, &aapcs] {
        assert!(!ir.contains("byref("), "no convention here passes by `byref`\n{ir}");
        let sret = declare_of(ir, "mk_big");
        assert!(
            sret.starts_with("declare void ") && sret.contains("ptr sret(%__sl_layout_0) align 8, i64"),
            "a 24-byte aggregate result is an sret buffer, then the scalar parameter\n{sret}\n{ir}"
        );
    }
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
fn small_aggregates_cross_in_the_registers_c_uses() {
    // The register rules, against real C on the host: a 16-byte integer
    // pair (System V INTEGER+INTEGER, AAPCS64 `[2 x i64]`), a double and
    // an integer (SSE+INTEGER), three floats in 12 bytes (`<2 x float>`
    // then `float`; an AAPCS64 HFA), three doubles (System V MEMORY; an
    // AAPCS64 HFA in registers), a pair arriving when only one integer
    // register is left (System V moves it to the stack whole, and the
    // scalar after it still takes the register), and the export side,
    // where C passes and receives a pair by value.
    use std::io::Write;
    let dir = std::env::temp_dir().join(format!("slp27-small-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join("small.c");
    let o_path = dir.join("small.o");
    let mut f = std::fs::File::create(&c_path).expect("write fixture");
    f.write_all(
        br#"struct Pair { long a; long b; };
struct Mix { double x; long n; };
struct F3 { float a; float b; float c; };
struct D3 { double a; double b; double c; };
struct Pair mk_pair(long a) { struct Pair p = { a, a + 1 }; return p; }
long long sum_pair(struct Pair p) { return p.a + p.b; }
struct Mix mk_mix(long n) { struct Mix m = { n + 0.5, n }; return m; }
long long read_mix(struct Mix m) { return (long long)(m.x * 2) * 10 + m.n; }
struct F3 mk_f3(long n) { struct F3 f = { n, n + 0.25f, n + 0.5f }; return f; }
long long read_f3(struct F3 f) { return (long long)((f.a + f.b + f.c) * 4); }
struct D3 mk_d3(long n) { struct D3 d = { n, n + 0.25, n + 0.5 }; return d; }
long long read_d3(struct D3 d) { return (long long)((d.a + d.b + d.c) * 4); }
long long late_pair(long a, long b, long c, long d, long e, struct Pair p, long f) {
  return a + b + c + d + e + p.a * 100 + p.b * 1000 + f * 10000;
}
extern long long cinch_sum_pair(struct Pair p);
extern struct Pair cinch_swap(struct Pair p);
long long call_back(long a) {
  struct Pair p = { a, a * 2 };
  struct Pair q = cinch_swap(p);
  return cinch_sum_pair(p) * 100 + q.a * 10 + q.b;
}
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
  record Pair where
    a :: I64
    b :: I64

  #[repr(C)]
  record Mix where
    x :: F64
    n :: I64

  #[repr(C)]
  record F3 where
    a :: F32
    b :: F32
    c :: F32

  #[repr(C)]
  record D3 where
    a :: F64
    b :: F64
    c :: F64

  #[sret]
  foreign import ccall "mk_pair" mkPair :: I64 ->{Foreign Fix, Diverge} Pair
  #[byval]
  foreign import ccall "sum_pair" sumPair :: Pair ->{Foreign Fix, Diverge} I64
  #[sret]
  foreign import ccall "mk_mix" mkMix :: I64 ->{Foreign Fix, Diverge} Mix
  #[byval]
  foreign import ccall "read_mix" readMix :: Mix ->{Foreign Fix, Diverge} I64
  #[sret]
  foreign import ccall "mk_f3" mkF3 :: I64 ->{Foreign Fix, Diverge} F3
  #[byval]
  foreign import ccall "read_f3" readF3 :: F3 ->{Foreign Fix, Diverge} I64
  #[sret]
  foreign import ccall "mk_d3" mkD3 :: I64 ->{Foreign Fix, Diverge} D3
  #[byval]
  foreign import ccall "read_d3" readD3 :: D3 ->{Foreign Fix, Diverge} I64
  #[byval]
  foreign import ccall "late_pair" latePair :: I64 -> I64 -> I64 -> I64 -> I64 -> Pair -> I64 ->{Foreign Fix, Diverge} I64
  foreign import ccall "call_back" callBack :: I64 ->{Foreign Fix, Diverge} I64

  cinchSumPair :: Pair -> I64
  cinchSumPair p = wrapping-add-i64 p.a p.b

  #[byval]
  foreign export ccall "cinch_sum_pair" cinchSumPair :: Pair -> I64

  cinchSwap :: Pair -> Pair
  cinchSwap p = Pair { a = p.b, b = p.a }

  #[byval]
  #[sret]
  foreign export ccall "cinch_swap" cinchSwap :: Pair -> Pair

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ =
    if sumPair (mkPair 5) == 11 then
      (if readMix (mkMix 3) == 73 then
        (if readF3 (mkF3 1) == 15 then
          (if readD3 (mkD3 1) == 15 then
            (if latePair 1 2 3 4 5 (mkPair 7) 9 == 98715 then
              (if callBack 3 == 963 then 0 else 6)
            else 5)
          else 4)
        else 3)
      else 2)
    else 1
"#;
    // The expected values: 5+6; (3.5*2)*10+3; (1+1.25+1.5)*4 twice;
    // 15 + 7*100 + 8*1000 + 9*10000; (3+6)*100 + 6*10 + 3.
    let o = o_path.to_string_lossy().into_owned();
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "small by-value aggregates: expected exit 0, got {code}"),
        Err(e) => panic!("small by-value aggregates: compilation failed: {e}"),
    }
}

#[test]
fn small_aggregates_spell_each_conventions_registers() {
    // The same declarations per target, without running them: the
    // spelling is a function of the convention, and both are checked on
    // any host.
    let src = r#"
module Main where
  #[repr(C)]
  record Pair where
    a :: I64
    b :: I64

  #[repr(C)]
  record F3 where
    a :: F32
    b :: F32
    c :: F32

  #[repr(C)]
  record D3 where
    a :: F64
    b :: F64
    c :: F64

  #[byval]
  foreign import ccall "sum_pair" sumPair :: Pair ->{Foreign Fix, Diverge} I64
  #[sret]
  foreign import ccall "mk_pair" mkPair :: I64 ->{Foreign Fix, Diverge} Pair
  #[byval]
  foreign import ccall "read_f3" readF3 :: F3 ->{Foreign Fix, Diverge} I64
  #[byval]
  foreign import ccall "read_d3" readD3 :: D3 ->{Foreign Fix, Diverge} I64
  #[sret]
  foreign import ccall "mk_d3" mkD3 :: I64 ->{Foreign Fix, Diverge} D3
  #[sret]
  foreign import ccall "mk_f3" mkF3 :: I64 ->{Foreign Fix, Diverge} F3

  main :: Unit ->{Foreign Fix, Diverge} Int
  main _ = if sumPair (mkPair 1) == readF3 (mkF3 1) then 0 else if readD3 (mkD3 1) == 0 then 1 else 2
"#;
    let sysv = ir_for("small-sysv", src, "x86_64", "unknown", "linux");
    let expect = |ir: &str, symbol: &str, spelled: &str| {
        let d = declare_of(ir, symbol);
        assert!(d.contains(spelled), "{symbol}: expected `{spelled}`\n{d}\n{ir}");
    };
    expect(&sysv, "sum_pair", "@\"sum_pair\"(i64, i64)");
    expect(&sysv, "mk_pair", "declare { i64, i64 } @\"mk_pair\"(i64");
    expect(&sysv, "read_f3", "@\"read_f3\"(<2 x float>, float)");
    expect(&sysv, "read_d3", "@\"read_d3\"(ptr byval(");
    expect(&sysv, "mk_d3", "declare void @\"mk_d3\"(ptr sret(");
    expect(&sysv, "mk_f3", "declare { <2 x float>, float } @\"mk_f3\"(i64");
    let aapcs = ir_for("small-aapcs", src, "aarch64", "apple", "darwin");
    expect(&aapcs, "sum_pair", "@\"sum_pair\"([2 x i64])");
    expect(&aapcs, "mk_pair", "declare [2 x i64] @\"mk_pair\"(i64");
    expect(&aapcs, "read_f3", "@\"read_f3\"([3 x float])");
    expect(&aapcs, "read_d3", "@\"read_d3\"([3 x double])");
    expect(&aapcs, "mk_d3", "declare [3 x double] @\"mk_d3\"(i64");
    expect(&aapcs, "mk_f3", "declare [3 x float] @\"mk_f3\"(i64");
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
    for Define { name, body, attrs } in defines_of(&ir) {
        let bare = format!("@{name}(");
        let quoted = format!("\"{name}\"(");
        let calls_self = body.iter().any(|l| l.contains(&bare) || l.contains(&quoted));
        if !calls_self {
            continue;
        }
        checked += 1;
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
    for Define { name, attrs, .. } in defines_of(&ir) {
        if !attrs.contains("noreturn") {
            continue;
        }
        assert!(
            !attrs.contains("willreturn"),
            "{name} claims both noreturn and willreturn in {attrs:?}\n{ir}"
        );
    }
}

#[test]
fn foreign_scalar_declarations_preserve_the_c_abi_parameter_types() {
    let ir = ir_of("foreign-scalars", r#"
module Main where
  foreign import ccall "native_mix" mix :: I64 -> F64 ->{Foreign Native, IO} I64
  main :: Unit ->{Foreign Native, IO} Int
  main _ = 0
"#);
    let declaration = declare_of(&ir, "native_mix");
    assert!(declaration.contains("(i64 noundef, double noundef)"), "{declaration}");
}
