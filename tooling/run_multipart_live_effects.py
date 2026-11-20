#!/usr/bin/env python3
"""Execute the already source-checked live both-effect fixture on one exact Rust tuple.

No source or runtime is substituted. Start/pause guards bound one serial
consumer; accepted outputs are archived only after both memory profiles pass.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
from check_compare import digest, run
from multipart_qualification_support import archive_completed, artifact_digest, build_fixture


def main() -> None:
    """Verify fresh source admission, exact immutable inputs and guarded staged execution."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-work', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.source_work.resolve()
    gate_path = work / 'report.json'; gate = json.loads(gate_path.read_text())
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    if not any(row['frontend'] == 'rust' and row['returncode'] == 0 and row['compiler_sha256'] == digest(compiler) for row in gate['checks']):
        raise RuntimeError('fresh source gate does not admit this exact compiler')
    if gate.get('future_rust_runtime') != {'path': str(runtime), 'sha256': digest(runtime)}:
        raise RuntimeError('runtime differs from the explicitly recorded source tuple')
    if digest(work / 'Main.cinch') != gate['source_sha256'] or digest(work / 'bundle.source') != gate['bundle_sha256']:
        raise RuntimeError('changed live fixture input')
    for name, expected in gate['sources'].items():
        if digest(work / (name + '.cinch')) != expected:
            raise RuntimeError('changed frozen candidate/dependency: ' + name)
    path = work / 'runtime-rust.json'
    report = json.loads(path.read_text()) if path.exists() else {
        'scope': __doc__, 'source_gate_sha256': digest(gate_path), 'source_gate_path': str(gate_path),
        'compiler_sha256': digest(compiler), 'runtime_sha256': digest(runtime), 'runtime': str(runtime),
        'compiler_host_stack_bytes': 67108864, 'consumer_optimization': 2, 'compiled_rc_debug': False,
        'production_adopted': False, 'cpu_affinity': [args.cpu], 'runs': [], 'phases': [],
        'initial_artifact_bytes': sum(file.stat().st_size for file in work.rglob('*') if file.is_file()),
        'new_output_cap_bytes': 20 * 1024 * 1024}
    if report['source_gate_sha256'] != digest(gate_path) or report['compiler_sha256'] != digest(compiler) or report['runtime_sha256'] != digest(runtime):
        raise RuntimeError('changed execution epoch')

    def save() -> None:
        """Retain each completed boundary before another artifact or process is started."""
        path.write_text(json.dumps(report, indent=2) + '\n')

    def guard(minimum: int, phase: str) -> None:
        """Pause before further work when the coordinated workspace reserve is unavailable."""
        free = shutil.disk_usage(work).free
        growth = sum(file.stat().st_size for file in work.rglob('*') if file.is_file()) - report['initial_artifact_bytes']
        report['phases'].append({'phase': phase, 'free_bytes': free, 'minimum_bytes': minimum, 'new_artifact_bytes': growth})
        save()
        if free < minimum:
            raise RuntimeError('resource pause before ' + phase + '; no compiler or runtime started')
        if growth > report['new_output_cap_bytes']:
            raise RuntimeError('new-output cap reached before ' + phase + '; preserve current boundary')

    guard(350 * 1024 * 1024 if not report['runs'] else 250 * 1024 * 1024, 'start-or-resume')
    clang = os.environ.get('CINCH_CLANG', 'clang')
    if 'fixture_object_sha256' not in report:
        report.update(build_fixture(work, clang)); save()
    elif digest(work / 'fixture.o') != report['fixture_object_sha256'] or digest(work / 'kernels.o') != report['kernel_object_sha256']:
        raise RuntimeError('changed actual foreign object')
    ir, executable = work / 'rust-live-effects.ll', work / 'rust-live-effects'
    if not report['runs']:
        guard(250 * 1024 * 1024, 'emit')
        command = [str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(ir)]
        emitted = run(command, work, 600, {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        triple = re.search(r'target triple = "([^"]+)"', ir.read_text()).group(1)
        report['runs'].append({'frontend': 'rust', 'case': 'live-effects', 'source_sha256': gate['source_sha256'],
                               'command': command, 'stdout': emitted.stdout, 'stderr': emitted.stderr,
                               'target': triple, 'ir_sha256': digest(ir), 'profiles': []}); save()
    row = report['runs'][0]
    if 'executable_sha256' not in row:
        if digest(ir) != row['ir_sha256']:
            raise RuntimeError('changed checked LLVM')
        guard(250 * 1024 * 1024, 'link')
        link = [clang, '-O2', '--target=' + row['target'], str(ir), str(work / 'fixture.o'), str(work / 'kernels.o'),
                str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(executable)]
        linked = run(link, work, 120)
        row.update(link_command=link, link_stdout=linked.stdout, link_stderr=linked.stderr, executable_sha256=digest(executable)); save()
    for poison, heap in [('0', '32M'), ('1', '256M')]:
        if any(profile['poison'] == poison for profile in row['profiles']):
            continue
        guard(250 * 1024 * 1024, 'profile-' + poison)
        if artifact_digest(executable) != row['executable_sha256']:
            raise RuntimeError('changed executable')
        observed = run([str(executable)], work, 60, {**os.environ, 'CINCH_RC_DEBUG': poison,
                       'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1'})
        if 'leaked=0' not in observed.stderr:
            raise RuntimeError('missing zero-leak observation')
        row['profiles'].append({'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': observed.stdout, 'stderr': observed.stderr}); save()
    archive_completed(work, 'rust', 'live-effects', report)
    report['runtime_claim'] = 'This exact candidate/fixture and seed/archive passed both profiles; production remains unchanged.'
    save(); print('live both-effect runtime normal/poison passed and archived', flush=True)


if __name__ == '__main__':
    main()
