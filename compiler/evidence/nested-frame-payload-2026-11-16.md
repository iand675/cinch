# Nested frame payload lifetime

`Run.NestedFramePayload.cinch` reduces a CSV optimization failure to two
constructors and two ordinary borrowed accessors. `Inner` owns a heap byte
buffer, `Outer` owns an `Inner`, and `take-inner` returns an owned reference
to its input's child. The caller drops `Outer` before reading the returned
`Inner`.

Both the original Rust CLI (`f2d5ae4c...`) and the compiler containing the
two strict-order corrections (`16f2f4bf...`) fail this source at O2 with
exit 71, in normal 32 MiB heap / 8 MiB stack and RC poisoning 256 MiB heap /
8 MiB stack. The latter compiler passes both modes with
`--no-stack-alloc`, with zero leaks. The full original CSV boundary control
also fails both modes and passes with stack allocation disabled. Runtime
archive `c4b1097b...` is frozen and explicitly linked in these checks.
The JSON receipt retains complete hashes, outputs, and the minimal Core.

The relevant lowered ownership sequence is:

```text
inner = Inner (bytes-new 1)             -- frame slot 0
before = length-inner inner            -- borrowed read
outer = Outer inner                    -- frame slot 1 owns inner
detached = take-inner outer            -- accessor duplicates the field
drop outer                             -- releases inner's counted payload
after = length-inner detached          -- reads the already released buffer
```

The accessor's Core is `case outer of Outer child -> dup child; child`.
The caller's nested frame child has no explicit `Dup`, so its scan alone
does not see the new reference. A frame `Dup` does not retain its payload:
there is no count to keep that payload alive after the enclosing owner's
release. This is a planning error in the nested-store widening, and does
not require changing the generic `Release` emission protocol.

Review added `Run.ReturnedFrameProjection.cinch`: a getter can also return a
raw-payload child beyond the producer's activation. The original producer's
LLVM returns a pointer into its own `alloca`, even when a run happens to see
the previous stack bytes unchanged. Absence of counted payload protects
neither the frame address nor its contents after the activation ends.

The correction therefore keeps every nested child on the heap, giving a
projected result an independent lifetime. The parent can still take a frame
slot. Direct duplication of a standalone block without counted payload and
safe consuming-callee placements retain their existing rules. Counted and
raw nested controls expect one parent slot; the returned-child control also
checks that the producer's child allocation does not use frame storage.
Runtime regressions compare normal placement with disabled stack allocation.

This is deliberately conservative: a nested child stays on the heap even
when a particular local accessor could transfer it safely.
Recovering those placements requires tracking field aliases and their
independent lifetimes across enclosing owners and callees. The guard uses
the existing `Duplicated` refusal class for these independently aliased
values. The initial counted-only proposal and its successful local tests
remain historical evidence, rather than proof for an escaping raw result.
