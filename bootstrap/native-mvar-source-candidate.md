# Native MVar source candidate

This separate patch is based on the reviewed source/pin proposal3804. It is
source-only: no new Native compiler, provider, LLVM or runtime execution has
occurred. The qualified330/d91 checker accepts the changed Builtin module and
its dependency closure; the Nominal/ExportType, Resolve and larger
ExportNominal closures also pass.
The complete51-source checks reached separate60-second and180-second
deadlines without a diagnostic or verdict; the full source gate remains open.
The longer observation consumed179.52 sampled CPU seconds, peaked near1.13GiB
RSS, and produced no new full51 interface set. The default check command runs
specialization and optimization before returning, so these observations do not
isolate source elaboration cost. The original
RuntimeIr layout/parser failure and the corrected candidate are preserved
separately. These checks type the compiler source; they do not execute the
candidate MVar checker or emitter. Existing published Rust MVar qualification is not
relabelled as evidence for this candidate. The pin proposal is inherited and
unchanged; no Rust metadata allowlist or runtime source is edited here.

The candidate adds one14-op canonical registry, MVarCell's sealed Type→Type
kind and word representation, full source/imported intrinsic contract checks,
source export support, kernel-effect distinction, structural Copy/Share rules,
and concrete word workers. Raw/object/tagged payloads use existing classes
0/1/2. Each partial closure retains the **actual** parameter classes; runtime
consumption is a separate final-drop rule. Put/swap inputs are consumed even
on failed try/cancel outcomes; borrowed cell handles are dropped after access.

Kernel effect identity is internal `#kernel.MVar`, exported as the canonical
MVar label. Resolve keeps a public datatype named MVar from capturing that row.
An actual source-owned effect with spelling MVar retains its own identity;
canonical runtime signatures force the kernel identity and cannot use that
source effect as substitute. ExportNominal recognizes only the internal
kernel identity, leaving ordinary custom-effect transport unresolved. This
patch does not broaden the general C foreign effect envelope.

MVarCell handles are Copy independently of their payload. Share and transfer
require structural Share for the payload even when a handle is moved; another
copy may remain outside the task. Region provenance is inspected through the
payload. No finite-array positivity certificate is assigned to MVarCell.
Source data/record/opaque/refinement declarations cannot claim the kernel
carrier. Imported foreign rows are held in their own observation constructor,
matched by defining binding identity, canonical scheme, effects and ccall ABI.
No serialized executable body or display-name substitution is used.

The emitted wrappers call only `cinch_mvar_new(value,format,full)` and
`cinch_mvar_access(cell,value,operation,mode,&out)`. Status is separate from
payload; zero remains valid data. Optional paths never load uninitialized out
on failure. Some uses the genuine defining Option family and element scan
class. New runtime declarations deliberately have no memory/purity,
willreturn or norecurse claims.

## Explicitly open boundaries

- I128/U128 MVar operations refuse with `MVar I128/U128 payload requires a
  checked boxed-wide bridge; it is not lowered yet`. The existing Native wide
  apply/closure/Option paths are not silently widened. Completing the public
  wide payload API requires an actual typed bridge and independent tests.
- Runtime ABI10 is only a minimum. Historical ABI10 c4/80bd runtimes lack MVar
  kind18/entries. Exact producer/runtime source/hash admission and missing-entry
  negatives are required; unresolved linkage is a refusal, never a fallback.
- Rust currently emits resolved:null for MVar's row. This patch deliberately
  does not grant that metadata transport. Source-absent tests must use a
  genuinely checked Native-produced contract until a separate Rust exporter
  change is reviewed and tested.
- General custom-effect/open-row transport, packed/generic wide arrays and
  direct-wide columnar or parallel tuple layouts remain separate.

## Prepared validation, not executed

`native-mvar-contract-vectors.json` contains canonical imports for all14 ops,
wrong scheme/arity/carrier/Option/effects/kind controls, carrier redeclarations,
source-owned-effect identity controls, Share-task positives/negatives and wide
named refusals. Optional results need actual Data.Option from the matched
interface context. Public-MVar cases explicitly request the exact published
module as source; this is a source-capability test, not a no-shadow provider
claim. Original producer/source-absent tests must subsequently remove that
source and use genuine exported interfaces and real object units.

`native-mvar-word-runtime.cinch` checks all14 ready-state operations, raw0 and
all-one payloads, Unit as full zero, boxed mathematical values, failed counted
try-put consumption and a read alias surviving the cell's lifetime. Run at
consumer O0/O2 with compiled checked RC, normal32MiB/poison256MiB heaps and
8MiB stack, requiring exact observations and zero leaks. Blocking FIFO,
scheduler/cancellation/handoff races and wide payloads remain independent
mandatory gates from `native-mvar-admission-plan.md`.

Before a host/emitter build, check all modified source modules together using
the current51-source snapshot and qualified330/d91 seed, recording real source
hashes and exact diagnostics. Then validate checked Core signatures, kinds,
effect identities, class/consumed maps, Option tags and runtime call modes.
Separate interface/reexport/source-absent and forged foreign/nominal/effect/
runtime/source/hash controls must pass. A C --wrap trace on the real runtime
entries can independently observe exactly-once operation/mode calls after
ordered effectful arguments; it must call the genuine runtime and must not
replace ownership semantics. Only then consider a complete new-family gate;
a fixed point on an unused subset does not qualify66 public modules.
