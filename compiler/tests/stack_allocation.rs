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
///
/// `CINCH_NO_PRUNE=1` because every assertion in this file is about a named
/// unit's body: the specializer unfolds a small callee into its caller, and
/// the reachability sweep then removes the now uncalled unit
/// (design/stack-allocation.md section 7.4), which is the sweep's subject and
/// not this file's.
fn cinch(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cinch"))
        .current_dir(dir)
        .env("CINCH_NO_PRUNE", "1")
        .args(args)
        .output()
        .expect("run cinch")
}

fn emit_ir(dir: &Path, src: &str, flags: &[&str]) -> String {
    let entry = source(dir, src);
    let out = dir.join("prog.ll");
    // `--no-inline` keeps the callee a unit of its own: dead-unit
    // elimination removes a unit the inliner left with no caller
    // (design/stack-allocation.md section 7.4), and every assertion here is
    // about a named unit's body.
    let mut args: Vec<String> = vec!["emit-ir".into(), "--no-inline".into()];
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

/// A block stored in another block: admissible only when the holder is
/// itself frame-bound (`CINCH_STACK_WIDEN=store`), because only then does the
/// holder's own release drop the stored reference.
const NATIVE_STORE: &str = r#"
module Main where
  import Data.List

  data Box a where
    Box :: a -> Box a

  data Holder where
    Holder :: Box (List Int) -> Int -> Holder

  local :: Int
  local =
    let b = Box (Cons 1 Nil) in
    let h = Holder b 0 in
    case h of
      Holder x _ -> case x of
        Box xs -> length xs

  main :: Int
  main = local
"#;

#[test]
fn a_block_stored_in_a_frame_block_takes_a_slot() {
    let dir = dir("store");
    let entry = source(&dir, NATIVE_STORE);
    let output = cinch(
        &dir,
        &[
            "core",
            "emit",
            "--max-simplifier-iterations",
            "0",
            "--no-inline",
            "-o",
            "lowered.core",
            &entry.display().to_string(),
        ],
    );
    assert!(
        output.status.success(),
        "core emit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let dump = std::fs::read_to_string(dir.join("lowered.core")).expect("lowered.core");
    // Two slots: the box, and the holder that stores it (16 + 32 bytes).
    let start = dump.find("unit Main.local").expect("Main.local in the dump");
    let unit = &dump[start..start + dump[start..].find("\nend").expect("unit end")];
    assert!(unit.contains("@frame(2 slots, 48 bytes)"), "the nested store took no slot:\n{unit}");
    assert!(unit.contains("@frame(f0)") && unit.contains("@frame(f1)"), "{unit}");
    let stack = stack_of(&dir);
    assert!(slots(&stack) >= 2, "{stack}");
    // Released exactly once, at the holder's death: the payload's box is
    // released through its own frame block, whose list is the counted word.
    let exe = dir.join("prog");
    let output = cinch(&dir, &["build", "Main.cinch", "-o", exe.to_str().expect("path")]);
    assert!(output.status.success(), "build failed: {}", String::from_utf8_lossy(&output.stderr));
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
            "the nested frame block's payload leaked (debug {debug}): {stderr}"
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
    CAlt, CBranch, CPat, Core, FnDef, FnHints, FnMeta, GlobalId, InstanceDef, LocalId, Program,
    UnitRepr,
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
        lent_entries: Default::default(),
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

// ------------------------------------------------- the two widenings
//
// Rule 1 admits two transfers beyond a read or a death: a pass to a callee's
// owned parameter that the callee consumes, and a store into a block that is
// itself frame-bound. These are Core programs, for the same reason the
// duplicated rule's are: the shape a source program produces is decided by
// unfold and borrow inference before the pass sees it.

use cinch::repr::Class;
use cinch::stack::{Options, Widen};

/// `Main.first` takes a pair, keeps one field and drops the block: the
/// parameter is consumed, so the caller may hand it a frame block.
fn consuming_callee() -> FnDef {
    test_unit(
        "Main.first",
        vec![Class::Obj],
        Core::Release {
            block: LocalId(0),
            moved: vec![],
            rest: vec![],
            // No token: the `Release` finds the block unshared and keeps it
            // alive itself.
            token: None,
            body: Box::new(Core::Int(0)),
        },
    )
}

/// `let p = MkPair in <tail>`: one block, handed to `Main.first` by `tail`.
/// `<tail>` is the unit's result, so a bare [`call_first`] there is a call in
/// tail position; [`bind`] puts the same call in a `let` value, which it is
/// not.
fn consumed_argument(tail: Core) -> Program {
    let body = Core::Let(
        LocalId(0),
        Class::Obj,
        Box::new(Core::App(
            Box::new(Core::Ctor(test_gid("Main.Pair"), vec![])),
            vec![Core::Int(1), Core::Int(2)],
        )),
        Box::new(tail),
    );
    let mut p = test_program(vec![
        test_ctor("Main.Pair", 0, vec![Class::Raw, Class::Raw]),
        consuming_callee(),
        test_unit("Main.local", vec![], body),
    ]);
    // The borrow inference's answer for the callee: an owned parameter.
    p.borrowed.insert("Main.first".to_string(), vec![false]);
    p
}

/// The call the caller makes, as a value: `Main.first p`.
fn call_first() -> Core {
    Core::App(Box::new(Core::Global(test_gid("Main.first"), vec![])), vec![Core::Var(LocalId(0))])
}

/// `let r = <v> in r`: the same value, one `let` away from tail position.
fn bind(v: Core) -> Core {
    Core::Let(LocalId(7), Class::Raw, Box::new(v), Box::new(Core::Var(LocalId(7))))
}

fn local_allocated(p: &mut Program, widen: Widen) -> bool {
    let opts = Options { widen, ..Options::default() };
    cinch::stack::allocate(p, &cinch::core::TargetProfile::native(), &opts);
    p.frame.get("Main.local").is_some_and(|plan| plan.site(0).is_some())
}

#[test]
fn a_frame_block_passed_to_a_consuming_owned_parameter_takes_a_slot() {
    for widen in [Widen::All, Widen::Consume] {
        let mut p = consumed_argument(bind(call_first()));
        assert!(local_allocated(&mut p, widen), "{widen:?}: the consumed argument has no frame slot: {:?}", p.stack);
        // The callee needs the runtime's frame arms even though it builds
        // no frame block of its own.
        assert!(
            p.stack.receives_frame.contains("Main.first"),
            "{widen:?}: the callee does not carry the frame arms: {:?}",
            p.stack
        );
    }
}

/// The callee's frame replaces the caller's in a tail call: a `musttail`
/// call for a saturated call whose signature matches, a jump to the loop
/// header for a self tail call. The callee's own spills then land in the
/// caller's slot, so the argument it was handed describes a block the callee
/// has already overwritten. Nothing in the argument's position, ownership or
/// read-only-ness can make that safe, so every tail-call argument stays on
/// the heap.
#[test]
fn a_frame_block_passed_to_a_tail_call_stays_on_the_heap() {
    // A consumed owned parameter: the transfer the widening admits. Every
    // form that keeps tail position is the same call in the emitted code
    // (`CoreEmitter::emit_to` settles each into one destination), so each
    // one must be refused: the unit's body, a count operation's body, and
    // an `If` branch.
    let count_use = Core::Dup(LocalId(0), Class::Obj, Box::new(call_first()));
    let branch_use = Core::If(
        Box::new(Core::Bool(true)),
        CBranch::new(call_first()),
        CBranch::new(Core::Int(0)),
    );
    for tail in [call_first(), count_use, branch_use] {
        for widen in [Widen::All, Widen::Consume] {
            let mut p = consumed_argument(tail.clone());
            assert!(
                !local_allocated(&mut p, widen),
                "{widen:?}: a tail-call argument took a frame slot: {:?}",
                p.frame
            );
            assert_eq!(p.stack.refused.get("escapes"), Some(&1), "{widen:?}: {:?}", p.stack);
            assert!(p.stack.receives_frame.is_empty(), "{widen:?}: {:?}", p.stack);
        }
    }
    // A lent parameter: a read, which the rule admits in any other
    // position, and which the same frame reuse makes just as unsafe here.
    let mut p = consumed_argument(call_first());
    p.borrowed.insert("Main.first".to_string(), vec![true]);
    assert!(!local_allocated(&mut p, Widen::None), "a lent tail-call argument took a frame slot: {:?}", p.frame);
    assert_eq!(p.stack.refused.get("escapes"), Some(&1), "{:?}", p.stack);
}

#[test]
fn a_frame_block_passed_to_a_borrowed_or_unconsuming_parameter_stays_on_the_heap() {
    // The rule as it was before the widening: a pass to a callee's
    // parameter is an escape.
    let mut p = consumed_argument(bind(call_first()));
    assert!(!local_allocated(&mut p, Widen::None), "the disabled rule admitted the pass: {:?}", p.frame);
    assert_eq!(p.stack.refused.get("escapes"), Some(&1), "{:?}", p.stack);
    assert!(p.stack.receives_frame.is_empty(), "{:?}", p.stack);
    // A callee that does not consume its parameter (it returns it) leaves
    // the block reachable from its caller's caller, so the pass refuses
    // even with the widening on.
    let mut p = consumed_argument(bind(call_first()));
    for f in p.fns.iter_mut() {
        if f.id.0 == "Main.first" {
            f.body = Core::Var(LocalId(0));
        }
    }
    assert!(!local_allocated(&mut p, Widen::All), "an unconsumed parameter took a frame block: {:?}", p.frame);
    assert_eq!(p.stack.refused.get("escapes"), Some(&1), "{:?}", p.stack);
}

#[test]
fn a_frame_block_parameter_that_is_lent_is_not_a_consumption() {
    // A borrowed parameter is not the callee's to release: the caller's
    // own drop still is, so the transfer is admissible in a non-tail call
    // and the receiver needs no frame arms.
    let mut p = consumed_argument(Core::Drop(
        LocalId(0),
        Class::Obj,
        Box::new(bind(call_first())),
    ));
    p.borrowed.insert("Main.first".to_string(), vec![true]);
    assert!(local_allocated(&mut p, Widen::None), "{:?}", p.frame);
    assert!(p.stack.receives_frame.is_empty(), "{:?}", p.stack);
}
