# Formal Soundness Invariant of `cinch-lrat-kernel`

## Scope and Trusted Computing Base

`tools/cinch-lrat-kernel` is a zero-dependency, `#![forbid(unsafe_code)]` Rust
implementation of a Reverse Unit Propagation (RUP) LRAT proof checker (~200 LOC).
It shares **zero** code with any SAT/SMT solver (`CdclLratProver`, CaDiCaL, Z3,
Bitwuzla, cvc5) and is the minimal TCB artifact for Cinch's `TrustTier::KernelCert`
(`cinch-kernel-lrat@1`).

## Theorem (Soundness of `verify_lrat`)

Let $F = \{C_1, \dots, C_m\}$ be a finite set of propositional clauses over
variables $V = \{1, \dots, n\}$, and let $P$ be an untrusted `lrat/v1` proof text.

**Theorem.** If `verify_lrat(n, F, P)` returns `Ok(steps)`, then $F$ is
propositionally unsatisfiable (no Boolean assignment $\sigma : V \to \{\bot, \top\}$
satisfies every clause $C_i \in F$).

### Proof Sketch (Loop Invariant)

Let `db` denote the active clause map `HashMap<u64, Vec<i32>>` maintained by
`verify_lrat`. We maintain the invariant:

$$
\mathcal{I}(\mathrm{db}) \;\triangleq\; \forall (k, C) \in \mathrm{db},\quad F \models C
$$

1. **Initialization:** `db` is populated with $\{(i, C_i) \mid 1 \le i \le m\}$.
   Every $C_i \in F$ trivially satisfies $F \models C_i$.
2. **Deletion step (`<id> d <d1> ... 0`):** Removing entries from `db` preserves
   $\mathcal{I}(\mathrm{db})$ monotonically. Deleted clauses cannot be referenced
   by subsequent hints (`UnknownHintClause`).
3. **Addition step (`<new_id> C 0 h_1 ... h_r 0`):**
   - Suppose toward contradiction that there exists a model $\sigma \models F$
     such that $\sigma \not\models C$.
   - Since $\sigma \not\models C$, every literal $\ell \in C$ evaluates to $\bot$
     under $\sigma$, i.e. $\sigma(\neg \ell) = \top$.
   - **Tautology check:** If $C$ contains complementary literals $x$ and $\neg x$,
     no assignment falsifies $C$, so $\sigma \models C$ trivially; `conflict = true`
     is sound.
   - **Unit propagation along hints $h_1, \dots, h_r$:**
     The checker initializes partial assignment `val` with $\{\neg \ell \mapsto \top \mid \ell \in C\}$,
     which agrees with $\sigma$ on every assigned variable ($\text{val} \subseteq \sigma$).
     For each hint $h_j$:
     - The checker looks up $H = \mathrm{db}[h_j]$. By $\mathcal{I}(\mathrm{db})$,
       $F \models H$, so $\sigma \models H$.
     - If $H$ has a literal already true in `val`, it is skipped.
     - If $H$ has **zero** unassigned literals and none true (`unassigned_count == 0`),
       then $\text{val}$ falsifies every literal of $H$, contradicting $\sigma \models H$
       since $\text{val} \subseteq \sigma$. Thus no such $\sigma$ can exist, proving $F \models C$.
     - If $H$ has **exactly one** unassigned literal $u$ (`unassigned_count == 1`),
       every other literal in $H$ is falsified by $\text{val} \subseteq \sigma$; since
       $\sigma \models H$, we must have $\sigma(u) = \top$. Extending `val` with
       $u \mapsto \top$ therefore preserves $\text{val} \subseteq \sigma$.
     - Otherwise (`unassigned_count >= 2`), the checker immediately rejects the proof
       (`HintNotUnitOrConflict`).
   - If the hint loop terminates without `conflict == true`, the checker rejects
     the proof (`StepDidNotReachConflict`).
   - Negative (RAT) hints are rejected outright; only pure RUP steps are accepted.
   - Therefore, whenever `db.insert(new_id, C)` executes, $F \models C$, preserving
     $\mathcal{I}(\mathrm{db})$.
4. **Termination:** `verify_lrat` returns `Ok(_)` only when `derived_empty == true`,
   which occurs only after the empty clause $C = \emptyset$ has been verified and
   inserted into `db`. By $\mathcal{I}(\mathrm{db})$, $F \models \emptyset$, so $F$
   has no model and is unsatisfiable. $\blacksquare$

## Verification & Audit Hooks

- `#![forbid(unsafe_code)]` enforced at compile time.
- Zero external crate dependencies (`Cargo.toml` has empty `[dependencies]`).
- Property-based soundness suite (`cargo test --manifest-path tools/cinch-lrat-kernel/Cargo.toml`)
  verifies valid multi-step RUP derivations with deletions, rejects deleted-clause reuse,
  and fuzzes 1,000 forged proofs against satisfiable random 3-SAT instances.
