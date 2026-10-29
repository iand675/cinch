//! `Effect.Resolv` consumer semantics: the DNS vocabulary over
//! `Effect.Net`'s `net-resolve` lines (the name-resolution half of the
//! networking family design/stdlib-parity-ruby.md names as Ruby's
//! `Resolv`, the piece D363 leaves open: "a DNS home over
//! `net-resolve`'s lines remain").
//!
//! Every program is the real module sources (`prelude/Effect/Net.cinch`,
//! its `Data.IpAddr` address home and its `Text.Scan` text home, plus
//! `prelude/Effect/Resolv.cinch` under test) written beside a `Main` and
//! driven through the CLI pipeline (`cinch::driver::{driver, Mode}`),
//! the shape `runtime_net_prims.rs` uses for the socket slice. The
//! resolution itself is the platform resolver (nothing here touches the
//! public internet: `localhost` is the one name asked about), so the
//! checks pin the SHAPE of the answer and the reachable refusals, never
//! a fixed address list:
//!
//!   1. `localhost` resolves to at least one typed answer, every answer
//!      round-trips through `Data.IpAddr`'s renderer
//!      (`ip-parse (ip-to-string a) == Some a`) and renders nonempty, so
//!      every element is an address and not text;
//!   2. the requested port is carried through every line;
//!   3. the answer list is well formed: the family counts
//!      (`answer-family-count`) sum to the answer count and
//!      `answer-addresses` preserves the count, so no line is dropped or
//!      invented;
//!   4. the platform assumption is stated where it bites: `localhost`
//!      is not required to have an A record, so the 127.0.0.1
//!      cross-check is asserted only WHEN an IPv4 answer is present (it
//!      is on the macOS/Linux hosts these tests run on, where
//!      `localhost` is both `::1` and `127.0.0.1`), and is skipped when
//!      the answer set is IPv6-only;
//!   5. the reachable named failures are exercised: a name that cannot
//!      resolve (a resolution failure, `ResolvIoError` or
//!      `ResolvNotFound`, never `ResolvInvalidArgument`) and a port the
//!      boundary rejects (70000, outside the `u16` range, is
//!      `ResolvInvalidArgument`);
//!   6. `answer-family-count`/`answer-has-family` classify a
//!      hand-built answer list exactly, using `Data.IpAddr`'s
//!      `ip-family` query (deterministic, independent of the resolver).
//!
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and the total stays
//! inside the 8-bit exit status. For the resolve checks the program also
//! WRITES its typed answers, rendered by `Data.IpAddr`, to a file the
//! test reads back, so the observed output (the addresses and the ports
//! the resolver answered with, as this library renders them) is
//! asserted from Rust beside the exit-code bitmask.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The modules under test, written beside `Main.cinch`. `Effect.Net`
/// binds the `cinch.runtime.net*` units; `Data.IpAddr` is the address
/// home; `Text.Scan` is the text home `Effect.Resolv` rides; and
/// `Effect.Resolv` is the DNS vocabulary itself.
const NET: &str = include_str!("../prelude/Effect/Net.cinch");
const IPADDR: &str = include_str!("../prelude/Data/IpAddr.cinch");
const SCAN: &str = include_str!("../prelude/Text/Scan.cinch");
const RESOLV: &str = include_str!("../prelude/Effect/Resolv.cinch");

/// The port every resolution check asks for (a fixed, unbound port: no
/// socket is opened, the port only rides each resolved line).
const PORT: u32 = 4321;

fn fresh_dir(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_resolv_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    dir
}

/// Compile and run `main_src` (with the modules under test) in `dir` on a
/// wide-stack thread: the checker recurses per nested expression, so a
/// project of this size needs more than the default test thread has (the
/// `lib_*`/`runtime_net_prims` harnesses do the same).
fn run_main_in(dir: &Path, main_src: &str) -> i32 {
    for (file, src) in [
        ("Net.cinch", NET),
        ("IpAddr.cinch", IPADDR),
        ("Scan.cinch", SCAN),
        ("Resolv.cinch", RESOLV),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let entry = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .name("cinch-resolv".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None).map_err(|e| e.to_string()))
        .expect("spawn compiler thread")
        .join();
    match compiled {
        Ok(Ok(code)) => code,
        Ok(Err(e)) => panic!("compilation failed: {e}"),
        Err(_) => panic!("the compiler thread panicked"),
    }
}

fn run_main(name: &str, main_src: &str) -> i32 {
    run_main_in(&fresh_dir(name), main_src)
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// A path as a Cinch string literal (the probes splice their scratch
/// directory into the program text).
fn lit(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// An IPv4 address literal in `Data.IpAddr`'s renderer's form: four
/// decimal octets, three digits at most, no leading zero on a
/// multi-digit octet.
fn is_ipv4_literal(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 4
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.len() <= 3
                && p.chars().all(|c| c.is_ascii_digit())
                && (p.len() == 1 || !p.starts_with('0'))
                && p.parse::<u16>().map(|v| v <= 255).unwrap_or(false)
        })
}

/// An IPv6 address literal in `Data.IpAddr`'s renderer's form
/// (RFC 5952): lower-case hex and colons only, no brackets, no zone.
fn is_ipv6_literal(s: &str) -> bool {
    s.contains(':')
        && !s.contains('[')
        && !s.contains(']')
        && !s.contains('%')
        && s.chars().all(|c| c.is_ascii_hexdigit() || c == ':')
        && !s.chars().any(|c| c.is_ascii_uppercase())
}

/// The resolve-and-classify program: resolve `localhost` at `__PORT__`,
/// check the answer shape, and write the answers (rendered by
/// `Data.IpAddr`) as `<addr> <port>` lines to `__OUT__` for the test to
/// read back.
const LOCALHOST_PROGRAM: &str = r#"
module Main where

  import Data.Bool
  import Data.IpAddr
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Effect.Resolv

  answer-count :: List (IpAddr, Nat) -> Nat
  answer-count answers = case answers of
    Nil -> 0
    Cons e rest -> 1 + answer-count rest

  addr-count :: List IpAddr -> Nat
  addr-count xs = case xs of
    Nil -> 0
    Cons y ys -> 1 + addr-count ys

  -- Every answer is an address: it renders nonempty and its rendering
  -- parses back to the same address (the Data.IpAddr round trip).
  all-round-trip :: List (IpAddr, Nat) -> Bool
  all-round-trip answers = case answers of
    Nil -> True
    Cons e rest -> case e of
      (a, _) -> case ip-parse (ip-to-string a) of
        None -> False
        Some b -> (string-byte-length (ip-to-string a) > 0) && (a == b) && all-round-trip rest

  -- The requested port rides every line.
  all-port-is :: Nat -> List (IpAddr, Nat) -> Bool
  all-port-is want answers = case answers of
    Nil -> True
    Cons e rest -> case e of
      (_, p) -> (p == want) && all-port-is want rest

  member-v4 :: IpAddr -> List (IpAddr, Nat) -> Bool
  member-v4 want answers = case answers of
    Nil -> False
    Cons e rest -> case e of
      (a, _) -> case ip-family a of
        Ipv6Family -> member-v4 want rest
        Ipv4Family -> (a == want) || member-v4 want rest

  -- The 127.0.0.1 cross-check, asserted only when an IPv4 answer is
  -- present (localhost is not required to have an A record).
  ipv4-is-loopback :: List (IpAddr, Nat) -> Bool
  ipv4-is-loopback answers = case answer-has-family Ipv4Family answers of
    False -> True
    True -> case ip-parse "127.0.0.1" of
      None -> False
      Some loop -> member-v4 loop answers

  render-answers :: List (IpAddr, Nat) -> String
  render-answers answers = case answers of
    Nil -> ""
    Cons e rest -> case e of
      (a, p) -> string-concat (ip-to-string a)
        (string-concat " "
          (string-concat (nat-to-string p)
            (string-concat "\n" (render-answers rest))))

  checks :: Nat -> List (IpAddr, Nat) -> Int
  checks port answers =
    let c1 = if answer-count answers > 0 then 0 else 1 in
    let c2 = if all-round-trip answers then 0 else 2 in
    let c3 = if all-port-is port answers then 0 else 4 in
    let c4 = if answer-family-count Ipv4Family answers + answer-family-count Ipv6Family answers == answer-count answers then 0 else 8 in
    let c5 = if ipv4-is-loopback answers then 0 else 16 in
    let c6 = if addr-count (answer-addresses answers) == answer-count answers then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case resolve "localhost" __PORT__ of
      Err e -> 1
      Ok answers ->
        let code = checks __PORT__ answers in
        let w = fs-write (path-from-string __OUT__) (string-to-bytes (render-answers answers)) in
        case w of
          Err e -> 64
          Ok _ -> code
"#;

#[test]
fn localhost_resolves_to_typed_addresses_with_carried_ports() {
    // The resolve probe: `localhost` at PORT becomes a nonempty typed
    // list, every element round-trips through the renderer, the port
    // rides every line, the family counts sum to the count, the IPv4
    // answer (when the platform gives one) is 127.0.0.1, and the
    // addresses written to the output file are the library's own
    // rendering.
    let dir = fresh_dir("localhost");
    let out = dir.join("answers.txt");
    let main = LOCALHOST_PROGRAM
        .replace("__PORT__", &PORT.to_string())
        .replace("__OUT__", &lit(&out));
    let code = run_main_in(&dir, &main);
    assert_eq!(code, 0, "localhost: check bitmask {code}");

    let content = std::fs::read_to_string(&out).expect("answers file");
    assert!(
        content.ends_with('\n'),
        "every rendered answer must be newline terminated: {content:?}"
    );
    let lines: Vec<&str> = content.lines().collect();
    assert!(!lines.is_empty(), "no answer was written");
    let mut saw_v4 = false;
    let mut saw_loopback = false;
    for line in &lines {
        assert!(!line.is_empty(), "a blank line was written: {content:?}");
        let fields: Vec<&str> = line.split(' ').collect();
        assert_eq!(
            fields.len(),
            2,
            "answer line is not `<addr> <port>`: {line:?}"
        );
        let (addr, port) = (fields[0], fields[1]);
        assert!(
            is_ipv4_literal(addr) || is_ipv6_literal(addr),
            "answer is not a rendered address: {addr:?}"
        );
        assert_eq!(port, PORT.to_string(), "port not carried: {line:?}");
        if is_ipv4_literal(addr) {
            saw_v4 = true;
            if addr == "127.0.0.1" {
                saw_loopback = true;
            }
        }
    }
    // The platform assumption, stated: on the hosts these tests run on
    // `localhost` has both A and AAAA records, so the strict cross-check
    // applies; when the answer set is IPv6-only it is skipped, because
    // `localhost` is not required to have an A record.
    if saw_v4 {
        assert!(
            saw_loopback,
            "an IPv4 answer was present, so it must be 127.0.0.1: {content:?}"
        );
    }
}

#[test]
fn named_error_paths_for_names_and_ports() {
    // The refusal probe: a name that cannot resolve (RFC 2606's
    // `.invalid` TLD) is a resolution failure and never an argument one;
    // a port outside the `u16` range the boundary takes is
    // `ResolvInvalidArgument`. Both are named values, never a fault.
    let src = r#"
module Main where

  import Data.Bool
  import Data.List
  import Data.Nat
  import Data.Result
  import Data.String
  import Effect.Resolv

  -- The resolution failures the boundary can report for a name: the
  -- platform resolver's failure maps through the shared codes, which is
  -- `ResolvIoError` where the platform reports an uncategorized DNS
  -- failure (macOS/glibc `getaddrinfo`) and `ResolvNotFound` where it
  -- reports a not-found one, so both are accepted.
  is-name-failure :: ResolvError -> Bool
  is-name-failure e = case e of
    ResolvNotFound -> True
    ResolvIoError -> True
    _ -> False

  is-invalid-argument :: ResolvError -> Bool
  is-invalid-argument e = case e of
    ResolvInvalidArgument -> True
    _ -> False

  check-unresolved :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-unresolved u = case resolve "no-such-host.invalid" 80 of
    Err e -> if is-name-failure e then 0 else 1
    Ok _ -> 1

  check-bad-port :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-bad-port u = case resolve "localhost" 70000 of
    Err e -> if is-invalid-argument e then 0 else 2
    Ok _ -> 2

  check-unresolved-not-argument :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-unresolved-not-argument u = case resolve "no-such-host.invalid" 80 of
    Err e -> if is-invalid-argument e then 4 else 0
    Ok _ -> 0

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    check-unresolved u + check-bad-port u + check-unresolved-not-argument u
"#;
    expect_ok("named-errors", src);
}

#[test]
fn answer_classification_uses_the_ipaddr_family_query() {
    // The classification probe: a hand-built answer list (two IPv4, one
    // IPv6) classified exactly through `answer-family-count`/
    // `answer-has-family` and `Data.IpAddr`'s `ip-family`, deterministic
    // and independent of the resolver.
    let src = r#"
module Main where

  import Data.IpAddr
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Resolv

  pair :: String -> Nat -> Option (IpAddr, Nat)
  pair s p = case ip-parse s of
    None -> None
    Some a -> Some (a, p)

  sample :: Option (List (IpAddr, Nat))
  sample = case pair "127.0.0.1" 80 of
    None -> None
    Some x1 -> case pair "::1" 80 of
      None -> None
      Some x2 -> case pair "10.0.0.1" 80 of
        None -> None
        Some x3 -> Some (Cons x1 (Cons x2 (Cons x3 Nil)))

  addr-count :: List IpAddr -> Nat
  addr-count xs = case xs of
    Nil -> 0
    Cons y ys -> 1 + addr-count ys

  -- The expectations as `Nat` values (an annotated binding keeps the
  -- comparison's numeric type unambiguous).
  expected-v4 :: Nat
  expected-v4 = 2

  expected-v6 :: Nat
  expected-v6 = 1

  expected-total :: Nat
  expected-total = 3

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case sample of
      None -> 255
      Some answers ->
        let c1 = if answer-family-count Ipv4Family answers == expected-v4 then 0 else 1 in
        let c2 = if answer-family-count Ipv6Family answers == expected-v6 then 0 else 2 in
        let c3 = if answer-has-family Ipv4Family answers then 0 else 4 in
        let c4 = if answer-has-family Ipv6Family answers then 0 else 8 in
        let c5 = if addr-count (answer-addresses answers) == expected-total then 0 else 16 in
        c1 + c2 + c3 + c4 + c5
"#;
    expect_ok("classification", src);
}
