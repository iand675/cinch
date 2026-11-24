#!/usr/bin/env python3
"""Write independent SHA-3, SHAKE and cSHAKE digest, output-stream, checkpoint and load vectors.

SHA-3 and SHAKE outputs come from Python's hashlib. cSHAKE outputs and every
checkpoint come from a separate model sponge over the generator's reference
permutation; the model is cross-checked against hashlib on every row and
against the NIST SP 800-185 cSHAKE sample values.

    python3 tooling/sha3_state_vectors.py > seed/tests/fixtures/sha3-state-vectors.json
"""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from generate_sha3_kernels import permute  # noqa: E402

RATE = {1: 144, 2: 136, 3: 104, 4: 72, 5: 168, 6: 136, 7: 168, 8: 136}
SIZE = {1: 28, 2: 32, 3: 48, 4: 64, 5: 32, 6: 64, 7: 32, 8: 64}
DOMAIN = {1: 6, 2: 6, 3: 6, 4: 6, 5: 0x1F, 6: 0x1F, 7: 4, 8: 4}
HASHLIB = {1: "sha3_224", 2: "sha3_256", 3: "sha3_384", 4: "sha3_512", 5: "shake_128", 6: "shake_256"}


def absorb(lanes: list[int], block: bytes) -> list[int]:
    """XOR one rate block of little-endian lanes and permute."""
    words = [int.from_bytes(block[i:i + 8], "little") for i in range(0, len(block), 8)]
    return permute([lane ^ (words[i] if i < len(words) else 0) for i, lane in enumerate(lanes)])


def left_encode(value: int) -> bytes:
    """Minimal big-endian length-prefixed encoding."""
    digits = value.to_bytes(max(1, (value.bit_length() + 7) // 8), "big")
    return bytes([len(digits)]) + digits


def initial(variant: int, name: bytes = b"", custom: bytes = b"") -> list[int]:
    """Lanes after any cSHAKE customization prefix."""
    lanes = [0] * 25
    if variant >= 7:
        rate = RATE[variant]
        prefix = left_encode(rate) + left_encode(len(name) * 8) + name + left_encode(len(custom) * 8) + custom
        prefix += bytes(-len(prefix) % rate)
        for i in range(0, len(prefix), rate):
            lanes = absorb(lanes, prefix[i:i + rate])
    return lanes


def state(variant: int, message: bytes, start: list[int]) -> tuple[list[int], bytes]:
    """Absorb complete blocks; return lanes and the pending tail."""
    rate = RATE[variant]
    full = len(message) - len(message) % rate
    lanes = start
    for i in range(0, full, rate):
        lanes = absorb(lanes, message[i:i + rate])
    return lanes, message[full:]


def output(variant: int, message: bytes, count: int, start: list[int]) -> bytes:
    """Model sponge output."""
    rate = RATE[variant]
    lanes, tail = state(variant, message, start)
    block = bytearray(tail + bytes(rate - len(tail)))
    block[len(tail)] ^= DOMAIN[variant]
    block[rate - 1] ^= 0x80
    lanes = absorb(lanes, bytes(block))
    out = b""
    while len(out) < count:
        out += b"".join(w.to_bytes(8, "little") for w in lanes)[:rate]
        lanes = permute(lanes)
    return out[:count]


def checkpoint(variant: int, message: bytes, start: list[int]) -> str:
    """Identifier, initial lanes, current lanes, pending length and zero-padded pending bytes."""
    lanes, tail = state(variant, message, start)
    pack = lambda ls: b"".join(w.to_bytes(8, "little") for w in ls)
    out = b"kec" + bytes([variant]) + pack(start) + pack(lanes) + bytes([len(tail)]) + tail + bytes(168 - len(tail))
    assert len(out) == 573
    return out.hex()


def data(size: int) -> bytes:
    """Mirror the consumer's deterministic corpus."""
    return bytes((i * 37 + 11) % 256 for i in range(size))


def main() -> None:
    """Emit valid, customized, stream and malformed-load rows."""
    sample = bytes(range(4))
    assert output(7, sample, 32, initial(7, b"", b"Email Signature")).hex().startswith("c1c36925b6409a04")
    assert output(7, bytes(range(200)), 32, initial(7, b"", b"Email Signature")).hex().startswith("c5221d50e4f822d9")
    assert output(8, sample, 64, initial(8, b"", b"Email Signature")).hex().startswith("d008828e2b80ac9d")
    valid = []
    for variant in range(1, 7):
        for size in sorted({0, 1, 71, 72, 73, 103, 104, 135, 136, 137, 143, 144, 167, 168, 169, 400, 65536}):
            message = data(size)
            expected = output(variant, message, SIZE[variant], [0] * 25)
            h = hashlib.new(HASHLIB[variant], message)
            assert expected == (h.digest(SIZE[variant]) if variant >= 5 else h.digest())
            mid = size // 2
            valid.append({"variant": variant, "name": "", "custom": "", "size": size, "digest": expected.hex(),
                          "saved": checkpoint(variant, message, [0] * 25), "mid": mid, "checkpoint": checkpoint(variant, message[:mid], [0] * 25),
                          "splits": sorted(p for p in {0, 1, mid, size} | ({RATE[variant] - 1, RATE[variant]} if size > RATE[variant] else set()) if p <= size)})
    for variant in (7, 8):
        for name, custom in [(b"", b"Email Signature"), (b"KMAC", b""), (b"N" * 200, b"S" * 300), (b"", b"")]:
            for size in (0, 4, 200, 1000):
                message = data(size)
                plain = variant - 2 if not name and not custom else variant
                start = initial(variant, name, custom) if plain == variant else [0] * 25
                expected = output(plain, message, SIZE[variant], start)
                if plain != variant:
                    assert expected == hashlib.new(HASHLIB[plain], message).digest(SIZE[variant])
                mid = size // 2
                valid.append({"variant": variant, "name": name.hex(), "custom": custom.hex(), "size": size, "digest": expected.hex(),
                              "saved": checkpoint(plain, message, start), "mid": mid, "checkpoint": checkpoint(plain, message[:mid], start),
                              "splits": sorted(p for p in {0, 1, mid, size} if p <= size)})
    streams = []
    for variant in (5, 6):
        for size in (0, 500):
            message = data(size)
            total = 1000
            streams.append({"variant": variant, "size": size, "output": hashlib.new(HASHLIB[variant], message).hexdigest(total),
                            "reads": [1, 0, 167, 168, 169, 300, 195]})
    loads = []
    for variant in range(1, 9):
        start = initial(variant, b"", b"x") if variant >= 7 else [0] * 25
        good = bytes.fromhex(checkpoint(variant, data(50), start))
        loads.append({"variant": variant, "input": good.hex(), "kind": ""})
        for cut in (0, 3, 4, 300, 572):
            loads.append({"variant": variant, "input": good[:cut].hex(), "kind": "identifier" if cut < 4 else "size"})
        loads.append({"variant": variant, "input": (good + b"\x00").hex(), "kind": "size"})
        loads.append({"variant": variant, "input": (good[:3] + bytes([variant % 8 + 1]) + good[4:]).hex(), "kind": "identifier"})
        bad = bytearray(good)
        bad[404] = RATE[variant]
        loads.append({"variant": variant, "input": bytes(bad).hex(), "kind": "tail"})
    json.dump({"valid": valid, "streams": streams, "loads": loads}, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
