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
const PARSE_SRC: &str = include_str!("../prelude/Text/Parse.cinch");

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

/// A self-contained `(chunk-bytes ...)` expression over the source's
/// bytes (string literals are compile-time only in the language,
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
    format!("(String {})", bytes_lit(s))
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
        program.push_str("            (string-concat (text ['\\n']) (render-all rest))\n\n");
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
