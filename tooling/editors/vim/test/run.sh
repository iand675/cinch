#!/usr/bin/env bash
# Headless syntax checks for the Cinch Core vim integration.
#
#   tooling/editors/vim/test/run.sh [/path/to/vim]
#
# Runs check.vim in Ex mode (no vimrc, no plugins), prints the per-category
# results from check.out, and exits nonzero when any check failed. `nvim`
# also works as the binary: the syntax file is plain Vim script.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
vim_bin="${1:-${VIM:-vim}}"

status=0
"$vim_bin" -Nu NONE -es -S "$here/check.vim" || status=$?
cat "$here/check.out"
exit "$status"
