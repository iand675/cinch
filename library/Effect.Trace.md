# Effect.Trace

Scoped tracing/span vocabulary and context propagation tied to the runtime identity.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Trace`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Trace.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.String](Data.String.md), [Effect.Log](Effect.Log.md).

Non-embedded source closure: [Effect.Log](Effect.Log.md), [Effect.Trace](Effect.Trace.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Trace: the `Trace r` scope vocabulary (the P32 observability
rows): spans as lexical scopes, and the W3C trace context carried by
the runtime identity `r`.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary; this family is the scope signals beside
`Effect.Log`'s point signals. `span`/`end-span` are ordinary
operations in the `Io r` mold (D21/D81): `{Trace r}` on the row of
`forall r.` signatures, interpreted by a handler, refused at
`main`'s boundary when unhandled.

SPANS AS LEXICAL SCOPES (reported): `span name fields` opens the
scope and `end-span name` closes it, so a scope is the lexical
region between the two statements (a bracketed block of the
computation, named at both ends). A `span` whose scope runs to the
end of the handled computation needs no close (the tail scope), so
nested spans read innermost-last with no ceremony. The collecting
handler gets this shape for free: the close marker rides the
resumption chain (`end-marker`), and `span-scope` wraps exactly the
records up to the matching close (or, with no matching close, the
whole tail) in the span record. This is the `withScope` shape of the
roadmap under the bootstrap handler calculus (D23): dispatch is
lexical to the handled computation's own expression, so a
body-taking `span` could not carry handled operations in its body;
the bracket pair is the honest lexical scope until call-dispatched
handling lands.

THE TRACE CONTEXT (reported): the W3C traceparent fields (version,
trace-id, span-id, trace flags) indexed by the runtime identity
(`TraceContext r`): a context is built for one `r` and every
function that carries it quantifies the same `r`, so the context
travels with the identity the `{Log r}`/`{Trace r}` rows name. The
header text conversion lives in `Text.Otlp` (the wire formats).

</details>

# Types and constructors

<a id="type-5472616365436f6e74657874"></a>

## `TraceContext`

```cinch
data TraceContext (r :: Type)
```

The W3C trace context of runtime identity `r` (trace context
recommendation, section 3.2): the traceparent header's fields,
version (`0`-`255`, `255` is the forbidden `ff`), the 32
lowercase-hex trace id, the 16 lowercase-hex parent span id and
the trace flags (`0`-`255`), all indexed by the identity the
context propagates through.

[Source](/reference-sources/Effect.Trace.cinch) · line 59

```cinch
TraceContext :: forall r . (Nat -> (String -> (String -> (Nat -> TraceContext r))))
```

The W3C trace context of runtime identity `r` (trace context
recommendation, section 3.2): the traceparent header's fields,
version (`0`-`255`, `255` is the forbidden `ff`), the 32
lowercase-hex trace id, the 16 lowercase-hex parent span id and
the trace flags (`0`-`255`), all indexed by the identity the
context propagates through.

# Values

<a id="value-656e642d6d61726b6572"></a>

## `end-marker`

```cinch
end-marker :: (String -> Json)
```

The scope-end marker a collecting handler's `end-span` clause
conses onto the resumption chain; `span-scope` consumes it. The
name is the scope being closed (the bracket pair's check).

[Source](/reference-sources/Effect.Trace.cinch) · line 74

<a id="value-7370616e2d7265636f7264"></a>

## `span-record`

```cinch
span-record :: (String -> (Json -> (List Json -> Json)))
```

A span record: `{"name": ..., "fields": ..., "children": [...]}`,
where children are the collected records of the scope (log,
metric and nested span records).

[Source](/reference-sources/Effect.Trace.cinch) · line 65

<a id="value-7370616e2d73636f7065"></a>

## `span-scope`

```cinch
span-scope :: (String -> (Json -> (List Json -> List Json)))
```

The collecting handler's `span` clause body: wrap `scope` (the
resumption's records) in the span record, children ending at the
matching close marker (or the tail when the scope is unclosed).

[Source](/reference-sources/Effect.Trace.cinch) · line 80

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Trace.cinch`. Source SHA-256: `fe464ff8726132226cb84a56a1cd6f985553d0b4ab0118ca68922d98f8923366`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
