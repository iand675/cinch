#!/usr/bin/env python3
"""Check real mixed-effect field contracts and their deliberate effect-erasure refusal with an exact Native pair."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import shutil
import subprocess

from bench_stdlib import source_closure
from validate_byte_view import digest

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Preserve every source, emitted unit and diagnostic without linking or claiming a runtime result."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path, required=True)
    parser.add_argument('--objects', type=Path, required=True)
    parser.add_argument('--tuple-context', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--negative', action='store_true')
    args = parser.parse_args()
    context = json.loads(args.tuple_context.read_text())
    for file, expected in context['tuple_files'].items():
        assert digest(Path(file)) == expected, file
    assert digest(Path(context['raw_compiler']['path'])) == context['raw_compiler']['sha256']
    for file in [args.compiler, args.interfaces, args.objects]:
        assert digest(file) == context['tuple_files'][str(file.resolve())]
    work = args.artifacts.resolve()
    work.mkdir(parents=True, exist_ok=True)
    fixture = ROOT/'seed/tests/fixtures'/('byte-fields-row-negative.cinch' if args.negative else 'byte-fields-row-positive.cinch')
    source = fixture.read_text()
    entry = work/'Main.cinch'
    entry.write_text(source)
    closure = source_closure(source)
    for name, path in closure.items():
        shutil.copyfile(path, work/(name+'.cinch'))
    bundle, ir = work/'Bundle.cinch', work/'consumer.ll'
    bundle.write_text('\n'.join(path.read_text() for path in reversed(list(closure.values())))+'\n'+source)
    command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.interfaces.resolve()),
               '--objects', str(args.objects.resolve()), str(bundle), str(ir)]
    report = dict(scope='source-only actual effect row contract', intended_acceptance=not args.negative,
                  compiler_sha256=digest(args.compiler), interfaces_sha256=digest(args.interfaces),
                  objects_sha256=digest(args.objects), tuple_context_sha256=digest(args.tuple_context),
                  fixture_sha256=digest(fixture), bundle_sha256=digest(bundle),
                  dependencies={name:digest(path) for name,path in closure.items()}, command=command,
                  compiler_main_stack_bytes=67108864, runtime_executed=False, full_family_adoption=False,
                  status='prepared')
    def save() -> None:
        """Record the checked stage before the command and before interpreting its exact result."""
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+'\n')
    save()
    result = subprocess.run(command, capture_output=True, text=True)
    diagnostic_file = ir.with_suffix('.ll.diagnostics')
    detail = result.stdout+result.stderr+(diagnostic_file.read_text() if diagnostic_file.exists() else '')
    (work/'compile.log').write_text(detail)
    report.update(exit_code=result.returncode, diagnostics=detail, status='returned-before-verdict')
    save()
    if args.negative:
        accepted = result.returncode != 0 and 'effect rows do not match' in detail
    else:
        accepted = result.returncode == 0 and ir.is_file()
        if ir.is_file():
            report['llvm_sha256'] = digest(ir)
    report['status'] = 'passed' if accepted else 'unexpected-verdict'
    save()
    assert accepted, detail


if __name__ == '__main__':
    main()
