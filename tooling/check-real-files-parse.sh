#!/usr/bin/env bash
# Parse every real Cinch source file with the tree-sitter grammar and
# require zero ERROR nodes. This is the property net on realistic input
# (the corpus covers the rules point by point; this covers whole files).
#
# Surface files: every `.cinch` under compiler/prelude/ and examples/source/.
#
# Core files: every `*.core` under compiler/tests/core/ (the printer's
# golden corpus, which another slice creates) plus every `*.core` under
# tooling/tree-sitter-cinch-core/samples/. The golden directory is allowed
# to be absent: while the printer has not landed there is nothing to parse
# yet, so the script says so and still checks the samples.
#
# Usage:
#
#   tooling/check-real-files-parse.sh            # surface and Core
#   tooling/check-real-files-parse.sh --surface  # surface only
#   tooling/check-real-files-parse.sh --core     # Core only
#
# Requirements: the tree-sitter CLI on PATH (0.26 tested; `npx
# tree-sitter-cli` also works via TREE_SITTER below), a C compiler, and a
# generated parser per grammar (run tooling/check-grammar-drift.sh [--core]
# first, or let this script generate one).
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"

case "${1:-}" in
  ""|--all)   check_surface=1; check_core=1 ;;
  --surface)  check_surface=1; check_core=0 ;;
  --core)     check_surface=0; check_core=1 ;;
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

# Counts for the group that just ran. parse_group is called with process
# substitution, so it runs in this shell and these stay visible.
group_total=0
group_failed=0

# parse_group <grammar dir> <label>
# Parses the newline separated paths on stdin with that grammar's parser and
# sets group_total/group_failed.
parse_group() {
  local grammar="$1" label="$2" f out
  group_total=0
  group_failed=0

  cd "$grammar"

  if [ ! -f src/parser.c ]; then
    echo "check-real-files-parse: generating $label parser"
    $TREE_SITTER generate
  fi

  while IFS= read -r f; do
    [ -n "$f" ] || continue
    group_total=$((group_total + 1))
    if ! out="$($TREE_SITTER parse "$f" 2>/dev/null)" ||
       printf '%s' "$out" | grep -qE '\((ERROR|MISSING)'; then
      group_failed=$((group_failed + 1))
      echo "check-real-files-parse: FAIL $f"
      printf '%s\n' "$out" | grep -nE '\((ERROR|MISSING)' | head -5 || true
    fi
  done

  echo "check-real-files-parse: $label: $group_total files, $group_failed failing"
}

surface_files() {
  find "$root/compiler/prelude" "$root/examples/source" -name '*.cinch' | sort
}

core_files() {
  local golden="$root/compiler/tests/core"
  if [ -d "$golden" ]; then
    find "$golden" -name '*.core' | sort
  else
    # Diagnostics go to stderr so they never enter the file list.
    echo "check-real-files-parse: $golden does not exist yet, skipping the Core golden files" >&2
  fi
  find "$root/tooling/tree-sitter-cinch-core/samples" -name '*.core' | sort
}

total=0
failed=0
surface_total=0
core_total=0

if [ "$check_surface" -eq 1 ]; then
  parse_group "$root/tooling/tree-sitter-cinch" surface < <(surface_files)
  total=$((total + group_total))
  failed=$((failed + group_failed))
  surface_total=$group_total
fi

if [ "$check_core" -eq 1 ]; then
  parse_group "$root/tooling/tree-sitter-cinch-core" core < <(core_files)
  total=$((total + group_total))
  failed=$((failed + group_failed))
  core_total=$group_total
fi

echo "check-real-files-parse: $total files, $failed failing"
# A group that was asked for but found no files is a failure: the check
# would otherwise pass vacuously.
if { [ "$check_surface" -eq 1 ] && [ "$surface_total" -eq 0 ]; } ||
   { [ "$check_core" -eq 1 ] && [ "$core_total" -eq 0 ]; } ||
   [ "$failed" -ne 0 ]; then
  exit 1
fi
echo "check-real-files-parse: OK"
