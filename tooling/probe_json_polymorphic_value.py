#!/usr/bin/env python3
"""Compare a retained computed-polymorphic Map binding with a final raw Native compiler epoch."""
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
    """Identify exact diagnostic sources and immutable seed/provider inputs."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Run one raw emit-only diagnostic without compiler fallback or consumer memory changes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--snapshot", type=Path, required=True)
    parser.add_argument("--source-project", type=Path, default=ROOT / "build/json-map-polymorphic-value-repro")
    parser.add_argument("--entry", choices=["Main.cinch", "Annotated.cinch"], default="Main.cinch")
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, default=2)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    snapshot = args.snapshot.resolve()
    compiler = snapshot / "cinch"
    project = args.source_project.resolve()
    directory = args.artifacts.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    modules = ["Data.Array","Data.Bytes.Word","Data.Unicode.Tables","Data.Unicode.Kernel","Text.Json.Value","Text.Base64.Encoding","Text.Json.Codec"]
    files = [project / f"{module}.cinch" for module in modules] + [project / args.entry]
    bundle = directory / "Bundle.cinch"
    bundle.write_text("\n\n".join(path.read_text() for path in files))
    (directory / "probe.py").write_bytes(Path(__file__).read_bytes())
    ir = directory / "consumer.ll"
    command = [str(compiler),"--emit-ir","--rc-debug","--interfaces",str(snapshot / "base-interfaces.json"),"--objects",str(snapshot / "objects.json"),str(bundle),str(ir)]
    def compiler_stack() -> None:
        """Apply only the established64MiB compiler-host main-stack allowance."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK,(64 * 1024 * 1024,hard))
    environment = {**os.environ,"CINCH_HEAP_LIMIT":"2G","CINCH_RC_DEBUG":"0","CINCH_LEAK_CHECK":"1"}
    result = subprocess.run(command,cwd=directory,env=environment,capture_output=True,text=True,timeout=90,preexec_fn=compiler_stack)
    (directory / "stdout.log").write_text(result.stdout)
    (directory / "stderr.log").write_text(result.stderr)
    diagnostic = ir.with_name(ir.name + ".diagnostics")
    text = diagnostic.read_text() if diagnostic.exists() else ""
    matched = result.returncode != 0 and "polymorphic or aggregate let needs runtime lowering" in text
    report = {"description":"Separate raw-Native diagnostic epoch; original control and library contracts remain preserved","compiler_sha256":digest(compiler),"interfaces_sha256":digest(snapshot / "base-interfaces.json"),"objects_sha256":digest(snapshot / "objects.json"),"runtime_archive_sha256":digest(snapshot / "libcinchruntime.a"),"source_files":{path.name:digest(path) for path in files},"bundle_sha256":digest(bundle),"runner_sha256":digest(Path(__file__)),"command":command,"exit_code":result.returncode,"diagnostics":text,"reproduces_polymorphic_let_refusal":matched,"emit_succeeded":result.returncode == 0,"consumer_executed":False,"compiler_host_stack_kib":65536,"cpu_affinity":[args.cpu]}
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+"\n")
    if result.returncode and not matched:
        raise RuntimeError(f"different diagnostic ({result.returncode}): {text}\n{result.stderr}")
    print("computed polymorphic let refusal reproduced" if matched else "computed polymorphic let emitted successfully")


if __name__ == "__main__":
    main()
