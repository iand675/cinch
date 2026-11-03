# Compiler-provided vocabulary and special forms

Most operations are ordinary library declarations, indexed in the [library reference](../library/00-index.md). Some vocabulary is installed directly by the Rust checker or elaborated specially and therefore does not appear as a source declaration in Prelude. This page covers that additional surface.

# Core values and indexed types

`True`, `False`, and `()` are core values. Primitive types include mathematical `Nat`/`Int`, machine scalars, `Bool`, `Unit`, `Bytes`, and `Sym`; library declarations layer operations and text/container representations on top.

`Vec n a`, `Fin n`, `Sigma`, equality/proposition vocabulary, `Dec`, `Yes`, `No`, `refl`, `congr`, and `decide` have dependent/logical checking rules. `Fin value evidence` requires its bound; `Vec.getCopy` reads through an admitted borrowed vector with a finite index and structural copy evidence. See [dependent types](08-dependent-types-and-proofs.md) for erasure and the rigid-index restriction.

`Copy`/`Share`, `Ref`, `Mut`, `borrow`, `mutBorrow`, `copyOut`, and `refSet` are ownership-aware vocabulary. They do not behave as arbitrary user functions detached from use/escape checking. `Thunk`, `Lazy`, `delay`, `force`, `demand`, and `Later` likewise use special deferred-evaluation and guardedness rules. A local binding may shadow some ordinary-name forms; reserved intrinsic namespaces cannot be repurposed as user primitives.

# Runtime entry and I/O

| Name | Current role |
| --- | --- |
| `newIO ()`, `Runtime.newIO ()` | Construct a fresh runtime capability identity. |
| `ioWrite bytes` | `Bytes ->{IO r} Unit`, requiring the applicable `Io r` capability. |
| `ioRead count` | `Nat ->{IO r, Diverge} Option Bytes`: read a requested count; handle absence/end of input. |
| `ioSleep duration` | `Nat ->{IO r, Cancel r} Unit`, requiring `Io r`. |
| `pollCancel ()` | `Unit ->{Cancel r} Bool`, requiring `Io r`. |
| `withScope` | Open a structured task scope with scoped lifetime. |
| `spawn scope body` | Start a task with capture/transfer checking and return a linear handle. |
| `joinAllOrCancel scope handles...` | Consume a nonempty admitted join group, cancel peers on failure, await acknowledgment. |

Task handle, scope, and error vocabulary is installed alongside these operations. The complete schemes and specialized n-ary checking live in `install_concurrency_api` and the application elaborator, rather than in a single source `foreign import` signature. [Concurrency](13-concurrency-and-runtime.md) specifies ordering, effects, and lifetime rules.

`ignore expression` explicitly acknowledges discarding a must-use result. A wildcard binding does not serve as `ignore`. Typed expression holes are currently rejected with a named diagnostic; `_` remains valid as a pattern wildcard.

# Staging vocabulary

`Code a`, `Syntax`, `Declarations`, and `Name` distinguish typed code from syntax/name categories. `Secret`, `IoCap`, `FileReadCap`, `FileWriteCap`, `ProcCap`, `Database`, `SchemaRead`, and `SchemaSnapshot` are staging capabilities/data, not runtime authority supplied by arbitrary strings.

`snapshot` supplies the staging root to a callback. `withReadFiles`, `withWriteFiles`, and `withProcess` refine that authority. `readFile`, `writeFile`, `run`, `secret`, and `emit` perform their corresponding staging effects. `quoteNat`, `quoteInt`, `quoteBool`, `strNat`, `natStr`, `strAppend`, and `strLen` cover the admitted constant/text transport helpers. They also have the registered `Stage.` qualification.

`withDatabase`, `ReadSchema`, and `introspectConsistently` obtain a checked snapshot through the configured introspection path. `CodecGenerator.derive` returns generated declarations. `name-to-sym` and `sym-to-name` connect staging names to symbols. `[| ... |]`, `$(...)`, `quoteDecls`, `macro`, and `generate` are the relevant syntax forms. `eval` consumes typed code under target admission.

## Staging signatures

These schemes are rendered by the Rust REPL type-query API; local metavariable spellings are normalized to `a` and `e`. Callback effects flow through the capability providers. Stage-only operations are not ambient runtime I/O.

```cinch
snapshot :: forall a e . (String -> ((IoCap ->{e} a) ->{e} a))
withReadFiles :: forall a e . (IoCap -> ((FileReadCap ->{e} a) ->{e} a))
withWriteFiles :: forall a e . (IoCap -> ((FileWriteCap ->{e} a) ->{e} a))
withProcess :: forall a e . (IoCap -> ((ProcCap ->{e} a) ->{e} a))
readFile :: (FileReadCap -> (String ->{StageFile} String))
writeFile :: (FileWriteCap -> (String -> (String ->{StageFile} Unit)))
run :: (ProcCap -> (String -> (List String ->{StageProcess} String)))
secret :: (String ->{StageSecret} Secret)
emit :: (Declarations ->{StageEmit} Unit)
quoteNat :: (Nat -> Code Nat)
quoteInt :: (Int -> Code Int)
quoteBool :: (Bool -> Code Bool)
name-to-sym :: (Name -> Sym)
sym-to-name :: (Sym -> Name)
strNat :: (String -> Nat)
natStr :: (Nat -> String)
strAppend :: (String -> (String -> String))
strLen :: (String -> Nat)
withDatabase :: forall a e . (IoCap -> (Secret -> (SchemaRead -> ((Database ->{e} a) ->{e} a))))
ReadSchema :: SchemaRead
introspectConsistently :: (Database -> SchemaSnapshot)
CodecGenerator.derive :: (SchemaSnapshot -> Declarations)
```

# Boundary and target vocabulary

`RawPtr`, opaque foreign types, generated WIT vocabulary, `WasiStream`, `WasiFuture`, and adapter error types are introduced under their admitted boundary paths. `wasiStreamRead`, `wasiStreamWrite`, and `wasiFutureWait` carry foreign/divergence effects. Target conditions, `prim` bodies, and target rewrites require profile evidence.

`cinch.prim.*` and `cinch.runtime.*` are reserved intrinsic namespaces. Their source declarations in the library record exact public wrappers and result obligations; guessing a reserved symbol does not provide a supported extension API. Define an admitted `prim` or a real foreign binding with explicit containment/ABI contracts instead.

Implementation: `compiler/src/check_decls.rs` (`install_builtins`, `install_concurrency_api`, `install_eval_api`, `install_staging_api`, `install_index_families`, `install_index_values`, `install_wit_api`, `install_stream_api`) and special applications in `check_terms.rs`.
