"""Check the book's complete examples using the Rust compiler CLI.

Run: python3 scripts/verify-doc-examples.py --compiler /path/to/cinch
All projects, package stores, generated IR, and executables live in a temporary
folder. Snippets without a cinch marker are explanatory fragments. Every
complete module in learn/ must have a marker, including expected rejections.
"""
from pathlib import Path
import argparse
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
    options = parser.parse_args()
    if not options.compiler:
        parser.error('supply --compiler with a built Rust cinch executable')
    compiler = str(Path(options.compiler).resolve())
    results = []
    with tempfile.TemporaryDirectory(prefix='cinch-book-') as temp:
        workspace = Path(temp)
        env = {**os.environ, 'CINCH_STORE': str(workspace / 'store'), 'CINCH_HEAP_LIMIT': '512M'}
        def invoke(command, directory):
            return subprocess.run(command, cwd=directory, env=env, capture_output=True, text=True, timeout=90)
        for case in examples():
            if options.match and options.match not in case['case']:
                continue
            directory = workspace / case['case']
            directory.mkdir()
            for name, source in case['files'].items():
                (directory / name).write_text(source + '\n')
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
            result = {'case': case['case'], 'mode': case['mode'], 'passed': passed, 'source_sha256': hashlib.sha256(json.dumps(case['files'], sort_keys=True).encode()).hexdigest()}
            results.append(result)
            print(('PASS ' if passed else 'FAIL ') + case['case'], flush=True)
            if not passed:
                print(detail, flush=True)
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
