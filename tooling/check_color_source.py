#!/usr/bin/env python3
"""Check a complete isolated color source with an explicit frozen compiler/runtime identity."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
import time
from check_compare import digest


#: Small public consumer; complete imported module definitions are checked as well.
MAIN = '''-- | Check a pure color module without emission or execution.
module Main where
  import Image.Color
  import Data.Machine

  -- | Observe exact public straight-alpha expansion in the source checker.
  main :: Int
  main = case color-rgba (color-nrgba8 255 128 0 127) of
    (r, g, b, a) -> if eq-u32 a 32639 then 0 else 1
'''


def main() -> None:
    """Preserve every source, actual defining artifact and failure without compiling a runtime."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    free = shutil.disk_usage(work).free
    if free < 350 * 1024 * 1024:
        raise RuntimeError('350 MiB initial source-unit guard')
    shutil.copyfile(args.source, work / 'Image.Color.cinch')
    (work / 'Main.cinch').write_text(MAIN)
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    command = [str(compiler), 'check', str(work / 'Main.cinch')]
    report = {'scope': __doc__, 'runtime_claim': False, 'compiler': str(compiler),
              'compiler_sha256': digest(compiler), 'future_runtime': str(runtime),
              'future_runtime_sha256': digest(runtime), 'source_sha256': digest(work / 'Image.Color.cinch'),
              'main_sha256': digest(work / 'Main.cinch'), 'runner_sha256': digest(Path(__file__)),
              'cpu_affinity': [args.cpu], 'compiler_host_stack_bytes': 67108864,
              'command': command, 'initial_free_bytes': free,
              'resource_policy': {'initial_mib': 350, 'reserve_mib': 250, 'unit_mib': 30}}
    started = time.monotonic()
    try:
        result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=30,
                                env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr,
                      failed_artifacts_retained_plain=result.returncode != 0)
    except subprocess.TimeoutExpired as error:
        report.update(timeout_seconds=30, failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        report.update(seconds=time.monotonic() - started, final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()),
                      defining_interfaces={path.name: digest(path) for path in sorted((work / 'build').glob('*.cinchif'))})
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    if report['output_bytes'] > 30 * 1024 * 1024 or report['final_free_bytes'] < 250 * 1024 * 1024:
        raise RuntimeError('source-unit output/reserve guard exceeded')
    if report['returncode'] != 0:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
