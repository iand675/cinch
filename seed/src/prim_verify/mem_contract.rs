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

use crate::ast::{Decl, PrimBody, PrimDecl, Term, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Pinned Linux kernel syscall numbers `(canonical_name, x86_64_nr, aarch64_nr)`.
/// Covers all `Prim.Sys.Linux` syscalls plus all high-throughput I/O syscalls.
pub const LINUX_SYSCALL_TABLE: &[(&str, u64, u64)] = &[
    // Existing Prim.Sys.Linux syscalls
    ("read", 0, 63),
    ("write", 1, 64),
    ("close", 3, 57),
    ("lseek", 8, 62),
    ("getpid", 39, 172),
    ("fsync", 74, 82),
    ("openat", 257, 56),
    ("mkdirat", 258, 34),
    ("newfstatat", 262, 79),
    ("unlinkat", 263, 35),
    ("renameat", 264, 38),
    // High-throughput io_uring syscalls
    ("io_uring_setup", 425, 425),
    ("io_uring_enter", 426, 426),
    ("io_uring_register", 427, 427),
    // High-throughput networking / socket syscalls
    ("socket", 41, 198),
    ("bind", 49, 200),
    ("listen", 50, 201),
    ("accept4", 288, 242),
    ("setsockopt", 54, 208),
    ("getsockopt", 55, 209),
    ("sendto", 44, 206),
    ("recvfrom", 45, 207),
    ("sendmsg", 46, 211),
    ("recvmsg", 47, 212),
    ("sendmmsg", 307, 269),
    ("recvmmsg", 299, 243),
    // High-throughput memory, scheduling, clock, file, zero-copy & epoll syscalls
    ("mmap", 9, 222),
    ("munmap", 11, 215),
    ("madvise", 28, 233),
    ("eventfd2", 290, 19),
    ("sched_setaffinity", 203, 122),
    ("clock_gettime", 228, 113),
    ("fallocate", 285, 47),
    ("fdatasync", 75, 83),
    ("fcntl", 72, 25),
    ("splice", 275, 76),
    ("sendfile", 40, 71),
    ("epoll_create1", 291, 20),
    ("epoll_ctl", 233, 21),
    ("epoll_pwait", 281, 22),
];

/// Normalize a Cinch prim name (e.g. `linux-io-uring-setup`, `sys_socket`, `raw-mmap`)
/// and look up its `(x86_64, aarch64)` Linux syscall numbers.
pub fn lookup_linux_syscall(name: &str) -> Option<(&'static str, u64, u64)> {
    let norm = name.trim().to_ascii_lowercase().replace('-', "_");
    let stripped = norm
        .strip_prefix("prim_linux_")
        .or_else(|| norm.strip_prefix("prim_sys_"))
        .or_else(|| norm.strip_prefix("linux_raw_"))
        .or_else(|| norm.strip_prefix("sys_linux_"))
        .or_else(|| norm.strip_prefix("linux_"))
        .or_else(|| norm.strip_prefix("sys_"))
        .or_else(|| norm.strip_prefix("raw_"))
        .unwrap_or(&norm);
    LINUX_SYSCALL_TABLE
        .iter()
        .copied()
        .find(|&(canonical, _, _)| canonical == stripped || canonical == norm)
}

pub fn lookup_linux_syscall_by_x86_64(nr: u64) -> Option<(&'static str, u64, u64)> {
    LINUX_SYSCALL_TABLE.iter().copied().find(|&(_, x, _)| x == nr)
}

pub fn lookup_linux_syscall_by_aarch64(nr: u64) -> Option<(&'static str, u64, u64)> {
    LINUX_SYSCALL_TABLE.iter().copied().find(|&(_, _, a)| a == nr)
}

/// Verify that an `(x86_64_nr, aarch64_nr)` syscall pair refers to the same Linux syscall.
pub fn validate_linux_syscall_pair(x86_64_nr: u64, aarch64_nr: u64) -> Result<&'static str, String> {
    let by_x = lookup_linux_syscall_by_x86_64(x86_64_nr);
    let by_a = lookup_linux_syscall_by_aarch64(aarch64_nr);
    match (by_x, by_a) {
        (Some((nx, _, expected_a)), Some((na, expected_x, _))) => {
            if nx == na {
                Ok(nx)
            } else {
                Err(format!(
                    "mismatched Linux syscall numbers: x86_64 #{x86_64_nr} is `{nx}` (expected aarch64 #{expected_a}), \
                     but aarch64 #{aarch64_nr} is `{na}` (expected x86_64 #{expected_x})"
                ))
            }
        }
        (Some((nx, _, expected_a)), None) => Err(format!(
            "mismatched Linux syscall numbers: x86_64 #{x86_64_nr} is `{nx}` (expected aarch64 #{expected_a}), \
             got unpinned aarch64 #{aarch64_nr}"
        )),
        (None, Some((na, expected_x, _))) => Err(format!(
            "mismatched Linux syscall numbers: aarch64 #{aarch64_nr} is `{na}` (expected x86_64 #{expected_x}), \
             got unpinned x86_64 #{x86_64_nr}"
        )),
        (None, None) => Err(format!(
            "unpinned Linux syscall number pair (x86_64=#{x86_64_nr}, aarch64=#{aarch64_nr})"
        )),
    }
}

fn parse_asm_imm(tok: &str) -> Option<u64> {
    let s = tok
        .trim()
        .trim_end_matches(',')
        .trim_start_matches('#')
        .trim_start_matches("$$")
        .trim_start_matches('$');
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}

fn split_asm_insns(asm_body: &str) -> Vec<String> {
    asm_body
        .replace("\\0A", "\n")
        .replace("\\0a", "\n")
        .replace("\\n", "\n")
        .split(['\n', ';'])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Extract a pinned x86_64 Linux syscall number loaded into `rax`/`eax` in an inline `asm` body.
pub fn extract_x86_64_syscall_nr(asm_body: &str) -> Option<u64> {
    for insn in split_asm_insns(asm_body) {
        let lower = insn.to_ascii_lowercase();
        let parts: Vec<&str> = lower
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|w| !w.is_empty())
            .collect();
        if parts.len() == 3 {
            match parts[0] {
                "mov" | "movq" | "movl" => {
                    // Intel syntax: `mov rax, 257` / `mov eax, 257`
                    if matches!(parts[1], "rax" | "eax") {
                        if let Some(n) = parse_asm_imm(parts[2]) {
                            return Some(n);
                        }
                    }
                    // AT&T syntax: `mov $257, %rax` / `movq $$257, %rax`
                    if matches!(parts[2], "%rax" | "%eax" | "%%rax" | "%%eax") {
                        if let Some(n) = parse_asm_imm(parts[1]) {
                            return Some(n);
                        }
                    }
                }
                "xor" | "xorl" | "xorq" => {
                    if (parts[1] == "eax" && parts[2] == "eax")
                        || (parts[1] == "rax" && parts[2] == "rax")
                        || (parts[1] == "%eax" && parts[2] == "%eax")
                        || (parts[1] == "%rax" && parts[2] == "%rax")
                    {
                        return Some(0);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// Extract a pinned aarch64 Linux syscall number loaded into `x8`/`w8` in an inline `asm` body.
pub fn extract_aarch64_linux_syscall_nr(asm_body: &str) -> Option<u64> {
    for insn in split_asm_insns(asm_body) {
        let lower = insn.to_ascii_lowercase();
        let parts: Vec<&str> = lower
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|w| !w.is_empty())
            .collect();
        if parts.len() == 3
            && matches!(parts[0], "mov" | "movz")
            && matches!(parts[1], "x8" | "w8")
        {
            if let Some(n) = parse_asm_imm(parts[2]) {
                return Some(n);
            }
        }
    }
    None
}

fn body_when_arch(body: &PrimBody) -> Option<String> {
    for clause in &body.when {
        if let Term::App(head, args, _) = clause {
            if let Term::Var(vp) = head.as_ref() {
                if vp.name.text == "arch" {
                    if let Some(Term::Var(arg)) = args.first() {
                        return Some(arg.name.text.clone());
                    }
                }
            }
        }
    }
    None
}

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
    #[serde(default)]
    pub linux_syscalls_checked: usize,
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
                if is_pure && tokens.iter().any(|&t| t == "volatile") {
                    push_err(
                        "effect-accountability",
                        format!("line {}: `load volatile` is not permitted in a pure prim signature", line_no + 1),
                    );
                }
                if !tokens.iter().any(|&t| t == "ptr") {
                    push_err(
                        "pointer-hygiene",
                        format!("line {}: `load` must operate on an explicit LLVM `ptr` operand", line_no + 1),
                    );
                }
                if stmt.contains("syncscope(\"\")") {
                    push_err(
                        "atomic-syncscope",
                        format!("line {}: `load` has empty `syncscope(\"\")`", line_no + 1),
                    );
                }
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
                if !tokens.iter().any(|&t| t == "ptr") {
                    push_err(
                        "pointer-hygiene",
                        format!("line {}: `store` must operate on an explicit LLVM `ptr` operand", line_no + 1),
                    );
                }
                if stmt.contains("syncscope(\"\")") {
                    push_err(
                        "atomic-syncscope",
                        format!("line {}: `store` has empty `syncscope(\"\")`", line_no + 1),
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
                if !tokens.iter().any(|&t| t == "ptr") {
                    push_err(
                        "pointer-hygiene",
                        format!("line {}: `cmpxchg` must operate on an explicit LLVM `ptr` operand", line_no + 1),
                    );
                }
                if stmt.contains("syncscope(\"\")") {
                    push_err(
                        "atomic-syncscope",
                        format!("line {}: `cmpxchg` has empty `syncscope(\"\")`", line_no + 1),
                    );
                }
                if let Some(al) = parse_align(stmt) {
                    if !al.is_power_of_two() {
                        push_err(
                            "atomic-alignment",
                            format!("line {}: `cmpxchg` align {al} is not a power of two", line_no + 1),
                        );
                    } else if let Some(ptr_idx) = tokens.iter().position(|&t| t == "ptr") {
                        if let Some(val_ty) = tokens.get(ptr_idx + 2) {
                            if let Some(sz) = llvm_scalar_byte_width(val_ty, ptr_bytes) {
                                if al < sz {
                                    push_err(
                                        "atomic-alignment",
                                        format!("line {}: `cmpxchg {val_ty}` requires align >= {sz}, got align {al}", line_no + 1),
                                    );
                                }
                            }
                        }
                    }
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
                if !tokens.iter().any(|&t| t == "ptr") {
                    push_err(
                        "pointer-hygiene",
                        format!("line {}: `atomicrmw` must operate on an explicit LLVM `ptr` operand", line_no + 1),
                    );
                }
                if stmt.contains("syncscope(\"\")") {
                    push_err(
                        "atomic-syncscope",
                        format!("line {}: `atomicrmw` has empty `syncscope(\"\")`", line_no + 1),
                    );
                }
                let rmw_op = tokens
                    .iter()
                    .skip(1)
                    .find(|&&t| t != "volatile")
                    .copied()
                    .unwrap_or("");
                if !matches!(
                    rmw_op,
                    "xchg" | "add" | "sub" | "and" | "nand" | "or" | "xor"
                        | "max" | "min" | "umax" | "umin"
                        | "fadd" | "fsub" | "fmax" | "fmin"
                        | "uinc_wrap" | "udec_wrap"
                ) {
                    push_err(
                        "atomic-op",
                        format!("line {}: invalid `atomicrmw` operation `{rmw_op}`", line_no + 1),
                    );
                }
                if let Some(al) = parse_align(stmt) {
                    if !al.is_power_of_two() {
                        push_err(
                            "atomic-alignment",
                            format!("line {}: `atomicrmw` align {al} is not a power of two", line_no + 1),
                        );
                    } else if let Some(ptr_idx) = tokens.iter().position(|&t| t == "ptr") {
                        if let Some(val_ty) = tokens.get(ptr_idx + 2) {
                            if let Some(sz) = llvm_scalar_byte_width(val_ty, ptr_bytes) {
                                if al < sz {
                                    push_err(
                                        "atomic-alignment",
                                        format!("line {}: `atomicrmw {val_ty}` requires align >= {sz}, got align {al}", line_no + 1),
                                    );
                                }
                            }
                        }
                    }
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
                    let when_arch = body_when_arch(body);
                    let known_syscall = lookup_linux_syscall(prim_name);
                    if asm_body.contains("syscall") {
                        if matches!(when_arch.as_deref(), Some("aarch64" | "arm64")) {
                            push_err(
                                "linux-syscall-arch",
                                format!(
                                    "line {}: x86_64 `syscall` instruction inside `when arch {}` body",
                                    line_no + 1,
                                    when_arch.as_deref().unwrap_or("")
                                ),
                            );
                        }
                        if !constraints.contains("~{rcx}") || !constraints.contains("~{r11}") {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: x86_64 `syscall` must declare `~{{rcx}}` and `~{{r11}}` clobbers",
                                    line_no + 1
                                ),
                            );
                        }
                        if stmt.contains("sideeffect") && !constraints.contains("~{memory}") {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: x86_64 `syscall` with `sideeffect` must declare `~{{memory}}` clobber",
                                    line_no + 1
                                ),
                            );
                        }
                        if let Some(nr) = extract_x86_64_syscall_nr(asm_body) {
                            report.linux_syscalls_checked += 1;
                            if let Some((canonical, expected_x86, _)) = known_syscall {
                                if nr != expected_x86 {
                                    push_err(
                                        "linux-syscall-number",
                                        format!(
                                            "line {}: x86_64 Linux syscall `{canonical}` pinned as #{nr}, expected #{expected_x86}",
                                            line_no + 1
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    if asm_body.contains("svc #0") || asm_body.contains("svc 0") {
                        if matches!(when_arch.as_deref(), Some("x86_64" | "amd64")) {
                            push_err(
                                "linux-syscall-arch",
                                format!(
                                    "line {}: aarch64 `svc #0` instruction inside `when arch {}` body",
                                    line_no + 1,
                                    when_arch.as_deref().unwrap_or("")
                                ),
                            );
                        }
                        if !constraints.contains("~{x8}") && !constraints.contains("{x8}")
                            && !constraints.contains("~{x16}") && !constraints.contains("{x16}")
                        {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: aarch64 `svc #0` must account for syscall register (`~{{x8}}`/`{{x8}}` or `~{{x16}}`/`{{x16}}`)",
                                    line_no + 1
                                ),
                            );
                        }
                        if stmt.contains("sideeffect") && !constraints.contains("~{memory}") {
                            push_err(
                                "inline-asm-clobbers",
                                format!(
                                    "line {}: aarch64 `svc #0` with `sideeffect` must declare `~{{memory}}` clobber",
                                    line_no + 1
                                ),
                            );
                        }
                        if let Some(nr) = extract_aarch64_linux_syscall_nr(asm_body) {
                            report.linux_syscalls_checked += 1;
                            if let Some((canonical, _, expected_arm64)) = known_syscall {
                                if nr != expected_arm64 {
                                    push_err(
                                        "linux-syscall-number",
                                        format!(
                                            "line {}: aarch64 Linux syscall `{canonical}` pinned as #{nr}, expected #{expected_arm64}",
                                            line_no + 1
                                        ),
                                    );
                                }
                            }
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
    let mut x86_nrs: Vec<(usize, u64)> = Vec::new();
    let mut arm64_nrs: Vec<(usize, u64)> = Vec::new();
    for (idx, body) in p.bodies.iter().enumerate() {
        audit_prim_body_for_word_bits(module, &p.name.text, idx, pure, body, default_word_bits, report);
        if body.family.text == "llvm" {
            for raw in body.fragment.lines() {
                let line = strip_comment(raw).trim();
                if line.contains(" asm ") {
                    let quoted = extract_quoted_strings(line);
                    if let Some(asm_body) = quoted.first() {
                        if asm_body.contains("syscall") {
                            if let Some(nr) = extract_x86_64_syscall_nr(asm_body) {
                                x86_nrs.push((idx, nr));
                            }
                        }
                        if asm_body.contains("svc #0") || asm_body.contains("svc 0") {
                            if let Some(nr) = extract_aarch64_linux_syscall_nr(asm_body) {
                                arm64_nrs.push((idx, nr));
                            }
                        }
                    }
                }
            }
        }
    }
    // Cross-architecture Linux syscall number agreement across bodies of the same `prim`.
    for &(xi, xnr) in &x86_nrs {
        for &(ai, anr) in &arm64_nrs {
            if let Err(msg) = validate_linux_syscall_pair(xnr, anr) {
                report.violations.push(BoundaryViolation {
                    module: module.to_string(),
                    prim_name: p.name.text.clone(),
                    body_index: xi.min(ai),
                    rule: "linux-syscall-mismatch".to_string(),
                    message: format!("bodies #{xi} (x86_64) and #{ai} (aarch64): {msg}"),
                });
            }
        }
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

// =========================================================================
// Cache-Line Isolation Audit (`#[repr(C, align(128))]` atomic ring structs)
// =========================================================================

/// Required minimum separation between independently mutated atomic cursor fields
/// (128 bytes covers both x86_64 L2 spatial prefetcher 2x64B pairs and Apple Silicon /
/// ARM64 128-byte cache lines).
pub const MIN_CACHE_LINE_BYTES: u64 = 128;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheLineField {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub align: u64,
    #[serde(default)]
    pub is_atomic_cursor: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheLineStructLayout {
    #[serde(alias = "type")]
    pub name: String,
    pub repr: String,
    pub size: u64,
    pub align: u64,
    pub fields: Vec<CacheLineField>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheLineViolation {
    pub struct_name: String,
    pub rule: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CacheLineAuditReport {
    pub structs_checked: usize,
    pub cursor_pairs_checked: usize,
    pub violations: Vec<CacheLineViolation>,
}

fn is_padding_field(name: &str) -> bool {
    let l = name.to_ascii_lowercase();
    l.starts_with("_pad") || l.starts_with("pad_") || l.starts_with("padding") || l == "pad"
}

fn looks_like_atomic_cursor(name: &str) -> bool {
    if is_padding_field(name) {
        return false;
    }
    let l = name.to_ascii_lowercase();
    [
        "head", "tail", "cursor", "prod", "cons", "producer", "consumer",
        "read_idx", "write_idx", "rd_idx", "wr_idx", "seq", "epoch",
        "atomic", "ring_pos", "pos", "idx",
    ]
    .iter()
    .any(|kw| l.contains(kw))
}

/// Audit a single `#[repr(C, align(128))]` atomic ring struct layout:
/// - Must use `repr(C)` (`repr` starts with `C`/`c`).
/// - Struct alignment must be a power of two and `>= min_spacing` (default 128).
/// - Struct total size must be a multiple of `min_spacing` (prevents array false sharing).
/// - Every distinct pair of atomic cursor fields must be separated by `>= min_spacing`
///   bytes and occupy distinct `min_spacing`-byte cache lines.
pub fn audit_cache_line_layout(
    layout: &CacheLineStructLayout,
    min_spacing: u64,
) -> CacheLineAuditReport {
    let spacing = if min_spacing == 0 { MIN_CACHE_LINE_BYTES } else { min_spacing };
    let mut report = CacheLineAuditReport {
        structs_checked: 1,
        cursor_pairs_checked: 0,
        violations: Vec::new(),
    };
    let mut push = |rule: &str, message: String| {
        report.violations.push(CacheLineViolation {
            struct_name: layout.name.clone(),
            rule: rule.to_string(),
            message,
        });
    };

    let repr_lower = layout.repr.trim().to_ascii_lowercase();
    if !(repr_lower == "c" || repr_lower.starts_with("c,") || repr_lower.starts_with("c(") || repr_lower.starts_with("repr(c")) {
        push(
            "cache-line-repr",
            format!("atomic ring struct `{}` must use `#[repr(C, align({spacing}))]`, got repr `{}`", layout.name, layout.repr),
        );
    }
    if !layout.align.is_power_of_two() || layout.align < spacing {
        push(
            "cache-line-align",
            format!(
                "atomic ring struct `{}` requires power-of-two align >= {spacing}, got align {}",
                layout.name, layout.align
            ),
        );
    }
    if layout.size == 0 || layout.size % spacing != 0 {
        push(
            "cache-line-stride",
            format!(
                "atomic ring struct `{}` total size {} must be a positive multiple of {spacing} to prevent array false sharing",
                layout.name, layout.size
            ),
        );
    }

    // Select atomic cursor fields: explicit markers take priority; otherwise infer by cursor
    // field names (falling back to all non-padding fields).
    let explicit: Vec<&CacheLineField> = layout.fields.iter().filter(|f| f.is_atomic_cursor).collect();
    let mut cursors: Vec<&CacheLineField> = if !explicit.is_empty() {
        explicit
    } else {
        let by_name: Vec<&CacheLineField> = layout
            .fields
            .iter()
            .filter(|f| looks_like_atomic_cursor(&f.name))
            .collect();
        if !by_name.is_empty() {
            by_name
        } else {
            layout
                .fields
                .iter()
                .filter(|f| !is_padding_field(&f.name))
                .collect()
        }
    };
    cursors.sort_by_key(|f| f.offset);

    for f in &cursors {
        if f.align > 0 && f.offset % f.align != 0 {
            push(
                "cache-line-field-align",
                format!(
                    "atomic cursor `{}.{}` at offset {} is misaligned for align {}",
                    layout.name, f.name, f.offset, f.align
                ),
            );
        }
        if f.offset + f.size > layout.size {
            push(
                "cache-line-field-bounds",
                format!(
                    "atomic cursor `{}.{}` ({}..{}) exceeds struct size {}",
                    layout.name,
                    f.name,
                    f.offset,
                    f.offset + f.size,
                    layout.size
                ),
            );
        }
    }

    for i in 0..cursors.len() {
        for j in (i + 1)..cursors.len() {
            report.cursor_pairs_checked += 1;
            let a = cursors[i];
            let b = cursors[j];
            let delta = b.offset.saturating_sub(a.offset);
            if delta < spacing {
                push(
                    "cache-line-spacing",
                    format!(
                        "atomic cursors `{}.{}` (offset {}) and `{}.{}` (offset {}) are only {} bytes apart (minimum {} bytes required)",
                        layout.name, a.name, a.offset, layout.name, b.name, b.offset, delta, spacing
                    ),
                );
            } else {
                let a_end_line = (a.offset + a.size.saturating_sub(1)) / spacing;
                let b_start_line = b.offset / spacing;
                if a_end_line == b_start_line {
                    push(
                        "cache-line-false-sharing",
                        format!(
                            "atomic cursors `{}.{}` (offset {}, size {}) and `{}.{}` (offset {}) share cache line #{}",
                            layout.name, a.name, a.offset, a.size, layout.name, b.name, b.offset, b_start_line
                        ),
                    );
                }
            }
        }
    }

    report
}

pub fn audit_cache_line_layouts(
    layouts: &[CacheLineStructLayout],
    min_spacing: u64,
) -> CacheLineAuditReport {
    let mut agg = CacheLineAuditReport::default();
    for l in layouts {
        let sub = audit_cache_line_layout(l, min_spacing);
        agg.structs_checked += sub.structs_checked;
        agg.cursor_pairs_checked += sub.cursor_pairs_checked;
        agg.violations.extend(sub.violations);
    }
    agg
}

/// Audit a `.cinchif` file (checking all `abi.layouts` entries whose `align >= 128` or whose
/// name contains `Ring`/`Queue`/`Cursor`) or a standalone JSON array/object of `CacheLineStructLayout`.
pub fn audit_cache_line_file(path: &Path, min_spacing: u64) -> Result<CacheLineAuditReport, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read `{}`: {e}", path.display()))?;
    let val: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("invalid JSON in `{}`: {e}", path.display()))?;
    let layouts: Vec<CacheLineStructLayout> = if let Some(arr) = val
        .get("abi")
        .and_then(|a| a.get("layouts"))
        .and_then(|l| l.as_array())
    {
        let mut selected = Vec::new();
        for item in arr {
            let layout: CacheLineStructLayout = serde_json::from_value(item.clone())
                .map_err(|e| format!("malformed abi.layouts entry in `{}`: {e}", path.display()))?;
            let lower = layout.name.to_ascii_lowercase();
            if layout.align >= min_spacing
                || lower.contains("ring")
                || lower.contains("spsc")
                || lower.contains("mpsc")
                || lower.contains("mpmc")
            {
                selected.push(layout);
            }
        }
        selected
    } else if val.is_array() {
        serde_json::from_value(val)
            .map_err(|e| format!("invalid layout array in `{}`: {e}", path.display()))?
    } else {
        let single: CacheLineStructLayout = serde_json::from_value(val)
            .map_err(|e| format!("invalid layout object in `{}`: {e}", path.display()))?;
        vec![single]
    };
    Ok(audit_cache_line_layouts(&layouts, min_spacing))
}
