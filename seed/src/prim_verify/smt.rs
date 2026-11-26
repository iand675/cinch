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

/// Emit a complete, standalone SMT-LIB 2 script checking `!obligation.goal`.
/// `unsat` means `obligation.goal` is valid; `sat` yields a counterexample.
pub fn emit_smtlib2(obligation: &Obligation) -> String {
    let mut out = String::new();
    out.push_str(&format!("; Cinch primitive verification obligation: {}\n", obligation.name));
    out.push_str(&format!("; Semantics: {}\n", obligation.semantics_id));
    out.push_str(&format!("; Kind: {}\n", obligation.kind));
    out.push_str(&format!("; Goal hash: {}\n", obligation.goal_hash));

    let theories = obligation.theories();
    let logic = if theories.fp { "QF_FPBV" } else { "QF_BV" };
    out.push_str(&format!("(set-logic {logic})\n"));
    out.push_str("(set-option :produce-models true)\n");

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
}

impl ExternalSmtProver {
    pub fn z3(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-z3".to_string(),
            binary: binary.into(),
            args: vec!["-in".to_string()],
        }
    }

    pub fn bitwuzla(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-bitwuzla".to_string(),
            binary: binary.into(),
            args: vec!["--smt2".to_string()],
        }
    }

    pub fn cvc5(binary: impl Into<String>) -> Self {
        Self {
            solver_id: "smt-cvc5".to_string(),
            binary: binary.into(),
            args: vec!["--lang=smt2".to_string(), "--produce-models".to_string()],
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
        }
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
