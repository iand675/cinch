# Retained byte splitting

`Data.Bytes.Split` is a source-distributed layer over checked `ByteView` ranges.
Every output aliases the input and retains its complete fixed allocation. Use
`byte-view-snapshot` to release that retention and obtain independent mutable
storage. Output views cannot be resized into adjacent fragments.

`None` is an unlimited split count. `Some 0` emits no fragments, and a positive
maximum includes the unsplit remainder as its last fragment. Empty delimiters
advance by bounded UTF-8 decoding: a valid scalar occupies one fragment and a
malformed byte occupies one fragment. An empty input with an empty delimiter
emits no fragments. Nonempty delimiters preserve leading and trailing empty
fragments. Lines include LF terminators, preserve CR and omit a trailing empty
fragment after a final LF.

Factories retain fixed endpoints without reading current byte contents. A raw
separator is observed at every advance, including changes before the first
advance and changes caused by an alias into the input. Explicit compiled-pattern
factories retain a stable independent separator snapshot. Input content remains
live in both policies.

The existing `Producer` abstraction remains persistent and carries explicit
immutable progress. Repeating a fold from the same initial Producer starts a
fresh traversal of the current mutable bytes. `ByteSplitCursor` is a separately
named sequential pull adapter: each factory creates independent progress, copied
cursor handles share progress, and close/exhaustion causes every later pull to
return `None`. It retains input and delimiter owners until its last handle is
released. Concurrent pulls are outside this sequential cursor contract.

The pinned reference audit distinguishes the documented single-use sequence
contract from unsupported repeated invocation of that same sequence. Public
cursor controls cover independent fresh traversals, copied callbacks, early
stop, repeated exhausted pulls, mutable raw separators, aliased separators,
malformed scalar boundaries and line-boundary changes. Repeated invocation of
the same reference sequence is retained only as an observation, not promoted
to a supported requirement. The concrete Cinch factory/Producer/cursor mapping
preserves the existing Producer contract while expressing single-use pull
behavior explicitly.

All substring scans use the selected linear search implementation, including
its checked SWAR block filters and scalar fringes. Stable patterns reuse their
failure table. Raw separators compile current contents at each step. Across
nonoverlapping matches the total delimiter length processed is bounded by the
input length plus one delimiter, so this remains linear. The initial raw
implementation takes two independent delimiter copies per compiled step;
the retained complete-pipeline measurements expose that allocation overhead.
The separately tested exact-byte query allows a future single-byte optimization
without changing this initial implementation or its stable-pattern policy.

The independent fixture contains 8,295 complete output cases covering all byte
values, bounded valid and malformed UTF-8, empty/overlapping separators, limits,
line endings, and packed-word fringes. Initial source, nine owner/lifecycle
controls, and 16,384-fragment construction pass Rust and strictly paired Native
O0/O2 under the unchanged 32 MiB normal / 256 MiB poisoned heaps and 8 MiB
program stack. Every process reports zero leaks. A separate documented test-only
export checks both private counter fields, 255/65535 transitions, the largest
offset, the length-plus-one sentinel, untouched canaries and atomic refusals.
All 46 production declarations have canonical owned documentation.

Rust uses the frozen `b004049b` compiler and `c4b1097b` runtime. Native uses the
legacy `2f07707e` compiler with the strictly admitted V4 provider (`b02e2e0e`
object, `cb76fd09` pair), linked explicitly to the V4 `c4b1097b` consumer
runtime. Its compiler-host runtime remains `80bd5b54`; this is a qualified
compiler/provider pairing, not a newly self-built distribution. The original
provider refusals and the separate benchmark-fixture checker refusal remain
recorded. No base-module source override is used.

The original benchmark driver and its Native-compatible spelling have separate
source identities. Both retain independent complete output/order, original
input, mutated input and separator checks. Native passes all 34 public pipeline
groups in normal and poisoned profiles; those runs are functional preflights,
not timing claims. The initial Rust baseline preserves every raw warmup and
sample, including unrelated control variation and regressions. Counter and
single-byte-delimiter candidates remain separate and unselected in this
checkpoint.

The complete public Split fixture also produces actual wasm and ARM objects.
Its emitted entrypoint reaches split state transitions, complete packed blocks,
scalar tails and the exact word/byte/mask workers; four host O0/O2 normal/poison
profiles return the expected boundaries and zero leaks. This maintained fixture
uses the current source View dependency (`966efd5c`), recorded separately from
the frozen initial View (`2b7bfa41`) in the full Native corpus. The parent API
target adds actual PPC object compilation using its newer target-convention fix;
that parent gate has passed with the compiler correction recorded below.
This is a source-distributed increment;
no embedded-base or complete strings/bytes coverage is claimed.

The Rust CLI's default compiled profile uses inline RC helpers. The earlier
`--rc-debug` token after its entry filename was a program argument, not a
compiler option; the tooling now omits it and records that build distinction.
Dynamic normal/poison heaps and leak checks remain actual executed profiles.
The maintained Rust API tests explicitly set `rc_debug=true` and check every
count operation out of line. That stronger gate exposed a compiler declaration
bug: `cinch_nat_cmp` was falsely marked `readnone`, so clang O2 could release a
boxed comparison operand before reading it. The original IR, reordered assembly,
trace and single-declaration diagnostic treatment are retained in the compact
`bootstrap/stdlib-byte-split-nat-cmp-*` evidence. Parent fix `2ab93c5b` removes the
false attribute; the public boxed-Nat regression and all nine Split tests,
including checked-RC O0/O2 and actual PPC objects, pass. Split source is unchanged.
