# Immutable PCG state candidate

`Data.Random.Pcg` provides explicit reproducible state for simulation and sampling. It is an isolated source-distributed candidate, with no production prelude or package-completion claim.

| Operation | Cinch surface | Adaptation |
| --- | --- | --- |
| Construct or reseed | `pcg-seeded high low` | Construct a new immutable replay point. |
| Draw a machine word | `pcg-next state` | Return `(word, successor)` explicitly. |
| Marshal a state | `pcg-encode state` | Fresh owned buffer, exactly twenty bytes. |
| Append a state | `pcg-append prefix state` | Copy the prefix and state; preserve caller aliases. |
| Restore a state | `pcg-decode bytes` | Return a named refusal or independent immutable state. |

The transition and output permutation use inline LLVM with full-width wrapping arithmetic. The 128-bit product allows the target backend to select its machine product/carry sequence. Consecutive states depend on their predecessors; this implementation does not claim parallel independent draws. Exact byte-buffer word reads use the existing bounded SWAR helpers. Snapshot writes preserve the existing checked byte-buffer access contract.

Independent reference observations use the public `math/rand/v2.PCG` API of the pinned Go1.27.4 SDK; `REFERENCE-LICENSE` retains its distribution license. The reference driver does not implement candidate arithmetic. A separate Python integer model checks all 2,048 words, sixteen initial/final state snapshots and eight prefix appends without reading the Cinch implementation.

The exact frozen `ffb5d39c` compiler and `bdf524d8` entropy runtime pass the SDK-free consumer at O0 and O2, each with normal32M and poisoned256M heaps,8M consumer stacks and zero leaks. Checks include immutable replay, decoded-state independence after input mutation, fresh encoding, exact snapshot widths and all four invalid prefix positions. All14 parsed declarations have canonical documentation without diagnostics or orphans. These runs do not consume operating-system entropy.

The first link attempt failed before linking because the LLVM shared-library search path was missing; its source, interfaces, emitted LLVM and original failure receipt remain plain in `build/pcg-functional-o0-v1`. The corrected runner sets the actual LLVM library path explicitly and uses fresh projects. Both completed successful functional output pairs have decoded-SHA-verified recovery archives with original mode, owner/group and nanosecond timestamp metadata. All source and interface inputs remain plain.

Remaining work includes public-pipeline benchmarks and allocation/code-generation review, long replay/state persistence and independent privacy controls, portable object checks and actual Native qualification. The wider random family still needs generic sources, bounded/scalar draws, distribution samplers, permutations/shuffling, ChaCha8, Zipf and explicit concurrent state capabilities. This candidate does not complete that package surface.

Run the independent model with `python3 experiments/pcg-v1/check_reference.py experiments/pcg-v1/vectors.json`. Generate the SDK-free consumer with `prepare_consumer.py VECTORS OUTPUT`; run one fresh project with `run.py --compiler COMPILER --runtime ARCHIVE --work NEW_DIRECTORY --opt 0` (or2). Only after both profiles pass may `archive_success.py NEW_DIRECTORY` replace the two functional outputs with verified recovery payloads.
