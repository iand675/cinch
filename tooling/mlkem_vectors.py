#!/usr/bin/env python3
"""Write ML-KEM candidate vectors from the definition-level reference (tooling/mlkem_reference.py).

Each row has a parameter set, a 64-byte seed, explicit encapsulation
randomness, the encapsulation key, the ciphertext and shared key, and a
corrupted ciphertext with its implicit-rejection key. tooling/mlkem_check.go
then checks every row against the reference SDK and adds rows the SDK
encapsulated itself:

    python3 tooling/mlkem_vectors.py | go run tooling/mlkem_check.go > seed/tests/fixtures/mlkem-vectors.json
"""
from __future__ import annotations

import json
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import mlkem_reference as ml  # noqa: E402


def main() -> None:
    """Emit candidate rows."""
    rng = random.Random(203)
    rows = []
    for size in (768, 1024):
        for n in range(4):
            seed = bytes([n] * 64) if n == 0 else bytes(rng.randrange(256) for _ in range(64))
            m = bytes(32) if n == 0 else bytes(rng.randrange(256) for _ in range(32))
            ek, dk = ml.keygen(size, seed)
            key, c = ml.encapsulate(size, ek, m)
            bad = bytearray(c)
            bad[rng.randrange(len(c))] ^= 1 << rng.randrange(8)
            rows.append({"size": size, "seed": seed.hex(), "m": m.hex(), "ek": ek.hex(), "c": c.hex(), "key": key.hex(),
                         "bad": bytes(bad).hex(), "rejected": ml.decapsulate(size, dk, bytes(bad)).hex()})
    json.dump(rows, sys.stdout)


if __name__ == "__main__":
    main()
