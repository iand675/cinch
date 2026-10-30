//! Reclamation (design/reclamation.md slice S6): compiled programs built
//! with reference counting (`TargetProfile::reclaim`) free what they
//! allocate. Each program runs under `CINCH_LEAK_CHECK=1`, which reports
//! `cinch-leak: live= immortal= leaked=` at exit and exits 74 when a
//! non-immortal block is still live, and again from a `rc_debug` build
//! under `CINCH_RC_DEBUG=1`, where a count operation on a freed block
//! traps (exit 71).

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn project(tag: &str, src: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_rc_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    dir
}

fn build(dir: &Path, debug: bool) -> PathBuf {
    let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    let (src, out) = (dir.join("Main.cinch"), exe.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver_with_profile(Mode::Build, StageMode::Refresh, &src, Some(&out), &profile))
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("build failed: {e}"));
    exe
}

fn run(exe: &Path, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M").env_remove("CINCH_ALLOC_STATS");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn program")
}

fn field(out: &Output, prefix: &str, name: &str) -> u64 {
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err.lines().find(|l| l.starts_with(prefix)).unwrap_or_else(|| panic!("no {prefix} line: {err}"));
    line.split_whitespace()
        .find_map(|w| w.strip_prefix(&format!("{name}=")))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no {name} in {line}"))
}

/// Build `src` both ways, check the program exits `code` with
/// `leaked` blocks left, plainly and under the debug checks.
fn expect_leaked(tag: &str, src: &str, code: i32, leaked: u64) {
    expect_leaked_project(tag, src, &[], code, leaked);
}

/// `expect_leaked` for a project with `extra` modules beside `Main`.
/// `__DIR__` in `src` becomes the project directory, so a program doing
/// file IO keeps its files inside its own scratch directory.
fn expect_leaked_project(tag: &str, src: &str, extra: &[(&str, &str)], code: i32, leaked: u64) {
    let dir = project(tag, src);
    let main = src.replace("__DIR__", &dir.display().to_string());
    std::fs::write(dir.join("Main.cinch"), main).expect("write source");
    for (file, text) in extra {
        std::fs::write(dir.join(file), text).expect("write module");
    }
    for debug in [false, true] {
        let exe = build(&dir, debug);
        let mut env = vec![("CINCH_LEAK_CHECK", "1")];
        if debug {
            env.push(("CINCH_RC_DEBUG", "1"));
        }
        let out = run(&exe, &env);
        let n = field(&out, "cinch-leak:", "leaked");
        assert_eq!(n, leaked, "{tag} (debug {debug}): {out:?}");
        let want = if leaked == 0 { code } else { 74 };
        assert_eq!(out.status.code(), Some(want), "{tag} (debug {debug}): {out:?}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

const LISTS: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  sumL :: List Int -> Int -> Int
  sumL xs acc = case xs of
    Nil -> acc
    Cons h t -> sumL t (acc + h)

  mapL :: (Int -> Int) -> List Int -> List Int
  mapL f xs = case xs of
    Nil -> Nil
    Cons h t -> Cons (f h) (mapL f t)

  filterL :: (Int -> Bool) -> List Int -> List Int
  filterL p xs = case xs of
    Nil -> Nil
    Cons h t -> if p h then Cons h (filterL p t) else filterL p t

  main :: Int
  main =
    let xs = build 1000 Nil in
    let ys = mapL (\x -> x * 2) (filterL (\x -> x > 500) xs) in
    if sumL xs 0 == 500500 then (if sumL ys 0 == 750500 then 0 else 2) else 1
"#;

#[test]
fn lists_built_mapped_filtered_and_folded_are_freed() {
    expect_leaked("lists", LISTS, 0, 0);
}

const TREES: &str = r#"
module Main where
  data Tree where
    Leaf :: Tree
    Node :: Tree -> Int -> Tree -> Tree

  insert :: Int -> Tree -> Tree
  insert x t = case t of
    Leaf -> Node Leaf x Leaf
    Node l v r -> if x < v then Node (insert x l) v r else Node l v (insert x r)

  total :: Tree -> Int
  total t = case t of
    Leaf -> 0
    Node l v r -> total l + v + total r

  fill :: Int -> Tree -> Tree
  fill n t = if n == 0 then t else fill (n - 1) (insert ((n * 37) - (n / 7) * 7) t)

  main :: Int
  main = if total (fill 200 Leaf) > 0 then 0 else 1
"#;

#[test]
fn trees_rebuilt_by_insertion_are_freed() {
    expect_leaked("trees", TREES, 0, 0);
}

const CLOSURES: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  add3 :: Int -> Int -> Int -> Int
  add3 a b c = a + b + c

  apply :: (Int -> Int) -> Int -> Int
  apply f x = f x

  main :: Int
  main =
    let xs = build 50 Nil in
    let count = \u -> length xs in
    let g = add3 1 in
    let h = g 2 in
    if count () + count () == 100 then (if apply h 3 + apply (add3 1 1) 1 == 9 then 0 else 2) else 1
"#;

#[test]
fn closures_and_partial_applications_are_freed() {
    expect_leaked("closures", CLOSURES, 0, 0);
}

const DICTS: &str = r#"
module Main where
  signature Box a where
    unbox :: a -> Int

  data Wrap where
    W :: Int -> Wrap

  implicit instance Box_Wrap :: Box Wrap where
    unbox w = case w of
      W n -> n

  twice :: forall a. using (b :: Box a) => a -> Int
  twice w = unbox w + unbox w

  addN :: forall a. using (add :: Add a) => a -> a -> a
  addN x y = x + y

  main :: Int
  main = if twice (W 3) + addN 1 1 == 8 then 0 else 1
"#;

#[test]
fn dictionaries_are_freed() {
    expect_leaked("dicts", DICTS, 0, 0);
}

const BIGNUMS: &str = r#"
module Main where
  grow :: Int -> Int -> Int
  grow n acc = if n == 0 then acc else grow (n - 1) (acc * 3)

  main :: Int
  main = if grow 200 1 > 1000000000000000000000000000000 then 0 else 1
"#;

#[test]
fn bignum_temporaries_are_freed() {
    expect_leaked("bignums", BIGNUMS, 0, 0);
}

/// A `letrec` value knot stays allocated until S8 tears knots down
/// (D396): the box keeps its value (an uncounted slot), and the value's
/// closure keeps the box. Exactly those blocks remain: the box, the
/// tuple, the closure.
const KNOT: &str = r#"
module Main where
  main :: Int
  main =
    letrec r :: (Nat, Unit -> Nat); r = (41, \u -> case r of (a, g) -> a + 1)
    in case r of (a, g) -> if g () == 42 then 0 else 1
"#;

#[test]
fn a_letrec_knot_leaks_exactly_its_cycle() {
    expect_leaked("knot", KNOT, 0, 3);
}

fn loop_program(n: usize, rounds: usize) -> String {
    format!(
        r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  rounds :: Int -> Int -> Int
  rounds k acc = if k == 0 then acc else rounds (k - 1) (acc + length (build {n} Nil))

  main :: Int
  main = if rounds {rounds} 0 == {total} then 0 else 1
"#,
        total = n * rounds
    )
}

#[test]
fn building_and_dropping_a_list_repeatedly_peaks_near_one_list() {
    let dir = project("peak", &loop_program(1000, 1000));
    let exe = build(&dir, false);
    let out = run(&exe, &[("CINCH_ALLOC_STATS", "1"), ("CINCH_LEAK_CHECK", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let peak = field(&out, "cinch-alloc:", "peak");
    let bytes = field(&out, "cinch-alloc:", "bytes");
    // One list of 1000 cells is 32 KB with headers; a thousand of them
    // would be 32 MB.
    assert!(peak < 1_000_000, "peak {peak} of {bytes} allocated: {out:?}");
    assert!(bytes > 30_000_000, "{out:?}");
    assert_eq!(field(&out, "cinch-leak:", "leaked"), 0, "{out:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_million_cell_list_drops_without_stack_growth() {
    let dir = project("million", &loop_program(1_000_000, 1));
    let exe = build(&dir, false);
    let out = run(&exe, &[("CINCH_LEAK_CHECK", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert_eq!(field(&out, "cinch-leak:", "leaked"), 0, "{out:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

// Coverage programs: each `main` checks its own results and exits with one
// bit per failed check, so a nonzero exit names the check and a leak or a
// debug trap is a reclamation fault rather than a wrong answer.

const BUILDER: &str = include_str!("../prelude/Text/Builder.cinch");

const STRINGS: &str = r#"
module Main where
  import Text.Builder

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  add-items :: Nat -> Nat -> Builder -> Builder
  add-items k n b = if k > n then b else add-items (k + 1) n (builder-add-string (nat-to-string k) b)

  doc :: Nat -> Builder
  doc n = builder-add-nat 100000000000000000000 (builder-add-bytes (string-to-bytes "|") (add-items 1 n (builder-add-line "head" builder-new)))

  check-run :: Nat -> Int
  check-run n = if same (builder-run (doc n)) "head\n12345|100000000000000000000" then 0 else 1

  check-size :: Nat -> Int
  check-size n =
    let b = doc n in
    if builder-size b == 32 && builder-chunks b == n + 3 && bytes-len (builder-bytes b) == 32 then 0 else 2

  parts :: Nat -> List String
  parts n = Cons "ab" (Cons (nat-to-string n) (Cons "cd" (Cons (string-repeat 2 "xy") Nil)))

  check-concat :: Nat -> Int
  check-concat n = if same (string-concat-all (parts n)) "ab5cdxyxy" then 0 else 4

  check-slice :: Nat -> Int
  check-slice n =
    let s = string-concat-all (parts n) in
    if same (string-slice s 1 4) "b5c" && same (string-slice s 5 50) "xyxy" && same (string-slice s 4 2) "" then 0 else 8

  -- "Hi" plus one caller byte, written in place into a fresh buffer; the
  -- result's length is in its type, which proves the copy below.
  hi-bytes :: InRange 0 255 Nat -> refined (bytes-len out == 3) Bytes
  hi-bytes n =
    let b = bytes-new 3 in
    let u0 = bytes-set b 0 72 in
    let u1 = bytes-set b 1 105 in
    let u2 = bytes-set b 2 n in
    b

  check-bytes :: InRange 0 255 Nat -> Int
  check-bytes n =
    let b = hi-bytes n in
    let c = bytes-new 3 in
    let u = bytes-copy-range c b 0 0 3 in
    if bytes-get c 0 == 72 && bytes-compare b c == 0 && bytes-compare b (bytes-from-list (Cons 72 (Cons 105 (Cons n Nil)))) == 0 && bytes-len (sha256-bytes c) == 32 then 0 else 16

  check-utf8 :: InRange 0 255 Nat -> Int
  check-utf8 n = case string-from-bytes (hi-bytes n) of
    None -> 32
    Some s -> if same s "Hi!" && string-length s == 3 then 0 else 32

  main :: Int
  main = check-run 5 + check-size 5 + check-concat 5 + check-slice 5 + check-bytes 33 + check-utf8 33
"#;

#[test]
fn strings_builders_and_bytes_are_freed() {
    expect_leaked_project("strings", STRINGS, &[("Text.Builder.cinch", BUILDER)], 0, 0);
}

const EFFECTS: &str = r#"
module Main where
  import Data.Path
  import Effect.Fs

  effect Tally where
    tally :: String ->{Tally} Nat

  build :: Nat -> List Nat -> List Nat
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  -- Every clause resumes, with the argument string's length.
  check-resumed :: Nat -> Int
  check-resumed n =
    let r = handle tally (string-concat "ab" (nat-to-string n)) + tally "xyz" with (handler Tally where
      return x -> x * 10
      tally s resume k -> k (string-length s))
    in if r == 60 then 0 else 1

  -- The clause drops its continuation, which holds the list.
  check-dropped :: Nat -> Int
  check-dropped n =
    let xs = build n Nil in
    let r = handle list-length xs + tally "q" with (handler Tally where
      return x -> x
      tally s resume k -> 7)
    in if r == 7 then 0 else 2

  check-fs :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-fs p = case fs-write p (string-to-bytes (string-concat-all (Cons "hello " (Cons "file" Nil)))) of
    Err e -> 4
    Ok _ -> case fs-read p of
      Err e -> 8
      Ok b -> case string-from-bytes b of
        None -> 16
        Some s -> if same s "hello file" then 0 else 32

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let p = path-join (path-from-string "__DIR__") (path-from-string "out.txt") in
    case ioWrite (string-to-bytes (string-concat "cinch rc " "console line\n")) of
      _ -> check-resumed 5 + check-dropped 50 + check-fs p
"#;

#[test]
fn console_file_io_and_handled_effects_are_freed() {
    expect_leaked("effects", EFFECTS, 0, 0);
}

const RESULTS: &str = r#"
module Main where
  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  and-then :: forall e a b. Result e a -> (a -> Result e b) -> Result e b
  and-then r k = case r of
    Err e -> Err e
    Ok v -> k v

  opt-then :: forall a b. Option a -> (a -> Option b) -> Option b
  opt-then o k = case o of
    None -> None
    Some v -> k v

  digit :: Nat -> Result String Nat
  digit k = if k < 10 then Ok k else Err (string-concat "bad digit " (nat-to-string k))

  digits :: List Nat -> Result String (List Nat)
  digits xs = case xs of
    Nil -> Ok Nil
    Cons h t -> and-then (digit h) (\d -> and-then (digits t) (\ds -> Ok (Cons d ds)))

  non-empty :: List Nat -> Result String (List Nat)
  non-empty xs = case xs of
    Nil -> Err "empty"
    Cons h t -> Ok xs

  total :: List Nat -> Nat
  total xs = case xs of
    Nil -> 0
    Cons h t -> h + total t

  pipeline :: List Nat -> Result String Nat
  pipeline xs = and-then (digits xs) (\ds -> and-then (non-empty ds) (\ne -> Ok (total ne)))

  check-ok :: Nat -> Int
  check-ok n = case pipeline (Cons 1 (Cons 2 (Cons n Nil))) of
    Err e -> 1
    Ok v -> if v == n + 3 then 0 else 1

  check-err :: Nat -> Int
  check-err n = case pipeline (Cons 1 (Cons n (Cons 3 Nil))) of
    Err e -> if same e "bad digit 12" then 0 else 2
    Ok v -> 2

  check-empty :: Nat -> Int
  check-empty n = case map-err (\e -> string-concat e "!") (pipeline Nil) of
    Err e -> if same e "empty!" then 0 else 4
    Ok v -> 4

  -- The error payloads here are built and discarded unread.
  check-discard :: Nat -> Int
  check-discard n =
    let bad = is-err (pipeline (Cons n Nil)) in
    let gone = result-to-option (pipeline (Cons 1 (Cons n Nil))) in
    if bad && is-none gone && from-result 9 (pipeline (Cons n Nil)) == 9 then 0 else 8

  names :: Nat -> List String
  names n = Cons "alpha" (Cons (nat-to-string n) (Cons "gamma" Nil))

  doubled-second :: List String -> Option String
  doubled-second xs = opt-then (tail xs) (\t -> opt-then (head t) (\s -> opt-then (string-char-at s 0) (\c -> Some (string-concat s s))))

  check-option :: Nat -> Int
  check-option n = case doubled-second (names n) of
    None -> 16
    Some s -> if same s "4242" then 0 else 16

  check-option-none :: Nat -> Int
  check-option-none n =
    let a = doubled-second (Cons "only" Nil) in
    let b = doubled-second (Cons "x" (Cons "" Nil)) in
    let c = map-option (\s -> string-length s) (doubled-second (names n)) in
    if is-none a && is-none b && from-option 0 c == 4 then 0 else 32

  main :: Int
  main = check-ok 4 + check-err 12 + check-empty 0 + check-discard 77 + check-option 42 + check-option-none 42
"#;

#[test]
fn result_and_option_chains_are_freed() {
    expect_leaked("results", RESULTS, 0, 0);
}

/// Units whose `Raw` parameter is unused, with `string-concat` unfolded
/// into the body: the unfolded binders must get ids past every parameter.
/// A binder that reused the unused parameter's id took over its class,
/// so the parameter was dropped as an object (a segfault).
const RAW_PARAMS: &str = r#"
module Main where
  feed :: forall a b. a -> (a -> Option b) -> Option b
  feed x k = k x

  len-of :: Option String -> Nat
  len-of o = case o of
    None -> 0
    Some s -> string-length s

  on-char :: String -> Nat
  on-char s = len-of (feed 'q' (\c -> Some (string-concat s s)))

  on-bool :: String -> Nat
  on-bool s = len-of (feed True (\b -> Some (string-concat s s)))

  on-i64 :: String -> Nat
  on-i64 s = len-of (feed (7 :: I64) (\i -> Some (string-concat s s)))

  on-u8 :: String -> Nat
  on-u8 s = len-of (feed (7 :: U8) (\u -> Some (string-concat s s)))

  on-two :: String -> Nat
  on-two s = len-of (feed True (\b -> feed 'z' (\c -> Some (string-concat s s))))

  -- A top-level unit whose trailing Raw parameters are unused.
  ignore :: String -> Bool -> Char -> Option String
  ignore s b c = Some (string-concat s s)

  main :: Int
  main =
    let a = if on-char "ab" == 4 then 0 else 1 in
    let b = if on-bool "ab" == 4 then 0 else 2 in
    let c = if on-i64 "ab" == 4 then 0 else 4 in
    let d = if on-u8 "ab" == 4 then 0 else 8 in
    let e = if on-two "ab" == 4 then 0 else 16 in
    let f = if len-of (ignore "ab" False 'x') == 4 then 0 else 32 in
    a + b + c + d + e + f
"#;

#[test]
fn unused_raw_parameters_are_never_counted() {
    expect_leaked("rawparams", RAW_PARAMS, 0, 0);
}

const RECORDS: &str = r#"
module Main where
  record Account where
    owner :: String
    history :: List Int
    balance :: Int

  record Pair a where
    left :: a
    right :: a

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  -- No record update syntax: an update rebuilds from a record pattern.
  deposit :: Int -> Account -> Account
  deposit amt a = case a of
    Account { owner = o, history = h, balance = b } -> Account { owner = o, history = Cons amt h, balance = b + amt }

  rename :: String -> Account -> Account
  rename name a = case a of
    Account { history = h, balance = b, owner = o } -> Account { owner = string-concat name o, history = h, balance = b }

  open :: Nat -> Account
  open n = Account { owner = nat-to-string n, history = Nil, balance = 0 }

  check-fields :: Nat -> Int
  check-fields n =
    let a = deposit 100000000000000000000 (deposit 5 (open n)) in
    if a.balance == 100000000000000000005 && length a.history == 2 && same a.owner "7" then 0 else 1

  check-rename :: Nat -> Int
  check-rename n =
    let a = rename "acct-" (deposit 3 (open n)) in
    if same (rename "x" a).owner "xacct-7" && a.balance == 3 then 0 else 2

  swap :: forall a. Pair a -> Pair a
  swap p = case p of
    Pair { left = l, right = r } -> Pair { left = r, right = l }

  check-pair :: Nat -> Int
  check-pair n =
    let p = swap (Pair { left = Cons (1 :: Int) Nil, right = Cons 2 (Cons 3 Nil) }) in
    let q = swap (Pair { left = nat-to-string n, right = "r" }) in
    if length p.left == 2 && length p.right == 1 && same q.left "r" then 0 else 4

  main :: Int
  main = check-fields 7 + check-rename 7 + check-pair 7
"#;

#[test]
fn records_built_projected_and_rebuilt_are_freed() {
    expect_leaked("records", RECORDS, 0, 0);
}

const CLASSES: &str = r#"
module Main where
  import Category.Functor
  import Category.Traversable

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  words :: Nat -> List String
  words n = Cons "a" (Cons (nat-to-string n) (Cons "c" Nil))

  nonblank :: String -> Option String
  nonblank s = if string-length s > 0 then Some (string-concat s s) else None

  check-map-list :: Nat -> Int
  check-map-list n = if same (string-concat-all (map (\s -> string-concat s "!") (words n))) "a!7!c!" then 0 else 1

  check-map-option :: Nat -> Int
  check-map-option n = case map (\xs -> Cons n xs) (Some (Cons 1 (Cons 2 Nil))) of
    None -> 2
    Some ys -> if length ys == 3 then 0 else 2

  check-traverse-some :: Nat -> Int
  check-traverse-some n = case (traverse nonblank (words n) :: Option (List String)) of
    None -> 4
    Some ys -> if same (string-concat-all ys) "aa77cc" then 0 else 4

  -- The partial results built before the blank are dropped.
  check-traverse-none :: Nat -> Int
  check-traverse-none n = case (traverse nonblank (Cons (nat-to-string n) (Cons "" (Cons "z" Nil))) :: Option (List String)) of
    None -> 0
    Some ys -> 8

  grow :: List Nat -> Result String (List Nat)
  grow xs = if length xs < 5 then Ok (Cons 9 xs) else Err (string-concat "long " (nat-to-string (list-length xs)))

  check-traverse-option :: Nat -> Int
  check-traverse-option n = case (traverse grow (Some (Cons n Nil)) :: Result String (Option (List Nat))) of
    Err e -> 16
    Ok o -> case o of
      None -> 16
      Some ys -> if length ys == 2 then 0 else 16

  check-traverse-err :: Nat -> Int
  check-traverse-err n = case (traverse grow (Cons Nil (Cons (replicate n 0) Nil)) :: Result String (List (List Nat))) of
    Err e -> if same e "long 6" then 0 else 32
    Ok yss -> 32

  main :: Int
  main = check-map-list 7 + check-map-option 7 + check-traverse-some 7 + check-traverse-none 7 + check-traverse-option 7 + check-traverse-err 6
"#;

#[test]
fn functor_and_traversable_methods_over_heap_elements_are_freed() {
    expect_leaked("classes", CLASSES, 0, 0);
}

const POLY: &str = r#"
module Main where
  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  -- One polymorphic function: it stores its argument into fresh heap
  -- blocks and hands back a copy plus the block holding the others.
  spread :: forall a. a -> a -> (a, List a)
  spread x y = (y, Cons x (Cons y (Cons x Nil)))

  check-bool :: Bool -> Int
  check-bool b = case spread b False of
    (v, vs) -> if v == False && list-length vs == 3 then 0 else 1

  check-i64 :: Nat -> Int
  check-i64 n =
    if n < 9223372036854775807 then
      case spread (nat-to-i64 n) (nat-to-i64 (n + 1)) of
        (v, vs) -> if eq-i64 v (nat-to-i64 (n + 1)) && list-length vs == 3 then 0 else 2
    else 2

  check-list :: Int -> Int
  check-list n = case spread (build n Nil) (build (n + 1) Nil) of
    (v, vs) -> if length v == n + 1 && length vs == 3 then 0 else 4

  check-string :: Nat -> Int
  check-string n = case spread (nat-to-string n) "y" of
    (v, vs) -> if same v "y" && same (string-concat-all vs) "9y9" then 0 else 8

  pow2 :: Nat -> Nat -> Nat
  pow2 k acc = if k == 0 then acc else pow2 (k - 1) (acc * 2)

  check-nat :: Nat -> Int
  check-nat k = case spread (pow2 k 1) (pow2 k 3) of
    (v, vs) -> if v == 3541774862152233910272 && v > 4611686018427387904 && list-length vs == 3 then 0 else 16

  check-int :: Int -> Int
  check-int x = case spread (x * 100000000000) x of
    (v, vs) -> if v == x && sum-list 0 vs == x * 200000000001 then 0 else 32

  main :: Int
  main = check-bool True + check-i64 9 + check-list 20 + check-string 9 + check-nat 70 + check-int 100000000000
"#;

#[test]
fn one_polymorphic_function_at_raw_obj_and_tobj_types_is_freed() {
    expect_leaked("poly", POLY, 0, 0);
}

const LET_POLY: &str = r#"
module Main where
  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  append-l :: forall a. List a -> List a -> List a
  append-l xs ys = case xs of
    Nil -> ys
    Cons h t -> Cons h (append-l t ys)

  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  fst :: forall a. (a, a) -> a
  fst p = case p of
    (a, b) -> a

  snd :: forall a. (a, a) -> a
  snd p = case p of
    (a, b) -> b

  check-twice :: Int -> Int
  check-twice n =
    let twice = \k x -> k x x in
    let xs = twice (\a b -> append-l a b) (build n Nil) in
    let m = twice (\a b -> a + b) n in
    let s = twice (\a b -> string-concat a b) "ab" in
    if length xs == 2 * n && sum-list 0 xs == n * (n + 1) && m == 2 * n && same s "abab" then 0 else 1

  check-pair :: Int -> Int
  check-pair n =
    let dup = \y -> (y, y) in
    let p = dup (n * 1000000000000000000000) in
    let q = dup (build n Nil) in
    if fst p == snd p && length (append-l (fst q) (snd q)) == 2 * n then 0 else 2

  main :: Int
  main = check-twice 30 + check-pair 30
"#;

#[test]
fn let_generalized_local_functions_are_freed() {
    expect_leaked("letpoly", LET_POLY, 0, 0);
}

/// Let-generalized locals used at a raw type and at a counted one: each
/// use runs its own copy of the definition, so the copies count their
/// words by their own representation. Covered: a plain local, a local
/// used inside another (and a generalized local nested in a
/// definition), a recursive `letrec` group (mutual, capturing an outer
/// local), and the components of a destructured tuple.
const LET_POLY_MIXED: &str = r#"
module Main where
  same :: String -> String -> Bool
  same a b = string-compare a b == 0

  check-plain :: Nat -> Int
  check-plain n =
    let ident = \x -> x in
    let dup = \x -> (x, x) in
    let a = case dup (ident True) of
          (p, q) -> if p && q then 0 else 1 in
    let b = case dup (ident (nat-to-string n)) of
          (p, q) -> if same p q then 0 else 1 in
    let c = if ident 3 == 3 && ident 'c' == 'c' then 0 else 1 in
    a + b + c

  check-nested :: Nat -> Int
  check-nested n =
    let f = \x -> (x, x) in
    let g = \y -> f y in
    let h = \z -> (let k = \w -> (w, z) in k z) in
    let a = case g True of
          (p, q) -> if p && q then 0 else 2 in
    let b = case g (nat-to-string n) of
          (p, q) -> if same p q then 0 else 2 in
    let c = case h (string-concat "a" "b") of
          (p, q) -> if same p q then 0 else 2 in
    let d = case h (7 :: I64) of
          (p, q) -> if p == q then 0 else 2 in
    a + b + c + d

  check-letrec :: Nat -> Int
  check-letrec n =
    letrec ev = \k x -> if k == 0 then Nil else Cons x (od (k - 1) x); od = \k x -> if k == 0 then Nil else ev (k - 1) x
    in
    letrec rep = \k x -> if k == 0 then Nil else Cons (pair-with n x) (rep (k - 1) x)
    in
    let a = if length (ev 3 True) == 2 && length (od 4 (nat-to-string n)) == 2 then 0 else 4 in
    let b = if length (rep 3 'q') == 3 && length (rep 2 (Cons n Nil)) == 2 then 0 else 4 in
    a + b

  pair-with :: forall a. Nat -> a -> (Nat, a)
  pair-with n x = (n, x)

  check-destructured :: Nat -> Int
  check-destructured n =
    let (f, g) = (\x -> (x, x), \y -> y) in
    let a = case f True of
          (p, q) -> if p && q then 0 else 8 in
    let b = case f (nat-to-string n) of
          (p, q) -> if same p q then 0 else 8 in
    let c = if g 'z' == 'z' && same (g (nat-to-string n)) "5" then 0 else 8 in
    a + b + c

  main :: Int
  main = check-plain 5 + check-nested 5 + check-letrec 5 + check-destructured 5
"#;

#[test]
fn let_generalized_locals_at_raw_and_counted_types_are_freed() {
    expect_leaked("letpolymixed", LET_POLY_MIXED, 0, 0);
}

const QUOTES: &str = r#"
module Main where
  append-l :: forall a. List a -> List a -> List a
  append-l xs ys = case xs of
    Nil -> ys
    Cons h t -> Cons h (append-l t ys)

  total :: List Nat -> Nat
  total xs = case xs of
    Nil -> 0
    Cons h t -> h + total t

  measure :: List Nat -> String -> Nat
  measure xs s = total xs + string-length s

  -- The quotation captures two code blocks as holes; each `eval`
  -- rebuilds the list and the string they describe.
  run-twice :: Code (List Nat) -> Code String -> Nat
  run-twice c w =
    let q = [| measure (append-l $(c) $(c)) (string-concat $(w) "!") |] in
    eval q + eval q

  joined :: Code (List Nat) -> List Nat
  joined c = eval [| append-l $(c) (Cons 10 $(c)) |]

  check-run :: Nat -> Int
  check-run n = if run-twice [| Cons 1 (Cons 2 (Cons 3 Nil)) |] [| string-concat "ab" "cd" |] == 34 then 0 else 1

  check-joined :: Nat -> Int
  check-joined n =
    let c = [| Cons 4 (Cons 5 Nil) |] in
    if total (joined c) == 28 && total (joined c) == 28 then 0 else 2

  main :: Int
  main = check-run 0 + check-joined 0
"#;

#[test]
fn evaluated_quotations_with_heap_holes_are_freed() {
    expect_leaked("quotes", QUOTES, 0, 0);
}
