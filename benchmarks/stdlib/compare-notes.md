# Comparison capability checkpoint

The pinned SDK surface has four declaration records: Compare, Less, Or and
Ordered. `Data.Compare` maps them to `compare`, `less`, `first-non-default`, and
the `Compare` capability. The source spelling `Ordered` already denotes a Cinch
refinement predicate, so reusing that name for a class would change its meaning.
The result is `Data.Ordering.Ordering`, rather than a sign-bearing integer.

`Compare` instances cover all ten fixed/native machine integer types, `Int`,
`Nat`, `Char`, `String`, `F32` and `F64`. Float instances put all NaNs before
numbers, equate every NaN, and equate positive and negative zero. These are
explicit comparison policies; `Relation.Ord` and `Data.Machine` are unchanged,
and floats still have no global `Eq`/`Ord` instances.

`Default` explicitly supplies the distinguished value and its equality test.
Instances cover those same scalar types, plus `Bool` and `Unit`. The generic
list selection stops at the first non-default and preserves its representation.
An all-zero float list returns canonical positive zero. The unconstrained
`first-non-default-by fallback predicate` supports custom records, sums and
other caller-defined defaults without requiring a global equality instance.
The corpus includes a custom payload-bearing choice with both capabilities.

There are no built-in structural `Default (Option a)` or pair defaults. The
optional generic Option instance exposed a separate compiler constructor-shape
failure, reproduced even when its only method is `zero = None`; removing or
naming a `Some` pattern does not affect it. Both frozen Rust compilers refuse it
with `Data.Core.Option has payload words [0] per constructor; code generation
builds it as [0, 1]`. The complete minimal sources and six exact outcomes are
retained in `compare-option-shape-2026-11-16.json`. No shape check was weakened.
Callers can use the explicit fallback/predicate API for structural values.

## Functional evidence

`tooling/data/compare-reference.go` freezes independent SDK outputs for 19×19
binary32 pairs, 19×19 binary64 pairs, 300 integer pairs across twelve types and
100 UTF-8 string pairs. Float operands include both zero signs, normal and
subnormal boundaries, both infinities, both signs of signaling and quiet NaNs,
and distinct payloads. Strings include empty, prefix, embedded NUL, non-ASCII
and supplementary Unicode text. Each float row checks generic and explicit
float comparison/precedence operations.

`tooling/check_compare.py` adds 107 default/selection assertions, including exact
foreign-observed identity for every nonzero float fixture, positive-zero
canonicalization, explicit negative-zero fallback preservation, every machine
width, mathematical integers beyond 64 bits, custom capabilities and an empty
input. The native-width fixture records 64-bit SDK integers and is checked
against the actual emitted LLVM target or hashed Native provider target. The
Native compiler consumes an explicit source bundle and its exact provider.

The external C object in `tooling/data/compare-bits.c` constructs and observes
IEEE bits through the real FFI. Its declarations carry explicit Global effects
in consumers; public library comparisons remain pure. It is compiled and linked
with each consumer, rather than replacing a library comparison kernel.

- `compare-functional-epoch16-2026-11-16.json`: full frozen16 Rust and frozen
  Native Char-contract epochs; 32 consumers, 1,229 rows/policies per frontend,
  64 profile executions.
- `compare-functional-cad70-2026-11-16.json`: distinct repaired cad70 Rust epoch;
  16 consumers, 1,229 rows/policies, 32 profile executions.

Every executable ran once with a 32 MiB heap and 8 MiB runtime stack, and
separately with a 256 MiB heap, the same 8 MiB runtime stack, and poisoned freed
blocks. All 96 executions reported `live=0 immortal=0 leaked=0`. The compiler
host's main stack was separately raised to 64 MiB for the deep fixture source;
that setting does not replace the runtime's 8 MiB stack gate. Exact compiler,
runtime, provider, source, foreign-object, LLVM and executable identities are
retained in the reports. No Cargo build or full suite ran in this lane.

The frozen doc extractor finds canonical documentation on every declaration and
signature method, with zero diagnostics or unattached comments. Instance bodies
implement already documented methods and are documented by their instance.
`compiler/tests/lib_compare.rs` retains the documentation and fixture breadth
checks for integration. Root owns Cargo integration and shared doc generation.

## Code generation and performance

Optimized Clang O2 assembly was inspected from the actual emitted LLVM. Signed
I8 comparison lowers to `cmpb` and signed conditional moves; floating comparison
lowers to scalar `ucomiss`/`ucomisd`, parity branches for NaNs, numeric branches
and conditional selection of immortal `Ordering` constructors. Scalar input
pairs have no parallel byte/element lanes to exploit, so adding a SIMD or SWAR
kernel would add work. String comparison delegates the existing optimized
`string-compare`/byte comparison implementation; first-choice selection requires
ordered early exit. No speculative ISA dependency was added.

The one subsequent library edit removes an empty EOF line. The exact pre-edit
source is `compare-pre-normalization-source.json`; the one-line patch is
`compare-eof-normalization.patch`; both hashes are retained in
`compare-normalization-2026-11-16.json`. Final-source representative I8, complete
F32 NaN/payload matrix and all 107 default policies passed on cad70 and Native,
with 12 further zero-leak ordinary/poison executions, recorded in
`compare-final-source-smoke-2026-11-16.json`. Thus the retained functional
reports contain 108 separate profile executions. The final source hash is
`2c8339cfe0420a3ad27bed174308286e4c7e377318adf18dd8dcd5ce172686cb`.

Both benchmark reports use the distinct repaired cad70 compiler, the unchanged
c4 runtime archive, Clang O2 for the actual executable and retained assembly,
CPU0 affinity, 32 MiB heap and 8 MiB runtime stack. Every Cinch warmup/sample
also checked zero leaked blocks. They alternate Cinch/reference order, retain
all seven raw samples, and check the identical output checksum for each case.
Startup, parsing, clock-result formatting, exit leak checking and printing are
outside the timer. Scalar loop/dispatch, input phase updates, public calls and
selection list construction are inside it. Inputs are runtime command arguments.

The first report, `compare-benchmark-checked-remainder-cad70-2026-11-16.json`,
retains an allocation-heavy driver. Its arithmetic uses the public checked
machine remainder API; optimized assembly still allocates and destroys two
Option results per iteration. At 100,000 iterations its integer, float, choice
and string medians were 37.03, 55.38, 130.31 and 94.80 ns respectively, versus
3.75, 4.62, 4.01 and 12.56 ns for the independent reference. These figures
include those unrelated arithmetic allocations and do not isolate comparison.
The report is retained instead of discarding the slower measurements.

The second report, `compare-benchmark-progression-cad70-2026-11-16.json`, uses
bounded scalar operand phases in both independent drivers, producing the exact
same input sequence and checksums while avoiding remainder Options. Actual
optimized assembly was inspected again: the integer branch uses scalar compare
and conditional moves with no allocation, the float branch uses scalar
`ucomisd`, and the phase updates use conditional moves. Selection still shows
its representation and reclamation costs; these are complete public pipelines,
not isolated instruction throughput.

| Pipeline, 100,000 iterations | Cinch ns/iteration | Reference ns/iteration | Cinch/reference |
| --- | ---: | ---: | ---: |
| Generic integer compare + less | 5.06 | 3.37 | 1.50× |
| Generic NaN-first float compare + less | 6.34 | 5.25 | 1.21× |
| First non-default list selection | 35.46 | 5.75 | 6.16× |
| Generic UTF-8 string compare + less | 29.97 | 13.26 | 2.26× |

All slower results remain visible. The float 10,000-iteration sample medians
were 5.19 ns Cinch versus 6.45 ns reference, while the 100,000-iteration result
reversed that ordering; this small-sample noise is retained rather than used to
claim a speedup. String and selection results include different representations
and allocation/reclamation policies. No CI performance threshold or broad
throughput parity claim follows from these local measurements.

Reproduce functional validation with explicitly selected compatible artifacts:

```sh
python3 tooling/check_compare.py --work build/compare/check \
  --rust /absolute/frozen/cinch --rust-runtime /absolute/libcinchruntime.a \
  --native /absolute/frozen-native-directory --cpu 0
python3 tooling/bench_compare.py --work build/compare/bench \
  --compiler /absolute/frozen/cinch --runtime /absolute/libcinchruntime.a \
  --go /absolute/pinned/go --cpu 0 --progression
```

## Runtime CI regression follow-up

`lib_compare` now includes an explicitly Rust-only runtime test using the current
`CARGO_BIN_EXE_cinch`, the committed JSON fixture corpus, and the actual C
bit-pattern object. It refuses external frontend selector variables rather
than silently substituting Rust for selected Native coverage. No SDK executable
is needed. Before fixture emission, the runner requests an ordinary tiny
compiler `build`, which initializes or refreshes the compiler's exact
`runtime/target/release/libcinchruntime.a`. It captures that archive while
holding the runtime's shared lock, then records its identity and uses the
captured bytes for every fixture link. The initializer is also executed with
zero-leak checking. Parent Cargo integration executes this initialization path;
no Cargo invocation ran in the comparison lane.

The runtime test runs I8, the complete F32 NaN/zero/payload group, all default
policies and UTF-8 strings, with separate 32 MiB normal and 256 MiB poisoned heap
profiles, both using 8 MiB consumer stacks. Compiler subprocesses have a separate
64 MiB host stack limit. The test checks the exact captured archive identity,
the four executed groups, both profiles and zero-leak diagnostics.

The new optional `--native-rc-debug` flag emits Native fixture count operations
through checked runtime calls. Rust's current CLI exposes no equivalent
`--rc-debug` option, so reports explicitly record Rust compilation with
`compile_rc_debug=false`; runtime poisoning is still exercised separately.
`compare-runtime-regression-2026-11-16.json` retains focused new cad70 Rust and
Native instrumented I8/F32/default/string results: eight consumers, sixteen
ordinary/poison executions, all zero-leak. Inspection of the new Native LLVM
confirms its checked count wrappers call `cinch_dup` and `cinch_drop`. Earlier
frozen reports were left unchanged, and no library declaration changed.
