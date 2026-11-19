# The bootstrap ladder as cacheable Nix outputs.
#
#   seed-compiler      the pinned Rust seed (bootstrap/stage0.lock.json)
#   provider-compiler  the pinned library producer (bootstrap/library-provider.lock.json)
#   pins               stage0, both source snapshots and the library object:
#                      everything the pins determine, nothing the sources do
#   ladder             pins plus stages 1 to 3, built and checked by
#                      `tooling/bootstrap.py check` (fixed point, smokes,
#                      interface consumer, changed source)
#
# The split follows what invalidates what. The two cargo release builds are
# keyed by a pinned revision and the Rust toolchain alone, so they survive
# every commit that does not move a pin. `pins` adds the tooling that drives
# them. `ladder` is keyed by the compiler sources, so it is the per-commit
# output a binary cache serves to the next session.
#
# `ladder` builds with CINCH_BOOTSTRAP_BUILD set to its own output path:
# stage 2 and 3 receipts cover `objects.json`, which names the library object
# by absolute path, so the tree is only valid where it was built. Use it in
# place (CINCH_BOOTSTRAP_BUILD=<store path>); it is read-only, so it serves
# test runs, not rebuilds. `pins` is relocatable: `tooling/bootstrap.py
# hydrate --from <store path>` copies it into a writable build tree.
{ pkgs
  # The repository root, as a path.
, src
  # A fileset to intersect every source selection with, or null. A build
  # from a working checkout passes its tracked files here.
, tracked ? null
  # The repository the pinned revisions are fetched from. A pure evaluation
  # may fetch by revision alone, so no hash is recorded here: the bootstrap
  # checks each snapshot against the pinned source hash.
, repository ? "https://github.com/iand675/cinch.git"
, fetchPinned ? (rev: builtins.fetchGit { url = repository; inherit rev; allRefs = true; })
, llvmPackages ? pkgs.llvmPackages_20
, jobs ? 4
}:

let
  inherit (pkgs) lib;
  select = files:
    let chosen = lib.fileset.unions files; in
    lib.fileset.toSource {
      root = src;
      fileset = if tracked == null then chosen else lib.fileset.intersection tracked chosen;
    };
  readLock = path: builtins.fromJSON (builtins.readFile (src + path));
  seedLock = readLock "/bootstrap/stage0.lock.json";
  providerLock = readLock "/bootstrap/library-provider.lock.json";

  # One release build of a pinned revision's compiler. Nothing from the
  # current tree enters it.
  pinnedCompiler = name: lock:
    let tree = fetchPinned lock.revision; in
    pkgs.rustPlatform.buildRustPackage {
      pname = "cinch-${name}";
      version = builtins.substring 0 12 lock.revision;
      src = tree;
      sourceRoot = "source/compiler";
      cargoLock.lockFile = tree + "/compiler/Cargo.lock";
      nativeBuildInputs = [ pkgs.tree-sitter pkgs.nodejs ];
      cargoBuildFlags = [ "--bin" "cinch" ];
      # The grammar binding generates its parser beside the manifest.
      postUnpack = "chmod -R u+w source";
      doCheck = false;
      # The driver reads its runtime sources at run time from the snapshot
      # the bootstrap extracts, not from this output.
      dontStrip = true;
    };

  seed-compiler = pinnedCompiler "seed" seedLock;
  provider-compiler = pinnedCompiler "provider" providerLock;

  # `git archive REV compiler tooling/tree-sitter-cinch/...` without git.
  archive = lock: pkgs.runCommand "cinch-source-${builtins.substring 0 12 lock.revision}.tar" { } ''
    cd ${fetchPinned lock.revision}
    # The store may hard-link identical files; the driver admits only plain
    # files and directories, so archive each one whole.
    tar --hard-dereference --sort=name --mtime=@1 --owner=0 --group=0 --numeric-owner -cf $out \
      compiler $(for f in grammar.js tree-sitter.json src/scanner.c Cargo.toml bindings/rust/build.rs bindings/rust/lib.rs; do
        [ -e tooling/tree-sitter-cinch/$f ] && echo tooling/tree-sitter-cinch/$f; done)
  '';

  archives = pkgs.linkFarm "cinch-source-archives" [
    { name = "${seedLock.revision}.tar"; path = archive seedLock; }
    { name = "${providerLock.revision}.tar"; path = archive providerLock; }
  ];
  prebuilt = pkgs.linkFarm "cinch-prebuilt-compilers" [
    { name = "${seedLock.revision}/cinch"; path = "${seed-compiler}/bin/cinch"; }
    { name = "${providerLock.revision}/cinch"; path = "${provider-compiler}/bin/cinch"; }
  ];

  tools = [
    pkgs.python3 pkgs.cargo pkgs.rustc pkgs.patch pkgs.gnutar
    llvmPackages.clang llvmPackages.llvm llvmPackages.bintools
  ];

  environment = ''
    export HOME=$TMPDIR/home; mkdir -p $HOME
    export CINCH_CLANG=${llvmPackages.clang}/bin/clang
    export CINCH_SOURCE_ARCHIVES=${archives}
    export CINCH_PREBUILT_COMPILERS=${prebuilt}
    export CINCH_BUILD_SLOTS=1 CINCH_BUILD_SLOTS_DIR=$TMPDIR/slots
    export CINCH_CARGO_JOBS=${toString jobs}
    export CARGO_NET_OFFLINE=true
    export PYTHONDONTWRITEBYTECODE=1
  '';

  # Only what the pins and their driver read.
  pinSources = select [
      (src + "/tooling/bootstrap.py") (src + "/tooling/bootstrap_objects.py")
      (src + "/tooling/bootstrap_interfaces.py") (src + "/tooling/build_slots.py")
      (src + "/tooling/pgo.py") (src + "/tooling/shard_ir.py")
      (src + "/bootstrap/stage0.lock.json") (src + "/bootstrap/library-provider.lock.json")
      (src + "/bootstrap/library-object-roots.json") (src + "/bootstrap/seed")
      (src + "/bootstrap/Bootstrap.RuntimeAbi.cinch")
  ];

  pins = pkgs.stdenv.mkDerivation {
    pname = "cinch-bootstrap-pins";
    version = "${builtins.substring 0 8 seedLock.revision}-${builtins.substring 0 8 providerLock.revision}";
    src = pinSources;
    nativeBuildInputs = tools;
    dontConfigure = true;
    dontFixup = true;
    buildPhase = ''
      ${environment}
      export CINCH_BOOTSTRAP_BUILD=$out
      python3 tooling/bootstrap.py pins --no-pgo -j ${toString jobs}
    '';
    installPhase = ''
      rm -rf $out/cargo $out/library-cargo $out/library-runtime $out/.lock
    '';
  };

  # What `bootstrap.py check` reads from the tree.
  ladderSources = select [
    (src + "/tooling") (src + "/bootstrap") (src + "/compiler/prelude")
    (src + "/examples/compiler/fib.cinch")
  ];

  ladder = pkgs.stdenv.mkDerivation {
    pname = "cinch-bootstrap-ladder";
    version = "0";
    src = ladderSources;
    nativeBuildInputs = tools;
    dontConfigure = true;
    dontFixup = true;
    buildPhase = ''
      ${environment}
      export CINCH_BOOTSTRAP_BUILD=$out
      mkdir -p $out
      python3 tooling/bootstrap.py hydrate --from ${pins}
      python3 tooling/bootstrap.py check -j ${toString jobs}
    '';
    installPhase = ''
      rm -rf $out/cargo $out/library-cargo $out/library-runtime $out/.lock \
        $out/smoke $out/changed-source $out/shards
      ln -s stage3/cinch $out/cinch
    '';
  };
in
{
  inherit seed-compiler provider-compiler pins ladder;
}
