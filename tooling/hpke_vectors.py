#!/usr/bin/env python3
"""Write seed/tests/fixtures/hpke-vectors.json from an independent HPKE implementation (pyhpke).

Each row fixes a suite (DHKEM(X25519) or DHKEM(P-256) with HKDF-SHA256, HKDF-SHA256/384/512, AES-128-GCM,
AES-256-GCM, ChaCha20-Poly1305 or export-only) and a mode (base, PSK, auth, auth-PSK), derives the recipient,
sender and ephemeral key pairs from input keying material, and records the serialized keys, the
encapsulated key, three sealed messages in sequence and two exported secrets.

    PYTHONPATH=<dir with pyhpke> python3 tooling/hpke_vectors.py > seed/tests/fixtures/hpke-vectors.json
"""
from __future__ import annotations

import json
import random
import sys

from pyhpke import AEADId, CipherSuite, KDFId, KEMId

KEMS = {"x25519": KEMId.DHKEM_X25519_HKDF_SHA256, "p256": KEMId.DHKEM_P256_HKDF_SHA256}
KDFS = {"sha256": KDFId.HKDF_SHA256, "sha384": KDFId.HKDF_SHA384, "sha512": KDFId.HKDF_SHA512}
AEADS = {"aes128gcm": AEADId.AES128_GCM, "aes256gcm": AEADId.AES256_GCM, "chacha20poly1305": AEADId.CHACHA20_POLY1305, "export": AEADId.EXPORT_ONLY}
MODES = ["base", "psk", "auth", "authpsk"]


def main() -> None:
    rng = random.Random(9180)
    rb = lambda n: bytes(rng.randrange(256) for _ in range(n))
    def private(kem: str, key) -> bytes:
        """The RFC 9180 serialization of a private key (P-256 scalars as 32 big-endian bytes)."""
        raw = key.to_private_bytes()
        if kem == "p256":
            value = int.from_bytes(raw, "big")
            assert value < 1 << 256
            return value.to_bytes(32, "big")
        return raw
    rows = []
    for kem in KEMS:
        for kdf in KDFS:
            for aead in AEADS:
                for mode in MODES:
                    if rng.random() < 0.5 and not (kdf == "sha256" and mode == "base"):
                        continue
                    suite = CipherSuite.new(KEMS[kem], KDFS[kdf], AEADS[aead])
                    ikm_r, ikm_s, ikm_e = rb(32), rb(32), rb(32)
                    kr = suite.kem.derive_key_pair(ikm_r)
                    ks = suite.kem.derive_key_pair(ikm_s)
                    ke = suite.kem.derive_key_pair(ikm_e)
                    info = rb(rng.randrange(0, 40))
                    psk, psk_id = (rb(32), rb(rng.randrange(1, 20))) if "psk" in mode else (b"", b"")
                    sks = ks.private_key if "auth" in mode else None
                    enc, sender = suite.create_sender_context(kr.public_key, info, sks=sks, psk=psk, psk_id=psk_id, eks=ke)
                    messages = []
                    if aead != "export":
                        for i in range(3):
                            pt, aad = rb(rng.randrange(0, 70)), rb(rng.randrange(0, 20))
                            messages.append({"aad": aad.hex(), "plaintext": pt.hex(), "sealed": sender.seal(pt, aad).hex()})
                    exports = []
                    for n in (32, 77):
                        ctx = rb(rng.randrange(0, 16))
                        exports.append({"context": ctx.hex(), "length": n, "value": sender.export(ctx, n).hex()})
                    rows.append({
                        "kem": kem, "kdf": kdf, "aead": aead, "mode": mode, "info": info.hex(), "psk": psk.hex(), "psk_id": psk_id.hex(),
                        "ikm_r": ikm_r.hex(), "ikm_s": ikm_s.hex(), "ikm_e": ikm_e.hex(),
                        "sk_r": private(kem, kr.private_key).hex(), "pk_r": kr.public_key.to_public_bytes().hex(),
                        "sk_s": private(kem, ks.private_key).hex(), "pk_s": ks.public_key.to_public_bytes().hex(),
                        "sk_e": private(kem, ke.private_key).hex(), "enc": enc.hex(), "messages": messages, "exports": exports})
    json.dump(rows, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
