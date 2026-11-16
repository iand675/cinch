# Retaining array operands and frame lifetime

An ARC-borrowed argument need not be confined to the call. The callee may
retain it and return the new owner. The runtime array setters and replication
do exactly that. Treating their borrowed operands as proof of frame
noncapture could place a managed constructor in the caller's frame, retain
that frame pointer in an array, and release its payload immediately after the
setter returned. This failure also occurs with a nonrecursive `Box Bytes`;
recursive array admission is not required to trigger it.

The repair leaves `Program.borrowed`, `Program.lent`, imported ABI metadata,
`prim_consumes`, and runtime reference transfer unchanged. The stack planner
derives private modes instead. Actual typed primitive operations supply
primitive authority. Ordinary Core functions retain only modes whose parameter
uses do not escape under those private facts. A shrinking worklist revisits
callers whenever a callee loses a mode, including ordinary helper chains.
Opaque imports, foreign imports, and bodies marked as primitive implementation
stubs supply no noncapture proof from their legacy lending metadata.

The inference scans parameter lifetimes relative to the caller's frame. A
read-only helper may tail-forward a parameter borrowed from its own caller.
This does not change the existing prohibition on tail-transferring a freshly
allocated block from the allocating function's frame.

## Typed primitive audit

Positions below are zero-based source/runtime operands, excluding specialized
representation metadata supplied by code generation.

| Family | Operands without frame noncapture proof | Reason |
| --- | --- | --- |
| Array Replicate | 1 | The new buffer retains the element. |
| Array Set | 0, 2 | The result retains the element; a unique update may return the original array identity. |
| Tensor Set | 2 | The copied data array retains the replacement element. |
| Tensor Reshape | 0, 1 | The tensor retains the shape and data arrays themselves. |
| Columnar Column | 0 | The column retains the source array itself. |
| Columnar Zip | 0, 1 | The result retains both columnar children. |
| STM CellNew | 0 | The stored value is transferred into the cell. Its ARC mode already consumes it. |
| STM CellCast | 0 | The operation returns the same cell identity. Its ARC mode already consumes it. |
| STM CellAddr | 0 | The result exposes the cell address. |
| Slots Write | 2 | The value is transferred into a persistent slot. Its ARC mode already consumes it. |
| Delay | 0 | The result is the closure itself; its ARC mode already consumes it. |
| Bytes Data | 0 | The result exposes the buffer address. |
| Concurrent Spawn | all | The spawned task can outlive the allocating activation. The existing scan already refuses this transfer. |

Array FromList, Slice, and Append copy element owners rather than retaining
the container argument. Array Get, tensor Shape/Data/Get, columnar Left/Right/
Leaf/Row, and Slots Read/Take return child owners, not the container's
identity. Length, dimension, index, prefetch, clear, and buffer-copy operations
read or mutate their containers without storing those container identities.
Slots Move transfers stored elements between existing containers; it does not
retain either slot-array handle. STM Hold/Open return the separately supplied
word; they do not return or retain their cell operand. Force and ApplyErased
already consume the closure and provide no primitive borrowed mode. Numeric,
UTF-8, symbol, text, platform, network, process, and PTY primitives do not
accept arbitrary managed ADT or closure operands. Synchronous parallel
operations join their workers before returning; unlike Spawn, they do not
extend the supplied closures' lifetime beyond the call.

This is not a subtyping or ownership-convention change. It adds no interface
field and grants no authority from a source name. Array operations are matched
exhaustively so another array operation requires an explicit audit decision.

## Preserved evidence and gates

`array-frame-capture-baseline-results.json` records the pre-repair independent
`Box Bytes` Array Set reproduction produced by frozen Rust release
`ae9c3f812404bd43020346ee4b7464ebf906fe4263932d011d99b8fd9301334a`
and its matched runtime
`80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141`.
Default frame allocation exits 71 in both normal and poisoned-free profiles;
the same source with `--no-stack-alloc` exits 0 with no leaks in both profiles.
Source, LLVM, executable, and diagnostic hashes are retained in that receipt.
The original artifacts remain under
`build/array-capture-probes/set-direct/` in this lane.

The separate experimental Native emitter
`645c518560ae5565e9713446e96140d1f8adb0fbccce05a0e37b3f72e7c5e17d`
already runs the finite recursive Array managed-owner fixture in both
profiles with no leaks. That is a Rust-produced experimental emitter result,
not a self-built distribution or fixed-point claim.

The maintained regression is `compiler/tests/stack_array_capture.rs`. It
builds the same source with default frame allocation and with frame allocation
disabled. Both executables run normal 32 MiB heap / 8 MiB stack and poisoned
256 MiB heap / 8 MiB stack profiles. Direct and helper-forwarded Set,
Replicate, and TensorSet cases inspect retained managed payloads. Independent
managed and raw read-only controls must still report actual frame slots, so
globally disabling frame allocation cannot satisfy the test.

The focused static checkpoint requires these gates before publication:

```
cargo test --lib frame_capture_tests
cargo test --test stack_array_capture
cargo test --test nested_array_positivity recursive_array_runtime_owners_survive_persistent_aliases
cargo test --test stack_allocation
```

The parent integration lane owns Cargo execution. Syntax formatting and
whitespace checks alone are not a claim that these gates have passed.
