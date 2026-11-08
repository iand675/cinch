//! AST syntax objects shared by quotations and library-authored templates.
//!
//! A template is hygienically prepared before captured trees enter it. The
//! types keep that order explicit, preserve the captures' source spans, and
//! refuse invalid capture indices before the AST substitution routine runs.

use crate::ast::{self, Decl, HygCx, HygScope, LocalDecl, Term};
use crate::{Error, Result};
use std::sync::Arc;

pub(crate) mod declarations;
pub(crate) mod tokens;

/// The descriptive source module and explicitly available lexical binders.
/// A module spelling grants no global or private-name authority. Token
/// captures receive the caller's fixities separately from the template's.
#[derive(Clone, Debug)]
pub struct SyntaxScope {
    module: String,
    values: Vec<ast::Name>,
    globals: Option<Arc<ast::GlobalScope>>,
    dictionaries: Option<Vec<ast::LocalBinding>>,
    declarations: Vec<(ast::RefKind, ast::Name)>,
}

impl SyntaxScope {
    pub(crate) fn from_names(module: impl Into<String>, values: Vec<ast::Name>) -> Self {
        Self {
            module: module.into(),
            values,
            globals: None,
            dictionaries: None,
            declarations: Vec::new(),
        }
    }
    pub fn new(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            values: Vec::new(),
            globals: None,
            dictionaries: None,
            declarations: Vec::new(),
        }
    }
    pub fn module(&self) -> &str {
        &self.module
    }

    pub(crate) fn with_globals(mut self, globals: Arc<ast::GlobalScope>) -> Self {
        self.module = globals.module().into();
        self.globals = Some(match &self.dictionaries {
            Some(dictionaries) if !globals.has_dictionary_bindings() => Arc::new((*globals).clone().with_dictionary_bindings(dictionaries.clone())),
            _ => globals,
        });
        self
    }

    pub(crate) fn with_dictionary_bindings(mut self, dictionaries: Vec<ast::LocalBinding>) -> Self {
        self.dictionaries = Some(dictionaries); self
    }

    pub(crate) fn with_declarations(mut self, declarations: Vec<(ast::RefKind, ast::Name)>) -> Self {
        self.declarations = declarations;
        self
    }

    /// Token groups can retain their checked value names before their macro
    /// invocation is elaborated. Retain those names while attaching the actual
    /// invocation's dictionary capabilities, which exist only at elaboration.
    pub(crate) fn with_lexical_names(mut self, values: Vec<ast::Name>) -> Self {
        self.values = values;
        self
    }

    /// Introduce a new lexical binder. A module tag alone grants no existing
    /// local binding: only this returned scope can resolve the new handle.
    pub fn bind_value(&self, spelling: impl Into<String>, span: ast::Span) -> (Self, ScopedValue) {
        let name = ast::Name::bound(spelling, span, ast::LocalBinding::fresh());
        let mut scope = self.clone();
        scope.values.push(name.clone());
        (
            scope,
            ScopedValue {
                name,
                outer: self.clone(),
            },
        )
    }

    /// Deliberately introduce a library-authored binder in this source's
    /// lexical scope. The source's global authority and fixities stay its own.
    pub(crate) fn under_binding(&self, binding: &ScopedValue) -> Self {
        let mut scope = self.clone();
        scope.values.push(binding.name.clone());
        scope
    }

    /// Capture a reference now, before moving it beneath other binders.
    pub fn reference(&self, spelling: &str, span: ast::Span) -> Result<CapturedTerm> {
        let mut name = self
            .values
            .iter()
            .rev()
            .find(|n| n.text == spelling)
            .cloned()
            .ok_or_else(|| {
                Error::at(
                    span,
                    format!("no lexical syntax binding `{spelling}` in this scope"),
                )
            })?;
        name.span = span;
        Ok(CapturedTerm::new(
            Term::Var(ast::VarPath {
                quals: vec![],
                name,
            }),
            self.clone(),
        ))
    }
}

impl PartialEq for SyntaxScope {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
            && self.globals == other.globals
            && self.dictionaries == other.dictionaries
            && self.declarations.len() == other.declarations.len()
            && self.declarations.iter().zip(&other.declarations).all(|((ak, a), (bk, b))|
                ak == bk && a.text == b.text && a.declaration_binding() == b.declaration_binding())
            && self.values.len() == other.values.len()
            && self
                .values
                .iter()
                .zip(&other.values)
                .all(|(a, b)| a.text == b.text && a.local_binding() == b.local_binding())
    }
}
impl Eq for SyntaxScope {}

/// A newly introduced binder and its structural introduction operation.
/// Wrapping existing syntax never retargets references inside that syntax.
#[derive(Clone, Debug)]
pub struct ScopedValue {
    name: ast::Name,
    outer: SyntaxScope,
}

impl ScopedValue {
    pub fn name(&self) -> &ast::Name {
        &self.name
    }

    pub fn reference(&self) -> CapturedTerm {
        CapturedTerm::new(
            Term::Var(ast::VarPath {
                quals: Vec::new(),
                name: self.name.clone(),
            }),
            self.outer.under_binding(self),
        )
    }

    pub fn let_in(&self, value: CapturedTerm, body: CapturedTerm) -> CapturedTerm {
        let span = self.name.span;
        CapturedTerm::new(
            Term::Let(
                vec![LocalDecl::Bind {
                    pat: ast::Pat::Var(self.name.clone(), span),
                    body: value.into_datum(),
                    span,
                }],
                Box::new(body.into_datum()),
                span,
            ),
            self.outer.clone(),
        )
    }

    /// Bind this handle in a lambda's body without recapturing that body.
    /// References captured outside the handle keep their original identity.
    pub fn lambda(&self, body: CapturedTerm) -> CapturedTerm {
        let span = self.name.span;
        CapturedTerm::new(
            Term::Lam(
                vec![ast::Pat::Var(self.name.clone(), span)],
                Box::new(body.into_datum()),
                span,
            ),
            self.outer.clone(),
        )
    }
}

/// Caller syntax is a tree, not text to be interpolated and reparsed under
/// the template's binders. Its names and source spans survive substitution.
#[derive(Clone, Debug)]
pub struct CapturedTerm {
    datum: Term,
    scope: SyntaxScope,
}

impl CapturedTerm {
    /// Preserve an existing tree's bindings. To resolve raw parsed names in a
    /// lexical scope, use `capture` or `parse` instead.
    pub fn new(datum: Term, scope: SyntaxScope) -> Self {
        Self { datum, scope }
    }

    pub fn capture(datum: &Term, scope: SyntaxScope) -> Result<Self> {
        let datum = ast::capture_term_in_declarations(datum, &scope.values, scope.globals.clone(), &scope.declarations)?;
        Ok(Self::new(datum, scope))
    }
    pub fn datum(&self) -> &Term {
        &self.datum
    }
    pub fn scope(&self) -> &SyntaxScope {
        &self.scope
    }
    pub fn into_datum(self) -> Term {
        self.datum
    }

    pub fn parse(
        tokens: &[crate::lexer::Token],
        scope: SyntaxScope,
        fixities: &crate::parser::FixityTable,
    ) -> Result<Self> {
        Self::capture(&crate::parser::parse_term_tokens(tokens, fixities)?, scope)
    }
}

/// A type tree captured with the same namespace and phase rules as native
/// quotation annotations. Construction never resolves a printed type name in
/// the declaration's eventual insertion scope.
#[derive(Clone, Debug)]
pub struct CapturedType {
    datum: ast::Type,
    scope: SyntaxScope,
}

impl CapturedType {
    pub fn capture(datum: &ast::Type, scope: SyntaxScope) -> Result<Self> {
        Self::in_declaration(datum, scope, &[], &[])
    }
    pub(crate) fn in_declaration(
        datum: &ast::Type, scope: SyntaxScope, own_types: &[String], preceding_fields: &[String],
    ) -> Result<Self> {
        let datum = ast::capture_type_in_declaration_scope(datum, &scope.values, scope.globals.clone(), own_types, preceding_fields, &scope.declarations)?;
        Ok(Self { datum, scope })
    }
    pub fn datum(&self) -> &ast::Type { &self.datum }
    pub fn scope(&self) -> &SyntaxScope { &self.scope }
    pub fn into_datum(self) -> ast::Type { self.datum }
}

/// Parsed template syntax with explicit capture slots. Authoring uses an AST
/// quotation; there is no identifier discovery or replacement in strings.
#[derive(Clone, Debug)]
pub struct TermTemplate {
    datum: Term,
    captures: usize,
    scope: SyntaxScope,
}

impl TermTemplate {
    pub fn new(datum: Term, captures: usize, scope: SyntaxScope) -> Result<Self> {
        validate_slots(&datum, captures)?;
        Ok(Self {
            datum,
            captures,
            scope,
        })
    }

    pub fn prepare(self, cx: &mut HygCx<'_>) -> Result<PreparedTerm> {
        let datum = ast::hyg_term(&self.datum, cx, &mut HygScope::new())?;
        Ok(PreparedTerm {
            datum,
            captures: self.captures,
            scope: self.scope,
        })
    }
}

/// Only a prepared template accepts captures. This uses the native quotation
/// binding walk; copied syntax is never passed through that walk.
#[derive(Clone, Debug)]
pub struct PreparedTerm {
    datum: Term,
    captures: usize,
    scope: SyntaxScope,
}

impl PreparedTerm {
    /// Compiler provenance stamping changes spans, without reconstructing
    /// names or exposing the template to a text renderer.
    pub(crate) fn stamp(&mut self, apply: impl FnOnce(&mut Term)) {
        apply(&mut self.datum);
    }

    pub fn substitute(self, captures: &[CapturedTerm]) -> Result<CapturedTerm> {
        if captures.len() != self.captures {
            return Err(Error::at(
                self.datum.span(),
                format!(
                    "syntax template expects {} captures, received {}",
                    self.captures,
                    captures.len()
                ),
            ));
        }
        validate_slots(&self.datum, captures.len())?;
        let values: Vec<_> = captures.iter().map(|value| value.datum.clone()).collect();
        Ok(CapturedTerm::new(
            ast::subst_holes(&self.datum, &values),
            self.scope,
        ))
    }
}

fn validate_slots(term: &Term, count: usize) -> Result<()> {
    walk_term(term, &mut |node| {
        if let Term::SpliceHole { idx, span } = node {
            if *idx >= count {
                return Err(Error::at(
                    *span,
                    format!("syntax capture {idx} is outside {count} capture slots"),
                ));
            }
        }
        Ok(())
    })
}

/// Complete read-only traversal of expression positions, including nested
/// declaration quotations. It also supports later syntax provenance queries.
pub fn walk_term(term: &Term, visit: &mut impl FnMut(&Term) -> Result<()>) -> Result<()> {
    visit(term)?;
    match term {
        Term::Var(_)
        | Term::Con(_)
        | Term::Int(..)
        | Term::Str(..)
        | Term::Char(..)
        | Term::Sym(..)
        | Term::Unit(_)
        | Term::Hole(_)
        | Term::SeqOp(..)
        | Term::SpliceHole { .. }
        | Term::MacroCall { .. } => {}
        Term::App(head, args, _) => {
            walk_term(head, visit)?;
            for arg in args {
                walk_term(arg, visit)?;
            }
        }
        Term::Lam(_, body, _)
        | Term::Ann(body, _, _)
        | Term::Neg(body, _)
        | Term::Proj(body, _, _)
        | Term::CpsWrap(body, _)
        | Term::Quote { body, .. }
        | Term::Splice { body, .. }
        | Term::Forget { value: body, .. } => walk_term(body, visit)?,
        Term::Let(decls, body, _) | Term::Letrec(decls, body, _) => {
            for decl in decls {
                walk_local(decl, visit)?;
            }
            walk_term(body, visit)?;
        }
        Term::If {
            cond, then, els, ..
        } => {
            walk_term(cond, visit)?;
            walk_term(then, visit)?;
            walk_term(els, visit)?;
        }
        Term::Case(scrutinee, alts, _) => {
            walk_term(scrutinee, visit)?;
            for alt in alts {
                if let Some(guard) = &alt.guard {
                    walk_term(guard, visit)?;
                }
                walk_term(&alt.body, visit)?;
            }
        }
        Term::Narrow { value, witness, .. } => {
            walk_term(value, visit)?;
            walk_term(witness, visit)?;
        }
        Term::Infix(values, _, _) | Term::Tuple(values, _) | Term::List(values, _) => {
            for value in values {
                walk_term(value, visit)?;
            }
        }
        Term::Map(entries, _) => {
            for (key, value) in entries {
                walk_term(key, visit)?;
                walk_term(value, visit)?;
            }
        }
        Term::Range(start, step, end, _) => {
            walk_term(start, visit)?;
            if let Some(step) = step {
                walk_term(step, visit)?;
            }
            walk_term(end, visit)?;
        }
        Term::Record(_, fields, _) => {
            for (_, value) in fields {
                walk_term(value, visit)?;
            }
        }
        Term::Do(statements, _) => {
            for statement in statements {
                match statement {
                    ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                        walk_term(term, visit)?
                    }
                    ast::DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            walk_local(decl, visit)?;
                        }
                    }
                }
            }
        }
        Term::Handle(body, handler, _) => {
            walk_term(body, visit)?;
            walk_term(handler, visit)?;
        }
        Term::Handler(handler) => {
            walk_term(&handler.ret_body, visit)?;
            for clause in &handler.clauses {
                walk_term(&clause.body, visit)?;
            }
        }
        Term::With { value, body, .. } => {
            walk_term(value, visit)?;
            walk_term(body, visit)?;
        }
        Term::CapApply { head, supplies, .. } => {
            walk_term(head, visit)?;
            for (_, value) in supplies {
                walk_term(value, visit)?;
            }
        }
        Term::QuoteDecls { decls, .. } => {
            for decl in decls {
                walk_decl(decl, visit)?;
            }
        }
    }
    Ok(())
}

fn walk_local(decl: &LocalDecl, visit: &mut impl FnMut(&Term) -> Result<()>) -> Result<()> {
    match decl {
        LocalDecl::Sig { .. } => Ok(()),
        LocalDecl::Eq { body, .. }
        | LocalDecl::Bind { body, .. }
        | LocalDecl::Lazy { body, .. } => walk_term(body, visit),
    }
}

fn walk_equation(eq: &ast::Equation, visit: &mut impl FnMut(&Term) -> Result<()>) -> Result<()> {
    for term in eq.terms() { walk_term(term, visit)?; }
    for local in &eq.locals {
        walk_local(local, visit)?;
    }
    Ok(())
}

fn walk_decl(decl: &Decl, visit: &mut impl FnMut(&Term) -> Result<()>) -> Result<()> {
    match decl {
        Decl::Attributed { inner, .. } => walk_decl(inner, visit)?,
        Decl::NameSpliced { name_tm, inner } => {
            walk_term(name_tm, visit)?;
            walk_decl(inner, visit)?;
        }
        Decl::Equation { eq, .. } => walk_equation(eq, visit)?,
        Decl::Signature(signature) => {
            for eq in &signature.defaults {
                walk_equation(eq, visit)?;
            }
        }
        Decl::Instance(instance) => {
            for eq in &instance.methods {
                walk_equation(eq, visit)?;
            }
        }
        Decl::Rewrite(rewrite) => {
            walk_term(&rewrite.match_tm, visit)?;
            walk_term(&rewrite.replace_tm, visit)?;
            if let Some(require) = &rewrite.require {
                walk_term(require, visit)?;
            }
            walk_term(&rewrite.justify, visit)?;
        }
        Decl::Upgrade(upgrade) => {
            if let Some(migrate) = &upgrade.migrate {
                walk_term(migrate, visit)?;
            }
            if let Some(related) = &upgrade.related {
                walk_term(related, visit)?;
            }
        }
        Decl::Contract(contract) => {
            for clause in &contract.clauses {
                match clause {
                    ast::ContractClause::Formula { body, .. }
                    | ast::ContractClause::AssumeProp { body } => walk_term(body, visit)?,
                    ast::ContractClause::AssumeNamed { .. } => {}
                }
            }
        }
        Decl::Prim(prim) => {
            for body in &prim.bodies {
                if let Some(when) = &body.when {
                    walk_term(when, visit)?;
                }
                if let Some(justify) = &body.justify {
                    walk_term(justify, visit)?;
                }
            }
        }
        Decl::Generate { .. } => {
            return Err(Error::plain(
                "a syntax template cannot contain a generate declaration",
            ))
        }
        Decl::Import(_)
        | Decl::ImportData(_)
        | Decl::Export(_)
        | Decl::Fixity(_)
        | Decl::OpaqueType(_)
        | Decl::Foreign(_)
        | Decl::Data(_)
        | Decl::Refine(_)
        | Decl::Record(_)
        | Decl::Codata(_)
        | Decl::Effect(_)
        | Decl::ValueSig { .. } => {}
    }
    Ok(())
}
