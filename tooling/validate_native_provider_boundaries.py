#!/usr/bin/env python3
"""Qualify actual Native provider calls for independent full-width negation and conversion boundaries."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects


def main() -> None:
    """Keep compiler-host and consumer runtimes distinct while observing exact checked API results."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--compiler-build-receipt', type=Path, required=True)
    parser.add_argument('--compiler-host-runtime-receipt', type=Path, required=True,
                        help='prior provider receipt identifies the statically linked compiler-host runtime only')
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    build = bootstrap.read_json(args.compiler_build_receipt)
    host_receipt = bootstrap.read_json(args.compiler_host_runtime_receipt)
    host_runtime = args.compiler_host_runtime_receipt.resolve().parent / 'libcinchruntime.a'
    if (bootstrap.file_hash(args.compiler) != build['compiler_sha256']
            or bootstrap.file_hash(host_runtime) != build['runtime_sha256']
            or host_receipt['runtime']['sha256'] != build['runtime_sha256']):
        raise bootstrap.BootstrapError('compiler host identity differs from its retained build/runtime receipt')
    if (host_receipt['runtime']['runtime_abi'] != provider['identity']['runtime']['runtime_abi']
            or host_receipt['runtime']['source_sha256'] != provider['identity']['runtime']['source_sha256']):
        raise bootstrap.BootstrapError('compiler-host and consumer runtime source/ABI compatibility is not established')
    interfaces, objects = provider['interface_package'], args.artifacts / 'objects.json'
    bootstrap.write_json(objects, provider['configuration'])
    report = {'status': 'running', 'native_family_admission': False, 'performance_claim': False,
              'source_shadow_used': False, 'compiler_sha256': build['compiler_sha256'],
              'compiler_host_opt_level': build['compiler_host_opt_level'],
              'compiler_source_sha256': build['compiler_source_sha256'],
              'compiler_host_runtime': host_receipt['runtime'], 'consumer_runtime': provider['identity']['runtime'],
              'compiler_host_build_receipt_sha256': bootstrap.file_hash(args.compiler_build_receipt),
              'compiler_host_runtime_receipt_sha256': bootstrap.file_hash(args.compiler_host_runtime_receipt),
              'provider': provider['identity'], 'groups': []}
    bootstrap.write_json(args.output, report)
    for name, relative, observations, ordinary in [
        ('public-full-width', 'word-constant-folding.cinch', 33, ['saturating-neg-u64']),
        ('cross-signed', 'native-provider-cross-signed-boundaries.cinch', 6,
         ['checked-u64-to-i64', 'saturating-u64-to-i64', 'checked-i64-to-u64', 'saturating-i64-to-u64'])]:
        directory = args.artifacts / name
        directory.mkdir()
        source, ir = directory / 'Main.cinch', directory / 'consumer.ll'
        source.write_bytes((bootstrap.ROOT / 'seed/tests/fixtures' / relative).read_bytes())
        command = [str(args.compiler), '--emit-ir', '--rc-debug', '--interfaces', str(interfaces),
                   '--objects', str(objects), str(source), str(ir)]
        result = subprocess.run(['taskset', '-c', str(args.cpu), *command], cwd=directory,
            env={**bootstrap.environment(), 'CINCH_STACK_SIZE': '64M', 'CINCH_LEAK_CHECK': '1'}, capture_output=True, text=True)
        (directory / 'compile.log').write_text(result.stdout + result.stderr)
        group = {'name': name, 'observations': observations, 'source_sha256': bootstrap.file_hash(source),
                 'compile_command': command, 'compile_exit': result.returncode, 'runs': []}
        report['groups'].append(group)
        bootstrap.write_json(args.output, report)
        result.check_returncode()
        text = ir.read_text()
        group['ordinary_provider_calls'] = {}
        for binding in ordinary:
            encoded = ''.join('\\' + format(byte, '02X') for byte in ('Data.Machine.' + binding).encode())
            retained = any('call i64' in line and encoded in line for line in text.splitlines())
            if not retained:
                raise bootstrap.BootstrapError('boundary test did not retain its ordinary provider call: ' + binding)
            group['ordinary_provider_calls'][binding] = True
        group['llvm_sha256'] = bootstrap.file_hash(ir)
        for opt in [0, 2]:
            executable = directory / f'consumer-o{opt}'
            command = [os.environ.get('CINCH_CLANG', 'clang'), f'-O{opt}', str(ir), str(provider['object']),
                       str(provider['runtime']), '-ldl', '-lpthread', '-lm', '-o', str(executable)]
            result = subprocess.run(['taskset', '-c', str(args.cpu), *command], capture_output=True, text=True)
            (directory / f'link-o{opt}.log').write_text(result.stdout + result.stderr)
            result.check_returncode()
            for debug in [False, True]:
                result = subprocess.run(['taskset', '-c', str(args.cpu), str(executable)], capture_output=True, text=True,
                    env={**os.environ, 'CINCH_STACK_SIZE': '8M', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                         'CINCH_RC_DEBUG': str(int(debug)), 'CINCH_LEAK_CHECK': '1'})
                group['runs'].append({'optimization': opt, 'poisoned': debug, 'exit': result.returncode,
                    'stderr': result.stderr, 'executable_sha256': bootstrap.file_hash(executable), 'link_command': command})
                bootstrap.write_json(args.output, report)
                if result.returncode or 'leaked=0' not in result.stderr:
                    raise bootstrap.BootstrapError('Native provider boundary observation failed: ' + name)
        print(name + ': exact provider observations passed O0/O2 ordinary/poisoned.', flush=True)
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)


if __name__ == '__main__':
    main()
