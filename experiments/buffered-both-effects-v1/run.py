#!/usr/bin/env python3
"""Execute one exact positively source-gated composition fixture with bounded artifacts and matched tools."""
from pathlib import Path
import argparse
import json
import os
import resource
import shutil
import subprocess
import time
import check

MIB = 1024 * 1024


def main():
    """Keep the admitted source check, compiler epoch and two fixed consumer memory profiles explicit."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--checked', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=2)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    checked = args.checked.resolve()
    receipt_path = checked/'receipt.json'
    receipt = json.loads(receipt_path.read_text())
    assert receipt['returncode'] == 0 and not receipt['expected_effect_refusal']
    assert receipt['signature_manifest_sha256'] == check.digest(check.HERE/'signature-impact.json')
    project = checked/'project'
    for name, expected in receipt['sources'].items():
        assert check.digest(project/name) == expected, name
    compiler = Path(receipt['command'][0])
    snapshot = compiler.parent
    runtime = snapshot/'libcinchruntime.a'
    assert check.digest(compiler) == receipt['compiler_sha256']
    assert check.digest(runtime) == receipt['runtime_sha256']
    clang = Path(shutil.which('clang')).resolve()
    clang_version = subprocess.run([str(clang), '--version'], capture_output=True, text=True, check=True).stdout
    assert 'clang version 20.' in clang_version, clang_version
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    assert shutil.disk_usage(artifacts).free >= 250*MIB, 'standing 250 MiB start guard'
    ir, executable = artifacts/'consumer.ll', artifacts/'consumer'
    report = {'scope': 'One exact review-only composition runtime fixture; no API or collector promotion.',
              'source_receipt_sha256': check.digest(receipt_path), 'source_receipt': receipt,
              'clang': str(clang), 'clang_version': clang_version, 'opt_level': 2,
              'artifact_budget_bytes': 20*MIB, 'consumer_stack': '8M', 'cpu': args.cpu, 'steps': [], 'runs': []}

    def save():
        """Preserve partial diagnostics and completed outcomes before any next unit can start."""
        (artifacts/'receipt.json').write_text(json.dumps(report, indent=2)+'\n')

    def used():
        """Charge only this unit's actual outputs, keeping shared frozen source/tool artifacts untouched."""
        return sum(path.stat().st_size for path in artifacts.rglob('*') if path.is_file())

    def invoke(command, name, environment):
        """Limit each emitted file and enforce the remaining-output and shared disk reserve before compilation/linking."""
        allowance = 20*MIB-used()-128*1024
        assert allowance > 0 and shutil.disk_usage(artifacts).free >= 200*MIB+allowance

        def limits():
            """Keep the compiler host distinct from the fixed runtime stack and bound per-file output growth."""
            check.host_stack()
            resource.setrlimit(resource.RLIMIT_FSIZE, (allowance, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))

        before = time.monotonic()
        result = subprocess.run(command, cwd=project, env=environment, capture_output=True, text=True, preexec_fn=limits)
        (artifacts/(name+'.stdout')).write_text(result.stdout)
        (artifacts/(name+'.stderr')).write_text(result.stderr)
        diagnostic = ir.with_name(ir.name+'.diagnostics')
        report['steps'].append({'name': name, 'command': command, 'returncode': result.returncode,
                                'seconds': time.monotonic()-before, 'stdout': result.stdout, 'stderr': result.stderr,
                                'diagnostic': diagnostic.read_text() if diagnostic.exists() else ''})
        save()
        assert used() <= 20*MIB, 'unit output budget exceeded'
        if result.returncode:
            raise RuntimeError('first failing '+name+' result retained in receipt.json')

    host = {**os.environ, 'CINCH_HEAP_LIMIT': '2G', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'}
    if receipt['frontend'] == 'rust':
        command = [str(compiler), 'emit-ir', str(project/'Main.cinch'), '-o', str(ir), '--rc-debug']
        objects = []
    else:
        interfaces, object_manifest = snapshot/'base-interfaces.json', snapshot/'objects.json'
        assert check.digest(interfaces) == receipt['interfaces_sha256']
        provenance = receipt['provenance']
        assert check.digest(object_manifest) == provenance['files']['objects.json']['sha256']
        provider = json.loads(object_manifest.read_text())
        for name in ['object', 'metadata']:
            assert check.digest(Path(provider[name])) == provider[name+'_sha256']
        bundle = checked/'bundle.cinch'
        assert bundle.read_text() == '\n\n'.join(path.read_text() for path in sorted(project/name for name in receipt['sources']))
        command = [str(compiler), '--emit-ir', '--rc-debug', '--interfaces', str(interfaces), '--objects', str(object_manifest), str(bundle), str(ir)]
        objects = [provider['object']]
        report['provider'] = {'objects_sha256': check.digest(object_manifest), **provider}
    invoke(command, 'compile', host)
    invoke([str(clang), '-O2', str(ir), *objects, str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(executable)], 'link', os.environ)
    report.update(llvm_sha256=check.digest(ir), executable_sha256=check.digest(executable))
    save()
    for poisoned in [False, True]:
        environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M',
                       'CINCH_RC_DEBUG': '1' if poisoned else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
        result = subprocess.run([str(executable)], env=environment, capture_output=True, text=True)
        report['runs'].append({'poisoned': poisoned, 'command': [str(executable)], 'environment': {key: value for key, value in environment.items() if key.startswith('CINCH_')},
                               'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr})
        save()
        if result.returncode or 'live=0 immortal=0 leaked=0' not in result.stderr:
            raise RuntimeError('first runtime mismatch/leak retained in receipt.json')
    report['all_passed'] = True
    report['artifact_bytes'] = used()
    save()
    print('Exact '+receipt['frontend']+' composition unit passed normal/poison with zero live/leaks')


if __name__ == '__main__':
    main()
