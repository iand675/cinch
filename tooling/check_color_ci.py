#!/usr/bin/env python3
"""Initialize the current Rust host runtime normally, then run the SDK-free color unit."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
from check_compare import digest, initialize_runtime


#: Repository root for the maintained experiment and runtime consumer.
ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Keep provider initialization separate from the guarded functional-output unit."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(work).free < 350*1024*1024:
        raise RuntimeError('350 MiB initial current-runtime CI guard')
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    initializer = work / 'runtime-initialization'
    initializer.mkdir(exist_ok=False)
    report = {'scope': __doc__, 'compiler_sha256': digest(args.compiler),
              'provider_initialization_scope': 'Ordinary compiler build plus independently locked archive snapshot, retained outside the30MiB functional-output unit.',
              'compiler_host_stack_bytes': 67108864, 'runner_sha256': digest(Path(__file__))}
    try:
        archive, metadata = initialize_runtime(args.compiler.resolve(), args.runtime.resolve(), initializer)
        report['runtime_initialization'] = metadata
        cpu = min(os.sched_getaffinity(0))
        command = ['python3', str(ROOT / 'tooling/check_color.py'),
                   '--source', str(ROOT / 'seed/prelude/Image/Color.cinch'),
                   '--compiler', str(args.compiler.resolve()), '--runtime', str(archive),
                   '--work', str(work / 'functional'), '--cpu', str(cpu), '--opt', '2']
        result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, timeout=120,
                                env={**os.environ, 'CARGO_BUILD_JOBS': '1', 'RUST_TEST_THREADS': '1', 'RUST_MIN_STACK': '67108864'})
        report.update(command=command, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        if result.returncode:
            raise RuntimeError('current Rust color unit failed: '+result.stdout+'\n'+result.stderr)
        functional = json.loads((work / 'functional/report.json').read_text())
        if functional['runtime_sha256'] != metadata['archive_sha256'] or digest(archive) != metadata['snapshot_sha256']:
            raise RuntimeError('ordinary current host runtime snapshot changed')
        report['functional'] = functional
        report['status'] = 'passed'
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')


if __name__ == '__main__':
    main()
