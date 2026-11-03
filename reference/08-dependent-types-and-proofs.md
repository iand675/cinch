# Dependent types, the logical fragment, and evidence

The Rust compiler supports dependent function/pair types, indexed constructors, proof erasure, and a checked logical fragment. Its accepted elimination and proof procedures are intentionally bounded; a feature's design syntax is not itself evidence that every use is accepted.

# Universes and dependent binders

`Type` is stratified. An attempt to use `Type` as an element of itself is rejected rather than collapsing the universe hierarchy. `(n :: Nat) -> ResultType n` is a dependent function; the result can mention the argument. `Sigma (n :: Nat). Family n` packages a witness with a corresponding value.

`forall` binders may have value or kind annotations. The checker solves admitted implicit parameters from application context. Conversion recognizes definitionally equal supported computations, such as concrete index arithmetic. It does not use arbitrary optimizer rewrite rules as conversion axioms.

# Indexed families and coverage

`Vec n a` and `Fin n` are central examples. `Fin` needs evidence of an index bound. Constructor coverage accounts for index equations: an impossible constructor need not be matched, and an explicit impossible arm is rejected. A vector at index zero can be matched only by its empty constructor.

<span class="support-tag support-tag--planned">Coming soon</span> **broader rigid-index elimination.** Constructor matching for some families applied to unresolved rigid indices is a named rejection in the Rust checker. The diagnostic names the unsolved equation. The intended dependent language's broader matching story remains described in the core specification; current examples use supported decisions, computed indices, and witnesses.

# Proof terms and erasure

`Prop` is computationally irrelevant evidence. Proof irrelevance and erasure remove witnesses from runtime representations where required. An erased argument cannot supply runtime output; `%0` usage checking enforces that separation.

`refl` establishes definitional equality. `congr` transports equality through a function in the supported fragment. Explicit proof declarations and `decide` can supply other evidence. A declaration that loops or performs an inadmissible effect cannot be used as a logical proof. Recursive datatype occurrences must satisfy positivity.

# Decide and obligations

A decidable proposition produces `Yes evidence` or `No counterevidence` at the appropriate computation boundary. Successful runtime checking yields evidence only on its successful path. Concrete false obligations are rejected, and unresolved obligations do not silently become true. Facts from branches can combine to justify linear bounds, nonzero divisors, subtraction, and conversion ranges.

Obligation/manifest status distinguishes established facts from assumptions and unsupported checks. A finite law-sampling pass does not change that logical distinction. Mutating a place invalidates dependent facts about its old contents; holding the old proof token is insufficient.

# Proof and optimization boundaries

Kernel equality, signature laws, and optimizer assumptions have different roles. A `rewrite ... justify` may cite a signature law and record an undischarged assumption. That does not extend definitional equality or let an assumption prove a type-level contradiction. Residual code must still pass type, effect, usage, and preservation checks.

<span class="support-tag support-tag--planned">Coming soon</span> **more automatic proofs and certificates.** The [proof automation design](../design/proof-automation.md), [verified primitive design](../design/verified-primitives.md), and [dependent core design](../design/dependent-core-kernel.md) describe intended extensions. A future certificate checker or broader solver must retain the boundary that unsupported reasoning is not accepted by spelling a proposition.

Implementation: `check.rs`, `check_decls.rs`, `check_terms.rs`, `types.rs`; executable/checking suites: `dependent.rs`, `safety.rs`, `refinements.rs`, `case_evidence.rs`, `linear_facts.rs`.
