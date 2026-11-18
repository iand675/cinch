#!/usr/bin/env python3
"""Test the actual candidate hexadecimal primitive with exhaustive scalar fixtures.

The precise inline LLVM body becomes a linked host object and real AArch64 and
wasm32 objects. The oracle uses independent literal table lookup. This does not
claim whole decoder or foreign-target execution qualification.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import shutil
from check_compare import digest, run


#: Repository-owned independent scalar C fixture.
ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Freeze exact inputs and stop on the first source, object or runtime failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(work).free < 350 * 1024 * 1024:
        raise RuntimeError('resource pause before any compile')
    matches = re.findall(r'prim hex-value-word :: U8 -> U32 where\s+llvm \{(.*?)\n    \}', args.source.read_text(), re.S)
    if len(matches) != 1:
        raise RuntimeError('missing unique actual classifier body')
    ir = work / 'hex.ll'; ir.write_text('define i32 @qp_hex_value(i8 %0) {\n' + matches[0] + '\n}\n')
    fixture = work / 'fixture.c'; shutil.copyfile(ROOT / 'compiler/tests/fixtures/quoted-printable-hex-kernel.c', fixture)
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'scope': __doc__, 'selected_source_sha256': digest(args.source), 'actual_ir_sha256': digest(ir),
              'fixture_sha256': digest(fixture), 'cpu_affinity': [args.cpu], 'commands': []}

    def execute(command: list) -> None:
        """Retain the first genuine diagnostic and all failed artifacts plain."""
        if shutil.disk_usage(work).free < 250 * 1024 * 1024 or sum(path.stat().st_size for path in work.rglob('*') if path.is_file()) > 30 * 1024 * 1024:
            raise RuntimeError('resource pause before process')
        row = {'command': command}; report['commands'].append(row)
        try:
            result = run(command, work, 60)
            row.update(stdout=result.stdout, stderr=result.stderr, passed=True)
        except Exception as error:
            row['failure'] = str(error)
            raise
        finally:
            (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    execute([clang, '-O2', '-c', str(ir), '-o', str(work / 'hex.o')])
    execute([clang, '-O2', '-fno-vectorize', '-fno-slp-vectorize', str(fixture), str(work / 'hex.o'), '-o', str(work / 'check')])
    execute([str(work / 'check')])
    execute([clang, '-O2', '-S', str(ir), '-o', str(work / 'hex.s')])
    for target in ['aarch64-unknown-linux-gnu', 'wasm32-unknown-unknown']:
        execute([clang, '-O2', '--target=' + target, '-c', str(ir), '-o', str(work / (target + '.o'))])
    report.update(byte_cases=256, pair_cases=65536, foreign_targets=['aarch64-unknown-linux-gnu', 'wasm32-unknown-unknown'],
                  artifacts={path.name: digest(path) for path in work.iterdir() if path.is_file() and path.name != 'report.json'},
                  portable_execution_claim=False)
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print('256 byte/65536 pair primitive cases and two real portable objects passed', flush=True)


if __name__ == '__main__':
    main()
