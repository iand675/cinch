# Evidence Map: Refinement/Dependent Machinery for Proven-Total Operations

Status: survey taken 2026-10-30 by a read-only scout; line numbers drift.
It feeds the migration slices of `design/no-faults.md`.


## Summary

The checker already has a working, tested evidence pipeline: `refine`/`Dec`/`decide`, builtin `Fin`/`Vec`/`Sigma`, `using`-context obligation discharge with D192 staleness, and a table-driven implication/weakening/narrowing lattice (`forget`/`narrow`). What it does **not** have: (1) any evidence narrowing from a plain `if`/boolean comparison into its branches (only `case` on `Dec`'s `Yes`/`No` constructors binds usable evidence); (2) linear arithmetic over two variables (only literal-bounded interval containment); (3) any `Fin`/refined-index alternative wired onto `Bytes`, which is the type `bytes-get`/`bytes-set` actually index. Literal-divisor `NonZero` obligations already fold at compile time for free. This asymmetry drives the recommended order at the end: cheapest wins first, then the two missing primitives that gate the two largest call-site families.

## Files

- `design/refinement-types.md` (whole file): refinement shape, predicate vocabulary, weakening/narrowing/`forget`/compound-predicate design, `using`-context constraint reading.
- `design/dependent-core-kernel.md` (section 6, 8): `Nat`/`Int` as abstract totals; `decide` scoped to examined values, never a general proof; branch evidence design (not fully implemented for plain `if`).
- `design/proof-automation.md` (section 1, 3, 4): trusted-kernel-checks-untrusted-search architecture; catalogue naming linear arithmetic ("omega-like") and induction as still-needed, not yet built.
- `design/no-faults.md`, `design/no-faults-inventory.md`: the constraint and the current fault census (bytes-get/set 190+, div/rem families 170+, nat-sub 15+).
- `compiler/CONFORMANCE.md:830-996`: implementation status ledger for D248-D261 (refinements/decide/Fin/Vec, narrowing/forget, compound predicates, `using` obligations).
- `compiler/src/check_terms.rs`: `decide_shape` (2456), `decide_compound` (2325-2460), `decide_operand`/baked measures (4523-4600), `Forget`/`Narrow` elaboration (897-1010), `using`-obligation discharge (690-820, 7609-7748), evidence invalidation write/use tracking (2306-2311, 2983-2987), `ObligStatus`/`static_obligation`/`pred_obligation` (10-14, 3492-3602), plain `if` elaboration with **no** evidence capture (869-885, 1438-1441).
- `compiler/src/types.rs:838-1023`: `pred_conjuncts`, `pred_implies`/`pred_implies_canon`, the `Interval`/`interval_atom`/`atom_implied_by` table (literal-numeral bounds only, via `bound_value`).
- `compiler/src/check_decls.rs:1453-1560`: builtin `Vec`/`Fin`/`Dec` family registration (`install_index_families`).
- `compiler/prelude/Data/NonZero.cinch` (whole file, 27-50): the one implemented refinement example.
- `compiler/prelude/Data/Nat.cinch:22-80`, `Data/Int.cinch:19-99`, `Data/Bytes.cinch:19-93`: today's partial signatures and class instances.
- `compiler/prelude/Text/Lex.cinch:196-199, 315-316, 1029-1031`; `compiler/prelude/Text/Show.cinch:85-99`; `compiler/prelude/Data/Date.cinch:159-373`, `Data/Time.cinch:250-365`: representative call sites.
- `compiler/prelude/Effect/Http.cinch:213-217`, `compiler/prelude/Text/Toml.cinch:205-214`: the only existing bounds-guard wrappers, written with a plain `if`, not `decide`/`Fin`.
- `compiler/tests/dependent.rs:62-93, 313-322`; `compiler/tests/lattice.rs:682-830`: working examples of `decide`/`Fin`/`Sigma` and `using`-obligation discharge.
- `examples/source/01-bounded-index.cinch`, `02-vector-filter.cinch`: the canonical `Fin`/`Vec`/`Sigma` idiom at prose-example scale.

## Architecture

Surface terms elaborate through `check_terms.rs` into core terms; `decide` is a special application form recognized by `decide_shape`/`decide_compound`/`decide_operand`, producing a `Dec p` value (`Yes`/`No`, both builtin constructors registered alongside `Vec`/`Fin` in `check_decls.rs`). A `case` match on `Yes h`/`No h` binds `h` as an ordinary pattern variable carrying `Prop`-erased evidence; this is the **only** place evidence becomes usable in a branch. Refinements (`refine (x :: T) with P`) are a predicate-indexed family whose runtime representation is the container itself; `pred_implies` (interval-containment plus syntactic rows) decides implication for implicit weakening, `narrow` requires an explicit `Dec` for strengthening, `forget` drops named top-level conjuncts. `using`-context obligations on signatures and instances discharge at the call's own arguments through the same `Dec` path, re-checked for staleness whenever `check_terms.rs`'s mutable-write tracking (`note_ev_write_loc`/`record_ev_use`) sees the underlying location written. None of this touches plain `if`: its elaboration (869-885, 1438-1441) checks the condition against `Bool` and infers/checks the two branches in isolation, recording nothing from the condition.

---

## 1. Refinement types (D248-D267)

**Introduction.** `type NonZero = refine (n :: Int) with NonZero n` (`compiler/prelude/Data/NonZero.cinch:27`). The transparent constructor (`NonZero n w`) takes the container value and a `Dec`-discharged witness; the total front door is:

```
non-zero :: Int -> Option NonZero
non-zero n = case decide (NonZero n) of
  Yes w -> Some (NonZero n w)
  No _ -> None
```
(`NonZero.cinch:29-31`.)

**Elimination.** Rep-identity: a refined value *is* its container at runtime (zero-cost witness erasure, D31). `unrefine` is the explicit forgetful projection back to the base type (design doc, "Projection is free").

**Predicate vocabulary (closed, decidable).** `NotNull`/`NonZero`/`Ascii`/`Sorted`(`Ordered`) plus `Dec` atoms (bounded comparisons, index equalities, `Not`), with `&&`/`||` added by D256. Quantifiers are refused (`design/refinement-types.md`, "Compound predicates" section; `check.rs` kind registration for `Prop`).

**Implication and weakening (D260/D261 read as D258).** `pred_implies`/`pred_implies_canon` (`compiler/src/types.rs:899-920`) implements: syntactic identity, `p && q implies p`, and an interval-containment table for comparison/measure atoms (`compiler/src/types.rs:950-1007`). Weakening along this order is **implicit and free**; strengthening always needs a `narrow` cast carrying a `Dec` of the stronger predicate (`check_terms.rs:962-1010`).

**`using`-obligation discharge at a call site (the `Dec` path, D192 staleness).** A signature like `merge2 :: using (Sorted xs, Sorted ys) => (xs :: List Nat) -> (ys :: List Nat) -> List Nat` records its obligations as `PropOblig`s (`check.rs:2012-2016`); at the call, `check_terms.rs:690-820` discharges each named obligation against evidence already in scope through the `Dec` path. This is tested end-to-end in `compiler/tests/lattice.rs:682-830` (`using_obligations_discharge_through_the_dec_evidence_path`, `signature_obligations_constrain_callers_identically`). D192 staleness re-checks: a mutable write to the region an evidence item's statement reads invalidates it (`note_ev_write_loc`/`note_opaque_write_from`, `check_terms.rs:709-712, 2306-2311`; use-site re-check `record_ev_use`, `check_terms.rs:2983-2987`).

**MWE** (refine introduction/elimination/discharge, all working today):
```
non-zero 10          -- Yes w folds at compile time (see section 3); Option NonZero, Some
case decide (NonZero n) of
  Yes w -> NonZero n w     -- introduction: transparent constructor + witness
  No _  -> ...
let plain = unrefine nz   -- elimination: free projection to Int
```

---

## 2. `Dec`/`decide`

**Signature and shape.** `decide` accepts a comparison (`<`,`<=`,`>`,`>=`,`==`/`=`,`/=`/`!=`) or `Eq x y` (`decide_shape`, `compiler/src/check_terms.rs:2456-2473`), plus compound `&&`/`||`/`Not` forms recognized syntactically off the surface `if`-sugar (`decide_compound`, `check_terms.rs:2325-2460`). It returns `Dec p = Yes (h :: p) | No (h :: Not p)`, both builtin constructors (`check_decls.rs:1518-1537`). The `length`/`toNat` builtins are specially recognized operands so `decide (i < length b)` lowers to the real `BytesOp::Len` comparison rather than a source function call (`check_terms.rs:4523-4600`).

**MWE** (`compiler/tests/dependent.rs:65-69`):
```
safeAt :: (n :: Nat) -> Nat -> Option (Fin n)
safeAt n i = case decide (i < n) of
  Yes bound -> Some (Fin i bound)
  No _ -> None
```

**Does evidence flow from a `case` on a comparison into the branch?** Yes, exactly and only through pattern-matching on `Dec`'s `Yes`/`No` constructors: `bound` above is an ordinary bound variable carrying the erased proof `i < n`, usable to build `Fin i bound`.

**Does evidence flow from a plain `if` condition?** **No.** `ast::Term::If` elaboration (`check_terms.rs:869-885` for inference, `1438-1441` for checking) checks `cond :: Bool` and infers/checks the two branches independently; nothing about `cond`'s truth is recorded as evidence in either branch. Concretely, `Effect/Http.cinch:213-217`'s one existing bounds-guard wrapper:
```
byte-at b i = case i >= bytes-len b of
  True -> None
  False -> Some (bytes-get b i)
```
type-checks today only because `bytes-get`'s own signature carries no precondition at all (section 4); if `bytes-get` required `i < bytes-len b` evidence, this exact wrapper would stop compiling, because the `False` branch gets no narrowed fact from the guard.

**Path-sensitive narrowing: none beyond the `Dec` constructor match.** Every obligation is scoped "to the examined values" (`design/dependent-core-kernel.md` section 6); there is no flow-typing pass that derives `i < n` from a `Bool`-typed `if`/comparison outside `decide`.

---

## 3. `NonZero`, `Fin`, `Vec n`, `Sigma`, and the dependent kernel

**`NonZero`**: `compiler/prelude/Data/NonZero.cinch:27` (refinement over `Int` only; no `Nat` alias exists in the prelude, confirmed by directory listing: `Data/NonZero.cinch` is the only `NonZero`/`Fin`/`Vec`/`Sigma` prelude file; `Fin`/`Vec`/`Sigma` have no prelude files at all, only compiler builtins and test/example usage).

**`Fin`**: builtin family, `compiler/src/check_decls.rs:1497-1520`:
```
data Fin (n :: Nat) where Fin :: (i :: Nat) -> (bound :: i < n) -> Fin n
```
`bound` is erased; `i` is the runtime index.

**`Vec n a`**: builtin family, `check_decls.rs:1456-1495`:
```
data Vec (n :: Nat) (a :: Type) where
  Nil  :: Vec 0 a
  Cons :: forall k. a -> Vec k a -> Vec (k + 1) a
```

**`Sigma`**: `ast.rs:126-127` (`Sigma(Name, Box<Type>, Box<Type>, Span)`), `types.rs:1378-1381`, surface in `parser.rs:2997-3001`. MWE (`compiler/tests/dependent.rs:75-80`):
```
pack :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
pack xs = (n, xs)
unpack :: (Sigma (m :: Nat). Vec m Int) -> Nat
unpack p = case p of (m, v) -> m
```

**Does `10 :: NonZero Nat` (or `Int`) get accepted at compile time?** There is no direct ascription syntax tested for this, but the underlying mechanism already statically folds it: `pred_obligation` (`check_terms.rs:3590-3602`) evaluates `NonZero n` over a numeral via `eval_idx`, returning `ObligStatus::Checked`/`Disproved` with **zero runtime check**. So `decide (NonZero 10)` folds to `Yes` at compile time already; construction today still goes through `non-zero :: Int -> Option NonZero` (a runtime-shaped `Option`, even though the underlying decision was compile-time) or the transparent constructor fed that statically-folded witness directly.

**Does the kernel run `decide` at compile time on literals? Linear-arithmetic reasoning, or only syntactic atom-set inclusion?** `static_obligation` (`check_terms.rs:3492-3566`) folds `Cmp`/`Eq` over numerals via `eval_idx`, and `And`/`Or`/`Not` combine the sub-results; `pred_obligation` folds the structural vocabulary the same way. Beyond that, `pred_implies` (`types.rs:899-920`) is **not** general linear arithmetic: it is syntactic atom-set inclusion plus one interval-containment table (`Interval`/`interval_atom`/`atom_implied_by`, `types.rs:950-1007`) that requires **one side of each comparison to be a numeral** (`bound_value`, `types.rs:986-1000`, folds only `Num`/`+`/`-`/`*` over numerals). A comparison between two *variables* (`i < n` where `n` is itself a variable, e.g. `bytes-len b`) produces **no** interval fact at all; `interval_atom` returns `None` when neither side is a numeral. This is the exact gap `design/proof-automation.md` section 4 item 2 names ("omega-like" linear arithmetic) as needed and not yet built.

---

## 4. Current signatures

| name | signature | site | instance |
|---|---|---|---|
| `bytes-get` | `Bytes -> Nat -> Nat` | `compiler/prelude/Data/Bytes.cinch:33` |: |
| `bytes-set` | `Bytes -> Nat -> Nat -> Unit` | `Bytes.cinch:34` |: |
| `bytes-len` | `Bytes -> Nat` | `Bytes.cinch:32` |: |
| `nat-sub` | `Nat -> Nat -> Nat` | `compiler/prelude/Data/Nat.cinch:28` | `Sub_Nat :: Sub Nat`, `(-) x y = nat-sub x y` (`Nat.cinch:49-50`) |
| `nat-div` | `Nat -> Nat -> Nat` | `Nat.cinch:30` | `Div_Nat` (`Nat.cinch:58-59`) |
| `nat-rem` | `Nat -> Nat -> Nat` | `Nat.cinch:31` | `Rem_Nat` (`Nat.cinch:55-56`) |
| `int-div` | `Int -> Int -> Int` | `compiler/prelude/Data/Int.cinch:24` | `Div_Int` (`Int.cinch:68-69`) |
| `int-rem` | `Int -> Int -> Int` | `Int.cinch:25` | `Rem_Int` (`Int.cinch:65-66`) |

`-` on `Nat` resolves through ordinary class dispatch (`Sub` typeclass) straight to the faulting foreign `nat-sub`; no static precondition exists on any of these today. `int-sub` (`Int.cinch:22`) is total already (Int is unbounded signed), so it is not in the fault inventory.

---

## 5. Three call sites each

**(a) Byte indexing in a loop (lexer).**
1. `compiler/prelude/Text/Lex.cinch:196-199`, `bytes-eq`: recurses `i+1` up to `bytes-len x`, reading `bytes-get x i`/`bytes-get y i`.
2. `Lex.cinch:315-316`, `render-pieces`: `nat-cmp i (bytes-len b)` guard then `utf8-at b i`.
3. `Lex.cinch:1029-1031`, `trim-attr-end`: `bytes-get b (j - 1)` on a decrementing index.

Evidence needed: `i < bytes-len x` (and `i < bytes-len y` for site 1), re-derived every recursive step. Today's machinery (`decide`+`Fin`) **can** express it (section 2/3), but each call site needs its own `case decide (i < length x) of Yes bound -> Fin i bound; No _ -> ...` inline, replacing the plain `Nat` index with `Fin`, at one `decide` + one `case` + one `Fin` construction **per loop iteration per indexed buffer** (two per call for site 1, which compares two buffers).

**(b) Division by a literal (hex/decimal rendering).**
1. `compiler/prelude/Text/Show.cinch:92`, `nat-digits-into`: `bytes-set b (pos + k - 1) (48 + nat-rem n 10)`.
2. `Show.cinch:93`, same function: `nat-div n 10`.
3. `Show.cinch:99`, `nat-digit-count`: `nat-div n 10`.

Evidence needed: `NonZero 10`. This folds **statically for free** (`pred_obligation`, section 3): if `nat-div`/`nat-rem` took a `NonZero Nat` divisor, these three sites would need **zero** syntactic change at the literal `10`; the checker's existing numeral-folding already discharges it.

**(c) `Nat` subtraction.**
1. `compiler/prelude/Data/Date.cinch:160`: `mp = if m > 2 then nat-sub m 3 else nat-add m 9`.
2. `Date.cinch:373`, `digits-value-acc`: `nat-sub (char-code c) 48`.
3. `compiler/prelude/Data/Time.cinch:251`: `nat-sub 9 (list-length cs)`, guarded two lines earlier by `if list-length cs > 9 then None else ...` (`Time.cinch:250`).

Evidence needed: `b <= a` for each `a - b`. Site 1 and 3 are guarded by a plain `if`/boolean comparison whose truth would supply exactly the needed fact, but per section 2 that guard gives **no** narrowed evidence to the subtraction; today's machinery cannot express "the `if`'s condition proves the subtraction safe" without rewriting the guard itself as `decide`+`case`. Site 2 is worse: `char-code c` being an ASCII digit (hence `>= 48`) is established by a *different* function entirely (`all-digits`, called by the site's own caller); there is no mechanism today to carry that cross-function invariant as scoped evidence to this call site short of a refined `Char`/digit parameter type.

---

## 6. What is missing

1. **Evidence narrowing from plain `if`/boolean conditions.** `dependent-core-kernel.md` section 8 *designs* this ("if/case introduce facts into scope on their branches"), but the implementation (`check_terms.rs:869-885, 1438-1441`) does not do it: only matching `Dec`'s `Yes`/`No` constructors binds usable evidence. Nearly every existing guard in the prelude (`byte-at`, the `Date.cinch`/`Time.cinch` subtraction guards) is written as a plain `if`/`case` on `Bool`, not `decide`. This is the single highest-leverage gap, since fixing it would let an *existing* guard discharge its matching refinement obligation with no rewrite.
   *Closed 2026-10-30:* every match branch now binds what its test proves (`if`, `case` on a comparison, on a numeric value with literal rows, on `comparing`, on a list's constructors, guards, `&&`/`||`/`not`), discharging `using` obligations and letting a plain value enter a refinement; see `design/dependent-core-kernel.md` section 8 ("As built").
2. **Linear arithmetic over variables.** `i < n` implying `i + 1 <= n` when `n` is a variable (e.g. `bytes-len b`, `list-length cs`) is not decidable by `pred_implies`: `interval_atom`/`bound_value` require a numeral on one side of the comparison (`types.rs:986-1007`). `design/proof-automation.md` section 4 item 2 names this ("omega-like") as needed next; no code implements it. This gates almost every indexing *loop* (section 5a), since the bound advances by a non-literal amount each iteration.
   *Closed 2026-10-30:* the discharge reads every fact in scope together as difference constraints (`x - y <= c` over `Nat`, `Int`, machine integers and `Char`) and refutes them with the obligation's negation by negative-cycle detection, so `i < n` (with `n` a local, `bytes-len b`, `length xs` or any pure measure) discharges `i + 1 <= n`, `1 <= n - i`, `NonZero (n - i)`, `i <= n` and `0 < n`, and bounds chain through other variables. Machine-integer arithmetic stays opaque (a user `+` may wrap). Each proof carries a checked negative-cycle certificate; see `design/refinement-types.md` ("linear facts over variables") and `design/proof-automation.md` section 5. Item 4 (carrying an invariant across a recursion) remains: the procedure reads the facts in scope at each step, and a loop's bound is in scope only where a branch tests it.
3. **Literal discharge for `NonZero`/comparison divisors already works** (section 3, `pred_obligation`), so it is *not* missing for the literal-divisor pattern (section 5b); it *is* missing for a non-literal divisor (`nat-div n d` with `d` an arbitrary expression), which needs a threaded `NonZero` witness or falls back to `Option`-returning division.
4. **Loop-invariant/structural-recursion automation.** `decide` is explicitly "scoped to the examined values... never a general proof" (`dependent-core-kernel.md` section 6); there is no mechanism to prove an index bound once and have it automatically carry forward through a structural recursion's remaining steps (the catalogue's "induction and case analysis" tactic layer, `proof-automation.md` section 4 item 1, is design-only; no tactic/reflection layer exists in `check_terms.rs`). Each iteration must re-derive its own `Dec` by hand.
5. **Compound-predicate refinement *declarations*** (`refine (xs :: T) with p && q`, D256/D257's predicate-indexed-family generalization) are explicitly "implementation queued behind the language-tail slice" / "rides the P28 continuation" (`compiler/CONFORMANCE.md:854-885`), even though `decide` on a *source-level* `&&`/`||` expression already folds via `decide_compound`. The type-level compound-refinement syntax from `design/refinement-types.md`'s `SizedAscii` example is not buildable today.
6. **No `Fin`/refined-index alternative on `Bytes` at all.** `design/no-faults-inventory.md:100` confirms this directly; `Fin`/`Vec` exist only as compiler builtins exercised by tests/examples, never wired to `Bytes`, the one container dominating the fault call-site count.

---

## Recommended migration order

1. **`NonZero` divisor for literal-divisor `nat-div`/`nat-rem`/`int-div`/`int-rem` sites first** (section 5b). The checker already statically folds `NonZero` over numeral literals (`pred_obligation`), so changing these primitives' signatures to take a `NonZero Nat`/`NonZero Int` divisor costs **zero** proof burden at every literal-divisor call site (the hex/decimal rendering family across `Show.cinch`, `Base64.cinch`, etc.). This is the cheapest available win against the densest fault families (170+ sites), with no new language feature required; non-literal-divisor sites fall back to `Option`-returning division.
2. **Add branch narrowing from plain `if`/comparison conditions** before touching `bytes-get`/`bytes-set` or `nat-sub` in earnest. This is the smallest missing *feature* (already specified in `dependent-core-kernel.md` section 8, just unimplemented) and unlocks the largest number of call sites, because nearly every existing prelude guard (`byte-at`, the `Date.cinch`/`Time.cinch` subtraction guards) is a plain `if`, not a `decide`. Without it, migrating `nat-sub` or `bytes-get` forces every one of those guards to be hand-rewritten to `decide`+`case` syntax, multiplying the touched call-site count for no added safety over what the guard already proves informally.
3. **`Nat` subtraction** (`a - b` requiring `b <= a`): once (2) lands, the majority of existing `if (b <= a-implying condition) then nat-sub a b else ...` guards discharge for free. The remaining cross-function-invariant sites (the `char-code c` ASCII-digit pattern in `digits-value-acc`) need either a refined `Char`/digit-range parameter type or the already-named `saturating-sub` escape (`design/no-faults.md` section 7) rather than a proof.
4. **Variable-bound linear arithmetic** (`i < n` implies `i + 1 <= n` for variable `n`) before attempting `bytes-get`/`bytes-set` at scale. This is required by essentially every indexing loop (section 5a) once the index advances past a single `decide` call; `proof-automation.md` names it as the next needed automation (section 4 item 2), but no interval/omega reasoning over two variables exists in `types.rs` today. Without it, migrating `bytes-get`/`bytes-set` compiles (the `Fin`/`decide` machinery can express each step by hand) but at a per-iteration verbosity (`decide` + `case` + `Fin` construction, per buffer, per loop step) that makes a 190+-site mass migration impractical.
5. **A `Fin`/`Vec`-style indexed alternative wired onto `Bytes`** last. This closes the largest fault family (190+ sites, `bytes-get`/`bytes-set`) but should come after (2) and (4): `bytes-get`/`bytes-set` sit inside recursive loops in almost every call site found (`Lex.cinch`, and by extension `Base64.cinch`, `Digest.cinch`, `Json.cinch`, `Regex.cinch`), and without loop-carried narrowing and variable-bound arithmetic the per-site proof burden is one hand-written `decide`+`Fin` construction per call: expressible today (`Fin` is already a compiler builtin, `check_decls.rs:1497-1520`), but at a verbosity that defeats mass migration until (2) and (4) exist.

The order above puts the smallest missing language feature (`if`-condition narrowing) ahead of the headline call-site counts (`bytes-get`/`bytes-set`, variable-divisor `nat-div`), because those counts are gated behind two still-missing primitives, while the literal-`NonZero` win is available immediately against the checker as it stands today.
