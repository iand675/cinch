//! Type-signature pretty-printing and Hoogle-style search for `cinch doc`.
//!
//! This module parses Cinch type signatures (both canonical `.cinchif` schemes
//! with full parenthesization/qualifiers and concise user-authored signatures),
//! pretty-prints them with minimal parentheses and simplified kind binders, and
//! ranks documentation entries against Hoogle-style name, type, module, and
//! kind queries.

use std::collections::{HashMap, HashSet};

/// A binder inside a `forall` quantifier or type header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binder {
    pub name: String,
    pub kind: Option<Box<TypeExpr>>,
}

/// A constraint inside `using (...) =>` or Haskell-style `C a =>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub binder: Option<String>,
    pub ty: TypeExpr,
}

/// Parsed Cinch type expression for pretty-printing and Hoogle-style matching.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeExpr {
    Forall {
        binders: Vec<Binder>,
        body: Box<TypeExpr>,
    },
    Qual {
        constraints: Vec<Constraint>,
        body: Box<TypeExpr>,
    },
    Arrow {
        param_name: Option<String>,
        mult: Option<String>,
        effects: Option<String>,
        from: Box<TypeExpr>,
        to: Box<TypeExpr>,
    },
    Sigma {
        name: String,
        ty: Box<TypeExpr>,
        body: Box<TypeExpr>,
    },
    Refine {
        binder: String,
        carrier: Box<TypeExpr>,
        predicate: String,
    },
    Tuple(Vec<TypeExpr>),
    App(Vec<TypeExpr>),
    Con(String),
    Var(String),
    Raw(String),
}

/// An entry in a searchable Cinch documentation index.
#[derive(Clone, Debug)]
pub struct SearchEntry {
    pub id: String,
    pub module: String,
    pub kind: String,
    pub name: String,
    pub signature: String,
    pub pretty_signature: String,
    pub summary: String,
    pub source: String,
    pub line: usize,
}

/// A ranked search hit returned by [`search_entries`].
#[derive(Clone, Debug)]
pub struct SearchHit<'a> {
    pub entry: &'a SearchEntry,
    pub score: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    LParen,
    RParen,
    Comma,
    Dot,
    ColonColon,
    FatArrow,
    Arrow(Option<String>), // optional effect row contents inside `{...}`
    Mult(String),          // %0, %1, %aff, %many
    KwForall,
    KwUsing,
    KwSigma,
    KwRefine,
    KwWith,
    Ident(String),
    Other(String),
}

fn tokenize(input: &str) -> Vec<Tok> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut out = Vec::new();

    while i < len {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        match b {
            b'(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            b')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            b',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            b'.' => {
                out.push(Tok::Dot);
                i += 1;
            }
            b':' if i + 1 < len && bytes[i + 1] == b':' => {
                out.push(Tok::ColonColon);
                i += 2;
            }
            b'=' if i + 1 < len && bytes[i + 1] == b'>' => {
                out.push(Tok::FatArrow);
                i += 2;
            }
            b'-' if i + 1 < len && bytes[i + 1] == b'>' => {
                i += 2;
                if i < len && bytes[i] == b'{' {
                    let start = i + 1;
                    let mut depth = 1usize;
                    i += 1;
                    while i < len && depth > 0 {
                        if bytes[i] == b'{' {
                            depth += 1;
                        } else if bytes[i] == b'}' {
                            depth -= 1;
                        }
                        i += 1;
                    }
                    let end = if depth == 0 { i - 1 } else { i };
                    out.push(Tok::Arrow(Some(input[start..end].trim().to_string())));
                } else {
                    out.push(Tok::Arrow(None));
                }
            }
            b'%' => {
                let start = i;
                i += 1;
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                out.push(Tok::Mult(input[start..i].to_string()));
            }
            b'{' => {
                let start = i;
                let mut depth = 1usize;
                i += 1;
                while i < len && depth > 0 {
                    if bytes[i] == b'{' {
                        depth += 1;
                    } else if bytes[i] == b'}' {
                        depth -= 1;
                    }
                    i += 1;
                }
                out.push(Tok::Other(input[start..i].to_string()));
            }
            b'[' => {
                let start = i;
                let mut depth = 1usize;
                i += 1;
                while i < len && depth > 0 {
                    if bytes[i] == b'[' {
                        depth += 1;
                    } else if bytes[i] == b']' {
                        depth -= 1;
                    }
                    i += 1;
                }
                out.push(Tok::Other(input[start..i].to_string()));
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                i += 1;
                while i < len {
                    let ch = bytes[i];
                    if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'\'' {
                        i += 1;
                    } else if ch == b'-'
                        && i + 1 < len
                        && (bytes[i + 1].is_ascii_alphanumeric() || bytes[i + 1] == b'_')
                    {
                        // Kebab-case identifier (not `->`)
                        i += 1;
                    } else if ch == b'.'
                        && bytes[start].is_ascii_uppercase()
                        && i + 1 < len
                        && (bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == b'_')
                    {
                        // Qualified path like Data.Core.Result
                        i += 1;
                    } else {
                        break;
                    }
                }
                let word = &input[start..i];
                match word {
                    "forall" => out.push(Tok::KwForall),
                    "using" => out.push(Tok::KwUsing),
                    "Sigma" => out.push(Tok::KwSigma),
                    "refine" => out.push(Tok::KwRefine),
                    "with" => out.push(Tok::KwWith),
                    _ => out.push(Tok::Ident(word.to_string())),
                }
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                out.push(Tok::Other(input[start..i].to_string()));
            }
            _ => {
                // Operator chunk
                let start = i;
                i += 1;
                while i < len
                    && !bytes[i].is_ascii_whitespace()
                    && !matches!(
                        bytes[i],
                        b'(' | b')' | b',' | b'.' | b'{' | b'}' | b'[' | b']'
                    )
                    && !bytes[i].is_ascii_alphanumeric()
                    && bytes[i] != b'_'
                {
                    if bytes[i] == b'-' && i + 1 < len && bytes[i + 1] == b'>' {
                        break;
                    }
                    if bytes[i] == b'=' && i + 1 < len && bytes[i + 1] == b'>' {
                        break;
                    }
                    if bytes[i] == b':' && i + 1 < len && bytes[i + 1] == b':' {
                        break;
                    }
                    i += 1;
                }
                out.push(Tok::Other(input[start..i].to_string()));
            }
        }
    }
    out
}

struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Tok>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn peek_n(&self, n: usize) -> Option<&Tok> {
        self.tokens.get(self.pos + n)
    }

    fn next(&mut self) -> Option<Tok> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn parse_type(&mut self) -> Option<TypeExpr> {
        match self.peek() {
            Some(Tok::KwForall) => self.parse_forall(),
            Some(Tok::KwUsing) => self.parse_using(),
            Some(Tok::KwSigma) => self.parse_sigma(),
            Some(Tok::KwRefine) => self.parse_refine(),
            _ => self.parse_qual_or_arrow(),
        }
    }

    fn parse_forall(&mut self) -> Option<TypeExpr> {
        self.next(); // consume forall
        let mut binders = Vec::new();
        while let Some(tok) = self.peek() {
            match tok {
                Tok::Dot => {
                    self.next();
                    break;
                }
                Tok::LParen => {
                    let save = self.pos;
                    self.next();
                    if let Some(Tok::Ident(name)) = self.next() {
                        if matches!(self.peek(), Some(Tok::ColonColon)) {
                            self.next();
                            let kind = self.parse_type()?;
                            if matches!(self.peek(), Some(Tok::RParen)) {
                                self.next();
                                binders.push(Binder {
                                    name,
                                    kind: Some(Box::new(kind)),
                                });
                                continue;
                            }
                        }
                    }
                    self.pos = save;
                    break;
                }
                Tok::Ident(name) => {
                    let name = name.clone();
                    self.next();
                    binders.push(Binder { name, kind: None });
                }
                _ => break,
            }
        }
        let body = self.parse_type()?;
        if binders.is_empty() {
            Some(body)
        } else {
            Some(TypeExpr::Forall {
                binders,
                body: Box::new(body),
            })
        }
    }

    fn parse_using(&mut self) -> Option<TypeExpr> {
        self.next(); // consume using
        let mut constraints = Vec::new();
        if matches!(self.peek(), Some(Tok::LParen)) {
            self.next();
            loop {
                if matches!(self.peek(), Some(Tok::RParen)) {
                    self.next();
                    break;
                }
                let save = self.pos;
                let binder = if let (Some(Tok::Ident(name)), Some(Tok::ColonColon)) =
                    (self.peek(), self.peek_n(1))
                {
                    let name = name.clone();
                    self.next();
                    self.next();
                    Some(name)
                } else {
                    None
                };
                if let Some(ty) = self.parse_type() {
                    constraints.push(Constraint { binder, ty });
                } else {
                    self.pos = save;
                    break;
                }
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.next();
                } else if matches!(self.peek(), Some(Tok::RParen)) {
                    self.next();
                    break;
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek(), Some(Tok::FatArrow)) {
            self.next();
        }
        let body = self.parse_type()?;
        if constraints.is_empty() {
            Some(body)
        } else {
            Some(TypeExpr::Qual {
                constraints,
                body: Box::new(body),
            })
        }
    }

    fn parse_sigma(&mut self) -> Option<TypeExpr> {
        self.next(); // consume Sigma
        let mut name = "_".to_string();
        let mut ty = TypeExpr::Con("Type".to_string());
        if matches!(self.peek(), Some(Tok::LParen)) {
            self.next();
            if let Some(Tok::Ident(n)) = self.next() {
                name = n;
            }
            if matches!(self.peek(), Some(Tok::ColonColon)) {
                self.next();
                ty = self.parse_type()?;
            }
            if matches!(self.peek(), Some(Tok::RParen)) {
                self.next();
            }
        }
        if matches!(self.peek(), Some(Tok::Dot)) {
            self.next();
        }
        let body = self.parse_type()?;
        Some(TypeExpr::Sigma {
            name,
            ty: Box::new(ty),
            body: Box::new(body),
        })
    }

    fn parse_refine(&mut self) -> Option<TypeExpr> {
        self.next(); // consume refine
        let mut binder = "x".to_string();
        let mut carrier = TypeExpr::Var("a".to_string());
        if matches!(self.peek(), Some(Tok::LParen)) {
            self.next();
            if let Some(Tok::Ident(n)) = self.next() {
                binder = n;
            }
            if matches!(self.peek(), Some(Tok::ColonColon)) {
                self.next();
                carrier = self.parse_type()?;
            }
            if matches!(self.peek(), Some(Tok::RParen)) {
                self.next();
            }
        }
        if matches!(self.peek(), Some(Tok::KwWith)) {
            self.next();
        }
        let mut pred_parts = Vec::new();
        let mut depth = 0usize;
        while let Some(tok) = self.peek() {
            if depth == 0 && matches!(tok, Tok::RParen | Tok::Comma) {
                break;
            }
            match self.next()? {
                Tok::LParen => {
                    depth += 1;
                    pred_parts.push("(".into());
                }
                Tok::RParen => {
                    depth = depth.saturating_sub(1);
                    pred_parts.push(")".into());
                }
                Tok::Ident(s) | Tok::Other(s) | Tok::Mult(s) => pred_parts.push(s),
                Tok::Comma => pred_parts.push(",".into()),
                Tok::Dot => pred_parts.push(".".into()),
                Tok::ColonColon => pred_parts.push("::".into()),
                Tok::FatArrow => pred_parts.push("=>".into()),
                Tok::Arrow(None) => pred_parts.push("->".into()),
                Tok::Arrow(Some(eff)) => pred_parts.push(format!("->{{{eff}}}")),
                Tok::KwForall => pred_parts.push("forall".into()),
                Tok::KwUsing => pred_parts.push("using".into()),
                Tok::KwSigma => pred_parts.push("Sigma".into()),
                Tok::KwRefine => pred_parts.push("refine".into()),
                Tok::KwWith => pred_parts.push("with".into()),
            }
        }
        let predicate = join_raw_tokens(&pred_parts);
        Some(TypeExpr::Refine {
            binder,
            carrier: Box::new(carrier),
            predicate,
        })
    }

    fn parse_qual_or_arrow(&mut self) -> Option<TypeExpr> {
        // Check for dependent arrow `(x :: T) [%m] ->[{eff}] Body`
        if matches!(self.peek(), Some(Tok::LParen)) {
            let save = self.pos;
            if let Some((param_name, from)) = self.try_parse_named_param() {
                let mult = if let Some(Tok::Mult(m)) = self.peek() {
                    let m = m.clone();
                    self.next();
                    Some(m)
                } else {
                    None
                };
                if let Some(Tok::Arrow(effects)) = self.peek().cloned() {
                    self.next();
                    if let Some(to) = self.parse_type() {
                        return Some(TypeExpr::Arrow {
                            param_name: Some(param_name),
                            mult,
                            effects,
                            from: Box::new(from),
                            to: Box::new(to),
                        });
                    }
                }
            }
            self.pos = save;
        }

        let lhs = self.parse_app()?;

        // Haskell/Hoogle-style `Eq a => ...` or `(Eq a, Ord a) => ...`
        if matches!(self.peek(), Some(Tok::FatArrow)) {
            self.next();
            let body = self.parse_type()?;
            let constraints = match lhs {
                TypeExpr::Tuple(items) if !items.is_empty() => items
                    .into_iter()
                    .map(|ty| Constraint { binder: None, ty })
                    .collect(),
                other => vec![Constraint {
                    binder: None,
                    ty: other,
                }],
            };
            return Some(TypeExpr::Qual {
                constraints,
                body: Box::new(body),
            });
        }

        let mult = if let Some(Tok::Mult(m)) = self.peek() {
            let m = m.clone();
            self.next();
            Some(m)
        } else {
            None
        };

        if let Some(Tok::Arrow(effects)) = self.peek().cloned() {
            self.next();
            let rhs = self.parse_type()?;
            return Some(TypeExpr::Arrow {
                param_name: None,
                mult,
                effects,
                from: Box::new(lhs),
                to: Box::new(rhs),
            });
        }

        Some(lhs)
    }

    fn try_parse_named_param(&mut self) -> Option<(String, TypeExpr)> {
        if !matches!(self.next(), Some(Tok::LParen)) {
            return None;
        }
        let name = match self.next() {
            Some(Tok::Ident(n)) if !n.chars().next().is_some_and(|c| c.is_ascii_uppercase()) => n,
            _ => return None,
        };
        if !matches!(self.next(), Some(Tok::ColonColon)) {
            return None;
        }
        let ty = self.parse_type()?;
        if !matches!(self.next(), Some(Tok::RParen)) {
            return None;
        }
        Some((name, ty))
    }

    fn parse_app(&mut self) -> Option<TypeExpr> {
        let mut items = Vec::new();
        while let Some(atom) = self.parse_atom() {
            items.push(atom);
        }
        match items.len() {
            0 => None,
            1 => items.pop(),
            _ => Some(TypeExpr::App(items)),
        }
    }

    fn parse_atom(&mut self) -> Option<TypeExpr> {
        match self.peek()? {
            Tok::LParen => {
                self.next();
                if matches!(self.peek(), Some(Tok::RParen)) {
                    self.next();
                    return Some(TypeExpr::Con("Unit".to_string()));
                }
                let first = self.parse_type()?;
                if matches!(self.peek(), Some(Tok::Comma)) {
                    let mut elems = vec![first];
                    while matches!(self.peek(), Some(Tok::Comma)) {
                        self.next();
                        elems.push(self.parse_type()?);
                    }
                    if matches!(self.peek(), Some(Tok::RParen)) {
                        self.next();
                    }
                    Some(TypeExpr::Tuple(elems))
                } else {
                    if matches!(self.peek(), Some(Tok::RParen)) {
                        self.next();
                    }
                    Some(first)
                }
            }
            Tok::Ident(name) => {
                let name = name.clone();
                self.next();
                if name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                    Some(TypeExpr::Con(name))
                } else {
                    Some(TypeExpr::Var(name))
                }
            }
            Tok::Other(raw) => {
                let raw = raw.clone();
                self.next();
                Some(TypeExpr::Raw(raw))
            }
            _ => None,
        }
    }
}

fn join_raw_tokens(parts: &[String]) -> String {
    let mut out = String::new();
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            let prev = &parts[idx - 1];
            let no_space = prev == "(" || part == ")" || part == "," || part == ".";
            if !no_space {
                out.push(' ');
            }
        }
        out.push_str(part);
    }
    out
}

/// Parse a Cinch type scheme string into a [`TypeExpr`].
pub fn parse_type_expr(input: &str) -> Option<TypeExpr> {
    let tokens = tokenize(input.trim());
    if tokens.is_empty() {
        return None;
    }
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_type()?;
    if parser.pos < parser.tokens.len() {
        return None;
    }
    Some(expr)
}

/// Strip module qualifiers from a nominal type/class name (`Data.Core.List` -> `List`).
pub fn short_name(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Prec {
    Top,
    ArrowLhs,
    AppArg,
}

fn needs_parens(expr: &TypeExpr, ctx: Prec) -> bool {
    match ctx {
        Prec::Top => false,
        Prec::ArrowLhs => matches!(
            expr,
            TypeExpr::Forall { .. }
                | TypeExpr::Qual { .. }
                | TypeExpr::Arrow { .. }
                | TypeExpr::Sigma { .. }
                | TypeExpr::Refine { .. }
        ),
        Prec::AppArg => matches!(
            expr,
            TypeExpr::Forall { .. }
                | TypeExpr::Qual { .. }
                | TypeExpr::Arrow { .. }
                | TypeExpr::Sigma { .. }
                | TypeExpr::Refine { .. }
                | TypeExpr::App(_)
        ),
    }
}

fn is_plain_type_kind(kind: &TypeExpr) -> bool {
    matches!(kind, TypeExpr::Con(name) if name == "Type")
}

/// Pretty-print a parsed [`TypeExpr`] with minimal parentheses.
/// When `shorten_qualifiers` is `true`, qualified constructors like `Data.Core.List`
/// render as `List`.
pub fn format_type(expr: &TypeExpr, shorten_qualifiers: bool) -> String {
    format_type_prec(expr, Prec::Top, shorten_qualifiers)
}

fn format_type_prec(expr: &TypeExpr, ctx: Prec, shorten: bool) -> String {
    let inner = match expr {
        TypeExpr::Forall { binders, body } => {
            let mut parts = Vec::with_capacity(binders.len());
            for b in binders {
                match &b.kind {
                    Some(kind) if !is_plain_type_kind(kind) => {
                        parts.push(format!(
                            "({} :: {})",
                            b.name,
                            format_type_prec(kind, Prec::Top, shorten)
                        ));
                    }
                    _ => parts.push(b.name.clone()),
                }
            }
            format!(
                "forall {}. {}",
                parts.join(" "),
                format_type_prec(body, Prec::Top, shorten)
            )
        }
        TypeExpr::Qual { constraints, body } => {
            let rendered: Vec<String> = constraints
                .iter()
                .map(|c| {
                    let t = format_type_prec(&c.ty, Prec::Top, shorten);
                    match &c.binder {
                        Some(b) => format!("{b} :: {t}"),
                        None => t,
                    }
                })
                .collect();
            let has_named = constraints.iter().any(|c| c.binder.is_some());
            if has_named {
                format!(
                    "using ({}) => {}",
                    rendered.join(", "),
                    format_type_prec(body, Prec::Top, shorten)
                )
            } else if rendered.len() == 1 {
                format!(
                    "{} => {}",
                    rendered[0],
                    format_type_prec(body, Prec::Top, shorten)
                )
            } else {
                format!(
                    "({}) => {}",
                    rendered.join(", "),
                    format_type_prec(body, Prec::Top, shorten)
                )
            }
        }
        TypeExpr::Arrow {
            param_name,
            mult,
            effects,
            from,
            to,
        } => {
            let lhs = match param_name {
                Some(name) => format!("({name} :: {})", format_type_prec(from, Prec::Top, shorten)),
                None => format_type_prec(from, Prec::ArrowLhs, shorten),
            };
            let arrow = match (mult, effects) {
                (Some(m), Some(e)) => format!(" {m} ->{{{e}}} "),
                (Some(m), None) => format!(" {m} -> "),
                (None, Some(e)) => format!(" ->{{{e}}} "),
                (None, None) => " -> ".to_string(),
            };
            format!("{lhs}{arrow}{}", format_type_prec(to, Prec::Top, shorten))
        }
        TypeExpr::Sigma { name, ty, body } => format!(
            "Sigma ({name} :: {}). {}",
            format_type_prec(ty, Prec::Top, shorten),
            format_type_prec(body, Prec::Top, shorten)
        ),
        TypeExpr::Refine {
            binder,
            carrier,
            predicate,
        } => format!(
            "refine ({binder} :: {}) with {predicate}",
            format_type_prec(carrier, Prec::Top, shorten)
        ),
        TypeExpr::Tuple(elems) => {
            let items: Vec<String> = elems
                .iter()
                .map(|e| format_type_prec(e, Prec::Top, shorten))
                .collect();
            format!("({})", items.join(", "))
        }
        TypeExpr::App(elems) => {
            let mut out = Vec::with_capacity(elems.len());
            for (idx, e) in elems.iter().enumerate() {
                if idx == 0 {
                    let first_ctx = if matches!(e, TypeExpr::App(_)) {
                        Prec::Top
                    } else {
                        Prec::AppArg
                    };
                    out.push(format_type_prec(e, first_ctx, shorten));
                } else {
                    out.push(format_type_prec(e, Prec::AppArg, shorten));
                }
            }
            out.join(" ")
        }
        TypeExpr::Con(name) => {
            if shorten {
                short_name(name).to_string()
            } else {
                name.clone()
            }
        }
        TypeExpr::Var(name) => name.clone(),
        TypeExpr::Raw(raw) => raw.clone(),
    };
    if needs_parens(expr, ctx) {
        format!("({inner})")
    } else {
        inner
    }
}

/// Pretty-print a declaration line or bare type scheme.
///
/// Handles `name :: scheme`, `data/record/codata/signature` headers (dropping
/// redundant `(a :: Type)` annotations), `type ... = refine ...`, and bare schemes.
pub fn pretty_signature(raw: &str, shorten_qualifiers: bool) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Refinement / type alias declaration: `type Name ... = <rhs>`
    if let Some(rest) = trimmed.strip_prefix("type ") {
        if let Some((lhs, rhs)) = rest.split_once(" = ") {
            let pretty_lhs = simplify_header_params(lhs);
            let pretty_rhs = parse_type_expr(rhs)
                .map(|ty| format_type(&ty, shorten_qualifiers))
                .unwrap_or_else(|| rhs.trim().to_string());
            return format!("type {pretty_lhs} = {pretty_rhs}");
        }
    }

    // Container header: `data Name (a :: Type) ...`
    for kw in ["data ", "record ", "codata ", "signature ", "opaque data "] {
        if let Some(rest) = trimmed.strip_prefix(kw) {
            let has_where = rest.ends_with(" where");
            let header = rest.strip_suffix(" where").unwrap_or(rest).trim();
            let simplified = simplify_header_params(header);
            return if has_where {
                format!("{kw}{simplified} where")
            } else {
                format!("{kw}{simplified}")
            };
        }
    }

    // `name :: scheme`
    if let Some((lhs, rhs)) = split_decl_colon(trimmed) {
        if let Some(ty) = parse_type_expr(rhs) {
            return format!("{} :: {}", lhs.trim(), format_type(&ty, shorten_qualifiers));
        }
        return trimmed.to_string();
    }

    // Bare scheme
    if let Some(ty) = parse_type_expr(trimmed) {
        return format_type(&ty, shorten_qualifiers);
    }

    trimmed.to_string()
}

fn split_decl_colon(s: &str) -> Option<(&str, &str)> {
    // Only split on `::` at paren depth 0 when lhs looks like a declaration name
    // (not starting with `forall`, `using`, `Sigma`, `refine`, or containing `->`).
    let bytes = s.as_bytes();
    let mut depth = 0usize;
    let mut i = 0;
    while i + 1 < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b':' if depth == 0 && bytes[i + 1] == b':' => {
                let lhs = s[..i].trim();
                if lhs.is_empty()
                    || lhs.starts_with("forall ")
                    || lhs.starts_with("using ")
                    || lhs.contains("->")
                    || lhs.contains("=>")
                {
                    return None;
                }
                // Check lhs is a single token (identifier or `(op)`)
                if lhs.split_whitespace().count() == 1 {
                    return Some((lhs, &s[i + 2..]));
                }
                return None;
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn simplify_header_params(header: &str) -> String {
    // Turn `Result (e :: Type) (a :: Type)` into `Result e a`, preserving non-Type kinds.
    let tokens = tokenize(header);
    let mut out = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if matches!(tokens[i], Tok::LParen) {
            if let (
                Some(Tok::Ident(name)),
                Some(Tok::ColonColon),
                Some(Tok::Ident(kind)),
                Some(Tok::RParen),
            ) = (
                tokens.get(i + 1),
                tokens.get(i + 2),
                tokens.get(i + 3),
                tokens.get(i + 4),
            ) {
                if kind == "Type" {
                    out.push(name.clone());
                    i += 5;
                    continue;
                }
            }
            // Otherwise reconstruct parenthesized group
            let start = i;
            let mut depth = 1usize;
            i += 1;
            let mut group = vec!["(".to_string()];
            while i < tokens.len() && depth > 0 {
                match &tokens[i] {
                    Tok::LParen => {
                        depth += 1;
                        group.push("(".into());
                    }
                    Tok::RParen => {
                        depth -= 1;
                        group.push(")".into());
                    }
                    Tok::ColonColon => group.push("::".into()),
                    Tok::Arrow(None) => group.push("->".into()),
                    Tok::Arrow(Some(e)) => group.push(format!("->{{{e}}}")),
                    Tok::Ident(s) | Tok::Other(s) | Tok::Mult(s) => group.push(s.clone()),
                    Tok::Comma => group.push(",".into()),
                    Tok::Dot => group.push(".".into()),
                    Tok::FatArrow => group.push("=>".into()),
                    Tok::KwForall => group.push("forall".into()),
                    Tok::KwUsing => group.push("using".into()),
                    Tok::KwSigma => group.push("Sigma".into()),
                    Tok::KwRefine => group.push("refine".into()),
                    Tok::KwWith => group.push("with".into()),
                }
                i += 1;
            }
            let _ = start;
            out.push(join_raw_tokens(&group));
            continue;
        }
        match &tokens[i] {
            Tok::Ident(s) | Tok::Other(s) => out.push(s.clone()),
            _ => {}
        }
        i += 1;
    }
    out.join(" ")
}

/// Extract all constructor/type identifiers referenced inside a type expression,
/// in order of appearance (useful for cross-linking).
pub fn referenced_types(expr: &TypeExpr) -> Vec<String> {
    let mut out = Vec::new();
    collect_types(expr, &mut out);
    out
}

fn collect_types(expr: &TypeExpr, out: &mut Vec<String>) {
    match expr {
        TypeExpr::Forall { binders, body } => {
            for b in binders {
                if let Some(k) = &b.kind {
                    collect_types(k, out);
                }
            }
            collect_types(body, out);
        }
        TypeExpr::Qual { constraints, body } => {
            for c in constraints {
                collect_types(&c.ty, out);
            }
            collect_types(body, out);
        }
        TypeExpr::Arrow { from, to, .. } => {
            collect_types(from, out);
            collect_types(to, out);
        }
        TypeExpr::Sigma { ty, body, .. } => {
            collect_types(ty, out);
            collect_types(body, out);
        }
        TypeExpr::Refine { carrier, .. } => collect_types(carrier, out),
        TypeExpr::Tuple(elems) | TypeExpr::App(elems) => {
            for e in elems {
                collect_types(e, out);
            }
        }
        TypeExpr::Con(name) => out.push(name.clone()),
        TypeExpr::Var(_) | TypeExpr::Raw(_) => {}
    }
}

/// Parsed Hoogle-style query.
#[derive(Clone, Debug)]
pub struct ParsedQuery {
    pub include_modules: Vec<String>,
    pub exclude_modules: Vec<String>,
    pub kinds: Vec<String>,
    pub name_query: Option<String>,
    pub type_query: Option<TypeExpr>,
    pub raw_terms: Vec<String>,
}

fn normalize_kind(k: &str) -> String {
    match k.to_ascii_lowercase().as_str() {
        "fn" | "func" | "function" | "val" | "value" => "value".into(),
        "class" | "sig" | "signature" | "trait" => "signature".into(),
        "ctor" | "constructor" => "constructor".into(),
        "type" => "type".into(),
        other => other.to_string(),
    }
}

fn looks_like_type_query(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.contains("->")
        || trimmed.contains("=>")
        || trimmed.starts_with("forall ")
        || trimmed.starts_with("using ")
        || trimmed.starts_with("Sigma ")
        || trimmed.starts_with("refine ")
        || (trimmed.starts_with('(') && trimmed.contains(','))
    {
        return true;
    }
    // Multi-token query starting with an uppercase type constructor like `Result e a` or `List Int`
    let mut words = trimmed.split_whitespace();
    if let (Some(first), Some(_second)) = (words.next(), words.next()) {
        if first.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
            return true;
        }
    }
    false
}

/// Parse a Hoogle-style search query.
///
/// Supports:
/// - Module filters: `+Data.List`, `module:Data.List`, `-Crypto`
/// - Kind filters: `is:type`, `is:value`, `is:class`, `is:method`, `is:ctor`, `is:instance`
/// - Type queries: `(a -> b) -> List a -> List b`, `Result e a -> Option a`, `Eq a => a -> a -> Bool`
/// - Combined queries: `map :: (a -> b) -> List a -> List b`
/// - Name / text queries: `partition-results`, `base64 decode`
pub fn parse_query(raw: &str) -> ParsedQuery {
    let mut include_modules = Vec::new();
    let mut exclude_modules = Vec::new();
    let mut kinds = Vec::new();
    let mut remainder_tokens = Vec::new();

    for word in raw.split_whitespace() {
        if let Some(m) = word.strip_prefix('+') {
            if !m.is_empty() {
                include_modules.push(m.to_ascii_lowercase());
                continue;
            }
        }
        if let Some(m) = word.strip_prefix('-') {
            if !m.is_empty() && m.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
                exclude_modules.push(m.to_ascii_lowercase());
                continue;
            }
        }
        if let Some(m) = word.strip_prefix("module:") {
            if !m.is_empty() {
                include_modules.push(m.to_ascii_lowercase());
                continue;
            }
        }
        if let Some(k) = word.strip_prefix("is:").or_else(|| word.strip_prefix("kind:")) {
            if !k.is_empty() {
                kinds.push(normalize_kind(k));
                continue;
            }
        }
        remainder_tokens.push(word);
    }

    let remainder = remainder_tokens.join(" ");
    let mut name_query = None;
    let mut type_query = None;

    if let Some((lhs, rhs)) = remainder.split_once("::") {
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        if !lhs.is_empty() {
            name_query = Some(lhs.to_string());
        }
        if !rhs.is_empty() {
            type_query = parse_type_expr(rhs);
        }
    } else if looks_like_type_query(&remainder) {
        if let Some(parsed) = parse_type_expr(&remainder) {
            type_query = Some(parsed);
        } else if !remainder.is_empty() {
            name_query = Some(remainder.clone());
        }
    } else if !remainder.is_empty() {
        name_query = Some(remainder.clone());
    }

    let raw_terms = remainder
        .split_whitespace()
        .map(|w| w.to_ascii_lowercase())
        .collect();

    ParsedQuery {
        include_modules,
        exclude_modules,
        kinds,
        name_query,
        type_query,
        raw_terms,
    }
}

#[derive(Clone, Debug)]
struct FlattenedType {
    constraints: Vec<TypeExpr>,
    args: Vec<TypeExpr>,
    effects: Vec<String>,
    ret: TypeExpr,
}

fn flatten_type(expr: &TypeExpr) -> FlattenedType {
    let mut constraints = Vec::new();
    let mut args = Vec::new();
    let mut effects = Vec::new();
    let mut cur = expr;
    loop {
        match cur {
            TypeExpr::Forall { body, .. } => cur = body,
            TypeExpr::Qual {
                constraints: cs,
                body,
            } => {
                for c in cs {
                    constraints.push(c.ty.clone());
                }
                cur = body;
            }
            TypeExpr::Arrow {
                effects: eff,
                from,
                to,
                ..
            } => {
                args.push((**from).clone());
                if let Some(e) = eff {
                    for part in e.split(|c| c == ',' || c == '|') {
                        let p = part.trim();
                        if !p.is_empty() {
                            effects.push(p.to_string());
                        }
                    }
                }
                cur = to;
            }
            TypeExpr::Refine { carrier, .. } => cur = carrier,
            _ => break,
        }
    }
    FlattenedType {
        constraints,
        args,
        effects,
        ret: cur.clone(),
    }
}

#[derive(Default, Clone)]
struct MatchEnv {
    q_to_c: HashMap<String, TypeExpr>,
    c_to_q: HashMap<String, TypeExpr>,
}

/// Structural similarity cost between a query type `q` and candidate type `c`.
/// `None` means structurally incompatible. Lower cost is a closer match.
fn type_distance(q: &TypeExpr, c: &TypeExpr, env: &mut MatchEnv) -> Option<i32> {
    // Unwrap top-level forall/qual/refine inside nested arguments
    if let TypeExpr::Forall { body, .. } = q {
        return type_distance(body, c, env);
    }
    if let TypeExpr::Forall { body, .. } = c {
        return type_distance(q, body, env);
    }
    if let TypeExpr::Refine { carrier, .. } = c {
        return type_distance(q, carrier, env).map(|d| d + 2);
    }

    match (q, c) {
        (TypeExpr::Var(qv), TypeExpr::Var(cv)) => {
            if let Some(bound) = env.q_to_c.get(qv) {
                return if *bound == TypeExpr::Var(cv.clone()) {
                    Some(0)
                } else {
                    Some(18)
                };
            }
            if let Some(bound) = env.c_to_q.get(cv) {
                return if *bound == TypeExpr::Var(qv.clone()) {
                    Some(0)
                } else {
                    Some(18)
                };
            }
            env.q_to_c.insert(qv.clone(), TypeExpr::Var(cv.clone()));
            env.c_to_q.insert(cv.clone(), TypeExpr::Var(qv.clone()));
            Some(0)
        }
        (TypeExpr::Var(qv), concrete) => {
            // Query is more general than candidate (`a` matching `Int` or `List b`)
            let extra = match concrete {
                TypeExpr::Arrow { .. } => 6,
                TypeExpr::App(_) | TypeExpr::Tuple(_) => 4,
                _ => 0,
            };
            if let Some(bound) = env.q_to_c.get(qv) {
                if bound == concrete {
                    Some(4)
                } else {
                    None
                }
            } else {
                env.q_to_c.insert(qv.clone(), concrete.clone());
                Some(6 + extra)
            }
        }
        (concrete, TypeExpr::Var(cv)) => {
            // Candidate is more polymorphic than query (`a -> a` matching `Int -> Int`).
            // Swallowing a compound query type (`Result e a`, `a -> b`) into a bare variable
            // carries a stronger generalization penalty than swallowing a plain scalar type.
            let extra = match concrete {
                TypeExpr::Arrow { .. } => 22,
                TypeExpr::App(_) | TypeExpr::Tuple(_) => 16,
                _ => 0,
            };
            if let Some(bound) = env.c_to_q.get(cv) {
                if bound == concrete {
                    Some(8)
                } else {
                    None
                }
            } else {
                env.c_to_q.insert(cv.clone(), concrete.clone());
                Some(12 + extra)
            }
        }
        (TypeExpr::Con(qn), TypeExpr::Con(cn)) => {
            let qs = short_name(qn);
            let cs = short_name(cn);
            if qn == cn {
                Some(0)
            } else if qs == cs {
                Some(0)
            } else if qs.eq_ignore_ascii_case(cs) {
                Some(10)
            } else {
                None
            }
        }
        (TypeExpr::Tuple(qs), TypeExpr::Tuple(cs)) => {
            if qs.len() != cs.len() {
                return None;
            }
            let mut total = 0;
            for (qe, ce) in qs.iter().zip(cs.iter()) {
                total += type_distance(qe, ce, env)?;
            }
            Some(total)
        }
        (TypeExpr::Arrow { .. }, TypeExpr::Arrow { .. }) => {
            let fq = flatten_type(q);
            let fc = flatten_type(c);
            if fq.args.len() != fc.args.len() {
                return None;
            }
            let mut total = type_distance(&fq.ret, &fc.ret, env)?;
            for (qa, ca) in fq.args.iter().zip(fc.args.iter()) {
                total += type_distance(qa, ca, env)?;
            }
            Some(total)
        }
        (TypeExpr::App(qs), TypeExpr::App(cs)) => {
            if qs.len() != cs.len() {
                // Allow partial match if head constructor matches and query omits trailing params
                // e.g. `Result` matching `Result e a` is handled by Con vs App below.
                return None;
            }
            let mut total = 0;
            for (qe, ce) in qs.iter().zip(cs.iter()) {
                total += type_distance(qe, ce, env)?;
            }
            Some(total)
        }
        // Query names just a type constructor like `Result` and candidate has `Result e a`
        (TypeExpr::Con(qn), TypeExpr::App(cs)) => {
            if let Some(TypeExpr::Con(cn)) = cs.first() {
                if short_name(qn).eq_ignore_ascii_case(short_name(cn)) {
                    return Some(10 + (cs.len() as i32 - 1) * 3);
                }
            }
            None
        }
        _ => None,
    }
}

fn score_type_match(query: &TypeExpr, candidate: &TypeExpr) -> Option<i32> {
    let fq = flatten_type(query);
    let fc = flatten_type(candidate);

    // Case 1: query has no arrows (e.g. `Result e a`, `Option Int`, `List a`)
    if fq.args.is_empty() {
        let mut env = MatchEnv::default();
        // Direct return type match
        if let Some(dist) = type_distance(&fq.ret, &fc.ret, &mut env) {
            let extra_args_cost = (fc.args.len() as i32) * 14;
            return Some((960 - dist * 8 - extra_args_cost).max(50));
        }
        // Matches one of the candidate's arguments
        let mut best: Option<i32> = None;
        for arg in &fc.args {
            let mut arg_env = MatchEnv::default();
            if let Some(dist) = type_distance(&fq.ret, arg, &mut arg_env) {
                let s = (760 - dist * 8 - (fc.args.len() as i32) * 10).max(40);
                best = Some(best.map_or(s, |b| b.max(s)));
            }
        }
        return best;
    }

    // Case 2: arrow query (e.g. `(a -> b) -> List a -> List b`)
    // Allow up to 2 extra candidate arguments, and 0 missing arguments.
    if fc.args.len() < fq.args.len() || fc.args.len() > fq.args.len() + 2 {
        return None;
    }

    let mut base_env = MatchEnv::default();
    let ret_dist = type_distance(&fq.ret, &fc.ret, &mut base_env)?;

    // Try greedy best match (preferring in-order first)
    let mut best_arg_cost: Option<i32> = None;

    // Fast path: in-order prefix/subsequence matching
    if fq.args.len() == fc.args.len() {
        let mut env = base_env.clone();
        let mut ok = true;
        let mut cost = 0;
        for (qa, ca) in fq.args.iter().zip(fc.args.iter()) {
            if let Some(d) = type_distance(qa, ca, &mut env) {
                cost += d;
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            best_arg_cost = Some(cost);
        }
    }

    // Order-tolerant matching (handles swapped args like `List a -> (a -> b) -> List b`
    // and extra candidate args)
    if best_arg_cost.is_none() || best_arg_cost.unwrap_or(999) > 0 {
        let mut env = base_env.clone();
        let mut used = vec![false; fc.args.len()];
        let mut total = 0i32;
        let mut all_matched = true;
        let mut last_idx: Option<usize> = None;

        for qa in &fq.args {
            let mut pick: Option<(usize, i32, MatchEnv)> = None;
            for (ci, ca) in fc.args.iter().enumerate() {
                if used[ci] {
                    continue;
                }
                let mut trial = env.clone();
                if let Some(d) = type_distance(qa, ca, &mut trial) {
                    let order_penalty = match last_idx {
                        Some(prev) if ci < prev => 14,
                        _ => 0,
                    };
                    let total_d = d + order_penalty;
                    if pick.as_ref().is_none_or(|p| total_d < p.1) {
                        pick = Some((ci, total_d, trial));
                    }
                }
            }
            if let Some((ci, d, next_env)) = pick {
                used[ci] = true;
                total += d;
                last_idx = Some(ci);
                env = next_env;
            } else {
                all_matched = false;
                break;
            }
        }
        if all_matched {
            best_arg_cost = Some(best_arg_cost.map_or(total, |b| b.min(total)));
        }
    }

    let arg_cost = best_arg_cost?;
    let extra_args_penalty = (fc.args.len() - fq.args.len()) as i32 * 45;

    // Constraints bonus/penalty
    let mut constraint_delta = 0i32;
    if !fq.constraints.is_empty() {
        let mut matched_cs = 0;
        for qc in &fq.constraints {
            if fc.constraints.iter().any(|cc| {
                let mut env = MatchEnv::default();
                type_distance(qc, cc, &mut env).is_some()
            }) {
                matched_cs += 1;
            }
        }
        if matched_cs == fq.constraints.len() {
            constraint_delta += 25;
        } else {
            constraint_delta -= (fq.constraints.len() - matched_cs) as i32 * 35;
        }
    } else if !fc.constraints.is_empty() {
        // Tiny preference for unconstrained when query has no constraints
        constraint_delta -= (fc.constraints.len() as i32) * 3;
    }

    // Effect matching
    let mut effect_delta = 0i32;
    if !fq.effects.is_empty() {
        let c_set: HashSet<String> = fc
            .effects
            .iter()
            .map(|e| e.split_whitespace().next().unwrap_or(e).to_ascii_lowercase())
            .collect();
        for qe in &fq.effects {
            let head = qe.split_whitespace().next().unwrap_or(qe).to_ascii_lowercase();
            if c_set.contains(&head) {
                effect_delta += 20;
            } else {
                effect_delta -= 40;
            }
        }
    } else if !fc.effects.is_empty() {
        effect_delta -= 6;
    }

    let score = 1000 - ret_dist * 10 - arg_cost * 7 - extra_args_penalty + constraint_delta + effect_delta;
    if score >= 120 {
        Some(score)
    } else {
        None
    }
}

fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut chars = needle.chars();
    let mut cur = match chars.next() {
        Some(c) => c,
        None => return true,
    };
    for h in haystack.chars() {
        if h == cur {
            match chars.next() {
                Some(next) => cur = next,
                None => return true,
            }
        }
    }
    false
}

fn score_name_match(query: &str, entry: &SearchEntry) -> Option<i32> {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return None;
    }
    let name_lc = entry.name.to_ascii_lowercase();
    let id_lc = entry.id.to_ascii_lowercase();
    let module_lc = entry.module.to_ascii_lowercase();
    let summary_lc = entry.summary.to_ascii_lowercase();

    let mut score = if entry.name == query.trim() {
        1000
    } else if entry.id == query.trim() {
        980
    } else if name_lc == q {
        950
    } else if id_lc == q {
        930
    } else if name_lc.starts_with(&q) {
        860 - (name_lc.len().saturating_sub(q.len()) as i32).min(60)
    } else if name_lc
        .split(['-', '_', '.'])
        .any(|seg| seg.starts_with(&q))
    {
        800 - (name_lc.len().saturating_sub(q.len()) as i32).min(60)
    } else if name_lc.contains(&q) {
        730 - (name_lc.len().saturating_sub(q.len()) as i32).min(80)
    } else if id_lc.contains(&q) {
        660
    } else if q.len() >= 3 && is_subsequence(&q, &name_lc) {
        540 - (name_lc.len().saturating_sub(q.len()) as i32 * 3).min(120)
    } else {
        // Multi-word search across name + module + summary
        let words: Vec<&str> = q.split_whitespace().collect();
        if words.iter().all(|w| {
            name_lc.contains(w) || module_lc.contains(w) || summary_lc.contains(w)
        }) {
            let name_hits = words.iter().filter(|w| name_lc.contains(**w)).count() as i32;
            440 + name_hits * 60
        } else {
            return None;
        }
    };

    // Gentle tiebreakers: prefer top-level Prelude/Data/Text/Effect over internal modules
    if entry.module == "Prelude" {
        score += 12;
    } else if !entry.module.contains(".Internal") && entry.module != "Data.Core" {
        score += 6;
    }
    Some(score)
}

fn kind_matches(filter: &str, entry_kind: &str) -> bool {
    let k = entry_kind.to_ascii_lowercase();
    match filter {
        "type" => matches!(k.as_str(), "type" | "data" | "record" | "codata" | "refine"),
        other => k == other,
    }
}

/// Rank documentation entries against a Hoogle-style query.
pub fn search_entries<'a>(entries: &'a [SearchEntry], raw_query: &str) -> Vec<SearchHit<'a>> {
    let query = parse_query(raw_query);
    let mut hits = Vec::new();

    for entry in entries {
        let mod_lc = entry.module.to_ascii_lowercase();
        if !query.include_modules.is_empty()
            && !query
                .include_modules
                .iter()
                .any(|m| mod_lc == *m || mod_lc.starts_with(&format!("{m}.")) || mod_lc.contains(m))
        {
            continue;
        }
        if query
            .exclude_modules
            .iter()
            .any(|m| mod_lc == *m || mod_lc.starts_with(&format!("{m}.")))
        {
            continue;
        }
        if !query.kinds.is_empty()
            && !query.kinds.iter().any(|k| kind_matches(k, &entry.kind))
        {
            continue;
        }

        // Filter-only query (e.g. `+Data.List` or `is:class`)
        if query.name_query.is_none() && query.type_query.is_none() {
            hits.push(SearchHit { entry, score: 500 });
            continue;
        }

        let single_type_word = if query.type_query.is_none() {
            query.name_query.as_deref().and_then(|nq| {
                if !nq.contains(char::is_whitespace)
                    && nq.chars().next().is_some_and(|c| c.is_ascii_uppercase())
                {
                    parse_type_expr(nq)
                } else {
                    None
                }
            })
        } else {
            None
        };

        let name_score = query
            .name_query
            .as_deref()
            .and_then(|nq| score_name_match(nq, entry));

        let effective_type = query.type_query.as_ref().or(single_type_word.as_ref());
        let type_score = effective_type.and_then(|tq| {
            // Strip leading `name :: ` if present in signature
            let scheme = split_decl_colon(&entry.signature)
                .map(|(_, rhs)| rhs)
                .unwrap_or(&entry.signature);
            let parsed = parse_type_expr(scheme)?;
            score_type_match(tq, &parsed)
        });

        let final_score = match (&query.name_query, &query.type_query) {
            (Some(_), Some(_)) => match (name_score, type_score) {
                (Some(ns), Some(ts)) => Some((ns + ts) / 2 + 100),
                _ => None,
            },
            (Some(_), None) => match (name_score, type_score) {
                (Some(ns), Some(ts)) => Some(ns.max((ts * 4) / 5)),
                (Some(ns), None) => Some(ns),
                (None, Some(ts)) => Some((ts * 4) / 5),
                (None, None) => None,
            },
            (None, Some(_)) => type_score,
            (None, None) => Some(500),
        };

        if let Some(score) = final_score {
            hits.push(SearchHit { entry, score });
        }
    }

    hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.entry.module.cmp(&b.entry.module))
            .then_with(|| a.entry.name.cmp(&b.entry.name))
    });
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_prints_canonical_compiler_schemes() {
        let raw = "either :: forall (e :: Type) (a :: Type) (b :: Type). ((e -> b) -> ((a -> b) -> ((Data.Core.Result e a) -> b)))";
        assert_eq!(
            pretty_signature(raw, true),
            "either :: forall e a b. (e -> b) -> (a -> b) -> Result e a -> b"
        );
    }

    #[test]
    fn preserves_non_type_kinds_effects_and_multiplicities() {
        let raw = "timeout :: forall (r :: Region) (a :: Type). using (io :: Io r) => (Effect.Clock.Duration -> ((Unit ->{Cancel r, IO r} a) %1 ->{Async r, Cancel r, Diverge, IO r} (Effect.Clock.Timeout a)))";
        assert_eq!(
            pretty_signature(raw, true),
            "timeout :: forall (r :: Region) a. using (io :: Io r) => Duration -> (Unit ->{Cancel r, IO r} a) %1 ->{Async r, Cancel r, Diverge, IO r} Timeout a"
        );
    }

    #[test]
    fn pretty_prints_higher_kinded_and_refinements() {
        let trav = "traverse :: forall (t :: Type -> Type) (f :: Type -> Type) (a :: Type) (b :: Type). using (dict :: Category.Traversable.Traversable t, app :: Category.Applicative.Applicative f) => ((a -> (f b)) -> ((t a) -> (f (t b))))";
        assert_eq!(
            pretty_signature(trav, true),
            "traverse :: forall (t :: Type -> Type) (f :: Type -> Type) a b. using (dict :: Traversable t, app :: Applicative f) => (a -> f b) -> t a -> f (t b)"
        );

        let refine = "type NonEmpty (a :: Type) = refine (xs :: Data.Core.List a) with NotNull xs";
        assert_eq!(
            pretty_signature(refine, true),
            "type NonEmpty a = refine (xs :: List a) with NotNull xs"
        );
    }

    fn sample_entries() -> Vec<SearchEntry> {
        let specs = [
            (
                "Data.List.map",
                "Data.List",
                "value",
                "map",
                "forall (a :: Type) (b :: Type). ((a -> b) -> ((Data.Core.List a) -> (Data.Core.List b)))",
                "Apply a function to every element of a list.",
            ),
            (
                "Data.Result.result-to-option",
                "Data.Result",
                "value",
                "result-to-option",
                "forall (e :: Type) (a :: Type). ((Data.Core.Result e a) -> (Data.Core.Option a))",
                "Convert a Result into an Option, dropping the error.",
            ),
            (
                "Data.Result.option-to-result",
                "Data.Result",
                "value",
                "option-to-result",
                "forall (e :: Type) (a :: Type). (e -> ((Data.Core.Option a) -> (Data.Core.Result e a)))",
                "Convert an Option into a Result using a fallback error.",
            ),
            (
                "Function.Combinators.id",
                "Function.Combinators",
                "value",
                "id",
                "forall (a :: Type). (a -> a)",
                "Identity function.",
            ),
            (
                "Data.List.sort",
                "Data.List",
                "value",
                "sort",
                "forall (a :: Type). using (ord :: Relation.Ord.Ord a) => ((Data.Core.List a) -> (Data.Core.List a))",
                "Sort a list using its Ord instance.",
            ),
        ];
        specs
            .into_iter()
            .enumerate()
            .map(|(i, (id, module, kind, name, sig, summary))| SearchEntry {
                id: id.into(),
                module: module.into(),
                kind: kind.into(),
                name: name.into(),
                signature: sig.into(),
                pretty_signature: pretty_signature(sig, true),
                summary: summary.into(),
                source: format!("{module}.cinch"),
                line: i + 1,
            })
            .collect()
    }

    #[test]
    fn hoogle_type_search_matches_alpha_renamed_and_reordered_args() {
        let entries = sample_entries();

        let hits = search_entries(&entries, "(x -> y) -> List x -> List y");
        assert!(!hits.is_empty());
        assert_eq!(hits[0].entry.name, "map");

        let reordered = search_entries(&entries, "List a -> (a -> b) -> List b");
        assert!(!reordered.is_empty());
        assert_eq!(reordered[0].entry.name, "map");

        let res_opt = search_entries(&entries, "Result e a -> Option a");
        assert!(!res_opt.is_empty());
        assert_eq!(res_opt[0].entry.name, "result-to-option");
    }

    #[test]
    fn hoogle_search_supports_polymorphic_subsumption_and_module_filters() {
        let entries = sample_entries();

        let poly = search_entries(&entries, "Int -> Int");
        assert!(poly.iter().any(|h| h.entry.name == "id"));

        let constrained = search_entries(&entries, "Ord a => List a -> List a");
        assert_eq!(constrained[0].entry.name, "sort");

        let scoped = search_entries(&entries, "+Data.Result option");
        assert_eq!(scoped.len(), 2);
        assert!(scoped.iter().all(|h| h.entry.module == "Data.Result"));

        let excluded = search_entries(&entries, "-Data.Result option");
        assert!(excluded.is_empty());
    }
}
