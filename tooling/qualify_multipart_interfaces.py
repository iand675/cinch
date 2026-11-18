#!/usr/bin/env python3
"""Check complete multipart session contracts through genuine source-absent artifacts.

The original compiler manifest supplies every interface hash. Interface bytes
are copied without rewriting schemes, nominal roles, capabilities or identities.
This check does not claim a runtime provider contains new multipart units.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
import time
from check_compare import digest


#: Ordinary client wrappers preserve the full callback and shared mutation region.
CLIENT = """-- | Genuine source-absent multipart session and callback contract controls.
module Main where
  import Effect.Multipart
  import Data.Bytes
  import Data.Option
  import Data.Result

  -- | Forward a live part without hiding either required capability.
  forward-part :: forall (r :: Region) e. MultipartPart r e -> Nat ->{IO r, Mut r} (Bytes, MultipartReadStatus (MultipartError e))
  forward-part part wanted = multipart-part-read part wanted

  -- | Forward arbitrary backend key, handle, state and caller error identities.
  forward-file :: forall (r :: Region) e key fileHandle state. MultipartFile r e key fileHandle state -> Nat ->{IO r, Mut r} (Bytes, MultipartReadStatus (MultipartError e))
  forward-file file wanted = multipart-file-read file wanted

  -- | Retain a caller source callback through the original pure factory.
  forward-source :: forall (r :: Region) e. (Nat ->{IO r} ((Bytes, MultipartReadStatus e), MultipartSource r e)) -> MultipartSource r e
  forward-source callback = multipart-source callback

  -- | Preserve both the optional form and independent deferred-close error.
  forward-form :: forall (r :: Region) e key fileHandle state. MultipartFormReadOutcome r e key fileHandle state -> (Option (MultipartForm r e key fileHandle state), Option (MultipartError e))
  forward-form outcome = (multipart-form-read-form outcome, multipart-form-read-error outcome)

  -- | Finish checking without constructing a fabricated session or backend.
  main :: Unit -> Int
  main unit = 0
"""


def main() -> None:
    """Copy a checked artifact closure and preserve actual admission or refusal output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--emission", type=Path, required=True)
    parser.add_argument("--native", type=Path, required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    artifacts = work / "interfaces"; artifacts.mkdir(exist_ok=True)
    emitted = args.emission.resolve() / "build"
    manifest_path = emitted / "Main.manifest.json"
    original = json.loads(manifest_path.read_text())
    rows, hashes = [], {}
    for row in original["modules"]:
        name = row["module"]
        if name == "Main":
            continue
        source, target = emitted / (name + ".cinchif"), artifacts / (name + ".cinchif")
        shutil.copyfile(source, target)
        rows.append([name, row["iface_hash"], str(target)])
        hashes[name] = digest(target)
    manifest = work / "interfaces.json"
    manifest.write_text(json.dumps({"schema": 1, "interfaces": rows}, indent=2) + "\n")
    entry = work / "Main.cinch"; entry.write_text(CLIENT)
    compiler = args.native.resolve() / "cinch"
    command = [str(compiler), "--check", "--interfaces", str(manifest), str(entry), str(work / "check.output")]
    started = time.monotonic()
    result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=45,
                            env={**os.environ, "CINCH_STACK_SIZE": "64M"})
    report = {"scope": __doc__, "runtime_claim": False, "source_manifest": str(manifest_path),
              "source_manifest_sha256": digest(manifest_path), "artifact_sha256": hashes,
              "compiler_sha256": digest(compiler), "client_sha256": digest(entry),
              "manifest_sha256": digest(manifest), "command": command, "returncode": result.returncode,
              "seconds": time.monotonic() - started, "stdout": result.stdout, "stderr": result.stderr,
              "diagnostics": {str(path): path.read_text() for path in work.glob("*.diagnostics")}}
    (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(result.stdout + result.stderr, end="")
    for diagnostic in report["diagnostics"].values():
        print(diagnostic, end="")
    raise SystemExit(result.returncode)


if __name__ == "__main__":
    main()
