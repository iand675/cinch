//! Bridge from real Cinch `.cinch` source modules (`Decl::Prim`) into
//! backend-agnostic verification `Obligation`s.
//!
//! Uses Cinch's own lexer, layout pass, and parser so real prelude files
//! (`seed/prelude/Prim/Machine.cinch`, `seed/prelude/Prim/Wide.cinch`, etc.)
//! are audited directly without hand-transcribing IR bodies.

use super::sem::{FpFmt, NodeId, Obligation, ObligationKind, SemGraph, SemParam, SemProgram, SemTy};
use super::{IrSemantics, SemError};
use crate::ast::{Assoc, Decl, PrimBody, PrimDecl, Term, Type};
use crate::parser::FixityTable;
use std::collections::HashMap;
use std::path::Path;

/// One extracted `prim` body ready for verification (or flagged as skipped/unsafe).
#[derive(Clone, Debug)]
pub struct ExtractedPrimBody {
    pub module: String,
    pub prim_name: String,
    pub body_index: usize,
    pub family: String,
    pub is_unsafe: bool,
    pub when_desc: String,
    pub obligations: Vec<Obligation>,
    pub skip_reason: Option<String>,
}

#[derive(Clone, Copy, Debug)]
struct ScalarInfo {
    /// LLVM ABI type width for `Bool` (which crosses Cinch `prim` ABI as `i8`).
    llvm_param_ty: SemTy,
    signed: bool,
}

fn default_fixities() -> FixityTable {
    let mut f = FixityTable::new();
    let _ = f.insert("||", Assoc::Right, 2);
    let _ = f.insert("&&", Assoc::Right, 3);
    for op in ["==", "/=", "<", "<=", ">", ">="] {
        let _ = f.insert(op, Assoc::None, 4);
    }
    let _ = f.insert("+", Assoc::Left, 6);
    let _ = f.insert("-", Assoc::Left, 6);
    let _ = f.insert("*", Assoc::Left, 7);
    f
}

/// Parse a `.cinch` file and extract verification obligations for every `prim` body.
pub fn extract_file_obligations(
    path: &Path,
    semantics: &[&dyn IrSemantics],
) -> Result<Vec<ExtractedPrimBody>, String> {
    let src = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read `{}`: {e}", path.display()))?;
    extract_source_obligations(&src, semantics)
}

/// Parse Cinch source text and extract verification obligations for every `prim` body.
pub fn extract_source_obligations(
    src: &str,
    semantics: &[&dyn IrSemantics],
) -> Result<Vec<ExtractedPrimBody>, String> {
    let tokens = crate::lexer::lex(src).map_err(|e| format!("lex error: {e}"))?;
    let tokens = crate::layout::layout(tokens).map_err(|e| format!("layout error: {e}"))?;
    let base = default_fixities();
    let parsed = crate::parser::parse_modules(&tokens, &base, &HashMap::new())
        .map_err(|e| format!("parse error: {e}"))?;

    let mut out = Vec::new();
    for m in &parsed.modules {
        let mname = m.name.text.clone();
        collect_decls(&mname, &m.decls, semantics, &mut out);
    }
    Ok(out)
}

fn collect_decls(
    module: &str,
    decls: &[Decl],
    semantics: &[&dyn IrSemantics],
    out: &mut Vec<ExtractedPrimBody>,
) {
    for decl in decls {
        match decl {
            Decl::Prim(p) => extract_prim(module, p, semantics, out),
            Decl::Attributed { inner, .. } | Decl::Conditional { inner, .. } => {
                collect_decls(module, std::slice::from_ref(inner.as_ref()), semantics, out);
            }
            _ => {}
        }
    }
}

fn extract_prim(
    module: &str,
    p: &PrimDecl,
    semantics: &[&dyn IrSemantics],
    out: &mut Vec<ExtractedPrimBody>,
) {
    for (body_idx, body) in p.bodies.iter().enumerate() {
        let family = body.family.text.clone();
        let when_desc = render_when(&body.when);
        let word_bits = body_word_bits(body);

        if body.unverified {
            out.push(ExtractedPrimBody {
                module: module.to_string(),
                prim_name: p.name.text.clone(),
                body_index: body_idx,
                family,
                is_unsafe: true,
                when_desc,
                obligations: vec![],
                skip_reason: Some("marked `unsafe` (trusted boundary)".to_string()),
            });
            continue;
        }

        let Some(sem_plugin) = semantics.iter().find(|s| s.family() == family) else {
            out.push(ExtractedPrimBody {
                module: module.to_string(),
                prim_name: p.name.text.clone(),
                body_index: body_idx,
                family: family.clone(),
                is_unsafe: false,
                when_desc,
                obligations: vec![],
                skip_reason: Some(format!("no IrSemantics plugin registered for `{family}`")),
            });
            continue;
        };

        let (param_specs, ret_info, ret_refinement) = match parse_prim_sig(&p.ty, word_bits) {
            Ok(sig) => sig,
            Err(reason) => {
                out.push(ExtractedPrimBody {
                    module: module.to_string(),
                    prim_name: p.name.text.clone(),
                    body_index: body_idx,
                    family,
                    is_unsafe: false,
                    when_desc,
                    obligations: vec![],
                    skip_reason: Some(reason),
                });
                continue;
            }
        };

        let sem_params: Vec<SemParam> = param_specs
            .iter()
            .enumerate()
            .map(|(i, (name, info))| SemParam {
                index: i,
                name: name.clone(),
                ty: info.llvm_param_ty,
                signed: info.signed,
            })
            .collect();

        let lowered = match sem_plugin.lower(&body.fragment, &sem_params, ret_info.llvm_param_ty) {
            Ok(prog) => prog,
            Err(SemError::Unsupported(reason)) => {
                out.push(ExtractedPrimBody {
                    module: module.to_string(),
                    prim_name: p.name.text.clone(),
                    body_index: body_idx,
                    family,
                    is_unsafe: false,
                    when_desc,
                    obligations: vec![],
                    skip_reason: Some(format!("unsupported in {}: {reason}", sem_plugin.id())),
                });
                continue;
            }
            Err(SemError::Parse(err)) => {
                out.push(ExtractedPrimBody {
                    module: module.to_string(),
                    prim_name: p.name.text.clone(),
                    body_index: body_idx,
                    family,
                    is_unsafe: false,
                    when_desc,
                    obligations: vec![],
                    skip_reason: Some(format!("IR parse error: {err}")),
                });
                continue;
            }
        };

        let full_name = if p.bodies.len() > 1 {
            format!("{module}.{}#body{body_idx}", p.name.text)
        } else {
            format!("{module}.{}", p.name.text)
        };

        let mut obligations = Vec::new();

        // 1. Well-definedness obligation (`!ub && !poison`).
        {
            let mut g = lowered.graph.clone();
            let not_ub = g.not(lowered.ub_cond);
            let any_p = lowered.result.any_poison(&mut g);
            let not_p = g.not(any_p);
            let goal = g.and(not_ub, not_p);
            obligations.push(Obligation::new(
                format!("{full_name}::well-defined"),
                lowered.semantics_id.clone(),
                ObligationKind::WellDefined,
                sem_params.clone(),
                g,
                goal,
            ));
        }

        // 2. Return refinement obligation (when the Cinch signature specifies `refined (...)`).
        if let Some(ref_pred) = ret_refinement {
            if let Some(ob) = build_refinement_obligation(
                &full_name,
                &lowered,
                &param_specs,
                ret_info,
                &ref_pred,
            ) {
                obligations.push(ob);
            }
        }

        // 3. Curated mathematical `spec` equivalence obligations for iconic prelude prims.
        if let Some(ob) = build_builtin_spec_obligation(
            &full_name,
            &p.name.text,
            &lowered,
            &sem_params,
            word_bits,
        ) {
            obligations.push(ob);
        }

        out.push(ExtractedPrimBody {
            module: module.to_string(),
            prim_name: p.name.text.clone(),
            body_index: body_idx,
            family,
            is_unsafe: false,
            when_desc,
            obligations,
            skip_reason: None,
        });
    }
}

fn body_word_bits(body: &PrimBody) -> u16 {
    for clause in &body.when {
        if let Term::App(head, args, _) = clause {
            if let Term::Var(vp) = head.as_ref() {
                if vp.name.text == "word-size" {
                    if let Some(Term::Int(n, _)) = args.first() {
                        if n == "32" {
                            return 32;
                        }
                    }
                }
            }
        }
    }
    64
}

fn render_when(when: &[Term]) -> String {
    if when.is_empty() {
        return "baseline".to_string();
    }
    when.iter()
        .map(|t| match t {
            Term::App(head, args, _) => {
                let h = match head.as_ref() {
                    Term::Var(v) => v.name.text.clone(),
                    Term::Con(c) => c.parts().join("."),
                    _ => "?".to_string(),
                };
                let a: Vec<String> = args
                    .iter()
                    .map(|x| match x {
                        Term::Var(v) => v.name.text.clone(),
                        Term::Con(c) => c.parts().join("."),
                        Term::Int(n, _) => n.clone(),
                        _ => "?".to_string(),
                    })
                    .collect();
                format!("when {h} {}", a.join(" "))
            }
            _ => "when <cond>".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_prim_sig(
    ty: &Type,
    word_bits: u16,
) -> Result<(Vec<(String, ScalarInfo)>, ScalarInfo, Option<Type>), String> {
    let mut params = Vec::new();
    let mut cur = ty;
    let mut idx = 0usize;
    loop {
        match cur {
            Type::Fun { arg, res, .. } => {
                let info = parse_scalar_type(arg, word_bits)?;
                params.push((format!("%{idx}"), info));
                idx += 1;
                cur = res;
            }
            Type::DepFun(name, arg, _, _, res, _) => {
                let info = parse_scalar_type(arg, word_bits)?;
                params.push((name.text.clone(), info));
                idx += 1;
                cur = res;
            }
            _ => break,
        }
    }
    let (base_ty, refinement) = unwrap_refined(cur);
    let ret = parse_scalar_type(base_ty, word_bits)?;
    Ok((params, ret, refinement.cloned()))
}

fn unwrap_refined(ty: &Type) -> (&Type, Option<&Type>) {
    // `refined (pred) Base` parses as `App(App(Var("refined"), pred), Base)`.
    if let Type::App(f, base, _) = ty {
        if let Type::App(head, pred, _) = f.as_ref() {
            if matches!(head.as_ref(), Type::Var(n, _) if n.text == "refined") {
                return (base.as_ref(), Some(pred.as_ref()));
            }
        }
    }
    (ty, None)
}

fn parse_scalar_type(ty: &Type, word_bits: u16) -> Result<ScalarInfo, String> {
    let (base, _) = unwrap_refined(ty);
    match base {
        Type::Con(path, _) => {
            let name = path.segments.last().map(|s| s.text.as_str()).unwrap_or("");
            match name {
                "Bool" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Bv(8),
                    signed: false,
                }),
                "I8" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(8), signed: true }),
                "I16" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(16), signed: true }),
                "I32" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: true }),
                "I64" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(64), signed: true }),
                "I128" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(128), signed: true }),
                "U8" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(8), signed: false }),
                "U16" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(16), signed: false }),
                "U32" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: false }),
                "U64" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(64), signed: false }),
                "U128" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(128), signed: false }),
                "Usize" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Bv(word_bits),
                    signed: false,
                }),
                "Isize" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Bv(word_bits),
                    signed: true,
                }),
                "F32" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Fp(FpFmt::F32),
                    signed: true,
                }),
                "F64" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Fp(FpFmt::F64),
                    signed: true,
                }),
                "Char" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: false }),
                other => Err(format!("non-scalar prim ABI type `{other}`")),
            }
        }
        _ => Err("complex prim ABI type".to_string()),
    }
}

/// Lower a Cinch return refinement predicate (like `r == x` or `InRange (-128) 127 r && r == x`)
/// into a `SemGraph` boolean goal over exact mathematical integer bit-widths.
fn build_refinement_obligation(
    full_name: &str,
    lowered: &SemProgram,
    param_specs: &[(String, ScalarInfo)],
    ret_info: ScalarInfo,
    pred: &Type,
) -> Option<Obligation> {
    let (res_node, res_poison) = lowered.result.as_scalar()?;
    let mut g = lowered.graph.clone();

    // Common exact signed width wide enough for every scalar parameter and return value.
    let mut max_w = match ret_info.llvm_param_ty {
        SemTy::Bv(w) => w,
        _ => return None,
    };
    for (_, info) in param_specs {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            max_w = max_w.max(w);
        }
    }
    let exact_w = (max_w + 2).min(128);

    let mut vars: HashMap<String, NodeId> = HashMap::new();
    for ((name, info), (_, pnode)) in param_specs.iter().zip(lowered.params.iter()) {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            let widened = if w == exact_w {
                *pnode
            } else if info.signed {
                g.bv_sext(*pnode, exact_w)
            } else {
                g.bv_zext(*pnode, exact_w)
            };
            vars.insert(name.clone(), widened);
        }
    }
    let ret_w = g.bv_width(res_node);
    let r_widened = if ret_w == exact_w {
        res_node
    } else if ret_info.signed {
        g.bv_sext(res_node, exact_w)
    } else {
        g.bv_zext(res_node, exact_w)
    };
    vars.insert("r".to_string(), r_widened);

    let pred_bool = lower_refinement_pred(&mut g, pred, &vars, exact_w)?;
    let not_ub = g.not(lowered.ub_cond);
    let not_p = g.not(res_poison);
    let wd = g.and(not_ub, not_p);
    let goal = g.and(wd, pred_bool);

    let desc = render_type_pred(pred);
    let params: Vec<SemParam> = lowered.params.iter().map(|(p, _)| p.clone()).collect();
    Some(Obligation::new(
        format!("{full_name}::refinement"),
        lowered.semantics_id.clone(),
        ObligationKind::Refinement { description: desc },
        params,
        g,
        goal,
    ))
}

fn lower_refinement_pred(
    g: &mut SemGraph,
    pred: &Type,
    vars: &HashMap<String, NodeId>,
    exact_w: u16,
) -> Option<NodeId> {
    // Handle binary app `App(App(op, lhs), rhs)`
    if let Type::App(f, rhs, _) = pred {
        if let Type::App(op, lhs, _) = f.as_ref() {
            if let Type::Var(op_name, _) = op.as_ref() {
                match op_name.text.as_str() {
                    "&&" => {
                        let a = lower_refinement_pred(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_pred(g, rhs, vars, exact_w)?;
                        return Some(g.and(a, b));
                    }
                    "||" => {
                        let a = lower_refinement_pred(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_pred(g, rhs, vars, exact_w)?;
                        return Some(g.or(a, b));
                    }
                    "==" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        return Some(g.bv_eq(a, b));
                    }
                    "/=" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        let eq = g.bv_eq(a, b);
                        return Some(g.not(eq));
                    }
                    "<=" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        return Some(g.bv_sle(a, b));
                    }
                    "<" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        return Some(g.bv_slt(a, b));
                    }
                    ">=" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        return Some(g.bv_sle(b, a));
                    }
                    ">" => {
                        let a = lower_refinement_int(g, lhs, vars, exact_w)?;
                        let b = lower_refinement_int(g, rhs, vars, exact_w)?;
                        return Some(g.bv_slt(b, a));
                    }
                    _ => {}
                }
            }
        }
        // Handle `InRange lo hi x`: `App(App(App(Con("InRange"), lo), hi), x)`
        if let Type::App(f2, hi, _) = f.as_ref() {
            if let Type::App(head, lo, _) = f2.as_ref() {
                if matches!(head.as_ref(), Type::Con(c, _) if c.segments.last().is_some_and(|s| s.text == "InRange")) {
                    let vlo = lower_refinement_int(g, lo, vars, exact_w)?;
                    let vhi = lower_refinement_int(g, hi, vars, exact_w)?;
                    let vx = lower_refinement_int(g, rhs, vars, exact_w)?;
                    let c1 = g.bv_sle(vlo, vx);
                    let c2 = g.bv_sle(vx, vhi);
                    return Some(g.and(c1, c2));
                }
            }
        }
    }
    None
}

fn lower_refinement_int(
    g: &mut SemGraph,
    expr: &Type,
    vars: &HashMap<String, NodeId>,
    exact_w: u16,
) -> Option<NodeId> {
    match expr {
        Type::Var(n, _) => vars.get(&n.text).copied(),
        Type::Num(s, _) => {
            let v: i128 = s.parse().ok()?;
            Some(g.bv_const(exact_w, v as u128))
        }
        Type::App(op, arg, _) => {
            if matches!(op.as_ref(), Type::Var(n, _) if n.text == "-") {
                let inner = lower_refinement_int(g, arg, vars, exact_w)?;
                return Some(g.bv_neg(inner));
            }
            None
        }
        _ => None,
    }
}

fn render_type_pred(ty: &Type) -> String {
    match ty {
        Type::Var(n, _) => n.text.clone(),
        Type::Con(c, _) => c.segments.last().map(|s| s.text.clone()).unwrap_or_default(),
        Type::Num(s, _) => s.clone(),
        Type::App(f, a, _) => {
            if let Type::App(op, l, _) = f.as_ref() {
                if let Type::Var(n, _) = op.as_ref() {
                    if matches!(n.text.as_str(), "&&" | "||" | "==" | "/=" | "<" | "<=" | ">" | ">=") {
                        return format!("({} {} {})", render_type_pred(l), n.text, render_type_pred(a));
                    }
                }
            }
            format!("({} {})", render_type_pred(f), render_type_pred(a))
        }
        _ => "<pred>".to_string(),
    }
}

/// Curated mathematical `spec` equivalence obligations for representative prelude prims,
/// demonstrating end-to-end functional correctness proofs (`llvm body == mathematical spec`).
fn build_builtin_spec_obligation(
    full_name: &str,
    prim_name: &str,
    lowered: &SemProgram,
    params: &[SemParam],
    _word_bits: u16,
) -> Option<Obligation> {
    let (res_node, res_poison) = lowered.result.as_scalar()?;
    let mut g = lowered.graph.clone();

    let (spec_name, eq_node) = match prim_name {
        // Proof that `saturating-mul-i8`'s 9-instruction LLVM overflow+select sequence
        // equals exact 16-bit multiplication clamped to [-128, 127].
        "saturating-mul-i8" => {
            let a = lowered.params[0].1;
            let b = lowered.params[1].1;
            let a16 = g.bv_sext(a, 16);
            let b16 = g.bv_sext(b, 16);
            let prod = g.bv_mul(a16, b16);
            let max16 = g.bv_const(16, 127);
            let min16 = g.bv_const(16, (-128i128) as u128);
            let too_hi = g.bv_slt(max16, prod);
            let too_lo = g.bv_slt(prod, min16);
            let trunc = g.bv_trunc(prod, 8);
            let max8 = g.bv_const(8, 127);
            let min8 = g.bv_const(8, (-128i128) as u128);
            let c1 = g.ite(too_hi, max8, trunc);
            let expected = g.ite(too_lo, min8, c1);
            ("clamp_i8(sext16(a) * sext16(b))", g.bv_eq(res_node, expected))
        }
        // Proof that `wrapping-shl-i8` shifts left by `s mod 8`.
        "wrapping-shl-i8" => {
            let a = lowered.params[0].1;
            let s = lowered.params[1].1;
            let mask = g.bv_const(32, 7);
            let smod = g.bv_and(s, mask);
            let smod8 = g.bv_trunc(smod, 8);
            let expected = g.bv_shl(a, smod8);
            ("shl_i8(a, s mod 8)", g.bv_eq(res_node, expected))
        }
        // Proof that `simd-bitselect(m, a, b)` equals bitwise `ite(m_i, a_i, b_i)` across all 128 bits.
        "simd-bitselect" => {
            let m = lowered.params[0].1;
            let a = lowered.params[1].1;
            let b = lowered.params[2].1;
            // Independent spec: `(a ^ b) & m ^ b` (standard mux identity!)
            let x = g.bv_xor(a, b);
            let y = g.bv_and(x, m);
            let expected = g.bv_xor(y, b);
            ("mux128(m, a, b) == ((a ^ b) & m) ^ b", g.bv_eq(res_node, expected))
        }
        // Proof that `simd-u8x16-splat(b)` replicates byte `b` into all 16 lanes of `U128` with zero poison.
        "simd-u8x16-splat" => {
            let byte = lowered.params[0].1;
            let mut acc = byte;
            for _ in 1..16 {
                acc = g.bv_concat(acc, byte);
            }
            ("repeat_u8_x16(b)", g.bv_eq(res_node, acc))
        }
        // Proof that `simd-u8x16-movemask(v)` extracts the MSB (bit 7) of each of the 16 byte lanes into low 16 bits of U32.
        "simd-u8x16-movemask" => {
            let v = lowered.params[0].1;
            let mut acc = g.bv_extract(v, 7, 7);
            for lane in 1..16u16 {
                let sign_bit = g.bv_extract(v, lane * 8 + 7, lane * 8 + 7);
                acc = g.bv_concat(sign_bit, acc);
            }
            let expected = g.bv_zext(acc, 32);
            ("pack_sign_bits_16x8(v)", g.bv_eq(res_node, expected))
        }
        // Proof that `ov-add-i8` returns 1 iff exact 16-bit sum is outside [-128, 127].
        "ov-add-i8" => {
            let a = lowered.params[0].1;
            let b = lowered.params[1].1;
            let a16 = g.bv_sext(a, 16);
            let b16 = g.bv_sext(b, 16);
            let sum16 = g.bv_add(a16, b16);
            let min16 = g.bv_const(16, (-128i128) as u128);
            let max16 = g.bv_const(16, 127);
            let lo = g.bv_slt(sum16, min16);
            let hi = g.bv_slt(max16, sum16);
            let ov = g.or(lo, hi);
            let one = g.bv_const(8, 1);
            let zero = g.bv_const(8, 0);
            let expected = g.ite(ov, one, zero);
            ("overflow_i8(sext16(a) + sext16(b))", g.bv_eq(res_node, expected))
        }
        _ => return None,
    };

    let not_ub = g.not(lowered.ub_cond);
    let not_p = g.not(res_poison);
    let wd = g.and(not_ub, not_p);
    let goal = g.and(wd, eq_node);

    Some(Obligation::new(
        format!("{full_name}::equiv"),
        lowered.semantics_id.clone(),
        ObligationKind::FunctionalEquiv { spec_name: spec_name.to_string() },
        params.to_vec(),
        g,
        goal,
    ))
}
