# Immutable URL package

`Text.Url` now implements the full reviewed `net/url` capability surface in a
byte-preserving, persistent API. Its module-specific mapping contains all 62
actual declaration records from the pinned Go 1.27.4 SDK, including returned
URL/userinfo/value/error capabilities, direct field access, cloning and binary
serialization. Existing `Text.Uri` remains unchanged.

Primary operations consume and return `Bytes`. Decoded NUL and invalid UTF-8 are
preserved in paths, fragments, user information, hosts and query keys/values.
Opaque bodies and raw queries retain bytes verbatim, including percent signs
that are not escapes. `url-to-text` returns `None` when complete rendered bytes
are not UTF-8. Escaping text helpers are safe because their output is ASCII.
No decoded-data conversion silently substitutes an empty string.

A `UrlParts` bundle supplies every independent URL field. Construction snapshots
byte fields; component/accessor observations return copied buffers. Credentials
and URL/query versions share only private immutable storage. Password presence
is independent of password content. Binary serialization and prefix append
produce fresh bytes; unmarshalling returns a parsed value. Query updates return
new values. An AVL tree bounds sorted-key insertion/lookup/deletion, retains
insertion order within each key and represents present empty lists explicitly.

Parsing covers relative, hierarchical and opaque forms, authority/userinfo,
escaped host and IPv6 zone policies, strict IPv6 including terminal dotted IPv4,
raw path/fragment hints, forced empty queries, request-target grammar and precise
failure context. Resolution removes literal dot segments while retaining repeated
separators and encoded dot bytes. Joining cleans separators, clamps parents at
the root and preserves a final slash. Invalid escaped joining elements retain
the prior URL in the direct method; the checked variant exposes their failure.

The independent generator records public reference API results, rather than
repeating the Cinch algorithms. Final fixtures contain:

- 1,792 all-byte/context escaping rows, 4,988 unescaping rows and 770 large cases
  through 65,536 bytes, with exact capacity and every-byte FNV observations.
- 3,226 parse/request rows, including all raw bytes in path, fragment, authority,
  port, userinfo and scheme positions.
- 534 partial-query, 364 reference-resolution, 104 joining and 23 manual-component
  rows. Each successful URL checks 23 stored-field or derived observations.
- 27 independent query stress cases: sorted distinct keys, repeated values and
  empty fields at 0, 1, 15, 16, 255, 256, 9,999, 10,000 and 10,001 fields.
- Additional mutation/retained-version, exact query-limit, error-offset, arbitrary
  byte/text-policy and exhaustive interval/fringe scan checks.

Both frozen frontends pass the complete component and public capability suites
on the exact final sources. There are 302 zero-leak process runs: one complete
ordinary-memory run per suite/frontend at 32 MiB heap / 8 MiB stack, plus every
bounded poisoned-free group at 256 MiB / 8 MiB. Bounded groups retain complete
coverage while limiting quarantine. Exact compiler/source/executable/fixture
hashes, commands and raw results are in `stdlib-url-validation.json`. All 258
library declarations have canonical documentation without source-language
comparisons. Three kernel-generation tests and generated-suffix freshness pass.

The additional pure two-lane hexadecimal kernel agrees with an independent public-decoder digest for all 65,536 digit pairs.

Private inline LLVM uses SIMD byte grammars and SWAR population counts. Exact
complete-block loads retain backing owners, with exact scalar fringes. Constant
policy counting loops dispatch once per component; ordinary decoding validation
and delimiter/control/scheme/userinfo scans keep machine counters inside their
loops. Resolution copies directly from checked source intervals rather than
allocating intermediate segments. All mutation uses checked byte APIs; no stores
are concealed inside inline assembly. Primitive argument computations are bound
before calls to preserve ABI widths in both frontends.

Intentional API differences are listed in `stdlib-url-capabilities.json`: byte
storage and optional UTF-8 views, persistent updates instead of caller-visible
mutation, typed causes and explicit error classification flags, independent
byte-safe diagnostic wording, explicit inclusive query limits and fixed strict
HTTP/HTTPS colon policy without implicit environment compatibility overrides.
The default query limit is 10,000 raw fields, including empty fields; zero in the
explicit limit API disables it. Raw semicolons discard their setting and take
error priority while valid settings survive. Serialization cannot fail after a
URL value exists, so it has no impossible error result.

`lib_url` provides shared-template Rust consumer, exhaustive pair-domain,
documentation and actual-kernel foreign-object gates for the parent's coordinated
Cargo run. This lane has not launched Cargo. Actual maintained kernels compile
to WebAssembly and AArch64 objects; the Rust target supplies the big-endian gate.
The unsupported big-endian CLI profile is not substituted with a host profile.

Coordinated CPU0 measurements cover all ten public pipelines, three sizes and
three patterns against independent checksums. Constant policy counting improved
64 KiB counts 1.91–1.94×; the adopted two-lane pair decoder improved dense 64 KiB
component decoding 1.82–2.01× in alternating retained-executable comparisons.
Dense downstream pipelines improve too. Remaining dense output costs and an
ordinary-path regression sample are retained and explained in
`benchmarks/stdlib/url-notes.md`, alongside raw reports, identities and codegen
inspection. No mutable inline-IR stores or ABI disguises were introduced. Final
candidate module manifests differ only in the component codec, and all complete
functional suites were rerun before publication.

The parent-owned Rust target remains an integration gate; this evidence does not
claim that the full standard-library initiative is complete.

Reproduce the exact suites with `tooling/validate_url.py --suite codec` and
`--suite url` (and exhaustive `--suite pairs`), selecting `--frontend rust` or `--frontend native`, `--cpu 2`, and
separate artifact/output paths. Regenerate fixtures with
`/tmp/cinch-go-current/go/bin/go run tooling/go_url_vectors.go` under writable
`GOCACHE` and disabled telemetry. `tooling/bench_url.py build` prepares artifacts;
its separate `measure` command requires an explicitly coordinated timing CPU.
