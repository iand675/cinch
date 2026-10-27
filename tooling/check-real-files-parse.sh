#!/usr/bin/env bash
# Parse every real SafeLang source file with the tree-sitter grammar and
# require zero ERROR nodes. This is the property net for the scanner on
# realistic input (the corpus covers the lexical rules point by point;
# this covers whole files): every `.lang` under compiler/prelude/ and
# examples/source/ must parse clean.
#
# CI invocation (no CI config is modified by this repository today):
#
#   tooling/check-real-files-parse.sh
#
# Requirements: the tree-sitter CLI on PATH (0.26 tested; `npx
# tree-sitter-cli` also works via TREE_SITTER below), a C compiler, and
# a generated parser (run tooling/check-grammar-drift.sh first, or let
# this script generate one).
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
grammar="$root/tooling/tree-sitter-safelang"

TREE_SITTER="${TREE_SITTER:-tree-sitter}"
if ! command -v "$TREE_SITTER" >/dev/null 2>&1; then
  # Fall back to the npx-packaged CLI.
  TREE_SITTER="npx --yes tree-sitter-cli"
fi

cd "$grammar"

if [ ! -f src/parser.c ]; then
  echo "check-real-files-parse: generating parser"
  $TREE_SITTER generate
fi

total=0
failed=0
while IFS= read -r f; do
  total=$((total + 1))
  if ! out="$($TREE_SITTER parse "$f" 2>/dev/null)" ||
     printf '%s' "$out" | grep -qE '\((ERROR|MISSING)'; then
    failed=$((failed + 1))
    echo "check-real-files-parse: FAIL $f"
    printf '%s\n' "$out" | grep -nE '\((ERROR|MISSING)' | head -5 || true
  fi
done < <(find "$root/compiler/prelude" "$root/examples/source" -name '*.lang' | sort)

echo "check-real-files-parse: $total files, $failed failing"
if [ "$total" -eq 0 ] || [ "$failed" -ne 0 ]; then
  exit 1
fi
echo "check-real-files-parse: OK"
