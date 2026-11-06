"""Build a complete, revision-aware checklist from the preserved run inventories.

This is a status reader, not a verifier or a source of compiler authority.
Older successful runs remain visible but never satisfy a newer compiler gate.
"""
from pathlib import Path
import collections
import ast
import functools
import os
import hashlib
import json
import re

BASE = Path(__file__).resolve().parent
STATES = ('verified', 'blocked', 'running', 'pending', 'recheck')
GITHUB = 'https://github.com/iand675/cinch/blob/main/'


def read(path):
    if not path:
        return {}
    try:
        return json.loads(Path(path).read_text())
    except (OSError, ValueError):
        return {}


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def link(path):
    return GITHUB + path


def counts(rows):
    return {s: sum(r['state'] == s for r in rows) for s in STATES}


def cases(log):
    if not log or not Path(log).is_file():
        return []
    p = Path(log)
    stat = p.stat()
    return _cases(str(p), stat.st_mtime_ns, stat.st_size)


@functools.lru_cache(maxsize=2048)
def _cases(log, modified, size):
    # Test logs can contain runtime leak reports or fixture output between the
    # test name and its final outcome. Preserve incomplete cases as running.
    text = Path(log).read_text(errors='replace')[:4_000_000]
    pattern = r'^(?:test )?(test_[\w]+ \([^\n]+?\)|[\w:]+) \.\.\. ([\s\S]*?)(?=\n(?:test |test_\w+ \(|test result:|Ran \d)|\Z)'
    out = []
    for m in re.finditer(pattern, text, re.M):
        body = m.group(2)
        status = 'passed' if re.search(r'(^|\n)ok(?:\n|$)', body) else 'failed' if 'FAILED' in body else 'ignored' if body.startswith('ignored') else 'running'
        out.append(dict(name=m.group(1), status=status))
    return out


def observed(row, source, log_root=None):
    if not row:
        return None
    log = row.get('log') or (str(Path(log_root)/(row['name']+'.log')) if log_root else None)
    return dict(status=row.get('status', 'pending'), source=source,
                startedAt=row.get('started'), finishedAt=row.get('finished'),
                exitCode=row.get('exit_code'), logSha256=row.get('log_sha256'),
                cases=cases(log))


def item(id, title, state, detail, done_when, evidence=None, **extra):
    assert state in STATES
    return dict(id=id, title=title, state=state, detail=detail,
                doneWhen=done_when, evidence=evidence, **extra)


def suite(id, title, unit, rows, detail, **extra):
    return dict(id=id, title=title, unit=unit, total=len(rows), counts=counts(rows),
                detail=detail, items=rows, **extra)


def build_checklist(root, cfg, previous=None):
    root = Path(root)
    compiler = read(cfg.get('publishedRustInputs')).get('compiler_source_sha256')
    native = cfg.get('publishedNativeSource')
    full = read(cfg['native'])
    front = read(cfg['selected'])
    scope = read(cfg['coverage'])
    chain = read(Path(cfg['chain'])/'independent-verification.json')
    old_native = {r['name']: r for r in full.get('rows', [])}
    old_front = {r['name']: r for r in front.get('rows', [])}
    catalog = read(Path(cfg['native']).parent/'test-binaries.json')
    native_names = sorted(set(catalog) | {'doc'} | {p.stem for p in (root/'compiler/tests').glob('*.rs')})
    front_names = scope['portable_targets'] + scope['interface_api_targets']
    assert len(native_names) >= 235
    assert len(front_names) == len(set(front_names)) >= 105
    assert set(front_names) >= set(scope['original_targets'])
    assert set(front_names) <= set(native_names)

    # Credit current Rust-only groups only when their production identity and
    # their exact test source match the current published inputs. These receipts
    # do not satisfy the separate final-Stage3 frontend requirement.
    current_native = {}
    for key in ('nativeQuotedReceipt', 'nativeDefaultAnnotationReceipt', 'nativeLiteralContextReceipt', 'nativeIndexKindReceipt'):
        receipt = read(cfg.get(key))
        if receipt.get('native_source_sha256') != native:
            continue
        source = Path(receipt.get('source', '/nonexistent'))
        for run in receipt.get('runs', []):
            name = 'lib' if run['name'] == 'all-native-unit-tests' else run['name']
            if name != 'lib':
                relative = Path('compiler/tests')/(name+'.rs')
                if not (source/relative).is_file() or digest(source/relative) != digest(root/relative):
                    continue
            current_native[name] = dict(run, evidence=link('bootstrap/'+Path(cfg[key]).name))

    native_rows = []
    for name in native_names:
        current = current_native.get(name)
        old = old_native.get(name)
        same = full.get('native_source_sha256') == native and full.get('compiler_source_sha256') == compiler
        if current:
            state = 'verified'
            detail = f"{current['tests']} tests passed; current Rust production identity and test source match."
        elif same and old:
            state = {'passed':'verified', 'failed':'blocked', 'running':'running'}.get(old['status'], 'pending')
            detail = 'Complete original target on the current compiler inputs.'
        else:
            state = 'recheck' if old and old.get('status') != 'pending' else 'pending'
            detail = 'Awaiting a complete run on the current compiler inputs.'
        native_rows.append(item('native:'+name, name, state, detail,
            'Every original case, including ignored cases, passes with zero failed, skipped or filtered tests on the matching inputs.',
            current.get('evidence') if current else link('compiler/src/lib.rs' if name == 'lib' else 'compiler/tests/'+name+'.rs') if name not in ('doc','bin-cinch') else link('compiler/Cargo.toml'),
            observed=observed(old, full.get('compiler_source_sha256'), Path(cfg['native']).parent),
            currentTests=current.get('tests') if current else None))

    front_rows = []
    for name in front_names:
        old = old_front.get(name)
        same = front.get('compiler_source_sha256') == compiler
        state = {'passed':'verified', 'failed':'blocked', 'running':'running'}.get(old.get('status'), 'pending') if same and old else 'recheck' if old and old.get('status') != 'pending' else 'pending'
        front_rows.append(item('stage3:'+name, name, state,
            'Requires the latest genuinely self-built Stage3. Development-compiler checks are listed separately.',
            'Run the complete original target through the matching Stage3; separately classify direct Rust API assertions.',
            link('compiler/tests/'+name+'.rs'), observed=observed(old, front.get('compiler_source_sha256'), Path(cfg['selected']).parent),
            category='interface / API' if name in scope['interface_api_targets'] else 'portable behavior'))

    suites = [suite('native', 'Rust compiler suite', 'targets', native_rows,
        '235 preserved targets, including library, integration, binary and documentation targets. A target can contain many tests.',
        observedCounts=dict(collections.Counter(r.get('status') for r in full.get('rows', []))), observedSource=full.get('compiler_source_sha256')),
        suite('stage3', 'Cinch-built compiler suite', 'targets', front_rows,
        'All 100 original targets plus five additions: 96 portable targets and nine interface/API groups.',
        observedCounts=dict(collections.Counter(r.get('status') for r in front.get('rows', []))), observedSource=front.get('compiler_source_sha256'))]

    auxiliary = read(cfg['auxiliary'])
    aux_source = Path(auxiliary.get('source', '/nonexistent'))
    # Runtime tests are standalone. Byte-for-byte equivalence of the complete
    # runtime tree allows their actual passing evidence to remain applicable.
    def runtime_files(base):
        found = {}
        for directory, dirs, files in os.walk(base/'compiler/runtime'):
            dirs[:] = [d for d in dirs if d not in ('target', '__pycache__', '.git')]
            for name in files:
                p = Path(directory)/name
                found[str(p.relative_to(base))] = digest(p)
        return found
    runtime_inputs = runtime_files(aux_source) if aux_source.is_dir() else {}
    runtime_current = bool(runtime_inputs) and runtime_inputs == runtime_files(root)
    aux_root = Path(cfg.get('artifactRoot', BASE))/'current4f-auxiliary-1'
    for id, title, log_name, total_key, required in [
        ('runtime','Runtime suite','runtime.log','runtime_tests',171),
        ('tooling','Bootstrap and tooling suite','tooling.log','tooling_tests',43),
        ('editor','Syntax and editor suite','syntax-editor.log','editor_tests',39)]:
        found = cases(aux_root/log_name)
        assert len(found) == auxiliary.get(total_key) == required, (id, len(found), auxiliary.get(total_key))
        same = runtime_current if id == 'runtime' else auxiliary.get('compiler_source_sha256') == compiler and auxiliary.get('native_source_sha256') == native
        rows = [item(id+':'+c['name'], c['name'], 'verified' if same and c['status']=='passed' else 'blocked' if same and c['status']=='failed' else 'recheck',
            'Passed; complete runtime source and test inputs are unchanged.' if id=='runtime' and same else 'Last run belongs to an earlier compiler snapshot.',
            'The complete original test passes against matching code and its required compiler role.',
            link('bootstrap/current-record-indexed-auxiliary-results.json'),
            observed=dict(status=c['status'], source=auxiliary.get('compiler_source_sha256'), cases=[])) for c in found]
        suites.append(suite(id, title, 'tests', rows,
            'Exact runtime/test input equivalence checked against the retained successful run.' if id=='runtime' and same else 'Older passing results remain visible; matching compiler/editor frontend execution is still required.'))

    external = read(cfg['external'])
    ext_rows = [item('external:'+str(i), r['name']+(' / '+r['phase'] if r.get('phase') else ''),
        'verified' if external.get('compiler_source_sha256')==compiler and r['status']=='passed' else 'recheck',
        'Includes actual emit/link/run, facade, class identity and defining-module fixity controls.',
        'Pass this original control using the final matching self-built executable.',
        link('bootstrap/current-record-indexed-external-acceptance-results.json'),
        observed=dict(status=r['status'], source=external.get('compiler_source_sha256'), cases=[])) for i,r in enumerate(external.get('rows', []))]
    assert len(ext_rows) == 14
    suites.append(suite('external','External acceptance controls','checks',ext_rows,'All 14 original checks remain required on the final compiler.'))

    language = read(cfg.get('currentExternalBehaviorReceipt'))
    # This independently audited receipt includes all thirteen positive examples
    # and all twenty-one required refusal reasons, rather than just exit codes.
    example_root = root/'experiments/malleable-syntax'
    test_text = (example_root/'tests/test_native.py').read_text()
    positive_names = sorted(p.stem for p in (example_root/'examples').glob('*.cinch'))
    negative_names = sorted(p.stem for p in (example_root/'examples/negative').glob('*.cinch'))
    reasons_node = next(n.value for n in ast.walk(ast.parse(test_text))
                        if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'reasons' for t in n.targets))
    reasons = ast.literal_eval(reasons_node)
    assert set(reasons) == set(negative_names), 'Every negative fixture must retain its original expected reason'
    assert len(positive_names)==13 and len(negative_names)==21, (positive_names,negative_names)
    assert set(language.get('positive_failures', [])) <= set(positive_names)
    assert set(language.get('negative_reason_failures', [])) <= set(negative_names)
    lang_rows=[]
    for positive,names in [(True,positive_names),(False,negative_names)]:
        failures=language.get('positive_failures' if positive else 'negative_reason_failures',[])
        for name in names:
            status='failed' if name in failures else 'passed'
            lang_rows.append(item('language:'+('positive:'if positive else'negative:')+name,
                name+(' / valid program'if positive else' / required refusal'),
                ('verified'if status=='passed'else'blocked') if language.get('compiler_source_sha256')==compiler else'recheck',
                'Valid programs must compile and run.' if positive else 'Required diagnostic: '+reasons[name]+'. An unrelated refusal does not pass.',
                'Pass the original program or exact refusal-reason assertion through the final matching Stage3.',
                link('bootstrap/current-record-indexed-stage3-language-results.json'),
                observed=dict(status=status,source=language.get('compiler_source_sha256'),cases=[])))
    suites.append(suite('language','Language behavior and refusal reasons','cases',lang_rows,'All 13 valid examples and all 21 intended-reason refusal checks.'))

    acceptance_text = [
        ('Pinned seed builds Stage1', 'The pinned Rust seed builds all latest compiler sources; Stage1 is runnable and passes its executable smoke checks.'),
        ('Stage1 builds Stage2', 'Stage1 freshly rebuilds every compiler module from those same inputs, without a Rust frontend or prebuilt compiler substitution.'),
        ('Stage2 builds matching Stage3', 'Build and run Stage3; Stage2/Stage3 emitted IR, executable and every canonical interface agree.'),
        ('Changed-source bootstrap works', 'Change actual compiler source, rebuild it through the chain, and prove that its changed behavior reaches the consumer.'),
        ('All tests and frontend parity pass', 'Complete all preserved compiler, runtime, tooling, editor and external behavior coverage, including ignored cases and the final coverage audit.')]
    chain_matches = chain.get('compiler_source_sha256')==compiler
    acceptance=[]
    for i,(title,requirement)in enumerate(acceptance_text,1):
        old_proven=i<5 and chain.get('status')=='passed-bootstrap-chain-only'
        state='verified'if chain_matches and old_proven else'recheck'if old_proven else'pending'
        acceptance.append(item('acceptance:'+str(i),str(i)+'. '+title,state,
            'Proven for earlier '+chain.get('compiler_source_sha256','unknown')[:8]+'; must be repeated for the latest compiler.' if old_proven and not chain_matches else'No complete matching final acceptance receipt yet.',
            requirement,link('design/self-hosting.md#acceptance'), observed=dict(status='passed'if old_proven else'pending',source=chain.get('compiler_source_sha256'),cases=[])))

    api=read(cfg.get('interfaceApiReceipt'))
    # A preserved failure from the unchanged actual Cinch frontend remains a
    # blocker while its Rust public-reader replay is pending. This grants no
    # current positive API credit to a different reader.
    api_current=api.get('compiler_source_sha256')==compiler
    failed_api=set(api.get('failed_groups',[])) if api_current else set()
    annotation_receipt=read(cfg.get('nativeDefaultAnnotationReceipt'))
    annotation_published=annotation_receipt.get('native_source_sha256')==native and annotation_receipt.get('whole_groups')==21 and annotation_receipt.get('rust_tests')==332
    default_receipt=read(cfg.get('sourceClassDefaultComponentReceipt'))
    gaps=[
        item('gap:prelude','Replace source copies of library modules safely','blocked'if 'interface_prelude_overrides'in failed_api else'pending',
             'The current interface/API run fails when source Data.Core is supplied alongside its cached interface. All four original families remain required.',
             'Core, Ordering, Category and Algebra replacements pass with fresh checked provider hashes, valid consumer chains and no Prelude back-edge.',link('bootstrap/current-source-prelude-counterexample.json')),
        item('gap:source-defaults','Check and inherit source class defaults','running','All four complete component groups now pass independent audit, including all15 defaults and all30 original source dictionary controls. The actual full compiler/object and runtime integration are running; imported defaults and complete bootstrap remain open.' if default_receipt else 'Development compiler8fce passes all15 controls in a retained-checker replay with the corrected Rust oracle; complete groups and runtime integration remain separate.',
             'All source default controls, unused errors, generic scopes, metadata stability, runtime and original class-default behavior pass.',link('bootstrap/current-source-class-default-development.json')),
        item('gap:imported-defaults','Transport checked imported default implementations','pending','A default hash alone does not provide the checked implementation required by an inheriting source instance.',
             'Inherited imported defaults retain owner scope, private closure, actual method contracts and compiled behavior; missing or stale contracts refuse.',link('design/compiled-interface-metadata.md')),
        item('gap:laws','Elaborate source class laws and declarations','pending','Category and Algebra source probes reach this explicit refusal.',
             'Retain and check the original class laws and namespace declarations; pass all original family and law assertions.',link('design/next-steps.md')),
        item('gap:base-names','Restore source-family implicit base names','pending','Invalidating the cached Prelude closure alone loses required constructor and base names.',
             'Source library homes retain their checked base scope without a facade back-edge or unchecked aliases.',link('bootstrap/current-source-prelude-counterexample.json')),
        item('gap:task-callback','Reject mutable task captures hidden by callbacks','blocked','Reproduced on the current published Rust compiler. This is an incorrectly accepted program.',
             'The callback carries checked capture/transfer/lifetime facts and refuses the bad capture while safe callbacks still pass.',link('bootstrap/current-native-task-capture-counterexample.json')),
        item('gap:task-helper','Reject mutable task captures hidden by helpers','blocked','Reproduced on the current published Rust compiler; helper indirection still hides a mutable capture.',
             'Capture requirements survive helpers, aliases and interfaces, with both bad and valid cases checked.',link('bootstrap/current-native-task-capture-counterexample.json')),
        item('gap:rust-annotations','Preserve Rust class/default annotation binders','verified' if annotation_published else 'blocked','Published8ba8 passes independent audit: all21 complete groups,332 tests, both44-module projects, actual inherited execution42 and three old/new CLI comparisons. Original refusal controls remain.' if annotation_published else 'The published995f compiler rejects three valid annotation examples. The private8ba8 repair has finished execution of its affected groups and both44-module checks; independent audit and publication remain required.',
             'Complete the affected proof/syntax/interface groups and both compiler-source projects, retain kind/scope refusals, then publish the validated repair.',link('bootstrap/current-native-default-annotation-results.json') if annotation_published else link('design/next-steps.md')),
        item('gap:records','Finish record interface transport','recheck','A known earlier compiler gap; latest exact behavior still needs a complete check.',
             'The original record construction, projection, privacy, layout and interface assertions pass on the matching compiler.',link('design/next-steps.md')),
        item('gap:copyout','Finish Copy / borrow / Thunk ownership','recheck','The original array target had13/15 passing cases; ownership/copyOut is one of its two retained failures.',
             'The original ownership case reaches and passes its required semantic checks and runtime behavior.',link('bootstrap/current-original-array-suite-results.json')),
        item('gap:staging','Finish staging and namespace behavior','recheck','The second original array failure and remaining language examples require real staging/declaration behavior.',
             'Preserve original sources and expected named refusals; execute the matching staging and language suites.',link('bootstrap/current-original-array-suite-results.json')),
        item('gap:hidden-types','Stop choosing a concrete hidden constructor type','pending','Native existential/hidden-type introduction remains a required investigation and repair.',
             'Hidden types remain existential, with correct construction, scoped elimination, kind checks and no guessed Nat carrier.',link('design/next-steps.md')),
        item('gap:fixities','Reverify defining-module-only fixities','recheck','The earlier self-built controls prove that both identical and changed importer declarations refuse.',
             'Repeat the original defining-owner, facade/rename and importer-refusal controls through the final matching Stage3.',link('bootstrap/current-record-indexed-external-acceptance-results.json')),
        item('gap:coverage','Audit every frontend-facing test outside the Stage3 matrix','pending',
             f'{len(set(native_names)-set(front_names))} Rust targets are outside the preserved105-target Stage3 matrix. Direct API assertions require classification; portable source cases require matching frontend coverage.',
             'Review all remaining targets and assertions, retain direct Rust API roles explicitly, and add every missing portable behavior case to final Stage3 execution.',link('bootstrap/external-coverage-inventory.json')),
        item('gap:final-audit','Complete the requirement-by-requirement final audit','pending','A green subset, source check or earlier fixed point cannot close the full goal.',
             'Every acceptance item, named artifact, invariant and complete matching suite has direct current evidence; all failures and unclassified coverage are closed.',link('design/self-hosting.md#acceptance'))]

    active=[]
    for key,title,total,skip in [
        ('nativeDefaultAnnotationValidation','Rust annotation repair: primary groups',14,'build-'),
        ('nativeDefaultAnnotationFollowup','Rust annotation repair: proof/syntax groups',7,'compile-'),
        ('sourceClassDefaultValidation','Cinch default implementation: complete component groups',4,'compile-')]:
        state=read(cfg.get(key))
        if state:
            wanted={'proof_guarded_lets','evidence_invalidation','index_local_carriers','syntax_index_evidence','syntax_index_lambdas','syntax_signature_scope','staging'} if 'Followup'in key else None
            rows=[r for r in state.get('rows',[])if not r['name'].startswith(skip)and(wanted is None or r['name']in wanted)]
            receipt=annotation_receipt if key.startswith('nativeDefaultAnnotation') else default_receipt
            audited=bool(receipt) and receipt.get('source')==state.get('source')
            active.append(dict(title=title,status=state.get('status'),phase=state.get('phase'),passed=sum(r['status']=='passed'for r in rows),total=total,source=state.get('compiler_source_sha256'),nativeSource=state.get('native_source_sha256'), auditPending=state.get('status','').startswith('passed-') and not audited))

    for key,title,total in [('sourceClassDefaultFullCompiler','Cinch defaults: full compiler/object gate',2), ('nativeDefaultAnnotationApi','Published Cinch: Rust reader/API replay',9)]:
        state=read(cfg.get(key))
        if state:
            rows=[r for r in state.get('rows',[]) if not r['name'].startswith(('compile-','build-'))]
            active.append(dict(title=title,status=state.get('status'),phase=state.get('phase'),passed=sum(r['status']=='passed'for r in rows),failed=sum(r['status']=='failed'for r in rows),total=total,source=state.get('compiler_source_sha256'),auditPending=state.get('status')=='passed'))

    for title, state, total in [('Earlier compiler: full Rust sweep', full, len(native_names)),
                                ('Earlier Stage3: complete frontend sweep', front, len(front_names))]:
        rows = state.get('rows', [])
        if state.get('status') == 'running':
            active.append(dict(title=title, status=state['status'], phase=state.get('phase'),
                passed=sum(r.get('status') == 'passed' for r in rows),
                failed=sum(r.get('status') == 'failed' for r in rows), total=total,
                source=state.get('compiler_source_sha256'), earlier=True))

    all_rows=acceptance+gaps+[r for s in suites for r in s['items']]
    assert len({r['id']for r in all_rows})==len(all_rows)
    return dict(version=1,target=dict(compiler=compiler,native=native),acceptance=acceptance,
                gaps=gaps,suites=suites,active=active,counts=counts(all_rows),total=len(all_rows),
                acceptanceVerified=sum(r['state']=='verified'for r in acceptance),acceptanceTotal=5,
                explanation='Checklist counts are recorded requirements and test entries, not an estimate of effort or time. Earlier results stay visible but do not count as verification of changed compiler inputs.',
                fullGoalComplete=False)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True, type=Path, help='Published compiler checkout')
    parser.add_argument('--config', required=True, type=Path, help='Run/evidence inventory configuration')
    parser.add_argument('--output', type=Path, help='Write JSON here; otherwise print to stdout')
    args = parser.parse_args()
    result = json.dumps(build_checklist(args.root, read(args.config)), indent=2) + '\n'
    if args.output:
        args.output.write_text(result)
    else:
        print(result, end='')
