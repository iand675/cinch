# Sealed finite-array payload positivity

This increment extends strict datatype positivity through the actual builtin `Array` payload. It does not introduce subtyping variance, certificates for ordinary containers or an interface-schema field. The source base is the preserved mutation-family checkpoint `fc1838389298772fffc471f3e22b302e2b4aedd9`.

`Array a` is a sealed primitive with finite contiguous owned element storage. Its ordinary interface is immutable: persistent updates return a new array. The trusted private slot API permits in-place updates only inside library-owned storage that is never exposed as an ordinary array alias. These private operations must maintain initialized-read and ownership invariants. A circular-sequence implementation additionally maintains a one-way forwarding DAG; the compiler certificate does not check that library algorithm or authorize reference cycles.

The Rust checker installs a nonserialized `StrictPositivePayload::FiniteOwnedArray` certificate only on its actual builtin type header. Source and imported headers receive no certificate. Positivity consults the registered canonical identity, exact application arity and retained compiler-owned evidence. A conflicting uncertified header for that identity revokes admission. The check never consults a display name or an interface-provided Boolean.

The Native elaborator installs the matching `PayloadCertificate` with primitive kinds. Ordinary kind or alias insertion cannot grant evidence and revokes a competing primitive identity. Context projections preserve the existing certificate. A candidate must be a saturated application of the certified nominal constructor with its full primitive kind. No certificate is serialized or recovered from JSON.

Both checkers continue to refuse unknown containers, including recursive nesting through `List`, and occurrences inside the recursive group's own arguments. Nested arrays and tuples may compose; functions may contain recursive values only in their result. Negative domains remain refused even when two arrows would reverse an ordinary variance sign. The Rust occurrence walk now covers higher-kinded applications and pointer/borrow payloads; an unknown wrapper cannot hide an occurrence from the new array traversal. Frozen Rust f2 baseline accepts direct double-negative fields, higher-kinded applications hiding recursive values inside Array, and boundary pointer payloads inside Array. These were preexisting fail-open edges that require explicit before/after verdict evidence rather than being attributed to the array feature.

Focused targets:

- `cargo test --lib sealed_array_positivity_tests`: actual primitive evidence, negative/double-negative domains, unknown higher-kinded and ordinary containers, exact arity, recursive arguments and forged competing headers.
- `cargo test --test nested_array_positivity`: the identical source corpus through the selected frontend, plus a compiled Native elaborator executing it under normal 32 MiB heap/8 MiB stack and poisoned 256 MiB heap/8 MiB stack profiles.
- Existing `dependent`, `codata`, `arrays`, imported interface/constructor, mutable-inference/value-restriction and effect-row targets retain their applicable boundaries. In particular, the intentionally unsupported nested-List tests remain refusals.
- An actual source-absent interface consumer must preserve builtin-array admission while rejecting ordinary imported carrier payloads and attempts to attach arbitrary certificate metadata.
- Before publication, run actual circular join/split/alias controls using the resulting frontends, the independent corpus, bounded heap profiles and leak checks. A prior frozen frontend is expected to continue rejecting this new source shape.

At the initial static checkpoint, Rust syntax parsing (`rustfmt --emit stdout`), Python source projection and whitespace checks passed. No Cargo, executable positive admission, interface-forgery or runtime-ring gate has been claimed by this note. Parent-owned integration gates and the preserved Native family remain required before publishing a validated compiler increment.
