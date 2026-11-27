//! Pluggable SMT-LIB 2 emitter, external SMT solver prover (`z3`, `bitwuzla`,
//! `cvc5`, or any custom SMT-LIB 2 command), and `SmtAttestationChecker`.
//!
//! Translates the *same* `SemGraph` used by the bit-blaster, covering pure
//! bit-vectors (`QF_BV`) as well as IEEE-754 floating-point (`QF_FP` / `QF_FPBV`)
//! and wide nonlinear operations (`mul`, `sdiv`, `udiv`).

use super::sem::{
    eval_node, ConcreteVal, FpFmt, FpPred, FpUnaryOp, NodeId, Obligation, SemGraph, SemOp, SemTy,
    TheorySet,
};
use super::{
    CheckError, Checker, Counterexample, Evidence, Prover, ProverOutcome, TrustReceipt, TrustTier,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

pub const SMT_ATTESTATION_FORMAT_ID: &str = "smt-attestation/v1";
pub const SMT_EMITTER_VERSION_ID: &str = "cinch-smtlib2-emitter@1";
pub const ALETHE_FORMAT_ID: &str = "alethe/v1";
pub const ALETHE_CHECKER_ID: &str = "cinch-alethe-checker@1";

/// Emit a complete, standalone SMT-LIB 2 script checking `!obligation.goal`.
/// `unsat` means `obligation.goal` is valid; `sat` yields a counterexample.
pub fn emit_smtlib2(obligation: &Obligation) -> String {
    let mut out = String::new();
    out.push_str(&format!("; Cinch primitive verification obligation: {}\n", obligation.name));
    out.push_str(&format!("; Semantics: {}\n", obligation.semantics_id));
    out.push_str(&format!("; Kind: {}\n", obligation.kind));
    out.push_str(&format!("; Goal hash: {}\n", obligation.goal_hash));

    let theories = obligation.theories();
    let uses_aes = obligation
        .graph
        .nodes()
        .iter()
        .any(|(op, _)| matches!(op, SemOp::AesCore(..)));
    let logic = match (theories.fp, uses_aes) {
        (true, true) => "QF_AUFFPBV",
        (true, false) => "QF_FPBV",
        (false, true) => "QF_UFBV",
        (false, false) => "QF_BV",
    };
    out.push_str(&format!("(set-logic {logic})\n"));
    out.push_str("(set-option :produce-models true)\n");
    if uses_aes {
        out.push_str("(declare-fun aes_enc_core ((_ BitVec 128)) (_ BitVec 128))\n");
        out.push_str("(declare-fun aes_enc_last_core ((_ BitVec 128)) (_ BitVec 128))\n");
        out.push_str("(declare-fun aes_dec_core ((_ BitVec 128)) (_ BitVec 128))\n");
        out.push_str("(declare-fun aes_dec_last_core ((_ BitVec 128)) (_ BitVec 128))\n");
    }

    for p in &obligation.params {
        out.push_str(&format!("(declare-const p{} {})\n", p.index, smt_sort(p.ty)));
    }

    // Emit reachable DAG nodes in topological order using `(define-const n<id> ...)`
    // so shared subexpressions never blow up exponentially!
    let mut visited = HashMap::new();
    let mut order = Vec::new();
    fn dfs(
        id: NodeId,
        g: &SemGraph,
        visited: &mut HashMap<NodeId, ()>,
        order: &mut Vec<NodeId>,
    ) {
        if visited.contains_key(&id) {
            return;
        }
        visited.insert(id, ());
        for c in g.op(id).children() {
            dfs(c, g, visited, order);
        }
        order.push(id);
    }
    dfs(obligation.goal, &obligation.graph, &mut visited, &mut order);

    for &id in &order {
        let ty = obligation.graph.ty(id);
        let expr = emit_op(&obligation.graph, id);
        out.push_str(&format!("(define-const n{} {} {})\n", id.0, smt_sort(ty), expr));
    }

    out.push_str(&format!("(assert (not n{}))\n", obligation.goal.0));
    out.push_str("(check-sat)\n");
    if !obligation.params.is_empty() {
        let pnames: Vec<String> = obligation.params.iter().map(|p| format!("p{}", p.index)).collect();
        out.push_str(&format!("(get-value ({}))\n", pnames.join(" ")));
    }
    out
}

fn smt_sort(ty: SemTy) -> String {
    match ty {
        SemTy::Bool => "Bool".to_string(),
        SemTy::Bv(w) => format!("(_ BitVec {w})"),
        SemTy::Fp(f) => format!("(_ FloatingPoint {} {})", f.eb(), f.sb()),
    }
}

fn bv_lit(w: u16, bits: u128) -> String {
    let m = bits & SemGraph::mask(w);
    if w % 4 == 0 {
        let hex_digits = (w / 4) as usize;
        format!("#x{m:0hex_digits$x}")
    } else {
        let mut s = String::from("#b");
        for i in (0..w).rev() {
            s.push(if ((m >> i) & 1) != 0 { '1' } else { '0' });
        }
        s
    }
}

fn fp_lit(fmt: FpFmt, bits: u64) -> String {
    match fmt {
        FpFmt::F32 => {
            let b = bits as u32;
            let sign = (b >> 31) & 1;
            let exp = (b >> 23) & 0xff;
            let sig = b & 0x7fffff;
            format!("(fp #b{sign} {} {})", bv_lit(8, exp as u128), bv_lit(23, sig as u128))
        }
        FpFmt::F64 => {
            let sign = (bits >> 63) & 1;
            let exp = (bits >> 52) & 0x7ff;
            let sig = bits & 0x000f_ffff_ffff_ffff;
            format!("(fp #b{sign} {} {})", bv_lit(11, exp as u128), bv_lit(52, sig as u128))
        }
    }
}

fn emit_op(g: &SemGraph, id: NodeId) -> String {
    let n = |c: NodeId| format!("n{}", c.0);
    match g.op(id) {
        SemOp::BoolConst(b) => if *b { "true" } else { "false" }.to_string(),
        SemOp::BvConst { width, bits } => bv_lit(*width, *bits),
        SemOp::FpConst { fmt, bits } => fp_lit(*fmt, *bits),
        SemOp::Param { index, .. } => format!("p{index}"),

        SemOp::Not(a) => format!("(not {})", n(*a)),
        SemOp::And(a, b) => format!("(and {} {})", n(*a), n(*b)),
        SemOp::Or(a, b) => format!("(or {} {})", n(*a), n(*b)),
        SemOp::Xor(a, b) => format!("(xor {} {})", n(*a), n(*b)),
        SemOp::Iff(a, b) => format!("(= {} {})", n(*a), n(*b)),
        SemOp::Implies(a, b) => format!("(=> {} {})", n(*a), n(*b)),
        SemOp::Ite(c, t, e) => format!("(ite {} {} {})", n(*c), n(*t), n(*e)),

        SemOp::BvNot(a) => format!("(bvnot {})", n(*a)),
        SemOp::BvNeg(a) => format!("(bvneg {})", n(*a)),
        SemOp::BvExtract { x, hi, lo } => format!("((_ extract {hi} {lo}) {})", n(*x)),
        SemOp::BvConcat(a, b) => format!("(concat {} {})", n(*a), n(*b)),
        SemOp::BvZExt { x, to } => {
            let pad = to - g.bv_width(*x);
            format!("((_ zero_extend {pad}) {})", n(*x))
        }
        SemOp::BvSExt { x, to } => {
            let pad = to - g.bv_width(*x);
            format!("((_ sign_extend {pad}) {})", n(*x))
        }
        SemOp::BvBitReverse(a) => {
            let w = g.bv_width(*a);
            let mut acc = format!("((_ extract 0 0) {})", n(*a));
            for i in 1..w {
                acc = format!("(concat {acc} ((_ extract {i} {i}) {}))", n(*a));
            }
            acc
        }
        SemOp::BvBSwap(a) => {
            let w = g.bv_width(*a);
            let bytes = w / 8;
            let mut acc = format!("((_ extract 7 0) {})", n(*a));
            for i in 1..bytes {
                let lo = i * 8;
                let hi = lo + 7;
                acc = format!("(concat {acc} ((_ extract {hi} {lo}) {}))", n(*a));
            }
            acc
        }
        SemOp::BvCtPop(a) => {
            let w = g.bv_width(*a);
            let mut acc = bv_lit(w, 0);
            for i in 0..w {
                let bit = format!("((_ zero_extend {}) ((_ extract {i} {i}) {}))", w - 1, n(*a));
                acc = format!("(bvadd {acc} {bit})");
            }
            acc
        }
        SemOp::BvCtlz(a) => {
            let w = g.bv_width(*a);
            let mut acc = bv_lit(w, w as u128);
            for i in 0..w {
                let bit = format!("(= ((_ extract {i} {i}) {}) #b1)", n(*a));
                acc = format!("(ite {bit} {} {acc})", bv_lit(w, (w - 1 - i) as u128));
            }
            acc
        }
        SemOp::BvCttz(a) => {
            let w = g.bv_width(*a);
            let mut acc = bv_lit(w, w as u128);
            for i in (0..w).rev() {
                let bit = format!("(= ((_ extract {i} {i}) {}) #b1)", n(*a));
                acc = format!("(ite {bit} {} {acc})", bv_lit(w, i as u128));
            }
            acc
        }
        SemOp::BvAdd(a, b) => format!("(bvadd {} {})", n(*a), n(*b)),
        SemOp::BvSub(a, b) => format!("(bvsub {} {})", n(*a), n(*b)),
        SemOp::BvMul(a, b) => format!("(bvmul {} {})", n(*a), n(*b)),
        SemOp::BvUDiv(a, b) => format!("(bvudiv {} {})", n(*a), n(*b)),
        SemOp::BvURem(a, b) => format!("(bvurem {} {})", n(*a), n(*b)),
        SemOp::BvSDiv(a, b) => format!("(bvsdiv {} {})", n(*a), n(*b)),
        SemOp::BvSRem(a, b) => format!("(bvsrem {} {})", n(*a), n(*b)),
        SemOp::BvAnd(a, b) => format!("(bvand {} {})", n(*a), n(*b)),
        SemOp::BvOr(a, b) => format!("(bvor {} {})", n(*a), n(*b)),
        SemOp::BvXor(a, b) => format!("(bvxor {} {})", n(*a), n(*b)),
        SemOp::BvShl(a, b) => format!("(bvshl {} {})", n(*a), n(*b)),
        SemOp::BvLShr(a, b) => format!("(bvlshr {} {})", n(*a), n(*b)),
        SemOp::BvAShr(a, b) => format!("(bvashr {} {})", n(*a), n(*b)),
        SemOp::BvFshl(a, b, s) => {
            let w = g.bv_width(*a);
            let w2 = w * 2;
            format!(
                "((_ extract {} {w}) (bvshl (concat {} {}) ((_ zero_extend {w}) (bvurem {} {}))))",
                w2 - 1,
                n(*a),
                n(*b),
                n(*s),
                bv_lit(w, w as u128)
            )
        }
        SemOp::BvFshr(a, b, s) => {
            let w = g.bv_width(*a);
            format!(
                "((_ extract {} 0) (bvlshr (concat {} {}) ((_ zero_extend {w}) (bvurem {} {}))))",
                w - 1,
                n(*a),
                n(*b),
                n(*s),
                bv_lit(w, w as u128)
            )
        }
        SemOp::BvClmul(a, b) => {
            let ea = format!("((_ zero_extend 64) {})", n(*a));
            let mut acc = bv_lit(128, 0);
            for i in 0..64u16 {
                let bit = format!("(= ((_ extract {i} {i}) {}) #b1)", n(*b));
                let shifted = if i == 0 {
                    ea.clone()
                } else {
                    format!("(bvshl {ea} {})", bv_lit(128, i as u128))
                };
                let term = format!("(ite {bit} {shifted} {})", bv_lit(128, 0));
                acc = if i == 0 { term } else { format!("(bvxor {acc} {term})") };
            }
            acc
        }
        SemOp::AesCore(kind, a) => {
            let fname = match kind {
                super::sem::AesRoundKind::EncCore => "aes_enc_core",
                super::sem::AesRoundKind::EncLastCore => "aes_enc_last_core",
                super::sem::AesRoundKind::DecCore => "aes_dec_core",
                super::sem::AesRoundKind::DecLastCore => "aes_dec_last_core",
            };
            format!("({fname} {})", n(*a))
        }
        SemOp::BvEq(a, b) => format!("(= {} {})", n(*a), n(*b)),
        SemOp::BvULt(a, b) => format!("(bvult {} {})", n(*a), n(*b)),
        SemOp::BvULe(a, b) => format!("(bvule {} {})", n(*a), n(*b)),
        SemOp::BvSLt(a, b) => format!("(bvslt {} {})", n(*a), n(*b)),
        SemOp::BvSLe(a, b) => format!("(bvsle {} {})", n(*a), n(*b)),

        SemOp::FpNeg(a) => format!("(fp.neg {})", n(*a)),
        SemOp::FpAdd(a, b) => format!("(fp.add RNE {} {})", n(*a), n(*b)),
        SemOp::FpSub(a, b) => format!("(fp.sub RNE {} {})", n(*a), n(*b)),
        SemOp::FpMul(a, b) => format!("(fp.mul RNE {} {})", n(*a), n(*b)),
        SemOp::FpDiv(a, b) => format!("(fp.div RNE {} {})", n(*a), n(*b)),
        SemOp::FpCmp(pred, a, b) => {
            let (sa, sb) = (n(*a), n(*b));
            let ord = format!("(and (not (fp.isNaN {sa})) (not (fp.isNaN {sb})))");
            let uno = format!("(or (fp.isNaN {sa}) (fp.isNaN {sb}))");
            match pred {
                FpPred::False => "false".to_string(),
                FpPred::True => "true".to_string(),
                FpPred::Ord => ord,
                FpPred::Uno => uno,
                FpPred::Oeq => format!("(fp.eq {sa} {sb})"),
                FpPred::Ogt => format!("(fp.gt {sa} {sb})"),
                FpPred::Oge => format!("(fp.geq {sa} {sb})"),
                FpPred::Olt => format!("(fp.lt {sa} {sb})"),
                FpPred::Ole => format!("(fp.leq {sa} {sb})"),
                FpPred::One => format!("(and {ord} (not (fp.eq {sa} {sb})))"),
                FpPred::Ueq => format!("(or {uno} (fp.eq {sa} {sb}))"),
                FpPred::Ugt => format!("(or {uno} (fp.gt {sa} {sb}))"),
                FpPred::Uge => format!("(or {uno} (fp.geq {sa} {sb}))"),
                FpPred::Ult => format!("(or {uno} (fp.lt {sa} {sb}))"),
                FpPred::Ule => format!("(or {uno} (fp.leq {sa} {sb}))"),
                FpPred::Une => format!("(or {uno} (not (fp.eq {sa} {sb})))"),
            }
        }
        SemOp::FpToBvBits(a) => {
            // SMT-LIB doesn't specify a single canonical NaN payload for fp.to_ubv;
            // represent IEEE bit-reinterpretation via fp.to_ieee_bv (supported by Z3/Bitwuzla/cvc5).
            format!("(fp.to_ieee_bv {})", n(*a))
        }
        SemOp::BvBitsToFp(fmt, a) => {
            format!("((_ to_fp {} {}) {})", fmt.eb(), fmt.sb(), n(*a))
        }
        SemOp::BvToFpSigned(fmt, a) => {
            format!("((_ to_fp {} {}) RNE {})", fmt.eb(), fmt.sb(), n(*a))
        }
        SemOp::BvToFpUnsigned(fmt, a) => {
            format!("((_ to_fp_unsigned {} {}) RNE {})", fmt.eb(), fmt.sb(), n(*a))
        }
        SemOp::FpToBvSigned(w, a) => format!("((_ fp.to_sbv {w}) RTZ {})", n(*a)),
        SemOp::FpToBvUnsigned(w, a) => format!("((_ fp.to_ubv {w}) RTZ {})", n(*a)),
        SemOp::FpToFp(fmt, a) => {
            format!("((_ to_fp {} {}) RNE {})", fmt.eb(), fmt.sb(), n(*a))
        }
        SemOp::FpUnary(op, a) => match op {
            FpUnaryOp::Abs => format!("(fp.abs {})", n(*a)),
            FpUnaryOp::Sqrt => format!("(fp.sqrt RNE {})", n(*a)),
            FpUnaryOp::Floor => format!("(fp.roundToIntegral RTN {})", n(*a)),
            FpUnaryOp::Ceil => format!("(fp.roundToIntegral RTP {})", n(*a)),
            FpUnaryOp::Trunc => format!("(fp.roundToIntegral RTZ {})", n(*a)),
            FpUnaryOp::Round => format!("(fp.roundToIntegral RNA {})", n(*a)),
            FpUnaryOp::RoundEven => format!("(fp.roundToIntegral RNE {})", n(*a)),
        },
        SemOp::FpCopySign(a, b) => {
            let sa = n(*a);
            let sb = n(*b);
            format!("(ite (fp.isNegative {sb}) (fp.neg (fp.abs {sa})) (fp.abs {sa}))")
        }
        SemOp::FpFitsSigned(w, a) => {
            let fmt = g.fp_fmt(*a);
            let sa = n(*a);
            let trunc = format!("(fp.roundToIntegral RTZ {sa})");
            let lo = -((1u128 << (w - 1)) as f64);
            let hi = ((1u128 << (w - 1)) - 1) as f64;
            let (lo_lit, hi_lit) = match fmt {
                FpFmt::F32 => (fp_lit(fmt, (lo as f32).to_bits() as u64), fp_lit(fmt, (hi as f32).to_bits() as u64)),
                FpFmt::F64 => (fp_lit(fmt, lo.to_bits()), fp_lit(fmt, hi.to_bits())),
            };
            format!(
                "(and (not (fp.isNaN {sa})) (not (fp.isInfinite {sa})) (fp.geq {trunc} {lo_lit}) (fp.leq {trunc} {hi_lit}))"
            )
        }
        SemOp::FpFitsUnsigned(w, a) => {
            let fmt = g.fp_fmt(*a);
            let sa = n(*a);
            let trunc = format!("(fp.roundToIntegral RTZ {sa})");
            let hi = if *w >= 64 { u64::MAX as f64 } else { ((1u64 << w) - 1) as f64 };
            let (lo_lit, hi_lit) = match fmt {
                FpFmt::F32 => (fp_lit(fmt, 0.0f32.to_bits() as u64), fp_lit(fmt, (hi as f32).to_bits() as u64)),
                FpFmt::F64 => (fp_lit(fmt, 0.0f64.to_bits()), fp_lit(fmt, hi.to_bits())),
            };
            format!(
                "(and (not (fp.isNaN {sa})) (not (fp.isInfinite {sa})) (fp.geq {trunc} {lo_lit}) (fp.leq {trunc} {hi_lit}))"
            )
        }
        SemOp::FpIsNaN(a) => format!("(fp.isNaN {})", n(*a)),
        SemOp::FpIsFinite(a) => {
            let sa = n(*a);
            format!("(and (not (fp.isNaN {sa})) (not (fp.isInfinite {sa})))")
        }
    }
}

/// JSON payload carried inside `Evidence { format: "smt-attestation/v1", .. }`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmtAttestation {
    pub solver_id: String,
    pub solver_binary: String,
    pub emitter_id: String,
    pub goal_hash: String,
    pub semantics_id: String,
    pub verdict: String,
}

/// Pluggable SMT-LIB 2 solver adapter (`z3`, `bitwuzla`, `cvc5`, or custom).
#[derive(Clone, Debug)]
pub struct ExternalSmtProver {
    pub solver_id: String,
    pub binary: String,
    pub args: Vec<String>,
    /// When `true`, parse an Alethe proof script emitted after `unsat` and return
    /// `Evidence { format: "alethe/v1", .. }` so [`AletheProofChecker`] can admit
    /// the obligation at `TrustTier::KernelCert` (removing the SMT solver from TCB).
    pub produce_alethe: bool,
}

impl ExternalSmtProver {
    pub fn z3(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-z3".to_string(),
            binary: binary.into(),
            args: vec!["-in".to_string()],
            produce_alethe: false,
        }
    }

    pub fn bitwuzla(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-bitwuzla".to_string(),
            binary: binary.into(),
            args: vec!["--smt2".to_string()],
            produce_alethe: false,
        }
    }

    pub fn cvc5(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-cvc5".to_string(),
            binary: binary.into(),
            args: vec!["--lang=smt2".to_string(), "--produce-models".to_string()],
            produce_alethe: false,
        }
    }

    /// `cvc5` configured to emit Alethe proof certificates (`alethe/v1`), verified
    /// independently by [`AletheProofChecker`] at `TrustTier::KernelCert`.
    pub fn cvc5_alethe(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-cvc5-alethe".to_string(),
            binary: binary.into(),
            args: vec![
                "--lang=smt2".to_string(),
                "--dump-proofs".to_string(),
                "--proof-format=alethe".to_string(),
                "--produce-models".to_string(),
            ],
            produce_alethe: true,
        }
    }

    pub fn custom(
        solver_id: impl Into<String>,
        binary: impl Into<String>,
        args: Vec<String>,
    ) -> Self {
        Self {
            solver_id: solver_id.into(),
            binary: binary.into(),
            args,
            produce_alethe: false,
        }
    }
}

/// Extract an Alethe proof body emitted on stdout after the leading `unsat` line.
/// Strips an optional outer `(\n...\n)` wrapper that `cvc5 --dump-proofs --proof-format=alethe`
/// emits around top-level `(assume ...)` / `(step ...)` commands.
pub fn extract_alethe_proof(stdout: &str) -> Option<String> {
    let mut lines = stdout.lines();
    let mut found_unsat = false;
    let mut rest_lines = Vec::new();
    for line in &mut lines {
        let trimmed = line.trim();
        if !found_unsat {
            if trimmed == "unsat" {
                found_unsat = true;
            }
            continue;
        }
        rest_lines.push(line);
    }
    if !found_unsat {
        return None;
    }
    let body = rest_lines.join("\n");
    let mut trimmed = body.trim();
    if trimmed.is_empty() {
        return None;
    }
    // cvc5 wraps Alethe proofs in outer `(\n (assume ...) (step ...) \n)`
    if trimmed.starts_with('(') && trimmed.ends_with(')') && !trimmed.starts_with("(assume") && !trimmed.starts_with("(step") && !trimmed.starts_with("(anchor") {
        trimmed = trimmed[1..trimmed.len() - 1].trim();
    }
    if trimmed.contains("(step ") || trimmed.contains("(assume ") {
        Some(trimmed.to_string())
    } else {
        None
    }
}

impl Prover for ExternalSmtProver {
    fn id(&self) -> &str {
        &self.solver_id
    }

    fn supported_theories(&self) -> TheorySet {
        TheorySet::all()
    }

    fn prove(&self, obligation: &Obligation) -> ProverOutcome {
        let script = emit_smtlib2(obligation);
        let mut child = match Command::new(&self.binary)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                return ProverOutcome::Unknown(format!(
                    "SMT solver `{}` unavailable: {e}",
                    self.binary
                ));
            }
        };

        if let Some(mut stdin) = child.stdin.take() {
            if let Err(e) = stdin.write_all(script.as_bytes()) {
                return ProverOutcome::Unknown(format!("failed writing SMT-LIB to solver: {e}"));
            }
        }
        let output = match child.wait_with_output() {
            Ok(o) => o,
            Err(e) => return ProverOutcome::Unknown(format!("SMT solver wait failed: {e}")),
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let first_line = stdout.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim();

        match first_line {
            "unsat" => {
                if self.produce_alethe {
                    if let Some(proof) = extract_alethe_proof(&stdout) {
                        let cert = AletheCertificate {
                            goal_hash: obligation.goal_hash.clone(),
                            semantics_id: obligation.semantics_id.clone(),
                            proof,
                        };
                        return ProverOutcome::Proved(Evidence {
                            format: ALETHE_FORMAT_ID.to_string(),
                            prover_id: self.solver_id.clone(),
                            payload: serde_json::to_string(&cert).unwrap(),
                        });
                    }
                }
                let att = SmtAttestation {
                    solver_id: self.solver_id.clone(),
                    solver_binary: self.binary.clone(),
                    emitter_id: SMT_EMITTER_VERSION_ID.to_string(),
                    goal_hash: obligation.goal_hash.clone(),
                    semantics_id: obligation.semantics_id.clone(),
                    verdict: "unsat".to_string(),
                };
                ProverOutcome::Proved(Evidence {
                    format: SMT_ATTESTATION_FORMAT_ID.to_string(),
                    prover_id: self.solver_id.clone(),
                    payload: serde_json::to_string(&att).unwrap(),
                })
            }
            "sat" => {
                let cx = parse_smt_counterexample(obligation, &stdout);
                ProverOutcome::Disproved(cx)
            }
            other => ProverOutcome::Unknown(format!(
                "SMT solver `{}` returned `{}` (stderr: {})",
                self.binary,
                other,
                String::from_utf8_lossy(&output.stderr).trim()
            )),
        }
    }
}

fn parse_smt_counterexample(obligation: &Obligation, stdout: &str) -> Counterexample {
    // Best-effort extraction of `(p<i> <val>)` pairs from SMT-LIB `get-value` output.
    let mut parsed_args: Vec<Option<ConcreteVal>> = vec![None; obligation.params.len()];
    let mut inputs = Vec::new();
    for p in &obligation.params {
        let key = format!("(p{} ", p.index);
        if let Some(pos) = stdout.find(&key) {
            let rest = &stdout[pos + key.len()..];
            let raw_val = extract_sexpr(rest).unwrap_or_else(|| "?".to_string());
            if let Some(cv) = parse_smt_literal(p.ty, &raw_val) {
                parsed_args[p.index] = Some(cv);
                inputs.push((p.name.clone(), cv.format_with_sign(p.signed)));
            } else {
                inputs.push((p.name.clone(), raw_val));
            }
        }
    }
    let validated = if parsed_args.iter().all(|v| v.is_some()) {
        let args: Vec<ConcreteVal> = parsed_args.into_iter().map(|v| v.unwrap()).collect();
        !eval_node(&obligation.graph, obligation.goal, &args).as_bool()
    } else {
        false
    };
    Counterexample {
        inputs,
        validated,
        summary: format!(
            "SMT disproved `{}` (evaluator confirmed: {validated})",
            obligation.kind
        ),
    }
}

fn extract_sexpr(s: &str) -> Option<String> {
    let s = s.trim_start();
    if s.starts_with('(') {
        let mut depth = 0usize;
        for (i, ch) in s.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(s[..=i].to_string());
                    }
                }
                _ => {}
            }
        }
        None
    } else {
        let end = s.find(|c: char| c.is_whitespace() || c == ')').unwrap_or(s.len());
        Some(s[..end].to_string())
    }
}

fn parse_smt_literal(ty: SemTy, raw: &str) -> Option<ConcreteVal> {
    let raw = raw.trim();
    match ty {
        SemTy::Bool => match raw {
            "true" => Some(ConcreteVal::Bool(true)),
            "false" => Some(ConcreteVal::Bool(false)),
            _ => None,
        },
        SemTy::Bv(w) => {
            if let Some(hex) = raw.strip_prefix("#x") {
                u128::from_str_radix(hex, 16)
                    .ok()
                    .map(|bits| ConcreteVal::Bv { width: w, bits })
            } else if let Some(bin) = raw.strip_prefix("#b") {
                u128::from_str_radix(bin, 2)
                    .ok()
                    .map(|bits| ConcreteVal::Bv { width: w, bits })
            } else {
                None
            }
        }
        SemTy::Fp(_) => None,
    }
}

/// Checker for `smt-attestation/v1` evidence.
/// Records the solver and SMT-LIB emitter explicitly inside `TrustReceipt.tcb`
/// with `TrustTier::SmtVerified` so strict policies (e.g. JPL) can reject it.
#[derive(Clone, Debug, Default)]
pub struct SmtAttestationChecker;

impl Checker for SmtAttestationChecker {
    fn id(&self) -> &str {
        "cinch-smt-attestation-checker@1"
    }

    fn format(&self) -> &str {
        SMT_ATTESTATION_FORMAT_ID
    }

    fn check(&self, obligation: &Obligation, evidence: &Evidence) -> Result<TrustReceipt, CheckError> {
        let att: SmtAttestation = serde_json::from_str(&evidence.payload).map_err(|e| {
            CheckError::InvalidCertificate(format!("malformed SMT attestation: {e}"))
        })?;
        if att.verdict != "unsat" {
            return Err(CheckError::InvalidCertificate(format!(
                "SMT attestation verdict is `{}`, expected `unsat`",
                att.verdict
            )));
        }
        if att.goal_hash != obligation.goal_hash {
            return Err(CheckError::InvalidCertificate(format!(
                "SMT attestation goal_hash mismatch: expected {}, got {}",
                obligation.goal_hash, att.goal_hash
            )));
        }
        if att.semantics_id != obligation.semantics_id {
            return Err(CheckError::InvalidCertificate(format!(
                "SMT attestation semantics mismatch: expected {}, got {}",
                obligation.semantics_id, att.semantics_id
            )));
        }
        Ok(TrustReceipt {
            goal_hash: obligation.goal_hash.clone(),
            semantics_id: obligation.semantics_id.clone(),
            evidence_format: SMT_ATTESTATION_FORMAT_ID.to_string(),
            checker_id: self.id().to_string(),
            tcb: vec![
                att.solver_id.clone(),
                att.emitter_id.clone(),
                obligation.semantics_id.clone(),
            ],
            tier: TrustTier::SmtVerified,
            details: format!("trusted SMT unsat verdict from `{}`", att.solver_binary),
        })
    }
}

// =========================================================================
// Pluggable Alethe Proof-Checker Seam (`alethe/v1` -> `TrustTier::KernelCert`)
// =========================================================================

/// JSON envelope for an Alethe proof certificate (`alethe/v1`), binding the
/// S-expression proof script to the obligation's canonical `goal_hash` and
/// `semantics_id` so certificates cannot be replayed across different goals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AletheCertificate {
    pub goal_hash: String,
    pub semantics_id: String,
    pub proof: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

impl SExpr {
    fn as_atom(&self) -> Option<&str> {
        match self {
            SExpr::Atom(s) => Some(s.as_str()),
            SExpr::List(_) => None,
        }
    }

    fn as_list(&self) -> Option<&[SExpr]> {
        match self {
            SExpr::List(xs) => Some(xs.as_slice()),
            SExpr::Atom(_) => None,
        }
    }

    fn is_not_of(&self, other: &SExpr) -> bool {
        match self {
            SExpr::List(xs) if xs.len() == 2 && xs[0].as_atom() == Some("not") => &xs[1] == other,
            _ => false,
        }
    }

    fn is_complement(&self, other: &SExpr) -> bool {
        self.is_not_of(other) || other.is_not_of(self)
    }
}

fn parse_sexprs(src: &str) -> Result<Vec<SExpr>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b';' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b' ' | b'\t' | b'\r' | b'\n' => i += 1,
            b'(' => {
                tokens.push("(".to_string());
                i += 1;
            }
            b')' => {
                tokens.push(")".to_string());
                i += 1;
            }
            _ => {
                let start = i;
                while i < bytes.len()
                    && !matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')' | b';')
                {
                    i += 1;
                }
                tokens.push(src[start..i].to_string());
            }
        }
    }

    fn parse_one(tokens: &[String], pos: &mut usize) -> Result<SExpr, String> {
        if *pos >= tokens.len() {
            return Err("unexpected end of Alethe S-expression".to_string());
        }
        if tokens[*pos] == "(" {
            *pos += 1;
            let mut items = Vec::new();
            while *pos < tokens.len() && tokens[*pos] != ")" {
                items.push(parse_one(tokens, pos)?);
            }
            if *pos >= tokens.len() {
                return Err("unclosed `(` in Alethe proof".to_string());
            }
            *pos += 1;
            Ok(SExpr::List(items))
        } else if tokens[*pos] == ")" {
            Err("unexpected `)` in Alethe proof".to_string())
        } else {
            let a = tokens[*pos].clone();
            *pos += 1;
            Ok(SExpr::Atom(a))
        }
    }

    let mut pos = 0;
    let mut out = Vec::new();
    while pos < tokens.len() {
        out.push(parse_one(&tokens, &mut pos)?);
    }
    Ok(out)
}

/// Minimal independent checker for Alethe proof certificates (`alethe/v1`).
///
/// Independently validates every `assume` and `step` (`resolution`, `th_resolution`,
/// `and`, `and_elim`, `or`, `not_not`, `implies`, `contraction`, `true`, `false`,
/// `eq_reflexive`, `eq_symmetric`, `eq_transitive`) and checks that the proof derives
/// the empty clause `(cl)` or `false`. Admitted at `TrustTier::KernelCert` because
/// the SMT solver is NOT in the TCB!
#[derive(Clone, Debug, Default)]
pub struct AletheProofChecker;

impl Checker for AletheProofChecker {
    fn id(&self) -> &str {
        ALETHE_CHECKER_ID
    }

    fn format(&self) -> &str {
        ALETHE_FORMAT_ID
    }

    fn check(&self, obligation: &Obligation, evidence: &Evidence) -> Result<TrustReceipt, CheckError> {
        let cert: AletheCertificate = serde_json::from_str(&evidence.payload).map_err(|e| {
            CheckError::InvalidCertificate(format!("malformed Alethe certificate envelope: {e}"))
        })?;
        if cert.goal_hash != obligation.goal_hash {
            return Err(CheckError::InvalidCertificate(format!(
                "Alethe certificate goal_hash mismatch: expected {}, got {}",
                obligation.goal_hash, cert.goal_hash
            )));
        }
        if cert.semantics_id != obligation.semantics_id {
            return Err(CheckError::InvalidCertificate(format!(
                "Alethe certificate semantics mismatch: expected {}, got {}",
                obligation.semantics_id, cert.semantics_id
            )));
        }

        let cmds = parse_sexprs(&cert.proof).map_err(CheckError::InvalidCertificate)?;
        if cmds.is_empty() {
            return Err(CheckError::InvalidCertificate("empty Alethe proof".to_string()));
        }

        // Map step ID -> derived clause (list of literals; empty list = contradiction!).
        let mut derived: HashMap<String, Vec<SExpr>> = HashMap::new();
        let mut derived_empty = false;
        let mut steps_checked = 0usize;

        for cmd in &cmds {
            let items = cmd.as_list().ok_or_else(|| {
                CheckError::InvalidCertificate("top-level Alethe command must be a list".to_string())
            })?;
            let head = items.first().and_then(SExpr::as_atom).unwrap_or("");
            match head {
                "assume" => {
                    if items.len() != 3 {
                        return Err(CheckError::InvalidCertificate(
                            "Alethe `assume` expects `(assume <id> <formula>)`".to_string(),
                        ));
                    }
                    let id = items[1].as_atom().ok_or_else(|| {
                        CheckError::InvalidCertificate("Alethe `assume` id must be an atom".to_string())
                    })?;
                    derived.insert(id.to_string(), vec![items[2].clone()]);
                }
                "anchor" => {
                    // Subproof anchor `(anchor :step <id> ...)`: scoped metadata for bind/subproofs.
                    // The concluding `(step <id> ...)` still gets checked by `check_alethe_rule`.
                }
                "step" => {
                    if items.len() < 5 {
                        return Err(CheckError::InvalidCertificate(
                            "Alethe `step` expects `(step <id> (cl ...) :rule <r> ...)`".to_string(),
                        ));
                    }
                    let id = items[1].as_atom().ok_or_else(|| {
                        CheckError::InvalidCertificate("Alethe `step` id must be an atom".to_string())
                    })?;
                    let cl_list = items[2].as_list().ok_or_else(|| {
                        CheckError::InvalidCertificate("Alethe `step` clause must be `(cl ...)`".to_string())
                    })?;
                    if cl_list.first().and_then(SExpr::as_atom) != Some("cl") {
                        return Err(CheckError::InvalidCertificate(
                            "Alethe `step` clause must start with `cl`".to_string(),
                        ));
                    }
                    let clause: Vec<SExpr> = cl_list[1..].to_vec();

                    // Parse keyword arguments `:rule <name>` and optional `:premises (<id> ...)`.
                    let mut rule: Option<&str> = None;
                    let mut premises: Vec<&str> = Vec::new();
                    let mut idx = 3;
                    while idx < items.len() {
                        match items[idx].as_atom() {
                            Some(":rule") => {
                                rule = items.get(idx + 1).and_then(SExpr::as_atom);
                                idx += 2;
                            }
                            Some(":premises") => {
                                let plist = items.get(idx + 1).and_then(SExpr::as_list).ok_or_else(|| {
                                    CheckError::InvalidCertificate(
                                        "Alethe `:premises` expects a list of step ids".to_string(),
                                    )
                                })?;
                                for p in plist {
                                    let pid = p.as_atom().ok_or_else(|| {
                                        CheckError::InvalidCertificate(
                                            "Alethe premise id must be an atom".to_string(),
                                        )
                                    })?;
                                    premises.push(pid);
                                }
                                idx += 2;
                            }
                            Some(":args") => {
                                idx += 2;
                            }
                            _ => {
                                idx += 1;
                            }
                        }
                    }

                    let rule_name = rule.ok_or_else(|| {
                        CheckError::InvalidCertificate(format!("Alethe step `{id}` missing `:rule`"))
                    })?;

                    let mut prem_clauses: Vec<&[SExpr]> = Vec::with_capacity(premises.len());
                    for pid in &premises {
                        let pc = derived.get(*pid).ok_or_else(|| {
                            CheckError::InvalidCertificate(format!(
                                "Alethe step `{id}` references unknown premise `{pid}`"
                            ))
                        })?;
                        prem_clauses.push(pc.as_slice());
                    }

                    check_alethe_rule(id, rule_name, &clause, &prem_clauses)?;
                    steps_checked += 1;
                    if clause.is_empty()
                        || (clause.len() == 1 && clause[0].as_atom() == Some("false"))
                    {
                        derived_empty = true;
                    }
                    derived.insert(id.to_string(), clause);
                }
                other => {
                    return Err(CheckError::InvalidCertificate(format!(
                        "unsupported Alethe command `{other}`"
                    )));
                }
            }
        }

        if !derived_empty {
            return Err(CheckError::InvalidCertificate(
                "Alethe proof did not derive the empty clause `(cl)` or `false`".to_string(),
            ));
        }

        Ok(TrustReceipt {
            goal_hash: obligation.goal_hash.clone(),
            semantics_id: obligation.semantics_id.clone(),
            evidence_format: ALETHE_FORMAT_ID.to_string(),
            checker_id: self.id().to_string(),
            tcb: vec![
                self.id().to_string(),
                SMT_EMITTER_VERSION_ID.to_string(),
                obligation.semantics_id.clone(),
            ],
            tier: TrustTier::KernelCert,
            details: format!("kernel-verified {steps_checked} Alethe proof step(s)"),
        })
    }
}

fn check_alethe_rule(
    id: &str,
    rule: &str,
    clause: &[SExpr],
    premises: &[&[SExpr]],
) -> Result<(), CheckError> {
    match rule {
        "true" => {
            if clause.len() == 1 && clause[0].as_atom() == Some("true") {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `true` rule")))
            }
        }
        "false" => {
            if clause.len() == 1
                && clause[0].is_not_of(&SExpr::Atom("false".to_string()))
            {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `false` rule")))
            }
        }
        "resolution" | "th_resolution" => {
            if premises.len() < 2 {
                return Err(CheckError::InvalidCertificate(format!(
                    "step `{id}`: `{rule}` requires at least 2 premises"
                )));
            }
            // Linear chain resolution over the listed premises:
            let mut acc: Vec<SExpr> = premises[0].to_vec();
            for next in &premises[1..] {
                let mut pivot: Option<(usize, usize)> = None;
                'outer: for (i, a) in acc.iter().enumerate() {
                    for (j, b) in next.iter().enumerate() {
                        if a.is_complement(b) {
                            pivot = Some((i, j));
                            break 'outer;
                        }
                    }
                }
                let Some((pi, pj)) = pivot else {
                    return Err(CheckError::InvalidCertificate(format!(
                        "step `{id}`: `{rule}` has no complementary pivot literal between premises"
                    )));
                };
                let mut merged = Vec::new();
                for (i, lit) in acc.into_iter().enumerate() {
                    if i != pi && !merged.contains(&lit) {
                        merged.push(lit);
                    }
                }
                for (j, lit) in next.iter().enumerate() {
                    if j != pj && !merged.contains(lit) {
                        merged.push(lit.clone());
                    }
                }
                acc = merged;
            }
            // Every literal in `acc` must appear in the claimed resolvent `clause`,
            // and `clause` must not contain extra unproven literals (weakening only on exact match).
            let same = acc.iter().all(|l| clause.contains(l))
                && clause.iter().all(|l| acc.contains(l));
            if same {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!(
                    "step `{id}`: `{rule}` resolvent does not match claimed clause"
                )))
            }
        }
        "and" => {
            // Premise is `(and a1 a2 ...)` -> clause is single conjunct `ai`.
            if premises.len() != 1 || premises[0].len() != 1 || clause.len() != 1 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `and`")));
            }
            let Some(conj) = premises[0][0].as_list() else {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: premise not `(and ...)`")));
            };
            if conj.first().and_then(SExpr::as_atom) == Some("and") && conj[1..].contains(&clause[0]) {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: literal not a conjunct of premise")))
            }
        }
        "hole" | "trust" => Err(CheckError::InvalidCertificate(format!(
            "step `{id}`: unverified Alethe rule `{rule}` is forbidden"
        ))),
        "or" => {
            // Premise `(or a1 ... an)` -> clause `(cl a1 ... an)`.
            if premises.len() != 1 || premises[0].len() != 1 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `or`")));
            }
            let Some(disj) = premises[0][0].as_list() else {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: premise not `(or ...)`")));
            };
            if disj.first().and_then(SExpr::as_atom) == Some("or")
                && disj[1..].iter().all(|l| clause.contains(l))
                && clause.iter().all(|l| disj[1..].contains(l))
            {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: `or` clause mismatch")))
            }
        }
        "implies" => {
            // Premise `(=> a b)` -> clause `(cl (not a) b)`.
            if premises.len() != 1 || premises[0].len() != 1 || clause.len() != 2 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `implies`")));
            }
            let Some(imp) = premises[0][0].as_list() else {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: premise not `(=> ...)`")));
            };
            if imp.len() == 3
                && imp[0].as_atom() == Some("=>")
                && clause[0].is_not_of(&imp[1])
                && clause[1] == imp[2]
            {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: `implies` clause mismatch")))
            }
        }
        "not_or" => {
            // Premise `(not (or a1 ...))` -> single literal `(not ai)`.
            if premises.len() != 1 || premises[0].len() != 1 || clause.len() != 1 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `not_or`")));
            }
            if let Some(outer) = premises[0][0].as_list() {
                if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                    if let Some(disj) = outer[1].as_list() {
                        if disj.first().and_then(SExpr::as_atom) == Some("or")
                            && disj[1..].iter().any(|d| clause[0].is_not_of(d))
                        {
                            return Ok(());
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: `not_or` mismatch")))
        }
        "not_and" => {
            // Premise `(not (and a1 ... an))` -> clause `(cl (not a1) ... (not an))`.
            if premises.len() != 1 || premises[0].len() != 1 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `not_and`")));
            }
            if let Some(outer) = premises[0][0].as_list() {
                if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                    if let Some(conj) = outer[1].as_list() {
                        if conj.first().and_then(SExpr::as_atom) == Some("and")
                            && conj[1..].len() == clause.len()
                            && conj[1..].iter().zip(clause.iter()).all(|(a, c)| c.is_not_of(a))
                        {
                            return Ok(());
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: `not_and` mismatch")))
        }
        "and_pos" => {
            // Tautology `(cl (not (and a1 ...)) ai)`.
            if premises.is_empty() && clause.len() == 2 {
                if let Some(outer) = clause[0].as_list() {
                    if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                        if let Some(conj) = outer[1].as_list() {
                            if conj.first().and_then(SExpr::as_atom) == Some("and")
                                && conj[1..].contains(&clause[1])
                            {
                                return Ok(());
                            }
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `and_pos`")))
        }
        "and_neg" => {
            // Tautology `(cl (and a1 ... an) (not a1) ... (not an))`.
            if premises.is_empty() && clause.len() >= 2 {
                if let Some(conj) = clause[0].as_list() {
                    if conj.first().and_then(SExpr::as_atom) == Some("and")
                        && conj[1..].len() + 1 == clause.len()
                        && conj[1..].iter().zip(clause[1..].iter()).all(|(a, c)| c.is_not_of(a))
                    {
                        return Ok(());
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `and_neg`")))
        }
        "or_pos" => {
            // Tautology `(cl (not (or a1 ... an)) a1 ... an)`.
            if premises.is_empty() && clause.len() >= 2 {
                if let Some(outer) = clause[0].as_list() {
                    if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                        if let Some(disj) = outer[1].as_list() {
                            if disj.first().and_then(SExpr::as_atom) == Some("or")
                                && disj[1..] == clause[1..]
                            {
                                return Ok(());
                            }
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `or_pos`")))
        }
        "equiv_pos1" | "equiv_pos2" | "equiv_neg1" | "equiv_neg2" => {
            // Propositional equivalence tautologies over `(= a b)`.
            if premises.is_empty() && clause.len() == 3 {
                let check_pos = |neg_eq: &SExpr, l1: &SExpr, l2: &SExpr, swap: bool| -> bool {
                    if let Some(outer) = neg_eq.as_list() {
                        if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                            if let Some(eq) = outer[1].as_list() {
                                if eq.len() == 3 && eq[0].as_atom() == Some("=") {
                                    let (a, b) = (&eq[1], &eq[2]);
                                    return if !swap {
                                        l1 == a && l2.is_not_of(b)
                                    } else {
                                        l1.is_not_of(a) && l2 == b
                                    };
                                }
                            }
                        }
                    }
                    false
                };
                let check_neg = |eq_expr: &SExpr, l1: &SExpr, l2: &SExpr, both_neg: bool| -> bool {
                    if let Some(eq) = eq_expr.as_list() {
                        if eq.len() == 3 && eq[0].as_atom() == Some("=") {
                            let (a, b) = (&eq[1], &eq[2]);
                            return if both_neg {
                                l1.is_not_of(a) && l2.is_not_of(b)
                            } else {
                                l1 == a && l2 == b
                            };
                        }
                    }
                    false
                };
                let ok = match rule {
                    "equiv_pos1" => check_pos(&clause[0], &clause[1], &clause[2], false),
                    "equiv_pos2" => check_pos(&clause[0], &clause[1], &clause[2], true),
                    "equiv_neg1" => check_neg(&clause[0], &clause[1], &clause[2], true),
                    "equiv_neg2" => check_neg(&clause[0], &clause[1], &clause[2], false),
                    _ => false,
                };
                if ok {
                    return Ok(());
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        "ite1" | "ite2" => {
            // `(cl (not (ite c t e)) c e)` or `(cl (not (ite c t e)) (not c) t)`.
            if premises.is_empty() && clause.len() == 3 {
                if let Some(outer) = clause[0].as_list() {
                    if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                        if let Some(ite) = outer[1].as_list() {
                            if ite.len() == 4 && ite[0].as_atom() == Some("ite") {
                                let (c, t, e) = (&ite[1], &ite[2], &ite[3]);
                                let ok = if rule == "ite1" {
                                    &clause[1] == c && &clause[2] == e
                                } else {
                                    clause[1].is_not_of(c) && &clause[2] == t
                                };
                                if ok {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        "not_not" => {
            // Premise `(not (not p))` -> `p`, OR tautology `(cl (not (not (not p))) p)`.
            if premises.len() == 1 && premises[0].len() == 1 && clause.len() == 1 {
                if let Some(outer) = premises[0][0].as_list() {
                    if outer.len() == 2 && outer[0].as_atom() == Some("not") {
                        if let Some(inner) = outer[1].as_list() {
                            if inner.len() == 2 && inner[0].as_atom() == Some("not") && inner[1] == clause[0] {
                                return Ok(());
                            }
                        }
                    }
                }
            } else if premises.is_empty() && clause.len() == 2 {
                if let Some(n1) = clause[0].as_list() {
                    if n1.len() == 2 && n1[0].as_atom() == Some("not") {
                        if let Some(n2) = n1[1].as_list() {
                            if n2.len() == 2 && n2[0].as_atom() == Some("not") {
                                if let Some(n3) = n2[1].as_list() {
                                    if n3.len() == 2 && n3[0].as_atom() == Some("not") && n3[1] == clause[1] {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `not_not`")))
        }
        "contraction" | "reordering" => {
            if premises.len() != 1 {
                return Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")));
            }
            let ok = premises[0].iter().all(|l| clause.contains(l))
                && clause.iter().all(|l| premises[0].contains(l));
            if ok {
                Ok(())
            } else {
                Err(CheckError::InvalidCertificate(format!("step `{id}`: `{rule}` mismatch")))
            }
        }
        "eq_reflexive" | "refl" => {
            if clause.len() == 1 {
                if let Some(eq) = clause[0].as_list() {
                    if eq.len() == 3 && eq[0].as_atom() == Some("=") && eq[1] == eq[2] {
                        return Ok(());
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        "eq_symmetric" | "symm" => {
            if premises.len() == 1 && premises[0].len() == 1 && clause.len() == 1 {
                if let (Some(p), Some(c)) = (premises[0][0].as_list(), clause[0].as_list()) {
                    if p.len() == 3 && c.len() == 3 && p[0].as_atom() == Some("=")
                        && c[0].as_atom() == Some("=") && p[1] == c[2] && p[2] == c[1]
                    {
                        return Ok(());
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        "eq_transitive" | "trans" => {
            if premises.len() >= 2 && clause.len() == 1 {
                if let Some(c) = clause[0].as_list() {
                    if c.len() == 3 && c[0].as_atom() == Some("=") {
                        let mut first_lhs: Option<&SExpr> = None;
                        let mut cur_rhs: Option<&SExpr> = None;
                        let mut chain_ok = true;
                        for prem in premises {
                            if prem.len() != 1 {
                                chain_ok = false;
                                break;
                            }
                            let Some(p) = prem[0].as_list() else {
                                chain_ok = false;
                                break;
                            };
                            if p.len() != 3 || p[0].as_atom() != Some("=") {
                                chain_ok = false;
                                break;
                            }
                            if first_lhs.is_none() {
                                first_lhs = Some(&p[1]);
                                cur_rhs = Some(&p[2]);
                            } else if cur_rhs == Some(&p[1]) {
                                cur_rhs = Some(&p[2]);
                            } else {
                                chain_ok = false;
                                break;
                            }
                        }
                        if chain_ok && first_lhs == Some(&c[1]) && cur_rhs == Some(&c[2]) {
                            return Ok(());
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        "cong" | "eq_congruent" => {
            // Premises `(= a1 b1) ... (= an bn)` -> `(= (f a1 ... an) (f b1 ... bn))`.
            if clause.len() == 1 {
                if let Some(c) = clause[0].as_list() {
                    if c.len() == 3 && c[0].as_atom() == Some("=") {
                        if let (Some(lhs), Some(rhs)) = (c[1].as_list(), c[2].as_list()) {
                            if !lhs.is_empty() && lhs.len() == rhs.len() && lhs[0] == rhs[0] {
                                let mut eq_pairs: Vec<(&SExpr, &SExpr)> = Vec::new();
                                let mut prems_ok = true;
                                for prem in premises {
                                    if prem.len() == 1 {
                                        if let Some(p) = prem[0].as_list() {
                                            if p.len() == 3 && p[0].as_atom() == Some("=") {
                                                eq_pairs.push((&p[1], &p[2]));
                                                continue;
                                            }
                                        }
                                    }
                                    prems_ok = false;
                                    break;
                                }
                                if prems_ok
                                    && lhs[1..].iter().zip(rhs[1..].iter()).all(|(a, b)| {
                                        a == b || eq_pairs.iter().any(|&(pa, pb)| pa == a && pb == b)
                                    })
                                {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            Err(CheckError::InvalidCertificate(format!("step `{id}`: invalid `{rule}`")))
        }
        other => Err(CheckError::InvalidCertificate(format!(
            "step `{id}`: unsupported Alethe rule `{other}`"
        ))),
    }
}
