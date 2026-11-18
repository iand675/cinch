#!/usr/bin/env python3
"""Translate committed independent color observations into read-only foreign fixture tables."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
from check_compare import digest


def padded(values: list[int], count: int = 4) -> list[int]:
    """Preserve exact bounded values, filling only unused structural slots with zero."""
    if len(values) > count or any(value < 0 or value > 65535 for value in values):
        raise ValueError('invalid bounded fixture observation')
    return values + [0]*(count-len(values))


def numbers(values: list[int]) -> str:
    """Spell a fixed-width initializer without introducing expected arithmetic."""
    return '{' + ','.join(map(str, values)) + '}'


def prepare(vectors: Path, output: Path) -> dict:
    """Keep original SDK expectations literal; no color conversion is reimplemented here."""
    data = json.loads(vectors.read_text())
    kinds = {name: index for index, name in enumerate(data['kinds'])}

    def observation(row: dict | None) -> str:
        """Encode optional original/converted identity, fields and canonical channels."""
        if row is None:
            return '{-1,{0,0,0,0},{0,0,0,0}}'
        return '{' + str(kinds[row['kind']]) + ',' + numbers(padded(row['channels'])) + ',' + numbers(padded(row['rgba'])) + '}'

    def specification(row: dict) -> str:
        """Encode raw palette inputs without deriving their expected converted values."""
        return '{' + str(kinds[row['kind']]) + ',' + numbers(padded(row['channels'])) + '}'

    code = '''/* Read-only independent color fixture tables; no expected conversion arithmetic. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

/** Exact original/converted identity, raw fields and canonical channels. */
typedef struct { int64_t kind; uint16_t fields[4]; uint16_t rgba[4]; } ColorObservation;
/** Exact source identity and raw component fields. */
typedef struct { int64_t kind; uint16_t fields[4]; } ColorSpecification;
/** Independent observations: original value followed by eleven real model results. */
static const ColorObservation observations[][12] = {
'''
    for vector in data['vectors']:
        if len(vector['converted']) != 11:
            raise ValueError('missing builtin model observation')
        code += ' {' + ','.join(observation(row) for row in [vector['value']] + vector['converted']) + '},\n'
    code += '};\n'
    helper_families = [
        [row['input'] + row['ycbcr'] + [0] for row in data['rgb_conversions']],
        [row['input'] + row['cmyk'] for row in data['rgb_conversions']],
        [row['input'] + row['from_ycbcr'] + [0] for row in data['rgb_conversions']],
        [row['input'] + row['rgb'] for row in data['cmyk_conversions']],
    ]
    for index, rows in enumerate(helper_families):
        code += f'/** Independent direct helper family{index} inputs and outputs. */\nstatic const int64_t helper{index}[][7] = {{\n'
        code += ',\n'.join(' '+numbers(row) for row in rows) + '\n};\n'
    palettes = data['palettes']
    code += '/** Original palette sizes, including empty palettes. */\nstatic const int64_t palette_sizes[] = ' + numbers([len(row['colors']) for row in palettes]) + ';\n'
    code += '/** Independent first nearest indices, with empty index zero. */\nstatic const int64_t palette_indices[] = ' + numbers([row['index'] for row in palettes]) + ';\n'
    code += '/** Original query specifications. */\nstatic const ColorSpecification palette_queries[] = {\n' + ',\n'.join(specification(row['query']) for row in palettes) + '\n};\n'
    maximum = max(1, max(len(row['colors']) for row in palettes))
    code += f'/** Original immutable palette element order. */\nstatic const ColorSpecification palette_colors[][{maximum}] = {{\n'
    for row in palettes:
        colors = [specification(color) for color in row['colors']]
        code += '{' + ','.join(colors or ['{0,{0,0,0,0}}']) + '},\n'
    code += '};\n/** Optional independent palette conversions. */\nstatic const ColorObservation palette_results[] = {\n' + ',\n'.join(observation(row['converted']) for row in palettes) + '\n};\n'
    code += '/** Independent standard color constants, including their original model identity. */\nstatic const ColorObservation constants[] = {\n' + ',\n'.join(observation(row) for row in data['constants']) + '\n};\n'
    code += f'''
/** Reject a fixture bug rather than silently returning a plausible expected value. */
static int64_t checked(int64_t index, int64_t size) {{ if(index<0 || index>=size) abort(); return index; }}
/** Observe the exact committed source-color count. */
int64_t color_fixture_total(int32_t unused) {{ (void)unused; return {len(data['vectors'])}; }}
/** Read an original or builtin-converted expected representation identity. */
int64_t color_fixture_kind(int64_t i,int64_t model) {{ return observations[checked(i,{len(data['vectors'])})][checked(model+1,12)].kind; }}
/** Read one original or builtin-converted raw component. */
int64_t color_fixture_field(int64_t i,int64_t model,int64_t lane) {{ return observations[checked(i,{len(data['vectors'])})][checked(model+1,12)].fields[checked(lane,4)]; }}
/** Read one original or builtin-converted canonical component. */
int64_t color_fixture_rgba(int64_t i,int64_t model,int64_t lane) {{ return observations[checked(i,{len(data['vectors'])})][checked(model+1,12)].rgba[checked(lane,4)]; }}
/** Observe a direct helper family length. */
int64_t color_fixture_helper_count(int64_t family) {{ switch(family) {{
'''
    for index, rows in enumerate(helper_families):
        code += f'case {index}: return {len(rows)};\n'
    code += 'default: abort(); } }\n/** Read literal direct helper inputs and outputs. */\nint64_t color_fixture_helpers(int64_t family,int64_t row,int64_t lane) { checked(lane,7); switch(family) {\n'
    for index, rows in enumerate(helper_families):
        code += f'case {index}: return helper{index}[checked(row,{len(rows)})][lane];\n'
    code += f'''default: abort(); }} }}
/** Observe the exact independent palette-case count. */
int64_t color_fixture_palette_count(int32_t unused) {{ (void)unused; return {len(palettes)}; }}
/** Observe one original palette size. */
int64_t color_fixture_palette_size(int64_t i) {{ return palette_sizes[checked(i,{len(palettes)})]; }}
/** Resolve an original query or element with checked structural coordinates. */
static const ColorSpecification *palette_spec(int64_t i,int64_t slot) {{ checked(i,{len(palettes)}); return slot == -1 ? &palette_queries[i] : &palette_colors[i][checked(slot,palette_sizes[i])]; }}
/** Observe a query or element's original representation. */
int64_t color_fixture_palette_kind(int64_t i,int64_t slot) {{ return palette_spec(i,slot)->kind; }}
/** Observe a query or element's exact raw component. */
int64_t color_fixture_palette_field(int64_t i,int64_t slot,int64_t lane) {{ return palette_spec(i,slot)->fields[checked(lane,4)]; }}
/** Observe the independent first nearest index. */
int64_t color_fixture_palette_index(int64_t i) {{ return palette_indices[checked(i,{len(palettes)})]; }}
/** Observe optional output identity or a raw/canonical channel without implementing selection. */
int64_t color_fixture_palette_result(int64_t i,int64_t field) {{
 const ColorObservation *value=&palette_results[checked(i,{len(palettes)})]; checked(field,9);
 if(field==0) return value->kind;
 if(field<5) return value->fields[field-1];
 return value->rgba[field-5];
}}
/** Observe a standard constant without recreating its expected representation. */
int64_t color_fixture_constant(int64_t i,int64_t field) {{
 const ColorObservation *value=&constants[checked(i,4)]; checked(field,9);
 if(field==0) return value->kind;
 if(field<5) return value->fields[field-1];
 return value->rgba[field-5];
}}
/** Preserve the first public-consumer mismatch coordinates without deriving expectations. */
int32_t color_fixture_failure(int64_t family,int64_t row,int64_t model) {{
 fprintf(stderr,"color-fixture: family=%lld row=%lld model=%lld\\n",(long long)family,(long long)row,(long long)model);
 return 0;
}}
'''
    output.write_text(code)
    return {'scope': __doc__, 'vectors_sha256': digest(vectors), 'fixture_sha256': digest(output),
            'source_colors': len(data['vectors']), 'observations_per_source': 12,
            'helper_counts': list(map(len, helper_families)), 'palettes': len(palettes),
            'expected_values_are_literal_sdk_observations': True}


def main() -> None:
    """Prepare an SDK-free table source and explicit provenance without compiling anything."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--vectors', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    args.report.write_text(json.dumps(prepare(args.vectors, args.output), indent=2) + '\n')


if __name__ == '__main__':
    main()
