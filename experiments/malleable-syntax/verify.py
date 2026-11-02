#!/usr/bin/env python3
"""Verify the current experiment and record source-bound evidence."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def main():
    commands = [
        (['cargo', 'build', '--quiet'], ROOT / 'compiler'),
        ([sys.executable, '-m', 'unittest', 'discover', '-s', str(HERE / 'tests'), '-v'], ROOT),
    ]
    runs = []
    for command, directory in commands:
        print('Running ' + ' '.join(command), flush=True)
        result = subprocess.run(command, cwd=directory, text=True, capture_output=True)
        runs.append({'command': command, 'directory': str(directory), 'exit_code': result.returncode,
                     'output': result.stdout + result.stderr})
        if result.returncode: break
    files = list(HERE.glob('*.py')) + list((HERE / 'tests').glob('*.py'))
    files += list((HERE / 'syntax').glob('*.json')) + list((HERE / 'libraries').glob('*.cinch'))
    files += list((HERE / 'examples').rglob('*.cinch'))
    files += [ROOT / p for p in ['compiler/src/check.rs', 'compiler/src/check_terms.rs', 'compiler/src/check_decls.rs',
        'compiler/src/types.rs', 'compiler/tests/goal_tactics.rs',
        'compiler/src/lsp.rs', 'tooling/tree-sitter-cinch/grammar.js',
        'tooling/tree-sitter-cinch/src/scanner.c', 'tooling/editors/zed/languages/cinch/highlights.scm']]
    report = {'version': 1, 'time_utc': datetime.now(timezone.utc).isoformat(),
              'passed': all(run['exit_code'] == 0 for run in runs) and len(runs) == len(commands),
              'positive_programs': len(list((HERE / 'examples').glob('*.cinch'))),
              'negative_programs': len(list((HERE / 'examples/negative').glob('*.cinch'))),
              'syntax_libraries': len(list((HERE / 'syntax').glob('*.json'))),
              'source_hashes': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)},
              'runs': runs}
    (HERE / 'verification.json').write_text(json.dumps(report, indent=2) + '\n')
    print(runs[-1]['output'], end='')
    print('PASS' if report['passed'] else 'FAIL')
    return 0 if report['passed'] else 1


if __name__ == '__main__': sys.exit(main())
