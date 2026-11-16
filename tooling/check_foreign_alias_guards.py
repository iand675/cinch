#!/usr/bin/env python3
"""Execute private foreign-alias guard probes from the actual implementation without exporting a compiler API."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess

import bootstrap

MAIN = '''-- | Execute private guarded-alias refusal and typing observations.
module Main where
  import Bootstrap.RuntimeIr (foreign-alias-guard-observations)
  -- | Name a failed guard by its stable one-based observation index.
  first-failure :: Int -> List Bool -> Int
  first-failure index values = case values of Nil -> 0; Cons first rest -> if first then first-failure (index + 1) rest else index
  -- | Fail if an unsupported foreign value shape acquires a rewrite.
  main :: Int
  main = first-failure 1 foreign-alias-guard-observations
'''


def main() -> None:
    """Retain the exact production prefix, test-only suffix, source snapshot and independent runtime verdicts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--producer', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    files = bootstrap.collect_compiler_sources()
    production_hash = bootstrap.source_hash(files)
    implementation = files['Bootstrap.RuntimeIr.cinch']
    suffix = (bootstrap.ROOT / 'compiler/tests/fixtures/foreign-alias-guard-probes.cinch.inc').read_bytes()
    files['Bootstrap.RuntimeIr.cinch'] = implementation + b'\n' + suffix
    files['Main.cinch'] = MAIN.encode()
    source, ir, executable = args.artifacts / 'Bundle.cinch', args.artifacts / 'probe.ll', args.artifacts / 'probe'
    source.write_bytes(bootstrap.bundle(files))
    report = {'status': 'building', 'production_source_sha256': production_hash,
              'production_runtime_ir_sha256': bootstrap.digest(implementation), 'test_only_suffix_sha256': bootstrap.digest(suffix),
              'test_source_sha256': bootstrap.source_hash(files), 'bundle_sha256': bootstrap.file_hash(source),
              'producer_sha256': bootstrap.file_hash(args.producer), 'runtime_sha256': bootstrap.file_hash(args.runtime),
              'compiler_host_opt_level': 0, 'observations': 20, 'steps': [], 'runs': []}
    bootstrap.write_json(args.output, report)
    for name, command in [('emit', [str(args.producer.resolve()), 'emit-ir', str(source), '-o', str(ir)]),
                          ('link', ['clang', '-O0', '-Wno-override-module', str(ir), str(args.runtime.resolve()), '-o', str(executable), '-ldl', '-lpthread', '-lm'])]:
        with (args.artifacts / (name + '.log')).open('w') as log:
            run = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, env=bootstrap.environment())
        report['steps'].append({'name': name, 'command': command, 'exit': run.returncode})
        bootstrap.write_json(args.output, report)
        if run.returncode:
            report['status'] = 'failed'
            bootstrap.write_json(args.output, report)
            raise bootstrap.BootstrapError('private alias guard build failed: ' + name)
    report.update(llvm_sha256=bootstrap.file_hash(ir), executable_sha256=bootstrap.file_hash(executable))
    for debug in [False, True]:
        run = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
            env={**os.environ, 'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M',
                 'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1'})
        report['runs'].append({'debug': debug, 'exit': run.returncode, 'stderr': run.stderr})
        bootstrap.write_json(args.output, report)
        if run.returncode or 'leaked=0' not in run.stderr:
            report['status'] = 'failed'
            bootstrap.write_json(args.output, report)
            raise bootstrap.BootstrapError('private alias guard observations failed')
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print('20 private alias guards passed normal and poisoned profiles.', flush=True)


if __name__ == '__main__':
    main()
