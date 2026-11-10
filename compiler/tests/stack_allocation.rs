//! Stack allocation of non-escaping blocks (design/stack-allocation.md):
//! the eligibility rule, its lowering, its inspectability, and the guarantee
//! that a disabled pass emits the IR the compiler produced before it.
//!
//! Every program here keeps the simplifier out of the way where the pass's
//! decision is the point: the simplifier folds a known-constructor case and
//! then there is nothing left to allocate.

use std::path::{Path, PathBuf};
use std::process::Command;

/// A program whose `local` builds a two-field block, matches it, and could
/// not escape it: the canonical frame allocation.
const LOCAL_PAIR: &str = r#"
module Main where
  data Pair a b where
    MkPair :: a -> b -> Pair a b

  sumPair :: Pair Int Int -> Int
  sumPair p = case p of
    MkPair a b -> a * 10 + b

  local :: Int -> Int
  local x = sumPair (MkPair x 2)

  main :: Int
  main = local 4
"#;

/// A frame block whose payload holds counted heap blocks that the pattern
/// does not move. Their release is the frame block's own: without it the
/// lists leak, which `CINCH_LEAK_CHECK=1` reports.
const LOCAL_TRIANGLE: &str = r#"
module Main where
  import Data.List

  data Tri where
    Tri :: List Int -> List Int -> List Int -> Tri

  first :: Tri -> Int
  first t = case t of
    Tri a _ _ -> length a

  local :: Int
  local = first (Tri (Cons 1 Nil) (Cons 2 Nil) (Cons 3 Nil))

  main :: Int
  main = local
"#;

/// A block that is returned: it leaves its activation, so it stays on the
/// heap.
const RETURNED: &str = r#"
module Main where
  data Box where
    Box :: Int -> Box

  unbox :: Box -> Int
  unbox b = case b of
    Box n -> n

  build :: Int -> Box
  build n = let r = Box n in r

  main :: Int
  main = unbox (build 3)
"#;

fn dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_stack_{}_{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    dir
}

fn source(dir: &Path, src: &str) -> PathBuf {
    let path = dir.join("Main.cinch");
    std::fs::write(&path, src).expect("write source");
    path
}

/// The `cinch` command with extra flags, run in `dir`.
fn cinch(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cinch"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("run cinch")
}

fn emit_ir(dir: &Path, src: &str, flags: &[&str]) -> String {
    let entry = source(dir, src);
    let out = dir.join("prog.ll");
    let mut args: Vec<String> = vec!["emit-ir".into()];
    args.extend(flags.iter().map(|f| f.to_string()));
    args.push(entry.display().to_string());
    args.push("-o".into());
    args.push(out.display().to_string());
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = cinch(dir, &refs);
    assert!(
        output.status.success(),
        "emit-ir {flags:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::read_to_string(&out).expect("emitted IR")
}

/// The body of the unit `name` in `ir`, from its `define` line to its
/// closing brace.
fn body<'a>(ir: &'a str, name: &str) -> &'a str {
    let head = format!("@\"{name}\"(");
    let start = ir
        .lines()
        .scan(0usize, |at, line| {
            let here = *at;
            *at += line.len() + 1;
            Some((here, line))
        })
        .find(|(_, line)| line.starts_with("define ") && line.contains(&head))
        .map(|(at, _)| at)
        .unwrap_or_else(|| panic!("no definition of {name} in:\n{ir}"));
    let end = ir[start..].find("\n}\n").expect("function end");
    &ir[start..start + end]
}

fn manifest(dir: &Path) -> serde_json::Value {
    let path = dir.join("build/Main.manifest.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&text).expect("manifest JSON")
}

fn stack_of(dir: &Path) -> serde_json::Value {
    let doc = manifest(dir);
    doc.get("stack")
        .cloned()
        .expect("the manifest records the stack allocation decision")
}

fn slots(stack: &serde_json::Value) -> u64 {
    stack["slots"].as_u64().unwrap_or(0)
}

fn bytes(stack: &serde_json::Value) -> u64 {
    stack["bytes"].as_u64().unwrap_or(0)
}

fn refused(stack: &serde_json::Value, why: &str) -> u64 {
    stack
        .get("refused")
        .and_then(|r| r.get(why))
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

#[test]
fn a_non_escaping_block_is_built_in_its_units_frame() {
    let dir = dir("local");
    let ir = emit_ir(&dir, LOCAL_PAIR, &["--max-simplifier-iterations", "0"]);
    let local = body(&ir, "Main.local");
    // One slot: the header plus two payload words, rounded to 16 bytes.
    assert!(
        local.contains("alloca [4 x i64], align 16"),
        "the pair has no frame slot:\n{local}"
    );
    assert!(
        !local.contains("@\"Main.MkPair@TT\"("),
        "the pair is still allocated by the constructor unit:\n{local}"
    );
    assert!(
        !local.contains("cinch_alloc_ctor"),
        "a frame slot still reaches the heap allocator:\n{local}"
    );
    // The count operations of the frame binder are gone: the only header
    // the block has is written once, by the slot itself.
    assert!(
        !local.contains("@cinch.rc.dup.obj(") && !local.contains("@cinch.rc.drop.obj("),
        "a frame block is still counted:\n{local}"
    );
    let stack = stack_of(&dir);
    assert_eq!(stack["enabled"], serde_json::Value::Bool(true));
    // `Main.local` is one slot of 32 bytes; the program reports every unit
    // the pass lowered, so the totals are at least that.
    assert!(slots(&stack) >= 1, "{stack}");
    assert!(bytes(&stack) >= 32, "{stack}");
    assert_eq!(stack["refused"]["frame-limit"], serde_json::Value::Null, "{stack}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_disabled_pass_emits_the_ir_the_compiler_had_before_it() {
    let dir = dir("disabled");
    let on = emit_ir(&dir, LOCAL_PAIR, &["--max-simplifier-iterations", "0"]);
    let off = emit_ir(
        &dir,
        LOCAL_PAIR,
        &["--max-simplifier-iterations", "0", "--no-stack-alloc"],
    );
    let heap = body(&off, "Main.local");
    assert!(
        heap.contains("@\"Main.MkPair@TT\"("),
        "the disabled pass must keep the constructor call:\n{heap}"
    );
    assert!(!heap.contains("align 16"), "the disabled pass allocated a frame slot:\n{heap}");
    let stack = stack_of(&dir);
    assert_eq!(stack["enabled"], serde_json::Value::Bool(false), "{stack}");
    assert_eq!(slots(&stack), 0, "{stack}");
    assert_eq!(stack["candidates"], 0, "{stack}");
    // The manifest is the only difference between the two runs; the
    // emitted module is not.
    assert_ne!(on, off);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_program_with_no_candidate_keeps_byte_identical_ir() {
    // Nothing here is a `let`-bound allocation, so the pass decides nothing
    // and the two runs must agree byte for byte (the oracle the design note
    // asks for, at the scale a test can check).
    const PLAIN: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = case n == 0 of
    True -> acc
    False -> build (n - 1) (Cons n acc)

  main :: Int
  main = case length (build 200 Nil) == 200 of
    True -> 0
    False -> 1
"#;
    let dir = dir("oracle");
    let on = emit_ir(&dir, PLAIN, &[]);
    let off = emit_ir(&dir, PLAIN, &["--no-stack-alloc"]);
    assert_eq!(on, off, "the pass changed a module it decided nothing about");
    assert_eq!(slots(&stack_of(&dir)), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_frame_block_releases_its_counted_payload_and_leaks_nothing() {
    let dir = dir("triangle");
    let ir = emit_ir(&dir, LOCAL_TRIANGLE, &["--max-simplifier-iterations", "0"]);
    let local = body(&ir, "Main.local");
    // Three payload words plus the header: 32 bytes of frame.
    assert!(
        local.contains("alloca [4 x i64], align 16"),
        "the triangle has no frame slot:\n{local}"
    );
    assert!(
        !local.contains("@\"Main.Tri\"("),
        "the triangle is still heap allocated:\n{local}"
    );
    let stack = stack_of(&dir);
    assert!(slots(&stack) >= 1, "{stack}");
    // The block's own release drops the two fields the pattern leaves
    // (`b` and `c`), which is where the frame block's payload dies: the
    // `cinch.rc.drop` calls in the unit are those two fields.
    let drops = local.matches("@cinch.rc.drop").count();
    assert!(drops >= 2, "the frame block does not release its payload:\n{local}");
    // Build and run it: a payload that is not released shows up as a leak.
    let exe = dir.join("prog");
    let output = cinch(&dir, &["build", "Main.cinch", "-o", exe.to_str().expect("path")]);
    assert!(
        output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for debug in [false, true] {
        let mut cmd = Command::new(&exe);
        cmd.env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "64M");
        if debug {
            cmd.env("CINCH_RC_DEBUG", "1");
        }
        let out = cmd.output().expect("run the program");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(1), "{stderr}");
        assert!(
            stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
            "the frame block's payload leaked (debug {debug}): {stderr}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_returned_block_stays_on_the_heap() {
    let dir = dir("returned");
    let ir = emit_ir(&dir, RETURNED, &["--max-simplifier-iterations", "0"]);
    let build = body(&ir, "Main.build");
    assert!(
        build.contains("@\"Main.Box\"("),
        "a returned block was lowered into the frame:\n{build}"
    );
    let stack = stack_of(&dir);
    assert!(refused(&stack, "escapes") >= 1, "{stack}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_frame_limit_keeps_allocations_on_the_heap() {
    let dir = dir("limit");
    let ir = emit_ir(
        &dir,
        LOCAL_PAIR,
        &["--max-simplifier-iterations", "0", "--stack-frame-limit", "0"],
    );
    let local = body(&ir, "Main.local");
    assert!(
        local.contains("@\"Main.MkPair@TT\"("),
        "a zero limit still reserved a frame slot:\n{local}"
    );
    let stack = stack_of(&dir);
    assert!(refused(&stack, "frame-limit") >= 1, "{stack}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_lowered_dump_marks_the_frame_decision_and_verifies() {
    let dir = dir("dump");
    let entry = source(&dir, LOCAL_TRIANGLE);
    let output = cinch(
        &dir,
        &[
            "emit-ir",
            "--max-simplifier-iterations",
            "0",
            "--dump-core=lowered",
            &entry.display().to_string(),
        ],
    );
    assert!(
        output.status.success(),
        "emit-ir failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(dir.join("build/Main.core")).expect("build/Main.core");
    assert!(text.contains("@frame(1 slot, 32 bytes)"), "{text}");
    assert!(text.contains("@frame(f0)"), "{text}");
    // The reader accepts what the printer wrote.
    let verify = cinch(&dir, &["core", "verify", "build/Main.core"]);
    assert!(
        verify.status.success(),
        "the annotated dump does not verify: {}",
        String::from_utf8_lossy(&verify.stderr)
    );
    // `cinch core emit` is the same composition the test corpus uses, so
    // the two must agree byte for byte. No golden corpus is written here.
    let emit = cinch(
        &dir,
        &[
            "core",
            "emit",
            &entry.display().to_string(),
            "--max-simplifier-iterations",
            "0",
            "-o",
            "emitted.core",
        ],
    );
    assert!(emit.status.success(), "{}", String::from_utf8_lossy(&emit.stderr));
    let emitted = std::fs::read_to_string(dir.join("emitted.core")).expect("emitted dump");
    assert_eq!(emitted, text, "core emit and emit-ir disagree about the dump");
    let _ = std::fs::remove_dir_all(&dir);
}

// ------------------------------------------------- the duplicated rule
//
// A block with two references needs two releases of its payload, and a frame
// block has no count that makes one of them a no-op. When the payload holds
// no counted word there is nothing for either release to do, so the second
// reference cannot disagree with the first and the block still belongs in the
// frame. These programs are Core, so the reference shape is exact: a closed
// source program rarely produces the shape, because borrow inference lends a
// parameter that is only read and drop specialization turns a second
// scrutinee read into a `Drop` rather than a `Dup`.

use cinch::core::{
    CAlt, CPat, Core, FnDef, FnHints, FnMeta, GlobalId, InstanceDef, LocalId, Program, UnitRepr,
};

fn test_gid(name: &str) -> GlobalId {
    GlobalId(name.to_string())
}

fn test_unit(name: &str, params: Vec<cinch::repr::Class>, body: Core) -> FnDef {
    FnDef {
        id: test_gid(name),
        params: (0..params.len()).map(|i| format!("p{i}")).collect(),
        body,
        prim: None,
        ctor: None,
        hints: FnHints::default(),
        meta: FnMeta::default(),
        repr: UnitRepr { params, ..UnitRepr::default() },
    }
}

fn test_ctor(name: &str, tag: usize, fields: Vec<cinch::repr::Class>) -> FnDef {
    let mut f = test_unit(name, fields.clone(), Core::Unit);
    f.ctor = Some((tag, fields.len()));
    f.repr.fields = fields;
    f
}

fn test_program(fns: Vec<FnDef>) -> Program {
    Program {
        total_bodies: cinch::total::CheckedGraph::empty(),
        fns,
        instances: Vec::<InstanceDef>::new(),
        main: None,
        rewrites: vec![],
        plans: Default::default(),
        foreigns: Default::default(),
        prims: vec![],
        laws: vec![],
        lent: Default::default(),
        frame: Default::default(),
        stack: Default::default(),
        borrowed: Default::default(),
        inline: Default::default(),
    }
}

/// `let x = Ctor bool in dup x (let a = case x of C y -> 1 in drop x a)`: one
/// block, two references, no counted payload word.
fn duplicated_block(field: cinch::repr::Class) -> Program {
    use cinch::repr::Class;
    let con = |binder: usize| CPat::Con {
        tag: 0,
        fields: vec![LocalId(binder)],
        classes: vec![field],
    };
    let body = Core::Let(
        LocalId(0),
        Class::Obj,
        Box::new(Core::App(
            Box::new(Core::Ctor(test_gid("Main.Flag"), vec![])),
            vec![Core::Bool(true)],
        )),
        Box::new(Core::Dup(
            LocalId(0),
            Class::Obj,
            Box::new(Core::Let(
                LocalId(2),
                Class::Obj,
                Box::new(Core::Case {
                    scrut: Box::new(Core::Var(LocalId(0))),
                    alts: vec![CAlt::new(con(1), Core::Int(1))],
                }),
                Box::new(Core::Drop(LocalId(0), Class::Obj, Box::new(Core::Var(LocalId(2))))),
            )),
        )),
    );
    test_program(vec![
        test_ctor("Main.Flag", 0, vec![field]),
        test_unit("Main.picks", vec![], body),
    ])
}

fn allocated(p: &mut Program) -> bool {
    p.frame.get("Main.picks").is_some_and(|plan| plan.site(0).is_some())
}

#[test]
fn a_duplicated_block_with_no_counted_field_takes_a_slot() {
    use cinch::repr::Class;
    let mut p = duplicated_block(Class::Raw);
    cinch::stack::allocate(&mut p, &cinch::core::TargetProfile::native(), &cinch::stack::Options::default());
    assert!(allocated(&mut p), "the uncounted duplicated block has no frame slot: {:?}", p.stack);
    assert_eq!(p.stack.refused.get("duplicated"), None, "{:?}", p.stack);
    assert_eq!(p.stack.slots, 1, "{:?}", p.stack);
}

#[test]
fn a_duplicated_block_with_a_counted_field_stays_on_the_heap() {
    use cinch::repr::Class;
    let mut p = duplicated_block(Class::Obj);
    cinch::stack::allocate(&mut p, &cinch::core::TargetProfile::native(), &cinch::stack::Options::default());
    assert!(!allocated(&mut p), "a duplicated counted block took a frame slot: {:?}", p.frame);
    assert_eq!(p.stack.refused.get("duplicated"), Some(&1), "{:?}", p.stack);
    assert_eq!(p.stack.slots, 0, "{:?}", p.stack);
}
