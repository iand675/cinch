#!/usr/bin/env python3
"""Check Cinch build manifest(s) against the TCB allowlist for `Assumed` prims.

Parses `build/<Entry>.manifest.json` and fails (exit code 1) if any `Assumed`
primitive (either `prim.tier == "assumed"` or any receipt with `tier == "assumed"`)
is not explicitly permitted by `tcb_allowlist.json`.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

DEFAULT_ALLOWLIST = Path(__file__).resolve().parent / "tcb_allowlist.json"


def load_allowlist(path: Path) -> list[dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("schema_version") != 1:
        raise ValueError(f"unsupported tcb_allowlist schema_version: {data.get('schema_version')!r}")
    entries = data.get("allowed_assumed_prims", [])
    if not isinstance(entries, list):
        raise ValueError("allowed_assumed_prims must be a list")
    return entries


def is_allowed(module: str, name: str, allowlist: list[dict[str, Any]]) -> bool:
    for entry in allowlist:
        em = entry.get("module")
        if em is not None and em != module:
            continue
        if "name" in entry:
            if entry["name"] == name:
                return True
        elif "name_prefix" in entry:
            if name.startswith(entry["name_prefix"]):
                return True
    return False


def audit_manifest(
    manifest_path: Path,
    allowlist: list[dict[str, Any]],
    strict_jpl: bool = False,
) -> dict[str, Any]:
    doc = json.loads(manifest_path.read_text(encoding="utf-8"))
    prims = doc.get("prims", [])
    if not isinstance(prims, list):
        raise ValueError(f"{manifest_path}: expected `prims` array")

    total_prims = len(prims)
    assumed_allowed: list[dict[str, str]] = []
    violations: list[dict[str, Any]] = []

    for p in prims:
        mod = str(p.get("module", ""))
        name = str(p.get("name", ""))
        tier = str(p.get("tier", "")).lower()
        receipts = p.get("receipts", []) or []
        receipt_tiers = [str(r.get("tier", "")).lower() for r in receipts if isinstance(r, dict)]
        has_assumed = tier == "assumed" or ("assumed" in receipt_tiers)

        if has_assumed:
            if is_allowed(mod, name, allowlist):
                assumed_allowed.append({"module": mod, "name": name, "tier": tier})
            else:
                violations.append(
                    {
                        "manifest": str(manifest_path),
                        "module": mod,
                        "name": name,
                        "tier": tier,
                        "receipt_tiers": receipt_tiers,
                        "reason": "unallowlisted Assumed prim in build manifest",
                    }
                )
        elif strict_jpl:
            # Under --strict-jpl, non-allowlisted verified prims must not rely on
            # compiled-cert or trusted SMT verdicts without kernel-checked certificates.
            bad_tiers = [t for t in receipt_tiers if t not in ("kernel-proof", "kernel-cert")]
            if bad_tiers:
                violations.append(
                    {
                        "manifest": str(manifest_path),
                        "module": mod,
                        "name": name,
                        "tier": tier,
                        "receipt_tiers": receipt_tiers,
                        "reason": f"non-kernel receipt tier(s) under --strict-jpl: {bad_tiers}",
                    }
                )

    return {
        "manifest": str(manifest_path),
        "entry": doc.get("entry", ""),
        "total_prims": total_prims,
        "assumed_allowed": assumed_allowed,
        "violations": violations,
        "passed": len(violations) == 0,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate Cinch build manifest(s) against the TCB Assumed-prim allowlist."
    )
    parser.add_argument("manifests", nargs="+", type=Path, help="Path(s) to build/<Entry>.manifest.json")
    parser.add_argument(
        "--allowlist",
        type=Path,
        default=DEFAULT_ALLOWLIST,
        help=f"Path to tcb_allowlist.json (default: {DEFAULT_ALLOWLIST})",
    )
    parser.add_argument(
        "--strict-jpl",
        action="store_true",
        help="Also require every non-allowlisted prim receipt to be kernel-proof or kernel-cert",
    )
    parser.add_argument("--json", action="store_true", help="Emit JSON report")
    args = parser.parse_args(argv)

    try:
        allowlist = load_allowlist(args.allowlist)
    except Exception as exc:
        print(f"check_tcb_allowlist: failed loading allowlist {args.allowlist}: {exc}", file=sys.stderr)
        return 2

    reports = []
    all_passed = True
    for mp in args.manifests:
        try:
            rep = audit_manifest(mp, allowlist, strict_jpl=args.strict_jpl)
        except Exception as exc:
            print(f"check_tcb_allowlist: failed reading {mp}: {exc}", file=sys.stderr)
            return 2
        reports.append(rep)
        if not rep["passed"]:
            all_passed = False

    if args.json:
        print(json.dumps({"passed": all_passed, "reports": reports}, indent=2))
    else:
        for rep in reports:
            status = "PASS" if rep["passed"] else "FAIL"
            print(
                f"[{status}] {rep['manifest']}: prims={rep['total_prims']} "
                f"allowed_assumed={len(rep['assumed_allowed'])} violations={len(rep['violations'])}"
            )
            for v in rep["violations"]:
                print(
                    f"  [TCB-VIOLATION] {v['module']}.{v['name']} "
                    f"(tier={v['tier']}, receipts={v['receipt_tiers']}): {v['reason']}"
                )

    return 0 if all_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
