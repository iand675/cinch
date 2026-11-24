#!/usr/bin/env bash
# Self-test for website/scripts/vercel-ignore.sh using a temporary git repo
# (including a --depth=10 shallow clone matching Vercel's build environment).

set -euo pipefail

SCRIPT="$(cd "$(dirname "$0")" && pwd)/vercel-ignore.sh"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

REPO="$TMPDIR_ROOT/repo"
mkdir -p "$REPO/website/src" "$REPO/website/design" "$REPO/compiler" "$REPO/learn" \
  "$REPO/reference" "$REPO/library" "$REPO/seed/prelude/Data" \
  "$REPO/tooling/editors/vscode/syntaxes"

git -C "$REPO" init -q -b main
git -C "$REPO" config user.email "test@example.com"
git -C "$REPO" config user.name "Test"

touch "$REPO/website/src/index.astro" \
  "$REPO/website/design/notes.md" \
  "$REPO/website/README.md" \
  "$REPO/compiler/Bootstrap.Check.cinch" \
  "$REPO/documentation.md" \
  "$REPO/learn/01-first-program.md" \
  "$REPO/reference/00-guide.md" \
  "$REPO/library/Data.List.md" \
  "$REPO/seed/prelude/Data/List.cinch" \
  "$REPO/seed/prelude/.cinchpkg" \
  "$REPO/tooling/editors/vscode/syntaxes/cinch.tmLanguage.json"

git -C "$REPO" add .
git -C "$REPO" commit -q -m "initial"
BASE_SHA="$(git -C "$REPO" rev-parse HEAD)"

run_case() {
  local name="$1"
  local expected="$2"
  local workdir="$3"
  local prev_sha="$4"
  local env_name="$5"
  local rc=0
  (
    cd "$workdir"
    VERCEL_GIT_PREVIOUS_SHA="$prev_sha" VERCEL_ENV="$env_name" bash "$SCRIPT" >/dev/null
  ) || rc=$?
  if [[ "$rc" -ne "$expected" ]]; then
    echo "FAIL [$name]: expected exit $expected, got $rc" >&2
    exit 1
  fi
  echo "PASS [$name] (exit $rc)"
}

commit_edit() {
  local rel="$1"
  local msg="$2"
  echo "$msg" >> "$REPO/$rel"
  git -C "$REPO" add "$rel"
  git -C "$REPO" commit -q -m "$msg"
}

# 1. Compiler-only change -> skip (0)
commit_edit "compiler/Bootstrap.Check.cinch" "compiler edit"
run_case "compiler-only skips" 0 "$REPO/website" "$BASE_SHA" "production"

# 2. Same-commit redeploy / deploy hook -> build (1)
CUR_SHA="$(git -C "$REPO" rev-parse HEAD)"
run_case "same-commit deploy hook builds" 1 "$REPO/website" "$CUR_SHA" "production"

# 3. Preview branch first push (no PREVIOUS_SHA) with compiler-only commit -> skip (0)
run_case "preview first push compiler-only skips" 0 "$REPO/website" "" "preview"

# 4. Initial production deploy (no PREVIOUS_SHA) -> build (1)
run_case "production initial deploy builds" 1 "$REPO/website" "" "production"

# 5. Non-build website docs/design & prelude manifest -> skip (0)
PREV="$(git -C "$REPO" rev-parse HEAD)"
commit_edit "website/design/notes.md" "design note"
commit_edit "website/README.md" "readme edit"
commit_edit "seed/prelude/.cinchpkg" "manifest edit"
run_case "non-build website docs and cinchpkg skip" 0 "$REPO/website" "$PREV" "production"

# 6. Astro source change -> build (1)
PREV="$(git -C "$REPO" rev-parse HEAD)"
commit_edit "website/src/index.astro" "astro edit"
run_case "astro source builds" 1 "$REPO/website" "$PREV" "production"

# 7. Multi-commit push: website commit followed by compiler commit -> build (1)
commit_edit "compiler/Bootstrap.Check.cinch" "trailing compiler edit"
run_case "multi-commit push with earlier website edit builds" 1 "$REPO/website" "$PREV" "production"

# 8. Each external website input -> build (1)
for external in \
  "documentation.md" \
  "learn/01-first-program.md" \
  "reference/00-guide.md" \
  "library/Data.List.md" \
  "seed/prelude/Data/List.cinch" \
  "tooling/editors/vscode/syntaxes/cinch.tmLanguage.json"; do
  PREV="$(git -C "$REPO" rev-parse HEAD)"
  commit_edit "$external" "edit $external"
  run_case "external input $external builds" 1 "$REPO/website" "$PREV" "production"
done

# 9. Shallow clone (--depth=10) with PREVIOUS_SHA older than 10 commits -> fail open (1)
OLD_SHA="$(git -C "$REPO" rev-parse HEAD)"
for i in $(seq 1 11); do
  commit_edit "compiler/Bootstrap.Check.cinch" "compiler filler $i"
done
SHALLOW="$TMPDIR_ROOT/shallow"
git clone -q --depth=10 "file://$REPO" "$SHALLOW"
run_case "shallow clone missing previous sha fails open" 1 "$SHALLOW/website" "$OLD_SHA" "production"
SHALLOW_PREV="$(git -C "$SHALLOW" rev-parse HEAD~3)"
run_case "shallow clone with recent previous sha skips compiler edits" 0 "$SHALLOW/website" "$SHALLOW_PREV" "production"

echo "All vercel-ignore.sh tests passed."
