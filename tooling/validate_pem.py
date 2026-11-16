#!/usr/bin/env python3
"""Validate source-distributed PEM public consumers against explicit matched compiler/base artifacts."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import tempfile
import time
from render_pem_consumer import rendered

ROOT = Path(__file__).resolve().parents[1]
ARTIFACT = Path('/workspace/dasl-stdlib-string-combined/build/combined-base-artifact-v1')
STAGE3 = ARTIFACT.parent / 'combined-stage3-frozen'
CONFIG = STAGE3
NATIVE = STAGE3 / 'cinch'


def digest(path: Path) -> str:
    """Identify exact compiler, provider, source, corpus and executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def host_stack() -> None:
    """Keep the compiler-host stack explicit without changing the consumer's eight-MiB stack."""
    resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))


def source_closure(project: Path, main: Path, embedded: set[str]) -> list[Path]:
    """Copy only source imports, preserving the selected embedded family without source shadows."""
    pending = re.findall(r'^\s*import\s+([A-Za-z0-9_.]+)', main.read_text(), re.M)
    found, paths = set(), []
    while pending:
        name = pending.pop()
        if name in embedded or name in found: continue
        found.add(name)
        original = ROOT / 'compiler/prelude' / (name.replace('.', '/') + '.cinch')
        target = project / (name + '.cinch'); target.write_bytes(original.read_bytes()); paths.append(target)
        pending.extend(re.findall(r'^\s*import\s+([A-Za-z0-9_.]+)', target.read_text(), re.M))
    if any((project / (name + '.cinch')).exists() for name in embedded): raise ValueError('Refusing embedded source shadow')
    return paths


def main() -> None:
    """Compile once, preserve identities, and separate complete live-space from bounded poisoned coverage."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--compiler', type=Path)
    parser.add_argument('--provider', type=Path, default=STAGE3 / 'provider/provider.json')
    parser.add_argument('--interfaces', type=Path, default=CONFIG / 'base-interfaces.json')
    parser.add_argument('--objects', type=Path, default=CONFIG / 'objects.json')
    parser.add_argument('--runtime', type=Path, default=STAGE3 / 'libcinchruntime.a')
    parser.add_argument('--source', type=Path, help='Standalone public controls instead of the independent corpus')
    parser.add_argument('--module-override',type=Path,help='Isolated Text.Pem payload-batching candidate')
    parser.add_argument('--cpu', type=int, default=2)
    parser.add_argument('--opt-level', choices=['0', '2'], default='2')
    parser.add_argument('--group', type=int, nargs=2, metavar=('MODE', 'INDEX'))
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--compile-only', action='store_true')
    parser.add_argument('--reuse', action='store_true')
    args = parser.parse_args(); os.sched_setaffinity(0, {args.cpu})
    compiler = args.compiler or (ARTIFACT / 'cinch' if args.frontend == 'rust' else NATIVE)
    compiler = compiler.resolve(); provider = json.loads(args.provider.read_text())
    interfaces = json.loads(args.interfaces.read_text())['interfaces']; objects = json.loads(args.objects.read_text())
    if digest(args.runtime) != provider['runtime']['sha256']: raise ValueError('Runtime/provider mismatch')
    if args.frontend == 'rust' and digest(compiler) != provider['producer']['compiler_sha256']: raise ValueError('Rust embedded producer mismatch')
    for key in ['object', 'metadata']:
        if digest(Path(objects[key])) != objects[key + '_sha256']: raise ValueError('Admitted object/metadata bytes changed')
    fixture_path = args.source or ROOT / 'compiler/tests/fixtures/pem-vectors.json'
    fixture = json.loads(fixture_path.read_text()) if args.source is None else None
    counts = [(len(fixture[key]) + 7) // 8 for key in ['encodes', 'decodes']] if fixture else []
    if args.group and (args.source or not 0 <= args.group[0] < 2 or not 0 <= args.group[1] < counts[args.group[0]]): parser.error('invalid corpus group')
    artifacts = args.artifacts.resolve(); artifacts.mkdir(parents=True, exist_ok=True)
    temporary = tempfile.TemporaryDirectory(prefix='reuse-source-check-', dir=artifacts) if args.reuse else None
    project = Path(temporary.name) if temporary else artifacts / 'project'; project.mkdir(parents=True, exist_ok=True)
    source = project / 'Main.cinch'; source.write_text(args.source.read_text() if args.source else rendered())
    paths = source_closure(project, source, {row[0] for row in interfaces}) + [source]
    if args.module_override:(project/'Text.Pem.cinch').write_bytes(args.module_override.read_bytes())
    exe = artifacts / 'consumer'; ir = artifacts / 'consumer.ll'; identity_path = artifacts / 'build-identity.json'
    identity = {'frontend':args.frontend,'opt_level':args.opt_level,'compiler_sha256':digest(compiler),'runtime_sha256':digest(args.runtime),'provider_sha256':digest(args.provider),'interfaces_sha256':digest(args.interfaces),'objects_sha256':digest(args.objects),'object_sha256':objects['object_sha256'],'metadata_sha256':objects['metadata_sha256'],'fixture_sha256':digest(fixture_path),'sources':{path.name:digest(path) for path in paths},'source_shadow_used':False}
    started = time.monotonic()
    if args.reuse:
        prior = json.loads(identity_path.read_text())
        if any(prior.get(key) != value for key, value in identity.items()) or prior['executable_sha256'] != digest(exe): raise ValueError('Refusing reuse after an exact identity change')
        compile_command, link_command = prior['compile_command'], prior['link_command']
    else:
        if args.frontend == 'rust': compile_command = [str(compiler),'emit-ir',str(source),'-o',str(ir),'--rc-debug']
        else:
            bundle = artifacts / 'bundle.cinch'; bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
            compile_command = [str(compiler),'--emit-ir','--rc-debug','--interfaces',str(args.interfaces.resolve()),'--objects',str(args.objects.resolve()),str(bundle),str(ir)]
        result = subprocess.run(compile_command,cwd=project,capture_output=True,text=True,preexec_fn=host_stack)
        (artifacts/'compile.stdout').write_text(result.stdout);(artifacts/'compile.stderr').write_text(result.stderr)
        if result.returncode:
            diagnostic=ir.with_name(ir.name+'.diagnostics')
            raise RuntimeError((diagnostic.read_text() if diagnostic.exists() else result.stdout)+'\n'+result.stderr)
        link_command=['clang','-O'+args.opt_level,str(ir)]
        if args.frontend=='native':link_command.append(objects['object'])
        link_command += [str(args.runtime.resolve()),'-ldl','-lpthread','-lm','-o',str(exe)]
        result=subprocess.run(link_command,capture_output=True,text=True)
        (artifacts/'link.stdout').write_text(result.stdout);(artifacts/'link.stderr').write_text(result.stderr)
        if result.returncode:raise RuntimeError(result.stderr)
    compile_seconds=time.monotonic()-started
    identity.update({'compile_command':compile_command,'link_command':link_command,'executable_sha256':digest(exe),'llvm_sha256':digest(ir)})
    identity_path.write_text(json.dumps(identity,indent=2)+'\n')
    if args.compile_only:print(f'{args.frontend} PEM consumer compiled',flush=True);return
    groups = [[]] if args.source else [[str(mode),str(index)] for mode,count in enumerate(counts) for index in range(count)]
    if args.group:groups=[[str(value) for value in args.group]]
    selections=[(False,groups[0] if args.group else [])]+[(True,group) for group in groups]
    runs=[]
    for poisoned,group in selections:
        env={**os.environ,'CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M'}
        started=time.monotonic();result=subprocess.run([str(exe),*group],capture_output=True,text=True,env=env)
        if result.returncode or 'leaked=0' not in result.stderr:raise RuntimeError(f'group={group}, poisoned={poisoned}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        runs.append({'group':group,'poisoned':poisoned,'heap':env['CINCH_HEAP_LIMIT'],'stack':'8M','seconds':time.monotonic()-started,'stderr':result.stderr})
    report={**identity,'compiler':str(compiler),'producer':provider['producer'],'runtime':str(args.runtime.resolve()),'compiler_host_stack_mib':64,'compile_seconds':compile_seconds,'cpu':args.cpu,'complete_corpus':args.source is None and args.group is None,'runs':runs,'all_passed':True}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print(f'{args.frontend} PEM passed {len(runs)} bounded-memory runs',flush=True)


if __name__=='__main__':main()
