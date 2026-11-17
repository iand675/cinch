"""Preserve one bounded source/documentation qualification epoch without code emission."""
from pathlib import Path
import hashlib
import json
import os
import re
import signal
import subprocess
import sys
import time

BASE = Path(__file__).resolve().parent
LANE = BASE.parent.parent
ROOT = Path('/workspace/dasl-stdlib-io')
COMPILER = ROOT / 'build/stdlib-entropy-docs-producer-frozen/cinch'
COMPILER_SHA = 'ffb5d39c42d0446a912b6027ebbe88bd825f5c6d91065aeb8a2048526c4999b9'
SOURCE_REVISION = '6fea5bb39f51722a8ee79537b36fab8f20f7caba'
DEPENDENCY_REVISION = 'f30db1451460d4571273637218233f589f984230'
MIB = 1024 * 1024


def digest(data):
    """Hash exact source or executable bytes without running them."""
    return hashlib.sha256(data).hexdigest()


def blob(repository, revision, path):
    """Read a committed source snapshot without changing either checkout."""
    return subprocess.check_output(['git', '-C', str(repository), 'show', revision + ':' + path])


def free_bytes():
    """Observe actual shared-filesystem available space for the current guard."""
    stats = os.statvfs(BASE)
    return stats.f_bavail * stats.f_frsize


def retained_bytes(directory):
    """Count every regular output and scratch payload in this unit."""
    return sum(path.stat().st_size for path in directory.rglob('*') if path.is_file())


def prepare(epoch, module, candidate_revision):
    """Copy exact candidate and nonembedded import closure, retaining builtin authority."""
    directory = BASE / epoch
    directory.mkdir()
    (directory / 'launcher.py').write_bytes(Path(__file__).read_bytes())
    project = directory / 'project'
    project.mkdir()
    manifest = json.loads(blob(ROOT, DEPENDENCY_REVISION, 'compiler/prelude/cinch-base.cinchpkg'))
    embedded = {row['name'] for row in manifest['modules'] if row.get('embedded', False)}
    proposal_names = {path.stem for path in (LANE / 'experiments/crypto-rand-surface-v1/proposed').glob('*.cinch')}
    queue = [module]
    sources = []
    seen = set()
    builtin_imports = set()
    while queue:
        current = queue.pop(0)
        if current in seen:
            continue
        seen.add(current)
        if current in embedded:
            builtin_imports.add(current)
            continue
        if current in proposal_names:
            relative = 'experiments/crypto-rand-surface-v1/proposed/' + current + '.cinch'
            override = BASE / 'candidates-v2' / (current + '.cinch')
            if candidate_revision == 'V2' and override.exists():
                data = override.read_bytes()
                origin = 'candidates-v2/' + override.name
            else:
                selected_revision = SOURCE_REVISION if candidate_revision == 'V2' else candidate_revision
                data = blob(LANE, selected_revision, relative)
                origin = selected_revision
        else:
            relative = 'compiler/prelude/' + current.replace('.', '/') + '.cinch'
            data = blob(ROOT, DEPENDENCY_REVISION, relative)
            origin = DEPENDENCY_REVISION
        destination = project / (current + '.cinch')
        destination.write_bytes(data)
        sources.append({'module': current, 'origin': origin, 'path': relative, 'sha256': digest(data), 'bytes': len(data)})
        queue.extend(re.findall(r'^\s*import\s+([A-Za-z][A-Za-z0-9_.]*)', data.decode(), re.M))
    assert not (project / 'Data.Bytes.cinch').exists()
    assert not (project / 'Data.Core.cinch').exists()
    (directory / 'scratch').mkdir()
    (directory / 'store').mkdir()
    record = {'candidate_module': module, 'candidate_epoch': epoch, 'source_revision': candidate_revision,
              'dependency_revision': DEPENDENCY_REVISION, 'sources': sources,
              'embedded_imports': sorted(builtin_imports), 'builtin_Data_Bytes_authority': True,
              'builtin_Data_Core_authority': True, 'compiler': str(COMPILER), 'compiler_sha256': COMPILER_SHA,
              'scope': 'source check and canonical documentation extraction only; no IR/link/runtime/timing'}
    (directory / 'inputs.json').write_text(json.dumps(record, indent=2) + '\n')
    return directory, project


def run(epoch, module, candidate_revision):
    """Run sequential source/doc children under the granted guards and preserve failures."""
    assert digest(COMPILER.read_bytes()) == COMPILER_SHA
    initial = free_bytes()
    assert initial >= 320 * MIB, ('start guard', initial)
    directory, project = prepare(epoch, module, candidate_revision)
    environment = os.environ.copy()
    controlled = {'LD_LIBRARY_PATH': '/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu',
                  'CINCH_STORE': str(directory / 'store'), 'TMPDIR': str(directory / 'scratch'),
                  'RUST_MIN_STACK': '67108864'}
    environment.update(controlled)
    commands = [
        ['taskset', '-c', '4', str(COMPILER), 'doc', 'extract', str(project / (module + '.cinch')), '--out', str(directory / 'docs.json')],
        ['taskset', '-c', '4', str(COMPILER), 'check', str(project / (module + '.cinch')), '--rebuild'],
    ]
    receipt = {'module': module, 'epoch': epoch, 'environment_overrides': controlled, 'initial_free_bytes': initial,
               'minimum_free_bytes': initial, 'peak_unit_bytes': retained_bytes(directory), 'commands': [],
               'guard_violation': None, 'runtime_executed': False, 'IR_emitted': False,
               'launcher_sha256': digest((directory / 'launcher.py').read_bytes())}
    start = time.monotonic()
    for index, command in enumerate(commands):
        if receipt['guard_violation']:
            break
        stdout = directory / ('command-' + str(index) + '.stdout')
        stderr = directory / ('command-' + str(index) + '.stderr')
        with stdout.open('wb') as out, stderr.open('wb') as err:
            child = subprocess.Popen(command, cwd=project, env=environment, stdout=out, stderr=err, start_new_session=True)
            while child.poll() is None:
                available = free_bytes()
                payload = retained_bytes(directory)
                receipt['minimum_free_bytes'] = min(receipt['minimum_free_bytes'], available)
                receipt['peak_unit_bytes'] = max(receipt['peak_unit_bytes'], payload)
                if available < 250 * MIB:
                    receipt['guard_violation'] = '250 MiB reserve'
                elif payload > 5 * MIB - 16384:
                    receipt['guard_violation'] = '5 MiB fresh output cap, including reserved receipt space'
                elif time.monotonic() - start > 15:
                    receipt['guard_violation'] = '15 second unit wall cap'
                if receipt['guard_violation']:
                    os.killpg(child.pid, signal.SIGKILL)
                    break
                time.sleep(0.025)
            code = child.wait()
        receipt['commands'].append({'argv': command, 'exit_code': code,
                                    'stdout': stdout.name, 'stderr': stderr.name})
    receipt['elapsed_seconds_not_benchmark'] = time.monotonic() - start
    receipt['final_free_bytes'] = free_bytes()
    receipt['final_unit_bytes_before_receipt'] = retained_bytes(directory)
    receipt['minimum_free_bytes'] = min(receipt['minimum_free_bytes'], receipt['final_free_bytes'])
    receipt['peak_unit_bytes'] = max(receipt['peak_unit_bytes'], receipt['final_unit_bytes_before_receipt'])
    if receipt['final_free_bytes'] < 250 * MIB:
        receipt['guard_violation'] = '250 MiB reserve'
    if receipt['final_unit_bytes_before_receipt'] > 5 * MIB - 16384:
        receipt['guard_violation'] = '5 MiB fresh output cap, including reserved receipt space'
    receipt['compiler_sha256_after'] = digest(COMPILER.read_bytes())
    assert receipt['compiler_sha256_after'] == COMPILER_SHA
    docs_path = directory / 'docs.json'
    if docs_path.exists():
        docs = json.loads(docs_path.read_text())
        receipt['doc_top_level_keys'] = sorted(docs)
        receipt['doc_diagnostics'] = docs.get('diagnostics')
        receipt['doc_orphans'] = docs.get('orphans')
    receipt['source_passed'] = len(receipt['commands']) == 2 and receipt['commands'][1]['exit_code'] == 0 and not receipt['guard_violation']
    (directory / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt, indent=2))


if __name__ == '__main__':
    if len(sys.argv) not in (3, 4):
        raise SystemExit('usage: check_unit.py EPOCH MODULE [V2]')
    run(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) == 4 else SOURCE_REVISION)
