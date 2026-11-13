# cx-sweep: the compiler's scaling exponents

One command measures how each axis of program size affects the self-hosted
compiler, and fits an exponent to it.

```
python3 experiments/cx-sweep/sweep.py --stage 3 --out /tmp/cx-sweep
```

That builds nothing. It needs a stage compiler at
`build/bootstrap/stage3/cinch` (or `--compiler PATH`), generates one program
per point, runs `--emit-ir` over it with `CINCH_ALLOC_STATS=1`, and prints a
table of exponents. `--out` is where the generated sources, per-point logs and
`cx-sweep.json` go.

To re-analyse a finished run without recompiling anything (for example after
changing the fitting method):

```
python3 experiments/cx-sweep/sweep.py --refit /tmp/cx-sweep/cx-sweep.json
```

## Why blocks and not seconds

The primary number is the runtime's own allocation count, because it does not
depend on what else the machine is doing. This machine runs several lanes at
once, so a wall-clock slope taken across a sweep is a slope through somebody
else's build. Wall clock is recorded and printed as a secondary column for the
same reason it is not trusted: it is real, but it is not reproducible in
isolation.

`bytes` is total allocated bytes (not peak), which is the honest measure of a
compiler whose footprint is its total allocation.

## The three exponents

Each axis reports three fits, because they answer different questions and
disagreeing is itself diagnostic.

| column | model | what it is |
|---|---|---|
| `k` | `floor + a*n^k`, floor free | the exponent to read. The compile has a large fixed cost (about 66M blocks: the base interface set, the manifest, the runtime ABI), so this is the only fit that separates that floor from the growth. |
| `delta` | `a*n^k` over `blocks(n) - blocks(n_min)` | the naive fit. It reads high whenever the smallest point already contains some growth, which it always does. |
| `bytes` | log-log of allocated bytes | whether the growth is allocation-shaped. |
| `ir` | log-log of emitted IR lines | whether the growth is output-shaped. If `ir` is near 1 and `k` is 2, the compiler is spending more per output line, not producing more. |

`local` is the exponent between each adjacent pair of points, so a curve that
bends (linear then quadratic) is visible without trusting any single fit.

`k` is fitted by gridding over the exponent alone: for a fixed exponent the
model is linear in `(floor, a)` and those come from the normal equations
exactly. The residual is measured in **linear** units. A log-space objective
cannot see a term worth 2% of the total, and reports an exponent of 0 for
exactly the axes whose growth is small but real; `string` and `identifier` are
the two axes where that difference decided the verdict.

## The axes

| axis | generator | sizes | what varies |
|---|---|---|---|
| `definitions` | `generators.definitions` | 25, 50, 100, 200 | independent `Int -> Int` definitions in one module, all called |
| `modules` | `generators.modules` | 8, 16, 32, 64 | modules in a chain, each importing the previous one |
| `constructors` | `generators.constructors` | 8, 16, 32, 64 | constructors in one data type, with a case over all of them |
| `typevars` | `generators.type_variables` | 8, 16, 32, 64 | type variables quantified in one signature, all applied |
| `depth` | `generators.expression_depth` | 32, 64, 128, 256 | left-nested operators in one term |
| `patterns` | `generators.case_patterns` | 16, 32, 64, 128 | literal alternatives in one `case` |
| `entries` | `generators.interface_entries` | 8, 16, 32, 64 | four exports per module across many modules |
| `imports` | `generators.imports` | 8, 16, 32, 64 | one importing module's fan-in, one export each |
| `string` | `generators.string_literal` | 256, 512, 1024, 2048 | bytes in one string literal |
| `identifier` | `generators.identifier` | 64, 128, 256, 512 | characters in one identifier |

Every generator is a pure function of its size, so a point is reproducible from
its number and there is no fixture corpus to rot. Sums are balanced rather than
left-nested so that the `definitions`, `entries` and `imports` axes do not
accidentally measure the `depth` axis. Every generated name is reachable from
`main`, because the emitter prunes unreachable units and a pruned workload
measures nothing.

## The control

`interface-bytes` pads one symbol in the provider's interface set by 1024 to
8192 bytes, re-settles every dependency identity, and runs `--check` on a
two-line program. The published law for this is a fixed number of blocks per
added interface byte, and the harness must reproduce it:

```
control (interface-bytes): floor=30467266 blocks, median 6.01 blocks/byte
(expected 6.0), exponent 0.998, reproduced=True
```

6.0 is the law after `Text.Json`'s string fast path
(`design/compilation-speed.md` 2.18); 10.0 is the law before it, and a tree
that predates the fix measures that instead. Both are accepted, and which one
was reproduced is reported. A harness that reproduced neither would be
measuring something other than interface bytes, and every axis above would be
suspect.

The control construction is `experiments/iface-scan/measure.py`'s, reused
rather than reinvented.

## Naming a site

An exponent says an axis is superlinear; it does not say where. For that:

```
python3 experiments/cx-sweep/profile.py --axis modules --size 64 --seconds 6
python3 experiments/cx-sweep/profile.py --axis modules --size 64 --root check-global-core
```

It runs one point, samples the running compiler with `sample`, and ranks frames
by their inclusive sample count under `--root` (default `elaborate-module`).
`--delay` controls when sampling starts: too early samples the interface load,
too late can miss the run.

## Options

```
--axes definitions,modules    a subset (default: all)
--sizes 8 16 32 64            override every axis's sizes
--repeats 3                   interleaved repeats per point; the median is kept
--timeout 900                 per-invocation seconds
--mode check                  check instead of emit-ir
--no-control                  skip the interface-byte control
--label TEXT                  recorded in cx-sweep.json
--refit PATH                  re-fit a finished cx-sweep.json, no compiling
```
