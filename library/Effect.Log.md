# Effect.Log

Structured log operations and records with runtime capability/effect accounting.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Log`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Log.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Json](Data.Json.md), [Data.List](Data.List.md).

Non-embedded source closure: [Effect.Log](Effect.Log.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Log: the `Log r` signal vocabulary (the P32 observability
rows): log levels, metric kinds, the `log`/`metric` effect
operations, and the structured `Json` records they carry.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary. The observability story is two families in the
`Io r` mold (the runtime capabilities, effects and sequencing
amendments, D21/D81): region-parameterized effect labels over the
runtime identity `r`, so a signature quantifies `forall r.` and its
body row carries `{Log r}` (here) and `{Trace r}` (Effect.Trace).
The operations are ordinary functions performing their label; a
handler interprets them (the P6 story), and the emitting evidence is
the ordinary `Io r` capability the handler holds. Unhandled, the
operations are just their effect rows and `main`'s boundary refuses
them; that is the whole capability story.

THE RECORDS (reported): a `log`/`metric` payload is a structured
`Json` record (`log-record`/`metric-record` build the shapes);
fields are a `Json` object built with `fields`/`num`/`str`/`flag`/
`arr` so a call site stays free of ceremony. The record shapes are
the contract `Text.Otlp` exports over the wire (OTLP/JSON).

</details>

# Types and constructors

<a id="type-4c6f674c6576656c"></a>

## `LogLevel`

```cinch
data LogLevel
```

The level of a log record. The lowercase values (`info` and
friends) are the DX spelling at a `log` call; the constructors
surface (D308) for handler clauses that match on them.

[Source](/reference-sources/Effect.Log.cinch) · line 36

```cinch
Debug :: LogLevel
```

```cinch
Info :: LogLevel
```

```cinch
Warn :: LogLevel
```

```cinch
Error :: LogLevel
```

<a id="type-4d65747269634b696e64"></a>

## `MetricKind`

```cinch
data MetricKind
```

The instrument family of a metric record: a counter accumulates
deltas, a gauge carries the latest reading, a histogram observes
values into buckets. The lowercase values are the DX spelling at a
`metric` call.

[Source](/reference-sources/Effect.Log.cinch) · line 67

```cinch
Counter :: MetricKind
```

```cinch
Gauge :: MetricKind
```

```cinch
Histogram :: MetricKind
```

# Values

<a id="value-617272"></a>

## `arr`

```cinch
arr :: (List Json -> Json)
```

[Source](/reference-sources/Effect.Log.cinch) · line 117

<a id="value-636f756e746572"></a>

## `counter`

```cinch
counter :: MetricKind
```

[Source](/reference-sources/Effect.Log.cinch) · line 72

<a id="value-6465627567"></a>

## `debug`

```cinch
debug :: LogLevel
```

[Source](/reference-sources/Effect.Log.cinch) · line 42

<a id="value-6572726f72"></a>

## `error`

```cinch
error :: LogLevel
```

[Source](/reference-sources/Effect.Log.cinch) · line 51

<a id="value-6669656c6473"></a>

## `fields`

```cinch
fields :: (List (String, Json) -> Json)
```

[Source](/reference-sources/Effect.Log.cinch) · line 102

<a id="value-666c6167"></a>

## `flag`

```cinch
flag :: (Bool -> Json)
```

[Source](/reference-sources/Effect.Log.cinch) · line 114

<a id="value-6761756765"></a>

## `gauge`

```cinch
gauge :: MetricKind
```

[Source](/reference-sources/Effect.Log.cinch) · line 75

<a id="value-686973746f6772616d"></a>

## `histogram`

```cinch
histogram :: MetricKind
```

[Source](/reference-sources/Effect.Log.cinch) · line 78

<a id="value-696e666f"></a>

## `info`

```cinch
info :: LogLevel
```

[Source](/reference-sources/Effect.Log.cinch) · line 45

<a id="value-6c6576656c2d6e616d65"></a>

## `level-name`

```cinch
level-name :: (LogLevel -> String)
```

The level's text in a record (`"debug"`, `"info"`, `"warn"`,
`"error"`).

[Source](/reference-sources/Effect.Log.cinch) · line 56

<a id="value-6c6f672d7265636f7264"></a>

## `log-record`

```cinch
log-record :: (LogLevel -> (String -> (Json -> Json)))
```

A log record: `{"level": ..., "message": ..., "fields": ...}`.

[Source](/reference-sources/Effect.Log.cinch) · line 123

<a id="value-6d65747269632d6b696e642d6e616d65"></a>

## `metric-kind-name`

```cinch
metric-kind-name :: (MetricKind -> String)
```

[Source](/reference-sources/Effect.Log.cinch) · line 81

<a id="value-6d65747269632d7265636f7264"></a>

## `metric-record`

```cinch
metric-record :: (MetricKind -> (String -> (Int -> (Json -> Json))))
```

A metric record: `{"metric": ..., "name": ..., "value": ...,
"fields": ...}`.

[Source](/reference-sources/Effect.Log.cinch) · line 131

<a id="value-6e6f2d6669656c6473"></a>

## `no-fields`

```cinch
no-fields :: Json
```

The field payload of a record: a `Json` object. `no-fields` is the
empty payload; `fields` builds one from its pairs (the DX shape:
`fields [("port", num 8080), ("host", str "localhost")]`); `num`,
`str` and `flag` are the scalar spellings; `arr` is a `Json`
array.

[Source](/reference-sources/Effect.Log.cinch) · line 99

<a id="value-6e756d"></a>

## `num`

```cinch
num :: (Int -> Json)
```

[Source](/reference-sources/Effect.Log.cinch) · line 108

<a id="value-737472"></a>

## `str`

```cinch
str :: (String -> Json)
```

[Source](/reference-sources/Effect.Log.cinch) · line 111

<a id="value-7761726e"></a>

## `warn`

```cinch
warn :: LogLevel
```

[Source](/reference-sources/Effect.Log.cinch) · line 48

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Log.cinch`. Source SHA-256: `c32dae495122e7b3963d8fa2fd19fab81031f29093493080206d0191e7db09ed`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
