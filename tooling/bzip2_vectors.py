#!/usr/bin/env python3
"""Write independent bzip2 decoding vectors.

Inputs come from libbzip2 (through Python's bz2 module) at block sizes 1, 5
and 9 over six reproducible corpus shapes: a stepped sequence, a period-three
pattern, a 32-bit linear congruential byte stream, long byte runs (which
exercise the initial run-length coding's count bytes), a four-letter random
alphabet, and a geometric distribution (trailing zeros of the generator
state, which forces Huffman codes longer than ten bits). Each row records the
compressed bytes and the SHA-256 digest and length of the corpus. Multi-block,
multi-stream and malformed inputs are included.

    python3 tooling/bzip2_vectors.py > seed/tests/fixtures/bzip2-vectors.json
"""
from __future__ import annotations

import bz2
import hashlib
import json
import sys


def corpus(kind: int, size: int) -> bytes:
    """Rebuild a corpus; the Cinch consumer implements the same generators."""
    out = bytearray()
    state = 12345
    for i in range(size):
        state = (state * 1103515245 + 12345) & 0xFFFFFFFF
        if kind == 0:
            out.append((i * 37 + 11) % 256)
        elif kind == 1:
            out.append(97 + i % 3)
        elif kind == 2:
            out.append((state >> 16) & 255)
        elif kind == 3:
            out.append((i // 300) % 7 * 40)
        elif kind == 4:
            out.append(65 + ((state >> 16) & 3))
        else:
            out.append(32 if state == 0 else (state & -state).bit_length() - 1)
    return bytes(out)


def main() -> None:
    """Emit decoding rows and malformed inputs."""
    rows = []
    plan = [(k, s, l) for k in range(6) for s in (1, 2, 5, 100, 1000) for l in (9,)]
    plan += [(k, 20000, l) for k in range(6) for l in (1, 9)]
    plan += [(0, 250000, 1), (3, 230000, 1), (5, 120000, 1), (2, 3000, 5)]
    for kind, size, level in plan:
        data = corpus(kind, size)
        packed = bz2.compress(data, level)
        assert bz2.decompress(packed) == data
        rows.append({"kind": kind, "size": size, "packed": packed.hex(), "digest": hashlib.sha256(data).hexdigest()})
    two = bz2.compress(corpus(2, 700), 9) + bz2.compress(corpus(4, 900), 3)
    rows.append({"kind": 6, "size": 1600, "packed": two.hex(), "digest": hashlib.sha256(corpus(2, 700) + corpus(4, 900)).hexdigest()})
    empty = bz2.compress(b"", 9)
    rows.append({"kind": 0, "size": 0, "packed": empty.hex(), "digest": hashlib.sha256(b"").hexdigest()})
    good = bytearray(bz2.compress(corpus(2, 500), 9))
    errors = []
    def err(data: bytes, kind: str, lenient: bool = False) -> None:
        try:
            bz2.decompress(data)
            if not lenient:
                raise SystemExit("libbzip2 accepted a malformed vector")
        except (OSError, ValueError, EOFError):
            pass
        errors.append({"packed": bytes(data).hex(), "error": kind})
    # libbzip2's one-shot interface returns nothing for no input; an empty input holds no stream here.
    err(b"", "truncated", lenient=True)
    err(b"BZx9" + bytes(good[4:]), "header")
    err(b"BZh0" + bytes(good[4:]), "header")
    for cut in (4, 10, 20, 60, len(good) // 2, len(good) - 5):
        err(bytes(good[:cut]), "truncated")
    bad_block_crc = bytearray(good)
    bad_block_crc[10] ^= 1
    err(bytes(bad_block_crc), "checksum")
    bad_stream_crc = bytearray(good)
    bad_stream_crc[-2] ^= 1
    err(bytes(bad_stream_crc), "checksum")
    randomized = bytearray(good)
    randomized[14] |= 0x80
    # libbzip2 still derandomizes; randomized blocks are obsolete and this decoder rejects them.
    err(bytes(randomized), "block112", lenient=True)
    # libbzip2's one-shot interface ignores trailing bytes; this decoder requires every byte to belong to a stream.
    err(bytes(good) + b"junk", "header", lenient=True)
    bad_magic = bytearray(good)
    bad_magic[5] ^= 0x10
    err(bytes(bad_magic), "block32")
    json.dump({"rows": rows, "errors": errors}, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
