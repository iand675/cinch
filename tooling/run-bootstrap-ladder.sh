#!/usr/bin/env bash
# Assemble a bootstrap-ladder project and run it through the real compiler
# CLI. Rationale: the compiler does not embed the P30/P31 toolkit
# (Text.Lex, Text.Scan, Text.Parse, Text.Json and Text.Builder are on disk and absent
# from compiler/prelude/cinch-base.cinchpkg), and every bootstrap artifact
# imports it, so a rung runs as a PROJECT: the toolkit sources, every
# bootstrap/*.cinch module and one program are copied beside each other in
# a temp directory and the program runs there. This is the assembly
# compiler/tests/common/mod.rs performs for the tests, exposed so a rung
# can be run by hand.
#
# Usage:
#   tooling/run-bootstrap-ladder.sh                          # type-check the ladder's modules
#   tooling/run-bootstrap-ladder.sh --all-programs           # build and run every rung program
#   tooling/run-bootstrap-ladder.sh programs/Run.Parse.cinch # run one rung program
#
# `--all-programs` asks whether each rung's artifact is RUNNABLE, which the
# design document requires of every milestone: it assembles a project per
# program and reports the program's own exit code, judging only whether the
# program BUILT (the compiler prints `cinch: ...` on stderr when it does not).
# A rung's exit code is its own business: `Bootstrap.Test`'s failing-suite
# program exits nonzero on purpose. Whether a rung is correct is its test's
# claim, not this script's.
#
# Requirements: a Rust toolchain (the script drives `cargo run` from
# compiler/), and clang on PATH for the rungs that link.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
compiler="$root/compiler"
dir="$(mktemp -d)"
trap 'rm -rf "$dir"' EXIT

# The toolkit the artifacts import and the package does not embed.
toolkit=(Lex Scan Parse Json Builder)
for m in "${toolkit[@]}"; do cp "$compiler/prelude/Text/$m.cinch" "$dir/"; done
# Every bootstrap module.
cp "$root"/bootstrap/*.cinch "$dir/"

if [ "$#" -eq 0 ]; then
  # No program given: type-check every ladder module as a project. A module
  # that is not the entry still has to resolve and type-check, and doing it
  # from the CLI is evidence that the artifacts do not depend on the test
  # harness for their correctness.
  failed=0
  for f in "$root"/bootstrap/*.cinch; do
    name="$(basename "$f")"
    if cargo run -q --manifest-path "$compiler/Cargo.toml" --bin cinch -- check "$dir/$name" >/dev/null 2>&1; then
      echo "run-bootstrap-ladder: OK   $name"
    else
      echo "run-bootstrap-ladder: FAIL $name"
      failed=$((failed + 1))
    fi
  done
  if [ "$failed" -ne 0 ]; then
    echo "run-bootstrap-ladder: $failed module(s) did not check"
    exit 1
  fi
  echo "run-bootstrap-ladder: every ladder module checks"
  exit 0
fi

if [ "$1" = "--all-programs" ]; then
  # One project per program: every program declares `module Main`, so they
  # cannot share a directory.
  built=0
  broke=0
  for prog in "$root"/bootstrap/programs/*.cinch; do
    [ -f "$prog" ] || continue
    name="$(basename "$prog")"
    one="$(mktemp -d)"
    for m in "${toolkit[@]}"; do cp "$compiler/prelude/Text/$m.cinch" "$one/"; done
    cp "$root"/bootstrap/*.cinch "$one/"
    cp "$prog" "$one/$name"
    err="$(mktemp)"
    set +e
    cargo run -q --manifest-path "$compiler/Cargo.toml" --bin cinch -- run "$one/$name" 2>"$err"
    code=$?
    set -e
    if grep -q '^cinch: ' "$err"; then
      echo "run-bootstrap-ladder: DID NOT BUILD $name"
      grep '^cinch: ' "$err" | head -3 | sed 's/^/    /'
      broke=$((broke + 1))
    else
      echo "run-bootstrap-ladder: ran $name, exit $code"
      built=$((built + 1))
    fi
    rm -rf "$one" "$err"
  done
  echo "run-bootstrap-ladder: $built program(s) built and ran, $broke did not build"
  [ "$broke" -eq 0 ] || exit 1
  exit 0
fi

program="$1"
[ -f "$program" ] || program="$root/bootstrap/$program"
if [ ! -f "$program" ]; then
  echo "run-bootstrap-ladder: no such program: $1" >&2
  exit 2
fi
name="$(basename "$program")"
cp "$program" "$dir/$name"
cargo run -q --manifest-path "$compiler/Cargo.toml" --bin cinch -- run "$dir/$name"
