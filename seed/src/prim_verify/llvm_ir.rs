//! Versioned Cinch-LLVM formal semantics plugin (`llvm-sem@21.1`).
//!
//! Parses Cinch's straight-line LLVM IR fragment subset into a typed AST and
//! lowers it directly into the shared `SemGraph` with Alive2-faithful UB and
//! per-lane `poison` propagation.

use super::sem::{
    FpFmt, FpPred, FpUnaryOp, NodeId, SemGraph, SemParam, SemProgram, SemTy, SemVal,
};
use super::{IrSemantics, SemError};
use std::collections::HashMap;

pub const LLVM_SEMANTICS_ID: &str = "llvm-sem@21.1";

/// Pluggable `IrSemantics` implementation for Cinch's straight-line LLVM subset.
#[derive(Clone, Debug, Default)]
pub struct LlvmSemantics;

impl IrSemantics for LlvmSemantics {
    fn id(&self) -> &str {
        LLVM_SEMANTICS_ID
    }

    fn family(&self) -> &str {
        "llvm"
    }

    fn lower(
        &self,
        fragment: &str,
        params: &[SemParam],
        ret_ty: SemTy,
    ) -> Result<SemProgram, SemError> {
        lower_llvm_fragment(fragment, params, ret_ty)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LlvmTy {
    Int(u16),
    Float,
    Double,
    Vec(u16, Box<LlvmTy>),
    Struct(Vec<LlvmTy>),
}

impl LlvmTy {
    fn to_sem_ty(&self) -> Option<SemTy> {
        match *self {
            LlvmTy::Int(w) => Some(SemTy::Bv(w)),
            LlvmTy::Float => Some(SemTy::Fp(FpFmt::F32)),
            LlvmTy::Double => Some(SemTy::Fp(FpFmt::F64)),
            _ => None,
        }
    }

    fn bit_width(&self) -> Option<u16> {
        match self {
            LlvmTy::Int(w) => Some(*w),
            LlvmTy::Float => Some(32),
            LlvmTy::Double => Some(64),
            LlvmTy::Vec(n, elem) => elem.bit_width().map(|w| w * n),
            LlvmTy::Struct(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Reg(String),
    Int(i128),
    FloatHex(u64),
    FloatDec(String),
    Bool(bool),
    Poison,
    Undef,
    ZeroInit,
    VecLit(Vec<(LlvmTy, Operand)>),
}

fn strip_comment(line: &str) -> &str {
    match line.find(';') {
        Some(i) => &line[..i],
        None => line,
    }
}

/// Lower a straight-line or acyclic multi-block LLVM IR fragment into `SemProgram`.
///
/// Supports forward-only basic blocks (`label:`, `br i1`, `br label`, `phi`),
/// guarding instruction-local UB by each block's exact path reachability condition
/// and enforcing strict acyclicity (rejecting any backward branch or loop).
pub fn lower_llvm_fragment(
    fragment: &str,
    params: &[SemParam],
    ret_ty: SemTy,
) -> Result<SemProgram, SemError> {
    lower_llvm_fragment_with_uf(fragment, params, ret_ty, &mut None)
}

/// Callback invoked for direct non-intrinsic scalar calls (`call <ty> @"<callee>"(...)`)
/// during translation validation, allowing Ackermann uninterpreted-function congruence.
pub type UfCallHook<'a> =
    &'a mut dyn FnMut(&mut SemGraph, &str, &[NodeId], SemTy) -> Result<NodeId, SemError>;

/// Lower an acyclic LLVM IR fragment into `SemProgram`, optionally routing non-intrinsic
/// direct scalar calls through `uf_hook` for interprocedural Ackermann congruence.
pub fn lower_llvm_fragment_with_uf(
    fragment: &str,
    params: &[SemParam],
    _ret_ty: SemTy,
    uf_hook: &mut Option<UfCallHook<'_>>,
) -> Result<SemProgram, SemError> {
    let mut g = SemGraph::new();
    let mut env: HashMap<String, (LlvmTy, SemVal)> = HashMap::new();
    let mut param_nodes = Vec::with_capacity(params.len());

    let false_node = g.bool_const(false);
    for p in params {
        let node = g.param(p.index, p.name.clone(), p.ty);
        param_nodes.push((p.clone(), node));
        let llvm_ty = match p.ty {
            SemTy::Bool => LlvmTy::Int(1),
            SemTy::Bv(w) => LlvmTy::Int(w),
            SemTy::Fp(FpFmt::F32) => LlvmTy::Float,
            SemTy::Fp(FpFmt::F64) => LlvmTy::Double,
        };
        env.insert(
            format!("%{}", p.index),
            (llvm_ty, SemVal::Scalar { val: node, poison: false_node }),
        );
    }

    // Partition lines into basic blocks. If the fragment starts with instructions
    // rather than an explicit label, they belong to implicit block `"entry"`.
    struct RawBlock {
        name: String,
        lines: Vec<(usize, String)>,
    }
    let mut blocks: Vec<RawBlock> = Vec::new();
    for (line_no, raw) in fragment.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(lbl) = line.strip_suffix(':') {
            let lbl = lbl.trim();
            if lbl.is_empty() || lbl.contains(char::is_whitespace) {
                return Err(SemError::Parse(format!(
                    "line {}: invalid basic block label `{line}`",
                    line_no + 1
                )));
            }
            if blocks.iter().any(|b| b.name == lbl) {
                return Err(SemError::Parse(format!(
                    "line {}: duplicate basic block label `{lbl}`",
                    line_no + 1
                )));
            }
            blocks.push(RawBlock { name: lbl.to_string(), lines: Vec::new() });
        } else {
            if blocks.is_empty() {
                blocks.push(RawBlock { name: "entry".to_string(), lines: Vec::new() });
            }
            blocks.last_mut().unwrap().lines.push((line_no + 1, line.to_string()));
        }
    }

    if blocks.is_empty() {
        return Err(SemError::Parse("LLVM fragment has no instructions".to_string()));
    }

    let block_index: HashMap<String, usize> = blocks
        .iter()
        .enumerate()
        .map(|(i, b)| (b.name.clone(), i))
        .collect();

    let mut reach: Vec<NodeId> = vec![g.bool_const(false); blocks.len()];
    reach[0] = g.bool_const(true);
    let mut edge_conds: HashMap<(usize, usize), NodeId> = HashMap::new();
    let mut ub_cond = g.bool_const(false);
    let mut returns: Vec<(NodeId, SemVal)> = Vec::new();

    for b_idx in 0..blocks.len() {
        // Compute reachability of block `b_idx` as OR of all incoming edges (block 0 is always reachable).
        if b_idx > 0 {
            let mut r = g.bool_const(false);
            for pred in 0..b_idx {
                if let Some(&ec) = edge_conds.get(&(pred, b_idx)) {
                    r = g.or(r, ec);
                }
            }
            reach[b_idx] = r;
        }
        let cur_reach = reach[b_idx];
        let mut block_ub = g.bool_const(false);
        let mut terminated = false;

        for (line_no, line) in &blocks[b_idx].lines {
            if terminated {
                return Err(SemError::Parse(format!(
                    "line {line_no}: instruction after block terminator in `{}`",
                    blocks[b_idx].name
                )));
            }
            // Optional `%dst = ...`
            let (dst, rhs) = if line.starts_with('%') {
                let eq = line.find('=').ok_or_else(|| {
                    SemError::Parse(format!("line {line_no}: expected `=` after register"))
                })?;
                (Some(line[..eq].trim().to_string()), line[eq + 1..].trim())
            } else {
                (None, line.as_str())
            };

            let (op, rest) = split_head_word(rhs);
            match op {
                "ret" => {
                    let (ty, rest) = parse_ty(rest)?;
                    let operand = parse_operand(rest.trim())?;
                    let val = eval_operand(&mut g, &env, &ty, &operand)?;
                    returns.push((cur_reach, val));
                    terminated = true;
                }
                "br" => {
                    let rest = rest.trim();
                    if let Some(lbl_rest) = rest.strip_prefix("label ") {
                        let target_name = lbl_rest.trim().trim_start_matches('%');
                        let &t_idx = block_index.get(target_name).ok_or_else(|| {
                            SemError::Parse(format!(
                                "line {line_no}: undefined branch target `%{target_name}`"
                            ))
                        })?;
                        if t_idx <= b_idx {
                            return Err(SemError::Unsupported(format!(
                                "line {line_no}: cyclic CFG (backward branch from `{}` to `{target_name}`)",
                                blocks[b_idx].name
                            )));
                        }
                        let entry = edge_conds
                            .entry((b_idx, t_idx))
                            .or_insert_with(|| g.bool_const(false));
                        *entry = g.or(*entry, cur_reach);
                    } else if let Some(cond_rest) = rest.strip_prefix("i1 ") {
                        let (cond_op, after_cond) = parse_operand_until_comma(cond_rest)?;
                        let (true_lbl, after_t) = parse_label_until_comma(after_cond)?;
                        let false_lbl = parse_label_end(after_t)?;
                        let &t_idx = block_index.get(true_lbl).ok_or_else(|| {
                            SemError::Parse(format!(
                                "line {line_no}: undefined branch target `%{true_lbl}`"
                            ))
                        })?;
                        let &f_idx = block_index.get(false_lbl).ok_or_else(|| {
                            SemError::Parse(format!(
                                "line {line_no}: undefined branch target `%{false_lbl}`"
                            ))
                        })?;
                        if t_idx <= b_idx || f_idx <= b_idx {
                            return Err(SemError::Unsupported(format!(
                                "line {line_no}: cyclic CFG (backward branch from `{}`)",
                                blocks[b_idx].name
                            )));
                        }
                        let cond_val = eval_operand(&mut g, &env, &LlvmTy::Int(1), &cond_op)?;
                        let (cv, cp) = cond_val.as_scalar().ok_or_else(|| {
                            SemError::Parse(format!("line {line_no}: br condition must be i1"))
                        })?;
                        // Branching on poison `i1` is immediate UB in LLVM!
                        block_ub = g.or(block_ub, cp);
                        let c_bool = g.bv1_to_bool(cv);
                        let not_c = g.not(c_bool);
                        let t_edge = g.and(cur_reach, c_bool);
                        let f_edge = g.and(cur_reach, not_c);
                        let te = edge_conds
                            .entry((b_idx, t_idx))
                            .or_insert_with(|| g.bool_const(false));
                        *te = g.or(*te, t_edge);
                        let fe = edge_conds
                            .entry((b_idx, f_idx))
                            .or_insert_with(|| g.bool_const(false));
                        *fe = g.or(*fe, f_edge);
                    } else {
                        return Err(SemError::Parse(format!(
                            "line {line_no}: malformed `br`: `{rhs}`"
                        )));
                    }
                    terminated = true;
                }
                "phi" => {
                    let (ty, pairs_str) = parse_ty(rest)?;
                    let pairs = parse_phi_incoming(pairs_str)?;
                    if pairs.is_empty() {
                        return Err(SemError::Parse(format!(
                            "line {line_no}: phi has no incoming edges"
                        )));
                    }
                    let mut incoming_vals: Vec<(NodeId, SemVal)> = Vec::new();
                    for (val_str, pred_lbl) in pairs {
                        let &p_idx = block_index.get(pred_lbl.as_str()).ok_or_else(|| {
                            SemError::Parse(format!(
                                "line {line_no}: phi references unknown label `%{pred_lbl}`"
                            ))
                        })?;
                        if p_idx >= b_idx {
                            return Err(SemError::Unsupported(format!(
                                "line {line_no}: cyclic phi incoming from `{pred_lbl}` into `{}`",
                                blocks[b_idx].name
                            )));
                        }
                        let ec = edge_conds
                            .get(&(p_idx, b_idx))
                            .copied()
                            .unwrap_or_else(|| g.bool_const(false));
                        let op_parsed = parse_operand(val_str.trim())?;
                        let v = eval_operand(&mut g, &env, &ty, &op_parsed)?;
                        incoming_vals.push((ec, v));
                    }
                    let merged = merge_guarded_semvals(&mut g, &incoming_vals)?;
                    bind_dst(&mut env, dst, ty, merged)?;
                }
                "freeze" => {
                    let (ty, rest) = parse_ty(rest)?;
                    let op1 = parse_operand(rest.trim())?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let f = g.bool_const(false);
                    let frozen = match v1 {
                        SemVal::Scalar { val, .. } => SemVal::Scalar { val, poison: f },
                        SemVal::Vector { elem_ty, lanes } => SemVal::Vector {
                            elem_ty,
                            lanes: lanes.into_iter().map(|(v, _)| (v, f)).collect(),
                        },
                        SemVal::Tuple(_) => {
                            return Err(SemError::Unsupported("aggregate freeze".to_string()))
                        }
                    };
                    bind_dst(&mut env, dst, ty, frozen)?;
                }
                "load" | "store" | "inttoptr" | "ptrtoint" | "getelementptr" | "alloca"
                | "cmpxchg" | "atomicrmw" | "fence" => {
                    return Err(SemError::Unsupported(format!(
                        "memory/pointer instruction `{op}` requires a memory-model semantics plugin"
                    )));
                }
                "switch" | "indirectbr" | "invoke" | "unreachable" => {
                    return Err(SemError::Unsupported(format!(
                        "control-flow instruction `{op}` is not modeled in {LLVM_SEMANTICS_ID}"
                    )));
                }
                "add" | "sub" | "mul" | "udiv" | "urem" | "sdiv" | "srem" | "and" | "or" | "xor"
                | "shl" | "lshr" | "ashr" => {
                    let (flags, rest) = parse_arith_flags(rest);
                    let (ty, rest) = parse_ty(rest)?;
                    let (op1, op2) = parse_two_operands(rest)?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let v2 = eval_operand(&mut g, &env, &ty, &op2)?;
                    let res = lower_int_binop(&mut g, &mut block_ub, op, flags, &ty, &v1, &v2)?;
                    bind_dst(&mut env, dst, ty, res)?;
                }
                "fadd" | "fsub" | "fmul" | "fdiv" => {
                    let rest = skip_fast_math_flags(rest);
                    let (ty, rest) = parse_ty(rest)?;
                    let (op1, op2) = parse_two_operands(rest)?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let v2 = eval_operand(&mut g, &env, &ty, &op2)?;
                    let res = lower_fp_binop(&mut g, op, &v1, &v2)?;
                    bind_dst(&mut env, dst, ty, res)?;
                }
                "fneg" => {
                    let rest = skip_fast_math_flags(rest);
                    let (ty, rest) = parse_ty(rest)?;
                    let op1 = parse_operand(rest.trim())?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let (val, poison) = v1.as_scalar().ok_or_else(|| {
                        SemError::Unsupported("vector fneg not yet modeled".to_string())
                    })?;
                    let r = g.fp_neg(val);
                    bind_dst(&mut env, dst, ty, SemVal::Scalar { val: r, poison })?;
                }
                "icmp" => {
                    let (pred, rest) = split_head_word(rest);
                    let (ty, rest) = parse_ty(rest)?;
                    let (op1, op2) = parse_two_operands(rest)?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let v2 = eval_operand(&mut g, &env, &ty, &op2)?;
                    let (out_ty, res) = lower_icmp(&mut g, pred, &ty, &v1, &v2)?;
                    bind_dst(&mut env, dst, out_ty, res)?;
                }
                "fcmp" => {
                    let rest = skip_fast_math_flags(rest);
                    let (pred, rest) = split_head_word(rest);
                    let (ty, rest) = parse_ty(rest)?;
                    let (op1, op2) = parse_two_operands(rest)?;
                    let v1 = eval_operand(&mut g, &env, &ty, &op1)?;
                    let v2 = eval_operand(&mut g, &env, &ty, &op2)?;
                    let (a, pa) = v1.as_scalar().ok_or_else(|| {
                        SemError::Unsupported("vector fcmp not yet modeled".to_string())
                    })?;
                    let (b, pb) = v2.as_scalar().ok_or_else(|| {
                        SemError::Unsupported("vector fcmp not yet modeled".to_string())
                    })?;
                    let fp_pred = parse_fp_pred(pred)?;
                    let cmp = g.fp_cmp(fp_pred, a, b);
                    let val = g.bool_to_bv1(cmp);
                    let poison = g.or(pa, pb);
                    bind_dst(&mut env, dst, LlvmTy::Int(1), SemVal::Scalar { val, poison })?;
                }
                "zext" | "sext" | "trunc" | "bitcast" | "sitofp" | "uitofp" | "fptosi" | "fptoui"
                | "fpext" | "fptrunc" => {
                    let (src_ty, rest) = parse_ty(rest)?;
                    let (op_str, dst_str) = split_on_keyword_to(rest)?;
                    let op1 = parse_operand(op_str.trim())?;
                    let (dst_ty, _) = parse_ty(dst_str.trim())?;
                    let v1 = eval_operand(&mut g, &env, &src_ty, &op1)?;
                    let res = lower_cast(&mut g, op, &src_ty, &dst_ty, &v1)?;
                    bind_dst(&mut env, dst, dst_ty, res)?;
                }
                "select" => {
                    let rest = skip_fast_math_flags(rest);
                    let (c_ty, rest) = parse_ty(rest)?;
                    let (c_op, rest) = parse_operand_until_comma(rest)?;
                    let (t_ty, rest) = parse_ty(rest)?;
                    let (t_op, rest) = parse_operand_until_comma(rest)?;
                    let (f_ty, rest) = parse_ty(rest)?;
                    let f_op = parse_operand(rest.trim())?;
                    if t_ty != f_ty {
                        return Err(SemError::Parse(format!(
                            "select branch type mismatch: {t_ty:?} vs {f_ty:?}"
                        )));
                    }
                    let vc = eval_operand(&mut g, &env, &c_ty, &c_op)?;
                    let vt = eval_operand(&mut g, &env, &t_ty, &t_op)?;
                    let vf = eval_operand(&mut g, &env, &f_ty, &f_op)?;
                    let res = lower_select(&mut g, &vc, &vt, &vf)?;
                    bind_dst(&mut env, dst, t_ty, res)?;
                }
                "insertelement" => {
                    let (v_ty, rest) = parse_ty(rest)?;
                    let (v_op, rest) = parse_operand_until_comma(rest)?;
                    let (e_ty, rest) = parse_ty(rest)?;
                    let (e_op, rest) = parse_operand_until_comma(rest)?;
                    let (i_ty, rest) = parse_ty(rest)?;
                    let i_op = parse_operand(rest.trim())?;
                    let vv = eval_operand(&mut g, &env, &v_ty, &v_op)?;
                    let ve = eval_operand(&mut g, &env, &e_ty, &e_op)?;
                    let res = lower_insertelement(&mut g, &v_ty, &vv, &ve, &i_ty, &i_op)?;
                    bind_dst(&mut env, dst, v_ty, res)?;
                }
                "extractelement" => {
                    let (v_ty, rest) = parse_ty(rest)?;
                    let (v_op, rest) = parse_operand_until_comma(rest)?;
                    let (_i_ty, rest) = parse_ty(rest)?;
                    let i_op = parse_operand(rest.trim())?;
                    let vv = eval_operand(&mut g, &env, &v_ty, &v_op)?;
                    let (elem_ty, res) = lower_extractelement(&mut g, &v_ty, &vv, &i_op)?;
                    bind_dst(&mut env, dst, elem_ty, res)?;
                }
                "shufflevector" => {
                    let (v1_ty, rest) = parse_ty(rest)?;
                    let (v1_op, rest) = parse_operand_until_comma(rest)?;
                    let (v2_ty, rest) = parse_ty(rest)?;
                    let (v2_op, rest) = parse_operand_until_comma(rest)?;
                    let (m_ty, rest) = parse_ty(rest)?;
                    let m_op = parse_operand(rest.trim())?;
                    let v1 = eval_operand(&mut g, &env, &v1_ty, &v1_op)?;
                    let v2 = eval_operand(&mut g, &env, &v2_ty, &v2_op)?;
                    let (out_ty, res) = lower_shufflevector(&mut g, &v1_ty, &v1, &v2, &m_ty, &m_op)?;
                    bind_dst(&mut env, dst, out_ty, res)?;
                }
                "extractvalue" => {
                    let (agg_ty, rest) = parse_ty(rest)?;
                    let (agg_op, rest) = parse_operand_until_comma(rest)?;
                    let idx: usize = rest.trim().parse().map_err(|_| {
                        SemError::Parse(format!("invalid extractvalue index: `{rest}`"))
                    })?;
                    let agg = eval_operand(&mut g, &env, &agg_ty, &agg_op)?;
                    match (&agg_ty, agg) {
                        (LlvmTy::Struct(tys), SemVal::Tuple(vals)) if idx < tys.len() => {
                            bind_dst(&mut env, dst, tys[idx].clone(), vals[idx].clone())?;
                        }
                        _ => {
                            return Err(SemError::Parse(format!(
                                "invalid extractvalue index {idx} on {agg_ty:?}"
                            )));
                        }
                    }
                }
                "insertvalue" => {
                    let (agg_ty, rest) = parse_ty(rest)?;
                    let (agg_op, rest) = parse_operand_until_comma(rest)?;
                    let (elt_ty, rest) = parse_ty(rest)?;
                    let (elt_op, rest) = parse_operand_until_comma(rest)?;
                    let idx: usize = rest.trim().parse().map_err(|_| {
                        SemError::Parse(format!("invalid insertvalue index: `{rest}`"))
                    })?;
                    let agg = eval_operand(&mut g, &env, &agg_ty, &agg_op)?;
                    let elt = eval_operand(&mut g, &env, &elt_ty, &elt_op)?;
                    match (&agg_ty, agg) {
                        (LlvmTy::Struct(tys), SemVal::Tuple(mut vals)) if idx < tys.len() => {
                            vals[idx] = elt;
                            bind_dst(&mut env, dst, agg_ty, SemVal::Tuple(vals))?;
                        }
                        _ => {
                            return Err(SemError::Parse(format!(
                                "invalid insertvalue index {idx} on {agg_ty:?}"
                            )));
                        }
                    }
                }
                "call" => {
                    if rest.contains(" asm ") || rest.starts_with("asm ") {
                        let (ret_t, res) = lower_verified_crypto_asm(&mut g, &env, rest)?;
                        bind_dst(&mut env, dst, ret_t, res)?;
                        continue;
                    }
                    let (ret_t, rest) = parse_ty(rest)?;
                    let rest = rest.trim();
                    let at = rest.find('@').ok_or_else(|| {
                        SemError::Unsupported("indirect or non-intrinsic call".to_string())
                    })?;
                    let lparen = rest[at..].find('(').ok_or_else(|| {
                        SemError::Parse(format!("missing `(` in call `{rest}`"))
                    })? + at;
                    let rparen = rest.rfind(')').ok_or_else(|| {
                        SemError::Parse(format!("missing `)` in call `{rest}`"))
                    })?;
                    let sym = rest[at + 1..lparen].trim();
                    let args_str = &rest[lparen + 1..rparen];
                    let call_args = parse_call_args(args_str)?;
                    let mut eval_args = Vec::with_capacity(call_args.len());
                    for (aty, aop) in call_args {
                        let av = eval_operand(&mut g, &env, &aty, &aop)?;
                        eval_args.push((aty, aop, av));
                    }
                    let res = match lower_intrinsic(&mut g, sym, &ret_t, &eval_args) {
                        Ok(v) => v,
                        Err(orig_err) => {
                            if let (Some(hook), Some(r_sem_ty)) = (uf_hook.as_deref_mut(), ret_t.to_sem_ty()) {
                                let clean_sym = sym.trim_matches('"');
                                let mut arg_nodes = Vec::with_capacity(eval_args.len());
                                let mut arg_poison = g.bool_const(false);
                                let mut all_scalar = true;
                                for (_, _, av) in &eval_args {
                                    if let Some((v, p)) = av.as_scalar() {
                                        arg_nodes.push(v);
                                        arg_poison = g.or(arg_poison, p);
                                    } else {
                                        all_scalar = false;
                                        break;
                                    }
                                }
                                if all_scalar {
                                    let out = hook(&mut g, clean_sym, &arg_nodes, r_sem_ty)?;
                                    SemVal::Scalar { val: out, poison: arg_poison }
                                } else {
                                    return Err(orig_err);
                                }
                            } else {
                                return Err(orig_err);
                            }
                        }
                    };
                    bind_dst(&mut env, dst, ret_t, res)?;
                }
                other => {
                    return Err(SemError::Unsupported(format!(
                        "LLVM instruction `{other}` is not modeled in {LLVM_SEMANTICS_ID}"
                    )));
                }
            }
        }

        // Guard this block's UB by its reachability condition!
        let guarded_ub = g.and(cur_reach, block_ub);
        ub_cond = g.or(ub_cond, guarded_ub);
    }

    if returns.is_empty() {
        return Err(SemError::Parse("LLVM fragment has no `ret`".to_string()));
    }
    let result = merge_guarded_semvals(&mut g, &returns)?;
    Ok(SemProgram {
        semantics_id: LLVM_SEMANTICS_ID.to_string(),
        graph: g,
        params: param_nodes,
        result,
        ub_cond,
    })
}

fn parse_label_until_comma(s: &str) -> Result<(&str, &str), SemError> {
    let s = s.trim_start();
    let rest = s
        .strip_prefix("label")
        .ok_or_else(|| SemError::Parse(format!("expected `label` in `br`, got `{s}`")))?
        .trim_start();
    let comma = rest
        .find(',')
        .ok_or_else(|| SemError::Parse(format!("expected `,` after branch label in `{s}`")))?;
    let lbl = rest[..comma].trim().trim_start_matches('%');
    Ok((lbl, &rest[comma + 1..]))
}

fn parse_label_end(s: &str) -> Result<&str, SemError> {
    let s = s.trim();
    let rest = s
        .strip_prefix("label")
        .ok_or_else(|| SemError::Parse(format!("expected `label` in `br`, got `{s}`")))?
        .trim();
    Ok(rest.trim_start_matches('%'))
}

fn parse_phi_incoming(s: &str) -> Result<Vec<(String, String)>, SemError> {
    let mut out = Vec::new();
    let mut rest = s.trim();
    while !rest.is_empty() {
        let lb = rest
            .find('[')
            .ok_or_else(|| SemError::Parse(format!("expected `[` in phi incoming `{rest}`")))?;
        let rb = rest[lb + 1..]
            .find(']')
            .ok_or_else(|| SemError::Parse(format!("unclosed `[` in phi incoming `{rest}`")))?
            + lb
            + 1;
        let inner = &rest[lb + 1..rb];
        let comma = inner
            .rfind(',')
            .ok_or_else(|| SemError::Parse(format!("expected `,` in phi pair `[{inner}]`")))?;
        let val = inner[..comma].trim().to_string();
        let lbl = inner[comma + 1..].trim().trim_start_matches('%').to_string();
        out.push((val, lbl));
        rest = rest[rb + 1..].trim_start();
        if let Some(after_comma) = rest.strip_prefix(',') {
            rest = after_comma.trim_start();
        }
    }
    Ok(out)
}

fn merge_guarded_semvals(
    g: &mut SemGraph,
    items: &[(NodeId, SemVal)],
) -> Result<SemVal, SemError> {
    if items.len() == 1 {
        return Ok(items[0].1.clone());
    }
    let mut acc = items.last().unwrap().1.clone();
    for (cond, val) in items[..items.len() - 1].iter().rev() {
        acc = match (val, &acc) {
            (
                SemVal::Scalar { val: tv, poison: tp },
                SemVal::Scalar { val: ev, poison: ep },
            ) => SemVal::Scalar {
                val: g.ite(*cond, *tv, *ev),
                poison: g.ite(*cond, *tp, *ep),
            },
            (
                SemVal::Vector { elem_ty, lanes: tl },
                SemVal::Vector { lanes: el, .. },
            ) if tl.len() == el.len() => {
                let lanes = tl
                    .iter()
                    .zip(el.iter())
                    .map(|(&(tv, tp), &(ev, ep))| (g.ite(*cond, tv, ev), g.ite(*cond, tp, ep)))
                    .collect();
                SemVal::Vector { elem_ty: *elem_ty, lanes }
            }
            (SemVal::Tuple(ts), SemVal::Tuple(es)) if ts.len() == es.len() => {
                let mut merged = Vec::with_capacity(ts.len());
                for (t_item, e_item) in ts.iter().zip(es.iter()) {
                    let t_true = g.bool_const(true);
                    merged.push(merge_guarded_semvals(
                        g,
                        &[(*cond, t_item.clone()), (t_true, e_item.clone())],
                    )?);
                }
                SemVal::Tuple(merged)
            }
            _ => {
                return Err(SemError::Parse(
                    "mismatched value shapes across CFG merge/return".to_string(),
                ))
            }
        };
    }
    Ok(acc)
}

fn bind_dst(
    env: &mut HashMap<String, (LlvmTy, SemVal)>,
    dst: Option<String>,
    ty: LlvmTy,
    val: SemVal,
) -> Result<(), SemError> {
    if let Some(name) = dst {
        env.insert(name, (ty, val));
    }
    Ok(())
}

#[derive(Clone, Copy, Default)]
struct ArithFlags {
    nuw: bool,
    nsw: bool,
    exact: bool,
}

fn parse_arith_flags(mut s: &str) -> (ArithFlags, &str) {
    let mut f = ArithFlags::default();
    loop {
        let (w, rest) = split_head_word(s);
        match w {
            "nuw" => {
                f.nuw = true;
                s = rest;
            }
            "nsw" => {
                f.nsw = true;
                s = rest;
            }
            "exact" => {
                f.exact = true;
                s = rest;
            }
            _ => break,
        }
    }
    (f, s)
}

fn skip_fast_math_flags(mut s: &str) -> &str {
    loop {
        let (w, rest) = split_head_word(s);
        match w {
            "nnan" | "ninf" | "nsz" | "arcp" | "contract" | "afn" | "reassoc" | "fast" => {
                s = rest;
            }
            _ => break,
        }
    }
    s
}

fn lower_int_binop(
    g: &mut SemGraph,
    ub_cond: &mut NodeId,
    op: &str,
    flags: ArithFlags,
    ty: &LlvmTy,
    v1: &SemVal,
    v2: &SemVal,
) -> Result<SemVal, SemError> {
    match (ty, v1, v2) {
        (
            LlvmTy::Int(w),
            SemVal::Scalar { val: a, poison: pa },
            SemVal::Scalar { val: b, poison: pb },
        ) => {
            let (val, poison) = lower_scalar_int_binop(g, ub_cond, op, flags, *w, *a, *pa, *b, *pb)?;
            Ok(SemVal::Scalar { val, poison })
        }
        (
            LlvmTy::Vec(_, elem),
            SemVal::Vector { elem_ty, lanes: l1 },
            SemVal::Vector { lanes: l2, .. },
        ) => {
            let LlvmTy::Int(w) = **elem else {
                return Err(SemError::Parse(format!("non-integer vector for `{op}`")));
            };
            let mut out = Vec::with_capacity(l1.len());
            for (&(a, pa), &(b, pb)) in l1.iter().zip(l2.iter()) {
                out.push(lower_scalar_int_binop(g, ub_cond, op, flags, w, a, pa, b, pb)?);
            }
            Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
        }
        _ => Err(SemError::Parse(format!("type mismatch for integer binop `{op}`"))),
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_scalar_int_binop(
    g: &mut SemGraph,
    ub_cond: &mut NodeId,
    op: &str,
    flags: ArithFlags,
    w: u16,
    a: NodeId,
    pa: NodeId,
    b: NodeId,
    pb: NodeId,
) -> Result<(NodeId, NodeId), SemError> {
    let mut poison = g.or(pa, pb);
    let val = match op {
        "add" => {
            let r = g.bv_add(a, b);
            if flags.nuw {
                let ov = g.bv_ult(r, a);
                poison = g.or(poison, ov);
            }
            if flags.nsw {
                let ov = signed_add_overflow(g, w, a, b, r);
                poison = g.or(poison, ov);
            }
            r
        }
        "sub" => {
            let r = g.bv_sub(a, b);
            if flags.nuw {
                let ov = g.bv_ult(a, b);
                poison = g.or(poison, ov);
            }
            if flags.nsw {
                let ov = signed_sub_overflow(g, w, a, b, r);
                poison = g.or(poison, ov);
            }
            r
        }
        "mul" => {
            let r = g.bv_mul(a, b);
            if flags.nuw {
                let a_ext = g.bv_zext(a, w * 2);
                let b_ext = g.bv_zext(b, w * 2);
                let wide = g.bv_mul(a_ext, b_ext);
                let hi = g.bv_extract(wide, w * 2 - 1, w);
                let zero = g.bv_const(w, 0);
                let nz = g.bv_eq(hi, zero);
                let ov = g.not(nz);
                poison = g.or(poison, ov);
            }
            if flags.nsw {
                let a_ext = g.bv_sext(a, w * 2);
                let b_ext = g.bv_sext(b, w * 2);
                let wide = g.bv_mul(a_ext, b_ext);
                let r_ext = g.bv_sext(r, w * 2);
                let eq = g.bv_eq(wide, r_ext);
                let ov = g.not(eq);
                poison = g.or(poison, ov);
            }
            r
        }
        "and" => g.bv_and(a, b),
        "or" => g.bv_or(a, b),
        "xor" => g.bv_xor(a, b),
        "shl" => {
            let w_const = g.bv_const(w, w as u128);
            let lt = g.bv_ult(b, w_const);
            let oob = g.not(lt);
            poison = g.or(poison, oob);
            g.bv_shl(a, b)
        }
        "lshr" => {
            let w_const = g.bv_const(w, w as u128);
            let lt = g.bv_ult(b, w_const);
            let oob = g.not(lt);
            poison = g.or(poison, oob);
            let r = g.bv_lshr(a, b);
            if flags.exact {
                let back = g.bv_shl(r, b);
                let eq = g.bv_eq(back, a);
                let not_eq = g.not(eq);
                poison = g.or(poison, not_eq);
            }
            r
        }
        "ashr" => {
            let w_const = g.bv_const(w, w as u128);
            let lt = g.bv_ult(b, w_const);
            let oob = g.not(lt);
            poison = g.or(poison, oob);
            let r = g.bv_ashr(a, b);
            if flags.exact {
                let back = g.bv_shl(r, b);
                let eq = g.bv_eq(back, a);
                let not_eq = g.not(eq);
                poison = g.or(poison, not_eq);
            }
            r
        }
        "udiv" | "urem" => {
            // Immediate UB if divisor is 0 OR divisor is poison!
            let zero = g.bv_const(w, 0);
            let div_zero = g.bv_eq(b, zero);
            let ub = g.or(div_zero, pb);
            *ub_cond = g.or(*ub_cond, ub);
            if op == "udiv" {
                let r = g.bv_udiv(a, b);
                if flags.exact {
                    let rem = g.bv_urem(a, b);
                    let rem_zero = g.bv_eq(rem, zero);
                    let not_exact = g.not(rem_zero);
                    poison = g.or(poison, not_exact);
                }
                r
            } else {
                g.bv_urem(a, b)
            }
        }
        "sdiv" | "srem" => {
            // Immediate UB if divisor == 0, or (a == INT_MIN && b == -1), or divisor is poison!
            let zero = g.bv_const(w, 0);
            let div_zero = g.bv_eq(b, zero);
            let int_min = g.bv_const(w, 1u128 << (w - 1));
            let neg_one = g.bv_const(w, SemGraph::mask(w));
            let is_min = g.bv_eq(a, int_min);
            let is_m1 = g.bv_eq(b, neg_one);
            let ov = g.and(is_min, is_m1);
            let ub1 = g.or(div_zero, ov);
            let ub = g.or(ub1, pb);
            *ub_cond = g.or(*ub_cond, ub);
            if op == "sdiv" {
                let r = g.bv_sdiv(a, b);
                if flags.exact {
                    let rem = g.bv_srem(a, b);
                    let rem_zero = g.bv_eq(rem, zero);
                    let not_exact = g.not(rem_zero);
                    poison = g.or(poison, not_exact);
                }
                r
            } else {
                g.bv_srem(a, b)
            }
        }
        _ => unreachable!(),
    };
    Ok((val, poison))
}

fn signed_add_overflow(g: &mut SemGraph, w: u16, a: NodeId, b: NodeId, sum: NodeId) -> NodeId {
    let sa = g.bv_extract(a, w - 1, w - 1);
    let sb = g.bv_extract(b, w - 1, w - 1);
    let sr = g.bv_extract(sum, w - 1, w - 1);
    let same_in = g.bv_eq(sa, sb);
    let diff_out = g.bv_eq(sa, sr);
    let diff_out = g.not(diff_out);
    g.and(same_in, diff_out)
}

fn signed_sub_overflow(g: &mut SemGraph, w: u16, a: NodeId, b: NodeId, diff: NodeId) -> NodeId {
    let sa = g.bv_extract(a, w - 1, w - 1);
    let sb = g.bv_extract(b, w - 1, w - 1);
    let sr = g.bv_extract(diff, w - 1, w - 1);
    let diff_in = g.bv_eq(sa, sb);
    let diff_in = g.not(diff_in);
    let diff_out = g.bv_eq(sa, sr);
    let diff_out = g.not(diff_out);
    g.and(diff_in, diff_out)
}

fn lower_fp_binop(g: &mut SemGraph, op: &str, v1: &SemVal, v2: &SemVal) -> Result<SemVal, SemError> {
    let (a, pa) = v1.as_scalar().ok_or_else(|| SemError::Unsupported(format!("vector `{op}`")))?;
    let (b, pb) = v2.as_scalar().ok_or_else(|| SemError::Unsupported(format!("vector `{op}`")))?;
    let val = match op {
        "fadd" => g.fp_add(a, b),
        "fsub" => g.fp_sub(a, b),
        "fmul" => g.fp_mul(a, b),
        "fdiv" => g.fp_div(a, b),
        _ => unreachable!(),
    };
    let poison = g.or(pa, pb);
    Ok(SemVal::Scalar { val, poison })
}

fn lower_icmp(
    g: &mut SemGraph,
    pred: &str,
    ty: &LlvmTy,
    v1: &SemVal,
    v2: &SemVal,
) -> Result<(LlvmTy, SemVal), SemError> {
    let cmp_one = |g: &mut SemGraph, a: NodeId, b: NodeId| -> Result<NodeId, SemError> {
        let bool_res = match pred {
            "eq" => g.bv_eq(a, b),
            "ne" => {
                let eq = g.bv_eq(a, b);
                g.not(eq)
            }
            "ult" => g.bv_ult(a, b),
            "ule" => g.bv_ule(a, b),
            "ugt" => g.bv_ult(b, a),
            "uge" => g.bv_ule(b, a),
            "slt" => g.bv_slt(a, b),
            "sle" => g.bv_sle(a, b),
            "sgt" => g.bv_slt(b, a),
            "sge" => g.bv_sle(b, a),
            other => return Err(SemError::Parse(format!("unknown icmp predicate `{other}`"))),
        };
        Ok(g.bool_to_bv1(bool_res))
    };

    match (ty, v1, v2) {
        (
            LlvmTy::Int(_),
            SemVal::Scalar { val: a, poison: pa },
            SemVal::Scalar { val: b, poison: pb },
        ) => {
            let val = cmp_one(g, *a, *b)?;
            let poison = g.or(*pa, *pb);
            Ok((LlvmTy::Int(1), SemVal::Scalar { val, poison }))
        }
        (
            LlvmTy::Vec(n, _),
            SemVal::Vector { lanes: l1, .. },
            SemVal::Vector { lanes: l2, .. },
        ) => {
            let mut out = Vec::with_capacity(l1.len());
            for (&(a, pa), &(b, pb)) in l1.iter().zip(l2.iter()) {
                let val = cmp_one(g, a, b)?;
                let poison = g.or(pa, pb);
                out.push((val, poison));
            }
            Ok((
                LlvmTy::Vec(*n, Box::new(LlvmTy::Int(1))),
                SemVal::Vector { elem_ty: SemTy::Bv(1), lanes: out },
            ))
        }
        _ => Err(SemError::Parse("type mismatch in icmp".to_string())),
    }
}

fn lower_select(
    g: &mut SemGraph,
    vc: &SemVal,
    vt: &SemVal,
    vf: &SemVal,
) -> Result<SemVal, SemError> {
    match (vc, vt, vf) {
        (
            SemVal::Scalar { val: c, poison: pc },
            SemVal::Scalar { val: t, poison: pt },
            SemVal::Scalar { val: f, poison: pf },
        ) => {
            let cb = g.bv1_to_bool(*c);
            let val = g.ite(cb, *t, *f);
            // Exact LLVM select poison rule: only the selected arm's poison propagates!
            let arm_poison = g.ite(cb, *pt, *pf);
            let poison = g.or(*pc, arm_poison);
            Ok(SemVal::Scalar { val, poison })
        }
        (
            SemVal::Vector { lanes: lc, .. },
            SemVal::Vector { elem_ty, lanes: lt },
            SemVal::Vector { lanes: lf, .. },
        ) => {
            let mut out = Vec::with_capacity(lt.len());
            for ((&(c, pc), &(t, pt)), &(f, pf)) in lc.iter().zip(lt.iter()).zip(lf.iter()) {
                let cb = g.bv1_to_bool(c);
                let val = g.ite(cb, t, f);
                let arm_p = g.ite(cb, pt, pf);
                let poison = g.or(pc, arm_p);
                out.push((val, poison));
            }
            Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
        }
        _ => Err(SemError::Parse("select operand shape mismatch".to_string())),
    }
}

fn lower_cast(
    g: &mut SemGraph,
    op: &str,
    src_ty: &LlvmTy,
    dst_ty: &LlvmTy,
    v: &SemVal,
) -> Result<SemVal, SemError> {
    match op {
        "zext" | "sext" | "trunc" => match (dst_ty, v) {
            (LlvmTy::Int(to), SemVal::Scalar { val, poison }) => {
                let r = match op {
                    "zext" => g.bv_zext(*val, *to),
                    "sext" => g.bv_sext(*val, *to),
                    "trunc" => g.bv_trunc(*val, *to),
                    _ => unreachable!(),
                };
                Ok(SemVal::Scalar { val: r, poison: *poison })
            }
            (LlvmTy::Vec(_, elem), SemVal::Vector { lanes, .. }) => {
                let LlvmTy::Int(to) = **elem else {
                    return Err(SemError::Parse(format!("non-int vector target for `{op}`")));
                };
                let mut out = Vec::with_capacity(lanes.len());
                for &(val, poison) in lanes {
                    let r = match op {
                        "zext" => g.bv_zext(val, to),
                        "sext" => g.bv_sext(val, to),
                        "trunc" => g.bv_trunc(val, to),
                        _ => unreachable!(),
                    };
                    out.push((r, poison));
                }
                Ok(SemVal::Vector { elem_ty: SemTy::Bv(to), lanes: out })
            }
            _ => Err(SemError::Parse(format!("invalid `{op}` from {src_ty:?} to {dst_ty:?}"))),
        },
        "bitcast" => lower_bitcast(g, src_ty, dst_ty, v),
        "sitofp" | "uitofp" => {
            let (val, poison) = v.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{op}` not modeled"))
            })?;
            let fmt = match dst_ty {
                LlvmTy::Float => FpFmt::F32,
                LlvmTy::Double => FpFmt::F64,
                _ => return Err(SemError::Parse(format!("invalid `{op}` target"))),
            };
            let r = if op == "sitofp" {
                g.bv_to_fp_signed(fmt, val)
            } else {
                g.bv_to_fp_unsigned(fmt, val)
            };
            Ok(SemVal::Scalar { val: r, poison })
        }
        "fptosi" | "fptoui" => {
            let (val, p0) = v.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{op}` not modeled"))
            })?;
            let LlvmTy::Int(w) = *dst_ty else {
                return Err(SemError::Parse(format!("invalid `{op}` target")));
            };
            // Exact LLVM LangRef semantics: NaN, +/-Inf, or out-of-range produces poison!
            let (r, fits) = if op == "fptosi" {
                (g.fp_to_bv_signed(w, val), g.fp_fits_signed(w, val))
            } else {
                (g.fp_to_bv_unsigned(w, val), g.fp_fits_unsigned(w, val))
            };
            let oob = g.not(fits);
            let poison = g.or(p0, oob);
            Ok(SemVal::Scalar { val: r, poison })
        }
        "fpext" | "fptrunc" => {
            let (val, poison) = v.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{op}` not modeled"))
            })?;
            let fmt = match dst_ty {
                LlvmTy::Float => FpFmt::F32,
                LlvmTy::Double => FpFmt::F64,
                _ => return Err(SemError::Parse(format!("invalid `{op}` target"))),
            };
            let r = g.fp_to_fp(fmt, val);
            Ok(SemVal::Scalar { val: r, poison })
        }
        _ => unreachable!(),
    }
}

fn lower_bitcast(
    g: &mut SemGraph,
    src_ty: &LlvmTy,
    dst_ty: &LlvmTy,
    v: &SemVal,
) -> Result<SemVal, SemError> {
    if src_ty.bit_width() != dst_ty.bit_width() {
        return Err(SemError::Parse(format!(
            "bitcast width mismatch: {src_ty:?} to {dst_ty:?}"
        )));
    }
    match (src_ty, dst_ty, v) {
        (LlvmTy::Float | LlvmTy::Double, LlvmTy::Int(_), SemVal::Scalar { val, poison }) => {
            let bits = g.fp_to_bv_bits(*val);
            Ok(SemVal::Scalar { val: bits, poison: *poison })
        }
        (LlvmTy::Int(_), LlvmTy::Float, SemVal::Scalar { val, poison }) => {
            let fp = g.bv_bits_to_fp(FpFmt::F32, *val);
            Ok(SemVal::Scalar { val: fp, poison: *poison })
        }
        (LlvmTy::Int(_), LlvmTy::Double, SemVal::Scalar { val, poison }) => {
            let fp = g.bv_bits_to_fp(FpFmt::F64, *val);
            Ok(SemVal::Scalar { val: fp, poison: *poison })
        }
        // Little-endian lane packing: lane 0 occupies the least-significant bits
        // (matches Prim.Wide's explicit contract and x86_64/aarch64 data layout).
        (LlvmTy::Int(_), LlvmTy::Vec(n, elem), SemVal::Scalar { val, poison }) => {
            let LlvmTy::Int(ew) = **elem else {
                return Err(SemError::Unsupported("bitcast to non-int vector".to_string()));
            };
            let mut lanes = Vec::with_capacity(*n as usize);
            for i in 0..*n {
                let lo = i * ew;
                let hi = lo + ew - 1;
                let lane = g.bv_extract(*val, hi, lo);
                lanes.push((lane, *poison));
            }
            Ok(SemVal::Vector { elem_ty: SemTy::Bv(ew), lanes })
        }
        (LlvmTy::Vec(_, _), LlvmTy::Int(_), SemVal::Vector { lanes, .. }) => {
            // Concatenate lanes from MSB (highest index) down to LSB (lane 0).
            // In LLVM, bitcasting a vector containing any poison lane to a scalar integer
            // produces poison.
            let mut acc = lanes.last().unwrap().0;
            let mut poison = lanes.last().unwrap().1;
            for &(lv, lp) in lanes.iter().rev().skip(1) {
                acc = g.bv_concat(acc, lv);
                poison = g.or(poison, lp);
            }
            Ok(SemVal::Scalar { val: acc, poison })
        }
        (LlvmTy::Int(_), LlvmTy::Int(_), SemVal::Scalar { val, poison }) => {
            Ok(SemVal::Scalar { val: *val, poison: *poison })
        }
        _ => Err(SemError::Unsupported(format!(
            "bitcast from {src_ty:?} to {dst_ty:?} not yet modeled"
        ))),
    }
}

fn lower_insertelement(
    g: &mut SemGraph,
    v_ty: &LlvmTy,
    vv: &SemVal,
    ve: &SemVal,
    _i_ty: &LlvmTy,
    i_op: &Operand,
) -> Result<SemVal, SemError> {
    let LlvmTy::Vec(n, _) = *v_ty else {
        return Err(SemError::Parse("insertelement requires vector operand".to_string()));
    };
    let SemVal::Vector { elem_ty, lanes } = vv else {
        return Err(SemError::Parse("insertelement requires vector value".to_string()));
    };
    let (ev, ep) = ve.as_scalar().ok_or_else(|| {
        SemError::Parse("insertelement requires scalar element".to_string())
    })?;
    match *i_op {
        Operand::Int(idx) => {
            let mut out = lanes.clone();
            if idx >= 0 && (idx as u16) < n {
                out[idx as usize] = (ev, ep);
                Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
            } else {
                // Out-of-bounds insertelement returns poison vector!
                let t = g.bool_const(true);
                let out = out.into_iter().map(|(v, _)| (v, t)).collect();
                Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
            }
        }
        _ => Err(SemError::Unsupported(
            "dynamic index insertelement not yet modeled".to_string(),
        )),
    }
}

fn lower_extractelement(
    g: &mut SemGraph,
    v_ty: &LlvmTy,
    vv: &SemVal,
    i_op: &Operand,
) -> Result<(LlvmTy, SemVal), SemError> {
    let LlvmTy::Vec(n, elem) = v_ty else {
        return Err(SemError::Parse("extractelement requires vector operand".to_string()));
    };
    let SemVal::Vector { lanes, .. } = vv else {
        return Err(SemError::Parse("extractelement requires vector value".to_string()));
    };
    match *i_op {
        Operand::Int(idx) => {
            if idx >= 0 && (idx as u16) < *n {
                let (v, p) = lanes[idx as usize];
                Ok(((**elem).clone(), SemVal::Scalar { val: v, poison: p }))
            } else {
                let z = default_zero_for_ty(g, elem)?;
                let t = g.bool_const(true);
                Ok(((**elem).clone(), SemVal::Scalar { val: z, poison: t }))
            }
        }
        _ => Err(SemError::Unsupported(
            "dynamic index extractelement not yet modeled".to_string(),
        )),
    }
}

fn lower_shufflevector(
    g: &mut SemGraph,
    v1_ty: &LlvmTy,
    v1: &SemVal,
    v2: &SemVal,
    m_ty: &LlvmTy,
    m_op: &Operand,
) -> Result<(LlvmTy, SemVal), SemError> {
    let LlvmTy::Vec(n, elem) = v1_ty else {
        return Err(SemError::Parse("shufflevector requires vector operands".to_string()));
    };
    let LlvmTy::Vec(out_n, _) = *m_ty else {
        return Err(SemError::Parse("shufflevector mask must be a vector".to_string()));
    };
    let (SemVal::Vector { elem_ty, lanes: l1 }, SemVal::Vector { lanes: l2, .. }) = (v1, v2) else {
        return Err(SemError::Parse("shufflevector operands must be vectors".to_string()));
    };

    let indices: Vec<Option<usize>> = match m_op {
        Operand::ZeroInit => vec![Some(0); out_n as usize],
        Operand::VecLit(items) => items
            .iter()
            .map(|(_, op)| match op {
                Operand::Int(k) if *k >= 0 => Ok(Some(*k as usize)),
                Operand::Poison | Operand::Undef => Ok(None),
                other => Err(SemError::Parse(format!("invalid shuffle mask element {other:?}"))),
            })
            .collect::<Result<_, _>>()?,
        other => {
            return Err(SemError::Parse(format!(
                "unsupported shufflevector mask {other:?}"
            )));
        }
    };

    let mut out = Vec::with_capacity(indices.len());
    for idx in indices {
        match idx {
            Some(k) if k < *n as usize => out.push(l1[k]),
            Some(k) if k < (*n as usize) * 2 => out.push(l2[k - *n as usize]),
            _ => {
                let z = default_zero_for_ty(g, elem)?;
                let t = g.bool_const(true);
                out.push((z, t));
            }
        }
    }
    Ok((
        LlvmTy::Vec(out_n, elem.clone()),
        SemVal::Vector { elem_ty: *elem_ty, lanes: out },
    ))
}

/// Lower pure register-only hardware crypto inline-asm templates (`pclmulqdq`, `pmull`,
/// `aesenc`/`aesenclast`/`aesdec`/`aesdeclast`, `aese`/`aesmc`/`aesd`/`aesimc`) into
/// exact `SemGraph` nodes (`BvClmul` / `AesCore`). Any `sideeffect`, memory clobber, or
/// unmodeled ISA instruction is strictly rejected.
fn lower_verified_crypto_asm(
    g: &mut SemGraph,
    env: &HashMap<String, (LlvmTy, SemVal)>,
    rest: &str,
) -> Result<(LlvmTy, SemVal), SemError> {
    let (ret_t, after_ret) = parse_ty(rest)?;
    let after_ret = after_ret.trim();
    if !after_ret.starts_with("asm ") {
        return Err(SemError::Unsupported(
            "inline `asm` call requires an ISA semantics plugin (e.g. Sail)".to_string(),
        ));
    }
    let after_asm = after_ret[4..].trim();
    if after_asm.starts_with("sideeffect ") || after_asm.contains("~{memory}") {
        return Err(SemError::Unsupported(
            "sideeffect/memory inline `asm` requires an ISA semantics plugin".to_string(),
        ));
    }
    // Extract the two quoted strings: `"<template>"`, `"<constraints>"`
    let mut quoted = Vec::new();
    let mut chars = after_asm.char_indices().peekable();
    let mut end_of_second_quote = 0usize;
    while let Some((idx, ch)) = chars.next() {
        if ch == '"' {
            let mut cur = String::new();
            while let Some((j, c2)) = chars.next() {
                if c2 == '"' {
                    end_of_second_quote = j + 1;
                    break;
                }
                cur.push(c2);
            }
            quoted.push(cur);
            if quoted.len() == 2 {
                break;
            }
        } else if ch == '(' && quoted.len() < 2 {
            break;
        }
        let _ = idx;
    }
    if quoted.len() < 2 {
        return Err(SemError::Unsupported(
            "malformed inline `asm` template/constraints".to_string(),
        ));
    }
    let tpl = quoted[0].trim();
    let constraints = quoted[1].trim();
    let args_tail = after_asm[end_of_second_quote..].trim();
    let lparen = args_tail.find('(').ok_or_else(|| {
        SemError::Parse(format!("missing `(` in inline asm `{rest}`"))
    })?;
    let rparen = args_tail.rfind(')').ok_or_else(|| {
        SemError::Parse(format!("missing `)` in inline asm `{rest}`"))
    })?;
    let call_args = parse_call_args(&args_tail[lparen + 1..rparen])?;
    let mut eval_args = Vec::with_capacity(call_args.len());
    for (aty, aop) in call_args {
        let av = eval_operand(g, env, &aty, &aop)?;
        eval_args.push((aty, aop, av));
    }

    // Helper: pack a 128-bit vector SemVal (<2 x i64> or <16 x i8>) into (Bv(128), poison).
    fn pack_vec128(g: &mut SemGraph, v: &SemVal) -> Result<(NodeId, NodeId), SemError> {
        let SemVal::Vector { lanes, .. } = v else {
            return Err(SemError::Unsupported("expected 128-bit vector for crypto asm".to_string()));
        };
        let mut acc = lanes[0].0;
        let mut p = lanes[0].1;
        for &(lv, lp) in &lanes[1..] {
            acc = g.bv_concat(lv, acc);
            p = g.or(p, lp);
        }
        Ok((acc, p))
    }

    // Helper: unpack a 128-bit (Bv(128), poison) into `<N x iW>`.
    fn unpack_vec128(
        g: &mut SemGraph,
        bits: NodeId,
        poison: NodeId,
        ret_t: &LlvmTy,
    ) -> Result<SemVal, SemError> {
        let LlvmTy::Vec(n, elem) = ret_t else {
            return Err(SemError::Unsupported("expected vector return type for crypto asm".to_string()));
        };
        let Some(SemTy::Bv(ew)) = elem.to_sem_ty() else {
            return Err(SemError::Unsupported("expected integer vector element for crypto asm".to_string()));
        };
        if (*n as u16) * ew != 128 {
            return Err(SemError::Unsupported("expected 128-bit vector return for crypto asm".to_string()));
        }
        let mut lanes = Vec::with_capacity(*n as usize);
        for i in 0..(*n as u16) {
            let lo = i * ew;
            let hi = lo + ew - 1;
            let lane = g.bv_extract(bits, hi, lo);
            lanes.push((lane, poison));
        }
        Ok(SemVal::Vector { elem_ty: SemTy::Bv(ew), lanes })
    }

    // Strip optional `.arch_extension aes\0A` prefix on aarch64 templates.
    let norm_tpl = tpl
        .strip_prefix(".arch_extension aes\\0A")
        .or_else(|| tpl.strip_prefix(".arch_extension aes\n"))
        .unwrap_or(tpl)
        .trim();

    // 1. x86_64 `pclmulqdq $$<imm>, $2, $0`, `"=x,0,x"` on `<2 x i64>`
    if norm_tpl.starts_with("pclmulqdq ") && constraints == "=x,0,x" && eval_args.len() == 2 {
        let imm_part = norm_tpl
            .strip_prefix("pclmulqdq ")
            .unwrap()
            .split(',')
            .next()
            .unwrap_or("")
            .trim();
        let imm_str = imm_part
            .strip_prefix("$$")
            .or_else(|| imm_part.strip_prefix('$'))
            .unwrap_or(imm_part);
        if let Ok(imm) = imm_str.parse::<u8>() {
            if let (SemVal::Vector { lanes: la, .. }, SemVal::Vector { lanes: lb, .. }) =
                (&eval_args[0].2, &eval_args[1].2)
            {
                if la.len() == 2 && lb.len() == 2 {
                    let ai = (imm & 1) as usize;
                    let bi = ((imm >> 4) & 1) as usize;
                    let prod = g.bv_clmul(la[ai].0, lb[bi].0);
                    let p = g.or(la[ai].1, lb[bi].1);
                    return Ok((ret_t.clone(), unpack_vec128(g, prod, p, &ret_t)?));
                }
            }
        }
    }

    // 2. aarch64 `pmull $0.1q, $1.1d, $2.1d`, `"=w,w,w"` on `<1 x i64>, <1 x i64>` -> `<16 x i8>`
    if norm_tpl == "pmull $0.1q, $1.1d, $2.1d" && constraints == "=w,w,w" && eval_args.len() == 2 {
        if let (SemVal::Vector { lanes: la, .. }, SemVal::Vector { lanes: lb, .. }) =
            (&eval_args[0].2, &eval_args[1].2)
        {
            if la.len() == 1 && lb.len() == 1 {
                let prod = g.bv_clmul(la[0].0, lb[0].0);
                let p = g.or(la[0].1, lb[0].1);
                return Ok((ret_t.clone(), unpack_vec128(g, prod, p, &ret_t)?));
            }
        }
    }

    // 3. x86_64 AES-NI: `aesenc`/`aesenclast`/`aesdec`/`aesdeclast $2, $0`, `"=x,0,x"`
    let x86_aes_kind = match norm_tpl {
        "aesenc $2, $0" => Some(super::sem::AesRoundKind::EncCore),
        "aesenclast $2, $0" => Some(super::sem::AesRoundKind::EncLastCore),
        "aesdec $2, $0" => Some(super::sem::AesRoundKind::DecCore),
        "aesdeclast $2, $0" => Some(super::sem::AesRoundKind::DecLastCore),
        _ => None,
    };
    if let Some(kind) = x86_aes_kind {
        if constraints == "=x,0,x" && eval_args.len() == 2 {
            let (s_bits, sp) = pack_vec128(g, &eval_args[0].2)?;
            let (k_bits, kp) = pack_vec128(g, &eval_args[1].2)?;
            let core = g.aes_core(kind, s_bits);
            let out = g.bv_xor(core, k_bits);
            let p = g.or(sp, kp);
            return Ok((ret_t.clone(), unpack_vec128(g, out, p, &ret_t)?));
        }
    }

    // 4. aarch64 ARMv8 Crypto: `aese`[+`aesmc`] / `aesd`[+`aesimc`], `"=w,0,w"`
    let arm_aes_kind = match norm_tpl {
        "aese $0.16b, $2.16b\\0Aaesmc $0.16b, $0.16b"
        | "aese $0.16b, $2.16b\naesmc $0.16b, $0.16b" => Some(super::sem::AesRoundKind::EncCore),
        "aese $0.16b, $2.16b" => Some(super::sem::AesRoundKind::EncLastCore),
        "aesd $0.16b, $2.16b\\0Aaesimc $0.16b, $0.16b"
        | "aesd $0.16b, $2.16b\naesimc $0.16b, $0.16b" => Some(super::sem::AesRoundKind::DecCore),
        "aesd $0.16b, $2.16b" => Some(super::sem::AesRoundKind::DecLastCore),
        _ => None,
    };
    if let Some(kind) = arm_aes_kind {
        if constraints == "=w,0,w" && eval_args.len() == 2 {
            let (s_bits, sp) = pack_vec128(g, &eval_args[0].2)?;
            let (k0_bits, kp) = pack_vec128(g, &eval_args[1].2)?;
            let xored = g.bv_xor(s_bits, k0_bits);
            let out = g.aes_core(kind, xored);
            let p = g.or(sp, kp);
            return Ok((ret_t.clone(), unpack_vec128(g, out, p, &ret_t)?));
        }
    }

    Err(SemError::Unsupported(format!(
        "inline `asm` call `{tpl}` requires an ISA semantics plugin (e.g. Sail)"
    )))
}

fn lower_intrinsic(
    g: &mut SemGraph,
    sym: &str,
    ret_ty: &LlvmTy,
    args: &[(LlvmTy, Operand, SemVal)],
) -> Result<SemVal, SemError> {
    let parts: Vec<&str> = sym.split('.').collect();
    if parts.first() != Some(&"llvm") || parts.len() < 3 {
        return Err(SemError::Unsupported(format!("non-intrinsic call `@{sym}`")));
    }

    match parts[1] {
        "sadd" | "ssub" | "uadd" | "usub" | "smul" | "umul" if parts.get(2) == Some(&"sat") => {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector saturating intrinsic `{sym}`"))
            })?;
            let (b, pb) = args[1].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector saturating intrinsic `{sym}`"))
            })?;
            let w = g.bv_width(a);
            let poison = g.or(pa, pb);
            let val = match parts[1] {
                "uadd" => {
                    let s = g.bv_add(a, b);
                    let ov = g.bv_ult(s, a);
                    let max = g.bv_const(w, SemGraph::mask(w));
                    g.ite(ov, max, s)
                }
                "usub" => {
                    let d = g.bv_sub(a, b);
                    let uf = g.bv_ult(a, b);
                    let zero = g.bv_const(w, 0);
                    g.ite(uf, zero, d)
                }
                "sadd" => {
                    let s = g.bv_add(a, b);
                    let ov = signed_add_overflow(g, w, a, b, s);
                    let zero = g.bv_const(w, 0);
                    let neg = g.bv_slt(a, zero);
                    let min = g.bv_const(w, 1u128 << (w - 1));
                    let max = g.bv_const(w, (1u128 << (w - 1)) - 1);
                    let bound = g.ite(neg, min, max);
                    g.ite(ov, bound, s)
                }
                "ssub" => {
                    let d = g.bv_sub(a, b);
                    let ov = signed_sub_overflow(g, w, a, b, d);
                    let zero = g.bv_const(w, 0);
                    let neg = g.bv_slt(a, zero);
                    let min = g.bv_const(w, 1u128 << (w - 1));
                    let max = g.bv_const(w, (1u128 << (w - 1)) - 1);
                    let bound = g.ite(neg, min, max);
                    g.ite(ov, bound, d)
                }
                _ => unreachable!(),
            };
            Ok(SemVal::Scalar { val, poison })
        }
        "sadd" | "ssub" | "smul" | "uadd" | "usub" | "umul"
            if parts.get(2) == Some(&"with") && parts.get(3) == Some(&"overflow") =>
        {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("non-scalar `{sym}`"))
            })?;
            let (b, pb) = args[1].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("non-scalar `{sym}`"))
            })?;
            let w = g.bv_width(a);
            let poison = g.or(pa, pb);
            let (val, ov_bool) = match parts[1] {
                "uadd" => {
                    let s = g.bv_add(a, b);
                    (s, g.bv_ult(s, a))
                }
                "usub" => {
                    let d = g.bv_sub(a, b);
                    (d, g.bv_ult(a, b))
                }
                "sadd" => {
                    let s = g.bv_add(a, b);
                    let ov = signed_add_overflow(g, w, a, b, s);
                    (s, ov)
                }
                "ssub" => {
                    let d = g.bv_sub(a, b);
                    let ov = signed_sub_overflow(g, w, a, b, d);
                    (d, ov)
                }
                "umul" => {
                    let p = g.bv_mul(a, b);
                    let a_ext = g.bv_zext(a, w * 2);
                    let b_ext = g.bv_zext(b, w * 2);
                    let wide = g.bv_mul(a_ext, b_ext);
                    let hi = g.bv_extract(wide, w * 2 - 1, w);
                    let zero = g.bv_const(w, 0);
                    let z = g.bv_eq(hi, zero);
                    (p, g.not(z))
                }
                "smul" => {
                    let p = g.bv_mul(a, b);
                    let a_ext = g.bv_sext(a, w * 2);
                    let b_ext = g.bv_sext(b, w * 2);
                    let wide = g.bv_mul(a_ext, b_ext);
                    let p_ext = g.bv_sext(p, w * 2);
                    let eq = g.bv_eq(wide, p_ext);
                    (p, g.not(eq))
                }
                _ => unreachable!(),
            };
            let ov_bv1 = g.bool_to_bv1(ov_bool);
            Ok(SemVal::Tuple(vec![
                SemVal::Scalar { val, poison },
                SemVal::Scalar { val: ov_bv1, poison },
            ]))
        }
        "fshl" | "fshr" => {
            let is_left = parts[1] == "fshl";
            match (&args[0].2, &args[1].2, &args[2].2) {
                (
                    SemVal::Scalar { val: a, poison: pa },
                    SemVal::Scalar { val: b, poison: pb },
                    SemVal::Scalar { val: s, poison: ps },
                ) => {
                    let p1 = g.or(*pa, *pb);
                    let poison = g.or(p1, *ps);
                    let val = if is_left { g.bv_fshl(*a, *b, *s) } else { g.bv_fshr(*a, *b, *s) };
                    Ok(SemVal::Scalar { val, poison })
                }
                (
                    SemVal::Vector { elem_ty, lanes: la },
                    SemVal::Vector { lanes: lb, .. },
                    SemVal::Vector { lanes: ls, .. },
                ) => {
                    let mut out = Vec::with_capacity(la.len());
                    for ((&(a, pa), &(b, pb)), &(s, ps)) in la.iter().zip(lb.iter()).zip(ls.iter()) {
                        let p1 = g.or(pa, pb);
                        let poison = g.or(p1, ps);
                        let val = if is_left { g.bv_fshl(a, b, s) } else { g.bv_fshr(a, b, s) };
                        out.push((val, poison));
                    }
                    Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
                }
                _ => Err(SemError::Parse(format!("operand mismatch for `{sym}`"))),
            }
        }
        "ctlz" | "cttz" => {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{sym}`"))
            })?;
            let is_zero_poison = matches!(args.get(1).map(|x| &x.1), Some(Operand::Bool(true) | Operand::Int(1)));
            let w = g.bv_width(a);
            let mut poison = pa;
            if is_zero_poison {
                let zero = g.bv_const(w, 0);
                let is_z = g.bv_eq(a, zero);
                poison = g.or(poison, is_z);
            }
            let val = if parts[1] == "ctlz" { g.bv_ctlz(a) } else { g.bv_cttz(a) };
            Ok(SemVal::Scalar { val, poison })
        }
        "ctpop" | "bitreverse" | "bswap" => {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{sym}`"))
            })?;
            let val = match parts[1] {
                "ctpop" => g.bv_ctpop(a),
                "bitreverse" => g.bv_bitreverse(a),
                "bswap" => g.bv_bswap(a),
                _ => unreachable!(),
            };
            Ok(SemVal::Scalar { val, poison: pa })
        }
        "smin" | "smax" | "umin" | "umax" => {
            let signed = parts[1].starts_with('s');
            let is_min = parts[1].ends_with("min");
            let one = |g: &mut SemGraph, a: NodeId, pa: NodeId, b: NodeId, pb: NodeId| {
                let cmp = if signed { g.bv_slt(a, b) } else { g.bv_ult(a, b) };
                let val = if is_min { g.ite(cmp, a, b) } else { g.ite(cmp, b, a) };
                let poison = g.or(pa, pb);
                (val, poison)
            };
            match (&args[0].2, &args[1].2) {
                (
                    SemVal::Scalar { val: a, poison: pa },
                    SemVal::Scalar { val: b, poison: pb },
                ) => {
                    let (val, poison) = one(g, *a, *pa, *b, *pb);
                    Ok(SemVal::Scalar { val, poison })
                }
                (
                    SemVal::Vector { elem_ty, lanes: l1 },
                    SemVal::Vector { lanes: l2, .. },
                ) => {
                    let out = l1
                        .iter()
                        .zip(l2.iter())
                        .map(|(&(a, pa), &(b, pb))| one(g, a, pa, b, pb))
                        .collect();
                    Ok(SemVal::Vector { elem_ty: *elem_ty, lanes: out })
                }
                _ => Err(SemError::Parse(format!("operand mismatch for `{sym}`"))),
            }
        }
        "fabs" | "sqrt" | "floor" | "ceil" | "trunc" | "round" | "roundeven" => {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{sym}`"))
            })?;
            let op = match parts[1] {
                "fabs" => FpUnaryOp::Abs,
                "sqrt" => FpUnaryOp::Sqrt,
                "floor" => FpUnaryOp::Floor,
                "ceil" => FpUnaryOp::Ceil,
                "trunc" => FpUnaryOp::Trunc,
                "round" => FpUnaryOp::Round,
                "roundeven" => FpUnaryOp::RoundEven,
                _ => unreachable!(),
            };
            let val = g.fp_unary(op, a);
            Ok(SemVal::Scalar { val, poison: pa })
        }
        "copysign" => {
            let (a, pa) = args[0].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{sym}`"))
            })?;
            let (b, pb) = args[1].2.as_scalar().ok_or_else(|| {
                SemError::Unsupported(format!("vector `{sym}`"))
            })?;
            let val = g.fp_copysign(a, b);
            let poison = g.or(pa, pb);
            Ok(SemVal::Scalar { val, poison })
        }
        _ => {
            let _ = ret_ty;
            Err(SemError::Unsupported(format!(
                "intrinsic `@{sym}` is not modeled in {LLVM_SEMANTICS_ID}"
            )))
        }
    }
}

fn eval_operand(
    g: &mut SemGraph,
    env: &HashMap<String, (LlvmTy, SemVal)>,
    ty: &LlvmTy,
    op: &Operand,
) -> Result<SemVal, SemError> {
    match op {
        Operand::Reg(name) => env
            .get(name)
            .map(|(_, v)| v.clone())
            .ok_or_else(|| SemError::Parse(format!("undefined SSA register `{name}`"))),
        Operand::Int(v) => match *ty {
            LlvmTy::Int(w) => {
                let node = g.bv_const(w, *v as u128);
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            LlvmTy::Float => {
                let node = g.fp_const(FpFmt::F32, (*v as f32).to_bits() as u64);
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            LlvmTy::Double => {
                let node = g.fp_const(FpFmt::F64, (*v as f64).to_bits());
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            _ => Err(SemError::Parse(format!("integer literal at non-scalar type {ty:?}"))),
        },
        Operand::Bool(b) => match *ty {
            LlvmTy::Int(1) => {
                let node = g.bv_const(1, if *b { 1 } else { 0 });
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            _ => Err(SemError::Parse(format!("bool literal at type {ty:?}"))),
        },
        Operand::FloatHex(bits) => match *ty {
            LlvmTy::Double => {
                let node = g.fp_const(FpFmt::F64, *bits);
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            LlvmTy::Float => {
                // LLVM hex float literals in `float` position are IEEE-754 double bits narrowed to f32.
                let f = f64::from_bits(*bits) as f32;
                let node = g.fp_const(FpFmt::F32, f.to_bits() as u64);
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            _ => Err(SemError::Parse(format!("hex float literal at type {ty:?}"))),
        },
        Operand::FloatDec(s) => match *ty {
            LlvmTy::Float => {
                let f: f32 = s.parse().map_err(|_| SemError::Parse(format!("bad float `{s}`")))?;
                let node = g.fp_const(FpFmt::F32, f.to_bits() as u64);
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            LlvmTy::Double => {
                let f: f64 = s.parse().map_err(|_| SemError::Parse(format!("bad double `{s}`")))?;
                let node = g.fp_const(FpFmt::F64, f.to_bits());
                let p = g.bool_const(false);
                Ok(SemVal::Scalar { val: node, poison: p })
            }
            _ => Err(SemError::Parse(format!("decimal float literal at type {ty:?}"))),
        },
        Operand::Poison | Operand::Undef => {
            let p = g.bool_const(true);
            match ty {
                LlvmTy::Vec(n, elem) => {
                    let elem_ty = elem
                        .to_sem_ty()
                        .ok_or_else(|| SemError::Unsupported("nested vector".to_string()))?;
                    let z = default_zero_for_ty(g, elem)?;
                    Ok(SemVal::Vector { elem_ty, lanes: vec![(z, p); *n as usize] })
                }
                LlvmTy::Struct(fields) => {
                    let mut vals = Vec::with_capacity(fields.len());
                    for ft in fields {
                        vals.push(eval_operand(g, env, ft, op)?);
                    }
                    Ok(SemVal::Tuple(vals))
                }
                _ => {
                    let z = default_zero_for_ty(g, ty)?;
                    Ok(SemVal::Scalar { val: z, poison: p })
                }
            }
        }
        Operand::ZeroInit => {
            let p = g.bool_const(false);
            match ty {
                LlvmTy::Vec(n, elem) => {
                    let elem_ty = elem
                        .to_sem_ty()
                        .ok_or_else(|| SemError::Unsupported("nested vector".to_string()))?;
                    let z = default_zero_for_ty(g, elem)?;
                    Ok(SemVal::Vector { elem_ty, lanes: vec![(z, p); *n as usize] })
                }
                LlvmTy::Struct(fields) => {
                    let mut vals = Vec::with_capacity(fields.len());
                    for ft in fields {
                        vals.push(eval_operand(g, env, ft, op)?);
                    }
                    Ok(SemVal::Tuple(vals))
                }
                _ => {
                    let z = default_zero_for_ty(g, ty)?;
                    Ok(SemVal::Scalar { val: z, poison: p })
                }
            }
        }
        Operand::VecLit(items) => {
            let LlvmTy::Vec(_, elem) = ty else {
                return Err(SemError::Parse("vector literal at non-vector type".to_string()));
            };
            let elem_ty = elem
                .to_sem_ty()
                .ok_or_else(|| SemError::Unsupported("nested vector".to_string()))?;
            let mut lanes = Vec::with_capacity(items.len());
            for (ity, iop) in items {
                let v = eval_operand(g, env, ity, iop)?;
                let (val, p) = v.as_scalar().ok_or_else(|| {
                    SemError::Parse("nested vector element".to_string())
                })?;
                lanes.push((val, p));
            }
            Ok(SemVal::Vector { elem_ty, lanes })
        }
    }
}

fn default_zero_for_ty(g: &mut SemGraph, ty: &LlvmTy) -> Result<NodeId, SemError> {
    match *ty {
        LlvmTy::Int(w) => Ok(g.bv_const(w, 0)),
        LlvmTy::Float => Ok(g.fp_const(FpFmt::F32, 0)),
        LlvmTy::Double => Ok(g.fp_const(FpFmt::F64, 0)),
        _ => Err(SemError::Parse(format!("cannot create zero constant for {ty:?}"))),
    }
}

fn parse_fp_pred(p: &str) -> Result<FpPred, SemError> {
    match p {
        "false" => Ok(FpPred::False),
        "oeq" => Ok(FpPred::Oeq),
        "ogt" => Ok(FpPred::Ogt),
        "oge" => Ok(FpPred::Oge),
        "olt" => Ok(FpPred::Olt),
        "ole" => Ok(FpPred::Ole),
        "one" => Ok(FpPred::One),
        "ord" => Ok(FpPred::Ord),
        "uno" => Ok(FpPred::Uno),
        "ueq" => Ok(FpPred::Ueq),
        "ugt" => Ok(FpPred::Ugt),
        "uge" => Ok(FpPred::Uge),
        "ult" => Ok(FpPred::Ult),
        "ule" => Ok(FpPred::Ule),
        "une" => Ok(FpPred::Une),
        "true" => Ok(FpPred::True),
        other => Err(SemError::Parse(format!("unknown fcmp predicate `{other}`"))),
    }
}

// ------------------------------------------------------------------ Parsing helpers

fn split_head_word(s: &str) -> (&str, &str) {
    let s = s.trim_start();
    match s.find(|c: char| c.is_whitespace()) {
        Some(i) => (&s[..i], s[i..].trim_start()),
        None => (s, ""),
    }
}

fn parse_ty(s: &str) -> Result<(LlvmTy, &str), SemError> {
    let s = s.trim_start();
    if let Some(rest) = s.strip_prefix('<') {
        let end = find_matching(rest, '<', '>').ok_or_else(|| {
            SemError::Parse(format!("unclosed vector type `{s}`"))
        })?;
        let inner = rest[..end].trim();
        let after = rest[end + 1..].trim_start();
        let x_pos = inner.find('x').ok_or_else(|| {
            SemError::Parse(format!("expected `x` in vector type `<{inner}>`"))
        })?;
        let n: u16 = inner[..x_pos].trim().parse().map_err(|_| {
            SemError::Parse(format!("invalid vector length in `<{inner}>`"))
        })?;
        let (elem, _) = parse_ty(inner[x_pos + 1..].trim())?;
        return Ok((LlvmTy::Vec(n, Box::new(elem)), after));
    }
    if let Some(rest) = s.strip_prefix('{') {
        let end = find_matching(rest, '{', '}').ok_or_else(|| {
            SemError::Parse(format!("unclosed struct type `{s}`"))
        })?;
        let inner = rest[..end].trim();
        let after = rest[end + 1..].trim_start();
        let mut fields = Vec::new();
        for part in split_top_level_commas(inner) {
            let (ft, _) = parse_ty(part.trim())?;
            fields.push(ft);
        }
        return Ok((LlvmTy::Struct(fields), after));
    }
    let end = s
        .find(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '>')
        .unwrap_or(s.len());
    let tok = &s[..end];
    let after = s[end..].trim_start();
    if tok == "float" {
        return Ok((LlvmTy::Float, after));
    }
    if tok == "double" {
        return Ok((LlvmTy::Double, after));
    }
    if let Some(bits) = tok.strip_prefix('i') {
        if let Ok(w) = bits.parse::<u16>() {
            if w > 0 && w <= 128 {
                return Ok((LlvmTy::Int(w), after));
            }
        }
    }
    Err(SemError::Unsupported(format!("unsupported LLVM type `{tok}`")))
}

fn find_matching(s: &str, open: char, close: char) -> Option<usize> {
    let mut depth = 1usize;
    for (i, ch) in s.char_indices() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, ch) in s.char_indices() {
        match ch {
            '<' | '{' | '(' => depth += 1,
            '>' | '}' | ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                out.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if !s[start..].trim().is_empty() {
        out.push(&s[start..]);
    }
    out
}

fn parse_two_operands(s: &str) -> Result<(Operand, Operand), SemError> {
    let parts = split_top_level_commas(s);
    if parts.len() != 2 {
        return Err(SemError::Parse(format!("expected 2 operands, got `{s}`")));
    }
    Ok((parse_operand(parts[0].trim())?, parse_operand(parts[1].trim())?))
}

fn parse_operand_until_comma(s: &str) -> Result<(Operand, &str), SemError> {
    let mut depth = 0usize;
    for (i, ch) in s.char_indices() {
        match ch {
            '<' | '{' | '(' => depth += 1,
            '>' | '}' | ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                let op = parse_operand(s[..i].trim())?;
                return Ok((op, s[i + 1..].trim_start()));
            }
            _ => {}
        }
    }
    Err(SemError::Parse(format!("expected `,` after operand in `{s}`")))
}

fn split_on_keyword_to(s: &str) -> Result<(&str, &str), SemError> {
    let mut depth = 0usize;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'<' | b'{' | b'(' => depth += 1,
            b'>' | b'}' | b')' => depth = depth.saturating_sub(1),
            _ if depth == 0 && s[i..].starts_with(" to ") => {
                return Ok((&s[..i], &s[i + 4..]));
            }
            _ => {}
        }
    }
    Err(SemError::Parse(format!("expected `to` in cast `{s}`")))
}

fn parse_call_args(s: &str) -> Result<Vec<(LlvmTy, Operand)>, SemError> {
    let mut out = Vec::new();
    for part in split_top_level_commas(s) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (ty, rest) = parse_ty(part)?;
        let op = parse_operand(rest.trim())?;
        out.push((ty, op));
    }
    Ok(out)
}

fn parse_operand(s: &str) -> Result<Operand, SemError> {
    let s = s.trim();
    if s.starts_with('%') {
        return Ok(Operand::Reg(s.to_string()));
    }
    if s == "poison" {
        return Ok(Operand::Poison);
    }
    if s == "undef" {
        return Ok(Operand::Undef);
    }
    if s == "zeroinitializer" {
        return Ok(Operand::ZeroInit);
    }
    if s == "true" {
        return Ok(Operand::Bool(true));
    }
    if s == "false" {
        return Ok(Operand::Bool(false));
    }
    if let Some(rest) = s.strip_prefix('<') {
        let inner = rest.strip_suffix('>').ok_or_else(|| {
            SemError::Parse(format!("unclosed vector literal `{s}`"))
        })?;
        let mut items = Vec::new();
        for part in split_top_level_commas(inner) {
            let (ty, rest) = parse_ty(part.trim())?;
            let op = parse_operand(rest.trim())?;
            items.push((ty, op));
        }
        return Ok(Operand::VecLit(items));
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let bits = u64::from_str_radix(hex, 16).map_err(|_| {
            SemError::Parse(format!("invalid hex float literal `{s}`"))
        })?;
        return Ok(Operand::FloatHex(bits));
    }
    if let Ok(v) = s.parse::<i128>() {
        return Ok(Operand::Int(v));
    }
    if s.contains('.') || s.contains('e') || s.contains('E') {
        return Ok(Operand::FloatDec(s.to_string()));
    }
    Err(SemError::Parse(format!("unrecognized LLVM operand `{s}`")))
}
