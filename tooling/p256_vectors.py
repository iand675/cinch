#!/usr/bin/env python3
"""Write P-256 ECDSA and ECDH vectors from the cryptography library.

Rows cover key derivation (uncompressed and compressed encodings),
deterministic RFC 6979 signatures over SHA-256 digests (raw and DER),
verification of randomized signatures over SHA-256, SHA-384, SHA-512 and SHA-1
digests and of tampered ones, ECDH agreement, compressed-point decoding, and
malformed keys and signatures.

    python3 tooling/p256_vectors.py > seed/tests/fixtures/p256-vectors.json
"""
from __future__ import annotations

import hashlib
import json
import random
import sys

from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.asymmetric.utils import Prehashed, decode_dss_signature

N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
P = 2**256 - 2**224 + 2**192 + 2**96 - 1


def point(key, compressed: bool) -> str:
    """A public key's SEC 1 encoding."""
    fmt = serialization.PublicFormat.CompressedPoint if compressed else serialization.PublicFormat.UncompressedPoint
    return key.public_bytes(serialization.Encoding.X962, fmt).hex()


def main() -> None:
    """Emit the fixture."""
    rng = random.Random(186)
    keys, signs, verifies, ecdh = [], [], [], []
    scalars = [1, 2, N - 1, N - 2, 0xC9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721] + [rng.randrange(1, N) for _ in range(6)]
    for d in scalars:
        k = ec.derive_private_key(d, ec.SECP256R1())
        keys.append({"d": f"{d:064x}", "public": point(k.public_key(), False), "compressed": point(k.public_key(), True)})
        for message in (b"", b"sample", b"test", bytes(range(200))):
            digest = hashlib.sha256(message).digest()
            der = k.sign(digest, ec.ECDSA(Prehashed(hashes.SHA256()), deterministic_signing=True))
            r, s = decode_dss_signature(der)
            signs.append({"d": f"{d:064x}", "digest": digest.hex(), "rs": f"{r:064x}{s:064x}", "der": der.hex()})
    for algorithm, fn in ((hashes.SHA256(), hashlib.sha256), (hashes.SHA384(), hashlib.sha384), (hashes.SHA512(), hashlib.sha512), (hashes.SHA1(), hashlib.sha1)):
        for _ in range(3):
            k = ec.generate_private_key(ec.SECP256R1())
            digest = fn(bytes(rng.randrange(256) for _ in range(40))).digest()
            der = k.sign(digest, ec.ECDSA(Prehashed(algorithm)))
            r, s = decode_dss_signature(der)
            pub = point(k.public_key(), False)
            verifies.append({"public": pub, "digest": digest.hex(), "der": der.hex(), "rs": f"{r:064x}{s:064x}", "valid": True})
            bad = bytearray(digest)
            bad[0] ^= 1
            verifies.append({"public": pub, "digest": bytes(bad).hex(), "der": der.hex(), "rs": f"{r:064x}{s:064x}", "valid": False})
            verifies.append({"public": pub, "digest": digest.hex(), "der": der.hex(), "rs": f"{r:064x}{(N - s) % N:064x}", "valid": True})
    for _ in range(6):
        a, b = ec.generate_private_key(ec.SECP256R1()), ec.generate_private_key(ec.SECP256R1())
        shared = a.exchange(ec.ECDH(), b.public_key())
        ecdh.append({"d": f"{a.private_numbers().private_value:064x}", "peer": point(b.public_key(), rng.random() < 0.5), "shared": shared.hex()})
    good = keys[5]["public"]
    non_residue = next(x for x in range(1, 100) if pow((x**3 - 3 * x + 0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B) % P, (P - 1) // 2, P) == P - 1)
    bad_points = ["00", good[:-2] + ("00" if good[-2:] != "00" else "01"), "04" + f"{P:064x}" + good[66:], "05" + good[2:], good[:64],
                  "02" + f"{non_residue:064x}", "03" + f"{P - 1:064x}"]
    for b in bad_points:
        try:
            ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256R1(), bytes.fromhex(b))
            raise SystemExit(f"the library accepts a supposedly invalid point {b[:10]}")
        except ValueError:
            pass
    bad_keys = [f"{0:064x}", f"{N:064x}", f"{N + 1:064x}", "ff" * 32, "01" * 31]
    der = signs[0]["der"]
    bad_ders = [der + "00", "30" + der[2:4] + "0221" + "00" * 33 + der[6:], "30" + f"{int(der[2:4], 16) + 1:02x}" + der[4:], "3006020100020100", "3008020200010202ff01",
                "30" + f"{int(der[2:4], 16):02x}" + der[4:-2]]
    for b in bad_ders:
        assert b != der, "a malformed signature mutation left the signature unchanged"
    json.dump({"keys": keys, "signs": signs, "verifies": verifies, "ecdh": ecdh, "bad_points": bad_points, "bad_keys": bad_keys, "bad_ders": bad_ders}, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
