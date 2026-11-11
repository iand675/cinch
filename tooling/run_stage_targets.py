#!/usr/bin/env python3
"""Run compiler test targets through a staged Cinch frontend, in parallel.

usage: run_stage_targets.py --stage N --out DIR [--jobs 4] [--all | target...]

Each target's source checks, builds and runs go to build/bootstrap/stage<N>/cinch
(or $CINCH_BOOTSTRAP_BUILD) through the CINCH_TEST_COMPILER selector, linked
against the pinned library object and runtime. --all selects every test target
that uses the shared selectors. Writes one log per target and DIR/summary.json.
This is a development runner; tooling/verify_stage3.py is the audited gate.
"""
import argparse, json, os, re, subprocess, sys, concurrent.futures as cf
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tooling'))
TREE = ROOT
import bootstrap  # noqa
from verify_stage3 import write_wrapper  # noqa


def setup(stage, out, tree):
    compiler = bootstrap.BUILD / f'stage{stage}' / 'cinch'
    provider = bootstrap.runtime_provider()
    manifest = out / 'base-interfaces.json'
    manifest.write_bytes(json.dumps(provider['interfaces'], sort_keys=True, separators=(',', ':')).encode())
    objects = out / 'objects.json'
    objects.write_bytes(json.dumps(provider['configuration'], sort_keys=True, separators=(',', ':')).encode())
    wrapper = out / 'frontend.py'
    write_wrapper(wrapper, compiler, objects, provider['runtime'])
    env = {k: v for k, v in bootstrap.environment().items() if not k.startswith('CINCH_TEST_')}
    env.update(CINCH_TEST_COMPILER=str(wrapper), CINCH_TEST_INTERFACES=str(manifest),
               CINCH_TEST_OBJECTS=str(objects), CINCH_TEST_RUNTIME_ARCHIVE=str(provider['runtime']),
               CINCH_TEST_LIBRARY_OBJECT=str(provider['object']),
               CINCH_TEST_SOURCE_ROOT=str(tree / 'compiler'), PYTHONDONTWRITEBYTECODE='1')
    return env


def run(target, env, out, cargo_target):
    import build_slots

    with build_slots.slot(owner=f"target {target}"):
        return run_in_slot(target, env, out, cargo_target)


def run_in_slot(target, env, out, cargo_target):
    fixtures = out / 'fixtures' / target
    fixtures.mkdir(parents=True, exist_ok=True)
    log = out / f'{target}.log'
    e = dict(env, TMPDIR=str(fixtures), CINCH_TEST_API_EMIT_INTERFACES='1' if target.startswith('interface_') else '0')
    cmd = ['nice', 'cargo', 'test', '--target-dir', str(cargo_target), '--test', target, '--',
           '--include-ignored', '--test-threads=1']
    with log.open('wb') as f:
        code = subprocess.run(cmd, cwd=TREE / 'compiler', env=e, stdout=f, stderr=subprocess.STDOUT).returncode
    text = log.read_text(errors='replace')
    passed = re.findall(r'^test (\S+) \.\.\. ok$', text, re.M)
    failed = re.findall(r'^test (\S+) \.\.\. FAILED$', text, re.M)
    return dict(target=target, exit=code, passed=len(passed), failed=failed)


TOOLING_TARGETS = {'lsp', 'jit', 'repl', 'package_store', 'package_manifest', 'package_link', 'incremental'}


def all_targets(tree):
    tests = tree / 'compiler' / 'tests'
    return sorted(p.stem for p in tests.glob('*.rs')
                  if 'mod common;' in p.read_text() and p.stem not in TOOLING_TARGETS)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--stage', type=int, default=3)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--jobs', type=int, default=4)
    p.add_argument('--cargo-target', type=Path, default=None)
    p.add_argument('--tree', type=Path, default=ROOT)
    p.add_argument('--all', action='store_true')
    p.add_argument('targets', nargs='*')
    a = p.parse_args()
    if a.all:
        a.targets = all_targets(a.tree)
    if not a.targets:
        p.error('name targets or pass --all')
    a.out.mkdir(parents=True, exist_ok=True)
    a.cargo_target = a.cargo_target or a.tree / 'compiler' / 'target'
    env = setup(a.stage, a.out.resolve(), a.tree)
    global TREE; TREE = a.tree
    # Build every test binary once so parallel runs do not race the cargo lock.
    subprocess.run(['nice', 'cargo', 'test', '--target-dir', str(a.cargo_target), '--no-run',
                    *sum([['--test', t] for t in a.targets], [])], cwd=TREE / 'compiler', check=True,
                   stdout=subprocess.DEVNULL)
    rows = []
    with cf.ThreadPoolExecutor(a.jobs) as pool:
        for row in pool.map(lambda t: run(t, env, a.out.resolve(), a.cargo_target), a.targets):
            rows.append(row)
            print(f"{row['target']}: exit={row['exit']} passed={row['passed']} failed={len(row['failed'])}", flush=True)
    (a.out / 'summary.json').write_text(json.dumps(rows, indent=1))
    print('targets passing:', sum(r['exit'] == 0 for r in rows), '/', len(rows))


if __name__ == '__main__':
    main()
