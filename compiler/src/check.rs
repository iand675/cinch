//! Type checking and elaboration into [`crate::core`].
//!
//! The checker is bidirectional with explicit `forall` quantification (this
//! draft does not silently generalize). Constraints from `using (...)`
//! contexts become dictionary parameters; method calls elaborate to
//! dictionary projection plus application; dictionary selection is solved
//! from visible `using` binders or unique `implicit instance` declarations.
//! Operators are ordinary value names, so `x + y` is just a method call.

use crate::ast;
use crate::core::{ArithMode, ArithOp, BytesOp, CAlt, CBranch, CPat, CmpOp, ConcOp, Core, FieldAccess, FieldShape, FnDef, GlobalId, InstanceDef, IntOp, LocalId, NatOp, PrimOp, Program, ProjField, SymOp};
use crate::types::{ClassApp, EffLabel, EffRow, EitherMeta, IxParam, ITerm, IdxCmp, IdxOp, Mult, PTerm, PredFn, PtrRole, RowTail, Scalar, Scheme, Sort, Ty, Unifier, c_abi_scalar, collect_row_names, default_mult};
use crate::{Error, Result};
use SolveOutcome::{Solved, Ambiguous};
use std::cell::RefCell;
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
    Global(GlobalId),
    Ctor(GlobalId),
    Prim(PrimOp),
    /// A real foreign import used as a value (its wrapper unit's name).
    Foreign(String),
    DictProj(Box<ECore>, usize),
    /// Record field projection: load the field from the record block.
    Proj(Box<ECore>, crate::core::ProjField),
    InstDict(GlobalId, Vec<ECore>),
    App(Box<ECore>, Vec<ECore>),
    /// Un-hoisted lambda: `params` are its pattern parameters.
    Lam { params: Vec<LocalId>, body: Box<ECore> },
    /// Hoisted form after [`hoist_lambdas`].
    MkClos { fn_id: GlobalId, env: Vec<ECore> },
    Let(LocalId, Box<ECore>, Box<ECore>),
    /// A two-way decision point: the condition and its two weighted
    /// branches (attribute annotations specification section 2.1).
    If(Box<ECore>, CBranch<ECore>, CBranch<ECore>),
    /// Case on a boxed ADT value; the alternatives carry their weights
    /// beside them (attribute annotations specification section 2.1).
    Case { scrut: Box<ECore>, alts: Vec<CAlt<ECore>> },
    /// Demand a memo cell (its state machine runs at code generation).
    Demand(Box<ECore>),
    /// Allocate an empty memo cell.
    LazyAlloc,
    /// Store a thunk into a memo cell.
    LazyInit(LocalId, Box<ECore>),
    /// Allocate an empty one-word fixpoint box: the knot for a recursive
    /// `letrec` value whose uses sit under thunks (surface grammar local
    /// amendments section 9).
    BoxAlloc,
    /// Evaluate `value`, store it into the box bound to `box_l`, and
    /// return it.
    BoxInit(LocalId, Box<ECore>),
    /// Read a fixpoint box: a use of a recursive `letrec` value inside a
    /// thunk, resolved when the thunk runs.
    BoxGet(Box<ECore>),
    /// Write through a `Mut` borrow (evidence invalidation over mutable
    /// state): store `val` into the cell the exclusive borrow `local`
    /// names, yielding `Unit`. Lowers to [`crate::core::Core::RefSet`].
    RefSet(LocalId, Box<ECore>),
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
    SyntaxLit(Vec<crate::lexer::Token>),
    /// A compile-time string constant (`String` values).
    StrLit(String),
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
        ECore::LitFold { body, .. } => free_locals(body, bound, out),
        ECore::Var(l) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
        }
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault
        | ECore::SyntaxLit(_) | ECore::StrLit(_) => {}
        ECore::Quote { holes, .. } | ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                free_locals(h, bound, out);
            }
        }
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
            free_locals(&t.body, bound, out);
            free_locals(&f.body, bound, out);
        }
        ECore::Demand(d) => free_locals(d, bound, out),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(t, bound, out);
        }
        ECore::BoxAlloc => {}
        ECore::BoxInit(l, v) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(v, bound, out);
        }
        ECore::RefSet(l, v) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
            free_locals(v, bound, out);
        }
        ECore::BoxGet(v) => free_locals(v, bound, out),
        ECore::Case { scrut, alts } => {
            free_locals(scrut, bound, out);
            for alt in alts {
                let n = bound.len();
                match &alt.pat {
                    CPat::Var(l) => bound.push(*l),
                    CPat::Con { fields, .. } => bound.extend(fields.iter().copied()),
                    CPat::Wild => {}
                }
                free_locals(&alt.body, bound, out);
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
        ECore::LitFold { body, .. } => hoist_lambdas(body, hint, counter, fns),
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_) | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::SyntaxLit(_)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                hoist_lambdas(h, hint, counter, fns);
            }
            // The retained body is reified program text; its lambdas
            // hoist like any others (supercompilation section 7).
            hoist_lambdas(body, hint, counter, fns);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                hoist_lambdas(h, hint, counter, fns);
            }
        }
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
            hoist_lambdas(&mut t.body, hint, counter, fns);
            hoist_lambdas(&mut f.body, hint, counter, fns);
        }
        ECore::Demand(d) => hoist_lambdas(d, hint, counter, fns),
        ECore::LazyAlloc => {}
        ECore::LazyInit(_, t) => hoist_lambdas(t, hint, counter, fns),
        ECore::BoxAlloc => {}
        ECore::BoxInit(_, v) => hoist_lambdas(v, hint, counter, fns),
        ECore::RefSet(_, v) => hoist_lambdas(v, hint, counter, fns),
        ECore::BoxGet(v) => hoist_lambdas(v, hint, counter, fns),
        ECore::Case { scrut, alts } => {
            hoist_lambdas(scrut, hint, counter, fns);
            for alt in alts {
                hoist_lambdas(&mut alt.body, hint, counter, fns);
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
        ECore::LitFold { body, .. } => assign_locals(body, map, next),
        ECore::Var(l) => note_local(*l, map, next),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_) | ECore::Unit | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault
        | ECore::SyntaxLit(_) | ECore::StrLit(_) => {}
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
            assign_locals(&t.body, map, next);
            assign_locals(&f.body, map, next);
        }
        ECore::Demand(d) => assign_locals(d, map, next),
        ECore::LazyAlloc => {}
        ECore::LazyInit(l, t) => {
            note_local(*l, map, next);
            assign_locals(t, map, next);
        }
        ECore::BoxAlloc => {}
        ECore::BoxInit(l, v) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
        }
        ECore::RefSet(l, v) => {
            note_local(*l, map, next);
            assign_locals(v, map, next);
        }
        ECore::BoxGet(v) => assign_locals(v, map, next),
        ECore::Case { scrut, alts } => {
            assign_locals(scrut, map, next);
            for alt in alts {
                match &alt.pat {
                    CPat::Var(l) => note_local(*l, map, next),
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
    let m = |l: &LocalId| *map.get(l).unwrap_or(l);
    match e {
        ECore::LitFold { idx, body } => ECore::LitFold {
            idx: *idx,
            body: Box::new(rewrite_locals(body, map)),
        },
        ECore::Var(l) => ECore::Var(m(l)),
        ECore::Int(v) => ECore::Int(*v),
        ECore::ConstBig { nat, digits } => ECore::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::ConstBytes(b) => ECore::ConstBytes(b.clone()),
        ECore::ConstSym(s) => ECore::ConstSym(s.clone()),
        ECore::Lit(i) => ECore::Lit(*i),
        ECore::Bool(b) => ECore::Bool(*b),
        ECore::Unit => ECore::Unit,
        ECore::Global(g) => ECore::Global(g.clone()),
        ECore::Ctor(g) => ECore::Ctor(g.clone()),
        ECore::Prim(p) => ECore::Prim(*p),
        ECore::Foreign(n) => ECore::Foreign(n.clone()),
        ECore::Hole(h) => ECore::Hole(*h),
        ECore::QuoteHole(h) => ECore::QuoteHole(*h),
        ECore::IdxPending(i) => ECore::IdxPending(*i),
        ECore::Fault => ECore::Fault,
        ECore::SyntaxLit(toks) => ECore::SyntaxLit(toks.clone()),
        ECore::StrLit(s) => ECore::StrLit(s.clone()),
        ECore::Quote { template, holes, body } => ECore::Quote {
            template: template.clone(),
            holes: holes.iter().map(|h| rewrite_locals(h, map)).collect(),
            body: Box::new(rewrite_locals(body, map)),
        },
        ECore::DeclsQuote { template, holes, name_holes } => ECore::DeclsQuote {
            template: template.clone(),
            holes: holes.iter().map(|h| rewrite_locals(h, map)).collect(),
            name_holes: name_holes.clone(),
        },
        ECore::DictProj(d, i) => ECore::DictProj(Box::new(rewrite_locals(d, map)), *i),
        ECore::Proj(d, i) => ECore::Proj(Box::new(rewrite_locals(d, map)), i.clone()),
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
            CBranch {
                weight: t.weight,
                body: Box::new(rewrite_locals(&t.body, map)),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(rewrite_locals(&f.body, map)),
            },
        ),
        ECore::Demand(d) => ECore::Demand(Box::new(rewrite_locals(d, map))),
        ECore::LazyAlloc => ECore::LazyAlloc,
        ECore::LazyInit(l, t) => {
            ECore::LazyInit(m(l), Box::new(rewrite_locals(t, map)))
        }
        ECore::BoxAlloc => ECore::BoxAlloc,
        ECore::BoxInit(l, v) => ECore::BoxInit(m(l), Box::new(rewrite_locals(v, map))),
        ECore::RefSet(l, v) => ECore::RefSet(m(l), Box::new(rewrite_locals(v, map))),
        ECore::BoxGet(v) => ECore::BoxGet(Box::new(rewrite_locals(v, map))),
        ECore::Case { scrut, alts } => ECore::Case {
            scrut: Box::new(rewrite_locals(scrut, map)),
            alts: alts
                .iter()
                .map(|a| {
                    let p = match &a.pat {
                        CPat::Var(l) => CPat::Var(m(l)),
                        CPat::Con { tag, fields } => {
                            CPat::Con { tag: *tag, fields: fields.iter().map(m).collect() }
                        }
                        CPat::Wild => CPat::Wild,
                    };
                    CAlt {
                        pat: p,
                        weight: a.weight,
                        body: Box::new(rewrite_locals(&a.body, map)),
                    }
                })
                .collect(),
        },
    }
}

/// Replace deferred literal nodes by their resolved constants.
fn resolve_lit_words(e: &mut ECore, lits: &[ECore]) {
    match e {
        ECore::LitFold { body, .. } => resolve_lit_words(body, lits),
        ECore::Lit(i) => *e = lits[*i].clone(),
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_) | ECore::Bool(_) | ECore::Unit | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_) | ECore::Prim(_) | ECore::Foreign(_) | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Fault | ECore::LazyAlloc | ECore::BoxAlloc
        | ECore::SyntaxLit(_) | ECore::StrLit(_) => {}
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
            resolve_lit_words(&mut t.body, lits);
            resolve_lit_words(&mut f.body, lits);
        }
        ECore::LazyInit(_, t) => resolve_lit_words(t, lits),
        ECore::BoxInit(_, v) => resolve_lit_words(v, lits),
        ECore::RefSet(_, v) => resolve_lit_words(v, lits),
        ECore::BoxGet(v) => resolve_lit_words(v, lits),
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
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_) | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(_)
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
        ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) => {
            resolve_idx_words(d, vals)
        }
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
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
        ECore::Lam { body, .. } => resolve_idx_words(body, vals),
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
        ECore::BoxInit(_, v) => resolve_idx_words(v, vals),
        ECore::RefSet(_, v) => resolve_idx_words(v, vals),
        ECore::BoxGet(v) => resolve_idx_words(v, vals),
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
/// reify, and their constructions keep their runtime class calls.
fn reify_value(v: &crate::core::Value) -> Option<ECore> {
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
            let mut core = ECore::Ctor(GlobalId(ctor.clone()));
            for f in fields {
                core = app_spine(core, vec![reify_value(f)?]);
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
        | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Var(_)
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_) => false,
        ECore::Lit(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::SyntaxLit(_)
        | ECore::StrLit(_)
        | ECore::Demand(_)
        | ECore::LazyAlloc
        | ECore::LazyInit(_, _)
        | ECore::BoxAlloc
        | ECore::BoxInit(_, _)
        | ECore::RefSet(_, _)
        | ECore::BoxGet(_)
        | ECore::Quote { .. }
        | ECore::DeclsQuote { .. } => true,
        ECore::LitFold { body, .. } => fold_blocked(body),
        ECore::DictProj(d, _) | ECore::Proj(d, _) => fold_blocked(d),
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
            args.iter().any(fold_blocked)
        }
        ECore::App(f, args) => fold_blocked(f) || args.iter().any(fold_blocked),
        ECore::Lam { body, .. } => fold_blocked(body),
        ECore::Let(_, v, body) => fold_blocked(v) || fold_blocked(body),
        ECore::If(c, t, f) => {
            fold_blocked(c) || fold_blocked(&t.body) || fold_blocked(&f.body)
        }
        ECore::Case { scrut, alts } => {
            fold_blocked(scrut) || alts.iter().any(|a| fold_blocked(&a.body))
        }
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
) -> FnDef {
    let body = renumber_fn(&body, &params);
    FnDef {
        id,
        params: params.iter().map(|l| format!("v{}", l.0)).collect(),
        body: lower(&body),
        prim,
        ctor,
        hints,
        meta,
    }
}

fn lower(e: &ECore) -> crate::core::Core {
    match e {
        ECore::Int(v) => crate::core::Core::Int(*v),
        ECore::ConstBig { nat, digits } => crate::core::Core::ConstBig {
            nat: *nat,
            digits: digits.clone(),
        },
        ECore::ConstBytes(b) => crate::core::Core::ConstBytes(b.clone()),
        ECore::ConstSym(s) => crate::core::Core::ConstSym(s.clone()),
        ECore::LitFold { body, .. } => lower(body),
        ECore::Lit(_) => {
            unreachable!("deferred literals are resolved before lowering")
        }
        ECore::Bool(b) => crate::core::Core::Bool(*b),
        ECore::Unit => crate::core::Core::Unit,
        ECore::Var(l) => crate::core::Core::Var(*l),
        ECore::Global(g) => crate::core::Core::Global(g.clone()),
        ECore::Ctor(g) => crate::core::Core::Ctor(g.clone()),
        ECore::Prim(p) => crate::core::Core::Prim(*p),
        ECore::Foreign(n) => crate::core::Core::Foreign(n.clone()),
        ECore::DictProj(d, i) => crate::core::Core::DictProj(Box::new(lower(d)), *i),
        ECore::Proj(d, f) => crate::core::Core::Proj(Box::new(lower(d)), f.clone()),
        ECore::InstDict(g, args) => {
            crate::core::Core::InstDict(g.clone(), args.iter().map(lower).collect())
        }
        ECore::App(f, args) => {
            crate::core::Core::App(Box::new(lower(f)), args.iter().map(lower).collect())
        }
        ECore::Lam { .. } | ECore::Hole(_) | ECore::IdxPending(_) => {
            unreachable!("lambda hoisting and dictionary solving run before lowering")
        }
        ECore::QuoteHole(idx) => crate::core::Core::SpliceVal(*idx),
        ECore::Quote { template, holes, body } => crate::core::Core::Quote {
            template: template.clone(),
            holes: holes.iter().map(lower).collect(),
            body: Some(Box::new(lower(body))),
        },
        ECore::DeclsQuote { template, holes, name_holes } => crate::core::Core::DeclsQuote {
            template: template.clone(),
            holes: holes.iter().map(lower).collect(),
            name_holes: name_holes.clone(),
        },
        ECore::SyntaxLit(toks) => crate::core::Core::SyntaxLit(toks.clone()),
        ECore::StrLit(s) => crate::core::Core::StrLit(s.clone()),
        ECore::Fault => crate::core::Core::Fault,
        ECore::MkClos { fn_id, env } => crate::core::Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(lower).collect(),
        },
        ECore::Let(l, v, body) => {
            crate::core::Core::Let(*l, Box::new(lower(v)), Box::new(lower(body)))
        }
        ECore::If(c, t, f) => crate::core::Core::If(
            Box::new(lower(c)),
            CBranch {
                weight: t.weight,
                body: Box::new(lower(&t.body)),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(lower(&f.body)),
            },
        ),
        ECore::Demand(d) => crate::core::Core::Demand(Box::new(lower(d))),
        ECore::LazyAlloc => crate::core::Core::LazyAlloc,
        ECore::LazyInit(l, t) => {
            crate::core::Core::LazyInit(*l, Box::new(lower(t)))
        }
        ECore::BoxAlloc => crate::core::Core::BoxAlloc,
        ECore::BoxInit(l, v) => {
            crate::core::Core::BoxInit(*l, Box::new(lower(v)))
        }
        ECore::RefSet(l, v) => crate::core::Core::RefSet(*l, Box::new(lower(v))),
        ECore::BoxGet(v) => crate::core::Core::BoxGet(Box::new(lower(v))),
        ECore::Case { scrut, alts } => crate::core::Core::Case {
            scrut: Box::new(lower(scrut)),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(lower(&a.body)),
                })
                .collect(),
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

#[derive(Clone)]
struct TypeKind {
    module: String,
    name: String,
    arity: usize,
    /// Each parameter's kind, in declaration order: type parameters,
    /// index values (`Vec (n :: Nat)`'s `n`), and proposition parameters
    /// (`Dec (p :: Prop)`'s `p`).
    param_kinds: Vec<ParamKind>,
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
    /// `law` members (primitive operations and inline target IR
    /// specification section 5): name and stated equation scheme.
    laws: Vec<(String, Scheme)>,
}

#[derive(Clone)]
struct InstanceInfo {
    module: String,
    name: String,
    params: Vec<String>,
    ctx: Vec<(String, ClassApp)>,
    target: ClassApp,
    /// The dictionary construction unit for this instance.
    base_id: GlobalId,
    implicit: bool,
    /// The declared instance of a parameterized module (module parameters
    /// as module arguments): its dictionary needs the module's parameters,
    /// so it is never resolved on its own. Each instantiation publishes its
    /// own copy under the instantiation unit's name.
    base: bool,
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

struct Want {
    id: usize,
    class: ClassApp,
    span: ast::Span,
    scope: Vec<(LocalId, ClassApp)>,
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
struct IdxPend {
    /// The instantiated index metavariable.
    iterm: ITerm,
    /// Name to local mapping at the use site, for reifying variables.
    names: HashMap<String, LocalId>,
    /// The value's type selects the `Nat` or `Int` runtime.
    nat: bool,
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

/// A user-declared effect (`effect L r where op :: ...`): its label and
/// its operations (design/effects-capabilities-sequencing.md section 2).
struct EffectInfo {
    /// The effect's region parameter name, when the label is `L r`.
    region_param: Option<String>,
    /// Operation name and scheme, in declaration order; the vtable slot is
    /// the index plus one (slot 0 is the return clause).
    ops: Vec<(String, Scheme)>,
}

/// How often a continuation closure's body runs. Every continuation runs
/// at most once per execution of its build site; a body over `%1` captures
/// needs exactly-once semantics, which handler resumptions must promise
/// (see [`Checker::note_use`]).
#[derive(Clone, Debug)]
enum ContKind {
    /// Ordinary control continuations and direct-provider `do` binds.
    Once,
    /// `Result`/`Option` provider binds: the continuation may be skipped
    /// by short-circuiting, so linear captures inside are rejected.
    AtMost,
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

/// Compiler-known sequencing provider class heads.
const SEQ_CLASSES: [&str; 3] = ["DirectSequencing", "ResultSequencing", "OptionSequencing"];

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
        [] => Ok(ClassApp { class: "DirectSequencing".into(), args: Vec::new() }),
        [one] => Ok(one.clone()),
        many => Err(Error::at(
            span,
            format!(
                "two active sequencing providers in one selection scope are ambiguous ({}); bind one explicitly",
                many.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
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
    match t {
        ITerm::Var(n) => {
            if !out.iter().any(|x| x == n) {
                out.push(n.clone());
            }
        }
        ITerm::App(f, args) => {
            collect_idx_reads(f, out);
            for a in args {
                collect_idx_reads(a, out);
            }
        }
        ITerm::Bin(_, a, b) => {
            collect_idx_reads(a, out);
            collect_idx_reads(b, out);
        }
        ITerm::Num(_) | ITerm::Fn(_) | ITerm::MVar(_) => {}
    }
}

/// The value variables a proposition reads across all its index terms.
fn collect_prop_reads(p: &PTerm, out: &mut Vec<String>) {
    match p {
        PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
            collect_idx_reads(a, out);
            collect_idx_reads(b, out);
        }
        PTerm::Not(q) => collect_prop_reads(q, out),
        PTerm::Pred(_, a) => collect_idx_reads(a, out),
        PTerm::Var(_) | PTerm::MVar(_) => {}
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
            !matches!(op, crate::core::BytesOp::Len | crate::core::BytesOp::Get)
        }
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
    let ECore::Global(g) = callee else {
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
    Ty::Adt("Result".into(), vec![e.clone(), v.clone()])
}

// ---------------------------------------------------------------- the checker

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
    qualified_sigs: HashMap<(String, String), SigInfo>,
    instances: HashMap<String, Vec<usize>>,
    instance_list: Vec<InstanceInfo>,
    fns: Vec<FnDef>,
    lam_counter: usize,
    /// `letrec` member lambdas awaiting their code generation unit:
    /// elaborated with their final unit id already chosen, finalized in
    /// `finish_fn` beside the enclosing definition's hoisted lambdas
    /// (surface grammar local amendments section 9).
    pending_rec: Vec<(crate::core::GlobalId, ECore)>,
    next_local: usize,
    param_counter: usize,
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
    region_counter: usize,
    /// The prelude module, home of `List`, `Cons` and `Nil` for list
    /// literal and list pattern desugaring.
    prelude_module: String,
    /// Tuple constructor code generation units already synthesized, by
    /// arity.
    tuple_ctors: Vec<usize>,
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
    /// Each module's foreign declarations, for its interface.
    module_foreign: Vec<(String, crate::interface::ForeignExport)>,
    /// Real foreign binding contracts by unit name (foreign function
    /// interface specification section 2), for the program's ABI maps.
    foreign_plans: Vec<(String, crate::core::ForeignPlan)>,
    /// Real foreign bindings awaiting contract construction (their
    /// classification reads the ABI layout table, computed after the
    /// declaration pass).
    pending_foreign: Vec<(String, crate::ast::ForeignDecl, Scheme, ForeignAbi)>,
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
    /// Record-projection path accumulated for a `Proj` operand place.
    place_path: Vec<String>,
    /// User-declared effects by name (`effect L where ...`).
    effects: HashMap<String, EffectInfo>,
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
    stage_emitted: std::collections::HashMap<String, Vec<crate::ast::Decl>>,
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
    /// The target profile's capability evidence: a body's `Available X`
    /// condition holds where `X` is listed (platform proofs and SIMD
    /// specification section 3).
    target_caps: Vec<String>,
    /// The target profile's arch (the target triple's first component):
    /// a body's `when arch <name>` platform condition holds where the
    /// names agree (primitive operations and inline target IR
    /// specification section 13).
    target_arch: String,
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
    ev_uses: Vec<(LocalId, Vec<String>, ast::Span)>,
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
}

impl ImportView {
    /// The source's name for this view's local name, or `None` when the
    /// view does not make it visible.
    fn source_name<'a>(&'a self, local: &'a str) -> Option<&'a str> {
        match &self.members {
            None => Some(local),
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
#[derive(Clone, Debug)]
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
            pending_rec: Vec::new(),
            next_local: 0,
            param_counter: 0,
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
            top_eqs: Vec::new(),
            instance_defs: Vec::new(),
            instance_defs_pending: Vec::new(),
            pending_instance_heads: Vec::new(),
            embedded_modules: std::collections::HashSet::new(),
            shadowed_modules: std::collections::HashSet::new(),
            self_instance: None,
            perf: EffRow::pure(),
            region_counter: 0,
            prelude_module: String::new(),
            tuple_ctors: Vec::new(),
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
            module_foreign: Vec::new(),
            foreign_plans: Vec::new(),
            pending_foreign: Vec::new(),
            uses: HashMap::new(),
            obligations: HashMap::new(),
            lam_depth: 0,
            memo_depth: 0,
            let_depth: 0,
            active_regions: Vec::new(),
            oneshot_frames: Vec::new(),
            scope_frames: Vec::new(),
            task_captures: Vec::new(),
            place_path: Vec::new(),
            effects: HashMap::new(),
            codata_types: HashSet::new(),
            effect_ops: HashMap::new(),
            conts: Vec::new(),
            cont_depth: 0,
            extents: Vec::new(),
            seq_binders: Vec::new(),
            current_cont: Vec::new(),
            compile_time: false,
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
            target_caps: Vec::new(),
            target_arch: String::new(),
            law_obligations: Vec::new(),
            pres_obligations: Vec::new(),
            law_rows: Vec::new(),
            pending_upgrades: Vec::new(),
            upgrade_evidence: Vec::new(),
            rewrites: Vec::new(),
            ev_writes: Vec::new(),
            ev_derivs: HashMap::new(),
            ev_uses: Vec::new(),
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
        self.ev_writes.clear();
        self.ev_derivs.clear();
        self.ev_uses.clear();
        self.cur_temporal = None;
        self.lam_depth = 0;
        self.memo_depth = 0;
        self.let_depth = 0;
        self.active_regions.clear();
        self.oneshot_frames.clear();
        self.scope_frames.clear();
        self.task_captures.clear();
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
        }
        env.locals.push((name.to_string(), l, scheme));
        Ok(())
    }

    /// The mutable locations an evidence item's statement reads (evidence
    /// invalidation over mutable state): the regions of `Mut` and `Ref`
    /// borrows named by the proposition's index terms, split by borrow
    /// kind so `Ref`-sourced (read-only) evidence is never invalidated.
    fn ev_footprint(&self, q: &PTerm, env: &Env) -> (Vec<String>, Vec<String>) {
        let mut vars = Vec::new();
        collect_prop_reads(q, &mut vars);
        let mut muts = Vec::new();
        let mut refs = Vec::new();
        for v in vars {
            if let Some((_, s)) = env.lookup(&v) {
                match self.uni.zonk(&s.ty) {
                    Ty::Mut(r, _) => {
                        if !muts.contains(&r) {
                            muts.push(r);
                        }
                    }
                    Ty::Ref(r, _) => {
                        if !refs.contains(&r) {
                            refs.push(r);
                        }
                    }
                    _ => {}
                }
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
    fn record_ev_use(&mut self, l: LocalId, prop: &PTerm, env: &Env, use_span: ast::Span) {
        if !self.ev_derivs.contains_key(&l) {
            return;
        }
        let (muts, _refs) = self.ev_footprint(prop, env);
        if muts.is_empty() {
            return;
        }
        self.ev_uses.push((l, muts, use_span));
    }

    /// Check every recorded evidence use for staleness (evidence
    /// invalidation over mutable state), now that regions have unified.
    /// A use whose evidence a later write invalidated is rejected, naming
    /// the derivation site and the invalidating mutation.
    fn check_ev_uses(&mut self) -> Result<()> {
        let uses = std::mem::take(&mut self.ev_uses);
        for (l, muts, use_span) in uses {
            let Some(&(deriv_len, deriv_span)) = self.ev_derivs.get(&l) else {
                continue;
            };
            for w in &self.ev_writes[deriv_len..] {
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
            targets.push(Some(r));
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
                Some(a) => targets.push(Some(a.clone())),
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
                    });
                } else {
                    // A `withScope` body capture: the borrowed
                    // environment rule (core specification, section 7)
                    // still applies.
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
        let (has_at_most, has_risky) = {
            let active = &self.conts[self.conts.len() - self.cont_depth..];
            (
                active.iter().any(|f| matches!(f.kind, ContKind::AtMost)),
                active.iter().any(|f| f.risky),
            )
        };
        if linear && has_at_most {
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
            let handle = crate::types::mentions_task(&t);
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
            let transferred = matches!(
                self.effective_mult(c.binder),
                Some(Mult::One) | Some(Mult::Aff)
            );
            if !transferred && !self.share_safe(&t, &c.dicts) {
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

    /// The `Share`-like evidence for a task capture (concurrency
    /// specification, section 3): structural `Copy`, or a class
    /// dictionary over shareable evidence (an immutable method record).
    fn share_safe(&self, t: &Ty, dicts: &[(LocalId, ClassApp)]) -> bool {
        match t {
            Ty::Adt(name, args)
                if self.lookup_type(&[name]).is_err() && self.sigs.contains_key(name) =>
            {
                args.iter().all(|a| self.share_safe(a, dicts))
            }
            _ => self.copy_structural(t, dicts, &mut Vec::new()),
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
            // `Sym` is a handle to runtime-owned interned state; the
            // specification grants it `Eq`/`Ord`/`Show` only (primitive
            // types specification section 9), never `Copy`.
            Ty::Sym => false,
            // An owned boundary pointer transfers ownership exactly once
            // (foreign function interface specification section 3), so it
            // is not duplicable; raw and borrow roles are plain addresses
            // and carry no ownership to duplicate.
            Ty::Ptr { role: PtrRole::Owned(_), .. } => false,
            Ty::Ptr { .. } => true,
            // `Later` keeps the thunk representation at runtime (codata
            // and guardedness design, section 6), so like a thunk it is
            // not duplicable evidence.
            Ty::Later(_) => false,
            Ty::Fun(_, _, _, _) | Ty::Pi { .. } | Ty::Handler { .. } => false,
            // Index values and erased propositions occupy no runtime
            // representation that could be duplicated (core specification
            // section 3).
            Ty::Sort(_) | Ty::Idx(_) | Ty::Prp(_) => true,
            Ty::Sigma { dom, cod, .. } => {
                self.copy_structural(dom, scope, seen) && self.copy_structural(cod, scope, seen)
            }
            Ty::Region(_) => true,
            Ty::Param(p) => scope.iter().any(|(_, c)| {
                c.class == "Copy" && c.args.iter().any(|a| matches!(a, Ty::Param(q) if q == p))
            }),
            // A higher-kinded application is duplicable only under an
            // explicit `Copy` constraint on the applied type itself; a
            // bare type former is never a value's type.
            Ty::App(..) | Ty::Former(_) => scope
                .iter()
                .any(|(_, c)| c.class == "Copy" && c.args.iter().any(|a| a == ty)),
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
                // A codata value is a record of observation closures: the
                // closures themselves are never `Copy`.
                if self.codata_types.contains(&(info.module.clone(), info.name.clone())) {
                    return false;
                }
                for (cname, _) in &info.ctors {
                    let v = match self.ctor_of_type(&info, cname) {
                        Ok(v) => v,
                        Err(_) => return false,
                    };
                    let fields = match self.ctor_parts(&v, args) {
                        Ok(p) => p.fields.into_iter().map(|(_, t, _)| t).collect::<Vec<_>>(),
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
    /// to [`Checker::resolved_literals`].
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
    /// the runtime, and a `Nat` literal must not be negative. `Sym` names
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
                format!("integer literal cannot have type {other}"),
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
            vals.push(self.reify_idx(&p.iterm, &p.names, p.nat, p.span)?);
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
                match if fold { self.fold_constant(body) } else { None } {
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
            | ECore::ConstBytes(_) | ECore::ConstSym(_)
            | ECore::Lit(_)
            | ECore::Bool(_)
            | ECore::Unit
            | ECore::Var(_)
            | ECore::Global(_)
            | ECore::Ctor(_)
            | ECore::Prim(_)
            | ECore::Foreign(_)
            | ECore::Hole(_)
            | ECore::QuoteHole(_)
            | ECore::IdxPending(_)
            | ECore::Fault
            | ECore::SyntaxLit(_)
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
            ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) => {
                self.fold_pass(d, unit, fold)
            }
            ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
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
            ECore::Lam { body, .. } => self.fold_pass(body, unit, fold),
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
            ECore::BoxInit(_, v) => self.fold_pass(v, unit, fold),
            ECore::RefSet(_, v) => self.fold_pass(v, unit, fold),
            ECore::BoxGet(v) => self.fold_pass(v, unit, fold),
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
    fn fold_constant(&mut self, body: &ECore) -> Option<ECore> {
        let mut free = Vec::new();
        free_locals(body, &mut Vec::new(), &mut free);
        if !free.is_empty() || fold_blocked(body) {
            return None;
        }
        let core = lower(body);
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let mut frame: Vec<crate::core::Value> = Vec::new();
        // A construction that does not evaluate at compile time (or
        // exceeds the fold's budget) computes at runtime, exactly as it
        // would without the fold.
        let v = self.eval_core(&mut cx, &core, &mut frame).ok()?;
        reify_value(&v)
    }

    /// An index term as runtime code (core specification section 3:
    /// witnesses execution needs are retained, and only executable
    /// definitions can be reified).
    fn reify_idx(
        &self,
        it: &ITerm,
        names: &HashMap<String, LocalId>,
        nat: bool,
        span: ast::Span,
    ) -> Result<ECore> {
        Ok(match self.uni.resolve_idx(it) {
            ITerm::Num(d) => ECore::ConstBig { nat, digits: d },
            ITerm::Var(n) => match names.get(&n) {
                Some(l) => ECore::Var(*l),
                None => {
                    return Err(Error::at(
                        span,
                        format!("the index value `{n}` has no runtime value at this call site"),
                    ))
                }
            },
            ITerm::Fn(g) => {
                self.check_executable(&g, span)?;
                ECore::Global(GlobalId(g))
            }
            ITerm::App(f, args) => {
                let mut cores = vec![self.reify_idx(&f, names, nat, span)?];
                for a in &args {
                    cores.push(self.reify_idx(a, names, nat, span)?);
                }
                let head = cores.remove(0);
                app_spine(head, cores)
            }
            ITerm::Bin(op, a, b) => {
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
                .unwrap_or(false);
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
                    Some(m) => m.values().any(|d| d.module == module && d.name == name),
                    None => {
                        v.target == module && self.surface_def(&v.target, name).is_some()
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
        let info = self.lookup_type(&[name]).ok()?;
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

    fn lookup_sig(&self, path: &ast::ConPath) -> Result<SigInfo> {
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
                    let (name, params) = if *name == "Io" {
                        ("Io", vec!["r".into()])
                    } else {
                        ("Copy", vec!["a".into()])
                    };
                    return Ok(SigInfo {
                        module: "<builtin>".into(),
                        name: name.into(),
                        params: params.clone(),
                        param_kinds: vec![ParamKind::Type; params.len()],
                        methods: Vec::new(),
                        laws: Vec::new(),
                    });
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
            [m, name] => {
                for (dm, dn) in self.scope_qual_candidates(m, name) {
                    if let Some(s) = self
                        .sigs
                        .get(&dn)
                        .and_then(|ss| ss.iter().find(|s| s.module == dm))
                    {
                        return Ok(s.clone());
                    }
                }
                if self.module_names.contains(*m) {
                    return Err(Error::plain(format!("unknown signature {m}.{name}")));
                }
                self.qualified_sigs
                    .get(&(m.to_string(), name.to_string()))
                    .cloned()
                    .ok_or_else(|| Error::plain(format!("unknown signature {m}.{name}")))
            }
            _ => Err(Error::plain("signature paths deeper than Module.Sig are not supported")),
        }
    }

    fn resolve_var(&self, p: &ast::VarPath, span: ast::Span) -> Result<ValueInfo> {
        // The registered name is the unit's real name: a qualified
        // spelling (`U32.wrappingAdd`) is an alias over it, and code
        // generation ids derive from the registered pair.
        match p.quals.as_slice() {
            [] => self.lookup_value(None, &p.name.text).map_err(|e| Error::at(span, e.message)),
            [q] => self
                .lookup_value(Some(&q.text), &p.name.text)
                .map_err(|e| Error::at(span, e.message)),
            _ => {
                Err(Error::at(span, "qualified names deeper than Module.name are not supported"))
            }
        }
    }

    fn resolve_con(&self, p: &ast::ConPath, span: ast::Span) -> Result<ValueInfo> {
        let parts = p.parts();
        let (module, name) = match parts.as_slice() {
            [name] => {
                let v = self.lookup_value(None, name)?;
                (v.module.clone(), name.to_string())
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
        if !self.tuple_ctors.contains(&n) {
            self.tuple_ctors.push(n);
            self.fns.push(FnDef {
                id: GlobalId(format!("Tuple.{n}")),
                params: (0..n).map(|i| format!("f{i}")).collect(),
                body: crate::core::Core::Unit,
                prim: None,
                ctor: Some((0, n)),
                hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
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
        // Class arguments check against the parameter kinds: `Functor
        // List` and `Functor ((->) r)` take a partially applied type
        // constructor, `Eq a` a fully applied one.
        let mut args = Vec::new();
        for (i, a) in c.args.iter().enumerate() {
            args.push(self.conv_kind(a, vars, sig.param_kinds.get(i))?);
        }
        let mut app = ClassApp { class: sig.name.clone(), args };
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
        Ok(t)
    }

    /// The kind of a converted type as far as it is known: a named
    /// constructor's missing argument count (`Result e` is `Type ->
    /// Type`), a partially applied type former's, or `Type`. A
    /// parameter's kind is being inferred here, so it matches its
    /// expected kind by construction.
    fn check_kind(&self, t: &Ty, expect: Option<&ParamKind>, span: ast::Span) -> Result<()> {
        let Some(expect) = expect else { return Ok(()) };
        let kind = match t {
            Ty::Adt(name, args) => {
                let info = self.lookup_type(&[name])?;
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
                            self.lookup_type(&[name])?.arity
                        ),
                    ));
                }
                Err(Error::at(
                    span,
                    format!("{t} is a type constructor (`Type -> Type`), but a fully applied type is expected here"),
                ))
            }
            (ParamKind::Type, ParamKind::Con(n)) => Err(Error::at(
                span,
                format!(
                    "{t} is a fully applied type, but a type constructor of kind `Type -> {rest}Type` is expected here",
                    rest = "Type -> ".repeat(n.saturating_sub(1))
                ),
            )),
            _ => Err(Error::at(
                span,
                format!("type {t} does not have the kind expected here"),
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
            other => Err(Error::at(span, format!("cannot apply non-type-constructor {other}"))),
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
    fn conv_idx(&self, ty: &ast::Type, vars: &[String], kind: &Ty) -> Result<ITerm> {
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
            ast::Type::Num(s, _) => {
                if !args.is_empty() {
                    return Err(Error::at(span, "a numeral takes no arguments"));
                }
                Ok(ITerm::Num(s.clone()))
            }
            ast::Type::Var(n, _) => {
                if args.is_empty() {
                    if !vars.contains(&n.text) {
                        // A bare named total definition is an index
                        // value too: a nullary method (`mempty`) or a
                        // named helper a law's equation mentions
                        // (primitive operations and inline target IR
                        // specification section 5).
                        return self.idx_app(&n.text, &args, vars, kind, n.span);
                    }
                    self.note_kind(&n.text, ParamKind::Val(kind.clone()));
                    return Ok(ITerm::Var(n.text.clone()));
                }
                if let Some(opc) = match n.text.as_str() {
                    "+" => Some(IdxOp::Add),
                    "-" => Some(IdxOp::Sub),
                    "*" => Some(IdxOp::Mul),
                    _ => None,
                } {
                    if args.len() != 2 {
                        return Err(Error::at(
                            span,
                            format!("`{}` takes two index arguments", n.text),
                        ));
                    }
                    return Ok(ITerm::Bin(
                        opc,
                        Box::new(self.conv_idx(args[0], vars, kind)?),
                        Box::new(self.conv_idx(args[1], vars, kind)?),
                    ));
                }
                self.idx_app(&n.text, &args, vars, kind, n.span)
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
                self.idx_app(&name, &args, vars, kind, *cspan)
            }
            other => Err(Error::at(
                other.span(),
                "expected an index value: a variable, a numeral, arithmetic (`n + 1`), or a total function application",
            )),
        }
    }

    /// An application in index position: `toNat limit`, `length bytes`,
    /// or a named total definition.
    fn idx_app(
        &self,
        name: &str,
        args: &[&ast::Type],
        vars: &[String],
        kind: &Ty,
        span: ast::Span,
    ) -> Result<ITerm> {
        // The builtin index vocabulary shadows ordinary names in index
        // position (decision D29): `length` over `Bytes` and `toNat` over
        // machine integers are total, executable, and reduce nowhere in
        // conversion.
        let head = if matches!(name, "toNat" | "length") {
            format!("<builtin>.{name}")
        } else {
            match self.lookup_value(None, name) {
                Ok(v) => fn_id(&v.module, &v.name).0,
                Err(_) => {
                    return Err(Error::at(
                        span,
                        format!("unknown name {name} in index position"),
                    ))
                }
            }
        };
        let mut out = Vec::new();
        for a in args {
            out.push(self.conv_idx(a, vars, kind)?);
        }
        Ok(ITerm::app_fn(head, out))
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
                                "`{}` is not a proposition former (use a comparison, `Eq x y`, or `Not p`)",
                                op.text
                            ),
                        ))
                    }
                };
                let unit = Ty::Nat;
                Ok(PTerm::Cmp(
                    cmp,
                    Box::new(self.conv_idx(args[0], vars, &unit)?),
                    Box::new(self.conv_idx(args[1], vars, &unit)?),
                ))
            }
            ast::Type::Con(p, _) if p.parts() == ["Eq"] && args.len() == 2 => {
                let unit = Ty::Nat;
                Ok(PTerm::Eq(
                    Box::new(self.conv_idx(args[0], vars, &unit)?),
                    Box::new(self.conv_idx(args[1], vars, &unit)?),
                ))
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
                let pf = PredFn::lookup(&name).ok_or_else(|| {
                    Error::at(
                        span,
                        format!(
                            "`{name}` is not a proposition former (use a comparison, `Eq x y`, `Not p`, or a decidable predicate: `NotNull`, `NonZero`, `Ascii`, `Ordered`)"
                        ),
                    )
                })?;
                let unit = Ty::Nat;
                let operand = self.conv_idx(args[0], vars, &unit)?;
                Ok(PTerm::Pred(pf, Box::new(operand)))
            }
            ast::Type::Var(n, _) if args.is_empty() && vars.contains(&n.text) => {
                Ok(PTerm::Var(n.text.clone()))
            }
            other => Err(Error::at(
                other.span(),
                "expected a proposition: a comparison (`i < n`), `Eq x y`, `Not p`, a decidable predicate (`NotNull`, `NonZero`, `Ascii`, `Ordered`), or a proposition variable",
            )),
        }
    }

    /// The admissible predicate forms of a refinement (refinement types
    /// design: the open question's answer). Admitted: the `Dec`
    /// propositions over index terms (bounded numeric comparisons and
    /// index equalities) when the refined container is numeric, the
    /// structural vocabulary predicates applied to the refined binder's
    /// value itself, and `Not` over admitted forms. Everything else is
    /// refused naming the form: quantified propositions, conjunction and
    /// disjunction (the decidable logic has neither), predicates over
    /// anything but the container's value, and proposition variables
    /// (their truth is assumed, never decided).
    fn check_pred_admissible(
        &self,
        pred: &PTerm,
        binder: &str,
        base: &Ty,
        span: ast::Span,
    ) -> Result<()> {
        match pred {
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
                        other => Err(Error::at(
                            span,
                            format!(
                                "a comparison or index equality is an admitted predicate over numeric values (bounded numeric comparisons and index equalities), not over `{other}` (the structural vocabulary decides container values: `NotNull`, `NonZero`, `Ascii`, `Ordered`)"
                            ),
                        )),
                    }
                } else {
                    Ok(())
                }
            }
            PTerm::Not(q) => self.check_pred_admissible(q, binder, base, span),
            PTerm::Var(_) | PTerm::MVar(_) => Err(Error::at(
                span,
                "a proposition variable is not a decidable predicate: its truth is assumed, never decided (state the predicate over the refined binder)",
            )),
        }
    }

    /// The operand shapes one structural-vocabulary predicate decides
    /// over (a wrong predicate is refused naming it and its admissible
    /// operand).
    fn check_pred_operand(&self, pf: PredFn, base: &Ty, span: ast::Span) -> Result<()> {
        let ok = match pf {
            PredFn::NotNull | PredFn::Ordered => {
                // A list-shaped container: constructors that distinguish
                // the empty form (no runtime fields) from non-empty
                // forms. `Ordered` further needs numeric elements.
                let Ty::Adt(name, args) = base else {
                    return Err(Error::at(
                        span,
                        format!(
                            "`{pf}` decides {}, not `{base}` (a wrong predicate for this refinement's container is refused naming the predicate)",
                            pf.operand_note()
                        ),
                    ));
                };
                let shape_ok = self
                    .types
                    .get(name)
                    .map(|tks| {
                        tks.iter().any(|tk| {
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
                    })
                    .unwrap_or(false);
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
            PredFn::Ascii => {
                matches!(base, Ty::Bytes)
                    || matches!(base, Ty::Adt(name, _) if name == "String")
            }
        };
        if ok {
            Ok(())
        } else {
            Err(Error::at(
                span,
                format!(
                    "`{pf}` decides {}, not `{base}` (a wrong predicate for this refinement's container is refused naming the predicate)",
                    pf.operand_note()
                ),
            ))
        }
    }

    /// The container instance of a refinement application (`NonEmpty x`'s
    /// base is `List x`): the declared container with the application's
    /// arguments substituted for the refinement's parameters. A partial
    /// application (the higher-kinded `NonEmpty` former) yields the
    /// container former. `None` for non-refinements.
    fn refine_base(&self, t: &Ty) -> Option<Ty> {
        let Ty::Adt(name, args) = t else {
            return None;
        };
        let tk = self
            .types
            .get(name)?
            .iter()
            .find(|t| t.refine.is_some())?;
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
            if self
                .types
                .get(n)
                .map(|ts| ts.iter().any(|tk| tk.refine.is_some()))
                .unwrap_or(false)
            {
                return Some(n.clone());
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
            Some("Eq" | "Not") => {
                return Ok(Ty::Prp(Box::new(self.conv_prop(ty, vars)?)));
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
                    return Ok(Ty::Adt("Io".into(), vec![Ty::Region(region)]));
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
                        vec![Ty::Region(r), Ty::Region(s)],
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
                        vec![Ty::Region(s), self.conv_type(x, vars)?, self.conv_type(v, vars)?],
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
            let mut result = Ty::Param(name);
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
                Some(self.lookup_type(&p.parts()).map_err(|_| {
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
                            if let Some(k) = expect {
                                self.note_kind(name, k.clone());
                            }
                            return Ok(Ty::Param(name.to_string()));
                        }
                        if let Some(s) = Scalar::parse(name) {
                            return Ok(Ty::Prim(s));
                        }
                        if let Some(s) = c_abi_scalar(name) {
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
                    if let Some(k) = expect {
                        self.note_kind(&n.text, k.clone());
                    }
                    Ok(Ty::Param(n.text.clone()))
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
            ast::Type::Forall(_, _, span) => Err(Error::at(
                *span,
                "unexpected forall inside a type; quantifiers are only supported at the outermost position of a signature",
            )),
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
                let cod = self.conv_type(res, &vars2)?;
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
                let cod = self.conv_type(body, &vars2)?;
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

    fn conv_scheme(&self, ty: &ast::Type) -> Result<Scheme> {
        self.kind_constraints.borrow_mut().clear();
        let mut params: Vec<String> = Vec::new();
        let mut ctx: Vec<(String, ClassApp)> = Vec::new();
        // `forall` binders in order, with their kind hints.
        let mut binders: Vec<(String, Option<ParamKind>, ast::Span)> = Vec::new();
        let mut rest = ty;
        loop {
            match rest {
                ast::Type::Forall(bs, body, _) => {
                    for b in bs {
                        if params.contains(&b.name.text) {
                            return Err(Error::at(b.name.span, "duplicate type parameter"));
                        }
                        let hint = match &b.ann {
                            Some(ann) => Some(self.binder_kind(ann, &params)?),
                            None => None,
                        };
                        params.push(b.name.text.clone());
                        binders.push((b.name.text.clone(), hint, b.name.span));
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
                    // are classified by how they are used (an un-annotated
                    // binder used only as an index value is an implicit
                    // value argument, decision D29).
                    let mut row_params: Vec<String> = Vec::new();
                    let mut region_params: Vec<String> = Vec::new();
                    collect_row_names(&t, &mut row_params, &mut region_params);
                    for (_, c) in &ctx {
                        for a in &c.args {
                            if let Ty::Region(r) = a {
                                if !region_params.contains(r) {
                                    region_params.push(r.clone());
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
                    let mut val_params: Vec<(String, Ty)> = Vec::new();
                    let mut prop_params: Vec<String> = Vec::new();
                    for (name, hint, span) in binders {
                        if row_params.contains(&name) || region_params.contains(&name) {
                            continue;
                        }
                        let kind =
                            self.resolve_binder_kind(&name, hint.as_ref(), &constraints, span)?;
                        match kind {
                            ParamKind::Type | ParamKind::Con(_) => type_params.push(name),
                            ParamKind::Val(vty) => val_params.push((name, vty)),
                            ParamKind::Prop => prop_params.push(name),
                        }
                    }
                    return Ok(Scheme {
                        params: type_params,
                        idx_params: Vec::new(),
                        val_params,
                        prop_params,
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
    /// region names (by fresh name). Index-level variables substitute from
    /// the same map: an index parameter's entry is a `Ty::Idx` value.
    fn subst_full(
        t: &Ty,
        types: &HashMap<String, Ty>,
        rows: &HashMap<String, RowTail>,
        regions: &HashMap<String, String>,
    ) -> Ty {
        match t {
            Ty::Param(p) => types.get(p).cloned().unwrap_or_else(|| t.clone()),
            Ty::Idx(it) => Ty::Idx(Box::new(Checker::subst_it(it, types))),
            Ty::Prp(p) => Ty::Prp(Box::new(Checker::subst_pt(p, types))),
            Ty::Pi { var, dom, mult, eff, cod } => {
                let cod = if types.contains_key(var) {
                    (**cod).clone()
                } else {
                    Checker::subst_full(cod, types, rows, regions)
                };
                Ty::Pi {
                    var: var.clone(),
                    dom: Box::new(Checker::subst_full(dom, types, rows, regions)),
                    mult: *mult,
                    eff: subst_row(eff, rows, regions),
                    cod: Box::new(cod),
                }
            }
            Ty::Sigma { var, dom, cod } => {
                let cod = if types.contains_key(var) {
                    (**cod).clone()
                } else {
                    Checker::subst_full(cod, types, rows, regions)
                };
                Ty::Sigma {
                    var: var.clone(),
                    dom: Box::new(Checker::subst_full(dom, types, rows, regions)),
                    cod: Box::new(cod),
                }
            }
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
            Ty::Later(inner) => {
                Ty::Later(Box::new(Checker::subst_full(inner, types, rows, regions)))
            }
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
            Ty::Region(r) => Ty::Region(regions.get(r).cloned().unwrap_or_else(|| r.clone())),
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: EffLabel {
                    head: label.head.clone(),
                    arg: label
                        .arg
                        .as_ref()
                        .map(|a| regions.get(a).cloned().unwrap_or_else(|| a.clone())),
                },
                input: Box::new(Checker::subst_full(input, types, rows, regions)),
                output: Box::new(Checker::subst_full(output, types, rows, regions)),
                eff: subst_row(eff, rows, regions),
                resumption: *resumption,
            },
            // Var-headed application substitutes with beta-reduction: a
            // head replaced by a constructor application collects the
            // arguments into it (`f a` with `f := Result e` is `Result e
            // a`), and a head replaced by a type former saturates to its
            // image (`f a` with `f := (->) r` is `r -> a`).
            Ty::App(h, args) => {
                let h = Checker::subst_full(h, types, rows, regions);
                let args: Vec<Ty> = args
                    .iter()
                    .map(|a| Checker::subst_full(a, types, rows, regions))
                    .collect();
                Ty::apply_spine(h, args)
            }
            other => other.clone(),
        }
    }

    /// Substitute index-level variables in an index term from the map.
    fn subst_it(t: &ITerm, map: &HashMap<String, Ty>) -> ITerm {
        match t {
            ITerm::Var(n) => match map.get(n) {
                Some(Ty::Idx(it)) => (**it).clone(),
                _ => t.clone(),
            },
            ITerm::App(f, args) => ITerm::App(
                Box::new(Checker::subst_it(f, map)),
                args.iter().map(|a| Checker::subst_it(a, map)).collect(),
            ),
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(Checker::subst_it(a, map)),
                Box::new(Checker::subst_it(b, map)),
            ),
            other => other.clone(),
        }
    }

    /// Substitute index-level variables in a proposition from the map.
    fn subst_pt(p: &PTerm, map: &HashMap<String, Ty>) -> PTerm {
        match p {
            PTerm::Var(n) => match map.get(n) {
                Some(Ty::Prp(q)) => (**q).clone(),
                Some(Ty::Idx(_)) => p.clone(),
                _ => p.clone(),
            },
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(Checker::subst_it(a, map)),
                Box::new(Checker::subst_it(b, map)),
            ),
            PTerm::Eq(a, b) => PTerm::Eq(
                Box::new(Checker::subst_it(a, map)),
                Box::new(Checker::subst_it(b, map)),
            ),
            PTerm::Not(q) => PTerm::Not(Box::new(Checker::subst_pt(q, map))),
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
    fn instantiate(&mut self, s: &Scheme) -> (Ty, Vec<(String, ClassApp)>, Vec<Ty>) {
        let mut types = HashMap::new();
        for p in &s.params {
            types.insert(p.clone(), self.uni.fresh());
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
        let ctx = s
            .ctx
            .iter()
            .map(|(n, c)| (n.clone(), Self::subst_full_class(c, &types, &rows, &regions)))
            .collect();
        (Self::subst_full(&s.ty, &types, &rows, &regions), ctx, val_args)
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
    fn rigidify(&mut self, s: &Scheme) -> (Ty, Vec<(String, ClassApp)>, HashMap<String, Ty>) {
        let mut types = HashMap::new();
        for p in &s.params {
            types.insert(p.clone(), Ty::Param(self.fresh_param(p)));
        }
        for p in &s.idx_params {
            match p {
                IxParam::Val(n, _) => {
                    types.insert(n.clone(), Ty::Idx(Box::new(ITerm::Var(n.clone()))));
                }
                IxParam::Prop(n) => {
                    types.insert(n.clone(), Ty::Prp(Box::new(PTerm::Var(n.clone()))));
                }
            }
        }
        for (n, _) in &s.val_params {
            types.insert(n.clone(), Ty::Idx(Box::new(ITerm::Var(n.clone()))));
        }
        for n in &s.prop_params {
            types.insert(n.clone(), Ty::Prp(Box::new(PTerm::Var(n.clone()))));
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
        // Visible `using` binders win over instances, nearest first (the
        // nearest explicitly introduced provider binds the name).
        for (local, binder) in scope.iter().rev() {
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
        // Runtime capabilities are lexical evidence only: there are no
        // `Io` instances, and nothing global is searched (decision D21).
        if is_capability_class(&wanted_z.class) {
            return Err(Error::at(
                span,
                format!(
                    "no {wanted_z} capability in scope; supply one lexically with `with (io = ...) in` or `f @{{io = ...}}`, or take it in a `using` clause"
                ),
            ));
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
        // Collection literals default their container when nothing else
        // determines it (surface grammar local amendments section 10):
        // `[..]` unconstrained is a `List`, `{..}` a `Map`.
        if class == "FromList" && wanted_z.args.len() == 2 {
            if matches!(self.uni.zonk(&wanted_z.args[0]), Ty::Meta(_)) {
                let list = Ty::Adt("List".into(), vec![wanted_z.args[1].clone()]);
                self.uni
                    .unify(&wanted_z.args[0], &list)
                    .map_err(|m| Error::at(span, m))?;
            }
        } else if class == "FromMap" && wanted_z.args.len() == 3 {
            if matches!(self.uni.zonk(&wanted_z.args[0]), Ty::Meta(_)) {
                let map = Ty::Adt(
                    "Map".into(),
                    vec![wanted_z.args[1].clone(), wanted_z.args[2].clone()],
                );
                self.uni
                    .unify(&wanted_z.args[0], &map)
                    .map_err(|m| Error::at(span, m))?;
            }
        }
        let wanted_z = self.uni.zonk_class(wanted);
        let wanted_app = Ty::Adt(class.clone(), wanted_z.args.clone());
        let mut matches: Vec<(usize, HashMap<String, Ty>, Unifier)> = Vec::new();
        if let Some(idxs) = self.instances.get(&class).cloned() {
            for i in idxs {
                if !self.instance_list[i].implicit && Some(i) != self.self_instance {
                    continue;
                }
                // A parameterized module's declared instance needs its
                // module's parameters: it resolves only inside that module,
                // where the parameters are the definition's leading
                // dictionary binders (module parameters as module
                // arguments). Each instantiation publishes its own copy.
                if self.instance_list[i].base
                    && self.instance_list[i].module != self.current_module
                {
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
                // A unique match that leaves an instance parameter bound
                // only to an undetermined literal defers to literal
                // defaulting (decision D7): `[1]` with no other constraint
                // is a `List Int`, not an unsolved literal type.
                let lits = self.literal_metas();
                let undetermined = sub.values().any(|t| {
                    matches!(t, Ty::Meta(m) if lits.contains(&EitherMeta::Type(*m)))
                });
                if undetermined {
                    return Ok(Ambiguous);
                }
                let (ctx, base_id, base) = {
                    let inst = &self.instance_list[*idx];
                    (inst.ctx.clone(), inst.base_id.clone(), inst.base)
                };
                self.uni = trial.clone();
                let mut subs = Vec::new();
                // The module's own instance dictionary closes over the
                // module's parameters (the definition's leading binders).
                if base {
                    for (_, l, _) in &self.cur_mod_params {
                        subs.push(DictExpr::Binder(*l));
                    }
                }
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
            ECore::LitFold { idx, body } => ECore::LitFold {
                idx: *idx,
                body: Box::new(self.fill_holes(body, sol)?),
            },
            ECore::Hole(id) => match sol.get(id) {
                Some(d) => dict_expr_ecore(d),
                None => return Err(Error::plain("internal error: unsolved dictionary hole")),
            },
            ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_) | ECore::Lit(_) | ECore::Bool(_)
            | ECore::Unit | ECore::Var(_) | ECore::Global(_)
            | ECore::Ctor(_) | ECore::Prim(_) | ECore::Foreign(_) | ECore::QuoteHole(_)
            | ECore::IdxPending(_) | ECore::Fault
            | ECore::SyntaxLit(_) | ECore::StrLit(_) => e.clone(),
            ECore::BoxAlloc => ECore::BoxAlloc,
            ECore::BoxInit(l, v) => {
                ECore::BoxInit(*l, Box::new(self.fill_holes(v, sol)?))
            }
            ECore::RefSet(l, v) => ECore::RefSet(*l, Box::new(self.fill_holes(v, sol)?)),
            ECore::BoxGet(v) => ECore::BoxGet(Box::new(self.fill_holes(v, sol)?)),
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
            ECore::DictProj(d, i) => {
                ECore::DictProj(Box::new(self.fill_holes(d, sol)?), *i)
            }
            ECore::Proj(d, i) => ECore::Proj(Box::new(self.fill_holes(d, sol)?), i.clone()),
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
                CBranch {
                    weight: t.weight,
                    body: Box::new(self.fill_holes(&t.body, sol)?),
                },
                CBranch {
                    weight: f.weight,
                    body: Box::new(self.fill_holes(&f.body, sol)?),
                },
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
    fn fn_ref(&self, v: &ValueInfo, span: ast::Span) -> Result<ECore> {
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
            let mut core = ECore::Global(fn_id(&v.module, &v.name));
            if !self.cur_mod_params.is_empty() {
                core = app_spine(
                    core,
                    self.cur_mod_params.iter().map(|(_, l, _)| ECore::Var(*l)).collect(),
                );
            }
            return Ok(core);
        }
        Ok(ECore::Global(fn_id(&v.module, &v.name)))
    }

    /// A module parameter used as a term (a capitalized dictionary
    /// parameter like the example's `Seq` is a constructor-shaped name
    /// that resolves to the parameter's binder first): the binder local
    /// and the parameter's type.
    fn module_param_use(&self, p: &ast::ConPath) -> Option<(LocalId, Ty)> {
        if p.segments.len() != 1 {
            return None;
        }
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
        crate::core::Core::InstDict(id, subs) => {
            let inner: Vec<String> = subs.iter().map(|s| arg_identity(s, "")).collect();
            format!("dict:{id}({})", inner.join(","))
        }
        crate::core::Core::Global(g) => format!("val:{g}"),
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
include!("check_decls.rs");
