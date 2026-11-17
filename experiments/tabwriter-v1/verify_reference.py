#!/usr/bin/env python3
"""Verify the SDK-free tabwriter observations and partial configuration receipts without claiming formatter coverage."""
from pathlib import Path
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent


def main():
    """Check every fixed observation and preserved failure binding, retaining all still-open package gates."""
    gates = json.loads((HERE / 'reference-gates.json').read_text())
    for name, expected in gates['files'].items():
        assert hashlib.sha256((HERE / name).read_bytes()).hexdigest() == expected, name
    requests = json.loads((HERE / 'fixtures/requests.json').read_text())
    observed = json.loads((HERE / 'fixtures/observations.json').read_text())
    assert len(requests) == len(observed['cases']) == gates['requests'] == 612
    assert [row['id'] for row in requests] == [row['id'] for row in observed['cases']]
    assert len({row['id'] for row in requests}) == len(requests)
    actions = [step for row in observed['cases'] for step in row['actions'] or []]
    offers = [offer for step in actions for offer in step['offers']]
    assert len(actions) == gates['actions'] == 1506
    assert len(offers) == gates['offers'] == 6756
    assert sum(not row['data'] for row in offers) == gates['empty_offers'] == 806
    assert sum(bool(row.get('panic')) for row in actions) == gates['method_panics'] == 10
    assert sum(bool(row.get('constructor_panic')) for row in observed['cases']) == gates['constructor_panics'] == 3
    assert sum(bool(row.get('error')) for row in actions) == gates['returned_errors'] == 56
    assert all('reported' not in row for row in offers if row.get('panic')), 'no invented return from a panicking sink'
    assert all('count' not in row for row in actions if row.get('panic')), 'no invented return from a panicking Write'
    for row in offers:
        raw = bytes.fromhex(row['data'])
        assert 0 <= row['accepted'] <= len(raw)
        assert isinstance(row.get('reported', 0), int), 'signed malformed reports remain literal'
    capture = json.loads((HERE / 'proof/reference-capture.json').read_text())
    assert capture['capture_complete'] and capture['reused_build']['prior_capture_complete'] is False
    assert [row['name'] for row in capture['steps']] == ['observe'], 'exact successful executable reused; no hidden rebuild'
    assert all(row['returncode'] == 0 and row['interrupted_reason'] is None for row in capture['steps'])
    failure = json.loads((HERE / 'proof/reference-build-budget-failure.json').read_text())
    assert failure['observed_cache_growth_bytes'] > failure['artifact_and_observed_cache_growth_cap_bytes']
    assert [row['name'] for row in failure['steps']] == ['build'] and not failure.get('capture_complete', False)
    source = json.loads((HERE / 'proof/config-source.json').read_text())
    assert source['source_gate_passed'] and source['source_only'] and not source['runtime_claim']
    assert source['compiler_sha256'] == gates['config_compiler'] and source['runtime_sha256'] == gates['config_runtime']
    assert source['sources']['Text.Tabwriter.Config']['sha256'] == gates['files']['sources/Text.Tabwriter.Config.cinch']
    runtime = json.loads((HERE / 'proof/config-runtime.json').read_text())
    assert runtime['all_passed'] and len(runtime['runs']) == 2
    assert runtime['compiler_sha256'] == gates['config_compiler'] and runtime['runtime_sha256'] == gates['config_runtime']
    for row in runtime['runs']:
        assert row['returncode'] == 0 and row['stdout'] == '' and 'live=0 immortal=0 leaked=0' in row['stderr']
    docs = json.loads((HERE / 'proof/config-doc.json').read_text())
    assert not docs['diagnostics'] and not docs['orphans']
    todo, count = list(docs['declarations']), 0
    while todo:
        declaration = todo.pop()
        count += 1
        assert declaration['doc'] and not declaration['doc']['legacy'], declaration['name']
        assert not re.search(r'\bgo(?:lang)?\b', declaration['doc']['markdown'], re.I)
        todo.extend(declaration['children'])
    assert count == gates['config_doc_declarations_including_children'] == 29
    assert not any(gates[key] for key in ('formatter_qualified', 'streaming_qualified', 'performance_reviewed', 'production_adopted'))
    print('Verified612 independent protocol cases and Config-only source/runtime/docs; full formatter/streaming/performance gates remain open')


if __name__ == '__main__':
    main()
