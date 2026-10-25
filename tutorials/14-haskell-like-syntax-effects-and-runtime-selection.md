# Tutorial 14 Haskell-like syntax, effects and runtime selection

Source: <https://chatgpt.com/space/page_edc8263b2390819188b488f4fbcf2372>  
Exported: 2026-10-22T18:59:36Z

---

The current notation is strict and Haskell-like. Types use `::`, definitions use equations, calls use whitespace, and indentation groups declarations and statements. `twice 21` evaluates to 42; `twice` can be defined by `twice x = x + x`.

# Bind a value or sequence an operation

```haskell
checkedIndex :: (n :: Nat) -> Nat -> Option (Fin n)
checkedIndex n i = case decide (i < n) of
  Yes bound -> Some (Fin i bound)
  No _      -> None
```

The first parameter appears in the result type. A successful decision supplies proof evidence for the concrete bound; the compiler may erase the proof but must retain the comparison. A failed check produces `None`.

In ordinary direct `do`, `<-` sequences a computation and binds its whole result. If a read returns `Result ReadError Bytes`, the binding receives that Result; use `case` to inspect it. `let` is reserved for pure bindings, even though both forms are strict.

# Spawn work explicitly

```haskell
loadPair :: forall r. using (io :: Io r) =>
            Request -> Request
            ->{Async r, IO r, Cancel r, Diverge}
              Result LoadError (Bytes, Bytes)
loadPair left right = withScope $ \s -> do
  a <- spawn s (\() -> fetch left)
  b <- spawn s (\() -> fetch right)
  joinAllOrCancel s a b
```

The lambdas delay the calls until the tasks run. Without them, strict argument evaluation would execute a call before passing its result to spawn. Ordinary suspension needs no `await`: the effect row already records it. Spawning creates explicit child tasks, while ordinary calls sequence within the current task.

The scope retains children and their resource obligations until joined. Cancelling one child does not prove its foreign call has stopped. The exposed divergence effect also prevents this function from being used as a total logical proof.

# Select an implementation

The `using` context asks for a lexical `Io r` provider. At a call site, `loadPair @{io = chosenIO} left right` supplies it explicitly. A surrounding `with (io = chosenIO) in ...` supplies it implicitly to calls in that lexical body. The entry point still needs a handler for the effect row; binding a capability alone does not handle it.

The same application function can use a native, Wasm or deterministic-test provider when each satisfies its contract. Parallel execution, real networking and deterministic replay are different capabilities, so an implementation cannot promise them interchangeably.

# Keep do unqualified

A module can select Result sequencing through a signature parameter:

```haskell
module ImportConfig (Seq :: ResultSequencing ConfigError) where
  loadConfig path = do
    bytes <- readConfigBytes path
    config <- parseConfig bytes
    pure config
```

This module's interface explicitly designates `Seq` as its sequencing provider. The selected laws say failures short-circuit, success payloads bind, and effects retain their order. A signature for `loadConfig` must expose its actual runtime effects when exported. The example omits it only to show local inference.

The `do` keyword is unchanged. Generic code can request the sequencing module it needs, and module wiring resolves that request. The compiler never guesses a new interpretation merely because it encounters a Result value. Pure, direct code that intentionally examines errors therefore remains possible.

# What the grammar does not prove

The grammar accepts the notation; elaboration checks dictionary selection, effect rows, dependent indices and ownership. Proof checking then establishes the specified laws. A syntactically valid program can still be rejected for an ambiguous provider, a missing proof, a borrowed task escaping its scope, or a suspension hidden behind a pure signature.

[Documentation overview](../overview.md)

