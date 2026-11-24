//! Mail addresses, address formatting, message header blocks and dates against reference
//! vectors from `tooling/mail_vectors.go`: RFC 5322 address lists and single addresses with
//! groups, comments, quoted strings, domain literals, UTF-8 and encoded words; quoting and
//! encoded-word formatting; lenient header blocks; and dates with numeric and named zones.
//! Errors are compared by kind. The atext and qtext scanners also lower for other targets.
mod common;

use serde_json::Value;

/// Hex of a string's UTF-8 bytes.
fn hex(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

/// A Cinch string literal built from hex, so no escaping is involved.
fn text(v: &Value) -> String {
    format!("(t \"{}\")", hex(v.as_str().unwrap()))
}

/// An expected address list.
fn list(v: &Value) -> String {
    let items = v.as_array().unwrap().iter().map(|a| format!("({}, {})", text(&a["name"]), text(&a["address"]))).collect::<Vec<_>>();
    format!("[{}]", items.join(", "))
}

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("mail", &[
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
        ("Data.Time.Date.cinch", include_str!("../prelude/Data/Time/Date.cinch")),
        ("Data.Time.DateTime.cinch", include_str!("../prelude/Data/Time/DateTime.cinch")),
        ("Data.Time.Duration.cinch", include_str!("../prelude/Data/Time/Duration.cinch")),
        ("Data.Time.Monotonic.cinch", include_str!("../prelude/Data/Time/Monotonic.cinch")),
        ("Data.Time.Offset.cinch", include_str!("../prelude/Data/Time/Offset.cinch")),
        ("Data.Time.TimeOfDay.cinch", include_str!("../prelude/Data/Time/TimeOfDay.cinch")),
        ("Data.Time.Timestamp.cinch", include_str!("../prelude/Data/Time/Timestamp.cinch")),
        ("Data.Time.Weekday.cinch", include_str!("../prelude/Data/Time/Weekday.cinch")),
        ("Data.Time.Zone.cinch", include_str!("../prelude/Data/Time/Zone.cinch")),
        ("Data.Time.Zone.Posix.cinch", include_str!("../prelude/Data/Time/Zone/Posix.cinch")),
        ("Data.Time.Zone.Types.cinch", include_str!("../prelude/Data/Time/Zone/Types.cinch")),
        ("Data.Time.Zoned.cinch", include_str!("../prelude/Data/Time/Zoned.cinch")),
        ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Effect.Time.cinch", include_str!("../prelude/Effect/Time.cinch")),
        ("Text.Base64.Encoding.cinch", include_str!("../prelude/Text/Base64/Encoding.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Mail.cinch", include_str!("../prelude/Text/Mail.cinch")),
        ("Text.Mime.cinch", include_str!("../prelude/Text/Mime.cinch")),
        ("Text.TextProto.cinch", include_str!("../prelude/Text/TextProto.cinch")),
        ("Text.Time.Rfc2822.cinch", include_str!("../prelude/Text/Time/Rfc2822.cinch")),
        ("Text.Time.Scan.cinch", include_str!("../prelude/Text/Time/Scan.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/mail-vectors.json")).unwrap()
}

/// Address lists parse to the reference addresses or fail with the reference kind.
#[test]
fn mail_address_lists_match_reference() {
    let f = fixture();
    let rows = f["lists"].as_array().unwrap().iter().map(|c| format!("many (b \"{}\") {:?} {}", c["hex"].as_str().unwrap(), c["error"].as_str().unwrap(), list(&c["list"]))).collect::<Vec<_>>();
    for batch in rows.chunks(45) {
        run_batch(batch);
    }
}

/// Single addresses, formatting, header blocks and dates match the reference.
#[test]
fn mail_single_addresses_formatting_messages_and_dates_match_reference() {
    let f = fixture();
    let mut rows = f["singles"].as_array().unwrap().iter().map(|c| format!("one (b \"{}\") {:?} {}", c["hex"].as_str().unwrap(), c["error"].as_str().unwrap(), list(&c["list"]))).collect::<Vec<_>>();
    rows.extend(f["formats"].as_array().unwrap().iter().map(|c| format!("shows {} {} {}", text(&c["name"]), text(&c["address"]), text(&c["text"]))));
    rows.extend(f["messages"].as_array().unwrap().iter().map(|c| {
        let fields = c["fields"].as_array().unwrap().iter().map(|kv| {
            let kv = kv.as_array().unwrap();
            format!("({}, [{}])", text(&kv[0]), kv[1..].iter().map(text).collect::<Vec<_>>().join(", "))
        }).collect::<Vec<_>>();
        format!("reads (b \"{}\") {:?} [{}] {}", hex(c["input"].as_str().unwrap()), c["error"].as_str().unwrap(), fields.join(", "), text(&c["body"]))
    }));
    rows.extend(f["dates"].as_array().unwrap().iter().map(|c| format!("dated {} {:?} ({}) ({})", text(&c["input"]), c["error"].as_str().unwrap(), c["unix"], c["offset"])));
    rows.push("named-zones ()".to_string());
    rows.push("accessors ()".to_string());
    run_batch(&rows);
}

/// The atext and qtext scanners lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn mail_kernels_compile_for_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
            .unwrap_or_else(|_| {
                let mut profile = cinch::core::TargetProfile::native();
                profile.triple = triple.into();
                profile.arch = arch.into();
                profile.endian = endian.into();
                profile
            });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Utf8", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Data.IpAddr", include_str!("../prelude/Data/IpAddr.cinch")),
            ("Data.Time.Date", include_str!("../prelude/Data/Time/Date.cinch")),
            ("Data.Time.DateTime", include_str!("../prelude/Data/Time/DateTime.cinch")),
            ("Data.Time.Duration", include_str!("../prelude/Data/Time/Duration.cinch")),
            ("Data.Time.Monotonic", include_str!("../prelude/Data/Time/Monotonic.cinch")),
            ("Data.Time.Offset", include_str!("../prelude/Data/Time/Offset.cinch")),
            ("Data.Time.TimeOfDay", include_str!("../prelude/Data/Time/TimeOfDay.cinch")),
            ("Data.Time.Timestamp", include_str!("../prelude/Data/Time/Timestamp.cinch")),
            ("Data.Time.Weekday", include_str!("../prelude/Data/Time/Weekday.cinch")),
            ("Data.Time.Zone", include_str!("../prelude/Data/Time/Zone.cinch")),
            ("Data.Time.Zone.Posix", include_str!("../prelude/Data/Time/Zone/Posix.cinch")),
            ("Data.Time.Zone.Types", include_str!("../prelude/Data/Time/Zone/Types.cinch")),
            ("Data.Time.Zoned", include_str!("../prelude/Data/Time/Zoned.cinch")),
            ("Data.Unicode.Kernel", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
            ("Data.Unicode.Tables", include_str!("../prelude/Data/Unicode/Tables.cinch")),
            ("Effect.Time", include_str!("../prelude/Effect/Time.cinch")),
            ("Text.Base64.Encoding", include_str!("../prelude/Text/Base64/Encoding.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
            ("Text.Mail", include_str!("../prelude/Text/Mail.cinch")),
            ("Text.Mime", include_str!("../prelude/Text/Mime.cinch")),
            ("Text.TextProto", include_str!("../prelude/Text/TextProto.cinch")),
            ("Text.Time.Rfc2822", include_str!("../prelude/Text/Time/Rfc2822.cinch")),
            ("Text.Time.Scan", include_str!("../prelude/Text/Time/Scan.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Text.Mail\n  -- | Reach the kernels.\n  main :: Int\n  main = case mail-parse-address \"\\\"A long quoted display name\\\" <a.very.long.local.part@example.com>\" of\n    Ok a -> 0\n    Err e -> 1\n").unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>"));
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn mail_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Mail", include_str!("../prelude/Text/Mail.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference mail vector consumers.
module Main where
  import Text.Mail
  import Text.Hex
  import Text.TextProto
  import Data.Time.Timestamp
  import Data.Time.Offset
  -- | Bytes from hex.
  b :: String -> Bytes
  b hex = case hex-decode hex of Ok v -> v; Err e -> bytes-new 0
  -- | A string from hex.
  t :: String -> String
  t hex = case string-from-bytes (b hex) of Some s -> s; None -> "invalid"
  -- | The fixture's kind name for an error.
  kind :: MailError -> String
  kind e = case e of
    MailNoAddress -> "no-address"
    MailExpectedComma -> "expected-comma"
    MailExpectedSingleAddress -> "expected-single"
    MailEmptyGroup -> "empty-group"
    MailGroupWithMultipleAddresses -> "group-multiple"
    MailMissingAtOrAngleAddress -> "missing-at-or-angle"
    MailNoAngleAddress -> "no-angle"
    MailUnclosedAngleAddress -> "unclosed-angle"
    MailBadComment -> "bad-comment"
    MailNoAddrSpec -> "no-addr-spec"
    MailEmptyQuotedLocalPart -> "empty-quoted-local"
    MailMissingAtInAddrSpec -> "missing-at-in-spec"
    MailNoDomain -> "no-domain"
    MailInvalidAtom -> "invalid-atom"
    MailLeadingDot -> "leading-dot"
    MailDoubleDot -> "double-dot"
    MailTrailingDot -> "trailing-dot"
    MailInvalidUtf8 at -> "invalid-utf8"
    MailUnclosedQuotedString -> "unclosed-quoted"
    MailBadQuotedCharacter at -> "bad-quoted-char"
    MailUnclosedDomainLiteral -> "unclosed-literal"
    MailBadDomainLiteralCharacter at -> "bad-literal-char"
    MailInvalidDomainLiteral -> "invalid-literal"
    MailMissingWordInPhrase inner -> string-concat "phrase:" (kind inner)
    MailUnknownCharset name -> "charset"
    MailHeaderNotPresent -> "not-present"
    MailMalformedInitialLine -> "malformed-initial"
    MailMalformedHeaderLine at -> "malformed-line"
    MailEmptyMessage -> "empty-message"
    MailBareCarriageReturn -> "bare-cr"
    MailDateSyntax cause -> "date"
  -- | Addresses equal the expected (name, addr-spec) pairs in order.
  same :: List MailAddress -> List (String, String) -> Bool
  same got want = case got of
    Nil -> (case want of Nil -> True; Cons w rest -> False)
    Cons a more -> case want of
      Nil -> False
      Cons w rest -> case w of (name, addr) -> mail-address-name a == name && mail-address-spec a == addr && same more rest
  -- | An address list row.
  many :: Bytes -> String -> List (String, String) -> Bool
  many input error want = case mail-parse-address-list-bytes input of
    Ok got -> error == "" && same got want
    Err e -> kind e == error
  -- | A single-address row.
  one :: Bytes -> String -> List (String, String) -> Bool
  one input error want = case mail-parse-address-bytes input of
    Ok got -> error == "" && same (Cons got Nil) want
    Err e -> kind e == error
  -- | A formatting row.
  shows :: String -> String -> String -> Bool
  shows name addr expected = mail-address-to-string (mail-address name addr) == expected
  -- | A message row: the same field names with the same values, and the same body.
  reads :: Bytes -> String -> List (String, List String) -> String -> Bool
  reads input error fields body = case mail-read-message input of
    Err e -> kind e == error
    Ok m -> error == "" && list-length (header-keys (mail-message-header m)) == list-length fields && fields-match m fields
      && text-is (mail-message-body m) body
  -- | Every expected field has exactly the expected values.
  fields-match :: MailMessage -> List (String, List String) -> Bool
  fields-match m fields = case fields of
    Nil -> True
    Cons f rest -> case f of (key, values) -> values-match (header-values (mail-message-header m) (string-to-bytes key)) values && fields-match m rest
  -- | Values equal in order.
  values-match :: List Bytes -> List String -> Bool
  values-match got want = case got of
    Nil -> (case want of Nil -> True; Cons w rest -> False)
    Cons g more -> case want of Nil -> False; Cons w rest -> text-is g w && values-match more rest
  -- | Whether bytes spell a string.
  text-is :: Bytes -> String -> Bool
  text-is got want = case string-from-bytes got of Some s -> s == want; None -> False
  -- | A date row: the instant and the offset in seconds.
  dated :: String -> String -> I64 -> I64 -> Bool
  dated input error unix offset = case mail-parse-date input of
    Err e -> kind e == error
    Ok found -> case found of (stamp, off) -> error == "" && timestamp-to-unix-seconds stamp == unix && offset-to-seconds off == offset
  -- | The obsolete North American zone names carry their RFC 5322 offsets.
  named-zones :: Unit -> Bool
  named-zones unit =
    dated "Fri, 21 Nov 1997 09:55:06 CST" "" 880127706 (-21600)
      && dated "Fri, 21 Nov 1997 09:55:06 EDT" "" 880120506 (-14400)
      && dated "Fri, 21 Nov 97 09:55:06 PST" "" 880134906 (-28800)
      && dated "Fri, 21 Nov 1997 09:55:06 XYZ" "date" 0 0
  -- | Header accessors and error messages.
  accessors :: Unit -> Bool
  accessors unit = (case mail-parse-address-list "Ann <a@b>, c@d" of Ok l -> list-length l == 2; Err e -> False)
    && (case mail-parse-address "Ann <a@b>" of Ok a -> mail-address-name a == "Ann"; Err e -> False)
    && (case mail-read-message (string-to-bytes "From: Ann <a@b>, c@d\r\nDate: Tue, 1 Jul 2003 10:52:37 +0200\r\n\r\nhi") of
    Err e -> False
    Ok m ->
      mail-header-get m "from" == "Ann <a@b>, c@d" && mail-header-get m "Cc" == ""
        && (case mail-header-address-list m "From" of Ok l -> list-length l == 2; Err e -> False)
        && (case mail-header-address-list m "To" of Err MailHeaderNotPresent -> True; other -> False)
        && (case mail-header-date m of Ok (stamp, off) -> timestamp-to-unix-seconds stamp == 1057049557; Err e -> False)
        && mail-error-message (MailMissingWordInPhrase MailUnclosedQuotedString) == "mail: missing word in phrase: mail: unclosed quoted-string"
        && mail-error-message (MailInvalidUtf8 3) == "mail: invalid utf-8 at byte 3")
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
