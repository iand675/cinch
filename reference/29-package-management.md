# Packages, manifests, and the registry

<span class="support-tag support-tag--planned">Partial</span> **`cinch pkg` installs, publishes, and verifies releases from the registry prototype.** `cinch pkg add` fetches a bundle, writes its sources under `vendor/`, checks them into `.cinchif` interfaces, emits a native package object and ABI manifest, and updates `.cinchlock`. It walks registry dependencies with exact versions and semver ranges, accepts bearer tokens for private packages, and checks conflicts against an already selected version. `cinch pkg publish` uploads a project `.cinchpkg` and its module files with the same token; `cinch pkg verify` rechecks lock identity, object hash, and interface files locally. Full backtracking resolution, path/git dependencies, and SLSA verification are still not implemented.

This chapter separates three things that are easy to confuse:

- what the Rust compiler and CLI do today;
- what the runnable registry prototype in `registry/` does today;
- what the package management design proposes for a future client.

The [package management design](../design/package-management.md) is the full design record for release identity, checked compatibility, resolution, lockfiles, private registries, withdrawal, and SLSA provenance.

# Current compiler behavior

Packages are currently a source distribution plus a deterministic single-version install path. The client resolves exact versions and semver ranges from registry metadata; it does not backtrack when a later constraint would require selecting a different version.

- A project imports module names. The compiler resolves imports against already loaded modules, source files below the entry (including installed packages), and embedded prelude modules under the module rules in [modules and packages](07-modules-and-packages.md).
- `cinch pkg add NAME@VERSION --registry URL` fetches one release from the registry prototype. It recursively installs registry dependencies using exact versions or semver ranges, writes each release under `vendor/<namespace>/<name>/<version>/`, verifies its identity, checks its sources, writes interfaces under that package's `build/`, emits `package.o` and `package.abi.json`, and records every release in `.cinchlock`.
- `--token` or `CINCH_REGISTRY_TOKEN` authenticates metadata and bundle reads for private packages. The token is not written to `.cinchlock`.
- `cinch pkg publish --registry URL --project DIR --token TOKEN [--visibility public|private]` reads `.cinchpkg` and the module files, computes the release identity, and uploads the bundle. The registry still authorizes the namespace and visibility.
- `cinch pkg verify --project DIR` checks every lock entry against its installed `.cinchpkg`, source files, object hash, and interface files. It is a local integrity check, not provenance.
- Installed packages are importable by ordinary project source once the vendor tree is present. The source loader indexes the vendor tree recursively.
- `cinch check`, `cinch build`, and `cinch run` use the verified package store as a cache for checked interfaces and artifacts. The store is not a resolver and does not choose versions.
- The compiler embeds a base package manifest, `compiler/prelude/cinch-base.cinchpkg`, which names the embedded modules and their check order. The manifest is data, so the module list is not duplicated in driver code.
- There is still no backtracking resolver, path/git dependency source, provenance verification, or package-object consumption by project builds. An unsupported command prints usage and exits 2.

| Surface | Current status |
| --- | --- |
| Source modules beside an entry file | Available |
| Embedded `cinch-base` package manifest | Available |
| Verified local interface/artifact store | Available |
| `CINCH_STORE` and `--rebuild` | Available |
| Project `.cinchpkg` loading through the CLI | Not implemented |
| Public and private registry install with `cinch pkg add` | Available for one resolved release |
| Vendored source install and package object/interface generation | Available |
| Registry dependency closure with exact and semver ranges | Available with selected-version conflict checks |
| Local lock and artifact verification with `cinch pkg verify` | Available |
| Publishing with `cinch pkg publish` | Available for authorized namespaces |
| Backtracking resolution and path/git dependencies | Not implemented |
| Provenance and SLSA verification | Not implemented |
| Registry HTTP prototype | Runnable in `registry/` |

# Manifest format

The compiler's manifest reader accepts the canonical JSON form used by the base package. The current schema is version 1.

```json
{
  "schema": 1,
  "name": "cinch-base",
  "version": "0.1.0",
  "modules": [
    {
      "name": "Data.Core",
      "path": "Data/Core.cinch"
    },
    {
      "name": "Prelude",
      "path": "Prelude.cinch",
      "facade": true
    }
  ],
  "order": ["Data.Core", "Prelude"]
}
```

`name` and `version` identify the package in the manifest. `modules` lists each module name, its source path, and optional facade metadata. `order` is the check order and must name every module exactly once. Imports are intentionally read from source, not copied into the manifest, so an added import cannot drift from a stale manifest copy.

The `embedded` field in the shipped base manifest is informational. The embedded source table decides whether a path has bytes. A manifest with an unknown schema currently is not rejected by `Package::parse`; schema validation belongs to the proposed package manager, not to the current CLI.

Schema 2 in the [design](../design/package-management.md#4-package-shape-and-manifest-schema-2) adds dependencies, path and git sources, stage policy, target requirements, and the `namespace/name` spelling. Those fields are not accepted as a working package workflow today. Do not treat a schema 2 example as a manifest the current compiler will resolve.

# Local package store

The store caches checked interfaces and build artifacts. It is keyed by content, compiler, runtime, target profile, and other build inputs. A malformed or mismatched entry is a diagnostic; `--rebuild` recomputes the entry instead of trusting or silently deleting it.

`CINCH_STORE` overrides the store root. On macOS the default is `~/Library/Caches/cinch/store`; other platforms use `XDG_CACHE_HOME/cinch/store` when set, otherwise `~/.cache/cinch/store`. The store is advisory: it cannot change program semantics or substitute an unchecked interface. See [artifacts and incremental compilation](18-artifacts-and-incremental.md).

The registry prototype is independent of this store. Publishing a package does not write the compiler's cache, and downloading a blob does not make it a resolved dependency.

# Registry prototype

`registry/` contains a runnable v0 server for the design's public and private package paths. It is a single Node process using Node 24's built-in SQLite and HTTP support, with no third-party runtime dependencies. It is not npmjs, Hackage, or the future Cinch client.

```sh
cd registry
CINCH_REGISTRY_ADMIN_TOKEN=change-me node server.mjs
# Cinch registry listening on http://127.0.0.1:8787
```

| Variable | Default | Meaning |
| --- | --- | --- |
| `PORT` | `8787` | HTTP port |
| `CINCH_REGISTRY_DATA` | `registry/data` | SQLite database, blob store, and signing key |
| `CINCH_REGISTRY_ADMIN_TOKEN` | generated and printed | Bootstrap admin token with `read`, `publish`, and `admin` scopes for `*` |

The admin token is rotated into the database on every start. In a real deployment, store it outside the data directory.

Run the registry suite with:

```sh
cd registry
npm test
```

The tests cover public publish/download, private namespaces, token scoping, duplicate and path-traversal rejection, yank, takedown, restore, DSSE signature verification, and revocation.

# Registry API

The API base path is `/api/v1`. Package names contain `/`; encode it as `%2F` in a path.

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `GET` | `/health` | none | Liveness |
| `POST` | `/principals` | none | Create a user and its namespace; returns a token |
| `POST` | `/namespaces` | token | Create an organization namespace |
| `POST` | `/namespaces/:namespace/members` | owner or admin | Add or update a member |
| `POST` | `/tokens` | token | Mint a scoped token |
| `DELETE` | `/tokens/:id` | owner or admin | Revoke a token |
| `POST` | `/packages` | `publish` | Publish a release |
| `GET` | `/packages/:name` | optional | List versions visible to the caller |
| `GET` | `/packages/:name/:version` | optional | Metadata, manifest, files, and attestations |
| `GET` | `/download/:sha256` | optional | Download the package bundle |
| `GET` | `/index` | none | Public index snapshot |
| `GET` | `/index/private` | token | Public plus authorized private entries |
| `GET` | `/search?q=` | optional | Public search, or authorized private results |
| `GET` | `/statements?since=` | optional | Signed yank, takedown, and restore events |
| `GET` | `/audit?since=` | admin | All registry events |
| `GET` | `/registry-key` | none | ECDSA public key for statement verification |
| `POST` | `/packages/:name/:version/yank` | `publish` | Mark a release yanked |
| `POST` | `/packages/:name/:version/takedown` | admin | Block a release |
| `POST` | `/packages/:name/:version/restore` | admin | Unblock a release |

An anonymous request sees public packages. A token request can additionally see private packages authorized by its scopes, namespace allowlist, and namespace membership. Unauthorized private releases return `404`, not `403`, so the registry does not leak their existence. A blocked release returns `410` on download.

# Identity and authentication

The registry computes a release identity from the uploaded bytes:

```text
sha256("package\n" + canonicalJson(manifest) + "\n" +
       sorted(path + "\t" + sha256(fileBytes)).join("\n"))
```

The manifest is excluded from the file list, so the digest has no self-reference. If a client supplies `packageId`, the server rejects a mismatch.

A **principal** is a user or an organization. A **namespace** is owned by a principal. A user gets a namespace equal to its handle; an organization is an explicit namespace. **Memberships** are `owner`, `publisher`, or `reader`.

A **token** has scopes (`read`, `publish`, `admin`) and a namespace allowlist (`*` for admin). Tokens are random 32-byte values stored only as SHA-256 hashes, and they can be revoked. Private access requires all three: a `read` scope, the package namespace in the token allowlist, and membership in that namespace.

# Publish and download

Create a user and capture its token:

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/principals \
  -H 'content-type: application/json' \
  -d '{"handle":"alice"}'
```

The response names the principal, its namespace, and a token. Publish a public release with that token:

```sh
MAIN=$(printf 'module Main where\n' | base64)
curl -s -X POST http://127.0.0.1:8787/api/v1/packages \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d "{
    \"manifest\": {
      \"schema\": 2,
      \"name\": \"alice/hello\",
      \"version\": \"0.1.0\",
      \"modules\": [{ \"name\": \"Main\", \"path\": \"Main.cinch\" }],
      \"order\": [\"Main\"]
    },
    \"files\": { \"Main.cinch\": \"$MAIN\" },
    \"visibility\": \"public\",
    \"readme\": \"Hello from Alice.\"
  }"
```

The response includes `entry.packageId` and `entry.bundle`. Download the bundle with the hex part of the package ID:

```sh
curl -s http://127.0.0.1:8787/api/v1/download/<hex>
```

The uploaded bundle is canonical JSON containing the manifest, base64 source files, visibility, and readme. Attestations are stored and returned with `verified: false`; SLSA verification is not implemented in v0.

# Private packages and organizations

Create an organization namespace with an existing token, then mint a token scoped to it:

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/namespaces \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"name":"acme","kind":"org"}'

curl -s -X POST http://127.0.0.1:8787/api/v1/tokens \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"scopes":["read","publish"],"namespaces":["acme"]}'
```

Publish with `"visibility":"private"` and the organization token. The anonymous index, search, and download endpoints will not reveal or serve the release. The public HTML site lists public packages only; private access is through the API.

Default visibility is `public` for a user namespace and `private` for an organization namespace. Send `visibility` explicitly when you want to override that default.

# Withdrawal, takedown, and restore

A publisher can yank a version:

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/packages/alice%2Fhello/0.1.0/yank \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"reason":"superseded"}'
```

An administrator can take a release down and later restore it:

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/packages/alice%2Fhello/0.1.0/takedown \
  -H "authorization: Bearer $ADMIN_TOKEN" \
  -H 'content-type: application/json' \
  -d '{"reason":"copyright"}'

curl -s -X POST http://127.0.0.1:8787/api/v1/packages/alice%2Fhello/0.1.0/restore \
  -H "authorization: Bearer $ADMIN_TOKEN" \
  -H 'content-type: application/json' \
  -d '{"reason":"dispute resolved"}'
```

A takedown sets `blocked`, removes the release from the index, and makes download return `410`. The bytes remain in the blob store for audit and restoration.

Each event is a signed in-toto Statement in a DSSE envelope. `GET /api/v1/statements` returns the statements; `GET /api/v1/registry-key` returns the ECDSA P-256 public key. The signature covers the DSSE pre-authentication encoding with SHA-256. Deleting a blob is not a takedown mechanism, because mirrors, caches, locks, and vendor trees can retain copies.

# Storage

The prototype keeps all state under `CINCH_REGISTRY_DATA`:

```text
data/
  registry.db          SQLite: principals, namespaces, members, tokens,
                       packages, attestations, statements
  registry-key.json    ECDSA P-256 private key (mode 0600)
  blobs/<aa>/<rest>    canonical package bundles keyed by sha256
```

The key file is a development convenience. A production registry needs role-separated keys, threshold signing, rotation, and an audited signing service.

# Production gaps

The prototype is deliberately small. A public or organizational registry still needs:

- OIDC/SSO, email verification, password reset, and organization invitations.
- Rate limits, quotas, abuse reporting, malware scanning, and retention policy.
- Object storage, a production database, backups, and multi-node consistency.
- Per-namespace signed private indexes instead of a filtered view.
- SLSA provenance and verification-summary verification at publish and fetch, not just storage.
- Search indexing, a private package UI, and package documentation rendering.
- CDN distribution for public blobs, mirroring, and anti-rollback snapshots.
- Billing for private packages, a legal takedown workflow, and audit retention.
- Namespace disputes, publisher verification, and typosquat detection.

# Design boundary

The design addresses the hard semantic questions the prototype does not:

- **Release identity** is a digest over canonical metadata and source bytes.
- **Compatibility** is a checked relation over module contracts, not a version-string promise.
- **One version per package name** follows from global module identity and global instance coherence.
- **Resolution is client-side and deterministic.** The registry serves immutable metadata and blobs; it does not choose versions for the build.
- **The lockfile records exact identities**, sources, policy, evidence, and artifacts. It never records a range as if it were a resolved version.
- **Withdrawal and takedown are signed statements**, not mutable flags or deleted bytes.
- **SLSA describes supply-chain integrity**, not semantic correctness, and the compiler emits only the provenance level it can actually observe.

Read the [package management design](../design/package-management.md) for the object model, schema 2, the one-version rule, lock format, registry protocol, private package policy, signing, SLSA mapping, rollout, and open decisions.

# Command surface

These commands are implemented by the current CLI:

```text
cinch pkg add NAME@VERSION [--registry URL] [--project DIR] [--token TOKEN]
cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
cinch pkg verify [--project DIR]
```

`CINCH_REGISTRY` sets the default registry URL and `CINCH_REGISTRY_TOKEN` supplies the default token. Tokens are sent only in the `authorization: Bearer` header; they are never written to `.cinchlock` or printed.

The design still proposes these commands; they are **not implemented**:

```text
cinch pkg init
cinch pkg remove NAME
cinch pkg fetch
cinch pkg update [NAME]
cinch pkg audit
cinch pkg attest
cinch pkg provenance NAME
cinch pkg sbom
cinch pkg vendor
cinch pkg purge NAME
cinch pkg yank VERSION
```

`cinch pkg verify` is local. The design's `--slsa` and `--online` verification modes, the producer flag `cinch build --emit-provenance`, and a remote audit command remain future work. Do not write those into build scripts yet. The current CLI's supported commands are in [using the Rust compiler](01-using-the-compiler.md).

# Verification

The current package surfaces are exercised by:

- `compiler/tests/package_manifest.rs` for the embedded manifest and check order;
- `compiler/tests/package_store.rs` for store keys, verification, and `--rebuild`;
- `compiler/tests/canonical_artifacts.rs` for canonical artifact identity;
- `compiler/src/pkg.rs` tests for semver range intersection, bundle identity, vendoring, object emission, lock updates, and verification;
- manual registry checks for private publish, authenticated install, and rejection of an unauthenticated private read;
- `registry/test/` for the registry HTTP and signing behavior.

Implementation: `compiler/src/pkg.rs`, `compiler/src/driver.rs`, `compiler/src/store.rs`, `compiler/prelude/cinch-base.cinchpkg`, `registry/server.mjs`, and `registry/lib/`. The registry source and its full request examples are in [`registry/README.md`](https://github.com/iand675/cinch/blob/main/registry/README.md).
