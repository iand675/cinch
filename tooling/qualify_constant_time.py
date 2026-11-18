"""Compile and observe independent constant-time consumers without conflating timing claims."""
from pathlib import Path
import argparse,hashlib,json,os,shutil,subprocess,time


def digest(path):
    """Identify every actual input/output artifact used by a qualification run."""
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    """Link exact emitted IR at both optimization levels and run bounded groups."""
    parser=argparse.ArgumentParser();parser.add_argument('project',type=Path);parser.add_argument('--runtime',type=Path,required=True);parser.add_argument('--clang',required=True);parser.add_argument('--object',type=Path);parser.add_argument('--compiled-rc-debug',action='store_true');args=parser.parse_args()
    project=args.project.resolve();runtime=args.runtime.resolve();ir=project/'consumer.ll';groups=json.loads((project/'Main.groups.json').read_text())['groups']
    report={'llvm_sha256':digest(ir),'runtime_sha256':digest(runtime),'source_sha256':{p.name:digest(p) for p in project.glob('*.cinch')},'compiled_rc_debug':args.compiled_rc_debug,'runs':[],'scope':'Functional result/owner qualification only; constant-time machine-code and public pipeline benchmark review remain separate.'}
    objects=[str(args.object.resolve())] if args.object else []
    if args.object:report['provider_object_sha256']=digest(args.object)
    for level in [0,2]:
        executable=project/f'consumer-o{level}';command=[args.clang,f'-O{level}',str(ir),*objects,str(runtime),'-lpthread','-ldl','-lm','-o',str(executable)]
        start=time.monotonic();built=subprocess.run(command,capture_output=True,text=True);(project/f'link-o{level}.log').write_text(built.stdout+built.stderr)
        if built.returncode:raise RuntimeError(f'link O{level}: {built.stderr}')
        report[f'executable_o{level}_sha256']=digest(executable);report[f'link_o{level}_seconds']=time.monotonic()-start
        for poisoned in [False,True]:
            for group in range(groups):
                env={**os.environ,'CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M'}
                result=subprocess.run([str(executable),str(group)],env=env,capture_output=True,text=True,timeout=30)
                row={'optimization':level,'poisoned':poisoned,'group':group,'exit':result.returncode,'stderr':result.stderr};report['runs'].append(row)
                (project/'qualification.json').write_text(json.dumps(report,indent=2)+'\n')
                if result.returncode or 'cinch-leak: live=0 immortal=0 leaked=0' not in result.stderr:raise RuntimeError(json.dumps(row))
    print(json.dumps({'processes':len(report['runs']),'groups':groups,'result':'passed'}))

if __name__=='__main__':main()
