#!/usr/bin/env python3
"""Validate guarded compiler-generated foreign aliases without widening foreign function-value support."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects

FOREIGN_C = '''#include <stdint.h>
/* Observe each argument action in order without hiding effects in inline assembly. */
static uint64_t observed;
uint8_t alias_tick(uint8_t value) { observed = observed * 10 + value; return value; }
/* Observe the actual foreign call after its argument has been fully evaluated. */
uint64_t alias_outer(uint8_t value) { observed = observed * 10 + 9; return (uint64_t)value * UINT64_C(72340172838076673); }
/* Read the independently accumulated observation after the foreign operation. */
uint64_t alias_trace(void) { return observed; }
'''


def trace_source(flag: bool, byte: int, empty: bool = False) -> str:
    """Force dependent argument scopes around explicit foreign effects and observe only the selected branch."""
    expected = 0 if empty else byte * 72340172838076673
    trace = 0 if empty else 129 if flag else 139
    return '''-- | Preserve foreign operand effects across dependent argument witness scopes.
module Main where
  export main
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  -- | Record one explicit global action and return its narrow input.
  foreign import ccall "alias_tick" tick :: U8 ->{Global} U8
  -- | Record a saturated foreign operation after its argument actions.
  foreign import ccall "alias_outer" outer :: U8 ->{Global} U64
  -- | Read the independently observed sequence.
  foreign import ccall "alias_trace" trace :: Unit ->{Global} U64
  -- | Retain effectful strict lets before the nested conversion and final foreign call.
  nested :: Bytes -> Bool ->{Global} U64
  nested needle flag = let size = bytes-len needle in
    if size == 0 then 0 else
      outer (let first = tick 1 in let selected = if flag then tick 2 else tick 3 in
        wrapping-i64-to-u8 (nat-to-i64 (bytes-get needle 0)))
  -- | Check the selected actions execute exactly once in source order.
  main :: Unit ->{Global} Int
  main unit = let value = nested (bytes-from-u8s ''' + ('[]' if empty else f'[{byte}]') + ') ' + str(flag) + ''' in
    let recorded = trace () in
    case value of ''' + str(expected) + ' -> (case recorded of ' + str(trace) + ' -> 0; _ -> 2); _ -> 1\n'


def source_cases() -> list[dict]:
    """Keep original controls byte-identical and give unsupported value shapes explicit refusal observations."""
    fixture = bootstrap.ROOT / 'seed/tests/fixtures'
    nested = (fixture / 'native-scalar-nested-abi.cinch').read_text()
    sequenced = (fixture / 'native-scalar-sequenced-abi.cinch').read_text()
    cases = [{'name': 'original-nested', 'source': nested, 'accepted': True, 'baseline_accepted': False},
             {'name': 'original-sequenced', 'source': sequenced, 'accepted': True, 'baseline_accepted': True}]
    for flag, byte, empty in [(True, 1, False), (False, 128, False), (True, 255, False), (False, 1, True)]:
        cases.append({'name': f'trace-{flag}-{byte}-{empty}', 'source': trace_source(flag, byte, empty), 'accepted': True})
    shadow = nested.replace('wrapping-i64-to-u8 (nat-to-i64 (bytes-get needle 0))',
        '(let repeated-byte = (\\byte -> wrapping-add-u8 byte 1) in let extra = repeated-byte 1 in wrapping-add-u8 extra (wrapping-i64-to-u8 (nat-to-i64 (bytes-get needle 0))))')
    shadow = shadow.replace('72340172838076673 then 0', '217020518514230019 then 0')
    cases.append({'name': 'lexical-function-shadow', 'source': shadow, 'accepted': True})
    for name, main in [
        ('ordinary-function-alias', 'let function = repeated-byte in case function 1 of 72340172838076673 -> 0; _ -> 1'),
        ('repeated-function-alias', 'let function = repeated-byte in let first = function 1 in case function 2 of 144680345676153346 -> 0; _ -> 1'),
        ('branching-function-alias', 'let function = repeated-byte in if True then (case function 1 of 72340172838076673 -> 0; _ -> 1) else 2'),
        ('higher-order-function-value', 'case apply repeated-byte 1 of 72340172838076673 -> 0; _ -> 1'),
    ]:
        source = nested[:nested.index('  -- | Keep the imported call reachable')]
        source += '''  -- | Keep a higher-order shape outside the guarded compiler alias path.
  apply :: (U8 -> U64) -> U8 -> U64
  apply function value = function value
  -- | Observe the existing unsupported general foreign function-value shape.
  main :: Int
  main = ''' + main + '\n'
        cases.append({'name': name, 'source': source, 'accepted': False, 'baseline_accepted': False})
    paired = '''-- | Keep proper foreign partial application outside the bare-alias repair.
module Main where
  export main
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  -- | Combine two narrow values through the existing checked inline ABI.
  prim pair :: U8 -> U8 -> U64 where
    llvm {
      %left = zext i8 %0 to i64
      %right = zext i8 %1 to i64
      %sum = add i64 %left, %right
      ret i64 %sum
    }
  -- | Introduce a proper partial prefix before a dependent final argument.
  nested :: Bytes -> U64
  nested needle = if bytes-len needle == 0 then 0 else let partial = pair 1 in partial (wrapping-i64-to-u8 (nat-to-i64 (bytes-get needle 0)))
  -- | Keep the proper partial foreign call reachable.
  main :: Int
  main = case nested (bytes-from-u8s [1]) of 2 -> 0; _ -> 1
'''
    cases.append({'name': 'proper-foreign-partial-prefix', 'source': paired, 'accepted': False, 'baseline_accepted': False})
    return cases


def main() -> None:
    """Retain exact seed/provider/source identities, named refusals and both runtime ownership profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--baseline', type=Path)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--opt-level', type=int, choices=[0, 2], required=True)
    args = parser.parse_args()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    manifest, objects = provider['interface_package'], args.artifacts / 'objects.json'
    bootstrap.write_json(objects, provider['configuration'])
    fixture = args.artifacts / 'trace.c'
    fixture.write_text(FOREIGN_C)
    fixture_object = args.artifacts / 'trace.o'
    subprocess.run(['clang', '-O2', '-c', str(fixture), '-o', str(fixture_object)], check=True)
    report = {'status': 'running', 'provider_receipt_sha256': provider['identity']['receipt_sha256'],
              'object_sha256': bootstrap.file_hash(provider['object']), 'runtime_sha256': bootstrap.file_hash(provider['runtime']),
              'foreign_fixture_sha256': bootstrap.file_hash(fixture), 'consumer_opt_level': args.opt_level, 'results': []}
    compilers = [('candidate', args.compiler)] + ([('baseline', args.baseline)] if args.baseline else [])
    try:
        for label, compiler in compilers:
            compiler = compiler.resolve()
            for case in source_cases():
                if label == 'baseline' and 'baseline_accepted' not in case:
                    continue
                expected = case['accepted'] if label == 'candidate' else case['baseline_accepted']
                directory = args.artifacts / (label + '-' + case['name'])
                directory.mkdir()
                source, ir, executable = directory / 'Main.cinch', directory / 'consumer.ll', directory / 'consumer'
                source.write_text(case['source'])
                command = [str(compiler), '--emit-ir', '--rc-debug', '--interfaces', str(manifest), '--objects', str(objects), str(source), str(ir)]
                compiled = subprocess.run(command, capture_output=True, text=True, env=bootstrap.environment())
                (directory / 'compile.log').write_text(compiled.stdout + compiled.stderr)
                diagnostics = Path(str(ir) + '.diagnostics')
                message = diagnostics.read_text() if diagnostics.exists() else compiled.stderr
                row = {'name': case['name'], 'frontend': label, 'compiler_sha256': bootstrap.file_hash(compiler),
                       'source_sha256': bootstrap.file_hash(source), 'command': command, 'exit': compiled.returncode,
                       'diagnostic': message, 'expected_accepted': expected, 'runs': []}
                report['results'].append(row)
                bootstrap.write_json(args.output, report)
                if (compiled.returncode == 0) != expected:
                    raise bootstrap.BootstrapError(f'{label} {case["name"]}: unexpected frontend verdict: {message}')
                if not expected:
                    if compiled.returncode != 5 or 'foreign call must supply every checked ABI argument' not in message:
                        raise bootstrap.BootstrapError(f'{label} {case["name"]}: refusal changed category: {message}')
                    continue
                link = ['clang', f'-O{args.opt_level}', str(ir), str(fixture_object), str(provider['object']), str(provider['runtime']), '-o', str(executable), '-ldl', '-lpthread', '-lm']
                linked = subprocess.run(link, capture_output=True, text=True, env=bootstrap.environment())
                (directory / 'link.log').write_text(linked.stdout + linked.stderr)
                linked.check_returncode()
                row.update(llvm_sha256=bootstrap.file_hash(ir), executable_sha256=bootstrap.file_hash(executable), link_command=link)
                for debug in [False, True]:
                    run = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
                        env={**os.environ, 'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M',
                             'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1'})
                    row['runs'].append({'debug': debug, 'exit': run.returncode, 'stderr': run.stderr})
                    bootstrap.write_json(args.output, report)
                    if run.returncode or 'leaked=0' not in run.stderr:
                        raise bootstrap.BootstrapError(f'{label} {case["name"]}: runtime observation failed: {run.stderr}')
                print(label, case['name'], 'passed both ownership profiles', flush=True)
        report['status'] = 'passed'
        bootstrap.write_json(args.output, report)
    except Exception as error:
        report.update(status='failed', failure=str(error))
        bootstrap.write_json(args.output, report)
        raise


if __name__ == '__main__':
    main()
