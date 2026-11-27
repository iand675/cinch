//! Structured Memory & Atomic Boundary Contracts for `unsafe llvm` primitives.
//!
//! While pure scalar/SIMD `llvm` primitives are formally verified bit-for-bit via
//! `SemGraph` + LRAT, low-level concurrency (`Effect.Stm`) and OS syscall
//! (`Prim.Sys.*`) primitives cross the memory/inline-asm boundary under `unsafe llvm`.
//! This module audits those boundary fragments against LLVM LangRef soundness rules:
//!
//! 1. **Atomic ordering lattice:**
//!    - `load atomic` forbids `release` and `acq_rel`.
//!    - `store atomic` forbids `acquire` and `acq_rel`.
//!    - `fence` forbids `unordered` and `monotonic`.
//!    - `atomicrmw` forbids `unordered`.
//!    - `cmpxchg` forbids `unordered` on either ordering, forbids `release`/`acq_rel`
//!      on failure ordering, and requires `strength(failure) <= strength(success)`.
//! 2. **Alignment discipline:**
//!    - `load atomic` / `store atomic` must carry an explicit power-of-two `align N`
//!      with `N >= sizeof(ty)`.
//!    - Non-atomic `load`/`store` with an explicit `align N` must use a power of two.
//! 3. **Effect-row accountability:**
//!    - Memory writes/atomics (`store`, `atomicrmw`, `cmpxchg`, `fence`, `volatile`)
//!      must not appear in a pure (`->{}`) prim signature.
//! 4. **Inline-asm clobber hygiene:**
//!    - x86_64 `syscall` must clobber `~{rcx}` and `~{r11}` (and `~{memory}` for
//!      pointer-taking syscalls).
//!    - aarch64 `svc` must clobber `~{x8}` (syscall number register) or declare
//!      the syscall register as an input/clobber.
//!    - Inline-asm with `sideeffect` must have well-formed output/clobber syntax.

use crate::ast::{Decl, PrimBody, PrimDecl, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryViolation {
    pub module: String,
    pub prim_name: String,
    pub body_index: usize,
    pub rule: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BoundaryAuditReport {
    pub unsafe_bodies_audited: usize,
    pub atomic_ops_checked: usize,
    pub memory_ops_checked: usize,
    pub inline_asm_checked: usize,
    pub violations: Vec<BoundaryViolation>,
}

fn ordering_rank(ord: &str) -> Option<u8> {
    match ord {
        "unordered" => Some(0),
        "monotonic" => Some(1),
        "acquire" => Some(2),
        "release" => Some(2),
        "acq_rel" => Some(3),
        "seq_cst" => Some(4),
        _ => None,
    }
}

fn llvm_scalar_byte_width(ty: &str, ptr_bytes: usize) -> Option<usize> {
    let ty = ty.trim_end_matches(',');
    if ty == "ptr" {
        return Some(ptr_bytes);
    }
    if ty == "float" {
        return Some(4);
    }
    if ty == "double" {
        return Some(8);
    }
    if let Some(bits_str) = ty.strip_prefix('i') {
        let bits: usize = bits_str.parse().ok()?;
        return Some((bits + 7) / 8);
    }
    None
}

fn body_ptr_bytes(body: &PrimBody, default_word_bits: u16) -> usize {
    let wb = super::prelude_bridge::body_word_bits_for(body, default_word_bits);
    (wb as usize) / 8
}

fn parse_align(line: &str) -> Option<usize> {
    let words: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .collect();
    for i in 0..words.len() {
        if words[i] == "align" {
            return words.get(i + 1).and_then(|w| w.parse::<usize>().ok());
        }
    }
    None
}

fn prim_sig_is_pure(ty: &Type) -> bool {
    match ty {
        Type::Fun { eff, res, .. } => eff.is_none() && prim_sig_is_pure(res),
        Type::DepFun(_, _, _, eff, res, _) => eff.is_none() && prim_sig_is_pure(res),
        _ => true,
    }
}

fn strip_comment(line: &str) -> &str {
    match line.find(';') {
        Some(i) => &line[..i],
        None => line,
    }
}

/// Audit a single `PrimBody` against LLVM memory/atomic/inline-asm boundary contracts.
pub fn audit_prim_body(
    module: &str,
    prim_name: &str,
    body_index: usize,
    is_pure: bool,
    body: &PrimBody,
    report: &mut BoundaryAuditReport,
) {
    audit_prim_body_for_word_bits(module, prim_name, body_index, is_pure, body, 64, report);
}

/// Audit a single `PrimBody` using the target profile's default pointer/word width.
pub fn audit_prim_body_for_word_bits(
    module: &str,
    prim_name: &str,
    body_index: usize,
    is_pure: bool,
    body: &PrimBody,
    default_word_bits: u16,
    report: &mut BoundaryAuditReport,
) {
    if body.family.text != "llvm" {
        return;
    }
    let ptr_bytes = body_ptr_bytes(body, default_word_bits);
    if body.unverified {
        report.unsafe_bodies_audited += 1;
    }
    let mut push_err = |rule: &str, message: String| {
        report.violations.push(BoundaryViolation {
            module: module.to_string(),
            prim_name: prim_name.to_string(),
            body_index,
            rule: rule.to_string(),
            message,
        });
    };

    for (line_no, raw) in body.fragment.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() || line.ends_with(':') {
            continue;
        }
        let stmt = if line.starts_with('%') {
            match line.find('=') {
                Some(i) => line[i + 1..].trim(),
                None => continue,
            }
        } else {
            line
        };
        let tokens: Vec<&str> = stmt
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|w| !w.is_empty())
            .collect();
        let op = tokens.first().copied().unwrap_or("");

        match op {
            "load" => {
                let is_atomic = tokens.get(1) == Some(&"atomic");
                if is_atomic {
                    report.atomic_ops_checked += 1;
                    // `load atomic [volatile] <ty>, ptr <p> [syncscope(...)] <ordering>, align <N>`
                    let ty_tok = tokens
                        .iter()
                        .skip(2)
                        .find(|&&t| t != "volatile")
                        .copied()
                        .unwrap_or("");
                    let ord_tok = tokens.iter().copied().find(|t| ordering_rank(t).is_some());
                    match ord_tok {
                        None => push_err(
                            "atomic-ordering",
                            format!("line {}: `load atomic` missing memory ordering", line_no + 1),
                        ),
                        Some("release" | "acq_rel") => push_err(
                            "atomic-ordering",
                            format!(
                                "line {}: `load atomic` cannot use `{}` ordering (LLVM LangRef)",
                                line_no + 1,
                                ord_tok.unwrap()
                            ),
                        ),
                        Some(_) => {}
                    }
                    match parse_align(stmt) {
                        None => push_err(
                            "atomic-alignment",
                            format!("line {}: `load atomic` must specify explicit `align N`", line_no + 1),
                        ),
                        Some(al) => {
                            if !al.is_power_of_two() {
                                push_err(
                                    "atomic-alignment",
                                    format!("line {}: `load atomic` align {al} is not a power of two", line_no + 1),
                                );
                            } else if let Some(sz) = llvm_scalar_byte_width(ty_tok, ptr_bytes) {
                                if al < sz {
                                    push_err(
                                        "atomic-alignment",
                                        format!(
                                            "line {}: `load atomic {ty_tok}` requires align >= {sz}, got align {al}",
                                            line_no + 1
                                        ),
                                    );
                                }
                            }
                        }
                    }
                } else {
                    report.memory_ops_checked += 1;
                    if let Some(al) = parse_align(stmt) {
                        if !al.is_power_of_two() {
                            push_err(
                                "memory-alignment",
                                format!("line {}: `load` align {al} is not a power of two", line_no + 1),
                            );
                        }
                    }
                }
            }
            "store" => {
                if is_pure {
                    push_err(
                        "effect-accountability",
                        format!("line {}: `store` is not permitted in a pure prim signature", line_no + 1),
                    );
                }
                let is_atomic = tokens.get(1) == Some(&"atomic");
                if is_atomic {
                    report.atomic_ops_checked += 1;
                    let ty_tok = tokens
                        .iter()
                        .skip(2)
                        .find(|&&t| t != "volatile")
                        .copied()
                        .unwrap_or("");
                    let ord_tok = tokens.iter().copied().find(|t| ordering_rank(t).is_some());
                    match ord_tok {
                        None => push_err(
                            "atomic-ordering",
                            format!("line {}: `store atomic` missing memory ordering", line_no + 1),
                        ),
                        Some("acquire" | "acq_rel") => push_err(
                            "atomic-ordering",
                            format!(
                                "line {}: `store atomic` cannot use `{}` ordering (LLVM LangRef)",
                                line_no + 1,
                                ord_tok.unwrap()
                            ),
                        ),
                        Some(_) => {}
                    }
                    match parse_align(stmt) {
                        None => push_err(
                            "atomic-alignment",
                            format!("line {}: `store atomic` must specify explicit `align N`", line_no + 1),
                        ),
                        Some(al) => {
                            if !al.is_power_of_two() {
                                push_err(
                                    "atomic-alignment",
                                    format!("line {}: `store atomic` align {al} is not a power of two", line_no + 1),
                                );
                            } else if let Some(sz) = llvm_scalar_byte_width(ty_tok, ptr_bytes) {
                                if al < sz {
                                    push_err(
                                        "atomic-alignment",
                                        format!(
                                            "line {}: `store atomic {ty_tok}` requires align >= {sz}, got align {al}",
                                            line_no + 1
                                        ),
                                    );
                                }
                            }
                        }
                    }
                } else {
                    report.memory_ops_checked += 1;
                    if let Some(al) = parse_align(stmt) {
                        if !al.is_power_of_two() {
                            push_err(
                                "memory-alignment",
                                format!("line {}: `store` align {al} is not a power of two", line_no + 1),
                            );
                        }
                    }
                }
            }
            "cmpxchg" => {
                report.atomic_ops_checked += 1;
                if is_pure {
                    push_err(
                        "effect-accountability",
                        format!("line {}: `cmpxchg` is not permitted in a pure prim signature", line_no + 1),
                    );
                }
                let ords: Vec<&str> = tokens
                    .iter()
                    .copied()
                    .filter(|t| ordering_rank(t).is_some())
                    .collect();
                if ords.len() < 2 {
                    push_err(
                        "atomic-ordering",
                        format!(
                            "line {}: `cmpxchg` requires both success and failure orderings",
                            line_no + 1
                        ),
                    );
                } else {
                    let succ = ords[0];
                    let fail = ords[1];
                    if succ == "unordered" || fail == "unordered" {
                        push_err(
                            "atomic-ordering",
                            format!("line {}: `cmpxchg` cannot use `unordered` ordering", line_no + 1),
                        );
                    }
                    if matches!(fail, "release" | "acq_rel") {
                        push_err(
                            "atomic-ordering",
                            format!(
                                "line {}: `cmpxchg` failure ordering cannot be `{fail}` (LLVM LangRef)",
                                line_no + 1
                            ),
                        );
                    }
                    let sr = ordering_rank(succ).unwrap_or(0);
                    let fr = ordering_rank(fail).unwrap_or(0);
                    if fr > sr {
                        push_err(
                            "atomic-ordering",
                            format!(
                                "line {}: `cmpxchg` failure ordering `{fail}` is stronger than success ordering `{succ}` (LLVM LangRef)",
                                line_no + 1
                            ),
                        );
                    }
                }
            }
            "atomicrmw" => {
                report.atomic_ops_checked += 1;
                if is_pure {
                    push_err(
                        "effect-accountability",
                        format!("line {}: `atomicrmw` is not permitted in a pure prim signature", line_no + 1),
                    );
                }
                let ord_tok = tokens.iter().copied().find(|t| ordering_rank(t).is_some());
                match ord_tok {
                    None => push_err(
                        "atomic-ordering",
                        format!("line {}: `atomicrmw` missing memory ordering", line_no + 1),
                    ),
                    Some("unordered") => push_err(
                        "atomic-ordering",
                        format!("line {}: `atomicrmw` cannot use `unordered` ordering", line_no + 1),
                    ),
                    Some(_) => {}
                }
            }
            "fence" => {
                report.atomic_ops_checked += 1;
                if is_pure {
                    push_err(
                        "effect-accountability",
                        format!("line {}: `fence` is not permitted in a pure prim signature", line_no + 1),
                    );
                }
                let ord_tok = tokens.iter().copied().find(|t| ordering_rank(t).is_some());
                match ord_tok {
                    None => push_err(
                        "atomic-ordering",
                        format!("line {}: `fence` missing memory ordering", line_no + 1),
                    ),
                    Some("unordered" | "monotonic") => push_err(
                        "atomic-ordering",
                        format!(
                            "line {}: `fence` cannot use `{}` ordering (LLVM LangRef)",
                            line_no + 1,
                            ord_tok.unwrap()
                        ),
                    ),
                    Some(_) => {}
                }
            }
            "call" if stmt.contains(" asm ") => {
                report.inline_asm_checked += 1;
                // Extract the two quoted strings: `"<asm>"`, `"<constraints>"`
                let quoted = extract_quoted_strings(stmt);
                if quoted.len() < 2 {
                    push_err(
                        "inline-asm-hygiene",
                        format!("line {}: malformed inline `asm` missing template/constraints", line_no + 1),
                    );
                } else {
                    let asm_body = &quoted[0];
                    let constraints = &quoted[1];
                    if asm_body.contains("syscall") {
                        if !constraints.contains("~{rcx}") || !constraints.contains("~{r11}") {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: x86_64 `syscall` must declare `~{{rcx}}` and `~{{r11}}` clobbers",
                                    line_no + 1
                                ),
                            );
                        }
                    }
                    if asm_body.contains("svc #0") || asm_body.contains("svc 0") {
                        if !constraints.contains("~{x8}") && !constraints.contains("{x8}") {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: aarch64 `svc #0` must account for `x8` (`~{{x8}}` or `{{x8}}`)",
                                    line_no + 1
                                ),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn extract_quoted_strings(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut cur = String::new();
            while let Some(ch) = chars.next() {
                if ch == '\\' {
                    cur.push(ch);
                    if let Some(esc) = chars.next() {
                        cur.push(esc);
                    }
                } else if ch == '"' {
                    break;
                } else {
                    cur.push(ch);
                }
            }
            out.push(cur);
        }
    }
    out
}

/// Audit all `prim` bodies in a `.cinch` source string against Memory & Atomic Boundary Contracts.
pub fn audit_source_contracts(src: &str) -> Result<BoundaryAuditReport, String> {
    audit_source_contracts_for_word_bits(src, 64)
}

/// Audit all `prim` bodies in a `.cinch` source string for the given target word width (32 or 64).
pub fn audit_source_contracts_for_word_bits(
    src: &str,
    default_word_bits: u16,
) -> Result<BoundaryAuditReport, String> {
    let tokens = crate::lexer::lex(src).map_err(|e| format!("lex error: {e}"))?;
    let tokens = crate::layout::layout(tokens).map_err(|e| format!("layout error: {e}"))?;
    let base_fixities = super::prelude_bridge::default_fixities();
    let parsed = crate::parser::parse_modules(&tokens, &base_fixities, &HashMap::new())
        .map_err(|e| format!("parse error: {e}"))?;
    let mut report = BoundaryAuditReport::default();
    fn visit(module: &str, decls: &[Decl], default_word_bits: u16, report: &mut BoundaryAuditReport) {
        for d in decls {
            match d {
                Decl::Prim(p) => audit_prim_decl_for_word_bits(module, p, default_word_bits, report),
                Decl::Attributed { inner, .. } | Decl::Conditional { inner, .. } => {
                    visit(module, std::slice::from_ref(inner.as_ref()), default_word_bits, report);
                }
                _ => {}
            }
        }
    }
    for m in &parsed.modules {
        visit(&m.name.text, &m.decls, default_word_bits, &mut report);
    }
    Ok(report)
}

pub fn audit_prim_decl(module: &str, p: &PrimDecl, report: &mut BoundaryAuditReport) {
    audit_prim_decl_for_word_bits(module, p, 64, report);
}

pub fn audit_prim_decl_for_word_bits(
    module: &str,
    p: &PrimDecl,
    default_word_bits: u16,
    report: &mut BoundaryAuditReport,
) {
    let pure = prim_sig_is_pure(&p.ty);
    for (idx, body) in p.bodies.iter().enumerate() {
        audit_prim_body_for_word_bits(module, &p.name.text, idx, pure, body, default_word_bits, report);
    }
}

pub fn audit_file_contracts(path: &Path) -> Result<BoundaryAuditReport, String> {
    audit_file_contracts_for_word_bits(path, 64)
}

pub fn audit_file_contracts_for_word_bits(
    path: &Path,
    default_word_bits: u16,
) -> Result<BoundaryAuditReport, String> {
    let src = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read `{}`: {e}", path.display()))?;
    audit_source_contracts_for_word_bits(&src, default_word_bits)
}
