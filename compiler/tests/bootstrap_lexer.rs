//! The lexing-coverage prerequisite (bootstrap/README.md): `Text.Lex`
//! must lex what the compiler's own sources contain. This test lexes a
//! corpus of real `.cinch` sources through the runnable driver program
//! `bootstrap/programs/Run.Lex.cinch`, plus two synthetic self-hosting
//! sources (attributes and tick names), and asserts two things:
//!
//!   (a) every source lexes without a `LexedFail`, observed through the
//!       program (which writes one `<name>.lex` per listed source and
//!       returns the number that failed as its exit code), and
//!   (b) for every source the bootstrap compiler's own lexer and layout
//!       pass can read, the token kinds and their order agree token for
//!       token with that compiler, canonically rendered
//!       (`kind:spelling@line:col[!v]`, the contract
//!       `compiler/tests/parsers.rs` states for the shared corpus).
//!
//! One run of the program per source. A run per source keeps each process
//! small and makes the runtime's `Exhausted` exit (73: the live heap past
//! `CINCH_HEAP_LIMIT`, or the stack overflowing) name the source that was
//! too big instead of failing the sweep.
//!
//! The default corpus is BOUNDED: the `prim` acceptance fixtures (which
//! carry the attribute, `prim` body and tick lexemes this rung adds), the
//! smallest prelude modules that exercise layout, and the synthetic
//! sources. The whole-prelude sweep the ladder's contract asks for is
//! `the_whole_prelude_lexes_and_agrees_with_the_compiler` below, marked
//! `#[ignore]` because it is not green yet: it runs the same assertions
//! over every prelude module, both example trees, the toolkit and the
//! synthetics, one bounded run per source, and the sweep stops at
//! `prelude/Data/BigDecimal.cinch`, where the compiled program dies on
//! SIGSEGV with no diagnostic (a runtime crash, not a `LexedFail`). Run it
//! with:
//!
//!   cargo test --test bootstrap_lexer -- --ignored --nocapture
//!
//! The program is a real artifact, built through `cinch::driver` and run
//! with its project directory as the working directory (it reads and writes
//! relative paths), never a program inlined in this file.

mod common;

use std::path::Path;

/// The runnable lexing-coverage program: it lexes the sources `corpus.txt`
/// names (or, without one, every `.cinch` file in its working directory)
/// and writes each one's rendered token stream.
const RUN: &str = include_str!("../../bootstrap/programs/Run.Lex.cinch");

/// A synthetic source exercising every attribute form whose raw inner
/// text the compiler's own `Attr` token reconstructs exactly.
const ATTRS: &str = "module M where\n  #[inline]\n  f :: Nat -> Nat\n  #[weight(3)]\n  g :: Nat -> Nat\n  #[repr(C)]\n  h :: Nat -> Nat\n  #[flag=true]\n  i :: Nat -> Nat\n  #[trim=on ]\n  j :: Nat -> Nat\n  f = 1\n  g = 2\n  h = 3\n  i = 4\n  j = 5\n";

/// A synthetic source exercising the tick lexeme's name forms beside the
/// `Char` literal form.
const TICKS: &str = "module M where\n  f :: Nat -> Nat\n  f n = n\n  'a'\n  'name\n  ''Nat\n  'b\n  g :: Nat -> Nat\n  g n = n\n";

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

/// The compiler's `Attr` spelling reconstructed as the raw inner text
/// `Text.Lex`'s `TAttr` carries. The compiler stores a paren group's
/// content WITHOUT its outer parens, and the `=` form's text with the
/// `=` included and already trimmed (`compiler/src/lexer.rs`'s
/// `lex_attr`), so the `=` case is the name followed by that text
/// verbatim and every other case is the name with a parenthesized group.
fn attr_spelling(name: &str, args: &Option<String>) -> String {
    match args {
        None => name.to_string(),
        Some(a) if a.starts_with('=') => format!("{name}{a}"),
        Some(a) => format!("{name}({a})"),
    }
}

/// The canonical `kind:spelling@line:col[!v]` rendering, matching
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
        Tok::SymLit(s) => ("sym", s.clone()),
        Tok::TypeNameLit(s) => ("typename", s.clone()),
        Tok::Attr { name, args } => ("attr", attr_spelling(name, args)),
        Tok::Fragment(s) => ("frag", s.clone()),
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
    let mut out = format!("{kind}:{}@{}:{}", escape_spelling(&spelling), tok.span.line, tok.span.col);
    if tok.virtual_ {
        out.push_str("!v");
    }
    out
}

/// The compiler's own token stream for a source, canonically rendered and
/// newline joined, or `None` when the compiler itself cannot read it.
fn expected_stream(src: &str) -> Option<String> {
    let toks = cinch::lexer::lex(src).ok()?;
    let toks = cinch::layout::layout(toks).ok()?;
    Some(toks.iter().map(canon).collect::<Vec<_>>().join("\n"))
}

/// One repo source, labeled by its path relative to the compiler crate.
fn real(root: &Path, rel: &str) -> (String, String) {
    let path = root.join(rel);
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    (rel.to_string(), src)
}

/// Every `.cinch` source under `dir` (recursively), labeled by its path.
fn collect(dir: &Path, out: &mut Vec<(String, String)>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for e in entries {
        let p = e.expect("dir entry").path();
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().map(|x| x == "cinch").unwrap_or(false) {
            let src = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
            out.push((p.display().to_string(), src));
        }
    }
}

/// The bounded corpus: small real sources, and the two synthetic ones.
/// The `prim` fixtures carry the attribute, `prim` body and tick lexemes
/// whose tokens this rung adds; the prelude modules are the smallest real
/// modules that exercise layout (nesting, `where`, `case`, comments).
fn bounded_corpus(root: &Path) -> Vec<(String, String)> {
    let mut corpus = Vec::new();
    for rel in [
        "tests/fixtures/prim/bodies.cinch",
        "tests/fixtures/prim/laws.cinch",
        "prelude/Relation/Eq.cinch",
        "prelude/Data/Compose.cinch",
        "prelude/Data/Ordering.cinch",
    ] {
        corpus.push(real(root, rel));
    }
    corpus.push(("synthetic-attrs.cinch".into(), ATTRS.to_string()));
    corpus.push(("synthetic-ticks.cinch".into(), TICKS.to_string()));
    corpus
}

/// The whole corpus: every prelude module, both example trees, the `prim`
/// fixtures, the toolkit the program ships, and the synthetics.
fn whole_corpus(root: &Path) -> Vec<(String, String)> {
    let mut corpus = Vec::new();
    collect(&root.join("prelude"), &mut corpus);
    collect(&root.join("../examples/compiler"), &mut corpus);
    collect(&root.join("../examples/source"), &mut corpus);
    collect(&root.join("tests/fixtures/prim"), &mut corpus);
    for (name, src) in common::TOOLKIT.iter() {
        corpus.push((format!("toolkit/{name}"), (*src).to_string()));
    }
    corpus.push(("synthetic-attrs.cinch".into(), ATTRS.to_string()));
    corpus.push(("synthetic-ticks.cinch".into(), TICKS.to_string()));
    corpus.sort_by(|a, b| a.0.cmp(&b.0));
    corpus
}

/// Build the project once and run the built program per source. The exit
/// code is the number of sources the program failed to lex.
fn run_program(exe: &Path, dir: &Path, label: &str) -> i32 {
    let out = std::process::Command::new(exe)
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap_or_else(|e| panic!("run {}: {e}", exe.display()));
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
    match out.status.code() {
        Some(73) => panic!(
            "{label}: Run.Lex took the runtime's Exhausted exit (73): its live heap passed \
             CINCH_HEAP_LIMIT or its stack overflowed (the stderr line names which), so this \
             source needs more than one run is given. stderr:\n{stderr}"
        ),
        Some(code) => code,
        None => panic!("{label}: Run.Lex was signalled ({}), not a lexing failure:\n{stderr}", out.status),
    }
}

/// Drive the program over `corpus`, one run per source, and compare every
/// rendered stream with the compiler's own lexer and layout pass, answering
/// how many sources the two agreed on. `min_compared` is the floor this test
/// insists on, so a corpus that silently shrank fails rather than passes.
fn lex_and_compare(tag: &str, corpus: &[(String, String)], min_compared: usize) -> usize {
    // The project: the toolkit (the program imports `Text.Lex` and
    // `Text.Builder` from it) and the driver program itself. Each source
    // goes in a `corpus/` subdirectory and is named by `corpus.txt`: the
    // compiler indexes only the entry directory, so a corpus source is data
    // for the lexer, never a module it compiles.
    let mut proj: Vec<(&str, &str)> = common::TOOLKIT.iter().map(|(n, s)| (*n, *s)).collect();
    proj.push(("Main.cinch", RUN));
    let dir = common::project(tag, &proj);
    std::fs::create_dir_all(dir.join("corpus")).unwrap();
    let exe = common::build(&dir, "Main.cinch", "program")
        .unwrap_or_else(|e| panic!("build Run.Lex: {e}"));

    let mut compared = 0usize;
    for (i, (label, src)) in corpus.iter().enumerate() {
        let name = format!("corpus/c{i:04}.cinch");
        std::fs::write(dir.join(&name), src).unwrap();
        std::fs::write(dir.join("corpus.txt"), format!("{name}\n")).unwrap();

        let code = run_program(&exe, &dir, label);
        assert_eq!(code, 0, "{label}: Text.Lex refused this source (exit = the number that failed)");

        let lexed = std::fs::read_to_string(dir.join(format!("{name}.lex")))
            .unwrap_or_else(|e| panic!("{label}: no .lex output: {e}"));
        assert!(
            !lexed.starts_with("FAIL "),
            "{label}: Text.Lex refused a source this corpus requires it to lex: {}",
            lexed.trim()
        );
        if let Some(exp) = expected_stream(src) {
            let got = lexed.strip_suffix('\n').unwrap_or(&lexed);
            assert_eq!(got, exp, "{label}: the token stream drifts from the compiler's");
            compared += 1;
        }
    }
    assert!(
        compared >= min_compared,
        "expected at least {min_compared} sources to agree with the compiler, compared only {compared}"
    );
    common::cleanup(&dir);
    compared
}

/// (a) every source in the bounded corpus lexes without a `LexedFail` and
/// (b) each source the compiler can read agrees token for token.
#[test]
fn lexer_reads_the_corpus_and_agrees_with_the_compiler() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let corpus = bounded_corpus(root);
    assert!(corpus.len() >= 7, "the bounded corpus must hold at least 7 sources, found {}", corpus.len());
    let compared = lex_and_compare("lexer", &corpus, corpus.len());
    assert!(compared >= corpus.len(), "compared only {compared}");
}

/// The whole-prelude sweep the ladder's contract asks for: every prelude
/// module (55 and up), both example trees, the `prim` fixtures, the toolkit
/// and the synthetics, one bounded program run per source.
#[test]
#[ignore = "not green yet: the sweep stops at prelude/Data/BigDecimal.cinch, where the compiled program dies on SIGSEGV (a runtime crash, not a LexedFail), and it needs minutes for the rest"]
fn the_whole_prelude_lexes_and_agrees_with_the_compiler() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let corpus = whole_corpus(root);
    assert!(corpus.len() >= 55, "the prelude's modules must be in the corpus, found {}", corpus.len());
    let compared = lex_and_compare("lexer-whole", &corpus, corpus.len());
    assert!(compared >= corpus.len(), "compared only {compared}");
}

/// The self-hosting forms are tokens that carry their text, not refusals:
/// the rendered streams for the synthetic attribute and tick sources name
/// the new kinds and their payloads, checked token for token against the
/// compiler's own reading of the same sources.
#[test]
fn the_self_hosting_forms_lex_to_carrying_tokens() {
    let mut proj: Vec<(&str, &str)> = common::TOOLKIT.iter().map(|(n, s)| (*n, *s)).collect();
    proj.push(("Main.cinch", RUN));
    let dir = common::project("lexer-forms", &proj);
    std::fs::write(dir.join("attrs.cinch"), ATTRS).unwrap();
    std::fs::write(dir.join("ticks.cinch"), TICKS).unwrap();
    std::fs::write(dir.join("corpus.txt"), "attrs.cinch\nticks.cinch\nMain.cinch\n").unwrap();
    let exe = common::build(&dir, "Main.cinch", "program")
        .unwrap_or_else(|e| panic!("build Run.Lex: {e}"));
    let code = run_program(&exe, &dir, "the self-hosting forms");
    assert_eq!(code, 0, "the self-hosting forms must all lex");

    let attrs = std::fs::read_to_string(dir.join("attrs.cinch.lex")).unwrap();
    let ticks = std::fs::read_to_string(dir.join("ticks.cinch.lex")).unwrap();

    for want in ["attr:inline@", "attr:weight(3)@", "attr:repr(C)@", "attr:flag=true@", "attr:trim=on@"] {
        assert!(attrs.contains(want), "attrs.cinch must render `{want}`:\n{attrs}");
    }
    for want in ["char:a@", "sym:name@", "typename:Nat@", "sym:b@"] {
        assert!(ticks.contains(want), "ticks.cinch must render `{want}`:\n{ticks}");
    }

    // The carried text is the compiler's own (`Attr`'s inner text, the tick
    // names), not merely present: both streams agree token for token.
    for (name, src) in [("attrs.cinch", ATTRS), ("ticks.cinch", TICKS)] {
        if let Some(exp) = expected_stream(src) {
            let got = std::fs::read_to_string(dir.join(format!("{name}.lex"))).unwrap();
            assert_eq!(got.strip_suffix('\n').unwrap_or(&got), exp, "{name}: drifts from the compiler's");
        }
    }

    common::cleanup(&dir);
}
