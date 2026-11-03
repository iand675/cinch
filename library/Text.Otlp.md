# Text.Otlp

OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Otlp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Otlp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Effect.Log](Effect.Log.md), [Effect.Trace](Effect.Trace.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Effect.Log](Effect.Log.md), [Effect.Trace](Effect.Trace.md), [Text.Otlp](Text.Otlp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Otlp: the observability wire formats (the P32
structured/OTLP/W3C story).

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes. This module is the P32 wire layer over the
`Effect.Log`/`Effect.Trace` record shapes and `Text.Json`'s canonical
printer:

  * the W3C `traceparent` header parsed and formatted around
    `Effect.Trace`'s `TraceContext` (trace context recommendation,
    section 3.2: `version-traceid-spanid-flags`, lowercase
    hexadecimal, `ff` version forbidden, zero trace/span ids
    forbidden, version `00` exactly the four fields and later
    versions parsed through their first 55 characters);
  * the OTLP/JSON export envelope (`otlp-export`) built over
    `Text.Json`: the collected records (log, metric and nested span
    records) become `resourceLogs`/`resourceMetrics`/
    `resourceSpans` in the protobuf-JSON mapping shape (JSON only;
    the protobuf binary encoding is a named boundary, not this
    module's job).

EXPORT ASSIGNMENT (reported): the trace context supplies the trace
id, the parent span id and the trace flags; the export walk assigns
each span a span id (the visit index zero-padded to 16 decimal
digits, which is valid lowercase hex) and nests children under their
parent, so logs and metrics inside a scope carry the enclosing span
id. Times (`timeUnixNano`) are deliberately absent: the P34
platform layer owns the clock and the export shape does not invent
one.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-616e792d76616c7565"></a>

## `any-value`

```cinch
any-value :: (Json -> Json)
```

One `Json` value as an OTLP `AnyValue` (the protobuf-JSON
mapping): strings, int64-as-string, doubles, bools, arrays as
`arrayValue`, objects as `kvlistValue`, `JNull` as the empty
value.

[Source](/reference-sources/Text.Otlp.cinch) · line 276

<a id="value-61747472696275746573"></a>

## `attributes`

```cinch
attributes :: (Json -> Json)
```

Json to OTLP values

A record's fields as OTLP attributes: `[{"key": ..., "value":
<AnyValue>}, ...]` in object-spine order.

[Source](/reference-sources/Text.Otlp.cinch) · line 263

<a id="value-666f726d61742d7472616365706172656e74"></a>

## `format-traceparent`

```cinch
format-traceparent :: forall r . (TraceContext r -> String)
```

Format the context back to its `traceparent` header: parse and
format round trip exactly (the header is canonical lowercase).

[Source](/reference-sources/Text.Otlp.cinch) · line 84

<a id="value-6f746c702d6578706f7274"></a>

## `otlp-export`

```cinch
otlp-export :: forall r . (TraceContext r -> (Json -> (List Json -> Json)))
```

The OTLP/JSON export envelope (protobuf-JSON mapping, JSON only)

The envelope over the collected records: `resourceLogs` carries
the log records, `resourceMetrics` the metric records and
`resourceSpans` the span trees (flattened with parent links). The
trace context stamps every signal: its trace id, the parent span
id of the roots (the incoming span) and the span's trace flags.

[Source](/reference-sources/Text.Otlp.cinch) · line 140

<a id="value-70617273652d7472616365706172656e74"></a>

## `parse-traceparent`

```cinch
parse-traceparent :: forall r . (String -> Option (TraceContext r))
```

W3C traceparent (the header grammar: version-traceid-spanid-flags)

Parse a `traceparent` header into the trace context of some
identity `r`. Refused (as `None`) unless the header is exactly the
section 3.2 grammar: 55 lowercase-hex characters in four
dash-delimited fields (a later version may carry trailing fields,
which are ignored), version not `ff`, version `00` with no
trailing data, and neither id all zeros.

[Source](/reference-sources/Text.Otlp.cinch) · line 65

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Otlp.cinch`. Source SHA-256: `650ee7a5b2e4c12c7c01d4218d520cc4dbd97432fedd85291c8831abdf543b9d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
