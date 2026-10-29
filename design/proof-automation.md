# Proof automation: decision procedures and tactics

Status: design direction for review, 2026-10-29. The user's direction: "the
bitvector example is just one, but we'd want other lean proof strategies as
well for this stuff, i'm sure there are proofs for other types of thing".
This note catalogues the proof strategies Cinch needs, by domain, says which
Cinch obligations each one discharges, and fixes the architectural rule they
share. Companions: `design/verified-primitives.md` (the bit-vector chain,
one instance of this note), `design/layouts-and-specialization.md` (the
obligations specialization creates), `design/dependent-core-kernel.md` (the
kernel that checks everything), `design/refinement-types.md` (decidable
refinement predicates).

## 1 The architectural rule: untrusted search, small trusted check

Every strategy below has the same shape, which is what keeps the trusted
base small no matter how much automation grows:

- **Search is untrusted.** A tactic, a decision procedure's search, or an
  external solver may be arbitrarily large and clever, and may be wrong.
- **The kernel checks the result.** Only a kernel-checked artifact becomes a
  proof. Three ways to produce one:
  1. **Proof terms.** The tactic emits an ordinary proof term (rewriting
     chains from `simp`, congruence-closure explanations from `grind`). The
     kernel type-checks it like any proof.
  2. **Proof by reflection.** A decision procedure is written in Cinch,
     proven correct once (soundness theorem: "if the procedure says yes, the
     proposition holds"), and run inside the kernel by evaluation on a
     reified goal. `omega`, `ring`, `decide` and `bv_decide`'s bit-blaster
     work this way in Lean.
  3. **Certificate checking.** An external solver emits a certificate (an
     LRAT refutation from a SAT solver, an Alethe or LFSC proof from an SMT
     solver, Farkas coefficients for linear arithmetic, a Positivstellensatz
     witness for polynomial inequalities) and a verified checker validates
     it in the kernel.
- **The check's tier is recorded.** Kernel evaluation is the minimal trusted
  base. Running the check as compiled code is faster and adds the Cinch
  compiler to the trusted base (Lean's `bv_decide` and `native_decide` take
  this route); it is allowed only as an explicit, recorded tier
  (`design/layouts-and-specialization.md` section 7), never silently.
- **Results are cached in the store.** A discharged goal is keyed by its
  content hash in the package store, so each fact is proven once per
  machine and shipped with its package.

## 2 Where tactics live in Cinch

Lean's tactics are Lean programs run at elaboration time that build proof
terms. Cinch already has the matching machinery: typed staging and macros
(P9: `stage`/`macro` definitions evaluated at compile time by the D92
evaluator, typed quotes, `quoteDecls`, hygienic name splices). A tactic is a
stage function from a goal to a proof term (or to a certificate plus the
reflective checker invocation), and the kernel checks what it returns. This
keeps tactics user-extensible, and it means the tactic language is Cinch
itself, not a second language. The open decision "Typed-code contexts and
stage dependencies" (`design/open-decisions-and-implementation-plan.md`)
must settle how a tactic sees its goal's context.

## 3 Catalogue by domain

Lean names are given because Lean is the reference point; the Cinch uses
are the obligations this project already creates.

| Domain | Lean (and other) strategies | Mechanism | Cinch obligations it discharges |
| --- | --- | --- | --- |
| Decidable finite propositions | `decide` (kernel evaluation of a `Decidable` instance) | reflection | small exhaustive facts (an 8-bit operation over all 2^16 inputs), refinement predicates on literals, constructor tags |
| Linear integer and natural arithmetic (Presburger) | `omega` (Lean core); Coq `lia` | reflection | array and `Bytes` index bounds, `Vec n` lengths, packed-layout offsets and sizes (`design/layouts-and-specialization.md` section 3), termination measures, the `D113` bounded-recursion depths |
| Rewriting with a lemma database | `simp`, `simp_arith`, `conv` | proof terms | representation-instance isomorphism laws (`view (cons x xs) = Cons x xs`), fast-path equalities, unfolding definitions for D55 rewrites |
| Equality and congruence reasoning, instantiation of lemmas | `grind` (congruence closure, E-matching, constraint propagation, case splits, linear arithmetic and commutative-ring solvers; [reference](https://lean-lang.org/doc/reference/latest/The--grind--tactic)) | proof terms | the "everything else" of specialization proofs: chaining constructor injectivity, `view`/build round trips, instance-method unfolding |
| Commutative rings and fields | `ring`, `field_simp` (Mathlib); Coq `ring` | reflection | algebraic class laws of numeric instances (`Monoid (Sum Nat)`, `Semiring`), SIMD reduction re-association under wrapping arithmetic (platform specification section 4) |
| Associativity and commutativity normalization | `ac_rfl`, `abel` (Mathlib) | reflection | `Semigroup`/`Monoid` laws, the balanced `mconcat` equalling the right fold (associativity, `design/layouts-and-specialization.md`), `sconcat`/`stimes` |
| Linear and nonlinear real/ordered arithmetic | `linarith`, `nlinarith`, `polyrith`, `positivity` (Mathlib) | certificates (Farkas, Positivstellensatz) | cost bounds and resource bounds stated as inequalities; `Rational`/`BigDecimal` laws |
| Numerals | `norm_num` (Mathlib) | proof terms | literal folding facts, `Nat`/`Int` constant arithmetic in specifications |
| Bit-vectors | `bv_decide` (bit-blast, SAT, LRAT) | certificates | machine-integer `prim` laws and `prim` bodies against LLVM/ISA semantics (`design/verified-primitives.md`) |
| First-order goals, general automation | `aesop` (proof search), [Duper](https://github.com/leanprover-community/duper) (superposition), [LeanHammer](https://cmu-l3.github.io/lean-hammer/) (premise selection plus external provers plus reconstruction) | proof terms after reconstruction | glue goals that no single domain covers |
| SMT combinations | [lean-smt](https://link.springer.com/chapter/10.1007/978-3-031-98682-6_11) (cvc5 with proof reconstruction); Isabelle's Sledgehammer | certificates | mixed arithmetic, datatypes and uninterpreted functions, as one call |
| Induction and case analysis | `induction`, `cases`, `rcases`, functional induction, `decreasing_by` | proof terms | structural proofs over `List`, trees and `Map`; totality of representation functions (a representation's `view` must be total to appear in a proof) |
| Ownership and mutable state | separation logic ([Iris](https://iris-project.org/)); [Verus](https://verus-lang.github.io/verus/guide) (linear ghost types plus SMT); RustBelt | proof terms over a heap logic | representations over mutable buffers, Perceus in-place reuse (FP2-style fully-in-place functions), `Mut r a` and region reasoning |
| Floating point | IEEE formalizations (Flocq in Coq), SMT floating-point theory | certificates (bit-blasting) | `F32`/`F64` `prim` laws; exact claims only, the platform specification's separate floating-point contract |
| Counterexamples (falsification, not proof) | `plausible` (Lean), QuickCheck | testing | already the law layer's sample checking: the "sample-checked" tier, and the first thing to run before attempting a proof |

## 4 What Cinch needs first

Ordered by the obligations the current roadmap creates:

1. **Induction and case analysis, rewriting (`simp`-like), `decide`.** These
   discharge representation-instance isomorphisms over algebraic data types,
   which section 6 of `design/layouts-and-specialization.md` makes the
   common case.
2. **Linear arithmetic (`omega`-like).** Every layout, index and length
   obligation is Presburger arithmetic; packed layouts and value types
   (section 3 there) generate them by construction.
3. **Congruence closure with E-matching (`grind`-like).** Glue for
   everything above; it is what makes specialization proofs short enough to
   write routinely.
4. **AC normalization and commutative rings.** Class laws of the base
   library's instances, and SIMD re-association.
5. **Bit-vectors.** The verified-primitives chain.
6. **A certificate bridge to SAT/SMT solvers.** One verified LRAT checker,
   then an SMT proof format.
7. **Separation logic over Cinch's ownership types.** Representations over
   mutable buffers and guaranteed in-place reuse (the user's choice of "both
   runtime checks and uniqueness discipline where able to be statically
   determined", `design/reclamation.md` section 12).
8. **Floating point.**

## 5 Relation to what exists

- The kernel (`design/dependent-core-kernel.md`, design draft) has `Eq`,
  `refl`, `congr`, structural-induction `case` and `decide` with a supplied
  decision procedure; `examples/source/03-map-fusion.cinch` carries a
  hand-written `mapFusion` proof (D230/D270). There is no tactic layer yet:
  proofs are written as terms.
- `rewrite ... justify` (D55) accepts kernel evidence or a citable class
  `law`, and records undischarged justifications in `assumptions.declared`;
  tactics produce exactly that evidence.
- The P18 law layer checks class laws sample-wise: the falsification step
  and the "sample-checked" tier.
- Refinement predicates are a closed, decidable vocabulary
  (`design/refinement-types.md`); `decide`-style reflection already fits it.

## 6 Open decisions

- The tactic surface: `by <tactic>` in proof positions (Lean's form) or
  `justify (tactic ...)` reusing D55's slot; and whether tactics are `stage`
  functions directly or a small tactic combinator library over them.
- How a tactic sees its goal and local context (the open "typed-code
  contexts" decision).
- Which certificate formats are first-class (LRAT proposed first, then an
  SMT proof format) and which external solvers are supported.
- Whether the compiled-check tier exists at all, or kernel evaluation only.
- How lemma databases (`simp` sets, `grind` E-matching patterns) are
  declared: attributes on theorems (Lean's `@[simp]`, `@[grind]`) fit the
  attribute system.
