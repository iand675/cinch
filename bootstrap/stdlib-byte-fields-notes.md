# Retained byte fields: functional and portable checkpoint

`Data.Bytes.Fields` implements the four audited field operations with concrete
state and callback traversal. This checkpoint claims source/type validation and
the complete Rust and historical Native O0/O2 functional scope below, plus real
host/wasm/ARM portable consumers. Parent checked-RC/PPC API tests and performance
review remain pending.

| Audited declaration | Primary whole-buffer operation | Bounded-view counterpart |
| --- | --- | --- |
| Fields | `bytes-fields` | `byte-view-fields` |
| FieldsFunc | effect-polymorphic `bytes-fields-func` | `byte-view-fields-func` |
| FieldsSeq | reusable `bytes-fields-sequence` / `bytes-fields-each` | `byte-view-fields-sequence` / `byte-view-fields-each` |
| FieldsFuncSeq | reusable effect-polymorphic `bytes-fields-func-sequence` / `bytes-fields-func-each` | `byte-view-fields-func-sequence` / `byte-view-fields-func-each` |

The opaque `ByteFieldsState` supplies immutable explicit start/next progress;
there is no hidden mutable cursor. Reusing a state replays its scan against
current aliases. Pure persistent Producer adapters are additional conveniences.
The general predicate and yield operations preserve arbitrary caller effects
through a shared row; they are not replaced by those pure adapters. Explicit
weakening wrappers combine distinct IO and mutation capabilities when needed.

## Contents, callbacks and lifetime

Fields are nonempty fixed logical views into the complete retained owner.
They expose subsequent alias writes, cannot extend into neighboring bytes and
perform no repair or transcoding. Malformed UTF8 advances one byte and presents
U+FFFD to a predicate; well-formed U+FFFD advances three bytes. Logical view
boundaries are independent inputs, including truncated or continuation starts.

Default classification uses the existing Unicode whitespace kernel. Complete
ASCII runs use the separately qualified sealed SIMD observation from
`ed70e1bc`; generic predicates remain scalar and just-in-time because even a
nominally pure callback can mutate captured byte buffers. No future decoded
bytes or block classifications survive a callback.

Predicates run once per decoded scalar, from left to right. The scalar and
encoded width are captured before invoking the predicate. A yielded field's
terminator is already consumed with that captured width before the yield
callback can mutate it. Returning False stops immediately. A reusable callback
factory does no scanning at construction and starts from byte zero on every
invocation; captured predicate state and aliases remain shared. Stateful
decisions have an explicit deterministic Cinch meaning, beyond the stable
predicate subset used by the reference SDK contract.

Collection uses an accumulator/reverse and tail traversal. It does not allocate
on recursive unwinding. No backing pointer, mutable String storage or embedded
base dependency is exposed or added.

## Independent evidence

The pinned SDK fixture and independent Python model agree on all 1,266 complete
output/trace rows and eight mutable callback scripts. The independent decoder
uses strict scalar validation and one-byte malformed progress; the whitespace
oracle explicitly lists the 25 required code points rather than using Python's
broader `isspace` policy. It covers all raw bytes, Unicode exclusions, valid
versus malformed replacement scalars, bounded UTF8 edges, SIMD fringes,
construction silence, early stop and repeated factory invocation.

Rust `855df836…` with explicit runtime `c4b1097b…` passes the following scope at both O0 and O2:

- All 1,266 output/trace rows across eager lists, repeated pure Producers and
  reusable effectful callback factories: 318 memory processes.
- Eight independently generated mutation scripts, including current/future/past
  writes, saved multibyte terminator width, retained aliases and early stop:
  16 memory processes.
- Temporary and empty owners, explicit state, factory replay and a genuinely
  state-dependent mutation-effect predicate: four memory processes.
- A runtime-generated 65,536-byte dense input, with all 32,768 fields checked in
  the eager list and an effectful visitor: two memory processes.

Each optimization level has 340 processes, for 680 total. They use ordinary 32 MiB or poisoned 256 MiB heaps, an 8 MiB program
stack and leak checking; every process reports `leaked=0`. Compiler main-thread
stacks are separately 64 MiB. Rust CLI compilation has `rc_debug=false`; actual
dynamic poisoning is recorded separately. No compiled checked-RC API or Native
result is claimed by these runs.

The historical Native `2f077`/admitted-V4 pair with consumer runtime `c4b1097b`
passes the same complete original scope at both O0 and O2: 340 processes per
optimization level. Its compiler-host runtime remains separately `80bd5b54`. O2
relinks the exact verified accepted O0 LLVM with matching source, module,
interface, object and runtime hashes. It does not rerun or relabel the compiler.
All Native runs have checked-RC compilation; none claims a newer compiler family
or self-built V4 distribution. The original four frontend/optimization gates
total 1,360 zero-leak processes.

An additional actual IO-predicate/mutation-yield fixture passes all four gates,
eight further normal/poison processes. One retained factory runs twice, prints
exactly `a b a b `, and stops before either `c` predicate observation. The IO
capability is explicitly captured rather than inferred from an effect annotation.
The first fixture-only missing-capability refusal remains plain.

The complete portable consumer reaches eager collection, predicate collection,
effectful traversal, bounded UTF8 decoding, Unicode classification and the
actual packed/scalar loads. Host O2 runs normally and poisoned with zero leaks.
Real wasm32 and AArch64 O2 objects pass header checks. The fixed pure predicate
causes `collect-with.sp0`; only a numeric specialization of that exact collector
may satisfy its graph assertion, and a real direct call path is still required.
The initial fixture-only Nat annotation and graph-assertion failures remain
plain alongside their input/IR/executable evidence. Raw CLI PowerPC support is
not claimed; the maintained parent API gate emits the complete PPC object.

Both frontends accept the actual mixed IO/Mut API/factory proof, and both
negative proofs reach the specific `effect rows do not match` rejection. Earlier test-only
layout/reserved-name failures are retained and are not counted as intended
refusals. All 31 module declarations have canonical owned documentation without
foreign standard-library references.

`stdlib-byte-fields-results.json` records exact source, fixture, input snapshot,
LLVM/executable and compiler/runtime hashes. Completed successful functional
ELF/IR files may be recovered from the durable indexes; source snapshots,
reports, failed diagnostics, frozen/provider/runtime and benchmark inputs remain
plain. Initial Split and its two optimization candidates remain unchanged and
unselected.

The full strings/bytes initiative is still open. No throughput claim or
production optimization selection is made by this checkpoint.
