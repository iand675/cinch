"""Retain all raw warmup and measurement observations without changing the preflighted invocation protocol."""
from pathlib import Path
import hashlib,json,shutil,sys
sys.path.insert(0,str(Path.cwd()/'tooling'))
import bench_uuid_parser_pair as pair
assert shutil.disk_usage(Path.cwd()).free>=300*1024*1024
original=pair.run
raw=[]
def observed(*args,**kwargs):
    """Record the exact result of every unchanged sealed-binary invocation, including warmup."""
    result=original(*args,**kwargs)
    raw.append({'executable':str(args[0]),'arguments':args[1],'observation':result.copy()})
    return result
pair.run=observed
sys.argv=['bench_uuid_parser_pair.py','measure','--baseline','build/uuid-current-baseline-a1','--candidate','build/uuid-current-packed-a1','--cpu','0','--output','build/uuid-current-pair-timing-a1.json','--preflight','build/uuid-current-pair-preflight-a1.json','--iterations','4096','--samples','5','--budget-seconds','20','--background','Root exclusiveCPU0START; allpeersackinactive/noheavyjobs; exactad6baseline+packed+sameSDK; Rootc2epoch notused']
try:
    pair.main()
finally:
    for index,row in enumerate(raw):
        row.update(condition_index=index//18,sample_index=(index%18)//3,warmup=(index%18)//3==0)
    report={'kind':'all-raw-UUID-pair-timing-observations','wrapper_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'invocation_runner_sha256':hashlib.sha256(Path('tooling/bench_uuid_parser_pair.py').read_bytes()).hexdigest(),'raw':raw,'includes_warmup':True,'clipped_observations':False}
    Path('build/uuid-current-pair-timing-a1-raw.json').write_text(json.dumps(report,indent=2)+'\n')
    assert sum(Path(p).stat().st_size for p in ('build/uuid-current-pair-timing-a1.json','build/uuid-current-pair-timing-a1-raw.json'))<=5*1024*1024
    assert shutil.disk_usage(Path.cwd()).free>=250*1024*1024
