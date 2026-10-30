//! Text.Uri and Data.IpAddr consumer semantics: RFC 3986 parsing,
//! printing, normalization and reference resolution (the section 5.4
//! examples), percent-encoding round trips, and the IPv4/IPv6 parse,
//! render, byte and order behavior.
//!
//! Each program is the real module sources (compiler/prelude/Text/Uri.cinch
//! and compiler/prelude/Data/IpAddr.cinch) written beside a Main module and
//! driven through the CLI pipeline, the shape lib_foldable.rs uses.
//!
//! Each check contributes one bit (check k contributes 2^(k-1)); the
//! program sums them, so a nonzero exit code is a bitmask naming the
//! failed checks. The RFC 3986 section 5.4 examples are split across
//! several tests because the process exit status carries eight bits.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const IPADDR: &str = include_str!("../prelude/Data/IpAddr.cinch");
const URI: &str = include_str!("../prelude/Text/Uri.cinch");

const HEADER: &str = "module Main where\n  import Text.Uri\n  import Data.IpAddr\n\n  resolve-eq :: Uri -> String -> String -> Bool\n  resolve-eq b r e = case uri-parse r of\n    Err _ -> False\n    Ok x -> uri-to-string (uri-resolve b x) == e\n\n  parse-print-eq :: String -> Bool\n  parse-print-eq s = case uri-parse s of\n    Err _ -> False\n    Ok u -> uri-to-string u == s\n\n  parse-error-is :: String -> String -> Bool\n  parse-error-is s want = case uri-parse s of\n    Err e -> render-uri-error e == want\n    Ok _ -> False\n\n  scheme-eq :: String -> String -> Bool\n  scheme-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-scheme u of\n      Some x -> x == want\n      None -> False\n\n  userinfo-eq :: String -> String -> Bool\n  userinfo-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-userinfo u of\n      Some x -> x == want\n      None -> False\n\n  host-eq :: String -> String -> Bool\n  host-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-host u of\n      Some x -> x == want\n      None -> False\n\n  port-eq :: String -> Nat -> Bool\n  port-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-port u of\n      Some p -> p == want\n      None -> False\n\n  path-eq :: String -> String -> Bool\n  path-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> uri-path u == want\n\n  query-eq :: String -> String -> Bool\n  query-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-query u of\n      Some q -> q == want\n      None -> False\n\n  fragment-eq :: String -> String -> Bool\n  fragment-eq s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-fragment u of\n      Some f -> f == want\n      None -> False\n\n  normalize-eq :: String -> String -> Bool\n  normalize-eq input want = case uri-parse input of\n    Err _ -> False\n    Ok u -> uri-to-string (uri-normalize u) == want\n\n  host-ip-is :: String -> String -> Bool\n  host-ip-is s want = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-host-ip u of\n      Some a -> ip-to-string a == want\n      None -> False\n\n  host-ip-none :: String -> Bool\n  host-ip-none s = case uri-parse s of\n    Err _ -> False\n    Ok u -> case uri-host-ip u of\n      Some _ -> False\n      None -> True\n\n  decode-eq :: String -> String -> Bool\n  decode-eq input want = case uri-decode input of\n    Err _ -> False\n    Ok s -> s == want\n\n  decode-roundtrip :: String -> Bool\n  decode-roundtrip s = case uri-decode (uri-encode s) of\n    Err _ -> False\n    Ok back -> back == s\n\n  decode-error-is :: String -> String -> Bool\n  decode-error-is input want = case uri-decode input of\n    Err e -> render-uri-error e == want\n    Ok _ -> False\n\n  encode-eq :: String -> String -> Bool\n  encode-eq input want = uri-encode input == want\n\n  ipv4-round-trip :: String -> String -> Bool\n  ipv4-round-trip s want = case ipv4-parse s of\n    None -> False\n    Some a -> ipv4-to-string a == want\n\n  ipv4-none :: String -> Bool\n  ipv4-none s = case ipv4-parse s of\n    None -> True\n    Some _ -> False\n\n  ipv4-bytes-len :: String -> Nat -> Bool\n  ipv4-bytes-len s want = case ipv4-parse s of\n    None -> False\n    Some a -> bytes-len (ipv4-bytes a) == want\n\n  ipv4-byte-at :: String -> Nat -> Nat -> Bool\n  ipv4-byte-at s i want = case ipv4-parse s of\n    None -> False\n    Some a -> if i < bytes-len (ipv4-bytes a) then bytes-get (ipv4-bytes a) i == want else False\n\n  octets-str :: Nat -> Nat -> Nat -> Nat -> String -> Bool\n  octets-str a b c d want = case ipv4-from-octets a b c d of\n    None -> False\n    Some v -> ipv4-to-string v == want\n\n  octets-none :: Nat -> Nat -> Nat -> Nat -> Bool\n  octets-none a b c d = case ipv4-from-octets a b c d of\n    None -> True\n    Some _ -> False\n\n  ipv6-canon :: String -> String -> Bool\n  ipv6-canon input want = case ipv6-parse input of\n    None -> False\n    Some a -> ipv6-to-string a == want\n\n  ipv6-none :: String -> Bool\n  ipv6-none s = case ipv6-parse s of\n    None -> True\n    Some _ -> False\n\n  ipv6-same :: String -> String -> Bool\n  ipv6-same a b = case ipv6-parse a of\n    None -> False\n    Some x -> case ipv6-parse b of\n      None -> False\n      Some y -> x == y\n\n  ipv6-bytes-len :: String -> Nat -> Bool\n  ipv6-bytes-len s want = case ipv6-parse s of\n    None -> False\n    Some a -> bytes-len (ipv6-bytes a) == want\n\n  ipv6-byte-at :: String -> Nat -> Nat -> Bool\n  ipv6-byte-at s i want = case ipv6-parse s of\n    None -> False\n    Some a -> if i < bytes-len (ipv6-bytes a) then bytes-get (ipv6-bytes a) i == want else False\n\n  groups-str :: List Nat -> String -> Bool\n  groups-str gs want = case ipv6-from-groups gs of\n    None -> False\n    Some v -> ipv6-to-string v == want\n\n  groups-none :: List Nat -> Bool\n  groups-none gs = case ipv6-from-groups gs of\n    None -> True\n    Some _ -> False\n\n  ip-family-v4 :: String -> Bool\n  ip-family-v4 s = case ip-parse s of\n    None -> False\n    Some a -> case ip-family a of\n      Ipv4Family -> True\n      Ipv6Family -> False\n\n  ip-family-v6 :: String -> Bool\n  ip-family-v6 s = case ip-parse s of\n    None -> False\n    Some a -> case ip-family a of\n      Ipv4Family -> False\n      Ipv6Family -> True\n\n  ip-round-trip :: String -> String -> Bool\n  ip-round-trip s want = case ip-parse s of\n    None -> False\n    Some a -> ip-to-string a == want\n\n  ip-none :: String -> Bool\n  ip-none s = case ip-parse s of\n    None -> True\n    Some _ -> False\n\n  ipv4-lt :: String -> String -> Bool\n  ipv4-lt a b = case ipv4-parse a of\n    None -> False\n    Some x -> case ipv4-parse b of\n      None -> False\n      Some y -> x < y\n\n  ipv6-lt :: String -> String -> Bool\n  ipv6-lt a b = case ipv6-parse a of\n    None -> False\n    Some x -> case ipv6-parse b of\n      None -> False\n      Some y -> x < y\n\n  ip-eq-check :: String -> String -> Bool\n  ip-eq-check a b = case ip-parse a of\n    None -> False\n    Some x -> case ip-parse b of\n      None -> False\n      Some y -> x == y\n";

fn cons_nat(ns: &[u32]) -> String {
    let mut s = String::from("Nil");
    for n in ns.iter().rev() {
        s = format!("Cons {n} ({s})");
    }
    s
}

fn s1(pred: &str, a: &str) -> String {
    format!("{pred} {a:?}")
}

fn s2(pred: &str, a: &str, b: &str) -> String {
    format!("{pred} {a:?} {b:?}")
}

fn sn(pred: &str, a: &str, n: u32) -> String {
    format!("{pred} {a:?} {n}")
}

fn snn(pred: &str, a: &str, n: u32, m: u32) -> String {
    format!("{pred} {a:?} {n} {m}")
}

fn groups1(pred: &str, ns: &[u32], want: &str) -> String {
    format!("{pred} ({}) {want:?}", cons_nat(ns))
}

fn octets1(a: u32, b: u32, c: u32, d: u32, want: &str) -> String {
    format!("octets-str {a} {b} {c} {d} {want:?}")
}

fn octets0(a: u32, b: u32, c: u32, d: u32) -> String {
    format!("octets-none {a} {b} {c} {d}")
}

fn groups0(pred: &str, ns: &[u32]) -> String {
    format!("{pred} ({})", cons_nat(ns))
}

fn neg(inner: String) -> String {
    format!("not ({inner})")
}

fn resolution_program(base: &str, pairs: &[(&str, &str)]) -> String {
    assert!(!pairs.is_empty() && pairs.len() <= 8);
    let mut s = String::from("  checks :: Uri -> Int");
    s.push_str("\n  checks b =\n");
    for (i, (r, e)) in pairs.iter().enumerate() {
        s.push_str(&format!(
            "    let c{} = if resolve-eq b {r:?} {e:?} then 0 else {} in\n",
            i + 1,
            1u32 << i
        ));
    }
    let sum: Vec<String> = (1..=pairs.len()).map(|i| format!("c{i}")).collect();
    s.push_str(&format!("    {}\n", sum.join(" + ")));
    s.push_str("\n  main :: Int\n");
    s.push_str(&format!(
        "  main = case uri-parse {base:?} of\n    Err _ -> 255\n    Ok b -> checks b\n"
    ));
    s
}

fn checks_program(exprs: &[String]) -> String {
    assert!(!exprs.is_empty() && exprs.len() <= 8);
    let mut s = String::from("  checks :: Int\n  checks =\n");
    for (i, e) in exprs.iter().enumerate() {
        s.push_str(&format!("    let c{} = if {} then 0 else {} in\n", i + 1, e, 1u32 << i));
    }
    let sum: Vec<String> = (1..=exprs.len()).map(|i| format!("c{i}")).collect();
    s.push_str(&format!("    {}\n", sum.join(" + ")));
    s.push_str("\n  main :: Int\n  main = checks\n");
    s
}

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_uritest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("IpAddr.cinch", IPADDR),
        ("Uri.cinch", URI),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let entry = dir.join("Main.cinch");
    // Compiling the embedded prelude plus these modules recurses deeply in
    // the compiler, so the driver runs on a thread with a large stack (the
    // harness default thread stack is too small for it).
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None).map_err(|e| e.to_string()))
        .expect("spawn compile thread");
    match handle.join() {
        Ok(Ok(code)) => code,
        Ok(Err(e)) => panic!("{name}: compilation failed: {e}"),
        Err(_) => panic!("{name}: compile thread panicked"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let src = format!("{HEADER}\n{body}");
    let code = run_main(name, &src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn rfc3986_normal_resolution_part1() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("g:h", "g:h"), ("g", "http://a/b/c/g"), ("./g", "http://a/b/c/g"), ("g/", "http://a/b/c/g/"), ("/g", "http://a/g"), ("//g", "http://g"), ("?y", "http://a/b/c/d;p?y"), ("g?y", "http://a/b/c/g?y")]);
    expect_ok("uri-normal-1", &body);
}

#[test]
fn rfc3986_normal_resolution_part2() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("#s", "http://a/b/c/d;p?q#s"), ("g#s", "http://a/b/c/g#s"), ("g?y#s", "http://a/b/c/g?y#s"), (";x", "http://a/b/c/;x"), ("g;x", "http://a/b/c/g;x"), ("g;x?y#s", "http://a/b/c/g;x?y#s"), ("", "http://a/b/c/d;p?q"), (".", "http://a/b/c/")]);
    expect_ok("uri-normal-2", &body);
}

#[test]
fn rfc3986_normal_resolution_part3() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("./", "http://a/b/c/"), ("..", "http://a/b/"), ("../", "http://a/b/"), ("../g", "http://a/b/g"), ("../..", "http://a/"), ("../../", "http://a/"), ("../../g", "http://a/g")]);
    expect_ok("uri-normal-3", &body);
}

#[test]
fn rfc3986_abnormal_resolution_part1() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("../../../g", "http://a/g"), ("../../../../g", "http://a/g"), ("/./g", "http://a/g"), ("/../g", "http://a/g"), ("g.", "http://a/b/c/g."), (".g", "http://a/b/c/.g"), ("g..", "http://a/b/c/g.."), ("..g", "http://a/b/c/..g")]);
    expect_ok("uri-abnormal-1", &body);
}

#[test]
fn rfc3986_abnormal_resolution_part2() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("./../g", "http://a/b/g"), ("./g/.", "http://a/b/c/g/"), ("g/./h", "http://a/b/c/g/h"), ("g/../h", "http://a/b/c/h"), ("g;x=1/./y", "http://a/b/c/g;x=1/y"), ("g;x=1/../y", "http://a/b/c/y"), ("g?y/./x", "http://a/b/c/g?y/./x"), ("g?y/../x", "http://a/b/c/g?y/../x")]);
    expect_ok("uri-abnormal-2", &body);
}

#[test]
fn rfc3986_abnormal_resolution_part3() {
    let body = resolution_program("http://a/b/c/d;p?q", &[("g#s/./x", "http://a/b/c/g#s/./x"), ("g#s/../x", "http://a/b/c/g#s/../x"), ("http:g", "http:g")]);
    expect_ok("uri-abnormal-3", &body);
}

#[test]
fn percent_encode_decode_round_trips() {
    let exprs: Vec<String> = vec![
        s1("decode-roundtrip", ""),
        s1("decode-roundtrip", "AZaz09-._~"),
        s1("decode-roundtrip", "hello world"),
        s1("decode-roundtrip", "a/b?c=d&e#f"),
        s1("decode-roundtrip", "100%"),
        s1("decode-roundtrip", "café ✓"),
        s2("decode-eq", "%C3%A9", "é"),
        s2("encode-eq", "AZaz09-._~", "AZaz09-._~"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-percent", &body);
}

#[test]
fn percent_decode_rejections() {
    let exprs: Vec<String> = vec![
        s2("decode-error-is", "%4", "malformed percent escape"),
        s2("decode-error-is", "%zz", "malformed percent escape"),
        s2("decode-error-is", "%", "malformed percent escape"),
        s2("decode-error-is", "%C3%28", "decoded bytes are not valid UTF-8"),
        s2("decode-eq", "%41", "A"),
        s2("decode-eq", "%20%21", " !"),
        s2("decode-eq", "a%2Fb", "a/b"),
        s2("encode-eq", "/", "%2F"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-percent-err", &body);
}

#[test]
fn uri_parse_print_round_trips() {
    let exprs: Vec<String> = vec![
        s1("parse-print-eq", "http://user@[::1]:8080/p/a?q#f"),
        s1("parse-print-eq", "mailto:user@example.com"),
        s1("parse-print-eq", "file:///etc/hosts"),
        s1("parse-print-eq", "//example.com"),
        s1("parse-print-eq", "a/b/c"),
        s1("parse-print-eq", ""),
        s1("parse-print-eq", "#frag"),
        s1("parse-print-eq", "?q"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-roundtrip", &body);
}

#[test]
fn uri_components() {
    let exprs: Vec<String> = vec![
        s2("scheme-eq", "http://example.com/x", "http"),
        s2("userinfo-eq", "http://alice@example.com/x", "alice"),
        s2("host-eq", "http://example.com:8080/x", "example.com"),
        sn("port-eq", "http://example.com:8080/x", 8080),
        s2("path-eq", "http://example.com/a/b", "/a/b"),
        s2("query-eq", "http://example.com/a?k=v", "k=v"),
        s2("fragment-eq", "http://example.com/a#sec", "sec"),
        s2("host-eq", "http://[2001:db8::1]/x", "[2001:db8::1]"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-components", &body);
}

#[test]
fn uri_host_classification() {
    let exprs: Vec<String> = vec![
        s2("host-ip-is", "http://[::1]/x", "::1"),
        s2("host-ip-is", "http://[2001:db8::1]:80/x", "2001:db8::1"),
        s2("host-ip-is", "http://192.168.0.1/x", "192.168.0.1"),
        s1("host-ip-none", "http://example.com/x"),
        s1("host-ip-none", "mailto:x@y"),
        s2("host-ip-is", "http://0.0.0.0/x", "0.0.0.0"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-host-ip", &body);
}

#[test]
fn uri_parse_failures() {
    let exprs: Vec<String> = vec![
        s2("parse-error-is", "%", "malformed percent escape"),
        s2("parse-error-is", "#a%b", "malformed percent escape"),
        s2("parse-error-is", "1a:b", "malformed scheme"),
        s2("parse-error-is", ":x", "malformed scheme"),
        s2("parse-error-is", "http://host:port/", "malformed port"),
        s2("parse-error-is", "http://h:80x/", "malformed port"),
        s2("parse-error-is", "http://[::1", "malformed IPv6 host literal"),
        s2("parse-error-is", "http://[1::2::3]/", "malformed IPv6 host literal"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-bad", &body);
}

#[test]
fn uri_normalization() {
    let exprs: Vec<String> = vec![
        s2("normalize-eq", "HTTP://Example.COM/Path", "http://example.com/Path"),
        s2("normalize-eq", "http://example.com:80/a", "http://example.com/a"),
        s2("normalize-eq", "https://example.com:443/a", "https://example.com/a"),
        s2("normalize-eq", "http://example.com:8080/a", "http://example.com:8080/a"),
        s2("normalize-eq", "http://example.com/%7euser", "http://example.com/%7Euser"),
        s2("normalize-eq", "ftp://example.com:21/f", "ftp://example.com/f"),
        s2("normalize-eq", "http://example.com/a%2fb", "http://example.com/a%2Fb"),
        s2("normalize-eq", "http://[2001:DB8::1]/x", "http://[2001:db8::1]/x"),
    ];
    let body = checks_program(&exprs);
    expect_ok("uri-normalize", &body);
}

#[test]
fn ipv6_canonical_compression_part1() {
    let exprs: Vec<String> = vec![
        s2("ipv6-canon", "2001:db8::1", "2001:db8::1"),
        s2("ipv6-canon", "::1", "::1"),
        s2("ipv6-canon", "::", "::"),
        s2("ipv6-canon", "0:0:0:0:0:0:0:0", "::"),
        s2("ipv6-canon", "2001:0db8:0000:0000:0000:0000:0000:0001", "2001:db8::1"),
        s2("ipv6-canon", "1:2:3:4:5:6:7:8", "1:2:3:4:5:6:7:8"),
        s2("ipv6-canon", "1:0:0:2:0:0:0:3", "1:0:0:2::3"),
        s2("ipv6-canon", "2001:db8:0:0:1:0:0:1", "2001:db8::1:0:0:1"),
    ];
    let body = checks_program(&exprs);
    expect_ok("ipv6-canon-1", &body);
}

#[test]
fn ipv6_canonical_compression_part2() {
    let exprs: Vec<String> = vec![
        s2("ipv6-canon", "1::", "1::"),
        s2("ipv6-canon", "2001:db8::", "2001:db8::"),
        s2("ipv6-canon", "FE80::AbCd", "fe80::abcd"),
        s2("ipv6-same", "2001:db8::1", "2001:0db8::0001"),
    ];
    let body = checks_program(&exprs);
    expect_ok("ipv6-canon-2", &body);
}

#[test]
fn ipv6_rejections() {
    let exprs: Vec<String> = vec![
        s1("ipv6-none", "1::2::3"),
        s1("ipv6-none", ":::"),
        s1("ipv6-none", "1:2:3:4:5:6:7:8:9"),
        s1("ipv6-none", "1:2:3"),
        s1("ipv6-none", "12345::"),
        s1("ipv6-none", "1:2:3:4:5:6:7"),
        s1("ipv6-none", "::ffff:1.2.3.4"),
        s1("ipv6-none", "1:2:3::4:5:6:7:8"),
    ];
    let body = checks_program(&exprs);
    expect_ok("ipv6-bad", &body);
}

#[test]
fn ipv4_boundary_octets() {
    let exprs: Vec<String> = vec![
        s2("ipv4-round-trip", "0.0.0.0", "0.0.0.0"),
        s2("ipv4-round-trip", "255.255.255.255", "255.255.255.255"),
        s1("ipv4-none", "256.1.1.1"),
        s1("ipv4-none", "1.1.1"),
        s1("ipv4-none", "1.1.1.1.1"),
        s1("ipv4-none", "01.1.1.1"),
        s1("ipv4-none", "1.1.1.256"),
        s1("ipv4-none", ""),
    ];
    let body = checks_program(&exprs);
    expect_ok("ipv4-boundary", &body);
}

#[test]
fn smart_constructors_enforce_ranges() {
    let exprs: Vec<String> = vec![
        octets1(0, 0, 0, 0, "0.0.0.0"),
        octets1(255, 255, 255, 255, "255.255.255.255"),
        octets0(256, 0, 0, 0),
        octets0(0, 0, 0, 256),
        groups1("groups-str", &[0, 0, 0, 0, 0, 0, 0, 0], "::"),
        groups0("groups-none", &[0, 0, 0, 0, 0, 0, 0]),
        groups0("groups-none", &[70000, 0, 0, 0, 0, 0, 0, 0]),
        groups1("groups-str", &[1, 0, 0, 0, 0, 0, 0, 0], "1::"),
    ];
    let body = checks_program(&exprs);
    expect_ok("ip-smart", &body);
}

#[test]
fn ip_family_bytes_and_order() {
    let exprs: Vec<String> = vec![
        s1("ip-family-v4", "1.2.3.4"),
        s1("ip-family-v6", "::1"),
        sn("ipv4-bytes-len", "1.2.3.4", 4),
        snn("ipv4-byte-at", "1.2.3.4", 0, 1),
        snn("ipv4-byte-at", "1.2.3.4", 3, 4),
        s2("ipv4-lt", "1.0.0.0", "1.0.0.1"),
        neg(s2("ipv4-lt", "1.0.0.1", "1.0.0.0")),
        s2("ip-eq-check", "2001:db8::1", "2001:0db8::0001"),
    ];
    let body = checks_program(&exprs);
    expect_ok("ip-misc", &body);
}

#[test]
fn ipv6_bytes_layout() {
    let exprs: Vec<String> = vec![
        sn("ipv6-bytes-len", "2001:db8::1", 16),
        snn("ipv6-byte-at", "2001:db8::1", 0, 32),
        snn("ipv6-byte-at", "2001:db8::1", 1, 1),
        snn("ipv6-byte-at", "2001:db8::1", 2, 13),
        snn("ipv6-byte-at", "2001:db8::1", 3, 184),
        snn("ipv6-byte-at", "2001:db8::1", 15, 1),
    ];
    let body = checks_program(&exprs);
    expect_ok("ipv6-bytes", &body);
}
