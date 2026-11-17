# Sealed ASCII run observation

`byte-view-ascii-space-run-end :: ByteView -> Nat -> Bool -> Option Nat`
observes the maximal selected ASCII class run from an owner-relative byte
position. `True` selects `09..0D` and `20`; `False` selects other ASCII bytes.
Either class stops before a non-ASCII byte. The logical end returns `Some end`;
an offset beyond it returns `None`, including boxed offsets larger than a
machine word. The operation retains the complete fixed owner and observes alias
mutations without copying or exposing backing pointers.

The implementation checks bounds and holds the owner outside ordinary private
I64 loops. A complete available 16-byte block uses an unaligned LLVM vector
load and straight-line classification. Only an all-matching block is skipped;
a mismatch or final fringe is scanned with the same scalar predicate. Comparing
the packed predicate mask with all ones is independent of lane bit order. No
stores or branches are hidden in the LLVM kernels.

This is a separate source-distributed dependency for byte fields. Existing
search and scalar observations are unchanged; no embedded dependency or shared
module inventory changed. Initial Split and both separate Split optimization
candidates remain unchanged and unselected.

## Functional evidence

The independent matrix has 1,607 cases: every byte value across sixteen
alignments and all relative positions of a 33-byte span; opposite-class and
non-ASCII mismatches at vector lanes and 15/16/17, 31/32/33, 63/64/65 fringes;
temporary owners, alias mutation, logical endpoint refusal and a boxed huge
offset. All cases pass under both compiler frontends at O0 and O2, using
32 MiB ordinary and 256 MiB poisoned heaps with an 8 MiB program stack.
The six bounded matrix receipts total 1,612 zero-leak memory processes.

The Rust compiler is frozen `855df836…` with runtime `c4b1097b…`. Its CLI
compile profile has `rc_debug=false`; dynamic poisoned execution is separately
recorded. Native compilation explicitly enables RC checks and uses only the
historical admitted `2f07707e…` compiler with V4 pair `cb76fd09…`, provider
`b02e2e0e…` and consumer runtime `c4b1097b…`. The compiler host uses
`80bd5b54…`; that distinct role is retained in the receipts. No newer family or
metadata substitution is claimed.

The actual public portable fixture also passes host O2 normal/poison execution.
Real wasm32 and AArch64 objects pass after IR reachability checks require
`Main.main → Main.endpoint → sealed adapter` to reach the SIMD classifier,
scalar classifier and byte load. The target triples and resulting object
headers are verified. These are object checks, not foreign runtime executions.
Compiler main-thread stacks are 64 MiB, distinct from the 8 MiB program stacks.

The missing test-only `all` helper diagnostic is retained separately and was
not counted as an intended refusal. Sources, failing diagnostics, all tuple
inputs and frozen artifacts stay plain. Completed functional ELF/IR files can
be recovered using the hash/size/mode and compressed/decompressed verification
records in `build/byte-ascii-functional-recovery/index.json`.

Exact source identities, artifact hashes, individual receipt paths and pending
checks are in `stdlib-byte-ascii-run-results.json` beside this note.

## Remaining gates

Parent integration must run `lib_byte_view_ascii_run`, including its checked-RC
Rust API builds and actual big-endian PPC full-adapter object. The frozen CLI's
supported target set is unchanged; PPC was not simulated by retargeting IR.

Changed-input public pipeline comparison and codegen review are still pending.
The prepared benchmark compares this exact source with a scalar source that
changes only the sealed adapter's worker call to its existing I64 scalar tail.
Bounds, owner hold, scalar classifier and result representation stay identical.
It includes complete run traversal, single-prefix queries, dense boundaries,
long runs, vector fringes, raw non-ASCII bytes and retained-input hashes, with an
independent scalar reference. No throughput claim or optimization selection is
made by this checkpoint. Fields runtime qualification remains separate.
