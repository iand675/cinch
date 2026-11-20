#!/usr/bin/env python3
"""Actual frozen2f protocol qualification; no epoch substitution or compiler bootstrap."""
from pathlib import Path
import hashlib,json,os,re,resource,subprocess,time
root=Path('/workspace/dasl-stdlib-json');inputs=root/'build/json-addressability-trace-native-input-v1';directory=root/'build/json-addressability-trace-native-v1'
receipt=root/'compiler/evidence/stdlib/stdlib-json-addressability-trace-native-v1-results.json'
source=json.loads((inputs/'report.json').read_text());rust=json.loads((root/'compiler/evidence/stdlib/stdlib-json-addressability-trace-rust-v1-results.json').read_text())
base=Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen');compiler=base/'cinch';interfaces=base/'base-interfaces.json';objects_path=base/'objects.json';runtime=base/'libcinchruntime.a';clang=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
assert not directory.exists() and not receipt.exists(),'inspect durable state; refuse repeated qualification'
assert digest(compiler)==source['compiler_sha256']=='2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f'
assert digest(interfaces)==source['interfaces_sha256']=='1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256'
assert digest(objects_path)=='22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624'
objects=json.loads(objects_path.read_text());provider=Path(objects['object'])
assert digest(provider)=='46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64'
assert digest(runtime)=='80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141'
assert source['sources']==rust['sources'] and digest(inputs/'Bundle.cinch')==source['bundle_sha256']
assert digest(inputs/'Main.cinch')==source['fixture_sha256']==rust['fixture_sha256']
for name,expected in source['sources'].items():assert digest(inputs/(name+'.cinch'))==expected
free=os.statvfs(root).f_bavail*os.statvfs(root).f_frsize
assert free>=250*1024*1024,'Root guard requires250MiB before a runtime unit'
directory.mkdir();os.sched_setaffinity(0,{2});_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(65536*1024,hard))
env={**os.environ,'CINCH_HEAP_LIMIT':'2G','CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}
output=directory/'consumer';ir=directory/'consumer.ll';started=time.monotonic();deadline=started+90
emit=[str(compiler),'--emit-ir','--rc-debug','--interfaces',str(interfaces),'--objects',str(objects_path),str(inputs/'Bundle.cinch'),str(ir)]
link=[str(clang),'-O2',str(ir),str(provider),str(runtime),'-ldl','-lpthread','-lm','-o',str(output)]
report={'description':'Actual frozen Native current-source79 callback-count/addressability rows +typed-interface boundary; no full1464/package claim','frontend':'native-raw','classification_epoch':'2f077/1d22/22ab/46a7/80bd; no newer root base/runtime adoption','compiler_sha256':digest(compiler),'interfaces_sha256':digest(interfaces),'objects_manifest_sha256':digest(objects_path),'provider_object_sha256':digest(provider),'observed_runtime_archive_sha256':digest(runtime),'snapshot':source['snapshot'],'sources':source['sources'],'fixture_sha256':source['fixture_sha256'],'bundle_sha256':source['bundle_sha256'],'frozen_inputs':str(inputs),'frozen_input_report_sha256':digest(inputs/'report.json'),'input_preparation_status':source['status'],'rust_protocol_receipt_sha256':digest(root/'compiler/evidence/stdlib/stdlib-json-addressability-trace-rust-v1-results.json'),'runner_sha256':digest(Path(__file__)),'raw_compile_command':emit,'raw_link_command':link,'opt_level':2,'cpu_affinity':[2],'compiler_host_stack_kib':65536,'consumer_stack_limit':'8M','free_bytes_before':free,'runs':[],'status':'emitting'}
def persist():receipt.write_text(json.dumps(report,indent=2)+'\n')
def budget():
    total=sum(p.stat().st_size for p in directory.rglob('*') if p.is_file());assert total<=20*1024*1024,f'20MiB output budget exceeded: {total}';return total
def stage(command,label,environment):
    left=deadline-time.monotonic();assert left>0,'90s unit deadline'
    try:run=subprocess.run(command,cwd=directory,env=environment,capture_output=True,text=True,timeout=left)
    except subprocess.TimeoutExpired as error:
        text=(error.stdout.decode() if isinstance(error.stdout,bytes) else error.stdout or '')+(error.stderr.decode() if isinstance(error.stderr,bytes) else error.stderr or '')
        (directory/(label+'.log')).write_text(text);report.update(status='deadline-no-verdict',failed_stage=label,elapsed_seconds=time.monotonic()-started);persist();raise
    text=run.stdout+run.stderr;sidecar=ir.with_suffix('.ll.diagnostics')
    if sidecar.exists() and sidecar.stat().st_size:text+='\n'+sidecar.read_text()
    (directory/(label+'.log')).write_text(text)
    if run.returncode:
        report.update(status='refused',failed_stage=label,exit_code=run.returncode,diagnostic=text,elapsed_seconds=time.monotonic()-started);persist();raise AssertionError(f'{label}: {text}')
    return run
persist();stage(emit,'emit',env);report.update(status='linking',llvm_sha256=digest(ir));budget();persist()
stage(link,'link',env);report.update(status='running',build_seconds=time.monotonic()-started,artifact_bytes=budget());persist()
for poisoned in [False,True]:
    environment={**env,'CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M','CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'1'}
    run=stage([str(output)],'poisoned' if poisoned else 'ordinary',environment)
    assert run.stdout=='' and 'leaked=0' in run.stderr,run.stdout+run.stderr
    line=next(line for line in run.stderr.splitlines() if line.startswith('cinch-alloc:'));stats={key:int(value) for key,value in re.findall(r'([a-z-]+)=(\d+)',line)};assert stats['live']==0,stats
    report['runs'].append({'poisoned_free_validation':poisoned,'heap_limit':environment['CINCH_HEAP_LIMIT'],'stdout':run.stdout,'stats':stats,'leaked':0});persist()
assert digest(runtime)==report['observed_runtime_archive_sha256'];report.update(status='passed',elapsed_seconds=time.monotonic()-started,artifact_bytes=budget());persist()
print('Actual2f/1d22/22ab/46a7/80bd current-source79 callback-count rows pass both profiles, zero live/leaks.',flush=True)
