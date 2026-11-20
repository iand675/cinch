#!/usr/bin/env python3
"""Prepare deterministic input recipes and independently check complete reference chunks."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import subprocess
import os

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'seed/tests/fixtures/byte-split-vectors.json'


def recipes() -> list[dict]:
    """Exercise empty, overlap, bounded UTF8, alignment, terminator and count boundaries."""
    inputs = [b'', b'a', b',', b',,', b'a,b,', b',a,,b,', b'abababa', b'aaaaa',
              b'a\x00b\x00', b'\r', b'\n', b'\r\n', b'a\nb\r\nc\rd\n', b'\xff\xc0\xaf',
              'aé€🙂z'.encode(), bytes(range(32)), bytes(range(128, 256))]
    inputs += [bytes([n]) for n in range(256)]
    inputs += [(b'ab,' * 20)[:n] for n in [7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 63, 64]]
    rows = []
    for i, raw in enumerate(inputs):
        separators = [b'', b',', b'a', b'aa', b'ab', b'\x00', b'\xff', '🙂'.encode()] if i < 17 or i >= 273 else [b'', raw, b'\xff']
        spans = [(0, len(raw))]
        if i < 17 and raw:
            spans += sorted({(a, b) for a in range(min(5, len(raw)) + 1) for b in [a, len(raw)]})
        for start, end in spans:
            for sep in (separators if (start, end) == (0, len(raw)) else [b'', b',']):
                for after in [False, True]:
                    limits = [-1] if 17 <= i < 273 else [-1, 0, 1, 2, 3, 9] if (start, end) == (0, len(raw)) else [-1, 1, 2]
                    for limit in limits:
                        rows.append(dict(input=raw.hex(), separator=sep.hex(), start=start, end=end, limit=limit, after=after, lines=False))
            rows.append(dict(input=raw.hex(), separator='', start=start, end=end, limit=-1, after=True, lines=True))
    # Every boundary of a valid and malformed multibyte sequence is an independent input.
    for raw in ['🙂é€'.encode(), b'\xf0\x9f\x99\x82\xed\xa0\x80\xe2\x82\xac\xc0\xaf']:
        for start in range(len(raw) + 1):
            for end in range(start, len(raw) + 1):
                for limit in [-1, 1, 2]:
                    rows.append(dict(input=raw.hex(), separator='', start=start, end=end, limit=limit, after=False, lines=False))
    for raw in [b'', b'a,b,', '🙂é'.encode()]:
        for sep in [b'', b',']:
            for after in [False, True]:
                for limit in [-7, 9223372036854775807]:
                    rows.append(dict(input=raw.hex(), separator=sep.hex(), start=0, end=len(raw), limit=limit, after=after, lines=False))
    return rows


def split(raw: bytes, separator: bytes, limit: int, after: bool, lines: bool) -> list[bytes]:
    """Use Python byte operations and its independent malformed-byte decoder."""
    if lines:
        parts = raw.split(b'\n')
        return [part + b'\n' for part in parts[:-1]] + ([parts[-1]] if parts[-1] else [])
    if limit == 0:
        return []
    if not separator:
        chunks = [ch.encode('utf-8', 'surrogateescape') for ch in raw.decode('utf-8', 'surrogateescape')]
        return chunks if limit < 0 or len(chunks) <= limit else chunks[:limit-1] + [b''.join(chunks[limit-1:])]
    chunks = raw.split(separator, -1 if limit < 0 else limit - 1)
    return [part + separator for part in chunks[:-1]] + chunks[-1:] if after else chunks


def crosscheck(rows: list[dict]) -> None:
    """Verify every complete chunk and ordered fingerprint without Cinch code."""
    for i, row in enumerate(rows):
        raw = bytes.fromhex(row['input'])[row['start']:row['end']]
        parts = split(raw, bytes.fromhex(row['separator']), row['limit'], row['after'], row['lines'])
        assert [part.hex() for part in parts] == row['chunks'], (i, row, parts)
        current = 14695981039346656037
        for part in parts:
            for value in [len(part), *part]:
                for byte in value.to_bytes(4, 'little'):
                    current = ((current ^ byte) * 1099511628211) & ((1 << 64) - 1)
        assert (current, len(parts)) == (row['hash'], row['count']), i


def main() -> None:
    """Require a pinned SDK invocation, then compare all reference outputs independently."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--go', type=Path, required=True)
    parser.add_argument('--output', type=Path, default=FIXTURE)
    args = parser.parse_args()
    result = subprocess.run([str(args.go), 'run', str(ROOT / 'tooling/go_byte_split_vectors.go')],
                            input=json.dumps(recipes()), text=True, capture_output=True, check=True,
                            env={**os.environ, 'GOCACHE':'/tmp/cinch-go-surface-cache','GOTELEMETRY':'off','GOMAXPROCS':'1'})
    fixture = json.loads(result.stdout)
    crosscheck(fixture['rows'])
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(fixture, indent=2) + '\n')
    print(f"Complete independent SDK/Python agreement: {len(fixture['rows'])} rows")


if __name__ == '__main__':
    main()
