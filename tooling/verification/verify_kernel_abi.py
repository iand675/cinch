#!/usr/bin/env python3
"""Verify `.cinchif` `abi.layouts` for `#[repr(C)]` kernel structs against `clang`.

Compares struct `size`, `align`, and per-field `offset`/`size` emitted in Cinch
interface files (`.cinchif`) against `clang` `sizeof`/`_Alignof`/`offsetof`
evaluated using host OS or cross-target system headers.

Supports both native execution probing and cross-compilation IR constant
extraction (`--cross` / `--target <triple>`) so Linux kernel ABI structs can be
audited against cross-headers without executing target binaries.
"""

from __future__ import annotations

import argparse
import json
import platform
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

# Built-in kernel struct descriptors mapping normalized Cinch type names -> C header & struct info.
BUILTIN_KERNEL_STRUCTS: dict[str, dict[str, Any]] = {
    "timespec": {
        "headers": ["<time.h>"],
        "c_type": "struct timespec",
        "fields": {"tv_sec": "tv_sec", "tv_nsec": "tv_nsec"},
    },
    "iovec": {
        "headers": ["<sys/uio.h>"],
        "c_type": "struct iovec",
        "fields": {"iov_base": "iov_base", "iov_len": "iov_len"},
    },
    "sockaddr_in": {
        "headers": ["<netinet/in.h>"],
        "c_type": "struct sockaddr_in",
        "fields": {
            "sin_family": "sin_family",
            "sin_port": "sin_port",
            "sin_addr": "sin_addr",
        },
    },
    "sockaddr_in6": {
        "headers": ["<netinet/in.h>"],
        "c_type": "struct sockaddr_in6",
        "fields": {
            "sin6_family": "sin6_family",
            "sin6_port": "sin6_port",
            "sin6_flowinfo": "sin6_flowinfo",
            "sin6_addr": "sin6_addr",
            "sin6_scope_id": "sin6_scope_id",
        },
    },
    "msghdr": {
        "headers": ["<sys/socket.h>"],
        "c_type": "struct msghdr",
        "fields": {
            "msg_name": "msg_name",
            "msg_namelen": "msg_namelen",
            "msg_iov": "msg_iov",
            "msg_iovlen": "msg_iovlen",
            "msg_control": "msg_control",
            "msg_controllen": "msg_controllen",
            "msg_flags": "msg_flags",
        },
    },
    "epoll_event": {
        "headers": ["<sys/epoll.h>"],
        "c_type": "struct epoll_event",
        "fields": {"events": "events", "data": "data"},
    },
    "io_sqring_offsets": {
        "headers": ["<linux/io_uring.h>"],
        "c_type": "struct io_sqring_offsets",
        "fields": {
            "head": "head",
            "tail": "tail",
            "ring_mask": "ring_mask",
            "ring_entries": "ring_entries",
            "flags": "flags",
            "dropped": "dropped",
            "array": "array",
        },
    },
    "io_cqring_offsets": {
        "headers": ["<linux/io_uring.h>"],
        "c_type": "struct io_cqring_offsets",
        "fields": {
            "head": "head",
            "tail": "tail",
            "ring_mask": "ring_mask",
            "ring_entries": "ring_entries",
            "overflow": "overflow",
            "cqes": "cqes",
            "flags": "flags",
        },
    },
    "io_uring_params": {
        "headers": ["<linux/io_uring.h>"],
        "c_type": "struct io_uring_params",
        "fields": {
            "sq_entries": "sq_entries",
            "cq_entries": "cq_entries",
            "flags": "flags",
            "sq_thread_cpu": "sq_thread_cpu",
            "sq_thread_idle": "sq_thread_idle",
            "features": "features",
            "wq_fd": "wq_fd",
            "sq_off": "sq_off",
            "cq_off": "cq_off",
        },
    },
    "io_uring_sqe": {
        "headers": ["<linux/io_uring.h>"],
        "c_type": "struct io_uring_sqe",
        "fields": {
            "opcode": "opcode",
            "flags": "flags",
            "ioprio": "ioprio",
            "fd": "fd",
            "off": "off",
            "addr": "addr",
            "len": "len",
            "user_data": "user_data",
        },
    },
    "io_uring_cqe": {
        "headers": ["<linux/io_uring.h>"],
        "c_type": "struct io_uring_cqe",
        "fields": {"user_data": "user_data", "res": "res", "flags": "flags"},
    },
}


def normalize_name(name: str) -> str:
    # Strip module prefix (`Mod.TypeName` -> `TypeName`) and convert CamelCase/kebab-case -> snake_case.
    base = name.split(".")[-1].replace("-", "_")
    snake = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", base).lower()
    for prefix in ("c_", "raw_", "linux_", "darwin_", "kernel_"):
        if snake.startswith(prefix):
            snake = snake[len(prefix) :]
            break
    return snake


def probe_c_layout_ir(
    clang_bin: str,
    spec: dict[str, Any],
    field_map: dict[str, str],
    target: str | None = None,
    sysroot: str | None = None,
    include_dirs: list[str] | None = None,
) -> dict[str, Any]:
    """Compile a C probe to LLVM IR (`-S -emit-llvm -O0`) and extract exact sizeof/alignof/offsetof."""
    lines = ["#include <stddef.h>", "#include <stdint.h>"]
    for hdr in spec.get("headers", []):
        if hdr.startswith("<") or hdr.startswith('"'):
            lines.append(f"#include {hdr}")
        else:
            lines.append(f'#include "{hdr}"')
    if spec.get("preamble"):
        lines.append(spec["preamble"])

    c_type = spec["c_type"]
    lines.append(f"const unsigned long long __cinch_abi_size = (unsigned long long)sizeof({c_type});")
    lines.append(f"const unsigned long long __cinch_abi_align = (unsigned long long)_Alignof({c_type});")
    sym_to_field: dict[str, str] = {}
    for idx, (cinch_field, c_field) in enumerate(field_map.items()):
        off_sym = f"__cinch_abi_off_{idx}"
        sz_sym = f"__cinch_abi_fsz_{idx}"
        sym_to_field[off_sym] = cinch_field
        sym_to_field[sz_sym] = cinch_field
        lines.append(f"const unsigned long long {off_sym} = (unsigned long long)offsetof({c_type}, {c_field});")
        lines.append(f"const unsigned long long {sz_sym} = (unsigned long long)sizeof((({c_type} *)0)->{c_field});")

    c_src = "\n".join(lines) + "\n"
    with tempfile.TemporaryDirectory(prefix="cinch_abi_") as tmpdir:
        src_path = Path(tmpdir) / "probe.c"
        ll_path = Path(tmpdir) / "probe.ll"
        src_path.write_text(c_src, encoding="utf-8")

        cmd = [clang_bin, "-x", "c", "-std=c11", "-S", "-emit-llvm", "-O0", "-o", str(ll_path), str(src_path)]
        if target:
            cmd.extend(["--target=" + target])
        if sysroot:
            cmd.extend(["--sysroot=" + sysroot])
        for inc in include_dirs or []:
            cmd.extend(["-I", inc])

        proc = subprocess.run(cmd, capture_output=True, text=True)
        if proc.returncode != 0:
            raise RuntimeError(f"clang ABI probe failed for `{c_type}`:\n{proc.stderr.strip()}")

        ir_text = ll_path.read_text(encoding="utf-8")

    def extract_const(sym: str) -> int:
        # Matches `@__cinch_abi_size = ... constant i64 16` or `global i64 16`
        m = re.search(rf"@{re.escape(sym)}\b[^=\n]*=\s*[^\n]*\bi\d+\s+(\d+|zeroinitializer)\b", ir_text)
        if not m:
            raise RuntimeError(f"could not extract constant @{sym} from clang LLVM IR")
        tok = m.group(1)
        return 0 if tok == "zeroinitializer" else int(tok)

    result: dict[str, Any] = {
        "c_type": c_type,
        "size": extract_const("__cinch_abi_size"),
        "align": extract_const("__cinch_abi_align"),
        "fields": {},
    }
    for idx, cinch_field in enumerate(field_map.keys()):
        result["fields"][cinch_field] = {
            "offset": extract_const(f"__cinch_abi_off_{idx}"),
            "size": extract_const(f"__cinch_abi_fsz_{idx}"),
        }
    return result


def resolve_spec(layout_name: str, custom_specs: dict[str, dict[str, Any]]) -> dict[str, Any] | None:
    if layout_name in custom_specs:
        return custom_specs[layout_name]
    norm = normalize_name(layout_name)
    if norm in custom_specs:
        return custom_specs[norm]
    return BUILTIN_KERNEL_STRUCTS.get(norm)


def verify_cinchif(
    cinchif_path: Path,
    clang_bin: str,
    custom_specs: dict[str, dict[str, Any]],
    target: str | None = None,
    sysroot: str | None = None,
    include_dirs: list[str] | None = None,
    require_all_repr_c: bool = False,
) -> dict[str, Any]:
    doc = json.loads(cinchif_path.read_text(encoding="utf-8"))
    layouts = doc.get("abi", {}).get("layouts", [])
    if not isinstance(layouts, list):
        raise ValueError(f"{cinchif_path}: missing abi.layouts array")

    checked = []
    skipped = []
    violations = []

    for layout in layouts:
        name = str(layout.get("type", ""))
        repr_str = str(layout.get("repr", "")).strip()
        if not repr_str.lower().startswith("c"):
            continue
        spec = resolve_spec(name, custom_specs)
        if spec is None:
            if require_all_repr_c:
                violations.append(
                    {
                        "type": name,
                        "reason": "no C struct ABI spec found for #[repr(C)] layout",
                    }
                )
            else:
                skipped.append(name)
            continue

        cinch_fields = {f["name"]: f for f in layout.get("fields", [])}
        spec_fields = spec.get("fields")
        if spec_fields:
            field_map = {}
            for cf_name, f_info in cinch_fields.items():
                norm_f = cf_name.replace("-", "_")
                if cf_name in spec_fields:
                    field_map[cf_name] = spec_fields[cf_name]
                elif norm_f in spec_fields:
                    field_map[cf_name] = spec_fields[norm_f]
        else:
            field_map = {
                cf_name: cf_name.replace("-", "_")
                for cf_name in cinch_fields
                if not cf_name.startswith("_pad") and not cf_name.startswith("pad_")
            }

        c_layout = probe_c_layout_ir(
            clang_bin, spec, field_map, target=target, sysroot=sysroot, include_dirs=include_dirs
        )
        struct_errors = []
        if int(layout["size"]) != c_layout["size"]:
            struct_errors.append(
                f"sizeof mismatch: cinch={layout['size']} vs clang({c_layout['c_type']})={c_layout['size']}"
            )
        if int(layout["align"]) < c_layout["align"] or (
            not spec.get("allow_overalign", False) and int(layout["align"]) != c_layout["align"]
        ):
            struct_errors.append(
                f"alignof mismatch: cinch={layout['align']} vs clang({c_layout['c_type']})={c_layout['align']}"
            )
        for cf_name, expected in c_layout["fields"].items():
            actual = cinch_fields[cf_name]
            if int(actual["offset"]) != expected["offset"]:
                struct_errors.append(
                    f"offsetof({cf_name}) mismatch: cinch={actual['offset']} vs clang={expected['offset']}"
                )
            if int(actual["size"]) != expected["size"]:
                struct_errors.append(
                    f"sizeof({cf_name}) mismatch: cinch={actual['size']} vs clang={expected['size']}"
                )

        checked.append(
            {
                "type": name,
                "c_type": c_layout["c_type"],
                "size": c_layout["size"],
                "align": c_layout["align"],
                "fields_checked": len(field_map),
                "passed": len(struct_errors) == 0,
            }
        )
        for err in struct_errors:
            violations.append({"type": name, "c_type": c_layout["c_type"], "reason": err})

    return {
        "cinchif": str(cinchif_path),
        "host": f"{platform.system()}-{platform.machine()}",
        "target": target or "host",
        "checked": checked,
        "skipped": skipped,
        "violations": violations,
        "passed": len(violations) == 0,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Compare `.cinchif` `abi.layouts` for `#[repr(C)]` kernel structs against `clang`."
    )
    parser.add_argument("cinchifs", nargs="+", type=Path, help="Path(s) to `.cinchif` interface file(s)")
    parser.add_argument("--clang", default=shutil.which("clang") or "clang", help="Path to clang binary")
    parser.add_argument("--target", default=None, help="Optional clang --target triple for cross-header verification")
    parser.add_argument("--sysroot", default=None, help="Optional clang --sysroot directory")
    parser.add_argument("-I", "--include", action="append", default=[], help="Extra include directory for clang")
    parser.add_argument("--spec", type=Path, default=None, help="Optional JSON file mapping Cinch type names to C struct specs")
    parser.add_argument("--require-all-repr-c", action="store_true", help="Fail if any #[repr(C)] struct lacks a C spec")
    parser.add_argument("--json", action="store_true", help="Emit JSON output")
    args = parser.parse_args(argv)

    custom_specs: dict[str, dict[str, Any]] = {}
    if args.spec is not None:
        custom_specs = json.loads(args.spec.read_text(encoding="utf-8"))

    reports = []
    all_passed = True
    for cp in args.cinchifs:
        try:
            rep = verify_cinchif(
                cp,
                clang_bin=args.clang,
                custom_specs=custom_specs,
                target=args.target,
                sysroot=args.sysroot,
                include_dirs=args.include,
                require_all_repr_c=args.require_all_repr_c,
            )
        except Exception as exc:
            print(f"verify_kernel_abi: error checking {cp}: {exc}", file=sys.stderr)
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
                f"[{status}] {rep['cinchif']} (target={rep['target']}): "
                f"checked={len(rep['checked'])} skipped={len(rep['skipped'])} violations={len(rep['violations'])}"
            )
            for v in rep["violations"]:
                print(f"  [ABI-MISMATCH] {v['type']}: {v['reason']}")

    return 0 if all_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
