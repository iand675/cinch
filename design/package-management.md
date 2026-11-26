# Package management and registry architecture

Cinch packages identify releases by cryptographic content hash and check compatibility directly on compiled interfaces rather than relying on version numbers as a social promise.

User-facing CLI documentation lives in [`reference/29-package-management.md`](../reference/29-package-management.md). The runnable registry prototype lives in [`registry/README.md`](../registry/README.md).

---

## 1. Core thesis: content identity and checked compatibility

### Strong identities
Local build caches use 64-bit FNV-1a keys (`iface_hash`, `package_key`) suitable for local bit-rot checks. Distribution, locking, and provenance use cryptographic SHA-256 identities:

```text
source tree      sorted (path, sha256(bytes)) entries (no timestamps, no symlinks)
package_id       sha256("package\n" + canonicalJson(manifest) + "\n" + source tree)
contract_digest  sha256 over target-independent module contract projection
iface_digest(P)  sha256 over full canonical interface for target profile P
artifact_id      sha256("artifact" | package_id | compiler | profile | adapters |
                        policy | module -> iface_digest | body digests)
```

Because the manifest is excluded from the file list before hashing, `package_id` has no self-reference and is self-authenticating regardless of mirror or host.

### Checked compatibility
Version strings order candidates and express author intent, but **compatibility is checked on interfaces**:

- **Safe (non-breaking):** adding exports/modules, adding defaulted class methods, weakening preconditions or effect rows, strengthening postconditions, removing unchecked assumptions, relaxing required target features.
- **Breaking:** removing/renaming exports, changing schemes, adding constructors to exported closed types (breaking exhaustive matches), adding required methods, introducing instance ambiguity, widening effects/usage/staging capabilities, adding unchecked assumptions, or changing ABI layouts.

---

## 2. Manifests, resolution, and lockfiles

- **Sources:** `cinch pkg add` and `cinch pkg install` resolve registry releases, local `path` dependencies, and `git` dependencies into a single dependency closure.
- **One version per package name:** modules share a global namespace across a build closure, so diamond dependencies must resolve to a single compatible release per package name (via fixed-point range intersection; irreconcilable constraints are refused).
- **Vendoring and prebuilt objects:** `cinch pkg install` vendors sources under `vendor/`, compiles `package.o`, `package.abi.json`, and `build/package.cinchif` (usable via `--package-object`), and pins SHA-256 `package_id` digests in `.cinchlock` (`--locked` refuses drift). Git checkouts cache under `$CINCH_STORE/git-packages/`.

---

## 3. Embedded core vs standard library packages

The compiler currently embeds `cinch-base` (`seed/prelude/cinch-base.cinchpkg`, 70 embedded modules out of 331 total prelude/stdlib modules in `seed/prelude/`).

The target modular split separates:
1. **Minimal embedded core:** the small set of compiler-known primitive types, classes, and kernel effects required to bootstrap any program (`Data.Core`, `Data.Nat`, `Data.Int`, `Data.Bool`, `Data.String`, `Data.Bytes`, `Effect.Io`, etc.).
2. **Official packages:** standard-library domains (crypto, HTTP/TLS, image codecs, formats, datetime, mutable containers) packaged as versioned releases under the official namespace with checked compatibility, reproducible builds, and documented trust/unsafe boundaries.

---

## 4. Registry protocol and supply chain

The prototype registry (`registry/server.mjs`, Node 24 with built-in SQLite and zero external dependencies) exercises the protocol:

- **Public and private namespaces:** user namespaces default to public; org namespaces default to private. Private access requires token scope + namespace allowlist + membership; unauthorized requests return `404` to avoid leaking existence.
- **Signed withdrawal statements:** yank, takedown, and restore events are recorded as append-only in-toto Statements in ECDSA P-256 DSSE envelopes (`/api/v1/statements`, `/api/v1/registry-key`). Takedowns return `410` on download while retaining blobs for audit/restoration.
- **SLSA provenance:** `cinch pkg verify --slsa` checks release integrity and provenance attestations. Unsigned envelopes verify structurally at `SLSA_BUILD_LEVEL_1`; envelopes signed by trusted builder keys (`CINCH_PROVENANCE_KEYS`) verify at `SLSA_BUILD_LEVEL_2`.

### Remaining package-ecosystem work
- Production registry infrastructure (OIDC/SSO, object storage, per-namespace signed private indexes, VSA issuance, anti-rollback snapshots).
- Automated `compat(old, new)` verification during publish and upgrade.
- Migrating non-core `seed/prelude/` modules into separately versioned official packages.
