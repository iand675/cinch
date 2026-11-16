#!/usr/bin/env python3
"""Observe scoped kinds and value restrictions through the actual self-built Native frontend."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import subprocess

import bootstrap

# The CLI performs dictionary resolution before the in-process checker used
# by Run.Infer. Both exact messages were reproduced with the prior verified
# compiler. The Eq messages also match fresh Stage1; retain the original
# in-process oracle and the precise CLI category as separate observations.
CLI_PRESERVED_DIAGNOSTICS = {
    "preserved-negative-14": ("Eq dictionary", "dictionary instance resolution failed: no instance for Eq a"),
    "preserved-negative-19": ("division needs checked NonZero evidence for its divisor",
                              "no evidence for `NonZero 0` is in scope here"),
    "preserved-negative-30": ("Eq dictionary", "ambiguous implicit dictionary instances"),
}


def preserved_cases(path: Path) -> list[dict]:
    """Project the existing literal source vectors without changing their programs or named refusals."""
    text = path.read_text()
    decoder = json.JSONDecoder()
    prefixes = {}
    for name in ["identity-source", "sequence-source"]:
        match = re.search(r"^  " + name + r" = (.*)$", text, re.M)
        if not match:
            raise ValueError("missing preserved source prefix: " + name)
        prefixes[name] = decoder.decode(match[1])

    def literal(expression: str) -> tuple[str, str]:
        """Accept only a source string or its existing single-prefix concatenation."""
        expression = expression.lstrip()
        prefix = ""
        if expression.startswith("string-concat "):
            match = re.match(r"string-concat ([a-z-]+)\s+", expression)
            if not match or match[1] not in prefixes:
                raise ValueError("unsupported preserved source expression")
            prefix = prefixes[match[1]]
            expression = expression[match.end():]
        value, end = decoder.raw_decode(expression)
        if not isinstance(value, str):
            raise ValueError("preserved case is not a string")
        return prefix + value, expression[end:].strip()

    cases = []
    for group, accepted, expected in [("positive", True, 26), ("negative", False, 30)]:
        match = re.search(r"^  " + group + r" =\n(.*?)^    \]", text, re.M | re.S)
        if not match:
            raise ValueError("missing preserved source group: " + group)
        rows = match[1].splitlines()
        if len(rows) != expected:
            raise ValueError("preserved vector shape changed: " + group)
        for index, line in enumerate(rows, 1):
            expression = line.strip()
            if not expression.startswith(("[ ", ", ")):
                raise ValueError("unexpected preserved vector delimiter")
            expression = expression[2:]
            if not accepted:
                if not expression.startswith("("):
                    raise ValueError("negative case is not a source/diagnostic pair")
                expression = expression[1:]
            source, rest = literal(expression)
            diagnostic = ""
            if not accepted:
                if not rest.startswith(","):
                    raise ValueError("missing preserved diagnostic")
                diagnostic, rest = literal(rest[1:])
                if rest != ")":
                    raise ValueError("unexpected preserved negative suffix")
            elif rest:
                raise ValueError("unexpected preserved positive suffix")
            cases.append({"name": f"preserved-{group}-{index:02}", "source": source,
                          "accepted": accepted, "diagnostic": diagnostic, "kinds": {}})
    return cases


def main() -> None:
    """Require actual frontend verdicts, named refusals, exported kinds, and exact retained identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, default=1)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.interfaces = args.interfaces.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    fixture = bootstrap.ROOT / "compiler/tests/fixtures/mutable-inference-vectors.json"
    preserved = bootstrap.ROOT / "bootstrap/programs/Run.Infer.cinch"
    focused = json.loads(fixture.read_text())
    cases = focused + preserved_cases(preserved)
    report = {"compiler_sha256": bootstrap.file_hash(args.compiler),
              "manifest_sha256": bootstrap.file_hash(args.interfaces),
              "focused_fixture_sha256": bootstrap.file_hash(fixture),
              "preserved_fixture_sha256": bootstrap.file_hash(preserved),
              "focused_cases": len(focused), "preserved_cases": len(cases) - len(focused),
              "status": "running", "results": []}
    bootstrap.write_json(args.output, report)
    environment = bootstrap.environment()
    environment.update(CINCH_LEAK_CHECK="1", CINCH_RC_DEBUG="0")
    for case in cases:
        directory = args.artifacts / case["name"]
        directory.mkdir()
        source, prefix = directory / "Main.cinch", directory / "checked"
        source.write_text(case["source"])
        mode = "--emit-interfaces" if case["accepted"] and case["kinds"] else "--check"
        command = ["taskset", "-c", str(args.cpu), str(args.compiler), mode, "--interfaces",
                   str(args.interfaces), str(source), str(prefix)]
        result = subprocess.run(command, env=environment, capture_output=True, text=True)
        log = directory / "check.log"
        log.write_text(result.stdout + result.stderr)
        diagnostics = Path(str(prefix) + ".diagnostics")
        details = diagnostics.read_text() if diagnostics.exists() else result.stderr
        expected = case["diagnostic"]
        if case["name"] in CLI_PRESERVED_DIAGNOSTICS:
            original, adapted = CLI_PRESERVED_DIAGNOSTICS[case["name"]]
            if expected != original:
                raise ValueError("preserved diagnostic oracle changed: " + case["name"])
            expected = adapted
        row = {"name": case["name"], "accepted": case["accepted"], "exit": result.returncode,
               "diagnostic": details, "command": command, "source_sha256": bootstrap.file_hash(source),
               "log_sha256": bootstrap.file_hash(log), "kinds": {},
               "original_expected_diagnostic": case["diagnostic"], "cli_expected_diagnostic": expected}
        valid = (result.returncode == (0 if case["accepted"] else 4)
                 and "leaked=0" in result.stderr
                 and (not details if case["accepted"] else expected in details))
        if valid and case["kinds"]:
            documents = {document["module"]: document for path in directory.glob("checked.*.cinchif")
                         if isinstance(document := json.loads(path.read_text()), dict)}
            for identity, expected in case["kinds"].items():
                module, name = identity.removeprefix("m.").rsplit(".", 1)
                declarations = documents[module]["exports"]["types"]
                actual = next([parameter["kind"] for parameter in declaration["parameters"]]
                              for declaration in declarations if declaration["name"] == name)
                row["kinds"][identity] = actual
                valid = valid and actual == expected
        row["status"] = "passed" if valid else "failed"
        report["results"].append(row)
        report["status"] = "running" if valid else "failed"
        bootstrap.write_json(args.output, report)
        if not valid:
            raise bootstrap.BootstrapError("self-built mutable source case failed: " + case["name"] + ": " + details)
    report["status"] = "passed"
    bootstrap.write_json(args.output, report)
    print(f"{len(focused)} scoped-kind/value cases and {len(cases) - len(focused)} preserved source cases passed.")


if __name__ == "__main__":
    main()
