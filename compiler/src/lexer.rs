//! Tokenizer for the draft 0.2 surface syntax.
//!
//! Lexical rules follow `specifications/surface-syntax-and-grammar.md`:
//! comment recognition wins over symbol identifiers, reserved punctuation is
//! matched longest first, tabs are rejected in indentation, identifiers are
//! ASCII, string and character contents are Unicode scalar values.

use crate::{Error, Result};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub line: u32,
    pub col: u32,
}

/// Expansion provenance for generated syntax (modules specification
/// section 8: source maps retain expansion provenance for diagnostics):
/// which expansion produced a node, its invocation site, and the enclosing
/// expansion when generation happens inside generated code.
#[derive(Clone, Debug)]
pub struct ExpSite {
    /// The macro or generator that produced the syntax.
    pub name: String,
    /// The invocation site in the caller's source.
    pub call: Span,
    /// The file the template came from, for the rendered form.
    pub origin: String,
    /// The template's own span, in `origin`'s coordinates.
    pub template: Span,
    /// The expansion this one happened inside, if any.
    pub parent: Option<Span>,
}

thread_local! {
    static EXPANSIONS: std::cell::RefCell<Vec<ExpSite>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// The marker `col` of a generated span; `line` indexes the expansion
/// arena (one-based). Real spans never carry this column.
const EXP_COL: u32 = u32::MAX;

/// Record one expansion and return a span that renders its provenance.
/// Generated syntax carries such spans so diagnostics trace back through
/// expansions to the template and the call site.
pub fn record_expansion(site: ExpSite) -> Span {
    EXPANSIONS.with(|e| {
        let mut e = e.borrow_mut();
        e.push(site);
        Span { line: e.len() as u32, col: EXP_COL }
    })
}

/// Is `span` a generated span carrying expansion provenance?
pub fn is_generated(span: Span) -> bool {
    span.col == EXP_COL
}

/// The provenance recorded in a generated span.
pub fn expansion_of(span: Span) -> Option<ExpSite> {
    if span.col != EXP_COL {
        return None;
    }
    EXPANSIONS.with(|e| e.borrow().get(span.line.saturating_sub(1) as usize).cloned())
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match expansion_of(*self) {
            Some(site) => {
                write!(f, "{}:{}:{} (in expansion of {} called at {}:{})", site.origin, site.template.line, site.template.col, site.name, site.call.line, site.call.col)?;
                let mut parent = site.parent;
                while let Some(p) = parent {
                    match expansion_of(p) {
                        Some(outer) => {
                            write!(f, ", expanded from `{}` at {}:{}", outer.name, outer.call.line, outer.call.col)?;
                            parent = outer.parent;
                        }
                        None => break,
                    }
                }
                Ok(())
            }
            None => write!(f, "{}:{}", self.line, self.col),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    // Names.
    VarId(String),
    ConId(String),
    /// Symbol identifier that is not reserved punctuation.
    Op(String),

    // Literals.
    /// Integer literal: the decimal digit string with a leading `-` for a
    /// signed literal or folded prefix negation (same form as `Pat::Int`);
    /// no other leading zeros.
    Int(String),
    FloatLit(String),
    StrLit(String),
    CharLit(String),
    /// `'name`: the tick lexeme's symbol form (surface grammar local
    /// amendments section 12). In expression or pattern position it is a
    /// `Sym` literal; in a staging or macro name position it is the value
    /// `Name` of the spelling. The name is the identifier spelling after
    /// the tick (`lex_word`'s shape, dashes included).
    SymLit(String),
    /// `''T`: the tick lexeme's type-name form, the type `Name` of a
    /// staging or macro name position (section 12).
    TypeNameLit(String),

    // Reserved words.
    Module,
    Where,
    Data,
    Record,
    Codata,
    Effect,
    Signature,
    Instance,
    Implicit,
    Import,
    As,
    TypeKw,
    Opaque,
    Forall,
    Sigma,
    Using,
    In,
    Let,
    Letrec,
    If,
    Then,
    Else,
    Case,
    Of,
    Do,
    Handle,
    With,
    Rewrite,
    Target,
    Justify,
    MatchKw,
    Replace,
    Require,
    Upgrade,
    /// `prim` (primitive operations and inline target IR specification
    /// section 2): a primitive operation with inline target-IR bodies.
    Prim,
    /// `law` (primitive operations and inline target IR specification
    /// section 5): a class law declaration, citable as `justify`
    /// evidence by identity.
    Law,
    /// `contract` (core specification section 10): a temporal contract
    /// declaration.
    Contract,
    To,
    Generate,
    QuoteDecls,
    Foreign,
    Export,
    Ccall,
    Component,
    Proof,
    Spec,
    Stage,
    Corec,
    Macro,
    Infix,
    Infixl,
    Infixr,
    Erased,
    Own,
    Return,
    Resume,

    // Reserved punctuation.
    ColonColon,
    Arrow,
    LArrow,
    Darrow,
    /// `[|` typed quote open.
    LQuote,
    /// `|]` typed quote close.
    RQuote,
    /// `$(` splice open.
    DollarParen,
    MultZero,
    MultOne,
    MultAff,
    MultMany,
    Bang,
    At,
    Dot,
    /// Bare `..`, the range-literal separator (reserved punctuation;
    /// `1..10` lexes as one integer, `..`, one integer since a float
    /// needs a digit after its dot).
    DotDot,
    Equals,
    Pipe,
    Dollar,
    Backslash,
    /// Bare `~`, reserved for memo bindings. Symbol runs like `~>` remain
    /// ordinary operators.
    Tilde,

    /// One `#[name]` or `#[name(args)]` attribute (attribute annotations
    /// specification, section 1). `args` is the raw argument text: the
    /// parentheses content, or the `= literal` tail of the named form.
    Attr { name: String, args: Option<String> },

    /// The verbatim inner text of a `prim` body's inline target-IR
    /// fragment (primitive operations and inline target IR
    /// specification section 2): the lexer captures the balanced `{
    /// ... }` region as raw text so fragment grammars (LLVM IR, native
    /// assembly) never pass through the surface tokenizer. The token
    /// sits between its real `LBrace` and `RBrace` tokens so the layout
    /// pass still sees balanced braces.
    Fragment(String),

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Underscore,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
    /// True for `{`, `;`, `}` inserted by the layout pass.
    pub virtual_: bool,
}

impl Token {
    pub fn is(&self, t: &Tok) -> bool {
        &self.tok == t
    }
}

/// Symbol identifier characters, excluding reserved `$`, `\` and `.`.
fn is_symbol_char(c: char) -> bool {
    matches!(c, '!' | '#' | '%' | '&' | '*' | '+' | '/' | '<' | '=' | '>' | '?' | '@' | '^' | '|' | '~' | ':' | '-')
}

/// Identifier characters (`lex_word`'s set): ASCII alphanumeric, `_`, `'`.
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '\''
}

/// The identifier characters a dash may glue onto inside a word. Digits do
/// not continue an identifier across a dash (surface grammar local
/// amendments section 8), so `x-1` is subtraction while `x-y` is one name.
fn is_dash_glue_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '\''
}

/// Characters that end an expression and so keep a following `-` in
/// operator position: `f ()-1` and `x.y-1` subtract, while `f -42` and
/// `(-42)` sign a literal (surface grammar local amendments section 8).
fn is_expr_end_char(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '"' | '\'')
}

fn keyword(w: &str) -> Option<Tok> {
    use Tok::*;
    Some(match w {
        "module" => Module,
        "where" => Where,
        "data" => Data,
        "record" => Record,
        "codata" => Codata,
        "effect" => Effect,
        "signature" => Signature,
        "instance" => Instance,
        "implicit" => Implicit,
        "import" => Import,
        "as" => As,
        "type" => TypeKw,
        "opaque" => Opaque,
        "forall" => Forall,
        "Sigma" => Sigma,
        "using" => Using,
        "in" => In,
        "let" => Let,
        "letrec" => Letrec,
        "if" => If,
        "then" => Then,
        "else" => Else,
        "case" => Case,
        "of" => Of,
        "do" => Do,
        "handle" => Handle,
        "with" => With,
        "rewrite" => Rewrite,
        "prim" => Prim,
        "law" => Law,
        "target" => Target,
        "justify" => Justify,
        "match" => MatchKw,
        "replace" => Replace,
        "require" => Require,
        "upgrade" => Upgrade,
        "contract" => Contract,
        "to" => To,
        "generate" => Generate,
        "quoteDecls" => QuoteDecls,
        "foreign" => Foreign,
        "export" => Export,
        "ccall" => Ccall,
        "component" => Component,
        "proof" => Proof,
        "spec" => Spec,
        "stage" => Stage,
        "corec" => Corec,
        "macro" => Macro,
        "infix" => Infix,
        "infixl" => Infixl,
        "infixr" => Infixr,
        "erased" => Erased,
        "own" => Own,
        "return" => Return,
        "resume" => Resume,
        _ => return None,
    })
}

/// `dec = dec * mul + add` on a canonical decimal digit string.
fn dec_mul_add(dec: &mut String, mul: u32, add: u32) {
    let mut carry = add;
    let bytes = dec.as_bytes();
    let mut out = vec![0u8; bytes.len()];
    for i in (0..bytes.len()).rev() {
        let x = (bytes[i] - b'0') as u32 * mul + carry;
        out[i] = b'0' + (x % 10) as u8;
        carry = x / 10;
    }
    let mut s = String::new();
    while carry > 0 {
        s.push(char::from_digit(carry % 10, 10).unwrap());
        carry /= 10;
    }
    let s: String = s.chars().rev().collect();
    dec.clear();
    dec.push_str(&s);
    dec.push_str(std::str::from_utf8(&out).unwrap());
}

/// Canonical decimal digit string: no leading zeros (`"0"` for zero).
fn canon_dec(s: &str) -> String {
    let t = s.trim_start_matches('0');
    if t.is_empty() {
        "0".to_string()
    } else {
        t.to_string()
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>> {
    Lexer::new(src).run()
}

struct Lexer {
    cs: Vec<char>,
    i: usize,
    line: u32,
    col: u32,
    /// True while only spaces have been seen on the current line.
    leading: bool,
    /// The innermost `prim` declaration being scanned: the `prim`
    /// keyword's column and whether its `where` has been seen. Inline
    /// fragments capture raw only inside a prim body block.
    prim: Option<(u32, bool)>,
    /// The last scanned word (`llvm`, a record constructor, ...), so a
    /// `family { fragment }` body opener can be recognized.
    last_word: Option<String>,
}

impl Lexer {
    fn new(src: &str) -> Self {
        Lexer {
            cs: src.chars().collect(),
            i: 0,
            line: 1,
            col: 1,
            leading: true,
            prim: None,
            last_word: None,
        }
    }

    fn span(&self) -> Span {
        Span { line: self.line, col: self.col }
    }

    fn peek(&self) -> Option<char> {
        self.cs.get(self.i).copied()
    }

    fn peek_at(&self, n: usize) -> Option<char> {
        self.cs.get(self.i + n).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.cs.get(self.i).copied()?;
        self.i += 1;
        match c {
            '\n' => {
                self.line += 1;
                self.col = 1;
                self.leading = true;
            }
            '\r' => {
                if self.peek() != Some('\n') {
                    self.line += 1;
                    self.col = 1;
                    self.leading = true;
                }
            }
            _ => {
                self.col += 1;
                if !c.is_whitespace() {
                    self.leading = false;
                }
            }
        }
        Some(c)
    }

    fn run(mut self) -> Result<Vec<Token>> {
        let mut out = Vec::new();
        loop {
            self.skip_blank()?;
            let span = self.span();
            let at_line_start = self.leading;
            let Some(c) = self.peek() else {
                out.push(Token { tok: Tok::Eof, span, virtual_: false });
                return Ok(out);
            };
            // A `prim` body's inline fragment: a registered target-IR
            // family name's `{` opens a raw capture (primitive
            // operations and inline target IR specification section 2;
            // the attribute chapter's `#[...]` raw arguments show the
            // pattern for foreign token content).
            if c == '{' && self.at_prim_fragment() {
                let (open, text, close) = self.lex_fragment(span)?;
                out.push(Token { tok: Tok::LBrace, span: open, virtual_: false });
                out.push(Token { tok: Tok::Fragment(text), span: open, virtual_: false });
                out.push(Token { tok: Tok::RBrace, span: close, virtual_: false });
                self.last_word = None;
                continue;
            }
            // `$(` is the splice opener, but the layout pass must see `(`
            // as its own delimiter (it shields layout and balances `)`):
            // emit `Dollar` + `LParen` sharing one span, and the parser
            // reconstitutes a splice from an adjacent pair (surface
            // grammar: `$(` is reserved punctuation, so the pair is only
            // one token when written together).
            if c == '$' && self.peek_at(1) == Some('(') {
                self.bump();
                self.bump();
                self.note(&Tok::Dollar, span, at_line_start);
                out.push(Token { tok: Tok::Dollar, span, virtual_: false });
                out.push(Token { tok: Tok::LParen, span, virtual_: false });
                continue;
            }
            let tok = self.scan_token(c, span)?;
            if tok == Tok::Letrec {
                // `letrec` is the `let` keyword with a recursion marker
                // (`$(` precedent: two tokens sharing one span). The pair
                // keeps `let`'s layout block opening at the first binding.
                self.note(&tok, span, at_line_start);
                out.push(Token { tok: Tok::Letrec, span, virtual_: false });
                out.push(Token { tok: Tok::Let, span, virtual_: false });
                continue;
            }
            self.note(&tok, span, at_line_start);
            out.push(Token { tok, span, virtual_: false });
        }
    }

    /// Track the `prim` declaration context (for raw fragment capture)
    /// and the last word (the fragment's family name).
    fn note(&mut self, tok: &Tok, span: Span, at_line_start: bool) {
        match tok {
            Tok::VarId(w) | Tok::ConId(w) => self.last_word = Some(w.clone()),
            _ => self.last_word = None,
        }
        if matches!(tok, Tok::Prim) {
            self.prim = Some((span.col, false));
            return;
        }
        if let Some((prim_col, seen_where)) = &mut self.prim {
            if matches!(tok, Tok::Where) {
                *seen_where = true;
            }
            // The body block ends at the next token on the `prim`
            // keyword's own (or an outer) indentation level.
            if at_line_start && span.col <= *prim_col {
                self.prim = None;
            }
        }
    }

    /// A `prim` body's fragment opens here: a registered target-IR
    /// family name followed by `{` inside the body block.
    fn at_prim_fragment(&self) -> bool {
        let Some((_, seen_where)) = self.prim else { return false };
        seen_where
            && matches!(
                self.last_word.as_deref(),
                Some("llvm" | "wasm" | "asm" | "clr" | "jvm")
            )
    }

    /// Capture a `prim` body's fragment verbatim: the balanced `{
    /// ... }` region after the family name, nesting counted and
    /// `"..."` string literals skipped (a brace inside a string does
    /// not count). Returns the open brace's span, the inner text and
    /// the close brace's span.
    fn lex_fragment(&mut self, open: Span) -> Result<(Span, String, Span)> {
        self.bump(); // the opening brace
        let text_start = self.i;
        let mut depth = 1usize;
        let mut in_str = false;
        loop {
            match self.peek() {
                None => return Err(Error::at(open, "unterminated `prim` body fragment")),
                Some('"') if in_str => {
                    in_str = false;
                    self.bump();
                }
                Some('\\') if in_str => {
                    self.bump();
                    self.bump();
                }
                Some('"') => {
                    in_str = true;
                    self.bump();
                }
                Some('{') if !in_str => {
                    depth += 1;
                    self.bump();
                }
                Some('}') if !in_str => {
                    let close = self.span();
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        let text: String = self.cs[text_start..self.i - 1].iter().collect();
                        return Ok((open, text, close));
                    }
                }
                Some(_) => {
                    self.bump();
                }
            }
        }
    }

    fn skip_blank(&mut self) -> Result<()> {
        loop {
            match self.peek() {
                Some(' ') => {
                    self.bump();
                }
                Some('\t') if self.leading => {
                    return Err(Error::at(self.span(), "tabs are rejected in indentation"));
                }
                Some('\t') | Some('\n') | Some('\r') => {
                    self.bump();
                }
                Some('-') if self.peek_at(1) == Some('-') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' || c == '\r' {
                            break;
                        }
                        self.bump();
                    }
                }
                Some('{') if self.peek_at(1) == Some('-') => {
                    self.bump();
                    self.bump();
                    let mut depth = 1usize;
                    loop {
                        match (self.peek(), self.peek_at(1)) {
                            (Some('-'), Some('}')) => {
                                self.bump();
                                self.bump();
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            (Some('{'), Some('-')) => {
                                self.bump();
                                self.bump();
                                depth += 1;
                            }
                            (Some(_), _) => {
                                self.bump();
                            }
                            (None, _) => {
                                return Err(Error::at(self.span(), "unterminated block comment"));
                            }
                        }
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn scan_token(&mut self, c: char, span: Span) -> Result<Tok> {
        match c {
            '.' if self.peek_at(1) == Some('.') => {
                self.bump();
                self.bump();
                Ok(Tok::DotDot)
            }
            '(' | ')' | ']' | '{' | '}' | ',' | ';' | '.' => {
                self.bump();
                Ok(match c {
                    '(' => Tok::LParen,
                    ')' => Tok::RParen,
                    ']' => Tok::RBracket,
                    '{' => Tok::LBrace,
                    '}' => Tok::RBrace,
                    ',' => Tok::Comma,
                    ';' => Tok::Semicolon,
                    '.' => Tok::Dot,
                    _ => unreachable!(),
                })
            }
            '[' => {
                if self.peek_at(1) == Some('|') {
                    self.bump();
                    self.bump();
                    Ok(Tok::LQuote)
                } else {
                    self.bump();
                    Ok(Tok::LBracket)
                }
            }
            '|' => {
                if self.peek_at(1) == Some(']') {
                    self.bump();
                    self.bump();
                    Ok(Tok::RQuote)
                } else {
                    Ok(self.lex_symbol())
                }
            }
            '#' if self.peek_at(1) == Some('[') => self.lex_attr(span),
            '%' => match self.lex_multiplicity() {
                Some(t) => Ok(t),
                None => Ok(self.lex_symbol()),
            },
            '$' => {
                self.bump();
                Ok(Tok::Dollar)
            }
            '\\' => {
                self.bump();
                Ok(Tok::Backslash)
            }
            '-' => self.lex_minus(),
            '"' => self.lex_string(),
            '\'' => self.lex_tick(span),
            c if c.is_ascii_digit() => self.lex_number(span),
            c if c.is_ascii_alphabetic() || c == '_' => Ok(self.lex_word()),
            c if is_symbol_char(c) => Ok(self.lex_symbol()),
            _ => Err(Error::at(span, format!("unexpected character {c:?}"))),
        }
    }

    /// `%0`, `%1`, `%aff`, `%many` are reserved multiplicities; any other
    /// symbol run starting with `%` is an ordinary operator.
    fn lex_multiplicity(&mut self) -> Option<Tok> {
        fn boundary(c: Option<char>) -> bool {
            !c.map(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'')
                .unwrap_or(false)
        }
        match self.peek_at(1) {
            Some('0') if boundary(self.peek_at(2)) => {
                self.bump();
                self.bump();
                Some(Tok::MultZero)
            }
            Some('1') if boundary(self.peek_at(2)) => {
                self.bump();
                self.bump();
                Some(Tok::MultOne)
            }
            Some('a')
                if self.peek_at(2) == Some('f')
                    && self.peek_at(3) == Some('f')
                    && boundary(self.peek_at(4)) =>
            {
                for _ in 0..4 {
                    self.bump();
                }
                Some(Tok::MultAff)
            }
            Some('m')
                if self.peek_at(2) == Some('a')
                    && self.peek_at(3) == Some('n')
                    && self.peek_at(4) == Some('y')
                    && boundary(self.peek_at(5)) =>
            {
                for _ in 0..5 {
                    self.bump();
                }
                Some(Tok::MultMany)
            }
            _ => None,
        }
    }

    /// `#[name]`, `#[name(args)]`, or `#[name = literal]`: one attribute
    /// token (attribute annotations specification, section 1). The name is
    /// dotted; the argument text is captured raw for per-family validation.
    fn lex_attr(&mut self, span: Span) -> Result<Tok> {
        self.bump(); // `#`
        self.bump(); // `[`
        let mut name = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '.' || c == '-' {
                name.push(c);
                self.bump();
            } else {
                break;
            }
        }
        if name.is_empty()
            || name.starts_with('.')
            || name.ends_with('.')
            || name.starts_with('-')
            || name.ends_with('-')
        {
            return Err(Error::at(span, "an attribute needs a name"));
        }
        let mut args = None;
        match self.peek() {
            Some('(') => {
                self.bump();
                let mut text = String::new();
                let mut depth = 1usize;
                loop {
                    match self.peek() {
                        Some('"') => {
                            // Keep string contents opaque so a `)` inside
                            // one does not close the arguments.
                            text.push('"');
                            self.bump();
                            loop {
                                match self.peek() {
                                    Some('\\') => {
                                        text.push('\\');
                                        self.bump();
                                        if let Some(c) = self.peek() {
                                            text.push(c);
                                            self.bump();
                                        }
                                    }
                                    Some('"') => {
                                        text.push('"');
                                        self.bump();
                                        break;
                                    }
                                    Some(c) => {
                                        text.push(c);
                                        self.bump();
                                    }
                                    None => {
                                        return Err(Error::at(
                                            span,
                                            "unterminated attribute arguments",
                                        ))
                                    }
                                }
                            }
                        }
                        Some('(') => {
                            depth += 1;
                            text.push('(');
                            self.bump();
                        }
                        Some(')') => {
                            depth -= 1;
                            self.bump();
                            if depth == 0 {
                                break;
                            }
                            text.push(')');
                        }
                        Some(c) => {
                            text.push(c);
                            self.bump();
                        }
                        None => return Err(Error::at(span, "unterminated attribute arguments")),
                    }
                }
                args = Some(text);
            }
            Some('=') => {
                self.bump();
                let mut text = "=".to_string();
                while let Some(c) = self.peek() {
                    if c == ']' {
                        break;
                    }
                    text.push(c);
                    self.bump();
                }
                args = Some(text.trim().to_string());
            }
            _ => {}
        }
        if self.peek() != Some(']') {
            return Err(Error::at(span, "unterminated attribute"));
        }
        self.bump();
        Ok(Tok::Attr { name, args })
    }

    /// `-` tight only to what follows (space before, digits after) is the
    /// sign of a negative literal: `f -42`, `x = -3`, `(-42)` are atoms
    /// with no `negate` needed. Any other `-` is the operator, and
    /// identifier glue is decided inside `lex_word`
    /// (surface grammar local amendments section 8).
    fn lex_minus(&mut self) -> Result<Tok> {
        let span = self.span();
        if self.at_sign_minus() {
            self.bump();
            return Ok(match self.lex_number(span)? {
                Tok::Int(v) => Tok::Int(format!("-{v}")),
                Tok::FloatLit(v) => Tok::FloatLit(format!("-{v}")),
                other => other,
            });
        }
        Ok(self.lex_symbol())
    }

    /// Is the `-` at the cursor the sign of a negative literal? It must be
    /// tight to following digits and tight to nothing that could end an
    /// expression before it.
    fn at_sign_minus(&self) -> bool {
        if !self.peek_at(1).map(|c| c.is_ascii_digit()).unwrap_or(false) {
            return false;
        }
        match self.cs.get(self.i.wrapping_sub(1)) {
            None => true,
            Some(c) if is_ident_char(*c) || is_expr_end_char(*c) => false,
            Some(_) => true,
        }
    }

    fn lex_symbol(&mut self) -> Tok {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            // `$` continues an operator run (`<$>` is the Functor
            // operator, design/base-library.md) but never starts one: a
            // leading `$` is reserved application, and `$(` opens a
            // splice.
            if is_symbol_char(c)
                || (c == '$' && !s.is_empty() && self.peek_at(1) != Some('('))
            {
                // A negative-literal sign never joins a symbol run:
                // `=-3` is `=` applied to the atom `-3`.
                if c == '-' && self.at_sign_minus() {
                    break;
                }
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        match s.as_str() {
            "::" => Tok::ColonColon,
            "->" => Tok::Arrow,
            "<-" => Tok::LArrow,
            "=>" => Tok::Darrow,
            "!" => Tok::Bang,
            "@" => Tok::At,
            "=" => Tok::Equals,
            "|" => Tok::Pipe,
            "~" => Tok::Tilde,
            _ => Tok::Op(s),
        }
    }

    fn lex_word(&mut self) -> Tok {
        let w = self.scan_ident();
        if let Some(kw) = keyword(&w) {
            return kw;
        }
        let first = w.chars().next().unwrap();
        if first.is_ascii_uppercase() {
            Tok::ConId(w)
        } else {
            Tok::VarId(w)
        }
    }

    fn lex_number(&mut self, span: Span) -> Result<Tok> {
        let mut raw = String::new();
        if self.peek() == Some('0') && matches!(self.peek_at(1), Some('x') | Some('X')) {
            raw.push(self.bump().unwrap());
            raw.push(self.bump().unwrap());
            let mut digits = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_hexdigit() {
                    digits.push(c);
                    self.bump();
                } else if c == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
            if digits.is_empty() {
                return Err(Error::at(span, "malformed hexadecimal literal"));
            }
            // Hexadecimal literals are unbounded like decimal ones: fold
            // the digits into a decimal digit string one nybble at a time.
            let mut dec = String::from("0");
            for c in digits.chars() {
                let d = c.to_digit(16).unwrap();
                dec_mul_add(&mut dec, 16, d);
            }
            return Ok(Tok::Int(canon_dec(&dec)));
        }
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                raw.push(c);
                self.bump();
            } else if c == '_' {
                self.bump();
            } else {
                break;
            }
        }
        // A float needs `.` immediately followed by a digit; `.` otherwise
        // starts qualification or projection.
        if self.peek() == Some('.') && self.peek_at(1).map(|c| c.is_ascii_digit()).unwrap_or(false) {
            raw.push(self.bump().unwrap());
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    raw.push(c);
                    self.bump();
                } else if c == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
            if matches!(self.peek(), Some('e') | Some('E')) {
                raw.push(self.bump().unwrap());
                if matches!(self.peek(), Some('+') | Some('-')) {
                    raw.push(self.bump().unwrap());
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        raw.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
            return Ok(Tok::FloatLit(raw));
        }
        let cleaned: String = {
            let src: String = raw.chars().filter(|c| *c != '_').collect();
            src
        };
        Ok(Tok::Int(canon_dec(&cleaned)))
    }

    fn lex_string(&mut self) -> Result<Tok> {
        let span = self.span();
        self.bump(); // opening quote
        let mut s = String::new();
        loop {
            match self.peek() {
                None | Some('\n') | Some('\r') => {
                    return Err(Error::at(span, "unterminated string literal"));
                }
                Some('"') => {
                    self.bump();
                    return Ok(Tok::StrLit(s));
                }
                Some('\\') => {
                    self.bump();
                    let c = self.lex_escape(span)?;
                    s.push(c);
                }
                Some(c) => {
                    self.bump();
                    s.push(c);
                }
            }
        }
    }

    /// The tick lexeme (surface grammar local amendments section 12):
    /// one spelling in three contexts. Char literal precedence is
    /// lexical and deterministic: tick, one character, tick means
    /// `Char`; `'name` is the symbol form (a `Sym` literal in expression
    /// or pattern position, a value `Name` in a staging or macro name
    /// position); `''T` is the type `Name` form.
    fn lex_tick(&mut self, span: Span) -> Result<Tok> {
        // `'c'` (one character, or one escape, between ticks) is a
        // `Char` literal and wins. The probe backtracks into the name
        // forms when the closing tick is not there (`'a` followed by
        // anything else is a symbol).
        let save = (self.i, self.line, self.col, self.leading);
        self.bump(); // opening tick
        let c = match self.peek() {
            None | Some('\n') | Some('\r') => None,
            Some('\\') => {
                self.bump();
                // A backslash can only open a character literal's
                // escape, so a malformed escape is a character-literal
                // error, never a name.
                let c = self.lex_escape(span)?;
                if self.peek() == Some('\'') {
                    self.bump();
                    return Ok(Tok::CharLit(c.to_string()));
                }
                return Err(Error::at(span, "unterminated character literal"));
            }
            Some(c) => {
                self.bump();
                Some(c)
            }
        };
        if let Some(c) = c {
            if self.peek() == Some('\'') {
                self.bump();
                return Ok(Tok::CharLit(c.to_string()));
            }
        }
        // Not a character literal: restore and scan the name forms.
        self.i = save.0;
        self.line = save.1;
        self.col = save.2;
        self.leading = save.3;
        self.bump(); // opening tick
        let ticked = self.peek() == Some('\'');
        if ticked {
            self.bump();
        }
        if !self
            .peek()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        {
            return Err(Error::at(
                span,
                "a tick literal needs an identifier name: `'name` is a symbol (or `''T` a type name) and `'c'` a character",
            ));
        }
        let name = self.scan_ident();
        Ok(if ticked {
            Tok::TypeNameLit(name)
        } else {
            Tok::SymLit(name)
        })
    }

    /// The identifier spelling at the cursor (`lex_word`'s shape:
    /// ident characters with dash glue), without the keyword lookup.
    fn scan_ident(&mut self) -> String {
        let mut w = String::new();
        loop {
            match self.peek() {
                Some(c) if is_ident_char(c) => {
                    w.push(c);
                    self.bump();
                }
                Some('-')
                    if self
                        .peek_at(1)
                        .map(is_dash_glue_char)
                        .unwrap_or(false) =>
                {
                    w.push('-');
                    self.bump();
                }
                _ => break,
            }
        }
        w
    }

    fn lex_escape(&mut self, span: Span) -> Result<char> {
        let Some(c) = self.bump() else {
            return Err(Error::at(span, "unterminated escape sequence"));
        };
        Ok(match c {
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'u' => {
                if self.bump() != Some('{') {
                    return Err(Error::at(span, "expected `{` after \\u escape"));
                }
                let mut hex = String::new();
                loop {
                    match self.bump() {
                        Some('}') => break,
                        Some(h) if h.is_ascii_hexdigit() => hex.push(h),
                        _ => return Err(Error::at(span, "malformed \\u escape")),
                    }
                }
                let v = u32::from_str_radix(&hex, 16)
                    .map_err(|_| Error::at(span, "malformed \\u escape"))?;
                match char::from_u32(v) {
                    Some(c) => c,
                    None => return Err(Error::at(span, "\\u escape is not a Unicode scalar value")),
                }
            }
            _ => return Err(Error::at(span, "unknown escape sequence")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<Tok> {
        lex(src).unwrap().into_iter().map(|t| t.tok).collect()
    }

    #[test]
    fn identifiers_and_keywords() {
        assert_eq!(
            kinds("module Foo where"),
            vec![Tok::Module, Tok::ConId("Foo".into()), Tok::Where, Tok::Eof]
        );
    }

    #[test]
    fn dashes_glue_identifiers_on_both_sides() {
        // A `-` tight on both sides between identifier characters is
        // glue; digits do not continue an identifier across a dash, so
        // `x-1` and `1-2` are subtraction (section 8).
        assert_eq!(
            kinds("int-digits-into x-y x-1 1-2 x_1-y X-y"),
            vec![
                Tok::VarId("int-digits-into".into()),
                Tok::VarId("x-y".into()),
                Tok::VarId("x".into()),
                Tok::Op("-".into()),
                Tok::Int("1".into()),
                Tok::Int("1".into()),
                Tok::Op("-".into()),
                Tok::Int("2".into()),
                Tok::VarId("x_1-y".into()),
                Tok::ConId("X-y".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn minus_signs_versus_operators() {
        // A `-` tight only to following digits signs a literal; anything
        // else is the binary minus (`f -42` applies, `f - 42` subtracts).
        assert_eq!(
            kinds("f -42 x = -3 (-42) x - 1 x-1 f ()-1"),
            vec![
                Tok::VarId("f".into()),
                Tok::Int("-42".into()),
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Int("-3".into()),
                Tok::LParen,
                Tok::Int("-42".into()),
                Tok::RParen,
                Tok::VarId("x".into()),
                Tok::Op("-".into()),
                Tok::Int("1".into()),
                Tok::VarId("x".into()),
                Tok::Op("-".into()),
                Tok::Int("1".into()),
                Tok::VarId("f".into()),
                Tok::LParen,
                Tok::RParen,
                Tok::Op("-".into()),
                Tok::Int("1".into()),
                Tok::Eof
            ]
        );
        // A signed literal never joins a symbol run.
        assert_eq!(
            kinds("=-3"),
            vec![Tok::Equals, Tok::Int("-3".into()), Tok::Eof]
        );
    }

    #[test]
    fn reserved_punctuation_longest_first() {
        assert_eq!(
            kinds("a :: b -> c <- d => e"),
            vec![
                Tok::VarId("a".into()),
                Tok::ColonColon,
                Tok::VarId("b".into()),
                Tok::Arrow,
                Tok::VarId("c".into()),
                Tok::LArrow,
                Tok::VarId("d".into()),
                Tok::Darrow,
                Tok::VarId("e".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn operators_versus_reserved() {
        // `==` is an operator; `=` is reserved. `|]` closes a quote.
        assert_eq!(
            kinds("x == y |] | || ! !="),
            vec![
                Tok::VarId("x".into()),
                Tok::Op("==".into()),
                Tok::VarId("y".into()),
                Tok::RQuote,
                Tok::Pipe,
                Tok::Op("||".into()),
                Tok::Bang,
                Tok::Op("!=".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn bare_tilde_is_reserved_but_runs_are_operators() {
        // `~` is reserved punctuation for memo bindings (spec: lazy
        // computation and reclamation); symbol runs keep their operator
        // identity, like `=` versus `==`.
        assert_eq!(
            kinds("~ ~x ~> <~> ~~~"),
            vec![
                Tok::Tilde,
                Tok::Tilde,
                Tok::VarId("x".into()),
                Tok::Op("~>".into()),
                Tok::Op("<~>".into()),
                Tok::Op("~~~".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn comments_win_over_symbol_identifiers() {
        assert_eq!(kinds("a -- comment\n{- nest {- in -} out -} b"), vec![
            Tok::VarId("a".into()),
            Tok::VarId("b".into()),
            Tok::Eof
        ]);
    }

    #[test]
    fn numbers() {
        assert_eq!(
            kinds("10 0x1F 1_000"),
            vec![
                Tok::Int("10".into()),
                Tok::Int("31".into()),
                Tok::Int("1000".into()),
                Tok::Eof
            ]
        );
        assert_eq!(
            kinds("0x00 007 18446744073709551616"),
            vec![
                Tok::Int("0".into()),
                Tok::Int("7".into()),
                Tok::Int("18446744073709551616".into()),
                Tok::Eof
            ]
        );
        assert!(matches!(kinds("1.5").as_slice(), [Tok::FloatLit(_), Tok::Eof]));
    }

    #[test]
    fn strings_and_chars() {
        assert_eq!(kinds(r#""a\n\u{41}""#), vec![Tok::StrLit("a\nA".into()), Tok::Eof]);
        assert_eq!(kinds("'x'"), vec![Tok::CharLit("x".into()), Tok::Eof]);
    }

    #[test]
    fn multiplicities_are_reserved() {
        assert_eq!(
            kinds("%0 %1 %aff %many %affix"),
            vec![
                Tok::MultZero,
                Tok::MultOne,
                Tok::MultAff,
                Tok::MultMany,
                Tok::Op("%".into()),
                Tok::VarId("affix".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn tabs_in_indentation_rejected() {
        assert!(lex("module M where\n\tfoo = 1").is_err());
        assert!(lex("x\t= 1").is_ok());
    }

    #[test]
    fn splice_and_quote() {
        // `$(` lexes as `Dollar` + `LParen` sharing one span (the parser
        // reconstitutes the splice from an adjacent pair); `$ (` stays
        // ordinary reserved application.
        let toks = crate::lexer::lex("[| $ ( ) $( x").unwrap();
        let kinds: Vec<Tok> = toks.iter().map(|t| t.tok.clone()).collect();
        assert_eq!(kinds, vec![
            Tok::LQuote,
            Tok::Dollar,
            Tok::LParen,
            Tok::RParen,
            Tok::Dollar,
            Tok::LParen,
            Tok::VarId("x".into()),
            Tok::Eof
        ]);
        // The `$(` pair shares a span; the spaced `$ (` pair does not.
        assert_eq!(toks[4].span, toks[5].span);
        assert_ne!(toks[1].span, toks[2].span);
    }
}
