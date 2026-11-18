#!/usr/bin/env python3
"""Run the actual Root JSON target serially with retained evidence and success-only recovery.

Proposal only: Root reviews/imports the Rust patch and explicitly starts this driver.
No compiler, target count or frozen toolchain is silently substituted.
"""
from pathlib import Path
import argparse, json, os, re, resource, shutil, signal, subprocess, time
from json_parent_gate_common import MIB, archive_project, dedup_passed, digest, durable_json, identity, matches

p=argparse.ArgumentParser()
p.add_argument('--compiler-root',type=Path,required=True)
p.add_argument('--work-dir',type=Path,required=True)
p.add_argument('--expected-tests',type=int,required=True)
p.add_argument('--baseline-tests',type=int,required=True)
p.add_argument('--added-tests',type=int,default=11)
p.add_argument('--cargo-wrapper',type=Path,default=Path('/tmp/cinch-stdlib-root-cargo-gate.sh'))
p.add_argument('--context',type=Path,required=True,help='Root actual producer/BDF/runtime provenance JSON; retained as identity evidence')
p.add_argument('--release',action='store_true',help='Use only if Root deliberately selects its warm release profile')
p.add_argument('--build-cap-mib',type=int,default=160)
p.add_argument('--build-seconds',type=int,default=300)
p.add_argument('--test-seconds',type=int,default=120)
p.add_argument('--apply-passed-input-dedup',action='store_true',help='Enable ONLY after Root reviews this exact helper/policy; default is dry plans')
a=p.parse_args()
compiler=a.compiler_root.resolve();work=a.work_dir.resolve();runtime=compiler/'runtime/target/release/libcinchruntime.a'
assert compiler.is_dir() and a.context.is_file() and a.cargo_wrapper.is_file()
assert a.expected_tests==a.baseline_tests+a.added_tests
assert not os.environ.get('CINCH_TEST_COMPILER') and not os.environ.get('CINCH_TEST_INTERFACES'),'Require actual in-process Root producer'
assert not work.exists(),'Refuse an existing evidence epoch; no retry into retained projects'
os.sched_setaffinity(0,{3})
_, stack_hard=resource.getrlimit(resource.RLIMIT_STACK)
resource.setrlimit(resource.RLIMIT_STACK,(64*MIB,stack_hard))
def free():
 """Measure actual available bytes on the Root filesystem."""
 return shutil.disk_usage(compiler).free
assert free()>=350*MIB,('initial start guard',free())
work.mkdir(parents=True);(work/'units').mkdir();(work/'archives').mkdir()
durable_json(work/'invocation.json',{'status':'prepared-not-qualified','args':{k:str(v) if isinstance(v,Path) else v for k,v in vars(a).items()},'source_root':str(compiler),'context_path':str(a.context),'context_sha256':digest(a.context),'runtime':identity(runtime),'cpu':[3],'source_test_sha256':digest(compiler/'tests/lib_json_codec.rs'),'no_compiler_family_or_promotion_claim':True})
shutil.copyfile(a.context,work/'root-context.json')
shutil.copyfile(Path(__file__),work/'executed-driver.py')
shutil.copyfile(Path(__file__).with_name('json_parent_gate_common.py'),work/'executed-common.py')
context_hash=digest(a.context);runtime_identity=identity(runtime)

def warm_runtime():
 """Refuse runtime/context drift and hidden stale-runtime Cargo rebuilding."""
 assert matches(runtime,runtime_identity),'Warm linked runtime identity changed'
 sources=[compiler/'runtime/Cargo.toml',*list((compiler/'runtime/src').rglob('*'))]
 assert max(x.stat().st_mtime_ns for x in sources if x.is_file())<=runtime.stat().st_mtime_ns,'Runtime is stale; no hidden runtime Cargo build permitted'
 assert digest(a.context)==context_hash,'Root producer context changed'

def size(root):
 """Measure ordinary bytes produced inside one bounded evidence unit."""
 return sum(p.stat().st_size for p in root.rglob('*') if p.is_file() and not p.is_symlink())
def target_snapshot():
 """Record warm target identities to count changed and new Cargo outputs."""
 target=Path(os.environ.get('CARGO_TARGET_DIR',compiler/'target'))
 if not target.is_absolute():target=compiler/target
 target=target.resolve()
 return {str(p):(p.stat().st_ino,p.stat().st_size,p.stat().st_mtime_ns) for p in target.rglob('*') if p.is_file() and not p.is_symlink()}

def run_guarded(command,log,unit,seconds,cap,baseline=None):
 """Supervise bounded production with safe pause and explicit operator resume."""
 assert free()>=350*MIB,('phase start guard',free())
 warm_runtime();started=time.monotonic();minimum=free();reason=None;paused_seconds=0.0;pause_number=0
 env={**os.environ,'TMPDIR':str(unit/'projects'),'CINCH_KEEP_QUALIFICATION_PROJECTS':'1','RUST_MIN_STACK':'67108864','RUST_TEST_THREADS':'1'}
 (unit/'projects').mkdir(exist_ok=True)
 with log.open('xb') as output:
  child=subprocess.Popen(command,cwd=compiler,env=env,stdout=output,stderr=subprocess.STDOUT,start_new_session=True)
  while child.poll() is None:
   observed=free();minimum=min(minimum,observed)
   fresh=size(unit)
   if baseline is not None:
    current=target_snapshot();fresh+=sum(v[1] for k,v in current.items() if baseline.get(k)!=v)
   if observed<270*MIB:reason=f'soft pause before250MiB reserve: {observed}'
   elif fresh>cap:reason=f'fresh output cap: {fresh}>{cap}'
   elif time.monotonic()-started-paused_seconds>seconds:reason=f'active deadline {seconds}s'
   if reason:
    pause_number+=1
    try:os.killpg(child.pid,signal.SIGSTOP)
    except ProcessLookupError:pass
    pause=unit/f'pause-{pause_number:03d}.json';resume=unit/f'RESUME-{pause_number:03d}.json'
    durable_json(pause,{'status':'PROCESS-GROUP-PAUSED-SUPERVISOR-ALIVE','pid_pgid':child.pid,'reason':reason,'command':command,'fresh_bytes':fresh,'cap_bytes':cap,'free_bytes':observed,'source_and_all_artifacts_retained':True,'resume_request':str(resume),'resume_instruction':'Root must explicitly create this RESUME JSON with action=resume-existing-process and nonempty approval. Fresh350MiB required. cap_bytes/additional_active_seconds are optional ONLY for a separately Root-approved scoped override; recorded request is retained. Elapsed time never grants approval.'})
    print('PAUSED '+str(child.pid)+'; explicit Root request required: '+str(resume),flush=True)
    paused_at=time.monotonic()
    while True:
     request=None
     if resume.is_file():
      try:request=json.loads(resume.read_text())
      except (ValueError,OSError):pass
     if request and request.get('action')=='resume-existing-process' and request.get('approval'):
      proposed_cap=request.get('cap_bytes',cap);additional=request.get('additional_active_seconds',0)
      fresh=size(unit)
      if baseline is not None:
       now=target_snapshot();fresh+=sum(v[1] for k,v in now.items() if baseline.get(k)!=v)
      deadline_ok=('deadline' not in reason or additional>0)
      if isinstance(proposed_cap,int) and isinstance(additional,int) and additional>=0 and fresh<=proposed_cap and free()>=350*MIB and deadline_ok:
       warm_runtime();cap=proposed_cap;seconds+=additional
       durable_json(unit/f'resumed-{pause_number:03d}.json',{'status':'explicitly-resumed-existing-process','request':str(resume),'request_sha256':digest(resume),'cap_bytes':cap,'active_seconds':seconds,'free_bytes':free()})
       if child.poll() is None:
        try:os.killpg(child.pid,signal.SIGCONT)
        except ProcessLookupError:pass
       paused_seconds+=time.monotonic()-paused_at;reason=None;break
     time.sleep(1)
   time.sleep(0.5)
  output.flush();os.fsync(output.fileno())
 code=child.returncode
 fresh=size(unit)
 if baseline is not None:
  current=target_snapshot();fresh+=sum(v[1] for k,v in current.items() if baseline.get(k)!=v)
 assert free()>=250*MIB and fresh<=cap,('completed output guard',free(),fresh)
 result={'command':command,'exit_code':code,'seconds':time.monotonic()-started,'paused_seconds':paused_seconds,'active_seconds':time.monotonic()-started-paused_seconds,'minimum_observed_free_bytes':min(minimum,free()),'fresh_bytes':fresh,'log':str(log),'log_sha256':digest(log)}
 durable_json(unit/'process.json',result)
 assert code==0,('test/build failed; all files retained plain',result)
 return result

warm_runtime()
build=work/'build';build.mkdir()
baseline=target_snapshot()
command=[str(a.cargo_wrapper),'test','--test','lib_json_codec','--no-run','--message-format=json']
if a.release:command.append('--release')
# The no-run Cargo phase has a separately reviewed build cap, not the per-test30MiB cap.
assert free()>=250*MIB+a.build_cap_mib*MIB,'Insufficient worst-case reserve for reviewed Cargo build cap'
run_guarded(command,build/'cargo.log',build,a.build_seconds,a.build_cap_mib*MIB,baseline)
artifacts=[]
for line in (build/'cargo.log').read_text().splitlines():
 try:row=json.loads(line)
 except ValueError:continue
 if row.get('reason')=='compiler-artifact' and row.get('target',{}).get('name')=='lib_json_codec' and row.get('executable'):artifacts.append(Path(row['executable']))
assert len(set(artifacts))==1,artifacts
exe=artifacts[-1].resolve();exe_identity=identity(exe)
listed=subprocess.run([str(exe),'--list','--format','terse'],cwd=compiler,capture_output=True,text=True,check=True,timeout=30)
(work/'test-list.txt').write_text(listed.stdout+listed.stderr)
tests=[x.removesuffix(': test') for x in listed.stdout.splitlines() if x.endswith(': test')]
assert len(tests)==len(set(tests))==a.expected_tests,('actual discovered test count',len(tests),a.expected_tests)
text=(compiler/'tests/lib_json_codec.rs').read_text()
starts=list(re.finditer(r'(?m)^fn (\w+)\([^\n]*\)\s*\{',text));bodies={m[1]:text[m.end():starts[i+1].start() if i+1<len(starts) else len(text)] for i,m in enumerate(starts)}
new_names=[n for n in tests if n.startswith('json_partial_')]
assert len(new_names)==a.added_tests,('new test names',new_names)
durable_json(work/'discovered-tests.json',{'test_executable':exe_identity,'tests':tests,'actual_tests':len(tests),'baseline_tests':a.baseline_tests,'added_tests':a.added_tests,'new_test_names':new_names,'root_context_sha256':context_hash})
canonicals={};complete=[]

def verify_marker(project,kind):
 """Validate exact files and every required successful runtime or object observation."""
 path=project/('qualified.json' if kind=='runtime' else 'portable-qualified.json')
 assert path.is_file(),('Missing full success marker',project)
 marker=json.loads(path.read_text());assert marker['kind']==kind and marker['source_root']==str(compiler)
 assert marker['test_process']['sha256']==exe_identity['sha256'] and Path(marker['test_process']['path']).resolve()==exe
 for row in marker['files']:
  rel=Path(row['path']);assert not rel.is_absolute() and '..' not in rel.parts
  assert matches(project/rel,{'path':str(project/rel),'sha256':row['sha256'],'bytes':row['bytes'],**row['identity']}),rel
 if kind=='runtime':
  assert marker['status']=='passed-both-memory-profiles' and len(marker['runs'])==2
  assert {r['poison'] for r in marker['runs']}=={False,True}
  assert Path(marker['runtime_path']).resolve()==runtime.resolve()
  warm=marker['warm_runtime'];assert warm['sha256']==runtime_identity['sha256'] and warm['bytes']==runtime_identity['bytes']
  for r in marker['runs']:
   assert r['exit_code']==0 and r['stack_limit']=='8M' and r['leak_check']=='1'
   assert r['heap_limit']==('256M' if r['poison'] else '32M') and r['rc_debug']==('1' if r['poison'] else '0')
   stderr=bytes(r['stderr']).decode('utf-8',errors='replace')
   assert re.search(r'(?:^|\s)live=0(?:\s|$)',stderr) and re.search(r'(?:^|\s)leaked=0(?:\s|$)',stderr),stderr
  assert (project/'consumer').is_file() and (project/'consumer.ll').is_file()
 else:
  assert marker['status']=='passed-three-portable-objects' and len(marker['runs'])==3
  assert {x['target'] for x in marker['runs']}=={'wasm32-unknown-unknown','aarch64-unknown-linux-gnu','powerpc64-unknown-linux-gnu'}
  assert all(r['exit_code']==0 and r['executed'] is False for r in marker['runs'])
 return path,marker

for number,test in enumerate(tests):
 assert matches(exe,exe_identity),'Parent test executable changed'
 unit=work/'units'/f'{number:03d}-{test}';unit.mkdir()
 print('START bounded parent test '+test,flush=True)
 result=run_guarded([str(exe),'--exact',test,'--nocapture','--test-threads=1'],unit/'test.log',unit,a.test_seconds,30*MIB)
 projects=sorted((unit/'projects').iterdir())
 assert all(x.is_dir() and x.name.startswith('cinch-json-qualification-') and not x.is_symlink() for x in projects)
 if test=='json_codec_declarations_have_canonical_documentation':assert not projects;kind='docs'
 elif test=='rust_json_field_kernel_compiles_for_wasm_arm_and_big_endian_targets':assert len(projects)==1;kind='portable'
 else:
  kind='runtime';assert len(projects)==bodies[test].count('run(')>0,('unexpected project count',test,len(projects))
 markers=[]
 for project in projects:
  path,marker=verify_marker(project,kind);markers.append((project,path,marker))
 # Complete test success and every expected marker are established before any archive/unlink.
 receipt=unit/'qualified-test.json'
 durable_json(receipt,{'status':'passed-exact-parent-test-and-full-markers','test':test,'kind':kind,'process':result,'test_executable':exe_identity,'root_context_sha256':context_hash,'markers':[{'path':str(path),'sha256':digest(path)} for _,path,_ in markers]})
 archives=[];dedup=[]
 for project,path,marker in markers:
  if kind=='runtime':archives.append(archive_project(project,path,work/'archives',receipt))
  if kind=='runtime':dedup.append(dedup_passed(project,marker,canonicals,work/'input-ledgers',a.apply_passed_input_dedup))
 durable_json(unit/'closed.json',{'status':'passed-closed','test_receipt':str(receipt),'test_receipt_sha256':digest(receipt),'archives':archives,'input_dedup':dedup,'all_source_CIF_paths_plain_and_sealed':True,'never_compile_these_projects_again':True,'free_bytes_after':free()})
 complete.append(test)
 print('RELEASE closed '+test+f'; {len(complete)}/{len(tests)}, free={free()}',flush=True)
durable_json(work/'gate.json',{'status':'passed-all-discovered-parent-JSON-tests','tests':complete,'actual_test_count':len(tests),'baseline_tests':a.baseline_tests,'added_tests':a.added_tests,'test_executable':exe_identity,'root_context_sha256':context_hash,'runtime':runtime_identity,'source_and_CIF_remain_plain':True,'source_epoch':'Actual Root producer/dependencies; not relabeled child855/c4','no_native_family_performance_or_full92_completion_claim':True})
