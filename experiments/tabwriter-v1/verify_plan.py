#!/usr/bin/env python3
"""Verify all exact installed tabwriter declaration records without treating a static plan as implementation coverage."""
from pathlib import Path
import argparse
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent


def digest(path):
    """Identify the exact SDK source, license and compatibility-file evidence used by this plan."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Require nineteen exact records, twelve current identities and explicit unimplemented status for every mapping."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sdk', type=Path, default=Path('/tmp/cinch-go-current/go'))
    args = parser.parse_args()
    surface = json.loads((HERE / 'reference-surface.json').read_text())
    provenance = json.loads((HERE / 'reference-provenance.json').read_text())
    plan = json.loads((HERE / 'capability-plan.json').read_text())
    assert surface['package'] == 'text/tabwriter'
    records = surface['declarations']
    assert len(records) == len(set(records)) == 19
    assert [row['reference_declaration'] for row in plan['records']] == records
    assert len({row['current_identity'] for row in plan['records']}) == 12
    assert not plan['production_adopted']
    assert all(row['status'] == 'planned; not implemented or qualified' for row in plan['records'])
    assert provenance['exact_declaration_records'] == 19 and provenance['historical_duplicate_records'] == 7
    assert digest(args.sdk / 'bin/go') == provenance['executable_sha256']
    assert (args.sdk / 'VERSION').read_text() == provenance['version_file']
    for item in [provenance['source'], provenance['license']]:
        assert digest(args.sdk / item['path']) == item['sha256']
    for name, expected in provenance['api_files'].items():
        assert digest(args.sdk / name) == expected
    found = set()
    for name in provenance['api_files']:
        for line in (args.sdk / name).read_text().splitlines():
            match = re.match(r'pkg text/tabwriter(?: \([^)]+\))?, (.*?)(?: #\d+)?$', line)
            if match:
                found.add(match[1])
    assert found == set(records)
    for record, origins in provenance['origins'].items():
        assert origins
        for origin in origins:
            actual = (args.sdk / origin['file']).read_text().splitlines()[origin['line'] - 1]
            assert record in actual
    print('Verified19 exact SDK records /12 current identities; all implementation/vector/runtime/performance gates remain open')


if __name__ == '__main__':
    main()
