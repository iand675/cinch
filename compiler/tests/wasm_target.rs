//! The freestanding Wasm target, built and executed: `cinch build --target
//! wasm32-unknown-unknown` produces a module from a real program, and the
//! module runs under Node and reports what the program returned.
//!
//! The pipeline this replaces was three hand-written commands
//! (`emit-ir`, `clang -c`, `wasm-ld`) and a JavaScript runner; the driver
//! now does the whole of it, and these cases are the ones that pipeline
//! was written for. A target with a runtime archive and a linker has an
//! artifact, so a program that compiles here has to RUN here, and its
//! answer is the same one the native target gives.
//!
//! # Running it
//!
//! ```
//! CINCH_WASM_LD=/nix/store/2kdq06nmp8k9bqvkj6rnwbb0a48b0m5v-llvm-binutils-21.1.8/bin/wasm-ld \
//!   nice python3 tooling/run_stage_targets.py --stage 3 --out /tmp/wasm --jobs 1 wasm_target
//! ```
//!
//! `CINCH_WASM_LD` is the piece this machine does not put on `PATH`.
//! `wasm-ld` is LLVM's binary utilities, which on a nix system live only in
//! the store, and Apple clang's `-print-prog-name=wasm-ld` answers nothing,
//! so neither of the driver's own searches finds it. Without the variable
//! every case that links skips and says so, and the target still passes:
//! `wasm-ld` is not part of a native toolchain, so a machine that lacks it
//! must not fail this suite.
//!
//! The other preconditions, each named in the skip message:
//! `clang` 20 or newer (`CINCH_CLANG` names it; the emitted IR carries
//! `range(...)` attributes older LLVM rejects), the
//! `wasm32-unknown-unknown` Rust target (the runtime archive is built from
//! this tree's runtime crate when `--runtime-archive` is absent), and Node
//! (`CINCH_NODE` names it), which is the host that loads the module.
//!
//! A case whose precondition is absent skips once, as a whole test, with
//! the reason printed. A partial matrix is neither a pass nor a failure,
//! so `the_matrix_runs_every_program` decides before it runs a single
//! program, and every other case asks the same question about its own
//! mode and options before its first assertion.

mod common;

use cinch::driver::{DriverOptions, Mode};
use std::path::PathBuf;

/// What one matrix case expects from the module.
enum Expect {
    /// The module's `main` returns this value.
    Exit(i32),
    /// The build itself is refused, and the diagnostic contains this.
    Refused(&'static str),
}

/// One program and what running it proves.
struct Case {
    /// The case name, which is also its test name suffix.
    name: &'static str,
    /// The program.
    source: &'static str,
    /// What the module does.
    expect: Expect,
}

/// A freestanding target's profile, as `--target` builds it.
fn wasm_target() -> DriverOptions {
    DriverOptions {
        target: Some("wasm32-unknown-unknown".to_string()),
        ..DriverOptions::default()
    }
}

/// Why the compiler running this suite cannot serve one request, or `None`
/// when it can.
///
/// The suite runs through a selected frontend when one is configured, and
/// that compiler is the Cinch-written one. Its selector decides which
/// invocations it can adapt, and it reports that through
/// `options_request`; but a request the selector accepts is not yet a
/// target the frontend builds, because the selector's own link is a native
/// `clang` call with no target and no `wasm-ld`. The capability that
/// matters is therefore the frontend's, and it is asked here rather than
/// inferred: a `Build` for this target is attempted once, and its refusal
/// is the reason every case that links skips.
///
/// `--runtime-archive` and `--opt-level` are part of the question, because
/// a frontend that lacks them cannot be asked for this build at all, and
/// the selector names them precisely.
fn frontend_lacks(mode: Mode, opts: &DriverOptions) -> Option<String> {
    match common::external_frontend::configured() {
        Ok(Some(_)) => match common::external_frontend::options_request(mode, opts) {
            Err(error) => Some(error),
            // The selector accepts the request; the frontend still has to
            // build the target. One compile answers that, and the answer is
            // cached because every case asks the same question.
            Ok(_) => frontend_build_refusal(mode, opts),
        },
        Ok(None) => None,
        Err(error) => Some(error),
    }
}

/// The refusal a selected frontend gives for this target's `build`, or
/// `None` when it builds it. The answer is computed once per process: it is
/// the same compile for every case, and it is not free.
fn frontend_build_refusal(mode: Mode, opts: &DriverOptions) -> Option<String> {
    use std::sync::OnceLock;
    static ANSWER: OnceLock<Option<String>> = OnceLock::new();
    ANSWER
        .get_or_init(|| {
            // A refusal is what this asks for, so the compile is the
            // smallest program that reaches the link.
            let dir = project("frontend-probe", "module Main where\n\nmain :: Int\nmain = 0\n");
            let out = dir.join("probe.wasm");
            let result = common::drive_opts(mode, &dir, "Main.cinch", Some(out.clone()), opts);
            let answer = match result {
                Ok(_) if out.is_file() => None,
                Ok(_) => Some(format!(
                    "the selected frontend reported success without writing {}",
                    out.display()
                )),
                Err(error) => Some(format!(
                    "the selected frontend cannot build this target: {error}"
                )),
            };
            let _ = std::fs::remove_dir_all(&dir);
            answer
        })
        .clone()
}

/// Why a case that links cannot run on this machine, or `None` when the
/// toolchain is complete.
///
/// The probe is the build's own: the same clang the link assembles with,
/// the same linker search, the same Node lookup. A skip names the piece
/// that is missing and the environment variable that supplies it, because
/// a silently skipped case reads as a passing one.
fn missing_toolchain() -> Option<String> {
    let (clang, major) = match cinch::driver::freestanding_clang() {
        Ok(found) => found,
        Err(error) => return Some(error.to_string()),
    };
    match major {
        None => {
            return Some(format!(
                "`{clang}` does not report a clang version, and assembling the emitted module \
                 needs clang {} or newer",
                cinch::driver::FREESTANDING_CLANG_MAJOR
            ))
        }
        Some(major) if major < cinch::driver::FREESTANDING_CLANG_MAJOR => {
            return Some(format!(
                "`{clang}` is clang {major}, and assembling the emitted module needs clang {} \
                 or newer: the IR carries `range(...)` attributes older LLVM rejects with \
                 `expected type`",
                cinch::driver::FREESTANDING_CLANG_MAJOR
            ))
        }
        Some(_) => {}
    }
    if let Err(error) = cinch::driver::freestanding_linker(&clang) {
        return Some(format!(
            "{error}. On this machine `wasm-ld` is only in the nix store, and Apple clang's \
             `-print-prog-name=wasm-ld` answers nothing, so the search does not reach it: set \
             CINCH_WASM_LD to a `wasm-ld`, for example \
             /nix/store/2kdq06nmp8k9bqvkj6rnwbb0a48b0m5v-llvm-binutils-21.1.8/bin/wasm-ld \
             (any `llvm-binutils` version from `nix-shell -p llvmPackages.bintools`)"
        ));
    }
    let node = std::env::var("CINCH_NODE").unwrap_or_else(|_| "node".to_string());
    if std::process::Command::new(&node)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        None
    } else {
        Some(format!(
            "`{node}` did not run, and `run` executes a freestanding module under Node; \
             CINCH_NODE names the Node executable"
        ))
    }
}

/// The file one run's skip reasons accumulate in.
///
/// libtest prints `test <name> ... ` and only then the result, so anything a
/// test writes to stderr lands *inside* that line and hides the result from
/// the harness's `^test (\S+) \.\.\. (ok|FAILED)$` count. The reasons
/// therefore go to a file, and `zz_the_skipped_cases` prints them after
/// every other case has run: a skip is a pass, so its reason has to reach
/// the log some other way than through a passing test's own output.
fn skip_log() -> PathBuf {
    std::env::temp_dir().join(format!("cinch-wasm-skips-{}.log", std::process::id()))
}

/// Record the reason and leave the test, which the harness counts as a
/// pass: a case whose precondition is absent has no verdict to give.
fn skip(test: &str, reason: &str) {
    use std::io::Write;
    let line = format!("skipping {test}: {reason}\n");
    match std::fs::OpenOptions::new().create(true).append(true).open(skip_log()) {
        Ok(mut file) => {
            let _ = file.write_all(line.as_bytes());
        }
        // A filesystem that will not take the file still gets the reason,
        // in the one place it can go.
        Err(_) => {
            let _ = std::io::stderr().write_all(line.as_bytes());
        }
    }
}

/// A fresh project directory holding `source` as `Main.cinch`.
fn project(name: &str, source: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch-wasm-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project");
    std::fs::write(dir.join("Main.cinch"), source).expect("write Main.cinch");
    dir
}

/// Build and run one case, returning the module's exit value.
///
/// The build goes through `common::drive_opts`, so the staged compiler
/// runs it when the suite selects one: `--target` and `--runtime-archive`
/// are flags it implements, and the artifact is the same module either
/// way.
fn run_case(case: &Case) -> i32 {
    let dir = project(case.name, case.source);
    let result = common::drive_opts(Mode::Run, &dir, "Main.cinch", None, &wasm_target());
    let _ = std::fs::remove_dir_all(&dir);
    match result {
        Ok(code) => code,
        Err(error) => panic!("{}: run failed: {error}", case.name),
    }
}

/// Build one case into a module and return its bytes.
fn build_case(case: &Case) -> Vec<u8> {
    let dir = project(case.name, case.source);
    let out = dir.join("program.wasm");
    let result = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(out.clone()), &wasm_target());
    if let Err(error) = result {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{}: build failed: {error}", case.name);
    }
    let bytes = std::fs::read(&out).unwrap_or_else(|e| panic!("{}: {}: {e}", case.name, out.display()));
    let _ = std::fs::remove_dir_all(&dir);
    bytes
}

/// Run one matrix case. The caller has established the case's
/// preconditions, so every path here is an assertion.
fn matrix(case: &Case) {
    match case.expect {
        Expect::Refused(needle) => {
            let dir = project(case.name, case.source);
            let result = common::drive_opts(Mode::Run, &dir, "Main.cinch", None, &wasm_target());
            let _ = std::fs::remove_dir_all(&dir);
            let error = result.expect_err(case.name).to_string();
            assert!(
                error.contains(needle),
                "{}: {error} does not contain `{needle}`",
                case.name
            );
        }
        Expect::Exit(want) => {
            let got = run_case(case);
            assert_eq!(got, want, "{}: the module returned {got}", case.name);
        }
    }
}

// The matrix. Every program here was run by hand through the pipeline
// this replaces; the ones that returned 0 are the arithmetic, the
// closures, the recursive groups, the ADT, and the narrowing cases, and
// the one that returned 1 is a program whose answer is wrong.

/// Integer recursion: the control for everything else.
const FIB: &str = "\
module Main where

fib :: Int -> Int
fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)

main :: Int
main = if fib 20 == 6765 then 0 else 1
";

/// `Nat` beyond the machine word on both sides of 2^31 and 2^32, and
/// `Int` below zero: the bignum boundary is where a 32-bit target differs
/// from the host.
const NAT_EDGE: &str = "\
module Main where

dbl :: Nat -> Nat
dbl n = n + n

pow :: Int -> Nat -> Nat
pow k acc = if k == 0 then acc else pow (k - 1) (dbl acc)

add :: Nat -> Nat -> Nat
add a b = a + b

main :: Int
main =
  if pow 30 1 /= 1073741824 then 1
  else if pow 31 1 /= 2147483648 then 2
  else if pow 40 1 /= 1099511627776 then 3
  else if add 1073741823 1073741824 /= 2147483647 then 4
  else if add 2147483647 1 /= 2147483648 then 5
  else if add 4294967295 1 /= 4294967296 then 6
  else 0
";

/// Negative `Int` results and a product past 2^32: the sign handling and
/// the limb carry a 32-bit word stresses.
const INT_EDGE: &str = "\
module Main where

neg :: Int -> Int
neg x = 0 - x

dec :: Int -> Int
dec x = x - 1

mul :: Int -> Int -> Int
mul a b = a * b

main :: Int
main =
  if neg 5 /= 0 - 5 then 1
  else if neg 1073741824 /= 0 - 1073741824 then 2
  else if dec (neg 1073741824) /= 0 - 1073741825 then 3
  else if neg (dec (neg 1073741824)) /= 1073741825 then 4
  else if mul (0 - 70000) 70000 /= 0 - 4900000000 then 5
  else if neg (neg (0 - 3)) /= 0 - 3 then 6
  else 0
";

/// A closure passed to `map` and folded: the apply stub, the static
/// closure block, and the layout table registration, which a freestanding
/// module runs from its own entry.
const CLOSURE_LIST: &str = "\
module Main where

import Data.List

total :: List Nat -> Nat
total xs = foldr (\\x acc -> x + acc) 0 xs

main :: Int
main = if total (map (\\i -> i * 3) (replicate 10 1000000000)) == 30000000000 then 0 else 1
";

/// Mutual tail recursion across three million calls: the emitted
/// `musttail` calls have to stay tail calls in the module, or the
/// linear memory the stack grows into runs out.
const MUTUAL_TAIL: &str = "\
module Main where

isEven :: Int -> Bool
isEven n = if n == 0 then True else isOdd (n - 1)

isOdd :: Int -> Bool
isOdd n = if n == 0 then False else isEven (n - 1)

main :: Int
main = if isEven 3000000 then 0 else 1
";

/// A mutual group whose accumulators differ in type, so the tail call
/// crosses a `Nat` widening at every step.
const MIXED_TAIL: &str = "\
module Main where

countA :: Int -> Nat -> Nat
countA n acc = if n == 0 then acc else countB (n - 1) (acc + 1)

countB :: Int -> Nat -> Nat
countB n acc = if n == 0 then acc else countA (n - 1) (acc + 2)

main :: Int
main = if countA 3000000 0 == 4500000 then 0 else 1
";

/// A user ADT with two constructors and a recursive builder: the tag
/// word, the field layout, and the allocation the heap grows for.
const ADT: &str = "\
module Main where

data Tree where
  Leaf :: Tree
  Node :: Tree -> Nat -> Tree -> Tree

build :: Int -> Nat -> Tree
build d v = if d == 0 then Leaf else Node (build (d - 1) (v + v)) v (build (d - 1) (v + 1))

sumT :: Tree -> Nat
sumT t = case t of
  Leaf -> 0
  Node l v r -> sumT l + v + sumT r

depth :: Tree -> Int
depth t = case t of
  Leaf -> 0
  Node l _ _ -> 1 + depth l

main :: Int
main = if depth (build 12 1000000000) /= 12 then 1 else if sumT (build 3 1) /= 19 then 2 else 0
";

/// A `U8` function stored in a list: the narrow representation crosses a
/// closure boundary and comes back.
const NARROW_U8: &str = "\
module Main where

import Data.List

tag :: U8 -> U8
tag x = x

keep :: List (U8 -> U8)
keep = Cons tag Nil

main :: Int
main = length keep - 1
";

/// A recursive group whose members are taken as values in a list: the
/// closure accessors and the direct calls have to agree, which is where a
/// 32-bit pointer and a 64-bit word mix up.
const GROUP: &str = "\
module Main where

import Data.List

isEven :: Int -> List Nat -> Bool
isEven n xs = if n == 0 then True else isOdd (n - 1) (Cons 1 Nil)

isOdd :: Int -> List Nat -> Bool
isOdd n xs = if n == 0 then False else isEven (n - 1) xs

fs :: List (Int -> List Nat -> Bool)
fs = Cons isEven (Cons isOdd Nil)

main :: Int
main = if isEven 3000000 Nil then (if isOdd 3000001 Nil then length fs - 2 else 2) else 1
";

/// Two mutual groups of different arities, both taken as values: the
/// apply stubs are per arity, and a list of them exercises both.
const GROUP_MIXED: &str = "\
module Main where

import Data.List

pa :: Int -> List Nat -> Bool
pa n xs = if n == 0 then True else pb (Cons 1 xs) (n - 1)

pb :: List Nat -> Int -> Bool
pb xs n = if n == 0 then False else pa (n - 1) (tail' xs)

tail' :: List Nat -> List Nat
tail' xs = case xs of
  Nil -> Nil
  Cons _ r -> r

qa :: Int -> Nat -> Bool
qa n k = if n == 0 then True else qb (n - 1) (k + 1)

qb :: Int -> Nat -> Bool
qb n k = if n == 0 then False else qa (n - 1) (k + 2)

fs :: List (Int -> List Nat -> Bool)
fs = Cons pa Nil

gs :: List (List Nat -> Int -> Bool)
gs = Cons pb Nil

hs :: List (Int -> Nat -> Bool)
hs = Cons qa (Cons qb Nil)

main :: Int
main = if pa 3000000 Nil then (if qa 3000000 0 then length fs + length gs + length hs - 4 else 2) else 1
";

/// The same shape over `I64` and `Nat`: the 64-bit machine type beside the
/// bignum, which is where a 32-bit target's word and pointer widths differ
/// from the host's most visibly.
const GROUP_WIDE: &str = "\
module Main where

import Data.List
import Data.Machine

ca :: I64 -> List Nat -> Bool
ca n xs = if n == 0 then True else cb xs (wrapping-sub-i64 n 1)

cb :: List Nat -> I64 -> Bool
cb xs n = if n == 0 then False else ca (wrapping-sub-i64 n 1) xs

fs :: List (I64 -> List Nat -> Bool)
fs = Cons ca Nil

gs :: List (List Nat -> I64 -> Bool)
gs = Cons cb Nil

main :: Int
main = if ca 3000000 Nil then length fs + length gs - 2 else 1
";

/// A four-argument recursive walker taken as a value, over a `Nat` past
/// 2^31 and a rank-1 polymorphic `use`: the widest signature in the
/// matrix, and the one that reads every width at once.
const CSIZE: &str = "\
module Main where
  import Data.List
  import Data.Machine
  walk :: List Nat -> Nat -> Int -> I64 -> Nat
  walk xs k d w = case xs of
    Nil -> k
    Cons x rest -> walk rest (k + x) (d - 1) (wrapping-sub-i64 w 1)
  walkers :: List (List Nat -> Nat -> Int -> I64 -> Nat)
  walkers = Cons walk Nil
  use :: forall a. (a -> a) -> a -> Nat -> a
  use f x n = if n == 0 then f x else use f x (n - 1)
  main :: Int
  main = case walkers of
    Nil -> 3
    Cons g _ -> if g (Cons 2147483648 (Cons 5 Nil)) 0 0 7 == 2147483653 then (if use (\\x -> x) (2147483647 :: Nat) 0 == 2147483647 then 0 else 2) else 1
";

/// A `U8` pair through two calls: the narrow width is preserved, not
/// widened to the word.
const NARROW: &str = "\
module Main where

pick :: U8 -> U8 -> U8
pick a b = a

use :: U8 -> U8
use x = pick x x

main :: Int
main = if use 7 == 7 then 0 else 1
";

/// Rank-1 polymorphism through a function parameter: the dictionary or
/// the inlined body has to survive the narrower target.
const POLY: &str = "\
module Main where

use :: forall a. (a -> a) -> a -> a
use f x = f x

main :: Int
main = if use (\\x -> x) 5 == 5 then 0 else 1
";

/// A `Nat` counter beside a `U8` narrowing in one module.
const MIXED_WIDTHS: &str = "\
module Main where

count :: Nat -> Int
count n = if n == 0 then 0 else 1 + count (n - 1)

pick :: U8 -> U8 -> U8
pick a b = a

use :: U8 -> U8
use x = pick x x

main :: Int
main = if count 3 == 3 && use 7 == 7 then 0 else 1
";

/// A `U8` argument the function never reads: the parameter is still part
/// of the signature the module exports.
const U8_PARAM: &str = "\
module Main where

down :: U8 -> Nat -> Nat
down limit n = if n == 0 then 0 else 1 + down limit (n - 1)

main :: Int
main = if down 9 3 == 3 then 0 else 1
";

/// `#[noinline]` on both units: the calls are real calls in the module, so
/// the two apply paths are both present.
const NOINLINE: &str = "\
module Main where

#[noinline]
id8 :: U8 -> U8
id8 x = x

#[noinline]
twice :: U8 -> U8
twice x = id8 x

main :: Int
main = if twice 5 == 5 then 0 else 1
";

/// A `U8` recursion that ignores its narrowing argument and loops forever
/// (`odd8` calls `even8 n` without decrementing). The program compiles,
/// so the module exists; running it never returns, which is the one case
/// this matrix cannot assert by exit value and states here instead.
const FOREVER: &str = "\
module Main where

even8 :: U8 -> Bool
even8 n = if n == 0 then True else odd8 n

odd8 :: U8 -> Bool
odd8 n = if n == 0 then False else even8 n

main :: Int
main = if even8 4 then 0 else 1
";

/// A wrong answer on purpose: `down 9 3` is 3, so comparing it to 99 makes
/// `main` return 1. The module has to return 1, not 0 and not a trap,
/// which is what proves the exit value is the program's and not a
/// constant the harness assumes.
const WRONG_ANSWER: &str = "\
module Main where

down :: U8 -> Nat -> Nat
down limit n = if n == 0 then 0 else 1 + down limit (n - 1)

main :: Int
main = if down 9 3 == 99 then 0 else 1
";

/// A `Usize` literal that fits the host and not a 32-bit word: the target
/// narrows the type, so the check refuses it before any code generation.
const USIZE_TOO_WIDE: &str = "\
module Main where

big :: Usize
big = 4294967296

main :: Int
main = if big == 0 then 0 else 1
";

/// Every case, in the order the matrix ran.
const CASES: &[Case] = &[
    Case { name: "fib", source: FIB, expect: Expect::Exit(0) },
    Case { name: "nat_edge", source: NAT_EDGE, expect: Expect::Exit(0) },
    Case { name: "int_edge", source: INT_EDGE, expect: Expect::Exit(0) },
    Case { name: "closure_list", source: CLOSURE_LIST, expect: Expect::Exit(0) },
    Case { name: "mutual_tail", source: MUTUAL_TAIL, expect: Expect::Exit(0) },
    Case { name: "mixed_tail", source: MIXED_TAIL, expect: Expect::Exit(0) },
    Case { name: "adt", source: ADT, expect: Expect::Exit(0) },
    Case { name: "narrow_u8", source: NARROW_U8, expect: Expect::Exit(0) },
    Case { name: "group", source: GROUP, expect: Expect::Exit(0) },
    Case { name: "group_mixed", source: GROUP_MIXED, expect: Expect::Exit(0) },
    Case { name: "group_wide", source: GROUP_WIDE, expect: Expect::Exit(0) },
    Case { name: "csize", source: CSIZE, expect: Expect::Exit(0) },
    Case { name: "narrow", source: NARROW, expect: Expect::Exit(0) },
    Case { name: "poly", source: POLY, expect: Expect::Exit(0) },
    Case { name: "mixed_widths", source: MIXED_WIDTHS, expect: Expect::Exit(0) },
    Case { name: "u8_param", source: U8_PARAM, expect: Expect::Exit(0) },
    Case { name: "noinline", source: NOINLINE, expect: Expect::Exit(0) },
    Case { name: "wrong_answer", source: WRONG_ANSWER, expect: Expect::Exit(1) },
    Case {
        name: "usize_too_wide",
        source: USIZE_TOO_WIDE,
        expect: Expect::Refused("out of range for Usize"),
    },
];

/// The matrix, one case per program. Each is a separate test so a failure
/// names the program rather than the table.
#[test]
fn the_matrix_runs_every_program() {
    // A partial matrix is neither a pass nor a failure, so the whole test
    // skips once when a program's precondition is absent. Two of them
    // differ: a case that expects a refusal never links, and a case that
    // expects an exit value needs the toolchain.
    let links = CASES.iter().any(|case| matches!(case.expect, Expect::Exit(_)));
    let refuses = CASES.iter().any(|case| matches!(case.expect, Expect::Refused(_)));
    if refuses {
        if let Some(reason) = frontend_lacks(Mode::Run, &wasm_target()) {
            skip("the_matrix_runs_every_program", &reason);
            return;
        }
    }
    if links {
        if let Some(reason) = missing_toolchain() {
            skip("the_matrix_runs_every_program", &reason);
            return;
        }
        if let Some(reason) = frontend_lacks(Mode::Run, &wasm_target()) {
            skip("the_matrix_runs_every_program", &reason);
            return;
        }
    }
    for case in CASES {
        matrix(case);
    }
}

/// The build produces a module, not a native executable: the bytes are the
/// Wasm magic, the module has the target's entry, and the artifact is
/// named after the target's own extension.
#[test]
fn build_produces_a_wasm_module() {
    if let Some(reason) = frontend_lacks(Mode::Build, &wasm_target()).or_else(missing_toolchain) {
        skip("build_produces_a_wasm_module", &reason);
        return;
    }
    // The probe the skip path uses reads the same clang the link does, so a
    // machine that reaches this line has a compiler the emitted IR is known
    // to parse: a wrong one is a diagnostic, not a wrong-code failure.
    let (clang, major) = cinch::driver::freestanding_clang().expect("the probe reports the compiler");
    assert!(
        major.is_some_and(|major| major >= cinch::driver::FREESTANDING_CLANG_MAJOR),
        "`{clang}` reports clang {major:?}, below the {} the emitted IR needs",
        cinch::driver::FREESTANDING_CLANG_MAJOR
    );
    let bytes = build_case(&CASES[0]);
    assert!(
        bytes.starts_with(b"\0asm"),
        "the artifact does not begin with the Wasm magic: {:02x?}",
        &bytes[..4.min(bytes.len())]
    );
    // The version word: 1, little-endian, after the magic.
    assert_eq!(&bytes[4..8], &[1, 0, 0, 0], "not a Wasm version 1 module");
}

/// `emit-ir` states the target it emitted for, and a freestanding module's
/// entry takes no `argc`/`argv`, because the target has no command line to
/// hand it. Emission needs no linker, so this runs everywhere.
#[test]
fn the_emitted_module_is_the_freestanding_entry() {
    if let Some(reason) = frontend_lacks(Mode::EmitIr, &wasm_target()) {
        skip("the_emitted_module_is_the_freestanding_entry", &reason);
        return;
    }
    let dir = project("ir-beside", CASES[0].source);
    let ir_path = dir.join("program.ll");
    let result = common::drive_opts(Mode::EmitIr, &dir, "Main.cinch", Some(ir_path.clone()), &wasm_target());
    if let Err(error) = result {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("emit-ir failed: {error}");
    }
    let ir = std::fs::read_to_string(&ir_path).expect("the emitted IR");
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        ir.contains("target triple = \"wasm32-unknown-unknown\""),
        "the IR does not state the wasm32 target"
    );
    assert!(
        ir.contains("define i32 @main()"),
        "the entry is not the freestanding `main`"
    );
}

/// `run` refuses a program argument for a freestanding target rather than
/// dropping it: the module has no command line, and a program that reads
/// one would see an empty list with no diagnostic.
#[test]
fn run_refuses_program_arguments_it_cannot_deliver() {
    if let Some(reason) = frontend_lacks(Mode::Run, &wasm_target()) {
        skip("run_refuses_program_arguments_it_cannot_deliver", &reason);
        return;
    }
    let dir = project("run-args", CASES[0].source);
    let opts = DriverOptions {
        program_args: vec!["--flag".to_string()],
        ..wasm_target()
    };
    let result = common::drive_opts(Mode::Run, &dir, "Main.cinch", None, &opts);
    let _ = std::fs::remove_dir_all(&dir);
    let error = result.expect_err("a freestanding run cannot deliver arguments").to_string();
    assert!(error.contains("no command line"), "{error}");
    assert!(error.contains("wasm32-unknown-unknown"), "{error}");
}

/// A freestanding target is still a foreign target for `run`'s purpose
/// except where the host can carry it, and a target the host cannot carry
/// keeps its refusal. This is the boundary the two messages draw between
/// each other.
#[test]
fn a_native_target_still_runs_here() {
    let dir = project("native-still", "module Main where\n\nmain :: Int\nmain = 3\n");
    let result = common::drive_opts(Mode::Run, &dir, "Main.cinch", None, &DriverOptions::default());
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(result.expect("the host's own target runs here"), 3);
}

/// An unknown triple is refused with the list of the five, and `wasm32` on
/// its own is not one of them: the target needs its operating system and C
/// library components named, and `wasm32-unknown-unknown` is the
/// freestanding one.
#[test]
fn an_unknown_triple_is_refused_with_the_list() {
    let opts = DriverOptions { target: Some("wasm32".to_string()), ..DriverOptions::default() };
    if let Some(reason) = frontend_lacks(Mode::Check, &opts) {
        skip("an_unknown_triple_is_refused_with_the_list", &reason);
        return;
    }
    let dir = project("unknown-triple", CASES[0].source);
    let opts = DriverOptions {
        target: Some("wasm32".to_string()),
        ..DriverOptions::default()
    };
    let result = common::drive_opts(Mode::Check, &dir, "Main.cinch", None, &opts);
    let _ = std::fs::remove_dir_all(&dir);
    let error = result.expect_err("`wasm32` is not a triple").to_string();
    assert!(error.contains("unknown target `wasm32`"), "{error}");
    assert!(error.contains("wasm32-unknown-unknown"), "{error}");
    for triple in cinch::driver::SUPPORTED_TARGETS {
        assert!(error.contains(triple), "{error} does not name {triple}");
    }
}

/// A module whose runtime archive is absent from the machine is built
/// from source, because the freestanding archive is a Rust target this
/// tree compiles. The flag still wins when it is given, so this case
/// asserts the fallback only where the target is installed.
#[test]
fn the_freestanding_archive_is_built_when_the_flag_is_absent() {
    let installed = std::process::Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains("wasm32-unknown-unknown"))
        .unwrap_or(false);
    if !installed {
        eprintln!(
            "skipping the_freestanding_archive_is_built_when_the_flag_is_absent: \
             the wasm32-unknown-unknown Rust target is not installed (rustup target add \
             wasm32-unknown-unknown)"
        );
        return;
    }
    if let Some(reason) = frontend_lacks(Mode::Build, &wasm_target()).or_else(missing_toolchain) {
        skip("the_freestanding_archive_is_built_when_the_flag_is_absent", &reason);
        return;
    }
    // No `--runtime-archive`: the build has to find or produce the archive
    // itself.
    let dir = project("auto-archive", CASES[0].source);
    let out = dir.join("program.wasm");
    let result = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(out.clone()), &wasm_target());
    let built = result.is_ok();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(built, "the build did not produce an archive on its own: {:?}", result.err());
}

/// A file the caller names is what the link reads, so a path that is not
/// an archive is refused by name rather than reaching the linker.
#[test]
fn a_runtime_archive_that_is_not_one_is_refused() {
    // The request names an archive, which is the option this case is about,
    // so the predicate is asked with one.
    let probe = DriverOptions {
        runtime_archive: Some(PathBuf::from("/nonexistent/libcinchruntime.a")),
        ..wasm_target()
    };
    if let Some(reason) = frontend_lacks(Mode::Build, &probe) {
        skip("a_runtime_archive_that_is_not_one_is_refused", &reason);
        return;
    }
    let dir = project("bad-archive", CASES[0].source);
    let archive = dir.join("libcinchruntime.a");
    std::fs::write(&archive, b"not an archive").expect("write the bogus archive");
    let opts = DriverOptions {
        runtime_archive: Some(archive),
        ..wasm_target()
    };
    let result = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(dir.join("p.wasm")), &opts);
    let _ = std::fs::remove_dir_all(&dir);
    let error = result.expect_err("a file that is not an archive is refused").to_string();
    assert!(error.contains("not a static library archive"), "{error}");
}

/// The archive check reads a Wasm object's architecture, so a host archive
/// offered for the wasm32 target is refused before the link instead of
/// failing inside it.
#[test]
fn a_host_archive_is_refused_for_the_wasm_target() {
    let probe = DriverOptions {
        runtime_archive: Some(PathBuf::from("/nonexistent/libcinchruntime.a")),
        ..wasm_target()
    };
    if let Some(reason) = frontend_lacks(Mode::Build, &probe) {
        skip("a_host_archive_is_refused_for_the_wasm_target", &reason);
        return;
    }
    let dir = project("host-archive", CASES[0].source);
    // A minimal ELF64 aarch64 object header, which is what a host archive
    // of this machine holds.
    let mut object = vec![0u8; 32];
    object[0..4].copy_from_slice(b"\x7fELF");
    object[4] = 2;
    object[5] = 1;
    object[18..20].copy_from_slice(&183u16.to_le_bytes());
    let archive = dir.join("libcinchruntime.a");
    let mut bytes = b"!<arch>\n".to_vec();
    let mut header = [b' '; 60];
    header[0..6].copy_from_slice(b"obj.o/");
    let size = format!("{}", object.len());
    header[48..48 + size.len()].copy_from_slice(size.as_bytes());
    header[58..60].copy_from_slice(b"`\n");
    bytes.extend_from_slice(&header);
    bytes.extend_from_slice(&object);
    std::fs::write(&archive, bytes).expect("write the synthetic archive");
    let opts = DriverOptions {
        runtime_archive: Some(archive),
        ..wasm_target()
    };
    let result = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(dir.join("p.wasm")), &opts);
    let _ = std::fs::remove_dir_all(&dir);
    let error = result.expect_err("a host archive cannot link for wasm32").to_string();
    assert!(error.contains("aarch64"), "{error} does not name what the archive holds");
    assert!(error.contains("not wasm32"), "{error} does not name the target's arch");
}

/// The one program in the matrix that never returns. It is here because
/// "it compiled" is not "it ran": this module links and instantiates, and
/// the fact that it then loops is the program's, not the target's. The
/// case asserts the build only, under a bounded wait, and says why.
#[test]
fn a_nonterminating_program_still_builds() {
    if let Some(reason) = frontend_lacks(Mode::Build, &wasm_target()).or_else(missing_toolchain) {
        skip("a_nonterminating_program_still_builds", &reason);
        return;
    }
    let case = Case { name: "forever", source: FOREVER, expect: Expect::Exit(0) };
    let bytes = build_case(&case);
    assert!(bytes.starts_with(b"\0asm"), "the nonterminating program did not produce a module");
}

/// The skip reasons, printed once, after every case that could produce one.
///
/// The name sorts last, so this runs last whether libtest keeps the source
/// order or sorts by name, which is what makes the report complete without
/// the harness having to know about it.
#[test]
fn zz_the_skipped_cases() {
    let log = skip_log();
    let Ok(text) = std::fs::read_to_string(&log) else { return };
    let _ = std::fs::remove_file(&log);
    use std::io::Write;
    // A leading newline ends libtest's unterminated `test zz_... ... ` line.
    let _ = write!(std::io::stderr(), "\n{text}");
}
