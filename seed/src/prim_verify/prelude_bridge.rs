//! Bridge from real Cinch `.cinch` source modules (`Decl::Prim`) into
//! backend-agnostic verification `Obligation`s.
//!
//! Uses Cinch's own lexer, layout pass, and parser so real prelude files
//! (`seed/prelude/Prim/Machine.cinch`, `seed/prelude/Prim/Wide.cinch`, etc.)
//! and compiler-checked `prim ... spec <fn> where ... justify verify` declarations
//! are verified directly against the shared `SemGraph`.

use super::sem::{FpFmt, NodeId, Obligation, ObligationKind, SemGraph, SemParam, SemProgram, SemTy};
use super::{IrSemantics, SemError};
use crate::ast::{Assoc, Decl, Equation, LocalDecl, Pat, PrimBody, PrimDecl, Term, Type};
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
    is_bool: bool,
}

pub(crate) fn default_fixities() -> FixityTable {
    let mut f = FixityTable::new();
    let _ = f.insert("||", Assoc::Right, 2);
    let _ = f.insert("&&", Assoc::Right, 3);
    for op in ["==", "/=", "<", "<=", ">", ">="] {
        let _ = f.insert(op, Assoc::None, 4);
    }
    let _ = f.insert("+", Assoc::Left, 6);
    let _ = f.insert("-", Assoc::Left, 6);
    let _ = f.insert("*", Assoc::Left, 7);
    let _ = f.insert("<>", Assoc::Right, 5);
    let _ = f.insert(":", Assoc::Right, 5);
    let _ = f.insert("$", Assoc::Right, 0);
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

    let mut eqs: HashMap<(String, String), Equation> = HashMap::new();
    for m in &parsed.modules {
        collect_equations(&m.name.text, &m.decls, &mut eqs);
    }

    let mut out = Vec::new();
    for m in &parsed.modules {
        let mname = m.name.text.clone();
        collect_decls(&mname, &m.decls, &eqs, semantics, &mut out);
    }
    Ok(out)
}

fn collect_equations(
    module: &str,
    decls: &[Decl],
    out: &mut HashMap<(String, String), Equation>,
) {
    for decl in decls {
        match decl {
            Decl::Equation { eq, .. } => {
                out.insert((module.to_string(), eq.name.text.clone()), eq.clone());
            }
            Decl::Attributed { inner, .. } | Decl::Conditional { inner, .. } => {
                collect_equations(module, std::slice::from_ref(inner.as_ref()), out);
            }
            _ => {}
        }
    }
}

fn collect_decls(
    module: &str,
    decls: &[Decl],
    eqs: &HashMap<(String, String), Equation>,
    semantics: &[&dyn IrSemantics],
    out: &mut Vec<ExtractedPrimBody>,
) {
    for decl in decls {
        match decl {
            Decl::Prim(p) => extract_prim(module, p, eqs, semantics, out),
            Decl::Attributed { inner, .. } | Decl::Conditional { inner, .. } => {
                collect_decls(module, std::slice::from_ref(inner.as_ref()), eqs, semantics, out);
            }
            _ => {}
        }
    }
}

/// Build all verification obligations for a single `PrimBody` (used by both the
/// standalone `cinch verify-prims` CLI and the compiler's `check_prims_for` pass).
pub fn build_prim_body_obligations(
    module: &str,
    p: &PrimDecl,
    body_idx: usize,
    body: &PrimBody,
    eqs: &HashMap<(String, String), Equation>,
    semantics: &[&dyn IrSemantics],
) -> Result<Vec<Obligation>, String> {
    let family = body.family.text.as_str();
    let word_bits = body_word_bits(body);

    let sem_plugin = semantics
        .iter()
        .find(|s| s.family() == family)
        .ok_or_else(|| format!("no IrSemantics plugin registered for `{family}`"))?;

    let (param_specs, ret_info, ret_refinement) = parse_prim_sig(&p.ty, word_bits)?;

    let sem_params: Vec<SemParam> = param_specs
        .iter()
        .enumerate()
        .map(|(i, (name, info, _))| SemParam {
            index: i,
            name: name.clone(),
            ty: info.llvm_param_ty,
            signed: info.signed,
        })
        .collect();

    let lowered = match sem_plugin.lower(&body.fragment, &sem_params, ret_info.llvm_param_ty) {
        Ok(prog) => prog,
        Err(SemError::Unsupported(reason)) => {
            return Err(format!("unsupported in {}: {reason}", sem_plugin.id()));
        }
        Err(SemError::Parse(err)) => {
            return Err(format!("IR parse error: {err}"));
        }
    };

    let full_name = if p.bodies.len() > 1 {
        format!("{module}.{}#body{body_idx}", p.name.text)
    } else {
        format!("{module}.{}", p.name.text)
    };

    let mut obligations = Vec::new();

    // 1. Well-definedness obligation (`precond => !ub && !poison`).
    {
        let mut g = lowered.graph.clone();
        let precond = build_param_precond(&mut g, &param_specs, &lowered.params);
        let not_ub = g.not(lowered.ub_cond);
        let any_p = lowered.result.any_poison(&mut g);
        let not_p = g.not(any_p);
        let wd = g.and(not_ub, not_p);
        let goal = g.implies(precond, wd);
        obligations.push(Obligation::new(
            format!("{full_name}::well-defined"),
            lowered.semantics_id.clone(),
            ObligationKind::WellDefined,
            sem_params.clone(),
            g,
            goal,
        ));
    }

    // 2. Return refinement obligation (`precond => !ub && !poison && postcond`).
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

    // 3. Source-level `spec <expr>` functional equivalence obligation, if declared.
    if let Some(spec_term) = &p.spec {
        let ob = build_source_spec_obligation(
            module,
            &full_name,
            spec_term,
            &lowered,
            &param_specs,
            ret_info,
            eqs,
        )?;
        obligations.push(ob);
    } else if let Some(ob) = build_builtin_spec_obligation(
        &full_name,
        &p.name.text,
        &lowered,
        &sem_params,
        word_bits,
    ) {
        // Fallback curated functional specs for existing prelude prims that do not
        // yet carry an inline `spec` clause.
        obligations.push(ob);
    }

    Ok(obligations)
}

fn body_is_big_endian(body: &PrimBody) -> bool {
    for clause in &body.when {
        if let Term::App(head, args, _) = clause {
            if let Term::Var(vp) = head.as_ref() {
                if vp.name.text == "endian" {
                    if let Some(Term::Var(arg)) = args.first() {
                        if arg.name.text == "big" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn body_is_arch_or_baseline(body: &PrimBody) -> bool {
    body.when.iter().all(|clause| match clause {
        Term::App(head, _, _) => matches!(
            head.as_ref(),
            Term::Var(vp) if matches!(vp.name.text.as_str(), "arch" | "feature" | "word-size")
        ),
        _ => false,
    })
}

/// Build `ObligationKind::CrossTargetEquiv` obligations across pairs of safe `llvm`
/// bodies on a multi-body `prim` that target the same `word_bits` and endianness
/// (e.g. `when arch x86_64` vs `when arch aarch64` vs portable baseline).
pub fn build_cross_target_obligations(
    module: &str,
    p: &PrimDecl,
    semantics: &[&dyn IrSemantics],
) -> Result<Vec<Obligation>, String> {
    let Some(sem_plugin) = semantics.iter().find(|s| s.family() == "llvm") else {
        return Ok(Vec::new());
    };
    let candidates: Vec<(usize, &PrimBody, u16, bool)> = p
        .bodies
        .iter()
        .enumerate()
        .filter(|(_, b)| {
            !b.unverified && b.family.text == "llvm" && body_is_arch_or_baseline(b)
        })
        .map(|(idx, b)| (idx, b, body_word_bits(b), body_is_big_endian(b)))
        .collect();
    if candidates.len() < 2 {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for i in 0..candidates.len() {
        for j in (i + 1)..candidates.len() {
            let (idx_a, body_a, wb_a, be_a) = candidates[i];
            let (idx_b, body_b, wb_b, be_b) = candidates[j];
            if wb_a != wb_b || be_a != be_b {
                continue;
            }
            let Ok((param_specs, ret_info, _)) = parse_prim_sig(&p.ty, wb_a) else {
                continue;
            };
            let sem_params: Vec<SemParam> = param_specs
                .iter()
                .enumerate()
                .map(|(k, (name, info, _))| SemParam {
                    index: k,
                    name: name.clone(),
                    ty: info.llvm_param_ty,
                    signed: info.signed,
                })
                .collect();
            let prog_a = match sem_plugin.lower(&body_a.fragment, &sem_params, ret_info.llvm_param_ty) {
                Ok(pr) => pr,
                Err(_) => continue,
            };
            let prog_b = match sem_plugin.lower(&body_b.fragment, &sem_params, ret_info.llvm_param_ty) {
                Ok(pr) => pr,
                Err(_) => continue,
            };
            let (Some((va, pa)), Some((vb, pb))) =
                (prog_a.result.as_scalar(), prog_b.result.as_scalar())
            else {
                continue;
            };

            let mut g = prog_a.graph.clone();
            let param_nodes: Vec<NodeId> = prog_a.params.iter().map(|(_, n)| *n).collect();
            let map_b = g.import_graph(&prog_b.graph, &param_nodes);
            let ub_a = prog_a.ub_cond;
            let ub_b = map_b[prog_b.ub_cond.0 as usize];
            let vb_m = map_b[vb.0 as usize];
            let pb_m = map_b[pb.0 as usize];

            let precond = build_param_precond(&mut g, &param_specs, &prog_a.params);
            let same_ub = g.iff(ub_a, ub_b);
            let same_poison = g.iff(pa, pb_m);
            let not_ub = g.not(ub_a);
            let not_p = g.not(pa);
            let defined = g.and(not_ub, not_p);
            let same_val = match g.ty(va) {
                SemTy::Bool => g.iff(va, vb_m),
                SemTy::Bv(_) => g.bv_eq(va, vb_m),
                SemTy::Fp(_) => {
                    let ba = g.fp_to_bv_bits(va);
                    let bb = g.fp_to_bv_bits(vb_m);
                    g.bv_eq(ba, bb)
                }
            };
            let val_when_def = g.implies(defined, same_val);
            let same_err = g.and(same_ub, same_poison);
            let equiv = g.and(same_err, val_when_def);
            let goal = g.implies(precond, equiv);

            let desc_a = format!("body{idx_a}({})", render_when(&body_a.when));
            let desc_b = format!("body{idx_b}({})", render_when(&body_b.when));
            out.push(Obligation::new(
                format!("{module}.{}::cross-target({desc_a}=={desc_b})", p.name.text),
                prog_a.semantics_id.clone(),
                ObligationKind::CrossTargetEquiv {
                    body_a: desc_a,
                    body_b: desc_b,
                },
                sem_params,
                g,
                goal,
            ));
        }
    }
    Ok(out)
}

fn extract_prim(
    module: &str,
    p: &PrimDecl,
    eqs: &HashMap<(String, String), Equation>,
    semantics: &[&dyn IrSemantics],
    out: &mut Vec<ExtractedPrimBody>,
) {
    for (body_idx, body) in p.bodies.iter().enumerate() {
        let family = body.family.text.clone();
        let when_desc = render_when(&body.when);

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

        match build_prim_body_obligations(module, p, body_idx, body, eqs, semantics) {
            Ok(obligations) => out.push(ExtractedPrimBody {
                module: module.to_string(),
                prim_name: p.name.text.clone(),
                body_index: body_idx,
                family,
                is_unsafe: false,
                when_desc,
                obligations,
                skip_reason: None,
            }),
            Err(reason) => out.push(ExtractedPrimBody {
                module: module.to_string(),
                prim_name: p.name.text.clone(),
                body_index: body_idx,
                family,
                is_unsafe: false,
                when_desc,
                obligations: vec![],
                skip_reason: Some(reason),
            }),
        }
    }
    if let Ok(cross_obs) = build_cross_target_obligations(module, p, semantics) {
        if !cross_obs.is_empty() {
            out.push(ExtractedPrimBody {
                module: module.to_string(),
                prim_name: p.name.text.clone(),
                body_index: p.bodies.len(),
                family: "llvm".to_string(),
                is_unsafe: false,
                when_desc: "cross-target".to_string(),
                obligations: cross_obs,
                skip_reason: None,
            });
        }
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

type ParamSpec = (String, ScalarInfo, Option<Type>);

fn parse_prim_sig(
    ty: &Type,
    word_bits: u16,
) -> Result<(Vec<ParamSpec>, ScalarInfo, Option<Type>), String> {
    let mut params = Vec::new();
    let mut cur = ty;
    let mut idx = 0usize;
    loop {
        match cur {
            Type::Fun { arg, res, .. } => {
                let (base, pred) = unwrap_refined(arg);
                let info = parse_scalar_type(base, word_bits)?;
                params.push((format!("%{idx}"), info, pred));
                idx += 1;
                cur = res;
            }
            Type::DepFun(name, arg, _, _, res, _) => {
                let (base, pred) = unwrap_refined(arg);
                let info = parse_scalar_type(base, word_bits)?;
                params.push((name.text.clone(), info, pred));
                idx += 1;
                cur = res;
            }
            _ => break,
        }
    }
    let (base_ty, refinement) = unwrap_refined(cur);
    let ret = parse_scalar_type(base_ty, word_bits)?;
    Ok((params, ret, refinement))
}

fn unwrap_refined(ty: &Type) -> (&Type, Option<Type>) {
    // `refined (pred) Base` parses as `App(App(Var("refined"), pred), Base)`.
    if let Type::App(f, base, _) = ty {
        if let Type::App(head, pred, _) = f.as_ref() {
            if matches!(head.as_ref(), Type::Var(n, _) if n.text == "refined") {
                return (base.as_ref(), Some((**pred).clone()));
            }
        }
        // `InRange lo hi Base` parses as `App(App(App(Con("InRange"), lo), hi), Base)`.
        if let Type::App(f2, hi, sp) = f.as_ref() {
            if let Type::App(head, lo, _) = f2.as_ref() {
                if matches!(head.as_ref(), Type::Con(c, _) if c.segments.last().is_some_and(|s| s.text == "InRange")) {
                    let r_var = Type::Var(crate::ast::Name::new("r", *sp), *sp);
                    let pred = Type::App(
                        Box::new(Type::App(
                            Box::new(Type::App(head.clone(), lo.clone(), *sp)),
                            hi.clone(),
                            *sp,
                        )),
                        Box::new(r_var),
                        *sp,
                    );
                    return (base.as_ref(), Some(pred));
                }
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
                    is_bool: true,
                }),
                "I8" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(8), signed: true, is_bool: false }),
                "I16" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(16), signed: true, is_bool: false }),
                "I32" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: true, is_bool: false }),
                "I64" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(64), signed: true, is_bool: false }),
                "I128" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(128), signed: true, is_bool: false }),
                "U8" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(8), signed: false, is_bool: false }),
                "U16" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(16), signed: false, is_bool: false }),
                "U32" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: false, is_bool: false }),
                "U64" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(64), signed: false, is_bool: false }),
                "U128" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(128), signed: false, is_bool: false }),
                "Usize" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Bv(word_bits),
                    signed: false,
                    is_bool: false,
                }),
                "Isize" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Bv(word_bits),
                    signed: true,
                    is_bool: false,
                }),
                "F32" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Fp(FpFmt::F32),
                    signed: true,
                    is_bool: false,
                }),
                "F64" => Ok(ScalarInfo {
                    llvm_param_ty: SemTy::Fp(FpFmt::F64),
                    signed: true,
                    is_bool: false,
                }),
                "Char" => Ok(ScalarInfo { llvm_param_ty: SemTy::Bv(32), signed: false, is_bool: false }),
                other => Err(format!("non-scalar prim ABI type `{other}`")),
            }
        }
        _ => Err("complex prim ABI type".to_string()),
    }
}

/// Build the conjunction of all parameter refinement preconditions (`refined (P) T`).
fn build_param_precond(
    g: &mut SemGraph,
    param_specs: &[ParamSpec],
    lowered_params: &[(SemParam, NodeId)],
) -> NodeId {
    let mut exact_w: u16 = 66;
    for (_, info, _) in param_specs {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            exact_w = exact_w.max((w + 2).min(128));
        }
    }
    let mut vars: HashMap<String, NodeId> = HashMap::new();
    for ((name, info, _), (_, pnode)) in param_specs.iter().zip(lowered_params.iter()) {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            let widened = widen_to(g, *pnode, w, info.signed, exact_w);
            vars.insert(name.clone(), widened);
        }
    }
    let mut acc = g.bool_const(true);
    for (name, _, pred_opt) in param_specs {
        if let Some(pred) = pred_opt {
            // In Cinch parameter refinements, `r` (or the parameter's own name) refers to the parameter.
            if let Some(&v) = vars.get(name) {
                let mut local_vars = vars.clone();
                local_vars.insert("r".to_string(), v);
                if let Some(p_bool) = lower_refinement_pred(g, pred, &local_vars, exact_w) {
                    acc = g.and(acc, p_bool);
                }
            }
        }
    }
    acc
}

fn widen_to(g: &mut SemGraph, node: NodeId, from_w: u16, signed: bool, to_w: u16) -> NodeId {
    if from_w == to_w {
        node
    } else if signed {
        g.bv_sext(node, to_w)
    } else {
        g.bv_zext(node, to_w)
    }
}

/// Lower a Cinch return refinement predicate (`refined (P) T`) under parameter preconditions.
fn build_refinement_obligation(
    full_name: &str,
    lowered: &SemProgram,
    param_specs: &[ParamSpec],
    ret_info: ScalarInfo,
    pred: &Type,
) -> Option<Obligation> {
    let (res_node, res_poison) = lowered.result.as_scalar()?;
    let mut g = lowered.graph.clone();

    let mut max_w = match ret_info.llvm_param_ty {
        SemTy::Bv(w) => w,
        _ => return None,
    };
    for (_, info, _) in param_specs {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            max_w = max_w.max(w);
        }
    }
    let exact_w = (max_w + 2).min(128);

    let mut vars: HashMap<String, NodeId> = HashMap::new();
    for ((name, info, _), (_, pnode)) in param_specs.iter().zip(lowered.params.iter()) {
        if let SemTy::Bv(w) = info.llvm_param_ty {
            let widened = widen_to(&mut g, *pnode, w, info.signed, exact_w);
            vars.insert(name.clone(), widened);
        }
    }
    let ret_w = g.bv_width(res_node);
    let r_widened = widen_to(&mut g, res_node, ret_w, ret_info.signed, exact_w);
    vars.insert("r".to_string(), r_widened);

    let precond = build_param_precond(&mut g, param_specs, &lowered.params);
    let pred_bool = lower_refinement_pred(&mut g, pred, &vars, exact_w)?;
    let not_ub = g.not(lowered.ub_cond);
    let not_p = g.not(res_poison);
    let wd = g.and(not_ub, not_p);
    let body_ok = g.and(wd, pred_bool);
    let goal = g.implies(precond, body_ok);

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
        // `Not p`
        if matches!(f.as_ref(), Type::Con(c, _) if c.segments.last().is_some_and(|s| s.text == "Not"))
            || matches!(f.as_ref(), Type::Var(n, _) if n.text == "not")
        {
            let inner = lower_refinement_pred(g, rhs, vars, exact_w)?;
            return Some(g.not(inner));
        }
        // `InRange lo hi x`: `App(App(App(Con("InRange"), lo), hi), x)`
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
            let v: u128 = if let Some(rest) = s.strip_prefix('-') {
                rest.parse::<u128>().ok()?.wrapping_neg()
            } else {
                s.parse::<u128>().ok()?
            };
            Some(g.bv_const(exact_w, v))
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

// =========================================================================
// Source-Level `spec <expr>` Lowering into `SemGraph`
// =========================================================================

#[derive(Clone, Copy, Debug)]
struct SpecVal {
    node: NodeId,
    signed: bool,
}

/// Build a `RewriteEquiv` obligation proving `lhs == rhs` for a `rewrite ... justify verify` rule.
pub fn build_rewrite_obligation(
    module: &str,
    rule_name: &str,
    binders: &[(String, SemTy, bool)],
    ret_ty: (SemTy, bool),
    match_tm: &Term,
    replace_tm: &Term,
) -> Result<Obligation, String> {
    let mut g = SemGraph::new();
    let mut env: HashMap<String, SpecVal> = HashMap::new();
    let mut sem_params = Vec::with_capacity(binders.len());
    for (idx, (name, ty, signed)) in binders.iter().enumerate() {
        let node = g.param(idx, name.clone(), *ty);
        sem_params.push(SemParam {
            index: idx,
            name: name.clone(),
            ty: *ty,
            signed: *signed,
        });
        env.insert(name.clone(), SpecVal { node, signed: *signed });
    }
    let lhs = lower_spec_term(&mut g, match_tm, &env, Some(ret_ty))?;
    let rhs = lower_spec_term(&mut g, replace_tm, &env, Some(ret_ty))?;
    let goal = match (g.ty(lhs.node), g.ty(rhs.node)) {
        (SemTy::Bool, SemTy::Bool) => g.iff(lhs.node, rhs.node),
        (SemTy::Bv(w1), SemTy::Bv(w2)) if w1 == w2 => g.bv_eq(lhs.node, rhs.node),
        (t1, t2) => {
            return Err(format!(
                "rewrite `{module}.{rule_name}` LHS ({t1}) and RHS ({t2}) have mismatched semantic types"
            ))
        }
    };
    Ok(Obligation::new(
        format!("{module}.rewrite.{rule_name}::equiv"),
        "cinch-rewrite-sem@1",
        ObligationKind::RewriteEquiv {
            rule_name: format!("{module}.{rule_name}"),
        },
        sem_params,
        g,
        goal,
    ))
}

fn build_source_spec_obligation(
    module: &str,
    full_name: &str,
    spec_term: &Term,
    lowered: &SemProgram,
    param_specs: &[ParamSpec],
    ret_info: ScalarInfo,
    eqs: &HashMap<(String, String), Equation>,
) -> Result<Obligation, String> {
    let (res_node, res_poison) = lowered
        .result
        .as_scalar()
        .ok_or_else(|| "source spec lowering requires a scalar prim return".to_string())?;
    let mut g = lowered.graph.clone();

    let (spec_name, spec_pat_names, spec_body): (String, Vec<String>, &Term) = match spec_term {
        Term::Var(vp) => {
            let target_mod = if vp.quals.is_empty() {
                module.to_string()
            } else {
                vp.quals.iter().map(|q| q.text.as_str()).collect::<Vec<_>>().join(".")
            };
            let key = (target_mod.clone(), vp.name.text.clone());
            let eq = eqs.get(&key).ok_or_else(|| {
                format!(
                    "spec `{}.{}` has no lowerable source equation in scope",
                    target_mod, vp.name.text
                )
            })?;
            let mut pnames = Vec::with_capacity(eq.pats.len());
            for pat in &eq.pats {
                match pat {
                    Pat::Var(n, _) => pnames.push(n.text.clone()),
                    Pat::Wild(_) => pnames.push("_".to_string()),
                    _ => {
                        return Err(format!(
                            "spec `{}` uses a non-variable parameter pattern",
                            vp.name.text
                        ))
                    }
                }
            }
            (format!("{target_mod}.{}", vp.name.text), pnames, &eq.body)
        }
        Term::Lam(pats, body, _) => {
            let mut pnames = Vec::with_capacity(pats.len());
            for pat in pats {
                match pat {
                    Pat::Var(n, _) => pnames.push(n.text.clone()),
                    Pat::Wild(_) => pnames.push("_".to_string()),
                    _ => return Err("inline spec lambda uses non-variable pattern".to_string()),
                }
            }
            ("inline-spec".to_string(), pnames, body.as_ref())
        }
        _ => {
            return Err(
                "prim `spec` must name a spec function or an inline lambda `\\x ... -> ...`"
                    .to_string(),
            )
        }
    };

    if spec_pat_names.len() != lowered.params.len() {
        return Err(format!(
            "spec `{spec_name}` expects {} parameter(s), but prim takes {}",
            spec_pat_names.len(),
            lowered.params.len()
        ));
    }

    let mut env: HashMap<String, SpecVal> = HashMap::new();
    for ((pname, (_, info, _)), (_, pnode)) in
        spec_pat_names.iter().zip(param_specs.iter()).zip(lowered.params.iter())
    {
        let node = if info.is_bool {
            let zero = g.bv_const(8, 0);
            let eq0 = g.bv_eq(*pnode, zero);
            g.not(eq0)
        } else {
            *pnode
        };
        env.insert(pname.clone(), SpecVal { node, signed: info.signed });
    }

    let expected_ty = if ret_info.is_bool {
        SemTy::Bool
    } else {
        ret_info.llvm_param_ty
    };
    let spec_val = lower_spec_term(&mut g, spec_body, &env, Some((expected_ty, ret_info.signed)))?;

    let eq_node = if ret_info.is_bool {
        let one = g.bv_const(8, 1);
        let zero = g.bv_const(8, 0);
        let expected_i8 = match g.ty(spec_val.node) {
            SemTy::Bool => g.ite(spec_val.node, one, zero),
            SemTy::Bv(8) => spec_val.node,
            other => {
                return Err(format!(
                    "spec `{spec_name}` returned {other}, expected Bool"
                ))
            }
        };
        g.bv_eq(res_node, expected_i8)
    } else {
        if g.ty(spec_val.node) != g.ty(res_node) {
            return Err(format!(
                "spec `{spec_name}` type mismatch: spec produced {}, prim returns {}",
                g.ty(spec_val.node),
                g.ty(res_node)
            ));
        }
        g.bv_eq(res_node, spec_val.node)
    };

    let precond = build_param_precond(&mut g, param_specs, &lowered.params);
    let not_ub = g.not(lowered.ub_cond);
    let not_p = g.not(res_poison);
    let wd = g.and(not_ub, not_p);
    let body_ok = g.and(wd, eq_node);
    let goal = g.implies(precond, body_ok);

    let params: Vec<SemParam> = lowered.params.iter().map(|(p, _)| p.clone()).collect();
    Ok(Obligation::new(
        format!("{full_name}::spec({spec_name})"),
        lowered.semantics_id.clone(),
        ObligationKind::FunctionalEquiv { spec_name },
        params,
        g,
        goal,
    ))
}

fn lower_spec_term(
    g: &mut SemGraph,
    term: &Term,
    env: &HashMap<String, SpecVal>,
    hint: Option<(SemTy, bool)>,
) -> Result<SpecVal, String> {
    match term {
        Term::Var(vp) => {
            if let Some(&v) = env.get(&vp.name.text) {
                return Ok(v);
            }
            match vp.name.text.as_str() {
                "true" => Ok(SpecVal { node: g.bool_const(true), signed: false }),
                "false" => Ok(SpecVal { node: g.bool_const(false), signed: false }),
                other => Err(format!("unbound variable `{other}` in prim spec")),
            }
        }
        Term::Con(cp) => match cp.segments.last().map(|s| s.text.as_str()).unwrap_or("") {
            "True" => Ok(SpecVal { node: g.bool_const(true), signed: false }),
            "False" => Ok(SpecVal { node: g.bool_const(false), signed: false }),
            other => Err(format!("unsupported constructor `{other}` in prim spec")),
        },
        Term::Int(s, _) => {
            let v: i128 = s.parse().map_err(|_| format!("invalid integer `{s}` in spec"))?;
            let (w, signed) = match hint {
                Some((SemTy::Bv(w), sgn)) => (w, sgn),
                _ => (64, v < 0),
            };
            Ok(SpecVal { node: g.bv_const(w, v as u128), signed })
        }
        Term::Neg(inner, _) => {
            let v = lower_spec_term(g, inner, env, hint)?;
            Ok(SpecVal { node: g.bv_neg(v.node), signed: v.signed })
        }
        Term::If { cond, then, els, .. } => {
            let vc = lower_spec_term(g, cond, env, Some((SemTy::Bool, false)))?;
            let vt = lower_spec_term(g, then, env, hint)?;
            let ve = lower_spec_term(g, els, env, Some((g.ty(vt.node), vt.signed)))?;
            Ok(SpecVal { node: g.ite(vc.node, vt.node, ve.node), signed: vt.signed })
        }
        Term::Let(locals, body, _) => {
            let mut local_env = env.clone();
            for l in locals {
                match l {
                    LocalDecl::Bind { pat: Pat::Var(name, _), body: rhs, .. }
                    | LocalDecl::Eq { name, body: rhs, .. } => {
                        let v = lower_spec_term(g, rhs, &local_env, None)?;
                        local_env.insert(name.text.clone(), v);
                    }
                    _ => return Err("unsupported `let` binding in prim spec".to_string()),
                }
            }
            lower_spec_term(g, body, &local_env, hint)
        }
        Term::Ann(inner, _, _) => lower_spec_term(g, inner, env, hint),
        Term::App(head, args, _) => {
            // Flatten curried applications `((f a) b)` into `(f, [a, b])`.
            let mut flat_args: Vec<&Term> = Vec::new();
            let mut cur_head = head.as_ref();
            let mut stack: Vec<&[Term]> = vec![args.as_slice()];
            while let Term::App(h2, a2, _) = cur_head {
                stack.push(a2.as_slice());
                cur_head = h2.as_ref();
            }
            for slice in stack.into_iter().rev() {
                flat_args.extend(slice.iter());
            }
            let op_name = match cur_head {
                Term::Var(vp) => vp.name.text.as_str(),
                other => return Err(format!("unsupported application head {other:?} in prim spec")),
            };
            lower_spec_app(g, op_name, &flat_args, env, hint)
        }
        other => Err(format!("unsupported term in prim spec: {other:?}")),
    }
}

fn lower_spec_app(
    g: &mut SemGraph,
    op: &str,
    args: &[&Term],
    env: &HashMap<String, SpecVal>,
    hint: Option<(SemTy, bool)>,
) -> Result<SpecVal, String> {
    // Unary helpers
    if args.len() == 1 {
        if op == "not" {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bool, false)))?;
            return Ok(SpecVal { node: g.not(a.node), signed: false });
        }
        if let Some((w, signed)) = suffix_width(op, "bit-not-") {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
            return Ok(SpecVal { node: g.bv_not(a.node), signed });
        }
        if let Some((w, signed)) = suffix_width(op, "wrapping-neg-") {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
            return Ok(SpecVal { node: g.bv_neg(a.node), signed });
        }
        if let Some((w, signed)) = suffix_width(op, "reverse-bits-") {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
            return Ok(SpecVal { node: g.bv_bitreverse(a.node), signed });
        }
        if let Some((w, signed)) = suffix_width(op, "swap-bytes-") {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
            return Ok(SpecVal { node: g.bv_bswap(a.node), signed });
        }
        if let Some((from_w, from_s, to_w, to_s)) = parse_conv_name(op) {
            let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(from_w), from_s)))?;
            let node = if to_w == from_w {
                a.node
            } else if to_w < from_w {
                g.bv_trunc(a.node, to_w)
            } else if from_s {
                g.bv_sext(a.node, to_w)
            } else {
                g.bv_zext(a.node, to_w)
            };
            return Ok(SpecVal { node, signed: to_s });
        }
    }

    // Binary helpers & operators
    if args.len() == 2 {
        match op {
            "&&" => {
                let a = lower_spec_term(g, args[0], env, Some((SemTy::Bool, false)))?;
                let b = lower_spec_term(g, args[1], env, Some((SemTy::Bool, false)))?;
                return Ok(SpecVal { node: g.and(a.node, b.node), signed: false });
            }
            "||" => {
                let a = lower_spec_term(g, args[0], env, Some((SemTy::Bool, false)))?;
                let b = lower_spec_term(g, args[1], env, Some((SemTy::Bool, false)))?;
                return Ok(SpecVal { node: g.or(a.node, b.node), signed: false });
            }
            "+" | "-" | "*" => {
                let a = lower_spec_term(g, args[0], env, hint)?;
                let b = lower_spec_term(g, args[1], env, Some((g.ty(a.node), a.signed)))?;
                let node = match op {
                    "+" => g.bv_add(a.node, b.node),
                    "-" => g.bv_sub(a.node, b.node),
                    "*" => g.bv_mul(a.node, b.node),
                    _ => unreachable!(),
                };
                return Ok(SpecVal { node, signed: a.signed });
            }
            "==" | "/=" | "<" | "<=" | ">" | ">=" => {
                let a = lower_spec_term(g, args[0], env, None)?;
                let b = lower_spec_term(g, args[1], env, Some((g.ty(a.node), a.signed)))?;
                let node = match op {
                    "==" => g.bv_eq(a.node, b.node),
                    "/=" => {
                        let e = g.bv_eq(a.node, b.node);
                        g.not(e)
                    }
                    "<" => if a.signed { g.bv_slt(a.node, b.node) } else { g.bv_ult(a.node, b.node) },
                    "<=" => if a.signed { g.bv_sle(a.node, b.node) } else { g.bv_ule(a.node, b.node) },
                    ">" => if a.signed { g.bv_slt(b.node, a.node) } else { g.bv_ult(b.node, a.node) },
                    ">=" => if a.signed { g.bv_sle(b.node, a.node) } else { g.bv_ule(b.node, a.node) },
                    _ => unreachable!(),
                };
                return Ok(SpecVal { node, signed: false });
            }
            _ => {}
        }

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
            if let Some((w, signed)) = suffix_width(op, prefix) {
                let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
                let b = lower_spec_term(g, args[1], env, Some((SemTy::Bv(w), signed)))?;
                let (node, out_signed) = match kind {
                    "add" => (g.bv_add(a.node, b.node), signed),
                    "sub" => (g.bv_sub(a.node, b.node), signed),
                    "mul" => (g.bv_mul(a.node, b.node), signed),
                    "and" => (g.bv_and(a.node, b.node), signed),
                    "or" => (g.bv_or(a.node, b.node), signed),
                    "xor" => (g.bv_xor(a.node, b.node), signed),
                    "eq" => (g.bv_eq(a.node, b.node), false),
                    "ne" => {
                        let e = g.bv_eq(a.node, b.node);
                        (g.not(e), false)
                    }
                    "lt" => (if signed { g.bv_slt(a.node, b.node) } else { g.bv_ult(a.node, b.node) }, false),
                    "le" => (if signed { g.bv_sle(a.node, b.node) } else { g.bv_ule(a.node, b.node) }, false),
                    "gt" => (if signed { g.bv_slt(b.node, a.node) } else { g.bv_ult(b.node, a.node) }, false),
                    "ge" => (if signed { g.bv_sle(b.node, a.node) } else { g.bv_ule(b.node, a.node) }, false),
                    _ => unreachable!(),
                };
                return Ok(SpecVal { node, signed: out_signed });
            }
        }

        for (prefix, kind) in [
            ("wrapping-shl-", "shl"),
            ("wrapping-shr-", "shr"),
            ("rotate-left-", "rotl"),
            ("rotate-right-", "rotr"),
        ] {
            if let Some((w, signed)) = suffix_width(op, prefix) {
                let a = lower_spec_term(g, args[0], env, Some((SemTy::Bv(w), signed)))?;
                let b = lower_spec_term(g, args[1], env, Some((SemTy::Bv(32), false)))?;
                let bw = g.bv_width(b.node);
                let mask = g.bv_const(bw, (w - 1) as u128);
                let masked = g.bv_and(b.node, mask);
                let sh = if bw > w {
                    g.bv_trunc(masked, w)
                } else if bw < w {
                    g.bv_zext(masked, w)
                } else {
                    masked
                };
                let node = match kind {
                    "shl" => g.bv_shl(a.node, sh),
                    "shr" => if signed { g.bv_ashr(a.node, sh) } else { g.bv_lshr(a.node, sh) },
                    "rotl" => g.bv_fshl(a.node, a.node, sh),
                    "rotr" => g.bv_fshr(a.node, a.node, sh),
                    _ => unreachable!(),
                };
                return Ok(SpecVal { node, signed });
            }
        }
    }

    Err(format!(
        "unsupported operation `{op}` ({} arg(s)) in prim spec lowering",
        args.len()
    ))
}

fn suffix_width(name: &str, prefix: &str) -> Option<(u16, bool)> {
    let rest = name.strip_prefix(prefix)?;
    parse_width_tag(rest)
}

fn parse_width_tag(tag: &str) -> Option<(u16, bool)> {
    match tag {
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
}

fn parse_conv_name(name: &str) -> Option<(u16, bool, u16, bool)> {
    let trimmed = name.strip_prefix("wrapping-").unwrap_or(name);
    let (lhs, rhs) = trimmed.split_once("-to-")?;
    let (fw, fs) = parse_width_tag(lhs)?;
    let (tw, ts) = parse_width_tag(rhs)?;
    Some((fw, fs, tw, ts))
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
        "wrapping-shl-i8" => {
            let a = lowered.params[0].1;
            let s = lowered.params[1].1;
            let mask = g.bv_const(32, 7);
            let smod = g.bv_and(s, mask);
            let smod8 = g.bv_trunc(smod, 8);
            let expected = g.bv_shl(a, smod8);
            ("shl_i8(a, s mod 8)", g.bv_eq(res_node, expected))
        }
        "simd-bitselect" => {
            let m = lowered.params[0].1;
            let a = lowered.params[1].1;
            let b = lowered.params[2].1;
            let x = g.bv_xor(a, b);
            let y = g.bv_and(x, m);
            let expected = g.bv_xor(y, b);
            ("mux128(m, a, b) == ((a ^ b) & m) ^ b", g.bv_eq(res_node, expected))
        }
        "simd-u8x16-splat" => {
            let byte = lowered.params[0].1;
            let mut acc = byte;
            for _ in 1..16 {
                acc = g.bv_concat(acc, byte);
            }
            ("repeat_u8_x16(b)", g.bv_eq(res_node, acc))
        }
        "simd-u16x8-splat" => {
            let lane = lowered.params[0].1;
            let mut acc = lane;
            for _ in 1..8 {
                acc = g.bv_concat(acc, lane);
            }
            ("repeat_u16_x8(v)", g.bv_eq(res_node, acc))
        }
        "simd-u32x4-splat" => {
            let lane = lowered.params[0].1;
            let mut acc = lane;
            for _ in 1..4 {
                acc = g.bv_concat(acc, lane);
            }
            ("repeat_u32_x4(v)", g.bv_eq(res_node, acc))
        }
        "simd-u64x2-splat" => {
            let lane = lowered.params[0].1;
            let acc = g.bv_concat(lane, lane);
            ("repeat_u64_x2(v)", g.bv_eq(res_node, acc))
        }
        "simd-u8x16-add" | "simd-u8x16-sub" | "simd-u8x16-cmpeq" | "simd-u8x16-cmplt"
        | "simd-u8x16-cmpgt" | "simd-u16x8-add" | "simd-u16x8-sub" | "simd-u16x8-cmpeq"
        | "simd-u16x8-cmplt" | "simd-i16x8-cmplt" | "simd-i16x8-smin" | "simd-i16x8-smax"
        | "simd-u32x4-add" | "simd-u32x4-sub" | "simd-u32x4-cmpeq" | "simd-u32x4-cmplt"
        | "simd-i32x4-smin" | "simd-i32x4-smax" | "simd-u64x2-add" | "simd-u64x2-sub" => {
            let (lanes, ew): (u16, u16) = if prim_name.contains("x16-") {
                (16, 8)
            } else if prim_name.contains("x8-") {
                (8, 16)
            } else if prim_name.contains("x4-") {
                (4, 32)
            } else {
                (2, 64)
            };
            let va = lowered.params[0].1;
            let vb = lowered.params[1].1;
            let all_ones = g.bv_const(ew, SemGraph::mask(ew));
            let zeros = g.bv_const(ew, 0);
            let mut lane_vals = Vec::with_capacity(lanes as usize);
            for lane in 0..lanes {
                let lo = lane * ew;
                let hi = lo + ew - 1;
                let la = g.bv_extract(va, hi, lo);
                let lb = g.bv_extract(vb, hi, lo);
                let out = if prim_name.ends_with("-add") {
                    g.bv_add(la, lb)
                } else if prim_name.ends_with("-sub") {
                    g.bv_sub(la, lb)
                } else if prim_name.ends_with("-cmpeq") {
                    let c = g.bv_eq(la, lb);
                    g.ite(c, all_ones, zeros)
                } else if prim_name.ends_with("-cmplt") {
                    let c = if prim_name.contains("-i") {
                        g.bv_slt(la, lb)
                    } else {
                        g.bv_ult(la, lb)
                    };
                    g.ite(c, all_ones, zeros)
                } else if prim_name.ends_with("-cmpgt") {
                    let c = g.bv_ult(lb, la);
                    g.ite(c, all_ones, zeros)
                } else if prim_name.ends_with("-smin") {
                    let c = g.bv_slt(la, lb);
                    g.ite(c, la, lb)
                } else {
                    let c = g.bv_slt(lb, la);
                    g.ite(c, la, lb)
                };
                lane_vals.push(out);
            }
            let mut acc = *lane_vals.last().unwrap();
            for &lv in lane_vals.iter().rev().skip(1) {
                acc = g.bv_concat(acc, lv);
            }
            (prim_name, g.bv_eq(res_node, acc))
        }
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
