//! `Data.NetIp` integration tests against Go 1.24's `net/netip` reference vectors
//! (`tooling/netip_vectors.go` -> `seed/tests/fixtures/netip-vectors.json`).
mod common;

use serde_json::Value;

/// A Cinch string literal.
fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn cbool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn opt_str(v: &Value) -> String {
    match v.as_str() {
        Some(s) => format!("(Some {})", lit(s)),
        None => "None".to_string(),
    }
}

fn int_lit(n: i64) -> String {
    if n < 0 {
        format!("(0 - {})", -n)
    } else {
        n.to_string()
    }
}

fn ord_lit(c: i64) -> &'static str {
    if c < 0 {
        "LT"
    } else if c > 0 {
        "GT"
    } else {
        "EQ"
    }
}

fn run_netip(tag: &str, main_src: &str) -> i32 {
    let dir = common::project(
        tag,
        &[
            ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
            ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
            ("Main.cinch", main_src),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    code
}

/// Every declaration in `Data.NetIp` has canonical documentation.
#[test]
fn netip_declarations_have_canonical_documentation() {
    common::assert_documented("Data.NetIp", include_str!("../prelude/Data/NetIp.cinch"));
}

/// All 105 `Addr` parse, formatting, representation, stepping, zone, binary, and predicate vectors match Go 1.24.
#[test]
fn addrs_match_go_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/netip-vectors.json")).unwrap();
    let addrs = f["addrs"].as_array().unwrap();
    assert!(addrs.len() < 200);

    let mut rows = Vec::new();
    for r in addrs {
        let input = lit(r["input"].as_str().unwrap());
        if !r["valid"].as_bool().unwrap() {
            rows.push(format!("check-bad-addr {input}"));
        } else {
            let s = lit(r["string"].as_str().unwrap());
            let exp = lit(r["expanded"].as_str().unwrap());
            let is4 = cbool(r["is4"].as_bool().unwrap_or(false));
            let is6 = cbool(r["is6"].as_bool().unwrap_or(false));
            let is4in6 = cbool(r["is4in6"].as_bool().unwrap_or(false));
            let bit_len = r["bit_len"].as_u64().unwrap();
            let zone = lit(r["zone"].as_str().unwrap_or(""));
            let unmap = lit(r["unmap"].as_str().unwrap());
            let as4 = opt_str(&r["as4_hex"]);
            let as16 = lit(r["as16_hex"].as_str().unwrap());
            let as_slice = lit(r["as_slice_hex"].as_str().unwrap());
            let mbin = lit(r["marshal_bin_hex"].as_str().unwrap());
            let next = opt_str(&r["next"]);
            let prev = opt_str(&r["prev"]);
            let p_unspec = cbool(r["is_unspecified"].as_bool().unwrap_or(false));
            let p_loop = cbool(r["is_loopback"].as_bool().unwrap_or(false));
            let p_mc = cbool(r["is_multicast"].as_bool().unwrap_or(false));
            let p_ifmc = cbool(r["is_if_local_mc"].as_bool().unwrap_or(false));
            let p_llmc = cbool(r["is_link_local_mc"].as_bool().unwrap_or(false));
            let p_lluc = cbool(r["is_link_local_uc"].as_bool().unwrap_or(false));
            let p_guc = cbool(r["is_global_unicast"].as_bool().unwrap_or(false));
            let p_priv = cbool(r["is_private"].as_bool().unwrap_or(false));
            let wz_eth0 = lit(r["with_zone_eth0"].as_str().unwrap());
            let wz_empty = lit(r["with_zone_empty"].as_str().unwrap());
            rows.push(format!(
                "check-addr {input} {s} {exp} {is4} {is6} {is4in6} {bit_len} {zone} {unmap} {as4} {as16} {as_slice} {mbin} {next} {prev} {p_unspec} {p_loop} {p_mc} {p_ifmc} {p_llmc} {p_lluc} {p_guc} {p_priv} {wz_eth0} {wz_empty}"
            ));
        }
    }

    let main = format!(
        r#"{COMMON_HELPERS}
  -- | Verify an invalid address string is rejected with a non-empty error message.
  check-bad-addr :: String -> Bool
  check-bad-addr input = case parse-addr input of
    Ok _ -> False
    Err e -> not (netip-error-message e == "")

  -- | Verify every Addr method against the Go reference row.
  check-addr :: String -> String -> String -> Bool -> Bool -> Bool -> Nat -> String -> String -> Option String -> String -> String -> String -> Option String -> Option String -> Bool -> Bool -> Bool -> Bool -> Bool -> Bool -> Bool -> Bool -> String -> String -> Bool
  check-addr input want-str want-exp want4 want6 want4in6 want-bits want-zone want-unmap want-as4 want-as16 want-slice want-bin want-next want-prev want-unspec want-loop want-mc want-ifmc want-llmc want-lluc want-guc want-priv want-wz-eth0 want-wz-empty =
    case parse-addr input of
      Err _ -> False
      Ok ip ->
        let ok-str = addr-to-string ip == want-str && show ip == want-str in
        let ok-exp = addr-string-expanded ip == want-exp in
        let ok-fam = addr-is4 ip == want4 && addr-is6 ip == want6 && addr-is4-in6 ip == want4in6 && addr-bit-len ip == want-bits in
        let ok-zone = addr-zone ip == want-zone && addr-to-string (addr-with-zone ip "eth0") == want-wz-eth0 && addr-to-string (addr-with-zone ip "") == want-wz-empty in
        let ok-unmap = addr-to-string (addr-unmap ip) == want-unmap in
        let ok-as4 = (case addr-as4 ip of
          None -> (case want-as4 of None -> True; Some _ -> False)
          Some b -> (case want-as4 of None -> False; Some h -> bytes-to-hex b == h)) in
        let ok-bytes = bytes-to-hex (addr-as16 ip) == want-as16 && bytes-to-hex (addr-as-slice ip) == want-slice in
        let bin = addr-marshal-binary ip in
        let ok-bin = bytes-to-hex bin == want-bin && (case addr-unmarshal-binary bin of Ok ip2 -> ip2 == ip && hash ip2 == hash ip; Err _ -> False) in
        let ok-slice-rt = if want-zone == "" then (case addr-from-slice (addr-as-slice ip) of Some ip2 -> ip2 == ip; None -> False) else True in
        let ok-next = (case addr-next ip of
          None -> (case want-next of None -> True; Some _ -> False)
          Some n -> (case want-next of None -> False; Some s -> addr-to-string n == s)) in
        let ok-prev = (case addr-prev ip of
          None -> (case want-prev of None -> True; Some _ -> False)
          Some p -> (case want-prev of None -> False; Some s -> addr-to-string p == s)) in
        let ok-preds = (
          addr-is-unspecified ip == want-unspec &&
          addr-is-loopback ip == want-loop &&
          addr-is-multicast ip == want-mc &&
          addr-is-interface-local-multicast ip == want-ifmc &&
          addr-is-link-local-multicast ip == want-llmc &&
          addr-is-link-local-unicast ip == want-lluc &&
          addr-is-global-unicast ip == want-guc &&
          addr-is-private ip == want-priv) in
        let ok-reparse = (case parse-addr (addr-to-string ip) of Ok ip2 -> ip2 == ip; Err _ -> False) in
        ok-str && ok-exp && ok-fam && ok-zone && ok-unmap && ok-as4 && ok-bytes && ok-bin && ok-slice-rt && ok-next && ok-prev && ok-preds && ok-reparse

  -- | Entry point: 200 if all rows pass, else the 0-based index of the first failing row.
  main :: Int
  main = first [{}] 0
"#,
        rows.join(",\n    ")
    );

    assert_eq!(
        run_netip("netip_addrs", &main),
        200,
        "exit status is the first failing addr row; 200 means all {} rows passed",
        rows.len()
    );
}

/// All 26 `Addr.Compare`, 32 `AddrPort`, and 11 `AddrPort.Compare` vectors match Go 1.24.
#[test]
fn comparisons_and_addrports_match_go_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/netip-vectors.json")).unwrap();
    let mut rows = Vec::new();

    for r in f["compare_addr"].as_array().unwrap() {
        rows.push(format!(
            "check-cmp-addr {} {} {}",
            lit(r["a"].as_str().unwrap()),
            lit(r["b"].as_str().unwrap()),
            ord_lit(r["cmp"].as_i64().unwrap())
        ));
    }

    for r in f["addr_ports"].as_array().unwrap() {
        let input = lit(r["input"].as_str().unwrap());
        if !r["valid"].as_bool().unwrap() {
            rows.push(format!("check-bad-ap {input}"));
        } else {
            rows.push(format!(
                "check-ap {input} {} {} {} {}",
                lit(r["string"].as_str().unwrap()),
                lit(r["addr"].as_str().unwrap()),
                r["port"].as_u64().unwrap_or(0),
                lit(r["marshal_bin_hex"].as_str().unwrap())
            ));
        }
    }

    for r in f["compare_addr_port"].as_array().unwrap() {
        rows.push(format!(
            "check-cmp-ap {} {} {}",
            lit(r["a"].as_str().unwrap()),
            lit(r["b"].as_str().unwrap()),
            ord_lit(r["cmp"].as_i64().unwrap())
        ));
    }

    assert!(rows.len() < 200);

    let main = format!(
        r#"{COMMON_HELPERS}
  -- | Compare two Ordering values.
  eq-ord :: Ordering -> Ordering -> Bool
  eq-ord a b = case a of
    LT -> case b of LT -> True; _ -> False
    EQ -> case b of EQ -> True; _ -> False
    GT -> case b of GT -> True; _ -> False

  -- | Verify Addr comparison, Less, Eq, Ord, and Hash consistency.
  check-cmp-addr :: String -> String -> Ordering -> Bool
  check-cmp-addr sa sb want =
    case parse-addr sa of
      Err _ -> False
      Ok a -> case parse-addr sb of
        Err _ -> False
        Ok b ->
          let c = addr-compare a b in
          let ok-c = eq-ord c want in
          let ok-less = addr-less a b == (a < b) && addr-less a b == eq-ord want LT in
          let ok-eq = (a == b) == eq-ord want EQ && (a /= b) == not (eq-ord want EQ) in
          let ok-ord = (a <= b) == not (eq-ord want GT) && (a > b) == eq-ord want GT && (a >= b) == not (eq-ord want LT) in
          let ok-hash = if a == b then hash a == hash b else True in
          ok-c && ok-less && ok-eq && ok-ord && ok-hash

  -- | Verify an invalid AddrPort string is rejected.
  check-bad-ap :: String -> Bool
  check-bad-ap input = case parse-addr-port input of
    Ok _ -> False
    Err e -> not (netip-error-message e == "")

  -- | Verify a valid AddrPort string, fields, formatting, Show, constructor, and binary codec.
  check-ap :: String -> String -> String -> U16 -> String -> Bool
  check-ap input want-str want-addr want-port want-bin =
    case parse-addr-port input of
      Err _ -> False
      Ok ap ->
        let ok-str = addr-port-to-string ap == want-str && show ap == want-str in
        let ok-fields = addr-to-string (addr-port-addr ap) == want-addr && addr-port-port ap == want-port in
        let ok-from = addr-port-from (addr-port-addr ap) (addr-port-port ap) == ap in
        let bin = addr-port-marshal-binary ap in
        let ok-bin = bytes-to-hex bin == want-bin && (case addr-port-unmarshal-binary bin of Ok ap2 -> ap2 == ap && hash ap2 == hash ap; Err _ -> False) in
        let ok-reparse = (case parse-addr-port (addr-port-to-string ap) of Ok ap2 -> ap2 == ap; Err _ -> False) in
        ok-str && ok-fields && ok-from && ok-bin && ok-reparse

  -- | Verify AddrPort comparison, Eq, Ord, and Hash consistency.
  check-cmp-ap :: String -> String -> Ordering -> Bool
  check-cmp-ap sa sb want =
    case parse-addr-port sa of
      Err _ -> False
      Ok a -> case parse-addr-port sb of
        Err _ -> False
        Ok b ->
          let c = addr-port-compare a b in
          let ok-c = eq-ord c want in
          let ok-eq = (a == b) == eq-ord want EQ && (a /= b) == not (eq-ord want EQ) in
          let ok-ord = (a < b) == eq-ord want LT && (a <= b) == not (eq-ord want GT) && (a > b) == eq-ord want GT && (a >= b) == not (eq-ord want LT) in
          let ok-hash = if a == b then hash a == hash b else True in
          ok-c && ok-eq && ok-ord && ok-hash

  -- | Entry point: 200 if all rows pass, else the 0-based index of the first failing row.
  main :: Int
  main = first [{}] 0
"#,
        rows.join(",\n    ")
    );

    assert_eq!(
        run_netip("netip_cmp_ap", &main),
        200,
        "exit status is the first failing compare/AddrPort row; 200 means all {} rows passed",
        rows.len()
    );
}

/// All 30 `Prefix`, 16 `Addr.Prefix`, 23 `Contains`, and 15 `Overlaps` vectors match Go 1.24,
/// plus `Prefix` ordering/hashing, well-known constants, direct constructors, binary errors, and `Data.IpAddr` interop.
#[test]
fn prefixes_and_interop_match_go_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/netip-vectors.json")).unwrap();
    let mut rows = Vec::new();

    for r in f["prefixes"].as_array().unwrap() {
        let input = lit(r["input"].as_str().unwrap());
        if !r["valid"].as_bool().unwrap() {
            rows.push(format!("check-bad-prefix {input}"));
        } else {
            rows.push(format!(
                "check-prefix {input} {} {} {} {} {} {}",
                lit(r["string"].as_str().unwrap()),
                lit(r["addr"].as_str().unwrap()),
                r["bits"].as_u64().unwrap_or(0),
                cbool(r["is_single_ip"].as_bool().unwrap_or(false)),
                lit(r["masked"].as_str().unwrap()),
                lit(r["marshal_bin_hex"].as_str().unwrap())
            ));
        }
    }

    for r in f["addr_prefixes"].as_array().unwrap() {
        let addr = lit(r["addr"].as_str().unwrap());
        let bits = int_lit(r["bits"].as_i64().unwrap());
        if !r["valid"].as_bool().unwrap() {
            rows.push(format!("check-bad-addr-prefix {addr} {bits}"));
        } else {
            rows.push(format!(
                "check-addr-prefix {addr} {bits} {}",
                lit(r["result"].as_str().unwrap())
            ));
        }
    }

    for r in f["contains"].as_array().unwrap() {
        rows.push(format!(
            "check-contains {} {} {}",
            lit(r["prefix"].as_str().unwrap()),
            lit(r["addr"].as_str().unwrap()),
            cbool(r["contains"].as_bool().unwrap())
        ));
    }

    for r in f["overlaps"].as_array().unwrap() {
        rows.push(format!(
            "check-overlaps {} {} {}",
            lit(r["a"].as_str().unwrap()),
            lit(r["b"].as_str().unwrap()),
            cbool(r["overlaps"].as_bool().unwrap())
        ));
    }

    // Extra unit checks for constants, constructors, prefix ordering, binary errors, and Data.IpAddr interop.
    rows.push("check-constants".to_string());
    rows.push("check-constructors".to_string());
    rows.push("check-prefix-order".to_string());
    rows.push("check-ipaddr-interop".to_string());
    rows.push("check-binary-errors".to_string());

    assert!(rows.len() < 200);

    let main = format!(
        r#"{COMMON_HELPERS}
  -- | Verify an invalid CIDR prefix string is rejected.
  check-bad-prefix :: String -> Bool
  check-bad-prefix input = case parse-prefix input of
    Ok _ -> False
    Err e -> not (netip-error-message e == "")

  -- | Verify a valid CIDR prefix against Go reference values.
  check-prefix :: String -> String -> String -> Nat -> Bool -> String -> String -> Bool
  check-prefix input want-str want-addr want-bits want-single want-masked want-bin =
    case parse-prefix input of
      Err _ -> False
      Ok p ->
        let ok-str = prefix-to-string p == want-str && show p == want-str in
        let ok-fields = addr-to-string (prefix-addr p) == want-addr && prefix-bits p == want-bits && prefix-is-single-ip p == want-single in
        let ok-masked = prefix-to-string (prefix-masked p) == want-masked in
        let ok-from = (case prefix-from (prefix-addr p) (nat-to-int (prefix-bits p)) of Ok p2 -> p2 == p && hash p2 == hash p; Err _ -> False) in
        let bin = prefix-marshal-binary p in
        let ok-bin = bytes-to-hex bin == want-bin && (case prefix-unmarshal-binary bin of Ok p2 -> p2 == p; Err _ -> False) in
        let ok-reparse = (case parse-prefix (prefix-to-string p) of Ok p2 -> p2 == p; Err _ -> False) in
        ok-str && ok-fields && ok-masked && ok-from && ok-bin && ok-reparse

  -- | Verify `addr-prefix` succeeds with the expected canonical prefix string.
  check-addr-prefix :: String -> Int -> String -> Bool
  check-addr-prefix addr-str bits want =
    case parse-addr addr-str of
      Err _ -> False
      Ok ip -> case addr-prefix ip bits of
        Err _ -> False
        Ok p -> prefix-to-string p == want

  -- | Verify `addr-prefix` and `prefix-from` reject out-of-range prefix lengths.
  check-bad-addr-prefix :: String -> Int -> Bool
  check-bad-addr-prefix addr-str bits =
    case parse-addr addr-str of
      Err _ -> False
      Ok ip ->
        let bad1 = (case addr-prefix ip bits of Ok _ -> False; Err e -> not (netip-error-message e == "")) in
        let bad2 = (case prefix-from ip bits of Ok _ -> False; Err e -> not (netip-error-message e == "")) in
        bad1 && bad2

  -- | Verify `prefix-contains`.
  check-contains :: String -> String -> Bool -> Bool
  check-contains p-str a-str want =
    case parse-prefix p-str of
      Err _ -> False
      Ok p -> case parse-addr a-str of
        Err _ -> False
        Ok a -> prefix-contains p a == want

  -- | Verify `prefix-overlaps` (and symmetry).
  check-overlaps :: String -> String -> Bool -> Bool
  check-overlaps a-str b-str want =
    case parse-prefix a-str of
      Err _ -> False
      Ok a -> case parse-prefix b-str of
        Err _ -> False
        Ok b -> prefix-overlaps a b == want && prefix-overlaps b a == want

  -- | Verify well-known address constants.
  check-constants :: Bool
  check-constants =
    addr-to-string ipv4-unspecified == "0.0.0.0" &&
    addr-to-string ipv6-unspecified == "::" &&
    addr-to-string ipv6-loopback == "::1" &&
    addr-to-string ipv6-link-local-all-nodes == "ff02::1" &&
    addr-to-string ipv6-link-local-all-routers == "ff02::2" &&
    addr-is-unspecified ipv4-unspecified &&
    addr-is-unspecified ipv6-unspecified &&
    addr-is-loopback ipv6-loopback &&
    addr-is-link-local-multicast ipv6-link-local-all-nodes &&
    addr-is-link-local-multicast ipv6-link-local-all-routers

  -- | Verify direct constructors (`addr-from4`, `addr-from16`, `addr-from-slice`, `addr-from-u32`, `addr-from-u128`).
  check-constructors :: Bool
  check-constructors =
    let a4 = addr-from4 192 168 1 42 in
    let ok4 = addr-to-string a4 == "192.168.1.42" && a4 == addr-from-u32 3232235818 in
    let ok16-mapped = (case addr-from16 (addr-as16 a4) of
      Some a6 -> addr-is4-in6 a6 && addr-to-string a6 == "::ffff:192.168.1.42" && addr-unmap a6 == a4
      None -> False) in
    let ok16-bad = (case addr-from16 (bytes-new 4) of None -> True; Some _ -> False) in
    let ok-slice-bad = (case addr-from-slice (bytes-new 5) of None -> True; Some _ -> False) in
    let ok-u128 = addr-from-u128 1 == ipv6-loopback in
    let ok-zone-strip = (case parse-addr "fe80::1%eth0" of
      Err _ -> False
      Ok z -> (case prefix-from z 64 of
        Err _ -> False
        Ok p -> addr-zone (prefix-addr p) == "" && prefix-to-string p == "fe80::1/64")) in
    ok4 && ok16-mapped && ok16-bad && ok-slice-bad && ok-u128 && ok-zone-strip

  -- | Verify `prefix-compare`, `Eq`, `Ord`, and `Hash` laws on `Prefix`.
  check-prefix-order :: Bool
  check-prefix-order =
    case parse-prefix "10.0.0.0/8" of
      Err _ -> False
      Ok p1 -> case parse-prefix "10.0.0.0/24" of
        Err _ -> False
        Ok p2 -> case parse-prefix "10.1.0.0/16" of
          Err _ -> False
          Ok p3 -> case parse-prefix "::/0" of
            Err _ -> False
            Ok p4 ->
              p1 < p2 && p2 < p3 && p3 < p4 &&
              p1 <= p1 && p2 >= p1 && p4 > p3 &&
              p1 /= p2 && p1 == p1 && hash p1 == hash p1 &&
              (case prefix-compare p1 p2 of LT -> True; _ -> False) &&
              (case prefix-compare p2 p1 of GT -> True; _ -> False) &&
              (case prefix-compare p2 p2 of EQ -> True; _ -> False)

  -- | Verify `addr-to-ip-addr` and `addr-from-ip-addr` round-trips and zone refusal.
  check-ipaddr-interop :: Bool
  check-ipaddr-interop =
    let a4 = addr-from4 198 51 100 7 in
    let ok-v4 = (case addr-to-ip-addr a4 of
      Some ip -> ip-to-string ip == "198.51.100.7" && addr-from-ip-addr ip == a4
      None -> False) in
    let ok-v6 = (case parse-addr "2001:db8::42" of
      Err _ -> False
      Ok a6 -> (case addr-to-ip-addr a6 of
        Some ip -> ip-to-string ip == "2001:db8::42" && addr-from-ip-addr ip == a6
        None -> False)) in
    let ok-mapped = (case parse-addr "::ffff:192.0.2.1" of
      Err _ -> False
      Ok m -> (case addr-to-ip-addr m of
        Some ip -> addr-from-ip-addr ip == m
        None -> False)) in
    let ok-zoned = (case parse-addr "fe80::1%eth0" of
      Err _ -> False
      Ok z -> (case addr-to-ip-addr z of
        None -> True
        Some _ -> False)) in
    ok-v4 && ok-v6 && ok-mapped && ok-zoned

  -- | Verify binary decoding errors and `NetIpError` Eq/Show instances.
  check-binary-errors :: Bool
  check-binary-errors =
    let bad-addr-len = (case addr-unmarshal-binary (bytes-new 3) of Err e -> e == e && show e == netip-error-message e; Ok _ -> False) in
    let bad-ap-len = (case addr-port-unmarshal-binary (bytes-new 1) of Err e -> not (show e == ""); Ok _ -> False) in
    let bad-pfx-len = (case prefix-unmarshal-binary (bytes-new 0) of Err e -> not (show e == ""); Ok _ -> False) in
    let bad-pfx-bits = (
      let b = bytes-new 5 in
      let u = bytes-set b 4 33 in
      case prefix-unmarshal-binary b of Err (NetIpInvalidPrefixBits 33 32) -> True; _ -> False) in
    let bad-utf8-zone = (
      let b = bytes-new 17 in
      let u = bytes-set b 16 255 in
      case addr-unmarshal-binary b of Err (NetIpInvalidBinary _) -> True; _ -> False) in
    bad-addr-len && bad-ap-len && bad-pfx-len && bad-pfx-bits && bad-utf8-zone

  -- | Entry point: 200 if all rows pass, else the 0-based index of the first failing row.
  main :: Int
  main = first [{}] 0
"#,
        rows.join(",\n    ")
    );

    assert_eq!(
        run_netip("netip_prefixes", &main),
        200,
        "exit status is the first failing prefix/interop row; 200 means all {} rows passed",
        rows.len()
    );
}

const COMMON_HELPERS: &str = r#"-- | Reference test consumer for Data.NetIp.
module Main where
  import Data.NetIp
  import Data.IpAddr
  import Data.Ordering
  import Data.Bytes
  import Data.String
  import Data.Nat
  import Data.Int
  import Data.Option
  import Data.Result
  import Data.Hash
  import Text.Show (show)

  -- | Convert a nibble in 0..=15 to a lowercase ASCII hex byte.
  hex-char-byte :: Nat -> InRange 0 255 Nat
  hex-char-byte n = if n < 10 then 48 + n else if n < 16 then 87 + n else 48

  -- | Encode a Bytes buffer as lowercase hexadecimal text.
  bytes-to-hex :: Bytes -> String
  bytes-to-hex b =
    let n = bytes-len b in
    let out = bytes-new (n * 2) in
    let u = bytes-to-hex-loop b 0 out in
    case string-from-bytes out of
      Some s -> s
      None -> ""

  -- | Tail-recursive hex encoding loop.
  bytes-to-hex-loop :: Bytes -> Nat -> Bytes -> Unit
  bytes-to-hex-loop b i out =
    if i < bytes-len b then
      let x = bytes-get b i in
      let hi = hex-char-byte (nat-rem (nat-div x 16) 16) in
      let lo = hex-char-byte (nat-rem x 16) in
      let u0 = if i * 2 + 1 < bytes-len out then bytes-set out (i * 2) hi else () in
      let u1 = if i * 2 + 1 < bytes-len out then bytes-set out (i * 2 + 1) lo else () in
      bytes-to-hex-loop b (i + 1) out
    else ()

  -- | Index of the first False element, or 200 if every element is True.
  first :: List Bool -> Int -> Int
  first xs i = case xs of
    Nil -> 200
    Cons x rest -> if x then first rest (i + 1) else i
"#;
