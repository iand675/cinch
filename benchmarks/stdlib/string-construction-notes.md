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
All 297 repeat and 101 concat observations from the independent Python oracle
now pass at O0 and O2, alongside regression and ownership cases in both heap
modes. The pinned SDK Go1.27.4 generator agrees on every decoded observation;
its JSON formatting differs and both file hashes are retained. Paired O2
baseline/new binaries also pass independent digest/checksum functional smokes
for ASCII and Unicode repeat/concat at 65536 and 65537 units. Initial paired samples are recorded; a corrected driver will separate final
input-owner teardown from the timed interval. The supplied four-test Rust
target is pending coordinated Cargo integration. The receipt is
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

## Paired performance and remaining costs

The [initial paired O2 results](string-construction-initial-O2-2026-11-16.json) retain five
alternating samples for each of twelve conditions, with independent exact wire
SHA-256 and changed-output checksums. CPU0 was exclusively granted; other
assigned CPUs continued compilation, so this is not a whole-machine isolation
claim. Input setup and the initial reference digest were outside the timer, but IR
inspection showed final input-owner teardown occurred inside the measured loop.
The initial driver also built an unused deep piece list for repeat. These raw
samples and the exact initial driver are retained as cleanup-inclusive pipeline
evidence. A corrected driver avoids that unused list and observes retained
inputs after the end clock; corrected measurements remain pending.

| Pipeline and input unit | 17 units | 4097 units | 65536 units |
| --- | ---: | ---: | ---: |
| Repeat, ASCII | 0.910× | 36.521× | 5.284× |
| Repeat, Unicode | 1.053× | 21.698× | 5.294× |
| Concat, ASCII | 0.875× | 0.910× | 0.953× |
| Concat, Unicode | 0.950× | 1.178× | 0.850× |

These provisional ratios are old/new median time including input teardown. Small repeat and several concat conditions
regress at O2. Concatenation now revisits list/string metadata in a size pass and
a copy pass and retains tagged natural counters, explicit checked-copy capacity
branches and ownership machinery. The result fixes unbounded native traversal
stack; it does not claim every O2 condition became faster. A separate measured
machine-counter trial can investigate these costs without silently changing the
already frozen source epoch used for fresh-base validation.

[Disassembly evidence](../../bootstrap/stdlib-string-construction-stack-analysis.json)
explains why the old O2 program passed while O0 failed. The measured old CSV
probe's O0 `repeat-from` reserves 528 local stack bytes per recursive call, about
544 bytes including saved base/return. At 65536 levels this is about 34 MiB,
beyond the ordinary 8 MiB limit. The paired O2 baseline still calls itself and
copies afterward, but its approximate per-level frame is 80 bytes (about 5 MiB).
Old O2 concat similarly keeps roughly 112 bytes per nonempty piece (about 7 MiB
at 65536 pieces). The new workers use loop backedges with no recursive calls.
These estimates describe worker frames, not exact whole-program peak stack.

## Focused reproduction

Use an exact phase-1 source checkout (`32d16d44`) or pass its extracted
`Data.String.cinch` with `--module-source`. The source SHA-256 is
`54ae8a339b996756323d8f5598287ca3abdfe8ce5e291cd62e71fdf975bd3b0e`;
its unmarked interface source hash is `9736755187434316968`.

```sh
source /workspace/.tools/cinch-env.sh
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RUST_MIN_STACK=67108864
python3 tooling/validate_string_construction.py \
  --compiler /workspace/dasl-stdlib-io/build/prim-wide-frozen/cinch \
  --module-source compiler/prelude/Data/String.cinch \
  --artifacts build/string-construction-o0 --opt-level 0 \
  --output build/string-construction-o0/full.json
# Repeat with --opt-level 2 and a separate artifact directory.
GOCACHE=/tmp/cinch-go-surface-cache GOTELEMETRY=off \
  /tmp/cinch-go-current/go/bin/go run tooling/go_string_construction_vectors.go
```

Run compiler work only on a granted functional CPU. The SDK output agrees with
all committed decoded fixture observations; its canonical JSON key order differs
from the independent Python fixture's order. Fresh-base validation must match
the unmarked source hash; the source-override marker deliberately has a different
hash and must not be substituted for the fresh embedded/provider receipt.
