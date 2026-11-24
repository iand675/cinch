# Standard library (`stdlib`)

Standard library API reference extracted from checked compiler interfaces across **288 modules** (**66 embedded** in the compiler, **222 source-distributed**) and **7,907 indexed declarations**. Published under [`/packages/stdlib/`](/packages/stdlib/) alongside the [package catalogue](/packages/).

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

# How modules are loaded

- **Embedded** modules are compiled into `cinch` and ready to `import` immediately. `Prelude` is implicitly in scope; compiler-installed special forms live in [compiler vocabulary](../reference/20-compiler-vocabulary.md).
- **Ship source** modules check with the compiler today and ship under `seed/prelude/`. Copy a module and its non-embedded closure (listed on each module page) beside your entry file before importing it.

# Module directory

## Prelude

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Prelude](Prelude.md) | Embedded | 1107 | The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library. |

## Algebra

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Algebra.Arithmetic](Algebra.Arithmetic.md) | Embedded | 8 | Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements. |
| [Algebra.Bits](Algebra.Bits.md) | Embedded | 20 | Bit operations, shifts, and the corresponding operator/fixity vocabulary. |
| [Algebra.Group](Algebra.Group.md) | Embedded | 1 | Group-style inverse/subtraction vocabulary over admitted carrier instances. |
| [Algebra.Monoid](Algebra.Monoid.md) | Embedded | 10 | Identity plus combination vocabulary, default folding, and unit laws with explicit checking status. |
| [Algebra.Semigroup](Algebra.Semigroup.md) | Embedded | 4 | Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs. |
| [Algebra.Semiring](Algebra.Semiring.md) | Embedded | 4 | Zero/one and additive/multiplicative structure, with its law vocabulary. |

## Category

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Category.Alternative](Category.Alternative.md) | Embedded | 2 | Empty and choice operations for admitted higher-kinded containers. |
| [Category.Applicative](Category.Applicative.md) | Embedded | 8 | Container application and value lifting for independent structure-preserving composition. |
| [Category.Foldable](Category.Foldable.md) | Embedded | 26 | Generic folds, conversion to lists, size/emptiness, and related operations over container instances. |
| [Category.Functor](Category.Functor.md) | Embedded | 2 | Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism. |
| [Category.Monad](Category.Monad.md) | Embedded | 4 | Container-dependent composition through the admitted higher-kinded class vocabulary. |
| [Category.Traversable](Category.Traversable.md) | Embedded | 6 | Generic effect/container-aware traversal and sequencing while retaining the input shape. |

## Relation

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Relation.Eq](Relation.Eq.md) | Embedded | 2 | Equality/inequality class vocabulary. Instances live with their types. |
| [Relation.Ord](Relation.Ord.md) | Embedded | 6 | Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit. |

## Function

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Function.Combinators](Function.Combinators.md) | Embedded | 13 | Small reusable function operations such as identity, composition, argument manipulation, and application. |

## Crypto

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Crypto.Aes](Crypto.Aes.md) | Ship source | 13 | AES-128/192/256 with AES-NI or ARMv8 instructions after detection and a constant-time, table-free SIMD fallback. |
| [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md) | Ship source | 15 | ChaCha20 and XChaCha20 streams, Poly1305 and the ChaCha20-Poly1305 and XChaCha20-Poly1305 AEADs with vector row kernels. |
| [Crypto.Cipher](Crypto.Cipher.md) | Ship source | 35 | CTR, OFB, CFB, CBC and GCM over any 128-bit block cipher, with constant-time or carry-less-instruction GHASH and constant-time tag checks. |
| [Crypto.ConstantTime](Crypto.ConstantTime.md) | Ship source | 8 | Branch-free scalar comparison and selection, full-input byte equality, conditional copying and in-place XOR with public bounds and atomic length errors. |
| [Crypto.Des](Crypto.Des.md) | Ship source | 13 | DES and Triple DES block ciphers with ECB, CBC and CTR for legacy interoperation, from a generated straight-line round kernel. |
| [Crypto.Ecdsa](Crypto.Ecdsa.md) | Ship source | 9 | Deterministic (RFC 6979) and hedged P-256 ECDSA signatures in raw and DER form with strict verification. |
| [Crypto.Ed25519](Crypto.Ed25519.md) | Ship source | 11 | Deterministic Ed25519 signatures with constant-time signing and strict verification. |
| [Crypto.Field25519](Crypto.Field25519.md) | Ship source | 20 | Constant-time arithmetic modulo 2^255 - 19 with 128-bit column products, shared by X25519 and Ed25519. |
| [Crypto.Hmac](Crypto.Hmac.md) | Ship source | 13 | Prepared HMAC-SHA-256/SHA-512 keys, persistent messages and full-tag verification. |
| [Crypto.Hpke](Crypto.Hpke.md) | Ship source | 32 | Hybrid public key encryption (RFC 9180) over DHKEM X25519 and P-256, HKDF and AES-GCM or ChaCha20-Poly1305, in all four modes. |
| [Crypto.Kdf](Crypto.Kdf.md) | Ship source | 8 | HKDF and PBKDF2 key derivation with checked lengths, binary inputs and HMAC-SHA-256. |
| [Crypto.MlKem](Crypto.MlKem.md) | Ship source | 18 | ML-KEM-768 and ML-KEM-1024 key encapsulation (FIPS 203) with branch-free SIMD lane kernels. |
| [Crypto.P256](Crypto.P256.md) | Ship source | 28 | Constant-time NIST P-256 keys, SEC 1 point encodings, ECDH and scalar/point arithmetic over generated Montgomery kernels. |
| [Crypto.Rc4](Crypto.Rc4.md) | Ship source | 5 | The RC4 stream cipher as a persistent keystream state, for legacy data only. |
| [Crypto.X25519](Crypto.X25519.md) | Ship source | 10 | Constant-time X25519 Diffie-Hellman key agreement with low-order peer rejection. |

## Data

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Data.Array.Edit](Data.Array.Edit.md) | Ship source | 8 | Checked immutable array range edits, predicate deletion, adjacent compaction, reversal and concatenation. |
| [Data.Array.Packed](Data.Array.Packed.md) | Ship source | 13 | Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it. |
| [Data.Array.Search](Data.Array.Search.md) | Ship source | 12 | Immutable array comparison, indexed lookups, binary search and sortedness with explicit callback policies. |
| [Data.Array](Data.Array.md) | Ship source | 9 | Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions. |
| [Data.Ascii](Data.Ascii.md) | Embedded | 2 | ASCII-refined values and checked construction at text boundaries. |
| [Data.BigDecimal](Data.BigDecimal.md) | Ship source | 19 | Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API. |
| [Data.Bool](Data.Bool.md) | Embedded | 12 | Boolean operations and their comparison instances. |
| [Data.Bytes.Build](Data.Bytes.Build.md) | Ship source | 4 | Fresh byte cloning, doubling repetition, concatenation and separator-aware joining. |
| [Data.Bytes.Fields](Data.Bytes.Fields.md) | Ship source | 21 | Nonempty raw-byte field views with Unicode whitespace, ordered scalar predicates, persistent producers and replayable effect-preserving iteration. |
| [Data.Bytes.Search](Data.Bytes.Search.md) | Ship source | 4 | Exact first, last, containment and count queries over raw bytes with bounded SWAR traversals. |
| [Data.Bytes.Split](Data.Bytes.Split.md) | Ship source | 27 | Retained byte splitting, persistent producer traversals, shared pull cursors and line iteration. |
| [Data.Bytes.Utf8](Data.Bytes.Utf8.md) | Ship source | 30 | Raw UTF-8 validation, forward and reverse decoding, scalar counting, fresh repair and atomic encoding. |
| [Data.Bytes.View](Data.Bytes.View.md) | Ship source | 53 | Owner-retaining checked byte views, relative UTF-8 observations, stable compiled substring patterns and bounded search, cut and comparison operations. |
| [Data.Bytes.Word](Data.Bytes.Word.md) | Ship source | 4 | Bounds-proved unaligned little-endian machine-word reads and writes for SIMD and SWAR processing. |
| [Data.Bytes](Data.Bytes.md) | Embedded | 10 | Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations. |
| [Data.ChaCha8](Data.ChaCha8.md) | Ship source | 9 | The ChaCha8 pseudorandom generator: four interleaved eight-round blocks per vector kernel call, byte reads and state encoding. |
| [Data.Char](Data.Char.md) | Embedded | 21 | Unicode scalar values and checked character/code conversions, with range information for safe use. |
| [Data.Columnar](Data.Columnar.md) | Ship source | 8 | Columnar leaves over typed arrays, zipped into nested pairs that share their buffers. A length mismatch is None. |
| [Data.Compare](Data.Compare.md) | Ship source | 10 | Explicit scalar comparison policies, NaN-aware floating orders, and first non-default selection. |
| [Data.Complex](Data.Complex.md) | Ship source | 16 | Complex values and numeric operations using the module's current floating representation. |
| [Data.Compose](Data.Compose.md) | Embedded | 2 | Composition of higher-kinded containers with the admitted generic instances. |
| [Data.Const](Data.Const.md) | Embedded | 3 | A constant container that retains its first parameter while ignoring the mapped value parameter. |
| [Data.CookieJar](Data.CookieJar.md) | Ship source | 7 | An RFC 6265 cookie jar as an immutable value: store the cookies responses set and choose the cookies a request sends. |
| [Data.Core](Data.Core.md) | Embedded | 3 | Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles. |
| [Data.Date](Data.Date.md) | Ship source | 18 | Civil date values, validation, parsing/formatting, and calendar calculations. |
| [Data.Enum](Data.Enum.md) | Embedded | 6 | Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step. |
| [Data.Float](Data.Float.md) | Ship source | 79 | Binary64 mathematics: roundings, roots, exponentials, logarithms, powers, trigonometric, hyperbolic and special functions (erf, Gamma, Bessel) and FMA as branch-free kernels, bit-identical on every target. |
| [Data.FromList](Data.FromList.md) | Embedded | 1 | The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type. |
| [Data.FromMap](Data.FromMap.md) | Embedded | 1 | The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type. |
| [Data.Hash](Data.Hash.md) | Embedded | 1 | Hashing vocabulary and admitted hash implementations for keys. |
| [Data.HashMap](Data.HashMap.md) | Embedded | 16 | Hash maps with explicit key constraints, lookup/update, and map-style combination operations. |
| [Data.Identity](Data.Identity.md) | Embedded | 8 | The identity container and its generic abstraction instances. |
| [Data.Int](Data.Int.md) | Embedded | 11 | Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions. |
| [Data.IpAddr](Data.IpAddr.md) | Ship source | 16 | Validated IP addresses, parsing/rendering, and address-family vocabulary. |
| [Data.Json](Data.Json.md) | Embedded | 1 | The JSON value representation shared by parsers, renderers, codecs, and structured observability. |
| [Data.List](Data.List.md) | Embedded | 42 | Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances. |
| [Data.Machine](Data.Machine.md) | Embedded | 825 | Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit. |
| [Data.Map](Data.Map.md) | Embedded | 16 | Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support. |
| [Data.Matrix](Data.Matrix.md) | Ship source | 17 | Matrix construction, shape checks, indexing, transformation, and arithmetic operations. |
| [Data.Mutable.Array](Data.Mutable.Array.md) | Ship source | 14 | A fixed-length mutable array whose slots always hold values, with copying freeze and thaw against the immutable Array. |
| [Data.Mutable.BTree](Data.Mutable.BTree.md) | Ship source | 26 | An ordered mutable map: a degree-8 B-tree whose nodes are indices into one pooled set of slot arrays, with floor, ceiling, range folds, pop-min/max and a structural verifier. |
| [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md) | Ship source | 19 | An ordered mutable set: the pooled B-tree with a Unit value per member. |
| [Data.Mutable.BitSet](Data.Mutable.BitSet.md) | Ship source | 16 | A dynamic bitset in 64-bit words: test/set/toggle, popcount, next-set-bit scanning, and word-at-a-time union, intersection and difference. |
| [Data.Mutable.Deque](Data.Mutable.Deque.md) | Ship source | 19 | A double-ended queue as a power-of-two ring buffer: amortised O(1) push and pop at both ends and O(1) indexing. |
| [Data.Mutable.HashSet](Data.Mutable.HashSet.md) | Ship source | 14 | A mutable hash set: the HashTable with a Unit value per member. |
| [Data.Mutable.HashTable](Data.Mutable.HashTable.md) | Ship source | 23 | A mutable hash table after SwissTable and F14: 8-slot chunks with 7-bit tags matched eight at a time, a 64-bit finalizer over the Hash digest, and overflow counts that make deletion tombstone-free. |
| [Data.Mutable.Heap](Data.Mutable.Heap.md) | Ship source | 25 | A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list. |
| [Data.Mutable.HeapView](Data.Mutable.HeapView.md) | Ship source | 8 | In-place heap ordering and indexed repairs through explicit borrowed storage callbacks. |
| [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md) | Ship source | 19 | An insertion-ordered hash map: dense entries with a tagged chunk index over them, positional access, O(1) swap-remove and order-keeping shift-remove. |
| [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md) | Ship source | 31 | Stable typed list elements, checked membership and movement, detached value access and bounded self-copy. |
| [Data.Mutable.LruCache](Data.Mutable.LruCache.md) | Ship source | 11 | A bounded least-recently-used cache: a hash table to nodes threaded on a recency list, with O(1) get, put and eviction. |
| [Data.Mutable.Map.Float](Data.Mutable.Map.Float.md) | Ship source | 2 | Explicit IEEE floating key policies without global equality or hashing instances. |
| [Data.Mutable.Map.Optional](Data.Mutable.Map.Optional.md) | Ship source | 12 | Explicit absent-map adapters without hidden null handles. |
| [Data.Mutable.Map](Data.Mutable.Map.md) | Ship source | 25 | A mutable map with captured key policies and deletion-aware live traversal. |
| [Data.Mutable.Ring](Data.Mutable.Ring.md) | Ship source | 19 | Mutable circular sequences with stable cursors, alias-preserving joins and splits, explicit value ownership and bounded arena growth. |
| [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md) | Ship source | 9 | Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements. |
| [Data.Mutable.Var](Data.Mutable.Var.md) | Ship source | 6 | A single mutable cell, written in place; not synchronized (TVar is the shared cell). |
| [Data.Mutable.Vec](Data.Mutable.Vec.md) | Ship source | 30 | A growable vector: amortised O(1) push and pop, O(1) indexing, middle insertion and removal, in-place introsort, and copying freeze/thaw against Array. |
| [Data.Mutable](Data.Mutable.md) | Ship source | 231 | A curated mutable-container facade. Region-indexed operations carry Mut r; callback-backed heaps and stable linked-list elements have their own module imports. |
| [Data.Nat](Data.Nat.md) | Embedded | 16 | Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions. |
| [Data.NonEmpty](Data.NonEmpty.md) | Embedded | 2 | Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate. |
| [Data.NonZero](Data.NonZero.md) | Embedded | 2 | Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors. |
| [Data.Option](Data.Option.md) | Embedded | 10 | Optional values with None and Some, plus combinators for propagating or consuming absence. |
| [Data.Ordering](Data.Ordering.md) | Embedded | 1 | The LT, EQ, and GT comparison result vocabulary and its instances. |
| [Data.Path](Data.Path.md) | Embedded | 11 | Platform path values and composition/conversion operations used by filesystem and process APIs. |
| [Data.Producer](Data.Producer.md) | Embedded | 10 | Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive. |
| [Data.Rand](Data.Rand.md) | Ship source | 26 | Pseudorandom sources as values (PCG and ChaCha8) and their distributions: bounded integers, floats, permutations, ziggurat normal and exponential variates, and Zipf. |
| [Data.Rational](Data.Rational.md) | Ship source | 14 | Rational numbers with explicit denominator/error policy and arithmetic operations. |
| [Data.Result](Data.Result.md) | Embedded | 12 | Success and named failure values, error-aware transformations, partitioning, and sequencing instances. |
| [Data.Search](Data.Search.md) | Ship source | 2 | Indexed binary search with monotone predicates, insertion positions and effectful three-way comparisons. |
| [Data.Set](Data.Set.md) | Embedded | 16 | Ordered sets and their membership, combination, difference, and conversion operations. |
| [Data.Sorted](Data.Sorted.md) | Embedded | 3 | Lists refined by sortedness, with construction and combination that preserve ordering. |
| [Data.String](Data.String.md) | Embedded | 32 | Immutable UTF-8 text with bounded-stack construction and joining, scalar/span decoding, SIMD scalar counting, slicing and comparison. Byte conversions preserve immutability. |
| [Data.StringIo](Data.StringIo.md) | Ship source | 14 | In-memory string-oriented I/O utilities with explicit state and error results. |
| [Data.Sym](Data.Sym.md) | Embedded | 2 | Interned symbols with identity equality, deterministic spelling order, and String conversions. |
| [Data.Target](Data.Target.md) | Embedded | 9 | Target/platform and worker vocabulary used by the compiler-supported runtime and target examples. |
| [Data.Tensor](Data.Tensor.md) | Ship source | 5 | Immutable row-major tensors with checked runtime shapes of any rank. |
| [Data.Time.Arithmetic](Data.Time.Arithmetic.md) | Ship source | 33 | Calendar-aware spans applied to dates, wall-clock times, date-times and instants, with explicit overflow results. |
| [Data.Time.Date](Data.Time.Date.md) | Ship source | 36 | Civil calendar dates with validated construction, weekday and ordinal queries, and day arithmetic. |
| [Data.Time.DateTime](Data.Time.DateTime.md) | Ship source | 31 | Civil date and wall-clock time pairs without a time zone. |
| [Data.Time.Duration](Data.Time.Duration.md) | Ship source | 43 | Exact signed elapsed-time durations with nanosecond precision and checked arithmetic. |
| [Data.Time.Monotonic](Data.Time.Monotonic.md) | Ship source | 9 | Monotonic clock readings for measuring elapsed time; never related to the civil calendar. |
| [Data.Time.Offset](Data.Time.Offset.md) | Ship source | 18 | Fixed offsets from UTC with validated ranges. |
| [Data.Time.Span](Data.Time.Span.md) | Ship source | 77 | Calendar-aware spans that keep each unit (years through nanoseconds), with balancing and rounding. |
| [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md) | Ship source | 22 | Civil wall-clock times of day with nanosecond precision. |
| [Data.Time.Timestamp](Data.Time.Timestamp.md) | Ship source | 24 | Instants on the Unix time line with nanosecond precision and checked arithmetic. |
| [Data.Time.Unit](Data.Time.Unit.md) | Ship source | 22 | Span units, rounding modes and the shared rounding kernel. |
| [Data.Time.Weekday](Data.Time.Weekday.md) | Ship source | 17 | The seven days of the week with ordering and offset arithmetic. |
| [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md) | Ship source | 17 | POSIX TZ rule strings: parsing and evaluating standard/daylight transition rules. |
| [Data.Time.Zone.Types](Data.Time.Zone.Types.md) | Ship source | 19 | Shared vocabulary for time zones: transitions, ambiguity and disambiguation policies. |
| [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md) | Ship source | 2 | Reading TZif time zone files (RFC 9636) into checked transition tables. |
| [Data.Time.Zone](Data.Time.Zone.md) | Ship source | 43 | Time zones: offsets for instants, resolution of civil times, and fixed or rule-based zones. |
| [Data.Time.Zoned.Arithmetic](Data.Time.Zoned.Arithmetic.md) | Ship source | 45 | Arithmetic, differences and rounding on zoned date-times. |
| [Data.Time.Zoned](Data.Time.Zoned.md) | Ship source | 27 | Instants paired with a time zone, with civil field access. |
| [Data.Time](Data.Time.md) | Ship source | 17 | Time values, durations, and conversions with explicit units and admitted ranges. |
| [Data.Unicode.Kernel](Data.Unicode.Kernel.md) | Ship source | 74 | Unicode scalar and numeric classification, simple case mappings, fold cycles and checked locale overrides. |
| [Data.Unicode.Names](Data.Unicode.Names.md) | Ship source | 29 | Named Unicode category, script and property tables with deterministic catalog observations. |
| [Data.Unicode.Range](Data.Unicode.Range.md) | Ship source | 27 | Immutable Unicode range tables with checked construction, exact membership and copied observations. |
| [Data.Unicode.Tables](Data.Unicode.Tables.md) | Ship source | 16 | Generated Unicode property, case and folding data with a pinned version and source provenance. |
| [Data.Unicode](Data.Unicode.md) | Ship source | 141 | Versioned Unicode classification, simple case mappings, fold cycles, named tables and checked custom ranges. |
| [Data.Uuid](Data.Uuid.md) | Ship source | 24 | Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state. |
| [Data.Wide](Data.Wide.md) | Embedded | 244 | Unboxed 128-bit integers with checked, wrapping and saturating arithmetic, bit operations, numeric conversions and explicit overflow policies. |
| [Data.Word](Data.Word.md) | Ship source | 33 | Bit lengths, byte reversal, signed rotations and checked carry/borrow, wide multiply, divide and remainder operations over fixed and native-width words. |

## Text

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Text.Adler32](Text.Adler32.md) | Ship source | 16 | Portable SIMD Adler-32 with bounded machine-word segments, persistent states, network-order digests and checked versioned checkpoints. |
| [Text.Args](Text.Args.md) | Ship source | 11 | Command-line argument processing and its named value/error vocabulary. |
| [Text.Ascii85](Text.Ascii85.md) | Ship source | 22 | ASCII85 text, caller-buffer and incremental codecs with consumed-input progress, checked source offsets, SIMD groups and control-run scanning. |
| [Text.Asn1](Text.Asn1.md) | Ship source | 21 | ASN.1 value trees with a strict DER decoder and canonical DER encoder, including explicit and implicit tags and times. |
| [Text.Base32](Text.Base32.md) | Ship source | 38 | Configurable Base32 byte alphabets and padding, SIMD codecs, prefix/caller-buffer operations and persistent incremental states. |
| [Text.Base64.Encoding](Text.Base64.Encoding.md) | Ship source | 35 | Configurable Base64 byte alphabets, padding and strict trailing bits, SIMD codecs, persistent states and checked caller buffers. |
| [Text.Base64](Text.Base64.md) | Ship source | 6 | Base64 encoding/decoding over bytes with explicit invalid-input results. |
| [Text.Binary.Layout](Text.Binary.Layout.md) | Ship source | 55 | Typed scalar, floating-point, complex and aggregate binary layouts with checked buffers, exact sizes and packed array kernels. |
| [Text.Binary](Text.Binary.md) | Ship source | 15 | Little- and big-endian integer encoding, checked buffer access, and signed/unsigned varints. |
| [Text.Builder](Text.Builder.md) | Ship source | 12 | An efficient text-construction vocabulary for assembling many fragments before rendering. |
| [Text.Bzip2](Text.Bzip2.md) | Ship source | 3 | Decompress concatenated bzip2 streams with table-driven Huffman decoding and verified block and stream CRCs. |
| [Text.Checksum](Text.Checksum.md) | Ship source | 12 | Incremental Adler-32, CRC-32 and CRC-64 checksums with explicit reflected polynomials. |
| [Text.Codec](Text.Codec.md) | Embedded | 6 | Encoding/decoding signatures and codec helpers with explicit failure results. |
| [Text.Cookie](Text.Cookie.md) | Ship source | 29 | HTTP cookies: parse Set-Cookie and Cookie header lines, serialize cookies as Set-Cookie values and validate them. |
| [Text.Crc32](Text.Crc32.md) | Ship source | 21 | Prepared reflected polynomial tables, persistent CRC-32 states and table-checked binary checkpoints. |
| [Text.Crc64](Text.Crc64.md) | Ship source | 25 | Immutable SIMD-built polynomial tables, slicing-by-eight CRC-64, persistent states, network-order digests and table-checked checkpoints. |
| [Text.Csv.Record](Text.Csv.Record.md) | Ship source | 49 | Configurable Unicode-delimited and lossless byte CSV records, checked partial results, positions and incremental parser state. |
| [Text.Csv](Text.Csv.md) | Ship source | 8 | CSV parsing/rendering with explicit field, record, and malformed-input handling. |
| [Text.Digest](Text.Digest.md) | Ship source | 8 | Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol. |
| [Text.FilePath](Text.FilePath.md) | Ship source | 4 | POSIX lexical relative paths, locality tests, filesystem-name conversion and search-list splitting. |
| [Text.Fnv.State](Text.Fnv.State.md) | Ship source | 21 | Persistent FNV states with reset, network-order digests, numeric observations and checked versioned binary checkpoints. |
| [Text.Fnv](Text.Fnv.md) | Ship source | 12 | Incremental 32-, 64- and 128-bit FNV-1 and FNV-1a hashing with explicit wrapping arithmetic. |
| [Text.Gzip](Text.Gzip.md) | Ship source | 5 | Whole-buffer gzip member encoding, checked decompression and concatenated members. |
| [Text.Hex](Text.Hex.md) | Ship source | 14 | SIMD hexadecimal text/byte codecs, prefix-preserving decoding, append and destination operations, and formatted byte dumps. |
| [Text.Html](Text.Html.md) | Ship source | 5 | HTML text/byte escaping, complete HTML5 character references, exact escaped sizing and SIMD/SWAR scanning. |
| [Text.Json.Binary32](Text.Json.Binary32.md) | Ship source | 2 | Exact decimal rounding and shortest finite binary32 JSON coefficients, with preserved signed zero and midpoint comparisons. |
| [Text.Json.Codec](Text.Json.Codec.md) | Ship source | 137 | Typed JSON codecs for records, persistent maps, explicit key capabilities, machine integers, binary buffers and exact binary32/binary64 values. |
| [Text.Json.Dynamic](Text.Json.Dynamic.md) | Ship source | 24 | Owned dynamic JSON trees, separate encoding and decoding descriptors, typed payloads and bounded construction. |
| [Text.Json.Legacy](Text.Json.Legacy.md) | Ship source | 3 | Explicit legacy codec policy and descriptor bridges over owned dynamic values. |
| [Text.Json.Value](Text.Json.Value.md) | Ship source | 137 | Owned JSON documents and navigation tapes, raw number spellings, exact numeric values and input-bearing errors, resumable lexical state and byte-preserving formatting. |
| [Text.Json](Text.Json.md) | Ship source | 31 | JSON parsing and rendering over Data.Json, with explicit errors and builder-based output. |
| [Text.Lex](Text.Lex.md) | Ship source | 24 | A source-distributed lexer toolkit: tokens, spans, and lexical errors for seed/parser clients. |
| [Text.Lzw](Text.Lzw.md) | Ship source | 7 | Variable-width-code LZW compression and decompression in either bit order, as used by GIF, TIFF and PDF. |
| [Text.Mail](Text.Mail.md) | Ship source | 19 | Parse and format RFC 5322 mail addresses, read message header blocks and parse header dates, with SIMD atom scanning. |
| [Text.Md5](Text.Md5.md) | Ship source | 16 | MD5 for legacy interoperability, with persistent states, little-endian digests and checked binary checkpoints. |
| [Text.Mime](Text.Mime.md) | Ship source | 36 | Byte-preserving media types, encoded words, immutable parameter maps and explicit extension registries. |
| [Text.Multipart](Text.Multipart.md) | Ship source | 29 | Multipart wire primitives over arbitrary bytes, exact multi-value headers and explicit delimiter observations. |
| [Text.OptionParser](Text.OptionParser.md) | Ship source | 23 | Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors. |
| [Text.Otlp](Text.Otlp.md) | Ship source | 5 | OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers. |
| [Text.Parse](Text.Parse.md) | Ship source | 52 | A source-distributed parser toolkit over the lexer/scanner vocabulary. |
| [Text.Path](Text.Path.md) | Ship source | 13 | Portable slash-path lexical operations and checked shell patterns with reusable compiled matchers and exact SIMD scans. |
| [Text.Pem](Text.Pem.md) | Ship source | 20 | Owned byte-preserving envelope blocks, persistent headers, exact malformed-input recovery and bounded payload encoding. |
| [Text.Punycode](Text.Punycode.md) | Ship source | 5 | Punycode (RFC 3492) encoding and decoding, and ASCII-compatible host names with xn-- labels. |
| [Text.QuotedPrintable](Text.QuotedPrintable.md) | Ship source | 24 | Quoted-printable byte encoding and decoding with checked line wrapping, escape and malformed-input policies. |
| [Text.Regex](Text.Regex.md) | Ship source | 22 | Regular-expression parsing/matching and replacement under the module's admitted regex language. |
| [Text.Scan](Text.Scan.md) | Ship source | 31 | Scanning primitives over input text and the shared position/error vocabulary. |
| [Text.Sha1](Text.Sha1.md) | Ship source | 16 | SHA-1 for legacy interoperability, with a four-lane SIMD schedule, persistent states and checked binary checkpoints. |
| [Text.Sha256](Text.Sha256.md) | Ship source | 19 | Portable SHA-224/SHA-256 with generated SIMD schedules, persistent states, network-order digests and checked binary checkpoints. |
| [Text.Sha3](Text.Sha3.md) | Ship source | 29 | SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256 over a register-resident Keccak permutation, with persistent sponges, output readers and checkpoints. |
| [Text.Sha512](Text.Sha512.md) | Ship source | 25 | Portable SHA-384, SHA-512, SHA-512/224 and SHA-512/256 with two-lane SIMD schedules, native rotates, persistent states and checked binary checkpoints. |
| [Text.Shellwords](Text.Shellwords.md) | Ship source | 5 | Shell-style word splitting/joining; this is data parsing, not execution of a shell command. |
| [Text.Show](Text.Show.md) | Embedded | 7 | Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself. |
| [Text.Smtp](Text.Smtp.md) | Ship source | 19 | SMTP without a connection: reply state machine, command validation, extensions, dot-stuffing and PLAIN/CRAM-MD5 exchanges. |
| [Text.TabWriter](Text.TabWriter.md) | Ship source | 14 | Elastic tabstops: align tab-separated cells into padded columns, with escapes, HTML-aware widths, right alignment and incremental flushing. |
| [Text.Tar](Text.Tar.md) | Ship source | 43 | Read V7, USTAR, PAX and GNU tape archives and write USTAR or PAX archives, with SIMD header checksums. |
| [Text.Template](Text.Template.md) | Ship source | 23 | Data-driven text templates: pipelines, control actions, definitions, builtins and printf over dynamic values. |
| [Text.TextProto](Text.TextProto.md) | Ship source | 21 | Text-protocol framing: lines, canonical MIME headers, status responses and dot encoding for HTTP, SMTP and mail. |
| [Text.Time.Duration](Text.Time.Duration.md) | Ship source | 35 | ISO 8601 and friendly duration text for spans and exact durations, printing and parsing. |
| [Text.Time.Iso](Text.Time.Iso.md) | Ship source | 43 | ISO 8601 extended format and RFC 3339 printing and parsing for dates, times, date-times and timestamps. |
| [Text.Time.Rfc2822](Text.Time.Rfc2822.md) | Ship source | 8 | RFC 2822/5322 internet-message date-time printing and parsing. |
| [Text.Time.Rfc9557](Text.Time.Rfc9557.md) | Ship source | 34 | RFC 9557 date-time text with bracketed time zone annotations for zoned values. |
| [Text.Time.Scan](Text.Time.Scan.md) | Ship source | 20 | Shared byte cursor, digit readers and digit writers for the time text modules. |
| [Text.Time.Strtime](Text.Time.Strtime.md) | Ship source | 63 | POSIX strftime/strptime-style formatting and parsing over broken-down time fields. |
| [Text.Time.Zoned](Text.Time.Zoned.md) | Ship source | 18 | Offset and zoned-value text functions. |
| [Text.Toml](Text.Toml.md) | Ship source | 3 | The module's supported TOML parsing/rendering surface and explicit unsupported/malformed cases. |
| [Text.Uri](Text.Uri.md) | Ship source | 17 | URI parsing, rendering, components, and validation/error vocabulary. |
| [Text.Url.Codec](Text.Url.Codec.md) | Ship source | 12 | Checked percent codecs for URL components, exact escaped sizing and optional UTF-8 text views. |
| [Text.Url.Error](Text.Url.Error.md) | Ship source | 18 | Structured URL failures with original byte offsets, context, diagnostics and explicit causes. |
| [Text.Url.Ip](Text.Url.Ip.md) | Ship source | 1 | URL host IP-literal and port validation with explicit IPv6 and zone rules. |
| [Text.Url.Model](Text.Url.Model.md) | Ship source | 31 | Immutable URL components, raw encoding hints, rendering and independent copied observations. |
| [Text.Url.Parser](Text.Url.Parser.md) | Ship source | 4 | Checked general and request URL parsing with precise scheme, host, port and escape rules. |
| [Text.Url.Query](Text.Url.Query.md) | Ship source | 16 | Persistent byte-preserving query multimaps with deterministic encoding, checked parsing and copied observations. |
| [Text.Url.Resolve](Text.Url.Resolve.md) | Ship source | 5 | URL reference resolution, lexical path joining and copied persistent URL updates. |
| [Text.Url.Scan](Text.Url.Scan.md) | Ship source | 9 | Private portable SIMD byte scanning shared by URL parsing, queries and path traversal. |
| [Text.Url.Userinfo](Text.Url.Userinfo.md) | Ship source | 6 | Immutable copied URL user information with explicit password presence and checked percent encoding. |
| [Text.Url](Text.Url.md) | Ship source | 92 | Immutable byte-preserving URLs, percent escaping, query multimaps, parsing and reference resolution. |
| [Text.Utf16](Text.Utf16.md) | Ship source | 10 | UTF-16 scalar and surrogate operations, code-unit sequences and explicit-order byte codecs with SWAR ASCII blocks. |
| [Text.Utf8](Text.Utf8.md) | Ship source | 35 | Raw UTF-8 and immutable-text decoding, validation, scalar counts, replacement encoding and checked atomic buffer writes. |
| [Text.Xml](Text.Xml.md) | Ship source | 33 | Decode XML into namespace-aware tokens or element trees and encode tokens with escaping and indentation. |
| [Text.Yaml](Text.Yaml.md) | Ship source | 9 | The module's supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature. |
| [Text.Zip](Text.Zip.md) | Ship source | 29 | Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or deflated archives. |
| [Text.Zlib](Text.Zlib.md) | Ship source | 8 | Compression/decompression operations and named malformed/unsupported-stream results. |

## Effect

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Effect.Adler32](Effect.Adler32.md) | Ship source | 3 | Inspectable scalar checksum sinks, generic byte writers and bounded reader-to-Adler-32 copying with advanced source continuations. |
| [Effect.Ascii85](Effect.Ascii85.md) | Ship source | 6 | ASCII85 encoding writers and decoding byte readers with explicit finalization, bounded read-ahead and sticky stream errors. |
| [Effect.Base32](Effect.Base32.md) | Ship source | 6 | Explicitly finalized Base32 encoding writers and bounded decoding readers with precise errors, retained cursors and sticky failures. |
| [Effect.Base64](Effect.Base64.md) | Ship source | 6 | Explicitly finalized Base64 writers and bounded decoding readers with sticky errors and isolated output chunks. |
| [Effect.Benchmark](Effect.Benchmark.md) | Ship source | 6 | Benchmark case execution and timing/reporting helpers over runtime clocks. |
| [Effect.Binary](Effect.Binary.md) | Ship source | 9 | Explicit fixed and variable binary stream codecs preserving progress and advanced handles after failures. |
| [Effect.Buffered.Core](Effect.Buffered.Core.md) | Ship source | 27 | Byte-stream events with signed progress, simultaneous failures, shared advanced continuations and explicit transfer capabilities. |
| [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md) | Ship source | 4 | Initialized opaque continuation cells whose allocation, observation and replacement require their I/O region. |
| [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md) | Ship source | 3 | Exact bounded byte scans shared by buffered input and token splitters. |
| [Effect.Buffered.Reader](Effect.Buffered.Reader.md) | Ship source | 27 | Buffered byte reading with explicit advanced handles and independently owned observations. |
| [Effect.Buffered.Writer](Effect.Buffered.Writer.md) | Ship source | 31 | Buffered byte output with sticky failures, exact partial progress and independently owned reservations. |
| [Effect.Buffered](Effect.Buffered.md) | Ship source | 6 | Buffered input and output components with explicit independent advanced handles. |
| [Effect.Clock](Effect.Clock.md) | Embedded | 8 | Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs. |
| [Effect.Concurrent.MVar](Effect.Concurrent.MVar.md) | Embedded | 15 | Native single-place shared variables with FIFO handoff, blocking take/put/read/swap, nonblocking operations, and cancellable waits through the task scheduler. |
| [Effect.Concurrent](Effect.Concurrent.md) | Embedded | 3 | Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations. |
| [Effect.Coverage](Effect.Coverage.md) | Ship source | 7 | Coverage collection/reporting utilities with explicit runtime and source metadata. |
| [Effect.Crc32](Effect.Crc32.md) | Ship source | 3 | CRC-32 stream writers and bounded copying into an inspectable checksum state. |
| [Effect.Crc64](Effect.Crc64.md) | Ship source | 3 | Inspectable CRC-64 states, generic byte writers and bounded reader-to-checksum copying with advanced source continuations. |
| [Effect.Crypto](Effect.Crypto.md) | Ship source | 12 | Ed25519, X25519 and AES key generation and random-nonce GCM over the operating system random source. |
| [Effect.Csv](Effect.Csv.md) | Ship source | 27 | Bounded CSV readers and writers with explicit continuation, record counts, position tracking and advanced handles after failures. |
| [Effect.FileUtils](Effect.FileUtils.md) | Ship source | 9 | Convenience filesystem operations built on the lower-level filesystem API. |
| [Effect.Find](Effect.Find.md) | Ship source | 4 | Filesystem walking/search utilities with the module's traversal and error policy. |
| [Effect.Fnv](Effect.Fnv.md) | Ship source | 6 | Inspectable FNV hashing sinks, generic byte-writer composition and bounded reader-to-hash copying with advanced continuations. |
| [Effect.Fs](Effect.Fs.md) | Embedded | 10 | Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors. |
| [Effect.Hex](Effect.Hex.md) | Ship source | 11 | Hexadecimal encoding writers, decoding readers and formatted dump sinks with explicit continuations and sticky errors. |
| [Effect.Http](Effect.Http.md) | Ship source | 20 | A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation. |
| [Effect.Io](Effect.Io.md) | Embedded | 5 | A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration. |
| [Effect.Log](Effect.Log.md) | Ship source | 21 | Structured log operations and records with runtime capability/effect accounting. |
| [Effect.Md5](Effect.Md5.md) | Ship source | 3 | Inspectable MD5 states, generic digest sinks and bounded reader-to-hash copying for legacy formats. |
| [Effect.Mime](Effect.Mime.md) | Ship source | 20 | Encoded-word and header decoding with explicit charset reader capabilities, typed errors and advanced writer continuations. |
| [Effect.Multipart](Effect.Multipart.md) | Ship source | 79 | Explicit shared multipart sessions with borrowed byte I/O capabilities. |
| [Effect.Net](Effect.Net.md) | Ship source | 8 | Socket operations with explicit handles, address values, errors, and runtime effects. |
| [Effect.Pem](Effect.Pem.md) | Ship source | 3 | Validated envelope output through explicit advanced writer continuations, bounded chunks and linear short-write draining. |
| [Effect.Process](Effect.Process.md) | Embedded | 15 | One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process's arguments. |
| [Effect.Pty](Effect.Pty.md) | Ship source | 13 | Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations. |
| [Effect.QuotedPrintable](Effect.QuotedPrintable.md) | Ship source | 13 | Incremental quoted-printable readers and writers with explicit binary policy, partial progress and sticky errors. |
| [Effect.Rand](Effect.Rand.md) | Ship source | 2 | Pseudorandom sources seeded from operating-system entropy. |
| [Effect.Random](Effect.Random.md) | Ship source | 7 | Runtime entropy/random utilities with explicit effectful access. |
| [Effect.Resolv](Effect.Resolv.md) | Ship source | 6 | Hostname/address resolution through the runtime, with named failure results. |
| [Effect.Scanner.Split](Effect.Scanner.Split.md) | Ship source | 11 | Byte-preserving token splitters with explicit absent, empty and final token states. |
| [Effect.Scanner](Effect.Scanner.md) | Ship source | 12 | Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures. |
| [Effect.Sequencing](Effect.Sequencing.md) | Embedded | 6 | The direct, Result, and Option do providers. One lexical provider/container is selected per definition. |
| [Effect.Sha1](Effect.Sha1.md) | Ship source | 3 | Inspectable SHA-1 states, generic digest sinks and bounded reader-to-hash copying for legacy formats. |
| [Effect.Sha256](Effect.Sha256.md) | Ship source | 3 | Inspectable SHA-224/SHA-256 states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations. |
| [Effect.Sha3](Effect.Sha3.md) | Ship source | 4 | Inspectable SHA-3/SHAKE states, generic digest sinks, bounded reader-to-hash copying and SHAKE output as a generic reader. |
| [Effect.Sha512](Effect.Sha512.md) | Ship source | 3 | Inspectable SHA-384/SHA-512/SHA-512/t states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations. |
| [Effect.Signal](Effect.Signal.md) | Ship source | 5 | Operating-system signal vocabulary and runtime operations with explicit error/effect handling. |
| [Effect.Smtp](Effect.Smtp.md) | Ship source | 15 | An SMTP client over sockets with explicit client state: greeting with HELO fallback, authentication, envelope, data, quit and send-mail. |
| [Effect.Stm.TBQueue](Effect.Stm.TBQueue.md) | Embedded | 11 | A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty. |
| [Effect.Stm.TMVar](Effect.Stm.TMVar.md) | Embedded | 13 | A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions. |
| [Effect.Stm.TQueue](Effect.Stm.TQueue.md) | Embedded | 10 | An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables. |
| [Effect.Stm.TSem](Effect.Stm.TSem.md) | Embedded | 8 | A counting semaphore for transactions: wait takes a permit or retries, signal returns one. |
| [Effect.Stm](Effect.Stm.md) | Embedded | 18 | Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR. |
| [Effect.Stream.Offset](Effect.Stream.Offset.md) | Ship source | 12 | Positional byte writers, signed cursor movement and streaming at a base offset. |
| [Effect.Stream.Section](Effect.Stream.Section.md) | Ship source | 14 | Positional byte sources, immutable bounded sections, signed seeks and sequential reader adapters. |
| [Effect.Stream](Effect.Stream.md) | Ship source | 27 | Composable byte readers and writers with explicit continuation state, memory and descriptor adapters, limits, mirrors, and exact or bulk transfers. |
| [Effect.Test](Effect.Test.md) | Ship source | 24 | A subprocess-based test runner and named test-result/report vocabulary. |
| [Effect.Time.Zone](Effect.Time.Zone.md) | Ship source | 15 | The system time zone database and the current system zone. |
| [Effect.Time](Effect.Time.md) | Ship source | 15 | Typed readers for the system and monotonic clocks. |
| [Effect.Trace](Effect.Trace.md) | Ship source | 6 | Scoped tracing/span vocabulary and context propagation tied to the runtime identity. |
| [Effect.Uuid](Effect.Uuid.md) | Ship source | 5 | Secure UUID generation with explicit serialized version 7 ownership. |

## Image

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Image.Color.Palette](Image.Color.Palette.md) | Ship source | 2 | Immutable factories for the ordered 256-color Plan 9 and 216-color WebSafe palettes. |
| [Image.Color](Image.Color.md) | Ship source | 58 | Color representations, conversion models and immutable palettes with first-nearest lookup. |
| [Image.Draw](Image.Draw.md) | Ship source | 7 | Composite rasters with Src and Over, masks and uniform colors, and dither onto palettes, with a vector Over kernel. |
| [Image.Gif](Image.Gif.md) | Ship source | 24 | Decode and encode GIF images and animations: palettes, transparency, delays, disposal, looping and interlace. |
| [Image.Jpeg](Image.Jpeg.md) | Ship source | 37 | Baseline and progressive JPEG decoding to YCbCr, gray, RGB or CMYK planes, and baseline encoding, with vector IDCT, FDCT, Huffman and colour kernels. |
| [Image.Png](Image.Png.md) | Ship source | 5 | PNG decoding of every color type, depth and interlace method with SIMD scanline reconstruction, and PNG encoding. |
| [Image.Raster](Image.Raster.md) | Ship source | 13 | Gray, gray-alpha, RGB, RGBA and paletted rasters at eight and sixteen bits with pixel access and RGBA conversion. |

## Prim

| Module | Distribution | Exports | Purpose |
| --- | --- | --- | --- |
| [Prim.Sys.Darwin](Prim.Sys.Darwin.md) | Embedded | 19 | The Darwin half of the syscall layer: libSystem calls bound with foreign import ccall (open, read, write, close, lseek, fstat, mkdir, unlink, rename, fsync, getpid), with the -1 result mapped to a named error read from the thread error slot. |
| [Prim.Sys.Linux](Prim.Sys.Linux.md) | Embedded | 18 | The Linux half of the syscall layer: prim bodies that issue syscall with the pinned number per architecture (x86_64 and aarch64), with the kernel negative result mapped to the same named errors as the Darwin module. |
| [Prim.Sys.Portable](Prim.Sys.Portable.md) | Embedded | 10 | The floor under Prim.Sys: the same entry names and the same named errors as the real backends, with every entry point answering Err SysIoError. It is what a target with no backend yet gets, and it keeps the prelude surface from being empty there. |
| [Prim.Sys](Prim.Sys.md) | Embedded | 18 | The syscall layer for the compile target under one name: conditional imports and re-exports select Prim.Sys.Darwin on a Darwin target, Prim.Sys.Linux on a Linux target, and the portable floor everywhere else, so a caller imports one module and never names an operating system. |

# Regeneration and verification

Every module page is generated from checked `.cinchif` interfaces and source comments, with clickable type links, cross-module instance indices, and line-numbered source views. Regenerate after changing `seed/prelude/` with `python3 website/scripts/generate-library-reference.py --compiler /path/to/cinch` and check for drift with `--check`.
