#!/usr/bin/env bash
# Batch checks for the Cinch Core Emacs mode.
#
#   tooling/editors/emacs/test/run.sh [/path/to/emacs]
#
# `emacs` is not always installed; `nix shell nixpkgs#emacs-nox --command
# tooling/editors/emacs/test/run.sh $(nix shell nixpkgs#emacs-nox --command which emacs)`
# is one way to run it on demand.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
emacs_bin="${1:-${EMACS:-emacs}}"

if ! command -v "$emacs_bin" >/dev/null 2>&1; then
  echo "no emacs binary named '$emacs_bin' on PATH" >&2
  exit 2
fi

"$emacs_bin" -Q --batch \
  -l "$here/../cinch-core-mode.el" \
  -l "$here/check.el"
