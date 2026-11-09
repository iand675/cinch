"""Check the book's complete examples using the Rust compiler CLI.

Run: python3 scripts/verify-doc-examples.py --compiler /path/to/cinch
All projects, package stores, generated IR, and executables live in a temporary
folder. Snippets without a cinch marker are explanatory fragments. Every
complete module in learn/ must have a marker, including expected rejections.
"""
from pathlib import Path
import argparse
from concurrent.futures import as_completed
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
FENCE = re.compile(r'(?P<marker><!-- cinch:(?P<meta>.*?) -->\s*)?```cinch\n(?P<source>.*?)\n```', re.S)

def examples():
    projects = {}
    for page in sorted((ROOT / 'learn').glob('*.md')) + sorted((ROOT / 'reference').glob('*.md')):
        for index, match in enumerate(FENCE.finditer(page.read_text())):
            source = match['source']
            if not match['meta']:
                if source.startswith('module '):
                    raise AssertionError(f'{page.relative_to(ROOT)}: complete module has no verification marker')
                continue
            args = shlex.split(match['meta'])
            mode = args.pop(0)
            opts = dict(arg.split('=', 1) for arg in args)
            case = f'{page.stem}-{index + 1}'
            if mode == 'project':
                group = opts.pop('group')
                project = projects.setdefault(group, {'files': {}, 'mode': 'run', 'exit': '0', 'case': group})
                project['files'][opts.pop('file')] = source
                project.update(opts)
            else:
                yield {'case': case, 'page': str(page.relative_to(ROOT)), 'mode': mode, 'files': {'Main.cinch': source}, **opts}
    yield from projects.values()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', default=shutil.which('cinch'))
    parser.add_argument('--report', type=Path)
    parser.add_argument('--match', help='Run cases whose name contains this text')
    parser.add_argument(
        '--jobs',
        type=int,
        default=1,
        help='Cases to run concurrently (each worker gets its own store; 1 is serial)',
    )
    parser.add_argument(
        '--timeout',
        type=int,
        default=300,
        help='Seconds one compiler invocation may take before the case is reported as timed out',
    )
    options = parser.parse_args()
    if not options.compiler:
        parser.error('supply --compiler with a built Rust cinch executable')
    compiler = str(Path(options.compiler).resolve())
    cases = [c for c in examples() if not options.match or options.match in c['case']]
    if not cases:
        parser.error('no documentation examples matched')
    jobs = max(1, options.jobs)
    results: list[dict] = []
    with tempfile.TemporaryDirectory(prefix='cinch-book-') as temp:
        workspace = Path(temp)
        # One store per worker: a store write is a directory swap under a
        # lock, so sharing one across concurrent cases would serialise the
        # very work this flag exists to overlap.
        envs = [
            {
                **os.environ,
                'CINCH_STORE': str(workspace / f'store-{worker}'),
                'CINCH_HEAP_LIMIT': '512M',
            }
            for worker in range(jobs)
        ]

        def run_case(indexed):
            index, case, worker = indexed
            directory = workspace / case['case']
            directory.mkdir()
            for name, source in case['files'].items():
                (directory / name).write_text(source + '\n')
            env = envs[worker]

            def invoke(command, directory):
                # A timeout is the machine's problem, not the case's: it is
                # reported as a failed case with the reason rather than
                # aborting the whole run, so one loaded moment cannot hide
                # the other four hundred results.
                return subprocess.run(
                    command,
                    cwd=directory,
                    env=env,
                    capture_output=True,
                    text=True,
                    timeout=options.timeout,
                )

            try:
                return run_case_inner(case, directory, env, invoke)
            except subprocess.TimeoutExpired:
                result = {
                    'case': case['case'],
                    'mode': case['mode'],
                    'passed': False,
                    'source_sha256': hashlib.sha256(
                        json.dumps(case['files'], sort_keys=True).encode()
                    ).hexdigest(),
                }
                return result, [
                    'FAIL ' + case['case'],
                    f'timed out after {options.timeout}s',
                ]

        def run_case_inner(case, directory, env, invoke):
            check = invoke([compiler, 'check', 'Main.cinch'], directory)
            if case['mode'] == 'reject':
                passed = check.returncode == 1 and case['contains'] in check.stderr
                detail = check.stderr.strip()
            else:
                passed = check.returncode == 0
                detail = check.stderr.strip()
                if passed and case['mode'] == 'run':
                    build = invoke([compiler, 'build', 'Main.cinch', '-o', 'example'], directory)
                    passed = build.returncode == 0
                    detail = build.stderr.strip()
                    if passed:
                        run = invoke([str(directory / 'example')], directory)
                        expected_stdout = json.loads('"' + case.get('stdout', '').replace('"', '\\"') + '"')
                        passed = run.returncode == int(case.get('exit', 0)) and run.stdout == expected_stdout
                        detail = f'exit={run.returncode}, stdout={run.stdout!r}, stderr={run.stderr!r}'
            result = {
                'case': case['case'],
                'mode': case['mode'],
                'passed': passed,
                'source_sha256': hashlib.sha256(
                    json.dumps(case['files'], sort_keys=True).encode()
                ).hexdigest(),
            }
            lines = [('PASS ' if passed else 'FAIL ') + case['case']]
            if not passed:
                lines.append(detail)
            return result, lines

        indexed = [
            (index, case, index % jobs) for index, case in enumerate(cases)
        ]
        slots: dict[int, tuple[dict, list[str]]] = {}

        def emit(index):
            # Print in case order as soon as a slot is ready: a parallel
            # run keeps the serial transcript's shape, so a diff of two
            # runs reads the same whether or not --jobs was used.
            while next_print[0] in slots:
                result, lines = slots.pop(next_print[0])
                results.append(result)
                for line in lines:
                    print(line, flush=True)
                next_print[0] += 1

        next_print = [0]
        if jobs == 1:
            for item in indexed:
                slots[item[0]] = run_case(item)
                emit(item[0])
        else:
            from concurrent.futures import ThreadPoolExecutor

            with ThreadPoolExecutor(max_workers=jobs) as pool:
                futures = {pool.submit(run_case, item): item[0] for item in indexed}
                for future in as_completed(futures):
                    index = futures[future]
                    slots[index] = future.result()
                    emit(index)
            emit(len(indexed) - 1)
    if not results:
        parser.error('no documentation examples matched')
    report = {'compiler_sha256': hashlib.sha256(Path(compiler).read_bytes()).hexdigest(), 'examples': results}
    if options.report:
        options.report.write_text(json.dumps(report, indent=2) + '\n')
    failed = sum(not r['passed'] for r in results)
    print(f'{len(results) - failed}/{len(results)} documentation examples passed')
    return int(failed != 0)

if __name__ == '__main__':
    raise SystemExit(main())
