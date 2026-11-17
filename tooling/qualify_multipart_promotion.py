#!/usr/bin/env python3
"""Run the isolated combined candidate's source, metadata and live Rust regression.

The selected compiler and runtime are explicit inputs. CI initializes its host
archive through an ordinary compiler build; frozen standalone gates retain
their exact immutable archive instead. No SDK, Native fallback or production
source substitution is used.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
from check_compare import digest, initialize_runtime


#: Own repository root; all candidate and fixture inputs remain source controlled.
ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Execute each independently recorded boundary before permitting the next one."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--initialize-rust-runtime', action='store_true')
    parser.add_argument('--cpu', type=int)
    args = parser.parse_args()
    cpu = args.cpu if args.cpu is not None else min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    report = {'scope': __doc__, 'compiler_sha256': digest(compiler), 'production_adopted': False,
              'compiler_host_stack_bytes': 67108864, 'cpu_affinity': [cpu], 'phases': []}
    if args.initialize_rust_runtime:
        initialization = work / 'runtime-initialization'; initialization.mkdir(exist_ok=True)
        runtime, report['runtime_initialization'] = initialize_runtime(compiler, runtime, initialization)
    report['runtime_sha256'] = digest(runtime)
    candidate = ROOT / 'experiments/multipart-promotion-v1'
    source_work, metadata_work = work / 'source', work / 'metadata'
    commands = [
        ('source', [sys.executable, str(ROOT / 'tooling/check_multipart_live_effects.py'),
                    '--work', str(source_work), '--rust', str(compiler), '--rust-runtime', str(runtime),
                    '--effect-source', str(candidate / 'sources/Effect.Multipart.cinch'),
                    '--text-source', str(candidate / 'sources/Text.Multipart.cinch'),
                    '--retain-interfaces', '--source-cap-mib', '6', '--cpu', str(cpu)]),
        ('metadata', [sys.executable, str(ROOT / 'tooling/inspect_multipart_carriers.py'),
                      '--source-work', str(source_work), '--candidate', str(candidate),
                      '--compiler', str(compiler), '--work', str(metadata_work), '--cpu', str(cpu)]),
        ('live-runtime', [sys.executable, str(ROOT / 'tooling/run_multipart_live_effects.py'),
                          '--source-work', str(source_work), '--compiler', str(compiler),
                          '--runtime', str(runtime), '--cpu', str(cpu)]),
    ]
    for name, command in commands:
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=300,
                                env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report['phases'].append({'name': name, 'command': command, 'returncode': result.returncode,
                                 'stdout': result.stdout, 'stderr': result.stderr})
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if result.returncode:
            raise RuntimeError(name + ' failed:\n' + result.stdout + result.stderr)
    print('isolated multipart source, defining metadata and live runtime passed', flush=True)


if __name__ == '__main__':
    main()
