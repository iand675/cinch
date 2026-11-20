{
  description = "Cinch: the bootstrap ladder as cacheable outputs";

  # flake.lock holds the same nixpkgs revision as devenv.lock, so both entry
  # points build with one toolchain. Move them together.
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      each = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = each (pkgs:
        let ladder = import ./tooling/nix { inherit pkgs; src = ./.; }; in
        {
          cinch-seed = ladder.seed-compiler;
          cinch-provider = ladder.provider-compiler;
          cinch-pins = ladder.pins;
          cinch-ladder = ladder.ladder;
          default = ladder.ladder;
        });
    };
}
