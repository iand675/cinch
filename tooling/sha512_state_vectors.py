#!/usr/bin/env python3
"""Write independent SHA-384/SHA-512/SHA-512/t digest, checkpoint and load vectors.

Digests come from Python's hashlib. Checkpoints come from a separate pure-Python
compression model whose initial values are checked against hashlib, so the
fixture does not depend on the Cinch implementation under test.

    python3 tooling/sha512_state_vectors.py > seed/tests/fixtures/sha512-state-vectors.json
"""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import generate_sha512_kernels as model  # noqa: E402

VARIANTS = {4: ("sha384", "384", 48), 5: ("sha512_224", "512/224", 28), 6: ("sha512_256", "512/256", 32), 7: ("sha512", "512", 64)}


def data(size: int, fill: bool) -> bytes:
    """Mirror the consumer's deterministic corpus: all-ones or a byte permutation."""
    return bytes(255 if fill else (i * 37 + 11) % 256 for i in range(size))


def checkpoint(variant: int, message: bytes) -> str:
    """Serialize the state after consuming message: magic, words, zero-padded tail, byte count."""
    state = model.initial_values()[VARIANTS[variant][1]]
    full = len(message) - len(message) % 128
    for offset in range(0, full, 128):
        state = model.compress(state, message[offset:offset + 128])
    tail = message[full:]
    out = b"sha" + bytes([variant]) + b"".join(w.to_bytes(8, "big") for w in state)
    out += tail + bytes(128 - len(tail)) + len(message).to_bytes(8, "big")
    assert len(out) == 204
    return out.hex()


def digest(variant: int, message: bytes) -> str:
    """Return the hashlib digest of one variant."""
    return hashlib.new(VARIANTS[variant][0], message).hexdigest()


def main() -> None:
    """Emit valid, split and malformed-load rows."""
    for variant in VARIANTS:
        assert digest(variant, b"abc")[:16] == bytes(
            b for w in model.digest(model.initial_values()[VARIANTS[variant][1]], b"abc") for b in w.to_bytes(8, "big")
        ).hex()[:16]
    valid = []
    sizes = [0, 1, 55, 111, 112, 113, 127, 128, 129, 239, 240, 255, 256, 257, 1000]
    for variant in VARIANTS:
        for size in sizes + ([65536] if variant in (4, 7) else []):
            for fill in ([False, True] if size in (112, 240, 1000) else [False]):
                message = data(size, fill)
                mid = size // 2
                points = sorted({0, 1, mid, size} | ({127, 128, 129} if size > 129 else set()))
                valid.append({"variant": variant, "size": size, "fill": fill, "digest": digest(variant, message),
                              "saved": checkpoint(variant, message), "mid": mid, "checkpoint": checkpoint(variant, message[:mid]),
                              "splits": [p for p in points if p <= size]})
    loads = []
    for variant in VARIANTS:
        good = bytes.fromhex(checkpoint(variant, data(300, False)))
        loads.append({"variant": variant, "input": good.hex(), "kind": ""})
        for other in VARIANTS:
            if other != variant:
                loads.append({"variant": variant, "input": (good[:3] + bytes([other]) + good[4:]).hex(), "kind": "identifier"})
        for cut in (0, 3, 4, 100, 203):
            loads.append({"variant": variant, "input": good[:cut].hex(), "kind": "identifier" if cut < 4 else "size"})
        loads.append({"variant": variant, "input": (good + b"\x00").hex(), "kind": "size"})
        loads.append({"variant": variant, "input": (b"shb" + good[3:]).hex(), "kind": "identifier"})
    json.dump({"valid": valid, "loads": loads}, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
