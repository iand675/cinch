# Current release compiler qualification plan

This is a static plan, not build authorization or a qualification receipt.
Root source checkpoint is `94732a7954ec56ef6bde1e46383a858fa774d93c`.
At plan creation, internal checker tracing `63b90a8b` was isolated. Parent
subsequently reviewed/applied it as `2a558db0`, pending actual compiler gates.
The next release build must record that distinct source checkpoint and qualify
the internal trace as well; no existing frozen binary is relabelled.
All historical compiler, provider, source and timeout epochs remain separate.

## Inputs and dependency inventory

Root `compiler/target/release` is absent. Its manifest has11 direct library
dependencies plus the `cc` build dependency and no custom Cargo profiles.
The lock has106 packages including105 registry packages; SHA256 is
`2d52d5a1e37279e8fd1dc752bc6c0804ed9f6f31111c5a9761a513ee1ffdc332`.
That lock matches the earlier single-configuration release cache in the own
base-construction lane. Source directories for the Linux host dependencies
are cached. The19 missing locked source directories belong to Windows,
WASI, Redox or R-efi target families; those do not imply a Linux download.
Use locked/offline resolution and preserve any actual cache refusal.

Both repository Cargo configurations apply by running Cargo from Root's
`compiler` directory. Its build configuration supplies
`rustflags = ["-C", "codegen-units=1"]`. Do not replace that with a profile
codegen-units setting or invoke Cargo from the repository root; those forms
are not fingerprint-equivalent. Record the actual toolchain, configuration
hashes, profile environment and command. Do not copy any historical compiler
artifact, fingerprint or dependency cache into the new release directory.

The existing runtime is independently verified SHA256
`d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`,
24,662,528 bytes. At this review it also satisfies the driver's existing
mtime freshness rule. No runtime rebuild is expected. Recheck source bytes,
archive hash and freshness at execution; a mismatch requires its own reviewed
runtime epoch, not a timestamp adjustment or fallback.

## Estimated resources

The earlier single-configuration release cache retains355.8MB logical/
359.0MB unique allocated bytes:225.5MB external artifacts and130.3MB compiler
artifacts. It includes the39.46MB generated parser source. Its fingerprints
record the same `-C codegen-units=1` flags. This is a sizing reference, not a
cache to reuse or a current build measurement. Another historical cache has
two flag configurations and is deliberately excluded from the estimate.

Current compiler growth justifies planning for up to700MiB retained release
output and about1.0GiB peak growth while building/linking the CLI. The release
unit executable can add roughly100–250MiB transient growth. Require actual
free space of at least1.5GiB after the reviewed recovery, reserve350MiB, and
allow one output producer. Monitor actual free space and target growth at
short intervals; do not start the next command if its reserve is unavailable.
If these estimates are exceeded, preserve the build log and review resources
before continuing. No unreviewed cleanup is part of this plan.

Time estimates are provisioning ranges only:5–12 CPU minutes for a cold
single-job release CLI, then3–8 additional minutes to compile the release
unit executable and up to5 minutes for its339 tests. There is no retained
same-source cold release build duration establishing those numbers. Keep
regular progress updates and preserve a time-budget stop as an incomplete
build, not a compiler failure. The historical release source-check samples
are22s for an older35K-line source and35.197s for a different51-module source;
they support a300s initial current-release source-check budget with progress
samples, not a promised completion time.

## Ordered gates

1. After recovery and explicit CPU3 allocation, verify the selected Root Rust,
   prelude, grammar, manifest/lock and both Cargo configuration bytes against
   the source receipt. Capture unrelated Root documentation work separately.
   Use the normal default feature set, release profile, positive2GiB heap cap,
   64MiB host stack, one Cargo worker/test thread and no incremental output.
   Ensure environment profile overrides and RUSTFLAGS do not supersede the
   recorded repository configuration. From Root `compiler`:

   ```sh
   taskset -c 3 cargo build --locked --offline --release --bin cinch
   ```

2. From that same directory/cache/profile, run all339 library units:

   ```sh
   taskset -c 3 cargo test --locked --offline --release --lib -- --test-threads=1
   ```

   Record the actual count, including any added tests; do not filter failures.
   This exercises release host optimization rather than relabelling the
   already-green debug unit run as a release gate.

3. Hash/freeze the resulting CLI with exact source/config/toolchain/runtime
   provenance. Reuse the retained `build/driver-phase-artifact-invariance`
   Main/Carrier sources in separate output directories and fresh `CINCH_STORE`
   directories. Compare the70 actual emitted artifacts byte for byte against
   the qualified75c debug epoch and release trace disabled/enabled. Record
   exit status and monotonic driver markers. Do not rewrite interface hashes
   or reuse the previous output tree as this compile's result. Budget20MiB.

4. With the newly identified release CLI, check the unchanged MVar v2 source
   set `6bca85551971e5d783d709cb2b2cfb0f6dc9cf3ab2580a2da3359bc99932e465`
   using default check options and `CINCH_TRACE_DRIVER_PHASES=1`. Copy only the
   exact51 source files into a separate directory; preserve all330 and75c
   outputs. Start with a300s diagnostic deadline, periodic CPU/RSS/output
   samples, positive host heap cap, 64MiB stack and20MiB new-output cap. Stop
   at the first actual diagnostic or completed verdict. No optimizer/check
   bypass, old compiler fallback or source/interface shadow substitution.

5. Include the parent-applied internal checker trace in the new source receipt
   and inspect its actual phase markers. Every source change receives new
   hashes and qualification; it is not silently folded into a preceding
   release or debug epoch.

Even a completed source check qualifies only the compiler source under that
identified Rust compiler. Native MVar emission/ownership/runtime and genuine
source-absent/reexport/forgery gates, fresh provider admission, Stage1 and the
new Native-family fixed point remain open. Proposed pins are unchanged here.

## Approved cache recovery outcome

The separately reviewed180-file cold cache archive completed with full
encoded/decoded/payload hashes and member metadata verified before unlink.
It recovered227,540,992 allocated bytes before recovery-ledger overhead,
retaining all promised executable/provider/runtime/source paths plain. See
`../evidence/native/native-mvar-release-cache-recovery-results.json` for exact index/hash and
unit-size bounds. End-of-run free space was1,511,419,904 bytes, still below
the1.5GiB start guard; this outcome does not authorize a release build.
