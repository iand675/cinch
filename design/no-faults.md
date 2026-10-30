# Bulletproof programs: a well-typed program does not fault

Status: constraint set by the user (2026-10-29); the model below is a
proposal for review. The inventory of today's fault paths is
`design/no-faults-inventory.md`.
Companions: `design/reclamation.md`, `design/allocators.md`,
`design/safety-assertion-attributes.md`, `design/refinement-types.md`,
`design/proof-automation.md`. No em dashes.

## 1 The constraint

The user, verbatim: "a constraint I haven't really laid out is that I'd
like to not support faulting, or at least make it incredibly onerous to do.
if you can make a cinch program compile, it should be bulletproof,
basically".

Definitions used below:

- A **fault** is an abnormal stop the program's types do not declare: a
  runtime check that exits (exit codes 70 and 71; 72 was division by
  zero, retired 2026-10-30), a native stack
  overflow, an abort.
- A **typed failure** is an outcome the types declare and the program must
  handle or pass on: a `Result`, or the failure branch of a resource
  boundary (section 4). Typed failures are allowed; they are how a
  bulletproof program says "this can go wrong".
- A **compiler-bug trap** is a check that fires only if the compiler or
  runtime is wrong (a corrupt block kind, a broken invariant). It is not a
  program fault: no well-typed program can reach it if the compiler is
  correct, and long term the verified-primitives work
  (`design/verified-primitives.md`) removes the need to trust that.

The rule: **every fault a program could reach is either statically
impossible or turned into a typed failure. A source-level way to fault on
purpose exists only behind an onerous, recorded opt-in (section 6).**

## 2 What this changes in decisions already made

| Earlier decision | Under this constraint |
| --- | --- |
| Count overflow faults (D398) | Impossible by construction. A count can never exceed the number of words that could hold a pointer to the block. The allocator caps the total heap below `8 * 2^count_bits` bytes (with a 40-bit count, 8 TiB), so overflow cannot happen, and the increment needs no check. The layout-id build option trades count bits for layout bits and lowers that cap accordingly. |
| Static pool exhaustion faults (`design/allocators.md` section 3) | A `jpl` program's pool is sized from its proven bound, so exhaustion is impossible after startup. Code without a proven bound allocates under a typed budget boundary (section 4). Reserving the pool can fail only before any user code runs, as a refusal to start. |
| Arena teardown check faults (`design/allocators.md` section 4) | The static escape rule is the guarantee. The runtime check stays only as a compiler-bug trap in debug builds. |
| Teardown fault sentinels for wrong escape proofs (D396, reclamation section 6) | The escape checker is the guarantee; the sentinel is a compiler-bug trap. |
| `CINCH_HEAP_LIMIT` exit 73 | A development and test guard against runaway programs; since reclamation it caps live bytes, and exit 73 is the one `Exhausted` outcome (section 4, as built). A program's own memory budget becomes a typed boundary (section 4). |

## 3 Partial operations become total

Each operation that faults today on some input gets a total form, by one of
three means, in this order of preference:

1. **Evidence in the type.** The operation demands a proof that the bad
   input cannot occur: a divisor of type `NonZero` (exists), an index of
   type `Fin (bytes-len b)` or a refinement `i < bytes-len b` discharged by
   linear arithmetic (`design/proof-automation.md`), a subtraction `a - b`
   on `Nat` that requires `b <= a`. Zero cost at runtime.
2. **A total result type.** `Option`/`Result` when the caller cannot know in
   advance (parsing, conversions out of range, lookups).
3. **Total by definition**, where a mathematical convention is standard and
   unsurprising: truncated subtraction (`monus`) as a separately named
   operation, saturating or wrapping machine arithmetic as named operations
   (the primitive types specification already distinguishes them).

The faulting forms are removed, not kept beside the total ones. The
inventory counts the call sites each change touches.

Already static and staying so: pattern-match exhaustiveness (checked; a
missing constructor is a compile error), the `Eval r` rule for memo groups
(no black hole is reachable from a checked `~` group), usage and escape
checking.

## 4 Resource exhaustion becomes a typed failure or a proven impossibility

Memory and stack are finite, and for a program whose allocation is not
bounded, no static check can rule exhaustion out. So there are two tiers:

- **Proven bounds (the `jpl` tier).** A program whose reachable code is
  `#[no-alloc]`, `#[fip]` or `#[alloc "bounded(n)"]`, with
  `#[stack "bounded(n)"]` / `#[recursion "bounded(n)"]`
  (`design/safety-assertion-attributes.md`), reserves its heap and stack at
  startup from the proven sums. Once it starts, it cannot exhaust.
- **Typed budget boundaries (everything else).** Exhaustion is a declared
  outcome at a boundary:

  ```
  with (heap = budget (mebibytes 256)) in parse-all input
    :: Result Exhausted Summary
  ```

  An allocation that would exceed the budget abandons the computation back
  to the nearest boundary, which returns `Exhausted`. Reference counting
  makes this clean: everything allocated inside the boundary that is not
  reachable from outside is released deterministically on the way out,
  as on a normal return. `main`'s own boundary is the program's declared
  result, so a program either proves its bounds or declares that it can run
  out of memory; it never silently aborts.
- **Stack.** The native stack is a second resource with the same rule.
  Deep non-tail recursion either has a proven bound or draws its frames
  from the heap budget (a growable, heap-backed stack for unbounded
  recursion), so stack exhaustion becomes the same `Exhausted` outcome
  instead of a guard-page crash. Today's 1 GiB reserved stack
  (`cinch_run_main`) is an interim measure.

### 4.1 As built (2026-10-30): the implicit `Exhausted` at `main`

Section 7's decision is implemented in the runtime
(`compiler/runtime/src/abi.rs`, `heap.rs`, `stack.rs`; tests in
`compiler/tests/exhaustion.rs`). Every resource exhaustion is one
outcome: exit code **73** and exactly one stderr line naming the
resource.

| resource | when | stderr line |
| --- | --- | --- |
| heap | the live heap (bytes allocated and not freed) would pass `CINCH_HEAP_LIMIT` (default 8 GiB), or the system allocator refuses, or a request is too large for any allocation | `cinch: exhausted: heap (<n> bytes)`, `<n>` the live bytes the program asked to hold |
| stack | a thread running Cinch code (the program's thread or a task's) overflows its stack, or the evaluator passes its nesting bound (`MAX_NESTING`, which stands in for the stack) | `cinch: exhausted: stack` |
| threads | the OS refuses the program's thread or a task's thread (a refusal for memory, `ENOMEM`, reports `stack`) | `cinch: exhausted: threads` |

73 was already the heap cap's code, the only resource exit before, so
every existing check of it keeps its meaning; it is distinct from the
fault codes 70-72, from the compiler-bug traps (71 with a message
starting `compiler bug:`) and from the leak check's 74. A `main` may
return any `Int`, so a program's own result can equal any of these; the
stderr line is what tells the outcome from a result.

Stack overflow is detected with a guard band plus a signal handler on
an alternate stack. Each thread that runs Cinch code runs it inside
`stack::guarded`, which makes the bottom 64 KiB of the thread's own
stack inaccessible (above whatever guard the C library put below it),
gives the thread an alternate signal stack, and records the band's
address in a thread-specific key. The `SIGSEGV`/`SIGBUS` handler takes
the `Exhausted` exit when the fault address lies in the calling
thread's band (or the 64 KiB below it, the C library's guard), through
`_exit`; any other fault reinstates the previous disposition and
re-faults, so a genuine crash stays a crash. A frame can only skip the
band if it is larger than the band and unprobed, so every generated
native unit carries `"probe-stack"="inline-asm"` (each page of a large
frame is touched in order); rustc probes the runtime's own frames on
x86_64 and aarch64. Explicit depth counters were rejected because they
cost every call and cannot see the C library's frames; a heap-backed
stack (section 4 above) remains the long-term answer for unbounded
recursion. Verified on macOS arm64; the Linux x86_64 and aarch64 paths
(glibc and musl signal layouts, `pthread_getattr_np`) are written from
each ABI and not yet run.

Reaching the `Exhausted` exit leaves through `exit` for the heap and
threads (so `CINCH_ALLOC_STATS` still prints), and through `_exit` for
the stack, the only exit a signal handler may take. When two threads
exhaust at once the first one's line and exit win; the others wait.

## 5 Foreign code

Foreign functions can do anything their language allows. They stay behind
`foreign import` with contracts, recorded in `assumptions.declared`, and a
profile may forbid them or require them to run inside a sandbox (the wasm
component target turns a foreign trap into a typed failure at the
component boundary). A foreign call is the one place where "the compiler
accepted it" cannot by itself mean "it cannot crash", so it is always
visible in the manifest.

## 6 The onerous opt-in (withdrawn for now, section 7)

A deliberate fault (an `unreachable` the programmer cannot yet prove, an
`assert` on runtime data that stops the program) is allowed only:

- behind a definition-level attribute naming the reason, for example
  `#[may-fault "why this is acceptable"]`;
- recorded in the artifact's `assumptions.declared` and the build manifest,
  so every faulting definition in a shipped program is listed;
- rejected by default: the default profile refuses it unless the package
  manifest opts in explicitly, and the `jpl` profile refuses it always;
- never inferred: calling a `#[may-fault]` function requires the caller to
  carry the attribute too, or to discharge the condition with a proof.

A runtime `assert` that the program can recover from is a typed failure
(`Result`), not a fault, and needs no opt-in.

## 7 Decisions (user, 2026-10-30)

- **No `#[may-fault]` for now.** Every fault must be proven impossible.
  Section 6's opt-in is withdrawn unless something compelling turns out to
  be unprovable, which then comes back to the user for discussion.
- **`Nat` subtraction:** the plain operator `a - b` requires evidence that
  `b <= a`. The truncating form exists under its own name,
  `saturating-sub` (`max(a - b, 0)`; mathematics calls it "monus", Rust
  `saturating_sub`), matching the prelude's machine-scalar vocabulary.
  As built (2026-10-30): an instance block may restate a class method
  with obligations over the method's own arguments
  (`(-) :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat` in
  `Sub_Nat`). Each direct, full application discharges them where it
  stands, and the dictionary of such an instance is refused wherever
  it could reach code that applies the method without them (generic
  `using (Sub a)` code, a sub-dictionary, a module argument, the method
  used as a value). The class and `-` at `Int` are unchanged. A
  definition's own `using` obligations hold in its body, so a
  recursion can carry its bound (`using (1 <= depth)`). `checked-sub`
  answers `None` when `b > a`.
- **Exhaustion at `main` is implicit for now.** Every program's declared
  outcome includes running out of memory or stack, reported as one defined
  `Exhausted` exit, never a crash. Budget boundaries inside a program
  (section 4) stay available for code that wants to handle it.

## 8 Open questions

- Indexing: `Fin`-typed indices, refinement predicates, or both; how much
  of the proof burden linear-arithmetic automation removes in practice.
- The budget boundary's spelling.
- Heap-backed stacks: segmented stacks (Go's history: split stacks were
  replaced by copying stacks) versus a per-boundary stack reservation.
