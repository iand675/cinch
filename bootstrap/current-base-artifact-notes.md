# Explicit current-source base artifacts

An ordinary bootstrap keeps using its pinned library producer. Reaching that
fixed point does not establish that a changed embedded `Data.String` was linked.
The explicit artifact workflow keeps the restored pins intact and selects a
separate, fully identified base object for stages 2 and 3 and their clients.

`tooling/build_base_artifact.py` builds the current Rust compiler, whose
`EMBEDDED_SOURCES` includes the current base family. A trivial artifact root
contains no implementation override. The producer emits one complete base
object and its actual checked interfaces, and retains every embedded module's
bytes beside the object and runtime archive. Its receipt records compiler,
source, runtime, object, LLVM, root-request and interface identities.

`bootstrap --library-artifact RECEIPT` admits that selection without falling
back to the pinned provider. Admission independently checks current module
bytes and their interface source hashes, the interface dependency graph, object
and LLVM content hashes, runtime identity, target, ABI, word size, byte order,
reclamation and self-describing descriptors. The String constructor units must
be present in the object. An altered receipt, missing payload or incompatible
source family remains a named failure.

The phase-1 String source is the repeat/concatenation implementation from
`32d16d4432b30d4e1b84d06d39aa89b97761aea9`: SHA-256
`54ae8a339b996756323d8f5598287ca3abdfe8ce5e291cd62e71fdf975bd3b0e`,
interface source hash `9736755187434316968`. The independently marked
source-override tests intentionally have a different source hash. Later
packing, byte and search work is outside this source snapshot.

Fresh-base validation requires separate observations:

- The fresh Rust CLI runs all 398 fixture rows, deep repetition/concatenation
  and temporary-owner/alias stress at O0 and O2 without a source override.
- A verified Native frontend consumes only the explicit base interfaces and
  links exactly that object and runtime. Its clients run the same observations
  in normal and poisoned-free modes on an 8 MiB stack. Consumer optimization
  and the base object's own recorded optimization are distinct settings.
- Concrete and generic ABI probes include `Data.List.is-empty` over a list of
  strings. A missing concrete representation is reported explicitly; selecting
  current module sources does not invent an unrequested generic variant.
- The complete compiler fixed point uses `--library-artifact` for stage 2,
  stage 3 and the changed-source rebuild. Verified unchanged seed/stage-1
  artifacts may be reused with their original hashes. Their old embedded base
  is the bootstrap bridge, not evidence for the new String implementation.

This workflow does not expose private backing buffers, change the uniform-word
ABI, merge duplicate definitions or disguise data as addresses. Validation
receipts must state which source and library epoch each gate actually used.
