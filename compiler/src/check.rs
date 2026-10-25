//! Type checking and elaboration into [`crate::core`].
//!
//! The checker is bidirectional with explicit `forall` quantification (this
//! draft does not silently generalize). Constraints from `using (...)`
//! contexts become dictionary parameters; method calls elaborate to
//! dictionary projection plus application; dictionary selection is solved
//! from visible `using` binders or unique `implicit instance` declarations.
//! Operators are ordinary value names, so `x + y` is just a method call.

use crate::ast;
use crate::core::{CPat, CmpOp, FnDef, GlobalId, InstanceDef, IntOp, LocalId, NatOp, PrimOp, Program};
use crate::types::{ClassApp, EffLabel, EffRow, EitherMeta, Mult, RowTail, Scalar, Scheme, Ty, Unifier, collect_row_names, default_mult};
use crate::{Error, Result};
use SolveOutcome::{Solved, Ambiguous};
use std::collections::HashMap;

// ---------------------------------------------------------------- elaboration IR

#[derive(Clone, Debug)]
enum ECore {
    /// A raw `i64` word constant (machine scalar values).
    Int(i64),
    /// A mathematical `Nat`/`Int` constant (decimal digit string with an
    /// optional leading `-`; `nat` selects the `Nat` runtime).
    ConstBig { nat: bool, digits: String },
    /// A deferred integer literal ([`Checker::literal_pends`]): its
    /// constant form is fixed once the literal's type is determined, then
    /// it lowers to `Int` or `ConstBig`.
    Lit(usize),
    Bool(bool),
    Unit,
    Var(LocalId),
    Global(GlobalId),
    Ctor(GlobalId),
    Prim(PrimOp),
    DictProj(Box<ECore>, usize),
    /// Record field projection: load field `idx` from the record block.
    Proj(Box<ECore>, usize),
    InstDict(GlobalId, Vec<ECore>),
    App(Box<ECore>, Vec<ECore>),
    /// Un-hoisted lambda: `params` are its pattern parameters.
    Lam { params: Vec<LocalId>, body: Box<ECore> },
    /// Hoisted form after [`hoist_lambdas`].
    MkClos { fn_id: GlobalId, env: Vec<ECore> },
    Let(LocalId, Box<ECore>, Box<ECore>),
    If(Box<ECore>, Box<ECore>, Box<ECore>),
    Case { scrut: Box<ECore>, alts: Vec<(CPat, ECore)> },
    /// Demand a memo cell (its state machine runs at code generation).
    Demand(Box<ECore>),
    /// Allocate an empty memo cell.
    LazyAlloc,
    /// Store a thunk into a memo cell.
    LazyInit(LocalId, Box<ECore>),
    /// The runtime fault path (exit 70): emitted when a match falls through
    /// with pattern coverage unprovable.
    Fault,
    /// Unresolved dictionary, filled before lowering.
    Hole(usize),
}

#[derive(Clone, Debug)]
enum DictExpr {
    Binder(LocalId),
    Instance(GlobalId, Vec<DictExpr>),
    /// Structural evidence for the compiler-known `Copy` class: a marker
    /// class has no methods, so its dictionary is a plain unit word.
    Marker,
}

/// One want's resolution: determined now, or several instances still match
/// and the fixpoint must retry after the other wants commit.
enum SolveOutcome {
    Solved(DictExpr),
    Ambiguous,
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
        DictExpr::Instance(id, subs) => {
            ECore::InstDict(id.clone(), subs.iter().map(dict_expr_ecore).collect())
        }
        DictExpr::Marker => ECore::Unit,
    }
}

fn free_locals(e: &ECore, bound: &mut Vec<LocalId>, out: &mut Vec<LocalId>) {
    match e {
        ECore::Var(l) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
        }
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Hole(_) | ECore::Fault => {}
        ECore::DictProj(d, _) => free_locals(d, bound, out),
        ECore::Proj(d, _) => free_locals(d, bound, out),
        ECore::InstDict(_, args) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
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
        ECore::Let(l, v, body) => {
            free_locals(v, bound, out);
            let n = bound.len();
            bound.push(*l);
            free_locals(body, bound, out);
            bound.truncate(n);
        }
        ECore::If(c, t, f) => {
            free_locals(c, bound, out);
            free_locals(t, bound, out);
            free_locals(f, bound, out);
        }
        ECore::Demand(d) => free_locals(d, bound, out),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(t, bound, out);
        }
        ECore::Case { scrut, alts } => {
            free_locals(scrut, bound, out);
            for (pat, body) in alts {
                let n = bound.len();
                match pat {
                    CPat::Var(l) => bound.push(*l),
                    CPat::Con { fields, .. } => bound.extend(fields.iter().copied()),
                    CPat::Wild => {}
                }
                free_locals(body, bound, out);
                bound.truncate(n);
            }
        }
    }
}

/// A lambda hoisted to a top level function; finalized by [`finalize_fn`].
struct HoistedFn {
    id: GlobalId,
    /// `env ++ lambda parameters`, in ABI order.
    params: Vec<LocalId>,
    body: ECore,
}

/// Replace lambda nodes with `MkClos` over hoisted functions.
fn hoist_lambdas(e: &mut ECore, hint: &str, counter: &mut usize, fns: &mut Vec<HoistedFn>) {
    match e {
        ECore::Int(_)
        | ECore::ConstBig { .. }
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_)
        | ECore::Hole(_)
        | ECore::Fault => {}
        ECore::DictProj(d, _) => hoist_lambdas(d, hint, counter, fns),
        ECore::Proj(d, _) => hoist_lambdas(d, hint, counter, fns),
        ECore::InstDict(_, args) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                hoist_lambdas(a, hint, counter, fns);
            }
            if let ECore::App(f, _) = e {
                hoist_lambdas(f, hint, counter, fns);
            }
        }
        ECore::Let(_, v, body) => {
            hoist_lambdas(v, hint, counter, fns);
            hoist_lambdas(body, hint, counter, fns);
        }
        ECore::If(c, t, f) => {
            hoist_lambdas(c, hint, counter, fns);
            hoist_lambdas(t, hint, counter, fns);
            hoist_lambdas(f, hint, counter, fns);
        }
        ECore::Demand(d) => hoist_lambdas(d, hint, counter, fns),
        ECore::LazyAlloc => {}
        ECore::LazyInit(_, t) => hoist_lambdas(t, hint, counter, fns),
        ECore::Case { scrut, alts } => {
            hoist_lambdas(scrut, hint, counter, fns);
            for (_, body) in alts {
                hoist_lambdas(body, hint, counter, fns);
            }
        }
        ECore::Lam { params, body } => {
            hoist_lambdas(body, hint, counter, fns);
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
            });
            *e = ECore::MkClos {
                fn_id: id,
                env: env.into_iter().map(ECore::Var).collect(),
            };
        }
    }
}

/// Renumber the locals of a function body so that its parameters are
/// `LocalId(0..arity)` in ABI order and every other local follows in
/// traversal order. Codegen relies on this convention.
fn renumber_fn(body: &ECore, params: &[LocalId]) -> ECore {
    let mut map: HashMap<LocalId, LocalId> = HashMap::new();
    for (i, p) in params.iter().enumerate() {
        map.insert(*p, LocalId(i));
    }
    let mut next = params.len();
    assign_locals(body, &mut map, &mut next);
    rewrite_locals(body, &map)
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
        ECore::Var(l) => note_local(*l, map, next),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Hole(_) | ECore::Fault => {}
        ECore::DictProj(d, _) => assign_locals(d, map, next),
        ECore::Proj(d, _) => assign_locals(d, map, next),
        ECore::InstDict(_, args) | ECore::App(_, args) | ECore::MkClos { env: args, .. } => {
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
        ECore::Let(l, v, body) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
            assign_locals(body, map, next);
        }
        ECore::If(c, t, f) => {
            assign_locals(c, map, next);
            assign_locals(t, map, next);
            assign_locals(f, map, next);
        }
        ECore::Demand(d) => assign_locals(d, map, next),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            note_local(*l, map, next);
            assign_locals(t, map, next);
        }
        ECore::Case { scrut, alts } => {
            assign_locals(scrut, map, next);
            for (pat, body) in alts {
                match pat {
                    CPat::Var(l) => note_local(*l, map, next),
                    CPat::Con { fields, .. } => {
                        for f in fields {
                            note_local(*f, map, next);
                        }
                    }
                    CPat::Wild => {}
                }
                assign_locals(body, map, next);
            }
        }
    }
}

fn rewrite_locals(e: &ECore, map: &HashMap<LocalId, LocalId>) -> ECore {
    let m = |l: &LocalId| *map.get(l).unwrap_or(l);
    match e {
        ECore::Var(l) => ECore::Var(m(l)),
        ECore::Int(v) => ECore::Int(*v),
        ECore::ConstBig { nat, digits } => ECore::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::Lit(i) => ECore::Lit(*i),
        ECore::Bool(b) => ECore::Bool(*b),
        ECore::Unit => ECore::Unit,
        ECore::Global(g) => ECore::Global(g.clone()),
        ECore::Ctor(g) => ECore::Ctor(g.clone()),
        ECore::Prim(p) => ECore::Prim(*p),
        ECore::Hole(h) => ECore::Hole(*h),
        ECore::Fault => ECore::Fault,
        ECore::DictProj(d, i) => ECore::DictProj(Box::new(rewrite_locals(d, map)), *i),
        ECore::Proj(d, i) => ECore::Proj(Box::new(rewrite_locals(d, map)), *i),
        ECore::InstDict(g, args) => ECore::InstDict(
            g.clone(),
            args.iter().map(|a| rewrite_locals(a, map)).collect(),
        ),
        ECore::App(f, args) => ECore::App(
            Box::new(rewrite_locals(f, map)),
            args.iter().map(|a| rewrite_locals(a, map)).collect(),
        ),
        ECore::Lam { params, body } => ECore::Lam {
            params: params.iter().map(m).collect(),
            body: Box::new(rewrite_locals(body, map)),
        },
        ECore::MkClos { fn_id, env } => ECore::MkClos {
            fn_id: fn_id.clone(),
            env: env.iter().map(|a| rewrite_locals(a, map)).collect(),
        },
        ECore::Let(l, v, body) => ECore::Let(
            m(l),
            Box::new(rewrite_locals(v, map)),
            Box::new(rewrite_locals(body, map)),
        ),
        ECore::If(c, t, f) => ECore::If(
            Box::new(rewrite_locals(c, map)),
            Box::new(rewrite_locals(t, map)),
            Box::new(rewrite_locals(f, map)),
        ),
        ECore::Demand(d) => ECore::Demand(Box::new(rewrite_locals(d, map))),
        ECore::LazyAlloc => ECore::LazyAlloc,
        ECore::LazyInit(l, t) => {
            ECore::LazyInit(m(l), Box::new(rewrite_locals(t, map)))
        }
        ECore::Case { scrut, alts } => ECore::Case {
            scrut: Box::new(rewrite_locals(scrut, map)),
            alts: alts
                .iter()
                .map(|(p, b)| {
                    let p = match p {
                        CPat::Var(l) => CPat::Var(m(l)),
                        CPat::Con { tag, fields } => {
                            CPat::Con { tag: *tag, fields: fields.iter().map(m).collect() }
                        }
                        CPat::Wild => CPat::Wild,
                    };
                    (p, rewrite_locals(b, map))
                })
                .collect(),
        },
    }
}

/// Replace deferred literal nodes by their resolved constants.
fn resolve_lit_words(e: &mut ECore, lits: &[ECore]) {
    match e {
        ECore::Lit(i) => *e = lits[*i].clone(),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::Bool(_) | ECore::Unit | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Hole(_) | ECore::Fault | ECore::LazyAlloc => {}
        ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) => {
            resolve_lit_words(d, lits)
        }
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
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
        ECore::Lam { body, .. } => resolve_lit_words(body, lits),
        ECore::Let(_, v, body) => {
            resolve_lit_words(v, lits);
            resolve_lit_words(body, lits);
        }
        ECore::If(c, t, f) => {
            resolve_lit_words(c, lits);
            resolve_lit_words(t, lits);
            resolve_lit_words(f, lits);
        }
        ECore::LazyInit(_, t) => resolve_lit_words(t, lits),
        ECore::Case { scrut, alts } => {
            resolve_lit_words(scrut, lits);
            for (_, b) in alts {
                resolve_lit_words(b, lits);
            }
        }
    }
}

fn finalize_fn(
    id: GlobalId,
    params: Vec<LocalId>,
    body: ECore,
    prim: Option<PrimOp>,
    ctor: Option<(usize, usize)>,
) -> FnDef {
    let body = renumber_fn(&body, &params);
    FnDef {
        id,
        params: params.iter().map(|l| format!("v{}", l.0)).collect(),
        body: lower(&body),
        prim,
        ctor,
    }
}

fn lower(e: &ECore) -> crate::core::Core {
    match e {
        ECore::Int(v) => crate::core::Core::Int(*v),
        ECore::ConstBig { nat, digits } => crate::core::Core::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::Lit(_) => {
            unreachable!("deferred literals are resolved before lowering")
        }
        ECore::Bool(b) => crate::core::Core::Bool(*b),
        ECore::Unit => crate::core::Core::Unit,
        ECore::Var(l) => crate::core::Core::Var(*l),
        ECore::Global(g) => crate::core::Core::Global(g.clone()),
        ECore::Ctor(g) => crate::core::Core::Ctor(g.clone()),
        ECore::Prim(p) => crate::core::Core::Prim(*p),
        ECore::DictProj(d, i) => crate::core::Core::DictProj(Box::new(lower(d)), *i),
        ECore::Proj(d, i) => crate::core::Core::Proj(Box::new(lower(d)), *i),
        ECore::InstDict(g, args) => {
            crate::core::Core::InstDict(g.clone(), args.iter().map(lower).collect())
        }
        ECore::App(f, args) => {
            crate::core::Core::App(Box::new(lower(f)), args.iter().map(lower).collect())
        }
        ECore::Lam { .. } | ECore::Hole(_) => {
            unreachable!("lambda hoisting and dictionary solving run before lowering")
        }
        ECore::Fault => crate::core::Core::Fault,
        ECore::MkClos { fn_id, env } => crate::core::Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(lower).collect(),
        },
        ECore::Let(l, v, body) => {
            crate::core::Core::Let(*l, Box::new(lower(v)), Box::new(lower(body)))
        }
        ECore::If(c, t, f) => {
            crate::core::Core::If(Box::new(lower(c)), Box::new(lower(t)), Box::new(lower(f)))
        }
        ECore::Demand(d) => crate::core::Core::Demand(Box::new(lower(d))),
        ECore::LazyAlloc => crate::core::Core::LazyAlloc,
        ECore::LazyInit(l, t) => {
            crate::core::Core::LazyInit(*l, Box::new(lower(t)))
        }
        ECore::Case { scrut, alts } => crate::core::Core::Case {
            scrut: Box::new(lower(scrut)),
            alts: alts.iter().map(|(p, b)| (p.clone(), lower(b))).collect(),
        },
    }
}

fn app_spine(head: ECore, args: Vec<ECore>) -> ECore {
    match head {
        ECore::App(f, mut existing) => {
            existing.extend(args);
            ECore::App(f, existing)
        }
        other => ECore::App(Box::new(other), args),
    }
}

// ---------------------------------------------------------------- info tables

#[derive(Clone)]
enum ValueKind {
    TopFn,
    Prim(PrimOp),
    /// Class method at position `idx` of its signature.
    Method { idx: usize },
    /// Data constructor: tag in declaration order, field count, field types
    /// and the data type's parameter names (for substitution at patterns).
    Ctor {
        tag: usize,
        arity: usize,
        field_tys: Vec<Ty>,
        ty_params: Vec<String>,
    },
    BoolLit(bool),
}

#[derive(Clone)]
struct ValueInfo {
    module: String,
    name: String,
    scheme: Scheme,
    kind: ValueKind,
}

#[derive(Clone)]
struct TypeKind {
    module: String,
    name: String,
    arity: usize,
    /// Constructor names with tags, in declaration order.
    ctors: Vec<(String, usize)>,
    /// For a `record` type: the declared field names in declaration order.
    /// `None` for `data` types.
    fields: Option<Vec<String>>,
}

#[derive(Clone)]
struct SigInfo {
    module: String,
    name: String,
    params: Vec<String>,
    /// Method name, method scheme; the class constraint is `ctx[0]`.
    methods: Vec<(String, Scheme)>,
}

struct InstanceInfo {
    module: String,
    name: String,
    params: Vec<String>,
    ctx: Vec<(String, ClassApp)>,
    target: ClassApp,
    /// The dictionary construction unit for this instance.
    base_id: GlobalId,
    implicit: bool,
}

struct Want {
    id: usize,
    class: ClassApp,
    span: ast::Span,
    scope: Vec<(LocalId, ClassApp)>,
}

/// A deferred integer literal (primitive types specification, section 7):
/// it elaborates against its expected type, which may only become known
/// after unification and instance resolution, so the constant is fixed by
/// [`Checker::resolve_literals`] once the definition is complete.
struct PendLit {
    /// The literal's type metavariable.
    ty: Ty,
    /// The decimal digit string, with an optional leading `-`.
    value: String,
    span: ast::Span,
}

#[derive(Clone, Default)]
struct Env {
    locals: Vec<(String, LocalId, Scheme)>,
    dicts: Vec<(LocalId, ClassApp)>,
}

impl Env {
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
}

/// Region names a type mentions in `Ref`/`Mut` borrow positions, with
/// whether each is an exclusive borrow.
fn borrow_regions(ty: &Ty, out: &mut Vec<(String, bool)>) {
    match ty {
        Ty::Ref(r, inner) => {
            out.push((r.clone(), false));
            borrow_regions(inner, out);
        }
        Ty::Mut(r, inner) => {
            out.push((r.clone(), true));
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
        Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } => borrow_regions(inner, out),
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
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                escape_regions(a, out);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------- the checker

struct Checker {
    uni: Unifier,
    values: HashMap<String, Vec<ValueInfo>>,
    qualified: HashMap<(String, String), ValueInfo>,
    types: HashMap<String, Vec<TypeKind>>,
    qualified_types: HashMap<(String, String), TypeKind>,
    sigs: HashMap<String, Vec<SigInfo>>,
    qualified_sigs: HashMap<(String, String), SigInfo>,
    instances: HashMap<String, Vec<usize>>,
    instance_list: Vec<InstanceInfo>,
    fns: Vec<FnDef>,
    lam_counter: usize,
    next_local: usize,
    param_counter: usize,
    want_counter: usize,
    wants: Vec<Want>,
    /// Module currently being checked, used for unqualified resolution.
    current_module: String,
    /// Modules whose exports are in scope unqualified.
    visible: Vec<String>,
    /// Top-level signature schemes, keyed by `(module, name)`.
    top_schemes: HashMap<(String, String), Scheme>,
    /// Top-level equations, keyed by module and name.
    top_eqs: Vec<(String, String, ast::Equation)>,
    /// Finished instance dictionaries.
    instance_defs: Vec<InstanceDef>,
    /// Instances awaiting body checking, keyed by module.
    instance_defs_pending: Vec<(String, ast::InstDecl)>,
    /// The instance whose method bodies are being checked: visible to
    /// resolution even when it is not `implicit`.
    self_instance: Option<usize>,
    /// Effects performed in the current checking scope.
    perf: EffRow,
    /// Fresh-name counter for memo regions.
    region_counter: usize,
    /// The prelude module, home of `List`, `Cons` and `Nil` for list
    /// literal and list pattern desugaring.
    prelude_module: String,
    /// Tuple constructor code generation units already synthesized, by
    /// arity.
    tuple_ctors: Vec<usize>,
    /// Integer literals awaiting type determination, in creation order.
    literal_pends: Vec<PendLit>,
    /// Each module's imported module names (interface metadata).
    module_imports: HashMap<String, Vec<String>>,
    /// Each module's own fixity declarations, for its interface.
    module_fixities: Vec<(String, crate::interface::FixityExport)>,
    /// Each module's foreign declarations, for its interface.
    module_foreign: Vec<(String, crate::interface::ForeignExport)>,
    /// Use counts accumulated for the definition being checked.
    uses: HashMap<LocalId, UseNode>,
    /// Usage obligations of the definition's binders.
    obligations: HashMap<LocalId, Obligation>,
    /// Lazy-position nesting: lambda and thunk bodies.
    lam_depth: usize,
    /// Lazy-position nesting: memo cell bodies.
    memo_depth: usize,
    /// Memo-group regions currently in scope (region escape checking).
    active_regions: Vec<String>,
    /// Record-projection path accumulated for a `Proj` operand place.
    place_path: Vec<String>,
}

fn fn_id(module: &str, name: &str) -> GlobalId {
    GlobalId(format!("{module}.{name}"))
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
                arg: l.arg.as_ref().map(|a| regions.get(a).cloned().unwrap_or_else(|| a.clone())),
            })
            .collect(),
        tail: match &r.tail {
            RowTail::Rigid(t) => rows.get(t).cloned().unwrap_or_else(|| RowTail::Rigid(t.clone())),
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
            qualified_sigs: HashMap::new(),
            instances: HashMap::new(),
            instance_list: Vec::new(),
            fns: Vec::new(),
            lam_counter: 0,
            next_local: 0,
            param_counter: 0,
            want_counter: 0,
            wants: Vec::new(),
            current_module: String::new(),
            visible: Vec::new(),
            top_schemes: HashMap::new(),
            top_eqs: Vec::new(),
            instance_defs: Vec::new(),
            instance_defs_pending: Vec::new(),
            self_instance: None,
            perf: EffRow::pure(),
            region_counter: 0,
            prelude_module: String::new(),
            tuple_ctors: Vec::new(),
            literal_pends: Vec::new(),
            module_imports: HashMap::new(),
            module_fixities: Vec::new(),
            module_foreign: Vec::new(),
            uses: HashMap::new(),
            obligations: HashMap::new(),
            lam_depth: 0,
            memo_depth: 0,
            active_regions: Vec::new(),
            place_path: Vec::new(),
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
    fn fresh_region(&mut self) -> String {
        let n = self.region_counter;
        self.region_counter += 1;
        format!("r#{n}")
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
                format!(
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
        self.lam_depth = 0;
        self.memo_depth = 0;
        self.active_regions.clear();
        self.place_path.clear();
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
        env.locals.push((name.to_string(), l, scheme));
        Ok(())
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
                },
            );
        }
    }

    /// Record one use of a local at a record-projection place. Uses under
    /// a lambda body are unbounded (the closure may be called any number
    /// of times); uses under a memo cell body happen at most once (the
    /// cell evaluates at most once).
    fn note_use(&mut self, l: LocalId, span: ast::Span, path: &[String]) -> Result<()> {
        let Some(ob) = self.obligations.get(&l) else {
            return Ok(());
        };
        let name = ob.name.clone();
        let ty = ob.ty.clone();
        let (lam, memo) = (ob.lam, ob.memo);
        if self.lam_depth > lam {
            // A callback cannot outlive a borrowed environment (core
            // specification, section 7). Capture-based ownership transfer
            // needs closure usage inference, which this slice does not
            // have, so capturing a borrow or memo cell of a live region is
            // rejected conservatively.
            let t = self.uni.zonk(&ty);
            let mut regions = Vec::new();
            escape_regions(&t, &mut regions);
            if let Some(r) = regions.into_iter().find(|r| self.active_regions.contains(r)) {
                return Err(Error::at(
                    span,
                    format!(
                        "region escape: {name} is captured in a lambda that may outlive region {r}"
                    ),
                ));
            }
        }
        let c = if self.lam_depth > lam {
            Count::UNBOUNDED
        } else if self.memo_depth > memo {
            Count::AT_MOST_ONCE
        } else {
            Count::ONE
        };
        self.uses.entry(l).or_default().add(path, c);
        Ok(())
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
        let mut binders: Vec<(&LocalId, &Obligation)> = self.obligations.iter().collect();
        binders.sort_by_key(|(l, _)| l.0);
        for (l, ob) in binders {
            let c = self
                .uses
                .get(l)
                .map(|n| n.consumptions())
                .unwrap_or(Count::ZERO);
            let mult = match ob.mult {
                Some(m) if m != Mult::Flex => m,
                _ => default_mult(&self.uni.zonk(&ob.ty)),
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
                            format!(
                                "multiplicity %1: {} must be used exactly once on every path, but is {}",
                                ob.name,
                                c.describe()
                            ),
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

    /// The effective multiplicity of a binder: its declared class, or the
    /// type-directed default once the type is determined.
    fn effective_mult(&self, l: LocalId) -> Option<Mult> {
        let ob = self.obligations.get(&l)?;
        Some(match ob.mult {
            Some(m) if m != Mult::Flex => m,
            _ => default_mult(&self.uni.zonk(&ob.ty)),
        })
    }

    /// Structural `Copy` evidence (ownership and usage design, section 3):
    /// machine scalars and the mathematical types, immutable products of
    /// `Copy` parts, and shared borrows. `Bytes` is a mutable buffer;
    /// closures, thunks, memo cells and `Mut` borrows are not duplicable.
    fn copy_structural(
        &self,
        ty: &Ty,
        scope: &[(LocalId, ClassApp)],
        seen: &mut Vec<(String, Vec<Ty>)>,
    ) -> bool {
        match ty {
            Ty::Int | Ty::Nat | Ty::Bool | Ty::Unit | Ty::Prim(_) | Ty::Ref(_, _) => true,
            Ty::Bytes | Ty::Meta(_) | Ty::Mut(_, _) | Ty::Thunk { .. } | Ty::Lazy { .. } => false,
            Ty::Fun(_, _, _, _) => false,
            Ty::Param(p) => scope.iter().any(|(_, c)| {
                c.class == "Copy" && c.args.iter().any(|a| matches!(a, Ty::Param(q) if q == p))
            }),
            Ty::Tuple(items) => items
                .iter()
                .all(|i| self.copy_structural(i, scope, seen)),
            Ty::Adt(name, args) => {
                if seen.iter().any(|(n, a)| n == name && a == args) {
                    return true;
                }
                seen.push((name.clone(), args.clone()));
                let info = match self.lookup_type(&[name]) {
                    Ok(i) => i,
                    Err(_) => return false,
                };
                for (cname, _) in &info.ctors {
                    let v = match self.ctor_of_type(&info, cname) {
                        Ok(v) => v,
                        Err(_) => return false,
                    };
                    let fields = match self.ctor_field_tys(&v, args) {
                        Ok((_, f)) => f,
                        Err(_) => return false,
                    };
                    if !fields.iter().all(|f| self.copy_structural(f, scope, seen)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn fresh_local(&mut self) -> LocalId {
        let l = LocalId(self.next_local);
        self.next_local += 1;
        l
    }

    fn fresh_param(&mut self, base: &str) -> String {
        let n = self.param_counter;
        self.param_counter += 1;
        format!("{base}#{n}")
    }

    // ------------------------------------------------------------ literals

    /// Elaborate an integer literal against its expected type (primitive
    /// types specification, section 7). A known expected type materialises
    /// the literal immediately; a metavariable defers the constant choice
    /// to [`Checker::resolve_literals`].
    fn lit_check(&mut self, expected: &Ty, digits: &str, span: ast::Span) -> Result<ECore> {
        match self.uni.zonk(expected) {
            Ty::Meta(m) => {
                let idx = self.literal_pends.len();
                self.literal_pends.push(PendLit {
                    ty: Ty::Meta(m),
                    value: digits.to_string(),
                    span,
                });
                Ok(ECore::Lit(idx))
            }
            t => self.literal_const(&t, digits, span),
        }
    }

    /// The constant of literal `digits` at type `ty`. `Nat` and `Int` are
    /// mathematical: the digits become a boxed bignum materialised through
    /// the runtime, and a `Nat` literal must not be negative. Every other
    /// type range-checks to a canonical `i64` word (section 7: out-of-range
    /// literals are compile errors).
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
            t => Ok(ECore::Int(self.literal_word(t, digits, span)?)),
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
                let w = s.width();
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
                format!("integer literal cannot have type {other}"),
            )),
        }
    }

    /// The metavariables of the literals still awaiting determination; they
    /// stay monomorphic through let-generalization (one literal has one
    /// type, section 7).
    fn literal_metas(&self) -> Vec<EitherMeta> {
        self.literal_pends
            .iter()
            .filter_map(|p| match &p.ty {
                Ty::Meta(m) => Some(EitherMeta::Type(*m)),
                _ => None,
            })
            .collect()
    }

    /// Fix every deferred literal's word once its type is determined
    /// (section 7: a literal whose type is not determined by context is an
    /// error in this draft).
    fn resolve_literals(&mut self, body: &mut ECore) -> Result<()> {
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
        resolve_lit_words(body, &lits);
        Ok(())
    }

    // ------------------------------------------------------------ resolution

    fn lookup_value(&self, module: Option<&str>, name: &str) -> Result<ValueInfo> {
        if let Some(m) = module {
            return self
                .qualified
                .get(&(m.to_string(), name.to_string()))
                .cloned()
                .ok_or_else(|| Error::plain(format!("unknown name {m}.{name}")));
        }
        for m in &self.visible {
            if let Some(v) = self
                .values
                .get(name)
                .and_then(|vs| vs.iter().find(|v| &v.module == m))
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

    fn lookup_type(&self, parts: &[&str]) -> Result<TypeKind> {
        match parts {
            [name] => self
                .types
                .get(*name)
                .and_then(|ts| {
                    ts.iter()
                        .find(|t| t.module == "<builtin>" || self.visible.contains(&t.module))
                })
                .cloned()
                .ok_or_else(|| Error::plain(format!("unknown type {name}"))),
            [m, name] => self
                .qualified_types
                .get(&(m.to_string(), name.to_string()))
                .cloned()
                .ok_or_else(|| Error::plain(format!("unknown type {m}.{name}"))),
            _ => Err(Error::plain("type paths deeper than Module.Type are not supported")),
        }
    }

    fn lookup_sig(&self, path: &ast::ConPath) -> Result<SigInfo> {
        let parts = path.parts();
        match parts.as_slice() {
            [name] => {
                // `Copy` (the design's `Share` class) is compiler-known
                // with structural instances: immutable values that may be
                // duplicated and dropped freely (primitive types
                // specification, section 3; ownership and usage design,
                // section 3). It is not user-extensible.
                if matches!(*name, "Copy" | "Share") {
                    return Ok(SigInfo {
                        module: "<builtin>".into(),
                        name: "Copy".into(),
                        params: vec!["a".into()],
                        methods: Vec::new(),
                    });
                }
                self.sigs
                    .get(*name)
                    .and_then(|ss| ss.iter().find(|s| self.visible.contains(&s.module)))
                    .cloned()
                    .ok_or_else(|| Error::plain(format!("unknown signature {name}")))
            }
            [m, name] => self
                .qualified_sigs
                .get(&(m.to_string(), name.to_string()))
                .cloned()
                .ok_or_else(|| Error::plain(format!("unknown signature {m}.{name}"))),
            _ => Err(Error::plain("signature paths deeper than Module.Sig are not supported")),
        }
    }

    fn resolve_var(&self, p: &ast::VarPath, span: ast::Span) -> Result<ValueInfo> {
        let mut v = match p.quals.as_slice() {
            [] => self.lookup_value(None, &p.name.text).map_err(|e| Error::at(span, e.message))?,
            [q] => self
                .lookup_value(Some(&q.text), &p.name.text)
                .map_err(|e| Error::at(span, e.message))?,
            _ => {
                return Err(Error::at(span, "qualified names deeper than Module.name are not supported"));
            }
        };
        v.name = p.name.text.clone();
        Ok(v)
    }

    fn resolve_con(&self, p: &ast::ConPath, span: ast::Span) -> Result<ValueInfo> {
        let parts = p.parts();
        let (module, name) = match parts.as_slice() {
            [name] => {
                let v = self.lookup_value(None, name)?;
                (v.module.clone(), name.to_string())
            }
            [q, name] => {
                // Module-qualified, or type-qualified as in `Option.None`.
                if self
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
                if !(t.module == "<builtin>" || self.visible.contains(&t.module)) {
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
        if !self.tuple_ctors.contains(&n) {
            self.tuple_ctors.push(n);
            self.fns.push(FnDef {
                id: GlobalId(format!("Tuple.{n}")),
                params: (0..n).map(|i| format!("f{i}")).collect(),
                body: crate::core::Core::Unit,
                prim: None,
                ctor: Some((0, n)),
            });
        }
        Ok(GlobalId(format!("Tuple.{n}")))
    }

    // ------------------------------------------------------------ type conversion

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
        let args = c
            .args
            .iter()
            .map(|a| self.conv_type(a, vars))
            .collect::<Result<Vec<_>>>()?;
        Ok((
            c.name.text.clone(),
            ClassApp { class: sig.name.clone(), args },
        ))
    }

    fn conv_type(&self, ty: &ast::Type, vars: &[String]) -> Result<Ty> {
        let t = self.conv_raw(ty, vars)?;
        self.check_applied(&t, ty.span())?;
        Ok(t)
    }

    fn check_applied(&self, t: &Ty, span: ast::Span) -> Result<()> {
        if let Ty::Adt(name, args) = t {
            let info = self.lookup_type(&[name])?;
            if args.len() != info.arity {
                return Err(Error::at(
                    span,
                    format!("type {name} must be fully applied (expects {} arguments)", info.arity),
                ));
            }
        }
        Ok(())
    }

    fn type_app(&self, f: Ty, a: Ty, span: ast::Span) -> Result<Ty> {
        match f {
            Ty::Adt(name, mut args) => {
                args.push(a);
                Ok(Ty::Adt(name, args))
            }
            other => Err(Error::at(span, format!("cannot apply non-type-constructor {other}"))),
        }
    }

    /// Convert an effect row. Label arguments are region names; the tail is
    /// a rigid row variable from the enclosing `forall`.
    fn conv_row(&self, e: &ast::EffectRow, vars: &[String]) -> Result<EffRow> {
        let mut labels = Vec::new();
        for (path, args) in &e.labels {
            let parts = path.parts();
            let head = parts[0].to_string();
            if parts.len() != 1 {
                return Err(Error::at(
                    path.span(),
                    "effect label paths deeper than one segment are not supported",
                ));
            }
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
            labels.push(EffLabel { head, arg });
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
                RowTail::Rigid(n.text.clone())
            }
        };
        Ok(EffRow { labels, tail })
    }

    /// Convert a possibly applied type. `Thunk {e} a` and `Lazy {e} r a`
    /// are builtin type constructors taking an effect-row argument first.
    fn conv_spine(&self, ty: &ast::Type, vars: &[String]) -> Result<Ty> {
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
                        region,
                        inner: Box::new(self.conv_type(a, vars)?),
                    });
                }
                (["Thunk"] | ["Lazy"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Thunk and Lazy must be fully applied as `Thunk {e} a` and `Lazy {e} r a`",
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
                        Ty::Ref(region, Box::new(inner))
                    } else {
                        Ty::Mut(region, Box::new(inner))
                    });
                }
                (["Ref"] | ["Mut"], _) => {
                    return Err(Error::at(
                        head_span,
                        "Ref and Mut must be fully applied as `Ref r a` and `Mut r a`",
                    ));
                }
                _ => {}
            }
        }
        let mut result = self.conv_raw(cur, vars)?;
        for a in args {
            match a {
                Arg::Ty(t) => {
                    let t = self.conv_type(&t, vars)?;
                    result = self.type_app(result, t, head_span)?;
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

    fn conv_raw(&self, ty: &ast::Type, vars: &[String]) -> Result<Ty> {
        match ty {
            ast::Type::Con(p, span) => {
                let parts = p.parts();
                match parts.as_slice() {
                    ["Nat"] => Ok(Ty::Nat),
                    ["Int"] => Ok(Ty::Int),
                    ["Bool"] => Ok(Ty::Bool),
                    ["Unit"] => Ok(Ty::Unit),
                    ["Bytes"] => Ok(Ty::Bytes),
                    ["Thunk"] | ["Lazy"] | ["Ref"] | ["Mut"] => Err(Error::at(
                        *span,
                        "Thunk, Lazy, Ref and Mut must be fully applied (`Thunk {e} a`, `Lazy {e} r a`, `Ref r a`, `Mut r a`)",
                    )),
                    [name] => {
                        if let Some(s) = Scalar::parse(name) {
                            return Ok(Ty::Prim(s));
                        }
                        let info = self.lookup_type(&parts).map_err(|_| {
                            Error::at(*span, format!("unknown type {}", parts.join(".")))
                        })?;
                        // Full application is enforced by `conv_type`; here a
                        // bare constructor may still collect arguments.
                        Ok(Ty::Adt(info.name, Vec::new()))
                    }
                    _ => {
                        let info = self.lookup_type(&parts).map_err(|_| {
                            Error::at(*span, format!("unknown type {}", parts.join(".")))
                        })?;
                        // Full application is enforced by `conv_type`; here a
                        // bare constructor may still collect arguments.
                        Ok(Ty::Adt(info.name, Vec::new()))
                    }
                }
            }
            ast::Type::Var(n, _) => {
                if vars.contains(&n.text) {
                    Ok(Ty::Param(n.text.clone()))
                } else {
                    Err(Error::at(n.span, format!("unknown type variable {}", n.text)))
                }
            }
            ast::Type::App(_, _, _) | ast::Type::AppRow(_, _, _) => self.conv_spine(ty, vars),
            ast::Type::Fun { arg, mult, eff, res, span } => {
                let arg_ty = self.conv_type(arg, vars)?;
                let m = match mult {
                    Some(n) => Mult::parse(&n.text)
                        .ok_or_else(|| Error::at(n.span, format!("unknown multiplicity {}", n.text)))?,
                    None => default_mult(&arg_ty),
                };
                let eff = match eff {
                    Some(e) => self.conv_row(e, vars)?,
                    None => EffRow::pure(),
                };
                let _ = span;
                Ok(Ty::fun_mult(arg_ty, m, eff, self.conv_type(res, vars)?))
            }
            ast::Type::Forall(_, _, span) => Err(Error::at(
                *span,
                "unexpected forall inside a type; quantifiers are only supported at the outermost position of a signature",
            )),
            ast::Type::Using(_, _, span) => Err(Error::at(
                *span,
                "using constraints are only supported at the outermost position of a signature",
            )),
            ast::Type::DepFun(_, _, _, span) | ast::Type::Sigma(_, _, _, span) => Err(Error::at(
                *span,
                "dependent function and pair types are not supported in the bootstrap slice",
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

    fn conv_scheme(&self, ty: &ast::Type) -> Result<Scheme> {
        let mut params: Vec<String> = Vec::new();
        let mut ctx: Vec<(String, ClassApp)> = Vec::new();
        let mut rest = ty;
        loop {
            match rest {
                ast::Type::Forall(binders, body, _) => {
                    for b in binders {
                        if let Some(ann) = &b.ann {
                            let is_type = matches!(ann, ast::Type::Con(p, _) if p.parts() == ["Type"]);
                            if !is_type {
                                return Err(Error::at(
                                    b.name.span,
                                    "kinds other than Type are not supported in the bootstrap slice",
                                ));
                            }
                        }
                        if params.contains(&b.name.text) {
                            return Err(Error::at(b.name.span, "duplicate type parameter"));
                        }
                        params.push(b.name.text.clone());
                    }
                    rest = body;
                }
                ast::Type::Using(cs, body, _) => {
                    for c in cs {
                        let (name, class) = self.conv_class(c, &params)?;
                        ctx.push((name, class));
                    }
                    rest = body;
                }
                other => {
                    let t = self.conv_type(other, &params)?;
                    // Names used in tail position are row variables; names
                    // used in region position are region variables; the rest
                    // are ordinary type parameters.
                    let mut row_params: Vec<String> = Vec::new();
                    let mut region_params: Vec<String> = Vec::new();
                    collect_row_names(&t, &mut row_params, &mut region_params);
                    let type_params: Vec<String> = params
                        .into_iter()
                        .filter(|p| !row_params.contains(p) && !region_params.contains(p))
                        .collect();
                    return Ok(Scheme {
                        params: type_params,
                        row_params,
                        region_params,
                        ctx,
                        ty: t,
                    });
                }
            }
        }
    }

    fn subst_ty(t: &Ty, map: &HashMap<String, Ty>) -> Ty {
        Checker::subst_full(t, map, &HashMap::new(), &HashMap::new())
    }

    /// Substitute type parameters (by type), row variables (by tail) and
    /// region names (by fresh name).
    fn subst_full(
        t: &Ty,
        types: &HashMap<String, Ty>,
        rows: &HashMap<String, RowTail>,
        regions: &HashMap<String, String>,
    ) -> Ty {
        match t {
            Ty::Param(p) => types.get(p).cloned().unwrap_or_else(|| t.clone()),
            Ty::Adt(n, args) => Ty::Adt(
                n.clone(),
                args.iter()
                    .map(|a| Checker::subst_full(a, types, rows, regions))
                    .collect(),
            ),
            Ty::Tuple(args) => Ty::Tuple(
                args.iter()
                    .map(|a| Checker::subst_full(a, types, rows, regions))
                    .collect(),
            ),
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                Checker::subst_full(a, types, rows, regions),
                *m,
                subst_row(e, rows, regions),
                Checker::subst_full(b, types, rows, regions),
            ),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: subst_row(eff, rows, regions),
                inner: Box::new(Checker::subst_full(inner, types, rows, regions)),
            },
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: subst_row(eff, rows, regions),
                region: regions.get(region).cloned().unwrap_or_else(|| region.clone()),
                inner: Box::new(Checker::subst_full(inner, types, rows, regions)),
            },
            Ty::Ref(region, inner) => Ty::Ref(
                regions.get(region).cloned().unwrap_or_else(|| region.clone()),
                Box::new(Checker::subst_full(inner, types, rows, regions)),
            ),
            Ty::Mut(region, inner) => Ty::Mut(
                regions.get(region).cloned().unwrap_or_else(|| region.clone()),
                Box::new(Checker::subst_full(inner, types, rows, regions)),
            ),
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
    /// parameters become type metavariables, row parameters become row
    /// metavariables, and region names are freshened to new rigid names.
    fn instantiate(&mut self, s: &Scheme) -> (Ty, Vec<(String, ClassApp)>) {
        let mut types = HashMap::new();
        for p in &s.params {
            types.insert(p.clone(), self.uni.fresh());
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
        let ctx = s
            .ctx
            .iter()
            .map(|(n, c)| (n.clone(), Self::subst_full_class(c, &types, &rows, &regions)))
            .collect();
        (Self::subst_full(&s.ty, &types, &rows, &regions), ctx)
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
    fn rigidify(&mut self, s: &Scheme) -> (Ty, Vec<(String, ClassApp)>, HashMap<String, Ty>) {
        let mut types = HashMap::new();
        for p in &s.params {
            types.insert(p.clone(), Ty::Param(self.fresh_param(p)));
        }
        let mut rows = HashMap::new();
        for p in &s.row_params {
            rows.insert(p.clone(), RowTail::Rigid(self.fresh_param(p)));
        }
        let mut regions = HashMap::new();
        for p in &s.region_params {
            regions.insert(p.clone(), self.fresh_param(p));
        }
        let ctx = s
            .ctx
            .iter()
            .map(|(n, c)| (n.clone(), Self::subst_full_class(c, &types, &rows, &regions)))
            .collect();
        (
            Self::subst_full(&s.ty, &types, &rows, &regions),
            ctx,
            types,
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
        });
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
                        solutions.insert(wants[i].id, d);
                        progress = true;
                    }
                    Ambiguous => next.push(i),
                }
            }
            pending = next;
            if pending.is_empty() {
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
                    format!(
                        "ambiguous instances for {wanted_z}{}",
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
        if depth > 16 {
            return Err(Error::at(span, "dictionary resolution too deep (recursive instance?)"));
        }
        let wanted_z = self.uni.zonk_class(wanted);
        // Visible `using` binders win over instances.
        for (local, binder) in scope {
            let binder_z = self.uni.zonk_class(binder);
            if binder_z.class == wanted_z.class {
                let mut trial = self.uni.clone();
                let ok = trial.unify(
                    &Ty::Adt(binder_z.class.clone(), binder_z.args.clone()),
                    &Ty::Adt(wanted_z.class.clone(), wanted_z.args.clone()),
                );
                if ok.is_ok() {
                    self.uni = trial;
                    return Ok(Solved(DictExpr::Binder(*local)));
                }
            }
        }
        let class = wanted_z.class.clone();
        // The `Copy` class is compiler-known with structural instances
        // (ownership and usage design, section 3); there are no user
        // instances and its dictionary is a marker.
        if class == "Copy" {
            if let [t] = wanted_z.args.as_slice() {
                if !self.uni.free_metas(t).is_empty() {
                    // An undetermined literal may still fix the type.
                    return Ok(Ambiguous);
                }
                return if self.copy_structural(t, scope, &mut Vec::new()) {
                    Ok(Solved(DictExpr::Marker))
                } else {
                    Err(Error::at(
                        span,
                        format!(
                            "no Copy evidence for {t} (copying needs a Copy type or a `using (copy :: Copy a)` constraint)"
                        ),
                    ))
                };
            }
            return Err(Error::at(
                span,
                format!("Copy expects exactly one type argument, got {wanted_z}"),
            ));
        }
        let wanted_app = Ty::Adt(class.clone(), wanted_z.args.clone());
        let mut matches: Vec<(usize, HashMap<String, Ty>, Unifier)> = Vec::new();
        if let Some(idxs) = self.instances.get(&class).cloned() {
            for i in idxs {
                if !self.instance_list[i].implicit && Some(i) != self.self_instance {
                    continue;
                }
                let inst = &self.instance_list[i];
                // One-way match: the instance's parameters become fresh
                // metavariables and the target is unified with the wanted
                // class application in a trial unifier, so a candidate that
                // fails leaves no bindings. A wanted metavariable is
                // determined by the unique matching instance, which is what
                // fixes literal types (primitive types specification,
                // section 7).
                let mut fresh: HashMap<String, Ty> = HashMap::new();
                for p in &inst.params {
                    fresh.insert(p.clone(), self.uni.fresh());
                }
                let target = Self::subst_class(&inst.target, &fresh);
                let mut trial = self.uni.clone();
                let target_app = Ty::Adt(class.clone(), target.args.clone());
                if trial.unify(&target_app, &wanted_app).is_ok() {
                    let mut sub = HashMap::new();
                    for p in &inst.params {
                        let t = trial.zonk(&fresh[p]);
                        if t == fresh[p] {
                            return Err(Error::at(
                                span,
                                format!(
                                    "instance {} parameter {} is not determined by its target",
                                    inst.name, p
                                ),
                            ));
                        }
                        sub.insert(p.clone(), t);
                    }
                    matches.push((i, sub, trial));
                }
            }
        }
        match matches.as_slice() {
            [] => Err(Error::at(
                span,
                format!(
                    "no instance for {wanted_z}{}",
                    undetermined_hint(&self.uni, &wanted_z)
                ),
            )),
            [(idx, sub, trial)] => {
                let (ctx, base_id) = {
                    let inst = &self.instance_list[*idx];
                    (inst.ctx.clone(), inst.base_id.clone())
                };
                self.uni = trial.clone();
                let mut subs = Vec::new();
                for (_, c) in &ctx {
                    let c = Self::subst_class(c, sub);
                    match self.solve(&c, scope, depth + 1, span)? {
                        Solved(d) => subs.push(d),
                        Ambiguous => {
                            let cz = self.uni.zonk_class(&c);
                            return Err(Error::at(
                                span,
                                format!(
                                    "ambiguous instances for {cz}{}",
                                    undetermined_hint(&self.uni, &cz)
                                ),
                            ));
                        }
                    }
                }
                Ok(Solved(DictExpr::Instance(base_id, subs)))
            }
            _ => Ok(Ambiguous),
        }
    }

    fn fill_holes(&mut self, e: &ECore, sol: &HashMap<usize, DictExpr>) -> Result<ECore> {
        Ok(match e {
            ECore::Hole(id) => match sol.get(id) {
                Some(d) => dict_expr_ecore(d),
                None => return Err(Error::plain("internal error: unsolved dictionary hole")),
            },
            ECore::Int(_) | ECore::ConstBig { .. } | ECore::Lit(_) | ECore::Bool(_)
            | ECore::Unit | ECore::Var(_) | ECore::Global(_)
            | ECore::Ctor(_) | ECore::Prim(_) | ECore::Fault => e.clone(),
            ECore::DictProj(d, i) => {
                ECore::DictProj(Box::new(self.fill_holes(d, sol)?), *i)
            }
            ECore::Proj(d, i) => ECore::Proj(Box::new(self.fill_holes(d, sol)?), *i),
            ECore::InstDict(g, args) => ECore::InstDict(
                g.clone(),
                args.iter()
                    .map(|a| self.fill_holes(a, sol))
                    .collect::<Result<Vec<_>>>()?,
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
                Box::new(self.fill_holes(t, sol)?),
                Box::new(self.fill_holes(f, sol)?),
            ),
            ECore::Demand(d) => ECore::Demand(Box::new(self.fill_holes(d, sol)?)),
            ECore::LazyAlloc => ECore::LazyAlloc,
            ECore::LazyInit(l, t) => {
                ECore::LazyInit(*l, Box::new(self.fill_holes(t, sol)?))
            }
            ECore::Case { scrut, alts } => ECore::Case {
                scrut: Box::new(self.fill_holes(scrut, sol)?),
                alts: alts
                    .iter()
                    .map(|(p, b)| Ok((p.clone(), self.fill_holes(b, sol)?)))
                    .collect::<Result<Vec<_>>>()?,
            },
        })
    }
}

// The rest of the checker (term elaboration, pattern-match compilation)
// lives in `check_terms.rs`; declaration collection and program assembly in
// `check_decls.rs`. Both are included at module scope.
include!("check_terms.rs");
include!("check_decls.rs");
