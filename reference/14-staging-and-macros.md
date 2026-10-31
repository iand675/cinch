# Staging, quotation, generation, and data imports

Staging distinguishes compile-time values, syntax, typed code, and runtime values. Generated code is checked under the same language rules as handwritten code.

# Typed quotation and execution

`[| expression |]` constructs a typed `Code a` value. `$(code)` splices an admitted code/syntax value into a quotation. Approved constant transport includes `quoteNat`, `quoteInt`, and `quoteBool`. Secrets and other compile-time-only values cannot be lifted into emitted program data.

`eval : Code a -> a` uses the admitted runtime evaluator path where the target profile permits it. It does not create proof evidence and may be forbidden by a target. A `Code` value is not an arbitrary unvalidated machine-code pointer. Syntax trees and typed code are distinct categories.

# Declaration macros

`macro name :: Syntax -> Declarations` declares a syntax transformation. `quoteDecls where ...` creates a declaration template; `name!(arguments)` invokes a macro over token/syntax arguments; `generate do emit (...)` inserts declarations. Macro expansion preserves definition-site helper identities, renames introduced local binders hygienically, and retains provenance for diagnostics.

A spliced name can deliberately name an emitted declaration. `'value` / `''Type` in staging-name contexts produce `Name` vocabulary. `name-to-sym` and `sym-to-name` connect that vocabulary with interned symbols where admitted. `CodecGenerator.derive` consumes an introspected schema snapshot to generate checked codec declarations.

Generated declarations undergo ordinary scope, type, effect, totality, and usage checks. A valid template producing an `Int` binding with a Boolean body still fails. Macro-owned attributes follow explicit ownership registration; unknown semantic attribute names are not silently accepted.

# Capabilities and recorded inputs

The staging vocabulary includes `snapshot`, `withReadFiles`, `withWriteFiles`, `withProcess`, `readFile`, `writeFile`, `run`, `secret`, `withDatabase`, `ReadSchema`, and `introspectConsistently`. Each operation is restricted to its capability and admitted staging effects. Runtime `Effect.Process` is not an ambient compile-time subprocess escape hatch.

`CINCH_STAGE_MODE` selects `refresh`, `replay`, or `live`; absent/unrecognized values select refresh. Refresh captures inputs, replay consumes captured inputs, and live uses the live staging path. Provenance and identities separate tool/input changes from credentials. `CINCH_SCHEMA_TOOL` selects the schema introspection tool used by that path.

Introspection requires a consistent schema snapshot; unsupported schema shapes and inconsistent snapshots are named failures. Secrets can authorize the operation but cannot be emitted into constants, errors, or captured public provenance.

# Dependencies and converters

`import stage Module` establishes the stage dependency. Dependency cycles are rejected. `import data "file.json" as value` selects a converter; built-ins cover JSON, text, and bytes, while registered stage modules can supply additional conversion. The manifest records input content and converter identity. A converter failure names the input and converter.

<span class="support-tag support-tag--planned">Coming soon</span> **broader staged value/solver support.** The specifications describe extensible staging and richer proofs over generated structures. Only the admitted constant family and checked capability paths work today. A new compile-time value category needs explicit transport and metadata rules, not a generic unsafe cast.

Implementation: `parser.rs`, `ast.rs`, `check.rs`, `check_terms.rs`, `check_decls.rs`, `driver.rs`; suites: `staging.rs`, `surface.rs`, `wasm.rs`. Tutorial: [compile-time code](../learn/18-staging-and-macros.md).
