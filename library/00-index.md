# Standard library (`cinch/stdlib`)

Standard library API reference extracted from checked `.cinchif` compiler interfaces across **329 modules** and **11,020 indexed declarations**. Published under [`/packages/stdlib/`](/packages/stdlib/) alongside the [package catalogue](/packages/).

<form class="doc-hoogle-box" action="/packages/search/" method="get" role="search">
<label for="lib-index-search">Search by symbol name or type signature</label>
<div class="doc-hoogle-row">
<input id="lib-index-search" name="q" type="search" placeholder="e.g. (a -> b) -> List a -> List b, Result e a -> Option a, or +Data.Map" autocomplete="off" />
<button type="submit">Search API</button>
</div>
<div class="doc-hoogle-chips">
<a href="/packages/search/?q=%28a+-%3E+b%29+-%3E+List+a+-%3E+List+b">(a -&gt; b) -&gt; List a -&gt; List b</a>
<a href="/packages/search/?q=Result+e+a+-%3E+Option+a">Result e a -&gt; Option a</a>
<a href="/packages/search/?q=Ord+a+%3D%3E+List+a+-%3E+List+a">Ord a =&gt; List a -&gt; List a</a>
<a href="/packages/search/?q=Bytes+-%3E+Result">Bytes -&gt; Result</a>
<a href="/packages/search/?q=is%3Aclass">is:class</a>
</div>
</form>

# Using `cinch/stdlib`

- Every module in `cinch/stdlib` resolves directly through the toolchain package manifest (`import Data.Rational`, `import Text.Json`, `import Crypto.Aes`) and checks against its `.cinchif`/`.cpk` interface closure.
- `Prelude` is implicitly in scope unless a package sets `"prelude": "none"` or names a custom prelude in `.cinchpkg`. Compiler-installed special forms live in [compiler vocabulary](../reference/20-compiler-vocabulary.md).
- Inspect the resolved target surface with `cinch pkg info cinch/stdlib`, or link precompiled package objects via `cinch emit-object` and `--package-object` (see [packages, manifests, and the registry](../reference/29-package-management.md)).

# Module directory

## Prelude

| Module | Exports | Purpose |
| --- | --- | --- |
| [Prelude](Prelude.md) | 1151 | The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library. |

## Algebra

| Module | Exports | Purpose |
| --- | --- | --- |
| [Algebra.Arithmetic](Algebra.Arithmetic.md) | 8 | Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements. |
| [Algebra.Bits](Algebra.Bits.md) | 20 | Bit operations, shifts, and the corresponding operator/fixity vocabulary. |
| [Algebra.Group](Algebra.Group.md) | 1 | Group-style inverse/subtraction vocabulary over admitted carrier instances. |
| [Algebra.Monoid](Algebra.Monoid.md) | 10 | Identity plus combination vocabulary, default folding, and unit laws with explicit checking status. |
| [Algebra.Semigroup](Algebra.Semigroup.md) | 4 | Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs. |
| [Algebra.Semiring](Algebra.Semiring.md) | 4 | Zero/one and additive/multiplicative structure, with its law vocabulary. |

## Category

| Module | Exports | Purpose |
| --- | --- | --- |
| [Category.Alternative](Category.Alternative.md) | 2 | Empty and choice operations for admitted higher-kinded containers. |
| [Category.Applicative](Category.Applicative.md) | 8 | Container application and value lifting for independent structure-preserving composition. |
| [Category.Foldable](Category.Foldable.md) | 26 | Generic folds, conversion to lists, size/emptiness, and related operations over container instances. |
| [Category.Functor](Category.Functor.md) | 2 | Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism. |
| [Category.Monad](Category.Monad.md) | 4 | Container-dependent composition through the admitted higher-kinded class vocabulary. |
| [Category.Traversable](Category.Traversable.md) | 6 | Generic effect/container-aware traversal and sequencing while retaining the input shape. |

## Relation

| Module | Exports | Purpose |
| --- | --- | --- |
| [Relation.Eq](Relation.Eq.md) | 2 | Equality/inequality class vocabulary. Instances live with their types. |
| [Relation.Ord](Relation.Ord.md) | 6 | Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit. |

## Function

| Module | Exports | Purpose |
| --- | --- | --- |
| [Function.Combinators](Function.Combinators.md) | 13 | Small reusable function operations such as identity, composition, argument manipulation, and application. |

## Crypto

| Module | Exports | Purpose |
| --- | --- | --- |
| [Crypto.Aes](Crypto.Aes.md) | 13 | AES-128/192/256 with AES-NI or ARMv8 instructions after detection and a constant-time, table-free SIMD fallback. |
| [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md) | 15 | ChaCha20 and XChaCha20 streams, Poly1305 and the ChaCha20-Poly1305 and XChaCha20-Poly1305 AEADs with vector row kernels. |
| [Crypto.Cipher](Crypto.Cipher.md) | 35 | CTR, OFB, CFB, CBC and GCM over any 128-bit block cipher, with constant-time or carry-less-instruction GHASH and constant-time tag checks. |
| [Crypto.ConstantTime](Crypto.ConstantTime.md) | 8 | Branch-free scalar comparison and selection, full-input byte equality, conditional copying and in-place XOR with public bounds and atomic length errors. |
| [Crypto.Des](Crypto.Des.md) | 13 | DES and Triple DES block ciphers with ECB, CBC and CTR for legacy interoperation, from a generated straight-line round kernel. |
| [Crypto.Ecdsa](Crypto.Ecdsa.md) | 9 | Deterministic (RFC 6979) and hedged P-256 ECDSA signatures in raw and DER form with strict verification. |
| [Crypto.Ed25519](Crypto.Ed25519.md) | 11 | Deterministic Ed25519 signatures with constant-time signing and strict verification. |
| [Crypto.Field25519](Crypto.Field25519.md) | 20 | Constant-time arithmetic modulo 2^255 - 19 with 128-bit column products, shared by X25519 and Ed25519. |
| [Crypto.Hmac](Crypto.Hmac.md) | 13 | Prepared HMAC-SHA-256/SHA-512 keys, persistent messages and full-tag verification. |
| [Crypto.Hpke](Crypto.Hpke.md) | 32 | Hybrid public key encryption (RFC 9180) over DHKEM X25519 and P-256, HKDF and AES-GCM or ChaCha20-Poly1305, in all four modes. |
| [Crypto.Kdf](Crypto.Kdf.md) | 8 | HKDF and PBKDF2 key derivation with checked lengths, binary inputs and HMAC-SHA-256. |
| [Crypto.MlDsa](Crypto.MlDsa.md) | 20 | FIPS 204 ML-DSA-44, ML-DSA-65, and ML-DSA-87 post-quantum digital signatures: deterministic and hedged signing, verification, and key/signature codecs. |
| [Crypto.MlKem](Crypto.MlKem.md) | 18 | ML-KEM-768 and ML-KEM-1024 key encapsulation (FIPS 203) with branch-free SIMD lane kernels. |
| [Crypto.P256](Crypto.P256.md) | 28 | Constant-time NIST P-256 keys, SEC 1 point encodings, ECDH and scalar/point arithmetic over generated Montgomery kernels. |
| [Crypto.Pkix](Crypto.Pkix.md) | 106 | PKIX distinguished names, OIDs, RFC 4514 formatting, algorithm identifiers, X.509 extensions, times and certificate revocation lists (CRLs). |
| [Crypto.Rc4](Crypto.Rc4.md) | 5 | The RC4 stream cipher as a persistent keystream state, for legacy data only. |
| [Crypto.Rsa](Crypto.Rsa.md) | 44 | RSA PKCS#1 v1.5 and PSS signing/verification, OAEP and PKCS#1 v1.5 encryption/decryption, PKCS#1 DER key codecs and seeded key generation. |
| [Crypto.Tls](Crypto.Tls.md) | 88 | Pure sans-I/O RFC 8446 TLS 1.3 client and server handshake state machine, key schedule, record AEAD framing, ALPN/SNI and X.509 verification. |
| [Crypto.X25519](Crypto.X25519.md) | 10 | Constant-time X25519 Diffie-Hellman key agreement with low-order peer rejection. |
| [Crypto.X509](Crypto.X509.md) | 133 | X.509 v3 certificate parsing, creation and chain verification, RFC 6125 hostname and IP SAN checks, CSRs, and PKIX/PKCS#8/SEC1/PKCS#1 key codecs. |

## Data

| Module | Exports | Purpose |
| --- | --- | --- |
| [Data.Array.Edit](Data.Array.Edit.md) | 8 | Checked immutable array range edits, predicate deletion, adjacent compaction, reversal and concatenation. |
| [Data.Array.Packed](Data.Array.Packed.md) | 13 | Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it. |
| [Data.Array.Search](Data.Array.Search.md) | 12 | Immutable array comparison, indexed lookups, binary search and sortedness with explicit callback policies. |
| [Data.Array](Data.Array.md) | 9 | Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions. |
| [Data.Ascii](Data.Ascii.md) | 2 | ASCII-refined values and checked construction at text boundaries. |
| [Data.BigDecimal](Data.BigDecimal.md) | 19 | Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API. |
| [Data.Bool](Data.Bool.md) | 12 | Boolean operations and their comparison instances. |
| [Data.Bytes.Build](Data.Bytes.Build.md) | 4 | Fresh byte cloning, doubling repetition, concatenation and separator-aware joining. |
| [Data.Bytes.Fields](Data.Bytes.Fields.md) | 21 | Nonempty raw-byte field views with Unicode whitespace, ordered scalar predicates, persistent producers and replayable effect-preserving iteration. |
| [Data.Bytes.Search](Data.Bytes.Search.md) | 4 | Exact first, last, containment and count queries over raw bytes with bounded SWAR traversals. |
| [Data.Bytes.Split](Data.Bytes.Split.md) | 27 | Retained byte splitting, persistent producer traversals, shared pull cursors and line iteration. |
| [Data.Bytes.Utf8](Data.Bytes.Utf8.md) | 30 | Raw UTF-8 validation, forward and reverse decoding, scalar counting, fresh repair and atomic encoding. |
| [Data.Bytes.View](Data.Bytes.View.md) | 53 | Owner-retaining checked byte views, relative UTF-8 observations, stable compiled substring patterns and bounded search, cut and comparison operations. |
| [Data.Bytes.Word](Data.Bytes.Word.md) | 4 | Bounds-proved unaligned little-endian machine-word reads and writes for SIMD and SWAR processing. |
| [Data.Bytes](Data.Bytes.md) | 27 | Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations. |
| [Data.ChaCha8](Data.ChaCha8.md) | 9 | The ChaCha8 pseudorandom generator: four interleaved eight-round blocks per vector kernel call, byte reads and state encoding. |
| [Data.Char](Data.Char.md) | 21 | Unicode scalar values and checked character/code conversions, with range information for safe use. |
| [Data.Columnar](Data.Columnar.md) | 8 | Columnar leaves over typed arrays, zipped into nested pairs that share their buffers. A length mismatch is None. |
| [Data.Compare](Data.Compare.md) | 10 | Explicit scalar comparison policies, NaN-aware floating orders, and first non-default selection. |
| [Data.Complex](Data.Complex.md) | 42 | Binary64 complex arithmetic, overflow-safe division, polar conversions, and full elementary, trigonometric, hyperbolic, and inverse functions over Data.Float. |
| [Data.Compose](Data.Compose.md) | 2 | Composition of higher-kinded containers with the admitted generic instances. |
| [Data.Const](Data.Const.md) | 3 | A constant container that retains its first parameter while ignoring the mapped value parameter. |
| [Data.CookieJar](Data.CookieJar.md) | 7 | An RFC 6265 cookie jar as an immutable value: store the cookies responses set and choose the cookies a request sends. |
| [Data.Core](Data.Core.md) | 3 | Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles. |
| [Data.Date](Data.Date.md) | 18 | Civil date values, validation, parsing/formatting, and calendar calculations. |
| [Data.Elf](Data.Elf.md) | 131 | Bounds-checked ELF32 and ELF64 (little/big-endian) parser for headers, segments, sections, symbols, relocations, dynamic tables and zlib-compressed sections. |
| [Data.Enum](Data.Enum.md) | 6 | Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step. |
| [Data.Finite](Data.Finite.md) | 7 | Exact finite domain enumeration for algebraic data, records, and compile-time state-machine action parameters. |
| [Data.Float](Data.Float.md) | 79 | Binary64 mathematics: roundings, roots, exponentials, logarithms, powers, trigonometric, hyperbolic and special functions (erf, Gamma, Bessel) and FMA as branch-free kernels, bit-identical on every target. |
| [Data.FromList](Data.FromList.md) | 1 | The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type. |
| [Data.FromMap](Data.FromMap.md) | 1 | The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type. |
| [Data.Hash](Data.Hash.md) | 1 | Hashing vocabulary and admitted hash implementations for keys. |
| [Data.HashMap](Data.HashMap.md) | 16 | Hash maps with explicit key constraints, lookup/update, and map-style combination operations. |
| [Data.Identity](Data.Identity.md) | 8 | The identity container and its generic abstraction instances. |
| [Data.Int](Data.Int.md) | 11 | Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions. |
| [Data.IpAddr](Data.IpAddr.md) | 16 | Validated IP addresses, parsing/rendering, and address-family vocabulary. |
| [Data.Json](Data.Json.md) | 1 | The JSON value representation shared by parsers, renderers, codecs, and structured observability. |
| [Data.List](Data.List.md) | 43 | Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances. |
| [Data.MachO](Data.MachO.md) | 116 | Bounds-checked Mach-O 32/64 (little/big-endian) and Universal/Fat binary parser for load commands, segments, sections, symbols and dylibs. |
| [Data.Machine](Data.Machine.md) | 851 | Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit. |
| [Data.Map](Data.Map.md) | 16 | Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support. |
| [Data.MapHash](Data.MapHash.md) | 22 | Fast keyed 64-bit non-cryptographic hash with persistent streaming state, one-shot byte/string fast paths and MapHashable typed comparable hashing. |
| [Data.Matrix](Data.Matrix.md) | 17 | Matrix construction, shape checks, indexing, transformation, and arithmetic operations. |
| [Data.Mutable.Array](Data.Mutable.Array.md) | 14 | A fixed-length mutable array whose slots always hold values, with copying freeze and thaw against the immutable Array. |
| [Data.Mutable.BTree](Data.Mutable.BTree.md) | 26 | An ordered mutable map: a degree-8 B-tree whose nodes are indices into one pooled set of slot arrays, with floor, ceiling, range folds, pop-min/max and a structural verifier. |
| [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md) | 19 | An ordered mutable set: the pooled B-tree with a Unit value per member. |
| [Data.Mutable.BitSet](Data.Mutable.BitSet.md) | 16 | A dynamic bitset in 64-bit words: test/set/toggle, popcount, next-set-bit scanning, and word-at-a-time union, intersection and difference. |
| [Data.Mutable.Deque](Data.Mutable.Deque.md) | 19 | A double-ended queue as a power-of-two ring buffer: amortised O(1) push and pop at both ends and O(1) indexing. |
| [Data.Mutable.HashSet](Data.Mutable.HashSet.md) | 14 | A mutable hash set: the HashTable with a Unit value per member. |
| [Data.Mutable.HashTable](Data.Mutable.HashTable.md) | 23 | A mutable hash table after SwissTable and F14: 8-slot chunks with 7-bit tags matched eight at a time, a 64-bit finalizer over the Hash digest, and overflow counts that make deletion tombstone-free. |
| [Data.Mutable.Heap](Data.Mutable.Heap.md) | 25 | A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list. |
| [Data.Mutable.HeapView](Data.Mutable.HeapView.md) | 8 | In-place heap ordering and indexed repairs through explicit borrowed storage callbacks. |
| [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md) | 19 | An insertion-ordered hash map: dense entries with a tagged chunk index over them, positional access, O(1) swap-remove and order-keeping shift-remove. |
| [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md) | 31 | Stable typed list elements, checked membership and movement, detached value access and bounded self-copy. |
| [Data.Mutable.LruCache](Data.Mutable.LruCache.md) | 11 | A bounded least-recently-used cache: a hash table to nodes threaded on a recency list, with O(1) get, put and eviction. |
| [Data.Mutable.Map.Float](Data.Mutable.Map.Float.md) | 2 | Explicit IEEE floating key policies without global equality or hashing instances. |
| [Data.Mutable.Map.Optional](Data.Mutable.Map.Optional.md) | 12 | Explicit absent-map adapters without hidden null handles. |
| [Data.Mutable.Map](Data.Mutable.Map.md) | 25 | A mutable map with captured key policies and deletion-aware live traversal. |
| [Data.Mutable.Ring](Data.Mutable.Ring.md) | 19 | Mutable circular sequences with stable cursors, alias-preserving joins and splits, explicit value ownership and bounded arena growth. |
| [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md) | 9 | Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements. |
| [Data.Mutable.Var](Data.Mutable.Var.md) | 6 | A single mutable cell, written in place; not synchronized (TVar is the shared cell). |
| [Data.Mutable.Vec](Data.Mutable.Vec.md) | 30 | A growable vector: amortised O(1) push and pop, O(1) indexing, middle insertion and removal, in-place introsort, and copying freeze/thaw against Array. |
| [Data.Mutable](Data.Mutable.md) | 231 | A curated mutable-container facade. Region-indexed operations carry Mut r; callback-backed heaps and stable linked-list elements have their own module imports. |
| [Data.Nat](Data.Nat.md) | 16 | Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions. |
| [Data.NetIp](Data.NetIp.md) | 65 | Immutable IPv4, IPv6, 4-in-6 and zoned addresses, AddrPort pairs and CIDR prefixes with RFC 5952 formatting, containment, overlap and binary codecs. |
| [Data.NonEmpty](Data.NonEmpty.md) | 2 | Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate. |
| [Data.NonZero](Data.NonZero.md) | 2 | Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors. |
| [Data.Option](Data.Option.md) | 10 | Optional values with None and Some, plus combinators for propagating or consuming absence. |
| [Data.Ordering](Data.Ordering.md) | 1 | The LT, EQ, and GT comparison result vocabulary and its instances. |
| [Data.Path](Data.Path.md) | 11 | Platform path values and composition/conversion operations used by filesystem and process APIs. |
| [Data.Pe](Data.Pe.md) | 123 | Bounds-checked PE32, PE32+ and COFF binary parser for headers, data directories, sections, COFF symbols, imports and exports. |
| [Data.Producer](Data.Producer.md) | 10 | Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive. |
| [Data.Rand](Data.Rand.md) | 26 | Pseudorandom sources as values (PCG and ChaCha8) and their distributions: bounded integers, floats, permutations, ziggurat normal and exponential variates, and Zipf. |
| [Data.Rational](Data.Rational.md) | 14 | Rational numbers with explicit denominator/error policy and arithmetic operations. |
| [Data.Result](Data.Result.md) | 12 | Success and named failure values, error-aware transformations, partitioning, and sequencing instances. |
| [Data.Search](Data.Search.md) | 2 | Indexed binary search with monotone predicates, insertion positions and effectful three-way comparisons. |
| [Data.Set](Data.Set.md) | 16 | Ordered sets and their membership, combination, difference, and conversion operations. |
| [Data.Sorted](Data.Sorted.md) | 3 | Lists refined by sortedness, with construction and combination that preserve ordering. |
| [Data.Sql](Data.Sql.md) | 77 | Pure typed SQL values, NULLs, named parameters, isolation levels, column metadata, errors and checked row scanners. |
| [Data.String](Data.String.md) | 32 | Immutable UTF-8 text with bounded-stack construction and joining, scalar/span decoding, SIMD scalar counting, slicing and comparison. Byte conversions preserve immutability. |
| [Data.StringIo](Data.StringIo.md) | 14 | In-memory string-oriented I/O utilities with explicit state and error results. |
| [Data.SuffixArray](Data.SuffixArray.md) | 13 | Immutable O(n log n) suffix arrays over raw bytes with binary-search lookup, regex search and checked versioned binary serialization. |
| [Data.Sym](Data.Sym.md) | 2 | Interned symbols with identity equality, deterministic spelling order, and String conversions. |
| [Data.Target](Data.Target.md) | 9 | Target/platform and worker vocabulary used by the compiler-supported runtime and target examples. |
| [Data.Tensor](Data.Tensor.md) | 5 | Immutable row-major tensors with checked runtime shapes of any rank. |
| [Data.Time.Arithmetic](Data.Time.Arithmetic.md) | 33 | Calendar-aware spans applied to dates, wall-clock times, date-times and instants, with explicit overflow results. |
| [Data.Time.Date](Data.Time.Date.md) | 36 | Civil calendar dates with validated construction, weekday and ordinal queries, and day arithmetic. |
| [Data.Time.DateTime](Data.Time.DateTime.md) | 31 | Civil date and wall-clock time pairs without a time zone. |
| [Data.Time.Duration](Data.Time.Duration.md) | 43 | Exact signed elapsed-time durations with nanosecond precision and checked arithmetic. |
| [Data.Time.Monotonic](Data.Time.Monotonic.md) | 9 | Monotonic clock readings for measuring elapsed time; never related to the civil calendar. |
| [Data.Time.Offset](Data.Time.Offset.md) | 18 | Fixed offsets from UTC with validated ranges. |
| [Data.Time.Span](Data.Time.Span.md) | 77 | Calendar-aware spans that keep each unit (years through nanoseconds), with balancing and rounding. |
| [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md) | 22 | Civil wall-clock times of day with nanosecond precision. |
| [Data.Time.Timestamp](Data.Time.Timestamp.md) | 24 | Instants on the Unix time line with nanosecond precision and checked arithmetic. |
| [Data.Time.Unit](Data.Time.Unit.md) | 22 | Span units, rounding modes and the shared rounding kernel. |
| [Data.Time.Weekday](Data.Time.Weekday.md) | 17 | The seven days of the week with ordering and offset arithmetic. |
| [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md) | 17 | POSIX TZ rule strings: parsing and evaluating standard/daylight transition rules. |
| [Data.Time.Zone.Types](Data.Time.Zone.Types.md) | 19 | Shared vocabulary for time zones: transitions, ambiguity and disambiguation policies. |
| [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md) | 2 | Reading TZif time zone files (RFC 9636) into checked transition tables. |
| [Data.Time.Zone](Data.Time.Zone.md) | 43 | Time zones: offsets for instants, resolution of civil times, and fixed or rule-based zones. |
| [Data.Time.Zoned.Arithmetic](Data.Time.Zoned.Arithmetic.md) | 45 | Arithmetic, differences and rounding on zoned date-times. |
| [Data.Time.Zoned](Data.Time.Zoned.md) | 27 | Instants paired with a time zone, with civil field access. |
| [Data.Time](Data.Time.md) | 17 | Time values, durations, and conversions with explicit units and admitted ranges. |
| [Data.Unicode.Kernel](Data.Unicode.Kernel.md) | 74 | Unicode scalar and numeric classification, simple case mappings, fold cycles and checked locale overrides. |
| [Data.Unicode.Names](Data.Unicode.Names.md) | 29 | Named Unicode category, script and property tables with deterministic catalog observations. |
| [Data.Unicode.Range](Data.Unicode.Range.md) | 27 | Immutable Unicode range tables with checked construction, exact membership and copied observations. |
| [Data.Unicode.Tables](Data.Unicode.Tables.md) | 16 | Generated Unicode property, case and folding data with a pinned version and source provenance. |
| [Data.Unicode](Data.Unicode.md) | 141 | Versioned Unicode classification, simple case mappings, fold cycles, named tables and checked custom ranges. |
| [Data.Uuid](Data.Uuid.md) | 24 | Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state. |
| [Data.Wide](Data.Wide.md) | 295 | Unboxed 128-bit integers with checked, wrapping and saturating arithmetic, bit operations, numeric conversions and explicit overflow policies. |
| [Data.Word](Data.Word.md) | 33 | Bit lengths, byte reversal, signed rotations and checked carry/borrow, wide multiply, divide and remainder operations over fixed and native-width words. |

## Text

| Module | Exports | Purpose |
| --- | --- | --- |
| [Text.Adler32](Text.Adler32.md) | 16 | Portable SIMD Adler-32 with bounded machine-word segments, persistent states, network-order digests and checked versioned checkpoints. |
| [Text.Args](Text.Args.md) | 11 | Command-line argument processing and its named value/error vocabulary. |
| [Text.Ascii85](Text.Ascii85.md) | 22 | ASCII85 text, caller-buffer and incremental codecs with consumed-input progress, checked source offsets, SIMD groups and control-run scanning. |
| [Text.Asn1](Text.Asn1.md) | 21 | ASN.1 value trees with a strict DER decoder and canonical DER encoder, including explicit and implicit tags and times. |
| [Text.Base32](Text.Base32.md) | 38 | Configurable Base32 byte alphabets and padding, SIMD codecs, prefix/caller-buffer operations and persistent incremental states. |
| [Text.Base64.Encoding](Text.Base64.Encoding.md) | 35 | Configurable Base64 byte alphabets, padding and strict trailing bits, SIMD codecs, persistent states and checked caller buffers. |
| [Text.Base64](Text.Base64.md) | 6 | Base64 encoding/decoding over bytes with explicit invalid-input results. |
| [Text.Binary.Layout](Text.Binary.Layout.md) | 55 | Typed scalar, floating-point, complex and aggregate binary layouts with checked buffers, exact sizes and packed array kernels. |
| [Text.Binary](Text.Binary.md) | 15 | Little- and big-endian integer encoding, checked buffer access, and signed/unsigned varints. |
| [Text.Builder](Text.Builder.md) | 12 | An efficient text-construction vocabulary for assembling many fragments before rendering. |
| [Text.Bzip2](Text.Bzip2.md) | 3 | Decompress concatenated bzip2 streams with table-driven Huffman decoding and verified block and stream CRCs. |
| [Text.Checksum](Text.Checksum.md) | 12 | Incremental Adler-32, CRC-32 and CRC-64 checksums with explicit reflected polynomials. |
| [Text.Codec](Text.Codec.md) | 6 | Encoding/decoding signatures and codec helpers with explicit failure results. |
| [Text.Cookie](Text.Cookie.md) | 29 | HTTP cookies: parse Set-Cookie and Cookie header lines, serialize cookies as Set-Cookie values and validate them. |
| [Text.Crc32](Text.Crc32.md) | 21 | Prepared reflected polynomial tables, persistent CRC-32 states and table-checked binary checkpoints. |
| [Text.Crc64](Text.Crc64.md) | 25 | Immutable SIMD-built polynomial tables, slicing-by-eight CRC-64, persistent states, network-order digests and table-checked checkpoints. |
| [Text.Csv.Record](Text.Csv.Record.md) | 49 | Configurable Unicode-delimited and lossless byte CSV records, checked partial results, positions and incremental parser state. |
| [Text.Csv](Text.Csv.md) | 8 | CSV parsing/rendering with explicit field, record, and malformed-input handling. |
| [Text.Digest](Text.Digest.md) | 8 | Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol. |
| [Text.FilePath](Text.FilePath.md) | 4 | POSIX lexical relative paths, locality tests, filesystem-name conversion and search-list splitting. |
| [Text.Flate](Text.Flate.md) | 15 | RFC 1951 raw DEFLATE compression (hash-chain LZ77 and Package-Merge dynamic/fixed Huffman) and bounded decompression with preset dictionaries. |
| [Text.Fnv.State](Text.Fnv.State.md) | 21 | Persistent FNV states with reset, network-order digests, numeric observations and checked versioned binary checkpoints. |
| [Text.Fnv](Text.Fnv.md) | 12 | Incremental 32-, 64- and 128-bit FNV-1 and FNV-1a hashing with explicit wrapping arithmetic. |
| [Text.Gob](Text.Gob.md) | 49 | Wire-compatible Go gob binary encoder and decoder for primitives, slices, arrays, maps and structs with stateful type-descriptor reuse. |
| [Text.Gzip](Text.Gzip.md) | 7 | Whole-buffer gzip member encoding, checked decompression and concatenated members. |
| [Text.Hex](Text.Hex.md) | 14 | SIMD hexadecimal text/byte codecs, prefix-preserving decoding, append and destination operations, and formatted byte dumps. |
| [Text.Html](Text.Html.md) | 5 | HTML text/byte escaping, complete HTML5 character references, exact escaped sizing and SIMD/SWAR scanning. |
| [Text.HtmlTemplate](Text.HtmlTemplate.md) | 30 | Context-aware auto-escaping HTML template engine over Text.Template and Text.Html with typed safe content wrappers for HTML, attributes, URLs, JS, CSS and srcset. |
| [Text.Http](Text.Http.md) | 187 | Pure HTTP/1.1 wire parsing/rendering, chunked transfer coding, status texts, canonical headers, forms and Go 1.22 ServeMux pattern routing. |
| [Text.Json.Binary32](Text.Json.Binary32.md) | 2 | Exact decimal rounding and shortest finite binary32 JSON coefficients, with preserved signed zero and midpoint comparisons. |
| [Text.Json.Codec](Text.Json.Codec.md) | 137 | Typed JSON codecs for records, persistent maps, explicit key capabilities, machine integers, binary buffers and exact binary32/binary64 values. |
| [Text.Json.Dynamic](Text.Json.Dynamic.md) | 24 | Owned dynamic JSON trees, separate encoding and decoding descriptors, typed payloads and bounded construction. |
| [Text.Json.Legacy](Text.Json.Legacy.md) | 3 | Explicit legacy codec policy and descriptor bridges over owned dynamic values. |
| [Text.Json.Value](Text.Json.Value.md) | 137 | Owned JSON documents and navigation tapes, raw number spellings, exact numeric values and input-bearing errors, resumable lexical state and byte-preserving formatting. |
| [Text.Json](Text.Json.md) | 31 | JSON parsing and rendering over Data.Json, with explicit errors and builder-based output. |
| [Text.Lex](Text.Lex.md) | 26 | Lexer toolkit: tokens, spans, and lexical errors for grammar and parser clients. |
| [Text.Lzw](Text.Lzw.md) | 7 | Variable-width-code LZW compression and decompression in either bit order, as used by GIF, TIFF and PDF. |
| [Text.Mail](Text.Mail.md) | 19 | Parse and format RFC 5322 mail addresses, read message header blocks and parse header dates, with SIMD atom scanning. |
| [Text.Md5](Text.Md5.md) | 16 | MD5 for legacy interoperability, with persistent states, little-endian digests and checked binary checkpoints. |
| [Text.Mime](Text.Mime.md) | 36 | Byte-preserving media types, encoded words, immutable parameter maps and explicit extension registries. |
| [Text.Multipart](Text.Multipart.md) | 29 | Multipart wire primitives over arbitrary bytes, exact multi-value headers and explicit delimiter observations. |
| [Text.OptionParser](Text.OptionParser.md) | 23 | Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors. |
| [Text.Otlp](Text.Otlp.md) | 5 | OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers. |
| [Text.Parse](Text.Parse.md) | 52 | Parser combinator toolkit over the lexer/scanner vocabulary. |
| [Text.Path](Text.Path.md) | 13 | Portable slash-path lexical operations and checked shell patterns with reusable compiled matchers and exact SIMD scans. |
| [Text.Pem](Text.Pem.md) | 20 | Owned byte-preserving envelope blocks, persistent headers, exact malformed-input recovery and bounded payload encoding. |
| [Text.Punycode](Text.Punycode.md) | 5 | Punycode (RFC 3492) encoding and decoding, and ASCII-compatible host names with xn-- labels. |
| [Text.QuotedPrintable](Text.QuotedPrintable.md) | 24 | Quoted-printable byte encoding and decoding with checked line wrapping, escape and malformed-input policies. |
| [Text.Regex](Text.Regex.md) | 22 | Regular-expression parsing/matching and replacement under the module's admitted regex language. |
| [Text.Rpc](Text.Rpc.md) | 71 | Pure JSON-RPC 1.0/2.0 (single, notification, batch) and length-prefixed gob-RPC wire codecs, envelopes, and standard error codes. |
| [Text.Scan](Text.Scan.md) | 31 | Scanning primitives over input text and the shared position/error vocabulary. |
| [Text.Sha1](Text.Sha1.md) | 16 | SHA-1 for legacy interoperability, with a four-lane SIMD schedule, persistent states and checked binary checkpoints. |
| [Text.Sha256](Text.Sha256.md) | 19 | Portable SHA-224/SHA-256 with generated SIMD schedules, persistent states, network-order digests and checked binary checkpoints. |
| [Text.Sha3](Text.Sha3.md) | 29 | SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256 over a register-resident Keccak permutation, with persistent sponges, output readers and checkpoints. |
| [Text.Sha512](Text.Sha512.md) | 25 | Portable SHA-384, SHA-512, SHA-512/224 and SHA-512/256 with two-lane SIMD schedules, native rotates, persistent states and checked binary checkpoints. |
| [Text.Shellwords](Text.Shellwords.md) | 5 | Shell-style word splitting/joining; this is data parsing, not execution of a shell command. |
| [Text.Show](Text.Show.md) | 7 | Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself. |
| [Text.Smtp](Text.Smtp.md) | 19 | SMTP without a connection: reply state machine, command validation, extensions, dot-stuffing and PLAIN/CRAM-MD5 exchanges. |
| [Text.Strconv](Text.Strconv.md) | 53 | Explicit-base and width-aware integer codecs, exact decimal and hexadecimal float parsing and formatting, complex parsing/formatting and Go string/rune quoting. |
| [Text.TabWriter](Text.TabWriter.md) | 14 | Elastic tabstops: align tab-separated cells into padded columns, with escapes, HTML-aware widths, right alignment and incremental flushing. |
| [Text.Tar](Text.Tar.md) | 43 | Read V7, USTAR, PAX and GNU tape archives and write USTAR or PAX archives, with SIMD header checksums. |
| [Text.Template](Text.Template.md) | 23 | Data-driven text templates: pipelines, control actions, definitions, builtins and printf over dynamic values. |
| [Text.TextProto](Text.TextProto.md) | 21 | Text-protocol framing: lines, canonical MIME headers, status responses and dot encoding for HTTP, SMTP and mail. |
| [Text.Time.Duration](Text.Time.Duration.md) | 35 | ISO 8601 and friendly duration text for spans and exact durations, printing and parsing. |
| [Text.Time.Iso](Text.Time.Iso.md) | 43 | ISO 8601 extended format and RFC 3339 printing and parsing for dates, times, date-times and timestamps. |
| [Text.Time.Rfc2822](Text.Time.Rfc2822.md) | 8 | RFC 2822/5322 internet-message date-time printing and parsing. |
| [Text.Time.Rfc9557](Text.Time.Rfc9557.md) | 34 | RFC 9557 date-time text with bracketed time zone annotations for zoned values. |
| [Text.Time.Scan](Text.Time.Scan.md) | 20 | Shared byte cursor, digit readers and digit writers for the time text modules. |
| [Text.Time.Strtime](Text.Time.Strtime.md) | 63 | POSIX strftime/strptime-style formatting and parsing over broken-down time fields. |
| [Text.Time.Zoned](Text.Time.Zoned.md) | 18 | Offset and zoned-value text functions. |
| [Text.Toml](Text.Toml.md) | 3 | The module's supported TOML parsing/rendering surface and explicit unsupported/malformed cases. |
| [Text.Uri](Text.Uri.md) | 17 | URI parsing, rendering, components, and validation/error vocabulary. |
| [Text.Url.Codec](Text.Url.Codec.md) | 12 | Checked percent codecs for URL components, exact escaped sizing and optional UTF-8 text views. |
| [Text.Url.Error](Text.Url.Error.md) | 18 | Structured URL failures with original byte offsets, context, diagnostics and explicit causes. |
| [Text.Url.Ip](Text.Url.Ip.md) | 1 | URL host IP-literal and port validation with explicit IPv6 and zone rules. |
| [Text.Url.Model](Text.Url.Model.md) | 31 | Immutable URL components, raw encoding hints, rendering and independent copied observations. |
| [Text.Url.Parser](Text.Url.Parser.md) | 4 | Checked general and request URL parsing with precise scheme, host, port and escape rules. |
| [Text.Url.Query](Text.Url.Query.md) | 16 | Persistent byte-preserving query multimaps with deterministic encoding, checked parsing and copied observations. |
| [Text.Url.Resolve](Text.Url.Resolve.md) | 5 | URL reference resolution, lexical path joining and copied persistent URL updates. |
| [Text.Url.Scan](Text.Url.Scan.md) | 9 | Private portable SIMD byte scanning shared by URL parsing, queries and path traversal. |
| [Text.Url.Userinfo](Text.Url.Userinfo.md) | 6 | Immutable copied URL user information with explicit password presence and checked percent encoding. |
| [Text.Url](Text.Url.md) | 92 | Immutable byte-preserving URLs, percent escaping, query multimaps, parsing and reference resolution. |
| [Text.User](Text.User.md) | 27 | Pure Unix /etc/passwd and /etc/group parsers, user/group lookup tables, and supplementary group resolution. |
| [Text.Utf16](Text.Utf16.md) | 10 | UTF-16 scalar and surrogate operations, code-unit sequences and explicit-order byte codecs with SWAR ASCII blocks. |
| [Text.Utf8](Text.Utf8.md) | 35 | Raw UTF-8 and immutable-text decoding, validation, scalar counts, replacement encoding and checked atomic buffer writes. |
| [Text.Xml](Text.Xml.md) | 33 | Decode XML into namespace-aware tokens or element trees and encode tokens with escaping and indentation. |
| [Text.Yaml](Text.Yaml.md) | 9 | The module's supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature. |
| [Text.Zip](Text.Zip.md) | 30 | Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or Text.Flate-compressed deflated archives. |
| [Text.Zlib](Text.Zlib.md) | 12 | RFC 1950 zlib and raw DEFLATE compression (backed by Text.Flate LZ77 + dynamic Huffman) and decompression with named error results. |

## Effect

| Module | Exports | Purpose |
| --- | --- | --- |
| [Effect.Adler32](Effect.Adler32.md) | 3 | Inspectable scalar checksum sinks, generic byte writers and bounded reader-to-Adler-32 copying with advanced source continuations. |
| [Effect.Arena](Effect.Arena.md) | 5 | Region-delimited bump arenas with O(1) fast escape (with-arena, arena-abort) and typed byte-budget boundaries (with-arena-budget, BudgetError). |
| [Effect.Ascii85](Effect.Ascii85.md) | 6 | ASCII85 encoding writers and decoding byte readers with explicit finalization, bounded read-ahead and sticky stream errors. |
| [Effect.Base32](Effect.Base32.md) | 6 | Explicitly finalized Base32 encoding writers and bounded decoding readers with precise errors, retained cursors and sticky failures. |
| [Effect.Base64](Effect.Base64.md) | 6 | Explicitly finalized Base64 writers and bounded decoding readers with sticky errors and isolated output chunks. |
| [Effect.Benchmark](Effect.Benchmark.md) | 6 | Benchmark case execution and timing/reporting helpers over runtime clocks. |
| [Effect.Binary](Effect.Binary.md) | 9 | Explicit fixed and variable binary stream codecs preserving progress and advanced handles after failures. |
| [Effect.Buffered.Core](Effect.Buffered.Core.md) | 27 | Byte-stream events with signed progress, simultaneous failures, shared advanced continuations and explicit transfer capabilities. |
| [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md) | 4 | Initialized opaque continuation cells whose allocation, observation and replacement require their I/O region. |
| [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md) | 3 | Exact bounded byte scans shared by buffered input and token splitters. |
| [Effect.Buffered.Reader](Effect.Buffered.Reader.md) | 27 | Buffered byte reading with explicit advanced handles and independently owned observations. |
| [Effect.Buffered.Writer](Effect.Buffered.Writer.md) | 31 | Buffered byte output with sticky failures, exact partial progress and independently owned reservations. |
| [Effect.Buffered](Effect.Buffered.md) | 6 | Buffered input and output components with explicit independent advanced handles. |
| [Effect.Clock](Effect.Clock.md) | 8 | Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs. |
| [Effect.Concurrent.MVar](Effect.Concurrent.MVar.md) | 15 | Native single-place shared variables with FIFO handoff, blocking take/put/read/swap, nonblocking operations, and cancellable waits through the task scheduler. |
| [Effect.Concurrent](Effect.Concurrent.md) | 3 | Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations. |
| [Effect.Coverage](Effect.Coverage.md) | 7 | Coverage collection/reporting utilities with explicit runtime and source metadata. |
| [Effect.Crc32](Effect.Crc32.md) | 3 | CRC-32 stream writers and bounded copying into an inspectable checksum state. |
| [Effect.Crc64](Effect.Crc64.md) | 3 | Inspectable CRC-64 states, generic byte writers and bounded reader-to-checksum copying with advanced source continuations. |
| [Effect.Crypto](Effect.Crypto.md) | 12 | Ed25519, X25519 and AES key generation and random-nonce GCM over the operating system random source. |
| [Effect.Csv](Effect.Csv.md) | 27 | Bounded CSV readers and writers with explicit continuation, record counts, position tracking and advanced handles after failures. |
| [Effect.Expvar](Effect.Expvar.md) | 46 | Region-scoped live metrics registry (Int, Float, String, Map, Func) with deterministic sorted JSON snapshots and /debug/vars HTTP response adapter. |
| [Effect.FileUtils](Effect.FileUtils.md) | 9 | Convenience filesystem operations built on the lower-level filesystem API. |
| [Effect.Find](Effect.Find.md) | 4 | Filesystem walking/search utilities with the module's traversal and error policy. |
| [Effect.Flate](Effect.Flate.md) | 11 | Incremental raw DEFLATE streaming readers and writers over Effect.Stream with sync flush and preset dictionaries. |
| [Effect.Fnv](Effect.Fnv.md) | 6 | Inspectable FNV hashing sinks, generic byte-writer composition and bounded reader-to-hash copying with advanced continuations. |
| [Effect.Fs](Effect.Fs.md) | 10 | Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors. |
| [Effect.Gob](Effect.Gob.md) | 14 | Streaming Go gob encoders and decoders over Effect.Stream preserving type IDs across multi-message streams. |
| [Effect.Hex](Effect.Hex.md) | 11 | Hexadecimal encoding writers, decoding readers and formatted dump sinks with explicit continuations and sticky errors. |
| [Effect.Http.Server](Effect.Http.Server.md) | 44 | HTTP/1.1 and TLS 1.3 HTTPS listener/socket servers with keep-alive, size limits and ServeMux dispatch, plus a TLS/CookieJar/redirect-aware HTTP client. |
| [Effect.Http.Util](Effect.Http.Util.md) | 21 | HTTP/1.1 wire request/response dumping, strict chunked codecs, hop-by-hop header stripping, and HTTP/HTTPS ReverseProxy forwarding. |
| [Effect.Http](Effect.Http.md) | 23 | Bounded HTTP/1.1 and TLS 1.3 HTTPS client operations with response framing, header helpers, and named errors. |
| [Effect.HttpTest](Effect.HttpTest.md) | 23 | In-memory HTTP ResponseRecorder, request builder and loopback wire/socket testing helpers. |
| [Effect.Io](Effect.Io.md) | 5 | A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration. |
| [Effect.Log](Effect.Log.md) | 21 | Structured log operations and records with runtime capability/effect accounting. |
| [Effect.MapHash](Effect.MapHash.md) | 1 | Operating-system entropy seed generation for Data.MapHash. |
| [Effect.Md5](Effect.Md5.md) | 3 | Inspectable MD5 states, generic digest sinks and bounded reader-to-hash copying for legacy formats. |
| [Effect.Mime](Effect.Mime.md) | 20 | Encoded-word and header decoding with explicit charset reader capabilities, typed errors and advanced writer continuations. |
| [Effect.Multipart](Effect.Multipart.md) | 79 | Explicit shared multipart sessions with borrowed byte I/O capabilities. |
| [Effect.Net](Effect.Net.md) | 15 | TCP client sockets, TCP listeners (listen/accept), UDP send, address values, named errors, and runtime effects. |
| [Effect.Pem](Effect.Pem.md) | 3 | Validated envelope output through explicit advanced writer continuations, bounded chunks and linear short-write draining. |
| [Effect.Process](Effect.Process.md) | 15 | One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process's arguments. |
| [Effect.Pty](Effect.Pty.md) | 13 | Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations. |
| [Effect.QuotedPrintable](Effect.QuotedPrintable.md) | 13 | Incremental quoted-printable readers and writers with explicit binary policy, partial progress and sticky errors. |
| [Effect.Rand](Effect.Rand.md) | 2 | Pseudorandom sources seeded from operating-system entropy. |
| [Effect.Random](Effect.Random.md) | 7 | Runtime entropy/random utilities with explicit effectful access. |
| [Effect.Reader](Effect.Reader.md) | 12 | Zero-overhead lexical reader capabilities (Read and TaggedRead), with ask, asks, local, and type-changing with-reader scoping that preserve purity. |
| [Effect.Resolv](Effect.Resolv.md) | 6 | Hostname/address resolution through the runtime, with named failure results. |
| [Effect.Rpc](Effect.Rpc.md) | 33 | Effectful JSON-RPC and gob-RPC TCP listener servers, clients, and HTTP POST JSON-RPC handler over Effect.Net and Effect.Http.Server. |
| [Effect.Scanner.Split](Effect.Scanner.Split.md) | 11 | Byte-preserving token splitters with explicit absent, empty and final token states. |
| [Effect.Scanner](Effect.Scanner.md) | 12 | Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures. |
| [Effect.Sequencing](Effect.Sequencing.md) | 6 | The direct, Result, and Option do providers. One lexical provider/container is selected per definition. |
| [Effect.Sha1](Effect.Sha1.md) | 3 | Inspectable SHA-1 states, generic digest sinks and bounded reader-to-hash copying for legacy formats. |
| [Effect.Sha256](Effect.Sha256.md) | 3 | Inspectable SHA-224/SHA-256 states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations. |
| [Effect.Sha3](Effect.Sha3.md) | 4 | Inspectable SHA-3/SHAKE states, generic digest sinks, bounded reader-to-hash copying and SHAKE output as a generic reader. |
| [Effect.Sha512](Effect.Sha512.md) | 3 | Inspectable SHA-384/SHA-512/SHA-512/t states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations. |
| [Effect.Signal](Effect.Signal.md) | 5 | Operating-system signal vocabulary and runtime operations with explicit error/effect handling. |
| [Effect.Smtp](Effect.Smtp.md) | 32 | An SMTP client over sockets and TLS 1.3 (implicit SMTPS and RFC 3207 STARTTLS) with explicit client state, HELO fallback, PLAIN/CRAM-MD5 authentication, envelope, data, quit and send-mail. |
| [Effect.Sql](Effect.Sql.md) | 78 | Capability-explicit SQL driver protocol, STM-backed connection pooling, prepared statements, transactions, retries and an in-memory transactional driver. |
| [Effect.Stm.TBQueue](Effect.Stm.TBQueue.md) | 11 | A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty. |
| [Effect.Stm.TMVar](Effect.Stm.TMVar.md) | 13 | A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions. |
| [Effect.Stm.TQueue](Effect.Stm.TQueue.md) | 10 | An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables. |
| [Effect.Stm.TSem](Effect.Stm.TSem.md) | 8 | A counting semaphore for transactions: wait takes a permit or retries, signal returns one. |
| [Effect.Stm](Effect.Stm.md) | 18 | Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR. |
| [Effect.Stream.Offset](Effect.Stream.Offset.md) | 12 | Positional byte writers, signed cursor movement and streaming at a base offset. |
| [Effect.Stream.Section](Effect.Stream.Section.md) | 14 | Positional byte sources, immutable bounded sections, signed seeks and sequential reader adapters. |
| [Effect.Stream](Effect.Stream.md) | 27 | Composable byte readers and writers with explicit continuation state, memory and descriptor adapters, limits, mirrors, and exact or bulk transfers. |
| [Effect.Syslog](Effect.Syslog.md) | 36 | RFC 3164 and RFC 5424 syslog message formatting, facility/severity priorities, Effect.Log sink bridge, and UDP/TCP network writers. |
| [Effect.Test.Fs](Effect.Test.Fs.md) | 33 | Immutable in-memory MapFs with synthesized directories, path validation, globbing, subtrees and filesystem conformance checks. |
| [Effect.Test.Hegel](Effect.Test.Hegel.md) | 47 | Choice-stream property testing and native-speed state-machine simulation with automatic integrated shrinking (Hegel / Hypothesis / Hedgehog style). |
| [Effect.Test.Io](Effect.Test.Io.md) | 13 | Stream fault-injection, logging and conformance-testing adapters over Effect.Stream. |
| [Effect.Test.Quick](Effect.Test.Quick.md) | 23 | Deterministic property-based testing engine with Arbitrary instances over Data.Rand. |
| [Effect.Test](Effect.Test.md) | 24 | A subprocess-based test runner and named test-result/report vocabulary. |
| [Effect.Throw](Effect.Throw.md) | 4 | Typed short-circuiting algebraic error effect (Throw e) with cross-call cooperative bubbling, throw, catch, and try-throw. |
| [Effect.Time.Zone](Effect.Time.Zone.md) | 15 | The system time zone database and the current system zone. |
| [Effect.Time](Effect.Time.md) | 15 | Typed readers for the system and monotonic clocks. |
| [Effect.Tls](Effect.Tls.md) | 12 | Effectful TLS 1.3 client and server connections over Effect.Net sockets with OS entropy and clock-checked certificate verification. |
| [Effect.Trace](Effect.Trace.md) | 6 | Scoped tracing/span vocabulary and context propagation tied to the runtime identity. |
| [Effect.User](Effect.User.md) | 11 | Operating-system user and group lookups (current user, by name/UID/GID, supplementary group IDs) over Prim.Sys and Effect.Process. |
| [Effect.Uuid](Effect.Uuid.md) | 5 | Secure UUID generation with explicit serialized version 7 ownership. |

## Image

| Module | Exports | Purpose |
| --- | --- | --- |
| [Image.Color.Palette](Image.Color.Palette.md) | 2 | Immutable factories for the ordered 256-color Plan 9 and 216-color WebSafe palettes. |
| [Image.Color](Image.Color.md) | 58 | Color representations, conversion models and immutable palettes with first-nearest lookup. |
| [Image.Draw](Image.Draw.md) | 7 | Composite rasters with Src and Over, masks and uniform colors, and dither onto palettes, with a vector Over kernel. |
| [Image.Gif](Image.Gif.md) | 24 | Decode and encode GIF images and animations: palettes, transparency, delays, disposal, looping and interlace. |
| [Image.Jpeg](Image.Jpeg.md) | 37 | Baseline and progressive JPEG decoding to YCbCr, gray, RGB or CMYK planes, and baseline encoding, with vector IDCT, FDCT, Huffman and colour kernels. |
| [Image.Png](Image.Png.md) | 5 | PNG decoding of every color type, depth and interlace method with SIMD scanline reconstruction, and PNG encoding. |
| [Image.Raster](Image.Raster.md) | 13 | Gray, gray-alpha, RGB, RGBA and paletted rasters at eight and sixteen bits with pixel access and RGBA conversion. |

## Prim

| Module | Exports | Purpose |
| --- | --- | --- |
| [Prim.Machine](Prim.Machine.md) | 355 | Leaf inline-LLVM scalar machine primitives (fixed-width wrapping/saturating/overflow/bitwise/shift/rotate/bitscan/comparison/conversion operations and word-size bridges), re-exported through Data.Machine and shared directly by Data.Char, Data.Bytes, and Data.String. |
| [Prim.Sys.Darwin](Prim.Sys.Darwin.md) | 19 | The Darwin half of the syscall layer: libSystem calls bound with foreign import ccall (open, read, write, close, lseek, fstat, mkdir, unlink, rename, fsync, getpid), with the -1 result mapped to a named error read from the thread error slot. |
| [Prim.Sys.Linux](Prim.Sys.Linux.md) | 18 | The Linux half of the syscall layer: prim bodies that issue syscall with the pinned number per architecture (x86_64 and aarch64), with the kernel negative result mapped to the same named errors as the Darwin module. |
| [Prim.Sys.Portable](Prim.Sys.Portable.md) | 10 | The floor under Prim.Sys: the same entry names and the same named errors as the real backends, with every entry point answering Err SysIoError. It is what a target with no backend yet gets, and it keeps the prelude surface from being empty there. |
| [Prim.Sys](Prim.Sys.md) | 18 | The syscall layer for the compile target under one name: conditional imports and re-exports select Prim.Sys.Darwin on a Darwin target, Prim.Sys.Linux on a Linux target, and the portable floor everywhere else, so a caller imports one module and never names an operating system. |
| [Prim.Wide](Prim.Wide.md) | 50 | Leaf inline-LLVM 128-bit SIMD lane primitives and target-guarded hardware AES/CLMUL instructions (AES-NI, ARMv8 AES, PCLMULQDQ, PMULL) with portable fallbacks, re-exported through Data.Wide and shared directly by Data.String. |

# Regeneration and verification

Every module page is generated from checked `.cinchif` interfaces and source comments, with clickable type links, cross-module instance indices, and line-numbered source views. Regenerate after changing `seed/prelude/` with `python3 website/scripts/generate-library-reference.py --compiler /path/to/cinch` and check for drift with `--check`.
