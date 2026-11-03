# Data.Producer

Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Producer`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Producer.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Producer: the vector step form and the producer library.

The step form (modules and compilation specification section 6):
`Done`, `Skip(state)` and `Yield(value, state)`. Producer pipelines
lower to state machines; the fusion rules compose them. `vec-stream`
is a producer over a borrowed vector: the block walk of the step
form is the `vecStreamStep` intrinsic. `scalar-sum-wrap32` is the
scalar model (platform proofs and SIMD specification section 5):
the sum modulo 2^32 over the borrowed vector.

</details>

# Types and constructors

<a id="type-50726f6475636572"></a>

## `Producer`

```cinch
data Producer (s :: Type) (a :: Type)
```

A producer: a state with its step function.

[Source](/reference-sources/Data.Producer.cinch) · line 23

```cinch
Producer :: forall s a . ((s -> Step s a) -> (s -> Producer s a))
```

A producer: a state with its step function.

<a id="type-53746570"></a>

## `Step`

```cinch
data Step (s :: Type) (a :: Type)
```

The vector step form (modules section 6): `Done`, `Skip(state)` and
`Yield(value, state)`. Producer pipelines lower to state machines.

[Source](/reference-sources/Data.Producer.cinch) · line 17

```cinch
Done :: forall s a . Step s a
```

```cinch
Skip :: forall s a . (s -> Step s a)
```

```cinch
Yield :: forall s a . (a -> (s -> Step s a))
```

<a id="type-54616b655374"></a>

## `TakeSt`

```cinch
data TakeSt (s :: Type)
```

[Source](/reference-sources/Data.Producer.cinch) · line 34

```cinch
TakeSt :: forall s . (Nat -> (s -> TakeSt s))
```

# Values

<a id="value-70726f642d666f6c64"></a>

## `prod-fold`

```cinch
prod-fold :: forall s a b . (b -> ((b -> (a -> b)) -> (Producer s a -> b)))
```

[Source](/reference-sources/Data.Producer.cinch) · line 56

<a id="value-70726f642d666f6c642d676f"></a>

## `prod-fold-go`

```cinch
prod-fold-go :: forall s a b . ((s -> Step s a) -> ((b -> (a -> b)) -> (b -> (s -> b))))
```

[Source](/reference-sources/Data.Producer.cinch) · line 50

<a id="value-70726f642d6d6170"></a>

## `prod-map`

```cinch
prod-map :: forall s a b . ((a -> b) -> (Producer s a -> Producer s b))
```

[Source](/reference-sources/Data.Producer.cinch) · line 26

<a id="value-70726f642d74616b65"></a>

## `prod-take`

```cinch
prod-take :: forall s a . (Nat -> (Producer s a -> Producer (TakeSt s) a))
```

[Source](/reference-sources/Data.Producer.cinch) · line 46

<a id="value-70726f642d74616b652d73746570"></a>

## `prod-take-step`

```cinch
prod-take-step :: forall s a . ((s -> Step s a) -> (TakeSt s -> Step (TakeSt s) a))
```

[Source](/reference-sources/Data.Producer.cinch) · line 37

<a id="value-7363616c61722d73756d2d777261703332"></a>

## `scalar-sum-wrap32`

```cinch
scalar-sum-wrap32 :: forall (n :: Nat) region . (Ref region Vec n U32 -> U32)
```

[Source](/reference-sources/Data.Producer.cinch) · line 65

<a id="value-7665632d73747265616d"></a>

## `vec-stream`

```cinch
vec-stream :: forall a (n :: Nat) region . (Ref region Vec n a -> Producer (Ref region Vec n a) a)
```

A producer over a borrowed vector: the block walk of the step form
is the `vecStreamStep` intrinsic.

[Source](/reference-sources/Data.Producer.cinch) · line 62

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Producer.cinch`. Source SHA-256: `47b6a989c18527ed19e80914ff23860e4421c239eba407effa85fe855f94f30f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
