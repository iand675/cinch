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

/// One parsed signature block item: an associated type member, a
/// method signature, or a class law declaration.
enum SigItem {
    Member(Decl),
    Method((Name, Type)),
    Law((Name, Type)),
}

/// The declared name of a signature's type member.
fn sig_member_name(d: &Decl) -> Name {
    match d {
        Decl::OpaqueType(o) => o.name.clone(),
        Decl::Data(t) => t.name.clone(),
        Decl::Record(r) | Decl::Codata(r) => r.name.clone(),
        Decl::Signature(s) => s.name.clone(),
        Decl::Attributed { inner, .. } => sig_member_name(inner),
        other => {
            let _ = other;
            Name::new("_", Span { line: 0, col: 0 })
        }
    }
}

/// A parsed module together with the fixity table for parsing the next one.
#[derive(Debug)]
pub struct ParsedModule {
    pub module: Module,
    pub fixities: FixityTable,
}

/// A parsed source file: one or more modules (the staging slice allows any
/// number of `module` blocks per file), plus each module's merged fixity
/// table.
#[derive(Debug)]
pub struct ParsedModules {
    pub modules: Vec<Module>,
    /// Per-module fixity tables (imported declarations plus the module's
    /// own), keyed by module name.
    pub fixities: std::collections::HashMap<String, FixityTable>,
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

/// Parse a whole source file: `program = { module_decl }` (the staging
/// slice supersedes the one-module-per-file rule). Each module resolves its
/// infix chains with `base` plus the fixities of the modules it imports
/// (`tables`) plus its own; modules of the same file see each other's
/// fixities when they import each other.
pub fn parse_modules(
    tokens: &[Token],
    base: &FixityTable,
    tables: &std::collections::HashMap<String, FixityTable>,
) -> Result<ParsedModules> {
    let mut p = Parser::new(tokens);
    let mut modules: Vec<(Module, FixityTable)> = Vec::new();
    while !p.at(&Tok::Eof) {
        let module = p.parse_module_decl()?;
        let own = std::mem::take(&mut p.own_fixities);
        modules.push((module, own));
    }
    let mut out = Vec::new();
    let mut fixities = std::collections::HashMap::new();
    for (module, own) in &modules {
        let mut module = module.clone();
        let mut table = base.clone();
        for imp in &module.imports {
            // Fixity tables key on the import's full dotted name.
            let name = imp.path.parts().join(".");
            if let Some(t) = modules.iter().find(|(m, _)| m.name.text == name).map(|(_, o)| o) {
                table = table.merge(t);
            } else if let Some(t) = tables.get(&name) {
                table = table.merge(t);
            }
        }
        table = table.merge(own);
        resolve_module(&mut module, &table)?;
        fixities.insert(module.name.text.clone(), table);
        out.push(module);
    }
    Ok(ParsedModules { modules: out, fixities })
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
        Tok::SymLit(s) => format!("symbol `'{s}`"),
        Tok::TypeNameLit(s) => format!("type name `''{s}`"),
        Tok::Fragment(_) => "an inline target-IR fragment".into(),
        Tok::Tilde => "`~`".into(),
        Tok::DotDot => "`..`".into(),
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
        // Module header attributes (attribute annotations specification
        // section 10) sit in prefix position before the `module`
        // keyword.
        let attrs = self.parse_attr_list()?;
        let start = self.expect(&Tok::Module, "`module`")?.span;
        let path = self.parse_con_path()?;
        // `module M (x :: T, sig :: S, ...) where`: the parameter block of
        // modules specification section 1 (module parameters and
        // signatures as module arguments). Each parameter is a named
        // binding of the stated type; a `signature`-typed parameter is a
        // dictionary supplied at instantiation.
        let mut params = Vec::new();
        if self.at(&Tok::LParen) {
            self.bump();
            loop {
                let pspan = self.peek().span;
                let pname = match self.peek_tok().clone() {
                    Tok::VarId(s) | Tok::ConId(s) if s != "_" => {
                        self.bump();
                        Name::new(s, pspan)
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a module parameter name, found {}",
                            describe(&t)
                        )))
                    }
                };
                self.expect(&Tok::ColonColon, "`::` in a module parameter")?;
                let ty = self.parse_type()?;
                if params.iter().any(|p: &ParamDecl| p.name.text == pname.text) {
                    return Err(Error::at(pspan, format!("duplicate module parameter {}", pname.text)));
                }
                params.push(ParamDecl { name: pname, ty, span: pspan });
                if self.at(&Tok::Comma) {
                    self.bump();
                } else {
                    break;
                }
            }
            self.expect(&Tok::RParen, "`)` closing the module parameter list")?;
        }
        if self.at(&Tok::ColonColon) {
            return Err(self.err("module signatures are not supported in the bootstrap slice"));
        }
        self.expect(&Tok::Where, "`where`")?;
        let decls = self.parse_block_items("the declaration block", |p| p.parse_declaration())?;
        // Signature type members are declared ahead of the signature so
        // its method types resolve them (decision D65).
        let decls = crate::ast::splice_sig_members(decls);
        let mut imports = Vec::new();
        for d in &decls {
            if let Decl::Import(i) = d {
                imports.push(i.clone());
            }
        }
        let name = Name::new(path.parts().join("."), start);
        Ok(Module { attrs, name, params, imports, decls, span: start })
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

    /// The `#[...]` attributes prefixing an annotated construct
    /// (attribute annotations specification section 1.1). Layout inserts
    /// a virtual `;` at a new line's start, so an attribute prefixing its
    /// construct across lines is preceded by one; those separators are
    /// skipped here. The list is empty when no attribute is written.
    fn parse_attr_list(&mut self) -> Result<Vec<Attribute>> {
        let mut attrs = Vec::new();
        loop {
            // An attribute prefixes its construct across lines too, but
            // only when an attribute actually follows the separator.
            if self.at(&Tok::Semicolon)
                && self.peek().virtual_
                && matches!(self.peek_tok_at(1), Some(Tok::Attr { .. }))
            {
                self.bump();
            }
            let span = self.peek().span;
            match self.peek_tok().clone() {
                Tok::Attr { name, args } => {
                    self.bump();
                    attrs.push(Attribute {
                        path: name
                            .split('.')
                            .map(|s| Name::new(s, span))
                            .collect(),
                        args,
                        span,
                    });
                }
                _ => break,
            }
        }
        if !attrs.is_empty() {
            // The annotated construct follows: drop any layout
            // separators between it and its attributes.
            while self.at(&Tok::Semicolon) && self.peek().virtual_ {
                self.bump();
            }
        }
        Ok(attrs)
    }

    fn parse_declaration(&mut self) -> Result<Decl> {
        // `#[...]` attributes prefix the annotated declaration (attribute
        // annotations specification section 1.1).
        if matches!(self.peek_tok(), Tok::Attr { .. })
            || (self.at(&Tok::Semicolon)
                && self.peek().virtual_
                && matches!(self.peek_tok_at(1), Some(Tok::Attr { .. })))
        {
            let attrs = self.parse_attr_list()?;
            let inner = self.parse_declaration()?;
            return Ok(Decl::Attributed { attrs, inner: Box::new(inner) });
        }
        match self.peek_tok().clone() {
            Tok::Import => {
                if matches!(self.peek_tok_at(1), Some(Tok::Data)) {
                    Ok(Decl::ImportData(self.parse_import_data()?))
                } else {
                    Ok(Decl::Import(self.parse_import()?))
                }
            }
            Tok::Export => Ok(Decl::Export(self.parse_export()?)),
            Tok::Signature => Ok(Decl::Signature(self.parse_signature()?)),
            Tok::Instance | Tok::Implicit => Ok(Decl::Instance(self.parse_instance()?)),
            Tok::Data => Ok(Decl::Data(self.parse_data()?)),
            Tok::Foreign => Ok(Decl::Foreign(self.parse_foreign()?)),
            Tok::Infix | Tok::Infixl | Tok::Infixr => Ok(Decl::Fixity(self.parse_fixity()?)),
            Tok::Record => Ok(Decl::Record(self.parse_record()?)),
            Tok::Codata => Ok(Decl::Codata(self.parse_codata()?)),
            Tok::Effect => Ok(Decl::Effect(self.parse_effect_decl()?)),
            Tok::Generate => self.parse_generate(),
            Tok::Opaque => Ok(Decl::OpaqueType(self.parse_opaque_type()?)),
            Tok::TypeKw => Ok(Decl::Refine(self.parse_refine()?)),
            Tok::Rewrite => self.parse_rewrite(false),
            Tok::Prim => self.parse_prim(false),
            Tok::VarId(s) if s == "unsafe" && matches!(self.peek_tok_at(1), Some(Tok::Prim)) => {
                self.bump();
                self.parse_prim(true)
            }
            Tok::Target => {
                self.bump();
                self.parse_rewrite(true)
            }
            Tok::Upgrade => self.parse_upgrade(),
            Tok::Contract => self.parse_contract(),
            Tok::QuoteDecls => Err(self.err(
                "`quoteDecls where ...` is an expression; use `generate do ... emit (quoteDecls where ...)` to emit declarations",
            )),
            Tok::Module => {
                Err(self.err("nested module declarations are not supported in the bootstrap slice"))
            }
            Tok::VarId(_) | Tok::LParen | Tok::DollarParen => self.parse_sig_or_equation(),
            Tok::Dollar if self.at_splice() => self.parse_sig_or_equation(),
            t if modifier_name(&t).is_some() => self.parse_sig_or_equation(),
            t => Err(self.err(format!("expected a declaration, found {}", describe(&t)))),
        }
    }

    /// `generate do { stmts }`: a compile-time generator (modules
    /// specification section 10).
    fn parse_generate(&mut self) -> Result<Decl> {
        let span = self.expect(&Tok::Generate, "`generate`")?.span;
        let stmts = self.parse_do_stmts()?;
        Ok(Decl::Generate { stmts, span })
    }

    /// `rewrite name forall binders. [using (ctx)] where { match e;
    /// replace e; [require e]; justify e }` and the `target rewrite` form
    /// (modules and compilation specification section 5; platform proofs
    /// and SIMD specification section 3). The clause keywords are the
    /// lexer's reserved words, so the clauses parse in any order but each
    /// appears at most once.
    fn parse_rewrite(&mut self, target: bool) -> Result<Decl> {
        let span = self.expect(&Tok::Rewrite, "`rewrite`")?.span;
        let name = self.parse_value_name()?;
        self.expect(&Tok::Forall, "`forall` binding the rule's pattern variables")?;
        let mut binders = vec![self.parse_type_binder()?];
        loop {
            match self.peek_tok() {
                Tok::VarId(s) if s != "_" => binders.push(self.parse_type_binder()?),
                Tok::LParen | Tok::LBrace => binders.push(self.parse_type_binder()?),
                _ => break,
            }
        }
        self.expect(&Tok::Dot, "`.` after the `forall` binders")?;
        let ctx = if self.at(&Tok::Using) {
            self.bump();
            self.expect(&Tok::LParen, "`(` after `using`")?;
            let cs = self.parse_constraint_items()?;
            self.expect(&Tok::RParen, "`)` after the constraint list")?;
            cs
        } else {
            Vec::new()
        };
        self.expect(&Tok::Where, "`where` opening the rewrite's clauses")?;
        let mut match_tm: Option<Term> = None;
        let mut replace_tm: Option<Term> = None;
        let mut require: Option<Term> = None;
        let mut justify: Option<Term> = None;
        let clauses = self.parse_block_items("the rewrite's clauses", |p| {
            let cspan = p.peek().span;
            let slot = match p.peek_tok() {
                Tok::MatchKw => 0,
                Tok::Replace => 1,
                Tok::Require => 2,
                Tok::Justify => 3,
                t => {
                    return Err(p.err(format!(
                        "expected a rewrite clause (`match`, `replace`, `require`, or `justify`), found {}",
                        describe(t)
                    )))
                }
            };
            p.bump();
            Ok((slot, cspan, p.parse_term()?))
        })?;
        for (slot, cspan, tm) in clauses {
            let (slot_name, cell) = match slot {
                0 => ("match", &mut match_tm),
                1 => ("replace", &mut replace_tm),
                2 => ("require", &mut require),
                _ => ("justify", &mut justify),
            };
            if cell.is_some() {
                return Err(Error::at(cspan, format!("duplicate `{slot_name}` clause in a rewrite declaration")));
            }
            *cell = Some(tm);
        }
        let missing = |what: &str| Error::at(span, format!("a rewrite declaration needs a `{what}` clause"));
        Ok(Decl::Rewrite(RewriteDecl {
            target,
            name,
            binders,
            ctx,
            match_tm: match_tm.ok_or_else(|| missing("match"))?,
            replace_tm: replace_tm.ok_or_else(|| missing("replace"))?,
            require,
            justify: justify.ok_or_else(|| missing("justify"))?,
            span,
        }))
    }

    /// `prim name :: type where { target_body ... }` and its bodies
    /// `family { fragment } [when cond] [justify e] [uses ...] [immarg
    /// ...]` (primitive operations and inline target IR specification
    /// section 2). The fragment is captured verbatim by the lexer (one
    /// `Fragment` token between real braces); the clause keywords are
    /// contextual except `justify`, a reserved word like the rewrite
    /// clause's.
    fn parse_prim(&mut self, all_unsafe: bool) -> Result<Decl> {
        let span = self.expect(&Tok::Prim, "`prim`")?.span;
        let name = self.parse_value_name()?;
        self.expect(&Tok::ColonColon, "`::` after the prim's name")?;
        let ty = self.parse_type()?;
        self.expect(&Tok::Where, "`where` opening the prim's target-IR bodies")?;
        let mut bodies =
            self.parse_block_items("the prim's target-IR bodies", |p| p.parse_prim_body())?;
        // `unsafe prim ...` marks every body (the per-body `unsafe`
        // spelling marks one): containment cannot verify a body so
        // marked, and its declared row is trusted, not verified
        // (primitive operations and inline target IR specification
        // section 4).
        if all_unsafe {
            for b in &mut bodies {
                b.unverified = true;
            }
        }
        if bodies.is_empty() {
            return Err(Error::at(
                span,
                "a prim declaration needs at least one target-IR body",
            ));
        }
        Ok(Decl::Prim(PrimDecl { name, ty, bodies, span }))
    }

    /// One target-IR body: `family { fragment }` followed by its
    /// clauses in any order, each at most once.
    fn parse_prim_body(&mut self) -> Result<PrimBody> {
        let span = self.peek().span;
        let unverified = match self.peek_tok() {
            Tok::VarId(s) if s == "unsafe" => {
                self.bump();
                true
            }
            _ => false,
        };
        let family = match self.peek_tok().clone() {
            Tok::VarId(s) => {
                self.bump();
                Name::new(s, span)
            }
            t => {
                return Err(self.err(format!(
                    "expected a target-IR family name (`llvm`, `wasm`, `asm`, `clr` or `jvm`), found {}",
                    describe(&t)
                )))
            }
        };
        self.expect(&Tok::LBrace, "`{` opening the body's inline fragment")?;
        let (fragment, fragment_span) = match self.peek_tok().clone() {
            Tok::Fragment(text) => {
                let sp = self.peek().span;
                self.bump();
                (text, sp)
            }
            t => {
                return Err(self.err(format!(
                    "expected the body's inline fragment, found {}",
                    describe(&t)
                )))
            }
        };
        self.expect(&Tok::RBrace, "`}` closing the body's inline fragment")?;
        let mut when: Option<Term> = None;
        let mut justify: Option<Term> = None;
        let mut uses: Vec<PrimUse> = Vec::new();
        let mut immarg: Vec<(u64, Span)> = Vec::new();
        loop {
            let cspan = self.peek().span;
            let slot = match self.peek_tok() {
                Tok::VarId(s) if s == "when" => 0,
                Tok::Justify => 1,
                Tok::VarId(s) if s == "uses" => 2,
                Tok::VarId(s) if s == "immarg" => 3,
                _ => break,
            };
            self.bump();
            let dup = |name: &str| {
                Error::at(cspan, format!("duplicate `{name}` clause in a target-IR body"))
            };
            match slot {
                0 => {
                    if when.is_some() {
                        return Err(dup("when"));
                    }
                    when = Some(self.parse_term()?);
                }
                1 => {
                    if justify.is_some() {
                        return Err(dup("justify"));
                    }
                    justify = Some(self.parse_term()?);
                }
                2 => {
                    if !uses.is_empty() {
                        return Err(dup("uses"));
                    }
                    loop {
                        let uspan = self.peek().span;
                        self.expect(&Tok::LParen, "`(` opening a `uses` entry")?;
                        let uname = self.parse_value_name()?;
                        self.expect(&Tok::ColonColon, "`::` in a `uses` entry")?;
                        let uty = self.parse_type()?;
                        self.expect(&Tok::RParen, "`)` closing the `uses` entry")?;
                        uses.push(PrimUse { name: uname, ty: uty, span: uspan });
                        if self.at(&Tok::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                _ => {
                    if !immarg.is_empty() {
                        return Err(dup("immarg"));
                    }
                    loop {
                        let ispan = self.peek().span;
                        match self.peek_tok().clone() {
                            Tok::Int(v) => {
                                self.bump();
                                let n = v.parse::<u64>().map_err(|_| {
                                    Error::at(ispan, "an `immarg` index is a non-negative integer")
                                })?;
                                immarg.push((n, ispan));
                            }
                            t => {
                                return Err(self.err(format!(
                                    "expected an immediate parameter index, found {}",
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
                }
            }
        }
        Ok(PrimBody {
            family,
            fragment,
            fragment_span,
            when,
            justify,
            uses,
            immarg,
            unverified,
            span,
        })
    }

    /// `upgrade Old to New where { migrate = e; [related = e] }` and its
    /// migration-free record-view form (runtime evolution specification
    /// sections 2-3). The assignment block maps the reserved component
    /// names `migrate` and `related` to their terms.
    fn parse_upgrade(&mut self) -> Result<Decl> {
        let span = self.expect(&Tok::Upgrade, "`upgrade`")?.span;
        let old = self.parse_con_path()?;
        self.expect(&Tok::To, "`to` between the old and new artifacts")?;
        let new = self.parse_con_path()?;
        self.expect(&Tok::Where, "`where` opening the upgrade's components")?;
        let mut migrate: Option<Term> = None;
        let mut related: Option<Term> = None;
        let items = self.parse_block_items("the upgrade's components", |p| {
            let name_span = p.peek().span;
            let name = match p.peek_tok().clone() {
                Tok::VarId(s) | Tok::ConId(s) => {
                    p.bump();
                    s
                }
                t => {
                    return Err(p.err(format!(
                        "expected an upgrade component (`migrate` or `related`), found {}",
                        describe(&t)
                    )))
                }
            };
            p.expect(&Tok::Equals, "`=` after an upgrade component name")?;
            let tm = p.parse_term()?;
            Ok((name, name_span, tm))
        })?;
        for (name, name_span, tm) in items {
            match name.as_str() {
                "migrate" => {
                    if migrate.is_some() {
                        return Err(Error::at(name_span, "duplicate `migrate` component in an upgrade"));
                    }
                    migrate = Some(tm);
                }
                "related" => {
                    if related.is_some() {
                        return Err(Error::at(name_span, "duplicate `related` component in an upgrade"));
                    }
                    related = Some(tm);
                }
                other => {
                    return Err(Error::at(
                        name_span,
                        format!(
                            "`{other}` is not an upgrade component; expected `migrate` or `related`"
                        ),
                    ))
                }
            }
        }
        Ok(Decl::Upgrade(UpgradeDecl {
            old,
            new,
            migrate,
            related,
            span,
        }))
    }

    /// `contract <kind> <name> for <def> where { <op> = (P); assume = (P)
    /// | "text"; ... }`: a temporal contract over a definition (core
    /// specification section 10). The kinds are `safety`, `liveness`,
    /// `recurrence` and `stability`; each states exactly one formula,
    /// whose operator matches the kind (`always`, `eventually`, `recur`,
    /// `stable`), plus the assumptions it relies upon (`assume`). The
    /// clause vocabulary is contextual, not reserved.
    fn parse_contract(&mut self) -> Result<Decl> {
        let span = self.expect(&Tok::Contract, "`contract`")?.span;
        let kind_span = self.peek().span;
        let kind = match self.peek_tok().clone() {
            Tok::VarId(s) => match s.as_str() {
                "safety" => ContractKind::Safety,
                "liveness" => ContractKind::Liveness,
                "recurrence" => ContractKind::Recurrence,
                "stability" => ContractKind::Stability,
                other => {
                    return Err(Error::at(
                        kind_span,
                        format!(
                            "`{other}` is not a temporal contract kind; expected `safety`, `liveness`, `recurrence` or `stability` (core specification section 10)"
                        ),
                    ))
                }
            },
            t => {
                return Err(self.err(format!(
                    "expected a temporal contract kind (`safety`, `liveness`, `recurrence` or `stability`), found {}",
                    describe(&t)
                )))
            }
        };
        self.bump();
        let name = self.parse_value_name()?;
        // The `for` of `contract ... for <def>` (contextual).
        match self.peek_tok().clone() {
            Tok::VarId(s) if s == "for" => {
                self.bump();
            }
            t => {
                return Err(self.err(format!(
                    "expected `for` naming the contract's definition, found {}",
                    describe(&t)
                )))
            }
        }
        let target = self.parse_value_name()?;
        self.expect(&Tok::Where, "`where` opening the contract's clauses")?;
        let mut formula: Option<ContractClause> = None;
        let mut assumes: Vec<ContractClause> = Vec::new();
        let items = self.parse_block_items("the contract's clauses", |p| {
            let name_span = p.peek().span;
            let name = match p.peek_tok().clone() {
                Tok::VarId(s) => {
                    p.bump();
                    s
                }
                t => {
                    return Err(p.err(format!(
                        "expected a contract clause (`always`, `eventually`, `recur`, `stable` or `assume`), found {}",
                        describe(&t)
                    )))
                }
            };
            p.expect(&Tok::Equals, "`=` after a contract clause name")?;
            match name.as_str() {
                "always" | "eventually" | "recur" | "stable" => {
                    let body = p.parse_paren_term("the contract's formula")?;
                    Ok(ContractClause::Formula {
                        op: Name::new(name, name_span),
                        body,
                    })
                }
                "assume" => match p.peek_tok().clone() {
                    Tok::StrLit(text) => {
                        let s = p.peek().span;
                        p.bump();
                        Ok(ContractClause::AssumeNamed { text, span: s })
                    }
                    _ => Ok(ContractClause::AssumeProp {
                        body: p.parse_paren_term("an assumed fact")?,
                    }),
                },
                other => Err(Error::at(
                    name_span,
                    format!(
                        "`{other}` is not a contract clause; expected `always`, `eventually`, `recur`, `stable` or `assume` (core specification section 10)"
                    ),
                )),
            }
        })?;
        for item in items {
            match item {
                ContractClause::Formula { ref op, .. } if formula.is_some() => {
                    return Err(Error::at(
                        op.span,
                        format!(
                            "duplicate `{}` formula: a temporal contract states exactly one formula (core specification section 10)",
                            op.text
                        ),
                    ))
                }
                f @ ContractClause::Formula { .. } => formula = Some(f),
                assume => assumes.push(assume),
            }
        }
        let Some(ContractClause::Formula { ref op, .. }) = formula else {
            return Err(Error::at(
                span,
                format!(
                    "a `{}` temporal contract states its formula: `{} = (P)` (core specification section 10)",
                    kind.keyword(),
                    kind.operator()
                ),
            ));
        };
        if op.text != kind.operator() {
            return Err(Error::at(
                op.span,
                format!(
                    "a `{}` temporal contract states `{} = (P)`, not `{} = (...)` (core specification section 10: the language distinguishes safety, liveness, recurrence and stability)",
                    kind.keyword(),
                    kind.operator(),
                    op.text
                ),
            ));
        }
        let mut clauses = Vec::new();
        clauses.extend(formula);
        clauses.append(&mut assumes);
        Ok(Decl::Contract(ContractDecl {
            kind,
            name,
            target,
            clauses,
            span,
        }))
    }

    /// A parenthesized term clause (`= (e)`).
    fn parse_paren_term(&mut self, what: &str) -> Result<Term> {
        self.expect(&Tok::LParen, &format!("`(` opening {what}"))?;
        let t = self.parse_term()?;
        self.expect(&Tok::RParen, &format!("`)` closing {what}"))?;
        Ok(t)
    }

    fn parse_sig_or_equation(&mut self) -> Result<Decl> {
        let modifier = self.parse_modifier()?;
        // A `quoteDecls` name position may splice the declaration's name
        // (`$( e ) :: ty`, `$( e ) pats = body`; D46 name splices).
        let name_tm = if self.at(&Tok::DollarParen) || (self.at(&Tok::Dollar) && self.at_splice()) {
            Some(self.parse_splice_term()?)
        } else {
            None
        };
        let name = match &name_tm {
            Some(t) => Name::new("", t.span()),
            None => self.parse_value_name()?,
        };
        // A macro signature may declare its owned attribute names: `macro
        // name attributes (codec, table) :: ty` (attribute annotations
        // specification section 6).
        let mut owned_attrs = Vec::new();
        if modifier.as_ref().map(|m| m.text.as_str()) == Some("macro") {
            if let Tok::VarId(s) = self.peek_tok() {
                if s == "attributes" && matches!(self.peek_tok_at(1), Some(Tok::LParen)) {
                    self.bump();
                    self.bump();
                    loop {
                        let span = self.peek().span;
                        match self.peek_tok().clone() {
                            Tok::VarId(n) | Tok::ConId(n) => {
                                self.bump();
                                owned_attrs.push(Name::new(n, span));
                            }
                            t => {
                                return Err(self.err(format!(
                                    "expected an attribute name in the macro's `attributes` list, found {}",
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
                    self.expect(&Tok::RParen, "`)` closing the `attributes` list")?;
                }
            }
        }
        if self.at(&Tok::ColonColon) {
            self.bump();
            let ty = self.parse_type()?;
            let span = name.span;
            let decl = Decl::ValueSig { name, modifier, owned_attrs, ty, span };
            Ok(match name_tm {
                Some(t) => Decl::NameSpliced { name_tm: Box::new(t), inner: Box::new(decl) },
                None => decl,
            })
        } else {
            if !owned_attrs.is_empty() {
                return Err(self.err(
                    "a macro's `attributes` list belongs on its signature (`macro name attributes (...) :: ty`)",
                ));
            }
            let eq = self.parse_equation_rest(name, true)?;
            let decl = Decl::Equation { modifier, eq };
            Ok(match name_tm {
                Some(t) => Decl::NameSpliced { name_tm: Box::new(t), inner: Box::new(decl) },
                None => decl,
            })
        }
    }

    /// `$( e )` in a declaration's name position: the splice body that
    /// names the generated declaration at splice time.
    fn parse_splice_term(&mut self) -> Result<Term> {
        let span = self.peek().span;
        if self.at(&Tok::DollarParen) {
            self.bump();
            let body = self.parse_term()?;
            self.expect(&Tok::RParen, "`)` closing the splice")?;
            Ok(Term::Splice { body: Box::new(body), span })
        } else {
            self.bump();
            self.bump();
            let body = self.parse_term()?;
            self.expect(&Tok::RParen, "`)` closing the splice")?;
            Ok(Term::Splice { body: Box::new(body), span })
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
                    // A bare `|` in operator-name position is the
                    // operator `|` (surface grammar local amendments
                    // section 6: the contextual Pipe rule).
                    Tok::Pipe => "|".to_string(),
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

    /// `name [clause attrs] pats = body [where locals]`: the clause
    /// attributes sit between the function name and its first pattern
    /// (attribute annotations specification section 1.1).
    fn parse_equation_rest(&mut self, name: Name, allow_where: bool) -> Result<Equation> {
        let span = name.span;
        let attrs = self.parse_attr_list()?;
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
        Ok(Equation { attrs, name, pats, body, locals, span })
    }

    fn parse_import(&mut self) -> Result<Import> {
        let span = self.expect(&Tok::Import, "`import`")?.span;
        // Import modifiers exist in the grammar; `stage` marks an import of
        // executable generator code (modules specification section 9).
        // `replaceable`/`specialized` are accepted here and the checker
        // judges the import itself.
        let mut stage = false;
        if self.at(&Tok::Stage) {
            self.bump();
            stage = true;
        } else if let Tok::VarId(s) = self.peek_tok() {
            if s == "replaceable" || s == "specialized" {
                self.bump();
            }
        }
        let path = self.parse_con_path()?;
        // `(...)` is the module argument list when a unit alias follows
        // (`import M (arg, ...) as N`, module parameters as module
        // arguments), and the selected member list otherwise (`import X
        // (a as b, c)`, surface grammar local amendments section 11).
        let mut args = Vec::new();
        let mut members = Vec::new();
        if self.at(&Tok::LParen) {
            let mut depth = 0usize;
            let mut k = 0usize;
            let mut inner_as = false;
            loop {
                match self.peek_tok_at(k) {
                    Some(Tok::LParen) => depth += 1,
                    Some(Tok::RParen) => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    Some(Tok::As) if depth == 1 => inner_as = true,
                    Some(Tok::Eof) | None => break,
                    _ => {}
                }
                k += 1;
            }
            let applied = matches!(self.peek_tok_at(k + 1), Some(Tok::As));
            if applied && inner_as {
                return Err(Error::at(
                    span,
                    "a member selection and a module alias cannot combine: `import X (a as b)` selects members, `import M (args) as N` instantiates",
                ));
            }
            if applied {
                self.bump();
                loop {
                    args.push(self.parse_module_arg()?);
                    if self.at(&Tok::Comma) {
                        self.bump();
                    } else {
                        break;
                    }
                }
                self.expect(&Tok::RParen, "`)` closing the module argument list")?;
            } else {
                self.bump();
                loop {
                    let nspan = self.peek().span;
                    let name = match self.peek_tok().clone() {
                        Tok::VarId(s) | Tok::ConId(s) => {
                            self.bump();
                            Name::new(s, nspan)
                        }
                        t => {
                            return Err(self.err(format!(
                                "expected a member name, found {}",
                                describe(&t)
                            )))
                        }
                    };
                    let as_name = if self.at(&Tok::As) {
                        self.bump();
                        let sspan = self.peek().span;
                        match self.peek_tok().clone() {
                            Tok::VarId(s) | Tok::ConId(s) => {
                                self.bump();
                                Some(Name::new(s, sspan))
                            }
                            t => {
                                return Err(self.err(format!(
                                    "expected a member name after `as`, found {}",
                                    describe(&t)
                                )))
                            }
                        }
                    } else {
                        None
                    };
                    if members
                        .iter()
                        .any(|m: &ImportMember| m.name.text == name.text)
                    {
                        return Err(Error::at(
                            nspan,
                            format!("duplicate member selection for {}", name.text),
                        ));
                    }
                    members.push(ImportMember { name, as_name });
                    if self.at(&Tok::Comma) {
                        self.bump();
                    } else {
                        break;
                    }
                }
                self.expect(&Tok::RParen, "`)` closing the member list")?;
            }
        }
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
        if !args.is_empty() {
            if stage {
                return Err(Error::at(
                    span,
                    "`import stage` takes no module arguments: compile-time code of a parameterized module is not supported in the bootstrap slice",
                ));
            }
            if alias.is_none() {
                return Err(Error::at(
                    span,
                    "an applied import names its instantiation unit: `import M (args) as N`",
                ));
            }
        }
        if !members.is_empty() && alias.is_some() {
            return Err(Error::at(
                span,
                "a member selection and a module alias cannot combine: `import X (a as b)` selects members under their imported names",
            ));
        }
        Ok(Import { path, alias, stage, args, members, span })
    }

    /// `import data "config.json" as config` (surface grammar local
    /// amendments section 11): the converted value of a non-source file,
    /// bound under its name; the file's extension selects the converter.
    fn parse_import_data(&mut self) -> Result<ImportData> {
        let span = self.expect(&Tok::Import, "`import`")?.span;
        self.expect(&Tok::Data, "`data` in a file import")?;
        let fspan = self.peek().span;
        let file = match self.peek_tok().clone() {
            Tok::StrLit(s) => {
                self.bump();
                s
            }
            t => {
                return Err(self.err(format!(
                    "expected the file path as a string literal, found {}",
                    describe(&t)
                )))
            }
        };
        self.expect(&Tok::As, "`as` naming the imported value")?;
        let nspan = self.peek().span;
        let name = match self.peek_tok().clone() {
            Tok::VarId(s) => {
                self.bump();
                Name::new(s, nspan)
            }
            t => {
                return Err(self.err(format!(
                    "expected the imported value's name, found {}",
                    describe(&t)
                )))
            }
        };
        Ok(ImportData { file, name, span: fspan })
    }

    /// `export name, ...` (surface grammar local amendments section 11):
    /// the module's export surface when present. An item is a name in
    /// scope (`foo`, `MyType`), an imported member (`A.foo`), or a whole
    /// import (`module A`).
    fn parse_export(&mut self) -> Result<ExportDecl> {
        let span = self.expect(&Tok::Export, "`export`")?.span;
        let mut items = Vec::new();
        loop {
            items.push(self.parse_export_item()?);
            if self.at(&Tok::Comma) {
                self.bump();
            } else {
                break;
            }
        }
        Ok(ExportDecl { items, span })
    }

    fn parse_export_item(&mut self) -> Result<ExportItem> {
        let span = self.peek().span;
        if self.at(&Tok::Module) {
            self.bump();
            let mut segments: Vec<String> = Vec::new();
            loop {
                match self.peek_tok().clone() {
                    Tok::ConId(s) => {
                        self.bump();
                        segments.push(s);
                    }
                    t => {
                        return Err(self.err(format!(
                            "expected a module name after `export module`, found {}",
                            describe(&t)
                        )))
                    }
                }
                if self.at(&Tok::Dot) {
                    self.bump();
                } else {
                    break;
                }
            }
            return Ok(ExportItem::Module(Name::new(segments.join("."), span)));
        }
        // An item name is a value or type name, or an operator in
        // parentheses (`(==)`: the method names of the operator classes
        // are surface names like any other, decision D2).
        let item_name = |p: &mut Self| -> Result<Name> {
            let nspan = p.peek().span;
            match p.peek_tok().clone() {
                Tok::VarId(s) | Tok::ConId(s) => {
                    p.bump();
                    Ok(Name::new(s, nspan))
                }
                Tok::LParen => {
                    p.bump();
                    let op = match p.peek_tok().clone() {
                        Tok::Op(s) => s,
                        // The contextual Pipe rule (surface grammar
                        // local amendments section 6): a bare `|` in
                        // operator-name position is the operator `|`.
                        Tok::Pipe => "|".to_string(),
                        t => {
                            return Err(p.err(format!(
                                "expected an operator name in parentheses, found {}",
                                describe(&t)
                            )))
                        }
                    };
                    p.bump();
                    p.expect(&Tok::RParen, "`)`")?;
                    Ok(Name::new(op, nspan))
                }
                t => Err(p.err(format!(
                    "expected an exported name, found {}",
                    describe(&t)
                ))),
            }
        };
        let mut segments = vec![item_name(self)?];
        while self.at(&Tok::Dot) {
            self.bump();
            segments.push(item_name(self)?);
        }
        if segments.len() == 1 {
            Ok(ExportItem::Name(segments.remove(0)))
        } else {
            Ok(ExportItem::Member(ConPath { segments }))
        }
    }

    /// One module argument: a static name (an `instance` dictionary or a
    /// top-level value) or a literal (module parameters as module
    /// arguments; the arguments are static so the instantiation is a
    /// compilation unit, not a runtime application).
    fn parse_module_arg(&mut self) -> Result<ModuleArg> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::Int(v) => {
                self.bump();
                Ok(ModuleArg::Int(v, span))
            }
            Tok::Op(s) if s == "-" && matches!(self.peek_tok_at(1), Some(Tok::Int(_))) => {
                self.bump();
                match self.peek_tok().clone() {
                    Tok::Int(v) => {
                        self.bump();
                        Ok(ModuleArg::Int(format!("-{v}"), span))
                    }
                    _ => unreachable!(),
                }
            }
            Tok::CharLit(c) => {
                self.bump();
                Ok(ModuleArg::Char(c, span))
            }
            Tok::ConId(_) => {
                let path = self.parse_con_path()?;
                let mut quals: Vec<Name> = Vec::new();
                for p in &path.segments[..path.segments.len().saturating_sub(1)] {
                    quals.push(p.clone());
                }
                let name = path
                    .segments
                    .last()
                    .cloned()
                    .expect("a con path has a name");
                Ok(ModuleArg::Name { quals, name })
            }
            Tok::VarId(s) if s != "_" => {
                self.bump();
                Ok(ModuleArg::Name {
                    quals: Vec::new(),
                    name: Name::new(s, span),
                })
            }
            Tok::LParen => {
                self.bump();
                self.expect(&Tok::RParen, "`)` for a unit argument")?;
                Ok(ModuleArg::Unit(span))
            }
            t => Err(self.err(format!(
                "a module argument is a static name or literal, found {}",
                describe(&t)
            ))),
        }
    }

    fn parse_signature(&mut self) -> Result<SigDecl> {
        let span = self.expect(&Tok::Signature, "`signature`")?.span;
        let name = self.parse_ctor_name("a signature name")?;
        let params = self.parse_params()?;
        self.expect(&Tok::Where, "`where`")?;
        let items = self.parse_block_items("the signature block", |p| p.parse_sig_item())?;
        let mut type_members = Vec::new();
        let mut member_decls = Vec::new();
        let mut methods = Vec::new();
        let mut laws = Vec::new();
        for it in items {
            match it {
                SigItem::Member(d) => {
                    type_members.push(sig_member_name(&d));
                    member_decls.push(d);
                }
                SigItem::Method(m) => methods.push(m),
                SigItem::Law(l) => laws.push(l),
            }
        }
        Ok(SigDecl { name, params, type_members, member_decls, methods, laws, span })
    }

    /// One signature block item: an associated type member (`opaque type`
    /// or `record`), a method signature (modules and compilation
    /// specification section 1; foreign function interface specification
    /// section 2), or a class `law` declaration (primitive operations and
    /// inline target IR specification section 5). Members are spliced
    /// ahead of the signature so its method types resolve them (decision
    /// D65).
    fn parse_sig_item(&mut self) -> Result<SigItem> {
        if self.at(&Tok::Opaque) {
            return Ok(SigItem::Member(Decl::OpaqueType(self.parse_opaque_type()?)));
        }
        if self.at(&Tok::Record) {
            return Ok(SigItem::Member(Decl::Record(self.parse_record()?)));
        }
        if self.at(&Tok::Law) {
            self.expect(&Tok::Law, "`law`")?;
            let name = self.parse_value_name()?;
            self.expect(&Tok::ColonColon, "`::`")?;
            let ty = self.parse_type()?;
            return Ok(SigItem::Law((name, ty)));
        }
        Ok(SigItem::Method(self.parse_method_signature()?))
    }

    /// `opaque type T [params] :: Type` (foreign function interface
    /// specification section 2): an abstract foreign resource with no
    /// constructors and no layout projection.
    /// `type R params = refine (x :: T) with p` (refinement types
    /// design): the refinement `R` of the container `T` by the decidable
    /// predicate `p` over the refined binder `x`. The binder takes the
    /// dependent binder's `::` or the design note's `:`.
    fn parse_refine(&mut self) -> Result<RefineDecl> {
        let span = self.expect(&Tok::TypeKw, "`type`")?.span;
        let name = self.parse_ctor_name("a refinement name")?;
        let params = self.parse_params()?;
        if self.at(&Tok::ColonColon) {
            return Err(self.err(
                "kind signatures on refinement declarations are not supported in the bootstrap slice",
            ));
        }
        self.expect(&Tok::Equals, "`=`")?;
        match self.peek_tok().clone() {
            Tok::VarId(s) if s == "refine" => {
                self.bump();
            }
            t => {
                return Err(self.err(format!(
                    "expected `refine (x :: T) with p` (a `type` declaration is a refinement; `opaque type T :: Type` declares an abstract foreign resource), found {}",
                    describe(&t)
                )))
            }
        }
        self.expect(&Tok::LParen, "the refined binder `(x :: T)`")?;
        let binder = match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => {
                let span = self.peek().span;
                self.bump();
                Name::new(s, span)
            }
            t => {
                return Err(self.err(format!(
                    "expected the refined binder's name, found {}",
                    describe(&t)
                )))
            }
        };
        let colon = self.at(&Tok::ColonColon)
            || matches!(self.peek_tok(), Tok::Op(s) if s == ":");
        if !colon {
            return Err(self.err(
                "expected `::` between the refined binder and its container type",
            ));
        }
        self.bump();
        let base = self.parse_type()?;
        self.expect(&Tok::RParen, "`)` after the refined binder")?;
        self.expect(&Tok::With, "`with` before the refinement's predicate")?;
        let pred = self.parse_type()?;
        Ok(RefineDecl { name, params, binder, base, pred, span })
    }

    fn parse_opaque_type(&mut self) -> Result<OpaqueTypeDecl> {
        let span = self.expect(&Tok::Opaque, "`opaque`")?.span;
        self.expect(&Tok::TypeKw, "`type` after `opaque`")?;
        let name = self.parse_ctor_name("an opaque type name")?;
        let params = self.parse_params()?;
        self.expect(&Tok::ColonColon, "`::`")?;
        let kind = self.parse_type()?;
        Ok(OpaqueTypeDecl { name, params, kind, span })
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

    /// `effect L [params] where { op :: ty, ... }`: a user-declared effect
    /// whose operations are ordinary functions performing the label
    /// (design/effects-capabilities-sequencing.md section 2). Parameters
    /// are region identities: `r` or `(r :: Region)`.
    fn parse_effect_decl(&mut self) -> Result<EffectDecl> {
        let span = self.expect(&Tok::Effect, "`effect`")?.span;
        let name = self.parse_ctor_name("an effect name")?;
        let mut params = Vec::new();
        while !self.at(&Tok::Where) {
            if self.at(&Tok::LParen) {
                self.bump();
                let p = self.parse_value_name()?;
                if self.at(&Tok::ColonColon) {
                    self.bump();
                    let kind = self.parse_ctor_name("a kind name")?;
                    if kind.text != "Region" {
                        return Err(Error::at(
                            kind.span,
                            "effect parameters are region identities (`(r :: Region)`); other kinds are not supported in the bootstrap slice",
                        ));
                    }
                }
                self.expect(&Tok::RParen, "`)`")?;
                params.push(p);
            } else if let Tok::VarId(s) = self.peek_tok().clone() {
                if s == "_" {
                    return Err(self.err("an effect parameter needs a name"));
                }
                let sp = self.peek().span;
                self.bump();
                params.push(Name::new(s, sp));
            } else {
                return Err(self.err(format!(
                    "expected an effect parameter or `where`, found {}",
                    describe(self.peek_tok())
                )));
            }
            if params.len() > 1 {
                return Err(Error::at(
                    span,
                    "an effect takes at most one region parameter in the bootstrap slice",
                ));
            }
        }
        self.expect(&Tok::Where, "`where`")?;
        let ops = self.parse_block_items("the effect block", |p| p.parse_method_signature())?;
        if ops.is_empty() {
            return Err(Error::at(span, "an effect needs at least one operation"));
        }
        Ok(EffectDecl { name, params, ops, span })
    }

    fn parse_instance(&mut self) -> Result<InstDecl> {
        let start = self.peek().span;
        let implicit = self.at(&Tok::Implicit);
        if implicit {
            self.bump();
        }
        self.expect(&Tok::Instance, "`instance`")?;
        let name = self.parse_ctor_name("an instance name")?;
        let params = self
            .parse_params()?
            .into_iter()
            .map(|b| b.name)
            .collect::<Vec<_>>();
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
            let attrs = p.parse_attr_list()?;
            let name = p.parse_ctor_name("a constructor name")?;
            p.expect(&Tok::ColonColon, "`::`")?;
            let ty = p.parse_type()?;
            let span = name.span;
            Ok(CtorDecl { attrs, name, ty, span })
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
            // Attributes may prefix the field declaration or its type
            // (attribute annotations specification sections 1.1 and 4);
            // both spell the field's layout modifiers.
            let mut attrs = p.parse_attr_list()?;
            let erased = p.at(&Tok::Erased);
            if erased {
                p.bump();
            }
            let name = p.parse_value_name()?;
            p.expect(&Tok::ColonColon, "`::`")?;
            attrs.extend(p.parse_attr_list()?);
            let ty = p.parse_type()?;
            let span = name.span;
            Ok(FieldDecl { attrs, name, ty, erased, span })
        })?;
        Ok(RecordDecl { name, params, fields, span })
    }

    /// `codata C params where { obs :: T, ... }`: a coinductive type of
    /// observations (codata and guardedness design, section 2). The block
    /// has the shape of a record's field block, but an observation is
    /// always runtime code, never `erased` evidence.
    fn parse_codata(&mut self) -> Result<RecordDecl> {
        let span = self.expect(&Tok::Codata, "`codata`")?.span;
        let name = self.parse_ctor_name("a codata type name")?;
        let params = self.parse_params()?;
        if self.at(&Tok::ColonColon) {
            return Err(self.err(
                "kind signatures on `codata` declarations are not supported in the bootstrap slice",
            ));
        }
        self.expect(&Tok::Where, "`where`")?;
        let fields = self.parse_block_items("the observation block", |p| {
            let mut attrs = p.parse_attr_list()?;
            if p.at(&Tok::Erased) {
                return Err(p.err(
                    "an observation is runtime code, not erased evidence; `erased` is only for record fields",
                ));
            }
            let name = p.parse_value_name()?;
            p.expect(&Tok::ColonColon, "`::`")?;
            attrs.extend(p.parse_attr_list()?);
            let ty = p.parse_type()?;
            let span = name.span;
            Ok(FieldDecl { attrs, name, ty, erased: false, span })
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
                // The contextual Pipe rule (surface grammar local
                // amendments section 6): `infixl 4 |` declares the `|`
                // operator's fixity.
                Tok::Pipe => {
                    self.bump();
                    ops.push(Name::new("|", op_span));
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

    /// Parameters of `data`, `record`, `signature` and `instance`:
    /// `x`, `X`, `(x :: T)` or `{x}`. Annotations survive: on `data` and
    /// `record` parameters they classify type parameters versus index
    /// values (dependent core kernel design, section 5); on `signature`
    /// parameters they state the kind (`Functor (f :: Type -> Type)`). A
    /// capitalized bare parameter (`signature Functor F`) is a type
    /// variable in a constructor's spelling.
    fn parse_params(&mut self) -> Result<Vec<TypeBinder>> {
        let mut binders = Vec::new();
        loop {
            match self.peek_tok() {
                Tok::VarId(s) if s != "_" => {
                    let span = self.peek().span;
                    let name = Name::new(s.clone(), span);
                    self.bump();
                    binders.push(TypeBinder { name, ann: None, implicit: false });
                }
                Tok::ConId(s) => {
                    // A capitalized bare parameter (`signature Functor
                    // F`): a type variable in a constructor's spelling.
                    let span = self.peek().span;
                    let name = Name::new(s.clone(), span);
                    self.bump();
                    binders.push(TypeBinder { name, ann: None, implicit: false });
                }
                Tok::LParen | Tok::LBrace => {
                    binders.push(self.parse_type_binder()?);
                }
                _ => break,
            }
        }
        Ok(binders)
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
            Tok::VarId(_)
            | Tok::ConId(_)
            | Tok::Int(_)
            | Tok::StrLit(_)
            | Tok::CharLit(_)
            | Tok::SymLit(_)
            | Tok::Underscore => {
                // `VarId` reaches here only as the wildcard binder `_`
                // (`let _ = e`: the purposeful discard of the
                // `Result` must-use default; attribute annotations
                // specification section 10).
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
                | Tok::SymLit(_) | Tok::TypeNameLit(_) | Tok::LParen | Tok::LBracket
                | Tok::Underscore | Tok::Tilde
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
            Tok::SymLit(s) => {
                // `'name` in pattern position: the interned `Sym`
                // constant, identity-tested (surface grammar local
                // amendments section 12).
                self.bump();
                Ok(Pat::Sym(s, span))
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
            Tok::TypeNameLit(s) => Err(self.err(format!(
                "the type name quote `''{s}` belongs to a staging or macro name position (surface grammar local amendments section 12)"
            ))),
            Tok::Tilde => Err(self.err(
                "irrefutable (`~`) patterns are not supported in the bootstrap slice",
            )),
            t => Err(self.err(format!("expected a pattern, found {}", describe(&t)))),
        }
    }

    // ------------------------------------------------------------ terms

    fn starts_postfix(&self) -> bool {
        self.at_splice()
            || matches!(
                self.peek_tok(),
                Tok::VarId(_)
                    | Tok::ConId(_)
                    | Tok::Int(_)
                    | Tok::FloatLit(_)
                    | Tok::StrLit(_)
                    | Tok::CharLit(_)
                    | Tok::SymLit(_)
                    | Tok::TypeNameLit(_)
                    | Tok::LParen
                    | Tok::LBracket
                    | Tok::LQuote
                    | Tok::Underscore
            )
    }

    /// Is the token pair here a splice opener `$(`? The lexer emits
    /// `Dollar` + `LParen` sharing one span for the reserved `$(` token;
    /// a spaced `$ (` is ordinary reserved application (surface grammar).
    fn at_splice(&self) -> bool {
        matches!(self.peek_tok(), Tok::Dollar)
            && matches!(self.peek_tok_at(1), Some(Tok::LParen))
            && self.toks.get(self.pos).map(|t| t.span) == self.toks.get(self.pos + 1).map(|t| t.span)
    }

    fn parse_term(&mut self) -> Result<Term> {
        match self.peek_tok() {
            Tok::Backslash => self.parse_lambda(),
            Tok::Let => self.parse_let(),
            Tok::Letrec => self.parse_letrec(),
            Tok::Tilde => Err(self.err(
                "memo bindings (`~ pattern = term`) are not supported in the bootstrap slice",
            )),
            Tok::If => self.parse_if(),
            Tok::Case => self.parse_case(),
            Tok::Do => self.parse_do(),
            Tok::Handle => self.parse_handle(),
            Tok::With => self.parse_with(),
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
                // The contextual Pipe rule (surface grammar local
                // amendments section 6): infix application of `|`; the
                // guard and row separators are consumed by their own
                // positions before a term is parsed.
                Tok::Pipe => false,
                Tok::Dollar if !self.at_splice() => true,
                _ => break,
            };
            let name = if is_dollar {
                self.bump();
                Name::new("$", span)
            } else {
                match self.bump().tok {
                    Tok::Op(s) => Name::new(s, span),
                    Tok::Pipe => Name::new("|", span),
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
                Tok::At => {
                    self.bump();
                    self.expect(&Tok::LBrace, "`{` after `@`")?;
                    let mut supplies = Vec::new();
                    if !self.at(&Tok::RBrace) {
                        loop {
                            let name = self.parse_value_name()?;
                            self.expect(&Tok::Equals, "`=` in the capability supply")?;
                            let value = self.parse_term()?;
                            supplies.push((name, value));
                            if self.at(&Tok::Comma) {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Tok::RBrace, "`}` closing the capability supplies")?;
                    let span = t.span();
                    t = Term::CapApply { head: Box::new(t), supplies, span };
                }
                Tok::Bang => {
                    let path = match &t {
                        Term::Var(p) => p.clone(),
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "a macro invocation must name a macro (`name!(...)`)",
                            ))
                        }
                    };
                    self.bump();
                    let args = self.parse_token_tree()?;
                    let span = t.span();
                    t = Term::MacroCall { path, args, span };
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
            Tok::VarId(s) if s == "handler" => self.parse_handler_expr(),
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
            Tok::SymLit(s) => {
                // `'name` in expression position: the interned `Sym`
                // constant (surface grammar local amendments section 12).
                self.bump();
                Ok(Term::Sym(s, span))
            }
            Tok::TypeNameLit(s) => Err(self.err(format!(
                "the type name quote `''{s}` belongs to a staging or macro name position (surface grammar local amendments section 12); type-level symbols are deferred (the spelling is reserved)"
            ))),
            Tok::FloatLit(_) => Err(self.err(
                "float literals are not supported in the bootstrap slice (there is no AST node for them)",
            )),
            Tok::LParen => {
                self.bump();
                if self.at(&Tok::RParen) {
                    self.bump();
                    return Ok(Term::Unit(span));
                }
                if (matches!(self.peek_tok(), Tok::Op(_)) || matches!(self.peek_tok(), Tok::Pipe))
                    && matches!(self.peek_tok_at(1), Some(Tok::RParen))
                {
                    let op = match self.bump().tok {
                        Tok::Op(s) => s,
                        // The contextual Pipe rule (surface grammar
                        // local amendments section 6).
                        Tok::Pipe => "|".to_string(),
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
            Tok::LBrace if !self.peek().virtual_ => {
                // `{k => v, ...}`, the map literal (surface grammar local
                // amendments section 10); empty is the empty map.
                self.bump();
                let mut kvs = Vec::new();
                if !self.at(&Tok::RBrace) {
                    loop {
                        let k = self.parse_term()?;
                        self.expect(&Tok::Darrow, "`=>` in the map literal")?;
                        let v = self.parse_term()?;
                        kvs.push((k, v));
                        if self.at(&Tok::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&Tok::RBrace, "`}` closing the map literal")?;
                Ok(Term::Map(kvs, span))
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
                if self.at(&Tok::DotDot) {
                    // `[e1..e3]` and `[e1, e2 .. e3]`: the range form
                    // of the surface range literal.
                    self.bump();
                    let end = self.parse_term()?;
                    self.expect(&Tok::RBracket, "`]` closing the range")?;
                    let mut it = items.into_iter();
                    let start = match it.next() {
                        Some(s) => s,
                        None => return Err(self.err("a range literal needs a start")),
                    };
                    let step = it.next();
                    if it.next().is_some() {
                        return Err(self.err(
                            "a range literal is `[e1..e3]` or `[e1, e2 .. e3]`",
                        ));
                    }
                    return Ok(Term::Range(
                        Box::new(start),
                        step.map(Box::new),
                        Box::new(end),
                        span,
                    ));
                }
                self.expect(&Tok::RBracket, "`]` closing the list")?;
                Ok(Term::List(items, span))
            }
            Tok::LQuote => {
                self.bump();
                let body = self.parse_term()?;
                let close = self.expect(&Tok::RQuote, "`|]` closing the quotation")?;
                let _ = close;
                Ok(Term::Quote { body: Box::new(body), span })
            }
            Tok::QuoteDecls => {
                self.bump();
                self.expect(&Tok::Where, "`where` after `quoteDecls`")?;
                let decls = self.parse_block_items("the quoted declaration block", |p| {
                    p.parse_declaration()
                })?;
                Ok(Term::QuoteDecls { decls, span })
            }
            Tok::DollarParen => {
                self.bump();
                let body = self.parse_term()?;
                self.expect(&Tok::RParen, "`)` closing the splice")?;
                Ok(Term::Splice { body: Box::new(body), span })
            }
            Tok::Dollar if self.at_splice() => {
                self.bump();
                self.bump();
                let body = self.parse_term()?;
                self.expect(&Tok::RParen, "`)` closing the splice")?;
                Ok(Term::Splice { body: Box::new(body), span })
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

    /// `letrec b1; ...; bn in e` (the `letrec` keyword lexes as its
    /// marker plus `let`, so the group opens a plain `let` layout block).
    fn parse_letrec(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Letrec, "`letrec`")?.span;
        self.expect(&Tok::Let, "`let` after `letrec`")?;
        let decls = self.parse_block_items("the recursive local block", |p| {
            p.parse_local_decl()
        })?;
        self.expect(&Tok::In, "`in`")?;
        let body = self.parse_term()?;
        Ok(Term::Letrec(decls, Box::new(body), span))
    }

    /// `if [cond attrs] c then [then attrs] t else [else attrs] e`
    /// (attribute annotations specification sections 1.1 and 2.1): the
    /// condition-position attributes annotate the true branch and
    /// complete the decision point's weight pair; the branch-position
    /// attributes annotate their branch.
    fn parse_if(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::If, "`if`")?.span;
        let cond_attrs = self.parse_attr_list()?;
        let c = self.parse_term()?;
        self.expect(&Tok::Then, "`then`")?;
        let then_attrs = self.parse_attr_list()?;
        let t = self.parse_term()?;
        self.expect(&Tok::Else, "`else`")?;
        let else_attrs = self.parse_attr_list()?;
        let e = self.parse_term()?;
        Ok(Term::If {
            cond: Box::new(c),
            then: Box::new(t),
            els: Box::new(e),
            cond_attrs,
            then_attrs,
            else_attrs,
            span,
        })
    }

    fn parse_case(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Case, "`case`")?.span;
        let scrut = self.parse_term()?;
        self.expect(&Tok::Of, "`of`")?;
        let alts = self.parse_block_items("the alternative block", |p| {
            // The alternative's attributes prefix its pattern; the
            // guard's attributes sit after `|` (attribute annotations
            // specification section 1.1).
            let attrs = p.parse_attr_list()?;
            let pat = p.parse_pattern()?;
            let mut guard_attrs = Vec::new();
            let guard = if p.at(&Tok::Pipe) {
                p.bump();
                guard_attrs = p.parse_attr_list()?;
                Some(p.parse_term()?)
            } else {
                None
            };
            p.expect(&Tok::Arrow, "`->`")?;
            let body = p.parse_term()?;
            let span = pat.span();
            Ok(Alt { attrs, pat, guard, guard_attrs, body, span })
        })?;
        Ok(Term::Case(Box::new(scrut), alts, span))
    }

    /// `do { stmts }`: statements are `pat <- term` or bare `term`; the
    /// last statement is the block's result.
    fn parse_do(&mut self) -> Result<Term> {
        let span = self.peek().span;
        let stmts = self.parse_do_stmts()?;
        Ok(Term::Do(stmts, span))
    }

    /// `do { stmts }` (the keyword through the closing brace); also the
    /// body of a `generate do` declaration.
    fn parse_do_stmts(&mut self) -> Result<Vec<DoStmt>> {
        self.expect(&Tok::Do, "`do`")?;
        let stmts = self.parse_block_items("the do block", |p| p.parse_do_stmt())?;
        if stmts.is_empty() {
            return Err(self.err("a do block needs at least one statement"));
        }
        Ok(stmts)
    }

    /// One balanced token tree: `(...)`, `[...]`, or `{...}` with nested
    /// delimiters matching. The tree is macro input and stays unparsed
    /// here (grammar: `token_tree`).
    fn parse_token_tree(&mut self) -> Result<TokenTree> {
        let start = self.peek().span;
        if !matches!(self.peek_tok(), Tok::LParen | Tok::LBracket | Tok::LBrace) {
            return Err(self.err(
                "a macro invocation needs a balanced token tree in `()`, `[]` or `{}`",
            ));
        }
        let mut stack: Vec<Tok> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let tok = self.peek_tok().clone();
            if tok == Tok::Eof {
                return Err(Error::at(start, "unterminated macro argument token tree"));
            }
            if stack.is_empty() && !tokens.is_empty() {
                return Err(Error::at(
                    start,
                    "a macro argument must be one balanced token tree",
                ));
            }
            match &tok {
                Tok::LParen => stack.push(Tok::RParen),
                Tok::LBracket => stack.push(Tok::RBracket),
                Tok::LBrace => stack.push(Tok::RBrace),
                Tok::LQuote => stack.push(Tok::RQuote),
                Tok::DollarParen => stack.push(Tok::RParen),
                Tok::RParen | Tok::RBracket | Tok::RBrace | Tok::RQuote => {
                    match stack.pop() {
                        Some(expected) if expected == tok => {}
                        _ => {
                            return Err(Error::at(
                                self.peek().span,
                                "unbalanced delimiters in the macro argument token tree",
                            ))
                        }
                    }
                }
                _ => {}
            }
            tokens.push(self.bump());
            if stack.is_empty() {
                break;
            }
        }
        Ok(TokenTree { tokens, span: start })
    }

    /// One `do` statement: a `let` binding group (`let` plus its whole
    /// local block, the surface EBNF `do_statement = "let", local_block`),
    /// `pat <- term`, or a bare `term`.
    fn parse_do_stmt(&mut self) -> Result<DoStmt> {
        let span = self.peek().span;
        // `let` plus a local block: one binding group of pure locals. The
        // layout pass opens an implicit block after `let` and closes it
        // with a virtual `}` at the statement boundary; an explicit `{`
        // block parses verbatim.
        if self.at(&Tok::Let) {
            self.bump();
            let decls = self.parse_block_items("the local block", |p| p.parse_local_decl())?;
            return Ok(DoStmt::Let { decls, span });
        }
        // `pat <- term` without consuming on failure: a statement that is
        // not a binding parses as a plain expression.
        let save = self.pos;
        if let Ok(pat) = self.parse_pattern() {
            if self.at(&Tok::LArrow) {
                self.bump();
                let term = self.parse_term()?;
                return Ok(DoStmt::Bind { pat, term, span });
            }
        }
        self.pos = save;
        let term = self.parse_term()?;
        Ok(DoStmt::Expr { term, span })
    }

    /// `handle e with h`.
    fn parse_handle(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::Handle, "`handle`")?.span;
        let handled = self.parse_term()?;
        self.expect(&Tok::With, "`with` after the handled computation")?;
        let handler = self.parse_term()?;
        Ok(Term::Handle(Box::new(handled), Box::new(handler), span))
    }

    /// `with (x = e) in body`: a lexical capability supply (the `in` is
    /// routed to the supply by the layout pass, layout.rs).
    fn parse_with(&mut self) -> Result<Term> {
        let span = self.expect(&Tok::With, "`with`")?.span;
        self.expect(&Tok::LParen, "`(` opening the capability supply")?;
        let name = self.parse_value_name()?;
        if !self.at(&Tok::Equals) {
            return Err(self.err(
                "expected a capability supply binding (`name = expression`); a parenthesized `with` operand is ambiguous at the layout layer, so bind it first",
            ));
        }
        self.bump();
        let value = self.parse_term()?;
        self.expect(&Tok::RParen, "`)` closing the capability supply")?;
        self.expect(&Tok::In, "`in` after the capability supply")?;
        let body = self.parse_term()?;
        Ok(Term::With { name, value: Box::new(value), body: Box::new(body), span })
    }

    /// `handler L [r] where { return p -> e; op pats [resume k] -> e }`.
    fn parse_handler_expr(&mut self) -> Result<Term> {
        let span = self.bump().span;
        let effect = self.parse_con_path()?;
        let region = match self.peek_tok().clone() {
            Tok::VarId(s) if s != "_" => {
                let sp = self.peek().span;
                self.bump();
                Some(Name::new(s, sp))
            }
            _ => None,
        };
        self.expect(&Tok::Where, "`where` after the handled label")?;
        enum HClause {
            Ret(Pat, Term, Span),
            Op(OpClause),
        }
        let items = self.parse_block_items("the handler block", |p| {
            let start = p.peek().span;
            if p.at(&Tok::Return) {
                p.bump();
                let pat = p.parse_pattern()?;
                p.expect(&Tok::Arrow, "`->` after the return pattern")?;
                let body = p.parse_term()?;
                return Ok(HClause::Ret(pat, body, start));
            }
            let op = p.parse_value_name()?;
            let mut pats = Vec::new();
            let mut resume = None;
            while !p.at(&Tok::Arrow) {
                if p.at(&Tok::Resume) {
                    if resume.is_some() {
                        return Err(p.err("a clause takes at most one `resume` binder"));
                    }
                    p.bump();
                    let kspan = p.peek().span;
                    let k = p.parse_value_name()?;
                    resume = Some(Name::new(k.text, kspan));
                } else {
                    pats.push(p.parse_atomic_pattern()?);
                }
            }
            p.expect(&Tok::Arrow, "`->` after the clause patterns")?;
            let body = p.parse_term()?;
            Ok(HClause::Op(OpClause { op, pats, resume, body, span: start }))
        })?;
        let mut ret: Option<(Pat, Term)> = None;
        let mut clauses = Vec::new();
        for item in items {
            match item {
                HClause::Ret(pat, body, start) => {
                    if ret.is_some() {
                        return Err(Error::at(start, "a handler takes exactly one `return` clause"));
                    }
                    ret = Some((pat, body));
                }
                HClause::Op(c) => clauses.push(c),
            }
        }
        let (ret_pat, ret_body) = ret.ok_or_else(|| {
            Error::at(span, "a handler needs a `return p -> e` clause")
        })?;
        Ok(Term::Handler(Box::new(HandlerExpr {
            effect,
            region,
            ret_pat,
            ret_body,
            clauses,
            span,
        })))
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

    /// An infix chain of type applications (`n + 1`, `length bytes <=
    /// limit`); the fixity pass resolves it into nested `App` spines over
    /// the operator names. Arrows bind looser than operators, so operands
    /// are type applications.
    fn parse_infix_type(&mut self) -> Result<Type> {
        let mut terms = vec![self.parse_type_app()?];
        let mut ops: Vec<Name> = Vec::new();
        loop {
            let span = self.peek().span;
            let name = match self.peek_tok() {
                Tok::Op(s) => {
                    let s = s.clone();
                    self.bump();
                    Name::new(s, span)
                }
                // The contextual Pipe rule (surface grammar local
                // amendments section 6).
                Tok::Pipe => {
                    self.bump();
                    Name::new("|", span)
                }
                _ => break,
            };
            ops.push(name);
            terms.push(self.parse_type_app()?);
        }
        if ops.is_empty() {
            Ok(terms.pop().expect("one type"))
        } else {
            let span = terms[0].span();
            Ok(Type::Infix(terms, ops, span))
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
        let cs = self.parse_constraint_items()?;
        self.expect(&Tok::RParen, "`)` after the constraint list")?;
        self.expect(&Tok::Darrow, "`=>`")?;
        let body = self.parse_type()?;
        Ok(Type::Using(cs, Box::new(body), span))
    }

    /// The comma-separated `name :: Class args` items of a `using (...)`
    /// clause (the `(` has been consumed).
    fn parse_constraint_items(&mut self) -> Result<Vec<Constraint>> {
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
        Ok(cs)
    }

    fn parse_arrow_type(&mut self) -> Result<Type> {
        let span = self.peek().span;
        if self.is_typed_binder_head() {
            let (name, ann) = self.parse_typed_binder()?;
            let mult = self.parse_mult_opt()?;
            self.expect(&Tok::Arrow, "`->` after the dependent binder")?;
            let eff = self.parse_effect_row_opt()?;
            let res = self.parse_type()?;
            return Ok(Type::DepFun(name, Box::new(ann), mult, eff, Box::new(res), span));
        }
        if self.at(&Tok::LBrace) {
            return Err(self.err(
                "implicit dependent function types (`{x :: A} -> B`) are not supported in the bootstrap slice",
            ));
        }
        let arg = self.parse_infix_type()?;
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
        matches!(self.peek_tok(), Tok::VarId(_) | Tok::ConId(_) | Tok::LParen | Tok::Int(_))
    }

    fn parse_type_atom(&mut self) -> Result<Type> {
        let span = self.peek().span;
        match self.peek_tok().clone() {
            Tok::Backslash => {
                // `\x -> t` in an index position: a lambda over index
                // values (`List.map (\x -> f (g x)) xs`); several binders
                // nest.
                self.bump();
                let mut binders: Vec<Name> = Vec::new();
                while let Tok::VarId(v) = self.peek_tok().clone() {
                    if v == "_" {
                        break;
                    }
                    let s = self.peek().span;
                    self.bump();
                    binders.push(Name::new(v, s));
                }
                if binders.is_empty() {
                    return Err(self.err(
                        "an index-level lambda needs at least one binder (`\\x -> ...`)",
                    ));
                }
                self.expect(&Tok::Arrow, "`->` in the index-level lambda")?;
                let mut body = self.parse_type()?;
                for n in binders.into_iter().rev() {
                    body = Type::Lam(n, Box::new(body), span);
                }
                Ok(body)
            }
            Tok::ConId(_) => {
                let mut path = self.parse_con_path()?;
                if self.at(&Tok::Dot) {
                    // A dotted value name in type position: `List.map` in
                    // `Eq (List.map f xs) ys` is the index-level function
                    // reference of a proposition's argument, not a field
                    // projection (dependent core kernel design section 2).
                    if let Some(Tok::VarId(v)) = self.peek_tok_at(1).cloned() {
                        let v_span = self.peek_tok_at(1).map(|_| self.toks[self.pos + 1].span).unwrap_or(span);
                        self.bump();
                        self.bump();
                        path.segments.push(Name::new(v, v_span));
                    } else {
                        return Err(self.err("field projection is not supported in types"));
                    }
                }
                Ok(Type::Con(path, span))
            }
            Tok::VarId(s) => {
                self.bump();
                Ok(Type::Var(Name::new(s, span), span))
            }
            Tok::Int(s) => {
                self.bump();
                Ok(Type::Num(s, span))
            }
            Tok::SymLit(s) => Err(self.err(format!(
                "the symbol `'{s}` is not a type: type-level symbols are deferred (the `'` spelling is reserved at the type level)"
            ))),
            Tok::TypeNameLit(s) => Err(self.err(format!(
                "the type name quote `''{s}` belongs to a staging or macro name position (surface grammar local amendments section 12); type-level symbols are deferred (the spelling is reserved)"
            ))),
            Tok::LParen => {
                self.bump();
                if self.at(&Tok::RParen) {
                    self.bump();
                    return Ok(Type::Unit(span));
                }
                // Type former atoms: `(->)` (the pure total arrow family)
                // and the structural tuple formers `(,)`, `(,,)`, ...
                if self.at(&Tok::Arrow) && matches!(self.peek_tok_at(1), Some(Tok::RParen)) {
                    self.bump();
                    self.bump();
                    return Ok(Type::Con(
                        ConPath { segments: vec![Name::new("->", span)] },
                        span,
                    ));
                }
                if self.at(&Tok::Comma) {
                    let mut commas = 0;
                    while self.at(&Tok::Comma) {
                        self.bump();
                        commas += 1;
                    }
                    if self.at(&Tok::RParen) {
                        self.bump();
                        let name = format!("({})", ",".repeat(commas));
                        return Ok(Type::Con(
                            ConPath { segments: vec![Name::new(name, span)] },
                            span,
                        ));
                    }
                    return Err(self.err(
                        "a parenthesized comma list is the tuple type former `(,)`/`(,,)`/...; tuple types are `(T1, T2)`",
                    ));
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
                    // Effect label arguments are region names (at most one),
                    // or the boundary name of a `Foreign <boundary>` label
                    // (foreign function interface specification section 2).
                    let mut args = Vec::new();
                    while let Tok::VarId(s) = self.peek_tok().clone() {
                        let span = self.peek().span;
                        self.bump();
                        args.push(Name::new(s, span));
                    }
                    if args.is_empty() {
                        if let Tok::ConId(s) = self.peek_tok().clone() {
                            let span = self.peek().span;
                            self.bump();
                            args.push(Name::new(s, span));
                        }
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
        resolve_module_decl(d, fix)?;
    }
    Ok(())
}

/// Fixity resolution of one declaration (shared by module and generated
/// declaration resolution).
fn resolve_module_decl(d: &mut Decl, fix: &FixityTable) -> Result<()> {
    match d {
        Decl::NameSpliced { name_tm, inner } => {
            resolve_term(name_tm, fix)?;
            resolve_module_decl(inner, fix)?;
        }
        Decl::Equation { eq, .. } => resolve_equation(eq, fix)?,
        Decl::Instance(inst) => {
            resolve_type(&mut inst.target, fix)?;
            for eq in &mut inst.methods {
                resolve_equation(eq, fix)?;
            }
        }
        Decl::ValueSig { ty, .. } => resolve_type(ty, fix)?,
        Decl::Prim(p) => {
            resolve_type(&mut p.ty, fix)?;
            for b in &mut p.bodies {
                if let Some(w) = &mut b.when {
                    resolve_term(w, fix)?;
                }
                if let Some(j) = &mut b.justify {
                    resolve_term(j, fix)?;
                }
                for u in &mut b.uses {
                    resolve_type(&mut u.ty, fix)?;
                }
            }
        }
        Decl::OpaqueType(o) => resolve_type(&mut o.kind, fix)?,
        Decl::Refine(r) => {
            for b in &mut r.params {
                if let Some(ann) = &mut b.ann {
                    resolve_type(ann, fix)?;
                }
            }
            resolve_type(&mut r.base, fix)?;
            resolve_type(&mut r.pred, fix)?;
        }
        Decl::Rewrite(r) => {
            for b in &mut r.binders {
                if let Some(ann) = &mut b.ann {
                    resolve_type(ann, fix)?;
                }
            }
            for c in &mut r.ctx {
                for a in &mut c.args {
                    resolve_type(a, fix)?;
                }
            }
            resolve_term(&mut r.match_tm, fix)?;
            resolve_term(&mut r.replace_tm, fix)?;
            if let Some(req) = &mut r.require {
                resolve_term(req, fix)?;
            }
            resolve_term(&mut r.justify, fix)?;
        }
        Decl::Upgrade(u) => {
            if let Some(m) = &mut u.migrate {
                resolve_term(m, fix)?;
            }
            if let Some(r) = &mut u.related {
                resolve_term(r, fix)?;
            }
        }
        Decl::Contract(c) => {
            for cl in &mut c.clauses {
                match cl {
                    ContractClause::Formula { body, .. }
                    | ContractClause::AssumeProp { body } => resolve_term(body, fix)?,
                    ContractClause::AssumeNamed { .. } => {}
                }
            }
        }
        Decl::Data(data) => {
            for b in &mut data.params {
                if let Some(ann) = &mut b.ann {
                    resolve_type(ann, fix)?;
                }
            }
            for c in &mut data.ctors {
                resolve_type(&mut c.ty, fix)?;
            }
        }
        Decl::Record(rec) | Decl::Codata(rec) => {
            for b in &mut rec.params {
                if let Some(ann) = &mut b.ann {
                    resolve_type(ann, fix)?;
                }
            }
            for f in &mut rec.fields {
                resolve_type(&mut f.ty, fix)?;
            }
        }
        Decl::Foreign(f) => resolve_type(&mut f.ty, fix)?,
        Decl::Effect(e) => {
            for (_, ty) in &mut e.ops {
                resolve_type(ty, fix)?;
            }
        }
        Decl::Signature(sig) => {
            for (_, ty) in &mut sig.methods {
                resolve_type(ty, fix)?;
            }
            for (_, ty) in &mut sig.laws {
                resolve_type(ty, fix)?;
            }
        }
        Decl::Import(_) | Decl::ImportData(_) | Decl::Export(_) | Decl::Fixity(_) => {}
        Decl::Attributed { inner, .. } => resolve_module_decl(inner, fix)?,
        Decl::Generate { stmts, .. } => {
            for s in stmts {
                match s {
                    DoStmt::Bind { term, .. } | DoStmt::Expr { term, .. } => {
                        resolve_term(term, fix)?
                    }
                    DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            match decl {
                                LocalDecl::Eq { body, .. }
                                | LocalDecl::Bind { body, .. }
                                | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
                                LocalDecl::Sig { ty, .. } => resolve_type(ty, fix)?,
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Parse spliced `Syntax` tokens as one term, resolving its infix chains
/// with the caller's fixities (macro expansion, modules specification
/// section 8).
pub fn parse_term_tokens(tokens: &[Token], fix: &FixityTable) -> Result<Term> {
    let mut toks = tokens.to_vec();
    let span = toks.last().map(|t| t.span).unwrap_or(Span { line: 0, col: 0 });
    toks.push(Token { tok: Tok::Eof, span, virtual_: false });
    let mut p = Parser::new(&toks);
    let mut t = p.parse_term()?;
    p.expect_eof()?;
    resolve_term(&mut t, fix)?;
    Ok(t)
}

fn resolve_equation(eq: &mut Equation, fix: &FixityTable) -> Result<()> {
    resolve_term(&mut eq.body, fix)?;
    for l in &mut eq.locals {
        match l {
            LocalDecl::Eq { body, .. }
            | LocalDecl::Bind { body, .. }
            | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
            LocalDecl::Sig { ty, .. } => resolve_type(ty, fix)?,
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
        Term::Let(decls, body, _) | Term::Letrec(decls, body, _) => {
            for l in decls {
                match l {
                    LocalDecl::Eq { body, .. }
                    | LocalDecl::Bind { body, .. }
                    | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
                    LocalDecl::Sig { ty, .. } => resolve_type(ty, fix)?,
                }
            }
            resolve_term(body, fix)?;
        }
        Term::If { cond, then, els, .. } => {
            resolve_term(cond, fix)?;
            resolve_term(then, fix)?;
            resolve_term(els, fix)?;
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
        Term::Ann(e, ty, _) => {
            resolve_term(e, fix)?;
            resolve_type(ty, fix)?;
        }
        Term::Neg(e, _) => resolve_term(e, fix)?,
        Term::Tuple(items, _) | Term::List(items, _) => {
            for x in items {
                resolve_term(x, fix)?;
            }
        }
        Term::Range(start, step, end, _) => {
            resolve_term(start, fix)?;
            if let Some(x) = step {
                resolve_term(x, fix)?;
            }
            resolve_term(end, fix)?;
        }
        Term::Map(kvs, _) => {
            for (k, v) in kvs {
                resolve_term(k, fix)?;
                resolve_term(v, fix)?;
            }
        }
        Term::Proj(e, _, _) => resolve_term(e, fix)?,
        Term::Record(_, fields, _) => {
            for (_, x) in fields {
                resolve_term(x, fix)?;
            }
        }
        Term::Do(stmts, _) => {
            for s in stmts {
                match s {
                    DoStmt::Bind { term, .. } | DoStmt::Expr { term, .. } => {
                        resolve_term(term, fix)?
                    }
                    DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            match decl {
                                LocalDecl::Eq { body, .. }
                                | LocalDecl::Bind { body, .. }
                                | LocalDecl::Lazy { body, .. } => resolve_term(body, fix)?,
                                LocalDecl::Sig { ty, .. } => resolve_type(ty, fix)?,
                            }
                        }
                    }
                }
            }
        }
        Term::Handle(e, h, _) => {
            resolve_term(e, fix)?;
            resolve_term(h, fix)?;
        }
        Term::Handler(hx) => {
            resolve_term(&mut hx.ret_body, fix)?;
            for c in &mut hx.clauses {
                resolve_term(&mut c.body, fix)?;
            }
        }
        Term::With { value, body, .. } => {
            resolve_term(value, fix)?;
            resolve_term(body, fix)?;
        }
        Term::CapApply { head, supplies, .. } => {
            resolve_term(head, fix)?;
            for (_, x) in supplies {
                resolve_term(x, fix)?;
            }
        }
        Term::SeqOp(_, _) => {}
        Term::Quote { body, .. } => resolve_term(body, fix)?,
        Term::QuoteDecls { decls, .. } => {
            for d in decls {
                resolve_module_decl(d, fix)?;
            }
        }
        Term::Splice { body, .. } => resolve_term(body, fix)?,
        Term::MacroCall { .. } | Term::SpliceHole { .. } | Term::Str(_, _) | Term::Char(_, _)
        | Term::Unit(_) | Term::Hole(_) | Term::Var(_) | Term::Con(_) | Term::Int(_, _) => {}
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
        let if_term = |cond: Term, then: Term, els: Term| Term::If {
            cond: Box::new(cond),
            then: Box::new(then),
            els: Box::new(els),
            cond_attrs: Vec::new(),
            then_attrs: Vec::new(),
            else_attrs: Vec::new(),
            span,
        };
        if op.text == "&&" {
            if_term(lhs, rhs, bool_con("False"))
        } else {
            if_term(lhs, bool_con("True"), rhs)
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

// ------------------------------------------------- type-level fixity

fn reduce_ty(vals: &mut Vec<Type>, op: Name, span: Span) -> Result<()> {
    let rhs = vals.pop().expect("right operand");
    if vals.is_empty() {
        return Err(Error::at(span, "malformed infix chain"));
    }
    let lhs = vals.pop().expect("left operand");
    let t = if op.text == "$" {
        Type::App(Box::new(lhs), Box::new(rhs), span)
    } else {
        let f = Type::Var(op.clone(), op.span);
        Type::App(
            Box::new(Type::App(Box::new(f), Box::new(lhs), span)),
            Box::new(rhs),
            span,
        )
    };
    vals.push(t);
    Ok(())
}

/// Fixity of a type-level operator: the module's fixity table when the
/// operator is declared, and reserved defaults for the compiler's index
/// vocabulary (`+ - *` and the comparisons) otherwise (decision D34: a
/// refinement like `length bytes <= limit` needs no fixity declaration;
/// these spellings mean the mathematical operations in index position).
fn type_op_fixity(op: &Name, fix: &FixityTable) -> Result<(i8, Assoc)> {
    if let Some((assoc, prec)) = fix.get(&op.text) {
        return Ok((prec as i8, assoc));
    }
    Ok(match op.text.as_str() {
        "*" => (7, Assoc::Left),
        "+" | "-" => (6, Assoc::Left),
        "<" | "<=" | ">" | ">=" | "==" | "=" | "/=" | "!=" => (4, Assoc::None),
        "$" => (-1, Assoc::Right),
        _ => {
            return Err(Error::at(
                op.span,
                format!(
                    "there is no fixity declared for the operator `{}`, and there is no default for an undeclared operator",
                    op.text
                ),
            ))
        }
    })
}

/// Resolve a type-level infix chain with the same fixity rules as terms.
fn resolve_type_chain(
    terms: Vec<Type>,
    ops: Vec<Name>,
    span: Span,
    fix: &FixityTable,
) -> Result<Type> {
    let mut vals: Vec<Type> = Vec::with_capacity(terms.len());
    let mut ostack: Vec<(Name, i8, Assoc)> = Vec::new();
    let mut ts = terms.into_iter();
    vals.push(ts.next().expect("nonempty chain"));
    for (op, term) in ops.into_iter().zip(ts) {
        let (prec, assoc) = type_op_fixity(&op, fix)?;
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
            reduce_ty(&mut vals, top, span)?;
        }
        ostack.push((op, prec, assoc));
        vals.push(term);
    }
    while let Some((op, _, _)) = ostack.pop() {
        reduce_ty(&mut vals, op, span)?;
    }
    if vals.len() != 1 {
        return Err(Error::at(span, "malformed infix chain"));
    }
    Ok(vals.pop().expect("one result"))
}

/// Fixity resolution over the type grammar: type-level infix chains are
/// resolved into `App` spines over the operator names, and nested types
/// are walked.
fn resolve_type(t: &mut Type, fix: &FixityTable) -> Result<()> {
    match t {
        Type::Con(_, _) | Type::Var(_, _) | Type::Unit(_) | Type::Num(_, _) => {}
        Type::Lam(_, b, _) => resolve_type(b, fix)?,
        Type::App(f, a, _) => {
            resolve_type(f, fix)?;
            resolve_type(a, fix)?;
        }
        Type::AppRow(f, _, _) => resolve_type(f, fix)?,
        Type::Fun { arg, res, .. } => {
            resolve_type(arg, fix)?;
            resolve_type(res, fix)?;
        }
        Type::Forall(binders, body, _) => {
            for b in binders {
                if let Some(ann) = &mut b.ann {
                    resolve_type(ann, fix)?;
                }
            }
            resolve_type(body, fix)?;
        }
        Type::Using(cs, body, _) => {
            for c in cs {
                for a in &mut c.args {
                    resolve_type(a, fix)?;
                }
            }
            resolve_type(body, fix)?;
        }
        Type::DepFun(_, ann, _, _, res, _) => {
            resolve_type(ann, fix)?;
            resolve_type(res, fix)?;
        }
        Type::Sigma(_, ann, body, _) => {
            resolve_type(ann, fix)?;
            resolve_type(body, fix)?;
        }
        Type::Tuple(items, _) => {
            for i in items {
                resolve_type(i, fix)?;
            }
        }
        Type::Infix(_, _, _) => {
            let (terms, ops, span) = match std::mem::replace(t, Type::Unit(Span { line: 0, col: 0 }))
            {
                Type::Infix(terms, ops, span) => (terms, ops, span),
                _ => unreachable!(),
            };
            let mut terms = terms;
            for x in &mut terms {
                resolve_type(x, fix)?;
            }
            *t = resolve_type_chain(terms, ops, span, fix)?;
        }
    }
    Ok(())
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
        assert_eq!(data.params[0].name.text, "a");
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
        assert_eq!(sig.params[0].name.text, "a");
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
        assert!(matches!(eq_body(m, "h"), Term::If { .. }));
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
            Type::DepFun(name, ann, _, _, res, _) => {
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
        let e = err("module T where\n  effect E (r :: Type) where\n    op :: Int\n");
        assert!(e.contains("effect parameters are region identities"), "{e}");
        let e = err("module T where\n  type T = Int\n");
        assert!(e.contains("a `type` declaration is a refinement"), "{e}");
        let e = err("module T where\n  upgrade A to B where\n    x = 1\n");
        assert!(e.contains("`x` is not an upgrade component"), "{e}");
        let e = err("module T where\n  quoteDecls where\n    x :: Int\n    x = 1\n");
        assert!(e.contains("`quoteDecls where ...` is an expression"), "{e}");
    }

    #[test]
    fn upgrade_declarations_parse_their_components() {
        let p = ok("module T where\n  upgrade A to B where\n    migrate = f\n");
        match &p.module.decls[0] {
            Decl::Upgrade(u) => {
                assert_eq!(u.old.segments[0].text, "A");
                assert_eq!(u.new.segments[0].text, "B");
                assert!(u.migrate.is_some(), "the migrate component parses");
                assert!(u.related.is_none());
            }
            other => panic!("expected an upgrade declaration, got {other:?}"),
        }
        // The migration-free record-view form parses too.
        let p = ok("module T where\n  upgrade A to B where\n    related = r\n");
        match &p.module.decls[0] {
            Decl::Upgrade(u) => {
                assert!(u.migrate.is_none());
                assert!(u.related.is_some());
            }
            other => panic!("expected an upgrade declaration, got {other:?}"),
        }
    }

    #[test]
    fn codata_declarations_parse_observations() {
        let p = ok(
            "module T where\n  codata Stream (a :: Type) where\n    head :: a\n    tail :: Later (Stream a)\n",
        );
        match &p.module.decls[0] {
            Decl::Codata(r) => {
                assert_eq!(r.name.text, "Stream");
                let names: Vec<&str> = r.fields.iter().map(|f| f.name.text.as_str()).collect();
                assert_eq!(names, vec!["head", "tail"]);
                assert!(r.fields.iter().all(|f| !f.erased));
            }
            other => panic!("expected a codata declaration, got {other:?}"),
        }
        let e = err("module T where\n  codata C where\n    erased x :: Int\n");
        assert!(e.contains("an observation is runtime code"), "{e}");
    }

    #[test]
    fn record_declarations_parse_and_erased_fields_parse() {
        let p = ok("module T where\n  record R where\n    x :: Int\n    y :: Bool\n");
        match &p.module.decls[0] {
            Decl::Record(r) => {
                assert_eq!(r.name.text, "R");
                let names: Vec<&str> = r.fields.iter().map(|f| f.name.text.as_str()).collect();
                assert_eq!(names, vec!["x", "y"]);
                assert!(r.fields.iter().all(|f| !f.erased));
            }
            other => panic!("expected a record, got {other:?}"),
        }
        let p = ok("module T where\n  record R where\n    x :: Int\n    erased bound :: x < 3\n");
        match &p.module.decls[0] {
            Decl::Record(r) => {
                let erased: Vec<bool> = r.fields.iter().map(|f| f.erased).collect();
                assert_eq!(erased, vec![false, true]);
            }
            other => panic!("expected a record, got {other:?}"),
        }
    }

    #[test]
    fn not_supported_terms_are_named() {
        let e = err("module T where\n  f = 1.5\n");
        assert!(e.contains("float literals are not supported"), "{e}");
        // A macro invocation needs a balanced token tree.
        let e = err("module T where\n  f = m! x\n");
        assert!(e.contains("needs a balanced token tree"), "{e}");
    }

    #[test]
    fn staging_forms_parse() {
        // Quotations, splices, `quoteDecls`, macro calls, `generate do`,
        // `import stage`, `stage`/`macro` modifiers, and attributes all
        // parse (staging, P9).
        let p = ok(
            "module T where\n\
             \x20 import stage Lib\n\
             \x20 stage addOne :: Code Nat -> Code Nat\n\
             \x20 stage addOne c = [| let x = $(c) in x |]\n\
             \x20 macro decorate attributes (codec) :: Syntax -> Declarations\n\
             \x20 macro decorate x = quoteDecls where\n\
             \x20   #[codec]\n\
             \x20   generated :: Int\n\
             \x20   generated = $(x)\n\
             \x20 generate do\n\
             \x20   emit (decorate!(41))\n\
             \x20 main :: Int\n\
             \x20 main = decorate!(1 + 1)\n",
        );
        let m = &p.module;
        assert!(m.imports.iter().any(|i| i.stage), "expected a stage import");
        let add_one = eq_body(m, "addOne");
        match add_one {
            Term::Quote { body, .. } => match &**body {
                Term::Let(_, inner, _) => {
                    assert!(matches!(**inner, Term::Var(_)), "got {inner:?}");
                }
                other => panic!("expected a let in the quote, got {other:?}"),
            },
            other => panic!("expected a quotation, got {other:?}"),
        }
        let decorate = eq_body(m, "decorate");
        match decorate {
            Term::QuoteDecls { decls, .. } => {
                assert_eq!(decls.len(), 2);
                assert!(matches!(decls[0], Decl::Attributed { .. }));
            }
            other => panic!("expected quoteDecls, got {other:?}"),
        }
        assert!(
            m.decls.iter().any(|d| matches!(d, Decl::Generate { .. })),
            "expected a generate block"
        );
        match eq_body(m, "main") {
            Term::MacroCall { path, args, .. } => {
                assert_eq!(path.name.text, "decorate");
                assert_eq!(args.arg_groups().len(), 1);
            }
            other => panic!("expected a macro call, got {other:?}"),
        }
    }

    #[test]
    fn effects_handlers_do_and_capabilities_parse() {
        let p = ok(
            "module T where\n  effect Poke where\n    poke :: Unit ->{Poke} Int\n\n  run :: forall r. using (io :: Io r) => Unit ->{Poke, IO r} Int\n  run u = with (io = newIO ()) in handle poke () with (handler Poke where\n    return x -> x\n    poke t resume k -> k 1)\n\n  call :: forall r. using (io :: Io r) => Unit ->{IO r} Unit\n  call u = run @{io = io} u2\n    where u2 = ()\n\n  seq :: Int -> Int\n  seq n = do\n    x <- n\n    pure x\n",
        );
        let m = &p.module;
        assert!(
            m.decls.iter().any(|d| matches!(d, Decl::Effect(_))),
            "expected an effect declaration"
        );
        let run = eq_body(m, "run");
        assert!(matches!(run, Term::With { .. }), "expected a with supply, got {run:?}");
        let call = eq_body(m, "call");
        assert!(
            matches!(call, Term::App(f, _, _) if matches!(**f, Term::CapApply { .. })),
            "expected capability supplies, got {call:?}"
        );
        match eq_body(m, "seq") {
            Term::Do(stmts, _) => {
                assert_eq!(stmts.len(), 2);
                assert!(matches!(stmts[0], DoStmt::Bind { .. }));
                assert!(matches!(stmts[1], DoStmt::Expr { .. }));
            }
            other => panic!("expected a do block, got {other:?}"),
        }
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
        let e = err("module T :: Sig where\n  f = 1\n");
        assert!(e.contains("module signatures are not supported"), "{e}");
        let e = err("module T where\n  module U where\n    f = 1\n");
        assert!(e.contains("nested module declarations"), "{e}");
        let e = err("module T where\n  import M (d as e) as N\n  f = 1\n");
        assert!(
            e.contains("a member selection and a module alias cannot combine"),
            "{e}"
        );
        let e = err("module T where\n  import M (3)\n  f = 1\n");
        assert!(e.contains("expected a member name"), "{e}");
    }

    #[test]
    fn import_renaming_and_exports_parse() {
        // `import X (a as b, c)` selects and renames members (surface
        // grammar local amendments section 11).
        let p = ok("module T where\n  import M (d as e, f)\n  f = 1\n");
        let imp = match &p.module.decls[0] {
            Decl::Import(i) => i.clone(),
            other => panic!("expected an import, got {other:?}"),
        };
        assert_eq!(imp.members.len(), 2);
        assert_eq!(imp.members[0].name.text, "d");
        assert_eq!(
            imp.members[0].as_name.as_ref().map(|a| a.text.as_str()),
            Some("e")
        );
        assert_eq!(imp.members[1].name.text, "f");
        assert!(imp.members[1].as_name.is_none());
        // `import X as Y` renames the module alias.
        let p = ok("module T where\n  import M as N\n  f = 1\n");
        let imp = match &p.module.decls[0] {
            Decl::Import(i) => i.clone(),
            other => panic!("expected an import, got {other:?}"),
        };
        assert_eq!(imp.alias.as_ref().map(|a| a.text.as_str()), Some("N"));
        assert!(imp.members.is_empty());
        // `export name, ...`, `export A.b`, `export module A`.
        let p = ok("module T where\n  import M as N\n  export module N, helper, N.thing\n  f = 1\n");
        let items = match &p.module.decls[1] {
            Decl::Export(e) => e.items.clone(),
            other => panic!("expected an export, got {other:?}"),
        };
        assert_eq!(items.len(), 3);
        assert!(matches!(&items[0], ExportItem::Module(n) if n.text == "N"));
        assert!(matches!(&items[1], ExportItem::Name(n) if n.text == "helper"));
        assert!(matches!(&items[2], ExportItem::Member(p) if p.parts() == ["N", "thing"]));
        // `import data "config.json" as config`.
        let p = ok("module T where\n  import data \"config.json\" as config\n  f = 1\n");
        match &p.module.decls[0] {
            Decl::ImportData(d) => {
                assert_eq!(d.file, "config.json");
                assert_eq!(d.name.text, "config");
            }
            other => panic!("expected a data import, got {other:?}"),
        }
    }

    #[test]
    fn module_parameters_and_instantiations_parse() {
        // `module M (x :: T, sig :: S) where` (module parameters as module
        // arguments) and `import M (args) as N` (instantiation).
        let p = ok("module T (Seq :: ResultSequencing ConfigError, k :: Int) where\n  f = 1\n");
        let names: Vec<&str> = p.module.params.iter().map(|x| x.name.text.as_str()).collect();
        assert_eq!(names, vec!["Seq", "k"]);
        let p = ok("module T where\n  import M (StepUp, 3) as N\n  f = 1\n");
        let imp = match &p.module.decls[0] {
            Decl::Import(i) => i.clone(),
            other => panic!("expected an import, got {other:?}"),
        };
        assert_eq!(imp.path.parts(), vec!["M"]);
        assert_eq!(imp.alias.as_ref().map(|a| a.text.as_str()), Some("N"));
        assert_eq!(imp.args.len(), 2);
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
        // The prelude is the split family: the facade's curated
        // re-export surface plus the scheme homes carrying the
        // declarations (the prelude split).
        let p = ok(include_str!("../prelude/Prelude.lang"));
        let m = &p.module;
        assert_eq!(m.name.text, "Prelude");
        let exports = m
            .decls
            .iter()
            .filter(|d| matches!(d, Decl::Export(_)))
            .count();
        assert!(exports >= 20, "expected the curated export surface, got {exports}");
        let base = p.fixities.clone();
        let mut datas: Vec<String> = Vec::new();
        let mut foreigns = 0;
        let mut insts: Vec<InstDecl> = Vec::new();
        for (_path, src) in crate::driver::PRELUDE_EARLY
            .iter()
            .chain(crate::driver::PRELUDE_LATE.iter())
        {
            let toks = crate::lexer::lex(src).expect("lex the family");
            let toks = crate::layout::layout(toks).expect("lay out the family");
            let pm = parse_module(&toks, &base).expect("parse should succeed");
            for d in &pm.module.decls {
                match d {
                    Decl::Data(x) => datas.push(x.name.text.clone()),
                    Decl::Foreign(_) => foreigns += 1,
                    Decl::Instance(x) => insts.push(x.clone()),
                    _ => {}
                }
            }
        }
        // The prelude grows with the language; pin the essentials only.
        for required in ["Option", "List", "Ordering"] {
            assert!(
                datas.iter().any(|d| d == required),
                "missing data {required} in {datas:?}"
            );
        }
        assert!(foreigns >= 17, "expected the primitive surface, got {foreigns}");
        assert!(insts.len() >= 7, "expected the class instances, got {}", insts.len());
        assert!(insts.iter().all(|i| i.implicit));
        assert!(
            insts.iter().any(|i| !i.methods.is_empty() && i.methods[0].name.text == "=="),
            "expected an Eq instance"
        );
        // The operator table (surface grammar local amendments section
        // 6): Rust's relative precedences in the 0..=9 band.
        assert_eq!(p.fixities.get("+"), Some((Assoc::Left, 8)));
        assert_eq!(p.fixities.get("*"), Some((Assoc::Left, 9)));
        assert_eq!(p.fixities.get("=="), Some((Assoc::None, 3)));
        assert_eq!(p.fixities.get("&&"), Some((Assoc::Right, 2)));
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


// -------------------------------------------- WIT: component interface grammar

/// Parse a WIT package (WebAssembly and WASI specification sections 3
/// and 4): the `package` header with its `interface` and `world` items.
/// The grammar is the WIT reference's interface subset: type definitions
/// (`type`, `enum`, `record`, `variant`, `flags`, `union`, `resource`),
/// `func` declarations and `world` import/export lists. `use` and
/// `include` are rejected with a diagnostic until a slice needs them.
pub fn parse_wit(src: &str) -> Result<WitPackage> {
    let toks = lex_wit(src)?;
    let mut p = WitParser { toks, pos: 0, src };
    p.package()
}

/// One WIT token with its source position.
#[derive(Clone, Debug, PartialEq, Eq)]
struct WitTok {
    kind: WitTokKind,
    line: u32,
    col: u32,
    /// Byte offsets into the source, for raw slices (version strings).
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum WitTokKind {
    /// `[A-Za-z0-9_-]+`, including WIT keywords and kebab-case names.
    Ident(String),
    /// A run of decimal digits.
    Num(String),
    At,
    Colon,
    Eq,
    Semi,
    Comma,
    Dot,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Lt,
    Gt,
    Arrow,
}

/// The WIT scanner: ASCII identifiers (kebab-case allowed), the single
/// reserved punctuation of the grammar, and `//` line comments.
fn lex_wit(src: &str) -> Result<Vec<WitTok>> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let (mut i, mut line, mut col) = (0usize, 1u32, 1u32);
    while i < b.len() {
        let start = i;
        let (sl, sc) = (line, col);
        let c = b[i];
        let kind = match c {
            b' ' | b'\t' | b'\r' => {
                i += 1;
                col += 1;
                continue;
            }
            b'\n' => {
                i += 1;
                line += 1;
                col = 1;
                continue;
            }
            b'/' if i + 1 < b.len() && b[i + 1] == b'/' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'@' => {
                i += 1;
                col += 1;
                WitTokKind::At
            }
            b'=' => {
                i += 1;
                col += 1;
                WitTokKind::Eq
            }
            b':' => {
                i += 1;
                col += 1;
                WitTokKind::Colon
            }
            b';' => {
                i += 1;
                col += 1;
                WitTokKind::Semi
            }
            b',' => {
                i += 1;
                col += 1;
                WitTokKind::Comma
            }
            b'.' => {
                i += 1;
                col += 1;
                WitTokKind::Dot
            }
            b'{' => {
                i += 1;
                col += 1;
                WitTokKind::LBrace
            }
            b'}' => {
                i += 1;
                col += 1;
                WitTokKind::RBrace
            }
            b'(' => {
                i += 1;
                col += 1;
                WitTokKind::LParen
            }
            b')' => {
                i += 1;
                col += 1;
                WitTokKind::RParen
            }
            b'<' => {
                i += 1;
                col += 1;
                WitTokKind::Lt
            }
            b'>' => {
                i += 1;
                col += 1;
                WitTokKind::Gt
            }
            b'-' if i + 1 < b.len() && b[i + 1] == b'>' => {
                i += 2;
                col += 2;
                WitTokKind::Arrow
            }
            b'0'..=b'9' => {
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                    col += 1;
                }
                WitTokKind::Num(src[start..i].to_string())
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => {
                while i < b.len()
                    && (b[i].is_ascii_alphanumeric() || b[i] == b'_' || b[i] == b'-')
                {
                    i += 1;
                    col += 1;
                }
                WitTokKind::Ident(src[start..i].to_string())
            }
            _ => {
                return Err(Error::at(
                    Span { line: sl, col: sc },
                    format!(
                        "WIT: unexpected character {:?}",
                        src[i..].chars().next().unwrap_or('?')
                    ),
                ))
            }
        };
        out.push(WitTok { kind, line: sl, col: sc, start, end: i });
    }
    Ok(out)
}

struct WitParser<'a> {
    toks: Vec<WitTok>,
    pos: usize,
    src: &'a str,
}

impl<'a> WitParser<'a> {
    fn peek(&self) -> Option<&WitTok> {
        self.toks.get(self.pos)
    }

    fn span(&self) -> Span {
        match self.peek() {
            Some(t) => Span { line: t.line, col: t.col },
            None => self
                .toks
                .last()
                .map(|t| Span { line: t.line, col: t.col + (t.end - t.start) as u32 })
                .unwrap_or(Span { line: 1, col: 1 }),
        }
    }

    fn err<T>(&self, msg: impl Into<String>) -> Result<T> {
        Err(Error::at(self.span(), msg))
    }

    fn bump(&mut self) -> Option<WitTok> {
        let t = self.toks.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn at_ident(&self, kw: &str) -> bool {
        matches!(self.peek(), Some(t) if t.kind == WitTokKind::Ident(kw.to_string()))
    }

    fn eat_ident(&mut self, kw: &str) -> bool {
        if self.at_ident(kw) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect_ident(&mut self, kw: &str) -> Result<()> {
        if self.eat_ident(kw) {
            Ok(())
        } else {
            self.err(format!("WIT: expected `{kw}`, found {}", self.describe()))
        }
    }

    fn at_punct(&self, k: &WitTokKind) -> bool {
        matches!(self.peek(), Some(t) if &t.kind == k)
    }

    fn eat_punct(&mut self, k: &WitTokKind) -> bool {
        if self.at_punct(k) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect_punct(&mut self, k: &WitTokKind) -> Result<()> {
        if self.eat_punct(k) {
            Ok(())
        } else {
            self.err(format!("WIT: expected `{}`, found {}", punct_spelling(k), self.describe()))
        }
    }

    fn describe(&self) -> String {
        match self.peek() {
            None => "end of input".to_string(),
            Some(t) => match &t.kind {
                WitTokKind::Ident(s) => format!("`{s}`"),
                WitTokKind::Num(s) => format!("number `{s}`"),
                k => format!("`{}`", punct_spelling(k)),
            },
        }
    }

    /// A kebab-case or snake_case name: any identifier, including `_`.
    fn name(&mut self, what: &str) -> Result<(String, Span)> {
        let span = self.span();
        match self.bump() {
            Some(WitTok { kind: WitTokKind::Ident(n), .. }) => Ok((n, span)),
            _ => self.err(format!("WIT: expected a {what} name")),
        }
    }

    /// `ns:name` with the `@version` suffix when present.
    fn package(&mut self) -> Result<WitPackage> {
        let span = self.span();
        self.expect_ident("package")?;
        let (ns, _) = self.name("package namespace")?;
        self.expect_punct(&WitTokKind::Colon)?;
        let (name, _) = self.name("package")?;
        let version = if self.eat_punct(&WitTokKind::At) {
            let t = self.span();
            let start = match self.peek() {
                Some(t) => t.start,
                None => return Err(Error::at(t, "WIT: expected a version after `@`")),
            };
            // The version runs to the `;`: digits, dots and a possible
            // pre-release suffix.
            let mut end = start;
            loop {
                let t = match self.peek() {
                    Some(t) => t.clone(),
                    None => break,
                };
                match &t.kind {
                    WitTokKind::Semi => break,
                    WitTokKind::Num(_) | WitTokKind::Dot | WitTokKind::At => {
                        end = t.end;
                        self.pos += 1;
                    }
                    WitTokKind::Ident(_)
                        if t.start == end || self.src[end..t.start].ends_with('-') =>
                    {
                        end = t.end;
                        self.pos += 1;
                    }
                    _ => {
                        return Err(Error::at(
                            Span { line: t.line, col: t.col },
                            "WIT: malformed package version",
                        ))
                    }
                }
            }
            Some(self.src[start..end].trim_end().to_string())
        } else {
            None
        };
        self.expect_punct(&WitTokKind::Semi)?;
        let mut interfaces = Vec::new();
        let mut worlds = Vec::new();
        while self.peek().is_some() {
            if self.at_ident("interface") {
                interfaces.push(self.interface()?);
            } else if self.at_ident("world") {
                worlds.push(self.world()?);
            } else if self.at_ident("use") || self.at_ident("include") {
                return self.err(format!(
                    "WIT: `{}` is not supported in this package subset",
                    if self.at_ident("use") { "use" } else { "include" }
                ));
            } else {
                return self.err(format!(
                    "WIT: expected `interface` or `world`, found {}",
                    self.describe()
                ));
            }
        }
        Ok(WitPackage {
            name: format!("{ns}:{name}"),
            version,
            interfaces,
            worlds,
            span,
        })
    }

    fn interface(&mut self) -> Result<WitInterface> {
        let span = self.span();
        self.expect_ident("interface")?;
        let (name, _) = self.name("interface")?;
        self.expect_punct(&WitTokKind::LBrace)?;
        let mut items = Vec::new();
        while !self.at_punct(&WitTokKind::RBrace) {
            if self.peek().is_none() {
                return self.err("WIT: unterminated interface body");
            }
            items.push(self.item()?);
        }
        self.expect_punct(&WitTokKind::RBrace)?;
        Ok(WitInterface { name, items, span })
    }

    fn item(&mut self) -> Result<WitItem> {
        if self.at_ident("type") {
            self.pos += 1;
            let (name, span) = self.name("type")?;
            self.expect_punct(&WitTokKind::Eq)?;
            let ty = self.ty()?;
            self.expect_punct(&WitTokKind::Semi)?;
            return Ok(WitItem::Type(WitTypeDef { name, kind: WitTypeDefKind::Alias(ty), span }));
        }
        if self.at_ident("enum") {
            let td = self.typedef_cases(WitTokKind::Ident("enum".into()), false)?;
            return Ok(WitItem::Type(td));
        }
        if self.at_ident("variant") {
            let td = self.typedef_cases(WitTokKind::Ident("variant".into()), true)?;
            return Ok(WitItem::Type(td));
        }
        if self.at_ident("flags") {
            let span = self.span();
            self.expect_ident("flags")?;
            let (name, _) = self.name("flags type")?;
            self.expect_punct(&WitTokKind::LBrace)?;
            let mut cases = Vec::new();
            while !self.at_punct(&WitTokKind::RBrace) {
                let (c, _) = self.name("flag")?;
                cases.push(c);
                if !self.eat_punct(&WitTokKind::Comma) {
                    break;
                }
            }
            self.expect_punct(&WitTokKind::RBrace)?;
            return Ok(WitItem::Type(WitTypeDef {
                name,
                kind: WitTypeDefKind::Flags(cases),
                span,
            }));
        }
        if self.at_ident("union") {
            let span = self.span();
            self.expect_ident("union")?;
            let (name, _) = self.name("union type")?;
            self.expect_punct(&WitTokKind::LBrace)?;
            let mut cases = Vec::new();
            while !self.at_punct(&WitTokKind::RBrace) {
                cases.push(self.ty()?);
                if !self.eat_punct(&WitTokKind::Comma) {
                    break;
                }
            }
            self.expect_punct(&WitTokKind::RBrace)?;
            return Ok(WitItem::Type(WitTypeDef {
                name,
                kind: WitTypeDefKind::Union(cases),
                span,
            }));
        }
        if self.at_ident("record") {
            let span = self.span();
            self.expect_ident("record")?;
            let (name, _) = self.name("record type")?;
            self.expect_punct(&WitTokKind::LBrace)?;
            let mut fields = Vec::new();
            while !self.at_punct(&WitTokKind::RBrace) {
                let (fnm, fspan) = self.name("field")?;
                self.expect_punct(&WitTokKind::Colon)?;
                let ty = self.ty()?;
                fields.push(WitField { name: fnm, ty, span: fspan });
                if !self.eat_punct(&WitTokKind::Comma) {
                    break;
                }
            }
            self.expect_punct(&WitTokKind::RBrace)?;
            return Ok(WitItem::Type(WitTypeDef {
                name,
                kind: WitTypeDefKind::Record(fields),
                span,
            }));
        }
        if self.at_ident("resource") {
            return Ok(WitItem::Resource(self.resource()?));
        }
        // `name: func(x: t, ...) -> u;`
        let (name, span) = self.name("function")?;
        self.expect_punct(&WitTokKind::Colon)?;
        self.expect_ident("func")?;
        let (params, result) = self.func_sig()?;
        self.expect_punct(&WitTokKind::Semi)?;
        Ok(WitItem::Func(WitFunc { name, params, result, span }))
    }

    /// `enum e { a, b }` or `variant v { a(t), b }`.
    fn typedef_cases(&mut self, _kw: WitTokKind, payloads: bool) -> Result<WitTypeDef> {
        let span = self.span();
        let kw = if payloads { "variant" } else { "enum" };
        self.expect_ident(kw)?;
        let (name, _) = self.name("type")?;
        self.expect_punct(&WitTokKind::LBrace)?;
        let mut cases = Vec::new();
        while !self.at_punct(&WitTokKind::RBrace) {
            let (c, cspan) = self.name("case")?;
            let payload = if payloads && self.at_punct(&WitTokKind::LParen) {
                self.pos += 1;
                let mut tys = Vec::new();
                while !self.at_punct(&WitTokKind::RParen) {
                    tys.push(self.ty()?);
                    if !self.eat_punct(&WitTokKind::Comma) {
                        break;
                    }
                }
                self.expect_punct(&WitTokKind::RParen)?;
                match tys.len() {
                    0 => None,
                    1 => Some(tys.remove(0)),
                    _ => Some(WitType::Tuple(tys, cspan)),
                }
            } else {
                None
            };
            if !payloads && payload.is_some() {
                return Err(Error::at(cspan, "WIT: an enum case has no payload"));
            }
            cases.push(WitCase { name: c, payload, span: cspan });
            if !self.eat_punct(&WitTokKind::Comma) {
                break;
            }
        }
        self.expect_punct(&WitTokKind::RBrace)?;
        Ok(WitTypeDef {
            name,
            kind: if payloads {
                WitTypeDefKind::Variant(cases)
            } else {
                WitTypeDefKind::Enum(cases)
            },
            span,
        })
    }

    fn resource(&mut self) -> Result<WitResource> {
        let span = self.span();
        self.expect_ident("resource")?;
        let (name, _) = self.name("resource")?;
        let mut items = Vec::new();
        if self.eat_punct(&WitTokKind::LBrace) {
            while !self.at_punct(&WitTokKind::RBrace) {
                if self.at_ident("constructor") {
                    let cspan = self.span();
                    self.pos += 1;
                    let (params, result) = self.func_sig()?;
                    if result.is_some() {
                        return Err(Error::at(cspan, "WIT: a constructor has no result"));
                    }
                    self.expect_punct(&WitTokKind::Semi)?;
                    items.push(WitResourceItem::Constructor { params, span: cspan });
                    continue;
                }
                let is_static = self.eat_ident("static");
                let (mname, mspan) = self.name("method")?;
                self.expect_punct(&WitTokKind::Colon)?;
                self.expect_ident("func")?;
                let (params, result) = self.func_sig()?;
                self.expect_punct(&WitTokKind::Semi)?;
                let _ = is_static;
                items.push(WitResourceItem::Method { name: mname, params, result, span: mspan });
            }
            self.expect_punct(&WitTokKind::RBrace)?;
        } else {
            self.expect_punct(&WitTokKind::Semi)?;
        }
        Ok(WitResource { name, items, span })
    }

    /// `(x: t, y: u) -> v`, the parameter and result list of a `func`.
    fn func_sig(&mut self) -> Result<(Vec<WitParam>, Option<WitType>)> {
        self.expect_punct(&WitTokKind::LParen)?;
        let mut params = Vec::new();
        while !self.at_punct(&WitTokKind::RParen) {
            let (name, span) = self.name("parameter")?;
            self.expect_punct(&WitTokKind::Colon)?;
            let ty = self.ty()?;
            params.push(WitParam { name, ty, span });
            if !self.eat_punct(&WitTokKind::Comma) {
                break;
            }
        }
        self.expect_punct(&WitTokKind::RParen)?;
        let result = if self.eat_punct(&WitTokKind::Arrow) { Some(self.ty()?) } else { None };
        Ok((params, result))
    }

    fn ty(&mut self) -> Result<WitType> {
        let span = self.span();
        let t = match self.bump() {
            Some(WitTok { kind: WitTokKind::Ident(n), .. }) => n,
            _ => return self.err("WIT: expected a type"),
        };
        let simple = match t.as_str() {
            "bool" => Some(WitType::Bool(span)),
            "u8" => Some(WitType::U8(span)),
            "u16" => Some(WitType::U16(span)),
            "u32" => Some(WitType::U32(span)),
            "u64" => Some(WitType::U64(span)),
            "s8" => Some(WitType::S8(span)),
            "s16" => Some(WitType::S16(span)),
            "s32" => Some(WitType::S32(span)),
            "s64" => Some(WitType::S64(span)),
            "f32" => Some(WitType::F32(span)),
            "f64" => Some(WitType::F64(span)),
            "char" => Some(WitType::Char(span)),
            "string" => Some(WitType::Str(span)),
            _ => None,
        };
        if let Some(ty) = simple {
            return Ok(ty);
        }
        match t.as_str() {
            "list" | "option" | "stream" | "future" => {
                self.expect_punct(&WitTokKind::Lt)?;
                let inner = self.ty()?;
                self.expect_punct(&WitTokKind::Gt)?;
                return Ok(match t.as_str() {
                    "list" => WitType::List(Box::new(inner), span),
                    "option" => WitType::Option(Box::new(inner), span),
                    "stream" => WitType::Stream(Box::new(inner), span),
                    _ => WitType::Future(Box::new(inner), span),
                });
            }
            "own" | "borrow" => {
                self.expect_punct(&WitTokKind::Lt)?;
                let (n, _) = self.name("resource")?;
                self.expect_punct(&WitTokKind::Gt)?;
                return Ok(if t == "own" {
                    WitType::Own(n, span)
                } else {
                    WitType::Borrow(n, span)
                });
            }
            // `result` and `tuple` also stand alone (`result`, `tuple<>`).
            "tuple" | "result" => {
                if !self.eat_punct(&WitTokKind::Lt) {
                    return Ok(match t.as_str() {
                        "result" => WitType::Result(None, None, span),
                        _ => WitType::Tuple(Vec::new(), span),
                    });
                }
                if t == "tuple" {
                    let mut tys = Vec::new();
                    while !self.at_punct(&WitTokKind::Gt) {
                        tys.push(self.ty()?);
                        if !self.eat_punct(&WitTokKind::Comma) {
                            break;
                        }
                    }
                    self.expect_punct(&WitTokKind::Gt)?;
                    return Ok(WitType::Tuple(tys, span));
                }
                // `result<t, e>` with `_` for a missing payload.
                let ok = self.underscore_ty()?;
                let err = if self.eat_punct(&WitTokKind::Comma) {
                    self.underscore_ty()?
                } else {
                    None
                };
                self.expect_punct(&WitTokKind::Gt)?;
                return Ok(WitType::Result(ok, err, span));
            }
            _ => {}
        }
        Ok(WitType::Named(t, span))
    }

    /// A `result` payload slot: `_` or a type.
    fn underscore_ty(&mut self) -> Result<Option<Box<WitType>>> {
        if self.at_ident("_") {
            self.pos += 1;
            return Ok(None);
        }
        Ok(Some(Box::new(self.ty()?)))
    }

    fn world(&mut self) -> Result<WitWorld> {
        let span = self.span();
        self.expect_ident("world")?;
        let (name, _) = self.name("world")?;
        self.expect_punct(&WitTokKind::LBrace)?;
        let (mut imports, mut exports) = (Vec::new(), Vec::new());
        while !self.at_punct(&WitTokKind::RBrace) {
            let is_export = if self.eat_ident("export") {
                true
            } else if self.eat_ident("import") {
                false
            } else {
                return self.err(format!(
                    "WIT: expected `import` or `export`, found {}",
                    self.describe()
                ));
            };
            let (mut path, pspan) = self.name("interface")?;
            while self.eat_punct(&WitTokKind::Dot) {
                let (seg, _) = self.name("interface")?;
                path.push('.');
                path.push_str(&seg);
            }
            // The versioned form `ns:name@version` ends with `;`.
            if self.eat_punct(&WitTokKind::Colon) {
                let (seg, _) = self.name("interface")?;
                path.push(':');
                path.push_str(&seg);
                if self.eat_punct(&WitTokKind::At) {
                    let start = match self.peek() {
                        Some(t) => t.start,
                        None => return self.err("WIT: expected a version after `@`"),
                    };
                    let mut end = start;
                    loop {
                        let t = match self.peek() {
                            Some(t) => t.clone(),
                            None => break,
                        };
                        match &t.kind {
                            WitTokKind::Semi => break,
                            WitTokKind::Num(_) | WitTokKind::Dot | WitTokKind::At => {
                                end = t.end;
                                self.pos += 1;
                            }
                            WitTokKind::Ident(_) => {
                                end = t.end;
                                self.pos += 1;
                            }
                            _ => return self.err("WIT: malformed interface version"),
                        }
                    }
                    path.push('@');
                    path.push_str(self.src[start..end].trim_end());
                }
            }
            self.expect_punct(&WitTokKind::Semi)?;
            let item = WitWorldItem { iface: path, span: pspan };
            if is_export {
                exports.push(item);
            } else {
                imports.push(item);
            }
        }
        self.expect_punct(&WitTokKind::RBrace)?;
        Ok(WitWorld { name, imports, exports, span })
    }
}

fn punct_spelling(k: &WitTokKind) -> &'static str {
    match k {
        WitTokKind::At => "@",
        WitTokKind::Colon => ":",
        WitTokKind::Eq => "=",
        WitTokKind::Semi => ";",
        WitTokKind::Comma => ",",
        WitTokKind::Dot => ".",
        WitTokKind::LBrace => "{",
        WitTokKind::RBrace => "}",
        WitTokKind::LParen => "(",
        WitTokKind::RParen => ")",
        WitTokKind::Lt => "<",
        WitTokKind::Gt => ">",
        WitTokKind::Arrow => "->",
        WitTokKind::Ident(_) | WitTokKind::Num(_) => "identifier",
    }
}
