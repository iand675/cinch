#!/usr/bin/env python3
"""Exercise optional proof-body authority against real interface closures and independent damage."""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import time

from bootstrap_interfaces import canonical, interface_hash

ROOT = Path(__file__).resolve().parents[1]
NAT_ADD = 'Data.Nat.Add_Nat.+'


def sha(path: Path) -> str:
    """Identify exact source, compiler, runtime and retained observation files."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, value: object) -> None:
    """Retain a readable observation after each completed boundary."""
    path.write_text(json.dumps(value, indent=2) + '\n')


def loader_sources() -> dict[str, bytes]:
    """Collect the actual loader implementation and its source dependency closure."""
    pending = ['Bootstrap.Load']
    files: dict[str, bytes] = {}
    while pending:
        name = pending.pop()
        if name in files:
            continue
        path = ROOT / 'bootstrap' / f'{name}.cinch' if name.startswith('Bootstrap.') else ROOT / 'compiler/prelude' / Path(*name.split('.')).with_suffix('.cinch')
        if not path.is_file():
            continue
        files[name] = path.read_bytes()
        pending.extend(re.findall(r'^  import (Bootstrap\.[A-Za-z]+|Text\.(?:Lex|Scan|Parse|Json|Builder))', files[name].decode(), re.M))
    files['Main'] = (ROOT / 'bootstrap/programs/Run.BodyClosure.cinch').read_bytes()
    return files


def prepare(interfaces: Path, artifacts: Path) -> dict:
    """Build fixture observations from unchanged admitted documents without trimming imports."""
    admitted = json.loads(interfaces.read_text())['interfaces']
    rows = {name: [name, str(identity), path] for name, identity, path in admitted}
    documents = {name: json.loads(Path(path).read_text()) for name, identity, path in admitted}
    fixtures: list[dict] = []

    def fixture(name: str, selected: dict, *, count: int | None = None,
                present: tuple[str, ...] = (), absent: tuple[str, ...] = (),
                body: str | None = 'NatAdd', error: str | None = None,
                known: list | None = None, verified: bool = True) -> None:
        """State an external semantic expectation for one independent loader observation."""
        manifest = {'inputs': list(selected.values()), 'expected_error': error,
                    'member_count': count, 'present_members': list(present),
                    'absent_members': list(absent), 'bodies': [[NAT_ADD, body]]}
        if known is not None:
            manifest.update(known_inputs=[known], known_verified=verified)
        path = artifacts / f'{name}.json'
        write_json(path, manifest)
        fixtures.append({'name': name, 'manifest': str(path), 'manifest_sha256': sha(path), 'expected': manifest})

    def replace(selected: dict, home: str, document: dict, label: str) -> dict:
        """Write a separate damaged document with its honestly recomputed identity."""
        path = artifacts / f'{label}.cinchif'
        path.write_text(canonical(document))
        return {**selected, home: [home, interface_hash(document), str(path)]}

    fixture('intact', rows, count=65, present=('Data.Nat', 'Data.Machine', 'Prelude'))
    fixture('intact-known-row', rows, count=len(rows), present=('Data.Nat',), known=rows['Data.Nat'])
    projected = {k: v for k, v in rows.items() if k not in {'Data.Json', 'Text.Codec'}}
    fixture('source-overlays', projected, count=48, present=('Data.Nat', 'Data.Hash'),
            absent=('Data.Json', 'Text.Codec', 'Data.Bool', 'Data.Machine', 'Prelude'))
    fixture('shadowed-defining-home', {k: v for k, v in rows.items() if k != 'Data.Nat'},
            present=('Data.Core',), absent=('Data.Nat',), body=None)
    fixture('shadowed-transitive-dependency', {k: v for k, v in rows.items() if k != 'Data.Hash'},
            present=('Data.Core',), absent=('Data.Hash', 'Data.Nat'), body=None)

    stale = copy.deepcopy(documents['Data.Nat'])
    next(row for row in stale['imports'] if row['module'] == 'Data.Hash')['iface_hash'] = '0'
    fixture('mismatched-dependency', replace(rows, 'Data.Nat', stale, 'stale-nat-dependency'),
            present=('Data.Core',), absent=('Data.Nat',), body=None)

    bad_hash = copy.deepcopy(projected)
    bad_hash['Data.Machine'][1] = '0'
    fixture('bad-hash-in-excluded-home', bad_hash, error='Data.Machine: interface contract identity mismatch')

    fixture('unverified-claimed-identity', bad_hash, known=bad_hash['Data.Machine'], verified=False,
            error='Data.Machine: interface contract identity mismatch')
    changed = {**rows, 'Data.Nat': [*rows['Data.Nat'][:2], str(artifacts / 'stale-nat-dependency.cinchif')]}
    fixture('changed-since-admission', changed, known=rows['Data.Nat'],
            error='Data.Nat: interface contract identity mismatch')

    malformed = copy.deepcopy(documents['Data.Machine'])
    malformed['exports']['values'][0]['total_body'] = {'version': 999, 'definitions': []}
    fixture('bad-body-in-excluded-home', replace(projected, 'Data.Machine', malformed, 'malformed-machine-body'),
            error='Data.Machine: malformed interface contract envelope')

    forged = copy.deepcopy(documents['Data.Nat'])
    addition = next(row for row in forged['exports']['instances'] if row['id'] == 'Data.Nat.Add_Nat')
    graph = addition['method_bodies'][0]['total_body']
    foreign = copy.deepcopy(graph['definitions'][0])
    foreign['identity'] = 'Outside.evil'
    graph['definitions'][0]['body'] = {'node': 'call', 'target': foreign['identity'],
                                       'arguments': [{'node': 'local', 'slot': 0},
                                                     {'node': 'local', 'slot': 1}]}
    graph['definitions'].append(foreign)
    fixture('forged-body-dependency', replace(rows, 'Data.Nat', forged, 'forged-nat-body'),
            error='Data.Nat: total body dependency is outside the checked module closure: Outside.evil')

    nominal = copy.deepcopy(documents['Data.Option'])
    mention = next(row for row in nominal['exports']['instances'][0]['resolved']['nominals'] if row['identity'] == 'Data.Core.Option')
    previous = mention['constructors'][0]
    mention['constructors'][0] = 'Data.Core.ForgedNone'
    for usage in mention.get('usage', []):
        if usage['identity'] == previous:
            usage['identity'] = 'Data.Core.ForgedNone'
    fixture('conflicting-retained-nominal', replace(rows, 'Data.Option', nominal, 'forged-option-nominal'),
            error='Data.Option: conflicting defining nominal contracts: Data.Core.Option')

    files = loader_sources()
    bundle = artifacts / 'Bundle.cinch'
    bundle.write_bytes(b'\n\n'.join(b'-- source: ' + name.encode() + b'.cinch\n' + data for name, data in sorted(files.items())) + b'\n')
    return {'status': 'prepared', 'validator_source_sha256': sha(Path(__file__)),
            'interfaces_sha256': sha(interfaces), 'fixtures': fixtures,
            'input_documents_sha256': {name: sha(Path(path)) for name, identity, path in admitted},
            'damaged_documents_sha256': {path.name: sha(path) for path in artifacts.glob('*.cinchif')},
            'source_files': {name: hashlib.sha256(data).hexdigest() for name, data in sorted(files.items())},
            'source_bytes': sum(map(len, files.values())), 'bundle': str(bundle), 'bundle_sha256': sha(bundle),
            'ordinary_admission_changed': False, 'runtime_profiles': [], 'steps': []}


def main() -> None:
    """Prepare independently stated cases or run them with an explicit compiler/runtime pair."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--interfaces', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--compiler', type=Path)
    parser.add_argument('--runtime', type=Path)
    parser.add_argument('--provenance', type=Path,
                        help='Frozen producer receipt; defaults to provenance.json beside the compiler')
    parser.add_argument('--clang', type=Path,
                        help='Explicit LLVM linker driver; defaults to CINCH_CLANG or the current toolchain path')
    parser.add_argument('--prepare-only', action='store_true')
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    report = prepare(args.interfaces.resolve(), args.artifacts)
    output = args.artifacts / 'report.json'
    write_json(output, report)
    if args.prepare_only:
        print(json.dumps({'source_modules': len(report['source_files']), 'source_bytes': report['source_bytes'], 'fixtures': len(report['fixtures'])}))
        return
    if args.compiler is None or args.runtime is None:
        parser.error('--compiler and --runtime are required without --prepare-only')
    provenance_path = args.provenance or args.compiler.resolve().parent / 'provenance.json'
    provenance = json.loads(provenance_path.read_text())
    compiler_hash, runtime_hash = sha(args.compiler), sha(args.runtime)
    if provenance.get('compiler_sha256') != compiler_hash or provenance.get('runtime_sha256') != runtime_hash:
        raise SystemExit('compiler/runtime hashes disagree with the explicit frozen provenance')
    selected_clang = args.clang or os.environ.get('CINCH_CLANG') or shutil.which('clang')
    if selected_clang is None:
        raise SystemExit('no LLVM linker driver is available; provide --clang or CINCH_CLANG')
    clang = Path(selected_clang).resolve()
    if not clang.is_file():
        raise SystemExit(f'explicit LLVM linker driver is missing: {clang}')
    report.update(compiler_sha256=compiler_hash, runtime_sha256=runtime_hash,
                  producer_provenance=provenance, producer_provenance_sha256=sha(provenance_path),
                  clang=str(clang), clang_sha256=sha(clang), consumer_opt_level=0,
                  compiler_host_profile='frozen Rust CLI; no experimental compiler-family claim', consumer_rc_debug_compiled=False)
    report['status'] = 'focused-loader-gates-running'
    llvm = args.artifacts / 'consumer.ll'
    executable = args.artifacts / 'consumer'
    environment = {**os.environ, 'CARGO_BUILD_JOBS': '1', 'RUST_TEST_THREADS': '1', 'RUST_MIN_STACK': '67108864', 'CARGO_INCREMENTAL': '0', 'CINCH_HEAP_LIMIT': '2G', 'CINCH_STACK_SIZE': '64M'}
    commands = [('emit', [str(args.compiler.resolve()), 'emit-ir', report['bundle'], '-o', str(llvm)]),
                ('link', [str(clang), '-O0', '-Wno-override-module', str(llvm), str(args.runtime.resolve()), '-ldl', '-lpthread', '-lm', '-o', str(executable)])]
    for label, command in commands:
        before = time.monotonic()
        run = subprocess.run(['taskset', '-c', str(args.cpu), *command], cwd=args.artifacts, env=environment, capture_output=True, text=True)
        (args.artifacts / f'{label}.log').write_text(run.stdout + run.stderr)
        report['steps'].append({'name': label, 'command': command, 'exit': run.returncode, 'seconds': time.monotonic() - before})
        write_json(output, report)
        if run.returncode:
            report['status'] = f'{label}-failed'
            write_json(output, report)
            raise SystemExit(f'{label} failed; preserved {output}')
    report.update(llvm_sha256=sha(llvm), executable_sha256=sha(executable))
    for case in report['fixtures']:
        for poisoned in (False, True):
            env = {**environment, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '1' if poisoned else '0'}
            run = subprocess.run(['taskset', '-c', str(args.cpu), str(executable), case['manifest']], cwd=args.artifacts, env=env, capture_output=True, text=True)
            label = case['name'] + ('-poison' if poisoned else '-normal')
            (args.artifacts / f'{label}.log').write_text(run.stdout + run.stderr)
            detail = args.artifacts / 'body-closure-detail.txt'
            report['runtime_profiles'].append({'name': label, 'exit': run.returncode, 'stderr': run.stderr,
                                              'unexpected_detail': detail.read_text() if detail.exists() else None})
            write_json(output, report)
            if run.returncode or 'leaked=0' not in run.stderr:
                report['status'] = 'focused-loader-observation-failed'
                report['failed_profile'] = label
                write_json(output, report)
                raise SystemExit(f'{label} failed; preserved {output}')
    report['status'] = 'focused-loader-gates-passed'
    write_json(output, report)
    print(output)


if __name__ == '__main__':
    main()
