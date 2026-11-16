# Slash-path operations and checked patterns

`Text.Path` supplies lexical operations for portable slash-separated UTF-8
paths. It does not access the filesystem. Only `/` separates components;
backslashes, drive-letter text, embedded NUL and newlines remain ordinary
path bytes outside the pattern grammar. Existing `Data.Path` behavior is
unchanged.

| Operation | Behavior |
| --- | --- |
| `path-clean` | Collapse repeated separators and dot components; resolve parents lexically; preserve leading relative parents; return `.` for an empty result |
| `path-base` | Final nonempty component, `/` for slash-only input, `.` for empty input |
| `path-dir` | Clean the directory portion, defaulting to `.` |
| `path-ext` | Final dot and suffix in the last component; a leading dot counts; trailing `/` has no extension |
| `path-is-absolute` | Test the leading `/` without normalization |
| `path-split` | Split immediately after the final `/`, preserving original separators |
| `path-join` | Join nonempty elements, then clean; all-empty elements produce empty text; later rooted elements do not replace earlier elements |
| `path-match` | Return a checked whole-name shell match or `InvalidPathPattern` |
| `path-pattern`, `path-pattern-match` | Check once and reuse immutable compiled literal/range storage |

Patterns support `*`, `?`, backslash escapes and nonempty Unicode classes such
as `[a-z]`, `[^a]` and `[é-界]`. Descending ranges match no scalar. Literal
hyphens and closing brackets inside classes require escapes. A class can match
`/`; `?` and a star's consumed prefix cannot. Every pattern is fully checked,
including malformed suffixes after a term that already fails.

Star retries advance through byte positions. A question mark or class at a
skipped UTF-8 continuation byte sees one replacement scalar, while literals
continue comparing exact bytes. Thus `*[�]` matches `é`, but `*�` does not.
Matching uses the earliest successful chunk after each star. Literal and range
storage remains private across reusable-pattern calls.

## Independent validation

The pinned SDK generator records 364 lexical cases, 170 joins and 2347 shell
matches/errors. Inputs cover empty/rooted/relative paths, leading parents,
repeated separators, embedded controls, Unicode and exact 7/8/9, 15/16/17,
63/64/65 and longer word boundaries. Match fixtures cover Unicode ranges,
escaped metacharacters, slash classes, bytewise star retries, descending
ranges and complete malformed-pattern validation.

Frozen Rust O2 and self-built Stage3 CLI consumers pass all cases with zero
leaks, using a normal 32 MiB heap/8 MiB stack and a separate 256 MiB
poisoned-free run. Both frontends also pass 100 repeated temporary-owner
operations on 16385-byte names and a retained compiled pattern. A runtime-size
identity keeps this large fixture construction out of the compiler's constant
evaluator after fully constant setup overflowed its stack; that compiler issue
is not claimed fixed. Actual private kernels compile to WebAssembly and AArch64
objects. Foreign execution is not claimed. The supplied `lib_path` regression
target includes explicit Rust optimized-owner and big-endian object checks for
coordinated integration.

## Generated code and measurements

The initial word scans already used broad-word delimiter tests, but their
natural-number indices retained tag/overflow branches and owner operations
inside each iteration. The final source checks ranges and retains backing
owners outside private loops. Ordinary Cinch tail calls carry I64 counters and
borrowed addresses; straight-line inline LLVM handles exact unaligned loads,
eight-lane delimiter classification and packed word equality. No primitive
hides mutation or branches. Complete traversal workers contain no natural-
number or ownership runtime calls, and short tails read exactly one byte at a
time.

[Alternating measurements](path-counter-O2-2026-11-16.json) retain every sample
for eight public pipelines at four sizes. The CPU0 window was coordinated with
the parent agent, and both versions use the same frozen compiler, driver,
runtime and optimization settings. All 32 observable checksums match the
independent reference. At 64 KiB the machine-counter source measures about
2.26× for cleaning a component, 2.06× for many short components, 1.40× for
basename, 1.95× for extension scanning, 2.14× for reusable matching, 2.50× for
whole-pattern matching and 2.27× for joining. The unchanged absolute-path
control varies between 0.99× and 1.09× across sizes. These are local pipeline
measurements, not universal timing thresholds.

The reusable-pattern reference row supplies matching output checksums, but its
timing includes parsing because the reference has no public precompile API.
Other string/copy allocation semantics also differ. Public text operations
create private byte/text copies; the reference can reuse unchanged strings.
Reusable matching still builds temporary chunk-list structure. Allocation or
throughput parity is not claimed.

[Ordinary-space probes](path-space-2026-11-16.json) measure one complete
operation with equal-length executable paths, including startup and input/
pattern preparation. The 64 KiB component-cleaning pipeline allocates 162
blocks and peaks at 393742 bytes in both versions. The tagged-number removal
mostly changes traversal work, rather than allocation counts. All 32 probes
finish with zero live allocations. Poisoned-free checks are separate from
these ordinary-space measurements.

The [natural-counter patch](path-natural-counter-baseline.patch) reconstructs
the measured baseline from this checkpoint's `Text/Path.cinch`; the applied
patch's source hash is checked against the report. The complete baseline
corpus also passes the frozen Rust compiler. Reproduce from the repository:

```sh
source /workspace/.tools/cinch-env.sh
python3 tooling/validate_path.py --compiler /path/to/frozen/cinch \
  --artifacts build/path-validation --output build/path-validation.json
python3 tooling/bench_path.py prepare --compiler /path/to/frozen/cinch \
  --before-source /path/to/patched/Text.Path.cinch \
  --reference-go /path/to/pinned/sdk/bin/go --artifacts build/path-benchmark
python3 tooling/bench_path.py measure --artifacts build/path-benchmark \
  --cpu 0 --output build/path-benchmark.json
```

The [validation record](../../bootstrap/stdlib-path-validation-results.json)
contains compiler/source identities, exact fixture counts, normal/poisoned
results and raw-loop assembly inspection. The full standard-library goal
remains open.
