#!/usr/bin/env python3
"""Verify actual source-only effect, region and private-constructor refusals without executing an invalid program."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent


def digest(path):
    """Bind each unchanged negative fixture and its precise diagnostic to the recorded compiler epoch."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Require all four distinct semantic barriers; parser errors or resource timeouts cannot satisfy these gates."""
    folder = HERE / 'refusals-v1'
    gates = json.loads((folder / 'gates.json').read_text())
    assert {row['kind'] for row in gates['rows']} == {'mutation-effect', 'private-writer', 'private-cell', 'region-identity'}
    for row in gates['rows']:
        assert digest(HERE / row['source']) == row['source_sha256']
        assert digest(folder / row['report']) == row['report_sha256']
        actual = json.loads((folder / row['report']).read_text())
        assert actual['main_sha256'] == row['source_sha256']
        assert actual['source_only'] and not actual['runtime_claim'] and not actual['production_adopted']
        assert not actual['source_gate_passed'] and actual['returncode'] == 1 and actual['interrupted_reason'] is None
        assert row['expected_diagnostic'] in actual['stderr']
        assert actual['compiler_sha256'] == '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
        assert actual['runtime_sha256'] == 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
    print('Verified four semantic refusals: mutation, region identity, writer privacy and initialized storage privacy')


if __name__ == '__main__':
    main()
