# Official packages

Status: proposal for review, 2026-11-09. This is the catalogue and the rules that make a package "official", not a schedule. The base-library plan it builds on is [the base as a package](base-as-a-package.md): a minimal embedded primitive layer, everything else resolved as packages, a prelude that is an ordinary injected module, and manifests that describe their platform conditions and native requirements.

The existing surface is not a starting point to re-do. The current base family already ships filesystem, process, pseudo-terminal, signal, clock, random, console, logging, HTTP client, name resolution, and text formats with real implementations, as `Effect.*` and `Text.*` modules. Under the base plan those become `cinch/effect` and `cinch/text` unchanged in behaviour, and only `cinch/prim` stays embedded. The catalogue below is what does not exist yet, ordered by what unblocks writing real programs.

# 1 What "official" means

A package in the `cinch/` namespace, published from this repository, with:

- **A named owner and a review rule.** Every change is reviewed by someone who did not write it. A package that binds native code needs a second reviewer for the link declaration alone.
- **Documentation that is executed.** Every documented example carries a `cinch:run`, `cinch:check`, or `cinch:reject` marker and is checked by the same harness the guide uses. A package page with an unverified example is a bug.
- **A target matrix it actually passes.** `cinch pkg verify --matrix` runs the checker and the example suite for every target the manifest declares. A declared target that fails is a release blocker, not a note in the README.
- **Semantic versioning of the interface.** `0.y` while the language is pre-1.0, `x.0` after. The rule is the one the language itself needs: a breaking change to a public scheme, a public type's constructors, an instance's existence, or an effect row is a major bump; a new name is a minor bump. The `.cinchif` interface hash is the mechanical check.
- **A deprecation path.** A name is deprecated in a minor release with a doc note and a `#[deprecated]` style marker once the language has one, and removed in the next major. No silent removals.
- **Provenance.** Published from CI with the L1 attestation the registry already records, and a signed tag at the source. A binary release, if we ever ship one, states the compiler version and the runtime ABI it was built with.
- **A reserved root.** `Unix.*`, `Net.*`, `Crypto.*`, `Sql.*`, `Term.*`, `Build.*`, and `Test.*` are reserved for official packages, and the compiler refuses a third-party package that declares a module under one of them. Without that, someone else takes `Unix.Stat` first and the official package has nowhere to live.

# 2 First-party rule: primitives in the runtime, policy in packages

A package requests operating-system capability through a `cinch.runtime.*` primitive implemented in this repository's runtime, never through a native library it links. That keeps the first-party stack free of the native-trust problem and keeps one auditable place for kernel access.

The native tiers of [base-as-a-package](base-as-a-package.md) section 4.3 exist for what cannot follow that rule (SQLite's C library, a platform TLS stack). No package in this catalogue needs them at P0 or P1. Where a row below names one, it is a deliberate exception with a stated reason.

# 3 Runtime primitives these packages need

Grouped by the packages that want them, in the order worth building. Each is a `cinch.runtime.*` unit plus a wrapper module, in the house style: pure at the boundary, failures as named values in `Result`, effects on the wrapper.

| Primitive group | Calls | Serves |
| --- | --- | --- |
| Socket listen and accept | `socket`, `bind`, `listen`, `accept4`, `setsockopt`, `getsockopt`, `shutdown` | `Net.Tcp` |
| Datagram and local sockets | `sendto`, `recvfrom`, `socketpair` | `Net.Udp`, `Net.Unix` |
| Readiness | `poll`, `epoll_create1`/`epoll_ctl`/`epoll_wait`, `kqueue`/`kevent`, IOCP on Windows | `Net.Event`, and every server package |
| File metadata and permissions | `statat`, `fstatat`, `fchmodat`, `fchownat`, `readlinkat` | `Unix.Stat`, `Unix.Perm`, `Unix.User` |
| Directory sync and atomic replace | `fsync`, `fdatasync`, `renameat`, `linkat` | `Unix.Atom` |
| Advisory locks | `flock`, `fcntl(F_SETLK)` | `Unix.Lock` |
| Mapping and advice | `mmap`, `munmap`, `madvise`, `msync` | `Unix.Mmap` |
| Watchers | `inotify_init1`, `inotify_add_watch`, `kqueue`/`kevent` | `Unix.Watch` |
| Process identity and limits | `getuid`, `geteuid`, `getgid`, `getrlimit`, `setrlimit`, `umask`, `uname` | `Unix.User`, `Unix.Limit`, `Unix.Sys` |
| Entropy | `getrandom`, `arc4random_buf`, `BCryptGenRandom` | `Crypto.Random`, every key API |
| Terminal attributes | `tcgetattr`, `tcsetattr`, `ioctl(TIOCGWINSZ)` | `Term.Tui` |
| Hashing and AEAD hot loops | our own implementations behind thin primitives | `Crypto.*` |

Everything above is small, testable, and already has a shape in `compiler/runtime/src/platform.rs` or `net.rs` to copy.

# 4 The catalogue

Priority is P0 (unblocks writing real programs this quarter), P1 (next), P2 (later, or needs a decision). "Cd" is Cinch only, "RT" needs a runtime primitive first.

## 4.1 Unix and filesystem

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Unix.Stat`, `Unix.Perm` | P0 | RT: statat | `file-stat :: Path ->{IO r} Result FsError Stat`, with `size`, `kind`, `mtime`, `mode`; permission bits as a refinement, `Perm` with `Perm.readable`, `Perm.writable`, `Perm.executable` projections, and `Perm.set` refusing a value outside `0o777` |
| `Unix.Atom` | P0 | RT: fsync, renameat | `write-atomically :: Path -> Bytes ->{IO r} Result FsError Unit` (temp file, `fsync`, `rename`, `fsync` the directory), and `sync-file`, `sync-dir`. This is the durability answer the current `fs-write` does not give |
| `Unix.Lock` | P0 | RT: flock | `with-lock :: Path ->{IO r, Cancel r} (Result FsError a) -> ...`, exclusive and shared, with the refusal when the path is on a filesystem that cannot lock |
| `Unix.Sys` | P1 | RT: uname, sysconf | `kernel-name`, `release`, `machine`, `cpu-count`, `page-size`, `tmp-dir` |
| `Unix.User` | P1 | RT: getuid, passwd lookup | `User { uid, name, home, shell }`, `current-user`, `user-by-uid`, `groups` |
| `Unix.Limit` | P1 | RT: rlimit | `Limit` enum, `get`, `set`, and the refusal for a limit the process cannot raise |
| `Unix.Mmap` | P1 | RT: mmap | `map-read :: Path ->{IO r} Result MapError (Mapped r)`, `bytes-of :: Mapped r -> Bytes`, unmapping tied to the region, and the length refinement `n < mapped-length` on reads |
| `Unix.Watch` | P1 | RT: inotify, kevent | `watch :: Path ->{IO r, Cancel r} Result WatchError Watcher`, `next-event :: Watcher ->{IO r, Cancel r} Result WatchError Event`, `Event` with `created`, `modified`, `removed`, `renamed`. Exposed platform modules: `Unix.Watch.Inotify` (Linux, with watch descriptors and mask control) and `Unix.Watch.Kevent` (Darwin, with filters and per-event flags) |

The pattern for the platform modules is the one section 4.2 of the base note describes: `Unix.Watch` is portable and carries the common subset; the platform module is `exposed: true` and available only where its condition holds, so a project that imports it has spent its portability on purpose.

## 4.2 Networking

The base has TCP connect, read, write, close, and one-shot name resolution. Servers, datagrams, and readiness are missing, and they are the gap that keeps the language off anything but a client.

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Net.Tcp` | P0 | RT: socket, bind, listen, accept, options | `listen :: Addr ->{IO r} Result NetError Listener`, `accept :: Listener ->{IO r, Cancel r} Result NetError (Connection, Addr)`, `connect`, `read`, `write`, `close`, `set-timeout`, `set-nodelay`, `keepalive`, plus `Addr` over the `ip-parse` and `ip-to-string` the base already has for both families |
| `Net.Event` | P0 | RT: poll, epoll, kqueue, IOCP | `Selector`, `register :: Selector -> Handle ->{IO r} Result NetError Interest`, `wait :: Selector -> Nat ->{IO r, Cancel r} Result NetError (List Ready)`, with `Interest.read`, `Interest.write`, `Interest.both`. This is what makes a server thread-per-connection only by choice |
| `Net.Udp` | P1 | RT: sendto, recvfrom | `bind`, `send-to`, `recv-from :: Socket -> Nat ->{IO r} Result NetError (Bytes, Addr)` |
| `Net.Unix` | P1 | RT: socketpair | Local sockets for the same three operations, plus `pair` for a connected duo |
| `Net.Http.Server` | P0 after `Net.Tcp` and `Net.Event` | Cd | `serve :: Config -> Route ->{IO r, Cancel r} Result HttpError Unit`, `Route` as a request-to-response function, keep-alive from `Config`, request body limits as a refinement, and the same `Request`/`Response` types `Effect.Http` uses as a client |
| `Net.WebSocket` | P1 | Cd | `upgrade :: Request ->{IO r} Result HttpError Socket`, frames with the text and binary split, and close codes as a named type |
| `Net.Http2` | P2 | Cd | Header compression and multiplexed streams over one connection; worth doing only once a server handles real traffic |
| `Net.Tls` | P1, after `Crypto.*` | RT: entropy; Cd: everything else | `ClientConfig`, `ServerConfig`, `handshake :: Socket -> Config ->{IO r, Cancel r} Result TlsError Session`, `read`/`write` over the session, ALPN list, and a refusal to negotiate below TLS 1.3. Implemented over `Crypto.*`, not over a linked library, so the first-party rule holds and a certificate chain is verified by code in this repository |

## 4.3 Crypto

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Crypto.Random` | P0 | RT: getrandom | `bytes :: Nat ->{IO r, Diverge} Bytes`, `nat-below`, and the refusal when the platform source fails instead of a silent fallback |
| `Crypto.Hash` | P0 | RT: hot loops | `Sha256`, `Sha512`, `Sha3_256`, `Blake3` with `hash :: Bytes -> Digest` and an incremental `Context` (`init`, `update`, `finish`), plus `Crypto.Digest` render and parse in hex and base64 through the existing `Text.Base64` |
| `Crypto.Mac` | P0 | RT | `hmac :: Digest -> Key -> Bytes -> Bytes`, `verify :: Bytes -> Bytes -> Bool` in constant time |
| `Crypto.Aead` | P1 | RT | `ChaCha20Poly1305` and `AesGcm` with `seal`/`open`, an explicit nonce length in the type, and the refusal when a nonce repeats in one key's stream |
| `Crypto.Kdf` | P1 | RT | `hkdf`, `pbkdf2`, `argon2id` with cost parameters as refinements |
| `Crypto.Sign` | P2 | RT | Ed25519 `sign`/`verify`, and ECDSA P-256 over `Crypto.Hash` |

The hot loops live in the runtime for constant-time discipline and speed; the API, the encodings, and the policy are Cinch. A claimed security property gets a test with a known vector, and the vector files ship with the package.

## 4.4 Data formats

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Text.Gzip` | P1 | Cd | The gzip container over what `Text.Zlib` already implements (`deflate`, `inflate`, `crc32`, `adler32`), so the work is framing rather than compression |
| `Text.Tar`, `Text.Zip` | P1 | Cd | Archive read and write with a path-safety rule (no absolute paths, no `..` after normalisation), and per-entry size limits |
| `Text.Cbor`, `Text.MsgPack` | P2 | Cd | Binary variants of the encoding style `Data.Json` already uses |
| `Time.Zone` | P1 | Cd | `load :: Path -> Result ZoneError Zone`, `offset-at :: Zone -> Instant -> Offset`, TZif v2 and v3 parsing, and DST boundaries as a fold. The civil side exists already: `Data.Date` carries `to-epoch-days`, `from-epoch-days`, `add-days`, and `day-of-week`, and `Effect.Clock` gives wall nanoseconds, so the package is zone data plus the instant-to-local conversion |

## 4.5 Build and storage

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Build.Manifest`, `Build.Lock` | P0 | Cd | Typed readers and writers for `.cinchpkg` and `.cinchlock`, including the `select` conditions, so tooling (the registry, CI, editors) does not reimplement the manifest in JavaScript. This is also the package that makes the manifest schema testable in the language it belongs to |
| `Sql.Sqlite` | P2 | RT: mmap, flock, fsync; or `link` for `libsqlite3` | A single-file relational store with a page cache, plus `query`/`exec` over a prepared statement type. P2 because binding the C library needs the native-tier decision, and writing the btree is a project rather than a package |
| `Sql.Postgres` | P2 after `Net.Tls` | Cd | The wire protocol client: `connect`, `query`, `prepare`, `bind`, parameter types as a refinement over the wire format, and notices surfaced as values |

## 4.6 Terminal, test, and tooling

| Package | Priority | Needs | What it carries |
| --- | --- | --- | --- |
| `Term.Tui` | P1 | RT: termios, ioctl, read | Alternate screen, raw mode with a restore-on-exit guarantee, `Key` decoding including modifiers and escape sequences, `size :: Unit ->{IO r} (Nat, Nat)`, cursor addressing, and style as a value rather than a string. The REPL's frontend becomes the first consumer, which is the honest test of the API |
| `Test.Property` | P2 | Cd | Generators over `Data.Enum` and the primitive types, shrinking with a deterministic seed in the report, and the same `cinch doc test` integration the current `Effect.Test` has |
| `Test.Golden` | P2 | Cd | Filesystem-backed expectation files with an update mode that is refused in CI |

# 5 Non-goals

Worth stating, because each is a tempting first package:

- **No catch-all `Unix` module.** A module that mixes paths, users, limits, and mmap cannot state a narrow capability, and its imports tell a reader nothing.
- **No C++ bindings.** The FFI is C-shaped by design; wrapping a C++ library would need an ABI of its own.
- **No package that hides a second runtime.** A package that embeds its own event loop, allocator, or thread pool conflicts with the runtime's guarantees, and the choice should be visible at the call site.
- **No framework wrappers in the P0 or P1 tiers.** Cocoa, Win32 UI, and the like are large, unstable, and not where the language earns its keep yet.

# 6 Sequencing

1. `Unix.Stat`, `Unix.Atom`, `Unix.Lock`, `Net.Tcp`, `Net.Event`, `Crypto.Random`, `Crypto.Hash`, `Build.Manifest`, `Build.Lock`. That set makes file-heavy programs, servers, and package tooling possible, and every one of them is testable with the marker harness.
2. `Term.Tui`, `Unix.Watch`, `Net.Http.Server`, `Text.Gzip`, `Text.Tar`, `Time.Zone`, `Crypto.Mac`, `Crypto.Aead`.
3. `Net.Tls` over `Crypto.*`, then `Net.WebSocket` and `Sql.Postgres`, which need it.
4. The P2 remainder, each gated on the decision it depends on (native tier, IOCP port, HTTP/2 appetite).

Every step lands with its package page, its verified examples, and its target matrix in the same release. A package that ships without those is a prototype, and the registry should be able to tell them apart by namespace or by a status field in the manifest.
