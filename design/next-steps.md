# Next steps (as of 2026-10-31)

The bootstrap continuation is now authorized (2026-10-30). Its self-hosting
acceptance and remaining frontend/backend work are in `self-hosting.md`.
The original merge status below records the starting point. The first
completed continuation baseline and its remaining repairs are recorded
separately; self-built frontend parity is still outstanding.

Where the work stands, what comes next in order, what needs a decision
from the user, and the working rules that kept parallel agents safe. Every
claim here is recorded in more detail in `compiler/CONFORMANCE.md`
(D388-D419) and the linked design notes. No em dashes.

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
  defined `Exhausted` exit (73), with stack overflow caught by guard bands;
  byte indexing is proven in bounds; conversions carry their ranges.
  Evidence comes from every `case` branch (`if`, guards, `&&`, `||` are
  case forms), and obligations discharge by difference logic, then
  Fourier-Motzkin, each proof with a re-checked certificate.
- **The original merged failure is fixed in the working tree.** The
  2026-10-30 merge baseline was 1081 passed, 1 failed. The completed serial
  first serial continuation run had 1102 passed and two failures. Both
  subsequent repairs are now verified in the complete frozen typed-data
  run: 1105 native compiler tests and 166 runtime tests pass, with zero
  failures or ignored tests. All six stage-0/stage-1 executable smoke
  checks passed too. Those are historical schema-4 execution results.
  A later run exposed shared-cache test binaries with live source paths;
  source content audits alone do not prove frozen runtime inputs. Newer
  schema-6 changes have current regressions and require verification with
  private caches. See section 2.0 and `self-hosting.md`.
- **Design directions recorded and decided:** representation option C with
  demand-driven specialization, compiler-chosen default layouts,
  expanding-edge boxing, representations as instances of `Represent t c`
  selected with `via` (`design/layouts-and-specialization.md`, D392-D394);
  allocators (`design/allocators.md`, D397-D398); floats as one unboxed type
  with proofs in refinements and unsafe, non-implicit IEEE instances
  (`design/float-refinements.md`, D414); verified primitives and proof
  automation (`design/verified-primitives.md`, `design/proof-automation.md`,
  D395).
- **The bootstrap ladder** now separates the pinned Rust seed (stage 0),
  the seed-built Cinch compiler (stage 1), and self-builds (stages 2/3).
  A schema-4 seed and stage 1 rebuilt and passed Fibonacci, precedence and
  named-refusal executable smoke checks before the typed-core changes.
  The heap-default repair refreshed the verified seed lock. Stages 0 and 1
  have now rebuilt with the typed-data continuation; all six fresh smoke
  checks pass in the completed full gate. The latest stage-2 attempt passes
  parsing, checked-interface loading and source namespace resolution,
  then exits 4 with 136 header diagnostics on schema 6, down from 232 on
  schema 5. Counts include cascading errors rather than independent missing
  features. Regular imported constructor
  refusals are gone. The remaining nominal refusals name `NonZero` and
  `InRange`, whose proof checking remains required. Header errors still
  prevent body checking and IR emission; stage 2 has not self-built.
  `Bootstrap.Type` and `Bootstrap.Infer` implement rank-one instantiation,
  rigid variables, occurs/scope checks, let generalization, annotated
  higher kinds, regular constructors, positivity and pattern coverage.
  Module aliases, private definitions and source provenance survive
  resolution. Lexical class givens and closed kernel IO effects now have
  focused coverage; global instances, dependent checking and the shared
  runtime lowering remain required. Pure typed
  terms now retain instantiations, constructor fields, generalized lets
  and scoped private types. Source data retain complete checked constructor
  schemes and parameter kinds. They are not yet the emitter's input.
  The reference-collection stack failure is fixed, with a regression over
  more than 6,000 declarations. All six affected checker/loader/parser/
  pipeline gates and the 11 orchestration tests pass. The completed native
  baseline's two failing targets pass after focused repairs; the self-built
  frontend's full behavior gate is still required. See `self-hosting.md`.
- **Interface contracts** and the pinned seed use schema 6. The seed patch
  and source hash were intentionally verified before installation. Stages
  0/1 rebuilt and pass all six executable smoke checks. Schema 6's native region/class and Cinch loader focused
  gates pass; resolved values retain Region binders, kernel Io contexts and
  closed kernel effect rows. Source IO checking passes 15 runtime-input cases,
  and all six affected regression targets pass. The full current-tree gate
  remains required.
  Schema 5 records proof preconditions and named
  refinements are joined by declaration kinds/implicitness, full constructor
  schemes and symbols, defining identities and explicit method visibility.
  Higher-kinded function quantifiers retain their resolved kinds. Pure
  schemes now retain definition-site nominal occurrences, private kinds
  and complete constructor identity sets; nominal dependencies record
  the checked interface hashes. Schema 5 adds pure class contexts,
  declaration-order dictionary slots, private class closure and portable
  pure instance heads while retaining conditional method proof contracts.
  The producer, loader, inference, Core and imported-class focused gates
  pass; calls use scoped givens and preserve dictionary arguments in ABI
  arity. The seed patch is intentionally refreshed and verified; stages
  0/1 rebuilt and pass all six smoke checks. The fresh schema-6 stage 2 exits
  4 with 136 header diagnostics, down from schema 5's 232. IO/region/effect-arrow
  refusals are gone; dependent/proof contexts and declarations still block
  body checking. It emits no IR or executable. The complete green
  regression baseline still belongs to schema 4. Complete interface-only
  checking needs richer evidence, coherence and ABI consumption.
- **Cinch interface loading** now checks real emitted contracts and their
  dependency closure, preserving the full JSON metadata and parsing
  selected value schemes with quantifiers and proof preconditions intact.
  Loader and parser-corpus checks passed with serial code generation. The
  loader is connected to parsing, resolution and pure polymorphic nominal
  checking; complete kind/coherence/ABI checking remains required.

## 2 Next, in order

### 2.0 Fix the one failing test

The latest continuation exposed and repaired an optimizer growth-bound
violation: a whistle retained the oversized expansion, leading inference
tests to emit more than 2 GB of LLVM. The driver now retains the preceding
bounded residual and discards the rejected round's helpers and report
entries. Two private-cache snapshots pass the focused bootstrap and optimizer
gates, including the formerly failing inference runner. The intentionally
updated seed source hash is
`475b8887d71c8becfd9b572897717ac58fac3d3df5db8294c2ed3ef2bb81fcec`.
Fresh staged builds and the complete current-tree gate remain required;
see `self-hosting.md` for exact snapshot identities and evidence.

Continuation: fixed in the working tree. Compile-time evaluation lacked
total integer widening and tried to decode `natFromI64`/`intFromI64` inputs
as mathematical integers before dispatching. The original failure, an
unsigned byte-boundary test and a staged signed/unsigned conversion test
pass in the completed full run too. That serial run finished with 1102
native compiler tests passing and two failures: `bootstrap_ladder` imports
`interface-global` from both Contracts and Nominal, and `prelude_split`
chooses a sequencing probe's scheme from unordered signature contracts.
The repairs keep one helper export home and choose the direct provider by
its canonical identity. The runtime's 165 tests and all stage-0 smoke
checks pass. A separate repair keeps an explicit heap cap on the quiet
allocation probe and checks default/fallback selection in a runtime unit
test; the runtime behavior is unchanged. The 24 affected compiler checks
and three heap unit tests pass. The runtime source edit required an
intentional seed refresh; that lock and its patched source hash are now
verified. Stages 0 and 1 have rebuilt; stage 2 still refuses the same 110
unsupported header contracts. The frozen schema-4 full gate completed on
2026-10-31 at 13:57 UTC: 1105 compiler tests, 166 runtime tests and all six
stage-0/stage-1 smoke checks pass, with zero failures or ignored tests.
The source audit confirms all 497 captured inputs are unchanged. The
snapshot SHA256 is
`fc21e0d1be1c5a53a1cd36d4b743e83c1ee38e3e6ba190f5acd4f1a319ad8dd3`.
Results and the audit are in `/tmp/cinch-typed-core-full/`. Schema-5 and
lexical dictionary changes postdate that snapshot and are being checked
separately; their first build caught a class lookup argument type mismatch,
which was repaired for its focused gate. These are historical execution
results. The later shared-cache provenance
failure means they must not be used as proof of the replacement frozen
verification setup. The schema-6 run had failed targets and was stopped
because its cached executables read live source paths. Focused repairs are
being tested before a new private-cache full run. The earlier overlapping gates were
interrupted by a machine crash. Verification now runs one command at a
time with one worker and test thread; interrupted gates are not passing
gates.

NF3b and NF5 (D418-D419) were merged with one failure outstanding:
`surface::bytes_literal_folds_and_map_literal_round_trips`. A bytes list
literal (`buf :: Bytes; buf = [1, 2, 3]`) no longer folds to a constant
buffer at compile time; the program still runs correctly. `FromList Bytes`
now takes `U8` elements (`Data/Bytes.cinch` `bytes-from-u8s`, through
`byte-of-u8`: `byte-word` then `nat-from-i64`), and `fold_constant`
(`check.rs`) returns `None` because the compile-time evaluator
(`eval_core`) fails somewhere on that path [INFERENCE: most likely a
widening intrinsic or the refined-result plumbing it does not evaluate].
Fix the evaluator (or the instance) so the literal folds again, and run
the full suite with no time limit. The same merge also changed one
diagnostic: a refinement flow that no fact discharges now
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
- The Cinch bootstrap parser now retains `using` predicates, refined
  results, index arguments, quantifier kinds and structured effect rows.
  Port their kind, dependent and effect checking into the Cinch checker;
  the current fragment refuses them explicitly. Parser coverage alone
  does not establish those obligations.
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
  For the current read-only Git setup, use
  `python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME`.
  Its source copy, Cargo caches and stage outputs are private to that run.
  Never symlink Cargo targets between checkouts: cached tests can retain a
  live CARGO_MANIFEST_DIR even when Cargo is invoked from the snapshot. The
  schema-6 run exposed this when its parser corpus read the new live proof
  module. The common harness now checks the embedded path against the active
  package root. The new verifier and its checks still need their first run.
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
- **Representation details** still open in
  `design/layouts-and-specialization.md` section 9 (value type spelling,
  `Copy`-only represented element types, specialized body spelling).
