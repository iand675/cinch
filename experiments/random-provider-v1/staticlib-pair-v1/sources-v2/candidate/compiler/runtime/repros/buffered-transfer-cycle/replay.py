#!/usr/bin/env python3
"""Replay frozen transfer-retention cycles with explicit compiler/runtime identities and fixed consumer limits."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess

HERE=Path(__file__).resolve().parent
FROZEN=Path('/workspace/dasl-stdlib-io/build/string-word-safe-frozen')


def sha(path: Path) -> str:
    """Identify exact source and tool bytes before compiling or reporting a result."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def host_stack() -> None:
    """Set the compiler-host stack independently of the fixed eight-MiB consumer stack."""
    resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))


def main() -> None:
    """Compile one frozen case and verify either the recorded leak or a separately identified reclaimed candidate."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--case',choices=['original','eof'],default='eof')
    parser.add_argument('--compiler',type=Path,default=FROZEN/'cinch')
    parser.add_argument('--runtime',type=Path,default=FROZEN/'libcinchruntime.a')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--poison',action='store_true')
    parser.add_argument('--expect-reclaimed',action='store_true')
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    expected=json.loads((HERE/'receipt.json').read_text())[args.case]
    if sha(args.compiler)!=expected['compiler_sha256']:raise ValueError('compiler does not match the frozen producer')
    if not args.expect_reclaimed and sha(args.runtime)!=expected['runtime_sha256']:raise ValueError('baseline runtime differs from the recorded leak epoch')
    artifacts=args.artifacts.resolve();project=artifacts/'project';project.mkdir(parents=True,exist_ok=True)
    for source in (HERE/'modules').glob('*.cinch'):shutil.copyfile(source,project/source.name)
    shutil.copyfile(HERE/(args.case+'.cinchpart'),project/'Main.cinch')
    for name,digest in expected['sources'].items():
        if sha(project/name)!=digest:raise ValueError('frozen source changed: '+name)
    ir=artifacts/'consumer.ll';exe=artifacts/'consumer'
    compile_command=[str(args.compiler.resolve()),'emit-ir',str(project/'Main.cinch'),'-o',str(ir),'--rc-debug']
    compiled=subprocess.run(compile_command,cwd=project,capture_output=True,text=True,preexec_fn=host_stack)
    (artifacts/'compile.stdout').write_text(compiled.stdout);(artifacts/'compile.stderr').write_text(compiled.stderr)
    if compiled.returncode:raise RuntimeError(compiled.stderr)
    link=['clang','-O2',str(ir),str(args.runtime.resolve()),'-ldl','-lpthread','-lm','-o',str(exe)]
    linked=subprocess.run(link,capture_output=True,text=True)
    (artifacts/'link.stderr').write_text(linked.stderr)
    if linked.returncode:raise RuntimeError(linked.stderr)
    env={**os.environ,'CINCH_RC_DEBUG':'1' if args.poison else '0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'1','CINCH_HEAP_LIMIT':'256M' if args.poison else '32M','CINCH_STACK_SIZE':'8M'}
    observed=subprocess.run([str(exe)],capture_output=True,text=True,env=env)
    (artifacts/'consumer.stdout').write_text(observed.stdout);(artifacts/'consumer.stderr').write_text(observed.stderr)
    result={'case':args.case,'compile_command':compile_command,'link_command':link,'compiler_sha256':sha(args.compiler),'runtime_sha256':sha(args.runtime),'llvm_sha256':sha(ir),'executable_sha256':sha(exe),'poisoned':args.poison,'consumer_heap':env['CINCH_HEAP_LIMIT'],'consumer_stack':'8M','compiler_host_stack_mib':64,'exit_code':observed.returncode,'stdout':observed.stdout,'stderr':observed.stderr,'expect_reclaimed':args.expect_reclaimed}
    (artifacts/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    if args.expect_reclaimed:
        if observed.returncode!=0 or 'live=0 immortal=0 leaked=0' not in observed.stderr:raise RuntimeError('candidate did not reclaim the frozen cycle')
    elif observed.returncode!=74 or 'live=22 immortal=0 leaked=22' not in observed.stderr:raise RuntimeError('recorded22-block leak did not reproduce')
    print('Verified reclaimed cycle' if args.expect_reclaimed else 'Verified known22-block cycle leak')

if __name__=='__main__':main()
