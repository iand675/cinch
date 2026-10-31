# Refinements, narrowing, and proof obligations

A refinement declares a predicate over a carrier without adding a runtime wrapper allocation. The erased witness justifies uses of the carrier in operations with stronger preconditions.

# Declaration and construction

```text
type Small = refine (n :: Nat) with n < 5
type NonEmpty a = refine (xs :: List a) with NotNull xs
```

A transparent constructor takes the carrier and checked evidence. For example, `Small 3 (decide (3 < 5))` discharges a concrete true predicate. For a dynamic value, branch on `decide`: only `Yes witness` grants the positive fact. `No` is an ordinary alternative that must be handled.

`unrefine value` explicitly recovers the carrier. A runtime numeral at a demanded refinement can be accepted when its predicate is statically decided, as with a nonzero literal divisor. This does not permit an arbitrary variable of the carrier type to pass without evidence.

# Predicates and facts

Supported predicates include numeric comparisons, nonzerohood, list nonemptiness, sorting, ASCII, and admitted compound predicates. `InRange lo hi a` expresses inclusive numeric bounds. Predicate/type checking rejects nonsensical carriers such as `NonZero Bool`.

The checker uses facts from successful decisions and branch conditions, including the relevant false branches, guards, and short-circuit forms. Obligations can appear in `using` contexts over named dependent arguments. A loop can state `i <= n`, advance to `i + 1` only under `i < n`, and discharge the recursive call's bound without a runtime check at every consumer.

The arithmetic implication machinery includes difference constraints and a supported linear-arithmetic certificate path. Mathematical `Nat`/`Int` operations are exact in these facts. Machine arithmetic with an overflow policy is opaque unless its applicable range/operation facts justify more. A true nonlinear proposition is not automatically solvable; unresolved obligations name the needed evidence.

# Narrowing and forgetting

`narrow value to Target with witness` checks strengthening to the target predicate. `forget [predicate, ...] from value` drops named top-level conjuncts (`forget (predicate) from value` is the single-predicate form). Type ascription can also weaken to an implied predicate without runtime work; strengthening requires evidence. `forget` does not invent another predicate. These operations are part of the refinement lattice, rather than a blanket implicit-subtyping rule. The exact parser forms are recorded in the [refinement design](../design/refinement-types.md) and exercised by `lattice.rs`.

# Mutation and scope

Facts are about values and places, not spellings alone. Rebinding a name or writing overlapping mutable state invalidates the applicable evidence. A stale witness cannot establish bounds on changed data. Region escape and capability identity rules still apply to a refined value's carrier.

# Instances and preservation

A transferred instance reuses carrier methods only where the refinement predicate is preserved. For `NonEmpty`, appending nonempty lists preserves the predicate, while the empty identity violates it. The compiler rejects a preservation failure by naming the method's preservation law.

A checked finite sample is reported as such. A method whose preservation is not established must not be described as a universally proved instance merely because a declaration was parsed. The interface/manifest records law and assumption statuses.

The shipped refined modules are [NonEmpty](../library/Data.NonEmpty.md), [NonZero](../library/Data.NonZero.md), [Sorted](../library/Data.Sorted.md), and [Ascii](../library/Data.Ascii.md). Use their total constructors at input boundaries.

<span class="support-tag support-tag--planned">Coming soon</span> **broader proof automation.** The planned solver/kernel extensions cover more reasoning patterns. Unsupported propositions still require explicit admissible evidence or a differently structured program today; the language's intended proof model is documented in [proof automation](../design/proof-automation.md).

Implementation: `check.rs`, `check_terms.rs`, `check_decls.rs`, `types.rs`; suites: `refinements.rs`, `lattice.rs`, `linear_facts.rs`, `case_evidence.rs`, `evidence_invalidation.rs`, `range_facts.rs`.
