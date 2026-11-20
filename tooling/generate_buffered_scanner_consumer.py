#!/usr/bin/env python3
"""Render complete independent scanner/direct-split observations with optional explicit built-in capabilities."""
from pathlib import Path
import argparse
import hashlib
import json
from generate_buffered_reader_consumer import literal, runner

ROOT = Path(__file__).resolve().parents[1]
KINDS = {'bytes': 0, 'lines': 1, 'runes': 2, 'words': 3, 'negative': 4, 'overadvance': 5, 'final-nil': 6, 'final-empty': 7, 'empty': 8}
CODES = {'none': 0, 'too-long': 1, 'negative-advance': 2, 'advance-too-far': 3}


def digest(path):
    """Identify the independent input and reused documented fixture helpers separately from generated source."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observed(row):
    """Adapt only the documented terminal-empty panic and retain absent/empty token and UTF-8 validity distinctions."""
    if row['panic']:
        assert bytes.fromhex(row['panic']).decode() == 'bufio.Scan: too many empty tokens without progressing'
        error = 6
    else:
        error = CODES[row['error']]
    token = 'None' if row['nil'] else f'(Some (unhex "{row["data"]}"))'
    try:
        bytes.fromhex(row['data']).decode('utf-8')
        valid = True
    except UnicodeDecodeError:
        valid = False
    return row['more'], token, error, valid


def main():
    """Emit all chosen observations with shallow groups, exact first-failure indices and a reproducible count receipt."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--vectors', type=Path, default=ROOT/'build/buffered-vectors-v3.json')
    parser.add_argument('--suite', choices=['scanner', 'split'], required=True)
    parser.add_argument('--builtins', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    vectors = json.loads(args.vectors.read_text())
    assert vectors['sdk_version'] == 'go1.27.4'
    fixture = ROOT/'seed/tests/fixtures'/('buffered-scanner-traces.cinch' if args.suite == 'scanner' else 'buffered-split-traces.cinch')
    marker = '  -- | Independent full scanner trace ' if args.suite == 'scanner' else '  -- | Independent direct observations '
    header = fixture.read_text().split(marker, 1)[0]
    assert 'report-failure ::' in header and 'main ::' not in header
    if args.builtins:
        for name, capability in [('bytes', 'byte'), ('runes', 'rune'), ('lines', 'line'), ('words', 'word')]:
            header = header.replace('scan-split scan-'+name, 'scan-'+capability+'-split')
    text = header
    functions = []
    if args.suite == 'scanner':
        rows = vectors['scanners']
        count = sum(len(row['expected']) for row in rows)
        for index, row in enumerate(rows):
            compressed = []
            for observation in row['expected']:
                item = observed(observation)
                if compressed and compressed[-1][1] == item:
                    compressed[-1] = (compressed[-1][0] + 1, item)
                else:
                    compressed.append((1, item))
            name = 'tracecase'+str(index)
            functions.append(name)
            assert len(row['expected']) < 998
            text += f'\n  -- | Complete independent scanner history {index}; terminal-empty panic uses the reviewed named refusal.\n  {name} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int\n  {name} unit = let calls = Cell.cell-new Nil in\n    let raw = scanner (source (unhex "{row["input"]}") 0 {row["chunk"]} {row["together"]} 0 calls False False) in\n    case scanner-buffer-size raw {row["capacity"]} {literal(row["maximum"])} of\n      Err failure -> {index*1000+998}\n      Ok buffered -> case scanner-split buffered (splitter {KINDS[row["kind"]]}) of\n        Err failure -> {index*1000+999}\n        Ok configured -> scan-checks configured [\n'
            text += ',\n'.join(f'          (ScanCheck {repetitions} {more} {token} {error} {valid})' for repetitions, (more, token, error, valid) in compressed)
            text += f'\n        ] {index*1000+1}\n'
    else:
        rows = vectors['splits']
        count = len(rows)
        for start in range(0, len(rows), 16):
            name = 'batchcase'+str(start//16)
            functions.append(name)
            text += f'\n  -- | Independent direct observations {start+1} through {min(start+16,len(rows))}, including reviewed safe empty-input adaptation.\n  {name} :: Unit -> Int\n  {name} unit = direct-checks [\n'
            checks = []
            for row in rows[start:start+16]:
                if row['panic']:
                    assert row['kind'] in ['bytes', 'runes'] and row['input'] == '' and not row['eof']
                    advance, token, error = 0, 'None', 0
                else:
                    advance = row['advance']
                    token = 'None' if row['nil'] else f'(Some (unhex "{row["token"]}"))'
                    error = CODES[row['error']]
                checks.append(f'    (SplitCheck {KINDS[row["kind"]]} (unhex "{row["input"]}") {row["eof"]} {advance} {token} {error})')
            text += ',\n'.join(checks)+f'\n  ] {start+1}\n'
        for name in functions:
            text = text.replace(name+' :: Unit -> Int', name+' :: forall r. using (io :: Io r) => Unit ->{IO r} Int')
    text += runner(functions)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(text)
    manifest = {'suite': args.suite, 'builtins': args.builtins, 'histories': len(rows) if args.suite == 'scanner' else None,
                'observations': count, 'source_sha256': digest(args.output), 'vectors_sha256': digest(args.vectors), 'sdk_version': vectors['sdk_version'],
                'helper_fixture_sha256': digest(fixture), 'generator_sha256': digest(Path(__file__)),
                'status': 'Generated independent observations; no seed/runtime result implied.'}
    args.output.with_suffix('.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(f'Generated {count} independent {args.suite} observations')


if __name__ == '__main__':
    main()
