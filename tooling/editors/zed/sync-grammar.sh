#!/usr/bin/env bash
# Build the local grammar repositories that Zed clones, one per grammar.
#
# Zed does not run `tree-sitter generate`: it clones a git repository, checks
# out one revision, and compiles `src/parser.c` (plus `src/scanner.c`) with
# the wasi-sdk clang into a wasm grammar. This repository does not commit the
# generated parsers, so this script builds a small throwaway repository per
# grammar from its `grammar.js`, generates the parser there, commits it, and
# pins that revision in `extension.toml`.
#
#   cinch       -> tooling/tree-sitter-cinch        -> zed/grammar-repo
#   cinch-core  -> tooling/tree-sitter-cinch-core   -> zed/grammar-repo-core
#
# Run this after any change to either grammar, then rebuild the dev
# extension in Zed (`zed: rebuild dev extension`).
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
manifest="$here/extension.toml"

TREE_SITTER="${TREE_SITTER:-tree-sitter}"
if ! command -v "$TREE_SITTER" >/dev/null 2>&1; then
  TREE_SITTER="npx --yes tree-sitter-cli"
fi

# grammar name : grammar source directory under tooling/ : local repository
grammars=(
  "cinch:tree-sitter-cinch:grammar-repo"
  "cinch-core:tree-sitter-cinch-core:grammar-repo-core"
)

# Check every grammar before touching anything. The Core grammar is written by
# its own slice (design/core-output.md section 8.2); this script never invents
# one, and it leaves extension.toml alone when a grammar is absent.
missing=()
for entry in "${grammars[@]}"; do
  IFS=: read -r name dir repo <<<"$entry"
  src="$(cd "$here/../.." && pwd)/$dir"
  if [ ! -f "$src/grammar.js" ]; then
    missing+=("$name: $src/grammar.js")
  fi
done

if [ "${#missing[@]}" -gt 0 ]; then
  echo "sync-grammar: missing grammar source, nothing generated:" >&2
  for item in "${missing[@]}"; do
    echo "  - $item" >&2
  done
  echo "sync-grammar: the Cinch Core grammar lives at tooling/tree-sitter-cinch-core" >&2
  echo "sync-grammar: (written by the grammar slice); extension.toml was not changed." >&2
  exit 1
fi

for entry in "${grammars[@]}"; do
  IFS=: read -r name dir repo <<<"$entry"
  src="$(cd "$here/../.." && pwd)/$dir"
  dest="$here/$repo"

  rm -rf "$dest"
  mkdir -p "$dest/src"

  cp "$src/grammar.js" "$dest/"
  if [ -f "$src/tree-sitter.json" ]; then
    cp "$src/tree-sitter.json" "$dest/"
  fi
  if [ -f "$src/src/scanner.c" ]; then
    cp "$src/src/scanner.c" "$dest/src/"
  fi

  echo "sync-grammar: generating the $name parser"
  (cd "$dest" && $TREE_SITTER generate)

  git -C "$dest" init -q -b main
  git -C "$dest" add -A
  git -C "$dest" -c user.name="Cinch grammar" \
    -c user.email="cinch-grammar@example.invalid" \
    commit -qm "$name grammar snapshot"
  rev="$(git -C "$dest" rev-parse HEAD)"

  # Zed keeps its own clone of each grammar under grammars/<name>; drop it so
  # the next extension build fetches this snapshot instead of checking out the
  # previous commit from the local objects.
  rm -rf "$here/grammars/$name" "$here/grammars/$name.wasm"

  python3 - "$manifest" "$name" "file://$dest" "$rev" <<'PY'
import re
import sys

path, grammar, url, rev = sys.argv[1:5]
text = open(path, encoding="utf-8").read()

header = re.compile(r"(?ms)^\[grammars\." + re.escape(grammar) + r"\]$.*?(?=^\[|\Z)")
match = header.search(text)
if match is None:
    sys.exit(f"sync-grammar: no [grammars.{grammar}] section in {path}")

block = match.group(0)
block = re.sub(r'(?m)^repository = ".*"$', f'repository = "{url}"', block, count=1)
block = re.sub(r'(?m)^rev = ".*"$', f'rev = "{rev}"', block, count=1)
open(path, "w", encoding="utf-8").write(text[: match.start()] + block + text[match.end() :])
PY

  echo "sync-grammar: $name repository at $dest"
  echo "sync-grammar: pinned $name $rev in $manifest"
done
