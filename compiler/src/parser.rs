//! Recursive-descent parser for the draft 0.2 surface subset, with the
//! infix-chain fixity pass described in the grammar chapter.
//!
//! The input must be a layout-processed token stream (see [`crate::layout`]).
//! Parsing records infix chains as [`Term::Infix`]; a fixity pass over the
//! whole parsed module then resolves them, so fixity declaration order does
//! not matter within a module. `$` needs no declaration: it is reserved
//! right-associative application looser than every ordinary operator.

use crate::ast::*;
use crate::lexer::{Span, Tok, Token};
use crate::{Error, Result};
use std::collections::BTreeMap;

// ---------------------------------------------------------------- fixities

/// Fixity of an operator: associativity and precedence 0 through 9.
pub type Fixity = (Assoc, u8);

/// Operator fixities visible to one module: imported declarations plus the
/// module's own, which shadow the imported ones.
#[derive(Clone, Debug, Default)]
pub struct FixityTable {
    map: BTreeMap<String, Fixity>,
}

impl FixityTable {
    pub fn new() -> Self {
        FixityTable { map: BTreeMap::new() }
    }

    /// Fixity of `op` if it is declared.
    pub fn get(&self, op: &str) -> Option<Fixity> {
        self.map.get(op).copied()
    }

    /// Declare `op`'s fixity. Redeclaring the same fixity is accepted;
    /// a conflicting redeclaration is an error.
    pub fn insert(&mut self, op: &str, assoc: Assoc, prec: u8) -> Result<()> {
        if prec > 9 {
            return Err(Error::plain(format!(
                "fixity precedence {prec} is out of range; precedences run from 0 to 9"
            )));
        }
        match self.map.get(op) {
            Some(&(a, p)) if (a, p) == (assoc, prec) => Ok(()),
            Some(&(a, p)) => Err(Error::plain(format!(
                "conflicting fixity declarations for `{op}`: already {a:?} {p}, now {assoc:?} {prec}"
            ))),
            None => {
                self.map.insert(op.to_string(), (assoc, prec));
                Ok(())
            }
        }
    }

    /// Merge two tables; entries of `other` shadow entries of `self`.
    pub fn merge(&self, other: &FixityTable) -> FixityTable {
        let mut map = self.map.clone();
        for (k, v) in &other.map {
            map.insert(k.clone(), *v);
        }
        FixityTable { map }
    }
}

/// A parsed module together with the fixity table for parsing the next one.
#[derive(Debug)]
pub struct ParsedModule {
    pub module: Module,
    pub fixities: FixityTable,
}

/// Parse one module from a layout-processed token stream. `imported` holds the
/// fixities visible through imports; the module's own fixity declarations
/// shadow them and are returned in `ParsedModule.fixities`.
pub fn parse_module(tokens: &[Token], imported: &FixityTable) -> Result<ParsedModule> {
    let mut p = Parser::new(tokens);
    let mut module = p.parse_module_decl()?;
    p.expect_eof()?;
    let fixities = imported.merge(&p.own_fixities);
    resolve_module(&mut module, &fixities)?;
    Ok(ParsedModule { module, fixities })
}

// ----------------------------------------------------------------- helpers

fn describe(tok: &Tok) -> String {
    match tok {
        Tok::Eof => "end of input".into(),
        Tok::VarId(s) => format!("identifier `{s}`"),
        Tok::ConId(s) => format!("constructor `{s}`"),
        Tok::Op(s) => format!("operator `{s}`"),
        Tok::Int(v) => format!("integer `{v}`"),
        Tok::FloatLit(_) => "a float literal".into(),
        Tok::StrLit(_) => "a string literal".into(),
        Tok::CharLit(_) => "a character literal".into(),
        Tok::Tilde => "`~`".into(),
        other => format!("{other:?}"),
    }
}

fn mult_name(tok: &Tok) -> Option<&'static str> {
    match tok {
        Tok::MultZero => Some("%0"),
        Tok::MultOne => Some("%1"),
        Tok::MultAff => Some("%aff"),
        Tok::MultMany => Some("%many"),
        _ => None,
    }
}

fn modifier_name(tok: &Tok) -> Option<&'static str> {
    match tok {
        Tok::Proof => Some("proof"),
        Tok::Spec => Some("spec"),
        Tok::Stage => Some("stage"),
        Tok::Corec => Some("corec"),
        Tok::Macro => Some("macro"),
        _ => None,
    }
}

struct Parser<'a> {
    toks: &'a [Token],
    pos: usize,
    own_fixities: FixityTable,
}

impl<'a> Parser<'a> {
    fn new(toks: &'a [Token]) -> Self {
        Parser { toks, pos: 0, own_fixities: FixityTable::new() }
    }

    fn peek(&self) -> &Token {
        self.toks.get(self.pos).unwrap_or_else(|| self.toks.last().expect("empty token stream"))
    }

    fn peek_tok(&self) -> &Tok {
        &self.peek().tok
    }

    fn peek_tok_at(&self, n: usize) -> Option<&Tok> {
        self.toks.get(self.pos + n).map(|t| &t.tok)
    }

    fn at(&self, t: &Tok) -> bool {
        self.peek_tok() == t
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if self.pos < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn err(&self, message: impl Into<String>) -> Error {
        Error::at(self.peek().span, message)
    }

    fn expect(&mut self, t: &Tok, what: &str) -> Result<Token> {
        if self.at(t) {
            Ok(self.bump())
        } else {
            Err(self.err(format!("expected {what}, found {}", describe(self.peek_tok()))))
        }
    }

    fn expect_eof(&mut self) -> Result<()> {
        if self.at(&Tok::Eof) {
            Ok(())
        } else {
            Err(self.err(format!("expected end of input, found {}", describe(self.peek_tok()))))
        }
    }

    // ------------------------------------------------------------- modules

    fn parse_module_decl(&mut self) -> Result<Module> {
        let start = self.expect(&Tok::Module, "`module`")?.span;
        let path = self.parse_con_path()?;
        if self.at(&Tok::LParen) {
            return Err(self.err("module parameters are not supported in the bootstrap slice"));
        }
        if self.at(&Tok::ColonColon) {
            return Err(self.err("module signatures are not supported in the bootstrap slice"));
        }
        self.expect(&Tok::Where, "`where`")?;
        let decls = self.parse_block_items("the declaration block", |p| p.parse_declaration())?;
        let mut imports = Vec::new();
        for d in &decls {
            if let Decl::Import(i) = d {
                imports.push(i.clone());
            }
        }
        let name = Name::new(path.parts().join("."), start);
        Ok(Module { name, imports, decls, span: start })
    }

    fn parse_block_items<T>(
        &mut self,
        what: &str,
        mut item: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        self.expect(&Tok::LBrace, "`{`")?;
        let mut items = Vec::new();
        if self.at(&Tok::RBrace) {
            self.bump();
            return Ok(items);
        }
        loop {
            items.push(item(self)?);
            if self.at(&Tok::Semicolon) {
                self.bump();
                if self.at(&Tok::RBrace) {
                    self.bump();
                    break;
                }
            } else if self.at(&Tok::RBrace) {
                self.bump();
                break;
            } else {
                return Err(self.err(format!(
                    "expected `;` or `}}` in {what}, found {}",
                    describe(self.peek_tok())
                )));
            }
        }
        Ok(items)
    }

    // ------------------------------------------------------- declarations

    fn parse_declaration(&mut self) -> Result<Decl> {
        match self.peek_tok().clone() {
            Tok::Import => Ok(Decl::Import(self.parse_import()?)),
            Tok::Signature => Ok(Decl::Signature(self.parse_signature()?)),
            Tok::Instance | Tok::Implicit => Ok(Decl::Instance(self.parse_instance()?)),
            Tok::Data => Ok(Decl::Data(self.parse_data()?)),
            Tok::Foreign => Ok(Decl::Foreign(self.parse_foreign()?)),
            Tok::Infix | Tok::Infixl | Tok::Infixr => Ok(Decl::Fixity(self.parse_fixity()?)),
            Tok::Record => Ok(Decl::Record(self.parse_record()?)),
            Tok::Codata => Err(self.err(
                "codata declarations are not supported in the bootstrap slice; guarded codata is the codata slice's work",
            )),
            Tok::Effect => Err(self.err("effect declarations are not supported in the bootstrap slice")),
            Tok::TypeKw | Tok::Opaque => {
                Err(self.err("type declarations are not supported in the bootstrap slice"))
            }
            Tok::Rewrite | Tok::Target => {
                Err(self.err("rewrite declarations are not supported in the bootstrap slice"))
            }
            Tok::Upgrade => Err(self.err("upgrade declarations are not supported in the bootstrap slice")),
            Tok::Generate => Err(self.err("generate declarations are not supported in the bootstrap slice")),
            Tok::QuoteDecls => Err(self.err("quoted declarations are not supported in the bootstrap slice")),
            Tok::Module => {
                Err(self.err("nested module declarations are not supported in the bootstrap slice"))
            }
            Tok::VarId(_) | Tok::LParen => self.parse_sig_or_equation(),
            t if modifier_name(&t).is_some() => self.parse_sig_or_equation(),
            t => Err(self.err(format!("expected a declaration, found {}", describe(&t)))),
        }
    }

    fn parse_sig_or_equation(&mut self) -> Result<Decl> {
        let modifier = self.parse_modifier()?;
        let name = self.parse_value_name()?;
        if self.at(&Tok::ColonColon) {
            self.bump();
            let ty = self.parse_type()?;
            let span = name.span;
            Ok(Decl::ValueSig { name, modifier, ty, span })
        } else {
            let eq = self.parse_equation_rest(name, true)?;
            Ok(Decl::Equation { modifier, eq })
        }
    }

    fn parse_modifier(&mut self) -> Result<Option<Name>> {
        let span = self.peek().span;
        match modifier_name(self.peek_tok()) {
            Some(m) => {
                self.bump();
                Ok(Some(Name::new(m, span)))
            }
            None => Ok(None),
        }
    }

    /// `var_id` or `( operator )`.
    fn parse_value_name(&mut self) -> Result<Name> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => {
                self.bump();
                Ok(Name::new(s, span))
            }
            Tok::LParen => {
                self.bump();
                let op = match self.peek_tok().clone() {
                    Tok::Op(s) => s,
                    t => {
                        return Err(self.err(format!(
                            "expected an operator name in parentheses, found {}",
                            describe(&t)
                        )))
                    }
                };
                self.bump();
                self.expect(&Tok::RParen, "`)`")?;
                Ok(Name::new(op, span))
            }
            t => Err(self.err(format!("expected a name, found {}", describe(&t)))),
        }
    }

    /// `name pats = body [where locals]`.
    fn parse_equation_rest(&mut self, name: Name, allow_where: bool) -> Result<Equation> {
        let span = name.span;
        let mut pats = Vec::new();
        while self.starts_atomic_pattern() {
            pats.push(self.parse_atomic_pattern()?);
        }
        self.expect(&Tok::Equals, "`=`")?;
        let body = self.parse_term()?;
        let mut locals = Vec::new();
        if self.at(&Tok::Where) {
            if !allow_where {
                return Err(self.err(
                    "nested `where` blocks on local equations are not supported in the bootstrap slice",
                ));
            }
            self.bump();
            locals = self.parse_block_items("the local block", |p| p.parse_local_decl())?;
        }
        Ok(Equation { name, pats, body, locals, span })
    }

    fn parse_import(&mut self) -> Result<Import> {
        let span = self.expect(&Tok::Import, "`import`")?.span;
        // Import modifiers exist in the grammar but have no AST slot; they
        // are accepted here and the checker judges the import itself.
        if self.at(&Tok::Stage) {
            self.bump();
        } else if let Tok::VarId(s) = self.peek_tok() {
            if s == "replaceable" || s == "specialized" {
                self.bump();
            }
        }
        let path = self.parse_con_path()?;
        let mut alias = None;
        if self.at(&Tok::As) {
            self.bump();
            let a_span = self.peek().span;
            match self.peek_tok().clone() {
                Tok::ConId(s) => {
                    self.bump();
                    alias = Some(Name::new(s, a_span));
                }
                t => {
                    return Err(self.err(format!(
                        "expected a constructor alias after `as`, found {}",
                        describe(&t)
                    )))
                }
            }
        }
        Ok(Import { path, alias, span })
    }

    fn parse_signature(&mut self) -> Result<SigDecl> {
        let span = self.expect(&Tok::Signature, "`signature`")?.span;
        let name = self.parse_ctor_name("a signature name")?;
        let params = self.parse_params()?;
        self.expect(&Tok::Where, "`where`")?;
        let methods =
            self.parse_block_items("the signature block", |p| p.parse_method_signature())?;
        Ok(SigDecl { name, params, methods, span })
    }

    fn parse_method_signature(&mut self) -> Result<(Name, Type)> {
        if self.parse_modifier()?.is_some() {
            return Err(self.err(
                "modifiers on signature methods are not supported in the bootstrap slice",
            ));
        }
        let name = self.parse_value_name()?;
        self.expect(&Tok::ColonColon, "`::`")?;
        let ty = self.parse_type()?;
        Ok((name, ty))
    }

    fn parse_instance(&mut self) -> Result<InstDecl> {
        let start = self.peek().span;
        let implicit = self.at(&Tok::Implicit);
        if implicit {
            self.bump();
        }
        self.expect(&Tok::Instance, "`instance`")?;
        let name = self.parse_ctor_name("an instance name")?;
        let params = self.parse_params()?;
        self.expect(&Tok::ColonColon, "`::`")?;
        let spec = self.parse_type()?;
        let (ctx, target) = match spec {
            Type::Using(ctx, target, _) => (ctx, *target),
            other => (Vec::new(), other),
        };
        self.expect(&Tok::Where, "`where`")?;
        let methods = self.parse_block_items("the instance block", |p| {
            if p.parse_modifier()?.is_some() {
                return Err(p.err(
                    "modifiers on instance methods are not supported in the bootstrap slice",
                ));
            }
            let name = p.parse_value_name()?;
            p.parse_equation_rest(name, true)
        })?;
        Ok(InstDecl { implicit, name, params, ctx, target, methods, span: start })
    }

    fn parse_data(&mut self) -> Result<DataDecl> {
        let span = self.expect(&Tok::Data, "`data`")?.span;
        let name = self.parse_ctor_name("a data type name")?;
        let params = self.parse_params()?;
        if self.at(&Tok::ColonColon) {
            return Err(self.err(
                "kind signatures on `data` declarations are not supported in the bootstrap slice",
            ));
        }
        self.expect(&Tok::Where, "`where`")?;
        let ctors = self.parse_block_items("the constructor block", |p| {
            let name = p.parse_ctor_name("a constructor name")?;
            p.expect(&Tok::ColonColon, "`::`")?;
            let ty = p.parse_type()?;
            let span = name.span;
            Ok(CtorDecl { name, ty, span })
        })?;
        Ok(DataDecl { name, params, ctors, span })
    }

    /// `record R params where { [erased] field :: T, ... }`.
    fn parse_record(&mut self) -> Result<RecordDecl> {
        let span = self.expect(&Tok::Record, "`record`")?.span;
        let name = self.parse_ctor_name("a record name")?;
        let params = self.parse_params()?;
        if self.at(&Tok::ColonColon) {
            return Err(self.err(
                "kind signatures on `record` declarations are not supported in the bootstrap slice",
            ));
        }
        self.expect(&Tok::Where, "`where`")?;
        let fields = self.parse_block_items("the field block", |p| {
            if p.at(&Tok::Erased) {
                return Err(p.err(
                    "erasure is not implemented in the bootstrap slice (`erased` record fields are a later slice)",
                ));
            }
            let name = p.parse_value_name()?;
            p.expect(&Tok::ColonColon, "`::`")?;
            let ty = p.parse_type()?;
            let span = name.span;
            Ok(FieldDecl { name, ty, span })
        })?;
        Ok(RecordDecl { name, params, fields, span })
    }

    fn parse_foreign(&mut self) -> Result<ForeignDecl> {
        let span = self.expect(&Tok::Foreign, "`foreign`")?.span;
        let is_import = match self.peek_tok() {
            Tok::Import => {
                self.bump();
                true
            }
            Tok::Export => {
                self.bump();
                false
            }
            t => {
                return Err(self.err(format!(
                    "expected `import` or `export` after `foreign`, found {}",
                    describe(t)
                )))
            }
        };
        let abi = match self.peek_tok() {
            Tok::Ccall => {
                self.bump();
                "ccall".to_string()
            }
            Tok::Component => {
                self.bump();
                "component".to_string()
            }
            t => {
                return Err(self.err(format!(
                    "expected `ccall` or `component`, found {}",
                    describe(t)
                )))
            }
        };
        let symbol = match self.peek_tok().clone() {
            Tok::StrLit(s) => {
                self.bump();
                s
            }
            t => {
                return Err(self.err(format!(
                    "expected the foreign symbol string, found {}",
                    describe(&t)
                )))
            }
        };
        let name = self.parse_value_name()?;
        self.expect(&Tok::ColonColon, "`::`")?;
        let ty = self.parse_type()?;
        Ok(ForeignDecl { is_import, abi, symbol, name, ty, span })
    }

    fn parse_fixity(&mut self) -> Result<FixityDecl> {
        let span = self.peek().span;
        let assoc = match self.bump().tok {
            Tok::Infix => Assoc::None,
            Tok::Infixl => Assoc::Left,
            _ => Assoc::Right,
        };
        let prec = match self.peek_tok().clone() {
            Tok::Int(v) => {
                self.bump();
                match v.parse::<u8>() {
                    Ok(d) if d <= 9 => d,
                    _ => {
                        return Err(self.err(format!(
                            "fixity precedence {v} is out of range; precedences run from 0 to 9"
                        )))
                    }
                }
            }
            t => {
                return Err(self.err(format!(
                    "expected a fixity precedence between 0 and 9, found {}",
                    describe(&t)
                )))
            }
        };
        let mut ops = Vec::new();
        loop {
            let op_span = self.peek().span;
            match self.peek_tok().clone() {
                Tok::Op(s) => {
                    self.bump();
                    ops.push(Name::new(s, op_span));
                }
                t => {
                    return Err(self.err(format!(
                        "expected an operator in the fixity declaration, found {}",
                        describe(&t)
                    )))
                }
            }
            if self.at(&Tok::Comma) {
                self.bump();
            } else {
                break;
            }
        }
        for op in &ops {
            self.own_fixities
                .insert(&op.text, assoc, prec)
                .map_err(|e| Error::at(op.span, e.message))?;
        }
        Ok(FixityDecl { assoc, prec, ops, span })
    }

    /// Parameters of `data`, `signature` and `instance`. Kind and class
    /// annotations on binders have no home in these AST nodes and are
    /// dropped; only the names survive.
    fn parse_params(&mut self) -> Result<Vec<Name>> {
        let mut names = Vec::new();
        loop {
            match self.peek_tok() {
                Tok::VarId(s) if s != "_" => {
                    let span = self.peek().span;
                    let name = Name::new(s.clone(), span);
                    self.bump();
                    names.push(name);
                }
                Tok::LParen | Tok::LBrace => {
                    let b = self.parse_type_binder()?;
                    names.push(b.name);
                }
                _ => break,
            }
        }
        Ok(names)
    }

    fn parse_ctor_name(&mut self, what: &str) -> Result<Name> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::ConId(s) => {
                self.bump();
                Ok(Name::new(s, span))
            }
            t => Err(self.err(format!("expected {what}, found {}", describe(&t)))),
        }
    }

    // ------------------------------------------------------ local blocks

    fn parse_local_decl(&mut self) -> Result<LocalDecl> {
        if self.parse_modifier()?.is_some() {
            return Err(self.err(
                "modifiers on local declarations are not supported in the bootstrap slice",
            ));
        }
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => match self.peek_tok_at(1) {
                Some(Tok::ColonColon) => {
                    let name = self.parse_value_name()?;
                    self.bump();
                    let ty = self.parse_type()?;
                    Ok(LocalDecl::Sig { name, ty, span })
                }
                Some(Tok::At) => {
                    let pat = self.parse_pattern()?;
                    self.expect(&Tok::Equals, "`=`")?;
                    let body = self.parse_term()?;
                    Ok(LocalDecl::Bind { pat, body, span })
                }
                Some(Tok::Equals) => {
                    let name = self.parse_value_name()?;
                    self.parse_equation_rest(name, false).map(|eq| LocalDecl::Eq {
                        name: eq.name,
                        pats: eq.pats,
                        body: eq.body,
                        span,
                    })
                }
                _ => {
                    let name = self.parse_value_name()?;
                    self.parse_equation_rest(name, false).map(|eq| LocalDecl::Eq {
                        name: eq.name,
                        pats: eq.pats,
                        body: eq.body,
                        span,
                    })
                }
            },
            Tok::LParen => {
                if matches!(self.peek_tok_at(1), Some(Tok::Op(_)))
                    && matches!(self.peek_tok_at(2), Some(Tok::RParen))
                {
                    let name = self.parse_value_name()?;
                    if self.at(&Tok::ColonColon) {
                        self.bump();
                        let ty = self.parse_type()?;
                        Ok(LocalDecl::Sig { name, ty, span })
                    } else {
                        self.parse_equation_rest(name, false).map(|eq| LocalDecl::Eq {
                            name: eq.name,
                            pats: eq.pats,
                            body: eq.body,
                            span,
                        })
                    }
                } else {
                    let pat = self.parse_pattern()?;
                    self.expect(&Tok::Equals, "`=`")?;
                    let body = self.parse_term()?;
                    Ok(LocalDecl::Bind { pat, body, span })
                }
            }
            Tok::ConId(_) | Tok::Int(_) | Tok::StrLit(_) | Tok::CharLit(_) | Tok::Underscore => {
                let pat = self.parse_pattern()?;
                self.expect(&Tok::Equals, "`=`")?;
                let body = self.parse_term()?;
                Ok(LocalDecl::Bind { pat, body, span })
            }
            Tok::Tilde => {
                // `~ name = term`, a memo binding (laziness and reclamation
                // specification). Only irrefutable variable patterns.
                self.bump();
                let name_span = self.peek().span;
                let pat = match self.peek_tok().clone() {
                    Tok::VarId(s) if s != "_" => {
                        self.bump();
                        Pat::Var(Name::new(s, name_span), name_span)
                    }
                    Tok::Underscore => {
                        self.bump();
                        Pat::Wild(name_span)
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a memo binding name after `~`, found {}",
                            describe(&t)
                        )))
                    }
                };
                self.expect(&Tok::Equals, "`=` after the memo binding")?;
                let body = self.parse_term()?;
                Ok(LocalDecl::Lazy { pat, body, span })
            }
            t => Err(self.err(format!("expected a local declaration, found {}", describe(&t)))),
        }
    }

    // ---------------------------------------------------------- patterns

    fn starts_atomic_pattern(&self) -> bool {
        matches!(
            self.peek_tok(),
            Tok::VarId(_) | Tok::ConId(_) | Tok::Int(_) | Tok::StrLit(_) | Tok::CharLit(_)
                | Tok::LParen | Tok::LBracket | Tok::Underscore | Tok::Tilde
        ) || matches!(self.peek_tok(), Tok::Op(s) if s == "-")
    }

    fn parse_pattern(&mut self) -> Result<Pat> {
        let pat = if matches!(self.peek_tok(), Tok::ConId(_)) {
            let path = self.parse_con_path()?;
            let span = path.span();
            if self.at(&Tok::LBrace) {
                let fields = self.parse_pattern_fields()?;
                return Ok(Pat::Record(path, fields, span));
            }
            let mut args = Vec::new();
            while self.starts_atomic_pattern() {
                args.push(self.parse_atomic_pattern()?);
            }
            Pat::Con(path, args, span)
        } else {
            self.parse_atomic_pattern()?
        };
        if self.at(&Tok::ColonColon) {
            return Err(self.err(
                "pattern type annotations are not supported in the bootstrap slice",
            ));
        }
        Ok(pat)
    }

    /// `{ x = p, ... }` after a constructor path; at least one field.
    fn parse_pattern_fields(&mut self) -> Result<Vec<(Name, Pat)>> {
        self.expect(&Tok::LBrace, "`{`")?;
        let mut fields = Vec::new();
        loop {
            let n_span = self.peek().span;
            let name = match self.peek_tok().clone() {
                Tok::VarId(s) if s != "_" => {
                    self.bump();
                    Name::new(s, n_span)
                }
                t => {
                    return Err(self.err(format!(
                        "expected a record field name in the pattern, found {}",
                        describe(&t)
                    )))
                }
            };
            self.expect(&Tok::Equals, "`=` in the record pattern field")?;
            let pat = self.parse_pattern()?;
            fields.push((name, pat));
            if self.at(&Tok::Comma) {
                self.bump();
            } else {
                break;
            }
        }
        self.expect(&Tok::RBrace, "`}` closing the record pattern")?;
        Ok(fields)
    }

    fn parse_atomic_pattern(&mut self) -> Result<Pat> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::Op(s) if s == "-" => {
                // Negative integer literal patterns fold the prefix minus
                // the way expressions do.
                self.bump();
                match self.parse_atomic_pattern()? {
                    Pat::Int(v, _) => Ok(Pat::Int(negate_literal(&v), span)),
                    other => Err(Error::at(
                        other.span(),
                        "only integer literals may be negated in patterns",
                    )),
                }
            }
            Tok::Underscore => {
                self.bump();
                Ok(Pat::Wild(span))
            }
            Tok::VarId(s) if s == "_" => {
                self.bump();
                Ok(Pat::Wild(span))
            }
            Tok::VarId(s) => {
                self.bump();
                if self.at(&Tok::At) {
                    self.bump();
                    let inner = self.parse_atomic_pattern()?;
                    Ok(Pat::As(Name::new(s, span), Box::new(inner), span))
                } else {
                    Ok(Pat::Var(Name::new(s, span), span))
                }
            }
            Tok::Int(v) => {
                self.bump();
                Ok(Pat::Int(v, span))
            }
            Tok::ConId(_) => {
                let path = self.parse_con_path()?;
                if self.at(&Tok::LBrace) {
                    let fields = self.parse_pattern_fields()?;
                    return Ok(Pat::Record(path, fields, span));
                }
                Ok(Pat::Con(path, Vec::new(), span))
            }
            Tok::LParen => {
                self.bump();
                if self.at(&Tok::RParen) {
                    self.bump();
                    return Ok(Pat::Unit(span));
                }
                let first = self.parse_pattern()?;
                if self.at(&Tok::Comma) {
                    let mut items = vec![first];
                    while self.at(&Tok::Comma) {
                        self.bump();
                        items.push(self.parse_pattern()?);
                    }
                    self.expect(&Tok::RParen, "`)` closing the tuple pattern")?;
                    return Ok(Pat::Tuple(items, span));
                }
                self.expect(&Tok::RParen, "`)`")?;
                Ok(first)
            }
            Tok::LBracket => {
                self.bump();
                let mut items = Vec::new();
                if !self.at(&Tok::RBracket) {
                    loop {
                        items.push(self.parse_pattern()?);
                        if self.at(&Tok::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&Tok::RBracket, "`]` closing the list pattern")?;
                Ok(Pat::List(items, span))
            }
            Tok::StrLit(_) | Tok::CharLit(_) | Tok::FloatLit(_) => Err(self.err(
                "string, character and float literal patterns are not supported in the bootstrap slice",
            )),
            Tok::Tilde => Err(self.err(
                "irrefutable (`~`) patterns are not supported in the bootstrap slice",
            )),
            t => Err(self.err(format!("expected a pattern, found {}", describe(&t)))),
        }
    }

    // ------------------------------------------------------------ terms

    fn starts_postfix(&self) -> bool {
        matches!(
            self.peek_tok(),
            Tok::VarId(_)
                | Tok::ConId(_)
                | Tok::Int(_)
                | Tok::FloatLit(_)
                | Tok::StrLit(_)
                | Tok::CharLit(_)
                | Tok::LParen
                | Tok::LBracket
                | Tok::LQuote
                | Tok::DollarParen
                | Tok::Underscore
        )
    }

    fn parse_term(&mut self) -> Result<Term> {
        match self.peek_tok() {
            Tok::Backslash => self.parse_lambda(),
            Tok::Let => self.parse_let(),
            Tok::Tilde => Err(self.err(
                "memo bindings (`~ pattern = term`) are not supported in the bootstrap slice",
            )),
            Tok::If => self.parse_if(),
            Tok::Case => self.parse_case(),
            Tok::Do => Err(self.err("do expressions are not supported in the bootstrap slice")),
            Tok::Handle => Err(self.err("handle expressions are not supported in the bootstrap slice")),
            Tok::With => Err(self.err("with expressions are not supported in the bootstrap slice")),
            Tok::Forall => Err(self.err(
                "forall terms are not supported in the bootstrap slice; write the quantifier in a type",
            )),
            Tok::Sigma => Err(self.err(
                "Sigma terms are not supported in the bootstrap slice; write the pair type in a type",
            )),
            Tok::Using => Err(self.err(
                "using constraints in terms are not supported in the bootstrap slice",
            )),
            _ => self.parse_annotation(),
        }
    }

    /// `dollar_expr [ "::" type ]`.
    fn parse_annotation(&mut self) -> Result<Term> {
        let t = self.parse_infix_chain()?;
        if self.at(&Tok::ColonColon) {
            self.bump();
            let ty = self.parse_type()?;
            let span = t.span();
            return Ok(Term::Ann(Box::new(t), ty, span));
        }
        Ok(t)
    }

    /// `application { operator, application }`, with `$` taking a full term
    /// on its right and ending the chain.
    fn parse_infix_chain(&mut self) -> Result<Term> {
        let mut terms = vec![self.parse_application()?];
        let mut ops: Vec<Name> = Vec::new();
        loop {
            let span = self.peek().span;
            let is_dollar = match self.peek_tok() {
                Tok::Op(_) => false,
                Tok::Dollar => true,
                _ => break,
            };
            let name = if is_dollar {
                self.bump();
                Name::new("$", span)
            } else {
                match self.bump().tok {
                    Tok::Op(s) => Name::new(s, span),
                    _ => unreachable!(),
                }
            };
            if is_dollar {
                let rhs = self.parse_term()?;
                let chain_span = terms[0].span();
                let lhs = if ops.is_empty() {
                    terms.pop().expect("one term")
                } else {
                    Term::Infix(std::mem::take(&mut terms), std::mem::take(&mut ops), chain_span)
                };
                return Ok(Term::Infix(vec![lhs, rhs], vec![name], chain_span));
            }
            ops.push(name);
            terms.push(self.parse_application()?);
        }
        let span = terms[0].span();
        if ops.is_empty() {
            Ok(terms.pop().expect("one term"))
        } else {
            Ok(Term::Infix(terms, ops, span))
        }
    }

    /// `prefix { postfix }`, where the head may carry prefix negation and the
    /// arguments are single postfixed atoms.
    fn parse_application(&mut self) -> Result<Term> {
        let head = self.parse_prefix()?;
        let mut args = Vec::new();
        while self.starts_postfix() {
            args.push(self.parse_postfix()?);
        }
        if args.is_empty() {
            Ok(head)
        } else {
            let span = head.span();
            Ok(Term::App(Box::new(head), args, span))
        }
    }

    /// `{ "-" }, postfix`; `- f x` is `App(Neg(f), [x])`.
    fn parse_prefix(&mut self) -> Result<Term> {
        let mut negs = Vec::new();
        while let Tok::Op(s) = self.peek_tok() {
            if s != "-" {
                break;
            }
            negs.push(self.peek().span);
            self.bump();
        }
        let mut t = self.parse_postfix()?;
        for span in negs.into_iter().rev() {
            t = Term::Neg(Box::new(t), span);
        }
        Ok(t)
    }

    /// `atomic_term { projection | type_argument | supplied_capabilities }`.
    /// Type arguments and capability supplies are outside the bootstrap
    /// slice; projection is `e.x`.
    fn parse_postfix(&mut self) -> Result<Term> {
        let mut t = self.parse_atom()?;
        loop {
            match self.peek_tok() {
                Tok::Dot => {
                    self.bump();
                    let n_span = self.peek().span;
                    let name = match self.peek_tok().clone() {
                        Tok::VarId(s) if s != "_" => {
                            self.bump();
                            Name::new(s, n_span)
                        }
                        tok => {
                            return Err(self.err(format!(
                                "expected a field name after `.`, found {}",
                                describe(&tok)
                            )))
                        }
                    };
                    let span = t.span();
                    t = Term::Proj(Box::new(t), name, span);
                }
                Tok::At => Err(self.err(
                    "type arguments and capability supplies (`@`) are not supported in the bootstrap slice",
                ))?,
                Tok::Bang => {
                    Err(self.err("macro invocations are not supported in the bootstrap slice"))?
                }
                Tok::LBrace => Err(self.err(
                    "record construction must place `R { x = e, ... }` directly on a constructor name",
                ))?,
                _ => break,
            }
        }
        Ok(t)
    }

    /// `{ x = e, ... }` after a constructor name; fields may be empty.
    fn parse_record_assignments(&mut self) -> Result<Vec<(Name, Term)>> {
        self.expect(&Tok::LBrace, "`{`")?;
        let mut fields = Vec::new();
        if !self.at(&Tok::RBrace) {
            loop {
                let n_span = self.peek().span;
                let name = match self.peek_tok().clone() {
                    Tok::VarId(s) if s != "_" => {
                        self.bump();
                        Name::new(s, n_span)
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a record field name, found {}",
                            describe(&t)
                        )))
                    }
                };
                self.expect(&Tok::Equals, "`=` in the record field assignment")?;
                let term = self.parse_term()?;
                fields.push((name, term));
                if self.at(&Tok::Comma) {
                    self.bump();
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RBrace, "`}` closing the record construction")?;
        Ok(fields)
    }

    fn parse_atom(&mut self) -> Result<Term> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::Underscore => {
                self.bump();
                Ok(Term::Hole(span))
            }
            Tok::VarId(s) if s == "_" => {
                self.bump();
                Ok(Term::Hole(span))
            }
            Tok::VarId(s) if s == "handler" => Err(self.err(
                "handler expressions are not supported in the bootstrap slice",
            )),
            Tok::VarId(s) => {
                self.bump();
                Ok(Term::Var(VarPath { quals: Vec::new(), name: Name::new(s, span) }))
            }
            Tok::ConId(_) => {
                let path = self.parse_con_path()?;
                if self.at(&Tok::Dot) && matches!(self.peek_tok_at(1), Some(Tok::VarId(_))) {
                    self.bump();
                    let v_span = self.peek().span;
                    if let Tok::VarId(v) = self.peek_tok().clone() {
                        self.bump();
                        return Ok(Term::Var(VarPath {
                            quals: path.segments,
                            name: Name::new(v, v_span),
                        }));
                    }
                    unreachable!()
                }
                if self.at(&Tok::LBrace) {
                    let span = path.span();
                    let fields = self.parse_record_assignments()?;
                    return Ok(Term::Record(path, fields, span));
                }
                Ok(Term::Con(path))
            }
            Tok::Int(v) => {
                self.bump();
                Ok(Term::Int(v, span))
            }
            Tok::StrLit(s) => {
                self.bump();
                Ok(Term::Str(s, span))
            }
            Tok::CharLit(s) => {
                self.bump();
                Ok(Term::Char(s, span))
            }
            Tok::FloatLit(_) => Err(self.err(
                "float literals are not supported in the bootstrap slice (there is no AST node for them)",
            )),
            Tok::LParen => {
                self.bump();
                if self.at(&Tok::RParen) {
                    self.bump();
                    return Ok(Term::Unit(span));
                }
                if matches!(self.peek_tok(), Tok::Op(_))
                    && matches!(self.peek_tok_at(1), Some(Tok::RParen))
                {
                    let op = match self.bump().tok {
                        Tok::Op(s) => s,
                        _ => unreachable!(),
                    };
                    self.bump();
                    return Ok(Term::Var(VarPath { quals: Vec::new(), name: Name::new(op, span) }));
                }
                let first = self.parse_term()?;
                if self.at(&Tok::Comma) {
                    let mut items = vec![first];
                    while self.at(&Tok::Comma) {
                        self.bump();
                        items.push(self.parse_term()?);
                    }
                    self.expect(&Tok::RParen, "`)` closing the tuple")?;
                    return Ok(Term::Tuple(items, span));
                }
                self.expect(&Tok::RParen, "`)`")?;
                Ok(first)
            }
            Tok::LBracket => {
                self.bump();
                let mut items = Vec::new();
                if !self.at(&Tok::RBracket) {
                    loop {
                        items.push(self.parse_term()?);
                        if self.at(&Tok::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&Tok::RBracket, "`]` closing the list")?;
                Ok(Term::List(items, span))
            }
            Tok::LQuote => Err(self.err(
                "typed quotations (`[| ... |]`) are not supported in the bootstrap slice",
            )),
            Tok::DollarParen => {
                Err(self.err("splices (`$( ...)`) are not supported in the bootstrap slice"))
            }
            t => Err(self.err(format!("expected an expression, found {}", describe(&t)))),
        }
    }

    fn parse_lambda(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Backslash, "`\\`")?.span;
        let mut pats = Vec::new();
        while self.starts_atomic_pattern() {
            pats.push(self.parse_atomic_pattern()?);
        }
        if pats.is_empty() {
            return Err(self.err("a lambda needs at least one pattern"));
        }
        self.expect(&Tok::Arrow, "`->`")?;
        let body = self.parse_term()?;
        Ok(Term::Lam(pats, Box::new(body), span))
    }

    fn parse_let(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Let, "`let`")?.span;
        let decls = self.parse_block_items("the local block", |p| p.parse_local_decl())?;
        self.expect(&Tok::In, "`in`")?;
        let body = self.parse_term()?;
        Ok(Term::Let(decls, Box::new(body), span))
    }

    fn parse_if(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::If, "`if`")?.span;
        let c = self.parse_term()?;
        self.expect(&Tok::Then, "`then`")?;
        let t = self.parse_term()?;
        self.expect(&Tok::Else, "`else`")?;
        let e = self.parse_term()?;
        Ok(Term::If(Box::new(c), Box::new(t), Box::new(e), span))
    }

    fn parse_case(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Case, "`case`")?.span;
        let scrut = self.parse_term()?;
        self.expect(&Tok::Of, "`of`")?;
        let alts = self.parse_block_items("the alternative block", |p| {
            let pat = p.parse_pattern()?;
            let guard = if p.at(&Tok::Pipe) {
                p.bump();
                Some(p.parse_term()?)
            } else {
                None
            };
            p.expect(&Tok::Arrow, "`->`")?;
            let body = p.parse_term()?;
            let span = pat.span();
            Ok(Alt { pat, guard, body, span })
        })?;
        Ok(Term::Case(Box::new(scrut), alts, span))
    }

    // ------------------------------------------------------------ types

    fn parse_type(&mut self) -> Result<Type> {
        match self.peek_tok() {
            Tok::Forall => self.parse_forall(),
            Tok::Sigma => self.parse_sigma(),
            Tok::Using => self.parse_using(),
            _ => self.parse_arrow_type(),
        }
    }

    fn parse_forall(&mut self) -> Result<Type> {
        let span = self.expect(&Tok::Forall, "`forall`")?.span;
        let mut binders = Vec::new();
        binders.push(self.parse_type_binder()?);
        loop {
            match self.peek_tok() {
                Tok::VarId(s) if s != "_" => binders.push(self.parse_type_binder()?),
                Tok::LParen | Tok::LBrace => binders.push(self.parse_type_binder()?),
                _ => break,
            }
        }
        self.expect(&Tok::Dot, "`.` after the `forall` binders")?;
        let body = self.parse_type()?;
        Ok(Type::Forall(binders, Box::new(body), span))
    }

    fn parse_sigma(&mut self) -> Result<Type> {
        let span = self.expect(&Tok::Sigma, "`Sigma`")?.span;
        if !self.is_typed_binder_head() {
            return Err(self.err(
                "a `Sigma` binder must be annotated, as in `Sigma (n :: Nat). t`",
            ));
        }
        let (name, ann) = self.parse_typed_binder()?;
        self.expect(&Tok::Dot, "`.` after the `Sigma` binder")?;
        let body = self.parse_type()?;
        Ok(Type::Sigma(name, Box::new(ann), Box::new(body), span))
    }

    fn parse_using(&mut self) -> Result<Type> {
        let span = self.expect(&Tok::Using, "`using`")?.span;
        self.expect(&Tok::LParen, "`(` after `using`")?;
        let mut cs = Vec::new();
        loop {
            let c_span = self.peek().span;
            let name = match self.peek_tok().clone() {
                Tok::VarId(s) if s != "_" => {
                    self.bump();
                    Name::new(s, c_span)
                }
                t => {
                    return Err(self.err(format!(
                        "expected a constraint name, found {}",
                        describe(&t)
                    )))
                }
            };
            self.expect(&Tok::ColonColon, "`::` in the constraint")?;
            let ty = self.parse_type_app()?;
            cs.push(decompose_constraint(name, ty, c_span)?);
            if self.at(&Tok::Comma) {
                self.bump();
            } else {
                break;
            }
        }
        self.expect(&Tok::RParen, "`)` after the constraint list")?;
        self.expect(&Tok::Darrow, "`=>`")?;
        let body = self.parse_type()?;
        Ok(Type::Using(cs, Box::new(body), span))
    }

    fn parse_arrow_type(&mut self) -> Result<Type> {
        let span = self.peek().span;
        if self.is_typed_binder_head() {
            let (name, ann) = self.parse_typed_binder()?;
            if mult_name(self.peek_tok()).is_some() {
                return Err(self.err(
                    "multiplicities on dependent function types are not supported in the bootstrap slice",
                ));
            }
            self.expect(&Tok::Arrow, "`->` after the dependent binder")?;
            if self.at(&Tok::LBrace) {
                return Err(self.err(
                    "effect rows on dependent function types are not supported in the bootstrap slice",
                ));
            }
            let res = self.parse_type()?;
            return Ok(Type::DepFun(name, Box::new(ann), Box::new(res), span));
        }
        if self.at(&Tok::LBrace) {
            return Err(self.err(
                "implicit dependent function types (`{x :: A} -> B`) are not supported in the bootstrap slice",
            ));
        }
        let arg = self.parse_type_app()?;
        if mult_name(self.peek_tok()).is_some() || self.at(&Tok::Arrow) {
            let mult = self.parse_mult_opt()?;
            self.expect(&Tok::Arrow, "`->`")?;
            let eff = self.parse_effect_row_opt()?;
            let res = self.parse_type()?;
            return Ok(Type::Fun {
                arg: Box::new(arg),
                mult,
                eff,
                res: Box::new(res),
                span,
            });
        }
        Ok(arg)
    }

    fn parse_type_app(&mut self) -> Result<Type> {
        let head = self.parse_type_atom()?;
        let mut ty = head;
        loop {
            if self.starts_type_atom() {
                let arg = self.parse_type_atom()?;
                let span = ty.span();
                ty = Type::App(Box::new(ty), Box::new(arg), span);
            } else if self.at(&Tok::LBrace) {
                // `Thunk {e}` / `Lazy {e}`: an effect row as a type argument.
                let row = self.parse_effect_row_opt()?.expect("`{` was present");
                let span = ty.span();
                ty = Type::AppRow(Box::new(ty), row, span);
            } else {
                break;
            }
        }
        Ok(ty)
    }

    fn starts_type_atom(&self) -> bool {
        matches!(self.peek_tok(), Tok::VarId(_) | Tok::ConId(_) | Tok::LParen)
    }

    fn parse_type_atom(&mut self) -> Result<Type> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::ConId(_) => {
                let path = self.parse_con_path()?;
                if self.at(&Tok::Dot) {
                    return Err(self.err("field projection is not supported in types"));
                }
                Ok(Type::Con(path, span))
            }
            Tok::VarId(s) => {
                self.bump();
                Ok(Type::Var(Name::new(s, span), span))
            }
            Tok::LParen => {
                self.bump();
                if self.at(&Tok::RParen) {
                    self.bump();
                    return Ok(Type::Unit(span));
                }
                if self.is_typed_binder_head_here() {
                    return Err(self.err(
                        "a dependent binder `(x :: A)` is only valid immediately before `->`",
                    ));
                }
                let t = self.parse_type()?;
                if self.at(&Tok::Comma) {
                    let mut items = vec![t];
                    while self.at(&Tok::Comma) {
                        self.bump();
                        items.push(self.parse_type()?);
                    }
                    self.expect(&Tok::RParen, "`)` closing the tuple type")?;
                    return Ok(Type::Tuple(items, span));
                }
                self.expect(&Tok::RParen, "`)`")?;
                Ok(t)
            }
            t => Err(self.err(format!("expected a type, found {}", describe(&t)))),
        }
    }

    fn parse_mult_opt(&mut self) -> Result<Option<Name>> {
        let span = self.peek().span;
        match mult_name(self.peek_tok()) {
            Some(m) => {
                self.bump();
                Ok(Some(Name::new(m, span)))
            }
            None => Ok(None),
        }
    }

    /// The `-> { ... }` effect row, when present.
    fn parse_effect_row_opt(&mut self) -> Result<Option<EffectRow>> {
        if !self.at(&Tok::LBrace) {
            return Ok(None);
        }
        let span = self.bump().span;
        let mut labels = Vec::new();
        let mut tail = None;
        if matches!(self.peek_tok(), Tok::VarId(_)) {
            // A lone identifier is the row tail: `{e}` (labels start with a
            // constructor name per the EBNF).
            let t_span = self.peek().span;
            if let Tok::VarId(s) = self.peek_tok().clone() {
                self.bump();
                tail = Some(Name::new(s, t_span));
            }
        } else {
            if !self.at(&Tok::RBrace) && !self.at(&Tok::Pipe) {
                loop {
                    let path = self.parse_con_path()?;
                    // Effect label arguments are region names (at most one).
                    let mut args = Vec::new();
                    while let Tok::VarId(s) = self.peek_tok().clone() {
                        let span = self.peek().span;
                        self.bump();
                        args.push(Name::new(s, span));
                    }
                    labels.push((path, args));
                    if self.at(&Tok::Comma) {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
            if self.at(&Tok::Pipe) {
                self.bump();
                let t_span = self.peek().span;
                match self.peek_tok().clone() {
                    Tok::VarId(s) => {
                        self.bump();
                        tail = Some(Name::new(s, t_span));
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a row variable after `|`, found {}",
                            describe(&t)
                        )))
                    }
                }
            }
        }
        self.expect(&Tok::RBrace, "`}` closing the effect row")?;
        Ok(Some(EffectRow { labels, tail, span }))
    }

    fn is_typed_binder_head(&self) -> bool {
        if !self.at(&Tok::LParen) {
            return false;
        }
        self.typed_binder_after_lparen(1)
    }

    fn is_typed_binder_head_here(&self) -> bool {
        // The `(` has already been consumed.
        self.typed_binder_after_lparen(0)
    }

    fn typed_binder_after_lparen(&self, base: usize) -> bool {
        match self.peek_tok_at(base) {
            Some(Tok::Erased) | Some(Tok::Own) => true,
            Some(Tok::VarId(s)) if s != "_" => {
                matches!(self.peek_tok_at(base + 1), Some(Tok::ColonColon))
            }
            _ => false,
        }
    }

    /// `( [erased|own] x :: t )`.
    fn parse_typed_binder(&mut self) -> Result<(Name, Type)> {
        self.expect(&Tok::LParen, "`(`")?;
        if matches!(self.peek_tok(), Tok::Erased | Tok::Own) {
            return Err(self.err(
                "the `erased` and `own` binder qualifiers are not supported in the bootstrap slice",
            ));
        }
        let span = self.peek().span;
        let name = match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => {
                self.bump();
                Name::new(s, span)
            }
            t => return Err(self.err(format!("expected a binder name, found {}", describe(&t)))),
        };
        self.expect(&Tok::ColonColon, "`::` in the binder")?;
        let ty = self.parse_type()?;
        self.expect(&Tok::RParen, "`)` closing the binder")?;
        Ok((name, ty))
    }

    /// A `forall` binder: `x`, `(x :: t)`, or `{x [:: t]}`.
    fn parse_type_binder(&mut self) -> Result<TypeBinder> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => {
                self.bump();
                Ok(TypeBinder { name: Name::new(s, span), ann: None, implicit: false })
            }
            Tok::LParen => {
                let (name, ann) = self.parse_typed_binder()?;
                Ok(TypeBinder { name, ann: Some(ann), implicit: false })
            }
            Tok::LBrace => {
                self.bump();
                if matches!(self.peek_tok(), Tok::Erased | Tok::Own) {
                    return Err(self.err(
                        "the `erased` and `own` binder qualifiers are not supported in the bootstrap slice",
                    ));
                }
                let name = match self.peek_tok().clone() {
                    Tok::VarId(s) if s != "_" => {
                        self.bump();
                        Name::new(s, span)
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a binder name, found {}",
                            describe(&t)
                        )))
                    }
                };
                let ann = if self.at(&Tok::ColonColon) {
                    self.bump();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.expect(&Tok::RBrace, "`}` closing the implicit binder")?;
                Ok(TypeBinder { name, ann, implicit: true })
            }
            t => Err(self.err(format!("expected a binder, found {}", describe(&t)))),
        }
    }

    // ------------------------------------------------------------ paths

    fn parse_con_path(&mut self) -> Result<ConPath> {
        let span = self.peek().span;
        let mut segments = Vec::new();
        match self.peek_tok().clone() {
            Tok::ConId(s) => {
                self.bump();
                segments.push(Name::new(s, span));
            }
            t => {
                return Err(self.err(format!("expected a constructor name, found {}", describe(&t))))
            }
        }
        while self.at(&Tok::Dot) && matches!(self.peek_tok_at(1), Some(Tok::ConId(_))) {
            self.bump();
            let d_span = self.peek().span;
            match self.bump().tok {
                Tok::ConId(s) => segments.push(Name::new(s, d_span)),
                _ => unreachable!(),
            }
        }
        Ok(ConPath { segments })
    }
}

fn decompose_constraint(name: Name, ty: Type, span: Span) -> Result<Constraint> {
    let mut args = Vec::new();
    let mut t = ty;
    let head = loop {
        match t {
            Type::App(f, a, _) => {
                args.push(*a);
                t = *f;
            }
            other => break other,
        }
    };
    args.reverse();
    match head {
        Type::Con(class, _) => Ok(Constraint { name, class, args, span }),
        _ => Err(Error::at(span, "the head of a constraint must be a class constructor")),
    }
}

// ------------------------------------------------------- fixity resolution

fn resolve_module(m: &mut Module, fix: &FixityTable) -> Result<()> {
    for d in &mut m.decls {
        match d {
            Decl::Equation { eq, .. } => resolve_equation(eq, fix)?,
            Decl::Instance(inst) => {
                for eq in &mut inst.methods {
                    resolve_equation(eq, fix)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn resolve_equation(eq: &mut Equation, fix: &FixityTable) -> Result<()> {
    resolve_term(&mut eq.body, fix)?;
    for l in &mut eq.locals {
        match l {
            LocalDecl::Eq { body, .. }
            | LocalDecl::Bind { body, .. }
            | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
            LocalDecl::Sig { .. } => {}
        }
    }
    Ok(())
}

fn resolve_term(t: &mut Term, fix: &FixityTable) -> Result<()> {
    match t {
        Term::App(f, args, _) => {
            resolve_term(f, fix)?;
            for a in args {
                resolve_term(a, fix)?;
            }
        }
        Term::Lam(_, body, _) => resolve_term(body, fix)?,
        Term::Let(decls, body, _) => {
            for l in decls {
                match l {
                    LocalDecl::Eq { body, .. }
                    | LocalDecl::Bind { body, .. }
                    | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
                    LocalDecl::Sig { .. } => {}
                }
            }
            resolve_term(body, fix)?;
        }
        Term::If(c, a, b, _) => {
            resolve_term(c, fix)?;
            resolve_term(a, fix)?;
            resolve_term(b, fix)?;
        }
        Term::Case(scrut, alts, _) => {
            resolve_term(scrut, fix)?;
            for alt in alts {
                if let Some(g) = &mut alt.guard {
                    resolve_term(g, fix)?;
                }
                resolve_term(&mut alt.body, fix)?;
            }
        }
        Term::Ann(e, _, _) => resolve_term(e, fix)?,
        Term::Neg(e, _) => resolve_term(e, fix)?,
        Term::Tuple(items, _) | Term::List(items, _) => {
            for x in items {
                resolve_term(x, fix)?;
            }
        }
        Term::Proj(e, _, _) => resolve_term(e, fix)?,
        Term::Record(_, fields, _) => {
            for (_, x) in fields {
                resolve_term(x, fix)?;
            }
        }
        Term::Infix(_, _, _) => {
            let (terms, ops, span) = match std::mem::replace(t, Term::Unit(Span { line: 0, col: 0 })) {
                Term::Infix(terms, ops, span) => (terms, ops, span),
                _ => unreachable!(),
            };
            let mut terms = terms;
            for x in &mut terms {
                resolve_term(x, fix)?;
            }
            *t = resolve_chain(terms, ops, span, fix)?;
        }
        _ => {}
    }
    Ok(())
}

fn op_fixity(op: &Name, fix: &FixityTable) -> Result<(i8, Assoc)> {
    if op.text == "$" {
        // Reserved right-associative application, looser than every ordinary
        // operator; no declaration is needed or consulted.
        return Ok((-1, Assoc::Right));
    }
    match fix.get(&op.text) {
        Some((assoc, prec)) => Ok((prec as i8, assoc)),
        None => Err(Error::at(
            op.span,
            format!(
                "there is no fixity declared for the operator `{}`, and there is no default for an undeclared operator",
                op.text
            ),
        )),
    }
}

fn reduce(vals: &mut Vec<Term>, op: Name, span: Span) -> Result<()> {
    let rhs = vals.pop().expect("right operand");
    if vals.is_empty() {
        return Err(Error::at(span, "malformed infix chain"));
    }
    let lhs = vals.pop().expect("left operand");
    let t = if op.text == "$" {
        Term::App(Box::new(lhs), vec![rhs], span)
    } else if op.text == "&&" || op.text == "||" {
        // Surface `&&`/`||` are special forms that short-circuit: they
        // elaborate to `if`, not to method application (primitive types
        // specification section 10). The strict `(&&)`/`(||)` class
        // methods are different operations and stay reachable as values.
        let bool_con = |text: &str| {
            Term::Con(ConPath {
                segments: vec![Name {
                    text: text.to_string(),
                    span,
                }],
            })
        };
        if op.text == "&&" {
            Term::If(Box::new(lhs), Box::new(rhs), Box::new(bool_con("False")), span)
        } else {
            Term::If(Box::new(lhs), Box::new(bool_con("True")), Box::new(rhs), span)
        }
    } else {
        let f = Term::Var(VarPath { quals: Vec::new(), name: op });
        Term::App(Box::new(f), vec![lhs, rhs], span)
    };
    vals.push(t);
    Ok(())
}

fn resolve_chain(
    terms: Vec<Term>,
    ops: Vec<Name>,
    span: Span,
    fix: &FixityTable,
) -> Result<Term> {
    let mut vals: Vec<Term> = Vec::with_capacity(terms.len());
    let mut ostack: Vec<(Name, i8, Assoc)> = Vec::new();
    let mut ts = terms.into_iter();
    vals.push(ts.next().expect("nonempty chain"));
    for (op, term) in ops.into_iter().zip(ts) {
        let (prec, assoc) = op_fixity(&op, fix)?;
        loop {
            let pop = match ostack.last() {
                None => false,
                Some((top, top_prec, top_assoc)) => {
                    if *top_prec > prec {
                        true
                    } else if *top_prec < prec {
                        false
                    } else {
                        match (*top_assoc, assoc) {
                            (Assoc::Left, Assoc::Left) => true,
                            (Assoc::Right, Assoc::Right) => false,
                            (Assoc::None, _) | (_, Assoc::None) => {
                                return Err(Error::at(
                                    op.span,
                                    format!(
                                        "the non-associative operator `{}` cannot be chained at precedence {prec}; add parentheses",
                                        if top_assoc == &Assoc::None { &top.text } else { &op.text }
                                    ),
                                ))
                            }
                            _ => {
                                return Err(Error::at(
                                    op.span,
                                    format!(
                                        "the operators `{}` and `{}` at equal precedence {prec} have incompatible associativity; add parentheses",
                                        top.text, op.text
                                    ),
                                ))
                            }
                        }
                    }
                }
            };
            if !pop {
                break;
            }
            let (top, _, _) = ostack.pop().expect("operator");
            reduce(&mut vals, top, span)?;
        }
        ostack.push((op, prec, assoc));
        vals.push(term);
    }
    while let Some((op, _, _)) = ostack.pop() {
        reduce(&mut vals, op, span)?;
    }
    if vals.len() != 1 {
        return Err(Error::at(span, "malformed infix chain"));
    }
    Ok(vals.pop().expect("one result"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Result<ParsedModule> {
        let toks = crate::lexer::lex(src)?;
        let toks = crate::layout::layout(toks)?;
        parse_module(&toks, &FixityTable::new())
    }

    fn ok(src: &str) -> ParsedModule {
        parse(src).expect("parse should succeed")
    }

    fn err(src: &str) -> String {
        parse(src).expect_err("parse should fail").message
    }

    fn eq_body<'a>(m: &'a Module, name: &str) -> &'a Term {
        for d in &m.decls {
            if let Decl::Equation { eq, .. } = d {
                if eq.name.text == name {
                    return &eq.body;
                }
            }
        }
        panic!("no equation named {name}");
    }

    fn sig_ty<'a>(m: &'a Module, name: &str) -> &'a Type {
        for d in &m.decls {
            if let Decl::ValueSig { name: n, ty, .. } = d {
                if n.text == name {
                    return ty;
                }
            }
        }
        panic!("no signature named {name}");
    }

    fn app(t: &Term) -> (&Term, &[Term]) {
        match t {
            Term::App(f, args, _) => (f, args),
            other => panic!("not an application: {other:?}"),
        }
    }

    fn var(t: &Term) -> &str {
        match t {
            Term::Var(p) => &p.name.text,
            other => panic!("not a variable: {other:?}"),
        }
    }

    fn int(t: &Term) -> i64 {
        match t {
            Term::Int(v, _) => v.parse().unwrap(),
            other => panic!("not an integer: {other:?}"),
        }
    }

    #[test]
    fn parses_every_declaration_form() {
        let p = ok(
            "module T where\n\
             \x20 import Prelude\n\
             \x20 import Data.List as DL\n\
             \x20 infixl 6 +\n\
             \x20 infixr 5 &&\n\
             \x20 data Option a where\n\
             \x20   None :: Option a\n\
             \x20   Some :: a -> Option a\n\
             \x20 signature Eq a where\n\
             \x20   eq :: a -> a -> Bool\n\
             \x20   (/=) :: a -> a -> Bool\n\
             \x20 implicit instance EqInt a :: using (c :: Eq a) => Eq a where\n\
             \x20   eq x y = primEq x y\n\
             \x20 proof thm :: Int -> Int\n\
             \x20 thm x = x\n\
             \x20 plus :: Int -> Int -> Int\n\
             \x20 plus x y = x + y\n\
             \x20 foreign import ccall \"sin\" sinF :: Int -> Int\n\
             \x20 foreign export ccall \"put\" putF :: Int -> Int\n",
        );
        let m = &p.module;
        assert_eq!(m.name.text, "T");
        assert_eq!(m.imports.len(), 2);
        assert_eq!(m.imports[0].path.parts(), vec!["Prelude"]);
        assert_eq!(m.imports[1].path.parts(), vec!["Data", "List"]);
        assert_eq!(m.imports[1].alias.as_ref().unwrap().text, "DL");
        assert_eq!(m.decls.len(), 13);

        let data = m
            .decls
            .iter()
            .find_map(|d| match d {
                Decl::Data(x) => Some(x),
                _ => None,
            })
            .expect("data decl");
        assert_eq!(data.name.text, "Option");
        assert_eq!(data.params.len(), 1);
        assert_eq!(data.params[0].text, "a");
        assert_eq!(data.ctors.len(), 2);
        assert_eq!(data.ctors[0].name.text, "None");
        assert!(matches!(data.ctors[1].ty, Type::Fun { .. }));

        let sig = m
            .decls
            .iter()
            .find_map(|d| match d {
                Decl::Signature(x) => Some(x),
                _ => None,
            })
            .expect("signature decl");
        assert_eq!(sig.name.text, "Eq");
        assert_eq!(sig.params[0].text, "a");
        assert_eq!(sig.methods.len(), 2);
        assert_eq!(sig.methods[1].0.text, "/=");

        let inst = m
            .decls
            .iter()
            .find_map(|d| match d {
                Decl::Instance(x) => Some(x),
                _ => None,
            })
            .expect("instance decl");
        assert!(inst.implicit);
        assert_eq!(inst.name.text, "EqInt");
        assert_eq!(inst.params.len(), 1);
        assert_eq!(inst.ctx.len(), 1);
        assert_eq!(inst.ctx[0].name.text, "c");
        assert_eq!(inst.ctx[0].class.parts(), vec!["Eq"]);
        assert!(matches!(inst.target, Type::App(_, _, _)));
        assert_eq!(inst.methods.len(), 1);

        match &m.decls[7] {
            Decl::ValueSig { name, modifier, .. } => {
                assert_eq!(name.text, "thm");
                assert_eq!(modifier.as_ref().unwrap().text, "proof");
            }
            other => panic!("expected a value signature, got {other:?}"),
        }

        let f = m
            .decls
            .iter()
            .find_map(|d| match d {
                Decl::Foreign(x) if x.is_import => Some(x),
                _ => None,
            })
            .expect("foreign import");
        assert_eq!(f.abi, "ccall");
        assert_eq!(f.symbol, "sin");
        assert_eq!(f.name.text, "sinF");

        assert_eq!(p.fixities.get("+"), Some((Assoc::Left, 6)));
        assert_eq!(p.fixities.get("&&"), Some((Assoc::Right, 5)));
    }

    #[test]
    fn precedence_and_order_independence() {
        // Fixity declarations come after the equation that needs them.
        let p = ok("module T where\n  f = 1 + 2 * 3\n  infixl 6 +\n  infixl 7 *\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "+");
        assert_eq!(int(&args[0]), 1);
        let (g, gargs) = app(&args[1]);
        assert_eq!(var(g), "*");
        assert_eq!(int(&gargs[0]), 2);
        assert_eq!(int(&gargs[1]), 3);
    }

    #[test]
    fn left_associativity() {
        let p = ok("module T where\n  infixl 6 -\n  f = 1 - 2 - 3\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "-");
        assert_eq!(int(&args[1]), 3);
        let (g, gargs) = app(&args[0]);
        assert_eq!(var(g), "-");
        assert_eq!(int(&gargs[0]), 1);
        assert_eq!(int(&gargs[1]), 2);
    }

    #[test]
    fn right_associativity() {
        let p = ok("module T where\n  infixr 6 ^\n  f = 1 ^ 2 ^ 3\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "^");
        assert_eq!(int(&args[0]), 1);
        let (g, gargs) = app(&args[1]);
        assert_eq!(var(g), "^");
        assert_eq!(int(&gargs[0]), 2);
        assert_eq!(int(&gargs[1]), 3);
    }

    #[test]
    fn non_associative_chain_is_rejected() {
        let e = err("module T where\n  infix 4 ==\n  f = 1 == 2 == 3\n");
        assert!(e.contains("cannot be chained"), "{e}");
    }

    #[test]
    fn mixed_associativity_at_equal_precedence_is_rejected() {
        let e = err("module T where\n  infixl 6 +\n  infixr 6 ^\n  f = 1 + 2 ^ 3\n");
        assert!(e.contains("incompatible associativity"), "{e}");
    }

    #[test]
    fn undeclared_operator_is_rejected() {
        let e = err("module T where\n  f = 1 + 2\n");
        assert!(e.contains("no default for an undeclared operator"), "{e}");
    }

    #[test]
    fn dollar_is_right_associative_application() {
        let p = ok("module T where\n  f = g $ h $ x\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "g");
        let (g, gargs) = app(&args[0]);
        assert_eq!(var(g), "h");
        assert_eq!(var(&gargs[0]), "x");
    }

    #[test]
    fn dollar_is_looser_than_infix_operators() {
        let p = ok("module T where\n  infixl 6 +\n  f = p + q $ x\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        // `f $ x` is App(f, [x]), so the head is `p + q`.
        let (g, gargs) = app(f);
        assert_eq!(var(g), "+");
        assert_eq!(var(&gargs[0]), "p");
        assert_eq!(var(&gargs[1]), "q");
        assert_eq!(var(&args[0]), "x");
    }

    #[test]
    fn dollar_takes_a_full_term_on_the_right() {
        let p = ok("module T where\n  f = g $ \\x -> x\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "g");
        assert!(matches!(&args[0], Term::Lam(pats, _, _) if pats.len() == 1));
    }

    #[test]
    fn lambda_let_if_case_and_ascription_terms() {
        let p = ok(
            "module T where\n\
             \x20 f = \\x y -> x\n\
             \x20 g = let x = 1 in x\n\
             \x20 h = if c then 1 else 2\n\
             \x20 k = case v of\n\
             \x20   A -> 1\n\
             \x20   B | guard v -> 2\n\
             \x20   _ -> 3\n\
             \x20 m = e :: Int\n",
        );
        let m = &p.module;
        match eq_body(m, "f") {
            Term::Lam(pats, body, _) => {
                assert_eq!(pats.len(), 2);
                assert!(matches!(body.as_ref(), Term::Var(_)));
            }
            other => panic!("expected a lambda, got {other:?}"),
        }
        match eq_body(m, "g") {
            Term::Let(decls, body, _) => {
                assert_eq!(decls.len(), 1);
                assert!(matches!(
                    &decls[0],
                    LocalDecl::Eq { name, pats, .. }
                        if name.text == "x" && pats.is_empty()
                ));
                assert!(matches!(body.as_ref(), Term::Var(_)));
            }
            other => panic!("expected a let, got {other:?}"),
        }
        assert!(matches!(eq_body(m, "h"), Term::If(_, _, _, _)));
        match eq_body(m, "k") {
            Term::Case(scrut, alts, _) => {
                assert!(matches!(scrut.as_ref(), Term::Var(_)));
                assert_eq!(alts.len(), 3);
                assert!(alts[0].guard.is_none());
                assert!(alts[1].guard.is_some());
                assert!(matches!(alts[2].pat, Pat::Wild(_)));
            }
            other => panic!("expected a case, got {other:?}"),
        }
        match eq_body(m, "m") {
            Term::Ann(e, ty, _) => {
                assert!(matches!(e.as_ref(), Term::Var(_)));
                assert!(matches!(ty, Type::Con(_, _)));
            }
            other => panic!("expected an annotation, got {other:?}"),
        }
    }

    #[test]
    fn memo_bindings_parse_and_tilde_stays_reserved_elsewhere() {
        // `~ name = term` in a `where` block lands in the equation's locals.
        let p = ok("module T where\n  f x = y\n    where ~y = x\n");
        let locals = p
            .module
            .decls
            .iter()
            .find_map(|d| match d {
                Decl::Equation { eq, .. } if eq.name.text == "f" => Some(eq.locals.clone()),
                _ => None,
            })
            .expect("equation f");
        assert!(
            matches!(locals.as_slice(), [LocalDecl::Lazy { .. }]),
            "expected one memo binding, got {locals:?}"
        );
        // `~` in an argument pattern stays rejected.
        assert!(err("module T where\n  f ~x = x\n").contains("irrefutable"));
        // `~` in term position stays rejected.
        assert!(err("module T where\n  main :: Int\n  main = ~x\n").contains("memo"));
    }

    #[test]
    fn effect_rows_and_thunk_lazy_types_parse() {
        let p = ok(
            "module T where\n\
             \x20 f :: forall a e r. Lazy {e} r a ->{Eval r | e} a\n\
             \x20 f x = demand x\n",
        );
        match &p.module.decls[0] {
            Decl::ValueSig { ty, .. } => match ty {
                Type::Forall(_, body, _) => {
                    let fun: &Type = body;
                    match fun {
                        Type::Fun { eff, .. } => {
                            let eff = eff.as_ref().expect("effect row present");
                            assert_eq!(eff.labels.len(), 1);
                            assert_eq!(eff.labels[0].0.parts(), vec!["Eval"]);
                            assert_eq!(eff.labels[0].1.len(), 1);
                            assert!(eff.tail.is_some());
                        }
                        other => panic!("expected an arrow, got {other:?}"),
                    }
                }
                other => panic!("expected a forall, got {other:?}"),
            },
            other => panic!("expected a signature, got {other:?}"),
        }
    }

    #[test]
    fn local_blocks_hold_sigs_equations_and_pattern_bindings() {
        let p = ok(
            "module T where\n\
             \x20 f x = y\n\
             \x20   where\n\
             \x20     y = g x\n\
             \x20     Just z = x\n\
             \x20     w :: Int\n\
             \x20     w = 1\n",
        );
        let eqs: Vec<&Equation> = p
            .module
            .decls
            .iter()
            .filter_map(|d| match d {
                Decl::Equation { eq, .. } => Some(eq),
                _ => None,
            })
            .collect();
        assert_eq!(eqs.len(), 1);
        let eq = eqs[0];
        assert_eq!(eq.name.text, "f");
        assert_eq!(eq.pats.len(), 1);
        assert_eq!(eq.locals.len(), 4);
        assert!(matches!(&eq.locals[0], LocalDecl::Eq { name, .. } if name.text == "y"));
        assert!(matches!(
            &eq.locals[1],
            LocalDecl::Bind { pat: Pat::Con(p, args, _), .. }
                if p.parts() == vec!["Just"] && args.len() == 1
        ));
        assert!(matches!(&eq.locals[2], LocalDecl::Sig { name, .. } if name.text == "w"));
    }

    #[test]
    fn forall_arrow_using_and_dependent_types() {
        let p = ok(
            "module T where\n\
             \x20 sigA :: forall a (b :: Type) {c}. a -> b -> c\n\
             \x20 sigB :: (x :: Int) -> Int\n\
             \x20 sigC :: Int %1 ->{IO | e} Int\n\
             \x20 sigD :: using (d :: Show a) => a -> Int\n\
             \x20 sigE :: Sigma (n :: Nat). Vec n a\n",
        );
        let m = &p.module;
        match sig_ty(m, "sigA") {
            Type::Forall(binders, body, _) => {
                assert_eq!(binders.len(), 3);
                assert!(binders[0].ann.is_none() && !binders[0].implicit);
                assert!(binders[1].ann.is_some());
                assert!(binders[2].implicit);
                assert!(matches!(body.as_ref(), Type::Fun { .. }));
            }
            other => panic!("expected a forall, got {other:?}"),
        }
        match sig_ty(m, "sigB") {
            Type::DepFun(name, ann, res, _) => {
                assert_eq!(name.text, "x");
                assert!(matches!(ann.as_ref(), Type::Con(_, _)));
                assert!(matches!(res.as_ref(), Type::Con(_, _)));
            }
            other => panic!("expected a dependent function, got {other:?}"),
        }
        match sig_ty(m, "sigC") {
            Type::Fun { mult, eff, .. } => {
                assert_eq!(mult.as_ref().unwrap().text, "%1");
                let eff = eff.as_ref().expect("effect row");
                assert_eq!(eff.labels[0].0.parts(), vec!["IO"]);
                assert_eq!(eff.tail.as_ref().unwrap().text, "e");
            }
            other => panic!("expected a function type, got {other:?}"),
        }
        match sig_ty(m, "sigD") {
            Type::Using(cs, body, _) => {
                assert_eq!(cs.len(), 1);
                assert_eq!(cs[0].name.text, "d");
                assert_eq!(cs[0].class.parts(), vec!["Show"]);
                assert_eq!(cs[0].args.len(), 1);
                assert!(matches!(body.as_ref(), Type::Fun { .. }));
            }
            other => panic!("expected a using type, got {other:?}"),
        }
        match sig_ty(m, "sigE") {
            Type::Sigma(name, ann, body, _) => {
                assert_eq!(name.text, "n");
                assert!(matches!(ann.as_ref(), Type::Con(_, _)));
                assert!(matches!(body.as_ref(), Type::App(_, _, _)));
            }
            other => panic!("expected a Sigma type, got {other:?}"),
        }
    }

    #[test]
    fn application_spine_and_operator_values() {
        let p = ok("module T where\n  f = g x y\n  h = (+)\n");
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "g");
        assert_eq!(args.len(), 2);
        assert_eq!(var(eq_body(&p.module, "h")), "+");
    }

    #[test]
    fn not_supported_declarations_are_named() {
        let e = err("module T where\n  codata C where\n    x :: Int\n");
        assert!(e.contains("codata declarations are not supported"), "{e}");
        assert!(e.contains("codata slice"), "{e}");
        let e = err("module T where\n  effect E where\n    op :: Int\n");
        assert!(e.contains("effect declarations are not supported"), "{e}");
        let e = err("module T where\n  type T = Int\n");
        assert!(e.contains("type declarations are not supported"), "{e}");
        let e = err("module T where\n  upgrade A to B where\n    x = 1\n");
        assert!(e.contains("upgrade declarations are not supported"), "{e}");
        let e = err("module T where\n  generate do\n    x\n");
        assert!(e.contains("generate declarations are not supported"), "{e}");
    }

    #[test]
    fn record_declarations_parse_and_erased_fields_are_rejected() {
        let p = ok("module T where\n  record R where\n    x :: Int\n    y :: Bool\n");
        match &p.module.decls[0] {
            Decl::Record(r) => {
                assert_eq!(r.name.text, "R");
                let names: Vec<&str> = r.fields.iter().map(|f| f.name.text.as_str()).collect();
                assert_eq!(names, vec!["x", "y"]);
            }
            other => panic!("expected a record, got {other:?}"),
        }
        let e = err("module T where\n  record R where\n    erased x :: Int\n");
        assert!(e.contains("erasure is not implemented"), "{e}");
    }

    #[test]
    fn not_supported_terms_are_named() {
        let e = err("module T where\n  f = do\n    x\n");
        assert!(e.contains("do expressions are not supported"), "{e}");
        let e = err("module T where\n  f = g @{a = b}\n");
        assert!(e.contains("capability supplies"), "{e}");
        let e = err("module T where\n  f = [| x |]\n");
        assert!(e.contains("typed quotations"), "{e}");
        let e = err("module T where\n  f = 1.5\n");
        assert!(e.contains("float literals are not supported"), "{e}");
    }

    #[test]
    fn tuples_lists_projection_and_record_construction_parse() {
        let p = ok("module T where\n  f = (1, 2, 3)\n  g = [1, 2]\n  h = []\n  i = x.y\n  j = R { a = 1, b = 2 }\n  k = R {}\n");
        match eq_body(&p.module, "f") {
            Term::Tuple(items, _) => assert_eq!(items.len(), 3),
            other => panic!("expected a tuple, got {other:?}"),
        }
        match eq_body(&p.module, "g") {
            Term::List(items, _) => assert_eq!(items.len(), 2),
            other => panic!("expected a list, got {other:?}"),
        }
        match eq_body(&p.module, "h") {
            Term::List(items, _) => assert!(items.is_empty()),
            other => panic!("expected a list, got {other:?}"),
        }
        match eq_body(&p.module, "i") {
            Term::Proj(t, f, _) => {
                assert_eq!(f.text, "y");
                assert!(matches!(**t, Term::Var(_)));
            }
            other => panic!("expected a projection, got {other:?}"),
        }
        match eq_body(&p.module, "j") {
            Term::Record(path, fields, _) => {
                assert_eq!(path.parts(), vec!["R"]);
                let names: Vec<&str> = fields.iter().map(|(n, _)| n.text.as_str()).collect();
                assert_eq!(names, vec!["a", "b"]);
            }
            other => panic!("expected record construction, got {other:?}"),
        }
        match eq_body(&p.module, "k") {
            Term::Record(_, fields, _) => assert!(fields.is_empty()),
            other => panic!("expected record construction, got {other:?}"),
        }
    }

    #[test]
    fn tuple_types_parse() {
        let p = ok("module T where\n  f :: (Int, Bool, Int)\n  f = (1, True, 3)\n");
        match sig_ty(&p.module, "f") {
            Type::Tuple(items, _) => {
                assert_eq!(items.len(), 3);
                assert!(matches!(items[1], Type::Con(_, _)));
            }
            other => panic!("expected a tuple type, got {other:?}"),
        }
    }

    #[test]
    fn not_supported_patterns_and_module_forms() {
        let e = err("module T (M :: Sig) where\n  f = 1\n");
        assert!(e.contains("module parameters are not supported"), "{e}");
        let e = err("module T where\n  module U where\n    f = 1\n");
        assert!(e.contains("nested module declarations"), "{e}");
    }

    #[test]
    fn tuple_list_and_record_patterns_parse() {
        let p = ok(
            "module T where\n  f = case x of\n    (a, b) -> 1\n    [p, q] -> 2\n    [] -> 3\n    C { a = p, b = q } -> 4\n",
        );
        let alts = match eq_body(&p.module, "f") {
            Term::Case(_, alts, _) => alts.clone(),
            other => panic!("expected a case, got {other:?}"),
        };
        assert!(matches!(&alts[0].pat, Pat::Tuple(ps, _) if ps.len() == 2));
        assert!(matches!(&alts[1].pat, Pat::List(ps, _) if ps.len() == 2));
        assert!(matches!(&alts[2].pat, Pat::List(ps, _) if ps.is_empty()));
        match &alts[3].pat {
            Pat::Record(path, fields, _) => {
                assert_eq!(path.parts(), vec!["C"]);
                let names: Vec<&str> = fields.iter().map(|(n, _)| n.text.as_str()).collect();
                assert_eq!(names, vec!["a", "b"]);
            }
            other => panic!("expected a record pattern, got {other:?}"),
        }
    }

    #[test]
    fn parses_the_prelude() {
        let p = ok(include_str!("../prelude/Prelude.lang"));
        let m = &p.module;
        assert_eq!(m.name.text, "Prelude");
        let datas: Vec<&str> = m
            .decls
            .iter()
            .filter_map(|d| match d {
                Decl::Data(x) => Some(x.name.text.as_str()),
                _ => None,
            })
            .collect();
        // The prelude grows with the language; pin the essentials only.
        for required in ["Option", "List", "Ordering"] {
            assert!(datas.contains(&required), "missing data {required} in {datas:?}");
        }
        let foreigns = m.decls.iter().filter(|d| matches!(d, Decl::Foreign(_))).count();
        assert!(foreigns >= 17, "expected the primitive surface, got {foreigns}");
        let insts: Vec<&InstDecl> = m
            .decls
            .iter()
            .filter_map(|d| match d {
                Decl::Instance(x) => Some(x),
                _ => None,
            })
            .collect();
        assert!(insts.len() >= 7, "expected the class instances, got {}", insts.len());
        assert!(insts.iter().all(|i| i.implicit));
        assert!(
            insts.iter().any(|i| !i.methods.is_empty() && i.methods[0].name.text == "=="),
            "expected an Eq instance"
        );
        assert_eq!(p.fixities.get("+"), Some((Assoc::Left, 6)));
        assert_eq!(p.fixities.get("=="), Some((Assoc::None, 4)));
        assert_eq!(p.fixities.get("&&"), Some((Assoc::Right, 3)));
    }

    #[test]
    fn fixity_table_conflicts_and_shadowing() {
        let mut own = FixityTable::new();
        own.insert("+", Assoc::Left, 6).unwrap();
        own.insert("+", Assoc::Left, 6).unwrap();
        assert!(own.insert("+", Assoc::Right, 7).is_err());
        assert!(own.insert("%", Assoc::Left, 10).is_err());

        let mut imported = FixityTable::new();
        imported.insert("+", Assoc::Right, 9).unwrap();
        let merged = imported.merge(&own);
        assert_eq!(merged.get("+"), Some((Assoc::Left, 6)));
    }

    #[test]
    fn imported_fixities_apply_and_own_decls_shadow_them() {
        let mut imported = FixityTable::new();
        imported.insert("+", Assoc::Right, 9).unwrap();
        let toks = crate::lexer::lex("module T where\n  infixl 6 +\n  f = 1 + 2 + 3\n").unwrap();
        let toks = crate::layout::layout(toks).unwrap();
        let p = parse_module(&toks, &imported).unwrap();
        assert_eq!(p.fixities.get("+"), Some((Assoc::Left, 6)));
        // Shadowed to infixl 6: `1 + 2` groups first.
        let (f, args) = app(eq_body(&p.module, "f"));
        assert_eq!(var(f), "+");
        assert_eq!(int(&args[1]), 3);
    }
}

