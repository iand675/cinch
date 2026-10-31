# Category.Functor

Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Functor`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Functor.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Function.Combinators](Function.Combinators.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Functor: the functor class over type constructors.

The method is `map` (not `fmap`; decision D152's lisp-style naming),
and `<$>` is its operator spelling (decision D2: operators are class
methods with per-module fixity declarations, so `infixl 4 <$>`
below is the declaration consumers import). Both methods have the
same type; a lawful instance defines them identically.

Class shape: `Functor (f :: Type -> Type)` with the methods at the
applied container `f a` (higher-kinded parameters, modules
specification section 1).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition, through a `using` binder with a
DirectSequencing/ResultSequencing/OptionSequencing head (default:
DirectSequencing). A do-provider is a fixed instance choice: exactly
one container per definition, no polymorphism. This class abstracts
over containers for polymorphic code; its methods are ordinary class
methods resolved per use site. The two mechanisms do not interact:
`do` never consults the class, and a class constraint never selects
a do provider.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module" (design/base-library.md). The function instance below
is structural and lives here permanently. The List, Option and
Result instances got theirs at the prelude split: they live in
Data.List/Data.Option/Data.Result now.

</details>

# Types and constructors

# Signatures and methods

<a id="class-46756e63746f72"></a>

## `Functor`

```cinch
signature Functor f
```

[Source](/reference-sources/Category.Functor.cinch) · line 33

<a id="method-46756e63746f722e6d6170"></a>

## `Functor.map`

```cinch
map :: forall (f :: Type -> Type) a b . using (dict :: Functor f) => ((a -> b) -> ((f a) -> (f b)))
```

[Source](/reference-sources/Category.Functor.cinch) · line 34

<a id="method-46756e63746f722e3c243e"></a>

## `Functor.<$>`

```cinch
<$> :: forall (f :: Type -> Type) a b . using (dict :: Functor f) => ((a -> b) -> ((f a) -> (f b)))
```

[Source](/reference-sources/Category.Functor.cinch) · line 35

# Operator fixities

```cinch
infixl 4 <$>
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Category.Functor.Functor_Function` | `Functor (((->) r))` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Functor.cinch`. Source SHA-256: `2b1b8078f2abdf140b110074f4f80f22ba2b6ea54d72baf15125ea8311639ba1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
