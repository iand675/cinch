#!/usr/bin/env python3
"""Qualify only pure UUID adapters against one explicit historical Native compiler/provider/runtime tuple."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
from generate_uuid_cases import source
from validate_uuid import ROOT,digest,execute
PAIR=Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen')


def main() -> None:
    """Preserve exact pair authority and refuse IO/MVar/new-family substitution before one bounded consumer."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode',choices=('source','parse','append','compare','generated','v7','control'),required=True)
    parser.add_argument('--start',type=int,default=0)
    parser.add_argument('--count',type=int,default=128)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--clang',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    assert shutil.disk_usage(ROOT).free>=350*1024*1024
    assert not args.artifacts.exists();directory=args.artifacts.resolve();directory.mkdir(parents=True)
    reviewed=ROOT/'bootstrap/evidence/stdlib/stdlib-uuid-native-pair-review.json';pair=json.loads(reviewed.read_text())
    for name,key in [('cinch','compiler_sha256'),('frontend.py','frontend_sha256'),('base-interfaces.json','interfaces_sha256'),('objects.json','objects_sha256'),('libcinchruntime.a','runtime_sha256')]:assert digest(PAIR/name)==pair[key]
    objects=json.loads((PAIR/'objects.json').read_text())
    for key in ('object','metadata'):assert digest(Path(objects[key]))==objects[key+'_sha256']
    assert objects['target']=='x86_64-unknown-linux-gnu'
    modules=('Data.Bytes.Word','Text.Hex','Data.Uuid');paths=[]
    for module in modules:
        path=directory/(module+'.cinch');path.write_bytes((ROOT/'compiler/prelude'/(module.replace('.','/')+'.cinch')).read_bytes());paths.append(path)
    if args.mode=='source':text='-- | Pure identifier source-only historical pair probe.\nmodule Main where\n  import Data.Uuid\n  -- | No IO or MVar operation is requested.\n  main :: Int\n  main = 0\n'
    elif args.mode=='control':text=(ROOT/'compiler/tests/fixtures/uuid-control.cinch').read_text()
    else:text=source(args.mode,args.start,args.count)
    entry=directory/'Main.cinch';entry.write_text(text);paths.append(entry)
    bundle=directory/'Bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in paths))
    ir=directory/'consumer.ll';executable=directory/'consumer'
    report={'kind':'strict-historical-Native-pure-UUID-unit','pair_review_sha256':digest(reviewed),
            'compiler_sha256':pair['compiler_sha256'],'runtime_sha256':pair['runtime_sha256'],
            'interface_sha256':pair['interfaces_sha256'],'objects_sha256':pair['objects_sha256'],
            'provider_object_sha256':objects['object_sha256'],'mode':args.mode,'start':args.start,'count':args.count,
            'sources':{path.name:digest(path) for path in paths},'bundle_sha256':digest(bundle),
            'candidate_selected':False,'package_complete':False,'new66_family_relabelled':False,
            'io_or_mvar_requested':False,'status':'running','runs':[]}
    command=['python3',str(PAIR/'frontend.py'),'--check' if args.mode=='source' else '--emit-ir','--interfaces',str(PAIR/'base-interfaces.json'),str(bundle),str(ir)]
    if args.mode!='source':command.insert(2,'--rc-debug')
    environment={**os.environ,'RUST_MIN_STACK':'67108864'}
    try:
        report['compile']=execute(command,directory,environment,True)
        diagnostic=ir.with_suffix('.ll.diagnostics');report['diagnostics']=diagnostic.read_text() if diagnostic.exists() else ''
        (directory/'build.log').write_text(report['compile']['stdout']+report['compile']['stderr']+report['diagnostics'])
        assert report['compile']['exit']==0,report['compile']|{'diagnostics':report['diagnostics']}
        if args.mode!='source':
            command=[str(args.clang.resolve()),'-O2',str(ir),str(objects['object']),str(PAIR/'libcinchruntime.a'),'-o',str(executable),'-ldl','-lpthread','-lm']
            report['link']=execute(command,directory,environment);assert report['link']['exit']==0,report['link']
            report.update(executable_sha256=digest(executable),llvm_sha256=digest(ir))
            for poison in (False,True):
                run=execute([str(executable)],directory,{**environment,'CINCH_RC_DEBUG':'1' if poison else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M'})
                run.update(poisoned_free=poison,heap='256M' if poison else '32M',stack='8M');report['runs'].append(run)
                (directory/('poisoned.log' if poison else 'normal.log')).write_text(run['stdout']+run['stderr'])
                assert run['exit']==0 and run['stdout']=='',run
                assert 'cinch-leak: live=0 immortal=0 leaked=0' in run['stderr'],run
        report['output_bytes']=sum(path.stat().st_size for path in directory.rglob('*') if path.is_file())
        assert report['output_bytes']<=20*1024*1024
        assert shutil.disk_usage(ROOT).free>=250*1024*1024
        report['status']='passed';print(f'HistoricalNative pureUUID {args.mode} passed; IO/MVar/new66 excluded')
    except BaseException as error:report.update(status='failed',failure=repr(error));raise
    finally:args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
