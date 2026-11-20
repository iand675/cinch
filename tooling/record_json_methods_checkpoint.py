#!/usr/bin/env python3
"""Verify a partial method checkpoint without conflating source epochs or package completion."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[1]
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def load(path):return json.loads((ROOT/path).read_text())
rust_sha='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
native_sha='2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f'
runtime_rust='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
runtime_native='80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141'
selected=[('method-protocol','json-codec-method-protocol.cinch'),('method-owner','json-codec-method-owner-control.cinch'),('addressability-trace','json-codec-addressability.cinch')]
receipts={};common=None;processes=0
for stage,fixture in selected:
    for frontend in ['rust','native']:
        suffix='native-v1' if frontend=='native' else 'rust-v1'
        path=f'compiler/evidence/stdlib/stdlib-json-{stage}-{suffix}-results.json'
        report=load(path)
        assert report['status']=='passed',path
        assert report['compiler_sha256']==(native_sha if frontend=='native' else rust_sha),path
        assert report['observed_runtime_archive_sha256']==(runtime_native if frontend=='native' else runtime_rust),path
        assert report['fixture_sha256']==digest(ROOT/'seed/tests/fixtures'/fixture),path
        assert report['opt_level']==2 and report['cpu_affinity']==[2] and report['consumer_stack_limit']=='8M',path
        assert report['artifact_bytes']<=20*1024*1024,path
        if common is None:common=report['sources']
        assert report['sources']==common,path
        for name,expected in report['sources'].items():assert digest(ROOT/'seed/prelude'/(name.replace('.','/')+'.cinch'))==expected,(path,name)
        assert len(report['runs'])==2,path
        for poisoned,run in zip([False,True],report['runs']):
            assert run['poisoned_free_validation']==poisoned and run['heap_limit']==('256M' if poisoned else '32M'),path
            assert run['stdout']=='' and run['stats']['live']==0 and run['leaked']==0,path
            processes+=1
        if frontend=='native':
            assert report['interfaces_sha256']=='1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256'
            assert report['objects_manifest_sha256']=='22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624'
            assert report['provider_object_sha256']=='46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64'
        ir=Path(report['raw_link_command'][2]);assert digest(ir)==report['llvm_sha256'],path
        receipts[path]={'sha256':digest(ROOT/path),'artifact_bytes':report['artifact_bytes'],'fixture_sha256':report['fixture_sha256']}

def walk(rows):
    for row in rows:
        yield row
        yield from walk(row.get('children',[]))
docs={}
for name in ['Value','Codec','Dynamic','Binary32','Legacy']:
    folder='json-method-protocol-docs-v1' if name in {'Value','Codec','Dynamic'} else 'json-dynamic-docs'
    path=ROOT/'build'/folder/(name+'.json');doc=json.loads(path.read_text())
    assert doc['sourceSha256']==digest(ROOT/f'seed/prelude/Text/Json/{name}.cinch')
    assert not doc['diagnostics'] and not doc['orphans']
    rows=list(walk(doc['declarations']))
    for row in rows:
        assert row.get('doc') and not row['doc']['legacy'],row['id']
        assert not re.search(r'\b(?:Go|Golang|Rust|Python)\b|C\+\+',row['doc']['markdown']),row['id']
    docs[name]={'source_sha256':doc['sourceSha256'],'canonical_declarations':len(rows),'doc_sha256':digest(path)}
assert sum(row['canonical_declarations'] for row in docs.values())==801
v1=load('seed/tests/fixtures/json-codec-methods-vectors.json');v2=load('seed/tests/fixtures/json-codec-methods-vectors-v2.json')
assert sum(map(len,v1.values()))==1160 and sum(map(len,v2.values()))==1464
for name,rows in v1.items():
    projected=[{**row,'error':{key:value for key,value in row['error'].items() if key!='json_kind'}} for row in v2[name][:len(rows)]]
    assert rows==projected,name
pending=load('seed/tests/fixtures/json-codec-addressability-pending.json')
assert len(pending['rows'])==1 and pending['rows'][0]['error']['cause']=='invalid use of `string` tag option'
plan=load('compiler/evidence/stdlib/stdlib-json-capability-plan.json');assert plan['reference_declarations']==92 and len(plan['declarations'])==92
assert plan['status'].startswith('partial-')
report={'description':'Partial independent method/addressability/semantic ownership checkpoint; full92 remains open','source_sha256':common,'compiler_epochs':{'rust':rust_sha,'native':native_sha},'runtime_epochs':{'rust':runtime_rust,'native':runtime_native},'receipts':receipts,'bounded_memory_processes':processes,'canonical_docs':docs,'canonical_declarations':801,'independent_observations':1464,'original_observations_preserved':1160,'tested_increment':'Protocol7 +directional5/metadata-owner3 +79 successful addressability output/callback-count rows and typed-interface boundary, each on both exact frontends','mandatory_pending_descriptor_error_rows':1,'remaining':['Full builtin/descriptor error reporting and application field/record metadata','Complete method nil-interface/object-key adaptations and remaining1464 outcomes','Malformed-input partial updates with reporting disabled','Bounded Effect.Json with shared streaming operation checkpoints and advanced handles','Paired public pipeline benchmarks/lowering and final92-record reconciliation'],'performance_claim':None}
output=ROOT/'compiler/evidence/stdlib/stdlib-json-methods-checkpoint.json';output.write_text(json.dumps(report,indent=2)+'\n')
print(f'PASS: partial methods checkpoint, {processes} bounded processes,801 canonical declarations; full92 remains open.')
