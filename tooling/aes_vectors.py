#!/usr/bin/env python3
"""Write independent AES block, keystream-mode, CBC and GCM vectors.

Values come from the Python `cryptography` package (OpenSSL), plus the FIPS-197
appendix known answers; the fixture is fixed so tests need no reference library.

    python3 tooling/aes_vectors.py > seed/tests/fixtures/aes-vectors.json
"""
from __future__ import annotations

import json
import sys
import warnings

from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

warnings.simplefilter("ignore")

FIPS = [
    ("000102030405060708090a0b0c0d0e0f", "00112233445566778899aabbccddeeff", "69c4e0d86a7b0430d8cdb78070b4c55a"),
    ("000102030405060708090a0b0c0d0e0f1011121314151617", "00112233445566778899aabbccddeeff", "dda97ca4864cdfe06eaf70a0ec0d7191"),
    ("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", "00112233445566778899aabbccddeeff", "8ea2b7ca516745bfeafc49904b496089"),
]


def data(size: int, salt: int) -> bytes:
    """Mirror the consumer's deterministic corpus with a per-field salt."""
    return bytes((i * 37 + 11 + salt) % 256 for i in range(size))


def stream(mode, key: bytes, message: bytes) -> bytes:
    """Encrypt with one of the library's streaming modes."""
    encryptor = Cipher(algorithms.AES(key), mode).encryptor()
    return encryptor.update(message) + encryptor.finalize()


def gcm_seal(key: bytes, nonce: bytes, message: bytes, aad: bytes, tag: int) -> bytes:
    """GCM ciphertext followed by the tag truncated to its leading bytes."""
    encryptor = Cipher(algorithms.AES(key), modes.GCM(nonce)).encryptor()
    encryptor.authenticate_additional_data(aad)
    body = encryptor.update(message) + encryptor.finalize()
    return body + encryptor.tag[:tag]


def main() -> None:
    """Emit block, mode and GCM rows."""
    blocks = [{"key": k, "plain": p, "cipher": c} for k, p, c in FIPS]
    for size in (16, 24, 32):
        for salt in range(6):
            key, plain = data(size, salt * 3 + 1), data(16, salt * 5 + 2)
            blocks.append({"key": key.hex(), "plain": plain.hex(), "cipher": stream(modes.ECB(), key, plain).hex()})
    stream_rows = []
    for size in (16, 24, 32):
        key, iv = data(size, 3), data(16, 7)
        for n in (0, 1, 15, 16, 17, 31, 32, 33, 100, 1000):
            message = data(n, 0)
            row = {"key": key.hex(), "iv": iv.hex(), "size": n,
                   "ctr": stream(modes.CTR(iv), key, message).hex(), "ofb": stream(modes.OFB(iv), key, message).hex(),
                   "cfb": stream(modes.CFB(iv), key, message).hex(), "splits": sorted({0, 1, 5, 16, 17, n // 2, n} - {x for x in range(n + 1, 2000)})}
            if n % 16 == 0:
                row["cbc"] = stream(modes.CBC(iv), key, message).hex()
            stream_rows.append(row)
    wrap = {"key": data(16, 1).hex(), "iv": "ff" * 16, "size": 48, "ctr": stream(modes.CTR(b"\xff" * 16), data(16, 1), data(48, 0)).hex()}
    gcm_rows = []
    for size in (16, 24, 32):
        key = data(size, 4)
        for n, a, nonce_size, tag in ((0, 0, 12, 16), (0, 20, 12, 16), (1, 0, 12, 16), (16, 16, 12, 16), (17, 3, 12, 12), (64, 0, 12, 13),
                                      (100, 33, 12, 14), (255, 1, 12, 15), (1000, 64, 12, 16), (40, 8, 9, 16), (40, 8, 8, 16), (40, 8, 60, 16), (33, 0, 16, 16)):
            nonce, message, aad = data(nonce_size, 9), data(n, 0), data(a, 5)
            gcm_rows.append({"key": key.hex(), "nonce": nonce.hex(), "size": n, "aad": aad.hex(), "tag": tag,
                             "sealed": gcm_seal(key, nonce, message, aad, tag).hex()})
    assert AESGCM(bytes(16)).encrypt(bytes(12), b"", b"").hex() == "58e2fccefa7e3061367f1d57a4e7455a"
    assert AESGCM(bytes(16)).encrypt(bytes(12), bytes(16), b"").hex() == "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf"
    json.dump({"blocks": blocks, "streams": stream_rows, "wrap": wrap, "gcm": gcm_rows}, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
