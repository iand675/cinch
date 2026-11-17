#!/usr/bin/env python3
"""Freeze and source-check the isolated Region/both-effect closure on its actual qualified compiler epoch."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import resource
import shutil
import subprocess
import time
import verify

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SNAPSHOT = Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen')
COMPILER = '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
RUNTIME = 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
SOURCE = '75b4bab70d18ed3dd66bf6b8c5153943764dabf3'


def digest(path):
    """Keep compiler, source, defining-interface and source-manifest identities distinct."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def guard(work):
    """Refuse a new phase below its reserve or above its complete persistent-output budget."""
    if shutil.disk_usage(ROOT).free < 250 * 1024 * 1024:
        raise RuntimeError('250MiB global pause guard')
    if sum(p.stat().st_size for p in work.rglob('*') if p.is_file()) > 30 * 1024 * 1024:
        raise RuntimeError('30MiB source-only output cap')


def limits():
    """Bound the compiler host independently of any future consumer memory profile."""
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    resource.setrlimit(resource.RLIMIT_FSIZE, (30 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))


def closure(entry):
    """Select the exact seven overrides without shadowing the compiler's 66 embedded modules."""
    family = HERE / 'embedded-family.cinchpkg'
    embedded = {row['name'] for row in json.loads(family.read_text())['modules']}
    assert len(embedded) == 66
    overrides = {p.name[:-6]: p for p in (HERE / 'sources').glob('*.cinch')}
    found = {}

    def visit(text):
        """Resolve every source-distributed import once; retain ordinary dependencies unchanged."""
        for name in re.findall(r'^\s*import\s+([A-Z][\w.]*)', text, re.M):
            if name in embedded or name in found:
                continue
            path = overrides.get(name, ROOT / 'compiler/prelude' / (name.replace('.', '/') + '.cinch'))
            found[name] = path
            visit(path.read_text())

    visit(entry)
    assert not set(overrides) - set(found), 'an intended proposal module was not imported'
    return found


def public_contracts(work, manifest):
    """Read genuine defining metadata and complete constructor identities without rewriting an interface."""
    output = []
    for module in manifest['modules']:
        path = work / 'build' / (module + '.cinchif')
        interface = json.loads(path.read_text())
        nominal = {}
        for value in interface['exports']['values']:
            for contract in (value.get('resolved') or {}).get('nominals', []):
                identity = contract['identity']
                if identity in nominal:
                    assert nominal[identity] == contract, identity
                nominal[identity] = contract
        exported = {row['name']: row for row in interface['exports']['types']}
        for header in manifest['headers']:
            identity = header['identity']
            if not header['public'] or identity.rsplit('.', 1)[0] != module:
                continue
            contract = nominal[identity]
            parameters = contract['parameters']
            assert parameters[0] == {'implicit': False, 'kind': 'Region', 'name': 'r'}, identity
            assert all(p['kind'] == 'Type' for p in parameters[1:]), identity
            assert exported[identity.rsplit('.', 1)[1]]['parameters'] == parameters, identity
            constructors = [module + '.' + row.split(' ::', 1)[0] for row in header['defining_constructors']]
            assert contract['constructors'] == constructors, identity
            output.append({'identity': identity, 'parameters': parameters, 'constructors': constructors,
                           'interface': str(path), 'interface_sha256': digest(path),
                           'direct_audit_witness': header['direct_audit_witness'] is not None})
    assert len(output) == 9
    return output


def main():
    """Stop at the first actual source result; neither runtime nor production adoption is implied."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--deadline', type=int, default=60)
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    verify.main()
    assert shutil.disk_usage(ROOT).free >= 350 * 1024 * 1024, '350MiB fresh-start guard'
    work = args.work.resolve()
    assert not work.exists(), 'refuse to overwrite a historical source-check epoch'
    compiler = SNAPSHOT / 'cinch'
    runtime = SNAPSHOT / 'libcinchruntime.a'
    provenance_path = SNAPSHOT / 'provenance.json'
    provenance = json.loads(provenance_path.read_text())
    assert digest(compiler) == COMPILER == provenance['compiler_sha256']
    assert digest(runtime) == RUNTIME == provenance['runtime_sha256']
    assert provenance['source_checkpoint'] == SOURCE
    manifest_path = HERE / 'proposal.json'
    manifest = json.loads(manifest_path.read_text())
    source = (HERE / 'Main.cinch').read_text()
    paths = closure(source)
    work.mkdir(parents=True)
    for name, path in paths.items():
        shutil.copyfile(path, work / (name + '.cinch'))
    (work / 'Main.cinch').write_text(source)
    shutil.copyfile(HERE / 'embedded-family.cinchpkg', work / 'embedded-family.cinchpkg')
    guard(work)
    command = [str(compiler), 'check', str(work / 'Main.cinch')]
    before = time.monotonic()
    timeout = False
    try:
        result = subprocess.run(command, cwd=work, capture_output=True, text=True,
                                preexec_fn=limits, timeout=args.deadline,
                                env={**os.environ, 'CINCH_STACK_SIZE': '64M', 'RUST_MIN_STACK': '67108864'})
        code, stdout, stderr = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        timeout = True
        code = None
        stdout = error.stdout or b''
        stderr = error.stderr or b''
        stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
    report = {'scope': __doc__, 'production_adopted': False, 'runtime_claim': False,
              'compiler_sha256': COMPILER, 'runtime_sha256': RUNTIME, 'compiler_source': SOURCE,
              'provenance_sha256': digest(provenance_path), 'provenance': provenance,
              'proposal_sha256': digest(manifest_path), 'main_sha256': digest(work / 'Main.cinch'),
              'embedded_family_sha256': digest(HERE / 'embedded-family.cinchpkg'),
              'source_selection': {name: {'original_path': str(path), 'sha256': digest(path)} for name, path in paths.items()},
              'command': command, 'returncode': code, 'timeout_without_verdict': timeout,
              'stdout': stdout, 'stderr': stderr, 'seconds': time.monotonic() - before,
              'deadline_seconds': args.deadline, 'compiler_host_stack_bytes': 67108864,
              'cpu': 2, 'source_gate_passed': code == 0 and not timeout,
              'private_WriterChain_contract_pending': True}
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    if code == 0 and not timeout:
        report['public_contracts'] = public_contracts(work, manifest)
        report['genuine_interfaces'] = {p.name: digest(p) for p in (work / 'build').glob('*.cinchif')}
        report['artifact_bytes'] = sum(p.stat().st_size for p in work.rglob('*') if p.is_file())
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    guard(work)
    print(stdout + stderr or ('Deadline without verdict' if timeout else 'Isolated Region source and nine public defining contracts accepted'), flush=True)
    raise SystemExit(124 if timeout else code)


if __name__ == '__main__':
    main()
