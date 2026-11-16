#!/usr/bin/env python3
"""Validate source-owned phantom kinds through a genuine interface-only client."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

from bootstrap_interfaces import interface_hash, read_contract

PRODUCER = '''-- | Supply source-owned region and ordinary phantom interface contracts.
module MutableFixture where
  export Vector, empty, append, Ordinary, ordinary
  -- | Carry a mutation lifetime independently of the stored element type.
  data Vector r a where
    -- | Start with no stored elements.
    Vector :: Vector r a
  -- | Create a vector in the caller's mutation region.
  empty :: forall r a. Unit ->{Mut r} Vector r a
  empty unit = Vector
  -- | Constrain a vector's element and mutation region together.
  append :: forall r a. Vector r a -> a ->{Mut r} Unit
  append values value = ()
  -- | Retain an ordinary phantom parameter with no region constraint.
  data Ordinary a where
    -- | Construct an ordinary phantom witness.
    Ordinary :: Ordinary a
  -- | Fix an ordinary witness to an ordinary value type.
  ordinary :: Unit -> Ordinary Int
  ordinary unit = Ordinary
-- | Give the export fixture a closed process entry.
module Main where
  -- | Return successfully after interface production.
  main :: Int
  main = 0
'''
CLIENTS = {
    'region-roundtrip': (True, '''  -- | Retain a fresh imported lifetime through an alias.
  use :: forall r. Unit ->{Mut r} Int
  use unit = let values = empty () in let alias = values in let first = append values (0 :: Int) in let second = append alias (1 :: Int) in 0
'''),
    'region-cannot-be-type': (False, '''  -- | Attempt a forbidden imported kind or element instantiation.
  bad :: Vector Int Int -> Unit
  bad value = ()
'''),
    'ordinary-cannot-be-region': (False, '''  -- | Attempt a forbidden imported kind or element instantiation.
  bad :: forall r. Ordinary r ->{Mut r} Unit
  bad value = ()
'''),
    'imported-allocation-monomorphic': (False, '''  -- | Attempt a forbidden imported kind or element instantiation.
  bad :: forall r. Unit ->{Mut r} Int
  bad unit = let values = empty () in let first = append values (0 :: Int) in let second = append values True in 0
'''),
}


def digest(path: Path) -> str:
    """Record the exact executable, source, or exported contract bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Export checked contracts, inspect their kinds, then consume only those files."""
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
    record = {'description': __doc__, 'compiler_sha256': digest(args.compiler),
              'manifest_sha256': digest(args.interfaces), 'cpu_affinity': [args.cpu],
              'producer': {}, 'clients': []}

    def invoke(label: str, arguments: list[str]) -> subprocess.CompletedProcess:
        """Keep process reclamation and exact command receipts for each independent check."""
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
    contract = Path(str(prefix) + '.MutableFixture.cinchif')
    document = read_contract(contract)
    actual = {row['name']: [param['kind'] for param in row['parameters']]
              for row in document['exports']['types']}
    assert actual == {'Ordinary': ['Type'], 'Vector': ['Region', 'Type']}, actual
    record['producer'] = {'source_sha256': digest(source), 'contract_sha256': digest(contract),
                          'interface_hash': interface_hash(document), 'parameter_kinds': actual}
    manifest = json.loads(args.interfaces.read_text())
    manifest['interfaces'].append(['MutableFixture', interface_hash(document), str(contract)])
    admitted = args.artifacts / 'interfaces.json'
    admitted.write_text(json.dumps(manifest, separators=(',', ':')))
    source.unlink()
    for label, (accepted, declarations) in CLIENTS.items():
        client = args.artifacts / f'{label}.cinch'
        client.write_text('-- | Consume only the admitted mutation interface.\nmodule Main where\n'
                          '  import MutableFixture\n' + declarations + '  -- | Return without executing the source contract.\n  main :: Int\n  main = 0\n')
        check_prefix = args.artifacts / f'{label}.checked'
        diagnostics = Path(str(check_prefix) + '.diagnostics')
        result = invoke(label, ['--check', '--interfaces', str(admitted), str(client), str(check_prefix)])
        details = diagnostics.read_text()
        assert (result.returncode == 0) == accepted, (label, result.returncode, details)
        if not accepted:
            assert ('type mismatch' if label == 'imported-allocation-monomorphic' else 'kind mismatch') in details, (label, details)
        record['clients'].append({'name': label, 'source_sha256': digest(client), 'accepted': accepted,
                                  'exit': result.returncode, 'diagnostic': details, 'leaked': 0})
    record['producer_source_removed_before_clients'] = not source.exists()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(record, indent=2) + '\n')
    print('Interface kinds and four source-absent consumers passed with zero leaks.', flush=True)


if __name__ == '__main__':
    main()
