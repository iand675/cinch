# Buffered nominal Region proposal

This is an isolated declaration-only proposal alongside the already isolated both-effect source candidate. Maintained sources, historical checked interfaces, frozen tools, providers and all historical evidence remain unchanged. The fresh source and public metadata gate below applies only to this separate source epoch; runtime qualification and production selection remain open.

The genuine defining-interface audit records `Effect.Buffered.Core.BufferSource`, `BufferSink`, and `Internal.Cell.IoCell` with a nominal `Type` parameter applied to explicitly quantified `Region` variables. It also supplies direct witnesses for `Effect.Stream.ByteReader` and `ByteWriter`. `proposal.json` retains those exact schemes and the audit's identity. Its audit input hashes describe defining interface files, not source files; original/proposed source hashes are recorded separately.

The proposed closure annotates ten headers in seven source copies:

| Module | Explicit Region parameter |
| --- | --- |
| `Effect.Buffered.Core` | `BufferSource`, `BufferSink` |
| `Effect.Buffered.Internal.Cell` | `IoCell`; its element parameter remains a value type |
| `Effect.Buffered.Reader` | `BufferedReader` |
| `Effect.Buffered.Writer` | `BufferedWriter` |
| `Effect.Scanner` | `Scanner` |
| `Effect.Buffered` | `BufferedReadWriter` |
| `Effect.Stream` | `ByteReader`, `ByteWriter`, private `WriterChain` |

The five additional headers follow exact defining-constructor uses of the directly witnessed or proposed Region-indexed types. Their private/public status and constructor witnesses are retained. Every existing `forall r` in these copies explicitly quantifies `(r :: Region)`. The four both-effect source copies retain their existing read/transfer rows; stream operations remain IO-only, ordinary sink writing/text/flush remains IO-only, and all pure getters/configuration remain pure. The private initialized slot cell and all existing intrinsic declarations/bodies are untouched.

`verify.py` checks the audit identity, each defining witness, all source hashes and that erasing only the added annotations yields the exact original input bytes. This static check does not establish new public/private compiler metadata. Fresh source gates on qualified330/d91, defining-contract observations, ownership/effect refusals, complete library and runtime evidence are still required. Historical855/2f results must not be relabeled as nominal Region qualification.

The optional live Multipart closure belongs to Compare's isolated19-header proposal. Its new source epoch is separate from the old checked Type-head source; Compare reports qualified330/d91 source,12 public plus7 private contracts, and three live factories. That closure currently retains historical Var1940d102 and Stream e2c2dfb9 sources. We therefore do not claim that330 already forces every transitive migration, or that those old checked families were silently rewritten. Any combined live-adapter gate must select and record that actual new dependency epoch explicitly.

The scanner optimization is also separate. This proposal starts from the historical full80 library plus the both-effect source copies; no optimized scanner selection or callback-cycle reclamation is introduced.

`check.py --work <new-output-directory> --deadline 60` prepares all fourteen source-distributed dependencies, including all seven explicit overrides, and checks them with the qualified `330adb4d` compiler and matching `d91ce012` runtime identity. The captured 66-module embedded-family manifest comes from the exact compiler source checkpoint `75b4bab7`; it is a source-selection input, not a generated interface. The runtime is identified but neither initialized nor linked by this source gate. Fresh global disk guards are 350 MiB at start and 250 MiB before each phase, with at most 30 MiB of persistent outputs. A timeout has no verdict. Every generated defining interface stays plain and unmodified. A successful source check additionally reads the nine public nominal contracts and constructor inventories; the private `WriterChain` observation remains a separate future source gate.

`source-gate.json` records the positive result: the complete closure passed source, type and proof checking in 11.06 seconds, using 3,499,827 bytes of artifacts. All nine public types have genuine defining `Region` parameters and exact constructor inventories. Eighty-one compiler-produced interfaces remain plain at the recorded work path; embedded sources were not shadowed. The five direct audit witnesses and five constructor-closure proposals are still distinguished, and the scalar payload parameter of `IoCell` remains `Type`.

`observe_private.py` is prepared for a separately authorized source-only metadata check. It requires the positive source receipt, preserves all exact source dependencies, and adds only a documented exported pure identity in a separately identified observation copy of `Effect.Stream`. It must verify the private `WriterChain` parameter and its complete `WriterEnd`/`WriterLink` constructor inventory from actual generated metadata while leaving the type private. `private-gate.json` records its positive 10.24-second source and metadata result, with 3,454,544 bytes of artifacts. The actual defining contract has `Region r` and exactly `WriterEnd`/`WriterLink`; the type remains absent from public exported types. All nine public contracts also passed in this independently identified observation closure. This observation copy is never the candidate API or a replacement for runtime and retention gates.
