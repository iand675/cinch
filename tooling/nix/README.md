# The bootstrap ladder as Nix outputs

A cold chain is two release cargo builds (the seed and the library
producer), the library object, three stage compiles and their links. These
derivations cut that into units a binary cache can serve separately, by what
invalidates each.

| Output | Key | Contents |
| --- | --- | --- |
| `cinch-seed`, `cinch-provider` | a pinned revision and the Rust toolchain | the pinned Rust compiler |
| `cinch-pins` | both pins and the bootstrap driver | `stage0`, the source snapshots, the library object and runtime |
| `cinch-ladder` | the compiler sources at this commit | `cinch-pins` plus stages 1 to 3, after `bootstrap.py check` |

## Using a cache

```sh
cachix use cinch                      # once per machine

# Everything the pins determine, copied into a writable build tree. The next
# `tooling/bootstrap.py build` starts at stage 1.
nix build .#cinch-pins -o build/pins
python3 tooling/bootstrap.py hydrate --from build/pins

# The checked stages for this commit, used where they are.
nix build .#cinch-ladder -o build/ladder
CINCH_BOOTSTRAP_BUILD=$(readlink -f build/ladder) \
  python3 tooling/run_stage_targets.py --stage 3 --out build/evidence ...
```

`cinch-pins` can be copied because its receipts hold hashes and no paths.
The seed inside it reads its runtime from the directory it was compiled in,
which for these outputs is the `cinch-seed` store path, so that path stays
in the closure and nothing has to be at a fixed location on the machine.
`cinch-ladder` is not: a stage 2 or 3 receipt covers `objects.json`, which
names the library object by absolute path, so the derivation builds with
`CINCH_BOOTSTRAP_BUILD` set to its own output and the tree is valid only
there. It is read-only, which suits a test run and not a rebuild; to iterate
on compiler sources, hydrate the pins and build.

Hydration is advisory. Every consumer re-verifies what it finds against the
pins, so a tree built for other pins is rebuilt, never trusted.

## Filling a cache

`.github/workflows/ladder-cache.yml` builds and checks native distributions
for `x86_64-linux`, `aarch64-linux`, and `aarch64-darwin` on relevant
pushes, pull requests, and manual runs. Every job pulls from the public
`cinch` cache. Trusted push/manual/scheduled jobs publish through the
cache-scoped `CACHIX_AUTH_TOKEN` repository secret; pull requests receive
no write token. A missing token fails trusted jobs instead of silently
skipping the compiler build. No repository variable is required.

The flake advertises the cache and public signing key. Accept its config
with `--accept-flake-config`, or run `cachix use cinch` once. The devenv
configuration also pulls from and pushes to `cinch`; provide a personal
cache-scoped token through `CACHIX_AUTH_TOKEN` to publish local builds.
Never put a token in the repository.

The same workflow runs nightly at 02:23 UTC. After all three builds pass,
it publishes a dated GitHub prerelease with native Stage 3 binaries,
SHA-256 sidecars, and per-platform build metadata. A draft keeps incomplete
uploads off the public release list. Manual runs on `main` can publish an
initial nightly using the `publish_nightly` input. Ordinary pushes produce
14-day Actions artifacts without creating releases.

See [nightly instructions](../nightly-release.md) for binary verification
and usage. The release executables run without Nix. Linux packaging checks
for an ELF interpreter and relinks the checked Stage 3 objects statically
when needed; macOS allows only system-library dependencies. The
website's downloads page reads completed GitHub nightlies at build time
and refreshes them in the browser; it does not need a redeploy for each
new release.

By hand, from a checkout that has the pinned revisions
(`git fetch origin <revision>` in a shallow clone):

```sh
nix build --impure -f tooling/nix/local.nix ladder | cachix push cinch
```

`local.nix` reads the pinned revisions from the checkout's own git objects.
The flake fetches them from GitHub by revision. Both produce the same
derivations, because a fetched tree is content addressed, so a cache filled
by one serves the other.

## What the driver needs from Nix

A sandboxed build has no git history and should not repeat a cargo build
because a file in `tooling/` changed. `tooling/bootstrap.py` reads two
variables for that:

- `CINCH_SOURCE_ARCHIVES`: a directory of `<revision>.tar` files, used
  instead of `git archive`. The extracted tree is still checked against the
  pinned source hash.
- `CINCH_PREBUILT_COMPILERS`: a directory of `<revision>/cinch` executables,
  used instead of the cargo build of that revision. Whoever sets it vouches
  for the binary; unset, the driver builds from the snapshot as before.
