#!/usr/bin/env python3
"""Translate committed independent fixed-palette observations into literal read-only fixture tables."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
from check_compare import digest


def numbers(values: list[int]) -> str:
    """Spell bounded literal expectations without recreating color or nearest-distance arithmetic."""
    if any(value < 0 or value > 65535 for value in values):
        raise ValueError('invalid bounded fixture value')
    return '{'+','.join(map(str, values))+'}'


def prepare(vectors: Path, output: Path) -> dict:
    """Retain actual SDK fields/results with checked coordinates and no runtime expected math."""
    data = json.loads(vectors.read_text())
    sizes = [len(row['entries']) for row in data['palettes']]
    if sizes != [256, 216] or any(len(row['queries']) != 512 for row in data['palettes']):
        raise ValueError('wrong actual table/query shape')
    code = '''/* Literal independent fixed-palette observations; no expected conversion arithmetic. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
/** Exact raw and canonical channels in original table order; unused structural rows stay zero. */
static const int64_t entries[2][256][8] = {
'''
    for palette in data['palettes']:
        code += '{\n'+',\n'.join(numbers(row['raw']+row['rgba']) for row in palette['entries'])+'\n},\n'
    code += '};\n/** Query channels, first index, and exact selected/model raw and canonical observations. */\nstatic const int64_t queries[2][512][21] = {\n'
    for palette in data['palettes']:
        rows = []
        for row in palette['queries']:
            selected = palette['entries'][row['index']]
            if row['converted'] != selected['raw'] or row['modeled'] != selected['raw']:
                raise ValueError('actual SDK selection/model representation disagrees with its table')
            rows.append(numbers(row['query']+[row['index']]+row['converted']+selected['rgba']+row['modeled']+selected['rgba']))
        code += '{\n'+',\n'.join(rows)+'\n},\n'
    code += '''};
/** Reject fixture-coordinate bugs instead of returning a plausible expected color. */
static int64_t checked(int64_t value,int64_t bound) { if(value<0 || value>=bound) abort(); return value; }
/** Observe the complete actual table/query domain through a fixed-width foreign boundary. */
int64_t fixed_palette_count(int64_t table,int64_t group) { checked(table,2); checked(group,2); return group==1 ? 512 : (table==0 ? 256 : 216); }
/** Read one literal raw/canonical entry or query/result field with complete bounds checks. */
int64_t fixed_palette_field(int64_t table,int64_t group,int64_t row,int64_t field) {
 checked(table,2); checked(group,2); checked(row,fixed_palette_count(table,group));
 return group==0 ? entries[table][row][checked(field,8)] : queries[table][row][checked(field,21)];
}
/** Supply a runtime-opaque bounded repetition count for escaped immutable-owner checks. */
int64_t fixed_palette_owner_count(int32_t unused) { (void)unused; return 32; }
/** Preserve the exact first failing table, family and row without deriving expectations. */
int32_t fixed_palette_failure(int64_t table,int64_t group,int64_t row) {
 fprintf(stderr,"fixed-palette-fixture: table=%lld group=%lld row=%lld\\n",(long long)table,(long long)group,(long long)row); return 0;
}
'''
    output.write_text(code)
    return {'scope': __doc__, 'vectors_sha256': digest(vectors), 'fixture_sha256': digest(output),
            'exact_table_entries': 472, 'palette_use_queries': 1024,
            'expected_values_are_literal_sdk_observations': True}


def main() -> None:
    """Prepare an SDK-free fixture source and record its exact immutable input identity."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--vectors', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    args.report.write_text(json.dumps(prepare(args.vectors, args.output), indent=2)+'\n')


if __name__ == '__main__':
    main()
