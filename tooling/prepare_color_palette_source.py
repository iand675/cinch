#!/usr/bin/env python3
"""Prepare immutable fixed-palette factories from the exact pinned reference table entries."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re


def table(source: str, name: str, count: int) -> list[tuple[int, int, int, int]]:
    """Require the complete named table, exact published size and opaque byte representations."""
    body = source.split('var '+name+' = []color.Color{', 1)[1].split('\n}', 1)[0]
    entries = [tuple(int(value, 16) for value in match)
               for match in re.findall(r'color\.RGBA\{(0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+), (0x[0-9a-f]+)\}', body)]
    if len(entries) != count or any(values[3] != 255 for values in entries):
        raise RuntimeError('unexpected fixed palette shape: '+name)
    return entries


def color_table(name: str, entries: list[tuple[int, int, int, int]]) -> str:
    """Emit published-order colors directly in bounded groups without an intermediate word list."""
    words = [(r << 16) | (g << 8) | b for r, g, b, unused_alpha in entries]
    chunks = [words[index:index+8] for index in range(0, len(words), 8)]
    lines = ['  -- | Construct exact packed table entries in published order without mutable shared state.',
             '  '+name+' :: Unit -> List Color', '  '+name+' unit =', '    let end = Nil in']
    tail = 'end'
    for index, chunk in reversed(list(enumerate(chunks))):
        expression = tail
        for value in reversed(chunk):
            expression = f'Cons (packed-color {value}) ({expression})'
        lines.append(f'    let block{index} = {expression} in')
        tail = 'block'+str(index)
    lines.append('    '+tail)
    return '\n'.join(lines)


def main() -> None:
    """Write only the isolated candidate and a precise source-table provenance record."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sdk-source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    raw = args.sdk_source.read_bytes()
    plan9, web = table(raw.decode(), 'Plan9', 256), table(raw.decode(), 'WebSafe', 216)
    source = '''-- | Construct immutable fixed color palettes in their exact published order.
module Image.Color.Palette where
  export color-palette-plan9, color-palette-web-safe

  import Image.Color
  import Data.Core
  import Data.List
  import Data.Machine

  -- | Return all256 Plan9 colors as opaque RGBA8 values in palette order.
  color-palette-plan9 :: Unit -> ColorPalette
  color-palette-plan9 unit = color-palette (plan9-colors ())

  -- | Return all216 web-safe colors as opaque RGBA8 values in palette order.
  color-palette-web-safe :: Unit -> ColorPalette
  color-palette-web-safe unit = color-palette (web-safe-colors ())

  -- | Extract bounded byte channels from one packed RGB entry and supply opaque alpha.
  packed-color :: U32 -> Color
  packed-color value = color-rgba8 (wrapping-u32-to-u8 (wrapping-shr-u32 value 16)) (wrapping-u32-to-u8 (wrapping-shr-u32 value 8)) (wrapping-u32-to-u8 value) 255

'''+color_table('plan9-colors', plan9)+'\n\n'+color_table('web-safe-colors', web)+'\n'
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(source)
    report = {'scope': __doc__, 'sdk_source': str(args.sdk_source),
              'sdk_source_sha256': hashlib.sha256(raw).hexdigest(),
              'runner_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'output_sha256': hashlib.sha256(source.encode()).hexdigest(),
              'table_counts': {'Plan9': len(plan9), 'WebSafe': len(web)},
              'total_entries': len(plan9)+len(web),
              'scope_limit': 'Exact static table extraction/generation only; source checking, independent vectors and runtime are separate.'}
    args.report.write_text(json.dumps(report, indent=2)+'\n')
    print(len(plan9)+len(web), 'exact table entries prepared; no source/runtime claim')


if __name__ == '__main__':
    main()
