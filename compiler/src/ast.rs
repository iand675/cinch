//! Abstract syntax for the draft 0.2 surface, restricted to the bootstrap
//! subset (see `compiler/README.md`). Unsupported forms parse and are
//! rejected by the checker or parser with an explicit message.

pub use crate::lexer::Span;
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct Name {
    pub text: String,
    pub span: Span,
}

impl Name {
    pub fn new(text: impl Into<String>, span: Span) -> Self {
        Name { text: text.into(), span }
    }
}

/// `con_id { "." con_id }`.
#[derive(Clone, Debug)]
pub struct ConPath {
    pub segments: Vec<Name>,
}

impl ConPath {
    pub fn span(&self) -> Span {
        self.segments[0].span
    }

    /// `Foo.Bar` -> ["Foo", "Bar"].
    pub fn parts(&self) -> Vec<&str> {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }
}

/// `{ con_id "." } var_id`.
#[derive(Clone, Debug)]
pub struct VarPath {
    pub quals: Vec<Name>,
    pub name: Name,
}

impl VarPath {
    pub fn span(&self) -> Span {
        if let Some(q) = self.quals.first() {
            q.span
        } else {
            self.name.span
        }
    }
}

#[derive(Clone, Debug)]
pub struct EffectRow {
    /// Effect labels: a constructor path with at most one region argument.
    pub labels: Vec<(ConPath, Vec<Name>)>,
    pub tail: Option<Name>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct TypeBinder {
    pub name: Name,
    /// Optional kind or domain annotation, as in `(a :: Type)`.
    pub ann: Option<Type>,
    pub implicit: bool,
}

/// `using (x :: Cls t, ...)`.
#[derive(Clone, Debug)]
pub struct Constraint {
    pub name: Name,
    pub class: ConPath,
    pub args: Vec<Type>,
    pub span: Span,
}

/// One item of a `using (...)` clause (refinement types design,
/// "instance heads at refinements"): a class constraint (`x :: Cls t`),
/// a refinement obligation (a decidable predicate over the carrier
/// values: `Ordered xs`, `length xs > 1`, `Sorted xs`), or the generic
/// transfer's preservation obligation (`Preserves (<>) p`).
#[derive(Clone, Debug)]
pub enum Oblig {
    Class(Constraint),
    Pred(Type),
    Preserves { method: Name, pred: Type, span: Span },
}

impl Oblig {
    pub fn span(&self) -> Span {
        match self {
            Oblig::Class(c) => c.span,
            Oblig::Pred(t) => t.span(),
            Oblig::Preserves { span, .. } => *span,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    /// `Int`, `Bool`, `T`, `M.T`.
    Con(ConPath, Span),
    /// Type variable `a`.
    Var(Name, Span),
    /// Type application `F a`.
    App(Box<Type>, Box<Type>, Span),
    /// `Thunk {e}` / `Lazy {e}`: an effect row used as a type argument.
    AppRow(Box<Type>, EffectRow, Span),
    /// `a -> b` with optional multiplicity and effect row.
    Fun {
        arg: Box<Type>,
        mult: Option<Name>,
        eff: Option<EffectRow>,
        res: Box<Type>,
        span: Span,
    },
    /// `forall a b. t`.
    Forall(Vec<TypeBinder>, Box<Type>, Span),
    /// `using (...) => t`.
    Using(Vec<Oblig>, Box<Type>, Span),
    /// Dependent function `(x :: A) -> B` with optional multiplicity and
    /// effect row.
    DepFun(Name, Box<Type>, Option<Name>, Option<EffectRow>, Box<Type>, Span),
    /// Dependent pair `Sigma (n :: A). B`.
    Sigma(Name, Box<Type>, Box<Type>, Span),
    /// `\x -> t` in an index position: an index-level function value
    /// (`List.map (\x -> f (g x)) xs`), a lambda over index values.
    Lam(Name, Box<Type>, Span),
    /// Unit type `()`.
    Unit(Span),
    /// Tuple type `(T1, ..., Tn)`, n >= 2. This extends the draft EBNF type
    /// grammar, which has tuple expressions but no tuple types (decision D1
    /// in `compiler/CONFORMANCE.md`).
    Tuple(Vec<Type>, Span),
    /// Infix chain awaiting fixity resolution (`n + 1` in an index
    /// position, `length bytes <= limit` in a proposition), resolved by the
    /// fixity pass into nested `App` spines over the operator names.
    Infix(Vec<Type>, Vec<Name>, Span),
    /// A numeral in index position (`Vec 0 a`).
    Num(String, Span),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Con(_, s)
            | Type::Var(_, s)
            | Type::App(_, _, s)
            | Type::AppRow(_, _, s)
            | Type::Fun { span: s, .. }
            | Type::Forall(_, _, s)
            | Type::Using(_, _, s)
            | Type::DepFun(_, _, _, _, _, s)
            | Type::Sigma(_, _, _, s)
            | Type::Lam(_, _, s)
            | Type::Unit(s)
            | Type::Tuple(_, s)
            | Type::Infix(_, _, s)
            | Type::Num(_, s) => *s,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Pat {
    Var(Name, Span),
    Wild(Span),
    Unit(Span),
    /// Integer literal: the decimal digit string, with a leading `-` when
    /// prefix negation folded into it.
    Int(String, Span),
    /// `'name` in pattern position: matches the interned `Sym` constant
    /// of the spelling by identity (primitive types specification
    /// section 9).
    Sym(String, Span),
    /// `C p1 ... pn`, with `args` empty for a bare constructor.
    Con(ConPath, Vec<Pat>, Span),
    /// `x@p`.
    As(Name, Box<Pat>, Span),
    /// `(p1, ..., pn)`, n >= 2.
    Tuple(Vec<Pat>, Span),
    /// `[p1, ..., pn]`; empty is the `Nil` pattern.
    List(Vec<Pat>, Span),
    /// `R { x = p, ... }`, a constructor pattern with named fields.
    Record(ConPath, Vec<(Name, Pat)>, Span),
}

impl Pat {
    pub fn span(&self) -> Span {
        match self {
            Pat::Var(_, s)
            | Pat::Wild(s)
            | Pat::Unit(s)
            | Pat::Int(_, s) | Pat::Sym(_, s)
            | Pat::Con(_, _, s)
            | Pat::As(_, _, s)
            | Pat::Tuple(_, s)
            | Pat::List(_, s)
            | Pat::Record(_, _, s) => *s,
        }
    }
}

/// Toggle the sign of an integer literal's digit string: prefix `-`
/// folding negates the literal it wraps (`-3` is `"-3"`, `- -3` is `"3"`,
/// `-0` is `"0"`).
pub fn negate_literal(digits: &str) -> String {
    if let Some(rest) = digits.strip_prefix('-') {
        rest.to_string()
    } else if digits.bytes().all(|b| b == b'0') {
        "0".to_string()
    } else {
        format!("-{digits}")
    }
}

#[derive(Clone, Debug)]
pub struct Alt {
    /// `#[...]` attributes before the pattern (attribute annotations
    /// specification section 1.1: `case` alternatives).
    pub attrs: Vec<Attribute>,
    pub pat: Pat,
    pub guard: Option<crate::ast::Term>,
    /// `#[...]` attributes after `|`, before the guard expression
    /// (attribute annotations specification section 1.1: guards).
    pub guard_attrs: Vec<Attribute>,
    pub body: crate::ast::Term,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum LocalDecl {
    /// `name :: ty`.
    Sig { name: Name, ty: Type, span: Span },
    /// `name pats = body`.
    Eq { name: Name, pats: Vec<Pat>, body: Term, span: Span },
    /// `pat = body`.
    Bind { pat: Pat, body: Term, span: Span },
    /// `~ pat = body`, a memo binding (laziness and reclamation specification).
    Lazy { pat: Pat, body: Term, span: Span },
}

/// A balanced token tree: the argument form of macro invocations
/// (`macro_call = var_path, "!", token_tree` in the grammar). The tokens
/// run from the opening delimiter through its matching close; the tree is
/// macro input and only becomes terms or declarations at expansion.
#[derive(Clone, Debug)]
pub struct TokenTree {
    pub tokens: Vec<crate::lexer::Token>,
    pub span: Span,
}

impl TokenTree {
    /// The tree's top-level groups, split on commas outside nested
    /// delimiters (the macro parameter binding form: `m!(a, b)` binds two
    /// groups). The delimiters of the outermost tree are not part of the
    /// content.
    pub fn arg_groups(&self) -> Vec<TokenTree> {
        let inner = match self.tokens.split_first() {
            Some((first, rest))
                if matches!(
                    first.tok,
                    crate::lexer::Tok::LParen
                        | crate::lexer::Tok::LBracket
                        | crate::lexer::Tok::LBrace
                ) =>
            {
                &rest[..rest.len().saturating_sub(1)]
            }
            _ => &self.tokens[..],
        };
        let mut groups = Vec::new();
        let mut cur: Vec<crate::lexer::Token> = Vec::new();
        let mut depth = 0i32;
        for t in inner {
            match t.tok {
                crate::lexer::Tok::LParen
                | crate::lexer::Tok::LBracket
                | crate::lexer::Tok::LBrace
                | crate::lexer::Tok::LQuote
                | crate::lexer::Tok::DollarParen => depth += 1,
                crate::lexer::Tok::RParen
                | crate::lexer::Tok::RBracket
                | crate::lexer::Tok::RBrace
                | crate::lexer::Tok::RQuote => depth -= 1,
                crate::lexer::Tok::Comma if depth == 0 => {
                    groups.push(TokenTree { tokens: std::mem::take(&mut cur), span: self.span });
                    continue;
                }
                _ => {}
            }
            cur.push(t.clone());
        }
        if !cur.is_empty() || groups.is_empty() {
            groups.push(TokenTree { tokens: cur, span: self.span });
        }
        groups
    }
}

/// One `#[name(args)]` attribute (attribute annotations specification,
/// section 1). `args` holds the raw argument text: the contents of the
/// parentheses, or the `= literal` tail of the named form.
#[derive(Clone, Debug)]
pub struct Attribute {
    /// The dotted attribute name (`repr`, `meta`, ...).
    pub path: Vec<Name>,
    pub args: Option<String>,
    pub span: Span,
}

impl Attribute {
    /// The dotted name as written.
    pub fn name(&self) -> String {
        self.path
            .iter()
            .map(|n| n.text.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
}

/// One statement of a `do` block: `pat <- term`, `let` bindings, or a
/// bare `term`.
#[derive(Clone, Debug)]
pub enum DoStmt {
    Bind { pat: Pat, term: Term, span: Span },
    /// `let` with a whole local block: one pure binding group ("`let`
    /// remains pure and strict in either interpretation"; the surface
    /// EBNF `do_statement = "let", local_block`).
    Let { decls: Vec<LocalDecl>, span: Span },
    Expr { term: Term, span: Span },
}

/// The sequencing-provider method a `do` block desugars through
/// (`design/effects-capabilities-sequencing.md` section 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeqKind {
    Pure,
    Bind,
}

/// One `op pats ... [resume k] -> body` clause of a handler expression.
#[derive(Clone, Debug)]
pub struct OpClause {
    pub op: Name,
    pub pats: Vec<Pat>,
    /// The `resume k` binder, when the clause may continue the handled
    /// computation (one-shot).
    pub resume: Option<Name>,
    pub body: Term,
    pub span: Span,
}

/// `handler Effect [r] where { return p -> e; op ... -> e }`, a handler
/// value: a return clause plus one clause per operation of the effect
/// (design section 3).
#[derive(Clone, Debug)]
pub struct HandlerExpr {
    pub effect: ConPath,
    /// The region argument of a label like `Async r`.
    pub region: Option<Name>,
    pub ret_pat: Pat,
    pub ret_body: Term,
    pub clauses: Vec<OpClause>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Term {
    Var(VarPath),
    Con(ConPath),
    /// Integer literal: the decimal digit string, with a leading `-` for
    /// a signed literal (`-42`) or a folded pattern negation (same form
    /// as [`Pat::Int`]).
    Int(String, Span),
    Str(String, Span),
    Char(String, Span),
    /// `'name` in expression position: the interned `Sym` constant of
    /// the spelling (primitive types specification section 9).
    Sym(String, Span),
    Unit(Span),
    Hole(Span),
    /// Left-associated curried application of a nonempty argument spine.
    App(Box<Term>, Vec<Term>, Span),
    /// `\p1 .. pn -> body`.
    Lam(Vec<Pat>, Box<Term>, Span),
    /// `let block in body`: a sequential binding group (later bindings
    /// see earlier ones).
    Let(Vec<LocalDecl>, Box<Term>, Span),
    /// `letrec block in body`: a recursive binding group; recursive
    /// occurrences need a guard (surface grammar local amendments
    /// section 9).
    Letrec(Vec<LocalDecl>, Box<Term>, Span),
    /// `if [cond attrs] c then [then attrs] t else [else attrs] e`. The
    /// condition-position attributes annotate the true branch and
    /// complete the decision point's weight pair for the false branch
    /// (attribute annotations specification sections 1.1 and 2.1); the
    /// branch-position attributes annotate their branch directly.
    If {
        cond: Box<Term>,
        then: Box<Term>,
        els: Box<Term>,
        cond_attrs: Vec<Attribute>,
        then_attrs: Vec<Attribute>,
        else_attrs: Vec<Attribute>,
        span: Span,
    },
    /// `case scrut of { alts }`.
    Case(Box<Term>, Vec<Alt>, Span),
    /// `e :: ty`.
    Ann(Box<Term>, Type, Span),
    /// Infix chain awaiting fixity resolution: `terms` and `ops` interleave,
    /// `terms.len() == ops.len() + 1`.
    Infix(Vec<Term>, Vec<Name>, Span),
    /// Prefix negation.
    Neg(Box<Term>, Span),
    /// `(e1, ..., en)`, n >= 2.
    Tuple(Vec<Term>, Span),
    /// `[e1, ..., en]`; empty is `Nil`.
    List(Vec<Term>, Span),
    /// `{k1 => v1, ...}`, the map literal (surface grammar local
    /// amendments section 10); empty is the empty map.
    Map(Vec<(Term, Term)>, Span),
    /// `[e1..e3]` and `[e1, e2 .. e3]`, the range literal: an
    /// enumeration from `e1` (stepping to `e2`, default step one) to
    /// `e3` inclusive, through the `Enum` class (Data.Enum).
    Range(Box<Term>, Option<Box<Term>>, Box<Term>, Span),
    /// `e.x`, field projection.
    Proj(Box<Term>, Name, Span),
    /// `R { x = e, ... }`, record construction with named fields.
    Record(ConPath, Vec<(Name, Term)>, Span),
    /// `do { stmts }`, sequencing through the selected provider.
    Do(Vec<DoStmt>, Span),
    /// `handle e with h`.
    Handle(Box<Term>, Box<Term>, Span),
    /// `handler L where { ... }`, a handler value.
    Handler(Box<HandlerExpr>),
    /// `with (x = e) in body`, a lexical capability supply.
    With { name: Name, value: Box<Term>, body: Box<Term>, span: Span },
    /// `f @{x = e, ...}`, explicit capability supplies at a call.
    CapApply { head: Box<Term>, supplies: Vec<(Name, Term)>, span: Span },
    /// The selected sequencing provider's `pure`/`bind` method, built by
    /// `do` desugaring (never produced by the parser).
    SeqOp(SeqKind, Span),
    /// Elaborate the body in the enclosing continuation-passing context
    /// (built by handler elaboration; never produced by the parser).
    CpsWrap(Box<Term>, Span),
    /// `[| e |]`: a typed quotation, a `Code t` value built at compile
    /// time from the template `e` (modules specification section 7).
    Quote { body: Box<Term>, span: Span },
    /// `quoteDecls where { decls }`: a quotation of declarations, a
    /// `Declarations` value whose expansion is checked under ordinary
    /// rules (modules specification section 8).
    QuoteDecls { decls: Vec<Decl>, span: Span },
    /// `$( e )`: a typed hole in a quotation, filled at quote construction
    /// with the `Code`/`Syntax` value of `e`.
    Splice { body: Box<Term>, span: Span },
    /// `m!(tokens)`: a macro invocation with a balanced token tree
    /// argument (grammar: `macro_call`). In compile-time code it applies
    /// the macro to `Syntax` values; in runtime code it expands in place.
    MacroCall { path: VarPath, args: TokenTree, span: Span },
    /// A template hole standing for a spliced value (elaboration of
    /// quotations; never produced by the parser).
    SpliceHole { idx: usize, span: Span },
    /// `forget p from e` / `forget [p1, p2] from e` (refinement types
    /// design, "partial weakening"): the value at its refinement's
    /// predicate minus the named top-level conjuncts, at zero runtime
    /// cost (the dropped evidence is simply no longer required).
    Forget {
        /// The named conjuncts being given up (a predicate or a list).
        atoms: Vec<Type>,
        value: Box<Term>,
        span: Span,
    },
    /// `narrow e to T with w` (refinement types design, "narrowings"):
    /// the checked strengthening cast along the refinement lattice,
    /// carrying a `Dec` of the stronger predicate.
    Narrow {
        value: Box<Term>,
        target: Type,
        witness: Box<Term>,
        span: Span,
    },
}

impl Term {
    pub fn span(&self) -> Span {
        match self {
            Term::Var(p) => p.span(),
            Term::Con(p) => p.span(),
            Term::Int(_, s)
            | Term::Str(_, s)
            | Term::Char(_, s) | Term::Sym(_, s)
            | Term::Unit(s)
            | Term::Hole(s)
            | Term::App(_, _, s)
            | Term::Lam(_, _, s)
            | Term::Let(_, _, s)
            | Term::Letrec(_, _, s)
            | Term::If { span: s, .. }
            | Term::Case(_, _, s)
            | Term::Ann(_, _, s)
            | Term::Infix(_, _, s)
            | Term::Neg(_, s)
            | Term::Forget { span: s, .. }
            | Term::Narrow { span: s, .. }
            | Term::Tuple(_, s)
            | Term::List(_, s)
            | Term::Map(_, s)
            | Term::Range(_, _, _, s)
            | Term::Proj(_, _, s)
            | Term::Record(_, _, s)
            | Term::Do(_, s)
            | Term::Handle(_, _, s)
            | Term::With { span: s, .. }
            | Term::CapApply { span: s, .. }
            | Term::SeqOp(_, s)
            | Term::CpsWrap(_, s)
            | Term::Quote { span: s, .. }
            | Term::QuoteDecls { span: s, .. }
            | Term::Splice { span: s, .. }
            | Term::MacroCall { span: s, .. }
            | Term::SpliceHole { span: s, .. } => *s,
            Term::Handler(h) => h.span,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
    None,
}

#[derive(Clone, Debug)]
pub struct FixityDecl {
    pub assoc: Assoc,
    pub prec: u8,
    pub ops: Vec<Name>,
    pub span: Span,
}

/// One static module argument in `import M (args) as N` (module
/// parameters as module arguments, A5): a named dictionary or top-level
/// value, or a literal. Arguments are static, so an instantiation is a
/// distinct compilation unit with a computed identity rather than a
/// runtime functor application.
#[derive(Clone, Debug)]
pub enum ModuleArg {
    /// A bare or qualified name: an `instance` (a signature dictionary) or
    /// a top-level value. `quals` is empty for a bare name.
    Name { quals: Vec<Name>, name: Name },
    Int(String, Span),
    Char(String, Span),
    Unit(Span),
}

impl ModuleArg {
    pub fn span(&self) -> Span {
        match self {
            ModuleArg::Name { name, .. } => name.span,
            ModuleArg::Int(_, s) | ModuleArg::Char(_, s) | ModuleArg::Unit(s) => *s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Import {
    pub path: ConPath,
    pub alias: Option<Name>,
    /// `import stage M`: the module's executable generators (`stage`
    /// definitions and macros) become visible to this module's compile-time
    /// code; ordinary names stay invisible (modules specification section 9).
    pub stage: bool,
    /// Instantiation arguments (`import M (args) as N`, module parameters
    /// as module arguments): non-empty means this import instantiates the
    /// parameterized module `path` at these static arguments, producing the
    /// unit named `alias`. Without arguments an import is the ordinary
    /// visibility form and `alias` is the module qualifier it is seen
    /// under (`import X as Y`, surface grammar local amendments section 11).
    pub args: Vec<ModuleArg>,
    /// `import X (a as b, c)`: the selected members under their imported
    /// names; empty selects the whole surface under its own names
    /// (surface grammar local amendments section 11).
    pub members: Vec<ImportMember>,
    pub span: Span,
}

/// One selected member of `import X (a as b, c)` (surface grammar local
/// amendments section 11): the member's name in `X`, and the name it is
/// imported under (`None`: its own name).
#[derive(Clone, Debug)]
pub struct ImportMember {
    pub name: Name,
    pub as_name: Option<Name>,
}

/// `import data "config.json" as config` (surface grammar local
/// amendments section 11): the compile-time converted value of a
/// non-source file, bound under `name`; the extension selects the
/// converter.
#[derive(Clone, Debug)]
pub struct ImportData {
    pub file: String,
    pub name: Name,
    pub span: Span,
}

/// One `export` item (surface grammar local amendments section 11):
/// `name` (a name in this module's scope), `A.name` (the member `name`
/// imported from `A`), or `module A` (everything imported from `A`).
#[derive(Clone, Debug)]
pub enum ExportItem {
    Name(Name),
    /// `A.name`: exactly two segments.
    Member(ConPath),
    Module(Name),
}

/// An `export name, ...` declaration: when present, the module's export
/// surface is exactly the names its items resolve to; without one a
/// module exports its declared names as before and imports stay private
/// (surface grammar local amendments section 11).
#[derive(Clone, Debug)]
pub struct ExportDecl {
    pub items: Vec<ExportItem>,
    pub span: Span,
}

/// One typed module parameter (`module M (x :: T, ...) where`, modules
/// specification section 1): an ordinary binding in the module's scope,
/// provided at instantiation. A parameter whose type is a `signature`
/// application is a dictionary (the instantiator's explicit dictionary
/// selection); any other type is an ordinary value parameter.
#[derive(Clone, Debug)]
pub struct ParamDecl {
    pub name: Name,
    pub ty: Type,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct SigDecl {
    pub name: Name,
    /// Class parameters with optional kind binders, as in `signature
    /// Functor (f :: Type -> Type)` or `signature Functor F` (an
    /// un-annotated parameter's kind is inferred from the method types).
    pub params: Vec<TypeBinder>,
    /// Associated type members written inside the signature block:
    /// `opaque type T [params] :: Type` and `record`/`data` members
    /// (modules and compilation specification section 1; the foreign
    /// function interface specification's `opaque` resources). The
    /// declarations themselves are spliced ahead of the signature so the
    /// method types resolve them (decision D65).
    pub type_members: Vec<Name>,
    /// The member declarations to splice (emptied by the splice).
    pub member_decls: Vec<Decl>,
    pub methods: Vec<(Name, Type)>,
    /// Default equations for declared methods, in source order: an
    /// instance that does not define a method takes its default, checked
    /// at the instance's types as if the instance had written it.
    pub defaults: Vec<Equation>,
    /// `law name :: prop` members (primitive operations and inline
    /// target IR specification section 5): class laws stating the
    /// equations instances are checked against, citable as `justify`
    /// evidence by identity.
    pub laws: Vec<(Name, Type)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Equation {
    /// `#[...]` attributes on the function clause: between the function
    /// name and its first pattern (attribute annotations specification
    /// section 1.1).
    pub attrs: Vec<Attribute>,
    pub name: Name,
    pub pats: Vec<Pat>,
    pub body: Term,
    pub locals: Vec<LocalDecl>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct InstDecl {
    pub implicit: bool,
    pub name: Name,
    /// Type parameters, as in `instance Add_T a :: ...`.
    pub params: Vec<Name>,
    pub ctx: Vec<Oblig>,
    pub target: Type,
    pub methods: Vec<Equation>,
    /// Method signatures restating a class method at this instance with
    /// refinement obligations over the method's own arguments
    /// (`(-) :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat`):
    /// a precondition only this instance has, discharged at every
    /// direct application of the method.
    pub method_sigs: Vec<(Name, Type)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct CtorDecl {
    /// `#[...]` attributes on the constructor declaration (attribute
    /// annotations specification section 1.1).
    pub attrs: Vec<Attribute>,
    pub name: Name,
    pub ty: Type,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct DataDecl {
    pub name: Name,
    /// Parameters with optional kind annotations, as in
    /// `data Vec (n :: Nat) (a :: Type) where`; an annotated parameter's
    /// kind decides whether it is a type parameter or an index value.
    pub params: Vec<TypeBinder>,
    pub ctors: Vec<CtorDecl>,
    pub span: Span,
}

/// One `field :: T` entry of a `record` declaration.
#[derive(Clone, Debug)]
pub struct FieldDecl {
    /// `#[...]` attributes on the field: both the declaration position
    /// (before `field`) and the type position (before `T`, where a
    /// representation attribute applies; attribute annotations
    /// specification sections 1.1 and 4).
    pub attrs: Vec<Attribute>,
    pub name: Name,
    pub ty: Type,
    /// `erased` fields are evidence: present in the type, absent from the
    /// runtime representation (core specification section 3).
    pub erased: bool,
    pub span: Span,
}

/// `record R params where { field :: T, ... }`, a single-constructor product.
#[derive(Clone, Debug)]
pub struct RecordDecl {
    pub name: Name,
    pub params: Vec<TypeBinder>,
    pub fields: Vec<FieldDecl>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct ForeignDecl {
    pub is_import: bool,
    /// `"ccall"` or `"component"`.
    pub abi: String,
    pub symbol: String,
    pub name: Name,
    pub ty: Type,
    pub span: Span,
}

/// One `uses` entry of a `prim` body (primitive operations and inline
/// target IR specification section 4): a full foreign binding contract
/// for a symbol the inline fragment may reference, recorded like a
/// foreign import.
#[derive(Clone, Debug)]
pub struct PrimUse {
    pub name: Name,
    pub ty: Type,
    pub span: Span,
}

/// One target-IR body of a `prim` declaration (primitive operations
/// and inline target IR specification section 2): `family { fragment }
/// [when cond] [justify e] [uses ...] [immarg ...]`. The fragment is
/// the family's raw text payload, kept verbatim for round-tripping and
/// provenance; `immarg` lists the parameter indices (from 0) the body
/// requires as compile-time immediates (`llvm-contract-attributes.md`:
/// shift amounts, SIMD lane indices).
#[derive(Clone, Debug)]
pub struct PrimBody {
    /// The target-IR family: `llvm`, `wasm`, `asm`, `clr`, `jvm`, or a
    /// user family name (rejected when unregistered).
    pub family: Name,
    /// The family's inline fragment, verbatim.
    pub fragment: String,
    /// The fragment's own span (the open brace's).
    pub fragment_span: Span,
    /// `when cond`: the capability condition scoping the body. `None`
    /// is the baseline body.
    pub when: Option<Term>,
    /// `justify e`: evidence that the body implements the declared
    /// type's mathematical meaning (specification section 5).
    pub justify: Option<Term>,
    /// `uses ...`: the external symbols the fragment may reference.
    pub uses: Vec<PrimUse>,
    /// `immarg ...`: immediate parameter indices with their spans.
    pub immarg: Vec<(u64, Span)>,
    /// `unsafe`: containment cannot verify this body's effects (an asm
    /// template, a body calling through `uses` into opaque code), so its
    /// declared effect row is trusted, not verified, and the trust is
    /// recorded as an assumption (primitive operations and inline
    /// target IR specification section 4).
    pub unverified: bool,
    pub span: Span,
}

/// `prim name :: type where { target_body ... }` (primitive operations
/// and inline target IR specification section 2): a primitive
/// operation with inline target-IR bodies. A `prim` name is a
/// module-level value like any other.
#[derive(Clone, Debug)]
pub struct PrimDecl {
    pub name: Name,
    pub ty: Type,
    pub bodies: Vec<PrimBody>,
    pub span: Span,
}

/// `opaque type T [params] :: Type`: an abstract foreign resource
/// (foreign function interface specification section 2). Values of the
/// type arise only from foreign calls; there are no constructors and no
/// layout projection.
#[derive(Clone, Debug)]
pub struct OpaqueTypeDecl {
    pub name: Name,
    pub params: Vec<TypeBinder>,
    pub kind: Type,
    pub span: Span,
}

/// `type R params = refine (x :: T) with p` (refinement types design):
/// the refinement `R` of the container `T` by the decidable predicate
/// `p` over the refined binder `x`. The runtime representation IS the
/// container: the witness is `Prop`-erased (D31), construction
/// discharges `p` explicitly in the `Dec`/`decide` world, and the
/// container's instances transfer exactly when their methods preserve
/// `p` (checked by the P18 law layer).
#[derive(Clone, Debug)]
pub struct RefineDecl {
    pub name: Name,
    /// The refinement's type parameters (`type NonEmpty a = ...`'s `a`).
    pub params: Vec<TypeBinder>,
    /// The refined binder (`refine (xs :: List a) with NotNull xs`'s
    /// `xs`): the value the predicate speaks about.
    pub binder: Name,
    /// The container type.
    pub base: Type,
    /// The decidable predicate over the binder.
    pub pred: Type,
    pub span: Span,
}

/// `effect L params where { op :: ty, ... }`: a user-declared effect whose
/// operations are ordinary functions performing the label (design section
/// 2). `params` holds the effect's region parameter, if any.
#[derive(Clone, Debug)]
pub struct EffectDecl {
    pub name: Name,
    pub params: Vec<Name>,
    pub ops: Vec<(Name, Type)>,
    pub span: Span,
}

/// `rewrite name forall binders. [using (ctx)] where { match e; replace e;
/// [require e]; justify e }`, and its `target rewrite` form (modules and
/// compilation specification section 5; platform proofs and SIMD
/// specification section 3). `match`/`replace` are the rule's pattern and
/// template terms, `require` its applicability condition (the target
/// rule's operational evidence), and `justify` its checked evidence under
/// the rule's semantic relation.
#[derive(Clone, Debug)]
pub struct RewriteDecl {
    /// `target rewrite`: parameterized by a target capability (modules
    /// specification section 13).
    pub target: bool,
    pub name: Name,
    pub binders: Vec<TypeBinder>,
    /// `using (...)` context: the evidence the rule's template may use.
    pub ctx: Vec<Oblig>,
    pub match_tm: Term,
    pub replace_tm: Term,
    pub require: Option<Term>,
    pub justify: Term,
    pub span: Span,
}

/// `upgrade Old to New where { migrate = e; [related = e] }`: the state
/// migration evidence for evolving record `Old` into record `New`
/// (runtime evolution specification section 3). `migrate` is the checked
/// migration function `Old %1 -> New` (the field mapping it induces is
/// verified against the two records' field tables). `related` optionally
/// records the state relation. A block without `migrate` declares a
/// record view instead: a compatible (additive) evolution admitted
/// without a migration, where added fields stay distinguishable from
/// missing fields under the view (runtime evolution section 2).
#[derive(Clone, Debug)]
pub struct UpgradeDecl {
    pub old: ConPath,
    pub new: ConPath,
    /// The migration function `Old %1 -> New`, when this is a migration.
    pub migrate: Option<Term>,
    /// The state relation evidence, when supplied.
    pub related: Option<Term>,
    pub span: Span,
}

/// The temporal kind a contract claims (core specification section 10:
/// the language distinguishes safety, liveness, recurrence and
/// stability).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractKind {
    /// `safety`: something bad never happens. `always (P)`: the
    /// invariant P holds at every step of the definition's transitions.
    Safety,
    /// `liveness`: something good eventually happens. `eventually (Q)`.
    Liveness,
    /// `recurrence`: a state recurs. `recur (P)`: P is re-established at
    /// every cycle of the definition's recursion.
    Recurrence,
    /// `stability`: a fact once true stays true. `stable (P)`: P
    /// survives the definition's transitions.
    Stability,
}

impl ContractKind {
    /// The kind's keyword, as written.
    pub fn keyword(self) -> &'static str {
        match self {
            ContractKind::Safety => "safety",
            ContractKind::Liveness => "liveness",
            ContractKind::Recurrence => "recurrence",
            ContractKind::Stability => "stability",
        }
    }

    /// The formula operator the kind states (the notation of core
    /// specification section 10: `always (P)`, `eventually (Q)`, and
    /// the recurrence and stability spellings).
    pub fn operator(self) -> &'static str {
        match self {
            ContractKind::Safety => "always",
            ContractKind::Liveness => "eventually",
            ContractKind::Recurrence => "recur",
            ContractKind::Stability => "stable",
        }
    }
}

/// One clause of a `contract` declaration's block (core specification
/// section 10): the temporal formula (its operator matching the
/// contract's kind), and the stated assumptions temporal contracts must
/// carry when they rely on the environment.
#[derive(Clone, Debug)]
pub enum ContractClause {
    /// The temporal formula: `<operator> = (P)`.
    Formula { op: Name, body: Term },
    /// `assume = (P)`: a stated assumed fact (a safety contract's
    /// establishment is one).
    AssumeProp { body: Term },
    /// `assume = "text"`: a stated environmental assumption (scheduler,
    /// dependency, crash, network, fairness).
    AssumeNamed { text: String, span: Span },
}

/// `contract <kind> <name> for <def> where { ... }`: a temporal contract
/// over a definition (core specification section 10). The contract
/// claims one temporal formula about the named definition's state
/// transitions and states the assumptions it relies upon; it is
/// verified with the definition's elaboration where decidable and
/// refused by name where not.
#[derive(Clone, Debug)]
pub struct ContractDecl {
    pub kind: ContractKind,
    /// The contract's own name (the report and manifest name it).
    pub name: Name,
    /// The definition the contract names, in the contract's module.
    pub target: Name,
    pub clauses: Vec<ContractClause>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Import(Import),
    /// `import data "f.ext" as x` (surface grammar local amendments
    /// section 11): a non-source file's converted value.
    ImportData(ImportData),
    /// `export name, ...` (surface grammar local amendments section 11).
    Export(ExportDecl),
    Signature(SigDecl),
    Instance(InstDecl),
    Data(DataDecl),
    Record(RecordDecl),
    /// `codata C params where { obs :: T, ... }`: a coinductive type of
    /// observations (codata and guardedness design, section 2). The shape
    /// is a record declaration, but construction is the observed record
    /// form and elimination is observation only.
    Codata(RecordDecl),
    /// `name :: ty` (with optional unsupported modifier in `modifier`).
    ValueSig {
        name: Name,
        modifier: Option<Name>,
        /// Attribute names a `macro` signature declares as its own
        /// (`macro name attributes (codec, ...) :: ty`; attribute
        /// annotations specification section 6).
        owned_attrs: Vec<Name>,
        ty: Type,
        span: Span,
    },
    Equation { modifier: Option<Name>, eq: Equation },
    /// A declaration whose name is computed: `$( e ) :: ty` and
    /// `$( e ) pats = body` inside a `quoteDecls` template (D46 name
    /// splices). The name-producing term is a template hole; at splice
    /// time its `Name`/`Sym`/name-quote value names `inner`, whose own
    /// name slot is a placeholder until then. Outside a template a name
    /// splice is refused.
    NameSpliced { name_tm: Box<Term>, inner: Box<Decl> },
    Foreign(ForeignDecl),
    /// `prim name :: type where { target_body ... }` (primitive
    /// operations and inline target IR specification section 2): a
    /// primitive operation with inline target-IR bodies.
    Prim(PrimDecl),
    /// `type R params = refine (x :: T) with p` (refinement types
    /// design): a refinement of a container by a decidable predicate.
    Refine(RefineDecl),
    /// `opaque type T [params] :: Type` (foreign function interface
    /// specification section 2).
    OpaqueType(OpaqueTypeDecl),
    Fixity(FixityDecl),
    Effect(EffectDecl),
    /// `#[...]`-prefixed declaration: the attributes annotate `inner`
    /// (attribute annotations specification section 1.1).
    Attributed { attrs: Vec<Attribute>, inner: Box<Decl> },
    /// `generate do { stmts }`: a compile-time generator (modules
    /// specification section 10).
    Generate { stmts: Vec<DoStmt>, span: Span },
    /// A verified rewrite rule (modules and compilation specification
    /// section 5), or its `target rewrite` form (platform proofs and SIMD
    /// specification section 3).
    Rewrite(RewriteDecl),
    /// `upgrade Old to New where { migrate = e; [related = e] }`: the
    /// state migration for evolving record `Old` into record `New`
    /// (runtime evolution specification section 3). `migrate` is the
    /// migration function `Old %1 -> New`. A block that omits `migrate`
    /// is a record view: a compatible (additive) evolution admitted
    /// without a migration, whose added fields stay distinguishable from
    /// missing under the view (runtime evolution section 2; core rule).
    Upgrade(UpgradeDecl),
    /// `contract <kind> <name> for <def> where { ... }`: a temporal
    /// contract over a definition (core specification section 10), one
    /// of safety, liveness, recurrence or stability.
    Contract(ContractDecl),
}

#[derive(Clone, Debug)]
pub struct Module {
    /// `#[...]` attributes before the `module` keyword (attribute
    /// annotations specification section 10: `#[profile "jpl"]` and
    /// `#[deny-warnings]` on a module header).
    pub attrs: Vec<Attribute>,
    pub name: Name,
    /// The parameter block of `module M (x :: T, ...) where` (module
    /// parameters as module arguments): ordinary bindings in the module
    /// scope, provided at instantiation. Empty for a plain module.
    pub params: Vec<ParamDecl>,
    pub imports: Vec<Import>,
    pub decls: Vec<Decl>,
    pub span: Span,
}

// ------------------------------------------------- staging: template hygiene

/// The namespace a top-level name resolves in: values (including
/// constructors and operators), or types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefKind {
    Value,
    Type,
}

/// A top-level name reference found by [`collect_term_refs`] or
/// [`collect_decl_refs`]: `quals` is the written module qualification (or
/// empty for a bare name).
#[derive(Clone, Debug)]
pub struct RefSite {
    pub kind: RefKind,
    pub quals: Vec<String>,
    pub name: String,
    pub span: Span,
}

/// Definition-site resolution of a bare name to its owning `(module,
/// name)`; `None` when the name does not resolve there.
pub type NameResolver<'a> = &'a dyn Fn(RefKind, &str) -> Option<(String, String)>;

fn ref_of_path(kind: RefKind, segments: &[Name], span: Span) -> RefSite {
    RefSite {
        kind,
        quals: segments[..segments.len().saturating_sub(1)]
            .iter()
            .map(|n| n.text.clone())
            .collect(),
        name: segments.last().map(|n| n.text.clone()).unwrap_or_default(),
        span,
    }
}

/// Every top-level name `t` references (module call graph and staging
/// dependency graph). Local binder occurrences are not references.
pub fn collect_term_refs(t: &Term, out: &mut Vec<RefSite>) {
    match t {
        Term::Var(p) => out.push(RefSite {
            kind: RefKind::Value,
            quals: p.quals.iter().map(|n| n.text.clone()).collect(),
            name: p.name.text.clone(),
            span: p.span(),
        }),
        Term::Con(p) => out.push(ref_of_path(RefKind::Value, &p.segments, p.span())),
        Term::Int(_, _)
        | Term::Str(_, _)
        | Term::Char(_, _) | Term::Sym(_, _)
        | Term::Unit(_)
        | Term::Hole(_)
        | Term::SeqOp(_, _)
        | Term::SpliceHole { .. } => {}
        Term::App(f, args, _) => {
            collect_term_refs(f, out);
            for a in args {
                collect_term_refs(a, out);
            }
        }
        Term::Lam(_, body, _) => collect_term_refs(body, out),
        Term::Let(_, body, _) | Term::Letrec(_, body, _) => collect_term_refs(body, out),
        Term::If { cond, then, els, .. } => {
            collect_term_refs(cond, out);
            collect_term_refs(then, out);
            collect_term_refs(els, out);
        }
        Term::Case(s, alts, _) => {
            collect_term_refs(s, out);
            for alt in alts {
                if let Some(g) = &alt.guard {
                    collect_term_refs(g, out);
                }
                collect_term_refs(&alt.body, out);
            }
        }
        Term::Ann(e, _, _) => collect_term_refs(e, out),
        Term::Forget { value, .. } => collect_term_refs(value, out),
        Term::Narrow { value, witness, .. } => {
            collect_term_refs(value, out);
            collect_term_refs(witness, out);
        }
        Term::Infix(ts, _, _) => {
            for x in ts {
                collect_term_refs(x, out);
            }
        }
        Term::Neg(e, _) => collect_term_refs(e, out),
        Term::Tuple(ts, _) | Term::List(ts, _) => {
            for x in ts {
                collect_term_refs(x, out);
            }
        }
        Term::Map(kvs, _) => {
            for (k, v) in kvs {
                collect_term_refs(k, out);
                collect_term_refs(v, out);
            }
        }
        Term::Range(start, step, end, _) => {
            collect_term_refs(start, out);
            if let Some(s) = step {
                collect_term_refs(s, out);
            }
            collect_term_refs(end, out);
        }
        Term::Proj(e, _, _) => collect_term_refs(e, out),
        Term::Record(p, fields, _) => {
            out.push(ref_of_path(RefKind::Value, &p.segments, p.span()));
            for (_, e) in fields {
                collect_term_refs(e, out);
            }
        }
        Term::Do(stmts, _) => {
            for s in stmts {
                match s {
                    DoStmt::Bind { term, .. } | DoStmt::Expr { term, .. } => {
                        collect_term_refs(term, out)
                    }
                    DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            collect_local_refs(decl, out)
                        }
                    }
                }
            }
        }
        Term::Handle(e, h, _) => {
            collect_term_refs(e, out);
            collect_term_refs(h, out);
        }
        Term::Handler(h) => {
            collect_term_refs(&h.ret_body, out);
            for c in &h.clauses {
                collect_term_refs(&c.body, out);
            }
        }
        Term::With { value, body, .. } => {
            collect_term_refs(value, out);
            collect_term_refs(body, out);
        }
        Term::CapApply { head, supplies, .. } => {
            collect_term_refs(head, out);
            for (_, e) in supplies {
                collect_term_refs(e, out);
            }
        }
        Term::CpsWrap(e, _) => collect_term_refs(e, out),
        Term::Quote { body, .. } => collect_term_refs(body, out),
        Term::QuoteDecls { decls, .. } => {
            for d in decls {
                collect_decl_refs(d, out);
            }
        }
        Term::Splice { body, .. } => collect_term_refs(body, out),
        Term::MacroCall { path, .. } => out.push(RefSite {
            kind: RefKind::Value,
            quals: path.quals.iter().map(|n| n.text.clone()).collect(),
            name: path.name.text.clone(),
            span: path.span(),
        }),
    }
}

fn collect_local_refs(d: &LocalDecl, out: &mut Vec<RefSite>) {
    match d {
        LocalDecl::Sig { ty, .. } => collect_type_refs(ty, out),
        LocalDecl::Eq { body, .. } | LocalDecl::Bind { body, .. } | LocalDecl::Lazy { body, .. } => {
            collect_term_refs(body, out)
        }
    }
}

/// Splice signature type members ahead of their signatures so the
/// method types resolve them (decision D65: associated type members are
/// declared in the enclosing scope; per-instance refinement belongs to
/// the module-packaging follow-up).
pub fn splice_sig_members(decls: Vec<Decl>) -> Vec<Decl> {
    let mut out = Vec::new();
    for d in decls {
        match d {
            Decl::Signature(mut s) => {
                let mut members = std::mem::take(&mut s.member_decls);
                out.append(&mut members);
                out.push(Decl::Signature(s));
            }
            Decl::Attributed { attrs, inner } => {
                let mut inner = *inner;
                if let Decl::Signature(s) = &mut inner {
                    let mut members = std::mem::take(&mut s.member_decls);
                    out.append(&mut members);
                }
                out.push(Decl::Attributed { attrs, inner: Box::new(inner) });
            }
            other => out.push(other),
        }
    }
    out
}

/// Every top-level name a declaration references. A temporal contract's
/// formula is specification notation over the named definition's
/// parameters (core specification section 10): erased, so it adds no
/// call-graph reference.
pub fn collect_decl_refs(d: &Decl, out: &mut Vec<RefSite>) {
    match d {
        Decl::Import(_)
        | Decl::ImportData(_)
        | Decl::Export(_)
        | Decl::Fixity(_)
        | Decl::OpaqueType(_)
        | Decl::Foreign(_)
        | Decl::Contract(_) => {}
        Decl::Attributed { inner, .. } => collect_decl_refs(inner, out),
        Decl::NameSpliced { name_tm, inner } => {
            collect_term_refs(name_tm, out);
            collect_decl_refs(inner, out);
        }
        Decl::Generate { stmts, .. } => {
            for s in stmts {
                match s {
                    DoStmt::Bind { term, .. } | DoStmt::Expr { term, .. } => {
                        collect_term_refs(term, out)
                    }
                    DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            collect_local_refs(decl, out)
                        }
                    }
                }
            }
        }
        Decl::Rewrite(r) => {
            for b in &r.binders {
                if let Some(ann) = &b.ann {
                    collect_type_refs(ann, out);
                }
            }
            for c in &r.ctx {
                match c {
                    Oblig::Class(c) => {
                        for a in &c.args {
                            collect_type_refs(a, out);
                        }
                    }
                    Oblig::Pred(t) => collect_type_refs(t, out),
                    Oblig::Preserves { pred, .. } => collect_type_refs(pred, out),
                }
            }
            collect_term_refs(&r.match_tm, out);
            collect_term_refs(&r.replace_tm, out);
            if let Some(req) = &r.require {
                collect_term_refs(req, out);
            }
            collect_term_refs(&r.justify, out);
        }
        Decl::Upgrade(u) => {
            out.push(ref_of_path(RefKind::Type, &u.old.segments, u.old.span()));
            out.push(ref_of_path(RefKind::Type, &u.new.segments, u.new.span()));
            if let Some(m) = &u.migrate {
                collect_term_refs(m, out);
            }
            if let Some(r) = &u.related {
                collect_term_refs(r, out);
            }
        }
        Decl::Signature(s) => {
            for (_, ty) in &s.methods {
                collect_type_refs(ty, out);
            }
            for eq in &s.defaults {
                collect_term_refs(&eq.body, out);
                for l in &eq.locals {
                    collect_local_refs(l, out);
                }
            }
            for (_, ty) in &s.laws {
                collect_type_refs(ty, out);
            }
        }
        Decl::Prim(p) => {
            collect_type_refs(&p.ty, out);
            for b in &p.bodies {
                if let Some(w) = &b.when {
                    collect_term_refs(w, out);
                }
                if let Some(j) = &b.justify {
                    collect_term_refs(j, out);
                }
                for u in &b.uses {
                    collect_type_refs(&u.ty, out);
                }
            }
        }
        Decl::Instance(i) => {
            collect_type_refs(&i.target, out);
            for c in &i.ctx {
                match c {
                    Oblig::Class(c) => {
                        for a in &c.args {
                            collect_type_refs(a, out);
                        }
                    }
                    Oblig::Pred(t) => collect_type_refs(t, out),
                    Oblig::Preserves { pred, .. } => collect_type_refs(pred, out),
                }
            }
            for (_, t) in &i.method_sigs {
                collect_type_refs(t, out);
            }
            for m in &i.methods {
                collect_term_refs(&m.body, out);
                for l in &m.locals {
                    collect_local_refs(l, out);
                }
            }
        }
        Decl::Data(data) => {
            for c in &data.ctors {
                collect_type_refs(&c.ty, out);
            }
        }
        Decl::Refine(r) => {
            for b in &r.params {
                if let Some(ann) = &b.ann {
                    collect_type_refs(ann, out);
                }
            }
            collect_type_refs(&r.base, out);
            collect_type_refs(&r.pred, out);
        }
        Decl::Record(r) | Decl::Codata(r) => {
            for f in &r.fields {
                collect_type_refs(&f.ty, out);
            }
        }
        Decl::Effect(e) => {
            for (_, ty) in &e.ops {
                collect_type_refs(ty, out);
            }
        }
        Decl::ValueSig { ty, .. } => collect_type_refs(ty, out),
        Decl::Equation { eq, .. } => {
            collect_term_refs(&eq.body, out);
            for l in &eq.locals {
                collect_local_refs(l, out);
            }
        }
    }
}

/// Every top-level name a type references.
pub fn collect_type_refs(t: &Type, out: &mut Vec<RefSite>) {
    match t {
        Type::Con(p, span) => out.push(ref_of_path(RefKind::Type, &p.segments, *span)),
        Type::Var(_, _) | Type::Unit(_) | Type::Num(_, _) => {}
        Type::App(a, b, _) => {
            collect_type_refs(a, out);
            collect_type_refs(b, out);
        }
        Type::AppRow(a, _, _) => collect_type_refs(a, out),
        Type::Fun { arg, res, .. } => {
            collect_type_refs(arg, out);
            collect_type_refs(res, out);
        }
        Type::Forall(_, b, _) => collect_type_refs(b, out),
        Type::Lam(_, b, _) => collect_type_refs(b, out),
        Type::Using(cs, b, _) => {
            for c in cs {
                match c {
                    Oblig::Class(c) => {
                        for a in &c.args {
                            collect_type_refs(a, out);
                        }
                    }
                    Oblig::Pred(t) => collect_type_refs(t, out),
                    Oblig::Preserves { pred, .. } => collect_type_refs(pred, out),
                }
            }
            collect_type_refs(b, out);
        }
        Type::DepFun(_, a, _, _, b, _) => {
            collect_type_refs(a, out);
            collect_type_refs(b, out);
        }
        Type::Sigma(_, a, b, _) => {
            collect_type_refs(a, out);
            collect_type_refs(b, out);
        }
        Type::Tuple(ts, _) => {
            for x in ts {
                collect_type_refs(x, out);
            }
        }
        Type::Infix(ts, _, _) => {
            for x in ts {
                collect_type_refs(x, out);
            }
        }
    }
}

// ------------------------------------------- staging: capture-avoiding templates

/// Hygiene context for rewriting one quotation or macro template (modules
/// specification section 8): generated binders are fresh by default, and
/// free top-level names resolve where the template was defined.
pub struct HygCx<'a> {
    /// Definition-site resolution of bare top-level names.
    pub resolve: NameResolver<'a>,
    /// Fresh name generation (one counter per expansion).
    pub fresh: &'a mut dyn FnMut(&str) -> String,
    /// Top-level names the template itself declares (`quoteDecls` items):
    /// references to them stay bare and bind the generated declarations.
    pub own_values: Vec<String>,
    pub own_types: Vec<String>,
}

#[derive(Default)]
pub struct HygScope {
    /// `(spelled, fresh)` pairs; the last one wins (shadowing).
    values: Vec<(String, String)>,
    types: Vec<(String, String)>,
}

impl HygScope {
    pub fn new() -> HygScope {
        HygScope::default()
    }

    fn lookup_value(&self, n: &str) -> Option<&str> {
        self.values.iter().rev().find(|(a, _)| a == n).map(|(_, b)| b.as_str())
    }
    fn lookup_type(&self, n: &str) -> Option<&str> {
        self.types.iter().rev().find(|(a, _)| a == n).map(|(_, b)| b.as_str())
    }
}

fn fresh_name(cx: &mut HygCx, base: &str) -> String {
    (cx.fresh)(base)
}

fn qualify_value(cx: &HygCx, path: &VarPath, span: Span) -> Result<VarPath> {
    if !path.quals.is_empty() || cx.own_values.iter().any(|n| *n == path.name.text) {
        return Ok(path.clone());
    }
    match (cx.resolve)(RefKind::Value, &path.name.text) {
        // Builtins are in scope everywhere and cannot be redefined: a
        // bare name keeps them (and stays readable in diagnostics).
        Some((module, name)) if module == "<builtin>" => {
            Ok(VarPath {
                quals: Vec::new(),
                name: Name::new(name, path.name.span),
            })
        }
        Some((module, name)) => Ok(VarPath {
            quals: vec![Name::new(module, span)],
            name: Name::new(name, path.name.span),
        }),
        None => Err(Error::at(
            path.span(),
            format!(
                "`{}` does not resolve where this template was defined; a template may only reference its own binders, its own declarations, and names visible at its definition site",
                path.name.text
            ),
        )),
    }
}

fn qualify_con(cx: &HygCx, path: &ConPath, span: Span) -> Result<ConPath> {
    if path.segments.len() > 1 || cx.own_values.iter().any(|n| *n == path.segments[0].text) {
        return Ok(path.clone());
    }
    match (cx.resolve)(RefKind::Value, &path.segments[0].text) {
        Some((module, name)) if module == "<builtin>" => Ok(ConPath {
            segments: vec![Name::new(name, path.segments[0].span)],
        }),
        Some((module, name)) => Ok(ConPath {
            segments: vec![Name::new(module, span), Name::new(name, path.segments[0].span)],
        }),
        None => Err(Error::at(
            span,
            format!(
                "`{}` does not resolve where this template was defined; a template may only reference its own binders, its own declarations, and names visible at its definition site",
                path.segments[0].text
            ),
        )),
    }
}

fn qualify_type_con(cx: &HygCx, path: &ConPath, span: Span) -> Result<ConPath> {
    if path.segments.len() > 1 || cx.own_types.iter().any(|n| *n == path.segments[0].text) {
        return Ok(path.clone());
    }
    match (cx.resolve)(RefKind::Type, &path.segments[0].text) {
        Some((module, name)) if module == "<builtin>" => Ok(ConPath {
            segments: vec![Name::new(name, path.segments[0].span)],
        }),
        Some((module, name)) => Ok(ConPath {
            segments: vec![Name::new(module, span), Name::new(name, path.segments[0].span)],
        }),
        None => Err(Error::at(
            span,
            format!(
                "type `{}` does not resolve where this template was defined; a template may only reference its own declarations and types visible at its definition site",
                path.segments[0].text
            ),
        )),
    }
}

fn hyg_pat(p: &Pat, cx: &mut HygCx, sc: &mut HygScope) -> Result<Pat> {
    Ok(match p {
        Pat::Var(n, s) => {
            let f = fresh_name(cx, &n.text);
            sc.values.push((n.text.clone(), f.clone()));
            Pat::Var(Name::new(f, n.span), *s)
        }
        Pat::Wild(s) => Pat::Wild(*s),
        Pat::Unit(s) => Pat::Unit(*s),
        Pat::Int(v, s) => Pat::Int(v.clone(), *s),
        Pat::Sym(v, s) => Pat::Sym(v.clone(), *s),
        Pat::Con(path, pats, s) => {
            let path = qualify_con(cx, path, *s)?;
            let mut out = Vec::new();
            for x in pats {
                out.push(hyg_pat(x, cx, sc)?);
            }
            Pat::Con(path, out, *s)
        }
        Pat::As(n, inner, s) => {
            let inner = hyg_pat(inner, cx, sc)?;
            let f = fresh_name(cx, &n.text);
            sc.values.push((n.text.clone(), f.clone()));
            Pat::As(Name::new(f, n.span), Box::new(inner), *s)
        }
        Pat::Tuple(pats, s) => {
            let mut out = Vec::new();
            for x in pats {
                out.push(hyg_pat(x, cx, sc)?);
            }
            Pat::Tuple(out, *s)
        }
        Pat::List(pats, s) => {
            let mut out = Vec::new();
            for x in pats {
                out.push(hyg_pat(x, cx, sc)?);
            }
            Pat::List(out, *s)
        }
        Pat::Record(path, fields, s) => {
            let path = qualify_con(cx, path, *s)?;
            let mut out = Vec::new();
            for (f, x) in fields {
                out.push((f.clone(), hyg_pat(x, cx, sc)?));
            }
            Pat::Record(path, out, *s)
        }
    })
}

fn hyg_type(t: &Type, cx: &mut HygCx, sc: &mut HygScope) -> Result<Type> {
    Ok(match t {
        Type::Con(path, s) => Type::Con(qualify_type_con(cx, path, *s)?, *s),
        Type::Var(n, s) => match sc.lookup_type(&n.text) {
            Some(f) => Type::Var(Name::new(f, n.span), *s),
            None => Type::Var(n.clone(), *s),
        },
        Type::App(a, b, s) => {
            Type::App(Box::new(hyg_type(a, cx, sc)?), Box::new(hyg_type(b, cx, sc)?), *s)
        }
        Type::AppRow(a, r, s) => Type::AppRow(Box::new(hyg_type(a, cx, sc)?), r.clone(), *s),
        Type::Lam(n, b, s) => {
            // An index lambda's binder binds its body's index variables
            // (they spell as `Type::Var`), so it renames like a template
            // binder.
            let f = fresh_name(cx, &n.text);
            sc.types.push((n.text.clone(), f.clone()));
            let b = hyg_type(b, cx, sc)?;
            sc.types.pop();
            Type::Lam(Name::new(f, n.span), Box::new(b), *s)
        }
        Type::Fun { arg, mult, eff, res, span } => Type::Fun {
            arg: Box::new(hyg_type(arg, cx, sc)?),
            mult: mult.clone(),
            eff: eff.clone(),
            res: Box::new(hyg_type(res, cx, sc)?),
            span: *span,
        },
        Type::Forall(binders, body, s) => {
            let mut binders = binders.clone();
            for b in &mut binders {
                if let Some(ann) = &b.ann {
                    b.ann = Some(hyg_type(ann, cx, sc)?);
                }
                let f = fresh_name(cx, &b.name.text);
                sc.types.push((b.name.text.clone(), f.clone()));
                b.name = Name::new(f, b.name.span);
            }
            let body = hyg_type(body, cx, sc)?;
            for _ in &binders {
                sc.types.pop();
            }
            Type::Forall(binders, Box::new(body), *s)
        }
        Type::Using(cs, body, s) => {
            let mut out = Vec::new();
            for c in cs {
                out.push(match c.clone() {
                    Oblig::Class(mut c) => {
                        for a in &mut c.args {
                            *a = hyg_type(a, cx, sc)?;
                        }
                        Oblig::Class(c)
                    }
                    Oblig::Pred(t) => Oblig::Pred(hyg_type(&t, cx, sc)?),
                    Oblig::Preserves { method, pred, span } => Oblig::Preserves {
                        method,
                        pred: hyg_type(&pred, cx, sc)?,
                        span,
                    },
                });
            }
            Type::Using(out, Box::new(hyg_type(body, cx, sc)?), *s)
        }
        Type::DepFun(n, a, mult, eff, b, s) => {
            let a = hyg_type(a, cx, sc)?;
            let f = fresh_name(cx, &n.text);
            sc.types.push((n.text.clone(), f.clone()));
            let b = hyg_type(b, cx, sc)?;
            sc.types.pop();
            Type::DepFun(Name::new(f, n.span), Box::new(a), mult.clone(), eff.clone(), Box::new(b), *s)
        }
        Type::Sigma(n, a, b, s) => {
            let a = hyg_type(a, cx, sc)?;
            let f = fresh_name(cx, &n.text);
            sc.types.push((n.text.clone(), f.clone()));
            let b = hyg_type(b, cx, sc)?;
            sc.types.pop();
            Type::Sigma(Name::new(f, n.span), Box::new(a), Box::new(b), *s)
        }
        Type::Unit(s) => Type::Unit(*s),
        Type::Tuple(ts, s) => {
            let mut out = Vec::new();
            for x in ts {
                out.push(hyg_type(x, cx, sc)?);
            }
            Type::Tuple(out, *s)
        }
        Type::Infix(ts, ops, s) => {
            let mut out = Vec::new();
            for x in ts {
                out.push(hyg_type(x, cx, sc)?);
            }
            Type::Infix(out, ops.clone(), *s)
        }
        Type::Num(v, s) => Type::Num(v.clone(), *s),
    })
}

fn hyg_local(d: &LocalDecl, cx: &mut HygCx, sc: &mut HygScope) -> Result<LocalDecl> {
    Ok(match d {
        LocalDecl::Sig { name, ty, span } => {
            let ty = hyg_type(ty, cx, sc)?;
            let f = fresh_name(cx, &name.text);
            sc.values.push((name.text.clone(), f.clone()));
            LocalDecl::Sig { name: Name::new(f, name.span), ty, span: *span }
        }
        LocalDecl::Eq { name, pats, body, span } => {
            let f = fresh_name(cx, &name.text);
            sc.values.push((name.text.clone(), f.clone()));
            let mut new_pats = Vec::new();
            for p in pats {
                new_pats.push(hyg_pat(p, cx, sc)?);
            }
            let body = hyg_term(body, cx, sc)?;
            LocalDecl::Eq { name: Name::new(f, name.span), pats: new_pats, body, span: *span }
        }
        LocalDecl::Bind { pat, body, span } => {
            let body = hyg_term(body, cx, sc)?;
            let pat = hyg_pat(pat, cx, sc)?;
            LocalDecl::Bind { pat, body, span: *span }
        }
        LocalDecl::Lazy { pat, body, span } => {
            let body = hyg_term(body, cx, sc)?;
            let pat = hyg_pat(pat, cx, sc)?;
            LocalDecl::Lazy { pat, body, span: *span }
        }
    })
}

fn hyg_alt(a: &Alt, cx: &mut HygCx, sc: &mut HygScope) -> Result<Alt> {
    let pat = hyg_pat(&a.pat, cx, sc)?;
    let guard = match &a.guard {
        Some(g) => Some(hyg_term(g, cx, sc)?),
        None => None,
    };
    let body = hyg_term(&a.body, cx, sc)?;
    Ok(Alt {
        attrs: a.attrs.clone(),
        pat,
        guard,
        guard_attrs: a.guard_attrs.clone(),
        body,
        span: a.span,
    })
}

fn hyg_do_stmt(s: &DoStmt, cx: &mut HygCx, sc: &mut HygScope) -> Result<DoStmt> {
    Ok(match s {
        DoStmt::Bind { pat, term, span } => {
            let term = hyg_term(term, cx, sc)?;
            let pat = hyg_pat(pat, cx, sc)?;
            DoStmt::Bind { pat, term, span: *span }
        }
        DoStmt::Let { decls, span } => DoStmt::Let {
            decls: decls
                .iter()
                .map(|d| hyg_local(d, cx, sc))
                .collect::<Result<Vec<_>>>()?,
            span: *span,
        },
        DoStmt::Expr { term, span } => DoStmt::Expr { term: hyg_term(term, cx, sc)?, span: *span },
    })
}

/// Alpha-rename one template term and qualify its free top-level names at
/// the definition site. `SpliceHole`s pass through: substituted syntax is
/// caller code and keeps its own binding relationships.
pub fn hyg_term(t: &Term, cx: &mut HygCx, sc: &mut HygScope) -> Result<Term> {
    Ok(match t {
        Term::Var(p) => {
            if p.quals.is_empty() {
                if let Some(f) = sc.lookup_value(&p.name.text) {
                    let f = f.to_string();
                    return Ok(Term::Var(VarPath {
                        quals: Vec::new(),
                        name: Name::new(f, p.name.span),
                    }));
                }
            }
            Term::Var(qualify_value(cx, p, p.span())?)
        }
        Term::Con(p) => Term::Con(qualify_con(cx, p, p.span())?),
        Term::Int(v, s) => Term::Int(v.clone(), *s),
        Term::Str(v, s) => Term::Str(v.clone(), *s),
        Term::Char(v, s) => Term::Char(v.clone(), *s),
        Term::Sym(v, s) => Term::Sym(v.clone(), *s),
        Term::Unit(s) => Term::Unit(*s),
        Term::Hole(s) => Term::Hole(*s),
        Term::SpliceHole { idx, span } => Term::SpliceHole { idx: *idx, span: *span },
        Term::SeqOp(k, s) => Term::SeqOp(*k, *s),
        Term::App(f, args, s) => {
            let f = hyg_term(f, cx, sc)?;
            let mut out = Vec::new();
            for a in args {
                out.push(hyg_term(a, cx, sc)?);
            }
            Term::App(Box::new(f), out, *s)
        }
        Term::Lam(pats, body, s) => {
            let mut new_pats = Vec::new();
            for p in pats {
                new_pats.push(hyg_pat(p, cx, sc)?);
            }
            let body = hyg_term(body, cx, sc)?;
            Term::Lam(new_pats, Box::new(body), *s)
        }
        Term::Let(decls, body, s) | Term::Letrec(decls, body, s) => {
            let rec = matches!(t, Term::Letrec(..));
            let mut new_decls = Vec::new();
            for d in decls {
                new_decls.push(hyg_local(d, cx, sc)?);
            }
            let body = hyg_term(body, cx, sc)?;
            if rec {
                Term::Letrec(new_decls, Box::new(body), *s)
            } else {
                Term::Let(new_decls, Box::new(body), *s)
            }
        }
        Term::If { cond, then, els, cond_attrs, then_attrs, else_attrs, span } => Term::If {
            cond: Box::new(hyg_term(cond, cx, sc)?),
            then: Box::new(hyg_term(then, cx, sc)?),
            els: Box::new(hyg_term(els, cx, sc)?),
            cond_attrs: cond_attrs.clone(),
            then_attrs: then_attrs.clone(),
            else_attrs: else_attrs.clone(),
            span: *span,
        },
        Term::Case(scrut, alts, s) => {
            let scrut = hyg_term(scrut, cx, sc)?;
            let mut out = Vec::new();
            for a in alts {
                out.push(hyg_alt(a, cx, sc)?);
            }
            Term::Case(Box::new(scrut), out, *s)
        }
        Term::Ann(e, ty, s) => Term::Ann(Box::new(hyg_term(e, cx, sc)?), hyg_type(ty, cx, sc)?, *s),
        Term::Forget { atoms, value, span } => Term::Forget {
            atoms: atoms
                .iter()
                .map(|t| hyg_type(t, cx, sc))
                .collect::<Result<Vec<_>>>()?,
            value: Box::new(hyg_term(value, cx, sc)?),
            span: *span,
        },
        Term::Narrow { value, target, witness, span } => Term::Narrow {
            value: Box::new(hyg_term(value, cx, sc)?),
            target: hyg_type(target, cx, sc)?,
            witness: Box::new(hyg_term(witness, cx, sc)?),
            span: *span,
        },
        Term::Infix(ts, ops, s) => {
            let mut out = Vec::new();
            for x in ts {
                out.push(hyg_term(x, cx, sc)?);
            }
            Term::Infix(out, ops.clone(), *s)
        }
        Term::Neg(e, s) => Term::Neg(Box::new(hyg_term(e, cx, sc)?), *s),
        Term::Tuple(ts, s) => {
            let mut out = Vec::new();
            for x in ts {
                out.push(hyg_term(x, cx, sc)?);
            }
            Term::Tuple(out, *s)
        }
        Term::List(ts, s) => {
            let mut out = Vec::new();
            for x in ts {
                out.push(hyg_term(x, cx, sc)?);
            }
            Term::List(out, *s)
        }
        Term::Map(kvs, s) => {
            let mut out = Vec::new();
            for (k, v) in kvs {
                out.push((hyg_term(k, cx, sc)?, hyg_term(v, cx, sc)?));
            }
            Term::Map(out, *s)
        }
        Term::Range(start, step, end, s) => Term::Range(
            Box::new(hyg_term(start, cx, sc)?),
            match step {
                Some(x) => Some(Box::new(hyg_term(x, cx, sc)?)),
                None => None,
            },
            Box::new(hyg_term(end, cx, sc)?),
            *s,
        ),
        Term::Proj(e, n, s) => Term::Proj(Box::new(hyg_term(e, cx, sc)?), n.clone(), *s),
        Term::Record(p, fields, s) => {
            let p = qualify_con(cx, p, *s)?;
            let mut out = Vec::new();
            for (n, e) in fields {
                out.push((n.clone(), hyg_term(e, cx, sc)?));
            }
            Term::Record(p, out, *s)
        }
        Term::Do(stmts, s) => {
            let mut out = Vec::new();
            for x in stmts {
                out.push(hyg_do_stmt(x, cx, sc)?);
            }
            Term::Do(out, *s)
        }
        Term::Handle(e, h, s) => Term::Handle(
            Box::new(hyg_term(e, cx, sc)?),
            Box::new(hyg_term(h, cx, sc)?),
            *s,
        ),
        Term::Handler(h) => {
            let mut h = (**h).clone();
            let ret_body = hyg_term(&h.ret_body, cx, sc)?;
            h.ret_body = ret_body;
            let ret_pat = hyg_pat(&h.ret_pat, cx, sc)?;
            h.ret_pat = ret_pat;
            let mut clauses = Vec::new();
            for c in &h.clauses {
                let mut c = c.clone();
                let mut pats = Vec::new();
                for p in &c.pats {
                    pats.push(hyg_pat(p, cx, sc)?);
                }
                c.pats = pats;
                if let Some(r) = &c.resume {
                    let f = fresh_name(cx, &r.text);
                    sc.values.push((r.text.clone(), f.clone()));
                    c.resume = Some(Name::new(f, r.span));
                }
                c.body = hyg_term(&c.body, cx, sc)?;
                if c.resume.is_some() {
                    sc.values.pop();
                }
                clauses.push(c);
            }
            h.clauses = clauses;
            Term::Handler(Box::new(h))
        }
        Term::With { name, value, body, span } => {
            let value = hyg_term(value, cx, sc)?;
            let f = fresh_name(cx, &name.text);
            sc.values.push((name.text.clone(), f.clone()));
            let body = hyg_term(body, cx, sc)?;
            sc.values.pop();
            Term::With {
                name: Name::new(f, name.span),
                value: Box::new(value),
                body: Box::new(body),
                span: *span,
            }
        }
        Term::CapApply { head, supplies, span } => {
            let head = hyg_term(head, cx, sc)?;
            let mut out = Vec::new();
            for (n, e) in supplies {
                out.push((n.clone(), hyg_term(e, cx, sc)?));
            }
            Term::CapApply { head: Box::new(head), supplies: out, span: *span }
        }
        Term::CpsWrap(e, s) => Term::CpsWrap(Box::new(hyg_term(e, cx, sc)?), *s),
        Term::Quote { body, span } => Term::Quote {
            body: Box::new(hyg_term(body, cx, sc)?),
            span: *span,
        },
        Term::QuoteDecls { decls, span } => {
            let mut out = Vec::new();
            for d in decls {
                out.push(hyg_decl(d, cx, sc)?);
            }
            Term::QuoteDecls { decls: out, span: *span }
        }
        Term::Splice { body, span } => Term::Splice {
            body: Box::new(hyg_term(body, cx, sc)?),
            span: *span,
        },
        Term::MacroCall { path, args, span } => {
            let path = qualify_value(cx, path, *span)?;
            Term::MacroCall { path, args: args.clone(), span: *span }
        }
    })
}

fn hyg_equation(eq: &Equation, cx: &mut HygCx, sc: &mut HygScope) -> Result<Equation> {
    // The equation's own name is a generated declaration's chosen name:
    // it stays as written and binds at the insertion site.
    let mut pats = Vec::new();
    for p in &eq.pats {
        pats.push(hyg_pat(p, cx, sc)?);
    }
    let mut locals = Vec::new();
    for l in &eq.locals {
        locals.push(hyg_local(l, cx, sc)?);
    }
    let body = hyg_term(&eq.body, cx, sc)?;
    Ok(Equation {
        attrs: eq.attrs.clone(),
        name: eq.name.clone(),
        pats,
        body,
        locals,
        span: eq.span,
    })
}

/// Alpha-rename and definition-site qualify one template declaration.
pub fn hyg_decl(d: &Decl, cx: &mut HygCx, sc: &mut HygScope) -> Result<Decl> {
    Ok(match d {
        Decl::Import(i) => Decl::Import(i.clone()),
        Decl::ImportData(d) => Decl::ImportData(d.clone()),
        Decl::Export(e) => Decl::Export(e.clone()),
        Decl::Fixity(f) => Decl::Fixity(f.clone()),
        Decl::OpaqueType(o) => Decl::OpaqueType(o.clone()),
        Decl::Foreign(f) => {
            let mut f = f.clone();
            f.ty = hyg_type(&f.ty, cx, sc)?;
            Decl::Foreign(f)
        }
        Decl::Attributed { attrs, inner } => Decl::Attributed {
            attrs: attrs.clone(),
            inner: Box::new(hyg_decl(inner, cx, sc)?),
        },
        Decl::Rewrite(r) => {
            let mut r = r.clone();
            for b in &mut r.binders {
                if let Some(ann) = &mut b.ann {
                    *ann = hyg_type(ann, cx, sc)?;
                }
            }
            for c in &mut r.ctx {
                match c {
                    Oblig::Class(c) => {
                        for a in &mut c.args {
                            *a = hyg_type(a, cx, sc)?;
                        }
                    }
                    Oblig::Pred(t) => *t = hyg_type(t, cx, sc)?,
                    Oblig::Preserves { pred, .. } => *pred = hyg_type(pred, cx, sc)?,
                }
            }
            r.match_tm = hyg_term(&r.match_tm, cx, sc)?;
            r.replace_tm = hyg_term(&r.replace_tm, cx, sc)?;
            if let Some(req) = &r.require {
                let t = hyg_term(req, cx, sc)?;
                r.require = Some(t);
            }
            r.justify = hyg_term(&r.justify, cx, sc)?;
            Decl::Rewrite(r)
        }
        Decl::Upgrade(u) => {
            let mut u = u.clone();
            if let Some(m) = &u.migrate {
                u.migrate = Some(hyg_term(m, cx, sc)?);
            }
            if let Some(r) = &u.related {
                u.related = Some(hyg_term(r, cx, sc)?);
            }
            Decl::Upgrade(u)
        }
        Decl::Contract(c) => {
            let mut c = c.clone();
            for cl in &mut c.clauses {
                match cl {
                    ContractClause::Formula { body, .. }
                    | ContractClause::AssumeProp { body } => *body = hyg_term(body, cx, sc)?,
                    ContractClause::AssumeNamed { .. } => {}
                }
            }
            Decl::Contract(c)
        }
        Decl::Generate { span, .. } => {
            return Err(Error::at(
                *span,
                "a generator cannot emit further `generate` declarations",
            ))
        }
        Decl::Signature(s) => {
            let mut s = s.clone();
            let mut methods = Vec::new();
            for (n, ty) in &s.methods {
                let ty = hyg_type(ty, cx, sc)?;
                let f = fresh_name(cx, &n.text);
                sc.values.push((n.text.clone(), f.clone()));
                methods.push((Name::new(f, n.span), ty));
            }
            s.methods = methods;
            // A default follows its method's fresh name, so it still
            // pairs with the method it defaults.
            let mut defaults = Vec::new();
            for eq in &s.defaults {
                let mut eq = hyg_equation(eq, cx, sc)?;
                if let Some((_, f)) = sc.values.iter().rev().find(|(orig, _)| *orig == eq.name.text) {
                    eq.name = Name::new(f.clone(), eq.name.span);
                }
                defaults.push(eq);
            }
            s.defaults = defaults;
            let mut laws = Vec::new();
            for (n, ty) in &s.laws {
                let ty = hyg_type(ty, cx, sc)?;
                let f = fresh_name(cx, &n.text);
                sc.values.push((n.text.clone(), f.clone()));
                laws.push((Name::new(f, n.span), ty));
            }
            s.laws = laws;
            Decl::Signature(s)
        }
        Decl::Prim(p) => {
            let mut p = p.clone();
            p.ty = hyg_type(&p.ty, cx, sc)?;
            for b in &mut p.bodies {
                if let Some(w) = &b.when {
                    b.when = Some(hyg_term(w, cx, sc)?);
                }
                if let Some(j) = &b.justify {
                    b.justify = Some(hyg_term(j, cx, sc)?);
                }
                for u in &mut b.uses {
                    u.ty = hyg_type(&u.ty, cx, sc)?;
                }
            }
            Decl::Prim(p)
        }
        Decl::Instance(i) => {
            let mut i = i.clone();
            i.target = hyg_type(&i.target, cx, sc)?;
            let mut methods = Vec::new();
            for m in &i.methods {
                methods.push(hyg_equation(m, cx, sc)?);
            }
            i.methods = methods;
            for (_, t) in &mut i.method_sigs {
                *t = hyg_type(t, cx, sc)?;
            }
            Decl::Instance(i)
        }
        Decl::Data(data) => {
            let mut data = data.clone();
            let mut params = Vec::new();
            for b in &data.params {
                let mut b = b.clone();
                if let Some(ann) = &b.ann {
                    b.ann = Some(hyg_type(ann, cx, sc)?);
                }
                let f = fresh_name(cx, &b.name.text);
                sc.types.push((b.name.text.clone(), f.clone()));
                b.name = Name::new(f, b.name.span);
                params.push(b);
            }
            let mut ctors = Vec::new();
            for c in &data.ctors {
                let mut c = c.clone();
                c.ty = hyg_type(&c.ty, cx, sc)?;
                ctors.push(c);
            }
            data.ctors = ctors;
            for _ in &params {
                sc.types.pop();
            }
            data.params = params;
            Decl::Data(data)
        }
        Decl::Refine(r) => {
            let mut r = r.clone();
            let mut params = Vec::new();
            for b in &r.params {
                let mut b = b.clone();
                if let Some(ann) = &b.ann {
                    b.ann = Some(hyg_type(ann, cx, sc)?);
                }
                let f = fresh_name(cx, &b.name.text);
                sc.types.push((b.name.text.clone(), f.clone()));
                b.name = Name::new(f, b.name.span);
                params.push(b);
            }
            // The refined binder is an index-position name of the
            // predicate: renamed with the type scope.
            let bf = fresh_name(cx, &r.binder.text);
            sc.types.push((r.binder.text.clone(), bf.clone()));
            r.binder = Name::new(bf, r.binder.span);
            r.base = hyg_type(&r.base, cx, sc)?;
            r.pred = hyg_type(&r.pred, cx, sc)?;
            sc.types.pop();
            for _ in &params {
                sc.types.pop();
            }
            r.params = params;
            Decl::Refine(r)
        }
        Decl::Record(r) | Decl::Codata(r) => {
            let is_codata = matches!(d, Decl::Codata(_));
            let mut r = r.clone();
            let mut params = Vec::new();
            for b in &r.params {
                let mut b = b.clone();
                if let Some(ann) = &b.ann {
                    b.ann = Some(hyg_type(ann, cx, sc)?);
                }
                let f = fresh_name(cx, &b.name.text);
                sc.types.push((b.name.text.clone(), f.clone()));
                b.name = Name::new(f, b.name.span);
                params.push(b);
            }
            let mut fields = Vec::new();
            for fd in &r.fields {
                let mut fd = fd.clone();
                fd.ty = hyg_type(&fd.ty, cx, sc)?;
                fields.push(fd);
            }
            r.fields = fields;
            for _ in &params {
                sc.types.pop();
            }
            r.params = params;
            if is_codata {
                Decl::Codata(r)
            } else {
                Decl::Record(r)
            }
        }
        Decl::Effect(e) => {
            let mut e = e.clone();
            let mut ops = Vec::new();
            for (n, ty) in &e.ops {
                let ty = hyg_type(ty, cx, sc)?;
                let f = fresh_name(cx, &n.text);
                sc.values.push((n.text.clone(), f.clone()));
                ops.push((Name::new(f, n.span), ty));
            }
            e.ops = ops;
            Decl::Effect(e)
        }
        Decl::ValueSig { name, modifier, owned_attrs, ty, span } => {
            let ty = hyg_type(ty, cx, sc)?;
            Decl::ValueSig {
                name: name.clone(),
                modifier: modifier.clone(),
                owned_attrs: owned_attrs.clone(),
                ty,
                span: *span,
            }
        }
        Decl::Equation { modifier, eq } => Decl::Equation {
            modifier: modifier.clone(),
            eq: hyg_equation(eq, cx, sc)?,
        },
        // The name slot is the generated declaration's chosen name: it
        // binds at the insertion site as written (the splice is caller
        // syntax and keeps its identity), and the inner declaration's
        // binders rename as usual.
        Decl::NameSpliced { name_tm, inner } => Decl::NameSpliced {
            name_tm: name_tm.clone(),
            inner: Box::new(hyg_decl(inner, cx, sc)?),
        },
    })
}

/// Substitute template holes with their spliced syntax (already
/// hygienically prepared): the substituted subtrees keep their own names.
pub fn subst_holes(t: &Term, vals: &[Term]) -> Term {
    match t {
        Term::SpliceHole { idx, .. } => vals[*idx].clone(),
        Term::Var(_) | Term::Con(_) | Term::Int(_, _) | Term::Str(_, _) | Term::Char(_, _) | Term::Sym(_, _)
        | Term::Unit(_) | Term::Hole(_) | Term::SeqOp(_, _) => t.clone(),
        Term::App(f, args, s) => Term::App(
            Box::new(subst_holes(f, vals)),
            args.iter().map(|a| subst_holes(a, vals)).collect(),
            *s,
        ),
        Term::Lam(pats, body, s) => {
            Term::Lam(pats.clone(), Box::new(subst_holes(body, vals)), *s)
        }
        Term::Let(decls, body, s) | Term::Letrec(decls, body, s) => {
            let rec = matches!(t, Term::Letrec(..));
            let new_decls: Vec<LocalDecl> =
                decls.iter().map(|d| subst_holes_local(d, vals)).collect();
            let body = Box::new(subst_holes(body, vals));
            if rec {
                Term::Letrec(new_decls, body, *s)
            } else {
                Term::Let(new_decls, body, *s)
            }
        }
        Term::If { cond, then, els, cond_attrs, then_attrs, else_attrs, span } => Term::If {
            cond: Box::new(subst_holes(cond, vals)),
            then: Box::new(subst_holes(then, vals)),
            els: Box::new(subst_holes(els, vals)),
            cond_attrs: cond_attrs.clone(),
            then_attrs: then_attrs.clone(),
            else_attrs: else_attrs.clone(),
            span: *span,
        },
        Term::Case(scrut, alts, s) => Term::Case(
            Box::new(subst_holes(scrut, vals)),
            alts
                .iter()
                .map(|a| Alt {
                    attrs: a.attrs.clone(),
                    pat: a.pat.clone(),
                    guard: a.guard.as_ref().map(|g| subst_holes(g, vals)),
                    guard_attrs: a.guard_attrs.clone(),
                    body: subst_holes(&a.body, vals),
                    span: a.span,
                })
                .collect(),
            *s,
        ),
        Term::Ann(e, ty, s) => Term::Ann(Box::new(subst_holes(e, vals)), ty.clone(), *s),
        Term::Forget { atoms, value, span } => Term::Forget {
            atoms: atoms.clone(),
            value: Box::new(subst_holes(value, vals)),
            span: *span,
        },
        Term::Narrow { value, target, witness, span } => Term::Narrow {
            value: Box::new(subst_holes(value, vals)),
            target: target.clone(),
            witness: Box::new(subst_holes(witness, vals)),
            span: *span,
        },
        Term::Infix(ts, ops, s) => {
            Term::Infix(ts.iter().map(|x| subst_holes(x, vals)).collect(), ops.clone(), *s)
        }
        Term::Neg(e, s) => Term::Neg(Box::new(subst_holes(e, vals)), *s),
        Term::Tuple(ts, s) => Term::Tuple(ts.iter().map(|x| subst_holes(x, vals)).collect(), *s),
        Term::List(ts, s) => Term::List(ts.iter().map(|x| subst_holes(x, vals)).collect(), *s),
        Term::Map(kvs, s) => Term::Map(
            kvs.iter()
                .map(|(k, v)| (subst_holes(k, vals), subst_holes(v, vals)))
                .collect(),
            *s,
        ),
        Term::Range(start, step, end, s) => Term::Range(
            Box::new(subst_holes(start, vals)),
            step.as_ref().map(|x| Box::new(subst_holes(x, vals))),
            Box::new(subst_holes(end, vals)),
            *s,
        ),
        Term::Proj(e, n, s) => Term::Proj(Box::new(subst_holes(e, vals)), n.clone(), *s),
        Term::Record(p, fields, s) => Term::Record(
            p.clone(),
            fields.iter().map(|(n, e)| (n.clone(), subst_holes(e, vals))).collect(),
            *s,
        ),
        Term::Do(stmts, s) => Term::Do(
            stmts
                .iter()
                .map(|x| match x {
                    DoStmt::Bind { pat, term, span } => DoStmt::Bind {
                        pat: pat.clone(),
                        term: subst_holes(term, vals),
                        span: *span,
                    },
                    DoStmt::Let { decls, span } => DoStmt::Let {
                        decls: decls.iter().map(|d| subst_holes_local(d, vals)).collect(),
                        span: *span,
                    },
                    DoStmt::Expr { term, span } => DoStmt::Expr {
                        term: subst_holes(term, vals),
                        span: *span,
                    },
                })
                .collect(),
            *s,
        ),
        Term::Handle(e, h, s) => Term::Handle(
            Box::new(subst_holes(e, vals)),
            Box::new(subst_holes(h, vals)),
            *s,
        ),
        Term::Handler(h) => {
            let mut h = (**h).clone();
            h.ret_body = subst_holes(&h.ret_body, vals);
            h.clauses = h
                .clauses
                .iter()
                .map(|c| {
                    let mut c = c.clone();
                    c.body = subst_holes(&c.body, vals);
                    c
                })
                .collect();
            Term::Handler(Box::new(h))
        }
        Term::With { name, value, body, span } => Term::With {
            name: name.clone(),
            value: Box::new(subst_holes(value, vals)),
            body: Box::new(subst_holes(body, vals)),
            span: *span,
        },
        Term::CapApply { head, supplies, span } => Term::CapApply {
            head: Box::new(subst_holes(head, vals)),
            supplies: supplies
                .iter()
                .map(|(n, e)| (n.clone(), subst_holes(e, vals)))
                .collect(),
            span: *span,
        },
        Term::CpsWrap(e, s) => Term::CpsWrap(Box::new(subst_holes(e, vals)), *s),
        Term::Quote { body, span } => Term::Quote {
            body: Box::new(subst_holes(body, vals)),
            span: *span,
        },
        Term::QuoteDecls { decls, span } => Term::QuoteDecls {
            decls: decls.iter().map(|d| subst_holes_decl(d, vals)).collect(),
            span: *span,
        },
        Term::Splice { body, span } => Term::Splice {
            body: Box::new(subst_holes(body, vals)),
            span: *span,
        },
        Term::MacroCall { .. } => t.clone(),
    }
}

fn subst_holes_local(d: &LocalDecl, vals: &[Term]) -> LocalDecl {
    match d {
        LocalDecl::Sig { .. } => d.clone(),
        LocalDecl::Eq { name, pats, body, span } => LocalDecl::Eq {
            name: name.clone(),
            pats: pats.clone(),
            body: subst_holes(body, vals),
            span: *span,
        },
        LocalDecl::Bind { pat, body, span } => LocalDecl::Bind {
            pat: pat.clone(),
            body: subst_holes(body, vals),
            span: *span,
        },
        LocalDecl::Lazy { pat, body, span } => LocalDecl::Lazy {
            pat: pat.clone(),
            body: subst_holes(body, vals),
            span: *span,
        },
    }
}

/// Substitute template holes in a declaration template.
pub fn subst_holes_decl(d: &Decl, vals: &[Term]) -> Decl {
    match d {
        Decl::Signature(s) => {
            let mut s = s.clone();
            for eq in &mut s.defaults {
                eq.body = subst_holes(&eq.body, vals);
                eq.locals = eq.locals.iter().map(|l| subst_holes_local(l, vals)).collect();
            }
            Decl::Signature(s)
        }
        Decl::Import(_) | Decl::ImportData(_) | Decl::Export(_) | Decl::Fixity(_)
        | Decl::Foreign(_) | Decl::OpaqueType(_)
        | Decl::Data(_) | Decl::Refine(_) | Decl::Record(_) | Decl::Codata(_) | Decl::Effect(_)
        | Decl::ValueSig { .. } | Decl::Generate { .. } => d.clone(),
        Decl::Attributed { attrs, inner } => Decl::Attributed {
            attrs: attrs.clone(),
            inner: Box::new(subst_holes_decl(inner, vals)),
        },
        Decl::NameSpliced { name_tm, inner } => Decl::NameSpliced {
            name_tm: Box::new(subst_holes(name_tm, vals)),
            inner: Box::new(subst_holes_decl(inner, vals)),
        },
        Decl::Rewrite(r) => {
            let mut r = r.clone();
            r.match_tm = subst_holes(&r.match_tm, vals);
            r.replace_tm = subst_holes(&r.replace_tm, vals);
            r.require = r.require.as_ref().map(|t| subst_holes(t, vals));
            r.justify = subst_holes(&r.justify, vals);
            Decl::Rewrite(r)
        }
        Decl::Upgrade(u) => {
            let mut u = u.clone();
            u.migrate = u.migrate.as_ref().map(|t| subst_holes(t, vals));
            u.related = u.related.as_ref().map(|t| subst_holes(t, vals));
            Decl::Upgrade(u)
        }
        Decl::Prim(p) => {
            let mut p = p.clone();
            for b in &mut p.bodies {
                b.when = b.when.as_ref().map(|t| subst_holes(t, vals));
                b.justify = b.justify.as_ref().map(|t| subst_holes(t, vals));
            }
            Decl::Prim(p)
        }
        Decl::Contract(c) => {
            let mut c = c.clone();
            for cl in &mut c.clauses {
                match cl {
                    ContractClause::Formula { body, .. }
                    | ContractClause::AssumeProp { body } => *body = subst_holes(body, vals),
                    ContractClause::AssumeNamed { .. } => {}
                }
            }
            Decl::Contract(c)
        }
        Decl::Instance(i) => {
            let mut i = i.clone();
            i.methods = i
                .methods
                .iter()
                .map(|m| {
                    let mut m = m.clone();
                    m.body = subst_holes(&m.body, vals);
                    m.locals = m.locals.iter().map(|l| subst_holes_local(l, vals)).collect();
                    m
                })
                .collect();
            Decl::Instance(i)
        }
        Decl::Equation { modifier, eq } => {
            let mut eq = eq.clone();
            eq.body = subst_holes(&eq.body, vals);
            eq.locals = eq.locals.iter().map(|l| subst_holes_local(l, vals)).collect();
            Decl::Equation { modifier: modifier.clone(), eq }
        }
    }
}

/// A template hole's source: a `$( e )` splice body or a macro invocation
/// `m!( ... )`. Both produce code or syntax that fills the hole when the
/// quotation is constructed. The `Name` form is a name-position splice's
/// body (a declaration's name slot; D46), whose `Name`/`Sym`/name-quote
/// value names the generated declaration.
#[derive(Clone, Debug)]
pub enum HoleExpr {
    Term(Box<Term>),
    Name(Box<Term>),
    Macro { path: VarPath, args: TokenTree, span: Span },
}

/// Replace every splice and macro invocation of `t` with `SpliceHole`
/// markers, collecting their code-producing expressions in order. Splice
/// bodies are compile-time expressions and are not descended into; a splice
/// inside a nested quotation belongs to the enclosing template (modules
/// specification section 8).
pub fn collect_template_holes(t: &mut Term, holes: &mut Vec<HoleExpr>) {
    match t {
        Term::Var(_)
        | Term::Con(_)
        | Term::Int(_, _)
        | Term::Str(_, _)
        | Term::Char(_, _) | Term::Sym(_, _)
        | Term::Unit(_)
        | Term::Hole(_)
        | Term::SeqOp(_, _)
        | Term::SpliceHole { .. } => {}
        Term::Splice { body, span } => {
            let idx = holes.len();
            holes.push(HoleExpr::Term(body.clone()));
            *t = Term::SpliceHole { idx, span: *span };
        }
        Term::MacroCall { path, args, span } => {
            let idx = holes.len();
            holes.push(HoleExpr::Macro {
                path: path.clone(),
                args: args.clone(),
                span: *span,
            });
            *t = Term::SpliceHole { idx, span: *span };
        }
        Term::App(f, args, _) => {
            collect_template_holes(f, holes);
            for a in args {
                collect_template_holes(a, holes);
            }
        }
        Term::Lam(_, body, _) => collect_template_holes(body, holes),
        Term::Let(decls, body, _) | Term::Letrec(decls, body, _) => {
            for d in decls {
                match d {
                    LocalDecl::Eq { body, .. }
                    | LocalDecl::Bind { body, .. }
                    | LocalDecl::Lazy { body, .. } => collect_template_holes(body, holes),
                    LocalDecl::Sig { .. } => {}
                }
            }
            collect_template_holes(body, holes);
        }
        Term::If { cond, then, els, .. } => {
            collect_template_holes(cond, holes);
            collect_template_holes(then, holes);
            collect_template_holes(els, holes);
        }
        Term::Case(scrut, alts, _) => {
            collect_template_holes(scrut, holes);
            for alt in alts {
                if let Some(g) = &mut alt.guard {
                    collect_template_holes(g, holes);
                }
                collect_template_holes(&mut alt.body, holes);
            }
        }
        Term::Ann(e, _, _) => collect_template_holes(e, holes),
        Term::Forget { value, .. } => collect_template_holes(value, holes),
        Term::Narrow { value, witness, .. } => {
            collect_template_holes(value, holes);
            collect_template_holes(witness, holes);
        }
        Term::Infix(ts, _, _) => {
            for x in ts {
                collect_template_holes(x, holes);
            }
        }
        Term::Neg(e, _) => collect_template_holes(e, holes),
        Term::Tuple(ts, _) | Term::List(ts, _) => {
            for x in ts {
                collect_template_holes(x, holes);
            }
        }
        Term::Map(kvs, _) => {
            for (k, v) in kvs {
                collect_template_holes(k, holes);
                collect_template_holes(v, holes);
            }
        }
        Term::Range(start, step, end, _) => {
            collect_template_holes(start, holes);
            if let Some(x) = step {
                collect_template_holes(x, holes);
            }
            collect_template_holes(end, holes);
        }
        Term::Proj(e, _, _) => collect_template_holes(e, holes),
        Term::Record(_, fields, _) => {
            for (_, e) in fields {
                collect_template_holes(e, holes);
            }
        }
        Term::Do(stmts, _) => {
            for s in stmts {
                match s {
                    DoStmt::Bind { term, .. } | DoStmt::Expr { term, .. } => {
                        collect_template_holes(term, holes)
                    }
                    DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            match decl {
                                LocalDecl::Eq { body, .. }
                                | LocalDecl::Bind { body, .. }
                                | LocalDecl::Lazy { body, .. } => {
                                    collect_template_holes(body, holes)
                                }
                                LocalDecl::Sig { .. } => {}
                            }
                        }
                    }
                }
            }
        }
        Term::Handle(e, h, _) => {
            collect_template_holes(e, holes);
            collect_template_holes(h, holes);
        }
        Term::Handler(h) => {
            collect_template_holes(&mut h.ret_body, holes);
            for c in &mut h.clauses {
                collect_template_holes(&mut c.body, holes);
            }
        }
        Term::With { value, body, .. } => {
            collect_template_holes(value, holes);
            collect_template_holes(body, holes);
        }
        Term::CapApply { head, supplies, .. } => {
            collect_template_holes(head, holes);
            for (_, e) in supplies {
                collect_template_holes(e, holes);
            }
        }
        Term::CpsWrap(e, _) => collect_template_holes(e, holes),
        Term::Quote { body, .. } => collect_template_holes(body, holes),
        Term::QuoteDecls { decls, .. } => collect_decl_template_holes(decls, holes),
    }
}

/// The declaration-template form of [`collect_template_holes`]: holes are
/// collected from every term of the quoted declarations.
pub fn collect_decl_template_holes(decls: &mut Vec<Decl>, holes: &mut Vec<HoleExpr>) {
    for d in decls.iter_mut() {
        collect_decl_holes(d, holes);
    }
}

fn collect_decl_holes(d: &mut Decl, holes: &mut Vec<HoleExpr>) {
    match d {
        Decl::Import(_)
        | Decl::ImportData(_)
        | Decl::Export(_)
        | Decl::Fixity(_)
        | Decl::Foreign(_)
        | Decl::OpaqueType(_)
        | Decl::Refine(_)
        | Decl::ValueSig { .. } => {}
        Decl::Attributed { inner, .. } => collect_decl_holes(inner, holes),
        Decl::NameSpliced { name_tm, inner } => {
            // A name-position splice's body is a compile-time expression
            // like any splice body: it becomes one `Name` hole and the
            // slot becomes its `SpliceHole` marker (the inner
            // declaration's term holes collect after it).
            match &mut **name_tm {
                Term::Splice { body, span } => {
                    let idx = holes.len();
                    holes.push(HoleExpr::Name(body.clone()));
                    **name_tm = Term::SpliceHole { idx, span: *span };
                }
                _ => {}
            }
            collect_decl_holes(inner, holes);
        }
        Decl::Rewrite(r) => {
            collect_template_holes(&mut r.match_tm, holes);
            collect_template_holes(&mut r.replace_tm, holes);
            if let Some(req) = &mut r.require {
                collect_template_holes(req, holes);
            }
            collect_template_holes(&mut r.justify, holes);
        }
        Decl::Upgrade(u) => {
            if let Some(m) = &mut u.migrate {
                collect_template_holes(m, holes);
            }
            if let Some(r) = &mut u.related {
                collect_template_holes(r, holes);
            }
        }
        Decl::Contract(c) => {
            for cl in &mut c.clauses {
                match cl {
                    ContractClause::Formula { body, .. }
                    | ContractClause::AssumeProp { body } => collect_template_holes(body, holes),
                    ContractClause::AssumeNamed { .. } => {}
                }
            }
        }
        Decl::Generate { .. } => {}
        Decl::Signature(s) => {
            for eq in &mut s.defaults {
                collect_template_holes(&mut eq.body, holes);
                for l in &mut eq.locals {
                    if let LocalDecl::Eq { body, .. }
                    | LocalDecl::Bind { body, .. }
                    | LocalDecl::Lazy { body, .. } = l
                    {
                        collect_template_holes(body, holes);
                    }
                }
            }
        }
        Decl::Instance(i) => {
            for m in &mut i.methods {
                collect_template_holes(&mut m.body, holes);
                for l in &mut m.locals {
                    if let LocalDecl::Eq { body, .. }
                    | LocalDecl::Bind { body, .. }
                    | LocalDecl::Lazy { body, .. } = l
                    {
                        collect_template_holes(body, holes);
                    }
                }
            }
        }
        Decl::Data(_) | Decl::Record(_) | Decl::Codata(_) | Decl::Effect(_) => {}
        Decl::Prim(p) => {
            for b in &mut p.bodies {
                if let Some(w) = &mut b.when {
                    collect_template_holes(w, holes);
                }
                if let Some(j) = &mut b.justify {
                    collect_template_holes(j, holes);
                }
            }
        }
        Decl::Equation { eq, .. } => {
            collect_template_holes(&mut eq.body, holes);
            for l in &mut eq.locals {
                if let LocalDecl::Eq { body, .. }
                | LocalDecl::Bind { body, .. }
                | LocalDecl::Lazy { body, .. } = l
                {
                    collect_template_holes(body, holes);
                }
            }
        }
    }
}

// --------------------------------------------------- WIT: component interfaces

/// A parsed WIT package (WebAssembly and WASI specification section 3):
/// `package ns:name@version;` with the `interface` and `world` items it
/// declares. WIT describes shapes and resource interfaces only; the
/// functional laws of a binding stay in the language's own contracts.
#[derive(Clone, Debug)]
pub struct WitPackage {
    /// `ns:name`, e.g. `example:codec`.
    pub name: String,
    /// The `@version` suffix when written.
    pub version: Option<String>,
    pub interfaces: Vec<WitInterface>,
    pub worlds: Vec<WitWorld>,
    pub span: Span,
}

impl WitPackage {
    /// The package identity recorded in artifacts: `ns:name@version`
    /// with the version spelled `@0` when absent.
    pub fn identity(&self) -> String {
        match &self.version {
            Some(v) => format!("{}@{}", self.name, v),
            None => format!("{}@0", self.name),
        }
    }

    /// The interface named `name`, if the package declares it.
    pub fn interface(&self, name: &str) -> Option<&WitInterface> {
        self.interfaces.iter().find(|i| i.name == name)
    }
}

#[derive(Clone, Debug)]
pub struct WitInterface {
    pub name: String,
    pub items: Vec<WitItem>,
    pub span: Span,
}

impl WitInterface {
    /// The type definitions of the interface, in declaration order.
    pub fn typedefs(&self) -> impl Iterator<Item = &WitTypeDef> {
        self.items.iter().filter_map(|i| match i {
            WitItem::Type(t) => Some(t),
            _ => None,
        })
    }

    /// The functions of the interface, in declaration order.
    pub fn funcs(&self) -> impl Iterator<Item = &WitFunc> {
        self.items.iter().filter_map(|i| match i {
            WitItem::Func(f) => Some(f),
            _ => None,
        })
    }

    /// The resources of the interface, in declaration order.
    pub fn resources(&self) -> impl Iterator<Item = &WitResource> {
        self.items.iter().filter_map(|i| match i {
            WitItem::Resource(r) => Some(r),
            _ => None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum WitItem {
    Type(WitTypeDef),
    Func(WitFunc),
    Resource(WitResource),
}

#[derive(Clone, Debug)]
pub struct WitTypeDef {
    pub name: String,
    pub kind: WitTypeDefKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum WitTypeDefKind {
    /// `enum e { a, b }`: payloads are forbidden.
    Enum(Vec<WitCase>),
    /// `record r { x: t, ... }`.
    Record(Vec<WitField>),
    /// `variant v { a(t), b }`: cases carry optional payloads.
    Variant(Vec<WitCase>),
    /// `flags f { a, b }`.
    Flags(Vec<String>),
    /// `union u { t1, t2 }`: a variant with positional cases.
    Union(Vec<WitType>),
    /// `type n = t`.
    Alias(WitType),
}

#[derive(Clone, Debug)]
pub struct WitCase {
    pub name: String,
    pub payload: Option<WitType>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct WitField {
    pub name: String,
    pub ty: WitType,
    pub span: Span,
}

/// A `resource r { ... }`: an opaque host-owned object whose values cross
/// the boundary as `own<r>`/`borrow<r>` handles, never as host pointers
/// (WebAssembly and WASI specification section 4).
#[derive(Clone, Debug)]
pub struct WitResource {
    pub name: String,
    pub items: Vec<WitResourceItem>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum WitResourceItem {
    /// `constructor(x: t, ...);`.
    Constructor { params: Vec<WitParam>, span: Span },
    /// `m: func(x: t, ...) -> u;`.
    Method {
        name: String,
        params: Vec<WitParam>,
        result: Option<WitType>,
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub struct WitFunc {
    pub name: String,
    pub params: Vec<WitParam>,
    pub result: Option<WitType>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct WitParam {
    pub name: String,
    pub ty: WitType,
    pub span: Span,
}

/// A WIT value type (the interface type grammar).
#[derive(Clone, Debug)]
pub enum WitType {
    Bool(Span),
    U8(Span),
    U16(Span),
    U32(Span),
    U64(Span),
    S8(Span),
    S16(Span),
    S32(Span),
    S64(Span),
    F32(Span),
    F64(Span),
    Char(Span),
    Str(Span),
    List(Box<WitType>, Span),
    Option(Box<WitType>, Span),
    /// `result`, `result<t>`, `result<_, e>`, `result<t, e>`: the ok and
    /// err payloads, each optional.
    Result(Option<Box<WitType>>, Option<Box<WitType>>, Span),
    Tuple(Vec<WitType>, Span),
    /// A reference to a named type of the package.
    Named(String, Span),
    /// `own<r>`: an owned resource handle (ownership transfers).
    Own(String, Span),
    /// `borrow<r>`: a borrowed resource handle (scoped, never owned).
    Borrow(String, Span),
    /// `stream<t>` (WASI 0.3): an asynchronous source or sink, never a
    /// productive `Stream a` (WebAssembly and WASI specification
    /// section 6).
    Stream(Box<WitType>, Span),
    /// `future<t>` (WASI 0.3): a fallible asynchronous completion.
    Future(Box<WitType>, Span),
}

impl WitType {
    pub fn span(&self) -> Span {
        match self {
            WitType::Bool(s)
            | WitType::U8(s)
            | WitType::U16(s)
            | WitType::U32(s)
            | WitType::U64(s)
            | WitType::S8(s)
            | WitType::S16(s)
            | WitType::S32(s)
            | WitType::S64(s)
            | WitType::F32(s)
            | WitType::F64(s)
            | WitType::Char(s)
            | WitType::Str(s)
            | WitType::List(_, s)
            | WitType::Option(_, s)
            | WitType::Result(_, _, s)
            | WitType::Tuple(_, s)
            | WitType::Named(_, s)
            | WitType::Own(_, s)
            | WitType::Borrow(_, s)
            | WitType::Stream(_, s)
            | WitType::Future(_, s) => *s,
        }
    }
}

/// A `world`: the imports and exports of a component, naming package
/// interfaces (WebAssembly and WASI specification sections 2 and 5: a
/// world lists its imports; linking supplies the concrete capabilities).
#[derive(Clone, Debug)]
pub struct WitWorld {
    pub name: String,
    pub imports: Vec<WitWorldItem>,
    pub exports: Vec<WitWorldItem>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct WitWorldItem {
    /// The referenced interface name (`export decoder;`).
    pub iface: String,
    pub span: Span,
}
