# Example catalogue

Source: <https://chatgpt.com/space/page_ca01753c1a108191b5a81b2138f93a2e>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

The accompanying files include twelve `.lang` files in provisional language notation. There is no executable compiler, and the files have not been typechecked. They specify intended behavior and rejection criteria. Some files contain complete small algorithms under the proposed prelude; others are explicitly labeled API or proof-package sketches.

# Files and assumptions

| File | Scope | Required environment |
| --- | --- | --- |
| `01-bounded-index.lang` | Runtime bounds decision and safe element access | `Nat`, `Fin`, `Vec`, `Option`, comparison decision procedure, borrowed access |
| `02-vector-filter.lang` | Recursive finite filter with dependent output length | Vector constructors, copying elements, total pure predicate |
| `03-map-fusion.lang` | Inductive proof and associated rewrite | Immutable finite `List`, total `map`, equality, `congr` |
| `04-productive-stream.lang` | Guarded infinite producer and mapping | A sound `Later` discipline and coinductive observations |
| `05-schema-generator.lang` | Compile-time database generator API | Stage I/O, snapshots, secret handles, declaration generator |
| `06-counter-upgrade.lang` | Concrete state migration and operation models | Natural arithmetic, consuming ownership, upgrade coordinator for deployment |
| `07-rejected-programs.lang` | Independent expected rejection cases | The rules identified by each case |
| `08-platform-simd.lang` | Target-specific wrapping reduction and dispatch | Instruction semantics, capability detection, memory proofs, and checked lowering |
| `09-foreign-codec.lang` | Foreign wrapper contract | Explicit native assumptions or isolation profile |
| `10-wasm-component.lang` and `codec.wit` | Checked component adapter | WIT bindings, host failure policy, runtime bound check |
| `11-structured-tasks.lang` | Scoped concurrent requests | Task outcome, cancellation and cleanup contracts |

`Copy` permits intentional element duplication. Mathematical `Int` and `Nat` do not silently use wrapping machine arithmetic. Proof constructors are logical primitives or checked library lemmas. Named generator and runtime services describe required library contracts rather than pretend implementations.

# Bounded indexing

`checkedIndex(3, 2)` returns an index usable with a vector of length three. `checkedIndex(3, 3)` returns `None`. A successful runtime comparison creates evidence about the concrete value; that evidence is erased, while the comparison remains where static proof is unavailable.

Expected rejection: fabricating `Fin 3` for the value seven. Expected acceptance: branching on a checked bound and using the result only on the successful branch.

# Vector filtering

Filtering `[1, 2, 3, 4]` with an even predicate returns an existential length of two and `[2, 4]`. Filtering the empty vector returns length zero. Filtering with an always-false predicate returns the empty vector.

The algorithm records predicate decisions in source order before recursing. The example returns a dependent pair but does not silently claim the additional proofs. The accompanying proof targets are size preservation and exact subsequence correctness. The induction outline shows what must be supplied to expose those stronger guarantees.

# Map fusion

For pure total functions, the list induction supplies a proof of the map-composition equation. The rewrite uses that exact theorem. The intended optimization removes an intermediate list under the selected pure semantic model.

An effectful counterexample uses callbacks that log. Nested eager maps log every inner callback before any outer callback. A fused map interleaves those logs. The pure theorem is inapplicable, and the compiler must reject that instantiation.

# Productive streams

`from(0)` supports every finite requested prefix. Mapping square over it produces `0, 1, 4, 9, ...`. The delayed recursive call is guarded; a raw self-reference that fails to produce a head must be rejected.

The bisimulation outline compares heads and then guarded tails. A full implementation must select and formalize the guarded calculus before this syntax can be accepted. Arbitrary filtering remains a process unless a constructive progress interface justifies productive output.

# Schema generation

The schema example scopes a compile-time database connection, captures introspection input, and passes the snapshot to a generator. No connection is attempted by this documentation package. The secret name is illustrative and contains no value.

Expected mode behavior: refresh obtains and records input; replay uses recorded input; live explicitly contacts the database. Generated declarations require checking. Runtime bindings require a compatible schema and a policy maintaining the facts they rely on.

# Counter migration

For old state `value = 12`, migration produces `base = 12, delta = 0`. Reading returns twelve. After three increments, the new representation returns fifteen. The relation is equality of total values, not equality of layouts.

These finite observations are illustrative checks, not the universal proof. The arithmetic proof establishes the relation for all naturals. Deployment also requires the operational and dependency checks described in the runtime specification.

# Conformance matrix

| Case | Intended outcome | Reason |
| --- | --- | --- |
| Branch-certified vector access | Accept | Bound evidence matches the live vector |
| Retained existential output length | Accept | Runtime witness supports the dependent value |
| Pure map-composition theorem | Accept | Structural induction and equality congruence |
| Productive guarded natural stream | Accept under selected guarded calculus | Each observation is justified |
| Unknown index without a check | Reject | Required evidence is unresolved |
| Recursive proof of `False` | Reject | Logical termination fails |
| Arbitrary productive stream filter | Reject claimed total signature | No next accepted element may exist |
| Effectful use of pure fusion rule | Reject | The theorem's effect conditions fail |
| Embedded compile-time file handle | Reject | Resource cannot cross stages by serialization |
| Same-module acyclic generator | Accept | Compile-time dependencies can be scheduled |
| Genuine generator dependency cycle | Reject | Generator cannot be compiled before its own result |
| Layout migration with a surviving borrow | Reject activation | Old reference would become invalid |
| Upgrade omitting affected specialized clients | Reject activation | Checked assumptions of old machine code may fail |

# Platform specific SIMD

The wrapping-sum example has a scalar semantic definition and a conditional SIMD rule. Its correctness theorem is parameterized by the target module. Positive cases include empty inputs, inputs shorter than a vector, exact blocks, partial tails, and arithmetic wrapping. A correct implementation handles all lengths through a general proof, not just those examples.

Expected rejection: an unsupported vector instruction without availability evidence, a full-width load beyond the valid tail, or reuse of the wrapping-integer theorem for floating-point reassociation. A scalar fallback keeps the public operation usable on the baseline platform.

# Reading an incomplete example

A named proof module in an upgrade sketch is an obligation to implement, not evidence by naming. An example comment listing proof targets is not an axiom declaration. This distinction is deliberate: the documentation should be useful for implementation without claiming that an unspecified prelude or runtime has already been verified.

# Source examples

These source examples are provided individually and reproduced below. The WIT interface accompanies the component adapter.

## Bounded indexing

```haskell
-- Surface 0.2. Proposed prelude and borrow-pattern rules; not compiled.
module BoundedIndex where
  checkedIndex :: (n :: Nat) -> Nat -> Option (Fin n)
  checkedIndex n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  tryGet :: forall a n region. using (copy :: Copy a) =>
            Ref region (Vec n a) -> Nat -> Option a
  tryGet xs i = case checkedIndex n i of
    Some index -> Some (Vec.getCopy xs index)
    None -> None

  -- For xs = [10, 20, 30], indices 0 and 2 succeed; index 3 returns None.
  -- Comparison stays executable; its successful proof is erased.
```

## Vector filtering

```haskell
-- Surface 0.2. Algorithm; additional sequence laws are proof targets.
module VectorFilter where
  filter :: forall a n region. using (copy :: Copy a) =>
            (a -> Bool) -> Ref region (Vec n a)
            -> Sigma (m :: Nat). Vec m a
  filter predicate xs = case xs of
    Nil -> (0, Nil)
    Cons x rest ->
      let keep = predicate (copyOut x)
          packed = filter predicate rest
      in case packed of
        (m, tail) -> if keep
          then (m + 1, Cons (copyOut x) tail)
          else (m, tail)

  -- Strict let preserves source predicate order.
  -- Borrow matching yields references; copyOut requires Copy evidence.
  -- Prove by induction: m <= n and exact predicate-selected subsequence.
  -- The result retains the witness m, not just an erased proof of existence.
```

## Map fusion

```haskell
-- Surface 0.2. Pure finite reusable lists and total callbacks.
module PureMapFusion where
  proof mapFusion :: (f :: Int -> Int) -> (g :: Int -> Int)
                  -> (xs :: List Int)
                  -> Eq (List.map f (List.map g xs))
                        (List.map (\x -> f (g x)) xs)
  proof mapFusion f g xs = case xs of
    Nil -> refl
    Cons x rest ->
      congr (\ys -> Cons (f (g x)) ys) (mapFusion f g rest)

  rewrite mapMap forall (f :: Int -> Int) (g :: Int -> Int)
                       (xs :: List Int). where
    match List.map f (List.map g xs)
    replace List.map (\x -> f (g x)) xs
    justify mapFusion f g xs

  -- This theorem does not allow effectful callbacks or extend definitional equality.
```

## Productive streams

```haskell
-- Surface 0.2. The guarded Later calculus still needs a formal choice.
module NaturalStreams where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  corec map :: forall a b. (a -> b) -> Stream a -> Stream b
  corec map f xs = Stream
    { head = f xs.head
    , tail = delay (\() -> map f (force xs.tail)) }

  -- delay takes a checked guarded thunk; ordinary strict calls do not create delay.
  -- Finite observations of map (\x -> x * x) (from 0): 0, 1, 4, 9, ...
  -- Bisimulation needs matching heads and guarded tails.
  -- Arbitrary filtering remains a process without recurring-match evidence.
```

## Schema generation

```haskell
-- Surface 0.2. API sketch; no real database connection or credentials.
module SchemaBindings where
  stage derive :: SchemaSnapshot -> Declarations
  stage derive schema = CodecGenerator.derive schema

module Billing where
  import stage SchemaBindings

  generate do
    snapshot <- Stage.snapshot "billing-schema" (\io ->
      Stage.withDatabase io (secret "SCHEMA_DATABASE_URL") ReadSchema
        (\db -> introspectConsistently db))
    emit (SchemaBindings.derive snapshot)

  -- Generator effects are checked against compile-time capabilities.
  -- Refresh captures input, replay uses it, live explicitly contacts a database.
  -- Generated bindings are checked; runtime use needs schema compatibility.
```

## Counter upgrade

```haskell
-- Surface 0.2. State model, not a complete deployable package.
module CounterEvolution where
  record StateV1 where
    value :: Nat
  record StateV2 where
    base :: Nat
    delta :: Nat

  viewV1 :: StateV1 -> Nat
  viewV1 s = s.value
  viewV2 :: StateV2 -> Nat
  viewV2 s = s.base + s.delta
  incrementV1 :: StateV1 %1 -> StateV1
  incrementV1 s = StateV1 { value = s.value + 1 }
  incrementV2 :: StateV2 %1 -> StateV2
  incrementV2 s = StateV2 { base = s.base, delta = s.delta + 1 }
  migrate :: StateV1 %1 -> StateV2
  migrate s = StateV2 { base = s.value, delta = 0 }

  -- Related old next means old.value = next.base + next.delta.
  -- Migration preserves the view by Nat's zero law; increments by addition laws.
  -- Drain-and-replace also needs quiescence, resources and dependency evidence.
```

## Expected rejections

```haskell
-- Independent intended semantic rejections; not one accepted module.
module Rejections where
  badIndex :: Fin 3
  badIndex = Fin 7 refl

  proof impossible :: Void
  proof impossible = impossible

  hiddenSuspend :: Request -> Bytes
  hiddenSuspend request = fetch request

  -- Other required rejections:
  -- A task outliving a borrowed buffer; copying a mutable handle.
  -- An unknown proof hole in a deployable artifact.
  -- An unguarded recursive stream, or filter claiming productivity without progress.
  -- Reusing a bound after mutating the indexed buffer.
  -- An effectful map instantiated at the pure fusion theorem.
  -- An unsupported SIMD module selected before validation.
  -- FFI inside retryable STM, or freeing a cancelled foreign operation's buffer.
  -- Two ambiguous do providers, or changing a closure's captured runtime implicitly.
```

## Platform specific SIMD source

```haskell
-- Surface 0.2. Target APIs and theorem module are obligations, not supplied proofs.
module PlatformSum where
  scalar :: forall n region. Ref region (Vec n U32) -> U32
  scalar xs = Stream.fold U32.zero U32.wrappingAdd (Vec.stream xs)

  target rewrite sumWrap32Vector
    forall t w n region (xs :: Ref region (Vec n U32)).
    using (vectors :: VecU32Target t w) where
    match scalar xs
    replace Simd.sumWrap32 vectors xs
    require Available vectors
    justify SumProofs.scalarToSimd vectors xs

  dispatched :: forall n region. Ref region (Vec n U32) -> U32
  dispatched xs = case Target.detectVecU32 () of
    Some available -> Simd.sumWrap32 available xs
    None -> scalar xs

  -- detectVecU32 reads runtime-established metadata in this proposed pure API.
  -- Bounds, tails, modular arithmetic, intrinsic semantics and target availability
  -- are required proof obligations. Floating-point reassociation needs other laws.
  -- Wasm can require separate variants selected before module instantiation.
```

## 09-foreign-codec.lang

```haskell
module ForeignCodecContract where
  -- Surface 0.2. A fictional codec interface, not an implemented wrapper.
  signature SafeCodec where
    opaque type Decoder :: Type
    record BoundedBytes (limit :: Nat) where
      bytes :: Bytes
      erased bound :: length bytes <= limit
  
    decode :: forall region. Mut region Decoder -> Ref region Bytes
           -> (limit :: Nat) ->{Foreign Codec, Blocking, Diverge}
              Result DecodeError (BoundedBytes limit)
  
  -- The signature intentionally leaves the concrete wrapper implementation abstract.
  -- The implementation must check target size conversions and allocator pairing,
  -- establish pointer access, interpret initialization, and check written <= limit.
  -- A native safety assumption remains unless verified; a postcheck cannot undo
  -- prior corruption. Isolation plus output validation is a separate trust profile.
  -- Cancellation must not free buffers or unload callbacks before acknowledgment.
```

## 10-wasm-component.lang

```haskell
-- Surface 0.2. Adapter sketch for codec.wit; host failure policy is explicit.
module CheckedComponentClient where
  record BoundedBytes (limit :: Nat) where
    bytes :: Bytes
    erased bound :: length bytes <= limit

  decode :: forall region. Ref region Bytes -> (limit :: U32)
         ->{Foreign Component, Diverge}
           Result BoundaryError (BoundedBytes (toNat limit))
  decode input limit = do
    outcome <- Raw.decode input limit
    case outcome of
      Err e -> Err (ForeignError e)
      Ok bytes -> case decide (length bytes <= toNat limit) of
        Yes bound -> Ok (BoundedBytes { bytes = bytes, bound = bound })
        No _ -> Err ContractViolation

  -- Direct do binds the whole Result; it does not automatically short-circuit.
  -- Only the concrete bound is proved, not all-input decompression correctness.
  -- Instance resource identity and trap policy survive adapter generation.
```

## 11-structured-tasks.lang

```haskell
-- Surface 0.2. Ordinary functions; no async/await function category.
module StructuredTasks where
  loadPair :: forall r. using (io :: Io r) =>
              Request -> Request
              ->{Async r, IO r, Cancel r, Diverge}
                Result LoadError (Bytes, Bytes)
  loadPair left right = withScope $ \s -> do
    a <- spawn s (\() -> fetch left)
    b <- spawn s (\() -> fetch right)
    joinAllOrCancel s a b

  -- spawn consumes a thunk; strict argument evaluation must not start fetch early.
  -- Direct sequencing is the default; a and b are task handles.
  -- On supported failure, request sibling cancellation and join cleanup.
  -- Shutdown is not time-bounded without progress assumptions.
  -- Migratable tasks additionally need transfer-safe captures/results.
  -- io is a lexical capability; its region identity is retained by tasks.
```

## codec.wit

```wit
package example:codec@0.1.0;

interface decoder {
  enum decode-error { malformed, too-large }
  decode: func(input: list<u8>, limit: u32)
    -> result<list<u8>, decode-error>;
}

world codec-plugin {
  export decoder;
}
```

## Runtime effects and unqualified do

The effect-polymorphic traversal works with both pure and suspending callbacks. A separate module selects Result sequencing through its signature parameter; the source still uses ordinary `do`.

```haskell
-- Surface 0.2. Proposed APIs, not a compiled program.
module GenericEffects where
  traverse :: forall a b e. (a ->{|e} b) -> List a ->{|e} List b
  traverse f xs = case xs of
    Nil -> Nil
    Cons x rest -> do
      y <- f x
      ys <- traverse f rest
      pure (Cons y ys)

  -- One definition supports pure and suspending callbacks through row e.
  -- Direct sequencing binds ordinary results; callbacks are reusable here.

module ImportConfig (Seq :: ResultSequencing ConfigError) where
  -- The signature marks Seq as this module's sequencing provider.
  loadConfig :: forall r. using (io :: Io r) =>
                Path ->{IO r, Async r, Cancel r, Diverge}
                  Result ConfigError Config
  loadConfig path = do
    bytes <- readConfigBytes path
    config <- parseConfig bytes
    pure config

  -- Same do spelling, explicitly selected Result composition.
  -- io is a lexical runtime argument. Supplying it does not handle effects.
  -- An explicit call can use: loadConfig @{io = chosenIO} path
  -- Handler installation belongs to the entry point or an enclosing runtime scope.
```