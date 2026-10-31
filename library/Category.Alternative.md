# Category.Alternative

Empty and choice operations for admitted higher-kinded containers.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Alternative`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Alternative.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Alternative: the class of containers with an identity for
choice.

`empty` is the nullary method (the dictionary carries the value
itself) and `<|>` is the choice operator (infixl 3, decision D2's
per-module fixity declarations). As in Haskell the class sits over
Applicative (the import below).

`Alternative (Result e)` is omitted by design: its `empty` needs a
distinguished error value, which is a Monoid on `e`'s error type;
it belongs with the Monoid integration, not here.

Instance homes: the List and Option instances got theirs at the
prelude split (Data.List/Data.Option).

</details>

# Types and constructors

# Signatures and methods

<a id="class-416c7465726e6174697665"></a>

## `Alternative`

```cinch
signature Alternative f
```

[Source](/reference-sources/Category.Alternative.cinch) · line 19

<a id="method-416c7465726e61746976652e656d707479"></a>

## `Alternative.empty`

```cinch
empty :: forall (f :: Type -> Type) a . using (dict :: Alternative f) => (f a)
```

[Source](/reference-sources/Category.Alternative.cinch) · line 20

<a id="method-416c7465726e61746976652e3c7c3e"></a>

## `Alternative.<|>`

```cinch
<|> :: forall (f :: Type -> Type) a . using (dict :: Alternative f) => ((f a) -> ((f a) -> (f a)))
```

[Source](/reference-sources/Category.Alternative.cinch) · line 21

# Operator fixities

```cinch
infixl 3 <|>
```

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Alternative.cinch`. Source SHA-256: `4d05c93facf9e53c06dfbdca83d64da51c945af15373f3658390ae90a5d4bcdc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
