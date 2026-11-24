//! HTTP cookies and the cookie jar against the reference SDK's (`tooling/cookie_vectors.go`): Set-Cookie
//! lines parse to identical fields or the identical error, strict and lenient Cookie header parsing agree,
//! constructed cookies serialize and validate identically, and scripted jar scenarios (hosts, ports, paths,
//! schemes, expiry over time, replacement order, public suffixes, IP addresses, internationalized hosts)
//! send the same cookies in the same order. Punycode, which canonicalizes internationalized hosts, is checked
//! against Python's bootstring codec.
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

/// A list of Cinch string literals.
fn list(v: &Value) -> String {
    format!("[{}]", v.as_array().unwrap().iter().map(|s| lit(s.as_str().unwrap())).collect::<Vec<_>>().join(", "))
}

/// A Cinch Int literal (negative values as a subtraction).
fn int(s: &str) -> String {
    match s.strip_prefix('-') {
        Some(n) => format!("(0 - {n})"),
        None => s.to_string(),
    }
}

/// The SameSite constructor for the reference's numbering.
fn same_site(s: &str) -> &'static str {
    match s {
        "1" => "SameSiteDefault",
        "2" => "SameSiteLax",
        "3" => "SameSiteStrict",
        "4" => "SameSiteNone",
        _ => "SameSiteUnset",
    }
}

/// A flag as a Cinch Bool.
fn flag(s: &str) -> &'static str {
    if s == "1" { "True" } else { "False" }
}

/// Every row of the fixture as a checked expression.
fn rows() -> Vec<String> {
    let f: Value = serde_json::from_str(include_str!("fixtures/cookie-vectors.json")).unwrap();
    let mut rows = Vec::new();
    for r in f["set"].as_array().unwrap() {
        rows.push(format!("set-row {} {}", lit(r["line"].as_str().unwrap()), lit(r["result"].as_str().unwrap())));
    }
    for r in f["cookie"].as_array().unwrap() {
        rows.push(format!("strict-row {} {}", lit(r["line"].as_str().unwrap()), lit(r["result"].as_str().unwrap())));
    }
    for r in f["read"].as_array().unwrap() {
        rows.push(format!("read-row {} {} {}", lit(r["line"].as_str().unwrap()), lit(r["filter"].as_str().unwrap()), lit(r["result"].as_str().unwrap())));
    }
    for r in f["string"].as_array().unwrap() {
        let x: Vec<&str> = r["fields"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let expires = if x[5] == "-" { "None".to_string() } else { format!("(Some {})", int(x[5])) };
        let cookie = format!(
            "(Cookie {} {} {} {} {} {expires} \"\" {} {} {} {} {} \"\" Nil)",
            lit(x[0]), lit(x[1]), flag(x[2]), lit(x[3]), lit(x[4]), int(x[6]), flag(x[7]), flag(x[8]), same_site(x[9]), flag(x[10])
        );
        rows.push(format!("string-row {cookie} {} {}", lit(r["string"].as_str().unwrap()), lit(r["valid"].as_str().unwrap())));
    }
    for sc in f["jar"].as_array().unwrap() {
        let steps = sc["steps"].as_array().unwrap().iter().map(|st| {
            let head = format!("{} {} {} {} {}", lit(st["url"].as_str().unwrap()), st["time"], lit(st["scheme"].as_str().unwrap()), lit(st["host"].as_str().unwrap()), lit(st["path"].as_str().unwrap()));
            if st["op"] == "set" { format!("Put {head} {}", list(&st["lines"])) } else { format!("Ask {head} {}", lit(st["want"].as_str().unwrap())) }
        }).collect::<Vec<_>>().join(", ");
        let psl = if sc["psl"].as_bool().unwrap() { "True" } else { "False" };
        rows.push(format!("scenario {psl} False [{steps}]"));
        rows.push(format!("scenario {psl} True [{steps}]"));
    }
    rows
}

/// Every row matches the reference.
#[test]
fn cookies_and_jar_match_reference() {
    let rows = rows();
    assert!(rows.len() < 200, "the exit status names the failing row, so rows must stay under 200");
    let dir = common::project("cookie", &[
        ("Data.CookieJar.cinch", include_str!("../prelude/Data/CookieJar.cinch")),
        ("Text.Cookie.cinch", include_str!("../prelude/Text/Cookie.cinch")),
        ("Text.Punycode.cinch", include_str!("../prelude/Text/Punycode.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Text.Url.cinch", include_str!("../prelude/Text/Url.cinch")),
        ("Text.Url.Codec.cinch", include_str!("../prelude/Text/Url/Codec.cinch")),
        ("Text.Url.Error.cinch", include_str!("../prelude/Text/Url/Error.cinch")),
        ("Text.Url.Ip.cinch", include_str!("../prelude/Text/Url/Ip.cinch")),
        ("Text.Url.Model.cinch", include_str!("../prelude/Text/Url/Model.cinch")),
        ("Text.Url.Parser.cinch", include_str!("../prelude/Text/Url/Parser.cinch")),
        ("Text.Url.Query.cinch", include_str!("../prelude/Text/Url/Query.cinch")),
        ("Text.Url.Resolve.cinch", include_str!("../prelude/Text/Url/Resolve.cinch")),
        ("Text.Url.Scan.cinch", include_str!("../prelude/Text/Url/Scan.cinch")),
        ("Text.Url.Userinfo.cinch", include_str!("../prelude/Text/Url/Userinfo.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row; 200 means all passed");
    common::cleanup(&dir);
}

/// Punycode encodings, decodings, refusals and host conversions match Python's bootstring codec
/// (`tooling/punycode_vectors.py`).
#[test]
fn punycode_matches_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/punycode-vectors.json")).unwrap();
    let pair = |r: &Value, i: usize| lit(r[i].as_str().unwrap());
    let mut rows = Vec::new();
    rows.extend(f["encode"].as_array().unwrap().iter().map(|r| format!("enc {} {}", pair(r, 0), pair(r, 1))));
    rows.extend(f["decode"].as_array().unwrap().iter().map(|r| match r[1].as_str() {
        Some(s) => format!("dec {} {}", pair(r, 0), lit(s)),
        None => format!("bad {}", pair(r, 0)),
    }));
    rows.extend(f["hosts"].as_array().unwrap().iter().map(|r| format!("host {} {}", pair(r, 0), pair(r, 1))));
    assert!(rows.len() < 200);
    let main = format!(r#"-- | Reference Punycode consumers.
module Main where
  import Text.Punycode
  -- | An encoding row.
  enc :: String -> String -> Bool
  enc s want = case punycode-encode s of Ok e -> e == want; Err e -> False
  -- | A decoding row.
  dec :: String -> String -> Bool
  dec s want = case punycode-decode s of Ok d -> d == want; Err e -> False
  -- | A refused decoding.
  bad :: String -> Bool
  bad s = case punycode-decode s of Ok d -> False; Err e -> not (punycode-error-message e == "")
  -- | A host conversion row.
  host :: String -> String -> Bool
  host s want = case punycode-to-ascii s of Ok a -> a == want; Err e -> False
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
  -- | Every row.
  main :: Int
  main = first [{}] 0
"#, rows.join(",\n    "));
    let dir = common::project("punycode", &[("Text.Punycode.cinch", include_str!("../prelude/Text/Punycode.cinch")), ("Main.cinch", &main)]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row; 200 means all passed");
    common::cleanup(&dir);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn cookie_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Cookie", include_str!("../prelude/Text/Cookie.cinch"));
    common::assert_documented("Data.CookieJar", include_str!("../prelude/Data/CookieJar.cinch"));
    common::assert_documented("Text.Punycode", include_str!("../prelude/Text/Punycode.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference cookie consumers.
module Main where
  import Data.CookieJar
  import Text.Cookie
  import Text.Show (int-to-string)
  import Text.Url (url-parse-text)
  -- | A flag as text.
  flag :: Bool -> String
  flag b = if b then "1" else "0"
  -- | The reference's SameSite numbering.
  same-site-number :: SameSite -> String
  same-site-number s = case s of
    SameSiteDefault -> "1"
    SameSiteLax -> "2"
    SameSiteStrict -> "3"
    SameSiteNone -> "4"
    SameSiteUnset -> "0"
  -- | Every field of a parsed cookie.
  summary :: Cookie -> String
  summary c = string-join [cookie-name c, cookie-value c, flag (cookie-quoted c), cookie-path c, cookie-domain c,
    (case cookie-expires c of Some t -> int-to-string t; None -> "-"), cookie-raw-expires c, int-to-string (cookie-max-age c),
    flag (cookie-secure c), flag (cookie-http-only c), same-site-number (cookie-same-site c), flag (cookie-partitioned c),
    string-join (cookie-unparsed c) ","] "|"
  -- | Request cookies as a Cookie header carries them.
  pairs :: List Cookie -> String
  pairs cs = string-join (pair-texts cs) "; "
  -- | Each pair's text.
  pair-texts :: List Cookie -> List String
  pair-texts cs = case cs of
    Nil -> Nil
    Cons c rest -> Cons (string-concat-all [cookie-name c, "=", (if cookie-quoted c then string-concat-all ["\"", cookie-value c, "\""] else cookie-value c)]) (pair-texts rest)
  -- | A Set-Cookie row.
  set-row :: String -> String -> Bool
  set-row line want = case cookie-parse-set line of
    Ok c -> summary c == want
    Err e -> string-concat "error: " (cookie-error-message e) == want
  -- | A strict Cookie row.
  strict-row :: String -> String -> Bool
  strict-row line want = case cookie-parse line of
    Ok cs -> pairs cs == want
    Err e -> string-concat "error: " (cookie-error-message e) == want
  -- | A lenient Cookie row.
  read-row :: String -> String -> String -> Bool
  read-row line filter want = pairs (cookie-read line filter) == want
  -- | A serialization and validation row.
  string-row :: Cookie -> String -> String -> Bool
  string-row c text valid = cookie-string c == text && (case cookie-valid c of
    Ok u -> valid == ""
    Err e -> cookie-error-message e == valid)
  -- | A jar step: store Set-Cookie lines from a response, or ask what a request sends.
  data Step where
    -- | URL, seconds, the URL's scheme, host and path, and Set-Cookie lines.
    Put :: String -> Int -> String -> String -> String -> List String -> Step
    -- | URL, seconds, the URL's scheme, host and path, and the expected Cookie header.
    Ask :: String -> Int -> String -> String -> String -> String -> Step
  -- | The clock's zero.
  base :: Int
  base = 1700000000
  -- | The lines that parse.
  parsed :: List String -> List Cookie
  parsed lines = case lines of
    Nil -> Nil
    Cons l rest -> (case cookie-parse-set l of
      Ok c -> Cons c (parsed rest)
      Err e -> parsed rest)
  -- | Run steps, through parsed URLs or the reference's URL fields.
  steps :: CookieJar -> Bool -> List Step -> Bool
  steps jar by-url xs = case xs of
    Nil -> True
    Cons x rest -> (case x of
      Put url t scheme host path lines ->
        if by-url then (case url-parse-text url of
          Ok u -> steps (jar-set-cookies-url jar u (parsed lines) (base + t)) by-url rest
          Err e -> False)
        else steps (jar-set-cookies jar scheme host path (parsed lines) (base + t)) by-url rest
      Ask url t scheme host path want ->
        if by-url then (case url-parse-text url of
          Ok u -> (case jar-cookies-url jar u (base + t) of (cs, next) -> pairs cs == want && steps next by-url rest)
          Err e -> False)
        else (case jar-cookies jar scheme host path (base + t) of
          (cs, next) -> pairs cs == want && steps next by-url rest))
  -- | Whether text ends with a suffix.
  ends :: String -> String -> Bool
  ends s x =
    let b = string-to-bytes s in
    let y = string-to-bytes x in
    bytes-len b >= bytes-len y && eq-i64 (bytes-compare (tail b (bytes-len b - bytes-len y)) y) 0
  -- | Bytes from an offset.
  tail :: Bytes -> Nat -> Bytes
  tail b o = let n = (if o <= bytes-len b then bytes-len b - o else 0) in let out = bytes-new n in let c = (if o + n <= bytes-len b then bytes-copy-range out b 0 o n else ()) in out
  -- | The text after the last dot.
  last-label :: String -> String
  last-label d = let b = string-to-bytes d in after-dot b (bytes-len b)
  -- | The text after the last dot before end.
  after-dot :: Bytes -> Nat -> String
  after-dot b end = if end == 0 then (case string-from-bytes b of Some s -> s; None -> "") else if (if end <= bytes-len b then bytes-get b (end - 1) else 0) == 46 then (case string-from-bytes (tail b end) of Some s -> s; None -> "") else after-dot b (end - 1)
  -- | The reference test's public suffix function.
  test-psl :: String -> String
  test-psl d =
    if d == "co.uk" || ends d ".co.uk" then "co.uk"
    else if d == "www.buggy.psl" then "xy"
    else if d == "www2.buggy.psl" then "com"
    else last-label d
  -- | A jar scenario, optionally through parsed URLs.
  scenario :: Bool -> Bool -> List Step -> Bool
  scenario psl by-url xs = steps (cookie-jar-new (if psl then Some test-psl else None)) by-url xs
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
