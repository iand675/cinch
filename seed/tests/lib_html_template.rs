//! Contextual HTML templates against the reference SDK (`tooling/html_template_vectors.go`):
//! HTML text, comments, double/single/unquoted attributes, attribute names, URL attributes
//! (safe schemes vs javascript:/vbscript:/data:/ftp:), srcset, `<script>` JS values/strings/
//! template literals/regexps/comments, event handlers, `<style>` and `style=` CSS contexts,
//! RCDATA (`<title>`/`<textarea>`), safe typed wrappers (`HTML`, `HTMLAttr`, `CSS`, `JS`,
//! `JSStr`, `URL`, `Srcset`), control flow (`if`, `with`, `range` with `break`/`continue`,
//! `define`, `block`, `template` with context mangling and recursion), pipelines, custom
//! delimiters, missing-key modes, and parse/escape/exec error stages.
mod common;

use serde_json::Value;

/// Hex of a string's bytes.
fn hex(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

/// Compile and run batches of fixture rows in one shared project directory so library interfaces
/// compile once; each batch exits 200 if every row passes, or the first failing row index.
fn run_batches(batches: &[(&[String], &[String])]) {
    let dir = common::project("html_template", &[
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Text.Builder.cinch", include_str!("../prelude/Text/Builder.cinch")),
        ("Text.Json.cinch", include_str!("../prelude/Text/Json.cinch")),
        ("Text.Lex.cinch", include_str!("../prelude/Text/Lex.cinch")),
        ("Text.Parse.cinch", include_str!("../prelude/Text/Parse.cinch")),
        ("Text.Scan.cinch", include_str!("../prelude/Text/Scan.cinch")),
        ("Text.Html.cinch", include_str!("../prelude/Text/Html.cinch")),
        ("Text.Template.cinch", include_str!("../prelude/Text/Template.cinch")),
        ("Text.HtmlTemplate.cinch", include_str!("../prelude/Text/HtmlTemplate.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
    ]);
    for (batch_idx, (rows, names)) in batches.iter().enumerate() {
        let main_src = format!(
            "{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n",
            rows.join(",\n    ")
        );
        std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main.cinch");
        let code = common::run(&dir, "Main.cinch");
        if code != 200 {
            common::cleanup(&dir);
            let failed = names.get(code as usize).map(|s| s.as_str()).unwrap_or("<unknown>");
            panic!("batch {batch_idx}: row {code} ({failed}) failed (expected 200)");
        }
    }
    common::cleanup(&dir);
}

/// Every reference HTML template renders the exact same bytes or fails at the same stage.
#[test]
fn html_templates_match_reference_outputs() {
    let cases: Value = serde_json::from_str(include_str!("fixtures/html-template-vectors.json")).unwrap();
    let arr = cases.as_array().unwrap();
    assert!(arr.len() >= 100, "expected at least 100 reference vectors, got {}", arr.len());
    let rows = arr.iter().map(|c| {
        let f = |k: &str| format!("(t \"{}\")", hex(c[k].as_str().unwrap()));
        format!("case-row {} {} {} {} {} {} {}", f("text"), f("data"), f("left"), f("right"), f("missingkey"), f("output"), f("error"))
    }).collect::<Vec<_>>();
    let names = arr.iter().map(|c| c["name"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    let batches: Vec<(&[String], &[String])> = rows.chunks(30).zip(names.chunks(30)).collect();
    run_batches(&batches);
}

/// Template sets: named execution, redefinition, safe helpers, JSON bridge, and standalone escapers.
#[test]
fn html_template_sets_and_helpers_behave() {
    let rows = vec!["sets ()".to_string()];
    let names = vec!["sets".to_string()];
    run_batches(&[(&rows, &names)]);
}

/// Every public and private declaration has canonical module-owned documentation and uses zero inline LLVM.
#[test]
fn html_template_documented_and_pure_cinch() {
    let src = include_str!("../prelude/Text/HtmlTemplate.cinch");
    common::assert_documented("Text.HtmlTemplate", src);
    assert!(!src.contains("llvm {"), "Text.HtmlTemplate must not contain inline LLVM");
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference HTML template consumers.
module Main where
  import Text.HtmlTemplate
  import Text.Template (
    TemplateValue, TvNil, TvBool, TvInt, TvFloat, TvString,
    MissingKey, MissingKeyDefault, MissingKeyZero, MissingKeyError
  )
  import Text.Hex
  import Text.Json
  import Text.Parse (Parsed, Failed)
  import Data.Json
  -- | A string from hex.
  t :: String -> String
  t text = case hex-decode text of Ok b -> (case string-from-bytes b of Some s -> s; None -> "invalid"); Err e -> "invalid"
  -- | Convert parsed JSON to `HtmlValue`, recognizing safe content markers.
  convert-json :: Json -> HtmlValue
  convert-json j = case j of
    JNull -> html-from-template TvNil
    JBool b -> html-from-template (TvBool b)
    JNum n -> html-from-template (TvInt n)
    JFloat f -> html-from-template (TvFloat f)
    JStr s -> html-from-template (TvString s)
    JArr item rest -> HvList (convert-json item) (convert-json rest)
    JArrEnd -> HvListEnd
    JObj key val JObjEnd -> case val of
      JStr s ->
        if key == "$html" then safe-html s
        else if key == "$htmlAttr" then safe-html-attr s
        else if key == "$css" then safe-css s
        else if key == "$js" then safe-js s
        else if key == "$jsStr" then safe-js-str s
        else if key == "$url" then safe-url s
        else if key == "$srcset" then safe-srcset s
        else html-map [(key, html-from-template (TvString s))]
      other -> html-map [(key, convert-json val)]
    JObj key val rest -> hmap-put (convert-json rest) key (convert-json val)
    JObjEnd -> HvMapEnd
  -- | Insert a key into a sorted `HtmlValue` map spine.
  hmap-put :: HtmlValue -> String -> HtmlValue -> HtmlValue
  hmap-put m k v = case m of
    HvMap key value rest ->
      if eq-i64 (string-compare k key) 0 then HvMap k v rest
      else if lt-i64 (string-compare k key) 0 then HvMap k v m
      else HvMap key value (hmap-put rest k v)
    other -> HvMap k v HvMapEnd
  -- | Data from JSON text.
  json-data :: String -> HtmlValue
  json-data text = case parse-json "data" (string-to-bytes text) of Parsed v errors -> convert-json v; Failed e -> html-from-template TvNil
  -- | Extract a string argument for custom test functions.
  arg-str :: List HtmlValue -> Result String String
  arg-str args = case args of
    Cons x Nil -> case x of
      HvTemplate (TvString s) -> Ok s
      HvSafeHtml s -> Ok s
      HvSafeHtmlAttr s -> Ok s
      HvSafeCss s -> Ok s
      HvSafeJs s -> Ok s
      HvSafeJsStr s -> Ok s
      HvSafeUrl s -> Ok s
      HvSafeSrcset s -> Ok s
      other -> Err "expected string"
    other -> Err "expected one argument"
  -- | ASCII upper case.
  upper-fn :: List HtmlValue -> Result String HtmlValue
  upper-fn args = case arg-str args of
    Err e -> Err e
    Ok s -> Ok (html-from-template (TvString (upper-ascii s)))
  -- | Upper-case ASCII letters.
  upper-ascii :: String -> String
  upper-ascii s = case string-from-bytes (raise-bytes (string-to-bytes s) 0) of Some r -> r; None -> s
  -- | Upper-case bytes in place in a fresh buffer.
  raise-bytes :: Bytes -> Nat -> Bytes
  raise-bytes b i = if i >= bytes-len b then b else let c = bytes-get b i in let w = (if c >= 97 && c <= 122 then bytes-set b i (c - 32) else ()) in raise-bytes b (i + 1)
  -- | Integer sum of two arguments.
  add-fn :: List HtmlValue -> Result String HtmlValue
  add-fn args = case args of
    Cons (HvTemplate (TvInt a)) (Cons (HvTemplate (TvInt b)) Nil) -> Ok (html-from-template (TvInt (a + b)))
    other -> Err "add wants two integers"
  -- | Wrap string as safe HTML.
  mk-safe-html :: List HtmlValue -> Result String HtmlValue
  mk-safe-html args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-html s)
  -- | Wrap string as safe HTMLAttr.
  mk-safe-attr :: List HtmlValue -> Result String HtmlValue
  mk-safe-attr args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-html-attr s)
  -- | Wrap string as safe CSS.
  mk-safe-css :: List HtmlValue -> Result String HtmlValue
  mk-safe-css args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-css s)
  -- | Wrap string as safe JS.
  mk-safe-js :: List HtmlValue -> Result String HtmlValue
  mk-safe-js args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-js s)
  -- | Wrap string as safe JSStr.
  mk-safe-js-str :: List HtmlValue -> Result String HtmlValue
  mk-safe-js-str args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-js-str s)
  -- | Wrap string as safe URL.
  mk-safe-url :: List HtmlValue -> Result String HtmlValue
  mk-safe-url args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-url s)
  -- | Wrap string as safe Srcset.
  mk-safe-srcset :: List HtmlValue -> Result String HtmlValue
  mk-safe-srcset args = case arg-str args of Err e -> Err e; Ok s -> Ok (safe-srcset s)
  -- | The template set every vector starts from.
  base :: String -> String -> String -> HtmlTemplate
  base left right mode =
    let funcs = html-template-funcs (html-template-new "t") [
      ("upper", HtmlTemplateFunction upper-fn),
      ("add", HtmlTemplateFunction add-fn),
      ("safeHTML", HtmlTemplateFunction mk-safe-html),
      ("safeHTMLAttr", HtmlTemplateFunction mk-safe-attr),
      ("safeCSS", HtmlTemplateFunction mk-safe-css),
      ("safeJS", HtmlTemplateFunction mk-safe-js),
      ("safeJSStr", HtmlTemplateFunction mk-safe-js-str),
      ("safeURL", HtmlTemplateFunction mk-safe-url),
      ("safeSrcset", HtmlTemplateFunction mk-safe-srcset)
    ] in
    let delimited = if left == "" then funcs else html-template-delims funcs left right in
    html-template-missing-key delimited (if mode == "error" then MissingKeyError else if mode == "zero" then MissingKeyZero else MissingKeyDefault)
  -- | Verify one reference case: exact output bytes, or exact error stage.
  case-row :: String -> String -> String -> String -> String -> String -> String -> Bool
  case-row text data-text left right mode output error = case html-template-parse (base left right mode) text of
    Err (HtmlTemplateParseError m) -> error == "parse"
    Err other -> False
    Ok tpl -> case html-template-execute tpl (json-data data-text) of
      Err (HtmlTemplateEscapeError m) -> error == "escape"
      Err (HtmlTemplateExecError m) -> error == "exec"
      Err (HtmlTemplateParseError m) -> False
      Ok out -> error == "" && (case string-from-bytes out of Some s -> s == output; None -> False)
  -- | Verify template set operations, JSON bridge, and standalone escapers.
  sets :: Unit -> Bool
  sets unit = case html-template-parse (html-template-new "page") "{{define \"row\"}}<li>{{.}}</li>{{end}}<ul>{{range .}}{{template \"row\" .}}{{end}}</ul>" of
    Err e -> False
    Ok page -> case html-template-parse page "{{define \"row\"}}<span>{{.}}</span>{{end}}" of
      Err e -> False
      Ok again ->
        (case html-template-execute again (html-list [html-from-template (TvString "<a>"), safe-html "<b>ok</b>"]) of
          Ok out -> out == string-to-bytes "<ul><span>&lt;a&gt;</span><span><b>ok</b></span></ul>"
          Err e -> False)
          && (case html-template-execute-named again "row" (html-from-template (TvString "<x>")) of
            Ok out -> out == string-to-bytes "<span>&lt;x&gt;</span>"
            Err e -> False)
          && (case html-template-execute-named again "nope" (html-from-template TvNil) of
            Err (HtmlTemplateExecError m) -> string-byte-length (html-template-error-message (HtmlTemplateExecError m)) > 0
            other -> False)
          && html-template-name again == "page"
          && list-length (html-template-names again) == 2
          && html-escape-string "a<b>&\"'" == "a&lt;b&gt;&amp;&#34;&#39;"
          && js-escape-string "a\"'<>\\n" == "a\\\"\\'\\u003C\\u003E\\\\n"
          && url-query-escape-string "a b&c" == "a+b%26c"
          && html-is-true (html-map [("a", html-from-template TvNil)])
          && not (html-is-true HvListEnd)
          && (case parse-json "j" (string-to-bytes "{\"x\":\"<hi>\"}") of
            Parsed v errs -> (case html-template-parse (html-template-new "j") "<p>{{.x}}</p>" of
              Ok jt -> (case html-template-execute jt (html-value-from-json v) of
                Ok out -> out == string-to-bytes "<p>&lt;hi&gt;</p>"
                Err e -> False)
              Err e -> False)
            Failed e -> False)
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
