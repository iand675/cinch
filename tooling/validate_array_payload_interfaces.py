#!/usr/bin/env python3
"""Check sealed-array authority using genuine source-absent interfaces and forged payload claims."""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess

from bootstrap_interfaces import canonical, interface_hash, read_contract

PRODUCER = '''-- | Supply an ordinary carrier with a private constructor and no primitive authority.
module PayloadFixture where
  export Carrier, make
  -- | Retain an ordinary typed value behind a private constructor.
  data Carrier a where
    -- | This constructor is complete defining layout evidence, not primitive evidence.
    Held :: a -> Carrier a
  -- | Expose the ordinary carrier's type without exposing its constructor name.
  make :: forall a. a -> Carrier a
  make value = Held value
-- | Give interface production a closed process entry.
module Main where
  -- | Return without allocating a recursive value.
  main :: Int
  main = 0
'''
CLIENTS = {
    'real-builtin-array': (True, '  import PayloadFixture\n', 'Array Arena'),
    'ordinary-import': (False, '  import PayloadFixture\n', 'Array (Carrier Arena)'),
    'qualified-import': (False, '  import PayloadFixture as Alias\n', 'Array (Alias.Carrier Arena)'),
    'shadowed-spelling': (False, '  import PayloadFixture (Carrier as Array)\n', 'Array Arena'),
}


def digest(path: Path) -> str:
    """Identify the exact consumed artifact bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Delete producer sources before checking clients against real and modified contracts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=4)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=True)
    environment = {**os.environ, 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0'}
    report = {'description': __doc__, 'compiler_sha256': digest(args.compiler),
              'base_manifest_sha256': digest(args.interfaces), 'cpu_affinity': [args.cpu], 'clients': []}

    def invoke(label: str, arguments: list[str]) -> subprocess.CompletedProcess:
        """Retain exact command diagnostics and require reclamation on accepted and rejected source alike."""
        command = ['taskset', '-c', str(args.cpu), str(args.compiler), *arguments]
        result = subprocess.run(command, env=environment, capture_output=True, text=True)
        (args.artifacts / f'{label}.log').write_text(result.stdout + result.stderr)
        assert 'leaked=0' in result.stderr, (label, result.stderr)
        return result

    source = args.artifacts / 'Producer.cinch'
    source.write_text(PRODUCER)
    prefix = args.artifacts / 'producer'
    result = invoke('export', ['--emit-interfaces', '--interfaces', str(args.interfaces.resolve()), str(source), str(prefix)])
    assert result.returncode == 0, (result.returncode, result.stderr, prefix.with_suffix('.diagnostics').read_text())
    contract = Path(str(prefix) + '.PayloadFixture.cinchif')
    document = read_contract(contract)
    report['producer'] = {'source_sha256': digest(source), 'contract_sha256': digest(contract),
                          'interface_hash': interface_hash(document)}
    source.unlink()

    variants = [('actual', document)]
    modified = copy.deepcopy(document)
    for row in modified['exports']['types']:
        row['strict_positive_payload'] = {'kind': 'finite-owned-array', 'identity': 'Array', 'verified': True}
    variants.append(('forged-payload-flag', modified))
    for variant, contents in variants:
        imported = args.artifacts / f'{variant}.cinchif'
        imported.write_text(canonical(contents))
        manifest = json.loads(args.interfaces.read_text())
        manifest['interfaces'].append(['PayloadFixture', interface_hash(contents), str(imported)])
        admitted = args.artifacts / f'{variant}.interfaces.json'
        admitted.write_text(json.dumps(manifest, separators=(',', ':')))
        for label, (accepted, imports, field) in CLIENTS.items():
            full_label = f'{variant}-{label}'
            client = args.artifacts / f'{full_label}.cinch'
            client.write_text('-- | Consume only the exact admitted interface contract.\nmodule Main where\n' + imports
                              + '  -- | Keep recursive payload authority independent of imported spelling.\n  data Arena where\n'
                              + '    -- | Carry the candidate application in one source-owned field.\n    Pool :: ' + field + ' -> Arena\n'
                              + '  -- | Return without constructing the recursive payload.\n  main :: Int\n  main = 0\n')
            checked = args.artifacts / f'{full_label}.checked'
            result = invoke(full_label, ['--check', '--interfaces', str(admitted), str(client), str(checked)])
            diagnostics = Path(str(checked) + '.diagnostics').read_text()
            assert (result.returncode == 0) == accepted, (full_label, result.returncode, diagnostics)
            if not accepted:
                assert 'positiv' in diagnostics, (full_label, diagnostics)
            report['clients'].append({'name': full_label, 'source_sha256': digest(client),
                                      'contract_sha256': digest(imported), 'accepted': accepted,
                                      'exit': result.returncode, 'diagnostics': diagnostics, 'leaked': 0})
    report['producer_source_removed_before_clients'] = not source.exists()
    report['runner_sha256'] = digest(Path(__file__))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print('Source-absent builtin and ordinary/impostor interface checks passed with zero leaks.', flush=True)


if __name__ == '__main__':
    main()
