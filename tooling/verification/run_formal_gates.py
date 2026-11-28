#!/usr/bin/env python3
"""Unified CI formal verification gate runner for Cinch.

Runs:
  1. Strict primitive verification (`--prim-verify=strict` / `verify-prims --policy=jpl --audit-unsafe`)
  2. TCB allowlist audit (`check_tcb_allowlist.py`) over `build/<Entry>.manifest.json`
  3. Exhaustive temporal model checking (`cinch model inspect --model-fail-on-capped --json`,
     asserting `capped == false`, zero failed contracts, and zero failed refinements)
  4. Runtime RC & leak verification (`CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1`)
  5. Optional kernel ABI verification (`verify_kernel_abi.py`) & cache-line isolation audit
     (`cinch verify-prims --audit-cache-line=...`) when `.cinchif` files are supplied.
  6. `--wave-sweep`: discovers all `seed/prelude/{Engine,Net,Store,Stream,Consensus}/**/Spec.cinch`
     formal models (M1-M12), verifies `capped == false` and 0 violations on every machine,
     audits 128-byte cache-line isolation on `Engine/Ring/Spsc.cinch`, runs
     `verify-prims --policy=jpl` on `Prim/SimdExt.cinch`, and runs `--audit-unsafe` on
     `Prim/Atomic.cinch` and `Prim/Sys/Io/{Linux,Darwin}.cinch`.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parents[1]
DEFAULT_ALLOWLIST = HERE / "tcb_allowlist.json"

WAVE_SPEC_ROOTS = ("Engine", "Net", "Store", "Stream", "Consensus")


def run_cmd(cmd: list[str], env: dict[str, str] | None = None, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, capture_output=True, text=True, env=env, cwd=cwd)


def resolve_cinch_bin(requested: str | None, repo_root: Path) -> str:
    if requested and requested != "cinch":
        return requested
    if requested == "cinch" and shutil.which("cinch"):
        return "cinch"
    for cand in (
        repo_root / "seed" / "target" / "release" / "cinch",
        repo_root / "build" / "bootstrap" / "stage3" / "cinch",
        repo_root / "seed" / "target" / "debug" / "cinch",
    ):
        if cand.is_file():
            return str(cand)
    return requested or "cinch"


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
    env = dict(os.environ)
    env["CINCH_MODEL_FAIL_ON_CAPPED"] = "1"
    for mf in model_files:
        cmd = [cinch_bin, "model", "inspect", str(mf), "--model-fail-on-capped", "--json"]
        proc = run_cmd(cmd, env=env)
        violations: list[str] = []
        machines: list[dict[str, Any]] = []
        machine_summaries: list[dict[str, Any]] = []
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
                    m_violations: list[str] = []
                    if m.get("capped", True) is not False:
                        passed = False
                        m_violations.append(
                            f"machine `{mname}` state space was CAPPED (capped={m.get('capped')}, max_states={m.get('max_states')})"
                        )
                    contracts = m.get("contracts", [])
                    contracts_passed = 0
                    for c in contracts:
                        if c.get("passed", False):
                            contracts_passed += 1
                        else:
                            passed = False
                            m_violations.append(f"machine `{mname}` contract `{c.get('name')}` failed")
                    refinements = m.get("refinements", [])
                    refinements_passed = 0
                    for rf in refinements:
                        if rf.get("passed", False):
                            refinements_passed += 1
                        else:
                            passed = False
                            m_violations.append(f"machine `{mname}` refinement `{rf.get('concrete')}` failed")
                    violations.extend(m_violations)
                    machine_summaries.append(
                        {
                            "module": m.get("module", ""),
                            "name": m.get("name", ""),
                            "state_count": m.get("state_count", 0),
                            "edge_count": m.get("edge_count", 0),
                            "capped": m.get("capped", True),
                            "contracts_passed": contracts_passed,
                            "contracts_total": len(contracts),
                            "refinements_passed": refinements_passed,
                            "refinements_total": len(refinements),
                            "violations": m_violations,
                            "passed": len(m_violations) == 0,
                        }
                    )
        checks.append(
            {
                "file": str(mf),
                "machines": len(machines),
                "machine_reports": machine_summaries,
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
        leak_detected = ("leak" in combined and "0 leak" not in combined and "leaks: 0" not in combined and "no leak" not in combined and "leaked=0" not in combined)
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


# =========================================================================
# Wave 1-7 Formal Verification Sweep (`--wave-sweep`)
# =========================================================================

def discover_wave_spec_files(repo_root: Path) -> list[Path]:
    prelude = repo_root / "seed" / "prelude"
    found: list[Path] = []
    for root_name in WAVE_SPEC_ROOTS:
        root_dir = prelude / root_name
        if root_dir.is_dir():
            found.extend(sorted(root_dir.rglob("Spec.cinch")))
    return sorted(found)


def _decode_cpk_module_doc(cpk_path: Path, target_module: str) -> dict[str, Any] | None:
    """Decode a module's canonical `.cinchif` document from a `CINCHPK4` `.cpk` artifact."""
    data = cpk_path.read_bytes()
    if len(data) < 88 or data[:8] != b"CINCHPK4":
        return None
    total_len, fmt = struct.unpack_from("<II", data, 8)
    if total_len != len(data) or fmt != 4:
        return None
    sections: list[bytes] = []
    for i in range(9):
        start, length = struct.unpack_from("<II", data, 16 + 8 * i)
        sections.append(data[start : start + length])
    sec_strings, sec_modules, _values, _bodies, _edges, _nodes, _ops, sec_docs, _abi = sections

    str_count = struct.unpack_from("<I", sec_strings, 0)[0]
    spans_end = 4 + str_count * 8
    str_data = sec_strings[spans_end:]

    def get_str(sid: int) -> str:
        s_off, s_len = struct.unpack_from("<II", sec_strings, 4 + sid * 8)
        return str_data[s_off : s_off + s_len].decode("utf-8")

    def decode_val(buf: bytes, pos: int) -> tuple[Any, int]:
        (tag,) = struct.unpack_from("<I", buf, pos)
        pos += 4
        if tag == 0:
            return None, pos
        if tag == 1:
            return False, pos
        if tag == 2:
            return True, pos
        if tag == 3:
            lo, hi = struct.unpack_from("<II", buf, pos)
            return lo | (hi << 32), pos + 8
        if tag == 4:
            (sid,) = struct.unpack_from("<I", buf, pos)
            return get_str(sid), pos + 4
        if tag == 5:
            (cnt,) = struct.unpack_from("<I", buf, pos)
            pos += 4
            arr = []
            for _ in range(cnt):
                v, pos = decode_val(buf, pos)
                arr.append(v)
            return arr, pos
        if tag == 6:
            (cnt,) = struct.unpack_from("<I", buf, pos)
            pos += 4
            obj: dict[str, Any] = {}
            for _ in range(cnt):
                (k_id,) = struct.unpack_from("<I", buf, pos)
                pos += 4
                v, pos = decode_val(buf, pos)
                obj[get_str(k_id)] = v
            return obj, pos
        if tag == 9:
            (root_id,) = struct.unpack_from("<I", buf, pos)
            return {"packed_body_root": root_id}, pos + 4
        raise ValueError(f"unsupported CINCHPK4 document tag: {tag}")

    mod_count = len(sec_modules) // 36
    for idx in range(mod_count):
        name_at, name_len, _ha, _hl, _a, _b, doc_at, doc_len, _rank = struct.unpack_from(
            "<9I", sec_modules, idx * 36
        )
        mod_name = str_data[name_at : name_at + name_len].decode("utf-8")
        if mod_name == target_module:
            doc, _ = decode_val(sec_docs[doc_at : doc_at + doc_len], 0)
            return doc if isinstance(doc, dict) else None
    return None


def gate_wave_cache_line_audit(cinch_bin: str, repo_root: Path) -> dict[str, Any]:
    spsc_src = repo_root / "seed" / "prelude" / "Engine" / "Ring" / "Spsc.cinch"
    if not spsc_src.is_file():
        return {
            "gate": "wave_cache_line_audit",
            "passed": False,
            "error": f"missing {spsc_src}",
            "checks": [],
        }

    with tempfile.TemporaryDirectory(prefix="cinch_cache_line_sweep_") as td:
        tdir = Path(td)
        main_file = tdir / "Main.cinch"
        main_file.write_text(
            "module Main where\n"
            "  import Engine.Ring.Spsc\n"
            "  main :: Int\n"
            "  main = 0\n",
            encoding="utf-8",
        )
        chk = run_cmd([cinch_bin, "check", str(main_file)], cwd=tdir)
        cpk_path = tdir / "build" / "Main.cpk"
        if chk.returncode != 0 or not cpk_path.is_file():
            return {
                "gate": "wave_cache_line_audit",
                "passed": False,
                "source": str(spsc_src),
                "returncode": chk.returncode,
                "error": "failed to check Engine.Ring.Spsc and produce build/Main.cpk",
                "stdout": chk.stdout.strip(),
                "stderr": chk.stderr.strip(),
                "checks": [],
            }

        doc = _decode_cpk_module_doc(cpk_path, "Engine.Ring.Spsc")
        if doc is None:
            return {
                "gate": "wave_cache_line_audit",
                "passed": False,
                "source": str(spsc_src),
                "error": "Engine.Ring.Spsc interface missing from build/Main.cpk",
                "checks": [],
            }

        cinchif = tdir / "Engine.Ring.Spsc.cinchif"
        cinchif.write_text(json.dumps(doc), encoding="utf-8")

        audit = run_cmd([cinch_bin, "verify-prims", f"--audit-cache-line={cinchif}"])
        structs = 0
        cursor_pairs = 0
        violations = 0
        m = re.search(r"structs=(\d+)\s+cursor_pairs=(\d+)\s+violations=(\d+)", audit.stdout)
        if m:
            structs = int(m.group(1))
            cursor_pairs = int(m.group(2))
            violations = int(m.group(3))
        ok = audit.returncode == 0 and structs > 0 and violations == 0
        return {
            "gate": "wave_cache_line_audit",
            "passed": ok,
            "checks": [
                {
                    "source": str(spsc_src),
                    "structs_checked": structs,
                    "cursor_pairs_checked": cursor_pairs,
                    "violations": violations,
                    "returncode": audit.returncode,
                    "stdout": audit.stdout.strip(),
                    "stderr": audit.stderr.strip(),
                    "passed": ok,
                }
            ],
        }


def gate_wave_prim_verify_jpl(cinch_bin: str, repo_root: Path) -> dict[str, Any]:
    simd_ext = repo_root / "seed" / "prelude" / "Prim" / "SimdExt.cinch"
    if not simd_ext.is_file():
        return {
            "gate": "wave_prim_verify_jpl",
            "passed": False,
            "error": f"missing {simd_ext}",
            "checks": [],
        }
    proc = run_cmd([cinch_bin, "verify-prims", "--policy=jpl", "--json", str(simd_ext)])
    payload: dict[str, Any] = {}
    if proc.stdout.strip():
        try:
            payload = json.loads(proc.stdout)
        except json.JSONDecodeError:
            payload = {"raw": proc.stdout.strip()}

    verified = int(payload.get("verified", 0)) if isinstance(payload, dict) else 0
    obligations = int(payload.get("obligations_total", 0)) if isinstance(payload, dict) else 0
    disproved = int(payload.get("disproved", 0)) if isinstance(payload, dict) else 0
    rejected = int(payload.get("rejected_by_policy", 0)) if isinstance(payload, dict) else 0
    unverified = int(payload.get("unverified", 0)) if isinstance(payload, dict) else 0
    ok = (
        proc.returncode == 0
        and obligations > 0
        and verified == obligations
        and disproved == 0
        and rejected == 0
        and unverified == 0
    )
    return {
        "gate": "wave_prim_verify_jpl",
        "passed": ok,
        "checks": [
            {
                "file": str(simd_ext),
                "policy": "jpl",
                "obligations_total": obligations,
                "verified": verified,
                "disproved": disproved,
                "rejected_by_policy": rejected,
                "unverified": unverified,
                "returncode": proc.returncode,
                "stderr": proc.stderr.strip(),
                "passed": ok,
            }
        ],
    }


def gate_wave_audit_unsafe(cinch_bin: str, repo_root: Path) -> dict[str, Any]:
    targets = [
        repo_root / "seed" / "prelude" / "Prim" / "Atomic.cinch",
        repo_root / "seed" / "prelude" / "Prim" / "Sys" / "Io" / "Linux.cinch",
        repo_root / "seed" / "prelude" / "Prim" / "Sys" / "Io" / "Darwin.cinch",
    ]
    checks: list[dict[str, Any]] = []
    passed = True

    for t in targets:
        if not t.is_file():
            passed = False
            checks.append({"file": str(t), "passed": False, "error": "file not found"})
            continue
        proc = run_cmd([cinch_bin, "verify-prims", "--audit-unsafe", str(t)])
        stats = {
            "unsafe": 0,
            "atomics": 0,
            "mem": 0,
            "asm": 0,
            "syscalls": 0,
            "violations": 0,
        }
        m = re.search(
            r"boundary-audit\s+\S+:\s+unsafe=(\d+)\s+atomics=(\d+)\s+mem=(\d+)\s+asm=(\d+)\s+syscalls=(\d+)\s+violations=(\d+)",
            proc.stdout,
        )
        if m:
            stats = {
                "unsafe": int(m.group(1)),
                "atomics": int(m.group(2)),
                "mem": int(m.group(3)),
                "asm": int(m.group(4)),
                "syscalls": int(m.group(5)),
                "violations": int(m.group(6)),
            }
        ok = proc.returncode == 0 and m is not None and stats["violations"] == 0
        if not ok:
            passed = False
        checks.append(
            {
                "file": str(t),
                **stats,
                "returncode": proc.returncode,
                "stderr": proc.stderr.strip(),
                "passed": ok,
            }
        )

    return {"gate": "wave_audit_unsafe", "passed": passed, "checks": checks}


def print_wave_sweep_table(summary: dict[str, Any]) -> None:
    print("Cinch Wave 1-7 Formal Verification Sweep")
    print("=" * 106)
    print(f"{'Gate / Target':<44} {'Scope / Metric':<38} {'Violations':>10} {'Status':>10}")
    print("-" * 106)
    for g in summary["gates"]:
        gname = g["gate"]
        if gname == "model_inspect" and not g.get("skipped"):
            for chk in g.get("checks", []):
                for m in chk.get("machine_reports", []):
                    label = f"model: {m['module']}.{m['name']}"
                    metric = (
                        f"states={m['state_count']} edges={m['edge_count']} "
                        f"c={m['contracts_passed']}/{m['contracts_total']} "
                        f"r={m['refinements_passed']}/{m['refinements_total']} "
                        f"cap={m['capped']}"
                    )
                    st = "PASS" if m["passed"] else "FAIL"
                    print(f"{label:<44} {metric:<38} {len(m['violations']):>10d} {st:>10}")
                if not chk.get("machine_reports"):
                    rel = Path(chk["file"]).name
                    st = "PASS" if chk["passed"] else "FAIL"
                    print(f"{'model: ' + rel:<44} {'(error)':<38} {len(chk['violations']):>10d} {st:>10}")
        elif gname == "wave_cache_line_audit":
            for chk in g.get("checks", []):
                label = "cache-line: Engine/Ring/Spsc.cinch"
                metric = f"structs={chk['structs_checked']} pairs={chk['cursor_pairs_checked']} (128B)"
                st = "PASS" if chk["passed"] else "FAIL"
                print(f"{label:<44} {metric:<38} {chk['violations']:>10d} {st:>10}")
        elif gname == "wave_prim_verify_jpl":
            for chk in g.get("checks", []):
                label = "prim-jpl: Prim/SimdExt.cinch"
                metric = f"verified={chk['verified']}/{chk['obligations_total']} (LRAT KernelCert)"
                viols = chk["disproved"] + chk["rejected_by_policy"] + chk["unverified"]
                st = "PASS" if chk["passed"] else "FAIL"
                print(f"{label:<44} {metric:<38} {viols:>10d} {st:>10}")
        elif gname == "wave_audit_unsafe":
            for chk in g.get("checks", []):
                p = Path(chk["file"])
                rel = str(p).split("seed/prelude/")[-1]
                label = f"unsafe-audit: {rel}"
                metric = f"unsafe={chk.get('unsafe', 0)} atom={chk.get('atomics', 0)} mem={chk.get('mem', 0)} sys={chk.get('syscalls', 0)}"
                st = "PASS" if chk["passed"] else "FAIL"
                print(f"{label:<44} {metric:<38} {chk.get('violations', 1):>10d} {st:>10}")
    print("=" * 106)
    print(f"Overall Wave Sweep: {'PASS' if summary['passed'] else 'FAIL'}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run unified Cinch formal verification CI gates.")
    parser.add_argument("--cinch", default="cinch", help="Path to cinch compiler binary")
    parser.add_argument("--repo-root", type=Path, default=REPO_ROOT, help="Repository root (default: auto-detected)")
    parser.add_argument("--wave-sweep", action="store_true", help="Sweep all Wave 1-7 Spec.cinch models, cache-line audit, JPL SIMD proofs, and unsafe boundary audits")
    parser.add_argument("--entry", action="append", default=[], type=Path, help="Entry .cinch program(s) to build with --prim-verify=strict")
    parser.add_argument("--verify-prims", action="append", default=[], type=Path, help="Source .cinch file(s) to check via `cinch verify-prims --policy=jpl --audit-unsafe`")
    parser.add_argument("--out-dir", type=Path, default=None, help="Output directory for `cinch build`")
    parser.add_argument("--manifest", action="append", default=[], type=Path, help="Build manifest(s) to check with check_tcb_allowlist.py")
    parser.add_argument("--allowlist", type=Path, default=DEFAULT_ALLOWLIST, help="Path to tcb_allowlist.json")
    parser.add_argument("--strict-jpl", action="store_true", help="Enforce kernel-proof/kernel-cert only in TCB allowlist check")
    parser.add_argument("--model", action="append", default=[], type=Path, help="Model .cinch file(s) to inspect with `cinch model inspect --model-fail-on-capped --json`")
    parser.add_argument("--run-bin", action="append", default=[], type=Path, help="Compiled binary/binaries to run under CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1")
    parser.add_argument("--cinchif", action="append", default=[], type=Path, help=".cinchif file(s) to audit for kernel ABI & 128B cache-line isolation")
    parser.add_argument("--abi-spec", type=Path, default=None, help="Optional JSON spec for verify_kernel_abi.py")
    parser.add_argument("--abi-target", default=None, help="Optional clang --target triple for verify_kernel_abi.py")
    parser.add_argument("--json", action="store_true", help="Emit pure JSON summary to stdout")
    parser.add_argument("--json-out", type=Path, default=None, help="Optionally write JSON summary to a file")
    args = parser.parse_args(argv)

    repo_root = args.repo_root.resolve()
    cinch_bin = resolve_cinch_bin(args.cinch, repo_root)

    gates: list[dict[str, Any]] = []
    manifests = list(args.manifest)
    models = list(args.model)

    if args.wave_sweep:
        discovered_specs = discover_wave_spec_files(repo_root)
        for sp in discovered_specs:
            if sp not in models:
                models.append(sp)

    if args.entry or args.verify_prims:
        g1, discovered = gate_prim_verify_strict(cinch_bin, args.entry, args.verify_prims, args.out_dir)
        gates.append(g1)
        for m in discovered:
            if m not in manifests:
                manifests.append(m)
    elif not args.wave_sweep:
        gates.append({"gate": "prim_verify_strict", "passed": True, "skipped": True})

    if manifests or not args.wave_sweep:
        gates.append(gate_tcb_allowlist(manifests, args.allowlist, strict_jpl=args.strict_jpl))

    gates.append(gate_model_inspect(cinch_bin, models))

    if args.wave_sweep:
        gates.append(gate_wave_cache_line_audit(cinch_bin, repo_root))
        gates.append(gate_wave_prim_verify_jpl(cinch_bin, repo_root))
        gates.append(gate_wave_audit_unsafe(cinch_bin, repo_root))

    if args.run_bin or not args.wave_sweep:
        gates.append(gate_rc_leak_check(args.run_bin))

    if args.cinchif:
        gates.append(gate_kernel_abi_and_cache_line(cinch_bin, args.cinchif, args.abi_spec, args.abi_target))

    overall = all(g["passed"] for g in gates)
    summary = {
        "passed": overall,
        "cinch": cinch_bin,
        "wave_sweep": args.wave_sweep,
        "gates": gates,
    }

    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")

    if args.json:
        print(json.dumps(summary, indent=2))
    elif args.wave_sweep:
        print_wave_sweep_table(summary)
        print("\n--- JSON Summary ---")
        print(json.dumps(summary, indent=2))
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
