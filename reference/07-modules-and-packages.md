# Modules, signatures, instances, and packages

Module identities determine ownership of names, operator fixities, type definitions, dictionaries, and contracts. A short identifier is not a globally unique identity.

# Modules and visibility

`module A.B where` declares a module. A file can contain several modules. Imports resolve by declarations, first against already loaded modules, then the matching sibling file, then a unique sibling declaration found by scanning. Missing or ambiguous modules produce named diagnostics. Runtime and stage dependency cycles are rejected according to their graph.

Without explicit exports a module exports its declarations. With `export ...` declarations, the listed surface is public. Imports are private until re-exported. Supported forms include `export name`, qualified member export, and `export module Other`. Selection/renaming uses `import Other (x as y, z)`; `import Other as O` supplies a module alias. Unqualified import conflicts are errors rather than import-order choices.

The implicit `Prelude` is a facade, not every module in the repository. It curates exports from defining homes. See [the library index](../library/00-index.md) for embedded/source distribution and exact APIs. Source modules can override embedded copies, and checked metadata preserves the defining identity rather than silently confusing the two.

# Signatures and instance resolution

A `signature` declares method types, optional defaults, and law vocabulary. An `instance` implements the dictionary. `implicit instance` makes it available to resolution, which requires a unique applicable instance. A `using (dict :: Signature arguments)` binder is an explicit required dictionary; calls elaborate to dictionary passing.

Signature blocks currently admit `opaque type` and `record` members, which the parser splices ahead of the signature so method types can name them. This is an implemented associated-member facility; it is distinct from per-instance associated result families such as the planned `Output` example below.

Default methods can call other methods through the selected dictionary, so omission is permitted only when a checked default supplies the method. Contracts and method visibility survive interfaces. Re-exporting a class name does not necessarily export every private method; the emitted public method names are listed in the library reference.

Superclass entailment is not implemented: state every class your body needs. `Copy`/`Share` is compiler-controlled and not user-extensible. Do not create competing implicit instances to simulate lexical policy selection.

# Module parameters

`module Tally (S :: Stepper) where` takes a signature dictionary. `import Tally (StepUp) as Up` instantiates it. Value-typed parameters are admitted too. Parameter identities and supplied arguments participate in metadata and coherence.

<span class="support-tag support-tag--planned">Coming soon</span> **general parameterized-module composition.** The broader composition forms described by the module design are not all admitted. The current implementation covers the dictionary/value parameterization exercised by `params.rs`; unsupported composition forms are named refusals rather than an inference feature.

# Intended dependent modules and associated types

<span class="support-tag support-tag--planned">Coming soon</span> **Richer dependent module interfaces.** The module specification requires modules that package abstract/concrete types, associated type families, semantic models, laws, and staged generators, with later fields depending on earlier fields. Current signatures/dictionaries and value/module parameters implement only part of that model.

This **specified schematic fragment** is not accepted as a runnable declaration by today's Rust compiler:

```text
signature Filtering (c :: Type -> Type) where
  type Output :: forall a. (a -> Bool) -> c a -> Type
  filter :: forall a. (p :: a -> Bool) -> (xs :: c a) -> Output p xs
```

The result family lets each implementation express its guarantee: filtering a finite list returns a list in input order; filtering a vector returns an existential length plus bounds/content evidence; unrestricted filtering of a productive stream returns a possibly searching producer unless progress evidence exists. Associated types alone supply no shared law about those results. A client needs a law/observation interface stating the common semantics, and an owning implementation needs a stable ghost model rather than a result type depending on invalidated resources.

Structural record/row composition and the full abstract-module/reflection restrictions are also specified beyond the current nominal-record surface. Exact applicative/generative functor identity rules remain open design choices. Do not present a chosen syntax or type-equality rule for those unresolved cases as already settled.

Current parameterized modules also refuse stage/macro/generate/rewrite/upgrade declarations that would need their parameters, nested unit instantiation/forwarding, parameterized instances as module arguments, and index references through parameterized functions. `Io` capabilities remain lexical and `Copy` remains structural; neither is an arbitrary module parameter. Value parameters need copy evidence because definitions share them.

# Data imports and staging imports

`import stage Other` establishes a compile-time dependency. `import data "path" as value` invokes a converter for a non-source input. Built-in text, bytes, and JSON conversion retain input/converter provenance. Registered stage converters follow staging capability and budget restrictions; conversion failure is a build diagnostic. A file extension does not authorize arbitrary ambient process execution.

# Package manifests and stores

The `.cinchpkg` format is canonical JSON with package name/version, module entries, and an explicit check order. A module entry names the module and source path, with facade metadata where relevant. Imports are read from source rather than duplicated in the manifest. `cinch-base.cinchpkg` defines the embedded family.

The current CLI does not install remote dependencies, resolve version ranges, or publish packages. Shipping source modules beside the entry is a supported development pattern. The package store is a verified local content-addressed artifact mechanism, described in [artifacts](18-artifacts-and-incremental.md).

Implementation: `driver.rs`, `interface.rs`, `check_interface.rs`; suites: `modules.rs`, `params.rs`, `surface.rs`, `fixity_interfaces.rs`, `class_defaults.rs`, `package_manifest.rs`, `package_store.rs`.
