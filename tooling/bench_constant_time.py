"""Preflight and compare actual prepared-buffer equality pipelines with raw paired controls."""
from pathlib import Path
import argparse,hashlib,json,os,resource,statistics,subprocess,time


def digest(path):
    """Hash immutable benchmark artifacts before observing their behavior."""
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def cpu_stat():
    """Capture global quota throttling so noisy timing epochs remain identifiable."""
    p=Path('/sys/fs/cgroup/cpu.stat');return p.read_text() if p.exists() else ''


def prepare(root, compiler, artifact_root, variant, source):
    """Emit an isolated source closure without replacing any historical benchmark input."""
    directory=artifact_root/variant
    directory.mkdir(parents=True,exist_ok=True)
    (directory/'Crypto.ConstantTime.cinch').write_bytes(source.read_bytes())
    for name,path in [('Data.Bytes.Word.cinch',root/'compiler/prelude/Data/Bytes/Word.cinch'),
                      ('Main.cinch',root/'benchmarks/stdlib/constant-time-equality.cinch')]:
        (directory/name).write_bytes(path.read_bytes())
    result=subprocess.run([str(compiler.resolve()),'emit-ir',str(directory/'Main.cinch'),
                           '-o',str(directory/'consumer.ll')],capture_output=True,text=True,timeout=90)
    (directory/'emit.log').write_text(result.stdout+result.stderr)
    assert result.returncode==0,result.stderr
    return directory


def main():
    """Link unchanged sources, preflight complete outcomes, then record paired ordinary-RC intervals."""
    parser=argparse.ArgumentParser();parser.add_argument('--clang',required=True);parser.add_argument('--runtime',type=Path,required=True);parser.add_argument('--samples',type=int,default=5);parser.add_argument('--work-bytes',type=int,default=64*1024*1024);parser.add_argument('--reuse-preflight',type=Path);parser.add_argument('--candidate',default='v4');parser.add_argument('--baseline',default='v1-final');parser.add_argument('--output',type=Path);parser.add_argument('--compiler',type=Path);parser.add_argument('--artifact-root',type=Path);parser.add_argument('--preflight-only',action='store_true');args=parser.parse_args()
    if args.samples<1 or args.work_bytes<1:parser.error('samples and work bytes must be positive')
    if args.compiler and (not args.artifact_root or not args.output):
        parser.error('--compiler requires a new --artifact-root and --output to preserve earlier epochs')
    if args.output and args.output.exists():
        parser.error('select a new output path to preserve earlier measurements')
    if args.baseline==args.candidate:parser.error('baseline and candidate must have distinct names')
    if any(Path(value).name!=value or value in ('.','..') for value in [args.baseline,args.candidate]):
        parser.error('variant names must be single directory names')
    soft,hard=resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK,(max(soft,64*1024*1024),hard))
    root=Path(__file__).resolve().parents[1];runtime=args.runtime.resolve();support=root/'benchmarks/stdlib/constant-time-equality.c'
    variants=[args.baseline,args.candidate];report={'runtime_sha256':digest(runtime),'support_sha256':digest(support),'compiled_rc_debug':False,'timing_profile':'ordinary generated RC; counters disabled; normal32M/8M','sizes':[16,256,4096,65536],'samples':args.samples,'artifacts':{},'preflight':[],'timings':[],'quota':Path('/sys/fs/cgroup/cpu.max').read_text().strip(),'cpu_stat_before':cpu_stat()}
    prior=json.loads(args.reuse_preflight.read_text()) if args.reuse_preflight else None
    if args.compiler:
        if prior is None and args.artifact_root.exists() and any(args.artifact_root.iterdir()):
            parser.error('fresh emission requires an empty artifact directory; select another epoch')
        report['compiler_sha256']=digest(args.compiler)
        report['observer_sha256']=digest(__file__)
    executables={}
    for variant in variants:
        work=args.artifact_root.resolve()/variant if args.artifact_root else root/f'build/constant-time-benchmark-{variant}'
        if args.compiler and prior is None:
            source=root/('benchmarks/stdlib/constant-time-equality-baseline.cinchpart' if variant==args.baseline else 'compiler/prelude/Crypto/ConstantTime.cinch')
            work=prepare(root,args.compiler,args.artifact_root.resolve(),variant,source)
        executable=work/'consumer';ir=work/'consumer.ll'
        command=[args.clang,'-O2',str(ir),str(support),str(runtime),'-lpthread','-ldl','-lm','-o',str(executable)]
        if prior is None:
            built=subprocess.run(command,capture_output=True,text=True);(work/'link.log').write_text(built.stdout+built.stderr)
            if built.returncode:raise RuntimeError(built.stderr)
        executables[variant]=executable;report['artifacts'][variant]={'llvm_sha256':digest(ir),'executable_sha256':digest(executable),'source_sha256':{p.name:digest(p) for p in work.glob('*.cinch')}}
    if prior is not None:
        assert prior['preflight_passed']==128 and prior['artifacts']==report['artifacts']
        assert prior['runtime_sha256']==report['runtime_sha256'] and prior['support_sha256']==report['support_sha256']
        if args.compiler:assert prior['compiler_sha256']==report['compiler_sha256'] and prior['observer_sha256']==report['observer_sha256']
        report['preflight_reused_sha256']=digest(args.reuse_preflight)
    report['work_bytes']=args.work_bytes
    output=args.output or root/f'build/constant-time-equality-benchmark-{args.candidate}.json'
    output.parent.mkdir(parents=True,exist_ok=True)
    def run(variant,size,count,mode,control,poisoned):
        """Run one fixed public recipe with stable argv0 and verify every observation."""
        env={**os.environ,'CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'0'}
        completed=subprocess.run(['constant-time-benchmark',str(size),str(count),mode,'control' if control else 'compare'],executable=str(executables[variant]),capture_output=True,text=True,env=env,timeout=30)
        if completed.returncode or 'cinch-leak: live=0 immortal=0 leaked=0' not in completed.stderr:raise RuntimeError(f'{variant}/{size}/{mode}: {completed.stdout} {completed.stderr}')
        fields=completed.stdout.strip().split();assert len(fields)==2,completed.stdout
        elapsed,observed=map(int,fields);expected=count if control or mode=='equal' or size==0 else 0
        assert elapsed>=0 and observed==expected,(elapsed,observed,expected)
        return {'variant':variant,'size':size,'iterations':count,'mode':mode,'control':control,'poisoned':poisoned,'nanoseconds':elapsed,'observed':observed}
    for variant in ([] if prior else variants):
        for size in [0,1,15,16,17,256,4096,65536]:
            for mode in ['equal','first','middle','last']:
                for poisoned in [False,True]:report['preflight'].append(run(variant,size,2,mode,False,poisoned))
    if prior is not None:report['preflight']=prior['preflight']
    report['preflight_passed']=len(report['preflight']);output.write_text(json.dumps(report,indent=2)+'\n')
    if args.preflight_only:
        report['cpu_stat_after']=cpu_stat();output.write_text(json.dumps(report,indent=2)+'\n')
        print(json.dumps({'preflight':report['preflight_passed'],'timing_processes':0}));return
    for sample in range(args.samples):
        for size in report['sizes']:
            count=max(16,args.work_bytes//size)
            for mode in ['equal','first','middle','last']:
                order=variants if sample%2==0 else list(reversed(variants))
                for variant in order:
                    if sample==0:run(variant,size,max(2,count//10),mode,False,False)
                    row=run(variant,size,count,mode,False,False);row['sample']=sample;report['timings'].append(row)
            for variant in variants:
                row=run(variant,size,count,'equal',True,False);row['sample']=sample;report['timings'].append(row)
        output.write_text(json.dumps(report,indent=2)+'\n')
    report['cpu_stat_after']=cpu_stat();report['medians']=[]
    for size in report['sizes']:
        for mode in ['equal','first','middle','last']:
            values={v:statistics.median(r['nanoseconds'] for r in report['timings'] if r['variant']==v and r['size']==size and r['mode']==mode and not r['control']) for v in variants}
            report['medians'].append({'size':size,'mode':mode,'baseline_nanoseconds':values[args.baseline],'baseline':args.baseline,'candidate_nanoseconds':values[args.candidate],'candidate':args.candidate,'speedup':values[args.baseline]/values[args.candidate]})
    output.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'preflight':report['preflight_passed'],'timing_processes':len(report['timings']),'medians':report['medians']}))

if __name__=='__main__':main()
