#!/usr/bin/env python3
"""Compute complete public field-pipeline observations with an independent strict byte/scalar model."""
from __future__ import annotations
import argparse
import hashlib
import json

from generate_byte_fields_vectors import decode, SPACE

MASK = (1 << 64) - 1


def input_byte(pattern: str, at: int) -> int:
    """Construct raw inputs without calling the implementation under measurement."""
    if pattern == 'space':
        return 32
    if pattern == 'text':
        return 122
    if pattern == 'dense':
        return 32 if at % 2 else 122
    if pattern == 'fringe':
        return 32 if at % 33 == 32 else 122
    return [97, 194, 160, 98, 227, 128, 128, 99, 32, 255, 128, 122][at % 12]


def operation(kind: str, raw: bytes) -> int:
    """Visit every exact nonempty field extent and count every scalar for the effectful-predicate path."""
    at, start, count, checksum = 0, None, 0, 0
    spans = []
    while at < len(raw):
        scalar, width = decode(raw, at)
        count += 1
        if scalar in SPACE:
            if start is not None:
                spans.append((start, at))
                start = None
        elif start is None:
            start = at
        at += width
    if start is not None:
        spans.append((start, len(raw)))
    for begin, end in spans:
        assert begin < end
        checksum = (checksum * 65599 + (end - begin) * 257 + raw[begin] * 17 + raw[end - 1] * 5 + 1) & MASK
    return (checksum + count * 104729) & MASK if kind == 'effect' else checksum


def expected(kind: str, pattern: str, size: int, iterations: int) -> dict:
    """Keep both sentinels, changed input and complete retained-owner hashes observable after timing."""
    owner = bytearray([255] + [input_byte(pattern, at) for at in range(size)] + [255])
    original = hashlib.sha256(owner).hexdigest()
    total = 0
    for index in range(iterations):
        owner[1] = 98 if index % 2 else 97
        total = (total + operation(kind, bytes(owner[1:size + 1]))) & MASK
    return dict(checksum=total, original=original, retained=hashlib.sha256(owner).hexdigest(), kept_size=size)


def main() -> None:
    """Print only independent observable fields; elapsed time must come from an actual public pipeline."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kind', choices=['eager', 'producer', 'visit', 'predicate', 'effect'], required=True)
    parser.add_argument('--pattern', choices=['space', 'text', 'dense', 'fringe', 'unicode'], required=True)
    parser.add_argument('--size', type=int, required=True)
    parser.add_argument('--iterations', type=int, required=True)
    args = parser.parse_args()
    assert args.size >= 0 and args.iterations >= 0
    print(json.dumps(expected(args.kind, args.pattern, args.size, args.iterations)))


if __name__ == '__main__':
    main()
