#!/usr/bin/env python3
"""Write independent X25519 and Ed25519 vectors.

Positive cases come from the Python cryptography package (OpenSSL) and the
RFC 7748 and RFC 8032 test vectors; negative cases are constructed here
(low-order peers, S + L malleability, invalid point encodings, wrong lengths).

    python3 tooling/curve25519_vectors.py > seed/tests/fixtures/curve25519-vectors.json
"""
from __future__ import annotations

import json
import random
import sys

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey, X25519PublicKey

RAW = serialization.Encoding.Raw, serialization.PublicFormat.Raw
L = 2**252 + 27742317777372353535851937790883648493


def exchange(scalar: bytes, point: bytes) -> bytes:
    """X25519 through the library."""
    return X25519PrivateKey.from_private_bytes(scalar).exchange(X25519PublicKey.from_public_bytes(point))


def main() -> None:
    """Emit X25519 exchanges, low-order rejections, the iterated RFC test and Ed25519 sign/verify rows."""
    rng = random.Random(25519)
    rand = lambda n: bytes(rng.getrandbits(8) for _ in range(n))
    rfc = [("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4", "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c"),
           ("4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d", "e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493")]
    exchanges = [(a, u, exchange(bytes.fromhex(a), bytes.fromhex(u)).hex()) for a, u in rfc]
    assert exchanges[0][2] == "c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552"
    for _ in range(12):
        a, b = rand(32), rand(32)
        pub = X25519PrivateKey.from_private_bytes(b).public_key().public_bytes(*RAW)
        exchanges.append((a.hex(), pub.hex(), exchange(a, pub).hex()))
    unreduced = bytes.fromhex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f")
    keys = [(s.hex(), X25519PrivateKey.from_private_bytes(s).public_key().public_bytes(*RAW).hex()) for s in [bytes.fromhex("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a")] + [rand(32) for _ in range(4)]]
    low_order = ["00" * 32, "01" + "00" * 31, "e0eb7a7c3b41b8ae1656e3faf19fc46ada098deb9c32b1fd866205165f49b800", "5f9c95bca3508c24b1d0b1559c83ef5b04445cc4581c8e86d8224eddd09f1157"]
    signatures = []
    messages = [b"", bytes([0x72]), bytes.fromhex("af82"), b"abc", rand(1023), rand(1)] + [rand(rng.choice([0, 7, 64, 255])) for _ in range(10)]
    seeds = ["9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60", "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
             "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7"] + [rand(32).hex() for _ in range(13)]
    for seed, message in zip(seeds, messages):
        key = Ed25519PrivateKey.from_private_bytes(bytes.fromhex(seed))
        signatures.append({"seed": seed, "message": message.hex(), "public": key.public_key().public_bytes(*RAW).hex(), "signature": key.sign(message).hex()})
    first = signatures[3]
    sig = bytes.fromhex(first["signature"])
    s_plus_l = (int.from_bytes(sig[32:], "little") + L).to_bytes(32, "little")
    rejects = [{"public": first["public"], "message": first["message"], "signature": (sig[:32] + s_plus_l).hex()},
               {"public": "02" + "00" * 31, "message": first["message"], "signature": first["signature"]},
               {"public": first["public"], "message": first["message"], "signature": first["signature"][:126]},
               {"public": first["public"][:62], "message": first["message"], "signature": first["signature"]},
               {"public": first["public"], "message": first["message"], "signature": first["signature"][:126] + "ff"}]
    json.dump({"exchanges": [{"scalar": a, "point": u, "shared": o} for a, u, o in exchanges], "keys": [{"secret": s, "public": p} for s, p in keys],
               "unreduced": {"scalar": exchanges[0][0], "point": unreduced.hex(), "shared": exchange(bytes.fromhex(exchanges[0][0]), unreduced).hex()},
               "low_order": low_order, "signatures": signatures, "rejects": rejects}, sys.stdout, indent=1)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
