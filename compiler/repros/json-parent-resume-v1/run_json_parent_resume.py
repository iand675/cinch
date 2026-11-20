#!/usr/bin/env python3
"""Resume the pinned remaining parent JSON tests without Cargo or replaying closed units."""
from pathlib import Path
import argparse, json, os, re, resource, shutil, signal, subprocess, time
from json_parent_gate_common import MIB, archive_project, dedup_passed, digest, durable_json, identity, matches, eligible_inputs
from json_parent_resume_verify import SHARED, read, profile_marker, verify_prefix

p=argparse.ArgumentParser()
p.add_argument('--prior-gate',type=Path,required=True)
p.add_argument('--existing-test-executable',type=Path,required=True)
p.add_argument('--work-dir',type=Path,required=True)
p.add_argument('--compiler-root',type=Path,required=True)
p.add_argument('--binding',type=Path,default=Path(__file__).with_name('prior-v2-binding.json'))
p.add_argument('--test-seconds',type=int,default=120)
p.add_argument('--apply-passed-input-dedup',action='store_true')
a=p.parse_args()
prior=a.prior_gate.resolve();compiler=a.compiler_root.resolve();work=a.work_dir.resolve();exe=a.existing_test_executable.resolve()
binding=read(a.binding)
assert str(prior)==binding['prior_gate'] and binding['closed_prefix_count']==15
assert binding['total_discovered_count']==45 and len(binding['remaining_tests'])==30
assert digest(exe)==binding['executable_sha256']=='025505f4c661b8a7787e5323a9d9851bbca51b63cfc122589df2b480d306cee0'
assert not work.exists(),'New epoch required; never overwrite prior or partial projects'
assert not os.environ.get('CINCH_TEST_COMPILER') and not os.environ.get('CINCH_TEST_INTERFACES')
os.sched_setaffinity(0,{3})
_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(64*MIB,hard))

def free():
 """Measure actual global available bytes without writing a progress file."""
 return shutil.disk_usage(compiler).free

def wait_headroom(reason):
 """Wait read-only with no child or new unit until the existing350MiB guard genuinely passes."""
 announced=False
 while free()<350*MIB:
  if not announced:print('WAIT nochild/nowrites '+reason+f'; free={free()}',flush=True);announced=True
  time.sleep(1)

def warm_runtime():
 """Bind the existing warm runtime and actual Root context without invoking any build path."""
 assert matches(runtime,runtime_identity)
 sources=[seed/'runtime/Cargo.toml',*list((seed/'runtime/src').rglob('*'))]
 assert max(p.stat().st_mtime_ns for p in sources if p.is_file())<=runtime.stat().st_mtime_ns,'Stale runtime would invoke hidden Cargo; refuse before launch'
 assert digest(prior/'root-context.json')==binding['context_sha256']
 assert digest(seed/'tests/lib_json_codec.rs')==binding['test_source_sha256']
 assert matches(exe,exe_identity)

def size(root):
 """Measure the complete current test's fresh ordinary output including all projects."""
 return sum(p.stat().st_size for p in root.rglob('*') if p.is_file() and not p.is_symlink())

wait_headroom('initial prefix verification; no epoch has been created')
verified=verify_prefix(prior,binding,compiler,exe)
invocation=verified['invocation'];discovery=verified['discovery'];tests=discovery['tests']
runtime_identity=invocation['runtime'];runtime=Path(runtime_identity['path']);exe_identity=discovery['test_executable']
canonicals=verified['canonicals'];complete=verified['closed'].copy()
assert len(complete)==15 and tests[15:]==binding['remaining_tests']
warm_runtime()
wait_headroom('initial new epoch creation')
work.mkdir(parents=True);(work/'units').mkdir();(work/'archives').mkdir()
for name in ['run_json_parent_resume.py','json_parent_resume_verify.py','json_parent_gate_common.py']:
 shutil.copyfile(Path(__file__).with_name(name),work/('executed-'+name))
shutil.copyfile(a.binding,work/'prior-binding.json')
durable_json(work/'invocation.json',{'status':'verified-prefix-resume-not-yet-complete','prior_gate':str(prior),'prior_binding_sha256':digest(a.binding),'source_root':str(compiler),'context_sha256':binding['context_sha256'],'runtime':runtime_identity,'test_executable':exe_identity,'source_test_sha256':binding['test_source_sha256'],'actual_tests':45,'owned_json_tests':42,'shared_controls':sorted(SHARED),'prior_closed_tests':complete,'remaining_tests':tests[15:],'no_Cargo_or_library_restore_path':True,'archive_codec':{'gzip_level':6,'tar':'pax'},'apply_passed_input_dedup':a.apply_passed_input_dedup})
durable_json(work/'verified-prior-prefix.json',{'status':'verified-existing-fifteen-closed-tests-not-reexecuted','prior_gate':str(prior),'binding_sha256':digest(a.binding),'closed':complete,'plain_passed_files_verified':len(verified['verified_plain']),'verified_canonical_keys':len(canonicals)})

def run_guarded(command,log,unit,seconds,cap):
 """Supervise bounded production with safe pause and explicit operator resume."""
 wait_headroom('before launching '+unit.name)
 warm_runtime();started=time.monotonic();minimum=free();reason=None;paused_seconds=0.0;pause_number=0
 env={**os.environ,'TMPDIR':str(unit/'projects'),'CINCH_KEEP_QUALIFICATION_PROJECTS':'1','RUST_MIN_STACK':'67108864','RUST_TEST_THREADS':'1'}
 (unit/'projects').mkdir(exist_ok=True)
 with log.open('xb') as output:
  child=subprocess.Popen(command,cwd=compiler,env=env,stdout=output,stderr=subprocess.STDOUT,start_new_session=True)
  while child.poll() is None:
   observed=free();minimum=min(minimum,observed)
   fresh=size(unit)
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
 assert free()>=250*MIB and fresh<=cap,('completed output guard',free(),fresh)
 result={'command':command,'exit_code':code,'seconds':time.monotonic()-started,'paused_seconds':paused_seconds,'active_seconds':time.monotonic()-started-paused_seconds,'minimum_observed_free_bytes':min(minimum,free()),'fresh_bytes':fresh,'log':str(log),'log_sha256':digest(log)}
 durable_json(unit/'process.json',result)
 assert code==0,('test/build failed; all files retained plain',result)
 return result


def verify_new_marker(project,kind):
 """Verify exact current project bytes and complete success evidence before any recovery mutation."""
 marker_path=project/('qualified.json' if kind=='runtime' else 'portable-qualified.json')
 marker=read(marker_path);profile_marker(marker,kind,compiler,exe,runtime_identity)
 nodes=list(project.rglob('*'));assert not any(p.is_symlink() for p in nodes)
 assert {str(p) for p in nodes if p.is_file()}=={str(project/Path(x['path'])) for x in marker['files']}|{str(marker_path)},'Unknown/missing project file'
 for row in marker['files']:
  rel=Path(row['path']);assert not rel.is_absolute() and '..' not in rel.parts
  assert matches(project/rel,{'path':str(project/rel),'sha256':row['sha256'],'bytes':row['bytes'],**row['identity']})
 if kind=='runtime':assert (project/'consumer').is_file() and (project/'consumer.ll').is_file()
 return marker_path,marker

text=(seed/'tests/lib_json_codec.rs').read_text()
starts=list(re.finditer(r'(?m)^fn (\w+)\([^\n]*\)\s*\{',text))
bodies={m[1]:text[m.end():starts[i+1].start() if i+1<len(starts) else len(text)] for i,m in enumerate(starts)}
for number in range(15,len(tests)):
 test=tests[number];unit=work/'units'/f'{number:03d}-{test}'
 # This wait occurs BEFORE mkdir/log/process and never discards completed progress.
 wait_headroom('next unstarted test '+str(number)+' '+test);warm_runtime();assert not unit.exists();unit.mkdir()
 print('START remaining test '+str(number)+' '+test,flush=True)
 result=run_guarded([str(exe),'--exact',test,'--nocapture','--test-threads=1'],unit/'test.log',unit,a.test_seconds,30*MIB)
 projects=sorted((unit/'projects').iterdir())
 assert all(x.is_dir() and not x.is_symlink() and x.name.startswith('cinch-json-qualification-') for x in projects)
 if test in SHARED:assert not projects;kind='shared-profile-control'
 elif test=='json_codec_declarations_have_canonical_documentation':assert not projects;kind='docs'
 elif test=='rust_json_field_kernel_compiles_for_wasm_arm_and_big_endian_targets':assert len(projects)==1;kind='portable'
 else:kind='runtime';assert len(projects)==bodies[test].count('run(')>0
 markers=[]
 for project in projects:markers.append((project,*verify_new_marker(project,kind)))
 receipt=unit/'qualified-test.json'
 durable_json(receipt,{'status':'passed-exact-parent-test-and-full-markers','test':test,'kind':kind,'process':result,'test_executable':exe_identity,'root_context_sha256':binding['context_sha256'],'markers':[{'path':str(path),'sha256':digest(path)} for _,path,_ in markers]})
 archives=[];dedup=[]
 for project,path,marker in markers:
  if kind=='runtime':
   archives.append(archive_project(project,path,work/'archives',receipt))
   dedup.append(dedup_passed(project,marker,canonicals,work/'input-ledgers',a.apply_passed_input_dedup))
 durable_json(unit/'closed.json',{'status':'passed-closed','test_receipt':str(receipt),'test_receipt_sha256':digest(receipt),'archives':archives,'input_dedup':dedup,'all_source_CIF_paths_plain_and_sealed':True,'never_compile_these_projects_again':True,'free_bytes_after':free()})
 complete.append({'index':number,'test':test,'closed':str(unit/'closed.json'),'closed_sha256':digest(unit/'closed.json'),'qualified_test_sha256':digest(receipt)})
 print('RELEASE closed '+test+f'; combined{len(complete)}/45 free={free()}',flush=True)
assert [x['test'] for x in complete]==tests and len(complete)==45
# Read-only revalidation prevents later source/context/ledger drift in the inherited prefix.
verify_prefix(prior,binding,compiler,exe)
durable_json(work/'gate.json',{'status':'passed-all45-parent-tests-across-preserved-prior15-and-new30','actual_tests':45,'owned_json_tests':42,'shared_profile_controls':3,'baseline_owned_json_tests':31,'added_owned_json_tests':11,'prior_gate':str(prior),'prior_binding_sha256':digest(a.binding),'prior_closed_count':15,'new_closed_count':30,'closed_tests':complete,'test_executable':exe_identity,'root_context_sha256':binding['context_sha256'],'runtime':runtime_identity,'no_Cargo_or_library_restore':True,'sources_and_CIF_plain':True,'no_native_family_performance_or_package_completion_claim':True})
