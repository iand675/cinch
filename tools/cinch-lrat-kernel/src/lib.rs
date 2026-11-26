//! Minimal, zero-dependency, `#![forbid(unsafe_code)]` LRAT RUP proof kernel.
//!
//! Trusted Computing Base (TCB) boundary:
//! Given a DIMACS CNF formula `F` (1-indexed clauses) and an untrusted `lrat/v1`
//! proof certificate `P`, [`verify_lrat`] returns `Ok(steps)` ONLY if `P` is a
//! sound Reverse Unit Propagation (RUP) derivation of the empty clause `⊥` from `F`.
//!
//! See `design/lrat-kernel-soundness.md` for the formal invariant proof.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

pub const KERNEL_ID: &str = "cinch-lrat-kernel-standalone@1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LratError {
    ParseError(String),
    DuplicateClauseId(u64),
    UnknownHintClause { step: u64, hint: u64 },
    HintNotUnitOrConflict { step: u64, hint: u64, unassigned: usize },
    StepDidNotReachConflict { step: u64 },
    MissingEmptyClause,
    LiteralOutOfRange { lit: i32, num_vars: usize },
}

impl fmt::Display for LratError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LratError::ParseError(s) => write!(f, "LRAT parse error: {s}"),
            LratError::DuplicateClauseId(id) => write!(f, "duplicate clause id {id}"),
            LratError::UnknownHintClause { step, hint } => {
                write!(f, "step {step}: hint clause {hint} does not exist (or was deleted)")
            }
            LratError::HintNotUnitOrConflict { step, hint, unassigned } => {
                write!(f, "step {step}: hint clause {hint} is not unit/conflict ({unassigned} unassigned)")
            }
            LratError::StepDidNotReachConflict { step } => {
                write!(f, "step {step}: RUP hints did not derive a conflict")
            }
            LratError::MissingEmptyClause => {
                write!(f, "LRAT proof terminated without deriving the empty clause")
            }
            LratError::LiteralOutOfRange { lit, num_vars } => {
                write!(f, "literal {lit} exceeds declared variable count {num_vars}")
            }
        }
    }
}

impl std::error::Error for LratError {}

/// Parse a standard DIMACS CNF string into `(num_vars, clauses)`.
pub fn parse_dimacs(cnf: &str) -> Result<(usize, Vec<Vec<i32>>), LratError> {
    let mut num_vars: Option<usize> = None;
    let mut clauses: Vec<Vec<i32>> = Vec::new();
    let mut cur: Vec<i32> = Vec::new();

    for line in cnf.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('c') {
            continue;
        }
        if let Some(rest) = t.strip_prefix("p cnf ") {
            let mut it = rest.split_whitespace();
            let nv: usize = it
                .next()
                .ok_or_else(|| LratError::ParseError("missing variable count".into()))?
                .parse()
                .map_err(|_| LratError::ParseError("invalid variable count".into()))?;
            num_vars = Some(nv);
            continue;
        }
        for tok in t.split_whitespace() {
            let lit: i32 = tok
                .parse()
                .map_err(|_| LratError::ParseError(format!("invalid CNF literal `{tok}`")))?;
            if lit == 0 {
                clauses.push(std::mem::take(&mut cur));
            } else {
                cur.push(lit);
            }
        }
    }
    if !cur.is_empty() {
        return Err(LratError::ParseError("unterminated CNF clause (missing 0)".into()));
    }
    let nv = num_vars.ok_or_else(|| LratError::ParseError("missing `p cnf` header".into()))?;
    for c in &clauses {
        for &l in c {
            let v = l.unsigned_abs() as usize;
            if v == 0 || v > nv {
                return Err(LratError::LiteralOutOfRange { lit: l, num_vars: nv });
            }
        }
    }
    Ok((nv, clauses))
}

/// Independently verify an `lrat/v1` proof text against `(num_vars, original_clauses)`.
///
/// Soundness invariant (maintained at every step):
/// Every clause stored in `db` is a logical consequence of `original_clauses`.
/// Returning `Ok(steps)` guarantees the empty clause `[]` was added to `db`, hence
/// `original_clauses` is unsatisfiable.
pub fn verify_lrat(
    num_vars: usize,
    original_clauses: &[Vec<i32>],
    proof: &str,
) -> Result<usize, LratError> {
    let mut db: HashMap<u64, Vec<i32>> = HashMap::with_capacity(original_clauses.len() * 2);
    let mut max_var = num_vars;
    for (idx, cl) in original_clauses.iter().enumerate() {
        for &l in cl {
            let v = l.unsigned_abs() as usize;
            if v == 0 || v > num_vars {
                return Err(LratError::LiteralOutOfRange { lit: l, num_vars });
            }
        }
        db.insert((idx + 1) as u64, cl.clone());
    }

    // Valuation scratch buffer: 0 = unassigned, 1 = true, -1 = false.
    let mut val: Vec<i8> = vec![0; max_var + 1];
    let mut trail: Vec<usize> = Vec::new();
    let mut derived_empty = false;
    let mut steps = 0usize;

    for (line_idx, raw) in proof.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 2 {
            return Err(LratError::ParseError(format!("line {}: too short", line_idx + 1)));
        }

        // Deletion line: `<id> d <del1> <del2> ... 0`
        if toks[1] == "d" {
            let mut saw_zero = false;
            for &tok in &toks[2..] {
                let id: u64 = tok.parse().map_err(|_| {
                    LratError::ParseError(format!("line {}: invalid deletion id `{tok}`", line_idx + 1))
                })?;
                if id == 0 {
                    saw_zero = true;
                    break;
                }
                db.remove(&id);
            }
            if !saw_zero {
                return Err(LratError::ParseError(format!(
                    "line {}: deletion line missing terminating 0",
                    line_idx + 1
                )));
            }
            continue;
        }

        // Addition line: `<new_id> <l1> ... <lk> 0 <h1> ... <hm> 0`
        let new_id: u64 = toks[0].parse().map_err(|_| {
            LratError::ParseError(format!("line {}: invalid clause id `{}`", line_idx + 1, toks[0]))
        })?;
        if db.contains_key(&new_id) {
            return Err(LratError::DuplicateClauseId(new_id));
        }

        let mut i = 1usize;
        let mut clause: Vec<i32> = Vec::new();
        let mut found_clause_end = false;
        while i < toks.len() {
            let lit: i32 = toks[i].parse().map_err(|_| {
                LratError::ParseError(format!("line {}: invalid literal `{}`", line_idx + 1, toks[i]))
            })?;
            i += 1;
            if lit == 0 {
                found_clause_end = true;
                break;
            }
            let v = lit.unsigned_abs() as usize;
            if v > max_var {
                max_var = v;
                val.resize(max_var + 1, 0);
            }
            clause.push(lit);
        }
        if !found_clause_end {
            return Err(LratError::ParseError(format!(
                "line {}: missing 0 terminating clause literals",
                line_idx + 1
            )));
        }

        let mut hints: Vec<i64> = Vec::new();
        let mut found_hints_end = false;
        while i < toks.len() {
            let h: i64 = toks[i].parse().map_err(|_| {
                LratError::ParseError(format!("line {}: invalid hint `{}`", line_idx + 1, toks[i]))
            })?;
            i += 1;
            if h == 0 {
                found_hints_end = true;
                break;
            }
            if h < 0 {
                return Err(LratError::ParseError(format!(
                    "line {}: RAT negative hint {h} is outside pure RUP kernel",
                    line_idx + 1
                )));
            }
            hints.push(h);
        }
        if !found_hints_end {
            return Err(LratError::ParseError(format!(
                "line {}: missing 0 terminating hints",
                line_idx + 1
            )));
        }

        // Check RUP: assume negation of every literal in `clause`.
        // If `clause` contains complementary literals `x` and `-x`, it is a tautology (trivially RUP).
        let mut conflict = false;
        for &lit in &clause {
            let v = lit.unsigned_abs() as usize;
            let neg_sign: i8 = if lit > 0 { -1 } else { 1 };
            if val[v] == 0 {
                val[v] = neg_sign;
                trail.push(v);
            } else if val[v] != neg_sign {
                conflict = true;
                break;
            }
        }

        if !conflict {
            for &h_raw in &hints {
                let h = h_raw as u64;
                let Some(hc) = db.get(&h) else {
                    clear_trail(&mut val, &mut trail);
                    return Err(LratError::UnknownHintClause { step: new_id, hint: h });
                };
                let mut satisfied = false;
                let mut unassigned_lit: i32 = 0;
                let mut unassigned_count: usize = 0;
                for &lit in hc {
                    let v = lit.unsigned_abs() as usize;
                    let s = if v < val.len() { val[v] } else { 0 };
                    let want: i8 = if lit > 0 { 1 } else { -1 };
                    if s == want {
                        satisfied = true;
                        break;
                    } else if s == 0 {
                        unassigned_lit = lit;
                        unassigned_count += 1;
                    }
                }
                if satisfied {
                    continue;
                }
                if unassigned_count == 0 {
                    conflict = true;
                    break;
                } else if unassigned_count == 1 {
                    let v = unassigned_lit.unsigned_abs() as usize;
                    if v >= val.len() {
                        val.resize(v + 1, 0);
                    }
                    val[v] = if unassigned_lit > 0 { 1 } else { -1 };
                    trail.push(v);
                } else {
                    clear_trail(&mut val, &mut trail);
                    return Err(LratError::HintNotUnitOrConflict {
                        step: new_id,
                        hint: h,
                        unassigned: unassigned_count,
                    });
                }
            }
        }

        clear_trail(&mut val, &mut trail);

        if !conflict {
            return Err(LratError::StepDidNotReachConflict { step: new_id });
        }

        steps += 1;
        if clause.is_empty() {
            derived_empty = true;
            db.insert(new_id, clause);
            break;
        }
        db.insert(new_id, clause);
    }

    if !derived_empty {
        return Err(LratError::MissingEmptyClause);
    }
    Ok(steps)
}

#[inline]
fn clear_trail(val: &mut [i8], trail: &mut Vec<usize>) {
    for v in trail.drain(..) {
        val[v] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_valid_pigeonhole_rup_proof() {
        // PHP(2,1): 2 pigeons into 1 hole: (1), (2), (-1 -2)
        let cnf = "p cnf 2 3\n1 0\n2 0\n-1 -2 0\n";
        let (nv, clauses) = parse_dimacs(cnf).unwrap();
        let proof = "4 0 1 2 3 0\n";
        assert_eq!(verify_lrat(nv, &clauses, proof).unwrap(), 1);
    }

    #[test]
    fn verifies_multi_step_rup_with_deletion() {
        // (1 2), (1 -2), (-1 2), (-1 -2)
        let cnf = "p cnf 2 4\n1 2 0\n1 -2 0\n-1 2 0\n-1 -2 0\n";
        let (nv, clauses) = parse_dimacs(cnf).unwrap();
        let proof = "\
5 1 0 1 2 0
5 d 1 2 0
6 -1 0 3 4 0
7 0 5 6 0
";
        assert_eq!(verify_lrat(nv, &clauses, proof).unwrap(), 3);
    }

    #[test]
    fn rejects_deleted_clause_reuse_and_bogus_hints() {
        let cnf = "p cnf 2 4\n1 2 0\n1 -2 0\n-1 2 0\n-1 -2 0\n";
        let (nv, clauses) = parse_dimacs(cnf).unwrap();
        // Using clause 1 after deleting it MUST fail!
        let bad_del = "5 d 1 0\n6 1 0 1 2 0\n";
        assert!(matches!(
            verify_lrat(nv, &clauses, bad_del),
            Err(LratError::UnknownHintClause { step: 6, hint: 1 })
        ));
        // Satisfiable formula with forged empty clause MUST fail!
        let sat = "p cnf 2 2\n1 2 0\n-1 2 0\n";
        let (nv2, cl2) = parse_dimacs(sat).unwrap();
        assert!(verify_lrat(nv2, &cl2, "3 0 1 2 0\n").is_err());
    }

    #[test]
    fn soundness_fuzz_never_accepts_proof_for_satisfiable_random_3sat() {
        // Simple xorshift64 PRNG (zero dependencies!).
        let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        // Generate 50 guaranteed-satisfiable formulas (satisfying assignment = all-true)
        // and attempt 20 mutated proof certificates against each.
        for _ in 0..50 {
            let nv = 6usize;
            let mut clauses = Vec::new();
            for _ in 0..18 {
                let mut cl = Vec::new();
                // Guarantee at least one positive literal so all-true satisfies every clause!
                let pos = (next() % nv as u64) as i32 + 1;
                cl.push(pos);
                for _ in 0..2 {
                    let v = (next() % nv as u64) as i32 + 1;
                    let lit = if next() & 1 == 0 { v } else { -v };
                    if !cl.contains(&lit) {
                        cl.push(lit);
                    }
                }
                clauses.push(cl);
            }
            for _ in 0..20 {
                let h1 = (next() % 18) + 1;
                let h2 = (next() % 18) + 1;
                let h3 = (next() % 18) + 1;
                let forged = format!("19 0 {h1} {h2} {h3} 0\n");
                assert!(
                    verify_lrat(nv, &clauses, &forged).is_err(),
                    "SOUNDNESS BUG: kernel accepted forged proof `{forged}` for SAT formula!"
                );
            }
        }
    }
}
