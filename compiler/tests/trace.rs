//! The fault path's backtrace: frame pointers, the walk, and the trace.
//!
//! A Cinch program cannot throw, but the C and Rust code it calls can, and
//! a fault has to say where it happened. Three mechanisms are tested here,
//! each on its own and then together:
//!
//! | mechanism | what it is | how it is observed |
//! |-----------|------------|--------------------|
//! | frame pointers | `"frame-pointer"="all"` on every native unit | the emitted IR, and the object's `add x29, sp` |
//! | the walk | `cinchruntime::trace::walk` | the runtime's own unit tests |
//! | the trace | `CINCH_BACKTRACE=1` at a fault | a program that faults through a foreign call |
//!
//! The program under test is deliberately shaped so the fault sits behind a
//! foreign frame: `main` calls a Cinch unit, that unit calls a C function,
//! and the C function faults through `cinch_panic`. The walk must cross the
//! C frame (which keeps a frame pointer because clang does by default at
//! `-O0`), name the Cinch frame above it, and terminate.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

/// The Cinch program: a chain whose deepest frame faults through a foreign
/// call. `id3` is a real foreign call so the unit that calls it cannot be
/// folded into its caller.
const PROGRAM: &str = r#"
module Trace where
  import Data.Int
  import Data.Machine

  foreign import ccall "fault_now" fault-now :: forall r. Unit ->{IO r} Unit
  foreign import ccall "id3" id3 :: forall r. I64 ->{IO r} I64

  deep :: forall r. I64 ->{IO r} I64
  deep n = let a = id3 n in let v = fault-now () in a

  main :: Unit ->{IO r} Int
  main u = let v = deep 0 in int-from-i64 v
"#;

/// The C half: a foreign callee that faults through the runtime's own
/// defined fault path, which is exactly what a C library does when it
/// aborts.
const FOREIGN: &str = r#"
extern void cinch_panic(const unsigned char *, unsigned long);
long id3(long x) { return x; }
void fault_now(void) {
  static const unsigned char msg[] = "fault behind a foreign frame";
  cinch_panic(msg, sizeof(msg) - 1);
}
"#;

fn project(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_trace_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    dir
}

/// Build the program and its foreign object into one executable.
fn build(dir: &Path) -> PathBuf {
    std::fs::write(dir.join("Trace.cinch"), PROGRAM).expect("write program");
    let c = dir.join("fault.c");
    std::fs::write(&c, FOREIGN).expect("write foreign source");
    let o = dir.join("fault.o");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = Command::new(&clang)
        .args(["-O0", "-c"])
        .arg(&c)
        .arg("-o")
        .arg(&o)
        .status()
        .expect("run clang");
    assert!(status.success(), "the foreign object did not compile");
    let exe = dir.join("prog");
    let out = exe.clone();
    let src = PROGRAM.to_string();
    let objects = vec![o.to_str().unwrap().to_string()];
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let refs: Vec<&str> = objects.iter().map(String::as_str).collect();
            cinch::driver::build_source_objects(&src, &refs, &out)
        })
        .expect("spawn build")
        .join()
        .expect("build thread")
        .unwrap_or_else(|e| panic!("build: {e}"));
    exe
}

fn run(exe: &Path, env: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new(exe);
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn program")
}

/// The fault message is the program's contract: without
/// `CINCH_BACKTRACE` the output is the one line it always was.
#[test]
fn a_fault_prints_one_line_by_default() {
    let dir = project("plain");
    let exe = build(&dir);
    let out = run(&exe, &[]);
    assert_eq!(out.status.code(), Some(71), "{out:?}");
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        err.trim_end(),
        "cinchruntime: fault behind a foreign frame",
        "the fault message changed: {err:?}"
    );
    assert!(
        !err.contains("cinch-backtrace"),
        "a trace printed without being asked for: {err:?}"
    );
    common::cleanup(&dir);
}

/// With `CINCH_BACKTRACE=1` the trace names the Cinch frames the fault
/// passed through, and it terminates rather than walking garbage.
#[test]
fn a_trace_names_the_cinch_frames_at_a_foreign_fault() {
    let dir = project("trace");
    let exe = build(&dir);
    let out = run(&exe, &[("CINCH_BACKTRACE", "1")]);
    assert_eq!(out.status.code(), Some(71), "{out:?}");
    let err = String::from_utf8_lossy(&out.stderr);
    // The message is still first and unchanged.
    assert!(
        err.contains("cinchruntime: fault behind a foreign frame"),
        "the fault message is gone: {err:?}"
    );
    assert!(err.contains("cinch-backtrace:"), "no trace printed: {err:?}");
    let frames: Vec<&str> = err
        .lines()
        .filter(|l| l.trim_start().starts_with('#'))
        .collect();
    assert!(
        frames.len() >= 3,
        "the walk found fewer frames than the chain has: {err:?}"
    );
    // Every frame prints a file address (`+0x...`), which is what a
    // symbolizer needs: the absolute address alone is slid and names
    // nothing.
    for f in &frames {
        assert!(f.contains("+0x"), "a frame has no file address: {f}");
    }
    // The Cinch frame that called the foreign function is one of them.
    // Its file address is resolved from the binary's own symbol table, so
    // this does not depend on a symbolizer being installed.
    let symbols = Command::new("nm")
        .arg("-n")
        .arg(&exe)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned());
    if let Some(symbols) = symbols {
        // The Cinch unit whose body calls the foreign function. The
        // inliner may copy `deep` into the entry adapter, so the symbol
        // to look for is whichever Cinch symbol exists; the assertion is
        // that a frame lands inside one, which is what "the trace names
        // Cinch code" means.
        let cinch: Vec<(u64, String)> = symbols
            .lines()
            .filter(|l| l.contains("_Trace."))
            .filter_map(|l| {
                let mut parts = l.split_whitespace();
                let addr = u64::from_str_radix(parts.next()?, 16).ok()?;
                let name = parts.last()?.to_string();
                Some((addr, name))
            })
            .collect();
        assert!(!cinch.is_empty(), "the program has no Cinch symbols at all:\n{symbols}");
        let named = frames.iter().any(|f| {
            let Some(off) = f
                .split("+0x")
                .nth(1)
                .and_then(|rest| rest.split_whitespace().next())
                .and_then(|a| u64::from_str_radix(a, 16).ok())
            else {
                return false;
            };
            cinch.iter().any(|(start, _)| off >= *start && off < start + 0x800)
        });
        assert!(
            named,
            "no frame landed in any Cinch unit ({:?}): {err:?}",
            cinch.iter().map(|(a, n)| format!("{n}@0x{a:x}")).collect::<Vec<_>>()
        );
    }
    common::cleanup(&dir);
}

/// The emitted IR asks for a frame pointer on every native unit, which is
/// what makes the walk possible at all: without it the register is saved
/// and restored but never set, so `[x29]` is not a chain.
#[test]
fn every_native_unit_asks_for_a_frame_pointer() {
    let program = cinch::driver::check_sources(PROGRAM, "<test>").expect("check");
    let ir = cinch::codegen::emit(&program).expect("emit");
    let groups: Vec<&str> = ir
        .lines()
        .filter(|l| l.starts_with("attributes #"))
        .collect();
    assert!(!groups.is_empty(), "the module emitted no attribute groups");
    for g in groups {
        assert!(
            g.contains("\"frame-pointer\"=\"all\""),
            "an attribute group omits the frame pointer: {g}"
        );
    }
    // And the Cinch units are among the functions that carry one.
    assert!(ir.contains("define i64 @\"Trace.deep\""), "{ir}");
}

/// A `nounwind` frame has no unwind information at all, so a foreign
/// unwind cannot cross it. This is the reason the attribute is claimed
/// only where the callee set proves it, and it is a fact about the object,
/// not about the IR: the linker's encoding for such a frame is
/// `DW_EH_PE_omit`.
#[test]
fn a_nounwind_frame_carries_no_unwind_information() {
    // Two functions, one of each shape, at -O2 through clang directly: the
    // point is what the backend does with the attribute, which the
    // compiler's own IR cannot answer.
    let dir = project("nounwind");
    let ll = dir.join("probe.ll");
    std::fs::write(
        &ll,
        "target triple = \"aarch64-apple-darwin\"\n\
         declare i64 @g(i64)\n\
         define i64 @unwindable(i64 %x) #0 {\n\
         entry:\n  %a = alloca i64\n  store i64 %x, ptr %a\n  %y = call i64 @g(i64 %x)\n  ret i64 %y\n}\n\
         define i64 @nounwind_frame(i64 %x) #1 {\n\
         entry:\n  %a = alloca i64\n  store i64 %x, ptr %a\n  %y = call i64 @g(i64 %x)\n  ret i64 %y\n}\n\
         attributes #0 = { uwtable \"frame-pointer\"=\"all\" }\n\
         attributes #1 = { nounwind uwtable \"frame-pointer\"=\"all\" }\n",
    )
    .expect("write probe");
    let o = dir.join("probe.o");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = Command::new(&clang)
        .args(["-O2", "-c"])
        .arg(&ll)
        .arg("-o")
        .arg(&o)
        .status()
        .expect("run clang");
    assert!(status.success(), "the probe did not compile");
    // `__compact_unwind` is the Mach-O table the linker turns into
    // `__unwind_info`. Read it: the unwindable function has an entry and
    // the `nounwind` one does not, which is the whole reason a foreign
    // unwind terminates at a `nounwind` frame.
    let unwind = Command::new("llvm-objdump")
        .args(["--macho", "--unwind-info"])
        .arg(&o)
        .output();
    let Ok(unwind) = unwind else {
        // No llvm-objdump on PATH: the claim is still checked by
        // `compiler/tests/trace.rs`'s end-to-end case, which aborts a
        // real unwind at a real `nounwind` frame.
        common::cleanup(&dir);
        return;
    };
    let text = String::from_utf8_lossy(&unwind.stdout);
    let symbols = Command::new("nm").arg(&o).output().expect("run nm");
    let syms = String::from_utf8_lossy(&symbols.stdout);
    let addr_of = |name: &str| -> u64 {
        syms.lines()
            .find(|l| l.ends_with(name))
            .and_then(|l| l.split_whitespace().next())
            .and_then(|a| u64::from_str_radix(a, 16).ok())
            .unwrap_or_else(|| panic!("no symbol {name} in {syms}"))
    };
    let unwindable = addr_of("_unwindable");
    let nounwind = addr_of("_nounwind_frame");
    // One `function offset=0x...` line per function with unwind info.
    let has_entry = |addr: u64| {
        text.lines()
            .filter(|l| l.contains("function offset=0x"))
            .any(|l| {
                l.split("function offset=0x")
                    .nth(1)
                    .and_then(|rest| rest.split(|c: char| !c.is_ascii_hexdigit()).next())
                    .and_then(|h| u64::from_str_radix(h, 16).ok())
                    .map(|a| a == addr)
                    .unwrap_or(false)
            })
    };
    assert!(has_entry(unwindable), "the unwindable function has no unwind entry:\n{text}");
    assert!(
        !has_entry(nounwind),
        "a `nounwind` function has unwind information, so this test's premise is wrong:\n{text}"
    );
    common::cleanup(&dir);
}
