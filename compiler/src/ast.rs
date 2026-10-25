//! Abstract syntax for the draft 0.2 surface, restricted to the bootstrap
//! subset (see `compiler/README.md`). Unsupported forms parse and are
//! rejected by the checker or parser with an explicit message.

pub use crate::lexer::Span;

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
    Using(Vec<Constraint>, Box<Type>, Span),
    /// Dependent function `(x :: A) -> B` with optional multiplicity and
    /// effect row.
    DepFun(Name, Box<Type>, Option<Name>, Option<EffectRow>, Box<Type>, Span),
    /// Dependent pair `Sigma (n :: A). B`.
    Sigma(Name, Box<Type>, Box<Type>, Span),
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
            | Pat::Int(_, s)
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
    pub pat: Pat,
    pub guard: Option<crate::ast::Term>,
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

/// One statement of a `do` block: `pat <- term`, `let` bindings, or a
/// bare `term`.
#[derive(Clone, Debug)]
pub enum DoStmt {
    Bind { pat: Pat, term: Term, span: Span },
    /// `let decls`: a pure local binding ("`let` remains pure and strict
    /// in either interpretation").
    Let { decl: LocalDecl, span: Span },
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
    /// Integer literal: the decimal digit string, with a leading `-` when
    /// prefix negation folded into it (same form as [`Pat::Int`]).
    Int(String, Span),
    Str(String, Span),
    Char(String, Span),
    Unit(Span),
    Hole(Span),
    /// Left-associated curried application of a nonempty argument spine.
    App(Box<Term>, Vec<Term>, Span),
    /// `\p1 .. pn -> body`.
    Lam(Vec<Pat>, Box<Term>, Span),
    /// `let block in body`.
    Let(Vec<LocalDecl>, Box<Term>, Span),
    If(Box<Term>, Box<Term>, Box<Term>, Span),
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
}

impl Term {
    pub fn span(&self) -> Span {
        match self {
            Term::Var(p) => p.span(),
            Term::Con(p) => p.span(),
            Term::Int(_, s)
            | Term::Str(_, s)
            | Term::Char(_, s)
            | Term::Unit(s)
            | Term::Hole(s)
            | Term::App(_, _, s)
            | Term::Lam(_, _, s)
            | Term::Let(_, _, s)
            | Term::If(_, _, _, s)
            | Term::Case(_, _, s)
            | Term::Ann(_, _, s)
            | Term::Infix(_, _, s)
            | Term::Neg(_, s)
            | Term::Tuple(_, s)
            | Term::List(_, s)
            | Term::Proj(_, _, s)
            | Term::Record(_, _, s)
            | Term::Do(_, s)
            | Term::Handle(_, _, s)
            | Term::With { span: s, .. }
            | Term::CapApply { span: s, .. }
            | Term::SeqOp(_, s)
            | Term::CpsWrap(_, s) => *s,
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

#[derive(Clone, Debug)]
pub struct Import {
    pub path: ConPath,
    pub alias: Option<Name>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct SigDecl {
    pub name: Name,
    pub params: Vec<Name>,
    pub methods: Vec<(Name, Type)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Equation {
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
    pub ctx: Vec<Constraint>,
    pub target: Type,
    pub methods: Vec<Equation>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct CtorDecl {
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

#[derive(Clone, Debug)]
pub enum Decl {
    Import(Import),
    Signature(SigDecl),
    Instance(InstDecl),
    Data(DataDecl),
    Record(RecordDecl),
    /// `name :: ty` (with optional unsupported modifier in `modifier`).
    ValueSig { name: Name, modifier: Option<Name>, ty: Type, span: Span },
    Equation { modifier: Option<Name>, eq: Equation },
    Foreign(ForeignDecl),
    Fixity(FixityDecl),
    Effect(EffectDecl),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Name,
    pub imports: Vec<Import>,
    pub decls: Vec<Decl>,
    pub span: Span,
}
