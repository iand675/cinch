# Compiled interface metadata

Design note for the Cinch toolchain. Companion to
[Modules and compilation specification](../specifications/modules-and-compilation-specification.md);
where that chapter states obligations, this note fixes the mechanism.

Status: design accepted for implementation (roadmap slice P3 in
`compiler/CONFORMANCE.md`).

# 1 Why a separate artifact

Separate compilation needs one question answered per import: what does this
module expose such that an importer can be checked, resolved, and linked
without reading its source? The answer must cover five consumers at once:

| Consumer | Needs |
| --- | --- |
| Type checker | Exported schemes (with effect rows, `using` contexts, quantifiers), types, constructors, records, class signatures |
| Fixity pass | Operator precedences and associativity for exported operators |
| Dictionary resolution | Which instances exist, for coherence across modules |
| Code generator and linker | Symbol names, arities, calling convention identity |
| Trust reporting | Primitive axioms, foreign contracts, declared trusted assumptions (core specification, section 11) |

Source text is not an interface: it is larger, it changes with comments and
layout, and it lets a recompilation cascade where an interface would not.

# 2 The artifact

Each compiled module produces a pair:

```
build/<Module>.o         native object (LLVM IR compiled by clang)
build/<Module>.cinchif   compiled interface metadata
```

and the toolchain additionally links the Rust runtime `libcinchruntime.a`
(runtime is Rust; see `compiler/runtime/src/lib.rs` for the runtime ABI
reference).

`.cinchif` is canonical JSON: UTF-8, keys in the fixed order below, one
canonical spelling for every type and row, no timestamps. Its content hash is
the module's **interface identity** (`iface_hash`); rebuilds that change
nothing observable produce the same identity.

## 2.1 Content model

```text
schema          interface format version (integer)
language        language draft this artifact conforms to
runtime_abi     runtime ABI version (must match libcinchruntime.a)
compiler        compiler version that produced it
module          module name
source_hash     hash of the canonical source text
imports         [ { module, iface_hash } ] for each import consumed
exports
  values        name -> scheme (quantifiers: type, row, region; using
                context; type with effect rows), plus symbol, arity
  types         name -> { params, kind: data|record|opaque, constructors:
                [{ name, tag, fields: [type], erased: [bool] }],
                fields: [name] (records, declaration order) }
  signatures    name -> { params, methods: [{ name, scheme }] }
  instances     [{ id, class, target, ctx, methods: [symbol] }]
  fixities      operator -> { assoc, prec }
  foreign       [{ symbol, abi, external, type, effects, notes }] (FFI
                contracts; see the FFI specification for their obligations)
assumptions
  primitives    [ "cinch.prim.*" ] used by this module's code
  foreign       external symbols relied on (the trusted boundary entries)
  declared      trusted assumptions named by policy, if any
abi
  symbols       exported name -> linker symbol (the compiler's quoted
                `<module>.<name>` convention)
  convention    "uniform-word, curried apply stub" (see core.rs)
```

`assumptions.primitives` is the acceptance manifest's primitive-axiom list:
`Nat`/`Int` semantics, machine arithmetic and memo/thunk machinery are the
trusted kernel of the artifact; everything else is checked against them.

## 2.2 Identity and invalidation

- `source_hash` identifies the input; `iface_hash` identifies the output
  contract. A module recompiles when its own source hash changes; its
  dependents recompile only when `iface_hash` changes. The identity is
  therefore computed over the contract with `source_hash` and `compiler`
  excluded: a body-only edit or a compiler point release does not invalidate
  dependents. (Review finding, fixed: the first implementation hashed them
  in, which would have made every body edit stale the world.)
- `imports` records the exact interface identities a module checked against,
  so a build can prove that every consumer was checked against the interfaces
  actually linked (the "checked against" chain, an assumption-manifest
  requirement).

# 3 Consumption rules

1. **Importers read interfaces, not sources.** The driver compiles each
   module to `.o` + `.cinchif`; checking a module loads the `.cinchif` of
   every import (recursively through `imports` for transitive names).
2. **Name visibility** follows the language rules: a module's own names,
   then imported modules' exports (unqualified and `M.x` qualified).
3. **Fixity merge**: the importing module's own fixity declarations shadow
   imported ones for its own code, matching the current parser behavior;
   the merged table is what its interface records for its own operators only.
4. **Instance coherence**: two `instances` entries across the closed artifact
   set whose `(class, target)` unifies are a link-time error ("ambiguous
   instance"). Instances are unrestricted in where they may be declared for
   now (no orphan rule); global uniqueness at admission is the coherence
   guarantee. A later slice may add an orphan rule; if so it replaces this
   paragraph in both the interface schema and the modules specification.
5. **Dictionary identity** is per instance id: generated code refers to
   `"<module>.<instance>.mk"` construction units; importers reference the
   symbol recorded in `abi.symbols` rather than recomputing names.
6. **Linking** produces one binary from the module objects plus
   `libcinchruntime.a`. The final artifact records the union of
   `assumptions` across its modules and the `runtime_abi` it linked against
   (an artifact manifest, per core specification section 11).

# 4 Runtime and ABI coupling

- Value representation (uniform words, boxed ADTs, closure records with the
  curried apply stub) is defined in `compiler/src/core.rs` and mirrored by the
  runtime crate's module docs. The interface's `abi.convention` names this
  pairing; a change to either bumps `runtime_abi`.
- Symbols are the compiler's quoted `<module>.<name>` identifiers; interface
  `abi.symbols` is the authority for what a linker should resolve, so codegen
  and metadata cannot drift silently.

# 5 Evolution hooks

Later slices extend the same artifact additively:

- **FFI**: `exports.foreign` gains layout and ownership-role fields (the FFI
  specification's binding obligations) and `assumptions.foreign` gains the
  trust profile of each external library.
- **Upgrade packages**: `upgrade` declarations add record-view and migration
  metadata (field maps, schema compatibility evidence) so a replacement
  module can be checked against its predecessor's interface.
- **Rewrites and optimization**: `exports.rewrites` lists optimizer rules with
  their justification symbols; cross-module inlining adds optional `bodies`
  payloads keyed by `iface_hash` so inlining cannot outlive its evidence.
- **Proofs**: `exports.proofs` records checked evidence identities and the
  kernel version that checked them (core specification section 11).

# 6 Rejected alternatives

- **Shipping sources as interfaces**: rejected above: rebuild cascades and no
  stable identity.
- **A single program-wide metadata file**: rejected: loses per-module
  identity, forces whole-program relinking of metadata, and cannot express
  "checked against exactly these interfaces".
- **Binary-only metadata**: rejected for now: canonical JSON keeps the trust
  story reviewable by a human; a binary form can be added behind the same
  schema version later.
