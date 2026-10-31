# Modules and compilation specification

Source: <https://chatgpt.com/space/page_d64504b7cf08819185f30f677ededc93>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Draft 0.2. This chapter specifies abstraction, proof-carrying optimization, hygienic staging, and compile-time I/O. MUST denotes an intended requirement. The module representation, syntax, and APIs are proposed; their formal integration with the dependent core remains to be proved.

# 1 Dependent modules

A module MUST be able to package abstract and concrete types, values, operations, associated type families, semantic models, laws, and staged generators. Later fields may depend on earlier fields. Module signatures are dependent interfaces; functors accept modules and return modules.

Module abstraction MUST restrict both ordinary code and reflection. Private constructors and layouts are not accessible merely because code executes at compile time. Publicly exposed type equations remain available to callers under the chosen transparency policy.

Higher-kinded parameters, associated types, and ordinary algebraic abstractions are required. Structural records and row polymorphism provide lightweight data composition. Opaque nominal types provide intentional identity where structural equivalence would erase a meaningful distinction.

Implicit module arguments provide typeclass-style ergonomics. Default resolution MUST be coherent within a compilation context. Explicit dictionary selection MUST remain available when multiple implementations are meaningful. Resolution MUST NOT silently select a different runtime algorithm merely because an unrelated proof or import has become available.

The exact rules for applicative and generative functors are open. The chosen rules MUST define when separately constructed abstract types are equal. Source spelling or an unsafe cast cannot establish that equality.

# 2 Operations with dependent guarantees

A shared operation name need not imply one fixed container-preserving result type. The following interface deliberately allows different filtering outcomes:

```haskell
signature Filtering (c :: Type -> Type) where
  type Output :: forall a. (a -> Bool) -> c a -> Type
  filter :: forall a. (p :: a -> Bool) -> (xs :: c a) -> Output p xs
```

This minimal interface is schematic: usage and effect parameters must be elaborated for each implementation. An owning implementation must use a ghost model or equivalent stable index rather than leave an executable result type depending on an invalidated resource.

The interface supplies an operation but little common reasoning. A useful client additionally requests a law module or a richer interface describing how to observe the result. Unconstrained associated types MUST NOT be mistaken for shared semantics.

| Implementation | Result and required semantic information |
| --- | --- |
| Finite list | Finite list with exactly the accepted elements in original order |
| Length-indexed vector | Existential length, size bound, and exact accepted contents |
| Productive stream | Search process that may fail to emit another value |
| Stream with progress evidence | Productive stream of accepted values |
| Resource source | Effectful source with specified order, ownership, and cleanup |

Semantic equality of finite sequences includes order and multiplicity. A membership-only theorem is insufficient: a filter could reorder elements, duplicate them, or return an empty result while satisfying weaker one-sided properties.

# 3 Models and law modules

For finite pure sequences, a semantic model can map a concrete representation to a mathematical list:

```haskell
signature FiniteSequence (c :: Type -> Type) where
  spec contents :: forall a. c a -> List a
  filter :: forall a. (a -> Bool) -> c a -> c a
  proof filterCorrect :: forall a. (p :: a -> Bool) -> (xs :: c a)
                      -> Eq (contents (filter p xs))
                            (List.filter p (contents xs))
```

The law notation refers to the logical interpretation of the pure operation; it is not a second runtime execution or resource consumption. For mutable or owning representations, a contract instead relates pre-state and post-state views and the linear permissions transferred by the operation.

Operations and laws may be supplied in separate modules. An unproved algebra instance can support ordinary calls, but a client or optimization requiring associativity MUST receive associativity evidence tied to that exact operation dictionary.

Effectful processes use traces or transition semantics. Their contracts distinguish output content, order of effects, observable termination, resource completion, and temporal progress where relevant. A value-only model cannot justify transformations that alter these observations.

# 4 Verification interfaces and compilation

A compiled interface MUST carry exported signatures, abstract type identities, public type equations, effect and usage information, contracts, required assumptions, and identities for referenced laws. It also records representation information intentionally exposed to specialization and update compatibility.

Caller verification SHOULD depend on public contracts rather than implementation unfolding. Explicit transparency and unfolding controls are required to keep proofs stable. A public type-level definition that callers reduce becomes part of their checked dependency graph.

The compiler MUST distinguish dependencies on an interface, a logical definition, an implementation body, a representation, a generated input, and an optimization law. These dependencies drive rebuilds, caches, and hot-update compatibility.

# 5 Verified rewrite rules

A rewrite declaration contains a pattern, replacement, applicability conditions, a semantic relation, and checked evidence under that relation. The optimizer MUST ensure the substitution satisfies the theorem's type, effect, resource, and other side conditions.

```haskell
rewrite mapMap forall (f :: Int -> Int) (g :: Int -> Int)
                     (xs :: List Int). where
  match List.map f (List.map g xs)
  replace List.map (\x -> f (g x)) xs
  justify mapFusion f g xs
```

This example is for finite vectors and total pure reusable callbacks. It does not authorize reordering effectful maps. Strict eager mapping performs all inner callbacks before outer callbacks; fusion can interleave them. A theorem proved for pure values MUST NOT be reused for effectful callbacks without a stronger argument.

Named observational equivalences require appropriate congruence or contextual-refinement evidence. Bisimilar streams are not automatically substitutable into a context that can observe an additional distinction excluded from the relation.

Optimizer rewriting MUST be separate from the type checker's definitional conversion. User rewrite rules MUST NOT make an otherwise terminating conversion procedure loop. The optimizer itself MUST enforce resource budgets or another termination policy: semantically correct rules can cycle or explode code size.

Rule correctness and profitability are separate. Proofs authorize transformations; cost models choose among them. The cost model need not be trusted for functional correctness unless a performance property is itself part of the contract.

Floating-point laws MUST match the selected machine semantics. Real-number associativity cannot justify reassociation under exact IEEE-style result requirements. Approximate transformations require an explicitly selected error relation and a checked error bound.

# 6 Stream fusion

The standard library SHOULD expose explicit stream or producer descriptions whose composition can be lowered to state machines and loops. A conceptual step form is `Done`, `Skip(state)`, or `Yield(value, state)`. The representation may be specialized away after its semantics are established.

Fusion SHOULD cover mapping, filtering, zipping, folds, and nested producers where the necessary evidence exists. It MUST preserve the selected observation model, including divergence and relevant effect order. Terminating each step is not enough to establish that the machine yields.

Productive streams, finite producers, and resource sources MUST remain distinguishable. Resource sources require cleanup for exhaustion, early termination, errors, and cancellation under the stated runtime policy. Filtering cannot implicitly duplicate or abandon an owned element whose protocol requires explicit disposal.

The design SHOULD offer inspectable optimization requirements such as no intermediate collection allocation in a specified generated function. These assertions need precisely defined IR scope. They do not imply zero allocation in callbacks or a universal performance bound.

A compiler report SHOULD identify which fusion steps occurred, where specialization stopped, and which allocation or effect prevented a requested transformation. Proof verification alone does not establish that a profitable lowering was selected.

# 7 Staging model

Metaprogramming MUST support syntax macros, typed code construction, and executable compile-time computation. They share hygiene and source tracking, but operate at different points: syntax expansion may precede complete type information, while typed quotations require an elaborated context.

Conceptually, typed code carries `Code<stage, context, effects, result>` plus usage constraints. Surface notation may omit inferred fields. Open code is valid only in an explicit context; an arbitrary unresolved identifier is not a valid free variable.

```haskell
stage composeCode :: forall a b c. Code (b -> c) -> Code (a -> b)
                  -> Code (a -> c)
stage composeCode f g = [| \x -> $(f) ($(g) x) |]
```

This tutorial form uses closed reusable callback code. More general quotations must track captured linear variables and context extension. Duplicating a quotation cannot duplicate a captured owned value. Erased evidence cannot be promoted to executable data through staging.

# 8 Hygiene and binding

Identifiers MUST carry binding identity and scope information. Generated binders are fresh by default. Macro helper references resolve at the definition site; caller syntax retains its original binding relationships. Source maps MUST retain expansion provenance for diagnostics.

Intentional capture and public-name generation require explicit operations. A declaration macro can export a chosen name, but accidental textual collisions MUST NOT change bindings. Dependently typed quotations must extend the context for both terms and types when introducing a binder.

Generated declarations, proofs, and rewrite rules MUST be checked under ordinary rules. The macro implementation is not authorized to introduce axioms silently. Generated names and module identities must have deterministic, documented behavior under reproducible builds.

# 9 Dependencies across stages and modules

Modules MAY export executable generators, typed quotations, and permitted reflection metadata. The compiler MUST schedule compile-time execution from an explicit dependency graph. Same-module generators are allowed when their prerequisites can be compiled before their uses, regardless of avoidable textual restrictions.

Genuine cyclic dependencies remain invalid unless a separately specified fixed-point mechanism resolves them soundly. A diagnostic MUST show the cycle, such as generator compilation depending on a declaration that executing that generator would produce.

Cross-stage persistence occurs through approved serialization, supported constants, stable references to target-stage definitions, or checked transport of logical information. Host pointers, open file descriptors, sockets, and compiler-process resources MUST NOT become target runtime resources by quotation.

Compile-time host architecture and target architecture may differ. Host-sized values, layouts, endianness, and subprocess outputs MUST NOT silently determine target semantics without declared inputs and a target-aware conversion.

# 10 Compile time I/O

The stage runtime MUST support file access, database introspection, network requests, and subprocess execution through explicit capabilities and build policy. Pure generation remains convenient, but external work is a supported use case.

Capabilities specify what a generator may access. A database schema reader should not receive arbitrary write authority by default. Effects record that I/O occurs; capabilities determine permitted targets. Cancellation and stage failure MUST release resources according to the host runtime policy.

| Mode | Required behavior |
| --- | --- |
| Refresh | Execute external reads and record replayable input snapshots |
| Replay | Use declared captured inputs; reject missing or incompatible inputs |
| Live | Perform explicitly requested current reads; record provenance and avoid false cache freshness |

The build identity MUST include generator and compiler versions, declared input hashes, relevant target configuration, and the selected dependency and policy information. For subprocesses, inputs include the tool identity and relevant environment. Untracked nondeterminism prevents a reproducibility claim.

Secrets authorize access but MUST NOT be embedded in generated source, normal logs, or snapshots. Error messages must redact credentials. Sensitive query results require an explicit recording policy; a snapshot can itself contain confidential schema or data even when credentials are removed.

Retries MUST NOT assume arbitrary stage I/O is idempotent. A generator that performs writes needs an explicit protocol; a failed build does not roll back external effects automatically.

# 11 Schema generation

Schema introspection SHOULD obtain a consistent database snapshot where the provider supports one. The generator records the interpreted schema and relevant dialect, extension, collation, and nullability assumptions needed by its binding semantics.

Generated binding proofs are relative to that captured schema. A proposed runtime API is:

```haskell
connect :: forall s. DatabaseConfig -> Schema s
        ->{IO} Result SchemaMismatch (Connection s)
```

Successful validation establishes the compatibility supported by the validator, not every possible database behavior. If external actors can change the schema afterward, startup validation alone is insufficient. A schema lease, managed migration epoch, connection invalidation, or equivalent protocol must maintain the relied-upon facts.

Typed queries cannot make external availability, transaction success, or network delivery total. These remain explicit effects and outcomes. A schema hash may be an input to compatibility checks; it is not a universal proof of semantic equivalence.

# 12 Compilation pipeline and artifacts

The proposed pipeline is dependency analysis; staged execution and syntax expansion; elaboration to the typed core; ownership, effects, termination and proof checking; checked optimization; erasure and lowering; code generation; artifact packaging. These phases may interleave where dependencies require it, but every generated or transformed artifact must pass its applicable boundary checks.

The artifact manifest records exported contracts, proof and assumption identities, external input provenance, ABI and layout versions, implementation dependencies, and optimization dependencies. Upgrade-capable modules additionally carry the metadata required by the runtime evolution specification.

The initial implementation may trust some compilation passes. It MUST report the actual trust boundary. Checked source proofs alone are not a proof of arbitrary backend machine code.

# 13 Target specific specialization

Rewrite libraries MAY be parameterized by target modules that expose instruction semantics, feature requirements, vector shapes, and memory preconditions. The optimizer MUST discharge the target conditions and preserve them in the artifact manifest. Source-to-SIMD proofs, backend lowering proofs, and runtime feature availability are distinct obligations. The Platform proofs and SIMD specification defines this extension, including scalar fallback, scalable vectors, and arithmetic-specific reduction laws.

# Foreign bindings and deployment interfaces

Binding generators consume declared headers, WIT interfaces, target ABI descriptions and captured toolchain inputs. Their output is checked like other generated code; header generation cannot infer a missing ownership or semantic contract. Cross-compilation must use target layouts, not the build host's layout.

Module artifacts record foreign implementations and assumptions, target features, component worlds, runtime capabilities and async ABI requirements. Changing these dependencies can invalidate a proof, adapter, specialization or upgrade package. See [Foreign function interface specification](foreign-function-interface-specification.md) and [WebAssembly and WASI specification](webassembly-and-wasi-specification.md).

# Current syntax and sequencing

The draft 0.2 surface uses layout, equations and unqualified `do`. See [Surface syntax and grammar](surface-syntax-and-grammar.md) and [Runtime capabilities, effects and sequencing](runtime-capabilities-effects-and-sequencing.md) for formal syntax, runtime arguments, effect-polymorphic calls and sequencing-module selection.

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Compiled interface metadata (D4, D9, D121), 2026-10-28.** This
  amends section 4: the compiled interface is a canonical JSON
  `.cinchif` artifact per module (exports, instances for coherence,
  fixities, foreign contracts, assumptions manifest, ABI symbol table,
  and landed fact rows: contracts (D198-D205), assertions/must-use
  (D112), law rows (D285-D287), upgrade evidence), identity by content
  hash with imported interfaces recorded as the checked-against chain.
  `iface_hash` covers the contract only, excluding `source_hash` and
  `compiler` (D9). Interface dependencies are a set: import refs are
  deduplicated (D121).
- **Module files and export surface (D42, D107-D108, D143-D146),
  2026-10-28.** A source file may contain any number of `module`
  blocks (the one-module-per-file rule is superseded); with `export`
  declarations a module's surface is exactly the named set and imports
  are private until re-exported (`export module X` bulk, `export A.b`
  named); `import X as Y` and `import X (a as b, c)` rename/select
  with the duplicate-name rule over explicit imports (the prelude is
  the implicit base layer; own names shadow imports). Instances
  and the foreign table stay module-owned facts, not names. Fixities
  belong to the module declaring the operator and follow that binding
  through imports and re-exports; consumers cannot redeclare or override
  them (user direction, 2026-10-30). An operator without a declared fixity
  remains available in prefix form.
- **Staging, macros, and compile-time I/O (D43-D48), 2026-10-28.**
  This amends sections 7-10: `Code t` is closed code and compile-time-
  only types (`Code`, `Syntax`, `Declarations`, `String`, `Secret`,
  capability tokens) never reach target code (D43). Compile-time
  evaluation interprets the elaborated core under an 8M-step budget
  with lazy/memo/`Bytes`/float/`Option`-returning prims refused.
  Capabilities determine permitted targets (`IoCap` root refined by
  `withReadFiles`/`withWriteFiles`/`withProcess`), builtin stage effect
  labels record occurrence, and callbacks are effect-row-polymorphic
  (D45). Build identity is the manifest `staging` section plus
  `build/<entry>.stage.snap` captured contents with refresh/replay/live
  modes; replay rejects missing or hash-incompatible inputs; secrets
  record names only and cannot be embedded; failed stages kill spawned
  children and are never retried. Macros are compile-time functions
  over token trees with capture-avoiding renaming plus definition-site
  qualification and per-node expansion provenance in spans (D46); the
  staging graph schedules units over statically-apparent emits with
  rendered cycle chains (D47); attributes in staging follow D48
  (unknown names outside `meta` need a macro declaration; `repr` is
  rejected, not ignored; macro-emitted attributes revalidate).
- **Module parameters and signatures as module arguments (D115-D121),
  2026-10-28.** This amends section 1: `module M (p :: T, ...) where`
  declares dictionary and value parameters, and instantiation is
  import-time application with static arguments (`import M (args) as
  N`), each instantiation a distinct compilation unit with computed
  identity `unit_id = hash(target | parameter signatures | argument
  identities)`. Instantiation is representation: the parameterized
  module's `iface_hash` is invariant and dictionary arguments pass
  through the ordinary instance machinery. Composition forms are
  rejected with named errors (an instantiation inside a parameterized
  module, a parameterized instance as an argument, index-position
  references): forwarding parameters would be full functor machinery
  (D116). Capability (`Io`) and `Copy` parameters are rejected; value
  parameters require `Copy` (D117). `stage`/`macro`/`generate`/
  `rewrite`/`upgrade` declarations in a parameterized module are
  rejected (D118). Explicit (named) dictionaries coexist for one
  class; coherence governs implicit resolution only (two instantiations
  publishing unifiable implicit copies reject as `ambiguous instance`),
  and module arguments are this chapter's explicit dictionary selection
  (D119).
- **Schema generation (D72), 2026-10-28.** This amends section 11:
  `SchemaSnapshot`/`Database`/`SchemaRead` are compile-time-only and
  introspection runs over the subprocess capability (the credential
  travels in the tool environment and never enters captured inputs);
  `introspectConsistently` checks the document and records
  dialect/collation/extensions/nullability, provider identity, and a
  content hash; `CodecGenerator.derive` emits per-table records plus
  `encode`/`decode` as ordinary checked declarations. Codec columns
  cover `INTEGER` and `BOOLEAN`; `REAL`/`TEXT` report explicitly at
  generation time.
- **Rewrite justification and fusion (D55-D57, D60), 2026-10-28.**
  This amends sections 5-6: `justify e` accepts kernel evidence
  (`refl`, `congr`, `decide` extractions, declared `proof`/`spec`
  functions) or a law method of a `signature`, with type-preservation
  and resource side conditions; bare-variable and erased/universe-type
  equations are rejected as defeq-changing (D56); effectful rules are
  rejected (pure `Eq` evidence does not authorize reordering) and
  pure-hole callbacks are body-scanned at fire time (D57). Pull-based
  producer fusion composes callbacks at the same per-element positions
  preserving effect order; barriers are memo/thunk allocation and, for
  eager pipelines under the `mapMap` law, callback purity; case folds
  let-bind constructor fields (D60).
- **Trust boundary reporting (D63), 2026-10-28.** This amends section
  12: the manifest records `trust.level` (level 2, full recheck, the
  default) and a `lowering` checked/trusted split, so compiler-pass
  trust is disclosed rather than implied.
- **Target specialization (D58-D59), 2026-10-28.** This amends section
  13: target profiles and vector shapes are marker types
  (`VecU32Target t w`, `CapAvailable t w`) and `Available v` is the
  validity-scope evidence (lexical region + thread, never cached); a
  `target rewrite` fires only where the firing unit carries a matching
  capability parameter, `Target.detectVecU32` is a replaceable dispatch
  boundary, and target conditions are discharged and preserved in the
  manifest (`target.conditions`).
- **Entry point (D12, D20, D65), 2026-10-28.** `Program.main` is
  `Option`: `check` accepts library modules without `main`, while
  `emit-ir`/`build`/`run` require `main :: Int` or `main :: Unit ->{e}
  Int` (D65); `foreign export` programs may be linked as libraries with
  the C side supplying `main`. The checker synthesizes a
  `"<module>.main.exit"` adapter calling `cinch_int_to_i64` on `main`'s
  bignum result so the C `main` stays on raw words (D12).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources are `.cinch`, compiled interfaces `.cinchif`, runtime externs
  `cinch_*`; artifact names in this chapter read with those names.
