"""Generate real Cinch stdlib documentation from Rust-checked interfaces.

python3 scripts/generate-library-reference.py --compiler /path/to/cinch
python3 scripts/generate-library-reference.py --check

The generator uses a temporary project and private package store, resolving
every module through the toolchain's `cinch/stdlib` manifest (`seed/prelude/.cinchpkg`).
No repository compiler sources or caches are modified.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import subprocess
import tempfile
import shutil
import sys

ROOT = Path(__file__).resolve().parents[2]
PRELUDE = ROOT / 'seed/prelude'
OUTPUT = ROOT / 'library'
MANIFEST = ROOT / 'website/src/data/library-sources.json'
DESCRIPTIONS = {
'Data.Time.Arithmetic':'Calendar-aware spans applied to dates, wall-clock times, date-times and instants, with explicit overflow results.',
'Data.Time.Date':'Civil calendar dates with validated construction, weekday and ordinal queries, and day arithmetic.',
'Data.Time.DateTime':'Civil date and wall-clock time pairs without a time zone.',
'Data.Time.Duration':'Exact signed elapsed-time durations with nanosecond precision and checked arithmetic.',
'Data.Time.Monotonic':'Monotonic clock readings for measuring elapsed time; never related to the civil calendar.',
'Data.Time.Offset':'Fixed offsets from UTC with validated ranges.',
'Data.Time.Span':'Calendar-aware spans that keep each unit (years through nanoseconds), with balancing and rounding.',
'Data.Time.TimeOfDay':'Civil wall-clock times of day with nanosecond precision.',
'Data.Time.Timestamp':'Instants on the Unix time line with nanosecond precision and checked arithmetic.',
'Data.Time.Unit':'Span units, rounding modes and the shared rounding kernel.',
'Data.Time.Weekday':'The seven days of the week with ordering and offset arithmetic.',
'Data.Time.Zone.Posix':'POSIX TZ rule strings: parsing and evaluating standard/daylight transition rules.',
'Data.Time.Zone.Types':'Shared vocabulary for time zones: transitions, ambiguity and disambiguation policies.',
'Data.Time.Zone.Tzif':'Reading TZif time zone files (RFC 9636) into checked transition tables.',
'Data.Time.Zone':'Time zones: offsets for instants, resolution of civil times, and fixed or rule-based zones.',
'Data.Time.Zoned.Arithmetic':'Arithmetic, differences and rounding on zoned date-times.',
'Data.Time.Zoned':'Instants paired with a time zone, with civil field access.',
'Text.Time.Duration':'ISO 8601 and friendly duration text for spans and exact durations, printing and parsing.',
'Text.Time.Iso':'ISO 8601 extended format and RFC 3339 printing and parsing for dates, times, date-times and timestamps.',
'Text.Time.Rfc2822':'RFC 2822/5322 internet-message date-time printing and parsing.',
'Text.Time.Rfc9557':'RFC 9557 date-time text with bracketed time zone annotations for zoned values.',
'Text.Time.Scan':'Shared byte cursor, digit readers and digit writers for the time text modules.',
'Text.Time.Strtime':'POSIX strftime/strptime-style formatting and parsing over broken-down time fields.',
'Text.Time.Zoned':'Offset and zoned-value text functions.',
'Effect.Time.Zone':'The system time zone database and the current system zone.',
'Effect.Time':'Typed readers for the system and monotonic clocks.',
'Crypto.Hmac':'Prepared HMAC-SHA-256/SHA-512 keys, persistent messages and full-tag verification.',
'Crypto.Aes':'AES-128/192/256 with AES-NI or ARMv8 instructions after detection and a constant-time, table-free SIMD fallback.',
'Crypto.Cipher':'CTR, OFB, CFB, CBC and GCM over any 128-bit block cipher, with constant-time or carry-less-instruction GHASH and constant-time tag checks.',
'Crypto.Field25519':'Constant-time arithmetic modulo 2^255 - 19 with 128-bit column products, shared by X25519 and Ed25519.',
'Crypto.X25519':'Constant-time X25519 Diffie-Hellman key agreement with low-order peer rejection.',
'Crypto.Ed25519':'Deterministic Ed25519 signatures with constant-time signing and strict verification.',
'Crypto.Kdf':'HKDF and PBKDF2 key derivation with checked lengths, binary inputs and HMAC-SHA-256.',
'Image.Png':'PNG decoding of every color type, depth and interlace method with SIMD scanline reconstruction, and PNG encoding.',
'Image.Raster':'Gray, gray-alpha, RGB, RGBA and paletted rasters at eight and sixteen bits with pixel access and RGBA conversion.',
'Image.Color':'Color representations, conversion models and immutable palettes with first-nearest lookup.',
'Image.Color.Palette':'Immutable factories for the ordered 256-color Plan 9 and 216-color WebSafe palettes.',
'Crypto.ConstantTime':'Branch-free scalar comparison and selection, full-input byte equality, conditional copying and in-place XOR with public bounds and atomic length errors.',
'Prelude':'The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library.',
'Data.Core':'Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles.',
'Data.List':'Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances.',
'Data.Option':'Optional values with None and Some, plus combinators for propagating or consuming absence.',
'Data.Result':'Success and named failure values, error-aware transformations, partitioning, and sequencing instances.',
'Data.Enum':'Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step.',
'Data.Nat':'Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions.',
'Data.Int':'Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions.',
'Data.Machine':'Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit.',
'Data.Wide':'Signed and unsigned 128-bit scalars, with explicit checked, wrapping, and saturating arithmetic, bit operations, comparisons, and numeric conversions.',
'Data.Bool':'Boolean operations and their comparison instances.',
'Data.Ordering':'The LT, EQ, and GT comparison result vocabulary and its instances.',
'Data.Char':'Unicode scalar values and checked character/code conversions, with range information for safe use.',
'Data.Compare':'Explicit scalar comparison policies, NaN-aware floating orders, and first non-default selection.',
'Data.Unicode':'Versioned Unicode classification, simple case mappings, fold cycles, named tables and checked custom ranges.',
'Data.Unicode.Kernel':'Unicode scalar and numeric classification, simple case mappings, fold cycles and checked locale overrides.',
'Data.Unicode.Tables':'Generated Unicode property, case and folding data with a pinned version and source provenance.',
'Data.Unicode.Range':'Immutable Unicode range tables with checked construction, exact membership and copied observations.',
'Data.Unicode.Names':'Named Unicode category, script and property tables with deterministic catalog observations.',
'Data.String':'Immutable UTF-8 text with bounded-stack construction and joining, scalar/span decoding, SIMD scalar counting, slicing and comparison. Byte conversions preserve immutability.',
'Text.Utf8':'Raw UTF-8 and immutable-text decoding, validation, scalar counts, replacement encoding and checked atomic buffer writes.',
'Data.Bytes':'Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations.',
'Data.Bytes.Fields':'Nonempty raw-byte field views with Unicode whitespace, ordered scalar predicates, persistent producers and replayable effect-preserving iteration.',
'Data.Array.Edit':'Checked immutable array range edits, predicate deletion, adjacent compaction, reversal and concatenation.',
'Data.Array.Search':'Immutable array comparison, indexed lookups, binary search and sortedness with explicit callback policies.',
'Data.Array.Packed':'Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it.',
'Data.Array':'Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions.',
'Data.Columnar':'Columnar leaves over typed arrays, zipped into nested pairs that share their buffers. A length mismatch is None.',
'Data.Tensor':'Immutable row-major tensors with checked runtime shapes of any rank.',
'Data.Map':'Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support.',
'Data.Set':'Ordered sets and their membership, combination, difference, and conversion operations.',
'Data.Hash':'Hashing vocabulary and admitted hash implementations for keys.',
'Data.HashMap':'Hash maps with explicit key constraints, lookup/update, and map-style combination operations.',
'Data.Mutable':'A curated mutable-container facade. Region-indexed operations carry Mut r; callback-backed heaps and stable linked-list elements have their own module imports.',
'Data.Mutable.Vec':'A growable vector: amortised O(1) push and pop, O(1) indexing, middle insertion and removal, in-place introsort, and copying freeze/thaw against Array.',
'Data.Mutable.HashTable':'A mutable hash table after SwissTable and F14: 8-slot chunks with 7-bit tags matched eight at a time, a 64-bit finalizer over the Hash digest, and overflow counts that make deletion tombstone-free.',
'Data.Mutable.HashSet':'A mutable hash set: the HashTable with a Unit value per member.',
'Data.Mutable.Deque':'A double-ended queue as a power-of-two ring buffer: amortised O(1) push and pop at both ends and O(1) indexing.',
'Data.Mutable.Heap':'A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list.',
'Data.Mutable.Array':'A fixed-length mutable array whose slots always hold values, with copying freeze and thaw against the immutable Array.',
'Data.Mutable.Var':'A single mutable cell, written in place; not synchronized (TVar is the shared cell).',
'Data.Mutable.BTree':'An ordered mutable map: a degree-8 B-tree whose nodes are indices into one pooled set of slot arrays, with floor, ceiling, range folds, pop-min/max and a structural verifier.',
'Data.Mutable.BTreeSet':'An ordered mutable set: the pooled B-tree with a Unit value per member.',
'Data.Mutable.IndexMap':'An insertion-ordered hash map: dense entries with a tagged chunk index over them, positional access, O(1) swap-remove and order-keeping shift-remove.',
'Data.Mutable.LruCache':'A bounded least-recently-used cache: a hash table to nodes threaded on a recency list, with O(1) get, put and eviction.',
'Data.Mutable.BitSet':'A dynamic bitset in 64-bit words: test/set/toggle, popcount, next-set-bit scanning, and word-at-a-time union, intersection and difference.',
'Data.Mutable.UnionFind':'Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements.',
'Data.Sym':'Interned symbols with identity equality, deterministic spelling order, and String conversions.',
'Data.NonEmpty':'Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate.',
'Data.NonZero':'Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors.',
'Data.Search':'Indexed binary search with monotone predicates, insertion positions and effectful three-way comparisons.',
'Data.Sorted':'Lists refined by sortedness, with construction and combination that preserve ordering.',
'Data.Ascii':'ASCII-refined values and checked construction at text boundaries.',
'Data.Identity':'The identity container and its generic abstraction instances.',
'Data.Const':'A constant container that retains its first parameter while ignoring the mapped value parameter.',
'Data.Compose':'Composition of higher-kinded containers with the admitted generic instances.',
'Data.FromList':'The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type.',
'Data.FromMap':'The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type.',
'Data.Path':'Platform path values and composition/conversion operations used by filesystem and process APIs.',
'Data.Json':'The JSON value representation shared by parsers, renderers, codecs, and structured observability.',
'Data.Producer':'Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive.',
'Data.Target':'Target/platform and worker vocabulary used by the compiler-supported runtime and target examples.',
'Data.Rational':'Rational numbers with explicit denominator/error policy and arithmetic operations.',
'Data.BigDecimal':'Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API.',
'Data.Complex':'Complex values and numeric operations using the module\'s current floating representation.',
'Data.Matrix':'Matrix construction, shape checks, indexing, transformation, and arithmetic operations.',
'Data.Date':'Civil date values, validation, parsing/formatting, and calendar calculations.',
'Data.Time':'Time values, durations, and conversions with explicit units and admitted ranges.',
'Data.IpAddr':'Validated IP addresses, parsing/rendering, and address-family vocabulary.',
'Data.StringIo':'In-memory string-oriented I/O utilities with explicit state and error results.',
'Function.Combinators':'Small reusable function operations such as identity, composition, argument manipulation, and application.',
'Relation.Eq':'Equality/inequality class vocabulary. Instances live with their types.',
'Relation.Ord':'Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit.',
'Algebra.Arithmetic':'Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements.',
'Algebra.Bits':'Bit operations, shifts, and the corresponding operator/fixity vocabulary.',
'Algebra.Semigroup':'Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs.',
'Algebra.Monoid':'Identity plus combination vocabulary, default folding, and unit laws with explicit checking status.',
'Algebra.Group':'Group-style inverse/subtraction vocabulary over admitted carrier instances.',
'Algebra.Semiring':'Zero/one and additive/multiplicative structure, with its law vocabulary.',
'Category.Functor':'Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism.',
'Category.Applicative':'Container application and value lifting for independent structure-preserving composition.',
'Category.Monad':'Container-dependent composition through the admitted higher-kinded class vocabulary.',
'Category.Alternative':'Empty and choice operations for admitted higher-kinded containers.',
'Category.Foldable':'Generic folds, conversion to lists, size/emptiness, and related operations over container instances.',
'Category.Traversable':'Generic effect/container-aware traversal and sequencing while retaining the input shape.',
'Effect.Sequencing':'The direct, Result, and Option do providers. One lexical provider/container is selected per definition.',
'Effect.Concurrent':'Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations.',
'Effect.Concurrent.MVar':'Native single-place shared variables with FIFO handoff, blocking take/put/read/swap, nonblocking operations, and cancellable waits through the task scheduler.',
'Effect.Stm':'Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR.',
'Effect.Stm.TMVar':'A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions.',
'Effect.Stm.TQueue':'An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables.',
'Effect.Stm.TBQueue':'A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty.',
'Effect.Stm.TSem':'A counting semaphore for transactions: wait takes a permit or retries, signal returns one.',
'Effect.Io':'A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration.',
'Effect.Clock':'Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs.',
'Prim.Sys.Darwin':'The Darwin half of the syscall layer: libSystem calls bound with foreign import ccall (open, read, write, close, lseek, fstat, mkdir, unlink, rename, fsync, getpid), with the -1 result mapped to a named error read from the thread error slot.',
'Prim.Sys.Linux':'The Linux half of the syscall layer: prim bodies that issue syscall with the pinned number per architecture (x86_64 and aarch64), with the kernel negative result mapped to the same named errors as the Darwin module.',
'Prim.Sys':'The syscall layer for the compile target under one name: conditional imports and re-exports select Prim.Sys.Darwin on a Darwin target, Prim.Sys.Linux on a Linux target, and the portable floor everywhere else, so a caller imports one module and never names an operating system.',
'Prim.Sys.Portable':'The floor under Prim.Sys: the same entry names and the same named errors as the real backends, with every entry point answering Err SysIoError. It is what a target with no backend yet gets, and it keeps the prelude surface from being empty there.',
'Effect.Fs':'Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors.',
'Effect.Buffered.Core':'Byte-stream events with signed progress, simultaneous failures, shared advanced continuations and explicit transfer capabilities.',
'Effect.Buffered.Internal.Cell':'Initialized opaque continuation cells whose allocation, observation and replacement require their I/O region.',
'Effect.Stream.Offset':'Positional byte writers, signed cursor movement and streaming at a base offset.',
'Effect.Stream.Section':'Positional byte sources, immutable bounded sections, signed seeks and sequential reader adapters.',
'Effect.Stream':'Composable byte readers and writers with explicit continuation state, memory and descriptor adapters, limits, mirrors, and exact or bulk transfers.',
'Effect.Process':'One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process\'s arguments.',
'Effect.Net':'Socket operations with explicit handles, address values, errors, and runtime effects.',
'Effect.Http':'A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation.',
'Effect.Resolv':'Hostname/address resolution through the runtime, with named failure results.',
'Effect.Signal':'Operating-system signal vocabulary and runtime operations with explicit error/effect handling.',
'Effect.Pty':'Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations.',
'Effect.Random':'Runtime entropy/random utilities with explicit effectful access.',
'Effect.FileUtils':'Convenience filesystem operations built on the lower-level filesystem API.',
'Effect.Find':'Filesystem walking/search utilities with the module\'s traversal and error policy.',
'Effect.Log':'Structured log operations and records with runtime capability/effect accounting.',
'Effect.Trace':'Scoped tracing/span vocabulary and context propagation tied to the runtime identity.',
'Effect.Test':'A subprocess-based test runner and named test-result/report vocabulary.',
'Effect.Benchmark':'Benchmark case execution and timing/reporting helpers over runtime clocks.',
'Effect.Coverage':'Coverage collection/reporting utilities with explicit runtime and source metadata.',
'Text.Show':'Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself.',
'Text.Codec':'Encoding/decoding signatures and codec helpers with explicit failure results.',
'Text.Lex':'Lexer toolkit: tokens, spans, and lexical errors for grammar and parser clients.',
'Text.Scan':'Scanning primitives over input text and the shared position/error vocabulary.',
'Text.Parse':'Parser combinator toolkit over the lexer/scanner vocabulary.',
'Text.Json':'JSON parsing and rendering over Data.Json, with explicit errors and builder-based output.',
'Text.Json.Value':'Owned JSON documents and navigation tapes, raw number spellings, exact numeric values and input-bearing errors, resumable lexical state and byte-preserving formatting.',
'Text.Json.Dynamic':'Owned dynamic JSON trees, separate encoding and decoding descriptors, typed payloads and bounded construction.',
'Text.Json.Legacy':'Explicit legacy codec policy and descriptor bridges over owned dynamic values.',
'Text.Json.Codec':'Typed JSON codecs for records, persistent maps, explicit key capabilities, machine integers, binary buffers and exact binary32/binary64 values.',
'Text.Json.Binary32':'Exact decimal rounding and shortest finite binary32 JSON coefficients, with preserved signed zero and midpoint comparisons.',
'Text.Mime':'Byte-preserving media types, encoded words, immutable parameter maps and explicit extension registries.',
'Effect.Mime':'Encoded-word and header decoding with explicit charset reader capabilities, typed errors and advanced writer continuations.',
'Text.Pem':'Owned byte-preserving envelope blocks, persistent headers, exact malformed-input recovery and bounded payload encoding.',
'Effect.Pem':'Validated envelope output through explicit advanced writer continuations, bounded chunks and linear short-write draining.',
'Text.QuotedPrintable':'Quoted-printable byte encoding and decoding with checked line wrapping, escape and malformed-input policies.',
'Effect.QuotedPrintable':'Incremental quoted-printable readers and writers with explicit binary policy, partial progress and sticky errors.',
'Text.Builder':'An efficient text-construction vocabulary for assembling many fragments before rendering.',
'Text.Regex':'Regular-expression parsing/matching and replacement under the module\'s admitted regex language.',
'Text.Args':'Command-line argument processing and its named value/error vocabulary.',
'Text.OptionParser':'Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors.',
'Text.Csv':'CSV parsing/rendering with explicit field, record, and malformed-input handling.',
'Text.Html':'HTML text/byte escaping, complete HTML5 character references, exact escaped sizing and SIMD/SWAR scanning.',
'Text.Yaml':'The module\'s supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature.',
'Text.Toml':'The module\'s supported TOML parsing/rendering surface and explicit unsupported/malformed cases.',
'Text.Uri':'URI parsing, rendering, components, and validation/error vocabulary.',
'Text.Base64':'Base64 encoding/decoding over bytes with explicit invalid-input results.',
'Text.Url':'Immutable byte-preserving URLs, percent escaping, query multimaps, parsing and reference resolution.',
'Text.Url.Codec':'Checked percent codecs for URL components, exact escaped sizing and optional UTF-8 text views.',
'Text.Url.Error':'Structured URL failures with original byte offsets, context, diagnostics and explicit causes.',
'Text.Url.Query':'Persistent byte-preserving query multimaps with deterministic encoding, checked parsing and copied observations.',
'Text.Url.Userinfo':'Immutable copied URL user information with explicit password presence and checked percent encoding.',
'Text.Url.Model':'Immutable URL components, raw encoding hints, rendering and independent copied observations.',
'Text.Url.Parser':'Checked general and request URL parsing with precise scheme, host, port and escape rules.',
'Text.Url.Resolve':'URL reference resolution, lexical path joining and copied persistent URL updates.',
'Text.Url.Ip':'URL host IP-literal and port validation with explicit IPv6 and zone rules.',
'Text.Url.Scan':'Private portable SIMD byte scanning shared by URL parsing, queries and path traversal.',
'Text.Binary.Layout':'Typed scalar, floating-point, complex and aggregate binary layouts with checked buffers, exact sizes and packed array kernels.',
'Effect.Binary':'Explicit fixed and variable binary stream codecs preserving progress and advanced handles after failures.',
'Text.Csv.Record':'Configurable Unicode-delimited and lossless byte CSV records, checked partial results, positions and incremental parser state.',
'Effect.Csv':'Bounded CSV readers and writers with explicit continuation, record counts, position tracking and advanced handles after failures.',
'Text.Base64.Encoding':'Configurable Base64 byte alphabets, padding and strict trailing bits, SIMD codecs, persistent states and checked caller buffers.',
'Effect.Base64':'Explicitly finalized Base64 writers and bounded decoding readers with sticky errors and isolated output chunks.',
'Text.Ascii85':'ASCII85 text, caller-buffer and incremental codecs with consumed-input progress, checked source offsets, SIMD groups and control-run scanning.',
'Text.Utf16':'UTF-16 scalar and surrogate operations, code-unit sequences and explicit-order byte codecs with SWAR ASCII blocks.',
'Effect.Hex':'Hexadecimal encoding writers, decoding readers and formatted dump sinks with explicit continuations and sticky errors.',
'Effect.Fnv':'Inspectable FNV hashing sinks, generic byte-writer composition and bounded reader-to-hash copying with advanced continuations.',
'Effect.Adler32':'Inspectable scalar checksum sinks, generic byte writers and bounded reader-to-Adler-32 copying with advanced source continuations.',
'Effect.Crc32':'CRC-32 stream writers and bounded copying into an inspectable checksum state.',
'Effect.Crc64':'Inspectable CRC-64 states, generic byte writers and bounded reader-to-checksum copying with advanced source continuations.',
'Effect.Sha512':'Inspectable SHA-384/SHA-512/SHA-512/t states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations.',
'Effect.Sha1':'Inspectable SHA-1 states, generic digest sinks and bounded reader-to-hash copying for legacy formats.',
'Effect.Md5':'Inspectable MD5 states, generic digest sinks and bounded reader-to-hash copying for legacy formats.',
'Effect.Sha3':'Inspectable SHA-3/SHAKE states, generic digest sinks, bounded reader-to-hash copying and SHAKE output as a generic reader.',
'Effect.Crypto':'Ed25519, X25519 and AES key generation and random-nonce GCM over the operating system random source.',
'Effect.Sha256':'Inspectable SHA-224/SHA-256 states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations.',
'Effect.Ascii85':'ASCII85 encoding writers and decoding byte readers with explicit finalization, bounded read-ahead and sticky stream errors.',
'Text.Base32':'Configurable Base32 byte alphabets and padding, SIMD codecs, prefix/caller-buffer operations and persistent incremental states.',
'Effect.Base32':'Explicitly finalized Base32 encoding writers and bounded decoding readers with precise errors, retained cursors and sticky failures.',
'Text.Path':'Portable slash-path lexical operations and checked shell patterns with reusable compiled matchers and exact SIMD scans.',
'Text.Binary':'Little- and big-endian integer encoding, checked buffer access, and signed/unsigned varints.',
'Text.Gzip':'Whole-buffer gzip member encoding, checked decompression and concatenated members.',
'Text.Hex':'SIMD hexadecimal text/byte codecs, prefix-preserving decoding, append and destination operations, and formatted byte dumps.',
'Text.Checksum':'Incremental Adler-32, CRC-32 and CRC-64 checksums with explicit reflected polynomials.',
'Data.Bytes.Word':'Bounds-proved unaligned little-endian machine-word reads and writes for SIMD and SWAR processing.',
'Data.Bytes.Search':'Exact first, last, containment and count queries over raw bytes with bounded SWAR traversals.',
'Data.Bytes.Build':'Fresh byte cloning, doubling repetition, concatenation and separator-aware joining.',
'Data.Bytes.View':'Owner-retaining checked byte views, relative UTF-8 observations, stable compiled substring patterns and bounded search, cut and comparison operations.',
'Data.Bytes.Split':'Retained byte splitting, persistent producer traversals, shared pull cursors and line iteration.',
'Data.Mutable.HeapView':'In-place heap ordering and indexed repairs through explicit borrowed storage callbacks.',
'Data.Mutable.LinkedList':'Stable typed list elements, checked membership and movement, detached value access and bounded self-copy.',
'Data.Mutable.Ring':'Mutable circular sequences with stable cursors, alias-preserving joins and splits, explicit value ownership and bounded arena growth.',
'Data.Bytes.Utf8':'Raw UTF-8 validation, forward and reverse decoding, scalar counting, fresh repair and atomic encoding.',
'Data.Word':'Bit lengths, byte reversal, signed rotations and checked carry/borrow, wide multiply, divide and remainder operations over fixed and native-width words.',
'Data.Wide':'Unboxed 128-bit integers with checked, wrapping and saturating arithmetic, bit operations, numeric conversions and explicit overflow policies.',
'Text.FilePath':'POSIX lexical relative paths, locality tests, filesystem-name conversion and search-list splitting.',
'Text.Fnv':'Incremental 32-, 64- and 128-bit FNV-1 and FNV-1a hashing with explicit wrapping arithmetic.',
'Text.Fnv.State':'Persistent FNV states with reset, network-order digests, numeric observations and checked versioned binary checkpoints.',
'Text.Adler32':'Portable SIMD Adler-32 with bounded machine-word segments, persistent states, network-order digests and checked versioned checkpoints.',
'Text.Crc32':'Prepared reflected polynomial tables, persistent CRC-32 states and table-checked binary checkpoints.',
'Text.Crc64':'Immutable SIMD-built polynomial tables, slicing-by-eight CRC-64, persistent states, network-order digests and table-checked checkpoints.',
'Text.Sha512':'Portable SHA-384, SHA-512, SHA-512/224 and SHA-512/256 with two-lane SIMD schedules, native rotates, persistent states and checked binary checkpoints.',
'Text.Sha1':'SHA-1 for legacy interoperability, with a four-lane SIMD schedule, persistent states and checked binary checkpoints.',
'Text.Md5':'MD5 for legacy interoperability, with persistent states, little-endian digests and checked binary checkpoints.',
'Text.Sha3':'SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256 over a register-resident Keccak permutation, with persistent sponges, output readers and checkpoints.',
'Text.TabWriter':'Elastic tabstops: align tab-separated cells into padded columns, with escapes, HTML-aware widths, right alignment and incremental flushing.',
'Text.Bzip2':'Decompress concatenated bzip2 streams with table-driven Huffman decoding and verified block and stream CRCs.',
'Text.Mail':'Parse and format RFC 5322 mail addresses, read message header blocks and parse header dates, with SIMD atom scanning.',
'Crypto.Des':'DES and Triple DES block ciphers with ECB, CBC and CTR for legacy interoperation, from a generated straight-line round kernel.',
'Crypto.Rc4':'The RC4 stream cipher as a persistent keystream state, for legacy data only.',
'Image.Gif':'Decode and encode GIF images and animations: palettes, transparency, delays, disposal, looping and interlace.',
'Text.Xml':'Decode XML into namespace-aware tokens or element trees and encode tokens with escaping and indentation.',
'Crypto.MlKem':'ML-KEM-768 and ML-KEM-1024 key encapsulation (FIPS 203) with branch-free SIMD lane kernels.',
'Text.Template':'Data-driven text templates: pipelines, control actions, definitions, builtins and printf over dynamic values.',
'Image.Draw':'Composite rasters with Src and Over, masks and uniform colors, and dither onto palettes, with a vector Over kernel.',
'Crypto.P256':'Constant-time NIST P-256 keys, SEC 1 point encodings, ECDH and scalar/point arithmetic over generated Montgomery kernels.',
'Crypto.Ecdsa':'Deterministic (RFC 6979) and hedged P-256 ECDSA signatures in raw and DER form with strict verification.',
'Image.Jpeg':'Baseline and progressive JPEG decoding to YCbCr, gray, RGB or CMYK planes, and baseline encoding, with vector IDCT, FDCT, Huffman and colour kernels.',
'Crypto.ChaCha20Poly1305':'ChaCha20 and XChaCha20 streams, Poly1305 and the ChaCha20-Poly1305 and XChaCha20-Poly1305 AEADs with vector row kernels.',
'Crypto.Hpke':'Hybrid public key encryption (RFC 9180) over DHKEM X25519 and P-256, HKDF and AES-GCM or ChaCha20-Poly1305, in all four modes.',
'Text.Smtp':'SMTP without a connection: reply state machine, command validation, extensions, dot-stuffing and PLAIN/CRAM-MD5 exchanges.',
'Effect.Smtp':'An SMTP client over sockets with explicit client state: greeting with HELO fallback, authentication, envelope, data, quit and send-mail.',
'Data.ChaCha8':'The ChaCha8 pseudorandom generator: four interleaved eight-round blocks per vector kernel call, byte reads and state encoding.',
'Text.Cookie':'HTTP cookies: parse Set-Cookie and Cookie header lines, serialize cookies as Set-Cookie values and validate them.',
'Data.CookieJar':'An RFC 6265 cookie jar as an immutable value: store the cookies responses set and choose the cookies a request sends.',
'Text.Punycode':'Punycode (RFC 3492) encoding and decoding, and ASCII-compatible host names with xn-- labels.',
'Data.Float':'Binary64 mathematics: roundings, roots, exponentials, logarithms, powers, trigonometric, hyperbolic and special functions (erf, Gamma, Bessel) and FMA as branch-free kernels, bit-identical on every target.',
'Data.Rand':'Pseudorandom sources as values (PCG and ChaCha8) and their distributions: bounded integers, floats, permutations, ziggurat normal and exponential variates, and Zipf.',
'Effect.Rand':'Pseudorandom sources seeded from operating-system entropy.',
'Text.Lzw':'Variable-width-code LZW compression and decompression in either bit order, as used by GIF, TIFF and PDF.',
'Text.Tar':'Read V7, USTAR, PAX and GNU tape archives and write USTAR or PAX archives, with SIMD header checksums.',
'Text.TextProto':'Text-protocol framing: lines, canonical MIME headers, status responses and dot encoding for HTTP, SMTP and mail.',
'Text.Zip':'Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or deflated archives.',
'Text.Asn1':'ASN.1 value trees with a strict DER decoder and canonical DER encoder, including explicit and implicit tags and times.',
'Text.Sha256':'Portable SHA-224/SHA-256 with generated SIMD schedules, persistent states, network-order digests and checked binary checkpoints.',
'Text.Digest':'Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol.',
'Text.Shellwords':'Shell-style word splitting/joining; this is data parsing, not execution of a shell command.',
'Text.Zlib':'Compression/decompression operations and named malformed/unsupported-stream results.',
'Text.Otlp':'OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers.',
}

SUMMARIES={}
STDLIB_BASE = '/packages/stdlib'
SEARCH_URL = '/packages/search/'
SEARCH_INDEX = ROOT / 'website/public/stdlib-search-index.json'

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cinch_type_format as ctf

BUILTIN_TYPE_LINKS = {
    'Int': f'{STDLIB_BASE}/Data.Int/',
    'Nat': f'{STDLIB_BASE}/Data.Nat/',
    'Bool': f'{STDLIB_BASE}/Data.Bool/',
    'Char': f'{STDLIB_BASE}/Data.Char/',
    'String': f'{STDLIB_BASE}/Data.String/',
    'Bytes': f'{STDLIB_BASE}/Data.Bytes/',
    'Array': f'{STDLIB_BASE}/Data.Array/',
    'Ordering': f'{STDLIB_BASE}/Data.Ordering/',
    'Sym': f'{STDLIB_BASE}/Data.Sym/',
    'Path': f'{STDLIB_BASE}/Data.Path/',
    'Json': f'{STDLIB_BASE}/Data.Json/',
    'I8': f'{STDLIB_BASE}/Data.Machine/',
    'I16': f'{STDLIB_BASE}/Data.Machine/',
    'I32': f'{STDLIB_BASE}/Data.Machine/',
    'I64': f'{STDLIB_BASE}/Data.Machine/',
    'Isize': f'{STDLIB_BASE}/Data.Machine/',
    'U8': f'{STDLIB_BASE}/Data.Machine/',
    'U16': f'{STDLIB_BASE}/Data.Machine/',
    'U32': f'{STDLIB_BASE}/Data.Machine/',
    'U64': f'{STDLIB_BASE}/Data.Machine/',
    'Usize': f'{STDLIB_BASE}/Data.Machine/',
    'F32': f'{STDLIB_BASE}/Data.Machine/',
    'F64': f'{STDLIB_BASE}/Data.Machine/',
    'I128': f'{STDLIB_BASE}/Data.Wide/',
    'U128': f'{STDLIB_BASE}/Data.Wide/',
    'Unit': '/reference/04-types-and-kinds/',
    'Io': '/reference/09-effects-and-capabilities/',
    'IO': '/reference/09-effects-and-capabilities/',
    'Diverge': '/reference/09-effects-and-capabilities/',
    'Mut': '/reference/10-ownership-and-memory/',
    'Ref': '/reference/10-ownership-and-memory/',
    'Async': '/reference/13-concurrency-and-runtime/',
    'Cancel': '/reference/13-concurrency-and-runtime/',
    'Scope': '/reference/13-concurrency-and-runtime/',
    'STM': f'{STDLIB_BASE}/Effect.Stm/',
    'TVar': f'{STDLIB_BASE}/Effect.Stm/',
    'MVar': f'{STDLIB_BASE}/Effect.Concurrent.MVar/',
}


def describe(name):
    """Return the curated description, or the first sentence of the module's own documentation comment."""
    return DESCRIPTIONS.get(name) or SUMMARIES.get(name) or f'The {name} module.'


def summarize(markdown):
    """First sentence (or line) of a module documentation comment, as one plain line."""
    text = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', markdown)
    text = re.sub(r'[`*_]', '', text)
    text = ' '.join(text.strip().split())
    match = re.match(r'(.+?[.!?])(\s|$)', text)
    summary = match.group(1) if match else text
    return summary[:220].rstrip() + ('…' if len(summary) > 220 else '')


def sources():
    return {'.'.join(p.relative_to(PRELUDE).with_suffix('').parts): p for p in sorted(PRELUDE.rglob('*.cinch'))}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def anchor(kind, name):
    prefix = {
        'type': 't',
        'ctor': 'k',
        'class': 'c',
        'method': 'm',
        'value': 'v',
        'instance': 'i',
    }.get(kind, 'v')
    if re.fullmatch(r'[A-Za-z0-9_-]+', name):
        return f'{prefix}-{name}'
    if '-' in name:
        head, tail = name.split('-', 1)
        if re.fullmatch(r'[A-Za-z0-9_]+', head) and tail and not re.search(r'[A-Za-z0-9]', tail):
            return f'{prefix}-{head}-op-{tail.encode().hex()}'
    if not re.search(r'[A-Za-z0-9]', name):
        return f'{prefix}-op-{name.encode().hex()}'
    safe = re.sub(r'[^A-Za-z0-9_-]', lambda m: f'_{m.group(0).encode().hex()}_', name).strip('_')
    return f'{prefix}-{safe}'


def alloc_anchor(used_ids, kind, name):
    base = anchor(kind, name)
    cand = base
    n = 2
    while cand in used_ids:
        cand = f'{base}-{n}'
        n += 1
    used_ids.add(cand)
    return cand


def public_class(item):
    return bool(item.get('exports'))


def markdown_text(text):
    return text.replace('|', '\\|').replace('\n', ' ')


def esc_html(text):
    return (
        text.replace('&', '&amp;')
        .replace('<', '&lt;')
        .replace('>', '&gt;')
        .replace('"', '&quot;')
    )


def flatten_decls(declarations):
    for decl in declarations:
        yield decl
        yield from flatten_decls(decl['children'])


def doc_json(compiler, path, module, iface=None, allow_orphans=False):
    command = [str(compiler), 'doc', 'extract', str(path), '--module', module, '--legacy']
    if iface:
        command += ['--iface', str(iface)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=120)
    ok_codes = (0, 1, 2) if allow_orphans else (0, 2)
    if result.returncode not in ok_codes or not result.stdout.strip():
        raise RuntimeError(f'{path}: cinch doc extract failed\n{result.stderr}')
    return json.loads(result.stdout)


def highlighted_markdown(compiler, text):
    if '```cinch' not in text:
        return text
    result = subprocess.run([str(compiler), 'doc', 'markdown'], input=text, capture_output=True, text=True, timeout=120)
    if result.returncode:
        raise RuntimeError(f'cinch doc markdown failed\n{result.stderr}')
    return result.stdout


def resolve_symbol_links(markdown, symbol_links):
    """Turn `[label](symbol:Target)` links into real links when Target is known."""
    def repl(match):
        label, target = match.group(1), match.group(2)
        href = symbol_links.get(target)
        if href:
            return f'[{label}]({href})'
        return label
    return re.sub(r'\[([^\]]+)\]\(symbol:([^)\s]+)\)', repl, markdown)


def unpack(package, out):
    """Project a packed interface object into one canonical `.cinchif` document per module.
    Builds that still write loose interfaces beside the package are copied unchanged."""
    loose = list(package.parent.glob('*.cinchif'))
    for path in loose:
        shutil.copyfile(path, out / path.name)
    if loose or not package.exists():
        return
    sys.path.insert(0, str(ROOT / 'tooling'))
    from pack_interfaces import PackedInterfaces, canonical
    for name, _, document in PackedInterfaces(package.read_bytes()).documents:
        (out / (name + '.cinchif')).write_bytes(canonical(document))


def collect_interfaces(compiler, save_dir=None):
    temp = tempfile.TemporaryDirectory(prefix='cinch-library-')
    work = Path(temp.name)
    stdlib_manifest = json.loads((PRELUDE / '.cinchpkg').read_text())
    all_sources = sources()
    ordered_names = [m['name'] for m in stdlib_manifest['modules'] if m['name'] in all_sources]
    env = {
        **os.environ,
        'CINCH_STORE': str(work / 'store'),
        'CINCH_STDLIB': str(PRELUDE),
        'RUST_MIN_STACK': '67108864',
    }
    entry = work / 'Main.cinch'
    ifaces = save_dir if save_dir else (work / 'interfaces')
    ifaces.mkdir(parents=True, exist_ok=True)

    def check_batch(names):
        imports_src = ''.join(f'  import {n} as M{i}\n' for i, n in enumerate(names))
        entry.write_text('module Main where\n' + imports_src + '  main :: Int\n  main = 0\n')
        result = subprocess.run([compiler, 'check', str(entry)], cwd=work, env=env, capture_output=True, text=True, timeout=180)
        if result.returncode:
            return result.stderr
        unpack(work / 'build' / 'Main.cpk', ifaces)
        return None

    # First check Prelude (which populates the base interfaces in one pass)
    if not (ifaces / 'Prelude.cinchif').exists():
        err = check_batch([])
        if err:
            raise RuntimeError(f'Rust compiler rejected Prelude:\n{err}')
        print('Checked Prelude', flush=True)

    remaining = [n for n in ordered_names if not (ifaces / (n + '.cinchif')).exists()]
    batch_size = 24
    for start in range(0, len(remaining), batch_size):
        batch = [n for n in remaining[start:start + batch_size] if not (ifaces / (n + '.cinchif')).exists()]
        if not batch:
            continue
        err = check_batch(batch)
        if err is None:
            print(f'Checked batch ({len(batch)} modules: {batch[0]} .. {batch[-1]})', flush=True)
            continue
        # Fallback to individual checks if a batch hits an instance/coherence clash
        for name in batch:
            if (ifaces / (name + '.cinchif')).exists():
                continue
            single_err = check_batch([name])
            if single_err:
                raise RuntimeError(f'Rust compiler rejected {name}:\n{single_err}')
            print('Checked ' + name, flush=True)
    return temp, ifaces


def build_global_maps(all_sources, interfaces):
    """Build cross-module type links, symbol links, and cross-module instance indices."""
    type_links = dict(BUILTIN_TYPE_LINKS)
    symbol_links = {}
    instances_by_type = {}
    instances_by_class = {}

    # Priority: public non-Prelude non-Data.Core modules win short names first.
    ordered_modules = sorted(
        all_sources.keys(),
        key=lambda m: (2 if m == 'Prelude' else 1 if m == 'Data.Core' else 0, m),
    )

    for mod in ordered_modules:
        exp = interfaces[mod]['exports']
        for ty in exp['types']:
            if ty.get('kind') == 'signature':
                continue
            aid = anchor('type', ty['name'])
            href = f'{STDLIB_BASE}/{mod}/#{aid}'
            ident = ty.get('identity', f'{mod}.{ty["name"]}')
            # Prefer public home (e.g. Data.Result for Data.Core.Result)
            if mod != 'Prelude':
                type_links.setdefault(ident, href)
                type_links.setdefault(f'{mod}.{ty["name"]}', href)
                type_links.setdefault(ty['name'], href)
                symbol_links.setdefault(ident, href)
                symbol_links.setdefault(f'{mod}.{ty["name"]}', href)
                symbol_links.setdefault(ty['name'], href)
            for ctor in ty.get('ctors', []):
                caid = anchor('ctor', ctor['name'])
                chref = f'{STDLIB_BASE}/{mod}/#{caid}'
                cid = f'{mod}.{ty["name"]}.{ctor["name"]}'
                if mod != 'Prelude':
                    symbol_links.setdefault(cid, chref)
                    symbol_links.setdefault(f'{mod}.{ctor["name"]}', chref)
        for sig in exp['signatures']:
            if not (public_class(sig) or any(m.get('exports') for m in sig['methods'])):
                continue
            aid = anchor('class', sig['name'])
            href = f'{STDLIB_BASE}/{mod}/#{aid}'
            ident = sig.get('identity', f'{mod}.{sig["name"]}')
            if mod != 'Prelude':
                type_links.setdefault(ident, href)
                type_links.setdefault(f'{mod}.{sig["name"]}', href)
                type_links.setdefault(sig['name'], href)
                symbol_links.setdefault(ident, href)
                symbol_links.setdefault(f'{mod}.{sig["name"]}', href)
                symbol_links.setdefault(sig['name'], href)
            for method in sig['methods']:
                if not method.get('exports'):
                    continue
                maid = anchor('method', f'{sig["name"]}-{method["name"]}')
                mhref = f'{STDLIB_BASE}/{mod}/#{maid}'
                if mod != 'Prelude':
                    symbol_links.setdefault(f'{ident}.{method["name"]}', mhref)
                    symbol_links.setdefault(f'{mod}.{method["name"]}', mhref)
        for val in exp['values']:
            if mod == 'Prelude':
                continue
            aid = anchor('value', val['name'])
            href = f'{STDLIB_BASE}/{mod}/#{aid}'
            sym = val.get('symbol', f'{mod}.{val["name"]}')
            symbol_links.setdefault(sym, href)
            symbol_links.setdefault(f'{mod}.{val["name"]}', href)
            symbol_links.setdefault(val['name'], href)

    # Collect cross-module instances (skip Prelude facade re-exports to avoid duplicates)
    seen_inst = set()
    for mod in sorted(all_sources.keys()):
        if mod == 'Prelude':
            continue
        exp = interfaces[mod]['exports']
        for inst in exp['instances']:
            iid = inst.get('id', '')
            target = inst.get('target', '')
            key = (iid, target)
            if key in seen_inst:
                continue
            seen_inst.add(key)
            cls_name, ty_name = ctf.extract_instance_heads(target)
            row = {
                'id': iid,
                'module': mod,
                'target': target,
                'implicit': bool(inst.get('implicit')),
            }
            if cls_name:
                instances_by_class.setdefault(cls_name, []).append(row)
            if ty_name:
                instances_by_type.setdefault(ty_name, []).append(row)

    return type_links, symbol_links, instances_by_type, instances_by_class


def render_instances_drawer(rows, type_links):
    if not rows:
        return ''
    items = []
    for row in rows:
        _, linked_target = ctf.link_instance_target(row['target'], type_links)
        res = 'implicit' if row['implicit'] else 'explicit'
        mod = row['module']
        items.append(
            f'<li><code>{linked_target}</code> '
            f'<span class="doc-inst-meta">{res} · <a href="{STDLIB_BASE}/{esc_html(mod)}/#instances">{esc_html(mod)}</a></span></li>'
        )
    return (
        f'<details class="doc-instances">'
        f'<summary>Instances ({len(rows)})</summary>'
        f'<ul>{"".join(items)}</ul>'
        f'</details>'
    )


def render_entry_block(aid, sig_html, body_md, src_mod, line, extra_html=''):
    links = [f'<a href="#{esc_html(aid)}" title="Permalink">#</a>']
    if src_mod:
        src_href = f'{STDLIB_BASE}/{esc_html(src_mod)}/source/' + (f'#L{line}' if line else '')
        src_title = f'View source at line {line}' if line else 'View source'
        links.append(f'<a href="{src_href}" title="{src_title}">Source</a>')
    sig_bar = (
        f'<div class="doc-sig-bar">'
        f'<code class="doc-sig">{sig_html}</code>'
        f'<span class="doc-sig-links">{"".join(links)}</span>'
        f'</div>'
    )
    parts = [f'<article class="doc-entry" id="{esc_html(aid)}">', sig_bar]
    if body_md.strip() or extra_html.strip():
        parts.append('<div class="doc-body">')
        if body_md.strip():
            parts.append('')
            parts.append(body_md.strip())
            parts.append('')
        if extra_html.strip():
            parts.append(extra_html.strip())
        parts.append('</div>')
    parts.append('</article>')
    parts.append('')
    return '\n'.join(parts)


def generate(ifaces, compiler):
    all_sources = sources()
    interfaces = {name: json.loads((ifaces / (name + '.cinchif')).read_text()) for name in all_sources}
    documents = {name: doc_json(compiler, path, name, ifaces / (name + '.cinchif')) for name, path in all_sources.items()}
    for name, parsed in documents.items():
        if parsed['doc'] and parsed['doc']['markdown'].strip():
            SUMMARIES[name] = summarize(parsed['doc']['markdown'])

    doc_by_id = {decl['id']: decl for parsed in documents.values() for decl in flatten_decls(parsed['declarations'])}
    comments = {
        name: {
            decl['name']: {'line': decl['source']['line'], 'doc': decl['doc']['markdown']}
            for decl in flatten_decls(parsed['declarations'])
            if decl['doc']
        }
        for name, parsed in documents.items()
    }
    lines_by_id = {
        decl['id']: decl['source']['line']
        for parsed in documents.values()
        for decl in flatten_decls(parsed['declarations'])
    }
    lines_by_mod_name = {
        (name, decl['name']): decl['source']['line']
        for name, parsed in documents.items()
        for decl in flatten_decls(parsed['declarations'])
    }

    type_links, symbol_links, instances_by_type, instances_by_class = build_global_maps(all_sources, interfaces)

    downloads = ROOT / 'website/public/reference-sources'
    downloads.mkdir(parents=True, exist_ok=True)

    def info(identity, name, fallback_mod=''):
        decl = doc_by_id.get(identity)
        if decl and decl['doc']:
            return {
                'line': decl['source']['line'],
                'doc': decl['doc']['markdown'],
                'params': decl['doc']['tags'].get('params', []),
                'typeParams': decl['doc']['tags'].get('typeParams', []),
                'fields': decl['doc']['tags'].get('fields', []),
                'returns': decl['doc']['tags'].get('returns'),
                'safety': decl['doc']['tags'].get('safety'),
                'errors': decl['doc']['tags'].get('errors'),
                'laws': decl['doc']['tags'].get('laws', []),
                'effects': decl['doc']['tags'].get('effects', []),
                'since': decl['doc']['tags'].get('since'),
                'deprecated': decl['doc']['tags'].get('deprecated'),
                'experimental': decl['doc']['tags'].get('experimental'),
            }
        mod = identity.rsplit('.', 1)[0] if '.' in identity else fallback_mod
        fallback = comments.get(mod, {}).get(name, {})
        if fallback:
            return fallback
        line = lines_by_id.get(identity) or lines_by_mod_name.get((mod, name)) or lines_by_mod_name.get((fallback_mod, name)) or 0
        return {'line': line, 'doc': ''}

    def format_body(item):
        prose = resolve_symbol_links(item.get('doc', ''), symbol_links)
        parts = []
        if prose.strip():
            parts.append(prose.strip())
        extras = []
        for label, key in (('Parameters', 'params'), ('Type parameters', 'typeParams'), ('Fields', 'fields')):
            rows = item.get(key)
            if rows:
                extras += [f'**{label}**', ''] + [
                    f"- `{r['name']}`: {resolve_symbol_links(r['text'], symbol_links)}" for r in rows
                ] + ['']
        for label, key in (('Returns', 'returns'), ('Safety', 'safety'), ('Errors', 'errors')):
            val = item.get(key)
            if val:
                extras += [f'**{label}:** {resolve_symbol_links(str(val), symbol_links)}', '']
        for label, key in (('Laws', 'laws'), ('Effects', 'effects')):
            rows = item.get(key)
            if rows:
                extras += [f'**{label}**', ''] + [f'- {resolve_symbol_links(str(r), symbol_links)}' for r in rows] + ['']
        meta = []
        if item.get('since'):
            meta.append(f'Since `{item["since"]}`')
        if item.get('deprecated'):
            meta.append(f'Deprecated: {item["deprecated"]}')
        if item.get('experimental'):
            meta.append(f'Experimental: {item["experimental"]}')
        if meta:
            extras += [' · '.join(meta), '']
        if extras:
            parts.append('\n'.join(extras).strip())
        return '\n\n'.join(parts)

    total_values = 0
    total_methods = 0
    entries = []
    search_entries = []

    for name, path in all_sources.items():
        iface = interfaces[name]
        export = iface['exports']
        text = path.read_text()
        (downloads / (name + '.cinch')).write_text(text)
        imports = sorted(set(re.findall(r'^  import ([\w.]+)', text, re.M)))

        used_ids = {'synopsis', 'types', 'signatures', 'values', 'operator-fixities', 'instances', 'dependencies'}

        import_note = (
            'Implicitly in scope.' if name == 'Prelude' else f'<code>import {esc_html(name)}</code>'
        )
        mod_bar = (
            f'<div class="doc-mod-bar">'
            f'<div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> {import_note}</div>'
            f'<div class="doc-mod-bar-links">'
            f'<a href="{STDLIB_BASE}/{esc_html(name)}/source/">View source</a>'
            f'<a href="{SEARCH_URL}?q=%2B{esc_html(name)}">Search module</a>'
            f'<a href="/reference-sources/{esc_html(name)}.cinch" download="{esc_html(name)}.cinch">Download .cinch</a>'
            f'<a href="{STDLIB_BASE}/">All stdlib modules</a>'
            f'</div></div>'
        )

        lines = [f'# {name}', '', describe(name), '', mod_bar, '']

        if imports:
            dep_links = ', '.join(
                f'[{i}]({i}.md)' if i in all_sources else f'`{i}`'
                for i in imports
            )
            lines += [f'**Dependencies:** {dep_links}.', '']

        module_doc = documents[name]['doc']
        header = module_doc['markdown'] if module_doc else ''
        if not header:
            header = re.split(r'^module ', text, maxsplit=1, flags=re.M)[0]
            header = '\n'.join(re.sub(r'^-- ?', '', l) for l in header.splitlines()).strip()
        if header:
            header_resolved = resolve_symbol_links(header, symbol_links)
            lines += ['<details class="doc-notes"><summary>Module notes</summary>', '', header_resolved, '', '</details>', '']

        # Prepare sections & synopsis rows simultaneously
        synopsis_rows = []
        types_md = []
        sigs_md = []
        values_md = []

        public_types = [ty for ty in export['types'] if ty.get('kind') != 'signature']
        for ty in public_types:
            aid = alloc_anchor(used_ids, 'type', ty['name'])
            params_plain, params_html = ctf.format_type_params(ty.get('parameters', []))
            refinement = ty.get('refinement')
            if refinement:
                carrier_plain = ctf.pretty_scheme(refinement['carrier'])
                carrier_html = ctf.scheme_to_html(refinement['carrier'], type_links)
                pred_plain = refinement['predicate']
                pred_html = esc_html(pred_plain)
                plain_sig = f'type {ty["name"]} {params_plain}'.rstrip() + f' = refine ({refinement["binder"]} :: {carrier_plain}) with {pred_plain}'
                sig_html = (
                    f'<span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">{esc_html(ty["name"])}</strong>'
                    + (f' {params_html}' if params_html else '')
                    + f' = <span class="tok-keyword">refine</span> (<span class="tok-variable">{esc_html(refinement["binder"])}</span> :: {carrier_html}) '
                    + f'<span class="tok-keyword">with</span> {pred_html}'
                )
            else:
                plain_sig = f'{ty["kind"]} {ty["name"]} {params_plain}'.rstrip()
                sig_html = (
                    f'<span class="tok-keyword">{esc_html(ty["kind"])}</span> '
                    f'<strong class="doc-decl-name tok-constructor">{esc_html(ty["name"])}</strong>'
                    + (f' {params_html}' if params_html else '')
                )

            ident = ty.get('identity', f'{name}.{ty["name"]}')
            src_mod = ident.rsplit('.', 1)[0] if '.' in ident and ident.rsplit('.', 1)[0] in all_sources else name
            item = info(ident, ty['name'], name)
            body_md = format_body(item)

            # Nested constructors / fields
            children_blocks = []
            ctors = ty.get('ctors', [])
            if ctors:
                ctor_lis = []
                for ctor in ctors:
                    caid = alloc_anchor(used_ids, 'ctor', ctor['name'])
                    raw_c_scheme = ctor.get('scheme') or ' -> '.join(ctor.get('fields', []) + [ty['name']])
                    pretty_c = ctf.pretty_scheme(raw_c_scheme)
                    c_html = (
                        f'<strong class="tok-constructor">{esc_html(ctor["name"])}</strong> :: '
                        f'{ctf.scheme_to_html(raw_c_scheme, type_links)}'
                    )
                    c_info = info(f'{name}.{ty["name"]}.{ctor["name"]}', ctor['name'], src_mod)
                    c_doc = c_info.get('doc', '').strip()
                    c_doc_html = f'<div class="doc-child-prose">{esc_html(summarize(c_doc))}</div>' if c_doc else ''
                    ctor_lis.append(f'<li id="{esc_html(caid)}"><code>{c_html}</code>{c_doc_html}</li>')
                    if name != 'Prelude':
                        search_entries.append({
                            'm': name,
                            'n': ctor['name'],
                            'k': 'k',
                            's': f'{ctor["name"]} :: {pretty_c}',
                            'd': summarize(c_doc) if c_doc else f'Constructor of {ty["name"]}.',
                            'h': caid,
                            'l': c_info.get('line', item.get('line', 0)) or 0,
                            **({'sm': src_mod} if src_mod != name else {}),
                        })
                children_blocks.append(f'<div class="doc-children"><h4>Constructors</h4><ul>{"".join(ctor_lis)}</ul></div>')

            fields = ty.get('fields', [])
            if fields:
                field_lis = []
                for f in fields:
                    if isinstance(f, dict):
                        fname = f['name']
                        fty = f.get('ty', f.get('scheme', ''))
                        f_html = f'<span class="tok-variable">{esc_html(fname)}</span> :: {ctf.scheme_to_html(fty, type_links)}'
                    else:
                        f_html = esc_html(str(f))
                    field_lis.append(f'<li><code>{f_html}</code></li>')
                children_blocks.append(f'<div class="doc-children"><h4>Fields</h4><ul>{"".join(field_lis)}</ul></div>')

            inst_html = render_instances_drawer(instances_by_type.get(ty['name'], []), type_links)
            if inst_html:
                children_blocks.append(inst_html)

            types_md.append(render_entry_block(aid, sig_html, body_md, src_mod, item.get('line', 0), '\n'.join(children_blocks)))
            synopsis_rows.append(f'<li><a class="syn-name" href="#{esc_html(aid)}">{esc_html(ty["name"])}</a><code class="syn-sig">{sig_html}</code></li>')

            if name != 'Prelude':
                search_entries.append({
                    'm': name,
                    'n': ty['name'],
                    'k': 't',
                    's': plain_sig,
                    'd': summarize(item.get('doc', '')) if item.get('doc') else '',
                    'h': aid,
                    'l': item.get('line', 0) or 0,
                    **({'sm': src_mod} if src_mod != name else {}),
                })

        classes = [s for s in export['signatures'] if public_class(s) or any(m.get('exports') for m in s['methods'])]
        for sig in classes:
            aid = alloc_anchor(used_ids, 'class', sig['name'])
            params_plain, params_html = ctf.format_type_params(sig.get('params', []))
            plain_sig = f'signature {sig["name"]} {params_plain}'.rstrip()
            sig_html = (
                f'<span class="tok-keyword">signature</span> '
                f'<strong class="doc-decl-name tok-constructor">{esc_html(sig["name"])}</strong>'
                + (f' {params_html}' if params_html else '')
            )
            ident = sig.get('identity', f'{name}.{sig["name"]}')
            src_mod = ident.rsplit('.', 1)[0] if '.' in ident and ident.rsplit('.', 1)[0] in all_sources else name
            item = info(ident, sig['name'], name)
            body_md = format_body(item)

            method_lis = []
            for method in sig['methods']:
                if not method.get('exports'):
                    continue
                total_methods += 1
                maid = alloc_anchor(used_ids, 'method', f'{sig["name"]}-{method["name"]}')
                pretty_m = ctf.pretty_scheme(method['scheme'])
                m_html = (
                    f'<strong class="tok-variable">{esc_html(method["name"])}</strong> :: '
                    f'{ctf.scheme_to_html(method["scheme"], type_links)}'
                )
                m_info = info(f'{ident}.{method["name"]}', method['name'], src_mod)
                m_line = m_info.get('line', 0) or item.get('line', 0)
                m_src = f'{STDLIB_BASE}/{esc_html(src_mod)}/source/' + (f'#L{m_line}' if m_line else '')
                m_doc = m_info.get('doc', '').strip()
                m_doc_html = f'<div class="doc-child-prose">{esc_html(summarize(m_doc))}</div>' if m_doc else ''
                method_lis.append(
                    f'<li id="{esc_html(maid)}"><div class="doc-method-row"><code>{m_html}</code>'
                    f'<span class="doc-sig-links"><a href="#{esc_html(maid)}" title="Permalink">#</a>'
                    f'<a href="{m_src}">Source</a></span></div>{m_doc_html}</li>'
                )
                synopsis_rows.append(
                    f'<li class="syn-sub"><a class="syn-name" href="#{esc_html(maid)}">{esc_html(method["name"])}</a>'
                    f'<code class="syn-sig">{m_html}</code></li>'
                )
                if name != 'Prelude':
                    search_entries.append({
                        'm': name,
                        'n': method['name'],
                        'k': 'm',
                        's': f'{method["name"]} :: {pretty_m}',
                        'd': summarize(m_doc) if m_doc else f'Method of {sig["name"]}.',
                        'h': maid,
                        'l': m_line or 0,
                        **({'sm': src_mod} if src_mod != name else {}),
                    })

            extra_parts = []
            if method_lis:
                extra_parts.append(f'<div class="doc-children"><h4>Methods</h4><ul>{"".join(method_lis)}</ul></div>')
            inst_html = render_instances_drawer(instances_by_class.get(sig['name'], []), type_links)
            if inst_html:
                extra_parts.append(inst_html)

            sigs_md.append(render_entry_block(aid, sig_html, body_md, src_mod, item.get('line', 0), '\n'.join(extra_parts)))
            synopsis_rows.insert(
                len(synopsis_rows) - len(method_lis),
                f'<li><a class="syn-name" href="#{esc_html(aid)}">{esc_html(sig["name"])}</a><code class="syn-sig">{sig_html}</code></li>',
            )
            if name != 'Prelude':
                search_entries.append({
                    'm': name,
                    'n': sig['name'],
                    'k': 'c',
                    's': plain_sig,
                    'd': summarize(item.get('doc', '')) if item.get('doc') else '',
                    'h': aid,
                    'l': item.get('line', 0) or 0,
                    **({'sm': src_mod} if src_mod != name else {}),
                })

        for value in export['values']:
            total_values += 1
            if name == 'Prelude':
                continue
            aid = alloc_anchor(used_ids, 'value', value['name'])
            symbol = value.get('symbol', f'{name}.{value["name"]}')
            src_mod = symbol.rsplit('.', 1)[0] if '.' in symbol and symbol.rsplit('.', 1)[0] in all_sources else name
            pretty_v = ctf.pretty_scheme(value['scheme'])
            sig_html = (
                f'<strong class="doc-decl-name tok-variable">{esc_html(value["name"])}</strong> :: '
                f'{ctf.scheme_to_html(value["scheme"], type_links)}'
            )
            item = info(symbol, value['name'], name)
            body_md = format_body(item)
            values_md.append(render_entry_block(aid, sig_html, body_md, src_mod, item.get('line', 0)))
            synopsis_rows.append(
                f'<li><a class="syn-name" href="#{esc_html(aid)}">{esc_html(value["name"])}</a>'
                f'<code class="syn-sig">{sig_html}</code></li>'
            )
            search_entries.append({
                'm': name,
                'n': value['name'],
                'k': 'v',
                's': f'{value["name"]} :: {pretty_v}',
                'd': summarize(item.get('doc', '')) if item.get('doc') else '',
                'h': aid,
                'l': item.get('line', 0) or 0,
                **({'sm': src_mod} if src_mod != name else {}),
            })

        # Emit collapsible Synopsis
        if synopsis_rows:
            open_attr = ' open' if len(synopsis_rows) <= 18 else ''
            lines += [
                f'<details class="doc-synopsis"{open_attr}><summary>Synopsis ({len(synopsis_rows)} declarations)</summary>',
                f'<ul class="doc-synopsis-list">{"".join(synopsis_rows)}</ul>',
                '</details>',
                '',
            ]

        if types_md:
            lines += ['# Types', ''] + types_md
        if sigs_md:
            lines += ['# Signatures', ''] + sigs_md
        if name == 'Prelude' and export['values']:
            lines += [
                '# Re-exported values',
                '',
                'Prelude re-exports a curated subset of the standard library. Follow the defining module link for documentation, or use [API Search](/packages/search/) to query by type signature. Compiler-installed forms such as `ioWrite` are documented in [compiler vocabulary](../reference/20-compiler-vocabulary.md).',
                '',
                '| Value | Signature | Defining module |',
                '| --- | --- | --- |',
            ]
            for value in export['values']:
                home = value.get('symbol', '').rsplit('.', 1)[0]
                link = f'[{home}]({home}.md#{anchor("value", value["name"])})' if home in all_sources else f'`{home}`'
                pretty_v = ctf.pretty_scheme(value['scheme'])
                lines.append(f'| `{markdown_text(value["name"])}` | `{markdown_text(pretty_v)}` | {link} |')
            lines += ['']
        elif values_md:
            lines += ['# Values', ''] + values_md

        if export['fixities']:
            lines += [
                '# Operator fixities',
                '',
                '| Operator | Associativity | Precedence |',
                '| --- | --- | --- |',
            ]
            for f in export['fixities']:
                assoc = {'left': 'infixl', 'right': 'infixr', 'none': 'infix'}.get(f.get('assoc'), 'infix')
                lines.append(f'| `{markdown_text(f.get("op", ""))}` | `{assoc}` | `{f.get("prec", 0)}` |')
            lines += ['']

        if export['instances']:
            lines += [
                '# Instances',
                '',
                'Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).',
                '',
                '<ul class="doc-module-instances">',
            ]
            for inst in export['instances']:
                iaid = alloc_anchor(used_ids, 'instance', inst['id'].rsplit('.', 1)[-1])
                plain_t, linked_t = ctf.link_instance_target(inst['target'], type_links)
                res = 'Implicit' if inst['implicit'] else 'Explicit'
                decl = doc_by_id.get(inst.get('id', ''))
                note_html = ''
                if decl and decl['doc']:
                    note_html = f'<div class="doc-child-prose">{esc_html(summarize(decl["doc"]["markdown"]))}</div>'
                lines.append(
                    f'<li id="{esc_html(iaid)}"><code>{linked_t}</code> '
                    f'<span class="doc-inst-meta">{res} · <code>{esc_html(inst["id"])}</code></span>{note_html}</li>'
                )
                if name != 'Prelude':
                    search_entries.append({
                        'm': name,
                        'n': inst['id'].rsplit('.', 1)[-1],
                        'k': 'i',
                        's': plain_t,
                        'd': summarize(decl['doc']['markdown']) if decl and decl['doc'] else f'{res} instance in {name}.',
                        'h': iaid,
                        'l': decl['source']['line'] if decl else 0,
                    })
            lines += ['</ul>', '']

        (OUTPUT / (name + '.md')).write_text(highlighted_markdown(compiler, '\n'.join(lines)))
        pub_sigs = sum(1 for s in export['signatures'] if public_class(s) or any(m.get('exports') for m in s['methods']))
        entries.append({
            'module': name,
            'family': name.split('.')[0],
            'description': describe(name),
            'path': str(path.relative_to(ROOT)),
            'sha256': digest(path),
            'imports': imports,
            'types': len(public_types),
            'signatures': pub_sigs,
            'values': len(export['values']),
            'public_methods': sum(bool(m.get('exports')) for s in export['signatures'] for m in s['methods']),
            'instances': len(export['instances']),
        })

    stdlib_pkg = json.loads((PRELUDE / '.cinchpkg').read_text())
    manifest = {
        'format': 3,
        'package': stdlib_pkg.get('name', 'cinch/stdlib'),
        'version': stdlib_pkg.get('version', '0.1.0'),
        'stdlib_manifest_sha256': digest(PRELUDE / '.cinchpkg'),
        'search_entry_count': len(search_entries),
        'modules': entries,
    }
    MANIFEST.write_text(json.dumps(manifest, indent=2) + '\n')

    search_payload = {
        'schema': 2,
        'package': 'cinch/stdlib',
        'moduleCount': len(entries),
        'typeLinks': type_links,
        'entries': search_entries,
    }
    SEARCH_INDEX.write_text(json.dumps(search_payload, separators=(',', ':')) + '\n')

    hoogle_widget = (
        f'<form class="doc-hoogle-box" action="{SEARCH_URL}" method="get" role="search">\n'
        '<label for="lib-index-search">Search by symbol name or type signature</label>\n'
        '<div class="doc-hoogle-row">\n'
        '<input id="lib-index-search" name="q" type="search" placeholder="e.g. (a -> b) -> List a -> List b, Result e a -> Option a, or +Data.Map" autocomplete="off" />\n'
        '<button type="submit">Search API</button>\n'
        '</div>\n'
        '<div class="doc-hoogle-chips">\n'
        f'<a href="{SEARCH_URL}?q=%28a+-%3E+b%29+-%3E+List+a+-%3E+List+b">(a -&gt; b) -&gt; List a -&gt; List b</a>\n'
        f'<a href="{SEARCH_URL}?q=Result+e+a+-%3E+Option+a">Result e a -&gt; Option a</a>\n'
        f'<a href="{SEARCH_URL}?q=Ord+a+%3D%3E+List+a+-%3E+List+a">Ord a =&gt; List a -&gt; List a</a>\n'
        f'<a href="{SEARCH_URL}?q=Bytes+-%3E+Result">Bytes -&gt; Result</a>\n'
        f'<a href="{SEARCH_URL}?q=is%3Aclass">is:class</a>\n'
        '</div>\n'
        '</form>'
    )

    index = [
        '# Standard library (`cinch/stdlib`)',
        '',
        f'Standard library API reference extracted from checked `.cinchif` compiler interfaces across **{len(entries)} modules** and **{len(search_entries):,} indexed declarations**. Published under [`/packages/stdlib/`](/packages/stdlib/) alongside the [package catalogue](/packages/).',
        '',
        hoogle_widget,
        '',
        '# Using `cinch/stdlib`',
        '',
        '- Every module in `cinch/stdlib` resolves directly through the toolchain package manifest (`import Data.Rational`, `import Text.Json`, `import Crypto.Aes`) and checks against its `.cinchif`/`.cpk` interface closure.',
        '- `Prelude` is implicitly in scope unless a package sets `"prelude": "none"` or names a custom prelude in `.cinchpkg`. Compiler-installed special forms live in [compiler vocabulary](../reference/20-compiler-vocabulary.md).',
        '- Inspect the resolved target surface with `cinch pkg info cinch/stdlib`, or link precompiled package objects via `cinch emit-object` and `--package-object` (see [packages, manifests, and the registry](../reference/29-package-management.md)).',
        '',
        '# Module directory',
        '',
    ]
    groups = ['Prelude', 'Algebra', 'Category', 'Relation', 'Function', 'Crypto', 'Data', 'Text', 'Effect', 'Image', 'Prim']
    for group in groups:
        subset = [e for e in entries if e['module'].split('.')[0] == group]
        if not subset:
            continue
        index += [f'## {group}', '', '| Module | Exports | Purpose |', '| --- | --- | --- |']
        for e in subset:
            name = e['module']
            count = e['types'] + e['values'] + e['public_methods']
            index.append(f'| [{name}]({name}.md) | {count} | {markdown_text(describe(name))} |')
        index += ['']

    index += [
        '# Regeneration and verification',
        '',
        'Every module page is generated from checked `.cinchif` interfaces and source comments, with clickable type links, cross-module instance indices, and line-numbered source views. Regenerate after changing `seed/prelude/` with `python3 website/scripts/generate-library-reference.py --compiler /path/to/cinch` and check for drift with `--check`.',
        '',
    ]
    (OUTPUT / '00-index.md').write_text('\n'.join(index))
    print(
        f'Generated {len(entries)} module pages, {total_values} value exports, '
        f'{total_methods} public methods, {len(search_entries)} Hoogle index entries'
    )


def main():
    args = argparse.ArgumentParser(description=__doc__)
    args.add_argument('--compiler')
    args.add_argument('--interfaces', type=Path, help='Use a previously checked interface directory from the same source snapshot')
    args.add_argument('--save-interfaces', type=Path, help='Persist extracted .cinchif files into this directory for fast regeneration')
    args.add_argument('--check', action='store_true')
    args.add_argument('--validate-docs', action='store_true', help='Parse documentation comments in every library source without regenerating pages')
    options = args.parse_args()
    if options.validate_docs:
        compiler = options.compiler or shutil.which('cinch')
        if not compiler:
            args.error('supply --compiler with a built cinch executable')
        compiler = str(Path(compiler).resolve())
        diagnostics = 0
        orphans = 0
        library_sources = sources()
        for name, path in library_sources.items():
            parsed = doc_json(compiler, path, name, allow_orphans=True)
            for item in parsed['diagnostics']:
                diagnostics += 1
                print(f'{path}:{item["line"]}: {item["code"]}: {item["message"]}')
            for item in parsed['orphans']:
                orphans += 1
                print(f'{path}:{item["line"]}: orphan {item["kind"]} doc: {item["text"][:80]}')
        print(f'Validated documentation comments in {len(library_sources)} module sources: {diagnostics} diagnostics, {orphans} orphan docs')
        return 1 if diagnostics or orphans else 0
    if options.check:
        recorded = json.loads(MANIFEST.read_text())
        actual = sources()
        assert set(actual) == {m['module'] for m in recorded['modules']}, 'Library source set changed; regenerate reference'
        assert recorded['stdlib_manifest_sha256'] == digest(PRELUDE / '.cinchpkg'), 'Stdlib manifest changed; regenerate reference'
        for entry in recorded['modules']:
            assert entry['sha256'] == digest(actual[entry['module']]), f'{entry["module"]}: source changed; regenerate reference'
            assert (OUTPUT / (entry['module'] + '.md')).exists(), f'Missing module page: {entry["module"]}'
            assert (ROOT / 'website/public/reference-sources' / (entry['module'] + '.cinch')).read_bytes() == actual[entry['module']].read_bytes(), f'Stale source download: {entry["module"]}'
        assert SEARCH_INDEX.exists(), 'Missing Hoogle search index: website/public/stdlib-search-index.json'
        print(f'Standard library (cinch/stdlib) reference matches all {len(actual)} module sources, stdlib manifest, and Hoogle search index')
        return
    compiler = options.compiler or shutil.which('cinch')
    if not compiler:
        args.error('supply --compiler to the Rust cinch executable')
    compiler = str(Path(compiler).resolve())
    temp = None
    if options.interfaces:
        ifaces = options.interfaces
    else:
        temp, ifaces = collect_interfaces(compiler, save_dir=options.save_interfaces)
    try:
        generate(ifaces, compiler)
    finally:
        if temp:
            temp.cleanup()


if __name__ == '__main__':
    raise SystemExit(main() or 0)
