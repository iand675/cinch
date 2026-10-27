#!/usr/bin/env bash
# Drift check for the SafeLang tree-sitter grammar.
#
# Regenerates the parser from grammar.js and runs the corpus tests in
# tooling/tree-sitter-safelang/corpus/ (the parse-tree assertions are
# the drift net for the lexical rules: the minus table, dash names, the
# tick Char-vs-Symbol rule, `$` operator runs, attributes, and the
# prim/contract forms). Any divergence from the asserted trees exits
# nonzero.
#
# CI invocation (no CI config is modified by this repository today):
#
#   tooling/check-grammar-drift.sh
#
# Requirements: the tree-sitter CLI on PATH (0.26 tested; `npx
# tree-sitter-cli` also works via TREE_SITTER below) and a C compiler.
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
grammar="$root/tree-sitter-safelang"

TREE_SITTER="${TREE_SITTER:-tree-sitter}"
if ! command -v "$TREE_SITTER" >/dev/null 2>&1; then
  # Fall back to the npx-packaged CLI.
  TREE_SITTER="npx --yes tree-sitter-cli"
fi

cd "$grammar"

echo "check-grammar-drift: generating parser"
$TREE_SITTER generate

echo "check-grammar-drift: running corpus tests"
$TREE_SITTER test

echo "check-grammar-drift: OK"
