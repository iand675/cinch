# Inline target IR and `prim` declarations

Implementation design for
`specifications/primitive-operations-and-inline-target-ir-specification.md`
(normative). This note covers the compiler-side representation, the
emission path, and the migration of the existing intrinsic set. No em
dashes.

## 1 What exists today (the thing being externalized)

`core.rs` has a closed `PrimOp` enum: `from_symbol`/`symbol` map
`"cinch.prim.checkedAddI64"` to a variant, `arity()`/`sig()` give the
type, and `codegen.rs` lowers each variant to inline target
instructions (checked arithmetic becomes
`llvm.<sign><op>.with.overflow.iN`, bytes ops become loads/stores, and
so on). The prelude declares them as `foreign import ccall` purely to
give them names and types in the surface language. No call is ever
emitted for a prim: the synthesized intrinsic unit's body is the
instruction sequence.

This is exactly a `prim` declaration with one hardcoded `llvm` body.
The new element generalizes the shape; the lowering strategy is
unchanged.

## 2 AST and pipeline

- `ast::PrimDecl { name, ty, bodies: Vec<PrimBody> }` where
  `PrimBody { family: Name, fragment: Fragment, when: Option<Cond>,
  justify: Option<Term>, uses: Vec<ForeignContract>, span }`.
  `Fragment` is the family's raw token/text payload (kept verbatim for
  round-tripping and provenance).
- Parse: `prim name :: type where` block; body forms
  `family { ... } [when cond] [justify e] [uses ...]`. The fragment is
  delimited by braces with nesting counted at the lexer level (the
  layout pass needs balanced braces; the attribute chapter's `#[...]`
  rule shows the pattern for foreign token content).
- Check: signature is a monomorphic closed-row type (the foreign
  chapter's rule); conditions typecheck against the platform chapter's
  capability evidence (the `Available`/`target rewrite` path, D58/D59);
  `justify` goes through the rewrite chapter's `justify` checker (D55);
  `uses` entries are full foreign binding contracts (ffi#2).
- Tiering: `justified` iff every body has `justify`; else `assumed`.
  The tier and the law/assumption identity feed `interface.rs` per the
  spec's section 5 (justified: bodies are representation, contract =
  signature + law identity; assumed: body identity is contract).
- Registry: `Registry` gains a `prim` table alongside units; each
  prim's synthesized unit is per FAMILY (one unit per selected body),
  chosen at emission from the artifact's target profile. Selection is
  recorded in the manifest (`prims` section: name, family, condition,
  tier, uses).

## 3 Emission

- Family emitters are pluggable: `llvm` reuses the existing builder
  (the body replaces the current hardcoded variant lowering);
  `wasm`/`asm`/`clr`/`jvm` emitters are separate modules that only
  need to splice the fragment into a function body with the declared
  ABI. An implementation without a family emitter rejects that family
  (spec section 3).
- Containment validation (spec section 4) runs per family: the `llvm`
  family parses the fragment with llvm-as-style validation if available
  or a structural scan otherwise; the no-escape rules are structural
  (calls only to `uses` symbols, no allocas beyond declared scratch, no
  invoke/landingpad, no stores outside declared buffers).
- The synthesized unit carries the attribute chapter's hints like any
  function (`#[inline]` family applies; `repr` does not).

## 4 Migration of the intrinsic set

- Phase 1 (this phase): `PrimOp` stays; `prim` declarations compile to
  the same unit shape. The intrinsic namespaces stay reserved (D73).
- Phase 2 (optional, Open decision): rewrite the prelude's
  `foreign import ccall "cinch.prim.*"` declarations as `prim`
  declarations with `llvm` bodies reproducing today's lowering, then
  delete the `PrimOp` variants one family at a time. The compiler's
  set becomes literally the user's set; `PrimOp` shrinks to whatever
  the backend truly needs to know (overflow intrinsics with target
  semantics may stay compiler-provided if a family cannot express
  them).

## 5 Identity rules (the sharp edge)

- Justified prim: body is representation. Swapping the `llvm` body for
  a better one (or adding a `wasm` body) does NOT change `iface_hash`.
  The law identity in `assumptions.declared` is the contract (the D55
  pattern).
- Assumed prim: the body's content hash enters the contract identity.
  Editing the body stales dependents (the representation-only rule does
  NOT apply; nothing proves the edit preserves meaning).
- Selection (which body ran) is never contract, always manifest (the
  supercompilation chapter's split).

## 6 Open implementation questions

- Whether the fragment for `llvm` is parsed at check time (needs
  llvm-as or an IR parser dependency) or validated structurally at
  check and fully at emission (this design: structural at check,
  authoritative at emission, both reported).
- How `uses` contracts interact with the interface foreign table
  (this design: they are recorded as foreign dependencies of the prim,
  so a prim's ABI surface is fully visible in `.cinchif`).
- Whether prim bodies may contain `select`/`phi`-style control flow
  (this design: structured straight-line plus the family's own branch
  forms; no back-edges, so prim bodies stay total-by-construction).
