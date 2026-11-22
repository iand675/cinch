# Cinch nightly compiler

Download the executable for Linux x86-64, Linux ARM64, or Apple Silicon macOS,
and its matching `.sha256` file. Verify it and make it executable:

```sh
sha256sum -c cinch-x86_64-linux.sha256
chmod +x cinch-x86_64-linux
# macOS: shasum -a 256 -c cinch-aarch64-darwin.sha256
```

The download is the native Stage 3 compiler. Linux binaries are statically
linked; macOS binaries use only system libraries. Nix is not needed to run
them. The build metadata records the source revision and binary hash.

Check a source file, or emit LLVM IR:

```sh
./cinch-x86_64-linux --check Main.cinch check
./cinch-x86_64-linux --emit-ir-only Main.cinch program.ll
```

On macOS use `./cinch-aarch64-darwin` instead. The current Stage 3 command
line takes explicit output paths. A named refusal exits with code 4 and
writes `check.diagnostics`. Invoking it without arguments exits with code 1.
Linking generated programs additionally needs a compatible Clang toolchain
(LLVM 20 is the version used by CI).

Every release passes the bootstrap fixed point, stage smoke programs,
interface consumption and changed-source checks. CI also tests the exact
published executable outside the build tree: it must emit the same IR as
the checked Stage 3 and give a named refusal for invalid source.
