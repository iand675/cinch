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

The focused correction keeps a counted-payload child on the heap whenever
it is stored inside another frame block. Its heap count then protects the
independent field reference. Direct placements in borrowed or consuming
callees retain their existing rules; nested children without counted
payload still retain their frame optimization. The existing counted nested
store control now expects the holder's frame slot and a heap child, while
retaining its runtime and leak checks. A separate raw-word nested control
pins the two-slot optimization, and the reduced executable regression
compares normal placement with disabled stack allocation in both modes.

This is deliberately conservative: a counted nested child stays on the
heap even when a particular local accessor could transfer it safely.
Recovering those placements requires tracking field aliases and their
independent lifetimes across enclosing owners and callees. The guard uses
the existing `Duplicated` refusal class because the hidden alias has the
same payload ownership requirement as an explicit duplication.
