#!/usr/bin/env python3
"""Cross-check actual fixed-palette observations with independent scalar generation and distance rules."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path


def plan9() -> list[list[int]]:
    """Derive all256 ordered colors from cube coordinates and independent intensity normalization."""
    result = [None]*256
    for red in range(4):
        for shade in range(4):
            base = (red*4+shade)*16
            for green in range(4):
                for blue in range(4):
                    maximum = max(red, green, blue)
                    channels = [17*shade]*3 if maximum == 0 else [coordinate*17*(4*maximum+shade)//maximum for coordinate in [red, green, blue]]
                    result[base+((shade-red+green*4+blue) % 16)] = channels+[255]
    if any(value is None for value in result):
        raise RuntimeError('incomplete independent Plan9 cube')
    return result


def nearest(colors: list[list[int]], query: list[int]) -> int:
    """Minimize each shifted squared channel distance with the first table index breaking ties."""
    distances = [sum(((channel*257-target)**2)//4 for channel, target in zip(color, query)) for color in colors]
    return min(range(len(colors)), key=distances.__getitem__)


def query_values() -> list[list[int]]:
    """Generate the specified query domain using unbounded scalar arithmetic and explicit reduction."""
    values = [[value*257]*3+[65535] for value in range(256)]
    state = 0x5eeda11c
    alphas = [0, 1, 255, 256, 32768, 65534, 65535]
    for index in range(256):
        channels = []
        for unused_lane in range(3):
            state = (state*1664525+1013904223) % 4294967296
            channels.append(state//65536)
        values.append(channels+[alphas[index % len(alphas)]])
    return values


def main() -> None:
    """Validate every table byte and both conversion paths without reading the Cinch implementation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--vectors', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    raw = args.vectors.read_bytes()
    vectors = json.loads(raw)
    tables = {'Plan9': plan9(), 'WebSafe': [[r*51, g*51, b*51, 255] for r in range(6) for g in range(6) for b in range(6)]}
    queries = query_values()
    checked_entries = checked_queries = 0
    for actual in vectors['palettes']:
        expected = tables[actual['name']]
        if len(actual['entries']) != len(expected) or len(actual['queries']) != 512:
            raise RuntimeError('unexpected actual table/query shape')
        for index, (value, channels) in enumerate(zip(actual['entries'], expected)):
            if value != {'raw': channels, 'rgba': [channel*257 for channel in channels], 'index': index, 'converted': channels, 'modeled': channels}:
                raise RuntimeError('table representation/canonical/exact-choice mismatch: '+actual['name']+'/'+str(index))
            checked_entries += 1
        for value, query in zip(actual['queries'], queries):
            index = nearest(expected, value['query'])
            if value['query'] != query or value['index'] != index or value['converted'] != expected[index] or value['modeled'] != expected[index]:
                raise RuntimeError('query/domain/first-choice/model mismatch: '+actual['name'])
            checked_queries += 1
    report = {'scope': __doc__, 'status': 'passed', 'vectors_sha256': hashlib.sha256(raw).hexdigest(),
              'runner_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'exact_table_entries': checked_entries, 'palette_use_queries': checked_queries,
              'query_operations': ['Index', 'Convert', 'Model.Convert'],
              'limits': 'Independent scalar/reference agreement only; no Cinch source/runtime or elapsed claim.'}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(checked_entries, 'exact entries and', checked_queries, 'scalar palette-use queries agree')


if __name__ == '__main__':
    main()
