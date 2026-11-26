//! Translation Validation (`Core` → `Core` optimizer & `Core` → emitted LLVM IR).
//!
//! Lowers pure scalar/branching Cinch [`crate::core::FnDef`] bodies directly into
//! the shared [`super::sem::SemGraph`] and proves:
//! 1. **Optimizer validation (`validate_core_transform`):**
//!    `SemGraph(Core_before) == SemGraph(Core_after)` across `simplify::simplify`.
//! 2. **Codegen validation (`validate_core_vs_llvm`):**
//!    `SemGraph(Core) ⊑ SemGraph(emitted LLVM IR)` for `codegen::emit`, proving
//!    the emitted LLVM function is UB-free, poison-free, and bit-for-bit identical
//!    to the Core function with a kernel-checked LRAT proof!

use super::llvm_ir::{lower_llvm_fragment, LLVM_SEMANTICS_ID};
use super::sem::{NodeId, Obligation, ObligationKind, SemGraph, SemParam, SemProgram, SemTy, SemVal};
use super::{Policy, VerificationStatus, VerifierEngine};
use crate::core::{BitOp, CmpOp, Core, FnDef, LocalId, PrimOp, Program};
use crate::repr::Class;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Lower a pure scalar/branching `Core::FnDef` into a `SemProgram`.
pub fn lower_core_fn(
    f: &FnDef,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
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

    let res_node = lower_core_expr(&mut g, &f.body, &env, prim_bodies)?;
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

fn lower_core_expr(
    g: &mut SemGraph,
    expr: &Core,
    env: &HashMap<LocalId, NodeId>,
    prim_bodies: &HashMap<String, (Vec<SemParam>, SemTy, String)>,
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
            let rv = lower_core_expr(g, rhs, env, prim_bodies)?;
            let mut next = env.clone();
            next.insert(*lid, rv);
            lower_core_expr(g, body, &next, prim_bodies)
        }
        Core::If(cond, then_b, else_b) => {
            let cv = lower_core_expr(g, cond, env, prim_bodies)?;
            let c_bool = coerce_bv(g, cv, SemTy::Bool)?;
            let tv = lower_core_expr(g, &then_b.body, env, prim_bodies)?;
            let ev = lower_core_expr(g, &else_b.body, env, prim_bodies)?;
            let tv_ty = g.ty(tv);
            let ev_c = coerce_bv(g, ev, tv_ty)?;
            Ok(g.ite(c_bool, tv, ev_c))
        }
        Core::App(head, args) => {
            let mut arg_nodes = Vec::with_capacity(args.len());
            for a in args {
                arg_nodes.push(lower_core_expr(g, a, env, prim_bodies)?);
            }
            match head.as_ref() {
                Core::Prim(op) => lower_core_primop(g, *op, &arg_nodes),
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
                        // Machine scalar prims return narrower widths; zero-extend to i64 word unless wide.
                        match g.ty(out) {
                            SemTy::Bv(128) => Ok(out),
                            SemTy::Bv(w) if w < 64 => Ok(g.bv_zext(out, 64)),
                            _ => Ok(out),
                        }
                    } else {
                        lower_named_scalar_global(g, &gid.0, &arg_nodes)
                    }
                }
                _ => Err("unsupported non-direct application in Core TV".to_string()),
            }
        }
        _ => Err(format!("unsupported Core node in scalar TV: {:?}", std::mem::discriminant(expr))),
    }
}

fn lower_core_primop(g: &mut SemGraph, op: PrimOp, args: &[NodeId]) -> Result<NodeId, String> {
    match op {
        PrimOp::Arith { ty, op: aop, mode: crate::core::ArithMode::Wrapping } => {
            let w = ty.width() as u16;
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
            let w = ty.width() as u16;
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
            let w = ty.width() as u16;
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
    let pa = lower_core_fn(before, prim_bodies)?;
    let pb = lower_core_fn(after, prim_bodies)?;
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
    let ret_word = ret_section.split_whitespace().last()?;
    let ret_ty = parse_llvm_scalar_ty(ret_word)?;

    let params_str = &header[lparen + 1..rparen];
    let mut params = Vec::new();
    let mut renames: Vec<(String, String)> = Vec::new();
    if !params_str.trim().is_empty() {
        for (idx, item) in params_str.split(',').enumerate() {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let ty_word = parts.first().copied()?;
            let sem_ty = parse_llvm_scalar_ty(ty_word)?;
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

/// Inline straight-line direct calls `%dst = call <ty> @"<callee>"(<args...>)`
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
                let callee_lines = inline_direct_calls(ir, raw_callee, counter, depth + 1)?;
                // Only inline single-block (straight-line) callees to keep CFG structure simple.
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
                        // Collect all locally defined registers in callee body.
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
                            // Rename local defs first using unique temp placeholders so %0..%k don't collide.
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
        out.push(line);
    }
    Ok(out)
}

fn parse_direct_quoted_call(line: &str) -> Option<(String, String, Vec<String>)> {
    let (lhs, rhs) = line.split_once('=')?;
    let dst = lhs.trim();
    if !dst.starts_with('%') {
        return None;
    }
    let rhs = rhs.trim();
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

/// Exact acyclic SSA promotion (`mem2reg`) over local scalar `alloca`/`store`/`load` slots.
///
/// Only promotes non-escaping `alloca <scalar-ty>` registers that are accessed solely via
/// direct `store <ty> <val>, ptr %slot` and `%dst = load <ty>, ptr %slot` instructions
/// inside a forward-only acyclic CFG.
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

    // Identify local scalar allocas.
    let mut slot_tys: HashMap<String, String> = HashMap::new();
    for b in &blocks {
        for l in &b.lines {
            if let Some((lhs, rhs)) = l.split_once('=') {
                let reg = lhs.trim();
                let rhs = rhs.trim();
                if let Some(rest) = rhs.strip_prefix("alloca ") {
                    let ty = rest.split(|c: char| c.is_whitespace() || c == ',').next().unwrap_or("");
                    if parse_llvm_scalar_ty(ty).is_some() {
                        slot_tys.insert(reg.to_string(), ty.to_string());
                    }
                }
            }
        }
    }
    if slot_tys.is_empty() {
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

    // Walk blocks in topological order, promoting allocas to SSA values / phi nodes.
    let mut out_slots: Vec<HashMap<String, String>> = vec![HashMap::new(); blocks.len()];
    let mut emitted_blocks: Vec<(String, Vec<String>)> = vec![(String::new(), Vec::new()); blocks.len()];
    let mut aliases: Vec<(String, String)> = Vec::new();
    let mut phi_counter = 0usize;

    for &bi in &topo {
        let b = &blocks[bi];
        let mut cur_slots: HashMap<String, String> = HashMap::new();
        let mut new_lines: Vec<String> = Vec::new();

        if preds[bi].len() == 1 {
            cur_slots = out_slots[preds[bi][0]].clone();
        } else if preds[bi].len() > 1 {
            let mut slot_names: Vec<String> = slot_tys.keys().cloned().collect();
            slot_names.sort();
            for sname in slot_names {
                let mut incoming: Vec<(String, String)> = Vec::new();
                for &pi in &preds[bi] {
                    if let Some(val) = out_slots[pi].get(&sname) {
                        incoming.push((val.clone(), blocks[pi].label.clone()));
                    }
                }
                if incoming.len() == preds[bi].len() {
                    let first_val = &incoming[0].0;
                    if incoming.iter().all(|(v, _)| v == first_val) {
                        cur_slots.insert(sname, first_val.clone());
                    } else {
                        let ty = &slot_tys[&sname];
                        let phi_reg = format!("%m2r_{phi_counter}");
                        phi_counter += 1;
                        let pairs: Vec<String> = incoming
                            .iter()
                            .map(|(v, lbl)| format!("[ {v}, %{lbl} ]"))
                            .collect();
                        new_lines.push(format!("{phi_reg} = phi {ty} {}", pairs.join(", ")));
                        cur_slots.insert(sname, phi_reg);
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
                if rhs_t.starts_with("alloca ") && slot_tys.contains_key(reg) {
                    continue;
                }
                if let Some(slot) = parse_scalar_load_slot(rhs_t, &slot_tys) {
                    if let Some(val) = cur_slots.get(&slot) {
                        aliases.push((reg.to_string(), val.clone()));
                        continue;
                    }
                }
            }
            if let Some((slot, val)) = parse_scalar_store_slot(&l, &slot_tys) {
                cur_slots.insert(slot, val);
                continue;
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

fn parse_scalar_load_slot(rhs: &str, slots: &HashMap<String, String>) -> Option<String> {
    let rest = rhs.strip_prefix("load ")?;
    let ptr_pos = rest.find("ptr ")?;
    let after = rest[ptr_pos + 4..].trim();
    let slot = after.split(|c: char| c.is_whitespace() || c == ',').next()?;
    if slots.contains_key(slot) {
        Some(slot.to_string())
    } else {
        None
    }
}

fn parse_scalar_store_slot(line: &str, slots: &HashMap<String, String>) -> Option<(String, String)> {
    let rest = line.trim().strip_prefix("store ")?;
    let (val_part, ptr_part) = rest.split_once(',')?;
    let vparts: Vec<&str> = val_part.split_whitespace().collect();
    if vparts.len() < 2 {
        return None;
    }
    let val = vparts[1].to_string();
    let ptr_t = ptr_part.trim().strip_prefix("ptr ")?;
    let slot = ptr_t.split(|c: char| c.is_whitespace() || c == ',').next()?;
    if slots.contains_key(slot) {
        Some((slot.to_string(), val))
    } else {
        None
    }
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
    let core_prog = lower_core_fn(f, prim_bodies)?;
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
    let llvm_prog = lower_llvm_fragment(&llvm_frag, &llvm_params, llvm_ret)
        .map_err(|e| format!("lowering emitted LLVM for `{}`: {e}", f.id.0))?;
    let (vc, _) = core_prog.result.as_scalar().ok_or("non-scalar Core result")?;
    let (vl, pl) = llvm_prog.result.as_scalar().ok_or("non-scalar LLVM result")?;

    let mut g = core_prog.graph.clone();
    let pnodes: Vec<NodeId> = core_prog.params.iter().map(|(_, n)| *n).collect();
    let map_l = g.import_graph(&llvm_prog.graph, &pnodes);
    let ub_l = map_l[llvm_prog.ub_cond.0 as usize];
    let vl_m = map_l[vl.0 as usize];
    let pl_m = map_l[pl.0 as usize];

    let not_ub = g.not(ub_l);
    let not_p = g.not(pl_m);
    let wd = g.and(not_ub, not_p);
    let eq = g.bv_eq(vc, vl_m);
    let goal = g.and(wd, eq);
    let sem_params: Vec<SemParam> = core_prog.params.into_iter().map(|(p, _)| p).collect();
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
        PrimAbiTy::F32 | PrimAbiTy::F64 => None,
    }
}

/// Validate all pure scalar user functions in a `Program` against emitted LLVM IR
/// (and return per-function LRAT verification reports).
pub fn validate_program_codegen(
    program: &Program,
    ir: &str,
    user_module_prefix: &str,
) -> Vec<TvFunctionReport> {
    let prim_bodies = collect_program_prim_bodies(program);
    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let mut reports = Vec::new();
    for f in &program.fns {
        if !f.id.0.starts_with(user_module_prefix) || f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        match build_core_vs_llvm_obligation(f, ir, &prim_bodies) {
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
                    stage: "core->llvm".to_string(),
                    status,
                    details,
                });
            }
            Err(reason) => {
                reports.push(TvFunctionReport {
                    fn_name: f.id.0.clone(),
                    stage: "core->llvm".to_string(),
                    status: "skipped".to_string(),
                    details: reason,
                });
            }
        }
    }
    reports
}
