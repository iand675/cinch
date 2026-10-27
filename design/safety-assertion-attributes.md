# Safety assertion attributes (the JPL rules at compile time)

Design note for P20. The normative attribute definitions live in
`specifications/attribute-annotations-specification.md` section 5.
No em dashes.

The ask: attributes that assert tail recursion, non-allocation, and
non-stack-growth at compile time, and a mapping of NASA/JPL's "Power of
10" rules (Holzmann) to compile-time checks. The finding that shapes the
design: most of the rules are already unbreakable in this language, so
the work splits into (a) rules the language enforces inherently, (b)
rules enforced by existing mechanisms, and (c) rules needing new
assertion attributes.

## The rule mapping

| JPL rule | Status in Cinch | Mechanism |
| --- | --- | --- |
| 1. Simple control flow; no goto/setjmp; recursion bounded | attribute | no `goto` exists; `#[recursion "none"\|"tail"\|"bounded(n)"]` asserts per definition (see below); `corec` productivity already bounds corecursion |
| 2. Every loop has a fixed upper bound | attribute + types | loops are recursion: `#[recursion "bounded(n)"]` asserts a static depth bound (structural recursion over `Vec`/`Nat` sizes proves it via the totality machinery; otherwise the bound is checked conservatively and the report says how); tail-recursive iteration is bounded by its `Nat`/`Fin` arguments (the bounded-counter pattern) |
| 3. No dynamic memory after initialization | attribute | `#[no-alloc]` and `#[alloc "once"]`: the allocation-effect analysis classifies every construction site (cons, closures, thunks, `Mut`, strings, `Bytes`, foreign aggregates) |
| 4. Small functions, bounded complexity | attribute | `#[max-complexity n]` (cyclomatic: branches + 1 over the body) and `#[max-lines n]` |
| 5. At least two runtime assertions per function | attribute | `#[assert-min n]`: counts explicit runtime `assert`/`decide`-on-runtime-value checks in the body (blunt, but it is exactly the rule); proof obligations do not count (they are compile-time) |
| 6. Restrict the scope of data | inherent | lexical scoping, `%0`/`%1` usage checking, region borrows with escape checks, no mutable globals; dropping is a check error |
| 7. Check return values; validate at boundaries | inherent + attribute | `Result` is a type and discarding one without `ignore` is a check error (new must-use default); `#[checked-params]` asserts every parameter of the definition is validated before use |
| 8. Restrict the preprocessor | attribute | macros are hygienic, token-bounded, and budgeted already; `#[no-macros]` asserts no macro expansion occurred in the definition (checkable from provenance) |
| 9. Restrict pointer use | attribute | `#[no-raw-ptr]` asserts no `RawPtr`/`opaque` in the signature and no foreign pointer operations in the body (they are already offset-checked and deallocator-paired) |
| 10. Warnings as errors; use the analyzers | pragma + attribute | `warn # error` exists per the spec; `#[deny-warnings]` scopes it to a module or definition |

## The assertion attributes (compiler-owned)

`#[recursion "none"|"tail"|"bounded(n)"]`, `#[no-alloc]`,
`#[alloc "once"]`, `#[stack "constant"|"bounded(n)"]`,
`#[max-complexity n]`, `#[max-lines n]`, `#[assert-min n]`,
`#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]`,
`#[deny-warnings]`.

Semantics (D111-D114):

- Each is **compiler-owned and verified at the definition**: a
  violation is a compile error naming the attribute, the rule it
  asserts, and the offending site (a tail-position violation names the
  call; an allocation violation names the constructor).
- Verified assertions are **part of the contract**: recorded in
  `.cinchif` and hashed into `iface_hash` exactly like `repr` facts
  (P17), so consumers may rely on `#[no-alloc]` across a call without
  re-deriving it. A definition that stops satisfying its assertion does
  not compile; an interface change is an interface change.
- `#[recursion "tail"]` covers self and mutual tail recursion within
  the definition's recursive group (D104's compilation); `#[stack
  "constant"]` is its stack-level companion and implies tail or
  non-recursive. `bounded(n)` accepts structural proofs first and a
  conservative call-graph bound otherwise; the report states which.
- **The JPL profile**: `#[profile "jpl"]` on a module (or the
  `jpl` target profile for a whole program) makes the full set
  mandatory (recursion bounded, no-alloc for non-init code, complexity
  and line limits at the rule's numbers (10, 60), assert-min 2 on
  exported functions, no macros, no raw pointers, deny-warnings), and
  reports the whole-program summary. Individual attributes may be
  stricter but never looser than the profile.

## Why this is honest

The dangerous rules of flight software are the ones about behavior the
language cannot see (pointer arithmetic, unbounded loops, hidden
allocation). Cinch removes the first category by construction and
makes the second and third *visible and assertable*: the assertions
compile-time-verified, contract-hashed, and profile-enforced. What
remains genuinely unverifiable (arbitrary numeric bounds on
non-structural recursion) is refused or conservatively bounded, never
silently trusted.
