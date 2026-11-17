#!/usr/bin/env python3
"""Qualify bounded snapshot, callback-failure, deep-stack and genuine mixed-effect controls on one frozen compiler tuple."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT / 'tooling'))
from bench_stdlib import source_closure


def digest(path: Path) -> str:
    """Record exact source and binary identities before starting any compiler phase."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Stop at the first source or runtime diagnostic while retaining every failed artifact plain."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--opt-level', choices=['0', '2'], default='0')
    parser.add_argument('--source-only', action='store_true')
    parser.add_argument('--artifacts', type=Path, required=True)
    args = parser.parse_args()
    work = args.artifacts.resolve()
    report = work.with_suffix('.json')
    assert work.parent == ROOT / 'build' and work.name.startswith('suffixarray-v1-runtime-controls-')
    assert not work.exists() and not report.exists()
    before = shutil.disk_usage(ROOT).free
    assert before >= 350 * 1024 * 1024
    assert digest(args.compiler) == 'c2ae24e64f7999060dce8610618f316710f6300ca29e89b9335056936344a86e'
    assert digest(args.runtime) == 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
    work.mkdir()
    entry = work / 'Main.cinch'
    entry.write_bytes((HERE / 'consumer-controls.cinch').read_bytes())
    dependencies = source_closure(entry.read_text(), {'Data.Index.SuffixArray': HERE / 'src/Data.Index.SuffixArray.cinch'})
    for name, path in dependencies.items():
        shutil.copyfile(path, work / (name + '.cinch'))
    executable = work / 'consumer'
    r = dict(scope='Independent snapshot/alias, typed callback and IO+Mut controls; no performance claim',
             status='checking', source_sha256=digest(HERE / 'src/Data.Index.SuffixArray.cinch'),
             entry_sha256=digest(entry), compiler_sha256=digest(args.compiler), runtime_sha256=digest(args.runtime),
             dependencies={n: digest(p) for n, p in dependencies.items()}, opt_level=args.opt_level,
             compiled_rc_debug=False, profiles=[], steps=[], before_free_bytes=before,
             compiler_main_thread_stack_bytes=67108864, consumer_stack_bytes=8388608)
    report.write_text(json.dumps(r, indent=2) + '\n')
    started = time.monotonic()
    try:
        commands = [[str(args.compiler), 'check', str(entry)]]
        if not args.source_only:
            commands.append([str(args.compiler), 'build', str(entry), '-o', str(executable),
                             '--opt-level', args.opt_level, '--runtime-archive', str(args.runtime)])
        for number, command in enumerate(commands):
            assert shutil.disk_usage(ROOT).free >= 250 * 1024 * 1024
            result = subprocess.run(command, capture_output=True, text=True, timeout=120)
            (work / f'compile-{number}.log').write_text(result.stdout + result.stderr)
            r['steps'].append(dict(command=command, exit_code=result.returncode))
            assert result.returncode == 0, result.stdout + result.stderr
        if not args.source_only:
            r.update(executable_sha256=digest(executable), llvm_sha256=digest(work / 'consumer.ll'), status='running')
            report.write_text(json.dumps(r, indent=2) + '\n')
            for poison in [False, True]:
                for mode in ['aliases', 'callbacks', 'cross', 'deep', 'effects', 'suppression']:
                    env = {**os.environ, 'CINCH_RC_DEBUG': str(int(poison)),
                           'CINCH_HEAP_LIMIT': '256M' if poison else '32M',
                           'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1'}
                    result = subprocess.run([str(executable), mode], env=env, capture_output=True, text=True, timeout=60)
                    (work / f'run-{poison}-{mode}.stdout').write_text(result.stdout)
                    (work / f'run-{poison}-{mode}.stderr').write_text(result.stderr)
                    expected = 'c c c c ' if mode == 'effects' else ('s s s ' if mode == 'suppression' else '')
                    assert result.returncode == 0 and result.stdout == expected and 'leaked=0' in result.stderr, (mode, result.returncode, result.stdout, result.stderr)
                    r['profiles'].append(dict(poison=poison, mode=mode, stdout=result.stdout, zero_leaks=True))
                    report.write_text(json.dumps(r, indent=2) + '\n')
        r['status'] = 'source-passed' if args.source_only else 'passed'
    except Exception as error:
        r.update(status='failed', diagnostics=repr(error))
        raise
    finally:
        r.update(seconds=time.monotonic() - started, after_free_bytes=shutil.disk_usage(ROOT).free,
                 own_output_bytes=sum(p.stat().st_size for p in work.rglob('*') if p.is_file()))
        report.write_text(json.dumps(r, indent=2) + '\n')
        print(json.dumps(r))
    assert r['own_output_bytes'] <= 30 * 1024 * 1024 and r['after_free_bytes'] >= 250 * 1024 * 1024


if __name__ == '__main__':
    main()
