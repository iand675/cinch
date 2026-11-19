# The flake's ladder outputs, built from this checkout without the network.
#
#   nix build --impure -f tooling/nix/local.nix ladder
#
# The pinned seed and library-producer revisions are read from this
# checkout's own git objects instead of being fetched from GitHub (fetch them
# first if the clone is shallow: `git fetch origin <revision>`). The fetched
# trees are content addressed, so these derivations are the ones the flake
# evaluates and a binary cache filled from either serves both.
{ system ? builtins.currentSystem
, pkgs ? (builtins.getFlake (toString ../..)).inputs.nixpkgs.legacyPackages.${system}
, jobs ? 4
}:
let
  root = ../..;
  # Only tracked files, as in the flake's copy of the tree: an untracked
  # scratch file or a `__pycache__` must not change a derivation.
  tracked = pkgs.lib.fileset.gitTracked root;
in
import ./. {
  inherit pkgs jobs tracked;
  src = root;
  fetchPinned = rev: builtins.fetchGit {
    url = "file://${toString root}";
    inherit rev;
    allRefs = true;
    shallow = true;
  };
}
