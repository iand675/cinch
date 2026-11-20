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
cachix use <cache>                      # once per machine

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

`.github/workflows/ladder-cache.yml` builds `pins` and `ladder` on every push
that touches their inputs and pushes them with `cachix-action`. It needs the
`CACHIX_CACHE` repository variable and the `CACHIX_AUTH_TOKEN` secret, and
does nothing without them.

By hand, from a checkout that has the pinned revisions
(`git fetch origin <revision>` in a shallow clone):

```sh
nix build --impure -f tooling/nix/local.nix ladder | cachix push <cache>
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
