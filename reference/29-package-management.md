# Packages, manifests, and the registry

<span class="support-tag support-tag--planned">Partial</span> **`cinch pkg` installs registry, path, and git packages and verifies them locally; publishing works against the prototype registry; L2 builder-key trust and VSA issuance do not exist.** This page documents the commands as they run today, with their real output.

Three surfaces are easy to confuse, so keep them apart as you read:

- the Rust compiler and its `pkg` subcommands, which resolve, vendor, check, and lock;
- the runnable registry prototype in `registry/`, a Node process with SQLite and a blob store;
- the [package management design](../design/package-management.md), which describes release identity, checked compatibility, withdrawal, and provenance policy that the code has not reached.

# The manifest

A package manifest is a file named `.cinchpkg`: canonical JSON with the package name and version, a module list with source paths, and an explicit check order. The embedded base library ships one, and it is the reference example:

```json
{
  "schema": 1,
  "name": "cinch-base",
  "version": "0.1.0",
  "source_manifest": "the embedded prelude family (compiler/prelude/Prelude.cinch is the facade)",
  "modules": [
    { "name": "Data.Core", "path": "Data/Core.cinch", "embedded": true },
    { "name": "Prelude", "path": "Prelude.cinch", "embedded": true, "facade": true }
  ],
  "order": ["Data.Core", "Prelude"]
}
```

That excerpt is two of its 55 module rows, one of which carries `"facade": true`. The real file lists every module, and `order` names each of them exactly once.

| Field | Read by | Meaning |
| --- | --- | --- |
| `name` | Compiler and registry | `namespace/name`, exactly one `/`, both segments non-empty |
| `version` | Compiler and registry | Selection hint and vendor directory name, not identity |
| `modules[].name` | Compiler and registry | The module's identity, dotted |
| `modules[].path` | Compiler and registry | Relative path to the source inside the package |
| `modules[].facade` | Compiler | Marks the re-export facade, which fixes the check-order split |
| `modules[].embedded` | Nobody | Informational; the embedded source table decides whether a path has bytes |
| `modules[].exposed` | Compiler | Whether another package may import it, default `true`; see [Exposed and internal modules](#exposed-and-internal-modules) |
| `order` | Compiler | Check order; a name the target does not resolve is an error |
| `schema` | Compiler and registry | `1` for a fixed module list, `2` to allow `select`, `prelude`, and `link`; absent means `1` |
| `prelude` | Compiler | The module injected as an implicit import into every module of this package, `"default"`, or `"none"`; see [The prelude setting](#the-prelude-setting) |
| `select` | Compiler | Conditional modules and link words, evaluated against the compile target; see [Conditions on the target](#conditions-on-the-target) |
| `link` | Compiler and installer | Semantic link requirements; see [Link requirements](#link-requirements) |
| `raw_link` | Compiler | Package names whose raw link words this project consents to; only a root manifest's list is consulted |
| `local_link` | Compiler | Machine-specific link words; project configuration, never package metadata |
| `source_manifest` | Nobody | A human note in the base manifest |
| `dependencies` | Installer | Package dependencies; see below |

An unknown key inside a module entry, a `when` condition, or a `link` object is refused rather than ignored, because a misspelled field would silently drop the thing it was meant to declare:

```text
cinch: package manifest: when: unknown condition `ostype` (allowed: os, arch, os_version, libc, compiler, runtime_abi, target)
cinch: package manifest: link: unknown key `librarys` (allowed: libs, frameworks, rpaths, libdirs, dll_dirs, soname, whole_archive, import_libs, system_libs, pkgconfig, raw)
```

A manifest with no `order` installs locally, because the installer derives its module list from `modules` and only the compiler reads `order`. Write both fields anyway: a build needs the order, because the fixity pre-pass and the check/interface split both depend on it.

Manifest faults are named at the source:

```text
cinch pkg: path ../b4: package name must be namespace/name
cinch pkg: path ../b5: module M: /tmp/b5/Missing.cinch: No such file or directory (os error 2)
```

A version is parsed leniently: `1.0` is accepted and padded, and the vendor directory is `vendor/<namespace>/<name>/1.0`. The registry requires `major.minor.patch` with an optional prerelease and rejects the publish instead.

## Conditions on the target

A package that binds a system library, or that ships one implementation per operating system, says so with `select`: a list of entries, each a `when` condition plus the modules and link words it adds when that condition holds.

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

| Key | Values | Meaning |
| --- | --- | --- |
| `os` | `darwin`, `linux`, `windows` | The target's operating system. `macos` is accepted and normalised |
| `arch` | `aarch64`, `x86_64`, `arm64` | The target's architecture; `arm64` and `aarch64` are the same value, as are `amd64` and `x86_64` |
| `libc` | `glibc`, `musl`, `libSystem`, `msvcrt` | The C library the target links |
| `os_version` | a comparison: `>=14.0`, `<13`, `=13.4`, or a bare `13.4` for exact | The target platform version, compared numerically so `14.10` is greater than `14.9` |
| `compiler` | a comparison against the compiler's version | For a package that needs a language feature |
| `runtime_abi` | an exact version | The runtime ABI a prebuilt object must match |
| `target` | an exact triple | The escape hatch for a case the other keys cannot express |

Inside one `when`, keys are ANDed; a list value is ORed; a missing key matches anything. Every matching entry applies, and its modules and link words are unioned, so a package can state a base set plus a version override. Two entries that name one module with different paths are refused, and an entry that changes nothing is refused too.

The facts come from the compile target, never the build host:

- `--target <triple>` accepts this compiler's own triple (`aarch64-apple-darwin` on an Apple silicon Mac) and refuses anything else, because a cross compile needs a runtime built for the target: `target `x86_64-unknown-linux-gnu` is not this compiler's host target `aarch64-apple-darwin``.
- `--target-version <version>` states the platform version. A manifest condition on `os_version` refuses without it rather than reading the host's version, which would make the lock machine-specific:

```text
cinch: acme/watch: the manifest selects on `os_version >=14.0`, and the target states no platform version; pass --target-version (for example --target-version 14.0)
```

The resolved module set changes the package store key, so a package resolved for one target never satisfies a compile for another.

## Exposed and internal modules

A module entry without `exposed` is part of the package's surface, and another package may import it. `"exposed": false` makes it internal: it compiles as part of its package and is refused to an importer:

```text
cinch: module Internal is declared by package acme/lib and is not exposed; the package's internal module cannot be imported from Main
```

Modules of the *same* package may import each other freely, and a project with no manifest is unaffected, because ownership only exists where a manifest claims it.

## The prelude setting

`prelude` names the module imported implicitly at the top of every module of that package. The setting belongs to the package, so a dependency and its consumer can differ:

| Value | Effect |
| --- | --- |
| absent or `"default"` | The compiler's own prelude, the embedded `Prelude` facade |
| `"none"` | Nothing is injected. The preregistered type names stay in scope, because a signature needs them; nothing else does |
| `"Acme.Prelude"` | That module is injected. It must be a module the package declares or a dependency provides |

The injected module is a real dependency: it lands in the module's import set, in the check order, and in the recorded interface. A prelude module is written against the default prelude rather than importing itself, and it typically re-exports the default facade so its readers keep the operators:

<!-- cinch:check -->
```cinch
module Acme.Prelude where
  import Prelude
  export module Prelude

  export helper

  helper :: Int -> Int
  helper n = n + 1
```

Two paths to one declaration are one name, so a module may import both a re-export and the module it re-exports. Two *different* declarations under one local name are the error, and the message names both paths and both origins:

```text
cinch: the name `x` is imported from both A (A) and C (C): rename one (a selection like `import C (x as ...)`)
```

## Link requirements

A package that needs native code says what it needs in semantic fields, and the compiler turns them into link words, reports them per package, and records them in the artifact manifest.

| Tier | Fields | Understood | In the lock | Portable |
| --- | --- | --- | --- | --- |
| Semantic | `libs`, `system_libs`, `import_libs`, `frameworks`, `rpaths`, `libdirs`, `dll_dirs`, `soname`, `whole_archive`, `pkgconfig` | yes: validated, ordered, reported per package | resolved words | yes, when an `rpaths` entry is relative or `@loader_path`-based |

Two of those fields belong to one linker each, so a target that cannot express them is a refusal rather than a silent drop: `frameworks` is ld64's (`-framework`), and `soname` is GNU ld's (`-Wl,-soname`). A manifest that sets either unconditionally fails the build on the other platform, and the message says which `select` entry to add:

```text
cinch: acme/tls: asks for the framework(s) Security, and frameworks belong to the Darwin linker; select that entry on `os: ["darwin"]`
cinch: acme/tls: asks for soname libacme.so.1, which belongs to the GNU linker; select that entry on `os: ["linux"]`
```
| Raw | `raw.ldflags`, `raw.cflags` | no: recorded verbatim and reported | verbatim | only for one machine |
| Local | a root manifest's `local_link` | no | no, and the build records that it was used | no, by definition |

`--print-link-plan` reports every contribution with the package that asked for it, before anything is built:

```text
$ cinch build ./Main.cinch -o prog --print-link-plan
link plan: semantic     acme/native              -Wl,-rpath,@loader_path/lib -lm
```

The manifest records the same plan, so a built artifact states what it was linked against and whether a local override took part:

```json
"link": { "local": false, "raw": [], "words": ["-Wl,-rpath,@loader_path/lib", "-lm"] }
```

Raw words are a gate, not a default. A closure that contains them fails the build with the package, the words, and both ways to consent:

```text
cinch: acme/raw: the manifest asks for raw link words (-Wl,--as-needed); this project has not consented. Add "raw_link": ["acme/raw"] to the root .cinchpkg, or pass --allow-raw-link acme/raw
```

Consent is per package and comes from the root project, so a transitive dependency cannot smuggle words in behind a package the consumer trusted. Four kinds stay refused even under consent, because they change what the produced binary *is* rather than how a library is found:

```text
cinch: acme/raw: raw link word `-Wl,--script=evil.ld` is refused even with consent; it changes what the produced binary is, not how a library is found
```

The refused set is `-Wl,--script`, `--script`, `-nostdlib`, `-nostartfiles`, `-Wl,-e`, `-Xlinker`, and a response file (`@path`).

A `pkgconfig` entry is resolved **once**, at install time: `cinch pkg install` runs `pkg-config --modversion`, `--libs`, and `--cflags`, checks the `min` constraint, and records the version and words in `.cinchlock` under `native`. A build then links exactly what the install resolved and never re-reads the ambient system. A package that asks for pkg-config with no lock record is refused rather than linked without its library:

```text
cinch: acme/gio needs native libraries recorded in .cinchlock (gio), and this project has no record for it; run `cinch pkg install`
```

## Dependencies

`dependencies` is an array of rows, and the `source` field selects the kind. Imports are deliberately not copied into the manifest; they are read from source, so a stale manifest copy cannot disagree with the code.

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
    { "name": "acme/schema", "source": "git", "url": "https://example/acme-schema", "rev": "abc123" }
  ]
}
```

| `source` | Required fields | Resolution |
| --- | --- | --- |
| `registry` (default) | `version` or `range`, optional | Highest version satisfying every reachable constraint |
| `path` | `path` | Read from that directory, resolved against the directory of the manifest that declares it |
| `git` | `url` and `rev` | Cloned at `rev` into a cache under the store, then read |

A `path` or `git` row may also carry `version` or `range` as a hint. A hint that does not match the target manifest's version is a named error, because the version is not the identity. `cinch/base` and `cinch-base` are skipped everywhere; the base package is embedded.

Registry constraints are recomputed to a fixed point by `resolve_constraints`, so adding a later constraint can lower an already-chosen version, and one package name resolves to one version or the install fails without writing a partial lock.

# A two-module project, packaged

Create a library package, then a project that depends on it. The library is one module and a manifest:

```text
greet/
  .cinchpkg          name acme/greet, version 0.1.0, module Greet
  Greet.cinch        module Greet exporting greeting
```

The project has its own manifest and one source file that imports the library by module name:

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

That pair compiles as soon as `Greet.cinch` sits beside `Main.cinch`. To make it a dependency instead, install it:

```sh
cd app
cinch pkg install
```

```text
installed acme/greet 0.1.0
  identity: sha256:94762afc77ea6ece4f40e193a2c9dad1166966113598b6d76134f3276da1af0d
  source: /private/tmp/cinch-docs/modpackages/pkgwork/app/vendor/acme/greet/0.1.0
  object: /private/tmp/cinch-docs/modpackages/pkgwork/app/vendor/acme/greet/0.1.0/package.o
  abi: /private/tmp/cinch-docs/modpackages/pkgwork/app/vendor/acme/greet/0.1.0/package.abi.json
  interfaces: /private/tmp/cinch-docs/modpackages/pkgwork/app/vendor/acme/greet/0.1.0/build
```

The install wrote a vendor tree, a lock, and a package object:

```text
app/
  .cinchlock
  .cinchpkg
  Main.cinch
  build/                       the project's own interfaces and manifest
  vendor/acme/greet/0.1.0/
    .cinchpkg                  the manifest, copied
    Greet.cinch                the source, copied
    package.o                  the native object
    package.ll                 its LLVM text
    package.abi.json           the physical manifest
    provenance.jsonl           an in-toto SLSA statement, one line
    build/
      Greet.cinchif
      CinchPackageRoot.cinchif
      package.cinchif          the ABI manifest plus kind, modules, object
```

`build/package.cinchif` is `package.abi.json` with three fields added, so a consumer needs one artifact:

```json
{"kind": "package-execution", "modules": ["Greet"], "object": "package.o", "object_content_hash": "15519496164294781924"}
```

Then compile the project against it:

```sh
cinch run ./Main.cinch
```

```text
hello, world
```

The `./` matters. `import Greet` resolves by looking for a module of that name in the loaded set, then for `Greet.cinch` beside the entry, then by scanning the entry's directory recursively for a `module Greet` declaration. The vendor tree is only reachable through the scan, and the scan reads the entry path's directory, which is empty when the entry is written as a bare `Main.cinch`. With the bare form the program is refused:

```text
cinch: module Greet is imported, but Greet.cinch does not exist and no other source file beside the entry declares `module Greet`
```

## Identity and the lock

The install identity is a SHA-256 over a domain-separated text form: the canonical manifest, then the sorted `path` and `sha256` rows of the package's source files.

```text
sha256("package\n" + canonicalJson(manifest) + "\n" +
       sorted(path + "\t" + sha256(fileBytes)).join("\n"))
```

Recomputing it for the library above reproduces the reported identity byte for byte. The version string only names the vendor directory; two directories with different bytes and the same version are two different packages.

`.cinchlock` records one row per installed name:

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
      "provenance": {"digest": "sha256:e49042046e72b66a3a1cfa98d42680fee9cd9da809c8457662ce946124f79ab8", "path": "vendor/acme/greet/0.1.0/provenance.jsonl"}
    }
  ]
}
```

`source` keeps the authored spelling, not the resolved absolute path: a `path` row holds the `path` as written, a `git` row holds the `url` and `rev`. A token is never written here.

# Installing sources

| Command | What it installs |
| --- | --- |
| `cinch pkg add NAME@VERSION --registry URL` | That registry release and its registry dependencies |
| `cinch pkg add PATH_DIR` or `--path DIR` | The package whose `.cinchpkg` is at that directory |
| `cinch pkg add URL --git-rev REV` or `--git URL --git-rev REV` | The package at the checkout root of `URL` at `REV` |
| `cinch pkg install [--locked]` | Every dependency the project manifest declares |

`add` always needs an explicit source; `install` with no argument reads the project manifest. A bare positional that is an existing directory is treated as a path source; a bare URL is a git source only when `--git-rev` is present; anything else must be `NAME@VERSION`. `--path` and `--git` are mutually exclusive. A CLI path argument resolves against the current directory, while a `path` row inside a manifest resolves against that manifest's directory. Adding the project directory itself is refused:

```text
cinch pkg: path . names the project directory; a project is not its own dependency
```

## Git sources

A git dependency shells out to `git`, so `<path>`, `file://`, and https URLs all work and authentication belongs to your credential helper or SSH agent. The checkout is cached under `<store>/git-packages/<key>/`, where `<key>` is the first 32 hex characters of `sha256("<url>\n<rev>")`, with `<key>.identity` beside it holding the content root recorded when it was fetched.

```sh
cinch pkg add --git /tmp/pkgs/gitpkg --git-rev f773c1bdbc6f6213b878a3174e87a9c5b59e6f66
```

```text
installed acme/greet2 0.2.0
  identity: sha256:a458080b098e5f4ba73d9b5cc4f23dba35d94f81961682ae1570e29318c565ac
  source: …/app/vendor/acme/greet2/0.2.0
  object: …/app/vendor/acme/greet2/0.2.0/package.o
  abi: …/app/vendor/acme/greet2/0.2.0/package.abi.json
  interfaces: …/app/vendor/acme/greet2/0.2.0/build
```

A cached checkout with local changes is a named error rather than a rebuild, and the cache is keyed by URL and revision only for reuse: the package is still identified by its content root. Removing the cache directory is always safe.

## Locked installs

`--locked` refuses to change any lock entry, which is how a dirty path dependency is reported:

```text
cinch pkg: --locked: path dependency is dirty: .cinchlock records sha256:94762afc… for acme/greet 0.1.0, but path ../greet now has content root sha256:9b9cb772…; run `cinch pkg install` without --locked to update the lock
```

The unlocked form reinstalls from the edited directory and rewrites the lock. Nothing watches a path dependency, so re-run `install` after editing it.

# Closures and conflicts

Path, git, and registry packages share one closure, and the errors below are all raised before anything is written.

| Situation | Diagnostic |
| --- | --- |
| Two packages declare one module | `cinch pkg: module Shared is declared by both acme/x and acme/y; a build selects one package per module name` |
| One name, two contents | `cinch pkg: one version per package name: acme/x resolves to two different contents (path ../X and path ../../X2); keep one release of a package name in a closure` |
| A cycle | `cinch pkg: dependency cycle: acme/a -> acme/b -> acme/a` |
| A registry release carrying a local source | see below |
| An unsatisfiable range | `cinch pkg: no version of alice/greet2 satisfies ^0.9.0; available 0.3.0, 0.2.0` |

The registry case deserves its own paragraph. A registry release cannot disclose a usable local source, so a `path` or `git` row in registry metadata is rejected with an instruction rather than a guess:

```text
cinch pkg: dependency resolution for alice/localdep@0.1.0: registry package alice/localdep declares a path dependency on acme/greet. A registry release cannot carry a usable local source; add acme/greet to the project manifest as a path dependency (or install it with `cinch pkg add --path` or `cinch pkg add --git URL --git-rev REV`) and run `cinch pkg install` from the project root
```

A mixed closure is ordinary: a project manifest declaring one registry row and one `path` row installs both, and the project then imports both modules.

One limit is worth stating because it costs an afternoon. `pkg add` records an install in `.cinchlock` and does not edit `.cinchpkg`, so a later `pkg install` sees only the dependencies the manifest declares. Installing a second version of a name leaves the first version's directory under `vendor/`, and the next compile then sees two files declaring one module:

```text
cinch pkg: build package object: module Greet2 is imported, but several source files declare it (…/vendor/alice/greet2/0.2.0/Greet2.cinch, …/vendor/alice/greet2/0.3.0/Greet2.cinch); imports resolve to a unique module
```

Remove `vendor/<namespace>/<name>` before installing a different version of one package.

# Verifying an install

`cinch pkg verify` is a local integrity check: every lock entry against its installed `.cinchpkg`, source files, object hash, and interface files.

```sh
cinch pkg verify
```

```text
verified 1 package(s)
```

Add `--slsa` to also check the recorded provenance against the lock identity, the predicate type, the Cinch build type, the external package parameter, and the builder id:

```sh
cinch pkg verify --slsa
```

```text
verified 1 package(s)
  provenance: acme/greet@0.1.0 SLSA_BUILD_LEVEL_1
```

That is the honest level. The provenance the compiler writes is an unsigned in-toto statement and the builder id is `local:cinch/0.1.0`, so the claim is that a local build happened, not that a trusted builder performed it.

# Publishing

```sh
cinch pkg publish --registry http://127.0.0.1:8787 --project DIR --token TOKEN [--visibility public|private]
```

The command reads the project's `.cinchpkg` and its module files, computes the release identity, and posts the bundle with the same bearer token used for private reads.

```text
published alice/greet2 0.2.0
  package: sha256:899ae7de7ffc5bb9b3e3bb97d289a68577e9ba19d15c3c4acb65c8db7fd38dd6
```

<span class="support-tag support-tag--partial">Partial</span> **publish diagnostics.** The client invokes `curl -fsSL`, and `-f` discards the response body on an error status, so a rejected publish reports only the HTTP status:

```text
cinch pkg: POST http://127.0.0.1:8899/api/v1/packages: curl: (22) The requested URL returned error: 400
```

To find out why, query the same endpoint with `curl -s` yourself or read the registry log. A version that is not `major.minor.patch`, a duplicate release, a missing namespace, and a namespace you do not own all arrive this way.

`--token` or `CINCH_REGISTRY_TOKEN` supplies the credential and `CINCH_REGISTRY` the default URL. Tokens are sent only in the `authorization: Bearer` header and are never written to `.cinchlock` or printed.

# The local store

The compiler caches the embedded base package's checked interfaces in a per-user store keyed by content, compiler version, target profile, and the base package's own bytes. `CINCH_STORE` overrides the root; the platform defaults and the entry layout are documented on [artifacts and incremental compilation](18-artifacts-and-incremental.md).

The store is not a resolver and does not choose versions. It caches one package, `cinch-base`. An installed package's artifacts live under `vendor/` in the shape shown above, not in the store.

Git checkouts share that root, under `git-packages/`. That directory is the only part of the store a package install writes, and deleting it is always safe.

# The registry prototype

`registry/` is a runnable v0 service: one Node process using Node 24's built-in SQLite and HTTP support, with no third-party runtime dependencies.

```sh
cd registry
CINCH_REGISTRY_ADMIN_TOKEN=change-me node server.mjs
```

```text
Cinch registry listening on http://127.0.0.1:8787
Data directory: /tmp/cinch-reg-exp
Registry key: 820eabd5c9a6d156a899fde7b5fae26305d96b807d959ad3280c3dc28d311a01
```

| Variable | Default | Meaning |
| --- | --- | --- |
| `PORT` | `8787` | HTTP port |
| `CINCH_REGISTRY_DATA` | `registry/data` | SQLite database, blob store, and signing key |
| `CINCH_REGISTRY_ADMIN_TOKEN` | a random 24-byte value, printed at startup | Bootstrap admin token with `read`, `publish`, and `admin` scopes for `*` |

The admin token is rotated into the database on every start, and the startup line `Admin token (store it now): ...` appears only when the variable was unset. In a real deployment it belongs outside the data directory. `CINCH_PROVENANCE_KEYS` takes a JSON map of trusted signing keys; envelopes it can verify are reported at L2.

## State on disk

```text
data/
  registry.db          SQLite
  registry.db-shm      SQLite WAL sidecar
  registry.db-wal      SQLite WAL sidecar
  registry-key.json    ECDSA P-256 private key, mode 0600
  blobs/<aa>/<rest>    canonical package bundles, keyed by the bundle digest
```

The tables are `principals`, `namespaces`, `memberships`, `tokens`, `packages`, `attestations`, `statements`, `meta`, and `sqlite_sequence`. The blob path uses the first two hex characters of the bundle digest as a directory and the remaining 62 as the file name. That digest covers the canonical bundle `{schema, manifest, files, visibility, readme}`, which is a different value from the release identity: the release identity covers the manifest and source bytes, the bundle digest covers the uploaded envelope too.

## Routes

Every route below is registered in `server.mjs`; the ones marked ran were exercised against a local instance.

| Method | Path | Auth | Purpose | Ran |
| --- | --- | --- | --- | --- |
| `GET` | `/health` | none | Liveness | yes |
| `GET` | `/api/v1` | none | Service description and endpoint list | yes |
| `POST` | `/api/v1/principals` | none | Create a user and its namespace, return a token | yes |
| `POST` | `/api/v1/namespaces` | token | Create an organization namespace | yes |
| `POST` | `/api/v1/namespaces/:namespace/members` | owner or admin | Add or update a member | no |
| `POST` | `/api/v1/tokens` | token | Mint a scoped token | yes |
| `DELETE` | `/api/v1/tokens/:id` | owner or admin | Revoke a token | no |
| `POST` | `/api/v1/packages` | `publish` | Publish a release | yes |
| `GET` | `/api/v1/packages/:name` | optional | List versions visible to the caller | yes |
| `GET` | `/api/v1/packages/:name/:version` | optional | Metadata, manifest, files, attestations | yes |
| `GET` | `/api/v1/download/:id` | optional | Download the bundle by release identity | yes |
| `GET` | `/api/v1/index` | none | Public index snapshot | yes |
| `GET` | `/api/v1/index/private` | token | Public plus authorized private entries | no |
| `GET` | `/api/v1/search?q=` | optional | Public search, or authorized private results | yes |
| `GET` | `/api/v1/statements?since=` | optional | Signed yank, takedown, and restore events | yes |
| `GET` | `/api/v1/audit?since=` | admin | All registry events | no |
| `GET` | `/api/v1/registry-key` | none | ECDSA public key for statement verification | no |
| `POST` | `/api/v1/packages/:name/:version/yank` | `publish` | Mark a release yanked | yes |
| `POST` | `/api/v1/packages/:name/:version/takedown` | admin | Block a release | yes |
| `POST` | `/api/v1/packages/:name/:version/restore` | admin | Unblock a release | yes |

Package names contain `/`; encode it as `%2F` in the path. Two HTML pages exist for browsers: `/` lists public packages and `/packages/:name` shows one.

## Identity and access

The registry computes the release identity with the same formula the compiler uses, and refuses a client-supplied `packageId` that disagrees:

```text
sha256("package\n" + canonicalJson(manifest) + "\n" +
       sorted(path + "\t" + sha256(fileBytes)).join("\n"))
```

A **principal** is a user or an organization. A **namespace** is owned by a principal; a user's namespace equals its handle, and an organization's is explicit. **Memberships** are `owner`, `publisher`, or `reader`. A **token** carries scopes (`read`, `publish`, `admin`) and a namespace allowlist (`*` for admin), is stored only as a SHA-256 hash, and can be revoked.

Create a user and capture the token:

```sh
curl -s -X POST http://127.0.0.1:8787/api/v1/principals \
  -H 'content-type: application/json' \
  -d '{"handle":"alice"}'
```

```json
{"principal": {"id": "p_9190be806f177361ee704171", "handle": "alice", "kind": "user"},
 "namespace": "alice",
 "token": "8bd37b54861a5f896e57dfb7d9e17dddab771e8a0016eb8511af336503fa4a44"}
```

Private access requires all three of a `read` scope, the package's namespace in the token allowlist, and membership in that namespace. Any one missing yields `404`, not `403`, so the registry does not disclose that a private release exists.

## Publishing, downloading, and withdrawal

`POST /api/v1/packages` takes the manifest, the files as a base64 map, a visibility, an optional readme, and optional attestations. The response `entry` carries `packageId` and `bundle`; download with the 64 hex characters of the release identity:

```sh
curl -s http://127.0.0.1:8787/api/v1/download/899ae7de7ffc5bb9b3e3bb97d289a68577e9ba19d15c3c4acb65c8db7fd38dd6
```

A publisher can yank a version with `POST /api/v1/packages/:name/:version/yank`. The row keeps its bytes, gains a signed withdrawal statement, and stays in the index with `"yanked": true`; only blocked releases leave the index. An administrator can take a release down, which sets `blocked`, removes it from the index, and makes download return `410` while keeping the blob for audit and restoration. `restore` reverses it.

Each event is a signed in-toto statement in a DSSE envelope with predicate type `https://cinch.systems/attestation/withdrawal/v1`. `GET /api/v1/statements?since=` returns them and `GET /api/v1/registry-key` returns the ECDSA P-256 public key; the signature covers the DSSE pre-authentication encoding with SHA-256. Deleting a blob is not a takedown mechanism, because mirrors, caches, locks, and vendor trees can retain copies.

The client's own yank handling is narrower than that: an explicit `cinch pkg add NAME@VERSION` installs a yanked release, because only the range-resolution path filters yanked and blocked versions out of the candidate list.

## Attestations

The registry checks the structure of SLSA provenance against the release identity. An unsigned envelope is `SLSA_BUILD_LEVEL_1` with `"verified": false` and `"verification": "unsigned-l1"`. An envelope signed by a configured trusted key is `SLSA_BUILD_LEVEL_2` with `"verified": true`. VSA issuance is not implemented, and the client holds no builder keys, so `cinch pkg verify --slsa` can never report more than L1.

## Test suites

```sh
cd registry
npm test
```

The suite covers public publish and download, private namespaces, token scoping, duplicate and path-traversal rejection, yank, takedown, restore, DSSE signature verification, and revocation.

# Command surface

Implemented:

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

An unsupported command prints this usage and exits 2.

`cinch pkg info` reports what one installed package resolves to for a target, before anything is compiled: the selected modules, which of them are internal, the prelude, and what the link plan will add.

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

The compile commands take four flags that a package manifest's conditions and link fields depend on:

```text
--target <triple>           the target to build for; only this compiler's own triple is accepted
--target-version <version>  the platform version `os_version` conditions are evaluated against
--print-link-plan           report every contribution to the final link, with its package
--allow-raw-link NAME       consent to one package's raw link words, the flag form of `raw_link`
```


Not implemented, and proposed by the design:

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

There is no source replacement, override map, workspace, `--frozen`, or offline mode. `cinch build --emit-provenance`, an online `verify`, client-side L2 builder roots, and a remote audit command are future work; do not write them into build scripts. The supported compiler commands are on [using the Rust compiler](01-using-the-compiler.md).

# Status

| Surface | Status |
| --- | --- |
| Source modules beside an entry file | Available |
| Embedded `cinch-base` package manifest | Available |
| Project `.cinchpkg` install with `pkg install` | Available |
| Registry install with `pkg add` | Available for one resolved release |
| Path dependency sources | Available |
| Git dependency sources | Available for `<path>`, `file://`, and https URLs |
| Mixed registry, path, and git closure | Available |
| Vendored sources plus object, ABI, and interface generation | Available |
| Package-level execution `.cinchif` | Available |
| Registry dependencies with exact versions and semver ranges | Available, fixed-point intersection |
| Local integrity and L1 provenance verification | Available |
| Publishing to an authorized namespace | Available |
| L2 builder-key verification and VSA issuance | Not implemented |
| Registry HTTP prototype | Runnable in `registry/` |

Implementation: `compiler/src/pkg.rs`, `compiler/src/driver.rs`, `compiler/src/store.rs`, `compiler/prelude/cinch-base.cinchpkg`, `registry/server.mjs`, `registry/lib/store.mjs`, `registry/lib/canonical.mjs`, `registry/lib/provenance.mjs`, `registry/lib/signer.mjs`. Suites: `compiler/tests/package_manifest.rs`, `compiler/tests/package_store.rs`, `compiler/tests/canonical_artifacts.rs`, the `pkg.rs` unit tests, and `registry/test/registry.test.mjs`.
