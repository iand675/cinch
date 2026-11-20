# Compiler source-check budget review

The exact MVar v2 input remains51 modules,3,208,102 bytes and49,126 lines,
with source-set SHA256 `6bca85551971e5d783d709cb2b2cfb0f6dc9cf3ab2580a2da3359bc99932e465`.
No source or proposed pin changes follow from this review.

## Existing bootstrap policy

Reviewed against published Root checkpoint `94732a7954ec56ef6bde1e46383a858fa774d93c`:

- `tooling/bootstrap.py:167` invokes `subprocess.run` without a timeout.
- `build_seed` builds the pinned Rust compiler with `cargo build --release`.
- `build_stage(1)` uses that release seed to compile the entire compiler
  source closure, including every bundled module through qualified imports.
  Its `--opt-level 2` sets the generated Stage1 link optimization; it does
  not turn an already-running debug Rust checker into a release executable.
- Self-built stages likewise link at `-O2`. Their timings are separate from
  a debug Rust source-only invocation.

The existing script therefore supplies no60-second or180-second normal
source-check limit. Those were intentionally bounded diagnostic budgets.
`design/compilation-speed.md:28` records a historical22-second seed frontend
check of35,000 lines/55 modules. That is an older release-seed measurement,
not a current49,126-line debug-host completion budget. Its documented
Stage1-to-Stage2 compile is9 minutes, a different workload again.

## Observations and limits

On exact qualified330/d91, default checks passed the Builtin closure in9.484s,
ExportType/Nominal in22.656s, Resolve in15.378s and ExportNominal in48.596s.
The entire51-source set reached separate60s and180s bounds without a verdict.
CPU remained busy; the180s run sampled179.52 CPU seconds and1.13GiB sampled peak RSS.
These observations do not establish a regression or an ordinary completion
time. They establish only that this exact debug-host/default-mode workload
needs more than the granted180s, or contains work requiring investigation.

An older independent source check is preserved in
`/workspace/dasl-stdlib-source-body-closure/bootstrap/stdlib-body-authority-projection-evidence/full51-fa979-check-report.json`:
a different release producer fa979 checked an earlier51-source set in35.197s.
The same earlier source timed out118.135s under the newer debug37dc producer.
Those compiler/source epochs differ from current330/75c and cannot qualify
this MVar candidate or supply a same-epoch release/debug ratio.

The separate75c/d91 driver trace entered `check_program_multi_full_with_preludes`
after3.297528s and had not returned at its60s bound. Driver optimization and
artifact serialization had not begun. That call includes header/body checks,
laws, interface construction and prepared Core verification, so a typechecking
algorithm has not yet been identified as the bottleneck.

## Next diagnostic boundary

Separate source-only patch `63b90a8b`, based on Root94732a79, adds optional
internal checker timestamps under the existing `CINCH_TRACE_DRIVER_PHASES`
flag. It records headers, layouts/foreign contracts, embedded bodies, staging,
source-module body checks, post-body primitives/laws/rewrites, interface
construction, prepared finish and safety. Module fields contain names only.
`prepare` and `full` scopes identify distinct monotonic clocks. Output errors
are ignored; existing source statements, checks and artifact data remain
unchanged. Rustfmt syntax parsing and reconstruction of every original byte
passed; compilation, artifact invariance and actual traced execution remain
parent gates. No own Cargo build or additional source invocation is implied.

A longer default-mode completion budget should follow actual subphase
observations and resource coordination. Existing bootstrap policy permits a
multi-minute workload, but provides no evidence for a specific debug-host
upper bound. No algorithm or metadata-contract change is justified solely by
the bounded timeouts.

## Current release measurement

The exact current release08929f8d/d91 compiler, source2a558db0, passed the
unchanged complete51-source set in37.047s with default flags. Checking took
14.920s, optimization17.918s and serialization1.168s; all51 source bodies and
117 interface files completed. The separately retained debug timeouts remain
incomplete resource observations, not regressions. Parent also completed
339 release units in105.91s and70-artifact debug/release/trace invariance.
No compiler-source algorithm change was needed to close this source gate.
See `../evidence/native/native-mvar-current-release-source-results.json` for exact identities.
