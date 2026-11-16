# Fresh imported constructor and join gate

This gate complements the earlier source-override receipts. It imports
`Data.String` and `Data.Bytes` from the admitted combined provider, with no
beside-consumer source files for either module. The only source-distributed
dependencies are the fixture's hexadecimal and fixed-word helpers.

The raw Native compiler is the managed-code snapshot with SHA-256
`58676dec9c2d2f6c4f5307a7106e1b94b6806ede88113f46d665d7e55955ad8f`.
The combined artifact was produced by Rust compiler `96c4f542…`; its matched
runtime is `80bd5b54…` and its library object is `46a7e57f…`.
The admitted interfaces and objects are supplied explicitly. This test does
not use the frozen wrapper's older default provider.

`validate_string_combined_native.py` requires the imported interface
`source_hash` values to match independently computed FNV-1a hashes of the
provider's exact source files: `416560347689830784` for String and
`12579928885703464370` for Bytes. It also checks the object configuration's
SHA-256 and records the provider, producer, runtime, manifest, consumer, IR
and executable identities in the result JSON.

Each corpus is compiled once with checked RC. Each emitted client IR is
then explicitly linked at `-O0` and `-O2` against the same prebuilt provider
object and matched runtime. These flags describe the client links; they do
not rebuild the provider object. Every independent fixture runs in a separate
normal 32 MiB and poison 256 MiB process, both with an 8 MiB program stack and
leak checking. The compiler main thread has a separate 64 MiB stack allowance.
No limits are raised to accommodate cumulative quarantine.

The requested surface is 184 scalar-packing vectors, 92 typed byte-constructor
vectors and 450 immutable clone/join vectors. Two deep construction regressions
and repeated owner/alias pipelines supplement those independent outputs.
Exact commands, memory observations and output identities belong to
`bootstrap/stdlib-string-combined-native-construction-results.json`.

Reproduction (after sourcing the tool environment and applying the documented
single-job/thread and compiler-stack settings):

```sh
taskset -c 0 python3 tooling/validate_string_combined_native.py \
  --compiler /workspace/dasl-stdlib-mutable-family/build/mutable-stage3-frozen/cinch \
  --interfaces /workspace/dasl-stdlib-string-combined/build/combined-client-config/interfaces.json \
  --objects /workspace/dasl-stdlib-string-combined/build/combined-client-config/objects.json \
  --provider /workspace/dasl-stdlib-string-combined/build/combined-base-artifact-v1 \
  --artifacts build/string-combined-native \
  --output bootstrap/stdlib-string-combined-native-construction-results.json
```

This is a focused fresh-provider gate. The parent owns the complete fixed-point,
interface-only, semantic and integration test admission.
