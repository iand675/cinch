#!/usr/bin/env python3
"""Check one isolated built-in scanner candidate with a strict source-only time/output budget."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import resource
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SNAPSHOT = Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen')
INTERFACES = Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen/base-interfaces.json')


def digest(path):
    """Keep exact source and tool identities alongside completed or rejected source checks."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def host_stack():
    """Keep the compiler-host stack separate from the candidate's later fixed consumer stack."""
    resource.setrlimit(resource.RLIMIT_STACK, (64*1024*1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))


def main():
    """Freeze only source modules and stop on the first actual checker result or the standing fifteen-second deadline."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--source', type=Path, default=HERE/'Main.cinch')
    parser.add_argument('--frontend', choices=['rust', 'native'], default='rust')
    parser.add_argument('--native-snapshot', type=Path, default=Path('/workspace/dasl-stdlib-mime/build/mime/2f077-v2-provisional-snapshot'))
    parser.add_argument('--deadline', type=int, default=15, help='Explicitly granted source-only deadline, at most45 seconds')
    args = parser.parse_args()
    assert 1 <= args.deadline <= 45
    os.sched_setaffinity(0, {2})
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    project = artifacts/'project'
    project.mkdir(exist_ok=True)
    source = project/'Main.cinch'
    source.write_bytes(args.source.read_bytes())
    snapshot = SNAPSHOT if args.frontend == 'rust' else args.native_snapshot.resolve()
    interfaces = INTERFACES if args.frontend == 'rust' else snapshot/'base-interfaces.json'
    embedded = {row[0] for row in json.loads(interfaces.read_text())['interfaces']}
    overrides = {path.name[:-6]: path for path in (HERE/'sources').glob('*.cinch')}
    pending = re.findall(r'^\s*import\s+([\w.]+)', source.read_text(), re.M)
    found, paths = set(), [source]
    while pending:
        name = pending.pop()
        if name in found or name in embedded:
            continue
        found.add(name)
        original = overrides.get(name, ROOT/'compiler/prelude'/(name.replace('.', '/')+'.cinch'))
        copied = project/(name+'.cinch')
        copied.write_bytes(original.read_bytes())
        paths.append(copied)
        pending.extend(re.findall(r'^\s*import\s+([\w.]+)', copied.read_text(), re.M))
    provenance_path = snapshot/'provenance.json'
    provenance = json.loads(provenance_path.read_text())
    compiler, runtime = snapshot/'cinch', snapshot/'libcinchruntime.a'
    if 'files' in provenance:
        for name in ['cinch', 'libcinchruntime.a', 'base-interfaces.json', 'objects.json']:
            assert digest(snapshot/name) == provenance['files'][name]['sha256']
    else:
        assert digest(compiler) == provenance['compiler_sha256'] and digest(runtime) == provenance['runtime_sha256']

    def limits():
        """Bound host stack and output without changing source refinements or consumer limits."""
        host_stack()
        resource.setrlimit(resource.RLIMIT_FSIZE, (5*1024*1024, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))

    prefix = artifacts/'check-output'
    if args.frontend == 'rust':
        command = [str(compiler), 'check', str(source)]
    else:
        bundle = artifacts/'bundle.cinch'
        bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
        command = [str(compiler), '--check', '--interfaces', str(interfaces), str(bundle), str(prefix)]
    before = time.monotonic()
    timeout = False
    try:
        result = subprocess.run(command, capture_output=True, text=True, preexec_fn=limits, timeout=args.deadline,
                                env={**os.environ, 'CINCH_HEAP_LIMIT': '2G', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'})
        code, stdout, stderr = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        timeout = True
        code = None
        stdout, stderr = error.stdout or b'', error.stderr or b''
        stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
    (artifacts/'check.stdout').write_text(stdout)
    (artifacts/'check.stderr').write_text(stderr)
    diagnostic = prefix.with_name(prefix.name+'.diagnostics')
    report = {'scope': 'Isolated candidate source/type/proof only; no runtime or performance result.',
              'frontend': args.frontend, 'expected_effect_refusal': False,
              'command': command, 'compiler_sha256': digest(compiler), 'runtime_sha256': digest(runtime),
              'provenance_sha256': digest(provenance_path), 'provenance': provenance,
              'interfaces_sha256': digest(interfaces), 'sources': {path.name: digest(path) for path in paths},
              'candidate_manifest_sha256': digest(HERE/'candidate.json'),
              'returncode': code, 'timeout_without_verdict': timeout, 'deadline_seconds': args.deadline, 'seconds': time.monotonic()-before,
              'stdout': stdout, 'stderr': stderr, 'diagnostic': diagnostic.read_text() if diagnostic.exists() else '', 'compiler_host_stack_mib': 64, 'cpu': 2}
    (artifacts/'receipt.json').write_text(json.dumps(report, indent=2)+'\n')
    assert sum(path.stat().st_size for path in artifacts.rglob('*') if path.is_file()) <= 5*1024*1024
    print(stderr or stdout or ('Source deadline reached without a verdict' if timeout else 'Candidate source/type/proof passed'))
    raise SystemExit(124 if timeout else code)


if __name__ == '__main__':
    main()
