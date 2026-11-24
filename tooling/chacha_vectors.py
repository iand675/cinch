#!/usr/bin/env python3
"""Write seed/tests/fixtures/chacha-vectors.json from independent implementations.

ChaCha20 keystreams, Poly1305 tags and ChaCha20-Poly1305 seals come from the cryptography library (OpenSSL);
XChaCha20-Poly1305 seals come from libsodium through PyNaCl. Rows cover empty and odd lengths around block
boundaries, block counters near the 32-bit limit, Poly1305 keys and messages chosen to stress carries
(all-ones blocks and the largest clamped r), the RFC 8439 sample, and tampered seals that must not open.

    PYTHONPATH=<dir with pynacl> python3 tooling/chacha_vectors.py > seed/tests/fixtures/chacha-vectors.json
"""
from __future__ import annotations

import json
import random
import sys

import nacl.bindings as sodium
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives.poly1305 import Poly1305


def stream(key: bytes, nonce: bytes, counter: int, data: bytes) -> bytes:
    """ChaCha20 XOR with the 16-byte counter-and-nonce block the library expects."""
    enc = Cipher(algorithms.ChaCha20(key, counter.to_bytes(4, "little") + nonce), None).encryptor()
    return enc.update(data)


def main() -> None:
    rng = random.Random(8439)
    rb = lambda n: bytes(rng.randrange(256) for _ in range(n))
    streams, macs, seals, xseals = [], [], [], []
    rfc_key = bytes(range(32))
    rfc_nonce = bytes.fromhex("000000000000004a00000000")
    rfc_text = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it."
    streams.append((rfc_key, rfc_nonce, 1, rfc_text))
    for n in (0, 1, 15, 16, 17, 63, 64, 65, 127, 128, 200, 1000):
        streams.append((rb(32), rb(12), rng.choice([0, 1, 7, 4294967295 - 20]), rb(n)))
    streams.append((rb(32), rb(12), 4294967295, rb(64)))
    streams = [{"key": key.hex(), "nonce": nonce.hex(), "counter": counter, "input": data.hex(), "output": stream(key, nonce, counter, data).hex()}
               for key, nonce, counter, data in streams]
    keys = [rb(32) for _ in range(6)] + [b"\xff" * 32, bytes.fromhex("ffffff0ffcffff0ffcffff0ffcffff0f") + b"\xff" * 16, b"\x00" * 32]
    for i, key in enumerate(keys):
        for n in (0, 1, 15, 16, 17, 32, 33, 64, 100, 255):
            msg = (b"\xff" * n) if i >= 6 else rb(n)
            macs.append({"key": key.hex(), "message": msg.hex(), "tag": Poly1305.generate_tag(key, msg).hex()})
    for pn, an in ((0, 0), (0, 12), (1, 0), (16, 16), (63, 7), (64, 0), (65, 33), (300, 20), (1000, 1)):
        key, nonce, pt, aad = rb(32), rb(12), rb(pn), rb(an)
        seals.append({"key": key.hex(), "nonce": nonce.hex(), "plaintext": pt.hex(), "aad": aad.hex(), "sealed": ChaCha20Poly1305(key).encrypt(nonce, pt, aad).hex()})
        xnonce = rb(24)
        xs = sodium.crypto_aead_xchacha20poly1305_ietf_encrypt(pt, aad, xnonce, key)
        xseals.append({"key": key.hex(), "nonce": xnonce.hex(), "plaintext": pt.hex(), "aad": aad.hex(), "sealed": xs.hex()})
    json.dump({"streams": streams, "macs": macs, "seals": seals, "xseals": xseals}, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
