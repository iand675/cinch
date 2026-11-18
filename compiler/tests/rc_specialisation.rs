//! Count-operation savings of design/reclamation.md section 4 ("As
//! built"): borrowed parameters of a closed program, drop specialisation
//! (`Core::Release`) and static dictionaries. Each program is checked for
//! where the operations went in its IR, and runs leak-free both plainly
//! and from a `rc_debug` build under `CINCH_RC_DEBUG=1`.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, driver_with_profile_opts, DriverOptions, Mode};
use std::path::{Path, PathBuf};
use std::process::Command;

const PROGRAM: &str = r#"
module Main where
  import Data.List

  signature Size a where
    size :: a -> Int

  implicit instance SizeInt :: Size Int where
    size x = 1

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons h t -> size h + count t

  lastOr :: Int -> List Int -> Int
  lastOr d xs = case xs of
    Nil -> d
    Cons h t -> lastOr h t

  bump :: List Int -> List Int
  bump xs = case xs of
    Nil -> xs
    Cons h t -> Cons (h + 1) (bump t)

  main :: Int
  main =
    let xs = build 200 Nil in
    let ys = bump xs in
    let zs = bump (build 50 Nil) in
    if count xs == 200 then (if count ys == 200 then (if count zs == 50 then (if lastOr 0 ys == 201 then 0 else 4) else 3) else 2) else 1
"#;

fn project(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_rc_spec_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    dir
}

fn drive(mode: Mode, dir: &Path, out: &Path, debug: bool) {
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    let (src, out) = (dir.join("Main.cinch"), out.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver_with_profile(mode, StageMode::Refresh, &src, Some(&out), &profile))
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("driver failed: {e}"));
}

/// The body of the function `name` in `ir`.
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
        .unwrap_or_else(|| panic!("no definition of {name}"));
    let end = ir[start..].find("\n}\n").expect("function end");
    &ir[start..start + end]
}

#[test]
fn borrowed_parameters_release_and_static_dictionaries_place_fewer_operations() {
    let dir = project("ir");
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // The instance has only methods: one immortal static block. The
    // simplifier's `direct-dict-call` rule turns the projection from that
    // block into a direct method call, so nothing reads the block and no
    // The instance has only methods and the method's body is a literal, so
    // the unfolding inliner copies it to the site and both the dictionary
    // block and the method unit are swept: neither is named anywhere. This
    // was once a surviving `Main.SizeInt.dict` block with a direct method
    // call; the copy is the last step, and the block is now dead.
    assert!(
        !ir.contains("Main.SizeInt"),
        "the static dictionary or its method survived:\n{ir}"
    );
    let count = body(&ir, "Main.count");
    assert!(!count.contains("@__apply("), "count still applies a closure:\n{count}");
    assert!(
        count.contains("call i64 @\"cinch.runtime.intAdd\""),
        "the sum is not the runtime addition:\n{count}"
    );
    // `lastOr` only reads its list, so the parameter is borrowed and no
    // count operation names it. The assertion is on the parameter's own
    // slot, not on every cell in the body: the loop re-reads interior cells
    // after the recursion returns, and those are not the parameter.
    let last = body(&ir, "Main.lastOr");
    for param in ["%p0", "%p1"] {
        for op in ["cinch.rc.dup.obj", "cinch.rc.dup.tobj", "cinch.rc.drop.obj", "cinch.rc.drop.tobj"] {
            let named = format!("{op}(i64 {param})");
            assert!(!last.contains(&named), "lastOr counts its parameter {param}:\n{last}");
        }
    }
    // `bump` returns its list on one path, so it owns the cell and releases
    // it. The release is the counted drop: the last-reference test and the
    // free live inside `cinch.rc.drop.obj` (whose tail switches on the
    // count and calls `cinch_drop_slow`), so the body names the drop rather
    // than a bare `cinch_free` it used to.
    let bump = body(&ir, "Main.bump");
    assert!(
        bump.contains("call void @cinch.rc.drop.obj(i64"),
        "bump does not release the cell it owns:\n{bump}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn borrowed_parameters_release_and_static_dictionaries_free_everything() {
    let dir = project("run");
    for debug in [false, true] {
        let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
        drive(Mode::Build, &dir, &exe, debug);
        let mut cmd = Command::new(&exe);
        cmd.env("CINCH_HEAP_LIMIT", "64M").env("CINCH_LEAK_CHECK", "1").env_remove("CINCH_ALLOC_STATS");
        if debug {
            cmd.env("CINCH_RC_DEBUG", "1");
        }
        let out = cmd.output().expect("spawn program");
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")), "debug {debug}: {err}");
        assert_eq!(out.status.code(), Some(0), "debug {debug}: {err}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The pattern-match-rebuild shape of a recursive accumulator, which is
/// what `reverse-into` is: the arm's scrutinee dies and the fresh `Cons`
/// is an *argument* of the recursive call, not a `let` on the spine.
///
/// `insert_reuse` searches a `Release` body's spine, and a call argument
/// is on that spine: it is evaluated after the release bound the token and
/// before the transfer, with no branch between them. Before this shape was
/// searched, the inliner's unrolled copies reused their cells and the
/// residual iteration, the one that carries the whole remaining tail,
/// allocated one cell per element. A 1,000-element build-then-reverse cost
/// 1,125 constructor blocks; with the argument searched it costs 1,000,
/// which is the build alone. The spine is allocated exactly once.
#[test]
fn accumulator_rebuild_reuses_the_cell_in_a_call_argument() {
    const PROGRAM: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  main :: Int
  main =
    let xs = build 1000 Nil in
    let ys = reverse xs in
    if length ys == 1000 then 0 else 1
"#;
    let dir = std::env::temp_dir().join(format!("cinch_reuse_arg_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    let exe = dir.join("prog");
    drive(Mode::Build, &dir, &exe, false);
    let out = Command::new(&exe)
        .env("CINCH_HEAP_LIMIT", "64M")
        .env("CINCH_ALLOC_KINDS", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .expect("spawn program");
    let err = String::from_utf8_lossy(&out.stderr);
    // `0` is the answer check: the reversed list has all 1,000 elements.
    assert_eq!(out.status.code(), Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the reverse leaked: {err}"
    );
    // The build allocates one constructor per element and the reverse
    // reuses every one of them, so the count is the build's 1,000. The
    // reverse's own output is the same cells; a single extra cell means a
    // path in the walk fell back to allocating.
    let ctor = err
        .lines()
        .find(|l| l.starts_with("cinch-alloc-kind:") && l.contains("kind=ctor"))
        .unwrap_or_else(|| panic!("no constructor count in {err}"));
    let blocks: u64 = ctor
        .split_whitespace()
        .find_map(|w| w.strip_prefix("blocks="))
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("no block count in {ctor}"));
    assert_eq!(blocks, 1000, "the reverse allocated again: {ctor}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A list whose tail another holder keeps: the reuse must copy the shared
/// nodes and leave the other holder's list intact. The argument being on
/// the spine says the cell is reachable, not that it is unique; the
/// runtime's `cinch.rc.unique` decides per node, and the cold path
/// allocates.
#[test]
fn accumulator_rebuild_copies_a_shared_tail() {
    const PROGRAM: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  pair :: Int -> (Int, Int)
  pair n =
    let xs = build n Nil in
    let ys = Cons 0 xs in
    let rs = reverse xs in
    (length rs, length ys)

  main :: Int
  main =
    let (a, b) = pair 1000 in
    if a == 1000 then (if b == 1001 then 0 else 2) else 1
"#;
    let dir = std::env::temp_dir().join(format!("cinch_reuse_shared_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    let exe = dir.join("prog");
    drive(Mode::Build, &dir, &exe, false);
    let out = Command::new(&exe)
        .env("CINCH_HEAP_LIMIT", "64M")
        .env("CINCH_LEAK_CHECK", "1")
        .env_remove("CINCH_ALLOC_KINDS")
        .output()
        .expect("spawn program");
    let err = String::from_utf8_lossy(&out.stderr);
    // `0` means both lists are the right length: the reverse copied the
    // shared spine and did not corrupt the holder's view of it.
    assert_eq!(out.status.code(), Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the shared case leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A `case` whose scrutinee dies and whose every alternative rebuilds it:
/// one release at the head of the branch, which binds the token that both
/// alternatives' reuse reads.
///
/// The first version of this optimisation copied the *release* into every
/// arm (Lean's `expandResetReuse` pushes the whole pair). That is wrong
/// here, and the shape below is what makes it visible: the branch head (an
/// `if`'s condition, a `case`'s scrutinee) runs after the release and
/// before the arms, so a field the head consumes is already dead at arm
/// entry, and a release copied into the arm drops it a second time. The
/// head keeps the release; only the reuse moves into the arms.
#[test]
fn a_branch_keeps_its_release_at_the_head_and_reuses_in_the_arms() {
    const PROGRAM: &str = r#"
module Main where
  import Data.List

  len :: List Int -> Int
  len xs = case xs of
    Nil -> 0
    Cons h t -> 1 + len t

  step :: Option (List Int) -> Option (List Int)
  step w = case w of
    None -> None
    Some x -> if len x == 1 then Some (Cons 9 x) else Some (Cons 8 x)

  main :: Int
  main = case step (Some (Cons 1 Nil)) of
    Some v -> if len v == 2 then 0 else 1
    None -> 2
"#;
    let dir = std::env::temp_dir().join(format!("cinch_reuse_branch_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    let core = owned_core(&dir);
    let step = core_unit(&core, "Main.step");
    // Exactly one release of the dying block, at the head of the branch,
    // and both alternatives read the one token it bound. Two releases (one
    // per arm, each with its own token) is the copied-pair bug: the head
    // runs before every arm, so an arm's release re-counts what the head
    // already consumed.
    assert_eq!(
        step.matches("(release t0 @token(").count(),
        1,
        "the branch does not keep exactly one tokenised release at its head:\n{step}"
    );
    let token = step
        .split("(release t0 @token(")
        .nth(1)
        .and_then(|rest| rest.split(')').next())
        .expect("a token name");
    assert_eq!(
        step.matches(&format!("(reuse {token}")).count(),
        2,
        "the two alternatives do not each read the token the head bound:\n{step}"
    );
    let (code, err) = build_and_run(&dir, "prog", true);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the branch reuse leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// The shape that crashed the bootstrap compiler: the branch's own head
/// consumes one of the dying block's counted fields, and every arm
/// rebuilds the block.
///
/// `branch_reuse` derives each arm's moved-field set from the arm's body.
/// The field the head read is dead by then, so it appeared in no arm's
/// moved set and in every arm's `rest`, and each arm's release dropped it
/// a second time. On the bootstrap sources that is a double free on every
/// call of `Bootstrap.Total.infer`, which is why stage 2 segfaulted inside
/// `allocate_block` with an empty log rather than reporting anything. The
/// head now keeps the release, so the field is dropped once, before the
/// condition runs.
#[test]
fn a_branch_whose_head_consumes_a_field_drops_it_once() {
    const PROGRAM: &str = r#"
module Main where
  import Data.List

  len :: List Int -> Int
  len xs = case xs of
    Nil -> 0
    Cons h t -> 1 + len t

  pick :: Option (List Int) -> Bool -> Option (List Int)
  pick w flag = case w of
    None -> None
    Some x -> if flag && len x == 1 then Some (Cons 7 x) else Some (Cons 8 x)

  main :: Int
  main = case pick (Some (Cons 1 Nil)) True of
    Some v -> if len v == 2 then 0 else 1
    None -> 2
"#;
    let dir = std::env::temp_dir().join(format!("cinch_reuse_head_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    let core = owned_core(&dir);
    let pick = core_unit(&core, "Main.pick");
    // The head consumed one of the dying block's fields, so the release
    // must be the branch's own, at its head, and the alternatives must not
    // carry copies. In the copied-pair form each arm carries its own
    // release *after* the condition and re-drops the field the condition
    // already dropped: the double free that crashed the bootstrap. The
    // head release is the one whose body is the branch (`(if` or `(case`),
    // and there must be exactly one release of the block in the unit.
    assert_eq!(
        pick.matches("(release t0 @token(").count(),
        1,
        "the branch does not keep exactly one tokenised release at its head:\n{pick}"
    );
    let after = pick
        .split_once("@token(")
        .and_then(|(_, rest)| rest.split_once(")"))
        .map(|(_, rest)| rest)
        .expect("the head release");
    // The head release hands the field over rather than dropping it, so it
    // appears in `moved`, not in `rest`: `rest` is the set the release
    // drops, and a field the head's own condition consumes must not be in
    // it (that is the second drop).
    assert!(
        after.trim_start().starts_with("(moved t2)"),
        "the head release does not hand the block's field to the reuse:\n{pick}"
    );
    let branch_at = after.find("(if").or_else(|| after.find("(case")).expect("the branch");
    let reuse_at = after.find("(reuse ").expect("the alternatives' reuse");
    assert!(
        branch_at < reuse_at,
        "the release is inside an alternative rather than at the head:\n{pick}"
    );
    // A debug build traps on a use after free rather than reading reused
    // memory, so this is the run that would have caught the original bug.
    let (code, err) = build_and_run(&dir, "prog", true);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the head-consuming branch leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// Build `dir`'s entry and run it under the leak check, returning the exit
/// code and stderr.
fn build_and_run(dir: &Path, name: &str, debug: bool) -> (Option<i32>, String) {
    let exe = dir.join(name);
    drive(Mode::Build, dir, &exe, debug);
    let mut cmd = Command::new(&exe);
    cmd.env("CINCH_HEAP_LIMIT", "64M").env("CINCH_LEAK_CHECK", "1").env_remove("CINCH_ALLOC_STATS");
    if debug {
        cmd.env("CINCH_RC_DEBUG", "1");
    }
    let out = cmd.output().expect("spawn program");
    (out.status.code(), String::from_utf8_lossy(&out.stderr).into_owned())
}

/// The count pass's all-owned convention, which isolates reset/reuse from
/// borrow inference. A driver's owned dump now uses its actual boundary;
/// it may lend this fixture's input instead, so it cannot pin the owned
/// release/reuse regression. Keep the complete checked constructor set for
/// the layout proof. The runtime assertions still build the real pipeline.
fn owned_core(dir: &Path) -> String {
    let source = std::fs::read_to_string(dir.join("Main.cinch")).expect("read source");
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let profile = TargetProfile { reclaim: true, rc_debug: false, ..TargetProfile::native() };
            let (program, ifaces) = cinch::driver::check_sources_profile_ifaces(&source, "Main.cinch", &profile)
                .expect("check reuse fixture");
            let owned = cinch::rc::insert(program, &profile).expect("count all-owned fixture");
            let modules = ["Main".to_string()].into_iter().collect();
            cinch::coretext::dump(&owned, cinch::coretext::Stage::Owned, &profile, &ifaces, &modules, &Default::default(), 0)
        })
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
}

/// The body of the unit `name` in a Core dump.
fn core_unit<'a>(text: &'a str, name: &str) -> &'a str {
    let head = format!("unit {name} ");
    let start = text
        .lines()
        .scan(0usize, |at, line| {
            let here = *at;
            *at += line.len() + 1;
            Some((here, line))
        })
        .find(|(_, line)| line.starts_with(&head))
        .map(|(at, _)| at)
        .unwrap_or_else(|| panic!("no unit {name}"));
    let end = text[start..].find("\nend\n").expect("unit end");
    &text[start..start + end]
}

/// A branch with a `Reuse` on one arm and none on the other: the arm that
/// rebuilds the block takes the kept cell, and the arm that does not must
/// still see it freed.
///
/// This is the shape that made every leak-checked compiler invocation exit
/// 74. `branch_reuse` keeps one tokenised `Release` at the head of the
/// branch and moves only the `Reuse` into the arms, and its doc comment
/// claimed an arm with no compatible allocation could leave the token
/// unread because the release had already freed the block. It had not: on
/// the unique path with a token bound `emit_release` stores the cell into
/// the token slot and frees nothing, so only a `Reuse` ever released it.
/// An arm that rebuilt nothing leaked one block per call.
///
/// The decision is pinned on the `Owned` Core dump, which is the only
/// artifact that states it: one tokenised release at the head, one `Reuse`
/// on one arm, and nothing on the other. The leak line is the behaviour
/// that follows from the code generator settling that token at the
/// release body's exits, and it is what the compiler's own nightly gate
/// reports, so it is asserted too.
#[test]
fn a_branch_with_one_reusing_arm_frees_the_cell_on_the_other() {
    const PROGRAM: &str = r#"
module Main where
  import Data.List

  step :: List Int -> List Int
  step xs = case xs of
    Nil -> Nil
    Cons h t -> if h > 0 then Cons h t else Nil

  spin :: Int -> Int -> Int
  spin n k = if n == 0 then 0 else spin (n - 1) (length (step (Cons k Nil)))

  main :: Int
  main = if spin 3 0 == 0 then 0 else 1
"#;
    let dir = std::env::temp_dir().join(format!("cinch_reuse_onesided_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    let core = owned_core(&dir);
    let step = core_unit(&core, "Main.step");
    // One release, at the head of the branch, binding the token the reuse
    // names.
    assert_eq!(
        step.matches("(release t0 @token(").count(),
        1,
        "the branch does not keep exactly one tokenised release at its head:\n{step}"
    );
    let token = step
        .split("(release t0 @token(")
        .nth(1)
        .and_then(|rest| rest.split(')').next())
        .expect("a token name");
    // The arm that rebuilds the block takes the cell; the arm that returns
    // `Nil` takes nothing. One reuse, not two, is the shape: a second one
    // would be the both-arms-rebuild case, which never leaked.
    assert_eq!(
        step.matches(&format!("(reuse {token}")).count(),
        1,
        "the branch does not reuse the cell on exactly one arm:\n{step}"
    );
    // The other arm allocates no compatible constructor: it drops both
    // moved fields instead of writing them into the kept cell. That is the
    // arm the token is left bound on, and so the arm that must free.
    let after_reuse = step.split("(reuse ").nth(1).expect("the reusing arm");
    assert!(
        after_reuse.contains("(drop t1") && after_reuse.contains("(drop t2"),
        "the arm that reuses nothing does not drop the moved fields:\n{step}"
    );
    // The cell the one reusing arm did not take is freed on the other
    // path, with and without frame allocation: this is the leak the
    // compiler's own gate reported.
    for no_stack in [false, true] {
        let exe = dir.join(if no_stack { "prog_heap" } else { "prog" });
        build_with(&dir, &exe, no_stack);
        let mut cmd = Command::new(&exe);
        cmd.env("CINCH_HEAP_LIMIT", "64M")
            .env("CINCH_LEAK_CHECK", "1")
            .env_remove("CINCH_ALLOC_STATS");
        let out = cmd.output().expect("spawn program");
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(0), "no_stack_alloc {no_stack}: {err}");
        assert!(
            err.lines()
                .any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
            "the one-sided reuse leaked (no_stack_alloc {no_stack}): {err}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// Build `dir`'s entry to `exe`, with frame allocation off when
/// `no_stack` is set.
fn build_with(dir: &Path, exe: &Path, no_stack: bool) {
    let profile = TargetProfile { reclaim: true, rc_debug: false, ..TargetProfile::native() };
    let opts = DriverOptions { no_stack_alloc: no_stack, ..DriverOptions::default() };
    let (src, out) = (dir.join("Main.cinch"), exe.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            driver_with_profile_opts(Mode::Build, StageMode::Refresh, &src, Some(&out), &profile, &opts)
        })
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("driver failed: {e}"));
}

/// One project per source, so each shape is measured on its own program.
fn source_project(tag: &str, source: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_rc_branch_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), source).expect("write source");
    dir
}

/// How many times a unit's body frees a block, which is what a reuse pair
/// removes from a path. A reuse emits no named symbol (it is inline stores
/// into the kept cell), so the observable is the `cinch_free` the unique
/// path would otherwise run: a branch whose every arm reuses frees nothing,
/// and an arm with no allocation to reuse into still frees.
fn frees(ir: &str, name: &str) -> usize {
    body(ir, name).matches("call void @cinch_free(i64").count()
}

const IF_SHAPED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> if y == 0 then Cons 1 (rebuild ys) else Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) == 1000 then 0 else 1
"#;

const CASE_SHAPED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> case y == 0 of
      True -> Cons 1 (rebuild ys)
      False -> Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) == 1000 then 0 else 1
"#;

/// One arm allocates, the other returns the tail unchanged. The pair must
/// be refused: the second arm has nothing to reuse the cell into, so a
/// per-arm pair there would strand the cell on that path.
const MIXED_ARMS: &str = r#"
module Main where
  import Data.List

  mixed :: List Int -> List Int
  mixed xs = case xs of
    Nil -> Nil
    Cons y ys -> case y == 0 of
      True -> Cons 1 (mixed ys)
      False -> ys

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (mixed (build 1000 Nil)) >= 0 then 0 else 1
"#;

/// A self-recursive rebuild whose allocation is inside a branch on every
/// path: the rule has to survive recursion, which is what the borrow-mode
/// fixed point is for (a recursive callee's own modes are derived with its
/// recursive call in view).
const RECURSIVE_BRANCHED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> if y == 0 then Cons (count (rebuild ys)) Nil else Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) >= 0 then 0 else 1
"#;

#[test]
fn an_if_shaped_rebuild_reuses_one_pair_per_branch() {
    let dir = source_project("if", IF_SHAPED);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // Both alternatives allocate a `Cons`, so both take a pair. The unit
    // names the reuse twice and, critically, frees the cell on neither
    // path: a free here would mean the arm kept the block alive with no
    // reuse to take it.
    assert_eq!(
        frees(&ir, "Main.rebuild"),
        0,
        "the if-shaped rebuild still frees the dying cell on a path:\n{}",
        body(&ir, "Main.rebuild")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the if-shaped rebuild leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_case_shaped_rebuild_reuses_one_pair_per_alternative() {
    let dir = source_project("case", CASE_SHAPED);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    assert_eq!(
        frees(&ir, "Main.rebuild"),
        0,
        "the case-shaped rebuild still frees the dying cell on a path:\n{}",
        body(&ir, "Main.rebuild")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the case-shaped rebuild leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_nonallocating_arm_frees_the_dying_cell() {
    let dir = source_project("mixed", MIXED_ARMS);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // The `False` arm returns the tail itself, so there is no allocation to
    // reuse the dying cell into. The pair must not be placed: a per-arm
    // release on that path would keep the cell alive with nothing to take
    // it, which is the leak the straight-line rule exists to prevent.
    assert!(
        frees(&ir, "Main.mixed") > 0,
        "a branch with a non-allocating arm took a per-arm pair, which would \
         strand the cell on that path:\n{}",
        body(&ir, "Main.mixed")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the refused shape leaked anyway: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_recursive_branched_rebuild_is_leak_free_in_both_builds() {
    let dir = source_project("rec", RECURSIVE_BRANCHED);
    // The property the rule has to survive: a recursive unit whose
    // allocation is inside a branch on every path. Both a plain build and
    // an `rc_debug` build must run leak-free, because the debug profile
    // traps on a use after free rather than reading reused memory.
    for (name, debug) in [("prog", false), ("prog_dbg", true)] {
        let (code, err) = build_and_run(&dir, name, debug);
        assert_eq!(code, Some(0), "debug {debug}: {err}");
        assert!(
            err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
            "debug {debug} leaked: {err}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
