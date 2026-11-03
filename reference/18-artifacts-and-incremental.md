# Artifacts, interfaces, stores, and incremental compilation

The compiler distinguishes the identity of a public contract from the identity of the implementation/artifact that realizes it. This supports checked separate metadata and incremental work without letting caches become a source of truth.

# Interface contents

A `.cinchif` file records module identity, source hash, imports and checked-against dependency facts, exports, schemes/kinds, constructor identities, relevant private type/class closure, method visibility, instance metadata, ABI layouts/symbols, contracts, laws, assumptions, and upgrades. The current schema is versioned; readers must validate the format and dependencies rather than deserialize a few convenient fields and assume the rest.

The contract projection excludes erasable hints and purely representational selection where required. It includes facts callers can rely on, such as semantic contracts and necessary schema/ABI information. The artifact manifest additionally records selected primitive bodies, target features, optimizer reports, captured stage inputs, and other implementation evidence.

Schema numbers are implementation details, not a claim that an old cached compiler can read every newer interface. A native Rust interface and a partially implemented self-hosted loader can support different subsets of a versioned schema.

# Package store

`CINCH_STORE` overrides the store root. On macOS the default is `~/Library/Caches/cinch/store`; other platforms use `XDG_CACHE_HOME/cinch/store` when set, otherwise `~/.cache/cinch/store`. Entries are keyed by content and compiler/runtime/profile identities and are verified before reuse.

A malformed or mismatched entry is a diagnostic. `--rebuild` permits recomputing that entry. The cache is advisory: a corrupt entry must not change program semantics or substitute an unchecked interface. Store keys include the base manifest/source witnesses, so changing embedded family bytes cannot reuse an unrelated artifact merely because the package's human version string stayed the same.

# Incremental APIs

The Rust implementation has item dependency graphs, persistent sessions, and object/artifact caching. Reachability and changed contract/body identities determine what needs rechecking or recompilation. The analysis layer exposes rechecked/kept reports for editors and REPL APIs. Current source tests exercise invalidation and keep/rebuild decisions; latency goals in the design are not benchmark results.

`cinch build` remains the documented CLI. There is no current watch/session daemon command. An editor frontend using the analysis API is not evidence that the CLI automatically maintains a persistent compiler process between shell invocations.

# Staging and identity

Captured file/subprocess/schema inputs, converter identity, and assumptions belong in provenance. Credentials are not public build data. Refresh/replay/live staging modes choose how inputs are observed; replay must not silently fall back to ambient live state. A source edit to a private helper can change an implementation artifact while leaving the public contract unchanged, provided no caller-visible fact changes.

# Investigating stale work

Run `check` first, inspect the named module/dependency in the diagnostic, and compare its source/contract witness. Use `--rebuild` for a failing verified store entry rather than manually editing `.cinchif` fields. For compiler development, use isolated source/target/store directories when testing a snapshot: embedded source paths in cached test binaries can otherwise refer to another checkout.

<span class="support-tag support-tag--planned">Coming soon</span> **complete self-hosted parity.** The Cinch-written frontend and bootstrap ladder are still progressing towards body checking, lowering, and self-build parity. Their latest status is in [self-hosting](../design/self-hosting.md). Current language support in this guide follows the Rust implementation, as requested.

Implementation: `driver.rs`, `incremental.rs`, `interface.rs`, `check_interface.rs`, `store.rs`, `analysis.rs`; suites: `incremental.rs`, `package_store.rs`, `canonical_artifacts.rs`, `interface_*`, `bootstrap_*`.
