"""Record what the real compiler does with each landing example.

Runnable blocks on the landing page play back these recordings until the
browser build of the compiler is available. Re-run after changing an
example or the compiler:

    CINCH_CLANG=/path/to/clang-20 python3 scripts/record-examples.py

Set CINCH to choose the compiler binary (default: the release build in
../compiler). `--check` fails if a stored recording no longer matches.
"""
import json, os, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / 'src' / 'data' / 'examples'
OUT = EXAMPLES / 'recordings.json'
CINCH = os.environ.get('CINCH', str(ROOT.parent / 'compiler' / 'target' / 'release' / 'cinch'))
# file -> command; "run" programs print, "check" programs are accepted or rejected.
PLAN = {
    'average-report.cinch': 'run',
    'average-unchecked.cinch': 'check',
}

def record(name, mode):
    proc = subprocess.run([CINCH, mode, name], cwd=EXAMPLES, capture_output=True, text=True)
    # The build step prints cargo progress for the runtime on stderr; keep only compiler diagnostics.
    stderr = '\n'.join(l for l in proc.stderr.splitlines() if l.startswith('cinch:'))
    return {'command': f'cinch {mode} {name}', 'stdout': proc.stdout, 'stderr': stderr, 'exit': proc.returncode}

def main():
    recordings = {name: record(name, mode) for name, mode in PLAN.items()}
    text = json.dumps(recordings, indent=2, ensure_ascii=False) + '\n'
    if '--check' in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            sys.exit('recordings.json is stale: re-run scripts/record-examples.py')
        print(f'{len(recordings)} recordings match the compiler')
        return
    OUT.write_text(text)
    print(f'wrote {OUT.relative_to(ROOT)}')

main()
