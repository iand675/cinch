#!/usr/bin/env python3
"""Verify current syntax-partial sources against the unchanged historical JSON observations."""
from pathlib import Path
import gzip, hashlib, json, stat, tarfile
ROOT = Path(__file__).resolve().parents[1]
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p): return json.loads((ROOT/p).read_text())
planpath = 'compiler/evidence/stdlib/stdlib-json-syntax-partial-affected1494-plan.json'
plan = read(planpath)
for module, expected in plan['sources'].items():
    assert sha(ROOT/'seed/prelude'/(module.replace('.','/')+'.cinch')) == expected, module
oracle = ROOT/plan['oracle']['path']
assert sha(oracle) == plan['oracle']['sha256']
assert {k:len(v) for k,v in json.loads(oracle.read_text()).items()} == {'method_errors':288,'tagged':1152,'updates':54}
units = {}; minimum_free = []; fresh_bytes = []
for unit in plan['units']:
    name = unit['unit']
    rel = f'compiler/evidence/stdlib/stdlib-json-syntax-affected1494-{name}-rust-v1-results.json'
    r = read(rel)
    assert r['status'] == 'passed' and r['sources'] == plan['sources']
    assert r['compiler_sha256'] == plan['compiler_sha256'] and r['runtime_sha256'] == plan['runtime_sha256']
    assert r['reference_sha256'] == plan['oracle']['sha256'] and not r['oracle_regenerated_or_normalized']
    assert r['independent_rows'] == unit['oracle_rows'] and r['retained_rows'] == unit['extra_retained_rows'] and r['owner_groups'] == unit['owner_groups']
    assert r['check_exit_code'] == r['emit_exit_code'] == r['link_exit_code'] == 0
    assert len(r['runs']) == 2 and {x['poisoned'] for x in r['runs']} == {False,True}
    for run in r['runs']:
        assert run['exit_code'] == 0 and run['stdout'] == '' and 'live=0' in run['stderr'] and 'leaked=0' in run['stderr']
    assert r['fixture_sha256'] == unit['fixture_sha256'] == sha(ROOT/unit['fixture'])
    project = ROOT/f'build/json-syntax-affected1494-{name}-v1'
    for module, expected in plan['sources'].items(): assert sha(project/'source'/(module+'.cinch')) == expected
    assert sha(project/'source/Main.cinch') == unit['fixture_sha256']
    assert sha(project/'executed-runner.py') == r['runner_sha256']
    assert sha(project/'qualification-plan.json') == r['qualification_plan_sha256'] == sha(ROOT/planpath)
    arcrel = f'compiler/evidence/stdlib/stdlib-json-syntax-affected1494-{name}-v1-functional-archive.json'
    a = read(arcrel); assert a['status'] == 'verified-closed-success' and a['verified_before_unlink']
    assert a['execution_receipt'] == rel and a['execution_receipt_sha256'] == sha(ROOT/rel)
    arc = ROOT/a['archive_path']; assert sha(arc) == a['archive_sha256'] and arc.stat().st_size == a['archive_bytes'] <= 10*1024*1024
    h = hashlib.sha256()
    with gzip.open(arc,'rb') as f:
        while b := f.read(1024*1024): h.update(b)
    assert h.hexdigest() == a['decompressed_tar_sha256']
    with tarfile.open(arc,'r:gz') as f:
        members = f.getmembers(); assert len(members) == len(a['members']) == 2
        for got,want in zip(members,a['members']):
            assert got.isfile() and got.name == want['path'] and got.size == want['bytes'] and stat.S_IMODE(got.mode) == want['mode']
            assert hashlib.sha256(f.extractfile(got).read()).hexdigest() == want['sha256']
        assert a['members'][0]['sha256'] == r['llvm_sha256'] and a['members'][1]['sha256'] == r['executable_sha256']
    assert sum(x['bytes'] for x in a['members']) <= 20*1024*1024 and r['artifact_bytes'] <= 30*1024*1024
    assert r['minimum_observed_free_bytes'] >= 250*1024*1024
    minimum_free.append(r['minimum_observed_free_bytes']); fresh_bytes.append(r['artifact_bytes'])
    units[name] = {'receipt':rel,'sha256':sha(ROOT/rel),'fixture':unit['fixture'],'fixture_sha256':unit['fixture_sha256'],'oracle_rows':unit['oracle_rows'],'retained_rows':unit['extra_retained_rows'],'owner_groups':unit['owner_groups'],'memory_processes':2,'archive_index':arcrel,'archive_index_sha256':sha(ROOT/arcrel),'archive_sha256':a['archive_sha256']}
# Verify the historical failure inventory rather than recompiling or replacing any failed epoch.
old = read('compiler/evidence/stdlib/stdlib-json-method-v5-baseline.json')['immutable_failure_files']
for path,expected in old.items():
    p = ROOT/path
    assert p.is_file() and p.stat().st_size == expected['bytes'] and stat.S_IMODE(p.stat().st_mode) == expected['mode'] and sha(p) == expected['sha256'], path
stage = read('compiler/evidence/stdlib/stdlib-json-syntax-partial-stage-checkpoint.json')
assert stage['sources'] == plan['sources'] and stage['oracle_rows'] == 902 and stage['focused_controls'] == 29 and stage['canonical_changed_module_docs'] == 866
for failure in stage['preserved_first_failure_epochs'].values():
    r = read(failure['receipt']); project = Path(r['emit_command'][2]).parent.parent/'runtime'
    assert sha(project/'consumer.ll') == r['llvm_sha256'] and sha(project/'consumer') == r['executable_sha256']
assert sum(x['oracle_rows'] for x in units.values()) == 1494
out = {'status':'passed-current-syntax-source-1494-plus-retained1-owner4-rust-only','sources':plan['sources'],'source_candidate_commit':'412ce5d130a894ce61f8561568a6ed342b34cba1','compiler_sha256':plan['compiler_sha256'],'runtime_sha256':plan['runtime_sha256'],'oracle':plan['oracle'],'units':units,'original_rows':1494,'extra_retained_rows':1,'owner_groups':4,'bounded_memory_processes':14,'minimum_observed_free_bytes':min(minimum_free),'maximum_fresh_unit_bytes':max(fresh_bytes),'historical_plain_failure_files_verified':len(old),'new_syntax_failures_preserved_plain':True,'syntax902_plus29_receipt':'compiler/evidence/stdlib/stdlib-json-syntax-partial-stage-checkpoint.json','syntax902_plus29_receipt_sha256':sha(ROOT/'compiler/evidence/stdlib/stdlib-json-syntax-partial-stage-checkpoint.json'),'canonical_docs':866,'all_source_epochs_and_oracles_preserved':True,'sdk_rendering_panics_retained':12,'parent_actual_producer_gate':'pending-Root-owned-fresh-qualification','native_qualified':False,'performance_claim':None,'full92_complete':False,'production_promoted':False,'remaining':['Fresh Root parent maintained tests and generated reference on actual producer/dependencies','Additional partial Unicode/delimiter/interface/key/method observations','Full Effect.Json actual streaming IO causes and advanced source/sink handles','Native qualification, public pipeline benchmarks and final92 mapping']}
p = ROOT/'compiler/evidence/stdlib/stdlib-json-syntax-affected1494-stage-checkpoint.json'; p.write_text(json.dumps(out,indent=2)+'\n')
print('PASS current source1494 +retained1 +owner4,14 memory profiles; syntax902+29 preserved, full92 open')
