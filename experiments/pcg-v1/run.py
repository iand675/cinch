#!/usr/bin/env python3
"""Check and run the immutable PCG public consumer with a supplied exact compiler/runtime pair."""
from pathlib import Path
import argparse,hashlib,json,os,resource,shutil,subprocess,sys,time

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tooling'))
from bench_stdlib import source_closure


def digest(path):
    """Identify the exact bytes used at each boundary."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Preserve all inputs and first failures in an independent bounded source epoch."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime',type=Path,required=True)
    parser.add_argument('--work',type=Path,required=True)
    parser.add_argument('--opt',choices=['0','2'],required=True)
    args=parser.parse_args();work=args.work.resolve();work.mkdir(exist_ok=False)
    assert shutil.disk_usage(work).free>=350*1024*1024
    resource.setrlimit(resource.RLIMIT_STACK,(67108864,resource.getrlimit(resource.RLIMIT_STACK)[1]))
    source=ROOT/'experiments/pcg-v1/sources/Data.Random.Pcg.cinch'
    consumer=ROOT/'experiments/pcg-v1/consumer.cinch'
    for name,path in source_closure(consumer.read_text(),{'Data.Random.Pcg':source}).items():shutil.copyfile(path,work/(name+'.cinch'))
    shutil.copyfile(consumer,work/'Main.cinch');shutil.copyfile(Path(__file__),work/'executed-runner.py')
    compiler,runtime=args.compiler.resolve(),args.runtime.resolve()
    record={'scope':'Independent PCG2048 outputs plus snapshots, ownership and refusals; wider random-package surface/performance/Native remain open.','compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'optimization':args.opt,'sources':{p.name:digest(p) for p in work.glob('*.cinch')},'commands':[],'runs':[],'status':'running','initial_free_bytes':shutil.disk_usage(work).free,'minimum_observed_free_bytes':shutil.disk_usage(work).free,'runtime_claim':False}
    environment={**os.environ,'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}

    def save():
        """Record exact diagnostics without deleting any source or unsuccessful output."""
        record['end_free_bytes']=shutil.disk_usage(work).free
        record['minimum_observed_free_bytes']=min(record['minimum_observed_free_bytes'],record['end_free_bytes'])
        (work/'receipt.json').write_text(json.dumps(record,indent=2)+'\n')

    def execute(command,env,deadline):
        """Bound each phase by disk reserve, total unit output and a wall deadline."""
        assert shutil.disk_usage(work).free>=250*1024*1024
        assert sum(p.stat().st_size for p in work.rglob('*') if p.is_file())<=30*1024*1024
        row={'command':command};record['commands'].append(row);save();started=time.monotonic()
        result=subprocess.run(command,cwd=work,env=env,capture_output=True,text=True,timeout=deadline)
        row.update(returncode=result.returncode,stdout=result.stdout,stderr=result.stderr,seconds=time.monotonic()-started);save();assert result.returncode==0,row
        assert sum(p.stat().st_size for p in work.rglob('*') if p.is_file())<=30*1024*1024
        return row

    try:
        execute(['taskset','-c','3',str(compiler),'check',str(work/'Main.cinch')],environment,30)
        execute(['taskset','-c','3',str(compiler),'emit-ir',str(work/'Main.cinch'),'-o',str(work/'consumer.ll')],environment,60)
        execute(['taskset','-c','3','/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang','-O'+args.opt,str(work/'consumer.ll'),str(runtime),'-ldl','-lpthread','-lm','-o',str(work/'consumer')],environment,30)
        for poison,heap in [('0','32M'),('1','256M')]:
            row=execute(['taskset','-c','3',str(work/'consumer')],{**environment,'CINCH_HEAP_LIMIT':heap,'CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':poison},30)
            assert row['stdout']=='' and 'live=0' in row['stderr'] and 'leaked=0' in row['stderr']
            record['runs'].append({'poison':poison,'heap':heap,**row})
        record.update(status='passed',runtime_claim=True,llvm_sha256=digest(work/'consumer.ll'),executable_sha256=digest(work/'consumer'))
    except BaseException as error:
        record.update(status='failed',error=repr(error),all_failed_artifacts_retained_plain=True);raise
    finally:save()
    print(json.dumps({'status':record['status'],'optimization':args.opt,'seeds':8,'independent_outputs':2048,'profiles':len(record['runs']),'end_free_bytes':record['end_free_bytes']}))


if __name__=='__main__':main()
