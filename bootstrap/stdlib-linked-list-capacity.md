# Physical linked-list capacity bounds

`Data.Mutable.LinkedList` previously bounded slot counts by `2^63 - 1`.
That bounds an index, but does not bound the backing allocation. A slot array
needs eight bytes per slot, sixteen bytes of array metadata, and up to sixteen
bytes of block headers. Its aligned allocation must fit the target's signed
address range.

The maximum representable capacity is therefore:

```
floor((target_isize_max - 32) / 8)
```

| Pointer width | Maximum slots | First illegal count |
| --- | ---: | ---: |
| 32 | 268,435,451 | 268,435,452 |
| 64 | 1,152,921,504,606,846,971 | 1,152,921,504,606,846,972 |

At the maximum, the complete allocation occupies `isize_max - 7` bytes,
the largest eight-byte-aligned layout. The next count would require exactly
`isize_max + 1` bytes. The module now calculates the bound from the target's
`Usize` maximum. Geometric growth clamps at that same physical limit instead
of doubling a legal capacity into an illegal layout.

The exported surface and its 21 reference operation records are unchanged.
Constructors and reserve return `ListCapacityExceeded` before allocation or
mutation when the requested logical size cannot fit. Resource exhaustion for
a representable allocation still follows normal allocator behavior. Link
indices, stable identities, generation checks, detached values, and bulk
ownership moves are unchanged. This is a capacity-correctness repair, with no
performance claim or new SIMD kernel.

The public boundary consumer checks the exact first illegal constructor size,
larger machine and mathematical counts, and overflowing reserve requests for
both empty and populated lists. Failed reserve must preserve capacity,
membership, value, length, and element identity. A separate source projection
tests the actual private growth helpers near the limit without allocating
those enormous capacities. It checks the native target plus the same helper
bodies with 32-bit and 64-bit target-maximum primitive values. Expected limits
and growth results use independent host-integer calculations. Modeled input
checks do not claim execution on a 32-bit target.

Validation uses explicit distinct frozen epochs: repaired Rust `b004049b`
with its selected `c4b1097b` runtime, and the full combined Native `2f07707e`
with its matched `80bd5b54` runtime and base provider. LinkedList has no
recursive arena carrier, so that Native family needs no Array positivity
extension. The validators preserve normal 32 MiB heap / 8 MiB stack and
separate poisoned-free 256 MiB heap / 8 MiB stack runs. The Rust runner now
selects its runtime archive explicitly and uses supported Rust CLI flags;
Native retains its own supported generated count-check flag.

The maintained `lib_linked_list` target adds physical-boundary and private
growth tests to the existing complete independent mutation traces and owned
handle controls. Exact before/after evidence and final corpus receipts follow
the frozen artifacts rather than implying that different compiler epochs are
interchangeable.


The final receipt, `stdlib-linked-list-capacity-validation-results.json`,
records all eight 96-action traces, the unchanged original owned-handle
controls, exact public capacity refusals, and private growth arithmetic:
11 normal and 11 poisoned processes per frontend, all with zero leaks.
The same new boundary consumer against the old module exits 73 at the first
illegal capacity in both profiles and both frontends; it reaches the bounded
allocator's layout refusal rather than returning the named error. That
before-state observation is retained separately and makes no zero-leak claim
for an early allocator exit.

A direct-literal arithmetic model also exposed the older Rust producer's
unsigned constant-folder defect: its all-ones logical-shift expression emits
`ret i64 -5` instead of the expected 64-bit physical limit. Native produces
the correct result. The failure source/LLVM identities remain in
`stdlib-linked-list-capacity-older-fold-observation.json`; the parent owns a
separate compiler repair. Production capacity arithmetic uses the target
primitive and passes the complete controls. The final growth test presents
modeled primitive inputs through a pure LLVM identity, preventing source
constant folding while allowing ordinary LLVM optimization. Both frontends
consume the identical fixture and independent expected values. No compiler
fix or unsigned constant-folder coverage is claimed by this library commit.
