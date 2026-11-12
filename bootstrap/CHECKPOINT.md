# Compiler checkpoint — 1 November 2026

Byte observations and strict-let proof identities are installed and validated.
Cinch retains the actual typed buffer through inference, substitution, private
refinement contracts and Core checking. The Native exporter emits a closed
kernel operation only for the checked primitive. Ordinary same-named functions
remain unresolved. ABI 5 supplies nonnegative, machine-sized byte-length facts
only for actual scoped byte buffers. Interface schema 7 and definition-owned
fixities are retained.

All 158 Native-assisted integration targets pass, together with compiler unit,
binary, documentation, runtime and tooling gates. Compiler units pass 140 tests
and runtime suites pass 166, with no ignored tests in those gates. This is not
the suite through a self-built frontend. The final validation snapshot contains
570 unchanged inputs, SHA256
`db363ca63a9fc22322ad420233ff206d00ffa879d7761a66c56a962e029ebe52`.
Twenty completed targets and ten focused targets retain byte-identical compiler,
bootstrap, prelude/runtime, shared harness and own-fixture inputs. The two
updated Rust fixtures are rerun; all remaining 127 integration targets and the
five general gates run fresh. Evidence and per-target reuse audits are in
`/tmp/cinch-byte-observer-validation-12c/`.

Strict monomorphic lets carry checked value witnesses separately from local
types. Core reconstructs them from each actual retained RHS. Nine source alias
cases pass, including refined buffers and precondition calls. The Type/Core
runner passes 30 checks at each of five runtime-generated identities above
2^128, including rejection of a copied alias certificate with a different RHS.
The imported NonZero runner passes 41 caller cases; wrong buffers, shadows and
missing conditions remain refused. Numeric interface tests retain ordinary
same-name refusals and check the actual closed primitive allocation promise.

The installed Native seed independently reproduces every checked source byte:
source SHA256
`74b4457a63c4898091ad5a31324a8ca098cf1aecfa789670926fff3be2f9b1ab`,
patch SHA256
`61685f05a4a781d58fa0417df2282402e9e2b76ad0fb1bb07303bcdfc6e25825`.
Archive plus patch is verified independently during preparation, staged builds
and installation. The Root installation preserves unrelated site/docs changes.

Fresh stage0 and stage1 builds pass all six smoke checks. The actual stage2
attempt exits 4 before body checking with seven header diagnostics, down from
66 at checkpoint 6d67ed6: decide/Yes/No evidence, two imported using contracts,
Text.Parse's effect declaration and Text.Json's instance declaration. All
byte-length/index-conversion header diagnostics are gone. Stage2 produces no IR
or compiler executable. Counts include cascades and do not measure completion.
The compiler bundle is
`ecebef95541d62e92f8020553668c31e4ab31fa152c136a2994ad906fa9b0a66`.
The unchanged staged snapshot has 570 inputs, SHA256
`85a0ddb9f861a44c76fb66635f8fde71c495073307eb15d05aa8db4d6fc7c232`;
its implementation is identical to this matrix, with the independently verified
seed files and preceding versions of the two Rust fixtures. Stage evidence is
in `/tmp/cinch-byte-observer-stages-12/`.

New machine-value/conversion support is under separate validation. Its Type/Core
runner passes 121 checks at five runtime-generated identities, and thirteen real
Cinch conversion cases pass. A private staged attempt passes all six smokes and
reaches six header diagnostics. These machine changes are not installed here;
plain machine declarations exposed a legacy-dispatch gap being checked in the
next candidate. This evidence does not establish a complete machine regression
suite or a self-built compiler.

Source declarations and decision/proof forms, checked imported measure contracts,
an independent global typing kernel and shared-runtime typed lowering remain
required. Actual-call probes identify nat-to-i64 and utf8-encode-into as the two
unresolved imports at this checkpoint. Stage2/3 fixed-point agreement,
changed-source behavior and all tests through a self-built frontend remain
unproven. Work uses at most five concurrent jobs, one Cargo worker and one test
thread per job, and generated Cinch heaps limited to 2G.

[checkpoint-results.json](checkpoint-results.json) records the installed seed,
complete Native-assisted validation and actual staged status.
[integration-matrix-results.json](integration-matrix-results.json) records exact
coverage and the audit for reused targets. Full logs and frozen snapshots remain
local. Self-hosting is incomplete; development continues toward the real stages.

## Lane notes: effects and library parity (2026-11-12)

Two changes were committed in a lane and then abandoned after measurement.
They are recorded here so nobody retries them.

**1. `#dependent-argument` bound as a bare local (rejected).** `Bootstrap.Infer.cinch`,
`materialize-dependent-view`, bound the fresh `#dependent-argument` local with
`MkLocalBinding` rather than the strict-let rule. The change was to route it through
`let-binding` and to widen `Bootstrap.Core.cinch`'s `stable-index-source` to accept a
pure global application. Neither moved a single test: `lib_uri_ipaddr` stayed 0 of 20
and the other four targets were bit-identical. The reasoning ("the two readings of one
buffer never meet") was right about the symptom and wrong about the lever, and it was
inferred from the diagnostic text rather than observed. Do not retry it.

**2. `lib_uri_ipaddr`'s actual mechanism, observed.** `lib_uri_ipaddr`'s 20 failures are
one cause. Every failing case builds a byte buffer in a call and indexes it in the same
expression, and the obligation arrives as `i < bytes-len #dependent-argument` while the
caller's guard states `i < bytes-len (mk n)`. Four probes settle it:

- `f b i = if i < bytes-len b then bytes-get b i == 0 else False` passes (same local
  on both sides).
- `if i < bytes-len (mk n) then (if i < bytes-len (mk n) then ... )` passes: the guard
  fact does form and is usable, so the branch-fact machinery is sound.
- `if i < bytes-len (mk n) then get-at (mk n) i` fails, with a *user-declared*
  `using (i < bytes-len b)` precondition, so this is not about `bytes-get`.
- Adding an explicit `let c = mk n` changes nothing.

The cause is that `scope-index-valid` (`Bootstrap.Core.cinch`) requires
`kind-equal (type-kind value) (value-kind carrier)`, so a `Bytes` local's kind is
`KType` and no index term can ever be recorded for it. A buffer's length therefore
cannot be carried by the recorded-value channel that makes two spellings of the same
index compare equal. A fix needs a definition channel for container values (or a
`byte-observation-reading` rule that follows a local to its strict let RHS), not a
change to the binding rule. That is a design decision, not a patch.

**3. Larger clusters, for whoever owns them.** `temporal` is 0 of 19 and most of
`staging` fails because the self-hosted frontend has no `contract` declaration in
`Bootstrap.Ast.cinch` or `Bootstrap.Parse.cinch` at all, so the declaration reaches
`Resolve` as `DUnmodeled` and is reported as `this declaration requires further
namespace elaboration`. `staging` quotations fail because
`Bootstrap.Parse.cinch:2070` sends `[| ... |]` to `parse-unmodeled-atom`.
`conditional_surface` fails because the staged CLI has no `--target` flag and no
target-profile concept. `lattice` needs `Ascii` and `Ordered`, which have no
representation anywhere in the bootstrap while the seed has a whole `PredFn` family.

**4. Three more attempts at the container channel, all rejected by measurement.** Recorded so
the next person does not repeat them.

- *A `definitions` field on `ProofScope`* carrying `(Nat, CoreTerm)` pairs, with
  `byte-observation-reading` following a local to its definition. Every construction site was
  updated and the record compiled, but any run that actually recorded a term failed the closure
  check with `typed core contains an unresolved scheme or error term` at span `1:1`
  (`Bootstrap.Infer.cinch:4900`). Disabling the recording made the tree build again, so the
  trigger is the stored term meeting `core-definition-checked-contracts-closed`, not the record
  shape; restricting the recorded shapes to closed ones did not help. A `CoreTerm` in the proof
  scope needs the closure walk to account for it, which is a bigger change than the record.
- *A container equation through the existing fact channel*: `scoped-core-container-let-fact`
  emitting `PCompare Equal (byte-length-variable id) reading` for a `Bytes` let, wired into
  `proof-let-env` in `Bootstrap.Infer` via an exported `let-facts` combinator. It built through
  stage3, but `lib_uri_ipaddr` stayed 0 of 20, so it is inert on this path.
- *Replacing `proof-let-env`'s single `scoped-core-let-fact` with the two-channel `let-facts`*
  broke the prelude at `Bootstrap.Nominal.cinch:58:90` (`NonZero 64`), because two facts
  prepended in sequence change discharge order. Restoring exact single-fact semantics for the
  numeric case fixed the prelude and kept the build green, which is why that shape is the one to
  start from if anyone tries again.

**The remaining gap, stated precisely.** The guard states `#value:i < <reading of (mk n)>`; the
dependent application states `#value:i < #bytes-length:<dependent-argument>`. Making those meet
requires the dependent argument's let to carry the same reading the guard used. Both sides go
through `actual-index`, and the fact channel can express the equation, so the missing piece is
narrower than "a definition channel for containers": it is that the guard's reading and the
dependent argument's reading are computed at different times from different terms, and nothing
currently relates them. Instrument `actual-index` on both sides and print the two `Arithmetic`
values; if they are already equal the bug is in entailment, and if they differ the fix is to
make the dependent let reuse the guard's reading rather than recompute it.

**5. The two-sided measurement, and what it overturns.** Main asked for the guard's reading and the
dependent application's reading printed side by side. I built exactly that: a `goal-trace` in
`Bootstrap.Core` (rendering the goal's core predicate and every hypothesis with `proposition-equal`
per hypothesis) emitted from `Bootstrap.Infer`'s unproved-call path. On the nine-line repro it
prints:

```
GTRACE goal=(#value:22 < #bytes-length:26) hypotheses=[(#value:21 >= 0) eq=no;
  (#value:22 >= 0) eq=no; (#bytes-length:26 >= 0) eq=no;
  (#bytes-length:26 <= 9223372036854775807) eq=no]
```

There is **no guard fact among the hypotheses at all**, so the two readings are never compared. That
overturns the conclusion recorded above in note 4: the missing piece is not a definition channel for
container values, and not "the two readings differ". It is that the guard's fact is absent from the
hypothesis set at the obligation site. Section 4's "remaining gap, stated precisely" is therefore
wrong and should be read as superseded by this note.

What the same probes still establish, unchanged:

- `proof-branch` does introduce a fact for this shape: an instrumented `scoped-core-branch` call
  reports `fact-introduced`, and `proof-facts` grows by one, on prelude code.
- A nested identical guard over a call-built buffer (`if i < bytes-len (mk n) then (if i <
  bytes-len (mk n) then True else False) else False`) checks clean, so the fact both forms and is
  usable when the use is a nested guard rather than a call with a precondition.
- A guard over an intrinsic call (`bytes-new n`) nested inside itself also checks clean.
- The failing shape differs in exactly one way: the use is a *call whose callee carries a
  precondition*, so the obligation is discharged by `discharge-checked-call` rather than by
  `scoped-core-branch`.

So the two candidate repairs, and which the measurement supports:

1. *A container-valued reading channel keyed on the type rather than the kind* (relaxing
   `scope-index-valid`, `Bootstrap.Core.cinch:1731`). **Not supported.** Nothing in the measurement
   needs a buffer to hold an index reading; the goal's reading `#bytes-length:26` is already
   computed correctly from the local.
2. *A reading attached to the let and followed at the use* (the seed's
   `IndexSubstitution::LocalValues` in miniature). **Not supported either**, for the same reason: the
   goal side already reads correctly.

What the measurement does support is a third, narrower repair: find why the guard's
`MkProofBinding` does not reach `discharge-checked-call`'s `proof-facts evaluated`. The prime
suspect is `proof-env-after-writes` in `Bootstrap.Infer` (called with
`call-operand-writes` of the term), since `proof-facts` only admits `MkProofBinding origin None
fact` and drops any entry carrying a write. `mutable-fact-write` should not fire for a `<` fact
(`fact-written` only matches borrow-read variables of mutable locals), so either the entry is
dropped elsewhere or the guard's fact never entered this env. Print `proof-facts` immediately
before `discharge-checked-call` and again inside it; that is one more measurement, not a redesign.
