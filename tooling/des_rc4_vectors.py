#!/usr/bin/env python3
"""Write DES, Triple DES and RC4 vectors from the cryptography library.

RC4 keys whose sizes the library refuses use a reference written from the
definition, which is checked against the library at every size it accepts.

Rows cover single-block encryption for random keys and the FIPS 81 / SP 800-17
style known answers, ECB, CBC and CTR over several lengths, and RC4 keystreams
for keys of 1 to 256 bytes, including the RFC 6229 key "0102030405".

    python3 tooling/des_rc4_vectors.py > seed/tests/fixtures/des-rc4-vectors.json
"""
from __future__ import annotations

import json
import random
import sys
import warnings

from cryptography.hazmat.decrepit.ciphers.algorithms import ARC4, TripleDES
from cryptography.hazmat.primitives.ciphers import Cipher, modes


def run(key: bytes, mode, data: bytes, encrypt: bool = True) -> bytes:
    """One cipher operation; an 8-byte key is single DES (three equal keys)."""
    full = key * 3 if len(key) == 8 else key
    c = Cipher(TripleDES(full), mode)
    op = c.encryptor() if encrypt else c.decryptor()
    return op.update(data) + op.finalize()


def ctr(key: bytes, iv: bytes, data: bytes) -> bytes:
    """CTR from ECB encryptions of a counter that increments as a big-endian 64-bit integer."""
    counter, pad = int.from_bytes(iv, "big"), b""
    while len(pad) < len(data):
        pad += run(key, modes.ECB(), counter.to_bytes(8, "big"))
        counter = (counter + 1) % (1 << 64)
    return bytes(a ^ b for a, b in zip(data, pad))


def rc4_reference(key: bytes, data: bytes) -> bytes:
    """RC4 written from its definition, for key lengths the library does not accept."""
    s, j = list(range(256)), 0
    for i in range(256):
        j = (j + s[i] + key[i % len(key)]) % 256
        s[i], s[j] = s[j], s[i]
    out, i, j = bytearray(), 0, 0
    for b in data:
        i = (i + 1) % 256
        j = (j + s[i]) % 256
        s[i], s[j] = s[j], s[i]
        out.append(b ^ s[(s[i] + s[j]) % 256])
    return bytes(out)


def main() -> None:
    """Emit rows."""
    warnings.simplefilter("ignore")
    rng = random.Random(46)
    rows = []
    known = [("0123456789abcdef", "4e6f772069732074", "3fa40e8a984d4815"), ("133457799bbcdff1", "0123456789abcdef", "85e813540f0ab405"),
             ("0000000000000000", "0000000000000000", "8ca64de9c1b123a7"), ("ffffffffffffffff", "ffffffffffffffff", "7359b2163e4edc58")]
    for k, p, c in known:
        assert run(bytes.fromhex(k), modes.ECB(), bytes.fromhex(p)).hex() == c
        rows.append({"mode": "ecb", "key": k, "iv": "", "plain": p, "cipher": c})
    for size in (8, 24):
        for length in (8, 16, 64, 200):
            key = bytes(rng.randrange(256) for _ in range(size))
            iv = bytes(rng.randrange(256) for _ in range(8))
            data = bytes(rng.randrange(256) for _ in range(length))
            rows.append({"mode": "ecb", "key": key.hex(), "iv": "", "plain": data.hex(), "cipher": run(key, modes.ECB(), data).hex()})
            rows.append({"mode": "cbc", "key": key.hex(), "iv": iv.hex(), "plain": data.hex(), "cipher": run(key, modes.CBC(iv), data).hex()})
        for length in (0, 1, 7, 8, 9, 100):
            key = bytes(rng.randrange(256) for _ in range(size))
            iv = bytes([255] * 7 + [254]) if length == 100 else bytes(rng.randrange(256) for _ in range(8))
            data = bytes(rng.randrange(256) for _ in range(length))
            rows.append({"mode": "ctr", "key": key.hex(), "iv": iv.hex(), "plain": data.hex(), "cipher": ctr(key, iv, data).hex()})
    rc4 = []
    for key in [bytes.fromhex("0102030405"), bytes([7]), bytes(range(256))] + [bytes(rng.randrange(256) for _ in range(n)) for n in (3, 16, 40, 129)]:
        data = bytes(rng.randrange(256) for _ in range(300))
        out = rc4_reference(key, data)
        if len(key) * 8 in ARC4.key_sizes:
            enc = Cipher(ARC4(key), mode=None).encryptor()
            assert enc.update(data[:117]) + enc.update(data[117:]) == out
        rc4.append({"key": key.hex(), "plain": data.hex(), "cipher": out.hex(), "split": 117})
    json.dump({"des": rows, "rc4": rc4}, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
