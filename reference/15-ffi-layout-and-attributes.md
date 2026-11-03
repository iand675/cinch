# Foreign code, representation attributes, and inline primitives

A foreign or inline-IR declaration crosses a trust boundary. The compiler checks containment, ABI representation, effects, and resource roles; these checks do not prove arbitrary external code's semantic truth.

# C imports and exports

`foreign import ccall "symbol" local-name :: Type` binds an external C symbol. `foreign export ccall "symbol" local-name :: Type` exposes a checked definition. The contract includes convention, widths, signedness, layouts, ownership roles, and effect row. `opaque type` and `RawPtr` represent admitted opaque boundaries with explicit lifetime/deallocation obligations.

Do not use bignum-capable `Int`/`Nat` as though they were C machine integers. Use the admitted machine scalar or C vocabulary and checked conversions. Integer-to-pointer ownership roles are rejected; an owned foreign allocation needs its required deallocator contract. Panics/exceptions must not unwind across the boundary. Blocking work retains its blocking/effect policy.

The Rust compiler APIs can link supplied objects (as in `run_source_objects`). The public CLI does not currently offer an arbitrary object/library option. A `foreign import` declaration can check and still fail at link time if the symbol is absent; document that dependency rather than presenting it as a standalone runnable program.

# Attributes

The [complete attribute and pragma index](23-attributes-and-pragmas.md) lists every accepted name, spelling, argument, placement, and conflict. [Type layout](24-type-layout.md) gives worked offsets, alignment, packing, wrappers, and current versus planned representation behaviour.

`#[...]` attaches to supported declarations, fields, constructors, clauses, alternatives, guards, and `if` positions. Duplicate/conflicting attributes and invalid placements are rejected. Semantic attributes affect contracts where required; optimization hints are representation-only.

| Family | Current meaning |
| --- | --- |
| `#[inline]`, `#[inline(always)]`, `#[inline(never)]`, `#[noinline]` | LLVM inlining hints/constraints. |
| `#[cold]` | Function cold hint or admitted branch-weight wrapper, depending on placement. |
| `#[weight(n)]`, `#[likely]`, `#[unlikely]` | Branch likelihood metadata with validated all-or-nothing decision weights. |
| `#[repr(C)]` | Source-order non-erased fields with C layout/padding facts. |
| `#[repr(transparent)]` | Admitted single-representation-field layout. |
| `#[repr(align(N))]` | Raise alignment to a valid power of two. |
| `#[repr(packed(N))]` | Packed layout; aligned borrows of unaligned fields are refused. |
| Discriminant representations | Recorded admitted integer tag representation. |
| Safety/LLVM contract attributes | Require proved facts or explicit admitted unsafe assumptions; see below. |

Partial decision weights are dropped with `incomplete-branch-weights`; incoherent transformed weights are reported. An unlikely branch still needs correct types, ownership, and coverage. Unknown attribute names outside the metadata/registered macro vocabulary are rejected.

# Safety and LLVM contracts

The backend derives LLVM range, nonnull, alignment, capture, memory-effect, and return/argument facts from established analyses. These LLVM names are generated output, not a general source attribute syntax. User safety assertions have their own compiler-owned vocabulary; [the complete assertion and LLVM reference](22-safety-assertions-and-llvm.md) lists it. Inline opaque code records the explicit assumptions admitted by its grammar. A requested assertion that cannot be proved is not silently upgraded into a theorem. [Safety attributes](../design/safety-assertion-attributes.md) and [LLVM contracts](../design/llvm-contract-attributes.md) give the complete design vocabulary and placement rules; current tests observe their presence and absence in actual emitted IR.

# Inline primitives

A primitive has an ordinary type and per-family bodies:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prim add-word :: U32 -> U32 -> U32 where
    llvm {
      %sum = add i32 %0, %1
      ret i32 %sum
    }

  main :: Int
  main = if add-word 40 2 == 42 then 0 else 1
```

Bodies can carry `when` feature conditions, `justify`, immediate-argument annotations, and explicit `uses` contracts in admitted forms. Containment rejects undeclared escape/calls, extra ABI parameters, unwinding, and hidden effects. `unsafe` bodies record assumptions. The selected body is an artifact fact; justified representation selection does not unnecessarily change contract identity.

<span class="support-tag support-tag--planned">Coming soon</span> **additional backend families.** LLVM and admitted native assembly have current lowering paths. CLR/JVM families are reserved, and registering their names does not provide executable CLR/JVM backends. See [target profiles](19-targets-and-components.md) for the Wasm subset and CLI limits.

Implementation: `repr.rs`, `check_decls.rs`, `codegen.rs`, `interface.rs`; suites: `ffi.rs`, `attributes.rs`, `prim.rs`, `safety.rs`, `llvm_contract.rs`, `repr_classes.rs`.
