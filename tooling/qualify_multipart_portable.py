#!/usr/bin/env python3
"""Emit complete public multipart paths and real foreign-target objects.

Host execution is covered separately by the independent corpus. These checks
claim object compilation only, preserve target-specific checked IR and require
the actual vector and scalar-fringe definitions to remain reachable.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
from bench_stdlib import source_closure
from check_compare import digest, run


#: Repository root for checked public source and the maintained portable entrypoint.
ROOT = Path(__file__).resolve().parents[1]


def paths(ir: str) -> dict[str, list[str]]:
    """Follow actual direct calls from the entrypoint through scanner and escape workers."""
    definitions = {}
    for match in re.finditer(r'^define [^\n]*@"([^"]+)"[^\n]*\{\n', ir, re.M):
        definitions[match.group(1)] = ir[match.start():ir.index('\n}', match.end()) + 2]
    pending, seen, found = [("Main.main", ["Main.main"])], set(), {}
    wanted = {"Text.Multipart.find-byte", "Text.Multipart.escape-end", "Text.Multipart.prefix-at", "Text.Multipart.equal-sixteen.llvm"}
    while pending:
        name, chain = pending.pop()
        if name in seen or name not in definitions:
            continue
        seen.add(name)
        if name in wanted:
            found[name] = chain
        for callee in re.findall(r'\bcall\s+[^\n]*?@"([^"]+)"\(', definitions[name]):
            pending.append((callee, [*chain, callee]))
    if found.keys() != wanted:
        raise RuntimeError("missing public worker paths: " + str(wanted - found.keys()))
    matcher = definitions["Text.Multipart.equal-sixteen.llvm"]
    if "icmp eq <16 x i8>" not in matcher or "bitcast <16 x i1>" not in matcher:
        raise RuntimeError("missing actual checked vector matcher")
    return found


def main() -> None:
    """Freeze complete public sources, compile bounded targets and retain exact code evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    source = ROOT / "seed/tests/fixtures/multipart-portable.cinch"
    dependencies = source_closure(source.read_text())
    report = {"scope": __doc__, "compiler_sha256": digest(args.compiler), "entry_sha256": digest(source),
              "dependencies": {name: digest(path) for name, path in dependencies.items()},
              "compiler_host_stack_bytes": 67108864, "runtime_execution": False, "targets": []}
    for target in ("wasm32-unknown-unknown", "aarch64-unknown-linux-gnu"):
        folder = work / target; folder.mkdir(exist_ok=True)
        entry = folder / "Main.cinch"; shutil.copyfile(source, entry)
        for name, path in dependencies.items():
            shutil.copyfile(path, folder / (name + ".cinch"))
        llvm = folder / "consumer.ll"
        command = [str(args.compiler.resolve()), "emit-ir", str(entry), "--target", target, "-o", str(llvm)]
        row = {"target": target, "command": command}; report["targets"].append(row)
        try:
            checked = run(command, folder, 120, {**os.environ, "CINCH_STACK_SIZE": "64M", "RUST_MIN_STACK": "67108864"})
            row.update(compiler_stdout=checked.stdout, compiler_stderr=checked.stderr, ir_sha256=digest(llvm))
            row["actual_worker_paths"] = paths(llvm.read_text())
            obj = folder / "consumer.o"
            object_command = [os.environ.get("CINCH_CLANG", "clang"), "-target", target, "-O2", "-c", str(llvm), "-o", str(obj)]
            emitted = run(object_command, folder)
            row.update(object_command=object_command, object_stdout=emitted.stdout, object_stderr=emitted.stderr,
                       object_sha256=digest(obj), object_bytes=obj.stat().st_size)
        except Exception as error:
            row["failure"] = str(error)
            raise
        finally:
            (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")
        print(target, "complete public object passed", flush=True)


if __name__ == "__main__":
    main()
