#!/usr/bin/env python3
"""Validate complete refined scalar ABI contracts and represented source-absent aliases."""
from __future__ import annotations

import argparse
import copy
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects
from bootstrap_interfaces import canonical, interface_hash

HEADER = '-- | Exercise complete refined foreign contracts.\nmodule Main where\n  import Data.Nat (nat-from-i64)\n'


def consumer(binding: str, imports: str) -> str:
    """Require the result refinement when converting the machine result into a natural number."""
    return HEADER + imports + '\n  -- | Observe the exact bounded machine result.\n  main :: Int\n  main = let observed = nat-from-i64 (' + binding + ' (-1) 4095) in if observed == 4095 then 0 else 1\n'


def main() -> None:
    """Retain positive contracts, contradictory claims and executable-body refusal as separate observations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    manifest, objects = args.artifacts / 'interfaces.json', args.artifacts / 'objects.json'
    bootstrap.write_json(manifest, provider['interfaces'])
    bootstrap.write_json(objects, provider['configuration'])
    machine_path = next(Path(row[2]) for row in provider['interfaces']['interfaces'] if row[0] == 'Data.Machine')
    original = bootstrap.read_json(machine_path)
    report = {'status': 'running', 'compiler_sha256': bootstrap.file_hash(args.compiler),
              'provider': provider['identity'], 'runtime_sha256': bootstrap.file_hash(provider['runtime']),
              'source_shadow_used': False, 'observations': [], 'runtime_profiles': []}

    def persist() -> None:
        """Keep original verdicts available even when a later contract fails."""
        bootstrap.write_json(args.output, report)

    def invoke(name: str, text: str, *, document: dict | None = None, admitted: Path = manifest,
               accepted: bool = True, mode: str = '--check', linked: bool = False,
               diagnostic: str | None = None) -> Path:
        """Check one exact source against its own retained interface inventory and diagnostic output."""
        directory = args.artifacts / name
        directory.mkdir()
        if document is not None:
            # Synthetic contract controls are separate modules, never changed provider files or fabricated object units.
            document = copy.deepcopy(document)
            document.update(module='MaskContractFixture', source_hash='0', units=[])
            value = next(row for row in document['exports']['values'] if row['name'] == 'mask-i64')
            foreign = next(row for row in document['exports']['foreign'] if row['symbol'] == 'mask-i64')
            value['symbol'] = 'MaskContractFixture.mask-i64'
            for key in document['exports']:
                document['exports'][key] = []
            document['exports'].update(values=[value], foreign=[foreign])
            contract = directory / 'MaskContractFixture.cinchif'
            contract.write_text(canonical(document))
            index = copy.deepcopy(provider['interfaces'])
            index['interfaces'].append(['MaskContractFixture', interface_hash(document), str(contract)])
            admitted = directory / 'interfaces.json'
            bootstrap.write_json(admitted, index)
            text = text.replace('import Data.Machine (mask-i64)', 'import MaskContractFixture (mask-i64)')
        source = directory / 'Main.cinch'
        source.write_text(text)
        prefix = directory / ('consumer.ll' if mode == '--emit-ir' else 'checked')
        command = [str(args.compiler.resolve()), mode, '--interfaces', str(admitted)]
        if mode == '--emit-ir':
            command.append('--rc-debug')
        if linked:
            command.extend(['--objects', str(objects)])
        command.extend([str(source), str(prefix)])
        result = subprocess.run(['taskset', '-c', str(args.cpu), *command], capture_output=True, text=True,
            env={**bootstrap.environment(), 'CINCH_STACK_SIZE': '64M', 'CINCH_LEAK_CHECK': '1'})
        (directory / 'compile.log').write_text(result.stdout + result.stderr)
        diagnostic_path = Path(str(prefix) + '.diagnostics')
        detail = diagnostic_path.read_text() if diagnostic_path.exists() else result.stdout + result.stderr
        report['observations'].append({'name': name, 'accepted': result.returncode == 0, 'exit': result.returncode,
            'diagnostics': detail, 'command': command, 'source_sha256': bootstrap.file_hash(source),
            'synthetic_contract': document is not None, 'manifest_sha256': bootstrap.file_hash(admitted)})
        persist()
        if (result.returncode == 0) != accepted or 'leaked=0' not in result.stderr:
            raise bootstrap.BootstrapError(f'{name}: unexpected compiler verdict {result.returncode}: {detail}')
        if diagnostic is not None and diagnostic not in detail:
            raise bootstrap.BootstrapError(f'{name}: missing exact contract diagnostic: {detail}')
        return prefix

    def runtime(name: str, ir: Path) -> None:
        """Require the ordinary provider call at both optimization levels and fixed consumer memory limits."""
        text = ir.read_text()
        encoded = ''.join('\\' + format(byte, '02X') for byte in b'Data.Machine.mask-i64.object-closure')
        if not any('call i64' in line and encoded in line for line in text.splitlines()) or ' = and i64 ' in text:
            raise bootstrap.BootstrapError('a stronger refined contract acquired intrinsic implementation authority')
        for opt in [0, 2]:
            executable = ir.parent / f'consumer-o{opt}'
            command = [os.environ.get('CINCH_CLANG', 'clang'), f'-O{opt}', str(ir), str(provider['object']),
                       str(provider['runtime']), '-ldl', '-lpthread', '-lm', '-o', str(executable)]
            result = subprocess.run(['taskset', '-c', str(args.cpu), *command], capture_output=True, text=True)
            (ir.parent / f'link-o{opt}.log').write_text(result.stdout + result.stderr)
            if result.returncode:
                raise bootstrap.BootstrapError(f'{name}: link failed: {result.stderr}')
            for debug in [False, True]:
                result = subprocess.run(['taskset', '-c', str(args.cpu), str(executable)], capture_output=True, text=True,
                    env={**os.environ, 'CINCH_STACK_SIZE': '8M', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                         'CINCH_RC_DEBUG': str(int(debug)), 'CINCH_LEAK_CHECK': '1'})
                report['runtime_profiles'].append({'name': name, 'optimization': opt, 'debug': debug,
                    'exit': result.returncode, 'stderr': result.stderr, 'executable_sha256': bootstrap.file_hash(executable),
                    'llvm_sha256': bootstrap.file_hash(ir), 'ordinary_provider_call_retained': True})
                persist()
                if result.returncode or 'leaked=0' not in result.stderr:
                    raise bootstrap.BootstrapError(f'{name}: refined result or ownership failed: {result.stderr}')

    normal = consumer('mask-i64', '  import Data.Machine (mask-i64)')
    invoke('actual-refined', normal)
    invoke('qualified', consumer('W.mask-i64', '  import Data.Machine as W'))
    invoke('renamed', consumer('clipped', '  import Data.Machine (mask-i64 as clipped)'))
    invoke('partial', HEADER + '  import Data.Machine (mask-i64)\n  -- | Preserve the dependent result through a partial value.\n  main :: Int\n  main = let clip = mask-i64 (-1) in let observed = nat-from-i64 (clip 4095) in if observed == 4095 then 0 else 1\n')
    invoke('negative-mask-refused', normal.replace('(-1) 4095', '(-1) (-1)'), accepted=False,
           diagnostic='refinement predicate is not established')
    mutations = [
        ('alpha-defining', lambda value, row: row.update(type='(I64 -> ((limit :: (Algebra.Arithmetic.InRange 0 9223372036854775807 I64)) -> refined ((0 <= answer && answer <= limit)) I64))')),
        ('contradictory-upper', lambda value, row: row.update(type=row['type'].replace('r <= a1', 'r < a1'))),
        ('contradictory-domain', lambda value, row: row.update(type=row['type'].replace('9223372036854775807', '4095'))),
        ('contradictory-resolved', lambda value, row: value['resolved'].update(scheme=value['resolved']['scheme'].replace('cinch-result <= m', 'cinch-result < m'))),
        ('defining-effects', lambda value, row: row.update(effects='Global')),
        ('defining-abi', lambda value, row: row.update(abi='component')),
        ('latent-effects', lambda value, row: value['resolved'].update(scheme=value['resolved']['scheme'].replace(' -> (refined', ' ->{Global} (refined'))),
    ]
    for name, mutation in mutations:
        document = copy.deepcopy(original)
        value = next(row for row in document['exports']['values'] if row['name'] == 'mask-i64')
        row = next(row for row in document['exports']['foreign'] if row['symbol'] == 'mask-i64')
        mutation(value, row)
        invoke(name, normal, document=document, accepted=name == 'alpha-defining',
               diagnostic=None if name == 'alpha-defining' else 'imported scalar intrinsic disagrees with its defining ABI')
    invoke('actual-no-object', normal, accepted=False, mode='--emit-ir')
    document = copy.deepcopy(original)
    value = next(row for row in document['exports']['values'] if row['name'] == 'mask-i64')
    row = next(row for row in document['exports']['foreign'] if row['symbol'] == 'mask-i64')
    row['type'] = row['type'].replace('r <= a1', 'r <= 666')
    value['resolved']['scheme'] = value['resolved']['scheme'].replace('cinch-result <= m', 'cinch-result <= 666')
    value['scheme'] = value['scheme'].replace('cinch-result <= m', 'cinch-result <= 666')
    invoke('matching-forged-refinement-no-body', normal, document=document, accepted=False, mode='--emit-ir')
    invoke('matching-forged-refinement-provider', normal, document=document, accepted=False, mode='--emit-ir', linked=True)
    runtime('actual-refined', invoke('actual-runtime', normal, mode='--emit-ir', linked=True))
    producer = invoke('facade-producer', '''-- | Reexport the exact represented refined value under a local alias.
module MaskFacade where
  import Data.Machine (mask-i64 as clipped)
  export clipped
-- | Close the interface producer without invoking its value.
module Main where
  -- | Return after exporting the contract.
  main :: Int
  main = 0
''', mode='--emit-interfaces')
    contract = Path(str(producer) + '.MaskFacade.cinchif')
    facade = bootstrap.read_json(contract)
    index = copy.deepcopy(provider['interfaces'])
    index['interfaces'].append(['MaskFacade', interface_hash(facade), str(contract)])
    admitted = args.artifacts / 'facade-interfaces.json'
    bootstrap.write_json(admitted, index)
    producer.with_name('Main.cinch').rename(producer.with_name('Main.source-retained'))
    report['producer_source_absent'] = not producer.with_name('Main.cinch').exists()
    runtime('source-absent-facade', invoke('source-absent-facade', consumer('clipped', '  import MaskFacade'),
                                         admitted=admitted, mode='--emit-ir', linked=True))
    for name in ['facade-bounds', 'facade-arity', 'facade-unit', 'facade-foreign']:
        forged = copy.deepcopy(facade)
        if name == 'facade-bounds':
            forged['exports']['values'][0]['resolved']['scheme'] = forged['exports']['values'][0]['resolved']['scheme'].replace('cinch-result <= m', 'cinch-result <= 666')
        elif name == 'facade-arity':
            forged['exports']['values'][0]['arity'] += 1
        elif name == 'facade-unit':
            forged['units'] = [{'name': 'MaskFacade.injected', 'body': 'untrusted'}]
        else:
            forged['exports']['foreign'] = [copy.deepcopy(original['exports']['foreign'][0])]
        path = args.artifacts / (name + '.cinchif')
        path.write_text(canonical(forged))
        modified = copy.deepcopy(index)
        modified['interfaces'][-1] = ['MaskFacade', interface_hash(forged), str(path)]
        inventory = args.artifacts / (name + '.interfaces.json')
        bootstrap.write_json(inventory, modified)
        invoke(name, consumer('clipped', '  import MaskFacade'), admitted=inventory,
               accepted=False, mode='--emit-ir', linked=True)
    bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    report['status'] = 'passed'
    persist()
    print(f'{len(report["observations"])} refined contract/alias observations and {len(report["runtime_profiles"])} runtime profiles passed.', flush=True)


if __name__ == '__main__':
    main()
