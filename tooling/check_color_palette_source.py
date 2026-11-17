#!/usr/bin/env python3
"""Check an isolated fixed-palette source closure and retain genuine defining artifacts."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
from bench_stdlib import source_closure
from check_compare import digest


#: The pure source caller exercises both factories and their public color/palette result types.
MAIN = '''-- | Check both immutable fixed-palette factories through their public result surface.
module Main where
  import Image.Color
  import Image.Color.Palette
  import Data.Bool
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option

  -- | Observe the first raw type without leaking private constructors.
  opaque-head :: List Color -> Bool
  opaque-head entries = case entries of
    Nil -> False
    Cons first rest -> case color-rgba8-components first of
      None -> False
      Some (r, g, b, a) -> eq-u8 a 255

  -- | Observe exact table length and representation through the public list surface.
  shape :: ColorPalette -> Nat -> Bool
  shape palette expected = list-length (color-palette-colors palette) == expected && opaque-head (color-palette-colors palette)

  -- | Typecheck independently retained immutable results from both factories.
  main :: Int
  main = if shape (color-palette-plan9 ()) 256 && shape (color-palette-web-safe ()) 216 then 0 else 1
'''


def main() -> None:
    """Use a fresh source project with explicit frozen identities and no emission/runtime claim."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--color', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350MiB source-unit start guard')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    sources = source_closure(MAIN, {'Image.Color': args.color, 'Image.Color.Palette': args.source})
    for name, source in sources.items():
        shutil.copyfile(source, work / (name+'.cinch'))
    (work / 'Main.cinch').write_text(MAIN)
    command = [str(args.compiler.resolve()), 'check', str(work / 'Main.cinch')]
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)), 'command': command,
              'compiler': str(args.compiler.resolve()), 'compiler_sha256': digest(args.compiler),
              'future_runtime': str(args.runtime.resolve()), 'runtime_sha256': digest(args.runtime),
              'source_closure_sha256': {name: digest(work / (name+'.cinch')) for name in sources},
              'main_sha256': digest(work / 'Main.cinch'), 'cpu_affinity': [args.cpu],
              'compiler_host_stack_bytes': 67108864, 'initial_free_bytes': initial,
              'source_only': True, 'runtime_claim': False}
    try:
        result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=30,
                                env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr,
                      status='passed' if result.returncode == 0 else 'failed')
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error))
        raise
    finally:
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()),
                      defining_interfaces={str(path.relative_to(work)): digest(path) for path in work.rglob('*.cinchif')})
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
    if report['output_bytes'] > 30*1024*1024 or report['final_free_bytes'] < 250*1024*1024:
        raise RuntimeError('source-unit output/reserve guard')
    if report['status'] != 'passed':
        raise SystemExit(1)
    print('full isolated fixed-palette source closure passed; no runtime claim')


if __name__ == '__main__':
    main()
