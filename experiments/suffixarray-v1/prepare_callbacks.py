#!/usr/bin/env python3
"""Independently compare three concrete search-from policies with recorded raw-byte leftmost matcher results."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def scalar_width(data: bytes, at: int) -> int:
    """Decode one strict UTF8 scalar locally, advancing one byte for any malformed or truncated sequence."""
    first = data[at]
    if first < 0x80:
        return 1
    if 0xC2 <= first <= 0xDF:
        width, low, high = 2, 0x80, 0xBF
    elif 0xE0 <= first <= 0xEF:
        width = 3
        low, high = (0xA0, 0xBF) if first == 0xE0 else ((0x80, 0x9F) if first == 0xED else (0x80, 0xBF))
    elif 0xF0 <= first <= 0xF4:
        width = 4
        low, high = (0x90, 0xBF) if first == 0xF0 else ((0x80, 0x8F) if first == 0xF4 else (0x80, 0xBF))
    else:
        return 1
    if at + width > len(data) or not low <= data[at + 1] <= high:
        return 1
    return width if all(0x80 <= b <= 0xBF for b in data[at + 2:at + width]) else 1


def observations(data: bytes, pattern: str, limit: int) -> list[list[int]]:
    """Enumerate leftmost empty, optional-byte and greedy-byte matches without invoking any regex library."""
    cursor, previous = 0, None
    found = []
    while cursor <= len(data) and (limit < 0 or len(found) < limit):
        end = cursor
        if pattern == 'a?':
            if cursor < len(data) and data[cursor] == 97:
                end += 1
        elif pattern == 'a*':
            while end < len(data) and data[end] == 97:
                end += 1
        elif pattern != '':
            raise ValueError(pattern)
        if end != cursor or previous != cursor:
            found.append([cursor, end])
        previous = end
        cursor = end if end != cursor else cursor + (scalar_width(data, cursor) if cursor < len(data) else 1)
    return found


def main() -> None:
    """Retain exact oracle identity and every bounded-subset distinction without changing fixture expectations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    raw = (HERE / 'vectors.json').read_bytes()
    assert hashlib.sha256(raw).hexdigest() == '9eeff6e14c5ca60315782cd75aa15e1a32cdfb57019f35ef04ab1533e8988de6'
    vectors = json.loads(raw)
    checked, indexed_differences = 0, 0
    for row in vectors['cases']:
        data = bytes.fromhex(row['data'])
        for query in row['regex']:
            if query['pattern'] not in ['', 'a?', 'a*']:
                continue
            actual = observations(data, query['pattern'], query['limit'])
            assert actual == (query['leftmost'] or []), (row['data'], query, actual)
            indexed_differences += actual != (query['indexed'] or [])
            checked += 1
    report = dict(scope='Independent Python semantic preparation only; no candidate source/runtime or built-in regex claim',
                  vectors_sha256=hashlib.sha256(raw).hexdigest(), generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  inputs=len(vectors['cases']), patterns=['', 'a?', 'a*'], observations=checked,
                  normal_leftmost_reference_equal=True, bounded_suffix_reference_differences=indexed_differences,
                  future_candidate_runtime='pending')
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
