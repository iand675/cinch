#!/usr/bin/env python3
"""Inspect genuine public and private defining contracts for one checked candidate.

Normal interfaces intentionally hide internal carrier names. A separately
identified observation copy exports seven pure identities, allowing its checked
private nominal contracts to be inspected without changing the candidate API.
No serialized header, constructor inventory or compiler identity is rewritten.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
from check_compare import digest
from multipart_qualification_support import archive_cache


def contracts(interface: dict) -> dict:
    """Collect consistent defining nominal contracts from genuine exported schemes."""
    found = {}
    for value in interface['exports']['values']:
        for nominal in (value.get('resolved') or {}).get('nominals', []):
            identity = nominal['identity']
            if identity in found and found[identity] != nominal:
                raise RuntimeError('contradictory defining contracts: ' + identity)
            found[identity] = nominal
    return found


def inspect(interface: dict, changes: list[dict], source: str, public: bool) -> list:
    """Check actual parameter kinds and complete defining constructor identities."""
    nominals = contracts(interface)
    exported = {row['name']: row for row in interface['exports']['types']}
    result = []
    for change in changes:
        if (change['visibility'] == 'public') != public:
            continue
        name = change['name']; identity = 'Effect.Multipart.' + name
        nominal = nominals[identity]
        parameters = nominal['parameters']
        if not parameters or parameters[0]['name'] != 'r' or parameters[0]['kind'] != 'Region':
            raise RuntimeError('missing defining Region authority: ' + identity)
        if any(row['kind'] != 'Type' for row in parameters[1:]):
            raise RuntimeError('changed payload parameter kind: ' + identity)
        block = re.search(r'^  ' + re.escape(change['proposal_header']) + r'\n((?:    .*\n|\n)+)', source, re.M)
        if not block:
            raise RuntimeError('missing exact declaration: ' + name)
        constructors = ['Effect.Multipart.' + match for match in re.findall(r'^    (\w+)\s*::', block.group(1), re.M)]
        if not constructors or nominal['constructors'] != constructors:
            raise RuntimeError('changed or incomplete constructor inventory: ' + identity)
        if public and exported[name]['parameters'] != parameters:
            raise RuntimeError('public header disagrees with private defining contract: ' + identity)
        result.append({'identity': identity, 'visibility': change['visibility'],
                       'parameters': parameters, 'constructors': nominal['constructors']})
    return result


def main() -> None:
    """Require the fresh exact source gate before checking an isolated observation copy."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-work', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    source_work, candidate, work = args.source_work.resolve(), args.candidate.resolve(), args.work.resolve()
    compiler = args.compiler.resolve(); work.mkdir(parents=True, exist_ok=True)
    gate_path = source_work / 'report.json'; gate = json.loads(gate_path.read_text())
    manifest_path = candidate / 'manifest.json'; manifest = json.loads(manifest_path.read_text())
    shutil.copyfile(manifest_path, work / 'candidate.manifest.json')
    if not any(
            row['frontend'] == 'rust' and row['returncode'] == 0 and row['compiler_sha256'] == digest(compiler)
            for row in gate['checks']):
        raise RuntimeError('exact compiler has no fresh candidate source admission')
    original = candidate / 'sources/Effect.Multipart.cinch'
    if digest(original) != gate['sources']['Effect.Multipart'] or digest(original) != manifest['sources']['Effect.Multipart.cinch']:
        raise RuntimeError('candidate differs from checked source')
    public_path = source_work / 'defining-interfaces/Effect.Multipart.cinchif'
    retained = next(row['genuine_defining_interfaces'] for row in gate['checks'] if row['frontend'] == 'rust')
    if digest(public_path) != retained['Effect.Multipart.cinchif']:
        raise RuntimeError('changed genuine candidate interface')
    source = original.read_text()
    rows = inspect(json.loads(public_path.read_text()), manifest['changes'], source, True)
    observed = source
    witnesses = []
    for change in manifest['changes']:
        if change['visibility'] != 'private':
            continue
        name = change['name']; witness = 'observe-' + re.sub(r'(?<!^)([A-Z])', r'-\1', name).lower()
        parameters = re.sub(r'^data \w+ \(r :: Region\)', '', change['proposal_header']).removesuffix(' where').strip()
        target = name + ' r' + (' ' + parameters if parameters else '')
        observed += f'\n  export {witness}\n\n  -- | Observe the defining private carrier kind without adding an operation.\n  {witness} :: forall (r :: Region) {parameters}. {target} -> {target}\n  {witness} value = value\n'
        witnesses.append({'carrier': name, 'observer': witness})
    for name, expected in gate['sources'].items():
        path = source_work / (name + '.cinch')
        if digest(path) != expected:
            raise RuntimeError('changed checked dependency: ' + name)
        shutil.copyfile(path, work / path.name)
    (work / 'Effect.Multipart.cinch').write_text(observed)
    (work / 'Main.cinch').write_text('-- | Inspect a source-checked carrier observation copy without execution.\nmodule Main where\n  import Effect.Multipart\n  -- | Keep metadata checking independent of runtime.\n  main :: Int\n  main = 0\n')
    command = [str(compiler), 'check', str(work / 'Main.cinch')]
    result = subprocess.run(command, cwd=work, capture_output=True, text=True, timeout=60,
                            env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
    report = {'scope': __doc__, 'production_adopted': False, 'runtime_claim': False,
              'compiler_sha256': digest(compiler), 'manifest_qualification_compiler_sha256': manifest['compiler']['sha256'],
              'source_gate_sha256': digest(gate_path),
              'candidate_manifest_sha256': digest(manifest_path), 'original_effect_sha256': digest(original),
              'public_interface_sha256': digest(public_path), 'public_contracts': rows,
              'observation_source_sha256': digest(work / 'Effect.Multipart.cinch'),
              'witnesses': witnesses, 'command': command, 'returncode': result.returncode,
              'stdout': result.stdout, 'stderr': result.stderr, 'compiler_host_stack_bytes': 67108864}
    path = work / 'report.json'; path.write_text(json.dumps(report, indent=2) + '\n')
    if result.returncode:
        raise RuntimeError(result.stderr)
    target = work / 'Effect.Multipart.defining.cinchif'
    shutil.copyfile(work / 'build/Effect.Multipart.cinchif', target)
    report['observation_interface_sha256'] = digest(target)
    report['private_contracts'] = inspect(json.loads(target.read_text()), manifest['changes'], source, False)
    if len(rows) != 12 or len(report['private_contracts']) != 7:
        raise RuntimeError('incomplete defining carrier inspection')
    archive_cache(work, 'private-carrier-observation', report)
    report['artifact_bytes'] = sum(file.stat().st_size for file in work.rglob('*') if file.is_file())
    path.write_text(json.dumps(report, indent=2) + '\n')
    if report['artifact_bytes'] > 5 * 1024 * 1024:
        raise RuntimeError('source-only artifact cap reached')
    print('12 public and 7 private genuine defining carrier contracts passed', flush=True)


if __name__ == '__main__':
    main()
