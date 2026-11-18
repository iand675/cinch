"""Inspect actual optimized workers, exercise guarded bounds, and emit foreign objects."""
from pathlib import Path
import argparse
import hashlib
import json
import re
import resource
import shutil
import subprocess


def digest(path):
    """Identify the exact artifact observed by a machine qualification step."""
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def definition(ir, name):
    """Extract one exact function rather than matching unrelated declarations."""
    marker = re.compile(r'@(?:"' + re.escape(name) + r'"|' + re.escape(name) + r')\(')
    start = next(m.start() for m in re.finditer(r'^define .*', ir, re.M)
                 if marker.search(m.group()))
    return ir[start:ir.index('\n}', start) + 2]


def audit_traversal(body):
    """Reject data-dependent branches, addresses, calls, and writes in a private loop.

    The optimized SSA graph has public pointer/offset arguments and a secret
    accumulated difference in p4. Every loaded byte is secret. This bounded
    intraprocedural audit covers these concrete workers, not arbitrary LLVM.
    """
    assert not re.search(r'\b(?:alloca|call|invoke|store|switch)\b', body), body
    tainted = {'%p4'}
    assignments = re.findall(r'^\s*(%[\w.]+) = (.*)$', body, re.M)
    while True:
        prior = tainted.copy()
        for result, expression in assignments:
            operands = set(re.findall(r'%[\w.]+', expression))
            if expression.startswith('load ') or operands & tainted:
                tainted.add(result)
        if tainted == prior:
            break
    branches = re.findall(r'\bbr i1 (%[\w.]+)', body)
    assert not set(branches) & tainted, (branches, tainted)
    for result, expression in assignments:
        if re.search(r'\b(?:getelementptr|inttoptr)\b', expression):
            assert result not in tainted, expression
        if expression.startswith('load '):
            address = re.search(r'\bptr (%[\w.]+)', expression)
            assert address and address.group(1) not in tainted, expression
    assert branches and any('load ' in expression for _, expression in assignments)
    return {'public_conditional_branches': len(branches),
            'secret_conditional_branches': 0, 'secret_address_dependencies': 0}


def main():
    """Record every tool result before asserting its exact supported boundary."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('project', type=Path)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--clang', required=True)
    parser.add_argument('--resume', action='store_true')
    args = parser.parse_args()
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK, (max(soft, 64 * 1024 * 1024), hard))
    project = args.project.resolve()
    root = Path(__file__).resolve().parents[1]
    work = project / 'machine'
    work.mkdir(exist_ok=True)
    report = {'compiler_sha256': digest(args.compiler),
              'runtime_sha256': digest(args.runtime),
              'source_llvm_sha256': digest(project / 'consumer.ll'),
              'compiler_stack_bytes': 67108864,
              'scope': 'Concrete host SSA/assembly audit and guarded private workers; '
                       'foreign targets are object emission, not execution or timing proof.',
              'commands': [], 'guards': [], 'objects': []}

    def save():
        """Preserve completed observations and any pending failure for review."""
        (work / 'results.json').write_text(json.dumps(report, indent=2) + '\n')

    def run(command, label):
        """Retain exact command, output, and exit before enforcing success."""
        row = {'command': list(map(str, command)), 'label': label}
        report['commands'].append(row)
        save()
        result = subprocess.run(command, capture_output=True, text=True, timeout=90)
        row.update(exit=result.returncode, stdout=result.stdout, stderr=result.stderr)
        save()
        assert result.returncode == 0, row
        return result

    if args.resume:
        previous = json.loads((work / 'results.json').read_text())
        for key in ['compiler_sha256', 'runtime_sha256', 'source_llvm_sha256']:
            assert previous[key] == report[key], key
        assert len(previous['guards']) == 2
        assert previous['guard_source_sha256'] == digest(root / 'compiler/tests/fixtures/constant-time-guard.c')
        assert previous['optimized_llvm_sha256'] == digest(work / 'optimized.ll')
        assert previous['optimized_assembly_sha256'] == digest(work / 'optimized.s')
        for guard in previous['guards']:
            assert guard['executable_sha256'] == digest(work / f"guard-o{guard['optimization']}")
        report = previous
    else:
        ir = work / 'optimized.ll'
        assembly = work / 'optimized.s'
        run([args.clang, '-O2', '-S', '-emit-llvm', project / 'consumer.ll', '-o', ir], 'optimized-ssa')
        run([args.clang, '-O2', '-S', project / 'consumer.ll', '-o', assembly], 'optimized-assembly')
        optimized = ir.read_text()
        report['traversal_audit'] = {
            name: audit_traversal(definition(optimized, 'Crypto.ConstantTime.' + name))
            for name in ['difference-blocks', 'difference-tail']}
        report['optimized_llvm_sha256'] = digest(ir)
        report['optimized_assembly_sha256'] = digest(assembly)
        # Both comparison loops are retained exported definitions in this artifact.
        assert 'movdqu' in assembly.read_text() and 'pxor' in assembly.read_text()
        scalar_names = ['constant-byte-equal', 'constant-i32-equal', 'constant-select',
                        'less-or-equal-word', 'select-word', 'select-byte', 'zero-word']
        report['scalar_audit'] = {}
        for name in scalar_names:
            body = definition(optimized, 'Crypto.ConstantTime.' + name + '.llvm')
            assert not re.search(r'\b(?:br|switch|call|invoke|load|store)\b', body), body
            report['scalar_audit'][name] = 'straight-line SSA; no branch, call, or memory access'
        guard = root / 'compiler/tests/fixtures/constant-time-guard.c'
        report['guard_source_sha256'] = digest(guard)
        for level in [0, 2]:
            executable = work / f'guard-o{level}'
            run([args.clang, f'-O{level}', project / 'consumer.ll', guard,
                 args.runtime.resolve(), '-Wl,--wrap=main', '-lpthread', '-ldl', '-lm',
                 '-o', executable], f'guard-link-o{level}')
            result = run([executable], f'guard-run-o{level}')
            assert result.stdout.strip() == '133644 guard observations passed', result.stdout
            report['guards'].append({'optimization': level,
                                     'executable_sha256': digest(executable),
                                     'observations': 133644})
    for target in ['wasm32-unknown-unknown', 'aarch64-unknown-linux-gnu']:
        directory = work / target
        directory.mkdir(exist_ok=True)
        for name in ['Main.cinch', 'Crypto.ConstantTime.cinch', 'Data.Bytes.Word.cinch']:
            shutil.copyfile(project / name, directory / name)
        target_ir, obj = directory / 'consumer.ll', directory / 'consumer.o'
        run([args.compiler.resolve(), 'emit-ir', directory / 'Main.cinch',
             '--target', target, '-o', target_ir], target + '-emit')
        target_text = target_ir.read_text()
        assert 'load <16 x i8>' in definition(target_text, 'Crypto.ConstantTime.difference-sixteen.llvm')
        assert 'load i8' in definition(target_text, 'Crypto.ConstantTime.difference-byte.llvm')
        run([args.clang, '-target', target, '-O2', '-c', target_ir, '-o', obj], target + '-object')
        report['objects'].append({'target': target, 'llvm_sha256': digest(target_ir),
                                  'object_sha256': digest(obj)})
    report['status'] = 'passed'
    save()
    print(json.dumps({'guards': report['guards'], 'objects': report['objects'], 'status': 'passed'}))


if __name__ == '__main__':
    main()
