//! Type checking and elaboration into [`crate::core`].
//!
//! The checker is bidirectional with explicit `forall` quantification (this
//! draft does not silently generalize). Constraints from `using (...)`
//! contexts become dictionary parameters; method calls elaborate to
//! dictionary projection plus application; dictionary selection is solved
//! from visible `using` binders or unique `implicit instance` declarations.
//! Operators are ordinary value names, so `x + y` is just a method call.

use crate::ast;
use crate::repr::Class;
use crate::core::{ArithMode, ArithOp, BytesOp, CAlt, CBranch, CPat, CmpOp, ConcOp, Core, FieldAccess, FieldShape, FnDef, GlobalId, InstanceDef, IntOp, LocalId, NatOp, PrimOp, Program, ProjField, SymOp};
use crate::core::{WHEN_VOCABULARY, WhenAtom, WhenCond, WhenKind};
use crate::types::{BYTE_LENGTH_INDEX, ClassApp, EffLabel, EffRow, EitherMeta, IntSort, IxParam, IndexHead, IndexInstantiation, IndexArgument, IndexEvidence, ITerm, IdxCmp, IdxOp, Mult, PTerm, PredFn, PropOblig, PtrRole, RowTail, Scalar, Scheme, Sort, Sorts, Ty, Unifier, c_abi_scalar, collect_row_names, default_mult, linear_entails, pred_alpha_eq, pred_binder, pred_conjuncts, pred_entails, pred_implies};
use crate::{Error, Result};
use SolveOutcome::{Solved, Ambiguous};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------- elaboration IR

#[derive(Clone, Debug)]
enum ECore {
    /// A raw `i64` word constant (machine scalar values).
    Int(i64),
    /// A mathematical `Nat`/`Int` constant (decimal digit string with an
    /// optional leading `-`; `nat` selects the `Nat` runtime).
    ConstBig { nat: bool, digits: String },
    /// A constant `Bytes` buffer (the compile-time fold of a collection
    /// literal; [`crate::core::Core::ConstBytes`] is its target form).
    ConstBytes(Vec<u8>),
    /// The runtime `String` constant (a string literal's value; the
    /// shared read-only data-section object of
    /// [`crate::core::Core::ConstString`], whose `ctor` this carries).
    ConstString {
        ctor: crate::core::GlobalId,
        bytes: Vec<u8>,
    },
    /// A constant `Sym`: the interned name of a tick literal
    /// ([`crate::core::Core::ConstSym`] is its target form).
    ConstSym(String),
    /// A collection literal's class desugar awaiting its compile-time
    /// fold (surface grammar local amendments section 10): `body` is the
    /// elaborated `from-list`/`from-map` call and `idx` indexes
    /// `Checker::literal_folds`, whose record the fold pass fills. A
    /// site that does not fold lowers to its class call.
    LitFold { idx: usize, body: Box<ECore> },
    /// A deferred integer literal ([`Checker::literal_pends`]): its
    /// constant form is fixed once the literal's type is determined, then
    /// it lowers to `Int` or `ConstBig`.
    Lit(usize),
    Bool(bool),
    Unit,
    Var(LocalId),
    /// A top-level unit, with the types its scheme's type parameters
    /// take at this use (empty: no instantiation was recorded, which
    /// lowering reads as the enclosing definition's own parameters for
    /// its hoisted units, and as monomorphic otherwise).
    Global(GlobalId, Vec<Ty>),
    /// A constructor, with its data type's parameter types at this use.
    Ctor(GlobalId, Vec<Ty>),
    Prim(PrimOp),
    /// A real foreign import used as a value (its wrapper unit's name).
    Foreign(String),
    /// A method projection, with the signature and the types of the
    /// method's own type parameters at this use, and the selected
    /// method value's type (its representation class at lowering).
    DictProj(Box<ECore>, usize, Option<(String, Vec<Ty>)>, Ty),
    /// Record field projection: load the field from the record block.
    /// The type is the field's, for its representation class.
    Proj(Box<ECore>, crate::core::ProjField, Ty),
    /// A dictionary, with the instance's own parameter types.
    InstDict(GlobalId, Vec<ECore>, Vec<Ty>, bool),
    App(Box<ECore>, Vec<ECore>),
    /// Un-hoisted lambda: `params` are its pattern parameters.
    Lam { params: Vec<LocalId>, body: Box<ECore> },
    /// Un-hoisted generalisation site (design/rank-n-polymorphism.md
    /// section 4): `body` was checked at the fresh rigid `skolems`, the
    /// polytype's type binders in order. Hoisting turns it into an
    /// anonymous instance of the `#poly.<k>` family and an `InstDict`.
    PolyIntro { skolems: Vec<String>, body: Box<ECore>, ty: Ty },
    /// Hoisted form after [`hoist_lambdas`].
    MkClos { fn_id: GlobalId, env: Vec<ECore> },
    Let(LocalId, Box<ECore>, Box<ECore>),
    /// A two-way decision point: the condition and its two weighted
    /// branches (attribute annotations specification section 2.1).
    If(Box<ECore>, CBranch<ECore>, CBranch<ECore>),
    /// Case on a boxed ADT value; the alternatives carry their weights
    /// beside them (attribute annotations specification section 2.1).
    Case { scrut: Box<ECore>, alts: Vec<CAlt<ECore>> },
    /// Demand a memo cell (its state machine runs at code generation),
    /// with the cached value's type.
    Demand(Box<ECore>, Ty),
    /// Allocate an empty memo cell.
    LazyAlloc,
    /// Store a thunk into a memo cell.
    LazyInit(LocalId, Box<ECore>),
    /// Allocate an empty one-word fixpoint box: the knot for a recursive
    /// `letrec` value whose uses sit under thunks (surface grammar local
    /// amendments section 9).
    BoxAlloc,
    /// Evaluate `value`, store it into the box bound to `box_l`, and
    /// return it; the type is the value's.
    BoxInit(LocalId, Box<ECore>, Ty),
    /// Read a fixpoint box: a use of a recursive `letrec` value inside a
    /// thunk, resolved when the thunk runs; the type is the value's.
    BoxGet(Box<ECore>, Ty),
    /// Release a scoped knot’s payload and invalidate subsequent reads.
    BoxClear(Box<ECore>, Ty),
    /// Write through a `Mut` borrow (evidence invalidation over mutable
    /// state): store `val` into the cell the exclusive borrow `local`
    /// names, yielding `Unit`. Lowers to [`crate::core::Core::RefSet`].
    RefSet(LocalId, Box<ECore>, Ty),
    /// The runtime fault path (exit 70): emitted when a match falls through
    /// with pattern coverage unprovable.
    Fault,
    /// Unresolved dictionary, filled before lowering.
    Hole(usize),
    /// A splice-hole position inside a quotation's retained body: the
    /// quote block captured at hole `idx` (supercompilation and JIT
    /// specification section 7).
    QuoteHole(usize),
    /// Deferred implicit index argument, reified once unification has
    /// solved it ([`Checker::resolved_idx_args`]).
    IdxPending(usize),
    /// A typed quotation template under construction (staging): the
    /// template with `SpliceHole` placeholders and the elaborated splice
    /// expressions evaluated at construction (modules specification
    /// section 7).
    Quote {
        template: Box<crate::ast::Term>,
        holes: Vec<ECore>,
        /// The retained elaboration of the template in closed
        /// runtime-side scope: the reified program `eval` runs.
        body: Box<ECore>,
    },
    /// A `quoteDecls` template under construction (modules specification
    /// section 8). `name_holes` marks the holes that fill declaration
    /// name slots (D46 name splices).
    DeclsQuote {
        template: Vec<crate::ast::Decl>,
        holes: Vec<ECore>,
        name_holes: Vec<usize>,
    },
    /// A macro argument's raw token tree (`Syntax` values).
    SyntaxLit(Vec<crate::lexer::Token>, crate::syntax::SyntaxScope),
    /// A compile-time string constant (`String` values).
    StrLit(String),
}

#[derive(Clone, Debug)]
enum DictExpr<B = LocalId> {
    Binder(B),
    /// An instance dictionary: the instance, its sub-dictionaries, and
    /// the types its own parameters matched.
    Instance(GlobalId, Vec<DictExpr<B>>, Vec<Ty>),
    /// Structural evidence for the compiler-known `Copy` class: a marker
    /// class has no methods, so its dictionary is a plain unit word.
    Marker,
}

/// One want's resolution: determined now, or several instances still match
/// and the fixpoint must retry after the other wants commit.
enum SolveOutcome<B = LocalId> {
    Solved(DictExpr<B>),
    Ambiguous,
}

/// A dictionary binder in an index telescope. Header parameters are bound
/// by their Scheme context; runtime binders come only from actual LocalIds.
#[derive(Clone)]
struct IndexDictionaryBinding(ITerm);

impl From<LocalId> for IndexDictionaryBinding {
    fn from(local: LocalId) -> Self {
        Self(ITerm::Evidence(IndexEvidence::Local(local.0)))
    }
}

fn dictionary_parameter(name: &str) -> IndexEvidence {
    IndexEvidence::Parameter(name.into())
}

impl<B> DictExpr<B> {
    fn index_with(&self, uni: &Unifier, binder: &dyn Fn(&B) -> ITerm) -> ITerm {
        let (head, types, dictionaries) = match self {
            Self::Binder(binding) => return binder(binding),
            Self::Instance(head, dictionaries, types) => (IndexHead::Dictionary(head.0.clone()),
                types.iter().map(|ty| uni.zonk(ty)).collect(), dictionaries.iter().map(|dictionary| dictionary.index_with(uni, binder)).collect()),
            Self::Marker => (IndexHead::Marker, Vec::new(), Vec::new()),
        };
        ITerm::Inst(Box::new(IndexInstantiation { head, types, dictionaries, arguments: Vec::new() }))
    }
}

#[derive(Clone)]
struct IndexLambdaBinder {
    name: ast::Name,
    variable: String,
    carrier: Option<Ty>,
}

struct IndexSignature {
    ty: Ty,
    types: Vec<Ty>,
    dictionaries: Vec<ITerm>,
    arguments: Vec<IndexArgument>,
}

struct PendingIndexDictionary {
    parameter: ITerm,
    class: ClassApp,
    scope: Vec<(IndexDictionaryBinding, ClassApp)>,
    span: ast::Span,
}

struct PendingIndexProof {
    scheme: Scheme,
    substitutions: HashMap<String, Ty>,
    rows: HashMap<String, RowTail>,
    regions: HashMap<String, String>,
    evidence: HashMap<IndexEvidence, ITerm>,
    signature: Ty,
    arguments: Vec<ast::Type>,
    variables: Vec<String>,
    span: ast::Span,
}

struct IndexInference {
    uni: Unifier,
    dictionaries: Vec<PendingIndexDictionary>,
    proofs: Vec<PendingIndexProof>,
    transfers: Vec<DictionaryTransfer>,
    regions: Vec<String>,
}

impl IndexInference {
    /// The store is a copy: index inference runs while the checker is only
    /// borrowed, its private metavariables must never reach the checker's
    /// store, and only fully resolved results leave it.
    fn new(uni: &Unifier) -> Self { Self { uni: uni.clone(), dictionaries: Vec::new(), proofs: Vec::new(), transfers: Vec::new(), regions: Vec::new() } }
}

#[derive(Clone)]
struct InstantiatedConstraint {
    name: String,
    class: ClassApp,
    witness: ITerm,
}

/// The instance-solver's hint when a wanted class mentions an unresolved
/// type: the likeliest cause is a literal nothing has typed yet (primitive
/// types specification, section 7).
fn undetermined_hint(uni: &Unifier, wanted: &ClassApp) -> &'static str {
    if wanted.args.iter().any(|a| !uni.free_metas(a).is_empty()) {
        " (a literal's type is not determined by context; add a type annotation)"
    } else {
        ""
    }
}

fn dict_expr_ecore(d: &DictExpr) -> ECore {
    match d {
        DictExpr::Binder(l) => ECore::Var(*l),
        DictExpr::Instance(id, subs, tys) => ECore::InstDict(
            id.clone(),
            subs.iter().map(dict_expr_ecore).collect(),
            tys.clone(),
            false,
        ),
        DictExpr::Marker => ECore::Unit,
    }
}

/// A method use's site: the class (the scheme's `ctx[0]`) and the types
/// of the method's own type parameters, those its class constraint does
/// not mention (in `params` order, the order an instance's method unit
/// appends them to the instance's parameters).
fn method_site(s: &Scheme, ptys: &[Ty]) -> Option<(String, Vec<Ty>)> {
    let (_, class) = s.ctx.first()?;
    let mentioned = |p: &str| class.args.iter().any(|a| matches!(a, Ty::Param(n) if n == p));
    let own = s
        .params
        .iter()
        .zip(ptys)
        .filter(|(p, _)| !mentioned(p))
        .map(|(_, t)| t.clone())
        .collect();
    Some((class.class.clone(), own))
}

impl Checker {
    fn resolution_instances(&self) -> &[InstanceInfo] {
        self.declaration_seed.as_deref().unwrap_or(&self.instance_list)
    }

    fn resolution_class_indices(&self, class: &str) -> Option<Vec<usize>> {
        match &self.declaration_seed {
            Some(instances) => Some(instances.iter().enumerate().filter_map(|(index, instance)|
                (instance.target.class == class).then_some(index)).collect()),
            None => self.instances.get(class).cloned(),
        }
    }
    fn runtime_method_site(&self, module: &str, scheme: &Scheme, ptys: &[Ty]) -> Result<Option<(String, Vec<Ty>)>> {
        let Some((class, own)) = method_site(scheme, ptys) else { return Ok(None) };
        let signature = self.qualified_sigs.get(&(module.to_string(), class.clone()))
            .or_else(|| self.qualified_sigs.values().find(|s| s.module == module && format!("{}.{}", s.module, s.name) == class))
            .ok_or_else(|| Error::plain(format!("method has no declaring signature {module}:{class}")))?;
        Ok(Some((format!("{}.{}", signature.module, signature.name), own)))
    }
}

fn free_locals(e: &ECore, bound: &mut Vec<LocalId>, out: &mut Vec<LocalId>) {
    match e {
        ECore::LitFold { body, .. } => free_locals(body, bound, out),
        ECore::Var(l) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
        }
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(..)
        | ECore::Ctor(..) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault
        | ECore::SyntaxLit(..) | ECore::StrLit(_) => {}
        ECore::Quote { holes, .. } | ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                free_locals(h, bound, out);
            }
        }
        ECore::DictProj(d, ..) => free_locals(d, bound, out),
        ECore::Proj(d, ..) => free_locals(d, bound, out),
        ECore::InstDict(_, args, _, _) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                free_locals(a, bound, out);
            }
            if let ECore::App(f, _) = e {
                free_locals(f, bound, out);
            }
        }
        ECore::Lam { params, body } => {
            let n = bound.len();
            bound.extend(params.iter().copied());
            free_locals(body, bound, out);
            bound.truncate(n);
        }
        ECore::PolyIntro { body, .. } => free_locals(body, bound, out),
        ECore::Let(l, v, body) => {
            free_locals(v, bound, out);
            let n = bound.len();
            bound.push(*l);
            free_locals(body, bound, out);
            bound.truncate(n);
        }
        ECore::If(c, t, f) => {
            free_locals(c, bound, out);
            free_locals(&t.body, bound, out);
            free_locals(&f.body, bound, out);
        }
        ECore::Demand(d, _) => free_locals(d, bound, out),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(t, bound, out);
        }
        ECore::BoxAlloc => {}
        ECore::BoxInit(l, v, _) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(v, bound, out);
        }
        ECore::RefSet(l, v, _) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(v, bound, out);
        }
        ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => free_locals(v, bound, out),
        ECore::Case { scrut, alts } => {
            free_locals(scrut, bound, out);
            for alt in alts {
                let n = bound.len();
                match &alt.pat {
                    CPat::Var(l, _) => bound.push(*l),
                    CPat::Con { fields, .. } => bound.extend(fields.iter().copied()),
                    CPat::Wild => {}
                }
                free_locals(&alt.body, bound, out);
                bound.truncate(n);
            }
        }
    }
}

/// The family of a polytype: an anonymous single-method class named by its
/// number of type binders (design/rank-n-polymorphism.md section 2). The
/// name must not depend on anything a substitution of free outer type
/// variables can change, or a projection and the dictionary it reads would
/// disagree on the slot table.
pub(crate) fn poly_family(scheme: &Scheme) -> String {
    poly_family_of(scheme.params.len())
}

pub(crate) fn poly_family_of(k: usize) -> String {
    format!("#poly.{k}")
}

/// The anonymous instance whose one method is generalisation unit `method`
/// (`Main.f.poly3` is built by `Main.f.inst3`). Neither name is a prefix of
/// the other's artifacts, so symbol attribution stays exact.
pub(crate) fn poly_instance_id(method: &GlobalId) -> GlobalId {
    match method.0.rsplit_once(".poly") {
        Some((base, n)) => GlobalId(format!("{base}.inst{n}")),
        None => GlobalId(format!("{}.inst", method.0)),
    }
}

/// A lambda hoisted to a top level function; finalized by [`finalize_fn`].
struct HoistedFn {
    id: GlobalId,
    /// `env ++ lambda parameters`, in ABI order.
    params: Vec<LocalId>,
    body: ECore,
    /// Type parameters beyond the definition's: the skolems of every
    /// generalisation site enclosing this unit, outermost first
    /// (design/rank-n-polymorphism.md section 4).
    extra: Vec<String>,
    /// For a generalisation site's method unit: its family's binder count
    /// and how many captured locals lead its parameters.
    poly: Option<(usize, usize)>,
    /// A strict method unit's result type (a dictionary slot stores the
    /// computed value, which its result class counts).
    result: Option<Ty>,
}

/// Replace lambda nodes with `MkClos` over hoisted functions.
fn hoist_lambdas(e: &mut ECore, hint: &str, counter: &mut usize, fns: &mut Vec<HoistedFn>) {
    hoist_lambdas_in(e, hint, counter, fns, &[], &mut Vec::new())
}

/// [`hoist_lambdas`] inside a definition with type parameters `tparams`,
/// under the generalisation skolems `scope`. A generalisation site becomes
/// the method unit of an anonymous instance of its family, built by an
/// `InstDict` over its captured locals at the enclosing unit's identity
/// vector (design/rank-n-polymorphism.md section 4).
fn hoist_lambdas_in(e: &mut ECore, hint: &str, counter: &mut usize, fns: &mut Vec<HoistedFn>, tparams: &[String], scope: &mut Vec<String>) {
    match e {
        ECore::LitFold { body, .. } => hoist_lambdas_in(body, hint, counter, fns, tparams, scope),
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_) | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::SyntaxLit(..)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                hoist_lambdas_in(h, hint, counter, fns, tparams, scope);
            }
            // The retained body is reified program text; its lambdas
            // hoist like any others (supercompilation section 7).
            hoist_lambdas_in(body, hint, counter, fns, tparams, scope);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                hoist_lambdas_in(h, hint, counter, fns, tparams, scope);
            }
        }
        ECore::DictProj(d, ..) => hoist_lambdas_in(d, hint, counter, fns, tparams, scope),
        ECore::Proj(d, ..) => hoist_lambdas_in(d, hint, counter, fns, tparams, scope),
        ECore::InstDict(_, args, _, _) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                hoist_lambdas_in(a, hint, counter, fns, tparams, scope);
            }
            if let ECore::App(f, _) = e {
                hoist_lambdas_in(f, hint, counter, fns, tparams, scope);
            }
        }
        ECore::Let(_, v, body) => {
            hoist_lambdas_in(v, hint, counter, fns, tparams, scope);
            hoist_lambdas_in(body, hint, counter, fns, tparams, scope);
        }
        ECore::If(c, t, f) => {
            hoist_lambdas_in(c, hint, counter, fns, tparams, scope);
            hoist_lambdas_in(&mut t.body, hint, counter, fns, tparams, scope);
            hoist_lambdas_in(&mut f.body, hint, counter, fns, tparams, scope);
        }
        ECore::Demand(d, _) => hoist_lambdas_in(d, hint, counter, fns, tparams, scope),
        ECore::LazyAlloc => {}
        ECore::LazyInit(_, t) => hoist_lambdas_in(t, hint, counter, fns, tparams, scope),
        ECore::BoxAlloc => {}
        ECore::BoxInit(_, v, _) => hoist_lambdas_in(v, hint, counter, fns, tparams, scope),
        ECore::RefSet(_, v, _) => hoist_lambdas_in(v, hint, counter, fns, tparams, scope),
        ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => hoist_lambdas_in(v, hint, counter, fns, tparams, scope),
        ECore::Case { scrut, alts } => {
            hoist_lambdas_in(scrut, hint, counter, fns, tparams, scope);
            for alt in alts {
                hoist_lambdas_in(&mut alt.body, hint, counter, fns, tparams, scope);
            }
        }
        ECore::Lam { params, body } => {
            hoist_lambdas_in(body, hint, counter, fns, tparams, scope);
            let mut free = Vec::new();
            free_locals(body, &mut Vec::new(), &mut free);
            let mut env: Vec<LocalId> =
                free.into_iter().filter(|l| !params.contains(l)).collect();
            env.sort_by_key(|l| l.0);
            let mut fn_params = env.clone();
            fn_params.extend(params.iter().copied());
            let id = GlobalId(format!("{hint}.lam{}", *counter));
            *counter += 1;
            fns.push(HoistedFn {
                id: id.clone(),
                params: fn_params,
                body: (**body).clone(),
                extra: scope.clone(),
                poly: None,
                result: None,
            });
            *e = ECore::MkClos {
                fn_id: id,
                env: env.into_iter().map(ECore::Var).collect(),
            };
        }
        ECore::PolyIntro { skolems, body, ty } => {
            // The instance's vector is the enclosing unit's identity.
            let enclosing: Vec<Ty> = tparams.iter().chain(scope.iter()).map(|n| Ty::Param(n.clone().into())).collect();
            let mark = scope.len();
            scope.extend(skolems.iter().cloned());
            // A lambda body is the method itself, so a slot holds the
            // method's partial application rather than a computed closure.
            let (lam_params, mut inner) = match std::mem::replace(&mut **body, ECore::Unit) {
                ECore::Lam { params, body } if !params.is_empty() => (params, *body),
                other => (Vec::new(), other),
            };
            let result = lam_params.is_empty().then(|| ty.clone());
            hoist_lambdas_in(&mut inner, hint, counter, fns, tparams, scope);
            let extra = scope.clone();
            scope.truncate(mark);
            let mut free = Vec::new();
            free_locals(&inner, &mut Vec::new(), &mut free);
            let mut env: Vec<LocalId> = free.into_iter().filter(|l| !lam_params.contains(l)).collect();
            env.sort_by_key(|l| l.0);
            env.dedup();
            let mut fn_params = env.clone();
            fn_params.extend(lam_params.iter().copied());
            let n = *counter;
            *counter += 1;
            let id = GlobalId(format!("{hint}.poly{n}"));
            fns.push(HoistedFn {
                id: id.clone(),
                params: fn_params,
                body: inner,
                extra,
                poly: Some((skolems.len(), env.len())),
                result,
            });
            *e = ECore::InstDict(
                poly_instance_id(&id),
                env.into_iter().map(ECore::Var).collect(),
                enclosing,
                true,
            );
        }
    }
}

/// Renumber the locals of a function body so that its parameters are
/// `LocalId(0..arity)` in ABI order and every other local follows in
/// traversal order. Codegen relies on this convention.
fn renumber_fn(body: &ECore, params: &[LocalId]) -> (ECore, HashMap<LocalId, LocalId>) {
    let mut map: HashMap<LocalId, LocalId> = HashMap::new();
    for (i, p) in params.iter().enumerate() {
        map.insert(*p, LocalId(i));
    }
    let mut next = params.len();
    assign_locals(body, &mut map, &mut next);
    (rewrite_locals(body, &map), map)
}

fn note_local(l: LocalId, map: &mut HashMap<LocalId, LocalId>, next: &mut usize) {
    map.entry(l).or_insert_with(|| {
        let id = LocalId(*next);
        *next += 1;
        id
    });
}

fn assign_locals(e: &ECore, map: &mut HashMap<LocalId, LocalId>, next: &mut usize) {
    match e {
        ECore::LitFold { body, .. } => assign_locals(body, map, next),
        ECore::Var(l) => note_local(*l, map, next),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(..)
        | ECore::Ctor(..) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault
        | ECore::SyntaxLit(..) | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                assign_locals(h, map, next);
            }
            assign_locals(body, map, next);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                assign_locals(h, map, next);
            }
        }
        ECore::DictProj(d, ..) => assign_locals(d, map, next),
        ECore::Proj(d, ..) => assign_locals(d, map, next),
        ECore::InstDict(_, args, _, _) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
            if let ECore::App(f, _) = e {
                assign_locals(f, map, next);
            }
            for a in args {
                assign_locals(a, map, next);
            }
        }
        ECore::Lam { params, body } => {
            for p in params {
                note_local(*p, map, next);
            }
            assign_locals(body, map, next);
        }
        ECore::PolyIntro { body, .. } => assign_locals(body, map, next),
        ECore::Let(l, v, body) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
            assign_locals(body, map, next);
        }
        ECore::If(c, t, f) => {
            assign_locals(c, map, next);
            assign_locals(&t.body, map, next);
            assign_locals(&f.body, map, next);
        }
        ECore::Demand(d, _) => assign_locals(d, map, next),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            note_local(*l, map, next);
            assign_locals(t, map, next);
        }
        ECore::BoxAlloc => {}
        ECore::BoxInit(l, v, _) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
        }
        ECore::RefSet(l, v, _) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
        }
        ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => assign_locals(v, map, next),
        ECore::Case { scrut, alts } => {
            assign_locals(scrut, map, next);
            for alt in alts {
                match &alt.pat {
                    CPat::Var(l, _) => note_local(*l, map, next),
                    CPat::Con { fields, .. } => {
                        for f in fields {
                            note_local(*f, map, next);
                        }
                    }
                    CPat::Wild => {}
                }
                assign_locals(&alt.body, map, next);
            }
        }
    }
}

fn rewrite_locals(e: &ECore, map: &HashMap<LocalId, LocalId>) -> ECore {
    remap_ecore(e, &Remap { locals: map, units: None, tys: None, deferred: None, substitutions: std::cell::OnceCell::new() })
}

/// A renaming of an elaborated body: its locals, and optionally its
/// `letrec` unit ids and the types it records (a let-generalized local's
/// copy, [`Checker::gen_copy`]).
struct Remap<'a> {
    locals: &'a HashMap<LocalId, LocalId>,
    units: Option<&'a HashMap<GlobalId, GlobalId>>,
    tys: Option<&'a dyn Fn(&Ty) -> Ty>,
    deferred: Option<&'a CopyDeferred>,
    /// `locals` as an evidence substitution and a local-value substitution,
    /// built once on the first `ty` without `tys` (a body has many types and
    /// `locals` covers every local of it).
    substitutions: std::cell::OnceCell<(HashMap<IndexEvidence, ITerm>, HashMap<LocalId, ITerm>)>,
}

impl Remap<'_> {
    fn ty(&self, t: &Ty) -> Ty {
        match self.tys {
            Some(f) => f(t),
            None => {
                let (evidence, local_values) = self.substitutions.get_or_init(|| (self.evidence(), self.local_values()));
                t.subst_evidence(evidence).subst_local_values(local_values)
            }
        }
    }

    fn evidence(&self) -> HashMap<IndexEvidence, ITerm> {
        self.locals.iter().map(|(old, new)|
            (IndexEvidence::Local(old.0), ITerm::Evidence(IndexEvidence::Local(new.0)))).collect()
    }

    fn index(&self, term: &ITerm) -> ITerm {
        let Ty::Idx(mapped) = self.ty(&Ty::Idx(Box::new(term.clone()))) else { unreachable!() };
        *mapped
    }

    fn local_values(&self) -> HashMap<LocalId, ITerm> {
        self.locals.iter().map(|(old, new)| (*old, ITerm::LocalValue(*new))).collect()
    }

    fn proposition(&self, proposition: &PTerm) -> PTerm {
        let Ty::Prp(mapped) = self.ty(&Ty::Prp(Box::new(proposition.clone()))) else { unreachable!() };
        *mapped
    }

    fn local(&self, local: LocalId) -> LocalId {
        self.locals.get(&local).copied().unwrap_or(local)
    }

    fn class(&self, class: &ClassApp) -> ClassApp {
        ClassApp { class: class.class.clone(), args: self.tys(&class.args) }
    }

    fn scheme(&self, scheme: &Scheme) -> Scheme {
        let mut scheme = scheme.clone();
        scheme.ty = self.ty(&scheme.ty);
        for parameter in &mut scheme.idx_params {
            if let IxParam::Val(_, carrier) = parameter { *carrier = self.ty(carrier); }
        }
        for (_, carrier) in &mut scheme.val_params { *carrier = self.ty(carrier); }
        for (_, class) in &mut scheme.ctx { class.args = self.tys(&class.args); }
        for obligation in &mut scheme.obligs {
            let (PropOblig::Pred(predicate) | PropOblig::Preserves { pred: predicate, .. }) = obligation;
            let Ty::Prp(mapped) = self.ty(&Ty::Prp(Box::new(predicate.clone()))) else { unreachable!() };
            *predicate = *mapped;
        }
        for parameter in &mut scheme.evidence_params {
            if let IndexEvidence::Local(local) = parameter {
                *local = self.locals.get(&LocalId(*local)).map_or(*local, |new| new.0);
            }
        }
        scheme
    }

    fn annotation(&self, annotation: &mut ast::Type) {
        match annotation {
            ast::Type::Elaborated(scheme, _) => **scheme = self.scheme(scheme),
            ast::Type::IndexOperand(term, carrier, _) => {
                let Ty::Idx(mapped) = self.ty(&Ty::Idx(term.clone())) else { unreachable!() };
                *term = mapped;
                **carrier = self.ty(carrier);
            }
            _ => {}
        }
    }

    fn tys(&self, ts: &[Ty]) -> Vec<Ty> {
        ts.iter().map(|t| self.ty(t)).collect()
    }

    fn unit(&self, g: &GlobalId) -> GlobalId {
        self.units.and_then(|u| u.get(g)).unwrap_or(g).clone()
    }
}

fn remap_ecore(e: &ECore, r: &Remap) -> ECore {
    let map = r.locals;
    let m = |l: &LocalId| *map.get(l).unwrap_or(l);
    match e {
        ECore::LitFold { idx, body } => ECore::LitFold {
            idx: *idx,
            body: Box::new(remap_ecore(body, r)),
        },
        ECore::Var(l) => ECore::Var(m(l)),
        ECore::Int(v) => ECore::Int(*v),
        ECore::ConstBig { nat, digits } => ECore::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::ConstBytes(b) => ECore::ConstBytes(b.clone()),
        ECore::ConstString { ctor, bytes } => ECore::ConstString {
            ctor: ctor.clone(),
            bytes: bytes.clone(),
        },
        ECore::ConstSym(s) => ECore::ConstSym(s.clone()),
        ECore::Lit(i) => ECore::Lit(*i),
        ECore::Bool(b) => ECore::Bool(*b),
        ECore::Unit => ECore::Unit,
        ECore::Global(g, t) => ECore::Global(r.unit(g), r.tys(t)),
        ECore::Ctor(g, t) => ECore::Ctor(g.clone(), r.tys(t)),
        ECore::Prim(p) => ECore::Prim(*p),
        ECore::Foreign(n) => ECore::Foreign(n.clone()),
        ECore::Hole(h) => ECore::Hole(r.deferred.and_then(|copy| copy.wants.get(h)).copied().unwrap_or(*h)),
        ECore::QuoteHole(h) => ECore::QuoteHole(*h),
        ECore::IdxPending(i) => ECore::IdxPending(r.deferred.and_then(|copy| copy.indices.get(i)).copied().unwrap_or(*i)),
        ECore::Fault => ECore::Fault,
        ECore::SyntaxLit(toks, scope) => ECore::SyntaxLit(toks.clone(), scope.clone()),
        ECore::StrLit(s) => ECore::StrLit(s.clone()),
        ECore::Quote { template, holes, body } => ECore::Quote {
            template: {
                let mut template = template.clone();
                template.visit_types_mut(&mut |annotation| r.annotation(annotation));
                template
            },
            holes: holes.iter().map(|h| remap_ecore(h, r)).collect(),
            body: Box::new(remap_ecore(body, r)),
        },
        ECore::DeclsQuote { template, holes, name_holes } => ECore::DeclsQuote {
            template: template.iter().map(|declaration| {
                let mut declaration = declaration.clone();
                declaration.visit_types_mut(&mut |annotation| r.annotation(annotation));
                declaration
            }).collect(),
            holes: holes.iter().map(|h| remap_ecore(h, r)).collect(),
            name_holes: name_holes.clone(),
        },
        ECore::DictProj(d, i, s, t) => {
            ECore::DictProj(
                Box::new(remap_ecore(d, r)),
                *i,
                s.as_ref().map(|(c, own)| (c.clone(), r.tys(own))),
                r.ty(t),
            )
        }
        ECore::Proj(d, i, t) => ECore::Proj(Box::new(remap_ecore(d, r)), i.clone(), r.ty(t)),
        ECore::InstDict(g, args, t, explicit) => ECore::InstDict(
            g.clone(),
            args.iter().map(|a| remap_ecore(a, r)).collect(),
            r.tys(t),
            *explicit,
        ),
        ECore::App(f, args) => ECore::App(
            Box::new(remap_ecore(f, r)),
            args.iter().map(|a| remap_ecore(a, r)).collect(),
        ),
        ECore::Lam { params, body } => ECore::Lam {
            params: params.iter().map(m).collect(),
            body: Box::new(remap_ecore(body, r)),
        },
        ECore::PolyIntro { skolems, body, ty } => ECore::PolyIntro { skolems: skolems.clone(), body: Box::new(remap_ecore(body, r)), ty: ty.clone() },
        ECore::MkClos { fn_id, env } => ECore::MkClos {
            fn_id: r.unit(fn_id),
            env: env.iter().map(|a| remap_ecore(a, r)).collect(),
        },
        ECore::Let(l, v, body) => ECore::Let(
            m(l),
            Box::new(remap_ecore(v, r)),
            Box::new(remap_ecore(body, r)),
        ),
        ECore::If(c, t, f) => ECore::If(
            Box::new(remap_ecore(c, r)),
            CBranch {
                weight: t.weight,
                body: Box::new(remap_ecore(&t.body, r)),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(remap_ecore(&f.body, r)),
            },
        ),
        ECore::Demand(d, t) => ECore::Demand(Box::new(remap_ecore(d, r)), r.ty(t)),
        ECore::LazyAlloc => ECore::LazyAlloc,
        ECore::LazyInit(l, t) => {
            ECore::LazyInit(m(l), Box::new(remap_ecore(t, r)))
        }
        ECore::BoxAlloc => ECore::BoxAlloc,
        ECore::BoxInit(l, v, t) => ECore::BoxInit(m(l), Box::new(remap_ecore(v, r)), r.ty(t)),
        ECore::RefSet(l, v, t) => ECore::RefSet(m(l), Box::new(remap_ecore(v, r)), r.ty(t)),
        ECore::BoxGet(v, t) => ECore::BoxGet(Box::new(remap_ecore(v, r)), r.ty(t)),
        ECore::BoxClear(v, t) => ECore::BoxClear(Box::new(remap_ecore(v, r)), r.ty(t)),
        ECore::Case { scrut, alts } => ECore::Case {
            scrut: Box::new(remap_ecore(scrut, r)),
            alts: alts
                .iter()
                .map(|a| {
                    let p = match &a.pat {
                        CPat::Var(l, c) => CPat::Var(m(l), *c),
                        CPat::Con { tag, fields, classes } => CPat::Con {
                            tag: *tag,
                            fields: fields.iter().map(m).collect(),
                            classes: classes.clone(),
                        },
                        CPat::Wild => CPat::Wild,
                    };
                    CAlt {
                        pat: p,
                        weight: a.weight,
                        body: Box::new(remap_ecore(&a.body, r)),
                    }
                })
                .collect(),
        },
    }
}

/// The binders an elaborated body introduces (`let` binders, lambda
/// parameters, `case` pattern binders), and the units it names (its
/// globals and closures; the `letrec` units among them are copied with a
/// let-generalized local's definition).
#[derive(Default)]
struct CopyParts {
    binders: Vec<LocalId>,
    units: Vec<GlobalId>,
    wants: std::collections::BTreeSet<usize>,
    indices: std::collections::BTreeSet<usize>,
}

#[derive(Default)]
struct CopyDeferred {
    wants: HashMap<usize, usize>,
    indices: HashMap<usize, usize>,
}

fn def_parts(e: &ECore, parts: &mut CopyParts) {
    match e {
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Var(_) | ECore::Ctor(..) | ECore::Prim(_)
        | ECore::Foreign(_) | ECore::QuoteHole(_) | ECore::Fault
        | ECore::LazyAlloc | ECore::BoxAlloc | ECore::SyntaxLit(..) | ECore::StrLit(_) => {}
        ECore::Hole(want) => { parts.wants.insert(*want); }
        ECore::IdxPending(index) => { parts.indices.insert(*index); }
        ECore::Global(g, _) => parts.units.push(g.clone()),
        ECore::LitFold { body, .. } => def_parts(body, parts),
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                def_parts(h, parts);
            }
            def_parts(body, parts);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                def_parts(h, parts);
            }
        }
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) | ECore::BoxGet(d, _) | ECore::BoxClear(d, _) => def_parts(d, parts),
        ECore::LazyInit(_, v) | ECore::BoxInit(_, v, _) | ECore::RefSet(_, v, _) => def_parts(v, parts),
        ECore::InstDict(_, args, _, _) => {
            for a in args {
                def_parts(a, parts);
            }
        }
        ECore::App(f, args) => {
            def_parts(f, parts);
            for a in args {
                def_parts(a, parts);
            }
        }
        ECore::MkClos { fn_id, env } => {
            parts.units.push(fn_id.clone());
            for a in env {
                def_parts(a, parts);
            }
        }
        ECore::Lam { params, body } => {
            parts.binders.extend(params.iter().copied());
            def_parts(body, parts);
        }
        ECore::PolyIntro { body, .. } => def_parts(body, parts),
        ECore::Let(l, v, body) => {
            parts.binders.push(*l);
            def_parts(v, parts);
            def_parts(body, parts);
        }
        ECore::If(c, t, f) => {
            def_parts(c, parts);
            def_parts(&t.body, parts);
            def_parts(&f.body, parts);
        }
        ECore::Case { scrut, alts } => {
            def_parts(scrut, parts);
            for a in alts {
                match &a.pat {
                    CPat::Var(l, _) => parts.binders.push(*l),
                    CPat::Con { fields, .. } => parts.binders.extend(fields.iter().copied()),
                    CPat::Wild => {}
                }
                def_parts(&a.body, parts);
            }
        }
    }
}

/// Replace deferred literal nodes by their resolved constants.
fn resolve_lit_words(e: &mut ECore, lits: &[ECore]) {
    match e {
        ECore::LitFold { body, .. } => resolve_lit_words(body, lits),
        ECore::Lit(i) => *e = lits[*i].clone(),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Bool(_) | ECore::Unit | ECore::Var(_)
        | ECore::Global(..)
        | ECore::Ctor(..) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault | ECore::LazyAlloc | ECore::BoxAlloc
        | ECore::SyntaxLit(..) | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                resolve_lit_words(h, lits);
            }
            resolve_lit_words(body, lits);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                resolve_lit_words(h, lits);
            }
        }
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) => {
            resolve_lit_words(d, lits)
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                resolve_lit_words(a, lits);
            }
        }
        ECore::App(f, args) => {
            resolve_lit_words(f, lits);
            for a in args {
                resolve_lit_words(a, lits);
            }
        }
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => resolve_lit_words(body, lits),
        ECore::Let(_, v, body) => {
            resolve_lit_words(v, lits);
            resolve_lit_words(body, lits);
        }
        ECore::If(c, t, f) => {
            resolve_lit_words(c, lits);
            resolve_lit_words(&mut t.body, lits);
            resolve_lit_words(&mut f.body, lits);
        }
        ECore::LazyInit(_, t) => resolve_lit_words(t, lits),
        ECore::BoxInit(_, v, _) => resolve_lit_words(v, lits),
        ECore::RefSet(_, v, _) => resolve_lit_words(v, lits),
        ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => resolve_lit_words(v, lits),
        ECore::Case { scrut, alts } => {
            resolve_lit_words(scrut, lits);
            for alt in alts {
                resolve_lit_words(&mut alt.body, lits);
            }
        }
    }
}

/// Replace resolved implicit index arguments (parallel to
/// [`resolve_lit_words`]).
fn resolve_idx_words(e: &mut ECore, vals: &[ECore]) {
    match e {
        ECore::LitFold { body, .. } => resolve_idx_words(body, vals),
        ECore::IdxPending(i) => *e = vals[*i].clone(),
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_) | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(..)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                resolve_idx_words(h, vals);
            }
            resolve_idx_words(body, vals);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                resolve_idx_words(h, vals);
            }
        }
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) => {
            resolve_idx_words(d, vals)
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                resolve_idx_words(a, vals);
            }
        }
        ECore::App(f, args) => {
            resolve_idx_words(f, vals);
            for a in args {
                resolve_idx_words(a, vals);
            }
        }
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => resolve_idx_words(body, vals),
        ECore::Let(_, v, body) => {
            resolve_idx_words(v, vals);
            resolve_idx_words(body, vals);
        }
        ECore::If(c, t, f) => {
            resolve_idx_words(c, vals);
            resolve_idx_words(&mut t.body, vals);
            resolve_idx_words(&mut f.body, vals);
        }
        ECore::LazyInit(_, t) => resolve_idx_words(t, vals),
        ECore::BoxInit(_, v, _) => resolve_idx_words(v, vals),
        ECore::RefSet(_, v, _) => resolve_idx_words(v, vals),
        ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => resolve_idx_words(v, vals),
        ECore::Case { scrut, alts } => {
            resolve_idx_words(scrut, vals);
            for alt in alts {
                resolve_idx_words(&mut alt.body, vals);
            }
        }
    }
}

/// A compile-time value reified to constant code (the P14 constant
/// emission forms; surface grammar local amendments section 10). Only
/// first-order constants have a constant form: closures, quotations,
/// compile-time-only values, and stage-synthesized structures do not
/// reify, and their constructions keep their runtime class calls. An
/// immutable `String` value (the `string_ctor` representation) reifies
/// to the shared constant: its content is its identity, so a computed
/// string folds to the same constant form a literal takes.
fn reify_value(v: &crate::core::Value, string_ctor: &str) -> Option<ECore> {
    use crate::core::Value;
    Some(match v {
        Value::Word(w) => ECore::Int(*w),
        Value::Big { nat, digits } => ECore::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        Value::Bytes(b) => ECore::ConstBytes(b.borrow().clone()),
        Value::Sym(s) => ECore::ConstSym(s.clone()),
        Value::Bool(b) => ECore::Bool(*b),
        Value::Unit => ECore::Unit,
        Value::Adt { ctor, fields, .. } => {
            if ctor.is_empty() {
                return None;
            }
            if ctor == string_ctor {
                return match fields.as_slice() {
                    [Value::Bytes(b)] => Some(ECore::ConstString {
                        ctor: GlobalId(ctor.clone()),
                        bytes: b.borrow().clone(),
                    }),
                    _ => None,
                };
            }
            // A compile-time value carries no types: its constructor's
            // instantiation is unrecorded.
            let mut core = ECore::Ctor(GlobalId(ctor.clone()), Vec::new());
            for f in fields {
                core = app_spine(core, vec![reify_value(f, string_ctor)?]);
            }
            core
        }
        _ => return None,
    })
}

/// Whether `e` contains a node the compile-time evaluator cannot run
/// (foreign code, lazy and fixpoint machinery, the fault path, quotation
/// machinery, compile-time-only values, or an unresolved placeholder): a
/// collection literal around one keeps its runtime class call.
fn fold_blocked(e: &ECore) -> bool {
    match e {
        ECore::Int(_)
        | ECore::ConstBig { .. }
        | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_) => false,
        ECore::Lit(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::SyntaxLit(..)
        | ECore::StrLit(_)
        | ECore::Demand(..)
        | ECore::LazyAlloc
        | ECore::LazyInit(_, _)
        | ECore::BoxAlloc
        | ECore::BoxInit(..)
        | ECore::RefSet(_, _, _)
        | ECore::BoxGet(..) | ECore::BoxClear(..)
        | ECore::Quote { .. }
        | ECore::DeclsQuote { .. } => true,
        ECore::LitFold { body, .. } => fold_blocked(body),
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) => fold_blocked(d),
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            args.iter().any(fold_blocked)
        }
        ECore::App(f, args) => fold_blocked(f) || args.iter().any(fold_blocked),
        // Folding runs before lambda hoisting. Even a closed callback
        // cannot be lowered yet, and closures have no constant reification.
        // Keep the ordinary runtime construction instead of trying to
        // lower an unhoisted lambda while folding its enclosing list.
        ECore::Lam { .. } | ECore::PolyIntro { .. } => true,
        ECore::Let(_, v, body) => fold_blocked(v) || fold_blocked(body),
        ECore::If(c, t, f) => {
            fold_blocked(c) || fold_blocked(&t.body) || fold_blocked(&f.body)
        }
        ECore::Case { scrut, alts } => {
            fold_blocked(scrut) || alts.iter().any(|a| fold_blocked(&a.body))
        }
    }
}

/// What lowering needs to attach representation classes: the checker's
/// binder types (keyed by the ids before renumbering), the definition's
/// type parameters, and its own hoisted and `letrec` units.
pub(crate) struct LowerCx<'a> {
    ck: &'a Checker,
    tparams: &'a [String],
    own: &'a HashSet<GlobalId>,
    /// Renumbered id to checker id.
    orig: HashMap<LocalId, LocalId>,
    /// Classes of untyped `let` binders, from their definitions (keyed by
    /// the lowered body's ids), so later uses of them resolve too.
    inferred: std::cell::RefCell<HashMap<LocalId, crate::repr::Class>>,
    /// The unit being lowered, for diagnostics.
    unit: Option<GlobalId>,
    /// Whether the lowered code runs. A word whose class is unknown would
    /// be miscompiled (a counted word read as `Raw` is consumed without
    /// its references being counted), so in runtime code it is a
    /// compiler bug; compile-time-only code never reads classes.
    runtime: bool,
    /// The compiler bugs found while lowering.
    bugs: std::cell::RefCell<Vec<String>>,
}

impl<'a> LowerCx<'a> {
    pub(crate) fn new(ck: &'a Checker, tparams: &'a [String], own: &'a HashSet<GlobalId>) -> Self {
        LowerCx {
            ck,
            tparams,
            own,
            orig: HashMap::new(),
            inferred: Default::default(),
            unit: None,
            runtime: true,
            bugs: Default::default(),
        }
    }

    /// A context for code that only runs at compile time (a fold, a
    /// probe): its classes are never read.
    pub(crate) fn compile_time(ck: &'a Checker, tparams: &'a [String], own: &'a HashSet<GlobalId>) -> Self {
        LowerCx { runtime: false, ..LowerCx::new(ck, tparams, own) }
    }

    /// Name the unit the next lowering belongs to.
    pub(crate) fn for_unit(mut self, unit: GlobalId) -> Self {
        self.unit = Some(unit);
        self
    }

    /// What lowering refused (compiler bugs, and representation
    /// conflicts it cannot compile), as one error.
    pub(crate) fn finish(&self) -> Result<()> {
        let bugs = self.bugs.borrow();
        match bugs.first() {
            None => Ok(()),
            Some(first) => Err(Error::plain(format!(
                "{first}{}",
                if bugs.len() > 1 { format!(" (and {} more)", bugs.len() - 1) } else { String::new() }
            ))),
        }
    }

    fn bug(&self, msg: String) {
        self.refuse(format!("compiler bug: {msg}"));
    }

    fn refuse(&self, msg: String) {
        if self.runtime {
            self.bugs.borrow_mut().push(msg);
        }
    }

    fn unit_name(&self) -> String {
        self.unit.as_ref().map_or_else(|| "a closed expression".to_string(), |u| u.0.clone())
    }

    /// The class of a type at lowering. A metavariable still unbound here
    /// is a type nothing constrains, so no word of it is ever produced at
    /// run time (the element type of an empty literal, say): `Raw`. A
    /// let-generalized local's quantified metavariables are no exception:
    /// each use runs its own copy of the definition, typed at that use's
    /// instantiation ([`Checker::gen_copy`]), and the definition itself is
    /// never run at them.
    fn class(&self, ty: &Ty) -> crate::repr::Class {
        use crate::repr::Class;
        let z = self.ck.uni.zonk(ty);
        match &z {
            Ty::Meta(_) => return Class::Raw,
            Ty::Ref(_, a) => return self.class(a),
            _ => {}
        }
        crate::repr::repr_of(&z, self.tparams, self.ck, &mut |t| match t {
            Ty::Meta(_) => {}
            _ => self.bug(crate::diagnostic!(
                "no representation class for {} in {} (type parameters {})",
                t,
                self.unit_name(),
                format!("{:?}", self.tparams)
            )),
        })
    }

    fn classes(&self, tys: &[Ty]) -> Vec<crate::repr::Class> {
        tys.iter().map(|t| self.class(t)).collect()
    }

    /// A binder's recorded class, if the checker recorded its type or
    /// class.
    fn known(&self, l: LocalId) -> Option<crate::repr::Class> {
        if let Some(c) = self.inferred.borrow().get(&l) {
            return Some(*c);
        }
        let l = self.orig.get(&l).copied().unwrap_or(l);
        if let Some(c) = self.ck.local_cls.get(&l) {
            return Some(*c);
        }
        self.ck.local_tys.get(&l).map(|t| self.class(t))
    }

    /// The class of a binder of the lowered body.
    fn local(&self, l: LocalId) -> crate::repr::Class {
        self.known(l).unwrap_or_else(|| self.missing(l))
    }

    /// A binder the elaboration created without recording its type: in
    /// runtime code, a compiler bug (its words could not be counted).
    fn missing(&self, l: LocalId) -> crate::repr::Class {
        let l = self.orig.get(&l).copied().unwrap_or(l);
        let name = match self.ck.obligations.get(&l) {
            Some(o) if o.name != "_" => format!("`{}` (v{})", o.name, l.0),
            _ => format!("v{}", l.0),
        };
        let site = match self.ck.local_origin.get(&l) {
            Some(loc) => format!(", created at {}:{}", loc.file(), loc.line()),
            None => String::new(),
        };
        self.bug(format!("binder {name} of {} has no recorded type{site}", self.unit_name()));
        crate::repr::Class::Raw
    }

    /// The class of a `let` binder: its recorded type's, or for a binder
    /// the elaboration introduced without one (match compilation's
    /// scrutinee temporaries, desugaring's intermediates), the class of
    /// the word it is bound to.
    fn let_class(&self, l: LocalId, v: &ECore) -> crate::repr::Class {
        self.known(l).or_else(|| self.expr_class(v)).unwrap_or_else(|| self.missing(l))
    }

    /// The class of an elaborated expression's word, where its shape
    /// shows it.
    fn expr_class(&self, e: &ECore) -> Option<crate::repr::Class> {
        use crate::repr::Class;
        match e {
            ECore::Var(x) => self.known(*x),
            ECore::Let(x, v, b) => match &**b {
                ECore::Var(y) if y == x => Some(self.let_class(*x, v)),
                _ => self.expr_class(b),
            },
            ECore::LitFold { body, .. } => self.expr_class(body),
            ECore::Proj(_, _, t) | ECore::DictProj(.., t) | ECore::Demand(_, t) | ECore::BoxGet(_, t)
            | ECore::BoxInit(_, _, t) => Some(self.class(t)),
            ECore::ConstBig { .. } => Some(Class::TObj),
            ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::MkClos { .. } | ECore::Lam { .. }
            | ECore::InstDict(..) | ECore::LazyAlloc | ECore::BoxAlloc | ECore::Quote { .. } => Some(Class::Obj),
            ECore::Int(_) | ECore::Bool(_) | ECore::Unit | ECore::ConstSym(_) | ECore::LazyInit(..)
            | ECore::RefSet(..) | ECore::BoxClear(..) => Some(Class::Raw),
            ECore::App(f, _) if matches!(**f, ECore::Ctor(..)) => Some(Class::Obj),
            ECore::If(_, t, f) => self.expr_class(&t.body).or_else(|| self.expr_class(&f.body)),
            ECore::Case { alts, .. } => alts.iter().find_map(|a| self.expr_class(&a.body)),
            _ => None,
        }
    }
}

impl crate::repr::TypeReprs for Checker {
    fn adt(&self, name: &str, args: &[Ty]) -> crate::repr::AdtRepr {
        let kind = match self.known_type(name) {
            Some(k) => k,
            _ => return crate::repr::AdtRepr::Obj,
        };
        if let Some(r) = kind.refine.as_ref() {
            let mut map: HashMap<String, Ty> = HashMap::new();
            for (p, a) in r.params.iter().zip(args) {
                map.insert(p.clone(), a.clone());
            }
            return crate::repr::AdtRepr::Base(Checker::subst_ty(&r.base, &map));
        }
        // A foreign opaque type's values come only from foreign calls:
        // they are handles the foreign side owns.
        if kind.opaque {
            return crate::repr::AdtRepr::Raw;
        }
        crate::repr::AdtRepr::Obj
    }
}

fn finalize_fn(
    id: GlobalId,
    params: Vec<LocalId>,
    body: ECore,
    prim: Option<PrimOp>,
    ctor: Option<(usize, usize)>,
    hints: crate::core::FnHints,
    meta: crate::core::FnMeta,
    cx: &mut LowerCx,
) -> Result<FnDef> {
    cx.orig.clear();
    cx.inferred.borrow_mut().clear();
    cx.unit = Some(id.clone());
    let param_classes = params.iter().map(|l| cx.local(*l)).collect();
    let (body, map) = renumber_fn(&body, &params);
    cx.orig = map.iter().map(|(o, n)| (*n, *o)).collect();
    let body = lower(&body, cx);
    cx.finish()?;
    Ok(FnDef {
        id,
        params: params.iter().map(|l| format!("v{}", l.0)).collect(),
        body,
        prim,
        ctor,
        hints,
        meta,
        repr: crate::core::UnitRepr {
            tparams: cx.tparams.len(),
            params: param_classes,
            fields: Vec::new(),
            ..Default::default()
        },
    })
}

/// Lower an elaborated body to [`Core`], attaching the class of every
/// binder and the class vector of every instantiation.
fn lower(e: &ECore, cx: &LowerCx) -> crate::core::Core {
    use crate::core::Core as C;
    let low = |x: &ECore| lower(x, cx);
    match e {
        ECore::Int(v) => C::Int(*v),
        ECore::ConstBig { nat, digits } => C::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::ConstBytes(b) => C::ConstBytes(b.clone()),
        ECore::ConstString { ctor, bytes } => C::ConstString {
            ctor: ctor.clone(),
            bytes: bytes.clone(),
        },
        ECore::ConstSym(s) => C::ConstSym(s.clone()),
        ECore::LitFold { body, .. } => low(body),
        ECore::Lit(_) => {
            unreachable!("deferred literals are resolved before lowering")
        }
        ECore::Bool(b) => C::Bool(*b),
        ECore::Unit => C::Unit,
        ECore::Var(l) => C::Var(*l),
        ECore::Global(g, tys) => {
            let v = if !tys.is_empty() {
                cx.classes(tys)
            } else if cx.own.contains(g) {
                crate::repr::identity(cx.tparams.len())
            } else {
                Vec::new()
            };
            C::Global(g.clone(), v)
        }
        ECore::Ctor(g, tys) => C::Ctor(g.clone(), cx.classes(tys)),
        ECore::Prim(p) => C::Prim(*p),
        ECore::Foreign(n) => C::Foreign(n.clone()),
        ECore::DictProj(d, i, site, ty) => C::DictProj(
            Box::new(low(d)),
            *i,
            site.as_ref().map(|(class, own)| {
                Box::new(crate::core::MethodSite { class: class.clone(), own: cx.classes(own) })
            }),
            cx.class(ty),
        ),
        ECore::Proj(d, f, ty) => {
            C::Proj(Box::new(low(d)), crate::core::ProjField { class: cx.class(ty), ..f.clone() })
        }
        ECore::InstDict(g, args, tys, explicit) => {
            let own = cx.ck.self_instance.and_then(|index| cx.ck.instance_list.get(index));
            let is_self = cx.runtime && !*explicit && own.is_some_and(|instance| {
                instance.base_id == *g && tys.len() == instance.params.len()
                    && tys.iter().enumerate().all(|(i, ty)| matches!(cx.ck.uni.zonk(ty), Ty::Param(ref n) if cx.tparams.get(i).is_some_and(|t| t.as_str() == n.as_str())))
                    && args.iter().enumerate().all(|(i, arg)| matches!(arg, ECore::Var(LocalId(j)) if *j == i) || matches!(arg, ECore::Unit))
            });
            let symbol = if is_self { GlobalId(format!("{}#self", g.0)) } else { g.clone() };
            C::InstDict(symbol, args.iter().map(low).collect(), cx.classes(tys))
        }
        ECore::App(f, args) => C::App(Box::new(low(f)), args.iter().map(low).collect()),
        ECore::Lam { .. } | ECore::PolyIntro { .. } | ECore::Hole(_) | ECore::IdxPending(_) => {
            unreachable!("lambda hoisting and dictionary solving run before lowering")
        }
        ECore::QuoteHole(idx) => C::SpliceVal(*idx),
        ECore::Quote { template, holes, body } => C::Quote {
            template: cx.ck.lower_code_expectation(template),
            holes: holes.iter().map(low).collect(),
            body: Some(Box::new(low(body))),
        },
        ECore::DeclsQuote { template, holes, name_holes } => C::DeclsQuote {
            template: template.clone(),
            holes: holes.iter().map(low).collect(),
            name_holes: name_holes.clone(),
        },
        ECore::SyntaxLit(toks, scope) => C::SyntaxLit(toks.clone(), scope.clone()),
        ECore::StrLit(s) => C::StrLit(s.clone()),
        ECore::Fault => C::Fault,
        ECore::MkClos { fn_id, env } => C::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(low).collect(),
            // A closure's unit is the definition's own hoisted lambda or
            // `letrec` member: it shares the definition's parameters.
            classes: crate::repr::identity(cx.tparams.len()),
        },
        ECore::Let(l, v, body) => {
            let c = cx.let_class(*l, v);
            cx.inferred.borrow_mut().insert(*l, c);
            C::Let(*l, c, Box::new(low(v)), Box::new(low(body)))
        }
        ECore::If(c, t, f) => C::If(
            Box::new(low(c)),
            CBranch {
                weight: t.weight,
                body: Box::new(low(&t.body)),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(low(&f.body)),
            },
        ),
        ECore::Demand(d, ty) => C::Demand(Box::new(low(d)), cx.class(ty)),
        ECore::LazyAlloc => C::LazyAlloc,
        ECore::LazyInit(l, t) => C::LazyInit(*l, Box::new(low(t))),
        ECore::BoxAlloc => C::BoxAlloc,
        ECore::BoxInit(l, v, ty) => C::BoxInit(*l, Box::new(low(v)), cx.class(ty)),
        ECore::RefSet(l, v, ty) => C::RefSet(*l, Box::new(low(v)), cx.class(ty)),
        ECore::BoxGet(v, ty) => C::BoxGet(Box::new(low(v)), cx.class(ty)),
        ECore::BoxClear(v, ty) => C::BoxClear(Box::new(low(v)), cx.class(ty)),
        ECore::Case { scrut, alts } => C::Case {
            scrut: Box::new(low(scrut)),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: match &a.pat {
                        CPat::Var(l, _) => {
                            let c = cx.let_class(*l, scrut);
                            cx.inferred.borrow_mut().insert(*l, c);
                            CPat::Var(*l, c)
                        }
                        CPat::Con { tag, fields, .. } => CPat::Con {
                            tag: *tag,
                            fields: fields.clone(),
                            classes: fields.iter().map(|f| cx.local(*f)).collect(),
                        },
                        CPat::Wild => CPat::Wild,
                    },
                    weight: a.weight,
                    body: Box::new(low(&a.body)),
                })
                .collect(),
        },
    }
}

/// A computational %0 binder creates a call boundary but takes no word.
/// Group adjacent runtime binders so ordinary saturated calls keep their
/// existing ABI. The erased stage itself is a zero-parameter lambda.
fn erased_parameter_stages(parameters: &[Option<LocalId>], body: ECore) -> ECore {
    match parameters.split_first() {
        None => body,
        Some((None, rest)) => ECore::Lam {
            params: Vec::new(),
            body: Box::new(erased_parameter_stages(rest, body)),
        },
        Some((Some(_), _)) => {
            let prefix = parameters.iter().take_while(|parameter| parameter.is_some()).count();
            let params = parameters[..prefix].iter().flatten().copied().collect();
            ECore::Lam {
                params,
                body: Box::new(erased_parameter_stages(&parameters[prefix..], body)),
            }
        }
    }
}

fn app_spine(head: ECore, args: Vec<ECore>) -> ECore {
    match head {
        ECore::App(f, mut existing) if !matches!(&*f, ECore::Prim(PrimOp::ApplyErased)) => {
            existing.extend(args);
            ECore::App(f, existing)
        }
        other => ECore::App(Box::new(other), args),
    }
}

/// The type former a parenthesized operator spelling names: `(->)` is
/// the pure total arrow family, `(,)`/`(,,)`/... the structural tuple
/// formers (one comma per component minus one).
fn former_of(name: &str) -> Option<crate::types::Former> {
    if name == "->" {
        return Some(crate::types::Former::Arrow);
    }
    let inner = name.strip_prefix('(')?.strip_suffix(')')?;
    if !inner.is_empty() && inner.bytes().all(|b| b == b',') {
        return Some(crate::types::Former::Tuple(inner.len() + 1));
    }
    None
}

// ---------------------------------------------------------------- info tables

/// A declared parameter's kind: a type parameter, an index value with its
/// type (`n :: Nat`), a proposition parameter (`p :: Prop`), or a type
/// constructor parameter of kind `Type -> ... -> Type` (`f :: Type -> Type`).
#[derive(Clone, Debug, PartialEq)]
pub enum ParamKind {
    Type,
    Val(Ty),
    Prop,
    /// `Type -> ... -> Type` with `n >= 1` arrows: a higher-kinded type
    /// constructor parameter (modules and compilation specification
    /// section 1).
    Con(usize),
}

impl ParamKind {
    /// How many `Type` arguments this kind still takes.
    fn arity(&self) -> usize {
        match self {
            ParamKind::Con(n) => *n,
            _ => 0,
        }
    }
}

#[derive(Clone)]
enum ValueKind {
    TopFn,
    Prim(PrimOp),
    /// A user `prim` declaration (primitive operations and inline target
    /// IR specification section 2): its synthesized unit, parameter
    /// count and immediate (`immarg`) parameter indices. Calls reference
    /// the unit like an ordinary function; the inline body is target
    /// code and the unit is a leaf.
    PrimDecl { unit: String, arity: usize, immarg: Vec<usize> },
    /// A real `foreign import ccall` binding: its synthesized marshalling
    /// wrapper unit and parameter count (foreign function interface
    /// specification section 2).
    Foreign { unit: String, arity: usize },
    /// Class method at position `idx` of its signature.
    Method { idx: usize },
    /// A `law` declaration on a `signature` (primitive operations and
    /// inline target IR specification section 5): citable `justify`
    /// evidence stating the equation instances are checked against. No
    /// instance supplies its body; it is erased evidence.
    Law,
    /// An effect operation (`effect L where op :: ...`): dispatches
    /// through the handler record of the enclosing `handle`.
    EffectOp {
        /// The effect's label head.
        head: String,
        /// Vtable slot (slot 0 is the return clause).
        slot: usize,
    },
    /// Data constructor: tag in declaration order, all field types
    /// (including erased evidence fields), the fields' erasure flags, the
    /// family's parameter names (positional against the type's
    /// arguments), constructor-local index binders, and the result's
    /// arguments (`Vec 0 a`'s `0`, `Vec (k + 1) a`'s `k + 1`) with the
    /// parameters still as placeholders. A refinement's transparent
    /// constructor carries `refine`: its runtime form is its container
    /// value (the witness field erases and no block is allocated).
    Ctor {
        tag: usize,
        /// Pattern arity: every declared field.
        arity: usize,
        /// Runtime arity: only the fields the block stores.
        runtime_arity: usize,
        field_tys: Vec<Ty>,
        field_erased: Vec<bool>,
        /// Each field's Pi binder name, for dependent field types
        /// (`(bound :: i < n)` mentions `i`).
        field_binders: Vec<String>,
        ty_params: Vec<String>,
        local_idx: Vec<IxParam>,
        res_args: Vec<Ty>,
        /// `Some(refinement type name)` for a refinement's transparent
        /// constructor (refinement types design: the runtime
        /// representation IS the container).
        refine: Option<String>,
    },
    BoolLit(bool),
    /// An erased evidence constructor (`refl`, `congr`): it constructs a
    /// proof and never becomes runtime code (core specification
    /// section 3).
    ProofCtor,
    /// Experimental, goal-directed proof elaboration. Never a runtime value.
    ProofSearch,
    /// A `stage` definition: compile-time code (staging, P9). It may only
    /// be referenced from compile-time code.
    StageFn,
    /// A `macro` definition: a compile-time function over token trees.
    MacroFn,
    /// A builtin of the compile-time capability API (`Stage.*`).
    StageBuiltin(crate::core::StageOp),
}

#[derive(Clone)]
struct ValueInfo {
    module: String,
    name: String,
    scheme: Scheme,
    kind: ValueKind,
}

#[derive(Clone)]
struct RefineInfo {
    /// The container type with the refinement's parameters as `Ty::Param`
    /// placeholders (substituted per application).
    base: Ty,
    /// The refined binder (`refine (xs : T) with p`'s `xs`): the name the
    /// predicate mentions and the transparent constructor binds.
    binder: String,
    /// The decidable predicate over the binder's value.
    pred: PTerm,
    /// The refinement's parameter names, positional against the type's
    /// arguments.
    params: Vec<String>,
}

/// One lattice point of the refinement family (refinement types design,
/// "narrowings and the implication theory"): the container, the
/// predicate over the refined binder, and whether the container is
/// stated (an anonymous `refined P` in an ascription may leave it to the
/// ascribed value). Named refinements (`SizedAscii`) and anonymous
/// points (`refined P T`) are the same family: nominal names are named
/// predicates for readability, nothing more.
#[derive(Clone)]
pub struct RefineView {
    /// The container type (the runtime representation).
    pub base: Option<Ty>,
    /// The predicate over the binder's value.
    pub pred: PTerm,
    /// The name the predicate states its truth over (a named
    /// refinement's declared binder; the anonymous former's first free
    /// name).
    pub binder: String,
}

/// One converted `using (...)` item (refinement types design,
/// "instance heads at refinements"): a class constraint (a dictionary
/// binder) or a refinement obligation the caller discharges through the
/// `Dec` evidence path.
#[derive(Clone)]
pub(crate) enum ConvOblig {
    Class(String, ClassApp),
    Prop(PropOblig),
}

/// One `using (...)` item on an instance: a dictionary constraint (a
/// sub-dictionary of the instance's construction) or a refinement
/// obligation (its per-argument predicates and the generic transfer's
/// `Preserves` validity condition).
#[derive(Clone, PartialEq)]
pub(crate) enum InstCtx {
    Dict(String, ClassApp),
    Oblig(PropOblig),
}

impl InstCtx {
    /// The dictionary items of one instance context (the sub-dictionaries
    /// of its construction unit).
    pub(crate) fn dicts(ctx: &[InstCtx]) -> Vec<(String, ClassApp)> {
        ctx.iter()
            .filter_map(|c| match c {
                InstCtx::Dict(n, app) => Some((n.clone(), app.clone())),
                InstCtx::Oblig(_) => None,
            })
            .collect()
    }
}

/// One instance candidate in lattice-aware resolution (refinement types
/// design, "instance heads at refinements"): the trial-matched instance
/// with the head condition it states at the want (the predicate whose
/// values its methods serve; `None` is the trivial condition of a plain
/// head) and whether the head's predicate came from a predicate
/// parameter (the generic transfer: the least specific head). The trial
/// itself is rolled back; `fresh` and `fresh_before` (how many type and
/// proposition parameters earlier candidates had allocated) are what
/// replaying it on the resolver's store needs.
struct MatchCand {
    idx: usize,
    sub: HashMap<String, Ty>,
    fresh: HashMap<String, Ty>,
    fresh_before: (usize, usize),
    cond: Option<PTerm>,
    generic_head: bool,
}

/// One application whose selected instance carries per-argument
/// refinement obligations (refinement types design, "instance heads at
/// refinements"): the arguments and the `Prop`-typed evidence in scope
/// at the call, recorded for the discharge after instance resolution
/// (the `Dec` evidence path; D192 staleness applies).
#[derive(Clone)]
struct PropCall {
    /// The method call's dictionary want.
    want: usize,
    /// The method index when the call's head is the method projection
    /// itself (`x - y`), the one call form at which an instance's
    /// per-method obligations over the method's own arguments can
    /// discharge. `None` when the want was found elsewhere in the call
    /// (a dictionary passed to a generic callee, a method passed as an
    /// argument).
    method: Option<usize>,
    /// The arguments in order: their values (where a named value stands
    /// for the argument), their types, and their spans.
    args: Vec<(Option<ITerm>, Ty, ast::Span)>,
    /// The evidence in scope at the call (`evidence_in`).
    evidence: Evidence,
    /// The application leaves a function (the method is not fully
    /// applied: its obligations cannot discharge at this call).
    residual_fun: bool,
    span: ast::Span,
}

/// What a refinement obligation's discharge may read at one use: the
/// `Prop`-typed binders in scope (name, proposition, local and mutable
/// footprint), the integer sorts of the locals (what their arithmetic
/// means to the linear procedure), and the predicates of refined
/// locals, stated of the local (they hold of every value of the type,
/// so they never go stale).
#[derive(Clone, Debug, Default)]
pub(crate) struct Evidence {
    pub items: Vec<(String, PTerm, LocalId, Vec<String>)>,
    pub sorts: Vec<(String, IntSort)>,
    pub value_sorts: Vec<(LocalId, IntSort)>,
    /// Lexical names for checked value identities at this evidence use.
    /// Hidden identities retain their identity instead of borrowing a shadow.
    pub visible_values: HashMap<LocalId, ITerm>,
    pub refined: Vec<PTerm>,
}

/// One refinement transfer recorded at a resolved want (refinement
/// types design, "composability"): the instance serving a refined point
/// owes the point's preservation check (the closure `post p p = p`) and
/// its preservation and postcondition rows in the law layer's honest
/// tier.
struct TransferOblig {
    /// The row identity: the rep-identity dictionary's owner when the
    /// transfer is the generic rule (its same-class sub-dictionary
    /// resolves to the container instance), the selected instance
    /// otherwise.
    container: String,
    module: String,
    instance: String,
    class: String,
    /// The method owing the point's preservation (its result is the
    /// point's carrier, or a refined result inside another former at
    /// the honest boundary).
    method: String,
    nested: bool,
    class_params: Vec<String>,
    class_args: Vec<Ty>,
    /// The wanted lattice point (the refined class argument), its
    /// predicate, binder and container.
    point: Ty,
    pred: PTerm,
    binder: String,
    base: Ty,
    /// The instance's sub-dictionary constraints (class and arguments),
    /// for the law machinery's dictionary resolution.
    subdicts: Vec<(String, Vec<Ty>)>,
    span: ast::Span,
}

#[derive(Clone)]
struct TypeKind {
    module: String,
    name: String,
    /// Stable nominal identity in checked types. Public names and import
    /// aliases remain surface spellings; they must not drive unification.
    identity: String,
    arity: usize,
    /// Each parameter's kind, in declaration order: type parameters,
    /// index values (`Vec (n :: Nat)`'s `n`), and proposition parameters
    /// (`Dec (p :: Prop)`'s `p`).
    param_kinds: Vec<ParamKind>,
    /// A checked data declaration (including the installed indexed data
    /// families), rather than an opaque, primitive or dictionary carrier.
    data_family: bool,
    /// Constructor names with tags, in declaration order.
    ctors: Vec<(String, usize)>,
    /// For a `record` type: the declared field names in declaration order.
    /// `None` for `data` types.
    fields: Option<Vec<String>>,
    /// Record fields' erasure flags, parallel to `fields`.
    field_erased: Vec<bool>,
    /// An `opaque type` (foreign function interface specification
    /// section 2): abstract, no constructors, no layout projection.
    opaque: bool,
    /// A refinement (refinement types design): the container, the
    /// refined binder and its decidable predicate. The type's runtime
    /// representation IS the container's.
    refine: Option<RefineInfo>,
}

/// The nominal identity of a source-declared type: its declaring module
/// and its name. It depends on nothing else, so adding, removing or
/// renaming another declaration anywhere never changes it, and a project
/// that ships its own source for a base module name declares the same
/// identities the embedded copy does.
fn nominal_identity(module: &str, name: &ast::Name) -> String {
    format!("{module}.{}", name.text)
}

#[derive(Clone)]
struct SigInfo {
    module: String,
    name: String,
    params: Vec<String>,
    /// Each parameter's kind, in declaration order: a class over type
    /// constructors takes partially applied arguments (`Functor List`).
    param_kinds: Vec<ParamKind>,
    /// Method name, method scheme; the class constraint is `ctx[0]`.
    methods: Vec<(String, Scheme)>,
    /// Default equations by method name, in method order: an instance
    /// that omits the method checks these at its own types, in the
    /// class module's scope (the names the default was written against).
    defaults: Vec<(String, Vec<ast::Equation>)>,
    /// `law` members (primitive operations and inline target IR
    /// specification section 5): name and stated equation scheme.
    laws: Vec<(String, Scheme)>,
}

#[derive(Clone, PartialEq)]
struct InstanceInfo {
    module: String,
    name: String,
    params: Vec<String>,
    ctx: Vec<InstCtx>,
    target: ClassApp,
    /// The dictionary construction unit for this instance.
    base_id: GlobalId,
    implicit: bool,
    /// The declared instance of a parameterized module (module parameters
    /// as module arguments): its dictionary needs the module's parameters,
    /// so it is never resolved on its own. Each instantiation publishes its
    /// own copy under the instantiation unit's name.
    base: bool,
    /// Per-method obligations over the method's own arguments, from a
    /// method signature in the instance block (`(-) :: using (y <= x)
    /// => (x :: Nat) -> (y :: Nat) -> Nat`): the method's index in its
    /// class, its name, and the predicates over the positional binders
    /// `$a{i}`. They discharge at each direct, full application of the
    /// method; the dictionary never reaches code that could apply the
    /// method without them (a generic callee, a sub-dictionary).
    method_obligs: Vec<(usize, String, Vec<PTerm>)>,
    /// Checked per-method signatures retained for interface proof contracts.
    method_contracts: Vec<(usize, String, Scheme)>,
}

impl InstanceInfo {
    fn closed_declaration_header(&self) -> bool {
        let empty = Unifier::new();
        let closed = |ty: &Ty| !ty.has_declaration_pending() && empty.owns_all_metas(ty);
        let class_closed = |class: &ClassApp| class.args.iter().all(&closed);
        let proposition_closed = |proposition: &PTerm| closed(&Ty::Prp(Box::new(proposition.clone())));
        let obligation_closed = |obligation: &PropOblig| match obligation {
            PropOblig::Pred(predicate) | PropOblig::Preserves { pred: predicate, .. } => proposition_closed(predicate),
        };
        class_closed(&self.target)
            && self.ctx.iter().all(|context| match context {
                InstCtx::Dict(_, class) => class_closed(class),
                InstCtx::Oblig(obligation) => obligation_closed(obligation),
            })
            && self.method_obligs.iter().all(|(_, _, predicates)| predicates.iter().all(&proposition_closed))
            && self.method_contracts.iter().all(|(_, _, scheme)| {
                closed(&scheme.ty)
                    && scheme.ctx.iter().all(|(_, class)| class_closed(class))
                    && scheme.obligs.iter().all(&obligation_closed)
                    && scheme.idx_params.iter().all(|parameter| match parameter { IxParam::Val(_, ty) => closed(ty), _ => true })
                    && scheme.val_params.iter().all(|(_, ty)| closed(ty))
            })
    }
}

/// One module parameter of a parameterized module (`module M (x :: T)
/// where`; modules specification section 1), converted against the
/// module's visible names. A `signature`-typed parameter is a dictionary
/// binding (`class` is its class application); any other type is an
/// ordinary value binding.
#[derive(Clone)]
pub(crate) struct ModParam {
    pub name: String,
    pub ty: Ty,
    pub class: Option<ClassApp>,
    /// The parameter's signature rendering for the interface's parameter
    /// identity record (`x :: T`).
    pub sig: String,
}

/// One instantiation unit (`import M (args) as N`, module parameters as
/// module arguments). The unit's values are wrapper units over the target
/// module's functions with the static arguments applied (instantiation is
/// representation; the target's contract is unchanged).
#[derive(Clone)]
pub(crate) struct UnitRec {
    pub owner: String,
    pub name: String,
    pub target: String,
    /// Closed argument code, in parameter order.
    pub args: Vec<crate::core::Core>,
    /// Argument identities for the unit's computed identity.
    pub arg_ids: Vec<String>,
    /// The unit's computed identity: the target module and its parameter
    /// signatures over the argument identities, so two instantiations at
    /// different arguments are distinct units.
    pub unit_id: String,
    /// Instances this unit publishes: `(instance name, class, target,
    /// ctx, implicit)` over the target module's declared instances.
    pub insts: Vec<UnitInst>,
}

/// One instance published by an instantiation unit: the target module's
/// declared instance, constructed at the unit's static arguments.
#[derive(Clone)]
pub(crate) struct UnitInst {
    pub inst_name: String,
}

struct RuntimeIndexReading {
    name: ast::Name,
    locals: Vec<LocalId>,
    dictionaries: Vec<LocalId>,
    core: ECore,
    carrier: Ty,
}

#[derive(Clone)]
struct Want {
    id: usize,
    class: ClassApp,
    span: ast::Span,
    scope: Vec<(LocalId, ClassApp)>,
    /// A method call's dictionary want (its instance's per-argument
    /// refinement obligations discharge at the call's arguments).
    method: bool,
    evidence_index: Option<ITerm>,
}

/// A deferred integer literal (primitive types specification, section 7):
/// it elaborates against its expected type, which may only become known
/// after unification and instance resolution, so the constant is fixed by
/// [`Checker::resolved_literals`] once the definition is complete.
struct PendLit {
    /// The literal's type metavariable.
    ty: Ty,
    /// The decimal digit string, with an optional leading `-`.
    value: String,
    span: ast::Span,
}

/// A deferred implicit index argument (insert-and-solve, dependent core
/// kernel design, section 2): a `forall`-bound value argument is a runtime
/// argument, so its solved index term is reified to runtime code once
/// unification is complete (core specification section 3: witnesses
/// execution needs are retained).
#[derive(Clone, Default)]
struct IndexRuntimeScope {
    values: HashMap<String, LocalId>,
    locals: HashSet<LocalId>,
    dictionaries: HashSet<LocalId>,
}

#[derive(Clone, Default)]
struct LawFrame {
    values: HashMap<String, crate::core::Value>,
    dictionaries: HashMap<IndexEvidence, crate::core::Value>,
}

impl std::ops::Deref for LawFrame {
    type Target = HashMap<String, crate::core::Value>;
    fn deref(&self) -> &Self::Target { &self.values }
}
impl std::ops::DerefMut for LawFrame {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.values }
}

struct IdxPend {
    /// The instantiated index metavariable.
    iterm: ITerm,
    /// Name to local mapping at the use site, for reifying variables.
    scope: IndexRuntimeScope,
    /// The checked carrier selects the complete runtime representation.
    carrier: Ty,
    span: ast::Span,
}

/// One write step recorded while elaborating a definition (evidence
/// invalidation over mutable state, core specification section 5; the
/// temporal contracts' per-step transitions, core specification section
/// 10). `loc` is the written location (a `Mut` borrow's region) or
/// `None` for an opaque write (an FFI / `RawPtr` write or `eval`). A
/// direct `refSet` records the written value's index shape; a call's
/// potential write records the callee and the call's index arguments,
/// so a self-recursive call can discharge its step by the inductive
/// hypothesis.
#[derive(Clone, Debug)]
pub(crate) struct EvWrite {
    pub loc: Option<String>,
    pub span: ast::Span,
    /// The written value's index shape at a direct `refSet` step.
    pub value: Option<ITerm>,
    /// The callee's identity and its index arguments at a call's
    /// potential write.
    pub call: Option<(GlobalId, Vec<Option<ITerm>>)>,
}

#[derive(Clone, Default)]
struct Env {
    locals: Vec<(String, LocalId, Scheme)>,
    dicts: Vec<(LocalId, ClassApp)>,
    syntax_locals: HashMap<ast::LocalBinding, usize>,
    syntax_dictionaries: HashMap<ast::LocalBinding, LocalId>,
}

impl Env {
    fn syntax_scope(&mut self, module: &str, span: ast::Span) -> crate::syntax::SyntaxScope {
        let mut names = Vec::new();
        for (slot, (spelling, _, _)) in self.locals.iter().enumerate().rev() {
            if spelling.starts_with(['#', '$']) || names.iter().any(|n: &ast::Name| n.text == *spelling) { continue; }
            let key = self.syntax_locals.iter().find(|(_, index)| **index == slot).map(|(key, _)| key.clone())
                .unwrap_or_else(|| {
                    let key = ast::LocalBinding::fresh();
                    self.syntax_locals.insert(key.clone(), slot);
                    key
                });
            names.push(ast::Name::bound(spelling, span, key));
        }
        let dictionaries = self.dicts.iter().map(|(local, _)| {
            if let Some((binding, _)) = self.syntax_dictionaries.iter().find(|(_, candidate)| *candidate == local) { return binding.clone(); }
            let binding = ast::LocalBinding::fresh();
            self.syntax_dictionaries.insert(binding.clone(), *local);
            binding
        }).collect();
        crate::syntax::SyntaxScope::from_names(module, names).with_dictionary_bindings(dictionaries)
    }
    fn name_slot(&self, name: &ast::Name) -> Option<usize> {
        if let Some(binding) = name.local_binding() {
            return self.syntax_locals.get(binding).copied().filter(|slot| *slot < self.locals.len());
        }
        if name.resolved_binding().is_some() || name.scoped_reference().is_some() || name.declaration_binding().is_some() { return None; }
        self.locals.iter().rposition(|(spelling, _, _)| *spelling == name.text)
    }

    fn lookup_name(&self, name: &ast::Name) -> Option<(LocalId, Scheme)> {
        let (_, local, scheme) = self.locals.get(self.name_slot(name)?)?;
        Some((*local, scheme.clone()))
    }

    fn lookup(&self, name: &str) -> Option<(LocalId, Scheme)> {
        self.locals
            .iter()
            .rev()
            .find(|(n, _, _)| n == name)
            .map(|(_, l, s)| (*l, s.clone()))
    }
}

/// A `~`-binding memo group: one region, one `Eval` label (laziness and
/// reclamation specification, sections 5 and 6).
struct MemoGroup {
    region: String,
    label: EffLabel,
}

/// A user-declared effect (`effect L r where op :: ...`): its label and
/// its operations (design/effects-capabilities-sequencing.md section 2).
struct EffectInfo {
    module: String,
    name: String,
    /// The effect's region parameter name, when the label is `L r`.
    region_param: Option<String>,
    /// Operation name and scheme, in declaration order; the vtable slot is
    /// the index plus one (slot 0 is the return clause).
    ops: Vec<(String, Scheme)>,
}

/// How often a continuation closure's body runs. A continuation runs at
/// most once per execution of its build site, except under a provider
/// that reruns it; a body over `%1` captures needs exactly-once
/// semantics, which handler resumptions must promise (see
/// [`Checker::note_use`]).
#[derive(Clone, Debug)]
enum ContKind {
    /// Ordinary control continuations and direct-provider `do` binds.
    Once,
    /// `Result`/`Option` provider binds: the continuation may be skipped
    /// by short-circuiting, so linear captures inside are rejected.
    AtMost,
    /// `STM` provider binds: the continuation is part of a transaction,
    /// which runs once per attempt and may be skipped by `retry`, so a
    /// capture is used any number of times, like a lambda's.
    Many,
}

/// A continuation closure currently being elaborated.
struct ContFrame {
    kind: ContKind,
    /// The continuation's build-site subterm contains an operation call:
    /// its evaluation may hand control to a handler clause, which may drop
    /// the resumption and skip this body.
    risky: bool,
    /// Set when the body consumes a `%1` capture: a resumption capturing
    /// this continuation must then resume exactly once.
    consumes_linear: bool,
}

/// A continuation as a value: the local bound to a one-parameter closure
/// (`ContFrame` at index `frame` recorded its body), whose parameter has
/// type `ty`. Continuation-passing elaboration flows every computed value
/// into it.
#[derive(Clone)]
struct Cont {
    kv: LocalId,
    ty: Ty,
    frame: usize,
}

/// The handler extent of one `handle e with h` elaboration: operation
/// calls for the handled label dispatch through the handler record.
struct HandlerExtent {
    /// The handled label.
    label: crate::types::EffLabel,
    /// The local bound to the handler vtable (a tuple of clause closures).
    vtable: LocalId,
    /// The `resume k` binders of the handler's operation clauses (per
    /// slot), when the handler is written inline at this site.
    k_binders: Vec<Option<LocalId>>,
    /// Whether those binders may still be promoted to `%1`.
    flexible: bool,
    /// Lambda and memo depths when the extent opened: an operation
    /// dispatches here only outside nested lambda and memo bodies.
    lam: usize,
    memo: usize,
}

/// Compiler-known sequencing provider class heads, as class keys
/// ([`Checker::signature_key`]): the three of the embedded family through
/// the registry, and the transaction provider at its defining module,
/// which ships as source.
const SEQ_CLASSES: [&str; 4] = [
    crate::known::DIRECT_SEQUENCING,
    crate::known::RESULT_SEQUENCING,
    crate::known::OPTION_SEQUENCING,
    STM_SEQUENCING,
];

/// The class key of `Effect.Stm`'s `do` provider.
const STM_SEQUENCING: &str = "Effect.Stm.StmSequencing";

/// The maximum `let`/`letrec` nesting the elaborator recurses into.
/// A deeper nesting gets the named let-nesting depth-limit diagnostic
/// (`in_let`) instead of a stack overflow or a silent pass: the limit
/// is the hardening, wide test stacks remain how deep-but-legal
/// programs run.
const LET_DEPTH_LIMIT: usize = 128;

/// The selected sequencing provider of the definition being checked
/// (`design/effects-capabilities-sequencing.md` section 5): the nearest
/// explicit `using` binder with a provider class head, or the direct
/// provider by default. Two active providers in one selection scope are
/// ambiguous.
fn select_provider(binders: &[ClassApp], span: ast::Span) -> Result<ClassApp> {
    match binders {
        [] => Ok(ClassApp { class: crate::known::DIRECT_SEQUENCING.into(), args: Vec::new() }),
        [one] => Ok(one.clone()),
        many => Err(Error::at(
            span,
            crate::diagnostic!(
                "two active sequencing providers in one selection scope are ambiguous ({}); bind one explicitly",
                crate::spelling::Listed(many, ", ")
            ),
        )),
    }
}

/// Is this class head a runtime capability? `Io r` is runtime evidence for
/// which implementation executes a computation (decision D21); every
/// other head is compile-time class evidence.
fn is_capability_class(class: &str) -> bool {
    class == "Io"
}

/// Is this expression a syntactic value? Let-generalization is restricted
/// to these (the value restriction).
fn is_syntactic_value(t: &ast::Term) -> bool {
    match t {
        ast::Term::Lam(_, _, _)
        | ast::Term::Var(_)
        | ast::Term::Con(_)
        | ast::Term::Int(_, _)
        | ast::Term::Unit(_) => true,
        ast::Term::Tuple(items, _) | ast::Term::List(items, _) => {
            items.iter().all(is_syntactic_value)
        }
        _ => false,
    }
}

/// The metavariables free in the environment (its locals and dictionary
/// binders), for let-generalization.
fn env_free_metas(uni: &Unifier, env: &Env) -> Vec<EitherMeta> {
    let mut out = Vec::new();
    for (_, _, s) in &env.locals {
        for m in uni.free_metas(&s.ty) {
            if !out.contains(&m) {
                out.push(m);
            }
        }
    }
    for (_, c) in &env.dicts {
        for a in &c.args {
            for m in uni.free_metas(a) {
                if !out.contains(&m) {
                    out.push(m);
                }
            }
        }
    }
    out
}

/// A stored Code object has one payload expectation. Quantifying that
/// expectation would re-evaluate a producer when the stored object is used.
/// Function bodies may produce fresh Code on each invocation, so arrows stop
/// this walk; containers of already-created Code do not.
fn stored_code_metas(ty: &Ty, out: &mut Vec<EitherMeta>) {
    match ty {
        Ty::Adt(name, _) if name == "Code" => ty.free_metas(out),
        Ty::Adt(_, arguments) | Ty::Tuple(arguments) => {
            for argument in arguments { stored_code_metas(argument, out); }
        }
        Ty::Sigma { dom, cod, .. } => {
            stored_code_metas(dom, out);
            stored_code_metas(cod, out);
        }
        Ty::Forall(scheme) => stored_code_metas(&scheme.ty, out),
        Ty::Ptr { pointee: inner, .. } | Ty::Thunk { inner, .. }
        | Ty::Lazy { inner, .. } | Ty::Later(inner) | Ty::Ref(_, inner)
        | Ty::Mut(_, inner) => stored_code_metas(inner, out),
        Ty::App(head, arguments) => {
            stored_code_metas(head, out);
            for argument in arguments { stored_code_metas(argument, out); }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------- usage tracking

/// Uses under a lambda may run any number of times, including none.
const INF: u32 = u32::MAX;

/// A use count over evaluation paths: at least `min` and at most `max`
/// uses on any one path (ownership and usage design, section 2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Count {
    min: u32,
    max: u32,
}

impl Count {
    const ZERO: Count = Count { min: 0, max: 0 };
    const ONE: Count = Count { min: 1, max: 1 };
    /// A use under a lambda body: the closure may be called zero or many
    /// times, so the use is unbounded there.
    const UNBOUNDED: Count = Count { min: 0, max: INF };
    /// A use under a memo cell body: the cell evaluates at most once.
    const AT_MOST_ONCE: Count = Count { min: 0, max: 1 };

    /// Sequential composition: uses add along one path.
    fn seq(self, o: Count) -> Count {
        Count {
            min: self.min.saturating_add(o.min),
            max: self.max.saturating_add(o.max),
        }
    }

    /// Branch composition: each path takes one side.
    fn join(self, o: Count) -> Count {
        Count { min: self.min.min(o.min), max: self.max.max(o.max) }
    }

    /// The fusion of disjoint places below one root: one split of the
    /// value per path, taken as the pointwise maximum of the parts.
    fn fuse(self, o: Count) -> Count {
        Count { min: self.min.max(o.min), max: self.max.max(o.max) }
    }

    fn describe(self) -> String {
        if self.max == 0 {
            "never used".to_string()
        } else if self.max == INF {
            "used under a lambda or thunk body (zero or unboundedly many times)".to_string()
        } else if self.min == self.max {
            format!("used {} time(s)", self.min)
        } else if self.min == 0 {
            format!("used 0 to {} time(s) (unused on some path)", self.max)
        } else {
            format!("used {} to {} time(s)", self.min, self.max)
        }
    }
}

/// Use counts over record-projection places rooted at one binder: `x`,
/// `x.f` and `x.f.g` are distinct places. Occurrences of pairwise disjoint
/// places fuse into one consumption of the root (a split of the value);
/// repeated or nested places are separate consumptions (ownership and
/// usage design, section 2).
#[derive(Clone, Debug, Default)]
struct UseNode {
    occ: Count,
    kids: Vec<(String, UseNode)>,
}

impl UseNode {
    fn add(&mut self, path: &[String], c: Count) {
        if path.is_empty() {
            self.occ = self.occ.seq(c);
            return;
        }
        match self.kids.iter_mut().find(|(n, _)| *n == path[0]) {
            Some((_, k)) => k.add(&path[1..], c),
            None => {
                let mut k = UseNode::default();
                k.add(&path[1..], c);
                self.kids.push((path[0].clone(), k));
            }
        }
    }

    /// Sequential composition along one path.
    fn seq_add(&mut self, other: &UseNode) {
        self.occ = self.occ.seq(other.occ);
        for (n, k) in &other.kids {
            match self.kids.iter_mut().find(|(m, _)| m == n) {
                Some((_, mine)) => mine.seq_add(k),
                None => self.kids.push((n.clone(), k.clone())),
            }
        }
    }

    /// Branch composition: each path takes one side; a place absent from
    /// one side counts zero there.
    fn join(&mut self, other: &UseNode) {
        self.occ = self.occ.join(other.occ);
        let mut names: Vec<String> = self.kids.iter().map(|(n, _)| n.clone()).collect();
        for (n, _) in &other.kids {
            if !names.contains(n) {
                names.push(n.clone());
            }
        }
        for n in names {
            let mine = match self.kids.iter_mut().find(|(m, _)| *m == n) {
                Some((_, k)) => k,
                None => {
                    self.kids.push((n.clone(), UseNode::default()));
                    &mut self.kids.last_mut().expect("just pushed").1
                }
            };
            match other.kids.iter().find(|(m, _)| *m == n) {
                Some((_, k)) => mine.join(k),
                None => mine.join(&UseNode::default()),
            }
        }
    }

    /// The consumptions of the root this subtree implies: the root's own
    /// occurrences, plus one split per path fusing the disjoint places
    /// below it.
    fn consumptions(&self) -> Count {
        let mut fused: Option<Count> = None;
        for (_, k) in &self.kids {
            let kc = k.consumptions();
            fused = Some(match fused {
                None => kc,
                Some(f) => f.fuse(kc),
            });
        }
        match fused {
            None => self.occ,
            Some(f) => self.occ.seq(f),
        }
    }
}

/// A binder's usage obligation (ownership and usage design, section 2).
struct Obligation {
    name: String,
    /// The declared multiplicity; `None` means the type-directed default
    /// ([`default_mult`]) applied once the type is determined.
    mult: Option<Mult>,
    ty: Ty,
    span: ast::Span,
    /// Lazy-position depths at binding time: lambda bodies and memo cell
    /// bodies (uses deeper than these are lazy-position uses).
    lam: usize,
    memo: usize,
    /// Continuation depth at binding time: uses inside a continuation
    /// closure over this binder are resumption-position uses.
    cont: usize,
}

/// A one-shot frame: a task body or a `withScope` body (concurrency
/// specification section 2). The body runs exactly once per spawn or
/// scope execution, so a use directly in it is a one-shot use of its
/// captures, not an unbounded closure capture.
struct OneshotFrame {
    /// The lambda depth inside the body (its uses have this depth).
    lam: usize,
    /// Whether this is a task body (the task capture rules apply) or a
    /// scope body.
    task: bool,
    /// Whether this is a parallel callback. Its value runs inside the call
    /// that builds it and is dropped when the call returns, so it cannot
    /// outlive a borrowed region: the conservative borrowed-environment
    /// rule does not apply, and the parallel capture rules decide instead
    /// (`check_parallel_captures`).
    parallel: bool,
    /// The scope this frame belongs to (its region identity).
    scope: String,
    /// The memo regions active when the scope opened: a borrow of these
    /// outlives the scope's joins and may be captured.
    outer: Vec<String>,
}

/// A structured scope whose body is being checked.
struct ScopeFrame {
    /// The scope's region identity (`Task s x a` and `Scope r s` mention
    /// it; it may not leave the scope).
    region: String,
    /// The memo regions active when the scope opened: a borrow of these
    /// outlives the scope's joins and may be captured by its tasks.
    outer: Vec<String>,
}

/// A capture of a definition-level binder into a task body, validated at
/// the definition's end when unification has resolved the binder's type
/// (concurrency specification, section 3).
struct TaskCapture {
    binder: LocalId,
    name: String,
    ty: Ty,
    span: ast::Span,
    /// The lexical class dictionaries live at the capture (for `Copy`
    /// evidence over abstract parameters).
    dicts: Vec<(LocalId, ClassApp)>,
    /// The task's scope region and the scope stack at the capture.
    scope: String,
    scopes: Vec<String>,
    /// The memo regions active at the capture.
    active: Vec<String>,
    /// The memo regions that predate the task's scope.
    outer: Vec<String>,
    /// Direct one-shot task bodies may transfer a linear or affine capture.
    transfer_allowed: bool,
}

/// A capture of a definition-level binder into a parallel callback,
/// validated at the definition's end when unification has resolved the
/// binder's type (`design/parallel-evaluation.md` section 2.3). A capture
/// must be `Share`, except that a `%1`/`%aff` value may move into exactly
/// one `parBoth` branch; the many-shot forms run their callback once per
/// index, so nothing may move into them.
struct ParCapture {
    binder: LocalId,
    name: String,
    ty: Ty,
    span: ast::Span,
    /// The lexical class dictionaries live at the capture (for `Copy`/
    /// `Share` evidence over abstract parameters).
    dicts: Vec<(LocalId, ClassApp)>,
    /// Whether the callback runs many times (`parTabulate`, `parMap`) or
    /// exactly once (`parBoth`).
    many_shot: bool,
    /// The builtin's source spelling, for the diagnostic.
    op: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StructuralUsage {
    Copy,
    Share,
    Transfer,
}

/// Region names a type mentions in `Ref`/`Mut` borrow positions, with
/// whether each is an exclusive borrow.
fn borrow_regions(ty: &Ty, out: &mut Vec<(String, bool)>) {
    match ty {
        Ty::Ref(r, inner) => {
            out.push((r.clone().to_string(), false));
            borrow_regions(inner, out);
        }
        Ty::Mut(r, inner) => {
            out.push((r.clone().to_string(), true));
            borrow_regions(inner, out);
        }
        Ty::Fun(a, _, _, b) => {
            borrow_regions(a, out);
            borrow_regions(b, out);
        }
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                borrow_regions(a, out);
            }
        }
        Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } => borrow_regions(inner, out),
        _ => {}
    }
}

/// Every region name a type mentions: `Ref`/`Mut`/`Lazy` region fields and
/// effect label arguments (`Eval r`). A value whose type mentions a region
/// cannot outlive that region (ownership and usage design, section 4).
fn escape_regions(ty: &Ty, out: &mut Vec<String>) {
    let note = |r: &str, out: &mut Vec<String>| {
        if !out.iter().any(|x| x == r) {
            out.push(r.to_string());
        }
    };
    match ty {
        Ty::Ref(r, inner) | Ty::Mut(r, inner) => {
            note(r, out);
            escape_regions(inner, out);
        }
        Ty::Lazy { eff, region, inner } => {
            note(region, out);
            for l in &eff.labels {
                if let Some(a) = &l.arg {
                    note(a, out);
                }
            }
            escape_regions(inner, out);
        }
        Ty::Fun(a, _, e, b) => {
            for l in &e.labels {
                if let Some(a2) = &l.arg {
                    note(a2, out);
                }
            }
            escape_regions(a, out);
            escape_regions(b, out);
        }
        Ty::Thunk { eff, inner } => {
            for l in &eff.labels {
                if let Some(a) = &l.arg {
                    note(a, out);
                }
            }
            escape_regions(inner, out);
        }
        Ty::Later(inner) => {
            escape_regions(inner, out);
        }
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                escape_regions(a, out);
            }
        }
        _ => {}
    }
}

/// The value variables a proposition's index terms read (evidence
/// invalidation over mutable state): collecting them lets the checker
/// classify an evidence item as reading a mutable location's contents.
fn collect_idx_reads(t: &ITerm, out: &mut Vec<String>) {
    t.free_vars(out);
}

/// Whether every occurrence of the binder in an index term sits under
/// a baked measure (`length`/`toNat`) or the checked byte-length
/// primitive over an actual Bytes carrier: a measure atom over a non-numeric container reads the
/// binder's value through the measure, never bare and never through a
/// source function a shadowing module could lie with.
fn measure_covers(t: &ITerm, binder: &str, byte_length: bool) -> bool {
    match t {
        ITerm::Var(n) => n != binder,
        ITerm::Inst(_) => !t.mentions(binder),
        ITerm::LocalValue(_) | ITerm::Num(_) | ITerm::Evidence(_) | ITerm::Fn(_) | ITerm::MVar(_) => true,
        ITerm::Lam(_, body) => measure_covers(body, binder, byte_length),
        ITerm::Bin(_, a, b) => measure_covers(a, binder, byte_length) && measure_covers(b, binder, byte_length),
        ITerm::App(f, args) => {
            let measured = matches!(
                &**f,
                ITerm::Fn(k) if k == "<builtin>.length" || k == "<builtin>.toNat"
            ) || (byte_length && args.len() == 1 && matches!(&**f, ITerm::Fn(k) if k == BYTE_LENGTH_INDEX));
            args.iter().all(|a| measure_covers(a, binder, byte_length) || (measured && is_binder(a, binder)))
                && measure_covers(f, binder, byte_length)
        }
    }
}

fn is_binder(t: &ITerm, binder: &str) -> bool {
    matches!(t, ITerm::Var(n) if n == binder)
}

/// The value variables a proposition reads across all its index terms.
fn collect_prop_reads(p: &PTerm, out: &mut Vec<String>) {
    match p {
        PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
            collect_idx_reads(a, out);
            collect_idx_reads(b, out);
        }
        PTerm::Not(q) => collect_prop_reads(q, out),
        PTerm::And(a, b) | PTerm::Or(a, b) => {
            collect_prop_reads(a, out);
            collect_prop_reads(b, out);
        }
        PTerm::Pred(_, a) => collect_idx_reads(a, out),
        PTerm::Lit(_) | PTerm::Var(_) | PTerm::MVar(_) => {}
    }
}

/// Whether an effect label's head denotes an effect that can mutate a
/// region's state (the effect-row reading of "can write"). Concurrency
/// control, memo `Eval`, termination and foreign-component labels do not
/// write a borrow's location; every other head is conservatively treated
/// as able to write the region it names.
fn write_effect_head(head: &str) -> bool {
    !matches!(
        head,
        "Async" | "Cancel" | "Eval" | "Diverge" | "Fail" | "ThreadAffine" | "Foreign" | "Blocking"
    )
}

/// Whether a callee's core form writes memory at an unknown target (a
/// foreign call, `eval`, or a mutating `Bytes` primitive). Such a write is
/// opaque: it conservatively invalidates every mutable-location evidence
/// item (a `RawPtr`/FFI write target carries no tracked evidence).
fn callee_writes_opaque(core: &ECore) -> bool {
    match core {
        ECore::Foreign(_) => true,
        ECore::Prim(crate::core::PrimOp::Eval) => true,
        ECore::Prim(crate::core::PrimOp::Bytes(op)) => {
            // `Data` reads the payload address and writes nothing.
            !matches!(
                op,
                crate::core::BytesOp::Len | crate::core::BytesOp::Get | crate::core::BytesOp::Data
            )
        }
        // The platform ops mutate the filesystem and the process table:
        // conservatively an opaque write.
        ECore::Prim(crate::core::PrimOp::Platform(_)) => true,
        _ => false,
    }
}

/// A call's discharge step (core specification section 10): the callee's
/// identity and the call's index arguments, when the callee is a named
/// definition. A temporal contract's own recursion discharges its step by
/// the inductive hypothesis; anything else has no contract to discharge
/// against.
fn call_step(
    checker: &Checker,
    callee: &ECore,
    env: &Env,
    args: &[ast::Term],
) -> Option<(GlobalId, Vec<Option<ITerm>>)> {
    let ECore::Global(g, _) = callee else {
        return None;
    };
    let idxs = args
        .iter()
        .map(|a| checker.idx_of_term(a, env).ok())
        .collect();
    Some((g.clone(), idxs))
}

/// Every region name a type mentions, including runtime identities and
/// structured-scope regions (`Io r`, `Scope r s`, `Task s x a`) that
/// [`escape_regions`] deliberately skips. Used for task-handle and scope
/// confinement (concurrency specification, section 2).
fn all_regions(ty: &Ty, out: &mut Vec<String>) {
    let note = |r: &str, out: &mut Vec<String>| {
        if !out.iter().any(|x| x == r) {
            out.push(r.to_string());
        }
    };
    match ty {
        Ty::Region(r) => note(r, out),
        Ty::Ref(r, inner) | Ty::Mut(r, inner) => {
            note(r, out);
            all_regions(inner, out);
        }
        Ty::Lazy { eff, region, inner } => {
            note(region, out);
            for l in &eff.labels {
                if let Some(a) = &l.arg {
                    note(a, out);
                }
            }
            all_regions(inner, out);
        }
        Ty::Fun(a, _, e, b) | Ty::Pi { dom: a, eff: e, cod: b, .. } => {
            for l in &e.labels {
                if let Some(a2) = &l.arg {
                    note(a2, out);
                }
            }
            all_regions(a, out);
            all_regions(b, out);
        }
        Ty::Sigma { dom, cod, .. } => {
            all_regions(dom, out);
            all_regions(cod, out);
        }
        Ty::Forall(scheme) => all_regions(&scheme.ty, out),
        Ty::Thunk { eff, inner } => {
            for l in &eff.labels {
                if let Some(a) = &l.arg {
                    note(a, out);
                }
            }
            all_regions(inner, out);
        }
        Ty::Later(inner) => all_regions(inner, out),
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                all_regions(a, out);
            }
        }
        Ty::Ptr { pointee, .. } => all_regions(pointee, out),
        Ty::Handler { label, input, output, eff, .. } => {
            if let Some(a) = &label.arg {
                note(a, out);
            }
            for l in &eff.labels {
                if let Some(a) = &l.arg {
                    note(a, out);
                }
            }
            all_regions(input, out);
            all_regions(output, out);
        }
        _ => {}
    }
}

/// The concurrency API's builtin type shapes (concurrency specification
/// section 2), shared by the builtin schemes and the special application
/// elaboration.
pub(crate) fn conc_io_ty(r: &str) -> Ty {
    Ty::Adt("Io".into(), vec![Ty::Region(r.into())])
}

/// `Scope r s`: a structured scope of runtime identity `r`, bounding the
/// tasks of scope region `s`.
pub(crate) fn conc_scope_ty(r: &str, s: &str) -> Ty {
    Ty::Adt("Scope".into(), vec![Ty::Region(r.into()), Ty::Region(s.into())])
}

/// `Task s x a`: a handle to a scoped task producing `a` with task
/// outcome `x`.
pub(crate) fn conc_task_ty(s: &str, x: &Ty, v: &Ty) -> Ty {
    Ty::Adt("Task".into(), vec![Ty::Region(s.into()), x.clone(), v.clone()])
}

/// The prelude `Result e a`: a task body's outcome and a join's result.
pub(crate) fn conc_result_ty(e: &Ty, v: &Ty) -> Ty {
    Ty::Adt(crate::known::RESULT.into(), vec![e.clone(), v.clone()])
}

// ---------------------------------------------------------------- the checker

/// A source reference resolved by the ordinary checker, including captured
/// macro operands. Kept outside interfaces/runtime ABI and requested only by
/// editor analysis; source spelling is never re-resolved in an insertion scope.
#[derive(Clone, Debug)]
pub(crate) struct CheckedReference {
    pub module: String,
    pub owner: String,
    pub name: String,
    pub kind: ast::RefKind,
    pub span: ast::Span,
}

/// A non-fatal diagnostic reported by the compiler (attribute
/// annotations specification section 2: the branch weight reports are
/// reported and the weights dropped, never a hard failure).
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// The report's code: `incomplete-branch-weights` or
    /// `incoherent-weights`.
    pub code: &'static str,
    pub message: String,
    pub span: ast::Span,
    /// `true` for an informational report (`incoherent-weights`: the
    /// weights remain usable), `false` for a warning-level one
    /// (`incomplete-branch-weights`: the weights are dropped).
    pub informational: bool,
}

/// One type's pending layout computation: its representation spec and
/// the source-order fields (attribute annotations specification section
/// 4). Recorded while declarations are collected and computed into ABI
/// layout facts once every type name is known.
#[derive(Clone)]
pub(crate) struct LayoutRecipe {
    pub module: String,
    pub name: String,
    pub repr: ReprSpec,
    /// `(field name, field type, erased, the field's layout modifiers)`.
    pub fields: Vec<(String, Ty, bool, ReprSpec)>,
    /// The number of constructors (a multi-constructor `data` has no
    /// field layout).
    pub ctor_count: usize,
    /// `true` for a `record`/`codata` declaration.
    pub record: bool,
    pub span: ast::Span,
}

/// One instance's obligation to satisfy a class law (primitive
/// operations and inline target IR specification section 5): the
/// instance's method units and type arguments beside the law's stated
/// equation. The checker evaluates the equation at sample values of the
/// law's binders where the machinery can decide it, and reports the
/// honest boundary where it cannot.
struct LawObligation {
    /// The instance's name (its declaration name).
    instance: String,
    module: String,
    class: String,
    law: String,
    /// The law's stated scheme (quantified over the class parameters and
    /// the law's own binders).
    scheme: Scheme,
    /// The class parameter names, positional against `class_args`.
    class_params: Vec<String>,
    /// The instance's type arguments for the class parameters.
    class_args: Vec<Ty>,
    /// The instance's method units, parallel to the class's method order.
    method_units: Vec<crate::core::GlobalId>,
    /// The instance's `using` constraints as (class, args) targets for
    /// its sub-dictionaries.
    subdicts: Vec<(String, Vec<Ty>)>,
    span: ast::Span,
}

/// One instance method's preservation obligation (refinement types
/// design: the container's instances transfer to the refinement exactly
/// when their methods PRESERVE the predicate, checked sample-wise by the
/// P18 law layer). A method whose result is the refined carrier must map
/// refined samples to refined values; the honest boundary reports `not
/// checked` where the machinery cannot decide it.
struct PresObligation {
    /// The instance's name and home (the manifest row's scope).
    instance: String,
    module: String,
    class: String,
    /// The method name (the preservation law's identity:
    /// `{refine}.preserve-{method}`).
    method: String,
    /// The method's unit.
    method_gid: crate::core::GlobalId,
    /// The refinement whose predicate the method must preserve.
    refine: String,
    /// The method's argument types (the preservation samples).
    arg_tys: Vec<Ty>,
    /// The instance's `using` constraints as (class, args) targets for
    /// its sub-dictionaries.
    subdicts: Vec<(String, Vec<Ty>)>,
    /// The class arguments (the dictionary resolution's targets).
    class_args: Vec<Ty>,
    /// The instance's method units, parallel to the class's method order.
    method_units: Vec<crate::core::GlobalId>,
    /// The refined result sits inside another type former (`f (R b)`):
    /// preservation is not decided through the former.
    nested: bool,
    span: ast::Span,
}

struct Checker {
    uni: Unifier,
    values: HashMap<String, Vec<ValueInfo>>,
    qualified: HashMap<(String, String), ValueInfo>,
    types: HashMap<String, Vec<TypeKind>>,
    qualified_types: HashMap<(String, String), TypeKind>,
    sigs: HashMap<String, Vec<SigInfo>>,
    /// Semantic dictionary families use the declaring signature identity.
    /// The bare-name registry above is only for lexical name lookup.
    class_sigs: HashMap<String, Vec<SigInfo>>,
    qualified_sigs: HashMap<(String, String), SigInfo>,
    /// Checked defaults by defining module, class, and method.
    method_defaults: HashMap<(String, String, String), crate::interface::MethodDefault>,
    class_law_contracts: HashMap<(String, String), Vec<crate::interface::ClassLawExport>>,
    instances: HashMap<String, Vec<usize>>,
    instance_list: Vec<InstanceInfo>,
    declaration_seed: Option<Vec<InstanceInfo>>,
    collecting_declarations: bool,
    // Ordinary headers that depend on names produced by staging are kept as
    // source AST, never as provisional checked types or evidence.
    defer_generated_headers: bool,
    deferred_headers: HashMap<(String, String), (ast::Decl, Vec<ast::Attribute>)>,
    declaration_bindings: HashMap<(ast::LocalBinding, ast::RefKind), (String, String)>,
    /// The strongly connected group of data declarations a positivity check
    /// covers, by `(module, name)` (core specification section 2, mutual
    /// inductive groups). A non-recursive declaration's group is itself.
    data_groups: HashMap<(String, String), Vec<String>>,
    provisional_declarations: bool,
    declaration_needs_headers: Cell<bool>,
    declaration_pending: Cell<usize>,
    fns: Vec<FnDef>,
    lam_counter: usize,
    /// `letrec` member lambdas awaiting their code generation unit:
    /// elaborated with their final unit id already chosen, finalized in
    /// `finish_fn` beside the enclosing definition's hoisted lambdas
    /// (surface grammar local amendments section 9).
    pending_rec: Vec<(crate::core::GlobalId, ECore)>,
    next_local: usize,
    param_counter: usize,
    /// Names the binders of converted nested quantifiers, which must be
    /// fresh so that substitution never captures them
    /// (design/rank-n-polymorphism.md section 1).
    forall_binder_counter: Cell<usize>,
    want_counter: usize,
    wants: Vec<Want>,
    /// Module currently being checked, used for unqualified resolution.
    current_module: String,
    /// Modules whose exports are in scope unqualified.
    visible: Vec<String>,
    /// The visible name sources of the checking module, in scope order
    /// (surface grammar local amendments section 11): its own names
    /// resolve first, then each import's contribution, then the
    /// prelude's.
    scope_views: Vec<ImportView>,
    /// Each module's imports as it sees them (surface grammar local
    /// amendments section 11), in declaration order.
    module_views: HashMap<String, Vec<ImportView>>,
    /// Each module's `export` items; a module with none exports its
    /// declared names as before.
    module_exports: HashMap<String, Vec<ast::ExportItem>>,
    /// Each module's `import data` bindings, in declaration order.
    module_data_imports: HashMap<String, Vec<ast::ImportData>>,
    /// The `import data` bindings' provenance (converter identity and
    /// file content hash), for the artifact manifest.
    data_imports: Vec<DataImport>,
    /// Each module's export surface: the name as its importers see it to
    /// the definition it names (a re-export names another module's
    /// definition).
    module_surface: HashMap<String, HashMap<String, SurfaceName>>,
    /// Top-level signature schemes, keyed by `(module, name)`.
    top_schemes: HashMap<(String, String), Scheme>,
    /// The runtime values introduced by a function's implicit telescope.
    /// Macro captures and equation checking must share their lexical handles.
    syntax_implicit_values: HashMap<(String, String), Vec<ast::Name>>,
    syntax_context_values: HashMap<(String, String), Vec<ast::Name>>,
    /// Top-level equations, keyed by module and name.
    top_eqs: Vec<(String, String, ast::Equation)>,
    /// Finished instance dictionaries.
    instance_defs: Vec<InstanceDef>,
    /// Instances awaiting body checking, keyed by module.
    instance_defs_pending: Vec<(String, ast::InstDecl)>,
    /// Instance heads awaiting conversion, in declaration order: an
    /// instance head names a class and a target type, and both may live
    /// in modules collected later (the prelude split's family), so the
    /// heads convert once every module is collected (the pass before
    /// `finish_module_params` and again after staging).
    pending_instance_heads: Vec<(String, ast::InstDecl)>,
    /// The embedded prelude family's collected modules: their instance
    /// heads drop (with their bodies) when a shadowing source module
    /// replaced the class's home without declaring the class, because
    /// such a program never had those instances (the source module IS
    /// the program's version of that home).
    embedded_modules: std::collections::HashSet<String>,
    /// Family modules an explicit source module shadowed: such a
    /// program's version of a home can lack the names the family's
    /// instance bodies expect (a stub replacing a class module), and a
    /// family instance that then fails to check drops instead of
    /// failing the program (the program never had that instance).
    shadowed_modules: std::collections::HashSet<String>,
    /// The instance whose method bodies are being checked: visible to
    /// resolution even when it is not `implicit`.
    self_instance: Option<usize>,
    /// Effects performed in the current checking scope.
    perf: EffRow,
    /// Fresh-name counter for memo regions.
    region_counter: Cell<usize>,
    /// The current module's prelude: the module imported implicitly, home
    /// of `List`, `Cons` and `Nil` for list literal and list pattern
    /// desugaring (design/base-as-a-package.md section 3). `build_scope`
    /// refreshes it from `module_preludes` for the module being checked.
    prelude_module: String,
    /// The prelude a module sees when its own package set none: the
    /// embedded facade today, `cinch/base`'s facade after the cut.
    prelude_default: String,
    /// Per-package preludes: a module whose package names a prelude gets
    /// that module injected instead of the default. Empty means every
    /// module sees the default.
    module_preludes: std::collections::HashMap<String, String>,
    /// Tuple constructor code generation units already synthesized, by
    /// arity.
    /// Integer literals awaiting type determination, in creation order.
    literal_pends: Vec<PendLit>,
    /// Collection literals' fold boundaries, in creation order (surface
    /// grammar local amendments section 10): the `LitFold` nodes index
    /// it, and the fold pass fills each record.
    literal_folds: Vec<LitFold>,
    /// The staging context the literal fold evaluates in (the D92
    /// evaluator's file and replay inputs).
    fold_scx: Option<StagingCx>,
    /// Implicit index arguments awaiting their solved values, in creation
    /// order (insert-and-solve, dependent core kernel design, section 2).
    idx_pends: Vec<IdxPend>,
    /// Kind constraints collected while converting one scheme's type:
    /// un-annotated `forall` binders are classified by where they are used
    /// (a type parameter, an index value, or a region).
    kind_constraints: RefCell<Vec<(String, ParamKind)>>,
    /// Logical definitions (`spec` and `proof` declarations): the erased
    /// total fragment, checked for termination and never emitted
    /// (core specification sections 3 and 8).
    logical: std::collections::HashSet<(String, String)>,
    /// `corec` declarations: definitions whose recursion must be
    /// productive (core language specification section 9).
    corec: std::collections::HashSet<(String, String)>,
    /// Whether the target profile admits runtime code generation
    /// (`eval` of a typed quote; supercompilation section 7: a profile
    /// MAY forbid it).
    allow_eval: bool,
    /// The target's `Usize`/`Isize` width in bits (primitive types
    /// specification section 2: widths key on the target, never the
    /// build host).
    usize_bits: u32,
    /// `eval` use sites (module and function), recorded as build-visible
    /// assumptions beside the foreign ones (supercompilation section 7).
    eval_sites: Vec<(String, String)>,
    /// The generated WIT adapters in scope (WebAssembly and WASI
    /// specification section 3): their raw bindings are registered as
    /// foreign plans with canonical ABI glue.
    wit_adapters: Vec<crate::interface::WitAdapter>,
    /// The logical definition being checked (its references are recorded
    /// for the termination and cycle checks).
    current_logical: Option<(String, String)>,
    /// Logical references recorded while checking one definition.
    pending_refs: Vec<(String, String)>,
    /// Each logical definition's logical references, for the mutual
    /// recursion check (a looping function must not inhabit a `Prop`).
    logical_refs: HashMap<(String, String), Vec<(String, String)>>,
    /// Checked bodies retained for termination checks of computations inside
    /// erased evidence. A pure arrow alone grants no termination authority.
    erasure_bodies: HashMap<GlobalId, (Vec<LocalId>, ECore)>,
    pending_runtime_arithmetic: RefCell<Vec<Vec<ArithmeticCertificate>>>,
    used_runtime_arithmetic: RefCell<Vec<(ArithmeticCertificate, ast::Span, String)>>,
    runtime_index_readings: Vec<RuntimeIndexReading>,
    comparison_facts: HashMap<LocalId, Vec<ComparisonCertificate>>,
    used_comparisons: RefCell<Vec<(ComparisonCertificate, ast::Span, String)>>,
    pending_erasure_checks: Vec<(String, ast::Span, ECore)>,
    erasure_checks: Vec<(String, ast::Span, ECore)>,
    /// Top-level bindings recorded for the eager-reference-cycle check
    /// (the guarded-letrec rule extended to the top level, where the
    /// `letrec` fixpoint discipline does not reach): `(unit id, name,
    /// definition span, is a value)` in check order.
    top_binds: Vec<(GlobalId, String, ast::Span, bool)>,
    /// The reference edges `(owner index, referenced, guarded)` each
    /// recorded binding's body walk found (guard counts the suspension
    /// positions entered: lambda bodies and `delay` arguments).
    top_edges: Vec<(usize, GlobalId, bool)>,
    /// Set while a top-level definition's equations are checked: only
    /// `check_bodies`' ordinary definitions are top-level values (the
    /// instance methods and staged units checked through the same entry
    /// are not).
    record_top_value: bool,
    /// The pattern parameters of the definition being checked and the
    /// subcomponent relation its pattern matching established, for the
    /// structural termination check of logical definitions.
    smaller: HashMap<LocalId, LocalId>,
    /// Each module's imported module names (interface metadata).
    module_imports: HashMap<String, Vec<String>>,
    /// Modules with a parameter block (`module M (x :: T, ...) where`,
    /// module parameters as module arguments): their values are reached
    /// only through instantiation units.
    param_modules: std::collections::HashSet<String>,
    /// The parameter declarations as parsed, by module; converted by
    /// [`Checker::finish_module_params`] once every name is collected.
    mod_param_asts: HashMap<String, Vec<ast::ParamDecl>>,
    /// Converted module parameters, by module.
    module_params: HashMap<String, Vec<ModParam>>,
    /// The definition being checked: its module-parameter locals, in
    /// declaration order (the leading runtime parameters of every unit of
    /// a parameterized module), with the parameter's type.
    cur_mod_params: Vec<(String, LocalId, Ty)>,
    /// Instantiation declarations (`import M (args) as N`) awaiting their
    /// owner's body pass, by owner module.
    unit_decls: Vec<(String, ast::Import)>,
    /// Processed instantiation units, in declaration order.
    units: Vec<UnitRec>,
    /// Declared instantiation unit names (globally unique: they name code
    /// generation units beside module names).
    unit_names: std::collections::HashSet<String>,
    /// Every collected module name, for unit-name collision checks.
    module_names: std::collections::HashSet<String>,
    /// Instance dictionary units of parameterized modules' instances, by
    /// `(module, instance name)`: the wrappers' method forwarding reads
    /// them at unit emission.
    param_inst_defs: HashMap<(String, String), crate::core::InstanceDef>,
    /// Each module's own fixity declarations, for its interface.
    module_fixities: Vec<(String, crate::interface::FixityExport)>,
    /// Source binders retain declaration order and implicitness for
    /// interface kind metadata; the converted TypeKind supplies kinds.
    interface_binders: HashMap<(String, String), Vec<ast::TypeBinder>>,
    /// Captured while checking each source signature: the contract text
    /// keeps the binders and references the source wrote, which the
    /// checked scheme alone does not carry.
    interface_source_schemes: HashMap<(String, String), DefinitionScheme>,
    interface_method_schemes: HashMap<GlobalId, Scheme>,
    interface_total_bodies: std::collections::BTreeMap<String, crate::total::PendingDefinition>,
    interface_guarded_bodies: std::collections::BTreeMap<String, (usize, PendingGuardedNat)>,
    interface_checked_bodies: Option<crate::total::CheckedGraph>,
    /// Each module's foreign declarations, for its interface.
    module_foreign: Vec<(String, crate::interface::ForeignExport)>,
    /// Real foreign binding contracts by unit name (foreign function
    /// interface specification section 2), for the program's ABI maps.
    foreign_plans: Vec<(String, crate::core::ForeignPlan)>,
    /// Real foreign bindings awaiting contract construction (their
    /// classification reads the ABI layout table, computed after the
    /// declaration pass).
    pending_foreign: Vec<(String, crate::ast::ForeignDecl, Scheme, ForeignAbi)>,
    /// Every binder's type, for its representation class at lowering
    /// (local ids are unique across the whole check, so entries never
    /// collide between definitions).
    local_tys: HashMap<LocalId, Ty>,
    /// Lexical value bindings at the derivation of an erased proof. A later
    /// source-name shadow cannot change which values its statement names.
    proof_value_scopes: HashMap<LocalId, HashMap<String, LocalId>>,
    /// Direct strict-let aliases preserve the same runtime value identity.
    value_aliases: HashMap<LocalId, LocalId>,
    /// Typed pattern equalities for the current erased proof branch. Subjects
    /// and fields use lexical identities, so a shadow cannot retarget them.
    proof_pattern_equalities: HashMap<LocalId, ITerm>,
    proof_pattern_values: HashSet<LocalId>,
    /// Runtime forall parameters already represented by identity in types.
    quantified_value_locals: HashSet<LocalId>,
    /// Binders whose class is fixed without a type: dictionaries,
    /// capability evidence and markers.
    local_cls: HashMap<LocalId, crate::repr::Class>,
    /// Where the elaboration created each binder (debug builds only), so
    /// a binder lowering finds untyped names its source.
    local_origin: HashMap<LocalId, &'static std::panic::Location<'static>>,
    /// The rigid names of the type parameters of the definition being
    /// checked, in class-vector order (its hoisted lambdas and `letrec`
    /// members share them).
    cur_tparams: Vec<String>,
    /// The regions the enclosing signatures quantify: a caller names each
    /// one and supplies the place it lives in, so it outlives every body
    /// checked under it. `borrow` builds a value in the body and cannot
    /// carry one (ownership and usage design, section 4); nested
    /// definitions inherit the enclosing entries.
    caller_regions: Vec<String>,
    /// An instance method's parameter list, set by the instance check for
    /// the method's own `check_equations` call: the instance's parameters
    /// then the method's own.
    method_tparams: Option<Vec<String>>,
    /// Let-generalized locals and their quantified metavariables, in the
    /// scheme's parameter order.
    gen_locals: HashMap<LocalId, Vec<crate::types::MetaId>>,
    /// The definition of each let-generalized local: the expression its
    /// value is (for a destructured component, the bound value with the
    /// projections down to the component; for a `letrec` member, its
    /// whole group's bindings). Each use elaborates to a copy of it with
    /// the quantified metavariables instantiated ([`Checker::gen_copy`]),
    /// so every copy has one concrete representation and the local can
    /// run at a raw type at one use and a counted one at another.
    gen_defs: HashMap<LocalId, ECore>,
    /// Use counts accumulated for the definition being checked.
    uses: HashMap<LocalId, UseNode>,
    /// Usage obligations of the definition's binders.
    obligations: HashMap<LocalId, Obligation>,
    /// Lazy-position nesting: lambda and thunk bodies.
    lam_depth: usize,
    /// Lazy-position nesting: memo cell bodies.
    memo_depth: usize,
    /// `let`/`letrec` nesting being elaborated, bounded by
    /// [`LET_DEPTH_LIMIT`] (`in_let`).
    let_depth: usize,
    /// Memo-group regions currently in scope (region escape checking).
    active_regions: Vec<String>,
    /// One-shot frames: task bodies and `withScope` bodies being
    /// elaborated (concurrency specification section 2).
    oneshot_frames: Vec<OneshotFrame>,
    /// Structured scopes whose bodies are being elaborated.
    scope_frames: Vec<ScopeFrame>,
    /// Task-capture sites of the definition being checked, validated at
    /// its end (concurrency specification section 3).
    task_captures: Vec<TaskCapture>,
    /// Parallel-callback capture sites of the definition being checked,
    /// validated at its end (`design/parallel-evaluation.md` section 2.3).
    par_captures: Vec<ParCapture>,
    /// The definition's type-parameter names in class-vector order, set
    /// before its body is elaborated: a parallel call records its result
    /// blocks' representation classes, and a type parameter's class is its
    /// index here.
    body_tparams: Vec<String>,
    /// Free locals of known closures before local aliases erase their shape.
    local_closure_captures: HashMap<LocalId, Vec<LocalId>>,
    /// Source definitions whose entire body directly forwards its parameters
    /// to the task kernel. The call site checks their capture obligation.
    task_forwarders: HashSet<GlobalId>,
    checking_task_forwarder: bool,
    /// Record-projection path accumulated for a `Proj` operand place.
    place_path: Vec<String>,
    /// User-declared effects by name (`effect L where ...`).
    effects: HashMap<String, EffectInfo>,
    /// Definition sites, independent of their public spelling and row label.
    effect_bindings: HashMap<(String, String), String>,
    /// Codata types by `(module, type name)` (codata and guardedness
    /// design, section 2): a record of delayed observations, built only by
    /// its observed construction form and eliminated only by observation.
    codata_types: HashSet<(String, String)>,
    /// Effect operations by `(module, name)`, for the dispatch gate.
    effect_ops: HashMap<(String, String), ()>,
    /// Continuation closures currently being elaborated (usage counting).
    /// Frames stay recorded after their body is done (a continuation value
    /// refers back to its frame); `cont_depth` counts the enclosing ones.
    conts: Vec<ContFrame>,
    /// Nesting depth of continuation bodies being elaborated.
    cont_depth: usize,
    /// Handler extents of enclosing `handle` expressions (operation
    /// dispatch is lexical: see `handle_expr`).
    extents: Vec<HandlerExtent>,
    /// The definition's selected sequencing providers (decision D22): a
    /// `using` binder with a provider class head selects the provider for
    /// its `do` blocks.
    seq_binders: Vec<ClassApp>,
    /// Continuation currently being elaborated at each continuation
    /// position, for `CpsWrap` nodes.
    current_cont: Vec<Cont>,
    /// Staging (P9): true while checking compile-time code (`stage`
    /// definitions, macros, `generate` blocks, and quotation splices).
    compile_time: bool,
    /// Preserve the actual type-directed bindings of checked quotation
    /// nodes. Their clones share immutable resolution cells; source tokens
    /// and raw captured spellings do not carry this authority.
    record_quote_bindings: bool,
    /// The source that requests the current proof goal. Captured proof
    /// syntax keeps its own origin; goal-directed diagnostics use this site.
    proof_goal_site: Option<ast::Span>,
    editor_references: RefCell<Option<Vec<CheckedReference>>>,
    editor_modules: HashSet<String>,
    /// The instance-search buckets the checks so far consulted, as
    /// `(declaration, class, head symbol)` (design/compilation-speed.md
    /// section 3, "negative lookups are dependencies"). A row is recorded
    /// whether or not the search found an instance: the miss is the
    /// dependency that has to invalidate when a matching instance is
    /// added, and it is the only place that fact exists.
    instance_searches: RefCell<Vec<(String, String, String)>>,
    /// The declaration whose check is running, for the rows above.
    current_declaration: String,
    scoped_type_env: RefCell<Vec<Env>>,
    /// Checked source-header value binders, before runtime locals exist.
    /// Captured references use their authoritative Env path instead.
    index_carriers: RefCell<Vec<(String, Ty)>>,
    /// Lambda references select opaque source handles, independently of the
    /// alpha-convertible spelling used by the kernel's bound-variable IR.
    index_lambda_binders: RefCell<Vec<IndexLambdaBinder>>,
    index_lambda_counter: Cell<usize>,
    index_header_context: RefCell<Vec<Vec<(String, ClassApp)>>>,
    index_dictionary_scope: RefCell<Vec<Vec<(IndexDictionaryBinding, ClassApp)>>>,
    law_index_context: RefCell<Option<Vec<(String, ClassApp)>>>,
    /// Quantified binders of the definition currently checking staged Code.
    /// These are native rigidification identities, never source spellings
    /// collected from a generated fragment.
    code_quantifiers: Scheme,
    /// Lexical type/row/region names map to this definition's actual rigid
    /// identities. A nested definition saves and restores this scope.
    body_annotation_scope: StageTypeBindings,
    /// Class defaults are written with the class/method parameter names;
    /// their instance copies retain those aliases after specialization.
    method_annotation_aliases: Option<HashMap<String, Ty>>,
    /// Checked compile-time references and their full use-site instantiations.
    /// These private identities never enter runtime interfaces or object data.
    stage_calls: RefCell<HashMap<String, StageCall>>,
    stage_definition_scopes: HashMap<String, (Scheme, Scheme)>,
    /// Staging: the quotation template's hole types, indexed by
    /// `SpliceHole`.
    hole_tys: Vec<Ty>,
    /// Staging: `stage` and `macro` definitions by `(module, name)`: the
    /// compile-time-only fragment.
    staging: std::collections::HashSet<(String, String)>,
    /// Staging: macro metadata by `(module, name)`.
    macros: std::collections::HashMap<(String, String), MacroInfo>,
    /// Staging: elaborated compile-time function units (stage definitions,
    /// macros and generate blocks), excluded from the runtime program.
    staging_fns: std::collections::HashMap<String, Vec<crate::core::FnDef>>,
    /// Staging: `generate do` blocks, in collection order.
    generate_blocks: Vec<GenBlock>,
    /// Staging: definitions already checked on demand (their Core is
    /// available to compile-time evaluation before the final pass).
    checked_early: std::collections::HashSet<(String, String)>,
    /// Staging: the captured inputs of the build (file and subprocess
    /// reads), recorded for the artifact manifest's build identity.
    stage_inputs: Vec<StageInput>,
    /// Staging: declarations emitted by executed generators, keyed by the
    /// module they belong to.
    stage_emitted: std::collections::HashMap<String, Vec<(crate::ast::Decl, crate::ast::Span)>>,
    /// Staging: the build mode (refresh, replay, live).
    stage_mode: StageMode,
    /// Module to source file, for expansion provenance.
    module_files: std::collections::HashMap<String, String>,
    /// Each module's `import stage` names (compile-time scope only).
    module_stage_imports: std::collections::HashMap<String, Vec<String>>,
    /// Staging: attribute names declared by macros (`name` -> declaring
    /// macro and its declaration span), for the collision check.
    macro_attr_names: std::collections::HashMap<String, (String, ast::Span)>,
    /// Non-fatal diagnostics (the branch weight reports), in report
    /// order (attribute annotations specification section 2).
    diags: Vec<Diagnostic>,
    /// Hint attributes recorded per function id (attribute annotations
    /// specification sections 3 and 5); erased before the contract
    /// projection.
    fn_hints: HashMap<String, crate::core::FnHints>,
    /// Layout computation pending per type (attribute annotations
    /// specification section 4.3).
    layout_recipes: HashMap<(String, String), LayoutRecipe>,
    /// Computed ABI layout facts per type, keyed `(module, name)`.
    type_layouts: HashMap<(String, String), crate::interface::AbiLayout>,
    /// P20: the safety assertion set recorded per definition (attribute
    /// annotations specification section 10).
    safety_attrs: HashMap<String, SafetySet>,
    /// P20: module-scope safety state (`#[profile "jpl"]`,
    /// `#[deny-warnings]` on the module header).
    module_safety: HashMap<String, ModuleSafety>,
    /// P20: the verified per-definition records for the group checks.
    safety_records: Vec<SafetyRecord>,
    /// P20: satisfied assertion facts per definition, for the
    /// interface's contract rows (decision D112).
    safety_facts: HashMap<String, Vec<String>>,
    /// P20: the runtime `decide` sites of the definition being checked
    /// (proof obligations do not count).
    cur_runtime_decides: Vec<ast::Span>,
    /// P20: `#[deny-warnings]` on the definition being checked.
    cur_def_deny: bool,
    /// P20: the `jpl` target profile in force (decision D114).
    jpl_target: bool,
    /// P20: the user modules (the profiled program; the prelude family
    /// is not flight code).
    jpl_user_modules: HashSet<String>,
    /// Temporal contracts awaiting the named definition's elaboration
    /// (core specification section 10), keyed by `Module.name`: collected
    /// at declaration time and verified with the definition.
    pending_contracts: HashMap<(String, String), Vec<ast::ContractDecl>>,
    /// The verified temporal contract facts per definition, for the
    /// interface's contract rows (like the P20 assertion facts).
    contract_facts: HashMap<String, Vec<crate::interface::ContractFact>>,
    /// The verified temporal contract rows for the artifact manifest
    /// (core specification section 10).
    temporal_rows: Vec<TemporalRow>,
    /// The temporal contracts pending on the definition being elaborated:
    /// prepared before its body checks (the parameter scope) and verified
    /// after constraint solving (the write log's regions resolve there).
    cur_temporal: Option<TemporalPending>,
    /// Rewrite declarations awaiting checking, in source order (modules
    /// and compilation specification section 5).
    pending_rewrites: Vec<(String, ast::RewriteDecl)>,
    /// `prim` declarations awaiting their deferred pass (primitive
    /// operations and inline target IR specification sections 2-6):
    /// capability conditions, justifications and body selection need
    /// the whole program's names and the target profile.
    pending_prims: Vec<(String, ast::PrimDecl, Scheme)>,
    /// Checked `prim` plans in declaration order (core specification
    /// section 11's manifest).
    prims: Vec<crate::core::PrimPlan>,
    /// The compile target profile: every atom of a `prim` body's `when`
    /// condition is read off this profile, never off the build host
    /// (primitive operations and inline target IR specification
    /// section 13; platform proofs and SIMD specification section 3 for
    /// the `Available` evidence). `None` before a driver sets one, and
    /// body selection then refuses rather than guessing a target.
    target: Option<crate::core::TargetProfile>,
    /// The build's two profiles (design/base-as-a-package.md, "Host and
    /// target are different questions"). The checker holds both so that a
    /// conditional position's context is a stated fact rather than an
    /// assumption: [`crate::core::condition_context`] is the one place
    /// that decision lives. `None` before a driver sets one.
    build: Option<crate::core::BuildProfiles>,
    /// Class law obligations collected per instance, checked sample-wise
    /// after every module's bodies (primitive operations and inline
    /// target IR specification section 5).
    law_obligations: Vec<LawObligation>,
    /// Instance methods' preservation obligations over refinements
    /// (checked sample-wise beside the class laws).
    pres_obligations: Vec<PresObligation>,
    /// The class law check rows for the artifact manifest: the honest
    /// record of what the sample-wise check decided.
    law_rows: Vec<crate::core::LawRow>,
    /// Upgrade declarations awaiting checking, in source order (runtime
    /// evolution specification section 3): the migration field checking
    /// runs after every module's records and functions are known.
    pending_upgrades: Vec<(String, ast::UpgradeDecl)>,
    /// Schema compatibility evidence from checked upgrades, for the
    /// module's interface (runtime evolution specification section 11),
    /// keyed by module.
    upgrade_evidence: Vec<(String, crate::interface::UpgradeEvidence)>,
    /// Checked rewrite rules, in declaration order.
    rewrites: Vec<crate::core::RewriteRule>,
    /// Evidence invalidation over mutable state (core specification
    /// section 5): the ordered log of writes to mutable locations within
    /// the definition being checked. Each entry is the written location
    /// (a `Mut` borrow's region) or `None` for an opaque write (an FFI /
    /// `RawPtr` write or `eval`), with the write's span for errors.
    /// Temporal contracts read the same log as the per-step transition
    /// record (core specification section 10).
    ev_writes: Vec<EvWrite>,
    /// Evidence derivation points: a `Prop`-typed local to the write-log
    /// length at its derivation and the derivation span (the site an
    /// invalidation error names).
    ev_derivs: HashMap<LocalId, (usize, ast::Span)>,
    /// Evidence uses recorded during elaboration (a stale check is deferred
    /// until regions unify, since a call's effect-row region may only
    /// resolve during constraint solving). Each is the evidence local, the
    /// mutable locations its statement reads, and the use span.
    /// Type conversion can share proof discharge without changing inference;
    /// its evidence uses enter this same deferred staleness journal.
    ev_uses: RefCell<Vec<(LocalId, Vec<String>, ast::Span)>>,
    /// A branch fact's in-condition window: the write-log range between
    /// its test and the end of the condition that holds it (`p || g m`
    /// tests `p`, then `g` may write before the branch is taken). Writes
    /// there count against the fact like writes after its binding; writes
    /// in sibling branches elaborated between the condition and the
    /// binding do not.
    ev_pre: HashMap<LocalId, (usize, usize)>,
    /// What each match column's value is known to be about (a `case`
    /// scrutinee read as a predicate, an index term, or a three-way
    /// comparison), so each branch of the decision tree binds the fact
    /// its test establishes.
    col_readings: HashMap<LocalId, Reading>,
    /// A `Bool` match's scrutinee test held back from its `let` so the
    /// `Bool` column branches on the test itself: `case c of True -> a;
    /// False -> b` compiles to exactly the `if c then a else b` branch.
    /// Present only while no pattern binds the scrutinee by name.
    inline_cols: HashMap<LocalId, ECore>,
    /// Refinement transfers recorded at resolved wants (refinement types
    /// design, "composability"): checked in the law layer after every
    /// module's bodies are known.
    transfer_obligs: RefCell<Vec<TransferOblig>>,
    /// The (dictionary, point, method) keys already recorded: one check
    /// per transfer however many uses resolve to it.
    transfer_seen: RefCell<HashSet<(String, String, String)>>,
    /// Method and function applications whose instance or signature
    /// carries refinement obligations: the arguments and the evidence in
    /// scope at the call, for the discharge after resolution (the `Dec`
    /// evidence path).
    prop_calls: Vec<PropCall>,
    /// Nonzero while elaborating an application's callee: an
    /// obligation-bearing value is legal exactly there (its obligations
    /// discharge at its call's arguments).
    app_callee_depth: usize,
    /// The checked definition's own `using` obligations over its
    /// parameters, with the parameters' column locals: each holds in
    /// the body (every caller discharged it), so the equations' rows
    /// bind it as hidden evidence over the names their patterns give
    /// those locals. Positional binders `$a{i}` index the locals.
    param_facts: Vec<(PTerm, Vec<LocalId>)>,
    /// The value-parameter locals of the definition being checked, so a
    /// result refinement naming parameter `i` (positional `$a{i}`) reads
    /// over the name each row gives it (`row_result`).
    result_params: Vec<LocalId>,
}

/// A `generate do` block collected for compile-time execution (staging).
struct GenBlock {
    module: String,
    stmts: Vec<crate::ast::DoStmt>,
    span: ast::Span,
}

/// A `macro` definition's compile-time metadata (staging).
#[derive(Clone, Debug)]
struct MacroInfo {
    /// Parameter count: the token-tree groups a call must supply.
    arity: usize,
    /// Attribute names the macro declares as its own (attribute
    /// annotations specification section 6).
    owned_attrs: Vec<String>,
    /// The macro's definition-site span, for expansion provenance.
    span: ast::Span,
}

/// One `import data` binding's provenance (surface grammar local
/// amendments section 11): the converter identity and the file's content
/// hash, recorded in the artifact manifest. The import's contract covers
/// the conversion, not the file bytes; the manifest records hashes, never
/// contents.
#[derive(Clone, Debug)]
pub struct DataImport {
    pub module: String,
    pub name: String,
    pub file: String,
    pub converter: String,
    pub hash: String,
}

/// One import as the importing module sees it (surface grammar local
/// amendments section 11).
#[derive(Clone, Debug)]
struct ImportView {
    /// The imported module's real name.
    target: String,
    /// The qualifier its members are seen under (`import X as Y` renames
    /// the module alias).
    alias: String,
    /// `import stage M`: compile-time-only visibility.
    stage: bool,
    /// `import M (args) as N`: the module's names stay out of scope
    /// (they are reached through the unit `N`).
    applied: bool,
    /// `import X as Y`: members are visible only through the alias.
    alias_only: bool,
    /// `import X (a as b, c)`: local name to the source's name; `None`
    /// imports the whole surface under its own names.
    members: Option<Vec<(String, String)>>,
    /// `import X hiding (a, b)`: names of the source's surface this view
    /// leaves out. Only a whole-surface view (`members` is `None`) has
    /// any.
    hidden: Vec<String>,
}

impl ImportView {
    /// The whole surface of `target` under its own names: the prelude's
    /// view, and an import that selects and hides nothing.
    fn whole(target: String, alias: String) -> ImportView {
        ImportView { target, alias, stage: false, applied: false, alias_only: false, members: None, hidden: Vec::new() }
    }

    /// Whether the view leaves the source's name `name` out.
    fn hides(&self, name: &str) -> bool {
        self.hidden.iter().any(|h| h == name)
    }

    /// The source's name for this view's local name, or `None` when the
    /// view does not make it visible.
    fn source_name<'a>(&'a self, local: &'a str) -> Option<&'a str> {
        match &self.members {
            None => (!self.hides(local)).then_some(local),
            Some(ms) => ms
                .iter()
                .find(|(l, _)| l == local)
                .map(|(_, s)| s.as_str()),
        }
    }
}

/// One export-surface entry (surface grammar local amendments section
/// 11): the definition a surface name names. A re-export names another
/// module's definition under the exporting module's surface name.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SurfaceName {
    module: String,
    name: String,
}

/// One collection literal's fold boundary (surface grammar local
/// amendments section 10): which literals the compile-time fold made
/// constant values and which stayed runtime `from-list`/`from-map`
/// calls. The list is the compiler report item; the artifact manifest
/// records it beside the staging identity.
#[derive(Clone, Debug)]
pub struct LitFold {
    /// The code generation unit the literal lives in (the enclosing
    /// definition, filled when the fold runs).
    pub unit: String,
    /// The literal's source site.
    pub site: String,
    /// `"list"` for `[a, b, c]`, `"map"` for `{k => v}`.
    pub kind: String,
    /// Whether the construction folded to a constant value.
    pub folded: bool,
    /// Checked result type retained while reifying constructor constants.
    result_ty: Ty,
}

/// How a build treats compile-time I/O (modules specification section 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageMode {
    /// Execute external reads and record replayable input snapshots.
    Refresh,
    /// Use declared captured inputs; reject missing or incompatible ones.
    Replay,
    /// Perform current reads and record provenance without cache claims.
    Live,
}

/// One captured compile-time input (modules specification section 10: the
/// build identity includes declared input hashes). A secret records its
/// name only: values never enter snapshots.
#[derive(Clone, Debug)]
pub struct StageInput {
    /// Snapshot name (`Stage.snapshot`'s label).
    pub snapshot: String,
    /// `file`, `subprocess`, or `secret`.
    pub kind: String,
    /// The path, tool identity, or secret name.
    pub name: String,
    /// Content hash of the captured input (empty for secrets).
    pub hash: String,
    /// Recorded content for replay (empty for secrets).
    pub captured: String,
    /// Set when the read ran in `live` mode.
    pub live: bool,
}

fn fn_id(module: &str, name: &str) -> GlobalId {
    GlobalId(format!("{module}.{name}"))
}

/// Two checked declarations of the same primitive denote the same index
/// operation. A similarly named ordinary definition keeps its own identity.
fn value_index_id(value: &ValueInfo) -> String {
    match value.kind {
        ValueKind::Prim(PrimOp::Bytes(BytesOp::Len)) => BYTE_LENGTH_INDEX.to_string(),
        _ => fn_id(&value.module, &value.name).0,
    }
}

/// Does `value` carry the identity `head` names (`module.name`)?
///
/// The index lookups search the qualified table for one head, so building
/// each candidate's identity would allocate once per candidate visited;
/// joining module and name is byte equality, compared here without it.
fn value_names_head(value: &ValueInfo, head: &str) -> bool {
    match value.kind {
        ValueKind::Prim(PrimOp::Bytes(BytesOp::Len)) => head == BYTE_LENGTH_INDEX,
        _ => {
            let module = value.module.as_str();
            head.len() == module.len() + 1 + value.name.len()
                && head.starts_with(module)
                && head.as_bytes()[module.len()] == b'.'
                && head[module.len() + 1..] == value.name
        }
    }
}

fn index_value_label(value: &ValueInfo) -> String {
    match value.kind {
        ValueKind::Prim(PrimOp::Bytes(BytesOp::Len)) => "byte-length".into(),
        _ => fn_id(&value.module, &value.name).0,
    }
}

/// The named functions a proposition's index terms apply, with the
/// argument count of each use (a bare reference is arity zero).
fn index_heads(p: &PTerm, out: &mut Vec<(String, usize)>) {
    fn term(t: &ITerm, out: &mut Vec<(String, usize)>) {
        let mut push = |h: &str, k: usize| {
            if !out.iter().any(|(g, j)| g == h && *j == k) {
                out.push((h.to_string(), k));
            }
        };
        match t {
            ITerm::Fn(h) => push(h, 0),
            ITerm::Inst(call) => {
                if let IndexHead::Function(head) = &call.head { push(head, 0); }
                for argument in call.terms() { term(argument, out); }
            },
            ITerm::App(f, args) => {
                match &**f {
                    ITerm::Fn(h) => push(h, args.len()),
                    ITerm::Inst(call) => {
                        if let IndexHead::Function(head) = &call.head { push(head, args.len()); }
                        for argument in call.terms() { term(argument, out); }
                    },
                    other => term(other, out),
                }
                args.iter().for_each(|a| term(a, out));
            }
            ITerm::Bin(_, a, b) => {
                term(a, out);
                term(b, out);
            }
            ITerm::Lam(_, b) => term(b, out),
            ITerm::Var(_) | ITerm::LocalValue(_) | ITerm::Evidence(_) | ITerm::Num(_) | ITerm::MVar(_) => {}
        }
    }
    match p {
        PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
            term(a, out);
            term(b, out);
        }
        PTerm::Not(q) => index_heads(q, out),
        PTerm::And(a, b) | PTerm::Or(a, b) => {
            index_heads(a, out);
            index_heads(b, out);
        }
        PTerm::Pred(_, a) => term(a, out),
        PTerm::Lit(_) | PTerm::Var(_) | PTerm::MVar(_) => {}
    }
}

/// Substitute row variables (by tail) and region names inside an effect row.
fn subst_row(
    r: &EffRow,
    rows: &HashMap<String, RowTail>,
    regions: &HashMap<String, String>,
) -> EffRow {
    EffRow {
        labels: r
            .labels
            .iter()
            .map(|l| EffLabel {
                head: l.head.clone(),
                arg: l.arg.as_ref().map(|a| regions.get(a.as_str()).cloned().unwrap_or_else(|| a.to_string()).into()),
            })
            .collect(),
        tail: match &r.tail {
            RowTail::Rigid(t) => rows.get(t.as_str()).cloned().unwrap_or(RowTail::Rigid(*t)),
            other => other.clone(),
        },
    }
}

impl Checker {
    fn new() -> Self {
        Checker {
            uni: Unifier::new(),
            values: HashMap::new(),
            qualified: HashMap::new(),
            types: HashMap::new(),
            qualified_types: HashMap::new(),
            sigs: HashMap::new(),
            class_sigs: HashMap::new(),
            qualified_sigs: HashMap::new(),
            method_defaults: HashMap::new(),
            class_law_contracts: HashMap::new(),
            instances: HashMap::new(),
            instance_list: Vec::new(),
            declaration_seed: None,
            collecting_declarations: false,
            defer_generated_headers: false,
            deferred_headers: HashMap::new(),
            declaration_bindings: HashMap::new(),
            data_groups: HashMap::new(),
            provisional_declarations: false,
            declaration_needs_headers: Cell::new(false),
            declaration_pending: Cell::new(0),
            fns: Vec::new(),
            lam_counter: 0,
            pending_rec: Vec::new(),
            next_local: 0,
            param_counter: 0,
            forall_binder_counter: Cell::new(0),
            want_counter: 0,
            wants: Vec::new(),
            current_module: String::new(),
            visible: Vec::new(),
            scope_views: Vec::new(),
            module_views: HashMap::new(),
            module_exports: HashMap::new(),
            module_data_imports: HashMap::new(),
            data_imports: Vec::new(),
            module_surface: HashMap::new(),
            top_schemes: HashMap::new(),
            syntax_implicit_values: HashMap::new(),
            syntax_context_values: HashMap::new(),
            top_eqs: Vec::new(),
            instance_defs: Vec::new(),
            instance_defs_pending: Vec::new(),
            pending_instance_heads: Vec::new(),
            embedded_modules: std::collections::HashSet::new(),
            shadowed_modules: std::collections::HashSet::new(),
            self_instance: None,
            perf: EffRow::pure(),
            region_counter: Cell::new(0),
            prelude_module: String::new(),
            prelude_default: String::new(),
            module_preludes: std::collections::HashMap::new(),
            literal_pends: Vec::new(),
            literal_folds: Vec::new(),
            fold_scx: None,
            idx_pends: Vec::new(),
            kind_constraints: RefCell::new(Vec::new()),
            logical: std::collections::HashSet::new(),
            corec: std::collections::HashSet::new(),
            allow_eval: true,
            usize_bits: 64,
            eval_sites: Vec::new(),
            wit_adapters: Vec::new(),
            current_logical: None,
            pending_refs: Vec::new(),
            logical_refs: HashMap::new(),
            erasure_bodies: HashMap::new(),
            pending_runtime_arithmetic: RefCell::new(Vec::new()),
            used_runtime_arithmetic: RefCell::new(Vec::new()),
            runtime_index_readings: Vec::new(),
            comparison_facts: HashMap::new(),
            used_comparisons: RefCell::new(Vec::new()),
            pending_erasure_checks: Vec::new(),
            erasure_checks: Vec::new(),
            top_binds: Vec::new(),
            top_edges: Vec::new(),
            record_top_value: false,
            smaller: HashMap::new(),
            module_imports: HashMap::new(),
            param_modules: std::collections::HashSet::new(),
            mod_param_asts: HashMap::new(),
            module_params: HashMap::new(),
            cur_mod_params: Vec::new(),
            unit_decls: Vec::new(),
            units: Vec::new(),
            unit_names: std::collections::HashSet::new(),
            module_names: std::collections::HashSet::new(),
            param_inst_defs: HashMap::new(),
            module_fixities: Vec::new(),
            interface_binders: HashMap::new(),
            interface_source_schemes: HashMap::new(),
            interface_method_schemes: HashMap::new(),
            interface_total_bodies: std::collections::BTreeMap::new(),
            interface_guarded_bodies: std::collections::BTreeMap::new(),
            interface_checked_bodies: None,
            module_foreign: Vec::new(),
            foreign_plans: Vec::new(),
            pending_foreign: Vec::new(),
            local_tys: HashMap::new(),
            proof_value_scopes: HashMap::new(),
            value_aliases: HashMap::new(),
            proof_pattern_equalities: HashMap::new(),
            proof_pattern_values: HashSet::new(),
            quantified_value_locals: HashSet::new(),
            local_cls: HashMap::new(),
            local_origin: HashMap::new(),
            cur_tparams: Vec::new(),
            caller_regions: Vec::new(),
            method_tparams: None,
            gen_locals: HashMap::new(),
            gen_defs: HashMap::new(),
            uses: HashMap::new(),
            obligations: HashMap::new(),
            lam_depth: 0,
            memo_depth: 0,
            let_depth: 0,
            active_regions: Vec::new(),
            oneshot_frames: Vec::new(),
            scope_frames: Vec::new(),
            task_captures: Vec::new(),
            par_captures: Vec::new(),
            body_tparams: Vec::new(),
            local_closure_captures: HashMap::new(),
            task_forwarders: HashSet::new(),
            checking_task_forwarder: false,
            place_path: Vec::new(),
            effects: HashMap::new(),
            effect_bindings: HashMap::new(),
            codata_types: HashSet::new(),
            effect_ops: HashMap::new(),
            conts: Vec::new(),
            cont_depth: 0,
            extents: Vec::new(),
            seq_binders: Vec::new(),
            current_cont: Vec::new(),
            compile_time: false,
            record_quote_bindings: false,
            proof_goal_site: None,
            editor_references: RefCell::new(None),
            editor_modules: HashSet::new(),
            instance_searches: RefCell::new(Vec::new()),
            current_declaration: String::new(),
            scoped_type_env: RefCell::new(Vec::new()),
            index_carriers: RefCell::new(Vec::new()),
            index_lambda_binders: RefCell::new(Vec::new()),
            index_lambda_counter: Cell::new(0),
            index_header_context: RefCell::new(Vec::new()),
            index_dictionary_scope: RefCell::new(Vec::new()),
            law_index_context: RefCell::new(None),
            code_quantifiers: Scheme::mono(Ty::Unit),
            body_annotation_scope: StageTypeBindings::default(),
            method_annotation_aliases: None,
            stage_calls: RefCell::new(HashMap::new()),
            stage_definition_scopes: HashMap::new(),
            hole_tys: Vec::new(),
            staging: std::collections::HashSet::new(),
            macros: std::collections::HashMap::new(),
            staging_fns: std::collections::HashMap::new(),
            generate_blocks: Vec::new(),
            checked_early: std::collections::HashSet::new(),
            stage_inputs: Vec::new(),
            stage_emitted: std::collections::HashMap::new(),
            stage_mode: StageMode::Refresh,
            module_files: std::collections::HashMap::new(),
            module_stage_imports: std::collections::HashMap::new(),
            macro_attr_names: std::collections::HashMap::new(),
            diags: Vec::new(),
            fn_hints: HashMap::new(),
            layout_recipes: HashMap::new(),
            type_layouts: HashMap::new(),
            safety_attrs: HashMap::new(),
            module_safety: HashMap::new(),
            safety_records: Vec::new(),
            safety_facts: HashMap::new(),
            pending_contracts: HashMap::new(),
            contract_facts: HashMap::new(),
            temporal_rows: Vec::new(),
            cur_temporal: None,
            cur_runtime_decides: Vec::new(),
            cur_def_deny: false,
            jpl_target: false,
            jpl_user_modules: HashSet::new(),
            pending_rewrites: Vec::new(),
            pending_prims: Vec::new(),
            prims: Vec::new(),
            target: None,
            build: None,
            law_obligations: Vec::new(),
            pres_obligations: Vec::new(),
            law_rows: Vec::new(),
            pending_upgrades: Vec::new(),
            upgrade_evidence: Vec::new(),
            rewrites: Vec::new(),
            ev_writes: Vec::new(),
            ev_derivs: HashMap::new(),
            ev_uses: RefCell::new(Vec::new()),
            ev_pre: HashMap::new(),
            col_readings: HashMap::new(),
            inline_cols: HashMap::new(),
            transfer_obligs: RefCell::new(Vec::new()),
            transfer_seen: RefCell::new(HashSet::new()),
            prop_calls: Vec::new(),
            app_callee_depth: 0,
            param_facts: Vec::new(),
            result_params: Vec::new(),
        }
    }

    /// Start a fresh performed-effects scope (lambda bodies, memo cell
    /// bodies); returns the enclosing scope's row.
    fn enter_perf(&mut self) -> EffRow {
        let fresh = self.uni.fresh_row();
        std::mem::replace(&mut self.perf, fresh)
    }

    /// Leave the current scope and restore the saved row; returns the
    /// effects performed inside it.
    fn exit_perf(&mut self, saved: EffRow) -> EffRow {
        std::mem::replace(&mut self.perf, saved)
    }

    /// Record effects performed in the current scope.
    fn perform(&mut self, row: &EffRow) -> Result<()> {
        let cur = std::mem::replace(&mut self.perf, EffRow::pure());
        let joined = self
            .uni
            .union_rows(&cur, row)
            .map_err(|m| Error::plain(m))?;
        self.perf = joined;
        Ok(())
    }

    /// The performed-effects row of the current scope.
    fn perf_row(&self) -> EffRow {
        self.perf.clone()
    }

    /// A fresh memo region name, e.g. `r#3`.
    fn fresh_region(&self) -> String {
        let n = self.region_counter.get();
        self.region_counter.set(n + 1);
        format!("r#{n}")
    }

    /// The source spelling of a rigid region name of the definition being
    /// checked, for diagnostics: the parameters' identities are fresh
    /// (`region#3`) and the source named them once.
    fn source_region_name(&self, rigid: &str) -> Option<String> {
        self.body_annotation_scope
            .regions
            .iter()
            .find(|(_, r)| r.as_str() == rigid)
            .map(|(name, _)| name.clone())
    }

    /// Discharge a memo group at scope end: its `Eval` label leaves the
    /// performed row (the memo construct is a built-in handler of its own
    /// label, laziness and reclamation specification, section 6), and
    /// nothing typed with the group's region may flow out of the scope
    /// (region escape checking, ownership and usage design, section 4).
    fn discharge_group(&mut self, g: &MemoGroup, result: &Ty, span: ast::Span) -> Result<()> {
        let cur = self.perf_row();
        let resolved = self.uni.resolve_row(&cur);
        let labels: Vec<EffLabel> =
            resolved.labels.into_iter().filter(|l| *l != g.label).collect();
        self.perf = EffRow { labels, tail: resolved.tail };
        if let Some(i) = self.active_regions.iter().position(|r| *r == g.region) {
            self.active_regions.remove(i);
        }
        let t = self.uni.zonk(result);
        let mut regions = Vec::new();
        escape_regions(&t, &mut regions);
        if regions.iter().any(|r| *r == g.region) {
            return Err(Error::at(
                span,
                crate::diagnostic!(
                    "region escape: a borrow, memo cell or Eval effect of region {} leaves its scope in {}",
                    g.region, t
                ),
            ));
        }
        Ok(())
    }

    // ------------------------------------------------------------ usage and regions

    /// Reset the per-definition usage state.
    fn reset_usage(&mut self) {
        self.uses.clear();
        self.obligations.clear();
        self.ev_writes.clear();
        self.ev_derivs.clear();
        self.ev_uses.get_mut().clear();
        self.ev_pre.clear();
        self.col_readings.clear();
        self.inline_cols.clear();
        self.cur_temporal = None;
        self.lam_depth = 0;
        self.memo_depth = 0;
        self.let_depth = 0;
        self.active_regions.clear();
        self.oneshot_frames.clear();
        self.scope_frames.clear();
        self.task_captures.clear();
        self.par_captures.clear();
        self.local_closure_captures.clear();
        self.place_path.clear();
        self.conts.clear();
        self.cont_depth = 0;
        self.extents.clear();
    }

    /// Bind a local name: enforce `Mut` uniqueness for its borrow regions
    /// (ownership and usage design, section 4.3) and register its usage
    /// obligation. The first obligation registered for a local wins:
    /// equation parameters are registered from their arrow's declared
    /// multiplicity before the pattern compiler binds their names.
    fn bind_local(
        &mut self,
        env: &mut Env,
        name: &str,
        l: LocalId,
        scheme: Scheme,
        mult: Option<Mult>,
        span: ast::Span,
    ) -> Result<()> {
        let mut regions = Vec::new();
        borrow_regions(&scheme.ty, &mut regions);
        for (r, is_mut) in &regions {
            for (_, _, s) in &env.locals {
                let mut others = Vec::new();
                borrow_regions(&self.uni.zonk(&s.ty), &mut others);
                for (r2, is_mut2) in &others {
                    if r == r2 && (*is_mut || *is_mut2) {
                        return Err(Error::at(
                            span,
                            format!(
                                "region {r}: exclusive borrows are unique; {} would alias another live borrow of {r}",
                                name
                            ),
                        ));
                    }
                }
            }
        }
        if !self.obligations.contains_key(&l) {
            self.register_obligation(l, name, mult, scheme.ty.clone(), span);
        }
        // Evidence invalidation over mutable state: a `Prop`-typed binder
        // is an evidence item. Record where it was derived (the write-log
        // length right now) so a later write to a location it reads can
        // name this derivation site when the item goes stale.
        if matches!(self.uni.zonk(&scheme.ty), Ty::Prp(_)) {
            self.ev_derivs.insert(l, (self.ev_writes.len(), span));
            let origin = self.value_aliases.get(&l)
                .and_then(|original| self.proof_value_scopes.get(original)).cloned()
                .unwrap_or_else(|| env.locals.iter().map(|(n, local, _)| (n.clone(), *local)).collect());
            self.proof_value_scopes.insert(l, origin);
        }
        env.locals.push((name.to_string(), l, scheme));
        Ok(())
    }

    /// Register the binding carried by syntax, alongside the ordinary source
    /// spelling. Looking up a captured handle never falls back to that spelling.
    fn bind_syntax_local(
        &mut self, env: &mut Env, name: &ast::Name, l: LocalId,
        scheme: Scheme, mult: Option<Mult>, span: ast::Span,
    ) -> Result<()> {
        if let Some(binding) = name.local_binding() {
            if env.syntax_locals.get(binding).and_then(|slot| env.locals.get(*slot))
                .is_some_and(|(_, previous, _)| *previous != l)
            {
                return Err(Error::at(span, format!("syntax binding `{}` is introduced twice in the same lexical scope", name.text)));
            }
        }
        self.bind_local(env, &name.text, l, scheme, mult, span)?;
        if let Some(binding) = name.local_binding() {
            env.syntax_locals.insert(binding.clone(), env.locals.len() - 1);
        }
        Ok(())
    }

    /// The mutable locations an evidence item's statement reads (evidence
    /// invalidation over mutable state): the regions of `Mut` and `Ref`
    /// borrows named by the proposition's index terms, split by borrow
    /// kind so `Ref`-sourced (read-only) evidence is never invalidated.
    fn ev_footprint(&self, q: &PTerm, env: &Env) -> (Vec<String>, Vec<String>) {
        let mut vars = Vec::new();
        collect_prop_reads(q, &mut vars);
        let mut muts: Vec<String> = Vec::new();
        let mut refs: Vec<String> = Vec::new();
        let mut locals = Vec::new();
        q.local_values(&mut locals);
        let bindings = vars.iter().filter_map(|name| env.lookup(name))
            .chain(locals.iter().filter_map(|id| env.locals.iter().find(|(_, local, _)| local == id)
                .map(|(_, local, scheme)| (*local, scheme.clone()))));
        for (_, scheme) in bindings {
            match self.uni.zonk(&scheme.ty) {
                Ty::Mut(region, _) => { if !muts.iter().any(|r| r.as_str() == region.as_str()) { muts.push(region.to_string()); } }
                Ty::Ref(region, _) => { if !refs.iter().any(|r| r.as_str() == region.as_str()) { refs.push(region.to_string()); } }
                _ => {}
            }
        }
        (muts, refs)
    }

    /// Record a write to a mutable location (the `Mut` borrow's region),
    /// or an opaque write when `loc` is `None`.
    fn note_ev_write(&mut self, loc: Option<String>, span: ast::Span) {
        self.ev_writes.push(EvWrite {
            loc,
            span,
            value: None,
            call: None,
        });
    }

    /// Record a direct write through a `Mut` borrow of region `loc`. The
    /// written value's index shape (`None` when the value has no index
    /// reading) feeds the temporal contracts' per-step obligations
    /// (core specification section 10).
    fn note_ev_write_loc(&mut self, loc: String, span: ast::Span, value: Option<ITerm>) {
        self.ev_writes.push(EvWrite {
            loc: Some(loc),
            span,
            value,
            call: None,
        });
    }

    /// Record an opaque write (FFI / `RawPtr` write or `eval`): its target
    /// is unknown, so it conservatively invalidates every mutable-location
    /// evidence item derived before it.
    fn note_ev_write_opaque(&mut self, span: ast::Span) {
        self.ev_writes.push(EvWrite {
            loc: None,
            span,
            value: None,
            call: None,
        });
    }

    /// Record an evidence use for a deferred staleness check. The check
    /// runs after constraint solving (which unifies regions), since a
    /// call's effect-row region may only resolve there. Only `Mut`-sourced
    /// evidence (reads a mutable location) can go stale; `Ref`-sourced and
    /// immutable evidence never do.
    fn record_ev_use(&self, l: LocalId, prop: &PTerm, env: &Env, use_span: ast::Span) {
        self.note_comparison_use(l, use_span);
        if !self.ev_derivs.contains_key(&l) {
            return;
        }
        let (muts, _refs) = self.ev_footprint(prop, env);
        if muts.is_empty() {
            return;
        }
        self.ev_uses.borrow_mut().push((l, muts, use_span));
    }

    /// The evidence in scope at a call (the `Dec` evidence path and
    /// branch-condition facts). Evidence names values by their source
    /// names, so an item whose statement mentions a name rebound after
    /// it (an inner `let x` or pattern `x` shadowing the `x` it was
    /// derived about) no longer speaks about what that name now denotes
    /// and is left out. Sorts and refined predicates come from each
    /// name's innermost binding only.
    fn values_in_scope(&self, env: &Env, values: &[LocalId]) -> bool {
        values.iter().all(|id| self.proof_pattern_values.contains(id) || env.locals.iter().any(|(_, local, _)|
            local == id || self.value_aliases.get(local).is_some_and(|original| original == id)))
    }

    fn evidence_in(&self, env: &Env) -> Evidence {
        let mut out = Evidence::default();
        for (name, local, _) in &env.locals {
            if env.lookup(name).is_some_and(|(active, _)| active == *local) {
                let original = self.value_aliases.get(local).copied().unwrap_or(*local);
                out.visible_values.insert(original, ITerm::Var(name.clone().into()));
            }
        }
        // Logical local identities remain scoped even if their source name
        // is shadowed. They are never ordinary source lookup bindings.
        for (_, local, scheme) in &env.locals {
            if scheme.params.is_empty() {
                if let Some((sort, _)) = self.int_view(&self.uni.zonk(&scheme.ty)) {
                    let original = self.value_aliases.get(local).copied().unwrap_or(*local);
                    if !out.value_sorts.iter().any(|(old, _)| *old == original) { out.value_sorts.push((original, sort)); }
                }
            }
        }
        for local in &self.proof_pattern_values {
            if let Some(ty) = self.local_tys.get(local) {
                if let Some((sort, _)) = self.int_view(&self.uni.zonk(ty)) {
                    if !out.value_sorts.iter().any(|(old, _)| old == local) {
                        out.value_sorts.push((*local, sort));
                    }
                }
            }
        }
        // Innermost first, so `later` holds every name bound after the
        // current local; the items are reversed back to scope order.
        let mut later: HashSet<&str> = HashSet::new();
        for (name, local, scheme) in env.locals.iter().rev() {
            let t = self.uni.zonk(&scheme.ty);
            let fresh = later.insert(name.as_str());
            let Ty::Prp(q) = &t else {
                if !fresh || !scheme.params.is_empty() {
                    continue;
                }
                // A refined local's predicate holds of it: over an
                // integer container with the sort, over any other (a
                // `Bytes` whose length a result refinement states) as the
                // fact alone. It may also name other locals (a result
                // refinement stated over the call's arguments, `bytes-len
                // out == n`), which must still denote what they denoted
                // at the binding: none of them is rebound after it.
                let rv = match self.int_view(&t) {
                    Some((sort, rv)) => {
                        out.sorts.push((name.clone(), sort));
                        rv
                    }
                    None => self.refine_view(&t),
                };
                if let Some(rv) = rv {
                    let mut values = Vec::new(); rv.pred.local_values(&mut values);
                    if !self.values_in_scope(env, &values) { continue; }
                    let mut fv = Vec::new();
                    rv.pred.free_vars(&mut fv);
                    if fv.iter().all(|v| {
                        *v == rv.binder || (!later.contains(v.as_str()) && env.lookup(v).is_some())
                    }) {
                        let fact = rv.pred.subst(&rv.binder, &ITerm::Var(name.clone().into()));
                        out.refined.push(fact.subst_local_values(&out.visible_values));
                    }
                }
                continue;
            };
            let q = self.uni.resolve_prop(q);
            let origin = self.proof_value_scopes.get(local);
            let q = origin.map(|source| self.proof_values_in_scope(&q, source, env)).unwrap_or(q);
            let mut values = Vec::new(); q.local_values(&mut values);
            if !self.values_in_scope(env, &values) { continue; }
            let mut fv = Vec::new();
            q.free_vars(&mut fv);
            // `later` already holds this binder's own name, which its
            // statement never mentions.
            if origin.is_none() && fv.iter().any(|v| v != name && later.contains(v.as_str())) {
                continue;
            }
            let (muts, _) = self.ev_footprint(&q, env);
            out.items.push((name.clone(), q, *local, muts));
        }
        out.items.reverse();
        out
    }

    /// A value type's integer sort, looking through a mutable or borrowed
    /// place (`copyOut m` reads the contents) and a refinement (its
    /// view is returned with the sort: its predicate holds of the value).
    fn int_view(&self, t: &Ty) -> Option<(IntSort, Option<RefineView>)> {
        match t {
            Ty::Nat => Some((IntSort::Nat, None)),
            Ty::Int => Some((IntSort::Int, None)),
            Ty::Prim(s) => IntSort::of_scalar(*s).map(|s| (s, None)),
            Ty::Mut(_, inner) | Ty::Ref(_, inner) => self.int_view(inner),
            Ty::Adt(..) => {
                let rv = self.refine_view(t)?;
                let (sort, _) = self.int_view(rv.base.as_ref()?)?;
                Some((sort, Some(rv)))
            }
            _ => None,
        }
    }

    /// The integer sort of a named function's result applied to `arity`
    /// arguments (the head of an index-level application).
    fn fn_int_sort(&self, head: &str, arity: usize) -> Option<IntSort> {
        if head == BYTE_LENGTH_INDEX { return (arity == 1).then_some(IntSort::Nat); }
        let (module, name) = head.rsplit_once('.')?;
        let v = self.qualified.get(&(module.to_string(), name.to_string()))?;
        let mut ty = v.scheme.ty.clone();
        for _ in 0..arity {
            ty = match ty {
                Ty::Fun(_, _, _, cod) => *cod,
                Ty::Pi { cod, .. } => *cod,
                _ => return None,
            };
        }
        self.int_view(&ty).map(|(s, _)| s)
    }

    /// Every fact in scope together, by the linear procedure (`types.rs`,
    /// "linear facts over variables"): the evidence items its checked
    /// proof cites, when it proves the obligation.
    fn linear_discharge(&self, ev: &Evidence, obl: &PTerm) -> Option<Vec<usize>> {
        let mut hyps: Vec<PTerm> = ev.items.iter().map(|(_, q, _, _)| q.clone()).collect();
        hyps.extend(ev.refined.iter().cloned());
        // Every application the facts and the goal mention holds its
        // callee's result refinement (`0 <= u32-to-i64 c` from
        // `u32-to-i64 :: U32 -> InRange 0 4294967295 I64`), one more
        // fact. A range holds of every result, so this is sound whatever
        // the arguments are and however often the term is evaluated.
        let mut facts = Vec::new();
        for p in hyps.iter().chain(std::iter::once(obl)) {
            self.app_result_facts(p, &mut facts);
        }
        for f in facts {
            if !hyps.contains(&f) {
                hyps.push(f);
            }
        }
        let mut heads: Vec<(String, usize)> = Vec::new();
        for p in hyps.iter().chain(std::iter::once(obl)) {
            index_heads(p, &mut heads);
        }
        let fns = heads
            .into_iter()
            .filter_map(|(h, k)| self.fn_int_sort(&h, k).map(|s| (h, k, s)))
            .collect();
        let sorts = Sorts { vars: ev.sorts.clone(), values: ev.value_sorts.clone(), fns };
        let proofs = linear_entails(&hyps, obl, &sorts)?;
        let mut used = Vec::new();
        for i in proofs.iter().flat_map(|p| p.hyps_used()) {
            if i < ev.items.len() && !used.contains(&i) {
                used.push(i);
            }
        }
        Some(used)
    }

    /// Whether an index term denotes one value wherever it is read: a
    /// numeral, a local, arithmetic over such terms, or a pure arithmetic
    /// intrinsic applied to all of its arguments (`PrimOp::is_pure_arith`).
    fn stable_reading(&self, t: &ITerm) -> bool {
        match t {
            ITerm::Num(_) | ITerm::Var(_) => true,
            ITerm::Bin(_, a, b) => self.stable_reading(a) && self.stable_reading(b),
            // A top-level constant (a nullary definition, or a nullary
            // pure intrinsic such as `max-bound-i64`) is one value.
            ITerm::Fn(h) => h.rsplit_once('.').is_some_and(|(module, name)| {
                self.qualified.get(&(module.to_string(), name.to_string())).is_some_and(|v| {
                    !matches!(v.scheme.ty, Ty::Fun(..) | Ty::Pi { .. })
                        && match v.kind {
                            ValueKind::TopFn => true,
                            ValueKind::Prim(op) => op.is_pure_arith(),
                            _ => false,
                        }
                })
            }),
            ITerm::App(f, args) => {
                let ITerm::Fn(h) = &**f else { return false };
                if h == BYTE_LENGTH_INDEX {
                    return args.len() == 1 && args.iter().all(|a| self.stable_reading(a));
                }
                let Some((module, name)) = h.rsplit_once('.') else { return false };
                match self.qualified.get(&(module.to_string(), name.to_string())).map(|v| &v.kind) {
                    Some(ValueKind::Prim(op)) => {
                        op.is_pure_arith() && op.arity() == args.len() && args.iter().all(|a| self.stable_reading(a))
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// The result facts of the applications a predicate mentions (see
    /// `linear_discharge`). A result refinement naming an argument
    /// (`refined (r < $a1) Nat`) states its fact only where that argument
    /// is a numeral, a local, arithmetic over them or a pure arithmetic
    /// intrinsic applied to them: such a reading denotes the same value
    /// at the call and in the fact.
    fn app_result_facts(&self, p: &PTerm, out: &mut Vec<PTerm>) {
        let simple = |t: &ITerm| self.stable_reading(t);
        fn terms<'a>(t: &'a ITerm, out: &mut Vec<(&'a str, &'a [ITerm], &'a ITerm)>) {
            match t {
                ITerm::Fn(h) => out.push((h, &[], t)),
                ITerm::App(f, args) => {
                    if let ITerm::Fn(h) = &**f {
                        out.push((h, args, t));
                    } else {
                        terms(f, out);
                    }
                    args.iter().for_each(|a| terms(a, out));
                }
                ITerm::Bin(_, a, b) => {
                    terms(a, out);
                    terms(b, out);
                }
                ITerm::Inst(call) => {
                    // Evidenceful globals need their instantiated result carrier
                    // before exporting result facts; do not use a bare scheme.
                    for argument in call.terms() { terms(argument, out); }
                }
                ITerm::Lam(..) | ITerm::Var(_) | ITerm::LocalValue(_) | ITerm::Evidence(_) | ITerm::Num(_) | ITerm::MVar(_) => {}
            }
        }
        fn preds<'a>(p: &'a PTerm, out: &mut Vec<(&'a str, &'a [ITerm], &'a ITerm)>) {
            match p {
                PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                    terms(a, out);
                    terms(b, out);
                }
                PTerm::Not(q) => preds(q, out),
                PTerm::And(a, b) | PTerm::Or(a, b) => {
                    preds(a, out);
                    preds(b, out);
                }
                PTerm::Pred(_, a) => terms(a, out),
                PTerm::Lit(_) | PTerm::Var(_) | PTerm::MVar(_) => {}
            }
        }
        let mut apps = Vec::new();
        preds(p, &mut apps);
        for (head, args, term) in apps {
            if head == BYTE_LENGTH_INDEX {
                if args.len() == 1 {
                    if let Some((promise, _)) = prim_result_promise(PrimOp::Bytes(BytesOp::Len)) {
                        out.push(promise.subst("$it", term));
                    }
                }
                continue;
            }
            let Some((module, name)) = head.rsplit_once('.') else { continue };
            let Some(v) = self.qualified.get(&(module.to_string(), name.to_string())) else { continue };
            let mut ty = v.scheme.ty.clone();
            let mut ok = true;
            for a in args {
                ty = match ty {
                    Ty::Fun(_, _, _, cod) => *cod,
                    Ty::Pi { var, cod, .. } => cod.subst_idx(&var, a),
                    _ => {
                        ok = false;
                        break;
                    }
                };
            }
            if !ok || matches!(ty, Ty::Fun(..) | Ty::Pi { .. }) {
                continue;
            }
            let readings: Vec<Option<ITerm>> =
                args.iter().map(|a| simple(a).then(|| a.clone())).collect();
            let ty = Self::resolve_positional_result(&ty, &readings);
            let Some(rv) = self.refine_view(&ty) else { continue };
            let mut fv = Vec::new();
            rv.pred.free_vars(&mut fv);
            let mut avs = Vec::new();
            for a in args {
                a.free_vars(&mut avs);
            }
            if !fv.iter().all(|x| *x == rv.binder || avs.contains(x)) {
                continue;
            }
            let f = rv.pred.subst(&rv.binder, term);
            if !out.contains(&f) {
                out.push(f);
            }
        }
    }

    /// Discharge one refinement obligation at a call (refinement types
    /// design, "instance heads at refinements"): the caller supplies
    /// evidence in scope, a `Dec` binder or a branch condition's fact
    /// (D192 staleness applies to both). One item may entail the
    /// obligation, or the items over its names together may (nested
    /// conditions each contribute a bound), or, by the linear procedure,
    /// every fact in scope together (`i <= j` and `j < n` give `i < n`).
    /// Entailment reads the names as they are, never up to renaming:
    /// evidence about `a < b` says nothing about `b < a`. A named binder
    /// that establishes a different fact is refused as forged evidence;
    /// no evidence is refused naming the obligation and its discharge
    /// paths.
    fn discharge_oblig(&self, evidence: &Evidence, obl: &PTerm, span: ast::Span) -> Result<()> {
        // Arguments have now solved the callee's actual instantiation. Resolve
        // its metadata before matching scoped facts; this never specializes
        // an unknown parameter or discards a type/dictionary identity.
        let resolved = self.uni.resolve_prop(obl);
        let normalized = resolved.subst_local_values(&evidence.visible_values);
        let obl = &normalized;
        let mut ov: Vec<String> = Vec::new();
        obl.free_vars(&mut ov);
        let mut forged: Option<(String, PTerm)> = None;
        // The items over exactly this obligation's names: evidence about
        // another argument proves nothing here. The innermost item is
        // tried first: a re-derived fact is the fresh one after a write.
        let mut over: Vec<&(String, PTerm, LocalId, Vec<String>)> = Vec::new();
        for item in evidence.items.iter().rev() {
            let (name, q, _, _) = item;
            let mut qv: Vec<String> = Vec::new();
            q.free_vars(&mut qv);
            if qv.iter().all(|v| ov.contains(v)) {
                if pred_entails(q, obl) {
                    self.note_ev_used(&[item], span);
                    return Ok(());
                }
                over.push(item);
            }
            if forged.is_none() && qv.iter().any(|v| ov.contains(v)) && !name.starts_with('%') {
                forged = Some((name.clone(), q.clone()));
            }
        }
        if over.len() > 1 {
            let mut it = over.iter().map(|(_, q, _, _)| q.clone());
            let first = it.next().expect("two or more items");
            let all = it.fold(first, |acc, q| PTerm::And(Box::new(acc), Box::new(q)));
            if pred_entails(&all, obl) {
                self.note_ev_used(&over, span);
                return Ok(());
            }
        }
        if let Some(used) = self.linear_discharge(evidence, obl) {
            let items: Vec<&(String, PTerm, LocalId, Vec<String>)> =
                used.iter().map(|i| &evidence.items[*i]).collect();
            self.note_ev_used(&items, span);
            return Ok(());
        }
        if let Some((name, q)) = forged {
            return Err(Error::at(
                span,
                format!(
                    "`{name}` is evidence of `{q}`, not of `{obl}` (forged evidence is refused: the `Dec`/`decide` world establishes evidence, and it must state this obligation)"
                ),
            ));
        }
        Err(Error::at(
            span,
            format!(
                "the obligation `{obl}` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide ({obl}) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `{obl}` is in scope here"
            ),
        ))
    }

    /// Record the evidence items a discharge read (evidence invalidation
    /// over mutable state): staleness is checked after regions unify.
    fn note_ev_used(&self, items: &[&(String, PTerm, LocalId, Vec<String>)], span: ast::Span) {
        for (_, _, local, muts) in items {
            self.note_comparison_use(*local, span);
            if self.ev_derivs.contains_key(local) && !muts.is_empty() {
                self.ev_uses.borrow_mut().push((*local, muts.clone(), span));
            }
        }
    }

    /// Check every recorded evidence use for staleness (evidence
    /// invalidation over mutable state), now that regions have unified.
    /// A use whose evidence a later write invalidated is rejected, naming
    /// the derivation site and the invalidating mutation.
    fn check_ev_uses(&mut self) -> Result<()> {
        let uses = std::mem::take(self.ev_uses.get_mut());
        for (l, muts, use_span) in uses {
            let Some(&(deriv_len, deriv_span)) = self.ev_derivs.get(&l) else {
                continue;
            };
            let pre = self.ev_pre.get(&l).map(|&(a, b)| &self.ev_writes[a..b]).unwrap_or(&[]);
            for w in pre.iter().chain(&self.ev_writes[deriv_len..]) {
                let hits = match &w.loc {
                    None => true,
                    Some(r) => {
                        let r = self.uni.resolve_region(r);
                        muts.iter().any(|m| self.uni.resolve_region(m) == r)
                    }
                };
                if hits {
                    let where_ = muts.join(", ");
                    return Err(Error::at(
                        use_span,
                        format!(
                            "stale evidence: evidence about the contents of `{where_}` derived at {deriv_span} is invalidated by the write at {}",
                            w.span
                        ),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Record the writes a call can perform (evidence invalidation over
    /// mutable state): a `Mut` argument hands the callee an exclusive
    /// borrow it may write, and the effect-row reading of "can write" is a
    /// write-capable label naming a region, or an open row that may write
    /// anything. Each step records the callee and the call's index
    /// arguments (the temporal contracts' per-step obligations: a
    /// self-recursive call discharges its step by the inductive
    /// hypothesis, core specification section 10).
    fn note_call_arg_writes(
        &mut self,
        env: &Env,
        args: &[ast::Term],
        dom: &Ty,
        eff: &EffRow,
        span: ast::Span,
        callee: &ECore,
    ) {
        // The steps this call can perform, computed before any push.
        let mut targets: Vec<Option<String>> = Vec::new();
        if let Ty::Mut(r, _) = self.uni.zonk(dom) {
            targets.push(Some(r.to_string()));
        }
        let resolved = self.uni.resolve_row(eff);
        for l in &resolved.labels {
            if !write_effect_head(&l.head) {
                continue;
            }
            match &l.arg {
                // A write-capable label naming a region writes there; one
                // naming none may write anything (the effect-row reading
                // of "can write", evidence invalidation over mutable
                // state).
                Some(a) => targets.push(Some(a.clone().to_string())),
                None => targets.push(None),
            }
        }
        if !matches!(resolved.tail, RowTail::Closed) {
            targets.push(None);
        }
        if targets.is_empty() {
            return;
        }
        let call = call_step(self, callee, env, args);
        for loc in targets {
            self.ev_writes.push(EvWrite {
                loc,
                span,
                value: None,
                call: call.clone(),
            });
        }
    }

    /// Record an opaque write performed by a callee whose write target is
    /// unknown (a foreign call, `eval`, or a mutating `Bytes` primitive):
    /// conservatively it invalidates every mutable-location evidence item
    /// derived before it.
    fn note_opaque_write_from(&mut self, callee: &ECore, span: ast::Span) {
        if callee_writes_opaque(callee) {
            self.ev_writes.push(EvWrite {
                loc: None,
                span,
                value: None,
                call: None,
            });
        }
    }

    /// Register a binder's usage obligation (first registration for a
    /// local wins).
    fn register_obligation(
        &mut self,
        l: LocalId,
        name: &str,
        mult: Option<Mult>,
        ty: Ty,
        span: ast::Span,
    ) {
        self.note_local_ty(l, &ty);
        if !self.obligations.contains_key(&l) {
            self.obligations.insert(
                l,
                Obligation {
                    name: name.to_string(),
                    mult,
                    ty,
                    span,
                    lam: self.lam_depth,
                    memo: self.memo_depth,
                    cont: self.cont_depth,
                },
            );
        }
    }

    /// Record a binder's type for its representation class (the first
    /// record wins, like obligations: an equation parameter's declared
    /// domain precedes its pattern binding).
    pub(crate) fn note_local_ty(&mut self, l: LocalId, ty: &Ty) {
        self.local_tys.entry(l).or_insert_with(|| ty.clone());
    }

    /// Fix a binder's class without a type (a dictionary or evidence).
    pub(crate) fn note_local_class(&mut self, l: LocalId, c: crate::repr::Class) {
        self.local_cls.insert(l, c);
    }

    /// The class of a dictionary binder for `class`: capabilities are
    /// runtime evidence words and the `Copy` marker is the unit word;
    /// every other dictionary is a counted block.
    pub(crate) fn dict_class(class: &str) -> crate::repr::Class {
        if is_capability_class(class) || class == "Copy" {
            crate::repr::Class::Raw
        } else {
            crate::repr::Class::Obj
        }
    }

    /// Record one use of a local at a record-projection place. Uses under
    /// a lambda body are unbounded (the closure may be called any number
    /// of times); uses under a memo cell body happen at most once (the
    /// cell evaluates at most once). A use directly in a task body or a
    /// `withScope` body is one-shot: that body runs exactly once per
    /// spawn or scope execution, so it transfers a capture exactly once
    /// (concurrency specification section 2). A use inside a continuation
    /// closure over the binder happens at most once per execution of the
    /// continuation's build site, and exactly once only when every
    /// handler in flight promises exactly-once resumption (see
    /// [`Checker::promote_resumptions`]).
    fn note_use(
        &mut self,
        l: LocalId,
        span: ast::Span,
        path: &[String],
        dicts: &[(LocalId, ClassApp)],
    ) -> Result<()> {
        let Some(ob) = self.obligations.get(&l) else {
            return Ok(());
        };
        let name = ob.name.clone();
        let ty = ob.ty.clone();
        let (lam, memo, created_cont) = (ob.lam, ob.memo, ob.cont);
        let crosses_lam = self.lam_depth > lam;
        // Every lambda layer between the binder and this use is a
        // one-shot frame (a task body or a `withScope` body), so the use
        // happens at most once per enclosing execution.
        let oneshot = crosses_lam
            && (lam + 1..=self.lam_depth)
                .all(|d| self.oneshot_frames.iter().any(|f| f.lam == d));
        if crosses_lam {
            let t = self.uni.zonk(&ty);
            if oneshot {
                let frame = self
                    .oneshot_frames
                    .iter()
                    .rev()
                    .find(|f| f.lam == self.lam_depth)
                    .expect("the innermost layer is a one-shot frame");
                if frame.task {
                    // A task capture (concurrency specification section
                    // 3): the ownership rules are validated at the
                    // definition's end, when the binder's type is fully
                    // resolved.
                    self.task_captures.push(TaskCapture {
                        binder: l,
                        name: name.clone(),
                        ty: t,
                        span,
                        dicts: dicts.to_vec(),
                        scope: frame.scope.clone(),
                        scopes: self.scope_frames.iter().map(|f| f.region.clone()).collect(),
                        active: self.active_regions.clone(),
                        outer: frame.outer.clone(),
                        transfer_allowed: true,
                    });
                } else if !frame.parallel {
                    // A `withScope` body capture: the borrowed
                    // environment rule (core specification, section 7)
                    // still applies. A parallel callback is exempt: it
                    // runs inside its call and is dropped before the call
                    // returns, and the parallel capture rules classify
                    // what it reads.
                    self.reject_live_region_capture(&t, &name, span)?;
                }
            } else {
                // A callback cannot outlive a borrowed environment (core
                // specification, section 7). Capture-based ownership
                // transfer needs closure usage inference, which this slice
                // does not have, so capturing a borrow or memo cell of a
                // live region is rejected conservatively.
                self.reject_live_region_capture(&t, &name, span)?;
                // A task handle or scope value must not be captured by a
                // lambda that may outlive its structured scope
                // (concurrency specification, section 2).
                if let Some(r) = self.open_scope_region(&t) {
                    return Err(Error::at(
                        span,
                        format!(
                            "region escape: {name} is captured in a lambda that may outlive the structured scope of region {r} (concurrency specification, section 2)"
                        ),
                    ));
                }
            }
        }
        let c = if oneshot && self.memo_depth == memo {
            if self.cont_depth > created_cont {
                // A one-shot body inside a continuation: the
                // continuation's own discipline composes on top of it.
                self.cont_use_count(l, span)?
            } else {
                Count::ONE
            }
        } else if crosses_lam {
            Count::UNBOUNDED
        } else if self.memo_depth > memo {
            Count::AT_MOST_ONCE
        } else if self.cont_depth > created_cont {
            self.cont_use_count(l, span)?
        } else {
            Count::ONE
        };
        self.uses.entry(l).or_default().add(path, c);
        if self.cont_depth > created_cont && self.effective_mult(l) == Some(Mult::One) {
            // The `%1` capture is consumed in a continuation body: mark
            // the enclosing frames so a resumption capture over them can
            // demand exactly-once resumption.
            let start = self.conts.len() - self.cont_depth;
            for f in &mut self.conts[start..] {
                f.consumes_linear = true;
            }
        }
        Ok(())
    }

    /// The conservative borrowed-environment rule (core specification,
    /// section 7): a borrow or memo cell of a live region is never
    /// captured by a closure that may outlive the region.
    fn reject_live_region_capture(&self, t: &Ty, name: &str, span: ast::Span) -> Result<()> {
        let mut regions = Vec::new();
        escape_regions(t, &mut regions);
        if let Some(r) = regions.into_iter().find(|r| self.active_regions.contains(r)) {
            return Err(Error::at(
                span,
                format!("region escape: {name} is captured in a lambda that may outlive region {r}"),
            ));
        }
        Ok(())
    }

    /// The scope region a captured type mentions while that scope is
    /// still open (its `Task`/`Scope` values cannot outlive it).
    fn open_scope_region(&self, t: &Ty) -> Option<String> {
        let mut regions = Vec::new();
        all_regions(t, &mut regions);
        regions
            .into_iter()
            .find(|r| self.scope_frames.iter().any(|f| f.region == r.as_str()))
    }

    /// A use inside a continuation closure over the binder: the body
    /// runs when its continuation is called: at most once, and exactly
    /// once only when no enclosing frame can skip it.
    fn cont_use_count(&mut self, l: LocalId, span: ast::Span) -> Result<Count> {
        let linear = self.effective_mult(l) == Some(Mult::One);
        let (has_at_most, has_many, has_risky) = {
            let active = &self.conts[self.conts.len() - self.cont_depth..];
            (
                active.iter().any(|f| matches!(f.kind, ContKind::AtMost)),
                active.iter().any(|f| matches!(f.kind, ContKind::Many)),
                active.iter().any(|f| f.risky),
            )
        };
        if has_many {
            Ok(Count::UNBOUNDED)
        } else if linear && has_at_most {
            Ok(Count::AT_MOST_ONCE)
        } else if linear && has_risky {
            self.promote_resumptions(span)?;
            Ok(Count::ONE)
        } else if linear {
            Ok(Count::ONE)
        } else {
            Ok(Count::AT_MOST_ONCE)
        }
    }

    /// A continuation consuming a `%1` capture is reachable through a
    /// resumption: every handler in flight must resume its one-shot
    /// continuation exactly once, so the capture is consumed exactly once
    /// and never duplicated or silently dropped. Handlers written inline
    /// at their `handle` site can promise this (their `resume k` binders
    /// become `%1`); a handler reached through a value cannot, and its
    /// resumption discipline is fixed at construction.
    fn promote_resumptions(&mut self, span: ast::Span) -> Result<()> {
        if self.extents.is_empty() {
            // No handler is in flight: an unhandled operation faults, so
            // the continuation is never skipped without aborting.
            return Ok(());
        }
        for e in &self.extents {
            if !e.flexible || e.k_binders.iter().any(|k| k.is_none()) {
                return Err(Error::at(
                    span,
                    "a %1 capture flows into a one-shot resumption, but this handler cannot promise exactly-once resumption (it is not written inline at its `handle` site, or a clause has no `resume k` binder)",
                ));
            }
        }
        for e in &self.extents {
            for k in &e.k_binders {
                if let Some(ob) = self.obligations.get_mut(&k.unwrap()) {
                    ob.mult = Some(Mult::One);
                }
            }
        }
        Ok(())
    }

    /// Elaborate one `let`/`letrec` nesting level deeper (`f` runs the
    /// whole nested elaboration): past [`LET_DEPTH_LIMIT`] the named
    /// let-nesting depth-limit diagnostic fires at the offending
    /// binding's span instead of the recursion consuming stack.
    fn in_let<T>(
        &mut self,
        span: ast::Span,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        self.let_depth += 1;
        let out = if self.let_depth > LET_DEPTH_LIMIT {
            Err(Error::at(
                span,
                format!(
                    "let nesting depth limit ({LET_DEPTH_LIMIT}) exceeded: at most {LET_DEPTH_LIMIT} nested `let`/`letrec` bodies elaborate; deeper nesting is refused rather than risking the elaborator's stack"
                ),
            ))
        } else {
            f(self)
        };
        self.let_depth -= 1;
        out
    }

    /// Run `f` with an isolated use accumulator: one arm of a branch. Each
    /// path through a branch takes exactly one arm, so arm accumulators
    /// merge with [`Count::join`] and the merge composes sequentially with
    /// the enclosing scope.
    fn isolate<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<(T, HashMap<LocalId, UseNode>)> {
        let saved = std::mem::take(&mut self.uses);
        let out = f(self);
        let inner = std::mem::take(&mut self.uses);
        self.uses = saved;
        out.map(|t| (t, inner))
    }

    /// Merge the arms of a branch into the enclosing scope: each path
    /// takes exactly one arm, so a binder's counts join over the arms
    /// (zero where an arm does not use it) and the join composes
    /// sequentially with the enclosing scope's uses.
    fn merge_branches(&mut self, branches: Vec<HashMap<LocalId, UseNode>>) {
        let mut all: Vec<LocalId> = Vec::new();
        for b in &branches {
            for l in b.keys() {
                if !all.contains(l) {
                    all.push(*l);
                }
            }
        }
        let mut joined: HashMap<LocalId, UseNode> = HashMap::new();
        for l in all {
            let mut acc: Option<UseNode> = None;
            for b in &branches {
                let node = b.get(&l).cloned().unwrap_or_default();
                acc = Some(match acc {
                    None => node,
                    Some(mut a) => {
                        a.join(&node);
                        a
                    }
                });
            }
            if let Some(a) = acc {
                joined.insert(l, a);
            }
        }
        for (l, node) in joined {
            self.uses.entry(l).or_default().seq_add(&node);
        }
    }

    /// Compare every binder's use count against its multiplicity
    /// (ownership and usage design, section 2).
    fn check_usage(&mut self) -> Result<()> {
        self.check_task_captures()?;
        self.check_parallel_captures()?;
        let mut binders: Vec<(&LocalId, &Obligation)> = self.obligations.iter().collect();
        binders.sort_by_key(|(l, _)| l.0);
        for (l, ob) in binders {
            let c = self
                .uses
                .get(l)
                .map(|n| n.consumptions())
                .unwrap_or(Count::ZERO);
            let t = self.uni.zonk(&ob.ty);
            // A task handle is consumed exactly once by joining, whatever
            // its annotation says (concurrency specification, section 2):
            // its multiplicity cannot be relaxed to dropping.
            let handle = ob.mult != Some(Mult::Zero) && crate::types::mentions_task(&t);
            let mult = if handle {
                Mult::One
            } else {
                match ob.mult {
                    Some(m) if m != Mult::Flex => m,
                    _ => default_mult(&t),
                }
            };
            match mult {
                Mult::Many | Mult::Flex => {}
                Mult::Zero => {
                    if c.max > 0 {
                        return Err(Error::at(
                            ob.span,
                            format!(
                                "multiplicity %0: {} is erased and must never be used, but is {}",
                                ob.name,
                                c.describe()
                            ),
                        ));
                    }
                }
                Mult::One => {
                    if c.min != 1 || c.max != 1 {
                        return Err(Error::at(
                            ob.span,
                            if handle {
                                format!(
                                    "task handle {} must be joined exactly once; dropping it would silently detach execution (concurrency specification, section 2), but it is {}",
                                    ob.name,
                                    c.describe()
                                )
                            } else {
                                format!(
                                    "multiplicity %1: {} must be used exactly once on every path, but is {}",
                                    ob.name,
                                    c.describe()
                                )
                            },
                        ));
                    }
                }
                Mult::Aff => {
                    if c.max > 1 {
                        return Err(Error::at(
                            ob.span,
                            format!(
                                "multiplicity %aff: {} must be used at most once, but is {}",
                                ob.name,
                                c.describe()
                            ),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// The task-capture rules (concurrency specification, section 3),
    /// once every binder type is resolved: an exclusive borrow never
    /// crosses a task boundary; a borrowed region released inside the
    /// task's scope never crosses it; a scope's handles stay in their
    /// scope; and a capture is transferred exactly once or shareable.
    fn check_task_captures(&mut self) -> Result<()> {
        for c in std::mem::take(&mut self.task_captures) {
            let t = self.uni.zonk(&c.ty);
            // A memo cell is single-threaded by construction: a
            // thread-shared memo needs an explicit synchronized
            // structure, not `Lazy` (lazy specification, sections 5 and
            // 9's required rejection cases).
            if crate::types::mentions_memo(&t) {
                return Err(Error::at(
                    c.span,
                    format!(
                        "a thread-shared memo cell cannot cross a task boundary: {} holds a `Lazy` cell, which is single-threaded by construction (a shared memo needs an explicit synchronized structure) (lazy specification, section 9)",
                        c.name
                    ),
                ));
            }
            let mut borrows = Vec::new();
            borrow_regions(&t, &mut borrows);
            if borrows.iter().any(|(_, is_mut)| *is_mut) {
                return Err(Error::at(
                    c.span,
                    format!(
                        "an exclusive `Mut` borrow cannot cross a task boundary: {} holds one (concurrency specification, section 3)",
                        c.name
                    ),
                ));
            }
            let mut regions = Vec::new();
            escape_regions(&t, &mut regions);
            for r in &regions {
                if c.active.contains(r) && !c.outer.contains(r) {
                    return Err(Error::at(
                        c.span,
                        format!(
                            "region escape: {} is captured by a task that outlives region {r} (the borrow's region is released inside the task's scope)",
                            c.name
                        ),
                    ));
                }
            }
            // A captured task handle or scope value belongs to this
            // task's scope or to a scope that encloses it.
            let mut names = Vec::new();
            all_regions(&t, &mut names);
            let scope_pos = c.scopes.iter().position(|s| *s == c.scope);
            for r in &names {
                let is_scope = c.scopes.iter().any(|s| s == r);
                if !is_scope {
                    continue;
                }
                let outlives = r == &c.scope
                    || match (scope_pos, c.scopes.iter().position(|s| s == r)) {
                        (Some(sp), Some(rp)) => rp < sp,
                        _ => false,
                    };
                if !outlives {
                    return Err(Error::at(
                        c.span,
                        format!(
                            "region escape: {} belongs to a scope that does not outlive the task's scope (region {r}) (concurrency specification, section 2)",
                            c.name
                        ),
                    ));
                }
            }
            let transferred = c.transfer_allowed && matches!(
                self.effective_mult(c.binder),
                Some(Mult::One) | Some(Mult::Aff)
            );
            let safe = if transferred {
                self.structural_usage(&t, &c.dicts, &mut Vec::new(), StructuralUsage::Transfer)
            } else {
                self.share_safe(&t, &c.dicts)
            };
            if !safe {
                return Err(Error::at(
                    c.span,
                    format!(
                        "a task capture must be `Copy`, a shared `Ref`, or transferred exactly once: {} is shared across tasks without shareable evidence (concurrency specification, section 3)",
                        c.name
                    ),
                ));
            }
        }
        Ok(())
    }

    /// The parallel-callback capture rules (`design/parallel-evaluation.md`
    /// section 2.3), once every binder type is resolved. A callback's
    /// captures are read from the caller's frame and released when the call
    /// returns, so nothing escapes a region and no scope rule applies. What
    /// remains is what the runtime can promise: the branch never writes a
    /// capture, so a capture is either shared (every participant reads it),
    /// or moved into the one branch that runs once.
    fn check_parallel_captures(&mut self) -> Result<()> {
        for c in std::mem::take(&mut self.par_captures) {
            let t = self.uni.zonk(&c.ty);
            if crate::types::mentions_memo(&t) {
                return Err(Error::at(
                    c.span,
                    format!(
                        "a parallel capture cannot be a memo cell: {} holds a `Lazy` cell, which is single-threaded by construction (design/parallel-evaluation.md section 2.3)",
                        c.name
                    ),
                ));
            }
            let mut borrows = Vec::new();
            borrow_regions(&t, &mut borrows);
            if borrows.iter().any(|(_, is_mut)| *is_mut) {
                return Err(Error::at(
                    c.span,
                    format!(
                        "an exclusive `Mut` borrow cannot enter a parallel callback: {} holds one (design/parallel-evaluation.md section 2.3)",
                        c.name
                    ),
                ));
            }
            let additive = matches!(
                self.effective_mult(c.binder),
                Some(Mult::One) | Some(Mult::Aff)
            );
            let safe = if !c.many_shot && additive {
                self.structural_usage(&t, &c.dicts, &mut Vec::new(), StructuralUsage::Transfer)
            } else {
                self.share_safe(&t, &c.dicts)
            };
            if safe {
                continue;
            }
            // A many-shot callback runs once per index, so a value it
            // consumes would be consumed many times; only sharing is left.
            if c.many_shot && additive {
                return Err(Error::at(
                    c.span,
                    format!(
                        "a {} callback runs once per index, so {} cannot move into it: a parallel capture must be Share (design/parallel-evaluation.md section 2.3)",
                        c.op, c.name
                    ),
                ));
            }
            return Err(Error::at(
                c.span,
                format!(
                    "a parallel capture must be Share, or move into one parBoth branch: {} is shared with a {} callback without shareable evidence (design/parallel-evaluation.md section 2.3)",
                    c.name, c.op
                ),
            ));
        }
        Ok(())
    }

    /// The `Share`-like evidence for a task capture (concurrency
    /// specification, section 3): structural `Copy`, or a class
    /// dictionary over shareable evidence (an immutable method record).
    fn share_safe(&self, t: &Ty, dicts: &[(LocalId, ClassApp)]) -> bool {
        match t {
            Ty::Adt(name, args)
                if self.class_sigs.contains_key(name.as_str())
                    && (name.contains('.') || self.lookup_type(&[name]).is_err()) =>
            {
                args.iter().all(|a| self.share_safe(a, dicts))
            }
            _ => self.structural_usage(t, dicts, &mut Vec::new(), StructuralUsage::Share),
        }
    }

    /// A parameter position that may receive a task handle must consume
    /// it exactly once (concurrency specification, section 2): a sink
    /// that could drop the handle would silently detach execution.
    fn check_handle_param(&self, dom: &Ty, m: Mult, span: ast::Span) -> Result<()> {
        if matches!(m, Mult::One | Mult::Flex) {
            return Ok(());
        }
        let t = self.uni.zonk(dom);
        if crate::types::mentions_task(&t) {
            return Err(Error::at(
                span,
                format!(
                    "a task handle must be consumed exactly once by its function: this argument position is {m}, so dropping it would silently detach execution (declare the parameter `%1`) (concurrency specification, section 2)"
                ),
            ));
        }
        Ok(())
    }

    /// The effective multiplicity of a binder: its declared class, or the
    /// type-directed default once the type is determined. A task handle
    /// is always exactly-once (concurrency specification, section 2).
    fn effective_mult(&self, l: LocalId) -> Option<Mult> {
        let ob = self.obligations.get(&l)?;
        let t = self.uni.zonk(&ob.ty);
        if crate::types::mentions_task(&t) {
            return Some(Mult::One);
        }
        Some(match ob.mult {
            Some(m) if m != Mult::Flex => m,
            _ => default_mult(&t),
        })
    }

    /// Can a value retain a closure whose captured regions are absent
    /// from its visible type? Follow actual constructor field types, not
    /// just type arguments: a monomorphic wrapper can hold a function too.
    fn value_may_hide_scope(&self, ty: &Ty, seen: &mut Vec<Ty>, fuel: usize) -> bool {
        if fuel == 0 { return true; }
        let ty = self.uni.zonk(ty);
        if seen.contains(&ty) { return false; }
        match &ty {
            Ty::Fun(..) | Ty::Pi { .. } | Ty::Handler { .. }
            | Ty::Thunk { .. } | Ty::Later(_) | Ty::Lazy { .. }
            | Ty::Meta(_) | Ty::Param(_) | Ty::App(..) | Ty::Former(_) => true,
            // A polymorphic value is a block of closures.
            Ty::Forall(_) => true,
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => self.value_may_hide_scope(inner, seen, fuel - 1),
            Ty::Tuple(items) => items.iter().any(|item| self.value_may_hide_scope(item, seen, fuel - 1)),
            Ty::Sigma { dom, cod, .. } => self.value_may_hide_scope(dom, seen, fuel - 1)
                || self.value_may_hide_scope(cod, seen, fuel - 1),
            Ty::Adt(name, args) => {
                if is_capability_class(name) || name == "Scope" || name == "Copy" { return false; }
                if name == "Task" { return true; }
                if matches!(name.as_str(), "Array" | "Tensor" | "Columnar" | "TVar") {
                    return args.iter().any(|arg| self.value_may_hide_scope(arg, seen, fuel - 1));
                }
                seen.push(ty.clone());
                // The nominal already carries its defining identity. Surface lookup
                // cannot resolve private or qualified checked field types here.
                let info = match self.lookup_known_type(name) { Ok(info) => info, Err(_) => return true };
                if self.codata_types.contains(&(info.module.clone(), info.name.clone())) || info.ctors.is_empty() {
                    return true;
                }
                for (name, _) in &info.ctors {
                    let ctor = match self.ctor_of_type(&info, name) { Ok(ctor) => ctor, Err(_) => return true };
                    let parts = match self.ctor_parts(&ctor, args) { Ok(parts) => parts, Err(_) => return true };
                    if parts.fields.iter().any(|(_, field, erased)| !erased && self.value_may_hide_scope(field, seen, fuel - 1)) {
                        return true;
                    }
                }
                false
            }
            Ty::Int | Ty::Nat | Ty::Bool | Ty::Unit | Ty::Bytes | Ty::Sym
            | Ty::Prim(_) | Ty::Region(_) | Ty::Ptr { .. } | Ty::Sort(_) | Ty::Idx(_) | Ty::Prp(_) => false,
        }
    }

    /// Structural `Copy` evidence (ownership and usage design, section 3):
    /// machine scalars and the mathematical types, immutable products of
    /// `Copy` parts, and shared borrows. `Bytes` is a mutable buffer;
    /// closures, thunks, memo cells and `Mut` borrows are not duplicable.
    fn copy_structural<B>(
        &self,
        ty: &Ty,
        scope: &[(B, ClassApp)],
        seen: &mut Vec<(String, Vec<Ty>)>,
    ) -> bool {
        self.structural_usage(ty, scope, seen, StructuralUsage::Copy)
    }

    fn structural_usage<B>(
        &self,
        ty: &Ty,
        scope: &[(B, ClassApp)],
        seen: &mut Vec<(String, Vec<Ty>)>,
        usage: StructuralUsage,
    ) -> bool {
        let transferring = usage == StructuralUsage::Transfer;
        match ty {
            Ty::Int | Ty::Nat | Ty::Bool | Ty::Unit | Ty::Prim(_) => true,
            Ty::Ref(_, inner) => match usage {
                StructuralUsage::Copy => true,
                StructuralUsage::Share => self.structural_usage(inner, scope, seen, usage),
                // Moving a reference moves its handle. Its referent can
                // remain aliased outside the task, including through an
                // actual constructor field of a transferred value.
                StructuralUsage::Transfer => {
                    self.structural_usage(inner, scope, &mut Vec::new(), StructuralUsage::Share)
                }
            },
            Ty::Mut(_, _) | Ty::Lazy { .. } => false,
            Ty::Bytes | Ty::Meta(_) | Ty::Thunk { .. } => transferring,
            // `Sym` is a handle to runtime-owned interned state; the
            // specification grants it `Eq`/`Ord`/`Show` only (primitive
            // types specification section 9), never `Copy`.
            Ty::Sym => transferring,
            // An owned boundary pointer transfers ownership exactly once
            // (foreign function interface specification section 3), so it
            // is not duplicable; raw and borrow roles are plain addresses
            // and carry no ownership to duplicate.
            Ty::Ptr { role: PtrRole::Owned(_), .. } => transferring,
            Ty::Ptr { .. } => true,
            // `Later` keeps the thunk representation at runtime (codata
            // and guardedness design, section 6), so like a thunk it is
            // not duplicable evidence.
            Ty::Later(_) => transferring,
            Ty::Fun(_, _, _, _) | Ty::Pi { .. } | Ty::Handler { .. } => transferring,
            // A polymorphic value is a dictionary of closures: like a
            // function, it is not duplicable evidence.
            Ty::Forall(_) => transferring,
            // Index values and erased propositions occupy no runtime
            // representation that could be duplicated (core specification
            // section 3).
            Ty::Sort(_) | Ty::Idx(_) | Ty::Prp(_) => true,
            Ty::Sigma { dom, cod, .. } => {
                self.structural_usage(dom, scope, seen, usage)
                    && self.structural_usage(cod, scope, seen, usage)
            }
            Ty::Region(_) => true,
            // Copy evidence for an unknown type can describe a shallow
            // reference copy. Task sharing needs the checked structure.
            Ty::Param(p) => {
                transferring || usage == StructuralUsage::Copy
                    && scope.iter().any(|(_, c)| {
                        c.class == "Copy"
                            && c.args.iter().any(|a| matches!(a, Ty::Param(q) if q == p))
                    })
            }
            // A higher-kinded application is duplicable only under an
            // explicit `Copy` constraint on the applied type itself; a
            // bare type former is never a value's type.
            Ty::App(..) | Ty::Former(_) => {
                transferring || usage == StructuralUsage::Copy
                    && scope
                        .iter()
                        .any(|(_, c)| c.class == "Copy" && c.args.iter().any(|a| a == ty))
            }
            Ty::Tuple(items) => items
                .iter()
                .all(|i| self.structural_usage(i, scope, seen, usage)),
            Ty::Adt(name, args) => {
                if matches!(name.as_str(), "Array" | "Tensor" | "Columnar") {
                    return args.iter().all(|a| self.structural_usage(a, scope, seen, usage));
                }
                // A transactional variable is a handle to a cell every
                // holder of the handle reads and writes. The handle itself
                // is duplicable. Across a task boundary the cell's value
                // is reached from both sides whether the handle is shared
                // or moved (another copy of it may stay behind), so both
                // need the value's type to be safe to share.
                if name == "TVar" {
                    return usage == StructuralUsage::Copy
                        || args.iter().all(|a| self.structural_usage(a, scope, &mut Vec::new(), StructuralUsage::Share));
                }
                if seen.iter().any(|(n, a)| n == name && a == args) {
                    return true;
                }
                seen.push((name.clone().to_string(), args.clone()));
                let info = match self.lookup_known_type(name) {
                    Ok(i) => i,
                    Err(_) => return transferring,
                };
                // A codata value is a record of observation closures: the
                // closures themselves are never `Copy`.
                if self.codata_types.contains(&(info.module.clone(), info.name.clone())) {
                    return transferring;
                }
                for (cname, _) in &info.ctors {
                    let v = match self.ctor_of_type(&info, cname) {
                        Ok(v) => v,
                        Err(_) if transferring => continue,
                        Err(_) => return false,
                    };
                    let parts = match self.ctor_parts(&v, args) {
                        Ok(parts) => parts,
                        Err(_) if transferring => continue,
                        Err(_) => return false,
                    };
                    if !parts.fields.iter().all(|(_, field, _)| {
                        self.structural_usage(field, scope, seen, usage)
                    }) {
                        return false;
                    }
                }
                true
            }
        }
    }

    #[track_caller]
    fn fresh_local(&mut self) -> LocalId {
        let l = LocalId(self.next_local);
        self.next_local += 1;
        if cfg!(debug_assertions) {
            self.local_origin.insert(l, std::panic::Location::caller());
        }
        l
    }

    fn fresh_param(&mut self, base: &str) -> String {
        let n = self.param_counter;
        self.param_counter += 1;
        // The counter alone makes the name unique, so a rigid copy of an
        // already fresh name keeps its source spelling (`a#f3` becomes
        // `a#17`, not `a#f3#17`).
        let base = base.split('#').next().unwrap_or(base);
        format!("{base}#{n}")
    }

    // ------------------------------------------------------------ literals

    /// Elaborate an integer literal against its expected type (primitive
    /// types specification, section 7). A known expected type materialises
    /// the literal immediately; a metavariable defers the constant choice
    /// to [`Checker::resolved_literals`].
    fn lit_check(&mut self, expected: &Ty, digits: &str, span: ast::Span) -> Result<ECore> {
        let t = self.uni.zonk(expected);
        // A refinement whose carrier is still a metavariable (`4` in
        // `7 % 4` before the context fixes the class's type) defers like
        // a bare metavariable: the carrier decides the constant and the
        // predicate's decision both.
        let open = match &t {
            Ty::Meta(_) => true,
            _ => self
                .refine_view(&t)
                .and_then(|v| v.base)
                .is_some_and(|b| matches!(self.uni.zonk(&b), Ty::Meta(_))),
        };
        if open {
            let idx = self.literal_pends.len();
            self.literal_pends.push(PendLit { ty: t, value: digits.to_string(), span });
            return Ok(ECore::Lit(idx));
        }
        self.literal_const(&t, digits, span)
    }

    /// The constant of literal `digits` at type `ty`. `Nat` and `Int` are
    /// mathematical: the digits become a `ConstBig` (a runtime immediate
    /// or bignum word), and a `Nat` literal must not be negative. `Sym` names
    /// the interned cell of the spelling (primitive types specification
    /// section 9). Every other type range-checks to a canonical `i64` word
    /// (section 7: out-of-range literals are compile errors).
    fn literal_const(&self, ty: &Ty, digits: &str, span: ast::Span) -> Result<ECore> {
        match ty {
            Ty::Nat => {
                if digits.starts_with('-') {
                    return Err(Error::at(
                        span,
                        format!("integer literal {digits} is out of range for Nat"),
                    ));
                }
                Ok(ECore::ConstBig { nat: true, digits: digits.to_string() })
            }
            Ty::Int => Ok(ECore::ConstBig { nat: false, digits: digits.to_string() }),
            Ty::Sym => Ok(ECore::ConstSym(digits.to_string())),
            Ty::Adt(..) if self.refine_view(ty).is_some() => self.literal_refined(ty, digits, span),
            t => Ok(ECore::Int(self.literal_word(t, digits, span)?)),
        }
    }

    /// A numeral at a named refinement of a numeric container (`10` at
    /// `NonZero Nat`, `3` at `Small`): the literal is its container's
    /// constant (the refinement's runtime representation IS the
    /// container), and the predicate instantiated at the literal's value
    /// is decided here, at compile time, by the same numeral folding
    /// every static obligation uses. A literal that satisfies it needs
    /// no witness and no runtime check; one that refutes it (`0` at
    /// `NonZero Nat`) is a compile error naming the predicate, never a
    /// refined value.
    fn literal_refined(&self, ty: &Ty, digits: &str, span: ast::Span) -> Result<ECore> {
        let not_numeric = || {
            Error::at(span, crate::diagnostic!("integer literal cannot have type {}", ty))
        };
        // Anonymous and named refinements already carry the actual checked
        // binder in their view. Looking it up only in the named-type table
        // loses the binder of `refined P T` and rejects valid literals.
        let Some(RefineView { base: Some(base), pred, binder }) = self.refine_view(ty) else {
            return Err(not_numeric());
        };
        // The container's own literal rules apply first (a negative
        // numeral is out of range for `Nat`; a non-numeric container
        // takes no integer literal at all).
        let k = self.literal_const(&self.uni.zonk(&base), digits, span)?;
        let stmt = Checker::subst_pt(
            &pred,
            &HashMap::from([(binder, Ty::Idx(Box::new(ITerm::Num(digits.to_string().into()))))]),
        );
        match self.static_obligation(&stmt)? {
            ObligStatus::Checked => Ok(k),
            ObligStatus::Disproved => Err(Error::at(
                span,
                crate::diagnostic!(
                    "integer literal {} is not a {}: its predicate `{}` is false (a refined value needs its predicate to hold)",
                    digits, ty, stmt
                ),
            )),
            _ => Err(Error::at(
                span,
                crate::diagnostic!(
                    "integer literal {} at {}: its predicate `{}` cannot be decided at compile time (construct the value with `decide` and the transparent constructor)",
                    digits, ty, stmt
                ),
            )),
        }
    }

    /// The canonical word of literal `digits` at the machine scalar type
    /// `ty` (section 7: out-of-range literals are compile errors). Float
    /// words are the IEEE bit pattern of the value, which must be exactly
    /// representable. `Nat` and `Int` never take this path: they carry
    /// their digits to the runtime ([`Checker::literal_const`]).
    fn literal_word(&self, ty: &Ty, digits: &str, span: ast::Span) -> Result<i64> {
        match ty {
            Ty::Int | Ty::Nat => {
                unreachable!("mathematical literals materialise through literal_const")
            }
            Ty::Prim(s) if s.is_int_like() => {
                // `Usize`/`Isize` take their width from the target
                // profile (primitive types specification section 2: the
                // target's width, never the build host's).
                let w = match s {
                    Scalar::Usize | Scalar::Isize => self.usize_bits,
                    other => other.width(),
                };
                let (lo, hi) = if s.is_signed() {
                    (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
                } else {
                    (0i128, (1i128 << w) - 1)
                };
                match literal_i128(digits) {
                    Some(v) if v >= lo && v <= hi => Ok(v as i64),
                    _ => Err(Error::at(
                        span,
                        format!("integer literal {digits} is out of range for {s}"),
                    )),
                }
            }
            Ty::Prim(Scalar::Char) => {
                // An integer literal at `Char` names a code point and must
                // be a scalar value.
                let bad = |v: i128| v < 0 || v > 0x10FFFF || (0xD800..=0xDFFF).contains(&v);
                match literal_i128(digits) {
                    Some(v) if !bad(v) => Ok(v as i64),
                    _ => Err(Error::at(
                        span,
                        format!(
                            "integer literal {digits} is not a Unicode scalar value (Char is 0x-0xD7FF, 0xE000-10FFFF)"
                        ),
                    )),
                }
            }
            Ty::Prim(s @ (Scalar::F32 | Scalar::F64)) => {
                // Exact representability of the integer (section 7): the
                // significand must hold the value's odd part and the binary
                // exponent must fit.
                let (p, emax) = if *s == Scalar::F32 { (24, 127) } else { (53, 1023) };
                if !float_exact_digits(digits.trim_start_matches('-'), p, emax) {
                    return Err(Error::at(
                        span,
                        format!(
                            "integer literal {digits} is not exactly representable in {s}; use a conversion"
                        ),
                    ));
                }
                let f: f64 = digits.parse().map_err(|_| {
                    Error::at(span, format!("integer literal {digits} is not a number"))
                })?;
                Ok(if *s == Scalar::F32 {
                    (f as f32).to_bits() as i64
                } else {
                    f.to_bits() as i64
                })
            }
            other => Err(Error::at(
                span,
                crate::diagnostic!("integer literal cannot have type {}", other),
            )),
        }
    }

    /// The metavariables of the literals still awaiting determination; they
    /// stay monomorphic through let-generalization (one literal has one
    /// type, section 7). A literal's meta may have unified with another
    /// meta since, so the reported metas are the zonked ones.
    fn literal_metas(&self) -> Vec<EitherMeta> {
        let mut out: Vec<EitherMeta> = Vec::new();
        for p in &self.literal_pends {
            for m in self.uni.free_metas(&self.uni.zonk(&p.ty)) {
                if !out.contains(&m) {
                    out.push(m);
                }
            }
        }
        out
    }

    /// The resolved constant words of one definition's deferred literals
    /// (section 7: a literal whose type is not determined by context is
    /// an error in this draft), positional against the `ECore::Lit`
    /// indices shared by the body and its `letrec` member units.
    fn resolved_literals(&mut self) -> Result<Vec<ECore>> {
        let pends = std::mem::take(&mut self.literal_pends);
        let mut lits: Vec<ECore> = Vec::with_capacity(pends.len());
        for p in &pends {
            let t = self.uni.zonk(&p.ty);
            if matches!(t, Ty::Meta(_)) {
                return Err(Error::at(
                    p.span,
                    "the type of this integer literal is not determined by context; add a type annotation",
                ));
            }
            lits.push(self.literal_const(&t, &p.value, p.span)?);
        }
        Ok(lits)
    }

    /// The resolved runtime code of one definition's deferred implicit
    /// index arguments (decision D29), positional against the
    /// `ECore::IdxPending` indices shared by the body and its `letrec`
    /// member units.
    fn resolved_idx_args(&mut self) -> Result<Vec<ECore>> {
        let pends = std::mem::take(&mut self.idx_pends);
        let mut vals: Vec<ECore> = Vec::with_capacity(pends.len());
        for p in &pends {
            vals.push(self.reify_index_value(&p.iterm, &p.scope, &p.carrier, p.span)?);
        }
        Ok(vals)
    }

    /// The collection literal fold of one unit (surface grammar local
    /// amendments section 10): a stage-known construction evaluates
    /// through the D92 evaluator and reifies to a constant value,
    /// replacing the class call; any other construction keeps its runtime
    /// `from-list`/`from-map` call. Each record in `literal_folds` fills
    /// with the unit and the fold boundary. `fold` is false inside a
    /// quotation's retained body: its reification cannot carry a constant
    /// buffer yet, so those sites keep their class calls and record the
    /// boundary.
    fn fold_pass(&mut self, e: &mut ECore, unit: &str, fold: bool) {
        match e {
            ECore::LitFold { idx, body } => {
                let idx = *idx;
                let result_ty = self.uni.zonk(&self.literal_folds[idx].result_ty);
                match if fold { self.fold_constant(body, &result_ty) } else { None } {
                    Some(k) => {
                        self.literal_folds[idx].folded = true;
                        self.literal_folds[idx].unit = unit.to_string();
                        *e = k;
                    }
                    None => {
                        self.fold_pass(body, unit, fold);
                        self.literal_folds[idx].unit = unit.to_string();
                    }
                }
            }
            ECore::Int(_)
            | ECore::ConstBig { .. }
            | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
            | ECore::Lit(_)
            | ECore::Bool(_)
            | ECore::Unit
            | ECore::Var(_)
            | ECore::Global(..)
            | ECore::Ctor(..)
            | ECore::Prim(_)
            | ECore::Foreign(_)
            | ECore::Hole(_)
            | ECore::QuoteHole(_)
            | ECore::IdxPending(_)
            | ECore::Fault
            | ECore::SyntaxLit(..)
            | ECore::StrLit(_)
            | ECore::LazyAlloc
            | ECore::BoxAlloc => {}
            ECore::Quote { holes, body, .. } => {
                for h in holes {
                    self.fold_pass(h, unit, fold);
                }
                self.fold_pass(body, unit, false);
            }
            ECore::DeclsQuote { holes, .. } => {
                for h in holes {
                    self.fold_pass(h, unit, fold);
                }
            }
            ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) => {
                self.fold_pass(d, unit, fold)
            }
            ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
                for a in args {
                    self.fold_pass(a, unit, fold);
                }
            }
            ECore::App(f, args) => {
                self.fold_pass(f, unit, fold);
                for a in args {
                    self.fold_pass(a, unit, fold);
                }
            }
            ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => self.fold_pass(body, unit, fold),
            ECore::Let(_, v, body) => {
                self.fold_pass(v, unit, fold);
                self.fold_pass(body, unit, fold);
            }
            ECore::If(c, t, f) => {
                self.fold_pass(c, unit, fold);
                self.fold_pass(&mut t.body, unit, fold);
                self.fold_pass(&mut f.body, unit, fold);
            }
            ECore::LazyInit(_, t) => self.fold_pass(t, unit, fold),
            ECore::BoxInit(_, v, _) => self.fold_pass(v, unit, fold),
            ECore::RefSet(_, v, _) => self.fold_pass(v, unit, fold),
            ECore::BoxGet(v, _) | ECore::BoxClear(v, _) => self.fold_pass(v, unit, fold),
            ECore::Case { scrut, alts } => {
                self.fold_pass(scrut, unit, fold);
                for alt in alts {
                    self.fold_pass(&mut alt.body, unit, fold);
                }
            }
        }
    }

    /// The constant a collection literal's construction folds to, when
    /// it is stage-known and evaluates to a constant value. Anything
    /// else (free runtime values, code the evaluator cannot run, a value
    /// with no constant form) keeps the runtime class call.
    fn fold_constant(&mut self, body: &ECore, ty: &Ty) -> Option<ECore> {
        let mut free = Vec::new();
        free_locals(body, &mut Vec::new(), &mut free);
        if !free.is_empty() || fold_blocked(body) {
            return None;
        }
        let own = HashSet::new();
        let core = lower(body, &LowerCx::compile_time(self, &[], &own));
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let mut frame: Vec<crate::core::Value> = Vec::new();
        // A construction that does not evaluate at compile time (or
        // exceeds the fold's budget) computes at runtime, exactly as it
        // would without the fold.
        let v = self.eval_core(&mut cx, &core, &mut frame).ok()?;
        let string_ctor = self.string_ctor_id().0;
        self.reify_constant(&v, ty, &string_ctor)
    }

    /// Preserve constructor instantiations when folding a collection. A
    /// type-erased constructor defaults its fields to Raw, which leaks
    /// bignums and objects that the list or map owns.
    fn reify_constant(&self, value: &crate::core::Value, ty: &Ty, string_ctor: &str) -> Option<ECore> {
        // A generalized literal can still carry an unsolved original
        // metavariable. Preserve its typed runtime construction instead
        // of letting an unknown constructor argument default to Raw.
        if !self.uni.free_metas(ty).is_empty() { return None; }
        let crate::core::Value::Adt { ctor, fields, .. } = value else {
            return reify_value(value, string_ctor);
        };
        if ctor == string_ctor { return reify_value(value, string_ctor); }
        let (args, field_types) = match self.uni.zonk(ty) {
            Ty::Tuple(types) => (types.clone(), types),
            Ty::Adt(name, args) => {
                let info = self.lookup_known_type(&name).ok()?;
                let cname = ctor.rsplit('.').next()?;
                let defining = self.ctor_of_type(&info, cname).ok()?;
                if format!("{}.{}", defining.module, defining.name) != *ctor { return None; }
                let parts = self.ctor_parts(&defining, &args).ok()?;
                if !parts.local_idx.is_empty() { return None; }
                let inst = args.iter().zip(&info.param_kinds).filter_map(|(a, kind)|
                    matches!(kind, ParamKind::Type | ParamKind::Con(_)).then(|| a.clone())).collect();
                (inst, parts.fields.into_iter().filter_map(|(_, ty, erased)| (!erased).then_some(ty)).collect())
            }
            _ => return None,
        };
        if fields.len() != field_types.len() { return None; }
        let mut out = ECore::Ctor(GlobalId(ctor.clone()), args);
        for (field, ty) in fields.iter().zip(field_types) {
            out = app_spine(out, vec![self.reify_constant(field, &ty, string_ctor)?]);
        }
        Some(out)
    }

    fn reify_index_value(&self, value: &ITerm, names: &IndexRuntimeScope, carrier: &Ty, span: ast::Span) -> Result<ECore> {
        let carrier = self.uni.zonk(carrier);
        let carrier = self.refine_base(&carrier).unwrap_or(carrier);
        if let ITerm::Num(digits) = self.uni.resolve_idx(value) {
            if carrier == Ty::Prim(Scalar::Char) {
                let code = digits.parse::<u32>().ok().filter(|code| char::from_u32(*code).is_some())
                    .ok_or_else(|| Error::at(span, "index character is not a Unicode scalar value"))?;
                return Ok(ECore::Int(code as i64));
            }
            return self.literal_const(&carrier, &digits, span);
        }
        self.reify_idx(value, names, carrier == Ty::Nat, span)
    }

    /// Read the complete checked telescope using declaration-owned parameter
    /// names. Erased arguments remain metadata and never enter the runtime ABI.
    fn index_instantiation_bindings(scheme: &Scheme, call: &IndexInstantiation, span: ast::Span) -> Result<(HashMap<String, Ty>, HashMap<String, EffRow>, HashMap<String, String>, HashMap<IndexEvidence, ITerm>)> {
        let expected = scheme.idx_params.len() + scheme.val_params.len() + scheme.prop_params.len() + scheme.row_params.len() + scheme.region_params.len() + scheme.evidence_params.len();
        if scheme.params.len() != call.types.len() || scheme.ctx.len() != call.dictionaries.len() || expected != call.arguments.len() {
            return Err(Error::at(span, "index function has an incomplete checked telescope"));
        }
        let mut types: HashMap<_, _> = scheme.params.iter().cloned().zip(call.types.iter().cloned()).collect();
        let mut evidence = HashMap::new();
        for ((name, _), dictionary) in scheme.ctx.iter().zip(&call.dictionaries) {
            evidence.insert(dictionary_parameter(name), dictionary.clone());
        }
        let mut arguments = call.arguments.iter();
        for parameter in &scheme.idx_params {
            let value = match (parameter, arguments.next().expect("length checked")) {
                (IxParam::Val(..), IndexArgument::Value { value, runtime: false, .. }) => Ty::Idx(Box::new(value.clone())),
                (IxParam::Prop(..), IndexArgument::Proposition(prop)) => Ty::Prp(Box::new(prop.clone())),
                _ => return Err(Error::at(span, "index parameter kind does not match its checked telescope")),
            };
            types.insert(parameter.name().to_string().into(), value);
        }
        for (name, _) in &scheme.val_params {
            let IndexArgument::Value { value, runtime: true, .. } = arguments.next().expect("length checked") else {
                return Err(Error::at(span, "implicit value does not match its checked telescope"));
            };
            types.insert(name.clone(), Ty::Idx(Box::new(value.clone())));
        }
        for name in &scheme.prop_params {
            let IndexArgument::Proposition(prop) = arguments.next().expect("length checked") else {
                return Err(Error::at(span, "proposition does not match its checked telescope"));
            };
            types.insert(name.clone(), Ty::Prp(Box::new(prop.clone())));
        }
        let mut rows = HashMap::new();
        for name in &scheme.row_params {
            let IndexArgument::Row(row) = arguments.next().expect("length checked") else {
                return Err(Error::at(span, "effect row does not match its checked telescope"));
            };
            rows.insert(name.clone(), row.clone());
        }
        let mut regions = HashMap::new();
        for name in &scheme.region_params {
            let IndexArgument::Region(region) = arguments.next().expect("length checked") else {
                return Err(Error::at(span, "region does not match its checked telescope"));
            };
            regions.insert(name.clone(), region.to_string());
        }
        for parameter in &scheme.evidence_params {
            let IndexArgument::Evidence(value) = arguments.next().expect("length checked") else {
                return Err(Error::at(span, "dictionary identity does not match its checked telescope"));
            };
            evidence.insert(parameter.clone(), value.clone());
        }
        Ok((types, rows, regions, evidence))
    }

    fn instantiated_index_type(&self, scheme: &Scheme, call: &IndexInstantiation, span: ast::Span) -> Result<Ty> {
        let (types, rows, regions, evidence) = Self::index_instantiation_bindings(scheme, call, span)?;
        // A private copy: this runs with the checker only borrowed, and the
        // row instantiation must not bind anything in the checker's store.
        let mut uni = self.uni.clone();
        let mut tails = HashMap::new();
        for (name, row) in rows {
            let fresh = uni.fresh_row();
            uni.unify_rows(&fresh, &row).map_err(|error| Error::at(span, error))?;
            tails.insert(name, fresh.tail);
        }
        let ty = uni.zonk(&Self::subst_namespaces(&scheme.ty, &types, &tails, &regions, &evidence));
        if !self.uni.owns_all_metas(&ty) {
            return Err(Error::at(span, "instantiated index carrier retains private inference variables"));
        }
        Ok(ty)
    }

    fn index_function_carrier(&self, term: &ITerm, span: ast::Span) -> Result<Option<Ty>> {
        match self.uni.resolve_idx(term) {
            ITerm::LocalValue(local) => Ok(self.local_tys.get(&local).map(|ty| self.uni.zonk(ty))),
            ITerm::Inst(call) => match &call.head {
                IndexHead::Function(head) => self.qualified.values().find(|value| value_names_head(value, head))
                    .map(|value| self.instantiated_index_type(&value.scheme, &call, span)).transpose(),
                _ => Ok(None),
            },
            ITerm::Fn(head) if head == BYTE_LENGTH_INDEX => Ok(Some(Ty::fun(Ty::Bytes, Ty::Nat))),
            ITerm::Fn(head) => Ok(self.qualified.values().find(|value| value_names_head(value, &head))
                .and_then(|value| Self::closed_index_signature(&value.scheme))),
            ITerm::App(head, args) => {
                let mut carrier = self.index_function_carrier(&head, span)?;
                for arg in args {
                    carrier = match carrier {
                        Some(Ty::Fun(_, _, _, cod)) => Some(*cod),
                        Some(Ty::Pi { var, cod, .. }) => Some(cod.subst_idx(&var, &arg)),
                        _ => None,
                    };
                }
                Ok(carrier)
            }
            _ => Ok(None),
        }
    }

    fn read_index_local(&self, local: LocalId) -> ECore {
        let value = ECore::Var(local);
        match self.local_tys.get(&local).map(|ty| self.uni.zonk(ty)) {
            Some(Ty::Mut(_, payload)) => Self::read_mutable_cell(value, &payload),
            _ => value,
        }
    }

    /// An index term as runtime code (core specification section 3:
    /// witnesses execution needs are retained, and only executable
    /// definitions can be reified).
    fn reify_idx(
        &self,
        it: &ITerm,
        names: &IndexRuntimeScope,
        nat: bool,
        span: ast::Span,
    ) -> Result<ECore> {
        Ok(match self.uni.resolve_idx(it) {
            ITerm::Num(d) => ECore::ConstBig { nat, digits: d .to_string()},
            ITerm::LocalValue(local) => {
                if !names.locals.contains(&local) {
                    return Err(Error::at(span, "index value escaped its checked runtime scope"));
                }
                self.read_index_local(local)
            }
            ITerm::Lam(_, _) => {
                return Err(Error::at(
                    span,
                    "an index-level lambda has no runtime value at this call site",
                ))
            }
            ITerm::Evidence(IndexEvidence::Local(local)) => {
                let local = LocalId(local);
                if !names.dictionaries.contains(&local) {
                    return Err(Error::at(span, "index dictionary escaped its checked runtime scope"));
                }
                ECore::Var(local)
            }
            ITerm::Evidence(IndexEvidence::Parameter(_)) => return Err(Error::at(span, "index dictionary parameter was not instantiated")),
            ITerm::Var(n) => match names.values.get(n.as_str()) {
                Some(l) => self.read_index_local(*l),
                None => {
                    return Err(Error::at(
                        span,
                        format!("the index value `{n}` has no runtime value at this call site"),
                    ))
                }
            },
            ITerm::Inst(call) => {
                let dictionaries = call.dictionaries.iter().map(|term| self.reify_idx(term, names, nat, span)).collect::<Result<Vec<_>>>()?;
                match &call.head {
                    IndexHead::Dictionary(head) => ECore::InstDict(GlobalId(head.clone()), dictionaries, call.types.clone(), false),
                    IndexHead::Marker => ECore::Unit,
                    IndexHead::DeclarationPending(_) => return Err(Error::at(span, "provisional declaration evidence cannot become runtime code")),
                    IndexHead::Function(head) => {
                        self.check_executable(head, span)?;
                        let mut arguments = dictionaries;
                        for (value, carrier) in call.runtime_values() { arguments.push(self.reify_index_value(value, names, carrier, span)?); }
                        if let Some(value) = self.qualified.values().find(|value| value_names_head(value, head)) {
                            if let ValueKind::Method { idx, .. } = value.kind {
                                if arguments.is_empty() { return Err(Error::at(span, "index method has no checked dictionary")); }
                                let dictionary = arguments.remove(0);
                                let carrier = self.instantiated_index_type(&value.scheme, &call, span)?;
                                let carrier = if arguments.is_empty() { carrier } else { Ty::fun(Ty::Unit, carrier) };
                                let function = ECore::DictProj(Box::new(dictionary), idx,
                                    self.runtime_method_site(&value.module, &value.scheme, &call.types)?, carrier);
                                app_spine(function, arguments)
                            } else { app_spine(ECore::Global(GlobalId(head.clone()), call.types.clone()), arguments) }
                        } else { return Err(Error::at(span, "instantiated index function is no longer available")); }
                    }
                }
            }
            ITerm::Fn(g) if g == BYTE_LENGTH_INDEX => ECore::Prim(PrimOp::Bytes(BytesOp::Len)),
            ITerm::Fn(g) => {
                self.check_executable(&g, span)?;
                ECore::Global(GlobalId(g.to_string()), Vec::new())
            }
            ITerm::App(f, args) => {
                let mut cores = vec![self.reify_idx(&f, names, nat, span)?];
                let mut carrier = self.index_function_carrier(&f, span)?;
                for a in &args {
                    match carrier.take() {
                        Some(Ty::Fun(domain, _, _, result)) => {
                            cores.push(self.reify_index_value(a, names, &domain, span)?);
                            carrier = Some(*result);
                        }
                        Some(Ty::Pi { var, dom, cod, .. }) => {
                            cores.push(self.reify_index_value(a, names, &dom, span)?);
                            carrier = Some(cod.subst_idx(&var, a));
                        }
                        _ => cores.push(self.reify_idx(a, names, nat, span)?),
                    }
                }
                let head = cores.remove(0);
                app_spine(head, cores)
            }
            ITerm::Bin(op, a, b) => {
                // A `Nat` index difference is defined only where the
                // subtrahend does not pass the minuend (the index
                // evaluator leaves `a - b` stuck otherwise), and an index
                // carries no evidence to the call site. So it reaches
                // runtime only as the constant it folds to; an open one
                // is refused rather than lowered to a subtraction that
                // could go below zero.
                if nat && op == IdxOp::Sub {
                    let whole = ITerm::Bin(op, a.clone(), b.clone()).normalize();
                    return match whole {
                        ITerm::Num(d) => Ok(ECore::ConstBig { nat, digits: d .to_string()}),
                        other => Err(Error::at(
                            span,
                            format!(
                                "the `Nat` index `{other}` would be computed at runtime here, but a `Nat` difference needs evidence that it is not negative, which an index cannot carry; pass the value as an ordinary argument and subtract with evidence"
                            ),
                        )),
                    };
                }
                let prim = if nat {
                    PrimOp::Nat(match op {
                        IdxOp::Add => NatOp::Add,
                        IdxOp::Sub => NatOp::Sub,
                        IdxOp::Mul => NatOp::Mul,
                    })
                } else {
                    PrimOp::Int(match op {
                        IdxOp::Add => IntOp::Add,
                        IdxOp::Sub => IntOp::Sub,
                        IdxOp::Mul => IntOp::Mul,
                    })
                };
                ECore::App(
                    Box::new(ECore::Prim(prim)),
                    vec![
                        self.reify_idx(&a, names, nat, span)?,
                        self.reify_idx(&b, names, nat, span)?,
                    ],
                )
            }
            ITerm::MVar(m) => {
                return Err(Error::at(
                    span,
                    format!(
                        "the implicit index argument ?i{m} was never determined; add a type annotation"
                    ),
                ))
            }
        })
    }

    /// A ghost (`spec`) definition has no runtime code (core
    /// specification section 4: a decision procedure must be executable).
    fn check_executable(&self, gid: &str, span: ast::Span) -> Result<()> {
        if let Some((m, n)) = gid.rsplit_once('.') {
            if self.logical.contains(&(m.to_string(), n.to_string())) {
                return Err(Error::at(
                    span,
                    format!(
                        "{gid} is a `spec` definition: it is erased ghost code and has no executable form here (obligation status: unsupported)"
                    ),
                ));
            }
        }
        Ok(())
    }

    // ------------------------------------------------------------ resolution

    /// The definition behind a module's surface name (surface grammar
    /// local amendments section 11). A module with `export` declarations
    /// exposes exactly the names they resolve to; one without exposes
    /// its declared names as before.
    fn surface_def(&self, module: &str, name: &str) -> Option<SurfaceName> {
        if let Some(m) = self.module_surface.get(module) {
            return m.get(name).cloned();
        }
        self.declared_surface_def(module, name)
    }

    /// A module's own declaration identity, independent of its previous
    /// public surface. Surface rebuilding must be able to add new outputs.
    fn declared_surface_def(&self, module: &str, name: &str) -> Option<SurfaceName> {
        let declared = self
            .values
            .get(name)
            .map(|vs| vs.iter().any(|v| v.module == module))
            .unwrap_or(false)
            || self
                .types
                .get(name)
                .map(|ts| ts.iter().any(|t| t.module == module))
                .unwrap_or(false)
            || self
                .sigs
                .get(name)
                .map(|ss| ss.iter().any(|s| s.module == module))
                .unwrap_or(false)
            || self.instance_defs_pending.iter().any(|(m, i)| m == module && i.name.text == name)
            || self.instance_list.iter().any(|i| i.module == module && i.name == name)
            || self.deferred_headers.contains_key(&(module.to_string(), name.to_string()));
        if declared {
            Some(SurfaceName {
                module: module.to_string(),
                name: name.to_string(),
            })
        } else {
            None
        }
    }

    /// The definitions a bare name may name, in scope order: the
    /// checking module's own, then each import's contribution (renaming
    /// and selection applied), then the prelude's.
    fn scope_candidates(&self, name: &str) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> =
            vec![(self.current_module.clone(), name.to_string())];
        for v in &self.scope_views {
            if v.alias_only {
                continue;
            }
            if let Some(s) = v.source_name(name) {
                if let Some(d) = self.surface_def(&v.target, s) {
                    let pair = (d.module.clone(), d.name.clone());
                    if !out.contains(&pair) {
                        out.push(pair);
                    }
                }
            }
        }
        out
    }

    /// The definitions `qual.name` may name when `qual` is a module
    /// qualifier: the checking module's own, then the matching import's
    /// (the import renaming makes the imported name the only spelling).
    fn scope_qual_candidates(&self, qual: &str, name: &str) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = Vec::new();
        if qual == self.current_module {
            out.push((self.current_module.clone(), name.to_string()));
        }
        for v in &self.scope_views {
            if v.alias != qual {
                continue;
            }
            if let Some(s) = v.source_name(name) {
                if let Some(d) = self.surface_def(&v.target, s) {
                    let pair = (d.module.clone(), d.name.clone());
                    if !out.contains(&pair) {
                        out.push(pair);
                    }
                }
            }
        }
        out
    }

    /// Whether the definition `(module, name)` is reachable through the
    /// checking module's scope (its own, or any import's surface).
    fn def_in_scope(&self, module: &str, name: &str) -> bool {
        if module == self.current_module || module == "<builtin>" {
            return true;
        }
        for v in &self.scope_views {
            let hit = match &v.members {
                None => match self.module_surface.get(&v.target) {
                    Some(m) => m.iter().any(|(local, d)| !v.hides(local) && d.module == module && d.name == name),
                    None => {
                        v.target == module && !v.hides(name) && self.surface_def(&v.target, name).is_some()
                    }
                },
                Some(ms) => ms.iter().any(|(_, s)| {
                    self.surface_def(&v.target, s)
                        .map(|d| d.module == module && d.name == name)
                        .unwrap_or(false)
                }),
            };
            if hit {
                return true;
            }
        }
        false
    }

    fn lookup_value(&self, module: Option<&str>, name: &str) -> Result<ValueInfo> {
        if let Some(m) = module {
            for (dm, dn) in self.scope_qual_candidates(m, name) {
                if let Some(v) = self
                    .values
                    .get(&dn)
                    .and_then(|vs| vs.iter().find(|v| v.module == dm))
                {
                    return Ok(v.clone());
                }
            }
            // Class-method spellings (`FromList.from-list`), dotted
            // presentation aliases (`U32.wrapping-add`) and module
            // instantiation units register their names directly; a
            // module qualifier resolves only through the scope.
            if self.module_names.contains(m) {
                return Err(Error::plain(format!("unknown name {m}.{name}")));
            }
            return self
                .qualified
                .get(&(m.to_string(), name.to_string()))
                .cloned()
                .ok_or_else(|| Error::plain(format!("unknown name {m}.{name}")));
        }
        for (dm, dn) in self.scope_candidates(name) {
            if let Some(v) = self
                .values
                .get(&dn)
                .and_then(|vs| vs.iter().find(|v| v.module == dm))
            {
                return Ok(v.clone());
            }
        }
        if let Some(v) = self
            .values
            .get(name)
            .and_then(|vs| vs.iter().find(|v| v.module == "<builtin>"))
        {
            return Ok(v.clone());
        }
        Err(Error::plain(format!("unknown name {name}")))
    }

    /// The recorded-offset access plan for one runtime field slot of a
    /// boundary-layout type (`repr(C)`, `repr(packed)`,
    /// `repr(transparent)`), from the interface ABI table's recorded
    /// offsets (attribute annotations specification section 4.3).
    /// `None` for uniform-word types, which keep the slot convention.
    pub(crate) fn field_access_plan(
        &self,
        module: &str,
        name: &str,
        slot: usize,
        field_ty: &Ty,
    ) -> Option<FieldAccess> {
        let layout = self.type_layouts.get(&(module.to_string(), name.to_string()))?;
        if layout.repr == "uniform-word" {
            return None;
        }
        let f = layout.fields.get(slot)?;
        Some(FieldAccess {
            offset: f.offset,
            size: f.size,
            shape: self.field_shape(field_ty, f.size),
            range: field_ty.value_range(),
        })
    }

    /// The stored shape of one boundary-layout field's type: machine
    /// scalars fill their recorded width; everything boxed (including
    /// pointer roles and opaque handles) is one word; a nested
    /// boundary-layout aggregate is stored inline at the recorded size.
    /// A pointer field records the alignment its role asserts of the
    /// pointee (the `!align` fact of the LLVM contract surface).
    pub(crate) fn field_shape(&self, ty: &Ty, size: u64) -> FieldShape {
        match ty {
            // `Unit` and `Bool` fill their recorded scalar width (the
            // primitive types specification section 2 table: 0 and 1
            // bytes).
            Ty::Unit | Ty::Bool => FieldShape::Scalar { size, signed: false },
            Ty::Prim(s) => {
                let float = matches!(s, Scalar::F32 | Scalar::F64);
                FieldShape::Scalar {
                    size: s.size(),
                    signed: s.is_signed() && !float,
                }
            }
            Ty::Adt(n, _) => {
                if let Some(nested) = self.inline_layout(n) {
                    nested
                } else {
                    FieldShape::Word { align: 0 }
                }
            }
            Ty::Ptr { role, pointee } => {
                let align = if matches!(
                    role,
                    crate::types::PtrRole::Raw | crate::types::PtrRole::Out
                ) {
                    0
                } else {
                    self.pointee_layout(pointee).map(|(a, _)| a).unwrap_or(0)
                };
                FieldShape::Word { align }
            }
            _ => FieldShape::Word { align: 0 },
        }
    }

    /// The inline plan for a nested boundary-layout aggregate (its own
    /// recorded layout, boxed out by `tag` when projected).
    fn inline_layout(&self, name: &str) -> Option<FieldShape> {
        let info = self.lookup_known_type(name).ok()?;
        let layout = self
            .type_layouts
            .get(&(info.module.clone(), info.name.clone()))?;
        if layout.repr == "uniform-word" || layout.fields.is_empty() {
            return None;
        }
        let tag = info.ctors.first().map(|(_, t)| *t).unwrap_or(0);
        let v = self.qualified.get(&(info.module.clone(), info.name.clone()))?;
        let (field_tys, field_erased) = match &v.kind {
            ValueKind::Ctor { field_tys, field_erased, .. } => (field_tys.clone(), field_erased.clone()),
            _ => return None,
        };
        let mut fields = Vec::new();
        let mut slot = 0;
        for (t, erased) in field_tys.iter().zip(field_erased.iter()) {
            if *erased {
                continue;
            }
            let f = layout.fields.get(slot)?;
            fields.push(FieldAccess {
                offset: f.offset,
                size: f.size,
                shape: self.field_shape(t, f.size),
                range: t.value_range(),
            });
            slot += 1;
        }
        Some(FieldShape::Inline { size: layout.size, tag, fields })
    }

    /// Record that the current declaration's check consulted the instance
    /// bucket `(class, head)`, whether or not it found anything. The
    /// declaration is the unit the checker is elaborating; a search with
    /// no enclosing declaration (a probe, a law's sample evaluation) is
    /// recorded against the empty name and belongs to no item.
    pub(crate) fn record_instance_search(&self, class: &str, head: &str) {
        self.instance_searches.borrow_mut().push((
            self.current_declaration.clone(),
            class.to_string(),
            head.to_string(),
        ));
    }

    /// Name the declaration whose check is starting. Returns the enclosing
    /// declaration's name, which the caller restores when its check ends
    /// (a staged unit compiled on demand nests inside its caller, and the
    /// rows it records belong to the outer declaration).
    pub(crate) fn enter_declaration(&mut self, decl: &str) -> String {
        std::mem::replace(&mut self.current_declaration, decl.to_string())
    }

    /// The search rows recorded so far, drained: the caller files them
    /// with the checked program.
    pub(crate) fn take_instance_searches(&mut self) -> Vec<(String, String, String)> {
        std::mem::take(self.instance_searches.get_mut())
    }

    /// The search rows the checks so far recorded, as the query graph
    /// files them: declaration item id to its `(class, head)` buckets.
    pub(crate) fn instance_search_rows(&self) -> Vec<(String, String, String)> {
        self.instance_searches.borrow().clone()
    }

    fn record_editor_reference(&self, span: ast::Span, kind: ast::RefKind, owner: &str, name: &str) {
        if !self.editor_modules.contains(&self.current_module)
            || crate::lexer::expansion_of(span).is_some()
        {
            return;
        }
        if let Some(references) = self.editor_references.borrow_mut().as_mut() {
            references.push(CheckedReference {
                module: self.current_module.clone(), owner: owner.into(), name: name.into(), kind, span,
            });
        }
    }

    fn lookup_type(&self, parts: &[&str]) -> Result<TypeKind> {
        match parts {
            [name] => {
                for (dm, dn) in self.scope_candidates(name) {
                    if let Some(t) = self
                        .types
                        .get(&dn)
                        .and_then(|ts| ts.iter().find(|t| t.module == dm))
                    {
                        return Ok(t.clone());
                    }
                }
                self.types
                    .get(*name)
                    .and_then(|ts| ts.iter().find(|t| t.module == "<builtin>"))
                    .cloned()
                    .ok_or_else(|| Error::plain(format!("unknown type {name}")))
            }
            [m, name] => {
                for (dm, dn) in self.scope_qual_candidates(m, name) {
                    if let Some(t) = self
                        .types
                        .get(&dn)
                        .and_then(|ts| ts.iter().find(|t| t.module == dm))
                    {
                        return Ok(t.clone());
                    }
                }
                if self.module_names.contains(*m) {
                    return Err(Error::plain(format!("unknown type {m}.{name}")));
                }
                self.qualified_types
                    .get(&(m.to_string(), name.to_string()))
                    .cloned()
                    .ok_or_else(|| Error::plain(format!("unknown type {m}.{name}")))
            }
            _ => Err(Error::plain("type paths deeper than Module.Type are not supported")),
        }
    }

    fn lookup_type_path(&self, path: &ast::ConPath) -> Result<TypeKind> {
        if let Some(binding) = path.segments.last().and_then(|name| name.declaration_binding()) {
            let (owner, name) = self.declaration_bindings.get(&(binding.clone(), ast::RefKind::Type))
                .ok_or_else(|| Error::at(path.span(), "owned syntax type is not bound by this declaration group"))?;
            return self.types.get(name).and_then(|types| types.iter().find(|ty| &ty.module == owner)).cloned()
                .ok_or_else(|| Error::at(path.span(), format!("unknown type {owner}.{name}")));
        }
        if let Some(binding) = path.segments.last().and_then(|n| n.resolved_binding()) {
            if binding.kind() != ast::RefKind::Type {
                return Err(Error::at(path.span(), "a resolved value is not a type name"));
            }
            return self.types.get(binding.name()).and_then(|ts| ts.iter().find(|t| t.module == binding.module()))
                .cloned().ok_or_else(|| Error::at(path.span(), "resolved syntax type is no longer available"));
        }
        if let Some(scope) = path.segments.last().and_then(|name| name.scoped_reference()) {
            let info = scope.candidates(ast::RefKind::Type).iter().find_map(|binding| {
                self.types.get(binding.name()).and_then(|ts| ts.iter().find(|t| t.module == binding.module()))
            }).cloned().ok_or_else(|| Error::at(path.span(), format!("unknown type {} (not bound in captured source scope {})", path.parts().join("."), scope.origin())))?;
            if self.record_quote_bindings { path.segments.last().unwrap().record_binding(ast::RefKind::Type, &info.module, &info.name); }
            return Ok(info);
        }
        let info = self.lookup_type(&path.parts())?;
        if self.record_quote_bindings {
            if let Some(name) = path.segments.last() {
                name.record_binding(ast::RefKind::Type, &info.module, &info.name);
            }
        }
        Ok(info)
    }

    /// Inspect an already checked type, rather than resolve source spelling.
    /// A private type can occur in checked generated code without becoming
    /// an import. AST names must still go through `lookup_type_path` (or the
    /// surface-gated `lookup_type`), so raw captured names gain no access.
    fn lookup_known_type(&self, name: &str) -> Result<TypeKind> {
        self.known_type(name).cloned().ok_or_else(|| Error::plain(format!("checked type {name} has no unambiguous definition")))
    }

    fn known_type(&self, identity: &str) -> Option<&TypeKind> {
        // Dictionary families use the declaring signature's identity, just as
        // ordinary nominal types use their defining owner below. A local data
        // carrier such as `Main.Bits` never collides with the class
        // `Algebra.Bits.Bits`. Looking up a checked identity must not depend
        // on insertion-site source imports.
        if let Some(signature) = self.class_sigs.get(identity).and_then(|rows| rows.first()) {
            return self.qualified_types.get(&(signature.module.clone(), signature.name.clone()));
        }
        let name = identity.rsplit('.').next()?;
        let kinds = self.types.get(name)?;
        let mut matching = kinds.iter().filter(|k| k.identity == identity);
        let first = matching.next()?;
        if matching.next().is_none() { return Some(first); }
        // Dictionary types still use the class identity selected by the
        // class scope. Ordinary nominal types have unique owner identities.
        for (module, name) in self.scope_candidates(name) {
            if let Some(kind) = kinds.iter().find(|k| k.module == module && k.name == name && k.identity == identity) {
                return Some(kind);
            }
        }
        None
    }

    fn signature_at(&self, owner: &str, name: &str) -> Option<SigInfo> {
        if owner == "<builtin>" && matches!(name, "Copy" | "Share" | "Io") {
            let (name, parameter) = if name == "Io" { ("Io", "r") } else { ("Copy", "a") };
            return Some(SigInfo {
                module: "<builtin>".into(), name: name.into(),
                params: vec![parameter.into()], param_kinds: vec![ParamKind::Type],
                methods: Vec::new(), defaults: Vec::new(), laws: Vec::new(),
            });
        }
        self.qualified_sigs.get(&(owner.into(), name.into())).cloned()
    }

    fn lookup_sig(&self, path: &ast::ConPath) -> Result<SigInfo> {
        let name = path.segments.last().ok_or_else(|| Error::plain("empty signature path"))?;
        let unavailable = || Error::at(path.span(), "captured signature is no longer available");
        let info = if let Some(binding) = name.declaration_binding() {
            let (owner, member) = self.declaration_bindings.get(&(binding.clone(), ast::RefKind::Type))
                .ok_or_else(|| Error::at(path.span(), "owned signature is not bound by this declaration group"))?;
            self.signature_at(owner, member).ok_or_else(unavailable)?
        } else if name.local_binding().is_some() {
            return Err(Error::at(path.span(), "a captured local name is not a signature"));
        } else if let Some(binding) = name.resolved_binding() {
            if binding.kind() != ast::RefKind::Type {
                return Err(Error::at(path.span(), "this captured name is not a signature type"));
            }
            self.signature_at(binding.module(), binding.name()).ok_or_else(unavailable)?
        } else if let Some(scope) = name.scoped_reference() {
            if scope.kind() != ast::RefKind::Type {
                return Err(Error::at(path.span(), "this captured name is not a signature type"));
            }
            scope.candidates(ast::RefKind::Type).iter()
                .find_map(|binding| self.signature_at(binding.module(), binding.name()))
                .ok_or_else(|| Error::at(path.span(), format!("unknown signature {} (not bound in captured source scope {})", path.parts().join("."), scope.origin())))?
        } else {
            self.lookup_source_sig(path)?
        };
        if self.record_quote_bindings {
            name.record_binding(ast::RefKind::Type, &info.module, &info.name);
        }
        Ok(info)
    }

    fn lookup_source_sig(&self, path: &ast::ConPath) -> Result<SigInfo> {
        let parts = path.parts();
        match parts.as_slice() {
            [name] => {
                // `Copy` (the design's `Share` class) is compiler-known
                // with structural instances: immutable values that may be
                // duplicated and dropped freely (primitive types
                // specification, section 3; ownership and usage design,
                // section 3). It is not user-extensible. `Io` is the
                // runtime capability head (decision D21): its instances
                // are runtime objects supplied lexically, never declared.
                if matches!(*name, "Copy" | "Share" | "Io") {
                    return Ok(self.signature_at("<builtin>", name).unwrap());
                }
                self.sigs
                    .get(*name)
                    .and_then(|ss| {
                        ss.iter().find(|s| {
                            self.scope_candidates(name)
                                .contains(&(s.module.clone(), s.name.clone()))
                        })
                    })
                    .cloned()
                    .ok_or_else(|| Error::plain(format!("unknown signature {name}")))
            }
            // `Module.Sig`, where the module name may itself be dotted: an
            // interface publishes a class constraint at its defining
            // identity (`Effect.Sequencing.DirectSequencing`), and a
            // constraint path keeps every dot as a segment.
            [qualifier @ .., name] => {
                let m = qualifier.join(".");
                for (dm, dn) in self.scope_qual_candidates(&m, name) {
                    if let Some(s) = self
                        .sigs
                        .get(&dn)
                        .and_then(|ss| ss.iter().find(|s| s.module == dm))
                    {
                        return Ok(s.clone());
                    }
                }
                // A checked contract names a class at its defining
                // identity, and a consumer of that contract resolves the
                // identity directly: which class a constraint names is
                // fixed by the identity, never by an import.
                self.qualified_sigs
                    .get(&(m.clone(), name.to_string()))
                    .cloned()
                    .ok_or_else(|| Error::plain(crate::diagnostic!(
                        "unknown signature {}.{}",
                        crate::spelling::Nominal(&m),
                        name
                    )))
            }
            [] => Err(Error::plain("an empty signature path names no signature")),
        }
    }

    fn scoped_values(&self, scope: &ast::ScopedReference) -> Vec<ValueInfo> {
        scope.candidates(ast::RefKind::Value).iter().filter_map(|binding| {
            self.qualified.get(&(binding.module().into(), binding.name().into())).cloned()
        }).collect()
    }

    fn scoped_constructors(&self, scope: &ast::ScopedReference) -> Vec<ValueInfo> {
        scope.constructor_candidates().into_iter().filter_map(|binding| {
            self.qualified.get(&(binding.module().into(), binding.name().into())).cloned()
        }).collect()
    }

    /// The class key of a signature: its declaring module and its name,
    /// the same rule `nominal_identity` gives a type. The `<builtin>`
    /// classes (`Copy`, `Share`, `Io`) have no declaring source and keep
    /// their bare keys.
    fn signature_key(&self, module: &str, name: &str) -> String {
        if module == "<builtin>" {
            name.to_string()
        } else {
            format!("{module}.{name}")
        }
    }

    fn type_key(&self, info: &TypeKind) -> String {
        if self.qualified_sigs.contains_key(&(info.module.clone(), info.name.clone())) {
            self.signature_key(&info.module, &info.name)
        } else {
            info.identity.clone()
        }
    }


    fn resolve_var(&self, p: &ast::VarPath, span: ast::Span) -> Result<ValueInfo> {
        if let Some(binding) = p.name.declaration_binding() {
            let key = self.declaration_bindings.get(&(binding.clone(), ast::RefKind::Value))
                .ok_or_else(|| Error::at(span, "owned syntax value is not bound by this declaration group"))?;
            return self.qualified.get(key).cloned()
                .ok_or_else(|| Error::at(span, "owned syntax value is not available"));
        }
        if p.name.local_binding().is_some() {
            return Err(Error::at(span, format!("captured local `{}` is not in scope", p.name.text)));
        }
        if let Some(binding) = p.name.resolved_binding() {
            if binding.kind() != ast::RefKind::Value {
                return Err(Error::at(span, "a resolved type is not a value name"));
            }
            return self.qualified.get(&(binding.module().into(), binding.name().into())).cloned()
                .ok_or_else(|| Error::at(span, "resolved syntax value is no longer available"));
        }
        if let Some(scope) = p.name.scoped_reference() {
            let value = self.scoped_values(scope).into_iter().next().ok_or_else(|| Error::at(span,
                format!("unknown name {} (not bound in captured source scope {})",
                    p.quals.iter().map(|q| q.text.as_str()).chain(std::iter::once(p.name.text.as_str())).collect::<Vec<_>>().join("."), scope.origin())))?;
            if self.record_quote_bindings { p.name.record_binding(ast::RefKind::Value, &value.module, &value.name); }
            return Ok(value);
        }
        // The registered name is the unit's real name: a qualified
        // spelling (`U32.wrappingAdd`) is an alias over it, and code
        // generation ids derive from the registered pair.
        let value = match p.quals.as_slice() {
            [] => self.lookup_value(None, &p.name.text).map_err(|e| Error::at(span, e.message)),
            [q] => self
                .lookup_value(Some(&q.text), &p.name.text)
                .map_err(|e| Error::at(span, e.message)),
            _ => {
                Err(Error::at(span, "qualified names deeper than Module.name are not supported"))
            }
        }?;
        if self.record_quote_bindings {
            p.name.record_binding(ast::RefKind::Value, &value.module, &value.name);
        }
        Ok(value)
    }

    fn resolve_con(&self, p: &ast::ConPath, span: ast::Span) -> Result<ValueInfo> {
        if let Some(binding) = p.segments.last().and_then(|name| name.declaration_binding()) {
            let key = self.declaration_bindings.get(&(binding.clone(), ast::RefKind::Value))
                .ok_or_else(|| Error::at(span, "owned syntax constructor is not bound by this declaration group"))?;
            return self.qualified.get(key).cloned()
                .ok_or_else(|| Error::at(span, "owned syntax constructor is not available"));
        }
        if let Some(binding) = p.segments.last().and_then(|n| n.resolved_binding()) {
            if binding.kind() != ast::RefKind::Value {
                return Err(Error::at(span, "a resolved type is not a constructor name"));
            }
            return self.qualified.get(&(binding.module().into(), binding.name().into())).cloned()
                .ok_or_else(|| Error::at(span, "resolved syntax constructor is no longer available"));
        }
        if let Some(scope) = p.segments.last().and_then(|name| name.scoped_reference()) {
            return self.scoped_values(scope).into_iter().next().ok_or_else(|| Error::at(span,
                format!("unknown constructor {} (not bound in captured source scope {})", p.parts().join("."), scope.origin())));
        }
        let parts = p.parts();
        let (module, name) = match parts.as_slice() {
            [name] => {
                let v = self.lookup_value(None, name)?;
                (v.module.clone(), v.name.clone())
            }
            [q, name] => {
                // Module-qualified (through the import scope, surface
                // grammar local amendments section 11), or
                // type-qualified as in `Option.None`.
                let scoped = self
                    .scope_qual_candidates(q, name)
                    .into_iter()
                    .find(|(dm, dn)| self.qualified.contains_key(&(dm.clone(), dn.clone())));
                if let Some(pair) = scoped {
                    pair
                } else if !self.module_names.contains(*q)
                    && self
                        .qualified
                        .contains_key(&(q.to_string(), name.to_string()))
                {
                    (q.to_string(), name.to_string())
                } else if let Ok(t) = self.lookup_type(&[q, name]) {
                    if t.ctors.iter().any(|(c, _)| c == name) {
                        (t.module, name.to_string())
                    } else {
                        return Err(Error::at(span, format!("type {q} has no constructor {name}")));
                    }
                } else {
                    return Err(Error::at(span, format!("unknown constructor {q}.{name}")));
                }
            }
            _ => return Err(Error::at(span, "constructor paths deeper than two segments are not supported")),
        };
        let mut v = self
            .qualified
            .get(&(module.clone(), name.clone()))
            .cloned()
            .ok_or_else(|| Error::at(span, format!("unknown constructor {name}")))?;
        v.name = name;
        Ok(v)
    }

    /// The record type declaring field `name`, when exactly one visible
    /// record has it.
    fn find_record_field(&self, name: &str, span: ast::Span) -> Result<(TypeKind, usize)> {
        let mut hits: Vec<(TypeKind, usize)> = Vec::new();
        for ts in self.types.values() {
            for t in ts {
                if !(t.module == "<builtin>" || self.def_in_scope(&t.module, &t.name)) {
                    continue;
                }
                if let Some(fields) = &t.fields {
                    if let Some(i) = fields.iter().position(|f| f == name) {
                        hits.push((t.clone(), i));
                    }
                }
            }
        }
        match hits.len() {
            0 => Err(Error::at(span, format!("unknown record field {name}"))),
            1 => Ok(hits.pop().expect("one hit")),
            _ => {
                let mut names: Vec<String> = hits
                    .iter()
                    .map(|(t, _)| t.name.clone())
                    .collect();
                names.sort();
                Err(Error::at(
                    span,
                    format!(
                        "ambiguous record field {name}: records {} all declare it; annotate the operand",
                        names.join(", ")
                    ),
                ))
            }
        }
    }

    /// The constructor code generation unit for `n`-tuples (tag 0, one
    /// block word per element), synthesized on first use.
    fn tuple_ctor(&mut self, n: usize, span: ast::Span) -> Result<GlobalId> {
        if n < 2 {
            return Err(Error::at(span, "a tuple needs at least two components"));
        }
        self.word_block_ctor(GlobalId(format!("Tuple.{n}")), n, span)
    }

    fn word_block_ctor(&mut self, id: GlobalId, n: usize, span: ast::Span) -> Result<GlobalId> {
        // Stage compilation moves newly produced units into its evaluator.
        // A cached arity does not mean the constructor remains in the target
        // program. Register against the actual owner registry on every use.
        if let Some(existing) = self.fns.iter().find(|unit| unit.id == id) {
            if existing.ctor != Some((0, n)) {
                return Err(Error::at(span, "a generated word-block constructor conflicts with an existing unit"));
            }
        } else {
            self.fns.push(FnDef {
                id: id.clone(),
                params: (0..n).map(|i| format!("f{i}")).collect(),
                body: crate::core::Core::Unit,
                prim: None,
                ctor: Some((0, n)),
                hints: crate::core::FnHints::default(),
                meta: crate::core::FnMeta::default(),
                // `(a, b, ...)`: each field is its own component's class.
                repr: crate::core::UnitRepr {
                    tparams: n,
                    params: crate::repr::identity(n),
                    fields: crate::repr::identity(n),
                    // A tuple type has one constructor, which the unit
                    // names, so it needs no data type of its own.
                    data: None,
                    result: Some(Class::Obj),
                    dictionary_values: Vec::new(),
                },
            });
        }
        Ok(id)
    }

    // ------------------------------------------------------------ type conversion

    /// One converted `using (...)` item: a class constraint (a
    /// dictionary the instance or caller supplies) or a refinement
    /// obligation (refinement types design, "instance heads at
    /// refinements": the caller discharges it through the `Dec`
    /// evidence path).
    fn conv_oblig(&self, o: &ast::Oblig, vars: &[String]) -> Result<ConvOblig> {
        Ok(match o {
            ast::Oblig::Class(c) => {
                let (n, app) = self.conv_class(c, vars)?;
                ConvOblig::Class(n, app)
            }
            ast::Oblig::Pred(t) => {
                // A using item is a proposition. Closed logical literals
                // must not go through the refinement-former's implicit
                // value-binder shorthand (which has a different grammar).
                let literal = matches!(t, ast::Type::Con(path, _)
                    if matches!(path.parts().as_slice(), ["True"] | ["False"]));
                let predicate = if literal { self.conv_prop(t, vars)? }
                    else { self.conv_pred_former(t, vars)? };
                ConvOblig::Prop(PropOblig::Pred(predicate))
            },
            ast::Oblig::Preserves { method, pred, .. } => ConvOblig::Prop(PropOblig::Preserves {
                method: method.text.clone(),
                pred: self.conv_pred_former(pred, vars)?,
            }),
        })
    }

    fn conv_class(&self, c: &ast::Constraint, vars: &[String]) -> Result<(String, ClassApp)> {
        let sig = self.lookup_sig(&c.class)?;
        if sig.params.len() != c.args.len() {
            return Err(Error::at(
                c.span,
                format!(
                    "signature {} expects {} type arguments, got {}",
                    sig.name,
                    sig.params.len(),
                    c.args.len()
                ),
            ));
        }
        // Class arguments check against the parameter kinds: `Functor
        // List` and `Functor ((->) r)` take a partially applied type
        // constructor, `Eq a` a fully applied one.
        let mut args = Vec::new();
        for (i, a) in c.args.iter().enumerate() {
            args.push(self.conv_kind(a, vars, sig.param_kinds.get(i))?);
        }
        let mut app = ClassApp { class: self.signature_key(&sig.module, &sig.name), args };
        if is_capability_class(&app.class) {
            // A capability's argument is a runtime identity (decision D21).
            for (a, src) in app.args.iter_mut().zip(&c.args) {
                match a {
                    Ty::Param(p) => *a = Ty::Region(p.clone()),
                    _ => {
                        return Err(Error::at(
                            src.span(),
                            "a capability's argument is a runtime identity (a region variable)",
                        ))
                    }
                }
            }
        }
        Ok((c.name.text.clone(), app))
    }

    fn conv_type(&self, ty: &ast::Type, vars: &[String]) -> Result<Ty> {
        self.conv_kind(ty, vars, Some(&ParamKind::Type))
    }

    fn with_type_values<T>(&self, env: &Env, apply: impl FnOnce(&Self) -> Result<T>) -> Result<T> {
        self.scoped_type_env.borrow_mut().push(env.clone());
        let result = apply(self);
        self.scoped_type_env.borrow_mut().pop();
        result
    }

    fn conv_type_in_env(&self, env: &Env, ty: &ast::Type, vars: &[String]) -> Result<Ty> {
        self.with_type_values(env, |checker| checker.conv_type(ty, vars))
    }

    fn conv_scheme_in_env(&self, env: &Env, ty: &ast::Type) -> Result<Scheme> {
        self.with_type_values(env, |checker| checker.conv_scheme(ty))
    }

    /// Convert a type against its expected kind. `expect: None` suppresses
    /// kind notes: an argument slot whose kind is not yet known (an
    /// unannotated parameter of the type being declared) carries no
    /// information about the argument's kind.
    fn conv_kind(
        &self,
        ty: &ast::Type,
        vars: &[String],
        expect: Option<&ParamKind>,
    ) -> Result<Ty> {
        let t = self.conv_raw(ty, vars, expect)?;
        self.check_kind(&t, expect, ty.span())?;
        self.check_refine_instance(&t, ty.span())?;
        Ok(t)
    }

    /// Whether `pred arg` in type position is the refinement TYPE named
    /// like the predicate applied to a carrier (`NonZero Nat`, `NonZero
    /// a`) rather than the proposition over a value (`NonZero n`). The
    /// surface shares one spelling, so the argument decides: a named
    /// type that is not a value (`Nat`, `I64`, `List Nat`), or a
    /// variable already used as a type in this signature (`a -> NonZero
    /// a -> a`), is a carrier. A value variable or index expression
    /// keeps the proposition reading, and so does every predicate with
    /// no parameterized refinement of its name in scope.
    fn refinement_type_app(&self, pred: &str, arg: &ast::Type) -> bool {
        let parametric = self
            .lookup_type(&[pred])
            .is_ok_and(|k| k.refine.is_some() && k.arity > 0);
        if !parametric {
            return false;
        }
        let mut head = arg;
        while let ast::Type::App(f, _, _) = head {
            head = f;
        }
        match head {
            ast::Type::Con(p, _) => self.lookup_value(None, &p.parts().join(".")).is_err(),
            ast::Type::Var(n, _) => self
                .kind_constraints
                .borrow()
                .iter()
                .any(|(m, k)| *m == n.text && matches!(k, ParamKind::Type | ParamKind::Con(_))),
            _ => false,
        }
    }

    /// A refinement whose container is one of its own parameters
    /// (`type NonZero a = refine (n :: a) with NonZero n`) states its
    /// predicate once for every carrier, so the predicate's
    /// admissibility is checked at each instantiation whose carrier is
    /// known: `NonZero Nat` is admitted, `NonZero Bool` is refused
    /// naming the predicate, exactly as a declaration over `Bool` would
    /// be. A carrier still open (a parameter of an enclosing signature)
    /// is checked where it is fixed.
    fn check_refine_instance(&self, t: &Ty, span: ast::Span) -> Result<()> {
        let Ty::Adt(name, args) = t else { return Ok(()) };
        let Some(r) = self
            .known_type(name)
            .and_then(|k| k.refine.as_ref())
        else {
            return Ok(());
        };
        let Ty::Param(p) = &r.base else { return Ok(()) };
        let Some(carrier) = r.params.iter().position(|q| q == p).and_then(|i| args.get(i)) else {
            return Ok(());
        };
        let carrier = self.uni.zonk(carrier);
        if matches!(carrier, Ty::Param(_) | Ty::Meta(_)) {
            return Ok(());
        }
        let Some(view) = self.refine_view(t) else { return Ok(()) };
        self.check_pred_admissible(&view.pred, &r.binder, &carrier, span)
    }

    /// The kind of a converted type as far as it is known: a named
    /// constructor's missing argument count (`Result e` is `Type ->
    /// Type`), a partially applied type former's, or `Type`. A
    /// parameter's kind is being inferred here, so it matches its
    /// expected kind by construction.
    fn check_kind(&self, t: &Ty, expect: Option<&ParamKind>, span: ast::Span) -> Result<()> {
        let Some(expect) = expect else { return Ok(()) };
        let kind = match t {
            Ty::Adt(name, args) if name == "refined" => {
                // The lattice former: `refined P T` and (in an
                // ascription) `refined P` are full types.
                let _ = args;
                ParamKind::Type
            }
            Ty::Adt(name, args) => {
                let info = self.lookup_known_type(name)?;
                if args.len() > info.arity {
                    return Err(Error::at(
                        span,
                        format!("type {name} expects at most {} arguments, got {}", info.arity, args.len()),
                    ));
                }
                let missing = info.arity - args.len();
                if missing == 0 {
                    ParamKind::Type
                } else {
                    ParamKind::Con(missing)
                }
            }
            Ty::App(h, args) => {
                let head_arity = match &**h {
                    Ty::Former(f) => f.arity(),
                    _ => return Ok(()),
                };
                if args.len() >= head_arity {
                    return Ok(());
                }
                ParamKind::Con(head_arity - args.len())
            }
            Ty::Former(f) => ParamKind::Con(f.arity()),
            Ty::Param(_) | Ty::Meta(_) => return Ok(()),
            _ => ParamKind::Type,
        };
        if kind == *expect {
            return Ok(());
        }
        match (&kind, expect) {
            (ParamKind::Con(_), ParamKind::Type) => {
                if let Ty::Adt(name, _) = t {
                    return Err(Error::at(
                        span,
                        format!(
                            "type {name} must be fully applied (expects {} arguments)",
                            self.lookup_known_type(name)?.arity
                        ),
                    ));
                }
                Err(Error::at(
                    span,
                    crate::diagnostic!("{} is a type constructor (`Type -> Type`), but a fully applied type is expected here", t),
                ))
            }
            (ParamKind::Type, ParamKind::Con(n)) => Err(Error::at(
                span,
                crate::diagnostic!(
                    "{} is a fully applied type, but a type constructor of kind `Type -> {}Type` is expected here",
                    t,
                    "Type -> ".repeat(n.saturating_sub(1))
                ),
            )),
            _ => Err(Error::at(
                span,
                crate::diagnostic!("type {} does not have the kind expected here", t),
            )),
        }
    }

    /// Apply one type-level argument with beta-reduction (named
    /// constructors collect into their spine, a saturated type former
    /// collapses to its image, and a type parameter becomes a
    /// higher-kinded application).
    fn type_app(&self, f: Ty, a: Ty, span: ast::Span) -> Result<Ty> {
        match &f {
            Ty::Adt(..) | Ty::Param(..) | Ty::App(..) | Ty::Former(..) | Ty::Meta(..) => {
                Ok(Ty::apply1(f, a))
            }
            other => Err(Error::at(span, crate::diagnostic!("cannot apply non-type-constructor {}", other))),
        }
    }

    /// The usage-multiplicity rule keeping `Later` from unrestricted-thunk
    /// semantics (laziness and reclamation specification, section 2): a
    /// `Later` value is consumed by `force` and may not be bound `%many`,
    /// which would make it a duplicable thunk in disguise.
    fn check_later_mult(&self, arg_ty: &Ty, m: Mult, span: ast::Span) -> Result<()> {
        if matches!(arg_ty, Ty::Later(_)) && m == Mult::Many {
            return Err(Error::at(
                span,
                "a Later-typed parameter must be used at most once: %many would give Later unrestricted-thunk semantics, which the laziness specification forbids (annotate %aff or %1)",
            ));
        }
        Ok(())
    }

    /// Note how a name is used, for classifying un-annotated `forall`
    /// binders after the scheme's type is converted. Conflicting uses are
    /// recorded as-is; `conv_scheme` reports them.
    fn note_kind(&self, name: &str, kind: ParamKind) {
        let mut ks = self.kind_constraints.borrow_mut();
        if !ks.iter().any(|(n, k)| n == name && *k == kind) {
            ks.push((name.to_string(), kind));
        }
    }

    /// Convert an index expression (a value in type position): a variable,
    /// a numeral, mathematical arithmetic, or an application of a named
    /// total definition (dependent core kernel design, section 2).
    fn index_lambda_binding(&self, name: &ast::Name) -> Option<IndexLambdaBinder> {
        self.index_lambda_binders.borrow().iter().rev().find(|binder| {
            match name.local_binding() {
                Some(binding) => binder.name.local_binding() == Some(binding),
                None if name.scoped_reference().is_some() || name.resolved_binding().is_some()
                    || name.declaration_binding().is_some() => false,
                None => binder.name.local_binding().is_none() && binder.name.text == name.text,
            }
        }).cloned()
    }

    fn fresh_index_lambda_variable(&self) -> String {
        let next = self.index_lambda_counter.get();
        self.index_lambda_counter.set(next + 1);
        format!("$index-lambda:{next}")
    }

    fn with_index_lambda<T>(&self, binder: IndexLambdaBinder,
        check: impl FnOnce(&Self) -> Result<T>) -> Result<T> {
        let start = self.index_lambda_binders.borrow().len();
        self.index_lambda_binders.borrow_mut().push(binder);
        let result = check(self);
        self.index_lambda_binders.borrow_mut().truncate(start);
        result
    }

    fn conv_idx(&self, ty: &ast::Type, vars: &[String], kind: &Ty) -> Result<ITerm> {
        self.conv_idx_expected(ty, vars, Some(kind))
    }

    fn conv_idx_expected(&self, ty: &ast::Type, vars: &[String], expected: Option<&Ty>) -> Result<ITerm> {
        let fallback = Ty::Nat;
        let kind = expected.unwrap_or(&fallback);
        let mut args: Vec<&ast::Type> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push(a);
                    cur = f;
                }
                _ => break,
            }
        }
        args.reverse();
        let head = cur;
        let span = ty.span();
        match head {
            ast::Type::IndexAnnotated(value, annotation, _) => {
                let carrier = self.conv_type(annotation, vars)?;
                let value = self.conv_idx_expected(value, vars, Some(&carrier))?;
                let (arguments, result) = self.index_arguments(Some(carrier), &args, vars, kind, span)?;
                if let (Some(expected), Some(actual)) = (expected, result) {
                    self.check_captured_index_carrier("annotated operand", &actual, expected, span)?;
                }
                Ok(if arguments.is_empty() { value } else { ITerm::App(Box::new(value), arguments) })
            }
            ast::Type::IndexOperand(value, carrier, _) => {
                let (arguments, result) = self.index_arguments(Some((**carrier).clone()), &args, vars, kind, span)?;
                if let (Some(expected), Some(actual)) = (expected, result) {
                    self.check_captured_index_carrier("annotated operand", &actual, expected, span)?;
                }
                let value = self.uni.resolve_idx(value);
                Ok(if arguments.is_empty() { value } else { ITerm::App(Box::new(value), arguments) })
            }
            ast::Type::Num(s, _) => {
                if !args.is_empty() {
                    return Err(Error::at(span, "a numeral takes no arguments"));
                }
                if let Some(carrier) = expected {
                    self.check_numeric_index_carrier(carrier, span)?;
                }
                Ok(ITerm::Num(s.clone().into()))
            }
            ast::Type::Var(n, _) => {
                if let Some(binding) = self.index_lambda_binding(n) {
                    let value = ITerm::Var(binding.variable.into());
                    let (arguments, result) = self.index_arguments(binding.carrier, &args, vars, kind, span)?;
                    if let (Some(expected), Some(actual)) = (expected, result) {
                        self.check_captured_index_carrier("index lambda parameter", &actual, expected, span)?;
                    }
                    return Ok(if arguments.is_empty() { value } else { ITerm::App(Box::new(value), arguments) });
                }
                // Mathematical operators and the baked observation vocabulary
                // retain their index meaning even if ordinary values shadow
                // those spellings. Explicit index-lambda variables keep the
                // existing variable rule for named observations.
                if !args.is_empty() {
                    if let Some(op) = match n.source_spelling() {
                        "+" => Some(IdxOp::Add), "-" => Some(IdxOp::Sub), "*" => Some(IdxOp::Mul), _ => None,
                    } {
                        if args.len() != 2 { return Err(Error::at(span, format!("`{}` takes two index arguments", n.source_spelling()))); }
                        self.check_numeric_index_carrier(kind, span)?;
                        return Ok(ITerm::Bin(op, Box::new(self.conv_idx(args[0], vars, kind)?), Box::new(self.conv_idx(args[1], vars, kind)?)));
                    }
                    if matches!(n.source_spelling(), "length" | "toNat")
                        && (!vars.contains(&n.text) || n.local_binding().is_some() || n.scoped_reference().is_some() || n.resolved_binding().is_some())
                    {
                        return self.idx_app(n.source_spelling(), Some(n), &args, vars, expected, n.span);
                    }
                }
                let header_carrier = self.header_index_carrier(n);
                let header_variable = vars.contains(&n.text) || header_carrier.is_some();
                let ordinary_local = !header_variable
                    && n.scoped_reference().is_none()
                    && n.resolved_binding().is_none()
                    && self.scoped_type_env.borrow().last()
                        .is_some_and(|env| env.lookup_name(n).is_some());
                if n.local_binding().is_some() || ordinary_local {
                    let scopes = self.scoped_type_env.borrow();
                    let env = scopes.last().ok_or_else(|| Error::at(n.span,
                        format!("captured local `{}` has no type elaboration scope", n.text)))?;
                    let (local, scheme) = env.lookup_name(n).ok_or_else(|| Error::at(n.span,
                        format!("captured local `{}` is not in scope", n.text)))?;
                    if !scheme.params.is_empty() || (!args.is_empty() && Self::closed_index_signature(&scheme).is_none()) {
                        return Err(Error::at(n.span, "a captured index needs a monomorphic value binding"));
                    }
                    let mut value = self.syntax_index_value(env, n)?;
                    // A type-level binder cannot capture a runtime value
                    // merely by having the same printed name.
                    if matches!(&value, ITerm::Var(name) if vars.iter().any(|v| v.as_str() == name.as_str())) {
                        let original = self.value_aliases.get(&local).copied().unwrap_or(local);
                        value = ITerm::LocalValue(original);
                    }
                    drop(scopes);
                    let (arguments, result) = self.index_arguments(Some(scheme.ty), &args, vars, kind, span)?;
                    if let Some(expected) = expected {
                        self.check_captured_index_carrier(&n.text, &result.ok_or_else(|| Error::at(span, "captured index has no checked result carrier"))?, expected, n.span)?;
                    }
                    return Ok(if arguments.is_empty() { value } else { ITerm::App(Box::new(value), arguments) });
                }
                if n.scoped_reference().is_some() || n.resolved_binding().is_some() {
                    return self.idx_app(n.source_spelling(), Some(n), &args, vars, expected, n.span);
                }
                if args.is_empty() {
                    if !header_variable {
                        // A bare named total definition is an index
                        // value too: a nullary method (`mempty`) or a
                        // named helper a law's equation mentions
                        // (primitive operations and inline target IR
                        // specification section 5).
                        return self.idx_app(&n.text, Some(n), &args, vars, expected, n.span);
                    }
                    if let (Some(expected), Some(actual)) = (expected, header_carrier) {
                        self.check_captured_index_carrier("index parameter", &actual, expected, n.span)?;
                    }
                    // Only quantified parameters need kind inference. A
                    // dependent argument already has its checked domain.
                    if vars.contains(&n.text) {
                        self.note_kind(&n.text, ParamKind::Val(kind.clone()));
                    }
                    return Ok(ITerm::Var(n.text.clone().into()));
                }
                if header_variable {
                    if let Some(carrier) = header_carrier {
                        let (arguments, result) = self.index_arguments(Some(carrier), &args, vars, kind, span)?;
                        if let (Some(expected), Some(actual)) = (expected, result) {
                            self.check_captured_index_carrier("index function parameter", &actual, expected, n.span)?;
                        }
                        return Ok(ITerm::App(Box::new(ITerm::Var(n.text.clone().into())), arguments));
                    }
                    // Unclassified header parameters still collect their kind
                    // constraints; checked carriers use the telescope above.
                    let mut out = Vec::new();
                    for a in args {
                        out.push(self.conv_idx(a, vars, kind)?);
                    }
                    return Ok(ITerm::App(Box::new(ITerm::Var(n.text.clone().into())), out));
                }
                self.idx_app(&n.text, Some(n), &args, vars, expected, n.span)
            }
            ast::Type::Con(p, cspan) => {
                let parts = p.parts();
                if parts == ["Type"] || parts == ["Prop"] {
                    return Err(Error::at(
                        *cspan,
                        "a universe is not an index value (it classifies types)",
                    ));
                }
                let name = parts.join(".");
                self.idx_app(&name, p.segments.last(), &args, vars, expected, *cspan)
            }
            ast::Type::Lam(n, body, _) => {
                // `\x -> t` in index position: a lambda over index
                // values (its binder binds the body's index variables).
                let mut inner = vars.to_vec();
                if n.local_binding().is_none() { inner.push(n.text.clone()); }
                let variable = self.fresh_index_lambda_variable();
                let (domain, result) = match expected {
                    Some(Ty::Fun(domain, _, _, result)) => (Some((**domain).clone()), Some((**result).clone())),
                    Some(Ty::Pi { var, dom, cod, .. }) => (Some((**dom).clone()), Some(cod.subst_idx(var, &ITerm::Var(variable.clone().into())))),
                    _ => (None, None),
                };
                let b = self.with_index_lambda(IndexLambdaBinder { name: n.clone(), variable: variable.clone(), carrier: domain },
                    |checker| checker.conv_idx_expected(body, &inner, result.as_ref()))?;
                let lam = ITerm::Lam(variable.into(), Box::new(b));
                if args.is_empty() {
                    Ok(lam)
                } else {
                    let mut out = Vec::new();
                    for a in args {
                        out.push(self.conv_idx(a, vars, kind)?);
                    }
                    Ok(ITerm::App(Box::new(lam), out))
                }
            }
            other => Err(Error::at(
                other.span(),
                "expected an index value: a variable, a numeral, arithmetic (`n + 1`), or a total function application",
            )),
        }
    }

    /// Numeric predicates read the contents of a borrowed place, retaining
    /// that place's identity for the existing write-invalidation checks.
    /// Generic function argument inference still checks the complete carrier.
    fn numeric_index_read_carrier(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Mut(_, inner) | Ty::Ref(_, inner) if self.int_view(inner).is_some() => self.numeric_index_read_carrier(inner),
            other => self.refine_base(other).unwrap_or_else(|| other.clone()),
        }
    }

    /// Signature argument names are not forall parameters. Their actual
    /// domains nevertheless determine the kind of every index occurrence.
    /// Captured syntax keeps its own lexical identity instead of acquiring
    /// a header binding merely by sharing its printed name.
    fn header_index_carrier(&self, name: &ast::Name) -> Option<Ty> {
        if name.local_binding().is_some() || name.resolved_binding().is_some()
            || name.scoped_reference().is_some() || name.declaration_binding().is_some() {
            return None;
        }
        self.index_carriers.borrow().iter().rev()
            .find(|(binder, _)| binder == &name.text).map(|(_, carrier)| carrier.clone())
    }

    fn check_numeric_index_carrier(&self, carrier: &Ty, span: ast::Span) -> Result<()> {
        let carrier = self.numeric_index_read_carrier(carrier);
        // Generic index applications validate their inferred domains after
        // their arguments and result constrain the private inference store.
        // This check never chooses a type for an unresolved metavariable.
        if self.int_view(&carrier).is_some() || matches!(carrier, Ty::Meta(_)) {
            return Ok(());
        }
        Err(Error::at(span, crate::diagnostic!("numeric index needs an integer carrier, got {}", carrier)))
    }

    fn check_captured_index_carrier(
        &self,
        name: &str,
        actual: &Ty,
        expected: &Ty,
        span: ast::Span,
    ) -> Result<()> {
        let actual = self.uni.zonk(actual);
        let expected = self.uni.zonk(expected);
        // Forget only an established refinement toward its carrier. The
        // requested predicate must remain intact.
        let carrier = self
            .refine_view(&actual)
            .and_then(|r| r.base)
            .unwrap_or_else(|| actual.clone());
        // Inference arrows may still contain an unspecified multiplicity.
        // Ordinary compatibility permits that placeholder, but it must not
        // specialize a type/index/row/region while comparing index carriers.
        let compatible = |scratch: &mut Option<Unifier>, actual: &Ty| {
            if *actual == expected { return true; }
            // One private copy (the checker is only borrowed) serves every
            // attempt; each runs in a trial scope that is rolled back.
            let uni = scratch.get_or_insert_with(|| self.uni.clone());
            let trial = uni.snapshot();
            let ok = uni.unify(actual, &expected).is_ok()
                && uni.zonk(actual) == *actual && uni.zonk(&expected) == expected;
            uni.rollback_to(trial);
            ok
        };
        let mut scratch = None;
        if !compatible(&mut scratch, &actual) && !compatible(&mut scratch, &carrier)
            && !compatible(&mut scratch, &self.numeric_index_read_carrier(&actual)) {
            return Err(Error::at(
                span,
                crate::diagnostic!("captured index `{}` has kind {}, expected {}", name, actual, expected),
            ));
        }
        Ok(())
    }

    /// A concrete domain telescope needs no speculative specialization of
    /// the enclosing checker while converting its index arguments.
    fn closed_index_signature(scheme: &Scheme) -> Option<Ty> {
        (scheme.params.is_empty()
            && scheme.con_params.is_empty()
            && scheme.idx_params.is_empty()
            && scheme.val_params.is_empty()
            && scheme.prop_params.is_empty()
            && scheme.row_params.is_empty()
            && scheme.region_params.is_empty()
            && scheme.evidence_params.is_empty()
            && scheme.ctx.is_empty()
            && scheme.obligs.is_empty())
        .then(|| scheme.ty.clone())
    }

    /// Infer generic index domains in a private store. Only a fully resolved
    /// signature leaves it; private metavariables must never reach conv_idx or
    /// specialize the surrounding checker. Evidence-bearing telescopes remain
    /// explicit rather than losing their constraints during this conversion.
    fn with_index_dictionary_scope<T>(&self, source: &ast::Name, apply: impl FnOnce(&Self) -> Result<T>) -> Result<T> {
        let Some(bindings) = source.scoped_reference().and_then(ast::ScopedReference::dictionary_bindings) else { return apply(self) };
        let mut scope = Vec::new();
        if !bindings.is_empty() {
            let environments = self.scoped_type_env.borrow();
            let environment = environments.last().ok_or_else(|| Error::at(source.span, "captured dictionary scope is no longer available"))?;
            for binding in bindings {
                let local = environment.syntax_dictionaries.get(binding).ok_or_else(|| Error::at(source.span, "captured dictionary binding escaped its lexical scope"))?;
                let class = environment.dicts.iter().find(|(candidate, _)| candidate == local)
                    .map(|(_, class)| class.clone()).ok_or_else(|| Error::at(source.span, "captured dictionary is no longer in scope"))?;
                scope.push((IndexDictionaryBinding::from(*local), class));
            }
        }
        self.index_dictionary_scope.borrow_mut().push(scope);
        let result = apply(self);
        self.index_dictionary_scope.borrow_mut().pop();
        result
    }

    fn index_signature(
        &self,
        scheme: &Scheme,
        args: &[&ast::Type],
        vars: &[String],
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<Option<IndexSignature>> {
        let mut inference = IndexInference::new(&self.uni);
        let Some(mut signature) = self.index_signature_in(&mut inference, scheme, args, vars, expected, span)? else { return Ok(None) };
        self.finish_index_inference(&mut inference)?;
        signature.ty = inference.uni.zonk(&signature.ty);
        let metadata = inference.uni.resolve_idx(&ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Marker, types: signature.types,
            dictionaries: signature.dictionaries, arguments: signature.arguments,
        })));
        let metadata_ty = Ty::Idx(Box::new(metadata));
        self.check_index_export(&inference, &signature.ty, span)?;
        self.check_index_export(&inference, &metadata_ty, span)?;
        let Ty::Idx(metadata) = metadata_ty else { unreachable!() };
        let ITerm::Inst(metadata) = *metadata else { unreachable!() };
        signature.types = metadata.types;
        signature.dictionaries = metadata.dictionaries;
        signature.arguments = metadata.arguments;
        self.commit_index_inference(&mut inference)?;
        Ok(Some(signature))
    }

    fn check_index_export(&self, inference: &IndexInference, ty: &Ty, span: ast::Span) -> Result<()> {
        if !self.provisional_declarations && !self.uni.owns_all_metas(ty) {
            return Err(Error::at(span, "generic index function has an unsolved quantified instantiation"));
        }
        let mut rows = Vec::new(); let mut regions = Vec::new();
        collect_row_names(ty, &mut rows, &mut regions);
        if regions.iter().any(|region| inference.regions.contains(region)) {
            return Err(Error::at(span, "generic index function has an unsolved region instantiation"));
        }
        Ok(())
    }

    fn provisional_index_dictionary(&self) -> ITerm {
        let id = self.declaration_pending.get();
        self.declaration_pending.set(id + 1);
        ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::DeclarationPending(id), types: vec![], dictionaries: vec![], arguments: vec![],
        }))
    }

    fn index_carrier_at(&self, uni: &Unifier, actual: &Ty, expected: &Ty) -> Ty {
        let actual = uni.zonk(actual);
        let expected = uni.zonk(expected);
        let comparison = self.refine_view(&actual).is_some_and(|view| Self::is_comparison_refinement(&view));
        if self.refine_view(&expected).is_none() && (!matches!(expected, Ty::Meta(_)) || comparison) {
            self.refine_base(&actual).unwrap_or(actual)
        } else {
            actual
        }
    }

    fn index_signature_in(
        &self,
        inference: &mut IndexInference,
        scheme: &Scheme,
        args: &[&ast::Type],
        vars: &[String],
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<Option<IndexSignature>> {
        if scheme.con_params.iter().any(|(name, _)| !scheme.params.contains(name)) {
            return Err(Error::at(span, "index function has an incomplete constructor telescope"));
        }
        let mut substitutions = HashMap::new();
        for name in &scheme.params {
            substitutions.insert(name.clone(), inference.uni.fresh());
        }
        for parameter in &scheme.idx_params {
            let value = match parameter {
                IxParam::Val(..) => inference.uni.fresh_idx(),
                IxParam::Prop(..) => inference.uni.fresh_prop(),
            };
            substitutions.insert(parameter.name().to_string().into(), value);
        }
        for (name, _) in &scheme.val_params { substitutions.insert(name.clone(), inference.uni.fresh_idx()); }
        for name in &scheme.prop_params { substitutions.insert(name.clone(), inference.uni.fresh_prop()); }
        let mut evidence = HashMap::new();
        for (name, _) in &scheme.ctx { evidence.insert(dictionary_parameter(name), inference.uni.fresh_idx_term()); }
        for parameter in &scheme.evidence_params { evidence.entry(parameter.clone()).or_insert_with(|| inference.uni.fresh_idx_term()); }
        let rows: HashMap<_, _> = scheme.row_params.iter().map(|name| (name.clone(), inference.uni.fresh_row().tail)).collect();
        let regions: HashMap<_, _> = scheme.region_params.iter().map(|name| {
            let region = self.fresh_region();
            inference.uni.note_flex_region(&region);
            inference.regions.push(region.clone());
            (name.clone(), region)
        }).collect();
        let signature = Self::subst_namespaces(&scheme.ty, &substitutions, &rows, &regions, &evidence);
        let unresolved = |ty: &Ty| !self.uni.owns_all_metas(ty);
        let mut result = signature.clone();
        let mut domains = Vec::new();
        for arg in args {
            let (domain, effect, codomain, binder) = match inference.uni.zonk(&result) {
                Ty::Fun(dom, _, eff, cod) => (*dom, eff, *cod, None),
                Ty::Pi {
                    var, dom, eff, cod, ..
                } => (*dom, eff, *cod, Some(var)),
                other => {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!("index applies non-function carrier {}", other),
                    ))
                }
            };
            if !inference.uni.resolve_row(&effect).is_pure() {
                return Err(Error::at(span, "index function application must be pure"));
            }
            if let Some(actual) = self.captured_index_carrier_in(inference, arg, vars, Some(&domain)).map_err(|error| Error::at(span, format!("index function argument carrier: {}", error.message)))? {
                // Ordinary argument checking treats comparison refinements
                // as carrier facts even under an open expectation. Semantic
                // refinement points retain their type and instance selection.
                let actual = self.index_carrier_at(&inference.uni, &actual, &domain);
                inference.uni.unify(&actual, &domain).map_err(|e| {
                    Error::at(span, format!("index function argument carrier: {e}"))
                })?;
            }
            domains.push((domain.clone(), *arg));
            result = if let Some(name) = binder.filter(|name| codomain.mentions_idx(name)) {
                let domain = inference.uni.zonk(&domain);
                if unresolved(&domain) {
                    return Err(Error::at(
                        span,
                        "dependent index argument has no inferred carrier",
                    ));
                }
                codomain.subst_idx(&name, &self.conv_idx(arg, vars, &domain)?)
            } else {
                codomain
            };
        }
        if let Some(expected) = expected {
            let actual = self.index_carrier_at(&inference.uni, &result, expected);
            inference.uni.unify(&actual, expected)
                .map_err(|e| Error::at(span, format!("index function result kind: {e}")))?;
        }
        // Numerical index literals default only after checked operands and the
        // expected result have constrained shared generic parameters.
        for (domain, arg) in domains {
            if let ast::Type::Num(digits, _) = arg {
                let domain = inference.uni.zonk(&domain);
                if matches!(domain, Ty::Meta(_)) {
                    inference.uni.unify(
                        &domain,
                        &if digits.starts_with('-') {
                            Ty::Int
                        } else {
                            Ty::Nat
                        },
                    )
                    .map_err(|e| Error::at(span, e))?;
                }
                let domain = inference.uni.zonk(&domain);
                if !matches!(&domain, Ty::Nat | Ty::Int)
                    && !matches!(&domain, Ty::Prim(primitive) if primitive.is_int_like())
                {
                    return Err(Error::at(
                        arg.span(),
                        crate::diagnostic!("index literal needs a numeric carrier, got {}", domain),
                    ));
                }
                if domain == Ty::Nat && digits.starts_with('-') {
                    return Err(Error::at(
                        arg.span(),
                        "negative index literal cannot inhabit Nat",
                    ));
                }
            }
        }
        let mut scope: Vec<(IndexDictionaryBinding, ClassApp)> = self.index_header_context.borrow().iter()
            .flatten().map(|(name, class)| (IndexDictionaryBinding(ITerm::Evidence(dictionary_parameter(name))), class.clone())).collect();
        if let Some(env) = self.scoped_type_env.borrow().last() {
            scope.extend(env.dicts.iter().map(|(local, class)| (IndexDictionaryBinding::from(*local), class.clone())));
        }
        if let Some(captured) = self.index_dictionary_scope.borrow().last() { scope = captured.clone(); }
        let mut dictionaries = Vec::new();
        for (name, class) in &scheme.ctx {
            let parameter = &evidence[&dictionary_parameter(name)];
            inference.dictionaries.push(PendingIndexDictionary {
                parameter: parameter.clone(), class: Self::subst_class_namespaces(class, &substitutions, &rows, &regions, &evidence), scope: scope.clone(), span,
            });
            dictionaries.push(parameter.clone());
        }
        let types = scheme.params.iter().map(|name| substitutions[name].clone()).collect();
        let mut arguments = Vec::new();
        for parameter in &scheme.idx_params {
            arguments.push(match parameter {
                IxParam::Val(name, carrier) => {
                    let Ty::Idx(value) = &substitutions[name] else { unreachable!() };
                    IndexArgument::Value { value: (**value).clone(), carrier: Self::subst_namespaces(carrier, &substitutions, &rows, &regions, &evidence), runtime: false }
                }
                IxParam::Prop(name) => {
                    let Ty::Prp(prop) = &substitutions[name] else { unreachable!() };
                    IndexArgument::Proposition((**prop).clone())
                }
            });
        }
        for (name, carrier) in &scheme.val_params {
            let Ty::Idx(value) = &substitutions[name] else { unreachable!() };
            arguments.push(IndexArgument::Value { value: (**value).clone(), carrier: Self::subst_namespaces(carrier, &substitutions, &rows, &regions, &evidence), runtime: true });
        }
        for name in &scheme.prop_params {
            let Ty::Prp(prop) = &substitutions[name] else { unreachable!() };
            arguments.push(IndexArgument::Proposition((**prop).clone()));
        }
        arguments.extend(scheme.row_params.iter().map(|name| IndexArgument::Row(EffRow { labels: Vec::new(), tail: rows[name].clone() })));
        arguments.extend(scheme.region_params.iter().map(|name| IndexArgument::Region(regions[name].clone().into())));
        arguments.extend(scheme.evidence_params.iter().map(|parameter| IndexArgument::Evidence(evidence[parameter].clone())));
        if !scheme.obligs.is_empty() {
            inference.proofs.push(PendingIndexProof {
                scheme: scheme.clone(), substitutions, rows, regions, evidence, signature: signature.clone(),
                arguments: args.iter().map(|argument| (**argument).clone()).collect(), variables: vars.to_vec(), span,
            });
        }
        Ok(Some(IndexSignature { ty: signature, types, dictionaries, arguments }))
    }

    fn finish_index_inference(&self, inference: &mut IndexInference) -> Result<()> {
        // The store moves into the resolver and back instead of being
        // copied. A failed resolution rolls it back, leaving it as it was on
        // entry, which is what keeping the original beside a copy did.
        let mut uni = std::mem::take(&mut inference.uni);
        let snapshot = uni.snapshot();
        let mut resolver = DictionaryResolver { checker: self, uni, transfers: Vec::new() };
        let pending = std::mem::take(&mut inference.dictionaries);
        let resolved = self.resolve_index_dictionaries(&mut resolver, pending);
        let DictionaryResolver { mut uni, transfers, .. } = resolver;
        match resolved {
            Ok(()) => uni.commit(snapshot),
            Err(_) => uni.rollback_to(snapshot),
        }
        inference.uni = uni;
        resolved?;
        inference.transfers.extend(transfers);
        if self.provisional_declarations {
            inference.proofs.clear();
            return Ok(());
        }
        for proof in std::mem::take(&mut inference.proofs) {
            let substitutions = proof.substitutions.into_iter().map(|(name, ty)| (name, inference.uni.zonk(&ty))).collect();
            let regions = proof.regions.into_iter().map(|(name, region)| (name, inference.uni.resolve_region(&region).to_string())).collect();
            let signature = inference.uni.zonk(&proof.signature);
            if !self.uni.owns_all_metas(&signature) {
                return Err(Error::at(proof.span, "index proof arguments have an unsolved carrier"));
            }
            let arguments: Vec<_> = proof.arguments.iter().collect();
            self.check_index_obligations(&inference.uni, &proof.scheme, &substitutions, &proof.rows, &regions, &proof.evidence, &inference.uni.zonk(&proof.signature), &arguments, &proof.variables, proof.span)?;
        }
        Ok(())
    }

    fn resolve_index_dictionaries(&self, resolver: &mut DictionaryResolver<'_>, mut pending: Vec<PendingIndexDictionary>) -> Result<()> {
        while !pending.is_empty() {
            let mut remaining = Vec::new(); let mut progress = false;
            for request in pending {
                let selection = if self.law_index_context.borrow().is_some() {
                    let mut scope = request.scope.clone();
                    scope.extend(self.law_index_context.borrow().as_ref().expect("law context").iter()
                        .map(|(name, class)| (IndexDictionaryBinding(ITerm::Evidence(dictionary_parameter(name))), class.clone())));
                    match resolver.lexical(&request.class, &scope) {
                        Some(dictionary) => Solved(dictionary),
                        None => {
                            let class = resolver.uni.zonk_class(&request.class);
                            if class.args.iter().any(|ty| !self.uni.owns_type_metas(ty)) {
                                return Err(Error::at(request.span, "a law's dictionary arguments must be determined by its binders"));
                            }
                            let mut context = self.law_index_context.borrow_mut();
                            let context = context.as_mut().expect("law context");
                            let name = format!("#law-evidence:{}", context.len());
                            context.push((name.clone(), class));
                            Solved(DictExpr::Binder(IndexDictionaryBinding(ITerm::Evidence(dictionary_parameter(&name)))))
                        }
                    }
                } else if let Some(dictionary) = resolver.lexical(&request.class, &request.scope) {
                    Solved(dictionary)
                } else {
                    let class = resolver.uni.zonk_class(&request.class);
                    if self.collecting_declarations && !self.provisional_declarations && self.declaration_seed.is_none()
                        && class.class != "Copy" && !is_capability_class(&class.class) {
                        self.declaration_needs_headers.set(true);
                        return Err(Error::at(request.span, "index declaration evidence needs the complete instance headers"));
                    }
                    match resolver.solve(&request.class, &request.scope, 0, request.span) {
                        Ok(Solved(dictionary)) => Solved(dictionary),
                        Ok(Ambiguous) => {
                            if self.collecting_declarations { self.declaration_needs_headers.set(true); }
                            Ambiguous
                        }
                        Err(_) if self.provisional_declarations => {
                            Solved(DictExpr::Binder(IndexDictionaryBinding(self.provisional_index_dictionary())))
                        }
                        Err(error) => {
                            if self.collecting_declarations { self.declaration_needs_headers.set(true); }
                            return Err(error);
                        }
                    }
                };
                match selection {
                    Solved(dictionary) => {
                        let actual = dictionary.index_with(&resolver.uni, &|binding| binding.0.clone());
                        if !self.provisional_declarations && actual.has_declaration_pending() {
                            return Err(Error::at(request.span, "unresolved declaration evidence in strict header checking"));
                        }
                        resolver.uni.unify_idx(&request.parameter, &actual).map_err(|error| Error::at(request.span, error))?;
                        progress = true;
                    }
                    Ambiguous => remaining.push(request),
                }
            }
            if !progress {
                if self.provisional_declarations {
                    for request in remaining {
                        resolver.uni.unify_idx(&request.parameter, &self.provisional_index_dictionary())
                            .map_err(|error| Error::at(request.span, error))?;
                    }
                    break;
                }
                let request = &remaining[0];
                return Err(Error::at(request.span, crate::diagnostic!("ambiguous index dictionary for {}", resolver.uni.zonk_class(&request.class))));
            }
            pending = remaining;
        }
        Ok(())
    }

    fn commit_index_inference(&self, inference: &mut IndexInference) -> Result<()> {
        if self.provisional_declarations {
            inference.transfers.clear();
            return Ok(());
        }
        if inference.uni.has_declaration_pending() {
            return Err(Error::plain("unresolved declaration evidence in strict index inference"));
        }
        for transfer in &inference.transfers {
            let wanted = inference.uni.zonk_class(&transfer.wanted);
            if wanted.args.iter().any(|ty| !self.uni.owns_all_metas(ty)) {
                return Err(Error::at(transfer.span, "index dictionary transfer has an unsolved carrier"));
            }
        }
        for transfer in std::mem::take(&mut inference.transfers) {
            self.record_transfer_in(&inference.uni, &self.resolution_instances()[transfer.instance], &transfer.obligs,
                transfer.container, &transfer.wanted, &transfer.subdicts, transfer.span)?;
        }
        Ok(())
    }

    /// Index applications use the same proposition checker as ordinary term
    /// applications. Proofs are erased, but their obligations are never erased
    /// before checking. Positional arguments retain their actual lexical names.
    fn check_index_obligations(
        &self, uni: &Unifier, scheme: &Scheme, substitutions: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>, signature: &Ty,
        args: &[&ast::Type], vars: &[String], span: ast::Span,
    ) -> Result<()> {
        if scheme.obligs.is_empty() { return Ok(()); }
        let (arguments, _) = self.index_arguments(Some(signature.clone()), args, vars, &Ty::Unit, span)?;
        for obligation in &scheme.obligs {
            let PropOblig::Pred(predicate) = obligation else {
                return Err(Error::at(span, "an index signature cannot carry an instance transfer obligation"));
            };
            // Resolve in the same inference store: a substituted row may
            // contribute labels as well as a tail, and its IDs belong here.
            let predicate = Self::subst_prop_namespaces(predicate, substitutions, rows, regions, evidence);
            let mut predicate = uni.resolve_prop(&predicate);
            let mut names = Vec::new(); predicate.free_vars(&mut names);
            let mut positions = Vec::new();
            for name in names {
                let position = name.strip_prefix("$a").and_then(|index| index.parse::<usize>().ok())
                    .ok_or_else(|| Error::at(span, format!("index obligation `{predicate}` names no argument of this function")))?;
                let argument = arguments.get(position).ok_or_else(|| Error::at(span,
                    format!("index obligation `{predicate}` must be discharged at a fully applied call")))?;
                predicate = predicate.subst(&name, argument);
                positions.push(position);
            }
            if self.decide_closed_oblig(&predicate, span)? { continue; }
            let mut static_ok = false;
            for position in positions {
                if let Some(carrier) = self.captured_index_carrier(args[position], vars)? {
                    static_ok |= self.static_discharge(&carrier, &predicate);
                }
            }
            if static_ok { continue; }
            let env = self.scoped_type_env.borrow().last().cloned().unwrap_or_default();
            self.discharge_oblig(&self.evidence_in(&env), &predicate, span)?;
        }
        Ok(())
    }

    fn captured_index_carrier(&self, ty: &ast::Type, vars: &[String]) -> Result<Option<Ty>> {
        let mut inference = IndexInference::new(&self.uni);
        let result = self.captured_index_carrier_in(&mut inference, ty, vars, None)?;
        self.finish_index_inference(&mut inference)?;
        let result = result.map(|ty| inference.uni.zonk(&ty));
        if let Some(result) = &result { self.check_index_export(&inference, result, ty.span())?; }
        self.commit_index_inference(&mut inference)?;
        Ok(result)
    }

    /// The baked length vocabulary counts either bytes or a checked list
    /// spine. Its domain follows the actual operand carrier, including a
    /// refinement's container; an ordinary source function named length
    /// cannot supply this meaning or change the carrier.
    fn length_measure_carrier(&self, carrier: &Ty) -> bool {
        if *carrier == Ty::Bytes { return true; }
        let Ty::Adt(name, arguments) = carrier else { return false };
        let Some(kind) = self.known_type(name) else { return false };
        if kind.ctors.len() != 2 || kind.arity != arguments.len() { return false; }
        let mut empty = false; let mut cons = false;
        for (constructor, _) in &kind.ctors {
            let Some(value) = self.qualified.get(&(kind.module.clone(), constructor.clone())) else { return false };
            let ValueKind::Ctor { runtime_arity, field_tys, field_erased, local_idx, res_args, .. } = &value.kind else { return false };
            if !value.scheme.ctx.is_empty() || !local_idx.is_empty() || res_args.len() != arguments.len() { return false; }
            let mut parameters = HashMap::new();
            for (parameter, actual) in res_args.iter().zip(arguments) {
                let Ty::Param(parameter) = parameter else { return false };
                if parameters.insert(parameter.to_string(), actual.clone()).is_some() { return false; }
            }
            match runtime_arity {
                0 if !empty => empty = true,
                2 if !cons => {
                    let fields = field_tys.iter().zip(field_erased).filter_map(|(ty, erased)| (!erased).then_some(ty)).collect::<Vec<_>>();
                    if fields.len() != 2 { return false; }
                    let tail = Self::subst_ty(fields[1], &parameters);
                    let tail = self.refine_base(&tail).unwrap_or(tail);
                    if tail != *carrier { return false; }
                    cons = true;
                }
                _ => return false,
            }
        }
        empty && cons
    }

    fn builtin_length_domain_in(&self, inference: &mut IndexInference, argument: &ast::Type, vars: &[String]) -> Result<Option<Ty>> {
        let Some(carrier) = self.captured_index_carrier_in(inference, argument, vars, None)? else { return Ok(None) };
        let carrier = inference.uni.zonk(&carrier);
        let carrier = self.refine_base(&carrier).unwrap_or(carrier);
        let supported = self.length_measure_carrier(&carrier);
        if !supported {
            return Err(Error::at(argument.span(), crate::diagnostic!("length index operand needs Bytes or a list-shaped carrier, got {}", carrier)));
        }
        Ok(Some(carrier))
    }

    fn captured_index_carrier_in(&self, inference: &mut IndexInference, ty: &ast::Type, vars: &[String], expected: Option<&Ty>) -> Result<Option<Ty>> {
        if let ast::Type::IndexAnnotated(value, annotation, _) = ty {
            let carrier = self.conv_type(annotation, vars)?;
            if let Some(actual) = self.captured_index_carrier_in(inference, value, vars, Some(&carrier))? {
                let actual = self.index_carrier_at(&inference.uni, &actual, &carrier);
                inference.uni.unify(&actual, &carrier).map_err(|error| Error::at(ty.span(), error))?;
            }
            return Ok(Some(inference.uni.zonk(&carrier)));
        }
        if let ast::Type::IndexOperand(_, carrier, _) = ty {
            return Ok(Some(inference.uni.zonk(carrier)));
        }
        if let ast::Type::Lam(name, body, span) = ty {
            let Some(expected) = expected.map(|ty| inference.uni.zonk(ty)) else { return Ok(None) };
            let variable = self.fresh_index_lambda_variable();
            let (domain, result) = match &expected {
                Ty::Fun(domain, _, _, result) => ((**domain).clone(), (**result).clone()),
                Ty::Pi { var, dom, cod, .. } => ((**dom).clone(), cod.subst_idx(var, &ITerm::Var(variable.clone().into()))),
                _ => return Err(Error::at(*span, "an index lambda needs a function carrier")),
            };
            let mut variables = vars.to_vec();
            if name.local_binding().is_none() { variables.push(name.text.clone()); }
            let actual = self.with_index_lambda(IndexLambdaBinder { name: name.clone(), variable, carrier: Some(domain) },
                |checker| checker.captured_index_carrier_in(inference, body, &variables, Some(&result)))?;
            if let Some(actual) = actual {
                let actual = self.index_carrier_at(&inference.uni, &actual, &result);
                inference.uni.unify(&actual, &result).map_err(|error| Error::at(*span, error))?;
            }
            return Ok(Some(inference.uni.zonk(&expected)));
        }
        let mut head = ty;
        let mut args = Vec::new();
        while let ast::Type::App(f, arg, _) = head {
            args.push(&**arg);
            head = f;
        }
        args.reverse();
        let name = match head {
            ast::Type::Var(name, _) => name,
            ast::Type::Con(path, _) => match path.segments.last() {
                Some(name) => name,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };
        if matches!(head, ast::Type::Var(..)) {
            if let Some(binding) = self.index_lambda_binding(name) {
                let Some(carrier) = binding.carrier else { return Ok(None) };
                if args.is_empty() { return Ok(Some(inference.uni.zonk(&carrier))); }
                let signature = self.index_signature_in(inference, &Scheme::mono(carrier), &args, vars, expected, ty.span())?
                    .ok_or_else(|| Error::at(ty.span(), "index lambda parameter has no checked carrier"))?;
                let (_, result) = self.index_arguments(Some(signature.ty), &args, vars, &Ty::Nat, ty.span())?;
                return Ok(result);
            }
        }
        if matches!(head, ast::Type::Var(..)) && matches!(name.source_spelling(), "+" | "-" | "*") && args.len() == 2 {
            let left = self.captured_index_carrier_in(inference, args[0], vars, None)?;
            let right = self.captured_index_carrier_in(inference, args[1], vars, left.as_ref())?;
            if let (Some(left), Some(right)) = (&left, &right) {
                let left = self.refine_base(left).unwrap_or_else(|| left.clone());
                let right = self.refine_base(right).unwrap_or_else(|| right.clone());
                inference.uni.unify(&left, &right).map_err(|error| Error::at(ty.span(), error))?;
            }
            let carrier = inference.uni.zonk(&left.or(right).or_else(|| expected.cloned()).unwrap_or(Ty::Nat));
            return Ok(Some(self.refine_base(&carrier).unwrap_or(carrier)));
        }
        if matches!(head, ast::Type::Var(..)) && !args.is_empty()
            && (!vars.contains(&name.text) || name.local_binding().is_some() || name.scoped_reference().is_some() || name.resolved_binding().is_some()) {
            let carrier = match name.source_spelling() {
                "length" => self.builtin_length_domain_in(inference, args[0], vars)?.map(|domain| Ty::fun(domain, Ty::Nat)),
                "toNat" => {
                    let operand = self.captured_index_carrier_in(inference, args[0], vars, None)?;
                    match operand.map(|ty| inference.uni.zonk(&ty)) {
                        Some(carrier) => {
                            let carrier = self.refine_base(&carrier).unwrap_or(carrier);
                            if !matches!(&carrier, Ty::Prim(primitive) if primitive.is_int_like()) {
                                return Err(Error::at(ty.span(), "toNat index operand needs a machine integer"));
                            }
                            Some(Ty::fun(carrier, Ty::Nat))
                        }
                        None => return Ok(None),
                    }
                }
                _ => None,
            };
            if let Some(carrier) = carrier {
                let scheme = Scheme::mono(carrier);
                let Some(signature) = self.index_signature_in(inference, &scheme, &args, vars, expected, ty.span())? else { unreachable!() };
                let (_, result) = self.index_arguments(Some(signature.ty), &args, vars, &Ty::Nat, ty.span())?;
                return Ok(result);
            }
        }
        if let Some(carrier) = self.header_index_carrier(name) {
            let mut result = carrier;
            for argument in &args {
                result = match result {
                    Ty::Fun(_, _, _, cod) => *cod,
                    Ty::Pi { var, dom, cod, .. } => cod.subst_idx(&var, &self.conv_idx(argument, vars, &dom)?),
                    _ => return Ok(None),
                };
            }
            return Ok(Some(result));
        }
        let value_scheme = |value: ValueInfo| {
            let label = index_value_label(&value);
            (value.scheme, Some(label))
        };
        let scheme = if name.local_binding().is_some() {
            let scopes = self.scoped_type_env.borrow();
            scopes
                .last()
                .and_then(|env| env.lookup_name(name))
                .map(|(_, scheme)| (scheme, None))
        } else if let Some(binding) = name.resolved_binding() {
            if binding.kind() != ast::RefKind::Value {
                return Ok(None);
            }
            self.qualified
                .get(&(binding.module().into(), binding.name().into()))
                .cloned().map(value_scheme)
        } else if let Some(scope) = name.scoped_reference() {
            self.scoped_values(scope)
                .into_iter()
                .next()
                .map(value_scheme)
        } else if matches!(head, ast::Type::Var(..)) && !vars.contains(&name.text) {
            // Ordinary source annotations use their lexical environment too.
            // Captured/resolved names took their authoritative paths above;
            // they must never fall back to insertion-site spelling lookup.
            self.scoped_type_env.borrow().last()
                .and_then(|env| env.lookup_name(name)).map(|(_, scheme)| (scheme, None))
                .or_else(|| self.lookup_value(None, &name.text).ok().map(value_scheme))
        } else if let ast::Type::Con(path, _) = head {
            let text = path.segments.iter().map(|name| name.text.as_str()).collect::<Vec<_>>().join(".");
            self.lookup_value(None, &text).or_else(|error| match text.rsplit_once('.') {
                Some((qualifier, base)) => self.lookup_value(Some(qualifier), base),
                None => Err(error),
            }).ok().map(value_scheme)
        } else {
            None
        };
        let Some((scheme, label)) = scheme else {
            return Ok(None);
        };
        let Some(signature) = self.with_index_dictionary_scope(name, |checker|
            checker.index_signature_in(inference, &scheme, &args, vars, expected, ty.span()))
            .map_err(|mut error| {
                if let Some(label) = label {
                    error.message = format!("index function `{label}`: {}", error.message);
                }
                error
            })? else {
            return Ok(None);
        };
        let mut result = inference.uni.zonk(&signature.ty);
        for arg in args {
            result = match result {
                Ty::Fun(_, _, _, cod) => *cod,
                Ty::Pi { var, dom, cod, .. } => {
                    cod.subst_idx(&var, &self.conv_idx(arg, vars, &dom)?)
                }
                _ => return Ok(None),
            };
        }
        Ok(Some(inference.uni.zonk(&result)))
    }

    fn index_arguments(
        &self,
        mut signature: Option<Ty>,
        args: &[&ast::Type],
        vars: &[String],
        kind: &Ty,
        span: ast::Span,
    ) -> Result<(Vec<ITerm>, Option<Ty>)> {
        let mut out = Vec::new();
        for arg in args {
            let (domain, next, binder) = match signature.take().map(|ty| self.uni.zonk(&ty)) {
                Some(Ty::Fun(domain, _, effect, result)) => {
                    if !self.uni.resolve_row(&effect).is_pure() {
                        return Err(Error::at(span, "index function application must be pure"));
                    }
                    (*domain, Some(*result), None)
                }
                Some(Ty::Pi {
                    var, dom, eff, cod, ..
                }) => {
                    if !self.uni.resolve_row(&eff).is_pure() {
                        return Err(Error::at(span, "index function application must be pure"));
                    }
                    (*dom, Some(*cod), Some(var))
                }
                Some(other) => {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!("index applies non-function carrier {}", other),
                    ))
                }
                None => (kind.clone(), None, None),
            };
            let argument = self.conv_idx(arg, vars, &domain)?;
            signature = next.map(|ty| match binder {
                Some(name) => ty.subst_idx(&name, &argument),
                None => ty,
            });
            out.push(argument);
        }
        Ok((out, signature))
    }

    /// An application in index position: `toNat limit`, `length bytes`,
    /// or a named total definition.
    fn idx_app(
        &self,
        name: &str,
        source: Option<&ast::Name>,
        args: &[&ast::Type],
        vars: &[String],
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<ITerm> {
        match source {
            Some(reference) => self.with_index_dictionary_scope(reference, |checker| checker.idx_app_in_scope(name, source, args, vars, expected, span)),
            None => self.idx_app_in_scope(name, source, args, vars, expected, span),
        }
    }

    fn idx_app_in_scope(
        &self,
        name: &str,
        source: Option<&ast::Name>,
        args: &[&ast::Type],
        vars: &[String],
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<ITerm> {
        // The builtin index vocabulary shadows ordinary names in index
        // position (decision D29): `length` over `Bytes` and `toNat` over
        // machine integers are total, executable, and reduce nowhere in
        // conversion.
        let fallback = Ty::Nat;
        let kind = expected.unwrap_or(&fallback);
        let mut signature = None;
        let mut type_arguments = Vec::new();
        let mut dictionaries = Vec::new();
        let mut implicit_arguments = Vec::new();
        let head = if matches!(name, "toNat" | "length") {
            if name == "length" {
                let domain = args.first().map(|argument| {
                    let mut inference = IndexInference::new(&self.uni);
                    let domain = self.builtin_length_domain_in(&mut inference, argument, vars)?;
                    self.finish_index_inference(&mut inference)?;
                    let domain = domain.map(|ty| inference.uni.zonk(&ty));
                    if let Some(domain) = &domain { self.check_index_export(&inference, domain, span)?; }
                    self.commit_index_inference(&mut inference)?;
                    Ok::<_, Error>(domain)
                }).transpose()?.flatten().unwrap_or(Ty::Bytes);
                signature = Some(Ty::fun(domain, Ty::Nat));
            } else if let Some(actual) = args.first().map(|arg| self.captured_index_carrier(arg, vars)).transpose()?.flatten() {
                let base = self.refine_base(&actual).unwrap_or(actual);
                if !matches!(&base, Ty::Prim(primitive) if primitive.is_int_like()) {
                    return Err(Error::at(
                        span,
                        "toNat index operand needs a machine integer",
                    ));
                }
                signature = Some(Ty::fun(base, Ty::Nat));
            }
            format!("<builtin>.{name}")
        } else if let Some(binding) = source.and_then(|n| n.resolved_binding()) {
            if binding.kind() != ast::RefKind::Value {
                return Err(Error::at(span, "a resolved type is not an index value"));
            }
            let value = self
                .qualified
                .get(&(binding.module().into(), binding.name().into()))
                .ok_or_else(|| Error::at(span, "resolved syntax index is no longer available"))?;
            if let Some(checked) = self.index_signature(&value.scheme, args, vars, expected, span)? {
                signature = Some(checked.ty); type_arguments = checked.types; dictionaries = checked.dictionaries; implicit_arguments = checked.arguments;
            }
            value_index_id(value)
        } else if let Some((source, scope)) =
            source.and_then(|name| name.scoped_reference().map(|scope| (name, scope)))
        {
            let value = self
                .scoped_values(scope)
                .into_iter()
                .next()
                .ok_or_else(|| {
                    Error::at(
                        span,
                        format!(
                            "unknown index {name} (not bound in captured source scope {})",
                            scope.origin()
                        ),
                    )
                })?;
            if self.record_quote_bindings {
                source.record_binding(ast::RefKind::Value, &value.module, &value.name);
            }
            if let Some(checked) = self.index_signature(&value.scheme, args, vars, expected, span)? {
                signature = Some(checked.ty); type_arguments = checked.types; dictionaries = checked.dictionaries; implicit_arguments = checked.arguments;
            }
            value_index_id(&value)
        } else {
            // A qualified name resolves through the same qualifier
            // machinery as term position (`List.map` is `Data.List`'s
            // `map` through the import's short qualifier).
            let found = self
                .lookup_value(None, name)
                .or_else(|e| match name.rsplit_once('.') {
                    Some((qual, base)) => self.lookup_value(Some(qual), base),
                    None => Err(e),
                });
            match found {
                Ok(v) => {
                    if let Some(checked) = self.index_signature(&v.scheme, args, vars, expected, span)? {
                        signature = Some(checked.ty); type_arguments = checked.types; dictionaries = checked.dictionaries; implicit_arguments = checked.arguments;
                    }
                    value_index_id(&v)
                }
                Err(_) => {
                    return Err(Error::at(
                        span,
                        format!("unknown name {name} in index position"),
                    ))
                }
            }
        };
        if signature.is_none() && !matches!(name, "length" | "toNat") && source.is_some_and(|name| {
            name.local_binding().is_some() || name.scoped_reference().is_some() || name.resolved_binding().is_some()
        }) {
            return Err(Error::at(span, "captured index function requires its quantified evidence telescope; it cannot be discarded"));
        }
        let (out, result) = self.index_arguments(signature, args, vars, kind, span)?;
        if source.is_some_and(|name| {
            name.local_binding().is_some()
                || name.scoped_reference().is_some()
                || name.resolved_binding().is_some()
        }) {
            if let (Some(actual), Some(expected)) = (result, expected) {
                self.check_captured_index_carrier(name, &actual, expected, span)?;
            }
        }
        if type_arguments.is_empty() && dictionaries.is_empty() && implicit_arguments.is_empty() { return Ok(ITerm::app_fn(head, out)); }
        let head = ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function(head), types: type_arguments,
            dictionaries, arguments: implicit_arguments,
        }));
        Ok(if out.is_empty() { head } else { ITerm::App(Box::new(head), out) })
    }

    fn proposition_indices(&self, left: &ast::Type, right: &ast::Type, vars: &[String], numeric_order: bool) -> Result<(ITerm, ITerm)> {
        let mut inference = IndexInference::new(&self.uni);
        let left_ty = self.captured_index_carrier_in(&mut inference, left, vars, None)?.map(|ty| self.numeric_index_read_carrier(&ty));
        let right_ty = self.captured_index_carrier_in(&mut inference, right, vars, left_ty.as_ref())?.map(|ty| self.numeric_index_read_carrier(&ty));
        let span = right.span();
        if let (Some(left), Some(right)) = (&left_ty, &right_ty) {
            let left = self.refine_base(left).unwrap_or_else(|| left.clone());
            let right_carrier = self.refine_base(right).unwrap_or_else(|| right.clone());
            inference.uni.unify(&left, &right_carrier).map_err(|error| Error::at(span, error))?;
        }
        self.finish_index_inference(&mut inference)?;
        let carrier = left_ty.or(right_ty).map(|ty| {
            let ty = inference.uni.zonk(&ty);
            self.refine_base(&ty).unwrap_or(ty)
        });
        if carrier.as_ref().is_some_and(|ty| !self.uni.owns_all_metas(ty)) {
            return Err(Error::at(left.span(), "proposition operands have an unsolved carrier"));
        }
        if numeric_order {
            if let Some(carrier) = &carrier {
                self.check_numeric_index_carrier(carrier, left.span())?;
            }
        }
        self.commit_index_inference(&mut inference)?;
        Ok((self.conv_idx_expected(left, vars, carrier.as_ref())?, self.conv_idx_expected(right, vars, carrier.as_ref())?))
    }

    /// Convert a proposition: a comparison over index values, `Eq x y`,
    /// `Not p`, a decidable predicate of the refinement vocabulary, or a
    /// bound proposition variable.
    fn conv_prop(&self, ty: &ast::Type, vars: &[String]) -> Result<PTerm> {
        let mut args: Vec<&ast::Type> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push(a);
                    cur = f;
                }
                _ => break,
            }
        }
        args.reverse();
        let head = cur;
        let span = ty.span();
        match head {
            ast::Type::Var(op, _) if args.len() == 2 => {
                if op.text == "&&" || op.text == "||" {
                    // `p && q` / `p || q` (refinement types design,
                    // "compound predicates"): the lattice meet and join
                    // over admissible predicates.
                    let a = self.conv_prop(args[0], vars)?;
                    let b = self.conv_prop(args[1], vars)?;
                    return Ok(if op.text == "&&" {
                        PTerm::And(Box::new(a), Box::new(b))
                    } else {
                        PTerm::Or(Box::new(a), Box::new(b))
                    });
                }
                let cmp = match op.text.as_str() {
                    "<" => IdxCmp::Lt,
                    "<=" => IdxCmp::Le,
                    ">" => IdxCmp::Gt,
                    ">=" => IdxCmp::Ge,
                    "==" | "=" => IdxCmp::Eq,
                    "/=" | "!=" => IdxCmp::Ne,
                    _ => {
                        return Err(Error::at(
                            span,
                            format!(
                                "`{}` is not a proposition former (use a comparison, `Eq x y`, `Not p`, or `&&`/`||`)",
                                op.text
                            ),
                        ))
                    }
                };
                let (left, right) = self.proposition_indices(args[0], args[1], vars, !matches!(cmp, IdxCmp::Eq | IdxCmp::Ne))?;
                Ok(PTerm::Cmp(cmp, Box::new(left), Box::new(right)))
            }
            ast::Type::Con(p, _) if args.is_empty() && (p.parts() == ["True"] || p.parts() == ["False"]) => {
                Ok(PTerm::Lit(p.parts() == ["True"]))
            }
            ast::Type::Con(p, _) if p.parts() == ["Eq"] && args.len() == 2 => {
                let (left, right) = self.proposition_indices(args[0], args[1], vars, false)?;
                Ok(PTerm::Eq(Box::new(left), Box::new(right)))
            }
            ast::Type::Con(p, _) if p.parts() == ["Not"] && args.len() == 1 => {
                Ok(PTerm::Not(Box::new(self.conv_prop(args[0], vars)?)))
            }
            ast::Type::Con(p, _) if args.len() == 1 => {
                // The refinement vocabulary's structural decidable
                // predicates (refinement types design, "shape"): a named
                // predicate over one index value (`NotNull xs`,
                // `NonZero n`, `Ascii s`, `Ordered xs`). The predicate's
                // operand shapes are checked where the predicate is used
                // (a refinement declaration or a `decide`).
                let name = p.parts().join(".");
                let pf = PredFn::lookup(&name);
                if let Some(pf) = pf {
                    // Structural predicates also decide lists, strings and
                    // other containers. A numeric expectation here would
                    // incorrectly constrain a checked operand's result type.
                    let operand = self.conv_idx_expected(args[0], vars, None)?;
                    return Ok(PTerm::Pred(pf, Box::new(operand)));
                }
                // A named refinement's predicate applied to a value
                // (`Sorted xs`, `NonEmpty xs`): nominal names are named
                // predicates for readability, nothing more.
                let info = self.lookup_type_path(p).ok();
                let tk = info.filter(|i| i.refine.is_some());
                if let Some(r) = tk.and_then(|t| t.refine) {
                    if r.params.is_empty() {
                        let operand = self.conv_idx_expected(args[0], vars, Some(&r.base))?;
                        return Ok(Checker::subst_pt(
                            &r.pred,
                            &HashMap::from([(
                                r.binder.clone(),
                                Ty::Idx(Box::new(operand)),
                            )]),
                        ));
                    }
                }
                Err(Error::at(
                    span,
                    format!(
                        "`{name}` is not a proposition former (use a comparison, `Eq x y`, `Not p`, a decidable predicate: `NotNull`, `NonZero`, `Ascii`, `Ordered`, or a named refinement over one value)"
                    ),
                ))
            }
            ast::Type::Con(p, _) if args.len() > 1 => {
                // A named refinement with index parameters applied to
                // its index arguments and a value (`InRange 0 255 r`, the
                // predicate of the type `InRange 0 255 Nat`): one
                // spelling for the type and its predicate.
                let name = p.parts().join(".");
                let tk = self.lookup_type_path(p).ok().filter(|i| i.refine.is_some());
                if let Some(tk) = tk {
                    let r = tk.refine.clone().expect("refine is some");
                    let idx: Vec<&String> = r
                        .params
                        .iter()
                        .zip(&tk.param_kinds)
                        .filter(|(_, k)| matches!(k, ParamKind::Val(_)))
                        .map(|(p, _)| p)
                        .collect();
                    if idx.len() + 1 == args.len() {
                        let unit = Ty::Nat;
                        let mut map: HashMap<String, Ty> = HashMap::new();
                        for (p, a) in idx.iter().zip(&args) {
                            map.insert((*p).clone(), Ty::Idx(Box::new(self.conv_idx(a, vars, &unit)?)));
                        }
                        let operand = self.conv_idx(args[args.len() - 1], vars, &unit)?;
                        map.insert(r.binder.clone(), Ty::Idx(Box::new(operand)));
                        return Ok(Checker::subst_pt(&r.pred, &map));
                    }
                }
                Err(Error::at(
                    span,
                    format!(
                        "`{name}` is not a proposition former here (a named refinement's predicate takes its index arguments and one value: `InRange 0 255 x`)"
                    ),
                ))
            }
            ast::Type::Var(n, _) if args.is_empty() && vars.contains(&n.text) => {
                self.note_kind(&n.text, ParamKind::Prop);
                Ok(PTerm::Var(n.text.clone()))
            }
            other => Err(Error::at(
                other.span(),
                "expected a proposition: a comparison (`i < n`), `Eq x y`, `Not p`, a decidable predicate (`NotNull`, `NonZero`, `Ascii`, `Ordered`), or a proposition variable",
            )),
        }
    }

    /// The admissible predicate forms of a refinement (refinement types
    /// design, "compound predicates"). Admitted: the `Dec` propositions
    /// over index terms (bounded numeric comparisons and index
    /// equalities) when the refined container is numeric or when the
    /// binder appears only under a baked measure (`length xs > 100` over
    /// any container), the structural vocabulary predicates applied to
    /// the refined binder's value itself, `Not` over admitted forms, and
    /// `&&`/`||` compounds of admitted forms (conjunction and
    /// disjunction of decidable propositions are decidable). Everything
    /// else is refused naming the form: quantified propositions (infinite
    /// domains need induction, the D113 discipline), predicates over
    /// anything but the container's value, measures other than the baked
    /// `length`/`toNat` vocabulary (never source a shadowing module could
    /// lie with), and proposition variables (their truth is assumed,
    /// never decided).
    fn check_pred_admissible(
        &self,
        pred: &PTerm,
        binder: &str,
        base: &Ty,
        span: ast::Span,
    ) -> Result<()> {
        match pred {
            PTerm::Lit(_) => Ok(()),
            PTerm::Pred(pf, operand) => {
                if !matches!(&**operand, ITerm::Var(n) if n == binder) {
                    return Err(Error::at(
                        span,
                        format!(
                            "`{pf}` applies to the refined binder `{binder}`'s value; a predicate over anything else is refused (the decidable vocabulary speaks about the container's values)"
                        ),
                    ));
                }
                self.check_pred_operand(*pf, base, span)
            }
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                if a.mentions(binder) || b.mentions(binder) {
                    match base {
                        Ty::Nat | Ty::Int | Ty::Prim(_) => Ok(()),
                        other => {
                            // A measure atom over any container: the
                            // binder may appear only under the baked
                            // measures (`length xs > 100`).
                            if measure_covers(a, binder, matches!(base, Ty::Bytes)) && measure_covers(b, binder, matches!(base, Ty::Bytes)) {
                                Ok(())
                            } else {
                                Err(Error::at(
                                    span,
                                    crate::diagnostic!(
                                        "a comparison or index equality is an admitted predicate over numeric values (bounded numeric comparisons and index equalities) or over the baked measures of the binder (`length xs > 100`), not over `{}` as written (the structural vocabulary decides container values: `NotNull`, `NonZero`, `Ascii`, `Ordered`)",
                                        other
                                    ),
                                ))
                            }
                        }
                    }
                } else {
                    Ok(())
                }
            }
            PTerm::Not(q) => self.check_pred_admissible(q, binder, base, span),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                self.check_pred_admissible(a, binder, base, span)?;
                self.check_pred_admissible(b, binder, base, span)
            }
            PTerm::Var(_) | PTerm::MVar(_) => Err(Error::at(
                span,
                "a proposition variable is not a decidable predicate: its truth is assumed, never decided (state the predicate over the refined binder)",
            )),
        }
    }

    /// A list-shaped container: constructors that distinguish the empty
    /// form (no runtime fields) from a non-empty (element-tail) form.
    fn list_shaped(&self, name: &str) -> bool {
        self.known_type(name)
            .map(|tk| {
                    let mut empties = 0usize;
                    let mut non_empties = 0usize;
                    for (cname, _) in &tk.ctors {
                        let ra = self
                            .qualified
                            .get(&(tk.module.clone(), cname.clone()))
                            .and_then(|v| match &v.kind {
                                ValueKind::Ctor { runtime_arity, .. } => Some(*runtime_arity),
                                _ => None,
                            })
                            .unwrap_or(1);
                        if ra == 0 {
                            empties += 1;
                        } else {
                            non_empties += 1;
                        }
                    }
                    empties >= 1 && non_empties >= 1
            })
            .unwrap_or(false)
    }

    /// The operand shapes one structural-vocabulary predicate decides
    /// over (a wrong predicate is refused naming it and its admissible
    /// operand). A refined value reads as its container (the runtime
    /// representation IS the container).
    fn check_pred_operand(&self, pf: PredFn, base: &Ty, span: ast::Span) -> Result<()> {
        let base = self
            .refine_view(base)
            .and_then(|v| v.base)
            .unwrap_or_else(|| base.clone());
        let base = &base;
        let ok = match pf {
            PredFn::NotNull | PredFn::Ordered => {
                // A list-shaped container: constructors that distinguish
                // the empty form (no runtime fields) from non-empty
                // forms. `Ordered` further needs numeric elements.
                let Ty::Adt(name, args) = base else {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!(
                            "`{}` decides {}, not `{}` (a wrong predicate for this refinement's container is refused naming the predicate)",
                            pf, pf.operand_note(), base
                        ),
                    ));
                };
                let shape_ok = self.list_shaped(name);
                let elems_ok = match pf {
                    PredFn::Ordered => matches!(
                        args.first(),
                        Some(Ty::Nat)
                    ),
                    _ => true,
                };
                shape_ok && elems_ok
            }
            PredFn::NonZero => {
                matches!(base, Ty::Nat | Ty::Int)
                    || matches!(base, Ty::Prim(s) if s.is_int_like())
            }
            PredFn::Ascii => match base {
                Ty::Bytes => true,
                Ty::Adt(name, _) if name == crate::known::STRING => true,
                // A list of chars: every char is below 128 (the
                // byte-range fact `Ascii` implies).
                Ty::Adt(name, args) => {
                    matches!(args.first(), Some(Ty::Prim(s)) if *s == Scalar::Char)
                        && self.list_shaped(name)
                }
                _ => false,
            },
        };
        if ok {
            Ok(())
        } else {
            Err(Error::at(
                span,
                crate::diagnostic!(
                    "`{}` decides {}, not `{}` (a wrong predicate for this refinement's container is refused naming the predicate)",
                    pf, pf.operand_note(), base
                ),
            ))
        }
    }

    /// One type's lattice point in the refinement family, whether named
    /// (`SizedAscii`) or anonymous (`refined P T` / `refined P`):
    /// `None` for non-refinements.
    fn refine_view(&self, t: &Ty) -> Option<RefineView> {
        match t {
            Ty::Adt(name, args) if name == "refined" => match args.as_slice() {
                [Ty::Prp(p)] => Some(RefineView {
                    base: None,
                    pred: (**p).clone(),
                    binder: pred_binder(p),
                }),
                [Ty::Prp(p), base] => Some(RefineView {
                    base: Some(base.clone()),
                    pred: (**p).clone(),
                    binder: pred_binder(p),
                }),
                _ => None,
            },
            Ty::Adt(name, args) => {
                let tk = self.known_type(name).filter(|t| t.refine.is_some())?;
                let r = tk.refine.as_ref()?;
                let mut map: HashMap<String, Ty> = HashMap::new();
                for (i, p) in r.params.iter().enumerate() {
                    if let Some(a) = args.get(i) {
                        map.insert(p.clone(), a.clone());
                    }
                }
                Some(RefineView {
                    base: Some(Self::subst_ty(&r.base, &map)),
                    pred: Checker::subst_pt(&r.pred, &map),
                    binder: r.binder.clone(),
                })
            }
            _ => None,
        }
    }

    /// A range refinement: a numeric container (`Nat`, `Int`, a machine
    /// integer, `Char`) under comparisons and their conjunctions only
    /// (`InRange 0 255 Nat`, `refined (r < d) Nat`). Such a value is a
    /// fact about a number rather than a carrier of its own, so an open
    /// expectation takes its container (see `unify_arg`).
    pub(crate) fn is_range_refinement(&self, rv: &RefineView) -> bool {
        rv.base.as_ref().is_some_and(|b| self.int_view(b).is_some_and(|(_, inner)| inner.is_none()))
            && Self::is_comparison_refinement(rv)
    }

    /// A refinement whose predicate is comparisons and their conjunctions
    /// only, over any container (`InRange 0 255 Nat`, `refined (bytes-len
    /// out == n) Bytes`): what the difference-logic discharge reads, so
    /// such a value enters a stronger refinement of the same kind by the
    /// facts in scope (see `check_subsume`).
    pub(crate) fn is_comparison_refinement(rv: &RefineView) -> bool {
        fn ranges(p: &PTerm) -> bool {
            match p {
                PTerm::Cmp(..) | PTerm::Eq(..) => true,
                PTerm::And(a, b) => ranges(a) && ranges(b),
                _ => false,
            }
        }
        ranges(&rv.pred)
    }

    /// `refined P T` (the anonymous lattice point of `T` by `P`), and
    /// `refined P` where the container is left to the ascription (the
    /// everyday form of partial weakening, refinement types design).
    fn conv_refine_former(
        &self,
        args: &[&ast::Type],
        vars: &[String],
        span: ast::Span,
    ) -> Result<Ty> {
        let pred = match args.first() {
            Some(p) => self.conv_pred_former(p, vars)?,
            None => {
                return Err(Error::at(
                    span,
                    "the `refined` former takes a predicate and its container (`refined (Ascii) (List Char)`); the container may be omitted in an ascription",
                ))
            }
        };
        if args.len() > 2 {
            return Err(Error::at(
                span,
                "the `refined` former takes a predicate and at most one container (`refined P T`)",
            ));
        }
        // The implicit refined value is the predicate's unbound name,
        // not the first name visited: an earlier telescope binder can
        // precede it (n <= bytes-len bytes). Retain that distinction in
        // the existing canonical binder understood by all refinement
        // consumers, before the surrounding telescope is transformed.
        let mut free = Vec::new();
        pred.free_vars(&mut free);
        let implicit: Vec<_> = free.iter()
            .filter(|name| !vars.contains(name) && !crate::types::is_positional_arg(name))
            .collect();
        let pred = match implicit.as_slice() {
            [binder] => pred.subst(binder, &ITerm::Var("$it".to_string().into())),
            _ => pred,
        };
        let mut out = vec![Ty::Prp(Box::new(pred))];
        if let Some(t) = args.get(1) {
            out.push(self.conv_type(t, vars)?);
        }
        Ok(Ty::Adt("refined".to_string().into(), out))
    }

    /// The predicate former's grammar (the `refined` type former's `P`):
    /// a named predicate of the closed vocabulary or of a named
    /// refinement (`refined Ascii`, `refined Sorted`) is the predicate
    /// over the implicit binder, and a compound predicate binds its free
    /// names (`refined (length xs > 100 && Ascii xs)` binds `xs`).
    fn conv_pred_former(&self, ty: &ast::Type, vars: &[String]) -> Result<PTerm> {
        let mut args: Vec<&ast::Type> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push(a);
                    cur = f;
                }
                _ => break,
            }
        }
        args.reverse();
        if args.is_empty() {
            if let ast::Type::Con(p, span) = cur {
                let name = p.parts().join(".");
                if let Some(pf) = PredFn::lookup(&name) {
                    return Ok(PTerm::Pred(pf, Box::new(ITerm::Var("$it".to_string().into()))));
                }
                // A named refinement's predicate (`refined Sorted`):
                // nominal names are named predicates for readability.
                let info = self.lookup_type_path(p).ok();
                if let Some(tk) =
                    info.filter(|i| i.refine.is_some())
                {
                    let r = tk.refine.expect("refine is some");
                    return Ok(Checker::subst_pt(
                        &r.pred,
                        &HashMap::from([(
                            r.binder.clone(),
                            Ty::Idx(Box::new(ITerm::Var("$it".to_string().into()))),
                        )]),
                    ));
                }
                return Err(Error::at(
                    *span,
                    format!(
                        "`{name}` is not a predicate of the `refined` former (use the closed vocabulary `NotNull`, `NonZero`, `Ascii`, `Ordered`, a named refinement, or a predicate over the binder)"
                    ),
                ));
            }
        }
        // A predicate over the binder: its free names are the binder
        // (the former closes the predicate over its value).
        let mut binders = Vec::new();
        self.pred_binder_names(ty, &mut binders);
        let mut vs = vars.to_vec();
        for b in binders {
            if !vs.contains(&b) {
                vs.push(b);
            }
        }
        self.conv_prop(ty, &vs)
    }

    /// The names a predicate's index expressions introduce (anything
    /// that is not the built-in measure vocabulary or a resolvable
    /// value): the `refined` former's implicit binder.
    fn pred_binder_names(&self, ty: &ast::Type, out: &mut Vec<String>) {
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    self.pred_binder_names(a, out);
                    cur = f;
                }
                _ => break,
            }
        }
        if let ast::Type::Var(n, _) = cur {
            let name = &n.text;
            if name == "length" || name == "toNat" {
                return;
            }
            if self.lookup_value(None, name).is_ok() {
                return;
            }
            if !out.contains(name) {
                out.push(name.clone());
            }
        }
    }

    /// An arrow chain's final result with its refinement erased to the
    /// container, and the refinement's view (`None` for a plain result).
    fn split_result_refinement(&self, t: &Ty) -> (Ty, Option<RefineView>) {
        match t {
            Ty::Fun(d, m, e, c) => {
                let (c, rv) = self.split_result_refinement(c);
                (Ty::Fun(d.clone(), *m, e.clone(), Box::new(c)), rv)
            }
            other => match self.refine_view(other) {
                Some(RefineView { base: Some(base), pred, binder }) => {
                    (base.clone(), Some(RefineView { base: Some(base), pred, binder }))
                }
                _ => (other.clone(), None),
            },
        }
    }

    /// Whether an intrinsic's promise (`prim_result_promise`) entails the
    /// result refinement its declaration states, under the declaration's
    /// own parameter refinements; the reason it does not otherwise.
    fn check_intrinsic_promise(&self, op: PrimOp, declared: &Ty, rv: &RefineView) -> std::result::Result<(), String> {
        let Some((promise, pre)) = prim_result_promise(op) else {
            return Err("the intrinsic promises nothing of its result (its declared result is its container)".to_string());
        };
        let mut facts: Vec<PTerm> = Vec::new();
        let mut vars: Vec<(String, IntSort)> = Vec::new();
        let mut cur = declared;
        let mut i = 0;
        while let Ty::Fun(d, _, _, c) = cur {
            let name = format!("$a{i}");
            if let Some((sort, view)) = self.int_view(d) {
                vars.push((name.clone(), sort));
                if let Some(v) = view {
                    facts.push(v.pred.subst(&v.binder, &ITerm::Var(name.clone().into())));
                }
            }
            cur = c;
            i += 1;
        }
        if let Some((sort, _)) = rv.base.as_ref().and_then(|b| self.int_view(b)) {
            vars.push(("$it".to_string(), sort));
        }
        let entails = |hyps: &[PTerm], goal: &PTerm| -> bool {
            if hyps.iter().any(|h| pred_entails(h, goal)) {
                return true;
            }
            let mut heads: Vec<(String, usize)> = Vec::new();
            for p in hyps.iter().chain(std::iter::once(goal)) {
                index_heads(p, &mut heads);
            }
            let fns = heads.into_iter().filter_map(|(h, k)| self.fn_int_sort(&h, k).map(|s| (h, k, s))).collect();
            linear_entails(hyps, goal, &Sorts { vars: vars.clone(), fns, values: vec![] }).is_some()
        };
        if let Some(pre) = &pre {
            if !entails(&facts, pre) {
                return Err(format!(
                    "its promise needs `{pre}` of its arguments: state it in a parameter's refinement"
                ));
            }
        }
        let goal = rv.pred.subst(&rv.binder, &ITerm::Var("$it".to_string().into()));
        facts.push(promise.clone());
        if entails(&facts, &goal) {
            Ok(())
        } else {
            Err(format!("the intrinsic promises `{promise}` of its result `$it`, which does not entail `{goal}`"))
        }
    }

    /// An arrow chain with each parameter's refinement erased to its
    /// container (`Nat -> NonZero Nat -> Nat` is `Nat -> Nat -> Nat`);
    /// the result is kept as written. An intrinsic's declared type is
    /// compared against its known signature through this view and
    /// `split_result_refinement`, so a declaration may add a
    /// precondition, and a promise only where the intrinsic keeps it.
    fn erase_param_refinements(&self, t: &Ty) -> Ty {
        match t {
            Ty::Fun(dom, m, eff, cod) => Ty::Fun(
                Box::new(self.refine_base(dom).unwrap_or_else(|| (**dom).clone())),
                *m,
                eff.clone(),
                Box::new(self.erase_param_refinements(cod)),
            ),
            other => other.clone(),
        }
    }

    /// The container instance of a refinement application (`NonEmpty x`'s
    /// base is `List x`): the declared container with the application's
    /// arguments substituted for the refinement's parameters. A partial
    /// application (the higher-kinded `NonEmpty` former) yields the
    /// container former. An anonymous `refined P T` yields `T`.
    /// `None` for non-refinements.
    fn refine_base(&self, t: &Ty) -> Option<Ty> {
        let Ty::Adt(name, args) = t else {
            return None;
        };
        if name == "refined" {
            return match args.as_slice() {
                [_, base] => Some(base.clone()),
                _ => None,
            };
        }
        let tk = self.known_type(name)?;
        let r = tk.refine.as_ref()?;
        let mut map: HashMap<String, Ty> = HashMap::new();
        for (i, p) in r.params.iter().enumerate() {
            if let Some(a) = args.get(i) {
                map.insert(p.clone(), a.clone());
            }
        }
        let base = Self::subst_ty(&r.base, &map);
        if args.len() < r.params.len() {
            // The former: the container with the still-open parameters
            // dropped from its application spine.
            if let Ty::Adt(n, as_) = base {
                let mut as_ = as_;
                while as_.len() > args.len() {
                    as_.pop();
                }
                return Some(Ty::Adt(n, as_));
            }
        }
        Some(base)
    }

    /// The first refinement type name a type mentions (anywhere in its
    /// structure), for the preservation obligation's result scan.
    fn ty_refine_mention(&self, t: &Ty) -> Option<String> {
        if let Ty::Adt(n, args) = t {
            if self.known_type(n).is_some_and(|tk| tk.refine.is_some())
            {
                return Some(n.clone().to_string());
            }
            return args.iter().find_map(|a| self.ty_refine_mention(a));
        }
        match t {
            Ty::App(h, args) => self
                .ty_refine_mention(h)
                .or_else(|| args.iter().find_map(|a| self.ty_refine_mention(a))),
            Ty::Tuple(args) => args.iter().find_map(|a| self.ty_refine_mention(a)),
            Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
                self.ty_refine_mention(a).or_else(|| self.ty_refine_mention(b))
            }
            Ty::Sigma { dom, cod, .. } => self
                .ty_refine_mention(dom)
                .or_else(|| self.ty_refine_mention(cod)),
            Ty::Forall(scheme) => self.ty_refine_mention(&scheme.ty),
            Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } => {
                self.ty_refine_mention(inner)
            }
            Ty::Ref(_, i) | Ty::Mut(_, i) => self.ty_refine_mention(i),
            Ty::Handler { input, output, .. } => self
                .ty_refine_mention(input)
                .or_else(|| self.ty_refine_mention(output)),
            _ => None,
        }
    }

    /// Convert an effect row. Label arguments are region names; the tail is
    /// a rigid row variable from the enclosing `forall`.
    fn effect_head(&self, path: &ast::ConPath, declared: bool) -> Result<String> {
        let name = path.segments.last().ok_or_else(|| Error::plain("empty effect path"))?;
        let lookup = |owner: &str, member: &str| self.effect_bindings.get(&(owner.to_string(), member.to_string())).cloned();
        if let Some(binding) = name.declaration_binding() {
            let (owner, member) = self.declaration_bindings.get(&(binding.clone(), ast::RefKind::Effect))
                .ok_or_else(|| Error::at(name.span, "owned syntax effect is not bound by this declaration group"))?;
            return lookup(owner, member).ok_or_else(|| Error::at(name.span, "owned syntax effect has no checked declaration"));
        }
        if let Some(binding) = name.resolved_binding() {
            if binding.kind() != ast::RefKind::Effect { return Err(Error::at(name.span, "this captured name is not an effect")); }
            return lookup(binding.module(), binding.name()).ok_or_else(|| Error::at(name.span, "resolved syntax effect is no longer available"));
        }
        if let Some(scope) = name.scoped_reference() {
            if scope.kind() != ast::RefKind::Effect { return Err(Error::at(name.span, "this captured name is not an effect")); }
            if let Some(binding) = scope.candidates(ast::RefKind::Effect).first() {
                return lookup(binding.module(), binding.name()).ok_or_else(|| Error::at(name.span, "captured effect is no longer available"));
            }
            if declared || path.segments.len() != 1 { return Err(Error::at(name.span, format!("unknown effect {} (not bound in captured source scope {})", path.parts().join("."), scope.origin()))); }
            // Existing open rows also contain symbolic labels, including
            // Foreign, Diverge and capability effects without operation tables.
            // A frozen symbolic label never acquires a caller's owned effect.
            return Ok(path.parts().join("."));
        }
        if name.local_binding().is_some() { return Err(Error::at(name.span, "a value binding is not an effect name")); }
        let reference = ast::RefSite { kind: ast::RefKind::Effect,
            quals: path.segments[..path.segments.len()-1].iter().map(|n| n.text.clone()).collect(),
            name: name.text.clone(), span: path.span() };
        if let Some((owner, member)) = self.resolve_top(&self.current_module, &reference) {
            if let Some(head) = lookup(&owner, &member) { return Ok(head); }
        }
        if path.segments.len() == 1 {
            if self.effects.contains_key(&name.text) { return Ok(name.text.clone()); }
            if !declared { return Ok(name.text.clone()); }
        }
        Err(Error::at(name.span, format!("unknown effect {}", path.parts().join("."))))
    }

    fn conv_row(&self, e: &ast::EffectRow, vars: &[String]) -> Result<EffRow> {
        let mut labels = Vec::new();
        for (path, args) in &e.labels {
            let head = self.effect_head(path, false)?;
            let arg = match args.as_slice() {
                [] => None,
                [n] => Some(n.text.clone()),
                _ => {
                    return Err(Error::at(
                        path.span(),
                        "effect labels take at most one region argument in the bootstrap slice",
                    ));
                }
            };
            labels.push(EffLabel { head: head.into(), arg: arg.map(|a| a.as_str().into()) });
        }
        let tail = match &e.tail {
            None => RowTail::Closed,
            Some(n) => {
                if !vars.contains(&n.text) {
                    return Err(Error::at(
                        n.span,
                        format!("unknown row variable {}", n.text),
                    ));
                }
                RowTail::Rigid(n.text.clone().into())
            }
        };
        Ok(EffRow { labels, tail })
    }

    /// A region argument of a builtin applied type (`Io r`, `Scope r s`,
    /// `Task s x a`): a `forall`-bound region variable.
    fn region_arg(&self, t: &ast::Type, vars: &[String], what: &str) -> Result<String> {
        match t {
            ast::Type::Var(n, _) => {
                if !vars.contains(&n.text) {
                    return Err(Error::at(
                        n.span,
                        format!("unknown region variable {}", n.text),
                    ));
                }
                Ok(n.text.clone())
            }
            other => Err(Error::at(
                other.span(),
                format!("{what} must be a region variable"),
            )),
        }
    }

    /// Convert a possibly applied type. `Thunk {e} a` and `Lazy {e} r a`
    /// are builtin type constructors taking an effect-row argument first.
    fn conv_spine(
        &self,
        ty: &ast::Type,
        vars: &[String],
        expect: Option<&ParamKind>,
    ) -> Result<Ty> {
        enum Arg {
            Ty(ast::Type),
            Row(ast::EffectRow),
        }
        let mut args: Vec<Arg> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push(Arg::Ty((**a).clone()));
                    cur = f;
                }
                ast::Type::AppRow(f, r, _) => {
                    args.push(Arg::Row(r.clone()));
                    cur = f;
                }
                _ => break,
            }
        }
        args.reverse();
        let head_span = cur.span();
        // Propositions (`i < n`, `Eq x y`, `Not p`) are types in `Prop`
        // (the surface shares term syntax between kinds, types and
        // values), and arithmetic computes index values, not types.
        let head_name = match cur {
            ast::Type::Var(n, _) => Some(n.text.clone()),
            ast::Type::Con(p, _) => Some(p.parts().join(".")),
            _ => None,
        };
        match head_name.as_deref() {
            Some("<" | "<=" | ">" | ">=" | "==" | "=" | "/=" | "!=") => {
                return Ok(Ty::Prp(Box::new(self.conv_prop(ty, vars)?)));
            }
            Some("Eq" | "Not" | "&&" | "||") => {
                return Ok(Ty::Prp(Box::new(self.conv_prop(ty, vars)?)));
            }
            Some(pred @ ("NotNull" | "NonZero" | "Ascii" | "Ordered"))
                if args.len() == 1
                    && !matches!(&args[0], Arg::Ty(a) if self.refinement_type_app(pred, a)) =>
            {
                return Ok(Ty::Prp(Box::new(self.conv_prop(ty, vars)?)));
            }
            Some("refined") => {
                // `refined P T` (and `refined P` where the container is
                // left to the ascription): the refinement lattice's
                // anonymous point, the refinement of `T` by `P` over the
                // predicate's binder (refinement types design,
                // "instance heads at refinements").
                let mut pargs: Vec<&ast::Type> = Vec::new();
                for a in &args {
                    match a {
                        Arg::Ty(t) => pargs.push(t),
                        Arg::Row(_) => {
                            return Err(Error::at(
                                head_span,
                                "`refined` takes a predicate and its container, not an effect row",
                            ))
                        }
                    }
                }
                return self.conv_refine_former(&pargs, vars, head_span);
            }
            Some("+" | "-" | "*") => {
                return Err(Error::at(
                    head_span,
                    "an arithmetic expression computes an index value, not a type; index expressions appear as arguments of an indexed family (as in `Vec (n + 1) a`)",
                ));
            }
            _ => {}
        }
        // Type formers (`(->)`, `(,)`/`(,,)`/...): prefix application of
        // the pure total arrow family or a structural tuple former
        // (partially applied, they are class arguments like `Functor
        // ((->) r)`).
        let former = match cur {
            ast::Type::Con(p, _) if p.parts().len() == 1 => former_of(p.parts()[0]),
            _ => None,
        };
        if let Some(former) = former {
            if args.len() > former.arity() {
                return Err(Error::at(
                    head_span,
                    format!("the type former takes at most {} arguments", former.arity()),
                ));
            }
            let mut result = Ty::Former(former);
            for a in args {
                match a {
                    Arg::Ty(t) => {
                        let arg = self.conv_type(&t, vars)?;
                        result = self.type_app(result, arg, head_span)?;
                    }
                    Arg::Row(r) => {
                        return Err(Error::at(
                            r.span,
                            "effect rows may only appear as arguments to Thunk and Lazy",
                        ));
                    }
                }
            }
            return Ok(result);
        }
        if let ast::Type::Con(p, _) = cur {
            let parts = p.parts();
            match (parts.as_slice(), args.as_slice()) {
                (["Thunk"], [Arg::Row(e), Arg::Ty(a)]) => {
                    return Ok(Ty::Thunk {
                        eff: self.conv_row(e, vars)?,
                        inner: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Lazy"], [Arg::Row(e), Arg::Ty(r), Arg::Ty(a)]) => {
                    let region = match r {
                        ast::Type::Var(n, _) => {
                            if !vars.contains(&n.text) {
                                return Err(Error::at(
                                    n.span,
                                    format!("unknown region variable {}", n.text),
                                ));
                            }
                            n.text.clone()
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "the region argument of Lazy must be a region variable",
                            ));
                        }
                    };
                    return Ok(Ty::Lazy {
                        eff: self.conv_row(e, vars)?,
                        region: region.into(),
                        inner: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Thunk"] | ["Lazy"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Thunk and Lazy must be fully applied as `Thunk {e} a` and `Lazy {e} r a`",
                    ));
                }
                (["Later"], [Arg::Ty(a)]) => {
                    // `Later a`: the guarded observation modality (core
                    // language specification section 9). One type argument,
                    // no effect row: it carries no effects to hide.
                    return Ok(Ty::Later(Box::new(self.conv_type(a, vars)?)));
                }
                (["Later"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Later must be applied to exactly one type (`Later a`): it is a logical modality and takes no effect row; effectful deferral is `Thunk {e} a`",
                    ));
                }
                (["Ref"] | ["Mut"], [Arg::Ty(r), Arg::Ty(a)]) => {
                    let is_ref = parts[0] == "Ref";
                    let region = match r {
                        ast::Type::Var(n, _) => {
                            if !vars.contains(&n.text) {
                                return Err(Error::at(
                                    n.span,
                                    format!("unknown region variable {}", n.text),
                                ));
                            }
                            n.text.clone()
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "the region argument of Ref and Mut must be a region variable",
                            ));
                        }
                    };
                    let inner = self.conv_type(a, vars)?;
                    return Ok(if is_ref {
                        Ty::Ref(region.into(), Box::new(inner))
                    } else {
                        Ty::Mut(region.into(), Box::new(inner))
                    });
                }
                (["Ref"] | ["Mut"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Ref and Mut must be fully applied as `Ref r a` and `Mut r a`",
                    ));
                }
                (["Io"], [Arg::Ty(r)]) => {
                    // `Io r`: runtime evidence for identity `r` (decision
                    // D21).
                    let region = match r {
                        ast::Type::Var(n, _) => {
                            if !vars.contains(&n.text) {
                                return Err(Error::at(
                                    n.span,
                                    format!("unknown region variable {}", n.text),
                                ));
                            }
                            n.text.clone()
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "the argument of Io is a runtime identity (a region variable)",
                            ));
                        }
                    };
                    return Ok(Ty::Adt("Io".into(), vec![Ty::Region(region.into())]));
                }
                (["Io"], _) => {
                    return Err(Error::at(head_span, "Io must be fully applied as `Io r`"));
                }
                (["Scope"], [Arg::Ty(r), Arg::Ty(s)]) => {
                    // `Scope r s`: a structured scope of runtime identity
                    // `r`, bounding the tasks of scope region `s`
                    // (concurrency specification, section 2).
                    let r = self.region_arg(r, vars, "Scope's runtime identity")?;
                    let s = self.region_arg(s, vars, "Scope's scope region")?;
                    return Ok(Ty::Adt(
                        "Scope".into(),
                        vec![Ty::Region(r.into()), Ty::Region(s.into())],
                    ));
                }
                (["Scope"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Scope must be fully applied as `Scope r s`",
                    ));
                }
                (["Task"], [Arg::Ty(s), Arg::Ty(x), Arg::Ty(v)]) => {
                    // `Task s x a`: a handle to a scoped task producing
                    // `a` with task outcome `x` (concurrency
                    // specification, sections 2 and 4).
                    let s = self.region_arg(s, vars, "Task's scope region")?;
                    return Ok(Ty::Adt(
                        "Task".into(),
                        vec![Ty::Region(s.into()), self.conv_type(x, vars)?, self.conv_type(v, vars)?],
                    ));
                }
                (["Task"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Task must be fully applied as `Task s x a`",
                    ));
                }
                // Boundary pointers (foreign function interface
                // specification section 3): every pointer role wraps its
                // pointee type; `Owned` additionally names the allocator
                // identity.
                (["RawPtr"], [Arg::Ty(a)]) => {
                    return Ok(Ty::Ptr {
                        role: PtrRole::Raw,
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Borrowed"], [Arg::Ty(a)]) => {
                    return Ok(Ty::Ptr {
                        role: PtrRole::Borrowed,
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Retained"], [Arg::Ty(a)]) => {
                    return Ok(Ty::Ptr {
                        role: PtrRole::Retained,
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["MaybeNull"], [Arg::Ty(a)]) => {
                    return Ok(Ty::Ptr {
                        role: PtrRole::MaybeNull,
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Out"], [Arg::Ty(a)]) => {
                    return Ok(Ty::Ptr {
                        role: PtrRole::Out,
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Owned"], [Arg::Ty(tok), Arg::Ty(a)]) => {
                    let alloc = match tok {
                        ast::Type::Con(p, _) if p.parts().len() == 1 => p.parts()[0].to_string(),
                        ast::Type::Var(n, _) => n.text.clone(),
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "the allocator identity of an owned pointer is a bare name (`Owned CodecHeap U8`); a foreign allocation is released only by its matching deallocator (foreign function interface specification section 4)",
                            ))
                        }
                    };
                    return Ok(Ty::Ptr {
                        role: PtrRole::Owned(alloc),
                        pointee: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (
                    ["RawPtr"] | ["Borrowed"] | ["Retained"] | ["MaybeNull"] | ["Out"]
                    | ["Owned"],
                    _,
                ) => {
                    return Err(Error::at(
                        head_span,
                        "RawPtr and the ownership roles must be fully applied (`RawPtr t`, `Borrowed t`, `Retained t`, `Owned Alloc t`, `Out t`, `MaybeNull t`)",
                    ));
                }
                _ => {}
            }
        }
        // Var-headed application (`f a`, and `F a` for a capitalized
        // signature parameter): a higher-kinded application spine
        // (modules and compilation specification section 1). The head's
        // kind is inferred from the application count and the expected
        // result kind.
        let var_head = match cur {
            ast::Type::Var(n, _) if vars.contains(&n.text) => Some(n.text.clone()),
            ast::Type::Con(p, _)
                if p.parts().len() == 1 && vars.iter().any(|v| v == p.parts()[0]) =>
            {
                Some(p.parts()[0].to_string())
            }
            _ => None,
        };
        if let Some(name) = var_head {
            self.note_kind(
                &name,
                ParamKind::Con(args.len() + expect.map(|k| k.arity()).unwrap_or(0)),
            );
            let mut result = Ty::Param(name.into());
            for a in args {
                match a {
                    Arg::Ty(t) => {
                        let arg = self.conv_type(&t, vars)?;
                        if let Ty::Sort(Sort::Type(n)) = &arg {
                            return Err(Error::at(
                                t.span(),
                                format!(
                                    "the universe `Type {n}` has kind `Type {}`: `Type : Type` is not admitted",
                                    n + 1
                                ),
                            ));
                        }
                        result = self.type_app(result, arg, head_span)?;
                    }
                    Arg::Row(r) => {
                        return Err(Error::at(
                            r.span,
                            "effect rows may only appear as arguments to Thunk and Lazy",
                        ));
                    }
                }
            }
            return Ok(result);
        }
        // Generic application: arguments convert by the head's parameter
        // kinds, so `Vec n a` reads `n` as an index value and `a` as a
        // type (dependent core kernel design, section 5). A type slot
        // carries no kind information for an un-annotated binder, so its
        // bare variables are not noted.
        let info = match cur {
            ast::Type::Con(p, span) => {
                Some(self.lookup_type_path(p).map_err(|_| {
                    Error::at(*span, format!("unknown type {}", p.parts().join(".")))
                })?)
            }
            _ => None,
        };
        let mut result = self.conv_raw(cur, vars, None)?;
        for (i, a) in args.into_iter().enumerate() {
            match a {
                Arg::Ty(t) => {
                    let slot = info.as_ref().and_then(|ik| ik.param_kinds.get(i).cloned());
                    let arg = match &slot {
                        Some(ParamKind::Val(vty)) => {
                            Ty::Idx(Box::new(self.conv_idx(&t, vars, vty)?))
                        }
                        Some(ParamKind::Prop) => Ty::Prp(Box::new(self.conv_prop(&t, vars)?)),
                        Some(k @ ParamKind::Con(_)) => {
                            self.conv_kind(&t, vars, Some(k))?
                        }
                        _ => self.conv_kind(&t, vars, None)?,
                    };
                    // A universe argument claims `Type : Type`: `Type n`
                    // has kind `Type (n + 1)` (core specification
                    // section 2), so it cannot fill a `Type 0` slot.
                    if let Ty::Sort(Sort::Type(n)) = &arg {
                        return Err(Error::at(
                            t.span(),
                            format!(
                                "the universe `Type {n}` has kind `Type {}`: `Type : Type` is not admitted",
                                n + 1
                            ),
                        ));
                    }
                    result = self.type_app(result, arg, head_span)?;
                }
                Arg::Row(r) => {
                    return Err(Error::at(
                        r.span,
                        "effect rows may only appear as arguments to Thunk and Lazy",
                    ));
                }
            }
        }
        Ok(result)
    }

    fn conv_raw(
        &self,
        ty: &ast::Type,
        vars: &[String],
        expect: Option<&ParamKind>,
    ) -> Result<Ty> {
        match ty {
            ast::Type::IndexOperand(value, _, _) => Ok(Ty::Idx(Box::new(self.uni.resolve_idx(value)))),
            ast::Type::IndexAnnotated(..) => Ok(Ty::Idx(Box::new(self.conv_idx_expected(ty, vars, None)?))),
            ast::Type::Elaborated(scheme, span) => {
                Self::closed_index_signature(scheme).ok_or_else(|| Error::at(
                    *span, "a polymorphic Code expectation must be instantiated by annotation checking",
                ))
            }
            ast::Type::Lam(_, _, span) => Err(Error::at(
                *span,
                "an index-level lambda (`\\x -> ...`) is a value in an index position, not a type",
            )),
            ast::Type::Con(p, span) => {
                let parts = p.parts();
                match parts.as_slice() {
                    ["Nat"] => Ok(Ty::Nat),
                    ["Int"] => Ok(Ty::Int),
                    ["Bool"] => Ok(Ty::Bool),
                    ["Unit"] => Ok(Ty::Unit),
                    ["Bytes"] => Ok(Ty::Bytes),
                    ["Sym"] => Ok(Ty::Sym),
                    // Universes (dependent core kernel design, section 2):
                    // `Type` is `Type 0`, and `Type : Type 1`, so
                    // `Type : Type` is never admitted.
                    ["Type"] => Ok(Ty::Sort(Sort::Type(0))),
                    ["Prop"] => Ok(Ty::Sort(Sort::Prop)),
                    ["True"] => Ok(Ty::Prp(Box::new(PTerm::Lit(true)))),
                    ["False"] => Ok(Ty::Prp(Box::new(PTerm::Lit(false)))),
                    ["CVoid"] => Ok(Ty::Unit),
                    ["Thunk"] | ["Lazy"] | ["Later"] | ["Ref"] | ["Mut"] | ["Io"] => Err(Error::at(
                        *span,
                        "Thunk, Lazy, Later, Ref, Mut and Io must be fully applied (`Thunk {e} a`, `Lazy {e} r a`, `Later a`, `Ref r a`, `Mut r a`, `Io r`)",
                    )),
                    ["RawPtr"] | ["Borrowed"] | ["Retained"] | ["Owned"] | ["Out"]
                    | ["MaybeNull"] => Err(Error::at(
                        *span,
                        "RawPtr, Borrowed, Retained, Owned, Out and MaybeNull must be fully applied (`RawPtr t`, `Owned Alloc t`, ...; foreign function interface specification section 3)",
                    )),
                    [name] => {
                        // A capitalized parameter name (`signature
                        // Functor F`): a type variable in a constructor's
                        // spelling (kind binders for signature
                        // parameters).
                        if vars.contains(&name.to_string()) {
                            if self.index_carriers.borrow().iter().rev()
                                .find(|(binder, _)| binder == name)
                                .is_some_and(|(_, carrier)| *carrier == Ty::Sort(Sort::Prop))
                            {
                                self.note_kind(name, ParamKind::Prop);
                                return Ok(Ty::Prp(Box::new(PTerm::Var(name.to_string()))));
                            }
                            if let Some(k) = expect {
                                self.note_kind(name, k.clone());
                            }
                            return Ok(Ty::Param(name.to_string().into()));
                        }
                        if let Some(s) = Scalar::parse(name) {
                            return Ok(Ty::Prim(s));
                        }
                        if let Some(s) = c_abi_scalar(name) {
                            return Ok(Ty::Prim(s));
                        }
                        let info = self.lookup_type_path(p).map_err(|_| {
                            Error::at(*span, format!("unknown type {}", parts.join(".")))
                        })?;
                        // Full application is enforced by `conv_type`; here a
                        // bare constructor may still collect arguments.
                        Ok(Ty::Adt(self.type_key(&info).into(), Vec::new()))
                    }
                    _ => {
                        let info = self.lookup_type_path(p).map_err(|_| {
                            Error::at(*span, format!("unknown type {}", parts.join(".")))
                        })?;
                        // Full application is enforced by `conv_type`; here a
                        // bare constructor may still collect arguments.
                        Ok(Ty::Adt(self.type_key(&info).into(), Vec::new()))
                    }
                }
            }
            ast::Type::Var(n, _) => {
                if n.local_binding().is_some() {
                    return Err(Error::at(n.span, "a captured lexical value is not a type variable"));
                }
                if vars.contains(&n.text) {
                    if self.index_carriers.borrow().iter().rev()
                        .find(|(binder, _)| binder == &n.text)
                        .is_some_and(|(_, carrier)| *carrier == Ty::Sort(Sort::Prop))
                    {
                        self.note_kind(&n.text, ParamKind::Prop);
                        return Ok(Ty::Prp(Box::new(PTerm::Var(n.text.clone()))));
                    }
                    if let Some(k) = expect {
                        self.note_kind(&n.text, k.clone());
                    }
                    Ok(Ty::Param(n.text.clone().into()))
                } else {
                    Err(Error::at(n.span, format!("unknown type variable {}", n.text)))
                }
            }
            ast::Type::App(_, _, _) | ast::Type::AppRow(_, _, _) => {
                self.conv_spine(ty, vars, expect)
            }
            ast::Type::Fun { arg, mult, eff, res, span } => {
                let arg_ty = self.conv_type(arg, vars)?;
                let m = match mult {
                    Some(n) => Mult::parse(&n.text)
                        .ok_or_else(|| Error::at(n.span, format!("unknown multiplicity {}", n.text)))?,
                    None => default_mult(&arg_ty),
                };
                self.check_later_mult(&arg_ty, m, *span)?;
                let eff = match eff {
                    Some(e) => self.conv_row(e, vars)?,
                    None => EffRow::pure(),
                };
                let _ = span;
                Ok(Ty::fun_mult(arg_ty, m, eff, self.conv_type(res, vars)?))
            }
            ast::Type::Forall(binders, body, span) => self.conv_nested_forall(binders, body, vars, *span),
            ast::Type::Using(_, _, span) => Err(Error::at(
                *span,
                "using constraints are only supported at the outermost position of a signature",
            )),
            ast::Type::DepFun(name, ann, mult, eff, res, span) => {
                let dom = self.conv_type(ann, vars)?;
                let m = match mult {
                    Some(n) => Mult::parse(&n.text).ok_or_else(|| {
                        Error::at(n.span, format!("unknown multiplicity {}", n.text))
                    })?,
                    None => default_mult(&dom),
                };
                self.check_later_mult(&dom, m, *span)?;
                let eff = match eff {
                    Some(e) => self.conv_row(e, vars)?,
                    None => EffRow::pure(),
                };
                if vars.contains(&name.text) {
                    return Err(Error::at(name.span, format!("duplicate binder {}", name.text)));
                }
                let mut vars2 = vars.to_vec();
                vars2.push(name.text.clone());
                match &dom {
                    Ty::Sort(_) => self.note_kind(&name.text, ParamKind::Type),
                    Ty::Prp(_) => self.note_kind(&name.text, ParamKind::Prop),
                    other => self.note_kind(&name.text, ParamKind::Val(other.clone())),
                }
                self.index_carriers.borrow_mut().push((name.text.clone(), dom.clone()));
                let cod = self.conv_type(res, &vars2);
                self.index_carriers.borrow_mut().pop();
                let cod = cod?;
                let _ = span;
                if cod.mentions_idx(&name.text) {
                    Ok(Ty::pi(name.text.clone(), dom, m, eff, cod))
                } else {
                    Ok(Ty::fun_mult(dom, m, eff, cod))
                }
            }
            ast::Type::Sigma(name, ann, body, span) => {
                let dom = self.conv_type(ann, vars)?;
                if vars.contains(&name.text) {
                    return Err(Error::at(name.span, format!("duplicate binder {}", name.text)));
                }
                let mut vars2 = vars.to_vec();
                vars2.push(name.text.clone());
                match &dom {
                    Ty::Sort(_) => self.note_kind(&name.text, ParamKind::Type),
                    Ty::Prp(_) => self.note_kind(&name.text, ParamKind::Prop),
                    other => self.note_kind(&name.text, ParamKind::Val(other.clone())),
                }
                self.index_carriers.borrow_mut().push((name.text.clone(), dom.clone()));
                let cod = self.conv_type(body, &vars2);
                self.index_carriers.borrow_mut().pop();
                let cod = cod?;
                if cod.mentions_idx(&name.text) {
                    Ok(Ty::sigma(name.text.clone(), dom, cod))
                } else {
                    // A non-dependent pair is an ordinary structural
                    // product (decision D1 tuples).
                    Ok(Ty::Tuple(vec![dom, cod]))
                }
            }
            ast::Type::Num(_, span) => Err(Error::at(
                *span,
                "a numeral is an index value, not a type; index values appear as arguments of an indexed family (as in `Vec 0 a`)",
            )),
            ast::Type::Infix(_, _, span) => Err(Error::at(
                *span,
                "internal error: unresolved infix chain in a type",
            )),
            ast::Type::Unit(_) => Ok(Ty::Unit),
            ast::Type::Tuple(items, span) => {
                if items.len() < 2 {
                    return Err(Error::at(*span, "a tuple type needs at least two components"));
                }
                Ok(Ty::Tuple(
                    items
                        .iter()
                        .map(|t| self.conv_type(t, vars))
                        .collect::<Result<Vec<_>>>()?,
                ))
            }
        }
    }

    /// What a binder's annotation binds: a universe binds a type
    /// parameter, `Prop` binds a proposition, `Type -> ... -> Type` binds
    /// a type constructor parameter of that arity, and any other
    /// annotation is the type of an index value (`(n :: Nat)`).
    fn binder_kind(&self, ann: &ast::Type, vars: &[String]) -> Result<ParamKind> {
        if let ast::Type::Con(p, _) = ann {
            match p.parts().as_slice() {
                ["Type"] => return Ok(ParamKind::Type),
                ["Prop"] => return Ok(ParamKind::Prop),
                _ => {}
            }
        }
        // `Type -> ... -> Type`: a type constructor kind of `n` arrows.
        // Only `Type` domains and a `Type` codomain are kinds in this
        // slice; anything else keeps its index-value reading.
        let mut n = 0;
        let mut cur = ann;
        loop {
            match cur {
                ast::Type::Fun { arg, mult, eff, res, .. } if mult.is_none() && eff.is_none() => {
                    let is_type = |t: &ast::Type| {
                        matches!(t, ast::Type::Con(p, _) if p.parts().as_slice() == ["Type"])
                    };
                    if !is_type(arg) {
                        break;
                    }
                    n += 1;
                    cur = res;
                }
                _ => break,
            }
        }
        if n > 0 {
            if matches!(cur, ast::Type::Con(p, _) if p.parts().as_slice() == ["Type"]) {
                return Ok(ParamKind::Con(n));
            }
            // An arrow chain ending anywhere else keeps its index-value
            // reading (`(h :: Int -> Int)` is a value binder).
        }
        Ok(ParamKind::Val(self.conv_type(ann, vars)?))
    }

    /// Resolve one binder's kind from its annotation and its recorded
    /// uses (kind inference for un-annotated binders, and the annotation
    /// versus inference equality check). `notes` are the conversion's
    /// kind constraints, keyed by binder name.
    fn resolve_binder_kind(
        &self,
        name: &str,
        hint: Option<&ParamKind>,
        notes: &[(String, ParamKind)],
        span: ast::Span,
    ) -> Result<ParamKind> {
        let uses: Vec<ParamKind> = notes
            .iter()
            .filter(|(n, _)| n == name)
            .map(|(_, k)| k.clone())
            .collect();
        // A type constructor use is forced by an application site: every
        // such use must agree on the arity, and it cannot mix with a
        // plain-type or index use without an annotation.
        let cons: Vec<usize> = uses
            .iter()
            .filter_map(|k| match k {
                ParamKind::Con(n) => Some(*n),
                _ => None,
            })
            .collect();
        if let Some(hint) = hint {
            if let ParamKind::Con(n) = hint {
                if let Some(m) = cons.iter().find(|m| **m != *n) {
                    return Err(Error::at(
                        span,
                        format!(
                            "`{name}` is annotated with kind `Type -> {rest}Type` ({n} arrows) but used applied to {m} arguments",
                            rest = "Type -> ".repeat(n.saturating_sub(1))
                        ),
                    ));
                }
                if uses.iter().any(|k| matches!(k, ParamKind::Type | ParamKind::Val(_) | ParamKind::Prop))
                {
                    return Err(Error::at(
                        span,
                        format!(
                            "`{name}` is annotated as a type constructor but also used as a plain type or value; annotate its uses consistently"
                        ),
                    ));
                }
                return Ok(hint.clone());
            }
            // The classification axis must be consistent: a binder used
            // both as a type and as an index value needs an annotation.
            let axis = |k: &ParamKind| match k {
                ParamKind::Type => 0u8,
                ParamKind::Val(_) => 1,
                ParamKind::Prop => 2,
                ParamKind::Con(_) => 3,
            };
            if let Some(k) = uses.iter().find(|k| axis(k) != axis(hint)) {
                let _ = k;
                return Err(Error::at(
                    span,
                    format!("`{name}` is used both as a type and as an index value; annotate it"),
                ));
            }
            return Ok(hint.clone());
        }
        if !cons.is_empty() {
            if uses.iter().any(|k| !matches!(k, ParamKind::Con(_))) {
                return Err(Error::at(
                    span,
                    format!(
                        "`{name}` is used both as a type constructor and as a plain type or value; annotate it"
                    ),
                ));
            }
            if cons.iter().any(|n| *n != cons[0]) {
                return Err(Error::at(
                    span,
                    format!(
                        "`{name}` is applied to different numbers of arguments; annotate its kind"
                    ),
                ));
            }
            return Ok(ParamKind::Con(cons[0]));
        }
        // The classification axis must be consistent: a binder used both
        // as a type and as an index value needs an annotation. Several
        // value uses only fix the axis, not the index type (the
        // annotation, or the first value use, supplies the type).
        let axis = |k: &ParamKind| match k {
            ParamKind::Type => 0u8,
            ParamKind::Val(_) => 1,
            ParamKind::Prop => 2,
            ParamKind::Con(_) => 3,
        };
        let mut distinct_axis: Option<u8> = None;
        let mut ambiguous = false;
        let mut first_val: Option<ParamKind> = None;
        for k in &uses {
            match distinct_axis {
                None => distinct_axis = Some(axis(k)),
                Some(prev) if prev != axis(k) => ambiguous = true,
                _ => {}
            }
            if first_val.is_none() && matches!(k, ParamKind::Val(_)) {
                first_val = Some(k.clone());
            }
        }
        Ok(match distinct_axis {
            // A mixed-use binder without an annotation keeps its
            // historic reading: a type parameter (the index unification
            // absorbs the difference).
            _ if ambiguous => ParamKind::Type,
            Some(1) => first_val.unwrap_or(ParamKind::Type),
            Some(2) => ParamKind::Prop,
            // An unused binder keeps its historic reading: a type
            // parameter.
            _ => ParamKind::Type,
        })
    }

    /// A nested quantifier (design/rank-n-polymorphism.md section 1): its
    /// binders classify as rows, regions, types or type constructors from
    /// their use in the body, exactly as a signature's do, and are renamed
    /// apart so no later substitution can capture them.
    fn conv_nested_forall(&self, binders: &[ast::TypeBinder], body: &ast::Type, vars: &[String], span: ast::Span) -> Result<Ty> {
        let mut names: Vec<String> = Vec::new();
        let mut hints = Vec::new();
        let mut explicit_rows = Vec::new();
        for b in binders {
            if vars.contains(&b.name.text) || names.contains(&b.name.text) {
                return Err(Error::at(b.name.span, format!(
                    "the nested forall binder `{}` shadows a type variable already in scope; rename it", b.name.text)));
            }
            let row = matches!(b.ann.as_ref(), Some(ast::Type::Con(path, _)) if path.parts().as_slice() == ["Row"]);
            if row { explicit_rows.push(b.name.text.clone()); }
            let hint = if row { None } else { match &b.ann { Some(ann) => Some(self.binder_kind(ann, vars)?), None => None } };
            names.push(b.name.text.clone());
            hints.push((b.name.text.clone(), hint, b.name.span));
        }
        let mut rest = body;
        while let ast::Type::Forall(more, inner, _) = rest {
            // `forall a. forall b. T` is one quantifier over `a b`.
            for b in more {
                if vars.contains(&b.name.text) || names.contains(&b.name.text) {
                    return Err(Error::at(b.name.span, format!(
                        "the nested forall binder `{}` shadows a type variable already in scope; rename it", b.name.text)));
                }
                let hint = match &b.ann { Some(ann) => Some(self.binder_kind(ann, vars)?), None => None };
                names.push(b.name.text.clone());
                hints.push((b.name.text.clone(), hint, b.name.span));
            }
            rest = inner;
        }
        if let ast::Type::Using(_, _, using_span) = rest {
            return Err(Error::at(*using_span,
                "a nested forall cannot carry `using` constraints yet; lift them to the signature (design/rank-n-polymorphism.md section 7)"));
        }
        let mut scope = vars.to_vec();
        scope.extend(names.iter().cloned());
        let converted = self.conv_type(rest, &scope)?;
        let (mut rows, mut regions) = (Vec::new(), Vec::new());
        collect_row_names(&converted, &mut rows, &mut regions);
        let row_params: Vec<String> = names.iter().filter(|n| rows.contains(n) || explicit_rows.contains(n)).cloned().collect();
        let region_params: Vec<String> = names.iter().filter(|n| regions.contains(n) && !row_params.contains(n)).cloned().collect();
        let converted = crate::types::regionize(&converted, &region_params);
        let constraints = self.kind_constraints.borrow().clone();
        let mut params = Vec::new();
        let mut con_params = Vec::new();
        for (name, hint, binder_span) in hints {
            if row_params.contains(&name) || region_params.contains(&name) { continue; }
            match self.resolve_binder_kind(&name, hint.as_ref(), &constraints, binder_span)? {
                ParamKind::Type => params.push(name),
                ParamKind::Con(n) => { con_params.push((name.clone(), n)); params.push(name); }
                ParamKind::Val(_) | ParamKind::Prop => return Err(Error::at(binder_span, format!(
                    "the nested forall binder `{name}` is used as a value or proposition; a nested quantifier binds only types, effect rows and regions"))),
            }
        }
        let _ = span;
        // Fresh binder names: `a` becomes `a#fN`.
        let mut renaming = crate::types::VarRenaming::default();
        let mut fresh = |name: &String| {
            let n = self.forall_binder_counter.get();
            self.forall_binder_counter.set(n + 1);
            format!("{name}#f{n}")
        };
        let params: Vec<String> = params.iter().map(|p| { let f = fresh(p); renaming.types.insert(p.clone(), f.clone().into()); f }).collect();
        let con_params = con_params.into_iter().map(|(p, n)| (renaming.types[&p].to_string(), n)).collect();
        let row_params: Vec<String> = row_params.iter().map(|p| { let f = fresh(p); renaming.rows.insert(p.clone(), f.clone().into()); f }).collect();
        let region_params: Vec<String> = region_params.iter().map(|p| { let f = fresh(p); renaming.regions.insert(p.clone(), f.clone().into()); f }).collect();
        let mut scheme = Scheme::mono(converted.rename_vars(&renaming));
        scheme.params = params;
        scheme.con_params = con_params;
        scheme.row_params = row_params;
        scheme.region_params = region_params;
        // A quantifier that binds nothing is its body.
        if scheme.params.is_empty() && scheme.row_params.is_empty() && scheme.region_params.is_empty() {
            return Ok(scheme.ty);
        }
        Ok(Ty::Forall(Box::new(scheme)))
    }

    fn conv_scheme(&self, ty: &ast::Type) -> Result<Scheme> {
        self.kind_constraints.borrow_mut().clear();
        let mut params: Vec<String> = Vec::new();
        let mut ctx: Vec<(String, ClassApp)> = Vec::new();
        let mut obligs: Vec<PropOblig> = Vec::new();
        let mut pending_predicates = Vec::new();
        // `forall` binders in order, with their kind hints.
        let mut binders: Vec<(String, Option<ParamKind>, ast::Span)> = Vec::new();
        // Rows occupy Scheme::row_params rather than the proper/index kind
        // namespace. Retain an explicit Row annotation until usage is known.
        let mut explicit_rows = Vec::new();
        let mut rest = ty;
        loop {
            match rest {
                ast::Type::Forall(bs, body, _) => {
                    for b in bs {
                        if params.contains(&b.name.text) {
                            return Err(Error::at(b.name.span, "duplicate type parameter"));
                        }
                        let explicit_row = matches!(b.ann.as_ref(),
                            Some(ast::Type::Con(path, _)) if path.parts().as_slice() == ["Row"]);
                        let hint = if explicit_row {
                            explicit_rows.push(b.name.text.clone());
                            None
                        } else { match &b.ann {
                            Some(ann) => Some(self.binder_kind(ann, &params)?),
                            None => None,
                        }};
                        params.push(b.name.text.clone());
                        binders.push((b.name.text.clone(), hint, b.name.span));
                    }
                    rest = body;
                }
                ast::Type::Using(cs, body, _) => {
                    for c in cs {
                        match c {
                            ast::Oblig::Class(class) => ctx.push(self.conv_class(class, &params)?),
                            ast::Oblig::Preserves { .. } => {
                                return Err(Error::at(
                                    c.span(),
                                    "`Preserves` is the generic transfer's obligation on an instance; a signature's `using` context states per-argument predicates (`Ordered xs`) or class constraints",
                                ))
                            }
                            ast::Oblig::Pred(_) => pending_predicates.push(c),
                        }
                    }
                    rest = body;
                }
                other => {
                    let carriers_start = self.index_carriers.borrow().len();
                    self.index_carriers.borrow_mut().extend(binders.iter().filter_map(|(name, kind, _)| {
                        match kind {
                            Some(ParamKind::Val(carrier)) => Some((name.clone(), carrier.clone())),
                            Some(ParamKind::Prop) => Some((name.clone(), Ty::Sort(Sort::Prop))),
                            _ => None,
                        }
                    }));
                    self.index_header_context.borrow_mut().push(ctx.clone());
                    let t: Result<Ty> = (|| {
                        let t = self.conv_type(other, &params)?;
                        // A precondition can precede the telescope it names.
                        // Infer generic measure arguments from those checked
                        // domains, rather than exporting an unsolved private
                        // instantiation or guessing its type from the name.
                        let mut syntax = other;
                        let mut carrier = &t;
                        loop {
                            let (domain, result) = match carrier {
                                Ty::Fun(domain, _, _, result) | Ty::Pi { dom: domain, cod: result, .. } => (domain, result),
                                _ => break,
                            };
                            syntax = match syntax {
                                ast::Type::DepFun(name, _, _, _, result, _) => {
                                    self.index_carriers.borrow_mut().push((name.text.clone(), (**domain).clone()));
                                    result
                                }
                                ast::Type::Fun { res, .. } => res,
                                _ => break,
                            };
                            carrier = result;
                        }
                        for predicate in pending_predicates {
                            if let ConvOblig::Prop(obligation) = self.conv_oblig(predicate, &params)? {
                                obligs.push(obligation);
                            }
                        }
                        Ok(t)
                    })();
                    self.index_header_context.borrow_mut().pop();
                    self.index_carriers.borrow_mut().truncate(carriers_start);
                    let t = t?;
                    // Refinement obligations can name quantified values or
                    // the signature's dependent arguments (`using (Sorted xs,
                    // Sorted ys)` over `(xs :: T) -> (ys :: T) -> ...`):
                    // the caller discharges one per named argument (the
                    // `Dec` evidence path). Dependent argument names normalize to
                    // positional binders (`$a{i}` names the i-th
                    // argument, counting every arrow, since the call
                    // site's arguments are positional), because a
                    // non-dependent chain collapses to `Fun` and takes
                    // its names with it. An unnamed arrow holds an empty
                    // name, which no predicate can mention.
                    let mut arg_binders: Vec<String> = Vec::new();
                    let mut cur_ast = rest;
                    loop {
                        match cur_ast {
                            ast::Type::DepFun(name, _, _, _, res, _) => {
                                arg_binders.push(name.text.clone());
                                cur_ast = res;
                            }
                            ast::Type::Fun { res, .. } => {
                                arg_binders.push(String::new());
                                cur_ast = res;
                            }
                            _ => break,
                        }
                    }
                    // Names used in tail position are row variables; names
                    // used in region position are region variables; the rest
                    // are classified by how they are used (an un-annotated
                    // binder used only as an index value is an implicit
                    // value argument, decision D29).
                    let mut row_params: Vec<String> = Vec::new();
                    let mut region_params: Vec<String> = Vec::new();
                    collect_row_names(&t, &mut row_params, &mut region_params);
                    for (_, c) in &ctx {
                        for a in &c.args {
                            if let Ty::Region(r) = a {
                                if !region_params.iter().any(|p| p.as_str() == r.as_str()) {
                                    region_params.push(r.clone().to_string());
                                }
                            }
                        }
                    }
                    // Region-classified parameters are runtime identities
                    // in argument position too (decision D21).
                    let t = crate::types::regionize(&t, &region_params);
                    let ctx = ctx
                        .into_iter()
                        .map(|(n, c)| (n, crate::types::regionize_class(&c, &region_params)))
                        .collect();
                    let constraints = self.kind_constraints.borrow().clone();
                    let mut type_params: Vec<String> = Vec::new();
                    let mut con_params = Vec::new();
                    let mut val_params: Vec<(String, Ty)> = Vec::new();
                    let mut prop_params: Vec<String> = Vec::new();
                    for (name, hint, span) in binders {
                        if row_params.contains(&name) || explicit_rows.contains(&name) {
                            // Inference cannot discard an explicit different
                            // kind or merge a row with a type, value or region.
                            if hint.is_some() || region_params.contains(&name)
                                || constraints.iter().any(|(used, _)| used == &name)
                            {
                                return Err(Error::at(span, format!(
                                    "effect row parameter `{name}` has an incompatible kind or non-row use"
                                )));
                            }
                            if !row_params.contains(&name) { row_params.push(name); }
                            continue;
                        }
                        if region_params.contains(&name) {
                            continue;
                        }
                        let kind =
                            self.resolve_binder_kind(&name, hint.as_ref(), &constraints, span)?;
                        match kind {
                            ParamKind::Type => type_params.push(name),
                            ParamKind::Con(n) => {
                                con_params.push((name.clone(), n));
                                type_params.push(name);
                            }
                            ParamKind::Val(vty) => val_params.push((name, vty)),
                            ParamKind::Prop => prop_params.push(name),
                        }
                    }
                    // Quantified values keep their lexical identities in
                    // obligations. Classify their checked kinds before
                    // deciding whether a free name denotes an argument.
                    for o in &obligs {
                        if let PropOblig::Pred(p) = o {
                            let mut fv = Vec::new();
                            p.free_vars(&mut fv);
                            for v in fv {
                                if v.is_empty() || (!arg_binders.contains(&v) && !val_params.iter().any(|(name, _)| name == &v)) {
                                    let named: Vec<&str> =
                                        arg_binders.iter().filter(|b| !b.is_empty()).map(|b| b.as_str()).collect();
                                    return Err(Error::plain(format!(
                                        "a signature's refinement obligation names its arguments, but `{v}` is not one of the signature's dependent binders ({})",
                                        named.join(", ")
                                    )));
                                }
                            }
                        }
                    }
                    for o in &mut obligs {
                        if let PropOblig::Pred(p) = o {
                            for (i, b) in arg_binders.iter().enumerate() {
                                if b.is_empty() {
                                    continue;
                                }
                                *p = p.subst(b, &ITerm::Var(format!("$a{i}").into()));
                            }
                        }
                    }
                    let scheme = Scheme {
                        evidence_params: Vec::new(),
                        con_params,
                        params: type_params,
                        idx_params: Vec::new(),
                        val_params,
                        prop_params,
                        row_params,
                        region_params,
                        ctx,
                        obligs,
                        ty: Self::positional_results(&t, 0),
                    };
                    self.check_length_scheme(&scheme, ty.span())?;
                    return Ok(scheme);
                }
            }
        }
    }

    /// Validate primitive observations against the converted signature's actual
    /// telescope. Kind-inference notes deliberately do not establish value
    /// carrier equality and must not be used as a typing environment here.
    fn check_length_scheme(&self, scheme: &Scheme, span: ast::Span) -> Result<()> {
        let mut env: HashMap<String, Ty> = scheme.val_params.iter().cloned().collect();
        for p in &scheme.idx_params {
            if let IxParam::Val(n, t) = p { env.insert(n.clone().to_string(), t.clone()); }
        }
        for (_, t) in &scheme.val_params { self.check_length_type(t, &env, 0, span)?; }
        self.check_length_type(&scheme.ty, &env, 0, span)?;
        // `using` predicates were rewritten to these positional names.
        let mut cur = &scheme.ty;
        let mut i = 0;
        loop {
            match cur {
                Ty::Fun(d, _, _, c) => { env.insert(format!("$a{i}"), (**d).clone()); cur = c; }
                Ty::Pi { var, dom, cod, .. } => {
                    env.insert(var.clone().to_string(), (**dom).clone());
                    env.insert(format!("$a{i}"), (**dom).clone()); cur = cod;
                }
                _ => break,
            }
            i += 1;
        }
        for (_, c) in &scheme.ctx {
            for t in &c.args { self.check_length_type(t, &env, 0, span)?; }
        }
        for p in &scheme.obligs {
            let p = match p { PropOblig::Pred(p) | PropOblig::Preserves { pred: p, .. } => p };
            self.check_length_prop(p, &env, span)?;
        }
        Ok(())
    }

    fn check_length_type(&self, t: &Ty, env: &HashMap<String, Ty>, pos: usize, span: ast::Span) -> Result<()> {
        match t {
            Ty::Fun(d, _, _, c) | Ty::Pi { dom: d, cod: c, .. } => {
                self.check_length_type(d, env, 0, span)?;
                let mut inner = env.clone();
                inner.insert(format!("$a{pos}"), (**d).clone());
                if let Ty::Pi { var, .. } = t { inner.insert(var.clone().to_string(), (**d).clone()); }
                self.check_length_type(c, &inner, pos + 1, span)?;
            }
            Ty::Forall(scheme) => {
                self.check_length_type(&scheme.ty, env, pos, span)?;
            }
            Ty::Sigma { var, dom, cod } => {
                self.check_length_type(dom, env, 0, span)?;
                let mut inner = env.clone(); inner.insert(var.clone().to_string(), (**dom).clone());
                self.check_length_type(cod, &inner, 0, span)?;
            }
            Ty::Adt(name, args) => {
                if let Some(view) = self.refine_view(t) {
                    let mut inner = env.clone();
                    if let Some(base) = &view.base {
                        self.check_length_type(base, env, 0, span)?;
                        inner.insert(view.binder.clone(), base.clone());
                    }
                    self.check_length_prop(&view.pred, &inner, span)?;
                    // The predicate's binder is scoped by this refinement,
                    // not by an unrelated sibling type argument.
                    for a in args { if !matches!(a, Ty::Prp(_)) { self.check_length_type(a, env, 0, span)?; } }
                } else {
                    let info = self.known_type(name);
                    for (i, a) in args.iter().enumerate() {
                        if let (Ty::Idx(it), Some(ParamKind::Val(carrier))) = (a, info.and_then(|x| x.param_kinds.get(i))) {
                            self.check_length_index(it, Some(carrier), env, span)?;
                        } else { self.check_length_type(a, env, 0, span)?; }
                    }
                }
            }
            Ty::Idx(it) => self.check_length_index(it, None, env, span)?,
            Ty::Prp(p) => self.check_length_prop(p, env, span)?,
            Ty::Tuple(args) => for a in args { self.check_length_type(a, env, 0, span)?; },
            Ty::App(f, args) => {
                self.check_length_type(f, env, 0, span)?;
                for a in args { self.check_length_type(a, env, 0, span)?; }
            }
            Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. }
            | Ty::Ref(_, inner) | Ty::Mut(_, inner) | Ty::Ptr { pointee: inner, .. } => self.check_length_type(inner, env, 0, span)?,
            Ty::Handler { input, output, .. } => {
                self.check_length_type(input, env, 0, span)?;
                self.check_length_type(output, env, 0, span)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn check_length_prop(&self, p: &PTerm, env: &HashMap<String, Ty>, span: ast::Span) -> Result<()> {
        match p {
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                self.check_length_index(a, None, env, span)?;
                self.check_length_index(b, None, env, span)?;
            }
            PTerm::Pred(_, a) => self.check_length_index(a, None, env, span)?,
            PTerm::Not(p) => self.check_length_prop(p, env, span)?,
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                self.check_length_prop(a, env, span)?; self.check_length_prop(b, env, span)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn length_in_index(it: &ITerm) -> bool {
        match it {
            ITerm::Fn(n) => n == BYTE_LENGTH_INDEX,
            ITerm::Inst(call) => call.terms().any(Self::length_in_index),
            ITerm::App(f, xs) => Self::length_in_index(f) || xs.iter().any(Self::length_in_index),
            ITerm::Bin(_, a, b) => Self::length_in_index(a) || Self::length_in_index(b),
            ITerm::Lam(_, body) => Self::length_in_index(body),
            _ => false,
        }
    }

    fn check_length_index(&self, it: &ITerm, expected: Option<&Ty>, env: &HashMap<String, Ty>, span: ast::Span) -> Result<()> {
        if !Self::length_in_index(it) { return Ok(()); }
        // This validation must not solve or specialize the enclosing checker,
        // and runs with it only borrowed, so it works on a private copy.
        let mut uni = self.uni.clone();
        let actual = self.length_index_type(it, expected, env, &mut uni, span)?;
        if let Some(want) = expected {
            self.length_index_unify(&actual, want, &mut uni, span)?;
        }
        Ok(())
    }

    fn length_index_unify(&self, actual: &Ty, want: &Ty, uni: &mut Unifier, span: ast::Span) -> Result<()> {
        // A checked refinement may weaken to its carrier. The requested
        // type is never weakened: that would discharge an unproved contract.
        let actual = uni.zonk(actual);
        let actual = if self.refine_view(want).is_none() { self.refine_base(&actual).unwrap_or(actual) } else { actual };
        uni.unify(&actual, want).map_err(|e| Error::at(span, format!("byte-length index operand/result type: {e}")))
    }

    fn length_index_type(&self, it: &ITerm, expected: Option<&Ty>, env: &HashMap<String, Ty>, uni: &mut Unifier, span: ast::Span) -> Result<Ty> {
        match it {
            ITerm::Var(n) => env.get(n.as_str()).cloned().ok_or_else(|| Error::at(span, format!("byte-length index operand `{n}` has no checked carrier in scope"))),
            ITerm::Num(n) => {
                let t = expected.map(|t| uni.zonk(t)).filter(|t| matches!(t, Ty::Nat | Ty::Int | Ty::Prim(_))).unwrap_or(Ty::Nat);
                if t == Ty::Nat && n.starts_with("-") { return Err(Error::at(span, "negative index literal cannot inhabit Nat")); }
                Ok(t)
            }
            ITerm::Fn(n) if n == BYTE_LENGTH_INDEX => Ok(Ty::fun_mult(Ty::Bytes, Mult::Many, EffRow::pure(), Ty::Nat)),
            ITerm::Fn(n) => {
                let value = self.qualified.values().find(|v| fn_id(&v.module, &v.name).0 == *n)
                    .ok_or_else(|| Error::at(span, format!("byte-length operand names unchecked index function {n}")))?;
                let s = &value.scheme;
                let mut types = HashMap::new(); let mut rows = HashMap::new();
                for p in &s.params { types.insert(p.clone(), uni.fresh()); }
                for p in &s.idx_params {
                    match p { IxParam::Val(n, _) => { types.insert(n.clone(), uni.fresh_idx()); }, IxParam::Prop(n) => { types.insert(n.clone(), uni.fresh_prop()); } }
                }
                for (n, _) in &s.val_params { types.insert(n.clone(), uni.fresh_idx()); }
                for n in &s.prop_params { types.insert(n.clone(), uni.fresh_prop()); }
                for n in &s.row_params { rows.insert(n.clone(), uni.fresh_row().tail); }
                Ok(Self::subst_full(&s.ty, &types, &rows, &HashMap::new()))
            }
            ITerm::App(f, args) => {
                let mut ty = self.length_index_type(f, None, env, uni, span)?;
                for a in args {
                    let (dom, eff, cod, binder) = match uni.zonk(&ty) {
                        Ty::Fun(d, _, e, c) => (*d, e, *c, None),
                        Ty::Pi { var, dom, eff, cod, .. } => (*dom, eff, *cod, Some(var)),
                        other => return Err(Error::at(span, crate::diagnostic!("byte-length index applies non-function carrier {}; check its arity", other))),
                    };
                    if !eff.is_pure() { return Err(Error::at(span, "byte-length index operand application must be pure")); }
                    let actual = self.length_index_type(a, Some(&dom), env, uni, span)?;
                    self.length_index_unify(&actual, &dom, uni, span)?;
                    ty = match binder { Some(n) => cod.subst_idx(&n, a), None => cod };
                }
                Ok(uni.zonk(&ty))
            }
            ITerm::Bin(_, a, b) => {
                let carrier = self.length_index_type(a, expected, env, uni, span)?;
                if !matches!(uni.zonk(&carrier), Ty::Nat | Ty::Int) { return Err(Error::at(span, "byte-length index arithmetic requires Nat or Int")); }
                let right = self.length_index_type(b, Some(&carrier), env, uni, span)?;
                self.length_index_unify(&right, &carrier, uni, span)?; Ok(uni.zonk(&carrier))
            }
            ITerm::Lam(n, body) => {
                let Some(Ty::Fun(dom, mult, eff, cod)) = expected.map(|t| uni.zonk(t)) else {
                    return Err(Error::at(span, "byte-length index lambda needs a checked function carrier"));
                };
                let mut inner = env.clone(); inner.insert(n.clone().to_string(), (*dom).clone());
                let actual = self.length_index_type(body, Some(&cod), &inner, uni, span)?;
                self.length_index_unify(&actual, &cod, uni, span)?;
                Ok(Ty::fun_mult(*dom, mult, eff, *cod))
            }
            ITerm::Inst(call) => {
                let IndexHead::Function(head) = &call.head else {
                    return Err(Error::at(span, "dictionary evidence is not a numeric index operand"));
                };
                let Some((module, name)) = head.rsplit_once('.') else {
                    return Err(Error::at(span, "index function has no checked owner"));
                };
                let value = self.qualified.get(&(module.into(), name.into()))
                    .ok_or_else(|| Error::at(span, "index function is no longer available"))?;
                if value.scheme.params.len() != call.types.len() {
                    return Err(Error::at(span, "index function has an incomplete type instantiation"));
                }
                self.instantiated_index_type(&value.scheme, &call, span)
            }
            ITerm::LocalValue(local) => self.local_tys.get(local).map(|ty| uni.zonk(ty))
                .ok_or_else(|| Error::at(span, "captured index value has no checked carrier")),
            ITerm::Evidence(_) => Err(Error::at(span, "dictionary evidence is not a numeric index operand")),
            ITerm::MVar(_) => Err(Error::at(span, "byte-length index operand has an unsolved carrier")),
        }
    }

    /// A result refinement that names earlier arguments (`bytes-new ::
    /// (n :: Nat) -> refined (bytes-len out == n) Bytes`, `nat-rem ::
    /// Nat -> (d :: NonZero Nat) -> refined (r < d) Nat`) states a fact
    /// about each call's result, not a dependent type: the container is
    /// the same for every argument. Such an arrow stays a plain function
    /// arrow, and the argument's name in the predicate becomes the
    /// positional `$a{i}` (as a `using` obligation's does), which a full
    /// application substitutes with the argument's index reading (and
    /// drops to the container where it has none). An arrow whose name
    /// reaches anywhere else (a later parameter, the container) stays a
    /// dependent `Pi`.
    fn positional_results(t: &Ty, i: usize) -> Ty {
        match t {
            Ty::Fun(d, m, e, c) => Ty::Fun(d.clone(), *m, e.clone(), Box::new(Self::positional_results(c, i + 1))),
            Ty::Pi { var, dom, mult, eff, cod } => {
                let cod = Self::positional_results(cod, i + 1);
                let mut skeleton = &cod;
                let mut elsewhere = false;
                while let Ty::Fun(d, _, _, c) | Ty::Pi { dom: d, cod: c, .. } = skeleton {
                    elsewhere |= d.mentions_idx(var);
                    skeleton = c;
                }
                let only_in_pred = match skeleton {
                    Ty::Adt(n, args) if n == "refined" => match args.as_slice() {
                        [_, base] => !base.mentions_idx(var),
                        _ => false,
                    },
                    _ => false,
                };
                if !elsewhere && only_in_pred {
                    Ty::Fun(dom.clone(), *mult, eff.clone(), Box::new(cod.subst_idx(var, &ITerm::Var(format!("$a{i}").into()))))
                } else {
                    Ty::Pi { var: var.clone(), dom: dom.clone(), mult: *mult, eff: eff.clone(), cod: Box::new(cod) }
                }
            }
            other => other.clone(),
        }
    }

    /// The positional argument names a result refinement mentions.
    fn positional_mentions(t: &Ty) -> Vec<usize> {
        let Ty::Adt(n, args) = t else { return Vec::new() };
        let [Ty::Prp(p), _] = args.as_slice() else { return Vec::new() };
        if n != "refined" {
            return Vec::new();
        }
        let mut fv = Vec::new();
        p.free_vars(&mut fv);
        fv.iter()
            .filter(|v| crate::types::is_positional_arg(v))
            .filter_map(|v| v[2..].parse::<usize>().ok())
            .collect()
    }

    /// Whether an arrow chain's final result is a refinement (a function
    /// whose result weakens covariantly, see `unify_arg`).
    pub(crate) fn final_result_refined(&self, t: &Ty) -> bool {
        match self.uni.zonk(t) {
            Ty::Fun(_, _, _, c) | Ty::Pi { cod: c, .. } => self.final_result_refined(&c),
            other => self.refine_view(&other).is_some(),
        }
    }

    /// Whether an arrow chain's final result is a refinement naming a
    /// positional argument.
    pub(crate) fn has_positional_result(t: &Ty) -> bool {
        match t {
            Ty::Fun(_, _, _, c) | Ty::Pi { cod: c, .. } => Self::has_positional_result(c),
            other => !Self::positional_mentions(other).is_empty(),
        }
    }

    /// A function's type where its application has no argument readings
    /// (a call elaborated in continuation-passing style): a result
    /// refinement's conjuncts naming an argument have no argument to
    /// name, so they drop (a weakening, always sound); the rest stay.
    pub(crate) fn strip_positional_results(t: &Ty) -> Ty {
        match t {
            Ty::Fun(d, m, e, c) => Ty::Fun(d.clone(), *m, e.clone(), Box::new(Self::strip_positional_results(c))),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var: var.clone(),
                dom: dom.clone(),
                mult: *mult,
                eff: eff.clone(),
                cod: Box::new(Self::strip_positional_results(cod)),
            },
            other => Self::resolve_positional_result(other, &[]),
        }
    }

    /// The result of a direct application to `args` (each argument's
    /// index reading, `None` where it has none or is not an admissible
    /// fact term): a result refinement's conjunct naming argument `i`
    /// states its fact over that argument's reading, and drops when an
    /// argument it names has no reading or was not supplied (a partial
    /// application); with every conjunct dropped the result is the
    /// container.
    pub(crate) fn resolve_positional_result(t: &Ty, args: &[Option<ITerm>]) -> Ty {
        if matches!(t, Ty::Fun(..) | Ty::Pi { .. }) {
            return Self::strip_positional_results(t);
        }
        if Self::positional_mentions(t).is_empty() {
            return t.clone();
        }
        let Ty::Adt(n, targs) = t else { return t.clone() };
        let [Ty::Prp(p), base] = targs.as_slice() else { return t.clone() };
        // The refined binder moves to `$it` first, so an argument that
        // mentions a local of the binder's name is not captured.
        let pred = p.subst(&crate::types::pred_binder(p), &ITerm::Var("$it".to_string().into()));
        let mut kept: Option<PTerm> = None;
        'conj: for c in pred_conjuncts(&pred) {
            let mut fv = Vec::new();
            c.free_vars(&mut fv);
            if !fv.iter().any(|v| v == "$it") {
                continue;
            }
            let mut c = c.clone();
            for v in fv.iter().filter(|v| crate::types::is_positional_arg(v)) {
                match v[2..].parse::<usize>().ok().and_then(|i| args.get(i)) {
                    Some(Some(it)) => c = c.subst(v, it),
                    _ => continue 'conj,
                }
            }
            kept = Some(match kept {
                None => c,
                Some(k) => PTerm::And(Box::new(k), Box::new(c)),
            });
        }
        match kept {
            Some(p) => Ty::Adt(n.clone(), vec![Ty::Prp(Box::new(p)), base.clone()]),
            None => base.clone(),
        }
    }

    fn subst_ty(t: &Ty, map: &HashMap<String, Ty>) -> Ty {
        Checker::subst_full(t, map, &HashMap::new(), &HashMap::new())
    }

    /// Substitute type parameters (by type), row variables (by tail) and
    /// region names (by fresh name). Index-level variables substitute from
    /// the same map: an index parameter's entry is a `Ty::Idx` value.
    fn subst_full(t: &Ty, types: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>) -> Ty {
        Self::subst_namespaces(t, types, rows, regions, &HashMap::new())
    }

    fn subst_index_full(term: &ITerm, types: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>) -> ITerm {
        Self::subst_index_namespaces(term, types, rows, regions, &HashMap::new())
    }

    fn subst_prop_full(prop: &PTerm, types: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>) -> PTerm {
        Self::subst_prop_namespaces(prop, types, rows, regions, &HashMap::new())
    }

    fn subst_class_namespaces(class: &ClassApp, types: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>) -> ClassApp {
        ClassApp { class: class.class.clone(), args: class.args.iter().map(|ty| Self::subst_namespaces(ty, types, rows, regions, evidence)).collect() }
    }

    fn subst_namespaces(
        t: &Ty,
        types: &HashMap<String, Ty>,
        rows: &HashMap<String, RowTail>,
        regions: &HashMap<String, String>,
        evidence: &HashMap<IndexEvidence, ITerm>,
    ) -> Ty {
        match t {
            Ty::Param(p) => types.get(p.as_str()).cloned().unwrap_or_else(|| t.clone()),
            Ty::Idx(it) => Ty::Idx(Box::new(Checker::subst_index_namespaces(it, types, rows, regions, evidence))),
            Ty::Prp(p) => Ty::Prp(Box::new(Checker::subst_prop_namespaces(p, types, rows, regions, evidence))),
            Ty::Pi { var, dom, mult, eff, cod } => {
                let (var, cod) = Self::subst_bound_type(var, cod, types, rows, regions, evidence);
                Ty::Pi {
                    var: var.into(),
                    dom: Box::new(Checker::subst_namespaces(dom, types, rows, regions, evidence)),
                    mult: *mult,
                    eff: subst_row(eff, rows, regions),
                    cod: Box::new(cod),
                }
            }
            Ty::Sigma { var, dom, cod } => {
                let (var, cod) = Self::subst_bound_type(var, cod, types, rows, regions, evidence);
                Ty::Sigma {
                    var: var.into(),
                    dom: Box::new(Checker::subst_namespaces(dom, types, rows, regions, evidence)),
                    cod: Box::new(cod),
                }
            }
            Ty::Forall(scheme) => Self::subst_under_forall(scheme, types, rows, regions, evidence),
            Ty::Adt(n, args) => Ty::Adt(
                n.clone(),
                args.iter()
                    .map(|a| Checker::subst_namespaces(a, types, rows, regions, evidence))
                    .collect(),
            ),
            Ty::Tuple(args) => Ty::Tuple(
                args.iter()
                    .map(|a| Checker::subst_namespaces(a, types, rows, regions, evidence))
                    .collect(),
            ),
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                Checker::subst_namespaces(a, types, rows, regions, evidence),
                *m,
                subst_row(e, rows, regions),
                Checker::subst_namespaces(b, types, rows, regions, evidence),
            ),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: subst_row(eff, rows, regions),
                inner: Box::new(Checker::subst_namespaces(inner, types, rows, regions, evidence)),
            },
            Ty::Later(inner) => {
                Ty::Later(Box::new(Checker::subst_namespaces(inner, types, rows, regions, evidence)))
            }
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: subst_row(eff, rows, regions),
                region: regions.get(region.as_str()).cloned().unwrap_or_else(|| region.to_string()).into(),
                inner: Box::new(Checker::subst_namespaces(inner, types, rows, regions, evidence)),
            },
            Ty::Ref(region, inner) => Ty::Ref(
                regions.get(region.as_str()).cloned().unwrap_or_else(|| region.to_string()).into(),
                Box::new(Checker::subst_namespaces(inner, types, rows, regions, evidence)),
            ),
            Ty::Mut(region, inner) => Ty::Mut(
                regions.get(region.as_str()).cloned().unwrap_or_else(|| region.to_string()).into(),
                Box::new(Checker::subst_namespaces(inner, types, rows, regions, evidence)),
            ),
            Ty::Region(r) => Ty::Region(regions.get(r.as_str()).cloned().unwrap_or_else(|| r.to_string()).into()),
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: EffLabel {
                    head: label.head.clone(),
                    arg: label
                        .arg
                        .as_ref()
                        .map(|a| regions.get(a.as_str()).cloned().unwrap_or_else(|| a.to_string()).into()),
                },
                input: Box::new(Checker::subst_namespaces(input, types, rows, regions, evidence)),
                output: Box::new(Checker::subst_namespaces(output, types, rows, regions, evidence)),
                eff: subst_row(eff, rows, regions),
                resumption: *resumption,
            },
            // Var-headed application substitutes with beta-reduction: a
            // head replaced by a constructor application collects the
            // arguments into it (`f a` with `f := Result e` is `Result e
            // a`), and a head replaced by a type former saturates to its
            // image (`f a` with `f := (->) r` is `r -> a`).
            Ty::App(h, args) => {
                let h = Checker::subst_namespaces(h, types, rows, regions, evidence);
                let args: Vec<Ty> = args
                    .iter()
                    .map(|a| Checker::subst_namespaces(a, types, rows, regions, evidence))
                    .collect();
                Ty::apply_spine(h, args)
            }
            other => other.clone(),
        }
    }

    /// Substitute under a nested quantifier: its binders shadow the
    /// substitution, and a binder that a substituted type mentions free is
    /// renamed first, so nothing is captured.
    fn subst_under_forall(
        scheme: &Scheme, types: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>,
        regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>,
    ) -> Ty {
        let (mut types, mut rows, mut regions) = (types.clone(), rows.clone(), regions.clone());
        for p in &scheme.params { types.remove(p); }
        for p in &scheme.row_params { rows.remove(p); }
        for p in &scheme.region_params { regions.remove(p); }
        let captured = |name: &str| types.values().any(|t| t.mentions_param(name))
            || rows.values().any(|t| matches!(t, RowTail::Rigid(r) if r.as_str() == name))
            || regions.values().any(|r| r == name);
        let mut renaming = crate::types::VarRenaming::default();
        let mut inner = scheme.clone();
        for (list, kind) in [(&mut inner.params, 0), (&mut inner.row_params, 1), (&mut inner.region_params, 2)] {
            for p in list.iter_mut() {
                if !captured(p) { continue; }
                let mut fresh = format!("{p}$");
                while captured(&fresh) || scheme.ty.mentions_param(&fresh) { fresh.push('$'); }
                match kind {
                    0 => { renaming.types.insert(p.clone(), fresh.clone().into()); }
                    1 => { renaming.rows.insert(p.clone(), fresh.clone().into()); }
                    _ => { renaming.regions.insert(p.clone(), fresh.clone().into()); }
                }
                *p = fresh;
            }
        }
        for (name, _) in inner.con_params.iter_mut() {
            if let Some(fresh) = renaming.types.get(name.as_str()) { *name = fresh.to_string(); }
        }
        let body = if renaming.types.is_empty() && renaming.rows.is_empty() && renaming.regions.is_empty() {
            scheme.ty.clone()
        } else {
            scheme.ty.rename_vars(&renaming)
        };
        inner.ty = Checker::subst_namespaces(&body, &types, &rows, &regions, evidence);
        Ty::Forall(Box::new(inner))
    }

    fn subst_bound_type(
        name: &str, cod: &Ty, types: &HashMap<String, Ty>,
        rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>,
    ) -> (String, Ty) {
        let mut inner = types.clone();
        inner.remove(name);
        let mut binder = name.to_owned();
        let mut cod = cod.clone();
        if inner.values().any(|ty| ty.mentions_idx(name)) || evidence.values().any(|value| value.mentions(name)) {
            binder.push_str("$sub");
            while cod.mentions_idx(&binder) || inner.values().any(|ty| ty.mentions_idx(&binder)) || evidence.values().any(|value| value.mentions(&binder)) || inner.contains_key(&binder) {
                binder.push('$');
            }
            cod = cod.subst_idx(name, &ITerm::Var(binder.clone().into()));
        }
        (binder, Self::subst_namespaces(&cod, &inner, rows, regions, evidence))
    }

    /// Substitute index-level variables in an index term from the map.
    fn subst_it(t: &ITerm, map: &HashMap<String, Ty>) -> ITerm {
        Self::subst_index_full(t, map, &HashMap::new(), &HashMap::new())
    }

    fn subst_index_namespaces(t: &ITerm, map: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>) -> ITerm {
        match t {
            ITerm::Inst(call) => ITerm::Inst(Box::new(call.map(
                &mut |ty| Self::subst_namespaces(ty, map, rows, regions, evidence),
                &mut |term| Self::subst_index_namespaces(term, map, rows, regions, evidence),
            ))),
            ITerm::Lam(name, body) => {
                let mut inner = map.clone();
                inner.remove(name.as_str());
                let mut binder = name.to_string();
                let mut body = (**body).clone();
                if inner.values().any(|ty| ty.mentions_idx(name)) || evidence.values().any(|value| value.mentions(name)) {
                    binder.push_str("$sub");
                    while body.mentions(&binder) || inner.values().any(|ty| ty.mentions_idx(&binder)) || evidence.values().any(|value| value.mentions(&binder)) || inner.contains_key(binder.as_str()) {
                        binder.push('$');
                    }
                    body = body.subst(name, &ITerm::Var(binder.as_str().into()));
                }
                ITerm::Lam(binder.as_str().into(), Box::new(Self::subst_index_namespaces(&body, &inner, rows, regions, evidence)))
            }
            ITerm::Evidence(binding) => evidence.get(binding).cloned().unwrap_or_else(|| t.clone()),
            ITerm::Var(n) => match map.get(n.as_str()) {
                Some(Ty::Idx(it)) => (**it).clone(),
                _ => t.clone(),
            },
            ITerm::App(f, args) => ITerm::App(
                Box::new(Checker::subst_index_namespaces(f, map, rows, regions, evidence)),
                args.iter().map(|a| Checker::subst_index_namespaces(a, map, rows, regions, evidence)).collect(),
            ),
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(Checker::subst_index_namespaces(a, map, rows, regions, evidence)),
                Box::new(Checker::subst_index_namespaces(b, map, rows, regions, evidence)),
            ),
            other => other.clone(),
        }
    }

    /// Substitute index-level variables in a proposition from the map.
    fn subst_pt(p: &PTerm, map: &HashMap<String, Ty>) -> PTerm {
        Self::subst_prop_full(p, map, &HashMap::new(), &HashMap::new())
    }

    fn subst_prop_namespaces(p: &PTerm, map: &HashMap<String, Ty>, rows: &HashMap<String, RowTail>, regions: &HashMap<String, String>, evidence: &HashMap<IndexEvidence, ITerm>) -> PTerm {
        match p {
            PTerm::Var(n) => match map.get(n) {
                Some(Ty::Prp(q)) => (**q).clone(),
                Some(Ty::Idx(_)) => p.clone(),
                _ => p.clone(),
            },
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(Checker::subst_index_namespaces(a, map, rows, regions, evidence)),
                Box::new(Checker::subst_index_namespaces(b, map, rows, regions, evidence)),
            ),
            PTerm::Eq(a, b) => PTerm::Eq(
                Box::new(Checker::subst_index_namespaces(a, map, rows, regions, evidence)),
                Box::new(Checker::subst_index_namespaces(b, map, rows, regions, evidence)),
            ),
            PTerm::Not(q) => PTerm::Not(Box::new(Checker::subst_prop_namespaces(q, map, rows, regions, evidence))),
            PTerm::And(a, b) => PTerm::And(
                Box::new(Checker::subst_prop_namespaces(a, map, rows, regions, evidence)),
                Box::new(Checker::subst_prop_namespaces(b, map, rows, regions, evidence)),
            ),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(Checker::subst_prop_namespaces(a, map, rows, regions, evidence)),
                Box::new(Checker::subst_prop_namespaces(b, map, rows, regions, evidence)),
            ),
            PTerm::Pred(pf, a) => {
                PTerm::Pred(*pf, Box::new(Checker::subst_index_namespaces(a, map, rows, regions, evidence)))
            }
            other => other.clone(),
        }
    }

    fn subst_class(c: &ClassApp, map: &HashMap<String, Ty>) -> ClassApp {
        ClassApp {
            class: c.class.clone(),
            args: c.args.iter().map(|a| Checker::subst_ty(a, map)).collect(),
        }
    }

    /// Instantiate a scheme with fresh unification variables: type
    /// parameters become type metavariables, index and implicit value
    /// parameters become index metavariables, row parameters become row
    /// metavariables, and region names are freshened to new rigid names.
    /// The returned list is the instantiated form of `val_params`: each
    /// entry becomes a runtime argument at the use site (insert-and-solve).
    fn instantiate(&mut self, s: &Scheme) -> (Ty, Vec<InstantiatedConstraint>, Vec<Ty>) {
        let (t, ctx, vals, _) = self.instantiate_at(s);
        (t, ctx, vals)
    }

    /// Instantiate the scheme of the local `l` at a use of its value. The
    /// use of a let-generalized local is a copy of its definition at this
    /// instantiation (`Some`, [`Checker::gen_copy`]); any other local's
    /// use is the local itself.
    fn instantiate_local(&mut self, l: LocalId, s: &Scheme) -> (Ty, Option<ECore>, Vec<PropOblig>) {
        let ((t, _, _, ptys), bindings) = self.instantiate_scoped(s);
        let obligations = Self::instantiated_obligations(s, &bindings);
        let map: HashMap<crate::types::MetaId, Ty> = match self.gen_locals.get(&l) {
            Some(metas) if self.gen_defs.contains_key(&l) => metas.iter().copied().zip(ptys).collect(),
            _ => return (t, None, obligations),
        };
        let (ty, body) = self.gen_copy(l, &map, &t);
        (ty, Some(body), obligations)
    }

    /// A copy of the definition of the let-generalized local `l` for one
    /// use, its quantified metavariables instantiated by `map`. One
    /// compiled body cannot serve uses of different representations (a
    /// raw word and a counted one), so each use gets its own: every
    /// binder of the copy is fresh and typed with its original's type
    /// instantiated, and every `letrec` unit the definition reaches (its
    /// own group, and the groups nested in it) is copied with it under a
    /// fresh unit id. Fresh binders follow the originals' id order,
    /// because a `letrec` unit takes its captured locals sorted by id, in
    /// the order its call sites pass them.
    fn gen_copy(&mut self, l: LocalId, map: &HashMap<crate::types::MetaId, Ty>, ty: &Ty) -> (Ty, ECore) {
        let def = self.gen_defs[&l].clone();
        self.copy_checked_definition(&def, map, &StageTypeBindings::default(), ty)
    }

    /// Copy an elaborated body, retaining its captured locals and remapping
    /// every owned binder, deferred check and nested recursive unit. A
    /// universally checked annotation substitutes rigid namespaces here;
    /// inferred let-polymorphism substitutes metavariables instead.
    fn copy_checked_definition(
        &mut self, def: &ECore, map: &HashMap<crate::types::MetaId, Ty>,
        scope: &StageTypeBindings, ty: &Ty,
    ) -> (Ty, ECore) {
        let rec: HashMap<GlobalId, usize> =
            self.pending_rec.iter().enumerate().map(|(i, (id, _))| (id.clone(), i)).collect();
        let mut parts = CopyParts::default();
        def_parts(&def, &mut parts);
        // Reachable letrec bodies and checked stage-call metadata belong to
        // this copy too. Merely copying their references keeps stale locals.
        let mut units = Vec::new();
        let mut calls = std::collections::BTreeMap::new();
        while let Some(global) = parts.units.pop() {
            if let Some(&i) = rec.get(&global) {
                if !units.contains(&i) {
                    units.push(i);
                    def_parts(&self.pending_rec[i].1, &mut parts);
                }
            }
            if let Some(call) = self.stage_calls.borrow().get(&global.0).cloned() {
                calls.insert(global.0.clone(), call);
            }
        }
        parts.binders.sort_by_key(|binder| binder.0);
        parts.binders.dedup();
        let mut locals = HashMap::with_capacity(parts.binders.len());
        for old in parts.binders {
            let new = self.fresh_local();
            if let Some(class) = self.local_cls.get(&old).copied() { self.local_cls.insert(new, class); }
            if let Some(origin) = self.local_origin.get(&old).copied() { self.local_origin.insert(new, origin); }
            locals.insert(old, new);
        }
        let originals: Vec<_> = self.wants.iter().filter(|want| parts.wants.contains(&want.id)).cloned().collect();
        let mut deferred = CopyDeferred::default();
        let mut witnesses = HashMap::new();
        for want in &originals {
            deferred.wants.insert(want.id, self.want_counter);
            self.want_counter += 1;
            if let Some(ITerm::MVar(variable)) = want.evidence_index.as_ref().map(|index| self.uni.resolve_idx(index)) {
                witnesses.entry(variable).or_insert_with(|| self.uni.fresh_idx_term());
            }
        }
        for (offset, index) in parts.indices.iter().enumerate() {
            deferred.indices.insert(*index, self.idx_pends.len() + offset);
        }
        let mut ids = HashMap::new();
        for &i in &units {
            let id = GlobalId(format!("{}.letrec#{}", self.current_module, self.lam_counter));
            self.lam_counter += 1;
            ids.insert(self.pending_rec[i].0.clone(), id);
        }
        let first_call = self.stage_calls.borrow().len();
        for (offset, original) in calls.keys().enumerate() {
            ids.insert(GlobalId(original.clone()), GlobalId(format!("<checked-stage-call:{}>", first_call + offset)));
        }
        let mut call_rows = HashMap::new();
        let mut call_regions = HashMap::new();
        for (original, call) in &calls {
            let mut rows = HashMap::new();
            for (name, tail) in &call.arguments.rows {
                let row = self.uni.resolve_row(&EffRow { labels: Vec::new(), tail: tail.clone() });
                let row = subst_row(&row, &scope.rows, &scope.regions);
                // A row argument is stored as a tail. Preserve any solved
                // labels by binding a fresh tail to the complete copied row.
                let fresh = self.uni.fresh_row();
                self.uni.unify_rows(&fresh, &row).expect("a fresh row accepts the checked row");
                rows.insert(name.clone(), fresh.tail);
            }
            call_rows.insert(original.clone(), rows);
            call_regions.insert(original.clone(), call.arguments.regions.iter().map(|(name, region)| {
                let region = self.uni.resolve_region(region);
                (name.clone(), scope.regions.get(region.as_str()).cloned().unwrap_or(region.to_string()))
            }).collect::<HashMap<_, _>>());
        }
        let (copy_type, out, copies, wants, indices, prop_calls, stage_calls, local_types) = {
            let uni = &self.uni;
            let evidence: HashMap<_, _> = locals.iter().map(|(old, new)|
                (IndexEvidence::Local(old.0), ITerm::Evidence(IndexEvidence::Local(new.0)))).collect();
            // Rename the body's own bindings before inserting caller type
            // arguments. Their free witnesses and locals belong to the caller
            // even when they coincide with a binding in the original body.
            let local_values = locals.iter().map(|(old, new)| (*old, ITerm::LocalValue(*new))).collect();
            let tys = |ty: &Ty| uni.subst_metas(&Self::subst_namespaces(
                &uni.zonk(ty).subst_index_metas(&witnesses).subst_evidence(&evidence).subst_local_values(&local_values),
                &scope.types, &scope.rows, &scope.regions, &scope.evidence,
            ), map);
            let rename = Remap { locals: &locals, units: Some(&ids), tys: Some(&tys), deferred: Some(&deferred), substitutions: std::cell::OnceCell::new() };
            // Complete the renaming first: types can refer to dictionaries
            // allocated after the value binder whose type retains them.
            let local_types: Vec<_> = locals.iter().filter_map(|(old, new)|
                self.local_tys.get(old).map(|ty| (*new, rename.ty(ty)))).collect();
            let wants: Vec<_> = originals.iter().map(|want| Want {
                id: deferred.wants[&want.id], class: rename.class(&want.class), span: want.span,
                scope: want.scope.iter().map(|(local, class)| (rename.local(*local), rename.class(class))).collect(),
                method: want.method, evidence_index: want.evidence_index.as_ref().map(|index| rename.index(index)),
            }).collect();
            let indices: Vec<_> = parts.indices.iter().map(|index| {
                let pending = &self.idx_pends[*index];
                IdxPend {
                    iterm: rename.index(&pending.iterm), carrier: rename.ty(&pending.carrier), span: pending.span,
                    scope: IndexRuntimeScope {
                        values: pending.scope.values.iter().map(|(name, local)| (name.clone(), rename.local(*local))).collect(),
                        locals: pending.scope.locals.iter().map(|local| rename.local(*local)).collect(),
                        dictionaries: pending.scope.dictionaries.iter().map(|local| rename.local(*local)).collect(),
                    },
                }
            }).collect();
            let prop_calls: Vec<_> = self.prop_calls.iter().filter_map(|call| {
                let want = *deferred.wants.get(&call.want)?;
                let mut call = call.clone();
                call.want = want;
                for (value, ty, _) in &mut call.args {
                    *value = value.as_ref().map(|value| rename.index(value));
                    *ty = rename.ty(ty);
                }
                for (_, predicate, local, _) in &mut call.evidence.items {
                    *predicate = rename.proposition(predicate);
                    let original = *local;
                    *local = rename.local(*local);
                    if let Some(certificates) = self.comparison_facts.get(&original).cloned() {
                        self.comparison_facts.insert(*local, certificates);
                    }
                }
                call.evidence.refined = call.evidence.refined.iter().map(|predicate| rename.proposition(predicate)).collect();
                call.evidence.value_sorts = call.evidence.value_sorts.iter().map(|(local, sort)| (rename.local(*local), *sort)).collect();
                Some(call)
            }).collect();
            let stage_calls: Vec<_> = calls.iter().map(|(original, call)| {
                let arguments = StageTypeBindings {
                    types: call.arguments.types.iter().map(|(name, ty)| (name.clone(), rename.ty(ty))).collect(),
                    evidence: call.arguments.evidence.iter().map(|(name, index)| (name.clone(), rename.index(index))).collect(),
                    rows: call_rows[original].clone(), regions: call_regions[original].clone(),
                };
                (ids[&GlobalId(original.clone())].0.clone(), StageCall { target: rename.unit(&call.target), arguments })
            }).collect();
            let copies: Vec<_> = units.iter().map(|&i|
                (ids[&self.pending_rec[i].0].clone(), remap_ecore(&self.pending_rec[i].1, &rename))).collect();
            (rename.ty(ty), remap_ecore(&def, &rename), copies, wants, indices, prop_calls, stage_calls, local_types)
        };
        self.local_tys.extend(local_types);
        self.wants.extend(wants);
        self.idx_pends.extend(indices);
        self.prop_calls.extend(prop_calls);
        self.stage_calls.borrow_mut().extend(stage_calls);
        self.pending_rec.extend(copies);
        (copy_type, out)
    }
    /// [`Checker::instantiate`], also returning the metavariables the
    /// scheme's type parameters took, in `params` order: the use site's
    /// instantiation, whose classes lowering records.
    fn instantiate_at(&mut self, s: &Scheme) -> (Ty, Vec<InstantiatedConstraint>, Vec<Ty>, Vec<Ty>) {
        self.instantiate_scoped(s).0
    }

    fn instantiated_obligations(s: &Scheme, bindings: &StageTypeBindings) -> Vec<PropOblig> {
        s.obligs.iter().map(|obligation| match obligation {
            PropOblig::Pred(predicate) => PropOblig::Pred(Self::subst_prop_namespaces(
                predicate, &bindings.types, &bindings.rows, &bindings.regions, &bindings.evidence)),
            PropOblig::Preserves { method, pred } => PropOblig::Preserves {
                method: method.clone(), pred: Self::subst_prop_namespaces(
                    pred, &bindings.types, &bindings.rows, &bindings.regions, &bindings.evidence),
            },
        }).collect()
    }

    fn instantiate_scoped(&mut self, s: &Scheme) -> ((Ty, Vec<InstantiatedConstraint>, Vec<Ty>, Vec<Ty>), StageTypeBindings) {
        let mut types = HashMap::new();
        let mut param_tys = Vec::with_capacity(s.params.len());
        for p in &s.params {
            let m = self.uni.fresh();
            param_tys.push(m.clone());
            types.insert(p.clone(), m);
        }
        for p in &s.idx_params {
            match p {
                IxParam::Val(n, _) => {
                    types.insert(n.clone(), self.uni.fresh_idx());
                }
                IxParam::Prop(n) => {
                    types.insert(n.clone(), self.uni.fresh_prop());
                }
            }
        }
        let mut val_args = Vec::new();
        for (n, _) in &s.val_params {
            let m = self.uni.fresh_idx();
            types.insert(n.clone(), m.clone());
            val_args.push(m);
        }
        for n in &s.prop_params {
            types.insert(n.clone(), self.uni.fresh_prop());
        }
        let mut rows = HashMap::new();
        for p in &s.row_params {
            let r = self.uni.fresh_row();
            rows.insert(p.clone(), r.tail.clone());
        }
        let mut regions = HashMap::new();
        for p in &s.region_params {
            let fresh = self.fresh_param(p);
            // An instantiated region is flexible: it takes the identity of
            // whatever rigid region the use site provides.
            self.uni.note_flex_region(&fresh);
            regions.insert(p.clone(), fresh);
        }
        let mut evidence = HashMap::new();
        for (name, _) in &s.ctx { evidence.insert(dictionary_parameter(name), self.uni.fresh_idx_term()); }
        for parameter in &s.evidence_params { evidence.entry(parameter.clone()).or_insert_with(|| self.uni.fresh_idx_term()); }
        let ctx = s
            .ctx
            .iter()
            .map(|(n, c)| InstantiatedConstraint {
                name: n.clone().to_string(), class: Self::subst_class_namespaces(c, &types, &rows, &regions, &evidence),
                witness: evidence[&dictionary_parameter(n)].clone(),
            })
            .collect();
        let result = (Self::subst_namespaces(&s.ty, &types, &rows, &regions, &evidence), ctx, val_args, param_tys);
        (result, StageTypeBindings { types, rows, regions, evidence })
    }

    fn subst_full_class(
        c: &ClassApp,
        types: &HashMap<String, Ty>,
        rows: &HashMap<String, RowTail>,
        regions: &HashMap<String, String>,
    ) -> ClassApp {
        ClassApp {
            class: c.class.clone(),
            args: c
                .args
                .iter()
                .map(|a| Checker::subst_full(a, types, rows, regions))
                .collect(),
        }
    }

    /// Turn a scheme into a definition signature with rigid parameters.
    /// Index and implicit value parameters keep their names: the
    /// definition's equation patterns and body mention them by name.
    fn rigidify(&mut self, s: &Scheme) -> (Ty, Vec<(String, ClassApp, LocalId)>, StageTypeBindings) {
        self.rigidify_with_value_locals(s, &[])
    }

    fn rigidify_with_value_locals(&mut self, s: &Scheme, values: &[(String, LocalId)]) -> (Ty, Vec<(String, ClassApp, LocalId)>, StageTypeBindings) {
        let mut types = HashMap::new();
        for p in &s.params {
            types.insert(p.clone(), Ty::Param(self.fresh_param(p).into()));
        }
        for p in &s.idx_params {
            match p {
                IxParam::Val(n, _) => {
                    types.insert(n.clone(), Ty::Idx(Box::new(ITerm::Var(n.clone().into()))));
                }
                IxParam::Prop(n) => {
                    types.insert(n.clone(), Ty::Prp(Box::new(PTerm::Var(n.clone().to_string()))));
                }
            }
        }
        for (n, _) in &s.val_params {
            let value = values.iter().find(|(name, _)| name == n)
                .map(|(_, local)| ITerm::LocalValue(*local)).unwrap_or_else(|| ITerm::Var(n.clone().into()));
            types.insert(n.clone(), Ty::Idx(Box::new(value)));
        }
        for n in &s.prop_params {
            types.insert(n.clone(), Ty::Prp(Box::new(PTerm::Var(n.clone().to_string()))));
        }
        let mut rows = HashMap::new();
        for p in &s.row_params {
            rows.insert(p.clone(), RowTail::Rigid(self.fresh_param(p).into()));
        }
        let mut regions = HashMap::new();
        for p in &s.region_params {
            regions.insert(p.clone(), self.fresh_param(p));
        }
        let mut dictionary_locals = Vec::new();
        let mut evidence = HashMap::new();
        for parameter in &s.evidence_params { evidence.insert(parameter.clone(), ITerm::Evidence(parameter.clone())); }
        for (name, _) in &s.ctx {
            let local = self.fresh_local();
            evidence.insert(dictionary_parameter(name), IndexDictionaryBinding::from(local).0);
            dictionary_locals.push(local);
        }
        let ctx: Vec<_> = s.ctx.iter().zip(dictionary_locals).map(|((name, class), local)|
            (name.clone(), Self::subst_class_namespaces(class, &types, &rows, &regions, &evidence), local)).collect::<Vec<(String, ClassApp, LocalId)>>();
        let mut scope = Scheme::mono(Ty::Unit);
        scope.params = s.params.iter().filter_map(|p| match types.get(p.as_str()) {
            Some(Ty::Param(name)) => Some(name.to_string()), _ => None,
        }).collect();
        scope.con_params = s.con_params.iter().filter_map(|(p, arity)| match types.get(p.as_str()) {
            Some(Ty::Param(name)) => Some((name.to_string(), *arity)), _ => None,
        }).collect();
        scope.idx_params = s.idx_params.iter().map(|p| match p {
            IxParam::Val(n, dom) => IxParam::Val(n.clone(), Self::subst_namespaces(dom, &types, &rows, &regions, &evidence)),
            IxParam::Prop(n) => IxParam::Prop(n.clone()),
        }).collect();
        scope.idx_params.extend(s.val_params.iter().map(|(n, dom)|
            IxParam::Val(n.clone(), Self::subst_namespaces(dom, &types, &rows, &regions, &evidence))));
        scope.idx_params.extend(s.prop_params.iter().cloned().map(IxParam::Prop));
        scope.row_params = s.row_params.iter().filter_map(|p| match &rows[p] {
            RowTail::Rigid(name) => Some(name.to_string()), _ => None,
        }).collect();
        scope.region_params = s.region_params.iter().map(|p| regions[p].clone()).collect();
        scope.ctx = ctx.iter().map(|(name, class, _)| (name.clone(), class.clone())).collect();
        scope.evidence_params = ctx.iter().map(|(_, _, local)| IndexEvidence::Local(local.0)).collect();
        scope.evidence_params.extend(s.evidence_params.iter().cloned());
        self.code_quantifiers = scope;
        (
            Self::subst_namespaces(&s.ty, &types, &rows, &regions, &evidence),
            ctx,
            StageTypeBindings { types, rows, regions, evidence },
        )
    }

    // ------------------------------------------------------------ wants and solving

    fn new_want(&mut self, class: ClassApp, span: ast::Span, env: &Env) -> usize {
        let id = self.want_counter;
        self.want_counter += 1;
        self.wants.push(Want {
            id,
            class,
            span,
            scope: env.dicts.clone(),
            method: false,
            evidence_index: None,
        });
        id
    }

    fn new_context_want(&mut self, constraint: &InstantiatedConstraint, span: ast::Span, env: &Env) -> usize {
        let id = self.new_want(constraint.class.clone(), span, env);
        self.wants.last_mut().expect("new want").evidence_index = Some(constraint.witness.clone());
        id
    }

    fn solve_wants(&mut self) -> Result<HashMap<usize, DictExpr>> {
        let wants = std::mem::take(&mut self.wants);
        let mut solutions = HashMap::new();
        let mut pending: Vec<usize> = (0..wants.len()).collect();
        // Fixpoint: committing one want's unique instance can determine the
        // types behind the next want, and with them the deferred literals
        // (primitive types specification, section 7).
        loop {
            let mut progress = false;
            let mut next = Vec::new();
            for &i in &pending {
                match self.solve(&wants[i].class, &wants[i].scope, 0, wants[i].span)? {
                    Solved(d) => {
                        if let Some(index) = &wants[i].evidence_index {
                            let actual = d.index_with(&self.uni, &|local| IndexDictionaryBinding::from(*local).0);
                            self.uni.unify_idx(index, &actual).map_err(|error| Error::at(wants[i].span, error))?;
                        }
                        solutions.insert(wants[i].id, d);
                        progress = true;
                    }
                    Ambiguous => next.push(i),
                }
            }
            pending = next;
            if pending.is_empty() {
                // The recorded method applications' refinement
                // obligations discharge now that their wants resolved
                // to instances (the `Dec` evidence path).
                self.discharge_prop_calls(&wants, &solutions)?;
                return Ok(solutions);
            }
            if !progress {
                // Nothing moved. A want blocked only by literals with no
                // determined type takes the mathematical `Int` (decision
                // D7) as its residual type: `Int` keeps working here, and
                // the specification's open decision on literal defaulting
                // revisits this. With no literal involved, several
                // instances genuinely match.
                let lit_metas = self.literal_metas();
                let mut defaulted = false;
                for &i in &pending {
                    let wanted_z = self.uni.zonk_class(&wants[i].class);
                    for a in &wanted_z.args {
                        for m in self.uni.free_metas(a) {
                            if lit_metas.contains(&m) {
                                let ty = match m {
                                    EitherMeta::Type(id) => Ty::Meta(id),
                                    EitherMeta::Row(_) => continue,
                                };
                                self.uni
                                    .unify(&ty, &Ty::Int)
                                    .unwrap_or_else(|_| {
                                        panic!("literal defaulting to Int must succeed")
                                    });
                                defaulted = true;
                            }
                        }
                    }
                    if defaulted {
                        break;
                    }
                }
                if defaulted {
                    continue;
                }
                let w = &wants[pending[0]];
                let wanted_z = self.uni.zonk_class(&w.class);
                return Err(Error::at(
                    w.span,
                    crate::diagnostic!(
                        "ambiguous instances for {}{}",
                        wanted_z,
                        undetermined_hint(&self.uni, &wanted_z)
                    ),
                ));
            }
        }
    }

    fn solve(
        &mut self,
        wanted: &ClassApp,
        scope: &[(LocalId, ClassApp)],
        depth: usize,
        span: ast::Span,
    ) -> Result<SolveOutcome> {
        // The resolver never reads `self.uni` (its checker helpers take the
        // store as an argument), so the store moves in and back out instead
        // of being copied. The resolver's store replaces the checker's on
        // every outcome, as before.
        let uni = std::mem::take(&mut self.uni);
        let mut resolver = DictionaryResolver { checker: self, uni, transfers: Vec::new() };
        let result = resolver.solve(wanted, scope, depth, span);
        let DictionaryResolver { uni, transfers, .. } = resolver;
        self.uni = uni;
        for transfer in transfers {
            let instance = self.instance_list[transfer.instance].clone();
            self.record_transfer(&instance, &transfer.obligs, transfer.container,
                &transfer.wanted, &transfer.subdicts, transfer.span)?;
        }
        result
    }

    /// The instance parameters a target states as proposition variables
    /// (`refined p T`'s `p`): they bind by proposition unification, not
    /// as type metavariables.
    fn prop_param_names(params: &[String], target: &ClassApp) -> Vec<String> {
        fn walk(t: &Ty, out: &mut Vec<String>) {
            match t {
                Ty::Prp(p) => {
                    let mut fv = Vec::new();
                    p.free_vars(&mut fv);
                    out.extend(fv);
                }
                Ty::Adt(_, args) | Ty::Tuple(args) => {
                    for a in args {
                        walk(a, out);
                    }
                }
                Ty::App(h, args) => {
                    walk(h, out);
                    for a in args {
                        walk(a, out);
                    }
                }
                Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
                    walk(a, out);
                    walk(b, out);
                }
                _ => {}
            }
        }
        let mut names = Vec::new();
        for a in &target.args {
            walk(a, &mut names);
        }
        names.retain(|n| params.contains(n));
        names.sort();
        names.dedup();
        names
    }

    /// Refinement-lattice matching of one instance head against the
    /// wanted (refinement types design, "instance heads at
    /// refinements"): refined arguments match when their containers
    /// unify and the wanted's predicate implies the head's (the value
    /// weakens to the head) or the head's predicate is a parameter
    /// binding the wanted's; a plain head serves refined wants (the
    /// transfer rule) but never the reverse (an instance at `Sorted`
    /// must not apply to unsorted lists).
    fn lattice_match(&self, head: &[Ty], want: &[Ty], trial: &mut Unifier) -> bool {
        if head.len() != want.len() {
            return false;
        }
        for (h, w) in head.iter().zip(want) {
            let hr = self.refine_view(h);
            let wr = self.refine_view(w);
            match (&hr, &wr) {
                (Some(hv), Some(wv)) => {
                    if let (Some(a), Some(b)) = (&hv.base, &wv.base) {
                        if trial.unify(a, b).is_err() {
                            return false;
                        }
                    }
                    if trial.unify_prop(&hv.pred, &wv.pred).is_ok() {
                        continue;
                    }
                    if !pred_implies(&wv.pred, &hv.pred) {
                        return false;
                    }
                }
                (Some(_), None) => return false,
                (None, Some(wv)) => {
                    // A plain head at a refined want: the container's
                    // own methods at the point (the transfer rule). The
                    // head's type is the want's container.
                    let wbase = wv.base.as_ref().unwrap_or(w);
                    if trial.unify(h, wbase).is_err() {
                        return false;
                    }
                }
                (None, None) => {
                    if trial.unify(h, w).is_err() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// The head condition a matched target states at the want: the
    /// conjunction of its refined arguments' predicates (`None` is the
    /// trivial condition of a plain head).
    fn head_cond(&self, args: &[Ty], trial: &Unifier) -> Option<PTerm> {
        let mut out: Option<PTerm> = None;
        for a in args {
            if let Some(rv) = self.refine_view(a) {
                let p = trial.resolve_prop(&rv.pred);
                out = Some(match out {
                    None => p,
                    Some(q) => PTerm::And(Box::new(q), Box::new(p)),
                });
            }
        }
        out
    }

    /// Whether condition `a` implies condition `b` (the trivial
    /// condition is implied by everything and implies only itself).
    fn cond_implies(a: &Option<PTerm>, b: &Option<PTerm>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(x), Some(y)) => pred_implies(x, y),
        }
    }

    /// Substitute an instance's refinement obligations through the
    /// match's parameter substitution.
    fn subst_oblig(o: &PropOblig, sub: &HashMap<String, Ty>) -> PropOblig {
        match o {
            PropOblig::Pred(p) => PropOblig::Pred(Checker::subst_pt(p, sub)),
            PropOblig::Preserves { method, pred } => PropOblig::Preserves {
                method: method.clone(),
                pred: Checker::subst_pt(pred, sub),
            },
        }
    }

    /// Discharge the recorded method applications' per-argument
    /// refinement obligations now that every want resolved (refinement
    /// types design, "instance heads at refinements"): statically where
    /// an argument's refinement implies the obligation, through the
    /// caller's `Dec` evidence otherwise. A method call whose call form
    /// did not record its arguments is refused: the obligations of an
    /// instance are the caller's to establish.
    fn discharge_prop_calls(
        &mut self,
        wants: &[Want],
        solutions: &HashMap<usize, DictExpr>,
    ) -> Result<()> {
        let calls = std::mem::take(&mut self.prop_calls);
        let mut seen: HashSet<usize> = HashSet::new();
        // Wants whose method was applied directly (the call's head), the
        // only uses an instance's per-method obligations allow.
        let mut direct: HashSet<usize> = HashSet::new();
        for call in calls {
            seen.insert(call.want);
            let Some(sol) = solutions.get(&call.want) else {
                continue;
            };
            let DictExpr::Instance(base_id, _, _) = sol else {
                // A dictionary supplied by a `using` binder: the
                // obligations ride on the binder's constraint (its
                // supplier's callers establish them).
                continue;
            };
            let Some(inst) = self
                .instance_list
                .iter()
                .find(|i| i.base_id == *base_id)
                .cloned()
            else {
                continue;
            };
            // Per-method obligations over the method's own arguments
            // (`y <= x` for `x - y` at `Nat`) discharge at the direct
            // application only: its arguments are the method's. A want
            // found elsewhere in a call (a dictionary or a method passed
            // to another function) is checked as an indirect use below.
            if let Some(midx) = call.method {
                direct.insert(call.want);
                if let Some((_, mname, preds)) =
                    inst.method_obligs.iter().find(|(i, _, _)| *i == midx)
                {
                    for p in preds {
                        let mut fv = Vec::new();
                        p.free_vars(&mut fv);
                        let mut obl = p.clone();
                        for v in &fv {
                            let arg = v
                                .strip_prefix("$a")
                                .and_then(|i| i.parse::<usize>().ok())
                                .and_then(|i| call.args.get(i));
                            match arg {
                                Some((Some(it), _, _)) => obl = obl.subst(v, it),
                                Some((None, _, aspan)) => {
                                    return Err(Error::at(
                                        *aspan,
                                        format!(
                                            "`{mname}` of instance `{}` needs `{p}` of its arguments, so this argument must be a value the checker can name (a variable, a numeral, arithmetic over them); bind it with `let` first",
                                            inst.name
                                        ),
                                    ))
                                }
                                None => {
                                    return Err(Error::at(
                                        call.span,
                                        format!(
                                            "`{mname}` of instance `{}` carries the obligation `{p}` over its arguments: apply it to all of them here (its obligation discharges at a full application)",
                                            inst.name
                                        ),
                                    ))
                                }
                            }
                        }
                        if self.decide_closed_oblig(&obl, call.span)? {
                            continue;
                        }
                        if call.args.iter().any(|(_, ty, _)| self.static_discharge(ty, &obl)) {
                            continue;
                        }
                        let evidence = call.evidence.clone();
                        self.discharge_oblig(&evidence, &obl, call.span)?;
                    }
                }
            }
            let obligs: Vec<PTerm> = inst
                .ctx
                .iter()
                .filter_map(|i| match i {
                    InstCtx::Oblig(PropOblig::Pred(p)) => Some(p.clone()),
                    _ => None,
                })
                .collect();
            if obligs.is_empty() {
                continue;
            }
            if call.residual_fun {
                return Err(Error::at(
                    call.span,
                    format!(
                        "the methods of instance `{}` carry refinement obligations ({}): apply the method to all of its arguments here (its obligations discharge at its call's arguments)",
                        inst.name,
                        obligs.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
                    ),
                ));
            }
            // The obligations apply to every argument of the carrier.
            let carrier = wants
                .iter()
                .find(|w| w.id == call.want)
                .map(|w| self.uni.zonk_class(&w.class))
                .and_then(|c| c.args.into_iter().next());
            for p in &obligs {
                let binder = pred_binder(p);
                for (value, ty, aspan) in &call.args {
                    if let Some(carrier) = &carrier {
                        if !self.is_carrier_arg(ty, carrier) {
                            continue;
                        }
                    }
                    let Some(v) = value else {
                        return Err(Error::at(
                            *aspan,
                            format!(
                                "an obligation-bearing call's argument must be a named value to discharge its obligation `{p}`"
                            ),
                        ));
                    };
                    let obl = p.subst(&binder, v);
                    if self.static_discharge(ty, &obl) {
                        continue;
                    }
                    let evidence = call.evidence.clone();
                    self.discharge_oblig(&evidence, &obl, call.span)?;
                }
            }
        }
        // A method call whose call form recorded no arguments (an
        // effect-dispatch call): the obligations cannot discharge.
        for w in wants {
            if !w.method || seen.contains(&w.id) {
                continue;
            }
            let Some(DictExpr::Instance(base_id, _, _)) = solutions.get(&w.id) else {
                continue;
            };
            let Some(inst) = self
                .instance_list
                .iter()
                .find(|i| i.base_id == *base_id)
                .cloned()
            else {
                continue;
            };
            let obligs: Vec<PTerm> = inst
                .ctx
                .iter()
                .filter_map(|i| match i {
                    InstCtx::Oblig(PropOblig::Pred(p)) => Some(p.clone()),
                    _ => None,
                })
                .collect();
            if obligs.is_empty() {
                continue;
            }
            return Err(Error::at(
                w.span,
                format!(
                    "the methods of instance `{}` carry refinement obligations ({}): this call form is not supported for them (obligations discharge at a direct application's arguments)",
                    inst.name,
                    obligs.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
                ),
            ));
        }
        // An instance whose methods carry obligations over their own
        // arguments never hands its dictionary to code that could apply
        // a method without them: every want it serves must be a direct
        // application recorded above. A method used as a value, or the
        // dictionary passed to a generic callee (`using (Sub a)` at
        // `Nat`), is refused.
        for w in wants {
            if direct.contains(&w.id) {
                continue;
            }
            let Some(DictExpr::Instance(base_id, _, _)) = solutions.get(&w.id) else {
                continue;
            };
            let Some(inst) = self.instance_list.iter().find(|i| i.base_id == *base_id) else {
                continue;
            };
            if inst.method_obligs.is_empty() {
                continue;
            }
            return Err(Error::at(
                w.span,
                Self::method_oblig_refusal(inst, &self.uni.zonk_class(&w.class)),
            ));
        }
        Ok(())
    }

    /// The refusal for an indirect use of an instance whose methods
    /// carry obligations over their own arguments.
    fn method_oblig_refusal(inst: &InstanceInfo, wanted: &dyn crate::spelling::Spell) -> String {
        let stated: Vec<String> = inst
            .method_obligs
            .iter()
            .map(|(_, m, ps)| {
                format!("`{m}` needs {}", ps.iter().map(|p| format!("`{}`", Self::positional_display(p))).collect::<Vec<_>>().join(", "))
            })
            .collect();
        crate::diagnostic!(
            "instance `{}` serves {} here, but its methods carry obligations over their own arguments ({}): such a method must be applied directly to its arguments, where the obligation discharges; it cannot be used as a value or passed as a dictionary to generic code",
            inst.name,
            wanted,
            stated.join("; ")
        )
    }

    /// A positional obligation rendered with argument names (`$a1 <=
    /// $a0` reads `arg2 <= arg1`).
    fn positional_display(p: &PTerm) -> PTerm {
        let mut fv = Vec::new();
        p.free_vars(&mut fv);
        let mut out = p.clone();
        for v in &fv {
            if let Some(i) = v.strip_prefix("$a").and_then(|i| i.parse::<usize>().ok()) {
                out = out.subst(v, &ITerm::Var(format!("arg{}", i + 1).into()));
            }
        }
        out
    }

    /// Whether an argument is of the instance's carrier (an instance's
    /// refinement obligations apply to the values its methods combine).
    fn is_carrier_arg(&self, ty: &Ty, carrier: &Ty) -> bool {
        if self.uni.conv_eq(ty, carrier) {
            return true;
        }
        let tb = self.refine_base(ty).unwrap_or_else(|| ty.clone());
        let cb = self.refine_base(carrier).unwrap_or_else(|| carrier.clone());
        self.uni.conv_eq(&tb, &cb)
    }

    /// The static discharge route (the implication table): an argument
    /// whose refinement implies the obligation owes no evidence.
    fn static_discharge(&self, ty: &Ty, obl: &PTerm) -> bool {
        match self.refine_view(ty) {
            Some(rv) => pred_implies(&rv.pred, obl),
            None => false,
        }
    }

    /// Record the transfer obligations of one resolved want (refinement
    /// types design, "composability"): at a refined point the serving
    /// instance owes the point's preservation check and its preservation
    /// and postcondition rows (the closure `post p p = p`, checked in
    /// the P18 law layer's honest tier). The generic transfer's
    /// `Preserves` obligation is the same check stated as its validity
    /// condition.
    fn record_transfer(
        &self,
        inst: &InstanceInfo,
        obligs: &[PropOblig],
        container: Option<GlobalId>,
        wanted: &ClassApp,
        subdicts: &[(String, Vec<Ty>)],
        span: ast::Span,
    ) -> Result<()> {
        self.record_transfer_in(&self.uni, inst, obligs, container, wanted, subdicts, span)
    }

    fn record_transfer_in(
        &self,
        uni: &Unifier,
        inst: &InstanceInfo,
        obligs: &[PropOblig],
        container: Option<GlobalId>,
        wanted: &ClassApp,
        subdicts: &[(String, Vec<Ty>)],
        span: ast::Span,
    ) -> Result<()> {
        let Some(sig) = self
            .class_sigs
            .get(&wanted.class)
            .and_then(|ss| ss.first())
            .cloned()
        else {
            return Ok(());
        };
        let wanted = uni.zonk_class(wanted);
        let scope = match &container {
            Some(g) => g.0.clone(),
            None => format!("{}.{}", inst.module, inst.name),
        };
        for arg in &wanted.args {
            let Some(rv) = self.refine_view(arg) else {
                continue;
            };
            let Some(base) = rv.base.clone() else {
                continue;
            };
            let binder = pred_binder(&rv.pred);
            let mut map: HashMap<String, Ty> = HashMap::new();
            for (i, p) in sig.params.iter().enumerate() {
                if let Some(a) = wanted.args.get(i) {
                    map.insert(p.clone(), a.clone());
                }
            }
            // A method owes the point's preservation exactly when its
            // result is the point's carrier (`<>` and `mempty` do;
            // `foldr` does not); a refined result inside another former
            // stays at the honest boundary.
            for (mname, mscheme) in &sig.methods {
                let ty = Self::subst_ty(&mscheme.ty, &map);
                let mut res = ty;
                let res = loop {
                    match res {
                        Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } => res = *cod,
                        other => break other,
                    }
                };
                let direct = uni.conv_eq(&res, arg);
                let nested = !direct && self.ty_refine_mention(&res).is_some();
                if !direct && !nested {
                    continue;
                }
                // One check per (dictionary, point, method): the same
                // transfer resolved at several uses checks once.
                let key = (scope.clone(), rv.pred.to_string(), mname.clone());
                if !self.transfer_seen.borrow_mut().insert(key) {
                    continue;
                }
                let _ = &obligs;
                self.transfer_obligs.borrow_mut().push(TransferOblig {
                    container: scope.clone(),
                    module: inst.module.clone(),
                    instance: inst.name.clone(),
                    class: wanted.class.clone(),
                    method: mname.clone(),
                    nested,
                    class_params: sig.params.clone(),
                    class_args: wanted.args.clone(),
                    point: arg.clone(),
                    pred: rv.pred.clone(),
                    binder: binder.clone(),
                    base: base.clone(),
                    subdicts: subdicts.to_vec(),
                    span,
                });
            }
        }
        Ok(())
    }

    fn fill_holes(&mut self, e: &ECore, sol: &HashMap<usize, DictExpr>) -> Result<ECore> {
        Ok(match e {
            ECore::LitFold { idx, body } => ECore::LitFold {
                idx: *idx,
                body: Box::new(self.fill_holes(body, sol)?),
            },
            ECore::Hole(id) => match sol.get(id) {
                Some(d) => dict_expr_ecore(d),
                None => return Err(Error::plain("internal error: unsolved dictionary hole")),
            },
            ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_)
            | ECore::Unit | ECore::Var(_) | ECore::Global(..)
            | ECore::Ctor(..) | ECore::Prim(_) | ECore::Foreign(_) | ECore::QuoteHole(_)
            | ECore::IdxPending(_) | ECore::Fault
            | ECore::SyntaxLit(..) | ECore::StrLit(_) => e.clone(),
            ECore::BoxAlloc => ECore::BoxAlloc,
            ECore::BoxInit(l, v, t) => {
                ECore::BoxInit(*l, Box::new(self.fill_holes(v, sol)?), t.clone())
            }
            ECore::RefSet(l, v, ty) => ECore::RefSet(*l, Box::new(self.fill_holes(v, sol)?), ty.clone()),
            ECore::BoxGet(v, t) => ECore::BoxGet(Box::new(self.fill_holes(v, sol)?), t.clone()),
            ECore::BoxClear(v, t) => ECore::BoxClear(Box::new(self.fill_holes(v, sol)?), t.clone()),
            ECore::Quote { template, holes, body } => ECore::Quote {
                template: template.clone(),
                holes: holes
                    .iter()
                    .map(|h| self.fill_holes(h, sol))
                    .collect::<Result<Vec<_>>>()?,
                body: Box::new(self.fill_holes(body, sol)?),
            },
            ECore::DeclsQuote { template, holes, name_holes } => ECore::DeclsQuote {
                template: template.clone(),
                holes: holes
                    .iter()
                    .map(|h| self.fill_holes(h, sol))
                    .collect::<Result<Vec<_>>>()?,
                name_holes: name_holes.clone(),
            },
            ECore::DictProj(d, i, s, t) => {
                ECore::DictProj(Box::new(self.fill_holes(d, sol)?), *i, s.clone(), t.clone())
            }
            ECore::Proj(d, i, t) => ECore::Proj(Box::new(self.fill_holes(d, sol)?), i.clone(), t.clone()),
            ECore::InstDict(g, args, t, explicit) => ECore::InstDict(
                g.clone(),
                args.iter()
                    .map(|a| self.fill_holes(a, sol))
                    .collect::<Result<Vec<_>>>()?,
                t.clone(),
                *explicit,
            ),
            ECore::App(f, args) => ECore::App(
                Box::new(self.fill_holes(f, sol)?),
                args.iter()
                    .map(|a| self.fill_holes(a, sol))
                    .collect::<Result<Vec<_>>>()?,
            ),
            ECore::Lam { params, body } => ECore::Lam {
                params: params.clone(),
                body: Box::new(self.fill_holes(body, sol)?),
            },
            ECore::PolyIntro { skolems, body, ty } => ECore::PolyIntro {
                skolems: skolems.clone(),
                body: Box::new(self.fill_holes(body, sol)?),
                ty: ty.clone(),
            },
            ECore::MkClos { fn_id, env } => ECore::MkClos {
                fn_id: fn_id.clone(),
                env: env.iter()
                    .map(|a| self.fill_holes(a, sol))
                    .collect::<Result<Vec<_>>>()?,
            },
            ECore::Let(l, v, body) => ECore::Let(
                *l,
                Box::new(self.fill_holes(v, sol)?),
                Box::new(self.fill_holes(body, sol)?),
            ),
            ECore::If(c, t, f) => ECore::If(
                Box::new(self.fill_holes(c, sol)?),
                CBranch {
                    weight: t.weight,
                    body: Box::new(self.fill_holes(&t.body, sol)?),
                },
                CBranch {
                    weight: f.weight,
                    body: Box::new(self.fill_holes(&f.body, sol)?),
                },
            ),
            ECore::Demand(d, t) => ECore::Demand(Box::new(self.fill_holes(d, sol)?), t.clone()),
            ECore::LazyAlloc => ECore::LazyAlloc,
            ECore::LazyInit(l, t) => {
                ECore::LazyInit(*l, Box::new(self.fill_holes(t, sol)?))
            }
            ECore::Case { scrut, alts } => ECore::Case {
                scrut: Box::new(self.fill_holes(scrut, sol)?),
                alts: alts
                    .iter()
                    .map(|a| {
                        Ok(CAlt {
                            pat: a.pat.clone(),
                            weight: a.weight,
                            body: Box::new(self.fill_holes(&a.body, sol)?),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
            },
        })
    }
}

// ------------------------------------------------- module parameters (A5)

impl Checker {
    /// The code generation unit of a top-level function reference (module
    /// parameters as module arguments). Inside the parameterized module
    /// being checked the module's parameter binders apply first (the unit's
    /// leading runtime parameters); from outside, a parameterized module's
    /// functions are reached only through instantiation units.
    fn fn_ref(&self, v: &ValueInfo, tys: Vec<Ty>, span: ast::Span) -> Result<ECore> {
        if self.param_modules.contains(&v.module) {
            if v.module != self.current_module {
                return Err(Error::at(
                    span,
                    format!(
                        "{} belongs to the parameterized module {}; reach it through an instantiation (`import {} (args) as N`)",
                        v.name, v.module, v.module
                    ),
                ));
            }
            let mut core = ECore::Global(fn_id(&v.module, &v.name), tys);
            if !self.cur_mod_params.is_empty() {
                core = app_spine(
                    core,
                    self.cur_mod_params.iter().map(|(_, l, _)| ECore::Var(*l)).collect(),
                );
            }
            return Ok(core);
        }
        Ok(ECore::Global(fn_id(&v.module, &v.name), tys))
    }

    /// A module parameter used as a term (a capitalized dictionary
    /// parameter like the example's `Seq` is a constructor-shaped name
    /// that resolves to the parameter's binder first): the binder local
    /// and the parameter's type.
    fn module_param_use(&self, p: &ast::ConPath) -> Option<(LocalId, Ty)> {
        if p.segments.len() != 1 {
            return None;
        }
        if p.segments[0].resolved_binding().is_some() { return None; }
        if p.segments[0].scoped_reference().is_some_and(|scope| scope.origin() != self.current_module) { return None; }
        let name = &p.segments[0].text;
        self.cur_mod_params
            .iter()
            .find(|(n, _, _)| n == name)
            .map(|(_, l, t)| (*l, t.clone()))
    }
}

/// The recorded identity of a module argument: an instance dictionary's
/// construction unit over its sub-dictionaries, or a value's unit or
/// source rendering. Two instantiations at differently identified
/// arguments are distinct units.
fn arg_identity(core: &crate::core::Core, text: &str) -> String {
    match core {
        crate::core::Core::InstDict(id, subs, _) => {
            let inner: Vec<String> = subs.iter().map(|s| arg_identity(s, "")).collect();
            format!("dict:{id}({})", inner.join(","))
        }
        crate::core::Core::Global(g, _) => format!("val:{g}"),
        _ => format!("val:{text}"),
    }
}

/// The term spelling of a named module argument: a capitalized name is a
/// constructor-shaped reference (`Seq`, `Lib.SeqResult`), a lowercase one
/// an ordinary value reference (`Lib.f`).
fn module_arg_term(quals: &[ast::Name], name: &ast::Name) -> ast::Term {
    if name.text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        let mut segments = quals.to_vec();
        segments.push(name.clone());
        ast::Term::Con(ast::ConPath { segments })
    } else {
        ast::Term::Var(ast::VarPath { quals: quals.to_vec(), name: name.clone() })
    }
}

/// The term spelling of a literal module argument.
fn module_arg_lit(arg: &ast::ModuleArg) -> Result<ast::Term> {
    match arg {
        ast::ModuleArg::Int(v, s) => Ok(ast::Term::Int(v.clone(), *s)),
        ast::ModuleArg::Char(c, s) => Ok(ast::Term::Char(c.clone(), *s)),
        ast::ModuleArg::Unit(s) => Ok(ast::Term::Unit(*s)),
        ast::ModuleArg::Name { .. } => {
            unreachable!("a named module argument is spelled as a term")
        }
    }
}

/// The source rendering of a named module argument.
fn arg_name_text(quals: &[ast::Name], name: &ast::Name) -> String {
    let mut out: Vec<String> = quals.iter().map(|q| q.text.clone()).collect();
    out.push(name.text.clone());
    out.join(".")
}

/// The source rendering of a module argument, for the unit's identity.
fn arg_lit_text(arg: &ast::ModuleArg) -> String {
    match arg {
        ast::ModuleArg::Int(v, _) => v.clone(),
        ast::ModuleArg::Char(c, _) => format!("'{c}'"),
        ast::ModuleArg::Unit(_) => "()".to_string(),
        ast::ModuleArg::Name { quals, name } => arg_name_text(quals, name),
    }
}

/// Optimizer metadata for a wrapper unit: the target's metadata over the
/// forwarded parameters (the leading module parameters are applied at the
/// unit, not forwarded).
fn shifted_meta(meta: &crate::core::FnMeta, k: usize) -> crate::core::FnMeta {
    let shift = |i: usize| if i >= k { Some(i - k) } else { None };
    crate::core::FnMeta {
        linear: meta
            .linear
            .iter()
            .filter_map(|(i, m)| shift(*i).map(|j| (j, *m)))
            .collect(),
        pure_arrows: meta.pure_arrows.iter().filter_map(|i| shift(*i)).collect(),
        dicts: meta
            .dicts
            .iter()
            .filter_map(|(i, c)| shift(*i).map(|j| (j, c.clone())))
            .collect(),
        pure: meta.pure,
        prim_body: meta.prim_body,
        contract: crate::core::FnContract {
            params: meta.contract.params.iter().skip(k).cloned().collect(),
            ..meta.contract.clone()
        },
    }
}

// The rest of the checker (term elaboration, pattern-match compilation)
// lives in `check_terms.rs`; declaration collection and program assembly in
// `check_decls.rs`. Both are included at module scope.
include!("check_terms.rs");
include!("check_code.rs");
include!("check_dictionary.rs");
include!("check_decls.rs");
include!("check_default_wire.rs");
include!("check_interface.rs");
include!("check_total.rs");
include!("check_erasure.rs");

#[cfg(test)]
mod proof_value_scope_tests {
    use super::*;
    #[test]
    fn logical_provenance_does_not_grant_source_names() {
        let mut env = Env::default();
        env.locals.push(("payload".into(), LocalId(1), Scheme::mono(Ty::Unit)));
        env.locals.push(("payload".into(), LocalId(2), Scheme::mono(Ty::Unit)));
        assert_eq!(env.lookup("payload").unwrap().0, LocalId(2));
        assert!(env.lookup(&ITerm::LocalValue(LocalId(1)).to_string()).is_none());
        let captured = PTerm::Eq(Box::new(ITerm::Var("payload".into())), Box::new(ITerm::num(0)));
        let checker = Checker::new();
        assert_eq!(checker.proof_values_in_scope(&captured,
            &HashMap::from([("payload".into(), LocalId(1))]), &env),
            PTerm::Eq(Box::new(ITerm::LocalValue(LocalId(1))), Box::new(ITerm::num(0))));
    }
}

#[cfg(test)]
mod index_substitution_tests {
    use super::*;

    #[test]
    fn index_headers_wait_for_complete_instances_even_when_one_matches() {
        let mut checker = Checker::new();
        checker.collecting_declarations = true;
        let class = ClassApp { class: "SampleMeasure".into(), args: vec![Ty::Int] };
        checker.instance_list.push(InstanceInfo {
            module: "Library".into(), name: "MeasureInt".into(), params: vec![], ctx: vec![], target: class.clone(),
            base_id: GlobalId("Library.MeasureInt".into()), implicit: true, base: false,
            method_obligs: vec![], method_contracts: vec![],
        });
        checker.instances.insert(class.class.clone(), vec![0]);
        let request = |checker: &Checker| {
            let mut inference = IndexInference::new(&checker.uni);
            let witness = inference.uni.fresh_idx_term();
            inference.dictionaries.push(PendingIndexDictionary {
                parameter: witness.clone(), class: class.clone(), scope: vec![], span: ast::Span::new(1, 1),
            });
            (inference, witness)
        };
        let (mut incomplete, _) = request(&checker);
        assert!(checker.finish_index_inference(&mut incomplete).is_err());
        assert!(checker.declaration_needs_headers.get());
        checker.declaration_seed = Some(checker.instance_list.clone());
        let (mut complete, witness) = request(&checker);
        checker.finish_index_inference(&mut complete).unwrap();
        assert!(matches!(complete.uni.resolve_idx(&witness), ITerm::Inst(call)
            if call.head == IndexHead::Dictionary("Library.MeasureInt".into())));
    }

    #[test]
    fn index_generic_copy_owns_its_deferred_evidence_and_preserves_outer_captures() {
        let mut checker = Checker::new();
        let span = ast::Span::new(1, 1);
        let binding = checker.fresh_local();
        let outer = checker.fresh_local();
        let inner = checker.fresh_local();
        let argument = checker.fresh_local();
        let parameter = checker.uni.fresh();
        let Ty::Meta(meta) = parameter else { unreachable!() };
        let witness = checker.uni.fresh_idx_term();
        let class = ClassApp { class: "SampleMeasure".into(), args: vec![] };
        let mut environment = Env::default();
        environment.dicts = vec![(outer, class.clone()), (inner, class.clone())];
        let want = checker.new_context_want(&InstantiatedConstraint {
            name: "provider".into(), class, witness: witness.clone(),
        }, span, &environment);
        checker.idx_pends.push(IdxPend {
            iterm: witness.clone(), carrier: Ty::Nat, span,
            scope: IndexRuntimeScope {
                values: HashMap::from([("item".into(), argument)]),
                locals: HashSet::from([argument]),
                dictionaries: HashSet::from([outer, inner]),
            },
        });
        checker.prop_calls.push(PropCall {
            want, method: None, args: vec![(Some(witness.clone()), parameter.clone(), span)],
            evidence: Evidence { items: vec![("source".into(), PTerm::Eq(Box::new(witness.clone()), Box::new(witness.clone())), inner, vec![])],
                value_sorts: vec![(argument, IntSort::Nat), (outer, IntSort::Nat)], ..Evidence::default() },
            residual_fun: false, span,
        });
        let stage_id = GlobalId("<checked-stage-call:0>".into());
        checker.stage_calls.borrow_mut().insert(stage_id.0.clone(), StageCall {
            target: GlobalId("Library.helper".into()),
            arguments: StageTypeBindings {
                types: HashMap::from([("a".into(), parameter.clone())]),
                evidence: HashMap::from([(dictionary_parameter("provider"), witness.clone())]),
                ..StageTypeBindings::default()
            },
        });
        let original_type = Ty::Tuple(vec![parameter.clone(), Ty::Idx(Box::new(witness.clone())),
            Ty::Idx(Box::new(ITerm::LocalValue(inner))), Ty::Idx(Box::new(ITerm::LocalValue(argument))),
            Ty::Idx(Box::new(ITerm::LocalValue(outer)))]);
        checker.local_tys.insert(inner, original_type.clone());
        checker.gen_defs.insert(binding, ECore::Lam {
            params: vec![argument], body: Box::new(ECore::Let(inner, Box::new(ECore::Unit), Box::new(ECore::App(
                Box::new(ECore::Global(stage_id.clone(), vec![parameter])), vec![ECore::Hole(want), ECore::IdxPending(0), ECore::Var(outer)],
            )))),
        });
        let caller_argument = Ty::Tuple(vec![Ty::Prim(Scalar::U32),
            Ty::Idx(Box::new(ITerm::Evidence(IndexEvidence::Local(inner.0)))), Ty::Idx(Box::new(witness.clone())),
            Ty::Idx(Box::new(ITerm::LocalValue(inner)))]);
        let (copy_type, body) = checker.gen_copy(binding, &HashMap::from([(meta, caller_argument.clone())]), &original_type);
        let ECore::Lam { params, body } = body else { panic!("lost function") };
        let ECore::Let(new_inner, _, body) = *body else { panic!("lost provider") };
        let ECore::App(head, args) = *body else { panic!("lost call") };
        let ECore::Global(new_stage, types) = *head else { panic!("lost stage metadata") };
        let [ECore::Hole(new_want), ECore::IdxPending(new_index), ECore::Var(captured)] = args.as_slice() else { panic!("lost deferred call records") };
        assert_ne!(params[0], argument);
        assert_ne!(new_inner, inner);
        assert_eq!(*captured, outer);
        assert_ne!(*new_want, want);
        assert_eq!(checker.wants[0].scope[1].0, inner);
        assert_eq!(checker.wants[1].scope[1].0, new_inner);
        assert_eq!(checker.wants[1].scope[0].0, outer);
        assert_eq!(types, vec![caller_argument.clone()]);
        assert_ne!(new_stage, stage_id);
        let pending = &checker.idx_pends[*new_index];
        assert_eq!(pending.scope.values["item"], params[0]);
        assert_eq!(pending.scope.locals, HashSet::from([params[0]]));
        assert_eq!(pending.scope.dictionaries, HashSet::from([outer, new_inner]));
        assert_eq!(checker.prop_calls[1].want, *new_want);
        assert_eq!(checker.prop_calls[1].evidence.items[0].2, new_inner);
        assert_eq!(checker.prop_calls[1].evidence.value_sorts, vec![(params[0], IntSort::Nat), (outer, IntSort::Nat)]);
        let call = checker.stage_calls.borrow()[&new_stage.0].clone();
        assert_eq!(call.arguments.types["a"], caller_argument);
        checker.solve_wants().unwrap();
        let expected = Ty::Tuple(vec![checker.uni.zonk(&caller_argument), Ty::Idx(Box::new(ITerm::Evidence(IndexEvidence::Local(new_inner.0)))),
            Ty::Idx(Box::new(ITerm::LocalValue(new_inner))), Ty::Idx(Box::new(ITerm::LocalValue(params[0]))),
            Ty::Idx(Box::new(ITerm::LocalValue(outer)))]);
        assert_eq!(checker.uni.zonk(&copy_type), expected);
        assert_eq!(checker.uni.zonk(&checker.local_tys[&new_inner]), expected);
        assert_eq!(checker.uni.resolve_idx(&call.arguments.evidence[&dictionary_parameter("provider")]), ITerm::Evidence(IndexEvidence::Local(new_inner.0)));
        assert_eq!(checker.uni.resolve_idx(&checker.idx_pends[*new_index].iterm), ITerm::Evidence(IndexEvidence::Local(new_inner.0)));
        assert_eq!(checker.uni.resolve_idx(&witness), ITerm::Evidence(IndexEvidence::Local(inner.0)));
    }

    #[test]
    fn index_substitution_does_not_capture_callers_same_named_evidence() {
        let caller = ITerm::Evidence(dictionary_parameter("provider"));
        let original = Ty::Tuple(vec![Ty::Param("a".into()), Ty::Idx(Box::new(caller.clone()))]);
        let result = Checker::subst_namespaces(&original,
            &HashMap::from([("a".into(), Ty::Idx(Box::new(caller.clone())))]),
            &HashMap::new(), &HashMap::new(),
            &HashMap::from([(dictionary_parameter("provider"), ITerm::Evidence(IndexEvidence::Local(12)))]));
        assert_eq!(result, Ty::Tuple(vec![Ty::Idx(Box::new(caller)), Ty::Idx(Box::new(ITerm::Evidence(IndexEvidence::Local(12))))]));
    }

    #[test]
    fn checked_body_copy_instantiates_stage_call_rows_and_regions() {
        let mut checker = Checker::new();
        let stage = GlobalId("<checked-stage-call:0>".into());
        let row = EffRow { labels: vec![EffLabel::with_arg("Read", "region#rigid")], tail: RowTail::Rigid("row#rigid".into()) };
        let ty = Ty::fun_eff(Ty::Unit, row.clone(), Ty::Region("region#rigid".into()));
        let solved = checker.uni.fresh_row();
        checker.uni.unify_rows(&solved, &row).unwrap();
        checker.stage_calls.borrow_mut().insert(stage.0.clone(), StageCall {
            target: GlobalId("Library.helper".into()),
            arguments: StageTypeBindings {
                types: HashMap::from([("a".into(), ty.clone())]),
                rows: HashMap::from([("e".into(), solved.tail)]),
                regions: HashMap::from([("r".into(), "region#rigid".into())]),
                ..StageTypeBindings::default()
            },
        });
        let scope = StageTypeBindings {
            rows: HashMap::from([("row#rigid".into(), RowTail::Closed)]),
            regions: HashMap::from([("region#rigid".into(), "caller-region".into())]),
            ..StageTypeBindings::default()
        };
        let (_, copied) = checker.copy_checked_definition(&ECore::Global(stage.clone(), vec![ty]), &HashMap::new(), &scope, &Ty::Unit);
        let ECore::Global(copied, _) = copied else { panic!("lost stage call") };
        assert_ne!(copied, stage);
        let call = checker.stage_calls.borrow()[&copied.0].clone();
        let expected = EffRow::closed(vec![EffLabel::with_arg("Read", "caller-region")]);
        assert_eq!(checker.uni.resolve_row(&EffRow { labels: vec![], tail: call.arguments.rows["e"].clone() }), expected);
        assert_eq!(call.arguments.regions["r"], "caller-region");
        assert_eq!(call.arguments.types["a"], Ty::fun_eff(Ty::Unit, expected, Ty::Region("caller-region".into())));
        assert_eq!(checker.stage_calls.borrow()[&stage.0].arguments.regions["r"], "region#rigid");
    }

    #[test]
    fn index_dictionary_reification_requires_checked_runtime_scope() {
        let checker = Checker::new();
        let span = ast::Span::new(1, 1);
        let evidence = ITerm::Evidence(IndexEvidence::Local(7));
        let mut scope = IndexRuntimeScope::default();
        scope.values.insert("$dictionary-local:7".into(), LocalId(8));
        assert!(checker.reify_idx(&evidence, &scope, true, span).is_err());
        scope.dictionaries.insert(LocalId(7));
        assert!(matches!(checker.reify_idx(&evidence, &scope, true, span).unwrap(), ECore::Var(LocalId(7))));
        assert!(matches!(checker.reify_idx(&ITerm::Var("$dictionary-local:7".into()), &scope, true, span).unwrap(), ECore::Var(LocalId(8))));
    }

    #[test]
    fn index_quote_copy_remaps_native_metadata_without_rebinding_source_names() {
        let span = ast::Span::new(1, 1);
        let name = ast::Name::bound("captured", span, ast::LocalBinding::fresh());
        let identity = |local| Ty::Idx(Box::new(ITerm::Evidence(IndexEvidence::Local(local))));
        let mut scope = Scheme::mono(Ty::Tuple(vec![identity(0), identity(1)]));
        scope.evidence_params.push(IndexEvidence::Local(0));
        scope.ctx.push(("provider".into(), ClassApp { class: "Measure".into(), args: vec![identity(1)] }));
        scope.obligs.push(PropOblig::Pred(PTerm::Eq(Box::new(ITerm::Evidence(IndexEvidence::Local(0))), Box::new(ITerm::num(42)))));
        let template = ast::Term::Ann(Box::new(ast::Term::Var(ast::VarPath { quals: vec![], name: name.clone() })), ast::Type::Elaborated(Box::new(scope), span), span);
        let quote = ECore::Quote { template: Box::new(template), holes: vec![], body: Box::new(ECore::Var(LocalId(0))) };
        let ECore::Quote { template, body, .. } = rewrite_locals(&quote, &HashMap::from([(LocalId(0), LocalId(1)), (LocalId(1), LocalId(2))])) else { panic!("lost quote") };
        assert!(matches!(*body, ECore::Var(LocalId(1))));
        let ast::Term::Ann(body, ast::Type::Elaborated(scope, _), _) = *template else { panic!("lost checked annotation") };
        assert_eq!(scope.ty, Ty::Tuple(vec![identity(1), identity(2)]));
        assert_eq!(scope.evidence_params, vec![IndexEvidence::Local(1)]);
        assert_eq!(scope.ctx[0].1.args, vec![identity(2)]);
        assert!(matches!(&scope.obligs[0], PropOblig::Pred(PTerm::Eq(left, _)) if **left == ITerm::Evidence(IndexEvidence::Local(1))));
        let ast::Term::Var(path) = *body else { panic!("lost source reference") };
        assert_eq!(path.name.local_binding(), name.local_binding());
    }

    #[test]
    fn separate_dictionary_calls_keep_separate_dependent_result_indices() {
        let mut checker = Checker::new();
        let class = ClassApp { class: "SampleMeasure".into(), args: vec![] };
        let mut scheme = Scheme::mono(Ty::Adt("Tagged".into(), vec![Ty::Idx(Box::new(ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function("Measures.measure".into()), types: vec![],
            dictionaries: vec![ITerm::Evidence(dictionary_parameter("provider"))], arguments: vec![],
        }))))]));
        scheme.ctx.push(("provider".into(), class.clone()));
        let first = checker.fresh_local(); let second = checker.fresh_local();
        let mut environment = Env::default();
        environment.dicts.push((first, class.clone()));
        let (first_type, first_context, _) = checker.instantiate(&scheme);
        checker.new_context_want(&first_context[0], ast::Span::new(1, 1), &environment);
        environment.dicts.push((second, class));
        let (second_type, second_context, _) = checker.instantiate(&scheme);
        checker.new_context_want(&second_context[0], ast::Span::new(2, 1), &environment);
        checker.solve_wants().unwrap();
        let first_type = checker.uni.zonk(&first_type);
        let second_type = checker.uni.zonk(&second_type);
        assert!(first_type.mentions_evidence(&IndexEvidence::Local(first.0)));
        assert!(second_type.mentions_evidence(&IndexEvidence::Local(second.0)));
        assert!(!crate::types::ty_eq(&first_type, &second_type));
        assert!(checker.uni.unify(&first_type, &second_type).is_err());
    }

    #[test]
    fn native_substitution_preserves_instantiated_types_rows_regions_and_evidence() {
        let call = ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function("Measure.measure".into()),
            types: vec![Ty::fun_eff(Ty::Param("a".into()), EffRow { labels: vec![EffLabel::with_arg("Read", "r")], tail: RowTail::Rigid("e".into()) }, Ty::Region("r".into()))],
            dictionaries: vec![ITerm::Var("dict".into())], arguments: vec![
                IndexArgument::Value { value: ITerm::Var("n".into()), carrier: Ty::Nat, runtime: true },
                IndexArgument::Row(EffRow { labels: vec![EffLabel::with_arg("Read", "r")], tail: RowTail::Rigid("e".into()) }),
                IndexArgument::Region("r".into()),
                IndexArgument::Proposition(PTerm::Eq(Box::new(ITerm::Var("n".into())), Box::new(ITerm::num(7)))),
            ],
        }));
        let types = HashMap::from([("a".into(), Ty::Bool), ("dict".into(), Ty::Idx(Box::new(ITerm::Var("provider#7".into())))), ("n".into(), Ty::Idx(Box::new(ITerm::num(7))))]);
        let result = Checker::subst_full(&Ty::Idx(Box::new(call)), &types,
            &HashMap::from([("e".into(), RowTail::Closed)]), &HashMap::from([("r".into(), "scope#9".into())]));
        let Ty::Idx(term) = result else { panic!("lost index") };
        let ITerm::Inst(call) = *term else { panic!("erased instantiation") };
        assert_eq!(call.types, vec![Ty::fun_eff(Ty::Bool, EffRow::closed(vec![EffLabel::with_arg("Read", "scope#9")]), Ty::Region("scope#9".into()))]);
        assert_eq!(call.dictionaries, vec![ITerm::Var("provider#7".into())]);
        assert_eq!(call.arguments, vec![
            IndexArgument::Value { value: ITerm::num(7), carrier: Ty::Nat, runtime: true },
            IndexArgument::Row(EffRow::closed(vec![EffLabel::with_arg("Read", "scope#9")])),
            IndexArgument::Region("scope#9".into()),
            IndexArgument::Proposition(PTerm::Eq(Box::new(ITerm::num(7)), Box::new(ITerm::num(7)))),
        ]);
    }

    #[test]
    fn a_shadowed_index_parameter_does_not_block_other_substitutions() {
        let ty = Ty::pi("n", Ty::Nat, Mult::Many, EffRow::pure(), Ty::Tuple(vec![Ty::Param("a".into()), Ty::Idx(Box::new(ITerm::Var("n".into())))]));
        let result = Checker::subst_ty(&ty, &HashMap::from([("n".into(), Ty::Idx(Box::new(ITerm::num(99)))), ("a".into(), Ty::Bool)]));
        assert_eq!(result, Ty::pi("n", Ty::Nat, Mult::Many, EffRow::pure(), Ty::Tuple(vec![Ty::Bool, Ty::Idx(Box::new(ITerm::Var("n".into())))])));
    }
}

#[cfg(test)]
mod local_value_capability_tests {
    use super::*;

    #[test]
    fn local_value_reification_and_export_require_structural_authority() {
        let mut checker = Checker::new();
        let outer = checker.fresh_local();
        let inner = checker.fresh_local();
        let value = ITerm::LocalValue(outer);
        let span = ast::Span::new(1, 1);
        let mut scope = IndexRuntimeScope::default();
        scope.values.insert("payload".into(), inner);
        scope.values.insert(value.to_string(), outer);
        assert!(checker.reify_idx(&value, &scope, true, span).is_err());
        scope.locals.insert(outer);
        assert!(matches!(checker.reify_idx(&value, &scope, true, span), Ok(ECore::Var(id)) if id == outer));
        let mut portable = InterfaceIndexScope::default();
        portable.locals.insert(value.to_string(), InterfaceIndexSort::Mathematical);
        assert!(checker.portable_index(&value, &portable).is_none());
    }

    #[test]
    fn hidden_value_reads_keep_the_original_mutable_region_and_numeric_sort() {
        let checker = Checker::new();
        let outer = LocalId(1);
        let inner = LocalId(2);
        let mut env = Env::default();
        env.locals.push(("payload".into(), outer, Scheme::mono(Ty::Mut("original".into(), Box::new(Ty::Nat)))));
        env.locals.push(("payload".into(), inner, Scheme::mono(Ty::Mut("replacement".into(), Box::new(Ty::Nat)))));
        let prop = PTerm::Eq(Box::new(ITerm::LocalValue(outer)), Box::new(ITerm::num(0)));
        assert_eq!(checker.ev_footprint(&prop, &env), (vec!["original".into()], vec![]));
        env.locals = vec![("payload".into(), outer, Scheme::mono(Ty::Nat)),
            ("payload".into(), inner, Scheme::mono(Ty::Nat))];
        let evidence = checker.evidence_in(&env);
        assert!(evidence.value_sorts.contains(&(outer, IntSort::Nat)));
        let sorts = Sorts { values: evidence.value_sorts, ..Sorts::default() };
        let nonnegative = PTerm::Cmp(IdxCmp::Ge, Box::new(ITerm::LocalValue(outer)), Box::new(ITerm::num(0)));
        assert!(linear_entails(&[], &nonnegative, &sorts).is_some());
    }
}

#[cfg(test)]
mod index_lambda_identity_tests {
    use super::*;

    #[test]
    fn index_lambda_handles_survive_renaming_and_cannot_be_guessed() {
        let checker = Checker::new();
        let span = ast::Span::new(1, 1);
        let handle = ast::LocalBinding::fresh();
        let owner = ast::Name::bound("item", span, handle.clone());
        let renamed = ast::Name::bound("renamed", span, handle);
        let other = ast::Name::bound("item", span, ast::LocalBinding::fresh());
        let variable = checker.fresh_index_lambda_variable();
        checker.with_index_lambda(IndexLambdaBinder { name: owner.clone(), variable: variable.clone(), carrier: Some(Ty::Nat) }, |checker| {
            assert_eq!(checker.index_lambda_binding(&renamed).unwrap().variable, variable);
            assert!(checker.index_lambda_binding(&other).is_none());
            assert!(checker.index_lambda_binding(&ast::Name::new("item", span)).is_none());
            assert!(checker.index_lambda_binding(&ast::Name::new(variable.clone(), span)).is_none());
            let term = ast::Type::Var(renamed.clone(), span);
            assert_eq!(checker.conv_idx_expected(&term, &[], Some(&Ty::Nat))?, ITerm::Var(variable.as_str().into()));
            let forged = ast::Type::Var(ast::Name::new(variable.clone(), span), span);
            assert!(checker.conv_idx_expected(&forged, &[], Some(&Ty::Nat)).is_err());
            Ok(())
        }).unwrap();
        assert!(checker.index_lambda_binding(&owner).is_none());
    }

    #[test]
    fn nested_index_lambda_scopes_restore_after_failure() {
        let checker = Checker::new();
        let span = ast::Span::new(1, 1);
        let outer = ast::Name::bound("item", span, ast::LocalBinding::fresh());
        let inner = ast::Name::bound("item", span, ast::LocalBinding::fresh());
        let outer_variable = checker.fresh_index_lambda_variable();
        let inner_variable = checker.fresh_index_lambda_variable();
        assert_ne!(outer_variable, inner_variable);
        checker.with_index_lambda(IndexLambdaBinder { name: outer.clone(), variable: outer_variable.clone(), carrier: Some(Ty::Nat) }, |checker| {
            let failed: Result<()> = checker.with_index_lambda(IndexLambdaBinder { name: inner.clone(), variable: inner_variable.clone(), carrier: Some(Ty::Bool) }, |checker| {
                assert_eq!(checker.index_lambda_binding(&outer).unwrap().variable, outer_variable);
                assert_eq!(checker.index_lambda_binding(&inner).unwrap().variable, inner_variable);
                Err(Error::at(span, "intentional scope failure"))
            });
            assert!(failed.is_err());
            assert!(checker.index_lambda_binding(&inner).is_none());
            assert_eq!(checker.index_lambda_binding(&outer).unwrap().variable, outer_variable);
            Ok(())
        }).unwrap();
        assert!(checker.index_lambda_binders.borrow().is_empty());
    }
}

#[cfg(test)]
mod index_carrier_compatibility_tests {
    use super::*;

    #[test]
    fn carrier_usage_holes_do_not_authorize_type_index_or_effect_specialization() {
        let mut checker = Checker::new();
        let span = ast::Span::new(1, 1);
        let inferred = Ty::fun(Ty::Nat, Ty::Nat);
        let unrestricted = Ty::fun_mult(Ty::Nat, Mult::Many, EffRow::pure(), Ty::Nat);
        assert!(checker.check_captured_index_carrier("f", &unrestricted, &inferred, span).is_ok());
        let linear = Ty::fun_mult(Ty::Nat, Mult::One, EffRow::pure(), Ty::Nat);
        assert!(checker.check_captured_index_carrier("f", &linear, &unrestricted, span).is_err());
        let unknown_type = checker.uni.fresh();
        assert!(checker.check_captured_index_carrier("x", &unknown_type, &Ty::Nat, span).is_err());
        assert_eq!(checker.uni.zonk(&unknown_type), unknown_type);
        let unknown_index = checker.uni.fresh_idx_term();
        assert!(checker.check_captured_index_carrier("x", &Ty::Idx(Box::new(unknown_index.clone())), &Ty::Idx(Box::new(ITerm::num(0))), span).is_err());
        assert_eq!(checker.uni.resolve_idx(&unknown_index), unknown_index);
        let unknown_effect = checker.uni.fresh_row();
        let effectful = Ty::fun_mult(Ty::Nat, Mult::Many, unknown_effect.clone(), Ty::Nat);
        assert!(checker.check_captured_index_carrier("f", &effectful, &unrestricted, span).is_err());
        assert_eq!(checker.uni.resolve_row(&unknown_effect), unknown_effect);
    }
}

#[cfg(test)]
mod signature_scope_tests {
    use super::*;

    fn signature(owner: &str, name: &str) -> SigInfo {
        SigInfo { module: owner.into(), name: name.into(), params: vec!["a".into()],
            param_kinds: vec![ParamKind::Type], methods: vec![], defaults: vec![], laws: vec![] }
    }

    fn captured(path: ast::ConPath, scope: ast::GlobalScope) -> ast::ConPath {
        let span = path.span();
        let ty = ast::capture_type_in_declaration_scope(&ast::Type::Con(path, span), &[],
            Some(std::sync::Arc::new(scope)), &[], &[], &[]).unwrap();
        let ast::Type::Con(path, _) = ty else { panic!("signature path") };
        path
    }

    #[test]
    fn signature_scope_uses_frozen_owners_and_never_repairs_absent_or_wrong_kind_names() {
        let mut checker = Checker::new();
        checker.current_module = "Main".into();
        for owner in ["Library", "Main"] {
            let sig = signature(owner, "Private");
            checker.sigs.entry("Private".into()).or_default().push(sig.clone());
            checker.qualified_sigs.insert((owner.into(), "Private".into()), sig);
        }
        let span = ast::Span::new(1, 1);
        let path = || ast::ConPath { segments: vec![ast::Name::new("Private", span)] };
        assert_eq!(checker.lookup_sig(&path()).unwrap().module, "Main");
        let mut scope = ast::GlobalScope::new("Library");
        scope.insert(ast::RefKind::Type, vec!["Private".into()], "Library", "Private");
        assert_eq!(checker.lookup_sig(&captured(path(), scope)).unwrap().module, "Library");
        assert!(checker.lookup_sig(&captured(path(), ast::GlobalScope::new("Before"))).is_err());
        let wrong_kind = path();
        wrong_kind.segments[0].record_binding(ast::RefKind::Value, "Main", "Private");
        assert!(checker.lookup_sig(&wrong_kind).is_err());
        let local = ast::ConPath { segments: vec![ast::Name::bound("Private", span, ast::LocalBinding::fresh())] };
        assert!(checker.lookup_sig(&local).is_err());
    }

    #[test]
    fn signature_scope_owned_handles_select_their_actual_member_without_spelling_fallback() {
        let mut checker = Checker::new();
        checker.qualified_sigs.insert(("Library".into(), "Actual".into()), signature("Library", "Actual"));
        let span = ast::Span::new(1, 1);
        let binding = ast::LocalBinding::fresh();
        checker.declaration_bindings.insert((binding.clone(), ast::RefKind::Type), ("Library".into(), "Actual".into()));
        let path = ast::ConPath { segments: vec![ast::Name::new("Decoy", span).with_declaration_binding(binding)] };
        assert_eq!(checker.lookup_sig(&path).unwrap().name, "Actual");
        let absent = ast::ConPath { segments: vec![ast::Name::new("Actual", span).with_declaration_binding(ast::LocalBinding::fresh())] };
        assert!(checker.lookup_sig(&absent).is_err());
    }

    #[test]
    fn signature_scope_keeps_synthetic_builtin_classes_available_to_captures() {
        let checker = Checker::new();
        let span = ast::Span::new(1, 1);
        for (source, expected) in [("Copy", "Copy"), ("Share", "Copy"), ("Io", "Io")] {
            let path = ast::ConPath { segments: vec![ast::Name::new(source, span)] };
            let info = checker.lookup_sig(&captured(path, (*checker.syntax_global_scope("Main")).clone())).unwrap();
            assert_eq!(info.module, "<builtin>");
            assert_eq!(info.name, expected);
        }
    }
}

#[cfg(test)]
mod owned_effect_scope_tests {
    use super::*;

    #[test]
    fn frozen_symbolic_labels_and_type_names_cannot_acquire_an_owned_effect() {
        let mut checker = Checker::new();
        checker.current_module = "Main".into();
        checker.effect_bindings.insert(("Main".into(), "Event".into()), "Main.Event".into());
        checker.effects.insert("Main.Event".into(), EffectInfo { module: "Main".into(), name: "Event".into(), region_param: None, ops: vec![] });
        let span = ast::Span::new(1, 1);
        let path = ast::ConPath { segments: vec![ast::Name::new("Event", span)] };
        assert_eq!(checker.effect_head(&path, false).unwrap(), "Main.Event");
        let annotation = ast::Type::Fun { arg: Box::new(ast::Type::Unit(span)), mult: None,
            eff: Some(ast::EffectRow { labels: vec![(path.clone(), vec![])], tail: None, span }),
            res: Box::new(ast::Type::Unit(span)), span };
        let captured = ast::capture_type_in_declaration_scope(&annotation, &[], Some(std::sync::Arc::new(ast::GlobalScope::new("Before"))), &[], &[], &[]).unwrap();
        let ast::Type::Fun { eff: Some(row), .. } = captured else { panic!("function annotation") };
        assert_eq!(checker.effect_head(&row.labels[0].0, false).unwrap(), "Event");
        assert!(checker.effect_head(&row.labels[0].0, true).is_err());
        let mut scope = ast::GlobalScope::new("Main");
        scope.insert(ast::RefKind::Type, vec!["Event".into()], "Main", "Event");
        scope.insert(ast::RefKind::Effect, vec!["Event".into()], "Main", "Event");
        let captured = ast::capture_type_in_declaration_scope(&ast::Type::Con(path, span), &[], Some(std::sync::Arc::new(scope)), &[], &[], &[]).unwrap();
        let ast::Type::Con(type_name, _) = captured else { panic!("type name") };
        assert!(checker.effect_head(&type_name, false).is_err());
    }
}

#[cfg(test)]
mod owned_effect_qualified_scope_tests {
    use super::*;

    #[test]
    fn an_unbound_qualified_label_cannot_spell_a_private_owned_head() {
        let mut checker = Checker::new();
        checker.current_module = "Outside".into();
        checker.effect_bindings.insert(("Owner".into(), "Secret".into()), "Owner.Secret".into());
        checker.effects.insert("Owner.Secret".into(), EffectInfo { module: "Owner".into(), name: "Secret".into(), region_param: None, ops: vec![] });
        let span = ast::Span::new(1, 1);
        let path = ast::ConPath { segments: vec![ast::Name::new("Owner", span), ast::Name::new("Secret", span)] };
        assert!(checker.effect_head(&path, false).is_err());
        let annotation = ast::Type::Fun { arg: Box::new(ast::Type::Unit(span)), mult: None,
            eff: Some(ast::EffectRow { labels: vec![(path, vec![])], tail: None, span }),
            res: Box::new(ast::Type::Unit(span)), span };
        let captured = ast::capture_type_in_declaration_scope(&annotation, &[], Some(std::sync::Arc::new(ast::GlobalScope::new("Outside"))), &[], &[], &[]).unwrap();
        let ast::Type::Fun { eff: Some(row), .. } = captured else { panic!("function annotation") };
        assert!(checker.effect_head(&row.labels[0].0, false).is_err());
    }
}
