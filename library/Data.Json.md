# Data.Json

The JSON value representation shared by parsers, renderers, codecs, and structured observability.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Json`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Json.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Json: the `Json`-shaped value a file converter binds.

With no declared type a JSON file binds this shape; with a declared
record type the converter decodes into it instead (surface grammar
local amendments section 11). An array is a cons spine (`JArr item
tail`, ended by `JArrEnd`) and an object entry is `JObj key value
tail` (ended by `JObjEnd`): the positivity discipline allows only
direct recursive occurrences, so the payloads stay inside the
family.

</details>

# Types and constructors

<a id="type-4a736f6e"></a>

## `Json`

```cinch
data Json
```

The `Json`-shaped value a file converter binds for a JSON file with
no declared type (surface grammar local amendments section 11); with
a declared record type the converter decodes into it instead. An
array is a cons spine (`JArr item tail`, ended by `JArrEnd`) and an
object entry is `JObj key value tail` (ended by `JObjEnd`): the
positivity discipline allows only direct recursive occurrences, so
the payloads stay inside the family.

[Source](/reference-sources/Data.Json.cinch) · line 23

```cinch
JNull :: Json
```

```cinch
JBool :: (Bool -> Json)
```

```cinch
JNum :: (Int -> Json)
```

```cinch
JFloat :: (F64 -> Json)
```

```cinch
JStr :: (String -> Json)
```

```cinch
JArr :: (Json -> (Json -> Json))
```

```cinch
JObj :: (String -> (Json -> (Json -> Json)))
```

```cinch
JArrEnd :: Json
```

```cinch
JObjEnd :: Json
```

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Json.cinch`. Source SHA-256: `6af8ff9d91b25b53c034b1626efe76bf1a6e277172920eabbd8c21e4c0781e3b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
