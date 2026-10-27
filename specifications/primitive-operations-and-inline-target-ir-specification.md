# Primitive operations and inline target IR specification

Source: local draft in this repository (added after the documentation export)
Added: 2026-10-26

---

[Documentation overview](../overview.md)

Draft 0.1. This chapter introduces `prim` declarations: a language
element for defining primitive operations with inline target-IR bodies,
replacing the closed compiler-known intrinsic set as the extension
mechanism. It defines the declaration form, the per-target body
registry, the selection rules (which reuse the platform chapter's
capability evidence), the justification and assumption split (which
reuses the rewrite chapter's `justify` discipline), the validation
rules for embedded IR, and the identity rules for compiled interfaces.
MUST denotes a requirement of this draft. Proposed choices are
identified explicitly; unresolved items are listed in Open decisions.

# 1 Motivation and scope

The compiler today carries a closed set of primitive operations
(`checkedAddI64`, `bytesGet`, and the rest) known to the compiler by
name, declared in the prelude as `foreign import ccall
"cinch.prim.*"`, and lowered by the code generator to inline target
instructions (for example `checkedAddI64` lowers to
`llvm.sadd.with.overflow.i64`; no call is emitted). Two limits follow:
the set can only grow by editing the compiler, and each operation's
target lowering is hardcoded to one IR family.

This chapter opens both. A `prim` declaration names an operation, its
type, and one or more target-IR bodies written inline in the source.
The compiler validates and emits the bodies like it emits the current
intrinsics. Arithmetic and similar operations then need no foreign call
surface at all.

Out of scope: real external C symbols remain `foreign import ccall`
(foreign function interface specification); a `prim` body is an
inlined definition, not a call out. Rewrite rules remain the mechanism
for transforming programs (modules specification section 5); `prim`
defines operations.

# 2 `prim` declarations

```
prim_decl  ::= "prim" name "::" type "where" INDENT { target_body } DEDENT
target_body ::= target_ir "{" fragment "}" [ "when" cond ] [ "justify" expr ]
target_ir  ::= "llvm" | "wasm" | "asm" | "clr" | "jvm" | name
```

- The type is an ordinary monomorphic function type with a closed
  explicit effect row, exactly as the foreign chapter requires of
  imports (effect honesty applies; most prim bodies are pure).
- At least one `target_body` MUST be present. Each names a target-IR
  family and carries that family's inline fragment.
- `when cond` scopes the body to a target capability condition (the
  platform chapter's validity scopes; the same conditions `target
  rewrite` accepts). A body without `when` is the baseline body.
- `justify expr` provides evidence that the body implements the
  declared type's mathematical meaning (section 5).

A `prim` name is a module-level value like any other: it can be
exported, imported, and used at ordinary types. It is not a keyword
form in expressions.

# 3 Target-IR families

A family is a named fragment grammar with validation rules and an
emission path. This draft registers:

| Family | Fragment | Validation (section 4 profile) |
| --- | --- | --- |
| `llvm` | LLVM IR text for one function body over the declared ABI | structural (this chapter) + LLVM parse |
| `wasm` | WebAssembly text for one function body | structural + Wasm parse (webassembly specification) |
| `asm` | native assembly for the declared target | structural only (future; no round-trip check) |
| `clr` | .NET CIL for one method body | structural (future) |
| `jvm` | Java bytecode or Jasmin text for one method body | structural (future) |

Implementations MUST reject a family they cannot emit (the
capability rule of the webassembly specification section 2: reject a
missing required capability or select a supported lowering). An
implementation MUST support `llvm`; the others are OPTIONAL until their
backends exist. A `prim` with bodies for families the build does not
use is fine: selection (section 6) picks per artifact.

Target-IR families are part of the target profile: a profile names the
families it can consume and the feature/capability evidence each body
requires.

# 4 Validation of inline bodies

An inline body is trusted code, so validation is about containment, not
about proving semantics:

- **Signature discipline**: the body receives exactly the declared
  arguments in the declared ABI (the binding contract of the foreign
  chapter section 2: widths, signedness, convention) and returns the
  declared result shape. The fragment MUST NOT declare extra parameters
  or results.
- **No escape**: the body MUST NOT contain jumps, calls, or references
  to symbols outside an explicit `uses` list attached to the family
  body (which is itself a binding contract, recorded like a foreign
  import). No allocation, no I/O, no task operations, and no fault
  paths beyond a declared failure mode. In particular a `prim` body
  MUST NOT smuggle a foreign call that would bypass the foreign
  chapter's contracts.
- **No unwind**: panics or exceptions MUST NOT leave the body (the
  foreign chapter's rule applies unchanged; the defined failure mode is
  a declared fault).
- **Determinism of effects**: the declared effect row is the body's
  entire permitted effect; anything the body does beyond pure
  computation MUST be visible in that row.
- **Grammar**: each family's fragment MUST parse under that family's
  parser (an implementation MAY reuse its own backend's parser, as the
  compiler reuses LLVM's text form).

An implementation MUST reject a body failing any of these rules, naming
the rule. It MUST NOT "best effort" an invalid body into code.

# 5 Justification and assumptions

Two tiers, mirroring the rewrite chapter's `justify` discipline:

- **Justified bodies** carry `justify expr` proving that the operation
  meets the mathematical meaning of its type (for example that
  `addU32` is addition modulo 2^32 in the primitive chapter's model).
  The evidence may be kernel evidence or a law method of a `signature`
  (the platform chapter's instruction laws), recorded under the same
  rules as rewrite justification. A justified `prim`'s contract identity
  is its signature plus the law identity: bodies MAY be replaced
  without staling dependents, exactly the rule the supercompilation
  chapter gives specialized variants (representation-only).
- **Assumed bodies** have no justification. Their identity includes the
  assumption (like an undischarged rewrite law): changing the body of
  an assumed `prim` changes the artifact's assumption identity and MUST
  stale dependents. Every assumed `prim` MUST appear in the
  transitive assumption manifest (core specification section 11's
  manifest MUST), tagged with its target family.

Tier selection is per `prim`, not per body: a `prim` with one justified
body and one assumed body is assumed overall (the weakest tier governs
the contract), and the report says so.

# 6 Selection

- A body applies when its target-IR family is the artifact's emission
  family AND its `when` condition holds in the target profile
  (capability evidence, scoped lexically exactly like `target
  rewrite`, decision D59's rule).
- The baseline body (no `when`) MUST exist whenever more specific
  bodies exist; a `prim` whose only bodies are conditional MUST be
  rejected at compile time when no condition matches, naming the
  missing capability (the webassembly chapter's reject-or-select rule).
- Selection is fixed at compile time per artifact. It MUST NOT appear
  in the compiled interface's contract projection (it is
  representation), and it MUST appear in the artifact manifest
  (which body was chosen and why), per the supercompilation chapter's
  reporting rules.
- Two bodies with overlapping conditions in one family are an error
  (the first-match ambiguity the attribute chapter's duplicate rule
  avoids).

# 7 Relationship to existing machinery

- **The intrinsic namespaces stay reserved** (`cinch.prim.*`,
  `cinch.runtime.*`, decision D73). The existing intrinsic
  declarations elaborate to compiler-provided `prim` definitions; a
  user `prim` MUST NOT claim those names. Migration of the intrinsic
  set to `prim` declarations in the prelude is an implementation
  choice (Open decisions).
- **`foreign import ccall`** remains the call-out mechanism with the
  foreign chapter's contracts; `prim` is an inline definition. A `prim`
  body MAY call out only through its `uses` list, which is itself
  subject to the foreign chapter's rules.
- **Rewrites and optimization**: a `prim` is opaque to the driver's
  unfolding (it is a leaf, like a foreign), but its body is ordinary
  target code and MAY be optimized by the backend. `justify`-proven
  prims MAY participate in rewrite laws like any function.
- **Attributes**: `prim` declarations accept the attribute chapter's
  declarations positions; hint attributes (`#[inline]` family, weights)
  apply to the synthesized unit; semantic attributes (`repr`) do not
  apply (the ABI comes from the declared type and the family contract).
- **Macros**: a macro MAY generate `prim` declarations; macro-emitted
  bodies revalidate under section 4 like any generated declaration.

# 8 Reporting

The compiler report and artifact manifest MUST record, per `prim`: the
selected body's family and condition, its tier (justified or assumed),
its `uses` list, and its validation result. Secrets and absolute paths
MUST NOT appear in bodies or reports (the staging rule applies).

# 9 Worked example

```
module Arithmetic where

  prim addU32 :: U32 -> U32 -> U32 where
    llvm {
      %s = add i32 %0, %1
      ret i32 %s
    } when Available VecU32Target
    llvm {
      %s = add i32 %0, %1
      ret i32 %s
    }
    wasm {
      local.get 0
      local.get 1
      i32.add
    } justify addU32Law

  signature Wrap32 where
    addU32Law :: Eq (addU32 x y) (wrap32Add x y)
```

The `llvm` body under `Available VecU32Target` and the baseline
`llvm` body are distinguished by condition; the `wasm` body is
justified by the `Wrap32` law method, so `addU32`'s contract is the
law and bodies may vary per target without staling dependents.

# 10 Open decisions

- Whether the existing intrinsic families migrate to prelude `prim`
  declarations (making the compiler's set literally the user's set) or
  remain compiler-provided with `prim` as the user extension path.
- The exact fragment grammars for `asm`/`clr`/`jvm` (this draft fixes
  only the containment rules).
- Whether a justified `prim` may be used in `spec`/`proof` positions by
  computation (this draft says no: bodies are executable code, never
  evidence, the supercompilation chapter's rule).
- Whether `uses` entries require full foreign binding contracts
  (this draft says yes) or admit a lighter intrinsic-to-intrinsic call
  form.
- The failure-mode vocabulary for bodies that can fail (a declared
  fault versus the `Option`-returning checked style of the primitive
  chapter).

# 11 Implementation status

Implemented (P18, `887d568`, `tests/prim.rs` 10/10 and `tests/laws.rs`
4/4): `prim` declarations with per-family inline bodies (llvm/asm;
wasm registered; clr/jvm reserved), verbatim fragment capture,
containment validation naming its violation classes, capability-scoped
selection recorded manifest-only, justify/assumed tiers with law and
assumption identity (`prim:unit#<bodyhash>`), the intrinsic-family
migration path, and class `law` declarations as citable justify axioms
with sample-wise instance checking. The design note
`design/inline-target-ir.md` remains the implementation reference.
Section 12's effect-purity treatment and the `unsafe` keyword are the
queued continuation (2026-10-27, user direction).

# 12 Effect purity and unsafe

A prim's effect row decides its treatment, exactly as for ordinary
functions (2026-10-27, user direction):

- A prim with an **empty effect row is proven pure by containment**:
  its fragment carries no stores, atomics, volatile accesses, or
  `uses` calls (section 4), so its calls do not invalidate evidence,
  do not defeat `memory(none)`/`nofree` in the code-generation scan,
  and may be `speculatable`.
- A prim whose row carries `Io` or a write effect is effectful per its
  row: its calls take part in evidence invalidation and the attribute
  scan exactly like any function with that row.
- **`unsafe`** marks a body whose behavior containment cannot verify
  (`asm` templates, `uses` into opaque code). The declared row is
  TRUSTED rather than verified, and the trust is recorded: the
  `prim:unit#<bodyhash>` assumption identity (body edits stale
  dependents through `iface_hash`) and the manifest trust rows. An
  `unsafe` prim claiming purity is a visible assumption in
  `assumptions.declared`, never an invisible claim; scan-gated
  attributes (`willreturn`, `nofree`) remain gated on the body scan
  (trust is recorded, the claim is not free).

Nothing is silently trusted: a prim's behavior is either verified by
containment or declared under `unsafe` and recorded as an assumption.

# 13 Platform-gated bodies

Body conditions gain platform predicates keyed on the target
profile's triple: `when arch x86_64`, `when arch aarch64`,
`when arch wasm32`, and other architecture names. The selection rules are
unchanged: the matching conditional wins, an unconditional body is the
baseline fallback, overlapping conditions in one family error, and a
conditional-only prim whose conditions do not match rejects naming the
missing platform. Selection remains manifest-only, with the platform
recorded in the body row's reason. The canonical shape is a portable
`llvm` baseline plus per-architecture `asm` bodies (for example
`popcnt` on x86_64 versus `cnt` on aarch64). Architecture names come
from the target profile's triple, the same target identity that keys
cross-compilation widths. `os` and pointer-width predicates may follow
if a consumer needs them.
