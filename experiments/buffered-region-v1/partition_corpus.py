#!/usr/bin/env python3
"""Partition complete original trace histories without changing their bodies, expected observations or absolute failure indices."""
from pathlib import Path
import argparse
import hashlib
import json
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tooling'))
from generate_buffered_reader_consumer import runner


def digest(data):
    """Bind exact original helper and trace bytes before generating a smaller driver."""
    return hashlib.sha256(data).hexdigest()


def partition(original, suite, width):
    """Keep every original trace block byte-identical and select each exactly once across bounded parts."""
    source = original.decode()
    matches = list(re.finditer(r'^  -- \| [^\n]*\n  tracecase(\d+) ::', source, re.M))
    assert matches and [int(row[1]) for row in matches] == list(range(len(matches)))
    end = re.search(r'^  -- \| Run independent [^\n]*\n  (?:run-group0|main) ::', source, re.M)
    assert end and end.start() > matches[-1].start()
    header = source[:matches[0].start()]
    blocks = [source[row.start():(matches[index + 1].start() if index + 1 < len(matches) else end.start())]
              for index, row in enumerate(matches)]
    assert header + ''.join(blocks) == source[:end.start()]
    result = []
    for start in range(0, len(blocks), width):
        selected = blocks[start:start + width]
        indices = list(range(start, start + len(selected)))
        output = header + ''.join(selected) + runner(['tracecase' + str(index) for index in indices])
        if suite == 'scanner':
            observations = sum(int(value) for block in selected for value in re.findall(r'\(ScanCheck\s+(\d+)\s', block))
        else:
            observations = sum(len(re.findall(r'\(Check\s', block)) for block in selected)
        assert observations > 0
        result.append({'bytes': output.encode(), 'original_histories': indices,
                       'observations': observations,
                       'original_trace_block_sha256': [digest(block.encode()) for block in selected]})
    assert [index for part in result for index in part['original_histories']] == list(range(len(blocks)))
    return result


def main():
    """Emit new bounded driver epochs and complete coverage metadata; perform no compiler/runtime invocation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--suite', choices=('reader', 'writer', 'scanner'), required=True)
    parser.add_argument('--histories-per-part', type=int, default=48)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    assert 1 <= args.histories_per_part <= 48
    assert not args.output.exists(), 'never overwrite a prior partition epoch'
    original = args.source.read_bytes()
    parts = partition(original, args.suite, args.histories_per_part)
    args.output.mkdir(parents=True)
    rows = []
    for number, part in enumerate(parts):
        name = 'part-' + str(number).zfill(2) + '.cinch'
        (args.output / name).write_bytes(part['bytes'])
        rows.append({key: value for key, value in part.items() if key != 'bytes'} |
                    {'source': name, 'sha256': digest(part['bytes'])})
    report = {'scope': __doc__, 'original_source': str(args.source.resolve()),
              'original_sha256': digest(original), 'suite': args.suite,
              'histories': sum(len(part['original_histories']) for part in rows),
              'observations': sum(part['observations'] for part in rows), 'parts': rows,
              'source_runtime_gate': 'pending; every part remains required', 'production_adopted': False}
    (args.output / 'partitions.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'parts': len(rows), 'histories': report['histories'], 'observations': report['observations']}), flush=True)


if __name__ == '__main__':
    main()
