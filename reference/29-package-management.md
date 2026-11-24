# Packages, manifests, and the registry

<span class="support-tag support-tag--partial">Partial</span> **`cinch pkg` resolves, vendors, compiles, and verifies registry, path, and git packages locally; publishing works against the local prototype registry.** Builder-key SLSA Level 2 verification and VSA issuance are not yet implemented.

Keep three components distinct when working with packages:

- **`cinch pkg`** in the compiler CLI: resolves closures, vendors sources, compiles objects and interfaces, writes `.cinchlock`, and verifies integrity locally.
- **`registry/`**: a runnable v0 HTTP registry prototype (Node 24 + built-in SQLite + content-addressed blob store).
- **The [package management design](../design/package-management.md)**: the full target design for provenance policy, withdrawal propagation, and release compatibility.

# The manifest (`.cinchpkg`)

A package manifest is a canonical JSON file named `.cinchpkg` declaring package metadata, module paths, check order, target conditions, link requirements, and dependencies. The embedded `cinch-base` library ships one as the canonical example:

```json
{
  "schema": 1,
  "name": "cinch-base",
  "version": "0.1.0",
  "modules": [
    { "name": "Data.Core", "path": "Data/Core.cinch", "embedded": true },
    { "name": "Prelude", "path": "Prelude.cinch", "embedded": true, "facade": true }
  ],
  "order": ["Data.Core", "Prelude"]
}
```

(The full `cinch-base` manifest lists all 55 embedded modules, each named once in `order`.)

| Field | Used by | Meaning |
| --- | --- | --- |
| `schema` | Compiler, registry | `1` (default) for a fixed module list; `2` enables `select`, `prelude`, and `link`. |
| `name` | Compiler, registry | `namespace/name` with exactly one `/` and both segments non-empty. |
| `version` | Compiler, registry | Selection hint and vendor directory name (not package identity). |
| `modules[].name` | Compiler, registry | Dotted module name. |
| `modules[].path` | Compiler, registry | Source path relative to the package root. |
| `modules[].facade` | Compiler | Marks the re-export facade for the check-order split. |
| `modules[].exposed` | Compiler | Whether external packages may import this module (default `true`). |
| `modules[].embedded` | Informational | Notes embedded base modules. |
| `order` | Compiler | Topological module check order; every resolved module must appear. |
| `prelude` | Compiler | Implicitly imported prelude module for this package (`"default"`, `"none"`, or a module name). |
| `select` | Compiler | Target-conditional modules and link requirements. |
| `link` | Compiler, installer | Semantic (or gated raw) native link requirements. |
| `raw_link` | Compiler | Root-manifest allowlist of package names permitted to supply raw link flags. |
| `local_link` | Compiler | Machine-local link overrides in a root project manifest (never package metadata). |
| `dependencies` | Installer | Package dependencies (`registry`, `path`, or `git`). |

Unknown keys inside module entries, `when` predicates, or `link` blocks are rejected explicitly rather than silently ignored:

```text
cinch: package manifest: when: unknown condition `ostype` (allowed: os, arch, os_version, libc, compiler, runtime_abi, target)
cinch: package manifest: link: unknown key `librarys` (allowed: libs, frameworks, rpaths, libdirs, dll_dirs, soname, whole_archive, import_libs, system_libs, pkgconfig, raw)
```

Although `cinch pkg install` can read a manifest without `order`, always declare `order`: building requires it for fixity resolution and interface splitting.

Local installs parse versions leniently (for example, `1.0` vendors under `vendor/<ns>/<name>/1.0`), whereas the registry strictly requires `major.minor.patch` (with optional prerelease) on publish.

## Target conditions (`select`)

Schema 2 manifests use `select` to add platform-specific modules or link requirements based on the **compile target** (never the build host):

```json
{
  "schema": 2,
  "name": "acme/watch",
  "version": "0.1.0",
  "modules": [ { "name": "Watch", "path": "Watch.cinch" } ],
  "order": ["Watch"],
  "select": [
    { "when": { "os": ["darwin"] },
      "modules": [ { "name": "Watch.Kevent", "path": "darwin/kevent.cinch", "exposed": true } ] },
    { "when": { "os": ["linux"], "libc": ["glibc"] },
      "modules": [ { "name": "Watch.Inotify", "path": "linux/inotify.cinch", "exposed": false } ] },
    { "when": { "os_version": ">=14.0" },
      "link": { "libs": ["dl"] } }
  ]
}
```

| Condition key | Accepted values | Meaning |
| --- | --- | --- |
| `os` | `darwin`, `linux`, `windows` (`macos` normalizes to `darwin`) | Target operating system |
| `arch` | `aarch64` (`arm64`), `x86_64` (`amd64`), `wasm32` | Target CPU architecture |
| `libc` | `glibc`, `musl`, `libSystem`, `msvcrt` | Target C library |
| `os_version` | `>=14.0`, `<13`, `=13.4`, or bare `13.4` | Target OS version (numeric segment comparison: `14.10 > 14.9`) |
| `compiler` | Version comparison | Minimum or exact compiler version |
| `runtime_abi` | Exact integer | Required runtime ABI version |
| `target` | Exact target triple | Escape hatch for exact triple matching |

Within a single `when`, keys are ANDed and list entries are ORed; omitted keys match any value. All matching `select` entries apply and union their modules and link requirements. Conflicting paths for the same module name, or no-op `select` entries, are rejected.

Target selection on the command line:

- `--target <triple>` selects one of the five supported target triples: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, or `wasm32-unknown-unknown`. `check`, `emit-ir`, and `pkg info` work for all five directly; `build` targeting a non-host triple additionally requires `--runtime-archive <path>` pointing to a `libcinchruntime.a` built for that target.
- `--target-version <version>` supplies the platform version. If a manifest selects on `os_version` without `--target-version`, compilation refuses rather than probing the host OS:

```text
cinch: acme/watch: the manifest selects on `os_version >=14.0`, and the target states no platform version; pass --target-version (for example --target-version 14.0)
```

The resolved module set feeds into the package store key, so target-specific resolutions never collide.

## Exposed and internal modules

Modules default to `"exposed": true`. Setting `"exposed": false` keeps a module private to its own package—siblings within the package may import it freely, while external importers are rejected:

```text
cinch: module Internal is declared by package acme/lib and is not exposed; the package's internal module cannot be imported from Main
```

## Custom preludes (`prelude`)

`prelude` controls the implicit import added to every module in that package (independently of dependencies or consumers):

| Setting | Effect |
| --- | --- |
| Absent or `"default"` | Implicitly imports the embedded `Prelude` facade |
| `"none"` | No implicit import (built-in type names remain in scope) |
| `"Acme.Prelude"` | Implicitly imports `Acme.Prelude` from this package or a dependency |

A custom prelude module itself compiles against the default prelude and usually re-exports `Prelude` alongside its own additions:

<!-- cinch:check -->
```cinch
module Acme.Prelude where
  import Prelude
  export module Prelude

  export helper

  helper :: Int -> Int
  helper n = n + 1
```

Importing the same underlying declaration through two re-export paths is valid. Importing two distinct declarations under the same local name is an ambiguity error naming both origins:

```text
cinch: the name `x` is imported from both A (A) and C (C): rename one (a selection like `import C (x as ...)`)
```

## Link requirements (`link`, `raw_link`, `local_link`)

Packages declare native link needs in three tiers:

| Tier | Manifest fields | Validated by compiler | Recorded in lock | Portable |
| --- | --- | --- | --- | --- |
| Semantic | `libs`, `system_libs`, `import_libs`, `frameworks`, `rpaths`, `libdirs`, `dll_dirs`, `soname`, `whole_archive`, `pkgconfig` | Yes (validated, ordered, attributed per package) | Resolved words | Yes (with relative or `@loader_path` rpaths) |
| Raw | `raw.ldflags`, `raw.cflags` | No (passed verbatim, gated by root consent) | Verbatim | Machine-specific |
| Local | Root manifest `local_link` | No | No (flagged as local override in build manifest) | No |

Linker-specific semantic fields are checked against the target rather than silently dropped: `frameworks` requires Darwin (`ld64`) and `soname` requires GNU ld (`linux`). Setting either unconditionally fails on other platforms and tells you which `select` predicate to add:

```text
cinch: acme/tls: asks for the framework(s) Security, and frameworks belong to the Darwin linker; select that entry on `os: ["darwin"]`
cinch: acme/tls: asks for soname libacme.so.1, which belongs to the GNU linker; select that entry on `os: ["linux"]`
```

Inspect every package's link contribution before building with `--print-link-plan`:

```text
$ cinch build ./Main.cinch -o prog --print-link-plan
link plan: semantic     acme/native              -Wl,-rpath,@loader_path/lib -lm
```

The build manifest records the resulting link summary (`"link": { "local": false, "raw": [], "words": [...] }`).

**Raw link gating:** if any dependency asks for `raw.ldflags` or `raw.cflags`, the build fails unless the **root project** explicitly consents either via `"raw_link": ["acme/raw"]` in the root `.cinchpkg` or via `--allow-raw-link acme/raw`:

```text
cinch: acme/raw: the manifest asks for raw link words (-Wl,--as-needed); this project has not consented. Add "raw_link": ["acme/raw"] to the root .cinchpkg, or pass --allow-raw-link acme/raw
```

Even with explicit consent, flags that alter binary structure rather than library lookup are always refused: `-Wl,--script`, `--script`, `-nostdlib`, `-nostartfiles`, `-Wl,-e`, `-Xlinker`, and `@response-file` arguments.

**`pkgconfig` resolution:** `cinch pkg install` runs `pkg-config` once at install time, checks any `min` version constraint, and pins the resolved version and flags in `.cinchlock` under `native`. Builds link only from the lock record and refuse to proceed if a `pkgconfig` entry has not been installed into `.cinchlock`.

## Dependencies

Declare dependencies in the `dependencies` array of `.cinchpkg`:

```json
{
  "schema": 2,
  "name": "acme/app",
  "version": "0.1.0",
  "modules": [{ "name": "Main", "path": "Main.cinch" }],
  "order": ["Main"],
  "dependencies": [
    { "name": "alice/greet2", "version": "0.2.0", "source": "registry" },
    { "name": "acme/greet", "source": "path", "path": "../greet" },
    { "name": "acme/schema", "source": "git", "url": "https://example.org/acme-schema", "rev": "abc123" }
  ]
}
```

| `source` | Required fields | Resolution |
| --- | --- | --- |
| `registry` (default) | `version` or `range` (optional) | Highest version satisfying the intersection of all reachable constraints |
| `path` | `path` | Directory resolved relative to the declaring manifest |
| `git` | `url`, `rev` | Cloned at `rev` into `<store>/git-packages/` |

Optional `version` or `range` fields on `path`/`git` rows act as validation checks against the target manifest's version. `cinch/base` and `cinch-base` are always satisfied by the embedded base package.

Registry version constraints are solved to a fixed point so every package name resolves to a single version across the closure, or installation fails atomically without writing a partial `.cinchlock`.

# Packaging walkthrough

Suppose a library `acme/greet` provides `Greet.cinch` and an application `acme/app` imports it:

<!-- cinch:project group=pkg-two-module file=Main.cinch exit=0 stdout="hello, world" -->
```cinch
module Main where
  import Greet (greeting)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = ioWrite (string-to-bytes (greeting "world"))
    in 0
```

<!-- cinch:project group=pkg-two-module file=Greet.cinch -->
```cinch
module Greet where
  export greeting

  greeting :: String -> String
  greeting name = string-concat "hello, " name
```

Running `cinch pkg install` inside `app/` vendors the package, compiles its interfaces and native object, emits SLSA L1 provenance, and updates `.cinchlock`:

```text
$ cinch pkg install
installed acme/greet 0.1.0
  identity: sha256:94762afc77ea6ece4f40e193a2c9dad1166966113598b6d76134f3276da1af0d
  source: /path/to/app/vendor/acme/greet/0.1.0
  object: /path/to/app/vendor/acme/greet/0.1.0/package.o
  abi: /path/to/app/vendor/acme/greet/0.1.0/package.abi.json
  interfaces: /path/to/app/vendor/acme/greet/0.1.0/build
```

Resulting project layout:

```text
app/
  .cinchlock
  .cinchpkg
  Main.cinch
  vendor/acme/greet/0.1.0/
    .cinchpkg                  copied manifest
    Greet.cinch                copied source
    package.o                  compiled native object
    package.ll                 LLVM IR
    package.abi.json           physical ABI manifest
    provenance.jsonl           unsigned in-toto SLSA statement
    build/
      Greet.cinchif
      CinchPackageRoot.cinchif
      package.cinchif          package execution interface + object hash
```

Run the project with an explicit relative path so module scanning locates `vendor/`:

```sh
cinch run ./Main.cinch
# hello, world
```

Passing bare `Main.cinch` (without `./`) has an empty parent directory path and will fail to discover `vendor/` modules.

## Package identity and `.cinchlock`

Package identity is a content-addressed SHA-256 hash over the canonical manifest and sorted source file hashes:

```text
sha256("package\n" + canonicalJson(manifest) + "\n" +
       sorted(path + "\t" + sha256(fileBytes)).join("\n"))
```

Two checkouts with identical version strings but different bytes have different identities. `.cinchlock` pins each installed package to its content identity and authored source reference (never storing auth tokens or machine-specific absolute paths for relative dependencies):

```json
{
  "schema": 1,
  "packages": [
    {
      "name": "acme/greet",
      "version": "0.1.0",
      "package_id": "sha256:94762afc77ea6ece4f40e193a2c9dad1166966113598b6d76134f3276da1af0d",
      "source": {"kind": "path", "path": "../greet"},
      "source_dir": "vendor/acme/greet/0.1.0",
      "object": "vendor/acme/greet/0.1.0/package.o",
      "interfaces": "vendor/acme/greet/0.1.0/build",
      "provenance": {
        "digest": "sha256:e49042046e72b66a3a1cfa98d42680fee9cd9da809c8457662ce946124f79ab8",
        "path": "vendor/acme/greet/0.1.0/provenance.jsonl"
      }
    }
  ]
}
```

# Installing and locking dependencies

| Command | Action |
| --- | --- |
| `cinch pkg add NAME@VERSION [--registry URL]` | Install a registry release and its transitive registry dependencies |
| `cinch pkg add PATH_DIR` or `--path DIR` | Install a local directory containing `.cinchpkg` |
| `cinch pkg add URL --git-rev REV` or `--git URL --git-rev REV` | Clone `URL` at `REV` and install from the checkout root |
| `cinch pkg install [--locked]` | Install every dependency declared in the project `.cinchpkg` |

Notes on `add` and `install`:

- CLI `--path` arguments resolve relative to the current working directory; `path` dependencies inside a `.cinchpkg` resolve relative to that manifest's directory.
- Git sources shell out to `git` (inheriting your SSH agent and credential helpers) and cache clean checkouts under `<store>/git-packages/<sha256(url\nrev)[:32]>/`. Deleting the git cache is always safe.
- `cinch pkg install --locked` refuses any lockfile modification, failing cleanly when a path dependency's content hash has changed since the last install:

```text
cinch pkg: --locked: path dependency is dirty: .cinchlock records sha256:94762afc... for acme/greet 0.1.0, but path ../greet now has content root sha256:9b9cb772...; run `cinch pkg install` without --locked to update the lock
```

- `cinch pkg add` updates `.cinchlock` and `vendor/` without editing `.cinchpkg`. When switching an already-vendored package to a different version, delete `vendor/<namespace>/<name>` first so two versions declaring the same module name do not coexist in `vendor/`.

## Closure conflict diagnostics

All closure errors are detected before anything is written to `vendor/` or `.cinchlock`:

| Conflict | Diagnostic |
| --- | --- |
| Duplicate module name across packages | `module Shared is declared by both acme/x and acme/y; a build selects one package per module name` |
| Same package name with different content | `one version per package name: acme/x resolves to two different contents (...); keep one release of a package name in a closure` |
| Dependency cycle | `dependency cycle: acme/a -> acme/b -> acme/a` |
| Unsatisfiable version range | `no version of alice/greet2 satisfies ^0.9.0; available 0.3.0, 0.2.0` |
| Registry package declaring a `path` or `git` dep | Rejected with instructions to declare the local/git dependency in the root manifest instead |

# Verifying installs

`cinch pkg verify` rechecks every `.cinchlock` entry against its vendored `.cinchpkg`, source hashes, object hash, and compiled interfaces:

```text
$ cinch pkg verify --slsa
verified 1 package(s)
  provenance: acme/greet@0.1.0 SLSA_BUILD_LEVEL_1
```

Adding `--slsa` additionally checks `provenance.jsonl` against the lock identity, predicate type, Cinch build type, package parameters, and builder ID. Local builds emit an unsigned in-toto statement with builder ID `local:cinch/0.1.0`, corresponding to **SLSA Build Level 1**.

# Publishing

```sh
cinch pkg publish --registry http://127.0.0.1:8787 --project DIR --token TOKEN [--visibility public|private]
```

`--token` (or `CINCH_REGISTRY_TOKEN`) supplies the bearer token, and `--registry` (or `CINCH_REGISTRY`) sets the registry URL. Tokens are sent only in `authorization: Bearer` headers and are never written to `.cinchlock`.

<span class="support-tag support-tag--partial">Partial</span> **Error reporting on publish:** the CLI invokes `curl -fsSL`, which suppresses the HTTP response body on 4xx/5xx responses (reporting only `curl: (22) The requested URL returned error: 400`). Check the registry log or replay the request with `curl -s` to see the JSON error detail (such as a non-semver version, duplicate release, or unauthorized namespace).

# The local store

The compiler caches compiled interfaces for the embedded `cinch-base` package in a per-user store (overridable via `CINCH_STORE`; see [artifacts and incremental compilation](18-artifacts-and-incremental.md)). Git checkouts for `cinch pkg` are cached under `<store>/git-packages/`. Installed third-party packages themselves live inside each project's `vendor/` directory, not in the global store.

# The prototype registry (`registry/`)

`registry/` contains a runnable zero-dependency v0 registry service built on Node 24's built-in HTTP server and `node:sqlite`:

```sh
cd registry
CINCH_REGISTRY_ADMIN_TOKEN=change-me node server.mjs
```

| Environment variable | Default | Purpose |
| --- | --- | --- |
| `PORT` | `8787` | Listen port |
| `CINCH_REGISTRY_DATA` | `registry/data` | Directory holding `registry.db`, `registry-key.json` (ECDSA P-256, mode `0600`), and `blobs/<aa>/<rest>` |
| `CINCH_REGISTRY_ADMIN_TOKEN` | Random 24-byte token printed on startup | Bootstrap admin token with `read`, `publish`, `admin` on `*` |
| `CINCH_PROVENANCE_KEYS` | unset | Optional JSON map of trusted builder signing keys for SLSA L2 envelope verification |

## HTTP API routes

Package names containing `/` are URL-encoded as `%2F` in path segments.

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `GET` | `/health` | none | Liveness probe |
| `GET` | `/api/v1` | none | Service metadata and endpoint index |
| `POST` | `/api/v1/principals` | none | Create user + personal namespace, returning an initial token |
| `POST` | `/api/v1/namespaces` | token | Create an organization namespace |
| `POST` | `/api/v1/namespaces/:namespace/members` | owner/admin | Add or update a namespace member (`owner`, `publisher`, `reader`) |
| `POST` | `/api/v1/tokens` | token | Mint a scoped token (`read`, `publish`, `admin`) |
| `DELETE` | `/api/v1/tokens/:id` | owner/admin | Revoke a token |
| `POST` | `/api/v1/packages` | `publish` | Publish a canonical package bundle |
| `GET` | `/api/v1/packages/:name` | optional | List versions visible to the caller |
| `GET` | `/api/v1/packages/:name/:version` | optional | Fetch release metadata, manifest, files, and attestations |
| `GET` | `/api/v1/download/:id` | optional | Download a bundle by 64-hex-char release identity |
| `GET` | `/api/v1/index` | none | Public package index |
| `GET` | `/api/v1/index/private` | token | Public plus authorized private entries |
| `GET` | `/api/v1/search?q=` | optional | Search public (and authorized private) packages |
| `GET` | `/api/v1/statements?since=` | optional | Signed DSSE withdrawal/restore statements |
| `GET` | `/api/v1/audit?since=` | admin | Full registry audit event stream |
| `GET` | `/api/v1/registry-key` | none | ECDSA P-256 public key for verifying DSSE statements |
| `POST` | `/api/v1/packages/:name/:version/yank` | `publish` | Mark a release yanked (stays downloadable, excluded from range resolution) |
| `POST` | `/api/v1/packages/:name/:version/takedown` | admin | Block a release (`410 Gone` on download, removed from index, blob retained for audit) |
| `POST` | `/api/v1/packages/:name/:version/restore` | admin | Restore a taken-down release |

Browser views are served at `/` (public catalog) and `/packages/:name`.

Private reads require a valid `read` scope, namespace allowlist match, and namespace membership; unauthorized requests return `404` rather than `403` to avoid disclosing private package existence.

Withdrawal events (`yank`, `takedown`, `restore`) emit DSSE-signed in-toto statements (`https://cinch.systems/attestation/withdrawal/v1`). Note that on the CLI side, range resolution excludes yanked releases while an explicit `cinch pkg add NAME@VERSION` still permits installing a pinned yanked version.

Run the registry test suite with `cd registry && npm test`.

# Command reference

## Implemented commands

```text
cinch pkg add NAME@VERSION [--registry URL] [--project DIR] [--token TOKEN]
cinch pkg add PATH_DIR [--project DIR]
cinch pkg add URL --git-rev REV [--project DIR]
cinch pkg add --path DIR [--project DIR]
cinch pkg add --git URL --git-rev REV [--project DIR]
cinch pkg install [--project DIR] [--registry URL] [--token TOKEN] [--locked]
cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
cinch pkg verify [--slsa] [--project DIR]
cinch pkg info NAME [--project DIR] [--target TRIPLE] [--target-version VERSION]
```

`cinch pkg info` prints how an installed package resolves for a target (selected modules, visibility, prelude, and link plan) before compilation:

```text
$ cinch pkg info acme/watch --target-version 14.0
acme/watch 0.1.0
target: aarch64-apple-darwin
target version: 14.0
schema: 2
prelude: default
modules:
  Watch from Watch.cinch
  Watch.Kevent from darwin/kevent.cinch
select:
  0: os=darwin (applies, 1 module(s))
  1: os=linux, libc=glibc (does not apply, 1 module(s))
link: nothing
```

Package-aware compiler flags:

```text
--target <triple>           target triple (aarch64-apple-darwin, x86_64-apple-darwin,
                            x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, or
                            wasm32-unknown-unknown)
--runtime-archive <path>    target cinchruntime archive required by cross-target `build`
--target-version <version>  platform version evaluated by `os_version` conditions
--print-link-plan           print every package's link contribution before building
--allow-raw-link NAME       consent to one package's raw link words (CLI form of `raw_link`)
```

## Planned commands

<span class="support-tag support-tag--planned">Coming soon</span> Proposed by the package design but not yet implemented: `pkg init`, `remove`, `fetch`, `update`, `audit`, `attest`, `provenance`, `sbom`, `vendor`, `purge`, `yank`, workspaces, `--frozen`, offline mode, client-side L2 builder trust roots, and VSA issuance.

# Feature status

| Surface | Status |
| --- | --- |
| Project `.cinchpkg` (schema 1 & 2), target `select`, `prelude`, `exposed` | **Available** |
| Semantic link plans, `pkgconfig` locking, gated `raw_link` | **Available** |
| Registry, `path`, and `git` installs with fixed-point range resolution | **Available** |
| Vendoring, package object/ABI/interface builds, `.cinchlock` | **Available** |
| Local integrity and SLSA L1 provenance verification (`pkg verify --slsa`) | **Available** |
| Publishing to authorized namespaces | **Available** (HTTP error bodies truncated by `curl -f`) |
| Local HTTP registry prototype (`registry/`) | **Available** |
| SLSA L2 builder-key verification, VSA issuance, workspaces | **Not implemented** |

See [using the compiler](01-using-the-compiler.md) for the full compiler CLI and [artifacts and incremental compilation](18-artifacts-and-incremental.md) for store layout and `.cinchif` artifacts.
