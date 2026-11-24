#!/usr/bin/env python3
"""Write independent SHA-1 and MD5 digest, checkpoint and load vectors.

Digests come from Python's hashlib. Checkpoint bytes come from separate
pure-Python compression models checked against hashlib on every row, so the
fixture does not depend on the Cinch implementation under test.

    python3 tooling/sha1_md5_vectors.py > seed/tests/fixtures/sha1-md5-vectors.json
"""
from __future__ import annotations

import hashlib
import json
import math
import struct
import sys

M32 = (1 << 32) - 1


def rotl(x: int, n: int) -> int:
    """Rotate a 32-bit word left."""
    return ((x << n) | (x >> (32 - n))) & M32


def sha1_compress(h: list[int], block: bytes) -> list[int]:
    """One SHA-1 block."""
    w = list(struct.unpack(">16I", block))
    for t in range(16, 80):
        w.append(rotl(w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16], 1))
    a, b, c, d, e = h
    for t in range(80):
        if t < 20:
            f, k = (b & c) | (~b & d), 0x5A827999
        elif t < 40:
            f, k = b ^ c ^ d, 0x6ED9EBA1
        elif t < 60:
            f, k = (b & c) | (b & d) | (c & d), 0x8F1BBCDC
        else:
            f, k = b ^ c ^ d, 0xCA62C1D6
        a, b, c, d, e = (rotl(a, 5) + (f & M32) + e + k + w[t]) & M32, a, rotl(b, 30), c, d
    return [(x + y) & M32 for x, y in zip(h, [a, b, c, d, e])]


SHIFTS = [7, 12, 17, 22] * 4 + [5, 9, 14, 20] * 4 + [4, 11, 16, 23] * 4 + [6, 10, 15, 21] * 4
K = [int(abs(math.sin(i + 1)) * (1 << 32)) & M32 for i in range(64)]


def md5_compress(h: list[int], block: bytes) -> list[int]:
    """One MD5 block."""
    m = struct.unpack("<16I", block)
    a, b, c, d = h
    for t in range(64):
        if t < 16:
            f, g = (b & c) | (~b & d), t
        elif t < 32:
            f, g = (d & b) | (~d & c), (5 * t + 1) % 16
        elif t < 48:
            f, g = b ^ c ^ d, (3 * t + 5) % 16
        else:
            f, g = c ^ (b | (~d & M32)), (7 * t) % 16
        a, b, c, d = d, (b + rotl((a + (f & M32) + K[t] + m[g]) & M32, SHIFTS[t])) & M32, b, c
    return [(x + y) & M32 for x, y in zip(h, [a, b, c, d])]


ALGORITHMS = {
    "sha1": (sha1_compress, [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0], b"sha\x01", ">", 96),
    "md5": (md5_compress, [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476], b"md5\x01", "<", 92),
}


def finish(name: str, message: bytes) -> bytes:
    """Digest via the independent model, for cross-checking hashlib."""
    compress, h, _, order, _ = ALGORITHMS[name]
    padded = message + b"\x80" + bytes((55 - len(message)) % 64) + struct.pack(order + "Q", len(message) * 8)
    for i in range(0, len(padded), 64):
        h = compress(h, padded[i:i + 64])
    return struct.pack(order + f"{len(h)}I", *h)


def checkpoint(name: str, message: bytes) -> str:
    """Magic, network-order chaining words, zero-padded tail and network-order byte count."""
    compress, h, magic, _, size = ALGORITHMS[name]
    full = len(message) - len(message) % 64
    for i in range(0, full, 64):
        h = compress(h, message[i:i + 64])
    tail = message[full:]
    out = magic + struct.pack(f">{len(h)}I", *h) + tail + bytes(64 - len(tail)) + struct.pack(">Q", len(message))
    assert len(out) == size
    return out.hex()


def data(size: int, fill: bool) -> bytes:
    """Mirror the consumer's deterministic corpus."""
    return bytes(255 if fill else (i * 37 + 11) % 256 for i in range(size))


def main() -> None:
    """Emit valid and malformed-load rows for both algorithms."""
    valid, loads = [], []
    for name in ALGORITHMS:
        for size in [0, 1, 55, 56, 63, 64, 65, 119, 120, 127, 128, 129, 1000, 65536]:
            for fill in ([False, True] if size in (56, 120, 1000) else [False]):
                message = data(size, fill)
                expected = hashlib.new(name, message).digest()
                assert finish(name, message) == expected
                mid = size // 2
                points = sorted({0, 1, mid, size} | ({63, 64, 65} if size > 65 else set()))
                valid.append({"algorithm": name, "size": size, "fill": fill, "digest": expected.hex(), "saved": checkpoint(name, message),
                              "mid": mid, "checkpoint": checkpoint(name, message[:mid]), "splits": [p for p in points if p <= size]})
        good = bytes.fromhex(checkpoint(name, data(100, False)))
        size = len(good)
        loads.append({"algorithm": name, "input": good.hex(), "kind": ""})
        for cut in (0, 3, 4, 50, size - 1):
            loads.append({"algorithm": name, "input": good[:cut].hex(), "kind": "identifier" if cut < 4 else "size"})
        loads.append({"algorithm": name, "input": (good + b"\x00").hex(), "kind": "size"})
        loads.append({"algorithm": name, "input": (good[:3] + b"\x02" + good[4:]).hex(), "kind": "identifier"})
        other = b"md5\x01" if name == "sha1" else b"sha\x01"
        loads.append({"algorithm": name, "input": (other + good[4:]).hex(), "kind": "identifier"})
    json.dump({"valid": valid, "loads": loads}, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
