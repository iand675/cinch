//! A collection literal that calls the definition it sits in.
//!
//! The collection literal fold (surface grammar local amendments section
//! 10) evaluates any literal with no free locals at compile time. A call
//! to the enclosing definition, as in `g [f 0]` inside `f`, has none, so
//! the fold asked the staging evaluator for `f`'s code. That code did not
//! exist yet. `compile_ordinary_on_demand` checked `f` again from the top,
//! and the re-check reached the same fold, so the checker recursed until
//! the stack overflowed. The checker now tracks the definitions on its own
//! stack and refuses to compile one of them on demand. The fold sees a
//! failed evaluation and keeps the runtime `from-list` call, as it does
//! for any literal it cannot evaluate.
//!
//! The cases are the minimal reduction (a terminating self call), a call
//! that diverges at runtime, mutual recursion through a second definition,
//! a positive control showing that a literal over a finished definition
//! still folds, and the `bytes-concat` shape that first hit the crash.

mod common;

use cinch::check::{check_program_multi_full, StagingCx};
use cinch::core::TargetProfile;
use cinch::driver::{parse_prelude, parse_source};
use std::collections::HashMap;

/// The `(kind, folded)` record of every collection literal site, in
/// source order, from the staging report of a checked single-file program.
fn literal_folds(src: &str) -> Vec<(String, bool)> {
    let prelude = parse_prelude(&[]).expect("prelude parses");
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new()).expect("source parses");
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (_program, _ifaces, report, _diags) = check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &[],
        &TargetProfile::native(),
    )
    .unwrap_or_else(|e| panic!("program checks: {e}"));
    report.literal_folds.iter().map(|f| (f.kind.clone(), f.folded)).collect()
}

fn runtime_list() -> (String, bool) {
    ("list".to_string(), false)
}

#[test]
fn self_call_in_a_list_literal_checks_and_stays_runtime() {
    // `f 0` terminates, so this was never a runaway evaluation. The
    // checker recursed through on-demand compilation alone.
    let folds = literal_folds(
        "module Main where\n\
         \x20 g :: List Nat -> Nat\n\
         \x20 g xs = 0\n\
         \x20 f :: Nat -> Nat\n\
         \x20 f n = if n == 0 then 0 else g [f 0]\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert_eq!(folds, vec![runtime_list()]);
}

#[test]
fn divergent_self_call_in_a_list_literal_checks() {
    // The program loops at runtime, which is the program's own business.
    // Checking it must still terminate.
    let folds = literal_folds(
        "module Main where\n\
         \x20 g :: List Nat -> Nat\n\
         \x20 g xs = 0\n\
         \x20 f :: Nat -> Nat\n\
         \x20 f n = g [f 0]\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert_eq!(folds, vec![runtime_list()]);
}

#[test]
fn mutual_recursion_through_a_list_literal_checks() {
    // `f`'s literal compiles `h` on demand, and `h` calls back into `f`,
    // which is still being checked.
    let folds = literal_folds(
        "module Main where\n\
         \x20 g :: List Nat -> Nat\n\
         \x20 g xs = 0\n\
         \x20 f :: Nat -> Nat\n\
         \x20 f n = if n == 0 then 0 else g [h 1]\n\
         \x20 h :: Nat -> Nat\n\
         \x20 h n = if n == 0 then 0 else f (n - 1)\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert_eq!(folds, vec![runtime_list()]);
}

#[test]
fn literal_over_a_finished_definition_still_folds() {
    // The guard only covers definitions on the checker's stack: `k` is
    // not on it, so `[k 0, 2]` still evaluates and folds.
    let folds = literal_folds(
        "module Main where\n\
         \x20 k :: Nat -> Nat\n\
         \x20 k n = n + 1\n\
         \x20 f :: Nat -> List Nat\n\
         \x20 f n = if n == 0 then [k 0, 2] else Nil\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert_eq!(folds, vec![("list".to_string(), true)]);
}

#[test]
fn bytes_concat_of_self_calls_checks() {
    // The shape that first overflowed, inside `lib_bzip2`: the recursive
    // calls are elements of the list literal passed to `bytes-concat`.
    // `expected` is the non-recursive wrapper that worked around it.
    let dir = common::project(
        "literal_fold_recursion",
        &[
            (
                "Data.Bytes.Build.cinch",
                include_str!("../prelude/Data/Bytes/Build.cinch"),
            ),
            (
                "Main.cinch",
                "module Main where\n\
                 \x20 import Data.Bytes.Build\n\
                 \x20 fill :: Bytes -> Nat -> Nat -> Nat -> Nat\n\
                 \x20 fill out kind i seed = if i >= bytes-len out then seed else let w = bytes-set out i 7 in fill out kind (i + 1) (seed * 31 + kind)\n\
                 \x20 corpus :: Nat -> Nat -> Bytes\n\
                 \x20 corpus kind size =\n\
                 \x20   if kind == 6 then bytes-concat [corpus 2 700, corpus 4 900]\n\
                 \x20   else let out = bytes-new size in let filled = fill out kind 0 12345 in out\n\
                 \x20 expected :: Nat -> Nat -> Bytes\n\
                 \x20 expected kind size = if kind == 6 then bytes-concat [corpus 2 700, corpus 4 900] else corpus kind size\n\
                 \x20 main :: Int\n\
                 \x20 main = nat-to-int (bytes-len (corpus 6 0) + bytes-len (expected 6 0))\n",
            ),
        ],
    );
    common::check(&dir, "Main.cinch").unwrap_or_else(|e| panic!("corpus checks: {e}"));
    common::cleanup(&dir);
}
