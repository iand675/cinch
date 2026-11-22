# Cinch nightly compiler

These are native Nix distributions for Linux x86-64, Linux ARM64 and
Apple Silicon macOS. Install Nix first and choose the matching system.
The archive contains the checked Stage 3 compiler and its complete Nix
runtime closure. It is not a relocatable standalone executable.

Verify the downloaded archive against its `.sha256` file:

```sh
sha256sum -c cinch-x86_64-linux.nar.gz.sha256
# macOS: shasum -a 256 -c cinch-aarch64-darwin.nar.gz.sha256
```

Import the archive, then find the ladder path in `build-SYSTEM.json`:

```sh
gzip -dc cinch-SYSTEM.nar.gz | nix-store --import
```

An unprivileged multi-user Nix installation may require an administrator
or a trusted Nix user for an unsigned archive import. Alternatively, use
the signed public Cachix cache to build the exact release revision:

```sh
cachix use cinch
nix build github:iand675/cinch/REVISION#cinch-ladder
```

`REVISION` is the full commit in the release notes and build metadata.
The checked tree is read-only. From a checkout of that revision, run stage
consumers with `CINCH_BOOTSTRAP_BUILD` set to the ladder store path. For a
writable development tree, build `cinch-pins` and hydrate it as described
in `tooling/nix/README.md` in the repository.

Every release passes `tooling/bootstrap.py check`: the bootstrap fixed
point, stage smokes, interface consumption and changed-source probe.
This does not imply that every experimental compiler feature is complete.
