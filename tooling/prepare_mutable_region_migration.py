#!/usr/bin/env python3
"""Prepare isolated mutable-container Region declarations from genuine scheme witnesses; never rewrite interfaces."""
from __future__ import annotations

import difflib
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
DEFINITIONS = Path('/workspace/dasl-stdlib-io')
AUDIT = DEFINITIONS / 'bootstrap/evidence/stdlib/stdlib-nominal-region-migration-audit.json'
INTERFACES = DEFINITIONS / 'build/nominal-region-library-imports/build'
OUTPUT = ROOT / 'build/mutable-region-migration-static'


def digest(path: Path) -> str:
    """Identify exact defining source and genuine interface bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def content_identity(raw: bytes) -> str:
    """Reproduce the interface source identity to bind witnesses to the actual defining source."""
    result = 14695981039346656037
    for byte in raw:
        result = ((result ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return str(result)


def main() -> None:
    """Snapshot a declaration-only proposal while recording unchanged public and private nominal roles."""
    audit = json.loads(AUDIT.read_text())
    inputs = {row['module']: row for row in audit['inputs']}
    candidates = [row for row in audit['candidates'] if row['identity'].startswith('Data.Mutable.')]
    assert len(candidates) == 17
    modules = sorted({row['identity'].rsplit('.', 1)[0] for row in candidates})
    assert len(modules) == 16
    OUTPUT.mkdir(parents=True, exist_ok=True)
    records, patches = [], []
    for module in modules:
        path = DEFINITIONS / 'compiler/prelude' / (module.replace('.', '/') + '.cinch')
        original = path.read_text()
        interface = INTERFACES / (module + '.cinchif')
        assert digest(interface) == inputs[module]['sha256']
        assert content_identity(path.read_bytes()) == inputs[module]['source_hash']
        document = json.loads(interface.read_text())
        public = {row['name']: row for row in document['exports']['types']}
        selected = [row for row in candidates if row['identity'].rsplit('.', 1)[0] == module]
        proposal = original
        roles = []
        for row in selected:
            name = row['identity'].rsplit('.', 1)[1]
            assert row['recorded_kind'] == 'Type' and row['position'] == 0 and row['parameter'] == 'r'
            assert public[name]['identity'] == row['identity']
            assert public[name]['parameters'][0]['kind'] == 'Type'
            old = f'  data {name} r '
            assert proposal.count(old) == 1
            proposal = proposal.replace(old, f'  data {name} (r :: Region) ')
            witness = next(item for item in row['uses'] if item['module'] == module)
            value = next(item for item in document['exports']['values'] if item['name'] == witness['value'])
            assert value['resolved']['scheme'] == witness['scheme']
            defining = next(item for item in value['resolved']['nominals'] if item['identity'] == row['identity'])
            assert defining['parameters'][0]['kind'] == 'Type'
            aliases = [item for item in row['uses'] if item['module'] == 'Data.Mutable']
            roles.append({'identity': row['identity'], 'source_declaration': f'data {name} r',
                          'proposed_declaration': f'data {name} (r :: Region)',
                          'classification': 'public nominal type; no constructor exported' if not public[name]['ctors'] else 'public nominal type with exported constructors',
                          'actual_public_header': public[name], 'witness': witness,
                          'facade_witness': aliases[0] if aliases else None,
                          'all_recorded_uses': len(row['uses'])})
        restored = proposal
        for row in selected:
            name = row['identity'].rsplit('.', 1)[1]
            restored = restored.replace(f'  data {name} (r :: Region) ', f'  data {name} r ')
        assert restored == original
        destination = OUTPUT / (module + '.cinch')
        destination.write_text(proposal)
        declarations = re.findall(r'^  data (\w+)([^\n]*)', original, re.M)
        private = [{'name': name, 'declaration': 'data ' + name + rest,
                    'classification': 'private nominal; unchanged ordinary payload/metadata parameters'}
                   for name, rest in declarations if name not in public]
        patches.append(''.join(difflib.unified_diff(original.splitlines(keepends=True), proposal.splitlines(keepends=True),
                                                    fromfile='defining-source/' + module + '.cinch',
                                                    tofile='isolated-proposal/' + module + '.cinch', n=0)))
        records.append({'module': module, 'source': str(path), 'source_sha256': digest(path),
                        'source_identity': inputs[module]['source_hash'], 'interface': str(interface),
                        'interface_sha256': digest(interface), 'proposal': str(destination.relative_to(ROOT)),
                        'proposal_sha256': digest(destination), 'roles': roles, 'unchanged_private_types': private,
                        'source_mutable_imports': re.findall(r'^  import (Data\.Mutable[^\s(]*)', original, re.M)})
    patch = ROOT / 'tooling/mutable-container-nominal-region-proposal.patch'
    patch.write_text(''.join(patches))
    result = {'kind': 'static-declaration-only-mutable-container-Region-migration', 'compiler_invoked': False,
              'source_or_historical_interface_modified': False, 'production_promoted': False,
              'audit': str(AUDIT), 'audit_sha256': digest(AUDIT), 'audit_epoch': audit['interface_epoch'],
              'modules': len(modules), 'nominal_parameters': len(candidates), 'records': records,
              'patch': str(patch.relative_to(ROOT)), 'patch_sha256': digest(patch),
              'unchanged_facade': 'Data.Mutable reexports the defining identities; it declares no replacement nominal.',
              'scope': 'Public and private classification plus exact source/witness linkage only; every migrated closure still needs qualified source/interface/runtime gates.'}
    (ROOT / 'bootstrap/evidence/stdlib/stdlib-mutable-region-migration-results.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'modules': len(modules), 'nominal_parameters': len(candidates),
                      'private_types_unchanged': sum(len(row['unchanged_private_types']) for row in records),
                      'proposal_bytes': sum((ROOT / row['proposal']).stat().st_size for row in records),
                      'audit_sha256': result['audit_sha256']}))


if __name__ == '__main__':
    main()
