# Resume the verified parent prefix without rebuilding

This is a source-only proposal. No Cargo, test, archive, deduplication or resume helper was executed by the child. Python AST checks and complete helper/class/method documentation checks pass. Root's original v1/v2 driver, common helper, projects, logs, markers, archive payloads and indices remain unchanged.

The actual compiled target has **45 tests: 42 owned JSON functions plus exactly three imported `common::external_frontend::profile_tests`**. The baseline is 31 owned + 3 shared = 34, followed by eleven new owned functions. The three shared controls only construct/check profiles and emitted request flags; they do not create Cinch projects or invoke compilation. Root v1 correctly refused the earlier `42` expectation before any test execution. Root v2 accepted the complete 45-name inventory. This corrects earlier owned-only count wording additively; no test is dropped or introduced.

Root v2 already closed tests 0–14. The fifteenth successful boundary was `json_map_owners_callbacks_and_large_values_are_bounded`. Index15 (`json_map_raw_names_match_independent_observations`) never launched: its prestart resource guard refused. The existing empty index15 directory remains untouched. The two-project Dynamic test at index7 has its original explicit 64 MiB approval and completed evidence; it is not repeated or silently granted to any later test.

The new entrypoint requires a prior epoch and existing executable. It has **no Cargo invocation, no target-cache inspection/build path, no no-run build and no library restoration path**. Normal rlib/rmeta recovery index `1e2c0ab4...` is bound as evidence only. The actual existing test executable remains plain and is required at SHA `025505f4c661b8a7787e5323a9d9851bbca51b63cfc122589df2b480d306cee0`; producer/context/runtime bindings remain the original v2 `040ade18...` / `58d8b11c...` / BDF `bdf524d8...`, with retained test-source SHA `8d7c84d5...`.

Root reviews the new sources and then uses its existing environment:

```bash
source /workspace/.tools/cinch-env.sh
python3 -B /workspace/dasl-stdlib-json/bootstrap/repros/json-parent-resume-v1/run_json_parent_resume.py \
  --compiler-root /workspace/dasl-stdlib-io/compiler \
  --prior-gate /workspace/dasl-stdlib-io/build/json-parent-retained-v2 \
  --existing-test-executable /workspace/dasl-stdlib-io/compiler/target/release/build/cinch/a9b8eddd120ee560/out/lib_json_codec-a9b8eddd120ee560 \
  --work-dir /workspace/dasl-stdlib-io/build/json-parent-retained-resume-v1 \
  --apply-passed-input-dedup
```

The new work directory must not exist. The supplied binding pins 119 prior metadata/log files, the exact ordered 45-name inventory, the prior driver/common-helper hashes and all identity fields. Before creating the new epoch, read-only verification validates all fifteen closed test receipts, process receipts, actual exit0/one-test-success logs, marker hashes, full memory-profile observations, archive-index hashes and decompressed archived bytes. Archived `consumer`/`consumer.ll` members must match the marker and index SHA/size/mode/uid/gid/exact nanosecond timestamp. No old executable/LLVM output is extracted or restored.

Every remaining plain project file is verified against its original marker. A changed mtime is accepted only when the already applied, hash-linked dedup plan records that same original metadata and a verified earlier eligible canonical file proves the current shared timestamp and inode. Byte/size/mode/uid/gid differences always refuse. Canonicals are rebuilt only from completely verified closed successful projects and eligible local source/CIF files; provider/ABI/runtime/compiler/benchmark/frozen inputs remain excluded. The new epoch can share against this proven prefix without writing into or recompiling it.

The initial 350 MiB guard and each next-test prestart guard now **wait without a child or new unit/log writes** until actual headroom passes. They never abort or discard the fifteen closed results. Once active, the reviewed 270 MiB soft pause, 250 MiB reserve, explicit numbered Root resume-request protocol and 30 MiB complete-test/10 MiB archive caps remain. There are no automatic cap changes. The remaining raw edge/kernel two-project test may need its own separately reviewed scoped cap; the earlier Dynamic allowance does not apply to it.

Exactly tests15–44 run in a new epoch on CPU3, each with the same frozen existing test executable and original fixtures. Whole-process success and every expected project marker precede archival/dedup. Newly continued archives use **gzip level6** with unchanged PAX nanosecond metadata and full compressed/decompressed/member verification; indices record the new helper SHA and codec parameters. This compression change applies only to newly created archives. Every prior payload/index/helper byte is retained unchanged. Source/CIF files remain plain, failure artifacts are retained, and the dedup ledger remains bounded at 5 MiB per epoch.

Only after all remaining thirty close does the final `gate.json` join the fifteen prior proof references and thirty new proof references into the exact 45-test ordered inventory. The prior prefix is read-only revalidated again. No child standalone 855/c4 proof is relabeled as this actual Root producer. Native, performance, Effect.Json and package-completion claims remain open.
