#!/usr/bin/env python3
"""Qualify only HKDF source and canonical docs under explicit immutable inputs.

No SDK program, LLVM emission, linking, runtime or timing workload is invoked.
Every source epoch uses a new directory; failed projects remain entirely plain.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import signal
import subprocess
import time
from bench_stdlib import source_closure

#: Owned experiment root, independent of the integration checkout.
ROOT = Path(__file__).resolve().parents[1]
#: Minimal pure caller; successful checking does not execute this program.
MAIN = '''-- | Typecheck a complete public derivation without executing it.
module Main where
  import Crypto.Hkdf
  import Text.Digest
  import Data.Bytes
  import Data.Bool
  import Data.Option
  import Data.Result
  import Data.Nat

  -- | Fix the provider-error identity for the built-in pure digest adapter.
  derive :: Bytes -> Result (HkdfError Unit) Bytes
  derive input = hkdf-key DigestSha256 input None (bytes-new 0) 42

  -- | Observe the intended result width only if a later runtime gate executes this caller.
  main :: Int
  main = case derive (bytes-new 22) of
    Err error -> 1
    Ok output -> if bytes-len output == 42 then 0 else 2
'''


def digest(path: Path) -> str:
    """Hash exact file bytes without modifying the selected input."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory(work: Path) -> list[dict]:
    """Retain every source, genuine interface and diagnostic without cache pruning."""
    return [{'path': str(path.relative_to(work)), 'bytes': path.stat().st_size,
             'sha256': digest(path)} for path in sorted(work.rglob('*'))
            if path.is_file() and path.name != 'report.json']


def run_unit(command: list[str], work: Path, seconds: float, reserve: int, cap: int) -> dict:
    """Bound one actual source invocation by wall time, free reserve and fresh bytes."""
    before = shutil.disk_usage(work).free
    started = time.monotonic()
    reason = None
    with (work / 'stdout.log').open('wb') as stdout, (work / 'stderr.log').open('wb') as stderr:
        process = subprocess.Popen(command, cwd=work, stdout=stdout, stderr=stderr,
                                   start_new_session=True, env={**os.environ,
                                   'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        while process.poll() is None:
            size = sum(p.stat().st_size for p in work.rglob('*') if p.is_file())
            if time.monotonic() - started >= seconds:
                reason = 'source-unit deadline'
            elif shutil.disk_usage(work).free < reserve:
                reason = 'free-space reserve'
            elif size >= cap:
                reason = 'fresh-output cap'
            if reason:
                os.killpg(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
                break
            time.sleep(0.05)
        code = process.wait()
    return {'command': command, 'returncode': code, 'resource_stop': reason,
            'source_wall_seconds': time.monotonic() - started,
            'free_before': before, 'free_after': shutil.disk_usage(work).free,
            'stdout': (work / 'stdout.log').read_text(errors='replace'),
            'stderr': (work / 'stderr.log').read_text(errors='replace')}


def main() -> None:
    """Create one immutable source epoch and preserve its first genuine outcome."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--compiler-sha', required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--runtime-sha', required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--stage', choices=('source', 'docs'), required=True)
    parser.add_argument('--consumer', type=Path, help='Explicit source-only callback or refusal caller')
    parser.add_argument('--expect-refusal', help='Exact required diagnostic substring for an intentional negative source fixture')
    parser.add_argument('--cpu', type=int, default=1)
    parser.add_argument('--start-mib', type=int, default=320)
    parser.add_argument('--reserve-mib', type=int, default=250)
    parser.add_argument('--cap-mib', type=int, default=5)
    parser.add_argument('--seconds', type=float, default=15)
    args = parser.parse_args()
    compiler, runtime, work = args.compiler.resolve(), args.runtime.resolve(), args.work.resolve()
    assert digest(compiler) == args.compiler_sha, 'compiler identity mismatch'
    assert digest(runtime) == args.runtime_sha, 'runtime identity mismatch'
    assert not work.exists(), 'source epochs cannot be overwritten'
    free = shutil.disk_usage(ROOT).free
    if free < args.start_mib * 1024**2:
        raise SystemExit(f'resource pause before any compiler launch: free={free}, start={args.start_mib * 1024**2}')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work.mkdir(parents=True)
    candidate = ROOT / 'experiments/hkdf-v1/sources/Crypto.Hkdf.cinch'
    caller = args.consumer.read_text() if args.consumer else MAIN
    closure = source_closure(caller, {'Crypto.Hkdf': candidate})
    assert 'Data.Bytes' not in closure, 'embedded byte-buffer authority must stay frozen'
    for name, source in closure.items():
        shutil.copyfile(source, work / (name + '.cinch'))
    (work / 'Main.cinch').write_text(caller)
    report = {'scope': 'Source/type/proof only' if args.stage == 'source' else 'Canonical documentation parser only',
              'sdk_executed': False, 'runtime_executed': False, 'ir_emitted': False,
              'stage': args.stage, 'compiler': {'path': str(compiler), 'sha256': digest(compiler)},
              'future_runtime_only': {'path': str(runtime), 'sha256': digest(runtime)},
              'cpu': args.cpu, 'compiler_host_stack_bytes': 67108864,
              'scoped_source_only_start_mib': args.start_mib, 'reserve_mib': args.reserve_mib,
              'cap_bytes': args.cap_mib * 1024**2, 'seconds': args.seconds,
              'runner_sha256': digest(Path(__file__)), 'candidate_sha256': digest(candidate),
              'selected_nonembedded_sources': {name: {'original': str(source), 'sha256': digest(source)}
                                               for name, source in closure.items()},
              'consumer': str(args.consumer.resolve()) if args.consumer else 'default pure caller',
              'expected_refusal': args.expect_refusal,
              'embedded_Data_Bytes_override': False,
              'resolver_builtin_manifest_sha256': digest(ROOT / 'compiler/prelude/cinch-base.cinchpkg')}
    command = [str(compiler), 'check', str(work / 'Main.cinch')] if args.stage == 'source' else [
        str(compiler), 'doc', 'extract', str(work / 'Crypto.Hkdf.cinch'),
        '--module', 'Crypto.Hkdf', '--out', str(work / 'docs.json')]
    report['result'] = run_unit(command, work, args.seconds, args.reserve_mib * 1024**2,
                                args.cap_mib * 1024**2)
    if args.stage == 'docs' and (work / 'docs.json').exists():
        docs = json.loads((work / 'docs.json').read_text())
        report['canonical_docs'] = {'diagnostics': docs.get('diagnostics', []), 'orphans': docs.get('orphans', []),
                                    'module': docs.get('module')}
    report['files'] = inventory(work)
    report['artifact_bytes_before_report'] = sum(row['bytes'] for row in report['files'])
    report['passed'] = report['result']['returncode'] == 0 and report['result']['resource_stop'] is None
    if args.expect_refusal:
        assert args.stage == 'source', 'negative fixtures are source-only'
        report['passed'] = (report['result']['returncode'] != 0 and report['result']['resource_stop'] is None
                            and args.expect_refusal in report['result']['stderr'])
    if args.stage == 'docs':
        report['passed'] = report['passed'] and 'canonical_docs' in report and not report['canonical_docs']['diagnostics'] and not report['canonical_docs']['orphans']
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'passed': report['passed'], 'candidate_sha256': report['candidate_sha256'],
                      'result': report['result'], 'artifact_bytes': report['artifact_bytes_before_report']}))
    if not report['passed']:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
