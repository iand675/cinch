#!/usr/bin/env python3
"""Project offline Base64 fixtures into real consumers and validate selected frontends."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "compiler/tests/fixtures/base64-encoding-vectors.json"
HELPERS = ROOT / "compiler/tests/fixtures/base64-encoding-consumer.cinch"
MODULES = {
    "Text.Base64.Encoding": "compiler/prelude/Text/Base64/Encoding.cinch",
    "Effect.Base64": "compiler/prelude/Effect/Base64.cinch",
    "Effect.Stream": "compiler/prelude/Effect/Stream.cinch",
    "Text.Hex": "compiler/prelude/Text/Hex.cinch",
    "Data.Bytes.Word": "compiler/prelude/Data/Bytes/Word.cinch",
}
GROUPS = [*(f"valid-{i}" for i in range(9)), *(f"malformed-{i}" for i in range(9)), "strict", "buffers", "streams", "long-stream"]


def flag(value: bool) -> str:
    """Render an explicit checked-language boolean rather than an integer coercion."""
    return "True" if value else "False"


def numbers(values: list[int]) -> str:
    """Render a finite Nat split list without expressions or hidden runtime dependencies."""
    return "[" + ",".join(map(str, values)) + "]"


def quoted(value: str) -> str:
    """Preserve ASCII fixture text with exact escapes in a source string literal."""
    return json.dumps(value)


def configuration(fixture: dict) -> str:
    """Use public fixed constructors and recorded independent custom alphabet bytes."""
    entries = ["base64-standard-encoding", "(base64-strict base64-standard-encoding)", "base64-url-encoding", "base64-raw-standard-encoding", "(base64-strict base64-raw-url-encoding)"]
    for scheme in fixture["schemes"][5:]:
        padding = "None" if scheme["padding"] == -1 else f"(Some {scheme['padding']})"
        entries.append(f"(custom {quoted(scheme['alphabet'])} {padding} {flag(scheme['strict'])})")
    lines = ["  -- | Select each independent fixture's exact public encoding configuration.", "  configuration :: Nat -> Base64Encoding", "  configuration n ="]
    lines += [f"    {'if' if i == 0 else 'else if'} n == {i} then {entry}" for i, entry in enumerate(entries[:-1])]
    lines += [f"    else {entries[-1]}", ""]
    return "\n".join(lines)


def source(name: str) -> str:
    """Project one bounded fixture group into a real standalone checked consumer."""
    fixture = json.loads(FIXTURE.read_text())
    checks = []
    if name.startswith("valid-"):
        index = int(name.split("-")[1])
        for row in fixture["valid"]:
            if row["config"] != index:
                continue
            # Long buffers exercise exact whole-buffer results and bounds; exhaustive
            # short feeds and selected medium splits validate the state machine separately.
            enc = row["encoder_splits"] if row["size"] <= 257 else []
            dec = row["decoder_splits"] if row["size"] <= 257 else []
            checks.append(f"valid (configuration {index}) {row['size']} {quoted(row['encoded'])} {row['fingerprint']} {row['encoded_length']} {row['decoded_bound']} {numbers(enc)} {numbers(dec)}")
    elif name.startswith("malformed-"):
        index = int(name.split("-")[1])
        for row in fixture["malformed"]:
            if row["config"] == index:
                checks.append(f"malformed (configuration {index}) {quoted(row['input'])} {quoted(row['decoded'])} {flag(row['error'])}")
    elif name == "strict":
        for row in fixture["strict"]:
            encoding = "base64-raw-standard-encoding" if row["raw"] else "base64-standard-encoding"
            checks.append(f"malformed (base64-strict {encoding}) {quoted(row['input'].encode().hex())} {quoted(row['decoded'])} {flag(row['error'])}")
        checks += ["precise-errors ()", "configuration-errors ()", "text-conversion ()"]
        for row in fixture["construct"]:
            alphabet = bytes.fromhex(row["input"])
            if len(alphabet) != 64:
                pattern = f"Base64AlphabetLength {len(alphabet)}"
            elif "newline" in row["reference_failure"]:
                offset = next(i for i, value in enumerate(alphabet) if value in (10, 13))
                pattern = f"Base64AlphabetNewline {offset}"
            else:
                offset = next(i for i, value in enumerate(alphabet) if value in alphabet[:i])
                pattern = f"Base64AlphabetDuplicate {offset}"
            checks.append(f"(case base64-encoding (hex {quoted(row['input'])}) of Err ({pattern}) -> True; other -> False)")
    elif name == "buffers":
        checks = ["buffers ()", "aliases ()"]
    elif name == "streams":
        checks = ["writer-case ()", "failed-write ()", "reader-case ()"]
    elif name == "long-stream":
        checks = ["long-stream ()"]
    else:
        raise ValueError(name)
    body = ",\n      ".join(checks)
    return HELPERS.read_text() + "\n" + configuration(fixture) + f"\n  -- | Execute one complete independent fixture group under a single runtime identity.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if all [\n      {body}\n    ] then 0 else 1\n"


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run one bounded command and retain useful failure output without fallback."""
    result = subprocess.run(command, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\n{result.stdout}\n{result.stderr}")
    return result


def digest(path: Path) -> str:
    """Identify exact compiler, source and fixture bytes for a validation report."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Emit an offline consumer or validate it through one explicitly selected frontend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--emit", choices=GROUPS)
    parser.add_argument("--compiler", type=Path)
    parser.add_argument("--native-wrapper", type=Path)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--case", nargs="+", choices=GROUPS, default=GROUPS)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--artifacts", type=Path)
    parser.add_argument("--cpu", type=int, default=4)
    args = parser.parse_args()
    if args.emit:
        print(source(args.emit), end="")
        return
    if bool(args.compiler) == bool(args.native_wrapper):
        parser.error("select exactly one of --compiler or --native-wrapper")
    if args.native_wrapper and not args.interfaces:
        parser.error("--interfaces is required with --native-wrapper")
    os.sched_setaffinity(0, {args.cpu})
    compiler = (args.compiler or args.native_wrapper).resolve()
    report = {"description": "Offline configurable Base64 wire, malformed-input, ownership and streaming validation", "frontend": "native" if args.native_wrapper else "rust", "compiler_sha256": digest(compiler), "fixture_sha256": digest(FIXTURE), "consumer_helpers_sha256": digest(HELPERS), "projector_sha256": digest(Path(__file__)), "source_sha256": {str(path): digest(ROOT / path) for path in MODULES.values()}, "cases": []}
    with tempfile.TemporaryDirectory(prefix="cinch-base64-validation-") as directory:
        work = Path(directory)
        for name, path in MODULES.items():
            shutil.copyfile(ROOT / path, work / f"{name}.cinch")
        for name in args.case:
            entry = work / "Main.cinch"
            entry.write_text(source(name))
            exe = work / "consumer"
            if args.native_wrapper:
                bundle = work / "Bundle.cinch"
                bundle.write_text("\n".join((ROOT / MODULES[name]).read_text() for name in ["Data.Bytes.Word", "Text.Hex", "Text.Base64.Encoding", "Effect.Stream", "Effect.Base64"]) + "\n" + entry.read_text())
                try:
                    run(["python3", str(compiler), "--compile-only", "--rc-debug", "--interfaces", str(args.interfaces.resolve()), str(bundle), str(exe.with_suffix(".ll")), str(exe), str(work / "link-report.txt")], cwd=work, timeout=600)
                except Exception:
                    if args.artifacts:
                        shutil.copytree(work, args.artifacts / f"failed-{name}", dirs_exist_ok=True)
                    raise
            else:
                run([str(compiler), "build", str(entry), "-o", str(exe), "--opt-level", "2", "--rc-debug"], cwd=work, timeout=600)
            for debug in [False, True]:
                result = run([str(exe)], cwd=work, timeout=180, env={**os.environ, "CINCH_RC_DEBUG": "1" if debug else "0", "CINCH_LEAK_CHECK": "1", "CINCH_HEAP_LIMIT": "256M" if debug else "32M", "CINCH_STACK_SIZE": "8M"})
                if "leaked=0" not in result.stderr:
                    raise RuntimeError(result.stderr)
            if args.artifacts:
                target = args.artifacts / name
                target.mkdir(parents=True, exist_ok=True)
                for path in [entry, exe, exe.with_suffix(".ll")]:
                    shutil.copyfile(path, target / path.name)
            report["cases"].append({"name": name, "ordinary_heap": "32M", "poisoned_heap": "256M", "stack": "8M", "zero_leaks": True})
            print(f"{report['frontend']} {name}: passed both ownership modes", flush=True)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
