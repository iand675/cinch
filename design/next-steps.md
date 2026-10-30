# Next steps (as of 2026-10-30)

Where the work stands, what comes next in order, what needs a decision
from the user, and the working rules that kept parallel agents safe. Every
claim here is recorded in more detail in `compiler/CONFORMANCE.md`
(D388-D417 on `main`, D418-D419 on `wip/nf3b-nf5`) and the linked design notes. No em dashes.

## 1 Where it stands

- **Memory: GC-free reference counting is on by default** (`design/reclamation.md`,
  D389-D411). Header word, small `Nat`/`Int` immediates, the tag in the
  header, pointer maps, iterative drop, Perceus insertion with every
  parameter owned, mark-on-spawn atomic counts, a leak check and a
  use-after-free debug mode. Peak memory fell 30-90x on the measured
  programs, and they end with zero live blocks. `CINCH_RECLAIM=0` turns it
  off for baselines.
- **Faults: a well-typed program does not fault** (`design/no-faults.md`,
  D401, D407-D417). Pattern coverage counts no guarded row; division takes
  a `NonZero` divisor; `Nat` subtraction needs `b <= a`; exhaustion is one
  defined `Exhausted` exit (73), with stack overflow caught by guard bands.
  Byte indexing and conversion ranges are built on a branch (next bullet).
  Evidence comes from every `case` branch (`if`, guards, `&&`, `||` are
  case forms), and obligations discharge by difference logic, then
  Fourier-Motzkin, each proof with a re-checked certificate.
- **NF3b and NF5 are built but not on `main`.** They live on branch
  `wip/nf3b-nf5` (CONFORMANCE D418-D419 there): refined results with ranges
  (`InRange`), total conversions, total `Bytes` operations, and a
  Fourier-Motzkin fallback with Farkas certificates. The branch's full
  suite is 1081 passed, 1 failed; see section 2.0.
- **Design directions recorded and decided:** representation option C with
  demand-driven specialization, compiler-chosen default layouts,
  expanding-edge boxing, representations as instances of `Represent t c`
  selected with `via` (`design/layouts-and-specialization.md`, D392-D394);
  allocators (`design/allocators.md`, D397-D398); floats as one unboxed type
  with proofs in refinements and unsafe, non-implicit IEEE instances
  (`design/float-refinements.md`, D414); verified primitives and proof
  automation (`design/verified-primitives.md`, `design/proof-automation.md`,
  D395).
- **The bootstrap ladder** (the compiler written in Cinch) was paused until
  reclamation landed. It has landed, so the ladder can resume when the user
  says so.

## 2 Next, in order

### 2.0 Land `wip/nf3b-nf5`

One failure stands between the branch and `main`:
`surface::bytes_literal_folds_and_map_literal_round_trips`. A bytes list
literal (`buf :: Bytes; buf = [1, 2, 3]`) no longer folds to a constant
buffer at compile time; the program still runs correctly. `FromList Bytes`
now takes `U8` elements (`Data/Bytes.cinch` `bytes-from-u8s`, through
`byte-of-u8`: `byte-word` then `nat-from-i64`), and `fold_constant`
(`check.rs`) returns `None` because the compile-time evaluator
(`eval_core`) fails somewhere on that path [INFERENCE: most likely a
widening intrinsic or the refined-result plumbing it does not evaluate].
Fix the evaluator (or the instance) so the literal folds again, run the
full suite with no time limit, and fast-forward `main`. The branch also
changed one diagnostic: a refinement flow that no fact discharges now
reports "the obligation `v > 5` ..." instead of "does not flow to"
(`tests/lattice.rs`), which names the failing obligation but no longer
names both types; restore the type names in the message if that is
preferred.

### 2.1 Close the remaining reachable faults

The current list is in `design/no-faults-inventory.md` ("Runtime exits
classified", reachable rows):

- `sym-intern-bytes` on non-UTF-8 bytes: give it an `Option` front door or
  a UTF-8 evidence obligation (the documented `intern :: String -> Sym`
  is already total).
- `OutputFailed` (the OS refusing a stdout write) and the IO buffer paths:
  surface them as `Result` in the IO API rather than an exit.
- The cooperative scheduler's deadlock check (`coop.rs`): decide whether a
  deadlock is a typed outcome or ruled out statically for the structured
  concurrency the language admits.
- `f64-from-decimal` on untrusted text: an `Option` front door (belongs
  with the float slice).
- Foreign code: stays behind contracts and the manifest. Bring it to the
  user if it ever needs more than that (D407: anything unprovable comes
  back for discussion).

### 2.2 Replace quiet fallbacks with proofs

These sites are total today, but rest on an invariant the checker cannot
see, so on a broken invariant they now do something quiet instead of
faulting. Each should become a proof, an own `using` obligation, or a
visible `Option`/`Result`:

- `Data.Rational.normalise` (zero gcd; needs the positive-denominator
  invariant in the `Rat` constructor or a product-of-non-zeros rule).
- `Text.Builder` overflow, `Text.Zlib` writing past the end and `out-back`
  at distance 0, `Text.Regex` program counters and repeat bounds,
  `Text.Yaml` offsets clamped to the length, `Data.Date` `civil-field`
  negatives, `Text.Lex` lengths above 2^63, `Bootstrap.Interface`
  `bytes-splice` and `nat-to-u64` residues, `Bootstrap.*` `string-stop`,
  `key-anchored`, `slice-src`.
- Behavior choices the user may veto (D417, D419): durations clamp at zero,
  reversed byte ranges are empty, `BigDecimal.div` and `Enum`'s
  `pred`/`to-enum` return `Option`, the bootstrap evaluator reports a zero
  divisor as a diagnostic.
- The `NonZero Int` `Semigroup`/`Monoid` preservation law is sample-checked,
  not proved.

### 2.3 Floats (`design/float-refinements.md`)

Tier 1 (the predicates, `decide`, IEEE-correct facts) and tier 2 (refined
operation signatures, lawful instances on `NotNaN F64`, total
float-to-integer conversion from `InRange`), then the `unsafe instance`
mechanism for the IEEE instances. Needs the user's answer in section 4.

### 2.4 Reclamation follow-ons (`design/reclamation.md` section 10)

- S6b: borrow inference (skip dup/drop pairs around calls) and drop
  specialization.
- S7: reuse of uniquely held cells (count exactly +1; a marked block at -1
  must not be reused, D409) and static dictionaries for instances without
  sub-dictionaries.
- S8: cycle teardown per D396 (region-scoped `letrec` knots, enforced memo
  regions); the knot test leaks exactly 3 blocks until then.
- Conservative `Raw` slots that leak and are not marked for atomic counts:
  memo and knot values, dictionary value slots, joins wider than four,
  packed and boxed-out boundary words, builtin index-family constructors,
  compile-time constants built without type arguments. Give each its real
  class.
- S10: wasm headers and frees. Linux verification of the stack guard and
  thread paths (written from each ABI, run only on macOS arm64).
- Allocators A1-A4 (`design/allocators.md` section 6): the in-house
  small-object allocator, the user-supplied allocator interface and the
  static pool, the `Heap r` capability with arenas, then `#[fip]` and
  `#[alloc "bounded(n)"]` for the `jpl` profile.

### 2.5 Proof automation and the kernel

- The difference-logic and Farkas certificates (`DiffCert`, the
  Fourier-Motzkin certificates) are re-checked in Rust today. Move the check
  into the dependent kernel by reflection (`design/proof-automation.md`
  section 5).
- The bootstrap parser does not model `using` predicates, `refined`
  results or `InRange` index parameters; `bootstrap_parse`'s coverage table
  records them as unmodeled (`Data.Machine` 52 and others). Model them.
- `row_result` substitution in nested local definitions may pick the wrong
  names (rare; noted by the NF3b slice).

### 2.6 Representation and layouts (`design/layouts-and-specialization.md` section 8)

Value types and field unpacking, demand-driven multi-word layouts with
expanding-edge boxing, `specialize` bodies, representation instances
(`Represent t c`, `via`), the justification rule.

### 2.7 Older follow-ups still open

A list literal of 300 or more elements overflowed the CLI stack (recheck:
it should now be the `Exhausted` exit); top-level values are re-evaluated
per use (CAF); `Mode::Run` reports a signal-killed process as exit 1;
`Text.Builder` copies twice; `Eq_Bytes` could use `bytes-compare`;
`Text.Parse.render-errors` is quadratic; `Text.Json`'s duplicate-key check
is O(k^2); `Bootstrap.Ir`'s signature search is O(decls^2);
`Bootstrap.Driver` copies per byte.

## 3 Working rules that kept the agent waves safe

- **Memory.** Never run a compiled program without `CINCH_HEAP_LIMIT` (2G or
  less; `compiler/.cargo/config.toml` sets it for `cargo test`). Never raise
  a cap.
- **Cargo.** Never pipe cargo output through `head` or anything that exits
  early: SIGPIPE kills the build and leaves the old binary in place, which
  looks like a stale cache.
- **The full suite takes about 70 minutes** on this machine (over 110 test
  binaries, longer when agents run tests at the same time). Run it with no
  time limit, and after every wave, because slices verify only their own
  target lists: four waves in a row, the full suite caught a regression no
  slice had run (a test program using a now-refused form, or a typing
  change in a target outside the slice's list).
- **Gate a snapshot, not a moving tree.** When agents keep editing during a
  gate, stage the tree, `git write-tree`, commit that tree to a branch with
  `git commit-tree`, and run the suite in a worktree of that branch (`mwt
  new <branch>`); fast-forward `main` only if it is green.
- **Shared tree.** Agents never commit, branch, stash or reset; each owns a
  named region and messages the owner before touching another region; a
  slice blocked by another's in-flight edit closes out with deferred checks
  instead of waiting or fixing it.
- **Embedded and non-embedded prelude.** 55 prelude modules are embedded in
  the compiler and 38 are not; the 38 compile only through a `Main` that
  imports them (or their `lib_*` test). Programs embedded in
  `compiler/tests/*.rs` are separate again. A migration must check all
  three; `rg` over `compiler/tests` for the changed functions.
- **Agent budgets.** A slice agent stops at about 200-300 requests. Large
  migrations should delegate disjoint module sets to sub-agents from the
  start; a stopped agent can be resumed with a precise list of what
  remains.

## 4 Decisions waiting for the user

- **Float equality:** should the lawful instances on `NotNaN F64`
  (`totalOrder` equality, so `-0 /= +0`) be implicit, or must every float
  equality be selected explicitly? Also the unsafe module's name and the
  `Real64` alias names (`design/float-refinements.md` section 8).
- **The behavior choices** in section 2.2.
- **Resuming the bootstrap ladder** now that reclamation has landed.
- **Representation details** still open in
  `design/layouts-and-specialization.md` section 9 (value type spelling,
  `Copy`-only represented element types, specialized body spelling).
