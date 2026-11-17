#!/usr/bin/env python3
"""Freeze and source-check only the tabwriter candidate closure on the explicitly qualified Region compiler."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import resource
import shutil
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SNAPSHOT = Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen')
COMPILER = '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
RUNTIME = 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
SOURCE = '75b4bab70d18ed3dd66bf6b8c5153943764dabf3'
MIB = 1024 * 1024


def digest(path):
    """Hash exact source and tool bytes without changing their independently frozen identities."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Preserve the first diagnostic or timeout as a separate source-only epoch, with no runtime claim."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--main', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--deadline', type=int, default=15)
    parser.add_argument('--start-free-mib', type=int, choices=(320, 350), default=350)
    parser.add_argument('--resource-policy-note', help='Exact Root authorization for a scope-specific source-only start threshold')
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    if args.start_free_mib != 350:
        assert args.resource_policy_note, 'lower source-only start requires the explicit reviewed scope'
    assert shutil.disk_usage(ROOT).free >= args.start_free_mib * MIB, 'fresh source-only start guard; no output created'
    work = args.work.resolve()
    assert not work.exists(), 'never overwrite a source epoch'
    compiler, runtime = SNAPSHOT / 'cinch', SNAPSHOT / 'libcinchruntime.a'
    assert digest(compiler) == COMPILER and digest(runtime) == RUNTIME
    family = ROOT / 'experiments/buffered-region-v1/embedded-family.cinchpkg'
    embedded = {row['name'] for row in json.loads(family.read_text())['modules']}
    assert len(embedded) == 66
    found = {}

    def visit(path):
        """Resolve candidate sources before ordinary distributed modules and never shadow embedded providers."""
        for name in re.findall(r'^\s*import\s+([A-Z][\w.]*)', path.read_text(), re.M):
            if name in embedded or name in found:
                continue
            candidate = HERE / 'sources' / (name + '.cinch')
            dependency = HERE / 'dependencies' / (name + '.cinch')
            selected = candidate if candidate.exists() else dependency if dependency.exists() else ROOT / 'compiler/prelude' / (name.replace('.', '/') + '.cinch')
            if selected == dependency:
                binding = json.loads((HERE / 'dependencies/manifest.json').read_text())
                assert name == binding['module'] and digest(dependency) == binding['sha256']
                assert binding['compiler_sha256'] == COMPILER and binding['runtime_sha256'] == RUNTIME
                qualified = Path(binding['qualified_source_report'])
                assert digest(qualified) == binding['qualified_source_report_sha256']
                assert json.loads(qualified.read_text())['source_gate_passed']
            assert selected.exists(), name
            found[name] = selected
            visit(selected)

    entry = args.main.resolve()
    visit(entry)
    work.mkdir(parents=True)
    shutil.copyfile(entry, work / 'Main.cinch')
    shutil.copyfile(family, work / 'embedded-family.cinchpkg')
    for name, path in found.items():
        shutil.copyfile(path, work / (name + '.cinch'))
    command = [str(compiler), 'check', str(work / 'Main.cinch')]
    report = {'scope': __doc__, 'compiler_sha256': COMPILER, 'runtime_sha256': RUNTIME,
              'compiler_source': SOURCE, 'provenance_sha256': digest(SNAPSHOT / 'provenance.json'),
              'main_sha256': digest(work / 'Main.cinch'), 'embedded_family_sha256': digest(family),
              'sources': {name: {'path': str(path), 'sha256': digest(path)} for name, path in found.items()},
              'command': command, 'source_only': True, 'production_adopted': False, 'runtime_claim': False,
              'cpu': 2, 'start_guard_bytes': args.start_free_mib * MIB, 'reserve_bytes': 250 * MIB, 'cap_bytes': 5 * MIB,
              'explicit_resource_policy': args.resource_policy_note}

    def limits():
        """Keep compiler-host stack and individual output files bounded independently of future consumer profiles."""
        resource.setrlimit(resource.RLIMIT_STACK, (64 * MIB, resource.getrlimit(resource.RLIMIT_STACK)[1]))
        resource.setrlimit(resource.RLIMIT_FSIZE, (5 * MIB, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))

    started = time.monotonic()
    reason = None
    with (work / 'check.stdout').open('wb') as out, (work / 'check.stderr').open('wb') as err:
        child = subprocess.Popen(command, cwd=work, stdout=out, stderr=err, preexec_fn=limits, start_new_session=True,
                                 env={**os.environ, 'CINCH_STACK_SIZE': '64M', 'RUST_MIN_STACK': '67108864'})
        while child.poll() is None:
            if time.monotonic() - started > args.deadline:
                reason = 'deadline without verdict'
            elif shutil.disk_usage(work).free < 250 * MIB:
                reason = '250MiB global pause reserve'
            elif sum(p.stat().st_size for p in work.rglob('*') if p.is_file()) > 5 * MIB:
                reason = '5MiB source-only output cap'
            if reason:
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                break
            time.sleep(0.1)
        code = child.wait()
    unchanged = digest(entry) == report['main_sha256'] and all(digest(path) == report['sources'][name]['sha256'] for name, path in found.items())
    report.update(returncode=code, interrupted_reason=reason, seconds=time.monotonic() - started,
                  source_gate_passed=code == 0 and reason is None,
                  original_main_and_library_sources_unchanged=unchanged,
                  stdout_sha256=digest(work / 'check.stdout'), stderr=(work / 'check.stderr').read_text(),
                  artifact_bytes=sum(p.stat().st_size for p in work.rglob('*') if p.is_file()),
                  genuine_interfaces={str(p.relative_to(work)): digest(p) for p in work.rglob('*.cinchif')})
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    assert unchanged, 'exact original main/library identity changed during the isolated check'
    print(json.dumps({key: report[key] for key in ('source_gate_passed', 'seconds', 'artifact_bytes', 'stderr')}), flush=True)
    raise SystemExit(124 if reason else code)


if __name__ == '__main__':
    main()
