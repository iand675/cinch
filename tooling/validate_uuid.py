#!/usr/bin/env python3
"""Run explicit-tuple bounded UUID source/runtime units against committed independent observations."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
import time
from generate_uuid_cases import source
ROOT = Path(__file__).resolve().parents[1]
MODULES = ('Data.Uuid','Effect.Uuid','Text.Hex','Data.Bytes.Word','Effect.Random','Effect.Clock','Effect.Concurrent.MVar')

def digest(path: Path) -> str:
    """Record exact compiler, runtime, source, fixture and generated artifact identities."""
    return hashlib.sha256(path.read_bytes()).hexdigest()

def host_stack() -> None:
    """Give the compiler a64MiB host stack without changing the tested runtime's8MiB stack."""
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK,(min(67108864,hard) if hard != resource.RLIM_INFINITY else 67108864,hard))

def execute(command: list[str], directory: Path, environment: dict, host: bool = False) -> dict:
    """Preserve every verdict and diagnostic, including the first genuine failure."""
    started=time.monotonic()
    result=subprocess.run(command,cwd=directory,env=environment,capture_output=True,text=True,
                          preexec_fn=host_stack if host else None,timeout=180)
    return {'command':command,'exit':result.returncode,'stdout':result.stdout,'stderr':result.stderr,'seconds':time.monotonic()-started}

def main() -> None:
    """Compile one selected unit, enforce resource guards, and distinguish source and runtime claims."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime',type=Path,required=True)
    parser.add_argument('--clang',type=Path,required=True)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--minimum-free-mib',type=int,choices=(320,350),default=350,
                        help='Use320 only for the explicitly coordinated single-producer resource exception')
    parser.add_argument('--mode',choices=('source','parse','append','compare','v7','generated','control','io'),required=True)
    parser.add_argument('--start',type=int,default=0)
    parser.add_argument('--count',type=int,default=128)
    parser.add_argument('--opt-level',choices=('0','2'),default='2')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    os.sched_setaffinity(0,{args.cpu})
    compiler,runtime,clang=args.compiler.resolve(),args.runtime.resolve(),args.clang.resolve()
    free=shutil.disk_usage(ROOT).free
    if free<args.minimum_free_mib*1024*1024:
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_text(json.dumps({'kind':'bounded-UUID-candidate-qualification','status':'resource-paused-before-compiler',
            'compiler_invoked':False,'runtime_executed':False,'mode':args.mode,'start':args.start,'count':args.count,
            'fresh_start_mib':args.minimum_free_mib,'free_before':free,'candidate_selected':False,'package_complete':False},indent=2)+'\n')
        print(f'Resource pause: {free} bytes free before compiler; requires{args.minimum_free_mib}MiB',flush=True)
        raise SystemExit(2)
    assert not args.artifacts.exists(),'Use a fresh directory and preserve every previous attempt'
    directory=args.artifacts.resolve();directory.mkdir(parents=True)
    modules={name:(ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')).read_bytes() for name in MODULES}
    for name,data in modules.items():(directory/(name+'.cinch')).write_bytes(data)
    if args.mode=='source':
        text='-- | Check both identifier surfaces without running effects.\nmodule Main where\n  import Data.Uuid\n  import Effect.Uuid\n  -- | Keep this consumer source-only.\n  main :: Int\n  main = 0\n'
    elif args.mode in ('control','io'):
        text=(ROOT/f'compiler/tests/fixtures/uuid-{args.mode}.cinch').read_text()
    else:text=source(args.mode,args.start,args.count)
    entry=directory/'Main.cinch';entry.write_text(text)
    executable,llvm=directory/'consumer',directory/'consumer.ll'
    report={'kind':'bounded-UUID-candidate-qualification','mode':args.mode,'start':args.start,'count':args.count,
            'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'clang_sha256':digest(clang),
            'runner_sha256':digest(Path(__file__)),'sources':{k:hashlib.sha256(v).hexdigest() for k,v in modules.items()},
            'fixture_sha256':digest(ROOT/'compiler/tests/fixtures/uuid-vectors.json'),'consumer_sha256':digest(entry),
            'candidate_selected':False,'package_complete':False,'native_family_relabelled':False,
            'cpu_affinity':[args.cpu],'free_before':free,'fresh_start_mib':args.minimum_free_mib,
            'minimum_reserve_mib':250,'maximum_new_output_mib':5 if args.mode=='source' else 20,'status':'running','runs':[]}
    environment={**os.environ,'CINCH_CLANG':str(clang),'RUST_MIN_STACK':'67108864','CINCH_TEST_API_EMIT_INTERFACES':'0'}
    command=[str(compiler),'check' if args.mode=='source' else 'build',str(entry)]
    if args.mode!='source':command+=['-o',str(executable),'--runtime-archive',str(runtime),'--opt-level',args.opt_level]
    try:
        report['compile']=execute(command,directory,environment,True)
        (directory/'build.log').write_text(report['compile']['stdout']+report['compile']['stderr'])
        assert report['compile']['exit']==0,report['compile']
        if args.mode!='source':
            report.update(executable_sha256=digest(executable),llvm_sha256=digest(llvm))
            for poison in (False,True):
                runtime_env={**environment,'CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M',
                             'CINCH_RC_DEBUG':'1' if poison else '0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'1'}
                run=execute([str(executable)],directory,runtime_env)
                run.update(poisoned_free=poison,heap=runtime_env['CINCH_HEAP_LIMIT'],stack='8M')
                report['runs'].append(run);(directory/('poisoned.log' if poison else 'normal.log')).write_text(run['stdout']+run['stderr'])
                assert run['exit']==0 and run['stdout']=='',run
                assert 'cinch-leak: live=0 immortal=0 leaked=0' in run['stderr'],run
        report['output_bytes']=sum(p.stat().st_size for p in directory.rglob('*') if p.is_file())
        assert report['output_bytes']<=(5 if args.mode=='source' else 20)*1024*1024,report['output_bytes']
        report['free_after']=shutil.disk_usage(ROOT).free;assert report['free_after']>=250*1024*1024
        report['status']='passed'
        print(f"{args.mode} start={args.start}: {'source' if args.mode=='source' else 'normal+poison zero-leak'} passed",flush=True)
    except BaseException as error:
        report.update(status='failed',failure=repr(error));raise
    finally:
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
