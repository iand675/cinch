# Immutable string construction: first bounded checkpoint

`Data.String.string-repeat` previously allocated its result at the recursive
base case and copied one unit on each return. `string-concat-all` used the same
shape over its piece list. Neither source traversal was tail-recursive. The
existing CSV diagnosis records an O0 failure while constructing
`string-repeat 65536 "x"` under an 8 MiB stack.

The replacement preserves both public signatures and the immutable backing
contract. Concatenation counts exact byte lengths with a tail pass, allocates
once, then copies each piece once through a checked tail pass. Repetition
allocates exact total bytes, seeds one complete unit, and doubles the initialized
prefix until the final remaining fringe. It performs logarithmically many bulk
copies and copies linear total bytes. Empty strings and zero repetitions require
no filling loop. All mutation remains in ordinary checked bulk-copy operations;
the runtime supplies their accelerated memory movement.

The result wraps only a fresh private buffer. No mutable alias of an input or
output conversion becomes a string backing. Concatenation and repetition of
valid UTF-8 preserve exact bytes, including embedded NUL and combining scalars.
Existing invalid-byte conversion policy is unchanged.

## Executed source-override epoch

The frozen Rust CLI contains an older embedded `Data.String`. Every preliminary
consumer explicitly imports a `Data.String.cinch` beside `Main`; its comment
receipt discriminator makes this source identity distinct even from a compiler
freshly rebuilt with the same implementation. The validator requires the emitted
`Data.String.cinchif.source_hash` to equal the selected override's independently
computed source identity. This prevents an old embedded implementation from
silently satisfying a source-override test.

At O0 and O2, the exact-source consumers pass the 64 KiB repeat and 64K-piece
concat/mconcat regression, conversion alias isolation, Unicode owner checks,
and 100 repeated temporary-owner construction pipelines over 16385 copies.
Each passes normal 32 MiB heap/8 MiB stack and separate poisoned-free 256 MiB heap
runs with `leaked=0`. O0 IR emits `concat-size`, `concat-copy` and `repeat-double`
workers with backward branches and no recursive call sites.

This evidence does **not** claim a fresh embedded compiler, a refreshed Native
base, or a full bootstrap gate. Those epochs remain explicit parent-owned gates.
The complete fixture corpus has 297 repeat and 101 concat observations from an
independent Python oracle; a pinned SDK generator is prepared for cross-checking.
Execution of that full corpus and SDK regeneration remain pending at this first
checkpoint. The supplied four-test Rust target is also pending coordinated Cargo
integration. The receipt is
`bootstrap/stdlib-string-construction-source-override-results.json`.

## Next phases

The actual pinned inventory contains 84 `strings` and 102 `bytes` declaration
records. This constructor repair is the first bounded increment, not package
completion. Follow-up work includes the analogous source stack shapes in
`Data.String.pack-from` and the two `Data.Bytes` list constructors; concrete
search/count/cut/split/join/replace and iterator surfaces; explicit reader,
mutable buffer/builder and replacer capabilities; and Unicode case, folding,
whitespace, map and trimming behavior.

`Data.String` accepts valid immutable UTF-8. Raw invalid-input rune and repair
policies belong to the byte surface, with an explicit repair conversion into
String. `Data.Char` retains its documented ASCII-only case/classification APIs.
A separate `Data.Unicode` dependency can provide pinned Unicode 17.0.0 simple
case/folding and whitespace data for the broader capabilities without changing
that existing contract. All remaining capability mappings stay open until their
concrete implementations and independent fixtures pass their gates.
