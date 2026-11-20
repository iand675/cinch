"""Read-only verification of a closed parent prefix, including lossless input sharing."""
from pathlib import Path
from decimal import Decimal
import gzip, hashlib, json, re, stat, tarfile
from json_parent_gate_common import digest, identity, matches, eligible_inputs, MIB
SHARED = frozenset('common::external_frontend::profile_tests::'+name for name in [
 'a_foreign_triple_must_match_its_own_profile_including_the_name',
 'a_native_request_is_accepted_for_the_hosts_own_triple',
 'a_profile_that_does_not_reclaim_asks_for_the_word'])


def read(path):
 """Read one existing ordinary JSON evidence file without modifying it."""
 p=Path(path);assert p.is_file() and not p.is_symlink(),p
 return json.loads(p.read_text())


def file_expected(project,row):
 """Convert a Rust marker's relative file record to exact absolute identity."""
 rel=Path(row['path']);assert not rel.is_absolute() and '..' not in rel.parts
 return {'path':str(Path(project)/rel),'sha256':row['sha256'],'bytes':row['bytes'],**row['identity']}


def comparable(row):
 """Select complete recoverable identity fields, excluding only storage location."""
 return {k:row[k] for k in ['sha256','bytes','mode','uid','gid','mtime_ns']}


def profile_marker(marker,kind,compiler,executable,runtime):
 """Require exact producer/root/runtime and all successful profile or object outcomes."""
 assert marker['kind']==kind and Path(marker['source_root'])==compiler
 assert Path(marker['test_process']['path']).resolve()==executable
 assert marker['test_process']['sha256']==digest(executable)
 if kind=='runtime':
  assert marker['status']=='passed-both-memory-profiles' and len(marker['runs'])==2
  assert {x['poison'] for x in marker['runs']}=={False,True}
  assert Path(marker['runtime_path']).resolve()==Path(runtime['path']).resolve()
  warm=marker['warm_runtime'];assert comparable({'sha256':warm['sha256'],'bytes':warm['bytes'],**warm['identity']})==comparable(runtime)
  for run in marker['runs']:
   assert run['exit_code']==0 and run['stack_limit']=='8M' and run['leak_check']=='1'
   assert run['heap_limit']==('256M' if run['poison'] else '32M') and run['rc_debug']==('1' if run['poison'] else '0')
   stderr=bytes(run['stderr']).decode('utf-8',errors='replace')
   assert re.search(r'(?:^|\s)live=0(?:\s|$)',stderr) and re.search(r'(?:^|\s)leaked=0(?:\s|$)',stderr)
 else:
  assert kind=='portable' and marker['status']=='passed-three-portable-objects'
  assert len(marker['runs'])==3 and {x['target'] for x in marker['runs']}=={'wasm32-unknown-unknown','aarch64-unknown-linux-gnu','powerpc64-unknown-linux-gnu'}
  assert all(x['exit_code']==0 and x['executed'] is False for x in marker['runs'])


def verify_archive(index_path,marker_path,marker,test_receipt):
 """Verify archived ELF/IR against marker identities, full hashes and exact PAX metadata."""
 a=read(index_path);assert a['status']=='verified-closed-test-and-both-profile-success' and a['verified_before_unlink']
 assert a['successful_test_receipt']==str(test_receipt) and a['successful_test_receipt_sha256']==digest(test_receipt)
 assert a['marker']==str(marker_path) and a['marker_sha256']==digest(marker_path)
 arc=Path(a['archive']['path']);assert matches(arc,a['archive'])
 assert a['archive']['bytes']<=10*MIB
 project=Path(marker_path).parent
 expected={str(project/Path(x['path'])):file_expected(project,x) for x in marker['files']}
 members=a['members'];assert {Path(x['path']).name for x in members}=={'consumer','consumer.ll'} and len(members)==2
 for row in members:
  assert row['path'] in expected and comparable(row)==comparable(expected[row['path']])
  assert not Path(row['path']).exists(),'Closed archived member unexpectedly recreated'
 h=hashlib.sha256()
 with gzip.open(arc,'rb') as f:
  while block:=f.read(MIB):h.update(block)
 assert h.hexdigest()==a['decompressed_tar_sha256']
 with tarfile.open(arc,'r:gz') as t:
  got=t.getmembers();assert len(got)==2
  for entry,row in zip(got,members):
   assert entry.isfile() and entry.name==Path(row['path']).name and entry.size==row['bytes']
   assert stat.S_IMODE(entry.mode)==row['mode'] and entry.uid==row['uid'] and entry.gid==row['gid']
   assert int(Decimal(entry.pax_headers['mtime'])*1_000_000_000)==row['mtime_ns']
   h=hashlib.sha256()
   with t.extractfile(entry) as f:
    while block:=f.read(MIB):h.update(block)
   assert h.hexdigest()==row['sha256']
 return {x['path'] for x in members}


def verify_ledger(item,project,marker,verified_plain,verified_canonical_paths):
 """Require durable applied-plan linkage and prove every changed timestamp through its canonical."""
 plan_path=Path(item['plan']);assert digest(plan_path)==item['plan_sha256']
 plan=read(plan_path);assert plan['project']==str(project) and plan['sealed_success_only'] and plan['all_sources_and_interfaces_remain_plain']
 assert item['applied'] is True and plan['status']=='reviewed-policy-exact-plan'
 applied_path=plan_path.with_name(plan_path.name.removesuffix('.plan.json')+'.applied.json')
 applied=read(applied_path)
 assert applied['status']=='applied-and-reverified' and applied['plan']==str(plan_path) and applied['plan_sha256']==digest(plan_path)
 assert applied['replacements']==item['replacements']==len(plan['rows'])
 expected={str(project/Path(x['path'])):file_expected(project,x) for x in marker['files']}
 changed={}
 allowed={str(x) for x in eligible_inputs(project,marker)}
 for row in plan['rows']:
  own=row['original'];canonical=row['canonical'];assert own['path'] in allowed and own['path'] in expected
  assert comparable(own)==comparable(expected[own['path']])
  assert canonical['path'] in verified_plain and canonical['path'] in verified_canonical_paths,'Canonical has no earlier verified eligible closed-success proof'
  assert matches(canonical['path'],canonical)
  assert comparable(canonical)==comparable(verified_plain[canonical['path']])
  assert all(own[k]==canonical[k] for k in ['sha256','bytes','mode','uid','gid'])
  assert row['restore_original_mtime_ns']==own['mtime_ns']
  assert row['shared_visible_mtime_ns_after_apply']==canonical['mtime_ns']
  assert matches(own['path'],own,mtime=False) and identity(own['path'])['mtime_ns']==canonical['mtime_ns']
  assert Path(own['path']).samefile(canonical['path'])
  assert own['path'] not in changed;changed[own['path']]=canonical['mtime_ns']
 return changed


def verify_prefix(prior,binding,compiler,executable):
 """Verify every pinned closed unit, preserving order and rebuilding only proven canonicals."""
 prior=Path(prior);compiler=Path(compiler);executable=Path(executable).resolve()
 library=binding['archived_normal_library_not_needed_or_restored']
 assert digest(library['index'])==library['sha256']
 for rel,expected in binding['pinned_files'].items():assert digest(prior/rel)==expected,rel
 invocation=read(prior/'invocation.json');discovery=read(prior/'discovered-tests.json')
 assert invocation['source_root']==str(compiler) and invocation['context_sha256']==binding['context_sha256']
 assert discovery['root_context_sha256']==binding['context_sha256'] and discovery['actual_tests']==45
 assert discovery['tests']==binding['tests'] and len(set(discovery['tests']))==45
 assert set(discovery['shared_profile_controls'])==SHARED
 assert discovery['owned_json_tests']==42 and discovery['baseline_owned_json_tests']==31
 assert matches(executable,discovery['test_executable']) and digest(executable)==binding['executable_sha256']
 assert matches(invocation['runtime']['path'],invocation['runtime'])
 assert digest(seed/'tests/lib_json_codec.rs')==binding['test_source_sha256']
 assert digest(seed/'tests/common/external_frontend.rs')==binding['common_profile_source_sha256']
 context=read(prior/'root-context.json')
 assert context['warm_runtime_sha256']==binding['runtime_sha256'] and context['actual_cli_sha256']==binding['actual_cli_sha256']
 assert context['test_source_sha256']==binding['test_source_sha256']
 for relative,expected in context['actual_source_hashes'].items():assert digest(compiler.parent/relative)==expected
 verified_plain={};canonicals={};closed=[]
 for number in range(binding['closed_prefix_count']):
  test=discovery['tests'][number];unit=prior/'units'/f'{number:03d}-{test}'
  close=read(unit/'closed.json');qpath=Path(close['test_receipt'])
  assert qpath==unit/'qualified-test.json' and close['test_receipt_sha256']==digest(qpath)
  qualified=read(qpath);process=read(unit/'process.json')
  assert qualified['status']=='passed-exact-parent-test-and-full-markers' and qualified['test']==test
  assert qualified['process']==process and process['exit_code']==0
  cap=30*MIB
  exception=binding['completed_scoped_output_exceptions'].get(str(number))
  if exception:
   assert exception['test']==test
   resumed=read(unit/'resumed-001.json');request=Path(resumed['request'])
   assert digest(request)==resumed['request_sha256']
   approval=read(request);assert approval['action']=='resume-existing-process' and approval['approval']
   assert resumed['cap_bytes']==approval['cap_bytes']==exception['cap_bytes'];cap=exception['cap_bytes']
  assert process['fresh_bytes']<=cap and process['minimum_observed_free_bytes']>=250*MIB
  assert process['command']==[str(executable),'--exact',test,'--nocapture','--test-threads=1']
  assert process['log']==str(unit/'test.log') and digest(unit/'test.log')==process['log_sha256']
  assert re.search(r'test result: ok\. 1 passed; 0 failed;', (unit/'test.log').read_text())
  assert qualified['root_context_sha256']==binding['context_sha256'] and qualified['test_executable']==discovery['test_executable']
  assert close['status']=='passed-closed' and close['all_source_CIF_paths_plain_and_sealed'] and close['never_compile_these_projects_again']
  kind=qualified['kind'];markers=qualified['markers']
  projects=sorted((unit/'projects').iterdir())
  assert all(x.is_dir() and not x.is_symlink() for x in projects)
  if test in SHARED:assert kind=='shared-profile-control' and not projects and not markers
  elif test=='json_codec_declarations_have_canonical_documentation':assert kind=='docs' and not projects and not markers
  else:assert kind=='runtime' and len(projects)==len(markers)>0
  assert len(close['archives'])==len(markers) and len(close['input_dedup'])==len(markers)
  for marker_ref,archive_ref,ledger_ref in zip(markers,close['archives'],close['input_dedup']):
   marker_path=Path(marker_ref['path']);project=marker_path.parent
   assert project in projects and digest(marker_path)==marker_ref['sha256']
   marker=read(marker_path);profile_marker(marker,kind,compiler,executable,invocation['runtime'])
   index=Path(archive_ref['index']);assert digest(index)==archive_ref['index_sha256']
   archived=verify_archive(index,marker_path,marker,qpath)
   changed=verify_ledger(ledger_ref,project,marker,verified_plain,{str(p) for p in canonicals.values()})
   expected_plain={str(project/Path(x['path'])) for x in marker['files']} - archived
   expected_plain.add(str(marker_path))
   nodes=list(project.rglob('*'));assert not any(p.is_symlink() for p in nodes)
   assert {str(p) for p in nodes if p.is_file()}==expected_plain,'Unknown/missing project file'
   for row in marker['files']:
    expected=file_expected(project,row);path=expected['path']
    if path in archived:continue
    assert matches(path,expected,mtime=path not in changed),path
    if path in changed:assert identity(path)['mtime_ns']==changed[path]
    verified_plain[path]=identity(path)
   for path in eligible_inputs(project,marker):
    own=identity(path);key=tuple(own[k] for k in ['sha256','bytes','mode','uid','gid']);canonicals.setdefault(key,path)
  closed.append({'index':number,'test':test,'closed':str(unit/'closed.json'),'closed_sha256':digest(unit/'closed.json'),'qualified_test_sha256':digest(qpath)})
 # No later closed/active unit is silently ignored or repeated.
 for d in (prior/'units').iterdir():
  if d.is_dir() and int(d.name.split('-',1)[0])>=binding['closed_prefix_count']:
   assert not any(d.iterdir()),('Later unit has evidence; refusing overlap',d)
 return {'closed':closed,'canonicals':canonicals,'verified_plain':verified_plain,'invocation':invocation,'discovery':discovery}
