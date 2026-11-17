#!/usr/bin/env python3
"""Prepare complete buffered trace consumers by explicit signature adaptations, preserving every observation and operation body."""
from pathlib import Path
import argparse
import hashlib
import json
import re


def digest(data):
    """Record exact original and proposed source bytes independently of their eventual compiler epoch."""
    return hashlib.sha256(data).hexdigest()


def needs_mutation(suite, name):
    """Select only reviewed consumer signatures; raw source/sink callback rows remain unchanged."""
    if name == 'main' or name.startswith(('tracecase', 'batchcase', 'run-group')):
        return suite != 'split'
    if suite in ('reader', 'writer'):
        return name in ('action', 'checks')
    if suite == 'scanner':
        return name == 'scan-checks'
    if suite == 'layers':
        return name in ('reader-observations', 'direct-unread', 'rune-history', 'read-into-bounds',
                        'writer-observations', 'reservation-bounds', 'nested-finish', 'nested-failure',
                        'scanner-ownership', 'scanner-callback-isolation', 'paired-components', 'reclaim')
    return False


def prepare(original, suite):
    """Make a reversible, narrowly listed header/factory transform without altering an independent expected value."""
    text = original.decode()
    changed = []
    rows = []
    for line in text.splitlines(keepends=True):
        before = line
        signature = re.match(r'^  ([a-z][\w-]*)\s*::', line)
        if signature:
            name = signature[1]
            line = re.sub(r'forall r\.', 'forall (r :: Region).', line)
            if needs_mutation(suite, name):
                assert '->{IO r}' in line, ('expected exactly reviewed outer row', name)
                assert line.count('->{IO r}') == 1, ('unexpected higher-order signature', name)
                line = line.replace('->{IO r}', '->{IO r, Mut r}')
        if suite == 'layers' and line.startswith('  counted-source current calls = '):
            assert 'buffer-source (\\wanted ->' in line
            line = line.replace('buffer-source (\\wanted ->', 'buffer-source-with-mutation (\\wanted ->', 1)
        if line != before:
            changed.append({'line': len(rows) + 1, 'old': before, 'new': line})
        rows.append(line)
    output = ''.join(rows)
    additions = []
    for name in ('Effect.Buffered', 'Effect.Scanner'):
        declaration = '  import ' + name + '\n'
        if declaration not in output:
            additions.append(declaration)
    if additions:
        marker = 'module Main where\n'
        assert output.count(marker) == 1
        output = output.replace(marker, marker + ''.join(additions), 1)
    reconstructed = output
    if additions:
        reconstructed = reconstructed.replace('module Main where\n' + ''.join(additions), 'module Main where\n', 1)
    restored = reconstructed.splitlines(keepends=True)
    for row in changed:
        assert restored[row['line'] - 1] == row['new']
        restored[row['line'] - 1] = row['old']
    assert ''.join(restored).encode() == original, 'an observation, body or unlisted signature changed'
    if suite != 'split':
        assert any('Mut r' in row['new'] for row in changed)
    return output.encode(), changed, additions


def main():
    """Write a new source epoch and exact reversible change manifest; no source/runtime verdict is implied."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--suite', choices=('reader', 'writer', 'scanner', 'split', 'layers'), required=True)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    original = args.source.read_bytes()
    output, changed, additions = prepare(original, args.suite)
    assert len(output) <= 2 * 1024 * 1024, 'bounded consumer-source size'
    assert not args.output.exists() and not args.output.with_suffix('.preparation.json').exists()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(output)
    report = {'scope': __doc__, 'suite': args.suite, 'source': str(args.source.resolve()),
              'original_sha256': digest(original), 'output_sha256': digest(output),
              'changes': changed, 'added_imports': additions,
              'every_observation_and_operation_body_preserved': True,
              'compiler_source_gate': 'pending', 'runtime_gate': 'pending', 'production_adopted': False,
              'exclusions': 'Core retained-read controls need an individually reviewed combined-effect fixture; no automatic rewrite or narrowed retention.'}
    args.output.with_suffix('.preparation.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Prepared reversible consumer signature changes; no compiler or runtime result claimed', flush=True)


if __name__ == '__main__':
    main()
