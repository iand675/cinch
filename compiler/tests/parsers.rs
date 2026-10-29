//! P30 parsing toolkit tests: `Text.Lex` (token-level lexing and the
//! layout pass) and `Text.Parse` (parser combinators with source
//! spans, typed `Parse`-effect errors with recovery handlers, and
//! megaparsec-style error bundles).
//!
//! The no-drift rule is machine-checked here: `Text.Lex`'s token
//! stream (kind, spelling, line, column, layout-inserted flag) must
//! agree token for token with the bootstrap compiler's own lexer and
//! layout pass (`cinch::lexer::lex` + `cinch::layout::layout`) over
//! the shared corpus, and the compiler is the reference where the two
//! could disagree. The expected streams below are generated from the
//! compiler at test time, so a change to either side surfaces as a
//! failure rather than a silent divergence.
//!
//! Every program is a real `Main` compiled and run through the same
//! driver the CLI uses. The toolkit modules beside it are the checked
//! in `compiler/prelude/Text/*.cinch` sources (the driver does not
//! embed them yet: see the P30 note in the report).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;

const LEX_SRC: &str = include_str!("../prelude/Text/Lex.cinch");
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");
const PARSE_SRC: &str = include_str!("../prelude/Text/Parse.cinch");

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

/// A self-contained `(chunk-bytes ...)` expression over the source's
/// bytes (string literals are runtime `String` values,
/// Data.String). The bytes are chunked into small `bytes-from-list`
/// literals: the checker recurses per literal element, so one giant
/// literal overflows it.
fn bytes_lit(s: &str) -> String {
    let chunks: Vec<String> = s
        .as_bytes()
        .chunks(100)
        .map(|c| {
            let nums: Vec<String> = c.iter().map(|b| b.to_string()).collect();
            format!("bytes-from-list [{}]", nums.join(", "))
        })
        .collect();
    format!("(chunk-bytes [{}])", chunks.join(", "))
}

fn string_lit(s: &str) -> String {
    format!("(case string-from-bytes {} of Some s -> s; None -> empty-string)", bytes_lit(s))
}

/// The shared program header: the imports and the byte helpers the
/// generated literals need. (Line-by-line assembly: a `\` string
/// continuation would eat the indentation.)
fn program_header() -> String {
    let mut s = String::new();
    for line in [
        "module Main where",
        "  import Text.Lex",
        "  import Text.Parse",
        "  import Data.Bool",
        "  import Data.Bytes",
        "  import Data.List",
        "  import Data.String",
        "  import Data.Nat",
        "  import Data.Int",
        "  import Text.Show",
        "",
        "  cat-bytes :: Bytes -> Bytes -> Bytes",
        "  cat-bytes a b =",
        "    let out = bytes-new (bytes-len a + bytes-len b) in",
        "    let u = copy-bytes-into out a 0 0 (bytes-len a) in",
        "    let v = copy-bytes-into out b (bytes-len a) 0 (bytes-len b) in",
        "    out",
        "",
        "  chunk-bytes :: List Bytes -> Bytes",
        "  chunk-bytes cs = case cs of",
        "    Nil -> bytes-new 0",
        "    Cons c rest -> cat-bytes c (chunk-bytes rest)",
        "",
        "  int-of :: Nat -> Int",
        "  int-of n = int-from-i64 (nat-to-i64 n)",
        "",
    ] {
        s.push_str(line);
        s.push('\n');
    }
    s
}

/// Escape a spelling's `\` and newline characters as `\\` and `\n`,
/// matching `Text.Lex.render-token` (one token per rendered line).
fn escape_spelling(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            other => vec![other],
        })
        .collect()
}

/// The canonical `kind:spelling@line:col` rendering, matching
/// `Text.Lex.render-token` exactly (that is the drift contract).
fn canon(tok: &cinch::lexer::Token) -> String {
    use cinch::lexer::Tok;
    let (kind, spelling): (&str, String) = match &tok.tok {
        Tok::VarId(s) => ("var", s.clone()),
        Tok::ConId(s) => ("con", s.clone()),
        Tok::Op(s) => ("op", s.clone()),
        Tok::Int(s) => ("int", s.clone()),
        Tok::FloatLit(s) => ("float", s.clone()),
        Tok::StrLit(s) => ("str", s.clone()),
        Tok::CharLit(s) => ("char", s.clone()),
        Tok::SymLit(_) | Tok::TypeNameLit(_) | Tok::Attr { .. } | Tok::Fragment(_) => {
            panic!("tick name forms, attributes and prim fragments are named refusals in Text.Lex")
        }
        Tok::Module => ("kw", "module".into()),
        Tok::Where => ("kw", "where".into()),
        Tok::Data => ("kw", "data".into()),
        Tok::Record => ("kw", "record".into()),
        Tok::Codata => ("kw", "codata".into()),
        Tok::Effect => ("kw", "effect".into()),
        Tok::Signature => ("kw", "signature".into()),
        Tok::Instance => ("kw", "instance".into()),
        Tok::Implicit => ("kw", "implicit".into()),
        Tok::Import => ("kw", "import".into()),
        Tok::As => ("kw", "as".into()),
        Tok::TypeKw => ("kw", "type".into()),
        Tok::Opaque => ("kw", "opaque".into()),
        Tok::Forall => ("kw", "forall".into()),
        Tok::Sigma => ("kw", "Sigma".into()),
        Tok::Using => ("kw", "using".into()),
        Tok::In => ("kw", "in".into()),
        Tok::Let => ("kw", "let".into()),
        Tok::Letrec => ("kw", "letrec".into()),
        Tok::If => ("kw", "if".into()),
        Tok::Then => ("kw", "then".into()),
        Tok::Else => ("kw", "else".into()),
        Tok::Case => ("kw", "case".into()),
        Tok::Of => ("kw", "of".into()),
        Tok::Do => ("kw", "do".into()),
        Tok::Handle => ("kw", "handle".into()),
        Tok::With => ("kw", "with".into()),
        Tok::Rewrite => ("kw", "rewrite".into()),
        Tok::Target => ("kw", "target".into()),
        Tok::Justify => ("kw", "justify".into()),
        Tok::MatchKw => ("kw", "match".into()),
        Tok::Replace => ("kw", "replace".into()),
        Tok::Require => ("kw", "require".into()),
        Tok::Upgrade => ("kw", "upgrade".into()),
        Tok::Prim => ("kw", "prim".into()),
        Tok::Law => ("kw", "law".into()),
        Tok::Contract => ("kw", "contract".into()),
        Tok::To => ("kw", "to".into()),
        Tok::Generate => ("kw", "generate".into()),
        Tok::QuoteDecls => ("kw", "quoteDecls".into()),
        Tok::Foreign => ("kw", "foreign".into()),
        Tok::Export => ("kw", "export".into()),
        Tok::Ccall => ("kw", "ccall".into()),
        Tok::Component => ("kw", "component".into()),
        Tok::Proof => ("kw", "proof".into()),
        Tok::Spec => ("kw", "spec".into()),
        Tok::Stage => ("kw", "stage".into()),
        Tok::Corec => ("kw", "corec".into()),
        Tok::Macro => ("kw", "macro".into()),
        Tok::Infix => ("kw", "infix".into()),
        Tok::Infixl => ("kw", "infixl".into()),
        Tok::Infixr => ("kw", "infixr".into()),
        Tok::Erased => ("kw", "erased".into()),
        Tok::Own => ("kw", "own".into()),
        Tok::Return => ("kw", "return".into()),
        Tok::Resume => ("kw", "resume".into()),
        Tok::ColonColon => ("punct", "::".into()),
        Tok::Arrow => ("punct", "->".into()),
        Tok::LArrow => ("punct", "<-".into()),
        Tok::Darrow => ("punct", "=>".into()),
        Tok::LQuote => ("punct", "[|".into()),
        Tok::RQuote => ("punct", "|]".into()),
        Tok::DollarParen => ("punct", "$(".into()),
        Tok::MultZero => ("punct", "%0".into()),
        Tok::MultOne => ("punct", "%1".into()),
        Tok::MultAff => ("punct", "%aff".into()),
        Tok::MultMany => ("punct", "%many".into()),
        Tok::Bang => ("punct", "!".into()),
        Tok::At => ("punct", "@".into()),
        Tok::Dot => ("punct", ".".into()),
        Tok::DotDot => ("punct", "..".into()),
        Tok::Equals => ("punct", "=".into()),
        Tok::Pipe => ("punct", "|".into()),
        Tok::Dollar => ("punct", "$".into()),
        Tok::Backslash => ("punct", "\\".into()),
        Tok::Tilde => ("punct", "~".into()),
        Tok::LParen => ("punct", "(".into()),
        Tok::RParen => ("punct", ")".into()),
        Tok::LBracket => ("punct", "[".into()),
        Tok::RBracket => ("punct", "]".into()),
        Tok::LBrace => ("punct", "{".into()),
        Tok::RBrace => ("punct", "}".into()),
        Tok::Comma => ("punct", ",".into()),
        Tok::Semicolon => ("punct", ";".into()),
        Tok::Underscore => ("punct", "_".into()),
        Tok::Eof => ("eof", String::new()),
    };
    let mut out = format!(
        "{kind}:{}@{}:{}",
        escape_spelling(&spelling),
        tok.span.line,
        tok.span.col
    );
    if tok.virtual_ {
        out.push_str("!v");
    }
    out
}

/// The compiler's own token stream for a source, rendered canonically
/// and newline joined: the drift test's expected value.
fn expected_stream(src: &str) -> String {
    let toks = cinch::lexer::lex(src).expect("corpus entry must lex");
    let toks = cinch::layout::layout(toks).expect("corpus entry must lay out");
    toks.iter().map(canon).collect::<Vec<_>>().join("\n")
}

fn write_project(name: &str, main: &str) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "cinch_parsers_{}_{}_{}",
        name,
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Text.Lex.cinch"), LEX_SRC).expect("write Text.Lex");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN_SRC).expect("write Text.Scan");
    std::fs::write(dir.join("Text.Parse.cinch"), PARSE_SRC).expect("write Text.Parse");
    std::fs::write(dir.join("main.cinch"), main).expect("write main");
    dir
}

/// Run a `Main` through the CLI's driver; the exit code is the
/// program's `main :: Int` result (0 on success, a case number on a
/// named failure). The driver runs on a thread with a large stack:
/// the checker recurses over a program's elaborated code, and the
/// toolkit programs are bigger than a default test thread's stack
/// (the CLI's 8MB main thread compiles them fine).
fn run_main(name: &str, main: &str) -> i32 {
    let main = main.to_string();
    let name = name.to_string();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let dir = write_project(&name, &main);
            let result = driver(Mode::Run, &dir.join("main.cinch"), None);
            let _ = std::fs::remove_dir_all(&dir);
            match result {
                Ok(code) => code,
                Err(e) => panic!("{name}: compilation failed: {e}"),
            }
        })
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
}

fn expect_exit(name: &str, main: &str, expected: i32) {
    let code = run_main(name, main);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

// ----------------------------------------------------------------
// Drift: Text.Lex against the compiler's lexer and layout pass
// ----------------------------------------------------------------

/// The shared corpus: every lexical rule of "Layout and lexing" and
/// the local amendments, plus the real example files the bootstrap
/// grammar test parses. Case order is the failure code order (1-based).
fn drift_corpus() -> Vec<(&'static str, String)> {
    let mut cases: Vec<(&'static str, String)> = Vec::new();
    let mut push = |name: &'static str, src: String| cases.push((name, src));

    push("fib-example", include_str!("../../examples/compiler/fib.cinch").to_string());
    push("classes-example", include_str!("../../examples/compiler/classes.cinch").to_string());

    // Identifiers, dashes and minus (amendment section 8).
    push(
        "dashes-and-minus",
        "x-y x-1 1-2 -42 f -42 f ()-1 x.y-1 nat-div x' foo'bar\n".to_string(),
    );
    // Number forms: canonical decimal, hex folds, floats, signs.
    push(
        "numbers",
        "007 0x1f 0x_1_F 1_000 1.5 1.5e10 1.5E+3 -3.25 0 0.5\n".to_string(),
    );
    // Reserved punctuation longest first, symbol runs, `$`
    // continuation, multiplicities.
    push(
        "punctuation",
        ":: -> <- => [| |] $() <$> %0 %1 %aff %many %x ~ ~> | |+| ! @ = .. . *==\n".to_string(),
    );
    // Strings, chars, escapes (including the tick name-form refusal
    // boundary: `'c'` is a char, nothing else begins with a tick).
    push(
        "strings-and-chars",
        "\"a\\n\\u{41}\\\"\\\\\" 'x' '\\n' '\\u{1F600}' a'b\n".to_string(),
    );
    // Comments take priority over symbol identifiers; nested block
    // comments; a comment never opens a symbol run.
    push(
        "comments",
        "-- comment\nx {- a {- b -} -} y--dash run is a comment\nz\n".to_string(),
    );
    // Layout: blocks from `where`/`of`/`do`/`let`, equal indent `;`,
    // outdent closes, explicit braces, `in` closing `let`, `letrec`'s
    // two-token pair, and the `with (...)` supply clause's own `in`.
    push(
        "layout-blocks",
        "module M where\n  a = 1\n  b =\n    let x = 1\n        y = 2 in\n    case x of\n      0 -> y\n      _ -> 0\n  c = do\n    let z = 1\n    f z\n".to_string(),
    );
    push(
        "layout-explicit",
        "module M where\n  f =\n    let x = 1\n        y = 2 in\n    x\n  g = case 0 of { 0 -> 1; _ -> 2 }\n  h =\n    if True then\n      1\n    else\n      2\n".to_string(),
    );
    push(
        "layout-supply",
        "f = let x = with (io = e) in x in x\n".to_string(),
    );
    push(
        "layout-letrec",
        "g =\n  letrec go :: Nat -> Nat\n         go n = go n\n  in go 0\n".to_string(),
    );
    // String content with multi-byte scalars keeps one column per
    // scalar (the compiler's columns are character columns).
    push("unicode-columns", "x = \"\u{00e9}\u{4E2D}\"\ny = 1\n".to_string());
    cases
}

/// The no-drift test: `Text.Lex` renders exactly the compiler's token
/// stream for every corpus entry.
#[test]
fn lex_agrees_with_compiler_token_stream() {
    let corpus = drift_corpus();
    // One program per batch of cases: the checker recurses over a
    // program's elaborated code, so a giant program overflows a test
    // thread's stack while batches stay small.
    for (batch_idx, batch) in corpus.chunks(1).enumerate() {
        let mut program = program_header();
        program.push_str("  render-all :: List LToken -> String\n");
        program.push_str("  render-all ts = case ts of\n");
        program.push_str("    Nil -> empty-string\n");
        program.push_str("    Cons t rest -> case rest of\n");
        program.push_str("      Nil -> render-token t\n");
        program.push_str("      _ -> string-concat (render-token t)\n");
        program.push_str("            (string-concat (\"\\n\") (render-all rest))\n\n");
        program.push_str("  run-cases :: List (Bytes, List String) -> Nat -> Int\n");
        program.push_str("  run-cases cs i = case cs of\n");
        program.push_str("    Nil -> 0\n");
        program.push_str("    Cons p rest -> case p of\n");
        program.push_str("      (src, expected) ->\n");
        program.push_str("        case lex-source src of\n");
        program.push_str("          LexedFail sp k -> int-of (i + 50)\n");
        program.push_str("          LexedTokens toks ->\n");
        program.push_str("            case first-diff (render-each toks) expected 0 of\n");
        program.push_str("              None -> run-cases rest (i + 1)\n");
        program.push_str("              Some d -> int-of (d + 1)\n\n");
        program.push_str("  render-each :: List LToken -> List String\n");
        program.push_str("  render-each ts = case ts of\n");
        program.push_str("    Nil -> Nil\n");
        program.push_str("    Cons t rest -> Cons (render-token t) (render-each rest)\n\n");
        program.push_str("  first-diff :: List String -> List String -> Nat -> Option Nat\n");
        program.push_str("  first-diff a b i = case a of\n");
        program.push_str("    Nil -> case b of\n");
        program.push_str("      Nil -> None\n");
        program.push_str("      Cons y ys -> Some i\n");
        program.push_str("    Cons x xs -> case b of\n");
        program.push_str("      Nil -> Some i\n");
        program.push_str("      Cons y ys ->\n");
        program.push_str("        if string-compare x y == 0 then first-diff xs ys (i + 1) else Some i\n\n");
        program.push_str("  main :: Int\n");
        program.push_str("  main = run-cases [\n");
        let mut entries: Vec<String> = Vec::new();
        for (name, src) in batch {
            let toks: Vec<String> = expected_stream(src).lines().map(|l| string_lit(l)).collect();
            entries.push(format!(
                "    -- {name}\n    ({}, [{}])",
                bytes_lit(src),
                toks.join(", ")
            ));
        }
        program.push_str(&entries.join(",\n"));
        program.push_str("\n  ] 1\n");

        let code = run_main("lex-drift", &program);
        if code != 0 {
            let (name, src) = &batch[0];
            let expected_str = expected_stream(src);
            let expected: Vec<&str> = expected_str.lines().collect();
            if code == 51 {
                panic!(
                    "lex drift in batch {} ({name}): Text.Lex rejected the source the compiler accepts:\n--- source ---\n{src}",
                    batch_idx + 1
                );
            }
            let tok_idx = (code - 1) as usize;
            panic!(
                "lex drift in batch {} ({name}) at rendered token {tok_idx} (of {}):\n--- source ---\n{src}\n--- expected token ---\n{:?}",
                batch_idx + 1,
                expected.len(),
                expected.get(tok_idx).copied().unwrap_or("<past the end of the expected stream>")
            );
        }
    }
}

/// Layout and lexing refusals and errors are named, not silent: the
/// tab rule, the layout indent rule, `in` without a `let`, and the
/// three named unproductions.
#[test]
fn lex_errors_are_named() {
    // Each case asserts the message names the rule (the program
    // checks a message substring per case; exit = failing case).
    let mut program = program_header();
    for line in [
        "  has-sub :: Bytes -> Bytes -> Nat -> Bool",
        "  has-sub hay needle i =",
        "    if i + bytes-len needle > bytes-len hay then False",
        "    else if eq-bytes-at hay needle i 0 then True",
        "    else has-sub hay needle (i + 1)",
        "",
        "  eq-bytes-at :: Bytes -> Bytes -> Nat -> Nat -> Bool",
        "  eq-bytes-at hay needle i j =",
        "    if nat-cmp j (bytes-len needle) == 0 then True",
        "    else if nat-cmp (bytes-get hay (i + j)) (bytes-get needle j) == 0 then eq-bytes-at hay needle i (j + 1)",
        "    else False",
        "",
        "  check-msg :: Bytes -> String -> Nat -> Int",
        "  check-msg src needle i = case lex-source src of",
        "    LexedTokens _ -> int-of i",
        "    LexedFail sp k ->",
        "      if has-sub (string-to-bytes (lex-error-message k)) (string-to-bytes needle) 0",
        "      then 0 else int-of i",
        "",
    ] {
        program.push_str(line);
        program.push('\n');
    }
    let cases: [(&str, &str); 6] = [
        ("module M where\n\tfoo = 1\n", "tabs are rejected in indentation"),
        ("module M where\n  f = case x of\n  0 -> 1\n", "layout block must indent farther"),
        ("f = 1 in 2\n", "has no matching implicit"),
        ("#[x] f = 1\n", "refused:"),
        ("prim law where\n", "refused:"),
        ("'name\n", "refused:"),
    ];
    program.push_str("  main :: Int\n");
    program.push_str("  main =\n");
    // Nested ifs: the deepest case runs last; any earlier failure
    // short-circuits with its case number.
    let mut tail = format!(
        "    check-msg {} {} 6\n",
        bytes_lit(cases[5].0),
        string_lit(cases[5].1)
    );
    for (i, (src, needle)) in cases.iter().enumerate().rev().skip(1) {
        let n = i + 1;
        tail = format!(
            "    let (r{n}) = check-msg {} {} {n} in\n    if r{n} == 0 then\n      {}    else r{n}\n",
            bytes_lit(src),
            string_lit(needle),
            indent_block(&tail, 2)
        );
    }
    program.push_str(&tail);
    expect_exit("lex-errors", &program, 0);
}

/// Indent every line of a program fragment.
fn indent_block(s: &str, n: usize) -> String {
    let pad = " ".repeat(n * 2);
    s.lines()
        .map(|l| format!("{pad}{l}\n"))
        .collect()
}

// ----------------------------------------------------------------
// The bootstrap grammar: a Cinch program that parses the language's
// own grammar (the declarative surface and the expressions of the
// real compiler examples) over the P30 toolkit. Unproductions are
// named refusals in the program's header and enforced below.
// ----------------------------------------------------------------

const GRAMMAR_SRC: &str = r#"module Main where
  import Text.Lex
  import Text.Parse
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.String
  import Data.Nat
  import Data.Option
  import Data.Int
  import Text.Show

  cat-bytes :: Bytes -> Bytes -> Bytes
  cat-bytes a b =
    let out = bytes-new (bytes-len a + bytes-len b) in
    let u = copy-bytes-into out a 0 0 (bytes-len a) in
    let v = copy-bytes-into out b (bytes-len a) 0 (bytes-len b) in
    out

  chunk-bytes :: List Bytes -> Bytes
  chunk-bytes cs = case cs of
    Nil -> bytes-new 0
    Cons c rest -> cat-bytes c (chunk-bytes rest)

  int-of :: Nat -> Int
  int-of n = int-from-i64 (nat-to-i64 n)

  join-sep :: List String -> String -> String
  join-sep xs sep = case xs of
    Nil -> empty-string
    Cons y ys -> case ys of
      Nil -> y
      _ -> string-concat y (string-concat sep (join-sep ys sep))

  -- ---------------------------------------------------------------------------
  -- The abstract syntax. The bootstrap's positivity discipline allows
  -- only DIRECT recursive occurrences, so every recursive list and
  -- optional payload is a cons spine inside the one family.
  -- ---------------------------------------------------------------------------
  data Tm where
    TNil :: Tm
    TCons :: Tm -> Tm -> Tm
    TNone :: Tm
    TSome :: Tm -> Tm
    EOpName :: String -> Tm
    EVar :: String -> Tm
    ECon :: String -> Tm
    EInt :: String -> Tm
    EFloat :: String -> Tm
    EChar :: Nat -> Tm
    EStr :: Bytes -> Tm
    EUnit :: Tm
    EApp :: Tm -> Tm -> Tm
    EChain :: Tm -> Tm
    EDollar :: Tm -> Tm -> Tm
    EArrow :: Tm -> Tm -> Tm -> Tm
    ELabel :: String -> Tm -> Tm
    ETail :: String -> Tm
    EForall :: List String -> Tm -> Tm
    EUsing :: Tm -> Tm -> Tm
    ECap :: String -> Tm -> Tm
    ENeg :: Tm -> Tm
    EIf :: Tm -> Tm -> Tm -> Tm
    ECase :: Tm -> Tm -> Tm
    EAlt :: Tm -> Tm -> Tm -> Tm
    ELet :: Bool -> Tm -> Tm -> Tm
    EBind :: String -> Tm -> Tm -> Tm
    EPatBind :: Tm -> Tm -> Tm
    ELam :: Tm -> Tm -> Tm
    EAnn :: Tm -> Tm -> Tm
    ETuple :: Tm -> Tm
    EList :: Tm -> Tm
    PVar :: String -> Tm
    PWild :: Tm
    PCon :: String -> Tm -> Tm
    PInt :: String -> Tm
    PLit :: Tm -> Tm
    PTuple :: Tm -> Tm
    PList :: Tm -> Tm
    PAs :: String -> Tm -> Tm
    PAnn :: Tm -> Tm -> Tm
    PUnit :: Tm
    DModule :: String -> Tm -> Tm
    DImport :: String -> Option String -> Tm
    DExport :: List String -> Tm
    DSignature :: String -> List String -> Tm -> Tm
    DInstance :: Bool -> String -> List String -> Tm -> Tm -> Tm
    DData :: String -> List String -> Tm -> Tm
    DCtor :: String -> Tm -> Tm
    DEffect :: String -> Tm -> Tm
    DOp :: String -> Tm -> Tm
    DSig :: String -> Tm -> Tm
    DEq :: String -> Tm -> Tm -> Tm
    DFixity :: String -> Nat -> List String -> Tm

  spine :: List Tm -> Tm
  spine xs = case xs of
    Nil -> TNil
    Cons y ys -> TCons y (spine ys)

  unspine :: Tm -> List Tm
  unspine t = case t of
    TCons y ys -> Cons y (unspine ys)
    _ -> Nil

  -- ---------------------------------------------------------------------------
  -- The grammar: the declarative surface and expressions of the
  -- surface grammar's EBNF, over the laid-out token stream.
  --
  -- Named refusals (unproductions this grammar deliberately does not
  -- produce; each parse failure names itself):
  --   foreign, rewrite, upgrade and generate declarations;
  --   record/codata declarations and record construction/patterns;
  --   the refinement `type`/`opaque` declarations;
  --   effect region parameters and module parameters (the dependent
  --     module forms are a named design refusal, amendment section 13);
  --   typed and implicit binders in binder position, and `erased`/`own`;
  --   `macro` modifiers and macro calls;
  --   quotations and splices;
  --   `do` notation, `handle`/`handler` and `with` supply expressions;
  --   import modifiers and file-type import converters;
  --   postfix projection, type arguments and capability supplies;
  --   range literals;
  --   operator fixity resolution (chains parse flat: `EChain`).
  -- ---------------------------------------------------------------------------

  refuse-p :: forall a. String -> Parser a
  refuse-p what = fail-p (string-concat ("refused: ") what)

  kwd :: String -> Parser String
  kwd w = expect-word w

  punct :: String -> Parser String
  punct w = expect-punct w

  op-tok :: String -> Parser String
  op-tok w = expect-op w

  and-then :: forall a b. Parser a -> Parser b -> Parser b
  and-then p q = parse-bind p (\_ -> q)

  ignore1 :: forall a. Parser a -> Parser Unit
  ignore1 p = parse-fmap (\x -> ()) p

  -- An operator token in infix position: a symbol run, or the
  -- contextual bare `|` (reserved punctuation that doubles as the
  -- operator `|` in infix application, amendment section 6).
  infix-op :: Parser String
  infix-op = MkParser (\s -> run-parser-fn (

or-else
    (token-p (\k -> case k of
        TOp s -> Some s
        _ -> None) (Cons (ILabel ("operator")) Nil))
    (parse-bind (punct ("|")) (\s -> parse-pure ("|")))) s)
  name-op :: Parser String
  name-op = MkParser (\s -> run-parser-fn (

or-else
    infix-op
    (parse-bind (punct ("=")) (\s -> parse-pure ("=")))) s)
  con-path :: Parser String
  con-path = MkParser (\s -> run-parser-fn (

parse-bind satisfy-con (\c ->
    parse-fmap (\ds -> join-sep (Cons c ds) (".")) (many-p (and-then (punct (".")) satisfy-con)))) s)
  atomic-name :: Parser String
  atomic-name = MkParser (\s -> run-parser-fn (

or-else
    (parse-bind satisfy-var (\v -> parse-pure v))
    (parse-bind con-path (\p ->
      or-else
        (and-then (punct (".")) (parse-bind satisfy-var (\v ->
          parse-pure (string-concat p (string-concat (".") v)))))
        (parse-pure p)))) s)
  effect-label :: Parser Tm
  effect-label = MkParser (\s -> run-parser-fn (

parse-bind con-path (\n ->
    parse-fmap (\args -> ELabel n (spine args)) (many-p atomic-term))) s)
  effect-row :: Parser Tm
  effect-row = MkParser (\s -> run-parser-fn (

between-p (punct ("{")) (punct ("}"))
    (parse-bind (sep-by-p effect-label (punct (","))) (\ls ->
      or-else
        (and-then (punct ("|")) (parse-bind satisfy-var (\v -> parse-pure (spine (Cons (ETail v) ls)))))
        (parse-pure (spine ls))))) s)
  term :: Parser Tm
  term = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (label-p ("forall") forall-term)
      (Cons (label-p ("using") using-term)
        (Cons (label-p ("lambda") lambda-term)
          (Cons (label-p ("let") let-term)
            (Cons (label-p ("if") if-term)
              (Cons (label-p ("case") case-term)
                (Cons (label-p ("arrow") arrow-term) Nil)))))))) s)
  forall-term :: Parser Tm
  forall-term = MkParser (\s -> run-parser-fn (

and-then (kwd ("forall"))
    (parse-bind binder-names (\ns ->
      and-then (punct (".")) (parse-fmap (\b -> EForall ns b) term)))) s)
  binder-names :: Parser (List String)
  binder-names = MkParser (\s -> run-parser-fn (

many-p (or-else satisfy-var typed-binder-refusal)) s)
  typed-binder-refusal :: Parser String
  typed-binder-refusal = MkParser (\s -> run-parser-fn (

and-then (punct ("(")) (refuse-p ("typed binders"))) s)
  using-term :: Parser Tm
  using-term = MkParser (\s -> run-parser-fn (

and-then (kwd ("using"))
    (parse-bind (between-p (punct ("(")) (punct (")")) (sep-by-p capability-binder (punct (",")))) (\caps ->
      and-then (punct ("=>")) (parse-fmap (\b -> EUsing (spine caps) b) term)))) s)
  capability-binder :: Parser Tm
  capability-binder = MkParser (\s -> run-parser-fn (

parse-bind satisfy-var (\n ->
    and-then (punct ("::")) (parse-fmap (\t -> ECap n t) term))) s)
  lambda-term :: Parser Tm
  lambda-term = MkParser (\s -> run-parser-fn (

and-then (punct ("\\\\"))
    (parse-bind (some-p atomic-pattern) (\ps ->
      and-then (punct ("->")) (parse-fmap (\b -> ELam (spine ps) b) term)))) s)
  let-term :: Parser Tm
  let-term = MkParser (\s -> run-parser-fn (

parse-bind
    (or-else
      (and-then (kwd ("letrec")) (and-then (kwd ("let")) (parse-pure True)))
      (and-then (kwd ("let")) (parse-pure False)))
    (\rec ->
      parse-bind (between-p (punct ("{")) (punct ("}")) (parse-bind (sep-by-p binding (punct (";"))) (\bs -> parse-fmap (\_ -> bs) (optional-p (punct (";")))))) (\bs ->
        and-then (kwd ("in")) (parse-fmap (\b -> ELet rec (spine bs) b) term)))) s)
  binding :: Parser Tm
  binding = MkParser (\s -> run-parser-fn (

parse-bind value-name (\n ->
    parse-bind (many-p atomic-pattern) (\ps ->
      or-else
        (and-then (punct ("::")) (parse-fmap (\t -> DSig n t) term))
        (and-then (punct ("=")) (parse-fmap (\b -> EBind n (spine ps) b) term))))) s)
  if-term :: Parser Tm
  if-term = MkParser (\s -> run-parser-fn (

and-then (kwd ("if"))
    (parse-bind term (\c ->
      and-then (kwd ("then"))
        (parse-bind term (\t ->
          and-then (kwd ("else")) (parse-fmap (\e -> EIf c t e) term)))))) s)
  case-term :: Parser Tm
  case-term = MkParser (\s -> run-parser-fn (
and-then (kwd ("case"))
    (parse-bind term (\s ->
      and-then (kwd ("of"))
        (parse-fmap (\alts -> ECase s (spine alts)) (between-p (punct ("{")) (punct ("}")) (parse-bind (sep-by-p alternative (punct (";"))) (\alts -> parse-fmap (\_ -> alts) (optional-p (punct (";")))))))))
) s)
  alternative :: Parser Tm
  alternative = MkParser (\s -> run-parser-fn (

parse-bind pattern (\p ->
    parse-bind (optional-p (and-then (punct ("|")) term)) (\g ->
      and-then (punct ("->")) (parse-fmap (\b -> EAlt p (opt-spine g) b) term)))) s)
  opt-spine :: Option Tm -> Tm
  opt-spine o = case o of
    None -> TNone
    Some t -> TSome t

  arrow-term :: Parser Tm
  arrow-term = MkParser (\s -> run-parser-fn (

parse-bind annotation (\a ->
    or-else
      (and-then (punct ("->"))
        (parse-bind (optional-p effect-row) (\row ->
          parse-fmap (\b -> EArrow a (row-spine row) b) term)))
      (parse-pure a))) s)
  row-spine :: Option Tm -> Tm
  row-spine o = case o of
    None -> TNil
    Some t -> t

  annotation :: Parser Tm
  annotation = MkParser (\s -> run-parser-fn (

parse-bind dollar-expr (\a ->
    or-else
      (and-then (punct ("::")) (parse-fmap (\t -> EAnn a t) term))
      (parse-pure a))) s)
  dollar-expr :: Parser Tm
  dollar-expr = MkParser (\s -> run-parser-fn (

parse-bind infix-expr (\a ->
    or-else
      (and-then (punct ("$")) (parse-fmap (\b -> EDollar a b) term))
      (parse-pure a))) s)
  infix-expr :: Parser Tm
  infix-expr = MkParser (\s -> run-parser-fn (

parse-bind application (\a ->
    parse-fmap (\items -> EChain (spine (Cons a items))) chain-atoms)) s)
  chain-atoms :: Parser (List Tm)
  chain-atoms = MkParser (\s -> run-parser-fn (

or-else
    (parse-bind infix-op (\op ->
      parse-bind application (\a ->
        parse-fmap (\rest -> Cons (EOpName op) (Cons a rest)) chain-atoms)))
    (parse-pure Nil)) s)
  application :: Parser Tm
  application = MkParser (\s -> run-parser-fn (

parse-bind prefix-expr (\f ->
    parse-bind (many-p postfix-chain) (\args -> case args of
      Nil -> parse-pure f
      _ -> parse-pure (EApp f (spine args))))) s)
  prefix-expr :: Parser Tm
  prefix-expr = MkParser (\s -> run-parser-fn (

or-else
    (and-then (op-tok ("-")) (parse-fmap (\t -> ENeg t) prefix-expr))
    postfix-chain) s)
  postfix-chain :: Parser Tm
  postfix-chain = MkParser (\s -> run-parser-fn (

parse-bind atomic-term (\t ->
    parse-fmap (\_ -> t) (many-p postfix-refusal))) s)
  postfix-refusal :: Parser Unit
  postfix-refusal = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (attempt-p (and-then (punct (".")) (and-then satisfy-var (refuse-p ("projection")))))
      (Cons (attempt-p (and-then (punct ("@")) (refuse-p ("type arguments"))))
        Nil))) s)
  atomic-term :: Parser Tm
  atomic-term = MkParser (\s -> run-parser-fn (

choice-p
    (Cons unit-or-paren
      (Cons list-or-range
        (Cons (parse-fmap (\s -> EInt s) satisfy-int)
          (Cons (parse-fmap (\s -> EFloat s) satisfy-float)
            (Cons (parse-fmap (\b -> EStr b) satisfy-str)
              (Cons (parse-fmap (\n -> EChar n) satisfy-char)
                (Cons quote-refusal
                  (Cons splice-refusal
                    (Cons macro-refusal
                      (Cons handler-refusal
                        (Cons do-refusal
                          (Cons (attempt-p record-refusal)
                            (Cons (parse-fmap (\n -> EVar n) atomic-name) Nil)))))))))))))) s)
  unit-or-paren :: Parser Tm
  unit-or-paren = MkParser (\s -> run-parser-fn (

and-then (punct ("(")) unit-or-paren-rest) s)
  unit-or-paren-rest :: Parser Tm
  unit-or-paren-rest = MkParser (\s -> run-parser-fn (

or-else
    (and-then (punct (")")) (parse-pure EUnit))
    (or-else
      (parse-bind name-op (\op -> and-then (punct (")")) (parse-pure (EOpName op))))
      tuple-or-single)) s)
  tuple-or-single :: Parser Tm
  tuple-or-single = MkParser (\s -> run-parser-fn (

parse-bind term (\t ->
    or-else
      (and-then (punct (")")) (parse-pure t))
      (parse-bind comma-terms (\ts ->
        and-then (punct (")")) (parse-pure (ETuple (spine (Cons t ts)))))))) s)
  comma-terms :: Parser (List Tm)
  comma-terms = MkParser (\s -> run-parser-fn (

some-p (and-then (punct (",")) term)) s)
  list-or-range :: Parser Tm
  list-or-range = MkParser (\s -> run-parser-fn (

and-then (punct ("["))
    (or-else
      (and-then (punct ("]")) (parse-pure (EList TNil)))
      (parse-bind (sep-by-p term (punct (","))) (\ts ->
        or-else
          (and-then (punct ("..")) (refuse-p ("range literals")))
          (and-then (punct ("]")) (parse-pure (EList (spine ts)))))))) s)
  record-refusal :: Parser Tm
  record-refusal = MkParser (\s -> run-parser-fn (

parse-bind satisfy-con (\c ->
    and-then (punct ("{")) (refuse-p ("record construction")))) s)
  quote-refusal :: Parser Tm
  quote-refusal = MkParser (\s -> run-parser-fn (

and-then (punct ("[|")) (refuse-p ("quotations"))) s)
  splice-refusal :: Parser Tm
  splice-refusal = MkParser (\s -> run-parser-fn (

and-then (punct ("$")) (and-then (punct ("(")) (refuse-p ("splices")))) s)
  macro-refusal :: Parser Tm
  macro-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("macro")) (refuse-p ("macro declarations"))) s)
  handler-refusal :: Parser Tm
  handler-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("handle")) (refuse-p ("handle expressions"))) s)
  do-refusal :: Parser Tm
  do-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("do")) (refuse-p ("do notation"))) s)
  pattern :: Parser Tm
  pattern = MkParser (\s -> run-parser-fn (

parse-bind constructor-pattern (\p ->
    or-else
      (and-then (punct ("::")) (parse-fmap (\t -> PAnn p t) term))
      (parse-pure p))) s)
  constructor-pattern :: Parser Tm
  constructor-pattern = MkParser (\s -> run-parser-fn (

or-else
    (parse-bind con-path (\c ->
      or-else
        (parse-bind (some-p atomic-pattern) (\ps -> parse-pure (PCon c (spine ps))))
        (parse-pure (PCon c TNil))))
    atomic-pattern) s)
  atomic-pattern :: Parser Tm
  atomic-pattern = MkParser (\s -> run-parser-fn (

or-else paren-or-unit-pattern
    (or-else list-pattern
      (or-else (parse-fmap (\s -> PInt s) satisfy-int)
        (or-else (parse-fmap (\t -> PLit t) literal-atom)
          (or-else var-or-wild
            (or-else (parse-bind satisfy-con (\c -> parse-pure (PCon c TNil)))
              (and-then (punct ("~")) (refuse-p ("memo patterns"))))))))) s)
  paren-or-unit-pattern :: Parser Tm
  paren-or-unit-pattern = MkParser (\s -> run-parser-fn (

and-then (punct ("(")) paren-pattern-rest) s)
  paren-pattern-rest :: Parser Tm
  paren-pattern-rest = MkParser (\s -> run-parser-fn (

or-else
    (and-then (punct (")")) (parse-pure PUnit))
    (parse-bind pattern (\p ->
      or-else
        (and-then (punct (")")) (parse-pure p))
        (parse-bind comma-patterns (\ps ->
          and-then (punct (")")) (parse-pure (PTuple (spine (Cons p ps))))))))) s)
  comma-patterns :: Parser (List Tm)
  comma-patterns = MkParser (\s -> run-parser-fn (

some-p (and-then (punct (",")) pattern)) s)
  var-or-wild :: Parser Tm
  var-or-wild = MkParser (\s -> run-parser-fn (

parse-bind satisfy-var (\v ->
    if string-compare v ("_") == 0 then parse-pure PWild
    else parse-pure (PVar v))) s)
  list-pattern :: Parser Tm
  list-pattern = MkParser (\s -> run-parser-fn (

and-then (punct ("["))
    (or-else
      (and-then (punct ("]")) (parse-pure (PList TNil)))
      (parse-bind (sep-by-p pattern (punct (","))) (\ps ->
        and-then (punct ("]")) (parse-pure (PList (spine ps))))))) s)
  literal-atom :: Parser Tm
  literal-atom = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (parse-fmap (\s -> EFloat s) satisfy-float)
      (Cons (parse-fmap (\b -> EStr b) satisfy-str)
        (Cons (parse-fmap (\n -> EChar n) satisfy-char) Nil)))) s)
  value-name :: Parser String
  value-name = MkParser (\s -> run-parser-fn (

or-else
    satisfy-var
    (parse-bind (punct ("(")) (\op1 -> parse-bind name-op (\op ->
      and-then (punct (")")) (parse-pure op))))) s)
  decl :: Parser Tm
  decl = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (label-p ("import") import-decl)
      (Cons (label-p ("export") export-decl)
        (Cons (label-p ("signature") signature-decl)
          (Cons (label-p ("instance") instance-decl)
            (Cons (label-p ("data") data-decl)
              (Cons (label-p ("effect") effect-decl)
                (Cons (label-p ("fixity") fixity-decl)
                  (Cons foreign-refusal
                    (Cons rewrite-refusal
                      (Cons upgrade-refusal
                        (Cons generate-refusal
                          (Cons record-decl-refusal
                            (Cons type-decl-refusal
                              (Cons (label-p ("value") value-decl) Nil))))))))))))))) s)
  foreign-refusal :: Parser Tm
  foreign-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("foreign")) (refuse-p ("foreign declarations"))) s)
  rewrite-refusal :: Parser Tm
  rewrite-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("rewrite")) (refuse-p ("rewrite rules"))) s)
  upgrade-refusal :: Parser Tm
  upgrade-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("upgrade")) (refuse-p ("upgrade declarations"))) s)
  generate-refusal :: Parser Tm
  generate-refusal = MkParser (\s -> run-parser-fn (

and-then (kwd ("generate")) (refuse-p ("generate declarations"))) s)
  record-decl-refusal :: Parser Tm
  record-decl-refusal = MkParser (\s -> run-parser-fn (

or-else
    (and-then (kwd ("record")) (refuse-p ("record declarations")))
    (and-then (kwd ("codata")) (refuse-p ("record declarations")))) s)
  type-decl-refusal :: Parser Tm
  type-decl-refusal = MkParser (\s -> run-parser-fn (

or-else
    (and-then (kwd ("type")) (refuse-p ("refinement type declarations")))
    (and-then (kwd ("opaque")) (refuse-p ("refinement type declarations")))) s)
  import-decl :: Parser Tm
  import-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("import"))
    (parse-bind (optional-p import-modifier) (\m ->
      parse-bind con-path (\p ->
        or-else
          (and-then (kwd ("as")) (parse-bind satisfy-con (\a -> parse-pure (DImport p (Some a)))))
          (parse-pure (DImport p None)))))) s)
  import-modifier :: Parser String
  import-modifier = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (and-then (kwd ("stage")) (refuse-p ("import modifiers")))
      (Cons (and-then (kwd ("replaceable")) (refuse-p ("import modifiers")))
        (Cons (and-then (kwd ("specialized")) (refuse-p ("import modifiers")))
          Nil)))) s)
  export-decl :: Parser Tm
  export-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("export"))
    (parse-fmap (\items -> DExport items) (sep-by-p export-item (punct (","))))) s)
  export-item :: Parser String
  export-item = MkParser (\s -> run-parser-fn (

or-else
    (and-then (kwd ("module")) (refuse-p ("module re-exports")))
    (or-else value-name satisfy-con)) s)
  signature-decl :: Parser Tm
  signature-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("signature"))
    (parse-bind satisfy-con (\n ->
      parse-bind binder-names (\bs ->
        and-then (kwd ("where"))
          (parse-fmap (\ms -> DSignature n bs (spine ms)) decl-block))))) s)
  instance-decl :: Parser Tm
  instance-decl = MkParser (\s -> run-parser-fn (

parse-bind (optional-p (kwd ("implicit"))) (\im ->
    and-then (kwd ("instance"))
      (parse-bind satisfy-con (\n ->
        parse-bind binder-names (\bs ->
          and-then (punct ("::"))
            (parse-bind term (\head ->
              and-then (kwd ("where"))
                (parse-fmap (\ms -> DInstance (is-just im) n bs head (spine ms)) decl-block)))))))) s)
  is-just :: Option String -> Bool
  is-just o = case o of
    None -> False
    Some s -> True

  data-decl :: Parser Tm
  data-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("data"))
    (parse-bind satisfy-con (\n ->
      parse-bind binder-names (\bs ->
        parse-bind data-ann (\ann ->
          and-then (kwd ("where"))
            (parse-fmap (\cs -> DData n bs (spine cs)) ctor-block)))))) s)
  ctor-block :: Parser (List Tm)
  ctor-block = MkParser (\s -> run-parser-fn (
between-p (punct ("{")) (punct ("}"))
    (parse-bind (sep-by-p ctor-item (punct (";"))) (\cs ->
      parse-fmap (\u -> cs) (optional-p (punct (";")))))) s)

  ctor-item :: Parser Tm
  ctor-item = MkParser (\s -> run-parser-fn (
parse-bind satisfy-con (\n ->
    and-then (punct ("::")) (parse-fmap (\t -> DCtor n t) term))) s)

  op-block :: Parser (List Tm)
  op-block = MkParser (\s -> run-parser-fn (
between-p (punct ("{")) (punct ("}"))
    (parse-bind (sep-by-p op-item (punct (";"))) (\os ->
      parse-fmap (\u -> os) (optional-p (punct (";")))))) s)

  op-item :: Parser Tm
  op-item = MkParser (\s -> run-parser-fn (
parse-bind satisfy-var (\n ->
    and-then (punct ("::")) (parse-fmap (\t -> DOp n t) term))) s)

  data-ann :: Parser (Option Tm)
  data-ann = MkParser (\s -> run-parser-fn (

optional-p (and-then (punct ("::")) term)) s)
  effect-decl :: Parser Tm
  effect-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("effect"))
    (parse-bind satisfy-con (\n ->
      parse-bind (optional-p (and-then (punct ("(")) (refuse-p ("effect region parameters")))) (\_ ->
        and-then (kwd ("where"))
          (parse-fmap (\ops -> DEffect n (spine ops)) op-block))))) s)
  fixity-decl :: Parser Tm
  fixity-decl = MkParser (\s -> run-parser-fn (

parse-bind assoc-word (\a ->
    parse-bind satisfy-int (\lv ->
      parse-fmap (\ops -> DFixity a (text-nat lv) ops) (sep-by-p name-op (punct (",")))))) s)
  assoc-word :: Parser String
  assoc-word = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (kwd ("infixl")) (Cons (kwd ("infixr")) (Cons (kwd ("infix")) Nil)))) s)
  text-nat :: String -> Nat
  text-nat s = text-nat-from (string-to-bytes s) 0 0

  text-nat-from :: Bytes -> Nat -> Nat -> Nat
  text-nat-from b i acc =
    if nat-cmp i (bytes-len b) == 0 then acc
    else text-nat-from b (i + 1) (acc * 10 + (bytes-get b i - 48))

  value-decl :: Parser Tm
  value-decl = MkParser (\s -> run-parser-fn (

parse-bind (many-p modifier-word) (\mods ->
    parse-bind value-name (\n ->
      parse-bind (many-p atomic-pattern) (\ps ->
        or-else
          (and-then (punct ("::")) (parse-fmap (\t -> DSig n t) term))
          (parse-bind (punct ("=")) (\_ ->
            parse-bind term (\b ->
              or-else
                (and-then (kwd ("where")) (refuse-p ("local `where` blocks")))
                (parse-pure (DEq n (spine ps) b))))))))) s)
  modifier-word :: Parser String
  modifier-word = MkParser (\s -> run-parser-fn (

choice-p
    (Cons (kwd ("proof"))
      (Cons (kwd ("spec"))
        (Cons (kwd ("stage"))
          (Cons (kwd ("corec"))
            (Cons (and-then (kwd ("macro")) (refuse-p ("macro modifiers")))
              Nil)))))) s)
  decl-block :: Parser (List Tm)
  decl-block = MkParser (\s -> run-parser-fn (

between-p (punct ("{")) (punct ("}"))
    (parse-bind (sep-by-p decl (punct (";"))) (\ds ->
      parse-fmap (\_ -> ds) (optional-p (punct (";")))))) s)
  module-decl :: Parser Tm
  module-decl = MkParser (\s -> run-parser-fn (

and-then (kwd ("module"))
    (parse-bind con-path (\n ->
      parse-bind (optional-p (and-then (punct ("(")) (refuse-p ("module parameters")))) (\_ ->
        parse-bind (optional-p (and-then (punct ("::")) term)) (\_ ->
          and-then (kwd ("where"))
            (parse-fmap (\ds -> DModule n (spine ds)) decl-block)))))) s)
  program :: Parser Tm
  program = MkParser (\s -> run-parser-fn (

parse-fmap (\ms -> spine ms) (many-p module-decl)) s)
  module-name :: Tm -> String
  module-name t = case t of
    DModule n ds -> n
    _ -> "?"

  decl-lines :: Tm -> List String
  decl-lines t = case t of
    DModule n ds -> decls-lines (unspine ds)
    _ -> Nil

  decls-lines :: (List Tm) -> List String
  decls-lines ts = case ts of
    Nil -> Nil
    Cons d rest -> Cons (decl-line d) (decls-lines rest)

  decl-line :: Tm -> String
  decl-line d = case d of
    DImport m a -> string-concat ("import:") m
    DExport ns -> string-concat ("export:") (join-sep ns (","))
    DSignature n bs ms -> string-concat ("signature:") n
    DInstance imp n bs h ms -> string-concat ("instance:") n
    DData n bs cs -> string-concat ("data:") n
    DEffect n ops -> string-concat ("effect:") n
    DSig n t -> string-concat ("sig:") n
    DEq n ps b -> string-concat ("eq:") n
    DFixity a lv ops -> string-concat ("fixity:") (join-sep ops (","))
    _ -> "?"

  int-lits :: Tm -> List String
  int-lits t = case t of
    TNil -> Nil
    TCons a b -> append-strs (int-lits a) (int-lits b)
    TNone -> Nil
    TSome a -> int-lits a
    EOpName s -> Nil
    EVar s -> Nil
    ECon s -> Nil
    EInt s -> Cons s Nil
    EFloat s -> Nil
    EChar n -> Nil
    EStr b -> Nil
    EUnit -> Nil
    EApp f args -> append-strs (int-lits f) (int-lits args)
    EChain items -> int-lits items
    EDollar a b -> append-strs (int-lits a) (int-lits b)
    EArrow a row b -> append-strs (int-lits a) (append-strs (int-lits row) (int-lits b))
    ELabel n args -> int-lits args
    ETail n -> Nil
    EForall ns b -> int-lits b
    EUsing caps b -> append-strs (int-lits caps) (int-lits b)
    ECap n t -> int-lits t
    ENeg t -> int-lits t
    EIf c t e -> append-strs (int-lits c) (append-strs (int-lits t) (int-lits e))
    ECase s alts -> append-strs (int-lits s) (int-lits alts)
    EAlt p g b -> append-strs (int-lits p) (append-strs (int-lits g) (int-lits b))
    ELet rec bs b -> append-strs (int-lits bs) (int-lits b)
    EBind n ps b -> append-strs (int-lits ps) (int-lits b)
    EPatBind p b -> append-strs (int-lits p) (int-lits b)
    ELam ps b -> append-strs (int-lits ps) (int-lits b)
    EAnn a t -> append-strs (int-lits a) (int-lits t)
    ETuple items -> int-lits items
    EList items -> int-lits items
    PVar s -> Nil
    PWild -> Nil
    PCon n args -> int-lits args
    PInt s -> Cons s Nil
    PLit t -> int-lits t
    PTuple items -> int-lits items
    PList items -> int-lits items
    PAs n p -> int-lits p
    PAnn p t -> append-strs (int-lits p) (int-lits t)
    PUnit -> Nil
    DModule n ds -> int-lits ds
    DImport m a -> Nil
    DExport ns -> Nil
    DSignature n bs ms -> int-lits ms
    DInstance imp n bs h ms -> append-strs (int-lits h) (int-lits ms)
    DData n bs cs -> int-lits cs
    DCtor n t -> int-lits t
    DEffect n ops -> int-lits ops
    DOp n t -> int-lits t
    DSig n t -> int-lits t
    DEq n ps b -> append-strs (int-lits ps) (int-lits b)
    DFixity a lv ops -> Nil

  append-strs :: List String -> List String -> List String
  append-strs xs ys = case xs of
    Nil -> ys
    Cons z zs -> Cons z (append-strs zs ys)

  -- The class signature's member names and the data declaration's
  -- constructor names, for the structural facts.
  member-names :: Tm -> List String
  member-names t = case t of
    DSignature n bs ms -> members-of (unspine ms)
    _ -> Nil

  members-of :: (List Tm) -> List String
  members-of ts = case ts of
    Nil -> Nil
    Cons d rest -> case d of
      DSig n t -> Cons n (members-of rest)
      DEq n ps b -> Cons n (members-of rest)
      _ -> members-of rest

  ctor-names :: Tm -> List String
  ctor-names t = case t of
    DData n bs cs -> ctors-of (unspine cs)
    _ -> Nil

  ctors-of :: (List Tm) -> List String
  ctors-of ts = case ts of
    Nil -> Nil
    Cons d rest -> case d of
      DCtor n t -> Cons n (ctors-of rest)
      _ -> ctors-of rest

  fixity-fact :: Tm -> String
  fixity-fact t = case t of
    DFixity a lv ops ->
      string-concat a (string-concat (":") (string-concat (nat-to-string lv) (string-concat (":") (join-sep ops (",")))))
    _ -> "?"

  find-decl :: Tm -> String -> Tm
  find-decl t kind = find-in (decl-lines-top t) (decls-top t) kind

  decls-top :: Tm -> List Tm
  decls-top t = case t of
    DModule n ds -> unspine ds
    _ -> Nil

  decl-lines-top :: Tm -> List String
  decl-lines-top t = decl-lines t

  find-in :: List String -> List Tm -> String -> Tm
  find-in kinds ds kind = case ds of
    Nil -> TNil
    Cons d rest -> case kinds of
      Nil -> TNil
      Cons k ks ->
        if string-compare k (string-concat kind (":")) == 0 then d
        else if has-prefix k kind then d
        else find-in ks rest kind

  has-prefix :: String -> String -> Bool
  has-prefix s p = eq-bytes-at (string-to-bytes s) (string-to-bytes p) 0 0

  eq-bytes-at :: Bytes -> Bytes -> Nat -> Nat -> Bool
  eq-bytes-at hay needle i j =
    if nat-cmp j (bytes-len needle) == 0 then True
    else if i + j >= bytes-len hay then False
    else if nat-cmp (bytes-get hay (i + j)) (bytes-get needle j) == 0 then eq-bytes-at hay needle i (j + 1)
    else False

  has-sub :: Bytes -> Bytes -> Nat -> Bool
  has-sub hay needle i =
    if i + bytes-len needle > bytes-len hay then False
    else if eq-bytes-at hay needle i 0 then True
    else has-sub hay needle (i + 1)

  -- ---------------------------------------------------------------------------
  -- The bootstrap assertions.
  -- ---------------------------------------------------------------------------
  eq-s :: String -> String -> Bool
  eq-s a b = string-compare a b == 0

  check-fib :: Tm -> Int
  check-fib mod =
    let (m) = module-name mod in
    if not (eq-s m ("Main")) then 10 else
    let (decls) = join-sep (decl-lines mod) ("\n") in
    if not (eq-s decls (case string-from-bytes __FIB_DECLS__ of Some s -> s; None -> empty-string)) then 11 else
    let (ints) = join-sep (int-lits mod) (",") in
    if not (eq-s ints (case string-from-bytes __FIB_INTS__ of Some s -> s; None -> empty-string)) then 12 else
    0

  check-classes :: Tm -> Int
  check-classes mod =
    let (m) = module-name mod in
    if not (eq-s m ("Main")) then 20 else
    let (decls) = join-sep (decl-lines mod) ("\n") in
    if not (eq-s decls (case string-from-bytes __CLASSES_DECLS__ of Some s -> s; None -> empty-string)) then 21 else
    let (ints) = join-sep (int-lits mod) (",") in
    if not (eq-s ints (case string-from-bytes __CLASSES_INTS__ of Some s -> s; None -> empty-string)) then 22 else
    let (data-dec) = find-decl mod ("data") in
    if not (eq-s (join-sep (ctor-names data-dec) (",")) (case string-from-bytes __CLASSES_CTORS__ of Some s -> s; None -> empty-string)) then 23 else
    let (fix-dec) = find-decl mod ("fixity") in
    if not (eq-s (fixity-fact fix-dec) (case string-from-bytes __CLASSES_FIX__ of Some s -> s; None -> empty-string)) then 24 else
    let (sig-dec) = find-decl mod ("signature") in
    if not (eq-s (join-sep (member-names sig-dec) (",")) (case string-from-bytes __CLASSES_MEMBERS__ of Some s -> s; None -> empty-string)) then 25 else
    0

  first-module :: Tm -> Tm
  first-module t = case unspine t of
    Nil -> TNil
    Cons m rest -> m

  run-fib :: Int
  run-fib = case run-parser ("fib") __FIB_SRC__ program of
    Parsed mod soft -> check-fib (first-module mod)
    Failed b -> case bundle-errors b of
      Cons e rest -> 100 + line-of e
      Nil -> 99

  run-classes :: Int
  run-classes = case run-parser ("classes") __CLASSES_SRC__ program of
    Parsed mod soft -> check-classes (first-module mod)
    Failed b -> case bundle-errors b of
      Cons e rest -> err-kind e
      Nil -> 99

  err-kind :: ParseError -> Int
  err-kind e = case e of
    EFail sp m -> 150
    EUnexpected sp u xs -> 180 + int-of (span-col sp)

  line-of :: ParseError -> Int
  line-of e = 180 + int-of (span-col (error-span e))

  main :: Int
  main =
    let (r1) = run-fib in
    if r1 == 0 then run-classes else r1
"#;

/// The bootstrap test: the grammar program parses
/// `examples/compiler/fib.cinch` and `examples/compiler/classes.cinch`
/// end to end and asserts the extracted facts (module name,
/// declaration list, integer literals in source order, and the
/// structural facts of the class/data/fixity declarations).
#[test]
fn bootstrap_grammar_parses_the_examples() {
    let fib = include_str!("../../examples/compiler/fib.cinch");
    let classes = include_str!("../../examples/compiler/classes.cinch");
    let src = GRAMMAR_SRC
        .replace("__FIB_SRC__", &bytes_lit(fib))
        .replace("__CLASSES_SRC__", &bytes_lit(classes))
        .replace("__FIB_DECLS__", &bytes_lit("sig:fib\neq:fib\nsig:main\neq:main"))
        .replace("__FIB_INTS__", &bytes_lit("0,0,1,1,1,2,10,55,0,1"))
        .replace(
            "__CLASSES_DECLS__",
            &bytes_lit(
                "signature:Box\nfixity:|+|\nsig:|+|\neq:|+|\ndata:Wrap\ninstance:Box_Wrap\nsig:twice\neq:twice\nsig:main\neq:main",
            ),
        )
        .replace("__CLASSES_INTS__", &bytes_lit("3,1,1,8,0,1"))
        .replace("__CLASSES_CTORS__", &bytes_lit("W"))
        .replace("__CLASSES_FIX__", &bytes_lit("infixl:5:|+|"))
        .replace("__CLASSES_MEMBERS__", &bytes_lit("unbox"));
    let code = run_main("grammar", &src);
    assert_eq!(code, 0, "bootstrap grammar: exit {code} (10.. facts of fib, 20.. facts of classes, 100 + line = parse failure line)");
}

// ----------------------------------------------------------------
// Bundle rendering: megaparsec-style error output with caret,
// unexpected, and expecting lines.
// ----------------------------------------------------------------

#[test]
fn parse_bundle_rendering() {
    let mut program = program_header();
    for line in [
        "  check-sub :: String -> String -> Bool",
        "  check-sub hay needle = has-sub (string-to-bytes hay) (string-to-bytes needle) 0",
        "",
        "  has-sub :: Bytes -> Bytes -> Nat -> Bool",
        "  has-sub hay needle i =",
        "    if i + bytes-len needle > bytes-len hay then False",
        "    else if eq-bytes-at hay needle i 0 then True",
        "    else has-sub hay needle (i + 1)",
        "",
        "  eq-bytes-at :: Bytes -> Bytes -> Nat -> Nat -> Bool",
        "  eq-bytes-at hay needle i j =",
        "    if nat-cmp j (bytes-len needle) == 0 then True",
        "    else if i + j >= bytes-len hay then False",
        "    else if nat-cmp (bytes-get hay (i + j)) (bytes-get needle j) == 0 then eq-bytes-at hay needle i (j + 1)",
        "    else False",
        "",
        "  main :: Int",
        "  main = case run-parser (\x22test\x22) (bytes-from-list [119, 111, 114, 108, 100]) (expect-word (\x22hello\x22)) of",
        "    Parsed x soft -> 1",
        "    Failed b ->",
        "      let rendered = render-bundle b in",
        "      if check-sub rendered (\x22unexpected:\x22) then",
        "        if check-sub rendered (\x22expecting:\x22) then",
        "          if check-sub rendered (\x22^\x22) then 0 else 3",
        "        else 2",
        "      else 4",
    ] {
        program.push_str(line);
        program.push('\n');
    }
    expect_exit("bundle-rendering", &program, 0);
}

#[test]
fn parse_recovery_handlers() {
    let mut program = program_header();
    for line in [
        "  main :: Int",
        "  main =",
        "    case run-parser-with (\"test\") (bytes-from-list [119, 111, 114, 108, 100]) (\\e -> e) (expect-word (\"hello\")) of",
        "      Parsed x soft -> 1",
        "      Failed b -> 0",
    ] {
        program.push_str(line);
        program.push('\n');
    }
    expect_exit("recovery-handlers", &program, 0);
}

#[test]
fn parse_refusals_named() {
    let mut program = program_header();
    for line in [
        "  main :: Int",
        "  main = case run-parser (\"test\") (bytes-from-list [102, 111, 114, 101, 105, 103, 110]) (expect-word (\"hello\")) of",
        "    Parsed x soft -> 1",
        "    Failed b -> 0",
    ] {
        program.push_str(line);
        program.push('\n');
    }
    expect_exit("refusals-named", &program, 0);
}

// ----------------------------------------------------------------
// expect-eof: the non-consuming idempotent guard (D307).
// ----------------------------------------------------------------

/// `expect-eof` is a GUARD, never a consumer (the API finding recorded
/// at D307: the old token-consuming form ran twice under
/// `run-parser-pure`, whose implicit whole-input `expect-eof` is
/// appended after the user parser, and the second run failed). Both
/// behaviors are pinned: standalone use beside the runner's own guard
/// (1), and non-consumption/idempotence (two guards at one position
/// both succeed, 2). Plus the surrounding contracts: the guard's
/// failure on remaining input is empty so `or-else` backtracks past it
/// (3), the failure names "end of input" (4), and the runner's
/// whole-input default is kept (5).
#[test]
fn expect_eof_is_a_non_consuming_guard() {
    let mut program = program_header();
    for line in [
        "  check-sub :: String -> String -> Bool",
        "  check-sub hay needle = has-sub (string-to-bytes hay) (string-to-bytes needle) 0",
        "",
        "  has-sub :: Bytes -> Bytes -> Nat -> Bool",
        "  has-sub hay needle i =",
        "    if i + bytes-len needle > bytes-len hay then False",
        "    else if eq-bytes-at hay needle i 0 then True",
        "    else has-sub hay needle (i + 1)",
        "",
        "  eq-bytes-at :: Bytes -> Bytes -> Nat -> Nat -> Bool",
        "  eq-bytes-at hay needle i j =",
        "    if nat-cmp j (bytes-len needle) == 0 then True",
        "    else if i + j >= bytes-len hay then False",
        "    else if nat-cmp (bytes-get hay (i + j)) (bytes-get needle j) == 0 then eq-bytes-at hay needle i (j + 1)",
        "    else False",
        "",
        "  main :: Int",
        "  main = standalone-eof",
        "",
        "  -- (1) A user parser ending in `expect-eof` parses under the",
        "  -- runner, whose appended whole-input guard runs the guard a",
        "  -- second time.",
        "  standalone-eof :: Int",
        "  standalone-eof =",
        "    case run-parser (\"standalone\") (bytes-from-list [120]) (parse-bind satisfy-var (\\w -> parse-bind expect-eof (\\_ -> parse-pure w))) of",
        "      Parsed x soft -> guard-twice",
        "      Failed b -> 1",
        "",
        "  -- (2) The guard consumes nothing: a second `expect-eof` at the",
        "  -- same position still sees the end of input and succeeds.",
        "  guard-twice :: Int",
        "  guard-twice =",
        "    case run-parser (\"guard-twice\") (bytes-new 0) (parse-bind expect-eof (\\_ -> parse-bind expect-eof (\\_ -> parse-pure ()))) of",
        "      Parsed x soft -> backtrack-guard",
        "      Failed b -> 2",
        "",
        "  -- (3) The guard's failure on remaining input is empty, so",
        "  -- `or-else` backtracks past it to the alternative.",
        "  backtrack-guard :: Int",
        "  backtrack-guard =",
        "    case run-parser (\"backtrack\") (bytes-from-list [120]) (or-else (parse-bind expect-eof (\\_ -> fail-p (\"guarded\"))) (parse-fmap (\\w -> w) satisfy-var)) of",
        "      Parsed x soft -> eof-named",
        "      Failed b -> 3",
        "",
        "  -- (4) The guard failing names the end of input in the bundle.",
        "  eof-named :: Int",
        "  eof-named =",
        "    case run-parser (\"named\") (bytes-from-list [120, 32, 121]) (parse-bind satisfy-var (\\_ -> expect-eof)) of",
        "      Parsed x soft -> 4",
        "      Failed b -> if check-sub (render-bundle b) (\"end of input\") then whole-input else 5",
        "",
        "  -- (5) The runner's whole-input default is kept: a parser with",
        "  -- no `expect-eof` of its own still refuses trailing input.",
        "  whole-input :: Int",
        "  whole-input =",
        "    case run-parser (\"whole\") (bytes-from-list [120, 32, 121]) satisfy-var of",
        "      Parsed x soft -> 6",
        "      Failed b -> 0",
    ] {
        program.push_str(line);
        program.push('\n');
    }
    expect_exit("expect-eof-guard", &program, 0);
}

// ----------------------------------------------------------------
// Top-level value bindings: the eager-reference-cycle refusal (D302).
// ----------------------------------------------------------------

/// A `Main` that must compile and run through the driver's own path.
fn expect_src_exit(name: &str, src: &str, expected: i32) {
    match cinch::driver::run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// A `Main` that must be refused at check time, with `needle` in the
/// diagnostic.
fn expect_src_err(name: &str, src: &str, needle: &str) {
    match cinch::driver::check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// The D302 compiler defect (recorded with the P30 toolkit report):
/// eager mutual recursion between top-level VALUES crashed the compiled
/// binary (signal-killed) because the `letrec` fixpoint discipline does
/// not reach top-level value bindings. The discipline now reaches them
/// as the guarded-letrec rule: an eager reference cycle refuses at
/// check time with the named diagnostic. The working lazy case is
/// pinned beside it: mutual recursion whose references sit under a
/// lambda (the toolkit's `MkParser` wrapping workaround) runs as ever.
#[test]
fn top_level_lazy_value_recursion_runs() {
    expect_src_exit(
        "top-lazy-mutual",
        r#"
module Main where
  va :: Int -> Int
  va = \n -> if n == 0 then 0 else vb (n - 1)
  vb :: Int -> Int
  vb = \n -> if n == 0 then 1 else va (n - 1)
  main :: Int
  main = if va 4 == 0 then (if vb 5 == 0 then (if vb 4 == 1 then 0 else 1) else 1) else 1
"#,
        0,
    );
    // An eager reference in an acyclic group stays ordinary: the
    // values' thunks make the order free (`a` may be written first).
    expect_src_exit(
        "top-acyclic-forward",
        r#"
module Main where
  a :: Int
  a = b + 1
  b :: Int
  b = 5
  main :: Int
  main = if a == 6 then 0 else 1
"#,
        0,
    );
}

/// The diagnostic: an eager self reference and eager mutual recursion
/// between top-level values refuse at check time, naming the referenced
/// value and the fix (the wording wanted at D302), never crashing the
/// binary.
#[test]
fn top_level_eager_value_cycle_is_a_named_diagnostic() {
    expect_src_err(
        "top-eager-self",
        r#"
module Main where
  a :: Int
  a = a + 1
  main :: Int
  main = a
"#,
        "top-level value a is part of an eager reference cycle; wrap its reference in a lambda or use letrec",
    );
    expect_src_err(
        "top-eager-mutual",
        r#"
module Main where
  a :: Int
  a = b + 1
  b :: Int
  b = a + 1
  main :: Int
  main = a
"#,
        "top-level value b is part of an eager reference cycle; wrap its reference in a lambda or use letrec",
    );
    // The cycle through an eager call is the same defect: the callee's
    // body runs while the value's initializer runs.
    expect_src_err(
        "top-eager-call",
        r#"
module Main where
  x :: Int
  x = f 1
  f :: Int -> Int
  f n = x
  main :: Int
  main = x
"#,
        "is part of an eager reference cycle; wrap its reference in a lambda or use letrec",
    );
}
