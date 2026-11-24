"""Regression gate ensuring stdlib modules stay in pure Cinch without portable inline LLVM."""
from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRELUDE = ROOT / "seed" / "prelude"

# Only genuine OS syscall, atomic STM concurrency, and target-ISA hardware crypto
# boundary modules may contain inline `llvm {` blocks.
ALLOWED_INLINE_LLVM = {
    "Prim/Sys/Linux.cinch",
    "Prim/Sys/Darwin.cinch",
    "Effect/Stm.cinch",
    "Crypto/Aes.cinch",
    "Crypto/Cipher.cinch",
    "Data/String.cinch",
}


class PureCinchStdlibGate(unittest.TestCase):
    """Verify portable stdlib modules rely on compiler codegen rather than inline LLVM."""

    def test_no_portable_inline_llvm_outside_allowlist(self) -> None:
        offenders: dict[str, int] = {}
        for path in sorted(PRELUDE.rglob("*.cinch")):
            rel = path.relative_to(PRELUDE).as_posix()
            count = path.read_text(encoding="utf-8").count("llvm {")
            if count > 0 and rel not in ALLOWED_INLINE_LLVM:
                offenders[rel] = count
        self.assertEqual(offenders, {}, f"Unexpected inline LLVM blocks in portable stdlib modules: {offenders}")

    def test_no_retired_llvm_kernel_generators_remain(self) -> None:
        generators = sorted(p.name for p in (ROOT / "tooling").glob("generate_*_kernels.py"))
        self.assertEqual(generators, [], f"Retired LLVM kernel generators still present: {generators}")


if __name__ == "__main__":
    unittest.main()
