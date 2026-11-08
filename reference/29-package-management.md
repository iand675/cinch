# Packages, manifests, and the registry

<span class="support-tag support-tag--planned">Partial</span> **`cinch pkg` installs registry, path, and git packages, publishes, and verifies.** `cinch pkg add` fetches a release or reads a local directory or git checkout, writes its sources under `vendor/`, checks them into `.cinchif` interfaces, emits a native package object and ABI manifest, and updates `.cinchlock`. Registry dependencies use exact versions and semver ranges, with one version per name by fixed-point intersection. Path and git packages are identified by a content root over the canonical manifest and their source bytes, never by directory or revision. `cinch pkg publish` uploads a project `.cinchpkg` and its module files with the same bearer token; `cinch pkg verify` rechecks lock identity, object hash, and interface files locally. Provenance and SLSA verification are still not implemented.

This chapter separates three things that are easy to confuse:

- what the Rust compiler and CLI do today;
- what the runnable registry prototype in `registry/` does today;
- what the package management design proposes for a future client.

The [package management design](../design/package-management.md) is the full design record for release identity, checked compatibility, resolution, lockfiles, private registries, withdrawal, and SLSA provenance.

# Current compiler behavior

Packages are currently a source distribution plus a deterministic one-version-per-name install path. The client resolves exact versions and semver ranges from registry metadata by recomputing reachable requirements to a fixed point; adding a later constraint can lower the selected version before any package is installed. If no version satisfies all reachable constraints, resolution fails without writing a partial lock.

- A project imports module names. The compiler resolves imports against already loaded modules, source files below the entry (including installed packages), and embedded prelude modules under the module rules in [modules and packages](07-modules-and-packages.md).
- `cinch pkg add NAME@VERSION --registry URL` fetches one release from the registry prototype. It recursively installs registry dependencies using exact versions or semver ranges, writes each release under `vendor/<namespace>/<name>/<version>/`, verifies its identity, checks its sources, writes interfaces under that package's `build/`, emits `package.o`, `package.abi.json`, and the package execution `build/package.cinchif`, and records every release in `.cinchlock`.
- `cinch pkg add PATH_DIR` (or `--path DIR`) installs the package whose `.cinchpkg` sits at that directory. Its own `source: "path"` dependencies resolve relative to its directory, and `source: "git"` dependencies are cloned at the named revision.
- `cinch pkg add URL --git-rev REV` (or `--git URL --git-rev REV`) clones `URL` at `REV` and installs the package at the checkout root. `<path>`, `file://`, and https URLs work because the client shells out to `git`.
- `cinch pkg install --project DIR` reads the project's `.cinchpkg` and installs its `path`, `git`, and `registry` dependencies without requiring the project to be published. `--locked` refuses to change any lock entry, which is how a dirty path dependency is reported.
- Path and git identity is `sha256` over the canonical manifest and the sorted `path`, `hash` rows of the package's module files. The version string is a selection hint and only names the vendor directory. `.cinchlock` records the content root in `package_id` plus `source.kind` (`path` or `git`) with the authored `path` or the `url` and `rev`.
- A registry release cannot disclose a usable local source. A `path` or `git` dependency in registry metadata is rejected with an error that names the package and asks for the dependency to be declared in the project manifest or installed from a local root.
- `--token` or `CINCH_REGISTRY_TOKEN` authenticates metadata and bundle reads for private packages. The token is not written to `.cinchlock`.
- `cinch pkg publish --registry URL --project DIR --token TOKEN [--visibility public|private]` reads `.cinchpkg` and the module files, computes the release identity, and uploads the bundle with an unsigned L1 in-toto SLSA provenance attestation. The registry still authorizes the namespace and visibility.
- `cinch pkg verify --slsa --project DIR` checks the local provenance file against the lock identity, predicate type, Cinch build type, external package parameter, and builder id. It reports `SLSA_BUILD_LEVEL_1`; a trusted builder signature raises the registry's claim to L2, but this client does not hold builder keys.
- `cinch pkg verify --project DIR` checks every lock entry against its installed `.cinchpkg`, source files, object hash, and interface files. It is a local integrity check, not provenance; add `--slsa` for the recorded L1 provenance check.
- Package objects define only their own modules and generated primitive helpers. Dependency modules are external declarations, not bundled copies; the JIT loads dependency objects first and the root object registers the combined layout prefix. `build/package.cinchif` carries the physical execution manifest (modules, units, layout words, object hash, interface hashes) so JIT and native object admission read a `.cinchif`, not a second manifest name. Installed packages are importable by ordinary project source once the vendor tree is present; the source loader indexes the vendor tree recursively.
- `cinch check`, `cinch build`, and `cinch run` use the verified package store as a cache for checked interfaces and artifacts. The store is not a resolver and does not choose versions.
- The compiler embeds a base package manifest, `compiler/prelude/cinch-base.cinchpkg`, which names the embedded modules and their check order. The manifest is data, so the module list is not duplicated in driver code.
- Path, git, and registry packages share one closure. Cycles, two contents under one package name, two packages declaring one module name, and mixed local/registry name collisions are named errors before anything is written.
- Native `build` and `run` consume installed package objects with `--package-object`; project IR keeps only project modules and declares object symbols. Client-side L2 builder-key verification and VSA issuance remain absent. An unsupported command prints usage and exits 2.

| Surface | Current status |
| --- | --- |
| Source modules beside an entry file | Available |
| Embedded `cinch-base` package manifest | Available |
| Verified local interface/artifact store | Available |
| `CINCH_STORE` and `--rebuild` | Available |
| Project `.cinchpkg` dependency install with `cinch pkg install` | Available |
| Public and private registry install with `cinch pkg add` | Available for one resolved release |
| Vendored source install and package object/interface generation | Available |
| Package-level execution `.cinchif` for JIT/native object admission | Available |
| Registry dependency closure with exact and semver ranges | Available with fixed-point intersection |
| Path dependency sources | Available |
| Git dependency sources | Available for `<path>`, `file://`, and https URLs |
| Mixed registry, path, and git closure | Available |
| Local lock and artifact verification with `cinch pkg verify` | Available |
| Publishing with `cinch pkg publish` | Available for authorized namespaces |
| L1 provenance on publish and local `verify --slsa` | Available |
| L2 builder-key verification and VSA issuance | Not implemented |
| Registry HTTP prototype | Runnable in `registry/` |

# Local source limits

- `pkg add` records an install in `.cinchlock` and does not edit `.cinchpkg`, so a later `pkg install` sees only the dependencies the project manifest declares. Add the row to the manifest yourself when the project should resolve it.
- Git authentication belongs to the `git` binary: a credential helper or an SSH agent. The lock records the URL as authored, so do not embed a secret in it.
- A path dependency is read from its directory at install time. Nothing watches it; re-run `cinch pkg install` after editing it, and use `--locked` to detect the change instead.
- There is no source replacement, override map, workspace, `--frozen`, or offline mode yet. A git checkout cache is local to `CINCH_STORE` and can always be removed.

# Manifest format

The compiler's manifest reader accepts the canonical JSON form used by the base package. Schema 1 is the current form for the base package and for publish; the installer also reads the schema 2 dependency list described below.

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

The installer reads `dependencies` from a schema 1 or schema 2 manifest. The design's schema 2 also adds stage policy, target requirements, and the `namespace/name` spelling; those fields are ignored by the current installer, and dependency entries are the only schema 2 addition it acts on.

```json
{
  "schema": 2,
  "name": "acme/text",
  "version": "0.2.0",
  "modules": [{ "name": "Text", "path": "Text.cinch" }],
  "order": ["Text"],
  "dependencies": [
    { "name": "cinch/base", "range": "^0.1.0", "source": "registry" },
    { "name": "acme/bytes", "source": "path", "path": "../bytes" },
    {
      "name": "acme/schema",
      "source": "git",
      "url": "https://example/acme-schema",
      "rev": "abc123"
    }
  ]
}
```

`source` defaults to `registry`, where `range` or `version` selects a release. A `path` dependency resolves relative to the directory of the manifest that declares it, so sibling packages need no absolute paths. A `git` dependency needs `url` and `rev`. Both local kinds may carry `version` or `range` as a hint; a mismatch with the target manifest's version is a named error, because the version is not the identity. `cinch/base` and `cinch-base` are skipped everywhere because the base package is embedded.

# Local package store

The store caches checked interfaces and build artifacts. It is keyed by content, compiler, runtime, target profile, and other build inputs. A malformed or mismatched entry is a diagnostic; `--rebuild` recomputes the entry instead of trusting or silently deleting it.

`CINCH_STORE` overrides the store root. On macOS the default is `~/Library/Caches/cinch/store`; other platforms use `XDG_CACHE_HOME/cinch/store` when set, otherwise `~/.cache/cinch/store`. The store is advisory: it cannot change program semantics or substitute an unchecked interface. See [artifacts and incremental compilation](18-artifacts-and-incremental.md).

Git dependencies use a checkout cache beside the store: `CINCH_STORE/git-packages/<hash of url and rev>`. The cache is keyed by the requested URL and revision only for reuse. The package it contains is still identified by its content root, and a cache checkout with local changes is a named error rather than a rebuild. Removing the cache directory is always safe.

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

The uploaded bundle is canonical JSON containing the manifest, base64 source files, visibility, and readme. The registry checks SLSA provenance structure against the release identity: unsigned envelopes are `SLSA_BUILD_LEVEL_1` with `verified: false`, and envelopes signed by a key in `CINCH_PROVENANCE_KEYS` are `SLSA_BUILD_LEVEL_2` with `verified: true`. VSA issuance is not implemented.

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
- VSA issuance and client-side trust in L2 builder keys; the registry currently performs structural L1 and optional trusted-key L2 verification.
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
cinch pkg add PATH_DIR [--project DIR]
cinch pkg add URL --git-rev REV [--project DIR]
cinch pkg add --path DIR [--project DIR]
cinch pkg add --git URL --git-rev REV [--project DIR]
cinch pkg install [--project DIR] [--registry URL] [--token TOKEN] [--locked]
cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
cinch pkg verify [--slsa] [--project DIR]
```

`install` is the long form of `add` for a registry spec. With no package argument it reads the project `.cinchpkg` and installs its declared dependencies; `add` always needs an explicit source. `--path` and `--git` are mutually exclusive, and `--git` needs `--git-rev`. A bare positional that is an existing directory is treated as a path source, and a bare URL is treated as a git source only when `--git-rev` is present; otherwise the argument must be `NAME@VERSION`. CLI path arguments resolve against the current directory, while a `path` entry inside a manifest resolves against the directory of that manifest. Adding the project directory itself is rejected with a named error; use `cinch pkg install` for the project's own dependencies. `--locked` applies to `add` and `install` and refuses any lock rewrite.

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

`cinch pkg verify` is local; `--slsa` verifies the recorded unsigned L1 provenance. The design's `--online` mode, client-side L2 builder roots, the producer flag `cinch build --emit-provenance`, and a remote audit command remain future work. Do not write those into build scripts yet. The current CLI's supported commands are in [using the Rust compiler](01-using-the-compiler.md).

# Verification

The current package surfaces are exercised by:

- `compiler/tests/package_manifest.rs` for the embedded manifest and check order;
- `compiler/tests/package_store.rs` for store keys, verification, and `--rebuild`;
- `compiler/tests/canonical_artifacts.rs` for canonical artifact identity;
- `compiler/src/pkg.rs` tests for semver range intersection, bundle identity, vendoring, object emission, lock updates, and verification;
- `compiler/src/pkg.rs` tests for a two-package path dependency graph, path lock entries, a cycle, a one-name conflict, a missing path, a dirty path under `--locked`, a git dependency from a local `git init` repository, a mixed registry and path closure against an in-process HTTP fixture, and rejection of a path source disclosed by registry metadata;
- `compiler/src/provenance.rs` tests for L1 subject and build-field verification;
- `registry/test/registry.test.mjs` for unsigned L1, trusted-key L2, and mismatched-subject rejection;
- manual registry checks for private publish, authenticated install, rejection of an unauthenticated private read, and local `verify --slsa`;
- `registry/test/` for the registry HTTP and signing behavior.

Implementation: `compiler/src/pkg.rs`, `compiler/src/driver.rs`, `compiler/src/store.rs`, `compiler/prelude/cinch-base.cinchpkg`, `registry/server.mjs`, and `registry/lib/`. The registry source and its full request examples are in [`registry/README.md`](https://github.com/iand675/cinch/blob/main/registry/README.md).
