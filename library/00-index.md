# Built-in and shipped libraries

The actual library source distributed with the **Rust compiler**, documented from checked compiler interfaces. This directory is separate from the website's [sample package catalogue](/packages/).

# Choose a module

Use site search to find a declaration by name or signature. The groups below distinguish embedded modules from source-distributed modules. Public types, constructors, values, methods, fixities, and instance entries come from `.cinchif` exports; source comments explain behaviour and named limits. Private class closure in an interface is not presented as a public method API.

# Load embedded modules

The compiler embeds the modules marked **Embedded**. Import their defining home when a name is absent from the curated Prelude. Compiler-installed special forms and operations such as `newIO`, `ioWrite`, `spawn`, and `decide` are documented in [compiler vocabulary](../reference/20-compiler-vocabulary.md).

# Use source-distributed modules

Modules marked **Ship source** are implemented and check with the Rust compiler, but are not embedded. Put their source and non-embedded dependency closure beside your entry file. For example, from the repository root:

```sh
mkdir -p /tmp/cinch-json-example
cp compiler/prelude/Text/Lex.cinch /tmp/cinch-json-example/Text.Lex.cinch
cp compiler/prelude/Text/Scan.cinch /tmp/cinch-json-example/Text.Scan.cinch
cp compiler/prelude/Text/Builder.cinch /tmp/cinch-json-example/Text.Builder.cinch
cp compiler/prelude/Text/Parse.cinch /tmp/cinch-json-example/Text.Parse.cinch
cp compiler/prelude/Text/Json.cinch /tmp/cinch-json-example/Text.Json.cinch
```

Save a `Main.cinch` there and use `import Text.Json`. Each module page lists its actual imports and the complete non-embedded source closure. The module declaration determines identity; the dotted filenames above make the sibling-directory arrangement unambiguous. These copy commands do not install a remote package.

<span class="support-tag support-tag--planned">Coming soon</span> a complete remote package-install/publish workflow and real binary release bundles. The current local package manifest/store is described in [modules](../reference/07-modules-and-packages.md) and [artifacts](../reference/18-artifacts-and-incremental.md).

# Module directory

## Prelude

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Prelude](Prelude.md) | Embedded | The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library. |

## Algebra

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Algebra.Arithmetic](Algebra.Arithmetic.md) | Embedded | Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements. |
| [Algebra.Bits](Algebra.Bits.md) | Embedded | Bit operations, shifts, and the corresponding operator/fixity vocabulary. |
| [Algebra.Group](Algebra.Group.md) | Embedded | Group-style inverse/subtraction vocabulary over admitted carrier instances. |
| [Algebra.Monoid](Algebra.Monoid.md) | Embedded | Identity plus combination vocabulary, default folding, and unit laws with explicit checking status. |
| [Algebra.Semigroup](Algebra.Semigroup.md) | Embedded | Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs. |
| [Algebra.Semiring](Algebra.Semiring.md) | Embedded | Zero/one and additive/multiplicative structure, with its law vocabulary. |

## Category

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Category.Alternative](Category.Alternative.md) | Embedded | Empty and choice operations for admitted higher-kinded containers. |
| [Category.Applicative](Category.Applicative.md) | Embedded | Container application and value lifting for independent structure-preserving composition. |
| [Category.Foldable](Category.Foldable.md) | Embedded | Generic folds, conversion to lists, size/emptiness, and related operations over container instances. |
| [Category.Functor](Category.Functor.md) | Embedded | Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism. |
| [Category.Monad](Category.Monad.md) | Embedded | Container-dependent composition through the admitted higher-kinded class vocabulary. |
| [Category.Traversable](Category.Traversable.md) | Embedded | Generic effect/container-aware traversal and sequencing while retaining the input shape. |

## Relation

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Relation.Eq](Relation.Eq.md) | Embedded | Equality/inequality class vocabulary. Instances live with their types. |
| [Relation.Ord](Relation.Ord.md) | Embedded | Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit. |

## Function

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Function.Combinators](Function.Combinators.md) | Embedded | Small reusable function operations such as identity, composition, argument manipulation, and application. |

## Data

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Data.Array.Packed](Data.Array.Packed.md) | Ship source | Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it. |
| [Data.Array](Data.Array.md) | Ship source | Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions. |
| [Data.Ascii](Data.Ascii.md) | Embedded | ASCII-refined values and checked construction at text boundaries. |
| [Data.BigDecimal](Data.BigDecimal.md) | Ship source | Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API. |
| [Data.Bool](Data.Bool.md) | Embedded | Boolean operations and their comparison instances. |
| [Data.Bytes](Data.Bytes.md) | Embedded | Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations. |
| [Data.Char](Data.Char.md) | Embedded | Unicode scalar values and checked character/code conversions, with range information for safe use. |
| [Data.Columnar](Data.Columnar.md) | Ship source | Columnar leaves over typed arrays, zipped into nested pairs that share their buffers. A length mismatch is None. |
| [Data.Complex](Data.Complex.md) | Ship source | Complex values and numeric operations using the module's current floating representation. |
| [Data.Compose](Data.Compose.md) | Embedded | Composition of higher-kinded containers with the admitted generic instances. |
| [Data.Const](Data.Const.md) | Embedded | A constant container that retains its first parameter while ignoring the mapped value parameter. |
| [Data.Core](Data.Core.md) | Embedded | Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles. |
| [Data.Date](Data.Date.md) | Ship source | Civil date values, validation, parsing/formatting, and calendar calculations. |
| [Data.Enum](Data.Enum.md) | Embedded | Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step. |
| [Data.FromList](Data.FromList.md) | Embedded | The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type. |
| [Data.FromMap](Data.FromMap.md) | Embedded | The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type. |
| [Data.Hash](Data.Hash.md) | Embedded | Hashing vocabulary and admitted hash implementations for keys. |
| [Data.HashMap](Data.HashMap.md) | Embedded | Hash maps with explicit key constraints, lookup/update, and map-style combination operations. |
| [Data.Identity](Data.Identity.md) | Embedded | The identity container and its generic abstraction instances. |
| [Data.Int](Data.Int.md) | Embedded | Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions. |
| [Data.IpAddr](Data.IpAddr.md) | Ship source | Validated IP addresses, parsing/rendering, and address-family vocabulary. |
| [Data.Json](Data.Json.md) | Embedded | The JSON value representation shared by parsers, renderers, codecs, and structured observability. |
| [Data.List](Data.List.md) | Embedded | Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances. |
| [Data.Machine](Data.Machine.md) | Embedded | Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit. |
| [Data.Map](Data.Map.md) | Embedded | Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support. |
| [Data.Matrix](Data.Matrix.md) | Ship source | Matrix construction, shape checks, indexing, transformation, and arithmetic operations. |
| [Data.Mutable.Array](Data.Mutable.Array.md) | Ship source | A fixed-length mutable array whose slots always hold values, with copying freeze and thaw against the immutable Array. |
| [Data.Mutable.BTree](Data.Mutable.BTree.md) | Ship source | An ordered mutable map: a degree-8 B-tree whose nodes are indices into one pooled set of slot arrays, with floor, ceiling, range folds, pop-min/max and a structural verifier. |
| [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md) | Ship source | An ordered mutable set: the pooled B-tree with a Unit value per member. |
| [Data.Mutable.BitSet](Data.Mutable.BitSet.md) | Ship source | A dynamic bitset in 64-bit words: test/set/toggle, popcount, next-set-bit scanning, and word-at-a-time union, intersection and difference. |
| [Data.Mutable.Deque](Data.Mutable.Deque.md) | Ship source | A double-ended queue as a power-of-two ring buffer: amortised O(1) push and pop at both ends and O(1) indexing. |
| [Data.Mutable.HashSet](Data.Mutable.HashSet.md) | Ship source | A mutable hash set: the HashTable with a Unit value per member. |
| [Data.Mutable.HashTable](Data.Mutable.HashTable.md) | Ship source | A mutable hash table after SwissTable and F14: 8-slot chunks with 7-bit tags matched eight at a time, a 64-bit finalizer over the Hash digest, and overflow counts that make deletion tombstone-free. |
| [Data.Mutable.Heap](Data.Mutable.Heap.md) | Ship source | A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list. |
| [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md) | Ship source | An insertion-ordered hash map: dense entries with a tagged chunk index over them, positional access, O(1) swap-remove and order-keeping shift-remove. |
| [Data.Mutable.LruCache](Data.Mutable.LruCache.md) | Ship source | A bounded least-recently-used cache: a hash table to nodes threaded on a recency list, with O(1) get, put and eviction. |
| [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md) | Ship source | Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements. |
| [Data.Mutable.Var](Data.Mutable.Var.md) | Ship source | A single mutable cell, written in place; not synchronized (TVar is the shared cell). |
| [Data.Mutable.Vec](Data.Mutable.Vec.md) | Ship source | A growable vector: amortised O(1) push and pop, O(1) indexing, middle insertion and removal, in-place introsort, and copying freeze/thaw against Array. |
| [Data.Mutable](Data.Mutable.md) | Ship source | One import for the mutable containers. Every container is indexed by a region r and every operation carries the Mut r label, a capability like IO r, so purity stays a checked property. |
| [Data.Nat](Data.Nat.md) | Embedded | Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions. |
| [Data.NonEmpty](Data.NonEmpty.md) | Embedded | Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate. |
| [Data.NonZero](Data.NonZero.md) | Embedded | Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors. |
| [Data.Option](Data.Option.md) | Embedded | Optional values with None and Some, plus combinators for propagating or consuming absence. |
| [Data.Ordering](Data.Ordering.md) | Embedded | The LT, EQ, and GT comparison result vocabulary and its instances. |
| [Data.Path](Data.Path.md) | Embedded | Platform path values and composition/conversion operations used by filesystem and process APIs. |
| [Data.Producer](Data.Producer.md) | Embedded | Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive. |
| [Data.Rational](Data.Rational.md) | Ship source | Rational numbers with explicit denominator/error policy and arithmetic operations. |
| [Data.Result](Data.Result.md) | Embedded | Success and named failure values, error-aware transformations, partitioning, and sequencing instances. |
| [Data.Set](Data.Set.md) | Embedded | Ordered sets and their membership, combination, difference, and conversion operations. |
| [Data.Sorted](Data.Sorted.md) | Embedded | Lists refined by sortedness, with construction and combination that preserve ordering. |
| [Data.String](Data.String.md) | Embedded | Immutable UTF-8 text, encoding/decoding, character access, slicing, comparison, and concatenation. Byte conversions copy to preserve immutability. |
| [Data.StringIo](Data.StringIo.md) | Ship source | In-memory string-oriented I/O utilities with explicit state and error results. |
| [Data.Sym](Data.Sym.md) | Embedded | Interned symbols with identity equality, deterministic spelling order, and String conversions. |
| [Data.Target](Data.Target.md) | Embedded | Target/platform and worker vocabulary used by the compiler-supported runtime and target examples. |
| [Data.Tensor](Data.Tensor.md) | Ship source | Immutable row-major tensors with checked runtime shapes of any rank. |
| [Data.Time](Data.Time.md) | Ship source | Time values, durations, and conversions with explicit units and admitted ranges. |

## Text

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Text.Args](Text.Args.md) | Ship source | Command-line argument processing and its named value/error vocabulary. |
| [Text.Base64](Text.Base64.md) | Ship source | Base64 encoding/decoding over bytes with explicit invalid-input results. |
| [Text.Builder](Text.Builder.md) | Ship source | An efficient text-construction vocabulary for assembling many fragments before rendering. |
| [Text.Codec](Text.Codec.md) | Embedded | Encoding/decoding signatures and codec helpers with explicit failure results. |
| [Text.Csv](Text.Csv.md) | Ship source | CSV parsing/rendering with explicit field, record, and malformed-input handling. |
| [Text.Digest](Text.Digest.md) | Ship source | Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol. |
| [Text.Json](Text.Json.md) | Ship source | JSON parsing and rendering over Data.Json, with explicit errors and builder-based output. |
| [Text.Lex](Text.Lex.md) | Ship source | A source-distributed lexer toolkit: tokens, spans, and lexical errors for compiler/parser clients. |
| [Text.OptionParser](Text.OptionParser.md) | Ship source | Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors. |
| [Text.Otlp](Text.Otlp.md) | Ship source | OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers. |
| [Text.Parse](Text.Parse.md) | Ship source | A source-distributed parser toolkit over the lexer/scanner vocabulary. |
| [Text.Regex](Text.Regex.md) | Ship source | Regular-expression parsing/matching and replacement under the module's admitted regex language. |
| [Text.Scan](Text.Scan.md) | Ship source | Scanning primitives over input text and the shared position/error vocabulary. |
| [Text.Shellwords](Text.Shellwords.md) | Ship source | Shell-style word splitting/joining; this is data parsing, not execution of a shell command. |
| [Text.Show](Text.Show.md) | Embedded | Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself. |
| [Text.Toml](Text.Toml.md) | Ship source | The module's supported TOML parsing/rendering surface and explicit unsupported/malformed cases. |
| [Text.Uri](Text.Uri.md) | Ship source | URI parsing, rendering, components, and validation/error vocabulary. |
| [Text.Yaml](Text.Yaml.md) | Ship source | The module's supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature. |
| [Text.Zlib](Text.Zlib.md) | Ship source | Compression/decompression operations and named malformed/unsupported-stream results. |

## Effect

| Module | Distribution | Purpose |
| --- | --- | --- |
| [Effect.Benchmark](Effect.Benchmark.md) | Ship source | Benchmark case execution and timing/reporting helpers over runtime clocks. |
| [Effect.Clock](Effect.Clock.md) | Embedded | Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs. |
| [Effect.Concurrent](Effect.Concurrent.md) | Embedded | Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations. |
| [Effect.Coverage](Effect.Coverage.md) | Ship source | Coverage collection/reporting utilities with explicit runtime and source metadata. |
| [Effect.FileUtils](Effect.FileUtils.md) | Ship source | Convenience filesystem operations built on the lower-level filesystem API. |
| [Effect.Find](Effect.Find.md) | Ship source | Filesystem walking/search utilities with the module's traversal and error policy. |
| [Effect.Fs](Effect.Fs.md) | Embedded | Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors. |
| [Effect.Http](Effect.Http.md) | Ship source | A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation. |
| [Effect.Io](Effect.Io.md) | Embedded | A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration. |
| [Effect.Log](Effect.Log.md) | Ship source | Structured log operations and records with runtime capability/effect accounting. |
| [Effect.Net](Effect.Net.md) | Ship source | Socket operations with explicit handles, address values, errors, and runtime effects. |
| [Effect.Process](Effect.Process.md) | Embedded | One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process's arguments. |
| [Effect.Pty](Effect.Pty.md) | Ship source | Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations. |
| [Effect.Random](Effect.Random.md) | Ship source | Runtime entropy/random utilities with explicit effectful access. |
| [Effect.Resolv](Effect.Resolv.md) | Ship source | Hostname/address resolution through the runtime, with named failure results. |
| [Effect.Sequencing](Effect.Sequencing.md) | Embedded | The direct, Result, and Option do providers. One lexical provider/container is selected per definition. |
| [Effect.Signal](Effect.Signal.md) | Ship source | Operating-system signal vocabulary and runtime operations with explicit error/effect handling. |
| [Effect.Stm.TBQueue](Effect.Stm.TBQueue.md) | Embedded | A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty. |
| [Effect.Stm.TMVar](Effect.Stm.TMVar.md) | Embedded | A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions. |
| [Effect.Stm.TQueue](Effect.Stm.TQueue.md) | Embedded | An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables. |
| [Effect.Stm.TSem](Effect.Stm.TSem.md) | Embedded | A counting semaphore for transactions: wait takes a permit or retries, signal returns one. |
| [Effect.Stm](Effect.Stm.md) | Embedded | Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR. |
| [Effect.Test](Effect.Test.md) | Ship source | A subprocess-based test runner and named test-result/report vocabulary. |
| [Effect.Trace](Effect.Trace.md) | Ship source | Scoped tracing/span vocabulary and context propagation tied to the runtime identity. |

# Scope and verification

This generation covers **120 source modules**: **64 embedded** and **56 source-distributed**. Each source-distributed module was imported in an isolated consumer project and checked by the Rust compiler before extraction. Checking an import establishes accepted declarations; it is not an end-to-end runtime test of every function. Runnable tutorial examples are verified separately.

Signatures preserve the compiler's rendered schemes, including constraints and effect rows. When a source comment and a checked scheme differ, treat the scheme and Rust behaviour as current and investigate the source note. Availability of a module does not imply every feature of another language's similarly named library (for example, plaintext HTTP is not TLS, and a YAML parser may support a subset).

Regenerate with `python3 website/scripts/generate-library-reference.py --compiler /path/to/cinch`. Check source drift with `python3 website/scripts/generate-library-reference.py --check`.
