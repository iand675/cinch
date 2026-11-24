#!/usr/bin/env bash
# Vercel Ignored Build Step for cinch.systems (Root Directory: website/).
#
# Vercel convention:
#   exit 0 -> skip/cancel the deployment build
#   exit 1 -> proceed with the deployment build
#
# Any unexpected error fails open (exits non-zero) so a broken check never
# silently drops a real website update.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HEAD_SHA="$(git -C "$REPO_ROOT" rev-parse HEAD)"
PREV_SHA="${VERCEL_GIT_PREVIOUS_SHA:-}"
ENV_NAME="${VERCEL_ENV:-}"

# Same-commit redeploy (manual Redeploy button or daily catalogue-refresh
# deploy hook when HEAD is already the deployed revision): always build.
if [[ -n "$PREV_SHA" && "$PREV_SHA" == "$HEAD_SHA" ]]; then
  echo "Proceeding with build: explicit redeploy of current commit ($HEAD_SHA)."
  exit 1
fi

BASE=""
if [[ -n "$PREV_SHA" ]]; then
  # Prefer the last successful deployment on this branch so multi-commit pushes
  # cannot hide an earlier website change behind a trailing compiler commit.
  if git -C "$REPO_ROOT" cat-file -e "${PREV_SHA}^{commit}" 2>/dev/null; then
    BASE="$PREV_SHA"
  else
    # Vercel shallow-clones 10 commits deep; if the last deployment is older,
    # fail open rather than risking a skipped site update.
    echo "Proceeding with build: previous deployment ($PREV_SHA) is outside shallow clone."
    exit 1
  fi
else
  # First deployment on this branch. Always build the initial production deploy;
  # for preview branches, inspect HEAD^..HEAD so non-website branch pushes skip.
  if [[ "$ENV_NAME" == "production" ]]; then
    echo "Proceeding with build: initial production deployment."
    exit 1
  fi
  if git -C "$REPO_ROOT" rev-parse --verify -q HEAD^ >/dev/null 2>&1; then
    BASE="HEAD^"
  else
    echo "Proceeding with build: no parent commit available."
    exit 1
  fi
fi

# Inputs read by `astro build` (astro.config.mjs, src/loaders/safelang-docs.ts,
# src/lib/stdlib.ts), excluding non-build design notes and screenshots.
if git -C "$REPO_ROOT" diff --quiet "$BASE" HEAD -- \
  website \
  documentation.md \
  learn \
  reference \
  library \
  ':(glob)seed/prelude/**/*.cinch' \
  tooling/editors/vscode/syntaxes/cinch.tmLanguage.json \
  ':(exclude)website/design' \
  ':(exclude)website/screenshots' \
  ':(exclude)website/AGENTS.md' \
  ':(exclude)website/README.md'; then
  echo "Skipping Vercel build: no Astro or published docs inputs changed between ${BASE:0:12} and ${HEAD_SHA:0:12}."
  exit 0
else
  status=$?
  if [[ "$status" -eq 1 ]]; then
    echo "Proceeding with Vercel build: website or published docs inputs changed between ${BASE:0:12} and ${HEAD_SHA:0:12}."
    exit 1
  fi
  echo "Proceeding with Vercel build: git diff returned unexpected status $status."
  exit 1
fi
