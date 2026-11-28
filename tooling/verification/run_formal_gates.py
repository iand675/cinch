#!/usr/bin/env python3
"""Unified CI formal verification gate runner for Cinch.

Runs:
  1. Strict primitive verification (`--prim-verify=strict` / `verify-prims --policy=jpl --audit-unsafe`)
  2. TCB allowlist audit (`check_tcb_allowlist.py`) over `build/<Entry>.manifest.json`
  3. Exhaustive temporal model checking (`cinch model inspect --json`, asserting `capped == false`,
     zero failed contracts, and zero failed refinements)
  4. Runtime RC & leak verification (`CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1`)
  5. Optional kernel ABI verification (`verify_kernel_abi.py`) & cache-line isolation audit
     (`cinch verify-prims --audit-cache-line=...`) when `.cinchif` files are supplied.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
DEFAULT_ALLOWLIST = HERE / "tcb_allowlist.json"


def run_cmd(cmd: list[str], env: dict[str, str] | None = None, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, capture_output=True, text=True, env=env, cwd=cwd)


def gate_prim_verify_strict(
    cinch_bin: str,
    entries: list[Path],
    verify_files: list[Path],
    out_dir: Path | None,
) -> tuple[dict[str, Any], list[Path]]:
    details: list[dict[str, Any]] = []
    discovered_manifests: list[Path] = []
    passed = True

    for entry in entries:
        cmd = [cinch_bin, "build", "--prim-verify=strict", str(entry)]
        if out_dir is not None:
            cmd.extend(["--out-dir", str(out_dir)])
        proc = run_cmd(cmd)
        ok = proc.returncode == 0
        if not ok:
            passed = False
        details.append(
            {
                "kind": "build-strict",
                "target": str(entry),
                "returncode": proc.returncode,
                "stdout": proc.stdout.strip(),
                "stderr": proc.stderr.strip(),
                "passed": ok,
            }
        )
        search_dir = out_dir if out_dir is not None else Path("build")
        candidate = search_dir / f"{entry.stem}.manifest.json"
        if candidate.is_file():
            discovered_manifests.append(candidate)

    for vf in verify_files:
        cmd = [cinch_bin, "verify-prims", "--policy=jpl", "--audit-unsafe", str(vf)]
        proc = run_cmd(cmd)
        ok = proc.returncode == 0
        if not ok:
            passed = False
        details.append(
            {
                "kind": "verify-prims-jpl",
                "target": str(vf),
                "returncode": proc.returncode,
                "stdout": proc.stdout.strip(),
                "stderr": proc.stderr.strip(),
                "passed": ok,
            }
        )

    return {"gate": "prim_verify_strict", "passed": passed, "checks": details}, discovered_manifests


def gate_tcb_allowlist(manifests: list[Path], allowlist: Path, strict_jpl: bool) -> dict[str, Any]:
    if not manifests:
        return {"gate": "tcb_allowlist", "passed": True, "skipped": True, "reason": "no manifests provided"}
    cmd = [sys.executable, str(HERE / "check_tcb_allowlist.py"), "--allowlist", str(allowlist), "--json"]
    if strict_jpl:
        cmd.append("--strict-jpl")
    cmd.extend(str(m) for m in manifests)
    proc = run_cmd(cmd)
    payload: Any = {}
    if proc.stdout.strip():
        try:
            payload = json.loads(proc.stdout)
        except json.JSONDecodeError:
            payload = {"raw": proc.stdout.strip()}
    return {
        "gate": "tcb_allowlist",
        "passed": proc.returncode == 0,
        "returncode": proc.returncode,
        "report": payload,
        "stderr": proc.stderr.strip(),
    }


def gate_model_inspect(cinch_bin: str, model_files: list[Path]) -> dict[str, Any]:
    if not model_files:
        return {"gate": "model_inspect", "passed": True, "skipped": True, "reason": "no model files provided"}
    passed = True
    checks = []
    for mf in model_files:
        cmd = [cinch_bin, "model", "inspect", str(mf), "--json"]
        proc = run_cmd(cmd)
        violations = []
        machines = []
        if proc.returncode != 0:
            passed = False
            violations.append(f"cinch model inspect exited with {proc.returncode}: {proc.stderr.strip()}")
        else:
            try:
                machines = json.loads(proc.stdout)
            except json.JSONDecodeError as exc:
                passed = False
                violations.append(f"invalid JSON from cinch model inspect: {exc}")
            else:
                if not isinstance(machines, list) or len(machines) == 0:
                    passed = False
                    violations.append("no machines found in model file")
                for m in machines:
                    mname = f"{m.get('module', '')}.{m.get('name', '')}".strip(".")
                    if m.get("capped", True) is not False:
                        passed = False
                        violations.append(
                            f"machine `{mname}` state space was CAPPED (capped={m.get('capped')}, max_states={m.get('max_states')})"
                        )
                    for c in m.get("contracts", []):
                        if not c.get("passed", False):
                            passed = False
                            violations.append(f"machine `{mname}` contract `{c.get('name')}` failed")
                    for rf in m.get("refinements", []):
                        if not rf.get("passed", False):
                            passed = False
                            violations.append(f"machine `{mname}` refinement `{rf.get('concrete')}` failed")
        checks.append(
            {
                "file": str(mf),
                "machines": len(machines),
                "violations": violations,
                "passed": len(violations) == 0,
            }
        )
    return {"gate": "model_inspect", "passed": passed, "checks": checks}


def gate_rc_leak_check(run_bins: list[Path]) -> dict[str, Any]:
    if not run_bins:
        return {"gate": "rc_leak_check", "passed": True, "skipped": True, "reason": "no binaries provided"}
    passed = True
    checks = []
    env = dict(os.environ)
    env["CINCH_RC_DEBUG"] = "1"
    env["CINCH_LEAK_CHECK"] = "1"
    for b in run_bins:
        proc = run_cmd([str(b)], env=env)
        combined = (proc.stdout + "\n" + proc.stderr).lower()
        leak_detected = ("leak" in combined and "0 leak" not in combined and "leaks: 0" not in combined and "no leak" not in combined)
        ok = proc.returncode == 0 and not leak_detected
        if not ok:
            passed = False
        checks.append(
            {
                "binary": str(b),
                "returncode": proc.returncode,
                "leak_detected": leak_detected,
                "stdout": proc.stdout.strip(),
                "stderr": proc.stderr.strip(),
                "passed": ok,
            }
        )
    return {"gate": "rc_leak_check", "passed": passed, "checks": checks}


def gate_kernel_abi_and_cache_line(
    cinch_bin: str,
    cinchifs: list[Path],
    abi_spec: Path | None,
    clang_target: str | None,
) -> dict[str, Any]:
    if not cinchifs:
        return {"gate": "kernel_abi_and_cache_line", "passed": True, "skipped": True}
    passed = True
    checks = []
    abi_cmd = [sys.executable, str(HERE / "verify_kernel_abi.py"), "--json"]
    if abi_spec is not None:
        abi_cmd.extend(["--spec", str(abi_spec)])
    if clang_target is not None:
        abi_cmd.extend(["--target", clang_target])
    abi_cmd.extend(str(c) for c in cinchifs)
    abi_proc = run_cmd(abi_cmd)
    if abi_proc.returncode != 0:
        passed = False
    checks.append({"kind": "verify_kernel_abi", "returncode": abi_proc.returncode, "passed": abi_proc.returncode == 0, "stderr": abi_proc.stderr.strip()})

    for cf in cinchifs:
        cl_proc = run_cmd([cinch_bin, "verify-prims", f"--audit-cache-line={cf}"])
        if cl_proc.returncode != 0:
            passed = False
        checks.append({"kind": "audit_cache_line", "file": str(cf), "returncode": cl_proc.returncode, "passed": cl_proc.returncode == 0, "stdout": cl_proc.stdout.strip(), "stderr": cl_proc.stderr.strip()})

    return {"gate": "kernel_abi_and_cache_line", "passed": passed, "checks": checks}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run unified Cinch formal verification CI gates.")
    parser.add_argument("--cinch", default="cinch", help="Path to cinch compiler binary")
    parser.add_argument("--entry", action="append", default=[], type=Path, help="Entry .cinch program(s) to build with --prim-verify=strict")
    parser.add_argument("--verify-prims", action="append", default=[], type=Path, help="Source .cinch file(s) to check via `cinch verify-prims --policy=jpl --audit-unsafe`")
    parser.add_argument("--out-dir", type=Path, default=None, help="Output directory for `cinch build`")
    parser.add_argument("--manifest", action="append", default=[], type=Path, help="Build manifest(s) to check with check_tcb_allowlist.py")
    parser.add_argument("--allowlist", type=Path, default=DEFAULT_ALLOWLIST, help="Path to tcb_allowlist.json")
    parser.add_argument("--strict-jpl", action="store_true", help="Enforce kernel-proof/kernel-cert only in TCB allowlist check")
    parser.add_argument("--model", action="append", default=[], type=Path, help="Model .cinch file(s) to inspect with `cinch model inspect --json` (asserts capped == false)")
    parser.add_argument("--run-bin", action="append", default=[], type=Path, help="Compiled binary/binaries to run under CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1")
    parser.add_argument("--cinchif", action="append", default=[], type=Path, help=".cinchif file(s) to audit for kernel ABI & 128B cache-line isolation")
    parser.add_argument("--abi-spec", type=Path, default=None, help="Optional JSON spec for verify_kernel_abi.py")
    parser.add_argument("--abi-target", default=None, help="Optional clang --target triple for verify_kernel_abi.py")
    parser.add_argument("--json", action="store_true", help="Emit JSON summary")
    args = parser.parse_args(argv)

    gates: list[dict[str, Any]] = []
    manifests = list(args.manifest)

    if args.entry or args.verify_prims:
        g1, discovered = gate_prim_verify_strict(args.cinch, args.entry, args.verify_prims, args.out_dir)
        gates.append(g1)
        for m in discovered:
            if m not in manifests:
                manifests.append(m)
    else:
        gates.append({"gate": "prim_verify_strict", "passed": True, "skipped": True})

    gates.append(gate_tcb_allowlist(manifests, args.allowlist, strict_jpl=args.strict_jpl))
    gates.append(gate_model_inspect(args.cinch, args.model))
    gates.append(gate_rc_leak_check(args.run_bin))
    if args.cinchif:
        gates.append(gate_kernel_abi_and_cache_line(args.cinch, args.cinchif, args.abi_spec, args.abi_target))

    overall = all(g["passed"] for g in gates)
    if args.json:
        print(json.dumps({"passed": overall, "gates": gates}, indent=2))
    else:
        for g in gates:
            if g.get("skipped"):
                print(f"[SKIP] {g['gate']}")
            elif g["passed"]:
                print(f"[PASS] {g['gate']}")
            else:
                print(f"[FAIL] {g['gate']}")
                print(json.dumps(g, indent=2))
    return 0 if overall else 1


if __name__ == "__main__":
    raise SystemExit(main())
