//! Pluggable formal verification framework for Cinch inline target-IR primitives.
//!
//! Architecture (five replaceable seams):
//! 1. [`IrSemantics`] — versioned target-IR parser & lowering into [`sem::SemGraph`]
//!    (e.g. `llvm-sem@21.1`; extensible to `wasm`, Sail ISA models, etc.).
//! 2. [`sem::Obligation`] — canonical, content-addressed verification goal derived
//!    from the shared semantic DAG (single source of truth across all backends).
//! 3. [`Prover`] — untrusted proof search / falsification plugins (`CdclLratProver`,
//!    `ExternalSatProver` for CaDiCaL, `ExternalSmtProver` for Z3/Bitwuzla/cvc5).
//! 4. [`Checker`] — TCB-bearing evidence verifiers (`KernelLratChecker`,
//!    `CompiledLratChecker`, `SmtAttestationChecker`), emitting explicit [`TrustReceipt`]s.
//! 5. [`Policy`] & [`EvidenceStore`] — admission rules (e.g. `dev` vs `jpl`) and
//!    content-addressed caching keyed by `(goal_hash, evidence_format)`.

pub mod bitblast;
pub mod llvm_ir;
pub mod prelude_bridge;
pub mod sem;
pub mod smt;

use bitblast::{CdclLratProver, CompiledLratChecker, ExternalSatProver, KernelLratChecker};
use llvm_ir::LlvmSemantics;
use prelude_bridge::{extract_file_obligations, ExtractedPrimBody};
use sem::{Obligation, SemParam, SemProgram, SemTy, TheorySet};
use serde::{Deserialize, Serialize};
use smt::{emit_smtlib2, ExternalSmtProver, SmtAttestationChecker};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

// =========================================================================
// Pluggable Core Traits & Data Types
// =========================================================================

/// Verification trust tiers, ordered from minimal TCB to unverified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TrustTier {
    /// Native proof term checked inside Cinch's dependent core kernel.
    KernelProof,
    /// Solver certificate (e.g. LRAT) checked by the minimal kernel verifier.
    KernelCert,
    /// Solver certificate checked by a compiled checker (adds compiler to TCB).
    CompiledCert,
    /// Trusted SMT solver verdict (`unsat` without kernel proof reconstruction).
    SmtVerified,
    /// Unverified assumption (containment-only or `unsafe`).
    Assumed,
}

impl fmt::Display for TrustTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrustTier::KernelProof => write!(f, "kernel-proof"),
            TrustTier::KernelCert => write!(f, "kernel-cert"),
            TrustTier::CompiledCert => write!(f, "compiled-cert"),
            TrustTier::SmtVerified => write!(f, "smt-verified"),
            TrustTier::Assumed => write!(f, "assumed"),
        }
    }
}

/// Compilation strictness mode for formal verification of `prim` bodies.
///
/// Selectable via `--prim-verify=strict|dev|lax|off` (or `--lax-prim-verify` /
/// `CINCH_PRIM_VERIFY=...`). Lax and off modes stay honest: unproved bodies
/// downgrade to `PrimTier::Assumed` with an explicit `TrustTier::Assumed`
/// receipt rather than forging a verified contract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimVerifyMode {
    /// Enforce `jpl` strictness under `#[profile("jpl")]`, `dev` elsewhere.
    #[default]
    Default,
    /// Enforce `Policy::jpl_strict()` (`KernelCert`/`KernelProof` only) everywhere.
    Strict,
    /// Permissive fast-iteration mode: uses a tighter SAT budget, admits all
    /// verified tiers (even in `jpl` modules), downgrades `Unverified` / missing
    /// solver prerequisites to an explicit `TrustTier::Assumed` receipt, and
    /// still fails on actively `Disproved` counterexamples.
    Lax,
    /// Skip running provers/checkers during compilation; records an explicit
    /// `TrustTier::Assumed` receipt (`cinch-verify-off@1`) and keeps the prim
    /// at `PrimTier::Assumed`.
    Off,
}

impl PrimVerifyMode {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "default" | "dev" => Some(Self::Default),
            "strict" | "jpl" => Some(Self::Strict),
            "lax" => Some(Self::Lax),
            "off" | "none" | "skip" => Some(Self::Off),
            _ => None,
        }
    }

    pub fn from_env() -> Self {
        std::env::var("CINCH_PRIM_VERIFY")
            .ok()
            .and_then(|v| Self::parse(&v))
            .unwrap_or(Self::Default)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Strict => "strict",
            Self::Lax => "lax",
            Self::Off => "off",
        }
    }
}

/// Errors produced by an [`IrSemantics`] plugin.
#[derive(Clone, Debug)]
pub enum SemError {
    Parse(String),
    Unsupported(String),
}

impl fmt::Display for SemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemError::Parse(s) => write!(f, "parse error: {s}"),
            SemError::Unsupported(s) => write!(f, "unsupported: {s}"),
        }
    }
}

/// Seam 1: Pluggable, versioned formal semantics for an inline target-IR family.
pub trait IrSemantics: Send + Sync {
    /// Versioned semantics identifier (e.g. `"llvm-sem@21.1"`).
    fn id(&self) -> &str;
    /// Inline target-IR family name (e.g. `"llvm"`, `"wasm"`, `"asm"`).
    fn family(&self) -> &str;
    /// Parse and lower an inline body fragment into the shared `SemGraph`.
    fn lower(
        &self,
        fragment: &str,
        params: &[SemParam],
        ret_ty: SemTy,
    ) -> Result<SemProgram, SemError>;
}

/// Serialized proof artifact crossing the untrusted-prover / trusted-checker seam.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidence {
    /// Extensible format tag (e.g. `"lrat/v1"`, `"smt-attestation/v1"`, `"alethe/v1"`).
    pub format: String,
    /// Identifier of the untrusted prover that produced this artifact.
    pub prover_id: String,
    /// Format-specific payload.
    pub payload: String,
}

/// Concrete failing assignment produced when an obligation is disproved.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Counterexample {
    pub inputs: Vec<(String, String)>,
    /// True iff the shared concrete evaluator independently confirmed `!goal`.
    pub validated: bool,
    pub summary: String,
}

/// Result returned by an untrusted [`Prover`].
#[derive(Clone, Debug)]
pub enum ProverOutcome {
    Proved(Evidence),
    Disproved(Counterexample),
    Unknown(String),
}

/// Seam 2: Pluggable untrusted proof search / falsification backend.
pub trait Prover: Send + Sync {
    fn id(&self) -> &str;
    fn supported_theories(&self) -> TheorySet;
    fn prove(&self, obligation: &Obligation) -> ProverOutcome;
}

/// Structured proof receipt emitted by a [`Checker`], recording the exact TCB.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrustReceipt {
    pub goal_hash: String,
    pub semantics_id: String,
    pub evidence_format: String,
    pub checker_id: String,
    pub tcb: Vec<String>,
    pub tier: TrustTier,
    pub details: String,
}

/// Errors produced when checking [`Evidence`].
#[derive(Clone, Debug)]
pub enum CheckError {
    InvalidCertificate(String),
    CannotReconstruct(String),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckError::InvalidCertificate(s) => write!(f, "invalid certificate: {s}"),
            CheckError::CannotReconstruct(s) => write!(f, "cannot reconstruct formula: {s}"),
        }
    }
}

/// Seam 3: Pluggable trusted verifier for one [`Evidence::format`].
pub trait Checker: Send + Sync {
    fn id(&self) -> &str;
    fn format(&self) -> &str;
    fn check(&self, obligation: &Obligation, evidence: &Evidence) -> Result<TrustReceipt, CheckError>;
}

// =========================================================================
// Seam 4: Content-Addressed Evidence Store
// =========================================================================

/// Content-addressed cache of [`Evidence`] keyed by `(goal_hash, format)`.
/// Enables prove-once, check-deterministically workflows without keeping solvers installed.
#[derive(Clone, Debug, Default)]
pub struct EvidenceStore {
    memory: HashMap<(String, String), Evidence>,
    disk_dir: Option<PathBuf>,
}

impl EvidenceStore {
    pub fn in_memory() -> Self {
        Self::default()
    }

    pub fn with_disk_dir(dir: PathBuf) -> Self {
        Self { memory: HashMap::new(), disk_dir: Some(dir) }
    }

    fn disk_path(&self, goal_hash: &str, format: &str) -> Option<PathBuf> {
        let dir = self.disk_dir.as_ref()?;
        let safe_hash = goal_hash.replace(':', "_");
        let safe_fmt = format.replace('/', "_");
        Some(dir.join(format!("{safe_hash}.{safe_fmt}.json")))
    }

    pub fn get(&mut self, goal_hash: &str, format: &str) -> Option<Evidence> {
        let key = (goal_hash.to_string(), format.to_string());
        if let Some(ev) = self.memory.get(&key) {
            return Some(ev.clone());
        }
        if let Some(path) = self.disk_path(goal_hash, format) {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(ev) = serde_json::from_str::<Evidence>(&text) {
                    self.memory.insert(key, ev.clone());
                    return Some(ev);
                }
            }
        }
        None
    }

    pub fn put(&mut self, goal_hash: &str, evidence: &Evidence) {
        let key = (goal_hash.to_string(), evidence.format.clone());
        self.memory.insert(key, evidence.clone());
        if let Some(path) = self.disk_path(goal_hash, &evidence.format) {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string(evidence) {
                let _ = std::fs::write(path, json);
            }
        }
    }
}

// =========================================================================
// Seam 5: Admission Policy (Dev vs JPL Strict)
// =========================================================================

/// Policy governing which [`TrustTier`]s are admitted for a build or safety profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub max_allowed_tier: TrustTier,
}

impl Policy {
    /// Development policy: admits kernel proofs, certificates, and SMT verdicts.
    pub fn dev_default() -> Self {
        Self {
            name: "dev".to_string(),
            max_allowed_tier: TrustTier::SmtVerified,
        }
    }

    /// Strict safety-profile policy (`#[profile("jpl")]`): admits only kernel-checked
    /// proofs and kernel-checked certificates; rejects compiled-cert and trusted SMT.
    pub fn jpl_strict() -> Self {
        Self {
            name: "jpl".to_string(),
            max_allowed_tier: TrustTier::KernelCert,
        }
    }

    /// Lax iteration policy: admits all tiers (including compiled-cert, SMT verdicts,
    /// and explicitly downgraded `assumed` receipts).
    pub fn lax() -> Self {
        Self {
            name: "lax".to_string(),
            max_allowed_tier: TrustTier::Assumed,
        }
    }

    pub fn admits(&self, receipt: &TrustReceipt) -> Result<(), String> {
        if receipt.tier <= self.max_allowed_tier {
            Ok(())
        } else {
            Err(format!(
                "policy `{}` requires tier <= `{}`, but checker `{}` produced `{}` (TCB: {:?})",
                self.name, self.max_allowed_tier, receipt.checker_id, receipt.tier, receipt.tcb
            ))
        }
    }
}

// =========================================================================
// Verification Engine (Pluggable Registry + Portfolio + Checker Dispatch)
// =========================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified {
        receipt: TrustReceipt,
        from_cache: bool,
    },
    RejectedByPolicy {
        receipt: TrustReceipt,
        reason: String,
    },
    Disproved {
        prover_id: String,
        counterexample: Counterexample,
    },
    Unverified {
        reasons: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObligationReport {
    pub name: String,
    pub kind: String,
    pub goal_hash: String,
    pub status: VerificationStatus,
}

pub struct VerifierEngine {
    pub semantics: Vec<Box<dyn IrSemantics>>,
    pub provers: Vec<Box<dyn Prover>>,
    pub checkers: Vec<Box<dyn Checker>>,
    pub store: EvidenceStore,
    pub policy: Policy,
}

impl VerifierEngine {
    /// Default engine: `llvm-sem@21.1`, built-in CDCL+LRAT prover, both kernel-LRAT
    /// and SMT-attestation checkers, in-memory cache, `dev` policy.
    pub fn default_engine(policy: Policy) -> Self {
        let mut engine = Self {
            semantics: vec![Box::new(LlvmSemantics)],
            provers: vec![Box::new(CdclLratProver::default())],
            checkers: vec![
                Box::new(KernelLratChecker::default()),
                Box::new(SmtAttestationChecker),
            ],
            store: EvidenceStore::in_memory(),
            policy,
        };
        // Auto-register any installed external SMT solver into the portfolio as fallback
        // (for IEEE-754 floats and wide nonlinear bit-vectors).
        for (kind, bin) in [("bitwuzla", "bitwuzla"), ("z3", "z3"), ("cvc5", "cvc5")] {
            if command_exists(bin) {
                match kind {
                    "bitwuzla" => engine.provers.push(Box::new(ExternalSmtProver::bitwuzla(bin))),
                    "z3" => engine.provers.push(Box::new(ExternalSmtProver::z3(bin))),
                    "cvc5" => engine.provers.push(Box::new(ExternalSmtProver::cvc5(bin))),
                    _ => {}
                }
            }
        }
        engine
    }

    /// Engine configured with a compiled-cert checker (demonstrates replacing the checker
    /// plugin independently of the prover).
    pub fn with_compiled_lrat_checker(mut self) -> Self {
        self.checkers.retain(|c| c.format() != bitblast::LRAT_FORMAT_ID);
        self.checkers.insert(0, Box::new(CompiledLratChecker::default()));
        self
    }

    /// Fast, permissive engine for `PrimVerifyMode::Lax`: tighter SAT budget so heavy
    /// bit-vector proofs don't stall iteration, paired with `Policy::lax()`.
    pub fn lax_engine() -> Self {
        let mut engine = Self::default_engine(Policy::lax());
        engine.provers[0] = Box::new(CdclLratProver {
            max_clauses: 50_000,
            max_conflicts: 2_000,
        });
        engine
    }

    pub fn add_prover(&mut self, prover: Box<dyn Prover>) {
        self.provers.push(prover);
    }

    pub fn add_checker(&mut self, checker: Box<dyn Checker>) {
        self.checkers.push(checker);
    }

    pub fn semantics_refs(&self) -> Vec<&dyn IrSemantics> {
        self.semantics.iter().map(|s| s.as_ref()).collect()
    }

    /// Verify one `Obligation`:
    /// 1. Try cached `Evidence` for any registered `Checker` format (no solver needed!).
    /// 2. Otherwise run the ordered `Prover` portfolio.
    /// 3. Independently verify returned `Evidence` via a matching `Checker`.
    /// 4. Enforce `self.policy` on the resulting `TrustReceipt`.
    pub fn verify(&mut self, obligation: &Obligation) -> ObligationReport {
        // Step 1: Check content-addressed cache first (deterministic replay without invoking solvers).
        let checker_formats: Vec<String> =
            self.checkers.iter().map(|c| c.format().to_string()).collect();
        for fmt in checker_formats {
            if let Some(ev) = self.store.get(&obligation.goal_hash, &fmt) {
                if let Some(report) = self.check_and_admit(obligation, &ev, true) {
                    return report;
                }
            }
        }

        // Step 2: Run pluggable prover portfolio.
        let needed = obligation.theories();
        let mut reasons = Vec::new();
        for prover in &self.provers {
            if !prover.supported_theories().subsumes(needed) {
                reasons.push(format!(
                    "{}: skipped (does not support {:?})",
                    prover.id(),
                    needed
                ));
                continue;
            }
            match prover.prove(obligation) {
                ProverOutcome::Proved(ev) => {
                    match self.check_evidence(obligation, &ev) {
                        Ok(receipt) => {
                            self.store.put(&obligation.goal_hash, &ev);
                            let status = match self.policy.admits(&receipt) {
                                Ok(()) => VerificationStatus::Verified {
                                    receipt,
                                    from_cache: false,
                                },
                                Err(reason) => {
                                    VerificationStatus::RejectedByPolicy { receipt, reason }
                                }
                            };
                            return ObligationReport {
                                name: obligation.name.clone(),
                                kind: obligation.kind.to_string(),
                                goal_hash: obligation.goal_hash.clone(),
                                status,
                            };
                        }
                        Err(err) => {
                            // Untrusted prover produced invalid evidence — never trust it!
                            reasons.push(format!(
                                "{}: REJECTED bogus evidence ({err})",
                                prover.id()
                            ));
                        }
                    }
                }
                ProverOutcome::Disproved(cx) => {
                    return ObligationReport {
                        name: obligation.name.clone(),
                        kind: obligation.kind.to_string(),
                        goal_hash: obligation.goal_hash.clone(),
                        status: VerificationStatus::Disproved {
                            prover_id: prover.id().to_string(),
                            counterexample: cx,
                        },
                    };
                }
                ProverOutcome::Unknown(msg) => {
                    reasons.push(format!("{}: {msg}", prover.id()));
                }
            }
        }

        ObligationReport {
            name: obligation.name.clone(),
            kind: obligation.kind.to_string(),
            goal_hash: obligation.goal_hash.clone(),
            status: VerificationStatus::Unverified { reasons },
        }
    }

    fn check_evidence(
        &self,
        obligation: &Obligation,
        ev: &Evidence,
    ) -> Result<TrustReceipt, String> {
        let checker = self
            .checkers
            .iter()
            .find(|c| c.format() == ev.format)
            .ok_or_else(|| format!("no checker registered for format `{}`", ev.format))?;
        checker.check(obligation, ev).map_err(|e| e.to_string())
    }

    fn check_and_admit(
        &self,
        obligation: &Obligation,
        ev: &Evidence,
        from_cache: bool,
    ) -> Option<ObligationReport> {
        let receipt = self.check_evidence(obligation, ev).ok()?;
        let status = match self.policy.admits(&receipt) {
            Ok(()) => VerificationStatus::Verified { receipt, from_cache },
            Err(reason) => VerificationStatus::RejectedByPolicy { receipt, reason },
        };
        Some(ObligationReport {
            name: obligation.name.clone(),
            kind: obligation.kind.to_string(),
            goal_hash: obligation.goal_hash.clone(),
            status,
        })
    }
}

fn command_exists(bin: &str) -> bool {
    std::process::Command::new("which")
        .arg(bin)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// =========================================================================
// CLI Entrypoint: `cinch verify-prims <file.cinch ...>`
// =========================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuditSummary {
    pub policy: String,
    pub prim_bodies_seen: usize,
    pub skipped_unsafe: usize,
    pub skipped_unsupported: usize,
    pub obligations_total: usize,
    pub verified: usize,
    pub verified_from_cache: usize,
    pub disproved: usize,
    pub rejected_by_policy: usize,
    pub unverified: usize,
    pub findings: Vec<ObligationReport>,
}

pub fn cli(args: &[String]) -> i32 {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut policy_name = "dev".to_string();
    let mut checker_mode = "kernel".to_string();
    let mut smt_bin: Option<String> = None;
    let mut cadical_bin: Option<String> = None;
    let mut cache_dir: Option<PathBuf> = None;
    let mut emit_smt_dir: Option<PathBuf> = None;
    let mut json_out = false;
    let mut verbose = false;

    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if let Some(p) = a.strip_prefix("--policy=") {
            policy_name = p.to_string();
        } else if let Some(c) = a.strip_prefix("--checker=") {
            checker_mode = c.to_string();
        } else if let Some(s) = a.strip_prefix("--smt=") {
            smt_bin = Some(s.to_string());
        } else if let Some(s) = a.strip_prefix("--cadical=") {
            cadical_bin = Some(s.to_string());
        } else if let Some(d) = a.strip_prefix("--cache-dir=") {
            cache_dir = Some(PathBuf::from(d));
        } else if let Some(d) = a.strip_prefix("--emit-smt=") {
            emit_smt_dir = Some(PathBuf::from(d));
        } else if a == "--json" {
            json_out = true;
        } else if a == "--verbose" || a == "-v" {
            verbose = true;
        } else if a == "--help" || a == "-h" {
            eprintln!(
                "usage: cinch verify-prims <file.cinch...> [options]\n\
                 options:\n\
                 \x20 --policy=dev|jpl|lax      admission policy (default: dev)\n\
                 \x20 --checker=kernel|compiled LRAT checker TCB tier (default: kernel)\n\
                 \x20 --smt=<binary>            plug in external SMT-LIB 2 solver (z3/bitwuzla/cvc5)\n\
                 \x20 --cadical=<binary>        plug in external CaDiCaL LRAT solver\n\
                 \x20 --cache-dir=<dir>         content-addressed evidence cache directory\n\
                 \x20 --emit-smt=<dir>          write generated .smt2 files for unverified/all goals\n\
                 \x20 --json                    emit machine-readable JSON report\n\
                 \x20 --verbose                 print every verified obligation receipt"
            );
            return 0;
        } else if a.starts_with('-') {
            eprintln!("cinch verify-prims: unknown flag `{a}`");
            return 2;
        } else {
            files.push(PathBuf::from(a));
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("cinch verify-prims: expected at least one `.cinch` file");
        return 2;
    }

    let policy = match policy_name.as_str() {
        "dev" => Policy::dev_default(),
        "jpl" => Policy::jpl_strict(),
        "lax" => Policy::lax(),
        other => {
            eprintln!("cinch verify-prims: unknown policy `{other}` (expected `dev`, `jpl`, or `lax`)");
            return 2;
        }
    };

    let mut engine = if policy.name == "lax" {
        VerifierEngine::lax_engine()
    } else {
        VerifierEngine::default_engine(policy)
    };
    if checker_mode == "compiled" {
        engine = engine.with_compiled_lrat_checker();
    }
    if let Some(dir) = cache_dir {
        engine.store = EvidenceStore::with_disk_dir(dir);
    }
    if let Some(bin) = cadical_bin {
        engine.provers.insert(0, Box::new(ExternalSatProver::cadical(bin)));
    }
    if let Some(bin) = smt_bin {
        let solver = if bin.contains("bitwuzla") {
            ExternalSmtProver::bitwuzla(bin)
        } else if bin.contains("cvc5") {
            ExternalSmtProver::cvc5(bin)
        } else {
            ExternalSmtProver::z3(bin)
        };
        engine.add_prover(Box::new(solver));
    }

    if let Some(dir) = &emit_smt_dir {
        let _ = std::fs::create_dir_all(dir);
    }

    let mut summary = AuditSummary {
        policy: engine.policy.name.clone(),
        ..Default::default()
    };

    for file in &files {
        let extracted: Vec<ExtractedPrimBody> = {
            let sems = engine.semantics_refs();
            match extract_file_obligations(file, &sems) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("cinch verify-prims: {}: {err}", file.display());
                    return 1;
                }
            }
        };

        for body in extracted {
            summary.prim_bodies_seen += 1;
            if body.is_unsafe {
                summary.skipped_unsafe += 1;
                continue;
            }
            if let Some(reason) = &body.skip_reason {
                summary.skipped_unsupported += 1;
                if verbose && !json_out {
                    println!(
                        "  [skip] {}.{} ({}) — {reason}",
                        body.module, body.prim_name, body.when_desc
                    );
                }
                continue;
            }

            for ob in &body.obligations {
                summary.obligations_total += 1;
                if let Some(dir) = &emit_smt_dir {
                    let fname = format!("{}.smt2", ob.name.replace(['.', ':', '#'], "_"));
                    let _ = std::fs::write(dir.join(fname), emit_smtlib2(ob));
                }
                let report = engine.verify(ob);
                match &report.status {
                    VerificationStatus::Verified { receipt, from_cache } => {
                        summary.verified += 1;
                        if *from_cache {
                            summary.verified_from_cache += 1;
                        }
                        if verbose && !json_out {
                            println!(
                                "  [PASS] {} ({}, {}, cache={}): {}",
                                report.name,
                                receipt.tier,
                                receipt.checker_id,
                                from_cache,
                                receipt.details
                            );
                        }
                    }
                    VerificationStatus::Disproved { prover_id, counterexample } => {
                        summary.disproved += 1;
                        if !json_out {
                            println!(
                                "  [FAIL] {} disproved by {}: {}",
                                report.name, prover_id, counterexample.summary
                            );
                            for (k, v) in &counterexample.inputs {
                                println!("         {k} = {v}");
                            }
                        }
                        summary.findings.push(report);
                    }
                    VerificationStatus::RejectedByPolicy { reason, .. } => {
                        summary.rejected_by_policy += 1;
                        if !json_out {
                            println!("  [POLICY] {}: {reason}", report.name);
                        }
                        summary.findings.push(report);
                    }
                    VerificationStatus::Unverified { reasons } => {
                        summary.unverified += 1;
                        if verbose && !json_out {
                            println!(
                                "  [OPEN] {} — {}",
                                report.name,
                                reasons.join("; ")
                            );
                        }
                    }
                }
            }
        }
    }

    if json_out {
        println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    } else {
        println!(
            "\nprim-verify summary (policy={}): bodies={} (unsafe={}, unsupported={}) | \
             obligations={} | verified={} (cached={}) | disproved={} | policy-rejected={} | open={}",
            summary.policy,
            summary.prim_bodies_seen,
            summary.skipped_unsafe,
            summary.skipped_unsupported,
            summary.obligations_total,
            summary.verified,
            summary.verified_from_cache,
            summary.disproved,
            summary.rejected_by_policy,
            summary.unverified
        );
    }

    if summary.disproved > 0 || summary.rejected_by_policy > 0 {
        1
    } else {
        0
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prim_verify::prelude_bridge::extract_source_obligations;
    use std::path::PathBuf;

    fn verify_snippet(src: &str, policy: Policy) -> Vec<ObligationReport> {
        let mut engine = VerifierEngine::default_engine(policy);
        let bodies = {
            let sems = engine.semantics_refs();
            extract_source_obligations(src, &sems).expect("snippet should parse")
        };
        let mut reports = Vec::new();
        for b in bodies {
            for ob in b.obligations {
                reports.push(engine.verify(&ob));
            }
        }
        reports
    }

    #[test]
    fn proves_poison_freedom_of_masked_shift_and_simd_splat_with_lrat() {
        let src = r#"
module Test where
  prim wrapping-shl-i8 :: I8 -> U32 -> I8 where
    llvm {
      %m = and i32 %1, 7
      %s = trunc i32 %m to i8
      %r = shl i8 %0, %s
      ret i8 %r
    }
  prim simd-u8x16-splat :: U8 -> U128 where
    llvm {
      %v0 = insertelement <16 x i8> poison, i8 %0, i64 0
      %v = shufflevector <16 x i8> %v0, <16 x i8> poison, <16 x i32> zeroinitializer
      %r = bitcast <16 x i8> %v to i128
      ret i128 %r
    }
"#;
        let reports = verify_snippet(src, Policy::jpl_strict());
        assert_eq!(reports.len(), 4); // 2 well-defined + 2 functional-equiv
        for r in &reports {
            match &r.status {
                VerificationStatus::Verified { receipt, .. } => {
                    assert_eq!(receipt.tier, TrustTier::KernelCert);
                    assert_eq!(receipt.evidence_format, "lrat/v1");
                }
                other => panic!("expected KernelCert verification for {}, got {other:?}", r.name),
            }
        }
    }

    #[test]
    fn disproves_unmasked_shift_poison_and_sdiv_ub_with_validated_counterexamples() {
        let src = r#"
module Bad where
  prim unmasked-shl-i8 :: I8 -> I8 -> I8 where
    llvm {
      %r = shl i8 %0, %1
      ret i8 %r
    }
  prim raw-sdiv-i8 :: I8 -> I8 -> I8 where
    llvm {
      %r = sdiv i8 %0, %1
      ret i8 %r
    }
"#;
        let reports = verify_snippet(src, Policy::dev_default());
        assert_eq!(reports.len(), 2);
        for r in &reports {
            match &r.status {
                VerificationStatus::Disproved { counterexample, .. } => {
                    assert!(
                        counterexample.validated,
                        "counterexample for {} must be independently confirmed",
                        r.name
                    );
                }
                other => panic!("expected Disproved for {}, got {other:?}", r.name),
            }
        }
    }

    #[test]
    fn catches_u64_to_usize_32bit_refinement_violation_and_proves_i8_to_isize() {
        let src = r#"
module Narrow where
  prim u64-to-usize :: (x :: U64) -> refined (r == x) Usize where
    llvm {
      %r = trunc i64 %0 to i32
      ret i32 %r
    } when word-size 32
  prim i8-to-isize :: (x :: I8) -> refined (InRange (-128) 127 r && r == x) Isize where
    llvm {
      %r = sext i8 %0 to i32
      ret i32 %r
    } when word-size 32
"#;
        let reports = verify_snippet(src, Policy::jpl_strict());
        let u64_ref = reports
            .iter()
            .find(|r| r.name.contains("u64-to-usize") && r.name.ends_with("::refinement"))
            .expect("u64-to-usize refinement obligation");
        match &u64_ref.status {
            VerificationStatus::Disproved { counterexample, .. } => {
                assert!(counterexample.validated);
            }
            other => panic!("expected u64-to-usize 32-bit refinement to be disproved, got {other:?}"),
        }

        let i8_ref = reports
            .iter()
            .find(|r| r.name.contains("i8-to-isize") && r.name.ends_with("::refinement"))
            .expect("i8-to-isize refinement obligation");
        assert!(matches!(i8_ref.status, VerificationStatus::Verified { .. }));
    }

    #[test]
    fn tampered_lrat_certificate_is_rejected_by_independent_checker() {
        let src = r#"
module Tamper where
  prim wrapping-shl-i8 :: I8 -> U32 -> I8 where
    llvm {
      %m = and i32 %1, 7
      %s = trunc i32 %m to i8
      %r = shl i8 %0, %s
      ret i8 %r
    }
"#;
        let sems: Vec<&dyn IrSemantics> = vec![&LlvmSemantics];
        let bodies = extract_source_obligations(src, &sems).unwrap();
        let ob = &bodies[0].obligations[0];

        // A malicious prover fabricates an invalid empty-clause derivation with bogus hint.
        let bogus = Evidence {
            format: bitblast::LRAT_FORMAT_ID.to_string(),
            prover_id: "evil-prover".to_string(),
            payload: "999999 0 1 0\n".to_string(),
        };
        let checker = KernelLratChecker::default();
        assert!(checker.check(ob, &bogus).is_err());
    }

    #[test]
    fn pluggable_smt_prover_and_policy_differentiation_dev_vs_jpl() {
        // Demonstrates plugging an SMT prover that produces `smt-attestation/v1`:
        // `dev` policy admits it (`TrustTier::SmtVerified`), while `jpl` policy rejects it!
        struct FakeSmtProver;
        impl Prover for FakeSmtProver {
            fn id(&self) -> &str {
                "fake-bitwuzla@test"
            }
            fn supported_theories(&self) -> TheorySet {
                TheorySet::all()
            }
            fn prove(&self, obligation: &Obligation) -> ProverOutcome {
                // Ensure SMT-LIB 2 emitter succeeds on IEEE-754 float obligation.
                let script = emit_smtlib2(obligation);
                assert!(script.contains("(set-logic QF_FPBV)"));
                assert!(script.contains("fp.roundToIntegral"));
                let att = smt::SmtAttestation {
                    solver_id: self.id().to_string(),
                    solver_binary: "fake-bitwuzla".to_string(),
                    emitter_id: smt::SMT_EMITTER_VERSION_ID.to_string(),
                    goal_hash: obligation.goal_hash.clone(),
                    semantics_id: obligation.semantics_id.clone(),
                    verdict: "unsat".to_string(),
                };
                ProverOutcome::Proved(Evidence {
                    format: smt::SMT_ATTESTATION_FORMAT_ID.to_string(),
                    prover_id: self.id().to_string(),
                    payload: serde_json::to_string(&att).unwrap(),
                })
            }
        }

        let src = r#"
module FloatTest where
  prim saturating-f64-to-i8 :: F64 -> I8 where
    llvm {
      %lo = fcmp ogt double %0, 0xC060200000000000
      %hi = fcmp olt double %0, 0x4060000000000000
      %ok = and i1 %lo, %hi
      %safe = select i1 %ok, double %0, double 0.0
      %v = fptosi double %safe to i8
      %above = fcmp oge double %0, 0x4060000000000000
      %below = fcmp ole double %0, 0xC060200000000000
      %a = select i1 %above, i8 127, i8 %v
      %r = select i1 %below, i8 -128, i8 %a
      ret i8 %r
    }
"#;
        let sems: Vec<&dyn IrSemantics> = vec![&LlvmSemantics];
        let bodies = extract_source_obligations(src, &sems).unwrap();
        let ob = &bodies[0].obligations[0];

        // Dev policy admits `SmtVerified`.
        let mut dev_engine = VerifierEngine::default_engine(Policy::dev_default());
        dev_engine.provers = vec![Box::new(FakeSmtProver)];
        let r_dev = dev_engine.verify(ob);
        assert!(matches!(
            r_dev.status,
            VerificationStatus::Verified {
                receipt: TrustReceipt { tier: TrustTier::SmtVerified, .. },
                ..
            }
        ));

        // JPL strict policy rejects `SmtVerified`!
        let mut jpl_engine = VerifierEngine::default_engine(Policy::jpl_strict());
        jpl_engine.provers = vec![Box::new(FakeSmtProver)];
        let r_jpl = jpl_engine.verify(ob);
        assert!(matches!(r_jpl.status, VerificationStatus::RejectedByPolicy { .. }));
    }

    #[test]
    fn evidence_cache_replays_verified_lrat_without_any_prover_registered() {
        let src = r#"
module CacheTest where
  prim simd-bitselect :: U128 -> U128 -> U128 -> U128 where
    llvm {
      %notm = xor i128 %0, -1
      %a = and i128 %0, %1
      %c = and i128 %notm, %2
      %r = or i128 %a, %c
      ret i128 %r
    }
"#;
        let dir = tempfile::tempdir().unwrap();
        let sems: Vec<&dyn IrSemantics> = vec![&LlvmSemantics];
        let bodies = extract_source_obligations(src, &sems).unwrap();
        let equiv_ob = bodies[0]
            .obligations
            .iter()
            .find(|o| o.name.ends_with("::equiv"))
            .unwrap();

        // Phase 1: Prove and populate disk cache.
        let mut producer = VerifierEngine::default_engine(Policy::jpl_strict());
        producer.store = EvidenceStore::with_disk_dir(dir.path().to_path_buf());
        let r1 = producer.verify(equiv_ob);
        assert!(matches!(
            r1.status,
            VerificationStatus::Verified { from_cache: false, .. }
        ));

        // Phase 2: Downstream build with ZERO provers installed replays from cache!
        let mut consumer = VerifierEngine::default_engine(Policy::jpl_strict());
        consumer.provers.clear();
        consumer.store = EvidenceStore::with_disk_dir(dir.path().to_path_buf());
        let r2 = consumer.verify(equiv_ob);
        assert!(matches!(
            r2.status,
            VerificationStatus::Verified { from_cache: true, .. }
        ));
    }

    #[test]
    fn source_spec_clause_proves_equivalent_llvm_body_and_rejects_wrong_body() {
        let good = r#"
module SpecTest where
  spec twice-spec :: U32 -> U32
  twice-spec x = wrapping-add-u32 x x

  prim twice :: U32 -> U32 spec twice-spec where
    llvm {
      %r = shl i32 %0, 1
      ret i32 %r
    }
"#;
        let reports = verify_snippet(good, Policy::jpl_strict());
        let spec_rep = reports
            .iter()
            .find(|r| r.name.contains("::spec("))
            .expect("source spec obligation");
        assert!(
            matches!(
                &spec_rep.status,
                VerificationStatus::Verified {
                    receipt: TrustReceipt { tier: TrustTier::KernelCert, .. },
                    ..
                }
            ),
            "expected shl 1 == wrapping-add-u32 x x to verify, got {spec_rep:?}"
        );

        let bad = good.replace("shl i32 %0, 1", "add i32 %0, 1");
        let bad_reports = verify_snippet(&bad, Policy::jpl_strict());
        let bad_spec = bad_reports
            .iter()
            .find(|r| r.name.contains("::spec("))
            .expect("source spec obligation");
        match &bad_spec.status {
            VerificationStatus::Disproved { counterexample, .. } => {
                assert!(counterexample.validated);
            }
            other => panic!("expected broken body to be disproved, got {other:?}"),
        }
    }

    #[test]
    fn audits_real_prelude_with_zero_disproved_obligations() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let files = [
            manifest_dir.join("prelude/Prim/Machine.cinch"),
            manifest_dir.join("prelude/Prim/Wide.cinch"),
            manifest_dir.join("prelude/Data/Machine.cinch"),
            manifest_dir.join("prelude/Data/Wide.cinch"),
        ];

        let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
        let mut verified = 0usize;
        let mut disproved_names = Vec::new();

        for path in &files {
            let bodies = {
                let sems = engine.semantics_refs();
                extract_file_obligations(path, &sems).expect("prelude file should parse")
            };
            for b in bodies {
                for ob in b.obligations {
                    match engine.verify(&ob).status {
                        VerificationStatus::Verified { .. } => verified += 1,
                        VerificationStatus::Disproved { .. } => disproved_names.push(ob.name),
                        _ => {}
                    }
                }
            }
        }

        // Hundreds of real prelude LLVM bodies are mathematically proved with LRAT,
        // and after fixing the 32-bit refinement preconditions and marking raw unchecked
        // division prims `unsafe`, zero obligations are disproved!
        assert!(
            verified >= 500,
            "expected >= 500 kernel-LRAT verified obligations across prelude prims, got {verified}"
        );
        assert!(
            disproved_names.is_empty(),
            "expected zero disproved prelude obligations, got {disproved_names:?}"
        );
    }
}
