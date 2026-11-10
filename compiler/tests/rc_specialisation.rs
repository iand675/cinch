//! Count-operation savings of design/reclamation.md section 4 ("As
//! built"): borrowed parameters of a closed program, drop specialisation
//! (`Core::Release`) and static dictionaries. Each program is checked for
//! where the operations went in its IR, and runs leak-free both plainly
//! and from a `rc_debug` build under `CINCH_RC_DEBUG=1`.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
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
    // The instance has only methods: one immortal static block, which
    // its factory returns and a use in this module reads directly.
    assert!(ir.contains("@\"Main.SizeInt.dict\" = "), "no static dictionary");
    assert!(ir.contains("ptrtoint ptr @\"Main.SizeInt.dict\" to i64"), "no direct use of the static dictionary");
    // `lastOr` only reads its list: the parameter and its fields are
    // borrowed, so nothing dups or drops a cell.
    let last = body(&ir, "Main.lastOr");
    assert!(!last.contains("@cinch.rc.dup.obj("), "lastOr dups a cell:\n{last}");
    assert!(!last.contains("@cinch.rc.drop.obj("), "lastOr drops a cell:\n{last}");
    // `bump` returns its list on one path, so it owns it; the other
    // path keeps both fields and frees a uniquely held cell outright.
    let bump = body(&ir, "Main.bump");
    assert!(bump.contains("call void @cinch_free(i64"), "bump does not specialise its drop:\n{bump}");
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
