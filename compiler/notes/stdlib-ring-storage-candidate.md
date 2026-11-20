# Unselected storage-pinning experiment

The selected Ring module remains source `9c286064`. The separate candidate
`0dd49d9c` is reproducible with
`tooling/ring-storage-pinning-candidate.patch`; it has not been adopted.

The private relocation traversal takes the already allocated dense node array
instead of repeatedly resolving the smaller arena's storage. It pattern
matches each node once and uses its link array for both reads and writes.
The transfer caller holds storage stable until this loop finishes, then moves
the owners and publishes the forwarding edge in the original order. No
callback occurs in the loop. Public callback traversal is unchanged and still
resolves cursors again after every user action. Public API, representation,
capacity checks, paired SIMD arithmetic and runtime ABI remain unchanged.

Frozen Rust `b004049b` with explicitly selected runtime `c4b1097b` passed the
complete 768-action corpus, 24 bounded large-trace poison observation windows,
owned-value/callback controls and 49 numeric lane triples. Every process used
normal 32 MiB heap / 8 MiB stack or separate poisoned-free 256 MiB heap /
8 MiB stack, with zero leaks.

Test-only source snapshots also append private zero/singleton/two-node prefix
canaries. Extra link slots, independent value slots and initialized suffix
nodes must remain unchanged. Both selected baseline and candidate pass both
memory profiles. These private hooks are never exported by the production
module. `tooling/generate_ring_storage_canary.py` records both temporary-module
hashes and the exact underlying source; the library file remains untouched.

The frozen candidate benchmark matches the independent reference for all 30
workload/size conditions and 20 small poisoned controls. A separate profile
discards its internal clock outputs and records whole-process allocation
counts, including startup, preparation and teardown. At 16,384 elements,
dense construction, growing joins, balanced joins and detached copying have
the same allocation totals as the selected Rust baseline. Balanced joins use
540,784 header blocks / 16.5 MB cumulative allocation / 8.3 MB peak in both.
Short split/rejoin differs by two small header blocks. This is not an
operation-only allocation count or a performance improvement claim.

Rust assembly has different loop expansion after the change: static call and
vector-instruction counts therefore cannot establish dynamic retain/release
costs. The exact artifact/provenance and per-function summaries are retained
by `tooling/profile_ring_artifacts.py` and
`../evidence/stdlib/stdlib-ring-storage-candidate-results.json`.

Provisional merged Native `7a3d` qualification stopped before emission at the
genuine refined `Data.Machine.mask-i64` imported-contract boundary. Its
foreign and resolved type texts use different binder names and normalization.
Codegen owns that repair. Neither candidate Native correctness nor a Native
gain is claimed. Selection requires repaired-provider full Ring qualification
and matched complete-pipeline measurements; no frozen artifact is mutated.
