#!/usr/bin/env python3
"""Compare nested-argument stack behavior and strict ordering through two staged emitters.

The source programs remain identical for both compilers. Matching interface,
object and runtime artifacts are explicit inputs; this does not build a compiler
or substitute another frontend after a rejection.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

from verify_stage3 import compile_client

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify exact source, compiler and artifact bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Compile unchanged probes and require bounded stack, exact results and strict prefix ordering."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path, required=True)
    parser.add_argument("--objects", type=Path, required=True)
    parser.add_argument("--runtime", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    report = {
        "description": "Identical nested checked-argument and strict-order programs through old/new staged emitters",
        "sources": {}, "compilers": {}, "artifacts": [], "executions": [],
        "interfaces_sha256": digest(args.interfaces), "objects_sha256": digest(args.objects),
        "runtime_sha256": digest(args.runtime), "cpu_affinity": sorted(os.sched_getaffinity(0)),
        "full_bootstrap_fixed_point_checked": False,
    }
    for label, source in [("nested", ROOT / "compiler/programs/Run.NestedArgumentTail.cinch"),
                          ("short", ROOT / "compiler/programs/Run.NestedArgumentTailSmall.cinch"),
                          ("order", ROOT / "compiler/programs/Run.ScopedArgumentOrder.cinch")]:
        report["sources"][label] = {"path": str(source.relative_to(ROOT)), "sha256": digest(source)}
    for variant, compiler in [("before", args.before.resolve()), ("after", args.after.resolve())]:
        directory = output / variant
        directory.mkdir(exist_ok=True)
        frozen = directory / "compiler"
        shutil.copy2(compiler, frozen)
        report["compilers"][variant] = {"path": str(compiler), "sha256": digest(frozen)}
        for label, entry in report["sources"].items():
            source = directory / f"{label}.cinch"
            source.write_bytes((ROOT / entry["path"]).read_bytes())
            ir, executable, link = directory / f"{label}.ll", directory / label, directory / f"{label}.link.txt"
            code = compile_client(str(frozen), str(args.objects.resolve()), str(args.runtime.resolve()),
                                  ["--compile-only", "--rc-debug", "--interfaces", str(args.interfaces.resolve()),
                                   str(source), str(ir), str(executable), str(link)])
            if code:
                diagnostics = ir.with_suffix(ir.suffix + ".diagnostics")
                raise RuntimeError(f"{variant}/{label} compilation failed ({code}): " +
                                   (diagnostics.read_text() if diagnostics.exists() else "no diagnostic artifact"))
            text = ir.read_text()
            worker = re.search(r'define[^\n]*@"[^"\n]*fingerprint#tail"\([^\n]*\) \{(.*?)\n\}', text, re.S)
            tail_worker = bool(worker and re.search(r'musttail call i64 @"[^"\n]*fingerprint#tail"', worker.group(1)))
            report["artifacts"].append({"variant": variant, "probe": label, "llvm_sha256": digest(ir),
                                         "executable_sha256": digest(executable), "nested_tail_worker": tail_worker})
            if label == "nested" and variant == "after" and not tail_worker:
                raise RuntimeError("new emitter did not retain a saturated recursive tail worker")
            # Equal-length names keep argv storage identical in complete-process allocation totals.
            runnable = output / ("run-old" if variant == "before" else "run-new")
            shutil.copy2(executable, runnable)
            for debug in ([False] if label == "nested" and variant == "before" else [False, True]):
                env = {**os.environ, "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M" if debug else "32M",
                       "CINCH_LEAK_CHECK": "1", "CINCH_ALLOC_STATS": "1", "CINCH_RC_DEBUG": "1" if debug else "0"}
                result = subprocess.run([str(runnable)], env=env, capture_output=True, text=True, timeout=90)
                expected = 73 if label == "nested" and variant == "before" else 0
                item = {"variant": variant, "probe": label, "debug": debug, "expected_exit": expected,
                        "actual_exit": result.returncode, "stderr": result.stderr, "stdout": result.stdout}
                stats = next((line for line in result.stderr.splitlines() if line.startswith("cinch-alloc:")), None)
                if stats:
                    item["allocation_stats"] = {key: int(value) for key, value in re.findall(r"([a-z-]+)=(\d+)", stats)}
                report["executions"].append(item)
                (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
                if result.returncode != expected:
                    raise RuntimeError(f"{variant}/{label}/debug={debug}: expected {expected}, got {result.returncode}: {result.stderr}")
                if expected == 0 and "leaked=0" not in result.stderr:
                    raise RuntimeError(f"{variant}/{label}/debug={debug}: missing zero-leak confirmation")
                print(f"{variant}/{label}/debug={debug}: expected exit {expected}", flush=True)


if __name__ == "__main__":
    main()
