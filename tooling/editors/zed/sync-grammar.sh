#!/usr/bin/env bash
# Build the local grammar repository that Zed clones for the Cinch grammar.
#
# Zed does not run `tree-sitter generate`: it clones a git repository, checks
# out one revision, and compiles `src/parser.c` (plus `src/scanner.c`) with
# the wasi-sdk clang into a wasm grammar. This repository does not commit the
# generated parser (tooling/tree-sitter-cinch/.gitignore), so the extension
# points at a small throwaway repository built here from grammar.js.
#
# Run this after any change to tooling/tree-sitter-cinch/, then rebuild the
# dev extension in Zed (`zed: rebuild dev extension`).
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
grammar="$(cd "$here/../../tree-sitter-cinch" && pwd)"
repo="$here/grammar-repo"
manifest="$here/extension.toml"

TREE_SITTER="${TREE_SITTER:-tree-sitter}"
if ! command -v "$TREE_SITTER" >/dev/null 2>&1; then
  TREE_SITTER="npx --yes tree-sitter-cli"
fi

rm -rf "$repo"
mkdir -p "$repo/src"

cp "$grammar/grammar.js" "$grammar/tree-sitter.json" "$repo/"
cp "$grammar/src/scanner.c" "$repo/src/"

echo "sync-grammar: generating parser"
(cd "$repo" && $TREE_SITTER generate)

git -C "$repo" init -q -b main
git -C "$repo" add -A
git -C "$repo" -c user.name="Cinch grammar" \
  -c user.email="cinch-grammar@example.invalid" \
  commit -qm "cinch grammar snapshot"
rev="$(git -C "$repo" rev-parse HEAD)"

# Zed keeps its own clone of the grammar under grammars/cinch; drop it so the
# next extension build fetches this snapshot instead of checking out the
# previous commit from the local objects.
rm -rf "$here/grammars/cinch" "$here/grammars/cinch.wasm"

python3 - "$manifest" "file://$repo" "$rev" <<'PY'
import re
import sys

path, url, rev = sys.argv[1:4]
text = open(path, encoding="utf-8").read()
text = re.sub(r'(?m)^repository = "file://.*"$', f'repository = "{url}"', text, count=1)
text = re.sub(r'(?m)^rev = ".*"$', f'rev = "{rev}"', text, count=1)
open(path, "w", encoding="utf-8").write(text)
PY

echo "sync-grammar: grammar repository at $repo"
echo "sync-grammar: pinned $rev in $manifest"
