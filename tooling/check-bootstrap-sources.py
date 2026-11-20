#!/usr/bin/env python3
"""Type-check the bootstrap sources with an already-built stage compiler.

The 11-16 minute path rebuilds the seed and every stage. When a change is
confined to `compiler/*.cinch` (not a seed source, so no lock bump), the
stage-3 compiler on disk is the compiler those sources are written for, and
it can check them directly: they are bundled the way
`tooling/bootstrap.py` bundles them for a stage build (one file, every
module's source under a `-- source:` header) and checked as one unit.

That is exactly the question a bootstrap-source edit asks: do these modules
still resolve, kind-check and type-check together? It is not the fixed
point, the smokes or the runtime tests; `tooling/bootstrap.py build` is
still the gate for those.

  python3 tooling/check-bootstrap-sources.py [--compiler PATH] [--stage N]
"""

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import bootstrap


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--compiler", default=None,
                        help="a stage compiler (default: build/bootstrap/stage3/cinch)")
    parser.add_argument("--stage", type=int, default=3)
    args = parser.parse_args()
    compiler = Path(args.compiler) if args.compiler else (
        bootstrap.BUILD / f"stage{args.stage}" / "cinch")
    if not compiler.is_file():
        sys.exit(f"no compiler at {compiler}")

    directory = Path(tempfile.mkdtemp(prefix="cinch-bootstrap-sources-"))
    entry = directory / "compiler.cinch"
    files = bootstrap.compiler_sources()
    entry.write_bytes(bootstrap.bundle(files))
    env = {k: v for k, v in os.environ.items() if not k.startswith("CINCH_")}
    env["CINCH_HEAP_LIMIT"] = env.get("CINCH_HEAP_LIMIT", "2G")
    # The bundle is the compiler's own sources, so the base interface set is
    # what its imports resolve against; without it every prelude import is
    # an unresolved module.
    manifest = compiler.parent / "base-interfaces.json"
    command = [str(compiler), "--check", str(entry), str(directory / "diagnostics.txt")]
    if manifest.is_file():
        command[1:1] = ["--interfaces", str(manifest)]
    completed = subprocess.run(command, cwd=directory, env=env,
                               capture_output=True, text=True)
    diagnostics = directory / "diagnostics.txt"
    if diagnostics.is_file():
        text = diagnostics.read_text(errors="replace")
        if text.strip():
            print(text, end="")
    if completed.returncode != 0:
        print(completed.stdout, end="")
        print(completed.stderr, end="", file=sys.stderr)
        sys.exit(f"check-bootstrap-sources: FAIL (exit {completed.returncode}); "
                 f"bundle kept at {directory}")
    print(f"check-bootstrap-sources: every bootstrap module checks "
          f"({len(files)} modules, {compiler})")


if __name__ == "__main__":
    main()
