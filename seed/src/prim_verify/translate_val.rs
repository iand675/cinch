//! Translation Validation (`Core` → `Core` optimizer & `Core` → emitted LLVM IR).
//!
//! Lowers pure scalar/branching, interprocedural, tail-recursive, and stack-aggregate
//! Cinch [`crate::core::FnDef`] bodies directly into the shared [`super::sem::SemGraph`]
//! and proves:
//! 1. **Optimizer validation (`validate_core_transform`):**
//!    `SemGraph(Core_before) == SemGraph(Core_after)` across `simplify::simplify`.
//! 2. **Codegen validation (`validate_core_vs_llvm`):**
//!    `SemGraph(Core) ⊑ SemGraph(emitted LLVM IR)` for `codegen::emit`, proving
//!    the emitted LLVM function is UB-free, poison-free, and bit-for-bit identical
//!    to the Core function with a kernel-checked LRAT proof, including:
//!    - Interprocedural calls (same-module scalar inlining + Ackermann uninterpreted-function congruence).
//!    - Tail-recursive / bounded loops (inductive loop-step bisimulation cutting the loop backedge).
//!    - Aggregate / `#[repr(C)]` / `#[repr(value)]` struct & `FixedArray` symbolic slot/GEP memory ops.
//! 3. **Perceus RC / drop-insertion safety (`validate_fn_rc_safety` / `validate_module_rc_safety`):**
//!    Audits emitted LLVM functions for balanced `@cinch.rc.dup.*` / `@cinch.rc.drop.*` /
//!    `@cinch_free` / `@cinch_frame_release` ownership across CFG paths, rejecting
//!    double-free, use-after-free, unbalanced branch drops, and RC leaks.
//!
//! TODO(verify-opt): Add Alive2-style LLVM IR-to-IR translation validation across
//! `opt -O2` / clang optimization passes (`validate_llvm_opt_pass`), comparing
//! pre-optimization and post-optimization LLVM IR modules for UB/poison refinement.

use super::llvm_ir::{
    lower_llvm_fragment, lower_llvm_fragment_with_uf, UfCallHook, LLVM_SEMANTICS_ID,
};
use super::sem::{NodeId, Obligation, ObligationKind, SemGraph, SemParam, SemProgram, SemTy, SemVal};
use super::{Policy, SemError, VerificationStatus, VerifierEngine};
use crate::core::{BitOp, CmpOp, Core, FnDef, LocalId, PrimOp, Program};
use crate::repr::Class;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const CORE_TV_SEMANTICS_ID: &str = "cinch-core-tv@1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TvFunctionReport {
    pub fn_name: String,
    pub stage: String,
    pub status: String,
    pub details: String,
}

fn class_to_sem_ty(class: &Class) -> Option<SemTy> {
    match class {
        Class::Raw => Some(SemTy::Bv(64)),
        Class::Wide => Some(SemTy::Bv(128)),
        _ => None,
    }
}

/// Tracks uninterpreted pure scalar calls `(callee, args) -> fresh_param_node`
/// so Ackermann congruence axioms `(/\i a_i == b_i) => res_a == res_b` can be
/// conjoined into pure bit-vector LRAT obligations.
#[derive(Clone, Debug, Default)]
pub(crate) struct UfRegistry {
    pub(crate) next_param_idx: usize,
    pub(crate) calls: Vec<(String, Vec<NodeId>, SemTy, NodeId)>,
}

impl UfRegistry {
    fn new(start_param_idx: usize) -> Self {
        Self {
            next_param_idx: start_param_idx,
            calls: Vec::new(),
        }
    }

    fn record(
        &mut self,
        g: &mut SemGraph,
        callee: &str,
        args: &[NodeId],
        ret_ty: SemTy,
    ) -> NodeId {
        // Exact structural reuse when same callee & identical NodeId args appear in same graph.
        for (c, prev_args, prev_ty, res) in &self.calls {
            if c == callee && prev_args.as_slice() == args && *prev_ty == ret_ty {
                return *res;
            }
        }
        let idx = self.next_param_idx;
        self.next_param_idx += 1;
        let pname = format!("uf_{idx}_{callee}");
        let node = g.param(idx, pname, ret_ty);
        self.calls.push((callee.to_string(), args.to_vec(), ret_ty, node));
        node
    }

    /// Conjoin Ackermann congruence axioms across every pair of calls to the same callee.
    fn ackermann_axioms(&self, g: &mut SemGraph) -> NodeId {
        let mut acc = g.bool_const(true);
        for i in 0..self.calls.len() {
            for j in (i + 1)..self.calls.len() {
                let (ref ca, ref aa, ta, ra) = self.calls[i];
                let (ref cb, ref ab, tb, rb) = self.calls[j];
                if ca != cb || aa.len() != ab.len() || ta != tb {
                    continue;
                }
                let mut same_args = g.bool_const(true);
                for (&x, &y) in aa.iter().zip(ab.iter()) {
                    let eq = match (g.ty(x), g.ty(y)) {
                        (SemTy::Bool, SemTy::Bool) => g.iff(x, y),
                        (SemTy::Bv(w1), SemTy::Bv(w2)) if w1 == w2 => g.bv_eq(x, y),
                        _ => g.bool_const(false),
                    };
                    same_args = g.and(same_args, eq);
                }
                let same_res = match ta {
                    SemTy::Bool => g.iff(ra, rb),
                    SemTy::Bv(_) => g.bv_eq(ra, rb),
                    SemTy::Fp(_) => g.bool_const(true),
                };
                let axiom = g.implies(same_args, same_res);
                acc = g.and(acc, axiom);
            }
        }
        acc
    }

    fn collect_sem_params(&self, g: &SemGraph) -> Vec<SemParam> {
        let mut out = Vec::new();
        for (_, _, ty, node) in &self.calls {
            if let super::sem::SemOp::Param { index, name, .. } = g.op(*node) {
                out.push(SemParam {
                    index: *index,
                    name: name.clone(),
                    ty: *ty,
                    signed: true,
                });
            }
        }
        out
    }
}

/// Lower a pure scalar/branching `Core::FnDef` into a `SemProgram`.
pub fn lower_core_fn(
    f: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
) -> Result<SemProgram, String> {
    lower_core_fn_for_word_bits(f, prim_bodies, 64)
}

/// Lower a pure scalar/branching `Core::FnDef` using `word_bits` (32 or 64) for `Usize`/`Isize`.
pub fn lower_core_fn_for_word_bits(
    f: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    word_bits: u16,
) -> Result<SemProgram, String> {
    let mut uf = UfRegistry::new(f.params.len());
    lower_core_fn_with_env(f, prim_bodies, &HashMap::new(), word_bits, &mut uf, false)
}

fn lower_core_fn_with_env(
    f: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    all_fns: &HashMap<String, &FnDef>,
    word_bits: u16,
    uf: &mut UfRegistry,
    allow_uf: bool,
) -> Result<SemProgram, String> {
    let mut g = SemGraph::new();
    let mut params = Vec::with_capacity(f.params.len());
    let mut env: HashMap<LocalId, NodeId> = HashMap::new();
    for (idx, pname) in f.params.iter().enumerate() {
        let cls = f.repr.params.get(idx).cloned().unwrap_or(Class::Raw);
        let ty = class_to_sem_ty(&cls)
            .ok_or_else(|| format!("non-scalar parameter class `{cls:?}` on `{}`", f.id.0))?;
        let node = g.param(idx, pname.clone(), ty);
        let p = SemParam {
            index: idx,
            name: pname.clone(),
            ty,
            signed: true,
        };
        params.push((p, node));
        // Core local IDs for function parameters are 0..params.len()
        env.insert(LocalId(idx), node);
    }

    let mut visiting = HashSet::new();
    visiting.insert(f.id.0.clone());
    let res_node = lower_core_expr_ext(
        &mut g,
        &f.body,
        &env,
        prim_bodies,
        all_fns,
        &mut visiting,
        word_bits,
        uf,
        allow_uf,
    )?;
    let ret_ty = f
        .repr
        .result
        .as_ref()
        .and_then(class_to_sem_ty)
        .unwrap_or(SemTy::Bv(64));
    let coerced = coerce_bv(&mut g, res_node, ret_ty)?;
    let f_bool = g.bool_const(false);
    Ok(SemProgram {
        semantics_id: CORE_TV_SEMANTICS_ID.to_string(),
        graph: g,
        params,
        result: SemVal::Scalar { val: coerced, poison: f_bool },
        ub_cond: f_bool,
    })
}

fn coerce_bv(g: &mut SemGraph, node: NodeId, target: SemTy) -> Result<NodeId, String> {
    let cur = g.ty(node);
    if cur == target {
        return Ok(node);
    }
    match (cur, target) {
        (SemTy::Bool, SemTy::Bv(w)) => {
            let one = g.bv_const(w, 1);
            let zero = g.bv_const(w, 0);
            Ok(g.ite(node, one, zero))
        }
        (SemTy::Bv(w), SemTy::Bool) => {
            let zero = g.bv_const(w, 0);
            let eq = g.bv_eq(node, zero);
            Ok(g.not(eq))
        }
        (SemTy::Bv(fw), SemTy::Bv(tw)) if fw < tw => Ok(g.bv_zext(node, tw)),
        (SemTy::Bv(fw), SemTy::Bv(tw)) if fw > tw => Ok(g.bv_trunc(node, tw)),
        _ => Err(format!("cannot coerce {cur} to {target} in Core TV")),
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_core_expr_ext(
    g: &mut SemGraph,
    expr: &Core,
    env: &HashMap<LocalId, NodeId>,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    all_fns: &HashMap<String, &FnDef>,
    visiting: &mut HashSet<String>,
    word_bits: u16,
    uf: &mut UfRegistry,
    allow_uf: bool,
) -> Result<NodeId, String> {
    match expr {
        Core::Int(n) => Ok(g.bv_const(64, *n as u64 as u128)),
        Core::Wide(n) => Ok(g.bv_const(128, *n)),
        Core::Bool(b) => Ok(g.bv_const(64, if *b { 1 } else { 0 })),
        Core::Unit => Ok(g.bv_const(64, 0)),
        Core::Var(lid) => env
            .get(lid)
            .copied()
            .ok_or_else(|| format!("unbound local `{lid:?}` in Core TV")),
        Core::Let(lid, _cls, rhs, body) => {
            let rv = lower_core_expr_ext(g, rhs, env, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf)?;
            let mut next = env.clone();
            next.insert(*lid, rv);
            lower_core_expr_ext(g, body, &next, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf)
        }
        Core::If(cond, then_b, else_b) => {
            let cv = lower_core_expr_ext(g, cond, env, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf)?;
            let c_bool = coerce_bv(g, cv, SemTy::Bool)?;
            let tv = lower_core_expr_ext(g, &then_b.body, env, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf)?;
            let ev = lower_core_expr_ext(g, &else_b.body, env, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf)?;
            let tv_ty = g.ty(tv);
            let ev_c = coerce_bv(g, ev, tv_ty)?;
            Ok(g.ite(c_bool, tv, ev_c))
        }
        Core::App(head, args) => {
            let mut arg_nodes = Vec::with_capacity(args.len());
            for a in args {
                arg_nodes.push(lower_core_expr_ext(
                    g, a, env, prim_bodies, all_fns, visiting, word_bits, uf, allow_uf,
                )?);
            }
            match head.as_ref() {
                Core::Prim(op) => lower_core_primop(g, *op, &arg_nodes, word_bits),
                Core::Global(gid, _) => {
                    if let Some((p_params, p_ret, frag)) = prim_bodies.get(&gid.0) {
                        let sub = lower_llvm_fragment(frag, p_params, *p_ret)
                            .map_err(|e| format!("lowering callee `{}`: {e}", gid.0))?;
                        let mut coerced_args = Vec::with_capacity(arg_nodes.len());
                        for (an, pp) in arg_nodes.iter().zip(p_params.iter()) {
                            coerced_args.push(coerce_bv(g, *an, pp.ty)?);
                        }
                        let map = g.import_graph(&sub.graph, &coerced_args);
                        let (rv, _) = sub
                            .result
                            .as_scalar()
                            .ok_or_else(|| format!("non-scalar callee `{}`", gid.0))?;
                        let out = map[rv.0 as usize];
                        match g.ty(out) {
                            SemTy::Bv(128) => Ok(out),
                            SemTy::Bv(w) if w < 64 => Ok(g.bv_zext(out, 64)),
                            _ => Ok(out),
                        }
                    } else if let Ok(named) = lower_named_scalar_global(g, &gid.0, &arg_nodes, word_bits) {
                        Ok(named)
                    } else if let Some(&callee_fn) = all_fns.get(&gid.0) {
                        if visiting.len() < 5
                            && !visiting.contains(&gid.0)
                            && callee_fn.params.len() == arg_nodes.len()
                            && callee_fn
                                .repr
                                .params
                                .iter()
                                .all(|c| class_to_sem_ty(c).is_some())
                            && callee_fn
                                .repr
                                .result
                                .as_ref()
                                .is_none_or(|c| class_to_sem_ty(c).is_some())
                        {
                            let mut sub_env = HashMap::new();
                            for (i, &an) in arg_nodes.iter().enumerate() {
                                let pcls = callee_fn.repr.params.get(i).cloned().unwrap_or(Class::Raw);
                                let pty = class_to_sem_ty(&pcls).unwrap_or(SemTy::Bv(64));
                                sub_env.insert(LocalId(i), coerce_bv(g, an, pty)?);
                            }
                            visiting.insert(gid.0.clone());
                            let inline_res = lower_core_expr_ext(
                                g,
                                &callee_fn.body,
                                &sub_env,
                                prim_bodies,
                                all_fns,
                                visiting,
                                word_bits,
                                uf,
                                allow_uf,
                            );
                            visiting.remove(&gid.0);
                            if let Ok(inlined) = inline_res {
                                let rty = callee_fn
                                    .repr
                                    .result
                                    .as_ref()
                                    .and_then(class_to_sem_ty)
                                    .unwrap_or(SemTy::Bv(64));
                                return coerce_bv(g, inlined, rty);
                            }
                        }
                        if allow_uf {
                            let rty = callee_fn
                                .repr
                                .result
                                .as_ref()
                                .and_then(class_to_sem_ty)
                                .unwrap_or(SemTy::Bv(64));
                            Ok(uf.record(g, &gid.0, &arg_nodes, rty))
                        } else {
                            Err(format!("callee `{}` could not be inlined in Core TV", gid.0))
                        }
                    } else if allow_uf {
                        Ok(uf.record(g, &gid.0, &arg_nodes, SemTy::Bv(64)))
                    } else {
                        Err(format!("callee `{}` is not a modeled pure scalar op in Core TV", gid.0))
                    }
                }
                Core::Foreign(fname) if allow_uf => Ok(uf.record(g, fname, &arg_nodes, SemTy::Bv(64))),
                _ => Err("unsupported non-direct application in Core TV".to_string()),
            }
        }
        _ => Err(format!("unsupported Core node in scalar TV: {:?}", std::mem::discriminant(expr))),
    }
}

fn lower_core_primop(g: &mut SemGraph, op: PrimOp, args: &[NodeId], word_bits: u16) -> Result<NodeId, String> {
    match op {
        PrimOp::Arith { ty, op: aop, mode: crate::core::ArithMode::Wrapping } => {
            let w = ty.width_for(word_bits as u32) as u16;
            let a = coerce_bv(g, args[0], SemTy::Bv(w))?;
            let b = coerce_bv(g, args[1], SemTy::Bv(w))?;
            let r = match aop {
                crate::core::ArithOp::Add => g.bv_add(a, b),
                crate::core::ArithOp::Sub => g.bv_sub(a, b),
                crate::core::ArithOp::Mul => g.bv_mul(a, b),
            };
            if w < 64 { Ok(g.bv_zext(r, 64)) } else { Ok(r) }
        }
        PrimOp::Bit { ty, op: bop } => {
            let w = ty.width_for(word_bits as u32) as u16;
            let a = coerce_bv(g, args[0], SemTy::Bv(w))?;
            let b = coerce_bv(g, args[1], SemTy::Bv(w))?;
            let r = match bop {
                BitOp::And => g.bv_and(a, b),
                BitOp::Or => g.bv_or(a, b),
                BitOp::Xor => g.bv_xor(a, b),
            };
            if w < 64 { Ok(g.bv_zext(r, 64)) } else { Ok(r) }
        }
        PrimOp::Cmp { ty, op: cop } => {
            let w = ty.width_for(word_bits as u32) as u16;
            let a = coerce_bv(g, args[0], SemTy::Bv(w))?;
            let b = coerce_bv(g, args[1], SemTy::Bv(w))?;
            let signed = ty.is_signed();
            let c = match cop {
                CmpOp::Eq => g.bv_eq(a, b),
                CmpOp::Ne => { let e = g.bv_eq(a, b); g.not(e) }
                CmpOp::Lt => if signed { g.bv_slt(a, b) } else { g.bv_ult(a, b) },
                CmpOp::Le => if signed { g.bv_sle(a, b) } else { g.bv_ule(a, b) },
                CmpOp::Gt => if signed { g.bv_slt(b, a) } else { g.bv_ult(b, a) },
                CmpOp::Ge => if signed { g.bv_sle(b, a) } else { g.bv_ule(b, a) },
            };
            coerce_bv(g, c, SemTy::Bv(64))
        }
        PrimOp::EqBool => {
            let a = coerce_bv(g, args[0], SemTy::Bool)?;
            let b = coerce_bv(g, args[1], SemTy::Bool)?;
            let c = g.iff(a, b);
            coerce_bv(g, c, SemTy::Bv(64))
        }
        PrimOp::NeBool => {
            let a = coerce_bv(g, args[0], SemTy::Bool)?;
            let b = coerce_bv(g, args[1], SemTy::Bool)?;
            let c = g.xor(a, b);
            coerce_bv(g, c, SemTy::Bv(64))
        }
        other => Err(format!("unsupported PrimOp `{other:?}` in Core TV")),
    }
}

fn lower_named_scalar_global(
    g: &mut SemGraph,
    name: &str,
    args: &[NodeId],
    word_bits: u16,
) -> Result<NodeId, String> {
    let base = name.rsplit('.').next().unwrap_or(name);
    let parse_w = |suf: &str| -> Option<(u16, bool)> {
        match suf {
            "i8" => Some((8, true)),
            "i16" => Some((16, true)),
            "i32" => Some((32, true)),
            "i64" => Some((64, true)),
            "i128" => Some((128, true)),
            "u8" => Some((8, false)),
            "u16" => Some((16, false)),
            "u32" => Some((32, false)),
            "u64" => Some((64, false)),
            "u128" => Some((128, false)),
            "usize" => Some((word_bits, false)),
            "isize" => Some((word_bits, true)),
            _ => None,
        }
    };
    if args.len() == 2 {
        for (prefix, kind) in [
            ("wrapping-add-", "add"),
            ("wrapping-sub-", "sub"),
            ("wrapping-mul-", "mul"),
            ("bit-and-", "and"),
            ("bit-or-", "or"),
            ("bit-xor-", "xor"),
            ("eq-", "eq"),
            ("ne-", "ne"),
            ("lt-", "lt"),
            ("le-", "le"),
            ("gt-", "gt"),
            ("ge-", "ge"),
        ] {
            if let Some(suf) = base.strip_prefix(prefix) {
                if let Some((w, signed)) = parse_w(suf) {
                    let a = coerce_bv(g, args[0], SemTy::Bv(w))?;
                    let b = coerce_bv(g, args[1], SemTy::Bv(w))?;
                    let r = match kind {
                        "add" => g.bv_add(a, b),
                        "sub" => g.bv_sub(a, b),
                        "mul" => g.bv_mul(a, b),
                        "and" => g.bv_and(a, b),
                        "or" => g.bv_or(a, b),
                        "xor" => g.bv_xor(a, b),
                        "eq" => g.bv_eq(a, b),
                        "ne" => { let e = g.bv_eq(a, b); g.not(e) }
                        "lt" => if signed { g.bv_slt(a, b) } else { g.bv_ult(a, b) },
                        "le" => if signed { g.bv_sle(a, b) } else { g.bv_ule(a, b) },
                        "gt" => if signed { g.bv_slt(b, a) } else { g.bv_ult(b, a) },
                        "ge" => if signed { g.bv_sle(b, a) } else { g.bv_ule(b, a) },
                        _ => unreachable!(),
                    };
                    return match g.ty(r) {
                        SemTy::Bv(128) => Ok(r),
                        _ => coerce_bv(g, r, SemTy::Bv(64)),
                    };
                }
            }
        }
    }
    Err(format!("callee `{name}` is not a modeled pure scalar op in Core TV"))
}

/// Build an `Obligation` proving `SemGraph(Core_before) == SemGraph(Core_after)`
/// (validating an optimizer pass such as `simplify::simplify`).
pub fn build_core_transform_obligation(
    before: &FnDef,
    after: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    stage: &str,
) -> Result<Obligation, String> {
    build_core_transform_obligation_for_word_bits(before, after, prim_bodies, stage, 64)
}

/// Target-width-aware version of [`build_core_transform_obligation`].
pub fn build_core_transform_obligation_for_word_bits(
    before: &FnDef,
    after: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    stage: &str,
    word_bits: u16,
) -> Result<Obligation, String> {
    let pa = lower_core_fn_for_word_bits(before, prim_bodies, word_bits)?;
    let pb = lower_core_fn_for_word_bits(after, prim_bodies, word_bits)?;
    let (va, _) = pa.result.as_scalar().ok_or("non-scalar before")?;
    let (vb, _) = pb.result.as_scalar().ok_or("non-scalar after")?;

    let mut g = pa.graph.clone();
    let pnodes: Vec<NodeId> = pa.params.iter().map(|(_, n)| *n).collect();
    let map_b = g.import_graph(&pb.graph, &pnodes);
    let vb_m = map_b[vb.0 as usize];
    let goal = g.bv_eq(va, vb_m);
    let sem_params: Vec<SemParam> = pa.params.into_iter().map(|(p, _)| p).collect();
    Ok(Obligation::new(
        format!("{}::tv({stage})", before.id.0),
        CORE_TV_SEMANTICS_ID,
        ObligationKind::TranslationVal {
            fn_name: before.id.0.clone(),
            stage: stage.to_string(),
        },
        sem_params,
        g,
        goal,
    ))
}

/// Extract one `define ... @"<unit>"(...) { ... }` function body from emitted LLVM IR,
/// recursively inline direct calls to other scalar `define`s in the same module (e.g.
/// prim wrappers and `.llvm` bodies), and perform exact acyclic SSA `mem2reg` over
/// non-escaping local `alloca` slots so `llvm-sem@21.1` can formally verify the real
/// codegen output without weakening its pure-SSA soundness boundary.
pub fn extract_emitted_llvm_fn(ir: &str, unit_name: &str) -> Option<(Vec<SemParam>, SemTy, String)> {
    let (params, ret_ty, raw_lines) = extract_raw_llvm_fn(ir, unit_name)?;
    let mut counter = 0usize;
    let inlined = inline_direct_calls(ir, raw_lines, &mut counter, 0).ok()?;
    let ssa_lines = acyclic_mem2reg(inlined).ok()?;
    Some((params, ret_ty, ssa_lines.join("\n")))
}

fn extract_raw_llvm_fn(ir: &str, unit_name: &str) -> Option<(Vec<SemParam>, SemTy, Vec<String>)> {
    let needle = format!("@\"{unit_name}\"(");
    let mut lines = ir.lines();
    let header = lines.find(|l| l.starts_with("define ") && l.contains(&needle))?;
    let lparen = header.find('(')?;
    let rparen = header.rfind(')')?;
    let ret_section = header["define ".len()..header.find('@')?].trim();
    let ret_ty = ret_section
        .split_whitespace()
        .rev()
        .find_map(parse_llvm_scalar_ty)?;

    let params_str = &header[lparen + 1..rparen];
    let mut params = Vec::new();
    let mut renames: Vec<(String, String)> = Vec::new();
    if !params_str.trim().is_empty() {
        for (idx, item) in params_str.split(',').enumerate() {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let sem_ty = parts.iter().copied().find_map(parse_llvm_scalar_ty)?;
            let pname = parts.last().copied().unwrap_or("");
            if pname.starts_with('%') && pname != format!("%{idx}") {
                renames.push((pname.to_string(), format!("%{idx}")));
            }
            params.push(SemParam {
                index: idx,
                name: format!("p{idx}"),
                ty: sem_ty,
                signed: true,
            });
        }
    }

    let mut body_lines = Vec::new();
    for raw in lines {
        let trimmed = raw.trim();
        if trimmed == "}" {
            break;
        }
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        let mut clean = if let Some(pos) = trimmed.find(", !prof") {
            trimmed[..pos].trim().to_string()
        } else {
            trimmed.to_string()
        };
        for (from, to) in &renames {
            clean = replace_llvm_reg(&clean, from, to);
        }
        body_lines.push(clean);
    }
    Some((params, ret_ty, body_lines))
}

/// Inline straight-line (or trivial linear-chain) direct calls `%dst = call <ty> @"<callee>"(<args...>)`
/// whose definition is present in the same emitted LLVM IR module.
fn inline_direct_calls(
    ir: &str,
    lines: Vec<String>,
    counter: &mut usize,
    depth: usize,
) -> Result<Vec<String>, String> {
    if depth > 6 {
        return Err("call inlining depth exceeded in TV".to_string());
    }
    let mut out = Vec::with_capacity(lines.len());
    let mut aliases: Vec<(String, String)> = Vec::new();

    for raw_line in lines {
        let mut line = raw_line;
        for (from, to) in &aliases {
            line = replace_llvm_reg(&line, from, to);
        }
        if let Some((dst, callee, arg_vals)) = parse_direct_quoted_call(&line) {
            if let Some((_cparams, _cret, raw_callee)) = extract_raw_llvm_fn(ir, &callee) {
                if let Ok(callee_inlined) = inline_direct_calls(ir, raw_callee, counter, depth + 1) {
                    // Run symbolic mem2reg + trivial linear-chain flattening on the callee so
                    // Cinch's `entry: alloca/store; br label %bb0; bb0: load/...; ret` callees
                    // inline cleanly into a single straight-line sequence!
                    let callee_lines = acyclic_mem2reg(callee_inlined.clone())
                        .map(flatten_trivial_linear_cfg)
                        .unwrap_or(callee_inlined);
                    let mut ret_val: Option<String> = None;
                    let mut inner_insts = Vec::new();
                    let mut has_cfg = false;
                    let mut label_count = 0usize;
                    for cl in &callee_lines {
                        let t = cl.trim();
                        if t.ends_with(':') {
                            label_count += 1;
                            if label_count > 1 {
                                has_cfg = true;
                                break;
                            }
                            continue;
                        }
                        if t.starts_with("br ") || t.starts_with("switch ") || t.starts_with("phi ") {
                            has_cfg = true;
                            break;
                        }
                        if let Some(rest) = t.strip_prefix("ret ") {
                            let parts: Vec<&str> = rest.split_whitespace().collect();
                            if parts.len() >= 2 {
                                ret_val = Some(parts[1].to_string());
                            }
                        } else {
                            inner_insts.push(t.to_string());
                        }
                    }
                    if !has_cfg {
                        if let Some(rv) = ret_val {
                            let id = *counter;
                            *counter += 1;
                            let prefix = format!("%in{id}_");
                            let mut local_defs: Vec<String> = Vec::new();
                            for inst in &inner_insts {
                                if let Some((lhs, _)) = inst.split_once('=') {
                                    let reg = lhs.trim().to_string();
                                    if reg.starts_with('%') && !local_defs.contains(&reg) {
                                        local_defs.push(reg);
                                    }
                                }
                            }
                            let rewrite_callee_line = |s: &str| -> String {
                                let mut cur = s.to_string();
                                for (li, lreg) in local_defs.iter().enumerate() {
                                    cur = replace_llvm_reg(&cur, lreg, &format!("%__INL_{id}_{li}__"));
                                }
                                for (ai, aval) in arg_vals.iter().enumerate() {
                                    cur = replace_llvm_reg(&cur, &format!("%{ai}"), aval);
                                }
                                for (li, lreg) in local_defs.iter().enumerate() {
                                    let bare = lreg.trim_start_matches('%');
                                    cur = replace_llvm_reg(
                                        &cur,
                                        &format!("%__INL_{id}_{li}__"),
                                        &format!("{prefix}{bare}"),
                                    );
                                }
                                cur
                            };
                            for inst in &inner_insts {
                                out.push(rewrite_callee_line(inst));
                            }
                            let mapped_rv = rewrite_callee_line(&rv);
                            aliases.push((dst, mapped_rv));
                            continue;
                        }
                    }
                }
            }
        }
        out.push(line);
    }
    Ok(out)
}

/// Flatten a trivial linear basic-block chain (`entry: ...; br label %bb0; bb0: ...; ret ...`)
/// with no conditional branches or phis into a single basic block for clean inlining.
fn flatten_trivial_linear_cfg(lines: Vec<String>) -> Vec<String> {
    if lines.iter().any(|l| {
        let t = l.trim();
        t.starts_with("br i1 ") || t.starts_with("switch ") || t.contains("= phi ")
    }) {
        return lines;
    }
    // Check that every `br label %X` immediately precedes `X:`
    let mut out = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        if let Some(target) = t.strip_prefix("br label %") {
            if let Some(next) = lines.get(i + 1) {
                if next.trim().strip_suffix(':') == Some(target.trim()) {
                    i += 2;
                    continue;
                }
            }
            return lines;
        }
        if t.ends_with(':') && out.is_empty() {
            i += 1;
            continue;
        }
        out.push(lines[i].clone());
        i += 1;
    }
    out
}

fn parse_direct_quoted_call(line: &str) -> Option<(String, String, Vec<String>)> {
    let (lhs, rhs) = line.split_once('=')?;
    let dst = lhs.trim();
    if !dst.starts_with('%') {
        return None;
    }
    let rhs = rhs.trim();
    let rhs = rhs
        .strip_prefix("musttail ")
        .or_else(|| rhs.strip_prefix("notail "))
        .or_else(|| rhs.strip_prefix("tail "))
        .unwrap_or(rhs);
    if !rhs.starts_with("call ") {
        return None;
    }
    let at_q = rhs.find("@\"")?;
    let after_q = &rhs[at_q + 2..];
    let end_q = after_q.find('"')?;
    let callee = after_q[..end_q].to_string();
    let rest = &after_q[end_q + 1..];
    let lparen = rest.find('(')?;
    let rparen = rest.rfind(')')?;
    let args_str = &rest[lparen + 1..rparen];
    let mut args = Vec::new();
    if !args_str.trim().is_empty() {
        for item in args_str.split(',') {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let val = parts.last().copied()?.to_string();
            args.push(val);
        }
    }
    Some((dst.to_string(), callee, args))
}

/// Layout of a local stack `alloca` slot (scalar, `FixedArray` `[N x T]`, or
/// `#[repr(C)]` / `#[repr(value)]` struct `{ T0, T1, ... }`).
#[derive(Clone, Debug)]
struct StackSlotLayout {
    total_bytes: i64,
    /// Map from constant byte offset -> LLVM scalar type string (e.g. `"i64"`).
    cells: HashMap<i64, String>,
    /// Element stride for `[N x T]` arrays, or struct field offsets for `{ T0, T1, ... }`.
    array_elem: Option<(String, i64, i64)>,
    struct_fields: Vec<(i64, String)>,
}

fn llvm_scalar_size_bytes(ty: &str) -> Option<i64> {
    match ty.trim() {
        "i1" | "i8" => Some(1),
        "i16" => Some(2),
        "i32" | "float" => Some(4),
        "i64" | "double" | "ptr" => Some(8),
        "i128" => Some(16),
        _ => None,
    }
}

fn parse_alloca_layout(rhs_after_alloca: &str) -> Option<StackSlotLayout> {
    let trimmed = rhs_after_alloca.trim();
    // 1. Struct: `alloca { i64, i32, i32 }[, align ...]`
    if let Some(after_lb) = trimmed.strip_prefix('{') {
        let rb = after_lb.find('}')?;
        let inner = &after_lb[..rb];
        let mut offset: i64 = 0;
        let mut cells = HashMap::new();
        let mut struct_fields = Vec::new();
        for part in inner.split(',') {
            let fty = part.trim();
            if fty.is_empty() {
                continue;
            }
            let sz = llvm_scalar_size_bytes(fty)?;
            // Natural C alignment (up to 16 bytes)
            let align = sz.min(16);
            if align > 1 {
                offset = (offset + align - 1) / align * align;
            }
            cells.insert(offset, fty.to_string());
            struct_fields.push((offset, fty.to_string()));
            offset += sz;
        }
        return Some(StackSlotLayout {
            total_bytes: offset.max(1),
            cells,
            array_elem: None,
            struct_fields,
        });
    }
    // 2. FixedArray: `alloca [N x i64][, align ...]`
    if let Some(after_lb) = trimmed.strip_prefix('[') {
        let rb = after_lb.find(']')?;
        let inner = &after_lb[..rb];
        let (cnt_str, ety_str) = inner.split_once(" x ")?;
        let count: i64 = cnt_str.trim().parse().ok()?;
        let ety = ety_str.trim();
        let esz = llvm_scalar_size_bytes(ety)?;
        if !(0..=1024).contains(&count) {
            return None;
        }
        let mut cells = HashMap::new();
        for i in 0..count {
            cells.insert(i * esz, ety.to_string());
        }
        return Some(StackSlotLayout {
            total_bytes: (count * esz).max(1),
            cells,
            array_elem: Some((ety.to_string(), esz, count)),
            struct_fields: Vec::new(),
        });
    }
    // 3. Scalar: `alloca i64[, align ...]`
    let ty = trimmed
        .split(|c: char| c.is_whitespace() || c == ',')
        .next()
        .unwrap_or("");
    let sz = llvm_scalar_size_bytes(ty)?;
    if parse_llvm_scalar_ty(ty).is_none() {
        return None;
    }
    let mut cells = HashMap::new();
    cells.insert(0, ty.to_string());
    Some(StackSlotLayout {
        total_bytes: sz,
        cells,
        array_elem: None,
        struct_fields: Vec::new(),
    })
}

/// Resolve `%dst = getelementptr [inbounds] <ty>, ptr %base, <iW idx0>[, <iW idx1>, ...]`
/// into `(base_slot, byte_offset)` when all indices are compile-time constants.
fn parse_const_gep(
    rhs: &str,
    slot_layouts: &HashMap<String, StackSlotLayout>,
    ptr_map: &HashMap<String, (String, i64)>,
) -> Option<Result<(String, i64), String>> {
    let rest = rhs.strip_prefix("getelementptr ")?.trim();
    let rest = rest.strip_prefix("inbounds ").unwrap_or(rest).trim();
    // Split `<ty>, ptr %base, idx0, ...` respecting `{ ... }` and `[ ... ]`.
    let parts = split_top_level_commas(rest);
    if parts.len() < 3 {
        return None;
    }
    let gep_ty = parts[0].trim();
    let ptr_part = parts[1].trim().strip_prefix("ptr ")?.trim();
    let (base_slot, base_off) = ptr_map.get(ptr_part)?.clone();
    let layout = slot_layouts.get(&base_slot)?;

    let mut idxs = Vec::new();
    for p in &parts[2..] {
        let toks: Vec<&str> = p.split_whitespace().collect();
        if toks.len() < 2 {
            return None;
        }
        let iv: i64 = toks[1].parse().ok()?;
        idxs.push(iv);
    }

    let delta: i64 = if gep_ty.starts_with('{') {
        let g_layout = parse_alloca_layout(gep_ty)?;
        let outer = idxs[0] * g_layout.total_bytes;
        if idxs.len() >= 2 {
            let fidx = idxs[1];
            if fidx < 0 || (fidx as usize) >= g_layout.struct_fields.len() {
                return Some(Err(format!(
                    "out-of-bounds struct GEP field index {fidx} on `{base_slot}`"
                )));
            }
            outer + g_layout.struct_fields[fidx as usize].0
        } else {
            outer
        }
    } else if gep_ty.starts_with('[') {
        let g_layout = parse_alloca_layout(gep_ty)?;
        let outer = idxs[0] * g_layout.total_bytes;
        if idxs.len() >= 2 {
            let (_, esz, cnt) = g_layout.array_elem?;
            let eidx = idxs[1];
            if eidx < 0 || eidx >= cnt {
                return Some(Err(format!(
                    "out-of-bounds FixedArray GEP index {eidx} (len {cnt}) on `{base_slot}`"
                )));
            }
            outer + eidx * esz
        } else {
            outer
        }
    } else {
        let esz = llvm_scalar_size_bytes(gep_ty)?;
        idxs[0] * esz
    };

    let new_off = base_off + delta;
    if new_off < 0 || new_off >= layout.total_bytes {
        return Some(Err(format!(
            "out-of-bounds stack GEP offset {new_off} on `{base_slot}` (size {} bytes)",
            layout.total_bytes
        )));
    }
    Some(Ok((base_slot, new_off)))
}

fn split_top_level_commas(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in s.chars() {
        match ch {
            '{' | '[' | '(' | '<' => {
                depth += 1;
                cur.push(ch);
            }
            '}' | ']' | ')' | '>' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// Exact acyclic SSA promotion & symbolic stack memory forwarding (`mem2reg`) over
/// non-escaping local `alloca` slots (scalars, `FixedArray` `[N x T]`, and
/// `#[repr(C)]`/`#[repr(value)]` structs `{ T0, T1, ... }`), with constant GEP
/// offset resolution and bounds/alignment well-definedness checks.
fn acyclic_mem2reg(lines: Vec<String>) -> Result<Vec<String>, String> {
    #[derive(Clone)]
    struct Block {
        label: String,
        lines: Vec<String>,
    }

    let mut blocks: Vec<Block> = Vec::new();
    let mut cur_label = "entry".to_string();
    let mut cur_lines: Vec<String> = Vec::new();
    let mut saw_label = false;

    for l in lines {
        let t = l.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(lbl) = t.strip_suffix(':') {
            let lbl = lbl.trim();
            if !lbl.is_empty() && !lbl.contains(char::is_whitespace) {
                if saw_label || !cur_lines.is_empty() {
                    blocks.push(Block {
                        label: cur_label,
                        lines: std::mem::take(&mut cur_lines),
                    });
                }
                cur_label = lbl.to_string();
                saw_label = true;
                continue;
            }
        }
        cur_lines.push(t.to_string());
    }
    blocks.push(Block {
        label: cur_label,
        lines: cur_lines,
    });

    // Identify local scalar/array/struct allocas and build their byte-offset cell tables.
    let mut slot_layouts: HashMap<String, StackSlotLayout> = HashMap::new();
    let mut cell_tys: HashMap<(String, i64), String> = HashMap::new();
    for b in &blocks {
        for l in &b.lines {
            if let Some((lhs, rhs)) = l.split_once('=') {
                let reg = lhs.trim();
                let rhs = rhs.trim();
                if let Some(rest) = rhs.strip_prefix("alloca ") {
                    if let Some(layout) = parse_alloca_layout(rest) {
                        for (&off, ty) in &layout.cells {
                            cell_tys.insert((reg.to_string(), off), ty.clone());
                        }
                        slot_layouts.insert(reg.to_string(), layout);
                    }
                }
            }
        }
    }
    if slot_layouts.is_empty() {
        let mut flat = Vec::new();
        for (i, b) in blocks.iter().enumerate() {
            if blocks.len() > 1 || i > 0 {
                flat.push(format!("{}:", b.label));
            }
            flat.extend(b.lines.iter().cloned());
        }
        return Ok(flat);
    }

    // Build label -> block index map and successor edges, then compute topological order.
    let label_to_idx: HashMap<String, usize> = blocks
        .iter()
        .enumerate()
        .map(|(i, b)| (b.label.clone(), i))
        .collect();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); blocks.len()];
    let mut succs: Vec<Vec<usize>> = vec![Vec::new(); blocks.len()];
    for (bi, b) in blocks.iter().enumerate() {
        if let Some(term) = b.lines.last() {
            for target in parse_branch_targets(term) {
                if let Some(&ti) = label_to_idx.get(&target) {
                    succs[bi].push(ti);
                    preds[ti].push(bi);
                }
            }
        }
    }
    // Kahn's topological sort (also verifies acyclicity!).
    let mut indeg: Vec<usize> = preds.iter().map(|p| p.len()).collect();
    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for (i, &d) in indeg.iter().enumerate() {
        if d == 0 {
            queue.push_back(i);
        }
    }
    let mut topo = Vec::with_capacity(blocks.len());
    while let Some(u) = queue.pop_front() {
        topo.push(u);
        for &v in &succs[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if topo.len() != blocks.len() {
        return Err("cyclic CFG encountered during TV mem2reg".to_string());
    }

    // Walk blocks in topological order, promoting stack cells `(slot, offset)` to SSA / phi.
    let mut ptr_map: HashMap<String, (String, i64)> = HashMap::new();
    for sname in slot_layouts.keys() {
        ptr_map.insert(sname.clone(), (sname.clone(), 0));
    }
    let mut out_slots: Vec<HashMap<(String, i64), String>> = vec![HashMap::new(); blocks.len()];
    let mut emitted_blocks: Vec<(String, Vec<String>)> = vec![(String::new(), Vec::new()); blocks.len()];
    let mut aliases: Vec<(String, String)> = Vec::new();
    let mut phi_counter = 0usize;

    for &bi in &topo {
        let b = &blocks[bi];
        let mut cur_slots: HashMap<(String, i64), String> = HashMap::new();
        let mut new_lines: Vec<String> = Vec::new();

        if preds[bi].len() == 1 {
            cur_slots = out_slots[preds[bi][0]].clone();
        } else if preds[bi].len() > 1 {
            let mut keys: Vec<(String, i64)> = cell_tys.keys().cloned().collect();
            keys.sort();
            for key in keys {
                let mut incoming: Vec<(String, String)> = Vec::new();
                for &pi in &preds[bi] {
                    if let Some(val) = out_slots[pi].get(&key) {
                        incoming.push((val.clone(), blocks[pi].label.clone()));
                    }
                }
                if incoming.len() == preds[bi].len() {
                    let first_val = &incoming[0].0;
                    if incoming.iter().all(|(v, _)| v == first_val) {
                        cur_slots.insert(key, first_val.clone());
                    } else {
                        let ty = &cell_tys[&key];
                        let phi_reg = format!("%m2r_{phi_counter}");
                        phi_counter += 1;
                        let pairs: Vec<String> = incoming
                            .iter()
                            .map(|(v, lbl)| format!("[ {v}, %{lbl} ]"))
                            .collect();
                        new_lines.push(format!("{phi_reg} = phi {ty} {}", pairs.join(", ")));
                        cur_slots.insert(key, phi_reg);
                    }
                }
            }
        }

        for raw_l in &b.lines {
            let mut l = raw_l.clone();
            for (from, to) in &aliases {
                l = replace_llvm_reg(&l, from, to);
            }
            if let Some((lhs, rhs)) = l.split_once('=') {
                let reg = lhs.trim();
                let rhs_t = rhs.trim();
                if rhs_t.starts_with("alloca ") && slot_layouts.contains_key(reg) {
                    continue;
                }
                if rhs_t.starts_with("getelementptr ") {
                    if let Some(gep_res) = parse_const_gep(rhs_t, &slot_layouts, &ptr_map) {
                        let (base_slot, off) = gep_res?;
                        ptr_map.insert(reg.to_string(), (base_slot, off));
                        continue;
                    }
                }
                if let Some((load_ty, ptr_reg, align_opt)) = parse_load_inst(rhs_t) {
                    if let Some((base_slot, off)) = ptr_map.get(&ptr_reg).cloned() {
                        let layout = &slot_layouts[&base_slot];
                        let sz = llvm_scalar_size_bytes(&load_ty).unwrap_or(8);
                        if off < 0 || off + sz > layout.total_bytes {
                            return Err(format!(
                                "out-of-bounds stack load at offset {off} (+{sz}) on `{base_slot}` (size {})",
                                layout.total_bytes
                            ));
                        }
                        if let Some(al) = align_opt {
                            if !al.is_power_of_two() || (off as usize) % al != 0 {
                                return Err(format!(
                                    "misaligned stack load align {al} at offset {off} on `{base_slot}`"
                                ));
                            }
                        }
                        if let Some(val) = cur_slots.get(&(base_slot.clone(), off)) {
                            aliases.push((reg.to_string(), val.clone()));
                            continue;
                        }
                    }
                }
            }
            if let Some((store_ty, val, ptr_reg, align_opt)) = parse_store_inst(&l) {
                if let Some((base_slot, off)) = ptr_map.get(&ptr_reg).cloned() {
                    let layout = &slot_layouts[&base_slot];
                    let sz = llvm_scalar_size_bytes(&store_ty).unwrap_or(8);
                    if off < 0 || off + sz > layout.total_bytes {
                        return Err(format!(
                            "out-of-bounds stack store at offset {off} (+{sz}) on `{base_slot}` (size {})",
                            layout.total_bytes
                        ));
                    }
                    if let Some(al) = align_opt {
                        if !al.is_power_of_two() || (off as usize) % al != 0 {
                            return Err(format!(
                                "misaligned stack store align {al} at offset {off} on `{base_slot}`"
                            ));
                        }
                    }
                    cell_tys.entry((base_slot.clone(), off)).or_insert(store_ty);
                    cur_slots.insert((base_slot, off), val);
                    continue;
                }
            }
            new_lines.push(l);
        }

        out_slots[bi] = cur_slots;
        emitted_blocks[bi] = (b.label.clone(), new_lines);
    }

    // Emit blocks in topological order so `lower_llvm_fragment`'s forward-only invariant holds!
    let mut flat = Vec::new();
    for &bi in &topo {
        let (ref lbl, ref blines) = emitted_blocks[bi];
        flat.push(format!("{lbl}:"));
        flat.extend(blines.iter().cloned());
    }
    Ok(flat)
}

fn parse_align_opt(line: &str) -> Option<usize> {
    let toks: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .collect();
    for i in 0..toks.len() {
        if toks[i] == "align" {
            return toks.get(i + 1).and_then(|s| s.parse::<usize>().ok());
        }
    }
    None
}

fn parse_load_inst(rhs: &str) -> Option<(String, String, Option<usize>)> {
    let rest = rhs.strip_prefix("load ")?;
    let (ty_part, after_comma) = rest.split_once(',')?;
    let ty = ty_part.trim().to_string();
    let ptr_t = after_comma.trim().strip_prefix("ptr ")?;
    let slot = ptr_t
        .split(|c: char| c.is_whitespace() || c == ',')
        .next()?
        .to_string();
    Some((ty, slot, parse_align_opt(rhs)))
}

fn parse_store_inst(line: &str) -> Option<(String, String, String, Option<usize>)> {
    let rest = line.trim().strip_prefix("store ")?;
    let (val_part, ptr_part) = rest.split_once(',')?;
    let vparts: Vec<&str> = val_part.split_whitespace().collect();
    if vparts.len() < 2 {
        return None;
    }
    let ty = vparts[0].to_string();
    let val = vparts[1].to_string();
    let ptr_t = ptr_part.trim().strip_prefix("ptr ")?;
    let slot = ptr_t
        .split(|c: char| c.is_whitespace() || c == ',')
        .next()?
        .to_string();
    Some((ty, val, slot, parse_align_opt(line)))
}

fn parse_branch_targets(term: &str) -> Vec<String> {
    let mut out = Vec::new();
    let t = term.trim();
    if !t.starts_with("br ") {
        return out;
    }
    for tok in t.split(|c: char| c.is_whitespace() || c == ',') {
        if let Some(lbl) = tok.strip_prefix('%') {
            if t.contains(&format!("label %{lbl}")) {
                out.push(lbl.to_string());
            }
        }
    }
    out
}

fn parse_llvm_scalar_ty(w: &str) -> Option<SemTy> {
    match w {
        "i8" => Some(SemTy::Bv(8)),
        "i16" => Some(SemTy::Bv(16)),
        "i32" => Some(SemTy::Bv(32)),
        "i64" => Some(SemTy::Bv(64)),
        "i128" => Some(SemTy::Bv(128)),
        _ => None,
    }
}

fn replace_llvm_reg(line: &str, from: &str, to: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let fbytes = from.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(fbytes) {
            let after = i + fbytes.len();
            let boundary = after >= bytes.len()
                || !matches!(bytes[after], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'.');
            if boundary {
                out.push_str(to);
                i = after;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// Build an `Obligation` proving `SemGraph(Core) ⊑ SemGraph(emitted LLVM IR)` for one function.
pub fn build_core_vs_llvm_obligation(
    f: &FnDef,
    ir: &str,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
) -> Result<Obligation, String> {
    build_core_vs_llvm_obligation_for_word_bits(f, ir, prim_bodies, 64)
}

/// Target-width-aware version of [`build_core_vs_llvm_obligation`].
pub fn build_core_vs_llvm_obligation_for_word_bits(
    f: &FnDef,
    ir: &str,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    word_bits: u16,
) -> Result<Obligation, String> {
    build_core_vs_llvm_obligation_with_fns(f, ir, prim_bodies, &HashMap::new(), word_bits)
}

/// Interprocedural & tail-loop aware Core→LLVM translation-validation obligation builder.
///
/// - When `f` is self-tail-recursive, delegates to [`build_tail_loop_bisimulation_obligation`]
///   which proves inductive loop-step bisimulation for arbitrary iteration counts via LRAT.
/// - Supports same-module scalar callee inlining AND Ackermann uninterpreted-function
///   congruence when calling non-inlined / external pure scalar callees.
pub fn build_core_vs_llvm_obligation_with_fns(
    f: &FnDef,
    ir: &str,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    all_fns: &HashMap<String, &FnDef>,
    word_bits: u16,
) -> Result<Obligation, String> {
    if core_has_self_tail_call(&f.body, &f.id.0) {
        return build_tail_loop_bisimulation_obligation(f, ir, prim_bodies, all_fns, word_bits);
    }

    let mut uf_core = UfRegistry::new(f.params.len());
    let core_prog =
        lower_core_fn_with_env(f, prim_bodies, all_fns, word_bits, &mut uf_core, true)?;
    let (llvm_params, llvm_ret, llvm_frag) = extract_emitted_llvm_fn(ir, &f.id.0)
        .ok_or_else(|| format!("emitted LLVM IR has no scalar `define` for `{}`", f.id.0))?;
    if llvm_params.len() != core_prog.params.len() {
        return Err(format!(
            "arity mismatch between Core ({}) and LLVM ({}) for `{}`",
            core_prog.params.len(),
            llvm_params.len(),
            f.id.0
        ));
    }

    let mut uf_llvm_calls: Vec<(String, Vec<NodeId>, SemTy, NodeId)> = Vec::new();
    let mut next_llvm_uf_idx = uf_core.next_param_idx;
    let llvm_prog = {
        let mut cb = |g: &mut SemGraph,
                      callee: &str,
                      args: &[NodeId],
                      rty: SemTy|
         -> Result<NodeId, SemError> {
            let idx = next_llvm_uf_idx;
            next_llvm_uf_idx += 1;
            let node = g.param(idx, format!("uf_llvm_{idx}_{callee}"), rty);
            uf_llvm_calls.push((callee.to_string(), args.to_vec(), rty, node));
            Ok(node)
        };
        let mut hook: Option<UfCallHook<'_>> = Some(&mut cb);
        lower_llvm_fragment_with_uf(&llvm_frag, &llvm_params, llvm_ret, &mut hook)
            .map_err(|e| format!("lowering emitted LLVM for `{}`: {e}", f.id.0))?
    };

    let (vc, _) = core_prog.result.as_scalar().ok_or("non-scalar Core result")?;
    let (vl, pl) = llvm_prog.result.as_scalar().ok_or("non-scalar LLVM result")?;

    let mut g = core_prog.graph.clone();
    let mut pnodes: Vec<NodeId> = Vec::with_capacity(core_prog.params.len());
    for ((_, cn), lp) in core_prog.params.iter().zip(llvm_params.iter()) {
        pnodes.push(coerce_bv(&mut g, *cn, lp.ty)?);
    }
    let map_l = import_graph_preserving_extra_params(&mut g, &llvm_prog.graph, &pnodes);
    let ub_l = map_l[llvm_prog.ub_cond.0 as usize];
    let vl_m = map_l[vl.0 as usize];
    let pl_m = map_l[pl.0 as usize];

    // Merge Core and LLVM UF calls into one registry and conjoin Ackermann congruence axioms.
    let mut merged_uf = uf_core.clone();
    merged_uf.next_param_idx = next_llvm_uf_idx;
    for (callee, args, rty, res_node) in uf_llvm_calls {
        let mapped_args: Vec<NodeId> = args.into_iter().map(|a| map_l[a.0 as usize]).collect();
        let mapped_res = map_l[res_node.0 as usize];
        merged_uf.calls.push((callee, mapped_args, rty, mapped_res));
    }
    let uf_axioms = merged_uf.ackermann_axioms(&mut g);

    let not_ub = g.not(ub_l);
    let not_p = g.not(pl_m);
    let wd = g.and(not_ub, not_p);
    let vl_ty = g.ty(vl_m);
    let vc_cmp = coerce_bv(&mut g, vc, vl_ty)?;
    let eq = g.bv_eq(vc_cmp, vl_m);
    let body_goal = g.and(wd, eq);
    let goal = if merged_uf.calls.is_empty() {
        body_goal
    } else {
        g.implies(uf_axioms, body_goal)
    };
    let mut sem_params: Vec<SemParam> = core_prog.params.into_iter().map(|(p, _)| p).collect();
    sem_params.extend(merged_uf.collect_sem_params(&g));
    Ok(Obligation::new(
        format!("{}::tv(core->llvm)", f.id.0),
        format!("{CORE_TV_SEMANTICS_ID}+{LLVM_SEMANTICS_ID}"),
        ObligationKind::TranslationVal {
            fn_name: f.id.0.clone(),
            stage: "core->llvm".to_string(),
        },
        sem_params,
        g,
        goal,
    ))
}

/// Import `src` graph into `dst`, substituting parameter indices `< param_nodes.len()`
/// with `param_nodes[index]` and preserving higher-index UF parameters verbatim.
fn import_graph_preserving_extra_params(
    dst: &mut SemGraph,
    src: &SemGraph,
    param_nodes: &[NodeId],
) -> Vec<NodeId> {
    use super::sem::SemOp;
    let mut map: Vec<NodeId> = Vec::with_capacity(src.len());
    for (op, _ty) in src.nodes() {
        let m = |id: NodeId| map[id.0 as usize];
        let mapped = match op {
            SemOp::Param { index, name, ty } => {
                if *index < param_nodes.len() {
                    param_nodes[*index]
                } else {
                    dst.param(*index, name.clone(), *ty)
                }
            }
            SemOp::BoolConst(b) => dst.bool_const(*b),
            SemOp::BvConst { width, bits } => dst.bv_const(*width, *bits),
            SemOp::FpConst { fmt, bits } => dst.fp_const(*fmt, *bits),
            SemOp::Not(a) => dst.not(m(*a)),
            SemOp::And(a, b) => dst.and(m(*a), m(*b)),
            SemOp::Or(a, b) => dst.or(m(*a), m(*b)),
            SemOp::Xor(a, b) => dst.xor(m(*a), m(*b)),
            SemOp::Iff(a, b) => dst.iff(m(*a), m(*b)),
            SemOp::Implies(a, b) => dst.implies(m(*a), m(*b)),
            SemOp::Ite(c, t, e) => dst.ite(m(*c), m(*t), m(*e)),
            SemOp::BvNot(a) => dst.bv_not(m(*a)),
            SemOp::BvNeg(a) => dst.bv_neg(m(*a)),
            SemOp::BvExtract { x, hi, lo } => dst.bv_extract(m(*x), *hi, *lo),
            SemOp::BvConcat(a, b) => dst.bv_concat(m(*a), m(*b)),
            SemOp::BvZExt { x, to } => dst.bv_zext(m(*x), *to),
            SemOp::BvSExt { x, to } => dst.bv_sext(m(*x), *to),
            SemOp::BvBitReverse(a) => dst.bv_bitreverse(m(*a)),
            SemOp::BvBSwap(a) => dst.bv_bswap(m(*a)),
            SemOp::BvCtPop(a) => dst.bv_ctpop(m(*a)),
            SemOp::BvCtlz(a) => dst.bv_ctlz(m(*a)),
            SemOp::BvCttz(a) => dst.bv_cttz(m(*a)),
            SemOp::BvAdd(a, b) => dst.bv_add(m(*a), m(*b)),
            SemOp::BvSub(a, b) => dst.bv_sub(m(*a), m(*b)),
            SemOp::BvMul(a, b) => dst.bv_mul(m(*a), m(*b)),
            SemOp::BvUDiv(a, b) => dst.bv_udiv(m(*a), m(*b)),
            SemOp::BvURem(a, b) => dst.bv_urem(m(*a), m(*b)),
            SemOp::BvSDiv(a, b) => dst.bv_sdiv(m(*a), m(*b)),
            SemOp::BvSRem(a, b) => dst.bv_srem(m(*a), m(*b)),
            SemOp::BvAnd(a, b) => dst.bv_and(m(*a), m(*b)),
            SemOp::BvOr(a, b) => dst.bv_or(m(*a), m(*b)),
            SemOp::BvXor(a, b) => dst.bv_xor(m(*a), m(*b)),
            SemOp::BvShl(a, b) => dst.bv_shl(m(*a), m(*b)),
            SemOp::BvLShr(a, b) => dst.bv_lshr(m(*a), m(*b)),
            SemOp::BvAShr(a, b) => dst.bv_ashr(m(*a), m(*b)),
            SemOp::BvFshl(a, b, s) => dst.bv_fshl(m(*a), m(*b), m(*s)),
            SemOp::BvFshr(a, b, s) => dst.bv_fshr(m(*a), m(*b), m(*s)),
            SemOp::BvEq(a, b) => dst.bv_eq(m(*a), m(*b)),
            SemOp::BvULt(a, b) => dst.bv_ult(m(*a), m(*b)),
            SemOp::BvULe(a, b) => dst.bv_ule(m(*a), m(*b)),
            SemOp::BvSLt(a, b) => dst.bv_slt(m(*a), m(*b)),
            SemOp::BvSLe(a, b) => dst.bv_sle(m(*a), m(*b)),
            SemOp::FpNeg(a) => dst.fp_neg(m(*a)),
            SemOp::FpAdd(a, b) => dst.fp_add(m(*a), m(*b)),
            SemOp::FpSub(a, b) => dst.fp_sub(m(*a), m(*b)),
            SemOp::FpMul(a, b) => dst.fp_mul(m(*a), m(*b)),
            SemOp::FpDiv(a, b) => dst.fp_div(m(*a), m(*b)),
            SemOp::FpCmp(p, a, b) => dst.fp_cmp(*p, m(*a), m(*b)),
            SemOp::FpToBvBits(a) => dst.fp_to_bv_bits(m(*a)),
            SemOp::BvBitsToFp(f, a) => dst.bv_bits_to_fp(*f, m(*a)),
            SemOp::BvToFpSigned(f, a) => dst.bv_to_fp_signed(*f, m(*a)),
            SemOp::BvToFpUnsigned(f, a) => dst.bv_to_fp_unsigned(*f, m(*a)),
            SemOp::FpToBvSigned(w, a) => dst.fp_to_bv_signed(*w, m(*a)),
            SemOp::FpToBvUnsigned(w, a) => dst.fp_to_bv_unsigned(*w, m(*a)),
            SemOp::FpToFp(f, a) => dst.fp_to_fp(*f, m(*a)),
            SemOp::FpUnary(u, a) => dst.fp_unary(*u, m(*a)),
            SemOp::FpCopySign(a, b) => dst.fp_copysign(m(*a), m(*b)),
            SemOp::FpFitsSigned(w, a) => dst.fp_fits_signed(*w, m(*a)),
            SemOp::FpFitsUnsigned(w, a) => dst.fp_fits_unsigned(*w, m(*a)),
            SemOp::FpIsNaN(a) => dst.fp_is_nan(m(*a)),
            SemOp::FpIsFinite(a) => dst.fp_is_finite(m(*a)),
        };
        map.push(mapped);
    }
    map
}

// =========================================================================
// Tail-Recursive Loop-Step Bisimulation (Inductive Loop Translation Validation)
// =========================================================================

fn core_has_self_tail_call(expr: &Core, self_id: &str) -> bool {
    match expr {
        Core::Let(_, _, _, body) => core_has_self_tail_call(body, self_id),
        Core::If(_, th, el) => {
            core_has_self_tail_call(&th.body, self_id) || core_has_self_tail_call(&el.body, self_id)
        }
        Core::App(head, _) => matches!(head.as_ref(), Core::Global(gid, _) if gid.0 == self_id),
        _ => false,
    }
}

struct CoreStep {
    done: NodeId,
    ret_val: NodeId,
    next_args: Vec<NodeId>,
}

#[allow(clippy::too_many_arguments)]
fn lower_core_tail_step(
    g: &mut SemGraph,
    expr: &Core,
    env: &HashMap<LocalId, NodeId>,
    self_id: &str,
    ret_ty: SemTy,
    param_tys: &[SemTy],
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    all_fns: &HashMap<String, &FnDef>,
    word_bits: u16,
    uf: &mut UfRegistry,
) -> Result<CoreStep, String> {
    let mut visiting = HashSet::new();
    visiting.insert(self_id.to_string());
    match expr {
        Core::Let(lid, _, rhs, body) => {
            let rv = lower_core_expr_ext(
                g, rhs, env, prim_bodies, all_fns, &mut visiting, word_bits, uf, true,
            )?;
            let mut next = env.clone();
            next.insert(*lid, rv);
            lower_core_tail_step(
                g, body, &next, self_id, ret_ty, param_tys, prim_bodies, all_fns, word_bits, uf,
            )
        }
        Core::If(cond, th, el) => {
            let cv = lower_core_expr_ext(
                g, cond, env, prim_bodies, all_fns, &mut visiting, word_bits, uf, true,
            )?;
            let cb = coerce_bv(g, cv, SemTy::Bool)?;
            let ts = lower_core_tail_step(
                g, &th.body, env, self_id, ret_ty, param_tys, prim_bodies, all_fns, word_bits, uf,
            )?;
            let es = lower_core_tail_step(
                g, &el.body, env, self_id, ret_ty, param_tys, prim_bodies, all_fns, word_bits, uf,
            )?;
            let done = g.ite(cb, ts.done, es.done);
            let ret_val = g.ite(cb, ts.ret_val, es.ret_val);
            let mut next_args = Vec::with_capacity(param_tys.len());
            for (t_a, e_a) in ts.next_args.into_iter().zip(es.next_args.into_iter()) {
                next_args.push(g.ite(cb, t_a, e_a));
            }
            Ok(CoreStep { done, ret_val, next_args })
        }
        Core::App(head, args)
            if matches!(head.as_ref(), Core::Global(gid, _) if gid.0 == self_id) =>
        {
            if args.len() != param_tys.len() {
                return Err(format!("self-tail-call arity mismatch on `{self_id}`"));
            }
            let mut next_args = Vec::with_capacity(args.len());
            for (a, &pty) in args.iter().zip(param_tys.iter()) {
                let v = lower_core_expr_ext(
                    g, a, env, prim_bodies, all_fns, &mut visiting, word_bits, uf, true,
                )?;
                next_args.push(coerce_bv(g, v, pty)?);
            }
            let ret_zero = match ret_ty {
                SemTy::Bv(w) => g.bv_const(w, 0),
                _ => g.bv_const(64, 0),
            };
            Ok(CoreStep {
                done: g.bool_const(false),
                ret_val: ret_zero,
                next_args,
            })
        }
        other => {
            let rv = lower_core_expr_ext(
                g, other, env, prim_bodies, all_fns, &mut visiting, word_bits, uf, true,
            )?;
            let coerced = coerce_bv(g, rv, ret_ty)?;
            let next_zeros = param_tys
                .iter()
                .map(|&pty| match pty {
                    SemTy::Bv(w) => g.bv_const(w, 0),
                    _ => g.bv_const(64, 0),
                })
                .collect();
            Ok(CoreStep {
                done: g.bool_const(true),
                ret_val: coerced,
                next_args: next_zeros,
            })
        }
    }
}

fn sem_ty_to_llvm_str(ty: SemTy) -> String {
    match ty {
        SemTy::Bool => "i1".to_string(),
        SemTy::Bv(w) => format!("i{w}"),
        SemTy::Fp(super::sem::FpFmt::F32) => "float".to_string(),
        SemTy::Fp(super::sem::FpFmt::F64) => "double".to_string(),
    }
}

/// Cut the unique loop backedge `br label %header` in a tail-recursive LLVM function into
/// an acyclic single-step function returning `{ i1 done, <ret_ty> ret, <p0_ty> next0, ... }`.
/// Supports both alloca-slot parameter loops (Cinch seed codegen) and SSA `phi`-header loops.
fn extract_llvm_tail_loop_step(
    ir: &str,
    unit_name: &str,
) -> Result<(Vec<SemParam>, SemTy, String), String> {
    let (params, ret_ty, raw_lines) = extract_raw_llvm_fn(ir, unit_name)
        .ok_or_else(|| format!("emitted LLVM IR has no scalar `define` for `{unit_name}`"))?;
    let mut counter = 0usize;
    let inlined = inline_direct_calls(ir, raw_lines, &mut counter, 0)?;

    // Parse blocks to locate backedge `br label %header` and entry parameter slots/phis.
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    let mut cur_lbl = "entry".to_string();
    let mut cur_ls: Vec<String> = Vec::new();
    let mut saw_lbl = false;
    for l in &inlined {
        let t = l.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(lbl) = t.strip_suffix(':') {
            if !lbl.contains(char::is_whitespace) {
                if saw_lbl || !cur_ls.is_empty() {
                    blocks.push((cur_lbl, std::mem::take(&mut cur_ls)));
                }
                cur_lbl = lbl.to_string();
                saw_lbl = true;
                continue;
            }
        }
        cur_ls.push(t.to_string());
    }
    blocks.push((cur_lbl, cur_ls));

    let lbl_idx: HashMap<String, usize> = blocks
        .iter()
        .enumerate()
        .map(|(i, (l, _))| (l.clone(), i))
        .collect();

    // Find backedge `(latch_idx -> header_idx)` where `header_idx <= latch_idx`.
    let mut backedge_header: Option<String> = None;
    for (bi, (_, ls)) in blocks.iter().enumerate() {
        if let Some(last) = ls.last() {
            if let Some(target) = last.trim().strip_prefix("br label %") {
                let target = target.trim();
                if let Some(&hi) = lbl_idx.get(target) {
                    if hi <= bi {
                        backedge_header = Some(target.to_string());
                        break;
                    }
                }
            }
        }
    }
    let header_lbl = backedge_header
        .ok_or_else(|| format!("no tail-loop backedge found in emitted LLVM for `{unit_name}`"))?;

    // Build step-tuple type `{ i1, <ret_ty>, <p0_ty>, ... }`.
    let ret_ty_s = sem_ty_to_llvm_str(ret_ty);
    let mut tuple_tys = vec!["i1".to_string(), ret_ty_s.clone()];
    for p in &params {
        tuple_tys.push(sem_ty_to_llvm_str(p.ty));
    }
    let step_struct_ty = format!("{{ {} }}", tuple_tys.join(", "));

    // Detect entry parameter alloca slots: `store <ty> %i, ptr %slot_i` in entry block.
    let mut param_slots: Vec<Option<String>> = vec![None; params.len()];
    if let Some((_, entry_ls)) = blocks.first() {
        for l in entry_ls {
            if let Some((_, val, slot, _)) = parse_store_inst(l) {
                if let Some(num) = val.strip_prefix('%').and_then(|s| s.parse::<usize>().ok()) {
                    if num < params.len() {
                        param_slots[num] = Some(slot);
                    }
                }
            }
        }
    }

    // Also check SSA `phi`-header loops: `header: %v = phi <ty> [ %i, %entry ], [ %next, %latch ]`.
    let mut phi_backedge_vals: Vec<Option<(String, String)>> = vec![None; params.len()];
    let mut header_phi_replacements: Vec<(String, String)> = Vec::new();
    let header_pos = lbl_idx[&header_lbl];
    for l in &blocks[header_pos].1 {
        if let Some((lhs, rhs)) = l.split_once('=') {
            let reg = lhs.trim();
            let rhs_t = rhs.trim();
            if rhs_t.starts_with("phi ") {
                if let Some(lb) = rhs_t.find('[') {
                    let pairs_str = &rhs_t[lb..];
                    let mut entry_val: Option<String> = None;
                    let mut latch_val: Option<String> = None;
                    for seg in pairs_str.split('[').skip(1) {
                        if let Some(rb) = seg.find(']') {
                            if let Some((v, bl)) = seg[..rb].rsplit_once(',') {
                                let v = v.trim().to_string();
                                let bl = bl.trim().trim_start_matches('%');
                                if let Some(&bi) = lbl_idx.get(bl) {
                                    if bi < header_pos {
                                        entry_val = Some(v);
                                    } else {
                                        latch_val = Some(v);
                                    }
                                }
                            }
                        }
                    }
                    if let (Some(ev), Some(lv)) = (entry_val, latch_val) {
                        if let Some(num) =
                            ev.strip_prefix('%').and_then(|s| s.parse::<usize>().ok())
                        {
                            if num < params.len() {
                                phi_backedge_vals[num] = Some((ev.clone(), lv));
                                header_phi_replacements.push((reg.to_string(), ev));
                            }
                        }
                    }
                }
            }
        }
    }

    let use_alloca_slots = param_slots.iter().all(|s| s.is_some());
    let use_phi_loop = !use_alloca_slots && phi_backedge_vals.iter().all(|s| s.is_some());
    if !use_alloca_slots && !use_phi_loop {
        return Err(format!(
            "cannot map loop-carried parameter state in `{unit_name}`"
        ));
    }

    // Rewrite blocks into an acyclic single-step fragment returning `step_struct_ty`:
    // - Each `ret <ty> <val>` becomes building `{ 1, <val>, 0, ... }` and returning it.
    // - Each backedge `br label %header` becomes loading/packing `{ 0, 0, next0, next1, ... }` and returning it.
    let mut rewritten: Vec<String> = Vec::new();
    let mut tmp_id = 0usize;
    for (bi, (blbl, blines)) in blocks.iter().enumerate() {
        rewritten.push(format!("{blbl}:"));
        for raw_l in blines {
            let mut l = raw_l.clone();
            if use_phi_loop && bi == header_pos && l.contains("= phi ") {
                if let Some((lhs, _)) = l.split_once('=') {
                    let reg = lhs.trim();
                    if header_phi_replacements.iter().any(|(r, _)| r == reg) {
                        continue;
                    }
                }
            }
            for (from, to) in &header_phi_replacements {
                l = replace_llvm_reg(&l, from, to);
            }
            if let Some(rest) = l.strip_prefix("ret ") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 {
                    let rv = parts[1];
                    let t0 = format!("%__step_r{tmp_id}_0");
                    let t1 = format!("%__step_r{tmp_id}_1");
                    tmp_id += 1;
                    rewritten.push(format!(
                        "{t0} = insertvalue {step_struct_ty} zeroinitializer, i1 1, 0"
                    ));
                    rewritten.push(format!(
                        "{t1} = insertvalue {step_struct_ty} {t0}, {ret_ty_s} {rv}, 1"
                    ));
                    rewritten.push(format!("ret {step_struct_ty} {t1}"));
                    continue;
                }
            }
            if let Some(target) = l.strip_prefix("br label %") {
                if target.trim() == header_lbl && header_pos <= bi {
                    let mut cur = format!("%__step_c{tmp_id}_0");
                    tmp_id += 1;
                    rewritten.push(format!(
                        "{cur} = insertvalue {step_struct_ty} zeroinitializer, i1 0, 0"
                    ));
                    for (pi, p) in params.iter().enumerate() {
                        let pty_s = sem_ty_to_llvm_str(p.ty);
                        let next_val = if use_alloca_slots {
                            let slot = param_slots[pi].as_ref().unwrap();
                            let ld = format!("%__step_ld{tmp_id}");
                            tmp_id += 1;
                            rewritten.push(format!("{ld} = load {pty_s}, ptr {slot}"));
                            ld
                        } else {
                            let (_, lv) = phi_backedge_vals[pi].as_ref().unwrap();
                            let mut lv_r = lv.clone();
                            for (from, to) in &header_phi_replacements {
                                lv_r = replace_llvm_reg(&lv_r, from, to);
                            }
                            lv_r
                        };
                        let nxt = format!("%__step_c{tmp_id}_{}", pi + 2);
                        tmp_id += 1;
                        rewritten.push(format!(
                            "{nxt} = insertvalue {step_struct_ty} {cur}, {pty_s} {next_val}, {}",
                            pi + 2
                        ));
                        cur = nxt;
                    }
                    rewritten.push(format!("ret {step_struct_ty} {cur}"));
                    continue;
                }
            }
            rewritten.push(l);
        }
    }

    let ssa = acyclic_mem2reg(rewritten)?;
    Ok((params, ret_ty, ssa.join("\n")))
}

/// Build an inductive loop-step bisimulation `Obligation` for a self-tail-recursive scalar
/// function `f`, proving that one iteration of the emitted LLVM loop is UB-free, poison-free,
/// and bit-for-bit identical (termination flag, return value, and next loop-carried state) to
/// `Core` — soundly covering arbitrary iteration counts `0..∞` via kernel-checked LRAT!
pub fn build_tail_loop_bisimulation_obligation(
    f: &FnDef,
    ir: &str,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
    all_fns: &HashMap<String, &FnDef>,
    word_bits: u16,
) -> Result<Obligation, String> {
    let mut g = SemGraph::new();
    let mut params = Vec::with_capacity(f.params.len());
    let mut param_tys = Vec::with_capacity(f.params.len());
    let mut env: HashMap<LocalId, NodeId> = HashMap::new();
    for (idx, pname) in f.params.iter().enumerate() {
        let cls = f.repr.params.get(idx).cloned().unwrap_or(Class::Raw);
        let ty = class_to_sem_ty(&cls)
            .ok_or_else(|| format!("non-scalar parameter class `{cls:?}` on `{}`", f.id.0))?;
        let node = g.param(idx, pname.clone(), ty);
        params.push((
            SemParam { index: idx, name: pname.clone(), ty, signed: true },
            node,
        ));
        param_tys.push(ty);
        env.insert(LocalId(idx), node);
    }
    let ret_ty = f
        .repr
        .result
        .as_ref()
        .and_then(class_to_sem_ty)
        .unwrap_or(SemTy::Bv(64));

    let mut uf = UfRegistry::new(f.params.len());
    let core_step = lower_core_tail_step(
        &mut g,
        &f.body,
        &env,
        &f.id.0,
        ret_ty,
        &param_tys,
        prim_bodies,
        all_fns,
        word_bits,
        &mut uf,
    )?;

    let (llvm_params, llvm_ret, step_frag) = extract_llvm_tail_loop_step(ir, &f.id.0)?;
    if llvm_params.len() != params.len() {
        return Err(format!("tail-loop arity mismatch on `{}`", f.id.0));
    }
    let llvm_prog = lower_llvm_fragment(&step_frag, &llvm_params, llvm_ret)
        .map_err(|e| format!("lowering tail-loop step LLVM for `{}`: {e}", f.id.0))?;
    let SemVal::Tuple(ref fields) = llvm_prog.result else {
        return Err(format!("tail-loop step did not produce tuple on `{}`", f.id.0));
    };
    if fields.len() != 2 + params.len() {
        return Err(format!("tail-loop step tuple arity mismatch on `{}`", f.id.0));
    }

    let mut pnodes: Vec<NodeId> = Vec::with_capacity(params.len());
    for ((_, cn), lp) in params.iter().zip(llvm_params.iter()) {
        pnodes.push(coerce_bv(&mut g, *cn, lp.ty)?);
    }
    let map_l = import_graph_preserving_extra_params(&mut g, &llvm_prog.graph, &pnodes);
    let ub_l = map_l[llvm_prog.ub_cond.0 as usize];

    let (done_l_raw, done_lp) = fields[0].as_scalar().ok_or("non-scalar step done")?;
    let (ret_l_raw, ret_lp) = fields[1].as_scalar().ok_or("non-scalar step ret")?;
    let done_l = coerce_bv(&mut g, map_l[done_l_raw.0 as usize], SemTy::Bool)?;
    let ret_l = map_l[ret_l_raw.0 as usize];
    let mut any_poison = g.or(map_l[done_lp.0 as usize], map_l[ret_lp.0 as usize]);

    let mut next_eq = g.bool_const(true);
    for (i, &nc) in core_step.next_args.iter().enumerate() {
        let (nl_raw, nl_p) = fields[2 + i].as_scalar().ok_or("non-scalar step next")?;
        let nl = map_l[nl_raw.0 as usize];
        any_poison = g.or(any_poison, map_l[nl_p.0 as usize]);
        let nl_ty = g.ty(nl);
        let nc_c = coerce_bv(&mut g, nc, nl_ty)?;
        let eq_i = g.bv_eq(nc_c, nl);
        next_eq = g.and(next_eq, eq_i);
    }

    let same_done = g.iff(core_step.done, done_l);
    let ret_l_ty = g.ty(ret_l);
    let ret_c_cmp = coerce_bv(&mut g, core_step.ret_val, ret_l_ty)?;
    let same_ret = g.bv_eq(ret_c_cmp, ret_l);
    let when_done = g.implies(core_step.done, same_ret);
    let not_done = g.not(core_step.done);
    let when_cont = g.implies(not_done, next_eq);

    let not_ub = g.not(ub_l);
    let not_p = g.not(any_poison);
    let wd = g.and(not_ub, not_p);
    let branches_ok = g.and(when_done, when_cont);
    let step_ok = g.and(same_done, branches_ok);
    let goal = g.and(wd, step_ok);

    let sem_params: Vec<SemParam> = params.into_iter().map(|(p, _)| p).collect();
    Ok(Obligation::new(
        format!("{}::tv(core->llvm:loop-step)", f.id.0),
        format!("{CORE_TV_SEMANTICS_ID}+{LLVM_SEMANTICS_ID}"),
        ObligationKind::TranslationVal {
            fn_name: f.id.0.clone(),
            stage: "core->llvm(loop-step)".to_string(),
        },
        sem_params,
        g,
        goal,
    ))
}

// =========================================================================
// Aggregate / `#[repr(C)]` / `FixedArray` Symbolic Memory Equivalence
// =========================================================================

/// Prove two LLVM fragments using local `#[repr(C)]` / `#[repr(value)]` struct or
/// `FixedArray` stack `alloca`/`getelementptr`/`load`/`store` operations are bounds-safe,
/// alignment-safe, UB/poison-free, and bit-for-bit identical via LRAT.
pub fn validate_llvm_memory_equiv(
    fn_name: &str,
    frag_before: &str,
    frag_after: &str,
    params: &[SemParam],
    ret_ty: SemTy,
) -> Result<Obligation, String> {
    let before_lines: Vec<String> = frag_before.lines().map(|s| s.trim().to_string()).collect();
    let after_lines: Vec<String> = frag_after.lines().map(|s| s.trim().to_string()).collect();
    let ssa_before = acyclic_mem2reg(before_lines)?.join("\n");
    let ssa_after = acyclic_mem2reg(after_lines)?.join("\n");

    let prog_a = lower_llvm_fragment(&ssa_before, params, ret_ty)
        .map_err(|e| format!("memory TV lhs `{fn_name}`: {e}"))?;
    let prog_b = lower_llvm_fragment(&ssa_after, params, ret_ty)
        .map_err(|e| format!("memory TV rhs `{fn_name}`: {e}"))?;

    let (va, pa) = prog_a.result.as_scalar().ok_or("non-scalar memory TV lhs")?;
    let (vb, pb) = prog_b.result.as_scalar().ok_or("non-scalar memory TV rhs")?;
    let mut g = prog_a.graph.clone();
    let pnodes: Vec<NodeId> = prog_a.params.iter().map(|(_, n)| *n).collect();
    let map_b = g.import_graph(&prog_b.graph, &pnodes);

    let ub_a = prog_a.ub_cond;
    let ub_b = map_b[prog_b.ub_cond.0 as usize];
    let vb_m = map_b[vb.0 as usize];
    let pb_m = map_b[pb.0 as usize];

    let not_ub_a = g.not(ub_a);
    let not_ub_b = g.not(ub_b);
    let not_ub = g.and(not_ub_a, not_ub_b);
    let not_pa = g.not(pa);
    let not_pb = g.not(pb_m);
    let not_p = g.and(not_pa, not_pb);
    let wd = g.and(not_ub, not_p);
    let eq = g.bv_eq(va, vb_m);
    let goal = g.and(wd, eq);
    Ok(Obligation::new(
        format!("{fn_name}::tv(mem-agg)"),
        LLVM_SEMANTICS_ID,
        ObligationKind::TranslationVal {
            fn_name: fn_name.to_string(),
            stage: "llvm-mem-agg".to_string(),
        },
        params.to_vec(),
        g,
        goal,
    ))
}

// =========================================================================
// Perceus RC / Drop-Insertion Safety Validator (`stage: "rc-safety"`)
// =========================================================================

fn classify_rc_call(line: &str) -> Option<(&'static str, String)> {
    let trimmed = line.trim();
    if !trimmed.contains("call ") {
        return None;
    }
    let kind = if trimmed.contains("@cinch.rc.dup.obj(")
        || trimmed.contains("@\"cinch.rc.dup.obj\"(")
        || trimmed.contains("@cinch.rc.dup.tobj(")
        || trimmed.contains("@\"cinch.rc.dup.tobj\"(")
        || trimmed.contains("@cinch_dup(")
        || trimmed.contains("@cinch_dup_mt(")
    {
        "dup"
    } else if trimmed.contains("@cinch.rc.drop.obj(")
        || trimmed.contains("@\"cinch.rc.drop.obj\"(")
        || trimmed.contains("@cinch.rc.drop.tobj(")
        || trimmed.contains("@\"cinch.rc.drop.tobj\"(")
        || trimmed.contains("@cinch_drop(")
        || trimmed.contains("@cinch_drop_slow(")
        || trimmed.contains("@cinch_drop_mt(")
        || trimmed.contains("@cinch_free(")
        || trimmed.contains("@cinch_frame_release(")
    {
        "drop"
    } else {
        return None;
    };
    let lparen = trimmed.find('(')?;
    let rparen = trimmed.rfind(')')?;
    let arg_str = trimmed[lparen + 1..rparen].trim();
    let first_arg = arg_str.split(',').next()?.trim();
    let reg = first_arg.split_whitespace().last()?.to_string();
    Some((kind, reg))
}

/// Audit a single emitted LLVM function for Perceus RC / drop-insertion safety:
/// verifies balanced `@cinch.rc.dup.*` / `@cinch.rc.drop.*` / `@cinch_free` /
/// `@cinch_frame_release` ownership across CFG paths, rejecting double-free,
/// use-after-free, unbalanced branch drops, and RC leaks.
pub fn validate_fn_rc_safety(fn_name: &str, ir: &str) -> Result<TvFunctionReport, String> {
    let needle = format!("@\"{fn_name}\"(");
    let mut lines = ir.lines();
    let header = lines
        .find(|l| l.starts_with("define ") && l.contains(&needle))
        .ok_or_else(|| format!("emitted LLVM IR has no `define` for `{fn_name}`"))?;

    let lparen = header.find('(').unwrap_or(0);
    let rparen = header.rfind(')').unwrap_or(header.len());
    let mut param_regs: Vec<String> = Vec::new();
    if lparen < rparen {
        for item in header[lparen + 1..rparen].split(',') {
            if let Some(p) = item.split_whitespace().last() {
                if p.starts_with('%') {
                    param_regs.push(p.to_string());
                }
            }
        }
    }

    // Parse blocks inside the function body.
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    let mut cur_lbl = "entry".to_string();
    let mut cur_ls: Vec<String> = Vec::new();
    let mut saw_lbl = false;
    for raw in lines {
        let t = raw.trim();
        if t == "}" {
            break;
        }
        if t.is_empty() || t.starts_with(';') {
            continue;
        }
        if let Some(lbl) = t.strip_suffix(':') {
            if !lbl.contains(char::is_whitespace) {
                if saw_lbl || !cur_ls.is_empty() {
                    blocks.push((cur_lbl, std::mem::take(&mut cur_ls)));
                }
                cur_lbl = lbl.to_string();
                saw_lbl = true;
                continue;
            }
        }
        cur_ls.push(t.to_string());
    }
    blocks.push((cur_lbl, cur_ls));

    // Check if the function uses any Perceus RC ops.
    let mut total_dups = 0usize;
    let mut total_drops = 0usize;
    for (_, ls) in &blocks {
        for l in ls {
            if let Some((k, _)) = classify_rc_call(l) {
                if k == "dup" {
                    total_dups += 1;
                } else {
                    total_drops += 1;
                }
            }
        }
    }
    if total_dups == 0 && total_drops == 0 {
        return Err("no Perceus RC operations in function".to_string());
    }

    // Build CFG predecessor edges and topological order (skipping backedges for initial pass).
    let lbl_idx: HashMap<String, usize> = blocks
        .iter()
        .enumerate()
        .map(|(i, (l, _))| (l.clone(), i))
        .collect();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); blocks.len()];
    for (bi, (_, ls)) in blocks.iter().enumerate() {
        if let Some(term) = ls.last() {
            for target in parse_branch_targets(term) {
                if let Some(&ti) = lbl_idx.get(&target) {
                    if ti > bi {
                        preds[ti].push(bi);
                    }
                }
            }
        }
    }

    // Abstract state per block exit using an ownership-count interval `[lo, hi]`
    // per canonical handle:
    // - `lo` is the minimum remaining local refcount (assuming every non-RC call consumes 1 ref),
    // - `hi` is the maximum remaining local refcount (assuming every non-RC call borrows 0 refs).
    // This soundly catches definite double-free (`hi <= 0` on drop), definite use-after-free
    // (`hi <= 0` on dup/use/ret), definite RC leaks (`lo > 0` at ret), and definite unbalanced
    // branch drops (`hi1 == 0 && lo2 > 0`) with zero false positives across borrowed vs owned calls!
    #[derive(Clone, Default)]
    struct RcState {
        reg_handle: HashMap<String, String>,
        slot_handle: HashMap<String, String>,
        rc: HashMap<String, (i32, i32)>,
        touched: HashSet<String>,
    }

    let mut init = RcState::default();
    for p in &param_regs {
        init.reg_handle.insert(p.clone(), p.clone());
        init.rc.insert(p.clone(), (1, 1));
    }

    // Collect local stack `alloca` registers so we distinguish local slot aliases
    // (`store %h, ptr %alloca_slot`) from heap field stores (`store %h, ptr %gep` -> ownership transfer).
    let mut local_allocas: HashSet<String> = HashSet::new();
    for (_, ls) in &blocks {
        for l in ls {
            if let Some((lhs, rhs)) = l.split_once('=') {
                if rhs.trim().starts_with("alloca ") {
                    local_allocas.insert(lhs.trim().to_string());
                }
            }
        }
    }

    let mut exit_states: Vec<Option<RcState>> = vec![None; blocks.len()];
    let mut violations: Vec<String> = Vec::new();

    for bi in 0..blocks.len() {
        let mut st = if bi == 0 {
            init.clone()
        } else {
            let mut merged: Option<RcState> = None;
            for &pi in &preds[bi] {
                if let Some(ref ps) = exit_states[pi] {
                    match &mut merged {
                        None => merged = Some(ps.clone()),
                        Some(m) => {
                            for h in m.touched.union(&ps.touched) {
                                let (lo1, hi1) = m.rc.get(h).copied().unwrap_or((1, 1));
                                let (lo2, hi2) = ps.rc.get(h).copied().unwrap_or((1, 1));
                                if (hi1 == 0 && lo2 > 0) || (hi2 == 0 && lo1 > 0) {
                                    violations.push(format!(
                                        "block `{}`: unbalanced branch RC on `{h}` ([{lo1},{hi1}] vs [{lo2},{hi2}])",
                                        blocks[bi].0
                                    ));
                                }
                                m.rc.insert(h.clone(), (lo1.min(lo2), hi1.max(hi2)));
                            }
                            for (k, v) in &ps.reg_handle {
                                m.reg_handle.entry(k.clone()).or_insert_with(|| v.clone());
                            }
                            for (k, v) in &ps.slot_handle {
                                m.slot_handle.entry(k.clone()).or_insert_with(|| v.clone());
                            }
                            for h in &ps.touched {
                                m.touched.insert(h.clone());
                            }
                        }
                    }
                }
            }
            merged.unwrap_or_else(|| init.clone())
        };

        for line in &blocks[bi].1 {
            if let Some((kind, arg_reg)) = classify_rc_call(line) {
                let was_tracked = st.reg_handle.contains_key(&arg_reg);
                let handle = st
                    .reg_handle
                    .get(&arg_reg)
                    .cloned()
                    .unwrap_or_else(|| {
                        st.reg_handle.insert(arg_reg.clone(), arg_reg.clone());
                        arg_reg.clone()
                    });
                st.touched.insert(handle.clone());
                // An untracked register (e.g. loaded from a static global dictionary or borrowed field)
                // holds 0 local refs prior to `dup` (`(0, 1)`), or 1 ref prior to `drop` (`(1, 1)`).
                let init_rc = if !was_tracked && kind == "dup" { (0, 1) } else { (1, 1) };
                let (lo, hi) = st.rc.entry(handle.clone()).or_insert(init_rc);
                if kind == "dup" {
                    if *hi <= 0 {
                        violations.push(format!(
                            "block `{}`: use-after-free on `dup` of `{handle}` (rc=[{},{}])",
                            blocks[bi].0, *lo, *hi
                        ));
                    } else {
                        *lo += 1;
                        *hi += 1;
                    }
                } else if *hi <= 0 {
                    violations.push(format!(
                        "block `{}`: double-free / over-release on `drop` of `{handle}` (rc=[{},{}])",
                        blocks[bi].0, *lo, *hi
                    ));
                } else {
                    *lo = (*lo - 1).max(0);
                    *hi -= 1;
                }
                continue;
            }

            if let Some((_, val, slot, _)) = parse_store_inst(line) {
                if let Some(h) = st.reg_handle.get(&val).cloned() {
                    if st.touched.contains(&h) && st.rc.get(&h).map(|(_, hi)| *hi).unwrap_or(1) <= 0 {
                        violations.push(format!(
                            "block `{}`: use-after-free storing dropped handle `{h}` into `{slot}`",
                            blocks[bi].0
                        ));
                    }
                    if local_allocas.contains(&slot) {
                        st.slot_handle.insert(slot, h);
                    } else if st.touched.contains(&h) {
                        // Storing into a heap object / closure field transfers 1 ownership unit.
                        let (lo, _) = st.rc.entry(h).or_insert((1, 1));
                        *lo = (*lo - 1).max(0);
                    }
                }
                continue;
            }

            // Void non-RC call (`call void @foo(...)`): check args for use-after-free and allow ownership transfer.
            if !line.contains('=') && line.contains("call ") {
                if let (Some(lp), Some(rp)) = (line.find('('), line.rfind(')')) {
                    for item in line[lp + 1..rp].split(',') {
                        if let Some(arg) = item.split_whitespace().last() {
                            if let Some(h) = st.reg_handle.get(arg).cloned() {
                                if st.touched.contains(&h) {
                                    let (lo, hi) = st.rc.entry(h.clone()).or_insert((1, 1));
                                    if *hi <= 0 {
                                        violations.push(format!(
                                            "block `{}`: use-after-free passing dropped handle `{h}` to call",
                                            blocks[bi].0
                                        ));
                                    } else {
                                        *lo = (*lo - 1).max(0);
                                    }
                                }
                            }
                        }
                    }
                }
                continue;
            }

            if let Some(rest) = line.strip_prefix("ret ") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 {
                    let rreg = parts[1];
                    if let Some(h) = st.reg_handle.get(rreg).cloned() {
                        if st.touched.contains(&h) {
                            let (lo, hi) = st.rc.entry(h.clone()).or_insert((1, 1));
                            if *hi <= 0 {
                                violations.push(format!(
                                    "block `{}`: use-after-free returning dropped handle `{h}`",
                                    blocks[bi].0
                                ));
                            } else {
                                *lo = (*lo - 1).max(0);
                                *hi -= 1;
                            }
                        }
                    }
                }
                for h in &st.touched {
                    let (lo, _) = st.rc.get(h).copied().unwrap_or((0, 0));
                    if lo > 0 {
                        violations.push(format!(
                            "block `{}`: RC leak on `{h}` at `ret` (remaining min rc={lo})",
                            blocks[bi].0
                        ));
                    }
                }
                continue;
            }

            if let Some((lhs, rhs)) = line.split_once('=') {
                let dst = lhs.trim().to_string();
                let rhs_t = rhs.trim();
                if let Some((_, slot, _)) = parse_load_inst(rhs_t) {
                    if let Some(h) = st.slot_handle.get(&slot).cloned() {
                        st.reg_handle.insert(dst, h);
                    }
                    continue;
                }
                if rhs_t.starts_with("inttoptr ") || rhs_t.starts_with("bitcast ") {
                    let toks: Vec<&str> = rhs_t.split_whitespace().collect();
                    if let Some(&src) = toks.get(2) {
                        if let Some(h) = st.reg_handle.get(src).cloned() {
                            if st.touched.contains(&h)
                                && st.rc.get(&h).map(|(_, hi)| *hi).unwrap_or(1) <= 0
                            {
                                violations.push(format!(
                                    "block `{}`: use-after-free dereferencing dropped handle `{h}`",
                                    blocks[bi].0
                                ));
                            }
                            st.reg_handle.insert(dst, h);
                        }
                    }
                    continue;
                }
                if rhs_t.contains("call ") {
                    if let (Some(lp), Some(rp)) = (rhs_t.find('('), rhs_t.rfind(')')) {
                        for item in rhs_t[lp + 1..rp].split(',') {
                            if let Some(arg) = item.split_whitespace().last() {
                                if let Some(h) = st.reg_handle.get(arg).cloned() {
                                    if st.touched.contains(&h) {
                                        let (lo, hi) = st.rc.entry(h.clone()).or_insert((1, 1));
                                        if *hi <= 0 {
                                            violations.push(format!(
                                                "block `{}`: use-after-free passing dropped handle `{h}` to call",
                                                blocks[bi].0
                                            ));
                                        } else {
                                            // Callee may consume 1 owned ref OR borrow 0 refs.
                                            *lo = (*lo - 1).max(0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    st.reg_handle.insert(dst.clone(), dst.clone());
                    st.rc.insert(dst, (1, 1));
                    continue;
                }
            }
        }

        exit_states[bi] = Some(st);
    }

    if violations.is_empty() {
        Ok(TvFunctionReport {
            fn_name: fn_name.to_string(),
            stage: "rc-safety".to_string(),
            status: "verified".to_string(),
            details: format!(
                "balanced Perceus RC across {} block(s) ({total_dups} dup, {total_drops} drop)",
                blocks.len()
            ),
        })
    } else {
        Ok(TvFunctionReport {
            fn_name: fn_name.to_string(),
            stage: "rc-safety".to_string(),
            status: "disproved".to_string(),
            details: violations.join("; "),
        })
    }
}

/// Audit all emitted LLVM functions matching `user_module_prefix` that perform Perceus RC ops.
pub fn validate_module_rc_safety(ir: &str, user_module_prefix: &str) -> Vec<TvFunctionReport> {
    let mut reports = Vec::new();
    for line in ir.lines() {
        if !line.starts_with("define ") {
            continue;
        }
        let Some(at_q) = line.find("@\"") else { continue };
        let after = &line[at_q + 2..];
        let Some(end_q) = after.find('"') else { continue };
        let fname = &after[..end_q];
        if !fname.starts_with(user_module_prefix)
            || fname.ends_with(".app")
            || fname.ends_with(".exit")
            || fname.ends_with(".wrap")
        {
            continue;
        }
        if let Ok(rep) = validate_fn_rc_safety(fname, ir) {
            reports.push(rep);
        }
    }
    reports
}

/// Extract selected `llvm` prim bodies from a checked `Program` for inlining during Core TV.
pub fn collect_program_prim_bodies(
    program: &Program,
) -> HashMap<String, (Vec<SemParam>, SemTy, String)> {
    let mut map = HashMap::new();
    for p in &program.prims {
        if let Some(body) = p.bodies.get(p.selected) {
            if body.family == "llvm" && !body.trusted {
                let mut params = Vec::new();
                let mut ok = true;
                for (i, a) in p.args.iter().enumerate() {
                    if let Some((st, signed)) = prim_abi_to_sem_ty(a) {
                        params.push(SemParam {
                            index: i,
                            name: format!("p{i}"),
                            ty: st,
                            signed,
                        });
                    } else {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    if let Some((rt, _)) = prim_abi_to_sem_ty(&p.res) {
                        map.insert(p.unit.clone(), (params, rt, body.fragment.clone()));
                    }
                }
            }
        }
    }
    map
}

fn prim_abi_to_sem_ty(ty: &crate::core::PrimAbiTy) -> Option<(SemTy, bool)> {
    use crate::core::PrimAbiTy;
    match ty {
        PrimAbiTy::Int { bits, signed } => Some((SemTy::Bv(*bits as u16), *signed)),
        PrimAbiTy::F32 | PrimAbiTy::F64 | PrimAbiTy::Ptr | PrimAbiTy::Unit => None,
    }
}

/// Validate all pure scalar, interprocedural, tail-recursive, and RC-managed user functions
/// in a `Program` against emitted LLVM IR (returning per-function verification reports).
pub fn validate_program_codegen(
    program: &Program,
    ir: &str,
    user_module_prefix: &str,
) -> Vec<TvFunctionReport> {
    validate_program_codegen_for_word_bits(program, ir, user_module_prefix, 64)
}

/// Target-width-aware version of [`validate_program_codegen`].
pub fn validate_program_codegen_for_word_bits(
    program: &Program,
    ir: &str,
    user_module_prefix: &str,
    word_bits: u16,
) -> Vec<TvFunctionReport> {
    let prim_bodies = collect_program_prim_bodies(program);
    let all_fns: HashMap<String, &FnDef> = program
        .fns
        .iter()
        .filter(|f| f.prim.is_none() && f.ctor.is_none())
        .map(|f| (f.id.0.clone(), f))
        .collect();
    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let mut reports = Vec::new();
    for f in &program.fns {
        if !f.id.0.starts_with(user_module_prefix) || f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let stage_name = if core_has_self_tail_call(&f.body, &f.id.0) {
            "core->llvm(loop-step)"
        } else {
            "core->llvm"
        };
        match build_core_vs_llvm_obligation_with_fns(f, ir, &prim_bodies, &all_fns, word_bits) {
            Ok(ob) => {
                let rep = engine.verify(&ob);
                let (status, details) = match rep.status {
                    VerificationStatus::Verified { receipt, .. } => {
                        ("verified".to_string(), format!("{}: {}", receipt.checker_id, receipt.details))
                    }
                    VerificationStatus::Disproved { counterexample, .. } => {
                        ("disproved".to_string(), counterexample.summary)
                    }
                    VerificationStatus::RejectedByPolicy { reason, .. } => {
                        ("rejected".to_string(), reason)
                    }
                    VerificationStatus::Unverified { reasons } => {
                        ("open".to_string(), reasons.join("; "))
                    }
                };
                reports.push(TvFunctionReport {
                    fn_name: f.id.0.clone(),
                    stage: stage_name.to_string(),
                    status,
                    details,
                });
            }
            Err(reason) => {
                reports.push(TvFunctionReport {
                    fn_name: f.id.0.clone(),
                    stage: stage_name.to_string(),
                    status: "skipped".to_string(),
                    details: reason,
                });
            }
        }
    }
    reports.extend(validate_module_rc_safety(ir, user_module_prefix));
    reports
}
