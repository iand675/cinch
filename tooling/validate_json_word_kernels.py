#!/usr/bin/env python3
"""Execute actual JSON SWAR primitives against every byte lane and compile their foreign objects."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify the exact production, toolchain and emitted artifact bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], directory: Path, environment: dict[str, str] | None = None, compiler_host: bool = True) -> str:
    """Keep complete diagnostics while applying only the established compiler-host stack allowance."""
    def host_stack() -> None:
        """Configure the compiler process main stack independently of the consumer's 8 MiB runtime stack."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK, ((64 if compiler_host else 8) * 1024 * 1024, hard))
    result = subprocess.run(command, cwd=directory, text=True, capture_output=True, env=environment, timeout=90, preexec_fn=host_stack)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\n{result.stdout}\n{result.stderr}")
    return result.stdout + result.stderr


def main() -> None:
    """Check both production primitives over 2048 lane mutations, two optimization levels and two memory modes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--runtime-receipt", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, default=2)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    compiler = args.compiler.resolve()
    runtime = args.runtime_receipt.resolve()
    directory = args.artifacts.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    production = ROOT / "compiler/prelude/Text/Json/Codec.cinch"
    source = production.read_text()
    primitives = []
    for name in ["decimal-eight", "binary-linebreak-eight"]:
        start = source.index(f"  prim {name} ::")
        end = source.index("\n  -- |", start)
        primitives.append("  -- | Exact load-free production SWAR primitive.\n" + source[start:end])
    body = (ROOT / "compiler/tests/fixtures/json-codec-word-kernel-body.cinch.inc").read_text()
    entry = directory / "Main.cinch"
    entry.write_text("-- | Independent runtime and foreign JSON word-kernel validation.\nmodule Main where\n  import Data.Machine\n  import Data.Bytes.Word\n" + "\n".join(primitives) + body)
    word = ROOT / "compiler/prelude/Data/Bytes/Word.cinch"
    (directory / "Data.Bytes.Word.cinch").write_bytes(word.read_bytes())
    (directory / "validator.py").write_bytes(Path(__file__).read_bytes())
    report = {"production_source_sha256":digest(production),"primitive_sha256":hashlib.sha256("\n".join(primitives).encode()).hexdigest(),"consumer_sha256":digest(entry),"control_body_sha256":digest(ROOT / "compiler/tests/fixtures/json-codec-word-kernel-body.cinch.inc"),"word_module_sha256":digest(word),"compiler_sha256":digest(compiler),"runtime_archive_sha256":digest(runtime),"runner_sha256":digest(Path(__file__)),"cases_per_run":2048,"cpu_affinity":[args.cpu],"compiler_host_stack_kib":65536,"runs":[],"foreign_targets":[],"foreign_execution_claimed":False}
    for optimization in [0,2]:
        executable = directory / f"consumer-o{optimization}"
        build = run([str(compiler),"build",str(entry),"-o",str(executable),"--opt-level",str(optimization),"--rc-debug"],directory)
        (directory / f"build-o{optimization}.log").write_text(build)
        for poison in [False,True]:
            environment = {**os.environ,"CINCH_HEAP_LIMIT":"256M" if poison else "32M","CINCH_STACK_SIZE":"8M","CINCH_RC_DEBUG":"1" if poison else "0","CINCH_LEAK_CHECK":"1","CINCH_ALLOC_STATS":"1"}
            observation = run([str(executable)],directory,environment,compiler_host=False)
            assert "live=0" in observation and "leaked=0" in observation, observation
            (directory / f"run-o{optimization}-poison{int(poison)}.log").write_text(observation)
            report["runs"].append({"opt_level":optimization,"poisoned":poison,"heap_limit":environment["CINCH_HEAP_LIMIT"],"stack_limit":"8M","executable_sha256":digest(executable),"observation":observation})
    for triple, name in [("wasm32-unknown-unknown","wasm"),("aarch64-unknown-linux-gnu","arm")]:
        ir = directory / f"{name}.ll"
        output = directory / f"{name}.o"
        diagnostics = run([str(compiler),"emit-ir",str(entry),"--target",triple,"-o",str(ir)],directory)
        diagnostics += run([os.environ.get("CINCH_CLANG","clang"),"-target",triple,"-O2","-c","-x","ir",str(ir),"-o",str(output)],directory)
        (directory / f"{name}.log").write_text(diagnostics)
        report["foreign_targets"].append({"triple":triple,"llvm_sha256":digest(ir),"object_sha256":digest(output),"object_bytes":output.stat().st_size})
    assert digest(runtime) == report["runtime_archive_sha256"]
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+"\n")


if __name__ == "__main__":
    main()
