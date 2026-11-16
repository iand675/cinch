# Sealed String scalar observations and counts

`string-utf8-first` and `string-utf8-last` return `Option (Char,Nat)`, with the
encoded byte width as the second value. Empty text returns `None`. The first
observation decodes at the start of private backing. The last locates a leading
byte in at most four reverse checks before decoding that scalar. Neither copies
input or exposes mutable backing.

`string-length` now counts non-continuation bytes of valid immutable UTF-8.
Complete sixteen-byte blocks use an LLVM vector comparison and exact byte-flag
reduction. Eight-byte blocks use SWAR, then exact scalar fringes read only their
available bytes. The packed arithmetic does not depend on host byte order.
Local machine counters avoid imported calls in hot loops; one outer owner hold
covers each traversal. Branches remain ordinary source tail loops and primitive
blocks contain no writes. The raw `string-length-from` body and stop-at-error
semantics remain unchanged. No imports or base dependency registrations change.

Independent Python and pinned SDK observations agree on 637 repeated texts,
2,048 endpoint/alignment cases, seven complete scalar ranges and 154 raw-prefix
controls. The complete range includes all 1,112,064 valid scalar values in
4,382,592 bytes, with exact independently checked wire digest. Its runtime input
builder walks descending code points in 64-character chunks, preserving ascending
output without a deep list or reversal. Full cad70 O0/O2 source-override gates,
including 100 temporary-owner pipelines per profile, pass all 5,740 normal and
poison processes with zero leaks. Ordinary runs check whole groups under
32 MiB/8 MiB; poisoned runs select one assertion under 256 MiB/8 MiB. Compiler
shell/Rust stacks independently use 64 MiB.

Actual extracted production kernels also pass the valid-text/alignment rows,
all 65,536 sixteen-lane masks, and every byte value at widths zero through 33
under cad70 Rust and frozen Native, both memory modes. Raw mask models check the
private classifier mathematics; they do not change the public raw-prefix policy.
Actual wasm32 and AArch64 objects compile. Five supplied Rust tests include a
fresh embedded source gate and actual big-endian object profile; Cargo has not
run in this lane. Twelve added/touched declarations have canonical docs.

Exact source/compiler/interface identities and complete functional evidence are
in `bootstrap/stdlib-string-scalar-source-override-results.json`. The paired
public count driver is built and passes 60 normal/poison smokes with independent
scalar sums and complete before/postclock wire digests. IR verifies clock order
and postclock backing reads; optimized assembly still calls public String count
inside measured work. Timing remains pending.

This evidence is an explicit final-cad70 source override. It does not claim a
fresh embedded module or fresh Native String provider. Those combined
phase2/3/4 epochs and the full Native public API corpus remain parent-owned.
The import-free observations let `Text.Utf8` implement valid-String adapters
without adding a String-to-byte-UTF8 dependency or exposing private backing.
