"""Regression gate ensuring stdlib modules stay in pure Cinch without portable inline LLVM."""
from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRELUDE = ROOT / "seed" / "prelude"

# Only leaf hardware/primitive/syscall modules may declare inline `llvm {`
# `prim` fragments; every algorithmic stdlib module (Crypto/*, Text/*, Image/*,
# Data/String.cinch, Data/Bytes/*, Effect/Http*, Effect/Sql*, etc.) must be
# 100% pure Cinch.
ALLOWED_INLINE_LLVM = {
    "Prim/Sys/Linux.cinch",
    "Prim/Sys/Darwin.cinch",
    "Prim/Machine.cinch",
    "Prim/Wide.cinch",
    "Data/AD.cinch",
    "Data/Machine.cinch",
    "Data/Wide.cinch",
    "Data/Char.cinch",
    "Effect/Stm.cinch",
}

MAX_LEAF_LLVM_LINES = 16


class PureCinchStdlibGate(unittest.TestCase):
    """Verify portable stdlib modules stay in pure Cinch and leaf primitives live in .cinch files."""

    def test_no_portable_inline_llvm_outside_allowlist(self) -> None:
        offenders: dict[str, int] = {}
        for path in sorted(PRELUDE.rglob("*.cinch")):
            rel = path.relative_to(PRELUDE).as_posix()
            count = path.read_text(encoding="utf-8").count("llvm {")
            if count > 0 and rel not in ALLOWED_INLINE_LLVM:
                offenders[rel] = count
        self.assertEqual(offenders, {}, f"Unexpected inline LLVM blocks in algorithmic stdlib modules: {offenders}")

    def test_no_cinch_prim_imports_in_stdlib(self) -> None:
        offenders: dict[str, int] = {}
        for path in sorted(PRELUDE.rglob("*.cinch")):
            rel = path.relative_to(PRELUDE).as_posix()
            count = path.read_text(encoding="utf-8").count('foreign import ccall "cinch.prim.')
            if count > 0:
                offenders[rel] = count
        self.assertEqual(
            offenders,
            {},
            f"Stdlib modules must declare leaf primitives in .cinch rather than importing cinch.prim.*: {offenders}",
        )

    def test_inline_llvm_blocks_are_leaf_primitives(self) -> None:
        oversized: list[str] = []
        for rel in sorted(ALLOWED_INLINE_LLVM):
            path = PRELUDE / rel
            if not path.exists():
                continue
            lines = path.read_text(encoding="utf-8").splitlines()
            in_block = False
            block_start = 0
            block_lines: list[str] = []
            for lineno, line in enumerate(lines, 1):
                stripped = line.strip()
                if not in_block:
                    if stripped.endswith("llvm {"):
                        in_block = True
                        block_start = lineno
                        block_lines = []
                else:
                    if stripped == "}" or stripped.startswith("} "):
                        in_block = False
                        if len(block_lines) > MAX_LEAF_LLVM_LINES:
                            oversized.append(f"{rel}:{block_start} ({len(block_lines)} lines)")
                        for bl in block_lines:
                            if bl.startswith(("br ", "switch ")) or bl.endswith(":"):
                                oversized.append(f"{rel}:{block_start} control-flow `{bl}`")
                    elif stripped and not stripped.startswith(";"):
                        block_lines.append(stripped)
        self.assertEqual(oversized, [], f"Non-leaf inline LLVM blocks detected: {oversized}")

    def test_no_retired_llvm_kernel_generators_remain(self) -> None:
        generators = sorted(p.name for p in (ROOT / "tooling").glob("generate_*_kernels.py"))
        self.assertEqual(generators, [], f"Retired LLVM kernel generators still present: {generators}")


if __name__ == "__main__":
    unittest.main()
