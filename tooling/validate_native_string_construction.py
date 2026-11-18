#!/usr/bin/env python3
"""Check string construction through an explicit fresh Native base object, without source shadowing."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

import bootstrap
import bootstrap_objects
from bench_stdlib import source_closure
from validate_string_construction import checks, consumer, digest

ROOT = Path(__file__).resolve().parents[1]


def missing_list_object_variant(name: str, code: int, diagnostics: str) -> bool:
    """Recognize only the named O-variant emission refusal, not a source-checking failure."""
    return (name == "list-string-O" and code == 5
            and {"symbol": "Data.List.is-empty", "classes": "O"}
            in bootstrap_objects.requested_roots(diagnostics)["units"])


def main() -> None:
    """Link every consumer against one admitted provider and retain exact source/object/runtime identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--library-artifact", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--opt-level", type=int, choices=[0, 2], required=True,
                        help="consumer LLVM optimization; the selected base object retains its recorded producer profile")
    args = parser.parse_args()
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    args.artifacts.mkdir(parents=True, exist_ok=False)
    manifest, objects = args.artifacts / "interfaces.json", args.artifacts / "objects.json"
    bootstrap.write_json(manifest, provider["interfaces"])
    bootstrap.write_json(objects, provider["configuration"])
    fixture_path = ROOT / "compiler/tests/fixtures/string-construction-vectors.json"
    fixture = json.loads(fixture_path.read_text())
    expected = {row[0]: row[1] for row in provider["interfaces"]["interfaces"]}["Data.String"]
    report = {"compiler_sha256": digest(args.compiler), "provider": provider["identity"],
              "base_object_sha256": digest(provider["object"]), "runtime_sha256": digest(provider["runtime"]),
              "fixture_sha256": digest(fixture_path), "consumer_opt_level": args.opt_level,
              "source_shadow_used": False, "abi_probes": [], "results": [], "status": "running"}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    bootstrap.write_json(args.output, report)
    for name, body in [
        ("concrete-string", "if string-byte-length (string-repeat 65536 \"x\") == 65536 then 0 else 1"),
        ("list-string-O", 'let value = string-repeat 257 "é" in let values = Cons value Nil in '
         'if is-empty values then 1 else if is-empty ([] :: List String) then '
         'if string-byte-length value == 514 then 0 else 3 else 2'),
    ]:
        project = (args.artifacts / name).resolve()
        project.mkdir()
        entry, ir, executable = project / "Main.cinch", project / "probe.ll", project / "probe"
        entry.write_text("-- | Observe a concrete base ABI call without a source override.\nmodule Main where\n  import Data.String\n  import Data.List (is-empty)\n  -- | Require the exact public result through the selected artifact.\n  main :: Int\n  main = " + body + "\n")
        command = [str(args.compiler.resolve()), "--emit-ir", "--rc-debug", "--interfaces", str(manifest.resolve()),
                   "--objects", str(objects.resolve()), str(entry), str(ir)]
        compiled = subprocess.run(command, env=bootstrap.environment(), cwd=project, capture_output=True, text=True)
        diagnostic = ir.with_name(ir.name + ".diagnostics")
        text = diagnostic.read_text() if diagnostic.exists() else compiled.stderr
        probe = {"name": name, "compile_exit": compiled.returncode, "diagnostics": text, "command": command,
                 "entry_sha256": digest(entry)}
        if compiled.returncode:
            demands = bootstrap_objects.requested_roots(text)
            metadata = bootstrap.read_json(Path(provider["configuration"]["metadata"]))
            probe.update(status="failed", demands=demands,
                         named_missing_list_object_variant=missing_list_object_variant(name, compiled.returncode, text),
                         emitted_is_empty_variants=[row for row in metadata["units"] if row["symbol"].startswith("Data.List.is-empty")])
            report.update(status="failed", failure=probe)
            report["abi_probes"].append(probe)
            bootstrap.write_json(args.output, report)
            raise bootstrap.BootstrapError(f"required ABI probe rejected: {name}: {text}")
        else:
            link = [os.environ.get("CINCH_CLANG", "clang"), str(ir), str(provider["object"]), str(provider["runtime"]),
                    f"-O{args.opt_level}", "-o", str(executable)]
            if bootstrap.HOST["system"] == "Linux":
                link.extend(["-ldl", "-lpthread", "-lm"])
            subprocess.run(link, env=bootstrap.environment(), capture_output=True, text=True, check=True)
            runs = []
            for debug in [False, True]:
                result = subprocess.run([str(executable)],
                    env={**os.environ, "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M" if debug else "32M",
                         "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "1" if debug else "0"},
                    capture_output=True, text=True, timeout=180)
                runs.append({"debug": debug, "exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr})
                if result.returncode or "leaked=0" not in result.stderr:
                    raise bootstrap.BootstrapError(f"ABI probe {name}/debug={debug}: {result.returncode}: {result.stderr}")
            probe.update(status="passed", executable_sha256=digest(executable), llvm_sha256=digest(ir), link_command=link, runs=runs)
        report["abi_probes"].append(probe)
        bootstrap.write_json(args.output, report)
    for group in ["repeats", "concats", "regression", "owners"]:
        project = (args.artifacts / group).resolve()
        project.mkdir()
        if group == "owners":
            rows, expressions = [None], ["owners-case ()"]
        elif group == "regression":
            repeat = next(row for row in fixture["repeats"] if row["text"] == "x" and row["count"] == 65536)
            concat = next(row for row in fixture["concats"] if row["pieces"] == ["x"] and row["cycles"] == 65536)
            rows, expressions = [repeat, concat], checks("repeats", [repeat]) + checks("concats", [concat])
        else:
            rows = fixture[group]
            expressions = checks(group, rows)
        source = consumer(expressions).replace("module Main where\n", "module Main where\n  export main\n", 1)
        sources = {"Main.cinch": source.encode()}
        for name, path in source_closure(source).items():
            if name == "Data.String":
                raise bootstrap.BootstrapError("a Native provider consumer must not shadow Data.String")
            shutil.copyfile(path, project / (name + ".cinch"))
            sources[name + ".cinch"] = path.read_bytes()
        entry = project / "Bundle.cinch"
        entry.write_bytes(bootstrap.bundle(sources))
        ir, executable = project / "consumer.ll", project / "consumer"
        command = [str(args.compiler.resolve()), "--emit-ir-with-interfaces", "--rc-debug", "--interfaces", str(manifest.resolve()),
                   "--objects", str(objects.resolve()), str(entry), str(ir)]
        with (project / "compile.log").open("w") as stream:
            compiled = subprocess.run(command, env=bootstrap.environment(), cwd=project, stdout=stream, stderr=subprocess.STDOUT)
        if compiled.returncode:
            diagnostics = ir.with_name(ir.name + ".diagnostics")
            report.update(status="failed", failure={"group": group, "compile_exit": compiled.returncode,
                          "diagnostics": diagnostics.read_text() if diagnostics.exists() else (project / "compile.log").read_text()})
            bootstrap.write_json(args.output, report)
            raise bootstrap.BootstrapError(f"{group}: selected Native compilation failed; see {project / 'compile.log'}")
        imports = json.loads(ir.with_name(ir.name + ".Main.cinchif").read_text())["imports"]
        if not any(row["module"] == "Data.String" and row["iface_hash"] == expected for row in imports):
            raise bootstrap.BootstrapError("consumer does not reference the selected String contract")
        if (project / "Data.String.cinch").exists() or re.search(r'define[^\n]*@"Data\.String\.(?:string-repeat|string-concat-all|repeat-double|concat-copy|concat-size)', ir.read_text()):
            raise bootstrap.BootstrapError("consumer contains a String implementation instead of importing its base object")
        link = [os.environ.get("CINCH_CLANG", "clang"), str(ir), str(provider["object"]), str(provider["runtime"]),
                f"-O{args.opt_level}", "-o", str(executable)]
        if bootstrap.HOST["system"] == "Linux":
            link.extend(["-ldl", "-lpthread", "-lm"])
        with (project / "link.log").open("w") as stream:
            subprocess.run(link, env=bootstrap.environment(), stdout=stream, stderr=subprocess.STDOUT, check=True)
        runs = []
        for debug in [False, True]:
            env = {**os.environ, "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M" if debug else "32M",
                   "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "1" if debug else "0"}
            result = subprocess.run([str(executable)], env=env, capture_output=True, text=True, timeout=180)
            runs.append({"debug": debug, "exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr})
            if result.returncode != 0 or "leaked=0" not in result.stderr:
                raise bootstrap.BootstrapError(f"{group}/debug={debug}: {result.returncode}: {result.stderr}")
        report["results"].append({"group": group, "cases": len(rows), "compile_command": command, "link_command": link,
                                  "entry_sha256": digest(entry), "llvm_sha256": digest(ir),
                                  "executable_sha256": digest(executable), "runs": runs})
        args.output.parent.mkdir(parents=True, exist_ok=True)
        bootstrap.write_json(args.output, report)
        print(f"{group}: {len(rows)} cases, imported fresh base, both ownership modes passed", flush=True)
    bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    report["status"] = "passed"
    bootstrap.write_json(args.output, report)


if __name__ == "__main__":
    main()
