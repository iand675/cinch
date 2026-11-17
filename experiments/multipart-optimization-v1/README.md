# Isolated multipart allocation optimization

This candidate follows the sealed baseline diagnosis in
`../multipart-promotion-v1/performance-diagnosis.json`. It leaves the reviewed
Effect.Multipart ae47 source unchanged. The Text.Multipart change counts exact
escaped length, allocates one fresh escaped buffer, copies ordinary runs and
emits exceptional bytes directly. Public field/file joining remains unchanged,
so source aliases and caller-visible output independence are preserved. A scalar
leading-byte exception avoids overlapping SIMD rescans; ordinary runs still use
checked complete windows and scalar fringes.

The explicit shared Text.QuotedPrintable copy replaces boxed per-digit/pair
options with one U8-to-U32 hexadecimal classifier and an unpacked invalid
sentinel. It keeps all original input-position guards and checks the decoded
byte range before output. Malformed input follows the original progress/error
branch. Decoder continuation cloning and buffer ownership are unchanged. The
primitive is scalar, branchless inline LLVM; a single byte does not justify
SIMD. Existing whole-window SIMD classification remains intact.

This is a separate source/ABI epoch on qualified Rust330/d91. Source admission,
actual live three-factory ownership, four intended mutation-erasure refusals,
the full1,128multipart O2 observations and exhaustive256-byte/65,536pair actual
primitive fixtures pass. Actual AArch64/wasm classifier objects also compile;
foreign execution is not claimed. Direct shared-codec qualification has begun.
O0/full direct-codec/optimized-code/performance gates remain pending. No
production, current Native admission or package-complete claim is made.

Only exact sources listed in `manifest.json` may be used by the candidate
tools. Shared preludes and existing baseline benchmark inputs are untouched.
Passed functional ELF/LLVM outputs have durable recovery records; all failed
source-check/negative diagnostic artifacts remain plain.

The complete current330/d91 functional qualification is now green:1,128
multipart observations and1,475 direct shared-codec observations atbothO2 and
exact-IR O0. All240 normal/poison8MiB-stack process runs report zero leaks.
Original sources/compiler/runtime/actual foreign objects are verified before
O0 relinking; no new compiler emission is claimed for that backend epoch.
Passed closed functional outputs are verified/archived durably. All407
declarations across the three actual source modules have canonical prose,
zero documentation diagnostics/orphans and no public-library SDK references.

The initial `direct-qp-o2-progress.json` remains a historical134-case checkpoint;
`direct-qp-o2-gate.json` and `direct-qp-o0-gate.json` carry the complete results.
New public pipeline builds/checksum/ownership preflights and allocation-counter
confirmation precede any separate Root-authorized performance measurement.
The sealed ae47/7435/cfc4 baseline and all historical families remain intact.
