//! PKIX (`Crypto.Pkix`) and X.509 (`Crypto.X509`) tests verified against
//! independent Go 1.24 `crypto/x509/pkix` and `crypto/x509` reference vectors
//! (`tooling/x509_vectors.go` -> `seed/tests/fixtures/x509-vectors.json`).
mod common;

use serde_json::Value;
use std::collections::HashMap;

/// All non-embedded, non-toolkit prelude modules transitively required by Crypto.Pkix and Crypto.X509.
const DEPS: &[(&str, &str)] = &[
    ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
    ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
    ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
    ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
    ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
    ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
    ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
    ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
    ("Text.Base64.Encoding.cinch", include_str!("../prelude/Text/Base64/Encoding.cinch")),
    ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
    ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
    ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
    ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
    ("Text.Pem.cinch", include_str!("../prelude/Text/Pem.cinch")),
    ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
    ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
    ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
    ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
    ("Crypto.Hmac.cinch", include_str!("../prelude/Crypto/Hmac.cinch")),
    ("Crypto.P256.cinch", include_str!("../prelude/Crypto/P256.cinch")),
    ("Crypto.Ecdsa.cinch", include_str!("../prelude/Crypto/Ecdsa.cinch")),
    ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
    ("Crypto.Ed25519.cinch", include_str!("../prelude/Crypto/Ed25519.cinch")),
    ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
    ("Crypto.Rsa.cinch", include_str!("../prelude/Crypto/Rsa.cinch")),
    ("Crypto.Pkix.cinch", include_str!("../prelude/Crypto/Pkix.cinch")),
    ("Crypto.X509.cinch", include_str!("../prelude/Crypto/X509.cinch")),
];

/// Compile and run one batch of test rows; exit code 200 means all rows passed,
/// otherwise the 0-based index of the first failing row.
fn run_batch(tag: &str, extra_defs: &str, rows: &[String]) {
    let main_src = format!(
        "{HELPERS}\n{extra_defs}\n  -- | Evaluate every test row in this batch.\n  main :: Int\n  main = first [\n    {}\n  ] 0\n",
        rows.join(",\n    ")
    );
    let mut files: Vec<(&str, &str)> = DEPS.to_vec();
    files.push(("Main.cinch", &main_src));
    let dir = common::project(tag, &files);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 200,
        "batch {tag}: exit status {code} is the 0-based index of the first failing row (200 = all {} rows passed)",
        rows.len()
    );
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/x509-vectors.json")).unwrap()
}

fn q(s: &str) -> String {
    format!("{:?}", s)
}

fn s(v: &Value, k: &str) -> String {
    q(v[k].as_str().unwrap())
}

fn str_list(v: &Value, k: &str) -> String {
    let items: Vec<String> = v[k]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| q(x.as_str().unwrap()))
        .collect();
    format!("[{}]", items.join(", "))
}

fn cinch_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn sig_alg_ctor(name: &str) -> &'static str {
    match name {
        "sha1_rsa" => "SigSha1WithRsa",
        "sha256_rsa" => "SigSha256WithRsa",
        "sha384_rsa" => "SigSha384WithRsa",
        "sha512_rsa" => "SigSha512WithRsa",
        "sha256_rsapss" => "SigSha256WithRsaPss",
        "sha384_rsapss" => "SigSha384WithRsaPss",
        "sha512_rsapss" => "SigSha512WithRsaPss",
        "ecdsa_sha256" => "SigEcdsaWithSha256",
        "ecdsa_sha384" => "SigEcdsaWithSha384",
        "ecdsa_sha512" => "SigEcdsaWithSha512",
        "ed25519" => "SigPureEd25519",
        other => panic!("unexpected sig_alg: {other}"),
    }
}

fn eku_ctor(name: &str) -> &'static str {
    match name {
        "any" => "ExtKeyUsageAny",
        "server_auth" => "ExtKeyUsageServerAuth",
        "client_auth" => "ExtKeyUsageClientAuth",
        "code_signing" => "ExtKeyUsageCodeSigning",
        "email_protection" => "ExtKeyUsageEmailProtection",
        "ipsec_end_system" => "ExtKeyUsageIpsecEndSystem",
        "ipsec_tunnel" => "ExtKeyUsageIpsecTunnel",
        "ipsec_user" => "ExtKeyUsageIpsecUser",
        "time_stamping" => "ExtKeyUsageTimeStamping",
        "ocsp_signing" => "ExtKeyUsageOcspSigning",
        other => panic!("unexpected eku: {other}"),
    }
}

fn eku_list(v: &Value, k: &str) -> String {
    let items: Vec<&str> = v[k]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| eku_ctor(x.as_str().unwrap()))
        .collect();
    format!("[{}]", items.join(", "))
}

fn cert_der_map(f: &Value) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for c in f["certs"].as_array().unwrap() {
        m.insert(
            c["name"].as_str().unwrap().to_string(),
            c["der"].as_str().unwrap().to_string(),
        );
    }
    m
}

/// Every public and private declaration in Crypto.Pkix has canonical documentation.
#[test]
fn pkix_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Pkix", include_str!("../prelude/Crypto/Pkix.cinch"));
}

/// Every public and private declaration in Crypto.X509 has canonical documentation.
#[test]
fn x509_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.X509", include_str!("../prelude/Crypto/X509.cinch"));
}

/// All 5 PKIX Name vectors, OID parsing/formatting, UTCTime/GeneralizedTime codecs,
/// AlgorithmIdentifier/Extension roundtrips, and all 3 CRL vectors + signature checks.
#[test]
fn pkix_names_oids_times_and_crls_match_go() {
    let f = fixture();
    let certs = cert_der_map(&f);
    let mut rows = Vec::new();

    for n in f["pkix_names"].as_array().unwrap() {
        rows.push(format!(
            "check-pkix-name {} {} {} {} {} {} {} {} {} {} {}",
            s(n, "der"),
            s(n, "string"),
            s(n, "common_name"),
            s(n, "serial_number"),
            str_list(n, "country"),
            str_list(n, "organization"),
            str_list(n, "organizational_unit"),
            str_list(n, "locality"),
            str_list(n, "province"),
            str_list(n, "street_address"),
            str_list(n, "postal_code"),
        ));
    }

    rows.push("check-pkix-oids-times-and-helpers".to_string());

    for crl in f["crls"].as_array().unwrap() {
        let issuer_der = &certs[crl["issuer_cert"].as_str().unwrap()];
        let rev_entries: Vec<String> = crl["revoked"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "({}, {}, {})",
                    r["serial"].as_str().unwrap(),
                    r["revocation_time"].as_i64().unwrap(),
                    r["ext_count"].as_u64().unwrap()
                )
            })
            .collect();
        rows.push(format!(
            "check-crl {} {} {} {} {} [{}] {}",
            s(crl, "der"),
            q(issuer_der),
            s(crl, "issuer"),
            crl["this_update"].as_i64().unwrap(),
            crl["next_update"].as_i64().unwrap(),
            rev_entries.join(", "),
            cinch_bool(crl["valid_sig"].as_bool().unwrap()),
        ));
    }

    run_batch("x509_pkix", "", &rows);
}

/// All 4 key codecs (RSA-2048, P-256, Ed25519, X25519) across PKIX, PKCS#8, SEC 1,
/// and PKCS#1 exact DER roundtrips plus all 4 malformed key DER rejections.
#[test]
fn x509_key_codecs_match_go() {
    let f = fixture();
    let mut rows = Vec::new();

    for k in f["keys"].as_array().unwrap() {
        rows.push(format!(
            "check-key-codec {} {} {} {} {} {} {} {}",
            s(k, "kind"),
            s(k, "pkix_public_der"),
            s(k, "pkcs8_private_der"),
            q(k.get("sec1_private_der").and_then(|v| v.as_str()).unwrap_or("")),
            q(k.get("pkcs1_public_der").and_then(|v| v.as_str()).unwrap_or("")),
            q(k.get("pkcs1_private_der").and_then(|v| v.as_str()).unwrap_or("")),
            q(k.get("raw_public").and_then(|v| v.as_str()).unwrap_or("")),
            q(k.get("raw_private").and_then(|v| v.as_str()).unwrap_or("")),
        ));
    }

    for bad in f["bad_key_ders"].as_array().unwrap() {
        rows.push(format!("check-bad-key-der {}", q(bad.as_str().unwrap())));
    }

    run_batch("x509_keys", "", &rows);
}

/// All 25 X.509 v3 certificate vectors (full field/extension/TBS/signature/key verification),
/// multi-DER `parse-certificates`, PEM bundle parsing & `CertPool`, and RSA/PSS/ECDSA/Ed25519
/// `check-signature-from` positive and negative checks.
#[test]
fn x509_certificates_parsing_pem_and_signatures_match_go() {
    let f = fixture();
    let certs = cert_der_map(&f);
    let mut defs = String::new();
    let mut rows = Vec::new();

    for (idx, c) in f["certs"].as_array().unwrap().iter().enumerate() {
        let mpl_expr = {
            let mpl = c["max_path_len"].as_i64().unwrap();
            let mpl0 = c["max_path_len_zero"].as_bool().unwrap();
            if mpl > 0 || (mpl == 0 && mpl0) {
                format!("(Some {})", mpl)
            } else {
                "None".to_string()
            }
        };
        let fn_name = format!("cert-row{idx}");
        defs.push_str(&format!(
            "  -- | Verify parsed fields of cert vector {}.\n  {fn_name} :: Bool\n  {fn_name} =\n    check-cert-core {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n      && check-cert-ext {} {} {} {} {} {} {} {} {} {} {}\n\n",
            c["name"].as_str().unwrap(),
            s(c, "der"),
            c["version"].as_u64().unwrap(),
            c["serial"].as_str().unwrap(),
            sig_alg_ctor(c["sig_alg"].as_str().unwrap()),
            s(c, "issuer"),
            s(c, "subject"),
            c["not_before"].as_i64().unwrap(),
            c["not_after"].as_i64().unwrap(),
            cinch_bool(c["is_ca"].as_bool().unwrap()),
            cinch_bool(c["basic_constraints_valid"].as_bool().unwrap()),
            mpl_expr,
            c["key_usage"].as_u64().unwrap(),
            eku_list(c, "ext_key_usages"),
            s(c, "raw_tbs"),
            s(c, "signature"),
            s(c, "public_key_kind"),
            s(c, "der"),
            str_list(c, "dns_names"),
            str_list(c, "email_addresses"),
            str_list(c, "ip_addresses"),
            str_list(c, "uris"),
            s(c, "subject_key_id"),
            s(c, "authority_key_id"),
            str_list(c, "permitted_dns"),
            str_list(c, "excluded_dns"),
            str_list(c, "permitted_ip_ranges"),
            str_list(c, "excluded_ip_ranges"),
        ));
        rows.push(fn_name);
    }

    // Multi-DER parse-certificates + PEM bundle parse-certificates-pem & cert-pool-append-pem.
    rows.push(format!(
        "check-multi-and-pem {} {} {} {} {}",
        q(&certs["leaf_p256"]),
        q(&certs["inter_p256"]),
        q(&certs["root_rsa"]),
        q(f["pem_bundle"].as_str().unwrap()),
        f["pem_cert_count"].as_u64().unwrap(),
    ));

    // Direct cryptographic signature verification across every signature algorithm in the fixture.
    let valid_sig_pairs = [
        ("root_rsa", "root_rsa"),
        ("inter_p256", "root_rsa"),
        ("leaf_p256", "inter_p256"),
        ("root_ed25519", "root_ed25519"),
        ("leaf_ed25519", "root_ed25519"),
        ("leaf_rsapss_sha256", "root_rsa"),
        ("leaf_rsa_sha384", "root_rsa"),
        ("leaf_rsa_sha512", "root_rsa"),
        ("leaf_rsapss_sha384", "root_rsa"),
        ("leaf_ecdsa_sha384", "inter_p256"),
        ("leaf_ecdsa_sha512", "inter_p256"),
    ];
    for (child, parent) in valid_sig_pairs {
        rows.push(format!(
            "check-sig-from-ok {} {}",
            q(&certs[child]),
            q(&certs[parent])
        ));
    }

    // Negative signature checks: tampered signature & non-CA issuer.
    rows.push(format!(
        "check-sig-from-err {} {} \"signature\"",
        q(&certs["leaf_tampered_sig"]),
        q(&certs["inter_p256"])
    ));
    rows.push(format!(
        "check-sig-from-err {} {} \"not_ca\"",
        q(&certs["leaf_by_non_ca"]),
        q(&certs["inter_non_ca"])
    ));

    run_batch("x509_certs", &defs, &rows);
}

/// All 18 RFC 6125 DNS and IP hostname verification cases cross-checked against Go.
#[test]
fn x509_hostname_verification_matches_go() {
    let f = fixture();
    let certs = cert_der_map(&f);
    let mut rows = Vec::new();

    for hc in f["hostname_cases"].as_array().unwrap() {
        let cert_der = &certs[hc["cert"].as_str().unwrap()];
        rows.push(format!(
            "check-hostname {} {} {}",
            q(cert_der),
            s(hc, "host"),
            cinch_bool(hc["valid"].as_bool().unwrap()),
        ));
    }

    run_batch("x509_hostnames", "", &rows);
}

/// All 19 CertPool chain verification cases (3-cert RSA/P-256, Ed25519, RSA-PSS, expiry,
/// tampered signature, missing intermediate, untrusted root, non-CA, MaxPathLen, DNS/IP
/// NameConstraints, EKU, and unhandled critical extension).
#[test]
fn x509_chain_verification_matches_go() {
    let f = fixture();
    let certs = cert_der_map(&f);
    let mut rows = Vec::new();

    for vc in f["verify_cases"].as_array().unwrap() {
        let leaf_der = &certs[vc["leaf"].as_str().unwrap()];
        let inter_ders: Vec<String> = vc["intermediates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| q(&certs[n.as_str().unwrap()]))
            .collect();
        let root_ders: Vec<String> = vc["roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| q(&certs[n.as_str().unwrap()]))
            .collect();
        rows.push(format!(
            "check-verify {} [{}] [{}] {} {} {} {} {} {}",
            q(leaf_der),
            inter_ders.join(", "),
            root_ders.join(", "),
            s(vc, "dns_name"),
            vc["current_time"].as_i64().unwrap(),
            eku_list(vc, "key_usages"),
            cinch_bool(vc["expect_ok"].as_bool().unwrap()),
            s(vc, "expect_reason"),
            vc["chain_len"].as_u64().unwrap(),
        ));
    }

    run_batch("x509_verify", "", &rows);
}

/// All 4 PKCS#10 CSR vectors plus end-to-end `create-certificate`, `create-certificate-request`,
/// `create-revocation-list`, error message coverage, and malformed DER rejections.
#[test]
fn x509_csrs_creation_and_error_rejections() {
    let f = fixture();
    let mut rows = Vec::new();

    for csr in f["csrs"].as_array().unwrap() {
        rows.push(format!(
            "check-csr {} {} {} {} {} {} {} {}",
            s(csr, "der"),
            s(csr, "subject"),
            sig_alg_ctor(csr["sig_alg"].as_str().unwrap()),
            str_list(csr, "dns_names"),
            str_list(csr, "email_addresses"),
            str_list(csr, "ip_addresses"),
            str_list(csr, "uris"),
            cinch_bool(csr["valid_sig"].as_bool().unwrap()),
        ));
    }

    // Grab the RSA-2048, P-256, and Ed25519 PKCS#8 private keys from the fixture for creation roundtrips.
    let key_pkcs8 = |name: &str| -> String {
        f["keys"]
            .as_array()
            .unwrap()
            .iter()
            .find(|k| k["name"].as_str().unwrap() == name)
            .unwrap()["pkcs8_private_der"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let rsa_k = key_pkcs8("rsa2048");
    let p256_k = key_pkcs8("p256");
    let ed_k = key_pkcs8("ed25519");

    rows.push(format!(
        "check-create-cert-chain {} {} {}",
        q(&ed_k),
        q(&p256_k),
        q(&rsa_k)
    ));
    rows.push(format!("check-create-rsa-certs {}", q(&rsa_k)));
    rows.push(format!(
        "check-create-csrs {} {} {}",
        q(&rsa_k),
        q(&p256_k),
        q(&ed_k)
    ));
    rows.push(format!("check-create-crls {} {}", q(&rsa_k), q(&ed_k)));
    rows.push("check-x509-errors-and-bad-ders".to_string());

    run_batch("x509_create", "", &rows);
}

const HELPERS: &str = r#"-- | Reference PKIX and X.509 test consumers.
module Main where
  import Crypto.Ed25519
  import Crypto.P256
  import Crypto.Pkix
  import Crypto.Rsa
  import Crypto.X25519
  import Crypto.X509
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.NetIp
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Asn1
  import Text.Hex

  -- | Decode hex string to Bytes.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err _ -> bytes-new 0

  -- | Exact byte equality.
  beq :: Bytes -> Bytes -> Bool
  beq x y = eq-i64 (bytes-compare x y) 0

  -- | Compare two string lists element-by-element.
  str-list-eq :: List String -> List String -> Bool
  str-list-eq xs ys = case (xs, ys) of
    (Nil, Nil) -> True
    (Cons a ra, Cons b0 rb) -> if a == b0 then str-list-eq ra rb else False
    other -> False

  -- | Compare two EKU lists element-by-element.
  eku-list-eq :: List ExtKeyUsage -> List ExtKeyUsage -> Bool
  eku-list-eq xs ys = case (xs, ys) of
    (Nil, Nil) -> True
    (Cons a ra, Cons b0 rb) -> if eku-equal a b0 then eku-list-eq ra rb else False
    other -> False

  -- | Map addr-to-string over a list of Addrs.
  addrs-to-strings :: List Addr -> List String -> List String
  addrs-to-strings xs acc = case xs of
    Nil -> reverse acc
    Cons a rest -> addrs-to-strings rest (Cons (addr-to-string a) acc)

  -- | Map prefix-to-string over a list of Prefixes.
  prefixes-to-strings :: List Prefix -> List String -> List String
  prefixes-to-strings xs acc = case xs of
    Nil -> reverse acc
    Cons p rest -> prefixes-to-strings rest (Cons (prefix-to-string p) acc)

  -- | Count list length tail-recursively.
  llen :: forall a. List a -> Nat -> Nat
  llen xs acc = case xs of
    Nil -> acc
    Cons _ rest -> llen rest (acc + 1)

  -- | Check Option String against expected string (empty means None).
  opt-str-eq :: Option String -> String -> Bool
  opt-str-eq opt exp = case opt of
    None -> exp == ""
    Some v -> v == exp

  -- | Check Option Bytes against expected hex string (empty means None).
  opt-hex-eq :: Option Bytes -> String -> Bool
  opt-hex-eq opt exp = case opt of
    None -> exp == ""
    Some v -> hex-encode v == exp

  -- | Check Option Nat equality.
  opt-nat-eq :: Option Nat -> Option Nat -> Bool
  opt-nat-eq a b0 = case (a, b0) of
    (None, None) -> True
    (Some x, Some y) -> x == y
    other -> False

  -- | Check Option I64 against expected value.
  opt-i64-eq :: Option I64 -> I64 -> Bool
  opt-i64-eq opt exp = case opt of
    None -> False
    Some v -> eq-i64 v exp

  -- | Classify a PublicKey kind string.
  pub-kind-str :: PublicKey -> String
  pub-kind-str pub = case pub of
    PubRsa _ -> "rsa"
    PubP256 _ -> "p256"
    PubEd25519 _ -> "ed25519"
    PubX25519 _ -> "x25519"

  -- | Match an X509Error against a fixture reason tag.
  err-matches-reason :: X509Error -> String -> Bool
  err-matches-reason err reason = case err of
    X509HostnameMismatch _ -> reason == "hostname"
    X509Expired -> reason == "expired"
    X509SignatureInvalid -> reason == "signature"
    X509UnknownAuthority -> reason == "unknown_authority"
    X509NotAuthorizedToSign -> reason == "not_ca"
    X509TooManyIntermediates -> reason == "pathlen"
    X509NameConstraintViolation _ -> reason == "name_constraint"
    X509IncompatibleUsage -> reason == "eku"
    X509UnhandledCriticalExtension _ -> reason == "critical_ext"
    other -> False

  -- | Verify a PKIX Name DER vector, Go canonical Name.String() formatting, wire RFC 4514 formatting, all X.520 accessors, and DER roundtrip.
  check-pkix-name :: String -> String -> String -> String -> List String -> List String -> List String -> List String -> List String -> List String -> List String -> Bool
  check-pkix-name der-hex expected-str cn sn country org ou loc prov street postal =
    case pkix-parse-name (b der-hex) of
      Err _ -> False
      Ok name ->
        let rt-der = pkix-marshal-name name in
        let wire-str = pkix-name-to-string name in
        let rt-ok = (case pkix-parse-name rt-der of
          Err _ -> False
          Ok name2 -> pkix-name-equal name name2 && pkix-name-to-canonical-string name2 == expected-str && pkix-name-to-string name2 == wire-str) in
        pkix-name-to-canonical-string name == expected-str
          && opt-str-eq (pkix-common-name name) cn
          && opt-str-eq (pkix-serial-number name) sn
          && str-list-eq (pkix-country name) country
          && str-list-eq (pkix-organization name) org
          && str-list-eq (pkix-organizational-unit name) ou
          && str-list-eq (pkix-locality name) loc
          && str-list-eq (pkix-province name) prov
          && str-list-eq (pkix-street-address name) street
          && str-list-eq (pkix-postal-code name) postal
          && rt-ok

  -- | Verify PKIX OID parsing/formatting, UTCTime/GeneralizedTime roundtrips, AlgorithmIdentifier, Extension, and error messages.
  check-pkix-oids-times-and-helpers :: Bool
  check-pkix-oids-times-and-helpers =
    let oid-ok = (oid-to-string oid-common-name == "2.5.4.3"
        && (case oid-from-string "2.5.4.3" of Ok o -> oid-equal o oid-common-name; Err _ -> False)
        && (case oid-from-string "1.2.840.113549.1.1.11" of Ok o -> oid-to-string o == "1.2.840.113549.1.1.11"; Err _ -> False)
        && (case oid-from-string "" of Err PkixInvalidOid -> True; other -> False)
        && (case oid-from-string "1" of Err PkixInvalidOid -> True; other -> False)
        && (case oid-from-string "3.1" of Err PkixInvalidOid -> True; other -> False)
        && (case oid-from-string "1.40" of Err PkixInvalidOid -> True; other -> False)
        && (case oid-from-string "1.01" of Err PkixInvalidOid -> True; other -> False)) in
    let t-utc = (1781524800 :: I64) in
    let t-gen = (2693106305 :: I64) in
    let t-old = (-631152001 :: I64) in
    let utc-asn1 = pkix-encode-time t-utc in
    let gen-asn1 = pkix-encode-time t-gen in
    let old-asn1 = pkix-encode-time t-old in
    let time-ok = ((case utc-asn1 of Asn1UtcTime _ -> True; other -> False)
        && (case gen-asn1 of Asn1GeneralizedTime _ -> True; other -> False)
        && (case old-asn1 of Asn1GeneralizedTime _ -> True; other -> False)
        && (case pkix-decode-time utc-asn1 of Ok v -> eq-i64 v t-utc; Err _ -> False)
        && (case pkix-decode-time gen-asn1 of Ok v -> eq-i64 v t-gen; Err _ -> False)
        && (case pkix-decode-time old-asn1 of Ok v -> eq-i64 v t-old; Err _ -> False)
        && (case pkix-decode-time Asn1Null of Err PkixInvalidTime -> True; other -> False)) in
    let alg = pkix-algorithm-identifier [1, 2, 840, 113549, 1, 1, 11] (Some Asn1Null) in
    let alg-ok = (case pkix-alg-from-asn1 (pkix-alg-to-asn1 alg) of
      Ok a2 -> pkix-alg-equal alg a2
      Err _ -> False) in
    let ext1 = pkix-extension [2, 5, 29, 19] True (bytes-from-u8s [48, 0]) in
    let ext2 = pkix-extension [1, 2, 3, 4] False (bytes-from-u8s [5, 0]) in
    let ext-ok = ((case pkix-ext-from-asn1 (pkix-ext-to-asn1 ext1) of
        Ok e2 -> oid-equal (pkix-ext-oid e2) [2, 5, 29, 19] && pkix-ext-critical e2 && beq (pkix-ext-value e2) (bytes-from-u8s [48, 0])
        Err _ -> False)
        && (case pkix-find-extension [1, 2, 3, 4] [ext1, ext2] of Some e -> not (pkix-ext-critical e); None -> False)) in
    let built-name = pkix-with-common-name (pkix-with-organization (pkix-with-country pkix-name-empty "US") "Cinch") "test.example" in
    let name-ok = pkix-name-to-canonical-string built-name == "CN=test.example,O=Cinch,C=US" in
    let msgs-ok = (pkix-error-message (PkixInvalidStructure "x") /= ""
        && pkix-error-message PkixInvalidTime /= ""
        && pkix-error-message PkixInvalidOid /= "") in
    oid-ok && time-ok && alg-ok && ext-ok && name-ok && msgs-ok

  -- | Check revoked CRL entries against expected (serial, revocation_time, ext_count) tuples.
  check-revoked-list :: List PkixRevokedCertificate -> List (Int, I64, Nat) -> Bool
  check-revoked-list actual expected = case (actual, expected) of
    (Nil, Nil) -> True
    (Cons r rr, Cons (ser, rtime, ecnt) er) ->
      pkix-revoked-serial r == ser
        && eq-i64 (pkix-revoked-time r) rtime
        && llen (pkix-revoked-extensions r) 0 == ecnt
        && check-revoked-list rr er
    other -> False

  -- | Verify a CRL vector: parse fields, revoked entries, and signature verification against issuer cert.
  check-crl :: String -> String -> String -> I64 -> I64 -> List (Int, I64, Nat) -> Bool -> Bool
  check-crl der-hex issuer-cert-hex expected-issuer this-u next-u expected-rev valid-sig =
    case (parse-crl (b der-hex), parse-certificate (b issuer-cert-hex)) of
      (Ok crl, Ok issuer-cert) ->
        let sig-ok = (case check-crl-signature-from crl issuer-cert of
          Ok _ -> valid-sig
          Err _ -> not valid-sig) in
        pkix-name-to-string (crl-issuer crl) == expected-issuer
          && eq-i64 (crl-this-update crl) this-u
          && opt-i64-eq (crl-next-update crl) next-u
          && check-revoked-list (crl-revoked-certificates crl) expected-rev
          && sig-ok
      other -> False

  -- | Verify PKIX public, PKCS#8 private, optional SEC 1 / PKCS#1, and raw key byte roundtrips.
  check-key-codec :: String -> String -> String -> String -> String -> String -> String -> String -> Bool
  check-key-codec kind pkix-hex pkcs8-hex sec1-hex pkcs1-pub-hex pkcs1-priv-hex raw-pub-hex raw-priv-hex =
    case (parse-pkix-public-key (b pkix-hex), parse-pkcs8-private-key (b pkcs8-hex)) of
      (Ok pub, Ok priv) ->
        let base-ok = (pub-kind-str pub == kind
            && public-key-equal pub (public-key-of priv)
            && hex-encode (marshal-pkix-public-key pub) == pkix-hex
            && hex-encode (marshal-pkcs8-private-key priv) == pkcs8-hex) in
        let extra-ok = (if kind == "rsa" then
            (case (parse-pkcs1-public-key (b pkcs1-pub-hex), parse-pkcs1-private-key (b pkcs1-priv-hex)) of
              (Ok rpub, Ok rpriv) ->
                hex-encode (marshal-pkcs1-public-key rpub) == pkcs1-pub-hex
                  && hex-encode (marshal-pkcs1-private-key rpriv) == pkcs1-priv-hex
                  && public-key-equal pub (PubRsa rpub)
                  && public-key-equal pub (PubRsa (rsa-public-key-of rpriv))
              other -> False)
          else if kind == "p256" then
            (case parse-ec-private-key (b sec1-hex) of
              Ok ec-priv ->
                hex-encode (marshal-ec-private-key ec-priv) == sec1-hex
                  && hex-encode (p256-private-key-bytes ec-priv) == raw-priv-hex
                  && hex-encode (p256-public-key-bytes (p256-public-key-of ec-priv)) == raw-pub-hex
                  && public-key-equal pub (PubP256 (p256-public-key-of ec-priv))
              Err _ -> False)
          else if kind == "ed25519" then
            (case (pub, priv) of
              (PubEd25519 epub, PrivEd25519 epriv) ->
                hex-encode (ed25519-public-key-bytes epub) == raw-pub-hex
                  && hex-encode (ed25519-seed epriv) == raw-priv-hex
              other -> False)
          else if kind == "x25519" then
            (case (pub, priv) of
              (PubX25519 xpub, PrivX25519 xpriv) ->
                hex-encode (x25519-public-key-bytes xpub) == raw-pub-hex
                  && hex-encode (x25519-private-bytes xpriv) == raw-priv-hex
              other -> False)
          else False) in
        base-ok && extra-ok
      other -> False

  -- | Malformed DER buffers must be rejected by every key parser.
  check-bad-key-der :: String -> Bool
  check-bad-key-der der-hex =
    let bs = b der-hex in
    let r1 = case parse-pkix-public-key bs of Err _ -> True; Ok _ -> False in
    let r2 = case parse-pkcs8-private-key bs of Err _ -> True; Ok _ -> False in
    let r3 = case parse-ec-private-key bs of Err _ -> True; Ok _ -> False in
    let r4 = case parse-pkcs1-public-key bs of Err _ -> True; Ok _ -> False in
    let r5 = case parse-pkcs1-private-key bs of Err _ -> True; Ok _ -> False in
    r1 && r2 && r3 && r4 && r5

  -- | Verify core X.509 certificate fields (version, serial, alg, names, validity, constraints, KU/EKU, raw TBS/sig/pub).
  check-cert-core :: String -> Nat -> Int -> SignatureAlgorithm -> String -> String -> I64 -> I64 -> Bool -> Bool -> Option Nat -> U32 -> List ExtKeyUsage -> String -> String -> String -> Bool
  check-cert-core der-hex ver serial alg issuer subject nb na is-ca bc-valid mpl ku ekus raw-tbs-hex sig-hex pub-kind =
    case parse-certificate (b der-hex) of
      Err _ -> False
      Ok c ->
        cert-version c == ver
          && cert-serial-number c == serial
          && sig-alg-equal (cert-signature-algorithm c) alg
          && pkix-name-to-string (cert-issuer c) == issuer
          && pkix-name-to-string (cert-subject c) == subject
          && eq-i64 (cert-not-before c) nb
          && eq-i64 (cert-not-after c) na
          && cert-is-ca c == is-ca
          && cert-basic-constraints-valid c == bc-valid
          && opt-nat-eq (cert-max-path-len c) mpl
          && eq-u32 (cert-key-usage c) ku
          && eku-list-eq (cert-ext-key-usage c) ekus
          && hex-encode (cert-raw-tbs-certificate c) == raw-tbs-hex
          && hex-encode (cert-signature c) == sig-hex
          && pub-kind-str (cert-public-key c) == pub-kind

  -- | Verify X.509 certificate SAN, SKI, AKI, and NameConstraints fields.
  check-cert-ext :: String -> List String -> List String -> List String -> List String -> String -> String -> List String -> List String -> List String -> List String -> Bool
  check-cert-ext der-hex dns emails ips uris ski-hex aki-hex perm-dns excl-dns perm-ips excl-ips =
    case parse-certificate (b der-hex) of
      Err _ -> False
      Ok c ->
        str-list-eq (cert-dns-names c) dns
          && str-list-eq (cert-email-addresses c) emails
          && str-list-eq (addrs-to-strings (cert-ip-addresses c) Nil) ips
          && str-list-eq (cert-uris c) uris
          && opt-hex-eq (cert-subject-key-id c) ski-hex
          && opt-hex-eq (cert-authority-key-id c) aki-hex
          && str-list-eq (cert-permitted-dns-domains c) perm-dns
          && str-list-eq (cert-excluded-dns-domains c) excl-dns
          && str-list-eq (prefixes-to-strings (cert-permitted-ip-ranges c) Nil) perm-ips
          && str-list-eq (prefixes-to-strings (cert-excluded-ip-ranges c) Nil) excl-ips

  -- | Verify concatenated DER `parse-certificates`, PEM `parse-certificates-pem`, and `cert-pool-append-pem`.
  check-multi-and-pem :: String -> String -> String -> String -> Nat -> Bool
  check-multi-and-pem d1 d2 d3 pem-str expected-cnt =
    let concat-der = bytes-concat [b d1, b d2, b d3] in
    let multi-ok = (case parse-certificates concat-der of
      Ok cs -> llen cs 0 == 3
      Err _ -> False) in
    let pem-bytes = string-to-bytes pem-str in
    let pem-ok = (case parse-certificates-pem pem-bytes of
      Ok cs -> llen cs 0 == expected-cnt
      Err _ -> False) in
    let pool-ok = (case cert-pool-append-pem cert-pool-new pem-bytes of
      (pool, added) -> added && cert-pool-len pool == expected-cnt) in
    multi-ok && pem-ok && pool-ok

  -- | Verify `check-signature-from` succeeds for a valid child/parent certificate pair.
  check-sig-from-ok :: String -> String -> Bool
  check-sig-from-ok child-hex parent-hex =
    case (parse-certificate (b child-hex), parse-certificate (b parent-hex)) of
      (Ok child, Ok parent) -> (case check-signature-from child parent of Ok _ -> True; Err _ -> False)
      other -> False

  -- | Verify `check-signature-from` fails with the expected error reason.
  check-sig-from-err :: String -> String -> String -> Bool
  check-sig-from-err child-hex parent-hex reason =
    case (parse-certificate (b child-hex), parse-certificate (b parent-hex)) of
      (Ok child, Ok parent) -> (case check-signature-from child parent of Err e -> err-matches-reason e reason; Ok _ -> False)
      other -> False

  -- | Verify `verify-hostname` matches Go's `Certificate.VerifyHostname`.
  check-hostname :: String -> String -> Bool -> Bool
  check-hostname cert-hex host expected =
    case parse-certificate (b cert-hex) of
      Err _ -> False
      Ok c -> (case verify-hostname c host of Ok _ -> expected; Err _ -> not expected)

  -- | Build a CertPool from a list of hex DER certificates.
  pool-from-hex :: List String -> CertPool -> Option CertPool
  pool-from-hex xs pool = case xs of
    Nil -> Some pool
    Cons h rest -> (case parse-certificate (b h) of
      Err _ -> None
      Ok c -> pool-from-hex rest (cert-pool-add pool c))

  -- | Verify a chain verification case against `verify-certificate`.
  check-verify :: String -> List String -> List String -> String -> I64 -> List ExtKeyUsage -> Bool -> String -> Nat -> Bool
  check-verify leaf-hex inter-hexes root-hexes dns now ekus expect-ok expect-reason expect-chain-len =
    case (parse-certificate (b leaf-hex), pool-from-hex inter-hexes cert-pool-new, pool-from-hex root-hexes cert-pool-new) of
      (Ok leaf, Some inters, Some roots) ->
        let o0 = verify-options-new roots now in
        let o1 = verify-with-intermediates o0 inters in
        let o2 = if dns == "" then o1 else verify-with-dns-name o1 dns in
        let opts = verify-with-key-usages o2 ekus in
        (case verify-certificate opts leaf of
          Ok chains ->
            if not expect-ok then False
            else (case chains of
              Cons ch _ -> llen ch 0 == expect-chain-len
              Nil -> False)
          Err e ->
            if expect-ok then False
            else err-matches-reason e expect-reason)
      other -> False

  -- | Verify a PKCS#10 CSR vector: subject, sig alg, SANs, and signature check.
  check-csr :: String -> String -> SignatureAlgorithm -> List String -> List String -> List String -> List String -> Bool -> Bool
  check-csr der-hex expected-sub alg dns emails ips uris valid-sig =
    case parse-certificate-request (b der-hex) of
      Err _ -> False
      Ok csr ->
        let sig-ok = (case check-certificate-request-signature csr of
          Ok _ -> valid-sig
          Err _ -> not valid-sig) in
        csr-version csr == 0
          && pkix-name-to-string (csr-subject csr) == expected-sub
          && sig-alg-equal (csr-signature-algorithm csr) alg
          && str-list-eq (csr-dns-names csr) dns
          && str-list-eq (csr-email-addresses csr) emails
          && str-list-eq (addrs-to-strings (csr-ip-addresses csr) Nil) ips
          && str-list-eq (csr-uris csr) uris
          && sig-ok

  -- | Parse an Addr helper (returns ipv4-unspecified on error).
  must-addr :: String -> Addr
  must-addr s = case parse-addr s of Ok a -> a; Err _ -> ipv4-unspecified

  -- | Parse a Prefix into a singleton list, or Nil on invalid input.
  one-prefix :: String -> List Prefix
  one-prefix s = case parse-prefix s of Ok p -> [p]; Err _ -> Nil

  -- | Create a 3-cert chain (Ed25519 Root -> P-256 Inter with NameConstraints -> RSA-2048 Leaf) and verify it!
  check-create-cert-chain :: String -> String -> String -> Bool
  check-create-cert-chain ed-hex p256-hex rsa-hex =
    case (parse-pkcs8-private-key (b ed-hex), parse-pkcs8-private-key (b p256-hex), parse-pkcs8-private-key (b rsa-hex)) of
      (Ok ed-priv, Ok p256-priv, Ok rsa-priv) ->
        let t0 = (1704067200 :: I64) in
        let t1 = (1781524800 :: I64) in
        let t2 = (2693106305 :: I64) in
        let root-sub = pkix-with-common-name (pkix-with-organization pkix-name-empty "Cinch Generated") "Gen Ed25519 Root" in
        let root-tmpl = template-with-subject-key-id (template-with-key-usage (template-with-max-path-len (template-with-ca (cert-template-new 1 root-sub t0 t2) True) 1) (key-usage-union key-usage-cert-sign key-usage-crl-sign)) (bytes-from-u8s [1, 2, 3, 4]) in
        (case create-certificate root-tmpl None (public-key-of ed-priv) ed-priv of
          Err _ -> False
          Ok root-der -> (case parse-certificate root-der of
            Err _ -> False
            Ok root-cert ->
              let inter-sub = pkix-with-common-name (pkix-with-organization pkix-name-empty "Cinch Generated") "Gen P256 Inter" in
              let inter-t0 = cert-template-new 2 inter-sub t0 t2 in
              let inter-t1 = template-with-max-path-len (template-with-ca inter-t0 True) 0 in
              let inter-t2 = template-with-key-usage inter-t1 (key-usage-union key-usage-cert-sign key-usage-crl-sign) in
              let inter-t3 = template-with-permitted-dns (template-with-excluded-dns inter-t2 ["denied.gen.example"]) ["gen.example"] in
              let inter-t4 = template-with-permitted-ips (template-with-excluded-ips inter-t3 (one-prefix "10.99.1.0/24")) (one-prefix "10.99.0.0/16") in
              let inter-tmpl = template-with-subject-key-id inter-t4 (bytes-from-u8s [5, 6, 7, 8]) in
              (case create-certificate inter-tmpl (Some root-cert) (public-key-of p256-priv) ed-priv of
                Err _ -> False
                Ok inter-der -> (case parse-certificate inter-der of
                  Err _ -> False
                  Ok inter-cert ->
                    let leaf-sub = pkix-with-common-name pkix-name-empty "app.gen.example" in
                    let leaf-t0 = cert-template-new 3 leaf-sub t0 t2 in
                    let leaf-t1 = template-with-key-usage leaf-t0 key-usage-digital-signature in
                    let leaf-t2 = template-with-ext-key-usage leaf-t1 [ExtKeyUsageServerAuth, ExtKeyUsageClientAuth] in
                    let leaf-t3 = template-with-dns-names leaf-t2 ["app.gen.example", "*.svc.gen.example"] in
                    let leaf-t4 = template-with-email-addresses leaf-t3 ["sec@gen.example"] in
                    let leaf-t5 = template-with-ip-addresses leaf-t4 [must-addr "10.99.2.10"] in
                    let leaf-tmpl = template-with-uris leaf-t5 ["https://app.gen.example/id"] in
                    (case create-certificate leaf-tmpl (Some inter-cert) (public-key-of rsa-priv) p256-priv of
                      Err _ -> False
                      Ok leaf-der -> (case parse-certificate leaf-der of
                        Err _ -> False
                        Ok leaf-cert ->
                          let roots = cert-pool-add cert-pool-new root-cert in
                          let inters = cert-pool-add cert-pool-new inter-cert in
                          let opts = verify-with-key-usages (verify-with-dns-name (verify-with-intermediates (verify-options-new roots t1) inters) "x.svc.gen.example") [ExtKeyUsageServerAuth] in
                          let chain-ok = (case verify-certificate opts leaf-cert of
                            Ok (Cons ch _) -> llen ch 0 == 3
                            other -> False) in
                          let host-ok = case verify-hostname leaf-cert "10.99.2.10" of Ok _ -> True; Err _ -> False in
                          let aki-ok = opt-hex-eq (cert-authority-key-id leaf-cert) "05060708" in
                          let self-ok = case check-signature-from root-cert root-cert of Ok _ -> True; Err _ -> False in
                          chain-ok && host-ok && aki-ok && self-ok))))))
      other -> False

  -- | Create RSA-signed certificates using both SHA256WithRSA and SHA256WithRSAPSS and verify signatures.
  check-create-rsa-certs :: String -> Bool
  check-create-rsa-certs rsa-hex =
    case parse-pkcs8-private-key (b rsa-hex) of
      Err _ -> False
      Ok rsa-priv ->
        let t0 = (1704067200 :: I64) in
        let t2 = (1800000000 :: I64) in
        let sub = pkix-with-common-name pkix-name-empty "rsa-ca.example" in
        let ca-tmpl = template-with-sig-alg (template-with-ca (template-with-key-usage (cert-template-new 10 sub t0 t2) key-usage-cert-sign) True) SigSha256WithRsa in
        let pss-tmpl = template-with-sig-alg (template-with-dns-names (cert-template-new 11 sub t0 t2) ["pss-gen.example"]) SigSha256WithRsaPss in
        (case create-certificate ca-tmpl None (public-key-of rsa-priv) rsa-priv of
          Err _ -> False
          Ok ca-der -> (case parse-certificate ca-der of
            Err _ -> False
            Ok ca-cert -> (case create-certificate pss-tmpl (Some ca-cert) (public-key-of rsa-priv) rsa-priv of
              Err _ -> False
              Ok pss-der -> (case parse-certificate pss-der of
                Err _ -> False
                Ok pss-cert ->
                  (case check-signature-from ca-cert ca-cert of Ok _ -> True; Err _ -> False)
                    && (case check-signature-from pss-cert ca-cert of Ok _ -> True; Err _ -> False)
                    && sig-alg-equal (cert-signature-algorithm pss-cert) SigSha256WithRsaPss))))

  -- | Create CSRs with RSA, P-256, and Ed25519 keys, parse them back, and verify their self-signatures and SANs.
  check-create-csrs :: String -> String -> String -> Bool
  check-create-csrs rsa-hex p256-hex ed-hex =
    case (parse-pkcs8-private-key (b rsa-hex), parse-pkcs8-private-key (b p256-hex), parse-pkcs8-private-key (b ed-hex)) of
      (Ok rsa-priv, Ok p256-priv, Ok ed-priv) ->
        let sub = pkix-with-common-name (pkix-with-organization pkix-name-empty "Cinch") "gen-csr.example" in
        let t = csr-template-with-uris (csr-template-with-ip-addresses (csr-template-with-email-addresses (csr-template-with-dns-names (csr-template-new sub) ["gen-csr.example"]) ["a@gen.example"]) [must-addr "192.0.2.77"]) ["https://gen-csr.example/x"] in
        let ok1 = (case create-certificate-request (csr-template-with-sig-alg t SigSha256WithRsa) rsa-priv of
          Ok der -> (case parse-certificate-request der of
            Ok csr -> (case check-certificate-request-signature csr of Ok _ -> str-list-eq (csr-dns-names csr) ["gen-csr.example"]; Err _ -> False)
            Err _ -> False)
          Err _ -> False) in
        let ok2 = (case create-certificate-request t p256-priv of
          Ok der -> (case parse-certificate-request der of
            Ok csr -> (case check-certificate-request-signature csr of Ok _ -> str-list-eq (addrs-to-strings (csr-ip-addresses csr) Nil) ["192.0.2.77"]; Err _ -> False)
            Err _ -> False)
          Err _ -> False) in
        let ok3 = (case create-certificate-request t ed-priv of
          Ok der -> (case parse-certificate-request der of
            Ok csr -> (case check-certificate-request-signature csr of Ok _ -> str-list-eq (csr-uris csr) ["https://gen-csr.example/x"]; Err _ -> False)
            Err _ -> False)
          Err _ -> False) in
        ok1 && ok2 && ok3
      other -> False

  -- | Create RSA and Ed25519 CRLs, parse them back, and verify signatures and revoked entries.
  check-create-crls :: String -> String -> Bool
  check-create-crls rsa-hex ed-hex =
    case (parse-pkcs8-private-key (b rsa-hex), parse-pkcs8-private-key (b ed-hex)) of
      (Ok rsa-priv, Ok ed-priv) ->
        let t0 = (1704067200 :: I64) in
        let t1 = (1735689600 :: I64) in
        let t2 = (1800000000 :: I64) in
        let sub = pkix-with-common-name pkix-name-empty "CRL Issuer" in
        let ca-tmpl = template-with-ca (template-with-key-usage (cert-template-new 50 sub t0 t2) (key-usage-union key-usage-cert-sign key-usage-crl-sign)) True in
        let rev = [pkix-revoked-certificate 777 t1 Nil] in
        let rsa-ok = (case create-certificate ca-tmpl None (public-key-of rsa-priv) rsa-priv of
          Ok cder -> (case parse-certificate cder of
            Ok issuer -> (case create-revocation-list issuer rsa-priv None t0 (Some t2) rev Nil of
              Ok crl-der -> (case parse-crl crl-der of
                Ok crl -> (case check-crl-signature-from crl issuer of Ok _ -> check-revoked-list (crl-revoked-certificates crl) [(777, t1, 0)]; Err _ -> False)
                Err _ -> False)
              Err _ -> False)
            Err _ -> False)
          Err _ -> False) in
        let ed-ok = (case create-certificate ca-tmpl None (public-key-of ed-priv) ed-priv of
          Ok cder -> (case parse-certificate cder of
            Ok issuer -> (case create-revocation-list issuer ed-priv None t0 (Some t2) Nil Nil of
              Ok crl-der -> (case parse-crl crl-der of
                Ok crl -> (case check-crl-signature-from crl issuer of Ok _ -> llen (crl-revoked-certificates crl) 0 == 0; Err _ -> False)
                Err _ -> False)
              Err _ -> False)
            Err _ -> False)
          Err _ -> False) in
        rsa-ok && ed-ok
      other -> False

  -- | Verify every X509Error variant produces a non-empty message and malformed DERs are rejected.
  check-x509-errors-and-bad-ders :: Bool
  check-x509-errors-and-bad-ders =
    let msgs = (x509-error-message (X509InvalidEncoding "e") /= ""
        && x509-error-message (X509InvalidKey "k") /= ""
        && x509-error-message X509UnsupportedAlgorithm /= ""
        && x509-error-message X509SignatureAlgorithmMismatch /= ""
        && x509-error-message X509SignatureInvalid /= ""
        && x509-error-message X509Expired /= ""
        && x509-error-message (X509HostnameMismatch "h") /= ""
        && x509-error-message X509UnknownAuthority /= ""
        && x509-error-message X509NotAuthorizedToSign /= ""
        && x509-error-message X509TooManyIntermediates /= ""
        && x509-error-message (X509NameConstraintViolation "n") /= ""
        && x509-error-message X509IncompatibleUsage /= ""
        && x509-error-message (X509UnhandledCriticalExtension [1, 2, 3]) /= ""
        && x509-error-message (X509InvalidExtension "x") /= "") in
    let bad = bytes-from-u8s [48, 3, 2, 1, 0] in
    let rej-cert = case parse-certificate bad of Err _ -> True; Ok _ -> False in
    let rej-csr = case parse-certificate-request bad of Err _ -> True; Ok _ -> False in
    let rej-crl = case parse-crl bad of Err _ -> True; Ok _ -> False in
    let rej-ed = case ed25519-public-key-from-bytes (bytes-new 16) of Err (X509InvalidKey _) -> True; other -> False in
    let rej-x25519 = case x25519-public-key-from-bytes (bytes-new 16) of Err (X509InvalidKey _) -> True; other -> False in
    msgs && rej-cert && rej-csr && rej-crl && rej-ed && rej-x25519

  -- | Return the 0-based index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of
    Nil -> 200
    Cons x rest -> if x then first rest (i + 1) else i
"#;
