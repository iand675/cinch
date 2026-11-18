# Keep constructor field reads visible to product-return lowering

The matched Fields experiment demonstrates a compiler opportunity rather than
a selected library change. Returning a three-word decoded step in registers
does not help when each caller reconstructs its box before calling two small
getters. Production UTF8 and Fields remain unchanged.

The rooted source-only probe retains the original public Main, Fields and
experimental UTF8 declaration bodies in one diagnostic entry bundle. This
makes the relevant library bodies visible in the Core dump while preserving
the original public root. Its exact compiler is Rust706/d91; Core hash is
`02fc8a7f9f9210d6fe2f0551d464c05eb0670cd09ee9cc704eee9723fccd7f23`.
The complete Core and trace remain plain at
`build/byte-fields-cpr-rooted-core-trace/`. The preceding library-only probe is
also retained: without a main or object root, it correctly prunes the functions
and cannot establish any inlining refusal.

The actual trace admits both UTF8 field getters with code size 5, `certain true`
and `folds true`, then refuses their calls from both scanners with reason
`fold` in all four passes. Their optimized bodies are one-arm constructor cases
returning a bound field. The width getter's lowered ownership form duplicates
the selected counted field before returning it.

`candidate` marks every parameter case as requiring a known constructor at a
duplicated call site. `inline_unit` applies that fold gate before the small-body
size bypass. As a result, even these single-arm field reads are left opaque.
The relevant source locations are the Benefit arm of `candidate`,
`case_scrutinee_params`, and the fold gate in `inline_unit` in
`compiler/src/simplify.rs`.

The proposed change recognizes only `case parameter of Constructor fields ->
selected-field`, with one constructor alternative and a directly returned
field. It bypasses only the known-constructor fold requirement. The case stays
in the copied body, so this does not infer nominal uniqueness from a numeric
tag, delete a check, or assume the argument is a known constructor. It gives
the copy no fold credit. Size, growth, loop allowance, recursion, depth, open
parameter and explicit no-inline restrictions remain in force.

Calls, computed scrutinees, multiple alternatives, computed results, allocation,
ownership wrappers and effectful work do not match the exception. Ordinary
strict inlining continues to bind computed arguments once, left to right,
including unused arguments. A local argument is substituted directly; every
copied binder receives a fresh identifier. Reference counting is derived after
the simplifier, so a selected owned field retains its required reference while
the remaining product fields are released. Existing foreign-body admission and
metadata checks are unchanged.

The proposed tests exercise default-model copies at unknown multi-site product
arguments and verify them with the ownership pass. Computed cell-read arguments
must stay in their original order, including an unused second argument. Negative
tests keep multi-alternative cases and computed case bodies behind the fold
gate and keep a configured size limit effective. A concrete source fixture
also returns byte owners and dynamically boxed naturals after their enclosing
product dies, churns 256 independent inputs, and observes strict unused
byte-write arguments. Its maintained runtime gate compiles checked-RC O0/O2
and runs normal 32 MiB and poisoned 256 MiB memory profiles with 8 MiB stacks,
retaining the complete project on failure.

The initial proposal was source-only. The fixture's first child source check
paused before creating a project because free space was below 350 MiB. Root
then integrated the isolated helper/admission hunk while preserving its newer
strict partial-application and nullary-global repairs. Its focused gates pass
all 48 simplifier tests and seven inlining tests, including actual checked-RC
O0/O2 normal/poison owned-field runs. The separate visibility gate passes ten
tests. These are parent-reported results, not child Cargo execution.

Root retained two fixture layout failures and corrected only the multiline
`accepted` binding and the parentheses around the conditional's case. The exact
accepted fixture is synced here, SHA256
`f2b1127e8e89fb7086a464f3e3c2ee9f39b984dfa3e686e88c770ac11a48398c`.
The original proposed source remains in commit `64ef15d7`. Broader parent tests
and an immutable compiler snapshot remain required before a new performance
experiment; the mutable warm compiler is not a benchmark input.

After validation, a matched experiment must distinguish production UTF8 plus
the compiler change from the reassociated UTF8 plus the compiler change. The
unselected reassociation already has a negative or mixed performance receipt.
Both actual aggregate consumption and owned-field lifetimes must be checked
before another source decision. This review does not claim a complete late-local
product propagation fix or a performance gain.

The subsequent immutable C2/d91 experiment has now completed both 650-profile
public preflights, 24 ordinary allocation diagnostics and 104 paired timing
observations. Exact data and ABI scope are in
`stdlib-field-projector-preflight-results.json` and
`stdlib-field-projector-benchmark-results.json`. Getter calls disappear, but
two expanded decoder branches still reconstruct the product. Default eager
allocation totals improve with the reassociated source; generic callback totals
do not. Mixed timing results, including 4 KiB and visitor regressions, keep
f516 unselected. A separate higher-count confirmation plan is prepared without
replacing the original raw samples or adopting a production source change.
