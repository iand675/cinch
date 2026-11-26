//! Core-IR Forward-Mode (JVP) and Reverse-Mode (VJP) Automatic Differentiation.
//!
//! Matches Futhark's `jvp`, `jvp2`, `vjp`, and `vjp2` semantics directly as a
//! source-to-source Core IR transformation running after specialization &
//! simplification and before Typed Array IR optimization, reference counting,
//! and LLVM codegen:
//!
//! - **Forward mode (`jvp2 f x dx -> (y, dy)`)**: pairs every active primal
//!   value with its tangent across scalars (`F32`, `F64`), tuples, ADTs/views
//!   (`NDView`, `View1`/`View2`/`View3`, user records), `Option`, `If`/`Case`,
//!   closures, recursion, tail-recursive `Join`/`Jump` loops, and parallel
//!   array primitives.
//! - **Reverse mode (`vjp2 f x dy -> (y, dx)`)**: generates a direct ANF
//!   forward+reverse sweep with hoisted pullback closures for branches, calls,
//!   and loops, accumulating cotangents in reverse topological order with exact
//!   parallel array adjoints (`dot`, `reduce`, `zip`, `map`, `row-reduce`,
//!   `gather`/`scatter-add`, exact-boundary `stencil2d-adjoint`).
//! - **Higher-order AD**: runs in an innermost-first fixed-point loop, so
//!   `deriv (deriv f)` (forward-over-forward) and `hvp f x v = jvp (\z -> vjp f z 1.0) x v`
//!   (forward-over-reverse Hessian-vector products) lower cleanly to pure Core.

use std::collections::{HashMap, HashSet};

use crate::arrays::{ArrayComputeOp, ArrayOp};
use crate::core::{
    AdOp, CAlt, CBranch, CPat, Class, Core, FnDef, GlobalId, JoinId, LocalId, PrimOp, Program,
    UnitRepr,
};
use crate::repr::spec_name;
use crate::{Error, Result};

// ---------------------------------------------------------------------------
// Differentiable type structure (inferred over specialized Core IR)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AdType {
    /// Non-differentiable value (Int, Nat, Bool, Unit, I32, U8, ShapeWitness, etc.).
    NonDiff,
    /// Single-precision IEEE 754 float (stored in a `Class::Raw` 64-bit word).
    F32,
    /// Double-precision IEEE 754 float (stored in a `Class::Raw` 64-bit word).
    F64,
    /// Packed `Array F32` (`Class::Obj`).
    ArrayF32,
    /// Packed `Array F64` (`Class::Obj`).
    ArrayF64,
    /// Structural tuple `(t0, t1, ...)` (`Tuple.N@...`, tag 0).
    Tuple(Vec<AdType>),
    /// Single-constructor product/record/view ADT (e.g., `MkNDView`, `MkView1`,
    /// `MkView2`, `MkView3`, user records) where some fields are differentiable.
    Product {
        ctor: GlobalId,
        fields: Vec<AdType>,
        field_classes: Vec<Class>,
    },
    /// `Option a` (tag 0 = `None`, tag 1 = `Some`) with differentiable payload `a`.
    Option(Box<AdType>),
    /// Active closure whose captured environment or parameters carry tangents.
    Closure,
}

impl AdType {
    pub fn is_active(&self) -> bool {
        match self {
            AdType::NonDiff => false,
            AdType::F32 | AdType::F64 | AdType::ArrayF32 | AdType::ArrayF64 | AdType::Closure => {
                true
            }
            AdType::Tuple(fs) => fs.iter().any(AdType::is_active),
            AdType::Product { fields, .. } => fields.iter().any(AdType::is_active),
            AdType::Option(inner) => inner.is_active(),
        }
    }

    pub fn class(&self, fallback: Class) -> Class {
        match self {
            AdType::F32 | AdType::F64 => Class::Raw,
            AdType::ArrayF32
            | AdType::ArrayF64
            | AdType::Tuple(_)
            | AdType::Product { .. }
            | AdType::Option(_)
            | AdType::Closure => Class::Obj,
            AdType::NonDiff => fallback,
        }
    }

    fn merge(&self, other: &AdType) -> AdType {
        match (self, other) {
            (AdType::NonDiff, x) | (x, AdType::NonDiff) => x.clone(),
            (AdType::Tuple(a), AdType::Tuple(b)) if a.len() == b.len() => {
                AdType::Tuple(a.iter().zip(b).map(|(x, y)| x.merge(y)).collect())
            }
            (
                AdType::Product { ctor: c1, fields: f1, field_classes: fc1 },
                AdType::Product { ctor: c2, fields: f2, .. },
            ) if c1 == c2 && f1.len() == f2.len() => AdType::Product {
                ctor: c1.clone(),
                fields: f1.iter().zip(f2).map(|(x, y)| x.merge(y)).collect(),
                field_classes: fc1.clone(),
            },
            (AdType::Option(a), AdType::Option(b)) => AdType::Option(Box::new(a.merge(b))),
            (a, _) => a.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Scalar float operation classification
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FloatPrec {
    F32,
    F64,
}

impl FloatPrec {
    fn ad_ty(self) -> AdType {
        match self {
            FloatPrec::F32 => AdType::F32,
            FloatPrec::F64 => AdType::F64,
        }
    }

    fn lit(self, v: f64) -> Core {
        match self {
            FloatPrec::F32 => Core::Int((v as f32).to_bits() as u64 as i64),
            FloatPrec::F64 => Core::Int(v.to_bits() as i64),
        }
    }

    fn add_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.add-f32".into(),
            FloatPrec::F64 => "Data.Machine.add-f64".into(),
        })
    }

    fn sub_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.sub-f32".into(),
            FloatPrec::F64 => "Data.Machine.sub-f64".into(),
        })
    }

    fn mul_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.mul-f32".into(),
            FloatPrec::F64 => "Data.Machine.mul-f64".into(),
        })
    }

    fn div_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.div-f32".into(),
            FloatPrec::F64 => "Data.Machine.div-f64".into(),
        })
    }

    fn neg_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.neg-f32".into(),
            FloatPrec::F64 => "Data.Machine.neg-f64".into(),
        })
    }

    fn fge_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.Machine.fge-f32".into(),
            FloatPrec::F64 => "Data.Machine.fge-f64".into(),
        })
    }

    fn sin_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.AD.sin-f32".into(),
            FloatPrec::F64 => "Data.AD.sin-f64".into(),
        })
    }

    fn cos_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.AD.cos-f32".into(),
            FloatPrec::F64 => "Data.AD.cos-f64".into(),
        })
    }

    fn exp_id(self) -> GlobalId {
        GlobalId(match self {
            FloatPrec::F32 => "Data.AD.exp-f32".into(),
            FloatPrec::F64 => "Data.AD.exp-f64".into(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScalarFloatOp {
    Add(FloatPrec),
    Sub(FloatPrec),
    Mul(FloatPrec),
    Div(FloatPrec),
    Neg(FloatPrec),
    Abs(FloatPrec),
    Sqrt(FloatPrec),
    Sin(FloatPrec),
    Cos(FloatPrec),
    Exp(FloatPrec),
    Log(FloatPrec),
    Cmp(FloatPrec),
}

fn classify_scalar_float_op(name: &str) -> Option<ScalarFloatOp> {
    let base = name.split('@').next().unwrap_or(name);
    let leaf = base.rsplit('.').next().unwrap_or(base);
    match leaf {
        "add-f32" => Some(ScalarFloatOp::Add(FloatPrec::F32)),
        "sub-f32" => Some(ScalarFloatOp::Sub(FloatPrec::F32)),
        "mul-f32" => Some(ScalarFloatOp::Mul(FloatPrec::F32)),
        "div-f32" => Some(ScalarFloatOp::Div(FloatPrec::F32)),
        "neg-f32" => Some(ScalarFloatOp::Neg(FloatPrec::F32)),
        "abs-f32" => Some(ScalarFloatOp::Abs(FloatPrec::F32)),
        "sqrt-f32" => Some(ScalarFloatOp::Sqrt(FloatPrec::F32)),
        "sin-f32" => Some(ScalarFloatOp::Sin(FloatPrec::F32)),
        "cos-f32" => Some(ScalarFloatOp::Cos(FloatPrec::F32)),
        "exp-f32" => Some(ScalarFloatOp::Exp(FloatPrec::F32)),
        "log-f32" => Some(ScalarFloatOp::Log(FloatPrec::F32)),
        "feq-f32" | "fne-f32" | "flt-f32" | "fle-f32" | "fgt-f32" | "fge-f32" => {
            Some(ScalarFloatOp::Cmp(FloatPrec::F32))
        }
        "add-f64" => Some(ScalarFloatOp::Add(FloatPrec::F64)),
        "sub-f64" => Some(ScalarFloatOp::Sub(FloatPrec::F64)),
        "mul-f64" => Some(ScalarFloatOp::Mul(FloatPrec::F64)),
        "div-f64" => Some(ScalarFloatOp::Div(FloatPrec::F64)),
        "neg-f64" => Some(ScalarFloatOp::Neg(FloatPrec::F64)),
        "abs-f64" => Some(ScalarFloatOp::Abs(FloatPrec::F64)),
        "sqrt-f64" => Some(ScalarFloatOp::Sqrt(FloatPrec::F64)),
        "sin-f64" => Some(ScalarFloatOp::Sin(FloatPrec::F64)),
        "cos-f64" => Some(ScalarFloatOp::Cos(FloatPrec::F64)),
        "exp-f64" => Some(ScalarFloatOp::Exp(FloatPrec::F64)),
        "log-f64" => Some(ScalarFloatOp::Log(FloatPrec::F64)),
        "feq-f64" | "fne-f64" | "flt-f64" | "fle-f64" | "fgt-f64" | "fge-f64" => {
            Some(ScalarFloatOp::Cmp(FloatPrec::F64))
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Lower every `jvp2` / `vjp2` automatic differentiation call in `program`
/// into pure, leak-free, array-IR-optimizable `Core` IR.
pub fn lower_program(program: &mut Program) -> Result<()> {
    // Quick exit when no AD primitive is referenced anywhere in the program.
    if !program_has_ad(program) {
        return Ok(());
    }

    // Inline small AD wrapper helpers (such as `Data.AD.jvp`, `Data.AD.vjp`,
    // `Data.AD.grad-f32`, `Data.AD.grad-f64`, `Data.AD.deriv-*`, `Data.AD.hvp-*`,
    // and user-written wrappers like `my-grad f x = vjp f x 1.0`) and specialize
    // lambda-lifted AD closures capturing function parameters (such as
    // `hvp-f64`'s `\z -> vjp f z 1.0`) so every `jvp2`/`vjp2` call sees its
    // concrete target function directly.
    for _ in 0..8 {
        let mut sub = false;
        if inline_ad_wrappers(program) {
            sub = true;
        }
        if specialize_ad_lams(program) {
            sub = true;
        }
        if specialize_ho_array_helpers(program) {
            sub = true;
        }
        if !sub {
            break;
        }
    }

    let ad_stub_ids: HashSet<GlobalId> = program
        .fns
        .iter()
        .filter(|f| matches!(f.prim, Some(PrimOp::Ad { .. })))
        .map(|f| f.id.clone())
        .collect();

    // Fixed-point innermost-first lowering so nested / higher-order AD
    // (`deriv (deriv f)`, `hvp f x v = jvp (\z -> vjp f z 1.0) x v`, etc.)
    // lowers inner AD sites into pure Core before differentiating the outer site.
    let mut ctx = AdContext::new(program);
    for _round in 0..16 {
        let mut progress = false;
        let fn_ids: Vec<GlobalId> = ctx.fn_order.clone();
        for fid in fn_ids {
            if ctx.is_ad_prim_stub(&fid) {
                continue;
            }
            if !ctx.fn_contains_ad(&fid) {
                continue;
            }
            let body = ctx.fns[&fid].body.clone();
            let param_classes = ctx.fns[&fid].repr.params.clone();
            let mut next_local = max_local_in_fn(&ctx.fns[&fid]) + 1;
            let mut local_classes: HashMap<LocalId, Class> = param_classes
                .iter()
                .enumerate()
                .map(|(i, c)| (LocalId(i), *c))
                .collect();
            collect_let_classes(&body, &mut local_classes);
            let mut changed = false;
            let new_body = ctx.lower_ad_calls_in_expr(
                &body,
                &local_classes,
                &mut next_local,
                &mut changed,
            )?;
            if changed {
                ctx.fns.get_mut(&fid).unwrap().body = new_body;
                progress = true;
            }
        }
        if !progress {
            break;
        }
        // Re-inline wrappers and beta-reduce known tuples/closures exposed after an
        // inner AD pass so outer AD passes (such as forward-over-reverse HVP) see
        // direct Global calls instead of opaque pullback closures.
        ctx.flush_into(program);
        for _ in 0..4 {
            let mut sub = false;
            if inline_ad_wrappers(program) {
                sub = true;
            }
            if specialize_ad_lams(program) {
                sub = true;
            }
            if beta_reduce_program(program) {
                sub = true;
            }
            if !sub {
                break;
            }
        }
        ctx = AdContext::new(program);
    }

    ctx.flush_into(program);
    let _ = beta_reduce_program(program);

    // Ensure every `prim` referenced by generated AD code (such as `Data.Machine.neg-f32`,
    // `Data.AD.cos-f32`, etc.) has its stub unit present in `program.fns` even if the
    // user program did not call it directly prior to AD.
    restore_referenced_prim_stubs(program);

    // Strip the now-unused `PrimOp::Ad` stub units and any now-dead AD wrapper definitions
    // (such as `Data.AD.deriv-f64`, `Data.AD.jvp`, `Data.AD.vjp`, `Data.AD.hvp-f64`)
    // that were fully inlined at all live call sites.
    program
        .fns
        .retain(|f| !matches!(f.prim, Some(PrimOp::Ad { .. })));

    // Compute functions reachable from entry points (`Main.*` or any non-wrapper function
    // that doesn't itself contain an unlowered AD stub). Any unreachable wrapper whose body
    // still references an AD stub is simply dead after inlining and can be dropped cleanly.
    let live_ids = reachable_from_roots(program, &ad_stub_ids);
    program
        .fns
        .retain(|f| live_ids.contains(&f.id) || !expr_contains_ad(&f.body, &ad_stub_ids));

    // Verify no live function still contains an unlowered AD call.
    for f in &program.fns {
        if expr_contains_ad(&f.body, &ad_stub_ids) {
            return Err(Error::plain(format!(
                "automatic differentiation could not lower an AD call in `{}`: target function must be statically resolvable",
                f.id.0
            )));
        }
    }
    Ok(())
}

fn program_has_ad(program: &Program) -> bool {
    let ad_stubs: HashSet<GlobalId> = program
        .fns
        .iter()
        .filter(|f| matches!(f.prim, Some(PrimOp::Ad { .. })))
        .map(|f| f.id.clone())
        .collect();
    if ad_stubs.is_empty() {
        return program
            .fns
            .iter()
            .any(|f| expr_contains_ad(&f.body, &ad_stubs));
    }
    program
        .fns
        .iter()
        .any(|f| !ad_stubs.contains(&f.id) && expr_contains_ad(&f.body, &ad_stubs))
}

fn expr_contains_ad(e: &Core, ad_stubs: &HashSet<GlobalId>) -> bool {
    match e {
        Core::Prim(PrimOp::Ad { .. }) => true,
        Core::Global(g, _) if ad_stubs.contains(g) => true,
        Core::App(h, args) => {
            expr_contains_ad(h, ad_stubs) || args.iter().any(|a| expr_contains_ad(a, ad_stubs))
        }
        Core::Let(_, _, rhs, body) => {
            expr_contains_ad(rhs, ad_stubs) || expr_contains_ad(body, ad_stubs)
        }
        Core::If(c, t, f) => {
            expr_contains_ad(c, ad_stubs)
                || expr_contains_ad(&t.body, ad_stubs)
                || expr_contains_ad(&f.body, ad_stubs)
        }
        Core::Case { scrut, alts } => {
            expr_contains_ad(scrut, ad_stubs)
                || alts.iter().any(|a| expr_contains_ad(&a.body, ad_stubs))
        }
        Core::Join { body, rest, .. } => {
            expr_contains_ad(body, ad_stubs) || expr_contains_ad(rest, ad_stubs)
        }
        Core::Jump { args, .. } | Core::InstDict(_, args, _) => {
            args.iter().any(|a| expr_contains_ad(a, ad_stubs))
        }
        Core::Lam { env, .. } => env.iter().any(|a| expr_contains_ad(a, ad_stubs)),
        Core::Proj(x, _)
        | Core::DictProj(x, _, _, _)
        | Core::Demand(x, _)
        | Core::BoxInit(_, x, _)
        | Core::BoxGet(x, _)
        | Core::BoxClear(x, _)
        | Core::RefSet(_, x, _)
        | Core::LazyInit(_, x) => expr_contains_ad(x, ad_stubs),
        Core::Dup(_, _, b) | Core::Drop(_, _, b) | Core::Release { body: b, .. } => {
            expr_contains_ad(b, ad_stubs)
        }
        _ => false,
    }
}

/// Inline non-recursive wrapper functions that contain an AD call (like `jvp`,
/// `vjp`, `grad-f32`, `grad-f64`, `deriv-f32`, `deriv-f64`, `hvp-f32`, `hvp-f64`)
/// at their call sites so the AD pass sees the concrete function argument `f`.
fn inline_ad_wrappers(program: &mut Program) -> bool {
    let ad_stubs: HashSet<GlobalId> = program
        .fns
        .iter()
        .filter(|f| matches!(f.prim, Some(PrimOp::Ad { .. })))
        .map(|f| f.id.clone())
        .collect();
    let mut wrappers: HashMap<GlobalId, FnDef> = HashMap::new();
    for f in &program.fns {
        if f.prim.is_none()
            && f.ctor.is_none()
            && !f.params.is_empty()
            && expr_contains_ad(&f.body, &ad_stubs)
            && !expr_calls_global(&f.body, &f.id)
        {
            wrappers.insert(f.id.clone(), f.clone());
        }
    }
    if wrappers.is_empty() {
        return false;
    }
    let mut changed = false;
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let mut next_local = max_local_in_fn(f) + 1;
        let mut next_join = max_join_in_expr(&f.body) + 1;
        let mut local_changed = false;
        let new_body = inline_calls_in_expr(
            &f.body,
            &wrappers,
            &mut next_local,
            &mut next_join,
            &mut local_changed,
        );
        if local_changed {
            f.body = new_body;
            changed = true;
        }
    }
    changed
}

fn expr_calls_global(e: &Core, target: &GlobalId) -> bool {
    match e {
        Core::Global(g, _) => g == target,
        Core::Lam { fn_id, env, .. } => {
            fn_id == target || env.iter().any(|a| expr_calls_global(a, target))
        }
        Core::App(h, args) => {
            expr_calls_global(h, target) || args.iter().any(|a| expr_calls_global(a, target))
        }
        Core::Let(_, _, r, b) => expr_calls_global(r, target) || expr_calls_global(b, target),
        Core::If(c, t, f) => {
            expr_calls_global(c, target)
                || expr_calls_global(&t.body, target)
                || expr_calls_global(&f.body, target)
        }
        Core::Case { scrut, alts } => {
            expr_calls_global(scrut, target)
                || alts.iter().any(|a| expr_calls_global(&a.body, target))
        }
        Core::Join { body, rest, .. } => {
            expr_calls_global(body, target) || expr_calls_global(rest, target)
        }
        Core::Jump { args, .. } | Core::InstDict(_, args, _) => {
            args.iter().any(|a| expr_calls_global(a, target))
        }
        _ => false,
    }
}

fn inline_calls_in_expr(
    e: &Core,
    wrappers: &HashMap<GlobalId, FnDef>,
    next_local: &mut usize,
    next_join: &mut usize,
    changed: &mut bool,
) -> Core {
    match e {
        Core::App(h, args) => {
            let h2 = inline_calls_in_expr(h, wrappers, next_local, next_join, changed);
            let args2: Vec<Core> = args
                .iter()
                .map(|a| inline_calls_in_expr(a, wrappers, next_local, next_join, changed))
                .collect();
            if let Core::Global(g, _) = &h2 {
                if let Some(w) = wrappers.get(g) {
                    if args2.len() == w.params.len() {
                        *changed = true;
                        let mut renamed = freshen_expr(&w.body, next_local, next_join);
                        // Bind each parameter to its argument via `Let`.
                        for (i, arg) in args2.into_iter().enumerate().rev() {
                            let fresh = LocalId(*next_local);
                            *next_local += 1;
                            let cls = w.repr.params.get(i).copied().unwrap_or(Class::Obj);
                            renamed = subst_local_in_expr(&renamed, LocalId(i), &Core::Var(fresh));
                            renamed = Core::Let(fresh, cls, Box::new(arg), Box::new(renamed));
                        }
                        return renamed;
                    }
                }
            }
            Core::App(Box::new(h2), args2)
        }
        Core::Let(x, c, rhs, body) => Core::Let(
            *x,
            *c,
            Box::new(inline_calls_in_expr(rhs, wrappers, next_local, next_join, changed)),
            Box::new(inline_calls_in_expr(body, wrappers, next_local, next_join, changed)),
        ),
        Core::If(c, t, f) => Core::If(
            Box::new(inline_calls_in_expr(c, wrappers, next_local, next_join, changed)),
            CBranch {
                weight: t.weight,
                body: Box::new(inline_calls_in_expr(
                    &t.body, wrappers, next_local, next_join, changed,
                )),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(inline_calls_in_expr(
                    &f.body, wrappers, next_local, next_join, changed,
                )),
            },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(inline_calls_in_expr(
                scrut, wrappers, next_local, next_join, changed,
            )),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(inline_calls_in_expr(&a.body, wrappers, next_local, next_join, changed)),
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(inline_calls_in_expr(
                body, wrappers, next_local, next_join, changed,
            )),
            rest: Box::new(inline_calls_in_expr(
                rest, wrappers, next_local, next_join, changed,
            )),
        },
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args
                .iter()
                .map(|a| inline_calls_in_expr(a, wrappers, next_local, next_join, changed))
                .collect(),
        },
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env
                .iter()
                .map(|a| inline_calls_in_expr(a, wrappers, next_local, next_join, changed))
                .collect(),
            classes: classes.clone(),
        },
        other => other.clone(),
    }
}

// ---------------------------------------------------------------------------
// AD Context & Unit Management
// ---------------------------------------------------------------------------

struct AdContext {
    fns: HashMap<GlobalId, FnDef>,
    fn_order: Vec<GlobalId>,
    /// Memoized forward-mode companions: `(orig_fn_id, arg_ad_types) -> (jvp_fn_id, ret_ad_type)`.
    jvp_cache: HashMap<(GlobalId, Vec<AdType>), (GlobalId, AdType)>,
    /// Memoized reverse-mode companions: `(orig_fn_id, arg_ad_types) -> (vjp_fn_id, ret_ad_type)`.
    vjp_cache: HashMap<(GlobalId, Vec<AdType>), (GlobalId, AdType)>,
    /// Memoized direct reverse-mode entrypoints: `(orig_fn_id, arg_ad_types) -> (vjp_direct_id, ret_ad_type)`.
    vjp_direct_cache: HashMap<(GlobalId, Vec<AdType>), (GlobalId, AdType)>,
    fresh_unit_counter: usize,
    /// Constructor metadata: `ctor_id -> (tag, field_classes)`.
    ctor_info: HashMap<GlobalId, (usize, Vec<Class>)>,
    /// Single-constructor ADTs indexed by field count & field classes (for Case inference).
    ad_stubs: HashMap<GlobalId, (AdOp, [Class; 2])>,
    /// Map from `ArrayComputeOp` -> existing unit id (or synthesized on demand).
    compute_units: HashMap<ArrayComputeOp, GlobalId>,
}

impl AdContext {
    fn new(program: &Program) -> Self {
        let mut fns = HashMap::new();
        let mut fn_order = Vec::new();
        let mut ctor_info = HashMap::new();
        let mut ad_stubs = HashMap::new();
        let mut compute_units = HashMap::new();
        for f in &program.fns {
            if let Some((tag, _arity)) = f.ctor {
                ctor_info.insert(f.id.clone(), (tag, f.repr.fields.clone()));
            }
            if let Some(PrimOp::Ad { op, classes }) = f.prim {
                ad_stubs.insert(f.id.clone(), (op, classes));
            }
            if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                compute_units.entry(op).or_insert_with(|| f.id.clone());
            }
            fn_order.push(f.id.clone());
            fns.insert(f.id.clone(), f.clone());
        }
        Self {
            fns,
            fn_order,
            jvp_cache: HashMap::new(),
            vjp_cache: HashMap::new(),
            vjp_direct_cache: HashMap::new(),
            fresh_unit_counter: 0,
            ctor_info,
            ad_stubs,
            compute_units,
        }
    }

    fn flush_into(mut self, program: &mut Program) {
        program.fns.clear();
        for id in self.fn_order {
            if let Some(f) = self.fns.remove(&id) {
                program.fns.push(f);
            }
        }
    }

    fn is_ad_prim_stub(&self, id: &GlobalId) -> bool {
        self.ad_stubs.contains_key(id)
    }

    fn ad_stub_set(&self) -> HashSet<GlobalId> {
        self.ad_stubs.keys().cloned().collect()
    }

    fn fn_contains_ad(&self, id: &GlobalId) -> bool {
        let stubs = self.ad_stub_set();
        self.fns
            .get(id)
            .is_some_and(|f| expr_contains_ad(&f.body, &stubs))
    }

    fn add_fn(&mut self, def: FnDef) {
        let id = def.id.clone();
        if let Some((tag, _)) = def.ctor {
            self.ctor_info.insert(id.clone(), (tag, def.repr.fields.clone()));
        }
        if !self.fns.contains_key(&id) {
            self.fn_order.push(id.clone());
        }
        self.fns.insert(id, def);
    }

    fn fresh_unit_id(&mut self, base: &str, suffix: &str) -> GlobalId {
        let n = self.fresh_unit_counter;
        self.fresh_unit_counter += 1;
        GlobalId(format!("{base}${suffix}{n}"))
    }

    /// Ensure a specialized `Tuple.N@<classes>` constructor unit exists and return its `GlobalId`.
    fn ensure_tuple_ctor(&mut self, classes: &[Class]) -> GlobalId {
        let n = classes.len();
        debug_assert!(n >= 2);
        let id = GlobalId(spec_name(&format!("Tuple.{n}"), classes));
        if !self.fns.contains_key(&id) {
            self.add_fn(FnDef {
                id: id.clone(),
                params: (0..n).map(|i| format!("f{i}")).collect(),
                body: Core::Unit,
                prim: None,
                ctor: Some((0, n)),
                hints: Default::default(),
                meta: Default::default(),
                repr: UnitRepr {
                    tparams: 0,
                    params: classes.to_vec(),
                    fields: classes.to_vec(),
                    data: None,
                    result: Some(Class::Obj),
                    dictionary_values: Vec::new(),
                    ..Default::default()
                },
            });
        }
        id
    }

    fn ensure_option_ctors(&mut self, payload_cls: Class) -> (GlobalId, GlobalId) {
        let none_id = GlobalId(spec_name("Data.Core.None", &[payload_cls]));
        if !self.fns.contains_key(&none_id) {
            self.add_fn(FnDef {
                id: none_id.clone(),
                params: Vec::new(),
                body: Core::Unit,
                prim: None,
                ctor: Some((0, 0)),
                hints: Default::default(),
                meta: Default::default(),
                repr: UnitRepr {
                    tparams: 0,
                    params: Vec::new(),
                    fields: Vec::new(),
                    data: Some("Data.Core.Option".into()),
                    result: Some(Class::Obj),
                    dictionary_values: Vec::new(),
                    ..Default::default()
                },
            });
        }
        let some_id = GlobalId(spec_name("Data.Core.Some", &[payload_cls]));
        if !self.fns.contains_key(&some_id) {
            self.add_fn(FnDef {
                id: some_id.clone(),
                params: vec!["f0".into()],
                body: Core::Unit,
                prim: None,
                ctor: Some((1, 1)),
                hints: Default::default(),
                meta: Default::default(),
                repr: UnitRepr {
                    tparams: 0,
                    params: vec![payload_cls],
                    fields: vec![payload_cls],
                    data: Some("Data.Core.Option".into()),
                    result: Some(Class::Obj),
                    dictionary_values: Vec::new(),
                    ..Default::default()
                },
            });
        }
        (none_id, some_id)
    }

    fn ensure_compute_op(&mut self, op: ArrayComputeOp) -> Core {
        if let Some(id) = self.compute_units.get(&op).cloned() {
            return Core::Global(id, Vec::new());
        }
        Core::Prim(PrimOp::ArrayCompute { op, classes: [Class::Raw; 2] })
    }

    fn array_len_call(&self, arr: Core) -> Core {
        Core::App(
            Box::new(Core::Prim(PrimOp::Array {
                op: ArrayOp::Len,
                classes: [Class::Raw; 2],
                packed: None,
            })),
            vec![arr],
        )
    }

    fn mk_tuple(&mut self, elems: Vec<(Core, Class)>) -> Core {
        let classes: Vec<Class> = elems.iter().map(|(_, c)| *c).collect();
        let vals: Vec<Core> = elems.into_iter().map(|(v, _)| v).collect();
        let ctor = self.ensure_tuple_ctor(&classes);
        Core::App(Box::new(Core::Ctor(ctor, Vec::new())), vals)
    }

    fn call_global(id: GlobalId, args: Vec<Core>) -> Core {
        Core::App(Box::new(Core::Global(id, Vec::new())), args)
    }

    fn unwrap_option_obj(&mut self, opt_expr: Core, fallback: Core, next_local: &mut usize) -> Core {
        let v = LocalId(*next_local);
        *next_local += 1;
        Core::Case {
            scrut: Box::new(opt_expr),
            alts: vec![
                CAlt {
                    pat: CPat::Con { tag: 0, fields: Vec::new(), classes: Vec::new() },
                    weight: None,
                    body: Box::new(fallback),
                },
                CAlt {
                    pat: CPat::Con { tag: 1, fields: vec![v], classes: vec![Class::Obj] },
                    weight: None,
                    body: Box::new(Core::Var(v)),
                },
            ],
        }
    }

    fn wrap_some_obj(&mut self, val: Core) -> Core {
        let (_, some_id) = self.ensure_option_ctors(Class::Obj);
        Core::App(Box::new(Core::Ctor(some_id, Vec::new())), vec![val])
    }

    /// Build a zero tangent / cotangent matching the shape of `primal`.
    fn zero_like(&mut self, primal: Core, ty: &AdType, next_local: &mut usize) -> Core {
        match ty {
            AdType::NonDiff => primal,
            AdType::F32 => FloatPrec::F32.lit(0.0),
            AdType::F64 => FloatPrec::F64.lit(0.0),
            AdType::ArrayF32 => {
                let len = self.array_len_call(primal);
                let fill = self.ensure_compute_op(ArrayComputeOp::FillF32);
                Core::App(Box::new(fill), vec![len, FloatPrec::F32.lit(0.0)])
            }
            AdType::ArrayF64 => {
                let len = self.array_len_call(primal);
                let fill = self.ensure_compute_op(ArrayComputeOp::FillF64);
                Core::App(Box::new(fill), vec![len, FloatPrec::F64.lit(0.0)])
            }
            AdType::Tuple(fs) => {
                let mut binders = Vec::with_capacity(fs.len());
                let mut classes = Vec::with_capacity(fs.len());
                for f in fs {
                    let l = LocalId(*next_local);
                    *next_local += 1;
                    binders.push(l);
                    classes.push(f.class(Class::Raw));
                }
                let elems: Vec<(Core, Class)> = binders
                    .iter()
                    .zip(fs.iter())
                    .zip(classes.iter())
                    .map(|((&l, ft), &c)| (self.zero_like(Core::Var(l), ft, next_local), c))
                    .collect();
                let tup = self.mk_tuple(elems);
                Core::Case {
                    scrut: Box::new(primal),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag: 0, fields: binders, classes },
                        weight: None,
                        body: Box::new(tup),
                    }],
                }
            }
            AdType::Product { ctor, fields, field_classes } => {
                let tag = self.ctor_info.get(ctor).map(|(t, _)| *t).unwrap_or(0);
                let mut binders = Vec::with_capacity(fields.len());
                for _ in fields {
                    let l = LocalId(*next_local);
                    *next_local += 1;
                    binders.push(l);
                }
                let args: Vec<Core> = binders
                    .iter()
                    .zip(fields.iter())
                    .map(|(&l, ft)| self.zero_like(Core::Var(l), ft, next_local))
                    .collect();
                let rebuilt = Core::App(Box::new(Core::Ctor(ctor.clone(), Vec::new())), args);
                Core::Case {
                    scrut: Box::new(primal),
                    alts: vec![CAlt {
                        pat: CPat::Con {
                            tag,
                            fields: binders,
                            classes: field_classes.clone(),
                        },
                        weight: None,
                        body: Box::new(rebuilt),
                    }],
                }
            }
            AdType::Option(inner) => {
                let icls = inner.class(Class::Obj);
                let (none_id, some_id) = self.ensure_option_ctors(icls);
                let v = LocalId(*next_local);
                *next_local += 1;
                let z = self.zero_like(Core::Var(v), inner, next_local);
                Core::Case {
                    scrut: Box::new(primal),
                    alts: vec![
                        CAlt {
                            pat: CPat::Con { tag: 0, fields: Vec::new(), classes: Vec::new() },
                            weight: None,
                            body: Box::new(Core::Ctor(none_id, Vec::new())),
                        },
                        CAlt {
                            pat: CPat::Con { tag: 1, fields: vec![v], classes: vec![icls] },
                            weight: None,
                            body: Box::new(Core::App(Box::new(Core::Ctor(some_id, Vec::new())), vec![z])),
                        },
                    ],
                }
            }
            AdType::Closure => primal,
        }
    }

    /// Add two cotangents of the same `AdType`.
    fn add_cotangents(&mut self, a: Core, b: Core, ty: &AdType, next_local: &mut usize) -> Core {
        match ty {
            AdType::NonDiff | AdType::Closure => a,
            AdType::F32 => Self::call_global(FloatPrec::F32.add_id(), vec![a, b]),
            AdType::F64 => Self::call_global(FloatPrec::F64.add_id(), vec![a, b]),
            AdType::ArrayF32 => {
                let la = LocalId(*next_local);
                *next_local += 1;
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF32);
                let sum_opt = Core::App(Box::new(zip), vec![Core::Var(la), b, Core::Int(0)]);
                let unwrapped = self.unwrap_option_obj(sum_opt, Core::Var(la), next_local);
                Core::Let(la, Class::Obj, Box::new(a), Box::new(unwrapped))
            }
            AdType::ArrayF64 => {
                let la = LocalId(*next_local);
                *next_local += 1;
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF64);
                let sum_opt = Core::App(Box::new(zip), vec![Core::Var(la), b, Core::Int(0)]);
                let unwrapped = self.unwrap_option_obj(sum_opt, Core::Var(la), next_local);
                Core::Let(la, Class::Obj, Box::new(a), Box::new(unwrapped))
            }
            AdType::Tuple(fs) => {
                let mut a_fields = Vec::with_capacity(fs.len());
                let mut b_fields = Vec::with_capacity(fs.len());
                let mut classes = Vec::with_capacity(fs.len());
                for f in fs {
                    let la = LocalId(*next_local);
                    let lb = LocalId(*next_local + 1);
                    *next_local += 2;
                    a_fields.push(la);
                    b_fields.push(lb);
                    classes.push(f.class(Class::Raw));
                }
                let elems: Vec<(Core, Class)> = a_fields
                    .iter()
                    .zip(b_fields.iter())
                    .zip(fs.iter())
                    .zip(classes.iter())
                    .map(|(((&la, &lb), ft), &c)| {
                        (self.add_cotangents(Core::Var(la), Core::Var(lb), ft, next_local), c)
                    })
                    .collect();
                let tup = self.mk_tuple(elems);
                let inner_case = Core::Case {
                    scrut: Box::new(b),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag: 0, fields: b_fields, classes: classes.clone() },
                        weight: None,
                        body: Box::new(tup),
                    }],
                };
                Core::Case {
                    scrut: Box::new(a),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag: 0, fields: a_fields, classes },
                        weight: None,
                        body: Box::new(inner_case),
                    }],
                }
            }
            AdType::Product { ctor, fields, field_classes } => {
                let tag = self.ctor_info.get(ctor).map(|(t, _)| *t).unwrap_or(0);
                let mut a_fields = Vec::with_capacity(fields.len());
                let mut b_fields = Vec::with_capacity(fields.len());
                for _ in fields {
                    let la = LocalId(*next_local);
                    let lb = LocalId(*next_local + 1);
                    *next_local += 2;
                    a_fields.push(la);
                    b_fields.push(lb);
                }
                let args: Vec<Core> = a_fields
                    .iter()
                    .zip(b_fields.iter())
                    .zip(fields.iter())
                    .map(|((&la, &lb), ft)| {
                        self.add_cotangents(Core::Var(la), Core::Var(lb), ft, next_local)
                    })
                    .collect();
                let rebuilt = Core::App(Box::new(Core::Ctor(ctor.clone(), Vec::new())), args);
                let inner_case = Core::Case {
                    scrut: Box::new(b),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag, fields: b_fields, classes: field_classes.clone() },
                        weight: None,
                        body: Box::new(rebuilt),
                    }],
                };
                Core::Case {
                    scrut: Box::new(a),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag, fields: a_fields, classes: field_classes.clone() },
                        weight: None,
                        body: Box::new(inner_case),
                    }],
                }
            }
            AdType::Option(inner) => {
                let icls = inner.class(Class::Obj);
                let (none_id, some_id) = self.ensure_option_ctors(icls);
                let va = LocalId(*next_local);
                let vb = LocalId(*next_local + 1);
                let lb = LocalId(*next_local + 2);
                *next_local += 3;
                let summed = self.add_cotangents(Core::Var(va), Core::Var(vb), inner, next_local);
                let some_sum = Core::App(Box::new(Core::Ctor(some_id.clone(), Vec::new())), vec![summed]);
                let some_va = Core::App(Box::new(Core::Ctor(some_id, Vec::new())), vec![Core::Var(va)]);
                let on_some_a = Core::Case {
                    scrut: Box::new(Core::Var(lb)),
                    alts: vec![
                        CAlt {
                            pat: CPat::Con { tag: 0, fields: Vec::new(), classes: Vec::new() },
                            weight: None,
                            body: Box::new(some_va),
                        },
                        CAlt {
                            pat: CPat::Con { tag: 1, fields: vec![vb], classes: vec![icls] },
                            weight: None,
                            body: Box::new(some_sum),
                        },
                    ],
                };
                let _ = none_id;
                Core::Let(
                    lb,
                    Class::Obj,
                    Box::new(b),
                    Box::new(Core::Case {
                        scrut: Box::new(a),
                        alts: vec![
                            CAlt {
                                pat: CPat::Con { tag: 0, fields: Vec::new(), classes: Vec::new() },
                                weight: None,
                                body: Box::new(Core::Var(lb)),
                            },
                            CAlt {
                                pat: CPat::Con { tag: 1, fields: vec![va], classes: vec![icls] },
                                weight: None,
                                body: Box::new(on_some_a),
                            },
                        ],
                    }),
                )
            }
        }
    }

    // -----------------------------------------------------------------------
    // Scanning & Lowering AD Call Sites (Innermost-First)
    // -----------------------------------------------------------------------

    fn recognize_ad_head(&self, head: &Core) -> Option<(AdOp, [Class; 2])> {
        match head {
            Core::Prim(PrimOp::Ad { op, classes }) => Some((*op, *classes)),
            Core::Global(g, _) => self.ad_stubs.get(g).copied(),
            _ => None,
        }
    }

    fn resolve_target_fn(&self, func: &Core) -> Option<(GlobalId, Vec<Core>)> {
        match func {
            Core::Global(g, _) => Some((g.clone(), Vec::new())),
            Core::Lam { fn_id, env, .. } => Some((fn_id.clone(), env.clone())),
            Core::App(h, partial_args) => {
                let (gid, mut env) = self.resolve_target_fn(h)?;
                env.extend(partial_args.iter().cloned());
                Some((gid, env))
            }
            _ => None,
        }
    }

    fn lower_ad_calls_in_expr(
        &mut self,
        expr: &Core,
        local_classes: &HashMap<LocalId, Class>,
        next_local: &mut usize,
        changed: &mut bool,
    ) -> Result<Core> {
        let mut let_bindings: HashMap<LocalId, Core> = HashMap::new();
        self.lower_ad_calls_rec(expr, local_classes, &mut let_bindings, next_local, changed)
    }

    fn lower_ad_calls_rec(
        &mut self,
        expr: &Core,
        local_classes: &HashMap<LocalId, Class>,
        let_bindings: &mut HashMap<LocalId, Core>,
        next_local: &mut usize,
        changed: &mut bool,
    ) -> Result<Core> {
        match expr {
            Core::App(head, args) => {
                if let Some((op, classes)) = self.recognize_ad_head(head) {
                    if args.len() == 3 {
                        let mut f_expr = args[0].clone();
                        for _ in 0..16 {
                            if let Core::Var(l) = &f_expr {
                                if let Some(bound) = let_bindings.get(l) {
                                    f_expr = bound.clone();
                                    continue;
                                }
                            }
                            break;
                        }
                        if let Some((target_id, captured_env)) = self.resolve_target_fn(&f_expr) {
                            // Innermost-first: if the target function (or any transitive callee
                            // in its body) still contains an unlowered AD site, wait until
                            // the inner AD site has been lowered on this pass!
                            if self.fn_reaches_ad(&target_id) {
                                return Ok(expr.clone());
                            }
                            let x_arg = self.lower_ad_calls_rec(
                                &args[1],
                                local_classes,
                                let_bindings,
                                next_local,
                                changed,
                            )?;
                            let seed_arg = self.lower_ad_calls_rec(
                                &args[2],
                                local_classes,
                                let_bindings,
                                next_local,
                                changed,
                            )?;
                            let lowered = self.lower_single_ad_site(
                                op,
                                classes,
                                &target_id,
                                &captured_env,
                                x_arg,
                                seed_arg,
                                let_bindings,
                                next_local,
                            )?;
                            *changed = true;
                            return Ok(lowered);
                        }
                    }
                }
                let h2 = self.lower_ad_calls_rec(head, local_classes, let_bindings, next_local, changed)?;
                let mut a2 = Vec::with_capacity(args.len());
                for a in args {
                    a2.push(self.lower_ad_calls_rec(a, local_classes, let_bindings, next_local, changed)?);
                }
                Ok(Core::App(Box::new(h2), a2))
            }
            Core::Let(x, c, rhs, body) => {
                let r2 = self.lower_ad_calls_rec(rhs, local_classes, let_bindings, next_local, changed)?;
                let prev = let_bindings.insert(*x, r2.clone());
                let b2 = self.lower_ad_calls_rec(body, local_classes, let_bindings, next_local, changed)?;
                if let Some(p) = prev {
                    let_bindings.insert(*x, p);
                } else {
                    let_bindings.remove(x);
                }
                Ok(Core::Let(*x, *c, Box::new(r2), Box::new(b2)))
            }
            Core::If(c, t, f) => Ok(Core::If(
                Box::new(self.lower_ad_calls_rec(c, local_classes, let_bindings, next_local, changed)?),
                CBranch {
                    weight: t.weight,
                    body: Box::new(self.lower_ad_calls_rec(
                        &t.body,
                        local_classes,
                        let_bindings,
                        next_local,
                        changed,
                    )?),
                },
                CBranch {
                    weight: f.weight,
                    body: Box::new(self.lower_ad_calls_rec(
                        &f.body,
                        local_classes,
                        let_bindings,
                        next_local,
                        changed,
                    )?),
                },
            )),
            Core::Case { scrut, alts } => {
                let s2 = self.lower_ad_calls_rec(scrut, local_classes, let_bindings, next_local, changed)?;
                let mut new_alts = Vec::with_capacity(alts.len());
                for a in alts {
                    new_alts.push(CAlt {
                        pat: a.pat.clone(),
                        weight: a.weight,
                        body: Box::new(self.lower_ad_calls_rec(
                            &a.body,
                            local_classes,
                            let_bindings,
                            next_local,
                            changed,
                        )?),
                    });
                }
                Ok(Core::Case { scrut: Box::new(s2), alts: new_alts })
            }
            Core::Join { id, params, body, rest } => Ok(Core::Join {
                id: *id,
                params: params.clone(),
                body: Box::new(self.lower_ad_calls_rec(
                    body,
                    local_classes,
                    let_bindings,
                    next_local,
                    changed,
                )?),
                rest: Box::new(self.lower_ad_calls_rec(
                    rest,
                    local_classes,
                    let_bindings,
                    next_local,
                    changed,
                )?),
            }),
            Core::Jump { id, args } => {
                let mut a2 = Vec::with_capacity(args.len());
                for a in args {
                    a2.push(self.lower_ad_calls_rec(a, local_classes, let_bindings, next_local, changed)?);
                }
                Ok(Core::Jump { id: *id, args: a2 })
            }
            Core::Lam { fn_id, env, classes } => {
                let mut e2 = Vec::with_capacity(env.len());
                for a in env {
                    e2.push(self.lower_ad_calls_rec(a, local_classes, let_bindings, next_local, changed)?);
                }
                Ok(Core::Lam { fn_id: fn_id.clone(), env: e2, classes: classes.clone() })
            }
            other => Ok(other.clone()),
        }
    }

    fn fn_reaches_ad(&self, start: &GlobalId) -> bool {
        let stubs = self.ad_stub_set();
        let mut visited = HashSet::new();
        let mut stack = vec![start.clone()];
        while let Some(id) = stack.pop() {
            if !visited.insert(id.clone()) {
                continue;
            }
            let Some(f) = self.fns.get(&id) else { continue };
            if expr_contains_ad(&f.body, &stubs) {
                return true;
            }
            collect_callees(&f.body, &mut stack);
        }
        false
    }

    fn lower_single_ad_site(
        &mut self,
        op: AdOp,
        classes: [Class; 2],
        target_id: &GlobalId,
        captured_env: &[Core],
        x_arg: Core,
        seed_arg: Core,
        caller_bindings: &HashMap<LocalId, Core>,
        next_local: &mut usize,
    ) -> Result<Core> {
        let Some(target_fn) = self.fns.get(target_id).cloned() else {
            return Err(Error::plain(format!(
                "AD target function `{}` not found in Core program",
                target_id.0
            )));
        };
        let n_env = captured_env.len();
        if target_fn.params.len() != n_env + 1 {
            return Err(Error::plain(format!(
                "AD target `{}` expects {} argument(s) (with {} captured), expected {}",
                target_id.0,
                target_fn.params.len(),
                n_env,
                n_env + 1
            )));
        }

        // Infer `AdType` of every parameter of `target_fn` (captured env params are
        // treated as non-diff constants with respect to the differentiated argument `x`,
        // while `x` is the active differentiated input!).
        let mut seed_arg_tys = vec![AdType::NonDiff; n_env + 1];
        let caller_hint_in = self.infer_expr_ad_hint(&x_arg, caller_bindings, classes[0]);
        let caller_hint_out = match op {
            AdOp::Vjp2 => self.infer_expr_ad_hint(&seed_arg, caller_bindings, classes[1]),
            AdOp::Jvp2 => AdType::NonDiff,
        };
        seed_arg_tys[n_env] = caller_hint_in.clone();

        let (inferred_params, inferred_ret) =
            self.infer_fn_signature(target_id, &seed_arg_tys, &caller_hint_out);
        let in_ty = inferred_params[n_env].merge(&caller_hint_in);
        let out_ty = inferred_ret.merge(&caller_hint_out);

        let mut final_arg_tys = vec![AdType::NonDiff; n_env];
        final_arg_tys.push(in_ty.clone());

        match op {
            AdOp::Jvp2 => {
                if !in_ty.is_active() || !out_ty.is_active() {
                    // Constant / non-diff function: primal = f(x), tangent = zero_like(primal).
                    let mut call_args = captured_env.to_vec();
                    call_args.push(x_arg);
                    let primal = Self::call_global(target_id.clone(), call_args);
                    let lp = LocalId(*next_local);
                    *next_local += 1;
                    let zero = self.zero_like(Core::Var(lp), &out_ty, next_local);
                    let tup = self.mk_tuple(vec![(Core::Var(lp), classes[1]), (zero, classes[1])]);
                    return Ok(Core::Let(lp, classes[1], Box::new(primal), Box::new(tup)));
                }
                let (jvp_id, _jvp_ret) = self.gen_jvp_fn(target_id, &final_arg_tys, &out_ty)?;
                let mut call_args = captured_env.to_vec();
                call_args.push(x_arg);
                call_args.push(seed_arg);
                Ok(Self::call_global(jvp_id, call_args))
            }
            AdOp::Vjp2 => {
                if !in_ty.is_active() || !out_ty.is_active() {
                    let lx = LocalId(*next_local);
                    *next_local += 1;
                    let mut call_args = captured_env.to_vec();
                    call_args.push(Core::Var(lx));
                    let primal = Self::call_global(target_id.clone(), call_args);
                    let zero_in = self.zero_like(Core::Var(lx), &in_ty, next_local);
                    let tup = self.mk_tuple(vec![(primal, classes[1]), (zero_in, classes[0])]);
                    return Ok(Core::Let(lx, classes[0], Box::new(x_arg), Box::new(tup)));
                }
                let (vjp_direct_id, _vjp_ret) =
                    self.gen_vjp_direct_fn(target_id, &final_arg_tys, &out_ty)?;
                let mut call_args = captured_env.to_vec();
                call_args.push(x_arg);
                call_args.push(seed_arg);
                Ok(Self::call_global(vjp_direct_id, call_args))
            }
        }
    }

    // -----------------------------------------------------------------------
    // AdType Inference over Specialized Core IR
    // -----------------------------------------------------------------------

    fn infer_expr_ad_hint(
        &self,
        e: &Core,
        bindings: &HashMap<LocalId, Core>,
        cls: Class,
    ) -> AdType {
        match e {
            Core::Var(l) => bindings
                .get(l)
                .map(|b| self.infer_expr_ad_hint(b, bindings, cls))
                .unwrap_or(AdType::NonDiff),
            Core::App(h, args) => match &**h {
                Core::Global(g, _) => {
                    if let Some(op) = classify_scalar_float_op(&g.0) {
                        return match op {
                            ScalarFloatOp::Add(p)
                            | ScalarFloatOp::Sub(p)
                            | ScalarFloatOp::Mul(p)
                            | ScalarFloatOp::Div(p)
                            | ScalarFloatOp::Neg(p)
                            | ScalarFloatOp::Abs(p)
                            | ScalarFloatOp::Sqrt(p)
                            | ScalarFloatOp::Sin(p)
                            | ScalarFloatOp::Cos(p)
                            | ScalarFloatOp::Exp(p)
                            | ScalarFloatOp::Log(p) => p.ad_ty(),
                            ScalarFloatOp::Cmp(_) => AdType::NonDiff,
                        };
                    }
                    if let Some(f) = self.fns.get(g) {
                        if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                            return array_compute_ret_ad_type(op);
                        }
                        if let Some(PrimOp::Array { op: ArrayOp::FromList, packed: Some(s), .. }) =
                            f.prim
                        {
                            return match s {
                                crate::types::Scalar::F32 => AdType::ArrayF32,
                                crate::types::Scalar::F64 => AdType::ArrayF64,
                                _ => AdType::NonDiff,
                            };
                        }
                    }
                    // Named helper returning a float or array (e.g., `nd-tabulate-f32`, `array-from-f32s`)
                    let base = g.0.split('@').next().unwrap_or(&g.0);
                    if base.ends_with("array-from-f32s") || base.ends_with("array-tabulate-f32") {
                        return AdType::ArrayF32;
                    }
                    if base.ends_with("array-from-f64s") || base.ends_with("array-tabulate-f64") {
                        return AdType::ArrayF64;
                    }
                    AdType::NonDiff
                }
                Core::Prim(PrimOp::ArrayCompute { op, .. }) => array_compute_ret_ad_type(*op),
                Core::Prim(PrimOp::Array { op: ArrayOp::FromList, packed: Some(s), .. }) => {
                    match s {
                        crate::types::Scalar::F32 => AdType::ArrayF32,
                        crate::types::Scalar::F64 => AdType::ArrayF64,
                        _ => AdType::NonDiff,
                    }
                }
                Core::Ctor(c, _) => {
                    let base = c.0.split('@').next().unwrap_or(&c.0);
                    let fclasses = self
                        .ctor_info
                        .get(c)
                        .map(|(_, fc)| fc.clone())
                        .unwrap_or_else(|| vec![Class::Raw; args.len()]);
                    let fields: Vec<AdType> = args
                        .iter()
                        .zip(fclasses.iter())
                        .map(|(a, &fc)| self.infer_expr_ad_hint(a, bindings, fc))
                        .collect();
                    if base.starts_with("Tuple.") {
                        AdType::Tuple(fields)
                    } else if base == "Data.Core.Some" && fields.len() == 1 {
                        AdType::Option(Box::new(fields[0].clone()))
                    } else if fields.iter().any(AdType::is_active) {
                        AdType::Product { ctor: c.clone(), fields, field_classes: fclasses }
                    } else {
                        AdType::NonDiff
                    }
                }
                _ => AdType::NonDiff,
            },
            _ => AdType::NonDiff,
        }
    }

    /// Infer `AdType` for every local and the return value of `fn_id` given initial
    /// parameter hints `seed_params` and return hint `seed_ret`.
    fn infer_fn_locals(
        &self,
        fn_id: &GlobalId,
        seed_params: &[AdType],
        seed_ret: &AdType,
    ) -> (HashMap<LocalId, AdType>, AdType) {
        let mut locals: HashMap<LocalId, AdType> = HashMap::new();
        for (i, t) in seed_params.iter().enumerate() {
            locals.insert(LocalId(i), t.clone());
        }
        let Some(f) = self.fns.get(fn_id) else {
            return (locals, seed_ret.clone());
        };
        let mut ret_ty = seed_ret.clone();
        // Run fixed-point bidirectional inference across the body.
        let mut visiting = HashSet::new();
        visiting.insert(fn_id.clone());
        for _ in 0..6 {
            let inferred = self.infer_expr_types(&f.body, &mut locals, &ret_ty, &mut visiting, 0);
            ret_ty = ret_ty.merge(&inferred);
        }
        (locals, ret_ty)
    }

    fn infer_fn_signature(
        &self,
        fn_id: &GlobalId,
        seed_params: &[AdType],
        seed_ret: &AdType,
    ) -> (Vec<AdType>, AdType) {
        let (locals, ret_ty) = self.infer_fn_locals(fn_id, seed_params, seed_ret);
        let params: Vec<AdType> = (0..seed_params.len())
            .map(|i| locals.get(&LocalId(i)).cloned().unwrap_or(AdType::NonDiff))
            .collect();
        (params, ret_ty)
    }

    fn infer_expr_types(
        &self,
        expr: &Core,
        locals: &mut HashMap<LocalId, AdType>,
        expected: &AdType,
        visiting: &mut HashSet<GlobalId>,
        depth: usize,
    ) -> AdType {
        if depth > 10 {
            return expected.clone();
        }
        match expr {
            Core::Var(l) => {
                let prev = locals.get(l).cloned().unwrap_or(AdType::NonDiff);
                let merged = prev.merge(expected);
                locals.insert(*l, merged.clone());
                merged
            }
            Core::Int(_) => expected.clone(),
            Core::Let(x, _, rhs, body) => {
                let x_exp = locals.get(x).cloned().unwrap_or(AdType::NonDiff);
                let r_ty = self.infer_expr_types(rhs, locals, &x_exp, visiting, depth);
                let merged_x = locals
                    .get(x)
                    .cloned()
                    .unwrap_or(AdType::NonDiff)
                    .merge(&r_ty);
                locals.insert(*x, merged_x.clone());
                let b_ty = self.infer_expr_types(body, locals, expected, visiting, depth);
                // Backward pass: if body refined `x`'s type, propagate back into `rhs`!
                let x_after = locals.get(x).cloned().unwrap_or(AdType::NonDiff);
                if x_after != merged_x {
                    let _ = self.infer_expr_types(rhs, locals, &x_after, visiting, depth);
                }
                b_ty
            }
            Core::If(c, t, f) => {
                let _ = self.infer_expr_types(c, locals, &AdType::NonDiff, visiting, depth);
                let tt = self.infer_expr_types(&t.body, locals, expected, visiting, depth);
                let ft = self.infer_expr_types(&f.body, locals, expected, visiting, depth);
                tt.merge(&ft)
            }
            Core::Case { scrut, alts } => {
                let scrut_ty = self.infer_expr_types(scrut, locals, &AdType::NonDiff, visiting, depth);
                let mut res = expected.clone();
                let mut refined_scrut = scrut_ty.clone();
                let is_option_case = alts.len() == 2
                    && alts.iter().any(|a| matches!(&a.pat, CPat::Con { tag: 0, fields, .. } if fields.is_empty()))
                    && alts.iter().any(|a| matches!(&a.pat, CPat::Con { tag: 1, fields, .. } if fields.len() == 1));
                for alt in alts {
                    match &alt.pat {
                        CPat::Con { tag, fields, classes } => {
                            // Propagate known scrutinee field types down into pattern binders.
                            match &refined_scrut {
                                AdType::Tuple(fs) if *tag == 0 && fs.len() == fields.len() => {
                                    for (&fl, ft) in fields.iter().zip(fs.iter()) {
                                        let m = locals.get(&fl).cloned().unwrap_or(AdType::NonDiff).merge(ft);
                                        locals.insert(fl, m);
                                    }
                                }
                                AdType::Product { fields: fs, .. } if fs.len() == fields.len() => {
                                    for (&fl, ft) in fields.iter().zip(fs.iter()) {
                                        let m = locals.get(&fl).cloned().unwrap_or(AdType::NonDiff).merge(ft);
                                        locals.insert(fl, m);
                                    }
                                }
                                AdType::Option(inner) if is_option_case && *tag == 1 && fields.len() == 1 => {
                                    let m = locals.get(&fields[0]).cloned().unwrap_or(AdType::NonDiff).merge(inner);
                                    locals.insert(fields[0], m);
                                }
                                _ => {}
                            }
                            let bt = self.infer_expr_types(&alt.body, locals, expected, visiting, depth);
                            res = res.merge(&bt);
                            // Backward: collect refined binder types back onto scrutinee!
                            let field_tys: Vec<AdType> = fields
                                .iter()
                                .map(|fl| locals.get(fl).cloned().unwrap_or(AdType::NonDiff))
                                .collect();
                            if field_tys.iter().any(AdType::is_active) {
                                if is_option_case && *tag == 1 && field_tys.len() == 1 {
                                    refined_scrut = refined_scrut
                                        .merge(&AdType::Option(Box::new(field_tys[0].clone())));
                                } else if alts.len() == 1 && *tag == 0 {
                                    // Determine whether scrutinee is a Tuple or a Product ADT.
                                    if matches!(refined_scrut, AdType::Product { .. }) {
                                        if let AdType::Product { ctor, field_classes, .. } = &refined_scrut {
                                            refined_scrut = AdType::Product {
                                                ctor: ctor.clone(),
                                                fields: field_tys,
                                                field_classes: field_classes.clone(),
                                            };
                                        }
                                    } else if let Some(ctor_id) =
                                        self.find_product_ctor_for_case(scrut, fields.len(), classes)
                                    {
                                        refined_scrut = AdType::Product {
                                            ctor: ctor_id,
                                            fields: field_tys,
                                            field_classes: classes.clone(),
                                        };
                                    } else {
                                        refined_scrut = refined_scrut.merge(&AdType::Tuple(field_tys));
                                    }
                                }
                            }
                        }
                        CPat::Var(l, _) => {
                            let m = locals.get(l).cloned().unwrap_or(AdType::NonDiff).merge(&refined_scrut);
                            locals.insert(*l, m);
                            let bt = self.infer_expr_types(&alt.body, locals, expected, visiting, depth);
                            res = res.merge(&bt);
                        }
                        CPat::Wild => {
                            let bt = self.infer_expr_types(&alt.body, locals, expected, visiting, depth);
                            res = res.merge(&bt);
                        }
                    }
                }
                if refined_scrut != scrut_ty {
                    let _ = self.infer_expr_types(scrut, locals, &refined_scrut, visiting, depth);
                }
                res
            }
            Core::Join { params, body, rest, .. } => {
                let b_ty = self.infer_expr_types(body, locals, expected, visiting, depth);
                let r_ty = self.infer_expr_types(rest, locals, expected, visiting, depth);
                let _ = params;
                b_ty.merge(&r_ty)
            }
            Core::Jump { args, .. } => {
                for a in args {
                    let _ = self.infer_expr_types(a, locals, &AdType::NonDiff, visiting, depth);
                }
                expected.clone()
            }
            Core::App(head, args) => match &**head {
                Core::Global(g, _) => {
                    if let Some(op) = classify_scalar_float_op(&g.0) {
                        return match op {
                            ScalarFloatOp::Add(p)
                            | ScalarFloatOp::Sub(p)
                            | ScalarFloatOp::Mul(p)
                            | ScalarFloatOp::Div(p) => {
                                for a in args {
                                    let _ = self.infer_expr_types(a, locals, &p.ad_ty(), visiting, depth);
                                }
                                p.ad_ty()
                            }
                            ScalarFloatOp::Neg(p)
                            | ScalarFloatOp::Abs(p)
                            | ScalarFloatOp::Sqrt(p)
                            | ScalarFloatOp::Sin(p)
                            | ScalarFloatOp::Cos(p)
                            | ScalarFloatOp::Exp(p)
                            | ScalarFloatOp::Log(p) => {
                                if let Some(a) = args.first() {
                                    let _ = self.infer_expr_types(a, locals, &p.ad_ty(), visiting, depth);
                                }
                                p.ad_ty()
                            }
                            ScalarFloatOp::Cmp(p) => {
                                for a in args {
                                    let _ = self.infer_expr_types(a, locals, &p.ad_ty(), visiting, depth);
                                }
                                AdType::NonDiff
                            }
                        };
                    }
                    if let Some(f) = self.fns.get(g) {
                        if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                            return self.infer_array_compute_types(op, args, locals, visiting, depth);
                        }
                        if let Some(PrimOp::Array { op: ArrayOp::Get, .. }) = f.prim {
                            if args.len() == 2 {
                                let arr_exp = match expected {
                                    AdType::Option(inner) => match &**inner {
                                        AdType::F32 => AdType::ArrayF32,
                                        AdType::F64 => AdType::ArrayF64,
                                        _ => AdType::NonDiff,
                                    },
                                    _ => AdType::NonDiff,
                                };
                                let arr_ty = self.infer_expr_types(&args[0], locals, &arr_exp, visiting, depth);
                                return match arr_ty {
                                    AdType::ArrayF32 => AdType::Option(Box::new(AdType::F32)),
                                    AdType::ArrayF64 => AdType::Option(Box::new(AdType::F64)),
                                    _ => expected.clone(),
                                };
                            }
                        }
                        if f.prim.is_none() && f.ctor.is_none() && args.len() == f.params.len() {
                            let arg_tys: Vec<AdType> = args
                                .iter()
                                .map(|a| self.infer_expr_types(a, locals, &AdType::NonDiff, visiting, depth))
                                .collect();
                            let (refined_params, ret_ty) = if depth < 8 && visiting.insert(g.clone()) {
                                let mut sub_locals = HashMap::new();
                                for (i, t) in arg_tys.iter().enumerate() {
                                    sub_locals.insert(LocalId(i), t.clone());
                                }
                                let mut r = expected.clone();
                                let iters = if depth < 3 { 2 } else { 1 };
                                for _ in 0..iters {
                                    let r2 = self.infer_expr_types(&f.body, &mut sub_locals, &r, visiting, depth + 1);
                                    r = r.merge(&r2);
                                }
                                visiting.remove(g);
                                let rp: Vec<AdType> = (0..arg_tys.len())
                                    .map(|i| sub_locals.get(&LocalId(i)).cloned().unwrap_or(AdType::NonDiff))
                                    .collect();
                                (rp, r)
                            } else {
                                (arg_tys.clone(), expected.clone())
                            };
                            for (a, rt) in args.iter().zip(refined_params.iter()) {
                                if rt.is_active() {
                                    let _ = self.infer_expr_types(a, locals, rt, visiting, depth);
                                }
                            }
                            return ret_ty;
                        }
                    }
                    expected.clone()
                }
                Core::Prim(PrimOp::ArrayCompute { op, .. }) => {
                    self.infer_array_compute_types(*op, args, locals, visiting, depth)
                }
                Core::Ctor(c, _) => {
                    let base = c.0.split('@').next().unwrap_or(&c.0);
                    let fclasses = self
                        .ctor_info
                        .get(c)
                        .map(|(_, fc)| fc.clone())
                        .unwrap_or_else(|| vec![Class::Raw; args.len()]);
                    let exp_fields: Vec<AdType> = match expected {
                        AdType::Tuple(fs) if fs.len() == args.len() => fs.clone(),
                        AdType::Product { fields, .. } if fields.len() == args.len() => fields.clone(),
                        AdType::Option(inner) if base == "Data.Core.Some" && args.len() == 1 => {
                            vec![(**inner).clone()]
                        }
                        _ => vec![AdType::NonDiff; args.len()],
                    };
                    let fields: Vec<AdType> = args
                        .iter()
                        .zip(exp_fields.iter())
                        .map(|(a, e)| self.infer_expr_types(a, locals, e, visiting, depth))
                        .collect();
                    if base.starts_with("Tuple.") {
                        AdType::Tuple(fields)
                    } else if base == "Data.Core.Some" && fields.len() == 1 {
                        AdType::Option(Box::new(fields[0].clone()))
                    } else if base == "Data.Core.None" {
                        expected.clone()
                    } else if fields.iter().any(AdType::is_active) {
                        AdType::Product { ctor: c.clone(), fields, field_classes: fclasses }
                    } else {
                        AdType::NonDiff
                    }
                }
                _ => {
                    for a in args {
                        let _ = self.infer_expr_types(a, locals, &AdType::NonDiff, visiting, depth);
                    }
                    expected.clone()
                }
            },
            Core::Lam { env, .. } => {
                let mut any_active = false;
                for a in env {
                    if self.infer_expr_types(a, locals, &AdType::NonDiff, visiting, depth).is_active() {
                        any_active = true;
                    }
                }
                if any_active { AdType::Closure } else { AdType::NonDiff }
            }
            _ => AdType::NonDiff,
        }
    }

    fn infer_array_compute_types(
        &self,
        op: ArrayComputeOp,
        args: &[Core],
        locals: &mut HashMap<LocalId, AdType>,
        visiting: &mut HashSet<GlobalId>,
        depth: usize,
    ) -> AdType {
        let param_tys = array_compute_param_ad_types(op);
        for (a, pt) in args.iter().zip(param_tys.iter()) {
            let _ = self.infer_expr_types(a, locals, pt, visiting, depth);
        }
        array_compute_ret_ad_type(op)
    }

    /// When a `Case` matches a single-constructor ADT (like `MkNDView`, `MkView1`,
    /// `MkView2`, `MkView3`, or a user record), look up its `GlobalId` if it is not a `Tuple.N`.
    fn find_product_ctor_for_case(
        &self,
        _scrut: &Core,
        arity: usize,
        classes: &[Class],
    ) -> Option<GlobalId> {
        // Check well-known view constructors first (NDView, View1, View2, View3).
        for prefix in [
            "Data.Array.ND.MkNDView",
            "Data.Array.Compute.MkView1",
            "Data.Array.Compute.MkView2",
            "Data.Array.Compute.MkView3",
        ] {
            for (id, (tag, fc)) in &self.ctor_info {
                if *tag == 0
                    && id.0.starts_with(prefix)
                    && fc.len() == arity
                    && fc.as_slice() == classes
                {
                    return Some(id.clone());
                }
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // FORWARD-MODE AD: Jacobian-Vector Product (JVP)
    // -----------------------------------------------------------------------

    fn gen_jvp_fn(
        &mut self,
        fn_id: &GlobalId,
        arg_tys: &[AdType],
        ret_hint: &AdType,
    ) -> Result<(GlobalId, AdType)> {
        let key = (fn_id.clone(), arg_tys.to_vec());
        if let Some(hit) = self.jvp_cache.get(&key) {
            return Ok(hit.clone());
        }
        let orig = self.fns.get(fn_id).cloned().ok_or_else(|| {
            Error::plain(format!("JVP: unknown function `{}`", fn_id.0))
        })?;
        let (local_tys, ret_ty) = self.infer_fn_locals(fn_id, arg_tys, ret_hint);
        let jvp_id = self.fresh_unit_id(&fn_id.0, "jvp");

        // Pre-insert into cache BEFORE transforming the body so recursive calls
        // to `fn_id` resolve to `jvp_id`!
        self.jvp_cache.insert(key, (jvp_id.clone(), ret_ty.clone()));

        // Build parameter list: each inactive param `p_i` keeps 1 slot; each active
        // param `p_i` expands to `(p_i, dp_i)`.
        let mut new_params: Vec<String> = Vec::new();
        let mut new_param_classes: Vec<Class> = Vec::new();
        let mut primal_map: HashMap<LocalId, LocalId> = HashMap::new();
        let mut tangent_map: HashMap<LocalId, LocalId> = HashMap::new();
        let mut cur_local = 0usize;

        for (i, orig_name) in orig.params.iter().enumerate() {
            let cls = orig.repr.params.get(i).copied().unwrap_or(Class::Raw);
            let p_new = LocalId(cur_local);
            cur_local += 1;
            new_params.push(orig_name.clone());
            new_param_classes.push(cls);
            primal_map.insert(LocalId(i), p_new);

            let at = arg_tys.get(i).cloned().unwrap_or(AdType::NonDiff);
            if at.is_active() {
                let dp_new = LocalId(cur_local);
                cur_local += 1;
                new_params.push(format!("d_{orig_name}"));
                new_param_classes.push(at.class(cls));
                tangent_map.insert(LocalId(i), dp_new);
            }
        }

        // Remap all non-parameter locals above the new parameter range to avoid collisions.
        let mut next_local = cur_local + max_local_in_fn(&orig) + 2;
        let orig_ret_cls = orig.repr.result.unwrap_or(Class::Raw);

        let mut env = JvpEnv {
            primal_map,
            tangent_map,
            local_tys,
            local_classes: orig
                .repr
                .params
                .iter()
                .enumerate()
                .map(|(i, c)| (LocalId(i), *c))
                .collect(),
            join_sigs: HashMap::new(),
            closure_targets: HashMap::new(),
        };
        collect_let_classes(&orig.body, &mut env.local_classes);

        let mut body_binds = Vec::new();
        let (p_body, t_body) =
            self.jvp_expr(&orig.body, &mut env, &mut body_binds, &mut next_local, &ret_ty)?;
        let ret_expr = if ret_ty.is_active() {
            let rcls = ret_ty.class(orig_ret_cls);
            self.mk_tuple(vec![(p_body, rcls), (t_body, rcls)])
        } else {
            p_body
        };
        let final_body = wrap_jvp_binds(body_binds, ret_expr);

        let result_class = if ret_ty.is_active() {
            Class::Obj
        } else {
            orig_ret_cls
        };

        self.add_fn(FnDef {
            id: jvp_id.clone(),
            params: new_params,
            body: final_body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: UnitRepr {
                tparams: 0,
                params: new_param_classes,
                fields: Vec::new(),
                data: None,
                result: Some(result_class),
                dictionary_values: Vec::new(),
                ..Default::default()
            },
        });

        Ok((jvp_id, ret_ty))
    }

    /// Emit a single-evaluation unpack binding `case pair of (lp, lt) -> ...` into `binds`
    /// and return `(Var(lp), Var(lt))`.
    fn emit_pair_bind(
        pair: Core,
        elem_cls: Class,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
    ) -> (Core, Core) {
        let lp = LocalId(*next_local);
        let lt = LocalId(*next_local + 1);
        *next_local += 2;
        binds.push(JvpBind::UnpackPair { p: lp, t: lt, cls: elem_cls, pair });
        (Core::Var(lp), Core::Var(lt))
    }

    /// Ensure an expression is atomic (`Var` or constant) before duplicating its reference.
    fn emit_atom(
        expr: Core,
        cls: Class,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
    ) -> Core {
        match expr {
            Core::Var(_) | Core::Int(_) | Core::Bool(_) | Core::Unit => expr,
            other => {
                let l = LocalId(*next_local);
                *next_local += 1;
                binds.push(JvpBind::Let(l, cls, other));
                Core::Var(l)
            }
        }
    }

    /// Transform `expr` in forward mode, appending any single-evaluation bindings
    /// into `binds` and returning `(primal_core, tangent_core)`.
    fn jvp_expr(
        &mut self,
        expr: &Core,
        env: &mut JvpEnv,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
        expected: &AdType,
    ) -> Result<(Core, Core)> {
        match expr {
            Core::Var(l) => {
                let p = env.primal_map.get(l).copied().unwrap_or(*l);
                let ty = env.local_tys.get(l).cloned().unwrap_or_else(|| expected.clone());
                if let Some(&t) = env.tangent_map.get(l) {
                    Ok((Core::Var(p), Core::Var(t)))
                } else {
                    let z = self.zero_like(Core::Var(p), &ty, next_local);
                    Ok((Core::Var(p), z))
                }
            }
            Core::Int(_)
            | Core::Wide(_)
            | Core::ConstBig { .. }
            | Core::ConstBytes(_)
            | Core::ConstString { .. }
            | Core::ConstSym(_)
            | Core::Bool(_)
            | Core::Unit
            | Core::Fault => {
                let z = self.zero_like(expr.clone(), expected, next_local);
                Ok((expr.clone(), z))
            }
            Core::Let(x, cls, rhs, body) => {
                let x_ty = env.local_tys.get(x).cloned().unwrap_or(AdType::NonDiff);
                // Track lambda bindings so indirect calls to active closures propagate tangents!
                if let Core::Lam { fn_id, env: cap_env, .. } = &**rhs {
                    let cap_tys: Vec<AdType> = cap_env
                        .iter()
                        .map(|a| self.expr_ad_type_quick(a, env))
                        .collect();
                    if cap_tys.iter().any(AdType::is_active) {
                        env.closure_targets
                            .insert(*x, (fn_id.clone(), cap_env.clone(), cap_tys));
                    }
                }
                let (rp, rt) = self.jvp_expr(rhs, env, binds, next_local, &x_ty)?;
                let xp = LocalId(*next_local);
                *next_local += 1;
                binds.push(JvpBind::Let(xp, *cls, rp));
                let prev_p = env.primal_map.insert(*x, xp);
                let prev_t = if x_ty.is_active() {
                    let xt = LocalId(*next_local);
                    *next_local += 1;
                    binds.push(JvpBind::Let(xt, x_ty.class(*cls), rt));
                    Some(env.tangent_map.insert(*x, xt))
                } else {
                    None
                };
                let res = self.jvp_expr(body, env, binds, next_local, expected)?;
                restore_map(&mut env.primal_map, *x, prev_p);
                if let Some(pt) = prev_t {
                    restore_map(&mut env.tangent_map, *x, pt);
                }
                Ok(res)
            }
            Core::If(cond, t_br, f_br) => {
                let (cp, _) = self.jvp_expr(cond, env, binds, next_local, &AdType::NonDiff)?;
                let mut t_binds = Vec::new();
                let (tp, tt) = self.jvp_expr(&t_br.body, env, &mut t_binds, next_local, expected)?;
                let mut f_binds = Vec::new();
                let (fp, ft) = self.jvp_expr(&f_br.body, env, &mut f_binds, next_local, expected)?;
                if expected.is_active() {
                    let rcls = expected.class(Class::Raw);
                    let t_tup = wrap_jvp_binds(t_binds, self.mk_tuple(vec![(tp, rcls), (tt, rcls)]));
                    let f_tup = wrap_jvp_binds(f_binds, self.mk_tuple(vec![(fp, rcls), (ft, rcls)]));
                    let if_expr = Core::If(
                        Box::new(cp),
                        CBranch { weight: t_br.weight, body: Box::new(t_tup) },
                        CBranch { weight: f_br.weight, body: Box::new(f_tup) },
                    );
                    Ok(Self::emit_pair_bind(if_expr, rcls, binds, next_local))
                } else {
                    let if_expr = Core::If(
                        Box::new(cp),
                        CBranch { weight: t_br.weight, body: Box::new(wrap_jvp_binds(t_binds, tp)) },
                        CBranch { weight: f_br.weight, body: Box::new(wrap_jvp_binds(f_binds, fp)) },
                    );
                    let rcls = self.infer_expr_class(expr, expected, &env.local_classes);
                    let lp = Self::emit_atom(if_expr, rcls, binds, next_local);
                    let z = self.zero_like(lp.clone(), expected, next_local);
                    Ok((lp, z))
                }
            }
            Core::Case { scrut, alts } => {
                let scrut_ty = self.expr_ad_type_quick(scrut, env);
                let (sp, st) = self.jvp_expr(scrut, env, binds, next_local, &scrut_ty)?;
                let rcls = self.infer_expr_class(expr, expected, &env.local_classes);

                if scrut_ty.is_active() {
                    let lst = LocalId(*next_local);
                    *next_local += 1;
                    binds.push(JvpBind::Let(lst, scrut_ty.class(Class::Obj), st));
                    let mut new_alts = Vec::with_capacity(alts.len());
                    for alt in alts {
                        let mut arm_binds = Vec::new();
                        match &alt.pat {
                            CPat::Con { tag, fields, classes } => {
                                let mut p_fields = Vec::with_capacity(fields.len());
                                let mut t_fields = Vec::with_capacity(fields.len());
                                let mut saved_p = Vec::new();
                                let mut saved_t = Vec::new();
                                for &fl in fields {
                                    let pf = LocalId(*next_local);
                                    let tf = LocalId(*next_local + 1);
                                    *next_local += 2;
                                    p_fields.push(pf);
                                    t_fields.push(tf);
                                    saved_p.push((fl, env.primal_map.insert(fl, pf)));
                                    let fty = env.local_tys.get(&fl).cloned().unwrap_or(AdType::NonDiff);
                                    if fty.is_active() {
                                        saved_t.push((fl, env.tangent_map.insert(fl, tf)));
                                    }
                                }
                                let (bp, bt) =
                                    self.jvp_expr(&alt.body, env, &mut arm_binds, next_local, expected)?;
                                for (fl, old) in saved_p {
                                    restore_map(&mut env.primal_map, fl, old);
                                }
                                for (fl, old) in saved_t {
                                    restore_map(&mut env.tangent_map, fl, old);
                                }
                                let arm_res = wrap_jvp_binds(
                                    arm_binds,
                                    if expected.is_active() {
                                        self.mk_tuple(vec![(bp, rcls), (bt, rcls)])
                                    } else {
                                        bp
                                    },
                                );
                                // Destructure the tangent scrutinee `lst` with the same tag!
                                let wrapped_body = if t_fields.is_empty() {
                                    arm_res
                                } else {
                                    Core::Case {
                                        scrut: Box::new(Core::Var(lst)),
                                        alts: vec![
                                            CAlt {
                                                pat: CPat::Con {
                                                    tag: *tag,
                                                    fields: t_fields,
                                                    classes: classes.clone(),
                                                },
                                                weight: None,
                                                body: Box::new(arm_res),
                                            },
                                            CAlt {
                                                pat: CPat::Wild,
                                                weight: None,
                                                body: Box::new(Core::Fault),
                                            },
                                        ],
                                    }
                                };
                                new_alts.push(CAlt {
                                    pat: CPat::Con {
                                        tag: *tag,
                                        fields: p_fields,
                                        classes: classes.clone(),
                                    },
                                    weight: alt.weight,
                                    body: Box::new(wrapped_body),
                                });
                            }
                            CPat::Var(l, c) => {
                                let pl = LocalId(*next_local);
                                *next_local += 1;
                                let old_p = env.primal_map.insert(*l, pl);
                                let old_t = env.tangent_map.insert(*l, lst);
                                let (bp, bt) =
                                    self.jvp_expr(&alt.body, env, &mut arm_binds, next_local, expected)?;
                                restore_map(&mut env.primal_map, *l, old_p);
                                restore_map(&mut env.tangent_map, *l, old_t);
                                let arm_res = wrap_jvp_binds(
                                    arm_binds,
                                    if expected.is_active() {
                                        self.mk_tuple(vec![(bp, rcls), (bt, rcls)])
                                    } else {
                                        bp
                                    },
                                );
                                new_alts.push(CAlt {
                                    pat: CPat::Var(pl, *c),
                                    weight: alt.weight,
                                    body: Box::new(arm_res),
                                });
                            }
                            CPat::Wild => {
                                let (bp, bt) =
                                    self.jvp_expr(&alt.body, env, &mut arm_binds, next_local, expected)?;
                                let arm_res = wrap_jvp_binds(
                                    arm_binds,
                                    if expected.is_active() {
                                        self.mk_tuple(vec![(bp, rcls), (bt, rcls)])
                                    } else {
                                        bp
                                    },
                                );
                                new_alts.push(CAlt {
                                    pat: CPat::Wild,
                                    weight: alt.weight,
                                    body: Box::new(arm_res),
                                });
                            }
                        }
                    }
                    let case_expr = Core::Case { scrut: Box::new(sp), alts: new_alts };
                    if expected.is_active() {
                        Ok(Self::emit_pair_bind(case_expr, rcls, binds, next_local))
                    } else {
                        let lp = Self::emit_atom(case_expr, rcls, binds, next_local);
                        let z = self.zero_like(lp.clone(), expected, next_local);
                        Ok((lp, z))
                    }
                } else {
                    let mut new_alts = Vec::with_capacity(alts.len());
                    for alt in alts {
                        let mut arm_binds = Vec::new();
                        let (pat2, saved) = match &alt.pat {
                            CPat::Con { tag, fields, classes } => {
                                let mut nf = Vec::with_capacity(fields.len());
                                let mut sv = Vec::new();
                                for &fl in fields {
                                    let pf = LocalId(*next_local);
                                    *next_local += 1;
                                    nf.push(pf);
                                    sv.push((fl, env.primal_map.insert(fl, pf)));
                                }
                                (CPat::Con { tag: *tag, fields: nf, classes: classes.clone() }, sv)
                            }
                            CPat::Var(l, c) => {
                                let pf = LocalId(*next_local);
                                *next_local += 1;
                                let old = env.primal_map.insert(*l, pf);
                                (CPat::Var(pf, *c), vec![(*l, old)])
                            }
                            CPat::Wild => (CPat::Wild, Vec::new()),
                        };
                        let (bp, bt) =
                            self.jvp_expr(&alt.body, env, &mut arm_binds, next_local, expected)?;
                        for (fl, old) in saved {
                            restore_map(&mut env.primal_map, fl, old);
                        }
                        let arm_res = wrap_jvp_binds(
                            arm_binds,
                            if expected.is_active() {
                                self.mk_tuple(vec![(bp, rcls), (bt, rcls)])
                            } else {
                                bp
                            },
                        );
                        new_alts.push(CAlt { pat: pat2, weight: alt.weight, body: Box::new(arm_res) });
                    }
                    let case_expr = Core::Case { scrut: Box::new(sp), alts: new_alts };
                    if expected.is_active() {
                        Ok(Self::emit_pair_bind(case_expr, rcls, binds, next_local))
                    } else {
                        let lp = Self::emit_atom(case_expr, rcls, binds, next_local);
                        let z = self.zero_like(lp.clone(), expected, next_local);
                        Ok((lp, z))
                    }
                }
            }
            Core::Join { id, params, body, rest } => {
                // Expand active join parameters `(p_i)` -> `(p_i, dp_i)`.
                let mut new_params = Vec::new();
                let mut param_tys = Vec::new();
                let mut saved_p = Vec::new();
                let mut saved_t = Vec::new();
                for &(pl, cls) in params {
                    let pt = env.local_tys.get(&pl).cloned().unwrap_or(AdType::NonDiff);
                    param_tys.push(pt.clone());
                    let np = LocalId(*next_local);
                    *next_local += 1;
                    new_params.push((np, cls));
                    saved_p.push((pl, env.primal_map.insert(pl, np)));
                    if pt.is_active() {
                        let nt = LocalId(*next_local);
                        *next_local += 1;
                        new_params.push((nt, pt.class(cls)));
                        saved_t.push((pl, env.tangent_map.insert(pl, nt)));
                    }
                }
                env.join_sigs.insert(*id, param_tys);
                let mut b_binds = Vec::new();
                let (bp, bt) = self.jvp_expr(body, env, &mut b_binds, next_local, expected)?;
                for (pl, old) in saved_p {
                    restore_map(&mut env.primal_map, pl, old);
                }
                for (pl, old) in saved_t {
                    restore_map(&mut env.tangent_map, pl, old);
                }
                let mut r_binds = Vec::new();
                let (rp, rt) = self.jvp_expr(rest, env, &mut r_binds, next_local, expected)?;
                let rcls = self.infer_expr_class(expr, expected, &env.local_classes);
                if expected.is_active() {
                    let b_tup = wrap_jvp_binds(b_binds, self.mk_tuple(vec![(bp, rcls), (bt, rcls)]));
                    let r_tup = wrap_jvp_binds(r_binds, self.mk_tuple(vec![(rp, rcls), (rt, rcls)]));
                    let jexpr = Core::Join {
                        id: *id,
                        params: new_params,
                        body: Box::new(b_tup),
                        rest: Box::new(r_tup),
                    };
                    Ok(Self::emit_pair_bind(jexpr, rcls, binds, next_local))
                } else {
                    let jexpr = Core::Join {
                        id: *id,
                        params: new_params,
                        body: Box::new(wrap_jvp_binds(b_binds, bp)),
                        rest: Box::new(wrap_jvp_binds(r_binds, rp)),
                    };
                    let lp = Self::emit_atom(jexpr, rcls, binds, next_local);
                    let z = self.zero_like(lp.clone(), expected, next_local);
                    Ok((lp, z))
                }
            }
            Core::Jump { id, args } => {
                let sig = env.join_sigs.get(id).cloned().unwrap_or_default();
                let mut jargs = Vec::new();
                for (i, a) in args.iter().enumerate() {
                    let at = sig.get(i).cloned().unwrap_or(AdType::NonDiff);
                    let (ap, atang) = self.jvp_expr(a, env, binds, next_local, &at)?;
                    jargs.push(ap);
                    if at.is_active() {
                        jargs.push(atang);
                    }
                }
                let jump = Core::Jump { id: *id, args: jargs };
                // `Jump` never returns to the caller; tangent slot is unreachable.
                Ok((jump, Core::Unit))
            }
            Core::App(head, args) => self.jvp_app(head, args, env, binds, next_local, expected),
            Core::Ctor(c, cls) => {
                let p = Core::Ctor(c.clone(), cls.clone());
                Ok((p.clone(), p))
            }
            Core::Lam { fn_id, env: cap_env, classes } => {
                let cap_tys: Vec<AdType> = cap_env
                    .iter()
                    .map(|a| self.expr_ad_type_quick(a, env))
                    .collect();
                let mut p_env = Vec::with_capacity(cap_env.len());
                for (a, ct) in cap_env.iter().zip(cap_tys.iter()) {
                    let (ap, _) = self.jvp_expr(a, env, binds, next_local, ct)?;
                    p_env.push(ap);
                }
                let lam = Core::Lam { fn_id: fn_id.clone(), env: p_env, classes: classes.clone() };
                Ok((lam.clone(), lam))
            }
            other => {
                let remapped = remap_locals(other, &env.primal_map);
                let z = self.zero_like(remapped.clone(), expected, next_local);
                Ok((remapped, z))
            }
        }
    }

    fn jvp_app(
        &mut self,
        head: &Core,
        args: &[Core],
        env: &mut JvpEnv,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
        expected: &AdType,
    ) -> Result<(Core, Core)> {
        // 1. Scalar float primitives (`Data.Machine.*-f32/f64`, `Data.AD.{sin,cos,exp,log}-*`).
        if let Core::Global(g, _) = head {
            if let Some(op) = classify_scalar_float_op(&g.0) {
                return self.jvp_scalar_op(op, head, args, env, binds, next_local);
            }
            if let Some(f) = self.fns.get(g).cloned() {
                if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                    return self.jvp_array_compute(op, args, env, binds, next_local, expected);
                }
                if let Some(PrimOp::Array { op: ArrayOp::Get, .. }) = f.prim {
                    if args.len() == 2 {
                        let arr_ty = self.expr_ad_type_quick(&args[0], env);
                        if matches!(arr_ty, AdType::ArrayF32 | AdType::ArrayF64) {
                            let (ap, at) = self.jvp_expr(&args[0], env, binds, next_local, &arr_ty)?;
                            let (ip, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                            let ip = Self::emit_atom(ip, Class::TObj, binds, next_local);
                            let p_get = Core::App(Box::new(head.clone()), vec![ap, ip.clone()]);
                            let t_get = Core::App(Box::new(head.clone()), vec![at, ip]);
                            return Ok((p_get, t_get));
                        }
                    }
                }
                // User-defined function call: call its memoized `$jvp` companion!
                if f.prim.is_none() && f.ctor.is_none() && args.len() == f.params.len() {
                    let arg_tys: Vec<AdType> = args
                        .iter()
                        .map(|a| self.expr_ad_type_quick(a, env))
                        .collect();
                    if arg_tys.iter().any(AdType::is_active) {
                        let (jvp_id, jvp_ret) = self.gen_jvp_fn(g, &arg_tys, expected)?;
                        let mut call_args = Vec::new();
                        for (a, at) in args.iter().zip(arg_tys.iter()) {
                            let (ap, atang) = self.jvp_expr(a, env, binds, next_local, at)?;
                            call_args.push(ap);
                            if at.is_active() {
                                call_args.push(atang);
                            }
                        }
                        let call = Self::call_global(jvp_id, call_args);
                        if jvp_ret.is_active() {
                            let rcls = jvp_ret.class(f.repr.result.unwrap_or(Class::Raw));
                            return Ok(Self::emit_pair_bind(call, rcls, binds, next_local));
                        } else {
                            let lp = Self::emit_atom(
                                call,
                                f.repr.result.unwrap_or(Class::Raw),
                                binds,
                                next_local,
                            );
                            let z = self.zero_like(lp.clone(), expected, next_local);
                            return Ok((lp, z));
                        }
                    }
                }
            }
        }

        // 2. Direct `PrimOp::ArrayCompute` call.
        if let Core::Prim(PrimOp::ArrayCompute { op, .. }) = head {
            return self.jvp_array_compute(*op, args, env, binds, next_local, expected);
        }

        // 3. Constructor application (`Tuple.N`, `Some`, `MkNDView`, `MkView1`, user records).
        if let Core::Ctor(c, _) = head {
            let fclasses = self
                .ctor_info
                .get(c)
                .map(|(_, fc)| fc.clone())
                .unwrap_or_else(|| vec![Class::Raw; args.len()]);
            let mut p_args = Vec::with_capacity(args.len());
            let mut t_args = Vec::with_capacity(args.len());
            for (i, a) in args.iter().enumerate() {
                let at = self.expr_ad_type_quick(a, env);
                let (ap, atang) = self.jvp_expr(a, env, binds, next_local, &at)?;
                if at.is_active() {
                    p_args.push(ap);
                    t_args.push(atang);
                } else {
                    let fc = fclasses.get(i).copied().unwrap_or(Class::Raw);
                    let ap_atom = Self::emit_atom(ap, fc, binds, next_local);
                    p_args.push(ap_atom.clone());
                    t_args.push(ap_atom);
                }
            }
            return Ok((
                Core::App(Box::new(head.clone()), p_args),
                Core::App(Box::new(head.clone()), t_args),
            ));
        }

        // 4. Indirect call to a locally bound closure whose environment captured active vars
        //    (critical for forward-over-reverse HVP: calling a pullback closure inside `jvp`!).
        if let Core::Var(f_var) = head {
            if let Some((clo_fn_id, cap_env, cap_tys)) = env.closure_targets.get(f_var).cloned() {
                let mut full_arg_tys = cap_tys.clone();
                for a in args {
                    full_arg_tys.push(self.expr_ad_type_quick(a, env));
                }
                if full_arg_tys.iter().any(AdType::is_active) {
                    let (jvp_id, jvp_ret) = self.gen_jvp_fn(&clo_fn_id, &full_arg_tys, expected)?;
                    let mut call_args = Vec::new();
                    for (a, at) in cap_env.iter().zip(cap_tys.iter()) {
                        let (ap, atang) = self.jvp_expr(a, env, binds, next_local, at)?;
                        call_args.push(ap);
                        if at.is_active() {
                            call_args.push(atang);
                        }
                    }
                    for (a, at) in args.iter().zip(full_arg_tys[cap_tys.len()..].iter()) {
                        let (ap, atang) = self.jvp_expr(a, env, binds, next_local, at)?;
                        call_args.push(ap);
                        if at.is_active() {
                            call_args.push(atang);
                        }
                    }
                    let call = Self::call_global(jvp_id, call_args);
                    if jvp_ret.is_active() {
                        let rcls = jvp_ret.class(Class::Raw);
                        return Ok(Self::emit_pair_bind(call, rcls, binds, next_local));
                    } else {
                        let lp = Self::emit_atom(call, Class::Raw, binds, next_local);
                        let z = self.zero_like(lp.clone(), expected, next_local);
                        return Ok((lp, z));
                    }
                }
            }
        }

        // 5. Fallback: non-differentiable primitive or helper call.
        let orig_app = Core::App(Box::new(head.clone()), args.to_vec());
        let rcls = self.infer_expr_class(&orig_app, expected, &env.local_classes);
        let (hp, _) = self.jvp_expr(head, env, binds, next_local, &AdType::NonDiff)?;
        let mut p_args = Vec::with_capacity(args.len());
        for a in args {
            let at = self.expr_ad_type_quick(a, env);
            let (ap, _) = self.jvp_expr(a, env, binds, next_local, &at)?;
            p_args.push(ap);
        }
        let call = Core::App(Box::new(hp), p_args);
        let lp = Self::emit_atom(call, rcls, binds, next_local);
        let z = self.zero_like(lp.clone(), expected, next_local);
        Ok((lp, z))
    }

    fn jvp_scalar_op(
        &mut self,
        op: ScalarFloatOp,
        head: &Core,
        args: &[Core],
        env: &mut JvpEnv,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
    ) -> Result<(Core, Core)> {
        match op {
            ScalarFloatOp::Add(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let (y, dy) = self.jvp_expr(&args[1], env, binds, next_local, &p.ad_ty())?;
                Ok((
                    Self::call_global(p.add_id(), vec![x, y]),
                    Self::call_global(p.add_id(), vec![dx, dy]),
                ))
            }
            ScalarFloatOp::Sub(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let (y, dy) = self.jvp_expr(&args[1], env, binds, next_local, &p.ad_ty())?;
                Ok((
                    Self::call_global(p.sub_id(), vec![x, y]),
                    Self::call_global(p.sub_id(), vec![dx, dy]),
                ))
            }
            ScalarFloatOp::Mul(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let (y, dy) = self.jvp_expr(&args[1], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let y = Self::emit_atom(y, Class::Raw, binds, next_local);
                let primal = Self::call_global(p.mul_id(), vec![x.clone(), y.clone()]);
                let term1 = Self::call_global(p.mul_id(), vec![dx, y]);
                let term2 = Self::call_global(p.mul_id(), vec![x, dy]);
                let tangent = Self::call_global(p.add_id(), vec![term1, term2]);
                Ok((primal, tangent))
            }
            ScalarFloatOp::Div(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let (y, dy) = self.jvp_expr(&args[1], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let y = Self::emit_atom(y, Class::Raw, binds, next_local);
                let primal = Self::call_global(p.div_id(), vec![x.clone(), y.clone()]);
                let num1 = Self::call_global(p.mul_id(), vec![dx, y.clone()]);
                let num2 = Self::call_global(p.mul_id(), vec![x, dy]);
                let num = Self::call_global(p.sub_id(), vec![num1, num2]);
                let den = Self::call_global(p.mul_id(), vec![y.clone(), y]);
                let tangent = Self::call_global(p.div_id(), vec![num, den]);
                Ok((primal, tangent))
            }
            ScalarFloatOp::Neg(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                Ok((
                    Self::call_global(p.neg_id(), vec![x]),
                    Self::call_global(p.neg_id(), vec![dx]),
                ))
            }
            ScalarFloatOp::Abs(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let dx = Self::emit_atom(dx, Class::Raw, binds, next_local);
                let abs_id = GlobalId(match p {
                    FloatPrec::F32 => "Data.Machine.abs-f32".into(),
                    FloatPrec::F64 => "Data.Machine.abs-f64".into(),
                });
                let primal = Self::call_global(abs_id, vec![x.clone()]);
                let cond = Self::call_global(p.fge_id(), vec![x, p.lit(0.0)]);
                let tangent = Core::If(
                    Box::new(cond),
                    CBranch { weight: None, body: Box::new(dx.clone()) },
                    CBranch {
                        weight: None,
                        body: Box::new(Self::call_global(p.neg_id(), vec![dx])),
                    },
                );
                Ok((primal, tangent))
            }
            ScalarFloatOp::Sqrt(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let sqrt_id = GlobalId(match p {
                    FloatPrec::F32 => "Data.Machine.sqrt-f32".into(),
                    FloatPrec::F64 => "Data.Machine.sqrt-f64".into(),
                });
                let s = Self::emit_atom(
                    Self::call_global(sqrt_id, vec![x]),
                    Class::Raw,
                    binds,
                    next_local,
                );
                let two_s = Self::call_global(p.mul_id(), vec![p.lit(2.0), s.clone()]);
                let tangent = Self::call_global(p.div_id(), vec![dx, two_s]);
                Ok((s, tangent))
            }
            ScalarFloatOp::Sin(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let primal = Self::call_global(p.sin_id(), vec![x.clone()]);
                let cos_x = Self::call_global(p.cos_id(), vec![x]);
                let tangent = Self::call_global(p.mul_id(), vec![dx, cos_x]);
                Ok((primal, tangent))
            }
            ScalarFloatOp::Cos(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let primal = Self::call_global(p.cos_id(), vec![x.clone()]);
                let neg_sin = Self::call_global(p.neg_id(), vec![Self::call_global(p.sin_id(), vec![x])]);
                let tangent = Self::call_global(p.mul_id(), vec![dx, neg_sin]);
                Ok((primal, tangent))
            }
            ScalarFloatOp::Exp(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let e = Self::emit_atom(
                    Self::call_global(p.exp_id(), vec![x]),
                    Class::Raw,
                    binds,
                    next_local,
                );
                let tangent = Self::call_global(p.mul_id(), vec![dx, e.clone()]);
                Ok((e, tangent))
            }
            ScalarFloatOp::Log(p) => {
                let (x, dx) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let x = Self::emit_atom(x, Class::Raw, binds, next_local);
                let log_id = GlobalId(match p {
                    FloatPrec::F32 => "Data.AD.log-f32".into(),
                    FloatPrec::F64 => "Data.AD.log-f64".into(),
                });
                let primal = Self::call_global(log_id, vec![x.clone()]);
                let tangent = Self::call_global(p.div_id(), vec![dx, x]);
                Ok((primal, tangent))
            }
            ScalarFloatOp::Cmp(p) => {
                let (x, _) = self.jvp_expr(&args[0], env, binds, next_local, &p.ad_ty())?;
                let (y, _) = self.jvp_expr(&args[1], env, binds, next_local, &p.ad_ty())?;
                let cmp_res = Core::App(Box::new(head.clone()), vec![x, y]);
                Ok((cmp_res, Core::Bool(false)))
            }
        }
    }

    fn jvp_array_compute(
        &mut self,
        op: ArrayComputeOp,
        args: &[Core],
        env: &mut JvpEnv,
        binds: &mut Vec<JvpBind>,
        next_local: &mut usize,
        expected: &AdType,
    ) -> Result<(Core, Core)> {
        let head = self.ensure_compute_op(op);
        match op {
            ArrayComputeOp::DotF32 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (b, db) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::ArrayF32)?;
                let (m, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let b = Self::emit_atom(b, Class::Obj, binds, next_local);
                let m = Self::emit_atom(m, Class::Raw, binds, next_local);
                let primal = Core::App(Box::new(head.clone()), vec![a.clone(), b.clone(), m.clone()]);
                let d1 = Core::App(Box::new(head.clone()), vec![da, b, m.clone()]);
                let d2 = Core::App(Box::new(head), vec![a, db, m]);
                let tangent = Self::call_global(FloatPrec::F32.add_id(), vec![d1, d2]);
                Ok((primal, tangent))
            }
            ArrayComputeOp::DotF64 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF64)?;
                let (b, db) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::ArrayF64)?;
                let (m, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let b = Self::emit_atom(b, Class::Obj, binds, next_local);
                let m = Self::emit_atom(m, Class::Raw, binds, next_local);
                let primal = Core::App(Box::new(head.clone()), vec![a.clone(), b.clone(), m.clone()]);
                let d1 = Core::App(Box::new(head.clone()), vec![da, b, m.clone()]);
                let d2 = Core::App(Box::new(head), vec![a, db, m]);
                let tangent = Self::call_global(FloatPrec::F64.add_id(), vec![d1, d2]);
                Ok((primal, tangent))
            }
            ArrayComputeOp::ReduceF32 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (m, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let m = Self::emit_atom(m, Class::Raw, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![a, m.clone()]),
                    Core::App(Box::new(head), vec![da, m]),
                ))
            }
            ArrayComputeOp::ReduceF64 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF64)?;
                let (m, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let m = Self::emit_atom(m, Class::Raw, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![a, m.clone()]),
                    Core::App(Box::new(head), vec![da, m]),
                ))
            }
            ArrayComputeOp::ZipBinopF32 | ArrayComputeOp::ZipBinopF64 => {
                let is_f64 = op == ArrayComputeOp::ZipBinopF64;
                let arr_ty = if is_f64 { AdType::ArrayF64 } else { AdType::ArrayF32 };
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &arr_ty)?;
                let (b, db) = self.jvp_expr(&args[1], env, binds, next_local, &arr_ty)?;
                let (binop, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let da = Self::emit_atom(da, Class::Obj, binds, next_local);
                let b = Self::emit_atom(b, Class::Obj, binds, next_local);
                let db = Self::emit_atom(db, Class::Obj, binds, next_local);
                let primal = Core::App(Box::new(head.clone()), vec![a.clone(), b.clone(), binop.clone()]);
                let tangent = match binop {
                    Core::Int(0) | Core::Int(1) => {
                        Core::App(Box::new(head), vec![da, db, binop])
                    }
                    Core::Int(2) => {
                        let t1_opt = Core::App(Box::new(head.clone()), vec![da.clone(), b, Core::Int(2)]);
                        let t2_opt = Core::App(Box::new(head.clone()), vec![a, db, Core::Int(2)]);
                        let t1 = self.unwrap_option_obj(t1_opt, da.clone(), next_local);
                        let t2 = self.unwrap_option_obj(t2_opt, da, next_local);
                        Core::App(Box::new(head), vec![t1, t2, Core::Int(0)])
                    }
                    Core::Int(3) => {
                        let t1_opt = Core::App(Box::new(head.clone()), vec![da.clone(), b.clone(), Core::Int(2)]);
                        let t2_opt = Core::App(Box::new(head.clone()), vec![a, db, Core::Int(2)]);
                        let bb_opt = Core::App(Box::new(head.clone()), vec![b.clone(), b, Core::Int(2)]);
                        let t1 = self.unwrap_option_obj(t1_opt, da.clone(), next_local);
                        let t2 = self.unwrap_option_obj(t2_opt, da.clone(), next_local);
                        let num_opt = Core::App(Box::new(head.clone()), vec![t1, t2, Core::Int(1)]);
                        let num = self.unwrap_option_obj(num_opt, da.clone(), next_local);
                        let den = self.unwrap_option_obj(bb_opt, da, next_local);
                        Core::App(Box::new(head), vec![num, den, Core::Int(3)])
                    }
                    _ => Core::App(Box::new(head), vec![da, db, Core::Int(0)]),
                };
                Ok((primal, tangent))
            }
            ArrayComputeOp::MapUnaryF32 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (uop, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (c, dc) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::F32)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let da = Self::emit_atom(da, Class::Obj, binds, next_local);
                let c = Self::emit_atom(c, Class::Raw, binds, next_local);
                let dc = Self::emit_atom(dc, Class::Raw, binds, next_local);
                let primal = Core::App(Box::new(head.clone()), vec![a.clone(), uop.clone(), c.clone()]);
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF32);
                let fill = self.ensure_compute_op(ArrayComputeOp::FillF32);
                let tangent = match uop {
                    Core::Int(0) => da,
                    Core::Int(1) => {
                        let fdc = Core::App(Box::new(fill), vec![self.array_len_call(da.clone()), dc]);
                        let s = Core::App(Box::new(zip), vec![da.clone(), fdc, Core::Int(0)]);
                        self.unwrap_option_obj(s, da, next_local)
                    }
                    Core::Int(2) => {
                        let t1 = Core::App(Box::new(head.clone()), vec![da.clone(), Core::Int(2), c]);
                        let t2 = Core::App(Box::new(head), vec![a, Core::Int(2), dc]);
                        let s = Core::App(Box::new(zip), vec![t1, t2, Core::Int(0)]);
                        self.unwrap_option_obj(s, da, next_local)
                    }
                    Core::Int(3) => {
                        let two_a = Core::App(Box::new(head), vec![a, Core::Int(2), FloatPrec::F32.lit(2.0)]);
                        let p = Core::App(Box::new(zip), vec![two_a, da.clone(), Core::Int(2)]);
                        self.unwrap_option_obj(p, da, next_local)
                    }
                    Core::Int(5) => Core::App(Box::new(head), vec![da, Core::Int(5), FloatPrec::F32.lit(0.0)]),
                    _ => da,
                };
                Ok((primal, tangent))
            }
            ArrayComputeOp::ScaleF64 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF64)?;
                let (c, dc) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::F64)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let da = Self::emit_atom(da, Class::Obj, binds, next_local);
                let c = Self::emit_atom(c, Class::Raw, binds, next_local);
                let dc = Self::emit_atom(dc, Class::Raw, binds, next_local);
                let primal = Core::App(Box::new(head.clone()), vec![a.clone(), c.clone()]);
                let t1 = Core::App(Box::new(head.clone()), vec![da.clone(), c]);
                let t2 = Core::App(Box::new(head), vec![a, dc]);
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF64);
                let s = Core::App(Box::new(zip), vec![t1, t2, Core::Int(0)]);
                let tangent = self.unwrap_option_obj(s, da, next_local);
                Ok((primal, tangent))
            }
            ArrayComputeOp::FillF32 => {
                let (n, _) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::NonDiff)?;
                let (v, dv) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::F32)?;
                let n = Self::emit_atom(n, Class::TObj, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![n.clone(), v]),
                    Core::App(Box::new(head), vec![n, dv]),
                ))
            }
            ArrayComputeOp::FillF64 => {
                let (n, _) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::NonDiff)?;
                let (v, dv) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::F64)?;
                let n = Self::emit_atom(n, Class::TObj, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![n.clone(), v]),
                    Core::App(Box::new(head), vec![n, dv]),
                ))
            }
            ArrayComputeOp::RowReduceF32 => {
                let (a, da) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (r, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (c, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let (m, _) = self.jvp_expr(&args[3], env, binds, next_local, &AdType::NonDiff)?;
                let r = Self::emit_atom(r, Class::TObj, binds, next_local);
                let c = Self::emit_atom(c, Class::TObj, binds, next_local);
                let m = Self::emit_atom(m, Class::Raw, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![a, r.clone(), c.clone(), m.clone()]),
                    Core::App(Box::new(head), vec![da, r, c, m]),
                ))
            }
            ArrayComputeOp::RowBroadcastF32 => {
                let (v, dv) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (r, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (c, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let r = Self::emit_atom(r, Class::TObj, binds, next_local);
                let c = Self::emit_atom(c, Class::TObj, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![v, r.clone(), c.clone()]),
                    Core::App(Box::new(head), vec![dv, r, c]),
                ))
            }
            ArrayComputeOp::GatherF32 => {
                let (src, dsrc) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (idx, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (pol, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let idx = Self::emit_atom(idx, Class::Obj, binds, next_local);
                let pol = Self::emit_atom(pol, Class::Raw, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![src, idx.clone(), pol.clone()]),
                    Core::App(Box::new(head), vec![dsrc, idx, pol]),
                ))
            }
            ArrayComputeOp::ScatterAddF32 => {
                let (n, _) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::NonDiff)?;
                let (idx, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (vals, dvals) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::ArrayF32)?;
                let (pol, _) = self.jvp_expr(&args[3], env, binds, next_local, &AdType::NonDiff)?;
                let n = Self::emit_atom(n, Class::TObj, binds, next_local);
                let idx = Self::emit_atom(idx, Class::Obj, binds, next_local);
                let pol = Self::emit_atom(pol, Class::Raw, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![n.clone(), idx.clone(), vals, pol.clone()]),
                    Core::App(Box::new(head), vec![n, idx, dvals, pol]),
                ))
            }
            ArrayComputeOp::Stencil2DF32 | ArrayComputeOp::Stencil2DAdjointF32 => {
                let (img, dimg) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::ArrayF32)?;
                let (f, _) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::NonDiff)?;
                let (h, _) = self.jvp_expr(&args[2], env, binds, next_local, &AdType::NonDiff)?;
                let (w, _) = self.jvp_expr(&args[3], env, binds, next_local, &AdType::NonDiff)?;
                let (wt, _) = self.jvp_expr(&args[4], env, binds, next_local, &AdType::NonDiff)?;
                let (pol, _) = self.jvp_expr(&args[5], env, binds, next_local, &AdType::NonDiff)?;
                let (st, _) = self.jvp_expr(&args[6], env, binds, next_local, &AdType::NonDiff)?;
                let f = Self::emit_atom(f, Class::TObj, binds, next_local);
                let h = Self::emit_atom(h, Class::TObj, binds, next_local);
                let w = Self::emit_atom(w, Class::TObj, binds, next_local);
                let wt = Self::emit_atom(wt, Class::Obj, binds, next_local);
                let pol = Self::emit_atom(pol, Class::Raw, binds, next_local);
                let st = Self::emit_atom(st, Class::TObj, binds, next_local);
                Ok((
                    Core::App(Box::new(head.clone()), vec![img, f.clone(), h.clone(), w.clone(), wt.clone(), pol.clone(), st.clone()]),
                    Core::App(Box::new(head), vec![dimg, f, h, w, wt, pol, st]),
                ))
            }
            ArrayComputeOp::MapF32 => {
                let (a, da) = self.jvp_expr(&args[1], env, binds, next_local, &AdType::ArrayF32)?;
                let a = Self::emit_atom(a, Class::Obj, binds, next_local);
                let da = Self::emit_atom(da, Class::Obj, binds, next_local);
                let primal = Core::App(
                    Box::new(head),
                    vec![remap_locals(&args[0], &env.primal_map), a.clone()],
                );
                if let Some((g_id, g_cap)) = self.resolve_target_fn(&args[0]) {
                    let mut g_arg_tys: Vec<AdType> = g_cap
                        .iter()
                        .map(|c| self.expr_ad_type_quick(c, env))
                        .collect();
                    g_arg_tys.push(AdType::F32);
                    let (g_jvp_id, _) = self.gen_jvp_fn(&g_id, &g_arg_tys, &AdType::F32)?;
                    let helper_id = self.fresh_unit_id(&g_id.0, "map_jvp");
                    let mut h_params = Vec::new();
                    let mut h_pclasses = Vec::new();
                    let mut h_call_args = Vec::new();
                    let mut cap_vals = Vec::new();
                    let mut p_idx = 0usize;
                    for (c_expr, ct) in g_cap.iter().zip(g_arg_tys.iter()) {
                        let (cp, ctang) = self.jvp_expr(c_expr, env, binds, next_local, ct)?;
                        let ccls = ct.class(Class::Raw);
                        h_params.push(format!("c{p_idx}"));
                        h_pclasses.push(ccls);
                        h_call_args.push(Core::Var(LocalId(p_idx)));
                        cap_vals.push(cp);
                        p_idx += 1;
                        if ct.is_active() {
                            h_params.push(format!("dc{p_idx}"));
                            h_pclasses.push(ccls);
                            h_call_args.push(Core::Var(LocalId(p_idx)));
                            cap_vals.push(ctang);
                            p_idx += 1;
                        }
                    }
                    let lx = LocalId(p_idx);
                    let ldx = LocalId(p_idx + 1);
                    h_params.push("x".into());
                    h_params.push("dx".into());
                    h_pclasses.push(Class::Raw);
                    h_pclasses.push(Class::Raw);
                    h_call_args.push(Core::Var(lx));
                    h_call_args.push(Core::Var(ldx));
                    let lp = LocalId(p_idx + 2);
                    let lt = LocalId(p_idx + 3);
                    let h_body = Core::Case {
                        scrut: Box::new(Self::call_global(g_jvp_id, h_call_args)),
                        alts: vec![CAlt {
                            pat: CPat::Con {
                                tag: 0,
                                fields: vec![lp, lt],
                                classes: vec![Class::Raw, Class::Raw],
                            },
                            weight: None,
                            body: Box::new(Core::Var(lt)),
                        }],
                    };
                    self.add_fn(FnDef {
                        id: helper_id.clone(),
                        params: h_params,
                        body: h_body,
                        prim: None,
                        ctor: None,
                        hints: Default::default(),
                        meta: Default::default(),
                        repr: UnitRepr {
                            tparams: 0,
                            params: h_pclasses,
                            fields: Vec::new(),
                            data: None,
                            result: Some(Class::Raw),
                            dictionary_values: Vec::new(),
                            ..Default::default()
                        },
                    });
                    let mapper = Core::Lam { fn_id: helper_id, env: cap_vals, classes: Vec::new() };
                    let zip_with = self.ensure_compute_op(ArrayComputeOp::ZipWithF32);
                    let t_opt = Core::App(Box::new(zip_with), vec![mapper, a, da.clone()]);
                    let tangent = self.unwrap_option_obj(t_opt, da, next_local);
                    return Ok((primal, tangent));
                }
                Ok((primal, da))
            }
            ArrayComputeOp::TabulateF32 => {
                let (n, _) = self.jvp_expr(&args[0], env, binds, next_local, &AdType::NonDiff)?;
                let n = Self::emit_atom(n, Class::TObj, binds, next_local);
                let primal = Core::App(
                    Box::new(head.clone()),
                    vec![n.clone(), remap_locals(&args[1], &env.primal_map)],
                );
                if let Some((g_id, g_cap)) = self.resolve_target_fn(&args[1]) {
                    let mut g_arg_tys: Vec<AdType> = g_cap
                        .iter()
                        .map(|c| self.expr_ad_type_quick(c, env))
                        .collect();
                    if g_arg_tys.iter().any(AdType::is_active) {
                        g_arg_tys.push(AdType::NonDiff);
                        let (g_jvp_id, _) = self.gen_jvp_fn(&g_id, &g_arg_tys, &AdType::F32)?;
                        let helper_id = self.fresh_unit_id(&g_id.0, "tab_jvp");
                        let mut h_params = Vec::new();
                        let mut h_pclasses = Vec::new();
                        let mut h_call_args = Vec::new();
                        let mut cap_vals = Vec::new();
                        let mut p_idx = 0usize;
                        for (c_expr, ct) in g_cap.iter().zip(g_arg_tys.iter()) {
                            let (cp, ctang) = self.jvp_expr(c_expr, env, binds, next_local, ct)?;
                            let orig_cls = self.expr_class_quick(c_expr, env);
                            h_params.push(format!("c{p_idx}"));
                            h_pclasses.push(orig_cls);
                            h_call_args.push(Core::Var(LocalId(p_idx)));
                            cap_vals.push(cp);
                            p_idx += 1;
                            if ct.is_active() {
                                h_params.push(format!("dc{p_idx}"));
                                h_pclasses.push(ct.class(orig_cls));
                                h_call_args.push(Core::Var(LocalId(p_idx)));
                                cap_vals.push(ctang);
                                p_idx += 1;
                            }
                        }
                        let lk = LocalId(p_idx);
                        h_params.push("k".into());
                        h_pclasses.push(Class::TObj);
                        h_call_args.push(Core::Var(lk));
                        let lp = LocalId(p_idx + 1);
                        let lt = LocalId(p_idx + 2);
                        let h_body = Core::Case {
                            scrut: Box::new(Self::call_global(g_jvp_id, h_call_args)),
                            alts: vec![CAlt {
                                pat: CPat::Con {
                                    tag: 0,
                                    fields: vec![lp, lt],
                                    classes: vec![Class::Raw, Class::Raw],
                                },
                                weight: None,
                                body: Box::new(Core::Var(lt)),
                            }],
                        };
                        self.add_fn(FnDef {
                            id: helper_id.clone(),
                            params: h_params,
                            body: h_body,
                            prim: None,
                            ctor: None,
                            hints: Default::default(),
                            meta: Default::default(),
                            repr: UnitRepr {
                                tparams: 0,
                                params: h_pclasses,
                                fields: Vec::new(),
                                data: None,
                                result: Some(Class::Raw),
                                dictionary_values: Vec::new(),
                                ..Default::default()
                            },
                        });
                        let tab_fn = Core::Lam { fn_id: helper_id, env: cap_vals, classes: Vec::new() };
                        let tangent = Core::App(Box::new(head), vec![n, tab_fn]);
                        return Ok((primal, tangent));
                    }
                }
                let lp = Self::emit_atom(primal, Class::Obj, binds, next_local);
                let z = self.zero_like(lp.clone(), expected, next_local);
                Ok((lp, z))
            }
            _ => {
                let mut p_args = Vec::with_capacity(args.len());
                for a in args {
                    let at = self.expr_ad_type_quick(a, env);
                    let (ap, _) = self.jvp_expr(a, env, binds, next_local, &at)?;
                    p_args.push(ap);
                }
                let primal = Core::App(Box::new(head), p_args);
                let lp = Self::emit_atom(primal, expected.class(Class::Obj), binds, next_local);
                let z = self.zero_like(lp.clone(), expected, next_local);
                Ok((lp, z))
            }
        }
    }

    fn expr_ad_type_quick(&self, e: &Core, env: &JvpEnv) -> AdType {
        match e {
            Core::Var(l) => env.local_tys.get(l).cloned().unwrap_or(AdType::NonDiff),
            Core::App(h, args) => match &**h {
                Core::Global(g, _) => {
                    if let Some(op) = classify_scalar_float_op(&g.0) {
                        return match op {
                            ScalarFloatOp::Cmp(_) => AdType::NonDiff,
                            ScalarFloatOp::Add(p)
                            | ScalarFloatOp::Sub(p)
                            | ScalarFloatOp::Mul(p)
                            | ScalarFloatOp::Div(p)
                            | ScalarFloatOp::Neg(p)
                            | ScalarFloatOp::Abs(p)
                            | ScalarFloatOp::Sqrt(p)
                            | ScalarFloatOp::Sin(p)
                            | ScalarFloatOp::Cos(p)
                            | ScalarFloatOp::Exp(p)
                            | ScalarFloatOp::Log(p) => p.ad_ty(),
                        };
                    }
                    if let Some(f) = self.fns.get(g) {
                        if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                            return array_compute_ret_ad_type(op);
                        }
                        if f.prim.is_none() && f.ctor.is_none() && args.len() == f.params.len() {
                            let atys: Vec<AdType> = args
                                .iter()
                                .map(|a| self.expr_ad_type_quick(a, env))
                                .collect();
                            let (_, rty) = self.infer_fn_signature(g, &atys, &AdType::NonDiff);
                            return rty;
                        }
                    }
                    AdType::NonDiff
                }
                Core::Prim(PrimOp::ArrayCompute { op, .. }) => array_compute_ret_ad_type(*op),
                Core::Ctor(c, _) => {
                    let base = c.0.split('@').next().unwrap_or(&c.0);
                    let fields: Vec<AdType> = args
                        .iter()
                        .map(|a| self.expr_ad_type_quick(a, env))
                        .collect();
                    if base.starts_with("Tuple.") {
                        AdType::Tuple(fields)
                    } else if base == "Data.Core.Some" && fields.len() == 1 {
                        AdType::Option(Box::new(fields[0].clone()))
                    } else if fields.iter().any(AdType::is_active) {
                        let fclasses = self
                            .ctor_info
                            .get(c)
                            .map(|(_, fc)| fc.clone())
                            .unwrap_or_else(|| vec![Class::Raw; args.len()]);
                        AdType::Product { ctor: c.clone(), fields, field_classes: fclasses }
                    } else {
                        AdType::NonDiff
                    }
                }
                _ => AdType::NonDiff,
            },
            _ => AdType::NonDiff,
        }
    }

    fn expr_class_quick(&self, e: &Core, env: &JvpEnv) -> Class {
        match e {
            Core::Var(l) => env.local_classes.get(l).copied().unwrap_or(Class::Obj),
            _ => self.expr_ad_type_quick(e, env).class(Class::Obj),
        }
    }

    // -----------------------------------------------------------------------
    // REVERSE-MODE AD: Vector-Jacobian Product (VJP)
    // -----------------------------------------------------------------------

    /// Generate a reverse-mode companion `f$vjp(all_args...) -> (primal_res, pb_closure)`
    /// where `pb_closure(d_res)` returns:
    /// - `Unit` if `f` has 0 active parameters
    /// - `d_p` if `f` has 1 active parameter
    /// - `(d_p0, d_p1, ...)` tuple if `f` has `k >= 2` active parameters.
    fn gen_vjp_fn(
        &mut self,
        fn_id: &GlobalId,
        arg_tys: &[AdType],
        ret_hint: &AdType,
    ) -> Result<(GlobalId, AdType)> {
        let key = (fn_id.clone(), arg_tys.to_vec());
        if let Some(hit) = self.vjp_cache.get(&key) {
            return Ok(hit.clone());
        }
        let orig = self.fns.get(fn_id).cloned().ok_or_else(|| {
            Error::plain(format!("VJP: unknown function `{}`", fn_id.0))
        })?;
        let (local_tys, ret_ty) = self.infer_fn_locals(fn_id, arg_tys, ret_hint);
        let vjp_id = self.fresh_unit_id(&fn_id.0, "vjp");

        // Pre-insert into cache BEFORE transforming the body so recursive functions
        // resolve their self-calls to `vjp_id`!
        self.vjp_cache.insert(key, (vjp_id.clone(), ret_ty.clone()));

        let mut local_classes: HashMap<LocalId, Class> = orig
            .repr
            .params
            .iter()
            .enumerate()
            .map(|(i, c)| (LocalId(i), *c))
            .collect();
        collect_let_classes(&orig.body, &mut local_classes);

        let active_params: Vec<(LocalId, AdType, Class)> = arg_tys
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if t.is_active() {
                    let cls = orig.repr.params.get(i).copied().unwrap_or_else(|| t.class(Class::Raw));
                    Some((LocalId(i), t.clone(), cls))
                } else {
                    None
                }
            })
            .collect();

        let mut next_local = max_local_in_fn(&orig) + 2;
        let orig_ret_cls = orig.repr.result.unwrap_or_else(|| ret_ty.class(Class::Raw));

        let body = self.vjp_block(
            &orig.body,
            &active_params,
            &local_tys,
            &local_classes,
            &ret_ty,
            orig_ret_cls,
            &vjp_id.0,
            None,
            &mut next_local,
        )?;

        self.add_fn(FnDef {
            id: vjp_id.clone(),
            params: orig.params.clone(),
            body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: UnitRepr {
                tparams: 0,
                params: orig.repr.params.clone(),
                fields: Vec::new(),
                data: None,
                result: Some(Class::Obj), // returns `(primal_res, pullback_closure)`
                dictionary_values: Vec::new(),
                ..Default::default()
            },
        });

        Ok((vjp_id, ret_ty))
    }

    /// Generate a direct reverse-mode entrypoint `f$vjp_direct(orig_params..., dy) -> (primal_y, dx)`
    /// for a top-level `vjp2` site. Directly calls the hoisted pullback function via `Core::Global`
    /// without allocating a pullback closure, enabling outer AD passes (like forward-over-reverse HVP)
    /// to differentiate cleanly through `vjp`!
    fn gen_vjp_direct_fn(
        &mut self,
        fn_id: &GlobalId,
        arg_tys: &[AdType],
        ret_hint: &AdType,
    ) -> Result<(GlobalId, AdType)> {
        let key = (fn_id.clone(), arg_tys.to_vec());
        if let Some(hit) = self.vjp_direct_cache.get(&key) {
            return Ok(hit.clone());
        }
        let orig = self.fns.get(fn_id).cloned().ok_or_else(|| {
            Error::plain(format!("VJP direct: unknown function `{}`", fn_id.0))
        })?;
        let (_local_tys, ret_ty) = self.infer_fn_locals(fn_id, arg_tys, ret_hint);
        let vjp_direct_id = self.fresh_unit_id(&fn_id.0, "vjp_direct");
        self.vjp_direct_cache.insert(key, (vjp_direct_id.clone(), ret_ty.clone()));

        let orig_ret_cls = orig.repr.result.unwrap_or_else(|| ret_ty.class(Class::Raw));
        let mut params = orig.params.clone();
        let mut pclasses = orig.repr.params.clone();
        let dy_local = LocalId(params.len());
        params.push("dy".into());
        pclasses.push(orig_ret_cls);

        // Shift any non-parameter local above `dy_local` if `orig.body` happened to use `LocalId(params.len())`.
        let mut local_classes: HashMap<LocalId, Class> = orig
            .repr
            .params
            .iter()
            .enumerate()
            .map(|(i, c)| (LocalId(i), *c))
            .collect();
        collect_let_classes(&orig.body, &mut local_classes);
        local_classes.insert(dy_local, orig_ret_cls);

        let active_params: Vec<(LocalId, AdType, Class)> = arg_tys
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if t.is_active() {
                    let cls = orig.repr.params.get(i).copied().unwrap_or_else(|| t.class(Class::Raw));
                    Some((LocalId(i), t.clone(), cls))
                } else {
                    None
                }
            })
            .collect();

        // Freshen non-param locals in `orig.body` so none collide with `dy_local`.
        let mut next_local = (max_local_in_fn(&orig) + 2).max(dy_local.0 + 1);
        let mut next_join = max_join_in_expr(&orig.body) + 1;
        let shifted_body = freshen_non_params(&orig.body, orig.params.len(), &mut next_local, &mut next_join);
        let mut shifted_local_tys = HashMap::new();
        for (i, t) in arg_tys.iter().enumerate() {
            shifted_local_tys.insert(LocalId(i), t.clone());
        }
        let mut shifted_local_classes: HashMap<LocalId, Class> = orig
            .repr
            .params
            .iter()
            .enumerate()
            .map(|(i, c)| (LocalId(i), *c))
            .collect();
        collect_let_classes(&shifted_body, &mut shifted_local_classes);
        shifted_local_classes.insert(dy_local, orig_ret_cls);
        let mut visiting = HashSet::new();
        visiting.insert(fn_id.clone());
        for _ in 0..6 {
            let _ = self.infer_expr_types(&shifted_body, &mut shifted_local_tys, &ret_ty, &mut visiting, 0);
        }

        let body = self.vjp_block(
            &shifted_body,
            &active_params,
            &shifted_local_tys,
            &shifted_local_classes,
            &ret_ty,
            orig_ret_cls,
            &vjp_direct_id.0,
            Some(dy_local),
            &mut next_local,
        )?;

        self.add_fn(FnDef {
            id: vjp_direct_id.clone(),
            params,
            body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: UnitRepr {
                tparams: 0,
                params: pclasses,
                fields: Vec::new(),
                data: None,
                result: Some(Class::Obj), // returns `(primal_y, dx)`
                dictionary_values: Vec::new(),
                ..Default::default()
            },
        });

        Ok((vjp_direct_id, ret_ty))
    }

    /// Transform a block expression `expr` (whose active inputs are `active_inputs`)
    /// into a Core expression returning `(primal_res, pullback_closure)` — or, when
    /// `direct_dy` is `Some(dy_local)`, directly calling the pullback function via
    /// `Core::Global` and returning `(primal_res, dx)`.
    #[allow(clippy::too_many_arguments)]
    fn vjp_block(
        &mut self,
        expr: &Core,
        active_inputs: &[(LocalId, AdType, Class)],
        local_tys: &HashMap<LocalId, AdType>,
        local_classes: &HashMap<LocalId, Class>,
        ret_ty: &AdType,
        ret_cls: Class,
        name_prefix: &str,
        direct_dy: Option<LocalId>,
        next_local: &mut usize,
    ) -> Result<Core> {
        // ANF-flatten `expr` into a linear tape of `Let` bindings + a terminal expression.
        let mut steps: Vec<AnfStep> = Vec::new();
        let mut step_tys: HashMap<LocalId, AdType> = local_tys.clone();
        let mut step_classes: HashMap<LocalId, Class> = local_classes.clone();
        let term = self.anf_flatten(expr, &mut steps, &mut step_tys, &mut step_classes, next_local);

        // Forward sweep + collect reverse adjoint instructions for every step!
        let mut fwd_binds: Vec<(LocalId, Class, Core)> = Vec::new();
        let mut rev_ops: Vec<RevOp> = Vec::new();

        for step in steps {
            // Recognize our inline single-constructor unpack marker emitted by `anf_flatten`:
            // `Core::Case { scrut: Var(scrut_var), alts: [Con(tag, fields, classes) -> Unit] }`
            // where `step.var == scrut_var`.
            if let Core::Case { scrut, alts } = &step.rhs {
                if alts.len() == 1 && matches!(&*alts[0].body, Core::Unit) {
                    if let (Core::Var(sv), CPat::Con { tag, fields, classes }) =
                        (&**scrut, &alts[0].pat)
                    {
                        if *sv == step.var {
                            let mut field_info = Vec::with_capacity(fields.len());
                            for (idx, (&fl, &fc)) in fields.iter().zip(classes.iter()).enumerate() {
                                let mut fresh_fs = Vec::with_capacity(fields.len());
                                for _ in fields {
                                    let nl = LocalId(*next_local);
                                    *next_local += 1;
                                    fresh_fs.push(nl);
                                }
                                let proj = Core::Case {
                                    scrut: Box::new(Core::Var(*sv)),
                                    alts: vec![CAlt {
                                        pat: CPat::Con {
                                            tag: *tag,
                                            fields: fresh_fs.clone(),
                                            classes: classes.clone(),
                                        },
                                        weight: None,
                                        body: Box::new(Core::Var(fresh_fs[idx])),
                                    }],
                                };
                                fwd_binds.push((fl, fc, proj));
                                let ft = step_tys.get(&fl).cloned().unwrap_or(AdType::NonDiff);
                                field_info.push((fl, ft, fc));
                            }
                            if step.ty.is_active() {
                                rev_ops.push(RevOp::UnpackCtor {
                                    scrut: *sv,
                                    scrut_ty: step.ty.clone(),
                                    fields: field_info,
                                });
                            }
                            continue;
                        }
                    }
                }
            }
            step_tys.insert(step.var, step.ty.clone());
            step_classes.insert(step.var, step.cls);
            self.vjp_emit_step(
                step.var,
                step.cls,
                &step.ty,
                &step.rhs,
                &mut fwd_binds,
                &mut rev_ops,
                &mut step_tys,
                &mut step_classes,
                name_prefix,
                next_local,
            )?;
        }

        // Handle the terminal expression: if it's not a simple `Var`, bind it to `y_var`.
        let y_var = match &term {
            Core::Var(l) => *l,
            _ => {
                let y = LocalId(*next_local);
                *next_local += 1;
                step_tys.insert(y, ret_ty.clone());
                step_classes.insert(y, ret_cls);
                self.vjp_emit_step(
                    y,
                    ret_cls,
                    ret_ty,
                    &term,
                    &mut fwd_binds,
                    &mut rev_ops,
                    &mut step_tys,
                    &mut step_classes,
                    name_prefix,
                    next_local,
                )?;
                y
            }
        };

        // Build the hoisted pullback function `pb(captured_primals..., dy) -> d_active_inputs`.
        let (pb_id, cap_env, pb_ret_cls) = self.build_pullback_fn(
            y_var,
            ret_ty,
            ret_cls,
            active_inputs,
            &rev_ops,
            &step_tys,
            &step_classes,
            name_prefix,
        );

        let result_pair = if let Some(dy_l) = direct_dy {
            let mut call_args = cap_env;
            call_args.push(Core::Var(dy_l));
            let dx_call = Self::call_global(pb_id, call_args);
            let ldx = LocalId(*next_local);
            *next_local += 1;
            let tup = self.mk_tuple(vec![(Core::Var(y_var), ret_cls), (Core::Var(ldx), pb_ret_cls)]);
            Core::Let(ldx, pb_ret_cls, Box::new(dx_call), Box::new(tup))
        } else {
            let pb_closure = Core::Lam { fn_id: pb_id, env: cap_env, classes: Vec::new() };
            self.mk_tuple(vec![(Core::Var(y_var), ret_cls), (pb_closure, Class::Obj)])
        };
        let mut body = result_pair;
        for (l, c, rhs) in fwd_binds.into_iter().rev() {
            body = Core::Let(l, c, Box::new(rhs), Box::new(body));
        }
        Ok(body)
    }

    fn anf_flatten(
        &self,
        expr: &Core,
        steps: &mut Vec<AnfStep>,
        local_tys: &mut HashMap<LocalId, AdType>,
        local_classes: &mut HashMap<LocalId, Class>,
        next_local: &mut usize,
    ) -> Core {
        match expr {
            Core::Let(x, c, rhs, body) => {
                let flat_rhs = self.anf_flatten_rhs(rhs, steps, local_tys, local_classes, next_local);
                let inferred = self.infer_single_expr_ty(&flat_rhs, local_tys);
                let x_ty = local_tys
                    .get(x)
                    .cloned()
                    .unwrap_or(AdType::NonDiff)
                    .merge(&inferred);
                local_tys.insert(*x, x_ty.clone());
                local_classes.insert(*x, *c);
                steps.push(AnfStep { var: *x, cls: *c, ty: x_ty, rhs: flat_rhs });
                self.anf_flatten(body, steps, local_tys, local_classes, next_local)
            }
            Core::Case { scrut, alts } if alts.len() == 1 => {
                if let CPat::Con { tag, fields, classes } = &alts[0].pat {
                    // Single-constructor destructuring (tuples, records, NDView, View1..3):
                    // flatten directly into linear projections so no pullback closure is allocated!
                    let scrut_atom =
                        self.anf_flatten_arg(scrut, steps, local_tys, local_classes, next_local);
                    if let Core::Var(scrut_var) = scrut_atom {
                        let scrut_ty =
                            local_tys.get(&scrut_var).cloned().unwrap_or(AdType::NonDiff);
                        let field_tys: Vec<AdType> = match &scrut_ty {
                            AdType::Tuple(fs) if fs.len() == fields.len() => fs.clone(),
                            AdType::Product { fields: fs, .. } if fs.len() == fields.len() => {
                                fs.clone()
                            }
                            _ => fields
                                .iter()
                                .map(|fl| local_tys.get(fl).cloned().unwrap_or(AdType::NonDiff))
                                .collect(),
                        };
                        let mut field_specs = Vec::with_capacity(fields.len());
                        for ((&fl, ft), &fc) in
                            fields.iter().zip(field_tys.iter()).zip(classes.iter())
                        {
                            local_tys.insert(fl, ft.clone());
                            local_classes.insert(fl, fc);
                            field_specs.push((fl, ft.clone(), fc));
                        }
                        steps.push(AnfStep {
                            var: scrut_var,
                            cls: Class::Obj,
                            ty: scrut_ty,
                            rhs: Core::Case {
                                scrut: Box::new(Core::Var(scrut_var)),
                                alts: vec![CAlt {
                                    pat: CPat::Con {
                                        tag: *tag,
                                        fields: fields.clone(),
                                        classes: classes.clone(),
                                    },
                                    weight: None,
                                    body: Box::new(Core::Unit),
                                }],
                            },
                        });
                        let _ = field_specs;
                        return self.anf_flatten(
                            &alts[0].body,
                            steps,
                            local_tys,
                            local_classes,
                            next_local,
                        );
                    }
                }
                self.anf_flatten_rhs(expr, steps, local_tys, local_classes, next_local)
            }
            other => self.anf_flatten_rhs(other, steps, local_tys, local_classes, next_local),
        }
    }

    fn anf_flatten_rhs(
        &self,
        expr: &Core,
        steps: &mut Vec<AnfStep>,
        local_tys: &mut HashMap<LocalId, AdType>,
        local_classes: &mut HashMap<LocalId, Class>,
        next_local: &mut usize,
    ) -> Core {
        match expr {
            Core::Let(..) => self.anf_flatten(expr, steps, local_tys, local_classes, next_local),
            Core::App(h, args) => {
                let mut flat_args = Vec::with_capacity(args.len());
                for a in args {
                    let fa = self.anf_flatten_arg(a, steps, local_tys, local_classes, next_local);
                    flat_args.push(fa);
                }
                Core::App(h.clone(), flat_args)
            }
            other => other.clone(),
        }
    }

    fn anf_flatten_arg(
        &self,
        arg: &Core,
        steps: &mut Vec<AnfStep>,
        local_tys: &mut HashMap<LocalId, AdType>,
        local_classes: &mut HashMap<LocalId, Class>,
        next_local: &mut usize,
    ) -> Core {
        match arg {
            Core::Var(_)
            | Core::Int(_)
            | Core::Bool(_)
            | Core::Unit
            | Core::ConstBig { .. }
            | Core::Global(..)
            | Core::Ctor(..)
            | Core::Prim(..)
            | Core::Lam { .. } => arg.clone(),
            _ => {
                let flat = self.anf_flatten_rhs(arg, steps, local_tys, local_classes, next_local);
                let ty = self.infer_single_expr_ty(&flat, local_tys);
                let cls = self.infer_expr_class(&flat, &ty, local_classes);
                let l = LocalId(*next_local);
                *next_local += 1;
                local_tys.insert(l, ty.clone());
                local_classes.insert(l, cls);
                steps.push(AnfStep { var: l, cls, ty, rhs: flat });
                Core::Var(l)
            }
        }
    }

    fn infer_expr_class(
        &self,
        e: &Core,
        ty: &AdType,
        local_classes: &HashMap<LocalId, Class>,
    ) -> Class {
        if ty.is_active() {
            return ty.class(Class::Raw);
        }
        match e {
            Core::Var(l) => local_classes.get(l).copied().unwrap_or(Class::Raw),
            Core::Int(_) | Core::Bool(_) | Core::Unit | Core::Wide(_) => Class::Raw,
            Core::ConstBig { .. } => Class::TObj,
            Core::ConstBytes(_) | Core::ConstString { .. } | Core::ConstSym(_) | Core::Lam { .. } => {
                Class::Obj
            }
            Core::App(h, _) => match &**h {
                Core::Global(g, _) => self
                    .fns
                    .get(g)
                    .and_then(|f| f.repr.result)
                    .unwrap_or_else(|| {
                        if classify_scalar_float_op(&g.0).is_some() {
                            Class::Raw
                        } else {
                            Class::Obj
                        }
                    }),
                Core::Ctor(..) => Class::Obj,
                Core::Prim(op) => crate::codegen::prim_result_class(*op).unwrap_or(Class::Obj),
                _ => Class::Obj,
            },
            Core::If(_, t, _) => self.infer_expr_class(&t.body, ty, local_classes),
            Core::Case { alts, .. } => alts
                .first()
                .map(|a| self.infer_expr_class(&a.body, ty, local_classes))
                .unwrap_or(Class::Obj),
            _ => Class::Obj,
        }
    }

    fn infer_single_expr_ty(&self, e: &Core, local_tys: &HashMap<LocalId, AdType>) -> AdType {
        match e {
            Core::Var(l) => local_tys.get(l).cloned().unwrap_or(AdType::NonDiff),
            Core::Let(x, _, rhs, body) => {
                let mut sub = local_tys.clone();
                let rt = self.infer_single_expr_ty(rhs, &sub);
                sub.insert(*x, rt);
                self.infer_single_expr_ty(body, &sub)
            }
            Core::If(_, t, f) => {
                let tt = self.infer_single_expr_ty(&t.body, local_tys);
                let ft = self.infer_single_expr_ty(&f.body, local_tys);
                tt.merge(&ft)
            }
            Core::Case { scrut, alts } => {
                let st = self.infer_single_expr_ty(scrut, local_tys);
                let mut out = AdType::NonDiff;
                for alt in alts {
                    let mut sub = local_tys.clone();
                    match &alt.pat {
                        CPat::Con { tag, fields, .. } => match &st {
                            AdType::Tuple(fs) if *tag == 0 && fs.len() == fields.len() => {
                                for (&fl, ft) in fields.iter().zip(fs.iter()) {
                                    sub.insert(fl, ft.clone());
                                }
                            }
                            AdType::Product { fields: fs, .. } if fs.len() == fields.len() => {
                                for (&fl, ft) in fields.iter().zip(fs.iter()) {
                                    sub.insert(fl, ft.clone());
                                }
                            }
                            AdType::Option(inner) if *tag == 1 && fields.len() == 1 => {
                                sub.insert(fields[0], (**inner).clone());
                            }
                            _ => {}
                        },
                        CPat::Var(l, _) => {
                            sub.insert(*l, st.clone());
                        }
                        CPat::Wild => {}
                    }
                    out = out.merge(&self.infer_single_expr_ty(&alt.body, &sub));
                }
                out
            }
            Core::Join { body, rest, .. } => {
                let bt = self.infer_single_expr_ty(body, local_tys);
                let rt = self.infer_single_expr_ty(rest, local_tys);
                bt.merge(&rt)
            }
            Core::App(h, args) => match &**h {
                Core::Global(g, _) => {
                    if let Some(op) = classify_scalar_float_op(&g.0) {
                        return match op {
                            ScalarFloatOp::Cmp(_) => AdType::NonDiff,
                            ScalarFloatOp::Add(p)
                            | ScalarFloatOp::Sub(p)
                            | ScalarFloatOp::Mul(p)
                            | ScalarFloatOp::Div(p)
                            | ScalarFloatOp::Neg(p)
                            | ScalarFloatOp::Abs(p)
                            | ScalarFloatOp::Sqrt(p)
                            | ScalarFloatOp::Sin(p)
                            | ScalarFloatOp::Cos(p)
                            | ScalarFloatOp::Exp(p)
                            | ScalarFloatOp::Log(p) => p.ad_ty(),
                        };
                    }
                    if let Some(f) = self.fns.get(g) {
                        if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                            return array_compute_ret_ad_type(op);
                        }
                        if f.prim.is_none() && f.ctor.is_none() {
                            let atys: Vec<AdType> = args
                                .iter()
                                .map(|a| self.infer_single_expr_ty(a, local_tys))
                                .collect();
                            let (_, rty) = self.infer_fn_signature(g, &atys, &AdType::NonDiff);
                            return rty;
                        }
                    }
                    AdType::NonDiff
                }
                Core::Prim(PrimOp::ArrayCompute { op, .. }) => array_compute_ret_ad_type(*op),
                Core::Ctor(c, _) => {
                    let base = c.0.split('@').next().unwrap_or(&c.0);
                    let fields: Vec<AdType> = args
                        .iter()
                        .map(|a| self.infer_single_expr_ty(a, local_tys))
                        .collect();
                    if base.starts_with("Tuple.") {
                        AdType::Tuple(fields)
                    } else if base == "Data.Core.Some" && fields.len() == 1 {
                        AdType::Option(Box::new(fields[0].clone()))
                    } else if fields.iter().any(AdType::is_active) {
                        let fclasses = self
                            .ctor_info
                            .get(c)
                            .map(|(_, fc)| fc.clone())
                            .unwrap_or_else(|| vec![Class::Raw; args.len()]);
                        AdType::Product { ctor: c.clone(), fields, field_classes: fclasses }
                    } else {
                        AdType::NonDiff
                    }
                }
                _ => AdType::NonDiff,
            },
            _ => AdType::NonDiff,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn vjp_emit_step(
        &mut self,
        out_var: LocalId,
        out_cls: Class,
        out_ty: &AdType,
        rhs: &Core,
        fwd_binds: &mut Vec<(LocalId, Class, Core)>,
        rev_ops: &mut Vec<RevOp>,
        step_tys: &mut HashMap<LocalId, AdType>,
        step_classes: &mut HashMap<LocalId, Class>,
        name_prefix: &str,
        next_local: &mut usize,
    ) -> Result<()> {
        match rhs {
            Core::Var(src) => {
                fwd_binds.push((out_var, out_cls, Core::Var(*src)));
                if out_ty.is_active() {
                    rev_ops.push(RevOp::Copy { out: out_var, src: *src, ty: out_ty.clone() });
                }
                Ok(())
            }
            Core::If(cond, t_br, f_br) => {
                if !out_ty.is_active() {
                    fwd_binds.push((out_var, out_cls, rhs.clone()));
                    return Ok(());
                }
                // Find all active free variables referenced in either branch.
                let mut fvs = Vec::new();
                collect_free_locals(&t_br.body, &HashSet::new(), &mut fvs);
                collect_free_locals(&f_br.body, &HashSet::new(), &mut fvs);
                let mut seen_fv = HashSet::new();
                let active_fvs: Vec<(LocalId, AdType, Class)> = fvs
                    .into_iter()
                    .filter(|l| seen_fv.insert(*l))
                    .filter_map(|l| {
                        let t = step_tys.get(&l)?.clone();
                        if t.is_active() {
                            let c = step_classes.get(&l).copied().unwrap_or_else(|| t.class(Class::Raw));
                            Some((l, t, c))
                        } else {
                            None
                        }
                    })
                    .collect();
                let t_vjp = self.vjp_block(
                    &t_br.body,
                    &active_fvs,
                    step_tys,
                    step_classes,
                    out_ty,
                    out_cls,
                    &format!("{name_prefix}_t"),
                    None,
                    next_local,
                )?;
                let f_vjp = self.vjp_block(
                    &f_br.body,
                    &active_fvs,
                    step_tys,
                    step_classes,
                    out_ty,
                    out_cls,
                    &format!("{name_prefix}_f"),
                    None,
                    next_local,
                )?;
                let pair_var = LocalId(*next_local);
                let pb_var = LocalId(*next_local + 1);
                *next_local += 2;
                let if_pair = Core::If(
                    cond.clone(),
                    CBranch { weight: t_br.weight, body: Box::new(t_vjp) },
                    CBranch { weight: f_br.weight, body: Box::new(f_vjp) },
                );
                fwd_binds.push((pair_var, Class::Obj, if_pair));
                fwd_binds.push((
                    out_var,
                    out_cls,
                    project_pair_field(Core::Var(pair_var), 0, out_cls, Class::Obj, next_local),
                ));
                fwd_binds.push((
                    pb_var,
                    Class::Obj,
                    project_pair_field(Core::Var(pair_var), 1, out_cls, Class::Obj, next_local),
                ));
                step_classes.insert(pb_var, Class::Obj);
                if !active_fvs.is_empty() {
                    rev_ops.push(RevOp::CallPullback {
                        out: out_var,
                        pb_var,
                        targets: active_fvs,
                    });
                }
                Ok(())
            }
            Core::Case { scrut, alts } => {
                if !out_ty.is_active() {
                    fwd_binds.push((out_var, out_cls, rhs.clone()));
                    return Ok(());
                }
                // Bind scrutinee to a local if it isn't already a `Var`.
                let scrut_var = match &**scrut {
                    Core::Var(l) => *l,
                    other => {
                        let sv = LocalId(*next_local);
                        *next_local += 1;
                        let st = self.infer_single_expr_ty(other, step_tys);
                        let sc = st.class(Class::Obj);
                        step_tys.insert(sv, st.clone());
                        step_classes.insert(sv, sc);
                        self.vjp_emit_step(
                            sv,
                            sc,
                            &st,
                            other,
                            fwd_binds,
                            rev_ops,
                            step_tys,
                            step_classes,
                            name_prefix,
                            next_local,
                        )?;
                        sv
                    }
                };
                let scrut_ty = step_tys.get(&scrut_var).cloned().unwrap_or(AdType::NonDiff);
                let scrut_cls = step_classes.get(&scrut_var).copied().unwrap_or(Class::Obj);

                // Collect active outer free variables across all arms (including `scrut_var` if active).
                let mut fvs = Vec::new();
                if scrut_ty.is_active() {
                    fvs.push(scrut_var);
                }
                for alt in alts {
                    let mut bound = HashSet::new();
                    match &alt.pat {
                        CPat::Con { fields, .. } => {
                            for &fl in fields {
                                bound.insert(fl);
                            }
                        }
                        CPat::Var(l, _) => {
                            bound.insert(*l);
                        }
                        CPat::Wild => {}
                    }
                    collect_free_locals(&alt.body, &bound, &mut fvs);
                }
                let mut seen_outer = HashSet::new();
                let active_outer: Vec<(LocalId, AdType, Class)> = fvs
                    .into_iter()
                    .filter(|l| seen_outer.insert(*l))
                    .filter_map(|l| {
                        let t = step_tys.get(&l)?.clone();
                        if t.is_active() {
                            let c = step_classes.get(&l).copied().unwrap_or_else(|| t.class(Class::Raw));
                            Some((l, t, c))
                        } else {
                            None
                        }
                    })
                    .collect();

                let mut new_alts = Vec::with_capacity(alts.len());
                for (ai, alt) in alts.iter().enumerate() {
                    let mut arm_tys = step_tys.clone();
                    let mut arm_classes = step_classes.clone();
                    // Build the active inputs of this arm: active pattern binders + active outer vars (excluding scrut_var, which we reconstruct from pattern binder cotangents!).
                    let mut arm_inputs: Vec<(LocalId, AdType, Class)> = Vec::new();
                    let mut pat_binders: Vec<(LocalId, AdType, Class)> = Vec::new();
                    match &alt.pat {
                        CPat::Con { tag, fields, classes } => {
                            let field_ad_tys: Vec<AdType> = match &scrut_ty {
                                AdType::Tuple(fs) if fs.len() == fields.len() => fs.clone(),
                                AdType::Product { fields: fs, .. } if fs.len() == fields.len() => {
                                    fs.clone()
                                }
                                AdType::Option(inner) if *tag == 1 && fields.len() == 1 => {
                                    vec![(**inner).clone()]
                                }
                                _ => fields
                                    .iter()
                                    .map(|fl| step_tys.get(fl).cloned().unwrap_or(AdType::NonDiff))
                                    .collect(),
                            };
                            for ((&fl, ft), &fc) in
                                fields.iter().zip(field_ad_tys.iter()).zip(classes.iter())
                            {
                                arm_tys.insert(fl, ft.clone());
                                arm_classes.insert(fl, fc);
                                pat_binders.push((fl, ft.clone(), fc));
                                if ft.is_active() {
                                    arm_inputs.push((fl, ft.clone(), fc));
                                }
                            }
                        }
                        CPat::Var(l, c) => {
                            arm_tys.insert(*l, scrut_ty.clone());
                            arm_classes.insert(*l, *c);
                            pat_binders.push((*l, scrut_ty.clone(), *c));
                            if scrut_ty.is_active() {
                                arm_inputs.push((*l, scrut_ty.clone(), *c));
                            }
                        }
                        CPat::Wild => {}
                    }
                    let n_pat_active = arm_inputs.len();
                    for item in &active_outer {
                        if item.0 != scrut_var && !arm_inputs.iter().any(|x| x.0 == item.0) {
                            arm_inputs.push(item.clone());
                        }
                    }

                    let arm_vjp = self.vjp_block(
                        &alt.body,
                        &arm_inputs,
                        &arm_tys,
                        &arm_classes,
                        out_ty,
                        out_cls,
                        &format!("{name_prefix}_a{ai}"),
                        None,
                        next_local,
                    )?;

                    // Wrap the arm's pullback so it returns cotangents matching `active_outer`
                    // (rebuilding `d_scrut` from the active pattern binders' cotangents!).
                    let adapted = self.adapt_case_arm_pullback(
                        arm_vjp,
                        out_cls,
                        &alt.pat,
                        scrut_var,
                        &scrut_ty,
                        scrut_cls,
                        &pat_binders,
                        n_pat_active,
                        &arm_inputs,
                        &active_outer,
                        &format!("{name_prefix}_ad{ai}"),
                        next_local,
                    );
                    new_alts.push(CAlt { pat: alt.pat.clone(), weight: alt.weight, body: Box::new(adapted) });
                }

                let pair_var = LocalId(*next_local);
                let pb_var = LocalId(*next_local + 1);
                *next_local += 2;
                fwd_binds.push((
                    pair_var,
                    Class::Obj,
                    Core::Case { scrut: Box::new(Core::Var(scrut_var)), alts: new_alts },
                ));
                fwd_binds.push((
                    out_var,
                    out_cls,
                    project_pair_field(Core::Var(pair_var), 0, out_cls, Class::Obj, next_local),
                ));
                fwd_binds.push((
                    pb_var,
                    Class::Obj,
                    project_pair_field(Core::Var(pair_var), 1, out_cls, Class::Obj, next_local),
                ));
                step_classes.insert(pb_var, Class::Obj);
                if !active_outer.is_empty() {
                    rev_ops.push(RevOp::CallPullback {
                        out: out_var,
                        pb_var,
                        targets: active_outer,
                    });
                }
                Ok(())
            }
            Core::Join { id, params, body, rest } => {
                // Lift the tail-recursive `Join` into a hoisted recursive helper function,
                // freshening body binders so they never shadow lifted free variables,
                // replace `Jump`s with calls to that helper, and differentiate normally!
                let mut fvs = Vec::new();
                let bound: HashSet<LocalId> = params.iter().map(|(l, _)| *l).collect();
                collect_free_locals(body, &bound, &mut fvs);
                let mut seen_fv = HashSet::new();
                fvs.retain(|l| seen_fv.insert(*l));
                let join_fn_id = self.fresh_unit_id(name_prefix, "join");
                let mut j_params = Vec::new();
                let mut j_pclasses = Vec::new();
                let mut subst_map: HashMap<LocalId, LocalId> = HashMap::new();
                let mut cap_args = Vec::new();
                for (i, fv) in fvs.iter().enumerate() {
                    j_params.push(format!("fv{i}"));
                    j_pclasses.push(step_classes.get(fv).copied().unwrap_or(Class::Raw));
                    subst_map.insert(*fv, LocalId(i));
                    cap_args.push(Core::Var(*fv));
                }
                let base = fvs.len();
                for (j, (pl, pc)) in params.iter().enumerate() {
                    j_params.push(format!("jp{j}"));
                    j_pclasses.push(*pc);
                    subst_map.insert(*pl, LocalId(base + j));
                }
                let cap_in_fn: Vec<Core> = (0..base).map(|i| Core::Var(LocalId(i))).collect();
                let mut lift_local = base + params.len() + 1;
                let mut lift_join = max_join_in_expr(body) + 1;
                let body_mapped =
                    freshen_with_seed(body, &subst_map, &mut lift_local, &mut lift_join);
                let lifted_body = replace_jump_with_call(
                    &body_mapped,
                    *id,
                    &join_fn_id,
                    &cap_in_fn,
                );
                self.add_fn(FnDef {
                    id: join_fn_id.clone(),
                    params: j_params,
                    body: lifted_body,
                    prim: None,
                    ctor: None,
                    hints: Default::default(),
                    meta: Default::default(),
                    repr: UnitRepr {
                        tparams: 0,
                        params: j_pclasses,
                        fields: Vec::new(),
                        data: None,
                        result: Some(out_cls),
                        dictionary_values: Vec::new(),
                        ..Default::default()
                    },
                });
                let rewritten_rest = replace_jump_with_call(rest, *id, &join_fn_id, &cap_args);
                self.vjp_emit_step(
                    out_var,
                    out_cls,
                    out_ty,
                    &rewritten_rest,
                    fwd_binds,
                    rev_ops,
                    step_tys,
                    step_classes,
                    name_prefix,
                    next_local,
                )
            }
            Core::App(head, args) => self.vjp_emit_app(
                out_var,
                out_cls,
                out_ty,
                head,
                args,
                fwd_binds,
                rev_ops,
                step_tys,
                step_classes,
                name_prefix,
                next_local,
            ),
            _ => {
                fwd_binds.push((out_var, out_cls, rhs.clone()));
                Ok(())
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn adapt_case_arm_pullback(
        &mut self,
        arm_vjp: Core,
        out_cls: Class,
        pat: &CPat,
        scrut_var: LocalId,
        scrut_ty: &AdType,
        scrut_cls: Class,
        pat_binders: &[(LocalId, AdType, Class)],
        n_pat_active: usize,
        arm_inputs: &[(LocalId, AdType, Class)],
        active_outer: &[(LocalId, AdType, Class)],
        name_prefix: &str,
        next_local: &mut usize,
    ) -> Core {
        let ly = LocalId(*next_local);
        let lpb = LocalId(*next_local + 1);
        *next_local += 2;

        // Build wrapper pullback `adapt_pb(lpb, non_diff_pat_binders..., scrut_var, dy) -> d_active_outer`.
        let adapt_id = self.fresh_unit_id(name_prefix, "pb");
        let mut p_names = vec!["sub_pb".into()];
        let mut p_classes = vec![Class::Obj];
        let mut cap_vals = vec![Core::Var(lpb)];

        // Capture non-active pattern binders (e.g., `ShapeWitness`, `off`, `strides` of `MkNDView`)
        // so we can rebuild `d_scrut` cleanly!
        let mut non_active_slot: HashMap<LocalId, LocalId> = HashMap::new();
        for (bl, bt, bc) in pat_binders {
            if !bt.is_active() {
                let slot = LocalId(p_names.len());
                p_names.push(format!("meta_{}", bl.0));
                p_classes.push(*bc);
                cap_vals.push(Core::Var(*bl));
                non_active_slot.insert(*bl, slot);
            }
        }
        // Also capture `scrut_var` if active (for Wild/None/zero_like fallback).
        let scrut_slot = if scrut_ty.is_active() {
            let slot = LocalId(p_names.len());
            p_names.push("scrut".into());
            p_classes.push(scrut_cls);
            cap_vals.push(Core::Var(scrut_var));
            Some(slot)
        } else {
            None
        };

        let dy_param = LocalId(p_names.len());
        p_names.push("dy".into());
        p_classes.push(out_cls);

        let mut pb_local = p_names.len() + 1;
        let sub_call = Core::App(Box::new(Core::Var(LocalId(0))), vec![Core::Var(dy_param)]);

        // Unpack `sub_call` into individual cotangent locals for `arm_inputs`.
        let mut arm_cot_locals: HashMap<LocalId, LocalId> = HashMap::new();
        let mut unpack_binds: Vec<(LocalId, Class, Core)> = Vec::new();
        if arm_inputs.len() == 1 {
            let l = LocalId(pb_local);
            pb_local += 1;
            unpack_binds.push((l, arm_inputs[0].2, sub_call));
            arm_cot_locals.insert(arm_inputs[0].0, l);
        } else if arm_inputs.len() >= 2 {
            let ltup = LocalId(pb_local);
            pb_local += 1;
            unpack_binds.push((ltup, Class::Obj, sub_call));
            let tup_classes: Vec<Class> = arm_inputs.iter().map(|(_, _, c)| *c).collect();
            for (idx, (al, _, ac)) in arm_inputs.iter().enumerate() {
                let fl = LocalId(pb_local);
                pb_local += 1;
                let mut fields = Vec::with_capacity(arm_inputs.len());
                for _ in 0..arm_inputs.len() {
                    let tmp = LocalId(pb_local);
                    pb_local += 1;
                    fields.push(tmp);
                }
                let proj = Core::Case {
                    scrut: Box::new(Core::Var(ltup)),
                    alts: vec![CAlt {
                        pat: CPat::Con { tag: 0, fields: fields.clone(), classes: tup_classes.clone() },
                        weight: None,
                        body: Box::new(Core::Var(fields[idx])),
                    }],
                };
                unpack_binds.push((fl, *ac, proj));
                arm_cot_locals.insert(*al, fl);
            }
        }
        let _ = n_pat_active;

        // Reconstruct `d_scrut` if `scrut_var` is in `active_outer`.
        let d_scrut_expr = if let Some(ss) = scrut_slot {
            match (pat, scrut_ty) {
                (CPat::Con { tag: 0, .. }, AdType::Tuple(_)) => {
                    let elems: Vec<(Core, Class)> = pat_binders
                        .iter()
                        .map(|(bl, bt, bc)| {
                            let v = if bt.is_active() {
                                Core::Var(arm_cot_locals[bl])
                            } else {
                                Core::Var(non_active_slot[bl])
                            };
                            (v, *bc)
                        })
                        .collect();
                    Some(self.mk_tuple(elems))
                }
                (CPat::Con { .. }, AdType::Product { ctor, .. }) => {
                    let args: Vec<Core> = pat_binders
                        .iter()
                        .map(|(bl, bt, _)| {
                            if bt.is_active() {
                                Core::Var(arm_cot_locals[bl])
                            } else {
                                Core::Var(non_active_slot[bl])
                            }
                        })
                        .collect();
                    Some(Core::App(Box::new(Core::Ctor(ctor.clone(), Vec::new())), args))
                }
                (CPat::Con { tag: 1, .. }, AdType::Option(inner)) => {
                    let (_, some_id) = self.ensure_option_ctors(inner.class(Class::Obj));
                    let v = Core::Var(arm_cot_locals[&pat_binders[0].0]);
                    Some(Core::App(Box::new(Core::Ctor(some_id, Vec::new())), vec![v]))
                }
                (CPat::Con { tag: 0, .. }, AdType::Option(inner)) => {
                    let (none_id, _) = self.ensure_option_ctors(inner.class(Class::Obj));
                    Some(Core::Ctor(none_id, Vec::new()))
                }
                (CPat::Var(l, _), _) => Some(Core::Var(arm_cot_locals[l])),
                _ => Some(self.zero_like(Core::Var(ss), scrut_ty, &mut pb_local)),
            }
        } else {
            None
        };

        // Pack cotangents in the exact order of `active_outer`.
        let mut out_vals: Vec<(Core, Class)> = Vec::with_capacity(active_outer.len());
        for (ol, ot, oc) in active_outer {
            if *ol == scrut_var {
                if let Some(ds) = &d_scrut_expr {
                    out_vals.push((ds.clone(), *oc));
                }
            } else if let Some(&cl) = arm_cot_locals.get(ol) {
                out_vals.push((Core::Var(cl), *oc));
            } else {
                // Capture `ol` to build a zero_like cotangent (arm didn't touch this outer var).
                let slot = LocalId(p_names.len());
                p_names.push(format!("z_{}", ol.0));
                p_classes.push(*oc);
                cap_vals.push(Core::Var(*ol));
                // Notice: we must insert BEFORE `dy_param`, so simpler: capture `ol` only if needed
                // Actually `dy_param` must stay the LAST parameter of the closure! Let's fix order below!
                let _ = (slot, ot);
            }
        }
        // Cleanly rebuild parameter order when zero-like outer captures were needed:
        // Every active_outer var not in `arm_inputs` (and != scrut_var) is captured BEFORE `dy`.
        let missing_outer: Vec<(LocalId, AdType, Class)> = active_outer
            .iter()
            .filter(|(ol, _, _)| *ol != scrut_var && !arm_cot_locals.contains_key(ol))
            .cloned()
            .collect();
        if !missing_outer.is_empty() {
            // Re-index cleanly so `dy_param` is ALWAYS the final parameter!
            let base_len = p_names.len() - 1; // index of old `dy_param`
            let saved_dy_name = p_names.pop().unwrap();
            let saved_dy_cls = p_classes.pop().unwrap();
            let mut missing_slots: HashMap<LocalId, LocalId> = HashMap::new();
            for (ol, _, oc) in &missing_outer {
                let slot = LocalId(p_names.len());
                p_names.push(format!("z_{}", ol.0));
                p_classes.push(*oc);
                cap_vals.push(Core::Var(*ol));
                missing_slots.insert(*ol, slot);
            }
            let new_dy_param = LocalId(p_names.len());
            p_names.push(saved_dy_name);
            p_classes.push(saved_dy_cls);
            // Fix `dy_param` in `unpack_binds` and rebuild `out_vals`.
            let delta_start = new_dy_param.0 + 1;
            let shift = delta_start - (base_len + 2);
            let mut new_pb_local = pb_local + shift;
            let mut new_unpack = Vec::new();
            let mut remap_sub: HashMap<LocalId, LocalId> = HashMap::new();
            remap_sub.insert(dy_param, new_dy_param);
            for (l, c, rhs) in unpack_binds {
                let nl = LocalId(l.0 + shift);
                remap_sub.insert(l, nl);
                new_unpack.push((nl, c, remap_locals(&rhs, &remap_sub)));
            }
            unpack_binds = new_unpack;
            out_vals.clear();
            for (ol, ot, oc) in active_outer {
                if *ol == scrut_var {
                    let ds = remap_locals(d_scrut_expr.as_ref().unwrap(), &remap_sub);
                    out_vals.push((ds, *oc));
                } else if let Some(&cl) = arm_cot_locals.get(ol) {
                    out_vals.push((Core::Var(LocalId(cl.0 + shift)), *oc));
                } else {
                    let slot = missing_slots[ol];
                    let z = self.zero_like(Core::Var(slot), ot, &mut new_pb_local);
                    out_vals.push((z, *oc));
                }
            }
        }

        let (pb_ret, pb_ret_cls) = if out_vals.is_empty() {
            (Core::Unit, Class::Raw)
        } else if out_vals.len() == 1 {
            out_vals.pop().unwrap()
        } else {
            (self.mk_tuple(out_vals), Class::Obj)
        };

        let mut pb_body = pb_ret;
        for (l, c, rhs) in unpack_binds.into_iter().rev() {
            pb_body = Core::Let(l, c, Box::new(rhs), Box::new(pb_body));
        }

        self.add_fn(FnDef {
            id: adapt_id.clone(),
            params: p_names,
            body: pb_body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: UnitRepr {
                tparams: 0,
                params: p_classes,
                fields: Vec::new(),
                data: None,
                result: Some(pb_ret_cls),
                dictionary_values: Vec::new(),
                ..Default::default()
            },
        });

        let adapt_clo = Core::Lam { fn_id: adapt_id, env: cap_vals, classes: Vec::new() };
        let out_pair = self.mk_tuple(vec![(Core::Var(ly), out_cls), (adapt_clo, Class::Obj)]);
        Core::Case {
            scrut: Box::new(arm_vjp),
            alts: vec![CAlt {
                pat: CPat::Con {
                    tag: 0,
                    fields: vec![ly, lpb],
                    classes: vec![out_cls, Class::Obj],
                },
                weight: None,
                body: Box::new(out_pair),
            }],
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn vjp_emit_app(
        &mut self,
        out_var: LocalId,
        out_cls: Class,
        out_ty: &AdType,
        head: &Core,
        args: &[Core],
        fwd_binds: &mut Vec<(LocalId, Class, Core)>,
        rev_ops: &mut Vec<RevOp>,
        step_tys: &mut HashMap<LocalId, AdType>,
        step_classes: &mut HashMap<LocalId, Class>,
        name_prefix: &str,
        next_local: &mut usize,
    ) -> Result<()> {
        // 1. Scalar float operations
        if let Core::Global(g, _) = head {
            if let Some(op) = classify_scalar_float_op(&g.0) {
                fwd_binds.push((out_var, out_cls, Core::App(Box::new(head.clone()), args.to_vec())));
                if !matches!(op, ScalarFloatOp::Cmp(_)) {
                    rev_ops.push(RevOp::Scalar { out: out_var, op, args: args.to_vec() });
                }
                return Ok(());
            }
            if let Some(f) = self.fns.get(g).cloned() {
                if let Some(PrimOp::ArrayCompute { op, .. }) = f.prim {
                    return self.vjp_emit_array_compute(
                        out_var,
                        out_cls,
                        out_ty,
                        op,
                        args,
                        fwd_binds,
                        rev_ops,
                        step_tys,
                        step_classes,
                        next_local,
                    );
                }
                // User function call: call `g$vjp(args...)` -> `(out_var, pb_var)`!
                if f.prim.is_none() && f.ctor.is_none() && args.len() == f.params.len() {
                    let arg_tys: Vec<AdType> = args
                        .iter()
                        .map(|a| self.infer_single_expr_ty(a, step_tys))
                        .collect();
                    if arg_tys.iter().any(AdType::is_active) && out_ty.is_active() {
                        let (vjp_id, _) = self.gen_vjp_fn(g, &arg_tys, out_ty)?;
                        let pair_var = LocalId(*next_local);
                        let pb_var = LocalId(*next_local + 1);
                        *next_local += 2;
                        fwd_binds.push((
                            pair_var,
                            Class::Obj,
                            Self::call_global(vjp_id, args.to_vec()),
                        ));
                        fwd_binds.push((
                            out_var,
                            out_cls,
                            project_pair_field(
                                Core::Var(pair_var),
                                0,
                                out_cls,
                                Class::Obj,
                                next_local,
                            ),
                        ));
                        fwd_binds.push((
                            pb_var,
                            Class::Obj,
                            project_pair_field(
                                Core::Var(pair_var),
                                1,
                                out_cls,
                                Class::Obj,
                                next_local,
                            ),
                        ));
                        step_classes.insert(pb_var, Class::Obj);
                        let mut targets = Vec::new();
                        for (a, at) in args.iter().zip(arg_tys.iter()) {
                            if at.is_active() {
                                if let Core::Var(al) = a {
                                    let ac = step_classes
                                        .get(al)
                                        .copied()
                                        .unwrap_or_else(|| at.class(Class::Raw));
                                    targets.push((*al, at.clone(), ac));
                                } else {
                                    // Constant active literal argument (e.g. `1.0`): dummy sink
                                    let dummy = LocalId(*next_local);
                                    *next_local += 1;
                                    targets.push((dummy, at.clone(), at.class(Class::Raw)));
                                }
                            }
                        }
                        if !targets.is_empty() {
                            rev_ops.push(RevOp::CallPullback { out: out_var, pb_var, targets });
                        }
                        return Ok(());
                    }
                }
            }
        }

        // 2. Direct `PrimOp::ArrayCompute`
        if let Core::Prim(PrimOp::ArrayCompute { op, .. }) = head {
            return self.vjp_emit_array_compute(
                out_var,
                out_cls,
                out_ty,
                *op,
                args,
                fwd_binds,
                rev_ops,
                step_tys,
                step_classes,
                next_local,
            );
        }

        // 3. Constructor (`Tuple.N`, `Some`, `MkNDView`, `MkView1`, user records)
        if let Core::Ctor(c, _) = head {
            fwd_binds.push((out_var, out_cls, Core::App(Box::new(head.clone()), args.to_vec())));
            if out_ty.is_active() {
                let tag = self.ctor_info.get(c).map(|(t, _)| *t).unwrap_or(0);
                let fclasses = self
                    .ctor_info
                    .get(c)
                    .map(|(_, fc)| fc.clone())
                    .unwrap_or_else(|| vec![Class::Raw; args.len()]);
                let field_tys: Vec<AdType> = args
                    .iter()
                    .map(|a| self.infer_single_expr_ty(a, step_tys))
                    .collect();
                rev_ops.push(RevOp::Ctor {
                    out: out_var,
                    tag,
                    args: args.to_vec(),
                    field_tys,
                    field_classes: fclasses,
                });
            }
            return Ok(());
        }

        let _ = name_prefix;
        fwd_binds.push((out_var, out_cls, Core::App(Box::new(head.clone()), args.to_vec())));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn vjp_emit_array_compute(
        &mut self,
        out_var: LocalId,
        out_cls: Class,
        out_ty: &AdType,
        op: ArrayComputeOp,
        args: &[Core],
        fwd_binds: &mut Vec<(LocalId, Class, Core)>,
        rev_ops: &mut Vec<RevOp>,
        step_tys: &mut HashMap<LocalId, AdType>,
        step_classes: &mut HashMap<LocalId, Class>,
        next_local: &mut usize,
    ) -> Result<()> {
        let head = self.ensure_compute_op(op);
        fwd_binds.push((out_var, out_cls, Core::App(Box::new(head), args.to_vec())));
        if !out_ty.is_active() {
            return Ok(());
        }
        // For `MapF32(g, a)`, pre-build the elementwise VJP mapper `\x dy -> vjp_scalar(g, x, dy)`!
        if op == ArrayComputeOp::MapF32 {
            if let Some((g_id, g_cap)) = self.resolve_target_fn(&args[0]) {
                let mut g_arg_tys: Vec<AdType> = g_cap
                    .iter()
                    .map(|c| self.infer_single_expr_ty(c, step_tys))
                    .collect();
                g_arg_tys.push(AdType::F32);
                let (g_vjp_id, _) = self.gen_vjp_fn(&g_id, &g_arg_tys, &AdType::F32)?;
                let helper_id = self.fresh_unit_id(&g_id.0, "map_vjp");
                let mut h_params = Vec::new();
                let mut h_pclasses = Vec::new();
                let mut h_call_args = Vec::new();
                for (i, c_expr) in g_cap.iter().enumerate() {
                    let cls = match c_expr {
                        Core::Var(l) => step_classes.get(l).copied().unwrap_or(Class::Raw),
                        _ => Class::Raw,
                    };
                    h_params.push(format!("c{i}"));
                    h_pclasses.push(cls);
                    h_call_args.push(Core::Var(LocalId(i)));
                }
                let base = g_cap.len();
                let lx = LocalId(base);
                let ldy = LocalId(base + 1);
                h_params.push("x".into());
                h_params.push("dy".into());
                h_pclasses.push(Class::Raw);
                h_pclasses.push(Class::Raw);
                h_call_args.push(Core::Var(lx));
                let ly = LocalId(base + 2);
                let lpb = LocalId(base + 3);
                let n_active_cap = g_arg_tys[..base].iter().filter(|t| t.is_active()).count();
                let pb_applied = Core::App(Box::new(Core::Var(lpb)), vec![Core::Var(ldy)]);
                let dx_extracted = if n_active_cap == 0 {
                    pb_applied
                } else {
                    // Last element of the cotangent tuple is `dx`.
                    let total_active = n_active_cap + 1;
                    let mut fs = Vec::new();
                    let mut fcs = Vec::new();
                    for k in 0..total_active {
                        fs.push(LocalId(base + 4 + k));
                        fcs.push(Class::Raw);
                    }
                    Core::Case {
                        scrut: Box::new(pb_applied),
                        alts: vec![CAlt {
                            pat: CPat::Con { tag: 0, fields: fs.clone(), classes: fcs },
                            weight: None,
                            body: Box::new(Core::Var(*fs.last().unwrap())),
                        }],
                    }
                };
                let h_body = Core::Case {
                    scrut: Box::new(Self::call_global(g_vjp_id, h_call_args)),
                    alts: vec![CAlt {
                        pat: CPat::Con {
                            tag: 0,
                            fields: vec![ly, lpb],
                            classes: vec![Class::Raw, Class::Obj],
                        },
                        weight: None,
                        body: Box::new(dx_extracted),
                    }],
                };
                self.add_fn(FnDef {
                    id: helper_id.clone(),
                    params: h_params,
                    body: h_body,
                    prim: None,
                    ctor: None,
                    hints: Default::default(),
                    meta: Default::default(),
                    repr: UnitRepr {
                        tparams: 0,
                        params: h_pclasses,
                        fields: Vec::new(),
                        data: None,
                        result: Some(Class::Raw),
                        dictionary_values: Vec::new(),
                        ..Default::default()
                    },
                });
                let mapper = if g_cap.is_empty() {
                    Core::Global(helper_id, Vec::new())
                } else {
                    Core::Lam { fn_id: helper_id, env: g_cap, classes: Vec::new() }
                };
                let lm = LocalId(*next_local);
                *next_local += 1;
                fwd_binds.push((lm, Class::Obj, mapper));
                step_classes.insert(lm, Class::Obj);
                rev_ops.push(RevOp::ArrayMapF32 {
                    out: out_var,
                    arr: args[1].clone(),
                    vjp_mapper: lm,
                });
                return Ok(());
            }
        }
        rev_ops.push(RevOp::ArrayCompute { out: out_var, op, args: args.to_vec() });
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn build_pullback_fn(
        &mut self,
        y_var: LocalId,
        ret_ty: &AdType,
        ret_cls: Class,
        active_inputs: &[(LocalId, AdType, Class)],
        rev_ops: &[RevOp],
        step_tys: &HashMap<LocalId, AdType>,
        step_classes: &HashMap<LocalId, Class>,
        name_prefix: &str,
    ) -> (GlobalId, Vec<Core>, Class) {
        let pb_id = self.fresh_unit_id(name_prefix, "pb");

        // Determine which forward locals must be captured into the pullback closure's env.
        let mut needed: Vec<LocalId> = Vec::new();
        let mut add_need = |l: LocalId| {
            if !needed.contains(&l) {
                needed.push(l);
            }
        };
        for (l, _, _) in active_inputs {
            add_need(*l);
        }
        for op in rev_ops {
            op.collect_captured_locals(&mut add_need);
        }

        // Pullback parameter layout: `[captured_0, ..., captured_{k-1}, dy]`.
        let mut p_names: Vec<String> = Vec::with_capacity(needed.len() + 1);
        let mut p_classes: Vec<Class> = Vec::with_capacity(needed.len() + 1);
        let mut cap_env: Vec<Core> = Vec::with_capacity(needed.len());
        let mut cap_map: HashMap<LocalId, LocalId> = HashMap::new();
        for (i, &orig_l) in needed.iter().enumerate() {
            let cls = step_classes.get(&orig_l).copied().unwrap_or(Class::Raw);
            p_names.push(format!("cap{i}"));
            p_classes.push(cls);
            cap_env.push(Core::Var(orig_l));
            cap_map.insert(orig_l, LocalId(i));
        }
        let dy_param = LocalId(needed.len());
        p_names.push("dy".into());
        p_classes.push(ret_cls);

        let mut next_pb_local = needed.len() + 1;
        let mut adj: HashMap<LocalId, LocalId> = HashMap::new();
        let mut pb_binds: Vec<(LocalId, Class, Core)> = Vec::new();

        // Seed output adjoint `adj[y_var] = dy_param`.
        if ret_ty.is_active() {
            adj.insert(y_var, dy_param);
        }

        // Helper closure to accumulate a cotangent contribution `contrib` onto `target_var`.
        let accum = |this: &mut Self,
                     target: LocalId,
                     contrib: Core,
                     ty: &AdType,
                     adj: &mut HashMap<LocalId, LocalId>,
                     pb_binds: &mut Vec<(LocalId, Class, Core)>,
                     next_l: &mut usize| {
            if !ty.is_active() {
                return;
            }
            let cls = ty.class(Class::Raw);
            if let Some(&prev) = adj.get(&target) {
                let l_new = LocalId(*next_l);
                *next_l += 1;
                let l_sum = LocalId(*next_l);
                *next_l += 1;
                let summed = this.add_cotangents(Core::Var(prev), Core::Var(l_new), ty, next_l);
                pb_binds.push((l_new, cls, contrib));
                pb_binds.push((l_sum, cls, summed));
                adj.insert(target, l_sum);
            } else {
                let l_new = LocalId(*next_l);
                *next_l += 1;
                pb_binds.push((l_new, cls, contrib));
                adj.insert(target, l_new);
            }
        };

        let map_arg = |e: &Core, cap_map: &HashMap<LocalId, LocalId>| -> Core {
            remap_locals(e, cap_map)
        };

        // Reverse sweep: process `rev_ops` in strict reverse topological order!
        for op in rev_ops.iter().rev() {
            if let RevOp::UnpackCtor { scrut, scrut_ty, fields } = op {
                // Reconstruct `d_scrut` from any accumulated field cotangents and accumulate onto `adj[scrut]`!
                if fields.iter().any(|(fl, ft, _)| ft.is_active() && adj.contains_key(fl)) {
                    let mut elems: Vec<(Core, Class)> = Vec::with_capacity(fields.len());
                    for (fl, ft, fc) in fields {
                        let v = if ft.is_active() {
                            if let Some(&al) = adj.get(fl) {
                                Core::Var(al)
                            } else {
                                self.zero_like(Core::Var(cap_map[fl]), ft, &mut next_pb_local)
                            }
                        } else {
                            Core::Var(cap_map[fl])
                        };
                        elems.push((v, *fc));
                    }
                    let rebuilt = match scrut_ty {
                        AdType::Tuple(_) => self.mk_tuple(elems),
                        AdType::Product { ctor, .. } => {
                            let cargs = elems.into_iter().map(|(v, _)| v).collect();
                            Core::App(Box::new(Core::Ctor(ctor.clone(), Vec::new())), cargs)
                        }
                        _ => self.mk_tuple(elems),
                    };
                    accum(self, *scrut, rebuilt, scrut_ty, &mut adj, &mut pb_binds, &mut next_pb_local);
                }
                continue;
            }
            let out_id = op.out_var();
            let Some(&dz_local) = adj.get(&out_id) else {
                continue;
            };
            let dz = Core::Var(dz_local);

            match op {
                RevOp::UnpackCtor { .. } => unreachable!(),
                RevOp::Copy { src, ty, .. } => {
                    accum(self, *src, dz, ty, &mut adj, &mut pb_binds, &mut next_pb_local);
                }
                RevOp::Scalar { op: sop, args, .. } => {
                    self.emit_scalar_rev(
                        *sop,
                        out_id,
                        dz,
                        args,
                        &cap_map,
                        &mut adj,
                        &mut pb_binds,
                        &mut next_pb_local,
                    );
                }
                RevOp::ArrayCompute { op: aop, args, .. } => {
                    self.emit_array_rev(
                        *aop,
                        dz,
                        args,
                        &cap_map,
                        &mut adj,
                        &mut pb_binds,
                        &mut next_pb_local,
                    );
                }
                RevOp::ArrayMapF32 { arr, vjp_mapper, .. } => {
                    if let Core::Var(al) = arr {
                        let zip_with = self.ensure_compute_op(ArrayComputeOp::ZipWithF32);
                        let mapper_var = Core::Var(cap_map[vjp_mapper]);
                        let a_var = Core::Var(cap_map[al]);
                        let d_opt = Core::App(Box::new(zip_with), vec![mapper_var, a_var.clone(), dz]);
                        let da = self.unwrap_option_obj(d_opt, a_var, &mut next_pb_local);
                        accum(
                            self,
                            *al,
                            da,
                            &AdType::ArrayF32,
                            &mut adj,
                            &mut pb_binds,
                            &mut next_pb_local,
                        );
                    }
                }
                RevOp::Ctor { tag, args, field_tys, field_classes, .. } => {
                    // Destructure `dz` by `tag`, accumulate each active field onto its source var!
                    for (idx, (a, ft)) in args.iter().zip(field_tys.iter()).enumerate() {
                        if !ft.is_active() {
                            continue;
                        }
                        if let Core::Var(al) = a {
                            let mut dz_fields = Vec::with_capacity(args.len());
                            for _ in args {
                                let l = LocalId(next_pb_local);
                                next_pb_local += 1;
                                dz_fields.push(l);
                            }
                            let proj = Core::Case {
                                scrut: Box::new(dz.clone()),
                                alts: vec![
                                    CAlt {
                                        pat: CPat::Con {
                                            tag: *tag,
                                            fields: dz_fields.clone(),
                                            classes: field_classes.clone(),
                                        },
                                        weight: None,
                                        body: Box::new(Core::Var(dz_fields[idx])),
                                    },
                                    CAlt {
                                        pat: CPat::Wild,
                                        weight: None,
                                        body: Box::new(self.zero_like(
                                            map_arg(a, &cap_map),
                                            ft,
                                            &mut next_pb_local,
                                        )),
                                    },
                                ],
                            };
                            accum(self, *al, proj, ft, &mut adj, &mut pb_binds, &mut next_pb_local);
                        }
                    }
                }
                RevOp::CallPullback { pb_var, targets, .. } => {
                    let pb_slot = cap_map[pb_var];
                    let res_expr = Core::App(Box::new(Core::Var(pb_slot)), vec![dz]);
                    if targets.len() == 1 {
                        let (tl, tt, _) = &targets[0];
                        accum(self, *tl, res_expr, tt, &mut adj, &mut pb_binds, &mut next_pb_local);
                    } else if targets.len() >= 2 {
                        let ltup = LocalId(next_pb_local);
                        next_pb_local += 1;
                        pb_binds.push((ltup, Class::Obj, res_expr));
                        let tclasses: Vec<Class> = targets.iter().map(|(_, _, c)| *c).collect();
                        for (idx, (tl, tt, _)) in targets.iter().enumerate() {
                            let mut fs = Vec::with_capacity(targets.len());
                            for _ in 0..targets.len() {
                                let fl = LocalId(next_pb_local);
                                next_pb_local += 1;
                                fs.push(fl);
                            }
                            let proj = Core::Case {
                                scrut: Box::new(Core::Var(ltup)),
                                alts: vec![CAlt {
                                    pat: CPat::Con {
                                        tag: 0,
                                        fields: fs.clone(),
                                        classes: tclasses.clone(),
                                    },
                                    weight: None,
                                    body: Box::new(Core::Var(fs[idx])),
                                }],
                            };
                            accum(self, *tl, proj, tt, &mut adj, &mut pb_binds, &mut next_pb_local);
                        }
                    }
                }
            }
        }

        // Gather the final cotangents of `active_inputs` (defaulting untouched inputs to `zero_like`).
        let mut ret_elems: Vec<(Core, Class)> = Vec::with_capacity(active_inputs.len());
        for (il, it, ic) in active_inputs {
            let val = if let Some(&al) = adj.get(il) {
                Core::Var(al)
            } else {
                let slot = cap_map[il];
                self.zero_like(Core::Var(slot), it, &mut next_pb_local)
            };
            ret_elems.push((val, *ic));
        }

        let (pb_ret, pb_ret_cls) = if ret_elems.is_empty() {
            (Core::Unit, Class::Raw)
        } else if ret_elems.len() == 1 {
            ret_elems.pop().unwrap()
        } else {
            (self.mk_tuple(ret_elems), Class::Obj)
        };

        let mut pb_body = pb_ret;
        for (l, c, rhs) in pb_binds.into_iter().rev() {
            pb_body = Core::Let(l, c, Box::new(rhs), Box::new(pb_body));
        }

        let _ = step_tys;
        self.add_fn(FnDef {
            id: pb_id.clone(),
            params: p_names,
            body: pb_body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: UnitRepr {
                tparams: 0,
                params: p_classes,
                fields: Vec::new(),
                data: None,
                result: Some(pb_ret_cls),
                dictionary_values: Vec::new(),
                ..Default::default()
            },
        });

        (pb_id, cap_env, pb_ret_cls)
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_scalar_rev(
        &mut self,
        op: ScalarFloatOp,
        out_id: LocalId,
        dz: Core,
        args: &[Core],
        cap_map: &HashMap<LocalId, LocalId>,
        adj: &mut HashMap<LocalId, LocalId>,
        pb_binds: &mut Vec<(LocalId, Class, Core)>,
        next_l: &mut usize,
    ) {
        let mut add_to = |arg: &Core, contrib: Core, p: FloatPrec| {
            if let Core::Var(al) = arg {
                let ty = p.ad_ty();
                if let Some(&prev) = adj.get(al) {
                    let ln = LocalId(*next_l);
                    let ls = LocalId(*next_l + 1);
                    *next_l += 2;
                    let sum = Self::call_global(p.add_id(), vec![Core::Var(prev), Core::Var(ln)]);
                    pb_binds.push((ln, Class::Raw, contrib));
                    pb_binds.push((ls, Class::Raw, sum));
                    adj.insert(*al, ls);
                } else {
                    let ln = LocalId(*next_l);
                    *next_l += 1;
                    pb_binds.push((ln, Class::Raw, contrib));
                    adj.insert(*al, ln);
                }
                let _ = ty;
            }
        };
        let a0 = || remap_locals(&args[0], cap_map);
        let a1 = || remap_locals(&args[1], cap_map);
        match op {
            ScalarFloatOp::Add(p) => {
                add_to(&args[0], dz.clone(), p);
                add_to(&args[1], dz, p);
            }
            ScalarFloatOp::Sub(p) => {
                add_to(&args[0], dz.clone(), p);
                add_to(&args[1], Self::call_global(p.neg_id(), vec![dz]), p);
            }
            ScalarFloatOp::Mul(p) => {
                add_to(&args[0], Self::call_global(p.mul_id(), vec![dz.clone(), a1()]), p);
                add_to(&args[1], Self::call_global(p.mul_id(), vec![dz, a0()]), p);
            }
            ScalarFloatOp::Div(p) => {
                let y = a1();
                add_to(&args[0], Self::call_global(p.div_id(), vec![dz.clone(), y.clone()]), p);
                let num = Self::call_global(p.neg_id(), vec![Self::call_global(p.mul_id(), vec![dz, a0()])]);
                let den = Self::call_global(p.mul_id(), vec![y.clone(), y]);
                add_to(&args[1], Self::call_global(p.div_id(), vec![num, den]), p);
            }
            ScalarFloatOp::Neg(p) => {
                add_to(&args[0], Self::call_global(p.neg_id(), vec![dz]), p);
            }
            ScalarFloatOp::Abs(p) => {
                let cond = Self::call_global(p.fge_id(), vec![a0(), p.lit(0.0)]);
                let c = Core::If(
                    Box::new(cond),
                    CBranch { weight: None, body: Box::new(dz.clone()) },
                    CBranch { weight: None, body: Box::new(Self::call_global(p.neg_id(), vec![dz])) },
                );
                add_to(&args[0], c, p);
            }
            ScalarFloatOp::Sqrt(p) => {
                let out_val = Core::Var(cap_map[&out_id]);
                let den = Self::call_global(p.mul_id(), vec![p.lit(2.0), out_val]);
                add_to(&args[0], Self::call_global(p.div_id(), vec![dz, den]), p);
            }
            ScalarFloatOp::Sin(p) => {
                let cos_x = Self::call_global(p.cos_id(), vec![a0()]);
                add_to(&args[0], Self::call_global(p.mul_id(), vec![dz, cos_x]), p);
            }
            ScalarFloatOp::Cos(p) => {
                let neg_sin = Self::call_global(p.neg_id(), vec![Self::call_global(p.sin_id(), vec![a0()])]);
                add_to(&args[0], Self::call_global(p.mul_id(), vec![dz, neg_sin]), p);
            }
            ScalarFloatOp::Exp(p) => {
                let out_val = Core::Var(cap_map[&out_id]);
                add_to(&args[0], Self::call_global(p.mul_id(), vec![dz, out_val]), p);
            }
            ScalarFloatOp::Log(p) => {
                add_to(&args[0], Self::call_global(p.div_id(), vec![dz, a0()]), p);
            }
            ScalarFloatOp::Cmp(_) => {}
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_array_rev(
        &mut self,
        op: ArrayComputeOp,
        dz: Core,
        args: &[Core],
        cap_map: &HashMap<LocalId, LocalId>,
        adj: &mut HashMap<LocalId, LocalId>,
        pb_binds: &mut Vec<(LocalId, Class, Core)>,
        next_l: &mut usize,
    ) {
        let accum_arg = |this: &mut Self,
                         arg: &Core,
                         contrib: Core,
                         ty: &AdType,
                         adj: &mut HashMap<LocalId, LocalId>,
                         pb_binds: &mut Vec<(LocalId, Class, Core)>,
                         next_l: &mut usize| {
            if let Core::Var(al) = arg {
                let cls = ty.class(Class::Raw);
                if let Some(&prev) = adj.get(al) {
                    let ln = LocalId(*next_l);
                    let ls = LocalId(*next_l + 1);
                    *next_l += 2;
                    let sum = this.add_cotangents(Core::Var(prev), Core::Var(ln), ty, next_l);
                    pb_binds.push((ln, cls, contrib));
                    pb_binds.push((ls, cls, sum));
                    adj.insert(*al, ls);
                } else {
                    let ln = LocalId(*next_l);
                    *next_l += 1;
                    pb_binds.push((ln, cls, contrib));
                    adj.insert(*al, ln);
                }
            }
        };
        let a = |i: usize| remap_locals(&args[i], cap_map);

        match op {
            ArrayComputeOp::DotF32 => {
                let mu = self.ensure_compute_op(ArrayComputeOp::MapUnaryF32);
                let da = Core::App(Box::new(mu.clone()), vec![a(1), Core::Int(2), dz.clone()]);
                let db = Core::App(Box::new(mu), vec![a(0), Core::Int(2), dz]);
                accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                accum_arg(self, &args[1], db, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::DotF64 => {
                let sc = self.ensure_compute_op(ArrayComputeOp::ScaleF64);
                let da = Core::App(Box::new(sc.clone()), vec![a(1), dz.clone()]);
                let db = Core::App(Box::new(sc), vec![a(0), dz]);
                accum_arg(self, &args[0], da, &AdType::ArrayF64, adj, pb_binds, next_l);
                accum_arg(self, &args[1], db, &AdType::ArrayF64, adj, pb_binds, next_l);
            }
            ArrayComputeOp::ReduceF32 => {
                let fill = self.ensure_compute_op(ArrayComputeOp::FillF32);
                let da = Core::App(Box::new(fill), vec![self.array_len_call(a(0)), dz]);
                accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::ReduceF64 => {
                let fill = self.ensure_compute_op(ArrayComputeOp::FillF64);
                let da = Core::App(Box::new(fill), vec![self.array_len_call(a(0)), dz]);
                accum_arg(self, &args[0], da, &AdType::ArrayF64, adj, pb_binds, next_l);
            }
            ArrayComputeOp::ZipBinopF32 => {
                // `dz` has type `Option (Array F32)`: unwrap `Some(dz_arr)`!
                let fb = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let dz_arr = self.unwrap_option_obj(dz, fb, next_l);
                let ldz = LocalId(*next_l);
                *next_l += 1;
                pb_binds.push((ldz, Class::Obj, dz_arr));
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF32);
                let mu = self.ensure_compute_op(ArrayComputeOp::MapUnaryF32);
                match args[2] {
                    Core::Int(0) => {
                        accum_arg(self, &args[0], Core::Var(ldz), &AdType::ArrayF32, adj, pb_binds, next_l);
                        accum_arg(self, &args[1], Core::Var(ldz), &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    Core::Int(1) => {
                        accum_arg(self, &args[0], Core::Var(ldz), &AdType::ArrayF32, adj, pb_binds, next_l);
                        let neg = Core::App(Box::new(mu), vec![Core::Var(ldz), Core::Int(5), FloatPrec::F32.lit(0.0)]);
                        accum_arg(self, &args[1], neg, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    Core::Int(2) => {
                        let da_opt = Core::App(Box::new(zip.clone()), vec![Core::Var(ldz), a(1), Core::Int(2)]);
                        let db_opt = Core::App(Box::new(zip), vec![Core::Var(ldz), a(0), Core::Int(2)]);
                        let da = self.unwrap_option_obj(da_opt, Core::Var(ldz), next_l);
                        let db = self.unwrap_option_obj(db_opt, Core::Var(ldz), next_l);
                        accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                        accum_arg(self, &args[1], db, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    Core::Int(3) => {
                        let da_opt = Core::App(Box::new(zip.clone()), vec![Core::Var(ldz), a(1), Core::Int(3)]);
                        let da = self.unwrap_option_obj(da_opt, Core::Var(ldz), next_l);
                        let num_opt = Core::App(Box::new(zip.clone()), vec![Core::Var(ldz), a(0), Core::Int(2)]);
                        let bb_opt = Core::App(Box::new(zip.clone()), vec![a(1), a(1), Core::Int(2)]);
                        let num = self.unwrap_option_obj(num_opt, Core::Var(ldz), next_l);
                        let bb = self.unwrap_option_obj(bb_opt, Core::Var(ldz), next_l);
                        let q_opt = Core::App(Box::new(zip), vec![num, bb, Core::Int(3)]);
                        let q = self.unwrap_option_obj(q_opt, Core::Var(ldz), next_l);
                        let db = Core::App(Box::new(mu), vec![q, Core::Int(5), FloatPrec::F32.lit(0.0)]);
                        accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                        accum_arg(self, &args[1], db, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    _ => {}
                }
            }
            ArrayComputeOp::ZipBinopF64 => {
                let fb = self.zero_like(a(0), &AdType::ArrayF64, next_l);
                let dz_arr = self.unwrap_option_obj(dz, fb, next_l);
                let ldz = LocalId(*next_l);
                *next_l += 1;
                pb_binds.push((ldz, Class::Obj, dz_arr));
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF64);
                let sc = self.ensure_compute_op(ArrayComputeOp::ScaleF64);
                match args[2] {
                    Core::Int(0) => {
                        accum_arg(self, &args[0], Core::Var(ldz), &AdType::ArrayF64, adj, pb_binds, next_l);
                        accum_arg(self, &args[1], Core::Var(ldz), &AdType::ArrayF64, adj, pb_binds, next_l);
                    }
                    Core::Int(1) => {
                        accum_arg(self, &args[0], Core::Var(ldz), &AdType::ArrayF64, adj, pb_binds, next_l);
                        let neg = Core::App(Box::new(sc), vec![Core::Var(ldz), FloatPrec::F64.lit(-1.0)]);
                        accum_arg(self, &args[1], neg, &AdType::ArrayF64, adj, pb_binds, next_l);
                    }
                    Core::Int(2) => {
                        let da_opt = Core::App(Box::new(zip.clone()), vec![Core::Var(ldz), a(1), Core::Int(2)]);
                        let db_opt = Core::App(Box::new(zip), vec![Core::Var(ldz), a(0), Core::Int(2)]);
                        let da = self.unwrap_option_obj(da_opt, Core::Var(ldz), next_l);
                        let db = self.unwrap_option_obj(db_opt, Core::Var(ldz), next_l);
                        accum_arg(self, &args[0], da, &AdType::ArrayF64, adj, pb_binds, next_l);
                        accum_arg(self, &args[1], db, &AdType::ArrayF64, adj, pb_binds, next_l);
                    }
                    _ => {}
                }
            }
            ArrayComputeOp::MapUnaryF32 => {
                let mu = self.ensure_compute_op(ArrayComputeOp::MapUnaryF32);
                let zip = self.ensure_compute_op(ArrayComputeOp::ZipBinopF32);
                let dot = self.ensure_compute_op(ArrayComputeOp::DotF32);
                let red = self.ensure_compute_op(ArrayComputeOp::ReduceF32);
                match args[1] {
                    Core::Int(0) => {
                        accum_arg(self, &args[0], dz, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    Core::Int(1) => {
                        accum_arg(self, &args[0], dz.clone(), &AdType::ArrayF32, adj, pb_binds, next_l);
                        let dc = Core::App(Box::new(red), vec![dz, Core::Int(10)]);
                        accum_arg(self, &args[2], dc, &AdType::F32, adj, pb_binds, next_l);
                    }
                    Core::Int(2) => {
                        let da = Core::App(Box::new(mu), vec![dz.clone(), Core::Int(2), a(2)]);
                        let dc = Core::App(Box::new(dot), vec![a(0), dz, Core::Int(10)]);
                        accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                        accum_arg(self, &args[2], dc, &AdType::F32, adj, pb_binds, next_l);
                    }
                    Core::Int(3) => {
                        let two_a = Core::App(Box::new(mu), vec![a(0), Core::Int(2), FloatPrec::F32.lit(2.0)]);
                        let da_opt = Core::App(Box::new(zip), vec![two_a, dz.clone(), Core::Int(2)]);
                        let da = self.unwrap_option_obj(da_opt, dz, next_l);
                        accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    Core::Int(5) => {
                        let da = Core::App(Box::new(mu), vec![dz, Core::Int(5), FloatPrec::F32.lit(0.0)]);
                        accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
                    }
                    _ => {}
                }
            }
            ArrayComputeOp::ScaleF64 => {
                let sc = self.ensure_compute_op(ArrayComputeOp::ScaleF64);
                let dot = self.ensure_compute_op(ArrayComputeOp::DotF64);
                let da = Core::App(Box::new(sc), vec![dz.clone(), a(1)]);
                let dc = Core::App(Box::new(dot), vec![a(0), dz, Core::Int(10)]);
                accum_arg(self, &args[0], da, &AdType::ArrayF64, adj, pb_binds, next_l);
                accum_arg(self, &args[1], dc, &AdType::F64, adj, pb_binds, next_l);
            }
            ArrayComputeOp::FillF32 => {
                let red = self.ensure_compute_op(ArrayComputeOp::ReduceF32);
                let dv = Core::App(Box::new(red), vec![dz, Core::Int(10)]);
                accum_arg(self, &args[1], dv, &AdType::F32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::FillF64 => {
                let red = self.ensure_compute_op(ArrayComputeOp::ReduceF64);
                let dv = Core::App(Box::new(red), vec![dz, Core::Int(10)]);
                accum_arg(self, &args[1], dv, &AdType::F64, adj, pb_binds, next_l);
            }
            ArrayComputeOp::RowReduceF32 => {
                let fb = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let dz_arr = self.unwrap_option_obj(dz, fb, next_l);
                let bcast = self.ensure_compute_op(ArrayComputeOp::RowBroadcastF32);
                let da = Core::App(Box::new(bcast), vec![dz_arr, a(1), a(2)]);
                accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::RowBroadcastF32 => {
                let rr = self.ensure_compute_op(ArrayComputeOp::RowReduceF32);
                let dv_opt = Core::App(Box::new(rr), vec![dz, a(1), a(2), Core::Int(10)]);
                let fb = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let dv = self.unwrap_option_obj(dv_opt, fb, next_l);
                accum_arg(self, &args[0], dv, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::GatherF32 => {
                let scat = self.ensure_compute_op(ArrayComputeOp::ScatterAddF32);
                let dsrc = Core::App(Box::new(scat), vec![self.array_len_call(a(0)), a(1), dz, a(2)]);
                accum_arg(self, &args[0], dsrc, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::ScatterAddF32 => {
                let gath = self.ensure_compute_op(ArrayComputeOp::GatherF32);
                let dvals = Core::App(Box::new(gath), vec![dz, a(1), a(3)]);
                accum_arg(self, &args[2], dvals, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::Stencil2DF32 => {
                let fb1 = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let dz_arr = self.unwrap_option_obj(dz, fb1, next_l);
                let adj_st = self.ensure_compute_op(ArrayComputeOp::Stencil2DAdjointF32);
                let da_opt = Core::App(
                    Box::new(adj_st),
                    vec![dz_arr, a(1), a(2), a(3), a(4), a(5), a(6)],
                );
                let fb2 = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let da = self.unwrap_option_obj(da_opt, fb2, next_l);
                accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            ArrayComputeOp::Stencil2DAdjointF32 => {
                let fb1 = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let dz_arr = self.unwrap_option_obj(dz, fb1, next_l);
                let fwd_st = self.ensure_compute_op(ArrayComputeOp::Stencil2DF32);
                let da_opt = Core::App(
                    Box::new(fwd_st),
                    vec![dz_arr, a(1), a(2), a(3), a(4), a(5), a(6)],
                );
                let fb2 = self.zero_like(a(0), &AdType::ArrayF32, next_l);
                let da = self.unwrap_option_obj(da_opt, fb2, next_l);
                accum_arg(self, &args[0], da, &AdType::ArrayF32, adj, pb_binds, next_l);
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Supporting Data Structures & Core IR Utilities
// ---------------------------------------------------------------------------

struct JvpEnv {
    primal_map: HashMap<LocalId, LocalId>,
    tangent_map: HashMap<LocalId, LocalId>,
    local_tys: HashMap<LocalId, AdType>,
    local_classes: HashMap<LocalId, Class>,
    join_sigs: HashMap<JoinId, Vec<AdType>>,
    closure_targets: HashMap<LocalId, (GlobalId, Vec<Core>, Vec<AdType>)>,
}

struct AnfStep {
    var: LocalId,
    cls: Class,
    ty: AdType,
    rhs: Core,
}

enum RevOp {
    Copy {
        out: LocalId,
        src: LocalId,
        ty: AdType,
    },
    Scalar {
        out: LocalId,
        op: ScalarFloatOp,
        args: Vec<Core>,
    },
    ArrayCompute {
        out: LocalId,
        op: ArrayComputeOp,
        args: Vec<Core>,
    },
    ArrayMapF32 {
        out: LocalId,
        arr: Core,
        vjp_mapper: LocalId,
    },
    Ctor {
        out: LocalId,
        tag: usize,
        args: Vec<Core>,
        field_tys: Vec<AdType>,
        field_classes: Vec<Class>,
    },
    UnpackCtor {
        scrut: LocalId,
        scrut_ty: AdType,
        fields: Vec<(LocalId, AdType, Class)>,
    },
    CallPullback {
        out: LocalId,
        pb_var: LocalId,
        targets: Vec<(LocalId, AdType, Class)>,
    },
}

impl RevOp {
    fn out_var(&self) -> LocalId {
        match self {
            RevOp::Copy { out, .. }
            | RevOp::Scalar { out, .. }
            | RevOp::ArrayCompute { out, .. }
            | RevOp::ArrayMapF32 { out, .. }
            | RevOp::Ctor { out, .. }
            | RevOp::CallPullback { out, .. } => *out,
            RevOp::UnpackCtor { scrut, .. } => *scrut,
        }
    }

    fn collect_captured_locals(&self, add: &mut impl FnMut(LocalId)) {
        match self {
            RevOp::Copy { .. } => {}
            RevOp::UnpackCtor { fields, .. } => {
                for (fl, _, _) in fields {
                    add(*fl);
                }
            }
            RevOp::Scalar { out, op, args } => {
                if matches!(op, ScalarFloatOp::Sqrt(_) | ScalarFloatOp::Exp(_)) {
                    add(*out);
                }
                for a in args {
                    collect_all_vars(a, add);
                }
            }
            RevOp::ArrayCompute { args, .. } | RevOp::Ctor { args, .. } => {
                for a in args {
                    collect_all_vars(a, add);
                }
            }
            RevOp::ArrayMapF32 { arr, vjp_mapper, .. } => {
                collect_all_vars(arr, add);
                add(*vjp_mapper);
            }
            RevOp::CallPullback { pb_var, .. } => {
                add(*pb_var);
            }
        }
    }
}

fn array_compute_param_ad_types(op: ArrayComputeOp) -> Vec<AdType> {
    use ArrayComputeOp::*;
    match op {
        DotF32 => vec![AdType::ArrayF32, AdType::ArrayF32, AdType::NonDiff],
        DotF64 => vec![AdType::ArrayF64, AdType::ArrayF64, AdType::NonDiff],
        ReduceF32 => vec![AdType::ArrayF32, AdType::NonDiff],
        ReduceF64 => vec![AdType::ArrayF64, AdType::NonDiff],
        ZipBinopF32 => vec![AdType::ArrayF32, AdType::ArrayF32, AdType::NonDiff],
        ZipBinopF64 => vec![AdType::ArrayF64, AdType::ArrayF64, AdType::NonDiff],
        MapUnaryF32 => vec![AdType::ArrayF32, AdType::NonDiff, AdType::F32],
        ScaleF64 => vec![AdType::ArrayF64, AdType::F64],
        FillF32 => vec![AdType::NonDiff, AdType::F32],
        FillF64 => vec![AdType::NonDiff, AdType::F64],
        RowReduceF32 => vec![AdType::ArrayF32, AdType::NonDiff, AdType::NonDiff, AdType::NonDiff],
        RowBroadcastF32 => vec![AdType::ArrayF32, AdType::NonDiff, AdType::NonDiff],
        GatherF32 => vec![AdType::ArrayF32, AdType::NonDiff, AdType::NonDiff],
        ScatterAddF32 => vec![AdType::NonDiff, AdType::NonDiff, AdType::ArrayF32, AdType::NonDiff],
        Stencil2DF32 | Stencil2DAdjointF32 => vec![
            AdType::ArrayF32,
            AdType::NonDiff,
            AdType::NonDiff,
            AdType::NonDiff,
            AdType::NonDiff,
            AdType::NonDiff,
            AdType::NonDiff,
        ],
        MapF32 => vec![AdType::NonDiff, AdType::ArrayF32],
        MapF64 => vec![AdType::NonDiff, AdType::ArrayF64],
        ZipWithF32 => vec![AdType::NonDiff, AdType::ArrayF32, AdType::ArrayF32],
        _ => Vec::new(),
    }
}

fn array_compute_ret_ad_type(op: ArrayComputeOp) -> AdType {
    use ArrayComputeOp::*;
    match op {
        DotF32 | ReduceF32 | StridedDotF32 | FusedMapReduceF32 | FoldlF32 => AdType::F32,
        DotF64 | ReduceF64 => AdType::F64,
        MapUnaryF32 | FillF32 | RowBroadcastF32 | GatherF32 | ScatterAddF32 | MapF32
        | TabulateF32 | SegmentedReduceF32 | SpmvCsrF32 => AdType::ArrayF32,
        ScaleF64 | FillF64 | MapF64 | TabulateF64 => AdType::ArrayF64,
        ZipBinopF32 | RowReduceF32 | Stencil2DF32 | Stencil2DAdjointF32 | ZipWithF32 => {
            AdType::Option(Box::new(AdType::ArrayF32))
        }
        ZipBinopF64 => AdType::Option(Box::new(AdType::ArrayF64)),
        _ => AdType::NonDiff,
    }
}

fn project_pair_field(
    pair: Core,
    idx: usize,
    c0: Class,
    c1: Class,
    next_local: &mut usize,
) -> Core {
    let l0 = LocalId(*next_local);
    let l1 = LocalId(*next_local + 1);
    *next_local += 2;
    Core::Case {
        scrut: Box::new(pair),
        alts: vec![CAlt {
            pat: CPat::Con { tag: 0, fields: vec![l0, l1], classes: vec![c0, c1] },
            weight: None,
            body: Box::new(Core::Var(if idx == 0 { l0 } else { l1 })),
        }],
    }
}

fn restore_map<K: Eq + std::hash::Hash, V>(m: &mut HashMap<K, V>, k: K, prev: Option<V>) {
    if let Some(v) = prev {
        m.insert(k, v);
    } else {
        m.remove(&k);
    }
}

fn max_local_in_fn(f: &FnDef) -> usize {
    let mut m = f.params.len();
    max_local_in_expr(&f.body, &mut m);
    m
}

fn max_local_in_expr(e: &Core, out: &mut usize) {
    match e {
        Core::Var(l) => *out = (*out).max(l.0),
        Core::Let(l, _, r, b) => {
            *out = (*out).max(l.0);
            max_local_in_expr(r, out);
            max_local_in_expr(b, out);
        }
        Core::App(h, args) => {
            max_local_in_expr(h, out);
            for a in args {
                max_local_in_expr(a, out);
            }
        }
        Core::If(c, t, f) => {
            max_local_in_expr(c, out);
            max_local_in_expr(&t.body, out);
            max_local_in_expr(&f.body, out);
        }
        Core::Case { scrut, alts } => {
            max_local_in_expr(scrut, out);
            for a in alts {
                match &a.pat {
                    CPat::Con { fields, .. } => {
                        for l in fields {
                            *out = (*out).max(l.0);
                        }
                    }
                    CPat::Var(l, _) => *out = (*out).max(l.0),
                    CPat::Wild => {}
                }
                max_local_in_expr(&a.body, out);
            }
        }
        Core::Join { params, body, rest, .. } => {
            for (l, _) in params {
                *out = (*out).max(l.0);
            }
            max_local_in_expr(body, out);
            max_local_in_expr(rest, out);
        }
        Core::Jump { args, .. } | Core::InstDict(_, args, _) => {
            for a in args {
                max_local_in_expr(a, out);
            }
        }
        Core::Lam { env, .. } => {
            for a in env {
                max_local_in_expr(a, out);
            }
        }
        Core::Proj(x, _)
        | Core::DictProj(x, _, _, _)
        | Core::Demand(x, _)
        | Core::BoxInit(_, x, _)
        | Core::BoxGet(x, _)
        | Core::BoxClear(x, _)
        | Core::RefSet(_, x, _)
        | Core::LazyInit(_, x) => max_local_in_expr(x, out),
        Core::Dup(l, _, b) | Core::Drop(l, _, b) => {
            *out = (*out).max(l.0);
            max_local_in_expr(b, out);
        }
        Core::Release { block, moved, rest, body, .. } => {
            *out = (*out).max(block.0);
            for (l, _) in moved.iter().chain(rest.iter()) {
                *out = (*out).max(l.0);
            }
            max_local_in_expr(body, out);
        }
        _ => {}
    }
}

fn max_join_in_expr(e: &Core) -> usize {
    let mut m = 0usize;
    fn walk(e: &Core, m: &mut usize) {
        match e {
            Core::Join { id, body, rest, .. } => {
                *m = (*m).max(id.0);
                walk(body, m);
                walk(rest, m);
            }
            Core::Jump { id, args } => {
                *m = (*m).max(id.0);
                for a in args {
                    walk(a, m);
                }
            }
            Core::Let(_, _, r, b) => {
                walk(r, m);
                walk(b, m);
            }
            Core::App(h, args) => {
                walk(h, m);
                for a in args {
                    walk(a, m);
                }
            }
            Core::If(c, t, f) => {
                walk(c, m);
                walk(&t.body, m);
                walk(&f.body, m);
            }
            Core::Case { scrut, alts } => {
                walk(scrut, m);
                for a in alts {
                    walk(&a.body, m);
                }
            }
            _ => {}
        }
    }
    walk(e, &mut m);
    m
}

fn collect_let_classes(e: &Core, out: &mut HashMap<LocalId, Class>) {
    match e {
        Core::Let(x, c, r, b) => {
            out.insert(*x, *c);
            collect_let_classes(r, out);
            collect_let_classes(b, out);
        }
        Core::App(h, args) => {
            collect_let_classes(h, out);
            for a in args {
                collect_let_classes(a, out);
            }
        }
        Core::If(c, t, f) => {
            collect_let_classes(c, out);
            collect_let_classes(&t.body, out);
            collect_let_classes(&f.body, out);
        }
        Core::Case { scrut, alts } => {
            collect_let_classes(scrut, out);
            for a in alts {
                match &a.pat {
                    CPat::Con { fields, classes, .. } => {
                        for (&l, &c) in fields.iter().zip(classes.iter()) {
                            out.insert(l, c);
                        }
                    }
                    CPat::Var(l, c) => {
                        out.insert(*l, *c);
                    }
                    CPat::Wild => {}
                }
                collect_let_classes(&a.body, out);
            }
        }
        Core::Join { params, body, rest, .. } => {
            for &(l, c) in params {
                out.insert(l, c);
            }
            collect_let_classes(body, out);
            collect_let_classes(rest, out);
        }
        _ => {}
    }
}

fn collect_callees(e: &Core, out: &mut Vec<GlobalId>) {
    match e {
        Core::Global(g, _) => out.push(g.clone()),
        Core::Lam { fn_id, env, .. } => {
            out.push(fn_id.clone());
            for a in env {
                collect_callees(a, out);
            }
        }
        Core::App(h, args) => {
            collect_callees(h, out);
            for a in args {
                collect_callees(a, out);
            }
        }
        Core::Let(_, _, r, b) => {
            collect_callees(r, out);
            collect_callees(b, out);
        }
        Core::If(c, t, f) => {
            collect_callees(c, out);
            collect_callees(&t.body, out);
            collect_callees(&f.body, out);
        }
        Core::Case { scrut, alts } => {
            collect_callees(scrut, out);
            for a in alts {
                collect_callees(&a.body, out);
            }
        }
        Core::Join { body, rest, .. } => {
            collect_callees(body, out);
            collect_callees(rest, out);
        }
        Core::Jump { args, .. } | Core::InstDict(_, args, _) => {
            for a in args {
                collect_callees(a, out);
            }
        }
        _ => {}
    }
}

fn collect_all_vars(e: &Core, add: &mut impl FnMut(LocalId)) {
    match e {
        Core::Var(l) => add(*l),
        Core::App(h, args) => {
            collect_all_vars(h, add);
            for a in args {
                collect_all_vars(a, add);
            }
        }
        Core::Lam { env, .. } => {
            for a in env {
                collect_all_vars(a, add);
            }
        }
        _ => {}
    }
}

fn collect_free_locals(e: &Core, bound: &HashSet<LocalId>, out: &mut Vec<LocalId>) {
    match e {
        Core::Var(l) => {
            if !bound.contains(l) && !out.contains(l) {
                out.push(*l);
            }
        }
        Core::Let(x, _, r, b) => {
            collect_free_locals(r, bound, out);
            let mut b2 = bound.clone();
            b2.insert(*x);
            collect_free_locals(b, &b2, out);
        }
        Core::App(h, args) => {
            collect_free_locals(h, bound, out);
            for a in args {
                collect_free_locals(a, bound, out);
            }
        }
        Core::If(c, t, f) => {
            collect_free_locals(c, bound, out);
            collect_free_locals(&t.body, bound, out);
            collect_free_locals(&f.body, bound, out);
        }
        Core::Case { scrut, alts } => {
            collect_free_locals(scrut, bound, out);
            for a in alts {
                let mut b2 = bound.clone();
                match &a.pat {
                    CPat::Con { fields, .. } => {
                        for &l in fields {
                            b2.insert(l);
                        }
                    }
                    CPat::Var(l, _) => {
                        b2.insert(*l);
                    }
                    CPat::Wild => {}
                }
                collect_free_locals(&a.body, &b2, out);
            }
        }
        Core::Join { params, body, rest, .. } => {
            let mut b2 = bound.clone();
            for &(l, _) in params {
                b2.insert(l);
            }
            collect_free_locals(body, &b2, out);
            collect_free_locals(rest, bound, out);
        }
        Core::Jump { args, .. } | Core::InstDict(_, args, _) => {
            for a in args {
                collect_free_locals(a, bound, out);
            }
        }
        Core::Lam { env, .. } => {
            for a in env {
                collect_free_locals(a, bound, out);
            }
        }
        Core::Proj(x, _)
        | Core::DictProj(x, _, _, _)
        | Core::Demand(x, _)
        | Core::BoxInit(_, x, _)
        | Core::BoxGet(x, _)
        | Core::BoxClear(x, _)
        | Core::RefSet(_, x, _)
        | Core::LazyInit(_, x) => collect_free_locals(x, bound, out),
        _ => {}
    }
}

fn remap_locals(e: &Core, map: &HashMap<LocalId, LocalId>) -> Core {
    match e {
        Core::Var(l) => Core::Var(map.get(l).copied().unwrap_or(*l)),
        Core::Let(x, c, r, b) => {
            let r2 = remap_locals(r, map);
            let mut m2 = map.clone();
            m2.remove(x);
            let b2 = remap_locals(b, &m2);
            Core::Let(*x, *c, Box::new(r2), Box::new(b2))
        }
        Core::App(h, args) => Core::App(
            Box::new(remap_locals(h, map)),
            args.iter().map(|a| remap_locals(a, map)).collect(),
        ),
        Core::If(c, t, f) => Core::If(
            Box::new(remap_locals(c, map)),
            CBranch { weight: t.weight, body: Box::new(remap_locals(&t.body, map)) },
            CBranch { weight: f.weight, body: Box::new(remap_locals(&f.body, map)) },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(remap_locals(scrut, map)),
            alts: alts
                .iter()
                .map(|a| {
                    let mut m2 = map.clone();
                    match &a.pat {
                        CPat::Con { fields, .. } => {
                            for l in fields {
                                m2.remove(l);
                            }
                        }
                        CPat::Var(l, _) => {
                            m2.remove(l);
                        }
                        CPat::Wild => {}
                    }
                    CAlt {
                        pat: a.pat.clone(),
                        weight: a.weight,
                        body: Box::new(remap_locals(&a.body, &m2)),
                    }
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => {
            let mut m2 = map.clone();
            for (l, _) in params {
                m2.remove(l);
            }
            Core::Join {
                id: *id,
                params: params.clone(),
                body: Box::new(remap_locals(body, &m2)),
                rest: Box::new(remap_locals(rest, map)),
            }
        }
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args.iter().map(|a| remap_locals(a, map)).collect(),
        },
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(|a| remap_locals(a, map)).collect(),
            classes: classes.clone(),
        },
        Core::InstDict(g, args, cls) => Core::InstDict(
            g.clone(),
            args.iter().map(|a| remap_locals(a, map)).collect(),
            cls.clone(),
        ),
        Core::Proj(x, f) => Core::Proj(Box::new(remap_locals(x, map)), f.clone()),
        Core::DictProj(x, i, s, c) => {
            Core::DictProj(Box::new(remap_locals(x, map)), *i, s.clone(), *c)
        }
        other => other.clone(),
    }
}

fn subst_local_in_expr(e: &Core, target: LocalId, by: &Core) -> Core {
    match e {
        Core::Var(l) if *l == target => by.clone(),
        Core::Let(x, c, r, b) => {
            let r2 = subst_local_in_expr(r, target, by);
            let b2 = if *x == target { (**b).clone() } else { subst_local_in_expr(b, target, by) };
            Core::Let(*x, *c, Box::new(r2), Box::new(b2))
        }
        Core::App(h, args) => Core::App(
            Box::new(subst_local_in_expr(h, target, by)),
            args.iter().map(|a| subst_local_in_expr(a, target, by)).collect(),
        ),
        Core::If(c, t, f) => Core::If(
            Box::new(subst_local_in_expr(c, target, by)),
            CBranch { weight: t.weight, body: Box::new(subst_local_in_expr(&t.body, target, by)) },
            CBranch { weight: f.weight, body: Box::new(subst_local_in_expr(&f.body, target, by)) },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(subst_local_in_expr(scrut, target, by)),
            alts: alts
                .iter()
                .map(|a| {
                    let shadowed = match &a.pat {
                        CPat::Con { fields, .. } => fields.contains(&target),
                        CPat::Var(l, _) => *l == target,
                        CPat::Wild => false,
                    };
                    CAlt {
                        pat: a.pat.clone(),
                        weight: a.weight,
                        body: if shadowed {
                            a.body.clone()
                        } else {
                            Box::new(subst_local_in_expr(&a.body, target, by))
                        },
                    }
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => {
            let shadowed = params.iter().any(|(l, _)| *l == target);
            Core::Join {
                id: *id,
                params: params.clone(),
                body: Box::new(if shadowed {
                    (**body).clone()
                } else {
                    subst_local_in_expr(body, target, by)
                }),
                rest: Box::new(subst_local_in_expr(rest, target, by)),
            }
        }
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args.iter().map(|a| subst_local_in_expr(a, target, by)).collect(),
        },
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(|a| subst_local_in_expr(a, target, by)).collect(),
            classes: classes.clone(),
        },
        other => other.clone(),
    }
}

fn freshen_expr(e: &Core, next_local: &mut usize, next_join: &mut usize) -> Core {
    let mut lmap = HashMap::new();
    let mut jmap = HashMap::new();
    fn go(
        e: &Core,
        lmap: &mut HashMap<LocalId, LocalId>,
        jmap: &mut HashMap<JoinId, JoinId>,
        nl: &mut usize,
        nj: &mut usize,
    ) -> Core {
        match e {
            Core::Var(l) => Core::Var(lmap.get(l).copied().unwrap_or(*l)),
            Core::Let(x, c, r, b) => {
                let r2 = go(r, lmap, jmap, nl, nj);
                let nx = LocalId(*nl);
                *nl += 1;
                let prev = lmap.insert(*x, nx);
                let b2 = go(b, lmap, jmap, nl, nj);
                restore_map(lmap, *x, prev);
                Core::Let(nx, *c, Box::new(r2), Box::new(b2))
            }
            Core::App(h, args) => Core::App(
                Box::new(go(h, lmap, jmap, nl, nj)),
                args.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
            ),
            Core::If(c, t, f) => Core::If(
                Box::new(go(c, lmap, jmap, nl, nj)),
                CBranch { weight: t.weight, body: Box::new(go(&t.body, lmap, jmap, nl, nj)) },
                CBranch { weight: f.weight, body: Box::new(go(&f.body, lmap, jmap, nl, nj)) },
            ),
            Core::Case { scrut, alts } => Core::Case {
                scrut: Box::new(go(scrut, lmap, jmap, nl, nj)),
                alts: alts
                    .iter()
                    .map(|a| match &a.pat {
                        CPat::Con { tag, fields, classes } => {
                            let mut nf = Vec::with_capacity(fields.len());
                            let mut saved = Vec::new();
                            for &fl in fields {
                                let nfl = LocalId(*nl);
                                *nl += 1;
                                nf.push(nfl);
                                saved.push((fl, lmap.insert(fl, nfl)));
                            }
                            let b2 = go(&a.body, lmap, jmap, nl, nj);
                            for (fl, p) in saved {
                                restore_map(lmap, fl, p);
                            }
                            CAlt {
                                pat: CPat::Con { tag: *tag, fields: nf, classes: classes.clone() },
                                weight: a.weight,
                                body: Box::new(b2),
                            }
                        }
                        CPat::Var(l, c) => {
                            let nfl = LocalId(*nl);
                            *nl += 1;
                            let prev = lmap.insert(*l, nfl);
                            let b2 = go(&a.body, lmap, jmap, nl, nj);
                            restore_map(lmap, *l, prev);
                            CAlt { pat: CPat::Var(nfl, *c), weight: a.weight, body: Box::new(b2) }
                        }
                        CPat::Wild => CAlt {
                            pat: CPat::Wild,
                            weight: a.weight,
                            body: Box::new(go(&a.body, lmap, jmap, nl, nj)),
                        },
                    })
                    .collect(),
            },
            Core::Join { id, params, body, rest } => {
                let nid = JoinId(*nj);
                *nj += 1;
                let prev_j = jmap.insert(*id, nid);
                let mut np = Vec::with_capacity(params.len());
                let mut saved = Vec::new();
                for &(pl, pc) in params {
                    let npl = LocalId(*nl);
                    *nl += 1;
                    np.push((npl, pc));
                    saved.push((pl, lmap.insert(pl, npl)));
                }
                let b2 = go(body, lmap, jmap, nl, nj);
                for (pl, p) in saved {
                    restore_map(lmap, pl, p);
                }
                let r2 = go(rest, lmap, jmap, nl, nj);
                restore_map(jmap, *id, prev_j);
                Core::Join { id: nid, params: np, body: Box::new(b2), rest: Box::new(r2) }
            }
            Core::Jump { id, args } => Core::Jump {
                id: jmap.get(id).copied().unwrap_or(*id),
                args: args.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
            },
            Core::Lam { fn_id, env, classes } => Core::Lam {
                fn_id: fn_id.clone(),
                env: env.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
                classes: classes.clone(),
            },
            other => other.clone(),
        }
    }
    go(e, &mut lmap, &mut jmap, next_local, next_join)
}

fn replace_jump_with_call(
    e: &Core,
    target_join: JoinId,
    fn_id: &GlobalId,
    cap_args: &[Core],
) -> Core {
    match e {
        Core::Jump { id, args } if *id == target_join => {
            let mut all = cap_args.to_vec();
            all.extend(args.iter().cloned());
            Core::App(Box::new(Core::Global(fn_id.clone(), Vec::new())), all)
        }
        Core::Let(x, c, r, b) => Core::Let(
            *x,
            *c,
            Box::new(replace_jump_with_call(r, target_join, fn_id, cap_args)),
            Box::new(replace_jump_with_call(b, target_join, fn_id, cap_args)),
        ),
        Core::If(c, t, f) => Core::If(
            c.clone(),
            CBranch {
                weight: t.weight,
                body: Box::new(replace_jump_with_call(&t.body, target_join, fn_id, cap_args)),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(replace_jump_with_call(&f.body, target_join, fn_id, cap_args)),
            },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: scrut.clone(),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(replace_jump_with_call(&a.body, target_join, fn_id, cap_args)),
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(replace_jump_with_call(body, target_join, fn_id, cap_args)),
            rest: Box::new(replace_jump_with_call(rest, target_join, fn_id, cap_args)),
        },
        other => other.clone(),
    }
}

enum JvpBind {
    Let(LocalId, Class, Core),
    UnpackPair { p: LocalId, t: LocalId, cls: Class, pair: Core },
}

fn wrap_jvp_binds(binds: Vec<JvpBind>, mut body: Core) -> Core {
    for b in binds.into_iter().rev() {
        body = match b {
            JvpBind::Let(x, c, rhs) => Core::Let(x, c, Box::new(rhs), Box::new(body)),
            JvpBind::UnpackPair { p, t, cls, pair } => Core::Case {
                scrut: Box::new(pair),
                alts: vec![CAlt {
                    pat: CPat::Con { tag: 0, fields: vec![p, t], classes: vec![cls, cls] },
                    weight: None,
                    body: Box::new(body),
                }],
            },
        };
    }
    body
}

/// Freshen all non-parameter locals (`LocalId(i)` with `i >= n_params`) so new parameters
/// appended after `n_params` cannot collide with body binders.
fn freshen_non_params(
    e: &Core,
    n_params: usize,
    next_local: &mut usize,
    next_join: &mut usize,
) -> Core {
    let mut lmap = HashMap::new();
    for i in 0..n_params {
        lmap.insert(LocalId(i), LocalId(i));
    }
    freshen_with_seed(e, &lmap, next_local, next_join)
}

fn freshen_with_seed(
    e: &Core,
    seed: &HashMap<LocalId, LocalId>,
    next_local: &mut usize,
    next_join: &mut usize,
) -> Core {
    let mut lmap = seed.clone();
    let mut jmap = HashMap::new();
    fn go(
        e: &Core,
        lmap: &mut HashMap<LocalId, LocalId>,
        jmap: &mut HashMap<JoinId, JoinId>,
        nl: &mut usize,
        nj: &mut usize,
    ) -> Core {
        match e {
            Core::Var(l) => Core::Var(lmap.get(l).copied().unwrap_or(*l)),
            Core::Let(x, c, r, b) => {
                let r2 = go(r, lmap, jmap, nl, nj);
                let nx = LocalId(*nl);
                *nl += 1;
                let prev = lmap.insert(*x, nx);
                let b2 = go(b, lmap, jmap, nl, nj);
                restore_map(lmap, *x, prev);
                Core::Let(nx, *c, Box::new(r2), Box::new(b2))
            }
            Core::App(h, args) => Core::App(
                Box::new(go(h, lmap, jmap, nl, nj)),
                args.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
            ),
            Core::If(c, t, f) => Core::If(
                Box::new(go(c, lmap, jmap, nl, nj)),
                CBranch { weight: t.weight, body: Box::new(go(&t.body, lmap, jmap, nl, nj)) },
                CBranch { weight: f.weight, body: Box::new(go(&f.body, lmap, jmap, nl, nj)) },
            ),
            Core::Case { scrut, alts } => Core::Case {
                scrut: Box::new(go(scrut, lmap, jmap, nl, nj)),
                alts: alts
                    .iter()
                    .map(|a| match &a.pat {
                        CPat::Con { tag, fields, classes } => {
                            let mut nf = Vec::with_capacity(fields.len());
                            let mut saved = Vec::new();
                            for &fl in fields {
                                let nfl = LocalId(*nl);
                                *nl += 1;
                                nf.push(nfl);
                                saved.push((fl, lmap.insert(fl, nfl)));
                            }
                            let b2 = go(&a.body, lmap, jmap, nl, nj);
                            for (fl, p) in saved {
                                restore_map(lmap, fl, p);
                            }
                            CAlt {
                                pat: CPat::Con { tag: *tag, fields: nf, classes: classes.clone() },
                                weight: a.weight,
                                body: Box::new(b2),
                            }
                        }
                        CPat::Var(l, c) => {
                            let nfl = LocalId(*nl);
                            *nl += 1;
                            let prev = lmap.insert(*l, nfl);
                            let b2 = go(&a.body, lmap, jmap, nl, nj);
                            restore_map(lmap, *l, prev);
                            CAlt { pat: CPat::Var(nfl, *c), weight: a.weight, body: Box::new(b2) }
                        }
                        CPat::Wild => CAlt {
                            pat: CPat::Wild,
                            weight: a.weight,
                            body: Box::new(go(&a.body, lmap, jmap, nl, nj)),
                        },
                    })
                    .collect(),
            },
            Core::Join { id, params, body, rest } => {
                let nid = JoinId(*nj);
                *nj += 1;
                let prev_j = jmap.insert(*id, nid);
                let mut np = Vec::with_capacity(params.len());
                let mut saved = Vec::new();
                for &(pl, pc) in params {
                    let npl = LocalId(*nl);
                    *nl += 1;
                    np.push((npl, pc));
                    saved.push((pl, lmap.insert(pl, npl)));
                }
                let b2 = go(body, lmap, jmap, nl, nj);
                for (pl, p) in saved {
                    restore_map(lmap, pl, p);
                }
                let r2 = go(rest, lmap, jmap, nl, nj);
                restore_map(jmap, *id, prev_j);
                Core::Join { id: nid, params: np, body: Box::new(b2), rest: Box::new(r2) }
            }
            Core::Jump { id, args } => Core::Jump {
                id: jmap.get(id).copied().unwrap_or(*id),
                args: args.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
            },
            Core::Lam { fn_id, env, classes } => Core::Lam {
                fn_id: fn_id.clone(),
                env: env.iter().map(|a| go(a, lmap, jmap, nl, nj)).collect(),
                classes: classes.clone(),
            },
            other => other.clone(),
        }
    }
    go(e, &mut lmap, &mut jmap, next_local, next_join)
}

/// Lightweight beta-reduction for known `Case(Ctor(..))` and `App(Lam(..))` patterns,
/// collapsing redundant intermediate tuple packs/unpacks and known pullback closures.
fn beta_reduce_program(program: &mut Program) -> bool {
    let mut changed = false;
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let mut known_lams: HashMap<LocalId, (GlobalId, Vec<Core>)> = HashMap::new();
        let mut local_changed = false;
        let new_body = beta_reduce_expr(&f.body, &mut known_lams, &mut local_changed);
        if local_changed {
            f.body = new_body;
            changed = true;
        }
    }
    changed
}

fn beta_reduce_expr(
    e: &Core,
    known_lams: &mut HashMap<LocalId, (GlobalId, Vec<Core>)>,
    changed: &mut bool,
) -> Core {
    match e {
        Core::Let(x, c, rhs, body) => {
            let r2 = beta_reduce_expr(rhs, known_lams, changed);
            let prev = if let Core::Lam { fn_id, env, .. } = &r2 {
                if env.iter().all(|a| matches!(a, Core::Var(_) | Core::Int(_) | Core::Bool(_) | Core::Unit)) {
                    known_lams.insert(*x, (fn_id.clone(), env.clone()))
                } else {
                    None
                }
            } else {
                None
            };
            let b2 = beta_reduce_expr(body, known_lams, changed);
            restore_map(known_lams, *x, prev);
            Core::Let(*x, *c, Box::new(r2), Box::new(b2))
        }
        Core::App(h, args) => {
            let h2 = beta_reduce_expr(h, known_lams, changed);
            let args2: Vec<Core> = args
                .iter()
                .map(|a| beta_reduce_expr(a, known_lams, changed))
                .collect();
            match &h2 {
                Core::Lam { fn_id, env, .. } => {
                    *changed = true;
                    let mut full = env.clone();
                    full.extend(args2);
                    Core::App(Box::new(Core::Global(fn_id.clone(), Vec::new())), full)
                }
                Core::Var(l) => {
                    if let Some((fn_id, env)) = known_lams.get(l) {
                        *changed = true;
                        let mut full = env.clone();
                        full.extend(args2);
                        return Core::App(Box::new(Core::Global(fn_id.clone(), Vec::new())), full);
                    }
                    Core::App(Box::new(h2), args2)
                }
                _ => Core::App(Box::new(h2), args2),
            }
        }
        Core::Case { scrut, alts } => {
            let s2 = beta_reduce_expr(scrut, known_lams, changed);
            // Case-of-known-tuple: `case Tuple(a, b) of (x, y) -> body` => `let x = a; let y = b; body`.
            if alts.len() == 1 {
                if let (
                    Core::App(ch, cargs),
                    CPat::Con { tag: 0, fields, classes },
                ) = (&s2, &alts[0].pat)
                {
                    if let Core::Ctor(cid, _) = &**ch {
                        if cid.0.starts_with("Tuple.") && cargs.len() == fields.len() {
                            *changed = true;
                            let mut res = beta_reduce_expr(&alts[0].body, known_lams, changed);
                            for ((&fl, &fc), arg) in
                                fields.iter().zip(classes.iter()).zip(cargs.iter()).rev()
                            {
                                res = Core::Let(fl, fc, Box::new(arg.clone()), Box::new(res));
                            }
                            return res;
                        }
                    }
                }
            }
            let new_alts = alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(beta_reduce_expr(&a.body, known_lams, changed)),
                })
                .collect();
            Core::Case { scrut: Box::new(s2), alts: new_alts }
        }
        Core::If(c, t, f) => Core::If(
            Box::new(beta_reduce_expr(c, known_lams, changed)),
            CBranch { weight: t.weight, body: Box::new(beta_reduce_expr(&t.body, known_lams, changed)) },
            CBranch { weight: f.weight, body: Box::new(beta_reduce_expr(&f.body, known_lams, changed)) },
        ),
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(beta_reduce_expr(body, known_lams, changed)),
            rest: Box::new(beta_reduce_expr(rest, known_lams, changed)),
        },
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args.iter().map(|a| beta_reduce_expr(a, known_lams, changed)).collect(),
        },
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(|a| beta_reduce_expr(a, known_lams, changed)).collect(),
            classes: classes.clone(),
        },
        other => other.clone(),
    }
}

/// Specialize lambda-lifted closures whose body contains an AD call and whose captured
/// environment binds a statically known function (e.g. `hvp-f64`'s `\z -> vjp f z 1.0`
/// capturing `f = Global(rosenbrock)`). Clones the closure function with the known
/// function argument(s) substituted directly into its body so inner AD calls see their
/// concrete target!
fn specialize_ad_lams(program: &mut Program) -> bool {
    let ad_stubs: HashSet<GlobalId> = program
        .fns
        .iter()
        .filter(|f| matches!(f.prim, Some(PrimOp::Ad { .. })))
        .map(|f| f.id.clone())
        .collect();
    let fn_map: HashMap<GlobalId, FnDef> = program
        .fns
        .iter()
        .map(|f| (f.id.clone(), f.clone()))
        .collect();
    let mut ad_fns: HashSet<GlobalId> = HashSet::new();
    for f in &program.fns {
        if f.prim.is_none() && f.ctor.is_none() && expr_contains_ad(&f.body, &ad_stubs) {
            ad_fns.insert(f.id.clone());
        }
    }
    if ad_fns.is_empty() {
        return false;
    }

    let mut new_fns: Vec<FnDef> = Vec::new();
    let mut counter = program.fns.len();
    let mut changed = false;

    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let mut let_bindings: HashMap<LocalId, Core> = HashMap::new();
        let mut local_changed = false;
        let new_body = specialize_ad_lams_in_expr(
            &f.body,
            &fn_map,
            &ad_fns,
            &mut let_bindings,
            &mut new_fns,
            &mut counter,
            &mut local_changed,
        );
        if local_changed {
            f.body = new_body;
            changed = true;
        }
    }

    program.fns.extend(new_fns);
    changed
}

fn resolve_known_fn_expr(e: &Core, bindings: &HashMap<LocalId, Core>) -> Option<Core> {
    let mut cur = e.clone();
    for _ in 0..16 {
        match &cur {
            Core::Var(l) => {
                if let Some(b) = bindings.get(l) {
                    cur = b.clone();
                    continue;
                }
                return None;
            }
            Core::Global(..) => return Some(cur),
            Core::Lam { env, .. } if env.is_empty() => return Some(cur),
            _ => return None,
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn specialize_ad_lams_in_expr(
    e: &Core,
    fn_map: &HashMap<GlobalId, FnDef>,
    ad_fns: &HashSet<GlobalId>,
    let_bindings: &mut HashMap<LocalId, Core>,
    new_fns: &mut Vec<FnDef>,
    counter: &mut usize,
    changed: &mut bool,
) -> Core {
    match e {
        Core::Lam { fn_id, env, classes } if ad_fns.contains(fn_id) && !env.is_empty() => {
            if let Some(target_def) = fn_map.get(fn_id) {
                let mut substs: Vec<(usize, Core)> = Vec::new();
                let mut rem_env: Vec<Core> = Vec::new();
                for (i, a) in env.iter().enumerate() {
                    if let Some(known) = resolve_known_fn_expr(a, let_bindings) {
                        substs.push((i, known));
                    } else {
                        rem_env.push(a.clone());
                    }
                }
                if !substs.is_empty() {
                    let subst_indices: HashSet<usize> = substs.iter().map(|(i, _)| *i).collect();
                    let mut new_params = Vec::new();
                    let mut new_pclasses = Vec::new();
                    let mut local_remap: HashMap<LocalId, LocalId> = HashMap::new();
                    let mut next_p = 0usize;
                    for (i, p) in target_def.params.iter().enumerate() {
                        if !subst_indices.contains(&i) {
                            new_params.push(p.clone());
                            new_pclasses.push(
                                target_def.repr.params.get(i).copied().unwrap_or(Class::Raw),
                            );
                            local_remap.insert(LocalId(i), LocalId(next_p));
                            next_p += 1;
                        }
                    }
                    let mut next_l = max_local_in_fn(target_def) + next_p + 2;
                    let mut next_j = max_join_in_expr(&target_def.body) + 1;
                    // First substitute known function params, then remap remaining params!
                    let mut spec_body = target_def.body.clone();
                    for (i, known) in &substs {
                        spec_body = subst_local_in_expr(&spec_body, LocalId(*i), known);
                    }
                    spec_body = freshen_with_seed(&spec_body, &local_remap, &mut next_l, &mut next_j);

                    let spec_id = GlobalId(format!("{}$adsp{}", fn_id.0, *counter));
                    *counter += 1;
                    let mut spec_def = target_def.clone();
                    spec_def.id = spec_id.clone();
                    spec_def.params = new_params;
                    spec_def.repr.params = new_pclasses;
                    spec_def.body = spec_body;
                    new_fns.push(spec_def);
                    *changed = true;
                    return if rem_env.is_empty() {
                        Core::Global(spec_id, Vec::new())
                    } else {
                        Core::Lam { fn_id: spec_id, env: rem_env, classes: classes.clone() }
                    };
                }
            }
            e.clone()
        }
        Core::Let(x, c, rhs, body) => {
            let r2 = specialize_ad_lams_in_expr(
                rhs, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            );
            let prev = let_bindings.insert(*x, r2.clone());
            let b2 = specialize_ad_lams_in_expr(
                body, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            );
            restore_map(let_bindings, *x, prev);
            Core::Let(*x, *c, Box::new(r2), Box::new(b2))
        }
        Core::App(h, args) => Core::App(
            Box::new(specialize_ad_lams_in_expr(
                h, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            )),
            args.iter()
                .map(|a| {
                    specialize_ad_lams_in_expr(
                        a, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
                    )
                })
                .collect(),
        ),
        Core::If(c, t, f) => Core::If(
            Box::new(specialize_ad_lams_in_expr(
                c, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            )),
            CBranch {
                weight: t.weight,
                body: Box::new(specialize_ad_lams_in_expr(
                    &t.body, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
                )),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(specialize_ad_lams_in_expr(
                    &f.body, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
                )),
            },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(specialize_ad_lams_in_expr(
                scrut, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            )),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(specialize_ad_lams_in_expr(
                        &a.body, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
                    )),
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(specialize_ad_lams_in_expr(
                body, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            )),
            rest: Box::new(specialize_ad_lams_in_expr(
                rest, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
            )),
        },
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args
                .iter()
                .map(|a| {
                    specialize_ad_lams_in_expr(
                        a, fn_map, ad_fns, let_bindings, new_fns, counter, changed,
                    )
                })
                .collect(),
        },
        other => other.clone(),
    }
}

/// Specialize non-recursive higher-order array helpers (like `Data.Array.ND.nd-map-f32`)
/// when called with a concrete function argument (`Global` or `Lam`) so `array-map-f32`
/// / `array-tabulate-f32` sees the elementwise lambda directly during JVP and VJP!
fn specialize_ho_array_helpers(program: &mut Program) -> bool {
    let compute_map_units: HashSet<GlobalId> = program
        .fns
        .iter()
        .filter(|f| {
            matches!(
                f.prim,
                Some(PrimOp::ArrayCompute {
                    op: ArrayComputeOp::MapF32 | ArrayComputeOp::TabulateF32,
                    ..
                })
            )
        })
        .map(|f| f.id.clone())
        .collect();
    if compute_map_units.is_empty() {
        return false;
    }
    let fn_map: HashMap<GlobalId, FnDef> = program
        .fns
        .iter()
        .map(|f| (f.id.clone(), f.clone()))
        .collect();
    let mut ho_helpers: HashSet<GlobalId> = HashSet::new();
    for f in &program.fns {
        if f.prim.is_none()
            && f.ctor.is_none()
            && !f.params.is_empty()
            && !expr_calls_global(&f.body, &f.id)
            && expr_calls_any(&f.body, &compute_map_units)
        {
            ho_helpers.insert(f.id.clone());
        }
    }
    if ho_helpers.is_empty() {
        return false;
    }

    let mut new_fns: Vec<FnDef> = Vec::new();
    let mut counter = program.fns.len();
    let mut changed = false;
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let mut let_bindings: HashMap<LocalId, Core> = HashMap::new();
        let mut local_changed = false;
        let new_body = specialize_ho_calls_in_expr(
            &f.body,
            &fn_map,
            &ho_helpers,
            &mut let_bindings,
            &mut new_fns,
            &mut counter,
            &mut local_changed,
        );
        if local_changed {
            f.body = new_body;
            changed = true;
        }
    }
    program.fns.extend(new_fns);
    changed
}

fn expr_calls_any(e: &Core, targets: &HashSet<GlobalId>) -> bool {
    match e {
        Core::Global(g, _) => targets.contains(g),
        Core::App(h, args) => {
            expr_calls_any(h, targets) || args.iter().any(|a| expr_calls_any(a, targets))
        }
        Core::Let(_, _, r, b) => expr_calls_any(r, targets) || expr_calls_any(b, targets),
        Core::If(c, t, f) => {
            expr_calls_any(c, targets)
                || expr_calls_any(&t.body, targets)
                || expr_calls_any(&f.body, targets)
        }
        Core::Case { scrut, alts } => {
            expr_calls_any(scrut, targets)
                || alts.iter().any(|a| expr_calls_any(&a.body, targets))
        }
        Core::Join { body, rest, .. } => {
            expr_calls_any(body, targets) || expr_calls_any(rest, targets)
        }
        Core::Jump { args, .. } | Core::Lam { env: args, .. } => {
            args.iter().any(|a| expr_calls_any(a, targets))
        }
        _ => false,
    }
}

#[allow(clippy::too_many_arguments)]
fn specialize_ho_calls_in_expr(
    e: &Core,
    fn_map: &HashMap<GlobalId, FnDef>,
    ho_helpers: &HashSet<GlobalId>,
    let_bindings: &mut HashMap<LocalId, Core>,
    new_fns: &mut Vec<FnDef>,
    counter: &mut usize,
    changed: &mut bool,
) -> Core {
    match e {
        Core::App(h, args) => {
            let h2 = specialize_ho_calls_in_expr(
                h, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            );
            let args2: Vec<Core> = args
                .iter()
                .map(|a| {
                    specialize_ho_calls_in_expr(
                        a, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                    )
                })
                .collect();
            if let Core::Global(g, _) = &h2 {
                if ho_helpers.contains(g) {
                    if let Some(target_def) = fn_map.get(g) {
                        if args2.len() == target_def.params.len() {
                            let mut substs: Vec<(usize, Core)> = Vec::new();
                            let mut rem_args: Vec<Core> = Vec::new();
                            for (i, a) in args2.iter().enumerate() {
                                if let Some(known) = resolve_known_fn_expr(a, let_bindings) {
                                    substs.push((i, known));
                                } else {
                                    rem_args.push(a.clone());
                                }
                            }
                            if !substs.is_empty() {
                                let subst_indices: HashSet<usize> =
                                    substs.iter().map(|(i, _)| *i).collect();
                                let mut new_params = Vec::new();
                                let mut new_pclasses = Vec::new();
                                let mut local_remap: HashMap<LocalId, LocalId> = HashMap::new();
                                let mut next_p = 0usize;
                                for (i, p) in target_def.params.iter().enumerate() {
                                    if !subst_indices.contains(&i) {
                                        new_params.push(p.clone());
                                        new_pclasses.push(
                                            target_def
                                                .repr
                                                .params
                                                .get(i)
                                                .copied()
                                                .unwrap_or(Class::Raw),
                                        );
                                        local_remap.insert(LocalId(i), LocalId(next_p));
                                        next_p += 1;
                                    }
                                }
                                let mut next_l = max_local_in_fn(target_def) + next_p + 2;
                                let mut next_j = max_join_in_expr(&target_def.body) + 1;
                                let mut spec_body = target_def.body.clone();
                                for (i, known) in &substs {
                                    spec_body =
                                        subst_local_in_expr(&spec_body, LocalId(*i), known);
                                }
                                spec_body = freshen_with_seed(
                                    &spec_body,
                                    &local_remap,
                                    &mut next_l,
                                    &mut next_j,
                                );
                                let spec_id = GlobalId(format!("{}$hosp{}", g.0, *counter));
                                *counter += 1;
                                let mut spec_def = target_def.clone();
                                spec_def.id = spec_id.clone();
                                spec_def.params = new_params;
                                spec_def.repr.params = new_pclasses;
                                spec_def.body = spec_body;
                                new_fns.push(spec_def);
                                *changed = true;
                                return Core::App(
                                    Box::new(Core::Global(spec_id, Vec::new())),
                                    rem_args,
                                );
                            }
                        }
                    }
                }
            }
            Core::App(Box::new(h2), args2)
        }
        Core::Let(x, c, rhs, body) => {
            let r2 = specialize_ho_calls_in_expr(
                rhs, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            );
            let prev = let_bindings.insert(*x, r2.clone());
            let b2 = specialize_ho_calls_in_expr(
                body, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            );
            restore_map(let_bindings, *x, prev);
            Core::Let(*x, *c, Box::new(r2), Box::new(b2))
        }
        Core::If(c, t, f) => Core::If(
            Box::new(specialize_ho_calls_in_expr(
                c, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            )),
            CBranch {
                weight: t.weight,
                body: Box::new(specialize_ho_calls_in_expr(
                    &t.body, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                )),
            },
            CBranch {
                weight: f.weight,
                body: Box::new(specialize_ho_calls_in_expr(
                    &f.body, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                )),
            },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(specialize_ho_calls_in_expr(
                scrut, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            )),
            alts: alts
                .iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(specialize_ho_calls_in_expr(
                        &a.body, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                    )),
                })
                .collect(),
        },
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(specialize_ho_calls_in_expr(
                body, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            )),
            rest: Box::new(specialize_ho_calls_in_expr(
                rest, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
            )),
        },
        Core::Jump { id, args } => Core::Jump {
            id: *id,
            args: args
                .iter()
                .map(|a| {
                    specialize_ho_calls_in_expr(
                        a, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                    )
                })
                .collect(),
        },
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env
                .iter()
                .map(|a| {
                    specialize_ho_calls_in_expr(
                        a, fn_map, ho_helpers, let_bindings, new_fns, counter, changed,
                    )
                })
                .collect(),
            classes: classes.clone(),
        },
        other => other.clone(),
    }
}

/// Restore stub `FnDef`s for any `program.prims` entries referenced by AD-generated code
/// (such as `Data.Machine.neg-f32`, `Data.AD.cos-f32`, etc.) whose stubs were pruned before AD.
fn restore_referenced_prim_stubs(program: &mut Program) {
    let existing: HashSet<GlobalId> = program.fns.iter().map(|f| f.id.clone()).collect();
    let mut referenced: Vec<GlobalId> = Vec::new();
    for f in &program.fns {
        collect_callees(&f.body, &mut referenced);
    }
    let mut added: HashSet<GlobalId> = HashSet::new();
    for g in referenced {
        if existing.contains(&g) || !added.insert(g.clone()) {
            continue;
        }
        if let Some(plan) = program.prims.iter().find(|p| p.unit == g.0) {
            let mut meta = crate::core::FnMeta::default();
            meta.prim_body = true;
            meta.pure = plan.pure;
            program.fns.push(FnDef {
                id: g,
                params: (0..plan.arity).map(|i| format!("p{i}")).collect(),
                body: Core::Unit,
                prim: None,
                ctor: None,
                hints: Default::default(),
                meta,
                repr: UnitRepr {
                    tparams: 0,
                    params: vec![Class::Raw; plan.arity],
                    result: None,
                    fields: Vec::new(),
                    ..Default::default()
                },
            });
        }
    }
}

/// Compute all functions reachable from entry points (`Main.main` and any non-AD-wrapper
/// user/library functions that do not themselves contain unlowered AD calls).
fn reachable_from_roots(program: &Program, ad_stubs: &HashSet<GlobalId>) -> HashSet<GlobalId> {
    let fn_map: HashMap<GlobalId, &FnDef> =
        program.fns.iter().map(|f| (f.id.clone(), f)).collect();
    let mut work: Vec<GlobalId> = Vec::new();
    for f in &program.fns {
        let is_entry = f.id.0 == "Main.main"
            || (!f.id.0.starts_with("Data.AD.") && !expr_contains_ad(&f.body, ad_stubs));
        if is_entry {
            work.push(f.id.clone());
        }
    }
    let mut visited: HashSet<GlobalId> = HashSet::new();
    while let Some(id) = work.pop() {
        if !visited.insert(id.clone()) {
            continue;
        }
        if let Some(f) = fn_map.get(&id) {
            collect_callees(&f.body, &mut work);
        }
    }
    visited
}
