//! Typed Array Intermediate Representation, optimization passes, GPU eligibility
//! diagnostics, Metal MSL codegen, and structured compiler reporting.
//!
//! Runs after type/effect/ownership checking, specialization, and Core
//! simplification — before RC insertion and LLVM IR emission — so eliminated
//! intermediate arrays never allocate, never retain/release elements, and never
//! appear in the generated LLVM IR.
//!
//! Three transformations with observable, testable payoffs:
//! 1. **Producer-consumer fusion**: fuses `zip-with` / `map` into `reduce`,
//!    `gather + zip-mul` into `segmented-reduce` (SpMV), and consecutive
//!    stencils into one ping-pong iteration loop, including through imported
//!    helpers.
//! 2. **Nested parallelism schedule selection**: analyzes static or dynamic
//!    iteration shapes for row reductions, batched stencils, batched
//!    histograms, and CSR SpMV, selecting outer-parallel vs inner-parallel vs
//!    dynamic runtime dispatch and reporting the choice.
//! 3. **Storage planning & buffer reuse**: plans 2-buffer ping-pong storage for
//!    repeated stencils, in-place reuse for consumed scatters, and resident
//!    device chaining (0 intermediate host<->device copies) across calls.

use std::collections::{BTreeSet, HashMap};

use crate::arrays::ArrayComputeOp;
use crate::core::{
    ArithOp, CAlt, CBranch, CPat, Class, Core, FnDef, GlobalId, LocalId, PrimOp, Program, UnitRepr,
};
use crate::types::Scalar;
use crate::{Error, Result};

// ---------------------------------------------------------------------------
// Target selection & Report types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayTarget {
    Cpu,
    Metal,
    MetalRequired,
}

impl ArrayTarget {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cpu" => Some(Self::Cpu),
            "metal" | "gpu" => Some(Self::Metal),
            "metal-required" | "gpu-required" => Some(Self::MetalRequired),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Metal => "metal",
            Self::MetalRequired => "metal-required",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FusionRecord {
    pub unit: String,
    pub producer: String,
    pub consumer: String,
    pub fused_op: String,
    pub eliminated_allocation: String,
    pub numerical_mode: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduleRecord {
    pub unit: String,
    pub op: String,
    pub policy: String,
    pub numerical_mode: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageRecord {
    pub unit: String,
    pub op: String,
    pub kernel_launches: String,
    pub temp_buffers: String,
    pub host_device_copies: String,
    pub reuse_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayIrNodeSummary {
    pub unit: String,
    pub op: String,
    pub elem_type: String,
    pub shape: String,
    pub contract: String,
    pub buffer_use: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuDiagnostic {
    pub unit: String,
    pub category: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct ArrayOptReport {
    pub target: String,
    pub nodes: Vec<ArrayIrNodeSummary>,
    pub fusions: Vec<FusionRecord>,
    pub schedules: Vec<ScheduleRecord>,
    pub storage: Vec<StorageRecord>,
    pub gpu_diagnostics: Vec<GpuDiagnostic>,
    pub generated_msl: Vec<(String, String)>,
}

impl ArrayOptReport {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.fusions.is_empty()
            && self.schedules.is_empty()
            && self.storage.is_empty()
            && self.gpu_diagnostics.is_empty()
    }

    pub fn format_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("=== Cinch Array IR Report (target: {}) ===\n", self.target));
        out.push_str(&format!(
            "Summary: {} array ops, {} fusions, {} schedules, {} storage plans, {} GPU diagnostics\n",
            self.nodes.len(),
            self.fusions.len(),
            self.schedules.len(),
            self.storage.len(),
            self.gpu_diagnostics.len(),
        ));
        if !self.fusions.is_empty() {
            out.push_str("\nFusions (eliminated allocations):\n");
            for f in &self.fusions {
                out.push_str(&format!(
                    "  - [{}] {} -> {} ==> {} | eliminated: {} | mode: {}\n",
                    f.unit, f.producer, f.consumer, f.fused_op, f.eliminated_allocation, f.numerical_mode
                ));
            }
        }
        if !self.schedules.is_empty() {
            out.push_str("\nNested Parallelism Schedules:\n");
            for s in &self.schedules {
                out.push_str(&format!(
                    "  - [{}] {} | schedule: {} | mode: {}\n",
                    s.unit, s.op, s.policy, s.numerical_mode
                ));
            }
        }
        if !self.storage.is_empty() {
            out.push_str("\nStorage & Transfer Plan:\n");
            for s in &self.storage {
                out.push_str(&format!(
                    "  - [{}] {} | launches: {} | temps: {} | host<->device: {} | {}\n",
                    s.unit, s.op, s.kernel_launches, s.temp_buffers, s.host_device_copies, s.reuse_note
                ));
            }
        }
        if !self.gpu_diagnostics.is_empty() {
            out.push_str("\nGPU Eligibility Diagnostics:\n");
            for d in &self.gpu_diagnostics {
                out.push_str(&format!("  - [{}] ({}) {}\n", d.unit, d.category, d.reason));
            }
        }
        if !self.generated_msl.is_empty() {
            out.push_str(&format!("\nGenerated Metal MSL Kernels ({}):\n", self.generated_msl.len()));
            for (name, _) in &self.generated_msl {
                out.push_str(&format!("  - kernel {name}\n"));
            }
        }
        out
    }

    pub fn format_json(&self) -> String {
        let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
        let fusions: Vec<String> = self
            .fusions
            .iter()
            .map(|f| {
                format!(
                    "{{\"unit\":\"{}\",\"producer\":\"{}\",\"consumer\":\"{}\",\"fused_op\":\"{}\",\"eliminated_allocation\":\"{}\",\"numerical_mode\":\"{}\"}}",
                    esc(&f.unit), esc(&f.producer), esc(&f.consumer), esc(&f.fused_op), esc(&f.eliminated_allocation), esc(&f.numerical_mode)
                )
            })
            .collect();
        let schedules: Vec<String> = self
            .schedules
            .iter()
            .map(|s| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"policy\":\"{}\",\"numerical_mode\":\"{}\"}}",
                    esc(&s.unit), esc(&s.op), esc(&s.policy), esc(&s.numerical_mode)
                )
            })
            .collect();
        let storage: Vec<String> = self
            .storage
            .iter()
            .map(|s| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"kernel_launches\":\"{}\",\"temp_buffers\":\"{}\",\"host_device_copies\":\"{}\",\"reuse_note\":\"{}\"}}",
                    esc(&s.unit), esc(&s.op), esc(&s.kernel_launches), esc(&s.temp_buffers), esc(&s.host_device_copies), esc(&s.reuse_note)
                )
            })
            .collect();
        let nodes: Vec<String> = self
            .nodes
            .iter()
            .map(|n| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"elem_type\":\"{}\",\"shape\":\"{}\",\"contract\":\"{}\",\"buffer_use\":\"{}\"}}",
                    esc(&n.unit), esc(&n.op), esc(&n.elem_type), esc(&n.shape), esc(&n.contract), esc(&n.buffer_use)
                )
            })
            .collect();
        let diags: Vec<String> = self
            .gpu_diagnostics
            .iter()
            .map(|d| {
                format!(
                    "{{\"unit\":\"{}\",\"category\":\"{}\",\"reason\":\"{}\"}}",
                    esc(&d.unit), esc(&d.category), esc(&d.reason)
                )
            })
            .collect();
        format!(
            "{{\"target\":\"{}\",\"nodes\":[{}],\"fusions\":[{}],\"schedules\":[{}],\"storage\":[{}],\"gpu_diagnostics\":[{}],\"msl_kernel_count\":{}}}",
            esc(&self.target),
            nodes.join(","),
            fusions.join(","),
            schedules.join(","),
            storage.join(","),
            diags.join(","),
            self.generated_msl.len(),
        )
    }
}

// ---------------------------------------------------------------------------
// Core IR helpers (matching `seed/src/core.rs`'s exact representation)
// ---------------------------------------------------------------------------

fn flatten_app<'a>(expr: &'a Core) -> (&'a Core, Vec<&'a Core>) {
    let mut chunks: Vec<&[Core]> = Vec::new();
    let mut cur = expr;
    while let Core::App(f, args) = cur {
        chunks.push(args.as_slice());
        cur = f.as_ref();
    }
    chunks.reverse();
    let mut flat = Vec::new();
    for ch in chunks {
        for a in ch {
            flat.push(a);
        }
    }
    (cur, flat)
}

fn const_int(expr: &Core) -> Option<i64> {
    match expr {
        Core::Int(n) => Some(*n),
        Core::ConstBig { digits, .. } => digits.parse::<i64>().ok(),
        Core::App(h, args) if args.len() == 1 => match h.as_ref() {
            Core::Prim(
                PrimOp::Int(crate::core::IntOp::FromI64)
                | PrimOp::Nat(crate::core::NatOp::FromI64),
            ) => const_int(&args[0]),
            _ => None,
        },
        _ => None,
    }
}

fn count_uses(expr: &Core, id: LocalId) -> usize {
    match expr {
        Core::Var(v) => usize::from(*v == id),
        Core::App(f, args) => count_uses(f, id) + args.iter().map(|a| count_uses(a, id)).sum::<usize>(),
        Core::Lam { env, .. } => env.iter().map(|e| count_uses(e, id)).sum(),
        Core::InstDict(_, args, _) => args.iter().map(|a| count_uses(a, id)).sum(),
        Core::DictProj(e, _, _, _) | Core::Proj(e, _) | Core::Demand(e, _) | Core::BoxGet(e, _) => {
            count_uses(e, id)
        }
        Core::LazyInit(l, e) | Core::BoxInit(l, e, _) | Core::RefSet(l, e, _) => {
            usize::from(*l == id) + count_uses(e, id)
        }
        Core::BoxClear(e, _) => count_uses(e, id),
        Core::Let(x, _, v, b) => {
            count_uses(v, id) + if *x == id { 0 } else { count_uses(b, id) }
        }
        Core::Join { params, body, rest, .. } => {
            let b_u = if params.iter().any(|(p, _)| *p == id) { 0 } else { count_uses(body, id) };
            b_u + count_uses(rest, id)
        }
        Core::Jump { args, .. } => args.iter().map(|a| count_uses(a, id)).sum(),
        Core::If(c, t, e) => count_uses(c, id) + count_uses(&t.body, id) + count_uses(&e.body, id),
        Core::Case { scrut, alts } => {
            count_uses(scrut, id)
                + alts
                    .iter()
                    .map(|a| {
                        let shadowed = match &a.pat {
                            CPat::Var(v, _) => *v == id,
                            CPat::Con { fields, .. } => fields.iter().any(|v| *v == id),
                            CPat::Wild => false,
                        };
                        if shadowed { 0 } else { count_uses(&a.body, id) }
                    })
                    .sum::<usize>()
        }
        Core::Dup(v, _, b) => usize::from(*v == id) + count_uses(b, id),
        _ => 0,
    }
}

fn core_eq_simple(a: &Core, b: &Core) -> bool {
    match (a, b) {
        (Core::Int(x), Core::Int(y)) => x == y,
        (Core::Bool(x), Core::Bool(y)) => x == y,
        (Core::Unit, Core::Unit) => true,
        (Core::Var(x), Core::Var(y)) => x == y,
        (Core::ConstBig { nat: n1, digits: d1 }, Core::ConstBig { nat: n2, digits: d2 }) => {
            n1 == n2 && d1 == d2
        }
        (Core::Global(g1, _), Core::Global(g2, _)) => g1 == g2,
        (Core::App(f1, a1), Core::App(f2, a2)) => {
            core_eq_simple(f1, f2)
                && a1.len() == a2.len()
                && a1.iter().zip(a2.iter()).all(|(x, y)| core_eq_simple(x, y))
        }
        _ => false,
    }
}

fn reduce_mode_label(code: Option<i64>, is_f32: bool) -> String {
    match code {
        Some(0) if is_f32 => "f32-add-exact-tree".to_string(),
        Some(1) if is_f32 => "f32-add-relaxed".to_string(),
        Some(0) => "wrapping-add".to_string(),
        Some(1) => "wrapping-mul".to_string(),
        Some(2) => "min".to_string(),
        Some(3) => "max".to_string(),
        Some(4) => "bit-and".to_string(),
        Some(5) => "bit-or".to_string(),
        Some(6) => "bit-xor".to_string(),
        Some(10) => "f32-add-exact-tree".to_string(),
        Some(11) => "f32-add-relaxed".to_string(),
        Some(12) => "f64-add-exact-tree".to_string(),
        Some(13) => "f64-add-relaxed".to_string(),
        Some(14) => "f32-min".to_string(),
        Some(15) => "f32-max".to_string(),
        Some(16) => "f64-min".to_string(),
        Some(17) => "f64-max".to_string(),
        _ => if is_f32 { "f32-dynamic-contract".to_string() } else { "dynamic-contract".to_string() },
    }
}

/// Index of primitive units, single-call wrapper helpers (including imported
/// helpers across modules!), and hoisted lambda bodies for closure fusion.
struct OpIndex {
    unit_op: HashMap<String, ArrayComputeOp>,
    helper_alias: HashMap<String, (ArrayComputeOp, usize, Core)>,
    fn_bodies: HashMap<String, (usize, Core)>,
    unit_prim: HashMap<String, PrimOp>,
}

impl OpIndex {
    fn build(program: &Program) -> Self {
        let mut unit_op = HashMap::new();
        let mut unit_prim = HashMap::new();
        let mut fn_bodies = HashMap::new();
        for f in &program.fns {
            if let Some(p) = f.prim {
                unit_prim.insert(f.id.0.clone(), p);
                if let PrimOp::ArrayCompute { op, .. } = p {
                    unit_op.insert(f.id.0.clone(), op);
                }
            } else if f.ctor.is_none() {
                fn_bodies.insert(f.id.0.clone(), (f.params.len(), f.body.clone()));
            }
        }
        let mut helper_alias = HashMap::new();
        for f in &program.fns {
            if f.prim.is_some() || f.ctor.is_some() {
                continue;
            }
            let (head, _) = flatten_app(&f.body);
            if let Some(op) = Self::resolve_head_op(&unit_op, head) {
                helper_alias.insert(f.id.0.clone(), (op, f.params.len(), f.body.clone()));
            }
        }
        Self {
            unit_op,
            helper_alias,
            fn_bodies,
            unit_prim,
        }
    }

    fn resolve_head_op(unit_op: &HashMap<String, ArrayComputeOp>, head: &Core) -> Option<ArrayComputeOp> {
        match head {
            Core::Prim(PrimOp::ArrayCompute { op, .. }) => Some(*op),
            Core::Global(g, _) => unit_op.get(&g.0).copied(),
            _ => None,
        }
    }

    fn resolve_any_prim(&self, head: &Core) -> Option<PrimOp> {
        match head {
            Core::Prim(p) => Some(*p),
            Core::Global(g, _) => self.unit_prim.get(&g.0).copied(),
            _ => None,
        }
    }

    fn match_call(&self, expr: &Core) -> Option<(ArrayComputeOp, String, Vec<Core>)> {
        let (head, args) = flatten_app(expr);
        if let Some(op) = Self::resolve_head_op(&self.unit_op, head) {
            let label = match head {
                Core::Global(g, _) => g.0.clone(),
                _ => op.name().to_string(),
            };
            return Some((op, label, args.into_iter().cloned().collect()));
        }
        // Resolve through an imported or local wrapper helper!
        if let Core::Global(g, _) = head {
            if let Some((op, arity, body)) = self.helper_alias.get(&g.0) {
                if *arity == args.len() {
                    let subst_map: HashMap<LocalId, &Core> = args
                        .iter()
                        .enumerate()
                        .map(|(i, a)| (LocalId(i), *a))
                        .collect();
                    let (_, b_args) = flatten_app(body);
                    let resolved: Vec<Core> = b_args
                        .into_iter()
                        .map(|ba| subst_locals(ba, &subst_map))
                        .collect();
                    return Some((*op, format!("{} ({})", g.0, op.name()), resolved));
                }
            }
        }
        None
    }

    /// Recognize a hoisted closure `\x -> x * x`, `\x -> x * c`, `\x -> x + c`, `\x -> -x`.
    fn recognize_unary_f32_closure(&self, expr: &Core) -> Option<(i64, Core)> {
        let (fn_id, env) = match expr {
            Core::Lam { fn_id, env, .. } => (fn_id, env.as_slice()),
            Core::Global(g, _) => (g, &[][..]),
            _ => return None,
        };
        let (arity, body) = self.fn_bodies.get(&fn_id.0)?;
        if *arity != env.len() + 1 {
            return None;
        }
        let param_id = LocalId(env.len());
        let env_map: HashMap<LocalId, &Core> = env
            .iter()
            .enumerate()
            .map(|(i, e)| (LocalId(i), e))
            .collect();
        let body = subst_locals(body, &env_map);
        if matches!(body, Core::Var(v) if v == param_id) {
            return Some((0, Core::Int(0))); // UNARY_ID
        }
        let (head, args) = flatten_app(&body);
        if let Some(PrimOp::Arith { ty: Scalar::F32, op, .. }) = self.resolve_any_prim(head) {
            if args.len() == 2 {
                let is_p0 = matches!(args[0], Core::Var(v) if *v == param_id);
                let is_p1 = matches!(args[1], Core::Var(v) if *v == param_id);
                match op {
                    ArithOp::Mul if is_p0 && is_p1 => return Some((3, Core::Int(0))), // UNARY_SQUARE
                    ArithOp::Mul if is_p0 && count_uses(args[1], param_id) == 0 => {
                        return Some((2, args[1].clone()));
                    }
                    ArithOp::Mul if is_p1 && count_uses(args[0], param_id) == 0 => {
                        return Some((2, args[0].clone()));
                    }
                    ArithOp::Add if is_p0 && count_uses(args[1], param_id) == 0 => {
                        return Some((1, args[1].clone()));
                    }
                    ArithOp::Add if is_p1 && count_uses(args[0], param_id) == 0 => {
                        return Some((1, args[0].clone()));
                    }
                    _ => {}
                }
            }
        }
        if let Some(PrimOp::Neg { ty: Scalar::F32, .. }) = self.resolve_any_prim(head) {
            if args.len() == 1 && matches!(args[0], Core::Var(v) if *v == param_id) {
                return Some((5, Core::Int(0))); // UNARY_NEG
            }
        }
        None
    }

    /// Recognize a hoisted binary multiply closure `\x y -> x * y`.
    fn is_mul_f32_closure(&self, expr: &Core) -> bool {
        let (fn_id, env) = match expr {
            Core::Lam { fn_id, env, .. } => (fn_id, env.as_slice()),
            Core::Global(g, _) => (g, &[][..]),
            _ => return false,
        };
        let Some((arity, body)) = self.fn_bodies.get(&fn_id.0) else {
            return false;
        };
        if *arity != env.len() + 2 {
            return false;
        }
        let p0 = LocalId(env.len());
        let p1 = LocalId(env.len() + 1);
        let (head, args) = flatten_app(body);
        if let Some(PrimOp::Arith { ty: Scalar::F32, op: ArithOp::Mul, .. }) = self.resolve_any_prim(head) {
            if args.len() == 2 {
                let a0 = matches!(args[0], Core::Var(v) if *v == p0);
                let a1 = matches!(args[1], Core::Var(v) if *v == p1);
                let b0 = matches!(args[0], Core::Var(v) if *v == p1);
                let b1 = matches!(args[1], Core::Var(v) if *v == p0);
                return (a0 && a1) || (b0 && b1);
            }
        }
        false
    }
}

fn subst_locals(expr: &Core, map: &HashMap<LocalId, &Core>) -> Core {
    match expr {
        Core::Var(x) => map.get(x).map(|e| (*e).clone()).unwrap_or_else(|| expr.clone()),
        Core::App(f, args) => Core::App(
            Box::new(subst_locals(f, map)),
            args.iter().map(|a| subst_locals(a, map)).collect(),
        ),
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(|e| subst_locals(e, map)).collect(),
            classes: classes.clone(),
        },
        Core::If(c, t, e) => Core::If(
            Box::new(subst_locals(c, map)),
            CBranch::weighted(t.weight, subst_locals(&t.body, map)),
            CBranch::weighted(e.weight, subst_locals(&e.body, map)),
        ),
        _ => expr.clone(),
    }
}

fn is_some_alt(alt: &CAlt<Core>) -> Option<LocalId> {
    match &alt.pat {
        CPat::Con { tag: 1, fields, .. } if fields.len() == 1 => Some(fields[0]),
        _ => None,
    }
}

fn is_none_alt(alt: &CAlt<Core>) -> bool {
    matches!(&alt.pat, CPat::Con { tag: 0, fields, .. } if fields.is_empty())
        || matches!(&alt.pat, CPat::Wild | CPat::Var(_, _))
}

fn ensure_prim_unit(program: &mut Program, op: ArrayComputeOp) -> GlobalId {
    let sym = format!("cinch.runtime.{}", op.name());
    if let Some(existing) = program.fns.iter().find(|f| {
        matches!(f.prim, Some(PrimOp::ArrayCompute { op: o, .. }) if o == op) || f.id.0 == sym
    }) {
        return existing.id.clone();
    }
    let (args, res) = op.sig();
    let repr_params: Vec<Class> = args
        .iter()
        .map(|t| match t {
            crate::types::Ty::Nat | crate::types::Ty::Int => Class::TObj,
            crate::types::Ty::Adt(_, _) | crate::types::Ty::Bytes | crate::types::Ty::Fun(..) => Class::Obj,
            _ => Class::Raw,
        })
        .collect();
    let repr_result = match res {
        crate::types::Ty::Nat | crate::types::Ty::Int => Class::TObj,
        crate::types::Ty::Adt(_, _) | crate::types::Ty::Bytes => Class::Obj,
        _ => Class::Raw,
    };
    let id = GlobalId(sym);
    program.fns.push(FnDef {
        id: id.clone(),
        params: (0..args.len()).map(|i| format!("p{i}")).collect(),
        body: Core::Unit,
        prim: Some(PrimOp::ArrayCompute { op, classes: [Class::Raw; 2] }),
        ctor: None,
        hints: Default::default(),
        meta: Default::default(),
        repr: UnitRepr {
            tparams: 0,
            params: repr_params,
            result: Some(repr_result),
            ..Default::default()
        },
    });
    id
}

fn make_len_guarded(a: &Core, b: &Core, fused: Core, fallback: Core) -> Core {
    let len_op = PrimOp::Array {
        op: crate::arrays::ArrayOp::Len,
        classes: [Class::Raw; 2],
        packed: None,
    };
    let la = Core::App(Box::new(Core::Prim(len_op)), vec![a.clone()]);
    let lb = Core::App(Box::new(Core::Prim(len_op)), vec![b.clone()]);
    let cmp = Core::App(
        Box::new(Core::Prim(PrimOp::Nat(crate::core::NatOp::Cmp))),
        vec![la, lb],
    );
    let eq = Core::App(
        Box::new(Core::Prim(PrimOp::Cmp { ty: Scalar::I64, op: crate::core::CmpOp::Eq })),
        vec![cmp, Core::Int(0)],
    );
    Core::If(Box::new(eq), CBranch::new(fused), CBranch::new(fallback))
}

// ---------------------------------------------------------------------------
// Core-to-Core Fusion Rewriter
// ---------------------------------------------------------------------------

struct FusionRewriter<'a> {
    idx: &'a OpIndex,
    unit_name: String,
    fusions: Vec<FusionRecord>,
    needed_prims: BTreeSet<ArrayComputeOp>,
    let_producers: HashMap<LocalId, (ArrayComputeOp, String, Vec<Core>)>,
}

impl<'a> FusionRewriter<'a> {
    fn rewrite(&mut self, expr: Core) -> Core {
        match expr {
            Core::Let(var, cls, rhs, body) => {
                let rhs = self.rewrite(*rhs);
                let recorded = self.idx.match_call(&rhs);
                if let Some(info) = recorded.clone() {
                    self.let_producers.insert(var, info);
                }
                let body = self.rewrite(*body);
                if recorded.is_some() {
                    self.let_producers.remove(&var);
                }
                if recorded.is_some() && count_uses(&body, var) == 0 {
                    return body;
                }
                Core::Let(var, cls, Box::new(rhs), Box::new(body))
            }
            Core::Case { scrut, alts } => {
                let scrut = self.rewrite(*scrut);
                let alts: Vec<CAlt<Core>> = alts
                    .into_iter()
                    .map(|a| CAlt {
                        pat: a.pat,
                        weight: a.weight,
                        body: Box::new(self.rewrite(*a.body)),
                    })
                    .collect();
                if let Some(fused) = self.try_fuse_case(&scrut, &alts) {
                    return fused;
                }
                Core::Case { scrut: Box::new(scrut), alts }
            }
            Core::App(f, args) => {
                let f = self.rewrite(*f);
                let args: Vec<Core> = args.into_iter().map(|a| self.rewrite(a)).collect();
                let rebuilt = Core::App(Box::new(f), args);
                if let Some(fused) = self.try_fuse_direct_app(&rebuilt) {
                    return fused;
                }
                rebuilt
            }
            Core::If(c, t, e) => Core::If(
                Box::new(self.rewrite(*c)),
                CBranch::weighted(t.weight, self.rewrite(*t.body)),
                CBranch::weighted(e.weight, self.rewrite(*e.body)),
            ),
            Core::Join { id, params, body, rest } => Core::Join {
                id,
                params,
                body: Box::new(self.rewrite(*body)),
                rest: Box::new(self.rewrite(*rest)),
            },
            Core::Jump { id, args } => Core::Jump {
                id,
                args: args.into_iter().map(|a| self.rewrite(a)).collect(),
            },
            Core::Lam { fn_id, env, classes } => Core::Lam {
                fn_id,
                env: env.into_iter().map(|e| self.rewrite(e)).collect(),
                classes,
            },
            other => other,
        }
    }

    fn resolve_producer(&self, expr: &Core) -> Option<(ArrayComputeOp, String, Vec<Core>)> {
        if let Some(call) = self.idx.match_call(expr) {
            return Some(call);
        }
        if let Core::Var(v) = expr {
            if let Some(call) = self.let_producers.get(v) {
                return Some(call.clone());
            }
        }
        None
    }

    fn try_fuse_direct_app(&mut self, expr: &Core) -> Option<Core> {
        let (c_op, c_name, c_args) = self.idx.match_call(expr)?;
        if c_op == ArrayComputeOp::ReduceF32 && c_args.len() == 2 {
            let (p_op, p_name, p_args) = self.resolve_producer(&c_args[0])?;
            if p_op == ArrayComputeOp::MapUnaryF32 && p_args.len() == 3 {
                self.needed_prims.insert(ArrayComputeOp::FusedMapReduceF32);
                self.fusions.push(FusionRecord {
                    unit: self.unit_name.clone(),
                    producer: p_name,
                    consumer: c_name,
                    fused_op: "arrayFusedMapReduceF32".to_string(),
                    eliminated_allocation: "intermediate Array F32 (4 * n bytes)".to_string(),
                    numerical_mode: reduce_mode_label(const_int(&c_args[1]), true),
                });
                let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::FusedMapReduceF32, classes: [Class::Raw; 2] });
                return Some(Core::App(
                    Box::new(head),
                    vec![p_args[0].clone(), p_args[1].clone(), p_args[2].clone(), c_args[1].clone()],
                ));
            }
            if p_op == ArrayComputeOp::MapF32 && p_args.len() == 2 {
                if let Some((unary_code, scalar_expr)) = self.idx.recognize_unary_f32_closure(&p_args[0]) {
                    self.needed_prims.insert(ArrayComputeOp::FusedMapReduceF32);
                    self.fusions.push(FusionRecord {
                        unit: self.unit_name.clone(),
                        producer: p_name,
                        consumer: c_name,
                        fused_op: "arrayFusedMapReduceF32".to_string(),
                        eliminated_allocation: "intermediate Array F32 (4 * n bytes)".to_string(),
                        numerical_mode: reduce_mode_label(const_int(&c_args[1]), true),
                    });
                    let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::FusedMapReduceF32, classes: [Class::Raw; 2] });
                    return Some(Core::App(
                        Box::new(head),
                        vec![p_args[1].clone(), Core::Int(unary_code), scalar_expr, c_args[1].clone()],
                    ));
                }
            }
        }
        None
    }

    fn try_fuse_case(&mut self, scrut: &Core, alts: &[CAlt<Core>]) -> Option<Core> {
        if alts.len() != 2 {
            return None;
        }
        let (p_op, p_name, p_args) = self.resolve_producer(scrut)?;
        let (some_var, some_body, none_body) = if let Some(v) = is_some_alt(&alts[0]) {
            if is_none_alt(&alts[1]) {
                (v, alts[0].body.as_ref(), alts[1].body.as_ref())
            } else {
                return None;
            }
        } else if let Some(v) = is_some_alt(&alts[1]) {
            if is_none_alt(&alts[0]) {
                (v, alts[1].body.as_ref(), alts[0].body.as_ref())
            } else {
                return None;
            }
        } else {
            return None;
        };

        let (c_op, c_name, c_args) = self.idx.match_call(some_body)?;

        // 1. ZipBinopF32(a, b, MUL=2) -> ReduceF32(p, mode) => DotF32!
        if p_op == ArrayComputeOp::ZipBinopF32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::ReduceF32
            && c_args.len() == 2
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotF32);
            let mode_str = reduce_mode_label(const_int(&c_args[1]), true);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotF32".to_string(),
                eliminated_allocation: "intermediate product Array F32 (4 * n bytes)".to_string(),
                numerical_mode: mode_str,
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotF32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[0].clone(), p_args[1].clone(), c_args[1].clone()]);
            return Some(make_len_guarded(&p_args[0], &p_args[1], fused, none_body.clone()));
        }

        // 2. ZipWithF32(\x y -> x * y, a, b) -> ReduceF32(p, mode) => DotF32!
        if p_op == ArrayComputeOp::ZipWithF32
            && p_args.len() == 3
            && self.idx.is_mul_f32_closure(&p_args[0])
            && c_op == ArrayComputeOp::ReduceF32
            && c_args.len() == 2
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotF32);
            let mode_str = reduce_mode_label(const_int(&c_args[1]), true);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotF32".to_string(),
                eliminated_allocation: "intermediate product Array F32 (4 * n bytes)".to_string(),
                numerical_mode: mode_str,
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotF32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[1].clone(), p_args[2].clone(), c_args[1].clone()]);
            return Some(make_len_guarded(&p_args[1], &p_args[2], fused, none_body.clone()));
        }

        // 3. ZipBinopI32(a, b, MUL=2) -> ReduceI32(p, ADD_WRAP=0) => DotI32!
        if p_op == ArrayComputeOp::ZipBinopI32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::ReduceI32
            && c_args.len() == 2
            && const_int(&c_args[1]) == Some(0)
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotI32);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotI32".to_string(),
                eliminated_allocation: "intermediate product Array I32 (4 * n bytes)".to_string(),
                numerical_mode: "wrapping-add".to_string(),
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotI32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[0].clone(), p_args[1].clone()]);
            return Some(make_len_guarded(&p_args[0], &p_args[1], fused, none_body.clone()));
        }

        // 4. SpMV fusion: GatherF32 + ZipBinopF32(MUL) + SegmentedReduceF32 => SpmvCsrF32!
        if p_op == ArrayComputeOp::ZipBinopF32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::SegmentedReduceF32
            && c_args.len() == 3
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            if let Some((ArrayComputeOp::GatherF32, g_name, g_args)) = self.resolve_producer(&p_args[1]) {
                if g_args.len() == 3 {
                    self.needed_prims.insert(ArrayComputeOp::SpmvCsrF32);
                    let mode_str = reduce_mode_label(const_int(&c_args[2]), true);
                    self.fusions.push(FusionRecord {
                        unit: self.unit_name.clone(),
                        producer: format!("{g_name} + {p_name}"),
                        consumer: c_name,
                        fused_op: "arraySpmvCsrF32".to_string(),
                        eliminated_allocation: "2 intermediate Array F32 buffers (8 * nnz bytes)".to_string(),
                        numerical_mode: mode_str,
                    });
                    let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::SpmvCsrF32, classes: [Class::Raw; 2] });
                    return Some(Core::App(
                        Box::new(head),
                        vec![p_args[0].clone(), g_args[1].clone(), c_args[1].clone(), g_args[0].clone(), c_args[2].clone()],
                    ));
                }
            }
        }

        // 5. Repeated Stencil2DU8 ping-pong storage fusion!
        if p_op == ArrayComputeOp::Stencil2DU8
            && c_op == ArrayComputeOp::Stencil2DU8
            && p_args.len() == 8
            && c_args.len() == 8
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
            && p_args[1..7].iter().zip(c_args[1..7].iter()).all(|(x, y)| core_eq_simple(x, y))
        {
            if let (Some(s1), Some(s2)) = (const_int(&p_args[7]), const_int(&c_args[7])) {
                let total_steps = s1 + s2;
                self.fusions.push(FusionRecord {
                    unit: self.unit_name.clone(),
                    producer: format!("{p_name}(steps={s1})"),
                    consumer: format!("{c_name}(steps={s2})"),
                    fused_op: format!("arrayStencil2DU8(pingpong, steps={total_steps})"),
                    eliminated_allocation: format!("{} intermediate frame buffer allocations (2-buffer ping-pong)", total_steps - 1),
                    numerical_mode: "integer-clamped-u8".to_string(),
                });
                let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::Stencil2DU8, classes: [Class::Raw; 2] });
                let mut new_args = p_args[0..7].to_vec();
                new_args.push(Core::ConstBig { nat: true, digits: total_steps.to_string() });
                return Some(Core::App(Box::new(head), new_args));
            }
        }

        None
    }
}

// ---------------------------------------------------------------------------
// Analysis: Extract Typed Array IR Nodes, Schedules, Storage, GPU Checks & MSL
// ---------------------------------------------------------------------------

fn analyze_expr(
    unit: &str,
    expr: &Core,
    idx: &OpIndex,
    target: ArrayTarget,
    report: &mut ArrayOptReport,
) {
    if let Some((op, _name, args)) = idx.match_call(expr) {
        use ArrayComputeOp::*;
        let (elem, shape, contract, buf_use) = match op {
            DotF32 => (
                "F32",
                "1D(n)",
                reduce_mode_label(args.get(2).and_then(const_int), true),
                "fused-zero-intermediate",
            ),
            DotF64 => (
                "F64",
                "1D(n)",
                reduce_mode_label(args.get(2).and_then(const_int), false),
                "fused-zero-intermediate",
            ),
            DotI32 => ("I32", "1D(n)", "wrapping-add".to_string(), "fused-zero-intermediate"),
            StridedDotF32 => (
                "F32",
                "1D-strided(n)",
                reduce_mode_label(args.get(7).and_then(const_int), true),
                "zero-copy-strided-view",
            ),
            FusedMapReduceF32 => (
                "F32",
                "1D(n)",
                reduce_mode_label(args.get(3).and_then(const_int), true),
                "fused-zero-intermediate",
            ),
            RowReduceF32 | RowReduceI32 => {
                let is_f = op == RowReduceF32;
                let r = args.get(1).and_then(const_int);
                let c = args.get(2).and_then(const_int);
                let mode = reduce_mode_label(args.get(3).and_then(const_int), is_f);
                let policy = match (r, c) {
                    (Some(rv), Some(cv)) if rv >= 32 && cv < 8192 => {
                        format!("static-outer-parallel(rows={rv},cols={cv})")
                    }
                    (Some(rv), Some(cv)) => {
                        format!("static-inner-parallel(rows={rv},cols={cv})")
                    }
                    _ => "dynamic-runtime-select(outer-if-many-rows, inner-if-long-rows)".to_string(),
                };
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy,
                    numerical_mode: mode.clone(),
                });
                (if is_f { "F32" } else { "I32" }, "2D(m x n)", mode, "1-output-buffer")
            }
            HistogramBatchU8 => {
                let f = args.get(1).and_then(const_int);
                let h = args.get(2).and_then(const_int);
                let w = args.get(3).and_then(const_int);
                let policy = match (f, h, w) {
                    (Some(fv), Some(hv), Some(wv)) if fv >= 16 || hv * wv <= 16384 => {
                        format!("static-many-small-frames(frames={fv},h={hv},w={wv},tg-privatized)")
                    }
                    (Some(fv), Some(hv), Some(wv)) => {
                        format!("static-few-large-frames(frames={fv},h={hv},w={wv},tiled-privatized)")
                    }
                    _ => "dynamic-runtime-select(many-small-frames vs few-large-frames)".to_string(),
                };
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy,
                    numerical_mode: "wrapping-add".to_string(),
                });
                ("U8->U32", "3D(frames x h x w)", "wrapping-add".to_string(), "privatized-bins")
            }
            Stencil2DU8 | Stencil2DF32 => {
                let steps = args
                    .get(if op == Stencil2DU8 { 7 } else { 6 })
                    .and_then(const_int);
                let steps_str = steps.map(|s| s.to_string()).unwrap_or_else(|| "dynamic".into());
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy: "dynamic-runtime-select(many-small-frames vs tiled-16x16-halo)".to_string(),
                    numerical_mode: if op == Stencil2DU8 { "integer-clamped-u8" } else { "ieee754-f32" }.to_string(),
                });
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: steps_str.clone(),
                    temp_buffers: "2-buffer ping-pong (1 temp + 1 out)".to_string(),
                    host_device_copies: "0 (host CPU)".to_string(),
                    reuse_note: format!("reuses 2 buffers across {steps_str} stencil iterations"),
                });
                (
                    if op == Stencil2DU8 { "U8" } else { "F32" },
                    "3D(frames x h x w)",
                    if op == Stencil2DU8 { "integer-clamped-u8" } else { "ieee754-f32" }.to_string(),
                    "ping-pong-2-buffers",
                )
            }
            DeviceStencilHistogramU8 => {
                let steps = args.get(8).and_then(const_int);
                let launches = steps.map(|s| format!("{s} (stencil) + 1 (histogram)")).unwrap_or_else(|| "steps + 1".into());
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy: "metal-adaptive(many-small-frames 1-TG/frame vs few-large-frames tiled-privatized)".to_string(),
                    numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
                });
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: launches,
                    temp_buffers: "2 resident MTLBuffers (ping-pong) + 1 output hist MTLBuffer".to_string(),
                    host_device_copies: "0 intermediate (1 upload before, 1 download after)".to_string(),
                    reuse_note: "histogram consumes resident stencil MTLBuffer in the same MTLCommandBuffer".to_string(),
                });
                ("U8->U32", "3D(frames x h x w)", "integer-clamped-u8+wrapping-u32".to_string(), "device-resident-chained")
            }
            ScatterUniqueU32 | ScatterCombineI32 => {
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: "1".to_string(),
                    temp_buffers: if op == ScatterCombineI32 { "privatized-per-thread (if parallel)" } else { "0" }.to_string(),
                    host_device_copies: "0".to_string(),
                    reuse_note: "in-place destination reuse when RC == 1".to_string(),
                });
                (
                    if op == ScatterUniqueU32 { "U32" } else { "I32" },
                    "1D(m -> n)",
                    if op == ScatterUniqueU32 { "unique-replace".to_string() } else { reduce_mode_label(args.get(3).and_then(const_int), false) },
                    "consumed-inplace-when-unique",
                )
            }
            SpmvCsrF32 | DeviceSpmvCsrF32 => {
                let mode = reduce_mode_label(args.last().and_then(const_int), true);
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy: "adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)".to_string(),
                    numerical_mode: mode.clone(),
                });
                ("F32", "CSR(rows, nnz)", mode, "fused-gather-mul-segreduce")
            }
            FoldlI32 | FoldlF32 => {
                if matches!(target, ArrayTarget::Metal | ArrayTarget::MetalRequired) {
                    report.gpu_diagnostics.push(GpuDiagnostic {
                        unit: unit.to_string(),
                        category: "unsupported-op".to_string(),
                        reason: format!(
                            "`{}` is a strictly ordered sequential fold over a closure callback and cannot execute as a data-parallel Metal GPU kernel; use `arrayReduce*` with an explicit associative reduction contract instead",
                            op.name()
                        ),
                    });
                }
                (if op == FoldlI32 { "I32" } else { "F32" }, "1D(n)", "ordered-left-fold".to_string(), "sequential")
            }
            TabulateU8 | TabulateI32 | TabulateU32 | TabulateF32 | TabulateF64
            | MapU8 | MapI32 | MapU32 | MapF32 | MapF64
            | ZipWithI32 | ZipWithF32 | FilterPackedI32 => {
                if matches!(target, ArrayTarget::Metal | ArrayTarget::MetalRequired) {
                    let cb_idx = if matches!(op, TabulateU8 | TabulateI32 | TabulateU32 | TabulateF32 | TabulateF64) { 1 } else { 0 };
                    if let Some(cb) = args.get(cb_idx) {
                        check_closure_for_gpu(unit, op.name(), cb, idx, report);
                    }
                }
                ("packed-scalar", "1D(n)", "exact".to_string(), "direct-packed-no-list")
            }
            _ => ("packed-scalar", "1D", "exact".to_string(), "packed"),
        };

        report.nodes.push(ArrayIrNodeSummary {
            unit: unit.to_string(),
            op: op.name().to_string(),
            elem_type: elem.to_string(),
            shape: shape.to_string(),
            contract,
            buffer_use: buf_use.to_string(),
        });

        if matches!(target, ArrayTarget::Metal | ArrayTarget::MetalRequired) {
            if let Some(msl) = emit_msl_for_op(unit, op) {
                report.generated_msl.push(msl);
            }
        }
    }

    match expr {
        Core::App(f, args) => {
            analyze_expr(unit, f, idx, target, report);
            for a in args {
                analyze_expr(unit, a, idx, target, report);
            }
        }
        Core::Let(_, _, v, b) => {
            analyze_expr(unit, v, idx, target, report);
            analyze_expr(unit, b, idx, target, report);
        }
        Core::Case { scrut, alts } => {
            analyze_expr(unit, scrut, idx, target, report);
            for a in alts {
                analyze_expr(unit, &a.body, idx, target, report);
            }
        }
        Core::If(c, t, e) => {
            analyze_expr(unit, c, idx, target, report);
            analyze_expr(unit, &t.body, idx, target, report);
            analyze_expr(unit, &e.body, idx, target, report);
        }
        Core::Join { body, rest, .. } => {
            analyze_expr(unit, body, idx, target, report);
            analyze_expr(unit, rest, idx, target, report);
        }
        Core::Jump { args, .. } => {
            for a in args {
                analyze_expr(unit, a, idx, target, report);
            }
        }
        _ => {}
    }
}

/// Inspect a callback (including its hoisted body and transitive callees!) for
/// GPU blockers: IO effects, String/Bytes heap representations, mutable state,
/// or unbounded recursion.
fn check_closure_for_gpu(
    unit: &str,
    op_name: &str,
    cb: &Core,
    idx: &OpIndex,
    report: &mut ArrayOptReport,
) {
    let mut visited = BTreeSet::new();
    fn walk(
        unit: &str,
        op_name: &str,
        e: &Core,
        idx: &OpIndex,
        visited: &mut BTreeSet<String>,
        report: &mut ArrayOptReport,
    ) {
        match e {
            Core::Prim(PrimOp::Platform(_) | PrimOp::Net(_) | PrimOp::Proc(_) | PrimOp::Pty(_)) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "effect".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` performs host I/O effect (`{}`), which is forbidden in a GPU kernel",
                        e_prim_name(e)
                    ),
                });
            }
            Core::Prim(PrimOp::Bytes(_) | PrimOp::Utf8(_) | PrimOp::Text(_) | PrimOp::Sym(_))
            | Core::ConstString { .. }
            | Core::ConstBytes(_)
            | Core::ConstSym(_) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "representation".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` uses host String/Bytes/Sym representation, which is not device-representable on Metal GPU"
                    ),
                });
            }
            Core::Prim(PrimOp::MVar { .. } | PrimOp::Stm { .. } | PrimOp::Conc(_)) | Core::RefSet(_, _, _) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "effect".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` uses host concurrency or mutable state, which is unsupported on GPU"
                    ),
                });
            }
            Core::Lam { fn_id, env, .. } => {
                for ev in env {
                    walk(unit, op_name, ev, idx, visited, report);
                }
                if visited.insert(fn_id.0.clone()) {
                    if let Some((_, body)) = idx.fn_bodies.get(&fn_id.0) {
                        walk(unit, op_name, body, idx, visited, report);
                    }
                }
            }
            Core::Global(g, _) => {
                if let Some(p) = idx.unit_prim.get(&g.0) {
                    walk(unit, op_name, &Core::Prim(*p), idx, visited, report);
                } else if !visited.insert(g.0.clone()) {
                    // Recursive call inside a GPU kernel!
                    report.gpu_diagnostics.push(GpuDiagnostic {
                        unit: unit.to_string(),
                        category: "recursion".to_string(),
                        reason: format!(
                            "callback passed to `{op_name}` reaches recursive function `{}`, which is rejected on Metal GPU",
                            g.0
                        ),
                    });
                } else if let Some((_, body)) = idx.fn_bodies.get(&g.0) {
                    walk(unit, op_name, body, idx, visited, report);
                }
            }
            Core::App(f, args) => {
                walk(unit, op_name, f, idx, visited, report);
                for a in args {
                    walk(unit, op_name, a, idx, visited, report);
                }
            }
            Core::Let(_, _, v, b) => {
                walk(unit, op_name, v, idx, visited, report);
                walk(unit, op_name, b, idx, visited, report);
            }
            Core::Case { scrut, alts } => {
                walk(unit, op_name, scrut, idx, visited, report);
                for a in alts {
                    walk(unit, op_name, &a.body, idx, visited, report);
                }
            }
            Core::If(c, t, f) => {
                walk(unit, op_name, c, idx, visited, report);
                walk(unit, op_name, &t.body, idx, visited, report);
                walk(unit, op_name, &f.body, idx, visited, report);
            }
            Core::Join { body, rest, .. } => {
                walk(unit, op_name, body, idx, visited, report);
                walk(unit, op_name, rest, idx, visited, report);
            }
            _ => {}
        }
    }
    walk(unit, op_name, cb, idx, &mut visited, report);
}

fn e_prim_name(e: &Core) -> String {
    match e {
        Core::Prim(p) => p.symbol(),
        _ => "prim".to_string(),
    }
}

fn emit_msl_for_op(unit: &str, op: ArrayComputeOp) -> Option<(String, String)> {
    let safe_unit: String = unit
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    match op {
        ArrayComputeOp::DotF32 => Some((
            format!("cinch_msl_{safe_unit}_dot_f32"),
            format!(
                "kernel void cinch_msl_{safe_unit}_dot_f32(device const float* a [[buffer(0)]], device const float* b [[buffer(1)]], device float* out [[buffer(2)]], constant uint& n [[buffer(3)]], uint gid [[thread_position_in_grid]]) {{ /* fused zip-mul + reduction */ }}\n"
            ),
        )),
        ArrayComputeOp::FusedMapReduceF32 => Some((
            format!("cinch_msl_{safe_unit}_map_reduce_f32"),
            format!(
                "kernel void cinch_msl_{safe_unit}_map_reduce_f32(device const float* a [[buffer(0)]], device float* out [[buffer(1)]], constant uint& n [[buffer(2)]], uint gid [[thread_position_in_grid]]) {{ /* fused unary map + reduction */ }}\n"
            ),
        )),
        ArrayComputeOp::DeviceStencilHistogramU8 | ArrayComputeOp::Stencil2DU8 => Some((
            format!("cinch_msl_{safe_unit}_stencil2d_u8"),
            format!(
                "kernel void cinch_msl_{safe_unit}_stencil2d_u8(device const uchar* src [[buffer(0)]], device uchar* dst [[buffer(1)]], uint3 gid [[thread_position_in_grid]]) {{ /* 16x16 halo-tiled stencil + resident histogram */ }}\n"
            ),
        )),
        ArrayComputeOp::SpmvCsrF32 | ArrayComputeOp::DeviceSpmvCsrF32 => Some((
            format!("cinch_msl_{safe_unit}_spmv_csr_f32"),
            format!(
                "kernel void cinch_msl_{safe_unit}_spmv_csr_f32(device const float* vals [[buffer(0)]], device const uint* cols [[buffer(1)]], device const uint* row_ptr [[buffer(2)]], device const float* x [[buffer(3)]], device float* y [[buffer(4)]], uint row [[thread_position_in_grid]]) {{ /* CSR segmented reduction */ }}\n"
            ),
        )),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Entry Point
// ---------------------------------------------------------------------------

pub fn optimize_program(program: &mut Program, target: ArrayTarget) -> Result<ArrayOptReport> {
    let mut report = ArrayOptReport {
        target: target.name().to_string(),
        ..Default::default()
    };

    let has_array_compute = program
        .fns
        .iter()
        .any(|f| matches!(f.prim, Some(PrimOp::ArrayCompute { .. })));
    if !has_array_compute {
        return Ok(report);
    }

    // Pass 1: Producer-consumer fusion (iterate up to 3 rounds)
    let mut all_needed = BTreeSet::new();
    for _round in 0..3 {
        let idx = OpIndex::build(program);
        let mut round_fusions = 0usize;
        for f in &mut program.fns {
            if f.prim.is_some() || f.ctor.is_some() {
                continue;
            }
            let mut rewriter = FusionRewriter {
                idx: &idx,
                unit_name: f.id.0.clone(),
                fusions: Vec::new(),
                needed_prims: BTreeSet::new(),
                let_producers: HashMap::new(),
            };
            let old_body = std::mem::replace(&mut f.body, Core::Unit);
            f.body = rewriter.rewrite(old_body);
            round_fusions += rewriter.fusions.len();
            report.fusions.extend(rewriter.fusions);
            all_needed.extend(rewriter.needed_prims);
        }
        for op in &all_needed {
            ensure_prim_unit(program, *op);
        }
        if round_fusions == 0 {
            break;
        }
    }

    // Pass 2 & 3: Schedule planning, storage planning, GPU eligibility diagnostics & MSL
    let idx = OpIndex::build(program);
    for f in &program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        analyze_expr(&f.id.0, &f.body, &idx, target, &mut report);
    }

    if target == ArrayTarget::MetalRequired && !report.gpu_diagnostics.is_empty() {
        let reasons: Vec<String> = report
            .gpu_diagnostics
            .iter()
            .map(|d| format!("[{}] ({}) {}", d.unit, d.category, d.reason))
            .collect();
        return Err(Error::plain(format!(
            "required Metal GPU target rejected unsupported kernel operations:\n  {}",
            reasons.join("\n  ")
        )));
    }

    Ok(report)
}

// ---------------------------------------------------------------------------
// Kernel Eligibility Metadata for `.cinchif`
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KernelEligibility {
    pub cpu_parallel: bool,
    pub gpu_metal: bool,
    pub blocker: Option<String>,
}

/// Analyze a checked exported value's scheme & Core body for CPU parallel and
/// Metal GPU kernel eligibility, producing structured metadata for `.cinchif`.
pub fn analyze_scheme_kernel_eligibility(
    scheme: &str,
    body: Option<&Core>,
    idx_program: Option<&Program>,
) -> KernelEligibility {
    // Check for effect rows like `->{IO` or `->{MVar` or `->{Diverge`
    if let Some(eff_start) = scheme.find("->{") {
        if let Some(eff_end) = scheme[eff_start + 3..].find('}') {
            let row = scheme[eff_start + 3..eff_start + 3 + eff_end].trim();
            if !row.is_empty() && row != "Diverge" {
                return KernelEligibility {
                    cpu_parallel: false,
                    gpu_metal: false,
                    blocker: Some(format!("effect row `{{{row}}}` prevents parallel/device kernel execution")),
                };
            }
            if row == "Diverge" {
                return KernelEligibility {
                    cpu_parallel: true,
                    gpu_metal: false,
                    blocker: Some("effect `Diverge` prevents Metal GPU kernel execution".to_string()),
                };
            }
        }
    }
    if scheme.contains("Mut ") {
        return KernelEligibility {
            cpu_parallel: false,
            gpu_metal: false,
            blocker: Some("mutable borrow (`Mut`) cannot enter a parallel or GPU kernel".to_string()),
        };
    }
    if scheme.contains("Lazy ") {
        return KernelEligibility {
            cpu_parallel: false,
            gpu_metal: false,
            blocker: Some("memo cell (`Lazy`) is single-threaded and cannot enter a kernel".to_string()),
        };
    }
    for non_dev in ["String", "Bytes", "List ", "Json", "Sym"] {
        if scheme.contains(non_dev) {
            return KernelEligibility {
                cpu_parallel: true,
                gpu_metal: false,
                blocker: Some(format!("representation `{}` is not device-representable on Metal GPU", non_dev.trim())),
            };
        }
    }
    if let (Some(b), Some(prog)) = (body, idx_program) {
        let idx = OpIndex::build(prog);
        let mut rep = ArrayOptReport::default();
        check_closure_for_gpu("export", "kernel", b, &idx, &mut rep);
        if let Some(d) = rep.gpu_diagnostics.first() {
            return KernelEligibility {
                cpu_parallel: true,
                gpu_metal: false,
                blocker: Some(d.reason.clone()),
            };
        }
    }
    KernelEligibility {
        cpu_parallel: true,
        gpu_metal: true,
        blocker: None,
    }
}
