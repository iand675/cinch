//! The argv gap: a compiled Cinch program reads its OWN command line.
//!
//! Three layers meet here. The runtime captures the process's
//! `argc`/`argv` (`cinch_argv_init`, `cinch_argv_count`,
//! `cinch_argv_get`; `compiler/runtime/src/platform.rs`), the generated
//! native entry hands them over before any program code runs
//! (`compiler/src/codegen.rs`'s `@main`), and `Effect.Process`'s
//! `process-args` is the binding a program reads them with. Each layer
//! has its own probe elsewhere (the `cinch_argv_*` unit tests, the
//! emitted entry); THIS test is the end-to-end half: it builds a program
//! whose `main` compares the words it RECEIVED against literals in its
//! own source, runs the built executable from Rust with a real command
//! line, and reads the verdict off the exit code.
//!
//! The exit code is the program's whole observable surface, and the
//! program encodes its verdict in it:
//!
//! | code | meaning |
//! |------|---------|
//! | 0 | the received words are exactly the expected ones, in order |
//! | 1..=9 | the word at that 1-based position differs |
//! | 10 | the received list is short (an expected word was missing) |
//! | 11 | the received list is long (an extra word arrived) |
//! | 12 | a received word is not valid UTF-8 |
//!
//! A program that ignored its command line could not produce these
//! codes, and a program that read a canned list could not produce
//! different ones from different argument vectors, which is what every
//! case below varies. An exit of 0 requires the received list to equal
//! the expected list exactly: same words, same order, same length. That
//! is also what pins the word-0 convention: `process-args` excludes the
//! program name, so a capture that INCLUDED it (or a list that started
//! at the wrong index) would report a mismatch at position 1 in every
//! case, the exact one included.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

/// How a program's exit code is reported when it is killed rather than
/// exiting (no case here expects it).
const KILLED: i32 = -1;

/// The program's source: its `main` compares what the process received
/// against `expected`, which is written as literals into the program's
/// own text, and encodes the verdict (this file's header) as its exit
/// code.
fn program(expected: &[&str]) -> String {
    let mut literal = String::from("Nil");
    for word in expected.iter().rev() {
        literal = format!("Cons \"{word}\" ({literal})");
    }
    format!(
        r#"
module Main where
  import Data.List
  import Data.Nat
  import Data.Result
  import Effect.Process

  -- The verdict: 0 for the exact list, the 1-based position of the
  -- first differing word, 10 when a word is missing, 11 when one is
  -- extra.
  verdict :: List String -> List String -> Nat -> Nat
  verdict got want i = case got of
    Nil -> case want of
      Nil -> 0
      Cons w ws -> 10
    Cons g gs -> case want of
      Nil -> 11
      Cons w ws -> case g == w of
        True -> verdict gs ws (i + 1)
        False -> i

  expected-words :: List String
  expected-words = {literal}

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in case process-args () of
    Err e -> 12
    Ok got -> nat-to-int (verdict got expected-words 1)
"#
    )
}

/// A project holding `program(expected)` as its `Main.cinch`, built into
/// an executable inside the project. The caller owns the directory
/// (`common::cleanup`).
fn built(tag: &str, expected: &[&str]) -> (PathBuf, PathBuf) {
    let src = program(expected);
    let dir = common::project(tag, &[("Main.cinch", src.as_str())]);
    let exe = common::build(&dir, "Main.cinch", "program").unwrap_or_else(|e| panic!("{tag}: {e}"));
    (dir, exe)
}

/// Run a built executable with `args` as its command line; its exit code.
fn run(exe: &Path, args: &[&str]) -> i32 {
    Command::new(exe)
        .args(args)
        .status()
        .expect("run the built program")
        .code()
        .unwrap_or(KILLED)
}

/// The words a real command line delivers arrive one for one: the count,
/// every word, and the words an argument parser exists to protect (a
/// word with a space, a word that looks like a flag, a word with a `-`
/// inside, the empty word) all arrive exactly as given.
#[test]
fn a_program_reads_its_own_command_line() {
    // The expectations are literals in the program's own source, so an
    // exit of 0 is the program's own word-by-word comparison.
    let (dir, exe) = built("argv-words", &["a b", "-x", "", "c-d e"]);
    // The exact command line: the space in `a b`, the leading `-` of
    // `-x`, the empty word, and the space beside the `-` in `c-d e` all
    // survive the crossing.
    assert_eq!(run(&exe, &["a b", "-x", "", "c-d e"]), 0);
    // Spacing is part of the word, not a separator that was re-split:
    // `a  b` and `a b ` are different words from `a b`.
    assert_eq!(run(&exe, &["a  b", "-x", "", "c-d e"]), 1);
    assert_eq!(run(&exe, &["a b ", "-x", "", "c-d e"]), 1);
    // A differing word reports its 1-based position, so the words are
    // read in order and not merely counted.
    assert_eq!(run(&exe, &["a b", "-y", "", "c-d e"]), 2);
    assert_eq!(run(&exe, &["a b", "-x", "empty", "c-d e"]), 3);
    assert_eq!(run(&exe, &["a b", "-x", "", "c-de"]), 4);
    // Short and long lists report themselves: the count is real, and an
    // exact exit requires the exact length.
    assert_eq!(run(&exe, &["a b", "-x", ""]), 10);
    assert_eq!(run(&exe, &[]), 10);
    assert_eq!(run(&exe, &["a b", "-x", "", "c-d e", "extra"]), 11);
    common::cleanup(&dir);
}

/// The empty command line is zero words, and the empty word is a word:
/// `""` is an argument that arrived, while no argument at all is not.
#[test]
fn an_empty_command_line_is_zero_words_and_the_empty_word_is_a_word() {
    let (none, none_exe) = built("argv-none", &[]);
    assert_eq!(run(&none_exe, &[]), 0);
    // The same program with one word arrives at "extra", so its 0 above
    // really is zero received words.
    assert_eq!(run(&none_exe, &[""]), 11);
    common::cleanup(&none);

    let (empty, empty_exe) = built("argv-empty-word", &[""]);
    assert_eq!(run(&empty_exe, &[""]), 0);
    assert_eq!(run(&empty_exe, &[]), 10);
    common::cleanup(&empty);
}

/// The CLI forwards the words after the entry file to the program's own
/// command line, and `--` ends the flags so a program argument that
/// looks like one can still be passed.
#[test]
fn the_cli_forwards_arguments_after_the_entry_file() {
    let src = program(&["a", "b c"]);
    let dir = common::project("argv-cli", &[("Main.cinch", src.as_str())]);
    let entry = dir.join("Main.cinch");
    let cinch_run = |args: &[&str]| -> i32 {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_cinch"));
        cmd.arg("run").arg(&entry);
        cmd.args(args);
        cmd.status()
            .expect("run the cinch CLI")
            .code()
            .unwrap_or(KILLED)
    };
    // The forwarded words are the program's own arguments, verbatim
    // (the space included).
    assert_eq!(cinch_run(&["a", "b c"]), 0);
    // A differing word arrives as itself: the program saw `b d`, so the
    // forwarding is what it read, not something the CLI canned.
    assert_eq!(cinch_run(&["a", "b d"]), 2);
    // One word short, the program says so.
    assert_eq!(cinch_run(&["a"]), 10);
    common::cleanup(&dir);

    // `--` forwards a word that looks like the compiler's own flag.
    let dash_src = program(&["--rebuild", "-x"]);
    let dir = common::project("argv-cli-dash", &[("Main.cinch", dash_src.as_str())]);
    let status = Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("run")
        .arg(dir.join("Main.cinch"))
        .args(["--", "--rebuild", "-x"])
        .status()
        .expect("run the cinch CLI");
    assert_eq!(status.code().unwrap_or(KILLED), 0);
    common::cleanup(&dir);
}
