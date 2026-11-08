# Cinch registry (v0)

A runnable package registry prototype for Cinch, covering the public and private
package paths in [the package management design](../design/package-management.md).
It is a single Node process with no third-party dependencies: built-in SQLite for
metadata, the filesystem for content-addressed blobs, and ECDSA P-256 DSSE
signatures for registry events.

This is not npmjs or Hackage. It is the smallest service that exercises the
protocol the design describes: publish, resolve, download, private namespaces,
signed yank/takedown/restore events, and a browsable public index. Production
gaps are listed at the end.

## Run

```sh
cd registry
CINCH_REGISTRY_ADMIN_TOKEN=change-me node server.mjs
# Cinch registry listening on http://127.0.0.1:8787
```

Environment:

| Variable | Default | Meaning |
| --- | --- | --- |
| `PORT` | `8787` | HTTP port |
| `CINCH_REGISTRY_DATA` | `registry/data` | SQLite database, blob store, and signing key |
| `CINCH_REGISTRY_ADMIN_TOKEN` | generated and printed | Bootstrap admin token with `read`, `publish`, and `admin` scopes for `*` |
| `CINCH_PROVENANCE_KEYS` | `{}` | JSON object mapping DSSE key ids to public JWK keys trusted for L2 builder provenance |

The admin token is rotated into the database on every start. Store it outside the
data directory in a real deployment.

## Test

```sh
cd registry
npm test
```

The suite covers public publish/download, private namespace access, token
scoping, duplicate and path-traversal rejection, yank, takedown, restore, DSSE
signature verification, and revocation.

## Package identity

`package_id` is the release identity from the design:

```
sha256("package\n" + canonicalJson(manifest) + "\n" +
       sorted(path + "\t" + sha256(fileBytes)).join("\n"))
```

The manifest is excluded from the file list, so the digest has no self-reference.
The server computes it from the uploaded bytes. If the client sends `packageId`,
the server rejects a mismatch.

## Auth model

- A **principal** is a user or an org.
- A **namespace** is owned by a principal. A user gets a namespace equal to its
  handle; an org is an explicit namespace.
- **Memberships** are `owner`, `publisher`, or `reader`.
- A **token** has scopes (`read`, `publish`, `admin`) and a namespace allowlist
  (`*` for admin). Tokens are random 32-byte values, stored only as SHA-256
  hashes, and can be revoked.
- Private package access requires all three: a `read` scope, the package
  namespace in the token allowlist, and a membership in that namespace.

Unauthorized private packages return `404`, not `403`, so the registry does not
leak their existence. A blocked package returns `410` on download.

## Public and private packages

- `visibility: "public"`: visible in the anonymous index, search, and downloads.
- `visibility: "private"`: visible only in the authenticated private index and to
  authorized tokens.
- Default visibility is `public` for a user namespace and `private` for an org
  namespace. Send `visibility` explicitly to override.
- The public index is cacheable. The private index is `no-store` and is filtered
  per token.
- The HTML site lists public packages only. Private access is through the API.

## API

Base path: `/api/v1`.

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `GET` | `/health` | none | Liveness |
| `POST` | `/principals` | none | Create a user and its namespace; returns a token |
| `POST` | `/namespaces` | token | Create an org namespace |
| `POST` | `/namespaces/:namespace/members` | owner or admin | Add or update a member |
| `POST` | `/tokens` | token | Mint a scoped token |
| `DELETE` | `/tokens/:id` | owner or admin | Revoke a token |
| `POST` | `/packages` | `publish` | Publish a release |
| `GET` | `/packages/:name` | optional | List versions visible to the caller |
| `GET` | `/packages/:name/:version` | optional | Metadata, manifest, files, attestations |
| `GET` | `/download/:sha256` | optional | Download the package bundle |
| `GET` | `/index` | none | Public index snapshot |
| `GET` | `/index/private` | token | Public plus authorized private entries |
| `GET` | `/search?q=` | optional | Public search, or authorized private results |
| `GET` | `/statements?since=` | optional | Signed yank/takedown/restore events |
| `GET` | `/audit?since=` | admin | All registry events |
| `GET` | `/registry-key` | none | ECDSA public key for statement verification |
| `POST` | `/packages/:name/:version/yank` | `publish` | Mark a release yanked |
| `POST` | `/packages/:name/:version/takedown` | admin | Block a release |
| `POST` | `/packages/:name/:version/restore` | admin | Unblock a release |

Package names contain `/`; encode it as `%2F` in a path.

### Create a user

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/principals \
  -H 'content-type: application/json' \
  -d '{"handle":"alice"}'
```

Response:

```json
{
  "principal": { "id": "p_...", "handle": "alice", "kind": "user" },
  "namespace": "alice",
  "token": "..."
}
```

### Publish a public package

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

The response includes `entry.packageId` (`sha256:...`) and `entry.bundle`.
Download with the hex part of the package id:

```sh
curl -s http://127.0.0.1:8787/api/v1/download/<hex>
```

### Create an org and publish privately

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

Publish with `"visibility":"private"` and the org token. Anonymous index and
download requests will not see it.

### Takedown and restore

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/packages/alice%2Fhello/0.1.0/takedown \
  -H "authorization: Bearer $ADMIN_TOKEN" \
  -H 'content-type: application/json' \
  -d '{"reason":"copyright"}'
```

A takedown sets `blocked`, removes the release from the index, and makes download
return `410`. The bytes stay in the blob store for audit and restoration.

### Verify a signed statement

`GET /api/v1/statements` returns each event's in-toto Statement and DSSE
envelope. `GET /api/v1/registry-key` returns the ECDSA P-256 public key. The
signature is over the DSSE pre-authentication encoding with SHA-256.

## Storage

```
data/
  registry.db          SQLite: principals, namespaces, members, tokens, packages, attestations, statements
  registry-key.json    ECDSA P-256 private key (mode 0600)
  blobs/<aa>/<rest>    canonical package bundles keyed by sha256
```

The bundle is canonical JSON containing the manifest, base64 source files,
visibility, and readme. Attestations are stored in SQLite and returned in package metadata.
Provenance attestations are checked structurally against the release identity.
Unsigned envelopes are reported as `SLSA_BUILD_LEVEL_1`; envelopes signed by a
key in `CINCH_PROVENANCE_KEYS` are reported as `SLSA_BUILD_LEVEL_2` with
`verified: true`.

## Production gaps

This prototype is deliberately small. A public registry needs:

- OIDC/SSO, email verification, password reset, and organization invitations.
- Rate limits, quotas, abuse reporting, malware scanning, and retention policy.
- Object storage, a real database, backups, and multi-node consistency.
- Per-namespace signed private indexes instead of a filtered view.
- VSA issuance and client-side trust in L2 builder keys; the registry currently performs structural L1 and optional trusted-key L2 verification.
- Search indexing, a private package UI, and package documentation rendering.
- CDN distribution for public blobs, mirroring, and anti-rollback snapshots.
- Billing for private packages, legal takedown workflow, and audit retention.
- Namespace disputes, publisher verification, and typosquat detection.

## Mapping to the design

| Design section | Implementation |
| --- | --- |
| Section 2, content identity | `lib/canonical.mjs`, package identity check in `lib/store.mjs` |
| Section 6, registry and private packages | `server.mjs` API, memberships, token scoping |
| Section 6, withdrawal statements | `lib/store.mjs` `addStatement`, `lib/signer.mjs` DSSE |
| Section 8, SLSA | Structural L1 verification, optional trusted-builder L2 verification, attestation storage |

The protocol is intentionally independent of this implementation. A Rust or
self-hosted Cinch service can replace it without changing the client contract.
