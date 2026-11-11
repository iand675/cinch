# Compiled interface metadata

Design note for the Cinch toolchain. Companion to
[Modules and compilation specification](../specifications/modules-and-compilation-specification.md);
where that chapter states obligations, this note fixes the mechanism.

Status: design accepted for implementation (roadmap slice P3 in
`compiler/CONFORMANCE.md`).

Rust and Cinch read and write interface schema 10 with runtime ABI 6. This
schema carries checked class laws and method defaults, closed executable
total-body payloads and complete defining measure declarations. Rebuild
interface artifacts and dependent objects from source when changing schema.
The bootstrap uses a separately pinned Rust library producer for its
base-library interfaces. Complete matching suites, original API parity and
Acceptance 5 remain open.

A value's `total_body` is null or a version1 closed, sorted first-order graph.
Only a supported monomorphic ordinary pure signature can advertise one. The
consumer validates the exact defining root, domain/result carriers, arity,
lexical scope, dependency closure and acyclicity. Supported numeric result
refinements must follow from independently derived body bounds. A pure scheme
alone supplies no executable semantics.

A value with the checked keyed ordering body may instead carry
`keyed_ordering` with its `less_slot` and `greater_slot`. The producer reads
both actual projections from the retained body and checks the two result
branches and the equality fallthrough. Both projections use the same `Ord`
argument and the same key application to each operand. Consumers require
the defining value contract, the exact pure keyed signature and a checked
`Ord` declaration in the dependency closure. The record describes how the
helper chooses an `Ordering` constructor. Numeric facts also require
executable comparison bodies for both slots of the dictionary selected at
that call. An opaque dictionary grants no branch facts.

ABI-checked primitive values also retain their own defining roots when calls
have been inlined into ordinary Core. Admission follows the actual `PrimOp`
and complete callable declaration, including purity and absence of hidden
arguments; a source name supplies no semantics. Renamed exports retain the
original identity. Unknown foreign calls, wrapping machine arithmetic and
partial/refined signatures remain outside this plain first-order tier. See
[the primitive-value receipt](../bootstrap/current-native-primitive-body-results.json).

A resolved scheme may carry `measures`, each with an `identity`, complete
`resolved` defining scheme and `total_body`. These private declarations grant no
source names. First-tier measure schemes cannot themselves reference further
measure schemes; executable body dependencies remain closed in their graph.
Consumers check declaring modules, checked import closure and agreement of every
shared body dependency across roots. The payload affects interface identity,
so changing a defining body invalidates its semantic consumers.

A closed unrestricted instance may additionally carry `method_bodies`. Each
row associates an actual dictionary `slot` and `implementation` identity with
its complete plain `resolved` declaration and exact closed `total_body`
dependency graph. The declaration must match the graph's parameter/result
carriers, and the implementation must match that dictionary's method table.
The dictionary and implementation belong to the declaring module, including
when distinct module names share a prefix. All graph dependencies belong to
the checked import closure, and shared definitions must agree across roots.
These records grant no public value or method names. Omit the field when there
are no method bodies. Conditional, generic, recursive, effectful and partial
implementations remain outside this first closed-body tier.

Each entry in `method_contracts` carries a `checked` declaration for the
conditional implementation. A producer emits an entry only when it can
publish that declaration. A checked declaration may also carry a guarded
`body`. Omit `body` when the guarded implementation is unavailable.

The Native producer and decoder pass all ten affected complete groups and all
43 published compiler-source modules; see
[the method-body receipt](../bootstrap/current-native-method-body-results.json).
The corresponding Cinch loader, exporter and actual-body proof consumer remain
a private candidate under validation. That candidate requires its own complete
gates and a fresh bootstrap before replacing the published compiler.

The Rust checker derives arithmetic and comparison facts from the actual
selected implementation. Source reification and independent Core typing retain
actual global association and argument identities. Full proof/usage/totality,
general typed runtime lowering and complete self-hosted API parity remain
required. Definition-owned fixities remain part of the exported contract;
importers cannot override them.

Earlier format history follows. Schema 1 omitted signature proof
preconditions and named refinements. Schema 2 added those facts but omitted
declaration parameter kinds, constructor result schemes, and precise method
visibility. Schema 3 records these, defining identities, constructor symbols
and implicit declaration binders. Schema 4 adds definition-site resolved pure
schemes, private nominal kinds and complete constructor identity sets.
Schema 5 adds definition-site pure class contexts, resolved method schemes,
explicit dictionary slots and portable pure instance heads. Richer schemes
retain an explicit null resolved contract until their
evidence has been ported. Older artifacts must be rebuilt.
Higher-kinded function parameters retain their resolved kind annotations.
The driver still checks dependency sources: contextual type-name resolution,
the remaining rich signature dependencies, coherence and ABI consumption remain work
for the standalone interface checker. This format update does not enable
interface-only checking by itself.

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
  values        name -> scheme (quantifiers: type, value, proposition, row,
                region; using class constraints and proof preconditions;
                type with effect rows), plus symbol, arity, resolved, total_body
  types         name -> { identity, params,
                parameters: [{ name, kind, implicit }],
                kind: data|record|refinement|opaque|signature, constructors:
                [{ name, symbol, scheme, resolved, tag, fields: [type], erased: [bool] }],
                fields: [name] (records, declaration order),
                refinement: null | { binder, carrier, predicate } }
  signatures    name -> { identity, exports: [public name], params,
                parameters: [{ name, kind, implicit }],
                methods: [{ name, slot, exports: [public name], scheme,
                resolved, default? }], laws }
  instances     [{ id, class, target, ctx, resolved, methods: [symbol],
                method_contracts: [{ slot, name, scheme,
                checked: { schema, head: { resolved },
                implementation: { identity, resolved, body? } } }],
                method_bodies?: [{ slot, implementation, resolved, total_body }], implicit }]
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

`identity` and constructor `symbol` name the defining module even through
renamed re-exports. Constructor schemes retain their result indices and
constructor-local quantifiers; field types alone cannot reconstruct those
contracts. `parameters` preserves declaration order and explicit kinds,
including a type with no constructors from which to infer parameter names.
The older `params` name list must agree with it.

A signature's `exports` list binds its public class names; a method's list
binds its public method names. An empty list exposes no name. A public
method still carries its private class's contract, and a public class still
carries its private methods' contracts. These retained definitions do not
make their private names available in an importer's source scope.
A value requiring a private class also carries that class's complete
method contract. Method slots are contiguous in declaration order; sorting
public spellings cannot change dictionary layout. Re-exported class
contracts must agree with the defining interface after removing public
aliases and export lists. Unit aliases retain one defining class contract.

Each method default is an optional `default` array containing its checked
patterns and bodies. The interface hash covers the array and its method
scheme. The consumer rechecks the clauses before inheriting them. Both
compilers require schema 10 and reject fingerprint-only defaults. The
library producer rebuilds its interfaces from pinned source.

A Cinch-built consumer inherited defaults from separate Rust and Cinch
producers. See [the inheritance replay](../bootstrap/current-schema9-default-replay-results.json).

### Resolved type occurrences

The display `scheme` is retained for diagnostics and richer consumers.
Its short nominal names cannot establish definition identity. Schema 4's
`resolved` field is either null or `{ scheme, nominals }`. The pure scheme
uses a fully qualified identity at every nominal occurrence; binders retain
their checked kinds and implicitness. It is captured from source lookup in
`check_interface.rs` while the definition's scope is available, before the
legacy `Ty::Adt(info.name, ...)` drops the module. A facade's exports or a
global search for a unique short name never reconstruct it.

Each nominal entry carries `identity`, `kind`, `params`, `parameters` and
`constructors`. The latter is the complete defining constructor identity
list for data, including private constructors, or null for opaque types
and signature dictionary types. A signature descriptor's parameter kinds
must agree with its defining class contract. Its private method schemes
retain definition-site references too; method-local higher kinds are
collected within that method's scope. Inherited and own forall binders
are hoisted together. The implicit class dictionary precedes the method's
own dictionaries, with an internal binder chosen to avoid source-name
collisions. A slot identifies a dictionary projection, not a standalone
linker function.
These private contracts grant no source names or field access. A private
constructor omitted from an importing scope still prevents a match on the
public constructors from claiming exhaustiveness. A wildcard can cover it.
Nominal dependencies record their checked interface hashes even when the
embedded family obtained their names from earlier kernel declarations.
The Cinch loader verifies reachability through those dependencies and
rejects conflicting descriptions of one defining identity.

The schema-5 consumer qualifies pure `using` constraints by those defining
signature identities and rejects duplicate dictionary binders or data
types supplied as classes. This supplies contracts for the dictionary
elaborator. The Cinch frontend resolves lexical dictionary givens and
retains checked method slots in Core; its runtime-input regressions pass.
Imported value ABI arity counts the source arrows plus dictionary
requirements for these pure schemes. Dependent implicit value parameters
are still unsupported. Source class/instance declarations, dictionary
construction and global instance selection remain outstanding. Effects, capabilities and dependent proof
contracts remain explicit unresolved cases. Constructor-local forall
schemes also remain unresolved; imported constructors must have regular
family results with exactly their declaration's parameter kinds, so an
existential witness cannot become a fresh universal at a pattern.

A portable pure instance's resolved scheme has its dictionary type as the
result, with declaration parameters and sub-dictionary constraints retained.
The head and context use their defining class/type identities. Instance
heads are captured in the declaring scope after the program's names have
been collected; ordinary display targets never recover those identities.
Its dictionary constructor remains identified by the instance's `id`.

Per-method instance preconditions retain their checked signatures with
positional argument binders in `method_contracts`. Such an instance has
`resolved: null` until dependent checking is implemented. In particular,
natural-number subtraction's `arg1 <= arg0` obligation must not become an
unrestricted `Sub Nat` dictionary. Module-parameter and other unsupported
instance contracts also remain unresolved. Their raw contracts are retained;
null never permits dictionary selection from the display target.

The shared base family has forward instance dependencies, including cycles
through `Text.Codec` and its carrier modules. The current acyclic per-module
hash protocol cannot publish these as standalone resolved contracts.
Publication therefore requires every resolved nominal/class owner to be
the current module or an already emitted checked interface. A complete
family contract mechanism is still required for forward instances; the
current continuation does not establish that support. The declaration's
proof preconditions remain present even when its head cannot be published.

The Cinch consumer checks the resolved spelling and kinds before accepting
pure values and regular constructors. A null contract retains the richer
scheme's explicit refusal; it does not permit guessing a nominal identity
from display text. Dictionary, effect, dependent, record and existential
contracts still need full elaboration. The legacy Rust checker's internal
short-name representation remains a separate type-identity limitation;
this metadata extension preserves source identities rather than changing
that machinery.

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
3. **Fixity ownership** (user direction, 2026-10-30): only the module
   declaring an operator may declare its fixity. Importers and facades
   inherit that fact with the binding and cannot redeclare or override it,
   even with an identical declaration. A newly declared operator is a
   different binding and may have its own fixity. Without one it is usable
   in prefix form only. Selections, aliases and re-exports carry the
   defining operator's fact; private parsing context is never exported.
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

## 4.1 Linkage identity spelling

A linkage identity carries one spelling, and that spelling is normative
(modules specification, linkage identity spelling amendment). It is what
`i.`, `m.` and `o.` globals use, what an object's symbol table holds and what
a stack frame prints; the interface's own `identity` and `symbol` fields keep
the source spelling, and an imported external's defining identity and every
default-clause global cross into the encoded spelling through
`interface-global`. `[A-Za-z0-9_]` is
itself, `$` is `$24`, and every other scalar is `$` followed by the
lowercase two-digit hex of each of its UTF-8 bytes:

| identity | spelling |
| --- | --- |
| `Data.Core.Nil` | `i.Data$2eCore$2eNil` |
| `Data.FromList.FromList.from-list` | `i.Data$2eFromList$2eFromList$2efrom$2dlist` |
| `Bootstrap.Infer.Bootstrap.Infer.add-error` | `i.Bootstrap$2eInfer$2eBootstrap$2eInfer$2eadd$2derror` |
| `Data.Sym.$` | `i.Data$2eSym$2e$24` |
| `Data.Sym.→` | `i.Data$2eSym$2e$e2$86$92` |

**Where the escaping applies.** Escaping applies to each encoded run: the
whole identity passed to `encode-identity`, or, for the `m.` and `o.` forms,
each of the two components separately. A run never contains a bare dot,
because a dot is escaped wherever it appears, so the bare dots a spelling
does contain are exactly the structural separators `i.`, `m.` and `o.`
introduce: `m.<escaped member>.<escaped owner>` splits at its first bare dot
without further syntax, and `i.<escaped identity>` never needs splitting.
That is why the escape covers the dot rather than only exotic scalars: keeping
the identity's own dots would make `m.A.B.C` ambiguous between member `A.B`
named `C` and member `A` named `B.C`.

**Decoding is the exact inverse.** `$` always starts an escape, the two
digits are lowercase, and a run of escapes is one scalar's UTF-8 encoding.
A stray continuation byte, an overlong form, a surrogate and a value past
`0x10FFFF` are refused, so a malformed spelling becomes an export refusal
rather than a guessed identity. A run embedded in a longer spelling (a
`.object-closure` accessor, a `.app` stub, a representation version `.r0`, an
`@ROT` variant suffix) ends at the first character outside the grammar. The
suffixes compose because each character escapes on its own: appending the
escaped class codes to an escaped name is the escape of the concatenation,
which is what the runtime object's variant and `.mk` factory lookups rely on.

**Why the change.** The first spelling wrote every scalar as its decimal
UTF-8 code and an underscore, so `Bootstrap.Infer` became
`66_111_111_116_115_116_114_97_112_46_73_110_102_101_114_`. Measured on the
compiler's own emitted IR (`build/bootstrap/stage2/compiler.ll`, 981,414
lines): 36,714 occurrences covering 48 modules, 5 identities and 8 builtins
spent 2,658,006 characters, 5.4% of the whole file, for 745 characters of
identities, a 4.6x expansion. The escape spells ordinary identifier text
whole, costs three characters for punctuation where the decimal form cost
three or four (`$2e` against `46_`), and is still reversible by construction.
The cost the decimal form imposed was not only bytes: the compiler's own
`Bootstrap.Infer.add-error` appeared 6,058 times as a 72-character string,
so the artifact used to diagnose codegen was the least readable one.

**Both implementations move together.** The pinned library object is produced
by the seed, so the seed encoder (`compiler/src/check_default_wire.rs`) and
the Cinch encoder (`bootstrap/Bootstrap.Nominal.cinch`) are one spelling:
the library producer pin and the compiler are rebuilt and repinned together,
and the roots extraction in `tooling/bootstrap_objects.py` decodes the new
spelling. Both encoders are pinned to the same table by
`compiler/tests/interface_method_bodies.rs` and
`compiler/tests/bootstrap_export_types.rs`, and both decoders are exercised
by the exporter and the type display.

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
