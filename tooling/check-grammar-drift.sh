#!/usr/bin/env bash
# Drift check for the Cinch tree-sitter grammars.
#
# Surface grammar (default). Regenerates the parser from grammar.js and runs
# the corpus tests in tooling/tree-sitter-cinch/corpus/ (the parse-tree
# assertions are the drift net for the lexical rules: the minus table, dash
# names, the tick Char-vs-Symbol rule, `$` operator runs, attributes, and the
# prim/contract forms). Any divergence from the asserted trees exits nonzero.
#
# Core grammar (--core). Same check for tooling/tree-sitter-cinch-core: it
# regenerates the parser from that grammar.js and runs the corpus in
# tooling/tree-sitter-cinch-core/corpus/ (one case per expression form of
# design/core-output.md section 10, plus the header, unit, annotation,
# params, type and layout cases).
#
# CI invocation (no CI config is modified by this repository today):
#
#   tooling/check-grammar-drift.sh            # surface
#   tooling/check-grammar-drift.sh --core     # Core
#
# Requirements: the tree-sitter CLI on PATH (0.26 tested; `npx
# tree-sitter-cli` also works via TREE_SITTER below) and a C compiler.
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"

case "${1:-}" in
  ""|--surface)
    grammar="$root/tree-sitter-cinch"
    label="surface"
    ;;
  --core)
    grammar="$root/tree-sitter-cinch-core"
    label="core"
    ;;
  *)
    echo "usage: $0 [--surface|--core]" >&2
    exit 2
    ;;
esac

if [ "$#" -gt 1 ]; then
  echo "usage: $0 [--surface|--core]" >&2
  exit 2
fi

TREE_SITTER="${TREE_SITTER:-tree-sitter}"
if ! command -v "$TREE_SITTER" >/dev/null 2>&1; then
  # Fall back to the npx-packaged CLI.
  TREE_SITTER="npx --yes tree-sitter-cli"
fi

cd "$grammar"

echo "check-grammar-drift: generating $label parser"
$TREE_SITTER generate

echo "check-grammar-drift: running $label corpus tests"
$TREE_SITTER test

echo "check-grammar-drift: $label OK"
