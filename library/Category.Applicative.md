# Category.Applicative

Container application and value lifting for independent structure-preserving composition.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Applicative`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Applicative.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Functor](Category.Functor.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Applicative: the applicative class over type constructors.

`pure` and the operator `<*>` (infixl 4, decision D2's per-module
fixity declarations) plus `lift-a2`, which lets an instance express
two-effect combination directly. All three are methods so a lawful
instance can implement whichever pair is primitive (`lift-a2 f x y`
is `(<*>) (map f x) y` for the instances below).

Bare `pure` (decision D153): a `pure` carried by an in-scope
non-provider class constraint answers a bare `pure` first; with no
such constraint and this module imported, a bare `pure` is this
class's method (its constraint is solved at the use site); with this
module unimported the D22 provider meaning stands (`do`'s lexically
selected provider is untouched).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition; this class abstracts over
containers. The two mechanisms do not interact.

Instance homes: the function instance is structural and lives here
permanently; the List, Option and Result instances got theirs at
the prelude split (Data.List/Data.Option/Data.Result; the list
append helper moved to Data.List with them). `map` in the instance
bodies is the Category.Functor method, resolved at the same
container through its instance there.

The combinators below the class are the free functions an
`Applicative` body needs and none of them is a class method (there
are none to inherit): `void`, `when`, `unless`, `ap`, `replicate-m`.
Each states its own constraint and is reached with an explicit
`import Category.Applicative` (the facade re-exports no class home).

</details>

# Types and constructors

# Signatures and methods

<a id="class-4170706c69636174697665"></a>

## `Applicative`

```cinch
signature Applicative f
```

[Source](/reference-sources/Category.Applicative.cinch) · line 36

<a id="method-4170706c696361746976652e70757265"></a>

## `Applicative.pure`

```cinch
pure :: forall (f :: Type -> Type) a . using (dict :: Applicative f) => (a -> (f a))
```

[Source](/reference-sources/Category.Applicative.cinch) · line 37

<a id="method-4170706c696361746976652e3c2a3e"></a>

## `Applicative.<*>`

```cinch
<*> :: forall (f :: Type -> Type) a b . using (dict :: Applicative f) => ((f (a -> b)) -> ((f a) -> (f b)))
```

[Source](/reference-sources/Category.Applicative.cinch) · line 38

<a id="method-4170706c696361746976652e6c6966742d6132"></a>

## `Applicative.lift-a2`

```cinch
lift-a2 :: forall (f :: Type -> Type) a b c . using (dict :: Applicative f) => ((a -> (b -> c)) -> ((f a) -> ((f b) -> (f c))))
```

[Source](/reference-sources/Category.Applicative.cinch) · line 39

# Values

<a id="value-6170"></a>

## `ap`

```cinch
ap :: forall (f :: Type -> Type) a b . using (app :: Applicative f) => ((f (a -> b)) -> ((f a) -> (f b)))
```

`ap` is `(<*>)` under the name `Control.Monad` carries for it.
`base` states it at `Monad` because its body there reaches `pure`;
here `Monad` entails no `Applicative`, so the honest constraint is
the `Applicative` itself, whose `<*>` is this function.

[Source](/reference-sources/Category.Applicative.cinch) · line 76

<a id="value-7265706c69636174652d6d"></a>

## `replicate-m`

```cinch
replicate-m :: forall (f :: Type -> Type) a . using (app :: Applicative f) => (Nat -> ((f a) -> (f (List a))))
```

`replicate-m`: the action run `n` times, its results in order. Zero
runs nothing and yields the empty list inside the effect; n runs
the action left to right through `lift-a2`. The count is a `Nat`
because a length cannot be negative and a count that cannot be
negative needs no refusal.

[Source](/reference-sources/Category.Applicative.cinch) · line 84

<a id="value-756e6c657373"></a>

## `unless`

```cinch
unless :: forall (f :: Type -> Type) . using (app :: Applicative f) => (Bool -> ((f Unit) -> (f Unit)))
```

[Source](/reference-sources/Category.Applicative.cinch) · line 69

<a id="value-766f6964"></a>

## `void`

```cinch
void :: forall (f :: Type -> Type) a . using (fun :: Functor f) => ((f a) -> (f Unit))
```

The free combinators. Each states its constraint: the class has no
default methods, and a `Monad` does not entail an `Applicative`
(the no-superbundle rule), so the constraint named is the one the
body actually uses.

`void` keeps the structure and discards its values (base's
Data.Functor.void). Its home is this module, whose combinators
below discard their results the same way; `()` is a constant, so
nothing is built and nothing is observed.

[Source](/reference-sources/Category.Applicative.cinch) · line 59

<a id="value-7768656e"></a>

## `when`

```cinch
when :: forall (f :: Type -> Type) . using (app :: Applicative f) => (Bool -> ((f Unit) -> (f Unit)))
```

`when`/`unless`: run the action only under the condition. The
branch that does not run still needs a value of the effect's type
and `pure ()` is the only operation either body uses, which is why
the constraint is `Applicative` (base's own constraint for both).

[Source](/reference-sources/Category.Applicative.cinch) · line 66

# Operator fixities

```cinch
infixl 4 <*>
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Category.Applicative.Applicative_Function` | `Applicative (((->) r))` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Applicative.cinch`. Source SHA-256: `430133dda538b4758b3f4c24dc2b0ceb162a3573a44b50a596a7b3e2696ca18a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
