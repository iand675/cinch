{ pkgs, ... }:

{
  # Rust toolchain for the compiler in compiler/.
  languages.rust = {
    enable = true;
    # Use the stable channel from nixpkgs; components stay minimal.
  };

  # Astro toolchain for the documentation site in website/.
  languages.javascript = {
    enable = true;
    npm.enable = true;
  };

  # LLVM backend: the compiler emits LLVM IR text and drives clang for
  # codegen and linking. Keep the LLVM version pinned by nixpkgs so the
  # IR we emit is validated by a known toolchain.
  packages = [
    pkgs.clang
    pkgs.llvm
    pkgs.llvmPackages.bintools
  ];

  env.SAFELANG_CLANG = "${pkgs.clang}/bin/clang";

  scripts.safelang = {
    exec = ''
      cargo run --manifest-path "$DEVENV_ROOT/compiler/Cargo.toml" --quiet -- "$@"
    '';
    description = "Run the SafeLang bootstrap compiler";
  };

  scripts.docs = {
    exec = ''
      cd "$DEVENV_ROOT/website"
      [ -d node_modules ] || npm install
      exec npm run dev -- --port "''${PORT:-4321}"
    '';
    description = "Serve the Astro documentation site";
  };

  scripts.docs-build = {
    exec = ''
      cd "$DEVENV_ROOT/website"
      [ -d node_modules ] || npm install
      exec npm run build
    '';
    description = "Build the static documentation site into website/dist";
  };

  scripts.safelang-test = {
    exec = ''
      cargo test --manifest-path "$DEVENV_ROOT/compiler/Cargo.toml" --quiet
    '';
    description = "Run the SafeLang compiler test suites";
  };

  enterShell = ''
    echo "devenv: rust $(rustc --version | cut -d' ' -f2), $($SAFELANG_CLANG --version | head -n1)"
  '';
}
