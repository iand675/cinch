# Package management

<span class="support-tag support-tag--planned">Coming soon</span> **This is a design record, not the current package manager.** For the compiler's present manifest, local store, and registry prototype, read [packages, manifests, and the registry](../reference/29-package-management.md).

Status: design proposal, 2026-11-06. The compiler has a package manifest, a
content-addressed local artifact store, and a checked interface per module. It
has no dependency graph, no registry, and no install or publish command. This
note fixes the object model, the resolution rules, and the supply-chain policy
those commands must implement, and names the first slice that can be built
without a network.

The short version: a package release is identified by a strong hash of its
source tree and canonical manifest. Compatibility is a checked relation between
interfaces, not a claim carried by a version string. The registry is a static,
signed, append-only index over content-addressed blobs. Resolution happens on
the client and is pinned by a lockfile. Version numbers remain useful labels and
selection hints; they never decide correctness.

# 1 The substrate that already exists

| Fact | Where | What it means |
| --- | --- | --- |
| Manifest | `compiler/prelude/cinch-base.cinchpkg`, parsed by `Package::parse` in `compiler/src/driver.rs` | Schema 1: name, version, `modules[]` (name, path, embedded flag, facade flag), and a total `order`. |
| Imports | Read from source by `scan_file` | Deliberately not duplicated in the manifest, so a manifest cannot disagree with the source. |
| Contract identity | `Iface::iface_hash` in `compiler/src/interface.rs` | 64-bit FNV-1a over canonical interface JSON with `source_hash`, `compiler`, and `units` cleared. Excludes body and source, so a body-only change keeps the identity. |
| Instantiation identity | `UnitExport.unit_id` | Hash of target module, parameter signatures, and argument identities. Excluded from `iface_hash`; a dictionary change moves the unit, not the target module contract. |
| Build identity | `package_key` in `compiler/src/store.rs` | Content hash over package name, version, manifest bytes, module entries, every source hash, interface schema, runtime ABI, language version, compiler version, target profile, and WIT adapters. This is the local cache key; section 2 defines the strong artifact digest. |
| Store | `Store` in `compiler/src/store.rs` | Per-user cache of `<root>/<key>` entries holding verified interfaces plus object and IR bodies. A mismatch is a hard error with `--rebuild`. |
| Build evidence | `Iface` in `compiler/src/interface.rs` and the artifact manifest | Records source hashes, captured stage inputs, assumptions, foreign contracts, and `trust.level`; this is the raw material for SLSA provenance. |
| Compatibility | `specifications/runtime-evolution-specification.md` | A replacement must refine the published contract for existing clients; rejection names the failing dimension. |
| Compile-time execution | `specifications/modules-and-compilation-specification.md` sections 7 to 10 | `stage` code runs under capabilities with refresh, replay, and live modes; build identity includes generator, inputs, and policy. |
| Transitive trust | `context/design-conversation.md` | An artifact must report its guarantees and assumptions transitively; a dependency cannot hide an unchecked foreign contract. |

Three consequences follow.

1. The identity substrate exists, but it is local and weak. Every hash in the
   current compiler is FNV-1a, including `package_key` and `iface_hash`. That is
   acceptable for a local cache whose threat model is bit rot. It is not
   collision-resistant and cannot be the security boundary of a registry.
2. The interface is already the right compatibility object. Contracts, laws,
   fixities, instance facts, ABI facts, and assumptions live in the `.cinchif`,
   and `iface_hash` already separates contract from body. Package management
   should consume that object, not invent a second notion of public surface.
3. The manifest describes a package set but has no edges. The missing layer is
   dependency resolution and distribution, not a new package format.

The website's `/packages/` catalogue is sample documentation data. It is not a
registry, and its dependency names are not a protocol.

# 2 Thesis: content identity plus checked compatibility

Most ecosystems treat a version range as a social promise about compatibility.
This language can do better: compatibility is a relation between two interface
facts, and the compiler already records the facts.

Define the identities first.

```
source tree   sorted (path, sha256(bytes)) entries, no timestamps, no symlinks
release       canonical manifest + source tree
package_id    sha256("package" | canonical manifest | every other path and file digest)
cache_key     current package_key (FNV-1a, local store directory)
artifact_id   sha256("artifact" | package_id | compiler | profile | adapters |
                     build policy | module -> iface_digest | body digests)
```

The manifest is hashed as canonical bytes and excluded from the file list, so
the digest has no self-reference. `package_id` is the release identity. It is
strong, self-authenticating, and independent of who hosts it. `artifact_id` is
a build of that release and belongs to exactly one release; the reverse is not
true, because one release has one artifact per compiler, profile, adapter set,
and build policy.

`iface_hash` is a local cache key over a 64-bit function, and it covers the
whole interface JSON with only `source_hash`, `compiler`, and `units` cleared.
That includes `schema`, `language`, `runtime_abi`, layout rows, and foreign ABI
facts, so it can vary by target profile where those facts vary. Do not redefine
it in place: the store, the bootstrap checkpoints, and recorded artifact seals
already depend on its values.

Add two strong digests at the package layer instead.

- `contract_digest`: SHA-256 over a target-independent contract projection.
  Include the module name, parameter signatures, exported types and schemes,
  instances, fixities, contracts, assertions, laws, assumptions, and upgrade
  evidence. Exclude layouts, foreign ABI widths, `runtime_abi`, `source_hash`,
  `compiler`, and `units`.
- `iface_digest(profile)`: SHA-256 over the full canonical interface for a named
  target profile.

Source-level compatibility uses `contract_digest`. ABI-sensitive consumers and
distributed objects use `iface_digest` for their profile. The registry publishes
`contract_digest` for every release and optional per-profile `iface_digest`
values. The weak `iface_hash` remains the local store and checked-against key,
and is never a registry, lock, or attestation input. A weak store key must never
decide whether a downloaded release is the one the lock names.

Compatibility is then a checked predicate, not a semver convention.

```
compat(old, new) = every client checked against old remains valid against new
```

Two scopes are possible: the whole exported surface, and the exact surface a
given client used. Start with the whole surface. It is conservative, cheaper to
implement, and already useful. A client-precise mode can come later without
changing the lock format, because the lock names releases and the verifier can
choose the scope.

| Interface fact | Safe for existing clients | Breaking |
| --- | --- | --- |
| Export set | Add a new value, type, or module. | Remove or rename an export; change a scheme. |
| Data types | Add an independent type. | Add a constructor to an exported closed type, because exhaustive matches in clients stop covering it; remove or retag a constructor; change representation facts. |
| Signatures | Add a defaulted method whose default is checked and does not require existing instances to change. | Add a required method; change a method scheme; change implicit resolution behavior. |
| Instances | Add an instance that cannot unify with an existing instance in any consumer closure. | Remove an instance; add an instance that creates ambiguity. |
| Effects and usage | Reduce the effect row; weaken a usage requirement. | Add an effect; strengthen a usage requirement; change cancellation or cleanup obligations. |
| Contracts | Weaken a precondition; strengthen a postcondition; add a proved fact. | Strengthen a precondition; weaken a postcondition; change a law's meaning. |
| Laws | Add a law that consumers may use (additive). | Remove or weaken a law, or rebind it to a different operation dictionary. |
| Assumptions | Remove an unchecked assumption. | Add an unchecked assumption or a new axiom; make an existing assumption stronger. |
| ABI | Add a symbol. | Change layout, symbol identity, calling convention, or ownership convention. |
| Staging | Narrow capabilities; remove compile-time I/O. | Widen capabilities; add stage execution; add a generator whose inputs are not declared. |
| Targets | Relax a required feature. | Require a new target feature, vector shape, or instruction semantics. |

Version numbers are still useful: they order candidates, communicate changelog
intent, support yanking, and keep resolution cheap. They are filters and
labels, never proof. If a version says minor but `compat` disagrees, the check
wins and the release is reported as mislabeled.

# 3 The object model

| Object | Identity | Produced by | Mutable |
| --- | --- | --- | --- |
| Source tree | Merkle root over sorted path and byte digests | Packaging | No |
| Manifest | Canonical JSON bytes | The package author | Only with a new `package_id` |
| Release | `package_id` + human version + publisher signature | Publishing | No; a change is a new release |
| Withdrawal statement | Log sequence + issuer signature + scope | Publisher, registry, legal, or security role | No; superseded by a later statement |
| Module contract | `iface_hash` (local), `contract_digest` (strong, target-independent), `iface_digest(profile)` (strong, full interface) | Compiler | No |
| Instantiated module | `unit_id` | Compiler, per consumer | No |
| Build artifact | `artifact_id` (strong digest of the entry; `package_key` is its local cache key) | Compiler, per build policy | No |
| Attestation | DSSE envelope digest over an in-toto Statement; subject digest plus predicate type | Builder, verifier, registry, or publisher | No; a new attestation is a new entry |
| Expectations | Signed policy digest bound to the package name | Producer, registry default, or consumer override | Only as a new signed document |
| Verification summary | VSA predicate digest plus verifier identity | Registry or independent verifier | No |
| Lock entry | Release identity + source + policy + evidence profile + verified attestations | Resolver | Rewritten only by `update` |
| Store entry | `artifact_id` | Compiler | Cache only; never authoritative |

The dependency graph is over releases, not modules. Module imports stay in
source and are resolved inside the selected package closure. A package groups
modules for distribution; it creates no separate name scope, and module names
remain globally unique, as they are today.

# 4 Package shape and manifest schema 2

Proposed canonical manifest:

```json
{
  "schema": 2,
  "name": "acme/json",
  "version": "0.4.0",
  "license": "BSD-3-Clause",
  "modules": [
    { "name": "Acme.Json", "path": "src/Acme/Json.cinch", "facade": false }
  ],
  "order": ["Acme.Json"],
  "dependencies": [
    { "name": "cinch/base", "range": "^0.1.0", "source": "registry" },
    { "name": "acme/bytes", "source": "path", "path": "../bytes" },
    { "name": "acme/schema", "source": "git", "url": "https://example/acme-schema", "rev": "abc123" }
  ],
  "stage": { "policy": "deny" },
  "targets": { "requires": [] }
}
```

Rules for schema 2:

- `schema` must be read and checked. Today `Package::parse` ignores it, so a
  schema 2 manifest with dependencies would be accepted by a version 1 compiler
  and the dependencies silently dropped. Reject an unknown major with a named
  error before any registry exists.
- `name` is `namespace/name`: lowercase ASCII, `a-z`, `0-9`, `-`, and `_`, with
  exactly one `/`. Dots belong to module names. Namespace ownership is a
  registry fact, not a compiler fact.
- `modules` and `order` keep their current meaning. Imports remain absent from
  the manifest by design.
- Dependencies are package-level. A dependency entry has `name`, `source`, and
  either `range` (registry), `path` (path), or `url` plus `rev` (git). Optional
  `kind` is `runtime` (default), `stage`, or `test`.
- `stage.policy` is `deny` (default) or `allow` with a capability list. It
  declares what the package's own build needs, not what its consumers grant.
- `targets.requires` lists target features or profiles the package needs. It is
  checked per build; resolution stays target-independent.
- No feature flags. A feature changes the module and instance graph, so one
  version would have many interfaces. Use separate packages or explicit module
  parameters.
- No arbitrary build scripts. `stage` declarations are the only build-time
  execution and are capability-gated.
- `license` and `maintainers` are informational. The registry does not verify
  them, and the design must not imply that it does.

Canonicalization: UTF-8, the canonical JSON spelling already used by
`.cinchif`, sorted dependency lists by name, no timestamps, and no duplicate
module paths. `order` is a semantic list and stays in authored order.

The source tree the digest covers has its own rules: relative UTF-8 paths with
`/` separators, no absolute paths, no `..`, no duplicate paths, no symlinks
(reject or dereference at packaging with the target recorded), and file bytes
preserved exactly. Do not normalize line endings, permissions, or Unicode. A
package that builds differently because the packer rewrote a byte is not the
package the author published.

# 5 Dependencies, coherence, and resolution

## One version per package name

A build selects at most one release of each package name. The language makes
this the honest rule:

- Module names are global identities. Two versions of a package would declare
  two `Data.Map` types under one name, and an import could not choose correctly.
- Instance coherence is global and checked across modules. Nested duplicate
  versions would create two dictionaries for one class and target, which the
  checker already rejects as `ambiguous instance`.
- Instantiated module identity assumes one target module surface per module
  name. Coexisting versions would require unit-qualified module identities
  everywhere, which is a language change, not a package-manager convenience.

If two incompatible versions are genuinely needed, the supported escape hatches
are explicit module parameters or a fork published under a distinct name. This
is stricter than Cargo and npm, and it follows from the semantics rather than
from taste.

## Resolution

1. Read the index snapshot named by the lock, or the newest snapshot allowed by
   policy.
2. Filter candidates by declared source, range, target requirements, and stage
   policy.
3. Select one release per name, deterministically. Prefer the highest version
   whose `compat` check against the currently selected dependents holds.
4. Build the module index for the selected closure: scan every package's
   manifest modules and reject two selected packages that declare the same
   module name, naming both packages. This is the package-level form of the
   existing ambiguous-module diagnostic.
5. Check declared reachability: every module import that resolves to another
   package must have that package in the importing package's declared
   dependency closure. A source import without a declared dependency is an
   undeclared-dependency error, not an implicit fetch.
6. Check global coherence: every implicit instance in the selected closure must
   be unique for its class and target. A conflict is the checker's existing
   `ambiguous instance` diagnostic, naming both packages and both instances.
7. Check for cycles and name the full chain.
8. Write the lock.

Determinism rules: sort by package name and then `package_id`; never depend on
index order, wall-clock time, or network response order. Running resolution
twice on the same inputs must produce byte-identical locks.

The global check order is the selected dependency topological order, with each
package's own `order` applied inside its package. A package cycle is rejected
before checking.

Ranges are candidate filters only. Support `^` and exact equality in version 1.
`^` means the same major with at least the given minor, but the resolver still
runs `compat`; the check is authoritative. Avoid `*`, unions, and range
intersection cleverness until there is evidence they earn their complexity.

Test dependencies join only the test build's module graph. They do not enter the
production closure, and their instances do not participate in production
coherence. A test build that imports both still gets one global coherence check.

## Workspaces

A workspace root holds several package manifests and one `.cinchlock`. Path
dependencies between members are ordinary path dependencies; the lock is shared
and lives at the root. Version 1 may ship single-package projects first, but the
lock location and resolver identity should be workspace-shaped from the start,
so adding workspaces later does not migrate every project.

## Lockfile

`.cinchlock` is canonical JSON:

```json
{
  "schema": 1,
  "index": {
    "url": "https://packages.example/index",
    "snapshot": "sha256:...",
    "sequence": 1234
  },
  "policy": {
    "revocation": "strict",
    "slsa": {
      "build": "SLSA_BUILD_LEVEL_1",
      "source": "SLSA_SOURCE_LEVEL_2"
    },
    "overrides": []
  },
  "packages": [
    {
      "name": "acme/json",
      "version": "0.4.0",
      "package_id": "sha256:...",
      "source": { "kind": "registry", "registry": "https://packages.example" },
      "dependencies": ["cinch/base"],
      "stage": { "grant": [] },
      "evidence": "source-checked",
      "artifacts": []
    }
  ]
}
```

The lock records exact identities, never ranges. A path dependency is recorded
by its content root at lock time and re-checked; a dirty path is a named error
under `--locked`, not a silent rebuild. A git revision is a fetch hint; the
content root is the identity. `--locked` fails when resolution would differ.
`--frozen` additionally forbids network access.

An `artifacts` entry names the profile, the `artifact_id`, and the
`iface_digest` of every module in that build. A source-only project leaves the
list empty and derives the digests after checking. An artifact-only project
records them so the artifact's own claim is not the authority for what it
contains.

The `sequence` is the anti-rollback watermark: a later verification refuses a
snapshot below it. The `policy` section is recorded because a revocation mode or
an override is an input to the build identity; a lock written under one policy
does not silently inherit another.

# 6 The registry

## Static first

The registry is immutable blob storage plus an append-only signed index. There
is no server-side resolver and no mutable `latest`. The client fetches an index
snapshot, verifies it, resolves locally, and fetches blobs by content hash. This
makes a mirror untrusted transport rather than a trusted authority.

An index entry records:

| Field | Purpose |
| --- | --- |
| `name`, `version` | Human selection and display |
| `package_id` | Strong release identity |
| `manifest_digest`, `source_digest` | Independent checks on the canonical pieces |
| `contracts` | Module name to `contract_digest`, so source-level compatibility is checkable without downloading bodies |
| `interfaces` | Optional module name to per-profile `iface_digest` maps, for artifact consumers |
| `publisher` | Public key fingerprint, not a display name |
| `source` | Repository URI, revision digest, Source level, Source VSA digest |
| `build` | Trusted Build level and builder identities |
| `expectations` | Digest of the signed expectations document for this package name |
| `attestations` | Predicate types and digests, including SBOM and contract facts |
| `published_at` | Ordering and first-seen warnings, never build identity |
| `withdrawal` | The latest signed statement for this entry: kind, issuer, reason, and sequence |
| `successor` | Suggested replacement, displayed but never applied automatically |
| `supersedes` | Explicit replacement chain, for audit |

The index is a Merkle log. Clients verify inclusion of every selected entry and
consistency of the snapshot against a previously pinned root. This is what makes
withdrawals and takedowns auditable without trusting one server.

## Names and ownership

Use scoped names (`namespace/name`) with `cinch/` reserved for the language and
base packages. A namespace claim is a signed registry record. A display name is
a claim; the publisher key fingerprint is the identity.

Typosquat and dependency-confusion mitigations:

- Always show the publisher fingerprint and the first-seen date for a new
  dependency.
- Warn when a locked dependency's publisher key changes.
- Never fall back from one registry to another, or from a registry package to a
  path package, on a name miss.
- Keep the full `source` in every lock entry.

## Withdrawal, revocation, and takedown

Copyright, security, and legal action need a stronger mechanism than a
publisher's yank. The design must also be honest about what a registry can and
cannot enforce.

### The honest limit

Content addressing means bytes cannot be erased from mirrors, user caches,
vendored trees, or existing locks. A registry can stop serving a blob from its
own storage, refuse to resolve it, and publish a signed statement that clients
must honour. It cannot make other copies disappear, and it must not claim that
it did. Enforcement lives in the client policy that consumes the statement, not
in the deletion of a file.

This is a build-time control, not a runtime kill switch. A deployment already
running a revoked release continues until its operator acts; the runtime
evolution protocol owns rollout and safe points. No protocol can force a
modified client either: the guarantee is that an honest client with a
sufficiently fresh snapshot refuses to resolve or build the blocked release.

Typical actions:

| Situation | Statement | Scope |
| --- | --- | --- |
| Copyright claim against one copied release | `takedown` | `release` |
| Trademark or impersonation in a name | `takedown` plus `successor` | `package` or `namespace` |
| Malware in one build | `revoke` plus `successor` | `release` |
| Compromised publisher key | `key-revoke` | `publisher` |
| Entire namespace used for fraud | `suspend` then `takedown` | `namespace` |

### Statements, not flags

A withdrawal is an append-only signed statement in the index log. It has a
scope, an issuer role, a public reason class, and a monotonic sequence. Later
statements supersede earlier ones; nothing is erased.

Unregistering is therefore a signed scope statement plus a permanent tombstone,
not an `rm` of a blob. A registry can make a name unresolvable, but the log and
any lock that already names the release keep the evidence of what was removed
and why.

| Field | Meaning |
| --- | --- |
| `kind` | `deprecate`, `yank`, `suspend`, `takedown`, `revoke`, `key-revoke`, `restore` |
| `scope` | `release` (`package_id`), `version` (`name@version`), `package` (all versions), `publisher` (all releases by a key), or `namespace` |
| `affected` | The exact identities the statement applies to |
| `issuer` | Signing key plus role: publisher, registry, legal, or security |
| `sequence` | Monotonic log position; clients never accept a lower one |
| `reason` | Public class: `malware`, `compromised-key`, `copyright`, `trademark`, `legal-order`, `impersonation`, `vulnerability`, `obsolete`, `renamed` |
| `notice` | Optional hash of the full notice, which may stay off-log |
| `successor` | Optional suggested replacement; never applied automatically |
| `advisory` | Optional public advisory identifier |

Scopes make the action proportionate. A single malicious release is a `release`
statement; a compromised publisher key is a `publisher` or `key-revoke`
statement; a namespace used entirely for impersonation is a `namespace`
suspension. Module-level blocks are not supported: a module can be re-exported,
so the release is the enforceable unit.

### Takedown and restoration

A copyright, trademark, or legal-order takedown is issued by the registry's
legal role and names the affected releases. The notice text can be confidential;
the log records the public reason class and a commitment to the notice.

The state machine is explicit:

```
active -> takedown -> (counter-notice) -> restored | upheld
```

A counter-notice is signed by the publisher. Restoration is a new `restore`
statement signed by the legal role. A build blocked between the two statements
proceeds once the client sees the restore. The protocol records the states; the
registry's published policy owns statutory deadlines and jurisdiction.

Registry enforcement is bounded and explicit. A `revoke` or `takedown` stops the
primary registry from serving the blobs, removes the release from default search
and listings, and blocks new publishes of the same `(name, version)`. It does
not delete the log entry, the tombstone, or the quarantined bytes needed for a
counter-notice or legal process. A `publisher` or `namespace` suspension also
blocks new publishes under that key or namespace. A blocked name stays frozen
during a dispute; the legal role can name a `successor`, but it cannot silently
transfer the name. A block applies to the named identity, not to similar code: a
fork that copies the bytes is a different release and needs its own notice.

### Revocation and key compromise

A security revocation is issued by the security role. It blocks new resolution
and existing builds by default. A replacement release can be named as
`successor`, but the client never substitutes it silently: the bytes differ, so
the lock would change.

A `key-revoke` blocks every release signed by a publisher key. The default is
all releases by that key, not only those after a guessed compromise timestamp,
because a malicious release may predate detection. The registry can re-sign
vetted releases under a continuity key; that is a new signed index entry with a
continuity proof, not a silent rewrite. Root key rotation is cross-signed by the
old root and pinned by clients.

### Tombstones and name reuse

A withdrawn `(name, version)` pair is permanently tombstoned. Republishing the
same label with different bytes is refused, even after a takedown, so an
attacker cannot wait for a removal and re-upload a malicious build under the
same version. New bytes require a new version.

Name reclaim after a withdrawal is a dispute process, not a race. A `successor`
field suggests the replacement, whether a fixed release or a renamed package.
Resolution may display it, but following it is an explicit user action.

Legal erasure and audit pull in opposite directions. The log retains the
minimum needed to block and audit: the affected `package_id`, the reason class,
the notice hash, and the sequence. Descriptive metadata can be removed under a
data-minimization policy. The tombstone must not disappear, or the block can be
undone by re-publication.

### Client policy and overrides

Default behaviour is class-specific:

| Class | New resolution | Existing build | Local cache |
| --- | --- | --- | --- |
| `deprecate` | allow with warning | allow | keep |
| `yank` | refuse | allow with warning | keep |
| `suspend` | refuse | allow with warning; `strict` refuses | keep, quarantined |
| `takedown` | refuse | refuse | quarantine |
| `revoke` | refuse | refuse | quarantine |
| `key-revoke` | refuse | refuse | quarantine |
| `restore` | allow | allow | release quarantine |

A `strict` project policy escalates `suspend` to a build refusal. A `forensic`
mode exists for archives and incident response, and it requires an explicit
override for every blocked release.

An override is never implicit:

- It is declared per package in project policy, with a required reason and an
  audit record, or passed as a build flag with the same information.
- It is recorded in the build report and the lock, and it makes the build
  identity differ from a clean build.
- A `takedown` override requires an explicit legal-hold mode; a `revoke` override
  is allowed only with a recorded justification. Neither is the default.

A closure fails at the blocked node and names the path, for example
`app -> acme/json -> acme/bytes (revoked)`. A dependent package remains
resolvable as metadata but unbuildable while the block stands.

### Freshness and anti-rollback

A takedown is effective only when the client sees it. Snapshots carry a
monotonic `sequence`; the client stores the highest sequence seen per registry
and refuses a lower one, so a mirror cannot hide a statement by serving an older
consistent snapshot. The lock records the sequence and root it resolved against.

- `cinch pkg verify --online` refreshes the statement feed even when the lock is
  unchanged, and fails if any locked release is blocked under project policy.
- `--offline` uses the last snapshot and reports its age and the fact that newer
  statements may exist.
- A build farm can require freshness (`max_staleness`), so a build that cannot
  refresh the feed fails instead of silently using an old snapshot. CI should
  run `verify --online` with the strict policy.
- A vendor bundle includes the statement snapshot from vendor time. A later
  takedown requires a fresh feed; a vendored build cannot claim otherwise.
- Multiple index sources are supported and compared. A single mirror can delay a
  statement, but it cannot forge one or erase it from the log.

### Transparency and confidentiality

Every statement is in the Merkle log, so a silent block is a protocol violation
and a silent unblock is equally visible. Clients verify inclusion and
consistency against a pinned root.

Legal confidentiality and transparency collide. The log records the public
reason class and a hash of the notice; the full text can stay off-log and be
released to the affected publisher through the dispute process. A jurisdiction
that forbids disclosing even the reason uses the `legal-order` class. The log
still shows that a block occurred, which is the minimum needed for audit and
for distinguishing enforcement from censorship.

Registry actions are bounded: stop serving a blob from primary storage, refuse
new index entries, suspend a publisher or namespace, and sign statements. They
never include erasing a mirror, a cache, a vendor tree, or a lock.

## Publishing and private registries

Publishing verifies the package locally first: canonical manifest, strong
digests, interface check, no absolute paths or path traversal, the policy's
required SLSA provenance, and a signature over the index entry. The registry
verifies the hashes, the DSSE signature, the provenance subject and
expectations, and the index signature, and rejects non-canonical input or
below-policy provenance.

A private registry speaks the same protocol with a different index URL and trust
root. Private packages can also live in the same service as public ones, with
authorization at the metadata and blob endpoints. The lock records which
registry each package came from. Offline work uses `cinch pkg vendor`, which
writes the selected blobs and attestations into `vendor/` beside the lock;
`--offline` consumes only the lock, the store cache, and `vendor/`.

## The hosted service

The protocol needs a control plane. `registry/` is a v0 implementation: one Node
process with built-in SQLite, filesystem blobs, and no third-party dependencies.
It serves public and private packages from one service.

- **Public plane:** anonymous, cacheable index, search, and downloads. Public
  blobs can sit behind a CDN; the index is the same signed snapshot for every
  client.
- **Private plane:** authenticated `GET /index/private`, search, and downloads
  with `no-store`. A token sees public entries plus private entries in its
  namespace allowlist where its principal has a membership. Unauthorized private
  packages return 404, not 403, so existence does not leak. A blocked package
  returns 410.
- **Accounts:** principals are users or orgs. A namespace has owner, publisher,
  and reader memberships. Tokens carry `read`, `publish`, and `admin` scopes plus
  a namespace allowlist; they are stored as SHA-256 hashes and can be revoked.
- **Publish:** the service validates the manifest, module paths, file paths, and
  total size; computes `package_id`; rejects duplicates; stores the canonical
  bundle by digest; and stores attestations without verifying them in v0.
- **Signed events:** yank, takedown, and restore are in-toto statements in DSSE
  envelopes signed with ECDSA P-256 and SHA-256. `GET /registry-key` publishes
  the verification key.
- **Search and browse:** a public HTML index and package pages. Private access is
  API-only in v0.
- **Storage:** SQLite metadata plus content-addressed blobs. Production replaces
  SQLite with a replicated database and the filesystem with object storage.

Private-index scaling is the main deliberate shortcut. v0 filters one snapshot
per token. Production should sign one index per namespace and have a token fetch
the public index plus the indexes of its namespaces, so private membership does
not force per-request signing. The API surface is documented in
`registry/README.md`; it is a v0 contract, not the final protocol.

Production gaps are explicit: OIDC/SSO, email verification, quotas and rate
limits, malware scanning, SLSA provenance verification at publish and fetch,
per-namespace signed indexes, CDN and mirroring, billing for private packages,
legal takedown workflow, and namespace disputes. The service is replaceable: a
Rust or self-hosted Cinch implementation can serve the same contract.

# 7 Supply chain

The dangerous surfaces are compile-time execution and foreign code.

- Dependencies are staging-denied by default. The root package grants
  capabilities per dependency in the lock (`stage.grant`), such as read-only
  file access or subprocess execution with named targets. The grant is an input
  to `artifact_id`, so an artifact built under a different policy is a different
  artifact.
- Macro expansion is checked and budgeted, and needs no I/O. A dependency that
  runs a generator to produce declarations needs a grant. The build report names
  every stage declaration that executed, its captured inputs, and its replay
  mode.
- Foreign code is declared, not discovered. A package that links a foreign
  library records its FFI contract, target features, and assumptions, which the
  compiler already puts in the artifact manifest. The lock records the foreign
  dependency identity, and the consumer sees the transitive assumption set. The
  package manager does not install system libraries; a missing foreign
  requirement is a named build error, not an implicit system mutation.
- Source-only distribution is the default and the strongest evidence. A
  source-only release still carries Build L1 provenance for its source archive
  and a Source VSA when the source control system provides one; the consumer's
  compiler then checks the code. A distributed binary requires Build L2, because
  the consumer cannot recheck what it did not compile.
- Reproducibility is a verified property, not a claim. A release may assert
  `SLSA_BUILD_REPRODUCED` only when two independently operated builders produce
  identical artifact hashes for a named package, compiler, profile, and policy.
  That is evidence about the build, not proof of semantics.
- SLSA Build L1 provenance is required for every published release and every
  distributed artifact once a remote registry exists. If binary artifacts are
  distributed (version 1 defaults to source-only), they require Build L2. A
  local registry may operate with unsigned L1 during development, and a
  consumer that cannot recheck source must declare the lower evidence profile
  explicitly.
- `cinch pkg audit` reports staging grants, publisher changes, new assumptions
  or axioms, interface changes against the lock, every withdrawal statement
  that touches the closure, the issuer role and reason class, whether an
  override is in effect, the snapshot sequence and its age, SLSA Build and
  Source levels, builder identity changes, expectations mismatches, missing or
  unverified attestations, artifacts with no source, and changed target
  requirements.

License metadata is informational. Do not present the registry as a legal
review.

# 8 SLSA provenance and attestations

The supply-chain evidence layer is [SLSA v1.2](https://slsa.dev/spec/v1.2/). It
answers a different question from the interface layer: not "does this package
keep its contracts" but "were these bytes produced from the source and build
process we expect, and can that be proved". Both are required and neither
substitutes for the other.

The design adopts the SLSA recommended suite: DSSE envelopes over in-toto
Statement v1, JSON Lines bundles, ECDSA P-256 with SHA-256 or stronger, and the
SLSA provenance and Verification Summary predicates. A Cinch package manager
that emits only a proprietary JSON file is not SLSA-conformant even if the
fields look similar.

## 8.1 Mapping

| SLSA term | Cinch object |
| --- | --- |
| Artifact | A release source archive, a `.cinchif`, an object, an IR file, or an executable |
| Package name | `namespace/name` plus the publisher key set and the expectations document |
| Package artifact | A source archive identified by `package_id`, or a build output identified by `artifact_id` |
| Build | `cinch build` of one release for one target profile and build policy |
| Build platform | The hosted builder or CI that runs the build; the compiler is a dependency, not the platform |
| External parameters | `package_id`, entry module, target profile, build policy digest, adapter set, staging mode |
| Resolved dependencies | Compiler artifact, base release, package closure, captured staging inputs, foreign libraries, WIT adapters |
| Provenance | `https://slsa.dev/provenance/v1` in an in-toto Statement |
| Verification summary | `https://slsa.dev/verification_summary/v1` issued by the registry or an independent verifier |
| Distribution | The registry index and blob store |

SLSA treats the package name as the primary security boundary. This design keeps
that: expectations attach to `namespace/name` and its publisher keys, while
`package_id` pins the exact bytes a name resolved to. Content identity does not
replace the name; it makes the name-to-bytes mapping auditable.

## 8.2 Artifacts and attestations

Every attestation is an in-toto Statement with
`_type: https://in-toto.io/Statement/v1`, carried in a DSSE envelope whose
`payloadType` is `application/vnd.in-toto+json`. One
release produces many artifacts, and attestations bind to artifacts, not
releases. `subject.digest` is a SHA-256 digest of the artifact, and one artifact
may carry several attestations (provenance, VSA, SBOM, Cinch contract facts).
The registry stores the attestation blobs and the index records their digests.
The lock records the digests too, so the registry's claim is not the authority
for what the lock contains.

Attestations are immutable. A build with different inputs produces a new
`artifact_id`; a rebuild with identical inputs is expected to produce the same
one, which is what makes `SLSA_BUILD_REPRODUCED` checkable. A new attestation
for an existing artifact is a new log entry, never an overwrite. This follows
SLSA's distribution rule that attestations are bound to artifacts and should not
be replaced.

## 8.3 The Cinch build type

The build type URI is
`https://cinch.systems/buildtypes/cinch/v1`. It must resolve to a document that
defines the parameters, the schema, and unambiguous instructions for reproducing
the build. The placeholder host is the project site; the URI is versioned and
must not change meaning.

`externalParameters`:

```json
{
  "package": "sha256:...",
  "entry": "Main",
  "profile": "native-x86_64",
  "policy": "sha256:...",
  "adapters": ["canonical-abi"],
  "stageMode": "replay"
}
```

- `package` is the `package_id`, so the provenance is bound to the exact source
  tree and manifest.
- `entry` names the root module when the build has one. Library-only packages
  omit it.
- `profile` is the target profile identity already used by `artifact_id`.
- `policy` is the digest of the build-affecting policy: staging grants, target
  requirements, and adapter set. Verification requirements and revocation modes
  are not part of it; including them would make the artifact identity depend on
  how a consumer chooses to check it.
- `stageMode` is `refresh`, `replay`, or `live`, exactly as the staging design
  defines it. A `live` or `refresh` build records the captured inputs.

`resolvedDependencies` should be complete, not merely best effort, because the
compiler already computes the closure:

- the source release itself: the source archive digest, the `package_id`, and
  the VCS revision. SLSA treats source as a dependency, not just a parameter.
- the compiler artifact, by digest of the binary or the bootstrap stage;
- the embedded or selected `cinch/base` release;
- every package release in the closure, by `package_id`;
- every captured staging input (file, subprocess identity, schema snapshot),
  with its content digest; secrets are named but never included;
- foreign libraries, their versions, and the WIT adapters used.

`internalParameters` may record the store backend, worker identity, and cache
state for debugging. They are trusted because the platform is trusted, and they
are optional.

`runDetails` records `builder.id`, `builder.version`, `metadata.invocationId`,
`metadata.startedOn`, `metadata.finishedOn`, and `byproducts` such as logs,
coverage, or diagnostics with their digests. A verifier needs `builder.id` and
the builder's key to assign a level; timestamps and byproducts are evidence for
incident response, not identity.

`subject` lists every output with a media type:

| Output | Media type |
| --- | --- |
| Source archive | `application/vnd.cinch.package+tar` |
| Interface | `application/vnd.cinch.interface+json` |
| Object | `application/vnd.cinch.object` |
| IR | `application/vnd.cinch.ir+json` |
| Executable | `application/vnd.cinch.executable` |

The compiler can emit this because it already computes the build key, source
hashes, interface digests, and captured inputs. Add
`cinch build --emit-provenance`, which writes `<artifact>.intoto.jsonl`. A local
build emits unsigned L1 provenance by definition; the compiler must not pretend
to be a hosted platform.

## 8.4 Build and source levels

Build track:

| Level | Requirement | Cinch position |
| --- | --- | --- |
| L0 | None | A local build without provenance |
| L1 | Provenance exists | `--emit-provenance` on any host; unsigned and forgeable, useful against mistakes |
| L2 | Hosted platform generates and signs provenance | A hosted Cinch builder or third-party CI whose control plane, not the tenant build steps, produces the signature; the registry verifies it |
| L3 | Hardened, isolated, unforgeable | A platform assessment that also isolates the signing key from build steps; the compiler cannot claim it |

Source track:

| Level | Requirement | Cinch position |
| --- | --- | --- |
| L1 | Version control | A git or other VCS revision |
| L2 | History and source provenance | Source VSA from the SCS; continuous history and immutable revisions |
| L3 | Continuous technical controls | Organization controls enforced by the SCS; Source VSA carries the claim |
| L4 | Two-party review | Source VSA plus `SLSA_SOURCE_TWO_PARTY_REVIEWED` when the SCS enforces it |

A remote registry requires at least Build L1 provenance for every published
release and every distributed artifact. L1 is unsigned and forgeable, so the
registry checks structure and subject digests but cannot establish authenticity;
L1 prevents mistakes, not tampering. A distributed binary artifact requires
Build L2, because a source-only package can be checked by the consumer's
compiler while a binary cannot. L3 is opt-in through expectations; the compiler
does not self-certify it. Source requirements are separate and configurable; a
package with no Source VSA is Source L0, not implicitly trusted.

A source archive is still a package artifact: the packer transformed a VCS
revision into a tarball. SLSA's exemption for ecosystems that install directly
from a source repository does not apply to that archive, so it carries Build L1
provenance and, when available, a Source VSA.

## 8.5 Source provenance

Every registry entry records `source.repo`, `source.revision` (`gitCommit` or a
directory digest), `source.level`, and the digest of a Source VSA when one
exists. The release's source tree digest must be derivable from that revision; a
source archive built from a git commit records both `gitCommit` and a `dirHash`
in the provenance.

A Source VSA follows the SLSA source track:

- `subject.uri` points at a human-readable revision page;
- `subject.digest` includes the revision identifier, for example `gitCommit`;
- `subject.annotations.sourceRefs` lists fully qualified refs;
- `resourceUri` is the canonical repository URI;
- `verifiedLevels` contains the highest `SLSA_SOURCE_LEVEL_n`;
- `verificationResult` is `PASSED`.

Source provenance formats are SCS-specific in SLSA v1.2. The registry stores the
VSA and an opaque source provenance bundle, and verifies that the VSA subject
digest matches the release's source revision. A claim that the source was
reviewed is only as good as the SCS and the verifier behind the VSA; the
expectations document says which verifiers are accepted.

## 8.6 Expectations

Expectations are the part SLSA requires and most ecosystems omit. A valid
signature proves that someone built something; expectations prove it is the
right something for this package name.

An expectations document is signed and bound to the package name:

```json
{
  "package": "acme/json",
  "source": {
    "repo": "https://github.com/acme/json",
    "minLevel": "SLSA_SOURCE_LEVEL_2"
  },
  "build": {
    "builders": ["https://builder.example/cinch"],
    "minLevel": "SLSA_BUILD_LEVEL_2",
    "buildTypes": ["https://cinch.systems/buildtypes/cinch/v1"]
  },
  "externalParameters": {
    "profile": "native-x86_64",
    "stageMode": "replay"
  },
  "publishers": ["sha256:..."],
  "dependencies": { "minLevel": "SLSA_BUILD_LEVEL_1" }
}
```

- Expectations attach to a package name; provenance attaches to an artifact.
  The registry exposes the current expectations for each name, and each index
  entry records the digest that was in force at publish time.
- The producer can publish expectations; the registry has a default; the
  consumer can override locally. The consumer's policy wins.
- The policy digest is recorded in the lock and in any VSA made against it.
- A first release uses trust on first use. A later expectation change requires a
  signed update and is reported by `audit`; `verify --online` re-checks the
  closure against the current policy.
- Verification rejects unknown `externalParameters`, as SLSA recommends.
- Expectations can require `SLSA_BUILD_REPRODUCED` or
  `SLSA_SOURCE_TWO_PARTY_REVIEWED`.

Without this document, provenance verification degenerates into "some trusted
builder signed something", which does not bind a package name to its canonical
source or build process.

## 8.7 Verification

`cinch pkg verify` performs the SLSA verification steps by default when the
lock's policy carries SLSA requirements; `--slsa` forces them even when the
policy is silent. The steps are:

1. Verify the DSSE envelope signature against the configured roots of trust,
   mapping `(builder key, builder.id)` to a maximum build level.
2. Verify that `subject.digest` matches the artifact being installed or built.
3. Verify `predicateType` is `https://slsa.dev/provenance/v1`.
4. Check `builder.id` against the expectations and take the lower of the claimed
   and trusted levels.
5. Check `buildType` and `externalParameters` against expectations; reject
   unexpected parameters.
6. Check that the source revision and tree digest match the release and its
   Source VSA.
7. Verify `resolvedDependencies` recursively, or accept a VSA's
   `dependencyLevels` under a pinned verifier and policy.
8. Record the result, the policy digest, and the verifier identity in the lock
   and the build report.

Verification happens in three places, as SLSA recommends:

- the registry at publish time, for the benefit of every client;
- the client at fetch or build time, which also covers registry compromise,
  mirrors, and transit;
- an independent monitor for continuous checks and `audit`.

A registry VSA is a delegation, not a substitute for client policy. The client
pins the verifier identity and the policy digest; a VSA from an untrusted
verifier is only a claim. Failed verification is a named error, not a warning.

## 8.8 Verification Summary Attestations

The registry, or an independent verifier, issues a VSA per artifact or release
using `https://slsa.dev/verification_summary/v1`:

- `verifiedLevels` carries the highest Build and Source level and verified
  properties such as `SLSA_BUILD_REPRODUCED`;
- `dependencyLevels` counts transitive dependencies by level, so a policy can
  require every dependency to be at least L2 without fetching every bundle;
- `policy.uri` and `policy.digest` name the exact expectations document;
- `inputAttestations` lists the provenance digests used, so a closed-source
  producer can keep full provenance private;
- `slsaVersion` is `1.2`.

A VSA is useful when a consumer trusts the verifier and does not want to fetch
the whole bundle. It does not extend the verifier's trust; it only summarizes
what that verifier checked. The registry should publish VSAs, but clients must
still be able to verify provenance directly.

## 8.9 Reproducibility and verified properties

`SLSA_BUILD_REPRODUCED` is issued when two or more independently operated
builders, both trusted by the issuer, produce identical artifact digests for the
same `package_id`, profile, build policy, compiler identity, and adapter set.
Cinch's `artifact_id` is deterministic given those inputs, so the claim is
checkable rather than aspirational. The compiler's own bootstrap fixed point is
evidence for the compiler, not for every package.

`SLSA_SOURCE_TWO_PARTY_REVIEWED` is issued only when the Source L4
two-party-review requirement is enforced. Both are verified properties in
`verifiedLevels`, not new levels.

Do not invent new SLSA levels for Cinch proofs. A contract or proof fact is not
a supply-chain level. Emit a separate in-toto predicate,
`https://cinch.systems/attestation/contract/v1`, with the module name,
`contract_digest`, assumptions, and law status. Keep SLSA levels about integrity
and provenance.

## 8.10 Threat coverage and gaps

| Threat | SLSA mitigation | Cinch mechanism |
| --- | --- | --- |
| A. Producer | Not mitigated; a trust judgement | Publisher keys, namespace ownership, first-seen warnings, expectations, takedown |
| B. Modifying source | Source track L2 to L4 | Source VSA, protected refs, `SLSA_SOURCE_TWO_PARTY_REVIEWED` |
| C. Source code management | Source track plus SCS assessment | Source VSA from a trusted SCS, verifier expectations |
| D. External build parameters | Provenance plus expectations | `buildType`, `externalParameters`, source revision check |
| E. Build process | Build L2 and L3 | Hosted builder, DSSE signature, isolation assessment, reproducibility |
| F. Artifact publication | Signed provenance plus registry verification | Registry verifies subject, provenance, and expectations; transparency log |
| G. Distribution channel | Client-side verification | `verify --slsa` at fetch or build, lock-pinned digests, untrusted mirrors |
| H. Package selection | Not fully solved | Scoped names, publisher keys, expectations, typosquat warnings, no fallback |
| I. Usage | Client verification at use | Build-time verification, no ambient trust, revoked releases refused |
| Dependency threats | `resolvedDependencies` and `dependencyLevels` | SBOM, recursive verification, per-dependency level expectations |
| Availability threats | Not covered | Mirrors, vendoring, lock, cache, and anti-rollback sequences |

What SLSA does not cover:

- Compiler correctness. A compromised compiler can produce a malicious artifact
  with valid provenance. The bootstrap fixed point, verified primitives, and
  reproducible builds are separate evidence.
- Semantic correctness. A `.cinchif` says nothing about whether the binary was
  built from that source; SLSA provenance says nothing about whether the
  contracts are true. Both are needed.
- Package selection and typosquatting. Scoped names, publisher keys, first-seen
  warnings, and expectations reduce the risk; SLSA does not solve it.
- Build platform administrators. L3 protects against tenant and external
  tampering, not a malicious platform operator. The roots of trust must be
  chosen deliberately.
- Registry compromise. Client-side verification and transparency logs mitigate
  it; a registry that both serves artifacts and signs VSAs is still one trust
  anchor.
- Revocation. SLSA has no revocation mechanism. The withdrawal statements in
  section 6 are a Cinch extension that uses the same DSSE and in-toto envelope,
  so one verifier can process both.

## 8.11 Storage, index, and lock

Attestations are stored as content-addressed blobs and referenced by digest.
The registry publishes them beside the artifact, using
`<artifact>.intoto.jsonl` for a bundle. `cinch pkg vendor` copies the attestation
bundle as well as the artifact.

The index entry gains:

```json
{
  "source": {
    "repo": "https://github.com/acme/json",
    "revision": { "gitCommit": "..." },
    "level": "SLSA_SOURCE_LEVEL_3",
    "vsa": "sha256:..."
  },
  "build": {
    "level": "SLSA_BUILD_LEVEL_2",
    "builders": ["https://builder.example/cinch"]
  },
  "expectations": "sha256:...",
  "attestations": [
    { "type": "https://slsa.dev/provenance/v1", "digest": "sha256:..." }
  ]
}
```

Each package entry in `.cinchlock` gains the verified result per artifact, not
just the claim. The example is a distributed object at Build L2; a source-only
entry records L1:

```json
{
  "provenance": [
    {
      "artifact": "sha256:...",
      "predicate": "https://slsa.dev/provenance/v1",
      "digest": "sha256:...",
      "builder": "https://builder.example/cinch",
      "buildLevel": "SLSA_BUILD_LEVEL_2",
      "sourceLevel": "SLSA_SOURCE_LEVEL_3",
      "policy": "sha256:...",
      "verified": true
    }
  ],
  "vsa": {
    "digest": "sha256:...",
    "verifier": "https://registry.example/verifier"
  }
}
```

The Merkle log indexes attestation digests. An optional Sigstore/Rekor entry can
publish the same digest, but the registry log remains the primary record and the
client verifies whichever roots it trusts.

## 8.12 Command surface

The producer command is `cinch build --emit-provenance`, which writes
`<artifact>.intoto.jsonl` with Build L1 provenance. The consumer commands
(`verify --slsa`, `attest`, `provenance`, `sbom`, `audit`, and `publish`) appear
once, in the CLI table in section 11. There is no second SLSA-only spelling.

An SBOM is not required by SLSA, but the package closure is already known, and
SPDX or CycloneDX is the expected ecosystem artifact. Emit it as a separate
in-toto predicate bound to the release, using the SPDX predicate type
`https://spdx.dev/Document` or the CycloneDX predicate. The SBOM names the same
dependency releases that appear in `resolvedDependencies`, so the two can be
cross-checked.

## 8.13 Rollout

- P1: Build L1 provenance for local builds; the lock records artifact and
  provenance digests; no signatures.
- P2: DSSE signatures, Build L2 verification, Source VSAs, expectations, VSAs,
  and the transparency log; the registry rejects missing or mismatched
  provenance.
- P3: hardened builder profiles for L3, Sigstore keyless support,
  `SLSA_BUILD_REPRODUCED`, and key revocation with continuity.
- P4: the Cinch client verifies DSSE and in-toto itself.

The compiler can produce L1 provenance before any registry exists. That is the
first useful SLSA step and it does not depend on hosted infrastructure.

# 9 Proofs, laws, and transitive trust

A package interface carries contracts, law rows, and an assumptions manifest.
A consumer inherits that set, and the build report lists the closure. This is
the design conversation's rule: a module cannot claim a proved contract while
depending on an unchecked foreign contract hidden in a dependency.

Evidence profiles, recorded per package in the lock:

| Profile | What the consumer checked | What remains trusted |
| --- | --- | --- |
| `source-checked` | Source, interfaces, bodies, contracts, laws under the consumer's compiler | The compiler and runtime |
| `interface-checked` | `.cinchif` chain and recorded assumptions; code comes as objects | The producer's codegen and the object producer |
| `attested` | Reproducible artifact hashes and a declared builder | The builder, compiler, and runtime |
| `trusted` | Nothing beyond declared assumptions | The whole package, reported as an assumption |

The build refuses a silent downgrade: a lock that records `source-checked` and a
cache that can only offer an artifact gets a named error, not a quiet switch.

Laws bind to the operation dictionary, not the package name. If a dependency
changes an instance or a dictionary, dependent proofs and rewrites are
invalidated even when the type is unchanged. `iface_hash` covers contract and
law rows; `unit_id` covers instantiated dictionaries. A proof that survives a
type change but not a dictionary change is correctly rejected.

Erased proof bodies need not ship. A consumer either rechecks source or accepts
the interface's stated assumptions at a declared trust level. A `.cinchif` is
never evidence about machine code.

# 10 Versioning policy

`major.minor.patch` is a recommendation that the tooling checks:

- Patch: `contract_digest` is unchanged. Body-only change. If `contract_digest`
  moved, it is not a patch, even when every scheme reads the same.
- Minor: additive in both directions. New exports, new independent types, new
  defaulted methods that do not burden existing instances, new instances that
  cannot create ambiguity, relaxed target requirements, narrowed staging.
- Major: anything else. Removing exports, adding required methods, adding a
  constructor to a closed exported type, changing a scheme, a fixity, a
  contract direction, a law, an assumption, an ABI fact, or a target
  requirement.

Adding a method to a signature is safe for consumers and breaking for
implementors. Adding a constructor is the reverse in the other direction.
Because a package is consumed in both roles, the conservative classification
wins unless the checker proves the change additive in both directions.

Adding an instance is usually minor, but global coherence makes it conditional.
If the resolver cannot prove that no consumer closure gains an ambiguity, the
change is major.

Pre-1.0 versions get no exemption. `compat` governs at every version.

The base package is special only because the compiler ships it. The compiler
declares which `cinch/base` interface range and language version it supports,
and checks that at build time. The current embedded manifest is named
`cinch-base`; schema 2 should rename it to `cinch/base` or keep a permanent
alias, so scoped names are the only spelling users see. In version 1 the
embedded family is the only base: `embedded_source` is the only source table,
so a lock entry for `cinch/base` must match the compiler's supported release.
Selecting a source-distributed base is a later decision. Base bumps that change
the contract or ABI require a compiler that supports both, or an explicit
migration. Editions are deferred: interface schema and language version already
gate incompatibility, and an edition layer would add a second compatibility axis
without evidence that it is needed.

## Upgrades are packages too

Hot replacement already has a unit of distribution: the checked upgrade package
in the runtime evolution specification. Package management should reuse it
rather than invent a parallel format.

- An upgrade release has `kind: "upgrade"` and names the exact source release or
  releases, the target release, the migration, the compatibility evidence, the
  activation strategy, and the environmental assumptions.
- Deployment resolves a complete update set, not one package at a time. Every
  dependent whose inlined code, representation assumptions, or law identities
  change must be in the set; an incomplete set is rejected by the runtime
  coordinator. The package manager produces and verifies the set; the runtime
  performs activation at a safe point.
- A package update must not change a type equation under existing checked code.
  If callers reduced a public type-level definition, the change is breaking even
  when the runtime representation is unchanged.
- Record the update set in the deployment lock: artifact identities, profiles,
  evidence, dependency closure, and activation order. Do not treat a successful
  activation report as a static compatibility proof.
- Do not conflate resolution with rollout. The manager is offline and static;
  the runtime owns safe points, draining, cancellation, and coexistence.

# 11 CLI and files

Proposed surface, not implemented:

| Command | Behavior |
| --- | --- |
| `cinch pkg init` | Write a schema 2 manifest for the current package. |
| `cinch pkg add NAME` | Resolve a candidate, write the manifest entry and the lock entry. |
| `cinch pkg remove NAME` | Remove the declaration and re-resolve. |
| `cinch pkg fetch` | Fetch selected blobs into the store; no compilation. |
| `cinch pkg update [NAME]` | Re-resolve within declared ranges and rewrite the lock. |
| `cinch pkg verify` | Verify lock, store entries, interface chain, and evidence profile. |
| `cinch pkg verify --slsa` | Verify provenance, expectations, source and build levels, and VSAs. |
| `cinch pkg verify --online` | Refresh withdrawal statements and re-check the locked closure. |
| `cinch pkg audit` | Report supply-chain facts (sections 7 and 8), including withdrawal statements, SLSA levels, builder changes, and overrides. |
| `cinch pkg attest` | Bundle provenance, Source VSA, SBOM, and contract attestations. |
| `cinch pkg provenance NAME` | Print the verified provenance, policy, and levels for a locked release. |
| `cinch pkg sbom` | Emit an SPDX or CycloneDX document for the selected closure. |
| `cinch pkg vendor` | Write the selected blobs, attestations, and statement snapshot to `vendor/`. |
| `cinch pkg purge NAME` | Remove a blocked release from the local cache and vendor tree. Never touches the registry. |
| `cinch pkg publish` | Verify at the policy's SLSA level, canonicalize, sign, upload. |
| `cinch pkg yank VERSION` | Publisher signs a `yank` statement. |

The producer-side command is `cinch build --emit-provenance`, which writes an
in-toto bundle beside each output. The package commands consume and verify that
bundle; they do not invent provenance the compiler did not observe.

Takedown, revocation, suspension, and restoration are issued by the registry's
role keys through a separate signed admin tool. They are not ordinary publisher
commands, and the public CLI does not let a publisher issue them.

Build flags: `--locked`, `--frozen`, `--offline`. Missing lock under `--locked`,
changed resolution, missing blob under `--offline`, and verification failure are
hard errors that name the package and the hash. Package commands are a
subcommand of the existing driver, not a second driver.

Files: `.cinchpkg` at the package root, `.cinchlock` at the project root,
`vendor/` when vendoring, and the existing per-user store as a cache. The store
also keeps the highest verified statement sequence per registry; `purge` is the
only command that deletes local bytes, and it never touches the registry.

# 12 Phased plan

## P0: path dependencies and a lock, no network

1. Parse schema 2, check `schema`, accept `dependencies`, and keep schema 1
   working.
2. Add `.cinchlock` read, write, canonical form, and `--locked`.
3. Add path dependencies only. Compute the strong source-tree root and ingest
   the closure into the store.
4. Add a deterministic resolver with the single-version rule, cycle reporting,
   and the coherence check.
5. Tests: a three-package path graph; lock drift with `--locked`; a cycle; an
   ambiguity across two packages; a dirty path dependency; canonical lock bytes
   across two runs.

Exit criterion: a path-only project builds twice with identical artifact
identities, and `--locked` catches both a changed source and a changed
dependency.

## P1: local registry and L1 provenance

Content-addressed tarballs with canonical ordering and no mtime or uid; a
`file://` index; fetch, verify, and vendor. `cinch build --emit-provenance`
writes unsigned Build L1 in-toto provenance for the source archive and every
output, and the lock records the artifact and provenance digests. Exit
criterion: publish into a directory and consume from it with no server, and the
recorded provenance subject digests match the installed artifacts.

## P2: remote static registry and SLSA verification

HTTPS index and blobs, publisher signatures, Merkle inclusion and consistency
proofs, signed withdrawal statements, anti-rollback sequences, `audit`, and
`verify --online`. DSSE signatures, Build L2 verification, Source VSAs,
expectations, VSAs, and the transparency log. The registry rejects missing,
mismatched, or below-policy provenance. Exit criterion: substituting a mirror
does not require trusting it, a signed takedown blocks both a fresh resolve and
a locked build by default, a signed restore unblocks after a newer snapshot, and
a package with provenance for the wrong source or builder is rejected.

## P3: supply chain hardening

Stage grants, evidence profiles, reproducibility claims, publisher-change
policy, key revocation with continuity re-signing, namespace suspension, the
restoration workflow, audited overrides, and multiple index sources. Hardened
builder profiles for Build L3, Sigstore keyless support,
`SLSA_BUILD_REPRODUCED`, and Cinch contract attestations. Exit criterion: a
dependency that requests compile-time process access is refused by default and
reported when the root grants it, a revoked publisher key blocks its releases
while vetted releases can be re-signed under a continuity key without deleting
any blob, and a reproducibility claim is backed by two independent builders.

## P4: self-hosting

Port fetch, verify, and resolution to Cinch. The registry stays static files,
and the bootstrap ladder proves the client.

Non-goals throughout: server-side resolution, semver as proof, multiple versions
of one package name in one build, feature flags, and arbitrary build scripts.

# 13 Open decisions

Each needs a direction before P1.

1. Strong digest: SHA-256 or BLAKE3. Recommended: SHA-256 at the package layer,
   leaving `iface_hash` untouched.
2. One version per name. Recommended: yes, with module parameters or a rename as
   the escape hatch.
3. Scoped names. Recommended: `namespace/name`, with `cinch/` reserved.
4. Range syntax. Recommended: `^` and exact only in version 1.
5. Lockfile name and format. Recommended: `.cinchlock`, canonical JSON.
6. Stage default for dependencies. Recommended: deny, with explicit grants.
7. Binary distribution in version 1. Recommended: source-only.
8. Registry trust root. Recommended: separate role keys for registry, legal, and
   security actions, each under a documented threshold, with cross-signed root
   rotation. One key is one point of compromise and one point of censorship.
9. Re-export of dependency modules. Recommended: allowed and recorded; identity
   follows the defining module and re-export transfers no ownership.
10. Test dependency coherence. Recommended: test dependencies join only the test
    build, which still gets one global coherence check.
11. Multi-target locks. Recommended: one target-independent lock; target
    requirements are checked per build.
12. Compiler and base compatibility. Recommended: an explicit supported range in
    the compiler, not an edition mechanism.
13. Distribution of the shipped source modules. Recommended: publish them as
    family packages (`cinch/text`, `cinch/data`, `cinch/effect`) while the base
    stays embedded, so the library directory and the registry describe the same
    releases.
14. Base replacement. Recommended: defer; version 1 accepts only the embedded
    base, because the compiler's source table and the bootstrap checkpoints
    assume it.
15. Who can issue takedowns and revocations. Recommended: separate legal,
    security, and registry role keys; legal actions need a threshold, security
    malware blocks need one emergency key with mandatory logging and a review
    path.
16. Default behaviour for an existing lock. Recommended: refuse builds for
    `takedown`, `revoke`, and `key-revoke`; warn for `yank`; refuse new
    resolution for `suspend` and warn on existing builds.
17. Override ergonomics. Recommended: project policy plus an explicit build flag,
    both with a required reason, recorded in the lock and the build report.
    Legal takedowns additionally require a legal-hold mode.
18. Jurisdiction. Recommended: not in version 1. A registry may refuse to serve
    blobs from a jurisdiction, but the protocol must not pretend that clients
    elsewhere are bound by it.
19. Name reuse. Recommended: permanent `(name, version)` tombstones and a
    dispute process for reclaiming a name, never a first-come race.
20. Statement distribution. Recommended: one Merkle log with role signatures plus
    a compact delta feed, not several independent trust roots that can disagree.
21. Notice confidentiality. Recommended: public reason class plus a notice hash in
    the log, full text off-log and available to the affected publisher through
    the dispute process.
22. SLSA version and predicate pinning. Recommended: SLSA v1.2, the stable
    `https://slsa.dev/provenance/v1` and
    `https://slsa.dev/verification_summary/v1` predicate types, and an explicit
    `slsaVersion` in every VSA.
23. Default level requirements. Recommended: Build L1 for every published
    source release, Build L2 for any distributed binary (version 1 defaults to
    source-only), and Source L2 when a Source VSA exists. A project can require
    more, never silently less.
24. Who signs provenance. Recommended: the build platform signs; the compiler
    emits unsigned L1 provenance and never self-certifies L2 or L3.
25. Expectations governance. Recommended: producer-signed expectations, a
    registry default, and a consumer override; the consumer wins, and the policy
    digest is recorded in the lock and VSA.
26. Registry VSA role. Recommended: the registry may issue VSAs, but client-side
    verification is the default and is mandatory for mirrors; pin the verifier
    and the policy digest.
27. Roots of trust. Recommended: a map from `(builder key, builder.id)` to a
    maximum level, with optional Sigstore/Fulcio keyless support. Do not accept
    a self-claimed level without a trusted builder identity.
28. Provenance privacy. Recommended: full provenance may stay private; publish a
    VSA plus the `inputAttestations` digests. Never make a private build's
    provenance a publication requirement.
29. Transparency log. Recommended: the registry Merkle log is primary; a Rekor
    entry is an optional mirror. A client verifies whichever roots it trusts.
30. SBOM format. Recommended: emit SPDX by default and CycloneDX on request; the
    package closure is already known, so an SBOM is cheap and auditable.
31. Contract attestations. Recommended: a separate
    `https://cinch.systems/attestation/contract/v1` predicate, not a new SLSA
    level and not a field in `verifiedLevels`.

# 14 Rejected alternatives

- Semver-only resolution. A version cannot express a changed contract, law,
  instance, or ABI fact, and `compat` can.
- Nested duplicate versions, as in npm. Global module identity and instance
  coherence make duplication a semantic contradiction, not an inconvenience.
- Feature flags. They multiply interfaces for one version and make the resolved
  module graph depend on consumer configuration.
- Global installation. Packages are project-scoped; the store is a cache. A
  global mutable set defeats reproducible builds.
- Server-side resolution. It hides the choice, defeats mirroring, and lets the
  registry decide versions.
- Mutable tags and branches. A content root pins the bytes; a git revision is a
  fetch hint.
- Build scripts. Arbitrary build-time execution is the largest supply-chain
  hole. `stage` declarations under capabilities are the controlled form.
- Nix-style source pinning as the whole model. It gives reproducibility but
  ignores checked compatibility; it can be a transport layer, not the package
  semantics.
- Deleting a blob as the takedown mechanism. Mirrors, caches, vendor trees, and
  locks keep copies; deletion is not enforcement. A signed statement plus client
  policy is.
- Unsigned blocklists. A mirror or operator could censor silently and could not
  be audited. Statements are signed and logged.
- Automatic successor substitution. Replacing a revoked release with a successor
  changes the bytes and the lock; suggest it, never apply it.
- Time-expiring revocations. Malware and legal orders do not expire on a timer;
  restoration is an explicit signed statement.
- A proprietary provenance format. It loses in-toto tooling, DSSE verification,
  and the SLSA verifier ecosystem, and it duplicates a standard that already
  defines the fields.
- Registry-only verification. It does not cover registry compromise or transit;
  the client verifies too.
- Treating a publisher signature as Build L2. L2 is a claim by the build
  platform about the build, not a publisher signature over an upload.
- Self-claimed build levels. A local build cannot claim L2 or L3 because the
  signing key and isolation properties are absent.
- Overwriting an attestation. Attestations are immutable; a new build is a new
  artifact and a new attestation.
- Encoding proof or contract facts as SLSA levels. Levels describe supply-chain
  integrity, not semantics; use a separate predicate.

# 15 First slice, concretely

P0 touches five places:

- `compiler/src/driver.rs`: schema check, dependency types, path resolution
  ahead of the directory scan.
- `compiler/src/store.rs`: strong source-tree root, package ingestion, and
  artifact identity that includes the selected closure and build policy.
- `compiler/src/lock.rs` (new): canonical read and write for `.cinchlock`.
- `compiler/src/resolve.rs` (new): deterministic selection, cycle detection, and
  the coherence handoff to the checker.
- `compiler/tests/package_deps.rs` (new): the P0 cases listed above, plus a
  golden lock file so canonical bytes are pinned.

The compiler already has the right primitive for the coherence handoff: the
checker's `ambiguous instance` diagnostic. The resolver should call the same
check over the selected closure rather than reimplement instance unification in
the package layer.

When P0 lands, this page stops being a proposal and the current-behaviour text
belongs in [modules and packages](../reference/07-modules-and-packages.md) and
[artifacts and incremental compilation](../reference/18-artifacts-and-incremental.md).
