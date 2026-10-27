# Standard library parity: the Ruby catalog as checklist (P36-P40)

Roadmap plan, 2026-10-27 (user direction: port as much of the Ruby
standard library as sensible, at our discretion).
docs.ruby-lang.org/en/master/standard_library_md.html is the
checklist; this note is the curation. No em dashes.

Ruby's own philosophy fits ours: a curated default surface (its
default gems; our Prelude facade) over independently organized
libraries (its gems; our scheme homes). The curation sorts every
family into PORT, PORT-REDESIGNED, SUBSUMED, or REFUSED.

## Port

- Collections/text utilities: `Set`, `Pathname`, `FileUtils`, `Find`,
  `Tempfile`, `CSV`, `StringIO`, `Base64`, `Shellwords`, string
  scanning/formatting utilities.
- Numeric: `BigDecimal` (arbitrary-precision decimal; money and
  measurement), `Rational`, `Complex`, `Matrix` as concrete types with
  explicit operations (the P2 discipline: types and operations, no
  numeric tower).
- Serialization breadth: `JSON` (P31), `CSV`, TOML, and YAML as the
  SAFE DATA SUBSET (below).
- OS/process: `Open3`/`PTY` shapes under the P34 `Process`
  capability; `Signal` handling reduced to the safe subset.
- Time: civil `Date`/`Time`/`DateTime` (calendars, formatting,
  parsing) atop the P34 `Clock` capability.
- Networking: `Net::HTTP`, `Socket`, `URI`, `IPAddr`, `Resolv`/DNS;
  also the transport P32's OTel export needs.
- Crypto/digest: `Digest` (SHA-256 first; P31 content identity needs
  it), `SecureRandom` (platform entropy), `HMAC`.
- Tooling: `OptionParser` (P31), `Benchmark`, `Coverage`, `Minitest`
  as the design reference for the P34 Cinch test runner.

## Port, redesigned on our principles

- **Regular expressions (P36)**: Ruby's backtracking engine is also
  its ReDoS surface. Cinch ships RE2-style automata matching with
  guaranteed linear time in the input: a safe language whose regexes
  cannot blow up is a feature the safety story demands. Unicode-aware
  over the UTF-8 machinery (the SIMD decoders exist).
- **YAML and the codec family (P37)**: YAML loads are the classic
  arbitrary-object-deserialization CVE. Port as the safe data subset
  (scalars, sequences, maps, anchors without object construction)
  through `Text.Codec`, one family with JSON/CSV/TOML (A6-aligned).
  `Marshal` is REFUSED outright: arbitrary heap deserialization
  contradicts the language's safety story; binary codecs go through
  `Text.Codec` with explicit schemas instead.

## Subsumed by the language (document the mapping; do not port)

- `ERB` (templating) is subsumed by staging and hygienic macros
  (metaprogramming replaces templating, without stringly-typed holes).
- `Struct`/`Data` are records; `Fiber`/`Thread`/`Queue` are thunks,
  codata (`Stream`/`Later`), and the P12 concurrency model; `Logger`
  is P32 observability; `IRB` is the P21 REPL; `Fiddle` is FFI;
  `Singleton`/`Observer` are patterns, not libraries.

## Refused (contradict the language)

- `Marshal` (unsafe deserialization), `ObjectSpace` and
  `Kernel#eval`-style reflection (D71's reflection refusal), `Drb`
  (distributed objects over exactly those mechanisms). Documented as
  refused with reasons so the parity gap is a stated decision.

## New phases

- **P36 Text and regex**: the RE2-style engine (lexers/parsers want
  it too), scanning/formatting utilities, Unicode handling.
- **P37 Serialization breadth**: YAML safe subset, CSV, TOML, and the
  binary-codec story on `Text.Codec`.
- **P38 Numeric and civil time**: BigDecimal/Rational/Complex/Matrix;
  Date/Time/DateTime.
- **P39 Crypto and compression**: Digest/SecureRandom/HMAC; Zlib
  (and friends) as prims/FFI.
- **P40 OS utilities and testing**: the Pathname/FileUtils/Find/
  Tempfile/Open3 sweep; the Cinch test runner (Minitest-shaped);
  Benchmark/Coverage.

Every family lands in the scheme homes with law-shaped tests and the
bootstrap discipline: any family milestone marked for bootstrapping
(P35) gets a runnable artifact, not a claim.
