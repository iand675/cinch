//! Text templates against the reference SDK (`tooling/template_vectors.go`): fields, maps,
//! lists, range (with break and continue, maps in key order and integers), if/else-if, with/
//! else-with, variables and scoping, trim markers, comments, define, block and template calls,
//! constants, every builtin, printf verbs, flags, widths and exactly rounded floats, escaping,
//! user functions, custom delimiters and missing-key modes. Outputs are compared byte for byte;
//! failures by whether parsing or execution fails.
mod common;

use serde_json::Value;

/// Hex of a string's bytes.
fn hex(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("template", &[
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Text.Builder.cinch", include_str!("../prelude/Text/Builder.cinch")),
        ("Text.Json.cinch", include_str!("../prelude/Text/Json.cinch")),
        ("Text.Lex.cinch", include_str!("../prelude/Text/Lex.cinch")),
        ("Text.Parse.cinch", include_str!("../prelude/Text/Parse.cinch")),
        ("Text.Scan.cinch", include_str!("../prelude/Text/Scan.cinch")),
        ("Text.Template.cinch", include_str!("../prelude/Text/Template.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// Every reference template renders the same bytes or fails at the same stage.
#[test]
fn templates_match_reference_outputs() {
    let cases: Value = serde_json::from_str(include_str!("fixtures/template-vectors.json")).unwrap();
    let rows = cases.as_array().unwrap().iter().map(|c| {
        let f = |k: &str| format!("(t \"{}\")", hex(c[k].as_str().unwrap()));
        format!("case-row {} {} {} {} {} {} {}", f("text"), f("data"), f("left"), f("right"), f("missingkey"), f("output"), f("error"))
    }).collect::<Vec<_>>();
    for batch in rows.chunks(30) {
        run_batch(batch);
    }
}

/// Template sets: named execution, redefinition, lookups and formatting helpers.
#[test]
fn template_sets_and_helpers_behave() {
    run_batch(&["sets ()".to_string()]);
}

/// The delimiter search and escaping kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn template_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Data.Unicode.Kernel", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
            ("Data.Unicode.Tables", include_str!("../prelude/Data/Unicode/Tables.cinch")),
            ("Text.Builder", include_str!("../prelude/Text/Builder.cinch")),
            ("Text.Json", include_str!("../prelude/Text/Json.cinch")),
            ("Text.Lex", include_str!("../prelude/Text/Lex.cinch")),
            ("Text.Parse", include_str!("../prelude/Text/Parse.cinch")),
            ("Text.Scan", include_str!("../prelude/Text/Scan.cinch")),
            ("Text.Template", include_str!("../prelude/Text/Template.cinch")),
            ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Text.Template\n  -- | Reach the kernels.\n  main :: Int\n  main = case template-parse (template-new \"t\") \"a long run of text before the action {{html .}}\" of\n    Ok t -> (case template-execute t (TvString \"<b>\") of Ok out -> 0; Err e -> 1)\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("<16 x i8>") && ir.contains("@llvm.vector.reduce.umin.v16i8"));
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
fn template_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Template", include_str!("../prelude/Text/Template.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference template consumers.
module Main where
  import Text.Template
  import Text.Hex
  import Text.Json
  import Text.Parse (Parsed, Failed)
  import Data.Json
  import Data.Bytes.Build
  -- | A string from hex.
  t :: String -> String
  t text = case hex-decode text of Ok b -> (case string-from-bytes b of Some s -> s; None -> "invalid"); Err e -> "invalid"
  -- | Data from JSON text.
  json-data :: String -> TemplateValue
  json-data text = case parse-json "data" (string-to-bytes text) of Parsed v errors -> template-from-json v; Failed e -> TvNil
  -- | ASCII upper case.
  upper :: List TemplateValue -> Result String TemplateValue
  upper args = case args of Cons x rest -> (case x of TvString s -> Ok (TvString (upper-ascii s)); other -> Err "upper wants a string"); Nil -> Err "upper wants one argument"
  -- | Upper-case ASCII letters.
  upper-ascii :: String -> String
  upper-ascii s = case string-from-bytes (raise-bytes (string-to-bytes s) 0) of Some r -> r; None -> s
  -- | Upper-case bytes in place in a copy.
  raise-bytes :: Bytes -> Nat -> Bytes
  raise-bytes b i = if i >= bytes-len b then b else let c = bytes-get b i in let w = (if c >= 97 && c <= 122 then bytes-set b i (c - 32) else ()) in raise-bytes b (i + 1)
  -- | Integer sum of two arguments.
  add :: List TemplateValue -> Result String TemplateValue
  add args = case args of Cons x rest -> (case rest of Cons y more -> (case (x, y) of (TvInt a, TvInt b) -> Ok (TvInt (a + b)); other -> Err "add wants integers"); Nil -> Err "add wants two"); Nil -> Err "add wants two"
  -- | The set every case starts from.
  base :: String -> String -> String -> Template
  base left right mode =
    let funcs = template-funcs (template-new "t") [("upper", TemplateFunction upper), ("add", TemplateFunction add)] in
    let delimited = if left == "" then funcs else template-delims funcs left right in
    template-missing-key delimited (if mode == "error" then MissingKeyError else if mode == "zero" then MissingKeyZero else MissingKeyDefault)
  -- | One case: the output, or the stage that fails.
  case-row :: String -> String -> String -> String -> String -> String -> String -> Bool
  case-row text data-text left right mode output error = case template-parse (base left right mode) text of
    Err e -> error == "parse"
    Ok tpl -> case template-execute tpl (json-data data-text) of
      Err e -> error == "exec"
      Ok out -> error == "" && (case string-from-bytes out of Some s -> s == output; None -> False)
  -- | Named execution, definitions across parses, and helpers.
  sets :: Unit -> Bool
  sets unit = case template-parse (template-new "page") "{{define \"row\"}}<{{.}}>{{end}}{{range .}}{{template \"row\" .}}{{end}}" of
    Err e -> False
    Ok page -> case template-parse page "{{define \"row\"}}[{{.}}]{{end}}" of
      Err e -> False
      Ok again ->
        (case template-execute again (template-list [TvInt 1, TvInt 2]) of Ok out -> out == string-to-bytes "[1][2]"; Err e -> False)
          && (case template-execute-named again "row" (TvString "x") of Ok out -> out == string-to-bytes "[x]"; Err e -> False)
          && (case template-execute-named again "nope" TvNil of Err (TemplateExecError m) -> True; other -> False)
          && list-length (template-names again) == 2
          && template-sprintf "%08.3f|%x|%U|%c" [TvFloat (f64-from-decimal (string-to-bytes "-3.14159")), TvString "hi", TvInt 128512, TvInt 9731] == bytes-concat [string-to-bytes "-003.142|6869|U+1F600|", bytes-from-u8s [226, 152, 131]]
          && template-html-escape (string-to-bytes "a<b>&\"'") == string-to-bytes "a&lt;b&gt;&amp;&#34;&#39;"
          && template-is-true (template-map [("a", TvNil)]) && not (template-is-true TvListEnd)
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
