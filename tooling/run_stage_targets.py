#!/usr/bin/env python3
"""Run compiler test targets through a staged Cinch frontend, in parallel.

usage: run_stage_targets.py --stage N --out DIR [--jobs 4] [--all | target...]

Each target's source checks, builds and runs go to build/bootstrap/stage<N>/cinch
(or $CINCH_BOOTSTRAP_BUILD) through the CINCH_TEST_COMPILER selector, linked
against the pinned library object and runtime. --all selects every test target
that uses the shared selectors. Writes one log per target and DIR/summary.json.
--all-integration includes every Cargo integration target, even those exercising
the Rust tooling directly. Its summary records actual selected frontend calls.
This is a development runner; tooling/verify_stage3.py is the audited gate.
"""
import argparse, fcntl, json, os, re, subprocess, sys, tempfile, concurrent.futures as cf
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tooling'))
TREE = ROOT
import bootstrap  # noqa
from verify_stage3 import standalone_inputs, supports_check_types, write_wrapper  # noqa


def selected_stage_inputs(stage, directory, compiler):
    if stage < 2:
        # The seed and the compiler built by the seed predate object receipts.
        provider = bootstrap.runtime_provider()
        encode = lambda value: json.dumps(value, sort_keys=True, separators=(',', ':')).encode()
        package = provider["interface_package"]
        config = {**provider["configuration"], "metadata": str(package.resolve()),
                  "metadata_sha256": bootstrap.file_hash(package)}
        return package.read_bytes(), encode(config), provider['object'], provider['runtime']

    receipt = bootstrap.read_json(directory / 'stage.json')
    if receipt.get('status') != 'built':
        raise bootstrap.BootstrapError(f'missing build receipt for stage {stage}')
    inputs = receipt['inputs']
    manifest, objects = directory / 'base.cpk', directory / 'base-objects.json'
    config = bootstrap.read_json(objects)
    if not all(key in config for key in ('object', 'metadata', 'object_sha256', 'metadata_sha256')):
        raise bootstrap.BootstrapError(f'incomplete object configuration for stage {stage}')
    library, metadata = Path(config['object']), Path(config['metadata'])
    runtime = library.with_name('libcinchruntime.a')
    for path, digest in ((compiler, receipt['executable_sha256']),
                         (manifest, inputs['interface_set_sha256']), (objects, inputs['object_configuration_sha256']),
                         (library, config['object_sha256']), (metadata, config['metadata_sha256']),
                         (runtime, inputs['runtime_archive_sha256'])):
        if not path.is_file() or bootstrap.file_hash(path) != digest:
            raise bootstrap.BootstrapError(f'stage {stage} input differs from its receipt: {path}')
    return manifest.read_bytes(), objects.read_bytes(), library, runtime


def setup(stage, out, tree, compiler=None):
    directory = bootstrap.BUILD / f'stage{stage}'
    if compiler is None:
        compiler = directory / 'cinch'
        manifest_bytes, object_bytes, library, runtime = selected_stage_inputs(stage, directory, compiler)
    else:
        # A compiler built outside the chain (tooling/module_build.py) has no
        # stage receipt. It targets the pinned library like every stage, so
        # it runs against the pinned provider's interfaces and object.
        manifest_bytes, object_bytes, library, runtime = selected_stage_inputs(0, directory, compiler)
    manifest = out / 'base.cpk'
    manifest.write_bytes(manifest_bytes)
    objects = out / 'base-objects.json'
    objects.write_bytes(object_bytes)
    wrapper = out / 'frontend.py'
    write_wrapper(wrapper, compiler, objects, runtime)
    env = {k: v for k, v in bootstrap.environment().items() if not k.startswith('CINCH_TEST_')}
    env.update(CINCH_TEST_STANDALONE=standalone_inputs(out / "standalone", compiler, manifest, objects, runtime),
               CINCH_TEST_CHECK_TYPES="1" if supports_check_types(compiler) else "0",
               CINCH_TEST_COMPILER=str(wrapper), CINCH_TEST_TARGET_INPUTS=str(wrapper), CINCH_TEST_INTERFACES=str(manifest),
               CINCH_TEST_OBJECTS=str(objects), CINCH_TEST_RUNTIME_ARCHIVE=str(runtime),
               CINCH_TEST_LIBRARY_OBJECT=str(library),
               CINCH_TEST_SOURCE_ROOT=str(tree / 'seed'), PYTHONDONTWRITEBYTECODE='1')
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
    cmd = ['cargo', 'test', '--target-dir', str(cargo_target), '--test', target, '--',
           '--include-ignored', '--test-threads=1']
    with log.open('wb') as f:
        code = subprocess.run(cmd, cwd=TREE / 'seed', env=e, stdout=f, stderr=subprocess.STDOUT).returncode
    text = log.read_text(errors='replace')
    # Child output can split a test's status line. Cargo's final libtest
    # summary still counts that test, so prefer it over per-test lines.
    summaries = re.findall(r'^test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;', text, re.M)
    failed = re.findall(r'^test (\S+) \.\.\. FAILED$', text, re.M)
    if summaries:
        passed, failed_count = map(int, summaries[-1])
    else:
        passed = len(re.findall(r'^test (\S+) \.\.\. ok$', text, re.M))
        failed_count = len(failed)
    return dict(target=target, exit=code, passed=passed, failed=failed, failed_count=failed_count)


def prebuild(targets, env, out, cargo_target):
    """Build every selected test executable in one Cargo invocation.

    Each worker runs `cargo test --test <target>`, and Cargo holds one build
    lock per target directory, so without this the executables are built one
    at a time by whichever worker holds the lock while the others wait: 345
    serial crate builds on however many cores the machine has. One
    invocation naming every target lets Cargo's own job pool build them side
    by side, and each worker's build step is then a fingerprint check.

    A target that does not compile is not this step's to report: the
    worker's own `cargo test` fails on it and its log says why.
    """
    command = ['cargo', 'test', '--no-run', '--target-dir', str(cargo_target)]
    for target in targets:
        command += ['--test', target]
    with (out / 'prebuild.log').open('wb') as log:
        subprocess.run(command, cwd=TREE / 'seed', env=env, stdout=log, stderr=subprocess.STDOUT)


TOOLING_TARGETS = {'lsp', 'jit', 'repl', 'package_store', 'package_manifest', 'package_link', 'incremental'}


def all_targets(tree):
    tests = tree / 'seed' / 'tests'
    targets = []
    for path in tests.glob('*.rs'):
        if path.stem not in TOOLING_TARGETS:
            source = path.read_text()
            if 'mod common;' in source or 'mod corpus_frontend;' in source:
                targets.append(path.stem)
    return sorted(targets)


def integration_targets(tree):
    metadata = json.loads(subprocess.check_output(
        ['cargo', 'metadata', '--locked', '--format-version', '1', '--no-deps'],
        cwd=tree / 'seed', env=bootstrap.environment()))
    package = next(pkg for pkg in metadata['packages'] if pkg['name'] == 'cinch')
    return sorted(target['name'] for target in package['targets'] if 'test' in target['kind'])


def run_with_receipts(target, env, out, cargo_target, receipts):
    directory = receipts / target
    corpus = directory / 'standalone'
    row = run(target, dict(env, CINCH_TEST_INVOCATIONS=str(directory),
                           CINCH_TEST_CORPUS_REPORTS=str(corpus)), out, cargo_target)
    row['standalone_invocations'] = sum(len(bootstrap.read_json(path)['runs'])
                                        for path in corpus.glob('*.json'))
    row['selected_invocations'] = sum(1 for _ in directory.glob('*.json')) + row['standalone_invocations']
    return row


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--stage', type=int, default=3)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--jobs', type=int, default=4)
    p.add_argument('--cargo-target', type=Path, default=None)
    p.add_argument('--tree', type=Path, default=ROOT)
    p.add_argument('--compiler', type=Path, default=None,
                   help='a development compiler to test instead of a stage, for example the '
                        'one tooling/module_build.py links; it is run against the pinned library')
    selection = p.add_mutually_exclusive_group()
    selection.add_argument('--all', action='store_true')
    selection.add_argument('--all-integration', action='store_true', help='run every Cargo integration test target')
    p.add_argument('--allow-dirty', action='store_true',
                   help='measure a tree with uncommitted changes instead of refusing')
    p.add_argument('targets', nargs='*')
    a = p.parse_args()
    # A suite measures the tree it compiles, and this runner compiles the
    # tree's own sources. An edit landing while it runs (another lane writing
    # one file by a relative path, an integration in progress) produces
    # results that describe neither the old tree nor the new one, so refuse
    # by default and make the exception explicit.
    dirty = subprocess.run(['git', '-c', 'core.fsmonitor=false', 'status', '--porcelain', '--',
                            'compiler', 'seed', 'tooling'],
                           cwd=a.tree, capture_output=True, text=True).stdout.split()
    if dirty and not a.allow_dirty:
        sys.stderr.write(f'refusing to measure a dirty tree: {" ".join(dirty[:8])}\n'
                         f'commit the change or pass --allow-dirty\n')
        return 2
    if a.all_integration:
        a.targets = integration_targets(a.tree)
    elif a.all:
        a.targets = all_targets(a.tree)
    if not a.targets:
        p.error('name targets, pass --all, or pass --all-integration')
    # Hold this tree's stage lock for the whole run, so a rebuild cannot
    # replace the compiler under the suite (that produced two void runs on
    # 2026-11-11: targets failing with a missing stage binary or a Core shape
    # that existed for a few minutes). bootstrap.py takes the same lock
    # exclusively and refuses rather than blocking.
    lock_path = bootstrap.BUILD / '.lock'
    # A read-only build tree (a Nix store path, see tooling/nix) cannot be
    # rebuilt under the suite, so it needs no lock and cannot hold one.
    if os.access(bootstrap.BUILD, os.W_OK) or not bootstrap.BUILD.exists():
        lock_path.parent.mkdir(parents=True, exist_ok=True)
        suite_lock = lock_path.open('a')
        try:
            fcntl.flock(suite_lock, fcntl.LOCK_SH | fcntl.LOCK_NB)
        except OSError:
            sys.stderr.write('refusing to measure: a build holds ' + str(lock_path) + '\n')
            return 3
    a.out.mkdir(parents=True, exist_ok=True)
    a.cargo_target = a.cargo_target or a.tree / 'seed' / 'target'
    global TREE; TREE = a.tree
    env = setup(a.stage, a.out.resolve(), a.tree, a.compiler.resolve() if a.compiler else None)
    prebuild(a.targets, env, a.out.resolve(), a.cargo_target)
    # Cargo releases its build lock before running a test executable. Each
    # worker can start its suite as soon as that executable is ready.
    rows = []
    with cf.ThreadPoolExecutor(a.jobs) as pool:
        if a.all_integration:
            receipts = Path(tempfile.mkdtemp(prefix='invocations-', dir=a.out.resolve()))
            futures = [pool.submit(run_with_receipts, t, env, a.out.resolve(), a.cargo_target, receipts) for t in a.targets]
        else:
            futures = [pool.submit(run, t, env, a.out.resolve(), a.cargo_target) for t in a.targets]
        for future in cf.as_completed(futures):
            row = future.result()
            rows.append(row)
            print(f"{row['target']}: exit={row['exit']} passed={row['passed']} failed={row['failed_count']}", flush=True)
    rows.sort(key=lambda row: row['target'])
    (a.out / 'summary.json').write_text(json.dumps(rows, indent=1))
    passing = sum(row['exit'] == 0 for row in rows)
    print('targets passing:', passing, '/', len(rows))
    return int(passing != len(rows))


if __name__ == '__main__':
    sys.exit(main())
