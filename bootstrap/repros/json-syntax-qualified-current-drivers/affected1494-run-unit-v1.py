#!/usr/bin/env python3
"""Qualify one unchanged historical JSON fixture against the syntax-partial candidate."""
from pathlib import Path
import argparse, hashlib, json, os, resource, subprocess, time
ROOT = Path('/workspace/dasl-stdlib-json')
MIB = 1024 * 1024
PLAN = ROOT / 'bootstrap/stdlib-json-syntax-partial-affected1494-plan.json'
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def free():
    s = os.statvfs(ROOT)
    return s.f_bavail * s.f_frsize
plan = json.loads(PLAN.read_text())
p = argparse.ArgumentParser()
p.add_argument('unit', choices=[x['unit'] for x in plan['units']])
a = p.parse_args()
unit = next(x for x in plan['units'] if x['unit'] == a.unit)
folder = ROOT / f'build/json-syntax-affected1494-{a.unit}-v1'
receipt = ROOT / f'bootstrap/stdlib-json-syntax-affected1494-{a.unit}-rust-v1-results.json'
assert free() >= 350*MIB, ('start guard', free())
assert not folder.exists() and not receipt.exists(), 'Refuse repeated source epoch'
compiler = Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
runtime = compiler.with_name('libcinchruntime.a')
clang = Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
assert digest(compiler) == plan['compiler_sha256']
assert digest(runtime) == plan['runtime_sha256']
oracle = ROOT / plan['oracle']['path']
assert digest(oracle) == plan['oracle']['sha256']
rows = json.loads(oracle.read_text())
assert {k:len(v) for k,v in rows.items()} == {'method_errors':288,'tagged':1152,'updates':54}
fixture = ROOT / unit['fixture']
assert digest(fixture) == unit['fixture_sha256']
for name, expected in plan['sources'].items():
    assert digest(ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')) == expected, name
folder.mkdir()
source = folder/'source'; source.mkdir()
output = folder/'runtime'; output.mkdir()
(folder/'qualification-plan.json').write_bytes(PLAN.read_bytes())
(folder/'executed-runner.py').write_bytes(Path(__file__).read_bytes())
for name, expected in plan['sources'].items():
    src = ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')
    (source/(name+'.cinch')).write_bytes(src.read_bytes())
(source/'Main.cinch').write_bytes(fixture.read_bytes())
os.sched_setaffinity(0, {2})
_, hard = resource.getrlimit(resource.RLIMIT_STACK)
resource.setrlimit(resource.RLIMIT_STACK, (65536*1024, hard))
env = {**os.environ, 'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}
r = {'status':'prepared','unit':a.unit,'scope':'One affected historical fixture on current syntax-partial source; no Native/full92/promotion claim','compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'clang_sha256':digest(clang),'sources':plan['sources'],'fixture':unit['fixture'],'fixture_sha256':unit['fixture_sha256'],'reference_sha256':digest(oracle),'oracle_regenerated_or_normalized':False,'original_sdk_rendering_panic_outcomes_retained':12,'independent_rows':unit['oracle_rows'],'retained_rows':unit['extra_retained_rows'],'owner_groups':unit['owner_groups'],'qualification_plan_sha256':digest(PLAN),'runner_sha256':digest(Path(__file__)),'free_bytes_before':free(),'minimum_observed_free_bytes':free(),'start_guard_bytes':350*MIB,'reserve_bytes':250*MIB,'fresh_output_limit_bytes':30*MIB,'ELF_IR_limit_bytes':20*MIB,'archive_scratch_limit_bytes':10*MIB,'runs':[]}
def save(): receipt.write_text(json.dumps(r,indent=2)+'\n')
def verify():
    for name, expected in plan['sources'].items():
        assert digest(source/(name+'.cinch')) == expected, name
    assert digest(source/'Main.cinch') == unit['fixture_sha256']
    assert digest(oracle) == plan['oracle']['sha256']
    assert digest(compiler) == plan['compiler_sha256'] and digest(runtime) == plan['runtime_sha256']
def budget(start=False):
    total = sum(f.stat().st_size for f in folder.rglob('*') if f.is_file()) + (receipt.stat().st_size if receipt.exists() else 0)
    assert total <= 30*MIB, ('fresh output cap',total)
    observed=free(); r['minimum_observed_free_bytes']=min(r['minimum_observed_free_bytes'],observed)
    assert observed >= (350 if start else 250)*MIB, ('disk guard',observed)
    return total
def command(argv,label,timeout):
    budget(start=True); verify(); r['status']=label; r[label+'_command']=argv; save()
    started=time.monotonic()
    try: v=subprocess.run(argv,cwd=source,env=env,capture_output=True,text=True,timeout=timeout)
    except subprocess.TimeoutExpired as e:
        (output/(label+'.log')).write_bytes((e.stdout or b'')+(e.stderr or b'')); raise
    (output/(label+'.log')).write_text(v.stdout+v.stderr)
    r[label+'_seconds']=time.monotonic()-started; r[label+'_exit_code']=v.returncode; save(); budget()
    assert v.returncode==0,v.stdout+v.stderr
    return v
save()
try:
    command([str(compiler),'check',str(source/'Main.cinch')],'check',15)
    assert sum(f.stat().st_size for f in source.rglob('*') if f.is_file()) <= 5*MIB
    print(a.unit+' source PASS',flush=True)
    ir=output/'consumer.ll'; exe=output/'consumer'
    command([str(compiler),'emit-ir',str(source/'Main.cinch'),'-o',str(ir),'--rc-debug'],'emit',60)
    command([str(clang),'-O2',str(ir),str(runtime),'-ldl','-lpthread','-lm','-o',str(exe)],'link',30)
    assert ir.stat().st_size+exe.stat().st_size <= 20*MIB
    r['llvm_sha256']=digest(ir); r['executable_sha256']=digest(exe); save()
    for poison in [False,True]:
        budget(start=True); verify()
        e={**env,'CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':'1' if poison else '0','CINCH_ALLOC_STATS':'1'}
        r['status']='running-poison' if poison else 'running-normal'; save()
        v=subprocess.run([str(exe)],cwd=output,env=e,capture_output=True,text=True,timeout=30)
        (output/('poison.log' if poison else 'normal.log')).write_text(v.stdout+v.stderr)
        row={'poisoned':poison,'heap_limit':e['CINCH_HEAP_LIMIT'],'stack_limit':'8M','exit_code':v.returncode,'stdout':v.stdout,'stderr':v.stderr}
        r['runs'].append(row); save(); budget()
        assert v.returncode==0 and v.stdout=='',row
        assert 'live=0' in v.stderr and 'leaked=0' in v.stderr,row
    verify(); r['artifact_bytes']=budget(); r['status']='passed'; save()
    print(a.unit+' both profiles PASS; closed success awaits verified archive',flush=True)
except BaseException as error:
    r['status']='failed-or-paused-first-outcome'; r['failure']=repr(error); save(); raise
