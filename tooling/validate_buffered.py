#!/usr/bin/env python3
"""Validate owned buffered-I/O consumers with explicit frozen compiler/runtime epochs and bounded memory."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import time

ROOT=Path(__file__).resolve().parents[1]
RUST=Path('/workspace/dasl-stdlib-io/build/array-frame-safe-frozen')
NATIVE=Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen')


def digest(path: Path) -> str:
    """Identify exact source, compiler, runtime, interface and generated artifact bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def host_stack() -> None:
    """Keep compiler-host stack explicit while consumer stack stays eight MiB."""
    resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))


def closure(project: Path, main: Path, embedded: set[str]) -> list[Path]:
    """Resolve source-distributed modules without shadowing an embedded base-family module."""
    pending=re.findall(r'^\s*import\s+([\w.]+)',main.read_text(),re.M);found=set();paths=[]
    while pending:
        name=pending.pop()
        if name in embedded or name in found:continue
        found.add(name);source=ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')
        target=project/(name+'.cinch');target.write_bytes(source.read_bytes());paths.append(target)
        pending.extend(re.findall(r'^\s*import\s+([\w.]+)',target.read_text(),re.M))
    if any((project/(name+'.cinch')).exists() for name in embedded):raise ValueError('embedded source shadow')
    return paths+[main]


def main() -> None:
    """Compile one frozen consumer, preserve the first diagnostic, then run both fixed memory profiles."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend',choices=['rust','native'],required=True)
    parser.add_argument('--rust-snapshot',type=Path,default=RUST)
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--source',type=Path,default=ROOT/'compiler/tests/fixtures/buffered-core-controls.cinch')
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--opt-level',choices=['0','2'],default='2')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--compile-only',action='store_true')
    parser.add_argument('--source-check',action='store_true',help='Check the complete source closure without linking or executing a consumer')
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    native=args.native_snapshot.resolve();rust=args.rust_snapshot.resolve()
    interfaces=native/'base-interfaces.json';objects_path=native/'objects.json'
    embedded={row[0] for row in json.loads(interfaces.read_text())['interfaces']}
    objects=json.loads(objects_path.read_text());provider_path=native/'provider/provider.json'
    provider=json.loads(provider_path.read_text())
    for key in ['object','metadata']:
        if digest(Path(objects[key]))!=objects[key+'_sha256']:raise ValueError('native provider payload changed')
    if args.frontend=='rust':
        compiler=rust/'cinch';runtime=rust/'libcinchruntime.a';provenance_path=rust/'provenance.json'
        provenance=json.loads(provenance_path.read_text())
        if digest(compiler)!=provenance['compiler_sha256'] or digest(runtime)!=provenance['runtime_sha256']:raise ValueError('Rust snapshot/runtime mismatch')
        string_epoch=provenance.get('string_epoch', {'description':provenance.get('description'), 'embedded_sources':provenance.get('embedded_sources'), 'source_checkpoint':provenance.get('source_checkpoint')})
    else:
        compiler=native/'cinch';runtime=native/'libcinchruntime.a';provenance_path=native/'snapshot.json'
        provenance=json.loads(provenance_path.read_text())
        if digest(compiler)!=provenance['compiler_sha256'] or digest(runtime)!=provenance['runtime_sha256']:raise ValueError('Native snapshot/compiler/runtime mismatch')
        if digest(runtime)!=provider['runtime']['sha256']:raise ValueError('Native provider/runtime mismatch')
        string_epoch='combined e5; explicit admitted interfaces/object; no source shadow'
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    project=artifacts/'project';project.mkdir(parents=True,exist_ok=True)
    source=project/'Main.cinch';source.write_bytes(args.source.read_bytes());paths=closure(project,source,embedded)
    ir=artifacts/'consumer.ll';exe=artifacts/'consumer'
    if args.frontend=='rust':command=([str(compiler),'check',str(source)] if args.source_check else [str(compiler),'emit-ir',str(source),'-o',str(ir),'--rc-debug'])
    else:
        bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
        command=[str(compiler),'--emit-ir','--rc-debug','--interfaces',str(interfaces),'--objects',str(objects_path),str(bundle),str(ir)]
    started=time.monotonic();result=subprocess.run(command,cwd=project,capture_output=True,text=True,preexec_fn=host_stack)
    (artifacts/'compile.stdout').write_text(result.stdout);(artifacts/'compile.stderr').write_text(result.stderr)
    if result.returncode:
        diagnostic=ir.with_name(ir.name+'.diagnostics');raise RuntimeError((diagnostic.read_text() if diagnostic.exists() else result.stdout)+'\n'+result.stderr)
    if args.source_check:
        identity={'phase':'complete source declaration/type/proof check; no link or consumer execution','frontend':args.frontend,'compiler':str(compiler),'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'snapshot_sha256':digest(provenance_path),'snapshot':provenance,'string_epoch':string_epoch,'sources':{path.name:digest(path) for path in paths},'source_sha256':digest(args.source),'source_shadow_used':False,'interfaces_sha256':digest(interfaces),'objects_sha256':digest(objects_path),'provider_sha256':digest(provider_path),'command':command,'compiler_host_stack_mib':64,'seconds':time.monotonic()-started,'cpu':args.cpu,'all_passed':True}
        if ir.exists():identity['discarded_entry_ir_sha256']=digest(ir)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(identity,indent=2)+'\n')
        print('Buffered '+args.frontend+' complete source closure checked; no link or execution',flush=True);return
    link=['clang','-O'+args.opt_level,str(ir)]
    if args.frontend=='native':link.append(objects['object'])
    link +=[str(runtime),'-ldl','-lpthread','-lm','-o',str(exe)]
    linked=subprocess.run(link,capture_output=True,text=True)
    (artifacts/'link.stdout').write_text(linked.stdout);(artifacts/'link.stderr').write_text(linked.stderr)
    if linked.returncode:raise RuntimeError(linked.stderr)
    identity={'frontend':args.frontend,'compiler':str(compiler),'compiler_sha256':digest(compiler),'runtime':str(runtime),'runtime_sha256':digest(runtime),'snapshot_sha256':digest(provenance_path),'snapshot':provenance,'string_epoch':string_epoch,'sources':{path.name:digest(path) for path in paths},'source_sha256':digest(args.source),'source_shadow_used':False,'source_closure_module_names_from':{'interfaces':str(interfaces),'sha256':digest(interfaces),'policy':'same base module names; Rust uses the explicitly recorded snapshot embedded definitions and matched runtime, not the Native provider object'},'compile_command':command,'link_command':link,'llvm_sha256':digest(ir),'executable_sha256':digest(exe),'opt_level':args.opt_level,'compiler_host_stack_mib':64,'compile_seconds':time.monotonic()-started,'cpu':args.cpu,'runs':[]}
    if args.frontend=='native':identity['provider']={'manifest_sha256':digest(provider_path),'interfaces_sha256':digest(interfaces),'objects_sha256':digest(objects_path),'object_sha256':objects['object_sha256'],'metadata_sha256':objects['metadata_sha256']}
    (artifacts/'build-identity.json').write_text(json.dumps(identity,indent=2)+'\n')
    if args.compile_only:print('Buffered '+args.frontend+' consumer compiled',flush=True);return
    for poisoned in [False,True]:
        env={**os.environ,'CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'1','CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M'}
        result=subprocess.run([str(exe)],capture_output=True,text=True,env=env)
        (artifacts/('poison' if poisoned else 'normal')).with_suffix('.stdout').write_text(result.stdout)
        (artifacts/('poison' if poisoned else 'normal')).with_suffix('.stderr').write_text(result.stderr)
        if result.returncode or 'leaked=0' not in result.stderr:raise RuntimeError(f'poisoned={poisoned}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        identity['runs'].append({'poisoned':poisoned,'heap':env['CINCH_HEAP_LIMIT'],'stack':'8M','allocation_statistics':True,'stderr':result.stderr})
    identity['all_passed']=True;args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(identity,indent=2)+'\n')
    print('Buffered '+args.frontend+' passed normal/poison with zero leaks',flush=True)


if __name__=='__main__':main()
