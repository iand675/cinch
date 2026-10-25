# Tutorial 8 Staging and schema generation

Source: <https://chatgpt.com/space/page_668e270bca9c8191a1822f4548af4a0c>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn to combine hygienic typed code generation with real compile-time I/O. Prerequisite: dependent modules and effects. The database and generator APIs here are proposed interfaces, not connections to an actual database.

# Construct typed code

```haskell
stage addOne :: Code Nat -> Code Nat
stage addOne body = [| let x = $(body) in x + 1 |]
```

The generated `x` is a fresh binding. A caller's `x` inside spliced syntax keeps its original identity. A macro helper resolves where the macro was defined. Intentional capture requires an explicit operation.

`Code<Nat>` abbreviates closed, pure code at the target stage. Open code, effects, and captured resources require richer inferred context information.

# Export a generator

```haskell
module SchemaBindings where
  stage derive :: SchemaSnapshot -> Declarations
  stage derive schema = CodecGenerator.derive schema

module Client where
  import stage SchemaBindings
```

The compiled module exports executable generator code and permitted reflection metadata. A generator may also be used in its defining module when the dependency graph allows it. It cannot inspect private representation just because it runs during compilation.

# Read the real schema

```haskell
generate do
  snapshot <- Stage.snapshot "billing-schema" (\io ->
    Stage.withDatabase io (secret "SCHEMA_DATABASE_URL") ReadSchema
      (\db -> introspectConsistently db))
  emit (SchemaBindings.derive snapshot)
```

The snapshot helper interprets the current build mode. Refresh executes the callback and captures the schema. Replay uses the recorded snapshot and does not connect. Live executes the external read and records the provenance without pretending an old cached answer is current.

`withDatabase` scopes the compile-time connection. Its handle is not embedded in generated runtime code. The secret authorizes access and is omitted from generated source and ordinary snapshots. Schema content may still be sensitive and needs the chosen recording policy.

# Check generated evidence

The generator can emit row types, typed queries, codecs, and proofs relative to the captured schema. Every declaration goes through normal elaboration and proof checking. A generator's claim that a decoder is total is not itself evidence.

At runtime, a proposed `connect` operation validates the expected compatibility and returns `Connection s` only on success. A migration epoch, lease, or equivalent deployment policy must maintain relevant schema facts after validation. A startup check cannot prevent an independent administrator from changing the database later.

# Diagnose a staging cycle

If `derive` needs the definition of `GeneratedRow` to compile, but executing `derive` is what creates `GeneratedRow`, the compiler reports that dependency cycle. Moving files cannot make the logical dependency disappear.

# Exercise and answer

May a generator invoke a subprocess to read a schema file? Yes, with a subprocess capability. Reproducible replay must capture the relevant input, tool identity, and configuration. Host pointers or unrecorded environment state cannot become portable target constants.

Next: Tutorial 9 packages a compatible live upgrade.

[Previous tutorial](07-rewrites-and-stream-fusion.md) · [Next tutorial](09-compatible-hot-upgrades.md) · [Documentation overview](../overview.md)