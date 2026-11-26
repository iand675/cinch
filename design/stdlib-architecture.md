# Standard library architecture

Cinch's standard library lives under `seed/prelude/` (331 `.cinch` modules across `Algebra`, `Category`, `Crypto`, `Data`, `Effect`, `Function`, `Image`, `Prim`, `Relation`, and `Text`). Checked API documentation is generated from compiled `.cinchif` interfaces into [`library/`](../library/00-index.md) and published with Hoogle-style type search at `/packages/stdlib/`.

---

## 1. Organization and embedded boundary

- **Embedded base (`seed/prelude/cinch-base.cinchpkg`):** 70 foundational modules embedded directly into the compiler (`Prelude`, core data types, algebraic/category classes, basic text/bytes/effects, and platform facades `Prim.Sys.*`).
- **Extended standard library (`seed/prelude/`):** 261 additional modules compiled on demand when imported by consumer projects or packaged into library objects, covering cryptography (`Crypto.*`, including AES, ChaCha20-Poly1305, Ed25519, P-256/ECDSA, X25519, HPKE, RSA, TLS 1.3, X.509, ML-KEM, ML-DSA), networking and protocols (`Effect.Http`, `Effect.Net`, `Effect.Tls`, `Effect.Smtp`, `Effect.Rpc`), formats (`Text.Json`, `Text.Toml`, `Text.Yaml`, `Text.Xml`, `Text.Csv`, `Text.Html`, `Text.Template`, `Text.Url`, `Text.Regex`), compression (`Text.Flate`, `Text.Gzip`, `Text.Zlib`, `Text.Bzip2`, `Text.Tar`, `Text.Zip`), image codecs (`Image.Png`, `Image.Jpeg`, `Image.Gif`, `Image.Draw`), datetime, and mutable containers.
- **Pure Cinch first:** cryptographic primitives, parsers, compression, and codecs are written in pure Cinch wherever possible, reserving inline LLVM `prim`s strictly for SIMD/atomic/hardware kernels that cannot be expressed portably.

---

## 2. Arrays and mutable containers

### Immutable arrays, tensors, and columnar tables
- **`Data.Array` / `Data.Array.Packed`:** native contiguous arrays backed by runtime array blocks, supporting bounds-checked indexing, slicing, binary search (`Data.Array.Search`), edits (`Data.Array.Edit`), and copy-on-write growable builders (`ArrayList`).
- **`Data.Tensor` / `Data.Array.Compute`:** multi-dimensional tensors and zero-copy shape-indexed views (`View1`, `View2`, `View3`) powering parallel CPU and Metal GPU pipelines (see [`effects-and-concurrency.md`](effects-and-concurrency.md)).
- **`Data.Columnar`:** structure-of-arrays (SoA) columnar storage for cache-friendly bulk processing.

### Scoped mutable containers (`Data.Mutable.*`)
Mutable data structures are built over a small trusted region-scoped slot core (`Data.Mutable.Var`, `Data.Mutable.Array`) so mutation is lexically scoped, cannot escape its region, and respects reference-counting mark-on-publish invariants:
- Sequence & buffer structures: `Data.Mutable.Vec`, `Data.Mutable.Deque`, `Data.Mutable.Ring`, `Data.Mutable.LinkedList`, `Data.Mutable.BitSet`.
- Associative & priority structures: `Data.Mutable.Map`, `Data.Mutable.HashTable`, `Data.Mutable.HashSet`, `Data.Mutable.IndexMap`, `Data.Mutable.BTree`, `Data.Mutable.BTreeSet`, `Data.Mutable.Heap`, `Data.Mutable.HeapView`, `Data.Mutable.LruCache`, `Data.Mutable.UnionFind`.

---

## 3. Datetime (`Data.Time.*`, `Text.Time.*`, `Effect.Time.*`)

Cinch separates machine time from civil calendar time in four clean layers:
1. **Machine time:** `Data.Time.Duration`, `Data.Time.Monotonic`, `Data.Time.Timestamp` (instant on a continuous timeline).
2. **Civil time & spans:** `Data.Time.Date`, `Data.Time.TimeOfDay`, `Data.Time.DateTime`, `Data.Time.Weekday`, `Data.Time.Unit`, `Data.Time.Span`, `Data.Time.Offset`.
3. **Time zones & zoned arithmetic:** `Data.Time.Zone`, `Data.Time.Zone.Tzif` (TZif binary database reader), `Data.Time.Zone.Posix`, `Data.Time.Zoned`, `Data.Time.Zoned.Arithmetic`, and `Effect.Time` / `Effect.Time.Zone` for system clock and local zone lookup.
4. **Formatting & parsing:** `Text.Time.Iso`, `Text.Time.Rfc2822`, `Text.Time.Rfc9557`, `Text.Time.Duration`, `Text.Time.Strtime`, `Text.Time.Zoned`, `Text.Time.Scan`.

---

## 4. JSON representation (`Text.Json.*`)

JSON parsing and encoding are layered so callers only pay for the fidelity they need:
- **`Text.Json.Value` / `Text.Json.Dynamic`:** low-allocation parsed value trees and dynamic accessors.
- **`Text.Json.Codec`:** typed encoding/decoding codecs.
- **`Text.Json.Binary32`:** compact binary/indexed representation for fast loading of large JSON documents (such as compiled interfaces and manifests).

---

## 5. Standard library parity tracking

Cinch uses Go's standard library (180 public packages) as a systematic coverage checklist rather than an ad-hoc wish list. Machine-readable coverage status is maintained in [`design/stdlib-surface.json`](stdlib-surface.json) and audited with:

```sh
python3 tooling/audit_stdlib_surface.py --go /path/to/go --check
python3 tooling/audit_stdlib_surface.py --go /path/to/go --package io
```

Every package entry distinguishes `complete`, `partial`, and `unreviewed` coverage along with remaining gaps. Performance regressions across codecs, hashes, byte search, and streaming pipelines are tracked via `tooling/bench_stdlib.py` (see [`benchmarks/stdlib/README.md`](../benchmarks/stdlib/README.md)).
