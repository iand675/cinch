# Dependent core kernel design

Design note for the Cinch toolchain, roadmap slice P7 in
`compiler/CONFORMANCE.md`. Companion to the core language specification,
which states obligations; this note fixes the kernel shape and its mapping
onto the existing pipeline.

Status: design draft for implementation. Items marked **open** are listed in
section 9.

# 1 Shape of the system

A small dependently typed core with bidirectional elaboration. The surface
elaborates into the core; the core is what the erasure pass and the code
generator see. The existing Hindley-Milner layer does not go away: it is the
elaborator's inference engine for the simply typed fragment and for effect
rows and multiplicities. Dependent checking is layered as an explicit
judgment on top, so every slice keeps working while the core grows.

```text
surface -> elaboration -> core terms (typed) -> erasure -> core IR -> LLVM
                 ^                |
                 | conversion     +-- evidence checked here (proofs, totality)
                 +-- unification (metas for inference)
```

# 2 Kinds and universes

- `Type` is the universe of computational types; `Prop` is the universe of
  erased propositions (section 7). `Type : Type` is NOT admitted; a universe
  hierarchy `Type 0 : Type 1 : ...` is implicit (predicative, cumulative in
  a limited form: `Type i` embeds into `Type j` for `i <= j`).
- Kinds, types and values share term syntax (draft 0.2); the checker assigns
  kinds. Universe levels are inferred with the usual max rule for Pi.
- `Nat` and `Int` are inductive types in `Type 0` with mathematical
  semantics (section 6); machine types are primitives in `Type 0` whose
  runtime representation is fixed by the primitive types specification.

# 3 Core terms and judgments

Terms: variables, `Pi (x : A) -> B`, `Sigma (x : A). B`, lambda, application,
`let`, constructors, dependent pattern matching, `Type i`, `Prop`, proof
terms (erased), and the existing effect-row and multiplicity annotations on
arrows.

Judgments (bidirectional):

```text
Gamma |- t => A          infer
Gamma |- t <= A          check
Gamma |- A type          well-formed types
A == B                   conversion (definitional equality)
Gamma |- t : A ok        usage/multiplicity check (P5)
```

Inference forms are the usual ones (variables, application of an inferable
head, annotations). Everything else checks against an expected type; the
expected type of a pattern match is propagated into branches.

# 4 Conversion

Definitional equality is beta (for dependent application), delta (transparent
definitions only), and iota (constructor case reduction on constructor
applications), plus proof irrelevance for `Prop` terms (**open** whether this
is definitional or propositional) and eta for records/units where useful
(**open**). Conversion is a terminating procedure under the termination
discipline of section 8: it never unfolds recursive definitions without a
structural decrease argument; unchecked unfolding is an error, not a loop.

Machine arithmetic is NOT part of conversion: `2 + 2 == 4` is proved, not
reduced (the primitive types specification makes wrapping/checked families
explicit; the mathematical `Nat`/`Int` operations reduce only on numerals
where a total decision procedure applies, see `decide` in section 8).

# 5 Inductive families

Declaration:

```haskell
data Vec (n :: Nat) (a :: Type) where
  Nil  :: Vec 0 a
  Cons :: forall k. a -> Vec k a -> Vec (k + 1) a
```

- Constructors declare their indices explicitly after elaboration (the
  surface suppresses constructor-local implicit binders; the elaborator
  re-inserts them).
- Positivity is checked syntactically (strict positivity for the bootstrap;
  positivity-preserving nesting is **open**).
- The runtime representation of an inductive family is the ADT block already
  in use: indices that are computational (like `n`) are RETAINED as fields
  when execution needs them (core specification section 3: erasure must not
  drop witnesses the runtime needs); erased indices are dropped in the
  erasure pass.

# 6 Mathematical numbers

`Nat` and `Int` are inductive (or primitive-backed) unbounded numbers whose
semantics are exact. Implementation: the Rust runtime's bignum `Nat`/`Int`
(P4) provides the representation; the core treats them as abstract types
with total operations (`+`, `-`, `*` on `Nat` with truncated subtraction
matching the inductive definition; `Int` exact). Literals elaborate to
numeral constructors; `decide (i < n)` uses the runtime comparison and
returns evidence scoped to the examined values (core specification
section 4), never a general proof.

# 7 Propositions and erasure

- `Prop` is a universe of erased propositions. `Eq x y`, `n <= m` etc. live
  in `Prop`; `refl`, `congr` and user `proof` declarations construct
  evidence in `Prop`.
- Erasure rules:
  1. Terms in `Prop`, `erased` binders and `erased` record fields are erased
     before code generation; they occupy no runtime representation.
  2. Erasure must not remove computation with observable effects or
     divergence; an erased proof of a total fact can vanish entirely.
  3. No elimination of `Prop` into `Type` except through the allowed
     singleton/decidable paths; computational decisions from erased evidence
     are rejected (`proof impossible = impossible` style cycles are rejected
     by termination, and `Fin 7` constructions by index checking).
  4. Erased evidence may appear in types (indices, refinements) but never in
     the runtime representation of a value that execution inspects.
- The erasure pass runs on core terms and is checked: it may only remove
  terms the typing judgment marks erased.

# 8 Totality, evidence, refinements

- Total functions: structural recursion (a recursive call must decrease a
  named argument structurally) or a checked well-founded measure. The
  checker returns a checkable result; automation may suggest, never assume.
  Divergence is explicit (`Diverge` in the effect row or a `spec`/partial
  form); a looping function cannot inhabit a `Prop`.
- Branch evidence: `if`/`case` introduce facts into scope on their branches
  (the scrutinee equals the pattern). Refinements are the surface for these
  facts; the core records them as `Eq`-style hypotheses, and flow-sensitive
  invalidation drops facts when mutable state changes (core specification
  section 5).
- As built (branch facts from matches): every branch of a match binds
  what its test proves about the column's value, by one rule. The
  decision tree's branches are exact, so the fact holds on every path into
  the branch: a default branch knows the value is none of the literals,
  and an earlier guarded row never removes values from a later branch (a
  failing guard falls through inside its own branch). The readings:
  - a `Bool` scrutinee the checker reads as a predicate: its `True` branch
    gets the predicate, its `False` branch the refutation (a wildcard row
    after `True` sits in the `False` branch). Readable predicates are a
    comparison (`<`, `<=`, `>`, `>=`, `==`, `/=`) whose operator is the
    prelude's `Ord`/`Eq` method and whose operands are `Nat`, `Int`,
    machine integers or `Char` (never floats: under IEEE a NaN makes
    `not (x < y)` differ from `x >= y`); a structural predicate written as
    the condition (`if NonZero d then`); and `&&`, `||`, and the prelude's
    `not` over those;
  - a numeric scrutinee: a literal branch states `x == k`, the default
    branch `x /= k` for every literal;
  - `comparing k x y` (the prelude's three-way comparison; `Ord` itself has
    only the four boolean methods) at a numeric key: `LT`, `EQ`, `GT` state
    `k x < k y`, `k x == k y`, `k x > k y`;
  - a list-shaped scrutinee (the `NotNull` vocabulary): a non-empty
    constructor's branch states `NotNull x`, the empty one `not (NotNull
    x)`. Other constructor types have no vocabulary predicate and state
    nothing (a predicate per constructor, `IsJust`-style, would be needed).
  The value readings state their fact about the scrutinee and about every
  name a row binds to the column (`e -> nat-div n e` after a `0` row, or a
  function equation's parameter after a literal equation). A scrutinee is
  read only when a fact may name it: numerals, locals, borrow reads,
  arithmetic (not at machine integers, whose arithmetic wraps while index
  arithmetic is mathematical), `bytes-len`, and pure global functions
  taking no borrow, pointer or byte buffer (a buffer's contents change
  under `bytes-set` with no tracked footprint). `length` and `toNat` are
  excluded: the index reading takes them as the baked measures whatever
  they resolve to.
  `if c then a else b` is `case c of True -> a; False -> b`: the same
  `Bool` reading and branch rule, and a `case` whose `Bool` scrutinee no
  pattern names branches on the test itself, so both compile to the same
  compare-and-branch. A guard is a `Bool` column over its row (its body is
  the `True` branch, the fallthrough to later rows the `False` branch,
  reaching only rows whose names denote the same values). `p && q` checks
  `q` in the `True` branch of `p` and holds both (or just `p` when `q` has
  no reading); `p || q` checks `q` in the `False` branch of `p` and
  refutes both. A `Bool` the checker cannot read carries no fact.
  - Mechanism: the facts are native to the checker, not a desugaring to
    `case decide c of`. Each fact conjunct is a hidden `Prop` binder in the
    branch scope (its name is no identifier, so source can neither name
    nor forge it), so the existing discharge (`discharge_oblig`) and the
    D192 staleness tracker (`ev_derivs`/`ev_uses`, the same write log)
    read it exactly as a `Yes h` binder.
  - Reading never changes typing: a condition elaborates exactly as a
    plain one (a comparison is the ordinary application of the prelude's
    `Ord`/`Eq` method, with the same test and cost), and the fact is read
    off that elaboration only when the method's class argument came out a
    total order and both operands are terms a fact may name. A condition
    whose reading would need other typing simply has no fact.
  - Staleness: a fact is bound when its branch is elaborated, so writes in
    sibling branches (not on its path) do not count, and it also carries
    the window between its own test and the end of its condition, so a
    write later in the condition (`copyOut m >= 5 || poke m`) does.
  - Cost: the plain `if`'s, one comparison and a branch. No `Dec` value is
    built, and a fact-bearing `if`, `case c of True/False` and literal
    `case` allocate exactly what a plain `if` does. (`case decide c of
    Yes/No` does not allocate either: `Yes`/`No` are static nullary words,
    but it materializes the word and dispatches on its header; the
    branch-fact path skips both.)
  - Use sites: a `using` obligation at a call, and a plain value checked
    against a refinement whose container it is (`nat-div n d` with `d ::
    Nat` against `NonZero Nat`): the predicate instantiated at the value's
    index reading is decided statically or entailed by evidence in scope;
    the coercion is the same word.
  - Entailment is `pred_entails`: `pred_implies`'s rows over the caller's
    own names, never up to renaming (`a < b` says nothing about `b < a`).
    One item, or the items over the obligation's names together, must
    entail the obligation; an item whose names are rebound after it (an
    inner `let x`) is out of scope. What discharges is what the table
    proves: syntactic identity, conjunct projection, interval containment
    and disjointness over literal bounds, `x /= 0` and a zero-excluding
    range for `NonZero x`, and a range missing `k` for `x /= k`.
    When the table fails, every fact in scope together (over any names)
    is decided by difference logic (`linear_entails`, `types.rs`): `i <
    n` entails `i + 1 <= n`, and `i <= j`, `j < n` entail `i < n`
    (`design/refinement-types.md`, "linear facts over variables"). Its
    proof is a negative-cycle certificate checked before the discharge
    accepts it; the facts the certificate cites are the ones the
    staleness check reads.
- `decide P` exists only with a supplied decision procedure; its result is
  `Yes evidence | No refutation` scoped to the examined values.

# 9 Open decisions

- Proof irrelevance definitional or propositional; eta for which types.
- Strict positivity vs positivity-preserving nesting.
- Whether `Nat`/`Int` are inductive types with runtime specialization or
  primitive abstract types with total semantics (implementation may pick the
  latter if the former blocks performance work; the surface cannot tell).
- Universe cumulativity: full or only at `Type 0` boundaries.
- Sized/guarded discipline choice for coinduction (core specification
  section 9) and its interaction with the totality checker.

# 10 Mapping onto the current pipeline

- `types.rs` gains `Ty::Pi`, `Ty::Sigma`, `Ty::Prop`, `Ty::Univ(level)`,
  indexed `Ty::Adt` with index arguments; unification gets a conversion
  fallback (try definitional equality when syntactic unification fails).
- `check_terms.rs` grows a dependent checking mode; the simply typed path
  remains the fast path when no dependent forms appear.
- The core IR (`core.rs`) stays the erasure output; erased slots are simply
  absent there, so codegen needs no changes for erasure beyond what P2/P4
  already provide for numbers.
