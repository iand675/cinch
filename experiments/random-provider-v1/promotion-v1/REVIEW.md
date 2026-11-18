# Minimal Root promotion proposal

`runtime-provider.patch` applies cleanly, read only checked, against Root commit `c7e9133bfcbd58cf1aafb882f53f3433bbf00214`. All seven baseline runtime/ABI/Cargo source hashes match the original baseline manifest, including platform F4F0. No Root source was edited by this lane.

The production patch changes only `compiler/runtime/src/lib.rs`, `platform.rs`, `sys/mod.rs`, `sys/linux.rs`, and the new private `entropy.rs`. It adds the Linux raw syscall seam and the documented exact-fill provider, retaining six deterministic helper tests under `cfg(test)`. It adds no crate dependency, C `getrandom` symbol, public ABI/layout/error-vocabulary change, compiler/provider pin change, or global descriptor cache. The final `sources/` copies and expected hashes are in `manifest.json`.

The separate test-only `entropy_boundary_tests` module and its library declaration are excluded from this production patch. `optional-maintained-boundary-tests.patch` provides the exact qualified boundary tests if Root chooses to maintain that gate. The measured non-test staticlib excluded those tests; nevertheless a newly integrated Root archive needs its own identity and qualification rather than being called 663D or D91. `optional-provider-docs.patch` only updates the existing public module comment that currently names `/dev/urandom` as the sole source; it leaves declarations/bodies unchanged.

The production patch also corrects the historical Linux module header that said neither architecture had run. Its replacement claims only the observed x86_64 entropy/provider/ABI/public-consumer path, explicitly excluding a new claim for every syscall or other targets. `linux-host-evidence-docs.patch` and the manifest separately identify that comment-only delta from the qualified Linux source SHA70C4. Executable source below the import boundary is byte-identical. The original qualified provider patch and source epochs remain unchanged.

The measured evidence supports a Linux provider improvement: about 3.7–3.8× faster 1/16/32-byte draws and 3.06×/2.64× faster version4/version7 UUID pipelines in the single complete matched window. All 13 conditions, full ranges, paired ratios, controls and regressions remain under `benchmark-v1/timing-full-v1`. The 4096-byte allocation control regresses 1.79% by median, while counter/clock medians are near one and retain large individual outliers. The SDK still leads most entropy/UUID conditions and uses an observed secure kernel vDSO path. No SDK parity or universal speedup is claimed.

Observable compatibility points require explicit integration review:

- Linux uses flags zero, waiting for cryptographic generator initialization. This strengthens early-boot readiness relative to the previous device path.
- Successful nonempty draws no longer require `/dev/urandom` to be mounted or a free file descriptor. This is an intended provider behavior change.
- `EINTR` retries preserve progress. Only `ENOSYS` delegates the unfinished suffix to the existing device provider; permission denial and other failures remain failures, with the same named error mapping.
- Empty draws preserve the device-open check and fresh owned empty Bytes result. Other hosted platforms retain their existing provider, and freestanding targets remain excluded.
- Partial success never exposes incomplete bytes. Allocation, boxing, `KIND_BYTES`/`RC_FRESH` and both release paths are unchanged and independently checked.

The actual host executes the new syscall path and public consumers; deterministic injectable tests cover interrupted/partial/ENOSYS/ordinary-error branches. No natural or process-injected ENOSYS/EPERM execution, older-kernel boot scenario, or new cross-target runtime execution is claimed. Those limits stay distinct from the successful host gates.

The evidence manifest links exact raw/source/compiler/runtime/consumer/measurement inputs. Root should apply the source patch in its integration lane, run its required maintained gates under normal build resource rules, and preserve the existing frozen D91 archive. A new integrated archive/hash and any eventual rollout decision remain Root's next steps; this lane has not changed Root or run another build/timing window.
