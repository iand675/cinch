# URL component checkpoint

This checkpoint implements `Text.Url.Codec` and `Text.Url.Error`. The full URL
package remains in progress: parsing, references, user information, query values
and URL rendering are not claimed by this checkpoint. Existing `Text.Uri` is
unchanged.

The component API accepts and returns bytes. Percent-decoded `FF`, NUL and invalid
UTF-8 sequences survive exactly. Only escaping has text convenience functions;
escaped output is ASCII. The seven explicit policies cover complete paths,
segments, hosts, IPv6 zones, user/password components, query components and
fragments. Raw path/fragment hints have separate validation functions. Query
spaces become plus, and only query decoding maps plus to space.

`UrlFailure` exposes a named kind, original component byte offset and copied raw
detail. `UrlError cause` retains a typed cause, operation, copied original input
and explicit temporary/timeout classification, with typed custom formatting.
Diagnostics quote arbitrary bytes with ASCII `\xHH` escapes instead of converting
invalid bytes to text. Diagnostic wording is intentionally independent of the
reference SDK. These pure failures are never temporary or timeouts.

The independent fixture generator uses public APIs in the pinned Go 1.27.4 SDK
at `/tmp/cinch-go-current/go/bin/go`, rather than duplicating Cinch algorithms.
The corpus contains 1,792 all-byte/context encoding cases, 4,988 decoding cases
(including malformed prefixes and host/zone restrictions), and 770 large-input
cases through 65,536 bytes. Large cases independently verify exact output size
and every-byte FNV-1a observations. Native and Rust consumers also mutate caller
inputs and returned diagnostic views to check independent ownership. Host
encoding is not a general inverse of host decoding: encoded ASCII host bytes
other than `%25` are rejected by the host decoder, as required by its policy.

Both frozen frontends compile the actual source modules and pass all cases.
Normal execution uses a 32 MiB heap and 8 MiB stack. Poisoned-free execution uses
256 MiB and 8 MiB; long-input cases run in shallow named groups in independent
processes because the combined native quarantine exceeds 256 MiB. Every process
reports zero leaks. Exact commands, source/executable/compiler hashes and run
results are preserved in `stdlib-url-codec-validation.json`. The Rust consumer
target `lib_url` shares the fixture template and has been added for parent-owned
serial Cargo integration; this lane has not launched Cargo.

Private inline LLVM kernels classify eight independent bytes with SIMD and use
SWAR population counting. Private exact loads retain source owners and never
read an incomplete block. Encoding counts capacity once, skips ordinary spans,
and uses checked copies of complete percent triplets from a 768-byte table.
Decoding validates before allocating output. Primitive argument computations are
bound before calls to preserve their machine ABI widths in both frontends.
Timing, foreign-object compilation and generated-code performance review remain
for the full URL package validation; no throughput claim is made here.

Reproduce the fixture source with:

```sh
GOCACHE=/tmp/cinch-go-surface-cache GOTELEMETRY=off GODEBUG= \
  /tmp/cinch-go-current/go/bin/go run tooling/go_url_vectors.go \
  > compiler/tests/fixtures/url-vectors.json
python3 tooling/generate_url_kernels.py --check
python3 -m unittest discover -s tooling/tests -p test_generate_url_kernels.py
python3 tooling/validate_url.py --suite codec --frontend rust --cpu 2 \
  --artifacts build/url-codec-rust --output build/url-codec-rust.json
python3 tooling/validate_url.py --suite codec --frontend native --cpu 2 \
  --artifacts build/url-codec-native --output build/url-codec-native.json
```
