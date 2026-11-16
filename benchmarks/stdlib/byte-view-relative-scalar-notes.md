# Checked relative scalar observation

`byte-view-utf8-at :: ByteView -> Nat -> Utf8Step` is a small dependency for
byte-oriented split and line consumers. It delegates to the existing bounded
UTF8 decoder with an absolute position only after checking the relative view
coordinate. Logical end returns `Utf8End`; past-end positions return
`Utf8OutOfBounds`. Malformed and view-truncated bytes advance one byte with
U+FFFD/`Utf8Invalid`. Complete valid scalars retain their exact encoded width.

The method exports no backing owner or address and creates no byte-buffer
snapshot. Returned opaque steps contain scalar/width/status observations only.
Current mutations of the owner or overlapping views are observed on each call.
The fixed view endpoints continue to bound all decoder reads.

The independent pinned SDK generator and a separate Python decoder agree on
1,505 arbitrary owners, all 147,105 legal subspans and 882,630 relative-position
observations. The retained fixture is self-contained: regeneration reads its
input hex fields and ignores previous expected hashes. Ordinary and poisoned
owner controls additionally cover temporary views, scalar truncation, end and
huge coordinates, and mutation between observations.

All complete fingerprints and controls pass Rust O0/O2 with frozen compiler
`b004049b…` and matched runtime `c4b1097b…`, and raw Native O0/O2 with compiler
`2f07707e…`, explicit matching manifests and runtime `80bd5b54…`. These new
epochs are separate from the historical cad70/586 View search receipts. Each
program retains the normal 32 MiB or poison 256 MiB heap cap, 8 MiB stack and
zero-leak check. The compiler main thread has a separate 64 MiB allowance.

No new unsafe operation, machine kernel or base-module dependency is introduced;
the existing UTF8 decoder owns its accelerated scanning and exact scalar fringe
loads. The change adds one canonical public declaration. Exact sources,
frontends, runtimes, fixture and process identities are retained in
`bootstrap/stdlib-byte-view-relative-scalar-results.json`. The supplied
`lib_byte_view_scalars` target has four focused tests; parent integration owns
Cargo and the updated documentation inventory.
