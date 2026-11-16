#!/usr/bin/env python3
"""Build exact Unicode/catalog/UTF8 edge consumers under an explicit frozen compiler epoch."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
import time
from render_unicode_edges import rendered as edges
from render_unicode_catalog import rendered as catalog
from validate_unicode import ROOT, RUST, NATIVE, digest
from validate_url import source_closure


def main() -> None:
    """Keep source closure, provider hashes and both bounded-memory outcomes with each probe."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kind',choices=['edges','catalog','utf8-ownership','utf8-spans'],required=True)
    parser.add_argument('--frontend',choices=['rust','native'],required=True)
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--utf8-source-override',type=Path,help='Retain an isolated byte-layer candidate in the UTF8 ownership probe')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=args.artifacts.resolve();project=artifacts/'project';project.mkdir(parents=True,exist_ok=True)
    text=edges() if args.kind=='edges' else catalog() if args.kind=='catalog' else (ROOT / ('compiler/tests/fixtures/utf8-span-controls.cinch' if args.kind == 'utf8-spans' else 'compiler/tests/fixtures/utf8-ownership-consumer.cinch')).read_text()
    source=project/'Main.cinch';source.write_text(text)
    paths=source_closure(project,source)+[source];exe=artifacts/'consumer'
    if args.utf8_source_override:
        if args.kind not in ['utf8-ownership','utf8-spans']:raise ValueError('UTF8 override only applies to its ownership probe')
        (project/'Data.Bytes.Utf8.cinch').write_bytes(args.utf8_source_override.read_bytes())
    compiler=RUST if args.frontend=='rust' else args.native_snapshot/'cinch'
    runtime=Path('/workspace/dasl-stdlib-io/compiler/runtime/target/release/libcinchruntime.a') if args.frontend=='rust' else args.native_snapshot/'libcinchruntime.a'
    provider={} if args.frontend=='rust' else {name:digest(args.native_snapshot/name) for name in ['frontend.py','base-interfaces.json','objects.json','library.o','library.abi.json','provenance.json']}
    if args.frontend=='rust':command=[str(compiler),'build',str(source),'-o',str(exe),'--opt-level','2','--rc-debug']
    else:
        bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
        command=['python3',str(args.native_snapshot/'frontend.py'),'--compile-only','--interfaces',str(args.native_snapshot/'base-interfaces.json'),'--rc-debug',str(bundle),str(artifacts/'consumer.ll'),str(exe),str(artifacts/'link-report.txt')]
    start=time.monotonic();result=subprocess.run(command,capture_output=True,text=True,cwd=project)
    (artifacts/'compile.stdout').write_text(result.stdout);(artifacts/'compile.stderr').write_text(result.stderr)
    if result.returncode:
        diagnostics=artifacts/'consumer.ll.diagnostics'
        raise RuntimeError((diagnostics.read_text() if diagnostics.exists() else result.stdout)+'\n'+result.stderr)
    compile_seconds=time.monotonic()-start;runs=[]
    for debug in [False,True]:
        env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M'}
        start=time.monotonic();result=subprocess.run([str(exe)],env=env,capture_output=True,text=True)
        if result.returncode or 'leaked=0' not in result.stderr:raise RuntimeError(f'poisoned={debug}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        runs.append({'poisoned':debug,'heap':env['CINCH_HEAP_LIMIT'],'stack':'8M','seconds':time.monotonic()-start,'stderr':result.stderr})
    report={'kind':args.kind,'frontend':args.frontend,'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'provider_sha256':provider,'sources':{path.name:digest(path) for path in paths},'executable_sha256':digest(exe),'compile_command':command,'compile_seconds':compile_seconds,'cpu':args.cpu,'runs':runs,'all_passed':True}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print(f'{args.kind} {args.frontend} passed normal and poisoned memory',flush=True)


if __name__=='__main__':main()
